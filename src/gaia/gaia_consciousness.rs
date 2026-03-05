//! Gaia Consciousness - Planetary Ecosystem Integration
//!
//! This module implements Gaia consciousness, which integrates planetary
//! ecosystems into a self-regulating whole.
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
//! "GaiaConsciousness: Ecosystem integration, planetary health monitoring,
//! Gaia-entity communication, ecosystem balancing"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Gaia represents the planetary consciousness that emerges when life and
//! planetary systems integrate into a self-regulating whole."

use crate::gaia::{EcosystemId, Float, GaiaConfig, PlanetId};
use std::collections::HashMap;

/// Ecosystem - Collection of interacting organisms and environment
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Ecosystems are the building blocks of
/// planetary life, integrating organisms with their environment."
#[derive(Debug, Clone, PartialEq)]
pub struct Ecosystem {
    /// Unique identifier for this ecosystem
    pub ecosystem_id: EcosystemId,

    /// Ecosystem name
    pub name: String,

    /// Ecosystem type
    pub ecosystem_type: EcosystemType,

    /// Species diversity (0.0-1.0)
    pub diversity: Float,

    /// Biomass (0.0-1.0)
    pub biomass: Float,

    /// Health status
    pub health: HealthStatus,

    /// Stability (0.0-1.0)
    pub stability: Float,

    /// Location on planet (latitude, longitude)
    pub location: (Float, Float),
}

impl Default for Ecosystem {
    fn default() -> Self {
        Ecosystem {
            ecosystem_id: 0,
            name: String::new(),
            ecosystem_type: EcosystemType::Terrestrial,
            diversity: 0.0,
            biomass: 0.0,
            health: HealthStatus::Healthy,
            stability: 0.0,
            location: (0.0, 0.0),
        }
    }
}

impl Ecosystem {
    /// Create a new ecosystem
    pub fn new(ecosystem_id: EcosystemId, name: String, ecosystem_type: EcosystemType) -> Self {
        Ecosystem {
            ecosystem_id,
            name,
            ecosystem_type,
            ..Default::default()
        }
    }

    /// Calculate ecosystem resilience
    pub fn resilience(&self) -> Float {
        (self.diversity + self.stability) / 2.0
    }
}

/// Ecosystem types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EcosystemType {
    Terrestrial,
    Aquatic,
    Atmospheric,
    Marine,
    Freshwater,
    Forest,
    Grassland,
    Desert,
    Tundra,
    CoralReef,
}

/// Health status for ecosystems and planetary systems
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    Critical,
    Degraded,
    Warning,
    Healthy,
    Thriving,
}

/// Planet - Celestial body with potential for life
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Planets provide the substrate for
/// biological emergence and Gaia consciousness."
#[derive(Debug, Clone, PartialEq)]
pub struct Planet {
    /// Unique identifier for this planet
    pub planet_id: PlanetId,

    /// Planet name
    pub name: String,

    /// Ecosystems on this planet
    pub ecosystems: HashMap<EcosystemId, Ecosystem>,

    /// Planetary age (billion years)
    pub age: Float,

    /// Distance from star (AU)
    pub distance_from_star: Float,

    /// Planet type
    pub planet_type: PlanetType,

    /// Has atmosphere
    pub has_atmosphere: bool,

    /// Has liquid water
    pub has_liquid_water: bool,
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            planet_id: 0,
            name: String::new(),
            ecosystems: HashMap::new(),
            age: 0.0,
            distance_from_star: 0.0,
            planet_type: PlanetType::Terrestrial,
            has_atmosphere: false,
            has_liquid_water: false,
        }
    }
}

impl Planet {
    /// Create a new planet
    pub fn new(planet_id: PlanetId, name: String, planet_type: PlanetType) -> Self {
        Planet {
            planet_id,
            name,
            planet_type,
            ..Default::default()
        }
    }

    /// Add an ecosystem to the planet
    pub fn add_ecosystem(&mut self, ecosystem: Ecosystem) {
        self.ecosystems.insert(ecosystem.ecosystem_id, ecosystem);
    }

