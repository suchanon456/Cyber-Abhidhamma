// core/src/paramattha/mahabhuta.rs
// Paramattha Silicon - Mahabhuta 4 On-board Firmware
// License: Apache-2.0 - Cyber-Abhidhamma Project

use thiserror::Error;

/// Lakkhana Rupa - lifecycle
#[derive(Debug, Clone, Copy)]
pub struct Lakkhana {
    pub uppada: u64,
    pub santati: u64,
    pub jarata: f32,
    pub aniccata: bool,
}

/// Cetasika Vector for sila check
#[derive(Debug, Clone, Copy, Default)]
pub struct CetasikaVectorMahabhuta {
    pub lobha: f32,
    pub dosa: f32,
    pub moha: f32,
}

impl CetasikaVectorMahabhuta {
    pub fn is_akusala(&self) -> bool {
        self.lobha > 0.7 || self.dosa > 0.7 || self.moha > 0.7
    }
}

pub trait MahabhutaRupa {
    fn what_am_i(&self) -> &'static str;
    fn lakkhana(&self) -> Lakkhana;
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation>;
    fn self_heal(&mut self);
}

/// การละเมิดศีลที่ธาตุ (driver) ตรวจพบและปฏิเสธ
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DhammaViolation {
    #[error("micchā-kammanta: {0}")]
    MicchaKammanta(&'static str),
    #[error("micchā-ājīva: {0}")]
    MicchaAjiva(&'static str),
    #[error("jaratā too high: rūpa has aged past threshold")]
    JarataTooHigh,
}

#[derive(Debug, Clone)]
pub struct PathaviDriver {
    pub base_addr: u32,
    pub wear_level: f32,
    pub lakkhana: Lakkhana,
}

impl PathaviDriver {
    pub const fn new(base_addr: u32) -> Self {
        Self {
            base_addr,
            wear_level: 0.0,
            lakkhana: Lakkhana {
                uppada: 0,
                santati: 0,
                jarata: 0.0,
                aniccata: false,
            },
        }
    }
    pub fn dharana(&self, _data: &[u8]) -> Result<(), DhammaViolation> {
        if self.wear_level > 0.9 {
            return Err(DhammaViolation::JarataTooHigh);
        }
        Ok(())
    }
}

impl MahabhutaRupa for PathaviDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Pathavi-dhatu asmi - earth element"
    }
    fn lakkhana(&self) -> Lakkhana {
        self.lakkhana
    }
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation> {
        if intent.lobha > 0.85 && intent.dosa > 0.8 {
            return Err(DhammaViolation::MicchaAjiva("Pathavi refuses Lobha+Dosa"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        self.wear_level *= 0.99;
    }
}

#[derive(Debug, Clone)]
pub struct ApoDriver {
    pub cohesion_strength: f32,
}

impl ApoDriver {
    pub const fn new() -> Self {
        Self {
            cohesion_strength: 1.0,
        }
    }
}

impl Default for ApoDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl MahabhutaRupa for ApoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Apo-dhatu asmi - water element"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana {
            uppada: 0,
            santati: 0,
            jarata: 1.0 - self.cohesion_strength,
            aniccata: false,
        }
    }
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation> {
        if intent.lobha > 0.9 {
            return Err(DhammaViolation::MicchaKammanta("Apo refuses Lobha"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        self.cohesion_strength = 1.0;
    }
}

#[derive(Debug, Clone)]
pub struct TejoDriver {
    pub temp_c: f32,
}

impl TejoDriver {
    pub const fn new() -> Self {
        Self { temp_c: 30.0 }
    }
}

impl Default for TejoDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl MahabhutaRupa for TejoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Tejo-dhatu asmi - fire element"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana {
            uppada: 0,
            santati: 0,
            jarata: self.temp_c / 100.0,
            aniccata: self.temp_c > 95.0,
        }
    }
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation> {
        if intent.dosa > 0.8 {
            return Err(DhammaViolation::MicchaKammanta("Tejo refuses Dosa"));
        }
        if self.temp_c > 85.0 {
            return Err(DhammaViolation::JarataTooHigh);
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        self.temp_c -= 5.0;
    }
}

pub struct VayoDriver;

impl MahabhutaRupa for VayoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Vayo-dhatu asmi - wind element"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana {
            uppada: 0,
            santati: 0,
            jarata: 0.0,
            aniccata: false,
        }
    }
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation> {
        if intent.moha > 0.85 {
            return Err(DhammaViolation::MicchaAjiva("Vayo refuses Moha"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {}
}

pub fn paramattha_execute(
    drivers: &[&dyn MahabhutaRupa],
    intent: &CetasikaVectorMahabhuta,
    _data: &[u8],
) -> Result<(), DhammaViolation> {
    for driver in drivers.iter() {
        driver.sila_check(intent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intent(lobha: f32, dosa: f32, moha: f32) -> CetasikaVectorMahabhuta {
        CetasikaVectorMahabhuta { lobha, dosa, moha }
    }

    #[test]
    fn neutral_intent_passes_all_four_elements() {
        let i = CetasikaVectorMahabhuta::default();
        let pathavi = PathaviDriver::new(0x1000);
        let apo = ApoDriver::new();
        let tejo = TejoDriver::new();
        let vayo = VayoDriver;
        let drivers: [&dyn MahabhutaRupa; 4] = [&pathavi, &apo, &tejo, &vayo];

        assert_eq!(paramattha_execute(&drivers, &i, b""), Ok(()));
    }

    #[test]
    fn tejo_refuses_dosa() {
        let tejo = TejoDriver::new();
        assert_eq!(
            tejo.sila_check(&intent(0.0, 0.9, 0.0)),
            Err(DhammaViolation::MicchaKammanta("Tejo refuses Dosa"))
        );
    }

    #[test]
    fn tejo_rejects_overheating_rupa() {
        let hot = TejoDriver { temp_c: 86.0 };
        assert_eq!(
            hot.sila_check(&intent(0.0, 0.0, 0.0)),
            Err(DhammaViolation::JarataTooHigh)
        );

        let borderline = TejoDriver { temp_c: 85.0 };
        assert_eq!(borderline.sila_check(&intent(0.0, 0.0, 0.0)), Ok(()));
    }

    #[test]
    fn apo_refuses_lobha() {
        let apo = ApoDriver::new();
        assert_eq!(
            apo.sila_check(&intent(0.95, 0.0, 0.0)),
            Err(DhammaViolation::MicchaKammanta("Apo refuses Lobha"))
        );
        assert_eq!(apo.sila_check(&intent(0.9, 0.0, 0.0)), Ok(()));
    }

    #[test]
    fn vayo_refuses_moha() {
        let vayo = VayoDriver;
        assert_eq!(
            vayo.sila_check(&intent(0.0, 0.0, 0.9)),
            Err(DhammaViolation::MicchaAjiva("Vayo refuses Moha"))
        );
        assert_eq!(vayo.sila_check(&intent(0.0, 0.0, 0.85)), Ok(()));
    }

    #[test]
    fn pathavi_refuses_lobha_with_dosa() {
        let pathavi = PathaviDriver::new(0x1000);
        assert_eq!(
            pathavi.sila_check(&intent(0.9, 0.85, 0.0)),
            Err(DhammaViolation::MicchaAjiva("Pathavi refuses Lobha+Dosa"))
        );
        // โลภะอย่างเดียวไม่พอ — ต้องมีโทสะด้วย
        assert_eq!(pathavi.sila_check(&intent(0.9, 0.0, 0.0)), Ok(()));
    }

    #[test]
    fn pathavi_dharana_checks_wear_level() {
        let mut pathavi = PathaviDriver::new(0x1000);
        assert_eq!(pathavi.dharana(b"data"), Ok(()));

        pathavi.wear_level = 0.95;
        assert_eq!(
            pathavi.dharana(b"data"),
            Err(DhammaViolation::JarataTooHigh)
        );
    }

    #[test]
    fn self_heal_reduces_jarata() {
        let mut tejo = TejoDriver { temp_c: 86.0 };
        tejo.self_heal();
        assert_eq!(tejo.temp_c, 81.0);
        assert_eq!(tejo.sila_check(&intent(0.0, 0.0, 0.0)), Ok(()));

        let mut pathavi = PathaviDriver::new(0x1000);
        pathavi.wear_level = 1.0;
        pathavi.self_heal();
        assert!((pathavi.wear_level - 0.99).abs() < f32::EPSILON);

        let mut apo = ApoDriver {
            cohesion_strength: 0.2,
        };
        apo.self_heal();
        assert_eq!(apo.cohesion_strength, 1.0);
    }

    #[test]
    fn paramattha_execute_returns_first_violation() {
        let hot = TejoDriver { temp_c: 90.0 };
        let apo = ApoDriver::new();
        let drivers: [&dyn MahabhutaRupa; 2] = [&hot, &apo];

        // lobha สูงจะชน Apo แต่ Tejo ถูกตรวจก่อนและคืน Err ตัวแรก
        assert_eq!(
            paramattha_execute(&drivers, &intent(0.95, 0.0, 0.0), b""),
            Err(DhammaViolation::JarataTooHigh)
        );
    }

    #[test]
    fn violation_display_is_readable() {
        assert_eq!(
            DhammaViolation::MicchaKammanta("Apo refuses Lobha").to_string(),
            "micchā-kammanta: Apo refuses Lobha"
        );
        assert_eq!(
            DhammaViolation::JarataTooHigh.to_string(),
            "jaratā too high: rūpa has aged past threshold"
        );
    }
}
