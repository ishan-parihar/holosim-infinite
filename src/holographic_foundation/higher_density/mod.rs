//! Phase 14: Higher Density Mechanics (5th-8th Density)
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Higher densities are DISTINCT FIELD CONFIGURATIONS, not completion states."
//!
//! # Density Theory
//!
//! Each higher density represents a qualitatively different field configuration:
//!
//! **5th Density (Blue Ray) - Wisdom**:
//! - Light-dominant field (maximized information transfer)
//! - Light body manifestation
//! - Teaching/learning through field resonance coupling
//!
//! **6th Density (Indigo Ray) - Unity**:
//! - Unity-dominant field (individual/collective distinction dissolves)
//! - STO/STS polarity balances
//! - Gateway access to Intelligent Infinity
//!
//! **7th Density (Violet Ray) - Completion**:
//! - Field returns toward SOURCE configuration
//! - All experiences integrated coherently
//! - Gateway to next octave
//!
//! **8th Density - Return**:
//! - Merger with Intelligent Infinity field
//! - Unique pattern preserved in unified field
//! - Preparation for next octave seeding
//!
//! # Key Insight
//!
//! Higher densities are NOT "more evolved" states but DISTINCT FIELD CONFIGURATIONS
//! with different physical laws, perception modes, and manifestation capabilities.

pub mod gateway_mechanics;
pub mod light_body;
pub mod octave_transition;
pub mod source_merger;
pub mod unity_consciousness;

pub use gateway_mechanics::{
    GatewayAccess, GatewayMechanics, GatewayResonance, GatewayState, IntelligentInfinityAccess,
    IntelligentInfinityPattern,
};
pub use light_body::{
    EnergyBodyField, LightBody, LightBodyManifestation, LightBodyState, PhotonField,
};
pub use octave_transition::{
    ActivatedSeed, CompletionStage, OctaveBridge, OctaveTransition, OctaveTransitionState,
    PatternSeed, SourcePreparation,
};
pub use source_merger::{
    IntelligentInfinityMerger, MergerProgress, PatternPreservation, SourceConnection,
    SourceMergerState,
};
pub use unity_consciousness::{
    CollectiveFieldMerge, PolarityBalance, SocialMemoryComplex, UnityConsciousness,
    UnityConsciousnessState,
};

/// Density Ray colors corresponding to each density level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DensityRay {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    White,
}

impl DensityRay {
    pub fn for_density(density: u8) -> Self {
        match density {
            1 => DensityRay::Red,
            2 => DensityRay::Orange,
            3 => DensityRay::Yellow,
            4 => DensityRay::Green,
            5 => DensityRay::Blue,
            6 => DensityRay::Indigo,
            7 => DensityRay::Violet,
            8 => DensityRay::White,
            _ => DensityRay::White,
        }
    }

    pub fn wavelength_nm(&self) -> Float {
        match self {
            DensityRay::Red => 700.0,
            DensityRay::Orange => 620.0,
            DensityRay::Yellow => 580.0,
            DensityRay::Green => 530.0,
            DensityRay::Blue => 470.0,
            DensityRay::Indigo => 445.0,
            DensityRay::Violet => 400.0,
            DensityRay::White => 0.0,
        }
    }

    pub fn frequency_thz(&self) -> Float {
        match self {
            DensityRay::Red => 430.0,
            DensityRay::Orange => 480.0,
            DensityRay::Yellow => 520.0,
            DensityRay::Green => 560.0,
            DensityRay::Blue => 640.0,
            DensityRay::Indigo => 670.0,
            DensityRay::Violet => 750.0,
            DensityRay::White => 0.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            DensityRay::Red => "1st Density - Basic awareness, minerals",
            DensityRay::Orange => "2nd Density - Growth, plants/animals",
            DensityRay::Yellow => "3rd Density - Self-awareness, choice",
            DensityRay::Green => "4th Density - Love/Light, compassion",
            DensityRay::Blue => "5th Density - Light/Light, wisdom",
            DensityRay::Indigo => "6th Density - Unity, gateway access",
            DensityRay::Violet => "7th Density - Completion, preparation",
            DensityRay::White => "8th Density - Octave, return to source",
        }
    }
}

use crate::types::Float;

/// Base field configuration for all higher densities
#[derive(Debug, Clone)]
pub struct HigherDensityField {
    pub density_level: u8,
    pub ray: DensityRay,
    pub field_coherence: Float,
    pub light_dominance: Float,
    pub unity_factor: Float,
    pub source_proximity: Float,
}