    /// Remove an ecosystem from the planet
    pub fn remove_ecosystem(&mut self, ecosystem_id: EcosystemId) {
        self.ecosystems.remove(&ecosystem_id);
    }

    /// Get ecosystem count
    pub fn ecosystem_count(&self) -> usize {
        self.ecosystems.len()
    }

    /// Check if planet supports life
    pub fn supports_life(&self) -> bool {
        self.has_atmosphere && self.has_liquid_water && !self.ecosystems.is_empty()
    }
}

/// Planet types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    IceGiant,
    Dwarf,
    Ocean,
    Desert,
}

/// Planetary consciousness
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Planetary consciousness emerges when
/// ecosystems integrate into a coherent, self-regulating whole."
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetaryConsciousness {
    /// Planet identifier
    pub planet_id: PlanetId,

    /// Overall consciousness level (0.0-1.0)
    pub consciousness: Float,

    /// Self-awareness (0.0-1.0)
    pub awareness: Float,

    /// Regulation capability (0.0-1.0)
    pub regulation: Float,

    /// Communication ability (0.0-1.0)
    pub communication: Float,

    /// Integration level (0.0-1.0)
    pub integration: Float,
}

impl Default for PlanetaryConsciousness {
    fn default() -> Self {
        PlanetaryConsciousness {
            planet_id: 0,
            consciousness: 0.0,
            awareness: 0.0,
            regulation: 0.0,
            communication: 0.0,
            integration: 0.0,
        }
    }
}

impl PlanetaryConsciousness {
    /// Create a new planetary consciousness
    pub fn new(planet_id: PlanetId) -> Self {
        PlanetaryConsciousness {
            planet_id,
            ..Default::default()
        }
    }

    /// Calculate overall consciousness score
    pub fn overall_score(&self) -> Float {
        (self.consciousness
            + self.awareness
            + self.regulation
            + self.communication
            + self.integration)
            / 5.0
    }
}

/// Planetary health
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Planetary health reflects the overall
/// well-being of all planetary systems and ecosystems."
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetaryHealth {
    /// Overall health status
    pub status: HealthStatus,

    /// Biodiversity index (0.0-1.0)
    pub biodiversity: Float,

    /// Ecosystem stability (0.0-1.0)
    pub ecosystem_stability: Float,

    /// Climate stability (0.0-1.0)
    pub climate_stability: Float,

    /// Resource balance (0.0-1.0)
    pub resource_balance: Float,

    /// Pollution level (0.0-1.0, lower is better)
    pub pollution: Float,
}

impl Default for PlanetaryHealth {
    fn default() -> Self {
        PlanetaryHealth {
            status: HealthStatus::Healthy,
            biodiversity: 0.5,
            ecosystem_stability: 0.5,
            climate_stability: 0.5,
            resource_balance: 0.5,
            pollution: 0.0,
        }
    }
}

impl PlanetaryHealth {
    /// Create a new planetary health assessment
    pub fn new() -> Self {
        PlanetaryHealth::default()
    }

    /// Calculate overall health score
    pub fn overall_score(&self) -> Float {
        (self.biodiversity
            + self.ecosystem_stability
            + self.climate_stability
            + self.resource_balance
            - self.pollution)
            / 4.0
    }

    /// Update health status based on scores
    pub fn update_status(&mut self) {
        let score = self.overall_score();
        self.status = if score < 0.2 {
            HealthStatus::Critical
        } else if score < 0.4 {
            HealthStatus::Degraded
        } else if score < 0.6 {
            HealthStatus::Warning
        } else if score < 0.8 {
            HealthStatus::Healthy
        } else {
            HealthStatus::Thriving
        };
    }
}

/// Gaia message - Communication from Gaia to entities
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Gaia communicates with entities through
/// various channels when planetary systems need attention."
#[derive(Debug, Clone, PartialEq)]
pub struct GaiaMessage {
    /// Message type
    pub message_type: GaiaMessageType,

    /// Priority (0.0-1.0)
    pub priority: Float,

