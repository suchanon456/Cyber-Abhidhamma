// core/src/vattapada.rs
// Vattapada-7 scoring engine.
//
// Reads spec/Vattapada-7.json (embedded at compile time) and scores a
// cetasika vector (each value 0.0..=1.0) against the seven vattapada.
// The spec is a heuristic draft, not a canonical Abhidhamma formula.

use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;
use thiserror::Error;

const SPEC_JSON: &str = include_str!("../../spec/Vattapada-7.json");

#[derive(Debug, Error, PartialEq)]
pub enum VattapadaError {
    #[error("invalid Vattapada spec: {0}")]
    Spec(String),
    #[error("cetasika `{name}` = {value} is outside 0.0..=1.0")]
    OutOfRange { name: String, value: f64 },
    #[error("threshold `{0}` missing or not numeric in spec")]
    MissingThreshold(String),
}

// ---- spec file shape (only the fields the engine needs) ----

#[derive(Deserialize)]
struct SpecFile {
    overall_governance: Governance,
    vattapada_7: Vec<VattaSpec>,
}

#[derive(Deserialize)]
struct Governance {
    thresholds: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct VattaSpec {
    id: String,
    cetasika_formula: Formula,
    #[serde(default)]
    policy_as_code: Option<PolicyAsCode>,
}

#[derive(Deserialize)]
struct Formula {
    weights: BTreeMap<String, f64>,
    #[serde(default)]
    penalties: BTreeMap<String, f64>,
}

#[derive(Deserialize)]
struct PolicyAsCode {
    #[serde(default)]
    ring_transition: Option<String>,
}

// ---- engine types ----

/// Indra tier, decided from the indra score and the spec thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    SakkaKingOfDevas,
    Deva,
    Human,
    Asura,
    Preta,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VattaScore {
    pub id: String,
    pub score: f64,
    /// The ring transition text from the spec, if the vatta has one.
    pub ring_transition: Option<String>,
    /// True when the score is below the `vatta_N < X` bound in that text.
    pub ring_triggered: bool,
    /// The ring number the spec says to move to (the `-> Ring N` part).
    pub ring_target: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub vatta: Vec<VattaScore>,
    pub indra_score: f64,
    pub tier: Tier,
}

#[derive(Debug, Clone)]
struct Rule {
    id: String,
    weights: BTreeMap<String, f64>,
    penalties: BTreeMap<String, f64>,
    ring_transition: Option<String>,
    ring_threshold: Option<f64>,
    ring_target: Option<u8>,
}

#[derive(Debug, Clone, Copy)]
struct TierBounds {
    sakka: f64,
    deva: f64,
    human: f64,
    preta: f64,
}

#[derive(Debug, Clone)]
pub struct VattapadaEngine {
    rules: Vec<Rule>,
    tiers: TierBounds,
}

impl VattapadaEngine {
    /// Engine built from the spec file in the repository.
    pub fn canonical() -> Result<Self, VattapadaError> {
        Self::from_json(SPEC_JSON)
    }

    /// Engine built from a spec JSON string with the same shape.
    pub fn from_json(json: &str) -> Result<Self, VattapadaError> {
        let spec: SpecFile =
            serde_json::from_str(json).map_err(|e| VattapadaError::Spec(e.to_string()))?;

        if spec.vattapada_7.len() != 7 {
            return Err(VattapadaError::Spec(format!(
                "expected 7 vattapada, found {}",
                spec.vattapada_7.len()
            )));
        }

        let thresholds = &spec.overall_governance.thresholds;
        let bound = |key: &str, prefix: &str| -> Result<f64, VattapadaError> {
            thresholds
                .get(key)
                .and_then(|text| parse_number_after(text, prefix))
                .ok_or_else(|| VattapadaError::MissingThreshold(key.to_string()))
        };
        let tiers = TierBounds {
            sakka: bound("sakka_king_of_devas", "indra_score >=")?,
            deva: bound("deva", "indra_score >=")?,
            human: bound("human", "indra_score >=")?,
            preta: bound("preta", "indra_score <")?,
        };

        let rules = spec
            .vattapada_7
            .into_iter()
            .map(|v| {
                let ring_transition = v.policy_as_code.and_then(|p| p.ring_transition);
                let ring_threshold = ring_transition
                    .as_deref()
                    .and_then(|text| parse_number_after(text, &format!("{} <", v.id)));
                let ring_target = ring_transition.as_deref().and_then(parse_ring_target);
                Rule {
                    id: v.id,
                    weights: v.cetasika_formula.weights,
                    penalties: v.cetasika_formula.penalties,
                    ring_transition,
                    ring_threshold,
                    ring_target,
                }
            })
            .collect();

        Ok(Self { rules, tiers })
    }

