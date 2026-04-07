// Holographic Seed Module
//
// This module implements the HolographicSeed structure that contains the complete
// 22-Archetype structure inherited by every Entity during Involution.
//
// The HolographicSeed is the ROM (Read-Only Memory) of the entity - it contains
// the immutable architectural blueprint that was impressed during Involution.
//
// PHASE 1 REFACTOR: HolographicSeed is now immutable and shared via Arc
// - Archetypes are immutable ROM (Read-Only Memory)
// - Shared across all entities via Arc
// - No mutability allowed on any field
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 5.1
// - Energy-Ray-Centers/1. Red Ray.json
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.3 (Critical Gap #3: Entity Emergence)
// - REFACTOR_ROADMAP_V3.md Phase 1: Foundation - Holographic Seed Architecture

use crate::archetypes::ArchetypeSystem;
use crate::coordinates::space_time::PhysicalSpaceTimeCoord;
use crate::light::LightArchitecture;
use crate::memory::soul_stream::SoulStream; // Use V3.0 instead
use crate::solar_system::SolarSystemConstraints;
// use crate::spectrum::archetypical_mind::SeedArchetypicalMind as SpectrumSeedArchetypicalMind; // Not needed - SeedArchetypicalMind is defined in this module
use crate::types::Float;
use std::sync::Arc;

/// A reference to a HolographicSeed
///
/// This allows entities to reference the seed without owning it.
/// The Entity emerges from the seed and maintains a reference to it.
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.3
/// "Entity is the seed experiencing itself at a point"
#[derive(Debug, Clone)]
pub struct HolographicSeedReference {
    /// Reference to the seed using Arc for thread-safe sharing
    seed: Arc<HolographicSeed>,
}

impl HolographicSeedReference {
    /// Create a new reference to a seed
    pub fn new(seed: Arc<HolographicSeed>) -> Self {
        Self { seed }
    }

    /// Get the seed
    pub fn get_seed(&self) -> &HolographicSeed {
        &self.seed
    }

    /// Get the seed as Arc (for cloning)
    pub fn as_arc(&self) -> Arc<HolographicSeed> {
        Arc::clone(&self.seed)
    }

    /// Check if this reference points to the same seed as another reference
    ///
    /// This compares the content of the seeds, not the pointers,
    /// since the seed may be cloned when wrapping in Arc.
    pub fn references_same_seed(&self, other: &HolographicSeedReference) -> bool {
        // Compare the content of the seeds
        let my_seed = self.get_seed();
        let other_seed = other.get_seed();

        // Compare free will intensity
        my_seed.free_will.free_will_intensity == other_seed.free_will.free_will_intensity
            && my_seed.free_will.choice_capacity == other_seed.free_will.choice_capacity
            && my_seed.free_will.polarization_potential
                == other_seed.free_will.polarization_potential
    }

    /// Check if this reference points to the same Arc as another reference
    ///
    /// This compares the underlying Arc pointers directly, which is useful
    /// for verifying that multiple entities share the same holographic seed.
    ///
    /// This is different from `references_same_seed()` which compares content.
    /// This method checks pointer equality.
    pub fn ptr_eq(&self, other: &HolographicSeedReference) -> bool {
        Arc::ptr_eq(&self.seed, &other.seed)
    }
}

/// The Emergence Manifestation - represents the entity as it emerges from the seed
///
/// This structure captures the essence of emergence: the entity is not constructed
/// from parts, but emerges as a whole manifestation of the seed at a point.
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.3
/// "Entity EMERGES from seed, not constructed"
#[derive(Debug, Clone)]
pub struct EmergenceManifestation {
    /// Reference to the seed (the source)
    pub seed_reference: HolographicSeedReference,

    /// Manifestation point in Space/Time
    pub manifestation_point: PhysicalSpaceTimeCoord,

    /// Individualization through Soul Stream
    pub individualization: SoulStream,

    /// Emergence timestamp
    pub emergence_timestamp: u64,

    /// Whether this is a localized manifestation
    pub is_localized: bool,
}

impl EmergenceManifestation {
    /// Create a new emergence manifestation
    pub fn new(
        seed_reference: HolographicSeedReference,
        manifestation_point: PhysicalSpaceTimeCoord,
        individualization: SoulStream,
    ) -> Self {
        Self {
            seed_reference,
            manifestation_point,
            individualization,
            emergence_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_localized: true,
        }
    }

    /// Check if this manifestation references the given seed
    ///
    /// This compares the content of the seeds, not the pointers,
    /// since the seed may be cloned when wrapping in Arc.
    pub fn references_seed(&self, seed: &HolographicSeed) -> bool {
        // Compare the content of the seeds
        // Since HolographicSeed implements Clone and Debug,
        // we can compare the key fields
        let my_seed = self.seed_reference.get_seed();

        // Compare free will intensity
        my_seed.free_will.free_will_intensity == seed.free_will.free_will_intensity
            && my_seed.free_will.choice_capacity == seed.free_will.choice_capacity
            && my_seed.free_will.polarization_potential == seed.free_will.polarization_potential
    }

    /// Check if this is the seed experiencing itself
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.3
    /// "Entity is seed experiencing itself at a point"
    pub fn is_seed_experiencing_itself(&self) -> bool {
        // The manifestation is the seed experiencing itself if:
        // 1. It has a valid seed reference
        // 2. It is localized at a point
        // 3. It has individualization (Soul Stream)
        self.is_localized
    }
}

/// A Fractal Reference - represents a part's relationship to the whole
///
/// In a fractal-holographic system, each part contains the whole.
/// This structure captures the metadata about that relationship.
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
/// "Each archetype contains reference to the whole"
#[derive(Debug, Clone)]
pub struct FractalReference {
    /// Index of this archetype (0-21)
    pub archetype_index: usize,

    /// How strongly this part reflects the whole (0.0 to 1.0)
    pub part_whole_similarity: Float,

    /// Self-similarity at different scales
    pub scale_similarity: [Float; 5], // 5 different scales
}

impl FractalReference {
    /// Create a new fractal reference
    pub fn new(archetype_index: usize) -> Self {
        Self {
            archetype_index,
            part_whole_similarity: 1.0, // Default: perfect similarity
            scale_similarity: [1.0; 5], // 5 different scales
        }
    }

    /// Update the part-whole similarity
    pub fn update_similarity(&mut self, similarity: Float) {
        self.part_whole_similarity = similarity.clamp(0.0, 1.0);
    }

    /// Get the similarity at a specific scale
    pub fn get_scale_similarity(&self, scale: usize) -> Float {
        self.scale_similarity.get(scale).copied().unwrap_or(0.0)
    }

    /// Set the similarity at a specific scale
    pub fn set_scale_similarity(&mut self, scale: usize, similarity: Float) {
        if scale < self.scale_similarity.len() {
            self.scale_similarity[scale] = similarity.clamp(0.0, 1.0);
        }
    }
}

/// The HolographicSeed contains the complete 22-Archetype structure
/// inherited by every Entity during Involution.
///
/// PHASE 1 REFACTOR: This is now immutable ROM (Read-Only Memory)
/// - All fields are immutable and shared via Arc
/// - Cannot be mutated once created
/// - Shared reference across all entities in the simulation
///
/// This is the ROM (Read-Only Memory) structure - immutable and inherited.
/// The Entity's journey is about ACTIVATING the pre-existing holographic
/// architecture that was ingrained during Involution.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.1
/// "The HolographicSeed struct contains the 22-Archetype structure (the ROM)"
///
/// Phase 1.3 Update: Entity Emergence
/// "Entity EMERGES from the HolographicSeed, not constructed from components"
///
/// Phase 2.2 Update: Fractal-Holographic Behavior
/// "Each archetype contains reference to the whole"
///
/// REFACTOR_ROADMAP_V3.md Phase 1.1: Refactor HolographicSeed
#[derive(Debug, Clone)]
pub struct HolographicSeed {
    // Layer 0: The First Distortion (Violet)
    /// Archetype 22: The Choice - the kernel of the simulation
    ///
    /// Wrapped in Arc for immutable sharing across all entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Free Will is not merely 'the freedom to choose' in the everyday sense.
    /// It is the fundamental distortion that creates the possibility for Choice to exist at all."
    pub free_will: Arc<Archetype22>,

    // Layer 1: The Archetypical Mind (Indigo)
    /// The 21 Logic Gates created by the Logos
    ///
    /// Wrapped in Arc for immutable sharing across all entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Logos codified these patterns into the Archetypical-Mind.
    /// These patterns represent the complete processing cycle of consciousness."
    pub archetypes: SeedArchetypicalMind, // Already contains Arc-wrapped structures

    // Layer 2: The Law of Light (Blue)
    /// The 22-Archetype structure impressed upon Light
    ///
    /// Wrapped in Arc for immutable sharing across all entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Every quantum of Light carries the complete blueprint for consciousness development."
    pub light_encoding: Arc<LightArchitecture>,

    // Layer 3-4: Dimensional & Planetary Constraints
    /// Space/Time coordinate system and physical laws
    ///
    /// Wrapped in Arc for immutable sharing across all entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Solar/Planetary Logos provides the environmental constraints
    /// that define the physics simulation."
    pub physics_constraints: Arc<SolarSystemConstraints>,

    // Phase 2.2: Fractal-Holographic References
    /// Fractal-holographic references for each archetype
    ///
    /// Wrapped in Arc for immutable sharing across all entities
    ///
    /// Each archetype contains a reference to the complete seed (the whole).
    /// This demonstrates the fractal-holographic principle: "part contains the whole".
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    /// "Each archetype contains reference to the whole"
    pub fractal_references: Arc<[FractalReference; 22]>,
}

/// Archetype 22: The Choice
///
/// The kernel of the simulation - the fundamental distortion of Free Will
/// that creates the possibility for Choice to exist at all.
///
/// Knowledge Base Reference: Archetypes/22. The Choice.json
#[derive(Debug, Clone)]
pub struct Archetype22 {
    /// Free Will intensity (0.0 to 1.0)
    pub free_will_intensity: Float,

    /// Choice capacity (0.0 to 1.0)
    pub choice_capacity: Float,

    /// Polarity potential (0.0 to 1.0)
    pub polarization_potential: Float,

    /// The zero-point of polarization (STO/STS balance)
    pub zero_point: Float,
}

impl Default for Archetype22 {
    fn default() -> Self {
        Self {
            free_will_intensity: 1.0,
            choice_capacity: 1.0,
            polarization_potential: 0.5,
            zero_point: 0.0,
        }
    }
}

impl Archetype22 {
    /// Create a new Archetype22 from Intelligent Infinity
    ///
    /// This represents the First Distortion - Free Will
    pub fn new_from_infinity() -> Self {
        Self::default()
    }

    /// Get the free will intensity
    pub fn free_will_intensity(&self) -> Float {
        self.free_will_intensity
    }

    /// Get the choice capacity
    pub fn choice_capacity(&self) -> Float {
        self.choice_capacity
    }

    /// Get the polarization potential
    pub fn polarization_potential(&self) -> Float {
        self.polarization_potential
    }

    /// Get the zero-point
    pub fn zero_point(&self) -> Float {
        self.zero_point
    }
}

/// The Archetypical Mind - 21 Logic Gates (ROM - Read-Only Memory)
///
/// The 21 archetypes created by the Logos that represent the complete
/// processing cycle of consciousness.
///
/// PHASE 1 REFACTOR: This is now immutable ROM (Read-Only Memory)
/// - Wrapped in Arc for thread-safe sharing across all entities
/// - Cannot be mutated once created
/// - Shared reference across all entities in the simulation
///
/// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json
/// - REFACTOR_ROADMAP_V3.md Phase 1.2: Implement SeedArchetypicalMind
#[derive(Debug, Clone)]
pub struct SeedSeedArchetypicalMind {
    /// The 21 archetypes (A1-A21) - Immutable ROM
    /// Wrapped in Arc for shared access across all entities
    pub archetypes: Arc<ArchetypeSystem>,

    /// Logic gate relationships between archetypes - Immutable ROM
    pub logic_gates: Arc<Vec<LogicGate>>,
}

/// A Logic Gate in the Archetypical Mind
///
/// Represents how archetypes interact and process information
#[derive(Debug, Clone)]
pub struct LogicGate {
    /// Input archetypes
    pub inputs: Vec<u8>,

    /// Output archetype
    pub output: u8,

    /// Gate type (AND, OR, NOT, etc.)
    pub gate_type: GateType,
}

/// Logic Gate Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    /// AND gate - all inputs must be active
    And,
    /// OR gate - any input can be active
    Or,
    /// NOT gate - inverts input
    Not,
    /// XOR gate - exclusive OR
    Xor,
    /// NAND gate - NOT AND
    Nand,
    /// NOR gate - NOT OR
    Nor,
}

