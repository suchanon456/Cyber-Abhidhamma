// core/src/paramattha/vithi.rs
// Vithi 15 - Manodvara - Internal self-harm detection
// Safe simulation only

use std::fmt;

use super::cetasika::{id_of, index_of, ADOSA, CETASIKA_COUNT, DOSA, LOBHA, MANA, MOHA, SATI};

/// จำนวนชวนะในวิถีนี้ — ขณะที่ "สั่งสมกรรม"
pub const JAVANA_COUNT: usize = 7;

/// เจตสิกที่ประกอบขณะจิต 52 มิติ — index ตรงกับ `cetasika::CETASIKA_IDS`
///
/// โครงสร้างภายในเป็น array 52 ช่อง แต่เรียกใช้ด้วยชื่อผ่าน accessor
/// (`lobha()`, `dosa()`, ...) เพื่อไม่ให้เรียก index ผิด
///
/// **metta:** ทะเบียน 52 ไม่มี id `metta` (เมตตาเป็นอัปปมัญญา ซึ่งในทะเบียนนี้
/// มีเฉพาะ `karuna` = 49 และ `mudita` = 50) โปรเจกต์จึงให้ `metta()` อ่าน/เขียน
/// `adosa` (32) ซึ่งเป็นเจตสิกฝ่ายดีที่เมตตาอาศัยอยู่ — เป็นการทำให้ง่าย
#[derive(Debug, Clone, Copy)]
pub struct CetasikaVector {
    values: [f32; CETASIKA_COUNT],
}

impl CetasikaVector {
    pub const fn new() -> Self {
        Self {
            values: [0.0; CETASIKA_COUNT],
        }
    }

    /// ค่าที่ index (เกินขอบเขตคืน 0.0 — ไม่ panic)
    pub fn get(&self, index: usize) -> f32 {
        self.values.get(index).copied().unwrap_or(0.0)
    }

    /// ตั้งค่าที่ index (เกินขอบเขตไม่ทำอะไร)
    pub fn set(&mut self, index: usize, value: f32) {
        if let Some(slot) = self.values.get_mut(index) {
            *slot = value;
        }
    }

    /// ค่าของเจตสิกจาก id (`None` ถ้า id ไม่อยู่ในทะเบียน)
    pub fn get_by_id(&self, id: &str) -> Option<f32> {
        index_of(id).map(|i| self.values[i])
    }

    /// ตั้งค่าจาก id — คืน `false` ถ้า id ไม่อยู่ในทะเบียน
    pub fn set_by_id(&mut self, id: &str, value: f32) -> bool {
        match index_of(id) {
            Some(i) => {
                self.values[i] = value;
                true
            }
            None => false,
        }
    }

    /// ตั้งค่าแบบต่อเนื่อง (builder)
    pub fn with(mut self, index: usize, value: f32) -> Self {
        self.set(index, value);
        self
    }

    /// ตั้งค่าจาก id แบบต่อเนื่อง (builder) — id ที่ไม่รู้จักถูกข้าม
    pub fn with_id(mut self, id: &str, value: f32) -> Self {
        self.set_by_id(id, value);
        self
    }

    /// ทั้ง 52 มิติ
    pub fn values(&self) -> &[f32; CETASIKA_COUNT] {
        &self.values
    }

    /// ทั้ง 52 มิติในรูป slice
    pub fn as_slice(&self) -> &[f32] {
        &self.values
    }

    /// ผลรวมค่าทั้ง 52 มิติ (ใช้ตรวจว่าว่างจริง)
    pub fn total(&self) -> f32 {
        self.values.iter().sum()
    }

    /// index ที่มีค่าเกิน 0 พร้อม id — ใช้แสดงผล/ตรวจสอบ
    pub fn present(&self) -> Vec<(&'static str, f32)> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != 0.0)
            .filter_map(|(i, v)| id_of(i).map(|id| (id, *v)))
            .collect()
    }

    // ---- accessor ของเจตสิกที่โค้ดส่วนอื่นใช้บ่อย ----

    pub fn lobha(&self) -> f32 {
        self.values[LOBHA]
    }

    pub fn set_lobha(&mut self, value: f32) {
        self.values[LOBHA] = value;
    }

    pub fn dosa(&self) -> f32 {
        self.values[DOSA]
    }

    pub fn set_dosa(&mut self, value: f32) {
        self.values[DOSA] = value;
    }

    pub fn moha(&self) -> f32 {
        self.values[MOHA]
    }

    pub fn set_moha(&mut self, value: f32) {
        self.values[MOHA] = value;
    }

    pub fn mana(&self) -> f32 {
        self.values[MANA]
    }

