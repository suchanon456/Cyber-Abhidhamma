// core/src/firewall/vithi_gate.rs
// Adapter: firewall pipeline → โวฏฐัพพนะ (PDP) ของวิถีปัญจทวาร (Vithi 17).
//
// ผลของ firewall แปลงเป็น PdpOutcome ดังนี้
//   Allow    → Allowed    (วิถีดำเนินครบ 17 ขณะ)
//   Throttle → Throttled  (ยังเข้า execution window แต่ถูกบันทึกว่าช้า)
//   Block    → Blocked    (วิถีหยุดที่โวฏฐัพพนะ 8 ขณะ ไม่มีชวนะ)
//
// การแมปนี้เป็น analogy ตามกติกาใน docs/GLOSSARY.md — ไม่ใช่การจำลองการทำงานของ kernel จริง

use crate::paramattha::vithi::CetasikaVector;
use crate::paramattha::vithi17::{Dvara, Gate, GateContext, PdpOutcome, Vithi17};

use super::{Decision, Firewall, Request};

/// ผลรวมของการรันหนึ่งคำขอผ่านวิถี 17
#[derive(Debug, Clone)]
pub struct VithiVerdict {
    pub vithi: Vithi17,
    pub decision: Decision,
}

/// แปลงผลของ firewall เป็นผลของ PDP
pub fn to_outcome(decision: &Decision) -> PdpOutcome {
    match decision {
        Decision::Allow => PdpOutcome::Allowed,
        Decision::Throttle { reason } => PdpOutcome::Throttled {
            reason: reason.clone(),
        },
        Decision::Block { stage, reason } => PdpOutcome::Blocked {
            reason: format!("{stage}: {reason}"),
        },
    }
}

/// Gate ที่ส่งคำขอเข้า firewall ที่ขั้นโวฏฐัพพนะ และเก็บ Decision ดิบไว้
struct FirewallGate<'a> {
    firewall: &'a mut Firewall,
    request: &'a Request,
    decision: Option<Decision>,
}

impl Gate for FirewallGate<'_> {
    fn decide(&mut self, _ctx: &GateContext<'_>) -> PdpOutcome {
        let decision = self.firewall.inspect(self.request);
        let outcome = to_outcome(&decision);
        self.decision = Some(decision);
        outcome
    }
}

/// รันคำขอหนึ่งรายการผ่านวิถี 17 โดยให้ firewall เป็นโวฏฐัพพนะ
///
/// `cetasika` คือเจตสิกที่ประกอบวิถีนี้ (เช่น โมหะ/โทสะของผู้ส่ง) — ไม่ได้คำนวณจาก request
/// และ firewall ไม่ได้ใช้ค่านี้ตัดสิน จึงใช้เพื่อบันทึกและตรวจสอบเท่านั้น
pub fn inspect_through_vithi(
    firewall: &mut Firewall,
    dvara: Dvara,
    request: &Request,
    cetasika: CetasikaVector,
) -> VithiVerdict {
    let mut gate = FirewallGate {
        firewall,
        request,
        decision: None,
    };
    let vithi = Vithi17::run(dvara, &request.path, cetasika, &mut gate);
    // Vithi17::run เรียก decide() ทุกครั้งเสมอ จึงมีค่าเสมอ
    let decision = gate
        .decision
        .expect("Vithi17::run always consults the gate once");
    VithiVerdict { vithi, decision }
}

#[cfg(test)]
mod tests {
    use super::super::stages::{AllowedRoutes, PayloadPatterns, RateLimit};
    use super::*;
    use crate::paramattha::vithi17::CittaJati17;

    fn pipeline() -> Firewall {
        Firewall::new(vec![
            Box::new(PayloadPatterns::default()),
            Box::new(AllowedRoutes::new(vec!["/api/".into()])),
            Box::new(RateLimit::new(2, 1000)),
        ])
    }

    fn req(path: &str, body: &str, now_ms: u64) -> Request {
        Request {
            actor: "alice".into(),
            path: path.into(),
            body: body.into(),
            now_ms,
        }
    }

