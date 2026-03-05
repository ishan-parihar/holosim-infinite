//! Universal Template Pattern
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Every element in the simulation follows the EXACT SAME template."
//!
//! This module implements the UniversalTemplate<T> pattern that:
//! 1. Implements holographic logic ONCE (applies to ALL component types)
//! 2. Uses shared references for common fields (Arc<HolographicField>)
//! 3. Instantiates with different parameters for different components
//! 4. Provides 100x+ memory savings through reference-based "include"
//!
//! ## The Template Structure
//!
//! Every component (entities, particles, worlds, stars, galaxies) has:
//! - Shared reference to holographic field (Arc<HolographicField>)
//! - Spectrum configuration (Space/Time ↔ Time/Space ratio)
//! - Archetype activation profile (22 coefficients)
//! - Density (1st → 8th)
//! - Free Will seed (8 bytes for non-deterministic choice)
//! - Component-specific data (T)

use super::field_address::HolographicAddress;
use crate::evolution_density_octave::density_octave::Density;
use crate::holographic::holographic_field::HolographicField;
use crate::spectrum::larson_framework::SpectrumRatio;
use std::collections::HashMap;
use std::hash::Hasher;
use std::sync::Arc;

/// Configuration for instantiating a UniversalTemplate
#[derive(Debug, Clone)]
pub struct TemplateConfig {
    /// Spectrum configuration (Space/Time ↔ Time/Space ratio)
    pub spectrum: SpectrumConfiguration,

    /// Archetype activation profile (22 coefficients)
    pub archetype_activation: ArchetypeActivationProfile,

    /// Density (1st → 8th)
    pub density: Density,

    /// Free Will seed (8 bytes for non-deterministic choice)
    pub free_will_seed: u64,
}

impl TemplateConfig {
    /// Create a new template configuration
    pub fn new(
        spectrum: SpectrumConfiguration,
        archetype_activation: ArchetypeActivationProfile,
        density: Density,
        free_will_seed: u64,
    ) -> Self {
        Self {
            spectrum,
            archetype_activation,
            density,
            free_will_seed,
        }
    }

    /// Create an initial configuration
    pub fn initial() -> Self {
        Self {
            spectrum: SpectrumConfiguration::balanced(),
            archetype_activation: ArchetypeActivationProfile::initial(),
            density: Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            free_will_seed: 0,
        }
    }
}

/// Spectrum configuration wrapper
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// The Space/Time and Time/Space spectrum is a continuous range of reciprocal ratios
/// (v = s/t to v = t/s) with a qualitative break at v = 1 (the Veil).
#[derive(Debug, Clone)]
pub struct SpectrumConfiguration {
    /// Spectrum ratio (v = s/t or v = t/s)
    pub ratio: SpectrumRatio,

    /// Veil transparency (0.0 = fully opaque, 1.0 = fully transparent)
    pub veil_transparency: f64,

    /// Space/Time access level (0.0 to 1.0)
    pub space_time_access: f64,

    /// Time/Space access level (0.0 to 1.0)
    pub time_space_access: f64,
}

impl SpectrumConfiguration {
    /// Create a new spectrum configuration
    pub fn new(
        ratio: SpectrumRatio,
        veil_transparency: f64,
        space_time_access: f64,
        time_space_access: f64,
    ) -> Self {
        Self {
            ratio,
            veil_transparency: veil_transparency.clamp(0.0, 1.0),
            space_time_access: space_time_access.clamp(0.0, 1.0),
            time_space_access: time_space_access.clamp(0.0, 1.0),
        }
    }

    /// Create a balanced spectrum configuration (at the Veil)
    pub fn balanced() -> Self {
        Self {
            ratio: SpectrumRatio::space_time(1.0, 1.0),
            veil_transparency: 0.5,
            space_time_access: 0.5,
            time_space_access: 0.5,
        }
    }

    /// Create a space/time dominant configuration (3rd Density)
    pub fn space_time_dominant() -> Self {
        Self {
            ratio: SpectrumRatio::space_time(20.0, 1.0),
            veil_transparency: 0.1,
            space_time_access: 0.95,
            time_space_access: 0.05,
        }
    }