    /// Scores a cetasika vector. Cetasikas missing from `input` count as 0.0;
    /// every given value must be finite and within 0.0..=1.0.
    pub fn evaluate(&self, input: &HashMap<String, f64>) -> Result<Report, VattapadaError> {
        for (name, &value) in input {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(VattapadaError::OutOfRange {
                    name: name.clone(),
                    value,
                });
            }
        }

        let vatta: Vec<VattaScore> = self
            .rules
            .iter()
            .map(|rule| {
                let score = score_vatta(rule, input);
                VattaScore {
                    id: rule.id.clone(),
                    score,
                    ring_transition: rule.ring_transition.clone(),
                    ring_triggered: rule.ring_threshold.is_some_and(|t| score < t),
                    ring_target: rule.ring_target,
                }
            })
            .collect();

        let scores: Vec<f64> = vatta.iter().map(|v| v.score).collect();
        let indra_score = weighted_harmonic_mean(&scores);

        Ok(Report {
            vatta,
            indra_score,
            tier: self.tier_for(indra_score),
        })
    }

    fn tier_for(&self, indra_score: f64) -> Tier {
        let t = self.tiers;
        if indra_score >= t.sakka {
            Tier::SakkaKingOfDevas
        } else if indra_score >= t.deva {
            Tier::Deva
        } else if indra_score >= t.human {
            Tier::Human
        } else if indra_score >= t.preta {
            Tier::Asura
        } else {
            Tier::Preta
        }
    }
}

/// `weights * values - |penalties| * values`, clamped to 0.0..=1.0.
fn score_vatta(rule: &Rule, input: &HashMap<String, f64>) -> f64 {
    let get = |name: &str| input.get(name).copied().unwrap_or(0.0);
    let positive: f64 = rule.weights.iter().map(|(k, w)| w * get(k)).sum();
    let negative: f64 = rule.penalties.iter().map(|(k, p)| p.abs() * get(k)).sum();
    (positive - negative).clamp(0.0, 1.0)
}

/// Equal-weight harmonic mean. A single zero gives 0.0, so one failed vatta
/// pulls the whole indra score down.
fn weighted_harmonic_mean(values: &[f64]) -> f64 {
    if values.is_empty() || values.iter().any(|&v| v <= 0.0) {
        return 0.0;
    }
    let reciprocal_sum: f64 = values.iter().map(|v| 1.0 / v).sum();
    values.len() as f64 / reciprocal_sum
}

/// Reads the number right after `prefix` in `text`,
/// e.g. `parse_number_after("indra_score >= 0.8 -> ...", "indra_score >=")` = 0.8.
fn parse_number_after(text: &str, prefix: &str) -> Option<f64> {
    let start = text.find(prefix)? + prefix.len();
    text[start..].split_whitespace().next()?.parse().ok()
}

