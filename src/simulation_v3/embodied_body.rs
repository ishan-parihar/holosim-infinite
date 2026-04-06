//! Embodied Body - Entity Bodies That Live, Metabolize, and Die
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
//! "Entities have bodies made of cells, need survival, feel environment"
//!
//! This module implements:
//! - EmbodiedBody: A body that wraps organism_lifecycle
//! - SensoryField: What consciousness receives from body
//! - BodyEnvironment: What body experiences from environment
//! - Survival mechanics: Death detection and handling
//!
//! Knowledge Base References:
//! - HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md: Phase 3 embodiment
//! - COSMOLOGICAL-ARCHITECTURE.md: Mind-body-spirit complex
//! - biology/organism_lifecycle.rs: Organism simulation
//! - biology/entity_body_coupling.rs: Energy centers and nervous system

use crate::biology::entity_body_coupling::{ConsciousnessExperience, EnergyCenter, NervousSystem};
use crate::biology::organism_lifecycle::{BodyPlan, Organism, OrganismEnvironment};
use crate::entity_layer7::layer7::EntityId;
use crate::holographic::field_address::HolographicAddress;
use crate::types::Float;

// ============================================================================
// SURVIVAL STATUS AND DEATH CAUSE
// ============================================================================

/// Survival status of an embodied body
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
/// "Bodies need survival, can die from starvation, trauma, disease"
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SurvivalStatus {
    /// Body is healthy and functioning
    #[default]
    Alive,
    /// Body is in critical condition (value indicates health percentage 0.0-1.0)
    Critical(Float),
    /// Body has died (includes cause of death)
    Dead(DeathCause),
}

impl SurvivalStatus {
    /// Check if the body is alive
    pub fn is_alive(&self) -> bool {
        matches!(self, SurvivalStatus::Alive | SurvivalStatus::Critical(_))
    }

    /// Check if the body is dead
    pub fn is_dead(&self) -> bool {
        matches!(self, SurvivalStatus::Dead(_))
    }

    /// Check if the body is in critical condition
    pub fn is_critical(&self) -> bool {
        matches!(self, SurvivalStatus::Critical(_))
    }
}

/// Cause of death for an embodied body
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
/// "Death from starvation, old age, trauma, disease, environmental factors"
#[derive(Debug, Clone, PartialEq)]
pub enum DeathCause {
    /// Energy reserves depleted
    Starvation,
    /// Exceeded maximum lifespan
    OldAge,
    /// Physical injury
    Trauma,
    /// Illness or infection
    Disease,
    /// Environmental conditions (extreme temperature, radiation, etc.)
    Environment,
    /// Other unspecified cause
    Unknown,
}

impl std::fmt::Display for DeathCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathCause::Starvation => write!(f, "Starvation"),
            DeathCause::OldAge => write!(f, "Old Age"),
            DeathCause::Trauma => write!(f, "Trauma"),
            DeathCause::Disease => write!(f, "Disease"),
            DeathCause::Environment => write!(f, "Environment"),
            DeathCause::Unknown => write!(f, "Unknown"),
        }
    }
}

// ============================================================================
// SENSORY FIELD
// ============================================================================

/// Sensory field - what consciousness receives from the body
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
/// "Entities feel environment through sensory fields"
///
/// The sensory field represents the body's internal and external sensations
/// that are transmitted to consciousness through the nervous system.
#[derive(Debug, Clone)]
pub struct SensoryField {
    // ========================================================================
    // INTERNAL SENSATIONS (interoception)
    // ========================================================================
    /// Hunger level (0.0 = satiated, 1.0 = starving)
    pub hunger: Float,

    /// Thirst level (0.0 = hydrated, 1.0 = dehydrated)
    pub thirst: Float,

    /// Pain level (0.0 = no pain, 1.0 = severe pain)
    pub pain: Float,

    /// Fatigue level (0.0 = rested, 1.0 = exhausted)
    pub fatigue: Float,

    /// Thermal discomfort (-1.0 = freezing, 0.0 = comfortable, 1.0 = overheating)
    pub thermal_discomfort: Float,

    // ========================================================================
    // EXTERNAL ENVIRONMENT (filtered through body)
    // ========================================================================
    /// Felt temperature (K) - filtered through body's thermal regulation
    pub felt_temperature: Float,

