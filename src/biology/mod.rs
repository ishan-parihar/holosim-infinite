// Biological Emergence - Phase 3 through Phase 5
//
// From SIMULATION-AUDIT-AND-REFACTOR_PLAN.md Phase 3:
// "Implement true biological emergence from holographic blueprint"
//
// This module implements:
// 1. DNA/RNA System - Encode holographic blueprint as DNA sequences
// 2. Cellular Emergence - Emerge prokaryotes and eukaryotes from blueprint
// 3. Ecosystem Dynamics - Simulate species interactions and energy flow
// 4. Epigenetic System - Environment-responsive gene expression
//
// Phase 4:
// - Cell Engine - Dynamic cellular simulation
// - Organism Lifecycle - Birth, growth, death cycle
// - Evolution Engine - Natural selection and adaptation
// - Entity-Body Coupling - Consciousness embodiment
//
// Phase 5 (V5):
// - Molecular Field - Atoms form molecules through bonding
// - Neural Field - Consciousness from neural activity
// - Archetype Processor - Catalyst-driven behavior
// - Consciousness Tick - Integration of all consciousness components
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "DNA/RNA are not random evolutionary developments—they unfold from this
// pre-existing holographic blueprint encoded as spectrum configurations"

// Phase 3: Foundation Biology
pub mod cellular_emergence;
pub mod dna_system;
pub mod ecosystem_dynamics;
pub mod epigenetic_system;

// Phase 4: Living Biology
pub mod cell_engine;
pub mod entity_body_coupling;
pub mod evolution_engine;
pub mod organism_lifecycle;

// Phase 5: Consciousness Biology (V5 Implementation)
pub mod archetype_processor;
pub mod consciousness_tick;
pub mod dual_experience_engine;
pub mod molecular_field;
pub mod neural_field;
pub mod neural_receiver;
pub mod veil_integration;

// Phase 3.4: Protein Folding as Blueprint Unfolding
pub mod protein_folding;

// Phase 4.1: DNA as Blueprint Manifestation
pub mod dna_blueprint;

// Phase 4.3: Molecular to Cellular Bridge
// The CRITICAL bridge connecting molecular_field to cellular_emergence
pub mod molecular_cellular_bridge;

// Phase 5.3: Unified Biology Pipeline
pub mod pipeline;

// ============================================================================
// RE-EXPORTS
// ============================================================================

// Phase 5.3: Pipeline exports
pub use pipeline::{BiologyPipeline, BiologyStats, BiologyTickResult};

// Phase 3 exports
pub use cellular_emergence::{
    Cell, CellDivisionResult, CellType, CellularConsciousness, CellularEmergence, Eukaryote,
    Mitochondrion, Nucleus, Organelle, Prokaryote,
};
pub use dna_system::{
    DnaRepairResult, DnaSystem, EpigeneticMarkerType, Gene, GeneExpression, Mutation, MutationType,
    Protein,
};
pub use ecosystem_dynamics::{
    CoEvolutionResult, Ecosystem, EcosystemDynamics, EnergyFlow, InteractionResult,
    InteractionType, PopulationDynamics, Species, TrophicLevel,
};
pub use epigenetic_system::{
    DevelopmentalPlasticity, EnvironmentalSignal, EpigeneticInheritance, EpigeneticSystem,
    GeneRegulation,
};

// Phase 4 exports
pub use cell_engine::{
    CellCategory, CellEngine, CellEnvironment, CellEpigeneticMarker, CellId, CellState, LiveCell,
};

pub use organism_lifecycle::{
    BehaviorState, BodyPlan, BodySymmetry, DietType, LocomotionType, Organism, OrganismEnvironment,
    OrganismManager, SocialStructure,
};

pub use evolution_engine::{
    EvolutionEngine, ExtinctionCause, ExtinctionEvent, Population, PopulationEnvironment,
    SpeciationCause, SpeciationEvent, Species as EvoSpecies, SpeciesId,
    TrophicLevel as EvoTrophicLevel,
};

pub use entity_body_coupling::{
    BioSimulation, ConsciousnessExperience, EmbodiedEntity, EnergyCenter, NervousSystem,
    PlanetConditions,
};

// Phase 5: Molecular Field exports
pub use molecular_field::{
    Bond, BondType, MolecularField, MolecularFieldStats, MolecularFormula, Molecule,
};

// Phase 5: Neural Field exports
pub use neural_field::{
    BrainwaveBand, InputPattern, InputSource, NeuralAttractor, NeuralField, NeuralFieldState,
    NeuralFieldStats, Neuron, NeuronId, NeuronType, Neurotransmitter, ReceptiveField, SensoryInput,
    SensoryModality, Synapse,
};

// Phase 5.2: Neural Receiver exports (consciousness receiver model)
pub use neural_receiver::{NeuralArchitecture, NeuralReceiver, ReceptionResult};

// Phase 5: Consciousness Engine exports
pub use archetype_processor::{
    ArchetypeActivationState, BehaviorOutput, BehaviorType, CatalystEvent, CatalystPolarity,
    CatalystSource, CatalystType, EntityArchetypeProcessor, GrowthDirection,
};

pub use dual_experience_engine::{
    CausalityExperience, DualExperienceEngine, ExperienceModality,
    GrowthPolarity as ExpGrowthPolarity, QualitativeExperience, SpaceExperience, TimeExperience,
};

