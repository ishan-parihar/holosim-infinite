//! Teleological Guidance - V5 Phase 6 Integration Implementation
//!
//! This module activates teleological guidance for entity evolution.
//! It computes guidance metrics and applies them to modify evolution
//! probability distributions.
//!
//! From V5 Phase 6 specifications:
//! - TeleologicalGuidance provides adaptive attractor-based evolution guidance
//! - AdaptiveAttractor represents spiritual gravity pulling entities toward densities
//! - EntityFeedback enables adaptive adjustment based on collective progress
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Evolution is not random but is guided by the Law of One toward greater
//! unity and service to others. This guidance is the teleological pull of
//! the Infinite Creator."

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::types::{Density, EntityId, Float};
use std::collections::HashMap;

// ============================================================================
// ADAPTIVE ATTRACTOR
// ============================================================================

/// An adaptive attractor representing spiritual gravity
///
/// Each density has an attractor that pulls entities toward it.
/// The strength adapts based on collective entity feedback.
#[derive(Clone, Debug)]
pub struct AdaptiveAttractor {
    /// Target density for this attractor
    pub target_density: Density,
    
    /// Current strength (0.0 to 1.0)
    pub current_strength: Float,
    
    /// Base pull strength
    pub base_pull: Float,
    
    /// Number of entities influenced
    pub entities_influenced: u64,
    
    /// Average progress of influenced entities
    pub average_progress: Float,
}

impl AdaptiveAttractor {
    /// Create an attractor for a density
    pub fn for_density(density: Density) -> Self {
        Self {
            target_density: density,
            current_strength: 0.5,
            base_pull: 0.1,
            entities_influenced: 0,
            average_progress: 0.0,
        }
    }
    
    /// Compute pull toward this density for an evolution option
    pub fn compute_pull(&self, purpose_alignment: Float, density_progress: Float) -> Float {
        // Pull based on alignment with teleological direction
        let alignment = purpose_alignment;
        
        // How much this option moves toward target density
        let density_alignment = if density_progress > 0.0 {
            self.current_strength * alignment
        } else {
            -self.current_strength * 0.5
        };
        
        self.base_pull * density_alignment
    }
    
    /// Strengthen the attractor
    pub fn strengthen(&mut self, amount: Float) {
        self.current_strength = (self.current_strength + amount).min(1.0);
    }
    
    /// Weaken the attractor
    pub fn weaken(&mut self, amount: Float) {
        self.current_strength = (self.current_strength - amount).max(0.1);
    }
    
    /// Record entity feedback
    pub fn record_feedback(&mut self, progress: Float) {
        let total_progress = self.average_progress * self.entities_influenced as Float + progress;
        self.entities_influenced += 1;
        self.average_progress = total_progress / self.entities_influenced as Float;
    }
}

// ============================================================================
// ENTITY FEEDBACK
// ============================================================================

/// Feedback from entity to attractors
#[derive(Clone, Debug)]
pub struct EntityFeedback {
    /// Entity ID
    pub entity_id: EntityId,
    
    /// Evolution progress (0.0 to 1.0)
    pub evolution_progress: Float,
    
    /// Total attractor pull received
    pub attractor_pull_received: Float,
    
    /// Effectiveness of guidance (progress / pull)
    pub effectiveness: Float,
    
    /// Timestamp
    pub timestamp: Float,
    
    /// Current density
    pub current_density: Density,
    
    /// Spectrum position
    pub spectrum_position: SpectrumPosition,
}

impl EntityFeedback {
    /// Create new feedback
    pub fn new(
        entity_id: EntityId,
        evolution_progress: Float,
        attractor_pull_received: Float,
        current_density: Density,
        spectrum_position: SpectrumPosition,
    ) -> Self {
        let effectiveness = if attractor_pull_received > 0.0 {
            evolution_progress / attractor_pull_received
        } else {
            0.0
        };
        
        Self {
            entity_id,
            evolution_progress,
            attractor_pull_received,
            effectiveness,
            timestamp: current_time(),
            current_density,
            spectrum_position,
        }
    }
}

// ============================================================================
// EVOLUTION OPTION
// ============================================================================

/// An evolution option for an entity
#[derive(Clone, Debug)]
pub struct EvolutionOption {
    /// Description of this option
    pub description: String,
    
    /// Base probability (before guidance)
    pub probability: Float,
    
    /// Archetype influence weights (22 archetypes)
    pub archetype_influence: [Float; 22],
    
    /// Polarity shift from this option (-1.0 to 1.0)
    pub polarity_shift: Float,
    
    /// Progress toward next density
    pub density_progress: Float,
}

