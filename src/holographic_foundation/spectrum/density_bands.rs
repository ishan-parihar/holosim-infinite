//! Eight Coupled Density Oscillator Fields
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The density octave consists of 8 densities (1st → 8th), each representing
//!  a stage of consciousness development. Each density has sub-levels that
//!  allow for gradual progression rather than discrete jumps."
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The holographic field is stored at multiple scale levels. Each level is
//!  a compressed version of the previous (MERA-style)."
//!
//! KEY INSIGHT: Density bands are COUPLED OSCILLATORS - energy flows between them.
//! An entity at density 3.5 has amplitudes in both D3 and D4 bands.
//! The coupling allows smooth transitions rather than discrete jumps.

use super::{DensityPosition, NUM_DENSITY_BANDS};
use crate::types::Float;

pub type ComplexAmplitude = num_complex::Complex<Float>;

#[derive(Debug, Clone)]
pub struct DensityBandConfig {
    pub coupling_strength: Float,
    pub oscillation_frequency: Float,
    pub damping: Float,
    pub resonance_width: Float,
}

impl Default for DensityBandConfig {
    fn default() -> Self {
        Self {
            coupling_strength: 0.1,
            oscillation_frequency: 1.0,
            damping: 0.01,
            resonance_width: 0.5,
        }
    }
}

impl DensityBandConfig {
    pub fn strong_coupling() -> Self {
        Self {
            coupling_strength: 0.3,
            oscillation_frequency: 1.0,
            damping: 0.005,
            resonance_width: 0.8,
        }
    }

