// core/src/lib.rs
// Cyber-Abhidhamma - Rust Core v2
// Paramattha OS - Rupa 28 + Vithi 15/17

pub mod citta;
pub mod monitor;
pub mod paramattha;

pub use paramattha::mahabhuta::{
    paramattha_execute, ApoDriver, DhammaViolation, Lakkhana, MahabhutaRupa, PathaviDriver,
    TejoDriver, VayoDriver,
};

// Vithi 15 - มโนทวารวิถี
pub use paramattha::vithi::{CetasikaVector, CittaJati, InternalCause, Vithi15, JAVANA_COUNT};