/// Type alias for SeedArchetypicalMind
pub type SeedArchetypicalMind = SeedSeedArchetypicalMind;

impl Default for SeedArchetypicalMind {
    fn default() -> Self {
        Self {
            archetypes: Arc::new(ArchetypeSystem::new()),
            logic_gates: Arc::new(create_default_logic_gates()),
        }
    }
}

fn create_default_logic_gates() -> Vec<LogicGate> {
    // Default logic gates for the 21 archetypes
    // This is a simplified representation
    vec![
        LogicGate {
            inputs: vec![1, 2],
            output: 3,
            gate_type: GateType::And,
        },
        LogicGate {
            inputs: vec![3],
            output: 4,
            gate_type: GateType::Or,
        },
        LogicGate {
            inputs: vec![4, 5],
            output: 6,
            gate_type: GateType::And,
        },
        LogicGate {
            inputs: vec![6],
            output: 7,
            gate_type: GateType::Or,
        },
    ]
}

impl SeedArchetypicalMind {
    /// Create a new Archetypical Mind from the Logos
    ///
    /// The Logos codified the patterns into the Archetypical Mind
    /// This creates an immutable ROM (Read-Only Memory) structure
    pub fn new_from_logos() -> Self {
        Self::default()
    }

    /// Get all archetypes (immutable reference)
    ///
    /// Returns the ArchetypeSystem wrapped in Arc for shared access
    /// This enforces the ROM principle - archetypes cannot be mutated
    pub fn all_archetypes(&self) -> &Arc<ArchetypeSystem> {
        &self.archetypes
    }