    /// Create a time/space dominant configuration (6th Density+)
    pub fn time_space_dominant() -> Self {
        Self {
            ratio: SpectrumRatio::time_space(1.0, 20.0),
            veil_transparency: 0.9,
            space_time_access: 0.05,
            time_space_access: 0.95,
        }
    }

    /// Evolve the spectrum by a delta
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Evolution is not about moving from Space/Time to Time/Space—it's about
    ///  accessing more of the spectrum"
    pub fn evolve(&mut self, delta: f64) {
        // Adjust veil transparency
        self.veil_transparency = (self.veil_transparency + delta).clamp(0.0, 1.0);

        // Recalculate access levels based on veil transparency
        // Higher veil transparency = more time/space access, less space/time access
        let veil_influence = self.veil_transparency;
        self.space_time_access = 1.0 - veil_influence;
        self.time_space_access = veil_influence;
    }
}

/// Archetype activation profile
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// The 22 archetypes form the archetypical mind system (7+7+7+1).
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Store 22 archetype coefficients instead of full patterns"
/// "100x compression (88 bytes vs thousands of floats)"
#[derive(Debug, Clone, Default)]
pub struct ArchetypeActivationProfile {
    /// Activation coefficients for all 22 archetypes (0.0 to 1.0)
    ///
    /// Archetypes 1-7: Mind complex
    /// Archetypes 8-14: Body complex
    /// Archetypes 15-21: Spirit complex
    /// Archetype 22: The Choice
    pub coefficients: [f64; 22],
}

impl ArchetypeActivationProfile {
    /// Create a new archetype activation profile
    pub fn new(coefficients: [f64; 22]) -> Self {
        Self {
            coefficients: coefficients.map(|c| c.clamp(0.0, 1.0)),
        }
    }

    /// Create an initial activation profile (all archetypes at baseline)
    pub fn initial() -> Self {
        Self {
            coefficients: [0.5; 22],
        }
    }

    /// Get the activation level for a specific archetype
    pub fn get_activation(&self, archetype_number: usize) -> Option<f64> {
        if (1..=22).contains(&archetype_number) {
            Some(self.coefficients[archetype_number - 1])
        } else {
            None
        }
    }

    /// Set the activation level for a specific archetype
    pub fn set_activation(&mut self, archetype_number: usize, activation: f64) {
        if (1..=22).contains(&archetype_number) {
            self.coefficients[archetype_number - 1] = activation.clamp(0.0, 1.0);
        }
    }

    /// Get all activations for the Mind complex (Archetypes 1-7)
    pub fn mind_complex(&self) -> [f64; 7] {
        [
            self.coefficients[0],
            self.coefficients[1],
            self.coefficients[2],
            self.coefficients[3],
            self.coefficients[4],
            self.coefficients[5],
            self.coefficients[6],
        ]
    }

    /// Get all activations for the Body complex (Archetypes 8-14)
    pub fn body_complex(&self) -> [f64; 7] {
        [
            self.coefficients[7],
            self.coefficients[8],
            self.coefficients[9],
            self.coefficients[10],
            self.coefficients[11],
            self.coefficients[12],
            self.coefficients[13],
        ]
    }

    /// Get all activations for the Spirit complex (Archetypes 15-21)
    pub fn spirit_complex(&self) -> [f64; 7] {
        [
            self.coefficients[14],
            self.coefficients[15],
            self.coefficients[16],
            self.coefficients[17],
            self.coefficients[18],
            self.coefficients[19],
            self.coefficients[20],
        ]
    }

    /// Get activation for The Choice (Archetype 22)
    pub fn choice(&self) -> f64 {
        self.coefficients[21]
    }

    /// Calculate total archetype coherence
    ///
    /// Higher coherence = more unity (Law of One)
    pub fn coherence(&self) -> f64 {
        let mean: f64 = self.coefficients.iter().sum::<f64>() / 22.0;
        let variance: f64 = self
            .coefficients
            .iter()
            .map(|c| (c - mean).powi(2))
            .sum::<f64>()
            / 22.0;
        1.0 - variance.sqrt()
    }
}

