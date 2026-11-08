// core/tests/spec_integrity.rs
// ตรวจความสอดคล้องของไฟล์ spec ทั้งสามไฟล์ด้วยโปรแกรม
// รันด้วย: cargo test --test spec_integrity

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn load(path: &str) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("อ่าน {path} ไม่ได้: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path} ไม่ใช่ JSON ที่ถูกต้อง: {e}"))
}

fn numbers(map: &Value) -> Vec<(&str, f64)> {
    map.as_object()
        .expect("ต้องเป็น object")
        .iter()
        .filter(|(k, _)| !k.starts_with('_'))
        .map(|(k, v)| (k.as_str(), v.as_f64().expect("ค่าต้องเป็น number")))
        .collect()
}

/// id เจตสิกที่ถูกต้องทั้งหมด: 52 ปรมัตถ์ + ของนอกปรมัตถ์ใน `extended_factors`
fn known_cetasika_ids(vattapada: &Value) -> BTreeSet<String> {
    let mut known = BTreeSet::new();
    for item in vattapada["cetasika_registry"]["canonical_52"]
        .as_array()
        .expect("canonical_52 ต้องเป็น array")
    {
        known.insert(item["id"].as_str().expect("id ต้องเป็น string").to_string());
    }
    for key in vattapada["cetasika_registry"]["extended_factors"]
        .as_object()
        .expect("extended_factors ต้องเป็น object")
        .keys()
    {
        if !key.starts_with('_') {
            known.insert(key.clone());
        }
    }
    known
}

/// `^[a-z][a-z0-9_]*$` ตาม `_schema.conventions.id_pattern`
fn is_ascii_snake_case(id: &str) -> bool {
    let mut chars = id.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// เก็บทุก `key_triggers` ที่อยู่ใน subtree พร้อม id ของเจ้าของที่ใกล้ที่สุด
fn collect_key_triggers<'a>(
    value: &'a Value,
    owner: Option<&'a str>,
    out: &mut Vec<(Option<&'a str>, Vec<&'a str>)>,
) {
    match value {
        Value::Object(map) => {
            let owner = map.get("id").and_then(|v| v.as_str()).or(owner);
            if let Some(Value::Array(list)) = map.get("key_triggers") {
                out.push((owner, list.iter().filter_map(|v| v.as_str()).collect()));
            }
            for (key, child) in map {
                if key != "key_triggers" {
                    collect_key_triggers(child, owner, out);
                }
            }
        }
        Value::Array(items) => {
            for child in items {
                collect_key_triggers(child, owner, out);
            }
        }
        _ => {}
    }
}

#[test]
fn all_spec_files_are_valid_json() {
    for path in [
        "../spec/Paramattha-Rupa.json",
        "../spec/Paramattha-Cetasika.json",
        "../spec/Paramattha-Citta.json",
        "../spec/Paramattha-Nibbana.json",
        "../spec/Paramattha-Index.json",
        "../spec/Rupa-Structure-Silicon.json",
        "../spec/Vattapada-7.json",
    ] {
        let v = load(path);
        assert!(v.is_object(), "{path} ต้องเป็น object");
    }
}

#[test]
fn cetasika_registry_has_52_contiguous_indices() {
    let spec = load("../spec/Vattapada-7.json");
    let list = spec["cetasika_registry"]["canonical_52"]
        .as_array()
        .expect("canonical_52 ต้องเป็น array");

    assert_eq!(list.len(), 52, "ต้องมีเจตสิก 52 ตัว");

    let mut indices = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for item in list {
        let index = item["index"].as_u64().expect("index ต้องเป็น integer");
        let id = item["id"].as_str().expect("id ต้องเป็น string");
        assert!(indices.insert(index), "index {index} ซ้ำ");
        assert!(ids.insert(id.to_string()), "id {id} ซ้ำ");
        assert!(
            id.is_ascii()
                && id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
            "id {id} ต้องเป็น ascii snake_case"
        );
    }

    let expected: BTreeSet<u64> = (0..52).collect();
    assert_eq!(indices, expected, "index ต้องต่อเนื่อง 0..51");

    let counts = &spec["cetasika_registry"]["counts"];
    assert_eq!(counts["total"].as_u64(), Some(52));
    assert_eq!(
        counts["sabbacittasadharana"].as_u64().unwrap()
            + counts["pakinnaka"].as_u64().unwrap()
            + counts["akusala"].as_u64().unwrap()
            + counts["sobhana"].as_u64().unwrap(),
        52
    );
}

