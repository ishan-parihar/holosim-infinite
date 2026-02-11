//! Atmospheric Dynamics - Climate and Weather Coupling with Biology
//!
//! This module implements atmospheric dynamics, climate simulation,
//! and weather patterns that couple with biological systems.
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
//! "AtmosphericDynamics: Climate simulation, atmospheric chemistry,
//! weather patterns, climate-biology coupling"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Atmospheric dynamics couple with biological systems through
//! gas exchange, temperature regulation, and weather patterns."

use crate::gaia::{AtmosphereId, Float, GaiaConfig};
use std::collections::HashMap;

/// Atmosphere - Gaseous envelope surrounding a planet
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The atmosphere provides the medium
/// for weather, climate, and gas exchange with biological systems."
#[derive(Debug, Clone, PartialEq)]
pub struct Atmosphere {
    /// Unique identifier for this atmosphere
    pub atmosphere_id: AtmosphereId,

    /// Atmospheric composition
    pub composition: ChemicalComposition,

    /// Temperature (Kelvin)
    pub temperature: Float,

    /// Pressure (atm)
    pub pressure: Float,

    /// Humidity (0.0-1.0)
    pub humidity: Float,

    /// Wind speed (m/s)
    pub wind_speed: Float,

    /// Cloud cover (0.0-1.0)
    pub cloud_cover: Float,

    /// Visibility (km)
    pub visibility: Float,
}

impl Default for Atmosphere {
    fn default() -> Self {
        Atmosphere {
            atmosphere_id: 0,
            composition: ChemicalComposition::default(),
            temperature: 288.0, // Earth-like temperature
            pressure: 1.0,      // Earth-like pressure
            humidity: 0.5,
            wind_speed: 10.0,
            cloud_cover: 0.5,
            visibility: 10.0,
        }
    }
}

impl Atmosphere {
    /// Create a new atmosphere
    pub fn new(atmosphere_id: AtmosphereId) -> Self {
        Atmosphere {
            atmosphere_id,
            ..Default::default()
        }
    }

    /// Calculate atmospheric density
    pub fn density(&self) -> Float {
        // Ideal gas law approximation
        (self.pressure * 1.225) / (self.temperature / 288.0)
    }
}

/// Chemical composition of atmosphere
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Atmospheric composition determines
/// climate, weather, and biological compatibility."
#[derive(Debug, Clone, PartialEq)]
pub struct ChemicalComposition {
    /// Nitrogen (N2) percentage (0.0-1.0)
    pub nitrogen: Float,

    /// Oxygen (O2) percentage (0.0-1.0)
    pub oxygen: Float,

    /// Carbon dioxide (CO2) percentage (0.0-1.0)
    pub carbon_dioxide: Float,

    /// Argon (Ar) percentage (0.0-1.0)
    pub argon: Float,

    /// Water vapor (H2O) percentage (0.0-1.0)
    pub water_vapor: Float,

    /// Other gases percentage (0.0-1.0)
    pub other: Float,
}

impl Default for ChemicalComposition {
    fn default() -> Self {
        ChemicalComposition {
            nitrogen: 0.78,
            oxygen: 0.21,
            carbon_dioxide: 0.0004,
            argon: 0.0093,
            water_vapor: 0.0,
            other: 0.0003,
        }
    }
}

impl ChemicalComposition {
    /// Create a new chemical composition
    pub fn new(
        nitrogen: Float,
        oxygen: Float,
        carbon_dioxide: Float,
        argon: Float,
        water_vapor: Float,
        other: Float,
    ) -> Self {
        ChemicalComposition {
            nitrogen: nitrogen.max(0.0).min(1.0),
            oxygen: oxygen.max(0.0).min(1.0),
            carbon_dioxide: carbon_dioxide.max(0.0).min(1.0),
            argon: argon.max(0.0).min(1.0),
            water_vapor: water_vapor.max(0.0).min(1.0),
            other: other.max(0.0).min(1.0),
        }
    }

    /// Calculate total percentage
    pub fn total(&self) -> Float {
        self.nitrogen
            + self.oxygen
            + self.carbon_dioxide
            + self.argon
            + self.water_vapor
            + self.other
    }

    /// Normalize to ensure total equals 1.0
    pub fn normalize(&mut self) {
        let total = self.total();
        if total > 0.0 {
            self.nitrogen /= total;
            self.oxygen /= total;
            self.carbon_dioxide /= total;
            self.argon /= total;
            self.water_vapor /= total;
            self.other /= total;
        }
    }

