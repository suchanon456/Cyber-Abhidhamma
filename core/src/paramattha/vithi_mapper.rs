// core/src/paramattha/vithi_mapper.rs
// VithiMapper — แมป "ขั้นของวิถีจิต" ไปยัง "จิต 121 ดวง" ที่เกิดในขั้นนั้น
//
// ## หลักการ
//
// วิถีจิต (`Vithi15`, `Vithi17`) บอกแค่ว่า "ขั้นไหนเกิด" แต่ไม่ได้บอกว่า
// "จิตดวงไหน" เกิดในขั้นนั้น ไฟล์นี้เติมส่วนนั้น โดยอ่านทะเบียนจิต 121 ดวง
// จาก `spec/Paramattha-Citta.json` ตอน compile (`include_str!`) แล้วเลือก
// ดวงที่ทำหน้าที่ตรงกับขั้นนั้น
//
// ## กฎที่ใช้ (และข้อจำกัด)
//
// | ขั้น | จิตที่เลือก | เหตุผล |
// |---|---|---|
// | ภวังค์ (3 ขณะ) | `mahavipaka_citta_1` | ตัวแทน — ของจริงขึ้นกับภูมิและกรรม |
// | ปัญจทวาราวัชชนะ | `pancadvaravajjana_citta` | ทำหน้าที่นี้โดยตรง |
// | ปัญจวิญญาณ | `*vinana_{kusala|akusala}_vipaka` | เลือกตามทวาร |
// | มโนทวาราวัชชนะ | `manodvaravajjana_citta` | ทำหน้าที่นี้โดยตรง |
// | สัมปฏิจฉนะ | `sampaticchanat_{kusala|akusala}_vipaka` | ตามวิบาก |
// | สันตีรณะ | `santirana_*_vipaka` | ตามวิบาก |
// | โวฏฐัพพนะ | `manodvaravajjana_citta` | จิตดวงนี้ทำหน้าที่โวฏฐัพพนะในวิถีปัญจทวาร |
// | ชวนะ ×7 | ขึ้นกับเจตสิก (ดูกฎข้างล่าง) | สั่งสมกรรม |
// | ตทาลัมพนะ ×2 | `mahavipaka_citta_1` | ตัวแทน |
//
// **กฎเลือกชวนะ** — เรียงตามลำดับความสำคัญ:
// 1. `lobha > 0.7` → `lobhamula_citta_1`
// 2. `dosa > 0.7` → `dosamula_citta_1`
// 3. `moha > 0.7` → `mohamula_citta_1`
// 4. `sati > 0.5` หรือ `metta > 0.5` → `mahakusala_citta_1`
// 5. ไม่เข้าเงื่อนไขใด → `mahakiriya_citta_1`
//
// ⚠️ **ข้อจำกัดที่ต้องรู้:** กฎข้อ 5 (เลือกกิริยาเมื่อไม่มีกิเลส/กุศลชัด)
// เป็น *การทำให้ง่าย* ของแบบจำลอง ไม่ใช่หลักในตำรา — ในความจริงชวนะของปุถุชน
// เป็นกุศลหรืออกุศลเสมอ ส่วนกิริยาเป็นของพระอรหันต์
// และการเลือกวิบากจาก "กิเลสของผู้รับ" ก็เป็นการทำให้ง่ายเช่นกัน
// (วิบากขึ้นกับอารมณ์และกรรม ไม่ใช่กิเลสของจิตที่รับผล)

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

use super::cetasika::{ADOSA, DOSA, LOBHA, MOHA, SATI};
use super::vithi::{CetasikaVector, CittaJati, Vithi15};
use super::vithi17::{CittaJati17, Dvara, Vithi17};

const CITTA_SPEC: &str = include_str!("../../../spec/Paramattha-Citta.json");

/// จำนวนจิตทั้งหมดตามตำรา
pub const CITTA_TOTAL: usize = 121;

