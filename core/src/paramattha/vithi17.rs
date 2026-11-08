// core/src/paramattha/vithi17.rs
// Vithi 17 - Pancadvara - external (hardware) interrupt path.
// Safe simulation only.
//
// ในเลนส์ OS วิถีนี้คือ interrupt path เต็มรูปแบบ (ดู docs/architecture/05-vithi-os.md § 2):
//   ภวังคจลนะ       = interrupt line ถูกยก (wake-up)
//   ภวังคุปัจเฉทะ    = context switch ออกจาก idle
//   ปัญจทวาราวัชชนะ  = interrupt vector dispatch (เลือกทวาร)
//   ปัญจวิญญาณ      = ISR / driver DMA
//   สัมปฏิจฉนะ      = เข้า kernel ring buffer
//   สันตีรณะ        = classifier
//   โวฏฐัพพนะ       = Policy Decision Point (PDP)
//   ชวนะ ×7         = execution window
//   ตทาลัมพนะ ×2    = flush ผลแล้วคืนบัฟเฟอร์
//
// **ข้อต่างที่สำคัญจากวิถี 15:** ถ้า PDP บล็อก วิถีจะหยุดที่โวฏฐัพพนะทันที
// ไม่มีชวนะและตทาลัมพนะ — คำขอที่ถูกปฏิเสธไม่เคยเข้าสู่ execution window
// (เทียบเท่า kernel ที่ไม่ execute syscall ที่ถูกปฏิเสธ)
//
// หมายเหตุเรื่องจำนวนขณะ: 3 + 1 + 1 + 1 + 1 + 1 + 7 + 2 = 17 ตามการนับในตำรา
// วิถีปัญจทวารมี 17 ขณะ (ต่างจากมโนทวารที่โครงการนี้ใช้ 15 ขณะ)

use std::fmt;

use super::vithi::{CetasikaVector, JAVANA_COUNT};

/// ทวาร 5 — ช่องทางที่ event ภายนอกเข้ามา
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dvara {
    /// ตา — จักขุทวาร
    Cakkhu,
    /// หู — โสตทวาร
    Sota,
    /// จมูก — ฆานทวาร
    Ghana,
    /// ลิ้น — ชิวหาทวาร
    Jivha,
    /// กาย — กายทวาร
    Kaya,
}

impl Dvara {
    /// ทั้ง 5 ทวาร ตามลำดับในตำรา
    pub const ALL: [Dvara; 5] = [
        Dvara::Cakkhu,
        Dvara::Sota,
        Dvara::Ghana,
        Dvara::Jivha,
        Dvara::Kaya,
    ];

    pub fn name_th(&self) -> &'static str {
        match self {
            Dvara::Cakkhu => "จักขุทวาร",
            Dvara::Sota => "โสตทวาร",
            Dvara::Ghana => "ฆานทวาร",
            Dvara::Jivha => "ชิวหาทวาร",
            Dvara::Kaya => "กายทวาร",
        }
    }

    /// ตัวอย่างการแมปไปยังอุปกรณ์ในแบบจำลอง OS
    ///
    /// ⚠️ เป็น *ตัวอย่าง* ที่ผู้พัฒนาตั้งขึ้น ไม่ใช่ข้อกำหนดจากตำรา
    pub fn os_channel(&self) -> &'static str {
        match self {
            Dvara::Cakkhu => "device:camera",
            Dvara::Sota => "device:audio_in",
            Dvara::Ghana => "device:sensor_gas",
            Dvara::Jivha => "device:sensor_chem",
            Dvara::Kaya => "device:touch",
        }
    }
}

impl fmt::Display for Dvara {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name_th())
    }
}

/// ผลการตัดสินที่โวฏฐัพพนะ (PDP)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdpOutcome {
    /// ผ่าน — วิถีดำเนินต่อจนครบ 17 ขณะ
    Allowed,
    /// ผ่านแต่ช้ากว่า — ยังเข้า execution window
    Throttled { reason: String },
    /// ปฏิเสธ — วิถีหยุดที่โวฏฐัพพนะ ไม่มีชวนะ
    Blocked { reason: String },
}

impl PdpOutcome {
    pub fn is_blocked(&self) -> bool {
        matches!(self, PdpOutcome::Blocked { .. })
    }

