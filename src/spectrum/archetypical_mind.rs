//! # Archetypical Mind System
//!
//! This module implements the archetypical mind system at Red-Ray.
//!
//! ## Theoretical Foundation
//!
//! Archetypical minds are specifically designed as **training aids for entities in veiled experience**.
//!
//! ## Purpose
//!
//! - Help entities navigate the veiled experience
//! - Make the choice of polarity (service-to-self or service-to-others)
//! - Contain "all facets which may affect mind or experience"
//!
//! ## Three-Tier Archetypical Mind Refinement
//!
//! 1. **Cosmic Mind**: Universal field of potential, the same for all sub-Logoi
//! 2. **Primary Logos Refinement**: Universal archetypical patterns created at Blue-Ray (structure unknown)
//! 3. **Sub-Logos Refinement**: Solar-scale Logoi further refine these patterns into specific archetypical mind systems
//!
//! ## Different Solar-Logoi, Different Choices
//!
//! - Some Solar-Logoi use **10-archetype systems** (two systems of five)
//! - Other Solar-Logoi use **22-archetype systems** (7+7+7+1)
//! - Our Solar-Logos chose the **22-archetype system** (maximum articulation) to support intense veiling and polarization acceleration
//!
//! ## The 22-Archetype System
//!
//! - **Archetypes 1-7**: Mind complex (Matrix, Potentiator, Catalyst, Experience, Significator, Transformation, Great Way)
//! - **Archetypes 8-14**: Body complex (Matrix, Potentiator, Catalyst, Experience, Significator, Transformation, Great Way)
//! - **Archetypes 15-21**: Spirit complex (Matrix, Potentiator, Catalyst, Experience, Significator, Transformation, Great Way)
//! - **Archetype 22**: The Choice - culmination of the system, the fundamental choice of polarity

use std::collections::HashMap;

/// Represents the type of archetypical system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypicalSystemType {
    /// 10-archetype system (two systems of five)
    TenArchetype,
    /// 22-archetype system (7+7+7+1)
    TwentyTwoArchetype,
}

// Backward compatibility alias for test code
impl ArchetypicalSystemType {
    /// Backward-compatible alias for TwentyTwoArchetype
    #[allow(non_upper_case_globals)]
    pub const TwentyTwoArchetypes: ArchetypicalSystemType =
        ArchetypicalSystemType::TwentyTwoArchetype;
}

/// Represents the complex type (Mind, Body, Spirit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexType {
    /// Mind complex
    Mind,
    /// Body complex
    Body,
    /// Spirit complex
    Spirit,
}

/// Represents the archetype role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchetypeRole {
    /// Matrix - the foundation
    Matrix,
    /// Potentiator - the catalyst of change
    Potentiator,
    /// Catalyst - the experience provider
    Catalyst,
    /// Experience - the result
    Experience,
    /// Significator - the meaning
    Significator,
    /// Transformation - the change
    Transformation,
    /// Great Way - the path
    GreatWay,
    /// The Choice - the culmination (Archetype 22 only)
    Choice,
}

/// Represents an archetype
#[derive(Debug, Clone)]
pub struct Archetype {
    /// The archetype number (1-22)
    pub number: usize,
    /// The complex type
    pub complex_type: Option<ComplexType>,
    /// The archetype role
    pub role: ArchetypeRole,
    /// The name
    pub name: String,
    /// The description
    pub description: String,
    /// The activation level (0.0 to 1.0)
    pub activation: f64,
}

impl Archetype {
    /// Creates a new archetype
    pub fn new(
        number: usize,
        complex_type: Option<ComplexType>,
        role: ArchetypeRole,
        name: String,
        description: String,
    ) -> Self {
        Archetype {
            number,
            complex_type,
            role,
            name,
            description,
            activation: 0.0,
        }
    }

    /// Sets the activation level
    pub fn set_activation(&mut self, activation: f64) {
        self.activation = activation.clamp(0.0, 1.0);
    }

    /// Checks if this archetype is active
    pub fn is_active(&self) -> bool {
        self.activation > 0.5
    }

