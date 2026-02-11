//! Archetypical Interference Engine - True Emergence from Holographic Interference
//!
//! This module implements behavior emergence from archetype interference,
//! eliminating the need for behavior trees.
//!
//! From GAMING_ENGINE_ROADMAP_v2.md:
//! "NO BEHAVIOR TREES - behavior emerges from archetype interference"
//! "Free Will + Archetypes create emergence"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 6.3:
//! "Emergent Behavior (No Behavior Trees): Traditional: Pre-defined behavior tree.
//! Holographic: Emergent from archetype interference."
//!
//! This module provides:
//! - ArchetypicalInterferenceEngine: Creates interference patterns from archetype profiles
//! - InterferencePattern: Holographic interference from archetype combinations
//! - EmergentBehavior: Behavior that emerges (not pre-defined)
//! - ActionVector: Collapsed action from possibility space

use crate::simulation_v3::{
    archetype_basis::{ArchetypeActivationProfile, ArchetypeBasis, NUM_ARCHETYPES},
    free_will_seed::{FreeWillChoiceEngine, FreeWillSeed, Possibility, PossibilitySpace},
    observer_effect::{ObservationEngine, ObserverEffect},
};
use crate::types::Float;
use std::collections::HashMap;

/// Interference pattern created from archetype combinations
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Behavior emerges from archetype interference, not behavior trees."
#[derive(Debug, Clone, PartialEq)]
pub struct InterferencePattern {
    /// Pattern components (interference field)
    pub components: Vec<Float>,
    /// Dominant archetypes (top N by coefficient)
    pub dominant_archetypes: Vec<(usize, Float)>,
    /// Coherence of the pattern (0.0 = chaotic, 1.0 = coherent)
    pub coherence: Float,
    /// Entropy of the pattern (0.0 = ordered, max = chaotic)
    pub entropy: Float,
    /// Resonance frequency of the pattern
    pub resonance: Float,
}

impl InterferencePattern {
    /// Create a new interference pattern
    pub fn new(components: Vec<Float>) -> Self {
        let coherence = Self::compute_coherence(&components);
        let entropy = Self::compute_entropy(&components);
        let resonance = Self::compute_resonance(&components);

        InterferencePattern {
            components,
            dominant_archetypes: Vec::new(),
            coherence,
            entropy,
            resonance,
        }
    }

    /// Create a zero interference pattern
    pub fn zero(dimension: usize) -> Self {
        InterferencePattern {
            components: vec![0.0; dimension],
            dominant_archetypes: Vec::new(),
            coherence: 0.0,
            entropy: 0.0,
            resonance: 0.0,
        }
    }

    /// Compute coherence of pattern
    pub fn compute_coherence(components: &[Float]) -> Float {
        if components.is_empty() {
            return 0.0;
        }
        let mean: Float = components.iter().sum::<Float>() / components.len() as Float;
        let variance: Float = components.iter().map(|c| (c - mean).powi(2)).sum::<Float>()
            / components.len() as Float;
        let std_dev = variance.sqrt();
        if std_dev < 1e-10 {
            1.0
        } else {
            1.0 / (1.0 + std_dev)
        }
    }

    /// Compute entropy of pattern
    pub fn compute_entropy(components: &[Float]) -> Float {
        let mut entropy = 0.0;
        let total: Float = components.iter().map(|c| c.abs()).sum();
        if total < 1e-10 {
            return 0.0;
        }
        for &c in components.iter() {
            let p = c.abs() / total;
            if p > 1e-10 {
                entropy -= p * p.ln();
            }
        }
        entropy
    }

    /// Compute resonance frequency
    pub fn compute_resonance(components: &[Float]) -> Float {
        if components.is_empty() {
            return 0.0;
        }
        let norm_sq: Float = components.iter().map(|c| c.powi(2)).sum();
        norm_sq.sqrt()
    }

