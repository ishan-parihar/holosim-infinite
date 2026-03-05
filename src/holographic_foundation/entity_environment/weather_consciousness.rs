//! Weather-Consciousness State Coupling
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Weather Interaction = Atmospheric field dynamics affecting consciousness"
//!
//! # Key Insight
//!
//! Weather is NOT just physical phenomena but atmospheric field dynamics
//! that directly affect entity consciousness states. Different weather
//! conditions have different field resonance patterns.

use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherState {
    Clear,
    Cloudy,
    Overcast,
    Rain,
    Storm,
    Snow,
    Fog,
    Wind,
    Drought,
    Flood,
    Aurora,
    Eclipse,
}

impl WeatherState {
    pub fn consciousness_modifier(&self) -> Float {
        match self {
            WeatherState::Clear => 1.0,
            WeatherState::Cloudy => 0.95,
            WeatherState::Overcast => 0.85,
            WeatherState::Rain => 0.9,
            WeatherState::Storm => 0.7,
            WeatherState::Snow => 0.88,
            WeatherState::Fog => 0.75,
            WeatherState::Wind => 0.92,
            WeatherState::Drought => 0.65,
            WeatherState::Flood => 0.6,
            WeatherState::Aurora => 1.3,
            WeatherState::Eclipse => 1.5,
        }
    }

    pub fn field_coherence(&self) -> Float {
        match self {
            WeatherState::Clear => 0.9,
            WeatherState::Cloudy => 0.8,
            WeatherState::Overcast => 0.7,
            WeatherState::Rain => 0.75,
            WeatherState::Storm => 0.5,
            WeatherState::Snow => 0.78,
            WeatherState::Fog => 0.55,
            WeatherState::Wind => 0.72,
            WeatherState::Drought => 0.4,
            WeatherState::Flood => 0.45,
            WeatherState::Aurora => 0.95,
            WeatherState::Eclipse => 1.0,
        }
    }

    pub fn emotional_resonance(&self) -> Float {
        match self {
            WeatherState::Clear => 0.8,
            WeatherState::Cloudy => 0.6,
            WeatherState::Overcast => 0.4,
            WeatherState::Rain => 0.5,
            WeatherState::Storm => 0.2,
            WeatherState::Snow => 0.65,
            WeatherState::Fog => 0.35,
            WeatherState::Wind => 0.55,
            WeatherState::Drought => 0.15,
            WeatherState::Flood => 0.1,
            WeatherState::Aurora => 0.95,
            WeatherState::Eclipse => 1.0,
        }
    }

    pub fn energy_cost(&self) -> Float {
        match self {
            WeatherState::Clear => 0.0,
            WeatherState::Cloudy => 0.02,
            WeatherState::Overcast => 0.05,
            WeatherState::Rain => 0.03,
            WeatherState::Storm => 0.15,
            WeatherState::Snow => 0.08,
            WeatherState::Fog => 0.05,
            WeatherState::Wind => 0.04,
            WeatherState::Drought => 0.12,
            WeatherState::Flood => 0.18,
            WeatherState::Aurora => -0.05,
            WeatherState::Eclipse => -0.1,
        }
    }