    /// Gets the archetype's position in the system
    pub fn position(&self) -> ArchetypePosition {
        if self.number == 22 {
            ArchetypePosition::Culmination
        } else if self.number <= 7 {
            ArchetypePosition::MindComplex
        } else if self.number <= 14 {
            ArchetypePosition::BodyComplex
        } else {
            ArchetypePosition::SpiritComplex
        }
    }
}

/// Represents the position of an archetype in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypePosition {
    /// Mind complex (Archetypes 1-7)
    MindComplex,
    /// Body complex (Archetypes 8-14)
    BodyComplex,
    /// Spirit complex (Archetypes 15-21)
    SpiritComplex,
    /// Culmination (Archetype 22)
    Culmination,
}

// ============================================================================
// MIGRATION: Holographic Archetypical Mind Hierarchy (Phase 4, Migration 6)
// Source: src/holographic_archetypical_mind.rs
// ============================================================================

/// Direction of influence in holographic link
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfluenceDirection {
    /// Both entities influence each other
    Bidirectional,
    /// Only source influences target
    ToTarget,
    /// Only target influences source
    FromTarget,
}

/// Archetype Template (immutable)
///
/// The universal pattern for an archetype, existing at the Cosmic Mind level.
/// This is the unmodified, perfect template from which all refinements derive.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeTemplate {
    /// Archetype ID (1-22)
    pub id: u8,
    /// Archetype name
    pub name: String,
    /// Base activation level (0.0 to 1.0)
    pub base_activation: f64,
    /// Base lambda value (0.0 to 1.0)
    pub base_lambda: f64,
    /// Complex type (Mind, Body, Spirit)
    pub complex_type: ComplexType,
}

impl ArchetypeTemplate {
    /// Create a new archetype template
    pub fn new(
        id: u8,
        name: String,
        base_activation: f64,
        base_lambda: f64,
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
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs (renamed from "Archetype" to avoid conflict)
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeInstance {
    /// Reference to the template
    pub template_id: u8,
    /// Refined activation level (0.0 to 1.0)
    pub activation: f64,
    /// Refined lambda value (0.0 to 1.0)
    pub lambda: f64,
    /// Bias applied to this archetype (cumulative from all levels)
    pub cumulative_bias: f64,
    /// Complex type (inherited from template)
    pub complex_type: ComplexType,
}

impl ArchetypeInstance {
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
    pub fn apply_bias(&mut self, bias: f64) {
        self.cumulative_bias += bias;

        // Bias affects activation and lambda
        // Positive bias increases activation and lambda
        // Negative bias decreases activation and lambda
        self.activation = (self.activation + bias * 0.1).clamp(0.0, 1.0);
        self.lambda = (self.lambda + bias * 0.1).clamp(0.0, 1.0);
    }
}

/// Bias Vector (22-dimensional)
///
/// A 22-dimensional vector representing the bias applied to each archetype.
/// Used at each level of the hierarchy to refine the archetypes with unique perspectives.
///
/// Bias values range from -1.0 to 1.0:
/// - Negative bias: Service-to-self (STS) tendency
/// - Positive bias: Service-to-others (STO) tendency
/// - Zero bias: Neutral (no preference)
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct BiasVector {
    /// 22-dimensional bias vector (one for each archetype)
    pub biases: [f64; 22],
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
            let combined = (seed as f64) * ((i + 1) as f64);
            let value = (combined.sin() * 2.0).fract() - 0.5;
            biases[i] = value.clamp(-1.0, 1.0);
        }

        Self { biases }
    }

    /// Create a bias vector with specific values
    pub fn from_values(biases: [f64; 22]) -> Self {
        Self { biases }
    }

    /// Set bias for a specific archetype
    pub fn set_bias(&mut self, archetype_id: u8, bias: f64) {
        if archetype_id >= 1 && archetype_id <= 22 {
            self.biases[(archetype_id - 1) as usize] = bias.clamp(-1.0, 1.0);
        }
    }

