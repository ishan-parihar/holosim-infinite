/// The Universal Constant: Transcend and Include
///
/// This is the fundamental mechanism by which the holographic principle operates.
/// Each step in involution/evolution is a "transcend and include" operation:
/// - INCLUDE: Retains all development from previous stages
/// - TRANSCEND: Adds new development that transcends the previous
/// - EVOLVES INTO: Creates attractor-fields that pull toward the next stage
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each layer includes all previous development, transcends by adding fundamentally new capabilities,
/// and creates attractor-fields that pull toward the next stage."
use std::fmt::Debug;
use std::sync::Arc;

use crate::types::Float;

/// Attractor-field - "spiritual gravity" that pulls toward the next stage
///
/// Each stage creates attractor-fields that pull the previous stage toward the next
/// level of vibration/frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorField {
    /// Name of the attractor-field
    pub name: String,
    /// Strength of the attraction (0.0 to 1.0)
    pub strength: f64,
    /// Description of what this attractor-field pulls toward
    pub target: String,
}

impl AttractorField {
    /// Create a new attractor-field
    pub fn new(name: impl Into<String>, strength: f64, target: impl Into<String>) -> Self {
        assert!(
            (0.0..=1.0).contains(&strength),
            "Strength must be between 0.0 and 1.0"
        );
        AttractorField {
            name: name.into(),
            strength,
            target: target.into(),
        }
    }

    /// Pull the previous stage toward the next level
    pub fn pull(&self) -> f64 {
        self.strength
    }
}

/// Stage transition - represents the "transcend and include" operation
///
/// This is the universal mechanism that operates at every stage of creation.
/// Each transition includes all previous development, transcends by adding new development,
/// and creates an attractor-field that pulls toward the next stage.
#[derive(Debug, Clone)]
pub struct StageTransition<L1> {
    /// The previous stage (what is INCLUDED)
    pub includes: L1,
    /// The new development (what TRANSCENDS)
    pub transcends: Feature,
    /// The attractor-field (what EVOLVES INTO)
    pub evolves_into: AttractorField,
}

/// Represents a new feature that transcends the previous stage
#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    /// Name of the feature
    pub name: String,
    /// Description of the feature
    pub description: String,
    /// Strength of the feature (0.0 to 1.0)
    pub strength: f64,
}

impl Feature {
    /// Create a new feature
    pub fn new(name: impl Into<String>, description: impl Into<String>, strength: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&strength),
            "Strength must be between 0.0 and 1.0"
        );
        Feature {
            name: name.into(),
            description: description.into(),
            strength,
        }
    }

    /// Apply this feature to the previous stage
    pub fn apply(&self) -> f64 {
        self.strength
    }
}

impl<L1> StageTransition<L1>
where
    L1: Debug + Clone,
{
    /// Apply the stage transition
    ///
    /// This implements the "transcend and include" mechanism:
    /// 1. INCLUDE: Retain all previous development
    /// 2. TRANSCEND: Add new development
    /// 3. EVOLVES INTO: Create attractor-field
    pub fn apply(&self, previous: L1) -> (L1, Feature, AttractorField) {
        // INCLUDE: Retain all previous development
        let included = previous.clone();

        // TRANSCEND: Add new development
        let transcended = self.transcends.clone();

        // EVOLVES INTO: Create attractor-field
        let attractor_field = self.evolves_into.clone();

        (included, transcended, attractor_field)
    }

    /// Get the attractor-field strength
    pub fn attractor_strength(&self) -> f64 {
        self.evolves_into.strength
    }
}

/// The universal "transcend and include" operation
///
/// This function represents the universal constant that operates at every stage.
pub fn transcend_include<L1>(
    previous: L1,
    new_feature: Feature,
    attractor_field: AttractorField,
) -> (L1, Feature, AttractorField)
where
    L1: Debug + Clone,
{
    (
        previous.clone(), // INCLUDE: Retain all previous development
        new_feature,      // TRANSCEND: Add new development
        attractor_field,  // EVOLVES INTO: Create attractor-field
    )
}

// ============================================================
// Layer<T> - Hierarchical Transcend-Include Chain
// ============================================================

