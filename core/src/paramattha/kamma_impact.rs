// core/src/paramattha/kamma_impact.rs
// Kamma Impact — ให้ "ชวนะอกุศล" ส่งผลต่อรูป (RupaDriver) 4 ธาตุ
//
// ## หลักการ
//
// ในวิถีจิต มีเพียง **ชวนะ 7 ขณะ** ที่สั่งสมกรรม (`CittaStage::accumulates_kamma`)
// ขั้นอื่นเป็นเพียงวิบาก/กิริยา ไม่สร้างกรรมใหม่ ไฟล์นี้จึงอ่านเจตสิก **เฉพาะชวนะ**
// แล้วแปลงเป็นผลกระทบต่อธาตุทั้ง 4 ที่จำลองไว้ใน `mahabhuta.rs`
//
// | ธาตุ (รูป) | ตัวแปรที่เปลี่ยน | เจตสิกที่ทำให้เสื่อม | เหตุผลเชิงอุปมา |
// |---|---|---|---|
// | ปฐวี (ความคงรูป) | `PathaviDriver.wear_level` ↑ | โลภะ + โทสะ + โมหะ | อกุศลทุกชนิดสึกกร่อนความคงรูป |
// | อาโป (ความเกาะกุม) | `ApoDriver.cohesion_strength` ↓ | โลภะ | ยึดติดเกินเหตุจนโครงสร้างไม่เกาะกุม |
// | เตโช (ความร้อน) | `TejoDriver.temp_c` ↑ | โทสะ | โทสะคือความร้อน เผาตัวเอง |
// | วาโย (การเคลื่อน) | `RupaCluster.vayo_turbulence` ↑ | โมหะ | โมหะคือความฟุ้ง/สับสนของสัญญาณ |
//
// กุศลชวนะ (สติ/เมตตา) ให้ผลตรงข้าม คือทำให้เย็นลง เกาะกุมคืน คลายความฟุ้ง
// และบรรเทาความสึกกร่อน
//
// ⚠️ **ข้อจำกัด**
// - ทั้งตารางเป็นการ **อุปมา (analogy)** ตามกติกาใน `docs/GLOSSARY.md` ไม่ใช่หลักในตำรา
//   ว่าโลภะทำให้ RAM เสียจริง
// - น้ำหนักตัวเลข (`HEAT_PER_DOSA` ฯลฯ) ตั้งขึ้นเพื่อให้ทดสอบได้ ไม่ได้มาจากคัมภีร์
// - `VayoDriver` ใน `mahabhuta.rs` เป็น unit struct ไม่มีสถานะ จึงเก็บความฟุ้งไว้ที่
//   `RupaCluster.vayo_turbulence` แทนการแก้ struct เดิม
// - หนึ่งวิถีคิดเป็นหนึ่งเหตุการณ์: ผลรวมของชวนะถูกหารด้วย `JAVANA_COUNT`
//   เพื่อไม่ให้ความรุนแรงคูณเจ็ด

use super::cetasika::{ADOSA, DOSA, LOBHA, MOHA, SATI};
use super::mahabhuta::{
    ApoDriver, CetasikaVectorMahabhuta, DhammaViolation, MahabhutaRupa, PathaviDriver, TejoDriver,
};
use super::vithi::{CetasikaVector, CittaJati, Vithi15, JAVANA_COUNT};
use super::vithi17::{CittaJati17, Vithi17};

/// อุณหภูมิเริ่มต้นของเตโช (องศาเซลเซียส) — เท่ากับ `TejoDriver::new()`
pub const AMBIENT_C: f32 = 30.0;
/// ความร้อนที่เพิ่มขึ้นต่อโทสะหนึ่งหน่วยเต็ม (1.0) ในหนึ่งวิถี
pub const HEAT_PER_DOSA: f32 = 40.0;
/// ความเกาะกุมที่สูญเสียต่อโลภะหนึ่งหน่วยเต็มในหนึ่งวิถี
pub const COHESION_LOSS_PER_LOBHA: f32 = 0.35;
/// ความสึกกร่อนต่อผลรวมอกุศล (โลภะ+โทสะ+โมหะ) หนึ่งหน่วยเต็มในหนึ่งวิถี
pub const WEAR_PER_AKUSALA: f32 = 0.05;
/// ความฟุ้งของวาโยต่อโมหะหนึ่งหน่วยเต็มในหนึ่งวิถี
pub const TURBULENCE_PER_MOHA: f32 = 1.0;
/// ความเย็นที่สติหนึ่งหน่วยเต็มให้ได้ในหนึ่งวิถี
pub const COOLING_PER_SATI: f32 = 10.0;
/// ความเกาะกุมที่เมตตาช่วยฟื้นคืนต่อหนึ่งหน่วยเต็มในหนึ่งวิถี
pub const RECOVERY_PER_METTA: f32 = 0.2;
/// ความสึกกร่อนที่ถูกบรรเทาด้วยกุศล (สติ+เมตตา) ต่อหนึ่งหน่วยเต็มในหนึ่งวิถี
pub const RELIEF_PER_KUSALA: f32 = 0.01;
/// ความฟุ้งของวาโยที่ถูกคลายด้วยกุศล (สติ+เมตตา) ต่อหนึ่งหน่วยเต็มในหนึ่งวิถี
pub const CALM_PER_KUSALA: f32 = 0.5;