    /// Check if composition supports aerobic life
    pub fn supports_aerobic_life(&self) -> bool {
        self.oxygen >= 0.15 && self.oxygen <= 0.35 && self.carbon_dioxide <= 0.01
    }
}

/// Climate - Long-term weather patterns
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Climate represents the long-term
/// patterns of temperature, precipitation, and weather."
#[derive(Debug, Clone, PartialEq)]
pub struct Climate {
    /// Climate type
    pub climate_type: ClimateType,

    /// Average temperature (Kelvin)
    pub average_temperature: Float,

    /// Temperature range (Kelvin)
    pub temperature_range: Float,

    /// Average precipitation (mm/year)
    pub average_precipitation: Float,

    /// Seasonality (0.0-1.0)
    pub seasonality: Float,

    /// Climate stability (0.0-1.0)
    pub stability: Float,

    /// Climate trend (0.0 = cooling, 0.5 = stable, 1.0 = warming)
    pub trend: Float,
}

impl Default for Climate {
    fn default() -> Self {
        Climate {
            climate_type: ClimateType::Temperate,
            average_temperature: 288.0,
            temperature_range: 20.0,
            average_precipitation: 1000.0,
            seasonality: 0.5,
            stability: 0.7,
            trend: 0.5,
        }
    }
}

impl Climate {
    /// Create a new climate
    pub fn new(climate_type: ClimateType) -> Self {
        let (avg_temp, temp_range, precip) = match climate_type {
            ClimateType::Tropical => (298.0, 5.0, 2000.0),
            ClimateType::Subtropical => (293.0, 10.0, 1500.0),
            ClimateType::Temperate => (288.0, 20.0, 1000.0),
            ClimateType::Boreal => (278.0, 30.0, 500.0),
            ClimateType::Polar => (263.0, 20.0, 200.0),
            ClimateType::Arid => (303.0, 25.0, 100.0),
            ClimateType::Mediterranean => (293.0, 15.0, 600.0),
        };

        Climate {
            climate_type,
            average_temperature: avg_temp,
            temperature_range: temp_range,
            average_precipitation: precip,
            ..Default::default()
        }
    }

    /// Check if climate is suitable for agriculture
    pub fn suitable_for_agriculture(&self) -> bool {
        self.average_temperature >= 273.0
            && self.average_temperature <= 303.0
            && self.average_precipitation >= 200.0
    }
}

/// Climate types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClimateType {
    Tropical,
    Subtropical,
    Temperate,
    Boreal,
    Polar,
    Arid,
    Mediterranean,
}

/// Weather pattern - Short-term atmospheric conditions
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Weather patterns emerge from
/// atmospheric dynamics and influence biological systems."
#[derive(Debug, Clone, PartialEq)]
pub struct WeatherPattern {
    /// Weather type
    pub weather_type: WeatherType,

    /// Temperature (Kelvin)
    pub temperature: Float,

    /// Precipitation (mm)
    pub precipitation: Float,

    /// Wind speed (m/s)
    pub wind_speed: Float,

    /// Visibility (km)
    pub visibility: Float,

    /// Duration (hours)
    pub duration: Float,

    /// Severity (0.0-1.0)
    pub severity: Float,
}

impl Default for WeatherPattern {
    fn default() -> Self {
        WeatherPattern {
            weather_type: WeatherType::Clear,
            temperature: 288.0,
            precipitation: 0.0,
            wind_speed: 10.0,
            visibility: 10.0,
            duration: 1.0,
            severity: 0.0,
        }
    }
}

impl WeatherPattern {
    /// Create a new weather pattern
    pub fn new(
        weather_type: WeatherType,
        temperature: Float,
        precipitation: Float,
        wind_speed: Float,
    ) -> Self {
        let (visibility, severity) = match weather_type {
            WeatherType::Clear => (10.0, 0.0),
            WeatherType::Cloudy => (8.0, 0.1),
            WeatherType::Rain => (5.0, 0.3),
            WeatherType::Storm => (2.0, 0.7),
            WeatherType::Snow => (3.0, 0.4),
            WeatherType::Fog => (0.5, 0.5),
            WeatherType::Hurricane => (0.1, 1.0),
            WeatherType::Drought => (15.0, 0.6),
        };

        WeatherPattern {
            weather_type,
            temperature,
            precipitation,
            wind_speed,
            visibility,
            duration: 1.0,
            severity,
        }
    }

    /// Check if weather is hazardous
    pub fn is_hazardous(&self) -> bool {
        self.severity > 0.6
    }
}