    /// `true` เมื่อ outcome นี้ทำให้วิถีไม่ดำเนินต่อถึงขั้นชวนะ
    pub fn stops_execution(&self) -> bool {
        self.is_blocked()
    }

    pub fn reason(&self) -> Option<&str> {
        match self {
            PdpOutcome::Allowed => None,
            PdpOutcome::Throttled { reason } | PdpOutcome::Blocked { reason } => Some(reason),
        }
    }
}

/// ข้อมูลที่ส่งให้ PDP เพื่อตัดสิน — มีเท่าที่ PDP ต้องใช้จริง
#[derive(Debug)]
pub struct GateContext<'a> {
    pub dvara: Dvara,
    pub source: &'a str,
    pub cetasika: &'a CetasikaVector,
}

/// Policy Decision Point — ต่อกับ firewall หรือ policy ใดก็ได้
///
/// closure ทั่วไปใช้เป็น gate ได้ทันทีเพราะมี blanket impl ข้างล่าง:
/// ```
/// use cyber_abhidhamma::paramattha::vithi::{CetasikaVector, JAVANA_COUNT};
/// use cyber_abhidhamma::paramattha::vithi17::{Dvara, GateContext, PdpOutcome, Vithi17};
///
/// let cetasika = CetasikaVector { moha: 0.9, ..Default::default() };
/// let mut gate = |ctx: &GateContext<'_>| {
///     if ctx.cetasika.moha > 0.7 {
///         PdpOutcome::Blocked { reason: "moha".into() }
///     } else {
///         PdpOutcome::Allowed
///     }
/// };
///
/// let vithi = Vithi17::run(Dvara::Cakkhu, "frame#1", cetasika, &mut gate);
/// assert!(vithi.is_aborted());
/// assert_eq!(vithi.moment_count(), 8);          // หยุดที่โวฏฐัพพนะ
/// assert_eq!(vithi.javana_count(), 0);           // ไม่มี execution window
///
/// let vithi = Vithi17::from_external_event(Dvara::Cakkhu, "frame#2", CetasikaVector::new());
/// assert_eq!(vithi.moment_count(), 17);
/// assert_eq!(vithi.javana_count(), JAVANA_COUNT);
/// ```
pub trait Gate {
    fn decide(&mut self, ctx: &GateContext<'_>) -> PdpOutcome;
}

impl<F> Gate for F
where
    F: FnMut(&GateContext<'_>) -> PdpOutcome,
{
    fn decide(&mut self, ctx: &GateContext<'_>) -> PdpOutcome {
        self(ctx)
    }
}

/// PDP ที่อนุญาตทุกอย่าง — ใช้เมื่อต้องการจำลองเฉพาะลำดับขณะ
#[derive(Debug, Default, Clone, Copy)]
pub struct AllowAll;

impl Gate for AllowAll {
    fn decide(&mut self, _ctx: &GateContext<'_>) -> PdpOutcome {
        PdpOutcome::Allowed
    }
}

/// 10 ชนิดของขณะจิตในวิถี 17 (ปัญจวิญญาณมี 5 ตัวแยกตามทวาร)
#[derive(Debug, Clone, PartialEq)]
pub enum CittaJati17 {
    AtitaBhavanga(CetasikaVector),
    BhavangaCalana(CetasikaVector),
    Bhavangupaccheda(CetasikaVector),
    Pancadvaravajjana(CetasikaVector),
    Pancavinnana(Dvara, CetasikaVector),
    Sampaticchana(CetasikaVector),
    Santirana(CetasikaVector),
    Votthapana(CetasikaVector),
    Javana(CetasikaVector),
    Tadalambana(CetasikaVector),
}

impl CittaJati17 {
    pub fn name_th(&self) -> &'static str {
        match self {
            CittaJati17::AtitaBhavanga(_) => "อดีตภวังค์",
            CittaJati17::BhavangaCalana(_) => "ภวังคจลนะ",
            CittaJati17::Bhavangupaccheda(_) => "ภวังคุปัจเฉทะ",
            CittaJati17::Pancadvaravajjana(_) => "ปัญจทวาราวัชชนะ",
            CittaJati17::Pancavinnana(..) => "ปัญจวิญญาณ",
            CittaJati17::Sampaticchana(_) => "สัมปฏิจฉนะ",
            CittaJati17::Santirana(_) => "สันตีรณะ",
            CittaJati17::Votthapana(_) => "โวฏฐัพพนะ",
            CittaJati17::Javana(_) => "ชวนะ",
            CittaJati17::Tadalambana(_) => "ตทาลัมพนะ",
        }
    }

    /// ชื่อขั้นในแบบจำลอง OS — ตรงกับตารางใน docs/architecture/05-vithi-os.md § 2
    pub fn os_stage(&self) -> &'static str {
        match self {
            CittaJati17::AtitaBhavanga(_) => "idle_loop_previous_cycle",
            CittaJati17::BhavangaCalana(_) => "interrupt_line_asserted",
            CittaJati17::Bhavangupaccheda(_) => "context_switch",
            CittaJati17::Pancadvaravajjana(_) => "interrupt_vector_dispatch",
            CittaJati17::Pancavinnana(..) => "isr_driver_dma",
            CittaJati17::Sampaticchana(_) => "kernel_ring_buffer",
            CittaJati17::Santirana(_) => "classifier",
            CittaJati17::Votthapana(_) => "policy_decision_point",
            CittaJati17::Javana(_) => "execution_window",
            CittaJati17::Tadalambana(_) => "flush_and_return",
        }
    }

