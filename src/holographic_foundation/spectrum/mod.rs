//! Spectrum Dynamics + Veil Crossing
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The spectrum is ONE unified reality with a QUALITATIVE BREAK at v=1.
//!  At v=1, the Veil: Space/Time (3D space, 1D time) ↔ Time/Space (1D space, 3D time)
//!  The spectrum is configured at galactic and solar scales before physical matter exists."
//!
//! KEY INSIGHT: The spectrum is CONTINUOUS with a phase transition at the Veil.
//! v < 1.0: Time/Space dominant (1D space, 3D time) - Oneness
//! v = 1.0: The Veil - qualitative break
//! v > 1.0: Space/Time dominant (3D space, 1D time) - Many-ness
//!
//! Phase 2 deliverables:
//! - Eight coupled oscillator fields for density bands
//! - Continuous spectrum position (not discrete states)
//! - Veil crossing dynamics at v=1
//! - Space/Time vs Time/Space coordinate transformation

mod coordinate_transform;
mod density_bands;
mod spectrum_position;
mod veil_crossing;

pub use coordinate_transform::{CoordinateTransform, SpaceTimeCoordinates, TimeSpaceCoordinates};
pub use density_bands::{DensityBandConfig, DensityBandOscillator, DensityBands};
pub use spectrum_position::{SpectrumConfig, SpectrumDynamics, SpectrumPosition};
pub use veil_crossing::{VeilCrossing, VeilState, VeilTransparency};

use crate::types::Float;

pub const NUM_DENSITY_BANDS: usize = 8;
pub const VEIL_POSITION: Float = 1.0;
pub const VEIL_THRESHOLD: Float = 0.1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpectrumSide {
    TimeSpace,
    AtVeil,
    SpaceTime,
}

impl SpectrumSide {
    pub fn from_velocity_ratio(v: Float) -> Self {
        if v < VEIL_POSITION - VEIL_THRESHOLD {
            SpectrumSide::TimeSpace
        } else if v > VEIL_POSITION + VEIL_THRESHOLD {
            SpectrumSide::SpaceTime
        } else {
            SpectrumSide::AtVeil
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SpectrumSide::TimeSpace => "Time/Space (Oneness)",
            SpectrumSide::AtVeil => "At the Veil (Transition)",
            SpectrumSide::SpaceTime => "Space/Time (Many-ness)",
        }
    }