    /// Pressure sensation (0.0-1.0 normalized)
    pub pressure: Float,

    /// Light level (0.0 = dark, 1.0 = bright)
    pub light_level: Float,

    /// Sound level (0.0 = silent, 1.0 = loud)
    pub sound_level: Float,

    // ========================================================================
    // CHAKRA ACTIVATIONS (energy center states)
    // ========================================================================
    /// Root chakra activation (survival, grounding)
    pub root_activation: Float,

    /// Sacral chakra activation (creativity, sexuality)
    pub sacral_activation: Float,

    /// Solar plexus chakra activation (power, will)
    pub solar_activation: Float,

    /// Heart chakra activation (love, compassion)
    pub heart_activation: Float,

    /// Throat chakra activation (communication, truth)
    pub throat_activation: Float,

    /// Third eye chakra activation (intuition, insight)
    pub third_eye_activation: Float,

    /// Crown chakra activation (spirit, connection)
    pub crown_activation: Float,

    // ========================================================================
    // AGGREGATE METRICS
    // ========================================================================
    /// Overall comfort level (0.0-1.0, higher is more comfortable)
    pub comfort: Float,

    /// Stress level (0.0-1.0, higher is more stressed)
    pub stress: Float,

    /// Threat level (0.0-1.0, higher indicates perceived danger)
    pub threat_level: Float,

    /// Vitality level (0.0-1.0, overall life energy)
    pub vitality: Float,
}

impl Default for SensoryField {
    fn default() -> Self {
        Self {
            // Internal sensations - default to comfortable
            hunger: 0.0,
            thirst: 0.0,
            pain: 0.0,
            fatigue: 0.0,
            thermal_discomfort: 0.0,

            // External sensations - neutral
            felt_temperature: 293.0, // ~20°C in Kelvin
            pressure: 0.5,
            light_level: 0.5,
            sound_level: 0.3,

            // Chakra activations - baseline
            root_activation: 0.5,
            sacral_activation: 0.5,
            solar_activation: 0.5,
            heart_activation: 0.5,
            throat_activation: 0.5,
            third_eye_activation: 0.5,
            crown_activation: 0.5,

            // Aggregate metrics
            comfort: 1.0,
            stress: 0.0,
            threat_level: 0.0,
            vitality: 1.0,
        }
    }
}

impl SensoryField {
    /// Create a new sensory field with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate overall distress level (0.0-1.0)
    ///
    /// Higher values indicate more distress from internal sensations
    pub fn calculate_distress(&self) -> Float {
        let internal_distress = (self.hunger + self.thirst + self.pain + self.fatigue) / 4.0;
        let thermal_distress = self.thermal_discomfort.abs();
        (internal_distress + thermal_distress + self.stress) / 3.0
    }

    /// Get chakra activations as an array
    pub fn chakra_activations(&self) -> [Float; 7] {
        [
            self.root_activation,
            self.sacral_activation,
            self.solar_activation,
            self.heart_activation,
            self.throat_activation,
            self.third_eye_activation,
            self.crown_activation,
        ]
    }

    /// Convert to consciousness experience for archetype processing
    ///
    /// This bridges the sensory field to the consciousness system
    pub fn to_consciousness_experience(&self) -> ConsciousnessExperience {
        ConsciousnessExperience {
            survival_effect: 1.0 - (self.hunger + self.thirst) / 2.0,
            creativity_effect: self.sacral_activation - 0.5,
            power_effect: self.solar_activation - 0.5,
            love_effect: self.heart_activation - 0.5,
            communication_effect: self.throat_activation - 0.5,
            intuition_effect: self.third_eye_activation - 0.5,
            spirit_effect: self.crown_activation - 0.5,
            stress: self.stress,
            joy: self.comfort * self.vitality,
        }
    }
}

// ============================================================================
// BODY ENVIRONMENT
// ============================================================================

/// Body environment - what the body experiences from the environment
///
/// This is a bridge between the simulation environment and the organism
/// environment, providing additional context for survival calculations.
#[derive(Debug, Clone)]
pub struct BodyEnvironment {
    // ========================================================================
    // PHYSICAL CONDITIONS
    // ========================================================================
    /// Temperature (Kelvin)
    pub temperature: Float,

    /// Atmospheric pressure (Pascals, normalized for display)
    pub pressure: Float,

