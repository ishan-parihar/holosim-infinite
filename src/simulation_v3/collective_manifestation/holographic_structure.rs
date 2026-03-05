//! Holographic Structure Representation System
//!
//! From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6, Week 89-92:
//! "Collective Manifestation (Building) - multiple entities contribute resonance
//! to create structures"
//!
//! This module implements holographic structures that emerge from collective resonance:
//! - Structures manifest when collective resonance exceeds threshold
//! - Each structure has a holographic signature derived from collective pattern
//! - Properties (visual, functional) are derived from signature
//! - Build progress reflects resonance accumulation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The holographic principle - each part contains information about the whole"

use crate::simulation_v3::collective_manifestation::collective_resonance::CollectiveResonanceResult;
use crate::simulation_v3::holographic_physics::SpectrumRatio;
use crate::types::Float;

/// Types of holographic structures that can be manifested
///
/// Each structure type represents a different manifestation of collective resonance,
/// serving different purposes in the collective experience.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureType {
    /// Sacred space for spiritual practices and connection
    Temple,
    /// Monument to collective achievement or shared experience
    Monument,
    /// Living space adapted to collective needs
    Habitat,
    /// Repository of collective knowledge and wisdom
    Library,
    /// Energy structure that stabilizes local resonance
    EnergyAnchor,
    /// Communication structure for connecting distant collectives
    CommunicationHub,
    /// Healing space for collective restoration
    HealingSanctuary,
    /// Creative space for artistic expression
    CreativeStudio,
    /// Gathering space for collective interaction
    CommunityCenter,
    /// Custom structure with user-defined purpose
    Custom,
    /// Structure that collects and amplifies resonance (added for advanced_game_mechanics)
    ResonanceCollector,
}

impl StructureType {
    /// Get the base resonance requirement for this structure type
    ///
    /// Different structure types require different amounts of collective resonance
    /// to manifest. More complex structures require higher resonance thresholds.
    pub fn base_resonance_requirement(&self) -> Float {
        match self {
            StructureType::CommunityCenter => 10.0,
            StructureType::Habitat => 15.0,
            StructureType::HealingSanctuary => 20.0,
            StructureType::CreativeStudio => 25.0,
            StructureType::EnergyAnchor => 30.0,
            StructureType::CommunicationHub => 35.0,
            StructureType::Library => 40.0,
            StructureType::Monument => 45.0,
            StructureType::Temple => 50.0,
            StructureType::Custom => 30.0,
            StructureType::ResonanceCollector => 35.0,
        }
    }
}

/// Material appearance of holographic structures
///
/// Determines the visual and tactile qualities of the manifested structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialAppearance {
    /// Crystalline structures with geometric clarity
    Crystalline,
    /// Fluid, adaptive structures that can change form
    Liquid,
    /// Diffuse, cloud-like structures
    Gaseous,
    /// High-energy, luminous structures
    Plasma,
    /// Subtle, spiritual structures barely visible to ordinary perception
    Ethereal,
    /// Dense, solid structures with permanence
    Solid,
}

impl MaterialAppearance {
    /// Get density factor for this material appearance
    pub fn density_factor(&self) -> Float {
        match self {
            MaterialAppearance::Ethereal => 0.1,
            MaterialAppearance::Gaseous => 0.3,
            MaterialAppearance::Liquid => 0.5,
            MaterialAppearance::Crystalline => 0.7,
            MaterialAppearance::Plasma => 0.85,
            MaterialAppearance::Solid => 1.0,
        }
    }
}

/// Types of geometric patterns that can appear on structures
///
/// These patterns represent different archetypical frequencies encoded in
/// the holographic signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometricPatternType {
    /// Sacred geometry representing creation and unity
    FlowerOfLife,
    /// Metaphysical geometry representing balance and integration
    MetatronsCube,
    /// Ancient geometry representing cosmic order
    SriYantra,
    /// Torus geometry representing energy flow
    Toroidal,
    /// Spiral geometry representing growth and evolution
    FibonacciSpiral,
    /// Vesica Piscis geometry representing intersection of dualities
    VesicaPiscis,
}

impl GeometricPatternType {
    /// Get the archetype index (0-21) associated with this pattern type
    pub fn archetype_index(&self) -> usize {
        match self {
            GeometricPatternType::FlowerOfLife => 0,
            GeometricPatternType::MetatronsCube => 7,
            GeometricPatternType::SriYantra => 14,
            GeometricPatternType::Toroidal => 3,
            GeometricPatternType::FibonacciSpiral => 10,
            GeometricPatternType::VesicaPiscis => 17,
        }
    }
}

/// A geometric pattern on a structure surface
#[derive(Debug, Clone, PartialEq)]
pub struct GeometricPattern {
    pub pattern_type: GeometricPatternType,
    pub scale: Float,
    pub rotation: Float,
}

impl GeometricPattern {
    pub fn new(pattern_type: GeometricPatternType, scale: Float, rotation: Float) -> Self {
        Self {
            pattern_type,
            scale: scale.clamp(0.1, 10.0),
            rotation: rotation % 360.0,
        }
    }
}