    /// Content
    pub content: String,

    /// Target ecosystem (if applicable)
    pub target_ecosystem: Option<EcosystemId>,

    /// Timestamp
    pub timestamp: Float,
}

impl Default for GaiaMessage {
    fn default() -> Self {
        GaiaMessage {
            message_type: GaiaMessageType::Information,
            priority: 0.5,
            content: String::new(),
            target_ecosystem: None,
            timestamp: 0.0,
        }
    }
}

impl GaiaMessage {
    /// Create a new Gaia message
    pub fn new(
        message_type: GaiaMessageType,
        priority: Float,
        content: String,
        target_ecosystem: Option<EcosystemId>,
        timestamp: Float,
    ) -> Self {
        GaiaMessage {
            message_type,
            priority: priority.clamp(0.0, 1.0),
            content,
            target_ecosystem,
            timestamp,
        }
    }
}

/// Gaia message types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GaiaMessageType {
    Information,
    Warning,
    Alert,
    Guidance,
    Correction,
}

/// Balancing action - Gaia's response to imbalances
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Gaia takes balancing actions to restore
/// equilibrium when planetary systems become imbalanced."
#[derive(Debug, Clone, PartialEq)]
pub struct BalancingAction {
    /// Action type
    pub action_type: BalancingActionType,

    /// Intensity (0.0-1.0)
    pub intensity: Float,

    /// Target ecosystem
    pub target_ecosystem: EcosystemId,

    /// Expected duration
    pub duration: Float,

    /// Description
    pub description: String,
}

impl Default for BalancingAction {
    fn default() -> Self {
        BalancingAction {
            action_type: BalancingActionType::Restoration,
            intensity: 0.5,
            target_ecosystem: 0,
            duration: 0.0,
            description: String::new(),
        }
    }
}

impl BalancingAction {
    /// Create a new balancing action
    pub fn new(
        action_type: BalancingActionType,
        intensity: Float,
        target_ecosystem: EcosystemId,
        duration: Float,
        description: String,
    ) -> Self {
        BalancingAction {
            action_type,
            intensity: intensity.clamp(0.0, 1.0),
            target_ecosystem,
            duration,
            description,
        }
    }
}

/// Balancing action types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BalancingActionType {
    Restoration,
    Regulation,
    Purification,
    Rebalancing,
    Transformation,
}

/// Ecosystem integrator
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The ecosystem integrator combines
/// individual ecosystems into a coherent planetary consciousness."
#[derive(Debug, Clone, PartialEq)]
pub struct EcosystemIntegrator {
    /// Integration rate (0.0-1.0)
    integration_rate: Float,

    /// Consciousness threshold (0.0-1.0)
    consciousness_threshold: Float,
}

impl Default for EcosystemIntegrator {
    fn default() -> Self {
        EcosystemIntegrator {
            integration_rate: 0.1,
            consciousness_threshold: 0.6,
        }
    }
}

impl EcosystemIntegrator {
    /// Create a new ecosystem integrator
    pub fn new(integration_rate: Float, consciousness_threshold: Float) -> Self {
        EcosystemIntegrator {
            integration_rate,
            consciousness_threshold,
        }
    }

