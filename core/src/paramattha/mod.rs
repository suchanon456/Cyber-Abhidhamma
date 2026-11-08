// core/src/paramattha/mod.rs
pub mod mahabhuta;
pub mod vithi;
pub mod vithi17;
pub mod vithi_mapper;

pub use mahabhuta::{
    paramattha_execute, ApoDriver, DhammaViolation, Lakkhana, MahabhutaRupa, PathaviDriver,
    TejoDriver, VayoDriver,
};
pub use vithi::Vithi15;
pub use vithi17::{CittaJati17, Dvara, GateContext, PdpOutcome, Vithi17};
pub use vithi_mapper::{
    CittaEntry, CittaGroup, CittaStage, MappedMoment, MappedVithi, MapperError, Vipaka,
    VithiMapper, CITTA_TOTAL,
};
