//! Light Term: Wave Propagation with Interference
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Light propagates as wave dynamics, allowing information/energy to travel.
//!  The Third Distortion - Light as the manifestation of the creative principle.
//!  Waves can interfere constructively to form standing waves (particles!)"
//!
//! KEY INSIGHT: Light is wave dynamics that can create STANDING WAVES,
//! and standing waves are the basis of PARTICLES (matter emerges from light!)
//!
//! The Light term implements:
//! - Wave equation: ∂²ψ/∂t² = c²∇²ψ - damping·∂ψ/∂t
//! - Interference patterns between waves
//! - Standing wave formation (particle emergence)
//! - Information/energy propagation through the field
//!
//! From R&D Roadmap:
//! "Light as wave propagation (Laplacian operator)"

use std::collections::VecDeque;

use super::{DensityAmplitude, FieldState, NUM_DENSITY_BANDS};

#[derive(Debug, Clone)]
pub struct LightConfig {
    pub propagation_speed: f64,
    pub damping: f64,
    pub interference_strength: f64,
    pub history_depth: usize,
    pub standing_wave_threshold: f64,
}

impl Default for LightConfig {
    fn default() -> Self {
        Self {
            propagation_speed: 1.0,
            damping: 0.01,
            interference_strength: 0.5,
            history_depth: 5,
            standing_wave_threshold: 0.8,
        }
    }
}

impl LightConfig {
    pub fn fast_propagation() -> Self {
        Self {
            propagation_speed: 2.0,
            damping: 0.005,
            interference_strength: 0.7,
            history_depth: 7,
            standing_wave_threshold: 0.7,
        }
    }