    /// เจตสิกที่ประกอบขณะจิตดวงนี้
    pub fn cetasika(&self) -> CetasikaVector {
        match self {
            CittaJati17::AtitaBhavanga(c)
            | CittaJati17::BhavangaCalana(c)
            | CittaJati17::Bhavangupaccheda(c)
            | CittaJati17::Pancadvaravajjana(c)
            | CittaJati17::Pancavinnana(_, c)
            | CittaJati17::Sampaticchana(c)
            | CittaJati17::Santirana(c)
            | CittaJati17::Votthapana(c)
            | CittaJati17::Javana(c)
            | CittaJati17::Tadalambana(c) => *c,
        }
    }

    /// ทวารของขณะนี้ — มีเฉพาะปัญจวิญญาณ (และอนุมานจากวิถีได้)
    pub fn dvara(&self) -> Option<Dvara> {
        match self {
            CittaJati17::Pancavinnana(d, _) => Some(*d),
            _ => None,
        }
    }
}

impl fmt::Display for CittaJati17 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name_th())
    }
}

/// วิถีจิต 17 ขณะทางปัญจทวาร — ภัยจากภายนอก (interrupt path)
#[derive(Debug, Clone)]
pub struct Vithi17 {
    pub moments: Vec<CittaJati17>,
    pub dvara: Dvara,
    pub source: String,
    pub outcome: PdpOutcome,
}

impl Vithi17 {
    /// ลำดับของโวฏฐัพพนะ (PDP) ในวิถี — index 7 (เริ่มนับที่ 0)
    pub const PDP_INDEX: usize = 7;

    /// จำนวนขณะเมื่อวิถีดำเนินจนครบ
    pub const FULL_MOMENTS: usize = 17;

    /// สร้างวิถี 17 ขณะโดยให้ PDP ตัดสิน
    ///
    /// ถ้า `gate` คืน `PdpOutcome::Blocked` วิถีจะจบที่โวฏฐัพพนะ (8 ขณะ)
    /// ไม่มีชวนะและตทาลัมพนะ
    pub fn run<G: Gate>(
        dvara: Dvara,
        source: &str,
        cetasika: CetasikaVector,
        gate: &mut G,
    ) -> Self {
        let outcome = gate.decide(&GateContext {
            dvara,
            source,
            cetasika: &cetasika,
        });

        let empty = CetasikaVector::new();
        let mut moments = vec![
            CittaJati17::AtitaBhavanga(empty),
            CittaJati17::BhavangaCalana(empty),
            CittaJati17::Bhavangupaccheda(cetasika),
            CittaJati17::Pancadvaravajjana(cetasika),
            CittaJati17::Pancavinnana(dvara, cetasika),
            CittaJati17::Sampaticchana(cetasika),
            CittaJati17::Santirana(cetasika),
            CittaJati17::Votthapana(cetasika),
        ];

        if !outcome.stops_execution() {
            moments.extend((0..JAVANA_COUNT).map(|_| CittaJati17::Javana(cetasika)));
            moments.push(CittaJati17::Tadalambana(cetasika));
            moments.push(CittaJati17::Tadalambana(cetasika));
        }

        Self {
            moments,
            dvara,
            source: source.to_string(),
            outcome,
        }
    }