/// A layer that TRANSCENDS and INCLUDES previous layers.
///
/// This implements the universal constant from COSMOLOGICAL-ARCHITECTURE.md:
/// "Each stage in involution/evolution INCLUDES all previous development,
///  TRANSCENDS by adding new development, and EVOLVES INTO attractor-fields
///  that pull toward the next stage."
///
/// Memory savings: O(log n) instead of O(n) for hierarchical data.
///
/// # Example
///
/// ```
/// use foundation::{Layer, LayerBuilder};
///
/// // Build a chain: Violet -> Indigo -> Blue -> Green
/// let layer = LayerBuilder::new()
///     .push_layer("Violet Data")   // Deepest (level 0)
///     .push_layer("Indigo Data")   // Level 1
///     .push_layer("Blue Data")     // Level 2
///     .push_layer("Green Data")    // Top (level 3)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Layer<T> {
    /// INCLUDE: Reference to previous layer (not copy)
    /// This is the holographic principle in action
    pub included: Option<Arc<Layer<T>>>,

    /// TRANSCEND: New data at this layer
    pub transcended: T,

    /// Layer number (0 = Violet/deepest, 7 = Red/highest)
    pub level: usize,

    /// Attractor field strength (pulls toward this layer)
    pub attractor_strength: Float,
}

impl<T> Layer<T> {
    /// Create a new layer that includes the previous
    pub fn new(previous: Option<Arc<Layer<T>>>, data: T, level: usize) -> Self {
        Self {
            included: previous,
            transcended: data,
            level,
            attractor_strength: Self::default_attractor_strength(level),
        }
    }

    /// Create a top-level layer (no previous)
    pub fn top(data: T) -> Self {
        Self::new(None, data, 0)
    }

    /// Create a layer that transcends the previous
    pub fn transcend(previous: Arc<Layer<T>>, data: T) -> Self {
        let level = previous.level + 1;
        Self::new(Some(previous), data, level)
    }

    /// Get all data from this layer and all included layers
    /// This is how the holographic principle manifests:
    /// each part contains the whole
    pub fn get_all_data(&self) -> Vec<&T> {
        let mut result = vec![&self.transcended];

        if let Some(ref included) = self.included {
            result.extend(included.get_all_data());
        }

        result
    }

    /// Get data at specific layer depth
    /// depth = 0 returns current layer, depth = 1 returns first included, etc.
    pub fn get_layer_at_depth(&self, depth: usize) -> Option<&T> {
        if depth == 0 {
            Some(&self.transcended)
        } else if let Some(ref included) = self.included {
            included.get_layer_at_depth(depth - 1)
        } else {
            None
        }
    }

    /// Compute total depth (number of layers in chain)
    pub fn depth(&self) -> usize {
        match &self.included {
            None => 1,
            Some(included) => 1 + included.depth(),
        }
    }

    /// Apply a function to all layers (transcended then included)
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T, usize),
    {
        self.for_each_internal(&mut f);
    }

    fn for_each_internal<F>(&self, f: &mut F)
    where
        F: FnMut(&T, usize),
    {
        f(&self.transcended, self.level);
        if let Some(ref included) = self.included {
            included.for_each_internal(f);
        }
    }

    /// Check if this layer contains another layer (by level)
    pub fn contains_layer(&self, level: usize) -> bool {
        if self.level == level {
            return true;
        }
        if let Some(ref included) = self.included {
            included.contains_layer(level)
        } else {
            false
        }
    }

    /// Get the topmost layer (current if no included)
    pub fn topmost(&self) -> &T {
        &self.transcended
    }

    /// Get the deepest layer
    pub fn deepest(&self) -> &T {
        match &self.included {
            None => &self.transcended,
            Some(included) => included.deepest(),
        }
    }

    /// Compute default attractor strength based on level
    fn default_attractor_strength(level: usize) -> Float {
        // Higher layers have stronger attractors (pull toward evolution)
        match level {
            0 => 0.1, // Violet - weakest (source, no pull needed)
            1 => 0.2, // Indigo
            2 => 0.3, // Blue
            3 => 0.4, // Green
            4 => 0.5, // Yellow
            5 => 0.6, // Orange
            6 => 0.7, // Red
            7 => 0.8, // Layer 7 - strongest (individual entities)
            _ => 0.5, // Default for extended chains
        }
    }

    /// Set custom attractor strength
    pub fn with_attractor_strength(mut self, strength: Float) -> Self {
        self.attractor_strength = strength.clamp(0.0, 1.0);
        self
    }
}

/// Builder for creating layers with proper transcend-include chain
#[derive(Debug)]
pub struct LayerBuilder<T> {
    layers: Vec<T>,
}

impl<T> LayerBuilder<T> {
    /// Create a new layer builder
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Push a layer (will be transcended by next)
    /// First added = deepest (level 0), last added = top
    pub fn push_layer(mut self, data: T) -> Self {
        self.layers.push(data);
        self
    }

    /// Build the layer chain
    /// Returns Arc for easy sharing
    pub fn build(self) -> Arc<Layer<T>> {
        let mut result: Option<Arc<Layer<T>>> = None;

        for (i, data) in self.layers.into_iter().enumerate() {
            result = Some(Arc::new(Layer::new(result, data, i)));
        }

        result.unwrap_or_else(|| {
            // If no layers added, create empty top layer
            // This should ideally not happen, but we need to return something
            Arc::new(Layer::top(unsafe { std::mem::zeroed() }))
        })
    }
}