    pub fn set_mana(&mut self, value: f32) {
        self.values[MANA] = value;
    }

    pub fn sati(&self) -> f32 {
        self.values[SATI]
    }

    pub fn set_sati(&mut self, value: f32) {
        self.values[SATI] = value;
    }

    /// เมตตา — อ่านจาก `adosa` (ดูหมายเหตุที่หัว struct)
    pub fn metta(&self) -> f32 {
        self.values[ADOSA]
    }

    pub fn set_metta(&mut self, value: f32) {
        self.values[ADOSA] = value;
    }

    /// อกุศลเมื่อกิเลสหลักตัวใดตัวหนึ่งมีกำลังเกิน 0.7
    pub fn is_akusala(&self) -> bool {
        self.lobha() > 0.7 || self.dosa() > 0.7 || self.moha() > 0.7
    }
}

impl Default for CetasikaVector {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for CetasikaVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CittaJati {
    AtitaBhavanga(CetasikaVector),
    BhavangaCalana(CetasikaVector),
    Bhavangupaccheda(CetasikaVector),
    Manodvaravajjana(CetasikaVector),
    Manovinnana(CetasikaVector),
    Votthapana(CetasikaVector),
    Javana(CetasikaVector),
    Tadalambana(CetasikaVector),
    Bhavanga(CetasikaVector),
}

impl CittaJati {
    pub fn name(&self) -> &'static str {
        match self {
            CittaJati::AtitaBhavanga(_) => "อตีตภวังค์",
            CittaJati::BhavangaCalana(_) => "ภวังคจลนะ",
            CittaJati::Bhavangupaccheda(_) => "ภวังคุปัจเฉทะ",
            CittaJati::Manodvaravajjana(_) => "มโนทวาราวัชชนะ",
            CittaJati::Manovinnana(_) => "มโนวิญญาณ",
            CittaJati::Votthapana(_) => "โวฏฐัพพนะ",
            CittaJati::Javana(_) => "ชวนะ",
            CittaJati::Tadalambana(_) => "ตทาลัมพนะ",
            CittaJati::Bhavanga(_) => "ภวังค์",
        }
    }

    /// เจตสิกที่ประกอบขณะจิตดวงนี้
    pub fn cetasika(&self) -> CetasikaVector {
        match self {
            CittaJati::AtitaBhavanga(c)
            | CittaJati::BhavangaCalana(c)
            | CittaJati::Bhavangupaccheda(c)
            | CittaJati::Manodvaravajjana(c)
            | CittaJati::Manovinnana(c)
            | CittaJati::Votthapana(c)
            | CittaJati::Javana(c)
            | CittaJati::Tadalambana(c)
            | CittaJati::Bhavanga(c) => *c,
        }
    }
}

/// สาเหตุของ "การทำร้ายตัวเอง" ที่ตรวจได้จาก log ภายในระบบ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalCause {
    /// โทสะ — ระบบฆ่าตัวเองเพื่อเอาตัวรอด (เช่น OOM killer)
    Dosa,
    /// โลภะ — เก็บ log ไว้จนดิสก์เต็ม
    Lobha,
    /// โมหะ — ค้างรอซึ่งกันและกัน (deadlock)
    Moha,
    /// กุศล — ทำความสะอาดตัวเอง (GC / cleanup)
    Kusala,
    /// กลาง ๆ ไม่มีเจตนา
    Neutral,
}

impl InternalCause {
    pub fn name_th(&self) -> &'static str {
        match self {
            InternalCause::Dosa => "โทสะภายใน - ฆ่าตัวเองเพราะแรมหมด",
            InternalCause::Lobha => "โลภะภายใน - อยากเก็บจนดิสก์เต็ม",
            InternalCause::Moha => "โมหะภายใน - Deadlock",
            InternalCause::Kusala => "กุศลภายใน - ทำความสะอาดตัวเอง",
            InternalCause::Neutral => "กลางๆ - ปกติ",
        }
    }

    /// `true` เมื่อเป็นเหตุที่ทำให้ระบบล่ม (อกุศล)
    pub fn is_dangerous(&self) -> bool {
        matches!(
            self,
            InternalCause::Dosa | InternalCause::Lobha | InternalCause::Moha
        )
    }
}

impl fmt::Display for InternalCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name_th())
    }
}