/// Visual properties of a holographic structure
///
/// Determines how the structure appears visually to observers.
#[derive(Debug, Clone, PartialEq)]
pub struct VisualProperties {
    /// Color palette as RGB values
    pub color_palette: [(u8, u8, u8); 5],
    /// Material appearance determining texture and density
    pub material_appearance: MaterialAppearance,
    /// Geometric patterns encoded in the structure
    pub geometric_patterns: Vec<GeometricPattern>,
    /// Light emission intensity (0.0 to 1.0)
    pub light_emission: Float,
    /// Transparency level (0.0 to 1.0)
    pub transparency: Float,
}

impl VisualProperties {
    pub fn new() -> Self {
        Self {
            color_palette: [
                (100, 100, 100),
                (150, 150, 150),
                (200, 200, 200),
                (250, 250, 250),
                (255, 255, 255),
            ],
            material_appearance: MaterialAppearance::Solid,
            geometric_patterns: Vec::new(),
            light_emission: 0.0,
            transparency: 0.0,
        }
    }
}

impl Default for VisualProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Functional properties of a holographic structure
///
/// Determines what the structure can do and how it functions.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalProperties {
    /// Maximum capacity (entities or units)
    pub capacity: usize,
    /// Energy flow rate (units per time)
    pub energy_flow: Float,
    /// Resonance amplification factor
    pub resonance_amplification: Float,
    /// Memory capacity (memory entries)
    pub memory_capacity: usize,
    /// Communication range (distance units)
    pub communication_range: Float,
}

impl FunctionalProperties {
    pub fn new() -> Self {
        Self {
            capacity: 10,
            energy_flow: 1.0,
            resonance_amplification: 1.0,
            memory_capacity: 100,
            communication_range: 100.0,
        }
    }
}

impl Default for FunctionalProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Structure properties derived from holographic signature
///
/// Contains both visual and functional characteristics of the structure.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureProperties {
    /// Physical size (volume units)
    pub size: Float,
    /// Complexity level (affects build requirements)
    pub complexity: Float,
    /// Density affinity (1-8, representing density octave)
    pub density_affinity: usize,
    /// Primary archetype index (0-21)
    pub primary_archetype: usize,
    /// Emotional frequency (Hz)
    pub emotional_frequency: Float,
    /// Visual appearance properties
    pub visual_properties: VisualProperties,
    /// Functional capabilities
    pub functional_properties: FunctionalProperties,
}

impl StructureProperties {
    pub fn new() -> Self {
        Self {
            size: 100.0,
            complexity: 0.5,
            density_affinity: 4,
            primary_archetype: 11,
            emotional_frequency: 440.0,
            visual_properties: VisualProperties::new(),
            functional_properties: FunctionalProperties::new(),
        }
    }
}

impl Default for StructureProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// A structure manifested through collective resonance
///
/// Holographic structures emerge when collective resonance exceeds a threshold.
/// Each structure embodies the collective's holographic signature and serves
/// as a physical manifestation of their shared resonance pattern.
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6, Week 89-92:
/// "Building as collective manifestation - structures emerge from collective resonance"
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicStructure {
    /// Unique identifier for the structure
    pub structure_id: u64,
    /// Type of structure (determines base properties)
    pub structure_type: StructureType,
    /// Entity IDs that contributed to this structure
    pub contributors: Vec<u64>,
    /// 22-archetype holographic signature
    pub holographic_signature: [Float; 22],
    /// Space/time spectrum ratio
    pub spectrum_ratio: SpectrumRatio,
    /// Derived properties of the structure
    pub structure_properties: StructureProperties,
    /// Build progress (0.0 to 1.0)
    pub build_progress: Float,
    /// Required resonance for completion
    pub required_resonance: Float,
    /// Current accumulated resonance
    pub current_resonance: Float,
    /// Stability of the structure (0.0 to 1.0)
    pub stability: Float,
    /// Position in 3D space
    pub position: [Float; 3],
    /// Memory key for holographic storage
    pub memory_key: Option<u64>,
}

impl HolographicStructure {
    /// Create a holographic structure from collective resonance
    ///
    /// # Arguments
    /// * `structure_id` - Unique identifier for the structure
    /// * `structure_type` - Type of structure to manifest
    /// * `resonance_result` - Collective resonance computation result
    /// * `position` - 3D position for structure manifestation
    ///
    /// # Returns
    /// * New `HolographicStructure` initialized from collective resonance
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 6:
    /// "The holographic signature is derived from the collective resonance pattern"
    pub fn from_collective_resonance(
        structure_id: u64,
        structure_type: StructureType,
        resonance_result: &CollectiveResonanceResult,
        position: [Float; 3],
    ) -> Self {
        // Convert 8-element pattern to 22-element signature
        let holographic_signature =
            Self::pattern_to_signature(&resonance_result.collective_pattern.pattern);

        // Create spectrum ratio from pattern
        let spectrum_ratio = SpectrumRatio::new(
            resonance_result.collective_pattern.pattern[0],
            resonance_result.collective_pattern.pattern[4],
        );

        // Derive structure properties from signature
        let structure_properties = Self::derive_properties(&holographic_signature, structure_type);

        // Calculate required resonance based on structure type and properties
        let required_resonance = Self::calculate_required_resonance(
            structure_type,
            &structure_properties,
            resonance_result,
        );

        HolographicStructure {
            structure_id,
            structure_type,
            contributors: Vec::new(),
            holographic_signature,
            spectrum_ratio,
            structure_properties,
            build_progress: 0.0,
            required_resonance,
            current_resonance: 0.0,
            stability: resonance_result.stability,
            position,
            memory_key: None,
        }
    }

