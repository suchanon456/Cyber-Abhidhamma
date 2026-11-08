// core/src/authority.rs
// Turns a Vattapada Report into a network Ring and a capability set.
//
// Ring numbers follow the spec: higher ring = less privilege.
// The final ring is the LEAST privileged of: the tier's base ring, and every
// ring transition whose `vatta_N < X` condition is triggered.

use crate::vattapada::{Report, Tier};

/// Network privilege level. Declared from most to least privileged, so `max`
/// gives the less privileged ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ring {
    Ring0,
    Ring1,
    Ring2,
    Ring3,
    /// Preta tier: no network calls at all.
    Quarantine,
}

impl Ring {
    fn from_number(n: u8) -> Ring {
        match n {
            0 => Ring::Ring0,
            1 => Ring::Ring1,
            2 => Ring::Ring2,
            _ => Ring::Ring3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    KernelAccess,
    PriorityBroadcast,
    PriorityCompute,
    MfaRequired,
    RateLimited,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Authority {
    pub ring: Ring,
    pub capabilities: Vec<Capability>,
    /// Human-readable reasons for the ring, one per applied rule.
    pub reasons: Vec<String>,
}

/// Maps a report to an authority. Stateless: the same report gives the same result.
pub fn evaluate(report: &Report) -> Authority {
    let mut ring = match report.tier {
        Tier::SakkaKingOfDevas | Tier::Deva => Ring::Ring0,
        Tier::Human => Ring::Ring1,
        Tier::Asura => Ring::Ring3,
        Tier::Preta => Ring::Quarantine,
    };
    let mut reasons = vec![format!("tier {:?}", report.tier)];

    for v in report.vatta.iter().filter(|v| v.ring_triggered) {
        if let Some(target) = v.ring_target {
            let candidate = Ring::from_number(target);
            if candidate > ring {
                ring = candidate;
                reasons.push(format!("{} below threshold -> Ring {}", v.id, target));
            }
        }
    }

    Authority {
        ring,
        capabilities: capabilities_for(ring),
        reasons,
    }
}

fn capabilities_for(ring: Ring) -> Vec<Capability> {
    use Capability::*;
    match ring {
        Ring::Ring0 => vec![KernelAccess, PriorityBroadcast, PriorityCompute],
        Ring::Ring1 => vec![PriorityBroadcast, MfaRequired],
        Ring::Ring2 => vec![MfaRequired],
        Ring::Ring3 => vec![MfaRequired, RateLimited],
        Ring::Quarantine => vec![RateLimited],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vattapada::VattaScore;

    fn vatta(id: &str, triggered: bool, target: Option<u8>) -> VattaScore {
        VattaScore {
            id: id.to_string(),
            score: if triggered { 0.1 } else { 0.9 },
            ring_transition: None,
            ring_triggered: triggered,
            ring_target: target,
        }
    }

    fn report(tier: Tier, vatta: Vec<VattaScore>) -> Report {
        Report {
            vatta,
            indra_score: 0.0,
            tier,
        }
    }

    #[test]
    fn sakka_without_triggers_gets_kernel_access() {
        let a = evaluate(&report(
            Tier::SakkaKingOfDevas,
            vec![vatta("vatta_1", false, Some(3))],
        ));
        assert_eq!(a.ring, Ring::Ring0);
        assert!(a.capabilities.contains(&Capability::KernelAccess));
    }

    #[test]
    fn triggered_vatta_moves_sakka_to_ring3() {
        let a = evaluate(&report(
            Tier::SakkaKingOfDevas,
            vec![vatta("vatta_1", true, Some(3))],
        ));
        assert_eq!(a.ring, Ring::Ring3);
        assert!(!a.capabilities.contains(&Capability::KernelAccess));
        assert!(a.capabilities.contains(&Capability::RateLimited));
    }

    #[test]
    fn least_privileged_ring_wins_not_the_last_one() {
        let a = evaluate(&report(
            Tier::Deva,
            vec![
                vatta("vatta_1", true, Some(3)),
                vatta("vatta_2", true, Some(1)),
            ],
        ));
        assert_eq!(a.ring, Ring::Ring3);
        // Only the transition that actually lowered the ring is recorded.
        assert_eq!(a.reasons.len(), 2);
        assert!(a.reasons[1].contains("vatta_1"));
    }

    #[test]
    fn preta_is_quarantined_regardless_of_vatta() {
        let a = evaluate(&report(Tier::Preta, vec![]));
        assert_eq!(a.ring, Ring::Quarantine);
    }

    #[test]
    fn human_gets_mfa() {
        let a = evaluate(&report(Tier::Human, vec![]));
        assert_eq!(a.ring, Ring::Ring1);
        assert!(a.capabilities.contains(&Capability::MfaRequired));
    }
}