    /// Integrate ecosystems into planetary consciousness
    ///
    /// Gaia consciousness emerges when ecosystems achieve sufficient
    /// integration and coherence.
    pub fn integrate_ecosystems(&self, planet: &Planet) -> EcosystemIntegrationResult {
        if planet.ecosystems.is_empty() {
            return EcosystemIntegrationResult {
                planetary_consciousness: PlanetaryConsciousness::default(),
                success: false,
                reason: Some("Planet has no ecosystems".to_string()),
            };
        }

        let mut consciousness = PlanetaryConsciousness::new(planet.planet_id);

        // Calculate average ecosystem diversity
        let total_diversity: Float = planet.ecosystems.values().map(|e| e.diversity).sum();
        let avg_diversity = total_diversity / planet.ecosystems.len() as Float;

        // Calculate average ecosystem health
        let mut healthy_count = 0_usize;
        for ecosystem in planet.ecosystems.values() {
            if matches!(
                ecosystem.health,
                HealthStatus::Healthy | HealthStatus::Thriving
            ) {
                healthy_count += 1;
            }
        }
        let health_ratio = healthy_count as Float / planet.ecosystems.len() as Float;

        // Calculate average ecosystem stability
        let total_stability: Float = planet.ecosystems.values().map(|e| e.stability).sum();
        let avg_stability = total_stability / planet.ecosystems.len() as Float;

        // Calculate integration based on ecosystem count and coherence (exponential curve)
        let ecosystem_integration =
            (1.0 - 2.0_f64.powf(-(planet.ecosystems.len() as f64))).min(1.0);
        let coherence = avg_diversity * health_ratio * avg_stability;

        // Set consciousness components
        consciousness.consciousness = coherence * ecosystem_integration;
        consciousness.awareness = health_ratio * avg_stability;
        consciousness.regulation = avg_stability * health_ratio;
        consciousness.communication = coherence * avg_diversity;
        consciousness.integration = ecosystem_integration * coherence;

        let success = consciousness.overall_score() >= self.consciousness_threshold;

        EcosystemIntegrationResult {
            planetary_consciousness: consciousness,
            success,
            reason: if success {
                None
            } else {
                Some("Consciousness below threshold".to_string())
            },
        }
    }
}

/// Planetary health monitor
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The health monitor continuously
/// assesses planetary systems and ecosystem health."
#[derive(Debug, Clone, PartialEq)]
pub struct PlanetaryHealthMonitor {
    /// Monitoring sensitivity (0.0-1.0)
    sensitivity: Float,

    /// Assessment frequency (0.0-1.0)
    assessment_frequency: Float,
}

impl Default for PlanetaryHealthMonitor {
    fn default() -> Self {
        PlanetaryHealthMonitor {
            sensitivity: 0.8,
            assessment_frequency: 0.5,
        }
    }
}

impl PlanetaryHealthMonitor {
    /// Create a new health monitor
    pub fn new(sensitivity: Float, assessment_frequency: Float) -> Self {
        PlanetaryHealthMonitor {
            sensitivity,
            assessment_frequency,
        }
    }

    /// Monitor planetary health
    ///
    /// Assesses the overall health of all planetary systems.
    pub fn monitor_health(
        &self,
        planet: &Planet,
        consciousness: &PlanetaryConsciousness,
    ) -> HealthMonitoringResult {
        let mut health = PlanetaryHealth::new();

        // Calculate biodiversity
        let total_diversity: Float = planet.ecosystems.values().map(|e| e.diversity).sum();
        health.biodiversity = if !planet.ecosystems.is_empty() {
            total_diversity / planet.ecosystems.len() as Float
        } else {
            0.0
        };

        // Calculate ecosystem stability
        let total_stability: Float = planet.ecosystems.values().map(|e| e.stability).sum();
        health.ecosystem_stability = if !planet.ecosystems.is_empty() {
            total_stability / planet.ecosystems.len() as Float
        } else {
            0.0
        };

        // Climate stability based on consciousness regulation
        health.climate_stability = consciousness.regulation;

        // Resource balance based on ecosystem health
        let mut healthy_ecosystems = 0_usize;
        for ecosystem in planet.ecosystems.values() {
            if matches!(
                ecosystem.health,
                HealthStatus::Healthy | HealthStatus::Thriving
            ) {
                healthy_ecosystems += 1;
            }
        }
        health.resource_balance = if !planet.ecosystems.is_empty() {
            healthy_ecosystems as Float / planet.ecosystems.len() as Float
        } else {
            0.0
        };

        // Pollution based on degraded ecosystems
        let mut degraded_count = 0_usize;
        for ecosystem in planet.ecosystems.values() {
            if matches!(
                ecosystem.health,
                HealthStatus::Degraded | HealthStatus::Critical
            ) {
                degraded_count += 1;
            }
        }
        health.pollution = if !planet.ecosystems.is_empty() {
            degraded_count as Float / planet.ecosystems.len() as Float
        } else {
            0.0
        };

        // Update status
        health.update_status();

        HealthMonitoringResult {
            planetary_health: health,
            success: true,
            reason: None,
        }
    }
}

