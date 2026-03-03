//! Holographic Field System
//!
//! This module implements the holographic field system that creates interference patterns
//! at different involution layers (Violet → Red) through spatial frequency reduction.
//!
//! # Core Concepts
//!
//! - **InvolutionLayer**: The 7 layers of the descent (Violet → Red), each with a specific spatial frequency
//! - **Spatial Frequency**: The primary mechanism (lines/mm), NOT eV values
//! - **Aperture Size**: Controls the resolution of the holographic encoding (0.0 to 1.0)
//! - **Effective Resolution**: spatial_frequency * aperture_size
//!
//! # The Descent Violet → Red
//!
//! The descent is NOT energy decrease, but SPATIAL FREQUENCY REDUCTION:
//! - Violet (5600 lines/mm): Highest spatial frequency, finest interference fringes, highest resolution
//! - Red (100 lines/mm): Lowest spatial frequency, coarsest fringes, lowest resolution
//!
//! Both contain the COMPLETE holographic encoding, but at different resolutions.
//! This is exactly like cutting a hologram - you lose resolution but maintain completeness.
//!
//! # Visible-Light-Spectrum as ANALOGIC
//!
//! Wavelength (nm) and eV values are SECONDARY, for visualization only.
//! They are returned as `Option<Float>` to indicate they are analogic, not primary.
//!
//! # Holographic Architecture
//!
//! "Each entity contains within it all densities and sub-densities of the octave"
//!
//! The holographic encoding at any layer contains the COMPLETE information about
//! all other layers. The difference is only in resolution (spatial frequency).

use crate::evolution_density_octave::density_octave::Density;
use crate::holographic::complex_vectors::{ComplexArchetype, ComplexVector};
use crate::holographic::field_address::HolographicAddress;
use crate::holographic::interference_pattern::InterferencePattern;
use crate::holographic::universal_template::{ArchetypeActivationProfile, SpectrumConfiguration};
use crate::holographic::Position;

/// Float type for holographic calculations
pub type Float = f64;

/// The 7 Involution Layers (Violet → Red descent)
///
/// Each layer represents a specific spatial frequency in the holographic architecture.
/// The descent Violet → Red is spatial frequency reduction, not energy decrease.
///
/// | Layer | Spatial Frequency | Analogic Wavelength | Analogic eV |
/// |-------|-------------------|---------------------|-------------|
/// | Violet | 5600 lines/mm | 400 nm | 3.11 eV |
/// | Indigo | 4000 lines/mm | 445 nm | 2.79 eV |
/// | Blue | 2800 lines/mm | 475 nm | 2.61 eV |
/// | Green | 2000 lines/mm | 510 nm | 2.43 eV |
/// | Yellow | 1400 lines/mm | 570 nm | 2.18 eV |
/// | Orange | 1000 lines/mm | 590 nm | 2.10 eV |
/// | Red | 100 lines/mm | 650 nm | 1.91 eV |
///
/// # Note on Analogic Values
///
/// Wavelength and eV values are ANALOGIC (secondary, for visualization only).
/// They are returned as `Option<Float>` to indicate they are not the primary mechanism.
/// The PRIMARY mechanism is spatial frequency (lines/mm).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InvolutionLayer {
    /// Violet Ray - Highest spatial frequency (5600 lines/mm)
    /// Finest interference fringes, highest resolution
    Violet,

    /// Indigo Ray (4000 lines/mm)
    Indigo,

    /// Blue Ray (2800 lines/mm)
    /// Law of Light - Where the interference pattern is impressed
    Blue,

    /// Green Ray (2000 lines/mm)
    /// The Bridge - Transition from Body to Spirit
    Green,

    /// Yellow Ray (1400 lines/mm)
    /// Mind Complex - First ray where mind becomes primary
    Yellow,

    /// Orange Ray (1000 lines/mm)
    /// Body Complex - Physical manifestation begins
    Orange,

    /// Red Ray - Lowest spatial frequency (100 lines/mm)
    /// Coarsest fringes, lowest resolution
    /// Start of the descent for entities
    Red,
}

impl InvolutionLayer {
    /// Returns the spatial frequency (lines per mm) for this layer.
    ///
    /// This is the PRIMARY mechanism for the holographic architecture.
    /// Spatial frequency determines the fineness of interference fringes.
    ///
    /// # Returns
    ///
    /// Spatial frequency in lines/mm (100.0 to 5600.0)
    pub fn spatial_frequency(&self) -> Float {
        match self {
            InvolutionLayer::Violet => 5600.0, // Highest frequency
            InvolutionLayer::Indigo => 4000.0,
            InvolutionLayer::Blue => 2800.0,
            InvolutionLayer::Green => 2000.0,
            InvolutionLayer::Yellow => 1400.0,
            InvolutionLayer::Orange => 1000.0,
            InvolutionLayer::Red => 100.0, // Lowest frequency
        }
    }

