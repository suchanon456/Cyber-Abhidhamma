// core/src/paramattha/cetasika.rs
// ทะเบียนเจตสิก 52 ดวง — ลำดับ index ตรงกับ spec/Vattapada-7.json (`index` 0..=51)
//
// ไฟล์นี้เป็นทะเบียนชื่อ (id) + index คงที่ของแต่ละดวง ใช้โดย
// `CetasikaVector` (52 มิติ) ใน `vithi.rs`

/// จำนวนเจตสิกทั้งหมดตาม spec (สัพพจิตตสาธารณเจตสิก 7 + ปกิณณกเจตสิก 6
/// + อกุศลเจตสิก 14 + โสภณเจตสิก 25)
pub const CETASIKA_COUNT: usize = 52;

/// id ของเจตสิกทั้ง 52 ดวง เรียงตาม index ใน spec
pub const CETASIKA_IDS: [&str; CETASIKA_COUNT] = [
    // 0..=6 สัพพจิตตสาธารณเจตสิก 7
    "phassa",
    "vedana",
    "sanna",
    "cetana",
    "ekaggata",
    "jivitindriya",
    "manasikara",
    // 7..=12 ปกิณณกเจตสิก 6
    "vitakka",
    "vicara",
    "adhimokkha",
    "viriya",
    "piti",
    "chanda",
    // 13..=26 อกุศลเจตสิก 14
    "moha",
    "ahirika",
    "anottappa",
    "uddhacca",
    "lobha",
    "ditthi",
    "mana",
    "dosa",
    "issa",
    "macchariya",
    "kukkucca",
    "thina",
    "middha",
    "vicikiccha",
    // 27..=45 โสภณเจตสิก 19 (ไม่รวม วิรตี และอัปปมัญญา ที่อยู่ท้าย)
    "saddha",
    "sati",
    "hiri",
    "ottappa",
    "alobha",
    "adosa",
    "tatramajjhattata",
    "kayapassaddhi",
    "cittapassaddhi",
    "kayalahuta",
    "cittalahuta",
    "kayamuduta",
    "cittamuduta",
    "kayakammannata",
    "cittakammannata",
    "kayapagunnata",
    "cittapagunnata",
    "kayujukata",
    "cittujukata",
    // 46..=48 วิรตีเจตสิก 3
    "sammavaca",
    "sammakammanta",
    "samma_ajiva",
    // 49..=50 อัปปมัญญาเจตสิก 2
    "karuna",
    "mudita",
    // 51 ปัญญาเจตสิก 1
    "panna",
];

macro_rules! cetasika_index {
    ($($name:ident = $index:expr;)*) => {
        $(
            #[doc = concat!("index ของ `", stringify!($name), "` ในทะเบียน")]
            pub const $name: usize = $index;
        )*
    };
}

// index คงที่ — ต้องตรงกับ CETASIKA_IDS ข้างบน (มีเทสต์ตรวจ)
cetasika_index! {
    PHASSA = 0;
    VEDANA = 1;
    SANNA = 2;
    CETANA = 3;
    EKAGGATA = 4;
    JIVITINDRIYA = 5;
    MANASIKARA = 6;
    VITAKKA = 7;
    VICARA = 8;
    ADHIMOKKHA = 9;
    VIRIYA = 10;
    PITI = 11;
    CHANDA = 12;
    MOHA = 13;
    AHIRIKA = 14;
    ANOTTAPPA = 15;
    UDDHACCA = 16;
    LOBHA = 17;
    DITTHI = 18;
    MANA = 19;
    DOSA = 20;
    ISSA = 21;
    MACCHARIYA = 22;
    KUKKUCCA = 23;
    THINA = 24;
    MIDDHA = 25;
    VICIKICCHA = 26;
    SADDHA = 27;
    SATI = 28;
    HIRI = 29;
    OTTAPPA = 30;
    ALOBHA = 31;
    ADOSA = 32;
    TATRAMAJJHATTATA = 33;
    KAYAPASSADDHI = 34;
    CITTAPASSADDHI = 35;
    KAYALAHUTA = 36;
    CITTALAHUTA = 37;
    KAYAMUDUTA = 38;
    CITTAMUDUTA = 39;
    KAYAKAMMANNATA = 40;
    CITTAKAMMANNATA = 41;
    KAYAPAGUNNATA = 42;
    CITTAPAGUNNATA = 43;
    KAYUJUKATA = 44;
    CITTUJUKATA = 45;
    SAMMAVACA = 46;
    SAMMAKAMMANTA = 47;
    SAMMA_AJIVA = 48;
    KARUNA = 49;
    MUDITA = 50;
    PANNA = 51;
}

/// index ของเจตสิกจาก id (`None` ถ้าไม่มีในทะเบียน)
pub fn index_of(id: &str) -> Option<usize> {
    CETASIKA_IDS.iter().position(|&s| s == id)
}

/// id ของเจตสิกที่ index นี้ (`None` ถ้าเกินขอบเขต)
pub fn id_of(index: usize) -> Option<&'static str> {
    CETASIKA_IDS.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn registry_has_52_unique_ids() {
        assert_eq!(CETASIKA_IDS.len(), CETASIKA_COUNT);
        let unique: HashSet<_> = CETASIKA_IDS.iter().collect();
        assert_eq!(unique.len(), CETASIKA_COUNT);
    }

    #[test]
    fn registry_matches_vattapada_spec_order() {
        let raw = include_str!("../../../spec/Vattapada-7.json");
        let json: serde_json::Value = serde_json::from_str(raw).expect("valid json");
        let entries = json["cetasika_registry"]["canonical_52"]
            .as_array()
            .expect("cetasika_registry.canonical_52 must be an array");
        assert_eq!(entries.len(), CETASIKA_COUNT);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry["index"].as_u64(), Some(i as u64));
            assert_eq!(entry["id"].as_str(), Some(CETASIKA_IDS[i]), "index {i}");
        }
    }

    #[test]
    fn index_constants_point_to_their_own_id() {
        let named: [(usize, &str); 8] = [
            (LOBHA, "lobha"),
            (DOSA, "dosa"),
            (MOHA, "moha"),
            (MANA, "mana"),
            (SATI, "sati"),
            (ADOSA, "adosa"),
            (PANNA, "panna"),
            (PHASSA, "phassa"),
        ];
        for (index, id) in named {
            assert_eq!(CETASIKA_IDS[index], id, "index {index}");
            assert_eq!(index_of(id), Some(index));
        }
    }

    #[test]
    fn index_of_finds_known_ids() {
        assert_eq!(index_of("phassa"), Some(0));
        assert_eq!(index_of("lobha"), Some(17));
        assert_eq!(index_of("panna"), Some(51));
        assert_eq!(index_of("metta"), None);
    }

    #[test]
    fn id_of_round_trips() {
        for (index, id) in CETASIKA_IDS.iter().enumerate() {
            assert_eq!(id_of(index), Some(*id));
            assert_eq!(index_of(id), Some(index));
        }
        assert_eq!(id_of(CETASIKA_COUNT), None);
    }
}