#[derive(Debug, Error, PartialEq)]
pub enum MapperError {
    #[error("invalid citta spec: {0}")]
    Spec(String),
    #[error("expected {CITTA_TOTAL} cittas, found {0}")]
    WrongCount(usize),
    #[error("citta `{0}` not found in spec")]
    UnknownCitta(String),
}

// ---- รูปร่างของ spec (เอาเฉพาะ field ที่ใช้) ----

#[derive(Deserialize)]
struct SpecFile {
    citta_121: SpecRoot,
}

#[derive(Deserialize)]
struct SpecRoot {
    kamavacara_citta_54: GroupWithSubgroups,
    rupavacara_citta_15: GroupWithItems,
    arupavacara_citta_12: GroupWithItems,
    lokuttara_citta_8_40: GroupWithItems,
}

#[derive(Deserialize)]
struct GroupWithSubgroups {
    /// JSON object: ชื่อกลุ่มย่อย → เนื้อหา (ไม่ใช่ array)
    subgroups: BTreeMap<String, Subgroup>,
}

#[derive(Deserialize)]
struct Subgroup {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct GroupWithItems {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    id: String,
    #[serde(default)]
    name: String,
}

// ---- ชนิดที่ mapper ใช้ ----

/// ภูมิของจิต
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CittaGroup {
    Kamavacara,
    Rupavacara,
    Arupavacara,
    Lokuttara,
}

impl CittaGroup {
    pub fn id_prefix(&self) -> &'static str {
        match self {
            CittaGroup::Kamavacara => "kamavacara_citta_54",
            CittaGroup::Rupavacara => "rupavacara_citta_15",
            CittaGroup::Arupavacara => "arupavacara_citta_12",
            CittaGroup::Lokuttara => "lokuttara_citta_8_40",
        }
    }

    pub fn name_th(&self) -> &'static str {
        match self {
            CittaGroup::Kamavacara => "กามาวจร",
            CittaGroup::Rupavacara => "รูปาวจร",
            CittaGroup::Arupavacara => "อรูปาวจร",
            CittaGroup::Lokuttara => "โลกุตตระ",
        }
    }
}

/// จิตหนึ่งดวงจากทะเบียน
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CittaEntry {
    pub id: String,
    pub name: String,
    pub group: CittaGroup,
}

impl CittaEntry {
    /// `true` เมื่อ id นี้เป็นของภูมิที่ระบุ
    pub fn in_group(&self, group: CittaGroup) -> bool {
        self.group == group
    }
}

/// ผลวิบากที่ใช้เลือกจิตในขั้นรับผล
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vipaka {
    Kusala,
    Akusala,
}

impl Vipaka {
    pub fn id_suffix(&self) -> &'static str {
        match self {
            Vipaka::Kusala => "kusala_vipaka",
            Vipaka::Akusala => "akusala_vipaka",
        }
    }

    pub fn name_th(&self) -> &'static str {
        match self {
            Vipaka::Kusala => "กุศลวิบาก",
            Vipaka::Akusala => "อกุศลวิบาก",
        }
    }
}

/// ขั้นของวิถีจิต — ใช้ร่วมกันทั้งวิถี 15 และ 17
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CittaStage {
    Bhavanga,
    Pancadvaravajjana,
    Manodvaravajjana,
    Pancavinnana(Dvara),
    Sampaticchana,
    Santirana,
    Votthapana,
    Javana,
    Tadalambana,
}