    /// Solar radiation (W/m²)
    pub solar_radiation: Float,

    /// Background radiation (Sv/h equivalent, normalized)
    pub radiation: Float,

    /// Gravitational acceleration (m/s², Earth = 9.8)
    pub gravity: Float,

    // ========================================================================
    // RESOURCE AVAILABILITY
    // ========================================================================
    /// Food density (0.0-1.0, normalized)
    pub food_density: Float,

    /// Water availability (0.0-1.0, normalized)
    pub water_availability: Float,

    /// Air quality (0.0-1.0, 1.0 = pure oxygen, 0.0 = unbreathable)
    pub air_quality: Float,

    // ========================================================================
    // ECOLOGICAL CONTEXT
    // ========================================================================
    /// Prey density (0.0-1.0, for carnivores)
    pub prey_density: Float,

    /// Predator density (0.0-1.0, threat level)
    pub predator_density: Float,

    /// Plant density (0.0-1.0, for herbivores)
    pub plant_density: Float,

    // ========================================================================
    // TEMPORAL CONTEXT
    // ========================================================================
    /// Season (0.0-1.0, 0.0 = winter start, 1.0 = winter end)
    pub season: Float,

    /// Day/night phase (0.0-1.0, 0.0 = midnight, 0.5 = noon)
    pub day_night_phase: Float,
}

impl Default for BodyEnvironment {
    fn default() -> Self {
        Self {
            // Physical conditions - Earth-like defaults
            temperature: 288.0, // ~15°C
            pressure: 101325.0, // 1 atm
            solar_radiation: 200.0,
            radiation: 0.0,
            gravity: 9.8,

            // Resources - moderate availability
            food_density: 0.5,
            water_availability: 0.8,
            air_quality: 0.9,

            // Ecological - balanced
            prey_density: 0.3,
            predator_density: 0.1,
            plant_density: 0.5,

            // Temporal
            season: 0.5,
            day_night_phase: 0.5,
        }
    }
}

impl BodyEnvironment {
    /// Create a new body environment with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create from an OrganismEnvironment (conversion from biology module)
    ///
    /// This provides bidirectional compatibility with the organism_lifecycle module
    pub fn from_organism_environment(org_env: &OrganismEnvironment) -> Self {
        Self {
            temperature: org_env.temperature,
            pressure: 101325.0, // Default atmospheric pressure
            solar_radiation: org_env.solar_radiation,
            radiation: 0.0,
            gravity: 9.8,
            food_density: match org_env.plant_density + org_env.prey_density {
                x if x > 1.0 => 1.0,
                x => x,
            },
            water_availability: org_env.water_availability,
            air_quality: 1.0, // Default good air quality
            prey_density: org_env.prey_density,
            predator_density: org_env.predator_density,
            plant_density: org_env.plant_density,
            season: org_env.season,
            day_night_phase: 0.5, // Default midday
        }
    }

    /// Convert to an OrganismEnvironment for organism tick
    ///
    /// This provides bidirectional compatibility with the organism_lifecycle module
    pub fn to_organism_environment(&self) -> OrganismEnvironment {
        OrganismEnvironment {
            solar_radiation: self.solar_radiation,
            plant_density: self.plant_density,
            prey_density: self.prey_density,
            predator_density: self.predator_density,
            carrion_density: 0.0, // Not tracked in BodyEnvironment
            water_availability: self.water_availability,
            temperature: self.temperature,
            season: self.season,
        }
    }

    /// Calculate habitability score (0.0-1.0)
    ///
    /// Higher scores indicate more favorable conditions for life
    pub fn calculate_habitability(&self) -> Float {
        // Temperature comfort (optimal around 288K / 15°C)
        let temp_score = 1.0 - ((self.temperature - 288.0) / 50.0).abs().min(1.0);

        // Resource availability
        let resource_score = (self.food_density + self.water_availability + self.air_quality) / 3.0;

        // Safety (low predators, low radiation)
        let safety_score = 1.0 - (self.predator_density + self.radiation).min(1.0);

        // Weighted average
        temp_score * 0.3 + resource_score * 0.4 + safety_score * 0.3
    }

