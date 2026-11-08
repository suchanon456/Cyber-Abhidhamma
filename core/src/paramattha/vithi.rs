// core/src/paramattha/vithi.rs
// Vithi 15 - Manodvara - Internal self-harm detection
// Safe simulation only

use std::fmt;

/// จำนวนชวนะในวิถีนี้ — ขณะที่ "สั่งสมกรรม"
pub const JAVANA_COUNT: usize = 7;

/// เจตสิกที่ประกอบขณะจิต — ใช้ตัดสินว่าเจตนาเป็นกุศลหรืออกุศล
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CetasikaVector {
    pub lobha: f32,
    pub dosa: f32,
    pub moha: f32,
    pub mana: f32,
    pub sati: f32,
    pub metta: f32,
}

impl CetasikaVector {
    pub fn new() -> Self {
        Self {
            lobha: 0.0,
            dosa: 0.0,
            moha: 0.0,
            mana: 0.0,
            sati: 0.0,
            metta: 0.0,
        }
    }

    /// อกุศลเมื่อกิเลสตัวใดตัวหนึ่งมีกำลังเกิน 0.7
    pub fn is_akusala(&self) -> bool {
        self.lobha > 0.7 || self.dosa > 0.7 || self.moha > 0.7
    }
}

impl Default for CetasikaVector {
    fn default() -> Self {
        Self::new()
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
            c.dosa = 0.9;
            InternalCause::Dosa
        } else if lowered.contains("100% full") {
            c.lobha = 0.9;
            InternalCause::Lobha
        } else if lowered.contains("deadlock") {
            c.moha = 0.85;
            InternalCause::Moha
        } else if lowered.contains("gc:") || lowered.contains("cleanup") {
            c.sati = 0.8;
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
        let c = CetasikaVector {
            lobha: 0.7,
            ..Default::default()
        };
        assert!(!c.is_akusala(), "0.7 ไม่เกิน threshold");

        let c = CetasikaVector {
            lobha: 0.71,
            ..Default::default()
        };
        assert!(c.is_akusala());

        let d = CetasikaVector {
            dosa: 0.95,
            ..Default::default()
        };
        assert!(d.is_akusala());

        let m = CetasikaVector {
            moha: 0.86,
            ..Default::default()
        };
        assert!(m.is_akusala());
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