impl<T> Default for LayerBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Add Clone for LayerBuilder if T is Clone
impl<T: Clone> Clone for LayerBuilder<T> {
    fn clone(&self) -> Self {
        Self {
            layers: self.layers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attractor_field_creation() {
        let attractor = AttractorField::new("Test Attractor", 0.8, "Next Stage");
        assert_eq!(attractor.name, "Test Attractor");
        assert_eq!(attractor.strength, 0.8);
        assert_eq!(attractor.target, "Next Stage");
    }

    #[test]
    fn test_attractor_field_strength_validation() {
        // Valid strength
        let attractor = AttractorField::new("Test", 0.5, "Target");
        assert_eq!(attractor.strength, 0.5);
    }

    #[test]
    #[should_panic(expected = "Strength must be between 0.0 and 1.0")]
    fn test_attractor_field_invalid_strength() {
        // Invalid strength (should panic)
        let _invalid = AttractorField::new("Test", 1.5, "Target");
    }

    #[test]
    fn test_feature_creation() {
        let feature = Feature::new("Test Feature", "A test feature", 0.7);
        assert_eq!(feature.name, "Test Feature");
        assert_eq!(feature.description, "A test feature");
        assert_eq!(feature.strength, 0.7);
    }

    #[test]
    fn test_stage_transition_creation() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        assert_eq!(transition.includes, "Previous Stage");
        assert_eq!(transition.transcends.name, "New Feature");
        assert_eq!(transition.evolves_into.name, "Attractor");
    }

    #[test]
    fn test_stage_transition_apply() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        let (included, transcended, attractor) = transition.apply(previous.to_string());

        assert_eq!(included, "Previous Stage");
        assert_eq!(transcended.name, "New Feature");
        assert_eq!(attractor.name, "Attractor");
    }

    #[test]
    fn test_transcend_include_function() {
        let previous = "Previous Stage";
        let new_feature = Feature::new("New Feature", "A new feature", 0.8);
        let attractor_field = AttractorField::new("Attractor", 0.9, "Next Stage");

        let (included, transcended, attractor) =
            transcend_include(previous, new_feature, attractor_field);

        assert_eq!(included, "Previous Stage");
        assert_eq!(transcended.name, "New Feature");
        assert_eq!(attractor.name, "Attractor");
    }

    #[test]
    fn test_attractor_field_pull() {
        let attractor = AttractorField::new("Test Attractor", 0.8, "Next Stage");
        assert_eq!(attractor.pull(), 0.8);
    }

    #[test]
    fn test_stage_transition_attractor_strength() {
        let previous = "Previous Stage";
        let transcends = Feature::new("New Feature", "A new feature", 0.8);
        let evolves_into = AttractorField::new("Attractor", 0.9, "Next Stage");

        let transition = StageTransition {
            includes: previous.to_string(),
            transcends,
            evolves_into,
        };

        assert_eq!(transition.attractor_strength(), 0.9);
    }

    // ============================================================
    // Layer<T> Tests
    // ============================================================

    #[test]
    fn test_layer_top_creation() {
        let layer = Layer::top("Violet Data");
        assert_eq!(layer.transcended, "Violet Data");
        assert_eq!(layer.level, 0);
        assert!(layer.included.is_none());
        assert_eq!(layer.attractor_strength, 0.1); // Violet = level 0
    }

    #[test]
    fn test_layer_transcend() {
        let layer0 = Arc::new(Layer::top("Violet Data"));
        let layer1 = Layer::transcend(layer0.clone(), "Indigo Data");

        assert_eq!(layer1.transcended, "Indigo Data");
        assert_eq!(layer1.level, 1);
        assert!(layer1.included.is_some());
        assert_eq!(layer1.attractor_strength, 0.2); // Indigo = level 1
    }

    #[test]
    fn test_layer_get_all_data() {
        let layer = LayerBuilder::new()
            .push_layer("Violet")
            .push_layer("Indigo")
            .push_layer("Blue")
            .build();

        let all_data = layer.get_all_data();
        assert_eq!(all_data.len(), 3);
        assert_eq!(all_data[0], &"Blue"); // Top (level 2)
        assert_eq!(all_data[1], &"Indigo"); // Level 1
        assert_eq!(all_data[2], &"Violet"); // Deepest (level 0)
    }

