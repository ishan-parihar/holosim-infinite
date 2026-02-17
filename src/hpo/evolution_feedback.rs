//! Evolution Feedback (Phase 4)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "The fifth phase adds the bottom-up feedback paths that allow entity decisions and 
//! experiences to modify the field configuration."
//!
//! This module implements:
//! - Entity decisions as field perturbations
//! - Continuous density progression via field growth
//! - Cosmic structure modification by entities
//! - Bidirectional flow: Involution (top-down) ↔ Evolution (bottom-up)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "All development flows from this moment" - entity choices shape the cosmos

use super::field_state::{Complex, DensityBand, FieldNodeData, Float, HolographicFieldState, OctreeNode};
use super::spectrum_dynamics::SpectrumDynamics;
use super::involution_flow::CosmicHierarchy;

/// Types of entity decisions that affect the field
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecisionType {
    Growth = 0,      // Seeking to grow/evolve
    Service = 1,      // Service to others
    Control = 2,      // Service to self/control
    Connection = 3,   // Seeking connection/resonance
    Isolation = 4,    // Seeking separation
    Learning = 5,     // Learning from catalyst
    Expression = 6,   // Expressing self
    Reception = 7,   // Receiving from others
}

impl DecisionType {
    pub fn count() -> usize { 8 }
    
    pub fn from_index(i: usize) -> Option<DecisionType> {
        match i {
            0 => Some(DecisionType::Growth),
            1 => Some(DecisionType::Service),
            2 => Some(DecisionType::Control),
            3 => Some(DecisionType::Connection),
            4 => Some(DecisionType::Isolation),
            5 => Some(DecisionType::Learning),
            6 => Some(DecisionType::Expression),
            7 => Some(DecisionType::Reception),
            _ => None,
        }
    }
    
    /// Get the field perturbation signature for this decision type
    /// Each decision type creates a distinct field pattern
    pub fn perturbation_signature(&self) -> [Float; 8] {
        match self {
            DecisionType::Growth => {
                // Growth increases all densities
                [0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]
            }
            DecisionType::Service => {
                // Service to others strengthens Green (4th density)
                [0.0, 0.0, 0.0, 0.8, 0.2, 0.1, 0.0, 0.0]
            }
            DecisionType::Control => {
                // Control focuses on self - Red (7th density)
                [0.0, 0.0, 0.0, 0.1, 0.2, 0.3, 0.8, 0.0]
            }
            DecisionType::Connection => {
                // Connection - all densities harmonize
                [0.15, 0.2, 0.25, 0.3, 0.25, 0.2, 0.15, 0.1]
            }
            DecisionType::Isolation => {
                // Isolation - focus on lower densities
                [0.4, 0.3, 0.2, 0.1, 0.0, 0.0, 0.0, 0.0]
            }
            DecisionType::Learning => {
                // Learning - Yellow (5th density) wisdom
                [0.0, 0.0, 0.1, 0.2, 0.7, 0.3, 0.1, 0.0]
            }
            DecisionType::Expression => {
                // Expression - Orange (6th density)
                [0.0, 0.0, 0.0, 0.1, 0.2, 0.7, 0.3, 0.0]
            }
            DecisionType::Reception => {
                // Reception - balanced
                [0.1, 0.15, 0.2, 0.25, 0.2, 0.15, 0.1, 0.05]
            }
        }
    }
}

/// Entity decision that affects the field
#[derive(Debug, Clone)]
pub struct EntityDecision {
    /// Entity position in field
    pub position: [Float; 3],
    
    /// Type of decision
    pub decision_type: DecisionType,
    
    /// Significance/strength of decision (0.0 - 1.0)
    pub significance: Float,
    
    /// Phase of decision (for resonance calculations)
    pub phase: Float,
    
    /// Time of decision
    pub time: Float,
}

impl EntityDecision {
    pub fn new(position: [Float; 3], decision_type: DecisionType, significance: Float) -> Self {
        EntityDecision {
            position,
            decision_type,
            significance: significance.clamp(0.0, 1.0),
            phase: 0.0,
            time: 0.0,
        }
    }
}

/// Configuration for evolution feedback
#[derive(Debug, Clone)]
pub struct EvolutionFeedbackConfig {
    /// Feedback strength - how much entity decisions affect field
    pub feedback_strength: Float,
    