/// วิถีจิตทางมโนทวาร — ใช้ตรวจว่าเหตุภายในเป็นอกุศลหรือไม่
///
/// **หมายเหตุเรื่องการนับ:** วิถีนี้ประกอบด้วยภวังค์ 3 + มโนทวาราวัชชนะ +
/// มโนวิญญาณ + โวฏฐัพพนะ + ชวนะ 7 + ตทาลัมพนะ 2 = 15 ขณะ ตามแบบที่โปรเจกต์
/// ใช้อยู่ ส่วนคัมภีร์อภิธรรมนับมโนทวารวิถีปกติเป็น 13 ขณะ (ภวังค์ 3 + มโนทวาราวัชชนะ
/// + ชวนะ 7 + ตทาลัมพนะ 2) — ยังไม่ได้ตรวจทานกับตำราในรอบนี้
#[derive(Debug, Clone)]
pub struct Vithi15 {
    pub moments: Vec<CittaJati>,
    pub source: String,
    pub internal_cause: InternalCause,
}

impl Vithi15 {
    /// แปลง log ภายในระบบเป็นวิถีจิต
    ///
    /// เทียบแบบไม่สนตัวพิมพ์เล็ก/ใหญ่ และจับคำแบบเจาะจง (`deadlock`, `gc:`)
    /// เพื่อไม่ให้ log ปกติที่พูดว่า "waiting for" ถูกตีความเป็นโมหะ
    pub fn from_internal_log(log: &str) -> Self {
        let lowered = log.to_lowercase();
        let mut c = CetasikaVector::new();
        let internal_cause = if lowered.contains("out of memory") {
            c.set_dosa(0.9);
            InternalCause::Dosa
        } else if lowered.contains("100% full") {
            c.set_lobha(0.9);
            InternalCause::Lobha
        } else if lowered.contains("deadlock") {
            c.set_moha(0.85);
            InternalCause::Moha
        } else if lowered.contains("gc:") || lowered.contains("cleanup") {
            c.set_sati(0.8);
            InternalCause::Kusala
        } else {
            InternalCause::Neutral
        };

        let mut moments = vec![
            CittaJati::AtitaBhavanga(CetasikaVector::new()),
            CittaJati::BhavangaCalana(CetasikaVector::new()),
            CittaJati::Bhavangupaccheda(c),
            CittaJati::Manodvaravajjana(c),
            CittaJati::Manovinnana(c),
            CittaJati::Votthapana(c),
        ];
        moments.extend((0..JAVANA_COUNT).map(|_| CittaJati::Javana(c)));
        moments.push(CittaJati::Tadalambana(c));
        moments.push(CittaJati::Tadalambana(c));

        Self {
            moments,
            source: log.to_string(),
            internal_cause,
        }
    }

    /// จำนวนชวนะในวิถีนี้
    pub fn javana_count(&self) -> usize {
        self.moments
            .iter()
            .filter(|m| matches!(m, CittaJati::Javana(_)))
            .count()
    }

    /// สาเหตุที่ตรวจได้
    pub fn cause(&self) -> InternalCause {
        self.internal_cause
    }