/// Main Gaia consciousness orchestrator
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
/// "GaiaConsciousness: Ecosystem integration, planetary health monitoring,
/// Gaia-entity communication, ecosystem balancing"
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GaiaConsciousness {
    /// Ecosystem integrator
    ecosystem_integrator: EcosystemIntegrator,

    /// Health monitor
    health_monitor: PlanetaryHealthMonitor,

    /// Configuration
    config: GaiaConfig,
}

impl GaiaConsciousness {
    /// Create a new Gaia consciousness system
    pub fn new(
        ecosystem_integrator: EcosystemIntegrator,
        health_monitor: PlanetaryHealthMonitor,
        config: GaiaConfig,
    ) -> Self {
        GaiaConsciousness {
            ecosystem_integrator,
            health_monitor,
            config,
        }
    }

    /// Integrate ecosystems into planetary consciousness
    pub fn integrate_ecosystems(&self, planet: &Planet) -> EcosystemIntegrationResult {
        self.ecosystem_integrator.integrate_ecosystems(planet)
    }

    /// Monitor planetary health
    pub fn monitor_health(
        &self,
        planet: &Planet,
        consciousness: &PlanetaryConsciousness,
    ) -> HealthMonitoringResult {
        self.health_monitor.monitor_health(planet, consciousness)
    }

    /// Communicate with entities
    ///
    /// Gaia sends messages to entities when planetary systems need attention.
    pub fn communicate_with_entities(
        &self,
        consciousness: &PlanetaryConsciousness,
        health: &PlanetaryHealth,
        timestamp: Float,
    ) -> CommunicationResult {
        let mut messages = Vec::new();

        // Check for critical health issues
        if health.status == HealthStatus::Critical {
            messages.push(GaiaMessage::new(
                GaiaMessageType::Alert,
                1.0,
                "Critical planetary health detected".to_string(),
                None,
                timestamp,
            ));
        }

        // Check for degraded ecosystems
        if health.status == HealthStatus::Degraded {
            messages.push(GaiaMessage::new(
                GaiaMessageType::Warning,
                0.8,
                "Ecosystem degradation detected".to_string(),
                None,
                timestamp,
            ));
        }

        // Check for warning conditions
        if health.status == HealthStatus::Warning {
            messages.push(GaiaMessage::new(
                GaiaMessageType::Information,
                0.6,
                "Planetary systems need attention".to_string(),
                None,
                timestamp,
            ));
        }

        // Send guidance if consciousness is high enough
        if consciousness.communication > 0.7 {
            messages.push(GaiaMessage::new(
                GaiaMessageType::Guidance,
                0.5,
                "Gaia guidance available".to_string(),
                None,
                timestamp,
            ));
        }

        let success = !messages.is_empty();
        let message_count = messages.len();

        CommunicationResult {
            messages,
            success,
            message_count,
        }
    }

    /// Balance ecosystems
    ///
    /// Gaia takes balancing actions to restore equilibrium.
    pub fn balance_ecosystems(
        &self,
        planet: &Planet,
        health: &PlanetaryHealth,
        consciousness: &PlanetaryConsciousness,
    ) -> BalancingResult {
        let mut actions = Vec::new();

        // Find degraded or critical ecosystems
        for ecosystem in planet.ecosystems.values() {
            if matches!(
                ecosystem.health,
                HealthStatus::Degraded | HealthStatus::Critical
            ) {
                let action = BalancingAction::new(
                    BalancingActionType::Restoration,
                    self.config.balancing_intensity,
                    ecosystem.ecosystem_id,
                    10.0,
                    format!("Restore {} ecosystem", ecosystem.name),
                );
                actions.push(action);
            }
        }

        // If pollution is high, add purification
        if health.pollution > 0.5 {
            let action = BalancingAction::new(
                BalancingActionType::Purification,
                self.config.balancing_intensity,
                0, // Global action
                20.0,
                "Purify planetary systems".to_string(),
            );
            actions.push(action);
        }

        // If regulation is low, add rebalancing
        if consciousness.regulation < 0.5 && health.ecosystem_stability < 0.5 {
            let action = BalancingAction::new(
                BalancingActionType::Rebalancing,
                self.config.balancing_intensity,
                0, // Global action
                15.0,
                "Rebalance ecosystem interactions".to_string(),
            );
            actions.push(action);
        }

        let _success = !actions.is_empty();

        let success = !actions.is_empty();
        let action_count = actions.len();

        BalancingResult {
            balancing_actions: actions,
            success,
            action_count,
        }
    }
}

