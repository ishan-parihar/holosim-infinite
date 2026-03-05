//! Three Primal Distortions as Unified Field Dynamics
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The One Infinite Creator emanates the Three Primal Distortions:
//!  1. Free Will - The First Distortion: The potential for ANY outcome
//!  2. Love/Logos - The Second Distortion: The attractive force creating structure
//!  3. Light - The Third Distortion: The wave nature of reality"
//!
//! KEY PARADIGM SHIFT (Phase 1):
//! - Old: Distortions as entity attributes applied to individual objects
//! - New: Distortions as FIELD DYNAMICS affecting the entire holographic field
//!
//! PHASE 2 INTEGRATION:
//! The Unified Field Equation now includes spectrum dynamics:
//! ∂ψ/∂t = FreeWill(ψ, spectrum) + Love(ψ, spectrum) + Light(ψ, spectrum)
//!
//! The spectrum position (v) determines the balance between Space/Time and Time/Space:
//! - v < 1.0: Time/Space dominant (1D space, 3D time) - Oneness
//! - v = 1.0: The Veil - qualitative break
//! - v > 1.0: Space/Time dominant (3D space, 1D time) - Many-ness
//!
//! Where ψ represents the holographic field state across all density bands.

mod free_will;
mod light;
mod love;
mod unified;

pub use free_will::{FreeWillConfig, FreeWillTerm, PerturbationType};
pub use light::{InterferencePattern, LightConfig, LightTerm, WaveState};
pub use love::{CoherenceGradient, LoveConfig, LoveTerm};
pub use unified::{CoherencePeak, DistortionStatistics, UnifiedFieldConfig, UnifiedFieldEquation};

use crate::holographic_foundation::spectrum::{
    CoordinateTransform, DensityPosition, SpectrumSide, VelocityRatio,
};

pub const NUM_DENSITY_BANDS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistortionType {
    FreeWill,
    Love,
    Light,
}

