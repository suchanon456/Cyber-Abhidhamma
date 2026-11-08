// core/src/firewall/stages.rs
// Concrete stages and the input normalizer.
//
// Mapping to the eightfold labels is for documentation only:
//   right_view (normalize)          -> decode input before matching
//   right_speech (payload)          -> PayloadPatterns
//   right_action (route)            -> AllowedRoutes
//   right_effort (rate)             -> RateLimit
// Not implemented yet: intent, livelihood (quota), concentration (tenant isolation).

use std::collections::{HashMap, VecDeque};

use super::{Decision, Request, Stage};

/// Percent-decodes up to 3 layers, then lowercases.
/// Layers are capped so that a crafted input cannot loop forever.
pub fn normalize(input: &str) -> String {
    let mut current = input.to_string();
    for _ in 0..3 {
        let decoded = percent_decode(&current);
        if decoded == current {
            break;
        }
        current = decoded;
    }
    current.to_lowercase()
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let byte = std::str::from_utf8(&bytes[i + 1..i + 3])
                .ok()
                .and_then(|hex| u8::from_str_radix(hex, 16).ok());
            if let Some(b) = byte {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Blocks known-bad substrings in the normalized path and body.
pub struct PayloadPatterns {
    patterns: Vec<&'static str>,
}

impl Default for PayloadPatterns {
    fn default() -> Self {
        Self {
            patterns: vec![
                "' or 1=1",
                "union select",
                "drop table",
                "<script",
                "javascript:",
                "../",
                "..\\",
            ],
        }
    }
}

impl Stage for PayloadPatterns {
    fn name(&self) -> &'static str {
        "right_speech_payload"
    }

    fn check(&mut self, req: &Request) -> Decision {
        let haystack = format!("{} {}", normalize(&req.path), normalize(&req.body));
        match self.patterns.iter().find(|p| haystack.contains(*p)) {
            Some(p) => Decision::Block {
                stage: self.name(),
                reason: format!("pattern `{p}` matched"),
            },
            None => Decision::Allow,
        }
    }
}

/// Only paths starting with one of the prefixes are allowed.
pub struct AllowedRoutes {
    prefixes: Vec<String>,
}

impl AllowedRoutes {
    pub fn new(prefixes: Vec<String>) -> Self {
        Self { prefixes }
    }
}

impl Stage for AllowedRoutes {
    fn name(&self) -> &'static str {
        "right_action_route"
    }

    fn check(&mut self, req: &Request) -> Decision {
        let path = normalize(&req.path);
        if self.prefixes.iter().any(|p| path.starts_with(p.as_str())) {
            Decision::Allow
        } else {
            Decision::Block {
                stage: self.name(),
                reason: "route not allowed".into(),
            }
        }
    }
}

/// At most `max` accepted requests per actor in any `window_ms` span.
/// Throttled requests are not counted.
pub struct RateLimit {
    max: usize,
    window_ms: u64,
    seen: HashMap<String, VecDeque<u64>>,
}

impl RateLimit {
    pub fn new(max: usize, window_ms: u64) -> Self {
        Self {
            max,
            window_ms,
            seen: HashMap::new(),
        }
    }
}

impl Stage for RateLimit {
    fn name(&self) -> &'static str {
        "right_effort_rate"
    }

    fn check(&mut self, req: &Request) -> Decision {
        let window = self.window_ms;
        let queue = self.seen.entry(req.actor.clone()).or_default();
        while let Some(&t) = queue.front() {
            if req.now_ms.saturating_sub(t) >= window {
                queue.pop_front();
            } else {
                break;
            }
        }
        if queue.len() >= self.max {
            return Decision::Throttle {
                reason: format!("more than {} requests in {} ms", self.max, window),
            };
        }
        queue.push_back(req.now_ms);
        Decision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(path: &str, body: &str) -> Request {
        Request {
            actor: "a".into(),
            path: path.into(),
            body: body.into(),
            now_ms: 0,
        }
    }

    #[test]
    fn normalize_decodes_and_lowercases() {
        assert_eq!(normalize("%27%20OR%201%3D1"), "' or 1=1");
    }

    #[test]
    fn normalize_handles_double_encoding() {
        assert_eq!(normalize("%252e%252e%252f"), "../");
    }

    #[test]
    fn normalize_leaves_bad_escape_alone() {
        assert_eq!(normalize("100%zz"), "100%zz");
    }

    #[test]
    fn payload_blocks_encoded_traversal() {
        let mut s = PayloadPatterns::default();
        assert!(matches!(
            s.check(&req("/api/%2e%2e%2fetc", "")),
            Decision::Block { .. }
        ));
    }

    #[test]
    fn payload_allows_benign_text() {
        let mut s = PayloadPatterns::default();
        assert_eq!(s.check(&req("/api/search", "q=red shoes")), Decision::Allow);
    }

    #[test]
    fn routes_reject_unlisted_prefix() {
        let mut s = AllowedRoutes::new(vec!["/api/".into()]);
        assert_eq!(s.check(&req("/api/x", "")), Decision::Allow);
        assert!(matches!(
            s.check(&req("/admin", "")),
            Decision::Block { .. }
        ));
    }

    #[test]
    fn rate_limit_is_per_actor() {
        let mut s = RateLimit::new(1, 1000);
        let mut a = req("/api/x", "");
        assert_eq!(s.check(&a), Decision::Allow);
        assert!(matches!(s.check(&a), Decision::Throttle { .. }));
        a.actor = "b".into();
        assert_eq!(s.check(&a), Decision::Allow);
    }
}
