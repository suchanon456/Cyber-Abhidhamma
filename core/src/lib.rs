//! # Cyber-Abhidhamma Core
//!
//! Core engine for Cyber-Abhidhamma — maps Abhidhamma concepts
//! (Citta 121, Cetasika 52, Vīthi-Citta 17/15) to computable
//! system architectures.
//!
//! ## Modules
//!
//! - [`citta`] — Citta 121 state machine
//! - [`monitor`] — Runtime monitors (lobha, dosa, etc.)
//!
//! ## Status
//!
//! Early prototype. Not production-ready.
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
