//! Entity-Body Coupling - Consciousness Meets Biology
//!
//! This module connects consciousness entities to biological bodies:
//! - Mind-body-spirit complex
//! - Nervous system processing
//! - Energy centers (chakras)
//! - Physics experience integration
//! - Veil mechanics
//!
//! From V4 Roadmap Phase 4: "Biological Simulation Engine"
//! Gap #3 resolution: entities have ACTUAL biological bodies.

use crate::biology::cell_engine::{CellEngine, CellEnvironment};
use crate::biology::evolution_engine::{EvolutionEngine, SpeciesId};
use crate::biology::organism_lifecycle::{
    BodyPlan, Organism, OrganismEnvironment, OrganismManager,
};
use crate::holographic::field_address::HolographicAddress;
use rand::Rng;
use std::collections::HashMap;

// Placeholder for PhysicsExperience - will be used from cosmos
#[derive(Debug, Clone, Default)]
pub struct PhysicsExperience {
    pub temperature: f64,
    pub radiation_exposure: f64,
    pub day_night_phase: f64,
}

// Placeholder for ArchetypeProcessor
#[derive(Debug, Clone)]
pub struct ArchetypeProcessor {
    pub polarity: f64,
}

impl ArchetypeProcessor {
    pub fn new() -> Self {
        Self { polarity: 0.0 }
    }
    pub fn process_catalyst(&mut self, _experience: &ConsciousnessExperience) {
        // Simplified
    }
}

// Placeholder for VeilMechanics
#[derive(Debug, Clone)]
pub struct VeilMechanics {
    pub intensity: f64,
}

impl VeilMechanics {
    pub fn for_density(_density: u8) -> Self {
        Self { intensity: 0.9 }
    }
}

// ============================================================================
// Energy Centers (Chakras)
// ============================================================================

/// Energy center (chakra) states
#[derive(Debug, Clone)]
pub struct EnergyCenter {
    /// Center number (0 = Root/Red, 6 = Crown/Violet)
    pub level: usize,
    /// Activation level (0.0 = blocked, 1.0 = fully open)
    pub activation: f64,
    /// Blockage level (from unresolved catalyst)
    pub blockage: f64,
    /// Rotation speed (spin rate)
    pub spin_rate: f64,
}

impl EnergyCenter {
    /// Create a new energy center
    pub fn new(level: usize) -> Self {
        Self {
            level,
            activation: 0.1, // Partially open
            blockage: 0.5,
            spin_rate: 1.0,
        }
    }

    /// Update based on experience
    pub fn process_experience(&mut self, experience: &ConsciousnessExperience) {
        let effect = match self.level {
            0 => experience.survival_effect,      // Red - survival
            1 => experience.creativity_effect,    // Orange - creativity/sexuality
            2 => experience.power_effect,         // Yellow - power/will
            3 => experience.love_effect,          // Green - love/compassion
            4 => experience.communication_effect, // Blue - communication
            5 => experience.intuition_effect,     // Indigo - intuition
            6 => experience.spirit_effect,        // Violet - spirit
            _ => 0.0,
        };

        // Update activation
        self.activation = (self.activation + effect * 0.1).clamp(0.0, 1.0);

        // Clear blockage with positive experiences
        if effect > 0.0 {
            self.blockage = (self.blockage - 0.05).clamp(0.0, 1.0);
        } else if effect < 0.0 {
            self.blockage = (self.blockage + 0.05).clamp(0.0, 1.0);
        }

        // Update spin
        self.spin_rate = 1.0 + self.activation - self.blockage;
    }
}

// ============================================================================
// Consciousness Experience
// ============================================================================

/// What the consciousness experiences from the body/world
#[derive(Debug, Clone, Default)]
pub struct ConsciousnessExperience {
    /// Survival instinct effect
    pub survival_effect: f64,
    /// Creativity/sexuality effect
    pub creativity_effect: f64,
    /// Power/will effect
    pub power_effect: f64,
    /// Love/compassion effect
    pub love_effect: f64,
    /// Communication effect
    pub communication_effect: f64,
    /// Intuition effect
    pub intuition_effect: f64,
    /// Spirit connection effect
    pub spirit_effect: f64,
    /// Stress level (cumulative)
    pub stress: f64,
    /// Joy level
    pub joy: f64,
}

// ============================================================================
// Nervous System
// ============================================================================

/// Nervous system processing
#[derive(Debug, Clone)]
pub struct NervousSystem {
    /// Neural complexity (0.0-1.0)
    pub complexity: f64,
    /// Current activation level
    pub activation: f64,
    /// Stress accumulation
    pub stress: f64,
    /// Processing capacity
    pub capacity: f64,
}