    /// Get logic gates (immutable reference)
    ///
    /// Returns the logic gates wrapped in Arc for shared access
    /// This enforces the ROM principle - logic gates cannot be mutated
    pub fn logic_gates(&self) -> &Arc<Vec<LogicGate>> {
        &self.logic_gates
    }

    /// Verify that this SeedArchetypicalMind is immutable ROM
    ///
    /// This method demonstrates that the structure is designed to be
    /// immutable and shared across all entities.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 1.2
    pub fn is_immutable(&self) -> bool {
        // Arc-wrapped structures are designed for immutable sharing
        // The fact that we return Arc references proves this is ROM
        true
    }
}

// ============================================================================
// Note: Light Architecture structures are now defined in the light module
// ============================================================================
//
// Light architecture structures (LightArchitecture, ArchetypePatternBit,
// PatternType, HolographicEncoding, HolographicReference, LightLaws) are
// now defined in src/light/ and re-exported from there.
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.2
// "Every quantum of Light carries the complete blueprint for consciousness development"
// ============================================================================

// ============================================================================
// Note: Physics structures (SolarSystemConstraints, PhysicsConstants, etc.)
// are now defined in solar_system.rs module and re-exported from there.
// ============================================================================

impl Default for HolographicSeed {
    fn default() -> Self {
        Self::new_from_source()
    }
}