/// ผลกระทบของกรรมหนึ่งวิถีต่อธาตุทั้ง 4
///
/// ค่าที่ทำให้เสื่อม (`pathavi_wear`, `apo_cohesion_loss`, `tejo_heat`,
/// `vayo_turbulence`) เป็นบวกเสมอ ส่วนค่าที่ทำให้ฟื้น (`*_recovery`,
/// `*_cooling`, `*_relief`, `vayo_calm`) ก็เป็นบวกและถูกลบออกตอน `apply`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RupaImpact {
    /// ความสึกกร่อนของปฐวีที่เพิ่มขึ้น
    pub pathavi_wear: f32,
    /// ความเกาะกุมของอาโปที่สูญเสีย
    pub apo_cohesion_loss: f32,
    /// ความร้อนของเตโชที่เพิ่มขึ้น (องศาเซลเซียส)
    pub tejo_heat: f32,
    /// ความฟุ้งของวาโยที่เพิ่มขึ้น
    pub vayo_turbulence: f32,
    /// ความเย็นที่เตโชได้รับคืน (องศาเซลเซียส)
    pub tejo_cooling: f32,
    /// ความเกาะกุมที่อาโปได้คืน
    pub apo_recovery: f32,
    /// ความสึกกร่อนที่ปฐวีถูกบรรเทา
    pub pathavi_relief: f32,
    /// ความฟุ้งของวาโยที่ถูกคลาย
    pub vayo_calm: f32,
    /// จำนวนชวนะที่ใช้คำนวณ (0 = ไม่มีชวนะ = ไม่มีผล)
    pub javana_moments: usize,
}

impl RupaImpact {
    /// ไม่มีผลกระทบเลย
    pub fn none() -> Self {
        Self::default()
    }

    /// `true` เมื่อไม่มีชวนะอยู่เลย (เช่น วิถีถูกตัดที่โวฏฐัพพนะ)
    pub fn is_empty(&self) -> bool {
        self.javana_moments == 0
    }

    /// `true` เมื่อผลนี้ทำให้รูปเสื่อมสุทธิ (อกุศลมากกว่ากุศล)
    pub fn is_harmful(&self) -> bool {
        self.pathavi_wear > self.pathavi_relief
            || self.apo_cohesion_loss > self.apo_recovery
            || self.tejo_heat > self.tejo_cooling
            || self.vayo_turbulence > self.vayo_calm
    }

    /// ผลกระทบของเจตสิก **หนึ่งขณะชวนะ** (ยังไม่หารด้วยจำนวนชวนะ)
    fn from_cetasika(cetasika: &CetasikaVector) -> Self {
        let lobha = cetasika.get(LOBHA).max(0.0);
        let dosa = cetasika.get(DOSA).max(0.0);
        let moha = cetasika.get(MOHA).max(0.0);
        let sati = cetasika.get(SATI).max(0.0);
        let metta = cetasika.get(ADOSA).max(0.0);

        Self {
            pathavi_wear: WEAR_PER_AKUSALA * (lobha + dosa + moha),
            apo_cohesion_loss: COHESION_LOSS_PER_LOBHA * lobha,
            tejo_heat: HEAT_PER_DOSA * dosa,
            vayo_turbulence: TURBULENCE_PER_MOHA * moha,
            tejo_cooling: COOLING_PER_SATI * sati,
            apo_recovery: RECOVERY_PER_METTA * metta,
            pathavi_relief: RELIEF_PER_KUSALA * (sati + metta),
            vayo_calm: CALM_PER_KUSALA * (sati + metta),
            javana_moments: 1,
        }
    }

