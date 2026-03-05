//! Holographic Unfolding (Dynamic Mechanism 4)
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 2:
//! "Implement holographic unfolding respecting space/time spectrum"
//!
//! This module implements holographic unfolding that:
//! - Unfolds holographic blueprint into physical forms
//! - Activates epigenetic factors based on environment
//! - Guides morphogenesis through developmental stages
//! - Respects space/time spectrum constraints

use crate::entity_layer7::dna_encoding::DNAPattern;
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::entity_layer7::layer7::{EntityId, EvolutionaryStage};
use crate::spectrum::SpectrumRatio;
use crate::types::Float;
use std::collections::HashMap;

/// Holographic Unfolding
///
/// Implements holographic unfolding respecting space/time spectrum.
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Success Criteria:
/// "Holographic unfolding produces diverse forms"
pub struct HolographicUnfolding {
    /// Unfolding history
    unfolding_history: Vec<UnfoldingRecord>,

    /// Active unfoldings for each entity
    active_unfoldings: HashMap<EntityId, UnfoldingProcess>,

    /// Simulation time
    simulation_time: u64,
}

/// Unfolding Process
///
/// Represents the ongoing unfolding process for an entity.
#[derive(Debug, Clone)]
pub struct UnfoldingProcess {
    /// Entity ID
    pub entity_id: EntityId,

    /// Current evolutionary stage
    pub current_stage: EvolutionaryStage,

    /// Stage progress (0.0 to 1.0)
    pub stage_progress: Float,

    /// DNA pattern for current stage
    pub dna_pattern: Option<DNAPattern>,

    /// Epigenetic factors
    pub epigenetic_factors: Vec<EpigeneticFactor>,

    /// Developmental instructions
    pub developmental_instructions: Vec<DevelopmentalInstruction>,
}

/// Unfolding Result
///
/// Result of holographic unfolding for an entity.
#[derive(Debug, Clone)]
pub struct UnfoldingResult {
    /// Entity ID
    pub entity_id: EntityId,

    /// Evolutionary stage
    pub evolutionary_stage: EvolutionaryStage,

    /// Stage progress (0.0 to 1.0)
    pub stage_progress: Float,

    /// Physical form characteristics
    pub physical_form: PhysicalForm,

    /// Consciousness level (0.0 to 1.0)
    pub consciousness_level: Float,

    /// Complexity level (0.0 to 1.0)
    pub complexity_level: Float,
}

/// Physical Form
///
/// Characteristics of the unfolded physical form.
#[derive(Debug, Clone)]
pub struct PhysicalForm {
    /// Form type
    pub form_type: FormType,

    /// Scale (10^-35 to 10^26 meters)
    pub scale: Float,

    /// Complexity (number of components)
    pub complexity: usize,

    /// Coherence (0.0 to 1.0)
    pub coherence: Float,

    /// Stability (0.0 to 1.0)
    pub stability: Float,
}

/// Form Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormType {
    /// Quantum particle
    QuantumParticle,

    /// Atom
    Atom,

    /// Molecule
    Molecule,

    /// Cell
    Cell,

    /// Organism
    Organism,

    /// Collective
    Collective,
}

/// Epigenetic Factor
///
/// Environmental factors that influence gene expression.
#[derive(Debug, Clone)]
pub struct EpigeneticFactor {
    /// Factor ID
    pub factor_id: String,

    /// Factor type
    pub factor_type: EpigeneticType,

    /// Influence strength (0.0 to 1.0)
    pub influence_strength: Float,

    /// Target gene or pathway
    pub target: String,

    /// Activation state (0.0 = off, 1.0 = fully on)
    pub activation: Float,
}

/// Epigenetic Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpigeneticType {
    /// DNA methylation
    Methylation,

    /// Histone modification
    HistoneModification,

    /// Non-coding RNA
    NonCodingRNA,

    /// Environmental signal
    EnvironmentalSignal,
}

