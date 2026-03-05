//! Unified Field Equation - Integration of Three Primal Distortions
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Unified Field Equation combines all three primal distortions:
//!  ∂ψ/∂t = FreeWill(ψ) + Love(ψ) + Light(ψ)
//!
//!  Free Will breaks symmetry (creates possibility)
//!  Love creates structure (coherence attraction)
//!  Light enables propagation (information transfer)
//!
//!  Together they create the manifest universe from unified potential."
//!
//! This is the PRIMARY FIELD from which everything emerges!
//! Matter, consciousness, space, time - all unfold from this unified field.

use std::collections::HashMap;

use super::free_will::{FreeWillConfig, FreeWillTerm};
use super::light::{InterferencePattern, LightConfig, LightTerm};
use super::love::{LoveConfig, LoveTerm};
use super::{DensityAmplitude, DistortionType, FieldState, NUM_DENSITY_BANDS};

#[derive(Debug, Clone)]
pub struct UnifiedFieldConfig {
    pub time_step: f64,
    pub free_will: FreeWillConfig,
    pub love: LoveConfig,
    pub light: LightConfig,
    pub coherence_peak_threshold: f64,
    pub coherence_peak_min_distance: f64,
    pub enable_entity_emergence: bool,
}

impl Default for UnifiedFieldConfig {
    fn default() -> Self {
        Self {
            time_step: 0.01,
            free_will: FreeWillConfig::default(),
            love: LoveConfig::default(),
            light: LightConfig::default(),
            coherence_peak_threshold: 0.7,
            coherence_peak_min_distance: 1.0,
            enable_entity_emergence: true,
        }
    }
}

impl UnifiedFieldConfig {
    pub fn balanced() -> Self {
        Self {
            time_step: 0.01,
            free_will: FreeWillConfig::default(),
            love: LoveConfig::default(),
            light: LightConfig::default(),
            coherence_peak_threshold: 0.7,
            coherence_peak_min_distance: 1.0,
            enable_entity_emergence: true,
        }
    }

    pub fn high_freedom() -> Self {
        Self {
            time_step: 0.01,
            free_will: FreeWillConfig::high_freedom(),
            love: LoveConfig::subtle_influence(),
            light: LightConfig::default(),
            coherence_peak_threshold: 0.6,
            coherence_peak_min_distance: 0.5,
            enable_entity_emergence: true,
        }
    }

    pub fn high_structure() -> Self {
        Self {
            time_step: 0.01,
            free_will: FreeWillConfig::low_amplitude(),
            love: LoveConfig::strong_attraction(),
            light: LightConfig::fast_propagation(),
            coherence_peak_threshold: 0.8,
            coherence_peak_min_distance: 1.5,
            enable_entity_emergence: true,
        }
    }

