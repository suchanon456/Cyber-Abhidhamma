//! Lobha — greed cetasika, and its monitor.
//!
//! ## Mapping Rules (Rust ↔ Abhidhamma)
//!
//! - `struct` = เจตสิก (cetasika) — a mental factor
//! - `fn`     = จิต (citta) — an act of knowing
//! - `enum`   = รูป (rūpa) — a category of form
//! - `impl`   = กิจ (kicca) — the function of a cetasika
//! - `field`  = ลักษณะ (lakkhana) — the characteristic
//!
//! ## What is Lobha?
//!
//! Lobha is one of the 14 akusala cetasikas. It is the mental
//! factor of greed / attachment. It arises when the mind contacts
//! a pleasant object (iṭṭhārammaṇa) and clings to it.
//!
//! In a computational system, lobha manifests as:
//! - Hoarding memory without release
//! - Pulling in more context than needed
//! - Holding locks longer than necessary
//! - Infinite request loops
//!
//! ## Structure of this file
//!
//! 1. `Lobha` — the cetasika itself (struct)
//! 2. `citta_*` — functions that model citta (fn)
//! 3. `LobhaMonitor` — the observer (struct)
//! 4. `LobhaObservation` — the result of observation (struct)

use serde::{Deserialize, Serialize};

// =========================================================
// 1. LOBHA — เจตสิก (struct)
// =========================================================

/// Lobha — the cetasika of greed.
///
/// ## Characteristics (ลักษณะ)
///
/// - **Lakkhana** (characteristic): clinging to object
/// - **Rasa** (function): not letting go
/// - **Paccupaṭṭhāna** (manifestation): not releasing
/// - **Padaṭṭhāna** (proximate cause): pleasant object
///
/// ## Fields
///
/// - `intensity`: 0.0 = not active, 1.0 = fully active
/// - `active`: whether lobha is currently arisen
/// - `object`: what lobha is clinging to
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lobha {
    /// Intensity of the cetasika (0.0 to 1.0).
    pub intensity: f32,

    /// Whether lobha is currently arisen.
    pub active: bool,

    /// The object that lobha is clinging to.
    pub object: Option<String>,
}

impl Lobha {
    /// Create a new, inactive lobha.
    pub fn new() -> Self {
        Self {
            intensity: 0.0,
            active: false,
            object: None,
        }
    }

    /// Arise — lobha arises when contacting a pleasant object.
    ///
    /// ## Kicca (กิจ)
    ///
    /// The function of lobha is to cling. When it arises,
    /// it immediately grasps the object.
    pub fn arise(&mut self, intensity: f32, object: impl Into<String>) {
        self.intensity = intensity.clamp(0.0, 1.0);
        self.active = true;
        self.object = Some(object.into());
    }

    /// Cease — lobha ceases when the object fades or is released.
    pub fn cease(&mut self) {
        self.intensity = 0.0;
        self.active = false;
        self.object = None;
    }

    /// Is lobha currently active?
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Current intensity.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Strengthen — lobha grows when it is fed.
    ///
    /// This is how lobha works: the more you give it,
    /// the stronger it becomes.
    pub fn strengthen(&mut self, delta: f32) {
        if self.active {
            self.intensity = (self.intensity + delta).clamp(0.0, 1.0);
        }
    }
}

// =========================================================
// 2. CITTA — จิต (fn)
// =========================================================

/// Citta: "seeing" lobha arise.
///
/// This is the act of knowing that lobha has arisen.
/// It is not lobha itself — it is the awareness of it.
pub fn citta_see_lobha(lobha: &Lobha) -> Option<CittaCognition> {
    if !lobha.is_active() {
        return None;
    }

    Some(CittaCognition {
        object: lobha.object.clone().unwrap_or_default(),
        intensity: lobha.intensity(),
        kind: CittaKind::Akusala,
    })
}

/// Citta: "seeing" lobha cease.
pub fn citta_see_lobha_cease(lobha: &Lobha) -> CittaCognition {
    CittaCognition {
        object: lobha.object.clone().unwrap_or_default(),
        intensity: 0.0,
        kind: CittaKind::Sobhana, // The moment of release is sobhana
    }
}

/// The kind of citta that arises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CittaKind {
    /// Akusala citta — unwholesome
    Akusala,
    /// Sobhana citta — beautiful / wholesome
    Sobhana,
    /// Ahetuka citta — rootless
    Ahetuka,
}

/// The result of a citta — what the mind produced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CittaCognition {
    /// The object of cognition.
    pub object: String,
    /// The intensity at the moment of cognition.
    pub intensity: f32,
    /// The kind of citta.
    pub kind: CittaKind,
}

// =========================================================
// 3. LOBHA MONITOR — รูป (ผลรวม)
// =========================================================