impl CittaStage {
    pub fn name_th(&self) -> &'static str {
        match self {
            CittaStage::Bhavanga => "ภวังค์",
            CittaStage::Pancadvaravajjana => "ปัญจทวาราวัชชนะ",
            CittaStage::Manodvaravajjana => "มโนทวาราวัชชนะ",
            CittaStage::Pancavinnana(_) => "ปัญจวิญญาณ",
            CittaStage::Sampaticchana => "สัมปฏิจฉนะ",
            CittaStage::Santirana => "สันตีรณะ",
            CittaStage::Votthapana => "โวฏฐัพพนะ",
            CittaStage::Javana => "ชวนะ",
            CittaStage::Tadalambana => "ตทาลัมพนะ",
        }
    }

    /// ขั้นนี้สั่งสมกรรมหรือไม่ — เฉพาะชวนะ
    pub fn accumulates_kamma(&self) -> bool {
        matches!(self, CittaStage::Javana)
    }
}

/// ขณะหนึ่งของวิถีที่แมปเป็นจิตดวงจริงแล้ว
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedMoment {
    pub stage: CittaStage,
    /// ชื่อขั้นแบบ OS (ว่างถ้าแมปจากวิถี 15 ซึ่งไม่มี os_stage)
    pub os_stage: &'static str,
    pub citta_id: String,
    pub citta_name: String,
    pub group: CittaGroup,
}

impl MappedMoment {
    pub fn stage_th(&self) -> &'static str {
        self.stage.name_th()
    }
}

/// วิถีที่แมปครบแล้ว
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappedVithi {
    pub moments: Vec<MappedMoment>,
    pub vipaka: Vipaka,
}

impl MappedVithi {
    pub fn len(&self) -> usize {
        self.moments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moments.is_empty()
    }

    /// รายการ id ของจิตตามลำดับที่เกิด
    pub fn citta_ids(&self) -> Vec<&str> {
        self.moments.iter().map(|m| m.citta_id.as_str()).collect()
    }

    /// จำนวนชวนะที่แมปได้
    pub fn javana_count(&self) -> usize {
        self.moments
            .iter()
            .filter(|m| m.stage == CittaStage::Javana)
            .count()
    }

    /// ชวนะทุกขณะใช้จิตดวงเดียวกันหรือไม่
    ///
    /// เป็น `true` ในแบบจำลองนี้ และตรงกับหลักที่ว่าชวนะทั้ง 7
    /// เป็นจิตดวงเดียวกันเกิดซ้ำ (ยังไม่ได้ตรวจทานกับตำราในรอบนี้)
    pub fn javana_is_uniform(&self) -> bool {
        let mut ids = self
            .moments
            .iter()
            .filter(|m| m.stage == CittaStage::Javana)
            .map(|m| m.citta_id.as_str());
        match ids.next() {
            None => true,
            Some(first) => ids.all(|id| id == first),
        }
    }

    /// จิตดวงเดียวที่ทำหน้าที่สั่งสมกรรมในวิถีนี้
    pub fn javana_citta(&self) -> Option<&MappedMoment> {
        self.moments.iter().find(|m| m.stage == CittaStage::Javana)
    }

    /// ชื่อขั้นแบบ OS ของทุกขณะ
    pub fn os_trace(&self) -> Vec<&'static str> {
        self.moments.iter().map(|m| m.os_stage).collect()
    }
}

/// ตัวแมปขั้นวิถี → จิต 121 ดวง
#[derive(Debug, Clone)]
pub struct VithiMapper {
    entries: Vec<CittaEntry>,
    by_id: BTreeMap<String, usize>,
}

