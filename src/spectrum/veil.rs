//! # The Veil
//!
//! This module implements the Veil as a structural feature of dimensional architecture.
//!
//! ## Theoretical Foundation
//!
//! The Veil is **not a barrier separating two realms**—it is a **structural feature of dimensional architecture** formed at Yellow-Ray by the fundamental opposition between Time/Space Oneness and Space/Time Many-ness.
//!
//! ## What the Veil Does
//!
//! - Limits **ACCESS** to the Oneness side of the spectrum
//! - Creates the **illusion of separation**—the illusion that Space/Time is the only reality
//! - Provides **contrast, limitation, challenge, choice, and growth**—the depth required for evolution
//! - Implements **Free Will**—the First Distortion becomes meaningful through this mechanism
//!
//! ## What the Veil Does NOT Do
//!
//! - Separate two independent realms (Space/Time and Time/Space are not separate)
//! - Create two different realities (they are ONE spectrum)
//! - Prevent the spectrum from existing (the entire spectrum exists simultaneously)
//!
//! ## The Veil as Structural Feature
//!
//! The Veil forms at v = 1 (the transition point between space/time and time/space). It is a **structural feature** of dimensional architecture, not a separator. The entire spectrum exists simultaneously; the Veil only limits **access** to the Oneness side.
//!
//! ## Phase 4: Veil Enhancement
//!
//! From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
//! - Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th)
//! - Veil limits access to Time/Space based on transparency
//! - Veil limits access to holographic connections based on transparency
//! - Veil limits access to higher consciousness based on transparency
//! - Veil provides contrast, limitation, challenge, choice, and growth

use super::larson_framework::SpectrumRatio;
use crate::evolution_density_octave::density_octave::Density;

/// Represents access to the spectrum
#[derive(Debug, Clone)]
pub struct SpectrumAccess {
    /// Access to the Oneness side of the spectrum (0.0 to 1.0)
    pub oneness_access: f64,
    /// Access to the Many-ness side of the spectrum (0.0 to 1.0)
    pub many_ness_access: f64,
}

impl SpectrumAccess {
    /// Creates a new spectrum access with full access to both sides
    pub fn full() -> Self {
        SpectrumAccess {
            oneness_access: 1.0,
            many_ness_access: 1.0,
        }
    }

    /// Creates a new spectrum access with limited access
    pub fn limited(oneness_access: f64, many_ness_access: f64) -> Self {
        SpectrumAccess {
            oneness_access: oneness_access.clamp(0.0, 1.0),
            many_ness_access: many_ness_access.clamp(0.0, 1.0),
        }
    }

    /// Creates a new spectrum access with only Many-ness access (full veil)
    pub fn space_time_only() -> Self {
        SpectrumAccess {
            oneness_access: 0.0,
            many_ness_access: 1.0,
        }
    }

    /// Creates a new spectrum access with only Oneness access (no veil)
    pub fn time_space_only() -> Self {
        SpectrumAccess {
            oneness_access: 1.0,
            many_ness_access: 0.0,
        }
    }

    /// Checks if access is limited by the Veil
    pub fn is_veiled(&self) -> bool {
        self.oneness_access < 1.0
    }

    /// Gets the veil strength (0.0 = no veil, 1.0 = full veil)
    pub fn veil_strength(&self) -> f64 {
        1.0 - self.oneness_access
    }

    /// Checks if the entity has no veil access
    pub fn fully_veiled(&self) -> bool {
        self.oneness_access == 0.0
    }

    /// Checks if the entity has full spectrum access
    pub fn unobstructed(&self) -> bool {
        self.oneness_access == 1.0 && self.many_ness_access == 1.0
    }
}

