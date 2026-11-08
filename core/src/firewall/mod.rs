// core/src/firewall/mod.rs
// Request gateway: runs an ordered list of stages and stops at the first
// non-Allow decision. Every decision is written to an in-memory audit log.
//
// Labels such as "right_speech_payload" are names for engineering checks,
// not claims that the Noble Eightfold Path is implemented in full.

pub mod stages;

use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub actor: String,
    pub path: String,
    pub body: String,
    pub now_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Decision {
    Allow,
    Throttle { reason: String },
    Block { stage: &'static str, reason: String },
}

/// One check in the pipeline. `check` returns `Decision::Allow` to pass.
pub trait Stage {
    fn name(&self) -> &'static str;
    fn check(&mut self, req: &Request) -> Decision;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditEntry {
    pub stage: &'static str,
    pub decision: Decision,
    pub actor: String,
    pub at_ms: u64,
}

pub struct Firewall {
    stages: Vec<Box<dyn Stage>>,
    audit: Vec<AuditEntry>,
}

impl Firewall {
    pub fn new(stages: Vec<Box<dyn Stage>>) -> Self {
        Self {
            stages,
            audit: Vec::new(),
        }
    }

    /// Runs the stages in order. The first non-Allow decision wins.
    pub fn inspect(&mut self, req: &Request) -> Decision {
        for stage in self.stages.iter_mut() {
            let decision = stage.check(req);
            if decision != Decision::Allow {
                self.audit.push(AuditEntry {
                    stage: stage.name(),
                    decision: decision.clone(),
                    actor: req.actor.clone(),
                    at_ms: req.now_ms,
                });
                return decision;
            }
        }
        self.audit.push(AuditEntry {
            stage: "pipeline",
            decision: Decision::Allow,
            actor: req.actor.clone(),
            at_ms: req.now_ms,
        });
        Decision::Allow
    }

    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit
    }

    /// Audit log as JSON Lines (one object per line).
    pub fn audit_jsonl(&self) -> Result<String, serde_json::Error> {
        let mut out = String::new();
        for entry in &self.audit {
            out.push_str(&serde_json::to_string(entry)?);
            out.push('\n');
        }
        Ok(out)
    }
}

/// Counts decisions by variant name, for quick summaries.
pub fn summarize(audit: &[AuditEntry]) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for entry in audit {
        let key = match entry.decision {
            Decision::Allow => "allow",
            Decision::Throttle { .. } => "throttle",
            Decision::Block { .. } => "block",
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::stages::{AllowedRoutes, PayloadPatterns, RateLimit};
    use super::*;

    fn req(path: &str, body: &str, now_ms: u64) -> Request {
        Request {
            actor: "alice".into(),
            path: path.into(),
            body: body.into(),
            now_ms,
        }
    }

    fn pipeline() -> Firewall {
        Firewall::new(vec![
            Box::new(PayloadPatterns::default()),
            Box::new(AllowedRoutes::new(vec!["/api/".into()])),
            Box::new(RateLimit::new(3, 1000)),
        ])
    }

    #[test]
    fn benign_request_is_allowed_and_audited() {
        let mut fw = pipeline();
        assert_eq!(
            fw.inspect(&req("/api/items", "q=shoes", 0)),
            Decision::Allow
        );
        assert_eq!(fw.audit_log().len(), 1);
        assert_eq!(fw.audit_log()[0].stage, "pipeline");
    }

    #[test]
    fn first_blocking_stage_wins_and_is_audited() {
        let mut fw = pipeline();
        let d = fw.inspect(&req("/api/items", "q=' OR 1=1 --", 0));
        assert!(matches!(
            d,
            Decision::Block {
                stage: "right_speech_payload",
                ..
            }
        ));
        assert_eq!(fw.audit_log()[0].stage, "right_speech_payload");
    }

    #[test]
    fn rate_limit_throttles_then_recovers_after_window() {
        let mut fw = pipeline();
        for t in 0..3 {
            assert_eq!(fw.inspect(&req("/api/x", "", t)), Decision::Allow);
        }
        assert!(matches!(
            fw.inspect(&req("/api/x", "", 10)),
            Decision::Throttle { .. }
        ));
        assert_eq!(fw.inspect(&req("/api/x", "", 1000)), Decision::Allow);
    }

    #[test]
    fn audit_jsonl_has_one_line_per_request() {
        let mut fw = pipeline();
        fw.inspect(&req("/api/x", "", 0));
        fw.inspect(&req("/admin", "", 1));
        let text = fw.audit_jsonl().unwrap();
        assert_eq!(text.lines().count(), 2);
        assert!(text.contains("right_action_route"));
        let counts = summarize(fw.audit_log());
        assert_eq!(counts.get("allow"), Some(&1));
        assert_eq!(counts.get("block"), Some(&1));
    }
}
