//! Active Intelligence Engine
//!
//! From ROADMAP Phase 8.1:
//! "Implement active, goal-directed intelligence"
//! "II learns, optimizes, creates - Not passive field emission"
//!
//! ## Overview
//!
//! The Active Intelligence Engine transforms Intelligent-Infinity from a passive
//! field emitter to an active, learning intelligence that:
//!
//! 1. **Learns from Outcomes**: Records and analyzes condensation outcomes
//! 2. **Optimizes Emergence**: Finds optimal patterns for target manifestations
//! 3. **Creates New Attractors**: Generates new attractor fields as needed
//!
//! ## Key Concepts
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Intelligent-Infinity is not passive - it actively directs emergence toward purpose"
//!
//! The engine implements:
//! - Feedback learning from condensation outcomes
//! - Pattern optimization for target manifestations
//! - Dynamic attractor field creation
//! - Goal-directed emergence guidance

use crate::entity_layer7::layer7::EntityId;
use crate::evolution::adaptive_attractor::EntityFeedback;
use crate::template::transcend_include::{AttractorField, Orientation, TargetDensity};
use crate::types::{Float, Polarity};
use std::collections::HashMap;

/// Golden ratio for scaling calculations
pub const PHI: Float = 1.618033988749895;

/// Result of a condensation event (matter manifestation)
#[derive(Debug, Clone)]
pub struct CondensationOutcome {
    /// Unique outcome ID
    pub outcome_id: u64,

    /// Entity that triggered condensation
    pub entity_id: Option<EntityId>,

    /// Type of condensation (particle, atom, molecule, cell, organism)
    pub condensation_type: CondensationType,

    /// Success level (0.0 to 1.0)
    pub success: Float,

    /// Coherence achieved
    pub coherence: Float,

    /// Energy efficiency (how much II energy was used effectively)
    pub energy_efficiency: Float,

    /// Pattern that was used
    pub pattern_used: String,

    /// Time taken for condensation
    pub condensation_time: u64,

    /// Stability of the resulting manifestation
    pub stability: Float,

    /// Catalyst that triggered this condensation
    pub catalyst: Option<CatalystData>,

    /// Lessons learned
    pub lessons: Vec<String>,
}

/// Types of condensation events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondensationType {
    /// Quantum particle condensation
    QuantumParticle,

    /// Atomic condensation
    Atom,

    /// Molecular condensation
    Molecule,

    /// Cellular condensation
    Cell,

    /// Organism condensation
    Organism,

    /// Collective condensation (SMC formation)
    Collective,

    /// Environmental condensation
    Environment,

    /// Archetype condensation
    Archetype,
}

/// Catalyst data that triggered condensation
#[derive(Debug, Clone)]
pub struct CatalystData {
    /// Catalyst type
    pub catalyst_type: String,

    /// Intensity of the catalyst
    pub intensity: Float,

    /// Duration of catalyst exposure
    pub duration: u64,
}

/// Target for emergence optimization
#[derive(Debug, Clone)]
pub struct EmergenceTarget {
    /// Target type
    pub target_type: EmergenceTargetType,

    /// Desired coherence level
    pub target_coherence: Float,

    /// Desired stability
    pub target_stability: Float,

    /// Time constraint (maximum ticks)
    pub time_constraint: Option<u64>,

    /// Energy budget
    pub energy_budget: Float,

    /// Polarity alignment (if applicable)
    pub polarity_alignment: Option<Polarity>,

    /// Additional constraints
    pub constraints: Vec<EmergenceConstraint>,
}

/// Types of emergence targets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergenceTargetType {
    /// Create stable particle
    StableParticle,

    /// Create coherent atom
    CoherentAtom,

    /// Create functional molecule
    FunctionalMolecule,

    /// Create living cell
    LivingCell,

    /// Create conscious entity
    ConsciousEntity,

    /// Create collective structure
    CollectiveStructure,

    /// Create archetype pattern
    ArchetypePattern,
}