    /// Check if environment is hostile (requires immediate survival response)
    pub fn is_hostile(&self) -> bool {
        self.temperature < 250.0 || self.temperature > 320.0  // Extreme cold/heat
            || self.water_availability < 0.1
            || self.air_quality < 0.2
            || self.predator_density > 0.7
    }
}

// ============================================================================
// EMBODIED BODY
// ============================================================================

/// An embodied body - the physical vessel for consciousness
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
/// "Entities have bodies made of cells, need survival, feel environment"
///
/// This struct wraps the Organism from organism_lifecycle and adds:
/// - Sensory field generation
/// - Survival status tracking
/// - Energy center (chakra) integration
/// - Nervous system processing
#[derive(Debug, Clone)]
pub struct EmbodiedBody {
    /// The underlying organism from biology module
    pub organism: Organism,

    /// Current sensory field (updated each tick)
    pub sensory_field: SensoryField,

    /// Entity ID that owns this body
    pub entity_id: EntityId,

    /// Nervous system for processing sensations
    pub nervous_system: NervousSystem,

    /// Energy centers (chakras) - array of 7 centers
    pub energy_centers: [EnergyCenter; 7],

    /// Current survival status
    pub survival_status: SurvivalStatus,

    /// Time since incarnation began
    pub incarnation_time: Float,
}

impl EmbodiedBody {
    /// Create a new embodied body from a body plan
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
    /// "Body created from blueprint at incarnation"
    ///
    /// # Arguments
    /// * `entity_id` - The entity that will inhabit this body
    /// * `body_plan` - The blueprint for body construction
    ///
    /// # Returns
    /// A new EmbodiedBody ready for simulation
    pub fn new(entity_id: EntityId, body_plan: BodyPlan) -> Self {
        // Create a new organism from the body plan
        let position = HolographicAddress::default();
        let organism = Organism::new(
            entity_id.as_u64(),
            0, // species_id - could be derived from body_plan later
            body_plan,
            position,
        );

        // Create nervous system based on body plan complexity
        let neural_complexity = organism.body_plan.neural_complexity;
        let nervous_system = NervousSystem::new(neural_complexity);

        // Initialize energy centers
        let energy_centers = [
            EnergyCenter::new(0), // Root
            EnergyCenter::new(1), // Sacral
            EnergyCenter::new(2), // Solar Plexus
            EnergyCenter::new(3), // Heart
            EnergyCenter::new(4), // Throat
            EnergyCenter::new(5), // Third Eye
            EnergyCenter::new(6), // Crown
        ];

        Self {
            organism,
            sensory_field: SensoryField::new(),
            entity_id,
            nervous_system,
            energy_centers,
            survival_status: SurvivalStatus::Alive,
            incarnation_time: 0.0,
        }
    }

    /// Create a human body for an entity
    pub fn human(entity_id: EntityId) -> Self {
        Self::new(entity_id, BodyPlan::human())
    }

    /// Create a simple animal body for an entity
    pub fn simple_animal(entity_id: EntityId, mass: Float) -> Self {
        Self::new(entity_id, BodyPlan::simple_animal(mass))
    }

    /// Create a plant body for an entity
    pub fn plant(entity_id: EntityId, size: Float) -> Self {
        Self::new(entity_id, BodyPlan::plant(size))
    }

    /// Main tick - update body state and generate sensory field
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
    /// "Body tick: metabolism -> sensory generation -> survival check"
    ///
    /// # Arguments
    /// * `environment` - The environment the body is in
    /// * `dt` - Time step
    ///
    /// # Returns
    /// Reference to the updated sensory field
    pub fn tick(&mut self, environment: &BodyEnvironment, dt: Float) -> &SensoryField {
        // Skip if already dead
        if self.survival_status.is_dead() {
            return &self.sensory_field;
        }

        // Update incarnation time
        self.incarnation_time += dt;

        // Convert environment and tick the organism
        let org_env = environment.to_organism_environment();
        self.organism.tick(&org_env, dt);

        // Generate sensory field from organism state
        self.generate_sensory_field();

        // Check survival status
        self.check_survival();

        // Return reference to sensory field
        &self.sensory_field
    }

