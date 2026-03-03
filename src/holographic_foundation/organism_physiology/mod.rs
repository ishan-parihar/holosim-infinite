//! Phase 11: Organism Physiology + Organ Systems
//!
//! This module implements the emergence of organism physiology as specialized
//! field configurations. Organ systems emerge as field nodes with archetype
//! resonance patterns.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Organ systems emerge as SPECIALIZED FIELD CONFIGURATIONS.
//!  Each organ is a field node with archetype resonance."
//!
//! # Organ Systems as Archetype Specializations
//!
//! | System | Dominant Archetype | Function |
//! |--------|-------------------|----------|
//! | Nervous | Mind (A1-A7) | Information processing |
//! | Circulatory | Catalyst (A3) | Energy distribution |
//! | Respiratory | Transformation (A6) | Energy exchange |
//! | Digestive | Experience (A4) | Matter transformation |
//! | Immune | Significator (A5) | Identity defense |
//! | Endocrine | Potentiator (A2) | Chemical signaling |
//! | Reproductive | Great Way (A7) | Continuation |
//!
//! # Key Components
//!
//! - `organ_field`: Organs as field nodes with archetype resonance
//! - `tissue_coherence`: Tissue as coherent cell fields
//! - `organ_systems`: Individual organ system implementations
//! - `physiology_engine`: Organ communication via field wave propagation
//! - `disease_healing`: Disease as field distortion, healing as realignment
//! - `organism_field`: Organism as unified organ field

pub mod disease_healing;
pub mod organ_field;
pub mod organ_systems;
pub mod organism_field;
pub mod physiology_engine;
pub mod tissue_coherence;

pub use disease_healing::{
    DiseaseState, DiseaseType, FieldDistortion, HealingMechanism, HealingResult, HealingSystem,
};
pub use organ_field::{
    Organ, OrganFieldNode, OrganHealth, OrganId, OrganState, OrganType, OrganVitality,
};
pub use organ_systems::{
    CirculatorySystem, DigestiveSystem, EndocrineSystem, ImmuneSystem, NervousSystem,
    OrganSystemCoordinator, OrganSystemId, OrganSystemType, ReproductiveSystem, RespiratorySystem,
};
pub use organism_field::{
    BodyConsciousness, OrganismField, OrganismId, OrganismState, OrganismVitality,
};
pub use physiology_engine::{
    FieldWave, OrganCommunication, PhysiologyEngine, PhysiologySignal, SignalType,
};
pub use tissue_coherence::{Tissue, TissueCoherence, TissueId, TissueType, TissueVitality};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _id = OrganId::new(1);
        let _tissue_id = TissueId::new(1);
        assert!(true);
    }
}