pub use veil_integration::{
    ConsciousPercept, FilteredPerception, GrowthPolarity as VeilGrowthPolarity, RawPerceptItem,
    RawPerception, SubconsciousContent, VeilIntegration,
};

pub use consciousness_tick::{
    BodyExperience, ConsciousnessState, ConsciousnessStateChanges, ConsciousnessTickEngine,
    ConsciousnessTickInput, ConsciousnessTickOutput, EnvironmentExperience, PolarityDirection,
    SocialExperience,
};

// Phase 3.4: Protein Folding exports
pub use protein_folding::{
    AminoAcid, AssemblyType, BlueprintFoldingStep, BlueprintProteinShape, FoldingError,
    FoldingIntermediate, FoldingResult, FoldingState, Protein as BlueprintProtein,
    ProteinFoldingEngine, ProteinSubunit, QuaternaryStructure, SecondaryStructure,
    SecondaryStructureContent, SecondaryStructureRegion, SecondaryStructureType,
};

// Phase 4.1: DNA Blueprint exports
pub use dna_blueprint::{
    AminoAcidCode, AssemblyInstruction, AssemblyResult, BlueprintDNA, BlueprintStageId,
    CellularFunction, Codon, EpigeneticMarker, EpigeneticType, FoldingPattern, FunctionalGene,
    GeneId, LipidComposition, Nucleotide, ProteinBlueprint, ProteinType, RegulatoryRegion,
    RegulatoryType, SecondaryStructurePrediction, RNA,
};

// Phase 4.3: Molecular to Cellular Bridge exports
pub use molecular_cellular_bridge::{
    AssembledProtein, BridgeConfig, BridgeError, BridgeStats, Cell as BridgeCell,
    CellType as BridgeCellType, Domain, LipidBilayer, LipidType, Membrane, Metabolism,
    MolecularCellularBridge, NucleotideComponent, Organelle as BridgeOrganelle, ProteinFunction,
    ProteinStructure, SecondaryStructureType as BridgeSecondaryStructureType, TertiaryFold,
};

// ============================================================================
// BIOLOGICAL CONFIGURATION
// ============================================================================

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

    /// Molecular bonding threshold (0.0 to 1.0)
    pub molecular_bond_threshold: f64,

    /// Neural firing threshold multiplier (0.0 to 2.0)
    pub neural_threshold_multiplier: f64,
}

impl Default for BiologicalConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.001,             // 0.1% mutation rate
            epigenetic_sensitivity: 0.5,      // Moderate sensitivity
            cell_division_rate: 0.1,          // 10% division rate per step
            interaction_strength: 0.7,        // Strong interactions
            energy_flow_efficiency: 0.8,      // Efficient energy flow
            molecular_bond_threshold: 0.3,    // Bond formation threshold
            neural_threshold_multiplier: 1.0, // Normal thresholds
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

    /// Create a configuration with custom molecular bond threshold
    pub fn with_molecular_bond_threshold(mut self, threshold: f64) -> Self {
        self.molecular_bond_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Create a configuration with custom neural threshold multiplier
    pub fn with_neural_threshold_multiplier(mut self, multiplier: f64) -> Self {
        self.neural_threshold_multiplier = multiplier.clamp(0.1, 2.0);
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
        assert_eq!(config.molecular_bond_threshold, 0.3);
        assert_eq!(config.neural_threshold_multiplier, 1.0);
    }

    #[test]
    fn test_biological_config_builder() {
        let config = BiologicalConfig::new()
            .with_mutation_rate(0.01)
            .with_epigenetic_sensitivity(0.8)
            .with_cell_division_rate(0.2)
            .with_interaction_strength(0.9)
            .with_energy_flow_efficiency(0.95)
            .with_molecular_bond_threshold(0.5)
            .with_neural_threshold_multiplier(1.2);

        assert_eq!(config.mutation_rate, 0.01);
        assert_eq!(config.epigenetic_sensitivity, 0.8);
        assert_eq!(config.cell_division_rate, 0.2);
        assert_eq!(config.interaction_strength, 0.9);
        assert_eq!(config.energy_flow_efficiency, 0.95);
        assert_eq!(config.molecular_bond_threshold, 0.5);
        assert_eq!(config.neural_threshold_multiplier, 1.2);
    }

    #[test]
    fn test_biological_config_clamping() {
        // Test that values outside [0.0, 1.0] are clamped
        let config = BiologicalConfig::new()
            .with_mutation_rate(-0.5)
            .with_epigenetic_sensitivity(1.5)
            .with_neural_threshold_multiplier(3.0);

        assert_eq!(config.mutation_rate, 0.0); // Clamped to 0.0
        assert_eq!(config.epigenetic_sensitivity, 1.0); // Clamped to 1.0
        assert_eq!(config.neural_threshold_multiplier, 2.0); // Clamped to max
    }

    #[test]
    fn test_molecular_formula_water() {
        let h2o = MolecularFormula::water();
        assert_eq!(h2o.total_atoms(), 3);
    }

    #[test]
    fn test_neural_field_new() {
        let field = NeuralField::new();
        assert_eq!(field.neuron_count(), 0);
        assert_eq!(field.synapse_count(), 0);
    }
}
