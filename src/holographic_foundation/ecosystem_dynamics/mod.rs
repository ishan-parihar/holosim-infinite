//! Phase 12: Ecosystem Dynamics + Food Webs
//!
//! This module implements ecosystems as field resonance networks, where
//! species emerge as specific field configuration patterns and ecological
//! relationships are field coupling dynamics.
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Ecosystems emerge as FIELD RESONANCE NETWORKS.
//!  Food Web as Field Coupling:
//!  - Species = specific field configuration pattern
//!  - Trophic levels = field coherence hierarchy
//!  - Energy flow = field amplitude transfer
//!  - Population growth = field amplitude growth
//!  - Oscillations = field resonance between coupled species
//!  - Extinction = field configuration instability"
//!
//! # Key Components
//!
//! - `species_field`: Species as field configuration patterns
//! - `trophic_coupling`: Trophic levels and food web dynamics
//! - `population_dynamics`: Population dynamics from field amplitude
//! - `spatial_ecosystem`: Patches, corridors, spatial structure
//! - `coevolution`: Co-evolution through field co-adaptation
//! - `ecosystem_field`: Unified ecosystem field

pub mod coevolution;
pub mod ecosystem_field;
pub mod population_dynamics;
pub mod spatial_ecosystem;
pub mod species_field;
pub mod trophic_coupling;

pub use coevolution::{
    AdaptationDirection, CoevolutionPair, CoevolutionRelationship, CoevolutionSystem,
    FitnessLandscape,
};
pub use ecosystem_field::{
    EcosystemField, EcosystemHealth, EcosystemId, EcosystemState, EnergyBudget,
};
pub use population_dynamics::{
    CarryingCapacity, OscillationPattern, Population, PopulationDynamics, PopulationId,
};
pub use spatial_ecosystem::{
    Corridor, EcologicalPatch, PatchConnectivity, PatchId, PatchType, SpatialEcosystem,
};
pub use species_field::{Species, SpeciesFieldPattern, SpeciesId, SpeciesInteraction, SpeciesType};
pub use trophic_coupling::{
    EnergyFlow, TrophicCoupling, TrophicLevel, TrophicLink, TrophicNetwork, TrophicNode,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _species_id = SpeciesId::new(1);
        let _trophic_level = TrophicLevel::Level0;
        let _population_id = PopulationId::new(1);
        // Test passes if types can be created
    }
}