    /// สร้างวิถี 17 ขณะโดยไม่มีการตัดสินใจ (PDP อนุญาตทุกอย่าง)
    pub fn from_external_event(dvara: Dvara, source: &str, cetasika: CetasikaVector) -> Self {
        let mut gate = AllowAll;
        Self::run(dvara, source, cetasika, &mut gate)
    }

    /// จำนวนขณะที่เกิดขึ้นจริง (17 ถ้าผ่าน, 8 ถ้าถูกบล็อก)
    pub fn moment_count(&self) -> usize {
        self.moments.len()
    }

    /// จำนวนชวนะที่เกิดขึ้นจริง
    pub fn javana_count(&self) -> usize {
        self.moments
            .iter()
            .filter(|m| matches!(m, CittaJati17::Javana(_)))
            .count()
    }

    /// `true` เมื่อวิถีดำเนินจนครบ 17 ขณะ
    pub fn is_complete(&self) -> bool {
        self.moments.len() == Self::FULL_MOMENTS
    }

    /// `true` เมื่อ PDP ปฏิเสธ — ไม่มี execution window
    pub fn is_aborted(&self) -> bool {
        self.outcome.is_blocked()
    }

    pub fn outcome(&self) -> &PdpOutcome {
        &self.outcome
    }

    /// ขณะที่ทำหน้าที่ PDP (โวฏฐัพพนะ) — `None` ถ้าวิถีสั้นเกินไป
    pub fn pdp_moment(&self) -> Option<&CittaJati17> {
        self.moments.get(Self::PDP_INDEX)
    }