/// Archetypical interference result
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Behavior emerges from archetype interference, not behavior trees"
#[derive(Debug, Clone)]
pub struct ArchetypicalInterference {
    /// Constructive interference nodes (stable configurations)
    pub constructive_nodes: Vec<f64>,

    /// Destructive interference nodes (unstable configurations)
    pub destructive_nodes: Vec<f64>,

    /// Overall interference pattern
    pub pattern: f64,

    /// Coherence of the interference (0.0 to 1.0)
    pub coherence: f64,
}

/// Archetype interference pattern (R&D-8)
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Compute archetype interference pattern"
#[derive(Debug, Clone, Default)]
pub struct ArchetypeInterference {
    /// Interference coefficients for all 22 archetypes
    pub coefficients: [f64; 22],
}

impl ArchetypicalInterference {
    /// Create a new archetypical interference result
    pub fn new(
        constructive_nodes: Vec<f64>,
        destructive_nodes: Vec<f64>,
        pattern: f64,
        coherence: f64,
    ) -> Self {
        Self {
            constructive_nodes,
            destructive_nodes,
            pattern,
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    /// Create an initial interference result
    pub fn initial() -> Self {
        Self {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern: 0.5,
            coherence: 0.5,
        }
    }
}

/// Possibility space for Free Will choice
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Free Will is the First Distortion - the Creator's choice to know Itself"
#[derive(Debug, Clone)]
pub struct PossibilitySpace {
    /// Available possibilities
    pub possibilities: Vec<Possibility>,

    /// Probability weights for each possibility
    pub weights: Vec<f64>,
}

impl PossibilitySpace {
    /// Create a new possibility space
    pub fn new(possibilities: Vec<Possibility>, weights: Vec<f64>) -> Self {
        assert_eq!(possibilities.len(), weights.len());
        Self {
            possibilities,
            weights,
        }
    }

    /// Create an initial possibility space
    pub fn initial() -> Self {
        Self {
            possibilities: vec![Possibility::initial()],
            weights: vec![1.0],
        }
    }

    /// Normalize weights to sum to 1.0
    pub fn normalize_weights(&mut self) {
        let sum: f64 = self.weights.iter().sum();
        if sum > 0.0 {
            for weight in &mut self.weights {
                *weight /= sum;
            }
        }
    }
}

/// A possibility in the possibility space
#[derive(Debug, Clone)]
pub struct Possibility {
    /// Possibility identifier
    pub id: String,

    /// Description
    pub description: String,

    /// Probability (before Free Will choice)
    pub probability: f64,

    /// Archetypical signature
    pub archetype_signature: [f64; 22],
}

impl Possibility {
    /// Create a new possibility
    pub fn new(
        id: String,
        description: String,
        probability: f64,
        archetype_signature: [f64; 22],
    ) -> Self {
        Self {
            id,
            description,
            probability: probability.clamp(0.0, 1.0),
            archetype_signature,
        }
    }

    /// Create an initial possibility
    pub fn initial() -> Self {
        Self {
            id: "default".to_string(),
            description: "Default possibility".to_string(),
            probability: 1.0,
            archetype_signature: [0.5; 22],
        }
    }
}

/// A Free Will choice
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Free Will as seed (8 bytes vs 100+ bytes)"
#[derive(Debug, Clone)]
pub struct Choice {
    /// Selected possibility
    pub selected_possibility: Possibility,

    /// Choice timestamp
    pub timestamp: f64,

    /// Archetypical resonance with the choice
    pub resonance: f64,
}

impl Choice {
    /// Create a new choice
    pub fn new(selected_possibility: Possibility, timestamp: f64, resonance: f64) -> Self {
        Self {
            selected_possibility,
            timestamp,
            resonance: resonance.clamp(0.0, 1.0),
        }
    }

    /// Create an initial choice
    pub fn initial() -> Self {
        Self {
            selected_possibility: Possibility::initial(),
            timestamp: 0.0,
            resonance: 0.5,
        }
    }
}

/// Actualized state after holographic collapse
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Collapse from possibility to actuality"
#[derive(Debug, Clone)]
pub struct ActualizedState {
    /// Actualized configuration
    pub configuration: Vec<f64>,

    /// Collapse coherence (how aligned with archetype profile)
    pub coherence: f64,

    /// Resonance with holographic field
    pub resonance: f64,
}

impl ActualizedState {
    /// Create a new actualized state
    pub fn new(configuration: Vec<f64>, coherence: f64, resonance: f64) -> Self {
        Self {
            configuration,
            coherence: coherence.clamp(0.0, 1.0),
            resonance: resonance.clamp(0.0, 1.0),
        }
    }

    /// Create an initial actualized state
    pub fn initial() -> Self {
        Self {
            configuration: vec![0.5],
            coherence: 0.5,
            resonance: 0.5,
        }
    }
}

/// Key for caching template instances
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateKey {
    /// Density level
    pub density: u8,

    /// Veil transparency (quantized to 10 levels)
    pub veil_level: u8,

    /// Archetype coherence (quantized to 10 levels)
    pub coherence_level: u8,
}

impl TemplateKey {
    /// Create a new template key
    pub fn new(density: u8, veil_level: u8, coherence_level: u8) -> Self {
        Self {
            density,
            veil_level: veil_level.clamp(0, 9),
            coherence_level: coherence_level.clamp(0, 9),
        }
    }

    /// Compute a template key from configuration
    pub fn from_config(config: &TemplateConfig) -> Self {
        let density = match config.density {
            Density::First(_) => 1,
            Density::Second(_) => 2,
            Density::Third => 3,
            Density::Fourth => 4,
            Density::Fifth => 5,
            Density::Sixth => 6,
            Density::Seventh => 7,
            Density::Eighth => 8,
        };

        // Use 9.0 to get range 0-9 (10 levels)
        let veil_level = ((config.spectrum.veil_transparency * 9.0) as u8).min(9);
        let coherence_level = ((config.archetype_activation.coherence() * 9.0) as u8).min(9);

        Self::new(density, veil_level, coherence_level)
    }
}

/// Universal Template for all simulation components
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Every element in the simulation follows the EXACT SAME template"
///
/// # Fields
///
/// - `field`: Shared reference to holographic field (Arc<HolographicField>)
/// - `spectrum`: Spectrum configuration (Space/Time ↔ Time/Space ratio)
/// - `archetype_activation`: Archetype activation profile (22 coefficients)
/// - `density`: Density (1st → 8th)
/// - `free_will_seed`: Free Will seed (8 bytes for non-deterministic choice)
/// - `component_data`: Component-specific data (T)
///
/// # Key Insight
///
/// The first 5 fields are EXACTLY THE SAME for all components.
/// Only `component_data` varies.
#[derive(Debug, Clone)]
pub struct UniversalTemplate<T> {
    /// SHARED across all instances (reference)
    pub field: Arc<HolographicField>,

    /// Holographic address in the field (replaces coordinate concept)
    /// From Phase 1: Space emerges from field coherence, position is an address
    pub address: HolographicAddress,

    /// UNIQUE per instance (parameter values)
    pub spectrum: SpectrumConfiguration,

    /// UNIQUE per instance (parameter values)
    pub archetype_activation: ArchetypeActivationProfile,

    /// UNIQUE per instance (parameter values)
    pub density: Density,

    /// UNIQUE per instance (parameter values)
    pub free_will_seed: u64,

    /// COMPONENT-SPECIFIC data
    pub component_data: T,
}

impl<T> UniversalTemplate<T> {
    /// Create a new universal template
    pub fn new(
        field: Arc<HolographicField>,
        spectrum: SpectrumConfiguration,
        archetype_activation: ArchetypeActivationProfile,
        density: Density,
        free_will_seed: u64,
        component_data: T,
    ) -> Self {
        Self {
            field,
            address: HolographicAddress::cosmic_origin(),
            spectrum,
            archetype_activation,
            density,
            free_will_seed,
            component_data,
        }
    }

    /// Create a new template from configuration
    pub fn from_config(
        field: Arc<HolographicField>,
        config: TemplateConfig,
        component_data: T,
    ) -> Self {
        Self {
            field,
            address: HolographicAddress::cosmic_origin(),
            spectrum: config.spectrum,
            archetype_activation: config.archetype_activation,
            density: config.density,
            free_will_seed: config.free_will_seed,
            component_data,
        }
    }

    /// Evolve the spectrum by a delta
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Implement holographic logic ONCE on template"
    /// "evolve_spectrum() (applies to all component types)"
    pub fn evolve_spectrum(&mut self, delta: f64) {
        self.spectrum.evolve(delta);
    }

    /// Process archetypes and return interference pattern
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "process_archetypes() (applies to all component types)"
    pub fn process_archetypes(&self) -> ArchetypicalInterference {
        let coherence = self.archetype_activation.coherence();

        // Simplified interference calculation
        let pattern: f64 = self
            .archetype_activation
            .coefficients
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let phase = (i as f64) * std::f64::consts::PI / 11.0;
                c * phase.cos()
            })
            .sum::<f64>()
            .abs();

        ArchetypicalInterference {
            constructive_nodes: vec![],
            destructive_nodes: vec![],
            pattern,
            coherence,
        }
    }