    /// รวมผลของชวนะทุกขณะในวิถี 17 ขณะ
    ///
    /// วิถีที่ถูกบล็อกที่โวฏฐัพพนะไม่มีชวนะ → ได้ [`Self::none`]
    pub fn from_vithi17(vithi: &Vithi17) -> Self {
        let javana: Vec<CetasikaVector> = vithi
            .moments
            .iter()
            .filter_map(|m| match m {
                CittaJati17::Javana(c) => Some(*c),
                _ => None,
            })
            .collect();
        Self::average(sum_javana(&javana))
    }

    /// รวมผลของชวนะทุกขณะในวิถี 15 ขณะ
    pub fn from_vithi15(vithi: &Vithi15) -> Self {
        let javana: Vec<CetasikaVector> = vithi
            .moments
            .iter()
            .filter_map(|m| match m {
                CittaJati::Javana(c) => Some(*c),
                _ => None,
            })
            .collect();
        Self::average(sum_javana(&javana))
    }

    /// นำผลกระทบไปใช้กับธาตุทั้ง 4
    pub fn apply(&self, cluster: &mut RupaCluster) {
        if self.is_empty() {
            return;
        }

        cluster.pathavi.wear_level =
            clamp01(cluster.pathavi.wear_level + self.pathavi_wear - self.pathavi_relief);
        cluster.apo.cohesion_strength =
            clamp01(cluster.apo.cohesion_strength - self.apo_cohesion_loss + self.apo_recovery);
        cluster.tejo.temp_c =
            (cluster.tejo.temp_c + self.tejo_heat - self.tejo_cooling).max(AMBIENT_C);
        cluster.vayo_turbulence =
            (cluster.vayo_turbulence + self.vayo_turbulence - self.vayo_calm).max(0.0);
    }

    /// หารด้วยจำนวนชวนะตามตำรา เพื่อให้หนึ่งวิถีมีน้ำหนักเท่ากับหนึ่งเหตุการณ์
    fn average(impact: Self) -> Self {
        if impact.javana_moments == 0 {
            return Self::none();
        }
        let divisor = JAVANA_COUNT as f32;
        Self {
            pathavi_wear: impact.pathavi_wear / divisor,
            apo_cohesion_loss: impact.apo_cohesion_loss / divisor,
            tejo_heat: impact.tejo_heat / divisor,
            vayo_turbulence: impact.vayo_turbulence / divisor,
            tejo_cooling: impact.tejo_cooling / divisor,
            apo_recovery: impact.apo_recovery / divisor,
            pathavi_relief: impact.pathavi_relief / divisor,
            vayo_calm: impact.vayo_calm / divisor,
            javana_moments: impact.javana_moments,
        }
    }
}

fn sum_javana(javana: &[CetasikaVector]) -> RupaImpact {
    let mut total = RupaImpact::none();
    for cetasika in javana {
        let one = RupaImpact::from_cetasika(cetasika);
        total.pathavi_wear += one.pathavi_wear;
        total.apo_cohesion_loss += one.apo_cohesion_loss;
        total.tejo_heat += one.tejo_heat;
        total.vayo_turbulence += one.vayo_turbulence;
        total.tejo_cooling += one.tejo_cooling;
        total.apo_recovery += one.apo_recovery;
        total.pathavi_relief += one.pathavi_relief;
        total.vayo_calm += one.vayo_calm;
        total.javana_moments += 1;
    }
    total
}

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// กลุ่มธาตุ 4 ที่รับผลของกรรม — ใช้เมื่อต้องการจำลองทั้งชุดพร้อมกัน
#[derive(Debug, Clone)]
pub struct RupaCluster {
    pub pathavi: PathaviDriver,
    pub apo: ApoDriver,
    pub tejo: TejoDriver,
    /// ความฟุ้งของวาโย (0.0 = สงบ)
    pub vayo_turbulence: f32,
}

impl RupaCluster {
    /// กลุ่มธาตุเริ่มต้นที่ยังไม่รับกรรมใด ๆ
    pub fn new() -> Self {
        Self {
            pathavi: PathaviDriver::new(0),
            apo: ApoDriver::new(),
            tejo: TejoDriver::new(),
            vayo_turbulence: 0.0,
        }
    }

    /// รับผลของกรรมหนึ่งวิถี
    pub fn absorb(&mut self, impact: &RupaImpact) {
        impact.apply(self);
    }

    /// รับผลของวิถี 17 ขณะ แล้วคืนผลกระทบที่เกิดขึ้น
    pub fn absorb_vithi17(&mut self, vithi: &Vithi17) -> RupaImpact {
        let impact = RupaImpact::from_vithi17(vithi);
        self.absorb(&impact);
        impact
    }