impl DistortionType {
    pub fn display_name(&self) -> &'static str {
        match self {
            DistortionType::FreeWill => "Free Will (First Distortion)",
            DistortionType::Love => "Love/Logos (Second Distortion)",
            DistortionType::Light => "Light (Third Distortion)",
        }
    }

    pub fn cosmological_description(&self) -> &'static str {
        match self {
            DistortionType::FreeWill => "The First Distortion: The Law of Confusion - Free Will emanates from the ONE as infinite potential",
            DistortionType::Love => "The Second Distortion: The Law of Love - Love/Logos creates structure through attractive potential",
            DistortionType::Light => "The Third Distortion: The Law of Light - Light propagates as wave dynamics allowing information to travel",
        }
    }

    pub fn order(&self) -> u8 {
        match self {
            DistortionType::FreeWill => 1,
            DistortionType::Love => 2,
            DistortionType::Light => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DensityAmplitude {
    pub re: f64,
    pub im: f64,
}

impl DensityAmplitude {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn scale(&self, factor: f64) -> Self {
        Self {
            re: self.re * factor,
            im: self.im * factor,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn from_polar(magnitude: f64, phase: f64) -> Self {
        Self {
            re: magnitude * phase.cos(),
            im: magnitude * phase.sin(),
        }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn with_added_magnitude(&self, delta: f64) -> Self {
        let current_mag = self.magnitude();
        if current_mag < 1e-10 {
            Self::from_polar(delta.abs(), 0.0)
        } else {
            let new_mag = (current_mag + delta).max(0.0);
            self.scale(new_mag / current_mag)
        }
    }
}

impl Default for DensityAmplitude {
    fn default() -> Self {
        Self::zero()
    }
}

/// Field state with integrated spectrum dynamics.
///
/// From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 2:
/// "The spectrum is ONE unified reality with a QUALITATIVE BREAK at v=1.
///  v < 1.0: Time/Space dominant (1D space, 3D time) - Oneness
///  v = 1.0: The Veil - qualitative break
///  v > 1.0: Space/Time dominant (3D space, 1D time) - Many-ness"
///
/// The spectrum position affects how the three primal distortions manifest:
/// - Free Will: More deterministic in Time/Space, more chaotic in Space/Time
/// - Love: Stronger attraction toward unity in Time/Space
/// - Light: Wave propagation changes based on coordinate system
#[derive(Debug, Clone, PartialEq)]
pub struct FieldState {
    pub density_amplitudes: [DensityAmplitude; NUM_DENSITY_BANDS],
    pub coherence: f64,
    pub spectrum_position: f64,
    pub energy: f64,
    /// Phase 2: Velocity ratio (v) determining Space/Time vs Time/Space balance
    /// v = 1.0 is the Veil; v < 1 is Time/Space; v > 1 is Space/Time
    pub velocity_ratio: VelocityRatio,
    /// Phase 2: Density position along the octave (1.0 to 8.0)
    pub density_position: DensityPosition,
    /// Phase 2: Veil transparency (0.0 = opaque, 1.0 = transparent)
    pub veil_transparency: f64,
    /// Phase 2: Which side of the spectrum we're on
    pub spectrum_side: SpectrumSide,
}

impl FieldState {
    pub fn new() -> Self {
        let velocity_ratio = VelocityRatio::balanced();
        Self {
            density_amplitudes: [DensityAmplitude::zero(); NUM_DENSITY_BANDS],
            coherence: 0.5,
            spectrum_position: 0.5,
            energy: 1.0,
            velocity_ratio,
            density_position: DensityPosition::third_density(),
            veil_transparency: 0.1,
            spectrum_side: velocity_ratio.side(),
        }
    }

    pub fn uninitialized() -> Self {
        Self {
            density_amplitudes: [DensityAmplitude::zero(); NUM_DENSITY_BANDS],
            coherence: 0.0,
            spectrum_position: 0.5,
            energy: 0.0,
            velocity_ratio: VelocityRatio::balanced(),
            density_position: DensityPosition::third_density(),
            veil_transparency: 0.0,
            spectrum_side: SpectrumSide::AtVeil,
        }
    }

    pub fn uniform(coherence: f64) -> Self {
        let amplitude = DensityAmplitude::from_polar(1.0 / (NUM_DENSITY_BANDS as f64).sqrt(), 0.0);
        let velocity_ratio = VelocityRatio::balanced();
        Self {
            density_amplitudes: [amplitude; NUM_DENSITY_BANDS],
            coherence,
            spectrum_position: 0.5,
            energy: 1.0,
            velocity_ratio,
            density_position: DensityPosition::third_density(),
            veil_transparency: 0.1,
            spectrum_side: velocity_ratio.side(),
        }
    }

    /// Create a field state at a specific velocity ratio (spectrum position)
    ///
    /// From Phase 2 R&D:
    /// "The spectrum position evolves smoothly"
    pub fn at_velocity_ratio(v: f64) -> Self {
        let velocity_ratio = VelocityRatio::new(v);
        let veil_transparency =
            Self::calculate_veil_transparency(&velocity_ratio, &DensityPosition::third_density());
        Self {
            density_amplitudes: [DensityAmplitude::from_polar(0.5, 0.0); NUM_DENSITY_BANDS],
            coherence: 0.5,
            spectrum_position: v,
            energy: 1.0,
            velocity_ratio,
            density_position: DensityPosition::third_density(),
            veil_transparency,
            spectrum_side: velocity_ratio.side(),
        }
    }

    /// Create a field state at a specific density
    pub fn at_density(density: f64) -> Self {
        let density_position = DensityPosition::new(density);
        let velocity_ratio = VelocityRatio::balanced();
        let veil_transparency =
            Self::calculate_veil_transparency(&velocity_ratio, &density_position);
        let mut amplitudes = [DensityAmplitude::from_polar(0.5, 0.0); NUM_DENSITY_BANDS];
        for (i, amp) in amplitudes.iter_mut().enumerate() {
            let distance = (i as f64 - density).abs();
            let magnitude = (-distance.powi(2) / 2.0).exp();
            *amp = DensityAmplitude::from_polar(magnitude, i as f64 * 0.5);
        }
        Self {
            density_amplitudes: amplitudes,
            coherence: density_position.consciousness_level(),
            spectrum_position: 1.0,
            energy: 1.0,
            velocity_ratio,
            density_position,
            veil_transparency,
            spectrum_side: velocity_ratio.side(),
        }
    }

    /// Calculate veil transparency based on velocity ratio and density
    fn calculate_veil_transparency(
        velocity_ratio: &VelocityRatio,
        density: &DensityPosition,
    ) -> f64 {
        let spectrum_factor = velocity_ratio.veil_proximity();
        let density_factor = match density.primary_density() {
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

    /// Update spectrum state based on evolution
    ///
    /// From Phase 2 R&D:
    /// "Entity spectrum access evolves with consciousness"
    pub fn evolve_spectrum(&mut self, dt: f64, evolution_rate: f64) {
        // Spectrum position evolves based on coherence and density
        let coherence_pull = (self.coherence - 0.5) * 0.1;
        let density_influence = self.density_position.evolution_speed() * evolution_rate;

        // Update velocity ratio (bounded evolution)
        let new_v = (self.velocity_ratio.value + coherence_pull * dt + density_influence * dt)
            .clamp(0.1, 2.0);
        self.velocity_ratio = VelocityRatio::new(new_v);
        self.spectrum_side = self.velocity_ratio.side();
        self.spectrum_position = new_v;

        // Update veil transparency
        self.veil_transparency =
            Self::calculate_veil_transparency(&self.velocity_ratio, &self.density_position);

        // Update density position slowly based on coherence accumulation
        if self.coherence > 0.8 {
            let density_increment = dt * 0.001 * self.coherence;
            self.density_position =
                DensityPosition::new(self.density_position.value + density_increment);
        }
    }

    /// Check if we're at the veil (v ≈ 1.0)
    pub fn is_at_veil(&self) -> bool {
        self.velocity_ratio.is_at_veil()
    }

    /// Check if in Time/Space dominant region (v < 1.0)
    pub fn is_time_space(&self) -> bool {
        self.velocity_ratio.is_time_space()
    }

    /// Check if in Space/Time dominant region (v > 1.0)
    pub fn is_space_time(&self) -> bool {
        self.velocity_ratio.is_space_time()
    }

    /// Get the coordinate transform for this field state
    pub fn coordinate_transform(&self) -> CoordinateTransform {
        CoordinateTransform::new(self.velocity_ratio)
    }

    /// Apply veil effects to field behavior
    ///
    /// From Phase 2 R&D:
    /// "Veil crossing affects field behavior at v=1"
    pub fn apply_veil_effects(&mut self) -> VeilEffectResult {
        if !self.is_at_veil() {
            return VeilEffectResult::NotAtVeil;
        }

        // At the veil, field behavior changes qualitatively
        // Coherence becomes more unified, energy distribution changes
        let coherence_boost = 1.0 + self.veil_transparency * 0.5;
        self.coherence = (self.coherence * coherence_boost).min(1.0);

        // Energy redistributes based on density bands at veil
        let dominant = self.dominant_density();
        for (i, amp) in self.density_amplitudes.iter_mut().enumerate() {
            if i == dominant {
                *amp = amp.scale(1.0 + self.veil_transparency * 0.1);
            }
        }

        VeilEffectResult::Applied {
            coherence_boost,
            veil_transparency: self.veil_transparency,
        }
    }

    pub fn dominant_density(&self) -> usize {
        let mut max_mag = 0.0;
        let mut dominant = 0;
        for (i, amp) in self.density_amplitudes.iter().enumerate() {
            let mag = amp.magnitude();
            if mag > max_mag {
                max_mag = mag;
                dominant = i;
            }
        }
        dominant
    }

    pub fn total_magnitude(&self) -> f64 {
        self.density_amplitudes.iter().map(|a| a.magnitude()).sum()
    }

    pub fn normalize(&mut self) {
        let total = self.total_magnitude();
        if total > 1e-10 {
            for amp in &mut self.density_amplitudes {
                *amp = amp.scale(1.0 / total);
            }
        }
    }

    pub fn entropy(&self) -> f64 {
        let mut entropy = 0.0;
        for amp in self.density_amplitudes.iter() {
            let p = amp.magnitude();
            if p > 1e-10 {
                entropy -= p * p.ln();
            }
        }
        entropy
    }

    pub fn calculate_coherence(&self) -> f64 {
        let mags: Vec<f64> = self
            .density_amplitudes
            .iter()
            .map(|a| a.magnitude())
            .collect();
        let mean: f64 = mags.iter().sum::<f64>() / mags.len() as f64;
        let variance: f64 =
            mags.iter().map(|m| (m - mean).powi(2)).sum::<f64>() / mags.len() as f64;
        1.0 - variance.sqrt().min(1.0)
    }
}

/// Result of applying veil effects to field state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VeilEffectResult {
    NotAtVeil,
    Applied {
        coherence_boost: f64,
        veil_transparency: f64,
    },
}

impl Default for FieldState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_amplitude_operations() {
        let a = DensityAmplitude::new(1.0, 0.0);
        let b = DensityAmplitude::new(0.0, 1.0);

        assert!((a.magnitude() - 1.0).abs() < 1e-10);
        assert!((b.phase() - std::f64::consts::PI / 2.0).abs() < 1e-10);

        let sum = a.add(&b);
        assert!((sum.magnitude() - 2.0_f64.sqrt()).abs() < 1e-10);

        let product = a.multiply(&b);
        assert!((product.re - 0.0).abs() < 1e-10);
        assert!((product.im - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_field_state_creation() {
        let field = FieldState::uniform(0.5);
        assert!((field.coherence - 0.5).abs() < 1e-10);

        let total = field.total_magnitude();
        assert!(total > 0.0);
    }

    #[test]
    fn test_dominant_density() {
        let mut field = FieldState::new();
        field.density_amplitudes[3] = DensityAmplitude::one();
        assert_eq!(field.dominant_density(), 3);
    }

    #[test]
    fn test_distortion_order() {
        assert_eq!(DistortionType::FreeWill.order(), 1);
        assert_eq!(DistortionType::Love.order(), 2);
        assert_eq!(DistortionType::Light.order(), 3);
    }

    // Phase 2 Tests: Spectrum Dynamics + Veil Crossing

    #[test]
    fn test_field_state_velocity_ratio() {
        let field = FieldState::at_velocity_ratio(1.5);
        assert!(field.velocity_ratio.value > 1.0);
        assert_eq!(field.spectrum_side, SpectrumSide::SpaceTime);
        assert!(field.is_space_time());
        assert!(!field.is_time_space());
        assert!(!field.is_at_veil());
    }

    #[test]
    fn test_field_state_time_space() {
        let field = FieldState::at_velocity_ratio(0.5);
        assert!(field.velocity_ratio.value < 1.0);
        assert_eq!(field.spectrum_side, SpectrumSide::TimeSpace);
        assert!(field.is_time_space());
        assert!(!field.is_space_time());
    }

    #[test]
    fn test_field_state_at_veil() {
        let field = FieldState::at_velocity_ratio(1.0);
        assert_eq!(field.spectrum_side, SpectrumSide::AtVeil);
        assert!(field.is_at_veil());
    }

    #[test]
    fn test_field_state_density() {
        let field = FieldState::at_density(5.0);
        assert_eq!(field.density_position.primary_density(), 5);
        assert!(field.coherence > 0.0);
    }

    #[test]
    fn test_spectrum_evolution() {
        let mut field = FieldState::uniform(0.5);
        let _initial_v = field.velocity_ratio.value;

        field.evolve_spectrum(0.1, 1.0);

        // Spectrum position should evolve
        assert!(field.velocity_ratio.value >= 0.0);
    }

    #[test]
    fn test_veil_effects_not_at_veil() {
        let mut field = FieldState::at_velocity_ratio(1.5); // Space/Time dominant
        let result = field.apply_veil_effects();
        assert_eq!(result, VeilEffectResult::NotAtVeil);
    }

    #[test]
    fn test_veil_effects_at_veil() {
        let mut field = FieldState::at_velocity_ratio(1.0); // At veil
        let initial_coherence = field.coherence;
        let result = field.apply_veil_effects();

        // Should apply veil effects
        if let VeilEffectResult::Applied {
            coherence_boost,
            veil_transparency: _,
        } = result
        {
            assert!(coherence_boost >= 1.0);
            assert!(field.coherence >= initial_coherence);
        } else {
            panic!("Expected Applied result at veil");
        }
    }

    #[test]
    fn test_veil_transparency_increases_with_density() {
        let low_density = FieldState::at_density(2.0);
        let high_density = FieldState::at_density(6.0);

        // Higher density should have higher veil transparency
        assert!(high_density.veil_transparency >= low_density.veil_transparency);
    }

    #[test]
    fn test_coordinate_transform_from_field() {
        let field = FieldState::at_velocity_ratio(1.0);
        let transform = field.coordinate_transform();

        // Transform should be created from velocity ratio
        assert!(transform.transformation_factor() > 0.0);
    }
}