impl VithiMapper {
    /// id ที่ใช้เป็นตัวแทนภวังค์
    pub const BHAVANGA_ID: &'static str = "mahavipaka_citta_1";
    /// id ที่ใช้เป็นตัวแทนตทาลัมพนะ
    pub const TADALAMBANA_ID: &'static str = "mahavipaka_citta_1";
    /// จิตที่ทำหน้าที่โวฏฐัพพนะในวิถีปัญจทวาร
    ///
    /// ในตำรา จิตดวงนี้ (กิริยาอเหตุกะ) ทำหน้าที่ทั้งมโนทวาราวัชชนะ
    /// และโวฏฐัพพนะ จึงใช้ id เดียวกัน
    pub const VOTTHAPANA_ID: &'static str = "manodvaravajjana_citta";
    pub const PANCADVARAVAJJANA_ID: &'static str = "pancadvaravajjana_citta";
    pub const MANODVARAVAJJANA_ID: &'static str = "manodvaravajjana_citta";
    pub const SAMPATICCHANA_KUSALA_ID: &'static str = "sampaticchanat_kusala_vipaka";
    pub const SAMPATICCHANA_AKUSALA_ID: &'static str = "sampaticchanat_akusala_vipaka";
    pub const SANTIRANA_KUSALA_ID: &'static str = "santirana_upeksha_vipaka";
    pub const SANTIRANA_AKUSALA_ID: &'static str = "santirana_akusala_vipaka";
    pub const JAVANA_LOBHA_ID: &'static str = "lobhamula_citta_1";
    pub const JAVANA_DOSA_ID: &'static str = "dosamula_citta_1";
    pub const JAVANA_MOHA_ID: &'static str = "mohamula_citta_1";
    pub const JAVANA_KUSALA_ID: &'static str = "mahakusala_citta_1";
    pub const JAVANA_KIRIYA_ID: &'static str = "mahakiriya_citta_1";

    /// โหลดจาก spec ใน repo
    pub fn canonical() -> Result<Self, MapperError> {
        Self::from_json(CITTA_SPEC)
    }

    /// โหลดจาก JSON string ที่มีรูปร่างเดียวกัน
    pub fn from_json(json: &str) -> Result<Self, MapperError> {
        let spec: SpecFile =
            serde_json::from_str(json).map_err(|e| MapperError::Spec(e.to_string()))?;

        let mut entries = Vec::with_capacity(CITTA_TOTAL);
        let push = |items: Vec<Item>, group: CittaGroup, entries: &mut Vec<CittaEntry>| {
            for item in items {
                entries.push(CittaEntry {
                    id: item.id,
                    name: item.name,
                    group,
                });
            }
        };

        for sg in spec.citta_121.kamavacara_citta_54.subgroups.into_values() {
            push(sg.items, CittaGroup::Kamavacara, &mut entries);
        }
        push(
            spec.citta_121.rupavacara_citta_15.items,
            CittaGroup::Rupavacara,
            &mut entries,
        );
        push(
            spec.citta_121.arupavacara_citta_12.items,
            CittaGroup::Arupavacara,
            &mut entries,
        );
        push(
            spec.citta_121.lokuttara_citta_8_40.items,
            CittaGroup::Lokuttara,
            &mut entries,
        );

        if entries.len() != CITTA_TOTAL {
            return Err(MapperError::WrongCount(entries.len()));
        }

        let mut by_id = BTreeMap::new();
        for (index, entry) in entries.iter().enumerate() {
            by_id.insert(entry.id.clone(), index);
        }

        Ok(Self { entries, by_id })
    }

    /// จิตทั้ง 121 ดวงในทะเบียน
    pub fn all(&self) -> &[CittaEntry] {
        &self.entries
    }

    /// จำนวนจิตในทะเบียน
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn is_registered(&self, id: &str) -> bool {
        self.by_id.contains_key(id)
    }

    /// ดึงจิตตาม id
    pub fn get(&self, id: &str) -> Result<&CittaEntry, MapperError> {
        self.by_id
            .get(id)
            .map(|&i| &self.entries[i])
            .ok_or_else(|| MapperError::UnknownCitta(id.to_string()))
    }

    /// จิตทั้งหมดในภูมิที่ระบุ
    pub fn in_group(&self, group: CittaGroup) -> Vec<&CittaEntry> {
        self.entries.iter().filter(|e| e.group == group).collect()
    }