    /// รับผลของวิถี 15 ขณะ แล้วคืนผลกระทบที่เกิดขึ้น
    pub fn absorb_vithi15(&mut self, vithi: &Vithi15) -> RupaImpact {
        let impact = RupaImpact::from_vithi15(vithi);
        self.absorb(&impact);
        impact
    }

    /// ตัววัดความเสื่อมของรูปทั้งชุด (0.0 = สมบูรณ์, 1.0 = เสื่อมสุด)
    pub fn degradation(&self) -> f32 {
        let tejo = ((self.tejo.temp_c - AMBIENT_C) / 100.0).clamp(0.0, 1.0);
        (self.pathavi.wear_level + (1.0 - self.apo.cohesion_strength) + tejo) / 3.0
    }

    /// ตรวจว่าธาตุใดในชุดนี้ "ปฏิเสธ" เจตนาที่ให้มา
    ///
    /// คืนเฉพาะข้อละเมิดจริง ไม่ใช่ทั้งหมด — ใช้กับ `paramattha_execute` ได้
    pub fn sila_check(&self, intent: &CetasikaVector) -> Result<(), DhammaViolation> {
        let intent = CetasikaVectorMahabhuta {
            lobha: intent.get(LOBHA),
            dosa: intent.get(DOSA),
            moha: intent.get(MOHA),
        };
        let drivers: [&dyn MahabhutaRupa; 3] = [&self.pathavi, &self.apo, &self.tejo];
        for driver in drivers {
            driver.sila_check(&intent)?;
        }
        Ok(())
    }
}

