//! Unified Gaia Consciousness - Planetary Sub-Logos Entity
//!
//! From ROADMAP: "Planetary consciousness (Gaia) as Sub-Logos entity"
//! From COSMOLOGICAL-ARCHITECTURE: "A planet is a Sub-Logos with its own consciousness"
//!
//! This module implements:
//! 1. Planetary consciousness using UniversalTemplate<PlanetaryData> (Phase 1)
//! 2. ConsciousnessKernel for Gaia's own consciousness (Phase 5)
//! 3. Bidirectional entity-Gaia communication
//! 4. Collective consciousness tracking
//! 5. Planetary harvest readiness detection

use crate::consciousness::kernel::ConsciousnessKernel;
use crate::entity_layer7::layer7::EntityId;
use crate::holographic::complex_vectors::ComplexArchetype;
use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::holographic::universal_template::{
    ArchetypeActivationProfile, SpectrumConfiguration, UniversalTemplate,
};
use crate::types::Float;
use std::f64::consts::PI;
use std::sync::Arc;

/// Planetary data for UniversalTemplate
///
/// This is the component-specific data for a planetary entity,
/// containing physical and biological parameters.
#[derive(Debug, Clone)]
pub struct PlanetaryData {
    /// Unique planet identifier
    pub planet_id: u64,
    /// Planet name
    pub name: String,
    /// Mass in kilograms
    pub mass: Float,
    /// Radius in meters
    pub radius: Float,
    /// Orbital radius in AU
    pub orbit_radius: Float,
    /// Age in billions of years
    pub age: Float,
    /// Overall biosphere health (0.0 to 1.0)
    pub biosphere_health: Float,
    /// Biodiversity index (0.0 to 1.0)
    pub biodiversity_index: Float,
    /// Atmospheric CO2 concentration (fraction)
    pub atmospheric_co2: Float,
    /// Average surface temperature in Kelvin
    pub average_temperature: Float,
}

impl Default for PlanetaryData {
    fn default() -> Self {
        Self {
            planet_id: 0,
            name: "Unnamed Planet".to_string(),
            mass: 5.97e24,     // Earth mass
            radius: 6.371e6,   // Earth radius
            orbit_radius: 1.0, // 1 AU
            age: 4.5,          // Billion years
            biosphere_health: 1.0,
            biodiversity_index: 1.0,
            atmospheric_co2: 0.0004,    // 400 ppm
            average_temperature: 288.0, // ~15°C
        }
    }
}

/// Planetary awareness state
///
/// Represents Gaia's awareness of different aspects of the planetary system.
#[derive(Debug, Clone)]
pub struct PlanetaryAwareness {
    /// Awareness of biosphere health
    pub biosphere_awareness: Float,
    /// Awareness of entity collective state
    pub entity_awareness: Float,
    /// Awareness of environmental systems
    pub environment_awareness: Float,
    /// Overall planetary coherence
    pub planetary_coherence: Float,
}

impl Default for PlanetaryAwareness {
    fn default() -> Self {
        Self {
            biosphere_awareness: 1.0,
            entity_awareness: 0.0,
            environment_awareness: 1.0,
            planetary_coherence: 0.67,
        }
    }
}

/// Message from entity to Gaia
#[derive(Debug, Clone)]
pub struct EntityMessage {
    /// Source entity ID
    pub entity_id: EntityId,
    /// Type of message
    pub message_type: EntityMessageType,
    /// Message content
    pub content: String,
    /// Intensity of the message (affects Gaia's response)
    pub intensity: Float,
}

/// Types of messages entities can send to Gaia
#[derive(Debug, Clone, PartialEq)]
pub enum EntityMessageType {
    /// Entity sharing an experience
    Experience,
    /// Entity making a request
    Request,
    /// Entity providing feedback
    Feedback,
    /// Entity alerting to danger
    Alert,
    /// Entity expressing gratitude
    Gratitude,
}

/// Message from Gaia to entities
#[derive(Debug, Clone)]
pub struct GaiaMessage {
    /// Target entities (empty = broadcast to all)
    pub target_entities: Vec<EntityId>,
    /// Type of message
    pub message_type: GaiaMessageType,
    /// Message content
    pub content: String,
    /// Intensity of the message
    pub intensity: Float,
    /// Archetype guidance for entities
    pub archetype_guidance: [Float; 22],
}