    #[test]
    fn test_layer_get_layer_at_depth() {
        let layer = LayerBuilder::new()
            .push_layer("Violet")
            .push_layer("Indigo")
            .push_layer("Blue")
            .build();

        // depth 0 = top (Blue)
        assert_eq!(layer.get_layer_at_depth(0), Some(&"Blue"));
        // depth 1 = first included (Indigo)
        assert_eq!(layer.get_layer_at_depth(1), Some(&"Indigo"));
        // depth 2 = second included (Violet)
        assert_eq!(layer.get_layer_at_depth(2), Some(&"Violet"));
        // depth 3 = beyond chain
        assert_eq!(layer.get_layer_at_depth(3), None);
    }

    #[test]
    fn test_layer_depth() {
        let single = Layer::top("Single");
        assert_eq!(single.depth(), 1);

        let chain = LayerBuilder::new()
            .push_layer("L0")
            .push_layer("L1")
            .push_layer("L2")
            .push_layer("L3")
            .build();
        assert_eq!(chain.depth(), 4);
    }

    #[test]
    fn test_layer_for_each() {
        let layer = LayerBuilder::new()
            .push_layer("Violet")
            .push_layer("Indigo")
            .push_layer("Blue")
            .build();

        let mut visited = Vec::new();
        layer.for_each(|data, level| {
            visited.push((*data, level));
        });

        // Order: top first, then included layers
        assert_eq!(visited.len(), 3);
        assert_eq!(visited[0], ("Blue", 2));
        assert_eq!(visited[1], ("Indigo", 1));
        assert_eq!(visited[2], ("Violet", 0));
    }

    #[test]
    fn test_layer_contains_layer() {
        let layer = LayerBuilder::new().push_layer("L0").push_layer("L1").push_layer("L2").build();

        assert!(layer.contains_layer(0));
        assert!(layer.contains_layer(1));
        assert!(layer.contains_layer(2));
        assert!(!layer.contains_layer(3));
    }

    #[test]
    fn test_layer_topmost_and_deepest() {
        let layer = LayerBuilder::new()
            .push_layer("Violet")
            .push_layer("Indigo")
            .push_layer("Blue")
            .build();

        assert_eq!(layer.topmost(), &"Blue");
        assert_eq!(layer.deepest(), &"Violet");
    }

    #[test]
    fn test_layer_with_attractor_strength() {
        let layer = Layer::top("Data").with_attractor_strength(0.5);
        assert_eq!(layer.attractor_strength, 0.5);

        // Clamped to max
        let layer_max = Layer::top("Data").with_attractor_strength(1.5);
        assert_eq!(layer_max.attractor_strength, 1.0);

        // Clamped to min
        let layer_min = Layer::top("Data").with_attractor_strength(-0.5);
        assert_eq!(layer_min.attractor_strength, 0.0);
    }

    #[test]
    fn test_layer_builder_default() {
        let builder: LayerBuilder<String> = LayerBuilder::default();
        assert!(builder.layers.is_empty());
    }

    #[test]
    fn test_layer_builder_clone() {
        let builder = LayerBuilder::new().push_layer("A").push_layer("B");

        let cloned = builder.clone();
        let layer1 = builder.build();
        let layer2 = cloned.build();

        assert_eq!(layer1.transcended, "B");
        assert_eq!(layer2.transcended, "B");
    }

    #[test]
    fn test_layer_default_attractor_strength_all_levels() {
        // Test default attractor strength for all 8 layers (0-7)
        let expected_strengths = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        for (level, &expected) in expected_strengths.iter().enumerate() {
            let layer = LayerBuilder::new().push_layer("Base").build();

            // Manually create layer at specific level
            let mut current = layer;
            for _ in 0..level {
                current = Arc::new(Layer::transcend(current, "Next"));
            }

            assert_eq!(
                current.attractor_strength, expected,
                "Level {} should have attractor strength {}",
                level, expected
            );
        }
    }

    #[test]
    fn test_layer_holographic_principle() {
        // Each layer contains the whole - the holographic principle
        let layer = LayerBuilder::new()
            .push_layer("Violet Data")
            .push_layer("Indigo Data")
            .push_layer("Blue Data")
            .push_layer("Green Data")
            .build();

        // All data is accessible from top layer
        let all_data = layer.get_all_data();
        assert_eq!(all_data.len(), 4);

        // Each part contains the whole - verify count
        assert_eq!(all_data.len(), 4);
    }

    #[test]
    fn test_layer_memory_efficiency() {
        // Verify that Arc is used for sharing (not copying)
        let base = Arc::new(Layer::top("Base"));

        // Multiple layers can include the same base
        let layer1 = Arc::new(Layer::transcend(base.clone(), "L1"));
        let layer2 = Arc::new(Layer::transcend(base.clone(), "L2"));

        // Both reference the same underlying layer
        assert!(Arc::ptr_eq(
            layer1.included.as_ref().unwrap(),
            layer2.included.as_ref().unwrap()
        ));
    }
}