    /// Decay rate for decision perturbations
    pub perturbation_decay: Float,
    
    /// Range of influence for decisions
    pub influence_range: Float,
    
    /// Whether to enable density progression via feedback
    pub enable_density_progression: bool,
    
    /// Minimum significance for density progression
    pub progression_threshold: Float,
    
    /// Time step
    pub time_step: Float,
}

impl Default for EvolutionFeedbackConfig {
    fn default() -> Self {
        EvolutionFeedbackConfig {
            feedback_strength: 0.5,
            perturbation_decay: 0.99,
            influence_range: 50.0,
            enable_density_progression: true,
            progression_threshold: 0.3,
            time_step: 0.01,
        }
    }
}

/// Feedback from entity decisions to field
pub struct DecisionFeedback {
    /// Pending decisions to apply
    pending_decisions: Vec<EntityDecision>,
    
    /// Historical decisions (for pattern analysis)
    decision_history: Vec<EntityDecision>,
    
    /// Configuration
    config: EvolutionFeedbackConfig,
}

impl DecisionFeedback {
    pub fn new(config: EvolutionFeedbackConfig) -> Self {
        DecisionFeedback {
            pending_decisions: Vec::new(),
            decision_history: Vec::new(),
            config,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(EvolutionFeedbackConfig::default())
    }

    /// Queue a decision for feedback
    pub fn add_decision(&mut self, decision: EntityDecision) {
        self.pending_decisions.push(decision.clone());
        self.decision_history.push(decision);
        
        // Keep history bounded
        if self.decision_history.len() > 10000 {
            self.decision_history.remove(0);
        }
    }

    /// Apply all pending decisions to the field
    pub fn apply_to_field(&mut self, field: &mut HolographicFieldState) {
        // Collect decisions first to avoid borrow issues
        let decisions: Vec<_> = self.pending_decisions.drain(..).collect();
        for decision in decisions {
            self.apply_decision_to_field(&decision, field);
        }
    }

    /// Apply a single decision to the field
    fn apply_decision_to_field(&self, decision: &EntityDecision, field: &mut HolographicFieldState) {
        // Get perturbation signature
        let signature = decision.decision_type.perturbation_signature();
        
        // Apply to field at entity position
        // The perturbation affects nearby nodes based on influence range
        self.apply_perturbation_at(field, &decision.position, &signature, decision.significance);
    }

    /// Apply perturbation at a position
    fn apply_perturbation_at(&self, field: &mut HolographicFieldState, position: &[Float; 3], signature: &[Float; 8], significance: Float) {
        // Walk the octree and apply perturbation
        self.apply_to_node(&mut field.root, position, signature, significance);
    }

    /// Recursively apply perturbation to nodes
    fn apply_to_node(&self, node: &mut OctreeNode, entity_pos: &[Float; 3], signature: &[Float; 8], significance: Float) {
        let center = node.bounds.center();
        
        // Calculate distance
        let dx = entity_pos[0] - center[0];
        let dy = entity_pos[1] - center[1];
        let dz = entity_pos[2] - center[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        
        // Calculate falloff based on influence range
        let falloff = if dist < self.config.influence_range {
            1.0 - dist / self.config.influence_range
        } else {
            0.0
        };
        
        if falloff > 0.001 {
            let strength = self.config.feedback_strength * significance * falloff;
            
            // Apply signature to density amplitudes
            for (i, &sig_val) in signature.iter().enumerate() {
                if i < 8 {
                    let perturbation = sig_val * strength;
                    node.field_data.density_amplitudes[i].re += perturbation;
                    
                    // Keep bounded
                    let mag = node.field_data.density_amplitudes[i].magnitude();
                    if mag > 1.0 {
                        let scale = 1.0 / mag;
                        node.field_data.density_amplitudes[i] = node.field_data.density_amplitudes[i].scale(scale);
                    }
                }
            }
            
            // Apply to coherence
            node.field_data.coherence = (node.field_data.coherence + strength * 0.1).min(1.0);
            
            // Apply to energy
            node.field_data.energy += strength * 0.5;
        }
        
        // Recurse to children
        if let Some(ref mut children) = node.children {
            for child in children.iter_mut() {
                self.apply_to_node(child, entity_pos, signature, significance);
            }
        }
    }

    /// Get decision history for analysis
    pub fn get_history(&self) -> &[EntityDecision] {
        &self.decision_history
    }

    /// Clear pending decisions
    pub fn clear_pending(&mut self) {
        self.pending_decisions.clear();
    }
}

/// Density progression via field growth
/// Instead of discrete jumps, entities progress continuously
pub struct DensityProgression {
    /// Current density for each entity
    densities: Vec<Float>,
    
    /// Progression rate for each entity
    progression_rates: Vec<Float>,
    
    /// Configuration
    config: EvolutionFeedbackConfig,
}

impl DensityProgression {
    pub fn new(entity_count: usize) -> Self {
        let mut densities = Vec::new();
        let mut progression_rates = Vec::new();
        
        for _ in 0..entity_count {
            densities.push(0.0); // Start at 1st density
            progression_rates.push(0.001); // Default progression rate
        }
        
        DensityProgression {
            densities,
            progression_rates,
            config: EvolutionFeedbackConfig::default(),
        }
    }

    /// Update progression based on entity decisions
    pub fn update_from_decisions(&mut self, decisions: &[EntityDecision]) {
        for decision in decisions {
            // Map position to entity index (simplified)
            let idx = self.entity_index_from_position(decision.position);
            
            if idx < self.densities.len() {
                // Update progression rate based on decision type
                let rate_change = match decision.decision_type {
                    DecisionType::Growth => 0.001,
                    DecisionType::Service => 0.0005,
                    DecisionType::Learning => 0.0008,
                    DecisionType::Control => -0.0002,
                    DecisionType::Isolation => -0.0005,
                    _ => 0.0,
                };
                
                self.progression_rates[idx] = (self.progression_rates[idx] + rate_change).max(0.0).min(0.01);
            }
        }
    }

    /// Evolve densities based on progression rates
    pub fn evolve(&mut self, dt: Float) {
        for i in 0..self.densities.len() {
            self.densities[i] += self.progression_rates[i] * dt;
            
            // Clamp to valid range (0-8, can exceed for transcendent beings)
            if self.densities[i] < 0.0 {
                self.densities[i] = 0.0;
            }
        }
    }

    /// Get density for entity
    pub fn get_density(&self, entity_idx: usize) -> Float {
        if entity_idx < self.densities.len() {
            self.densities[entity_idx]
        } else {
            0.0
        }
    }

    /// Convert density to density band amplitudes
    pub fn get_density_amplitudes(&self, entity_idx: usize) -> [Float; 8] {
        let density = self.get_density(entity_idx);
        
        // Convert to Gaussian distribution
        let mut result = [0.0; 8];
        
        for i in 0..8 {
            let density_center = i as Float;
            let distance = (density - density_center).abs();
            result[i] = (-distance * distance * 10.0).exp();
        }
        
        // Normalize
        let sum: Float = result.iter().sum();
        if sum > 0.0 {
            for r in result.iter_mut() {
                *r /= sum;
            }
        }
        
        result
    }

    /// Map position to entity index (simplified)
    fn entity_index_from_position(&self, position: [Float; 3]) -> usize {
        // Simplified - use hash of position
        ((position[0].abs() + position[1].abs() + position[2].abs()) as usize) % self.densities.len()
    }
    
    /// Add more entities
    pub fn add_entity(&mut self) {
        self.densities.push(0.0);
        self.progression_rates.push(0.001);
    }
}

/// Complete evolution feedback system
pub struct EvolutionFeedback {
    /// Decision feedback
    decision_feedback: DecisionFeedback,
    
    /// Density progression
    density_progression: DensityProgression,
    
    /// Configuration
    config: EvolutionFeedbackConfig,
    
    /// Statistics
    pub statistics: EvolutionFeedbackStatistics,
}

/// Statistics for evolution feedback
#[derive(Debug, Clone, Default)]
pub struct EvolutionFeedbackStatistics {
    /// Total decisions processed
    pub decisions_processed: usize,
    
    /// Average progression rate
    pub average_progression_rate: Float,
    
    /// Entities at each density level
    pub density_distribution: [usize; 8],
    
    /// Total field modification
    pub total_field_modification: Float,
}

impl EvolutionFeedback {
    pub fn new(entity_count: usize) -> Self {
        EvolutionFeedback {
            decision_feedback: DecisionFeedback::new(EvolutionFeedbackConfig::default()),
            density_progression: DensityProgression::new(entity_count),
            config: EvolutionFeedbackConfig::default(),
            statistics: EvolutionFeedbackStatistics::default(),
        }
    }

    /// Add an entity decision
    pub fn add_decision(&mut self, decision: EntityDecision) {
        self.decision_feedback.add_decision(decision);
    }

    /// Process evolution feedback
    pub fn process(&mut self, field: &mut HolographicFieldState) {
        // Apply pending decisions to field
        self.decision_feedback.apply_to_field(field);
        
        // Update density progression from decisions
        let decisions: Vec<_> = self.decision_feedback.get_history().iter().rev().take(100).cloned().collect();
        self.density_progression.update_from_decisions(&decisions);
        
        // Evolve densities
        self.density_progression.evolve(self.config.time_step);
        
        // Update statistics
        self.statistics.decisions_processed += 1;
        
        // Calculate average progression rate
        let sum: Float = self.density_progression.progression_rates.iter().sum();
        let count = self.density_progression.progression_rates.len();
        self.statistics.average_progression_rate = if count > 0 { sum / count as Float } else { 0.0 };
        
        // Calculate density distribution
        for density in &self.density_progression.densities {
            let idx = (*density as usize).min(7);
            self.statistics.density_distribution[idx] += 1;
        }
    }

    /// Get density amplitudes for entity
    pub fn get_entity_density_amplitudes(&self, entity_idx: usize) -> [Float; 8] {
        self.density_progression.get_density_amplitudes(entity_idx)
    }

    /// Add a new entity
    pub fn add_entity(&mut self) {
        self.density_progression.add_entity();
    }

    /// Get entity density
    pub fn get_entity_density(&self, entity_idx: usize) -> Float {
        self.density_progression.get_density(entity_idx)
    }

    /// Apply density progression to field
    pub fn apply_densities_to_field(&self, field: &mut HolographicFieldState, entity_idx: usize, position: [Float; 3]) {
        let amplitudes = self.get_entity_density_amplitudes(entity_idx);
        
        // Apply to field at position
        for (i, &amp) in amplitudes.iter().enumerate() {
            field.add_energy_at(position, i, amp);
        }
    }
}

/// Apply evolution feedback to a node
pub fn apply_evolution_to_node(
    node: &mut OctreeNode,
    density_amplitudes: &[Float; 8],
    coherence_boost: Float,
) {
    // Update density amplitudes
    for (i, &amp) in density_amplitudes.iter().enumerate() {
        if i < 8 {
            let current = node.field_data.density_amplitudes[i].magnitude();
            let updated = current * (1.0 - 0.1) + amp * 0.1;
            node.field_data.density_amplitudes[i] = Complex::from_polar(updated, node.field_data.density_amplitudes[i].phase());
        }
    }
    
    // Boost coherence
    node.field_data.coherence = (node.field_data.coherence + coherence_boost).min(1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_creation() {
        let decision = EntityDecision::new([0.0, 0.0, 0.0], DecisionType::Growth, 0.5);
        assert_eq!(decision.decision_type, DecisionType::Growth);
        assert_eq!(decision.significance, 0.5);
    }

    #[test]
    fn test_perturbation_signature() {
        let sig = DecisionType::Growth.perturbation_signature();
        assert_eq!(sig.len(), 8);
    }

    #[test]
    fn test_density_progression() {
        let mut progression = DensityProgression::new(10);
        progression.evolve(0.1);
        assert!(progression.get_density(0) >= 0.0);
    }

    #[test]
    fn test_evolution_feedback() {
        let mut feedback = EvolutionFeedback::new(10);
        let decision = EntityDecision::new([0.0, 0.0, 0.0], DecisionType::Growth, 0.5);
        feedback.add_decision(decision);
        
        let mut field = HolographicFieldState::with_defaults();
        feedback.process(&mut field);
        
        assert!(feedback.statistics.decisions_processed >= 1);
    }
}
