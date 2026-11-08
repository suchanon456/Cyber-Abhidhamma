// core/src/paramattha/mod.rs
pub mod cetasika;
pub mod kamma_impact;
pub mod mahabhuta;
pub mod vithi;
pub mod vithi17;
pub mod vithi_mapper;

pub use kamma_impact::{
    RupaCluster, RupaImpact, AMBIENT_C, CALM_PER_KUSALA, COHESION_LOSS_PER_LOBHA,
    COOLING_PER_SATI, HEAT_PER_DOSA, RECOVERY_PER_METTA, RELIEF_PER_KUSALA,
    TURBULENCE_PER_MOHA, WEAR_PER_AKUSALA,
};
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