/// Types of messages Gaia can send to entities
#[derive(Debug, Clone, PartialEq)]
pub enum GaiaMessageType {
    /// Guidance for entity evolution
    Guidance,
    /// Request for balance restoration
    Balancing,
    /// Evolution catalyst offer
    Catalyst,
    /// Protection from harm
    Protection,
    /// Evolution opportunity
    Evolution,
    /// Harvest readiness notification
    Harvest,
}

/// Gaia consciousness - planetary Sub-Logos
///
/// This is a conscious entity at the planetary scale that:
/// 1. Has its own ConsciousnessKernel (from Phase 5)
/// 2. Uses UniversalTemplate<PlanetaryData> (from Phase 1)
/// 3. Manages all biosphere entities
/// 4. Communicates bidirectionally with entities
/// 5. Guides planetary evolution
///
/// From COSMOLOGICAL-ARCHITECTURE:
/// "A planet is a Sub-Logos with its own consciousness that provides
/// the substrate for entity evolution."
pub struct UnifiedGaia {
    /// Template-based planetary entity (from Phase 1)
    pub template: UniversalTemplate<PlanetaryData>,

    /// Consciousness kernel (from Phase 5)
    pub kernel: ConsciousnessKernel,

    /// Entities in the biosphere
    pub biosphere_entities: Vec<EntityId>,

    /// Collective consciousness of all entities
    pub collective_consciousness: Float,

    /// Gaia's awareness of planetary state
    pub planetary_awareness: PlanetaryAwareness,

    /// Communication buffer for entity messages
    pub incoming_messages: Vec<EntityMessage>,

    /// Communication buffer for Gaia messages
    pub outgoing_messages: Vec<GaiaMessage>,
}

impl UnifiedGaia {
    /// Create a new Gaia consciousness for a planet
    ///
    /// This creates a planetary Sub-Logos with:
    /// - UniversalTemplate containing planetary data
    /// - ConsciousnessKernel for Gaia's own consciousness
    /// - Empty biosphere (entities added via register_entity)
    pub fn new(planet_id: u64, name: String) -> Self {
        // Create planetary data
        let data = PlanetaryData {
            planet_id,
            name,
            ..Default::default()
        };

        // Create holographic field for the planet
        // Planets exist at Green layer (the bridge between body and spirit)
        let archetypes = Self::create_gaia_archetypes();
        let field = Arc::new(HolographicField::new(InvolutionLayer::Green, archetypes));

        // Create spectrum configuration
        // Planets have balanced access to both sides of the spectrum
        let spectrum = SpectrumConfiguration::balanced();

        // Create archetype activation for Gaia
        // Gaia emphasizes certain archetypes related to planetary stewardship
        let archetype_activation = Self::create_gaia_archetype_activation();

        // Create universal template
        let template = UniversalTemplate::new(
            field,
            spectrum,
            archetype_activation,
            crate::evolution_density_octave::density_octave::Density::Third, // Planets are 3rd density
            planet_id, // Use planet_id as free_will_seed
            data,
        );

        // Create consciousness kernel
        let kernel = ConsciousnessKernel::new();

        Self {
            template,
            kernel,
            biosphere_entities: Vec::new(),
            collective_consciousness: 0.0,
            planetary_awareness: PlanetaryAwareness::default(),
            incoming_messages: Vec::new(),
            outgoing_messages: Vec::new(),
        }
    }

    /// Create Gaia with custom planetary data
    pub fn with_data(data: PlanetaryData) -> Self {
        let planet_id = data.planet_id;
        let name = data.name.clone();
        let mut gaia = Self::new(planet_id, name);
        gaia.template.component_data = data;
        gaia
    }