    /// Get bias for a specific archetype
    pub fn get_bias(&self, archetype_id: u8) -> f64 {
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
    pub fn refine_archetypes(
        &self,
        templates: &[ArchetypeTemplate; 22],
    ) -> [ArchetypeInstance; 22] {
        let mut refined = core::array::from_fn(|i| ArchetypeInstance::from_template(&templates[i]));

        for i in 0..22 {
            refined[i].apply_bias(self.biases[i]);
        }

        refined
    }

    /// Refine existing archetypes with this bias vector
    ///
    /// Takes a set of already-refined archetypes and applies additional bias.
    /// This is used for cumulative refinement through the hierarchy.
    pub fn refine_existing_archetypes(
        &self,
        archetypes: &[ArchetypeInstance; 22],
    ) -> [ArchetypeInstance; 22] {
        let mut refined = archetypes.clone();

        for i in 0..22 {
            refined[i].apply_bias(self.biases[i]);
        }

        refined
    }

    /// Calculate the magnitude of the bias vector
    pub fn magnitude(&self) -> f64 {
        let sum: f64 = self.biases.iter().map(|b| b * b).sum();
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

/// Holographic Link (non-local connection between entities)
///
/// Represents a non-local connection between entities in the holographic field.
/// Changes in one entity can propagate to another through these links.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicLink {
    /// Target entity ID
    pub target_entity_id: u64,
    /// Resonance strength (0.0 to 1.0)
    pub resonance: f64,
    /// Direction of influence
    pub influence_direction: InfluenceDirection,
    /// Last synchronization timestamp
    pub last_sync: f64,
}

impl HolographicLink {
    /// Create a new holographic link
    pub fn new(target_entity_id: u64, resonance: f64) -> Self {
        Self {
            target_entity_id,
            resonance: resonance.clamp(0.0, 1.0),
            influence_direction: InfluenceDirection::Bidirectional,
            last_sync: 0.0,
        }
    }

    /// Update the last sync timestamp
    pub fn update_sync(&mut self, timestamp: f64) {
        self.last_sync = timestamp;
    }
}

/// Cosmic Mind (Level 1: immutable template)
///
/// The first level of the archetypical mind hierarchy, containing the immutable
/// templates for all 22 archetypes. This is the universal pattern from which
/// all other levels derive.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
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

/// Logos Mind (Level 2: refined by Logos bias)
///
/// The second level of the archetypical mind hierarchy, where the Cosmic Mind
/// is refined by the Logos's unique bias. This level is shared among all entities
/// in the same Logos system.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct LogosMind {
    /// Reference to Cosmic Mind (immutable)
    pub cosmic_mind: CosmicMind,
    /// Logos's unique bias
    pub logos_bias: BiasVector,
    /// Refined archetypes (22)
    pub refined_archetypes: [ArchetypeInstance; 22],
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
    pub fn get_archetype(&self, id: u8) -> Option<&ArchetypeInstance> {
        if id >= 1 && id <= 22 {
            Some(&self.refined_archetypes[(id - 1) as usize])
        } else {
            None
        }
    }
}

/// Sub-Logos Mind (Level 3: refined by sub-Logos bias)
///
/// The third level of the archetypical mind hierarchy, where the Logos Mind
/// is further refined by the sub-Logos's unique bias. This level is shared among
/// all entities in the same sub-Logos system.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct SubLogosMind {
    /// Reference to Logos Mind
    pub logos_mind: LogosMind,
    /// Sub-Logos's unique bias
    pub sub_logos_bias: BiasVector,
    /// Refined archetypes (22)
    pub refined_archetypes: [ArchetypeInstance; 22],
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
    pub fn get_archetype(&self, id: u8) -> Option<&ArchetypeInstance> {
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

/// Entity Archetypical Mind (Level 4: refined by entity bias)
///
/// The fourth and final level of the archetypical mind hierarchy, where the
/// Sub-Logos Mind is further refined by the entity's unique bias. This level
/// is unique to each entity, representing their individual perspective on the
/// archetypical patterns.
///
/// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
#[derive(Debug, Clone, PartialEq)]
pub struct EntityArchetypicalMind {
    /// Reference to Sub-Logos Mind
    pub sub_logos_mind: SubLogosMind,
    /// Entity's unique bias
    pub entity_bias: BiasVector,
    /// Refined archetypes (22) - unique to this entity
    pub refined_archetypes: [ArchetypeInstance; 22],
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
    pub fn get_archetype(&self, id: u8) -> Option<&ArchetypeInstance> {
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
    pub fn add_holographic_link(&mut self, target_entity_id: u64, resonance: f64) {
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
    pub fn calculate_uniqueness(&self) -> f64 {
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
    // PHASE 9C: PROCESSING AND SYNCHRONIZATION (Migrated)
    // ============================================================================

    /// Process the entity archetypical mind
    ///
    /// This method applies holographic influences from connected entities,
    /// updates archetype activations, and generates holographic changes for propagation.
    ///
    /// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs
    /// NOTE: Returns Vec<crate::simulation_v3::holographic_field::HolographicChange> after Migration 5
    pub fn process(
        &mut self,
        coupling_coefficient: f64,
        developmental_level: f64,
        free_will_capacity: f64,
        timestamp: f64,
    ) -> Vec<crate::simulation_v3::holographic_field::HolographicChange> {
        // MIGRATION NOTE: ArchetypeID is re-exported from integration module

        use crate::simulation_v3::holographic_field::{ChangeType, HolographicChange};

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
                        (i + 1) as usize,
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
                    (i + 1) as usize,
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
    /// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs (placeholder implementation)
    pub fn sync_from_complex(
        &mut self,
        _mind_complex: &crate::complex::Complex,
        _body_complex: &crate::complex::Complex,
        _spirit_complex: &crate::complex::Complex,
        _choice_archetype: &crate::archetypes::a22_choice::ChoiceArchetype,
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
    /// MIGRATION NOTE: Migrated from holographic_archetypical_mind.rs (placeholder implementation)
    pub fn apply_to_complex(
        &self,
        _changes: &[crate::simulation_v3::holographic_field::HolographicChange],
        _mind_complex: &mut crate::complex::Complex,
        _body_complex: &mut crate::complex::Complex,
        _spirit_complex: &mut crate::complex::Complex,
        _choice_archetype: &mut crate::archetypes::a22_choice::ChoiceArchetype,
    ) {
        // Placeholder: In a full implementation, this would apply holographic changes
        // to Complex archetypes. For now, changes are tracked but not applied.
        // The Complex archetypes are processed independently, and synchronization
        // happens through sync_from_complex() instead.
    }
}

// ============================================================================
// END MIGRATION: Holographic Archetypical Mind Hierarchy
// ============================================================================

/// Represents an archetypical mind system
#[derive(Debug, Clone)]
pub struct ArchetypicalMind {
    /// The system type
    pub system_type: ArchetypicalSystemType,
    /// The archetypes
    pub archetypes: HashMap<usize, Archetype>,
    /// The Solar-Logos that created this system
    pub solar_logos_id: String,
}

impl ArchetypicalMind {
    /// Creates a new archetypical mind system
    pub fn new(system_type: ArchetypicalSystemType, solar_logos_id: String) -> Self {
        let mut mind = ArchetypicalMind {
            system_type,
            archetypes: HashMap::new(),
            solar_logos_id,
        };

        match system_type {
            ArchetypicalSystemType::TenArchetype => {
                mind.create_ten_archetype_system();
            }
            ArchetypicalSystemType::TwentyTwoArchetype => {
                mind.create_twenty_two_archetype_system();
            }
        }

        mind
    }

    /// Creates a new archetypical mind from the Logos (backward compatibility)
    pub fn new_from_logos() -> Self {
        Self::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "SolarLogos".to_string(),
        )
    }

    /// Creates a 10-archetype system (two systems of five)
    fn create_ten_archetype_system(&mut self) {
        // System 1: Mind (5 archetypes)
        let roles = [
            ArchetypeRole::Matrix,
            ArchetypeRole::Potentiator,
            ArchetypeRole::Catalyst,
            ArchetypeRole::Experience,
            ArchetypeRole::Significator,
        ];

        for (i, role) in roles.iter().enumerate() {
            let archetype = Archetype::new(
                i + 1,
                Some(ComplexType::Mind),
                *role,
                format!("Mind {}", role_to_name(role)),
                format!("Mind {} archetype", role_to_name(role)),
            );
            self.archetypes.insert(i + 1, archetype);
        }

        // System 2: Body (5 archetypes)
        for (i, role) in roles.iter().enumerate() {
            let archetype = Archetype::new(
                i + 6,
                Some(ComplexType::Body),
                *role,
                format!("Body {}", role_to_name(role)),
                format!("Body {} archetype", role_to_name(role)),
            );
            self.archetypes.insert(i + 6, archetype);
        }
    }

    /// Creates a 22-archetype system (7+7+7+1)
    fn create_twenty_two_archetype_system(&mut self) {
        // Mind complex (Archetypes 1-7)
        let roles = [
            ArchetypeRole::Matrix,
            ArchetypeRole::Potentiator,
            ArchetypeRole::Catalyst,
            ArchetypeRole::Experience,
            ArchetypeRole::Significator,
            ArchetypeRole::Transformation,
            ArchetypeRole::GreatWay,
        ];

        for (i, role) in roles.iter().enumerate() {
            let archetype = Archetype::new(
                i + 1,
                Some(ComplexType::Mind),
                *role,
                format!("Mind {}", role_to_name(role)),
                format!("Mind {} archetype", role_to_name(role)),
            );
            self.archetypes.insert(i + 1, archetype);
        }

        // Body complex (Archetypes 8-14)
        for (i, role) in roles.iter().enumerate() {
            let archetype = Archetype::new(
                i + 8,
                Some(ComplexType::Body),
                *role,
                format!("Body {}", role_to_name(role)),
                format!("Body {} archetype", role_to_name(role)),
            );
            self.archetypes.insert(i + 8, archetype);
        }

        // Spirit complex (Archetypes 15-21)
        for (i, role) in roles.iter().enumerate() {
            let archetype = Archetype::new(
                i + 15,
                Some(ComplexType::Spirit),
                *role,
                format!("Spirit {}", role_to_name(role)),
                format!("Spirit {} archetype", role_to_name(role)),
            );
            self.archetypes.insert(i + 15, archetype);
        }

        // Archetype 22: The Choice
        let archetype22 = Archetype::new(
            22,
            None,
            ArchetypeRole::Choice,
            "The Choice".to_string(),
            "The fundamental choice of polarity - Service-to-Others or Service-to-Self".to_string(),
        );
        self.archetypes.insert(22, archetype22);
    }

    /// Gets an archetype by number
    pub fn get_archetype(&self, number: usize) -> Option<&Archetype> {
        self.archetypes.get(&number)
    }

    /// Gets a mutable archetype by number
    pub fn get_archetype_mut(&mut self, number: usize) -> Option<&mut Archetype> {
        self.archetypes.get_mut(&number)
    }

    /// Activates an archetype
    pub fn activate_archetype(&mut self, number: usize, activation: f64) -> Result<(), String> {
        if let Some(archetype) = self.archetypes.get_mut(&number) {
            archetype.set_activation(activation);
            Ok(())
        } else {
            Err(format!("Archetype {} not found", number))
        }
    }

    /// Gets all archetypes for a complex
    pub fn get_complex_archetypes(&self, complex_type: ComplexType) -> Vec<&Archetype> {
        self.archetypes
            .values()
            .filter(|a| a.complex_type == Some(complex_type))
            .collect()
    }

    /// Gets Archetype 22 (The Choice)
    pub fn get_choice(&self) -> Option<&Archetype> {
        self.archetypes.get(&22)
    }

    /// Checks if this is a 22-archetype system
    pub fn is_twenty_two_system(&self) -> bool {
        matches!(self.system_type, ArchetypicalSystemType::TwentyTwoArchetype)
    }

    /// Gets the number of archetypes
    pub fn archetype_count(&self) -> usize {
        self.archetypes.len()
    }

    /// Gets the system description
    pub fn system_description(&self) -> String {
        match self.system_type {
            ArchetypicalSystemType::TenArchetype => {
                "10-archetype system (two systems of five)".to_string()
            }
            ArchetypicalSystemType::TwentyTwoArchetype => {
                "22-archetype system (7+7+7+1) - maximum articulation".to_string()
            }
        }
    }

    /// Get archetype activations as an array
    /// Returns a 22-element array with activation levels (0.0 to 1.0)
    pub fn get_activations(&self) -> [f64; 22] {
        let mut activations = [0.0; 22];
        for (idx, archetype) in &self.archetypes {
            if *idx < 22 {
                activations[*idx] = archetype.activation;
            }
        }
        activations
    }
}

/// Helper function to convert role to name
fn role_to_name(role: &ArchetypeRole) -> &'static str {
    match role {
        ArchetypeRole::Matrix => "Matrix",
        ArchetypeRole::Potentiator => "Potentiator",
        ArchetypeRole::Catalyst => "Catalyst",
        ArchetypeRole::Experience => "Experience",
        ArchetypeRole::Significator => "Significator",
        ArchetypeRole::Transformation => "Transformation",
        ArchetypeRole::GreatWay => "Great Way",
        ArchetypeRole::Choice => "Choice",
    }
}

/// Represents the training aid function of the archetypical mind
#[derive(Debug, Clone)]
pub struct TrainingAid {
    /// The archetypical mind
    pub archetypical_mind: ArchetypicalMind,
    /// The training effectiveness (0.0 to 1.0)
    pub effectiveness: f64,
    /// The polarity bias (0.0 = STO, 1.0 = STS)
    pub polarity_bias: f64,
}

impl TrainingAid {
    /// Creates a new training aid
    pub fn new(archetypical_mind: ArchetypicalMind) -> Self {
        TrainingAid {
            archetypical_mind,
            effectiveness: 0.8,
            polarity_bias: 0.5, // Neutral
        }
    }

    /// Provides guidance for veiled experience
    pub fn provide_guidance(&self, archetype_number: usize) -> Option<String> {
        if let Some(archetype) = self.archetypical_mind.get_archetype(archetype_number) {
            Some(format!(
                "{}: {} - Activation: {:.2}",
                archetype.name, archetype.description, archetype.activation
            ))
        } else {
            None
        }
    }

    /// Assists with polarity choice
    pub fn assist_polarity_choice(&self) -> PolarityGuidance {
        if let Some(choice) = self.archetypical_mind.get_choice() {
            if choice.activation > 0.5 {
                PolarityGuidance::ChoiceAvailable {
                    strength: choice.activation,
                    bias: self.polarity_bias,
                }
            } else {
                PolarityGuidance::ChoiceNotReady
            }
        } else {
            PolarityGuidance::NoChoiceArchetype
        }
    }

    /// Updates effectiveness based on entity progress
    pub fn update_effectiveness(&mut self, progress: f64) {
        self.effectiveness = 0.5 + (progress * 0.5);
    }
}

/// Represents polarity guidance
#[derive(Debug, Clone, PartialEq)]
pub enum PolarityGuidance {
    /// Choice is available
    ChoiceAvailable { strength: f64, bias: f64 },
    /// Choice is not ready yet
    ChoiceNotReady,
    /// No choice archetype in system
    NoChoiceArchetype,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_new() {
        let archetype = Archetype::new(
            1,
            Some(ComplexType::Mind),
            ArchetypeRole::Matrix,
            "Mind Matrix".to_string(),
            "Mind Matrix archetype".to_string(),
        );

        assert_eq!(archetype.number, 1);
        assert_eq!(archetype.complex_type, Some(ComplexType::Mind));
        assert_eq!(archetype.role, ArchetypeRole::Matrix);
        assert_eq!(archetype.activation, 0.0);
    }

    #[test]
    fn test_archetype_set_activation() {
        let mut archetype = Archetype::new(
            1,
            Some(ComplexType::Mind),
            ArchetypeRole::Matrix,
            "Mind Matrix".to_string(),
            "Mind Matrix archetype".to_string(),
        );

        archetype.set_activation(0.7);

        assert_eq!(archetype.activation, 0.7);
    }

    #[test]
    fn test_archetype_set_activation_clamping() {
        let mut archetype = Archetype::new(
            1,
            Some(ComplexType::Mind),
            ArchetypeRole::Matrix,
            "Mind Matrix".to_string(),
            "Mind Matrix archetype".to_string(),
        );

        archetype.set_activation(1.5);

        assert_eq!(archetype.activation, 1.0);
    }

    #[test]
    fn test_archetype_is_active() {
        let mut archetype = Archetype::new(
            1,
            Some(ComplexType::Mind),
            ArchetypeRole::Matrix,
            "Mind Matrix".to_string(),
            "Mind Matrix archetype".to_string(),
        );

        assert!(!archetype.is_active());

        archetype.set_activation(0.7);

        assert!(archetype.is_active());
    }

    #[test]
    fn test_archetype_position() {
        let archetype1 = Archetype::new(
            1,
            Some(ComplexType::Mind),
            ArchetypeRole::Matrix,
            "Mind Matrix".to_string(),
            "Mind Matrix archetype".to_string(),
        );

        assert_eq!(archetype1.position(), ArchetypePosition::MindComplex);

        let archetype8 = Archetype::new(
            8,
            Some(ComplexType::Body),
            ArchetypeRole::Matrix,
            "Body Matrix".to_string(),
            "Body Matrix archetype".to_string(),
        );

        assert_eq!(archetype8.position(), ArchetypePosition::BodyComplex);

        let archetype15 = Archetype::new(
            15,
            Some(ComplexType::Spirit),
            ArchetypeRole::Matrix,
            "Spirit Matrix".to_string(),
            "Spirit Matrix archetype".to_string(),
        );

        assert_eq!(archetype15.position(), ArchetypePosition::SpiritComplex);

        let archetype22 = Archetype::new(
            22,
            None,
            ArchetypeRole::Choice,
            "The Choice".to_string(),
            "The Choice".to_string(),
        );

        assert_eq!(archetype22.position(), ArchetypePosition::Culmination);
    }

    #[test]
    fn test_archetypical_mind_ten_archetype_system() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TenArchetype,
            "test_solar".to_string(),
        );

        assert_eq!(mind.archetype_count(), 10);
        assert!(!mind.is_twenty_two_system());
        assert_eq!(
            mind.system_description(),
            "10-archetype system (two systems of five)"
        );
    }

    #[test]
    fn test_archetypical_mind_twenty_two_archetype_system() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        assert_eq!(mind.archetype_count(), 22);
        assert!(mind.is_twenty_two_system());
        assert_eq!(
            mind.system_description(),
            "22-archetype system (7+7+7+1) - maximum articulation"
        );
    }

    #[test]
    fn test_archetypical_mind_get_archetype() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let archetype1 = mind.get_archetype(1);
        assert!(archetype1.is_some());
        assert_eq!(archetype1.unwrap().number, 1);

        let archetype22 = mind.get_archetype(22);
        assert!(archetype22.is_some());
        assert_eq!(archetype22.unwrap().role, ArchetypeRole::Choice);

        let archetype23 = mind.get_archetype(23);
        assert!(archetype23.is_none());
    }

    #[test]
    fn test_archetypical_mind_activate_archetype() {
        let mut mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let result = mind.activate_archetype(1, 0.7);

        assert!(result.is_ok());
        assert_eq!(mind.get_archetype(1).unwrap().activation, 0.7);
    }

    #[test]
    fn test_archetypical_mind_activate_archetype_not_found() {
        let mut mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let result = mind.activate_archetype(23, 0.7);

        assert!(result.is_err());
    }

    #[test]
    fn test_archetypical_mind_get_complex_archetypes() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let mind_archetypes = mind.get_complex_archetypes(ComplexType::Mind);
        assert_eq!(mind_archetypes.len(), 7);

        let body_archetypes = mind.get_complex_archetypes(ComplexType::Body);
        assert_eq!(body_archetypes.len(), 7);

        let spirit_archetypes = mind.get_complex_archetypes(ComplexType::Spirit);
        assert_eq!(spirit_archetypes.len(), 7);
    }