    /// Exercise Free Will and make a choice
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "exercise_free_will() (applies to all component types)"
    pub fn exercise_free_will(&self, possibility_space: &PossibilitySpace) -> Choice {
        if possibility_space.possibilities.is_empty() {
            return Choice::initial();
        }

        // Use Free Will seed to make deterministic non-deterministic choice
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&self.free_will_seed, &mut hasher);
        let hash = hasher.finish();

        let mut cumulative_weight = 0.0;
        let target = (hash as f64) / (u64::MAX as f64);

        for (i, weight) in possibility_space.weights.iter().enumerate() {
            cumulative_weight += weight;
            if cumulative_weight >= target {
                let selected = possibility_space.possibilities[i].clone();

                // Calculate resonance with archetype profile
                let resonance =
                    archetype_resonance(&self.archetype_activation, &selected.archetype_signature);

                return Choice::new(selected, 0.0, resonance);
            }
        }

        // Fallback to first possibility
        let selected = possibility_space.possibilities[0].clone();
        let resonance =
            archetype_resonance(&self.archetype_activation, &selected.archetype_signature);
        Choice::new(selected, 0.0, resonance)
    }

    /// Collapse a possibility to an actualized state
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "collapse_possibility() (applies to all component types)"
    pub fn collapse_possibility(&self, possibility: &Possibility) -> ActualizedState {
        let coherence =
            archetype_resonance(&self.archetype_activation, &possibility.archetype_signature);

        let resonance = self.field.phase_coherence().clamp(0.0, 1.0);

        let configuration = possibility.archetype_signature.to_vec();

        ActualizedState::new(configuration, coherence, resonance)
    }

    /// Evolve using holographic logic (R&D-8)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "All components use SAME evolve logic - implemented once"
    /// "Implement holographic logic ONCE on template"
    ///
    /// This method implements the core holographic evolution logic that applies
    /// to ALL component types (entities, particles, worlds, stars, galaxies).
    ///
    /// Evolution steps:
    /// 1. Spectrum evolution (Space/Time ↔ Time/Space)
    /// 2. Field evolution with MERA optimization (O(log n) queries)
    /// 3. Archetype interference pattern computation
    /// 4. Free will choice from possibility space
    /// 5. Component-specific data evolution
    ///
    /// # Arguments
    ///
    /// * `dt` - Time delta for evolution step
    pub fn evolve(&mut self, dt: f64) {
        // Spectrum evolution
        self.evolve_spectrum(dt);

        // Field evolution with MERA optimization
        // In full implementation, would use MeraField::query()
        // for O(log n) decompression instead of O(n)

        // Archetype processing
        let interference = self.compute_archetype_interference();

        // Free will choice
        let choice = self.exercise_free_will(&PossibilitySpace::initial());

        // Component-specific evolution (just data, not logic)
        self.evolve_component(interference, choice, dt);
    }

    /// Compute archetype interference pattern (R&D-8)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Compute archetype interference pattern"
    ///
    /// # Returns
    ///
    /// Archetype interference pattern
    fn compute_archetype_interference(&self) -> ArchetypeInterference {
        // Simplified: archetype activations modulate field
        // In full implementation, would compute actual interference pattern
        ArchetypeInterference {
            coefficients: self.archetype_activation.coefficients,
        }
    }

    /// Evolve component-specific data (R&D-8)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Component-specific evolution (just data, not logic)"
    ///
    /// This is the only part that varies by component type.
    /// The holographic evolution logic is shared.
    ///
    /// # Arguments
    ///
    /// * `interference` - Archetype interference pattern
    /// * `choice` - Free will choice
    /// * `dt` - Time delta
    fn evolve_component(&mut self, interference: ArchetypeInterference, choice: Choice, dt: f64) {
        // Component-specific logic goes here
        // This is the only part that varies by component type
        // The holographic evolution logic (spectrum, field, archetypes, free will) is shared

        // Simplified: apply interference and choice to component data
        // In full implementation, component-specific evolution would be here
        let _ = (interference, choice, dt); // Placeholder for component-specific logic
    }

    /// Get a reference to the component data
    pub fn component_data(&self) -> &T {
        &self.component_data
    }

    /// Get a mutable reference to the component data
    pub fn component_data_mut(&mut self) -> &mut T {
        &mut self.component_data
    }

    /// Get the holographic address
    pub fn address(&self) -> &HolographicAddress {
        &self.address
    }

    /// Set the holographic address
    pub fn set_address(&mut self, address: HolographicAddress) {
        self.address = address;
    }
}