    /// Get pattern dimension
    pub fn dimension(&self) -> usize {
        self.components.len()
    }

    /// Get reference to components
    pub fn components_ref(&self) -> &[Float] {
        &self.components
    }

    /// Add another interference pattern (constructive/destructive interference)
    pub fn add(&mut self, other: &InterferencePattern) {
        assert_eq!(
            self.components.len(),
            other.components.len(),
            "Pattern dimensions must match"
        );
        for (a, b) in self.components.iter_mut().zip(other.components.iter()) {
            *a += b;
        }
        self.coherence = Self::compute_coherence(&self.components);
        self.entropy = Self::compute_entropy(&self.components);
        self.resonance = Self::compute_resonance(&self.components);
    }

    /// Apply spectrum filter to pattern
    pub fn apply_spectrum_filter(&mut self, spectrum_ratio: Float) {
        let filter = 1.0 / (1.0 + spectrum_ratio.abs());
        for c in self.components.iter_mut() {
            *c *= filter;
        }
        self.resonance = Self::compute_resonance(&self.components);
    }

    /// Apply density constraint to pattern
    pub fn apply_density_constraint(&mut self, density_level: u8) {
        let constraint = 1.0 / (density_level as Float + 1.0);
        for c in self.components.iter_mut() {
            *c *= constraint;
        }
        self.resonance = Self::compute_resonance(&self.components);
    }
}

/// Action vector - collapsed action from possibility space
///
/// From GAMING_ENGINE_ROADMAP_v2.md:
/// "Free Will choice operator selects from possibility space"
#[derive(Debug, Clone, PartialEq)]
pub struct ActionVector {
    /// Action direction (normalized)
    pub direction: Vec<Float>,
    /// Action magnitude (strength of the action)
    pub magnitude: Float,
    /// Confidence in the action (0.0 to 1.0)
    pub confidence: Float,
    /// Archetypical signature of the action
    pub archetype_signature: [Float; NUM_ARCHETYPES],
    /// Catalyst value (how much catalyst this action provides)
    pub catalyst_value: Float,
}

impl ActionVector {
    /// Create a new action vector
    pub fn new(direction: Vec<Float>, magnitude: Float) -> Self {
        ActionVector {
            direction,
            magnitude,
            confidence: 0.0,
            archetype_signature: [0.0; NUM_ARCHETYPES],
            catalyst_value: 0.0,
        }
    }

    /// Create a zero action vector
    pub fn zero(dimension: usize) -> Self {
        ActionVector {
            direction: vec![0.0; dimension],
            magnitude: 0.0,
            confidence: 0.0,
            archetype_signature: [0.0; NUM_ARCHETYPES],
            catalyst_value: 0.0,
        }
    }

    /// Normalize direction to unit length
    pub fn normalize_direction(&mut self) {
        let norm: Float = self
            .direction
            .iter()
            .map(|c| c.powi(2))
            .sum::<Float>()
            .sqrt();
        if norm > 1e-10 {
            for c in self.direction.iter_mut() {
                *c /= norm;
            }
        }
    }

    /// Get action dimension
    pub fn dimension(&self) -> usize {
        self.direction.len()
    }
}

/// Emergent behavior - behavior that emerges from archetype interference
///
/// From GAMING_ENGINE_ROADMAP_v2.md:
/// "Emergent behavior emerges from holographic interference (no behavior trees)"
#[derive(Debug, Clone, PartialEq)]
pub struct EmergentBehavior {
    /// Interference pattern that generated this behavior
    pub interference_pattern: InterferencePattern,
    /// Action vector (collapsed action)
    pub action_vector: ActionVector,
    /// Confidence in the behavior (0.0 to 1.0)
    pub confidence: Float,
    /// Stability of the behavior (0.0 = volatile, 1.0 = stable)
    pub stability: Float,
    /// Novelty of the behavior (0.0 = habitual, 1.0 = novel)
    pub novelty: Float,
}