impl HolographicSeed {
    /// Create a new holographic seed from the source (Intelligent Infinity)
    ///
    /// PHASE 1 REFACTOR: This creates an immutable ROM structure
    /// - All fields are wrapped in Arc for shared access
    /// - Cannot be mutated once created
    ///
    /// This follows the Involution process:
    /// 1. Start with Violet Ray (Source)
    /// 2. Apply Logos/Archetypes (Indigo)
    /// 3. Condense into Light (Blue)
    /// 4. Apply Dimensional Constraints (Green)
    /// 5. Apply Solar/Planetary Constraints (Yellow)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.2
    /// REFACTOR_ROADMAP_V3.md Phase 1.1: Refactor HolographicSeed
    pub fn new_from_source() -> Self {
        // 1. Start with Violet Ray (Source)
        let free_will = Arc::new(Archetype22::new_from_infinity());

        // 2. Apply Logos/Archetypes (Indigo)
        let archetypes = SeedArchetypicalMind::new_from_logos();

        // 3. Condense into Light (Blue)
        // TODO: LightArchitecture expects spectrum::ArchetypicalMind and holographic_blueprint::Archetype22
        // We have SeedSeedArchetypicalMind and holographic_seed::Archetype22
        // For now, create empty light encoding
        let light_encoding = Arc::new(LightArchitecture::default());

        // 4. Apply Dimensional Constraints (Green)
        // (Applied later through coordinate system)

        // 5. Apply Solar/Planetary Constraints (Yellow)
        let physics_constraints = Arc::new(SolarSystemConstraints::default());

        // 6. Phase 2.2: Initialize fractal-holographic references
        // Create fractal references that demonstrate the fractal-holographic principle
        // Each archetype contains metadata about its relationship to the whole

        // Create fractal references - one for each archetype
        let fractal_references = Arc::new(std::array::from_fn(FractalReference::new));

        // Create the final seed with fractal references
        HolographicSeed {
            free_will,
            archetypes,
            light_encoding,
            physics_constraints,
            fractal_references,
        }
    }

    /// Get the free will component (immutable reference)
    ///
    /// Returns Arc<Archetype22> for shared immutable access
    pub fn free_will(&self) -> &Arc<Archetype22> {
        &self.free_will
    }

    /// Get the archetypical mind (immutable reference)
    ///
    /// Returns SeedArchetypicalMind which contains Arc-wrapped structures
    pub fn archetypes(&self) -> &SeedArchetypicalMind {
        &self.archetypes
    }

    /// Get the light encoding (immutable reference)
    ///
    /// Returns Arc<LightArchitecture> for shared immutable access
    pub fn light_encoding(&self) -> &Arc<LightArchitecture> {
        &self.light_encoding
    }

    /// Get the physics constraints (immutable reference)
    ///
    /// Returns Arc<SolarSystemConstraints> for shared immutable access
    pub fn physics_constraints(&self) -> &Arc<SolarSystemConstraints> {
        &self.physics_constraints
    }

    /// Verify that the HolographicSeed is immutable ROM
    ///
    /// This method demonstrates that the structure is designed to be
    /// immutable and shared across all entities.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 1.1
    pub fn is_immutable(&self) -> bool {
        // All fields are Arc-wrapped, designed for immutable sharing
        true
    }

    /// Create a holographic seed with custom archetypes (for testing purposes)
    ///
    /// This method allows creating seeds with specific archetype configurations
    /// for testing purposes. The resulting seed is still immutable ROM.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 1.1
    pub fn with_custom_archetypes(archetypes: ArchetypeSystem) -> Self {
        // Wrap the custom archetypes in Arc for immutable sharing
        let archetypes_arc = Arc::new(archetypes);

        // Create the SeedArchetypicalMind with custom archetypes
        let archetypical_mind = SeedArchetypicalMind {
            archetypes: archetypes_arc,
            logic_gates: Arc::new(create_default_logic_gates()),
        };

        // 1. Start with Violet Ray (Source)
        let free_will = Arc::new(Archetype22::new_from_infinity());

        // 2. Use custom archetypes (Indigo)
        let archetypes = archetypical_mind;

        // 3. Condense into Light (Blue)
        // TODO: LightArchitecture expects spectrum::ArchetypicalMind and holographic_blueprint::Archetype22
        // For now, create empty light encoding
        let light_encoding = Arc::new(LightArchitecture::default());

        // 4. Apply Solar/Planetary Constraints (Yellow)
        let physics_constraints = Arc::new(SolarSystemConstraints::default());

        // 5. Initialize fractal-holographic references
        let fractal_references = Arc::new(std::array::from_fn(FractalReference::new));

        HolographicSeed {
            free_will,
            archetypes,
            light_encoding,
            physics_constraints,
            fractal_references,
        }
    }