    /// ตรวจว่าทุก id ที่ mapper จะเลือกมีอยู่จริงในทะเบียน
    ///
    /// คืนรายการ id ที่หายไป (ว่าง = ผ่าน)
    pub fn verify(&self) -> Vec<String> {
        let mut all: Vec<&str> = vec![
            Self::BHAVANGA_ID,
            Self::TADALAMBANA_ID,
            Self::VOTTHAPANA_ID,
            Self::PANCADVARAVAJJANA_ID,
            Self::MANODVARAVAJJANA_ID,
            Self::SAMPATICCHANA_KUSALA_ID,
            Self::SAMPATICCHANA_AKUSALA_ID,
            Self::SANTIRANA_KUSALA_ID,
            Self::SANTIRANA_AKUSALA_ID,
            Self::JAVANA_LOBHA_ID,
            Self::JAVANA_DOSA_ID,
            Self::JAVANA_MOHA_ID,
            Self::JAVANA_KUSALA_ID,
            Self::JAVANA_KIRIYA_ID,
        ];
        all.extend(Dvara::ALL.iter().flat_map(|d| {
            [
                vinana_id(*d, Vipaka::Kusala),
                vinana_id(*d, Vipaka::Akusala),
            ]
        }));
        all.into_iter()
            .filter(|id| !self.is_registered(id))
            .map(str::to_string)
            .collect()
    }

    /// เลือกวิบากจากเจตสิกที่ประกอบ
    ///
    /// ⚠️ เป็นการทำให้ง่าย — ดูหัวข้อ "ข้อจำกัด" ของไฟล์นี้
    pub fn vipaka_for(cetasika: &CetasikaVector) -> Vipaka {
        if cetasika.is_akusala() {
            Vipaka::Akusala
        } else {
            Vipaka::Kusala
        }
    }

    /// เลือกชวนะตามกฎ 5 ข้อในหัวข้อ "หลักการ" ของไฟล์นี้
    pub fn javana_id(cetasika: &CetasikaVector) -> &'static str {
        if cetasika.get(LOBHA) > 0.7 {
            Self::JAVANA_LOBHA_ID
        } else if cetasika.get(DOSA) > 0.7 {
            Self::JAVANA_DOSA_ID
        } else if cetasika.get(MOHA) > 0.7 {
            Self::JAVANA_MOHA_ID
        } else if cetasika.get(SATI) > 0.5 || cetasika.get(ADOSA) > 0.5 {
            Self::JAVANA_KUSALA_ID
        } else {
            Self::JAVANA_KIRIYA_ID
        }
    }

    /// เลือกจิตสำหรับขั้นหนึ่ง
    pub fn resolve(
        &self,
        stage: CittaStage,
        cetasika: &CetasikaVector,
        vipaka: Vipaka,
    ) -> Result<&CittaEntry, MapperError> {
        let id = match stage {
            CittaStage::Bhavanga => Self::BHAVANGA_ID.to_string(),
            CittaStage::Pancadvaravajjana => Self::PANCADVARAVAJJANA_ID.to_string(),
            CittaStage::Manodvaravajjana => Self::MANODVARAVAJJANA_ID.to_string(),
            CittaStage::Pancavinnana(dvara) => vinana_id(dvara, vipaka).to_string(),
            CittaStage::Sampaticchana => match vipaka {
                Vipaka::Kusala => Self::SAMPATICCHANA_KUSALA_ID,
                Vipaka::Akusala => Self::SAMPATICCHANA_AKUSALA_ID,
            }
            .to_string(),
            CittaStage::Santirana => match vipaka {
                Vipaka::Kusala => Self::SANTIRANA_KUSALA_ID,
                Vipaka::Akusala => Self::SANTIRANA_AKUSALA_ID,
            }
            .to_string(),
            CittaStage::Votthapana => Self::VOTTHAPANA_ID.to_string(),
            CittaStage::Javana => Self::javana_id(cetasika).to_string(),
            CittaStage::Tadalambana => Self::TADALAMBANA_ID.to_string(),
        };
        self.get(&id)
    }

    /// แมปวิถี 17 ขณะ
    pub fn map_vithi17(&self, vithi: &Vithi17) -> Result<MappedVithi, MapperError> {
        let vipaka = Self::vipaka_for(
            &vithi
                .moments
                .last()
                .map(|m| m.cetasika())
                .unwrap_or_default(),
        );
        let mut moments = Vec::with_capacity(vithi.moments.len());
        for moment in &vithi.moments {
            let stage = stage_of_17(moment);
            let entry = self.resolve(stage, &moment.cetasika(), vipaka)?;
            moments.push(MappedMoment {
                stage,
                os_stage: moment.os_stage(),
                citta_id: entry.id.clone(),
                citta_name: entry.name.clone(),
                group: entry.group,
            });
        }
        Ok(MappedVithi { moments, vipaka })
    }

    /// แมปวิถี 15 ขณะ
    pub fn map_vithi15(&self, vithi: &Vithi15) -> Result<MappedVithi, MapperError> {
        let vipaka = Self::vipaka_for(
            &vithi
                .moments
                .last()
                .map(|m| m.cetasika())
                .unwrap_or_default(),
        );
        let mut moments = Vec::with_capacity(vithi.moments.len());
        for moment in &vithi.moments {
            let stage = stage_of_15(moment);
            let entry = self.resolve(stage, &moment.cetasika(), vipaka)?;
            moments.push(MappedMoment {
                stage,
                os_stage: "",
                citta_id: entry.id.clone(),
                citta_name: entry.name.clone(),
                group: entry.group,
            });
        }
        Ok(MappedVithi { moments, vipaka })
    }
}

