//! Unified Tick Engine - Integrates Biology, Consciousness, and Social Systems
//!
//! This module provides the unified tick function that coordinates all simulation
//! systems: biology (cells, organisms), consciousness (archetypes, experience),
//! and social (relationships, civilization).

use crate::biology::{
    CellEngine, ConsciousnessTickEngine, EvolutionEngine, OrganismManager,
    ConsciousnessTickInput, ConsciousnessTickOutput, BodyExperience, 
    EnvironmentExperience, SocialExperience,
};
use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use crate::social::{
    SocialState, SocialTickContext, SocialTickResult,
};
use std::collections::HashMap;

/// Configuration for the unified tick engine
#[derive(Debug, Clone)]
pub struct UnifiedTickConfig {
    /// Whether biology is enabled
    pub biology_enabled: bool,
    
    /// Whether consciousness is enabled
    pub consciousness_enabled: bool,
    
    /// Whether social systems are enabled
    pub social_enabled: bool,
    
    /// Tick rate for biology (every N ticks)
    pub biology_tick_rate: u64,
    
    /// Tick rate for social systems (every N ticks)
    pub social_tick_rate: u64,
    
    /// Whether to run in debug mode
    pub debug_mode: bool,
}

impl Default for UnifiedTickConfig {
    fn default() -> Self {
        UnifiedTickConfig {
            biology_enabled: true,
            consciousness_enabled: true,
            social_enabled: true,
            biology_tick_rate: 1,
            social_tick_rate: 10,
            debug_mode: false,
        }
    }
}

/// The unified tick engine - coordinates all simulation subsystems
pub struct UnifiedTickEngine {
    /// Configuration
    config: UnifiedTickConfig,
    
    /// Cell engine - manages cellular biology
    cell_engine: CellEngine,
    
    /// Evolution engine - manages species and populations
    evolution_engine: EvolutionEngine,
    
    /// Organism manager - manages individual organisms
    organism_manager: OrganismManager,
    
    /// Consciousness engine - manages entity consciousness
    consciousness_engine: ConsciousnessTickEngine,
    
    /// Social state - manages relationships, civilization, SMC
    social_state: SocialState,
    
    /// Global tick counter
    tick_count: u64,
    
    /// Current global density
    global_density: Density,
}

impl Default for UnifiedTickEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedTickEngine {
    /// Create a new unified tick engine
    pub fn new() -> Self {
        UnifiedTickEngine {
            config: UnifiedTickConfig::default(),
            cell_engine: CellEngine::new(),
            evolution_engine: EvolutionEngine::new(),
            organism_manager: OrganismManager::new(),
            consciousness_engine: ConsciousnessTickEngine::new(),
            social_state: SocialState::new(),
            tick_count: 0,
            global_density: Density::Third,
        }
    }
    
    /// Create with custom config
    pub fn with_config(config: UnifiedTickConfig) -> Self {
        UnifiedTickEngine {
            config,
            cell_engine: CellEngine::new(),
            evolution_engine: EvolutionEngine::new(),
            organism_manager: OrganismManager::new(),
            consciousness_engine: ConsciousnessTickEngine::new(),
            social_state: SocialState::new(),
            tick_count: 0,
            global_density: Density::Third,
        }
    }
    
    /// Process a unified tick for all systems
    pub fn tick(&mut self, entity_ids: &[EntityId]) -> UnifiedTickResult {
        self.tick_count += 1;
        let mut result = UnifiedTickResult::default();
        
        // 1. Biology tick (every biology_tick_rate ticks)
        if self.config.biology_enabled && self.tick_count % self.config.biology_tick_rate == 0 {
            result.biology_result = self.process_biology_tick();
        }
        
        // 2. Consciousness tick (every tick)
        if self.config.consciousness_enabled {
            result.consciousness_result = self.process_consciousness_tick(entity_ids);
        }
        
        // 3. Social tick (every social_tick_rate ticks)
        if self.config.social_enabled && self.tick_count % self.config.social_tick_rate == 0 {
            result.social_result = self.process_social_tick(entity_ids);
        }
        
        // Update global density based on simulation
        self.update_global_density();
        
        result.tick_count = self.tick_count;
        result.global_density = self.global_density;
        
        result
    }
    
    /// Process biology systems (simplified)
    fn process_biology_tick(&mut self) -> BiologyTickResult {
        let mut result = BiologyTickResult::default();
        
        // Simplified biology processing - in production, would call actual engine methods
        // with proper environment parameters
        
        result
    }
    