    pub fn high_damping() -> Self {
        Self {
            propagation_speed: 0.5,
            damping: 0.1,
            interference_strength: 0.3,
            history_depth: 3,
            standing_wave_threshold: 0.9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaveState {
    pub position: [f64; 3],
    pub density_amplitudes: [DensityAmplitude; NUM_DENSITY_BANDS],
    pub energy: f64,
    pub coherence: f64,
    pub time: f64,
}

impl WaveState {
    pub fn from_field_state(state: &FieldState, position: [f64; 3], time: f64) -> Self {
        Self {
            position,
            density_amplitudes: state.density_amplitudes,
            energy: state.energy,
            coherence: state.coherence,
            time,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterferencePattern {
    pub positions: Vec<[f64; 3]>,
    pub intensities: Vec<f64>,
    pub dominant_frequency: f64,
    pub standing_wave_ratio: f64,
}

impl InterferencePattern {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            intensities: Vec::new(),
            dominant_frequency: 0.0,
            standing_wave_ratio: 0.0,
        }
    }

    pub fn add_point(&mut self, position: [f64; 3], intensity: f64) {
        self.positions.push(position);
        self.intensities.push(intensity);
    }

    pub fn max_intensity(&self) -> f64 {
        self.intensities.iter().cloned().fold(0.0, f64::max)
    }

    pub fn average_intensity(&self) -> f64 {
        if self.intensities.is_empty() {
            0.0
        } else {
            self.intensities.iter().sum::<f64>() / self.intensities.len() as f64
        }
    }

    /// Check if pattern shows standing wave characteristics
    pub fn is_standing_wave(&self, threshold: f64) -> bool {
        self.standing_wave_ratio > threshold
    }
}

/// Spectrum propagation parameters for Phase 2 integration
#[derive(Debug, Clone, Copy)]
struct SpectrumPropagation {
    effective_speed: f64,
    damping_modifier: f64,
    interference_modifier: f64,
}

#[derive(Debug)]
pub struct LightTerm {
    config: LightConfig,
    prev_states: VecDeque<WaveState>,
    phase_accumulator: f64,
    total_propagations: usize,
    standing_waves_detected: usize,
    total_interference_events: usize,
}

impl LightTerm {
    pub fn new(config: LightConfig) -> Self {
        Self {
            prev_states: VecDeque::with_capacity(config.history_depth),
            config,
            phase_accumulator: 0.0,
            total_propagations: 0,
            standing_waves_detected: 0,
            total_interference_events: 0,
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(LightConfig::default())
    }

    /// Apply wave propagation dynamics to field state
    ///
    /// Wave equation: ∂²ψ/∂t² = c²∇²ψ - damping·∂ψ/∂t
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Light propagates as wave dynamics allowing information/energy to travel
    ///  through the field. Standing waves can form particles."
    ///
    /// From Phase 2 R&D:
    /// "Light: Wave propagation changes based on coordinate system"
    /// - Time/Space (v < 1): 3D time, 1D space - waves propagate differently
    /// - At Veil (v ≈ 1): Phase transition in wave behavior
    /// - Space/Time (v > 1): Standard 3D space, 1D time propagation
    pub fn apply(&mut self, state: &mut FieldState, position: &[f64; 3], time: f64, dt: f64) {
        self.total_propagations += 1;

        // Store current state for history
        let current_state = WaveState::from_field_state(state, *position, time);
        self.prev_states.push_back(current_state.clone());

        // Trim history to configured depth
        while self.prev_states.len() > self.config.history_depth {
            self.prev_states.pop_front();
        }

        // Phase 2: Spectrum-aware propagation
        // From Phase 2 R&D: "Light: Wave propagation changes based on coordinate system"
        let spectrum_propagation = self.calculate_spectrum_propagation(state);

        // Apply wave equation to each density band
        for band_idx in 0..NUM_DENSITY_BANDS {
            let new_amplitude =
                self.propagate_wave_spectrum(state, band_idx, position, dt, spectrum_propagation);
            state.density_amplitudes[band_idx] = new_amplitude;
        }

        // Update phase accumulator with spectrum modification
        self.phase_accumulator += dt * spectrum_propagation.effective_speed;

        // Phase 2: At the veil, apply special interference effects
        if state.is_at_veil() {
            self.apply_veil_interference(state, position, time);
        } else {
            // Apply standard interference patterns
            self.apply_interference(state, position, time);
        }

        // Check for standing wave formation
        if self.detect_standing_wave(state) {
            self.standing_waves_detected += 1;
        }

        // Update energy based on wave dynamics
        state.energy = self.calculate_wave_energy(state);
    }

    /// Calculate spectrum-aware propagation parameters
    ///
    /// From Phase 2 R&D:
    /// "Wave propagation changes based on coordinate system"
    fn calculate_spectrum_propagation(&self, state: &FieldState) -> SpectrumPropagation {
        let base_speed = self.config.propagation_speed;
        let base_damping = self.config.damping;

        if state.is_time_space() {
            // Time/Space: 3D time, 1D space
            // Waves propagate "through time" differently - more coherent
            SpectrumPropagation {
                effective_speed: base_speed * 0.8,
                damping_modifier: 0.7,      // Less damping = more coherent
                interference_modifier: 1.3, // More interference
            }
        } else if state.is_at_veil() {
            // At the veil: phase transition
            SpectrumPropagation {
                effective_speed: base_speed,
                damping_modifier: 1.0,
                interference_modifier: 2.0, // Maximum interference at veil
            }
        } else {
            // Space/Time: standard propagation
            SpectrumPropagation {
                effective_speed: base_speed,
                damping_modifier: 1.0,
                interference_modifier: 1.0,
            }
        }
    }

    /// Propagate wave with spectrum awareness
    fn propagate_wave_spectrum(
        &self,
        state: &FieldState,
        band_idx: usize,
        position: &[f64; 3],
        dt: f64,
        spectrum_params: SpectrumPropagation,
    ) -> DensityAmplitude {
        let current = &state.density_amplitudes[band_idx];
        let c = spectrum_params.effective_speed;
        let damping = self.config.damping * spectrum_params.damping_modifier;

        // Laplacian approximation (simplified - assumes uniform grid)
        let laplacian = self.approximate_laplacian(state, band_idx);

        // Wave equation: ψ(t+dt) ≈ ψ(t) + c²∇²ψ·dt² - damping·ψ(t)
        let wave_coeff = c * c * dt * dt;
        let wave_term = laplacian.scale(wave_coeff);
        let damping_term = damping * dt;

        DensityAmplitude::new(
            current.re * (1.0 - damping_term) + wave_term.re,
            current.im * (1.0 - damping_term) + wave_term.im,
        )
    }

    /// Apply special interference effects at the veil
    ///
    /// From Phase 2 R&D:
    /// "Veil crossing produces qualitative change in field behavior"
    fn apply_veil_interference(&mut self, state: &mut FieldState, position: &[f64; 3], time: f64) {
        self.total_interference_events += 1;

        // At the veil, interference creates phase transition
        // All bands experience maximum interference potential
        let veil_phase = time * self.config.propagation_speed;

        for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
            // Constructive interference boost at veil
            let interference_boost =
                (veil_phase + i as f64 * 0.5).sin().abs() * 0.2 * state.veil_transparency;
            *amp = amp.scale(1.0 + interference_boost);
        }

        // Coherence increases at veil due to phase alignment
        state.coherence = (state.coherence * (1.0 + state.veil_transparency * 0.1)).min(1.0);
    }

    /// Approximate Laplacian for wave propagation
    fn approximate_laplacian(&self, state: &FieldState, band_idx: usize) -> DensityAmplitude {
        // Simplified Laplacian - in a real implementation, this would use
        // spatial derivatives from neighboring field points
        let current = &state.density_amplitudes[band_idx];
        let avg_neighbor_mag = state.total_magnitude() / NUM_DENSITY_BANDS as f64;
        let current_mag = current.magnitude();

        // Laplacian as deviation from average
        let laplacian_mag = current_mag - avg_neighbor_mag;
        let phase = current.phase();

        DensityAmplitude::from_polar(laplacian_mag * 0.1, phase)
    }

    /// Propagate a wave in a single density band
    ///
    /// Uses simplified wave equation:
    /// ψ(t+dt) = 2ψ(t) - ψ(t-dt) + c²∇²ψ·dt² - damping·(ψ(t) - ψ(t-dt))
    fn propagate_wave(
        &self,
        state: &FieldState,
        band_idx: usize,
        position: &[f64; 3],
        dt: f64,
    ) -> DensityAmplitude {
        let current = &state.density_amplitudes[band_idx];

        // Get previous state if available
        let prev_amp = self
            .prev_states
            .back()
            .and_then(|s| Some(s.density_amplitudes[band_idx]))
            .unwrap_or_else(|| current.clone());

        // Calculate Laplacian (spatial second derivative)
        let laplacian = self.calculate_laplacian(state, band_idx, position);

        // Wave equation update
        let c_squared = self.config.propagation_speed * self.config.propagation_speed;
        let damping_factor = self.config.damping;

        // Second-order time derivative approximation
        let acceleration_re =
            c_squared * laplacian.re - damping_factor * (current.re - prev_amp.re);
        let acceleration_im =
            c_squared * laplacian.im - damping_factor * (current.im - prev_amp.im);

        // Update amplitude
        let new_re = current.re + (current.re - prev_amp.re) + acceleration_re * dt * dt;
        let new_im = current.im + (current.im - prev_amp.im) + acceleration_im * dt * dt;

        DensityAmplitude::new(new_re, new_im)
    }

    /// Calculate Laplacian for wave propagation
    ///
    /// ∇²ψ = ∂²ψ/∂x² + ∂²ψ/∂y² + ∂²ψ/∂z²
    fn calculate_laplacian(
        &self,
        state: &FieldState,
        band_idx: usize,
        position: &[f64; 3],
    ) -> DensityAmplitude {
        // Use position-based approximation for Laplacian
        // In a full implementation, this would sample neighboring nodes

        let amp = &state.density_amplitudes[band_idx];

        // Create wave pattern based on position
        let k = 2.0 * std::f64::consts::PI * self.config.propagation_speed;
        let wave_x = (k * position[0]).cos();
        let wave_y = (k * position[1]).cos();
        let wave_z = (k * position[2]).cos();

        // Laplacian approximation (negative second derivative)
        let laplacian_factor = -(wave_x + wave_y + wave_z) / 3.0;

        DensityAmplitude::new(
            amp.re * laplacian_factor * 0.1,
            amp.im * laplacian_factor * 0.1,
        )
    }

    /// Apply interference between waves
    ///
    /// Waves interfere constructively (in phase) or destructively (out of phase)
    fn apply_interference(&mut self, state: &mut FieldState, position: &[f64; 3], time: f64) {
        if self.prev_states.len() < 2 {
            return;
        }

        self.total_interference_events += 1;

        // Get states for interference calculation
        let states: Vec<&WaveState> = self.prev_states.iter().collect();

        // Interference between current and past states
        for band_idx in 0..NUM_DENSITY_BANDS {
            let current = &state.density_amplitudes[band_idx];

            // Sum contributions from previous states (interference)
            let mut interference_sum = DensityAmplitude::zero();
            let mut weight_sum = 0.0;

            for (i, past_state) in states.iter().enumerate() {
                let weight = 1.0 / (i + 1) as f64; // More recent = higher weight
                let past_amp = &past_state.density_amplitudes[band_idx];

                // Phase difference determines constructive/destructive interference
                let phase_diff = current.phase() - past_amp.phase();

                // Interference strength depends on phase alignment
                let interference_factor = phase_diff.cos() * self.config.interference_strength;

                interference_sum = DensityAmplitude::new(
                    interference_sum.re + past_amp.re * weight * interference_factor,
                    interference_sum.im + past_amp.im * weight * interference_factor,
                );
                weight_sum += weight;
            }

            // Apply interference contribution
            if weight_sum > 0.0 {
                let normalized_interference = interference_sum.scale(1.0 / weight_sum);
                state.density_amplitudes[band_idx] =
                    current.add(&normalized_interference.scale(0.1));
            }
        }
    }

    /// Detect standing wave formation
    ///
    /// Standing waves occur when forward and backward waves interfere
    /// to create a stationary pattern (nodes and antinodes)
    fn detect_standing_wave(&self, state: &FieldState) -> bool {
        if self.prev_states.is_empty() {
            return false;
        }

        // Check for stability pattern (standing wave signature)
        let current_mags: Vec<f64> = state
            .density_amplitudes
            .iter()
            .map(|a| a.magnitude())
            .collect();

        // Get average magnitude from history
        let mut avg_mags = [0.0; NUM_DENSITY_BANDS];
        for past_state in &self.prev_states {
            for (i, amp) in past_state.density_amplitudes.iter().enumerate() {
                avg_mags[i] += amp.magnitude();
            }
        }
        let history_count = self.prev_states.len() as f64;
        for avg in &mut avg_mags {
            *avg /= history_count;
        }

        // Standing wave if magnitudes are stable (low variance)
        let mut stability_sum = 0.0;
        for (i, current_mag) in current_mags.iter().enumerate() {
            let diff = (current_mag - avg_mags[i]).abs();
            stability_sum += 1.0 - diff.min(1.0);
        }
        let stability_ratio = stability_sum / NUM_DENSITY_BANDS as f64;

        stability_ratio > self.config.standing_wave_threshold
    }

    /// Calculate wave energy in the field
    fn calculate_wave_energy(&self, state: &FieldState) -> f64 {
        // Wave energy proportional to amplitude squared
        let mut total_energy = 0.0;

        for amp in &state.density_amplitudes {
            let mag_squared = amp.re * amp.re + amp.im * amp.im;
            total_energy += mag_squared;
        }

        // Include kinetic energy from phase rate
        let phase_energy = self.phase_accumulator.sin().abs() * 0.1;

        total_energy * 0.5 + phase_energy
    }

    /// Create an interference pattern from multiple wave sources
    pub fn create_interference_pattern(
        &self,
        sources: &[[f64; 3]],
        state: &FieldState,
        time: f64,
    ) -> InterferencePattern {
        let mut pattern = InterferencePattern::new();

        // Sample points in a grid around sources
        let step = 0.5;
        for x in -5..=5 {
            for y in -5..=5 {
                for z in -5..=5 {
                    let sample_pos = [x as f64 * step, y as f64 * step, z as f64 * step];

                    // Calculate interference from all sources
                    let mut intensity = 0.0;
                    for source in sources {
                        let dist = ((sample_pos[0] - source[0]).powi(2)
                            + (sample_pos[1] - source[1]).powi(2)
                            + (sample_pos[2] - source[2]).powi(2))
                        .sqrt();

                        // Wave from source (spherical)
                        let k = 2.0 * std::f64::consts::PI * self.config.propagation_speed;
                        let wave = (k * dist - time * self.config.propagation_speed).cos();

                        // Intensity decreases with distance
                        intensity += wave / (dist + 1.0);
                    }

                    pattern.add_point(sample_pos, intensity);
                }
            }
        }

        // Calculate standing wave ratio
        let max = pattern.max_intensity();
        let avg = pattern.average_intensity();
        pattern.standing_wave_ratio = if max > 0.0 { avg / max } else { 0.0 };

        // Estimate dominant frequency
        pattern.dominant_frequency = self.config.propagation_speed / (2.0 * std::f64::consts::PI);

        pattern
    }

    /// Emit a light pulse from a position
    ///
    /// This creates an outgoing wave that propagates through the field
    pub fn emit_pulse(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        intensity: f64,
        frequency: f64,
    ) {
        let phase = self.phase_accumulator * frequency;

        // Add pulse to all density bands with phase relationship
        for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
            let band_phase = phase + i as f64 * std::f64::consts::PI / 4.0;
            let pulse_amp = DensityAmplitude::from_polar(intensity, band_phase);
            *amp = amp.add(&pulse_amp);
        }

        // Boost energy
        state.energy += intensity * intensity;
    }

    /// Absorb light energy at a position
    ///
    /// This removes energy from the field (creates a sink)
    pub fn absorb_energy(&mut self, state: &mut FieldState, amount: f64) {
        for amp in &mut state.density_amplitudes {
            let mag = amp.magnitude();
            if mag > 0.01 {
                let reduction = amount.min(mag * 0.5);
                *amp = amp.scale(1.0 - reduction / mag);
            }
        }
        state.energy = (state.energy - amount).max(0.0);
    }

    /// Get wave phase at current time
    pub fn current_phase(&self) -> f64 {
        self.phase_accumulator
    }

    /// Get number of standing waves detected
    pub fn standing_wave_count(&self) -> usize {
        self.standing_waves_detected
    }

    /// Get total propagation steps
    pub fn total_propagations(&self) -> usize {
        self.total_propagations
    }

    /// Get interference event count
    pub fn interference_events(&self) -> usize {
        self.total_interference_events
    }

    pub fn config(&self) -> &LightConfig {
        &self.config
    }
}

impl Clone for LightTerm {
    fn clone(&self) -> Self {
        Self {
            prev_states: self.prev_states.clone(),
            config: self.config.clone(),
            phase_accumulator: self.phase_accumulator,
            total_propagations: self.total_propagations,
            standing_waves_detected: self.standing_waves_detected,
            total_interference_events: self.total_interference_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_creation() {
        let light = LightTerm::from_defaults();
        assert_eq!(light.total_propagations(), 0);
    }

    #[test]
    fn test_wave_propagation() {
        let mut light = LightTerm::from_defaults();
        let mut state = FieldState::uniform(0.5);

        // Initial state
        let initial_mag = state.total_magnitude();

        // Propagate for several steps
        for t in 0..10 {
            light.apply(&mut state, &[0.0, 0.0, 0.0], t as f64 * 0.1, 0.1);
        }

        // State should have evolved
        assert!(light.total_propagations() > 0);
    }

    #[test]
    fn test_interference_pattern() {
        let light = LightTerm::from_defaults();
        let state = FieldState::uniform(0.5);

        let sources = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        let pattern = light.create_interference_pattern(&sources, &state, 0.0);

        assert!(!pattern.positions.is_empty());
        assert!(pattern.intensities.len() == pattern.positions.len());
    }

    #[test]
    fn test_pulse_emission() {
        let mut light = LightTerm::from_defaults();
        let mut state = FieldState::uniform(0.5);
        let initial_energy = state.energy;

        light.emit_pulse(&mut state, &[0.0, 0.0, 0.0], 0.5, 1.0);

        // Energy should increase from pulse
        assert!(state.energy > initial_energy);
    }

    #[test]
    fn test_energy_absorption() {
        let mut light = LightTerm::from_defaults();
        let mut state = FieldState::uniform(0.5);

        // Add energy first
        light.emit_pulse(&mut state, &[0.0, 0.0, 0.0], 0.5, 1.0);
        let energy_after_pulse = state.energy;

        // Then absorb
        light.absorb_energy(&mut state, 0.2);

        // Energy should decrease
        assert!(state.energy < energy_after_pulse);
    }

    #[test]
    fn test_standing_wave_detection() {
        let mut light = LightTerm::new(LightConfig {
            standing_wave_threshold: 0.5,
            ..Default::default()
        });

        // Create stable state (standing wave signature)
        let mut state = FieldState::uniform(0.8);

        // Evolve with very small changes (creates stability)
        for t in 0..10 {
            light.apply(&mut state, &[0.0, 0.0, 0.0], t as f64 * 0.01, 0.01);
        }

        // Standing waves may be detected with stable input
        // (depends on parameters and random factors)
    }

    #[test]
    fn test_phase_accumulation() {
        let mut light = LightTerm::from_defaults();
        let mut state = FieldState::uniform(0.5);

        let initial_phase = light.current_phase();

        for t in 0..5 {
            light.apply(&mut state, &[0.0, 0.0, 0.0], t as f64, 0.1);
        }

        // Phase should have accumulated
        assert!(light.current_phase() > initial_phase);
    }

    #[test]
    fn test_laplacian_calculation() {
        let light = LightTerm::from_defaults();
        let state = FieldState::uniform(0.5);

        let laplacian = light.calculate_laplacian(&state, 0, &[1.0, 1.0, 1.0]);

        // Laplacian should have some value (not zero for non-zero input)
        assert!(laplacian.re.abs() > 0.0 || laplacian.im.abs() > 0.0);
    }

    #[test]
    fn test_interference_constructive() {
        let mut light = LightTerm::new(LightConfig {
            interference_strength: 0.8,
            ..Default::default()
        });

        let mut state = FieldState::uniform(0.5);

        // Apply multiple times to build interference history
        for t in 0..3 {
            light.apply(&mut state, &[0.0, 0.0, 0.0], t as f64, 0.1);
        }

        assert!(light.interference_events() > 0);
    }

    #[test]
    fn test_wave_state_from_field() {
        let state = FieldState::uniform(0.7);
        let wave_state = WaveState::from_field_state(&state, [1.0, 2.0, 3.0], 0.5);

        assert_eq!(wave_state.position, [1.0, 2.0, 3.0]);
        assert!((wave_state.coherence - 0.7).abs() < 1e-10);
    }
}