    /// Returns the resolution of this layer.
    ///
    /// Resolution is proportional to spatial frequency.
    /// Higher resolution = more information density.
    ///
    /// # Returns
    ///
    /// Resolution factor (1.0 to 56.0)
    pub fn resolution(&self) -> Float {
        self.spatial_frequency() / 100.0
    }

    /// Returns the analogic wavelength (nm) for this layer.
    ///
    /// # IMPORTANT
    ///
    /// This is ANALOGIC (secondary, for visualization only).
    /// The PRIMARY mechanism is spatial frequency.
    ///
    /// # Returns
    ///
    /// Wavelength in nanometers, or None if not applicable
    pub fn analogic_wavelength(&self) -> Option<Float> {
        match self {
            InvolutionLayer::Violet => Some(400.0), // nm
            InvolutionLayer::Indigo => Some(445.0),
            InvolutionLayer::Blue => Some(475.0),
            InvolutionLayer::Green => Some(510.0),
            InvolutionLayer::Yellow => Some(570.0),
            InvolutionLayer::Orange => Some(590.0),
            InvolutionLayer::Red => Some(650.0),
        }
    }

    /// Returns the analogic energy (eV) for this layer.
    ///
    /// # IMPORTANT
    ///
    /// This is ANALOGIC (secondary, for visualization only).
    /// The PRIMARY mechanism is spatial frequency.
    ///
    /// # Returns
    ///
    /// Energy in electron-volts, or None if not applicable
    pub fn analogic_ev(&self) -> Option<Float> {
        match self {
            InvolutionLayer::Violet => Some(3.11), // eV
            InvolutionLayer::Indigo => Some(2.79),
            InvolutionLayer::Blue => Some(2.61),
            InvolutionLayer::Green => Some(2.43),
            InvolutionLayer::Yellow => Some(2.18),
            InvolutionLayer::Orange => Some(2.10),
            InvolutionLayer::Red => Some(1.91),
        }
    }

    /// Returns the name of this layer.
    ///
    /// # Returns
    ///
    /// Layer name as a static string
    pub fn name(&self) -> &'static str {
        match self {
            InvolutionLayer::Violet => "Violet",
            InvolutionLayer::Indigo => "Indigo",
            InvolutionLayer::Blue => "Blue",
            InvolutionLayer::Green => "Green",
            InvolutionLayer::Yellow => "Yellow",
            InvolutionLayer::Orange => "Orange",
            InvolutionLayer::Red => "Red",
        }
    }

    /// Returns all involution layers in order from Violet to Red.
    ///
    /// # Returns
    ///
    /// Vector of all 7 layers in descending spatial frequency order
    pub fn all_layers() -> Vec<InvolutionLayer> {
        vec![
            InvolutionLayer::Violet,
            InvolutionLayer::Indigo,
            InvolutionLayer::Blue,
            InvolutionLayer::Green,
            InvolutionLayer::Yellow,
            InvolutionLayer::Orange,
            InvolutionLayer::Red,
        ]
    }

    /// Returns the next layer in the descent (Violet → Red).
    ///
    /// # Returns
    ///
    /// The next layer, or None if already at Red
    pub fn next_layer(&self) -> Option<InvolutionLayer> {
        match self {
            InvolutionLayer::Violet => Some(InvolutionLayer::Indigo),
            InvolutionLayer::Indigo => Some(InvolutionLayer::Blue),
            InvolutionLayer::Blue => Some(InvolutionLayer::Green),
            InvolutionLayer::Green => Some(InvolutionLayer::Yellow),
            InvolutionLayer::Yellow => Some(InvolutionLayer::Orange),
            InvolutionLayer::Orange => Some(InvolutionLayer::Red),
            InvolutionLayer::Red => None,
        }
    }

    /// Returns the previous layer in the ascent (Red → Violet).
    ///
    /// # Returns
    ///
    /// The previous layer, or None if already at Violet
    pub fn previous_layer(&self) -> Option<InvolutionLayer> {
        match self {
            InvolutionLayer::Violet => None,
            InvolutionLayer::Indigo => Some(InvolutionLayer::Violet),
            InvolutionLayer::Blue => Some(InvolutionLayer::Indigo),
            InvolutionLayer::Green => Some(InvolutionLayer::Blue),
            InvolutionLayer::Yellow => Some(InvolutionLayer::Green),
            InvolutionLayer::Orange => Some(InvolutionLayer::Yellow),
            InvolutionLayer::Red => Some(InvolutionLayer::Orange),
        }
    }
}