    /// Process consciousness systems
    fn process_consciousness_tick(&mut self, entity_ids: &[EntityId]) -> ConsciousnessTickResult {
        let mut result = ConsciousnessTickResult::default();
        
        // Build consciousness inputs for each entity
        let mut inputs = HashMap::new();
        for &entity_id in entity_ids {
            let input = ConsciousnessTickInput {
                body_experience: BodyExperience {
                    health: 0.8,
                    energy: 0.7,
                    pain_level: 0.1,
                    pleasure_level: 0.3,
                },
                environment_experience: EnvironmentExperience {
                    temperature: 0.5,
                    danger_present: 0.2,
                    opportunity_present: 0.4,
                    novelty: 0.3,
                },
                social_experience: SocialExperience {
                    isolation: 0.3,
                    conflict: 0.1,
                    connection: 0.4,
                    love_given: 0.2,
                    love_received: 0.2,
                },
                dt: 1.0,
            };
            inputs.insert(entity_id.as_u64(), input);
        }
        
        // Process consciousness tick
        let outputs = self.consciousness_engine.tick(&inputs);
        
        // Convert outputs
        result.entity_states = outputs;
        result
    }
    
    /// Process social systems
    fn process_social_tick(&mut self, entity_ids: &[EntityId]) -> SocialTickResult {
        let context = SocialTickContext {
            tick: self.tick_count,
            active_entities: entity_ids.to_vec(),
            global_density: self.global_density,
            ticks_since_harvest: self.tick_count % 1000,
        };
        
        self.social_state.tick(&context)
    }
    
    /// Update global density based on simulation state
    fn update_global_density(&mut self) {
        // Simplified: density increases with tick count
        self.global_density = match self.tick_count {
            0..=10_000 => Density::Third,
            10_001..=50_000 => Density::Third,
            50_001..=100_000 => Density::Fourth,
            100_001..=200_000 => Density::Fifth,
            _ => Density::Sixth,
        };
    }
    
    /// Register a new entity with all systems
    pub fn register_entity(&mut self, entity_id: EntityId, density: u8, initial_st_ratio: f64) {
        // Register with consciousness engine
        self.consciousness_engine.register_entity(
            entity_id.as_u64(),
            density,
            initial_st_ratio,
        );
        
        if self.config.debug_mode {
            println!("Registered entity {:?} at density {} with ST ratio {}", 
                entity_id, density, initial_st_ratio);
        }
    }
    
    /// Get current tick count
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
    
    /// Get current global density
    pub fn global_density(&self) -> Density {
        self.global_density
    }
}

/// Result of a unified tick
#[derive(Debug, Default)]
pub struct UnifiedTickResult {
    pub tick_count: u64,
    pub global_density: Density,
    pub biology_result: BiologyTickResult,
    pub consciousness_result: ConsciousnessTickResult,
    pub social_result: SocialTickResult,
}

/// Result of biology tick
#[derive(Debug, Default)]
pub struct BiologyTickResult {
    pub evolution_changes: Vec<EvoChange>,
    pub organism_updates: OrganismUpdates,
}

impl BiologyTickResult {
    pub fn has_changes(&self) -> bool {
        !self.evolution_changes.is_empty() || 
        self.organism_updates.born > 0 ||
        self.organism_updates.died > 0
    }
}

/// Evolution change event
#[derive(Debug, Clone)]
pub struct EvoChange {
    pub change_type: EvoChangeType,
    pub species_id: u64,
    pub details: String,
}

/// Type of evolution change
#[derive(Debug, Clone)]
pub enum EvoChangeType {
    Speciation,
    Extinction,
    PopulationShift,
}

/// Organism update statistics
#[derive(Debug, Default)]
pub struct OrganismUpdates {
    pub born: usize,
    pub died: usize,
    pub matured: usize,
    pub reproduced: usize,
}

/// Result of consciousness tick
#[derive(Debug, Default)]
pub struct ConsciousnessTickResult {
    pub entity_states: HashMap<u64, ConsciousnessTickOutput>,
}

impl ConsciousnessTickResult {
    pub fn state_count(&self) -> usize {
        self.entity_states.len()
    }
}

#[cfg(test)]
mod unified_tick_tests {
    use super::*;

    #[test]
    fn test_unified_tick_engine_creation() {
        let engine = UnifiedTickEngine::new();
        assert_eq!(engine.tick_count(), 0);
    }

    #[test]
    fn test_tick_increments() {
        let mut engine = UnifiedTickEngine::new();
        let entity_ids = vec![EntityId::new("test1".to_string())];
        
        engine.tick(&entity_ids);
        assert_eq!(engine.tick_count(), 1);
        
        engine.tick(&entity_ids);
        assert_eq!(engine.tick_count(), 2);
    }

    #[test]
    fn test_global_density_updates() {
        let mut engine = UnifiedTickEngine::new();
        let entity_ids = vec![EntityId::new("test1".to_string())];
        
        // Initial tick
        engine.tick(&entity_ids);
        assert_eq!(engine.global_density(), Density::Third);
    }
}