/// Constraints on emergence
#[derive(Debug, Clone)]
pub enum EmergenceConstraint {
    /// Maximum complexity allowed
    MaxComplexity(Float),

    /// Minimum resonance required
    MinResonance(Float),

    /// Specific archetype required
    RequiredArchetype(usize),

    /// Density level constraint
    DensityLevel(u8),

    /// Polarity requirement
    PolarityRequirement(Polarity),
}

/// Optimal pattern for emergence
#[derive(Debug, Clone)]
pub struct OptimalPattern {
    /// Pattern identifier
    pub pattern_id: String,

    /// Pattern type
    pub pattern_type: PatternType,

    /// Expected coherence
    pub expected_coherence: Float,

    /// Expected stability
    pub expected_stability: Float,

    /// Required energy
    pub required_energy: Float,

    /// Estimated time
    pub estimated_time: u64,

    /// Success probability
    pub success_probability: Float,

    /// Archetype coefficients
    pub archetype_coefficients: [Float; 22],

    /// Field configuration
    pub field_configuration: FieldConfiguration,

    /// Optimization score
    pub optimization_score: Float,
}

/// Types of patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Standing wave pattern
    StandingWave,

    /// Interference pattern
    Interference,

    /// Spiral pattern
    Spiral,

    /// Platonic solid pattern
    PlatonicSolid,

    /// Vesica Piscis pattern
    VesicaPiscis,

    /// Flower of Life pattern
    FlowerOfLife,

    /// Fibonacci pattern
    Fibonacci,

    /// Custom pattern
    Custom,
}

/// Field configuration for pattern
#[derive(Debug, Clone)]
pub struct FieldConfiguration {
    /// Base frequency
    pub base_frequency: Float,

    /// Amplitude
    pub amplitude: Float,

    /// Phase
    pub phase: Float,

    /// Harmonic content
    pub harmonics: Vec<Float>,

    /// Spatial extent
    pub spatial_extent: Float,
}

/// Specification for creating a new attractor
#[derive(Debug, Clone)]
pub struct AttractorSpec {
    /// Name for the attractor
    pub name: String,

    /// Target density
    pub target_density: TargetDensity,

    /// Orientation (STO/STS/Balanced)
    pub orientation: Orientation,

    /// Strength
    pub strength: Float,

    /// Archetype signature
    pub archetype_signature: [Float; 22],

    /// Stability requirement
    pub stability_requirement: Float,

    /// Whether to learn from entity feedback
    pub adaptive: bool,
}

/// Attractor optimizer for finding optimal patterns
#[derive(Debug, Clone)]
pub struct AttractorOptimizer {
    /// Optimization history
    pub optimization_history: Vec<OptimizationRecord>,

    /// Learning rate
    pub learning_rate: Float,

    /// Number of iterations to run
    pub max_iterations: usize,

    /// Convergence threshold
    pub convergence_threshold: Float,

    /// Best patterns found
    pub best_patterns: HashMap<String, OptimalPattern>,
}

/// Record of an optimization run
#[derive(Debug, Clone)]
pub struct OptimizationRecord {
    /// Target type
    pub target_type: EmergenceTargetType,

    /// Initial score
    pub initial_score: Float,

    /// Final score
    pub final_score: Float,

    /// Iterations used
    pub iterations: usize,

    /// Whether converged
    pub converged: bool,

    /// Best pattern found
    pub best_pattern: Option<OptimalPattern>,
}

