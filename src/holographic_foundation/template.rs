//! Universal Template for Holographic Simulation
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Every element in the simulation follows the EXACT SAME template.
//!  The first 5 fields are EXACTLY THE SAME for all components.
//!  Only component_data varies."
//!
//! This enables:
//! - Implement holographic logic ONCE, apply to EVERYTHING
//! - 100x memory reduction through shared field reference
//! - Consistent behavior across Entity, Particle, World, Star, Galaxy

use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use super::archetype_profile::ArchetypeActivationProfile;
use super::scale_level::ScaleLevel;
use crate::types::Float;

#[derive(Debug, Clone)]
pub struct SpectrumConfiguration {
    pub space_time_ratio: Float,
    pub time_space_ratio: Float,
    pub spectrum_position: Float,
    pub veil_transparency: Float,
}

impl PartialEq for SpectrumConfiguration {
    fn eq(&self, other: &Self) -> bool {
        (self.space_time_ratio - other.space_time_ratio).abs() < 1e-10
            && (self.time_space_ratio - other.time_space_ratio).abs() < 1e-10
            && (self.spectrum_position - other.spectrum_position).abs() < 1e-10
            && (self.veil_transparency - other.veil_transparency).abs() < 1e-10
    }
}

impl Eq for SpectrumConfiguration {}

impl Hash for SpectrumConfiguration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.space_time_ratio.to_bits().hash(state);
        self.time_space_ratio.to_bits().hash(state);
        self.spectrum_position.to_bits().hash(state);
        self.veil_transparency.to_bits().hash(state);
    }
}

impl Default for SpectrumConfiguration {
    fn default() -> Self {
        Self {
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
            veil_transparency: 0.0,
        }
    }
}

impl SpectrumConfiguration {
    pub fn new(space_time_ratio: Float, time_space_ratio: Float) -> Self {
        let spectrum_position = space_time_ratio / (space_time_ratio + time_space_ratio);
        let veil_transparency = if spectrum_position > 0.5 {
            (spectrum_position - 0.5) * 2.0
        } else {
            0.0
        };
        Self {
            space_time_ratio,
            time_space_ratio,
            spectrum_position,
            veil_transparency,
        }
    }

    pub fn space_time_dominant() -> Self {
        Self::new(2.0, 0.5)
    }

    pub fn time_space_dominant() -> Self {
        Self::new(0.5, 2.0)
    }