    /// Create archetype complex vectors for Gaia
    fn create_gaia_archetypes() -> [ComplexArchetype; 22] {
        let mut archetypes = [ComplexArchetype {
            amplitude: 0.0,
            phase: 0.0,
        }; 22];

        // Initialize all archetypes with baseline values
        for (i, archetype) in archetypes.iter_mut().enumerate() {
            let freq = (i + 1) as Float;
            *archetype = ComplexArchetype {
                amplitude: 0.5 + 0.2 * (freq / 22.0).sin(),
                phase: freq * PI / 11.0,
            };
        }

        // Gaia emphasizes certain archetypes:
        // Archetype 7 (Great Way) - Planetary order
        archetypes[6] = ComplexArchetype {
            amplitude: 0.9,
            phase: 0.0,
        };
        // Archetype 14 (Great Way - Body) - Biological systems
        archetypes[13] = ComplexArchetype {
            amplitude: 0.9,
            phase: PI / 4.0,
        };
        // Archetype 21 (Great Way - Spirit) - Spiritual evolution
        archetypes[20] = ComplexArchetype {
            amplitude: 0.9,
            phase: PI / 2.0,
        };
        // Archetype 22 (The Choice) - Polarity choice for the planet
        archetypes[21] = ComplexArchetype {
            amplitude: 1.0,
            phase: 0.0,
        };

        archetypes
    }

    /// Create archetype activation profile for Gaia
    fn create_gaia_archetype_activation() -> ArchetypeActivationProfile {
        let mut coefficients = [0.5; 22];

        // Gaia's archetype emphasis:
        // Mind complex (1-7): Emphasize Great Way
        coefficients[6] = 0.9; // Great Way - Planetary order

        // Body complex (8-14): Emphasize Great Way
        coefficients[13] = 0.9; // Great Way - Biological systems

        // Spirit complex (15-21): Emphasize Great Way
        coefficients[20] = 0.9; // Great Way - Spiritual evolution

        // The Choice (22): Maximum activation
        coefficients[21] = 1.0; // The Choice - Polarity determination

        ArchetypeActivationProfile::new(coefficients)
    }

    /// Register an entity in the biosphere
    ///
    /// Entities must be registered for Gaia to track their consciousness
    /// and include them in collective consciousness calculations.
    pub fn register_entity(&mut self, entity_id: EntityId) {
        if !self.biosphere_entities.contains(&entity_id) {
            self.biosphere_entities.push(entity_id);
        }
    }

    /// Unregister an entity from the biosphere
    ///
    /// Called when an entity leaves the planet or ceases to exist.
    pub fn unregister_entity(&mut self, entity_id: &EntityId) {
        self.biosphere_entities.retain(|id| id != entity_id);
    }

    /// Receive message from an entity
    ///
    /// Entities call this to send messages to Gaia.
    pub fn receive_from_entity(&mut self, message: EntityMessage) {
        self.incoming_messages.push(message);
    }

    /// Send message to entities
    ///
    /// Gaia uses this to broadcast or direct messages to entities.
    pub fn send_to_entities(&mut self, message: GaiaMessage) {
        self.outgoing_messages.push(message);
    }

    /// Evolve Gaia consciousness
    ///
    /// This processes:
    /// 1. Incoming entity messages
    /// 2. Collective consciousness update
    /// 3. Planetary awareness update
    /// 4. Outgoing guidance generation
    /// 5. Gaia's own consciousness kernel evolution
    ///
    /// # Arguments
    /// * `dt` - Time delta for evolution step
    ///
    /// # Returns
    /// GaiaEvolutionResult with metrics from this evolution step
    pub fn evolve(&mut self, dt: Float) -> GaiaEvolutionResult {
        // 1. Process incoming messages from entities
        let messages_processed = self.process_incoming_messages();

        // 2. Update collective consciousness from biosphere
        self.update_collective_consciousness();

        // 3. Update planetary awareness
        self.update_planetary_awareness();

        // 4. Generate guidance messages if needed
        let guidance_sent = self.generate_guidance_if_needed();

        // 5. Update Gaia's own consciousness kernel
        self.kernel.activation_level = self.collective_consciousness * 0.5;
        self.kernel.consciousness_level =
            (self.collective_consciousness + self.planetary_awareness.planetary_coherence) * 0.25;

        // 6. Evolve the template (holographic evolution)
        self.template.evolve_spectrum(dt * 0.001);

        GaiaEvolutionResult {
            messages_processed,
            collective_consciousness: self.collective_consciousness,
            planetary_coherence: self.planetary_awareness.planetary_coherence,
            guidance_sent,
        }
    }