    pub fn cosmological_description(&self) -> &'static str {
        match self {
            SpectrumSide::TimeSpace => {
                "1D space, 3D time - Unity consciousness, access to all time"
            }
            SpectrumSide::AtVeil => "The barrier between manifestations - where polarities meet",
            SpectrumSide::SpaceTime => {
                "3D space, 1D time - Individual consciousness, linear time perception"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VelocityRatio {
    pub value: Float,
}

impl VelocityRatio {
    pub fn new(value: Float) -> Self {
        Self {
            value: value.max(0.0),
        }
    }

    pub fn space_time_dominant() -> Self {
        Self { value: 2.0 }
    }

    pub fn time_space_dominant() -> Self {
        Self { value: 0.5 }
    }

    pub fn at_veil() -> Self {
        Self {
            value: VEIL_POSITION,
        }
    }

    pub fn balanced() -> Self {
        Self { value: 1.0 }
    }

    pub fn side(&self) -> SpectrumSide {
        SpectrumSide::from_velocity_ratio(self.value)
    }

    pub fn is_space_time(&self) -> bool {
        self.value > VEIL_POSITION
    }

    pub fn is_time_space(&self) -> bool {
        self.value < VEIL_POSITION
    }

    pub fn is_at_veil(&self) -> bool {
        (self.value - VEIL_POSITION).abs() < VEIL_THRESHOLD
    }

    pub fn distance_to_veil(&self) -> Float {
        (self.value - VEIL_POSITION).abs()
    }

    pub fn space_time_factor(&self) -> Float {
        if self.value <= VEIL_POSITION {
            0.0
        } else {
            (self.value - VEIL_POSITION) / self.value
        }
    }

    pub fn time_space_factor(&self) -> Float {
        if self.value >= VEIL_POSITION {
            0.0
        } else {
            (VEIL_POSITION - self.value) / VEIL_POSITION
        }
    }

    pub fn veil_proximity(&self) -> Float {
        1.0 - self.distance_to_veil().min(1.0)
    }
}

impl Default for VelocityRatio {
    fn default() -> Self {
        Self::balanced()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DensityPosition {
    pub value: Float,
}

impl DensityPosition {
    pub fn new(value: Float) -> Self {
        Self {
            value: value.clamp(0.0, 8.0),
        }
    }

    pub fn first_density() -> Self {
        Self { value: 1.0 }
    }

    pub fn second_density() -> Self {
        Self { value: 2.0 }
    }

    pub fn third_density() -> Self {
        Self { value: 3.0 }
    }

    pub fn fourth_density() -> Self {
        Self { value: 4.0 }
    }

    pub fn fifth_density() -> Self {
        Self { value: 5.0 }
    }

    pub fn sixth_density() -> Self {
        Self { value: 6.0 }
    }

    pub fn seventh_density() -> Self {
        Self { value: 7.0 }
    }

    pub fn eighth_density() -> Self {
        Self { value: 8.0 }
    }

    pub fn primary_density(&self) -> usize {
        self.value.floor() as usize
    }

    pub fn sub_density_phase(&self) -> Float {
        self.value.fract()
    }

    pub fn density_name(&self) -> &'static str {
        match self.primary_density() {
            0 => "Pre-Density (Potential)",
            1 => "1st Density (Awareness)",
            2 => "2nd Density (Growth)",
            3 => "3rd Density (Self-Awareness)",
            4 => "4th Density (Love/Light)",
            5 => "5th Density (Light/Light)",
            6 => "6th Density (Unity)",
            7 => "7th Density (Gateway)",
            8 => "8th Density (Octave)",
            _ => "Unknown",
        }
    }

    pub fn consciousness_level(&self) -> Float {
        let base = match self.primary_density() {
            0 => 0.01,
            1 => 0.05 + self.sub_density_phase() * 0.05,
            2 => 0.10 + self.sub_density_phase() * 0.10,
            3 => 0.20 + self.sub_density_phase() * 0.15,
            4 => 0.35 + self.sub_density_phase() * 0.20,
            5 => 0.55 + self.sub_density_phase() * 0.15,
            6 => 0.70 + self.sub_density_phase() * 0.15,
            7 => 0.85 + self.sub_density_phase() * 0.10,
            8 => 0.95,
            _ => 0.0,
        };
        base.min(1.0)
    }

    pub fn evolution_speed(&self) -> Float {
        match self.primary_density() {
            0 => 0.001,
            1 => 0.002,
            2 => 0.005,
            3 => 0.01,
            4 => 0.02,
            5 => 0.03,
            6 => 0.05,
            7 => 0.08,
            8 => 0.10,
            _ => 0.01,
        }
    }
}

impl Default for DensityPosition {
    fn default() -> Self {
        Self::third_density()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumState {
    pub density_amplitudes: [num_complex::Complex<Float>; NUM_DENSITY_BANDS],
    pub spectrum_position: VelocityRatio,
    pub density_position: DensityPosition,
    pub veil_transparency: Float,
}

impl SpectrumState {
    pub fn new() -> Self {
        Self {
            density_amplitudes: [num_complex::Complex::new(1.0, 0.0); NUM_DENSITY_BANDS],
            spectrum_position: VelocityRatio::default(),
            density_position: DensityPosition::default(),
            veil_transparency: 0.0,
        }
    }

    pub fn at_density(density: Float) -> Self {
        let mut state = Self::new();
        state.density_position = DensityPosition::new(density);
        state.update_amplitudes_for_density();
        state.update_veil_transparency();
        state
    }

    pub fn at_spectrum_position(velocity: Float) -> Self {
        let mut state = Self::new();
        state.spectrum_position = VelocityRatio::new(velocity);
        state.update_veil_transparency();
        state
    }

    fn update_amplitudes_for_density(&mut self) {
        let _primary = self.density_position.primary_density();
        let phase = self.density_position.sub_density_phase();

        for (i, amp) in self.density_amplitudes.iter_mut().enumerate() {
            let distance = (i as Float - self.density_position.value).abs();
            let magnitude = (-distance.powi(2) / 2.0).exp();
            let angle = phase * std::f64::consts::TAU * (i as Float / NUM_DENSITY_BANDS as Float);
            *amp = num_complex::Complex::from_polar(magnitude, angle);
        }
    }

    fn update_veil_transparency(&mut self) {
        self.veil_transparency = self.calculate_veil_transparency();
    }

    fn calculate_veil_transparency(&self) -> Float {
        let spectrum_factor = self.spectrum_position.veil_proximity();
        let density_factor = match self.density_position.primary_density() {
            0 | 1 => 0.05,
            2 => 0.10,
            3 => 0.10,
            4 => 0.30,
            5 => 0.50,
            6 => 0.80,
            7 => 1.00,
            8 => 1.00,
            _ => 0.0,
        };
        spectrum_factor * density_factor
    }

    pub fn side(&self) -> SpectrumSide {
        self.spectrum_position.side()
    }

    pub fn dominant_density_band(&self) -> usize {
        let mut max_mag = 0.0;
        let mut dominant = 0;
        for (i, amp) in self.density_amplitudes.iter().enumerate() {
            let mag = amp.norm();
            if mag > max_mag {
                max_mag = mag;
                dominant = i;
            }
        }
        dominant
    }

    pub fn total_amplitude(&self) -> Float {
        self.density_amplitudes.iter().map(|a| a.norm()).sum()
    }

    pub fn coherence(&self) -> Float {
        let mags: Vec<Float> = self.density_amplitudes.iter().map(|a| a.norm()).collect();
        let mean: Float = mags.iter().sum::<Float>() / mags.len() as Float;
        let variance: Float =
            mags.iter().map(|m| (m - mean).powi(2)).sum::<Float>() / mags.len() as Float;
        1.0 - variance.sqrt().min(1.0)
    }
}

impl Default for SpectrumState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_ratio_sides() {
        let st = VelocityRatio::space_time_dominant();
        assert_eq!(st.side(), SpectrumSide::SpaceTime);

        let ts = VelocityRatio::time_space_dominant();
        assert_eq!(ts.side(), SpectrumSide::TimeSpace);

        let veil = VelocityRatio::at_veil();
        assert_eq!(veil.side(), SpectrumSide::AtVeil);
    }

    #[test]
    fn test_velocity_ratio_factors() {
        let st = VelocityRatio::space_time_dominant();
        assert!(st.space_time_factor() > 0.0);
        assert_eq!(st.time_space_factor(), 0.0);

        let ts = VelocityRatio::time_space_dominant();
        assert_eq!(ts.space_time_factor(), 0.0);
        assert!(ts.time_space_factor() > 0.0);
    }

    #[test]
    fn test_density_position() {
        let d3 = DensityPosition::third_density();
        assert_eq!(d3.primary_density(), 3);
        assert_eq!(d3.sub_density_phase(), 0.0);

        let d3_5 = DensityPosition::new(3.5);
        assert_eq!(d3_5.primary_density(), 3);
        assert!((d3_5.sub_density_phase() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_density_names() {
        assert_eq!(
            DensityPosition::first_density().density_name(),
            "1st Density (Awareness)"
        );
        assert_eq!(
            DensityPosition::fourth_density().density_name(),
            "4th Density (Love/Light)"
        );
        assert_eq!(
            DensityPosition::eighth_density().density_name(),
            "8th Density (Octave)"
        );
    }

    #[test]
    fn test_consciousness_levels() {
        assert!(
            DensityPosition::first_density().consciousness_level()
                < DensityPosition::third_density().consciousness_level()
        );
        assert!(
            DensityPosition::third_density().consciousness_level()
                < DensityPosition::sixth_density().consciousness_level()
        );
    }

    #[test]
    fn test_spectrum_state_creation() {
        let state = SpectrumState::new();
        assert!((state.spectrum_position.value - 1.0).abs() < 1e-10);
        assert!((state.density_position.value - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectrum_state_at_density() {
        let state = SpectrumState::at_density(5.0);
        assert_eq!(state.density_position.primary_density(), 5);
    }

    #[test]
    fn test_veil_transparency() {
        let d3 = SpectrumState::at_density(3.0);
        let d6 = SpectrumState::at_density(6.0);

        assert!(d6.veil_transparency > d3.veil_transparency);
    }

    #[test]
    fn test_spectrum_side_from_velocity() {
        assert_eq!(
            SpectrumSide::from_velocity_ratio(0.5),
            SpectrumSide::TimeSpace
        );
        assert_eq!(
            SpectrumSide::from_velocity_ratio(1.5),
            SpectrumSide::SpaceTime
        );
        assert_eq!(SpectrumSide::from_velocity_ratio(1.0), SpectrumSide::AtVeil);
    }
}
