// core/src/attention.rs
// Stateful attention score for a node, updated once per tick.
//
// The multipliers come from the spec (vatta_3, vatta_5 policy_as_code):
//   harsh output (vatta_3 dosa > 0.7)         attention *= 0.2
//   high generosity (vatta_5 ... high)        attention *= 10
// Attention is capped to 0.0..=1.0 so a single good tick cannot run away.
// Like the spec, this is a heuristic.

use crate::vattapada::Report;

pub const HARSH_DECAY: f64 = 0.2;
pub const GENEROUS_BOOST: f64 = 10.0;
pub const HARSH_DOSA_LIMIT: f64 = 0.7;
pub const GENEROUS_VATTA5_LIMIT: f64 = 0.8;

#[derive(Debug, Clone, PartialEq)]
pub struct AttentionLoop {
    attention: f64,
}

impl AttentionLoop {
    pub fn new(initial: f64) -> Self {
        Self {
            attention: initial.clamp(0.0, 1.0),
        }
    }

    pub fn attention(&self) -> f64 {
        self.attention
    }

    /// Applies one tick. `dosa` is the input value of dosa for this tick.
    pub fn tick(&mut self, report: &Report, dosa: f64) {
        if dosa > HARSH_DOSA_LIMIT {
            self.attention *= HARSH_DECAY;
        } else if report
            .vatta
            .iter()
            .any(|v| v.id == "vatta_5" && v.score >= GENEROUS_VATTA5_LIMIT)
        {
            self.attention *= GENEROUS_BOOST;
        }
        self.attention = self.attention.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vattapada::{Report, Tier, VattaScore};

    fn report(vatta5: f64) -> Report {
        Report {
            vatta: vec![VattaScore {
                id: "vatta_5".into(),
                score: vatta5,
                ring_transition: None,
                ring_triggered: false,
                ring_target: None,
            }],
            indra_score: 0.0,
            tier: Tier::Human,
        }
    }

    #[test]
    fn harsh_tick_decays() {
        let mut a = AttentionLoop::new(0.5);
        a.tick(&report(0.0), 0.9);
        assert!((a.attention() - 0.1).abs() < 1e-9);
    }

    #[test]
    fn generous_tick_boosts_and_caps_at_one() {
        let mut a = AttentionLoop::new(0.5);
        a.tick(&report(0.9), 0.0);
        assert_eq!(a.attention(), 1.0);
    }

    #[test]
    fn neutral_tick_keeps_attention() {
        let mut a = AttentionLoop::new(0.3);
        a.tick(&report(0.5), 0.1);
        assert!((a.attention() - 0.3).abs() < 1e-9);
    }
}