/// Active Intelligence Engine
///
/// The main engine that transforms Intelligent-Infinity into an active,
/// learning, goal-directed intelligence.
///
/// From ROADMAP Phase 8.1:
/// ```text
/// ActiveIntelligenceEngine {
///     feedback_collector: Vec<EntityFeedback>,
///     condensation_history: Vec<CondensationOutcome>,
///     attractor_optimizer: AttractorOptimizer,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ActiveIntelligenceEngine {
    /// Collected entity feedback
    pub feedback_collector: Vec<EntityFeedback>,

    /// History of condensation outcomes
    pub condensation_history: Vec<CondensationOutcome>,

    /// Attractor optimizer
    pub attractor_optimizer: AttractorOptimizer,

    /// Active attractor fields
    pub active_attractors: HashMap<String, AttractorField>,

    /// Learning parameters
    pub learning_rate: Float,

    /// Adaptation rate
    pub adaptation_rate: Float,

    /// Intelligence level (how "smart" the engine has become)
    pub intelligence_level: Float,

    /// Number of successful optimizations
    pub successful_optimizations: usize,

    /// Number of failed optimizations
    pub failed_optimizations: usize,

    /// Next outcome ID
    next_outcome_id: u64,

    /// Pattern library (learned patterns)
    pub pattern_library: HashMap<String, OptimalPattern>,
}

impl ActiveIntelligenceEngine {
    /// Create a new active intelligence engine
    pub fn new() -> Self {
        Self {
            feedback_collector: Vec::new(),
            condensation_history: Vec::new(),
            attractor_optimizer: AttractorOptimizer::new(),
            active_attractors: HashMap::new(),
            learning_rate: 0.1,
            adaptation_rate: 0.05,
            intelligence_level: 0.5,
            successful_optimizations: 0,
            failed_optimizations: 0,
            next_outcome_id: 1,
            pattern_library: HashMap::new(),
        }
    }

    /// Create with custom learning parameters
    pub fn with_params(learning_rate: Float, adaptation_rate: Float) -> Self {
        let mut engine = Self::new();
        engine.learning_rate = learning_rate;
        engine.adaptation_rate = adaptation_rate;
        engine
    }

    /// Learn from a condensation outcome
    ///
    /// This is the primary learning method. The engine analyzes the outcome
    /// and updates its internal models accordingly.
    ///
    /// From ROADMAP:
    /// "Learning from condensation outcomes"
    pub fn learn_from_outcome(&mut self, outcome: &CondensationOutcome) {
        // Store the outcome
        self.condensation_history.push(outcome.clone());

        // Extract lessons
        if outcome.success > 0.7 {
            // Successful outcome - reinforce pattern
            self.reinforce_pattern(outcome);
            self.successful_optimizations += 1;
        } else if outcome.success < 0.3 {
            // Failed outcome - adjust pattern
            self.adjust_pattern(outcome);
            self.failed_optimizations += 1;
        }

        // Update intelligence level based on learning
        self.update_intelligence_level();

        // Record lesson in pattern library
        for lesson in &outcome.lessons.clone() {
            self.record_lesson(lesson, outcome.success);
        }
    }

    /// Reinforce a successful pattern
    fn reinforce_pattern(&mut self, outcome: &CondensationOutcome) {
        let pattern_key = outcome.pattern_used.clone();

        // If pattern exists, strengthen it
        if let Some(pattern) = self.pattern_library.get_mut(&pattern_key) {
            pattern.optimization_score = (pattern.optimization_score + outcome.success) / 2.0;
            pattern.expected_coherence = (pattern.expected_coherence + outcome.coherence) / 2.0;
            pattern.expected_stability = (pattern.expected_stability + outcome.stability) / 2.0;
        } else {
            // Create new pattern entry
            let new_pattern = OptimalPattern {
                pattern_id: pattern_key.clone(),
                pattern_type: PatternType::Custom,
                expected_coherence: outcome.coherence,
                expected_stability: outcome.stability,
                required_energy: 0.5,
                estimated_time: outcome.condensation_time,
                success_probability: outcome.success,
                archetype_coefficients: [0.5; 22],
                field_configuration: FieldConfiguration::default(),
                optimization_score: outcome.success,
            };
            self.pattern_library.insert(pattern_key, new_pattern);
        }
    }

