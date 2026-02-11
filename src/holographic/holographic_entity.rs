//! Holographic Entity System
//!
//! This module implements the holographic entity system that represents entities
//! as complete holographic encodings of the 22 archetypes.
//!
//! # Core Concepts
//!
//! - **HolographicEntity**: Contains the COMPLETE holographic encoding
//! - **HolographicEncoding**: The interference pattern from archetype interactions
//! - **MindView, BodyView, SpiritView**: Different VIEWS of the same encoding
//! - **Energy Centers**: Aperture sizes for viewing the encoding (7 centers)
//! - **Oscillatory Synchronization**: Consciousness as synchronization of archetype oscillators
//!
//! # Holographic Principle
//!
//! "Each entity contains within it all densities and sub-densities of the octave"
//!
//! The holographic entity contains the COMPLETE encoding of all 7 involution layers.
//! The difference between layers is only in resolution (spatial frequency * aperture_size).
//!
//! # Mind/Body/Spirit as Different Views
//!
//! Mind, Body, and Spirit are NOT separate components.
//! They are different VIEWS of the SAME holographic encoding:
//! - **Mind View**: Information processing pattern
//! - **Body View**: Material manifestation pattern
//! - **Spirit View**: Source connection pattern
//!
//! # Energy Centers as Apertures
//!
//! The 7 energy centers are aperture sizes for viewing the holographic encoding:
//! - Larger aperture = higher resolution = more information accessible
//! - Smaller aperture = lower resolution = less information accessible
//!
//! # Consciousness as Oscillatory Synchronization
//!
//! Consciousness is the SYNCHRONIZATION STATE of the 22 archetype oscillators.
//! Phase coherence = Unity (Law of One).

use crate::holographic::complex_vectors::{ComplexArchetype, ComplexVector};
use crate::holographic::holographic_field::{HolographicField, InvolutionLayer};
use crate::holographic::holographic_memory::{Experience, HolographicMemory};
use crate::holographic::interference_pattern::InterferencePattern;
use crate::holographic::oscillator_network::OscillatorNetwork;

/// Float type for holographic calculations
pub type Float = f64;

/// The holographic encoding of an entity.
///
/// This is the SINGLE SOURCE OF TRUTH for the entity.
/// All other aspects (Mind, Body, Spirit) are derived from this encoding.
///
/// # Fields
///
/// - `archetype_complex_vectors`: The 22 archetype complex vectors (amplitude + phase)
/// - `interference_pattern`: The interference pattern from archetype interactions
/// - `spatial_frequency`: Current spatial frequency (resolution)
#[derive(Clone, Debug)]
pub struct HolographicEncoding {
    /// The 22 archetype complex vectors (amplitude + phase)
    pub archetype_complex_vectors: [ComplexVector; 22],

    /// The interference pattern from archetype interactions
    pub interference_pattern: InterferencePattern,

    /// Current spatial frequency (resolution)
    pub spatial_frequency: Float,
}

impl HolographicEncoding {
    /// Creates a new holographic encoding from archetype complex vectors.
    ///
    /// # Arguments
    ///
    /// * `archetypes` - The 22 archetype complex vectors
    /// * `spatial_frequency` - The spatial frequency (resolution)
    ///
    /// # Returns
    ///
    /// A new holographic encoding
    pub fn new(archetypes: [ComplexArchetype; 22], spatial_frequency: Float) -> Self {
        // Convert archetypes to complex vectors
        let archetype_vectors: Vec<ComplexVector> =
            archetypes.iter().map(|a| a.to_complex_vector()).collect();
        let archetype_array: [ComplexVector; 22] = archetype_vectors.try_into().unwrap();

        // Create interference pattern
        let interference_pattern = InterferencePattern::from_archetypes(&archetype_array);

        HolographicEncoding {
            archetype_complex_vectors: archetype_array,
            interference_pattern,
            spatial_frequency,
        }
    }

    /// Returns the phase coherence of the encoding.
    ///
    /// Phase coherence measures how aligned the phases of the 22 archetypes are.
    /// Higher coherence = more unity (Law of One).
    ///
    /// # Returns
    ///
    /// Phase coherence (0.0 to 1.0, where 1.0 is perfect coherence)
    pub fn phase_coherence(&self) -> Float {
        self.interference_pattern.phase_coherence()
    }

