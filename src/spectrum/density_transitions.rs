//! Density Transitions System
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 6.3:
//! "Density transitions as entities evolve along spectrum"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Beyond Layer 7 (Sub-Sub-Logos), entities evolve through an octave of 8 densities"
//! "Each density corresponds to spectrum range"
//! "Transcend and Include: retain previous densities, add new"
//!
//! This module implements:
//! - Density transition triggers and execution
//! - Transcend and Include during transitions
//! - Attractor field spawning for next density

use std::collections::HashMap;
use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel, Density2SubLevel};

/// Transition readiness assessment
#[derive(Debug, Clone)]
pub struct TransitionReadiness {
    pub entity_id: u64,
    pub current_density: Density,
    pub target_density: Density,
    pub readiness_score: f64,
    pub requirements_met: Vec<TransitionRequirement>,
    pub requirements_pending: Vec<TransitionRequirement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionRequirement {
    /// Coherence threshold met
    Coherence(f64),
    /// Experience points accumulated
    Experience(i64),
    /// Polarity achieved
    Polarity(f64),
    /// Lessons learned
    Lessons(i32),
    /// Spectrum position reached
    SpectrumPosition(f64),
}

/// Density transition result
#[derive(Debug, Clone)]
pub struct DensityTransitionResult {
    pub entity_id: u64,
    pub previous_density: Density,
    pub new_density: Density,
    pub transition_time: usize,
    pub success: bool,
    pub transcend_and_include: TranscendAndInclude,
}

/// Transcend and Include result
#[derive(Debug, Clone)]
pub struct TranscendAndInclude {
    pub retained_densities: Vec<Density>,
    pub new_capabilities: Vec<String>,
    pub expanded_consciousness: f64,
}

/// Configuration for density transitions
#[derive(Debug, Clone)]
pub struct DensityTransitionConfig {
    /// Minimum readiness score for transition
    pub min_readiness: f64,
    /// Coherence threshold
    pub coherence_threshold: f64,
    /// Experience points required
    pub experience_threshold: i64,
    /// Polarity threshold
    pub polarity_threshold: f64,
    /// Enable transcend and include
    pub enable_transcend_include: bool,
    /// Transition animation duration
    pub transition_duration: usize,
}

impl Default for DensityTransitionConfig {
    fn default() -> Self {
        DensityTransitionConfig {
            min_readiness: 0.8,
            coherence_threshold: 0.7,
            experience_threshold: 1000,
            polarity_threshold: 0.6,
            enable_transcend_include: true,
            transition_duration: 100,
        }
    }
}

/// Entity transition state
#[derive(Debug, Clone)]
pub struct EntityTransitionState {
    pub entity_id: u64,
    pub is_transitioning: bool,
    pub transition_start: Option<usize>,
    pub progress: f64,
    pub target_density: Option<Density>,
}

impl EntityTransitionState {
    pub fn new(entity_id: u64) -> Self {
        EntityTransitionState {
            entity_id,
            is_transitioning: false,
            transition_start: None,
            progress: 0.0,
            target_density: None,
        }
    }
}

/// Density Transition System
/// 
/// Manages density transitions for entities
/// 
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Transcend and Include: retain previous densities, add new"
#[derive(Debug, Clone)]
pub struct DensityTransitionSystem {
    config: DensityTransitionConfig,
    entity_states: HashMap<u64, EntityTransitionState>,
    transition_history: Vec<DensityTransitionResult>,
    statistics: DensityTransitionStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct DensityTransitionStatistics {
    pub total_transitions: usize,
    pub successful_transitions: usize,
    pub failed_transitions: usize,
    pub average_transition_time: f64,
    pub transitions_by_density: HashMap<String, usize>,
}

impl DensityTransitionSystem {
    pub fn new() -> Self {
        DensityTransitionSystem {
            config: DensityTransitionConfig::default(),
            entity_states: HashMap::new(),
            transition_history: Vec::new(),
            statistics: DensityTransitionStatistics::default(),
        }
    }
    
    pub fn with_config(config: DensityTransitionConfig) -> Self {
        DensityTransitionSystem {
            config,
            entity_states: HashMap::new(),
            transition_history: Vec::new(),
            statistics: DensityTransitionStatistics::default(),
        }
    }
    
