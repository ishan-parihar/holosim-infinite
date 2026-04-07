//! Organism Lifecycle - Multicellular Organisms
//!
//! This module implements multicellular organisms with:
//! - Body plans and morphology
//! - Growth and development
//! - Behavior and metabolism
//! - Reproduction
//! - Aging and death
//!
//! From V4 Roadmap Phase 4: "Biological Simulation Engine"
//! Organisms are actual collections of cells with coherent behavior.

use crate::biology::cell_engine::CellId;
use crate::holographic::field_address::HolographicAddress;
use rand::Rng;
use std::collections::HashMap;

// ============================================================================
// Constants
// ============================================================================

/// Age at which organism reaches sexual maturity (simulation time units)
const MATURITY_AGE: f64 = 100.0;

/// Maximum organism lifespan (simulation time units)
const MAX_LIFESPAN: f64 = 1000.0;

/// Energy reserve multiplier (body_mass * this = max energy store)
const ENERGY_RESERVE_MULTIPLIER: f64 = 100.0;

// ============================================================================
// Body Plan
// ============================================================================

/// Body symmetry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodySymmetry {
    /// Radial symmetry (starfish, flowers)
    Radial,
    /// Bilateral symmetry (humans, mammals)
    Bilateral,
    /// No clear symmetry
    Asymmetric,
}

/// Locomotion type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionType {
    /// Fixed in place
    Sessile,
    /// Crawling/walking
    Ambulatory,
    /// Swimming
    Aquatic,
    /// Flying
    Aerial,
    /// Burrowing
    Fossorial,
}

/// Diet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DietType {
    /// Plants (photosynthetic)
    Autotrophic,
    /// Herbivore
    Herbivore,
    /// Carnivore
    Carnivore,
    /// Both plants and animals
    Omnivore,
    /// Decomposer
    Decomposer,
}

/// Social structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialStructure {
    /// Solitary
    Solitary,
    /// Pair-bonded
    Pair,
    /// Family group
    Family,
    /// Herd/flock
    Group,
    /// Complex society
    Society,
}

// ============================================================================
// Body Plan
// ============================================================================

/// Body plan defining organism morphology
#[derive(Debug, Clone)]
pub struct BodyPlan {
    /// Body symmetry
    pub symmetry: BodySymmetry,
    /// How the organism moves
    pub locomotion: LocomotionType,
    /// Diet type
    pub diet: DietType,
    /// Social structure
    pub social: SocialStructure,
    /// Body mass (kg)
    pub mass: f64,
    /// Characteristic body size (meters)
    pub size: f64,
    /// Number of limbs
    pub limb_count: u32,
    /// Has nervous system
    pub has_nervous_system: bool,
    /// Neural complexity (0.0-1.0)
    pub neural_complexity: f64,
    /// Has eyes (visual capability)
    pub has_eyes: bool,
    /// Has hands/appendages for manipulation
    pub has_manipulators: bool,
    /// Metabolic rate (basal)
    pub basal_metabolic_rate: f64,
    /// Gestation period (for viviparous)
    pub gestation_period: f64,
    /// Litter size
    pub litter_size: u32,
}

impl BodyPlan {
    /// Create a simple animal body plan
    pub fn simple_animal(mass: f64) -> Self {
        Self {
            symmetry: BodySymmetry::Bilateral,
            locomotion: LocomotionType::Ambulatory,
            diet: DietType::Herbivore,
            social: SocialStructure::Solitary,
            mass,
            size: mass.powf(0.33), // Approximate size from mass
            limb_count: 4,
            has_nervous_system: true,
            neural_complexity: 0.3,
            has_eyes: true,
            has_manipulators: false,
            basal_metabolic_rate: 70.0 * mass.powf(0.75), // Kleiber's law
            gestation_period: 30.0,
            litter_size: 1,
        }
    }

    /// Create a human body plan
    pub fn human() -> Self {
        Self {
            symmetry: BodySymmetry::Bilateral,
            locomotion: LocomotionType::Ambulatory,
            diet: DietType::Omnivore,
            social: SocialStructure::Society,
            mass: 70.0,
            size: 1.7,
            limb_count: 4,
            has_nervous_system: true,
            neural_complexity: 1.0,
            has_eyes: true,
            has_manipulators: true,
            basal_metabolic_rate: 1500.0, // Watts
            gestation_period: 270.0,
            litter_size: 1,
        }
    }