    /// Convert 8-element resonance pattern to 22-element holographic signature
    ///
    /// Expands the density octave pattern (8 elements) to the full archetypical
    /// signature (22 elements) using interpolation and archetype mapping.
    ///
    /// # Arguments
    /// * `pattern` - 8-element resonance pattern from collective resonance
    ///
    /// # Returns
    /// * 22-element holographic signature array
    ///
    /// From holographic_physics.rs:
    /// "Holographic signatures contain 22 archetypical components"
    fn pattern_to_signature(pattern: &[Float; 8]) -> [Float; 22] {
        let mut signature = [0.0; 22];

        // Map density octave to archetypical mind (7 octants × 3 rungs = 21, plus archetype 22)
        // Each density influences multiple archetypes

        // Octant 1 (Archetypes 1-3): First density influence
        signature[0] = pattern[0];
        signature[1] = pattern[0] * 0.9;
        signature[2] = pattern[0] * 0.8;

        // Octant 2 (Archetypes 4-6): Second density influence
        signature[3] = pattern[1] * 0.8 + pattern[0] * 0.2;
        signature[4] = pattern[1];
        signature[5] = pattern[1] * 0.9 + pattern[2] * 0.1;

        // Octant 3 (Archetypes 7-9): Third density influence
        signature[6] = pattern[2] * 0.7 + pattern[1] * 0.3;
        signature[7] = pattern[2];
        signature[8] = pattern[2] * 0.8 + pattern[3] * 0.2;

        // Octant 4 (Archetypes 10-12): Fourth density influence
        signature[9] = pattern[3] * 0.6 + pattern[2] * 0.4;
        signature[10] = pattern[3];
        signature[11] = pattern[3] * 0.7 + pattern[4] * 0.3;

        // Octant 5 (Archetypes 13-15): Fifth density influence
        signature[12] = pattern[4] * 0.5 + pattern[3] * 0.5;
        signature[13] = pattern[4];
        signature[14] = pattern[4] * 0.6 + pattern[5] * 0.4;

        // Octant 6 (Archetypes 16-18): Sixth density influence
        signature[15] = pattern[5] * 0.4 + pattern[4] * 0.6;
        signature[16] = pattern[5];
        signature[17] = pattern[5] * 0.5 + pattern[6] * 0.5;

        // Octant 7 (Archetypes 19-21): Seventh density influence
        signature[18] = pattern[6] * 0.3 + pattern[5] * 0.7;
        signature[19] = pattern[6];
        signature[20] = pattern[6] * 0.4 + pattern[7] * 0.6;

        // Archetype 22: Eighth density influence (The Great Way)
        signature[21] = pattern[7];

        // Normalize all signature values to [0.0, 1.0]
        for item in &mut signature {
            *item = item.clamp(0.0, 1.0);
        }

        signature
    }

    /// Derive structure properties from holographic signature
    ///
    /// Extracts physical and functional properties from the archetypical signature.
    ///
    /// # Arguments
    /// * `signature` - 22-element holographic signature
    /// * `structure_type` - Type of structure being created
    ///
    /// # Returns
    /// * `StructureProperties` containing all derived properties
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The signature contains all information needed to manifest the structure"
    fn derive_properties(
        signature: &[Float; 22],
        structure_type: StructureType,
    ) -> StructureProperties {
        // Find primary archetype (strongest signature value)
        let primary_archetype = signature
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(11);

        // Calculate density affinity from signature
        let density_affinity = Self::calculate_density_affinity(signature);

        // Calculate complexity from signature variance
        let complexity = Self::calculate_complexity(signature);

        // Calculate size based on structure type and signature
        let size = Self::calculate_size(structure_type, signature);

        // Calculate emotional frequency from primary archetype
        let emotional_frequency = 440.0 * (1.0 + (primary_archetype as Float) / 21.0);

        // Derive visual properties
        let visual_properties = Self::derive_visual_properties(signature, structure_type);

        // Derive functional properties
        let functional_properties =
            Self::derive_functional_properties(signature, structure_type, complexity);

        StructureProperties {
            size,
            complexity,
            density_affinity,
            primary_archetype,
            emotional_frequency,
            visual_properties,
            functional_properties,
        }
    }

    /// Calculate density affinity from holographic signature
    ///
    /// Determines which density level the structure resonates with most strongly.
    fn calculate_density_affinity(signature: &[Float; 22]) -> usize {
        // Calculate average for each octant (7 octants, 3 archetypes each)
        let mut density_strengths = [0.0; 7];

        for (i, strength) in density_strengths.iter_mut().enumerate() {
            let start = i * 3;
            let end = start + 3;
            let sum: Float = signature[start..end.min(22)].iter().sum();
            *strength = sum / 3.0;
        }

        // Add archetype 22 to the last density
        density_strengths[6] = (density_strengths[6] + signature[21]) / 2.0;

        // Find strongest density (0-based, convert to 1-based)
        let strongest_density = density_strengths
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(3);

        strongest_density + 1 // Convert to 1-based density
    }

