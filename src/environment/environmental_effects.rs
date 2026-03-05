//! Environmental Effects on Entities
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.2:
//! "Entity experiences weather, terrain, atmosphere"
//! "Weather affects mood/state"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "Entities are not separate from their environment"
//! > "The environment affects entity consciousness through field coherence"
//!
//! This module implements:
//! - Weather effects on entity state
//! - Terrain effects on movement and metabolism
//! - Temperature effects on biology
//! - Atmospheric effects on consciousness

use super::*;
use crate::hpo::planetary_emergence::{WeatherPattern as PlanetWeather, WeatherSystem};

/// Configuration for environmental effects
#[derive(Debug, Clone)]
pub struct EnvironmentalEffectsConfig {
    /// Enable weather effects
    pub enable_weather_effects: bool,

    /// Enable terrain effects
    pub enable_terrain_effects: bool,

    /// Enable temperature effects
    pub enable_temperature_effects: bool,

    /// Enable atmospheric effects
    pub enable_atmosphere_effects: bool,

    /// Base metabolic rate
    pub base_metabolic_rate: f64,

    /// Temperature deviation threshold
    pub temp_deviation_threshold: f64,
}

impl Default for EnvironmentalEffectsConfig {
    fn default() -> Self {
        EnvironmentalEffectsConfig {
            enable_weather_effects: true,
            enable_terrain_effects: true,
            enable_temperature_effects: true,
            enable_atmosphere_effects: true,
            base_metabolic_rate: 1.0,
            temp_deviation_threshold: 10.0, // Celsius
        }
    }
}

/// Entity mutable state affected by environment
#[derive(Debug, Clone)]
pub struct EntityMutableState {
    /// Metabolic rate multiplier
    pub metabolism_rate: f64,

    /// Consciousness state (affected by environment)
    pub consciousness_state: ConsciousnessState,

    /// Energy level
    pub energy: f64,

    /// Mood modifier
    pub mood_modifier: f64,

    /// Movement efficiency
    pub movement_efficiency: f64,

    /// Perception clarity
    pub perception_clarity: f64,
}

/// Consciousness states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ConsciousnessState {
    #[default]
    Calm,
    Alert,
    Agitated,
    Restless,
    Serene,
    Anxious,
}


/// Environmental Effects Applicator
///
/// Applies environmental conditions to entity state
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "The entity contains within it all densities and sub-densities of the octave"
/// > "Environment affects entity through field coherence patterns"
#[derive(Debug, Clone)]
pub struct EnvironmentalEffects {
    config: EnvironmentalEffectsConfig,
    statistics: EnvironmentalEffectsStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct EnvironmentalEffectsStatistics {
    pub entities_affected: usize,
    pub weather_effects_applied: usize,
    pub terrain_effects_applied: usize,
    pub temperature_effects_applied: usize,
    pub atmosphere_effects_applied: usize,
    pub average_mood_modifier: f64,
    pub average_metabolism_change: f64,
}

impl EnvironmentalEffects {
    pub fn new() -> Self {
        EnvironmentalEffects {
            config: EnvironmentalEffectsConfig::default(),
            statistics: EnvironmentalEffectsStatistics::default(),
        }
    }

    pub fn with_config(config: EnvironmentalEffectsConfig) -> Self {
        EnvironmentalEffects {
            config,
            statistics: EnvironmentalEffectsStatistics::default(),
        }
    }

    /// Apply all environmental effects to entity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Entities experience their environment through field coherence"
    pub fn apply_effects(
        &mut self,
        entity_state: &mut EntityEnvironmentState,
        entity_mutable: &mut EntityMutableState,
    ) {
        self.statistics.entities_affected += 1;

        // Apply weather effects
        if self.config.enable_weather_effects {
            self.apply_weather_effects(entity_state, entity_mutable);
            self.statistics.weather_effects_applied += 1;
        }

        // Apply terrain effects
        if self.config.enable_terrain_effects {
            self.apply_terrain_effects(entity_state, entity_mutable);
            self.statistics.terrain_effects_applied += 1;
        }

        // Apply temperature effects
        if self.config.enable_temperature_effects {
            self.apply_temperature_effects(entity_state, entity_mutable);
            self.statistics.temperature_effects_applied += 1;
        }

        // Apply atmosphere effects
        if self.config.enable_atmosphere_effects {
            self.apply_atmosphere_effects(entity_state, entity_mutable);
            self.statistics.atmosphere_effects_applied += 1;
        }
    }

