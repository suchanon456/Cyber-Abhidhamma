// core/src/lib.rs
// Cyber-Abhidhamma - Rust Core v2
// Paramattha OS - Rupa 28 as On-board Firmware

pub mod citta;
pub mod paramattha;
pub mod monitor;

pub use paramattha::mahabhuta::{
    MahabhutaRupa, 
    CetasikaVector, 
    DhammaViolation, 
    Lakkhana,
    PathaviDriver,
    ApoDriver,
    TejoDriver,
    VayoDriver,
    paramattha_execute
};
