//! Veil Creates "External" Environment Illusion
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "The Veil creates the ILLUSION of separation from environment"
//!
//! # Key Insight
//!
//! The Veil is not a barrier but a perceptual filter that creates the
//! experience of "external" environment. Beneath the Veil, all is unified
//! field - the separation is illusion. Breaking Veil transparency reveals
//! the underlying unity of entity and environment.

use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VeilPerceptionLevel {
    Opaque,
    Thin,
    Permeable,
    Transparent,
    Shattered,
}

impl VeilPerceptionLevel {
    pub fn separation_factor(&self) -> Float {
        match self {
            VeilPerceptionLevel::Opaque => 1.0,
            VeilPerceptionLevel::Thin => 0.7,
            VeilPerceptionLevel::Permeable => 0.4,
            VeilPerceptionLevel::Transparent => 0.1,
            VeilPerceptionLevel::Shattered => 0.0,
        }
    }

    pub fn unity_perception(&self) -> Float {
        1.0 - self.separation_factor()
    }

    pub fn field_merge_rate(&self) -> Float {
        match self {
            VeilPerceptionLevel::Opaque => 0.0,
            VeilPerceptionLevel::Thin => 0.1,
            VeilPerceptionLevel::Permeable => 0.3,
            VeilPerceptionLevel::Transparent => 0.6,
            VeilPerceptionLevel::Shattered => 1.0,
        }
    }

    pub fn consciousness_expansion(&self) -> Float {
        match self {
            VeilPerceptionLevel::Opaque => 1.0,
            VeilPerceptionLevel::Thin => 1.2,
            VeilPerceptionLevel::Permeable => 1.5,
            VeilPerceptionLevel::Transparent => 2.0,
            VeilPerceptionLevel::Shattered => 3.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            VeilPerceptionLevel::Opaque => {
                "Full separation - environment appears completely external"
            }
            VeilPerceptionLevel::Thin => "Slight transparency - hints of underlying unity",
            VeilPerceptionLevel::Permeable => "Partial transparency - unity partially perceived",
            VeilPerceptionLevel::Transparent => "Mostly clear - unity visible through Veil",
            VeilPerceptionLevel::Shattered => "Veil broken - complete unity perception",
        }
    }

    pub fn next_level(&self) -> Option<Self> {
        match self {
            VeilPerceptionLevel::Opaque => Some(VeilPerceptionLevel::Thin),
            VeilPerceptionLevel::Thin => Some(VeilPerceptionLevel::Permeable),
            VeilPerceptionLevel::Permeable => Some(VeilPerceptionLevel::Transparent),
            VeilPerceptionLevel::Transparent => Some(VeilPerceptionLevel::Shattered),
            VeilPerceptionLevel::Shattered => None,
        }
    }