impl Default for RupaCluster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paramattha::vithi17::{Dvara, GateContext, PdpOutcome};

    fn dosa() -> CetasikaVector {
        CetasikaVector::new().with(DOSA, 0.9)
    }

    fn lobha() -> CetasikaVector {
        CetasikaVector::new().with(LOBHA, 0.9)
    }

    fn moha() -> CetasikaVector {
        CetasikaVector::new().with(MOHA, 0.9)
    }

    fn kusala() -> CetasikaVector {
        CetasikaVector::new().with(SATI, 0.8).with(ADOSA, 0.8)
    }

    #[test]
    fn blocked_vithi_has_no_impact() {
        let mut gate = |_: &GateContext<'_>| PdpOutcome::Blocked {
            reason: "blocked".into(),
        };
        let vithi = Vithi17::run(Dvara::Cakkhu, "frame", dosa(), &mut gate);
        let impact = RupaImpact::from_vithi17(&vithi);

        assert!(impact.is_empty());
        assert_eq!(impact.javana_moments, 0);
        assert_eq!(impact, RupaImpact::none());
    }

    #[test]
    fn dosa_heats_tejo_without_touching_apo() {
        let vithi = Vithi17::from_external_event(Dvara::Kaya, "hit", dosa());
        let impact = RupaImpact::from_vithi17(&vithi);

        assert_eq!(impact.javana_moments, JAVANA_COUNT);
        assert!(impact.tejo_heat > 0.0, "โทสะต้องเพิ่มความร้อน");
        assert_eq!(impact.apo_cohesion_loss, 0.0, "โทสะไม่ทำให้อาโปเสีย");
        assert_eq!(impact.vayo_turbulence, 0.0, "โทสะไม่ทำให้วาโยฟุ้ง");
        assert!(impact.is_harmful());
    }

    #[test]
    fn lobha_weakens_apo_without_heating_tejo() {
        let vithi = Vithi17::from_external_event(Dvara::Jivha, "taste", lobha());
        let impact = RupaImpact::from_vithi17(&vithi);

        assert!(impact.apo_cohesion_loss > 0.0);
        assert_eq!(impact.tejo_heat, 0.0);
        assert_eq!(impact.vayo_turbulence, 0.0);
    }

    #[test]
    fn moha_turbulates_vayo_without_heating_tejo() {
        let vithi = Vithi17::from_external_event(Dvara::Sota, "audio", moha());
        let impact = RupaImpact::from_vithi17(&vithi);

        assert!(impact.vayo_turbulence > 0.0);
        assert_eq!(impact.tejo_heat, 0.0);
        assert_eq!(impact.apo_cohesion_loss, 0.0);
    }

    #[test]
    fn kusala_cools_and_restores() {
        let vithi = Vithi17::from_external_event(Dvara::Ghana, "smell", kusala());
        let impact = RupaImpact::from_vithi17(&vithi);

        assert!(impact.tejo_cooling > 0.0);
        assert!(impact.apo_recovery > 0.0);
        assert!(impact.vayo_calm > 0.0);
        assert!(!impact.is_harmful());
    }

    #[test]
    fn one_vithi_does_not_multiply_harm_by_seven() {
        // ผลของวิถีเดียวต้องเท่ากับผลของชวนะหนึ่งขณะ (เพราะเฉลี่ยด้วย JAVANA_COUNT)
        let one = RupaImpact::from_cetasika(&dosa());
        let vithi_impact =
            RupaImpact::from_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", dosa()));
        assert!((vithi_impact.tejo_heat - one.tejo_heat).abs() < 1e-4);
        assert_eq!(vithi_impact.javana_moments, JAVANA_COUNT);
    }

    #[test]
    fn absorbing_dosa_raises_temperature_and_wear() {
        let mut cluster = RupaCluster::new();
        assert_eq!(cluster.tejo.temp_c, AMBIENT_C);

        let impact =
            cluster.absorb_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", dosa()));

        assert!(cluster.tejo.temp_c > AMBIENT_C);
        assert!(cluster.pathavi.wear_level > 0.0);
        assert_eq!(cluster.apo.cohesion_strength, 1.0);
        assert_eq!(cluster.vayo_turbulence, 0.0);
        assert_eq!(impact.javana_moments, JAVANA_COUNT);
    }

    #[test]
    fn repeated_akusala_accumulates_degradation() {
        let mut cluster = RupaCluster::new();
        let before = cluster.degradation();

        for _ in 0..10 {
            cluster.absorb_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", dosa()));
        }

        assert!(cluster.degradation() > before);
        assert!(cluster.tejo.temp_c > 60.0, "10 วิถีโทสะต้องร้อนขึ้นชัดเจน");
    }

    #[test]
    fn kusala_after_akusala_heals_the_rupa() {
        let mut cluster = RupaCluster::new();
        cluster.absorb_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", dosa()));
        let hot = cluster.tejo.temp_c;
        let worn = cluster.pathavi.wear_level;

        cluster.absorb_vithi17(&Vithi17::from_external_event(
            Dvara::Cakkhu,
            "see",
            kusala(),
        ));

        assert!(cluster.tejo.temp_c < hot, "กุศลต้องทำให้เย็นลง");
        assert!(cluster.pathavi.wear_level < worn, "กุศลต้องบรรเทาความสึก");
        assert!(
            cluster.apo.cohesion_strength > 1.0 - f32::EPSILON
                || cluster.apo.cohesion_strength == 1.0
        );
    }

    #[test]
    fn temperature_never_drops_below_ambient() {
        let mut cluster = RupaCluster::new();
        for _ in 0..20 {
            cluster.absorb_vithi17(&Vithi17::from_external_event(
                Dvara::Cakkhu,
                "see",
                kusala(),
            ));
        }
        assert_eq!(cluster.tejo.temp_c, AMBIENT_C);
    }

    #[test]
    fn wear_and_cohesion_stay_in_range() {
        let mut cluster = RupaCluster::new();
        for _ in 0..500 {
            cluster.absorb_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", lobha()));
        }
        assert!((0.0..=1.0).contains(&cluster.pathavi.wear_level));
        assert!((0.0..=1.0).contains(&cluster.apo.cohesion_strength));
    }

    #[test]
    fn vithi15_log_drive_the_same_rupa_effects() {
        let mut cluster = RupaCluster::new();
        let vithi = Vithi15::from_internal_log("kernel: Out of memory: Kill process 1");
        let impact = cluster.absorb_vithi15(&vithi);

        assert_eq!(impact.javana_moments, JAVANA_COUNT);
        assert!(cluster.tejo.temp_c > AMBIENT_C);
    }

    #[test]
    fn neutral_vithi15_has_no_harm() {
        let mut cluster = RupaCluster::new();
        let vithi = Vithi15::from_internal_log("systemd: Starting Session - INFO");
        let impact = cluster.absorb_vithi15(&vithi);

        assert_eq!(impact.javana_moments, JAVANA_COUNT);
        assert!(!impact.is_harmful());
    }

    #[test]
    fn sila_check_is_delegated_to_each_driver() {
        let cluster = RupaCluster::new();
        assert_eq!(cluster.sila_check(&CetasikaVector::new()), Ok(()));
        assert_eq!(
            cluster.sila_check(&dosa()),
            Err(DhammaViolation::MicchaKammanta("Tejo refuses Dosa"))
        );
    }
}