    pub fn veil_transparency_effect(&self) -> Float {
        match self {
            WeatherState::Clear => 0.0,
            WeatherState::Cloudy => 0.02,
            WeatherState::Overcast => 0.05,
            WeatherState::Rain => 0.08,
            WeatherState::Storm => 0.15,
            WeatherState::Snow => 0.07,
            WeatherState::Fog => 0.12,
            WeatherState::Wind => 0.04,
            WeatherState::Drought => 0.03,
            WeatherState::Flood => 0.1,
            WeatherState::Aurora => 0.25,
            WeatherState::Eclipse => 0.4,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            WeatherState::Clear => "Clear skies with optimal field coherence",
            WeatherState::Cloudy => "Partial cloud cover with moderate field",
            WeatherState::Overcast => "Heavy cloud cover reducing field clarity",
            WeatherState::Rain => "Precipitation cleansing field patterns",
            WeatherState::Storm => "Chaotic field dynamics from atmospheric turbulence",
            WeatherState::Snow => "Crystalline field patterns from frozen water",
            WeatherState::Fog => "Dense field obscuring perception",
            WeatherState::Wind => "Moving field patterns from air currents",
            WeatherState::Drought => "Depleted field energy from aridity",
            WeatherState::Flood => "Overwhelming field from excess water",
            WeatherState::Aurora => "Luminous field amplification at polar regions",
            WeatherState::Eclipse => "Rare field alignment opening gateways",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeatherField {
    pub current_state: WeatherState,
    pub intensity: Float,
    pub duration: Float,
    pub transition_progress: Float,
    pub previous_state: Option<WeatherState>,
    pub field_coherence: Float,
    pub affected_area: Float,
}

impl WeatherField {
    pub fn new(initial_state: WeatherState) -> Self {
        Self {
            current_state: initial_state,
            intensity: 0.5,
            duration: 0.0,
            transition_progress: 0.0,
            previous_state: None,
            field_coherence: initial_state.field_coherence(),
            affected_area: 100.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.duration += dt;

        if self.transition_progress > 0.0 && self.transition_progress < 1.0 {
            self.transition_progress = (self.transition_progress + 0.1 * dt).min(1.0);
            self.interpolate_coherence();
        }

        let coherence_drift = 0.01 * dt * (0.5 - self.intensity).sin();
        self.field_coherence = (self.field_coherence + coherence_drift).clamp(0.1, 1.0);
    }

    fn interpolate_coherence(&mut self) {
        let current_target = self.current_state.field_coherence();
        let previous = self
            .previous_state
            .map(|s| s.field_coherence())
            .unwrap_or(current_target);

        self.field_coherence = previous + (current_target - previous) * self.transition_progress;
    }

    pub fn transition_to(&mut self, new_state: WeatherState) {
        if new_state != self.current_state {
            self.previous_state = Some(self.current_state);
            self.current_state = new_state;
            self.transition_progress = 0.0;
            self.duration = 0.0;
        }
    }

    pub fn set_intensity(&mut self, intensity: Float) {
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    pub fn consciousness_effect(&self) -> Float {
        self.current_state.consciousness_modifier() * self.intensity
    }

    pub fn veil_effect(&self) -> Float {
        self.current_state.veil_transparency_effect() * self.intensity
    }
}

#[derive(Debug, Clone)]
pub struct AtmosphericInfluence {
    pub weather_field: WeatherField,
    pub temperature: Float,
    pub humidity: Float,
    pub pressure: Float,
    pub wind_speed: Float,
    pub wind_direction: Float,
}

impl AtmosphericInfluence {
    pub fn new(weather_state: WeatherState) -> Self {
        Self {
            weather_field: WeatherField::new(weather_state),
            temperature: 20.0,
            humidity: 0.5,
            pressure: 1.0,
            wind_speed: 0.0,
            wind_direction: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.weather_field.update(dt);

        let temp_drift = 0.1 * dt * (self.weather_field.duration * 0.1).sin();
        self.temperature += temp_drift;

        let humidity_drift = 0.05 * dt * (self.weather_field.duration * 0.15).cos();
        self.humidity = (self.humidity + humidity_drift).clamp(0.0, 1.0);
    }

    pub fn comfort_level(&self) -> Float {
        let temp_factor = 1.0 - (self.temperature - 20.0).abs() / 30.0;
        let humidity_factor = 1.0 - (self.humidity - 0.5).abs();
        let weather_factor = self.weather_field.consciousness_effect();

        (temp_factor * 0.3 + humidity_factor * 0.3 + weather_factor * 0.4).clamp(0.0, 1.0)
    }

    pub fn total_field_influence(&self) -> Float {
        self.weather_field.field_coherence
            * (1.0 + self.pressure * 0.1)
            * (1.0 - self.wind_speed * 0.05)
    }
}

#[derive(Debug, Clone)]
pub struct ConsciousnessWeatherCoupling {
    pub entity_id: u64,
    pub atmospheric_influence: AtmosphericInfluence,
    pub consciousness_modifier: Float,
    pub emotional_state: Float,
    pub energy_drain: Float,
    pub veil_perception_change: Float,
    pub adaptation_level: Float,
    pub weather_memory: HashMap<WeatherState, Float>,
}

impl ConsciousnessWeatherCoupling {
    pub fn new(entity_id: u64, initial_weather: WeatherState) -> Self {
        let atmospheric_influence = AtmosphericInfluence::new(initial_weather);

        Self {
            entity_id,
            atmospheric_influence,
            consciousness_modifier: 1.0,
            emotional_state: 0.5,
            energy_drain: 0.0,
            veil_perception_change: 0.0,
            adaptation_level: 0.5,
            weather_memory: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: Float, _base_consciousness: Float) {
        self.atmospheric_influence.update(dt);

        let weather = self.atmospheric_influence.weather_field.current_state;
        let weather_modifier = weather.consciousness_modifier();
        let intensity = self.atmospheric_influence.weather_field.intensity;

        self.consciousness_modifier =
            1.0 + (weather_modifier - 1.0) * intensity * self.adaptation_level;

        let target_emotional = weather.emotional_resonance();
        self.emotional_state = self.emotional_state * 0.95 + target_emotional * 0.05;

        self.energy_drain = weather.energy_cost() * intensity * (1.0 - self.adaptation_level) * dt;

        self.veil_perception_change = weather.veil_transparency_effect() * intensity * dt;

        let memory_entry = self.weather_memory.entry(weather).or_insert(0.0);
        *memory_entry += dt;

        self.adaptation_level = (self.adaptation_level + 0.0001 * dt).min(1.0);

        if let Some(memory) = self.weather_memory.get(&weather) {
            self.adaptation_level = (self.adaptation_level + *memory * 0.00001 * dt).min(1.0);
        }
    }

    pub fn change_weather(&mut self, new_weather: WeatherState) {
        self.atmospheric_influence
            .weather_field
            .transition_to(new_weather);
        self.adaptation_level *= 0.8;
    }

    pub fn effective_consciousness(&self, base: Float) -> Float {
        base * self.consciousness_modifier * (1.0 - self.energy_drain * 0.1)
    }

    pub fn preferred_weather(&self) -> Option<WeatherState> {
        self.weather_memory
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| *k)
    }

    pub fn adaptation_to_weather(&self, weather: WeatherState) -> Float {
        let time_experienced = self.weather_memory.get(&weather).copied().unwrap_or(0.0);
        (time_experienced / 100.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_state_consciousness_modifier() {
        assert!(WeatherState::Aurora.consciousness_modifier() > 1.0);
        assert!(WeatherState::Clear.consciousness_modifier() == 1.0);
        assert!(WeatherState::Storm.consciousness_modifier() < 1.0);
    }

    #[test]
    fn test_weather_state_field_coherence() {
        assert!(WeatherState::Clear.field_coherence() > WeatherState::Storm.field_coherence());
    }

    #[test]
    fn test_weather_state_emotional_resonance() {
        assert!(
            WeatherState::Aurora.emotional_resonance() > WeatherState::Flood.emotional_resonance()
        );
    }

    #[test]
    fn test_weather_state_energy_cost() {
        assert!(WeatherState::Aurora.energy_cost() < 0.0);
        assert!(WeatherState::Storm.energy_cost() > 0.0);
    }

    #[test]
    fn test_weather_state_veil_effect() {
        assert!(
            WeatherState::Eclipse.veil_transparency_effect()
                > WeatherState::Clear.veil_transparency_effect()
        );
    }

    #[test]
    fn test_weather_field_creation() {
        let field = WeatherField::new(WeatherState::Clear);
        assert_eq!(field.current_state, WeatherState::Clear);
        assert!(field.field_coherence > 0.0);
    }

    #[test]
    fn test_weather_field_update() {
        let mut field = WeatherField::new(WeatherState::Clear);
        field.update(1.0);
        assert!(field.duration > 0.0);
    }

    #[test]
    fn test_weather_field_transition() {
        let mut field = WeatherField::new(WeatherState::Clear);
        field.transition_to(WeatherState::Storm);

        assert_eq!(field.current_state, WeatherState::Storm);
        assert_eq!(field.previous_state, Some(WeatherState::Clear));
    }

    #[test]
    fn test_weather_field_consciousness_effect() {
        let mut field = WeatherField::new(WeatherState::Aurora);
        field.intensity = 1.0;
        let effect = field.consciousness_effect();
        assert!(effect > 1.0);
    }

    #[test]
    fn test_atmospheric_influence_creation() {
        let atm = AtmosphericInfluence::new(WeatherState::Clear);
        assert!(atm.comfort_level() > 0.0);
    }

    #[test]
    fn test_atmospheric_influence_update() {
        let mut atm = AtmosphericInfluence::new(WeatherState::Clear);
        atm.update(1.0);
        assert!(atm.weather_field.duration > 0.0);
    }

    #[test]
    fn test_consciousness_weather_coupling_creation() {
        let coupling = ConsciousnessWeatherCoupling::new(1, WeatherState::Clear);
        assert_eq!(coupling.entity_id, 1);
        assert!(coupling.consciousness_modifier > 0.0);
    }

    #[test]
    fn test_consciousness_weather_coupling_update() {
        let mut coupling = ConsciousnessWeatherCoupling::new(1, WeatherState::Clear);
        coupling.update(1.0, 0.8);

        assert!(coupling.atmospheric_influence.weather_field.duration > 0.0);
        assert!(!coupling.weather_memory.is_empty());
    }

    #[test]
    fn test_consciousness_weather_coupling_change() {
        let mut coupling = ConsciousnessWeatherCoupling::new(1, WeatherState::Clear);
        coupling.change_weather(WeatherState::Storm);

        assert_eq!(
            coupling.atmospheric_influence.weather_field.current_state,
            WeatherState::Storm
        );
    }

    #[test]
    fn test_consciousness_weather_coupling_effective() {
        let coupling = ConsciousnessWeatherCoupling::new(1, WeatherState::Clear);
        let effective = coupling.effective_consciousness(0.8);
        assert!(effective > 0.0);
    }

    #[test]
    fn test_consciousness_weather_coupling_preferred() {
        let mut coupling = ConsciousnessWeatherCoupling::new(1, WeatherState::Clear);
        coupling.weather_memory.insert(WeatherState::Aurora, 100.0);
        coupling.weather_memory.insert(WeatherState::Storm, 10.0);

        let preferred = coupling.preferred_weather();
        assert_eq!(preferred, Some(WeatherState::Aurora));
    }
}