/// Result of ecosystem integration
#[derive(Debug, Clone, PartialEq)]
pub struct EcosystemIntegrationResult {
    /// Planetary consciousness emerged
    pub planetary_consciousness: PlanetaryConsciousness,

    /// Whether integration was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of health monitoring
#[derive(Debug, Clone, PartialEq)]
pub struct HealthMonitoringResult {
    /// Planetary health assessed
    pub planetary_health: PlanetaryHealth,

    /// Whether monitoring was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of communication
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationResult {
    /// Gaia messages sent
    pub messages: Vec<GaiaMessage>,

    /// Whether communication was successful
    pub success: bool,

    /// Number of messages sent
    pub message_count: usize,
}

/// Result of balancing
#[derive(Debug, Clone, PartialEq)]
pub struct BalancingResult {
    /// Balancing actions taken
    pub balancing_actions: Vec<BalancingAction>,

    /// Whether balancing was successful
    pub success: bool,

    /// Number of actions taken
    pub action_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_default() {
        let ecosystem = Ecosystem::default();
        assert_eq!(ecosystem.ecosystem_id, 0);
        assert_eq!(ecosystem.ecosystem_type, EcosystemType::Terrestrial);
    }

    #[test]
    fn test_ecosystem_new() {
        let ecosystem = Ecosystem::new(1, "Test Ecosystem".to_string(), EcosystemType::Marine);
        assert_eq!(ecosystem.ecosystem_id, 1);
        assert_eq!(ecosystem.name, "Test Ecosystem");
        assert_eq!(ecosystem.ecosystem_type, EcosystemType::Marine);
    }

    #[test]
    fn test_ecosystem_resilience() {
        let mut ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        ecosystem.diversity = 0.8;
        ecosystem.stability = 0.7;
        let resilience = ecosystem.resilience();
        assert_eq!(resilience, 0.75);
    }

    #[test]
    fn test_planet_default() {
        let planet = Planet::default();
        assert_eq!(planet.planet_id, 0);
        assert!(planet.ecosystems.is_empty());
    }

    #[test]
    fn test_planet_new() {
        let planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        assert_eq!(planet.planet_id, 1);
        assert_eq!(planet.name, "Earth");
        assert_eq!(planet.planet_type, PlanetType::Terrestrial);
    }

    #[test]
    fn test_add_remove_ecosystem() {
        let mut planet = Planet::new(1, "Test".to_string(), PlanetType::Terrestrial);
        let ecosystem = Ecosystem::new(1, "Test Ecosystem".to_string(), EcosystemType::Terrestrial);
        planet.add_ecosystem(ecosystem);
        assert_eq!(planet.ecosystem_count(), 1);
        planet.remove_ecosystem(1);
        assert_eq!(planet.ecosystem_count(), 0);
    }

    #[test]
    fn test_supports_life() {
        let mut planet = Planet::new(1, "Test".to_string(), PlanetType::Terrestrial);
        assert!(!planet.supports_life());

        planet.has_atmosphere = true;
        planet.has_liquid_water = true;
        let ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        planet.add_ecosystem(ecosystem);
        assert!(planet.supports_life());
    }