/// Weather types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
    Hurricane,
    Drought,
}

/// Coupled system - Biology-atmosphere interaction
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Biological systems and atmospheric
/// dynamics are coupled through gas exchange, temperature regulation,
/// and weather patterns."
#[derive(Debug, Clone, PartialEq)]
pub struct CoupledSystem {
    /// Biological oxygen production (0.0-1.0)
    pub oxygen_production: Float,

    /// Biological carbon dioxide consumption (0.0-1.0)
    pub co2_consumption: Float,

    /// Biological water vapor release (0.0-1.0)
    pub water_vapor_release: Float,

    /// Temperature regulation strength (0.0-1.0)
    pub temperature_regulation: Float,

    /// Coupling strength (0.0-1.0)
    pub coupling_strength: Float,
}

impl Default for CoupledSystem {
    fn default() -> Self {
        CoupledSystem {
            oxygen_production: 0.0,
            co2_consumption: 0.0,
            water_vapor_release: 0.0,
            temperature_regulation: 0.0,
            coupling_strength: 0.0,
        }
    }
}

impl CoupledSystem {
    /// Create a new coupled system
    pub fn new(
        oxygen_production: Float,
        co2_consumption: Float,
        water_vapor_release: Float,
        temperature_regulation: Float,
        coupling_strength: Float,
    ) -> Self {
        CoupledSystem {
            oxygen_production: oxygen_production.max(0.0).min(1.0),
            co2_consumption: co2_consumption.max(0.0).min(1.0),
            water_vapor_release: water_vapor_release.max(0.0).min(1.0),
            temperature_regulation: temperature_regulation.max(0.0).min(1.0),
            coupling_strength: coupling_strength.max(0.0).min(1.0),
        }
    }

    /// Calculate overall coupling effect
    pub fn overall_effect(&self) -> Float {
        (self.oxygen_production
            + self.co2_consumption
            + self.water_vapor_release
            + self.temperature_regulation)
            / 4.0
            * self.coupling_strength
    }
}

/// Climate simulator
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The climate simulator models
/// long-term atmospheric patterns and their evolution."
#[derive(Debug, Clone, PartialEq)]
pub struct ClimateSimulator {
    /// Simulation resolution (0.0-1.0)
    resolution: Float,

    /// Time step size
    pub time_step: Float,
}

impl Default for ClimateSimulator {
    fn default() -> Self {
        ClimateSimulator {
            resolution: 0.5,
            time_step: 1.0,
        }
    }
}

impl ClimateSimulator {
    /// Create a new climate simulator
    pub fn new(resolution: Float, time_step: Float) -> Self {
        ClimateSimulator {
            resolution: resolution.max(0.0).min(1.0),
            time_step,
        }
    }

    /// Simulate climate from atmosphere
    ///
    /// Derives climate patterns from atmospheric conditions.
    pub fn simulate_climate(&self, atmosphere: &Atmosphere) -> ClimateSimulationResult {
        // Determine climate type based on temperature
        let climate_type = if atmosphere.temperature > 295.0 {
            if atmosphere.humidity > 0.7 {
                ClimateType::Tropical
            } else {
                ClimateType::Arid
            }
        } else if atmosphere.temperature > 285.0 {
            ClimateType::Temperate
        } else if atmosphere.temperature > 275.0 {
            ClimateType::Boreal
        } else {
            ClimateType::Polar
        };

        let mut climate = Climate::new(climate_type);
        climate.average_temperature = atmosphere.temperature;
        climate.stability = self.resolution;

        ClimateSimulationResult {
            climate,
            success: true,
            reason: None,
        }
    }
}

/// Chemistry modeler
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "The chemistry modeler tracks
/// atmospheric composition and chemical reactions."
#[derive(Debug, Clone, PartialEq)]
pub struct ChemistryModeler {
    /// Reaction rate (0.0-1.0)
    reaction_rate: Float,

    /// Diffusion rate (0.0-1.0)
    diffusion_rate: Float,
}

impl Default for ChemistryModeler {
    fn default() -> Self {
        ChemistryModeler {
            reaction_rate: 0.3,
            diffusion_rate: 0.2,
        }
    }
}

impl ChemistryModeler {
    /// Create a new chemistry modeler
    pub fn new(reaction_rate: Float, diffusion_rate: Float) -> Self {
        ChemistryModeler {
            reaction_rate: reaction_rate.max(0.0).min(1.0),
            diffusion_rate: diffusion_rate.max(0.0).min(1.0),
        }
    }