#[test]
fn every_cetasika_referenced_by_a_formula_exists() {
    let spec = load("../spec/Vattapada-7.json");
    let known = known_cetasika_ids(&spec);

    for vatta in spec["vattapada_7"].as_array().unwrap() {
        let id = vatta["id"].as_str().unwrap();
        let formula = &vatta["cetasika_formula"];
        for (name, _) in numbers(&formula["weights"]) {
            assert!(
                known.contains(name),
                "{id}: weights อ้าง {name} ที่ไม่มีในทะเบียน"
            );
        }
        for (name, value) in numbers(&formula["penalties"]) {
            assert!(
                known.contains(name),
                "{id}: penalties อ้าง {name} ที่ไม่มีในทะเบียน"
            );
            assert!(value < 0.0, "{id}: penalty {name} ต้องเป็นค่าลบตาม schema");
        }
    }
}

#[test]
fn weights_of_each_vatta_sum_to_one() {
    let spec = load("../spec/Vattapada-7.json");
    let vattas = spec["vattapada_7"].as_array().unwrap();
    assert_eq!(vattas.len(), 7, "ต้องมีวัตรบท 7 ตัว");

    for (i, vatta) in vattas.iter().enumerate() {
        let id = vatta["id"].as_str().unwrap();
        assert_eq!(
            id,
            format!("vatta_{}", i + 1),
            "ลำดับ id ต้องเรียง vatta_1..vatta_7"
        );
        let sum: f64 = numbers(&vatta["cetasika_formula"]["weights"])
            .iter()
            .map(|(_, v)| v)
            .sum();
        assert!(
            (sum - 1.0).abs() < 1e-9,
            "{id}: ผลรวม weights = {sum} ต้องเป็น 1.0"
        );
    }
}

#[test]
fn tier_bands_are_ordered_and_thresholds_present() {
    let spec = load("../spec/Vattapada-7.json");
    let bands = spec["overall_governance"]["tier_bands"].as_array().unwrap();
    let mut previous = f64::INFINITY;
    for band in bands {
        let min = band["min_inclusive"].as_f64().unwrap();
        assert!(min < previous, "min_inclusive ต้องเรียงจากมากไปน้อย");
        previous = min;
        assert!(band["ring"].as_u64().is_some(), "ring ต้องเป็น integer");
        assert!(band["capabilities"].is_array(), "capabilities ต้องเป็น array");
    }

    let thresholds = spec["overall_governance"]["thresholds"]
        .as_object()
        .unwrap();
    for key in ["sakka_king_of_devas", "deva", "human", "preta"] {
        assert!(
            thresholds.contains_key(key),
            "thresholds ขาด key {key} ที่ engine ใช้"
        );
    }
}

#[test]
fn declared_derived_bounds_match_the_formula() {
    let spec = load("../spec/Vattapada-7.json");
    let bounds = &spec["engine"]["derived_bounds"];
    let n = 7.0_f64;
    let threshold = bounds["sakka_threshold"].as_f64().unwrap();
    let budget = n * (1.0 / threshold - 1.0);
    assert!(
        (budget - bounds["reciprocal_budget_at_sakka"].as_f64().unwrap()).abs() < 1e-9,
        "budget ที่ประกาศไม่ตรงกับสูตร n*(1/T-1)"
    );
    let floor = 1.0 / (1.0 + budget);
    assert!(
        (floor - bounds["single_weak_dimension_floor"].as_f64().unwrap()).abs() < 1e-9,
        "ขอบเขตมิติอ่อนตัวเดียวไม่ตรงกับ 1/(1+budget)"
    );
    assert!((floor - 4.0 / 11.0).abs() < 1e-12, "ต้องเท่ากับ 4/11");
}

#[test]
fn silicon_rupas_count_to_28() {
    let spec = load("../spec/Rupa-Structure-Silicon.json");
    let mut counted = 0;
    let mut listed = 0;
    let mut ids = BTreeSet::new();

    for category in spec["categories"].as_array().unwrap() {
        for rupa in category["rupas"].as_array().unwrap() {
            listed += 1;
            let id = rupa["id"].as_str().unwrap();
            assert!(ids.insert(id.to_string()), "id รูป {id} ซ้ำ");
            assert!(rupa["sila_check"].is_object(), "{id} ขาด sila_check");
            assert!(rupa["self_heal"].is_string(), "{id} ขาด self_heal");
            if rupa["counts_toward_total"].as_bool().unwrap_or(false) {
                counted += 1;
            }
        }
    }

    assert_eq!(counted, 28, "รูปที่นับได้ต้องเท่ากับ 28");
    assert_eq!(spec["counts"]["counted_rupas"].as_u64(), Some(28));
    assert_eq!(spec["counts"]["listed_rupas"].as_u64(), Some(listed as u64));
}

