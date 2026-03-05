//! # Layer Reference System - "Transcend and Include"
//!
//! Implementation of the holographic principle through reference-based layering.
//!
//! ## Concept
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each layer INCLUDES all previous development (retains the whole)
//!  and TRANSCENDS by adding new development (adds something new).
//!  The result is that each layer contains the WHOLE plus something NEW."
//!
//! ## Implementation
//!
//! Instead of copying data from previous layers, we use Arc references.
//! This provides:
//! - Memory efficiency: O(log n) instead of O(n) for layer depth
//! - Holographic completeness: Each layer contains the whole
//! - Fractal self-similarity: Nested structure mirrors the whole
//!
//! ## The 7 Layers
//!
//! 0. Violet-Ray (Infinity) - Source
//! 1. Indigo-Ray (IntelligentInfinity) - Gateway
//! 2. Blue-Ray (Love/Light) - Logos
//! 3. Green-Ray (Light/Love) - Field
//! 4. Yellow-Ray (Dimensions) - Fabric
//! 5. Orange-Ray (Galactic) - Patterns
//! 6. Red-Ray (Solar) - Systems
//! 7. Layer 7 (Sub-Sub-Logos) - Individual

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::types::Float;

// =============================================================================
// LAYER REFERENCE - The Core Data Structure
// =============================================================================

/// Reference-based layer for "Transcend and Include"
///
/// Each layer INCLUDES a reference to the previous layer (not a copy)
/// and TRANSCENDS by adding new data at this layer.
///
/// Memory savings: O(n) → O(log n) for layer depth
#[derive(Debug, Clone)]
pub struct LayerReference<T> {
    /// INCLUDE: Reference to previous layer (shared, immutable)
    /// None for base layer (Violet-Ray)
    pub included: Option<Arc<LayerReference<T>>>,

    /// TRANSCEND: New data at this layer
    pub transcended: T,

    /// Layer index (0 = Violet, 7 = Layer 7)
    pub layer_index: usize,

    /// Layer name for debugging
    pub layer_name: LayerName,
}

/// Named layers matching involution sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerName {
    Violet, // Layer 0: Infinity
    Indigo, // Layer 1: IntelligentInfinity
    Blue,   // Layer 2: Love/Light
    Green,  // Layer 3: Light/Love
    Yellow, // Layer 4: Dimensions
    Orange, // Layer 5: Galactic
    Red,    // Layer 6: Solar
    Entity, // Layer 7: Sub-Sub-Logos
}

impl LayerName {
    /// Get layer index
    pub fn index(&self) -> usize {
        match self {
            LayerName::Violet => 0,
            LayerName::Indigo => 1,
            LayerName::Blue => 2,
            LayerName::Green => 3,
            LayerName::Yellow => 4,
            LayerName::Orange => 5,
            LayerName::Red => 6,
            LayerName::Entity => 7,
        }
    }

    /// Get layer from index
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(LayerName::Violet),
            1 => Some(LayerName::Indigo),
            2 => Some(LayerName::Blue),
            3 => Some(LayerName::Green),
            4 => Some(LayerName::Yellow),
            5 => Some(LayerName::Orange),
            6 => Some(LayerName::Red),
            7 => Some(LayerName::Entity),
            _ => None,
        }
    }

    /// Get corresponding involution layer
    pub fn to_involution_layer(&self) -> Option<InvolutionLayer> {
        match self {
            LayerName::Violet => Some(InvolutionLayer::Violet),
            LayerName::Indigo => Some(InvolutionLayer::Indigo),
            LayerName::Blue => Some(InvolutionLayer::Blue),
            LayerName::Green => Some(InvolutionLayer::Green),
            LayerName::Yellow => Some(InvolutionLayer::Yellow),
            LayerName::Orange => Some(InvolutionLayer::Orange),
            LayerName::Red => Some(InvolutionLayer::Red),
            LayerName::Entity => None, // Layer 7 is beyond involution layers
        }
    }
}

impl<T: Clone> LayerReference<T> {
    /// Create a new base layer (Violet-Ray - no previous layer)
    pub fn base(data: T) -> Self {
        Self {
            included: None,
            transcended: data,
            layer_index: 0,
            layer_name: LayerName::Violet,
        }
    }

    /// Create a new layer that includes a previous layer
    pub fn transcend(previous: Arc<LayerReference<T>>, new_data: T) -> Self {
        let layer_index = previous.layer_index + 1;
        let layer_name = LayerName::from_index(layer_index).unwrap_or(LayerName::Entity);

        Self {
            included: Some(previous),
            transcended: new_data,
            layer_index,
            layer_name,
        }
    }