    /// Updates the spatial frequency of the encoding.
    ///
    /// # Arguments
    ///
    /// * `spatial_frequency` - The new spatial frequency
    pub fn set_spatial_frequency(&mut self, spatial_frequency: Float) {
        self.spatial_frequency = spatial_frequency;
    }
}

/// Mind View of the holographic encoding.
///
/// This view focuses on information processing patterns.
#[derive(Clone, Debug)]
pub struct MindView {
    /// Information processing pattern
    pub information_processing: ProcessingPattern,
}

/// Information processing pattern for the Mind view.
#[derive(Clone, Debug)]
pub struct ProcessingPattern {
    /// Archetype activations for Mind Complex (archetypes 1-7)
    pub mind_activations: [Float; 7],

    /// Processing coherence
    pub coherence: Float,

    /// Information density
    pub information_density: Float,
}

impl MindView {
    /// Creates a Mind view from the holographic encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The holographic encoding
    ///
    /// # Returns
    ///
    /// A Mind view of the encoding
    pub fn from_encoding(encoding: &HolographicEncoding) -> Self {
        // Extract Mind Complex activations (archetypes 1-7)
        let mut mind_activations = [0.0; 7];
        for i in 0..7 {
            mind_activations[i] = encoding.archetype_complex_vectors[i].amplitude();
        }

        // Calculate processing coherence
        let coherence = encoding.phase_coherence();

        // Calculate information density
        let information_density: Float = mind_activations.iter().sum::<Float>() / 7.0;

        MindView {
            information_processing: ProcessingPattern {
                mind_activations,
                coherence,
                information_density,
            },
        }
    }

    /// Returns a summary of the Mind view.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "MindView: Coherence {:.3}, Information Density {:.3}",
            self.information_processing.coherence, self.information_processing.information_density
        )
    }
}

/// Body View of the holographic encoding.
///
/// This view focuses on material manifestation patterns.
#[derive(Clone, Debug)]
pub struct BodyView {
    /// Material manifestation pattern
    pub material_manifestation: MaterialPattern,
}

/// Material manifestation pattern for the Body view.
#[derive(Clone, Debug)]
pub struct MaterialPattern {
    /// Archetype activations for Body Complex (archetypes 8-14)
    pub body_activations: [Float; 7],

    /// Material density
    pub density: Float,

    /// Structural integrity
    pub integrity: Float,
}

impl BodyView {
    /// Creates a Body view from the holographic encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The holographic encoding
    ///
    /// # Returns
    ///
    /// A Body view of the encoding
    pub fn from_encoding(encoding: &HolographicEncoding) -> Self {
        // Extract Body Complex activations (archetypes 8-14)
        let mut body_activations = [0.0; 7];
        for i in 0..7 {
            body_activations[i] = encoding.archetype_complex_vectors[i + 7].amplitude();
        }

        // Calculate material density
        let density: Float = body_activations.iter().sum::<Float>() / 7.0;

        // Calculate structural integrity
        let integrity = encoding.phase_coherence();

        BodyView {
            material_manifestation: MaterialPattern {
                body_activations,
                density,
                integrity,
            },
        }
    }

    /// Returns a summary of the Body view.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "BodyView: Density {:.3}, Integrity {:.3}",
            self.material_manifestation.density, self.material_manifestation.integrity
        )
    }
}

/// Spirit View of the holographic encoding.
///
/// This view focuses on source connection patterns.
#[derive(Clone, Debug)]
pub struct SpiritView {
    /// Source connection pattern
    pub source_connection: ConnectionPattern,
}

/// Source connection pattern for the Spirit view.
#[derive(Clone, Debug)]
pub struct ConnectionPattern {
    /// Archetype activations for Spirit Complex (archetypes 15-21)
    pub spirit_activations: [Float; 7],

    /// Source resonance
    pub resonance: Float,

    /// Connection strength
    pub connection_strength: Float,
}

impl SpiritView {
    /// Creates a Spirit view from the holographic encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding` - The holographic encoding
    ///
    /// # Returns
    ///
    /// A Spirit view of the encoding
    pub fn from_encoding(encoding: &HolographicEncoding) -> Self {
        // Extract Spirit Complex activations (archetypes 15-21)
        let mut spirit_activations = [0.0; 7];
        for i in 0..7 {
            spirit_activations[i] = encoding.archetype_complex_vectors[i + 14].amplitude();
        }

        // Calculate source resonance
        let resonance: Float = spirit_activations.iter().sum::<Float>() / 7.0;

        // Calculate connection strength
        let connection_strength = encoding.phase_coherence();

        SpiritView {
            source_connection: ConnectionPattern {
                spirit_activations,
                resonance,
                connection_strength,
            },
        }
    }