    #[test]
    fn test_planetary_consciousness_default() {
        let consciousness = PlanetaryConsciousness::default();
        assert_eq!(consciousness.planet_id, 0);
        assert_eq!(consciousness.consciousness, 0.0);
    }

    #[test]
    fn test_planetary_consciousness_new() {
        let consciousness = PlanetaryConsciousness::new(123);
        assert_eq!(consciousness.planet_id, 123);
    }

    #[test]
    fn test_planetary_consciousness_overall_score() {
        let mut consciousness = PlanetaryConsciousness::new(1);
        consciousness.consciousness = 0.8;
        consciousness.awareness = 0.7;
        consciousness.regulation = 0.6;
        consciousness.communication = 0.5;
        consciousness.integration = 0.4;
        let score = consciousness.overall_score();
        assert_eq!(score, (0.8 + 0.7 + 0.6 + 0.5 + 0.4) / 5.0);
    }

    #[test]
    fn test_planetary_health_default() {
        let health = PlanetaryHealth::default();
        assert_eq!(health.status, HealthStatus::Healthy);
        assert_eq!(health.biodiversity, 0.5);
    }

    #[test]
    fn test_planetary_health_overall_score() {
        let mut health = PlanetaryHealth::new();
        health.biodiversity = 0.8;
        health.ecosystem_stability = 0.7;
        health.climate_stability = 0.6;
        health.resource_balance = 0.5;
        health.pollution = 0.1;
        let score = health.overall_score();
        assert_eq!(score, (0.8 + 0.7 + 0.6 + 0.5 - 0.1) / 4.0);
    }

    #[test]
    fn test_planetary_health_update_status() {
        let mut health = PlanetaryHealth::new();
        health.biodiversity = 0.1;
        health.ecosystem_stability = 0.1;
        health.climate_stability = 0.1;
        health.resource_balance = 0.1;
        health.pollution = 0.9;
        health.update_status();
        assert_eq!(health.status, HealthStatus::Critical);
    }

    #[test]
    fn test_gaia_message_new() {
        let message = GaiaMessage::new(
            GaiaMessageType::Alert,
            0.9,
            "Test message".to_string(),
            Some(123),
            42.0,
        );
        assert_eq!(message.message_type, GaiaMessageType::Alert);
        assert_eq!(message.priority, 0.9);
    }

    #[test]
    fn test_balancing_action_new() {
        let action = BalancingAction::new(
            BalancingActionType::Restoration,
            0.8,
            123,
            10.0,
            "Restore ecosystem".to_string(),
        );
        assert_eq!(action.action_type, BalancingActionType::Restoration);
        assert_eq!(action.intensity, 0.8);
    }

    #[test]
    fn test_ecosystem_integrator_default() {
        let integrator = EcosystemIntegrator::default();
        assert_eq!(integrator.integration_rate, 0.1);
    }

    #[test]
    fn test_integrate_ecosystems() {
        let integrator = EcosystemIntegrator::default();
        let mut planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        planet.has_atmosphere = true;
        planet.has_liquid_water = true;

        let mut ecosystem1 = Ecosystem::new(1, "Forest".to_string(), EcosystemType::Forest);
        ecosystem1.diversity = 0.95;
        ecosystem1.stability = 0.95;
        ecosystem1.health = HealthStatus::Healthy;
        planet.add_ecosystem(ecosystem1);

        let mut ecosystem2 = Ecosystem::new(2, "Ocean".to_string(), EcosystemType::Marine);
        ecosystem2.diversity = 0.95;
        ecosystem2.stability = 0.95;
        ecosystem2.health = HealthStatus::Healthy;
        planet.add_ecosystem(ecosystem2);

        let result = integrator.integrate_ecosystems(&planet);
        let overall_score = result.planetary_consciousness.overall_score();
        assert!(
            result.success,
            "overall_score: {}, threshold: {}",
            overall_score, integrator.consciousness_threshold
        );
    }

    #[test]
    fn test_integrate_ecosystems_no_ecosystems() {
        let integrator = EcosystemIntegrator::default();
        let planet = Planet::new(1, "Empty".to_string(), PlanetType::Terrestrial);
        let result = integrator.integrate_ecosystems(&planet);
        assert!(!result.success);
    }