/// Represents access control provided by the Veil
///
/// Phase 4: Veil Enhancement
/// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
/// "Veil limits access to Time/Space, holographic connections, and higher consciousness"
#[derive(Debug, Clone)]
pub struct VeilAccessControl {
    /// Access to Time/Space (0.0 to 1.0)
    /// Higher values indicate more access to Time/Space (unity consciousness)
    pub time_space_access: f64,
    /// Access to holographic connections (0.0 to 1.0)
    /// Higher values indicate more ability to form holographic connections
    pub holographic_connection_access: f64,
    /// Access to higher consciousness (0.0 to 1.0)
    /// Higher values indicate more access to higher consciousness levels
    pub higher_consciousness_access: f64,
}

impl VeilAccessControl {
    /// Create new access control based on veil transparency
    pub fn from_transparency(transparency: f64) -> Self {
        // Time/Space access is direct relationship with transparency
        let time_space_access = transparency;

        // Holographic connections require more veil thinning (0.8 multiplier)
        let holographic_connection_access = transparency * 0.8;

        // Higher consciousness requires the most veil thinning (0.6 multiplier)
        let higher_consciousness_access = transparency * 0.6;

        VeilAccessControl {
            time_space_access,
            holographic_connection_access,
            higher_consciousness_access,
        }
    }

    /// Check if entity can access Time/Space at required level
    pub fn can_access_time_space(&self, required_access: f64) -> bool {
        self.time_space_access >= required_access
    }

    /// Check if entity can form holographic connections at required level
    pub fn can_access_holographic_connections(&self, required_access: f64) -> bool {
        self.holographic_connection_access >= required_access
    }

    /// Check if entity can access higher consciousness at required level
    pub fn can_access_higher_consciousness(&self, required_access: f64) -> bool {
        self.higher_consciousness_access >= required_access
    }
}

/// Represents the Veil as a structural feature
#[derive(Debug, Clone)]
pub struct Veil {
    /// The spectrum position where the Veil forms (v = 1)
    pub spectrum_position: SpectrumRatio,
    /// The access limitation (0.0 = no limitation, 1.0 = full limitation)
    pub access_limitation: f64,
    /// The current spectrum access
    pub current_access: SpectrumAccess,
    /// Phase 4: Veil transparency (0.0 = fully opaque, 1.0 = fully transparent)
    pub transparency: f64,
    /// Phase 4: Access control based on veil transparency
    pub access_control: VeilAccessControl,
    /// Phase 4: Illusion strength (0.0 = no illusion, 1.0 = complete illusion)
    pub illusion_strength: f64,
}

impl Veil {
    /// Creates a new Veil at the transition point
    pub fn new(access_limitation: f64) -> Self {
        let spectrum_position = SpectrumRatio::space_time(1.0, 1.0);
        let current_access = SpectrumAccess::limited(1.0 - access_limitation, 1.0);
        let transparency = 1.0 - access_limitation;
        let access_control = VeilAccessControl::from_transparency(transparency);
        let illusion_strength = 1.0 - transparency;

        Veil {
            spectrum_position,
            access_limitation: access_limitation.clamp(0.0, 1.0),
            current_access,
            transparency,
            access_control,
            illusion_strength,
        }
    }