/// A holographic field at a specific involution layer.
///
/// The holographic field contains the COMPLETE encoding of the 22 archetypes
/// as an interference pattern. The difference between layers is only in
/// spatial frequency (resolution), not in content.
///
/// # Fields
///
/// - `layer`: Which involution layer this field represents
/// - `spatial_frequency`: Spatial frequency in lines/mm
/// - `interference_pattern`: The interference pattern from archetype interactions
/// - `archetype_complex_vectors`: The 22 archetype complex vectors (amplitude + phase)
/// - `aperture_size`: Viewing aperture (0.0 to 1.0) - controls effective resolution
///
/// # Holographic Principle
///
/// "Each entity contains within it all densities and sub-densities of the octave"
///
/// The holographic field at ANY layer contains the COMPLETE encoding of all other layers.
/// The difference is only in resolution (spatial frequency * aperture_size).
// ============================================================================
// PHASE 1: MERA CONFIGURATION AND TYPES
// ============================================================================

/// MERA Configuration for holographic compression (Phase 1)
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "MERA implements holographic compression through hierarchical layers
/// that progressively compress data while maintaining information."
#[derive(Debug, Clone)]
pub struct MeraConfig {
    /// Number of MERA hierarchy levels (typically 7 for density octave)
    pub num_levels: usize,
    /// Maximum cache size in megabytes for decompressed views
    pub cache_size_mb: usize,
    /// Compression threshold for wavelet coefficient pruning
    pub compression_threshold: Float,
}

impl Default for MeraConfig {
    fn default() -> Self {
        Self {
            num_levels: 7,
            cache_size_mb: 100,
            compression_threshold: 0.01,
        }
    }
}

impl MeraConfig {
    /// Create a new MERA configuration
    pub fn new(num_levels: usize, cache_size_mb: usize, compression_threshold: Float) -> Self {
        Self {
            num_levels,
            cache_size_mb,
            compression_threshold,
        }
    }
}

/// Result of field evolution (Phase 1)
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Field is primary reality"
#[derive(Debug, Clone, Default)]
pub struct FieldEvolutionResult {
    /// Change in phase coherence during evolution
    pub coherence_delta: Float,
    /// Number of stable constructive interference nodes after evolution
    pub stable_nodes_count: usize,
    /// Number of nodes that dissolved during evolution
    pub dissolved_nodes_count: usize,
    /// Change in aperture size (veil thinning)
    pub aperture_change: Float,
}

/// Potential entity extracted from field coherence (Phase 1)
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Entities manifest at stable interference nodes"
///
/// When the holographic field has high-coherence constructive interference
/// patterns, entity potentials can be extracted. These represent the
/// possibility of entity manifestation, not yet actualized.
#[derive(Debug, Clone)]
pub struct ExtractedEntityPotential {
    /// Holographic address in the field
    /// From Phase 1: Space emerges from field coherence, position is an address
    pub address: HolographicAddress,

    /// Archetype activation profile derived from local field values
    /// The 22 archetype coefficients represent the consciousness pattern
    pub archetype_activation: crate::holographic::universal_template::ArchetypeActivationProfile,

    /// Spectrum configuration derived from field ratio at location
    /// Space/Time ↔ Time/Space ratio determines the density of manifestation
    pub spectrum: crate::holographic::universal_template::SpectrumConfiguration,

    /// Density level determined by coherence level
    /// Higher coherence = higher density potential
    pub density: crate::evolution_density_octave::density_octave::Density,

    /// Local coherence at this manifestation point
    /// Must be above threshold for stable entity potential
    pub coherence: Float,

    /// Free Will seed for non-deterministic choice
    /// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will is the First Distortion"
    pub free_will_seed: u64,
}

impl Default for ExtractedEntityPotential {
    fn default() -> Self {
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        use crate::holographic::universal_template::{
            ArchetypeActivationProfile, SpectrumConfiguration,
        };

        Self {
            address: HolographicAddress::cosmic_origin(),
            archetype_activation: ArchetypeActivationProfile::default(),
            spectrum: SpectrumConfiguration::balanced(),
            density: Density::First(Density1SubLevel::Quantum),
            coherence: 0.5,
            free_will_seed: 0,
        }
    }
}
#[derive(Clone, Debug)]
pub struct HolographicField {
    /// The involution layer (Violet → Red)
    pub layer: InvolutionLayer,

    /// Spatial frequency in lines/mm (PRIMARY mechanism)
    pub spatial_frequency: Float,

    /// The interference pattern from archetype interactions
    pub interference_pattern: InterferencePattern,