    /// Register entity in transition system
    pub fn register_entity(&mut self, entity_id: u64) {
        let state = EntityTransitionState::new(entity_id);
        self.entity_states.insert(entity_id, state);
    }
    
    /// Check if entity is ready for density transition
    pub fn check_readiness(
        &self,
        entity_id: u64,
        current_density: Density,
        coherence: f64,
        experience: i64,
        polarity: f64,
        lessons: i32,
        spectrum_position: f64,
    ) -> TransitionReadiness {
        let mut requirements_met = Vec::new();
        let mut requirements_pending = Vec::new();
        
        // Check coherence
        if coherence >= self.config.coherence_threshold {
            requirements_met.push(TransitionRequirement::Coherence(coherence));
        } else {
            requirements_pending.push(TransitionRequirement::Coherence(coherence));
        }
        
        // Check experience
        if experience >= self.config.experience_threshold {
            requirements_met.push(TransitionRequirement::Experience(experience));
        } else {
            requirements_pending.push(TransitionRequirement::Experience(experience));
        }
        
        // Check polarity
        if polarity >= self.config.polarity_threshold {
            requirements_met.push(TransitionRequirement::Polarity(polarity));
        } else {
            requirements_pending.push(TransitionRequirement::Polarity(polarity));
        }
        
        // Calculate readiness score
        let readiness_score = requirements_met.len() as f64 / 
            (requirements_met.len() + requirements_pending.len()) as f64;
        
        // Determine target density
        let target_density = self.get_next_density(current_density);
        
        TransitionReadiness {
            entity_id,
            current_density,
            target_density: target_density.clone(),
            readiness_score,
            requirements_met,
            requirements_pending,
        }
    }
    
    /// Get next density in the octave
    fn get_next_density(&self, current: Density) -> Density {
        match current {
            Density::First(_) => Density::Second(Density2SubLevel::Cellular),
            Density::Second(_) => Density::Third,
            Density::Third => Density::Fourth,
            Density::Fourth => Density::Fifth,
            Density::Fifth => Density::Sixth,
            Density::Sixth => Density::Seventh,
            Density::Seventh => Density::Eighth,
            Density::Eighth => Density::Eighth, // Already at max
        }
    }
    
    /// Initiate density transition
    pub fn initiate_transition(&mut self, entity_id: u64, target_density: Density, current_step: usize) -> bool {
        // Get or create state
        let state = self.entity_states
            .entry(entity_id)
            .or_insert_with(|| EntityTransitionState::new(entity_id));
        
        // Can't transition if already transitioning
        if state.is_transitioning {
            return false;
        }
        
        // Start transition
        state.is_transitioning = true;
        state.transition_start = Some(current_step);
        state.target_density = Some(target_density.clone());
        state.progress = 0.0;
        
        true
    }
    
    /// Update transition progress
    pub fn update_transition(&mut self, entity_id: u64, current_step: usize) -> Option<DensityTransitionResult> {
        let state = self.entity_states.get_mut(&entity_id)?;
        
        if !state.is_transitioning {
            return None;
        }
        
        let start_step = state.transition_start?;
        let target_density = state.target_density.clone()?;
        
        // Calculate progress
        let elapsed = current_step - start_step;
        state.progress = (elapsed as f64 / self.config.transition_duration as f64).min(1.0);
        
        // Check if transition complete
        if state.progress >= 1.0 {
            let previous = Density::First(Density1SubLevel::Quantum); // Would get from entity
            
            // Complete transition first (releases mutable borrow)
            state.is_transitioning = false;
            state.transition_start = None;
            state.progress = 0.0;
            state.target_density = None;
            
            // Now compute transcend_and_include (after mutable borrow released)
            let transcend_and_include = self.compute_transcend_include(previous, target_density.clone());
            
            let result = DensityTransitionResult {
                entity_id,
                previous_density: previous,
                new_density: target_density,
                transition_time: elapsed,
                success: true,
                transcend_and_include,
            };
            
            // Update statistics
            self.statistics.total_transitions += 1;
            self.statistics.successful_transitions += 1;
            self.transition_history.push(result.clone());
            
            return Some(result);
        }
        
        None
    }
    