impl EmergentBehavior {
    /// Create a new emergent behavior
    pub fn new(interference_pattern: InterferencePattern, action_vector: ActionVector) -> Self {
        let coherence = interference_pattern.coherence;
        let entropy = interference_pattern.entropy;
        let max_entropy = (NUM_ARCHETYPES as Float) * 2.0_f64.ln();

        EmergentBehavior {
            interference_pattern,
            action_vector,
            confidence: coherence,
            stability: 1.0 - (entropy / max_entropy),
            novelty: entropy / max_entropy,
        }
    }

    /// Check if behavior is stable (high confidence, low novelty)
    pub fn is_stable(&self, threshold: Float) -> bool {
        self.stability > threshold && self.confidence > threshold
    }

    /// Check if behavior is novel (high novelty)
    pub fn is_novel(&self, threshold: Float) -> bool {
        self.novelty > threshold
    }
}

/// Archetypical interference engine - creates emergence from archetype combinations
///
/// From GAMING_ENGINE_ROADMAP_v2.md:
/// "The archetypical interference engine creates emergent behavior by combining
/// archetype activation profiles and collapsing to concrete actions."
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 6.3:
/// "Emergent Behavior: Storage: 22 floats + 3 floats + 1 enum (~100 bytes).
/// No behavior tree: Save 1-10 KB per entity. Computation: O(1) tensor operations.
/// Emergence: Infinite variety from finite parameters."
#[derive(Debug, Clone)]
pub struct ArchetypicalInterferenceEngine {
    /// Archetype basis set
    archetype_basis: ArchetypeBasis,
    /// Free Will choice engine
    free_will_engine: FreeWillChoiceEngine,
    /// Observation engine (for observer effect)
    observation_engine: ObservationEngine,
    /// Cached interference patterns
    interference_cache: HashMap<ArchetypeKey, InterferencePattern>,
    /// Maximum cache size
    max_cache_size: usize,
}

/// Cache key for interference patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ArchetypeKey {
    coefficients: [u8; NUM_ARCHETYPES],
}

impl ArchetypeKey {
    fn from_profile(profile: &ArchetypeActivationProfile) -> Self {
        let mut coefficients = [0u8; NUM_ARCHETYPES];
        for (i, &coeff) in profile.coefficients().iter().enumerate() {
            coefficients[i] = (coeff * 255.0) as u8;
        }
        ArchetypeKey { coefficients }
    }
}

/// Engine error types
#[derive(Debug, Clone, PartialEq)]
pub enum InterferenceEngineError {
    EmptyPossibilitySpace,
    InvalidProfile,
    CollapseFailed,
    CacheError(String),
}

impl std::fmt::Display for InterferenceEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterferenceEngineError::EmptyPossibilitySpace => {
                write!(f, "Possibility space is empty")
            }
            InterferenceEngineError::InvalidProfile => {
                write!(f, "Invalid archetype activation profile")
            }
            InterferenceEngineError::CollapseFailed => {
                write!(f, "Failed to collapse possibility to action")
            }
            InterferenceEngineError::CacheError(msg) => {
                write!(f, "Cache error: {}", msg)
            }
        }
    }
}

impl std::error::Error for InterferenceEngineError {}

/// Engine statistics
#[derive(Debug, Clone, Default)]
pub struct InterferenceEngineStatistics {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub behaviors_evaluated: usize,
    pub collapse_attempts: usize,
    pub collapse_failures: usize,
}

impl InterferenceEngineStatistics {
    pub fn hit_rate(&self) -> Float {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as Float / total as Float
        }
    }

    pub fn success_rate(&self) -> Float {
        if self.collapse_attempts == 0 {
            0.0
        } else {
            let successful = self
                .collapse_attempts
                .saturating_sub(self.collapse_failures);
            successful as Float / self.collapse_attempts as Float
        }
    }
}