    /// Generate sensory field from organism state
    ///
    /// This translates the organism's internal state into sensations
    /// that consciousness can perceive through the nervous system.
    pub fn generate_sensory_field(&mut self) {
        let organism = &self.organism;

        // Calculate energy ratio for hunger
        let max_energy = organism.body_plan.mass * 100.0; // ENERGY_RESERVE_MULTIPLIER
        let energy_ratio = if max_energy > 0.0 {
            (max_energy - organism.energy) / max_energy
        } else {
            0.0
        };

        // Internal sensations
        self.sensory_field.hunger = energy_ratio.clamp(0.0, 1.0);
        self.sensory_field.thirst = 0.0; // Would need water tracking in organism
        self.sensory_field.pain = 1.0 - organism.health;
        self.sensory_field.fatigue = 1.0 - organism.fitness;

        // Thermal discomfort based on behavior state
        self.sensory_field.thermal_discomfort = match organism.behavior {
            crate::biology::organism_lifecycle::BehaviorState::Resting => 0.0,
            crate::biology::organism_lifecycle::BehaviorState::Fleeing => 0.3,
            _ => 0.1,
        };

        // Update chakra activations from energy centers
        for (i, center) in self.energy_centers.iter().enumerate() {
            let activation = center.activation;
            match i {
                0 => self.sensory_field.root_activation = activation,
                1 => self.sensory_field.sacral_activation = activation,
                2 => self.sensory_field.solar_activation = activation,
                3 => self.sensory_field.heart_activation = activation,
                4 => self.sensory_field.throat_activation = activation,
                5 => self.sensory_field.third_eye_activation = activation,
                6 => self.sensory_field.crown_activation = activation,
                _ => {}
            }
        }

        // Calculate aggregate metrics
        self.sensory_field.comfort = 1.0 - self.sensory_field.calculate_distress();
        self.sensory_field.stress = self.nervous_system.stress;
        self.sensory_field.threat_level = 0.0; // Would need predator detection
        self.sensory_field.vitality = organism.health * (1.0 - energy_ratio / 2.0);
    }

    /// Check survival status and update if necessary
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 3:
    /// "Death detection: starvation, old age, health collapse"
    pub fn check_survival(&mut self) {
        // Check for death conditions
        if self.organism.energy <= 0.0 {
            self.survival_status = SurvivalStatus::Dead(DeathCause::Starvation);
            return;
        }

        if self.organism.health <= 0.0 {
            self.survival_status = SurvivalStatus::Dead(DeathCause::Disease);
            return;
        }

        // Check for old age (MAX_LIFESPAN = 1000.0 in organism_lifecycle)
        if self.organism.age > 1000.0 {
            self.survival_status = SurvivalStatus::Dead(DeathCause::OldAge);
            return;
        }

        // Check for critical condition
        if self.organism.health < 0.2 || self.organism.energy < self.organism.body_plan.mass * 20.0
        {
            self.survival_status = SurvivalStatus::Critical(self.organism.health);
            return;
        }

        // Otherwise alive
        self.survival_status = SurvivalStatus::Alive;
    }

    /// Check if the body is alive
    pub fn is_alive(&self) -> bool {
        self.survival_status.is_alive()
    }

    /// Check if the body is dead
    pub fn is_dead(&self) -> bool {
        self.survival_status.is_dead()
    }

    /// Get the health percentage (0.0-1.0)
    pub fn health_percentage(&self) -> Float {
        self.organism.health
    }