    /// ลำดับชื่อขั้น OS ของทุกขณะที่เกิดขึ้น — ใช้ตรวจว่าลำดับถูกต้อง
    pub fn os_trace(&self) -> Vec<&'static str> {
        self.moments.iter().map(|m| m.os_stage()).collect()
    }

    /// อกุศลหรือไม่ — พิจารณาจากเจตสิกที่ประกอบชวนะ
    ///
    /// วิถีที่ถูกบล็อกไม่มีชวนะ จึงคืน `false` ตามหลัก "ยังไม่มีการสั่งสมกรรม"
    pub fn is_akusala(&self) -> bool {
        self.moments
            .iter()
            .filter(|m| matches!(m, CittaJati17::Javana(_)))
            .any(|m| m.cetasika().is_akusala())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn akusala() -> CetasikaVector {
        CetasikaVector {
            dosa: 0.9,
            ..Default::default()
        }
    }

    #[test]
    fn vithi_has_17_moments_for_every_door() {
        for dvara in Dvara::ALL {
            let vithi = Vithi17::from_external_event(dvara, "event", CetasikaVector::new());
            assert_eq!(vithi.moment_count(), 17, "ทวาร: {dvara}");
            assert!(vithi.is_complete());
        }
    }

    #[test]
    fn vithi_has_7_javana_and_2_tadalambana() {
        let vithi = Vithi17::from_external_event(Dvara::Cakkhu, "frame", CetasikaVector::new());
        assert_eq!(vithi.javana_count(), JAVANA_COUNT);
        let tadalambana = vithi
            .moments
            .iter()
            .filter(|m| matches!(m, CittaJati17::Tadalambana(_)))
            .count();
        assert_eq!(tadalambana, 2);
    }

    #[test]
    fn moment_order_matches_the_abhidhamma_sequence() {
        let vithi = Vithi17::from_external_event(Dvara::Sota, "audio", CetasikaVector::new());
        let names: Vec<&'static str> = vithi.moments.iter().map(|m| m.name_th()).collect();
        assert_eq!(
            names[0..8],
            [
                "อดีตภวังค์",
                "ภวังคจลนะ",
                "ภวังคุปัจเฉทะ",
                "ปัญจทวาราวัชชนะ",
                "ปัญจวิญญาณ",
                "สัมปฏิจฉนะ",
                "สันตีรณะ",
                "โวฏฐัพพนะ",
            ]
        );
        assert_eq!(names[8], "ชวนะ");
        assert_eq!(names[15], "ตทาลัมพนะ");
        assert_eq!(names[16], "ตทาลัมพนะ");
    }

    #[test]
    fn votthapana_is_the_policy_decision_point() {
        let vithi = Vithi17::from_external_event(Dvara::Kaya, "touch", CetasikaVector::new());
        let pdp = vithi.pdp_moment().expect("ต้องมีโวฏฐัพพนะ");
        assert_eq!(pdp.name_th(), "โวฏฐัพพนะ");
        assert_eq!(pdp.os_stage(), "policy_decision_point");
    }

    #[test]
    fn pancavinnana_carries_the_door_and_is_the_isr() {
        for dvara in Dvara::ALL {
            let vithi = Vithi17::from_external_event(dvara, "e", CetasikaVector::new());
            let vinnana = &vithi.moments[4];
            assert_eq!(vinnana.name_th(), "ปัญจวิญญาณ");
            assert_eq!(vinnana.dvara(), Some(dvara));
            assert_eq!(vinnana.os_stage(), "isr_driver_dma");
        }
    }

    #[test]
    fn blocked_request_never_reaches_the_execution_window() {
        let mut gate = |ctx: &GateContext<'_>| {
            if ctx.cetasika.dosa > 0.7 {
                PdpOutcome::Blocked {
                    reason: "dosa too high".into(),
                }
            } else {
                PdpOutcome::Allowed
            }
        };
        let vithi = Vithi17::run(Dvara::Cakkhu, "frame", akusala(), &mut gate);

        assert!(vithi.is_aborted());
        assert_eq!(vithi.moment_count(), 8, "หยุดที่โวฏฐัพพนะ");
        assert_eq!(vithi.javana_count(), 0, "ไม่มีชวนะ");
        assert!(!vithi
            .moments
            .iter()
            .any(|m| matches!(m, CittaJati17::Tadalambana(_))));
        assert!(!vithi.is_akusala(), "ยังไม่มีการสั่งสมกรรม");
        assert_eq!(vithi.os_trace().last(), Some(&"policy_decision_point"));
    }

    #[test]
    fn throttled_request_still_executes() {
        let mut gate = |_: &GateContext<'_>| PdpOutcome::Throttled {
            reason: "rate limit".into(),
        };
        let vithi = Vithi17::run(Dvara::Ghana, "smell", CetasikaVector::new(), &mut gate);

        assert!(!vithi.is_aborted());
        assert_eq!(vithi.moment_count(), 17);
        assert_eq!(vithi.javana_count(), JAVANA_COUNT);
        assert_eq!(vithi.outcome().reason(), Some("rate limit"));
    }

    #[test]
    fn allowed_request_passes_through_allow_all() {
        let vithi = Vithi17::from_external_event(Dvara::Jivha, "taste", CetasikaVector::new());
        assert_eq!(vithi.outcome(), &PdpOutcome::Allowed);
        assert!(!vithi.is_aborted());
        assert!(vithi.is_complete());
    }

    #[test]
    fn akusala_cetasika_makes_a_completed_vithi_akusala() {
        let vithi = Vithi17::from_external_event(Dvara::Kaya, "hit", akusala());
        assert!(vithi.is_akusala(), "ชวนะมี dosa 0.9");
    }

    #[test]
    fn every_os_stage_name_is_present() {
        let vithi = Vithi17::from_external_event(Dvara::Cakkhu, "frame", CetasikaVector::new());
        let trace = vithi.os_trace();
        assert_eq!(
            &trace[0..8],
            &[
                "idle_loop_previous_cycle",
                "interrupt_line_asserted",
                "context_switch",
                "interrupt_vector_dispatch",
                "isr_driver_dma",
                "kernel_ring_buffer",
                "classifier",
                "policy_decision_point",
            ]
        );
        assert_eq!(trace[8], "execution_window");
        assert_eq!(trace[16], "flush_and_return");
        assert!(trace.iter().all(|s| !s.is_empty()));
    }

    #[test]
    fn dvara_display_uses_thai_names() {
        assert_eq!(Dvara::Cakkhu.to_string(), "จักขุทวาร");
        assert_eq!(Dvara::Kaya.os_channel(), "device:touch");
    }
}