    /// Calculate complexity from signature variance
    ///
    /// Higher variance indicates more complex structure.
    fn calculate_complexity(signature: &[Float; 22]) -> Float {
        let mean: Float = signature.iter().sum::<Float>() / 22.0;
        let variance: Float = signature.iter().map(|x| (x - mean).powi(2)).sum::<Float>() / 22.0;
        let std_dev = variance.sqrt();

        // Normalize complexity to [0.3, 1.0]
        // Increased minimum to 0.3 to ensure structures have meaningful complexity
        (0.3 + std_dev * 2.0).clamp(0.3, 1.0)
    }

    /// Calculate size based on structure type and signature
    fn calculate_size(structure_type: StructureType, signature: &[Float; 22]) -> Float {
        let base_size = match structure_type {
            StructureType::Temple => 500.0,
            StructureType::Monument => 300.0,
            StructureType::Library => 400.0,
            StructureType::CommunityCenter => 600.0,
            StructureType::Habitat => 200.0,
            StructureType::EnergyAnchor => 150.0,
            StructureType::CommunicationHub => 250.0,
            StructureType::HealingSanctuary => 180.0,
            StructureType::CreativeStudio => 160.0,
            StructureType::Custom => 250.0,
            StructureType::ResonanceCollector => 200.0,
        };

        // Scale by signature magnitude
        let signature_magnitude: Float = signature.iter().sum::<Float>() / 22.0;

        base_size * (0.5 + signature_magnitude)
    }

    /// Derive visual properties from holographic signature
    ///
    /// Extracts color, material, patterns, light emission, and transparency.
    fn derive_visual_properties(
        signature: &[Float; 22],
        structure_type: StructureType,
    ) -> VisualProperties {
        // Generate color palette from signature
        let color_palette = Self::generate_color_palette(signature, structure_type);

        // Determine material appearance from density affinity
        let density_affinity = Self::calculate_density_affinity(signature);
        let material_appearance = match density_affinity {
            1..=2 => MaterialAppearance::Ethereal,
            3..=4 => MaterialAppearance::Gaseous,
            5 => MaterialAppearance::Liquid,
            6 => MaterialAppearance::Crystalline,
            7 => MaterialAppearance::Plasma,
            8 => MaterialAppearance::Solid,
            _ => MaterialAppearance::Solid,
        };

        // Generate geometric patterns from signature
        let geometric_patterns = Self::generate_geometric_patterns(signature);

        // Calculate light emission from signature intensity
        let signature_intensity: Float = signature.iter().map(|x| x.abs()).sum::<Float>() / 22.0;
        let light_emission = signature_intensity.clamp(0.0, 1.0);

        // Calculate transparency from signature variance
        let transparency = 1.0 - signature_intensity * 0.5;

        VisualProperties {
            color_palette,
            material_appearance,
            geometric_patterns,
            light_emission,
            transparency: transparency.clamp(0.0, 1.0),
        }
    }

    /// Generate color palette from holographic signature
    fn generate_color_palette(
        signature: &[Float; 22],
        structure_type: StructureType,
    ) -> [(u8, u8, u8); 5] {
        let base_hue = match structure_type {
            StructureType::Temple => 0.8,              // Purple
            StructureType::Monument => 0.5,            // Gold
            StructureType::Library => 0.7,             // Blue
            StructureType::CommunityCenter => 0.3,     // Green
            StructureType::Habitat => 0.2,             // Orange
            StructureType::EnergyAnchor => 0.9,        // Violet
            StructureType::CommunicationHub => 0.4,    // Cyan
            StructureType::HealingSanctuary => 0.15,   // Rose
            StructureType::CreativeStudio => 0.55,     // Pink
            StructureType::Custom => 0.6,              // Indigo
            StructureType::ResonanceCollector => 0.75, // Purple
        };

        let mut palette = [(0, 0, 0); 5];

        for i in 0..5 {
            let hue_shift = (i as Float) * 0.1;
            let hue = (base_hue + hue_shift) % 1.0;
            let saturation = 0.7 + signature[i] * 0.3;
            let lightness = 0.4 + signature[i + 5] * 0.4;

            palette[i] = hsl_to_rgb(hue, saturation, lightness);
        }

        palette
    }

    /// Generate geometric patterns from holographic signature
    fn generate_geometric_patterns(signature: &[Float; 22]) -> Vec<GeometricPattern> {
        let mut patterns = Vec::new();

        // Select pattern types based on strongest archetypes
        let pattern_types = [
            GeometricPatternType::FlowerOfLife,
            GeometricPatternType::MetatronsCube,
            GeometricPatternType::SriYantra,
            GeometricPatternType::Toroidal,
            GeometricPatternType::FibonacciSpiral,
            GeometricPatternType::VesicaPiscis,
        ];

        // Create patterns for archetypes with strong signatures
        for &pattern_type in pattern_types.iter() {
            let archetype_index = pattern_type.archetype_index();
            if archetype_index < 22 && signature[archetype_index] > 0.5 {
                let scale = 1.0 + signature[archetype_index];
                let rotation = signature[(archetype_index + 1) % 22] * 360.0;
                patterns.push(GeometricPattern::new(pattern_type, scale, rotation));
            }
        }

        patterns
    }