    /// Get the energy percentage (0.0-1.0)
    pub fn energy_percentage(&self) -> Float {
        let max_energy = self.organism.body_plan.mass * 100.0;
        if max_energy > 0.0 {
            (self.organism.energy / max_energy).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Get the age in simulation time units
    pub fn age(&self) -> Float {
        self.organism.age
    }

    /// Process experience through energy centers
    ///
    /// Updates chakra activations based on consciousness experience
    pub fn process_experience(&mut self, experience: &ConsciousnessExperience) {
        for center in &mut self.energy_centers {
            center.process_experience(experience);
        }
    }

    /// Get the body's position in the holographic field
    pub fn position(&self) -> &HolographicAddress {
        &self.organism.position
    }

    /// Set the body's position in the holographic field
    pub fn set_position(&mut self, position: HolographicAddress) {
        self.organism.position = position;
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survival_status_is_alive() {
        assert!(SurvivalStatus::Alive.is_alive());
        assert!(SurvivalStatus::Critical(0.1).is_alive());
        assert!(!SurvivalStatus::Dead(DeathCause::Starvation).is_alive());
    }

    #[test]
    fn test_survival_status_is_dead() {
        assert!(!SurvivalStatus::Alive.is_dead());
        assert!(!SurvivalStatus::Critical(0.1).is_dead());
        assert!(SurvivalStatus::Dead(DeathCause::OldAge).is_dead());
    }

    #[test]
    fn test_survival_status_is_critical() {
        assert!(!SurvivalStatus::Alive.is_critical());
        assert!(SurvivalStatus::Critical(0.1).is_critical());
        assert!(!SurvivalStatus::Dead(DeathCause::Trauma).is_critical());
    }

    #[test]
    fn test_death_cause_display() {
        assert_eq!(format!("{}", DeathCause::Starvation), "Starvation");
        assert_eq!(format!("{}", DeathCause::OldAge), "Old Age");
        assert_eq!(format!("{}", DeathCause::Trauma), "Trauma");
        assert_eq!(format!("{}", DeathCause::Disease), "Disease");
        assert_eq!(format!("{}", DeathCause::Environment), "Environment");
    }

    #[test]
    fn test_sensory_field_default() {
        let field = SensoryField::default();
        assert_eq!(field.hunger, 0.0);
        assert_eq!(field.thirst, 0.0);
        assert_eq!(field.pain, 0.0);
        assert_eq!(field.comfort, 1.0);
        assert_eq!(field.stress, 0.0);
    }

    #[test]
    fn test_sensory_field_calculate_distress() {
        let mut field = SensoryField::default();
        assert!((field.calculate_distress() - 0.0).abs() < 0.001);

        field.hunger = 0.5;
        field.pain = 0.5;
        let distress = field.calculate_distress();
        assert!(distress > 0.0 && distress < 1.0);
    }

    #[test]
    fn test_sensory_field_chakra_activations() {
        let field = SensoryField::default();
        let activations = field.chakra_activations();
        assert_eq!(activations.len(), 7);
        for activation in activations.iter() {
            assert!(*activation >= 0.0 && *activation <= 1.0);
        }
    }

    #[test]
    fn test_sensory_field_to_consciousness_experience() {
        let field = SensoryField::default();
        let experience = field.to_consciousness_experience();

        // Default field should produce positive survival effect
        assert!(experience.survival_effect > 0.0);
        // Default stress should be 0
        assert!((experience.stress - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_body_environment_default() {
        let env = BodyEnvironment::default();
        assert!((env.temperature - 288.0).abs() < 0.1);
        assert!((env.pressure - 101325.0).abs() < 1.0);
        assert!((env.gravity - 9.8).abs() < 0.1);
        assert_eq!(env.food_density, 0.5);
        assert_eq!(env.water_availability, 0.8);
    }

    #[test]
    fn test_body_environment_habitability() {
        let env = BodyEnvironment::default();
        let habitability = env.calculate_habitability();
        assert!(habitability > 0.5 && habitability <= 1.0);

        // Hostile environment should have lower habitability
        let hostile = BodyEnvironment {
            temperature: 350.0,       // Very hot
            water_availability: 0.05, // Almost no water
            ..Default::default()
        };
        let hostile_habitability = hostile.calculate_habitability();
        assert!(hostile_habitability < habitability);
    }

    #[test]
    fn test_body_environment_is_hostile() {
        let env = BodyEnvironment::default();
        assert!(!env.is_hostile());

        let hostile = BodyEnvironment {
            temperature: 200.0, // Very cold
            ..Default::default()
        };
        assert!(hostile.is_hostile());

        let hostile = BodyEnvironment {
            predator_density: 0.8,
            ..Default::default()
        };
        assert!(hostile.is_hostile());
    }

    #[test]
    fn test_body_environment_conversion() {
        let body_env = BodyEnvironment::default();
        let org_env = body_env.to_organism_environment();

        assert!((org_env.temperature - body_env.temperature).abs() < 0.1);
        assert!((org_env.solar_radiation - body_env.solar_radiation).abs() < 0.1);
        assert!((org_env.water_availability - body_env.water_availability).abs() < 0.1);

        // Round-trip conversion
        let back_to_body = BodyEnvironment::from_organism_environment(&org_env);
        assert!((back_to_body.temperature - body_env.temperature).abs() < 0.1);
    }

    #[test]
    fn test_embodied_body_new() {
        let entity_id = EntityId::new("test-entity".to_string());
        let body = EmbodiedBody::human(entity_id);

        assert!(body.is_alive());
        assert!(!body.is_dead());
        assert!((body.health_percentage() - 1.0).abs() < 0.001);
        assert!(body.energy_percentage() > 0.0);
    }

    #[test]
    fn test_embodied_body_tick() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);
        let env = BodyEnvironment::default();

        // Tick should update sensory field
        body.tick(&env, 1.0);

        // Body should still be alive after one tick
        assert!(body.is_alive());

        // Sensory field should have been updated
        assert!(body.sensory_field.vitality > 0.0);
    }

    #[test]
    fn test_embodied_body_survival_check() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);

        // Initially alive
        body.check_survival();
        assert!(body.is_alive());

        // Simulate starvation
        body.organism.energy = 0.0;
        body.check_survival();
        assert!(body.is_dead());
        assert_eq!(
            body.survival_status,
            SurvivalStatus::Dead(DeathCause::Starvation)
        );
    }

    #[test]
    fn test_embodied_body_health_percentage() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);

        // Initially full health
        assert!((body.health_percentage() - 1.0).abs() < 0.001);

        // Damage the body
        body.organism.health = 0.5;
        assert!((body.health_percentage() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_embodied_body_tick_updates_incarnation_time() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);
        let env = BodyEnvironment::default();

        // Initial incarnation time
        assert!(
            (body.incarnation_time - 0.0).abs() < 0.01,
            "Initial incarnation_time should be 0, got {}",
            body.incarnation_time
        );

        // Use small dt values to avoid rapid metabolism death
        // Human basal_metabolic_rate is 1500, dt=0.1 means 150 energy per tick
        // Initial energy is ~7000, so this gives many ticks before starvation

        body.tick(&env, 0.1);
        assert!(
            (body.incarnation_time - 0.1).abs() < 0.01,
            "After first tick, incarnation_time should be 0.1, got {}",
            body.incarnation_time
        );

        // Check body is still alive for second tick
        assert!(
            body.is_alive(),
            "Body should still be alive for second tick"
        );

        body.tick(&env, 0.1);
        assert!(
            (body.incarnation_time - 0.2).abs() < 0.01,
            "After second tick, incarnation_time should be 0.2, got {} (survival: {:?})",
            body.incarnation_time,
            body.survival_status
        );
    }

    #[test]
    fn test_embodied_body_simple_animal() {
        let entity_id = EntityId::new("test-animal".to_string());
        let body = EmbodiedBody::simple_animal(entity_id, 10.0);

        assert!(body.is_alive());
        assert!(body.organism.body_plan.mass > 0.0);
    }

    #[test]
    fn test_embodied_body_plant() {
        let entity_id = EntityId::new("test-plant".to_string());
        let body = EmbodiedBody::plant(entity_id, 1.0);

        assert!(body.is_alive());
        assert!(!body.organism.body_plan.has_nervous_system);
    }

    #[test]
    fn test_embodied_body_generate_sensory_field() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);

        // Set some organism state
        body.organism.health = 0.8;
        body.organism.energy = body.organism.body_plan.mass * 50.0; // 50% energy

        body.generate_sensory_field();

        // Pain should reflect health loss
        assert!((body.sensory_field.pain - 0.2).abs() < 0.01);

        // Hunger should reflect energy loss
        assert!(body.sensory_field.hunger > 0.0);
    }

    #[test]
    fn test_embodied_body_process_experience() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);

        // Create a positive experience
        let experience = ConsciousnessExperience {
            survival_effect: 0.8,
            love_effect: 0.5,
            joy: 0.7,
            ..Default::default()
        };

        body.process_experience(&experience);

        // Energy centers should have been updated
        for center in body.energy_centers.iter() {
            assert!(center.activation >= 0.0 && center.activation <= 1.0);
        }
    }

    #[test]
    fn test_embodied_body_critical_status() {
        let entity_id = EntityId::new("test-entity".to_string());
        let mut body = EmbodiedBody::human(entity_id);

        // Set to critical health
        body.organism.health = 0.1;
        body.check_survival();

        assert!(body.survival_status.is_critical());
        assert!(body.is_alive()); // Critical but still alive
    }
}