impl EvolutionOption {
    /// Create a new evolution option
    pub fn new(description: String, probability: Float) -> Self {
        Self {
            description,
            probability,
            archetype_influence: [0.0; 22],
            polarity_shift: 0.0,
            density_progress: 0.0,
        }
    }
    
    /// Create with density progress
    pub fn with_density_progress(mut self, progress: Float) -> Self {
        self.density_progress = progress;
        self
    }
    
    /// Create with polarity shift
    pub fn with_polarity_shift(mut self, shift: Float) -> Self {
        self.polarity_shift = shift;
        self
    }
    
    /// Create with archetype influence
    pub fn with_archetype_influence(mut self, influence: [Float; 22]) -> Self {
        self.archetype_influence = influence;
        self
    }
}

// ============================================================================
// TELEOLOGICAL PROGRESS
// ============================================================================

/// Teleological progress evaluation for an entity
#[derive(Clone, Debug)]
pub struct TeleologicalProgress {
    /// Purpose alignment (0.0 to 1.0)
    pub purpose_alignment: Float,
    
    /// Service-to-others orientation (0.0 to 1.0)
    pub sto_orientation: Float,
    
    /// Service-to-self orientation (0.0 to 1.0)
    pub sts_orientation: Float,
    
    /// Current density
    pub current_density: Density,
    
    /// Density progression (0.0 to 1.0 within current density)
    pub density_progression: Float,
    
    /// Archetype activation level
    pub archetype_activation: Float,
}

impl TeleologicalProgress {
    /// Evaluate teleological progress from entity data
    pub fn evaluate(
        density: Density,
        archetype_activation: [Float; 22],
        polarity: Float,
        spectrum_position: &SpectrumPosition,
    ) -> Self {
        // Purpose alignment based on veil transparency and archetype activation
        let avg_archetype: Float = archetype_activation.iter().sum::<Float>() / 22.0;
        let purpose_alignment = (avg_archetype + spectrum_position.veil_transparency()) / 2.0;
        
        // STO/STS orientation from polarity
        let sto_orientation = if polarity > 0.0 { polarity } else { 0.5 };
        let sts_orientation = if polarity < 0.0 { -polarity } else { 0.5 };
        
        // Density progression from spectrum position
        let density_progression = spectrum_position.veil_transparency();
        
        Self {
            purpose_alignment,
            sto_orientation,
            sts_orientation,
            current_density: density,
            density_progression,
            archetype_activation: avg_archetype,
        }
    }
    
    /// Create default progress
    pub fn default_for_density(density: Density) -> Self {
        Self {
            purpose_alignment: 0.5,
            sto_orientation: 0.5,
            sts_orientation: 0.5,
            current_density: density,
            density_progression: 0.0,
            archetype_activation: 0.5,
        }
    }
}

// ============================================================================
// TELEOLOGICAL GUIDANCE
// ============================================================================

/// Activates teleological guidance for entity evolution
///
/// This system:
/// 1. Creates attractors for each density (spiritual gravity)
/// 2. Computes guidance for entity evolution choices
/// 3. Modifies probability distributions based on teleological pull
/// 4. Receives feedback to adapt attractor strengths
pub struct TeleologicalGuidance {
    /// Adaptive attractors (one per density, indexed by density as usize)
    attractors: HashMap<u8, AdaptiveAttractor>,
    
    /// Entity feedback cache
    feedback_cache: HashMap<EntityId, EntityFeedback>,
    
    /// Guidance strength (how much attractors pull)
    guidance_strength: Float,
    
    /// Statistics
    stats: GuidanceStats,
}

/// Statistics for the guidance system
#[derive(Debug, Clone, Default)]
pub struct GuidanceStats {
    pub total_guidance_applied: u64,
    pub total_feedback_received: u64,
    pub average_effectiveness: Float,
    pub peak_alignment: Float,
}

impl TeleologicalGuidance {
    /// Create new teleological guidance system
    pub fn new() -> Self {
        Self {
            attractors: Self::create_attractors(),
            feedback_cache: HashMap::new(),
            guidance_strength: 0.3,
            stats: GuidanceStats::default(),
        }
    }
    
    /// Create with custom guidance strength
    pub fn with_strength(strength: Float) -> Self {
        Self {
            guidance_strength: strength.clamp(0.0, 1.0),
            ..Self::new()
        }
    }
    
    /// Create the 8 attractors (one per density)
    fn create_attractors() -> HashMap<u8, AdaptiveAttractor> {
        let mut attractors = HashMap::new();
        for d in 1..=8 {
            let density = Density::from_u8(d);
            attractors.insert(d, AdaptiveAttractor::for_density(density));
        }
        attractors
    }
    