/// LobhaMonitor — an observer of lobha-like behavior.
///
/// ## This is NOT a cetasika.
///
/// LobhaMonitor is a tool — a "rūpa" (result aggregate) that
/// observes the system. It does not cling; it watches.
///
/// ## What it does
///
/// It watches memory allocation patterns and reports
/// when they resemble lobha.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobhaMonitor {
    /// Threshold ratio (allocated / released) above which
    /// the behavior is flagged as lobha-like.
    pub threshold: f32,

    /// History of observations.
    pub history: Vec<LobhaObservation>,
}

/// A single observation — รูป (ผลรวม)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobhaObservation {
    /// Bytes allocated.
    pub allocated: u64,
    /// Bytes released.
    pub released: u64,
    /// Ratio allocated / released.
    pub ratio: f32,
    /// Whether lobha was detected.
    pub lobha_detected: bool,
    /// The lobha cetasika, if detected.
    pub lobha: Option<Lobha>,
}

impl LobhaMonitor {
    /// Create a new monitor.
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold,
            history: Vec::new(),
        }
    }

    /// Observe a memory allocation pattern.
    ///
    /// This is the function (จิต) that "sees" the pattern.
    pub fn observe(&mut self, allocated: u64, released: u64) -> &LobhaObservation {
        let released_safe = released.max(1);
        let ratio = allocated as f32 / released_safe as f32;

        let lobha = if ratio > self.threshold {
            let mut l = Lobha::new();
            l.arise(
                ((ratio / 10.0).min(1.0)),
                format!("memory:allocated={},released={}", allocated, released),
            );
            Some(l)
        } else {
            None
        };

        let observation = LobhaObservation {
            allocated,
            released,
            ratio,
            lobha_detected: lobha.is_some(),
            lobha,
        };

        self.history.push(observation);
        self.history.last().unwrap()
    }

    /// Number of lobha detections.
    pub fn detection_count(&self) -> usize {
        self.history.iter().filter(|o| o.lobha_detected).count()
    }

    /// Total observations.
    pub fn observation_count(&self) -> usize {
        self.history.len()
    }

    /// Clear history.
    pub fn clear(&mut self) {
        self.history.clear();
    }
}

impl Default for LobhaMonitor {
    fn default() -> Self {
        Self::new(2.0)
    }
}

// =========================================================
// 4. TESTS
// =========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lobha_arises() {
        let mut lobha = Lobha::new();
        assert!(!lobha.is_active());

        lobha.arise(0.8, "test_object");
        assert!(lobha.is_active());
        assert_eq!(lobha.intensity(), 0.8);
        assert_eq!(lobha.object.as_deref(), Some("test_object"));
    }

    #[test]
    fn test_lobha_ceases() {
        let mut lobha = Lobha::new();
        lobha.arise(0.8, "test");
        lobha.cease();

        assert!(!lobha.is_active());
        assert_eq!(lobha.intensity(), 0.0);
        assert_eq!(lobha.object, None);
    }

    #[test]
    fn test_lobha_strengthens_when_fed() {
        let mut lobha = Lobha::new();
        lobha.arise(0.5, "test");
        lobha.strengthen(0.3);

        assert_eq!(lobha.intensity(), 0.8);
    }

    #[test]
    fn test_lobha_caps_at_1() {
        let mut lobha = Lobha::new();
        lobha.arise(0.9, "test");
        lobha.strengthen(0.5);

        assert_eq!(lobha.intensity(), 1.0);
    }

    #[test]
    fn test_citta_sees_lobha() {
        let mut lobha = Lobha::new();
        lobha.arise(0.7, "object");

        let cognition = citta_see_lobha(&lobha);
        assert!(cognition.is_some());

        let c = cognition.unwrap();
        assert_eq!(c.intensity, 0.7);
        assert_eq!(c.kind, CittaKind::Akusala);
    }

    #[test]
    fn test_citta_sees_nothing_when_lobha_inactive() {
        let lobha = Lobha::new();
        let cognition = citta_see_lobha(&lobha);
        assert!(cognition.is_none());
    }

    #[test]
    fn test_monitor_detects_lobha() {
        let mut monitor = LobhaMonitor::new(2.0);
        let obs = monitor.observe(1000, 100); // ratio = 10.0

        assert!(obs.lobha_detected);
        assert_eq!(monitor.detection_count(), 1);
    }

    #[test]
    fn test_monitor_does_not_detect_when_balanced() {
        let mut monitor = LobhaMonitor::new(2.0);
        let obs = monitor.observe(1000, 1000); // ratio = 1.0

        assert!(!obs.lobha_detected);
        assert_eq!(monitor.detection_count(), 0);
    }

    #[test]
    fn test_monitor_history() {
        let mut monitor = LobhaMonitor::new(2.0);
        monitor.observe(1000, 100);
        monitor.observe(500, 500);
        monitor.observe(3000, 100);

        assert_eq!(monitor.observation_count(), 3);
        assert_eq!(monitor.detection_count(), 2);
    }

    #[test]
    fn test_monitor_clear() {
        let mut monitor = LobhaMonitor::new(2.0);
        monitor.observe(1000, 100);
        monitor.clear();

        assert_eq!(monitor.observation_count(), 0);
    }
}