/// Developmental Instruction
///
/// Instruction guiding morphogenesis and development.
#[derive(Debug, Clone)]
pub struct DevelopmentalInstruction {
    /// Instruction ID
    pub instruction_id: String,

    /// Instruction type
    pub instruction_type: InstructionType,

    /// Priority (0.0 to 1.0)
    pub priority: Float,

    /// Target structure
    pub target: String,

    /// Action to take
    pub action: String,

    /// Timing (relative to stage start)
    pub timing: Float,
}

/// Instruction Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    /// Cell division
    CellDivision,

    /// Differentiation
    Differentiation,

    /// Migration
    Migration,

    /// Pattern formation
    PatternFormation,

    /// Growth
    Growth,

    /// Maturation
    Maturation,
}

/// Unfolding Record
///
/// Records unfolding events for analysis.
#[derive(Debug, Clone)]
pub struct UnfoldingRecord {
    /// Timestamp
    pub timestamp: u64,

    /// Entity ID
    pub entity_id: EntityId,

    /// Evolutionary stage
    pub evolutionary_stage: EvolutionaryStage,

    /// Stage progress
    pub stage_progress: Float,

    /// Form type
    pub form_type: FormType,

    /// Success metric (0.0 to 1.0)
    pub success_metric: Float,
}

/// Holographic Unfolding State
///
/// Current state of the holographic unfolding system.
#[derive(Debug, Clone)]
pub struct HolographicUnfoldingState {
    /// Total unfoldings completed
    pub total_unfoldings: usize,

    /// Forms by type
    pub forms_by_type: HashMap<String, usize>,

    /// Average stage progress
    pub avg_stage_progress: Float,

    /// Success rate (unfoldings that produced stable forms)
    pub success_rate: Float,
}

impl Default for HolographicUnfoldingState {
    fn default() -> Self {
        Self {
            total_unfoldings: 0,
            forms_by_type: HashMap::new(),
            avg_stage_progress: 0.0,
            success_rate: 0.0,
        }
    }
}

impl HolographicUnfolding {
    /// Create a new holographic unfolding system
    pub fn new() -> Self {
        Self {
            unfolding_history: Vec::new(),
            active_unfoldings: HashMap::new(),
            simulation_time: 0,
        }
    }

    /// Initialize unfolding for an entity
    ///
    /// Creates an unfolding process based on the entity's holographic blueprint.
    pub fn initialize_unfolding(
        &mut self,
        entity_id: EntityId,
        blueprint: &HolographicBlueprint,
        initial_stage: EvolutionaryStage,
    ) {
        let stage_blueprint = blueprint.get_stage_blueprint(initial_stage);

        let process = UnfoldingProcess {
            entity_id: entity_id.clone(),
            current_stage: initial_stage,
            stage_progress: 0.0,
            dna_pattern: stage_blueprint.and_then(|sb| sb.dna_pattern),
            epigenetic_factors: Vec::new(),
            developmental_instructions: Vec::new(),
        };

        self.active_unfoldings.insert(entity_id, process);
    }

    /// Process unfolding for all active entities
    ///
    /// Advances unfolding based on spectrum ratio and environmental factors.
    pub fn process_unfoldings(
        &mut self,
        spectrum_ratio: &SpectrumRatio,
        environmental_factors: &EnvironmentalFactors,
    ) -> Vec<UnfoldingResult> {
        let mut results = Vec::new();

        // Collect entity IDs first to avoid borrow issues
        let entity_ids: Vec<_> = self.active_unfoldings.keys().cloned().collect();

        for entity_id in entity_ids {
            // Remove and re-insert to avoid borrow issues
            if let Some(mut process) = self.active_unfoldings.remove(&entity_id) {
                // Advance unfolding
                if let Some(result) =
                    self.advance_unfolding(&mut process, spectrum_ratio, environmental_factors)
                {
                    results.push(result);
                }

                // Re-insert process if still active
                if self.active_unfoldings.contains_key(&entity_id) {
                    self.active_unfoldings.insert(entity_id.clone(), process);
                }
            }
        }

        self.simulation_time += 1;

        results
    }