impl ArchetypicalInterferenceEngine {
    /// Create a new interference engine
    pub fn new(archetype_basis: ArchetypeBasis, free_will_seed: FreeWillSeed) -> Self {
        ArchetypicalInterferenceEngine {
            archetype_basis,
            free_will_engine: FreeWillChoiceEngine::new(free_will_seed),
            observation_engine: ObservationEngine::new(1000),
            interference_cache: HashMap::new(),
            max_cache_size: 1000,
        }
    }

    /// Evaluate emergent behavior for an entity
    pub fn evaluate_behavior(
        &mut self,
        archetype_profile: &ArchetypeActivationProfile,
        spectrum_ratio: Float,
        density_level: u8,
        observer_id: Option<u64>,
        entity_id: Option<u64>,
    ) -> Result<EmergentBehavior, InterferenceEngineError> {
        let mut stats = self.get_statistics();
        stats.behaviors_evaluated += 1;

        let mut interference = self.create_interference_pattern(archetype_profile)?;

        if let Some(obs_id) = observer_id {
            if let Some(ent_id) = entity_id {
                self.apply_observer_effect(&mut interference, obs_id, ent_id);
            }
        }

        interference.apply_spectrum_filter(spectrum_ratio);
        interference.apply_density_constraint(density_level);

        let possibility_space =
            self.generate_possibility_space(&interference, archetype_profile)?;

        if possibility_space.is_empty() {
            return Err(InterferenceEngineError::EmptyPossibilitySpace);
        }

        let choice = self
            .free_will_engine
            .make_choice(&possibility_space, archetype_profile.coefficients())
            .map_err(|_| InterferenceEngineError::CollapseFailed)?;

        let action_vector = self.collapse_to_action(&interference, &choice, archetype_profile)?;

        stats.collapse_attempts += 1;

        let behavior = EmergentBehavior::new(interference, action_vector);

        Ok(behavior)
    }

    /// Create interference pattern from archetype activation profile
    fn create_interference_pattern(
        &mut self,
        profile: &ArchetypeActivationProfile,
    ) -> Result<InterferencePattern, InterferenceEngineError> {
        let key = ArchetypeKey::from_profile(profile);

        if let Some(cached) = self.interference_cache.get(&key) {
            return Ok(cached.clone());
        }

        let pattern = self.archetype_basis.reconstruct(profile);
        let mut components = pattern.components().to_vec();

        self.apply_interference_physics(&mut components, profile);

        let mut interference = InterferencePattern::new(components);

        self.identify_dominant_archetypes(&mut interference, profile);

        if self.interference_cache.len() >= self.max_cache_size {
            self.evict_oldest_entry();
        }
        self.interference_cache.insert(key, interference.clone());

        Ok(interference)
    }

    /// Apply interference physics to pattern components
    fn apply_interference_physics(
        &self,
        components: &mut [Float],
        profile: &ArchetypeActivationProfile,
    ) {
        let coefficients = profile.coefficients();

        for (i, c) in components.iter_mut().enumerate() {
            let profile_coeff = coefficients.get(i).copied().unwrap_or(0.0);
            *c *= (1.0 + profile_coeff).tanh();
        }

        let total: Float = components.iter().map(|c| c.abs()).sum();
        if total > 1e-10 {
            for c in components.iter_mut() {
                *c /= total;
            }
        }
    }

    /// Identify dominant archetypes in interference pattern
    fn identify_dominant_archetypes(
        &self,
        interference: &mut InterferencePattern,
        profile: &ArchetypeActivationProfile,
    ) {
        let mut archetypes: Vec<(usize, Float)> = profile
            .coefficients()
            .iter()
            .enumerate()
            .map(|(i, &c)| (i, c.abs()))
            .collect();

        archetypes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        interference.dominant_archetypes = archetypes.into_iter().take(5).collect();
    }

