//! Gaia Module - Planetary Consciousness and Atmospheric Dynamics
//!
//! This module implements Gaia consciousness and atmospheric dynamics for
//! planetary systems with life.
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
//! "Gaia Systems: Planetary consciousness, ecosystem integration,
//! atmospheric dynamics, climate simulation"

pub mod atmospheric_dynamics;
pub mod gaia_consciousness;

pub use gaia_consciousness::{
    BalancingAction, BalancingResult, CommunicationResult, EcosystemIntegrationResult,
    EcosystemIntegrator, GaiaConsciousness, GaiaMessage, HealthMonitoringResult, HealthStatus,
    PlanetaryConsciousness, PlanetaryHealth, PlanetaryHealthMonitor,
};

pub use atmospheric_dynamics::{
    Atmosphere, AtmosphericDynamics, ChemicalComposition, ChemistryModeler,
    ChemistryModelingResult, Climate, ClimateSimulationResult, ClimateSimulator, CoupledSystem,
    CouplingResult, WeatherPattern, WeatherPredictionResult,
};

/// Configuration for Gaia systems
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Gaia represents the planetary
/// consciousness that emerges when life and planetary systems integrate
/// into a self-regulating whole."
#[derive(Debug, Clone, PartialEq)]
pub struct GaiaConfig {
    /// Consciousness threshold for Gaia emergence (0.0-1.0)
    pub consciousness_threshold: Float,

    /// Health monitoring sensitivity (0.0-1.0)
    pub health_sensitivity: Float,

    /// Balancing action intensity (0.0-1.0)
    pub balancing_intensity: Float,

    /// Climate simulation resolution (0.0-1.0)
    pub climate_resolution: Float,

    /// Atmospheric chemistry rate (0.0-1.0)
    pub chemistry_rate: Float,

    /// Weather prediction accuracy (0.0-1.0)
    pub weather_accuracy: Float,

    /// Biology-atmosphere coupling strength (0.0-1.0)
    pub coupling_strength: Float,
}

impl Default for GaiaConfig {
    fn default() -> Self {
        GaiaConfig {
            consciousness_threshold: 0.6,
            health_sensitivity: 0.8,
            balancing_intensity: 0.7,
            climate_resolution: 0.5,
            chemistry_rate: 0.3,
            weather_accuracy: 0.6,
            coupling_strength: 0.5,
        }
    }
}

impl GaiaConfig {
    /// Create a new Gaia configuration
    pub fn new(
        consciousness_threshold: Float,
        health_sensitivity: Float,
        balancing_intensity: Float,
        climate_resolution: Float,
        chemistry_rate: Float,
        weather_accuracy: Float,
        coupling_strength: Float,
    ) -> Self {
        GaiaConfig {
            consciousness_threshold,
            health_sensitivity,
            balancing_intensity,
            climate_resolution,
            chemistry_rate,
            weather_accuracy,
            coupling_strength,
        }
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.consciousness_threshold) {
            return Err(format!(
                "Consciousness threshold must be between 0.0 and 1.0, got {}",
                self.consciousness_threshold
            ));
        }
        if !(0.0..=1.0).contains(&self.health_sensitivity) {
            return Err(format!(
                "Health sensitivity must be between 0.0 and 1.0, got {}",
                self.health_sensitivity
            ));
        }
        if !(0.0..=1.0).contains(&self.balancing_intensity) {
            return Err(format!(
                "Balancing intensity must be between 0.0 and 1.0, got {}",
                self.balancing_intensity
            ));
        }
        if !(0.0..=1.0).contains(&self.climate_resolution) {
            return Err(format!(
                "Climate resolution must be between 0.0 and 1.0, got {}",
                self.climate_resolution
            ));
        }
        if !(0.0..=1.0).contains(&self.chemistry_rate) {
            return Err(format!(
                "Chemistry rate must be between 0.0 and 1.0, got {}",
                self.chemistry_rate
            ));
        }
        if !(0.0..=1.0).contains(&self.weather_accuracy) {
            return Err(format!(
                "Weather accuracy must be between 0.0 and 1.0, got {}",
                self.weather_accuracy
            ));
        }
        if !(0.0..=1.0).contains(&self.coupling_strength) {
            return Err(format!(
                "Coupling strength must be between 0.0 and 1.0, got {}",
                self.coupling_strength
            ));
        }
        Ok(())
    }
}

// Re-export common types
pub type Float = f64;
pub type PlanetId = u64;
pub type EcosystemId = u64;
pub type AtmosphereId = u64;