    /// Adjust a failing pattern
    fn adjust_pattern(&mut self, outcome: &CondensationOutcome) {
        let pattern_key = outcome.pattern_used.clone();

        if let Some(pattern) = self.pattern_library.get_mut(&pattern_key) {
            // Reduce confidence in this pattern
            pattern.optimization_score *= 0.9;
            pattern.success_probability *= 0.95;
        }

        // Record what went wrong
        self.record_lesson(
            &format!(
                "Avoid pattern {} with coherence < {}",
                pattern_key, outcome.coherence
            ),
            0.0,
        );
    }

    /// Update intelligence level based on learning history
    fn update_intelligence_level(&mut self) {
        let total = self.successful_optimizations + self.failed_optimizations;

        if total > 0 {
            let success_rate = self.successful_optimizations as Float / total as Float;

            // Intelligence grows with successful learning
            self.intelligence_level = self.intelligence_level * (1.0 - self.adaptation_rate)
                + success_rate * self.adaptation_rate;

            self.intelligence_level = self.intelligence_level.min(1.0);
        }
    }

    /// Record a learned lesson
    fn record_lesson(&mut self, lesson: &str, success: Float) {
        // Lessons are stored implicitly in pattern library updates
        // This could be extended to have explicit lesson storage
        let _ = (lesson, success);
    }

    /// Optimize emergence for a target
    ///
    /// Finds the optimal pattern for achieving a specific emergence target.
    ///
    /// From ROADMAP:
    /// "optimize_emergence(&mut self, target: EmergenceTarget) -> OptimalPattern"
    pub fn optimize_emergence(&mut self, target: &EmergenceTarget) -> OptimalPattern {
        // Check pattern library first
        let library_key = format!("{:?}_{}", target.target_type, target.target_coherence);

        if let Some(pattern) = self.pattern_library.get(&library_key) {
            return pattern.clone();
        }

        // Run optimization
        let optimal = self
            .attractor_optimizer
            .optimize(target, &self.pattern_library);

        // Store in library if good enough
        if optimal.optimization_score > 0.5 {
            self.pattern_library.insert(library_key, optimal.clone());
        }

        optimal
    }

    /// Create a new attractor field
    ///
    /// Generates a new attractor field based on specification.
    ///
    /// From ROADMAP:
    /// "create_new_attractor(&mut self, spec: AttractorSpec) -> AttractorField"
    pub fn create_new_attractor(&mut self, spec: &AttractorSpec) -> AttractorField {
        // Create the attractor field
        let attractor = AttractorField::with_signature(
            spec.name.clone(),
            spec.strength,
            spec.orientation,
            spec.archetype_signature,
            spec.target_density,
        );

        // Store in active attractors
        self.active_attractors
            .insert(spec.name.clone(), attractor.clone());

        attractor
    }

    /// Receive entity feedback
    pub fn receive_feedback(&mut self, feedback: EntityFeedback) {
        self.feedback_collector.push(feedback);

        // Trim old feedback if needed
        if self.feedback_collector.len() > 10000 {
            self.feedback_collector.drain(0..1000);
        }
    }

    /// Record a condensation outcome
    pub fn record_condensation(&mut self, mut outcome: CondensationOutcome) {
        outcome.outcome_id = self.next_outcome_id;
        self.next_outcome_id += 1;

        self.learn_from_outcome(&outcome);
    }

    /// Get statistics
    pub fn statistics(&self) -> EngineStatistics {
        let total_outcomes = self.condensation_history.len();
        let successful = self
            .condensation_history
            .iter()
            .filter(|o| o.success > 0.7)
            .count();

        EngineStatistics {
            total_outcomes,
            successful_outcomes: successful,
            failed_outcomes: total_outcomes - successful,
            success_rate: if total_outcomes > 0 {
                successful as Float / total_outcomes as Float
            } else {
                0.0
            },
            intelligence_level: self.intelligence_level,
            pattern_library_size: self.pattern_library.len(),
            active_attractors: self.active_attractors.len(),
            feedback_count: self.feedback_collector.len(),
        }
    }

