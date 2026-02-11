// Biological Emergence - Phase 3
//
// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 3:
// "Implement true biological emergence from holographic blueprint"
//
// This module implements:
// 1. DNA/RNA System - Encode holographic blueprint as DNA sequences
// 2. Cellular Emergence - Emerge prokaryotes and eukaryotes from blueprint
// 3. Ecosystem Dynamics - Simulate species interactions and energy flow
// 4. Epigenetic System - Environment-responsive gene expression
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "DNA/RNA are not random evolutionary developments—they unfold from this
// pre-existing holographic blueprint encoded as spectrum configurations"

pub mod cellular_emergence;
pub mod dna_system;
pub mod ecosystem_dynamics;
pub mod epigenetic_system;

// Re-export key types for convenience
pub use cellular_emergence::{
    Cell, CellDivisionResult, CellType, CellularConsciousness, CellularEmergence, Eukaryote,
    Mitochondrion, Nucleus, Organelle, Prokaryote,
};
pub use dna_system::{
    DnaRepairResult, DnaSystem, EpigeneticMarker, EpigeneticMarkerType, Gene, GeneExpression,
    Mutation, MutationType, Protein,
};
pub use ecosystem_dynamics::{
    CoEvolutionResult, Ecosystem, EcosystemDynamics, EnergyFlow, InteractionResult,
    InteractionType, PopulationDynamics, Species, TrophicLevel,
};
pub use epigenetic_system::{
    DevelopmentalPlasticity, EnvironmentalSignal, EpigeneticInheritance, EpigeneticSystem,
    GeneRegulation,
};

/// Biological system configuration
#[derive(Debug, Clone)]
pub struct BiologicalConfig {
    /// Mutation rate (0.0 to 1.0)
    pub mutation_rate: f64,

    /// Epigenetic sensitivity (0.0 to 1.0)
    pub epigenetic_sensitivity: f64,

    /// Cell division rate (0.0 to 1.0)
    pub cell_division_rate: f64,

    /// Species interaction strength (0.0 to 1.0)
    pub interaction_strength: f64,

    /// Energy flow efficiency (0.0 to 1.0)
    pub energy_flow_efficiency: f64,
}

impl Default for BiologicalConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.001,        // 0.1% mutation rate
            epigenetic_sensitivity: 0.5, // Moderate sensitivity
            cell_division_rate: 0.1,     // 10% division rate per step
            interaction_strength: 0.7,   // Strong interactions
            energy_flow_efficiency: 0.8, // Efficient energy flow
        }
    }
}

impl BiologicalConfig {
    /// Create a new biological configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration with custom parameters
    pub fn with_mutation_rate(mut self, rate: f64) -> Self {
        self.mutation_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Create a configuration with custom epigenetic sensitivity
    pub fn with_epigenetic_sensitivity(mut self, sensitivity: f64) -> Self {
        self.epigenetic_sensitivity = sensitivity.clamp(0.0, 1.0);
        self
    }

    /// Create a configuration with custom cell division rate
    pub fn with_cell_division_rate(mut self, rate: f64) -> Self {
        self.cell_division_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Create a configuration with custom interaction strength
    pub fn with_interaction_strength(mut self, strength: f64) -> Self {
        self.interaction_strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Create a configuration with custom energy flow efficiency
    pub fn with_energy_flow_efficiency(mut self, efficiency: f64) -> Self {
        self.energy_flow_efficiency = efficiency.clamp(0.0, 1.0);
        self
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biological_config_default() {
        let config = BiologicalConfig::default();

        assert_eq!(config.mutation_rate, 0.001);
        assert_eq!(config.epigenetic_sensitivity, 0.5);
        assert_eq!(config.cell_division_rate, 0.1);
        assert_eq!(config.interaction_strength, 0.7);
        assert_eq!(config.energy_flow_efficiency, 0.8);
    }

    #[test]
    fn test_biological_config_builder() {
        let config = BiologicalConfig::new()
            .with_mutation_rate(0.01)
            .with_epigenetic_sensitivity(0.8)
            .with_cell_division_rate(0.2)
            .with_interaction_strength(0.9)
            .with_energy_flow_efficiency(0.95);

        assert_eq!(config.mutation_rate, 0.01);
        assert_eq!(config.epigenetic_sensitivity, 0.8);
        assert_eq!(config.cell_division_rate, 0.2);
        assert_eq!(config.interaction_strength, 0.9);
        assert_eq!(config.energy_flow_efficiency, 0.95);
    }

    #[test]
    fn test_biological_config_clamping() {
        // Test that values outside [0.0, 1.0] are clamped
        let config = BiologicalConfig::new()
            .with_mutation_rate(-0.5)
            .with_epigenetic_sensitivity(1.5);

        assert_eq!(config.mutation_rate, 0.0); // Clamped to 0.0
        assert_eq!(config.epigenetic_sensitivity, 1.0); // Clamped to 1.0
    }
}
