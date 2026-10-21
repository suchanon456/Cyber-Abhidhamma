// core/src/paramattha/mahabhuta.rs
// Paramattha Silicon - Mahabhuta 4 On-board Firmware
// License: Apache-2.0 - Cyber-Abhidhamma Project

/// Lakkhana Rupa - lifecycle
#[derive(Debug, Clone, Copy)]
pub struct Lakkhana {
    pub uppada: u64,
    pub santati: u64,
    pub jarata: f32,
    pub aniccata: bool,
}

/// Cetasika Vector for sila check
#[derive(Debug, Clone, Copy)]
pub struct CetasikaVector {
    pub lobha: f32,
    pub dosa: f32,
    pub moha: f32,
}

impl CetasikaVector {
    pub fn is_akusala(&self) -> bool {
        self.lobha > 0.7 || self.dosa > 0.7 || self.moha > 0.7
    }
}

pub trait MahabhutaRupa {
    fn what_am_i(&self) -> &'static str;
    fn lakkhana(&self) -> Lakkhana;
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation>;
    fn self_heal(&mut self);
}

#[derive(Debug)]
pub enum DhammaViolation {
    MicchaKammanta(&'static str),
    MicchaAjiva(&'static str),
    JarataTooHigh,
}

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
            lakkhana: Lakkhana { uppada: 0, santati: 0, jarata: 0.0, aniccata: false },
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
    fn lakkhana(&self) -> Lakkhana { self.lakkhana }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        if intent.lobha > 0.85 && intent.dosa > 0.8 {
            return Err(DhammaViolation::MicchaAjiva("Pathavi refuses Lobha+Dosa"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        self.wear_level *= 0.99;
    }
}

pub struct ApoDriver {
    pub cohesion_strength: f32,
}

impl ApoDriver {
    pub const fn new() -> Self {
        Self { cohesion_strength: 1.0 }
    }
}

impl MahabhutaRupa for ApoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Apo-dhatu asmi - water element"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana { uppada: 0, santati: 0, jarata: 1.0 - self.cohesion_strength, aniccata: false }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        if intent.lobha > 0.9 {
            return Err(DhammaViolation::MicchaKammanta("Apo refuses Lobha"));
        }
        Ok(())
    }
    fn self_heal(&mut self) { self.cohesion_strength = 1.0; }
}

pub struct TejoDriver {
    pub temp_c: f32,
}

impl TejoDriver {
    pub const fn new() -> Self {
        Self { temp_c: 30.0 }
    }
}

impl MahabhutaRupa for TejoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Tejo-dhatu asmi - fire element"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana { uppada: 0, santati: 0, jarata: self.temp_c / 100.0, aniccata: self.temp_c > 95.0 }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
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
        Lakkhana { uppada: 0, santati: 0, jarata: 0.0, aniccata: false }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        if intent.moha > 0.85 {
            return Err(DhammaViolation::MicchaAjiva("Vayo refuses Moha"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {}
}

pub fn paramattha_execute(
    drivers: &[&dyn MahabhutaRupa],
    intent: &CetasikaVector,
    _data: &[u8],
) -> Result<(), DhammaViolation> {
    for driver in drivers.iter() {
        driver.sila_check(intent)?;
    }
    Ok(())
}