    /// Get all data from all layers (holographic completeness)
    ///
    /// This iterates through the reference chain to collect all data.
    /// The most recent layer comes first.
    pub fn get_all_data(&self) -> Vec<&T> {
        let mut all_data = vec![&self.transcended];

        let mut current = self.included.as_ref();
        while let Some(layer) = current {
            all_data.push(&layer.transcended);
            current = layer.included.as_ref();
        }

        all_data
    }

    /// Get data at a specific layer depth
    pub fn get_layer_data(&self, depth: usize) -> Option<&T> {
        if depth == self.layer_index {
            Some(&self.transcended)
        } else if depth < self.layer_index {
            self.included.as_ref()?.get_layer_data(depth)
        } else {
            None
        }
    }

    /// Get the base layer data (Violet-Ray)
    pub fn get_base_data(&self) -> &T {
        if self.layer_index == 0 {
            &self.transcended
        } else {
            self.included.as_ref().unwrap().get_base_data()
        }
    }

    /// Count total layers in chain
    pub fn depth(&self) -> usize {
        self.layer_index + 1
    }

    /// Check if this is the base layer
    pub fn is_base(&self) -> bool {
        self.included.is_none()
    }

    /// Check if this is the top layer (Layer 7)
    pub fn is_entity_layer(&self) -> bool {
        self.layer_name == LayerName::Entity
    }
}

// =============================================================================
// LAYERED DATA STRUCTURES
// =============================================================================

/// Data specific to each involution layer
///
/// Each layer adds specific features as described in COSMOLOGICAL-ARCHITECTURE.md
#[derive(Debug, Clone)]
pub struct LayerData {
    /// Layer name
    pub name: LayerName,

    /// Features added at this layer (TRANSCEND)
    pub features: LayerFeatures,

    /// Attractor field created at this layer (EVOLVES INTO)
    pub attractor_field: Option<AttractorFieldData>,

    /// Coherence level at this layer
    pub coherence: Float,
}

/// Features added at each layer during involution
#[derive(Debug, Clone)]
pub enum LayerFeatures {
    /// Violet: Undifferentiated Unity
    Violet {
        /// Rhythmic pulse frequency
        pulse_frequency: Float,
    },

    /// Indigo: IntelligentInfinity, Free Will
    Indigo {
        /// Free Will kernel activation level
        free_will_activation: Float,
        /// Archetype 22 (The Choice) presence
        has_choice_operator: bool,
    },

    /// Blue: Love/Light, Logos, Universal Archetypical Patterns
    Blue {
        /// Logos presence
        has_logos: bool,
        /// Universal archetype patterns configured
        archetype_patterns: [Float; 22],
    },

    /// Green: Light/Love, Holographic Field
    Green {
        /// Light/Love ratio
        light_love_ratio: Float,
        /// Field of potential coherence
        field_coherence: Float,
    },

    /// Yellow: Dimensions, Spectrum, Veil
    Yellow {
        /// Space/Time ratio
        space_time_ratio: Float,
        /// Veil transparency
        veil_transparency: Float,
        /// Dimensional configuration
        dimensions: u8,
    },

    /// Orange: Galactic-scale Logoi
    Orange {
        /// Galactic pattern configuration
        galactic_pattern: Float,
        /// Number of solar systems in galaxy
        solar_system_count: u64,
    },

    /// Red: Solar-scale Logoi/Sub-Logoi
    Red {
        /// Solar system archetype system type
        archetype_system_type: ArchetypeSystemType,
        /// Planetary count
        planetary_count: u8,
    },

    /// Entity: Sub-Sub-Logos
    Entity {
        /// Individual spectrum configuration
        individual_spectrum: Float,
        /// Holographic blueprint presence
        has_blueprint: bool,
    },
}

/// Archetype system type (10 or 22 archetypes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeSystemType {
    Ten,
    TwentyTwo,
}

/// Attractor field data for "EVOLVES INTO" phase
#[derive(Debug, Clone)]
pub struct AttractorFieldData {
    /// Attractor strength
    pub strength: Float,

    /// Attractor range
    pub range: Float,

    /// Resonance frequency
    pub frequency: Float,

    /// Target layer (what this attractor pulls toward)
    pub target_layer: LayerName,
}

// =============================================================================
// LAYER BUILDER
// =============================================================================

/// Builder for constructing layered data structures
pub struct LayerBuilder<T> {
    layers: Vec<(LayerName, T)>,
}

impl<T: Clone> LayerBuilder<T> {
    /// Create a new layer builder starting with Violet-Ray
    pub fn new(base_data: T) -> Self {
        Self {
            layers: vec![(LayerName::Violet, base_data)],
        }
    }

    /// Add a layer (transcend)
    pub fn add_layer(mut self, name: LayerName, data: T) -> Self {
        self.layers.push((name, data));
        self
    }