    /// Apply weather effects on entity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Weather affects mood/state"
    fn apply_weather_effects(
        &self,
        env_state: &EntityEnvironmentState,
        entity_mutable: &mut EntityMutableState,
    ) {
        match env_state.weather {
            WeatherPattern::Clear => {
                // Clear weather - calm, alert, good perception
                entity_mutable.mood_modifier += 0.1;
                entity_mutable.perception_clarity += 0.1;
                if entity_mutable.consciousness_state == ConsciousnessState::Anxious {
                    entity_mutable.consciousness_state = ConsciousnessState::Calm;
                }
            }
            WeatherPattern::Cloudy => {
                // Cloudy - slightly subdued
                entity_mutable.mood_modifier += 0.0;
                entity_mutable.perception_clarity -= 0.05;
            }
            WeatherPattern::Rain => {
                // Rain - introspective, calm
                entity_mutable.mood_modifier -= 0.05;
                entity_mutable.consciousness_state =
                    if entity_mutable.consciousness_state == ConsciousnessState::Agitated {
                        ConsciousnessState::Calm
                    } else {
                        entity_mutable.consciousness_state
                    };
            }
            WeatherPattern::Storm => {
                // Storm - agitated, anxious
                entity_mutable.mood_modifier -= 0.2;
                entity_mutable.consciousness_state = ConsciousnessState::Agitated;
                entity_mutable.perception_clarity -= 0.15;
            }
            WeatherPattern::Snow => {
                // Snow - serene, but cold
                entity_mutable.mood_modifier += 0.05;
                entity_mutable.consciousness_state = ConsciousnessState::Serene;
            }
            WeatherPattern::Fog => {
                // Fog - confusion, reduced perception
                entity_mutable.perception_clarity -= 0.2;
                entity_mutable.mood_modifier -= 0.1;
            }
        }
    }

    /// Apply terrain effects on entity
    fn apply_terrain_effects(
        &self,
        env_state: &EntityEnvironmentState,
        entity_mutable: &mut EntityMutableState,
    ) {
        match env_state.terrain {
            EnvironmentTerrain::Ocean => {
                // Ocean - swimming, high energy cost
                entity_mutable.movement_efficiency = 0.5;
                entity_mutable.metabolism_rate *= 1.5;
            }
            EnvironmentTerrain::Plains => {
                // Plains - easy movement
                entity_mutable.movement_efficiency = 1.0;
                entity_mutable.metabolism_rate *= 1.0;
            }
            EnvironmentTerrain::Hills => {
                // Hills - moderate difficulty
                entity_mutable.movement_efficiency = 0.8;
                entity_mutable.metabolism_rate *= 1.3;
            }
            EnvironmentTerrain::Mountains => {
                // Mountains - difficult
                entity_mutable.movement_efficiency = 0.5;
                entity_mutable.metabolism_rate *= 1.6;
            }
            EnvironmentTerrain::Forest => {
                // Forest - navigation required
                entity_mutable.movement_efficiency = 0.7;
                entity_mutable.metabolism_rate *= 1.2;
            }
            EnvironmentTerrain::Desert => {
                // Desert - high heat, energy drain
                entity_mutable.movement_efficiency = 0.6;
                entity_mutable.metabolism_rate *= 1.4;
                entity_mutable.mood_modifier -= 0.1;
            }
            EnvironmentTerrain::Tundra => {
                // Tundra - cold, slow
                entity_mutable.movement_efficiency = 0.7;
                entity_mutable.metabolism_rate *= 1.5;
            }
            EnvironmentTerrain::Volcanic => {
                // Volcanic - dangerous, stressful
                entity_mutable.movement_efficiency = 0.4;
                entity_mutable.metabolism_rate *= 1.8;
                entity_mutable.consciousness_state = ConsciousnessState::Anxious;
            }
        }
    }