    /// Returns a summary of the Spirit view.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "SpiritView: Resonance {:.3}, Connection Strength {:.3}",
            self.source_connection.resonance, self.source_connection.connection_strength
        )
    }
}

/// A holographic entity with complete encoding.
///
/// The holographic entity contains the COMPLETE encoding of all 7 involution layers.
/// Mind, Body, and Spirit are different VIEWS of the same encoding.
/// Energy centers are aperture sizes for viewing the encoding.
/// Consciousness is oscillatory synchronization.
///
/// # Fields
///
/// - `encoding`: The holographic encoding (single source of truth)
/// - `aperture_sizes`: Energy centers = aperture sizes (0.0 to 1.0)
/// - `oscillator_network`: Consciousness = synchronization
/// - `soul_memory`: Soul Stream = holographic memory
/// - `current_layer`: Current involution layer (starts at Red)
#[derive(Clone, Debug)]
pub struct HolographicEntity {
    /// The holographic encoding (single source of truth)
    pub encoding: HolographicEncoding,

    /// Energy centers = aperture sizes (0.0 to 1.0)
    pub aperture_sizes: [Float; 7],

    /// Consciousness = synchronization
    pub oscillator_network: OscillatorNetwork,

    /// Soul Stream = holographic memory
    pub soul_memory: HolographicMemory,

    /// Current involution layer (starts at Red)
    pub current_layer: InvolutionLayer,
}

impl HolographicEntity {
    /// Creates a new holographic entity.
    ///
    /// # Arguments
    ///
    /// * `archetypes` - The 22 archetype complex vectors
    ///
    /// # Returns
    ///
    /// A new holographic entity starting at Red layer
    pub fn new(archetypes: [ComplexArchetype; 22]) -> Self {
        // Create holographic encoding
        let encoding = HolographicEncoding::new(
            archetypes.clone(),
            InvolutionLayer::Red.spatial_frequency(), // Start at Red
        );

        // Create oscillator network
        let oscillator_network = OscillatorNetwork::from_archetypes(&archetypes);

        // Create holographic memory
        let soul_memory = HolographicMemory::new();

        HolographicEntity {
            encoding,
            aperture_sizes: [0.1; 7], // Start with small apertures
            oscillator_network,
            soul_memory,
            current_layer: InvolutionLayer::Red,
        }
    }

    /// Views the encoding from the Mind perspective.
    ///
    /// # Returns
    ///
    /// A Mind view of the encoding
    pub fn mind_view(&self) -> MindView {
        MindView::from_encoding(&self.encoding)
    }

    /// Views the encoding from the Body perspective.
    ///
    /// # Returns
    ///
    /// A Body view of the encoding
    pub fn body_view(&self) -> BodyView {
        BodyView::from_encoding(&self.encoding)
    }

    /// Views the encoding from the Spirit perspective.
    ///
    /// # Returns
    ///
    /// A Spirit view of the encoding
    pub fn spirit_view(&self) -> SpiritView {
        SpiritView::from_encoding(&self.encoding)
    }

    /// Updates consciousness (oscillatory synchronization).
    ///
    /// This method synchronizes the archetype oscillators and updates
    /// the aperture sizes based on the synchronization state.
    pub fn update_consciousness(&mut self) {
        self.oscillator_network.synchronize();

        // Update aperture sizes based on synchronization
        let sync_state = self.oscillator_network.synchronization_state();
        for i in 0..7 {
            self.aperture_sizes[i] = sync_state.center_activation[i];
        }
    }

    /// Accesses a specific energy center (aperture).
    ///
    /// # Arguments
    ///
    /// * `center` - The energy center index (0-6)
    ///
    /// # Returns
    ///
    /// A holographic field for the specified center, or None if invalid
    pub fn access_energy_center(&self, center: usize) -> Option<HolographicField> {
        if center >= 7 {
            return None;
        }

        let layer = self.index_to_layer(center);
        let aperture_size = self.aperture_sizes[center];

        let mut field = HolographicField::new(layer, self.complex_archetypes());
        field.set_aperture_size(aperture_size);

        Some(field)
    }