    /// Compute guidance for entity evolution choice
    pub fn compute_guidance(
        &self,
        teleological: &TeleologicalProgress,
        options: &[EvolutionOption],
    ) -> Vec<(usize, Float)> {
        // Get relevant attractor
        let density_idx = teleological.current_density as u8;
        let attractor = self.attractors.get(&density_idx);
        
        // Modify option probabilities based on guidance
        options.iter().enumerate().map(|(i, option)| {
            let base_prob = option.probability;
            
            // Compute pull toward this option
            let pull = match attractor {
                Some(a) => a.compute_pull(
                    teleological.purpose_alignment,
                    option.density_progress,
                ),
                None => 0.0,
            };
            
            // Modified probability
            let modified = base_prob * (1.0 + pull * self.guidance_strength);
            
            (i, modified.max(0.0))
        }).collect()
    }
    
    /// Apply guidance to evolution options
    pub fn apply(&self, teleological: &TeleologicalProgress, evolution_options: &mut [EvolutionOption]) {
        let guided_probs = self.compute_guidance(teleological, evolution_options);
        
        // Normalize probabilities
        let total: Float = guided_probs.iter().map(|(_, p)| *p).sum();
        
        // Update option probabilities
        if total > 0.0 {
            for (i, prob) in guided_probs {
                evolution_options[i].probability = prob / total;
            }
        }
    }
    
    /// Receive feedback from entity (for adaptive adjustment)
    pub fn receive_feedback(
        &mut self,
        entity_id: EntityId,
        progress: Float,
        pull_received: Float,
        current_density: Density,
        spectrum_position: SpectrumPosition,
    ) {
        let feedback = EntityFeedback::new(
            entity_id.clone(),
            progress,
            pull_received,
            current_density,
            spectrum_position,
        );
        
        // Update stats
        self.stats.total_feedback_received += 1;
        let total_eff = self.stats.average_effectiveness * (self.stats.total_feedback_received - 1) as Float;
        self.stats.average_effectiveness = (total_eff + feedback.effectiveness) / self.stats.total_feedback_received as Float;
        
        if feedback.effectiveness > self.stats.peak_alignment {
            self.stats.peak_alignment = feedback.effectiveness;
        }
        
        // Store feedback
        self.feedback_cache.insert(entity_id, feedback);
        
        // Adjust attractors based on feedback
        self.adjust_attractors();
    }
    
    /// Adjust attractor strengths based on collective feedback
    fn adjust_attractors(&mut self) {
        // Group feedback by density
        let mut density_feedback: HashMap<u8, Vec<&EntityFeedback>> = HashMap::new();
        for feedback in self.feedback_cache.values() {
            let density_idx = feedback.current_density as u8;
            density_feedback.entry(density_idx).or_default().push(feedback);
        }
        
        // Adjust each attractor
        for (density_idx, attractor) in self.attractors.iter_mut() {
            if let Some(feedback_list) = density_feedback.get(density_idx) {
                if feedback_list.is_empty() {
                    continue;
                }
                
                // Compute average effectiveness
                let avg_effectiveness: Float = feedback_list.iter()
                    .map(|f| f.effectiveness)
                    .sum() / feedback_list.len() as Float;
                
                // Adjust strength based on effectiveness
                if avg_effectiveness > 0.7 {
                    attractor.strengthen(0.01);
                } else if avg_effectiveness < 0.3 {
                    attractor.weaken(0.01);
                }
                
                // Record average progress
                let avg_progress: Float = feedback_list.iter()
                    .map(|f| f.evolution_progress)
                    .sum() / feedback_list.len() as Float;
                attractor.record_feedback(avg_progress);
            }
        }
    }
    
    /// Get attractor for a density
    pub fn get_attractor(&self, density: Density) -> Option<&AdaptiveAttractor> {
        self.attractors.get(&(density as u8))
    }
    
    /// Get all attractors
    pub fn attractors(&self) -> &HashMap<u8, AdaptiveAttractor> {
        &self.attractors
    }
    
    /// Get statistics
    pub fn stats(&self) -> &GuidanceStats {
        &self.stats
    }
    
    /// Set guidance strength
    pub fn set_strength(&mut self, strength: Float) {
        self.guidance_strength = strength.clamp(0.0, 1.0);
    }
    
    /// Get guidance strength
    pub fn strength(&self) -> Float {
        self.guidance_strength
    }
    
    /// Clear feedback cache
    pub fn clear_feedback(&mut self) {
        self.feedback_cache.clear();
    }
    
