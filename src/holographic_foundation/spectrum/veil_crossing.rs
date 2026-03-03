//! Veil Crossing Dynamics
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Veil is the barrier at v=1 that separates Space/Time from Time/Space.
//!  At the Veil, a coordinate transformation occurs:
//!  Space/Time (3D space, 1D time) ↔ Time/Space (1D space, 3D time)
//!
//!  The Veil has THICKNESS that varies by density:
//!  - D1-D2: Thick (less transparent)
//!  - D3: Medium thickness
//!  - D4-D5: Thinning
//!  - D6-D8: Thin or transparent"
//!
//! KEY INSIGHT: Veil is not a binary barrier but a CONTINUOUS TRANSPARENCY field.
//! Transparency increases with evolution and varies across the density octave.

use super::{DensityPosition, VelocityRatio, VEIL_POSITION, VEIL_THRESHOLD};
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VeilTransparency {
    pub value: Float,
}

impl VeilTransparency {
    pub fn new(value: Float) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
        }
    }

    pub fn opaque() -> Self {
        Self { value: 0.0 }
    }

    pub fn transparent() -> Self {
        Self { value: 1.0 }
    }

    pub fn is_opaque(&self) -> bool {
        self.value < 0.1
    }

    pub fn is_transparent(&self) -> bool {
        self.value > 0.9
    }

    pub fn partial(&self) -> bool {
        self.value >= 0.1 && self.value <= 0.9
    }
}

impl Default for VeilTransparency {
    fn default() -> Self {
        Self { value: 0.1 }
    }
}

#[derive(Debug, Clone)]
pub struct VeilState {
    transparency: VeilTransparency,
    thickness: Float,
    thin_spots: Vec<ThinSpot>,
    piercing_accumulation: Float,
    density_modifier: Float,
    spectrum_modifier: Float,
}

impl VeilState {
    pub fn new(density: &DensityPosition, spectrum: &VelocityRatio) -> Self {
        let density_modifier = Self::calculate_density_modifier(density);
        let spectrum_modifier = Self::calculate_spectrum_modifier(spectrum);

        let base_transparency = density_modifier * spectrum_modifier;
        let base_thickness = 1.0 - density_modifier;

        Self {
            transparency: VeilTransparency::new(base_transparency),
            thickness: base_thickness,
            thin_spots: Vec::new(),
            piercing_accumulation: 0.0,
            density_modifier,
            spectrum_modifier,
        }
    }

    fn calculate_density_modifier(density: &DensityPosition) -> Float {
        match density.primary_density() {
            0 => 0.05,
            1 => 0.05,
            2 => 0.10,
            3 => 0.10,
            4 => 0.30,
            5 => 0.50,
            6 => 0.80,
            7 => 1.00,
            8 => 1.00,
            _ => 0.0,
        }
    }

    fn calculate_spectrum_modifier(spectrum: &VelocityRatio) -> Float {
        let proximity = spectrum.veil_proximity();
        let at_veil = spectrum.is_at_veil();

        if at_veil {
            1.0
        } else {
            0.3 + 0.7 * proximity
        }
    }

    pub fn update(&mut self, density: &DensityPosition, spectrum: &VelocityRatio) {
        self.density_modifier = Self::calculate_density_modifier(density);
        self.spectrum_modifier = Self::calculate_spectrum_modifier(spectrum);

        let base_transparency = self.density_modifier * self.spectrum_modifier;
        let piercing_bonus = self.piercing_accumulation * 0.1;

        self.transparency = VeilTransparency::new((base_transparency + piercing_bonus).min(1.0));
        self.thickness = 1.0 - self.transparency.value;

        // Decay thin spots
        self.thin_spots.retain(|spot| spot.strength > 0.01);
        for spot in &mut self.thin_spots {
            spot.strength *= 0.99;
        }
    }

    pub fn pierce(&mut self, strength: Float) -> PiercingResult {
        if self.thickness < 0.01 {
            return PiercingResult::AlreadyPierced;
        }

        let effective_strength = strength * (1.0 - self.transparency.value);
        self.piercing_accumulation += effective_strength;

        // Create thin spot
        self.thin_spots.push(ThinSpot {
            position: 0.0,
            strength: effective_strength,
            age: 0,
        });

        // Update transparency
        self.transparency =
            VeilTransparency::new((self.transparency.value + effective_strength * 0.1).min(1.0));
        self.thickness = 1.0 - self.transparency.value;

        if self.transparency.is_transparent() {
            PiercingResult::FullyPierced
        } else if effective_strength > 0.3 {
            PiercingResult::SignificantPiercing
        } else {
            PiercingResult::MinorPiercing
        }
    }