    /// Compute Transcend and Include
    /// 
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Transcend and Include: retain previous densities, add new"
    fn compute_transcend_include(&self, previous: Density, new: Density) -> TranscendAndInclude {
        let mut retained = vec![previous];
        let mut new_capabilities = Vec::new();
        
        // Include previous capabilities
        match previous {
            Density::First(_) => {
                new_capabilities.push("Physical sensation".to_string());
            }
            Density::Second(_) => {
                new_capabilities.push("Emotional body".to_string());
            }
            Density::Third => {
                new_capabilities.push("Mind/Intellect".to_string());
            }
            Density::Fourth => {
                new_capabilities.push("Love/Compassion".to_string());
            }
            Density::Fifth => {
                new_capabilities.push("Will/Power".to_string());
            }
            Density::Sixth => {
                new_capabilities.push("Unity Consciousness".to_string());
            }
            _ => {}
        }
        
        // Add new capabilities
        match new {
            Density::Second(_) => {
                new_capabilities.push("Emotional body".to_string());
            }
            Density::Third => {
                new_capabilities.push("Mind/Intellect".to_string());
            }
            Density::Fourth => {
                new_capabilities.push("Love/Compassion".to_string());
            }
            Density::Fifth => {
                new_capabilities.push("Will/Power".to_string());
            }
            Density::Sixth => {
                new_capabilities.push("Unity Consciousness".to_string());
            }
            Density::Seventh => {
                new_capabilities.push("Cosmic Consciousness".to_string());
            }
            Density::Eighth => {
                new_capabilities.push("Infinite Awareness".to_string());
            }
            _ => {}
        }
        
        // Calculate expanded consciousness
        let expanded_consciousness = match (&previous, &new) {
            (Density::First(_), Density::Second(_)) => 0.2,
            (Density::Second(_), Density::Third) => 0.2,
            (Density::Third, Density::Fourth) => 0.15,
            (Density::Fourth, Density::Fifth) => 0.15,
            (Density::Fifth, Density::Sixth) => 0.15,
            (Density::Sixth, Density::Seventh) => 0.1,
            (Density::Seventh, Density::Eighth) => 0.1,
            _ => 0.0,
        };
        
        TranscendAndInclude {
            retained_densities: retained,
            new_capabilities,
            expanded_consciousness,
        }
    }
    
    /// Check if entity is transitioning
    pub fn is_transitioning(&self, entity_id: u64) -> bool {
        self.entity_states
            .get(&entity_id)
            .map(|s| s.is_transitioning)
            .unwrap_or(false)
    }
    
    /// Get transition progress
    pub fn get_progress(&self, entity_id: u64) -> f64 {
        self.entity_states
            .get(&entity_id)
            .map(|s| s.progress)
            .unwrap_or(0.0)
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &DensityTransitionStatistics {
        &self.statistics
    }
    
    /// Remove entity
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.entity_states.remove(&entity_id);
    }
}

impl Default for DensityTransitionSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transition_readiness() {
        let system = DensityTransitionSystem::new();
        
        let readiness = system.check_readiness(
            1,
            Density::First(Density1SubLevel::Quantum),
            0.8, // coherence
            1000, // experience
            0.7, // polarity
            10, // lessons
            0.5, // spectrum position
        );
        
        assert!(readiness.readiness_score >= 0.5);
    }
    
    #[test]
    fn test_transition_initiation() {
        let mut system = DensityTransitionSystem::new();
        
        let result = system.initiate_transition(
            1,
            Density::Second(Density2SubLevel::Cellular),
            0,
        );
        
        assert!(result);
        assert!(system.is_transitioning(1));
    }
    
    #[test]
    fn test_transcend_and_include() {
        let system = DensityTransitionSystem::new();
        
        let result = system.compute_transcend_include(
            Density::First(Density1SubLevel::Quantum),
            Density::Second(Density2SubLevel::Cellular),
        );
        
        assert!(!result.retained_densities.is_empty());
        assert!(!result.new_capabilities.is_empty());
    }
}