#[test]
fn every_threshold_ref_resolves() {
    let spec = load("../spec/Rupa-Structure-Silicon.json");
    let registry = spec["threshold_registry"].as_object().unwrap();

    let mut refs = 0;
    for category in spec["categories"].as_array().unwrap() {
        for rupa in category["rupas"].as_array().unwrap() {
            for rule in rupa["sila_check"]["rules"].as_array().unwrap() {
                let when = rule["when"].as_object().unwrap();
                for (field, condition) in when {
                    if let Some(reference) = condition.get("gt_ref").and_then(|v| v.as_str()) {
                        refs += 1;
                        assert!(
                            registry.contains_key(reference),
                            "{}.{} อ้าง gt_ref `{reference}` ที่ไม่มีใน threshold_registry",
                            rupa["id"].as_str().unwrap(),
                            field
                        );
                        assert!(
                            registry[reference].as_f64().is_some(),
                            "threshold {reference} ต้องเป็น number"
                        );
                    }
                }
            }
        }
    }

    assert!(refs > 0, "ต้องมีการอ้าง threshold อย่างน้อยหนึ่งครั้ง");
}

#[test]
fn no_inline_thresholds_and_every_registry_entry_is_used() {
    let spec = load("../spec/Rupa-Structure-Silicon.json");
    let registry = spec["threshold_registry"].as_object().unwrap();

    let mut used = BTreeSet::new();
    let mut problems: Vec<String> = Vec::new();
    for category in spec["categories"].as_array().unwrap() {
        for rupa in category["rupas"].as_array().unwrap() {
            let id = rupa["id"].as_str().unwrap();
            for rule in rupa["sila_check"]["rules"].as_array().unwrap() {
                for (field, condition) in rule["when"].as_object().unwrap() {
                    let Some(obj) = condition.as_object() else {
                        continue;
                    };
                    for op in ["gt", "gte", "lt", "lte"] {
                        if obj.get(op).is_some_and(|v| v.is_number()) {
                            problems
                                .push(format!("{id}.{field}: ตัวเลข `{op}` ฝังในกติกา ให้ใช้ gt_ref"));
                        }
                    }
                    if let Some(r) = obj.get("gt_ref").and_then(|v| v.as_str()) {
                        used.insert(r.to_string());
                    }
                }
            }
        }
    }

    for key in registry.keys().filter(|k| !k.starts_with('_')) {
        if !used.contains(key) {
            problems.push(format!("threshold_registry `{key}` ไม่มี rule ใดอ้างถึง"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn paramattha_counts_are_consistent_with_its_items() {
    let rupa_spec = load("../spec/Paramattha-Rupa.json");
    let cetasika_spec = load("../spec/Paramattha-Cetasika.json");
    let counts = &rupa_spec["counts"];

    assert_eq!(counts["rupa_28"]["claimed"].as_u64(), Some(28));
    let breakdown = counts["rupa_28"]["breakdown"].as_object().unwrap();
    let total: u64 = breakdown.values().map(|v| v.as_u64().unwrap()).sum();
    assert_eq!(total, 28, "ผลรวม breakdown ของรูปต้องเป็น 28");

    let cetasika = &cetasika_spec["counts"]["cetasika_52"];
    assert_eq!(cetasika["claimed"].as_u64(), Some(52));
    let breakdown = cetasika["breakdown"].as_object().unwrap();
    let total: u64 = breakdown.values().map(|v| v.as_u64().unwrap()).sum();
    assert_eq!(total, 52, "ผลรวม breakdown ของเจตสิกต้องเป็น 52");

    // นับจริงจากรายการ: sabbacittasadharana 7 + pakinnaka 6 + akusala 14 + sobhana 25
    let cetasika_root = &cetasika_spec["cetasika_52"];
    let count_items = |group: &str| -> usize {
        cetasika_root[group]["subgroups"]
            .as_object()
            .unwrap()
            .values()
            .map(|sub| sub["items"].as_array().map(|a| a.len()).unwrap_or(0))
            .sum()
    };

    assert_eq!(count_items("annasamana_cetasika_13"), 13, "อัญญสมานต้องมี 13");
    assert_eq!(count_items("akusala_cetasika_14"), 14, "อกุศลต้องมี 14");
    assert_eq!(count_items("sobhana_cetasika_25"), 25, "โสภณต้องมี 25");
}

#[test]
fn paramattha_cetasika_items_match_the_central_registry() {
    let vattapada = load("../spec/Vattapada-7.json");
    let spec = load("../spec/Paramattha-Cetasika.json");
    let known = known_cetasika_ids(&vattapada);

    let mut problems: Vec<String> = Vec::new();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();

    for (group, body) in spec["cetasika_52"]
        .as_object()
        .expect("cetasika_52 ต้องเป็น object")
    {
        if group.starts_with('_') {
            continue;
        }
        for (subgroup, sub) in body["subgroups"]
            .as_object()
            .unwrap_or_else(|| panic!("{group}.subgroups ต้องเป็น object"))
        {
            for item in sub["items"]
                .as_array()
                .unwrap_or_else(|| panic!("{group}.{subgroup}.items ต้องเป็น array"))
            {
                let id = item["id"].as_str().expect("id ต้องเป็น string");
                *seen.entry(id.to_string()).or_insert(0) += 1;
                if !known.contains(id) {
                    problems.push(format!(
                        "{group}.{subgroup}: id `{id}` ไม่มีใน cetasika_registry"
                    ));
                }
                if !is_ascii_snake_case(id) {
                    problems.push(format!(
                        "{group}.{subgroup}: id `{id}` ไม่ใช่ ascii snake_case"
                    ));
                }
            }
        }
    }

    for item in vattapada["cetasika_registry"]["canonical_52"]
        .as_array()
        .unwrap()
    {
        let id = item["id"].as_str().unwrap();
        if !seen.contains_key(id) {
            problems.push(format!(
                "ทะเบียนกลางมี `{id}` แต่ cetasika_52 ของ Paramattha ไม่มี"
            ));
        }
    }
    for (id, count) in &seen {
        if *count > 1 {
            problems.push(format!("id `{id}` ซ้ำ {count} ครั้งใน cetasika_52"));
        }
    }

    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

fn count_ids(value: &Value) -> usize {
    match value {
        Value::Object(map) => {
            usize::from(map.contains_key("id")) + map.values().map(count_ids).sum::<usize>()
        }
        Value::Array(items) => items.iter().map(count_ids).sum(),
        _ => 0,
    }
}

#[test]
fn split_files_hold_the_canonical_counts() {
    let rupa = load("../spec/Paramattha-Rupa.json");
    let cetasika = load("../spec/Paramattha-Cetasika.json");
    let citta = load("../spec/Paramattha-Citta.json");
    let nibbana = load("../spec/Paramattha-Nibbana.json");

    assert_eq!(count_ids(&citta["citta_121"]), 121, "จิตต้องมี 121 ดวง");
    assert_eq!(count_ids(&cetasika["cetasika_52"]), 52, "เจตสิกต้องมี 52 ตัว");
    assert_eq!(rupa["counts"]["rupa_28"]["claimed"].as_u64(), Some(28));
    assert!(
        nibbana["nibbana_paramattha"].is_object(),
        "ต้องมี nibbana_paramattha"
    );
}

#[test]
fn paramattha_rupa_groups_cover_every_silicon_rupa() {
    let silicon = load("../spec/Rupa-Structure-Silicon.json");
    let rupa = load("../spec/Paramattha-Rupa.json");

    let mut silicon_ids = BTreeSet::new();
    for category in silicon["categories"].as_array().unwrap() {
        for item in category["rupas"].as_array().unwrap() {
            silicon_ids.insert(item["id"].as_str().unwrap().to_string());
        }
    }

    let mut referenced = BTreeSet::new();
    let mut problems: Vec<String> = Vec::new();
    for category in rupa["paramattha_rupa_28"]["categories"].as_array().unwrap() {
        for element in category["elements"].as_array().unwrap() {
            let group = element["group"].as_str().unwrap();
            match element["silicon_ref"].as_array() {
                None => problems.push(format!("{group}: ขาด silicon_ref")),
                Some(list) => {
                    for id in list {
                        let id = id.as_str().unwrap();
                        if !silicon_ids.contains(id) {
                            problems.push(format!("{group}: silicon_ref `{id}` ไม่มีในไฟล์ Silicon"));
                        }
                        referenced.insert(id.to_string());
                    }
                }
            }
        }
    }
    for id in &silicon_ids {
        if !referenced.contains(id) {
            problems.push(format!("Silicon `{id}` ไม่ถูกอ้างจาก Paramattha-Rupa"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn every_key_trigger_resolves_to_a_registered_cetasika() {
    let vattapada = load("../spec/Vattapada-7.json");
    let spec = load("../spec/Paramattha-Citta.json");
    let known = known_cetasika_ids(&vattapada);

    let mut found: Vec<(Option<&str>, Vec<&str>)> = Vec::new();
    collect_key_triggers(&spec["citta_121"], None, &mut found);
    assert!(!found.is_empty(), "ต้องมี key_triggers อย่างน้อยหนึ่งรายการ");

    let mut problems: Vec<String> = Vec::new();
    let mut refs = 0;
    for (owner, ids) in &found {
        for id in ids {
            refs += 1;
            if !known.contains(*id) {
                problems.push(format!(
                    "citta_121 `{}`: key_triggers อ้าง `{id}` ที่ไม่มีในทะเบียน",
                    owner.unwrap_or("<ไม่มี id>")
                ));
            }
        }
    }

    assert!(refs > 0, "ต้องมีการอ้างเจตสิกอย่างน้อยหนึ่งครั้ง");
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