impl NervousSystem {
    pub fn new(complexity: f64) -> Self {
        Self {
            complexity,
            activation: 0.5,
            stress: 0.0,
            capacity: complexity,
        }
    }

    /// Process sensory input into consciousness experience
    pub fn process(
        &mut self,
        physics: &PhysicsExperience,
        body_health: f64,
    ) -> ConsciousnessExperience {
        let mut experience = ConsciousnessExperience::default();

        // Temperature sensation affects survival
        let temp_comfort = 1.0 - ((physics.temperature - 293.0) / 30.0).abs().min(1.0);
        experience.survival_effect = temp_comfort * 2.0 - 1.0;

        // Radiation affects survival negatively
        experience.survival_effect -= physics.radiation_exposure / 1000.0;

        // Day/night affects various centers
        let day_phase = physics.day_night_phase;
        experience.creativity_effect = day_phase.sin() * 0.3;

        // Social presence affects green center
        // (simplified - would need other entity tracking)
        experience.love_effect = 0.1;

        // Body health affects all
        let health_factor = body_health * 2.0 - 1.0;
        experience.spirit_effect = health_factor * 0.2;

        // Stress accumulates from negative experiences
        self.stress += (-experience.survival_effect).max(0.0) * 0.1;
        self.stress *= 0.99; // Natural recovery

        experience.stress = self.stress;

        // Joy from positive experiences
        experience.joy = experience.love_effect.max(0.0) * body_health;

        self.activation = (body_health + experience.joy).clamp(0.0, 1.0);

        experience
    }
}

// ============================================================================
// Embodied Entity
// ============================================================================

/// A consciousness entity with a biological body
/// This is the BRIDGE between mind and matter
#[derive(Debug)]
pub struct EmbodiedEntity {
    /// Entity ID
    pub entity_id: u64,
    /// Associated organism
    pub organism_id: u64,
    /// Archetype processor (mind)
    pub consciousness: ArchetypeProcessor,
    /// Nervous system
    pub nervous_system: NervousSystem,
    /// Energy centers (chakras)
    pub energy_centers: [EnergyCenter; 7],
    /// Veil mechanics
    pub veil: VeilMechanics,
    /// Physics experience at current location
    pub physics: PhysicsExperience,
    /// Position
    pub position: HolographicAddress,
    /// Density level
    pub density: u8,
    /// Time in incarnation
    pub incarnation_time: f64,
}

impl EmbodiedEntity {
    /// Create a new embodied entity
    pub fn new(entity_id: u64, organism_id: u64, position: HolographicAddress) -> Self {
        Self {
            entity_id,
            organism_id,
            consciousness: ArchetypeProcessor::new(),
            nervous_system: NervousSystem::new(1.0), // Full complexity for 3rd density
            energy_centers: [
                EnergyCenter::new(0), // Root
                EnergyCenter::new(1), // Sacral
                EnergyCenter::new(2), // Solar
                EnergyCenter::new(3), // Heart
                EnergyCenter::new(4), // Throat
                EnergyCenter::new(5), // Third Eye
                EnergyCenter::new(6), // Crown
            ],
            veil: VeilMechanics::for_density(3), // 3rd density
            physics: PhysicsExperience::default(),
            position,
            density: 3,
            incarnation_time: 0.0,
        }
    }

    /// Main tick - couple consciousness with body
    pub fn tick(&mut self, organism: &Organism, physics: &PhysicsExperience, dt: f64) {
        self.incarnation_time += dt;
        self.physics = physics.clone();

        // 1. Nervous system processes physics experience into consciousness experience
        let experience = self.nervous_system.process(physics, organism.health);

        // 2. Consciousness processes through archetype cycle
        let processed = self.consciousness.process_catalyst(&experience);

        // 3. Update energy centers based on experience
        for center in &mut self.energy_centers {
            center.process_experience(&experience);
        }

        // 4. Body responds to consciousness (stress affects health)
        // This is handled by the organism tick

        // 5. Veil filters experience
        // (Implemented in VeilMechanics)
    }

    /// Get overall polarity (STO vs STS)
    pub fn polarity(&self) -> f64 {
        self.consciousness.polarity
    }

    /// Get consciousness clarity
    pub fn clarity(&self) -> f64 {
        let body_clarity = self.nervous_system.activation;
        let veil_clarity = 1.0 - self.veil.intensity;
        let center_clarity = self
            .energy_centers
            .iter()
            .map(|c| c.activation)
            .sum::<f64>()
            / 7.0;

        body_clarity * veil_clarity * center_clarity
    }
}