    pub fn previous_level(&self) -> Option<Self> {
        match self {
            VeilPerceptionLevel::Opaque => None,
            VeilPerceptionLevel::Thin => Some(VeilPerceptionLevel::Opaque),
            VeilPerceptionLevel::Permeable => Some(VeilPerceptionLevel::Thin),
            VeilPerceptionLevel::Transparent => Some(VeilPerceptionLevel::Permeable),
            VeilPerceptionLevel::Shattered => Some(VeilPerceptionLevel::Transparent),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnityPerception {
    pub entity_id: u64,
    pub veil_perception_level: VeilPerceptionLevel,
    pub perceived_separation: Float,
    pub underlying_unity: Float,
    pub environment_connection: Float,
    pub consciousness_expansion: Float,
    pub unity_insights: Vec<String>,
    pub peak_unity_experiences: Float,
}

impl UnityPerception {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            veil_perception_level: VeilPerceptionLevel::Opaque,
            perceived_separation: 1.0,
            underlying_unity: 0.0,
            environment_connection: 0.0,
            consciousness_expansion: 1.0,
            unity_insights: Vec::new(),
            peak_unity_experiences: 0.0,
        }
    }

    pub fn update(&mut self, _dt: Float, catalyst: Float) {
        let transparency = self.veil_perception_level;
        let target_separation = transparency.separation_factor();
        let target_unity = transparency.unity_perception();

        let catalyst_factor = 1.0 + catalyst * 0.1;
        self.perceived_separation =
            self.perceived_separation * 0.99 + target_separation * 0.01 * catalyst_factor.recip();
        self.underlying_unity =
            self.underlying_unity * 0.99 + target_unity * 0.01 * catalyst_factor;
        self.environment_connection = self.underlying_unity * self.consciousness_expansion;
        self.consciousness_expansion = transparency.consciousness_expansion();

        if self.underlying_unity > 0.5 && self.peak_unity_experiences < self.underlying_unity {
            self.peak_unity_experiences = self.underlying_unity;
            self.add_unity_insight();
        }
    }

    fn add_unity_insight(&mut self) {
        let insight = match self.veil_perception_level {
            VeilPerceptionLevel::Opaque => "Glimpsed the possibility of connection",
            VeilPerceptionLevel::Thin => "Felt the boundary between self and world waver",
            VeilPerceptionLevel::Permeable => "Touched the unity beneath appearance",
            VeilPerceptionLevel::Transparent => "Saw clearly: all is one field",
            VeilPerceptionLevel::Shattered => "BECAME the field - no separation remains",
        };
        self.unity_insights.push(insight.to_string());
    }

    pub fn thin_veil(&mut self, amount: Float) {
        while let Some(next) = self.veil_perception_level.next_level() {
            if amount >= self.veil_perception_level.separation_factor() - next.separation_factor() {
                self.veil_perception_level = next;
            } else {
                break;
            }
        }
    }

    pub fn thicken_veil(&mut self, amount: Float) {
        while let Some(prev) = self.veil_perception_level.previous_level() {
            if amount >= prev.separation_factor() - self.veil_perception_level.separation_factor() {
                self.veil_perception_level = prev;
            } else {
                break;
            }
        }
    }

    pub fn is_enlightened(&self) -> bool {
        self.veil_perception_level == VeilPerceptionLevel::Shattered
    }

    pub fn unity_ratio(&self) -> Float {
        if self.perceived_separation < 0.001 {
            return Float::INFINITY;
        }
        self.underlying_unity / self.perceived_separation
    }
}

#[derive(Debug, Clone)]
pub struct SeparationIllusion {
    pub entity_id: u64,
    pub illusion_strength: Float,
    pub illusion_components: HashMap<String, Float>,
    pub dissonance_points: Vec<String>,
    pub dissolution_progress: Float,
    pub reinforcement_sources: HashMap<String, Float>,
}

impl SeparationIllusion {
    pub fn new(entity_id: u64) -> Self {
        let mut illusion_components = HashMap::new();
        illusion_components.insert("spatial".to_string(), 1.0);
        illusion_components.insert("temporal".to_string(), 1.0);
        illusion_components.insert("identity".to_string(), 1.0);
        illusion_components.insert("ownership".to_string(), 1.0);
        illusion_components.insert("causation".to_string(), 1.0);

        Self {
            entity_id,
            illusion_strength: 1.0,
            illusion_components,
            dissonance_points: Vec::new(),
            dissolution_progress: 0.0,
            reinforcement_sources: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: Float, unity_perception: &UnityPerception) {
        let unity_factor = unity_perception.underlying_unity;

        for (_, strength) in self.illusion_components.iter_mut() {
            *strength = (*strength * (1.0 - unity_factor * 0.01 * dt)).max(0.0);
        }

        self.illusion_strength = self.illusion_components.values().sum::<Float>()
            / self.illusion_components.len() as Float;

        self.dissolution_progress = 1.0 - self.illusion_strength;

        if unity_factor > 0.3 {
            self.check_dissonance();
        }

        for (_, source) in self.reinforcement_sources.iter_mut() {
            *source = (*source * 0.995).max(0.0);
        }
    }

    fn check_dissonance(&mut self) {
        if self
            .illusion_components
            .get("spatial")
            .copied()
            .unwrap_or(0.0)
            < 0.5
            && !self
                .dissonance_points
                .contains(&"spatial_dissolution".to_string())
        {
            self.dissonance_points
                .push("spatial_dissolution".to_string());
        }

        if self
            .illusion_components
            .get("identity")
            .copied()
            .unwrap_or(0.0)
            < 0.5
            && !self
                .dissonance_points
                .contains(&"identity_crisis".to_string())
        {
            self.dissonance_points.push("identity_crisis".to_string());
        }
    }

    pub fn add_reinforcement(&mut self, source: &str, strength: Float) {
        *self
            .reinforcement_sources
            .entry(source.to_string())
            .or_insert(0.0) += strength;
    }

    pub fn apply_dissonance(&mut self, dissonance_type: &str) {
        match dissonance_type {
            "spatial_dissolution" => {
                *self.illusion_components.get_mut("spatial").unwrap() *= 0.5;
            }
            "identity_crisis" => {
                *self.illusion_components.get_mut("identity").unwrap() *= 0.5;
            }
            "causal_awareness" => {
                *self.illusion_components.get_mut("causation").unwrap() *= 0.5;
            }
            _ => {}
        }
    }

    pub fn is_dissolved(&self) -> bool {
        self.illusion_strength < 0.1
    }

    pub fn strongest_illusion(&self) -> Option<String> {
        self.illusion_components
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| k.clone())
    }
}

#[derive(Debug, Clone)]
pub struct VeilSeparationMechanics {
    pub entity_id: u64,
    pub unity_perception: UnityPerception,
    pub separation_illusion: SeparationIllusion,
    pub veil_thickness: Float,
    pub veil_stability: Float,
    pub natural_thinning_rate: Float,
    pub catalyst_sensitivity: Float,
    pub integration_capacity: Float,
}

impl VeilSeparationMechanics {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            unity_perception: UnityPerception::new(entity_id),
            separation_illusion: SeparationIllusion::new(entity_id),
            veil_thickness: 1.0,
            veil_stability: 1.0,
            natural_thinning_rate: 0.001,
            catalyst_sensitivity: 1.0,
            integration_capacity: 0.1,
        }
    }

    pub fn update(&mut self, dt: Float, catalyst: Float) {
        self.veil_thickness -= self.natural_thinning_rate * dt;
        self.veil_thickness = self.veil_thickness.max(0.0);

        let effective_catalyst = catalyst * self.catalyst_sensitivity;
        self.unity_perception.update(dt, effective_catalyst);
        self.separation_illusion.update(dt, &self.unity_perception);

        let expansion_rate = self.unity_perception.consciousness_expansion - 1.0;
        self.integration_capacity =
            (self.integration_capacity + expansion_rate * 0.01 * dt).min(1.0);

        if self.unity_perception.underlying_unity > self.integration_capacity {
            self.veil_stability *= 0.99;
        } else {
            self.veil_stability = (self.veil_stability + 0.001 * dt).min(1.0);
        }
    }

    pub fn apply_catalyst(&mut self, catalyst_type: &str, strength: Float) {
        let effective = strength * self.catalyst_sensitivity;

        match catalyst_type {
            "meditation" => {
                self.unity_perception.thin_veil(effective * 0.1);
            }
            "trauma" => {
                self.separation_illusion
                    .add_reinforcement("trauma", effective);
                self.veil_stability *= 1.0 - effective * 0.1;
            }
            "insight" => {
                self.unity_perception.thin_veil(effective * 0.2);
                self.separation_illusion
                    .apply_dissonance("spatial_dissolution");
            }
            "unity_experience" => {
                self.unity_perception.thin_veil(effective * 0.3);
                self.integration_capacity = (self.integration_capacity + effective * 0.1).min(1.0);
            }
            "density_transition" => {
                self.unity_perception.thin_veil(effective * 0.5);
            }
            _ => {}
        }
    }

    pub fn perceive_environment(&self) -> Float {
        1.0 - self.unity_perception.underlying_unity * self.separation_illusion.illusion_strength
    }

    pub fn is_veil_intact(&self) -> bool {
        self.veil_stability > 0.1 && self.veil_thickness > 0.0
    }

    pub fn unity_experience_available(&self) -> Float {
        self.unity_perception.underlying_unity * self.integration_capacity
    }

    pub fn field_merge_potential(&self) -> Float {
        self.unity_perception
            .veil_perception_level
            .field_merge_rate()
            * self.integration_capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veil_perception_level_separation() {
        assert!(
            VeilPerceptionLevel::Opaque.separation_factor()
                > VeilPerceptionLevel::Transparent.separation_factor()
        );
    }

    #[test]
    fn test_veil_perception_level_unity() {
        assert!(
            VeilPerceptionLevel::Shattered.unity_perception()
                > VeilPerceptionLevel::Opaque.unity_perception()
        );
    }

    #[test]
    fn test_veil_perception_level_field_merge() {
        assert_eq!(VeilPerceptionLevel::Opaque.field_merge_rate(), 0.0);
        assert_eq!(VeilPerceptionLevel::Shattered.field_merge_rate(), 1.0);
    }

    #[test]
    fn test_veil_perception_level_next_level() {
        assert_eq!(
            VeilPerceptionLevel::Opaque.next_level(),
            Some(VeilPerceptionLevel::Thin)
        );
        assert_eq!(VeilPerceptionLevel::Shattered.next_level(), None);
    }

    #[test]
    fn test_veil_perception_level_previous_level() {
        assert_eq!(
            VeilPerceptionLevel::Thin.previous_level(),
            Some(VeilPerceptionLevel::Opaque)
        );
        assert_eq!(VeilPerceptionLevel::Opaque.previous_level(), None);
    }

    #[test]
    fn test_unity_perception_creation() {
        let perception = UnityPerception::new(1);
        assert_eq!(perception.entity_id, 1);
        assert_eq!(
            perception.veil_perception_level,
            VeilPerceptionLevel::Opaque
        );
    }

    #[test]
    fn test_unity_perception_update() {
        let mut perception = UnityPerception::new(1);
        perception.update(1.0, 0.5);
        assert!(perception.perceived_separation >= 0.0);
    }

    #[test]
    fn test_unity_perception_thin_veil() {
        let mut perception = UnityPerception::new(1);
        perception.thin_veil(0.5);
        assert_ne!(
            perception.veil_perception_level,
            VeilPerceptionLevel::Opaque
        );
    }

    #[test]
    fn test_unity_perception_enlightened() {
        let mut perception = UnityPerception::new(1);
        assert!(!perception.is_enlightened());
        perception.veil_perception_level = VeilPerceptionLevel::Shattered;
        assert!(perception.is_enlightened());
    }

    #[test]
    fn test_separation_illusion_creation() {
        let illusion = SeparationIllusion::new(1);
        assert_eq!(illusion.entity_id, 1);
        assert!(!illusion.illusion_components.is_empty());
    }

    #[test]
    fn test_separation_illusion_update() {
        let mut illusion = SeparationIllusion::new(1);
        let unity = UnityPerception::new(1);
        illusion.update(1.0, &unity);
        assert!(illusion.illusion_strength >= 0.0);
    }

    #[test]
    fn test_separation_illusion_reinforcement() {
        let mut illusion = SeparationIllusion::new(1);
        illusion.add_reinforcement("test", 0.5);
        assert!(illusion.reinforcement_sources.contains_key("test"));
    }

    #[test]
    fn test_separation_illusion_dissolved() {
        let mut illusion = SeparationIllusion::new(1);
        for strength in illusion.illusion_components.values_mut() {
            *strength = 0.05;
        }
        illusion.illusion_strength = 0.05;
        assert!(illusion.is_dissolved());
    }

    #[test]
    fn test_veil_separation_mechanics_creation() {
        let mechanics = VeilSeparationMechanics::new(1);
        assert_eq!(mechanics.entity_id, 1);
        assert!(mechanics.veil_thickness > 0.0);
    }

    #[test]
    fn test_veil_separation_mechanics_update() {
        let mut mechanics = VeilSeparationMechanics::new(1);
        mechanics.update(1.0, 0.5);
        assert!(mechanics.veil_thickness < 1.0);
    }

    #[test]
    fn test_veil_separation_mechanics_catalyst() {
        let mut mechanics = VeilSeparationMechanics::new(1);
        mechanics.apply_catalyst("meditation", 5.0);
        assert_ne!(
            mechanics.unity_perception.veil_perception_level,
            VeilPerceptionLevel::Opaque
        );
    }

    #[test]
    fn test_veil_separation_mechanics_perceive() {
        let mechanics = VeilSeparationMechanics::new(1);
        let perception = mechanics.perceive_environment();
        assert!((0.0..=1.0).contains(&perception));
    }

    #[test]
    fn test_veil_separation_mechanics_intact() {
        let mechanics = VeilSeparationMechanics::new(1);
        assert!(mechanics.is_veil_intact());
    }

    #[test]
    fn test_separation_illusion_strongest() {
        let illusion = SeparationIllusion::new(1);
        let strongest = illusion.strongest_illusion();
        assert!(strongest.is_some());
    }
}