    /// Advance unfolding for a single entity
    fn advance_unfolding(
        &mut self,
        process: &mut UnfoldingProcess,
        spectrum_ratio: &SpectrumRatio,
        env_factors: &EnvironmentalFactors,
    ) -> Option<UnfoldingResult> {
        // Calculate unfolding rate based on spectrum ratio
        let ratio = spectrum_ratio.calculate_ratio();
        let unfolding_rate = self.calculate_unfolding_rate(ratio, env_factors);

        // Advance stage progress
        process.stage_progress = (process.stage_progress + unfolding_rate).min(1.0);

        // Generate physical form based on stage and progress
        let physical_form = self.generate_physical_form(
            &process.current_stage,
            process.stage_progress,
            env_factors,
        );

        // Calculate consciousness level
        let consciousness_level =
            self.calculate_consciousness_level(&process.current_stage, process.stage_progress);

        // Calculate complexity level
        let complexity_level =
            self.calculate_complexity_level(&process.current_stage, process.stage_progress);

        // Check if stage is complete
        if process.stage_progress >= 1.0 {
            // Record unfolding
            self.record_unfolding(process, &physical_form);

            // Move to next stage
            let next_stage = self.get_next_stage(&process.current_stage);
            if let Some(next_stage) = next_stage {
                process.current_stage = next_stage;
                process.stage_progress = 0.0;
            } else {
                // No more stages - remove from active unfoldings
                let entity_id = process.entity_id.clone();
                self.active_unfoldings.remove(&entity_id);
            }
        }

        Some(UnfoldingResult {
            entity_id: process.entity_id.clone(),
            evolutionary_stage: process.current_stage,
            stage_progress: process.stage_progress,
            physical_form,
            consciousness_level,
            complexity_level,
        })
    }

    /// Calculate unfolding rate based on spectrum ratio and environment
    fn calculate_unfolding_rate(&self, ratio: Float, env_factors: &EnvironmentalFactors) -> Float {
        // Base rate from spectrum ratio
        let base_rate = (ratio / 2.0).min(1.0);

        // Environmental factors
        let energy_factor = env_factors.available_energy * 0.3;
        let coherence_factor = env_factors.coherence_level * 0.2;

        // Entropy slows unfolding
        let entropy_penalty = env_factors.entropy_level * 0.2;

        (base_rate + energy_factor + coherence_factor - entropy_penalty)
            .clamp(0.0, 1.0)
            * 0.1
    }

    /// Generate physical form based on stage and progress
    fn generate_physical_form(
        &self,
        stage: &EvolutionaryStage,
        progress: Float,
        env_factors: &EnvironmentalFactors,
    ) -> PhysicalForm {
        let form_type = self.determine_form_type(stage);
        let scale = self.calculate_scale(stage, progress);
        let complexity = self.calculate_complexity(stage, progress);
        let coherence = env_factors.coherence_level * progress;
        let stability = (1.0 - env_factors.entropy_level) * progress;

        PhysicalForm {
            form_type,
            scale,
            complexity,
            coherence,
            stability,
        }
    }

    /// Determine form type based on evolutionary stage
    fn determine_form_type(&self, stage: &EvolutionaryStage) -> FormType {
        match stage {
            EvolutionaryStage::QuantumRealm => FormType::QuantumParticle,
            EvolutionaryStage::AtomicRealm => FormType::Atom,
            EvolutionaryStage::MolecularRealm => FormType::Molecule,
            EvolutionaryStage::CellularRealm => FormType::Cell,
            EvolutionaryStage::SimpleLifeRealm => FormType::Organism,
            EvolutionaryStage::ComplexLifeRealm => FormType::Organism,
            EvolutionaryStage::ConsciousLifeRealm => FormType::Organism,
            EvolutionaryStage::SocietalRealm => FormType::Collective,
            EvolutionaryStage::FourthDensityRealm => FormType::Collective,
            EvolutionaryStage::FifthDensityRealm => FormType::Collective,
            EvolutionaryStage::SixthDensityRealm => FormType::Collective,
            EvolutionaryStage::SeventhDensityRealm => FormType::Collective,
            EvolutionaryStage::PlanetaryRealm => FormType::Collective,
        }
    }