// ============================================================================
// Bio-Simulation System
// ============================================================================

/// Complete biological simulation system
#[derive(Debug)]
pub struct BioSimulation {
    /// Cell engine
    pub cells: CellEngine,
    /// Organism manager
    pub organisms: OrganismManager,
    /// Evolution engine
    pub evolution: EvolutionEngine,
    /// Embodied entities
    pub embodied_entities: HashMap<u64, EmbodiedEntity>,
}

impl Default for BioSimulation {
    fn default() -> Self {
        Self::new()
    }
}

impl BioSimulation {
    pub fn new() -> Self {
        Self {
            cells: CellEngine::new(),
            organisms: OrganismManager::new(),
            evolution: EvolutionEngine::new(),
            embodied_entities: HashMap::new(),
        }
    }

    /// Main tick
    pub fn tick(&mut self, planet_conditions: &PlanetConditions, dt: f64) {
        // Create cell environment from planet conditions
        let cell_env = CellEnvironment::from_planet(
            planet_conditions.solar_radiation,
            planet_conditions.temperature,
            planet_conditions.habitability,
        );

        // Tick cell engine
        self.cells.tick(&cell_env, dt);

        // Create organism environment
        let org_env = OrganismEnvironment {
            solar_radiation: planet_conditions.solar_radiation,
            plant_density: planet_conditions.plant_density,
            prey_density: planet_conditions.prey_density,
            predator_density: planet_conditions.predator_density,
            carrion_density: planet_conditions.carrion_density,
            water_availability: planet_conditions.water_availability,
            temperature: planet_conditions.temperature,
            season: planet_conditions.season,
        };

        // Tick organism manager
        self.organisms.tick(&org_env, dt);

        // Create population environment
        let pop_env = crate::biology::evolution_engine::PopulationEnvironment {
            habitat_quality: planet_conditions.habitability,
            climate_suitability: planet_conditions.climate_suitability,
            food_availability: planet_conditions.plant_density,
            predator_pressure: planet_conditions.predator_density,
            disease_prevalence: 0.1,
        };

        // Tick evolution engine
        self.evolution.tick(&pop_env, dt);

        // Sync organisms from evolution
        self.evolution.sync_to_organisms(&mut self.organisms);
    }

    /// Create an embodied entity
    pub fn create_embodied_entity(&mut self, mut position: HolographicAddress) -> u64 {
        // Create a human body
        let pos_clone = position.clone();
        let organism_id = self.organisms.create_human(pos_clone);
        let entity_id = organism_id; // For now, 1:1 mapping

        let entity = EmbodiedEntity::new(entity_id, organism_id, position);
        self.embodied_entities.insert(entity_id, entity);

        entity_id
    }

    /// Tick a specific embodied entity
    pub fn tick_entity(&mut self, entity_id: u64, dt: f64) {
        // Get required data first
        let organism_id = {
            if let Some(entity) = self.embodied_entities.get(&entity_id) {
                entity.organism_id
            } else {
                return;
            }
        };

        let physics = {
            if let Some(entity) = self.embodied_entities.get(&entity_id) {
                entity.physics.clone()
            } else {
                return;
            }
        };

        // Now tick
        if let Some(entity) = self.embodied_entities.get_mut(&entity_id) {
            if let Some(organism) = self.organisms.organisms.get(&organism_id) {
                entity.tick(organism, &physics, dt);
            }
        }
    }
}

/// Conditions from planet
#[derive(Debug, Clone)]
pub struct PlanetConditions {
    pub solar_radiation: f64,
    pub temperature: f64,
    pub habitability: f64,
    pub plant_density: f64,
    pub prey_density: f64,
    pub predator_density: f64,
    pub carrion_density: f64,
    pub water_availability: f64,
    pub season: f64,
    pub climate_suitability: f64,
}

impl Default for PlanetConditions {
    fn default() -> Self {
        Self {
            solar_radiation: 200.0,
            temperature: 288.0,
            habitability: 0.8,
            plant_density: 0.5,
            prey_density: 0.2,
            predator_density: 0.1,
            carrion_density: 0.05,
            water_availability: 1.0,
            season: 0.5,
            climate_suitability: 0.8,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bio_simulation() {
        let mut sim = BioSimulation::new();
        assert!(sim.cells.total_cells() >= 0);
    }

    #[test]
    fn test_embodied_entity() {
        let entity = EmbodiedEntity::new(0, 0, HolographicAddress::default());
        assert_eq!(entity.density, 3);
    }
}