    #[test]
    fn test_archetypical_mind_get_choice() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let choice = mind.get_choice();
        assert!(choice.is_some());
        assert_eq!(choice.unwrap().role, ArchetypeRole::Choice);
    }

    #[test]
    fn test_archetypical_mind_get_choice_ten_system() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TenArchetype,
            "test_solar".to_string(),
        );

        let choice = mind.get_choice();
        assert!(choice.is_none());
    }

    #[test]
    fn test_training_aid_new() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let training_aid = TrainingAid::new(mind);

        assert_eq!(training_aid.effectiveness, 0.8);
        assert_eq!(training_aid.polarity_bias, 0.5);
    }

    #[test]
    fn test_training_aid_provide_guidance() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let training_aid = TrainingAid::new(mind);

        let guidance = training_aid.provide_guidance(1);

        assert!(guidance.is_some());
        assert!(guidance.unwrap().contains("Mind Matrix"));
    }

    #[test]
    fn test_training_aid_provide_guidance_not_found() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let training_aid = TrainingAid::new(mind);

        let guidance = training_aid.provide_guidance(23);

        assert!(guidance.is_none());
    }

    #[test]
    fn test_training_aid_assist_polarity_choice() {
        let mut mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        mind.activate_archetype(22, 0.7).unwrap();

        let training_aid = TrainingAid::new(mind);

        let guidance = training_aid.assist_polarity_choice();

        match guidance {
            PolarityGuidance::ChoiceAvailable { strength, bias } => {
                assert_eq!(strength, 0.7);
                assert_eq!(bias, 0.5);
            }
            _ => panic!("Expected ChoiceAvailable"),
        }
    }

    #[test]
    fn test_training_aid_assist_polarity_choice_not_ready() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let training_aid = TrainingAid::new(mind);

        let guidance = training_aid.assist_polarity_choice();

        assert_eq!(guidance, PolarityGuidance::ChoiceNotReady);
    }

    #[test]
    fn test_training_aid_assist_polarity_choice_no_choice() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TenArchetype,
            "test_solar".to_string(),
        );

        let training_aid = TrainingAid::new(mind);

        let guidance = training_aid.assist_polarity_choice();

        assert_eq!(guidance, PolarityGuidance::NoChoiceArchetype);
    }

    #[test]
    fn test_training_aid_update_effectiveness() {
        let mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "test_solar".to_string(),
        );

        let mut training_aid = TrainingAid::new(mind);

        training_aid.update_effectiveness(0.5);

        assert_eq!(training_aid.effectiveness, 0.75);
    }

    #[test]
    fn test_three_tier_refinement() {
        // Solar-Logoi refine universal patterns into specific archetypical mind systems

        // Our Solar-Logos chose the 22-archetype system
        let our_solar_mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "our_solar_logos".to_string(),
        );

        assert!(our_solar_mind.is_twenty_two_system());
        assert_eq!(our_solar_mind.archetype_count(), 22);

        // Another Solar-Logos might choose the 10-archetype system
        let other_solar_mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TenArchetype,
            "other_solar_logos".to_string(),
        );

        assert!(!other_solar_mind.is_twenty_two_system());
        assert_eq!(other_solar_mind.archetype_count(), 10);
    }

    #[test]
    fn test_training_aid_purpose() {
        // Archetypical minds are training aids for entities in veiled experience

        let mut mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "our_solar_logos".to_string(),
        );

        // Activate some archetypes to simulate experience
        mind.activate_archetype(1, 0.6).unwrap(); // Mind Matrix
        mind.activate_archetype(8, 0.7).unwrap(); // Body Matrix
        mind.activate_archetype(15, 0.8).unwrap(); // Spirit Matrix

        let training_aid = TrainingAid::new(mind.clone());

        // The training aid provides guidance
        let guidance = training_aid.provide_guidance(1);
        assert!(guidance.is_some());

        // The training aid assists with polarity choice
        mind.activate_archetype(22, 0.9).unwrap();
        // Create new training aid with updated mind
        let updated_training_aid = TrainingAid::new(mind.clone());
        let polarity_guidance = updated_training_aid.assist_polarity_choice();

        match polarity_guidance {
            PolarityGuidance::ChoiceAvailable { strength, .. } => {
                assert!(strength > 0.5);
            }
            _ => panic!("Expected ChoiceAvailable"),
        }
    }
}
