//! Citta — a single moment of mind.
//!
//! For now, this is a simple state holder. Future versions will
//! implement the full 121-citta state machine.

use serde::{Deserialize, Serialize};

/// The 3 canonical groups of citta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CittaGroup {
    Akusala,   // 12 unwholesome
    Ahetuka,   // 18 rootless
    KamavacaraSobhana, // 24 sense-sphere beautiful
    Rupavacara,        // 15 form-sphere
    Arupavacara,       // 12 formless
    Lokuttara,         // 8/40 supramundane
}

/// A single citta instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citta {
    pub id: u8,
    pub name: String,
    pub group: CittaGroup,
    pub cetasika_ids: Vec<u8>,
}

impl Citta {
    pub fn new(id: u8, name: impl Into<String>, group: CittaGroup) -> Self {
        Self {
            id,
            name: name.into(),
            group,
            cetasika_ids: Vec::new(),
        }
    }

    pub fn with_cetasika(mut self, ids: Vec<u8>) -> Self {
        self.cetasika_ids = ids;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citta_creation() {
        let c = Citta::new(1, "Lobha-mula citta", CittaGroup::Akusala);
        assert_eq!(c.id, 1);
        assert_eq!(c.group, CittaGroup::Akusala);
    }
}
