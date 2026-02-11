// Holographic Archetypical Mind Architecture
//
// This module implements the hierarchical archetypical mind system where each entity
// contains the entire archetypical mind hierarchy, holographically connected.
//
// Architecture:
// - Level 1: CosmicMind (immutable template with 22 archetypes)
// - Level 2: LogosMind (refined by Logos bias, shared)
// - Level 3: SubLogosMind (refined by sub-Logos bias, shared)
// - Level 4: EntityArchetypicalMind (refined by entity bias, unique)
//
// Key Principles:
// - Each entity contains all 4 levels of archetypical mind
// - Entity mind is unique (refined by entity bias)
// - Holographic connections enable non-local influence
// - "Any portion contains the whole"
//
// ============================================================================
// MIGRATION: Phase 4, Migration 6
// This module has been migrated to: src/spectrum/archetypical_mind.rs
// The types below are re-exported for backward compatibility.
// ============================================================================

// Re-export all types from the new location
pub use crate::spectrum::archetypical_mind::{
    ArchetypeInstance,

    // Template and instance types
    ArchetypeTemplate,
    // Bias and link types
    BiasVector,
    // Hierarchical mind types
    CosmicMind,
    EntityArchetypicalMind,
    HolographicLink,

    // Enum types
    InfluenceDirection,

    LogosMind,
    SubLogosMind,
};