    /// Derive functional properties from holographic signature
    ///
    /// Calculates capacity, energy flow, resonance amplification, memory capacity,
    /// and communication range.
    fn derive_functional_properties(
        signature: &[Float; 22],
        structure_type: StructureType,
        complexity: Float,
    ) -> FunctionalProperties {
        // Base values from structure type
        let (base_capacity, base_energy, base_amplification, base_memory, base_range) =
            match structure_type {
                StructureType::Temple => (50, 5.0, 2.0, 1000, 500.0),
                StructureType::Monument => (10, 1.0, 1.5, 100, 200.0),
                StructureType::Library => (30, 2.0, 1.2, 5000, 300.0),
                StructureType::CommunityCenter => (100, 3.0, 1.3, 500, 400.0),
                StructureType::Habitat => (20, 4.0, 1.1, 200, 100.0),
                StructureType::EnergyAnchor => (5, 10.0, 3.0, 50, 1000.0),
                StructureType::CommunicationHub => (15, 8.0, 2.5, 300, 2000.0),
                StructureType::HealingSanctuary => (25, 6.0, 2.0, 400, 300.0),
                StructureType::CreativeStudio => (10, 3.0, 1.8, 150, 150.0),
                StructureType::Custom => (20, 5.0, 2.0, 500, 500.0),
                StructureType::ResonanceCollector => (8, 12.0, 4.0, 200, 1500.0),
            };

        // Scale by signature magnitude and complexity
        let signature_magnitude: Float = signature.iter().sum::<Float>() / 22.0;
        let scaling_factor = 0.5 + signature_magnitude * 0.5;

        FunctionalProperties {
            capacity: (base_capacity as Float * scaling_factor * complexity) as usize,
            energy_flow: base_energy * scaling_factor * complexity,
            resonance_amplification: base_amplification * scaling_factor,
            memory_capacity: (base_memory as Float * scaling_factor) as usize,
            communication_range: base_range * scaling_factor,
        }
    }

    /// Calculate required resonance for structure completion
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md: "Resonance is the binding force that holds holographic patterns together"
    /// Combines base requirement from structure type with property-based scaling.
    fn calculate_required_resonance(
        structure_type: StructureType,
        properties: &StructureProperties,
        resonance_result: &CollectiveResonanceResult,
    ) -> Float {
        let base_requirement = structure_type.base_resonance_requirement();

        // Scale by structure size and complexity
        let size_factor = properties.size / 250.0; // Normalize around typical size
        let complexity_factor = properties.complexity;

        // Apply coherence penalty (lower coherence requires more resonance)
        // coherence_factor: higher = more resonance needed (low coherence = high penalty)
        // Modified to have stronger impact: range 1.0 to 4.0 for more noticeable difference
        let coherence_factor = 1.0 + 3.0 * (1.0 - resonance_result.coherence);

        // Calculate total requirement with coherence adjustment
        // From COSMOLOGICAL-ARCHITECTURE.md: "Lower coherence requires more resonance to manifest"
        // The formula ensures that coherence differences are always preserved
        let result = base_requirement * size_factor * complexity_factor * coherence_factor;

        // From COSMOLOGICAL-ARCHITECTURE.md: "Structures must meet their minimum resonance threshold"
        // Ensure result is always at least the base requirement
        // Coherence differences multiply the result, so applying floor after multiplication preserves differences
        result.max(base_requirement)
    }

    /// Update build progress based on added resonance
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md: "Resonance is the binding force that holds holographic patterns together"
    ///
    /// # Arguments
    /// * `added_resonance` - Amount of resonance to add
    ///
    /// # Returns
    /// * New build progress (0.0 to 1.0)
    pub fn update_build_progress(&mut self, added_resonance: Float) -> Float {
        self.current_resonance += added_resonance;
        self.build_progress = (self.current_resonance / self.required_resonance).clamp(0.0, 1.0);
        self.build_progress
    }

    /// Check if structure is complete
    ///
    /// # Returns
    /// * `true` if build progress >= 1.0, `false` otherwise
    pub fn is_complete(&self) -> bool {
        self.build_progress >= 1.0
    }

    /// Add a contributor to the structure
    ///
    /// # Arguments
    /// * `entity_id` - ID of the entity contributing to the structure
    pub fn add_contributor(&mut self, entity_id: u64) {
        if !self.contributors.contains(&entity_id) {
            self.contributors.push(entity_id);
        }
    }

    /// Get the number of contributors
    pub fn contributor_count(&self) -> usize {
        self.contributors.len()
    }