    /// Get attractor by name
    pub fn get_attractor(&self, name: &str) -> Option<&AttractorField> {
        self.active_attractors.get(name)
    }

    /// Remove an attractor
    pub fn remove_attractor(&mut self, name: &str) -> Option<AttractorField> {
        self.active_attractors.remove(name)
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.condensation_history.clear();
        self.feedback_collector.clear();
    }

    /// Get average coherence from recent outcomes
    pub fn average_coherence(&self, n: usize) -> Float {
        let recent: Vec<_> = self.condensation_history.iter().rev().take(n).collect();

        if recent.is_empty() {
            return 0.5;
        }

        recent.iter().map(|o| o.coherence).sum::<Float>() / recent.len() as Float
    }

    /// Get average stability from recent outcomes
    pub fn average_stability(&self, n: usize) -> Float {
        let recent: Vec<_> = self.condensation_history.iter().rev().take(n).collect();

        if recent.is_empty() {
            return 0.5;
        }

        recent.iter().map(|o| o.stability).sum::<Float>() / recent.len() as Float
    }
}

impl Default for ActiveIntelligenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AttractorOptimizer {
    /// Create a new optimizer
    pub fn new() -> Self {
        Self {
            optimization_history: Vec::new(),
            learning_rate: 0.1,
            max_iterations: 100,
            convergence_threshold: 0.001,
            best_patterns: HashMap::new(),
        }
    }

    /// Optimize for a target
    pub fn optimize(
        &mut self,
        target: &EmergenceTarget,
        pattern_library: &HashMap<String, OptimalPattern>,
    ) -> OptimalPattern {
        let initial_score = 0.5;
        let mut best_pattern = self.create_initial_pattern(target);
        let mut converged = false;

        // Iterative optimization
        for iteration in 0..self.max_iterations {
            let score = self.evaluate_pattern(&best_pattern, target);

            // Check convergence
            if (score - best_pattern.optimization_score).abs() < self.convergence_threshold {
                converged = true;
                break;
            }

            best_pattern.optimization_score = score;

            // Improve pattern
            self.improve_pattern(&mut best_pattern, target, pattern_library);
        }

        // Record optimization
        self.optimization_history.push(OptimizationRecord {
            target_type: target.target_type,
            initial_score,
            final_score: best_pattern.optimization_score,
            iterations: self.max_iterations,
            converged,
            best_pattern: Some(best_pattern.clone()),
        });

        best_pattern
    }

    /// Create initial pattern for a target
    fn create_initial_pattern(&self, target: &EmergenceTarget) -> OptimalPattern {
        let pattern_type = match target.target_type {
            EmergenceTargetType::StableParticle => PatternType::StandingWave,
            EmergenceTargetType::CoherentAtom => PatternType::PlatonicSolid,
            EmergenceTargetType::FunctionalMolecule => PatternType::VesicaPiscis,
            EmergenceTargetType::LivingCell => PatternType::FlowerOfLife,
            EmergenceTargetType::ConsciousEntity => PatternType::Spiral,
            EmergenceTargetType::CollectiveStructure => PatternType::Fibonacci,
            EmergenceTargetType::ArchetypePattern => PatternType::Interference,
        };

        OptimalPattern {
            pattern_id: format!("{:?}_initial", target.target_type),
            pattern_type,
            expected_coherence: target.target_coherence,
            expected_stability: target.target_stability,
            required_energy: target.energy_budget,
            estimated_time: target.time_constraint.unwrap_or(100),
            success_probability: 0.5,
            archetype_coefficients: self.default_archetype_coefficients(target),
            field_configuration: FieldConfiguration::default(),
            optimization_score: 0.5,
        }
    }