    /// Create a plant body plan
    pub fn plant(size: f64) -> Self {
        Self {
            symmetry: BodySymmetry::Radial,
            locomotion: LocomotionType::Sessile,
            diet: DietType::Autotrophic,
            social: SocialStructure::Solitary,
            mass: size.powf(3.0) * 1000.0, // Approximate mass from size
            size,
            limb_count: 0,
            has_nervous_system: false,
            neural_complexity: 0.0,
            has_eyes: false,
            has_manipulators: false,
            basal_metabolic_rate: size * 10.0,
            gestation_period: 0.0,
            litter_size: 100,
        }
    }
}

// ============================================================================
// Behavior State
// ============================================================================

/// Current behavior state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorState {
    /// Resting/sleeping
    Resting,
    /// Foraging for food
    Foraging,
    /// Actively hunting (carnivore)
    Hunting,
    /// Fleeing from threat
    Fleeing,
    /// Seeking mate
    SeekingMate,
    /// Courting/mating
    Mating,
    /// Caring for offspring
    Parenting,
    /// Territorial display
    Territorial,
    /// Migrating
    Migrating,
    /// Exploring
    Exploring,
    /// Dead
    Dead,
}

// ============================================================================
// Main Organism Structure
// ============================================================================

/// A multicellular organism
#[derive(Debug, Clone)]
pub struct Organism {
    /// Unique identifier
    pub id: u64,
    /// Species identifier
    pub species_id: u64,
    /// Cell IDs that make up this organism
    pub cell_ids: Vec<CellId>,
    /// Body plan
    pub body_plan: BodyPlan,
    /// Current behavior
    pub behavior: BehaviorState,
    /// Age
    pub age: f64,
    /// Energy reserves (fat/glucose)
    pub energy: f64,
    /// Health (0.0-1.0)
    pub health: f64,
    /// Sexually mature
    pub is_mature: bool,
    /// Currently pregnant
    pub pregnant: bool,
    /// Pregnancy progress
    pub pregnancy_progress: f64,
    /// Offspring count produced
    pub offspring_count: u32,
    /// Position in field
    pub position: HolographicAddress,
    /// Velocity (for moving organisms)
    pub velocity: (f64, f64),
    /// Target position (for goal-directed movement)
    pub target: Option<HolographicAddress>,
    /// Fitness score
    pub fitness: f64,
    /// Last behavior change time
    pub last_behavior_change: f64,
}

impl Organism {
    /// Create a new organism
    pub fn new(
        id: u64,
        species_id: u64,
        body_plan: BodyPlan,
        position: HolographicAddress,
    ) -> Self {
        let energy = body_plan.mass * ENERGY_RESERVE_MULTIPLIER;

        Self {
            id,
            species_id,
            cell_ids: Vec::new(),
            body_plan,
            behavior: BehaviorState::Resting,
            age: 0.0,
            energy,
            health: 1.0,
            is_mature: false,
            pregnant: false,
            pregnancy_progress: 0.0,
            offspring_count: 0,
            position,
            velocity: (0.0, 0.0),
            target: None,
            fitness: 0.5,
            last_behavior_change: 0.0,
        }
    }

    /// Create a simple animal
    pub fn simple_animal(
        id: u64,
        species_id: u64,
        mass: f64,
        position: HolographicAddress,
    ) -> Self {
        Self::new(id, species_id, BodyPlan::simple_animal(mass), position)
    }

    /// Create a human
    pub fn human(id: u64, position: HolographicAddress) -> Self {
        Self::new(id, 0, BodyPlan::human(), position)
    }

    /// Create a plant
    pub fn plant(id: u64, species_id: u64, size: f64, position: HolographicAddress) -> Self {
        Self::new(id, species_id, BodyPlan::plant(size), position)
    }

    /// Main tick - metabolism, behavior, aging
    pub fn tick(&mut self, environment: &OrganismEnvironment, dt: f64) {
        // Skip dead organisms
        if matches!(self.behavior, BehaviorState::Dead) {
            return;
        }

        self.age += dt;

        // Check maturity
        if !self.is_mature && self.age > MATURITY_AGE {
            self.is_mature = true;
        }

        // Metabolism
        self.metabolize(environment, dt);

        // Behavior decision
        self.decide_behavior(environment);

        // Movement
        self.move_toward_target(dt);

        // Pregnancy
        if self.pregnant {
            self.pregnancy_progress += dt;
            if self.pregnancy_progress >= self.body_plan.gestation_period {
                self.give_birth();
            }
        }

        // Health update
        self.update_health();

        // Check for death
        if self.should_die() {
            self.behavior = BehaviorState::Dead;
        }
    }