    /// The 22 archetype complex vectors (amplitude + phase)
    pub archetype_complex_vectors: [ComplexVector; 22],

    /// Viewing aperture size (0.0 to 1.0)
    /// Controls the effective resolution of the field
    pub aperture_size: Float,
}

impl HolographicField {
    /// Creates a new holographic field for the specified layer.
    ///
    /// # Arguments
    ///
    /// * `layer` - The involution layer (Violet → Red)
    /// * `archetypes` - The 22 archetype complex vectors
    ///
    /// # Returns
    ///
    /// A new holographic field with default aperture size (0.1)
    pub fn new(layer: InvolutionLayer, archetypes: [ComplexArchetype; 22]) -> Self {
        let spatial_frequency = layer.spatial_frequency();

        // Convert archetypes to complex vectors
        let archetype_vectors: Vec<ComplexVector> =
            archetypes.iter().map(|a| a.to_complex_vector()).collect();
        let archetype_array: [ComplexVector; 22] = archetype_vectors.try_into().unwrap();

        // Create interference pattern
        let interference_pattern = InterferencePattern::from_archetypes(&archetype_array);

        HolographicField {
            layer,
            spatial_frequency,
            interference_pattern,
            archetype_complex_vectors: archetype_array,
            aperture_size: 0.1, // Default: small aperture (low resolution)
        }
    }

    /// Sets the aperture size for this field.
    ///
    /// The aperture size controls the effective resolution of the holographic encoding.
    /// Larger aperture = higher resolution = more information accessible.
    ///
    /// # Arguments
    ///
    /// * `size` - The aperture size (will be clamped to 0.0 to 1.0)
    pub fn set_aperture_size(&mut self, size: Float) {
        self.aperture_size = size.clamp(0.0, 1.0);
    }

    /// Returns the effective resolution of this field.
    ///
    /// Effective resolution = spatial_frequency * aperture_size
    ///
    /// This determines how much information from the holographic encoding
    /// is accessible. Higher resolution = more constructive nodes accessible.
    ///
    /// # Returns
    ///
    /// Effective resolution in lines/mm
    pub fn effective_resolution(&self) -> Float {
        self.spatial_frequency * self.aperture_size
    }

    /// Returns the accessible constructive interference nodes.
    ///
    /// Constructive nodes are stable configurations (bright fringes).
    /// Only nodes within the effective resolution are accessible.
    ///
    /// # Returns
    ///
    /// Vector of accessible constructive node positions
    pub fn accessible_constructive_nodes(&self) -> Vec<Position> {
        let resolution = self.effective_resolution();

        self.interference_pattern
            .constructive_nodes
            .iter()
            .filter(|node| self.is_node_accessible(node, resolution))
            .cloned()
            .collect()
    }

    /// Returns the accessible destructive interference nodes.
    ///
    /// Destructive nodes are unstable configurations (dark fringes).
    /// Only nodes within the effective resolution are accessible.
    ///
    /// # Returns
    ///
    /// Vector of accessible destructive node positions
    pub fn accessible_destructive_nodes(&self) -> Vec<Position> {
        let resolution = self.effective_resolution();

        self.interference_pattern
            .destructive_nodes
            .iter()
            .filter(|node| self.is_node_accessible(node, resolution))
            .cloned()
            .collect()
    }

    /// Checks if a node is accessible at the current resolution.
    ///
    /// # Arguments
    ///
    /// * `node` - The node position to check
    /// * `resolution` - The effective resolution
    ///
    /// # Returns
    ///
    /// True if the node is accessible, false otherwise
    fn is_node_accessible(&self, node: &Position, resolution: Float) -> bool {
        // Node is accessible if it's within current resolution
        node.spatial_frequency() <= resolution
    }

    /// Returns the phase coherence of the holographic encoding.
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

    /// Returns a summary of this field as a string.
    ///
    /// # Returns
    ///
    /// A human-readable summary of the field
    pub fn summary(&self) -> String {
        format!(
            "HolographicField @ {} ({} lines/mm, aperture {:.2}, effective resolution {:.2} lines/mm)\n\
             - Constructive nodes: {} (accessible: {})\n\
             - Destructive nodes: {} (accessible: {})\n\
             - Phase coherence: {:.3}",
            self.layer.name(),
            self.spatial_frequency,
            self.aperture_size,
            self.effective_resolution(),
            self.interference_pattern.constructive_nodes.len(),
            self.accessible_constructive_nodes().len(),
            self.interference_pattern.destructive_nodes.len(),
            self.accessible_destructive_nodes().len(),
            self.phase_coherence()
        )
    }