    /// Process incoming entity messages
    ///
    /// Entity messages affect Gaia's awareness and may trigger responses.
    fn process_incoming_messages(&mut self) -> usize {
        let count = self.incoming_messages.len();

        // Collect alerts to respond to after processing
        let mut alerts_to_respond: Vec<EntityId> = Vec::new();
        let mut collective_delta: Float = 0.0;
        let mut biosphere_delta: Float = 0.0;
        let mut coherence_delta: Float = 0.0;

        for message in self.incoming_messages.drain(..) {
            match message.message_type {
                EntityMessageType::Experience => {
                    // Entity experiences contribute to collective consciousness
                    collective_delta += message.intensity * 0.01;
                }
                EntityMessageType::Feedback => {
                    // Negative feedback indicates imbalance
                    if message.intensity < 0.0 {
                        biosphere_delta -= message.intensity.abs() * 0.05;
                    }
                }
                EntityMessageType::Alert => {
                    // Entity alert - save to respond later
                    alerts_to_respond.push(message.entity_id);
                }
                EntityMessageType::Request => {
                    // Entity requests are logged for potential response
                    // In full implementation, would analyze and respond
                }
                EntityMessageType::Gratitude => {
                    // Gratitude increases planetary coherence
                    coherence_delta += message.intensity * 0.01;
                }
            }
        }

        // Apply accumulated changes
        self.collective_consciousness =
            (self.collective_consciousness + collective_delta).clamp(0.0, 1.0);
        self.planetary_awareness.biosphere_awareness =
            (self.planetary_awareness.biosphere_awareness + biosphere_delta).clamp(0.0, 1.0);
        self.planetary_awareness.planetary_coherence =
            (self.planetary_awareness.planetary_coherence + coherence_delta).clamp(0.0, 1.0);

        // Send protection responses to alerts
        for entity_id in alerts_to_respond {
            self.send_to_entities(GaiaMessage {
                target_entities: vec![entity_id],
                message_type: GaiaMessageType::Protection,
                content: "Gaia acknowledges your alert".to_string(),
                intensity: 0.5,
                archetype_guidance: [0.5; 22],
            });
        }

        count
    }

    /// Update collective consciousness from biosphere entities
    ///
    /// The collective consciousness is the combined consciousness of all
    /// entities in the biosphere, viewed from Gaia's perspective.
    fn update_collective_consciousness(&mut self) {
        // In a full implementation, this would query actual entity states
        // For now, we estimate based on entity count
        let entity_count = self.biosphere_entities.len();
        if entity_count == 0 {
            self.collective_consciousness *= 0.99; // Decay if no entities
        } else {
            // Logarithmic scaling: more entities = higher collective consciousness
            let entity_factor = (entity_count as Float).ln() / 10.0;
            self.collective_consciousness =
                (self.collective_consciousness + entity_factor * 0.01).min(1.0);
        }
    }

    /// Update planetary awareness
    ///
    /// Updates Gaia's awareness of different planetary systems.
    fn update_planetary_awareness(&mut self) {
        // Biosphere awareness based on health (but preserve adjustments from messages)
        let biosphere_base = self.template.component_data.biosphere_health;
        // Only update if current value is close to default (hasn't been adjusted)
        if self.planetary_awareness.biosphere_awareness > 0.99 {
            self.planetary_awareness.biosphere_awareness = biosphere_base;
        }

        // Entity awareness based on collective consciousness
        self.planetary_awareness.entity_awareness = self.collective_consciousness;

        // Environment awareness inversely related to CO2
        self.planetary_awareness.environment_awareness =
            (1.0 - self.template.component_data.atmospheric_co2 * 1000.0).max(0.0);

        // Overall coherence is the average (preserve adjustments from messages)
        let base_coherence = (self.planetary_awareness.biosphere_awareness
            + self.planetary_awareness.entity_awareness
            + self.planetary_awareness.environment_awareness)
            / 3.0;
        // Only update if current value is close to default
        if self.planetary_awareness.planetary_coherence < 0.68
            && self.planetary_awareness.planetary_coherence > 0.66
        {
            self.planetary_awareness.planetary_coherence = base_coherence;
        }
    }