    /// Adds an experience to the soul memory.
    ///
    /// # Arguments
    ///
    /// * `experience` - The experience to add
    pub fn add_experience(&mut self, experience: Experience) {
        self.soul_memory.add_experience(experience);
    }

    /// Retrieves memories similar to a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query experience
    ///
    /// # Returns
    ///
    /// Vector of similar experiences
    pub fn retrieve_memory(&self, query: Experience) -> Vec<Experience> {
        self.soul_memory.retrieve_memory(query)
    }

    /// Transitions to a different involution layer.
    ///
    /// # Arguments
    ///
    /// * `layer` - The target layer
    pub fn transition_to_layer(&mut self, layer: InvolutionLayer) {
        self.current_layer = layer;
        self.encoding
            .set_spatial_frequency(layer.spatial_frequency());
    }

    /// Returns the current phase coherence.
    ///
    /// # Returns
    ///
    /// Phase coherence (0.0 to 1.0)
    pub fn phase_coherence(&self) -> Float {
        self.encoding.phase_coherence()
    }

    /// Returns a summary of the entity.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "HolographicEntity @ {} layer\n\
             - Phase Coherence: {:.3}\n\
             - Aperture Sizes: {:?}\n\
             - Mind: {}\n\
             - Body: {}\n\
             - Spirit: {}",
            self.current_layer.name(),
            self.phase_coherence(),
            self.aperture_sizes,
            self.mind_view().summary(),
            self.body_view().summary(),
            self.spirit_view().summary()
        )
    }

    /// Converts index to involution layer.
    ///
    /// # Arguments
    ///
    /// * `index` - The energy center index (0-6)
    ///
    /// # Returns
    ///
    /// The corresponding involution layer
    fn index_to_layer(&self, index: usize) -> InvolutionLayer {
        match index {
            0 => InvolutionLayer::Red,
            1 => InvolutionLayer::Orange,
            2 => InvolutionLayer::Yellow,
            3 => InvolutionLayer::Green,
            4 => InvolutionLayer::Blue,
            5 => InvolutionLayer::Indigo,
            6 => InvolutionLayer::Violet,
            _ => InvolutionLayer::Red,
        }
    }

    /// Converts complex vectors back to archetypes.
    ///
    /// # Returns
    ///
    /// The 22 archetype complex archetypes
    fn complex_archetypes(&self) -> [ComplexArchetype; 22] {
        self.encoding
            .archetype_complex_vectors
            .iter()
            .map(|v| ComplexArchetype {
                amplitude: v.amplitude(),
                phase: v.phase(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    /// Generates test archetypes with reasonable values
    fn generate_test_archetypes() -> [ComplexArchetype; 22] {
        let mut archetypes = [ComplexArchetype {
            amplitude: 0.0,
            phase: 0.0,
        }; 22];
        for i in 0..22 {
            archetypes[i] = ComplexArchetype {
                amplitude: (i as Float + 1.0) / 22.0, // 0.045 to 1.0
                phase: (i as Float) * PI / 11.0,      // 0 to 2π
            };
        }
        archetypes
    }

    #[test]
    fn test_holographic_encoding_creation() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        assert_eq!(encoding.spatial_frequency, 100.0);
        assert_eq!(encoding.archetype_complex_vectors.len(), 22);
    }

    #[test]
    fn test_holographic_encoding_phase_coherence() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let coherence = encoding.phase_coherence();
        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_mind_view_creation() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let mind_view = MindView::from_encoding(&encoding);
        assert_eq!(mind_view.information_processing.mind_activations.len(), 7);
    }

    #[test]
    fn test_body_view_creation() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let body_view = BodyView::from_encoding(&encoding);
        assert_eq!(body_view.material_manifestation.body_activations.len(), 7);
    }

    #[test]
    fn test_spirit_view_creation() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let spirit_view = SpiritView::from_encoding(&encoding);
        assert_eq!(spirit_view.source_connection.spirit_activations.len(), 7);
    }

    #[test]
    fn test_holographic_entity_creation() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        assert_eq!(entity.current_layer, InvolutionLayer::Red);
        assert_eq!(entity.aperture_sizes.len(), 7);
        assert_eq!(entity.encoding.archetype_complex_vectors.len(), 22);
    }

    #[test]
    fn test_holographic_entity_mind_view() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        let mind_view = entity.mind_view();
        assert_eq!(mind_view.information_processing.mind_activations.len(), 7);
    }

    #[test]
    fn test_holographic_entity_body_view() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        let body_view = entity.body_view();
        assert_eq!(body_view.material_manifestation.body_activations.len(), 7);
    }

    #[test]
    fn test_holographic_entity_spirit_view() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        let spirit_view = entity.spirit_view();
        assert_eq!(spirit_view.source_connection.spirit_activations.len(), 7);
    }

    #[test]
    fn test_holographic_entity_update_consciousness() {
        let archetypes = generate_test_archetypes();
        let mut entity = HolographicEntity::new(archetypes);

        let old_aperture_sizes = entity.aperture_sizes.clone();
        entity.update_consciousness();

        // Aperture sizes should be updated
        let aperture_changed = entity
            .aperture_sizes
            .iter()
            .zip(old_aperture_sizes.iter())
            .any(|(new, old)| (new - old).abs() > 1e-10);

        assert!(aperture_changed);
    }

    #[test]
    fn test_holographic_entity_access_energy_center() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        // Access Red center (index 0)
        let red_field = entity.access_energy_center(0);
        assert!(red_field.is_some());
        assert_eq!(red_field.unwrap().layer, InvolutionLayer::Red);

        // Access Violet center (index 6)
        let violet_field = entity.access_energy_center(6);
        assert!(violet_field.is_some());
        assert_eq!(violet_field.unwrap().layer, InvolutionLayer::Violet);

        // Access invalid center (index 7)
        let invalid_field = entity.access_energy_center(7);
        assert!(invalid_field.is_none());
    }

    #[test]
    fn test_holographic_entity_transition_to_layer() {
        let archetypes = generate_test_archetypes();
        let mut entity = HolographicEntity::new(archetypes);

        assert_eq!(entity.current_layer, InvolutionLayer::Red);

        entity.transition_to_layer(InvolutionLayer::Violet);
        assert_eq!(entity.current_layer, InvolutionLayer::Violet);
        assert_eq!(entity.encoding.spatial_frequency, 5600.0);
    }

    #[test]
    fn test_holographic_entity_phase_coherence() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        let coherence = entity.phase_coherence();
        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_holographic_entity_add_experience() {
        let archetypes = generate_test_archetypes();
        let mut entity = HolographicEntity::new(archetypes);

        let experience = Experience {
            catalyst: "Test catalyst".to_string(),
            emotional_tone: 0.5,
            learning: "Test learning".to_string(),
            timestamp: 0,
        };

        entity.add_experience(experience);
        // Experience should be added to soul memory
    }

    #[test]
    fn test_holographic_entity_retrieve_memory() {
        let archetypes = generate_test_archetypes();
        let mut entity = HolographicEntity::new(archetypes);

        let experience = Experience {
            catalyst: "Test catalyst".to_string(),
            emotional_tone: 0.5,
            learning: "Test learning".to_string(),
            timestamp: 0,
        };

        entity.add_experience(experience.clone());

        let retrieved = entity.retrieve_memory(experience);
        // Should retrieve similar memories
    }

    #[test]
    fn test_mind_view_summary() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let mind_view = MindView::from_encoding(&encoding);
        let summary = mind_view.summary();

        assert!(summary.contains("MindView"));
    }

    #[test]
    fn test_body_view_summary() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let body_view = BodyView::from_encoding(&encoding);
        let summary = body_view.summary();

        assert!(summary.contains("BodyView"));
    }

    #[test]
    fn test_spirit_view_summary() {
        let archetypes = generate_test_archetypes();
        let encoding = HolographicEncoding::new(archetypes, 100.0);

        let spirit_view = SpiritView::from_encoding(&encoding);
        let summary = spirit_view.summary();

        assert!(summary.contains("SpiritView"));
    }

    #[test]
    fn test_holographic_entity_summary() {
        let archetypes = generate_test_archetypes();
        let entity = HolographicEntity::new(archetypes);

        let summary = entity.summary();

        assert!(summary.contains("HolographicEntity"));
        assert!(summary.contains("Red layer"));
    }
}