    /// Get grid resolution for MERA field compression (R&D-8)
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Grid resolution for multi-scale representation"
    ///
    /// # Returns
    ///
    /// Grid resolution (32 for default holographic field)
    pub fn grid_resolution(&self) -> usize {
        32
    }

    // ========================================================================
    // PHASE 1: ENTITY EXTRACTION METHODS
    // ========================================================================

    /// Calculate local intensity at a position in the field
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities manifest at stable interference nodes"
    pub fn calculate_local_intensity(&self, _position: &Position) -> Float {
        // Use the interference pattern's intensity field
        let intensity = self
            .interference_pattern
            .intensity_field
            .grid
            .iter()
            .sum::<Float>()
            / self.interference_pattern.intensity_field.grid.len().max(1) as Float;
        intensity.clamp(0.0, 1.0)
    }

    /// Derive archetype activation profile from local field values
    ///
    /// Phase 1: Entities emerge from the field with archetypical patterns
    pub fn derive_archetype_activation(&self, _position: &Position) -> ArchetypeActivationProfile {
        let mut coefficients = [0.5; 22];

        // Each archetype coefficient is influenced by the archetype vectors
        for i in 0..22 {
            let cv = &self.archetype_complex_vectors[i];
            coefficients[i] = cv.amplitude().clamp(0.0, 1.0);
        }

        ArchetypeActivationProfile::new(coefficients)
    }

    /// Extract entity potentials from the field
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities manifest at stable interference nodes"
    ///
    /// This method extracts potential entities from the holographic field
    /// based on coherence thresholds. Entities emerge FROM the field.
    pub fn extract_entities(&self, min_coherence: Float) -> Vec<ExtractedEntityPotential> {
        let mut potentials = Vec::new();
        let global_coherence = self.phase_coherence();

        // Get constructive interference nodes (stable configurations)
        let constructive_nodes = self.accessible_constructive_nodes();

        for (node_index, node) in constructive_nodes.iter().enumerate() {
            // Calculate local coherence at this node
            let local_intensity = self.calculate_local_intensity(node);
            let local_coherence = local_intensity * global_coherence;

            // Skip if below minimum coherence threshold
            if local_coherence < min_coherence {
                continue;
            }

            // Create holographic address from node position
            let address = self.position_to_address(node);

            // Derive archetype activation from local field values
            let archetype_activation = self.derive_archetype_activation(node);

            // Calculate spectrum configuration from field ratio at location
            let spectrum = self.derive_spectrum_configuration(node, local_coherence);

            // Determine density from coherence level
            let density = self.determine_density_from_coherence(local_coherence);

            // Generate free will seed from position hash and node index
            let free_will_seed = self.generate_free_will_seed(node, node_index);

            potentials.push(ExtractedEntityPotential {
                address,
                archetype_activation,
                spectrum,
                density,
                coherence: local_coherence,
                free_will_seed,
            });
        }

        potentials
    }

    /// Convert a Position to a HolographicAddress
    fn position_to_address(&self, position: &Position) -> HolographicAddress {
        use crate::holographic::field_address::ScaleLevel;
        use crate::holographic::field_address::Vector3;

        // Determine scale level based on spatial frequency
        let scale = self.spatial_frequency_to_scale_level();

        // Create local offset (normalized to [0, 1))
        let local_offset = Vector3::new(
            (position.x.fract() + 1.0).fract(),
            (position.y.fract() + 1.0).fract(),
            (position.z.fract() + 1.0).fract(),
        );

        HolographicAddress::new(scale, vec![], local_offset)
    }

    /// Map spatial frequency to scale level
    fn spatial_frequency_to_scale_level(&self) -> crate::holographic::field_address::ScaleLevel {
        use crate::holographic::field_address::ScaleLevel;
        match self.spatial_frequency {
            f if f >= 5000.0 => ScaleLevel::Quantum,
            f if f >= 3500.0 => ScaleLevel::Atomic,
            f if f >= 2500.0 => ScaleLevel::Molecular,
            f if f >= 1500.0 => ScaleLevel::Cellular,
            f if f >= 800.0 => ScaleLevel::Biological,
            f if f >= 300.0 => ScaleLevel::Planetary,
            f if f >= 150.0 => ScaleLevel::Stellar,
            _ => ScaleLevel::Cosmic,
        }
    }