    /// Model atmospheric chemistry
    ///
    /// Simulates chemical reactions and composition changes.
    pub fn model_chemistry(
        &self,
        atmosphere: &Atmosphere,
        time_delta: Float,
    ) -> ChemistryModelingResult {
        let mut composition = atmosphere.composition.clone();

        // Simulate photosynthesis (CO2 -> O2) based on humidity and temperature
        let photosynthesis_rate = self.reaction_rate
            * atmosphere.humidity
            * ((atmosphere.temperature - 273.0) / 30.0).max(0.0).min(1.0);

        let co2_reduction = composition.carbon_dioxide * photosynthesis_rate * time_delta * 0.01;
        let oxygen_increase = co2_reduction;

        composition.carbon_dioxide -= co2_reduction;
        composition.oxygen += oxygen_increase;

        // Normalize composition
        composition.normalize();

        ChemistryModelingResult {
            chemical_composition: composition,
            success: true,
            reason: None,
        }
    }
}

/// Main atmospheric dynamics orchestrator
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 4:
/// "AtmosphericDynamics: Climate simulation, atmospheric chemistry,
/// weather patterns, climate-biology coupling"
#[derive(Debug, Clone, PartialEq)]
pub struct AtmosphericDynamics {
    /// Climate simulator
    climate_simulator: ClimateSimulator,

    /// Chemistry modeler
    chemistry_modeler: ChemistryModeler,

    /// Configuration
    config: GaiaConfig,
}

impl Default for AtmosphericDynamics {
    fn default() -> Self {
        AtmosphericDynamics {
            climate_simulator: ClimateSimulator::default(),
            chemistry_modeler: ChemistryModeler::default(),
            config: GaiaConfig::default(),
        }
    }
}

impl AtmosphericDynamics {
    /// Create a new atmospheric dynamics system
    pub fn new(
        climate_simulator: ClimateSimulator,
        chemistry_modeler: ChemistryModeler,
        config: GaiaConfig,
    ) -> Self {
        AtmosphericDynamics {
            climate_simulator,
            chemistry_modeler,
            config,
        }
    }

    /// Simulate climate
    pub fn simulate_climate(&self, atmosphere: &Atmosphere) -> ClimateSimulationResult {
        self.climate_simulator.simulate_climate(atmosphere)
    }

    /// Model chemistry
    pub fn model_chemistry(
        &self,
        atmosphere: &Atmosphere,
        time_delta: Float,
    ) -> ChemistryModelingResult {
        self.chemistry_modeler
            .model_chemistry(atmosphere, time_delta)
    }

    /// Predict weather
    ///
    /// Generates short-term weather patterns from climate and atmosphere.
    pub fn predict_weather(
        &self,
        climate: &Climate,
        atmosphere: &Atmosphere,
    ) -> WeatherPredictionResult {
        // Determine weather type based on conditions
        let (weather_type, precipitation, wind_speed, severity) = if atmosphere.cloud_cover > 0.8 {
            if atmosphere.humidity > 0.8 {
                (WeatherType::Storm, 50.0, 30.0, 0.7)
            } else {
                (WeatherType::Cloudy, 0.0, 15.0, 0.1)
            }
        } else if atmosphere.cloud_cover > 0.5 {
            if atmosphere.humidity > 0.6 {
                (WeatherType::Rain, 20.0, 15.0, 0.3)
            } else {
                (WeatherType::Cloudy, 0.0, 10.0, 0.1)
            }
        } else if atmosphere.temperature < 273.0 && atmosphere.humidity > 0.5 {
            (WeatherType::Snow, 10.0, 10.0, 0.4)
        } else if atmosphere.humidity < 0.3 {
            (WeatherType::Clear, 0.0, 5.0, 0.0)
        } else if atmosphere.visibility < 1.0 {
            (WeatherType::Fog, 0.0, 2.0, 0.5)
        } else {
            (WeatherType::Clear, 0.0, 8.0, 0.0)
        };

        let weather = WeatherPattern::new(
            weather_type,
            atmosphere.temperature,
            precipitation,
            wind_speed,
        );

        let success = matches!(weather_type, WeatherType::Clear | WeatherType::Cloudy)
            || severity <= self.config.weather_accuracy;

        WeatherPredictionResult {
            weather_pattern: weather,
            success,
            reason: if success {
                None
            } else {
                Some("Weather prediction uncertainty too high".to_string())
            },
        }
    }