/// Reads the target ring from text like `Ring 0 -> Ring 3 if vatta_1 < 0.3`.
fn parse_ring_target(text: &str) -> Option<u8> {
    parse_number_after(text, "-> Ring ").map(|n| n as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> VattapadaEngine {
        VattapadaEngine::canonical().expect("canonical spec must load")
    }

    fn input(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn canonical_spec_loads_seven_vattas_and_tiers() {
        let e = engine();
        let ids: Vec<&str> = e.rules.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(
            ids,
            ["vatta_1", "vatta_2", "vatta_3", "vatta_4", "vatta_5", "vatta_6", "vatta_7"]
        );
        assert_eq!(e.tiers.sakka, 0.8);
        assert_eq!(e.tiers.deva, 0.6);
        assert_eq!(e.tiers.human, 0.4);
        assert_eq!(e.tiers.preta, 0.2);
    }

    #[test]
    fn ring_thresholds_are_read_from_spec() {
        let e = engine();
        assert_eq!(e.rules[0].ring_threshold, Some(0.3)); // vatta_1 < 0.3
        assert_eq!(e.rules[1].ring_threshold, Some(0.4)); // vatta_2 < 0.4
        assert_eq!(e.rules[6].ring_threshold, Some(0.3)); // vatta_7 < 0.3
    }

    #[test]
    fn all_positive_cetasikas_at_one_gives_full_score() {
        let e = engine();
        let mut values = HashMap::new();
        for rule in &e.rules {
            for name in rule.weights.keys() {
                values.insert(name.clone(), 1.0);
            }
        }
        let report = e.evaluate(&values).unwrap();
        for v in &report.vatta {
            assert!((v.score - 1.0).abs() < 1e-9, "{} = {}", v.id, v.score);
            assert!(!v.ring_triggered);
        }
        assert!((report.indra_score - 1.0).abs() < 1e-9);
        assert_eq!(report.tier, Tier::SakkaKingOfDevas);
    }

    #[test]
    fn penalty_is_subtracted_and_triggers_ring_rule() {
        // vatta_1 = saddha*0.4 + jivitindriya*0.3 + hiri*0.2 + sati*0.1 - moha*0.8
        let e = engine();
        let report = e
            .evaluate(&input(&[
                ("saddha", 1.0),
                ("jivitindriya", 1.0),
                ("hiri", 1.0),
                ("sati", 1.0),
                ("moha", 1.0),
            ]))
            .unwrap();
        let v1 = &report.vatta[0];
        assert!((v1.score - 0.2).abs() < 1e-9, "got {}", v1.score);
        assert!(v1.ring_triggered); // 0.2 < 0.3
    }

    #[test]
    fn score_is_clamped_to_zero() {
        let e = engine();
        let report = e.evaluate(&input(&[("moha", 1.0)])).unwrap();
        assert_eq!(report.vatta[0].score, 0.0);
        assert_eq!(report.indra_score, 0.0);
        assert_eq!(report.tier, Tier::Preta);
    }

    #[test]
    fn missing_cetasikas_count_as_zero() {
        let e = engine();
        let report = e.evaluate(&HashMap::new()).unwrap();
        assert!(report.vatta.iter().all(|v| v.score == 0.0));
    }

    #[test]
    fn harmonic_mean_is_dragged_down_by_one_zero() {
        assert_eq!(
            weighted_harmonic_mean(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0]),
            0.0
        );
        assert!((weighted_harmonic_mean(&[0.5; 7]) - 0.5).abs() < 1e-9);
        // 2 / (1/1.0 + 1/0.25) = 0.4
        assert!((weighted_harmonic_mean(&[1.0, 0.25]) - 0.4).abs() < 1e-9);
    }

    #[test]
    fn one_element_at_four_elevenths_is_the_sakka_boundary() {
        // Six perfect vatta plus one at 4/11: 7 / (6 + 2.75) = 0.8 exactly.
        let at = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 4.0 / 11.0];
        assert!((weighted_harmonic_mean(&at) - 0.8).abs() < 1e-9);
        let below = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.36];
        assert!(weighted_harmonic_mean(&below) < 0.8);
    }

    #[test]
    fn tier_boundaries() {
        let e = engine();
        assert_eq!(e.tier_for(0.8), Tier::SakkaKingOfDevas);
        assert_eq!(e.tier_for(0.79), Tier::Deva);
        assert_eq!(e.tier_for(0.6), Tier::Deva);
        assert_eq!(e.tier_for(0.4), Tier::Human);
        assert_eq!(e.tier_for(0.2), Tier::Asura);
        assert_eq!(e.tier_for(0.19), Tier::Preta);
        assert_eq!(e.tier_for(0.0), Tier::Preta);
    }

    #[test]
    fn out_of_range_input_is_rejected() {
        let e = engine();
        let err = e.evaluate(&input(&[("saddha", 1.5)])).unwrap_err();
        assert!(matches!(err, VattapadaError::OutOfRange { .. }));
        let err = e.evaluate(&input(&[("saddha", f64::NAN)])).unwrap_err();
        assert!(matches!(err, VattapadaError::OutOfRange { .. }));
    }

    #[test]
    fn bad_spec_is_rejected() {
        assert!(matches!(
            VattapadaEngine::from_json("{}"),
            Err(VattapadaError::Spec(_))
        ));
    }
}