    /// Phase 4: Create Veil based on density
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th)"
    ///
    /// Transparency mapping:
    /// - 1st/2nd Density: 0.0 (Veil not yet active)
    /// - 3rd Density: 0.0 (Veil fully active, complete illusion)
    /// - 4th Density: 0.2 (Veil starts to thin)
    /// - 5th Density: 0.5 (Veil mostly dissolved)
    /// - 6th Density: 0.8 (Veil mostly gone)
    /// - 7th Density: 1.0 (Veil fully dissolved)
    /// - 8th Density: 1.0 (Return to IntelligentInfinity)
    pub fn from_density(density: &Density) -> Self {
        let transparency = match density {
            Density::First(_) => 0.0,
            Density::Second(_) => 0.0,
            Density::Third => 0.0,
            Density::Fourth => 0.2,
            Density::Fifth => 0.5,
            Density::Sixth => 0.8,
            Density::Seventh => 1.0,
            Density::Eighth => 1.0,
        };

        let access_limitation = 1.0 - transparency;
        let spectrum_position = SpectrumRatio::space_time(1.0, 1.0);
        let current_access = SpectrumAccess::limited(transparency, 1.0);
        let access_control = VeilAccessControl::from_transparency(transparency);
        let illusion_strength = 1.0 - transparency;

        Veil {
            spectrum_position,
            access_limitation,
            current_access,
            transparency,
            access_control,
            illusion_strength,
        }
    }

    /// Creates a Veil with full limitation (entity fully veiled)
    pub fn full_veil() -> Self {
        Veil::new(1.0)
    }

    /// Creates a Veil with no limitation (entity unobstructed)
    pub fn no_veil() -> Self {
        Veil::new(0.0)
    }

    /// Creates a Veil with partial limitation
    pub fn partial_veil(limitation: f64) -> Self {
        Veil::new(limitation)
    }

    /// Limits access to the Oneness side of the spectrum
    pub fn limit_access(&self, spectrum_access: &mut SpectrumAccess) {
        spectrum_access.oneness_access *= 1.0 - self.access_limitation;
    }

    /// Checks if a spectrum ratio is at the Veil transition
    pub fn is_at_transition(&self, ratio: &SpectrumRatio) -> bool {
        ratio.is_veil_transition()
    }

    /// Gets the veil strength
    pub fn strength(&self) -> f64 {
        self.access_limitation
    }

    /// Thins the veil (reduces limitation)
    pub fn thin(&mut self, amount: f64) {
        self.access_limitation = (self.access_limitation - amount).max(0.0);
        self.transparency = (self.transparency + amount).min(1.0);
        self.current_access.oneness_access = 1.0 - self.access_limitation;
        self.access_control = VeilAccessControl::from_transparency(self.transparency);
        self.illusion_strength = 1.0 - self.transparency;
    }

    /// Thickens the veil (increases limitation)
    pub fn thicken(&mut self, amount: f64) {
        self.access_limitation = (self.access_limitation + amount).min(1.0);
        self.transparency = (self.transparency - amount).max(0.0);
        self.current_access.oneness_access = 1.0 - self.access_limitation;
        self.access_control = VeilAccessControl::from_transparency(self.transparency);
        self.illusion_strength = 1.0 - self.transparency;
    }

    /// Phase 4: Check if entity can access Time/Space at required level
    pub fn can_access_time_space(&self, required_access: f64) -> bool {
        self.access_control.can_access_time_space(required_access)
    }

    /// Phase 4: Check if entity can form holographic connections at required level
    pub fn can_access_holographic_connections(&self, required_access: f64) -> bool {
        self.access_control
            .can_access_holographic_connections(required_access)
    }

    /// Phase 4: Check if entity can access higher consciousness at required level
    pub fn can_access_higher_consciousness(&self, required_access: f64) -> bool {
        self.access_control
            .can_access_higher_consciousness(required_access)
    }

    /// Checks if the veil creates the illusion of separation
    pub fn creates_illusion_of_separation(&self) -> bool {
        self.access_limitation > 0.5
    }

    /// Gets the quality of experience through the veil
    pub fn experience_quality(&self) -> VeilExperience {
        if self.access_limitation == 0.0 {
            VeilExperience::Unobstructed
        } else if self.access_limitation < 0.3 {
            VeilExperience::ThinVeil
        } else if self.access_limitation < 0.7 {
            VeilExperience::ModerateVeil
        } else if self.access_limitation < 1.0 {
            VeilExperience::ThickVeil
        } else {
            VeilExperience::FullVeil
        }
    }
}

/// Represents the quality of experience through the Veil
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VeilExperience {
    /// No veil - full spectrum access
    Unobstructed,
    /// Thin veil - mostly unobstructed
    ThinVeil,
    /// Moderate veil - balanced separation
    ModerateVeil,
    /// Thick veil - strong illusion of separation
    ThickVeil,
    /// Full veil - complete illusion of separation
    FullVeil,
}

