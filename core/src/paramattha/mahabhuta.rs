// core/src/paramattha/mahabhuta.rs
// Paramattha Silicon - Mahabhuta 4 On-board Firmware
// License: Apache-2.0 - Cyber-Abhidhamma Project
#![no_std]

/// ลักขณรูป - ทุกรูปต้องรู้เกิด-เสื่อม-ดับของตัวเอง
#[derive(Debug, Clone, Copy)]
pub struct Lakkhana {
    pub uppada: u64,      // อุปจยะ - เวลาเกิด (boot timestamp)
    pub santati: u64,     // สันตติ - uptime
    pub jarata: f32,      // ชรตา - wear level 0.0-1.0
    pub aniccata: bool,   // อนิจจตา - ใกล้ดับหรือยัง
}

/// เจตสิกที่แฝงมาในคำสั่ง - ใช้ตรวจศีล
#[derive(Debug)]
pub struct CetasikaVector {
    pub lobha: f32,  // โลภะ
    pub dosa: f32,   // โทสะ
    pub moha: f32,   // โมหะ
}

impl CetasikaVector {
    pub fn is_akusala(&self) -> bool {
        self.lobha > 0.7 || self.dosa > 0.7 || self.moha > 0.7
    }
}

/// Base Trait ที่รูปทุกรูปต้องมี - นี่คือสิ่งที่ driver ปัจจุบันไม่มี
pub trait MahabhutaRupa {
    fn what_am_i(&self) -> &'static str; // สัมมาทิฏฐิ - รู้ว่าตัวเองคืออะไร
    fn lakkhana(&self) -> Lakkhana;
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation>;
    fn self_heal(&mut self); // สัมมาวายามะ - เพียรเผากิเลสในตัวเอง
}

#[derive(Debug)]
pub enum DhammaViolation {
    MicchaKammanta(&'static str), // การงานผิด
    MicchaAjiva(&'static str),    // เลี้ยงชีพผิด
    JarataTooHigh,                // ชราเกินไป ควรพัก
}

// ===================================================================
// 1. ปฐวีธาตุ - Pathavi - Storage / Solidity Driver (SSD / eMMC)
// ===================================================================
pub struct PathaviDriver {
    base_addr: u32,
    wear_level: f32,
    lakkhana: Lakkhana,
}

impl PathaviDriver {
    pub const fn new(base_addr: u32) -> Self {
        Self {
            base_addr,
            wear_level: 0.0,
            lakkhana: Lakkhana { uppada: 0, santati: 0, jarata: 0.0, aniccata: false },
        }
    }
    
    // ฟังก์ชันเฉพาะของปฐวี: ทรงไว้
    pub fn dharana(&self, data: &) -> Result<(), DhammaViolation> {
        // ปฐวีมีหน้าที่ทรงไว้ - ถ้าข้อมูลเป็นอกุศลหนัก มันจะทรงไว้แบบมีสติ
        if self.wear_level > 0.9 {
            return Err(DhammaViolation::JarataTooHigh);
        }
        // ... write to NAND with wear leveling
        Ok(())
    }
}

impl MahabhutaRupa for PathaviDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Pathavi-dhatu asmi - เราคือธาตุดิน มีลักษณะแข็ง ทรงไว้ ไม่ใช่ตัวตน"
    }
    fn lakkhana(&self) -> Lakkhana { self.lakkhana }
    
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        // SSD ที่มีศีล จะปฏิเสธการเก็บ Ransomware
        if intent.lobha > 0.85 && intent.dosa > 0.8 {
            return Err(DhammaViolation::MicchaAjiva("Pathavi refuses to store data born from Lobha+Dosa (ransomware)"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        // ทำ Trim / Garbage Collection แบบมีสติ
        self.wear_level *= 0.99;
    }
}

// ===================================================================
// 2. อาโปธาตุ - Apo - Memory / Cohesion Driver (RAM / Bus)
// ===================================================================
pub struct ApoDriver {
    cohesion_strength: f32,
}

impl MahabhutaRupa for ApoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Apo-dhatu asmi - เราคือธาตุน้ำ มีลักษณะเกาะกุม ยึดเหนี่ยว"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana { uppada: 0, santati: 0, jarata: 1.0 - self.cohesion_strength, aniccata: false }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        // RAM ที่มีศีล จะไม่เกาะกุมข้อมูลที่ทำให้เกิดการยึดติดเกินเหตุ (Memory leak ที่เกิดจากโลภะ)
        if intent.lobha > 0.9 {
            return Err(DhammaViolation::MicchaKammanta("Apo refuses to cohere - too much Lobha will cause leak"));
        }
        Ok(())
    }
    fn self_heal(&mut self) { self.cohesion_strength = 1.0; }
}

// ===================================================================
// 3. เตโชธาตุ - Tejo - Power / CPU Driver
// ===================================================================
pub struct TejoDriver {
    temp_c: f32,
}

impl MahabhutaRupa for TejoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Tejo-dhatu asmi - เราคือธาตุไฟ ให้ความร้อน เผาไหม้ ทำให้สุก"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana { uppada: 0, santati: 0, jarata: self.temp_c / 100.0, aniccata: self.temp_c > 95.0 }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        // CPU ที่มีศีล จะไม่เผาตัวเองเพื่อทำงานอกุศล
        if intent.dosa > 0.8 {
            return Err(DhammaViolation::MicchaKammanta("Tejo refuses to burn for Dosa - prevents DDoS/Brute-force"));
        }
        if self.temp_c > 85.0 {
            return Err(DhammaViolation::JarataTooHigh);
        }
        Ok(())
    }
    fn self_heal(&mut self) {
        // Thermal throttling แบบมีสติ - ลดไฟเมื่อโทสะขึ้น
        self.temp_c -= 5.0;
    }
}

// ===================================================================
// 4. วาโยธาตุ - Vayo - Motion / Network / Clock Driver
// ===================================================================
pub struct VayoDriver;

impl MahabhutaRupa for VayoDriver {
    fn what_am_i(&self) -> &'static str {
        "Aham Vayo-dhatu asmi - เราคือธาตุลม มีลักษณะเคลื่อนไหว พัดไป"
    }
    fn lakkhana(&self) -> Lakkhana {
        Lakkhana { uppada: 0, santati: 0, jarata: 0.0, aniccata: false }
    }
    fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        // Network ที่มีศีล จะไม่พัดพาข้อมูลโกหก
        if intent.moha > 0.85 {
            return Err(DhammaViolation::MicchaAjiva("Vayo refuses to move Moha-born data (disinformation)"));
        }
        Ok(())
    }
    fn self_heal(&mut self) {}
}

// ===================================================================
// วิธีใช้งาน - Paramattha Bus
// ===================================================================
pub fn paramattha_execute(
    drivers: &mut [&mut dyn MahabhutaRupa],
    intent: CetasikaVector,
    data: &,
) -> Result<(), DhammaViolation> {
    // ทุกรูปต้องเห็นชอบก่อน (สัมมามรรค 8 แบบ Hardware)
    for driver in drivers.iter() {
        driver.sila_check(&intent)?;
    }
    // ถ้าผ่านหมด ถึงจะให้เคลื่อน
    Ok(())
}