    /// Calculate scale based on stage and progress
    fn calculate_scale(&self, stage: &EvolutionaryStage, progress: Float) -> Float {
        let base_scale = match stage {
            EvolutionaryStage::QuantumRealm => -35.0,   // 10^-35 m
            EvolutionaryStage::AtomicRealm => -10.0,    // 10^-10 m
            EvolutionaryStage::MolecularRealm => -9.0,  // 10^-9 m
            EvolutionaryStage::CellularRealm => -6.0,   // 10^-6 m
            EvolutionaryStage::SimpleLifeRealm => -3.0, // 10^-3 m
            EvolutionaryStage::ComplexLifeRealm => -3.0,
            EvolutionaryStage::ConsciousLifeRealm => -3.0,
            EvolutionaryStage::SocietalRealm => 0.0, // 10^0 m
            EvolutionaryStage::FourthDensityRealm => 6.0, // 10^6 m
            EvolutionaryStage::FifthDensityRealm => 9.0, // 10^9 m
            EvolutionaryStage::SixthDensityRealm => 21.0, // 10^21 m
            EvolutionaryStage::SeventhDensityRealm => 26.0, // 10^26 m
            EvolutionaryStage::PlanetaryRealm => 6.0,
        };

        base_scale + progress * 1.0
    }

    /// Calculate complexity based on stage and progress
    fn calculate_complexity(&self, stage: &EvolutionaryStage, progress: Float) -> usize {
        let base_complexity = match stage {
            EvolutionaryStage::QuantumRealm => 1,
            EvolutionaryStage::AtomicRealm => 10,
            EvolutionaryStage::MolecularRealm => 100,
            EvolutionaryStage::CellularRealm => 1000,
            EvolutionaryStage::SimpleLifeRealm => 10000,
            EvolutionaryStage::ComplexLifeRealm => 100000,
            EvolutionaryStage::ConsciousLifeRealm => 1000000,
            EvolutionaryStage::SocietalRealm => 10000000,
            EvolutionaryStage::FourthDensityRealm => 100000000,
            EvolutionaryStage::FifthDensityRealm => 1000000000,
            EvolutionaryStage::SixthDensityRealm => 10000000000_usize,
            EvolutionaryStage::SeventhDensityRealm => 100000000000_usize,
            EvolutionaryStage::PlanetaryRealm => 100000,
        };

        (base_complexity as Float * progress) as usize
    }

    /// Calculate consciousness level based on stage and progress
    fn calculate_consciousness_level(&self, stage: &EvolutionaryStage, progress: Float) -> Float {
        let base_consciousness = match stage {
            EvolutionaryStage::QuantumRealm => 0.0,
            EvolutionaryStage::AtomicRealm => 0.0,
            EvolutionaryStage::MolecularRealm => 0.0,
            EvolutionaryStage::CellularRealm => 0.1,
            EvolutionaryStage::SimpleLifeRealm => 0.2,
            EvolutionaryStage::ComplexLifeRealm => 0.3,
            EvolutionaryStage::ConsciousLifeRealm => 0.5,
            EvolutionaryStage::SocietalRealm => 0.7,
            EvolutionaryStage::FourthDensityRealm => 0.8,
            EvolutionaryStage::FifthDensityRealm => 0.9,
            EvolutionaryStage::SixthDensityRealm => 0.95,
            EvolutionaryStage::SeventhDensityRealm => 1.0,
            EvolutionaryStage::PlanetaryRealm => 0.0,
        };

        base_consciousness + progress * 0.1
    }

