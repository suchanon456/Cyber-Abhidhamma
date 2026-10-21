// core/src/paramattha/mod.rs
pub mod mahabhuta;
pub mod vithi;

pub use mahabhuta::{
    MahabhutaRupa,
    DhammaViolation,
    Lakkhana,
    PathaviDriver,
    ApoDriver,
    TejoDriver,
    VayoDriver,
    paramattha_execute
};
pub use vithi::Vithi15;