    /// Derive spectrum configuration from field ratio at location
    fn derive_spectrum_configuration(
        &self,
        _position: &Position,
        local_coherence: Float,
    ) -> SpectrumConfiguration {
        // Calculate space/time ratio based on coherence
        // Higher coherence shifts toward time/space (oneness)
        let ratio_value = 1.0 + (1.0 - local_coherence) * 10.0;

        // Determine spectrum side based on coherence
        let ratio = if local_coherence > 0.5 {
            crate::spectrum::larson_framework::SpectrumRatio::time_space(1.0, ratio_value)
        } else {
            crate::spectrum::larson_framework::SpectrumRatio::space_time(ratio_value, 1.0)
        };

        // Veil transparency based on coherence
        let veil_transparency = local_coherence;

        // Access levels
        let space_time_access = 1.0 - local_coherence;
        let time_space_access = local_coherence;

        SpectrumConfiguration::new(
            ratio,
            veil_transparency,
            space_time_access,
            time_space_access,
        )
    }

    /// Determine density from coherence level
    fn determine_density_from_coherence(&self, coherence: Float) -> Density {
        use crate::evolution_density_octave::density_octave::{
            Density, Density1SubLevel, Density2SubLevel,
        };
        match coherence {
            c if c >= 0.95 => Density::Eighth,
            c if c >= 0.85 => Density::Seventh,
            c if c >= 0.75 => Density::Sixth,
            c if c >= 0.60 => Density::Fifth,
            c if c >= 0.45 => Density::Fourth,
            c if c >= 0.30 => Density::Third,
            c if c >= 0.15 => Density::Second(Density2SubLevel::Cellular),
            _ => Density::First(Density1SubLevel::Quantum),
        }
    }

    /// Generate free will seed from position and index
    fn generate_free_will_seed(&self, position: &Position, index: usize) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        position.x.to_bits().hash(&mut hasher);
        position.y.to_bits().hash(&mut hasher);
        position.z.to_bits().hash(&mut hasher);
        index.hash(&mut hasher);
        self.spatial_frequency.to_bits().hash(&mut hasher);

        hasher.finish()
    }

    /// Compute molecular interference pattern at a position
    ///
    /// From ROADMAP Phase 3.2: "Molecular Shape = Field interference patterns"
    pub fn compute_molecular_interference(
        &self,
        position: &Position,
    ) -> crate::chemistry::molecular_geometry::MolecularInterferencePattern {
        // Create a simple pattern for now
        crate::chemistry::molecular_geometry::MolecularInterferencePattern::new(
            18,
            36,
            crate::holographic::field_address::Vector3::new(position.x, position.y, position.z),
        )
    }
}

/// Generator for holographic fields at different involution layers.
///
/// The field generator creates holographic fields from archetype complex vectors
/// at any of the 7 involution layers (Violet → Red).
///
/// # Example
///
/// ```rust
/// use holographic_field::{HolographicFieldGenerator, InvolutionLayer};
/// use holographic::complex_vectors::ComplexArchetype;
///
/// let mut generator = HolographicFieldGenerator::new();
/// let archetypes = generate_test_archetypes();
/// generator.set_archetypes(archetypes);
///
/// // Generate a field at Violet layer
/// let violet_field = generator.generate_field(InvolutionLayer::Violet);
///
/// // Generate all fields
/// let all_fields = generator.generate_all_fields();
/// ```
#[derive(Clone, Debug)]
pub struct HolographicFieldGenerator {
    /// The base archetypes (complex vectors with amplitude and phase)
    pub base_archetypes: Option<[ComplexArchetype; 22]>,
}

impl HolographicFieldGenerator {
    /// Creates a new holographic field generator.
    ///
    /// # Returns
    ///
    /// A new field generator with no archetypes set
    pub fn new() -> Self {
        HolographicFieldGenerator {
            base_archetypes: None,
        }
    }

    /// Sets the base archetypes for the generator.
    ///
    /// # Arguments
    ///
    /// * `archetypes` - The 22 archetype complex vectors
    pub fn set_archetypes(&mut self, archetypes: [ComplexArchetype; 22]) {
        self.base_archetypes = Some(archetypes);
    }

    /// Generates a holographic field at the specified layer.
    ///
    /// # Arguments
    ///
    /// * `layer` - The involution layer (Violet → Red)
    ///
    /// # Returns
    ///
    /// A holographic field, or None if no archetypes are set
    pub fn generate_field(&self, layer: InvolutionLayer) -> Option<HolographicField> {
        let archetypes = self.base_archetypes.as_ref()?;
        Some(HolographicField::new(layer, archetypes.clone()))
    }

    /// Generates holographic fields for all 7 involution layers.
    ///
    /// # Returns
    ///
    /// Vector of holographic fields from Violet to Red, or empty if no archetypes are set
    pub fn generate_all_fields(&self) -> Vec<HolographicField> {
        let mut fields = Vec::new();

        let archetypes = match &self.base_archetypes {
            Some(a) => a.clone(),
            None => return fields,
        };

        for layer in InvolutionLayer::all_layers() {
            fields.push(HolographicField::new(layer, archetypes.clone()));
        }

        fields
    }