    /// Metabolic processes
    fn metabolize(&mut self, environment: &OrganismEnvironment, dt: f64) {
        // Calculate metabolic cost based on behavior
        let behavior_multiplier = match self.behavior {
            BehaviorState::Resting => 1.0,
            BehaviorState::Foraging => 2.0,
            BehaviorState::Hunting => 4.0,
            BehaviorState::Fleeing => 6.0,
            BehaviorState::Mating => 2.5,
            BehaviorState::Parenting => 2.0,
            BehaviorState::Migrating => 3.0,
            _ => 1.5,
        };

        let metabolic_cost = self.body_plan.basal_metabolic_rate * behavior_multiplier * dt;

        // Subtract from energy
        self.energy -= metabolic_cost;

        // Energy intake based on diet
        let energy_intake = match self.body_plan.diet {
            DietType::Autotrophic => {
                // Photosynthesis
                environment.solar_radiation * 0.001 * dt * self.body_plan.size.powi(2)
            }
            DietType::Herbivore => environment.plant_density * 0.5 * dt * self.body_plan.mass,
            DietType::Carnivore => environment.prey_density * 1.0 * dt * self.body_plan.mass,
            DietType::Omnivore => {
                (environment.plant_density * 0.3 + environment.prey_density * 0.3)
                    * dt
                    * self.body_plan.mass
            }
            DietType::Decomposer => environment.carrion_density * 0.2 * dt * self.body_plan.mass,
        };

        self.energy += energy_intake;

        // Clamp energy
        let max_energy = self.body_plan.mass * ENERGY_RESERVE_MULTIPLIER;
        self.energy = self.energy.clamp(0.0, max_energy);
    }

    /// Decide behavior based on needs
    fn decide_behavior(&mut self, environment: &OrganismEnvironment) {
        // Priority hierarchy:
        // 1. Flee if predators present
        // 2. Eat if hungry
        // 3. Mate if mature and well-fed
        // 4. Rest otherwise

        let hunger = 1.0 - (self.energy / (self.body_plan.mass * ENERGY_RESERVE_MULTIPLIER));

        // Check for predators
        if environment.predator_density > 0.3 && rand::thread_rng().gen::<f64>() < 0.1 {
            self.behavior = BehaviorState::Fleeing;
            return;
        }

        // Hungry?
        if hunger > 0.3 {
            match self.body_plan.diet {
                DietType::Herbivore | DietType::Autotrophic => {
                    self.behavior = BehaviorState::Foraging;
                }
                DietType::Carnivore | DietType::Omnivore => {
                    if environment.prey_density > 0.2 {
                        self.behavior = BehaviorState::Hunting;
                    } else {
                        self.behavior = BehaviorState::Foraging;
                    }
                }
                _ => self.behavior = BehaviorState::Foraging,
            }
            return;
        }

        // Ready to reproduce?
        if self.is_mature
            && hunger < 0.2
            && self.age > MATURITY_AGE * 1.5
            && rand::thread_rng().gen::<f64>() < 0.01
        {
            self.behavior = BehaviorState::SeekingMate;
            return;
        }

        // Default: rest
        self.behavior = BehaviorState::Resting;
    }

    /// Move toward target
    fn move_toward_target(&mut self, dt: f64) {
        if !matches!(self.body_plan.locomotion, LocomotionType::Sessile) {
            let speed = match self.behavior {
                BehaviorState::Fleeing => 5.0,
                BehaviorState::Hunting => 3.0,
                BehaviorState::Migrating => 2.0,
                BehaviorState::Foraging => 1.0,
                _ => 0.1,
            };

            // Simple random walk if no target
            self.velocity.0 += (rand::thread_rng().gen::<f64>() - 0.5) * speed * dt;
            self.velocity.1 += (rand::thread_rng().gen::<f64>() - 0.5) * speed * dt;

            // Clamp velocity
            let max_speed = 10.0;
            self.velocity.0 = self.velocity.0.clamp(-max_speed, max_speed);
            self.velocity.1 = self.velocity.1.clamp(-max_speed, max_speed);
        }
    }

    /// Give birth
    fn give_birth(&mut self) {
        self.pregnant = false;
        self.pregnancy_progress = 0.0;
        self.offspring_count += self.body_plan.litter_size;

        // Energy cost of reproduction
        self.energy -= self.body_plan.litter_size as f64 * 50.0;
    }