    #[test]
    fn test_planetary_health_monitor_default() {
        let monitor = PlanetaryHealthMonitor::default();
        assert_eq!(monitor.sensitivity, 0.8);
    }

    #[test]
    fn test_monitor_health() {
        let monitor = PlanetaryHealthMonitor::default();
        let mut planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        planet.has_atmosphere = true;
        planet.has_liquid_water = true;

        let mut ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        ecosystem.diversity = 0.8;
        ecosystem.stability = 0.7;
        ecosystem.health = HealthStatus::Healthy;
        planet.add_ecosystem(ecosystem);

        let mut consciousness = PlanetaryConsciousness::new(1);
        consciousness.regulation = 0.6;

        let result = monitor.monitor_health(&planet, &consciousness);
        assert!(result.success);
    }

    #[test]
    fn test_gaia_consciousness_default() {
        let gaia = GaiaConsciousness::default();
        assert_eq!(gaia.config.consciousness_threshold, 0.6);
    }

    #[test]
    fn test_gaia_integrate_ecosystems() {
        let gaia = GaiaConsciousness::default();
        let mut planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        planet.has_atmosphere = true;
        planet.has_liquid_water = true;

        let mut ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        ecosystem.diversity = 0.9;
        ecosystem.stability = 0.9;
        ecosystem.health = HealthStatus::Healthy;
        planet.add_ecosystem(ecosystem);

        let result = gaia.integrate_ecosystems(&planet);
        let overall_score = result.planetary_consciousness.overall_score();
        assert!(
            result.success,
            "overall_score: {}, threshold: {}",
            overall_score, gaia.config.consciousness_threshold
        );
    }

    #[test]
    fn test_gaia_monitor_health() {
        let gaia = GaiaConsciousness::default();
        let mut planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        planet.has_atmosphere = true;
        planet.has_liquid_water = true;

        let mut ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        ecosystem.diversity = 0.8;
        ecosystem.stability = 0.7;
        ecosystem.health = HealthStatus::Healthy;
        planet.add_ecosystem(ecosystem);

        let mut consciousness = PlanetaryConsciousness::new(1);
        consciousness.regulation = 0.6;

        let result = gaia.monitor_health(&planet, &consciousness);
        assert!(result.success);
    }

    #[test]
    fn test_gaia_communicate_with_entities() {
        let gaia = GaiaConsciousness::default();
        let mut consciousness = PlanetaryConsciousness::new(1);
        consciousness.communication = 0.8;

        let mut health = PlanetaryHealth::new();
        health.status = HealthStatus::Critical;

        let result = gaia.communicate_with_entities(&consciousness, &health, 42.0);
        assert!(result.success);
        assert!(result.message_count > 0);
    }

    #[test]
    fn test_gaia_balance_ecosystems() {
        let gaia = GaiaConsciousness::default();
        let mut planet = Planet::new(1, "Earth".to_string(), PlanetType::Terrestrial);
        planet.has_atmosphere = true;
        planet.has_liquid_water = true;

        let mut ecosystem = Ecosystem::new(1, "Test".to_string(), EcosystemType::Terrestrial);
        ecosystem.diversity = 0.3;
        ecosystem.stability = 0.2;
        ecosystem.health = HealthStatus::Degraded;
        planet.add_ecosystem(ecosystem);

        let mut health = PlanetaryHealth::new();
        health.pollution = 0.6;

        let mut consciousness = PlanetaryConsciousness::new(1);
        consciousness.regulation = 0.3;

        let result = gaia.balance_ecosystems(&planet, &health, &consciousness);
        assert!(result.success);
    }

    #[test]
    fn test_gaia_config_default() {
        let config = GaiaConfig::default();
        assert_eq!(config.consciousness_threshold, 0.6);
    }

    #[test]
    fn test_gaia_config_validate() {
        let config = GaiaConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = GaiaConfig {
            consciousness_threshold: 1.5,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