    pub fn thin_spots(&self) -> &[ThinSpot] {
        &self.thin_spots
    }

    pub fn transparency(&self) -> Float {
        self.transparency.value
    }

    pub fn thickness(&self) -> Float {
        self.thickness
    }

    pub fn is_pierceable(&self) -> bool {
        self.thickness > 0.01 && self.thickness < 0.5
    }
}

impl Default for VeilState {
    fn default() -> Self {
        Self::new(&DensityPosition::default(), &VelocityRatio::default())
    }
}

#[derive(Debug, Clone)]
pub struct ThinSpot {
    pub position: Float,
    pub strength: Float,
    pub age: usize,
}

impl ThinSpot {
    pub fn new(position: Float, strength: Float) -> Self {
        Self {
            position,
            strength,
            age: 0,
        }
    }

    pub fn age(&mut self) {
        self.age += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PiercingResult {
    AlreadyPierced,
    MinorPiercing,
    SignificantPiercing,
    FullyPierced,
}

#[derive(Debug, Clone)]
pub struct VeilCrossing {
    state: VeilState,
    crossing_events: Vec<CrossingEvent>,
    total_crossings: usize,
    successful_crossings: usize,
}

impl VeilCrossing {
    pub fn new(density: &DensityPosition, spectrum: &VelocityRatio) -> Self {
        Self {
            state: VeilState::new(density, spectrum),
            crossing_events: Vec::new(),
            total_crossings: 0,
            successful_crossings: 0,
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(&DensityPosition::default(), &VelocityRatio::default())
    }

    /// Attempt to cross the veil
    ///
    /// Crossing probability depends on:
    /// - Veil transparency
    /// - Spectrum position proximity to v=1
    /// - Consciousness level (density)
    pub fn attempt_crossing(
        &mut self,
        spectrum: &VelocityRatio,
        density: &DensityPosition,
    ) -> CrossingResult {
        self.total_crossings += 1;

        // Update veil state
        self.state.update(density, spectrum);

        // Check if already on other side
        if !spectrum.is_at_veil() && self.state.transparency.value < 0.5 {
            return CrossingResult::NotAtVeil;
        }

        // Calculate crossing probability
        let transparency_factor = self.state.transparency.value;
        let consciousness_factor = density.consciousness_level();
        let spectrum_factor = spectrum.veil_proximity();

        let crossing_probability =
            transparency_factor * 0.4 + consciousness_factor * 0.3 + spectrum_factor * 0.3;

        // Record crossing attempt
        let event = CrossingEvent {
            spectrum_position: spectrum.value,
            density_position: density.value,
            transparency: self.state.transparency.value,
            probability: crossing_probability,
            success: crossing_probability > 0.5,
        };
        self.crossing_events.push(event);

        if crossing_probability > 0.5 {
            self.successful_crossings += 1;
            CrossingResult::Success {
                new_transparency: self.state.transparency.value,
            }
        } else if crossing_probability > 0.3 {
            // Partial crossing - thin the veil
            self.state.pierce(crossing_probability);
            CrossingResult::Partial {
                transparency: self.state.transparency.value,
            }
        } else {
            CrossingResult::Failed {
                reason: if self.state.thickness > 0.8 {
                    "Veil too thick"
                } else if spectrum.distance_to_veil() > 0.3 {
                    "Not at veil position"
                } else {
                    "Insufficient consciousness"
                },
            }
        }
    }

    /// Force a crossing (for high-consciousness entities or special events)
    pub fn force_crossing(&mut self) -> CrossingResult {
        self.state.transparency = VeilTransparency::transparent();
        self.state.thickness = 0.0;
        self.successful_crossings += 1;
        self.total_crossings += 1;

        CrossingResult::Success {
            new_transparency: 1.0,
        }
    }

    /// Apply catalyst to thin the veil
    pub fn apply_catalyst(&mut self, intensity: Float) {
        let piercing_result = self.state.pierce(intensity);
        self.total_crossings += 1;

        if piercing_result == PiercingResult::FullyPierced {
            self.successful_crossings += 1;
        }
    }

    pub fn state(&self) -> &VeilState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut VeilState {
        &mut self.state
    }

    pub fn total_crossings(&self) -> usize {
        self.total_crossings
    }

    pub fn successful_crossings(&self) -> usize {
        self.successful_crossings
    }

    pub fn crossing_rate(&self) -> Float {
        if self.total_crossings == 0 {
            0.0
        } else {
            self.successful_crossings as Float / self.total_crossings as Float
        }
    }

    pub fn crossing_events(&self) -> &[CrossingEvent] {
        &self.crossing_events
    }
}

#[derive(Debug, Clone)]
pub struct CrossingEvent {
    pub spectrum_position: Float,
    pub density_position: Float,
    pub transparency: Float,
    pub probability: Float,
    pub success: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrossingResult {
    NotAtVeil,
    Failed { reason: &'static str },
    Partial { transparency: Float },
    Success { new_transparency: Float },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veil_transparency() {
        let opaque = VeilTransparency::opaque();
        assert!(opaque.is_opaque());

        let transparent = VeilTransparency::transparent();
        assert!(transparent.is_transparent());

        let partial = VeilTransparency::new(0.5);
        assert!(partial.partial());
    }

    #[test]
    fn test_veil_state_creation() {
        let state = VeilState::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());
        assert!(state.thickness > 0.0);
        assert!(state.transparency() >= 0.0);
    }

    #[test]
    fn test_veil_density_modifier() {
        let d3 = VeilState::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());
        let d6 = VeilState::new(&DensityPosition::sixth_density(), &VelocityRatio::at_veil());

        // Higher density should have higher transparency
        assert!(d6.transparency() > d3.transparency());
    }

    #[test]
    fn test_veil_spectrum_modifier() {
        let at_veil = VeilState::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());
        let far_from_veil = VeilState::new(
            &DensityPosition::third_density(),
            &VelocityRatio::space_time_dominant(),
        );

        // At veil should have higher modifier
        assert!(at_veil.spectrum_modifier > far_from_veil.spectrum_modifier);
    }

    #[test]
    fn test_veil_piercing() {
        let mut state =
            VeilState::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());
        let initial_transparency = state.transparency();

        state.pierce(0.5);

        // Piercing should increase transparency
        assert!(state.transparency() > initial_transparency);
    }

    #[test]
    fn test_veil_crossing_creation() {
        let crossing = VeilCrossing::from_defaults();
        assert_eq!(crossing.total_crossings(), 0);
    }

    #[test]
    fn test_veil_crossing_attempt() {
        let mut crossing =
            VeilCrossing::new(&DensityPosition::sixth_density(), &VelocityRatio::at_veil());

        let result =
            crossing.attempt_crossing(&VelocityRatio::at_veil(), &DensityPosition::sixth_density());

        // High density at veil should have good crossing chance
        assert!(matches!(
            result,
            CrossingResult::Success { .. } | CrossingResult::Partial { .. }
        ));
    }

    #[test]
    fn test_veil_crossing_not_at_veil() {
        let mut crossing = VeilCrossing::from_defaults();

        let result = crossing.attempt_crossing(
            &VelocityRatio::space_time_dominant(),
            &DensityPosition::third_density(),
        );

        assert_eq!(result, CrossingResult::NotAtVeil);
    }

    #[test]
    fn test_force_crossing() {
        let mut crossing = VeilCrossing::from_defaults();

        let result = crossing.force_crossing();

        assert!(matches!(result, CrossingResult::Success { .. }));
        assert!((crossing.state().transparency() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_catalyst() {
        let mut crossing =
            VeilCrossing::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());

        let initial_transparency = crossing.state().transparency();

        crossing.apply_catalyst(0.5);

        // Catalyst should increase transparency
        assert!(crossing.state().transparency() >= initial_transparency);
    }

    #[test]
    fn test_crossing_rate() {
        let mut crossing =
            VeilCrossing::new(&DensityPosition::sixth_density(), &VelocityRatio::at_veil());

        // Attempt several crossings
        for _ in 0..10 {
            crossing.attempt_crossing(&VelocityRatio::at_veil(), &DensityPosition::sixth_density());
        }

        let rate = crossing.crossing_rate();
        assert!(rate >= 0.0 && rate <= 1.0);
    }

    #[test]
    fn test_thin_spot_creation() {
        let spot = ThinSpot::new(0.5, 0.3);
        assert!((spot.position - 0.5).abs() < 1e-10);
        assert!((spot.strength - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_piercing_result() {
        let mut state =
            VeilState::new(&DensityPosition::third_density(), &VelocityRatio::at_veil());

        // Multiple small piercings
        for _ in 0..10 {
            state.pierce(0.1);
        }

        assert!(state.transparency() > 0.1);
    }
}