/// Calculate resonance between archetype activation and signature
fn archetype_resonance(activation: &ArchetypeActivationProfile, signature: &[f64; 22]) -> f64 {
    let mut sum = 0.0;
    for (a, s) in activation.coefficients.iter().zip(signature.iter()) {
        sum += (a - s).abs();
    }
    1.0 - (sum / 22.0)
}

/// Trait for creating component data from configuration
pub trait FromConfig {
    /// Create component data from template configuration
    fn from_config(config: &TemplateConfig) -> Self;
}

/// Template factory for instantiating universal templates
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Create TemplateFactory for instantiation"
#[derive(Debug, Clone)]
pub struct TemplateFactory {
    /// Base holographic field (shared across all instances)
    pub base_field: Arc<HolographicField>,

    /// Cache of template instances
    pub cache: HashMap<TemplateKey, Arc<dyn std::any::Any + Send + Sync>>,
}

impl TemplateFactory {
    /// Create a new template factory
    pub fn new(field: Arc<HolographicField>) -> Self {
        Self {
            base_field: field,
            cache: HashMap::new(),
        }
    }

    /// Instantiate a universal template
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "instantiate<T>(&mut self, config: TemplateConfig) -> UniversalTemplate<T>"
    pub fn instantiate<T>(&mut self, config: TemplateConfig) -> UniversalTemplate<T>
    where
        T: FromConfig + Clone + Send + Sync + 'static,
    {
        let key = TemplateKey::from_config(&config);

        // Check cache first (for component data)
        let component_data = if let Some(cached) = self.cache.get(&key) {
            cached.downcast_ref::<T>().cloned()
        } else {
            let data = T::from_config(&config);
            self.cache.insert(key, Arc::new(data.clone()));
            Some(data)
        };

        let component_data = component_data.unwrap_or_else(|| T::from_config(&config));

        UniversalTemplate::from_config(self.base_field.clone(), config, component_data)
    }