/// id ของปัญจวิญญาณตามทวารและวิบาก
fn vinana_id(dvara: Dvara, vipaka: Vipaka) -> &'static str {
    match (dvara, vipaka) {
        (Dvara::Cakkhu, Vipaka::Kusala) => "cakkhuvinana_kusala_vipaka",
        (Dvara::Cakkhu, Vipaka::Akusala) => "cakkhuvinana_akusala_vipaka",
        (Dvara::Sota, Vipaka::Kusala) => "sotavinana_kusala_vipaka",
        (Dvara::Sota, Vipaka::Akusala) => "sotavinana_akusala_vipaka",
        (Dvara::Ghana, Vipaka::Kusala) => "ghanavinana_kusala_vipaka",
        (Dvara::Ghana, Vipaka::Akusala) => "ghanavinana_akusala_vipaka",
        (Dvara::Jivha, Vipaka::Kusala) => "jivhavinana_kusala_vipaka",
        (Dvara::Jivha, Vipaka::Akusala) => "jivhavinana_akusala_vipaka",
        (Dvara::Kaya, Vipaka::Kusala) => "kayavinana_kusala_vipaka",
        (Dvara::Kaya, Vipaka::Akusala) => "kayavinana_akusala_vipaka",
    }
}

fn stage_of_17(moment: &CittaJati17) -> CittaStage {
    match moment {
        CittaJati17::AtitaBhavanga(_)
        | CittaJati17::BhavangaCalana(_)
        | CittaJati17::Bhavangupaccheda(_) => CittaStage::Bhavanga,
        CittaJati17::Pancadvaravajjana(_) => CittaStage::Pancadvaravajjana,
        CittaJati17::Pancavinnana(dvara, _) => CittaStage::Pancavinnana(*dvara),
        CittaJati17::Sampaticchana(_) => CittaStage::Sampaticchana,
        CittaJati17::Santirana(_) => CittaStage::Santirana,
        CittaJati17::Votthapana(_) => CittaStage::Votthapana,
        CittaJati17::Javana(_) => CittaStage::Javana,
        CittaJati17::Tadalambana(_) => CittaStage::Tadalambana,
    }
}