    /// Get default archetype coefficients for a target
    fn default_archetype_coefficients(&self, target: &EmergenceTarget) -> [Float; 22] {
        let mut coefficients = [0.5; 22];

        // Adjust based on target type
        match target.target_type {
            EmergenceTargetType::StableParticle => {
                coefficients[0] = 0.8; // Matrix - stability
                coefficients[4] = 0.7; // Teaching/learning
            }
            EmergenceTargetType::CoherentAtom => {
                coefficients[1] = 0.8; // Potentiator
                coefficients[2] = 0.7; // Catalyst
            }
            EmergenceTargetType::FunctionalMolecule => {
                coefficients[2] = 0.8; // Catalyst
                coefficients[3] = 0.7; // Experience
            }
            EmergenceTargetType::LivingCell => {
                coefficients[3] = 0.8; // Experience
                coefficients[5] = 0.7; // Transformation
            }
            EmergenceTargetType::ConsciousEntity => {
                coefficients[5] = 0.8; // Transformation
                coefficients[6] = 0.7; // Great Way
            }
            EmergenceTargetType::CollectiveStructure => {
                coefficients[6] = 0.8; // Great Way
                coefficients[21] = 0.7; // Choice
            }
            EmergenceTargetType::ArchetypePattern => {
                coefficients[21] = 0.9; // Choice - highest
            }
        }

        coefficients
    }

    /// Evaluate a pattern against a target
    fn evaluate_pattern(&self, pattern: &OptimalPattern, target: &EmergenceTarget) -> Float {
        let coherence_score = 1.0 - (pattern.expected_coherence - target.target_coherence).abs();
        let stability_score = 1.0 - (pattern.expected_stability - target.target_stability).abs();
        let energy_score = if pattern.required_energy <= target.energy_budget {
            1.0
        } else {
            target.energy_budget / pattern.required_energy
        };

        (coherence_score + stability_score + energy_score) / 3.0
    }

    /// Improve a pattern
    fn improve_pattern(
        &self,
        pattern: &mut OptimalPattern,
        target: &EmergenceTarget,
        _pattern_library: &HashMap<String, OptimalPattern>,
    ) {
        // Adjust archetype coefficients toward target
        for i in 0..22 {
            let adjustment =
                (target.target_coherence - pattern.expected_coherence) * self.learning_rate * 0.1;
            pattern.archetype_coefficients[i] =
                (pattern.archetype_coefficients[i] + adjustment).clamp(0.0, 1.0);
        }

        // Improve field configuration
        pattern.field_configuration.amplitude *=
            1.0 + self.learning_rate * (target.target_coherence - 0.5);
    }
}

impl Default for AttractorOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FieldConfiguration {
    fn default() -> Self {
        Self {
            base_frequency: 432.0, // A=432Hz sacred frequency
            amplitude: 0.5,
            phase: 0.0,
            harmonics: vec![1.0, 0.5, 0.33, 0.25], // First 4 harmonics
            spatial_extent: 1.0,
        }
    }
}

/// Engine statistics
#[derive(Debug, Clone)]
pub struct EngineStatistics {
    pub total_outcomes: usize,
    pub successful_outcomes: usize,
    pub failed_outcomes: usize,
    pub success_rate: Float,
    pub intelligence_level: Float,
    pub pattern_library_size: usize,
    pub active_attractors: usize,
    pub feedback_count: usize,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = ActiveIntelligenceEngine::new();