    /// Update health
    fn update_health(&mut self) {
        let energy_factor =
            (self.energy / (self.body_plan.mass * ENERGY_RESERVE_MULTIPLIER)).min(1.0);
        let age_factor = 1.0 - (self.age / MAX_LIFESPAN).min(1.0);

        self.health = (energy_factor * age_factor).clamp(0.0, 1.0);
    }

    /// Check if should die
    fn should_die(&self) -> bool {
        // Starvation
        if self.energy <= 0.0 {
            return true;
        }

        // Old age
        if self.age > MAX_LIFESPAN {
            return true;
        }

        // Health collapsed
        if self.health <= 0.0 {
            return true;
        }

        false
    }

    /// Check if can reproduce
    pub fn can_reproduce(&self) -> bool {
        self.is_mature
            && self.health > 0.5
            && self.energy > self.body_plan.mass * ENERGY_RESERVE_MULTIPLIER * 0.5
            && !self.pregnant
    }
}

/// Environment for organisms
#[derive(Debug, Clone)]
pub struct OrganismEnvironment {
    /// Solar radiation (W/m²)
    pub solar_radiation: f64,
    /// Plant density (0.0-1.0)
    pub plant_density: f64,
    /// Prey density (0.0-1.0)
    pub prey_density: f64,
    /// Predator density (0.0-1.0)
    pub predator_density: f64,
    /// Carrion density (0.0-1.0)
    pub carrion_density: f64,
    /// Water availability (0.0-1.0)
    pub water_availability: f64,
    /// Temperature (K)
    pub temperature: f64,
    /// Season (0.0-1.0)
    pub season: f64,
}

impl Default for OrganismEnvironment {
    fn default() -> Self {
        Self {
            solar_radiation: 200.0,
            plant_density: 0.5,
            prey_density: 0.2,
            predator_density: 0.1,
            carrion_density: 0.05,
            water_availability: 1.0,
            temperature: 288.0,
            season: 0.5,
        }
    }
}

// ============================================================================
// Organism Manager
// ============================================================================

/// Manages all organisms
#[derive(Debug)]
pub struct OrganismManager {
    /// All organisms
    pub organisms: HashMap<u64, Organism>,
    /// Next organism ID
    next_id: u64,
    /// Total organisms created
    pub total_created: u64,
    /// Total died
    pub total_died: u64,
}

impl Default for OrganismManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganismManager {
    pub fn new() -> Self {
        Self {
            organisms: HashMap::new(),
            next_id: 0,
            total_created: 0,
            total_died: 0,
        }
    }

    /// Create a new organism
    pub fn create_organism(
        &mut self,
        species_id: u64,
        body_plan: BodyPlan,
        position: HolographicAddress,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let organism = Organism::new(id, species_id, body_plan, position);
        self.organisms.insert(id, organism);
        self.total_created += 1;

        id
    }

    /// Create a human
    pub fn create_human(&mut self, position: HolographicAddress) -> u64 {
        self.create_organism(0, BodyPlan::human(), position)
    }

    /// Create an animal
    pub fn create_animal(
        &mut self,
        species_id: u64,
        mass: f64,
        position: HolographicAddress,
    ) -> u64 {
        self.create_organism(species_id, BodyPlan::simple_animal(mass), position)
    }

    /// Tick all organisms
    pub fn tick(&mut self, environment: &OrganismEnvironment, dt: f64) {
        let mut dead_ids = Vec::new();

        for organism in self.organisms.values_mut() {
            organism.tick(environment, dt);

            if matches!(organism.behavior, BehaviorState::Dead) {
                dead_ids.push(organism.id);
            }
        }

        // Remove dead organisms
        for id in dead_ids {
            self.organisms.remove(&id);
            self.total_died += 1;
        }
    }

    /// Get total organism count
    pub fn total(&self) -> usize {
        self.organisms.len()
    }

    /// Get average health
    pub fn average_health(&self) -> f64 {
        if self.organisms.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.organisms.values().map(|o| o.health).sum();
        sum / self.organisms.len() as f64
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organism_creation() {
        let mut manager = OrganismManager::new();
        let pos = HolographicAddress::default();

        let id = manager.create_animal(0, 10.0, pos);
        assert!(manager.organisms.contains_key(&id));
    }

    #[test]
    fn test_organism_tick() {
        let mut manager = OrganismManager::new();
        let pos = HolographicAddress::default();

        let id = manager.create_animal(0, 10.0, pos);
        let env = OrganismEnvironment::default();

        manager.tick(&env, 1.0);

        let org = manager.organisms.get(&id).unwrap();
        assert!(org.age > 0.0);
    }
}