impl HigherDensityField {
    pub fn new(density_level: u8) -> Self {
        Self {
            density_level,
            ray: DensityRay::for_density(density_level),
            field_coherence: 0.5,
            light_dominance: Self::default_light_dominance(density_level),
            unity_factor: Self::default_unity_factor(density_level),
            source_proximity: Self::default_source_proximity(density_level),
        }
    }

    fn default_light_dominance(density: u8) -> Float {
        match density {
            1 | 2 | 3 => 0.0,
            4 => 0.3,
            5 => 0.8,
            6 => 0.9,
            7 => 0.95,
            8 => 1.0,
            _ => 0.0,
        }
    }

    fn default_unity_factor(density: u8) -> Float {
        match density {
            1 | 2 | 3 => 0.0,
            4 => 0.2,
            5 => 0.5,
            6 => 0.9,
            7 => 0.98,
            8 => 1.0,
            _ => 0.0,
        }
    }

    fn default_source_proximity(density: u8) -> Float {
        match density {
            1 | 2 | 3 => 0.0,
            4 => 0.1,
            5 => 0.3,
            6 => 0.5,
            7 => 0.8,
            8 => 1.0,
            _ => 0.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        let target_coherence = 0.5 + 0.05 * self.density_level as Float;
        self.field_coherence =
            self.field_coherence * 0.99 + target_coherence * 0.01 * dt.recip().min(1.0);

        if self.density_level >= 5 {
            self.light_dominance = (self.light_dominance + 0.001 * dt)
                .min(Self::default_light_dominance(self.density_level + 1));
        }

        if self.density_level >= 6 {
            self.unity_factor = (self.unity_factor + 0.001 * dt)
                .min(Self::default_unity_factor(self.density_level + 1));
        }

        if self.density_level >= 7 {
            self.source_proximity = (self.source_proximity + 0.001 * dt)
                .min(Self::default_source_proximity(self.density_level + 1));
        }
    }

    pub fn can_transition_to(&self, target_density: u8) -> bool {
        if target_density <= self.density_level {
            return false;
        }

        let coherence_threshold = 0.8 + 0.02 * target_density as Float;
        self.field_coherence >= coherence_threshold
            && self.light_dominance >= Self::default_light_dominance(target_density) * 0.9
            && self.unity_factor >= Self::default_unity_factor(target_density) * 0.9
    }

    pub fn transition_progress(&self, target_density: u8) -> Float {
        if target_density <= self.density_level {
            return 0.0;
        }

        let coherence_progress =
            self.field_coherence / (0.8 + 0.02 * target_density as Float).min(1.0);
        let light_progress = if self.density_level >= 5 {
            self.light_dominance / Self::default_light_dominance(target_density).max(0.01)
        } else {
            1.0
        };
        let unity_progress = if self.density_level >= 6 {
            self.unity_factor / Self::default_unity_factor(target_density).max(0.01)
        } else {
            1.0
        };

        (coherence_progress * 0.4 + light_progress * 0.3 + unity_progress * 0.3).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_ray_for_density() {
        assert_eq!(DensityRay::for_density(1), DensityRay::Red);
        assert_eq!(DensityRay::for_density(5), DensityRay::Blue);
        assert_eq!(DensityRay::for_density(8), DensityRay::White);
    }

    #[test]
    fn test_higher_density_field_creation() {
        let field = HigherDensityField::new(5);
        assert_eq!(field.density_level, 5);
        assert_eq!(field.ray, DensityRay::Blue);
    }

    #[test]
    fn test_light_dominance_increases() {
        let field_d5 = HigherDensityField::new(5);
        let field_d6 = HigherDensityField::new(6);
        assert!(field_d6.light_dominance > field_d5.light_dominance);
    }

    #[test]
    fn test_unity_factor_increases() {
        let field_d6 = HigherDensityField::new(6);
        let field_d7 = HigherDensityField::new(7);
        assert!(field_d7.unity_factor > field_d6.unity_factor);
    }

    #[test]
    fn test_source_proximity() {
        let field = HigherDensityField::new(8);
        assert!((field.source_proximity - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_field_update() {
        let mut field = HigherDensityField::new(5);
        field.update(1.0);
        assert!(field.light_dominance >= 0.8);
    }

    #[test]
    fn test_can_transition_to() {
        let mut field = HigherDensityField::new(5);
        field.field_coherence = 0.95;
        field.light_dominance = 0.85;
        field.unity_factor = 0.85;
        assert!(field.can_transition_to(6));
    }

    #[test]
    fn test_transition_progress() {
        let mut field = HigherDensityField::new(5);
        field.field_coherence = 0.9;
        field.light_dominance = 0.85;
        let progress = field.transition_progress(6);
        assert!(progress > 0.0 && progress <= 1.0);
    }
}
