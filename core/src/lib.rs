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

pub mod citta;
pub mod monitor;

pub use citta::{citta_knows, citta_moment, citta_stream, citta_anatta, citta_with_cetasika, Cognition};
pub use monitor::lobha::{Lobha, LobhaWithFields};