    pub fn weak_coupling() -> Self {
        Self {
            coupling_strength: 0.05,
            oscillation_frequency: 1.0,
            damping: 0.02,
            resonance_width: 0.3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DensityBandOscillator {
    pub density_index: usize,
    pub amplitude: ComplexAmplitude,
    pub velocity: ComplexAmplitude,
    pub phase: Float,
    pub energy: Float,
}

impl DensityBandOscillator {
    pub fn new(density_index: usize) -> Self {
        Self {
            density_index,
            amplitude: ComplexAmplitude::new(1.0, 0.0),
            velocity: ComplexAmplitude::new(0.0, 0.0),
            phase: 0.0,
            energy: 1.0,
        }
    }

    pub fn with_amplitude(density_index: usize, amplitude: ComplexAmplitude) -> Self {
        Self {
            density_index,
            amplitude,
            velocity: ComplexAmplitude::new(0.0, 0.0),
            phase: 0.0,
            energy: amplitude.norm_sqr(),
        }
    }

    pub fn magnitude(&self) -> Float {
        self.amplitude.norm()
    }

    pub fn phase_angle(&self) -> Float {
        self.amplitude.arg()
    }

    pub fn set_magnitude(&mut self, mag: Float) {
        let current_phase = self.phase_angle();
        self.amplitude = ComplexAmplitude::from_polar(mag, current_phase);
        self.energy = mag * mag;
    }

    pub fn set_phase(&mut self, phase: Float) {
        let current_mag = self.magnitude();
        self.amplitude = ComplexAmplitude::from_polar(current_mag, phase);
        self.phase = phase;
    }

    pub fn oscillate(&mut self, dt: Float, frequency: Float) {
        self.phase += frequency * dt * (self.density_index + 1) as Float;
        let mag = self.magnitude();
        self.amplitude = ComplexAmplitude::from_polar(mag, self.phase);
    }

    pub fn apply_damping(&mut self, damping: Float) {
        self.amplitude *= 1.0 - damping;
        self.energy = self.amplitude.norm_sqr();
    }

    pub fn add_energy(&mut self, energy: Float) {
        let current_mag = self.magnitude();
        let new_mag = (current_mag * current_mag + energy).sqrt();
        self.set_magnitude(new_mag);
    }

    pub fn remove_energy(&mut self, energy: Float) -> Float {
        let current_mag = self.magnitude();
        let current_energy = current_mag * current_mag;
        let removed = energy.min(current_energy);
        let new_mag = (current_energy - removed).sqrt();
        self.set_magnitude(new_mag);
        removed
    }
}

#[derive(Debug, Clone)]
pub struct DensityBands {
    oscillators: [DensityBandOscillator; NUM_DENSITY_BANDS],
    config: DensityBandConfig,
    coupling_matrix: [[Float; NUM_DENSITY_BANDS]; NUM_DENSITY_BANDS],
    total_energy: Float,
    dominant_band: usize,
}

impl DensityBands {
    pub fn new(config: DensityBandConfig) -> Self {
        let oscillators: [DensityBandOscillator; NUM_DENSITY_BANDS] =
            std::array::from_fn(DensityBandOscillator::new);

        let coupling_matrix = Self::build_coupling_matrix(config.coupling_strength);

        let mut bands = Self {
            oscillators,
            config,
            coupling_matrix,
            total_energy: NUM_DENSITY_BANDS as Float,
            dominant_band: 0,
        };
        bands.update_total_energy();
        bands
    }

    pub fn from_defaults() -> Self {
        Self::new(DensityBandConfig::default())
    }

    fn build_coupling_matrix(strength: Float) -> [[Float; NUM_DENSITY_BANDS]; NUM_DENSITY_BANDS] {
        let mut matrix = [[0.0; NUM_DENSITY_BANDS]; NUM_DENSITY_BANDS];

        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                if i != j {
                    let distance = (i as Float - j as Float).abs();
                    *cell = strength / (1.0 + distance);
                }
            }
        }

        matrix
    }

    pub fn evolve(&mut self, dt: Float) {
        // Phase 1: Individual oscillation
        for osc in &mut self.oscillators {
            osc.oscillate(dt, self.config.oscillation_frequency);
        }

        // Phase 2: Coupling between bands
        self.apply_coupling(dt);

        // Phase 3: Damping
        for osc in &mut self.oscillators {
            osc.apply_damping(self.config.damping * dt);
        }

        // Update derived quantities
        self.update_total_energy();
        self.update_dominant_band();
    }

    fn apply_coupling(&mut self, dt: Float) {
        let mut energy_transfers: [Float; NUM_DENSITY_BANDS] = [0.0; NUM_DENSITY_BANDS];

        for i in 0..NUM_DENSITY_BANDS {
            for j in (i + 1)..NUM_DENSITY_BANDS {
                let coupling = self.coupling_matrix[i][j];
                let phase_diff =
                    self.oscillators[i].phase_angle() - self.oscillators[j].phase_angle();
                let mag_i = self.oscillators[i].magnitude();
                let mag_j = self.oscillators[j].magnitude();

                // Energy flows from higher to lower magnitude (toward equilibrium)
                // But modulated by phase alignment
                let phase_factor = phase_diff.cos();
                let energy_flow = coupling * dt * (mag_i - mag_j) * phase_factor;

                energy_transfers[i] -= energy_flow;
                energy_transfers[j] += energy_flow;
            }
        }

        // Apply energy transfers
        for (i, osc) in self.oscillators.iter_mut().enumerate() {
            if energy_transfers[i] > 0.0 {
                osc.add_energy(energy_transfers[i]);
            } else {
                osc.remove_energy(-energy_transfers[i]);
            }
        }
    }

    fn update_total_energy(&mut self) {
        self.total_energy = self.oscillators.iter().map(|o| o.energy).sum();
    }

    fn update_dominant_band(&mut self) {
        let mut max_energy = 0.0;
        for (i, osc) in self.oscillators.iter().enumerate() {
            if osc.energy > max_energy {
                max_energy = osc.energy;
                self.dominant_band = i;
            }
        }
    }

    pub fn set_position(&mut self, position: &DensityPosition) {
        let _primary = position.primary_density();
        let phase = position.sub_density_phase();

        // Set amplitudes based on position using Gaussian-like distribution
        for (i, osc) in self.oscillators.iter_mut().enumerate() {
            let band_center = (i + 1) as Float;
            let distance = (band_center - position.value).abs();

            // Gaussian envelope centered at position
            let magnitude = (-distance.powi(2) / (2.0 * self.config.resonance_width.powi(2))).exp();

            // Phase depends on sub-density phase
            let angle = phase * std::f64::consts::TAU * (i as Float / NUM_DENSITY_BANDS as Float);

            osc.amplitude = ComplexAmplitude::from_polar(magnitude, angle);
            osc.phase = angle;
            osc.energy = magnitude * magnitude;
        }

        self.update_total_energy();
        self.update_dominant_band();
    }

    pub fn get_density_position(&self) -> DensityPosition {
        // Weighted average of band positions
        let mut weighted_sum = 0.0;
        let mut weight_sum = 0.0;

        for (i, osc) in self.oscillators.iter().enumerate() {
            let band_position = (i + 1) as Float;
            let weight = osc.energy;
            weighted_sum += band_position * weight;
            weight_sum += weight;
        }

        let position = if weight_sum > 0.0 {
            weighted_sum / weight_sum
        } else {
            1.0
        };

        DensityPosition::new(position)
    }

    pub fn add_energy_at(&mut self, band: usize, energy: Float) {
        if band < NUM_DENSITY_BANDS {
            self.oscillators[band].add_energy(energy);
            self.update_total_energy();
            self.update_dominant_band();
        }
    }

    pub fn remove_energy_at(&mut self, band: usize, energy: Float) -> Float {
        if band < NUM_DENSITY_BANDS {
            let removed = self.oscillators[band].remove_energy(energy);
            self.update_total_energy();
            self.update_dominant_band();
            removed
        } else {
            0.0
        }
    }

    pub fn get_amplitude(&self, band: usize) -> Option<ComplexAmplitude> {
        if band < NUM_DENSITY_BANDS {
            Some(self.oscillators[band].amplitude)
        } else {
            None
        }
    }

    pub fn get_magnitude(&self, band: usize) -> Float {
        if band < NUM_DENSITY_BANDS {
            self.oscillators[band].magnitude()
        } else {
            0.0
        }
    }

    pub fn get_phase(&self, band: usize) -> Float {
        if band < NUM_DENSITY_BANDS {
            self.oscillators[band].phase_angle()
        } else {
            0.0
        }
    }

    pub fn amplitudes(&self) -> [ComplexAmplitude; NUM_DENSITY_BANDS] {
        std::array::from_fn(|i| self.oscillators[i].amplitude)
    }

    pub fn magnitudes(&self) -> [Float; NUM_DENSITY_BANDS] {
        std::array::from_fn(|i| self.oscillators[i].magnitude())
    }

    pub fn phases(&self) -> [Float; NUM_DENSITY_BANDS] {
        std::array::from_fn(|i| self.oscillators[i].phase_angle())
    }

    pub fn energies(&self) -> [Float; NUM_DENSITY_BANDS] {
        std::array::from_fn(|i| self.oscillators[i].energy)
    }

    pub fn total_energy(&self) -> Float {
        self.total_energy
    }

    pub fn dominant_band(&self) -> usize {
        self.dominant_band
    }

    pub fn coherence(&self) -> Float {
        let mags = self.magnitudes();
        let mean: Float = mags.iter().sum::<Float>() / mags.len() as Float;
        let variance: Float =
            mags.iter().map(|m| (m - mean).powi(2)).sum::<Float>() / mags.len() as Float;
        1.0 - variance.sqrt().min(1.0)
    }

    pub fn band_names() -> [&'static str; NUM_DENSITY_BANDS] {
        [
            "1st Density (Awareness)",
            "2nd Density (Growth)",
            "3rd Density (Self-Awareness)",
            "4th Density (Love/Light)",
            "5th Density (Light/Light)",
            "6th Density (Unity)",
            "7th Density (Gateway)",
            "8th Density (Octave)",
        ]
    }

    pub fn config(&self) -> &DensityBandConfig {
        &self.config
    }

    pub fn oscillator(&self, band: usize) -> Option<&DensityBandOscillator> {
        self.oscillators.get(band)
    }

    pub fn oscillator_mut(&mut self, band: usize) -> Option<&mut DensityBandOscillator> {
        self.oscillators.get_mut(band)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator_creation() {
        let osc = DensityBandOscillator::new(3);
        assert_eq!(osc.density_index, 3);
        assert!((osc.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_oscillator_magnitude() {
        let mut osc = DensityBandOscillator::new(0);
        osc.set_magnitude(2.0);
        assert!((osc.magnitude() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_oscillator_phase() {
        let mut osc = DensityBandOscillator::new(0);
        osc.set_phase(std::f64::consts::PI);
        assert!((osc.phase_angle() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_oscillator_energy() {
        let mut osc = DensityBandOscillator::new(0);
        osc.add_energy(3.0);
        assert!(osc.energy > 1.0);

        let removed = osc.remove_energy(10.0);
        assert!(removed < 10.0);
        assert!(osc.energy < 1.0);
    }

    #[test]
    fn test_bands_creation() {
        let bands = DensityBands::from_defaults();
        assert!(bands.total_energy() > 0.0);
    }

    #[test]
    fn test_bands_evolution() {
        let mut bands = DensityBands::from_defaults();
        let initial_energy = bands.total_energy();

        for _ in 0..10 {
            bands.evolve(0.01);
        }

        // Energy should decrease due to damping
        assert!(bands.total_energy() < initial_energy);
    }

    #[test]
    fn test_bands_set_position() {
        let mut bands = DensityBands::from_defaults();
        bands.set_position(&DensityPosition::fourth_density());

        // D4 should be dominant
        assert_eq!(bands.dominant_band(), 3);
    }

    #[test]
    fn test_bands_get_position() {
        let mut bands = DensityBands::from_defaults();
        bands.set_position(&DensityPosition::new(3.5));

        let pos = bands.get_density_position();
        // Should be close to 3.5
        assert!((pos.value - 3.5).abs() < 0.5);
    }

    #[test]
    fn test_coupling_matrix() {
        let config = DensityBandConfig::strong_coupling();
        let bands = DensityBands::new(config);

        // Adjacent bands should have higher coupling
        assert!(bands.coupling_matrix[0][1] > bands.coupling_matrix[0][7]);
    }

    #[test]
    fn test_coherence() {
        let mut bands = DensityBands::from_defaults();

        // Set all bands to same magnitude = high coherence
        for osc in &mut bands.oscillators {
            osc.set_magnitude(1.0);
        }
        let high_coherence = bands.coherence();

        // Set one band dominant = lower coherence
        bands.oscillators[0].set_magnitude(5.0);
        let low_coherence = bands.coherence();

        assert!(high_coherence > low_coherence);
    }

    #[test]
    fn test_energy_transfer() {
        let mut bands = DensityBands::from_defaults();

        bands.add_energy_at(3, 5.0);
        let mag_after_add = bands.get_magnitude(3);

        bands.remove_energy_at(3, 2.0);
        let mag_after_remove = bands.get_magnitude(3);

        assert!(mag_after_add > mag_after_remove);
    }

    #[test]
    fn test_band_names() {
        let names = DensityBands::band_names();
        assert_eq!(names[0], "1st Density (Awareness)");
        assert_eq!(names[3], "4th Density (Love/Light)");
        assert_eq!(names[7], "8th Density (Octave)");
    }
}