    /// Build the layered reference structure
    pub fn build(self) -> Arc<LayerReference<T>> {
        let mut iter = self.layers.into_iter();
        let (first_name, first_data) = iter.next().unwrap();

        let mut current = Arc::new(LayerReference {
            included: None,
            transcended: first_data,
            layer_index: first_name.index(),
            layer_name: first_name,
        });

        for (_name, data) in iter {
            current = Arc::new(LayerReference::transcend(current, data));
        }

        current
    }
}

// =============================================================================
// HIERARCHICAL TEMPLATE - Combining Template with Layers
// =============================================================================

/// A template with layered data structure
///
/// This combines the UniversalTemplate pattern with the LayerReference
/// pattern to create a complete holographic entity structure.
pub struct HierarchicalTemplate<T> {
    /// The current (top) layer
    pub current_layer: Arc<LayerReference<T>>,

    /// Shared holographic field for all layers
    pub shared_field: Arc<HolographicField>,
}

impl<T: Clone> HierarchicalTemplate<T> {
    /// Create from a layered structure
    pub fn new(layers: Arc<LayerReference<T>>, field: Arc<HolographicField>) -> Self {
        Self {
            current_layer: layers,
            shared_field: field,
        }
    }

    /// Get data at all layers
    pub fn get_all_data(&self) -> Vec<&T> {
        self.current_layer.get_all_data()
    }

    /// Get data at a specific layer
    pub fn get_layer(&self, depth: usize) -> Option<&T> {
        self.current_layer.get_layer_data(depth)
    }

    /// Get the base (Violet-Ray) data
    pub fn get_base(&self) -> &T {
        self.current_layer.get_base_data()
    }

    /// Get the current (top) layer data
    pub fn get_current(&self) -> &T {
        &self.current_layer.transcended
    }

    /// Total depth of the hierarchy
    pub fn depth(&self) -> usize {
        self.current_layer.depth()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_name_index() {
        assert_eq!(LayerName::Violet.index(), 0);
        assert_eq!(LayerName::Indigo.index(), 1);
        assert_eq!(LayerName::Entity.index(), 7);
    }

    #[test]
    fn test_layer_base() {
        let layer: LayerReference<i32> = LayerReference::base(42);
        assert!(layer.is_base());
        assert_eq!(layer.layer_index, 0);
        assert_eq!(*layer.get_base_data(), 42);
    }

    #[test]
    fn test_layer_transcend() {
        let base = Arc::new(LayerReference::base(1));
        let next = LayerReference::transcend(base, 2);

        assert!(!next.is_base());
        assert_eq!(next.layer_index, 1);
        assert_eq!(next.layer_name, LayerName::Indigo);
        assert_eq!(*next.get_base_data(), 1);
        assert_eq!(next.transcended, 2);
    }

    #[test]
    fn test_layer_get_all_data() {
        let base = Arc::new(LayerReference::base(1));
        let middle = Arc::new(LayerReference::transcend(base, 2));
        let top = LayerReference::transcend(middle, 3);

        let all = top.get_all_data();
        assert_eq!(all.len(), 3);
        assert_eq!(*all[0], 3); // Top first
        assert_eq!(*all[1], 2);
        assert_eq!(*all[2], 1); // Base last
    }

    #[test]
    fn test_layer_get_specific() {
        let base = Arc::new(LayerReference::base(1));
        let middle = Arc::new(LayerReference::transcend(base, 2));
        let top = LayerReference::transcend(middle, 3);

        assert_eq!(top.get_layer_data(0), Some(&1));
        assert_eq!(top.get_layer_data(1), Some(&2));
        assert_eq!(top.get_layer_data(2), Some(&3));
        assert_eq!(top.get_layer_data(3), None);
    }

    #[test]
    fn test_layer_builder() {
        let layers = LayerBuilder::new(1)
            .add_layer(LayerName::Indigo, 2)
            .add_layer(LayerName::Blue, 3)
            .build();

        assert_eq!(layers.depth(), 3);
        assert_eq!(*layers.get_base_data(), 1);
        assert_eq!(layers.transcended, 3);
    }

    #[test]
    fn test_memory_efficiency() {
        // Demonstrate that Arc references share memory
        let base = Arc::new(LayerReference::base(vec![1, 2, 3]));

        // Create multiple layers all referencing the same base
        let layer1 = Arc::new(LayerReference::transcend(Arc::clone(&base), vec![4]));
        let layer2 = Arc::new(LayerReference::transcend(Arc::clone(&base), vec![5]));

        // Both layers reference the same base data (Arc)
        assert_eq!(layer1.included.as_ref().unwrap().transcended, vec![1, 2, 3]);
        assert_eq!(layer2.included.as_ref().unwrap().transcended, vec![1, 2, 3]);

        // But they have different transcended data
        assert_eq!(layer1.transcended, vec![4]);
        assert_eq!(layer2.transcended, vec![5]);
    }
}