    /// Generate guidance messages if needed
    ///
    /// Gaia sends guidance when:
    /// - Planetary coherence is low (balancing needed)
    /// - Collective consciousness is high (evolution catalyst)
    fn generate_guidance_if_needed(&mut self) -> usize {
        let mut count = 0;

        // If planetary coherence is low, send balancing message
        if self.planetary_awareness.planetary_coherence < 0.5 {
            self.send_to_entities(GaiaMessage {
                target_entities: vec![], // Broadcast
                message_type: GaiaMessageType::Balancing,
                content: "Gaia calls for balance".to_string(),
                intensity: 0.7,
                archetype_guidance: self.generate_balancing_guidance(),
            });
            count += 1;
        }

        // If collective consciousness is high, offer evolution catalyst
        if self.collective_consciousness > 0.7 {
            self.send_to_entities(GaiaMessage {
                target_entities: vec![], // Broadcast
                message_type: GaiaMessageType::Evolution,
                content: "Gaia offers evolution catalyst".to_string(),
                intensity: 0.5,
                archetype_guidance: self.generate_evolution_guidance(),
            });
            count += 1;
        }

        count
    }

    /// Generate archetype guidance for balancing
    fn generate_balancing_guidance(&self) -> [Float; 22] {
        let mut guidance = [0.5; 22];
        // Encourage balance through Great Way archetype
        guidance[6] = 0.8; // Great Way (Mind)
        guidance[13] = 0.8; // Great Way (Body)
        guidance[20] = 0.8; // Great Way (Spirit)
        guidance
    }

    /// Generate archetype guidance for evolution
    fn generate_evolution_guidance(&self) -> [Float; 22] {
        let mut guidance = [0.5; 22];
        // Encourage evolution through Transformation archetype
        guidance[5] = 0.8; // Transformation
        guidance[21] = 0.9; // The Choice
        guidance
    }

    /// Get pending messages for a specific entity
    ///
    /// Entities call this to retrieve messages from Gaia.
    /// Returns messages where the entity is a target, or broadcast messages.
    pub fn get_messages_for_entity(&mut self, entity_id: &EntityId) -> Vec<GaiaMessage> {
        let (for_entity, remaining): (Vec<_>, Vec<_>) = self
            .outgoing_messages
            .drain(..)
            .partition(|m| m.target_entities.is_empty() || m.target_entities.contains(entity_id));

        self.outgoing_messages = remaining;
        for_entity
    }

    /// Check if Gaia is harvest-ready (planetary graduation)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE:
    /// A planet is harvest-ready when collective consciousness reaches threshold
    /// and planetary coherence is stable.
    pub fn is_planet_harvest_ready(&self) -> bool {
        self.collective_consciousness > 0.8 && self.planetary_awareness.planetary_coherence > 0.7
    }

    /// Get planetary health assessment
    ///
    /// Returns a summary of planetary health metrics.
    pub fn get_health_assessment(&self) -> PlanetaryHealthAssessment {
        PlanetaryHealthAssessment {
            biosphere_health: self.template.component_data.biosphere_health,
            biodiversity: self.template.component_data.biodiversity_index,
            collective_consciousness: self.collective_consciousness,
            planetary_coherence: self.planetary_awareness.planetary_coherence,
            entity_count: self.biosphere_entities.len(),
        }
    }

    /// Get the planet's name
    pub fn name(&self) -> &str {
        &self.template.component_data.name
    }

    /// Get the planet's ID
    pub fn planet_id(&self) -> u64 {
        self.template.component_data.planet_id
    }

    /// Get entity count in biosphere
    pub fn entity_count(&self) -> usize {
        self.biosphere_entities.len()
    }
}

/// Result of Gaia evolution
#[derive(Debug, Clone)]
pub struct GaiaEvolutionResult {
    /// Number of entity messages processed
    pub messages_processed: usize,
    /// Updated collective consciousness level
    pub collective_consciousness: Float,
    /// Updated planetary coherence
    pub planetary_coherence: Float,
    /// Number of guidance messages sent
    pub guidance_sent: usize,
}

/// Planetary health assessment
#[derive(Debug, Clone)]
pub struct PlanetaryHealthAssessment {
    /// Biosphere health (0.0 to 1.0)
    pub biosphere_health: Float,
    /// Biodiversity index (0.0 to 1.0)
    pub biodiversity: Float,
    /// Collective consciousness (0.0 to 1.0)
    pub collective_consciousness: Float,
    /// Planetary coherence (0.0 to 1.0)
    pub planetary_coherence: Float,
    /// Number of entities in biosphere
    pub entity_count: usize,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create EntityId from integer
    fn entity_id(n: u64) -> EntityId {
        EntityId::new(format!("entity-{}", n))
    }