    /// Apply observer effect to interference pattern
    fn apply_observer_effect(
        &mut self,
        interference: &mut InterferencePattern,
        observer_id: u64,
        entity_id: u64,
    ) {
        let observer_effect = ObserverEffect::new(observer_id, entity_id).with_strength(0.5);

        match self
            .observation_engine
            .observe(observer_effect, &interference.components_ref())
        {
            Ok(collapsed) => {
                for (c, &state) in interference
                    .components
                    .iter_mut()
                    .zip(collapsed.state_vector().iter())
                {
                    *c = *c * 0.7 + state * 0.3;
                }
                interference.coherence =
                    InterferencePattern::compute_coherence(&interference.components);
                interference.entropy =
                    InterferencePattern::compute_entropy(&interference.components);
                interference.resonance =
                    InterferencePattern::compute_resonance(&interference.components);
            }
            Err(_) => {}
        }
    }

    /// Generate possibility space from interference pattern
    fn generate_possibility_space(
        &self,
        interference: &InterferencePattern,
        profile: &ArchetypeActivationProfile,
    ) -> Result<PossibilitySpace, InterferenceEngineError> {
        let mut possibility_space = PossibilitySpace::new();

        let components = interference.components_ref();
        let dimension = components.len();

        let top_indices: Vec<usize> = interference
            .dominant_archetypes
            .iter()
            .take(7)
            .map(|(i, _)| *i)
            .collect();

        for i in 0..7.min(top_indices.len()) {
            let archetype_idx = top_indices[i];
            let weight = components.get(archetype_idx).copied().unwrap_or(0.0);

            if weight > 0.01 {
                let signature = self.compute_action_signature(archetype_idx, profile);
                let action_type = self.infer_action_type(archetype_idx);
                let possibility = Possibility {
                    id: format!("action_{}", archetype_idx),
                    action_type,
                    archetype_signature: signature,
                    catalyst_value: weight * interference.coherence,
                };
                possibility_space.add_choice(possibility, weight);
            }
        }

        Ok(possibility_space)
    }

    /// Compute action signature for archetype
    fn compute_action_signature(
        &self,
        archetype_idx: usize,
        profile: &ArchetypeActivationProfile,
    ) -> [Float; NUM_ARCHETYPES] {
        let mut signature = [0.0; NUM_ARCHETYPES];

        signature[archetype_idx] = 1.0;

        let coefficients = profile.coefficients();
        for (i, &coeff) in coefficients.iter().enumerate() {
            if i != archetype_idx {
                signature[i] = coeff * 0.3;
            }
        }

        let total: Float = signature.iter().sum();
        if total > 1e-10 {
            for s in signature.iter_mut() {
                *s /= total;
            }
        }

        signature
    }

    /// Infer action type from archetype
    fn infer_action_type(
        &self,
        archetype_idx: usize,
    ) -> crate::simulation_v3::free_will_seed::ActionType {
        use crate::simulation_v3::free_will_seed::ActionType;

        match archetype_idx {
            0 | 1 | 2 => ActionType::Move,
            3 | 4 | 5 => ActionType::Interact,
            6 | 7 => ActionType::Observe,
            8 | 9 | 10 => ActionType::Create,
            11 | 12 => ActionType::Transform,
            13 | 14 => ActionType::Communicate,
            15 | 16 => ActionType::Rest,
            _ => ActionType::Evolve,
        }
    }

    /// Collapse possibility to action vector
    fn collapse_to_action(
        &self,
        interference: &InterferencePattern,
        choice: &crate::simulation_v3::free_will_seed::Choice,
        profile: &ArchetypeActivationProfile,
    ) -> Result<ActionVector, InterferenceEngineError> {
        let dimension = interference.dimension();
        let mut direction = vec![0.0; dimension];

        let components = interference.components_ref();
        for (d, c) in direction.iter_mut().zip(components.iter()) {
            *d = *c * choice.confidence;
        }

        let magnitude = interference.resonance * choice.confidence;

        let mut action_vector = ActionVector::new(direction, magnitude);
        action_vector.normalize_direction();
        action_vector.confidence = choice.confidence;
        action_vector.archetype_signature = *profile.coefficients();
        action_vector.catalyst_value = interference.coherence * magnitude;

        Ok(action_vector)
    }

