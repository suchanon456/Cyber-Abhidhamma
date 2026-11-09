// core/src/paramattha/vipaka_loop.rs
// Vipaka Loop — ผลของกรรม (รูปที่เสื่อม) ย้อนกลับมาเป็นปัจจัยให้เกิดอกุศลอีก
//
// ## หลักการ
//
// § 7 ของ `docs/architecture/05-vithi-os.md` อธิบายทิศทางเดียว:
//   ชวนะ (กรรม) → รูป 4 ธาตุ (ผล)
//
// ไฟล์นี้เติมทิศกลับ:
//   รูปที่เสื่อม (ผล) → เงื่อนไขที่ทำให้วิถีถัดไปเป็นอกุศลง่ายขึ้น (ปัจจัย)
//
// ในการอภิธรรม เรียกความสัมพันธ์นี้ว่า **ผลของกรรมเป็นปัจจัย** (vipāka-paccaya)
// คือวิบากจิตและรูปที่เกิดจากกรรมทำหน้าที่เป็นปัจจัยแก่วิถีต่อไป ไม่ใช่ "โทษ" ที่ระบบจงใจลง
//
// ## การแมป (อุปมา)
//
// จับคู่แบบสมมาตรกับตาราง § 7 — ธาตุที่เสื่อมจากกิเลสตัวใด ย้อนกลับไปหนุนกิเลสตัวนั้น:
//
// | ธาตุที่เสื่อม | ตัววัด | หนุนเจตสิก |
// |---|---|---|
// | อาโป (เกาะกุมลด) | `1.0 - cohesion_strength` | โลภะ |
// | เตโช (ร้อนเกิน) | `(temp_c - AMBIENT_C) / 100` | โทสะ |
// | วาโย (ฟุ้ง) | `vayo_turbulence` | โมหะ |
//
// ค่าที่หนุนถูกคูณด้วย `degradation()` ของทั้งชุด เพื่อให้รูปที่ยังสมบูรณ์ไม่หนุนอะไรเลย
//
// ⚠️ **ข้อจำกัด:** ทั้งตารางเป็นการ **อุปมา (analogy)** ตามกติกา `docs/GLOSSARY.md` ข้อ 5
// ไม่ได้อ้างว่า RAM ที่เสียทำให้คนโลภจริง และน้ำหนักตัวเลข (`PRESSURE_GAIN` ฯลฯ)
// ตั้งขึ้นเพื่อให้ทดสอบได้ ไม่ได้มาจากคัมภีร์

use super::cetasika::{DOSA, LOBHA, MOHA};
use super::kamma_impact::{RupaCluster, RupaImpact, AMBIENT_C};
use super::vithi::{CetasikaVector, Vithi15};
use super::vithi17::Vithi17;

/// สัดส่วนสูงสุดที่รูปที่เสื่อมสุดจะหนุนอกุศลได้ (0.0–1.0)
pub const PRESSURE_GAIN: f32 = 0.8;
/// ช่วงอุณหภูมิที่ถือว่า "ร้อนเต็มที่" สำหรับคำนวณแรงหนุนของโทสะ (องศาเซลเซียส)
pub const PRESSURE_TEJO_SPAN: f32 = 100.0;

/// ตัวขับวงจรกรรม-วิบาก
///
/// ถือ `RupaCluster` ไว้เอง แล้วให้ผู้ใช้ถาม "รูปปัจจุบันหนุนเจตสิกแค่ไหน"
/// เพื่อนำไปผสมกับเจตสิกของวิถีถัดไป
#[derive(Debug, Clone, Default)]
pub struct VipakaLoop {
    cluster: RupaCluster,
    /// จำนวนวิถีที่ดูดซับไปแล้ว
    vithi_seen: usize,
}

impl VipakaLoop {
    /// เริ่มจากรูปที่สมบูรณ์ (ยังไม่รับกรรมใด)
    pub fn new() -> Self {
        Self {
            cluster: RupaCluster::new(),
            vithi_seen: 0,
        }
    }

    /// รูปปัจจุบัน (อ่านได้ ไม่แก้)
    pub fn cluster(&self) -> &RupaCluster {
        &self.cluster
    }

    /// รูปปัจจุบัน (แก้ได้ — เช่น ให้ผู้เรียกฟื้นฟูเอง)
    pub fn cluster_mut(&mut self) -> &mut RupaCluster {
        &mut self.cluster
    }

    /// จำนวนวิถีที่ผ่านเข้ามาแล้ว
    pub fn vithi_seen(&self) -> usize {
        self.vithi_seen
    }