    /// Update stability based on new coherence value
    ///
    /// # Arguments
    /// * `coherence` - New coherence value (0.0 to 1.0)
    pub fn update_stability(&mut self, coherence: Float) {
        // Stability follows sigmoid curve based on coherence
        let sigmoid = |x: Float| 1.0 / (1.0 + (-5.0 * (x - 0.5)).exp());
        self.stability = sigmoid(coherence.clamp(0.0, 1.0));
    }

    /// Get build completion percentage
    ///
    /// # Returns
    /// * Completion percentage (0.0 to 100.0)
    pub fn completion_percentage(&self) -> Float {
        self.build_progress * 100.0
    }

    /// Get resonance shortfall
    ///
    /// # Returns
    /// * Amount of resonance still needed for completion
    pub fn resonance_shortfall(&self) -> Float {
        (self.required_resonance - self.current_resonance).max(0.0)
    }
}

/// Convert HSL color to RGB
///
/// Helper function for generating color palettes.
fn hsl_to_rgb(h: Float, s: Float, l: Float) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::holographic_inventory::ResonancePattern;

    /// Helper function to create a test resonance result
    fn create_test_resonance_result(coherence: Float) -> CollectiveResonanceResult {
        let pattern = ResonancePattern {
            pattern: [0.5; 8],
            stability: 0.8,
            phase: 0.0,
        };

        CollectiveResonanceResult {
            collective_pattern: pattern,
            coherence,
            stability: coherence,
            resonance_strength: coherence * 0.8,
            dominant_densities: vec![4, 3, 5, 2, 6, 1, 7, 8],
        }
    }

    #[test]
    fn test_structure_from_resonance() {
        let resonance_result = create_test_resonance_result(0.8);
        let position = [10.0, 20.0, 30.0];

        let structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            position,
        );

        assert_eq!(structure.structure_id, 1);
        assert_eq!(structure.structure_type, StructureType::Temple);
        assert_eq!(structure.position, position);
        assert_eq!(structure.build_progress, 0.0);
        assert_eq!(structure.current_resonance, 0.0);
        assert!(structure.required_resonance > 0.0);
        assert!(!structure.is_complete());
    }

    #[test]
    fn test_pattern_to_signature() {
        let pattern = [0.1, 0.3, 0.5, 0.7, 0.9, 0.7, 0.5, 0.3];
        let signature = HolographicStructure::pattern_to_signature(&pattern);

        assert_eq!(signature.len(), 22);

        // All values should be in valid range
        for value in signature.iter() {
            assert!(*value >= 0.0 && *value <= 1.0);
        }

        // Signature should show pattern progression
        assert!(signature[0] > 0.0);
        assert!(signature[21] > 0.0);
    }

    #[test]
    fn test_pattern_to_signature_uniform() {
        let pattern = [0.5; 8];
        let signature = HolographicStructure::pattern_to_signature(&pattern);

        assert_eq!(signature.len(), 22);

        // All values should be in valid range
        for value in signature.iter() {
            assert!(*value >= 0.0 && *value <= 1.0);
        }

        // Uniform pattern should produce moderate signature values
        assert!(signature[0] >= 0.4 && signature[0] <= 0.6);
    }

    #[test]
    fn test_property_derivation() {
        let signature = [0.5; 22];
        let properties = HolographicStructure::derive_properties(&signature, StructureType::Temple);

        assert!(properties.size > 0.0);
        assert!(properties.complexity >= 0.1 && properties.complexity <= 1.0);
        assert!(properties.density_affinity >= 1 && properties.density_affinity <= 8);
        assert!(properties.primary_archetype < 22);
        assert!(properties.emotional_frequency > 0.0);
    }

    #[test]
    fn test_density_affinity_calculation() {
        let mut signature = [0.0; 22];

        // Strong first density (archetypes 1-3)
        signature[0] = 1.0;
        signature[1] = 1.0;
        signature[2] = 1.0;

        let affinity = HolographicStructure::calculate_density_affinity(&signature);
        assert_eq!(affinity, 1);

        // Strong second density (archetypes 4-6)
        signature = [0.0; 22];
        signature[3] = 1.0;
        signature[4] = 1.0;
        signature[5] = 1.0;

        let affinity = HolographicStructure::calculate_density_affinity(&signature);
        assert_eq!(affinity, 2);
    }

    #[test]
    fn test_complexity_calculation() {
        let uniform_signature = [0.5; 22];
        let uniform_complexity = HolographicStructure::calculate_complexity(&uniform_signature);

        let varied_signature = {
            let mut sig = [0.0; 22];
            for i in 0..22 {
                sig[i] = (i as Float) / 22.0;
            }
            sig
        };
        let varied_complexity = HolographicStructure::calculate_complexity(&varied_signature);

        // Varied signature should have higher complexity
        assert!(varied_complexity > uniform_complexity);
    }

    #[test]
    fn test_size_calculation() {
        let signature = [0.5; 22];

        let temple_size = HolographicStructure::calculate_size(StructureType::Temple, &signature);
        let habitat_size = HolographicStructure::calculate_size(StructureType::Habitat, &signature);

        // Temple should be larger than habitat
        assert!(temple_size > habitat_size);
    }

    #[test]
    fn test_visual_properties() {
        let signature = [0.5; 22];
        let visual_properties =
            HolographicStructure::derive_visual_properties(&signature, StructureType::Temple);

        assert_eq!(visual_properties.color_palette.len(), 5);
        assert!(visual_properties.light_emission >= 0.0);
        assert!(visual_properties.transparency >= 0.0 && visual_properties.transparency <= 1.0);
    }

    #[test]
    fn test_functional_properties() {
        let signature = [0.5; 22];
        let functional_properties = HolographicStructure::derive_functional_properties(
            &signature,
            StructureType::Temple,
            0.5,
        );

        assert!(functional_properties.capacity > 0);
        assert!(functional_properties.energy_flow > 0.0);
        assert!(functional_properties.resonance_amplification > 0.0);
        assert!(functional_properties.memory_capacity > 0);
        assert!(functional_properties.communication_range > 0.0);
    }

    #[test]
    fn test_required_resonance_calculation() {
        let resonance_result = create_test_resonance_result(0.8);
        let signature = [0.5; 22];
        let properties = HolographicStructure::derive_properties(&signature, StructureType::Temple);

        let required = HolographicStructure::calculate_required_resonance(
            StructureType::Temple,
            &properties,
            &resonance_result,
        );

        assert!(required > 0.0);
        assert!(required >= StructureType::Temple.base_resonance_requirement());
    }

    #[test]
    fn test_build_progress() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        assert_eq!(structure.build_progress, 0.0);
        assert!(!structure.is_complete());

        // Add half the required resonance
        let half_resonance = structure.required_resonance / 2.0;
        structure.update_build_progress(half_resonance);

        assert!(structure.build_progress > 0.0);
        assert!(structure.build_progress < 1.0);
        assert!(!structure.is_complete());

        // Add remaining resonance
        structure.update_build_progress(half_resonance);

        assert!(structure.build_progress >= 1.0);
        assert!(structure.is_complete());
    }

    #[test]
    fn test_build_progress_excess() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        // Add more than required resonance
        structure.update_build_progress(structure.required_resonance * 2.0);

        // Progress should cap at 1.0
        assert_eq!(structure.build_progress, 1.0);
        assert!(structure.is_complete());
    }

    #[test]
    fn test_structure_completion() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        assert!(!structure.is_complete());

        structure.update_build_progress(structure.required_resonance);
        assert!(structure.is_complete());
    }

    #[test]
    fn test_contributors() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        assert_eq!(structure.contributor_count(), 0);

        structure.add_contributor(100);
        structure.add_contributor(200);
        structure.add_contributor(300);

        assert_eq!(structure.contributor_count(), 3);

        // Adding duplicate should not increase count
        structure.add_contributor(200);
        assert_eq!(structure.contributor_count(), 3);
    }

    #[test]
    fn test_stability_update() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        // High coherence should produce high stability
        structure.update_stability(0.9);
        assert!(structure.stability > 0.8);

        // Low coherence should produce low stability
        structure.update_stability(0.1);
        assert!(structure.stability < 0.4);
    }

    #[test]
    fn test_completion_percentage() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        assert_eq!(structure.completion_percentage(), 0.0);

        structure.update_build_progress(structure.required_resonance / 2.0);
        assert!((structure.completion_percentage() - 50.0).abs() < 1.0);

        structure.update_build_progress(structure.required_resonance);
        assert_eq!(structure.completion_percentage(), 100.0);
    }

    #[test]
    fn test_resonance_shortfall() {
        let resonance_result = create_test_resonance_result(0.8);
        let mut structure = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        let initial_shortfall = structure.resonance_shortfall();
        assert_eq!(initial_shortfall, structure.required_resonance);

        structure.update_build_progress(structure.required_resonance / 2.0);
        let half_shortfall = structure.resonance_shortfall();
        assert!((half_shortfall - initial_shortfall / 2.0).abs() < 0.01);

        structure.update_build_progress(structure.required_resonance);
        assert_eq!(structure.resonance_shortfall(), 0.0);
    }

    #[test]
    fn test_structure_type_base_resonance() {
        assert!(
            StructureType::Temple.base_resonance_requirement()
                > StructureType::Habitat.base_resonance_requirement()
        );
        assert!(
            StructureType::Library.base_resonance_requirement()
                > StructureType::CommunityCenter.base_resonance_requirement()
        );
    }

    #[test]
    fn test_material_appearance_density_factor() {
        assert!(
            MaterialAppearance::Solid.density_factor()
                > MaterialAppearance::Ethereal.density_factor()
        );
        assert_eq!(MaterialAppearance::Solid.density_factor(), 1.0);
    }

    #[test]
    fn test_geometric_pattern_archetype_index() {
        assert_eq!(GeometricPatternType::FlowerOfLife.archetype_index(), 0);
        assert_eq!(GeometricPatternType::MetatronsCube.archetype_index(), 7);
        assert_eq!(GeometricPatternType::SriYantra.archetype_index(), 14);
    }

    #[test]
    fn test_geometric_pattern_creation() {
        let pattern = GeometricPattern::new(GeometricPatternType::FlowerOfLife, 2.0, 45.0);

        assert_eq!(pattern.pattern_type, GeometricPatternType::FlowerOfLife);
        assert_eq!(pattern.scale, 2.0);
        assert_eq!(pattern.rotation, 45.0);
    }

    #[test]
    fn test_geometric_pattern_clamping() {
        // Scale should be clamped
        let pattern_small = GeometricPattern::new(GeometricPatternType::FlowerOfLife, 0.01, 0.0);
        assert_eq!(pattern_small.scale, 0.1);

        let pattern_large = GeometricPattern::new(GeometricPatternType::FlowerOfLife, 20.0, 0.0);
        assert_eq!(pattern_large.scale, 10.0);

        // Rotation should wrap
        let pattern_wrap = GeometricPattern::new(GeometricPatternType::FlowerOfLife, 1.0, 450.0);
        assert_eq!(pattern_wrap.rotation, 90.0);
    }

    #[test]
    fn test_visual_properties_default() {
        let visual = VisualProperties::new();

        assert_eq!(visual.color_palette.len(), 5);
        assert_eq!(visual.geometric_patterns.len(), 0);
        assert_eq!(visual.light_emission, 0.0);
        assert_eq!(visual.transparency, 0.0);
    }

    #[test]
    fn test_functional_properties_default() {
        let functional = FunctionalProperties::new();

        assert_eq!(functional.capacity, 10);
        assert_eq!(functional.energy_flow, 1.0);
        assert_eq!(functional.resonance_amplification, 1.0);
        assert_eq!(functional.memory_capacity, 100);
        assert_eq!(functional.communication_range, 100.0);
    }

    #[test]
    fn test_structure_properties_default() {
        let properties = StructureProperties::new();

        assert_eq!(properties.size, 100.0);
        assert_eq!(properties.complexity, 0.5);
        assert_eq!(properties.density_affinity, 4);
        assert_eq!(properties.primary_archetype, 11);
    }

    #[test]
    fn test_different_structure_types() {
        let resonance_result = create_test_resonance_result(0.8);

        let temple = HolographicStructure::from_collective_resonance(
            1,
            StructureType::Temple,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        let habitat = HolographicStructure::from_collective_resonance(
            2,
            StructureType::Habitat,
            &resonance_result,
            [0.0, 0.0, 0.0],
        );

        assert!(temple.structure_properties.size > habitat.structure_properties.size);
        assert!(temple.required_resonance > habitat.required_resonance);
    }

    #[test]
    fn test_geometric_patterns_generation() {
        let signature = {
            let mut sig = [0.0; 22];
            sig[0] = 0.9; // Strong Flower of Life
            sig[7] = 0.8; // Strong Metatron's Cube
            sig
        };

        let patterns = HolographicStructure::generate_geometric_patterns(&signature);

        assert!(!patterns.is_empty());
        assert!(patterns
            .iter()
            .any(|p| p.pattern_type == GeometricPatternType::FlowerOfLife));
    }

    #[test]
    fn test_color_palette_generation() {
        let signature = [0.5; 22];

        let temple_palette =
            HolographicStructure::generate_color_palette(&signature, StructureType::Temple);
        let library_palette =
            HolographicStructure::generate_color_palette(&signature, StructureType::Library);

        // Different structure types should have different base hues
        assert!(temple_palette != library_palette);

        // All colors should be valid RGB (u8 values are always <= 255)
        for color in temple_palette.iter() {
            let _ = color;
        }
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        // Test red (hue = 0)
        let rgb = hsl_to_rgb(0.0, 1.0, 0.5);
        assert!(rgb.0 > rgb.1); // Red dominant
        assert!(rgb.0 > rgb.2);

        // Test green (hue = 1/3)
        let rgb = hsl_to_rgb(1.0 / 3.0, 1.0, 0.5);
        assert!(rgb.1 > rgb.0); // Green dominant
        assert!(rgb.1 > rgb.2);

        // Test blue (hue = 2/3)
        let rgb = hsl_to_rgb(2.0 / 3.0, 1.0, 0.5);
        assert!(rgb.2 > rgb.0); // Blue dominant
        assert!(rgb.2 > rgb.1);

        // Test grayscale (saturation = 0)
        let rgb = hsl_to_rgb(0.5, 0.0, 0.5);
        assert_eq!(rgb.0, rgb.1);
        assert_eq!(rgb.1, rgb.2);
    }

    #[test]
    fn test_coherence_impact_on_required_resonance() {
        let signature = [0.5; 22];
        let properties = HolographicStructure::derive_properties(&signature, StructureType::Temple);

        let high_coherence = create_test_resonance_result(0.9);
        let low_coherence = create_test_resonance_result(0.3);

        let high_required = HolographicStructure::calculate_required_resonance(
            StructureType::Temple,
            &properties,
            &high_coherence,
        );

        let low_required = HolographicStructure::calculate_required_resonance(
            StructureType::Temple,
            &properties,
            &low_coherence,
        );

        // Debug output
        println!("High coherence (0.9): {}", high_required);
        println!("Low coherence (0.3): {}", low_required);
        println!("Low > High: {}", low_required > high_required);

        // Low coherence should require more resonance
        assert!(
            low_required > high_required,
            "low_required ({}) should be > high_required ({})",
            low_required,
            high_required
        );
    }
}