    pub fn deterministic(seed: u64) -> Self {
        Self {
            time_step: 0.01,
            free_will: FreeWillConfig::deterministic(seed),
            love: LoveConfig::default(),
            light: LightConfig::default(),
            coherence_peak_threshold: 0.7,
            coherence_peak_min_distance: 1.0,
            enable_entity_emergence: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DistortionStatistics {
    pub free_will_applications: usize,
    pub love_applications: usize,
    pub light_propagations: usize,
    pub average_coherence: f64,
    pub total_energy: f64,
    pub coherence_peak_count: usize,
    pub max_coherence: f64,
    pub standing_waves_detected: usize,
    pub interference_events: usize,
    pub entropy_accumulated: f64,
}

#[derive(Debug, Clone)]
pub struct CoherencePeak {
    pub position: [f64; 3],
    pub coherence: f64,
    pub energy: f64,
    pub dominant_density: usize,
    pub radius: f64,
    pub age: usize,
}

impl CoherencePeak {
    pub fn new(position: [f64; 3], coherence: f64, energy: f64, dominant_density: usize) -> Self {
        Self {
            position,
            coherence,
            energy,
            dominant_density,
            radius: 1.0,
            age: 0,
        }
    }

    pub fn age(&mut self) {
        self.age += 1;
    }

    pub fn is_valid(&self, threshold: f64) -> bool {
        self.coherence >= threshold && self.age < 100
    }
}

#[derive(Debug)]
pub struct UnifiedFieldEquation {
    config: UnifiedFieldConfig,
    free_will: FreeWillTerm,
    love: LoveTerm,
    light: LightTerm,
    time: f64,
    coherence_peaks: Vec<CoherencePeak>,
    statistics: DistortionStatistics,
    field_states: HashMap<[i64; 3], FieldState>,
}

impl UnifiedFieldEquation {
    pub fn new(config: UnifiedFieldConfig) -> Self {
        let free_will = FreeWillTerm::new(config.free_will.clone());
        let love = LoveTerm::new(config.love.clone());
        let light = LightTerm::new(config.light.clone());

        Self {
            config,
            free_will,
            love,
            light,
            time: 0.0,
            coherence_peaks: Vec::new(),
            statistics: DistortionStatistics::default(),
            field_states: HashMap::new(),
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(UnifiedFieldConfig::default())
    }

    pub fn balanced() -> Self {
        Self::new(UnifiedFieldConfig::balanced())
    }

    pub fn high_freedom() -> Self {
        Self::new(UnifiedFieldConfig::high_freedom())
    }

    pub fn high_structure() -> Self {
        Self::new(UnifiedFieldConfig::high_structure())
    }

    pub fn deterministic(seed: u64) -> Self {
        Self::new(UnifiedFieldConfig::deterministic(seed))
    }

    /// Evolve the field by one time step
    ///
    /// Applies all three primal distortions in the correct order:
    /// 1. Free Will (breaks symmetry - must be first)
    /// 2. Love (creates structure through attraction)
    /// 3. Light (propagates information)
    /// 4. Spectrum evolution (Phase 2: Space/Time ↔ Time/Space dynamics)
    ///
    /// From R&D Roadmap Phase 2:
    /// "Integrate spectrum evolution into unified field equation"
    /// "Veil crossing affects field behavior at v=1"
    pub fn evolve(&mut self, state: &mut FieldState, position: &[f64; 3]) {
        let dt = self.config.time_step;

        // 1. FREE WILL: Break symmetry first
        // From COSMOLOGICAL-ARCHITECTURE.md: "Free Will emanates from the ONE as the first distortion"
        self.free_will.apply(state, position, self.time);
        self.statistics.free_will_applications += 1;

        // 2. LOVE: Create structure through attraction
        // From COSMOLOGICAL-ARCHITECTURE.md: "Love/Logos creates structure through attractive potential"
        self.love.apply(state, position, None);
        self.statistics.love_applications += 1;

        // 3. LIGHT: Propagate information/energy
        // From COSMOLOGICAL-ARCHITECTURE.md: "Light propagates as wave dynamics"
        self.light.apply(state, position, self.time, dt);
        self.statistics.light_propagations += 1;

        // 4. PHASE 2: Spectrum Evolution
        // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 2:
        // "Entity spectrum access evolves with consciousness"
        // "Spectrum position evolves smoothly"
        state.evolve_spectrum(dt, self.calculate_spectrum_evolution_rate(state));

        // 5. PHASE 2: Veil Effects
        // Apply veil effects if at v=1
        state.apply_veil_effects();

        // Update time
        self.time += dt;

        // Update statistics
        self.update_statistics(state);

        // Cache the evolved state
        let cache_key = [
            (position[0] * 100.0) as i64,
            (position[1] * 100.0) as i64,
            (position[2] * 100.0) as i64,
        ];
        self.field_states.insert(cache_key, state.clone());
    }

    /// Calculate spectrum evolution rate based on field state
    ///
    /// From Phase 2 R&D:
    /// "Entity spectrum access evolves with consciousness"
    fn calculate_spectrum_evolution_rate(&self, state: &FieldState) -> f64 {
        // Evolution rate depends on:
        // 1. Coherence - higher coherence = faster evolution
        // 2. Density position - higher density = faster evolution
        // 3. Veil transparency - more transparent = faster evolution
        let coherence_factor = state.coherence;
        let density_factor = state.density_position.evolution_speed();
        let veil_factor = 0.5 + state.veil_transparency * 0.5;

        coherence_factor * density_factor * veil_factor
    }

    /// Evolve the field with neighboring context
    ///
    /// This enables proper Love gradient calculation and spectrum-aware evolution
    pub fn evolve_with_neighbors(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        neighbors: &[(&FieldState, [f64; 3])],
    ) {
        let dt = self.config.time_step;

        // Free Will first
        self.free_will.apply(state, position, self.time);
        self.statistics.free_will_applications += 1;

        // Love with neighbor context for gradient
        self.love.apply(state, position, Some(neighbors));
        self.statistics.love_applications += 1;

        // Light propagation
        self.light.apply(state, position, self.time, dt);
        self.statistics.light_propagations += 1;

        // Phase 2: Spectrum evolution with neighbor influence
        let neighbor_coherence: f64 = neighbors.iter().map(|(s, _)| s.coherence).sum::<f64>()
            / (neighbors.len().max(1) as f64);
        let spectrum_rate =
            self.calculate_spectrum_evolution_rate(state) * (1.0 + neighbor_coherence * 0.2);
        state.evolve_spectrum(dt, spectrum_rate);

        // Phase 2: Veil effects
        state.apply_veil_effects();

        self.time += dt;
        self.update_statistics(state);
    }

    /// Apply a specific distortion only
    pub fn apply_distortion(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        distortion: DistortionType,
    ) {
        match distortion {
            DistortionType::FreeWill => {
                self.free_will.apply(state, position, self.time);
                self.statistics.free_will_applications += 1;
            }
            DistortionType::Love => {
                self.love.apply(state, position, None);
                self.statistics.love_applications += 1;
            }
            DistortionType::Light => {
                self.light
                    .apply(state, position, self.time, self.config.time_step);
                self.statistics.light_propagations += 1;
            }
        }

        self.update_statistics(state);
    }

    /// Detect coherence peaks where entities could emerge
    ///
    /// From R&D Roadmap:
    /// "Coherence peak detection - these are locations where matter/structure can emerge"
    pub fn detect_coherence_peaks(
        &mut self,
        states: &[(FieldState, [f64; 3])],
    ) -> Vec<CoherencePeak> {
        let threshold = self.config.coherence_peak_threshold;
        let min_distance = self.config.coherence_peak_min_distance;

        // Find states above threshold
        let candidates: Vec<CoherencePeak> = states
            .iter()
            .filter(|(state, _)| state.coherence >= threshold)
            .map(|(state, pos)| {
                CoherencePeak::new(
                    *pos,
                    state.coherence,
                    state.energy,
                    state.dominant_density(),
                )
            })
            .collect();

        // Filter to distinct peaks
        let mut filtered: Vec<CoherencePeak> = Vec::new();

        for candidate in candidates {
            let mut too_close = false;

            for existing in &filtered {
                let dist = ((candidate.position[0] - existing.position[0]).powi(2)
                    + (candidate.position[1] - existing.position[1]).powi(2)
                    + (candidate.position[2] - existing.position[2]).powi(2))
                .sqrt();

                if dist < min_distance {
                    too_close = true;
                    break;
                }
            }

            if !too_close {
                filtered.push(candidate);
            }
        }

        // Age existing peaks and merge
        for peak in &mut self.coherence_peaks {
            peak.age();
        }

        // Add new peaks
        self.coherence_peaks.extend(filtered.clone());

        // Remove invalid peaks
        self.coherence_peaks.retain(|p| p.is_valid(threshold * 0.5));

        self.statistics.coherence_peak_count = self.coherence_peaks.len();

        filtered
    }

    /// Check if an entity should emerge at a position
    ///
    /// Entities emerge when coherence peaks are stable
    pub fn should_entity_emerge(&self, position: &[f64; 3], threshold: f64) -> bool {
        if !self.config.enable_entity_emergence {
            return false;
        }

        self.coherence_peaks.iter().any(|peak| {
            let dist = ((peak.position[0] - position[0]).powi(2)
                + (peak.position[1] - position[1]).powi(2)
                + (peak.position[2] - position[2]).powi(2))
            .sqrt();

            dist < peak.radius && peak.coherence >= threshold
        })
    }

    /// Get the field state at a cached position
    pub fn get_state_at(&self, position: &[f64; 3]) -> Option<&FieldState> {
        let key = [
            (position[0] * 100.0) as i64,
            (position[1] * 100.0) as i64,
            (position[2] * 100.0) as i64,
        ];
        self.field_states.get(&key)
    }

    /// Calculate the unified field equation result
    ///
    /// Returns the rate of change of the field state
    pub fn calculate_field_derivative(
        &self,
        state: &FieldState,
        position: &[f64; 3],
    ) -> FieldState {
        // This is the mathematical form of the unified equation:
        // dψ/dt = FreeWill(ψ) + Love(ψ) + Light(ψ)

        let mut derivative = FieldState::new();

        // Each distortion contributes to the derivative
        // This is a simplified analytical form for testing/analysis

        // Free Will contribution: stochastic noise centered around current state
        for i in 0..NUM_DENSITY_BANDS {
            let noise = self.free_will.archetype_22_state() - 0.5;
            derivative.density_amplitudes[i] = DensityAmplitude::new(
                noise * self.config.free_will.amplitude,
                noise * self.config.free_will.amplitude * 0.5,
            );
        }

        // Love contribution: gradient toward coherence
        let coherence_grad = self
            .love
            .calculate_coherence_gradient(state, position, None);
        for amp in &mut derivative.density_amplitudes {
            amp.re += coherence_grad.magnitude * self.config.love.strength;
        }

        // Light contribution: wave propagation tendency
        let phase = self.light.current_phase();
        for (i, amp) in derivative.density_amplitudes.iter_mut().enumerate() {
            let wave_contrib = (phase + i as f64 * 0.5).sin() * self.config.light.propagation_speed;
            amp.re += wave_contrib * 0.1;
        }

        derivative
    }

    /// Create an interference pattern at a position
    pub fn create_interference_pattern(
        &self,
        sources: &[[f64; 3]],
        time: f64,
    ) -> InterferencePattern {
        let state = FieldState::uniform(0.5);
        self.light
            .create_interference_pattern(sources, &state, time)
    }

    /// Emit a light pulse
    pub fn emit_light_pulse(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        intensity: f64,
        frequency: f64,
    ) {
        self.light.emit_pulse(state, position, intensity, frequency);
    }

    /// Create a coherence well (Love attractor)
    pub fn create_coherence_well(&mut self, center: [f64; 3], strength: f64) {
        self.love.create_coherence_well(center, strength);
    }

    /// Calculate spiritual gravity at a position
    pub fn spiritual_gravity(&self, coherence: f64, position: &[f64; 3]) -> f64 {
        self.love.spiritual_gravity(coherence, position)
    }

    fn update_statistics(&mut self, state: &FieldState) {
        self.statistics.average_coherence =
            (self.statistics.average_coherence + state.coherence) / 2.0;
        self.statistics.total_energy = state.energy;
        self.statistics.max_coherence = self.statistics.max_coherence.max(state.coherence);
        self.statistics.standing_waves_detected = self.light.standing_wave_count();
        self.statistics.interference_events = self.light.interference_events();
        self.statistics.entropy_accumulated = self.free_will.current_entropy();
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn statistics(&self) -> &DistortionStatistics {
        &self.statistics
    }

    pub fn coherence_peaks(&self) -> &[CoherencePeak] {
        &self.coherence_peaks
    }

    pub fn config(&self) -> &UnifiedFieldConfig {
        &self.config
    }

    pub fn free_will(&self) -> &FreeWillTerm {
        &self.free_will
    }

    pub fn love(&self) -> &LoveTerm {
        &self.love
    }

    pub fn light(&self) -> &LightTerm {
        &self.light
    }

    /// Reset the field equation to initial state
    pub fn reset(&mut self) {
        self.time = 0.0;
        self.coherence_peaks.clear();
        self.statistics = DistortionStatistics::default();
        self.field_states.clear();
    }
}

impl Clone for UnifiedFieldEquation {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            free_will: self.free_will.clone(),
            love: self.love.clone(),
            light: self.light.clone(),
            time: self.time,
            coherence_peaks: self.coherence_peaks.clone(),
            statistics: self.statistics.clone(),
            field_states: self.field_states.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_field_creation() {
        let field = UnifiedFieldEquation::from_defaults();
        assert!((field.time() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_evolution_sequence() {
        let mut field = UnifiedFieldEquation::balanced();
        let mut state = FieldState::uniform(0.5);
        let pos = [0.0, 0.0, 0.0];

        let _initial_coherence = state.coherence;

        field.evolve(&mut state, &pos);

        // All three distortions should have been applied
        assert!(field.statistics().free_will_applications > 0);
        assert!(field.statistics().love_applications > 0);
        assert!(field.statistics().light_propagations > 0);

        // Time should advance
        assert!(field.time() > 0.0);
    }

    #[test]
    fn test_deterministic_evolution() {
        let mut field1 = UnifiedFieldEquation::deterministic(42);
        let mut field2 = UnifiedFieldEquation::deterministic(42);

        let mut state1 = FieldState::uniform(0.5);
        let mut state2 = FieldState::uniform(0.5);

        for _ in 0..10 {
            field1.evolve(&mut state1, &[0.0, 0.0, 0.0]);
            field2.evolve(&mut state2, &[0.0, 0.0, 0.0]);
        }

        // Same seed should produce same results
        for i in 0..NUM_DENSITY_BANDS {
            assert!(
                (state1.density_amplitudes[i].re - state2.density_amplitudes[i].re).abs() < 1e-10
            );
        }
    }

    #[test]
    fn test_coherence_peak_detection() {
        let mut field = UnifiedFieldEquation::high_structure();

        // Create states with varying coherence
        let states: Vec<(FieldState, [f64; 3])> = vec![
            (
                FieldState {
                    coherence: 0.8,
                    energy: 1.0,
                    ..FieldState::uniform(0.8)
                },
                [0.0, 0.0, 0.0],
            ),
            (
                FieldState {
                    coherence: 0.9,
                    energy: 1.5,
                    ..FieldState::uniform(0.9)
                },
                [2.0, 0.0, 0.0],
            ),
            (
                FieldState {
                    coherence: 0.6,
                    energy: 0.5,
                    ..FieldState::uniform(0.6)
                },
                [4.0, 0.0, 0.0],
            ),
        ];

        let peaks = field.detect_coherence_peaks(&states);

        // Should detect peaks above threshold
        assert!(!peaks.is_empty());
        assert!(peaks[0].coherence >= 0.7);
    }

    #[test]
    fn test_entity_emergence_check() {
        let mut field = UnifiedFieldEquation::high_structure();

        // Create a coherence peak
        let states = vec![(
            FieldState {
                coherence: 0.9,
                energy: 2.0,
                ..FieldState::uniform(0.9)
            },
            [1.0, 1.0, 1.0],
        )];

        field.detect_coherence_peaks(&states);

        // Should detect entity emergence at peak position
        assert!(field.should_entity_emerge(&[1.0, 1.0, 1.0], 0.7));
    }

    #[test]
    fn test_distortion_type_application() {
        let mut field = UnifiedFieldEquation::from_defaults();
        let mut state = FieldState::uniform(0.5);

        field.apply_distortion(&mut state, &[0.0, 0.0, 0.0], DistortionType::FreeWill);
        assert_eq!(field.statistics().free_will_applications, 1);

        field.apply_distortion(&mut state, &[0.0, 0.0, 0.0], DistortionType::Love);
        assert_eq!(field.statistics().love_applications, 1);

        field.apply_distortion(&mut state, &[0.0, 0.0, 0.0], DistortionType::Light);
        assert_eq!(field.statistics().light_propagations, 1);
    }

    #[test]
    fn test_field_derivative() {
        let field = UnifiedFieldEquation::balanced();
        let state = FieldState::uniform(0.5);

        let derivative = field.calculate_field_derivative(&state, &[0.0, 0.0, 0.0]);

        // Derivative should have some non-zero components
        let total_mag: f64 = derivative
            .density_amplitudes
            .iter()
            .map(|a| a.magnitude())
            .sum();
        assert!(total_mag > 0.0);
    }

    #[test]
    fn test_light_pulse_emission() {
        let mut field = UnifiedFieldEquation::from_defaults();
        let mut state = FieldState::uniform(0.5);
        let initial_energy = state.energy;

        field.emit_light_pulse(&mut state, &[0.0, 0.0, 0.0], 0.5, 1.0);

        // Energy should increase from pulse
        assert!(state.energy > initial_energy);
    }

    #[test]
    fn test_coherence_well_creation() {
        let mut field = UnifiedFieldEquation::from_defaults();
        field.create_coherence_well([0.0, 0.0, 0.0], 0.9);

        // Well should affect gravity calculation
        let gravity = field.spiritual_gravity(0.5, &[0.0, 0.0, 0.0]);
        assert!(gravity > 0.0);
    }

    #[test]
    fn test_interference_pattern() {
        let field = UnifiedFieldEquation::from_defaults();
        let sources = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];

        let pattern = field.create_interference_pattern(&sources, 0.0);

        assert!(!pattern.positions.is_empty());
        assert!(pattern.dominant_frequency > 0.0);
    }

    #[test]
    fn test_statistics_tracking() {
        let mut field = UnifiedFieldEquation::balanced();
        let mut state = FieldState::uniform(0.5);

        for _ in 0..10 {
            field.evolve(&mut state, &[0.0, 0.0, 0.0]);
        }

        let stats = field.statistics();

        assert_eq!(stats.free_will_applications, 10);
        assert_eq!(stats.love_applications, 10);
        assert_eq!(stats.light_propagations, 10);
        assert!(stats.total_energy > 0.0);
    }

    #[test]
    fn test_reset() {
        let mut field = UnifiedFieldEquation::balanced();
        let mut state = FieldState::uniform(0.5);

        for _ in 0..10 {
            field.evolve(&mut state, &[0.0, 0.0, 0.0]);
        }

        field.reset();

        assert!((field.time() - 0.0).abs() < 1e-10);
        assert!(field.coherence_peaks().is_empty());
    }

    #[test]
    fn test_high_freedom_config() {
        let field = UnifiedFieldEquation::high_freedom();

        // High freedom should have larger amplitude
        assert!(field.config().free_will.amplitude > 0.1);
    }

    #[test]
    fn test_high_structure_config() {
        let field = UnifiedFieldEquation::high_structure();

        // High structure should have strong love attraction
        assert!(field.config().love.strength > 0.1);
    }
}