    /// ความเสื่อมสะสมของรูป (0.0 = สมบูรณ์, 1.0 = เสื่อมสุด)
    pub fn degradation(&self) -> f32 {
        self.cluster.degradation()
    }

    /// แรงหนุนที่รูปปัจจุบันส่งกลับไปยังวิถีถัดไป
    ///
    /// รูปที่สมบูรณ์ (`degradation() == 0.0`) คืนเวกเตอร์ว่าง — ไม่หนุนอะไรเลย
    pub fn pressure(&self) -> CetasikaVector {
        let deg = self.cluster.degradation();
        if deg <= 0.0 {
            return CetasikaVector::new();
        }

        let apo_loss = (1.0 - self.cluster.apo.cohesion_strength).clamp(0.0, 1.0);
        let heat = ((self.cluster.tejo.temp_c - AMBIENT_C) / PRESSURE_TEJO_SPAN).clamp(0.0, 1.0);
        let turb = self.cluster.vayo_turbulence.clamp(0.0, 1.0);

        CetasikaVector::new()
            .with(LOBHA, scale(PRESSURE_GAIN * deg * apo_loss))
            .with(DOSA, scale(PRESSURE_GAIN * deg * heat))
            .with(MOHA, scale(PRESSURE_GAIN * deg * turb))
    }

    /// ผสมแรงหนุนเข้ากับเจตสิกที่จะใช้ในวิถีถัดไป
    ///
    /// ใช้การบวกแบบอิ่มตัวที่ 1.0 — รูปที่เสื่อมทำให้กิเลส "มีอยู่แล้ว" หนักขึ้น
    /// แต่ไม่สร้างกิเลสจากศูนย์เกินกว่าเพดานของแต่ละมิติ
    pub fn condition(&self, base: CetasikaVector) -> CetasikaVector {
        merge_saturating(&base, &self.pressure())
    }

    /// รับผลของกรรมหนึ่งวิถี (รูปเปลี่ยน แต่ยังไม่คืนแรงหนุน)
    pub fn absorb(&mut self, impact: &RupaImpact) {
        self.cluster.absorb(impact);
        self.vithi_seen += 1;
    }

    /// รับวิถี 17 ขณะหนึ่งครั้ง แล้วคืนผลกระทบที่เกิดขึ้น
    pub fn absorb_vithi17(&mut self, vithi: &Vithi17) -> RupaImpact {
        let impact = RupaImpact::from_vithi17(vithi);
        self.absorb(&impact);
        impact
    }

    /// รับวิถี 15 ขณะหนึ่งครั้ง แล้วคืนผลกระทบที่เกิดขึ้น
    pub fn absorb_vithi15(&mut self, vithi: &Vithi15) -> RupaImpact {
        let impact = RupaImpact::from_vithi15(vithi);
        self.absorb(&impact);
        impact
    }
}