    /// Couple with biology
    ///
    /// Simulates the interaction between atmospheric dynamics and biological systems.
    pub fn couple_with_biology(
        &self,
        atmosphere: &Atmosphere,
        biomass: Float,      // 0.0-1.0
        biodiversity: Float, // 0.0-1.0
    ) -> CouplingResult {
        // Calculate biological effects on atmosphere
        let oxygen_production = biomass * biodiversity * self.config.chemistry_rate;
        let co2_consumption = biomass * biodiversity * self.config.chemistry_rate * 0.8;
        let water_vapor_release = biomass * atmosphere.humidity * self.config.chemistry_rate * 0.5;
        let temperature_regulation = biomass * biodiversity * self.config.coupling_strength;

        let coupled_system = CoupledSystem::new(
            oxygen_production,
            co2_consumption,
            water_vapor_release,
            temperature_regulation,
            self.config.coupling_strength,
        );

        let success = coupled_system.overall_effect() > 0.1;

        CouplingResult {
            coupled_system,
            success,
            reason: if success {
                None
            } else {
                Some("Insufficient biological coupling".to_string())
            },
        }
    }
}

/// Result of climate simulation
#[derive(Debug, Clone, PartialEq)]
pub struct ClimateSimulationResult {
    /// Climate simulated
    pub climate: Climate,

    /// Whether simulation was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of chemistry modeling
#[derive(Debug, Clone, PartialEq)]
pub struct ChemistryModelingResult {
    /// Chemical composition modeled
    pub chemical_composition: ChemicalComposition,

    /// Whether modeling was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of weather prediction
#[derive(Debug, Clone, PartialEq)]
pub struct WeatherPredictionResult {
    /// Weather pattern predicted
    pub weather_pattern: WeatherPattern,

    /// Whether prediction was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

/// Result of biology-atmosphere coupling
#[derive(Debug, Clone, PartialEq)]
pub struct CouplingResult {
    /// Coupled system simulated
    pub coupled_system: CoupledSystem,

    /// Whether coupling was successful
    pub success: bool,

    /// Reason for failure (if any)
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atmosphere_default() {
        let atmosphere = Atmosphere::default();
        assert_eq!(atmosphere.temperature, 288.0);
        assert_eq!(atmosphere.pressure, 1.0);
    }

    #[test]
    fn test_atmosphere_new() {
        let atmosphere = Atmosphere::new(123);
        assert_eq!(atmosphere.atmosphere_id, 123);
    }

    #[test]
    fn test_atmosphere_density() {
        let mut atmosphere = Atmosphere::new(1);
        atmosphere.temperature = 288.0;
        atmosphere.pressure = 1.0;
        let density = atmosphere.density();
        assert_eq!(density, 1.225); // Standard density
    }

    #[test]
    fn test_chemical_composition_default() {
        let composition = ChemicalComposition::default();
        assert_eq!(composition.nitrogen, 0.78);
        assert_eq!(composition.oxygen, 0.21);
    }