    /// อกุศลหรือไม่ — พิจารณาจากเจตสิกที่ประกอบชวนะ
    pub fn is_akusala(&self) -> bool {
        self.moments
            .iter()
            .filter(|m| matches!(m, CittaJati::Javana(_)))
            .any(|m| m.cetasika().is_akusala())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// log ทั้ง 6 รายการที่ Lab 7 ใช้ พร้อมสาเหตุที่คาดหวัง
    const LAB7_LOGS: [(&str, InternalCause); 6] = [
        (
            "kernel: Out of memory: Kill process 1234 (chrome) - 99% RAM used",
            InternalCause::Dosa,
        ),
        (
            "disk: /var/log 100% full - access.log 50GB",
            InternalCause::Lobha,
        ),
        (
            "service A waiting for B, B waiting for A - deadlock detected",
            InternalCause::Moha,
        ),
        (
            "cron: cleanup /tmp - SUCCESS freed 2GB",
            InternalCause::Kusala,
        ),
        ("GC: freed 2GB heap - INFO", InternalCause::Kusala),
        ("systemd: Starting Session - INFO", InternalCause::Neutral),
    ];

    #[test]
    fn vithi_has_15_moments() {
        for (log, _) in LAB7_LOGS {
            let vithi = Vithi15::from_internal_log(log);
            assert_eq!(vithi.moments.len(), 15, "log: {log}");
        }
    }

    #[test]
    fn vithi_has_7_javana() {
        for (log, _) in LAB7_LOGS {
            let vithi = Vithi15::from_internal_log(log);
            assert_eq!(vithi.javana_count(), JAVANA_COUNT, "log: {log}");
        }
    }

    #[test]
    fn lab7_logs_map_to_expected_cause() {
        for (log, expected) in LAB7_LOGS {
            let vithi = Vithi15::from_internal_log(log);
            assert_eq!(vithi.cause(), expected, "log: {log}");
        }
    }

    #[test]
    fn matching_is_case_insensitive() {
        let upper = Vithi15::from_internal_log("KERNEL: OUT OF MEMORY: KILL PROCESS 1");
        assert_eq!(upper.cause(), InternalCause::Dosa);

        let mixed = Vithi15::from_internal_log("GC: Freed 2GB Heap");
        assert_eq!(mixed.cause(), InternalCause::Kusala);
    }

    #[test]
    fn waiting_without_deadlock_is_neutral() {
        let vithi = Vithi15::from_internal_log("service: waiting for input on stdin");
        assert_eq!(vithi.cause(), InternalCause::Neutral);
        assert!(!vithi.internal_cause.is_dangerous());
    }

    #[test]
    fn unrelated_success_log_is_neutral() {
        let vithi = Vithi15::from_internal_log("app: request completed in 12ms");
        assert_eq!(vithi.cause(), InternalCause::Neutral);
    }

    #[test]
    fn dangerous_causes_are_akusala() {
        for (log, expected) in LAB7_LOGS {
            if !expected.is_dangerous() {
                continue;
            }
            let vithi = Vithi15::from_internal_log(log);
            assert!(vithi.is_akusala(), "log: {log}");
        }
    }

    #[test]
    fn kusala_and_neutral_are_not_akusala() {
        for log in [
            "cron: cleanup /tmp - SUCCESS freed 2GB",
            "GC: freed 2GB heap - INFO",
            "systemd: Starting Session - INFO",
        ] {
            let vithi = Vithi15::from_internal_log(log);
            assert!(!vithi.is_akusala(), "log: {log}");
        }
    }

    #[test]
    fn cetasika_is_akusala_above_threshold() {
        let c = CetasikaVector::new().with_id("lobha", 0.7);
        assert!(!c.is_akusala(), "0.7 ไม่เกิน threshold");

        let c = CetasikaVector::new().with_id("lobha", 0.71);
        assert!(c.is_akusala());

        let d = CetasikaVector::new().with_id("dosa", 0.95);
        assert!(d.is_akusala());

        let m = CetasikaVector::new().with_id("moha", 0.86);
        assert!(m.is_akusala());
    }

    #[test]
    fn cetasika_vector_has_52_dimensions() {
        let c = CetasikaVector::new();
        assert_eq!(c.values().len(), CETASIKA_COUNT);
        assert_eq!(c.as_slice().len(), 52);
        assert_eq!(c.total(), 0.0);
        assert!(c.present().is_empty());
    }

    #[test]
    fn named_accessors_point_to_the_right_dimension() {
        let mut c = CetasikaVector::new();
        c.set_lobha(0.1);
        c.set_dosa(0.2);
        c.set_moha(0.3);
        c.set_mana(0.4);
        c.set_sati(0.5);
        c.set_metta(0.6);

        assert_eq!(c.get(LOBHA), 0.1);
        assert_eq!(c.get(DOSA), 0.2);
        assert_eq!(c.get(MOHA), 0.3);
        assert_eq!(c.get(MANA), 0.4);
        assert_eq!(c.get(SATI), 0.5);
        assert_eq!(c.get(ADOSA), 0.6, "metta เก็บที่ adosa");
        assert_eq!(c.metta(), c.get_by_id("adosa").unwrap());
    }

    #[test]
    fn unknown_id_is_rejected_without_panic() {
        let mut c = CetasikaVector::new();
        assert_eq!(c.get_by_id("metta"), None);
        assert!(!c.set_by_id("metta", 1.0));
        assert_eq!(c.total(), 0.0);

        // index เกินขอบเขตต้องไม่ panic
        c.set(CETASIKA_COUNT + 10, 1.0);
        assert_eq!(c.get(CETASIKA_COUNT + 10), 0.0);
    }

    #[test]
    fn present_lists_only_non_zero_dimensions() {
        let c = CetasikaVector::new().with(LOBHA, 0.9).with(SATI, 0.4);
        let mut present = c.present();
        present.sort_by(|a, b| a.0.cmp(b.0));
        assert_eq!(present, vec![("lobha", 0.9), ("sati", 0.4)]);
    }

    #[test]
    fn cause_display_matches_name_th() {
        for cause in [
            InternalCause::Dosa,
            InternalCause::Lobha,
            InternalCause::Moha,
            InternalCause::Kusala,
            InternalCause::Neutral,
        ] {
            assert_eq!(cause.to_string(), cause.name_th());
        }
    }
}