impl VeilExperience {
    /// Gets the veil strength for this experience
    pub fn strength(&self) -> f64 {
        match self {
            VeilExperience::Unobstructed => 0.0,
            VeilExperience::ThinVeil => 0.15,
            VeilExperience::ModerateVeil => 0.5,
            VeilExperience::ThickVeil => 0.85,
            VeilExperience::FullVeil => 1.0,
        }
    }

    /// Gets the density level associated with this experience
    pub fn density_level(&self) -> &str {
        match self {
            VeilExperience::Unobstructed => "6th Density+ (Veil dissolved)",
            VeilExperience::ThinVeil => "5th Density (Veil mostly dissolved)",
            VeilExperience::ModerateVeil => "4th Density (Veil thinning)",
            VeilExperience::ThickVeil => "3rd Density (Veil active)",
            VeilExperience::FullVeil => "3rd Density (Veil fully active)",
        }
    }
}

/// Represents the illusion of separation created by the Veil
#[derive(Debug, Clone)]
pub struct IllusionOfSeparation {
    /// The strength of the illusion (0.0 to 1.0)
    pub strength: f64,
    /// The perceived separation from unity
    pub perceived_separation: f64,
    /// The contrast available for growth
    pub contrast: f64,
}

impl IllusionOfSeparation {
    /// Creates a new illusion of separation
    pub fn new(veil_strength: f64) -> Self {
        let strength = veil_strength;
        let perceived_separation = strength;
        let contrast = strength * 0.8; // Contrast is slightly less than full strength

        IllusionOfSeparation {
            strength,
            perceived_separation,
            contrast,
        }
    }

    /// Checks if the illusion is strong enough to create meaningful choice
    pub fn enables_meaningful_choice(&self) -> bool {
        self.strength > 0.3
    }

    /// Gets the depth of experience available
    pub fn depth_of_experience(&self) -> f64 {
        self.contrast
    }

    /// Gets the growth potential
    pub fn growth_potential(&self) -> f64 {
        self.contrast * 1.2 // Growth potential is slightly higher than contrast
    }
}

/// Represents the evolutionary mechanism of the Veil
#[derive(Debug, Clone)]
pub struct VeilEvolution {
    /// The current veil strength
    pub current_strength: f64,
    /// The target veil strength (for evolution)
    pub target_strength: f64,
    /// The evolution rate
    pub evolution_rate: f64,
}

impl VeilEvolution {
    /// Creates a new veil evolution mechanism
    pub fn new(current_strength: f64, target_strength: f64, evolution_rate: f64) -> Self {
        VeilEvolution {
            current_strength: current_strength.clamp(0.0, 1.0),
            target_strength: target_strength.clamp(0.0, 1.0),
            evolution_rate: evolution_rate.clamp(0.0, 1.0),
        }
    }

    /// Advances the evolution by one step
    pub fn advance(&mut self) -> bool {
        if self.current_strength == self.target_strength {
            return false; // Already at target
        }

        let difference = self.target_strength - self.current_strength;
        let step = difference * self.evolution_rate;

        // Apply step but don't overshoot
        if step.abs() < (self.target_strength - self.current_strength).abs() {
            self.current_strength += step;
        } else {
            self.current_strength = self.target_strength;
        }

        true // Evolution occurred
    }

    /// Checks if evolution is complete
    pub fn is_complete(&self) -> bool {
        (self.current_strength - self.target_strength).abs() < 0.01
    }