    #[test]
    fn allow_runs_the_full_17_moments() {
        let mut fw = pipeline();
        let v = inspect_through_vithi(
            &mut fw,
            Dvara::Cakkhu,
            &req("/api/items", "q=shoes", 0),
            CetasikaVector::new(),
        );
        assert_eq!(v.decision, Decision::Allow);
        assert_eq!(v.vithi.outcome(), &PdpOutcome::Allowed);
        assert!(v.vithi.is_complete());
        assert_eq!(v.vithi.javana_count(), 7);
        assert_eq!(fw.audit_log().len(), 1);
    }

    #[test]
    fn block_stops_at_votthapana_and_is_audited_once() {
        let mut fw = pipeline();
        let v = inspect_through_vithi(
            &mut fw,
            Dvara::Sota,
            &req("/api/items", "q=' OR 1=1 --", 0),
            CetasikaVector::new(),
        );
        assert!(matches!(
            v.decision,
            Decision::Block {
                stage: "right_speech_payload",
                ..
            }
        ));
        assert!(v.vithi.is_aborted());
        assert_eq!(v.vithi.moment_count(), 8);
        assert_eq!(v.vithi.javana_count(), 0);
        assert_eq!(v.vithi.pdp_moment().map(|m| m.name_th()), Some("โวฏฐัพพนะ"));
        assert!(matches!(
            v.vithi.outcome(),
            PdpOutcome::Blocked { reason } if reason.starts_with("right_speech_payload:")
        ));
        assert_eq!(fw.audit_log().len(), 1);
        assert_eq!(fw.audit_log()[0].stage, "right_speech_payload");
    }

    #[test]
    fn route_block_also_stops_the_vithi() {
        let mut fw = pipeline();
        let v = inspect_through_vithi(
            &mut fw,
            Dvara::Kaya,
            &req("/admin", "", 0),
            CetasikaVector::new(),
        );
        assert!(v.vithi.is_aborted());
        assert_eq!(v.vithi.moment_count(), 8);
        assert_eq!(fw.audit_log()[0].stage, "right_action_route");
    }

    #[test]
    fn throttle_still_executes_and_keeps_the_reason() {
        let mut fw = pipeline();
        for t in 0..2 {
            inspect_through_vithi(
                &mut fw,
                Dvara::Ghana,
                &req("/api/x", "", t),
                CetasikaVector::new(),
            );
        }
        let v = inspect_through_vithi(
            &mut fw,
            Dvara::Ghana,
            &req("/api/x", "", 10),
            CetasikaVector::new(),
        );
        assert!(matches!(v.decision, Decision::Throttle { .. }));
        assert!(!v.vithi.is_aborted());
        assert!(v.vithi.is_complete());
        assert!(matches!(v.vithi.outcome(), PdpOutcome::Throttled { .. }));
        assert_eq!(v.vithi.javana_count(), 7);
    }

    #[test]
    fn blocked_vithi_has_no_akusala_javana() {
        let mut fw = pipeline();
        let cetasika = CetasikaVector {
            dosa: 0.9,
            ..Default::default()
        };
        let v = inspect_through_vithi(&mut fw, Dvara::Cakkhu, &req("/admin", "", 0), cetasika);
        assert!(!v.vithi.is_akusala());
        assert!(!v
            .vithi
            .moments
            .iter()
            .any(|m| matches!(m, CittaJati17::Javana(_))));
    }

    #[test]
    fn to_outcome_maps_every_decision() {
        assert_eq!(to_outcome(&Decision::Allow), PdpOutcome::Allowed);
        assert_eq!(
            to_outcome(&Decision::Throttle { reason: "r".into() }),
            PdpOutcome::Throttled { reason: "r".into() }
        );
        assert_eq!(
            to_outcome(&Decision::Block {
                stage: "s",
                reason: "x".into()
            }),
            PdpOutcome::Blocked {
                reason: "s: x".into()
            }
        );
    }
}