    pub fn at_veil() -> Self {
        Self::new(1.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density {
    First(u8),
    Second(u8),
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Default for Density {
    fn default() -> Self {
        Density::Third
    }
}

impl Density {
    pub fn to_u8(&self) -> u8 {
        match self {
            Density::First(sub) => 10 + sub,
            Density::Second(sub) => 20 + sub,
            Density::Third => 30,
            Density::Fourth => 40,
            Density::Fifth => 50,
            Density::Sixth => 60,
            Density::Seventh => 70,
            Density::Eighth => 80,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            11..=14 => Some(Density::First(value - 10)),
            21..=23 => Some(Density::Second(value - 20)),
            30 => Some(Density::Third),
            40 => Some(Density::Fourth),
            50 => Some(Density::Fifth),
            60 => Some(Density::Sixth),
            70 => Some(Density::Seventh),
            80 => Some(Density::Eighth),
            _ => None,
        }
    }

    pub fn consciousness_level(&self) -> Float {
        match self {
            Density::First(_) => 0.05,
            Density::Second(_) => 0.15,
            Density::Third => 0.30,
            Density::Fourth => 0.60,
            Density::Fifth => 0.80,
            Density::Sixth => 0.95,
            Density::Seventh => 0.99,
            Density::Eighth => 1.00,
        }
    }

    pub fn veil_transparency(&self) -> Float {
        match self {
            Density::First(_) => 0.05,
            Density::Second(_) => 0.10,
            Density::Third => 0.10,
            Density::Fourth => 0.40,
            Density::Fifth => 0.70,
            Density::Sixth => 1.00,
            Density::Seventh => 1.00,
            Density::Eighth => 1.00,
        }
    }
}

pub struct UniversalTemplate<T> {
    pub field: Arc<HolographicFieldReference>,
    pub spectrum: SpectrumConfiguration,
    pub archetype_activation: ArchetypeActivationProfile,
    pub density: Density,
    pub free_will_seed: u64,
    pub component_data: T,
}

impl<T> UniversalTemplate<T> {
    pub fn new(
        field: Arc<HolographicFieldReference>,
        spectrum: SpectrumConfiguration,
        archetype_activation: ArchetypeActivationProfile,
        density: Density,
        free_will_seed: u64,
        component_data: T,
    ) -> Self {
        Self {
            field,
            spectrum,
            archetype_activation,
            density,
            free_will_seed,
            component_data,
        }
    }

    pub fn evolve_spectrum(&mut self, delta: Float) {
        let evolution_factor = 0.01 * delta;
        self.spectrum.spectrum_position =
            (self.spectrum.spectrum_position + evolution_factor).min(1.0);
        if self.spectrum.spectrum_position > 0.5 {
            self.spectrum.veil_transparency = (self.spectrum.spectrum_position - 0.5) * 2.0;
        }
    }

    pub fn process_archetypes(&self) -> ArchetypicalInterference {
        let mind = self.archetype_activation.mind_complex();
        let body = self.archetype_activation.body_complex();
        let spirit = self.archetype_activation.spirit_complex();
        let choice = self.archetype_activation.choice();

        let mind_weight: Float = mind.iter().sum();
        let body_weight: Float = body.iter().sum();
        let spirit_weight: Float = spirit.iter().sum();

        ArchetypicalInterference {
            mind_dominance: mind_weight / (mind_weight + body_weight + spirit_weight + 0.001),
            body_dominance: body_weight / (mind_weight + body_weight + spirit_weight + 0.001),
            spirit_dominance: spirit_weight / (mind_weight + body_weight + spirit_weight + 0.001),
            choice_activation: choice,
            coherence: self.archetype_activation.coherence(),
        }
    }

    pub fn exercise_free_will(&self, possibilities: &[Float]) -> usize {
        if possibilities.is_empty() {
            return 0;
        }

        let interference = self.process_archetypes();
        let choice_weight = interference.choice_activation;

        let mut best_idx = 0;
        let mut best_score = Float::NEG_INFINITY;

        for (idx, &possibility) in possibilities.iter().enumerate() {
            let seed_influence = ((self.free_will_seed.wrapping_add(idx as u64)) as Float).sin();
            let choice_influence = if choice_weight > 0.5 { 1.0 } else { -1.0 };
            let score = possibility + seed_influence * 0.1 + choice_influence * choice_weight.abs();

            if score > best_score {
                best_score = score;
                best_idx = idx;
            }
        }

        best_idx
    }

    pub fn scale_level(&self) -> ScaleLevel {
        match self.density {
            Density::First(_) => ScaleLevel::Quantum,
            Density::Second(_) => ScaleLevel::Cellular,
            Density::Third => ScaleLevel::Biological,
            Density::Fourth => ScaleLevel::Biological,
            Density::Fifth => ScaleLevel::Planetary,
            Density::Sixth => ScaleLevel::Stellar,
            Density::Seventh => ScaleLevel::Cosmic,
            Density::Eighth => ScaleLevel::Cosmic,
        }
    }
}

impl<T: Clone> Clone for UniversalTemplate<T> {
    fn clone(&self) -> Self {
        Self {
            field: self.field.clone(),
            spectrum: self.spectrum.clone(),
            archetype_activation: self.archetype_activation.clone(),
            density: self.density,
            free_will_seed: self.free_will_seed,
            component_data: self.component_data.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for UniversalTemplate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UniversalTemplate")
            .field("spectrum", &self.spectrum)
            .field("density", &self.density)
            .field("free_will_seed", &self.free_will_seed)
            .field("component_data", &self.component_data)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypicalInterference {
    pub mind_dominance: Float,
    pub body_dominance: Float,
    pub spirit_dominance: Float,
    pub choice_activation: Float,
    pub coherence: Float,
}

#[derive(Debug, Clone)]
pub struct HolographicFieldReference {
    pub field_id: u64,
    pub coherence: Float,
    pub scale_level: ScaleLevel,
}

impl HolographicFieldReference {
    pub fn new(field_id: u64, coherence: Float, scale_level: ScaleLevel) -> Self {
        Self {
            field_id,
            coherence,
            scale_level,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateConfig {
    pub spectrum: SpectrumConfiguration,
    pub archetype_activation: ArchetypeActivationProfile,
    pub density: Density,
    pub free_will_seed: u64,
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            spectrum: SpectrumConfiguration::default(),
            archetype_activation: ArchetypeActivationProfile::default(),
            density: Density::default(),
            free_will_seed: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateKey {
    pub density: Density,
    pub veil_level: u8,
    pub coherence_level: u8,
}

impl TemplateKey {
    pub fn new(density: Density, veil_transparency: Float, coherence: Float) -> Self {
        Self {
            density,
            veil_level: (veil_transparency * 10.0).min(10.0) as u8,
            coherence_level: (coherence * 10.0).min(10.0) as u8,
        }
    }
}

pub struct TemplateFactory {
    field: Arc<HolographicFieldReference>,
    cache: HashMap<TemplateKey, Arc<dyn Any + Send + Sync>>,
    cache_hits: u64,
    cache_misses: u64,
}

impl TemplateFactory {
    pub fn new(field: Arc<HolographicFieldReference>) -> Self {
        Self {
            field,
            cache: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    pub fn instantiate<T: Clone + 'static>(
        &mut self,
        config: &TemplateConfig,
        component_data: T,
    ) -> UniversalTemplate<T> {
        let key = TemplateKey::new(
            config.density,
            config.spectrum.veil_transparency,
            config.archetype_activation.coherence(),
        );

        UniversalTemplate::new(
            self.field.clone(),
            config.spectrum.clone(),
            config.archetype_activation.clone(),
            config.density,
            config.free_will_seed,
            component_data,
        )
    }

    pub fn cache_hit_rate(&self) -> Float {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as Float / total as Float
        }
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentTypeId {
    Particle,
    Atom,
    Molecule,
    Cell,
    Entity,
    World,
    Star,
    Galaxy,
}

impl ComponentTypeId {
    pub fn default_scale(&self) -> ScaleLevel {
        match self {
            ComponentTypeId::Particle => ScaleLevel::Quantum,
            ComponentTypeId::Atom => ScaleLevel::Atomic,
            ComponentTypeId::Molecule => ScaleLevel::Molecular,
            ComponentTypeId::Cell => ScaleLevel::Cellular,
            ComponentTypeId::Entity => ScaleLevel::Biological,
            ComponentTypeId::World => ScaleLevel::Planetary,
            ComponentTypeId::Star => ScaleLevel::Stellar,
            ComponentTypeId::Galaxy => ScaleLevel::Cosmic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_configuration() {
        let st = SpectrumConfiguration::space_time_dominant();
        assert!(st.spectrum_position > 0.5);
        assert!(st.veil_transparency > 0.0);

        let ts = SpectrumConfiguration::time_space_dominant();
        assert!(ts.spectrum_position < 0.5);
    }

    #[test]
    fn test_density_consciousness() {
        assert!(Density::Third.consciousness_level() < Density::Fourth.consciousness_level());
        assert!(Density::Sixth.veil_transparency() > Density::Third.veil_transparency());
    }

    #[test]
    fn test_template_creation() {
        let field = Arc::new(HolographicFieldReference::new(
            1,
            0.5,
            ScaleLevel::Biological,
        ));
        let config = TemplateConfig::default();
        let template: UniversalTemplate<()> = UniversalTemplate::new(
            field,
            config.spectrum,
            config.archetype_activation,
            config.density,
            config.free_will_seed,
            (),
        );

        assert_eq!(template.scale_level(), ScaleLevel::Biological);
    }

    #[test]
    fn test_archetype_processing() {
        let field = Arc::new(HolographicFieldReference::new(
            1,
            0.5,
            ScaleLevel::Biological,
        ));
        let mut profile = ArchetypeActivationProfile::default();
        profile.set_activation(22, 1.0);

        let template: UniversalTemplate<()> = UniversalTemplate::new(
            field,
            SpectrumConfiguration::default(),
            profile,
            Density::Third,
            42,
            (),
        );

        let interference = template.process_archetypes();
        assert_eq!(interference.choice_activation, 1.0);
    }

    #[test]
    fn test_free_will_exercise() {
        let field = Arc::new(HolographicFieldReference::new(
            1,
            0.5,
            ScaleLevel::Biological,
        ));
        let template: UniversalTemplate<()> = UniversalTemplate::new(
            field,
            SpectrumConfiguration::default(),
            ArchetypeActivationProfile::default(),
            Density::Third,
            12345,
            (),
        );

        let possibilities = vec![0.3, 0.5, 0.7, 0.2];
        let choice = template.exercise_free_will(&possibilities);
        assert!(choice < possibilities.len());
    }

    #[test]
    fn test_template_factory() {
        let field = Arc::new(HolographicFieldReference::new(
            1,
            0.5,
            ScaleLevel::Biological,
        ));
        let mut factory = TemplateFactory::new(field);

        let config = TemplateConfig::default();
        let template: UniversalTemplate<f64> = factory.instantiate(&config, 42.0);

        assert_eq!(template.component_data, 42.0);
    }

    #[test]
    fn test_component_type_scales() {
        assert_eq!(
            ComponentTypeId::Particle.default_scale(),
            ScaleLevel::Quantum
        );
        assert_eq!(ComponentTypeId::Atom.default_scale(), ScaleLevel::Atomic);
        assert_eq!(ComponentTypeId::Galaxy.default_scale(), ScaleLevel::Cosmic);
    }
}