    /// Apply temperature effects on entity
    fn apply_temperature_effects(
        &self,
        env_state: &EntityEnvironmentState,
        entity_mutable: &mut EntityMutableState,
    ) {
        let optimal_temp = 20.0; // Celsius
        let deviation = (env_state.temperature - optimal_temp).abs();

        if deviation > self.config.temp_deviation_threshold {
            // Outside optimal range - metabolism affected
            let metabolic_change = deviation / 20.0;
            entity_mutable.metabolism_rate *= 1.0 + metabolic_change;

            // Mood affected by extreme temperatures
            if deviation > 20.0 {
                entity_mutable.mood_modifier -= 0.15;
                if env_state.temperature > 40.0 {
                    entity_mutable.consciousness_state = ConsciousnessState::Agitated;
                } else if env_state.temperature < 0.0 {
                    entity_mutable.consciousness_state = ConsciousnessState::Restless;
                }
            }
        }
    }

    /// Apply atmosphere effects on entity
    fn apply_atmosphere_effects(
        &self,
        env_state: &EntityEnvironmentState,
        entity_mutable: &mut EntityMutableState,
    ) {
        let optimal_oxygen = 0.21;
        let oxygen_deviation = (env_state.oxygen_level - optimal_oxygen).abs();

        if oxygen_deviation > 0.05 {
            // Oxygen level affects clarity and energy
            let effect = oxygen_deviation * 2.0;
            entity_mutable.perception_clarity *= 1.0 - effect;
            entity_mutable.energy *= 1.0 - effect;

            // Low oxygen causes anxiety
            if env_state.oxygen_level < 0.15 {
                entity_mutable.consciousness_state = ConsciousnessState::Anxious;
            }
        }
    }

    /// Update weather for entity from planet weather system
    pub fn update_weather(
        &self,
        entity_state: &mut EntityEnvironmentState,
        weather_system: &WeatherSystem,
    ) {
        // Convert planet weather to entity weather
        let entity_weather = match weather_system.pattern {
            PlanetWeather::Clear => WeatherPattern::Clear,
            PlanetWeather::PartlyCloudy => WeatherPattern::Cloudy,
            PlanetWeather::Cloudy => WeatherPattern::Cloudy,
            PlanetWeather::Rain => WeatherPattern::Rain,
            PlanetWeather::Storm => WeatherPattern::Storm,
            PlanetWeather::Snow => WeatherPattern::Snow,
            _ => WeatherPattern::Clear,
        };

        entity_state.weather = entity_weather;
    }

    /// Create default mutable state
    pub fn create_default_mutable_state() -> EntityMutableState {
        EntityMutableState {
            metabolism_rate: 1.0,
            consciousness_state: ConsciousnessState::Calm,
            energy: 1.0,
            mood_modifier: 0.0,
            movement_efficiency: 1.0,
            perception_clarity: 1.0,
        }
    }

    /// Update statistics
    pub fn update_statistics(&mut self, affected_count: usize) {
        self.statistics.entities_affected = affected_count;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &EnvironmentalEffectsStatistics {
        &self.statistics
    }
}

impl Default for EnvironmentalEffects {
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
    fn test_weather_effects() {
        let effects = EnvironmentalEffects::new();

        // Test clear weather
        let env_state = EntityEnvironmentState {
            weather: WeatherPattern::Clear,
            ..Default::default()
        };
        let mut mutable_state = EnvironmentalEffects::create_default_mutable_state();

        effects.apply_weather_effects(&env_state, &mut mutable_state);

        assert!(mutable_state.mood_modifier > 0.0);
    }

    #[test]
    fn test_terrain_effects() {
        let effects = EnvironmentalEffects::new();

        // Test mountains
        let env_state = EntityEnvironmentState {
            terrain: EnvironmentTerrain::Mountains,
            ..Default::default()
        };
        let mut mutable_state = EnvironmentalEffects::create_default_mutable_state();

        effects.apply_terrain_effects(&env_state, &mut mutable_state);

        assert!(mutable_state.movement_efficiency < 1.0);
        assert!(mutable_state.metabolism_rate > 1.0);
    }

    #[test]
    fn test_temperature_effects() {
        let effects = EnvironmentalEffects::new();

        // Test extreme heat
        let env_state = EntityEnvironmentState {
            temperature: 45.0,
            ..Default::default()
        };
        let mut mutable_state = EnvironmentalEffects::create_default_mutable_state();

        effects.apply_temperature_effects(&env_state, &mut mutable_state);

        assert!(mutable_state.metabolism_rate > 1.0);
    }
}