    /// Returns a summary of the generator state.
    ///
    /// # Returns
    ///
    /// A human-readable summary
    pub fn summary(&self) -> String {
        match &self.base_archetypes {
            Some(_) => {
                "HolographicFieldGenerator: Archetypes set, ready to generate fields".to_string()
            }
            None => "HolographicFieldGenerator: No archetypes set".to_string(),
        }
    }
}

impl Default for HolographicFieldGenerator {
    fn default() -> Self {
        Self::new()
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
    fn test_involution_layer_spatial_frequency() {
        assert_eq!(InvolutionLayer::Violet.spatial_frequency(), 5600.0);
        assert_eq!(InvolutionLayer::Indigo.spatial_frequency(), 4000.0);
        assert_eq!(InvolutionLayer::Blue.spatial_frequency(), 2800.0);
        assert_eq!(InvolutionLayer::Green.spatial_frequency(), 2000.0);
        assert_eq!(InvolutionLayer::Yellow.spatial_frequency(), 1400.0);
        assert_eq!(InvolutionLayer::Orange.spatial_frequency(), 1000.0);
        assert_eq!(InvolutionLayer::Red.spatial_frequency(), 100.0);
    }

    #[test]
    fn test_involution_layer_resolution() {
        assert_eq!(InvolutionLayer::Violet.resolution(), 56.0);
        assert_eq!(InvolutionLayer::Indigo.resolution(), 40.0);
        assert_eq!(InvolutionLayer::Blue.resolution(), 28.0);
        assert_eq!(InvolutionLayer::Green.resolution(), 20.0);
        assert_eq!(InvolutionLayer::Yellow.resolution(), 14.0);
        assert_eq!(InvolutionLayer::Orange.resolution(), 10.0);
        assert_eq!(InvolutionLayer::Red.resolution(), 1.0);
    }

    #[test]
    fn test_involution_layer_analogic_values() {
        // Test that analogic values are returned as Some
        assert!(InvolutionLayer::Violet.analogic_wavelength().is_some());
        assert!(InvolutionLayer::Violet.analogic_ev().is_some());

        // Test specific values
        assert_eq!(InvolutionLayer::Violet.analogic_wavelength(), Some(400.0));
        assert_eq!(InvolutionLayer::Violet.analogic_ev(), Some(3.11));

        assert_eq!(InvolutionLayer::Red.analogic_wavelength(), Some(650.0));
        assert_eq!(InvolutionLayer::Red.analogic_ev(), Some(1.91));
    }

    #[test]
    fn test_involution_layer_name() {
        assert_eq!(InvolutionLayer::Violet.name(), "Violet");
        assert_eq!(InvolutionLayer::Red.name(), "Red");
    }

    #[test]
    fn test_involution_layer_all_layers() {
        let layers = InvolutionLayer::all_layers();
        assert_eq!(layers.len(), 7);
        assert_eq!(layers[0], InvolutionLayer::Violet);
        assert_eq!(layers[6], InvolutionLayer::Red);
    }

    #[test]
    fn test_involution_layer_navigation() {
        assert_eq!(
            InvolutionLayer::Violet.next_layer(),
            Some(InvolutionLayer::Indigo)
        );
        assert_eq!(InvolutionLayer::Red.next_layer(), None);

        assert_eq!(
            InvolutionLayer::Red.previous_layer(),
            Some(InvolutionLayer::Orange)
        );
        assert_eq!(InvolutionLayer::Violet.previous_layer(), None);
    }

    #[test]
    fn test_holographic_field_creation() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        assert_eq!(field.layer, InvolutionLayer::Violet);
        assert_eq!(field.spatial_frequency, 5600.0);
        assert_eq!(field.aperture_size, 0.1);
        assert_eq!(field.archetype_complex_vectors.len(), 22);
    }

    #[test]
    fn test_holographic_field_aperture_size_management() {
        let archetypes = generate_test_archetypes();
        let mut field = HolographicField::new(InvolutionLayer::Red, archetypes);

        field.set_aperture_size(0.5);
        assert_eq!(field.aperture_size, 0.5);

        field.set_aperture_size(1.5); // Should be clamped to 1.0
        assert_eq!(field.aperture_size, 1.0);

        field.set_aperture_size(-0.5); // Should be clamped to 0.0
        assert_eq!(field.aperture_size, 0.0);
    }