        assert!(engine.feedback_collector.is_empty());
        assert!(engine.condensation_history.is_empty());
        assert!((engine.intelligence_level - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_learn_from_successful_outcome() {
        let mut engine = ActiveIntelligenceEngine::new();

        let outcome = CondensationOutcome {
            outcome_id: 0,
            entity_id: None,
            condensation_type: CondensationType::Atom,
            success: 0.9,
            coherence: 0.85,
            energy_efficiency: 0.8,
            pattern_used: "test_pattern".to_string(),
            condensation_time: 100,
            stability: 0.9,
            catalyst: None,
            lessons: vec!["Lesson 1".to_string()],
        };

        engine.learn_from_outcome(&outcome);

        assert_eq!(engine.successful_optimizations, 1);
        assert!(engine.pattern_library.contains_key("test_pattern"));
    }

    #[test]
    fn test_learn_from_failed_outcome() {
        let mut engine = ActiveIntelligenceEngine::new();

        // First add a pattern
        engine.pattern_library.insert(
            "failing_pattern".to_string(),
            OptimalPattern {
                pattern_id: "failing_pattern".to_string(),
                pattern_type: PatternType::Custom,
                expected_coherence: 0.5,
                expected_stability: 0.5,
                required_energy: 0.5,
                estimated_time: 100,
                success_probability: 0.8,
                archetype_coefficients: [0.5; 22],
                field_configuration: FieldConfiguration::default(),
                optimization_score: 0.8,
            },
        );

        let outcome = CondensationOutcome {
            outcome_id: 0,
            entity_id: None,
            condensation_type: CondensationType::Atom,
            success: 0.1,
            coherence: 0.2,
            energy_efficiency: 0.3,
            pattern_used: "failing_pattern".to_string(),
            condensation_time: 100,
            stability: 0.1,
            catalyst: None,
            lessons: vec![],
        };

        engine.learn_from_outcome(&outcome);

        assert_eq!(engine.failed_optimizations, 1);
        // Score should be reduced
        assert!(
            engine
                .pattern_library
                .get("failing_pattern")
                .unwrap()
                .optimization_score
                < 0.8
        );
    }

    #[test]
    fn test_optimize_emergence() {
        let mut engine = ActiveIntelligenceEngine::new();

        let target = EmergenceTarget {
            target_type: EmergenceTargetType::CoherentAtom,
            target_coherence: 0.8,
            target_stability: 0.8,
            time_constraint: Some(100),
            energy_budget: 1.0,
            polarity_alignment: None,
            constraints: vec![],
        };

        let pattern = engine.optimize_emergence(&target);

        assert!(pattern.optimization_score >= 0.0);
        assert!(!pattern
            .archetype_coefficients
            .iter()
            .any(|&c| c < 0.0 || c > 1.0));
    }

    #[test]
    fn test_create_new_attractor() {
        let mut engine = ActiveIntelligenceEngine::new();

        let spec = AttractorSpec {
            name: "test_attractor".to_string(),
            target_density: TargetDensity::Fourth,
            orientation: Orientation::STO,
            strength: 0.8,
            archetype_signature: [0.5; 22],
            stability_requirement: 0.7,
            adaptive: true,
        };

        let attractor = engine.create_new_attractor(&spec);

        assert_eq!(attractor.strength, 0.8);
        assert!(engine.active_attractors.contains_key("test_attractor"));
    }

    #[test]
    fn test_receive_feedback() {
        let mut engine = ActiveIntelligenceEngine::new();

        let feedback = EntityFeedback::new(EntityId::new("test".to_string()), 0.7, 0.8, 0.9);
        engine.receive_feedback(feedback);

        assert_eq!(engine.feedback_collector.len(), 1);
    }

    #[test]
    fn test_record_condensation() {
        let mut engine = ActiveIntelligenceEngine::new();

        let outcome = CondensationOutcome {
            outcome_id: 0,
            entity_id: None,
            condensation_type: CondensationType::Molecule,
            success: 0.8,
            coherence: 0.75,
            energy_efficiency: 0.8,
            pattern_used: "molecular_pattern".to_string(),
            condensation_time: 200,
            stability: 0.85,
            catalyst: None,
            lessons: vec![],
        };

        engine.record_condensation(outcome);

        assert_eq!(engine.condensation_history.len(), 1);
        assert_eq!(engine.condensation_history[0].outcome_id, 1);
    }

    #[test]
    fn test_statistics() {
        let mut engine = ActiveIntelligenceEngine::new();

        // Record some outcomes
        for i in 0..10 {
            let outcome = CondensationOutcome {
                outcome_id: i,
                entity_id: None,
                condensation_type: CondensationType::Atom,
                success: if i < 7 { 0.8 } else { 0.2 },
                coherence: 0.7,
                energy_efficiency: 0.7,
                pattern_used: format!("pattern_{}", i),
                condensation_time: 100,
                stability: 0.7,
                catalyst: None,
                lessons: vec![],
            };
            engine.learn_from_outcome(&outcome);
        }

        let stats = engine.statistics();

        assert_eq!(stats.total_outcomes, 10);
        assert_eq!(stats.successful_outcomes, 7);
        assert_eq!(stats.failed_outcomes, 3);
        assert!((stats.success_rate - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_average_coherence() {
        let mut engine = ActiveIntelligenceEngine::new();

        for i in 0..5 {
            let outcome = CondensationOutcome {
                outcome_id: i,
                entity_id: None,
                condensation_type: CondensationType::Atom,
                success: 0.8,
                coherence: 0.5 + i as Float * 0.1,
                energy_efficiency: 0.7,
                pattern_used: format!("pattern_{}", i),
                condensation_time: 100,
                stability: 0.7,
                catalyst: None,
                lessons: vec![],
            };
            engine.learn_from_outcome(&outcome);
        }

        // Average of last 3 coherences: 0.7, 0.8, 0.9
        let avg = engine.average_coherence(3);
        assert!((avg - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_intelligence_level_increases() {
        let mut engine = ActiveIntelligenceEngine::new();
        let initial_level = engine.intelligence_level;

        // Add many successful outcomes
        for _ in 0..10 {
            let outcome = CondensationOutcome {
                outcome_id: 0,
                entity_id: None,
                condensation_type: CondensationType::Atom,
                success: 0.9,
                coherence: 0.9,
                energy_efficiency: 0.9,
                pattern_used: "success_pattern".to_string(),
                condensation_time: 100,
                stability: 0.9,
                catalyst: None,
                lessons: vec![],
            };
            engine.learn_from_outcome(&outcome);
        }

        assert!(engine.intelligence_level > initial_level);
    }

    #[test]
    fn test_attractor_optimizer_creation() {
        let optimizer = AttractorOptimizer::new();

        assert!(optimizer.optimization_history.is_empty());
        assert_eq!(optimizer.max_iterations, 100);
    }

    #[test]
    fn test_field_configuration_default() {
        let config = FieldConfiguration::default();

        assert_eq!(config.base_frequency, 432.0);
        assert!((config.amplitude - 0.5).abs() < 0.001);
        assert_eq!(config.harmonics.len(), 4);
    }

    #[test]
    fn test_get_attractor() {
        let mut engine = ActiveIntelligenceEngine::new();

        let spec = AttractorSpec {
            name: "my_attractor".to_string(),
            target_density: TargetDensity::Fifth,
            orientation: Orientation::Balanced,
            strength: 0.6,
            archetype_signature: [0.5; 22],
            stability_requirement: 0.5,
            adaptive: false,
        };

        engine.create_new_attractor(&spec);

        let attractor = engine.get_attractor("my_attractor");
        assert!(attractor.is_some());

        let missing = engine.get_attractor("nonexistent");
        assert!(missing.is_none());
    }

    #[test]
    fn test_remove_attractor() {
        let mut engine = ActiveIntelligenceEngine::new();

        let spec = AttractorSpec {
            name: "to_remove".to_string(),
            target_density: TargetDensity::Third,
            orientation: Orientation::STO,
            strength: 0.5,
            archetype_signature: [0.5; 22],
            stability_requirement: 0.5,
            adaptive: false,
        };

        engine.create_new_attractor(&spec);
        let removed = engine.remove_attractor("to_remove");

        assert!(removed.is_some());
        assert!(!engine.active_attractors.contains_key("to_remove"));
    }
}