    /// Gets the evolution progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.current_strength == self.target_strength {
            return 1.0;
        }

        let initial_strength = 1.0; // Starting point
        let total_change = (self.target_strength - initial_strength).abs();
        let current_change = (self.current_strength - initial_strength).abs();

        if total_change == 0.0 {
            return 1.0;
        }

        current_change / total_change
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};
    use crate::spectrum::larson_framework::SpectrumQuality;

    #[test]
    fn test_spectrum_access_full() {
        let access = SpectrumAccess::full();
        assert_eq!(access.oneness_access, 1.0);
        assert_eq!(access.many_ness_access, 1.0);
        assert!(!access.is_veiled());
        assert_eq!(access.veil_strength(), 0.0);
        assert!(access.unobstructed());
    }

    #[test]
    fn test_spectrum_access_limited() {
        let access = SpectrumAccess::limited(0.5, 1.0);
        assert_eq!(access.oneness_access, 0.5);
        assert_eq!(access.many_ness_access, 1.0);
        assert!(access.is_veiled());
        assert_eq!(access.veil_strength(), 0.5);
    }

    #[test]
    fn test_spectrum_access_space_time_only() {
        let access = SpectrumAccess::space_time_only();
        assert_eq!(access.oneness_access, 0.0);
        assert_eq!(access.many_ness_access, 1.0);
        assert!(access.is_veiled());
        assert!(access.fully_veiled());
    }

    #[test]
    fn test_spectrum_access_time_space_only() {
        let access = SpectrumAccess::time_space_only();
        assert_eq!(access.oneness_access, 1.0);
        assert_eq!(access.many_ness_access, 0.0);
        assert!(!access.is_veiled());
    }

    #[test]
    fn test_spectrum_access_clamping() {
        let access = SpectrumAccess::limited(1.5, -0.5);
        assert_eq!(access.oneness_access, 1.0);
        assert_eq!(access.many_ness_access, 0.0);
    }

    #[test]
    fn test_veil_new() {
        let veil = Veil::new(0.5);
        assert_eq!(veil.access_limitation, 0.5);
        assert_eq!(veil.current_access.oneness_access, 0.5);
        assert_eq!(veil.current_access.many_ness_access, 1.0);
    }

    #[test]
    fn test_veil_full() {
        let veil = Veil::full_veil();
        assert_eq!(veil.access_limitation, 1.0);
        assert_eq!(veil.current_access.oneness_access, 0.0);
        assert!(veil.creates_illusion_of_separation());
    }

    #[test]
    fn test_veil_no_veil() {
        let veil = Veil::no_veil();
        assert_eq!(veil.access_limitation, 0.0);
        assert_eq!(veil.current_access.oneness_access, 1.0);
        assert!(!veil.creates_illusion_of_separation());
    }

    #[test]
    fn test_veil_limit_access() {
        let veil = Veil::new(0.5);
        let mut access = SpectrumAccess::full();

        veil.limit_access(&mut access);

        assert_eq!(access.oneness_access, 0.5);
        assert_eq!(access.many_ness_access, 1.0);
    }

    #[test]
    fn test_veil_is_at_transition() {
        let veil = Veil::new(0.5);
        let ratio = SpectrumRatio::space_time(1.0, 1.0);

        assert!(veil.is_at_transition(&ratio));
    }

    #[test]
    fn test_veil_thin() {
        let mut veil = Veil::new(0.8);
        veil.thin(0.3);

        assert!((veil.access_limitation - 0.5).abs() < 0.01);
        assert!((veil.current_access.oneness_access - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_veil_thicken() {
        let mut veil = Veil::new(0.3);
        veil.thicken(0.4);

        assert!((veil.access_limitation - 0.7).abs() < 0.01);
        assert!((veil.current_access.oneness_access - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_veil_thin_clamping() {
        let mut veil = Veil::new(0.2);
        veil.thin(0.5);

        assert_eq!(veil.access_limitation, 0.0);
    }

    #[test]
    fn test_veil_thicken_clamping() {
        let mut veil = Veil::new(0.8);
        veil.thicken(0.5);

        assert_eq!(veil.access_limitation, 1.0);
    }

    #[test]
    fn test_veil_experience_quality() {
        let veil = Veil::new(0.5);
        assert_eq!(veil.experience_quality(), VeilExperience::ModerateVeil);

        let veil = Veil::new(0.0);
        assert_eq!(veil.experience_quality(), VeilExperience::Unobstructed);

        let veil = Veil::new(1.0);
        assert_eq!(veil.experience_quality(), VeilExperience::FullVeil);
    }

    #[test]
    fn test_veil_experience_strength() {
        assert_eq!(VeilExperience::Unobstructed.strength(), 0.0);
        assert_eq!(VeilExperience::ThinVeil.strength(), 0.15);
        assert_eq!(VeilExperience::ModerateVeil.strength(), 0.5);
        assert_eq!(VeilExperience::ThickVeil.strength(), 0.85);
        assert_eq!(VeilExperience::FullVeil.strength(), 1.0);
    }

    #[test]
    fn test_illusion_of_separation() {
        let illusion = IllusionOfSeparation::new(0.5);

        assert_eq!(illusion.strength, 0.5);
        assert_eq!(illusion.perceived_separation, 0.5);
        assert_eq!(illusion.contrast, 0.4);
        assert!(illusion.enables_meaningful_choice());
    }

    #[test]
    fn test_illusion_weak() {
        let illusion = IllusionOfSeparation::new(0.2);

        assert!(!illusion.enables_meaningful_choice());
    }

    #[test]
    fn test_illusion_growth_potential() {
        let illusion = IllusionOfSeparation::new(0.5);

        assert_eq!(illusion.depth_of_experience(), 0.4);
        assert_eq!(illusion.growth_potential(), 0.48);
    }

    #[test]
    fn test_veil_evolution() {
        let evolution = VeilEvolution::new(1.0, 0.0, 0.1);

        assert_eq!(evolution.current_strength, 1.0);
        assert_eq!(evolution.target_strength, 0.0);
        assert!(!evolution.is_complete());
    }

    #[test]
    fn test_veil_evolution_advance() {
        let mut evolution = VeilEvolution::new(1.0, 0.0, 0.1);

        evolution.advance();

        assert_eq!(evolution.current_strength, 0.9);
    }

    #[test]
    fn test_veil_evolution_complete() {
        let mut evolution = VeilEvolution::new(1.0, 0.0, 0.5);

        for _ in 0..50 {
            evolution.advance();
        }

        assert!(evolution.is_complete());
        assert!((evolution.current_strength - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_veil_evolution_progress() {
        let evolution = VeilEvolution::new(1.0, 0.0, 0.1);

        let progress = evolution.progress();

        assert_eq!(progress, 0.0);
    }

    #[test]
    fn test_veil_evolution_progress_halfway() {
        let mut evolution = VeilEvolution::new(1.0, 0.0, 0.1);

        let mut progress = evolution.progress();
        assert!((progress - 0.0).abs() < 0.01);

        // Advance halfway
        for _ in 0..5 {
            evolution.advance();
            progress = evolution.progress();
        }

        // Progress should be around 0.5
        assert!((progress - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_veil_is_structural_feature() {
        // The Veil is a structural feature, not a separator
        let veil = Veil::new(0.5);

        // The entire spectrum exists simultaneously
        let ratio_oneness = SpectrumRatio::time_space(1.0, 3.0);
        let ratio_mannyness = SpectrumRatio::space_time(3.0, 1.0);

        // Both sides exist
        assert_eq!(ratio_oneness.quality(), SpectrumQuality::Oneness);
        assert_eq!(ratio_mannyness.quality(), SpectrumQuality::ManyNess);

        // The Veil only limits access
        let mut access = SpectrumAccess::full();
        veil.limit_access(&mut access);

        // Access is limited, but the spectrum still exists
        assert!(access.oneness_access < 1.0);
        assert_eq!(access.many_ness_access, 1.0);
    }

    // Phase 4: Veil Enhancement Tests

    #[test]
    fn test_veil_access_control_from_transparency() {
        let access_control = VeilAccessControl::from_transparency(0.5);

        assert_eq!(access_control.time_space_access, 0.5);
        assert_eq!(access_control.holographic_connection_access, 0.4);
        assert_eq!(access_control.higher_consciousness_access, 0.3);
    }

    #[test]
    fn test_veil_access_control_full_transparency() {
        let access_control = VeilAccessControl::from_transparency(1.0);

        assert_eq!(access_control.time_space_access, 1.0);
        assert_eq!(access_control.holographic_connection_access, 0.8);
        assert_eq!(access_control.higher_consciousness_access, 0.6);
    }

    #[test]
    fn test_veil_access_control_no_transparency() {
        let access_control = VeilAccessControl::from_transparency(0.0);

        assert_eq!(access_control.time_space_access, 0.0);
        assert_eq!(access_control.holographic_connection_access, 0.0);
        assert_eq!(access_control.higher_consciousness_access, 0.0);
    }

    #[test]
    fn test_veil_from_density_first() {
        let veil = Veil::from_density(&Density::First(Density1SubLevel::Quantum));

        assert_eq!(veil.transparency, 0.0);
        assert_eq!(veil.access_limitation, 1.0);
        assert_eq!(veil.illusion_strength, 1.0);
        assert_eq!(veil.access_control.time_space_access, 0.0);
    }

    #[test]
    fn test_veil_from_density_second() {
        let veil = Veil::from_density(&Density::Second(Density2SubLevel::Cellular));

        assert_eq!(veil.transparency, 0.0);
        assert_eq!(veil.access_limitation, 1.0);
        assert_eq!(veil.illusion_strength, 1.0);
    }

    #[test]
    fn test_veil_from_density_third() {
        let veil = Veil::from_density(&Density::Third);

        assert_eq!(veil.transparency, 0.0);
        assert_eq!(veil.access_limitation, 1.0);
        assert_eq!(veil.illusion_strength, 1.0);
        assert_eq!(veil.access_control.time_space_access, 0.0);
    }

    #[test]
    fn test_veil_from_density_fourth() {
        let veil = Veil::from_density(&Density::Fourth);

        assert!((veil.transparency - 0.2).abs() < 0.01);
        assert!((veil.access_limitation - 0.8).abs() < 0.01);
        assert!((veil.illusion_strength - 0.8).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.2).abs() < 0.01);
        assert!((veil.access_control.holographic_connection_access - 0.16).abs() < 0.01);
        assert!((veil.access_control.higher_consciousness_access - 0.12).abs() < 0.01);
    }

    #[test]
    fn test_veil_from_density_fifth() {
        let veil = Veil::from_density(&Density::Fifth);

        assert!((veil.transparency - 0.5).abs() < 0.01);
        assert!((veil.access_limitation - 0.5).abs() < 0.01);
        assert!((veil.illusion_strength - 0.5).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.5).abs() < 0.01);
        assert!((veil.access_control.holographic_connection_access - 0.4).abs() < 0.01);
        assert!((veil.access_control.higher_consciousness_access - 0.3).abs() < 0.01);
    }

    #[test]
    fn test_veil_from_density_sixth() {
        let veil = Veil::from_density(&Density::Sixth);

        assert!((veil.transparency - 0.8).abs() < 0.01);
        assert!((veil.access_limitation - 0.2).abs() < 0.01);
        assert!((veil.illusion_strength - 0.2).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.8).abs() < 0.01);
        assert!((veil.access_control.holographic_connection_access - 0.64).abs() < 0.01);
        assert!((veil.access_control.higher_consciousness_access - 0.48).abs() < 0.01);
    }

    #[test]
    fn test_veil_from_density_seventh() {
        let veil = Veil::from_density(&Density::Seventh);

        assert_eq!(veil.transparency, 1.0);
        assert_eq!(veil.access_limitation, 0.0);
        assert_eq!(veil.illusion_strength, 0.0);
        assert_eq!(veil.access_control.time_space_access, 1.0);
        assert_eq!(veil.access_control.holographic_connection_access, 0.8);
        assert_eq!(veil.access_control.higher_consciousness_access, 0.6);
    }

    #[test]
    fn test_veil_from_density_eighth() {
        let veil = Veil::from_density(&Density::Eighth);

        assert_eq!(veil.transparency, 1.0);
        assert_eq!(veil.access_limitation, 0.0);
        assert_eq!(veil.illusion_strength, 0.0);
        assert_eq!(veil.access_control.time_space_access, 1.0);
    }

    #[test]
    fn test_veil_can_access_time_space() {
        let veil = Veil::from_density(&Density::Fourth);

        // 4th density has 0.2 transparency, so can't access 0.3
        assert!(!veil.can_access_time_space(0.3));

        // But can access 0.1
        assert!(veil.can_access_time_space(0.1));
    }

    #[test]
    fn test_veil_can_access_holographic_connections() {
        let veil = Veil::from_density(&Density::Fourth);

        // 4th density has 0.16 holographic access, so can't access 0.3
        assert!(!veil.can_access_holographic_connections(0.3));

        // But can access 0.1
        assert!(veil.can_access_holographic_connections(0.1));
    }

    #[test]
    fn test_veil_can_access_higher_consciousness() {
        let veil = Veil::from_density(&Density::Fourth);

        // 4th density has 0.12 higher consciousness access, so can't access 0.3
        assert!(!veil.can_access_higher_consciousness(0.3));

        // But can access 0.05
        assert!(veil.can_access_higher_consciousness(0.05));
    }

    #[test]
    fn test_veil_thin_updates_access_control() {
        let mut veil = Veil::from_density(&Density::Third);
        assert!((veil.transparency - 0.0).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.0).abs() < 0.01);

        veil.thin(0.2);

        assert!((veil.transparency - 0.2).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.2).abs() < 0.01);
        assert!((veil.access_control.holographic_connection_access - 0.16).abs() < 0.01);
        assert!((veil.access_control.higher_consciousness_access - 0.12).abs() < 0.01);
        assert!((veil.illusion_strength - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_veil_thicken_updates_access_control() {
        let mut veil = Veil::from_density(&Density::Fifth);
        assert!((veil.transparency - 0.5).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.5).abs() < 0.01);

        veil.thicken(0.3);

        assert!((veil.transparency - 0.2).abs() < 0.01);
        assert!((veil.access_control.time_space_access - 0.2).abs() < 0.01);
        assert!((veil.access_control.holographic_connection_access - 0.16).abs() < 0.01);
        assert!((veil.access_control.higher_consciousness_access - 0.12).abs() < 0.01);
        assert!((veil.illusion_strength - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_veil_transparency_progression() {
        let first = Veil::from_density(&Density::First(Density1SubLevel::Quantum));
        let third = Veil::from_density(&Density::Third);
        let fourth = Veil::from_density(&Density::Fourth);
        let fifth = Veil::from_density(&Density::Fifth);
        let sixth = Veil::from_density(&Density::Sixth);
        let seventh = Veil::from_density(&Density::Seventh);

        assert_eq!(first.transparency, 0.0);
        assert_eq!(third.transparency, 0.0);
        assert_eq!(fourth.transparency, 0.2);
        assert_eq!(fifth.transparency, 0.5);
        assert_eq!(sixth.transparency, 0.8);
        assert_eq!(seventh.transparency, 1.0);

        // Transparency should increase monotonically
        assert!(fourth.transparency > third.transparency);
        assert!(fifth.transparency > fourth.transparency);
        assert!(sixth.transparency > fifth.transparency);
        assert!(seventh.transparency > sixth.transparency);
    }
}