    #[test]
    fn test_holographic_field_effective_resolution() {
        let archetypes = generate_test_archetypes();
        let mut field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        // With default aperture (0.1)
        assert_eq!(field.effective_resolution(), 560.0);

        // With larger aperture (0.5)
        field.set_aperture_size(0.5);
        assert_eq!(field.effective_resolution(), 2800.0);

        // With maximum aperture (1.0)
        field.set_aperture_size(1.0);
        assert_eq!(field.effective_resolution(), 5600.0);
    }

    #[test]
    fn test_holographic_field_generator_creation() {
        let generator = HolographicFieldGenerator::new();
        assert!(generator.base_archetypes.is_none());
    }

    #[test]
    fn test_holographic_field_generator_set_archetypes() {
        let mut generator = HolographicFieldGenerator::new();
        let archetypes = generate_test_archetypes();
        generator.set_archetypes(archetypes);
        assert!(generator.base_archetypes.is_some());
    }

    #[test]
    fn test_holographic_field_generator_generate_field() {
        let mut generator = HolographicFieldGenerator::new();
        let archetypes = generate_test_archetypes();
        generator.set_archetypes(archetypes);

        let violet_field = generator.generate_field(InvolutionLayer::Violet);
        assert!(violet_field.is_some());

        let field = violet_field.unwrap();
        assert_eq!(field.layer, InvolutionLayer::Violet);
        assert_eq!(field.spatial_frequency, 5600.0);
    }

    #[test]
    fn test_holographic_field_generator_generate_field_no_archetypes() {
        let generator = HolographicFieldGenerator::new();
        let violet_field = generator.generate_field(InvolutionLayer::Violet);
        assert!(violet_field.is_none());
    }

    #[test]
    fn test_holographic_field_generator_generate_all_fields() {
        let mut generator = HolographicFieldGenerator::new();
        let archetypes = generate_test_archetypes();
        generator.set_archetypes(archetypes);

        let fields = generator.generate_all_fields();
        assert_eq!(fields.len(), 7);

        // Check that all layers are present
        assert_eq!(fields[0].layer, InvolutionLayer::Violet);
        assert_eq!(fields[1].layer, InvolutionLayer::Indigo);
        assert_eq!(fields[2].layer, InvolutionLayer::Blue);
        assert_eq!(fields[3].layer, InvolutionLayer::Green);
        assert_eq!(fields[4].layer, InvolutionLayer::Yellow);
        assert_eq!(fields[5].layer, InvolutionLayer::Orange);
        assert_eq!(fields[6].layer, InvolutionLayer::Red);
    }

    #[test]
    fn test_holographic_field_generator_generate_all_fields_no_archetypes() {
        let generator = HolographicFieldGenerator::new();
        let fields = generator.generate_all_fields();
        assert_eq!(fields.len(), 0);
    }

    #[test]
    fn test_spatial_frequency_gradient() {
        let mut generator = HolographicFieldGenerator::new();
        let archetypes = generate_test_archetypes();
        generator.set_archetypes(archetypes);

        let fields = generator.generate_all_fields();

        // Check that spatial frequency decreases from Violet to Red
        assert!(fields[0].spatial_frequency > fields[1].spatial_frequency);
        assert!(fields[1].spatial_frequency > fields[2].spatial_frequency);
        assert!(fields[2].spatial_frequency > fields[3].spatial_frequency);
        assert!(fields[3].spatial_frequency > fields[4].spatial_frequency);
        assert!(fields[4].spatial_frequency > fields[5].spatial_frequency);
        assert!(fields[5].spatial_frequency > fields[6].spatial_frequency);

        // Check specific values
        assert_eq!(fields[0].spatial_frequency, 5600.0); // Violet
        assert_eq!(fields[6].spatial_frequency, 100.0); // Red
    }

    #[test]
    fn test_holographic_field_phase_coherence() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        // Phase coherence should be between 0.0 and 1.0
        let coherence = field.phase_coherence();
        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_holographic_field_summary() {
        let archetypes = generate_test_archetypes();
        let field = HolographicField::new(InvolutionLayer::Violet, archetypes);

        let summary = field.summary();
        assert!(summary.contains("Violet"));
        assert!(summary.contains("5600 lines/mm"));
    }

    #[test]
    fn test_holographic_field_generator_summary() {
        let mut generator = HolographicFieldGenerator::new();

        let summary = generator.summary();
        assert!(summary.contains("No archetypes set"));

        let archetypes = generate_test_archetypes();
        generator.set_archetypes(archetypes);

        let summary = generator.summary();
        assert!(summary.contains("Archetypes set"));
    }

    #[test]
    fn test_holographic_field_default() {
        let generator = HolographicFieldGenerator::default();
        assert!(generator.base_archetypes.is_none());
    }
}
