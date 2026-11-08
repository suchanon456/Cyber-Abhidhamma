// core/src/lib.rs
// Cyber-Abhidhamma - Rust Core v2
// Paramattha OS - Rupa 28 + Vithi 15/17

pub mod attention;
pub mod authority;
pub mod citta;
pub mod firewall;
pub mod interceptor;
pub mod monitor;
pub mod paramattha;
pub mod vattapada;

pub use attention::AttentionLoop;
pub use authority::{Authority, Capability, Ring};
pub use interceptor::{Action, Decision};
pub use vattapada::{Report, Tier, VattaScore, VattapadaEngine, VattapadaError};

pub use paramattha::mahabhuta::{
    paramattha_execute, ApoDriver, DhammaViolation, Lakkhana, MahabhutaRupa, PathaviDriver,
    TejoDriver, VayoDriver,
};

// Vithi 15 - มโนทวารวิถี
pub use paramattha::vithi::{CetasikaVector, CittaJati, InternalCause, Vithi15, JAVANA_COUNT};