    /// Get feedback for an entity
    pub fn get_feedback(&self, entity_id: &EntityId) -> Option<&EntityFeedback> {
        self.feedback_cache.get(entity_id)
    }
    
    /// Tick - process guidance updates
    pub fn tick(&mut self, _dt: Float) {
        // Periodic adjustment of attractors
        // In a real implementation, this would do more sophisticated adjustments
    }
}

impl Default for TeleologicalGuidance {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Get current time as Float
fn current_time() -> Float {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adaptive_attractor_creation() {
        let attractor = AdaptiveAttractor::for_density(Density::Third);
        assert_eq!(attractor.target_density, Density::Third);
        assert_eq!(attractor.current_strength, 0.5);
    }
    
    #[test]
    fn test_adaptive_attractor_pull() {
        let attractor = AdaptiveAttractor::for_density(Density::Third);
        
        // Positive progress should give positive pull
        let pull = attractor.compute_pull(0.8, 0.1);
        assert!(pull > 0.0);
        
        // Negative progress should give negative pull
        let pull = attractor.compute_pull(0.8, -0.1);
        assert!(pull < 0.0);
    }
    
    #[test]
    fn test_adaptive_attractor_adjustment() {
        let mut attractor = AdaptiveAttractor::for_density(Density::Third);
        let initial_strength = attractor.current_strength;
        
        attractor.strengthen(0.1);
        assert!(attractor.current_strength > initial_strength);
        
        attractor.weaken(0.1);
        assert!(attractor.current_strength < initial_strength + 0.1);
    }
    
    #[test]
    fn test_evolution_option() {
        let option = EvolutionOption::new("Test option".to_string(), 0.5)
            .with_density_progress(0.1)
            .with_polarity_shift(0.2);
        
        assert_eq!(option.description, "Test option");
        assert_eq!(option.probability, 0.5);
        assert_eq!(option.density_progress, 0.1);
        assert_eq!(option.polarity_shift, 0.2);
    }
    
    #[test]
    fn test_teleological_progress() {
        let progress = TeleologicalProgress::default_for_density(Density::Third);
        
        assert_eq!(progress.current_density, Density::Third);
        assert_eq!(progress.purpose_alignment, 0.5);
    }
    
    #[test]
    fn test_teleological_guidance_creation() {
        let guidance = TeleologicalGuidance::new();
        
        assert_eq!(guidance.attractors().len(), 8);
        assert_eq!(guidance.strength(), 0.3);
    }
    
    #[test]
    fn test_compute_guidance() {
        let guidance = TeleologicalGuidance::new();
        let teleological = TeleologicalProgress::default_for_density(Density::Third);
        
        let options = vec![
            EvolutionOption::new("Option A".to_string(), 0.3).with_density_progress(0.1),
            EvolutionOption::new("Option B".to_string(), 0.7).with_density_progress(-0.1),
        ];
        
        let guided = guidance.compute_guidance(&teleological, &options);
        
        assert_eq!(guided.len(), 2);
        
        // Both should have modified probabilities
        for (_, prob) in &guided {
            assert!(*prob >= 0.0);
        }
    }
    
    #[test]
    fn test_apply_guidance() {
        let guidance = TeleologicalGuidance::new();
        let teleological = TeleologicalProgress::default_for_density(Density::Third);
        
        let mut options = vec![
            EvolutionOption::new("Option A".to_string(), 0.5),
            EvolutionOption::new("Option B".to_string(), 0.5),
        ];
        
        guidance.apply(&teleological, &mut options);
        
        // Probabilities should be normalized
        let total: Float = options.iter().map(|o| o.probability).sum();
        assert!((total - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_receive_feedback() {
        let mut guidance = TeleologicalGuidance::new();
        let entity_id = EntityId::new(1);
        
        guidance.receive_feedback(
            entity_id.clone(),
            0.7,
            0.5,
            Density::Third,
            SpectrumPosition::new(1.0, Density::Third, 0.0),
        );
        
        assert_eq!(guidance.stats().total_feedback_received, 1);
        assert!(guidance.get_feedback(&entity_id).is_some());
    }
    
    #[test]
    fn test_guidance_strength() {
        let mut guidance = TeleologicalGuidance::new();
        
        guidance.set_strength(0.8);
        assert_eq!(guidance.strength(), 0.8);
        
        // Should be clamped
        guidance.set_strength(1.5);
        assert_eq!(guidance.strength(), 1.0);
        
        guidance.set_strength(-0.5);
        assert_eq!(guidance.strength(), 0.0);
    }
}