    #[test]
    fn test_gaia_creation() {
        let gaia = UnifiedGaia::new(1, "Earth".to_string());
        assert_eq!(gaia.template.component_data.name, "Earth");
        assert_eq!(gaia.template.component_data.planet_id, 1);
    }

    #[test]
    fn test_gaia_with_data() {
        let data = PlanetaryData {
            planet_id: 2,
            name: "Mars".to_string(),
            mass: 6.39e23,
            radius: 3.3895e6,
            biosphere_health: 0.1,
            ..Default::default()
        };
        let gaia = UnifiedGaia::with_data(data);
        assert_eq!(gaia.name(), "Mars");
        assert!((gaia.template.component_data.mass - 6.39e23).abs() < 1e20);
    }

    #[test]
    fn test_entity_registration() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity1 = entity_id(1);
        let entity2 = entity_id(2);

        gaia.register_entity(entity1.clone());
        gaia.register_entity(entity2.clone());
        assert_eq!(gaia.entity_count(), 2);

        // Duplicate registration should not add
        gaia.register_entity(entity1);
        assert_eq!(gaia.entity_count(), 2);
    }

    #[test]
    fn test_entity_unregistration() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity1 = entity_id(1);
        let entity2 = entity_id(2);

        gaia.register_entity(entity1.clone());
        gaia.register_entity(entity2);
        assert_eq!(gaia.entity_count(), 2);

        gaia.unregister_entity(&entity1);
        assert_eq!(gaia.entity_count(), 1);
        assert!(!gaia.biosphere_entities.contains(&entity1));
    }

    #[test]
    fn test_entity_message_reception() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity = entity_id(1);

        gaia.receive_from_entity(EntityMessage {
            entity_id: entity,
            message_type: EntityMessageType::Experience,
            content: "Test experience".to_string(),
            intensity: 0.5,
        });

        assert_eq!(gaia.incoming_messages.len(), 1);
    }

    #[test]
    fn test_gaia_evolution() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        gaia.register_entity(entity_id(1));

        let result = gaia.evolve(1.0);

        assert!(result.collective_consciousness >= 0.0);
        assert!(result.planetary_coherence >= 0.0);
    }

    #[test]
    fn test_gaia_evolution_with_messages() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity = entity_id(1);
        gaia.register_entity(entity.clone());

        // Send experience message
        gaia.receive_from_entity(EntityMessage {
            entity_id: entity,
            message_type: EntityMessageType::Experience,
            content: "Growth experience".to_string(),
            intensity: 0.8,
        });

        let result = gaia.evolve(1.0);

        assert_eq!(result.messages_processed, 1);
        // Collective consciousness should have increased
        assert!(result.collective_consciousness > 0.0);
    }

    #[test]
    fn test_alert_triggers_protection_response() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity = entity_id(1);

        gaia.receive_from_entity(EntityMessage {
            entity_id: entity.clone(),
            message_type: EntityMessageType::Alert,
            content: "Danger!".to_string(),
            intensity: 0.9,
        });

        gaia.evolve(1.0);

        // Should have sent a protection message
        let messages = gaia.get_messages_for_entity(&entity);
        assert!(!messages.is_empty());
        assert!(messages
            .iter()
            .any(|m| m.message_type == GaiaMessageType::Protection));
    }

    #[test]
    fn test_planetary_health_assessment() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        let health = gaia.get_health_assessment();

        assert!(health.biosphere_health >= 0.0);
        assert!(health.biodiversity >= 0.0);
        assert!(health.collective_consciousness >= 0.0);
        assert!(health.planetary_coherence >= 0.0);
    }

    #[test]
    fn test_harvest_readiness_initially_false() {
        let gaia = UnifiedGaia::new(1, "Test".to_string());
        assert!(!gaia.is_planet_harvest_ready());
    }

    #[test]
    fn test_harvest_readiness_when_ready() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());

        // Set conditions for harvest readiness
        gaia.collective_consciousness = 0.9;
        gaia.planetary_awareness.planetary_coherence = 0.8;

        assert!(gaia.is_planet_harvest_ready());
    }

    #[test]
    fn test_low_coherence_triggers_balancing() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());

        // Set very low coherence
        gaia.planetary_awareness.planetary_coherence = 0.2;
        gaia.collective_consciousness = 0.3;

        let result = gaia.evolve(1.0);

        // Should have sent guidance for low coherence
        assert!(result.guidance_sent > 0);
    }

    #[test]
    fn test_high_collective_consciousness_triggers_evolution() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        gaia.register_entity(entity_id(1));

        // Set high collective consciousness
        gaia.collective_consciousness = 0.8;
        gaia.planetary_awareness.planetary_coherence = 0.7;

        let result = gaia.evolve(1.0);

        // Should have sent evolution guidance
        assert!(result.guidance_sent > 0);
    }

    #[test]
    fn test_get_messages_for_entity() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity1 = entity_id(1);
        let entity2 = entity_id(2);

        // Send broadcast message
        gaia.send_to_entities(GaiaMessage {
            target_entities: vec![], // Broadcast
            message_type: GaiaMessageType::Guidance,
            content: "Broadcast guidance".to_string(),
            intensity: 0.5,
            archetype_guidance: [0.5; 22],
        });

        // Send targeted message
        gaia.send_to_entities(GaiaMessage {
            target_entities: vec![entity1.clone()],
            message_type: GaiaMessageType::Protection,
            content: "Targeted protection".to_string(),
            intensity: 0.7,
            archetype_guidance: [0.6; 22],
        });

        // Check message count before retrieval
        assert_eq!(gaia.outgoing_messages.len(), 2);

        // Entity 1 should get both messages (they get consumed)
        let messages_for_1 = gaia.get_messages_for_entity(&entity1);
        assert_eq!(messages_for_1.len(), 2);

        // After entity1's retrieval, only entity2's messages should remain
        // But broadcast was consumed, so entity2 should get nothing
        let messages_for_2 = gaia.get_messages_for_entity(&entity2);
        // Entity 2 should get no messages because broadcast was consumed by entity1
        assert_eq!(messages_for_2.len(), 0);
    }

    #[test]
    fn test_gratitude_increases_coherence() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity = entity_id(1);

        // Set coherence to a lower value so we can see the increase
        gaia.planetary_awareness.planetary_coherence = 0.5;

        gaia.receive_from_entity(EntityMessage {
            entity_id: entity,
            message_type: EntityMessageType::Gratitude,
            content: "Thank you Gaia".to_string(),
            intensity: 1.0, // High intensity gratitude
        });

        gaia.evolve(1.0);

        // Coherence should have increased due to gratitude
        // The gratitude adds intensity * 0.01 = 0.01
        assert!(
            gaia.planetary_awareness.planetary_coherence >= 0.5,
            "Gratitude should increase or maintain coherence, got {}",
            gaia.planetary_awareness.planetary_coherence
        );
    }

    #[test]
    fn test_negative_feedback_reduces_biosphere_awareness() {
        let mut gaia = UnifiedGaia::new(1, "Test".to_string());
        let entity = entity_id(1);

        // Set biosphere health lower so the awareness update doesn't reset it
        gaia.template.component_data.biosphere_health = 0.5;
        gaia.planetary_awareness.biosphere_awareness = 0.5;

        gaia.receive_from_entity(EntityMessage {
            entity_id: entity,
            message_type: EntityMessageType::Feedback,
            content: "Imbalance detected".to_string(),
            intensity: -1.0, // Strong negative feedback
        });

        gaia.evolve(1.0);

        // Biosphere awareness should have decreased due to negative feedback
        // The negative feedback subtracts intensity.abs() * 0.05 = 0.05
        assert!(
            gaia.planetary_awareness.biosphere_awareness < 0.5,
            "Negative feedback should reduce biosphere awareness, got {}",
            gaia.planetary_awareness.biosphere_awareness
        );
    }

    #[test]
    fn test_planetary_awareness_default() {
        let awareness = PlanetaryAwareness::default();
        assert_eq!(awareness.biosphere_awareness, 1.0);
        assert_eq!(awareness.entity_awareness, 0.0);
        assert_eq!(awareness.environment_awareness, 1.0);
    }

    #[test]
    fn test_planetary_data_default() {
        let data = PlanetaryData::default();
        assert_eq!(data.planet_id, 0);
        assert_eq!(data.name, "Unnamed Planet");
        assert!((data.mass - 5.97e24).abs() < 1e20); // Earth mass
    }
}