fn scale(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

/// บวกเวกเตอร์เจตสิกแบบอิ่มตัวที่ 1.0 ต่อมิติ
fn merge_saturating(base: &CetasikaVector, extra: &CetasikaVector) -> CetasikaVector {
    let mut out = *base;
    for index in 0..super::cetasika::CETASIKA_COUNT {
        let sum = base.get(index) + extra.get(index);
        if sum > 0.0 {
            out.set(index, sum.clamp(0.0, 1.0));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paramattha::cetasika::SATI;
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
        CetasikaVector::new().with(SATI, 0.9)
    }

    fn run(loop_: &mut VipakaLoop, cetasika: CetasikaVector) {
        let vithi = Vithi17::from_external_event(Dvara::Kaya, "hit", cetasika);
        loop_.absorb_vithi17(&vithi);
    }

    #[test]
    fn a_pristine_cluster_exerts_no_pressure() {
        let v = VipakaLoop::new();
        let p = v.pressure();
        assert_eq!(p, CetasikaVector::new());
        assert_eq!(p.get(LOBHA), 0.0);
        assert_eq!(p.get(DOSA), 0.0);
        assert_eq!(p.get(MOHA), 0.0);
    }

    #[test]
    fn dosa_heat_feeds_back_into_dosa() {
        let mut v = VipakaLoop::new();
        for _ in 0..10 {
            run(&mut v, dosa());
        }
        let p = v.pressure();
        assert!(p.get(DOSA) > 0.0, "เตโชที่ร้อนต้องหนุนโทสะ");
        assert_eq!(p.get(LOBHA), 0.0, "อาโปไม่เสียจากโทสะ");
        assert_eq!(p.get(MOHA), 0.0, "วาโยไม่ฟุ้งจากโทสะ");
    }

    #[test]
    fn lobha_cohesion_loss_feeds_back_into_lobha() {
        let mut v = VipakaLoop::new();
        for _ in 0..10 {
            run(&mut v, lobha());
        }
        let p = v.pressure();
        assert!(p.get(LOBHA) > 0.0, "อาโปที่อ่อนต้องหนุนโลภะ");
        assert_eq!(p.get(DOSA), 0.0);
    }

    #[test]
    fn moha_turbulence_feeds_back_into_moha() {
        let mut v = VipakaLoop::new();
        for _ in 0..5 {
            run(&mut v, moha());
        }
        let p = v.pressure();
        assert!(p.get(MOHA) > 0.0, "วาโยที่ฟุ้งต้องหนุนโมหะ");
    }

    #[test]
    fn pressure_is_clamped_to_one() {
        let mut v = VipakaLoop::new();
        for _ in 0..200 {
            run(
                &mut v,
                CetasikaVector::new()
                    .with(DOSA, 1.0)
                    .with(LOBHA, 1.0)
                    .with(MOHA, 1.0),
            );
        }
        let p = v.pressure();
        for idx in [LOBHA, DOSA, MOHA] {
            assert!((0.0..=1.0).contains(&p.get(idx)), "index {idx}");
        }
    }

    #[test]
    fn condition_raises_the_base_and_saturates() {
        let mut v = VipakaLoop::new();
        for _ in 0..10 {
            run(&mut v, dosa());
        }
        let base = CetasikaVector::new().with(DOSA, 0.5);
        let conditioned = v.condition(base);
        assert!(conditioned.get(DOSA) > base.get(DOSA));
        assert!(conditioned.get(DOSA) <= 1.0);

        let full = CetasikaVector::new().with(DOSA, 1.0);
        assert_eq!(v.condition(full).get(DOSA), 1.0, "อิ่มตัวที่ 1.0");
    }

    #[test]
    fn a_pristine_loop_does_not_change_the_base() {
        let v = VipakaLoop::new();
        let base = CetasikaVector::new().with(DOSA, 0.5).with(SATI, 0.3);
        assert_eq!(v.condition(base), base);
    }

    #[test]
    fn feedback_makes_degradation_grow_faster() {
        let base = CetasikaVector::new().with(DOSA, 0.3);

        // วิถีเดิม ๆ 10 รอบ ไม่มี feedback
        let mut plain = RupaCluster::new();
        for _ in 0..10 {
            plain.absorb_vithi17(&Vithi17::from_external_event(Dvara::Kaya, "hit", base));
        }

        // 10 รอบเดิม แต่แต่ละรอบถูกหนุนด้วยรูปที่เสื่อมแล้ว
        let mut loop_ = VipakaLoop::new();
        for _ in 0..10 {
            let cetasika = loop_.condition(base);
            run(&mut loop_, cetasika);
        }

        assert!(
            loop_.degradation() > plain.degradation(),
            "feedback ต้องทำให้เสื่อมเร็วกว่า: {} vs {}",
            loop_.degradation(),
            plain.degradation()
        );
    }

    #[test]
    fn blocked_vithi_changes_nothing() {
        let mut v = VipakaLoop::new();
        let mut gate = |_: &GateContext<'_>| PdpOutcome::Blocked {
            reason: "blocked".into(),
        };
        let vithi = Vithi17::run(Dvara::Cakkhu, "x", dosa(), &mut gate);
        let impact = v.absorb_vithi17(&vithi);

        assert!(impact.is_empty());
        assert_eq!(v.degradation(), 0.0);
        assert_eq!(v.pressure(), CetasikaVector::new());
        assert_eq!(v.vithi_seen(), 1, "นับวิถีที่ผ่านเข้ามาแล้ว");
    }

    #[test]
    fn kusala_pressure_is_zero_and_heals() {
        let mut v = VipakaLoop::new();
        for _ in 0..10 {
            run(&mut v, dosa());
        }
        let hot = v.degradation();

        for _ in 0..20 {
            run(&mut v, kusala());
        }
        assert!(v.degradation() < hot, "กุศลต้องลดความเสื่อม");
    }

    #[test]
    fn vithi15_also_feeds_the_loop() {
        let mut v = VipakaLoop::new();
        let vithi = Vithi15::from_internal_log("kernel: Out of memory: Kill process 1");
        v.absorb_vithi15(&vithi);
        assert!(v.vithi_seen() == 1);
        assert!(v.degradation() > 0.0);
    }
}