// ============================================================================
// OLD DEFINITIONS (commented out for reference during migration)
// ============================================================================
/*

use crate::holon::HolonID;
use crate::types::Float;

// Re-exports for use in EntityArchetypicalMind methods
use crate::archetypes::a22_choice::ChoiceArchetype as ChoiceArchetypeType;
use crate::attractor_pattern_system::ArchetypeID;

// ArchetypeType enum for identifying archetype types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeType {
    Matrix,
    Potentiator,
    Catalyst,
    Experience,
    Significator,
    Transformation,
    GreatWay,
}

// ============================================================================
// Archetype Template and Instance
// ============================================================================

/// Archetype Template (immutable)
///
/// The universal pattern for an archetype, existing at the Cosmic Mind level.
/// This is the unmodified, perfect template from which all refinements derive.
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeTemplate {
    /// Archetype ID (1-22)
    pub id: u8,
    /// Archetype name
    pub name: String,
    /// Base activation level (0.0 to 1.0)
    pub base_activation: Float,
    /// Base lambda value (0.0 to 1.0)
    pub base_lambda: Float,
    /// Complex type (Mind, Body, Spirit)
    pub complex_type: ComplexType,
}

impl ArchetypeTemplate {
    /// Create a new archetype template
    pub fn new(
        id: u8,
        name: String,
        base_activation: Float,
        base_lambda: Float,
        complex_type: ComplexType,
    ) -> Self {
        Self {
            id,
            name,
            base_activation,
            base_lambda,
            complex_type,
        }
    }
}

/// Archetype Instance (refined)
///
/// A refined instance of an archetype, modified by bias vectors at various levels.
/// Each level in the hierarchy refines the archetypes with its unique bias.
#[derive(Debug, Clone, PartialEq)]
pub struct Archetype {
    /// Reference to the template
    pub template_id: u8,
    /// Refined activation level (0.0 to 1.0)
    pub activation: Float,
    /// Refined lambda value (0.0 to 1.0)
    pub lambda: Float,
    /// Bias applied to this archetype (cumulative from all levels)
    pub cumulative_bias: Float,
    /// Complex type (inherited from template)
    pub complex_type: ComplexType,
}

impl Archetype {
    /// Create a new archetype instance from a template
    pub fn from_template(template: &ArchetypeTemplate) -> Self {
        Self {
            template_id: template.id,
            activation: template.base_activation,
            lambda: template.base_lambda,
            cumulative_bias: 0.0,
            complex_type: template.complex_type,
        }
    }

    /// Apply bias to this archetype
    pub fn apply_bias(&mut self, bias: Float) {
        self.cumulative_bias += bias;

        // Bias affects activation and lambda
        // Positive bias increases activation and lambda
        // Negative bias decreases activation and lambda
        self.activation = (self.activation + bias * 0.1).clamp(0.0, 1.0);
        self.lambda = (self.lambda + bias * 0.1).clamp(0.0, 1.0);
    }
}

// ============================================================================
// Bias Vector
// ============================================================================

/// Bias Vector (22-dimensional)
///
/// A 22-dimensional vector representing the bias applied to each archetype.
/// Used at each level of the hierarchy to refine the archetypes with unique perspectives.
///
/// Bias values range from -1.0 to 1.0:
/// - Negative bias: Service-to-self (STS) tendency
/// - Positive bias: Service-to-others (STO) tendency
/// - Zero bias: Neutral (no preference)
#[derive(Debug, Clone, PartialEq)]
pub struct BiasVector {
    /// 22-dimensional bias vector (one for each archetype)
    pub biases: [Float; 22],
}

impl BiasVector {
    /// Create a new bias vector with all zeros (neutral)
    pub fn new() -> Self {
        Self { biases: [0.0; 22] }
    }

    /// Create a bias vector with random variations based on a seed
    ///
    /// Note: This is not random in the artificial sense. The variations are
    /// deterministic based on the seed, representing unique configurations
    /// that emerge from free will capacity rather than injected randomness.
    pub fn from_seed(seed: u64) -> Self {
        let mut biases = [0.0; 22];

        // Use deterministic pseudo-random based on seed
        for i in 0..22 {
            let combined = (seed as Float) * (i as Float + 1.0);
            let value = (combined.sin() * 2.0).fract() - 0.5;
            biases[i] = value.clamp(-1.0, 1.0);
        }

        Self { biases }
    }

    /// Create a bias vector with specific values
    pub fn from_values(biases: [Float; 22]) -> Self {
        Self { biases }
    }

    /// Set bias for a specific archetype
    pub fn set_bias(&mut self, archetype_id: u8, bias: Float) {
        if archetype_id >= 1 && archetype_id <= 22 {
            self.biases[(archetype_id - 1) as usize] = bias.clamp(-1.0, 1.0);
        }
    }

    /// Get bias for a specific archetype
    pub fn get_bias(&self, archetype_id: u8) -> Float {
        if archetype_id >= 1 && archetype_id <= 22 {
            self.biases[(archetype_id - 1) as usize]
        } else {
            0.0
        }
    }

    /// Refine archetypes with this bias vector
    ///
    /// Takes a set of archetypes and applies the bias vector to create refined versions.
    /// This is the core mechanism of hierarchical refinement.
    pub fn refine_archetypes(&self, templates: &[ArchetypeTemplate; 22]) -> [Archetype; 22] {
        let mut refined = core::array::from_fn(|i| Archetype::from_template(&templates[i]));

        for i in 0..22 {
            refined[i].apply_bias(self.biases[i]);
        }

        refined
    }

    /// Refine existing archetypes with this bias vector
    ///
    /// Takes a set of already-refined archetypes and applies additional bias.
    /// This is used for cumulative refinement through the hierarchy.
    pub fn refine_existing_archetypes(&self, archetypes: &[Archetype; 22]) -> [Archetype; 22] {
        let mut refined = archetypes.clone();

        for i in 0..22 {
            refined[i].apply_bias(self.biases[i]);
        }

        refined
    }

    /// Calculate the magnitude of the bias vector
    pub fn magnitude(&self) -> Float {
        let sum: Float = self.biases.iter().map(|b| b * b).sum();
        sum.sqrt()
    }

    /// Normalize the bias vector to unit magnitude
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            for bias in &mut self.biases {
                *bias /= mag;
            }
        }
    }
}

impl Default for BiasVector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Holographic Link
// ============================================================================

/// Holographic Link (non-local connection between entities)
///
/// Represents a non-local connection between entities in the holographic field.
/// Changes in one entity can propagate to another through these links.
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicLink {
    /// Target entity ID
    pub target_entity_id: HolonID,
    /// Resonance strength (0.0 to 1.0)
    pub resonance: Float,
    /// Direction of influence
    pub influence_direction: InfluenceDirection,
    /// Last synchronization timestamp
    pub last_sync: Float,
}

impl HolographicLink {
    /// Create a new holographic link
    pub fn new(target_entity_id: HolonID, resonance: Float) -> Self {
        Self {
            target_entity_id,
            resonance: resonance.clamp(0.0, 1.0),
            influence_direction: InfluenceDirection::Bidirectional,
            last_sync: 0.0,
        }
    }

    /// Update the last sync timestamp
    pub fn update_sync(&mut self, timestamp: Float) {
        self.last_sync = timestamp;
    }
}

/// Direction of influence in holographic link
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfluenceDirection {
    /// Both entities influence each other
    Bidirectional,
    /// Only source influences target
    ToTarget,
    /// Only target influences source
    FromTarget,
}

// ============================================================================
// Cosmic Mind (Level 1)
// ============================================================================

/// Cosmic Mind (immutable template)
///
/// The first level of the archetypical mind hierarchy, containing the immutable
/// templates for all 22 archetypes. This is the universal pattern from which
/// all other levels derive.
#[derive(Debug, Clone, PartialEq)]
pub struct CosmicMind {
    /// 22 archetype templates (immutable)
    pub archetypes: [ArchetypeTemplate; 22],
}

impl CosmicMind {
    /// Create a new cosmic mind with default archetype templates
    pub fn new() -> Self {
        Self {
            archetypes: Self::create_default_templates(),
        }
    }

    /// Create default archetype templates
    fn create_default_templates() -> [ArchetypeTemplate; 22] {
        // Mind Complex (A1-A7)
        let a1 =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);
        let a2 = ArchetypeTemplate::new(
            2,
            "Potentiator of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );
        let a3 = ArchetypeTemplate::new(
            3,
            "Catalyst of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );
        let a4 = ArchetypeTemplate::new(
            4,
            "Experience of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );
        let a5 = ArchetypeTemplate::new(
            5,
            "Significator of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );
        let a6 = ArchetypeTemplate::new(
            6,
            "Transformation of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );
        let a7 = ArchetypeTemplate::new(
            7,
            "Great Way of Mind".to_string(),
            0.5,
            0.5,
            ComplexType::Mind,
        );

        // Body Complex (A8-A14)
        let a8 =
            ArchetypeTemplate::new(8, "Matrix of Body".to_string(), 0.5, 0.5, ComplexType::Body);
        let a9 = ArchetypeTemplate::new(
            9,
            "Potentiator of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );
        let a10 = ArchetypeTemplate::new(
            10,
            "Catalyst of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );
        let a11 = ArchetypeTemplate::new(
            11,
            "Experience of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );
        let a12 = ArchetypeTemplate::new(
            12,
            "Significator of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );
        let a13 = ArchetypeTemplate::new(
            13,
            "Transformation of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );
        let a14 = ArchetypeTemplate::new(
            14,
            "Great Way of Body".to_string(),
            0.5,
            0.5,
            ComplexType::Body,
        );

        // Spirit Complex (A15-A21)
        let a15 = ArchetypeTemplate::new(
            15,
            "Matrix of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a16 = ArchetypeTemplate::new(
            16,
            "Potentiator of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a17 = ArchetypeTemplate::new(
            17,
            "Catalyst of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a18 = ArchetypeTemplate::new(
            18,
            "Experience of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a19 = ArchetypeTemplate::new(
            19,
            "Significator of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a20 = ArchetypeTemplate::new(
            20,
            "Transformation of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );
        let a21 = ArchetypeTemplate::new(
            21,
            "Great Way of Spirit".to_string(),
            0.5,
            0.5,
            ComplexType::Spirit,
        );

        // Unified Archetype (A22)
        let a22 = ArchetypeTemplate::new(22, "Choice".to_string(), 0.5, 0.5, ComplexType::Mind);

        [
            a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19,
            a20, a21, a22,
        ]
    }

    /// Get archetype template by ID
    pub fn get_archetype(&self, id: u8) -> Option<&ArchetypeTemplate> {
        if id >= 1 && id <= 22 {
            Some(&self.archetypes[(id - 1) as usize])
        } else {
            None
        }
    }

    /// Get all archetype templates
    pub fn get_archetypes(&self) -> &[ArchetypeTemplate; 22] {
        &self.archetypes
    }
}

impl Default for CosmicMind {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Logos Mind (Level 2)
// ============================================================================

/// Logos Mind (refined by Logos bias)
///
/// The second level of the archetypical mind hierarchy, where the Cosmic Mind
/// is refined by the Logos's unique bias. This level is shared among all entities
/// in the same Logos system.
#[derive(Debug, Clone, PartialEq)]
pub struct LogosMind {
    /// Reference to Cosmic Mind (immutable)
    pub cosmic_mind: CosmicMind,
    /// Logos's unique bias
    pub logos_bias: BiasVector,
    /// Refined archetypes (22)
    pub refined_archetypes: [Archetype; 22],
}

impl LogosMind {
    /// Create a new Logos mind with given bias
    pub fn new(cosmic_mind: CosmicMind, logos_bias: BiasVector) -> Self {
        // Refine cosmic archetypes with Logos bias
        let refined_archetypes = logos_bias.refine_archetypes(&cosmic_mind.archetypes);

        Self {
            cosmic_mind,
            logos_bias,
            refined_archetypes,
        }
    }

    /// Create a new Logos mind with neutral bias
    pub fn with_neutral_bias(cosmic_mind: CosmicMind) -> Self {
        let logos_bias = BiasVector::new();
        Self::new(cosmic_mind, logos_bias)
    }

    /// Get refined archetype by ID
    pub fn get_archetype(&self, id: u8) -> Option<&Archetype> {
        if id >= 1 && id <= 22 {
            Some(&self.refined_archetypes[(id - 1) as usize])
        } else {
            None
        }
    }
}

// ============================================================================
// Sub-Logos Mind (Level 3)
// ============================================================================

/// Sub-Logos Mind (refined by sub-Logos bias)
///
/// The third level of the archetypical mind hierarchy, where the Logos Mind
/// is further refined by the sub-Logos's unique bias. This level is shared among
/// all entities in the same sub-Logos system.
#[derive(Debug, Clone, PartialEq)]
pub struct SubLogosMind {
    /// Reference to Logos Mind
    pub logos_mind: LogosMind,
    /// Sub-Logos's unique bias
    pub sub_logos_bias: BiasVector,
    /// Refined archetypes (22)
    pub refined_archetypes: [Archetype; 22],
}

impl SubLogosMind {
    /// Create a new sub-Logos mind with given bias
    pub fn new(logos_mind: LogosMind, sub_logos_bias: BiasVector) -> Self {
        // Refine Logos archetypes with sub-Logos bias
        let refined_archetypes =
            sub_logos_bias.refine_existing_archetypes(&logos_mind.refined_archetypes);

        Self {
            logos_mind,
            sub_logos_bias,
            refined_archetypes,
        }
    }

    /// Create a new sub-Logos mind with neutral bias
    pub fn with_neutral_bias(logos_mind: LogosMind) -> Self {
        let sub_logos_bias = BiasVector::new();
        Self::new(logos_mind, sub_logos_bias)
    }

    /// Get refined archetype by ID
    pub fn get_archetype(&self, id: u8) -> Option<&Archetype> {
        if id >= 1 && id <= 22 {
            Some(&self.refined_archetypes[(id - 1) as usize])
        } else {
            None
        }
    }

    /// Access cosmic mind through hierarchy
    pub fn access_cosmic_mind(&self) -> &CosmicMind {
        &self.logos_mind.cosmic_mind
    }

    /// Access logos mind through hierarchy
    pub fn access_logos_mind(&self) -> &LogosMind {
        &self.logos_mind
    }
}

// ============================================================================
// Entity Archetypical Mind (Level 4)
// ============================================================================

/// Entity Archetypical Mind (refined by entity bias)
///
/// The fourth and final level of the archetypical mind hierarchy, where the
/// Sub-Logos Mind is further refined by the entity's unique bias. This level
/// is unique to each entity, representing their individual perspective on the
/// archetypical patterns.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityArchetypicalMind {
    /// Reference to Sub-Logos Mind
    pub sub_logos_mind: SubLogosMind,
    /// Entity's unique bias
    pub entity_bias: BiasVector,
    /// Refined archetypes (22) - unique to this entity
    pub refined_archetypes: [Archetype; 22],
    /// Holographic connections to other entities
    pub holographic_links: Vec<HolographicLink>,
}

impl EntityArchetypicalMind {
    /// Create a new entity archetypical mind with given bias
    pub fn new(sub_logos_mind: SubLogosMind, entity_bias: BiasVector) -> Self {
        // Refine sub-Logos archetypes with entity bias
        let refined_archetypes =
            entity_bias.refine_existing_archetypes(&sub_logos_mind.refined_archetypes);

        Self {
            sub_logos_mind,
            entity_bias,
            refined_archetypes,
            holographic_links: Vec::new(),
        }
    }

    /// Create a new entity archetypical mind with neutral bias
    pub fn with_neutral_bias(sub_logos_mind: SubLogosMind) -> Self {
        let entity_bias = BiasVector::new();
        Self::new(sub_logos_mind, entity_bias)
    }

    /// Create a new entity archetypical mind from seed (deterministic)
    ///
    /// This creates a unique entity mind based on a seed value, ensuring
    /// deterministic behavior without artificial randomness.
    pub fn from_seed(sub_logos_mind: SubLogosMind, seed: u64) -> Self {
        let entity_bias = BiasVector::from_seed(seed);
        Self::new(sub_logos_mind, entity_bias)
    }

    /// Get refined archetype by ID
    pub fn get_archetype(&self, id: u8) -> Option<&Archetype> {
        if id >= 1 && id <= 22 {
            Some(&self.refined_archetypes[(id - 1) as usize])
        } else {
            None
        }
    }

    /// Access cosmic mind through hierarchy
    pub fn access_cosmic_mind(&self) -> &CosmicMind {
        &self.sub_logos_mind.logos_mind.cosmic_mind
    }

    /// Access logos mind through hierarchy
    pub fn access_logos_mind(&self) -> &LogosMind {
        &self.sub_logos_mind.logos_mind
    }

    /// Access sub-Logos mind through hierarchy
    pub fn access_sub_logos_mind(&self) -> &SubLogosMind {
        &self.sub_logos_mind
    }

    /// Add a holographic link to another entity
    pub fn add_holographic_link(&mut self, target_entity_id: HolonID, resonance: Float) {
        let link = HolographicLink::new(target_entity_id, resonance);
        self.holographic_links.push(link);
    }

    /// Get holographic links
    pub fn get_holographic_links(&self) -> &[HolographicLink] {
        &self.holographic_links
    }

    /// Calculate the total refinement depth (how many levels of refinement)
    pub fn refinement_depth(&self) -> u32 {
        4 // Cosmic -> Logos -> Sub-Logos -> Entity
    }

    /// Calculate the uniqueness of this entity mind
    ///
    /// Returns a value from 0.0 to 1.0 indicating how unique this entity's
    /// archetypical mind is compared to the cosmic template.
    pub fn calculate_uniqueness(&self) -> Float {
        let mut total_deviation = 0.0;

        for i in 0..22 {
            let cosmic_activation = self.access_cosmic_mind().archetypes[i].base_activation;
            let entity_activation = self.refined_archetypes[i].activation;
            total_deviation += (entity_activation - cosmic_activation).abs();
        }

        // Average deviation across all archetypes
        let avg_deviation = total_deviation / 22.0;

        // Cap at 1.0
        avg_deviation.min(1.0)
    }

    // ============================================================================
    // PHASE 9C: PROCESSING AND SYNCHRONIZATION
    // ============================================================================

    /// Process the entity archetypical mind
    ///
    /// This method applies holographic influences from connected entities,
    /// updates archetype activations, and generates holographic changes for propagation.
    ///
    /// Returns: Vector of holographic changes to propagate to connected entities
    pub fn process(
        &mut self,
        coupling_coefficient: Float,
        developmental_level: Float,
        free_will_capacity: Float,
        timestamp: Float,
    ) -> Vec<crate::holographic_connections::HolographicChange> {
        use crate::holographic_connections::{ChangeType, HolographicChange};

        let mut holographic_changes = Vec::new();

        // Step 1: Apply holographic influences from connected entities
        // This updates archetype activations based on non-local influence
        for link in &self.holographic_links {
            // Note: In a full implementation, we would fetch the actual archetype states
            // from the connected entity. For now, we simulate the effect.
            // The resonance determines the strength of influence.
            let influence_strength = link.resonance * coupling_coefficient * free_will_capacity;

            // Apply influence to all archetypes (simplified)
            // In a full implementation, specific archetypes would be affected based on
            // the connected entity's actual changes.
            for i in 0..22 {
                let archetype = &mut self.refined_archetypes[i];

                // Calculate influence based on archetype compatibility
                // Higher developmental level = more sensitive to influence
                // Higher free will capacity = more receptive to influence
                let sensitivity = (developmental_level / 8.0) * free_will_capacity;
                let influence = influence_strength * sensitivity * 0.01;

                // Apply influence to activation
                let old_activation = archetype.activation;
                archetype.activation = (archetype.activation + influence).clamp(0.0, 1.0);

                // Track significant changes
                if (archetype.activation - old_activation).abs() > 0.001 {
                    holographic_changes.push(HolographicChange::new(
                        (i + 1) as ArchetypeID,
                        ChangeType::ActivationChange,
                        (archetype.activation - old_activation).abs(),
                    ));
                }
            }

            // Update link sync timestamp
            // Note: We need mutable access to links, so we'll do this in a separate pass
        }

        // Update link sync timestamps
        for link in &mut self.holographic_links {
            link.update_sync(timestamp);
        }

        // Step 2: Apply entity bias to archetypes
        // This represents the entity's unique perspective influencing its archetypes
        let bias_strength = free_will_capacity * coupling_coefficient * 0.01;
        for i in 0..22 {
            let archetype = &mut self.refined_archetypes[i];
            let bias = self.entity_bias.biases[i];

            // Apply bias to activation
            let old_activation = archetype.activation;
            archetype.activation = (archetype.activation + bias * bias_strength).clamp(0.0, 1.0);

            // Track significant changes
            if (archetype.activation - old_activation).abs() > 0.001 {
                holographic_changes.push(HolographicChange::new(
                    (i + 1) as ArchetypeID,
                    ChangeType::ActivationChange,
                    (archetype.activation - old_activation).abs(),
                ));
            }
        }

        // Step 3: Update lambda values based on activation changes
        // Lambda represents the quality/depth of the archetype's expression
        for i in 0..22 {
            let archetype = &mut self.refined_archetypes[i];
            // Lambda tends to follow activation but with some inertia
            let lambda_target = archetype.activation * (0.8 + developmental_level * 0.025);
            archetype.lambda += (lambda_target - archetype.lambda) * 0.1 * coupling_coefficient;
            archetype.lambda = archetype.lambda.clamp(0.0, 1.0);
        }

        holographic_changes
    }

    /// Synchronize archetype activations from Complex-based processing
    ///
    /// This method updates the EntityArchetypicalMind archetypes to match
    /// the activation levels from the Complex-based processing.
    ///
    /// Parameters:
    /// - mind_complex: The mind complex (archetypes 1-7)
    /// - body_complex: The body complex (archetypes 8-14)
    /// - spirit_complex: The spirit complex (archetypes 15-21)
    /// - choice_archetype: The choice archetype (archetype 22)
    pub fn sync_from_complex(
        &mut self,
        _mind_complex: &crate::complex::Complex,
        _body_complex: &crate::complex::Complex,
        _spirit_complex: &crate::complex::Complex,
        _choice_archetype: &ChoiceArchetypeType,
    ) {

        // Sync mind complex (archetypes 1-7)
        // TODO: Implement synchronization from Complex archetypes
        // For now, this is a placeholder. The EntityArchetypicalMind archetypes
        // are updated through the process() method, which applies holographic influences.

        // Sync body complex (archetypes 8-14)
        // TODO: Implement synchronization from Complex archetypes

        // Sync spirit complex (archetypes 15-21)
        // TODO: Implement synchronization from Complex archetypes

        // Sync choice archetype (archetype 22)
        // TODO: Implement synchronization from Complex archetypes
    }

    /// Apply holographic changes to Complex-based processing
    ///
    /// This method applies holographic changes from EntityArchetypicalMind
    /// to the Complex-based archetypes.
    ///
    /// Parameters:
    /// - changes: Vector of holographic changes to apply
    /// - mind_complex: The mind complex (archetypes 1-7)
    /// - body_complex: The body complex (archetypes 8-14)
    /// - spirit_complex: The spirit complex (archetypes 15-21)
    /// - choice_archetype: The choice archetype (archetype 22)
    ///
    /// Note: This method is a placeholder for future implementation.
    /// For now, holographic changes are tracked but not applied to Complex archetypes.
    pub fn apply_to_complex(
        &self,
        _changes: &[crate::holographic_connections::HolographicChange],
        _mind_complex: &mut crate::complex::Complex,
        _body_complex: &mut crate::complex::Complex,
        _spirit_complex: &mut crate::complex::Complex,
        _choice_archetype: &mut ChoiceArchetypeType,
    ) {
        // Placeholder: In a full implementation, this would apply holographic changes
        // to Complex archetypes. For now, changes are tracked but not applied.
        // The Complex archetypes are processed independently, and synchronization
        // happens through sync_from_complex() instead.
    }
}

// ============================================================================
// Complex Type
// ============================================================================

/// Complex type (Mind, Body, Spirit)
///
/// Represents which complex an archetype belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    Mind,   // σA - Archetypes 1-7
    Body,   // σB - Archetypes 8-14
    Spirit, // σC - Archetypes 15-21
}

impl ComplexType {
    /// Get complex type from archetype ID
    pub fn from_archetype_id(id: u8) -> Option<Self> {
        match id {
            1..=7 => Some(ComplexType::Mind),
            8..=14 => Some(ComplexType::Body),
            15..=21 => Some(ComplexType::Spirit),
            22 => Some(ComplexType::Mind), // Choice is unified, but often grouped with Mind
            _ => None,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_template_creation() {
        let template =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);

        assert_eq!(template.id, 1);
        assert_eq!(template.name, "Matrix of Mind");
        assert_eq!(template.base_activation, 0.5);
        assert_eq!(template.base_lambda, 0.5);
        assert_eq!(template.complex_type, ComplexType::Mind);
    }

    #[test]
    fn test_archetype_from_template() {
        let template =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);

        let archetype = Archetype::from_template(&template);

        assert_eq!(archetype.template_id, 1);
        assert_eq!(archetype.activation, 0.5);
        assert_eq!(archetype.lambda, 0.5);
        assert_eq!(archetype.cumulative_bias, 0.0);
    }

    #[test]
    fn test_archetype_apply_bias() {
        let template =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);

        let mut archetype = Archetype::from_template(&template);
        archetype.apply_bias(0.3);

        assert_eq!(archetype.cumulative_bias, 0.3);
        assert_eq!(archetype.activation, 0.53); // 0.5 + 0.3 * 0.1
        assert_eq!(archetype.lambda, 0.53);
    }

    #[test]
    fn test_archetype_apply_negative_bias() {
        let template =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);

        let mut archetype = Archetype::from_template(&template);
        archetype.apply_bias(-0.3);

        assert_eq!(archetype.cumulative_bias, -0.3);
        assert_eq!(archetype.activation, 0.47); // 0.5 - 0.3 * 0.1
        assert_eq!(archetype.lambda, 0.47);
    }

    #[test]
    fn test_archetype_apply_bias_clamping() {
        let template =
            ArchetypeTemplate::new(1, "Matrix of Mind".to_string(), 0.5, 0.5, ComplexType::Mind);

        let mut archetype = Archetype::from_template(&template);

        // Test upper clamp
        archetype.apply_bias(100.0);
        assert_eq!(archetype.activation, 1.0);
        assert_eq!(archetype.lambda, 1.0);

        // Reset and test lower clamp
        let mut archetype = Archetype::from_template(&template);
        archetype.apply_bias(-100.0);
        assert_eq!(archetype.activation, 0.0);
        assert_eq!(archetype.lambda, 0.0);
    }

    #[test]
    fn test_bias_vector_creation() {
        let bias = BiasVector::new();

        assert_eq!(bias.biases.len(), 22);
        for b in &bias.biases {
            assert_eq!(*b, 0.0);
        }
    }

    #[test]
    fn test_bias_vector_from_seed() {
        let bias1 = BiasVector::from_seed(12345);
        let bias2 = BiasVector::from_seed(12345);
        let bias3 = BiasVector::from_seed(67890);

        // Same seed should produce same result
        assert_eq!(bias1, bias2);

        // Different seed should produce different result
        assert_ne!(bias1, bias3);
    }

    #[test]
    fn test_bias_vector_set_get() {
        let mut bias = BiasVector::new();

        bias.set_bias(1, 0.5);
        bias.set_bias(22, -0.3);

        assert_eq!(bias.get_bias(1), 0.5);
        assert_eq!(bias.get_bias(22), -0.3);
        assert_eq!(bias.get_bias(5), 0.0);
    }

    #[test]
    fn test_bias_vector_set_clamping() {
        let mut bias = BiasVector::new();

        bias.set_bias(1, 2.0);
        assert_eq!(bias.get_bias(1), 1.0);

        bias.set_bias(2, -2.0);
        assert_eq!(bias.get_bias(2), -1.0);
    }

    #[test]
    fn test_bias_vector_refine_archetypes() {
        let templates = CosmicMind::new().archetypes;
        let mut bias = BiasVector::new();
        bias.set_bias(1, 0.5);
        bias.set_bias(2, -0.3);

        let refined = bias.refine_archetypes(&templates);

        assert_eq!(refined[0].template_id, 1);
        assert_eq!(refined[0].cumulative_bias, 0.5);
        assert_eq!(refined[1].cumulative_bias, -0.3);
    }

    #[test]
    fn test_bias_vector_magnitude() {
        let mut bias = BiasVector::new();
        bias.set_bias(1, 1.0);
        bias.set_bias(2, 1.0);

        let mag = bias.magnitude();
        assert!(mag > 0.0);
    }

    #[test]
    fn test_bias_vector_normalize() {
        let mut bias = BiasVector::new();
        bias.set_bias(1, 1.0);
        bias.set_bias(2, 1.0);

        bias.normalize();

        let mag = bias.magnitude();
        assert!((mag - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosmic_mind_creation() {
        let cosmic = CosmicMind::new();

        assert_eq!(cosmic.archetypes.len(), 22);
        assert_eq!(cosmic.archetypes[0].id, 1);
        assert_eq!(cosmic.archetypes[21].id, 22);
    }

    #[test]
    fn test_cosmic_mind_get_archetype() {
        let cosmic = CosmicMind::new();

        let archetype = cosmic.get_archetype(5);
        assert!(archetype.is_some());
        assert_eq!(archetype.unwrap().id, 5);

        let invalid = cosmic.get_archetype(99);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_logos_mind_creation() {
        let cosmic = CosmicMind::new();
        let bias = BiasVector::new();
        let logos = LogosMind::new(cosmic, bias);

        assert_eq!(logos.refined_archetypes.len(), 22);
        assert_eq!(logos.logos_bias.biases.len(), 22);
    }

    #[test]
    fn test_logos_mind_refinement() {
        let cosmic = CosmicMind::new();
        let mut bias = BiasVector::new();
        bias.set_bias(1, 0.5);

        let logos = LogosMind::new(cosmic, bias);

        // Archetype 1 should have bias applied
        assert_eq!(logos.refined_archetypes[0].cumulative_bias, 0.5);
    }

    #[test]
    fn test_sub_logos_mind_creation() {
        let cosmic = CosmicMind::new();
        let logos_bias = BiasVector::new();
        let logos = LogosMind::new(cosmic, logos_bias);
        let sub_logos_bias = BiasVector::new();
        let sub_logos = SubLogosMind::new(logos, sub_logos_bias);

        assert_eq!(sub_logos.refined_archetypes.len(), 22);
        assert_eq!(sub_logos.sub_logos_bias.biases.len(), 22);
    }

    #[test]
    fn test_sub_logos_mind_access_hierarchy() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);

        // Access through hierarchy
        let cosmic_ref = sub_logos.access_cosmic_mind();
        assert_eq!(cosmic_ref.archetypes.len(), 22);

        let logos_ref = sub_logos.access_logos_mind();
        assert_eq!(logos_ref.refined_archetypes.len(), 22);
    }

    #[test]
    fn test_entity_archetypical_mind_creation() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let entity_bias = BiasVector::new();
        let entity = EntityArchetypicalMind::new(sub_logos, entity_bias);

        assert_eq!(entity.refined_archetypes.len(), 22);
        assert_eq!(entity.entity_bias.biases.len(), 22);
        assert_eq!(entity.holographic_links.len(), 0);
    }

    #[test]
    fn test_entity_archetypical_mind_from_seed() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);

        let entity1 = EntityArchetypicalMind::from_seed(sub_logos.clone(), 12345);
        let entity2 = EntityArchetypicalMind::from_seed(sub_logos.clone(), 12345);
        let entity3 = EntityArchetypicalMind::from_seed(sub_logos, 67890);

        // Same seed should produce same result
        assert_eq!(entity1.entity_bias, entity2.entity_bias);

        // Different seed should produce different result
        assert_ne!(entity1.entity_bias, entity3.entity_bias);
    }

    #[test]
    fn test_entity_archetypical_mind_access_hierarchy() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Access through hierarchy
        let cosmic_ref = entity.access_cosmic_mind();
        assert_eq!(cosmic_ref.archetypes.len(), 22);

        let logos_ref = entity.access_logos_mind();
        assert_eq!(logos_ref.refined_archetypes.len(), 22);

        let sub_logos_ref = entity.access_sub_logos_mind();
        assert_eq!(sub_logos_ref.refined_archetypes.len(), 22);
    }

    #[test]
    fn test_entity_archetypical_mind_add_holographic_link() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let mut entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        entity.add_holographic_link(123, 0.7);
        entity.add_holographic_link(456, 0.5);

        assert_eq!(entity.holographic_links.len(), 2);
        assert_eq!(entity.holographic_links[0].target_entity_id, 123);
        assert_eq!(entity.holographic_links[0].resonance, 0.7);
    }

    #[test]
    fn test_entity_archetypical_mind_refinement_depth() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        assert_eq!(entity.refinement_depth(), 4);
    }

    #[test]
    fn test_entity_archetypical_mind_calculate_uniqueness() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);

        let entity_neutral = EntityArchetypicalMind::with_neutral_bias(sub_logos.clone());
        let entity_biased = EntityArchetypicalMind::from_seed(sub_logos, 12345);

        // Neutral entity should have low uniqueness
        let uniqueness_neutral = entity_neutral.calculate_uniqueness();
        assert!(uniqueness_neutral >= 0.0 && uniqueness_neutral <= 1.0);

        // Biased entity should have higher uniqueness
        let uniqueness_biased = entity_biased.calculate_uniqueness();
        assert!(uniqueness_biased >= uniqueness_neutral);
    }

    #[test]
    fn test_holographic_link_creation() {
        let link = HolographicLink::new(123, 0.7);

        assert_eq!(link.target_entity_id, 123);
        assert_eq!(link.resonance, 0.7);
        assert_eq!(link.influence_direction, InfluenceDirection::Bidirectional);
        assert_eq!(link.last_sync, 0.0);
    }

    #[test]
    fn test_holographic_link_clamping() {
        let link = HolographicLink::new(123, 2.0);
        assert_eq!(link.resonance, 1.0);

        let link = HolographicLink::new(123, -1.0);
        assert_eq!(link.resonance, 0.0);
    }

    #[test]
    fn test_holographic_link_update_sync() {
        let mut link = HolographicLink::new(123, 0.7);
        link.update_sync(123.45);

        assert_eq!(link.last_sync, 123.45);
    }

    #[test]
    fn test_complex_type_from_archetype_id() {
        assert_eq!(ComplexType::from_archetype_id(1), Some(ComplexType::Mind));
        assert_eq!(ComplexType::from_archetype_id(7), Some(ComplexType::Mind));
        assert_eq!(ComplexType::from_archetype_id(8), Some(ComplexType::Body));
        assert_eq!(ComplexType::from_archetype_id(14), Some(ComplexType::Body));
        assert_eq!(
            ComplexType::from_archetype_id(15),
            Some(ComplexType::Spirit)
        );
        assert_eq!(
            ComplexType::from_archetype_id(21),
            Some(ComplexType::Spirit)
        );
        assert_eq!(ComplexType::from_archetype_id(22), Some(ComplexType::Mind));
        assert_eq!(ComplexType::from_archetype_id(99), None);
    }

    #[test]
    fn test_full_hierarchy_creation() {
        // Create full hierarchy
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Verify hierarchy
        assert_eq!(entity.access_cosmic_mind().archetypes.len(), 22);
        assert_eq!(entity.access_logos_mind().refined_archetypes.len(), 22);
        assert_eq!(entity.access_sub_logos_mind().refined_archetypes.len(), 22);
        assert_eq!(entity.refined_archetypes.len(), 22);
    }

    #[test]
    fn test_cumulative_refinement() {
        let cosmic = CosmicMind::new();

        // Level 1: Cosmic
        let cosmic_activation = cosmic.archetypes[0].base_activation;

        // Level 2: Logos
        let mut logos_bias = BiasVector::new();
        logos_bias.set_bias(1, 0.1);
        let logos = LogosMind::new(cosmic, logos_bias.clone());
        let logos_activation = logos.refined_archetypes[0].activation;

        // Level 3: Sub-Logos
        let mut sub_logos_bias = BiasVector::new();
        sub_logos_bias.set_bias(1, 0.2);
        let sub_logos = SubLogosMind::new(logos, sub_logos_bias.clone());
        let sub_logos_activation = sub_logos.refined_archetypes[0].activation;

        // Level 4: Entity
        let mut entity_bias = BiasVector::new();
        entity_bias.set_bias(1, 0.3);
        let entity = EntityArchetypicalMind::new(sub_logos, entity_bias);
        let entity_activation = entity.refined_archetypes[0].activation;

        // Verify cumulative refinement
        assert!(entity_activation > sub_logos_activation);
        assert!(sub_logos_activation > logos_activation);
        assert!(logos_activation > cosmic_activation);
    }

    // ============================================================================
    // PHASE 9C: PROCESSING AND SYNCHRONIZATION TESTS
    // ============================================================================

    #[test]
    fn test_entity_mind_process_without_links() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        // Create entity with non-neutral bias to generate changes
        let mut entity = EntityArchetypicalMind::from_seed(sub_logos, 12345);

        // Process entity mind
        let changes = entity.process(0.5, 4.0, 0.5, 0.0);

        // Should have changes from entity bias application
        assert!(!changes.is_empty());

        // Verify archetypes were updated
        for i in 0..22 {
            let archetype = entity.get_archetype((i + 1) as u8).unwrap();
            assert!(archetype.activation >= 0.0);
            assert!(archetype.activation <= 1.0);
            assert!(archetype.lambda >= 0.0);
            assert!(archetype.lambda <= 1.0);
        }
    }

    #[test]
    fn test_entity_mind_process_with_links() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        // Create entity with non-neutral bias to generate changes
        let mut entity = EntityArchetypicalMind::from_seed(sub_logos, 12345);

        // Add holographic links
        entity.add_holographic_link(2, 0.8);
        entity.add_holographic_link(3, 0.6);

        // Process entity mind
        let changes = entity.process(0.5, 4.0, 0.5, 0.0);

        // Should have changes from holographic influences and entity bias
        assert!(!changes.is_empty());

        // Verify holographic links were updated
        assert_eq!(entity.get_holographic_links().len(), 2);
        assert_eq!(entity.get_holographic_links()[0].last_sync, 0.0);
        assert_eq!(entity.get_holographic_links()[1].last_sync, 0.0);
    }

    #[test]
    fn test_entity_mind_process_with_timestamp() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let mut entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Add holographic link
        entity.add_holographic_link(2, 0.8);

        // Process entity mind with timestamp
        let timestamp = 100.0;
        let changes = entity.process(0.5, 4.0, 0.5, timestamp);

        // Verify link sync timestamp was updated
        assert_eq!(entity.get_holographic_links()[0].last_sync, timestamp);
    }

    #[test]
    fn test_entity_mind_process_clamping() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let mut entity = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Set all archetypes to maximum activation
        for i in 0..22 {
            entity.refined_archetypes[i].activation = 1.0;
        }

        // Process entity mind (should not exceed 1.0)
        entity.process(1.0, 8.0, 1.0, 0.0);

        for i in 0..22 {
            let archetype = entity.get_archetype((i + 1) as u8).unwrap();
            assert!(
                archetype.activation <= 1.0,
                "Archetype {} activation exceeds 1.0",
                i + 1
            );
            assert!(
                archetype.lambda <= 1.0,
                "Archetype {} lambda exceeds 1.0",
                i + 1
            );
        }

        // Set all archetypes to minimum activation
        for i in 0..22 {
            entity.refined_archetypes[i].activation = 0.0;
        }

        // Process entity mind (should not go below 0.0)
        entity.process(1.0, 8.0, 1.0, 0.0);

        for i in 0..22 {
            let archetype = entity.get_archetype((i + 1) as u8).unwrap();
            assert!(
                archetype.activation >= 0.0,
                "Archetype {} activation below 0.0",
                i + 1
            );
            assert!(
                archetype.lambda >= 0.0,
                "Archetype {} lambda below 0.0",
                i + 1
            );
        }
    }

    #[test]
    fn test_entity_mind_process_developmental_level_influence() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let mut entity_low = EntityArchetypicalMind::with_neutral_bias(sub_logos.clone());
        let mut entity_high = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Store initial activations
        let initial_low = entity_low.refined_archetypes[0].activation;
        let initial_high = entity_high.refined_archetypes[0].activation;

        // Process with low developmental level
        entity_low.process(0.5, 1.0, 0.5, 0.0);

        // Process with high developmental level
        entity_high.process(0.5, 7.0, 0.5, 0.0);

        // Higher developmental level should result in more changes
        // (more sensitive to holographic influences)
        let change_low = (entity_low.refined_archetypes[0].activation - initial_low).abs();
        let change_high = (entity_high.refined_archetypes[0].activation - initial_high).abs();

        assert!(change_high >= change_low);
    }

    #[test]
    fn test_entity_mind_process_free_will_capacity_influence() {
        let cosmic = CosmicMind::new();
        let logos = LogosMind::with_neutral_bias(cosmic);
        let sub_logos = SubLogosMind::with_neutral_bias(logos);
        let mut entity_low = EntityArchetypicalMind::with_neutral_bias(sub_logos.clone());
        let mut entity_high = EntityArchetypicalMind::with_neutral_bias(sub_logos);

        // Add holographic links
        entity_low.add_holographic_link(2, 0.8);
        entity_high.add_holographic_link(3, 0.8);

        // Store initial activations
        let initial_low = entity_low.refined_archetypes[0].activation;
        let initial_high = entity_high.refined_archetypes[0].activation;

        // Process with low free will capacity
        entity_low.process(0.5, 4.0, 0.1, 0.0);

        // Process with high free will capacity
        entity_high.process(0.5, 4.0, 1.0, 0.0);

        // Higher free will capacity should result in more changes
        // (more receptive to holographic influences)
        let change_low = (entity_low.refined_archetypes[0].activation - initial_low).abs();
        let change_high = (entity_high.refined_archetypes[0].activation - initial_high).abs();

        assert!(change_high >= change_low);
    }
}
*/