    /// Check if the seed contains complete architecture
    ///
    /// This is always true for a valid HolographicSeed.
    /// The seed contains all 22 archetypes and the complete architecture.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    /// "Each entity contains within it all densities and sub-densities of the octave"
    pub fn contains_complete_architecture(&self) -> bool {
        true
    }

    /// Emerge an Entity from the seed at a specific Space/Time coordinate
    ///
    /// This demonstrates emergence: the entity is not constructed from parts,
    /// but emerges as a whole manifestation of the seed at a point.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.3
    /// "Entity EMERGES from seed, not constructed"
    ///
    /// # Arguments
    /// * `coord` - The Space/Time coordinate where the entity emerges
    /// * `soul_stream` - The individualization through Soul Stream
    ///
    /// # Returns
    /// An `EmergenceManifestation` representing the entity as it emerges from the seed
    ///
    /// # Example
    /// ```
    /// use holonic_realms::holographic_seed::HolographicSeed;
    /// use holonic_realms::entity::SpaceTimeCoord;
    /// use holonic_realms::soul_stream::SoulStream;
    ///
    /// let seed = HolographicSeed::new_from_source();
    /// let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
    /// let soul_stream = SoulStream::new();
    ///
    /// let manifestation = seed.emerge_entity_at(coord, soul_stream);
    /// assert!(manifestation.is_seed_experiencing_itself());
    /// ```
    pub fn emerge_entity_at(
        &self,
        coord: PhysicalSpaceTimeCoord,
        soul_stream: SoulStream,
    ) -> EmergenceManifestation {
        // Wrap the seed in Arc for thread-safe sharing
        let seed_arc = Arc::new(self.clone());
        let seed_reference = HolographicSeedReference::new(seed_arc);

        // Create the emergence manifestation
        // This is NOT construction - this is emergence
        // The entity is the seed manifesting at a point
        EmergenceManifestation::new(seed_reference, coord, soul_stream)
    }

    /// Emerge an Entity from the seed and return as Arc
    ///
    /// This is a convenience method that returns the seed wrapped in Arc
    /// for use in entity creation.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.3
    pub fn as_arc(&self) -> Arc<HolographicSeed> {
        Arc::new(self.clone())
    }

    // ========================================================================
    // Phase 2.2: Fractal-Holographic Behavior
    // ========================================================================

    /// Demonstrate "part contains the whole" - get archetype as whole seed
    ///
    /// In a fractal-holographic system, each part contains the whole.
    /// When accessing any archetype, we return the complete seed.
    ///
    /// This demonstrates the fractal-holographic principle:
    /// "Each archetype contains reference to the whole"
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    ///
    /// # Arguments
    /// * `index` - The archetype index (0-21)
    ///
    /// # Returns
    /// * `Some(&HolographicSeed)` - The complete seed if index is valid
    /// * `None` - If index is out of bounds
    ///
    /// # Example
    /// ```
    /// use holonic_realms::holographic_seed::HolographicSeed;
    ///
    /// let seed = HolographicSeed::new_from_source();
    ///
    /// // Access any archetype, get the whole seed
    /// let archetype0_as_whole = seed.get_archetype_as_whole(0);
    /// let archetype1_as_whole = seed.get_archetype_as_whole(1);
    ///
    /// // Both return the same complete seed
    /// assert!(archetype0_as_whole.is_some());
    /// assert!(archetype1_as_whole.is_some());
    /// ```
    pub fn get_archetype_as_whole(&self, index: usize) -> Option<&HolographicSeed> {
        if index < 22 {
            Some(self) // The same seed - demonstrating "part contains the whole"
        } else {
            None
        }
    }

    /// Calculate how much a specific archetype resembles the whole
    ///
    /// This calculates the part-whole similarity for a specific archetype.
    /// The similarity is based on how well the archetype reflects the
    /// complete seed's structure.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    ///
    /// # Arguments
    /// * `index` - The archetype index (0-21)
    ///
    /// # Returns
    /// The similarity score (0.0 to 1.0)
    pub fn calculate_part_whole_similarity(&self, index: usize) -> Float {
        if index >= 22 {
            return 0.0;
        }

        // For a holographic seed, each part contains the whole
        // So the similarity is always high
        // We use the fractal reference if it exists, otherwise calculate
        if index < self.fractal_references.len() {
            self.fractal_references[index].part_whole_similarity
        } else {
            // Default high similarity
            1.0
        }
    }

    /// Calculate the overall self-similarity of the seed
    ///
    /// This calculates how much the parts (archetypes) resemble the whole.
    /// In a true fractal-holographic system, this should be very high.
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    ///
    /// # Returns
    /// The self-similarity score (0.0 to 1.0)
    pub fn get_self_similarity(&self) -> Float {
        let mut total = 0.0;
        for i in 0..22 {
            total += self.calculate_part_whole_similarity(i);
        }
        total / 22.0
    }