    /// Clear the template cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic::complex_vectors::ComplexArchetype;

    fn create_test_holographic_field() -> HolographicField {
        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for i in 0..22 {
                a[i] = ComplexArchetype {
                    amplitude: (i as f64 + 1.0) / 22.0,
                    phase: (i as f64) * std::f64::consts::PI / 11.0,
                };
            }
            a
        };
        HolographicField::new(crate::holographic::InvolutionLayer::Green, archetypes)
    }

    #[test]
    fn test_spectrum_configuration_creation() {
        let spectrum =
            SpectrumConfiguration::new(SpectrumRatio::space_time(1.0, 1.0), 0.5, 0.5, 0.5);

        assert_eq!(spectrum.ratio.calculate_ratio(), 1.0);
        assert_eq!(spectrum.veil_transparency, 0.5);
        assert_eq!(spectrum.space_time_access, 0.5);
        assert_eq!(spectrum.time_space_access, 0.5);
    }

    #[test]
    fn test_spectrum_configuration_balanced() {
        let spectrum = SpectrumConfiguration::balanced();
        assert_eq!(spectrum.ratio.calculate_ratio(), 1.0);
    }

    #[test]
    fn test_spectrum_configuration_space_time_dominant() {
        let spectrum = SpectrumConfiguration::space_time_dominant();
        assert_eq!(spectrum.ratio.calculate_ratio(), 20.0);
        assert_eq!(spectrum.veil_transparency, 0.1);
    }

    #[test]
    fn test_spectrum_configuration_time_space_dominant() {
        let spectrum = SpectrumConfiguration::time_space_dominant();
        assert_eq!(spectrum.ratio.calculate_ratio(), 20.0);
        assert_eq!(spectrum.veil_transparency, 0.9);
    }

    #[test]
    fn test_spectrum_evolve() {
        let mut spectrum = SpectrumConfiguration::balanced();
        spectrum.evolve(0.1);

        assert_eq!(spectrum.veil_transparency, 0.6);
        assert!(spectrum.space_time_access < 0.5);
        assert!(spectrum.time_space_access > 0.5);
    }

    #[test]
    fn test_archetype_activation_profile_creation() {
        let coefficients = [0.5; 22];
        let profile = ArchetypeActivationProfile::new(coefficients);

        assert_eq!(profile.coefficients.len(), 22);
    }

    #[test]
    fn test_archetype_activation_profile_default() {
        let profile = ArchetypeActivationProfile::initial();
        assert_eq!(profile.coefficients.len(), 22);
        assert!(profile.coefficients.iter().all(|c| *c == 0.5));
    }

    #[test]
    fn test_archetype_activation_get_set() {
        let mut profile = ArchetypeActivationProfile::initial();

        profile.set_activation(1, 0.8);
        assert_eq!(profile.get_activation(1), Some(0.8));
    }

    #[test]
    fn test_archetype_activation_invalid() {
        let profile = ArchetypeActivationProfile::initial();
        assert_eq!(profile.get_activation(0), None);
        assert_eq!(profile.get_activation(23), None);
    }

    #[test]
    fn test_archetype_complexes() {
        let mut profile = ArchetypeActivationProfile::initial();

        for i in 1..=22 {
            profile.set_activation(i, (i as f64) / 22.0);
        }

        let mind = profile.mind_complex();
        assert_eq!(mind.len(), 7);

        let body = profile.body_complex();
        assert_eq!(body.len(), 7);

        let spirit = profile.spirit_complex();
        assert_eq!(spirit.len(), 7);

        let choice = profile.choice();
        assert_eq!(choice, 1.0);
    }

    #[test]
    fn test_archetype_coherence() {
        let profile = ArchetypeActivationProfile::initial();
        let coherence = profile.coherence();
        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
        assert_eq!(coherence, 1.0);
    }

    #[test]
    fn test_possibility_space_creation() {
        let possibilities = vec![Possibility::initial()];
        let weights = vec![1.0];

        let space = PossibilitySpace::new(possibilities, weights);
        assert_eq!(space.possibilities.len(), 1);
        assert_eq!(space.weights.len(), 1);
    }

    #[test]
    fn test_possibility_space_normalize() {
        let possibilities = vec![
            Possibility::initial(),
            Possibility::initial(),
            Possibility::initial(),
        ];
        let weights = vec![1.0, 1.0, 1.0];

        let mut space = PossibilitySpace::new(possibilities, weights);
        space.normalize_weights();

        assert_eq!(space.weights[0], 1.0 / 3.0);
        assert_eq!(space.weights[1], 1.0 / 3.0);
        assert_eq!(space.weights[2], 1.0 / 3.0);
    }

    #[test]
    fn test_universal_template_creation() {
        let field = Arc::new(create_test_holographic_field());
        let spectrum = SpectrumConfiguration::balanced();
        let archetype_activation = ArchetypeActivationProfile::initial();
        let density = Density::First(
            crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
        );
        let free_will_seed = 42;
        let component_data = "test_data".to_string();

        let template = UniversalTemplate::new(
            field,
            spectrum,
            archetype_activation,
            density,
            free_will_seed,
            component_data,
        );

        assert_eq!(template.free_will_seed, 42);
        assert_eq!(template.component_data, "test_data");
    }

    #[test]
    fn test_universal_template_from_config() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let template =
            UniversalTemplate::from_config(field.clone(), config, "test_data".to_string());
        assert_eq!(template.free_will_seed, 0);
        assert_eq!(template.component_data, "test_data");
    }

    #[test]
    fn test_universal_template_evolve_spectrum() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let mut template = UniversalTemplate::from_config(field, config, "test_data".to_string());
        template.evolve_spectrum(0.1);

        assert_eq!(template.spectrum.veil_transparency, 0.6);
    }

    #[test]
    fn test_universal_template_process_archetypes() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let template = UniversalTemplate::from_config(field, config, "test_data".to_string());
        let interference = template.process_archetypes();

        assert!(interference.coherence >= 0.0);
        assert!(interference.coherence <= 1.0);
    }

    #[test]
    fn test_universal_template_exercise_free_will() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let template = UniversalTemplate::from_config(field, config, "test_data".to_string());

        let possibility = Possibility::initial();
        let space = PossibilitySpace::new(vec![possibility], vec![1.0]);

        let choice = template.exercise_free_will(&space);
        assert_eq!(choice.selected_possibility.id, "default");
    }

    #[test]
    fn test_universal_template_collapse_possibility() {
        let field = Arc::new(create_test_holographic_field());
        let config = TemplateConfig::initial();

        let template = UniversalTemplate::from_config(field, config, "test_data".to_string());

        let possibility = Possibility::initial();
        let state = template.collapse_possibility(&possibility);

        assert!(state.coherence >= 0.0);
        assert!(state.coherence <= 1.0);
    }

    #[test]
    fn test_template_factory_creation() {
        let field = Arc::new(create_test_holographic_field());
        let factory = TemplateFactory::new(field);

        assert_eq!(factory.cache_size(), 0);
    }

    #[test]
    fn test_template_key_from_config() {
        let config = TemplateConfig::initial();
        let key = TemplateKey::from_config(&config);

        assert_eq!(key.density, 1);
        // Default spectrum is balanced with veil_transparency = 0.5
        // Using 9.0 multiplier: 0.5 * 9 = 4.5 -> 4
        assert_eq!(key.veil_level, 4);
        // Default archetype activation is uniform (0.5), coherence = 1.0
        // Using 9.0 multiplier: 1.0 * 9 = 9
        assert_eq!(key.coherence_level, 9);
    }

    #[test]
    fn test_archetype_resonance() {
        let activation = ArchetypeActivationProfile::default();
        let signature = [0.5; 22];

        let resonance = archetype_resonance(&activation, &signature);
        assert_eq!(resonance, 1.0);
    }

    #[test]
    fn test_archetype_resonance_different() {
        let mut activation = ArchetypeActivationProfile::default();
        activation.set_activation(1, 1.0);
        let signature = [0.0; 22];

        let resonance = archetype_resonance(&activation, &signature);
        assert!(resonance < 1.0);
        assert!(resonance >= 0.0);
    }
}