    /// Evict oldest entry from cache
    fn evict_oldest_entry(&mut self) {
        if let Some(key) = self.interference_cache.keys().next().cloned() {
            self.interference_cache.remove(&key);
        }
    }

    /// Get engine statistics
    pub fn get_statistics(&self) -> InterferenceEngineStatistics {
        InterferenceEngineStatistics {
            cache_hits: self.free_will_engine.statistics().total_choices,
            cache_misses: self.interference_cache.len(),
            behaviors_evaluated: 0,
            collapse_attempts: 0,
            collapse_failures: 0,
        }
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.interference_cache.clear();
    }

    /// Set max cache size
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
        while self.interference_cache.len() > size {
            self.evict_oldest_entry();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_pattern_creation() {
        let components = vec![0.1, 0.2, 0.3, 0.4];
        let pattern = InterferencePattern::new(components.clone());

        assert_eq!(pattern.dimension(), 4);
        assert!(pattern.coherence >= 0.0 && pattern.coherence <= 1.0);
        assert!(pattern.resonance >= 0.0);
    }

    #[test]
    fn test_interference_pattern_zero() {
        let pattern = InterferencePattern::zero(10);
        assert_eq!(pattern.dimension(), 10);
        assert_eq!(pattern.coherence, 0.0);
        assert_eq!(pattern.resonance, 0.0);
    }

    #[test]
    fn test_interference_pattern_add() {
        let mut pattern1 = InterferencePattern::new(vec![0.1, 0.2, 0.3]);
        let pattern2 = InterferencePattern::new(vec![0.4, 0.5, 0.6]);

        pattern1.add(&pattern2);

        assert_eq!(pattern1.components_ref(), &[0.5, 0.7, 0.9]);
    }

    #[test]
    fn test_interference_pattern_spectrum_filter() {
        let mut pattern = InterferencePattern::new(vec![1.0, 1.0, 1.0]);
        pattern.apply_spectrum_filter(2.0);

        for c in pattern.components_ref() {
            assert!(*c < 1.0);
        }
    }

    #[test]
    fn test_action_vector_creation() {
        let direction = vec![1.0, 0.0, 0.0];
        let mut action = ActionVector::new(direction, 5.0);

        action.normalize_direction();

        assert_eq!(action.magnitude, 5.0);
        assert_eq!(action.dimension(), 3);
    }

    #[test]
    fn test_emergent_behavior_creation() {
        let interference = InterferencePattern::new(vec![0.5, 0.5, 0.5, 0.5]);
        let action = ActionVector::new(vec![1.0, 0.0, 0.0, 0.0], 1.0);

        let behavior = EmergentBehavior::new(interference.clone(), action);

        assert_eq!(behavior.interference_pattern.dimension(), 4);
        assert!(behavior.confidence >= 0.0 && behavior.confidence <= 1.0);
    }

    #[test]
    fn test_engine_statistics() {
        let stats = InterferenceEngineStatistics::default();
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
    }

    #[test]
    fn test_compute_coherence() {
        let components = vec![1.0, 1.0, 1.0, 1.0];
        let coherence = InterferencePattern::compute_coherence(&components);
        assert_eq!(coherence, 1.0);
    }

    #[test]
    fn test_compute_entropy() {
        let components = vec![0.25, 0.25, 0.25, 0.25];
        let entropy = InterferencePattern::compute_entropy(&components);
        assert!(entropy > 0.0);
    }

    #[test]
    fn test_compute_resonance() {
        let components = vec![3.0, 4.0];
        let resonance = InterferencePattern::compute_resonance(&components);
        assert!((resonance - 5.0).abs() < 1e-10);
    }
}