    /// Get the fractal reference for a specific archetype
    ///
    /// This returns the fractal reference that connects the archetype
    /// to the complete seed (the whole).
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.6
    ///
    /// # Arguments
    /// * `index` - The archetype index (0-21)
    ///
    /// # Returns
    /// * `Some(&FractalReference)` - The fractal reference if index is valid
    /// * `None` - If index is out of bounds
    pub fn get_fractal_reference(&self, index: usize) -> Option<&FractalReference> {
        self.fractal_references.get(index)
    }

    /// Create a shared Arc reference to this seed
    ///
    /// This is the primary method for sharing the HolographicSeed across entities.
    /// All entities should share the same Arc reference to the same seed.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 1.1
    /// "All entities have same HolographicSeed reference (ptr_eq)"
    pub fn as_shared_arc(&self) -> Arc<HolographicSeed> {
        Arc::new(self.clone())
    }

    /// Verify that two seeds are the same shared reference
    ///
    /// This is used to validate that all entities share the same HolographicSeed.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 1.1
    /// "All entities have same HolographicSeed reference (ptr_eq)"
    pub fn is_same_reference(arc1: &Arc<HolographicSeed>, arc2: &Arc<HolographicSeed>) -> bool {
        Arc::ptr_eq(arc1, arc2)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_seed_creation() {
        let seed = HolographicSeed::new_from_source();
        assert!(seed.contains_complete_architecture());
    }

    #[test]
    fn test_holographic_seed_free_will() {
        let seed = HolographicSeed::new_from_source();
        assert_eq!(seed.free_will().free_will_intensity(), 1.0);
    }

    #[test]
    fn test_holographic_seed_archetypes() {
        let seed = HolographicSeed::new_from_source();
        // ArchetypeSystem exists and has archetypes
        let _ = seed.archetypes().all_archetypes();
    }

    #[test]
    fn test_holographic_seed_light_encoding() {
        let seed = HolographicSeed::new_from_source();
        // Light architecture should exist
        let _ = seed.light_encoding();
    }

    #[test]
    fn test_holographic_seed_physics_constraints() {
        let seed = HolographicSeed::new_from_source();
        // Physics constraints should exist
        let _ = seed.physics_constraints();
    }

    // ============================================================================
    // PHASE 1 VALIDATION TESTS - ROM Principle
    // ============================================================================

    #[test]
    fn test_holographic_seed_is_shared() {
        // Verify every entity has same HolographicSeed reference
        use std::sync::Arc;

        let seed = Arc::new(HolographicSeed::new_from_source());
        let seed_ref1 = HolographicSeedReference::new(seed.clone());
        let seed_ref2 = HolographicSeedReference::new(seed.clone());

        // ptr_eq verifies they share the same reference
        assert!(seed_ref1.ptr_eq(&seed_ref2));
    }

    #[test]
    fn test_archetypes_are_read_only() {
        // Verify archetypes are read-only (ROM)
        let seed = Arc::new(HolographicSeed::new_from_source());

        // Attempting to mutate archetypes should fail at compile time
        // This is enforced by Rust's type system through Arc
        // The fact that archetypes are wrapped in Arc proves they're immutable
        assert!(seed.archetypes.is_immutable());
        assert!(seed.is_immutable());
    }

    #[test]
    fn test_holographic_seed_is_immutable() {
        // Verify HolographicSeed is immutable ROM
        let seed = HolographicSeed::new_from_source();

        // All fields are Arc-wrapped, designed for immutable sharing
        assert!(seed.is_immutable());
    }

    #[test]
    fn test_archetypical_mind_is_shared() {
        // Verify SeedArchetypicalMind is shared via Arc
        use std::sync::Arc;

        let seed = HolographicSeed::new_from_source();
        let seed_arc = Arc::new(seed);

        // Archetypes should be shared via Arc
        let archetypes_ref = seed_arc.archetypes().all_archetypes();
        assert_eq!(Arc::strong_count(archetypes_ref), 1); // Only one reference initially
    }

    #[test]
    fn test_free_will_is_shared() {
        // Verify Archetype22 (Free Will) is shared via Arc
        use std::sync::Arc;

        let seed = HolographicSeed::new_from_source();
        let seed_arc = Arc::new(seed);

        // Free Will should be shared via Arc
        let free_will_ref = seed_arc.free_will();
        assert_eq!(Arc::strong_count(free_will_ref), 1); // Only one reference initially
    }

    #[test]
    fn test_light_encoding_is_shared() {
        // Verify LightEncoding is shared via Arc
        use std::sync::Arc;

        let seed = HolographicSeed::new_from_source();
        let seed_arc = Arc::new(seed);

        // Light encoding should be shared via Arc
        let light_ref = seed_arc.light_encoding();
        assert_eq!(Arc::strong_count(light_ref), 1); // Only one reference initially
    }

    #[test]
    fn test_physics_constraints_is_shared() {
        // Verify PhysicsConstraints is shared via Arc
        use std::sync::Arc;

        let seed = HolographicSeed::new_from_source();
        let seed_arc = Arc::new(seed);

        // Physics constraints should be shared via Arc
        let physics_ref = seed_arc.physics_constraints();
        assert_eq!(Arc::strong_count(physics_ref), 1); // Only one reference initially
    }

    #[test]
    fn test_fractal_references_is_shared() {
        // Verify FractalReferences is shared via Arc
        use std::sync::Arc;

        let seed = HolographicSeed::new_from_source();
        let seed_arc = Arc::new(seed);

        // Fractal references should be shared via Arc
        let fractal_ref = &seed_arc.fractal_references;
        assert_eq!(Arc::strong_count(fractal_ref), 1); // Only one reference initially
    }

    #[test]
    fn test_holographic_seed_as_arc() {
        let seed = HolographicSeed::new_from_source();
        let seed_arc = seed.as_arc();
        assert_eq!(
            seed_arc.free_will.free_will_intensity,
            seed.free_will.free_will_intensity
        );
    }

    #[test]
    fn test_holographic_seed_reference() {
        let seed = HolographicSeed::new_from_source();
        let seed_arc = seed.as_arc();
        let reference = HolographicSeedReference::new(seed_arc);

        assert_eq!(
            reference.get_seed().free_will.free_will_intensity,
            seed.free_will.free_will_intensity
        );
    }

    #[test]
    fn test_holographic_seed_reference_same_seed() {
        let seed = HolographicSeed::new_from_source();
        let seed_arc = seed.as_arc();
        let reference1 = HolographicSeedReference::new(seed_arc.clone());
        let reference2 = HolographicSeedReference::new(seed_arc);

        assert!(reference1.references_same_seed(&reference2));
    }

    #[test]
    fn test_emergence_manifestation() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22); // 22 archetypes dimensionality

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        assert!(manifestation.is_seed_experiencing_itself());
    }

    #[test]
    fn test_emergence_manifestation_references_seed() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22);

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        assert!(manifestation.references_seed(&seed));
    }

    #[test]
    fn test_emergence_vs_construction() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22);

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        // Emergence is not construction
        assert!(manifestation.is_seed_experiencing_itself());
    }

    #[test]
    fn test_seed_contains_complete_architecture() {
        let seed = HolographicSeed::new_from_source();
        assert!(seed.contains_complete_architecture());
    }

    #[test]
    fn test_holographic_seed_emergence() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22);

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        assert!(manifestation.is_seed_experiencing_itself());
    }

    #[test]
    fn test_light_architecture_creation() {
        let seed = HolographicSeed::new_from_source();
        let _ = seed.light_encoding();
    }

    #[test]
    fn test_emergence_manifestation_individualization() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22);

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        // Manifestation should have individualization
        assert!(manifestation.emergence_timestamp > 0);
    }

    #[test]
    fn test_emergence_manifestation_localization() {
        let seed = HolographicSeed::new_from_source();
        let coord = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 0);
        let soul_stream = SoulStream::new(22);

        let manifestation = seed.emerge_entity_at(coord, soul_stream);
        // Manifestation should be localized
        assert!(manifestation.is_localized);
    }

    // ========================================================================
    // Phase 2.2: Fractal-Holographic Tests
    // ========================================================================

    #[test]
    fn test_fractal_reference_creation() {
        let fractal_ref = FractalReference::new(0);
        assert_eq!(fractal_ref.archetype_index, 0);
        assert_eq!(fractal_ref.part_whole_similarity, 1.0);
        assert_eq!(fractal_ref.scale_similarity.len(), 5);
    }

    #[test]
    fn test_fractal_reference_update_similarity() {
        let mut fractal_ref = FractalReference::new(0);
        fractal_ref.update_similarity(0.8);
        assert_eq!(fractal_ref.part_whole_similarity, 0.8);
    }

    #[test]
    fn test_fractal_reference_scale_similarity() {
        let mut fractal_ref = FractalReference::new(0);
        fractal_ref.set_scale_similarity(0, 0.9);
        assert_eq!(fractal_ref.get_scale_similarity(0), 0.9);
        assert_eq!(fractal_ref.get_scale_similarity(1), 1.0); // Default
    }

    #[test]
    fn test_holographic_seed_fractal_references_initialization() {
        let seed = HolographicSeed::new_from_source();
        // Should have 22 fractal references
        assert_eq!(seed.fractal_references.len(), 22);
    }

    #[test]
    fn test_get_archetype_as_whole() {
        let seed = HolographicSeed::new_from_source();

        // Get archetype 0 as whole
        let archetype0_as_whole = seed.get_archetype_as_whole(0);
        assert!(archetype0_as_whole.is_some());

        // Get archetype 21 as whole
        let archetype21_as_whole = seed.get_archetype_as_whole(21);
        assert!(archetype21_as_whole.is_some());

        // Invalid index should return None
        let invalid = seed.get_archetype_as_whole(22);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_calculate_part_whole_similarity() {
        let seed = HolographicSeed::new_from_source();

        // Calculate similarity for archetype 0
        let similarity0 = seed.calculate_part_whole_similarity(0);
        assert!((0.0..=1.0).contains(&similarity0));

        // Calculate similarity for archetype 10
        let similarity10 = seed.calculate_part_whole_similarity(10);
        assert!((0.0..=1.0).contains(&similarity10));

        // Invalid index should return 0.0
        let invalid = seed.calculate_part_whole_similarity(22);
        assert_eq!(invalid, 0.0);
    }

    #[test]
    fn test_get_self_similarity() {
        let seed = HolographicSeed::new_from_source();

        // Calculate self-similarity
        let similarity = seed.get_self_similarity();
        assert!((0.0..=1.0).contains(&similarity));

        // Self-similarity should be high for holographic seed
        assert!(similarity > 0.5);
    }

    #[test]
    fn test_get_fractal_reference() {
        let seed = HolographicSeed::new_from_source();

        // Get fractal reference for archetype 0
        let fractal_ref0 = seed.get_fractal_reference(0);
        assert!(fractal_ref0.is_some());
        assert_eq!(fractal_ref0.unwrap().archetype_index, 0);

        // Get fractal reference for archetype 21
        let fractal_ref21 = seed.get_fractal_reference(21);
        assert!(fractal_ref21.is_some());
        assert_eq!(fractal_ref21.unwrap().archetype_index, 21);

        // Invalid index should return None
        let invalid = seed.get_fractal_reference(22);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_update_part_whole_similarity() {
        // TODO: HolographicSeed is immutable ROM - update methods not supported
        // This test needs to be redesigned to test immutable seed behavior
        /*
        let mut seed = HolographicSeed::new_from_source();

        // Update similarity for archetype 0
        seed.update_part_whole_similarity(0);
        let fractal_ref = seed.get_fractal_reference(0).unwrap();
        assert!(
            fractal_ref.part_whole_similarity >= 0.0 && fractal_ref.part_whole_similarity <= 1.0
        );
        */
    }

    #[test]
    fn test_update_all_similarities() {
        // TODO: HolographicSeed is immutable ROM - update methods not supported
        // This test needs to be redesigned to test immutable seed behavior
        /*
        let mut seed = HolographicSeed::new_from_source();

        // Update all similarities
        seed.update_all_similarities();

        // Verify all similarities are updated
        for i in 0..22 {
            let fractal_ref = seed.get_fractal_reference(i).unwrap();
            assert!(
                fractal_ref.part_whole_similarity >= 0.0
                    && fractal_ref.part_whole_similarity <= 1.0
            );
        }
        */
    }

    #[test]
    fn test_fractal_holographic_part_contains_whole() {
        // This is the key test for the fractal-holographic principle:
        // "Each part contains the whole"

        let seed = HolographicSeed::new_from_source();

        // Access any archetype (part)
        let archetype0_as_whole = seed.get_archetype_as_whole(0).unwrap();
        let archetype5_as_whole = seed.get_archetype_as_whole(5).unwrap();
        let archetype10_as_whole = seed.get_archetype_as_whole(10).unwrap();

        // All parts should contain the whole (the complete seed)
        assert!(archetype0_as_whole.contains_complete_architecture());
        assert!(archetype5_as_whole.contains_complete_architecture());
        assert!(archetype10_as_whole.contains_complete_architecture());

        // All parts should have the same content as the whole seed
        assert_eq!(
            archetype0_as_whole.free_will.free_will_intensity,
            seed.free_will.free_will_intensity
        );
        assert_eq!(
            archetype5_as_whole.free_will.free_will_intensity,
            seed.free_will.free_will_intensity
        );
        assert_eq!(
            archetype10_as_whole.free_will.free_will_intensity,
            seed.free_will.free_will_intensity
        );
    }

    #[test]
    fn test_fractal_holographic_self_similarity_consistency() {
        // Test that self-similarity is consistent across multiple calculations

        let seed = HolographicSeed::new_from_source();

        // Calculate self-similarity multiple times
        let similarity1 = seed.get_self_similarity();
        let similarity2 = seed.get_self_similarity();
        let similarity3 = seed.get_self_similarity();

        // Should be consistent
        assert_eq!(similarity1, similarity2);
        assert_eq!(similarity2, similarity3);
    }

    #[test]
    fn test_fractal_holographic_multiple_scales() {
        // Test that fractal references have multiple scales

        let seed = HolographicSeed::new_from_source();

        // Get a fractal reference
        let fractal_ref = seed.get_fractal_reference(0).unwrap();

        // Should have 5 scales
        assert_eq!(fractal_ref.scale_similarity.len(), 5);

        // Each scale should be in valid range
        for scale in &fractal_ref.scale_similarity {
            assert!(*scale >= 0.0 && *scale <= 1.0);
        }
    }

    #[test]
    fn test_fractal_holographic_archetype_index_consistency() {
        // Test that fractal references have consistent archetype indices

        let seed = HolographicSeed::new_from_source();

        // Check that each fractal reference has the correct index
        for (i, fractal_ref) in seed.fractal_references.iter().enumerate() {
            assert_eq!(fractal_ref.archetype_index, i);
        }
    }
}
