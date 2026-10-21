// core/src/lib.rs
// Cyber-Abhidhamma - Rust Core v2
// Paramattha OS - Rupa 28 + Vithi 15/17

pub mod paramattha;
pub mod citta;
pub mod monitor;

pub use paramattha::mahabhuta::{
    MahabhutaRupa, 
    DhammaViolation, 
    Lakkhana,
    PathaviDriver,
    ApoDriver,
    TejoDriver,
    VayoDriver,
    paramattha_execute
};

// Vithi 15 - มโนทวารวิถี
pub use paramattha::vithi::{
    Vithi15,
    CittaJati,
    CetasikaVector
};
