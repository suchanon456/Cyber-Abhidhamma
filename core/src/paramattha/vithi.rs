// core/src/paramattha/vithi.rs
// Vithi 15 - Manodvara - Internal self-harm detection
// Safe simulation only

#[derive(Debug, Clone)]
pub struct CetasikaVector {
    pub lobha: f32, pub dosa: f32, pub moha: f32, pub mana: f32,
    pub sati: f32, pub metta: f32,
}
impl CetasikaVector {
    pub fn new() -> Self { Self { lobha: 0.0, dosa: 0.0, moha: 0.0, mana: 0.0, sati: 0.0, metta: 0.0 } }
    pub fn is_akusala(&self) -> bool { self.lobha > 0.7 || self.dosa > 0.7 || self.moha > 0.7 }
}

#[derive(Debug, Clone)]
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
    pub fn name(&self) -> &str {
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
}

pub struct Vithi15 {
    pub moments: Vec<CittaJati>,
    pub source: String,
    pub internal_cause: String,
}

impl Vithi15 {
    pub fn from_internal_log(log: &str) -> Self {
        let mut c = CetasikaVector::new();
        let cause = if log.contains("Out of memory") {
            c.dosa = 0.9; "โทสะภายใน - ฆ่าตัวเองเพราะแรมหมด"
        } else if log.contains("100% full") {
            c.lobha = 0.9; "โลภะภายใน - อยากเก็บจนดิสก์เต็ม"
        } else if log.contains("waiting for") {
            c.moha = 0.85; "โมหะภายใน - Deadlock"
        } else if log.contains("GC") || log.contains("cleanup") {
            c.sati = 0.8; "กุศลภายใน - ทำความสะอาดตัวเอง"
        } else { "กลางๆ - ปกติ" };

        let mut moments = Vec::new();
        moments.push(CittaJati::AtitaBhavanga(CetasikaVector::new()));
        moments.push(CittaJati::BhavangaCalana(CetasikaVector::new()));
        moments.push(CittaJati::Bhavangupaccheda(c.clone()));
        moments.push(CittaJati::Manodvaravajjana(c.clone()));
        moments.push(CittaJati::Manovinnana(c.clone()));
        moments.push(CittaJati::Votthapana(c.clone()));
        for _ in 0..7 { moments.push(CittaJati::Javana(c.clone())); }
        moments.push(CittaJati::Tadalambana(c.clone()));
        moments.push(CittaJati::Tadalambana(c.clone()));

        Self { moments, source: log.to_string(), internal_cause: cause.to_string() }
    }
}