    /// Calculate complexity level based on stage and progress
    fn calculate_complexity_level(&self, stage: &EvolutionaryStage, progress: Float) -> Float {
        let base_complexity = match stage {
            EvolutionaryStage::QuantumRealm => 0.0,
            EvolutionaryStage::AtomicRealm => 0.1,
            EvolutionaryStage::MolecularRealm => 0.2,
            EvolutionaryStage::CellularRealm => 0.3,
            EvolutionaryStage::SimpleLifeRealm => 0.4,
            EvolutionaryStage::ComplexLifeRealm => 0.5,
            EvolutionaryStage::ConsciousLifeRealm => 0.6,
            EvolutionaryStage::SocietalRealm => 0.7,
            EvolutionaryStage::FourthDensityRealm => 0.8,
            EvolutionaryStage::FifthDensityRealm => 0.9,
            EvolutionaryStage::SixthDensityRealm => 0.95,
            EvolutionaryStage::SeventhDensityRealm => 1.0,
            EvolutionaryStage::PlanetaryRealm => 0.0,
        };

        base_complexity * progress
    }

    /// Get next evolutionary stage
    fn get_next_stage(&self, current_stage: &EvolutionaryStage) -> Option<EvolutionaryStage> {
        match current_stage {
            EvolutionaryStage::QuantumRealm => Some(EvolutionaryStage::AtomicRealm),
            EvolutionaryStage::AtomicRealm => Some(EvolutionaryStage::MolecularRealm),
            EvolutionaryStage::MolecularRealm => Some(EvolutionaryStage::CellularRealm),
            EvolutionaryStage::CellularRealm => Some(EvolutionaryStage::SimpleLifeRealm),
            EvolutionaryStage::SimpleLifeRealm => Some(EvolutionaryStage::ComplexLifeRealm),
            EvolutionaryStage::ComplexLifeRealm => Some(EvolutionaryStage::ConsciousLifeRealm),
            EvolutionaryStage::ConsciousLifeRealm => Some(EvolutionaryStage::SocietalRealm),
            EvolutionaryStage::SocietalRealm => Some(EvolutionaryStage::FourthDensityRealm),
            EvolutionaryStage::FourthDensityRealm => Some(EvolutionaryStage::FifthDensityRealm),
            EvolutionaryStage::FifthDensityRealm => Some(EvolutionaryStage::SixthDensityRealm),
            EvolutionaryStage::SixthDensityRealm => Some(EvolutionaryStage::SeventhDensityRealm),
            EvolutionaryStage::SeventhDensityRealm => None, // Final stage
            EvolutionaryStage::PlanetaryRealm => Some(EvolutionaryStage::CellularRealm),
        }
    }

    /// Record unfolding event
    fn record_unfolding(&mut self, process: &UnfoldingProcess, physical_form: &PhysicalForm) {
        let record = UnfoldingRecord {
            timestamp: self.simulation_time,
            entity_id: process.entity_id.clone(),
            evolutionary_stage: process.current_stage,
            stage_progress: process.stage_progress,
            form_type: physical_form.form_type,
            success_metric: physical_form.stability,
        };

        self.unfolding_history.push(record);
    }

    /// Get unfolding process for an entity
    pub fn get_unfolding_process(&self, entity_id: &EntityId) -> Option<&UnfoldingProcess> {
        self.active_unfoldings.get(entity_id)
    }