    #[test]
    fn test_chemical_composition_total() {
        let composition = ChemicalComposition::default();
        let total = composition.total();
        assert!((total - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_chemical_composition_normalize() {
        let mut composition = ChemicalComposition::new(0.5, 0.3, 0.1, 0.05, 0.03, 0.02);
        composition.normalize();
        let total = composition.total();
        assert!((total - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_chemical_composition_supports_aerobic_life() {
        let composition = ChemicalComposition::default();
        assert!(composition.supports_aerobic_life());

        let mut low_oxygen = ChemicalComposition::default();
        low_oxygen.oxygen = 0.1;
        assert!(!low_oxygen.supports_aerobic_life());
    }

    #[test]
    fn test_climate_default() {
        let climate = Climate::default();
        assert_eq!(climate.climate_type, ClimateType::Temperate);
        assert_eq!(climate.average_temperature, 288.0);
    }

    #[test]
    fn test_climate_new() {
        let climate = Climate::new(ClimateType::Tropical);
        assert_eq!(climate.climate_type, ClimateType::Tropical);
        assert_eq!(climate.average_temperature, 298.0);
    }

    #[test]
    fn test_climate_suitable_for_agriculture() {
        let climate = Climate::new(ClimateType::Temperate);
        assert!(climate.suitable_for_agriculture());

        let polar = Climate::new(ClimateType::Polar);
        assert!(!polar.suitable_for_agriculture());
    }

    #[test]
    fn test_weather_pattern_default() {
        let weather = WeatherPattern::default();
        assert_eq!(weather.weather_type, WeatherType::Clear);
    }

    #[test]
    fn test_weather_pattern_new() {
        let weather = WeatherPattern::new(WeatherType::Storm, 290.0, 50.0, 30.0);
        assert_eq!(weather.weather_type, WeatherType::Storm);
        assert_eq!(weather.severity, 0.7);
    }

    #[test]
    fn test_weather_pattern_is_hazardous() {
        let storm = WeatherPattern::new(WeatherType::Storm, 290.0, 50.0, 30.0);
        assert!(storm.is_hazardous());

        let clear = WeatherPattern::new(WeatherType::Clear, 290.0, 0.0, 5.0);
        assert!(!clear.is_hazardous());
    }

    #[test]
    fn test_coupled_system_new() {
        let coupled = CoupledSystem::new(0.8, 0.7, 0.6, 0.5, 0.4);
        assert_eq!(coupled.oxygen_production, 0.8);
        assert_eq!(coupled.coupling_strength, 0.4);
    }

    #[test]
    fn test_coupled_system_overall_effect() {
        let coupled = CoupledSystem::new(0.8, 0.7, 0.6, 0.5, 0.4);
        let effect = coupled.overall_effect();
        assert_eq!(effect, (0.8 + 0.7 + 0.6 + 0.5) / 4.0 * 0.4);
    }

    #[test]
    fn test_climate_simulator_default() {
        let simulator = ClimateSimulator::default();
        assert_eq!(simulator.resolution, 0.5);
    }

    #[test]
    fn test_simulate_climate() {
        let simulator = ClimateSimulator::default();
        let mut atmosphere = Atmosphere::new(1);
        atmosphere.temperature = 298.0;
        atmosphere.humidity = 0.8;

        let result = simulator.simulate_climate(&atmosphere);
        assert!(result.success);
        assert_eq!(result.climate.climate_type, ClimateType::Tropical);
    }

    #[test]
    fn test_chemistry_modeler_default() {
        let modeler = ChemistryModeler::default();
        assert_eq!(modeler.reaction_rate, 0.3);
    }

    #[test]
    fn test_model_chemistry() {
        let modeler = ChemistryModeler::default();
        let mut atmosphere = Atmosphere::new(1);
        atmosphere.temperature = 298.0;
        atmosphere.humidity = 0.8;

        let result = modeler.model_chemistry(&atmosphere, 1.0);
        assert!(result.success);
        // CO2 should decrease due to photosynthesis
        assert!(result.chemical_composition.carbon_dioxide < atmosphere.composition.carbon_dioxide);
    }

    #[test]
    fn test_atmospheric_dynamics_default() {
        let dynamics = AtmosphericDynamics::default();
        assert_eq!(dynamics.config.climate_resolution, 0.5);
    }

    #[test]
    fn test_atmospheric_dynamics_simulate_climate() {
        let dynamics = AtmosphericDynamics::default();
        let mut atmosphere = Atmosphere::new(1);
        atmosphere.temperature = 288.0;

        let result = dynamics.simulate_climate(&atmosphere);
        assert!(result.success);
    }

    #[test]
    fn test_atmospheric_dynamics_model_chemistry() {
        let dynamics = AtmosphericDynamics::default();
        let atmosphere = Atmosphere::new(1);

        let result = dynamics.model_chemistry(&atmosphere, 1.0);
        assert!(result.success);
    }

    #[test]
    fn test_atmospheric_dynamics_predict_weather() {
        let dynamics = AtmosphericDynamics::default();
        let climate = Climate::new(ClimateType::Temperate);
        let mut atmosphere = Atmosphere::new(1);
        atmosphere.cloud_cover = 0.3;
        atmosphere.humidity = 0.4;
        atmosphere.temperature = 288.0;

        let result = dynamics.predict_weather(&climate, &atmosphere);
        assert!(result.success);
    }

    #[test]
    fn test_atmospheric_dynamics_couple_with_biology() {
        let dynamics = AtmosphericDynamics::default();
        let atmosphere = Atmosphere::new(1);

        let result = dynamics.couple_with_biology(&atmosphere, 0.9, 0.9);
        assert!(result.success);
    }

    #[test]
    fn test_gaia_config_default() {
        let config = GaiaConfig::default();
        assert_eq!(config.climate_resolution, 0.5);
    }

    #[test]
    fn test_gaia_config_validate() {
        let config = GaiaConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = GaiaConfig {
            climate_resolution: 1.5,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }
}