fn stage_of_15(moment: &CittaJati) -> CittaStage {
    match moment {
        CittaJati::AtitaBhavanga(_)
        | CittaJati::BhavangaCalana(_)
        | CittaJati::Bhavangupaccheda(_)
        | CittaJati::Bhavanga(_) => CittaStage::Bhavanga,
        CittaJati::Manodvaravajjana(_) => CittaStage::Manodvaravajjana,
        CittaJati::Manovinnana(_) => CittaStage::Manodvaravajjana,
        CittaJati::Votthapana(_) => CittaStage::Votthapana,
        CittaJati::Javana(_) => CittaStage::Javana,
        CittaJati::Tadalambana(_) => CittaStage::Tadalambana,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paramattha::vithi17::{GateContext, PdpOutcome};

    fn mapper() -> VithiMapper {
        VithiMapper::canonical().expect("spec ต้องโหลดได้")
    }

    fn akusala_lobha() -> CetasikaVector {
        CetasikaVector::new().with(LOBHA, 0.9).with(MOHA, 0.8)
    }

    #[test]
    fn spec_loads_all_121_cittas() {
        let m = mapper();
        assert_eq!(m.count(), CITTA_TOTAL);
        assert_eq!(m.in_group(CittaGroup::Kamavacara).len(), 54);
        assert_eq!(m.in_group(CittaGroup::Rupavacara).len(), 15);
        assert_eq!(m.in_group(CittaGroup::Arupavacara).len(), 12);
        assert_eq!(m.in_group(CittaGroup::Lokuttara).len(), 40);
    }

    #[test]
    fn every_id_the_mapper_uses_exists_in_the_spec() {
        let missing = mapper().verify();
        assert!(missing.is_empty(), "id ที่หายไป: {missing:?}");
    }

    #[test]
    fn wrong_count_is_rejected() {
        let err = VithiMapper::from_json("{}").unwrap_err();
        assert!(matches!(err, MapperError::Spec(_)));
    }

    #[test]
    fn vithi17_complete_maps_to_17_moments() {
        let m = mapper();
        let vithi = Vithi17::from_external_event(Dvara::Cakkhu, "frame", CetasikaVector::new());
        let mapped = m.map_vithi17(&vithi).unwrap();
        assert_eq!(mapped.len(), 17);
        assert_eq!(mapped.javana_count(), 7);
    }

    #[test]
    fn vithi17_blocked_maps_to_8_moments_and_no_javana() {
        let m = mapper();
        let mut gate = |_: &GateContext<'_>| PdpOutcome::Blocked {
            reason: "blocked".into(),
        };
        let vithi = Vithi17::run(Dvara::Sota, "audio", CetasikaVector::new(), &mut gate);
        let mapped = m.map_vithi17(&vithi).unwrap();
        assert_eq!(mapped.len(), 8);
        assert_eq!(mapped.javana_count(), 0);
        assert!(mapped.javana_citta().is_none());
    }

    #[test]
    fn vithi15_maps_to_15_moments() {
        let m = mapper();
        let vithi = Vithi15::from_internal_log("kernel: Out of memory: Kill process 1");
        let mapped = m.map_vithi15(&vithi).unwrap();
        assert_eq!(mapped.len(), 15);
        assert_eq!(mapped.javana_count(), 7);
    }

    #[test]
    fn javana_follows_the_dominant_defilement() {
        let akusala = VithiMapper::javana_id(&akusala_lobha());
        assert_eq!(akusala, VithiMapper::JAVANA_LOBHA_ID);

        let dosa = CetasikaVector::new().with(DOSA, 0.9);
        assert_eq!(VithiMapper::javana_id(&dosa), VithiMapper::JAVANA_DOSA_ID);

        let moha = CetasikaVector::new().with(MOHA, 0.9);
        assert_eq!(VithiMapper::javana_id(&moha), VithiMapper::JAVANA_MOHA_ID);

        let kusala = CetasikaVector::new().with(SATI, 0.8);
        assert_eq!(
            VithiMapper::javana_id(&kusala),
            VithiMapper::JAVANA_KUSALA_ID
        );

        let neutral = CetasikaVector::new();
        assert_eq!(
            VithiMapper::javana_id(&neutral),
            VithiMapper::JAVANA_KIRIYA_ID
        );
    }

    #[test]
    fn pancavinnana_matches_the_open_dvara() {
        let m = mapper();
        for dvara in Dvara::ALL {
            let vithi = Vithi17::from_external_event(dvara, "e", CetasikaVector::new());
            let mapped = m.map_vithi17(&vithi).unwrap();
            let vinnana = &mapped.moments[4];
            assert_eq!(vinnana.stage, CittaStage::Pancavinnana(dvara));
            assert_eq!(vinnana.citta_id, vinana_id(dvara, Vipaka::Kusala));
        }
    }

    #[test]
    fn akusala_vithi_uses_akusala_vipaka_cittas() {
        let m = mapper();
        let vithi = Vithi17::from_external_event(Dvara::Kaya, "hit", akusala_lobha());
        let mapped = m.map_vithi17(&vithi).unwrap();
        assert_eq!(mapped.vipaka, Vipaka::Akusala);
        assert_eq!(mapped.moments[4].citta_id, "kayavinana_akusala_vipaka");
        assert_eq!(
            mapped.moments[5].citta_id,
            VithiMapper::SAMPATICCHANA_AKUSALA_ID
        );
        assert_eq!(
            mapped.moments[6].citta_id,
            VithiMapper::SANTIRANA_AKUSALA_ID
        );
    }

    #[test]
    fn votthapana_uses_the_manodvaravajjana_citta() {
        let m = mapper();
        let vithi = Vithi17::from_external_event(Dvara::Cakkhu, "frame", CetasikaVector::new());
        let mapped = m.map_vithi17(&vithi).unwrap();
        let pdp = &mapped.moments[Vithi17::PDP_INDEX];
        assert_eq!(pdp.stage, CittaStage::Votthapana);
        assert_eq!(pdp.citta_id, VithiMapper::VOTTHAPANA_ID);
        assert_eq!(pdp.os_stage, "policy_decision_point");
    }

    #[test]
    fn javana_is_uniform_across_the_seven_moments() {
        let m = mapper();
        let vithi = Vithi17::from_external_event(Dvara::Jivha, "taste", akusala_lobha());
        let mapped = m.map_vithi17(&vithi).unwrap();
        assert!(mapped.javana_is_uniform());
        let javana = mapped.javana_citta().unwrap();
        assert_eq!(javana.citta_id, VithiMapper::JAVANA_LOBHA_ID);
        assert_eq!(javana.group, CittaGroup::Kamavacara);
    }

    #[test]
    fn mapped_os_trace_matches_the_vithi_stages() {
        let m = mapper();
        let vithi = Vithi17::from_external_event(Dvara::Ghana, "smell", CetasikaVector::new());
        let mapped = m.map_vithi17(&vithi).unwrap();
        assert_eq!(mapped.os_trace(), vithi.os_trace());
    }

    #[test]
    fn only_javana_accumulates_kamma() {
        assert!(CittaStage::Javana.accumulates_kamma());
        for stage in [
            CittaStage::Bhavanga,
            CittaStage::Pancadvaravajjana,
            CittaStage::Manodvaravajjana,
            CittaStage::Sampaticchana,
            CittaStage::Santirana,
            CittaStage::Votthapana,
            CittaStage::Tadalambana,
        ] {
            assert!(!stage.accumulates_kamma(), "{stage:?}");
        }
    }

    #[test]
    fn unknown_citta_id_is_an_error() {
        let m = mapper();
        let err = m.get("no_such_citta").unwrap_err();
        assert!(matches!(err, MapperError::UnknownCitta(_)));
    }
}