    /// Get unfolding state
    pub fn get_state(&self) -> HolographicUnfoldingState {
        let total_unfoldings = self.unfolding_history.len();

        let mut forms_by_type = HashMap::new();
        for record in &self.unfolding_history {
            let type_str = format!("{:?}", record.form_type);
            *forms_by_type.entry(type_str).or_insert(0) += 1;
        }

        let avg_stage_progress = if !self.active_unfoldings.is_empty() {
            let total: Float = self
                .active_unfoldings
                .values()
                .map(|p| p.stage_progress)
                .sum();
            total / self.active_unfoldings.len() as Float
        } else {
            0.0
        };

        let success_rate = if !self.unfolding_history.is_empty() {
            let total: Float = self
                .unfolding_history
                .iter()
                .map(|r| r.success_metric)
                .sum();
            total / self.unfolding_history.len() as Float
        } else {
            0.0
        };

        HolographicUnfoldingState {
            total_unfoldings,
            forms_by_type,
            avg_stage_progress,
            success_rate,
        }
    }
}

/// Environmental Factors
///
/// Global environmental factors that influence unfolding.
#[derive(Debug, Clone)]
pub struct EnvironmentalFactors {
    /// Available energy (0.0 to 1.0)
    pub available_energy: Float,

    /// Coherence level (0.0 to 1.0)
    pub coherence_level: Float,

    /// Entropy level (0.0 to 1.0)
    pub entropy_level: Float,

    /// Temperature (relative scale)
    pub temperature: Float,

    /// Pressure (relative scale)
    pub pressure: Float,
}

impl Default for EnvironmentalFactors {
    fn default() -> Self {
        Self {
            available_energy: 0.7,
            coherence_level: 0.8,
            entropy_level: 0.2,
            temperature: 0.5,
            pressure: 0.5,
        }
    }
}

impl Default for HolographicUnfolding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::IndividualSpectrumConfiguration;

    fn create_test_blueprint() -> HolographicBlueprint {
        // Create a simple test blueprint
        let spectrum_config = IndividualSpectrumConfiguration::new(SpectrumRatio::new(
            1.5,
            crate::spectrum::SpectrumSide::SpaceTime,
        ));

        // Create a simple archetype blueprint
        let archetype_blueprint =
            crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint::default();

        HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetype_blueprint)
    }

    #[test]
    fn test_holographic_unfolding_creation() {
        let unfolding = HolographicUnfolding::new();
        assert_eq!(unfolding.simulation_time, 0);
        assert!(unfolding.unfolding_history.is_empty());
    }

    #[test]
    fn test_initialize_unfolding() {
        let mut unfolding = HolographicUnfolding::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let blueprint = create_test_blueprint();

        unfolding.initialize_unfolding(
            entity_id.clone(),
            &blueprint,
            EvolutionaryStage::QuantumRealm,
        );

        assert!(unfolding.active_unfoldings.contains_key(&entity_id));
    }

    #[test]
    fn test_process_unfoldings() {
        let mut unfolding = HolographicUnfolding::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let blueprint = create_test_blueprint();

        unfolding.initialize_unfolding(
            entity_id.clone(),
            &blueprint,
            EvolutionaryStage::QuantumRealm,
        );

        let spectrum_ratio = SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let env_factors = EnvironmentalFactors::default();

        let results = unfolding.process_unfoldings(&spectrum_ratio, &env_factors);

        assert!(!results.is_empty());
    }

    #[test]
    fn test_physical_form_generation() {
        let unfolding = HolographicUnfolding::new();
        let env_factors = EnvironmentalFactors::default();

        let form =
            unfolding.generate_physical_form(&EvolutionaryStage::CellularRealm, 0.5, &env_factors);

        assert_eq!(form.form_type, FormType::Cell);
        assert!(form.scale > -10.0);
        assert!(form.scale < 0.0);
    }

    #[test]
    fn test_get_state() {
        let mut unfolding = HolographicUnfolding::new();
        let entity_id = EntityId::new("test-entity".to_string());
        let blueprint = create_test_blueprint();

        unfolding.initialize_unfolding(
            entity_id.clone(),
            &blueprint,
            EvolutionaryStage::QuantumRealm,
        );

        let state = unfolding.get_state();
        assert_eq!(state.total_unfoldings, 0); // No unfoldings completed yet
    }
}
