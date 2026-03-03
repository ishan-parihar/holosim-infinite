//! Transcend and Include - Universal Mechanism for All Layer Transitions
//!
//! From COSMOLOGICAL-ARCHITECTURE.md section 2.6:
//! "Each stage of involution/evolution operates by a single universal constant:
//!  the Transcend and Include operation"
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 1:
//! "Create src/template/transcend_include.rs - Universal transcend/include logic"
//!
//! # The Three Operations
//!
//! 1. **INCLUDE**: Retain all previous development
//!    - Each layer contains all previous layers (holographic principle)
//!    - Violet is in Indigo, which is in Blue, which is in Green, etc.
//!    - Nothing is lost or replaced - everything is integrated
//!
//! 2. **TRANSCEND**: Add new development
//!    - Each layer adds something new that didn't exist before
//!    - Violet → Indigo adds Free Will
//!    - Indigo → Blue adds Logos
//!    - Blue → Green adds Light/Love
//!    - etc.
//!
//! 3. **EVOLVE INTO**: Create attractor-field for next stage
//!    - Each layer creates an evolutionary pull toward the next layer
//!    - This is how entities progress through the density octave
//!    - Attractor fields guide evolution without forcing it
//!
//! # Example: Violet → Indigo Transition
//!
//! ```text
//! Violet: Infinite potential, undifferentiated unity
//!   ↓ INCLUDE: Retain Violet (infinite potential)
//!   ↓ TRANSCEND: Add Free Will (the Choice to know itself)
//!   ↓ EVOLVE INTO: Create attractor for Logos
//! Indigo: Intelligent Infinity + Archetype 22 + Free Will
//! ```
//!
//! # Holographic Implication
//!
//! Because each layer "includes" all previous layers, each entity
//! contains the whole cosmological architecture:
//!
//! ```text
//! Entity = Violet + Indigo + Blue + Green + Yellow + Orange + Red + Layer7
//! ```
//!
//! This is why "Each entity contains within it all densities and sub-densities
//! of the octave" (Law of One principle).
//!
//! # Universal Constant
//!
//! The Transcend and Include operation applies at ALL scales:
//! - Cosmological layers (Violet → Layer 7)
//! - Density transitions (1st → 8th)
//! - Individual evolution (archetype processing, Free Will choice)
//! - Physical emergence (quantum → atomic → molecular → cellular)

use std::fmt;

/// Universal "Transcend and Include" operation
///
/// From COSMOLOGICAL-ARCHITECTURE.md section 2.6:
/// "Each stage of involution/evolution operates by a single universal constant:
///  the Transcend and Include operation"
///
/// # Type Parameters
///
/// - `L1`: Type of the previous stage (what is INCLUDED)
/// - `F`: Type of the new feature (what TRANSCENDS)
/// - `A`: Type of the attractor field (what EVOLVES INTO)
///
/// # Example
///
/// ```ignore
/// // Violet → Indigo transition
/// let violet = VioletRealm::new();
/// let free_will = Feature::FreeWill;
/// let indigo_attractor = AttractorField::logos();
///
/// let transcend_include = TranscendInclude::apply(violet, free_will, indigo_attractor);
/// let indigo = transcend_include.generate_next_stage();
/// ```
#[derive(Debug, Clone)]
pub struct TranscendInclude<L1, F, A> {
    /// INCLUDE: Retain all previous development
    pub includes: L1,

    /// TRANSCEND: Add new development
    pub transcends: F,

    /// EVOLVE INTO: Create attractor-field for next stage
    pub evolves_into: A,
}

impl<L1, F, A> TranscendInclude<L1, F, A> {
    /// Apply the universal "Transcend and Include" operation
    ///
    /// This is the single constant that operates at ALL scales:
    /// - Cosmological layers (Violet → Indigo → Blue → ...)
    /// - Density transitions (1st → 2nd → 3rd → ...)
    /// - Individual evolution (choice → action → experience → ...)
    /// - Physical emergence (quantum → atomic → molecular → ...)
    ///
    /// # Arguments
    ///
    /// * `previous` - What is INCLUDED from the previous stage
    /// * `new_feature` - What TRANSCENDS (new development)
    /// * `next_attractor` - What EVOLVES INTO (attractor for next stage)
    ///
    /// # Returns
    ///
    /// A TranscendInclude structure representing the transition
    pub fn apply(previous: L1, new_feature: F, next_attractor: A) -> Self {
        Self {
            includes: previous,
            transcends: new_feature,
            evolves_into: next_attractor,
        }
    }

    /// Get the included previous stage
    pub fn includes(&self) -> &L1 {
        &self.includes
    }

    /// Get the transcended new feature
    pub fn transcends(&self) -> &F {
        &self.transcends
    }

    /// Get the attractor field for next stage
    pub fn evolves_into(&self) -> &A {
        &self.evolves_into
    }

    /// Mutate the included previous stage
    pub fn includes_mut(&mut self) -> &mut L1 {
        &mut self.includes
    }

    /// Mutate the transcended new feature
    pub fn transcends_mut(&mut self) -> &mut F {
        &mut self.transcends
    }

    /// Mutate the attractor field
    pub fn evolves_into_mut(&mut self) -> &mut A {
        &mut self.evolves_into
    }
}

impl<L1: Clone, F: Clone, A> TranscendInclude<L1, F, A> {
    /// Generate the next stage from current TranscendInclude
    ///
    /// This method implements the "holographic principle":
    /// The next stage CONTAINS the whole previous stage
    /// (nothing is lost or replaced)
    ///
    /// # Returns
    ///
    /// The next stage (includes previous + transcends new)
    pub fn generate_next_stage(&self) -> L1 {
        // INCLUDE: Retain the whole previous stage (holographic principle)
        self.includes.clone()
    }
}

impl<L1, F, A> fmt::Display for TranscendInclude<L1, F, A>
where
    L1: fmt::Display,
    F: fmt::Display,
    A: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TranscendInclude(includes: {}, transcends: {}, evolves_into: {})",
            self.includes, self.transcends, self.evolves_into
        )
    }
}

/// Feature type for new development in Transcend and Include
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// - Violet → Indigo: Free Will
/// - Indigo → Blue: Logos
/// - Blue → Green: Light/Love
/// - Green → Yellow: Dimensions/Veil
/// - Yellow → Orange: Galactic Logoi
/// - Orange → Red: Solar Logoi
/// - Red → Layer 7: Individual entities
///
/// This enum represents the type of new feature being added at each transition.
#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    /// Free Will - First Distortion
    FreeWill,

    /// Logos - Creative Will
    Logos,

    /// Light/Love - Field of Potential
    LightLove,

    /// Dimensions/Veil - Space/Time ↔ Time/Space
    Dimensions,

    /// Galactic Logoi - Galactic-scale consciousness
    GalacticLogos,

    /// Solar Logoi - Solar-scale consciousness
    SolarLogos,

    /// Individuality - Sub-Sub-Logos
    Individuality,

    /// Custom feature
    Custom(String),
}

impl Feature {
    /// Get the layer transition this feature represents
    pub fn transition_layer(&self) -> Option<LayerTransition> {
        match self {
            Feature::FreeWill => Some(LayerTransition::VioletToIndigo),
            Feature::Logos => Some(LayerTransition::IndigoToBlue),
            Feature::LightLove => Some(LayerTransition::BlueToGreen),
            Feature::Dimensions => Some(LayerTransition::GreenToYellow),
            Feature::GalacticLogos => Some(LayerTransition::YellowToOrange),
            Feature::SolarLogos => Some(LayerTransition::OrangeToRed),
            Feature::Individuality => Some(LayerTransition::RedToLayer7),
            Feature::Custom(_) => None,
        }
    }

    /// Get the feature name as a string
    pub fn name(&self) -> &str {
        match self {
            Feature::FreeWill => "Free Will",
            Feature::Logos => "Logos",
            Feature::LightLove => "Light/Love",
            Feature::Dimensions => "Dimensions/Veil",
            Feature::GalacticLogos => "Galactic Logoi",
            Feature::SolarLogos => "Solar Logoi",
            Feature::Individuality => "Individuality",
            Feature::Custom(name) => name,
        }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Layer transitions in cosmological involution/evolution
///
/// From COSMOLOGICAL-ARCHITECTURE.md section 3:
/// Violet → Indigo → Blue → Green → Yellow → Orange → Red → Layer 7
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerTransition {
    /// Violet → Indigo: Add Free Will
    VioletToIndigo,

    /// Indigo → Blue: Add Logos
    IndigoToBlue,

    /// Blue → Green: Add Light/Love
    BlueToGreen,

    /// Green → Yellow: Add Dimensions/Veil
    GreenToYellow,

    /// Yellow → Orange: Add Galactic Logoi
    YellowToOrange,

    /// Orange → Red: Add Solar Logoi
    OrangeToRed,

    /// Red → Layer 7: Add Individual entities
    RedToLayer7,
}

impl LayerTransition {
    /// Get all layer transitions in order
    pub fn all_transitions() -> Vec<LayerTransition> {
        vec![
            LayerTransition::VioletToIndigo,
            LayerTransition::IndigoToBlue,
            LayerTransition::BlueToGreen,
            LayerTransition::GreenToYellow,
            LayerTransition::YellowToOrange,
            LayerTransition::OrangeToRed,
            LayerTransition::RedToLayer7,
        ]
    }

    /// Get the source layer
    pub fn source_layer(&self) -> &str {
        match self {
            LayerTransition::VioletToIndigo => "Violet",
            LayerTransition::IndigoToBlue => "Indigo",
            LayerTransition::BlueToGreen => "Blue",
            LayerTransition::GreenToYellow => "Green",
            LayerTransition::YellowToOrange => "Yellow",
            LayerTransition::OrangeToRed => "Orange",
            LayerTransition::RedToLayer7 => "Red",
        }
    }

    /// Get the target layer
    pub fn target_layer(&self) -> &str {
        match self {
            LayerTransition::VioletToIndigo => "Indigo",
            LayerTransition::IndigoToBlue => "Blue",
            LayerTransition::BlueToGreen => "Green",
            LayerTransition::GreenToYellow => "Yellow",
            LayerTransition::YellowToOrange => "Orange",
            LayerTransition::OrangeToRed => "Red",
            LayerTransition::RedToLayer7 => "Layer 7",
        }
    }

    /// Get the feature added at this transition
    pub fn feature(&self) -> Feature {
        match self {
            LayerTransition::VioletToIndigo => Feature::FreeWill,
            LayerTransition::IndigoToBlue => Feature::Logos,
            LayerTransition::BlueToGreen => Feature::LightLove,
            LayerTransition::GreenToYellow => Feature::Dimensions,
            LayerTransition::YellowToOrange => Feature::GalacticLogos,
            LayerTransition::OrangeToRed => Feature::SolarLogos,
            LayerTransition::RedToLayer7 => Feature::Individuality,
        }
    }
}

impl fmt::Display for LayerTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source_layer(), self.target_layer())
    }
}

/// Attractor field for evolution
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each layer creates an attractor-field that pulls evolution toward the next stage"
///
/// Attractor fields are not deterministic forces - they are probabilistic
/// influences that guide evolution while preserving Free Will.
#[derive(Debug, Clone)]
pub struct AttractorField {
    /// Name of the attractor field
    pub name: String,

    /// Strength of the attractor (0.0 to 1.0)
    pub strength: f64,

    /// Orientation of the attractor (STO or STS)
    pub orientation: Orientation,

    /// Archetypical signature of the attractor
    pub archetype_signature: [f64; 22],

    /// Target density this attractor leads toward
    pub target_density: Option<TargetDensity>,
}

/// Orientation of an attractor field
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// - STO: Service to Others (polarity of unity)
/// - STS: Service to Self (polarity of separation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Service to Others
    STO,

    /// Service to Self
    STS,

    /// Balanced / Undecided
    Balanced,
}

impl Orientation {
    /// Get orientation name
    pub fn name(&self) -> &str {
        match self {
            Orientation::STO => "Service to Others",
            Orientation::STS => "Service to Self",
            Orientation::Balanced => "Balanced",
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Target density for attractor field
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// The density octave represents 8 stages of consciousness evolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetDensity {
    /// 1st Density: Awareness (mineral kingdom)
    First,

    /// 2nd Density: Growth (plant and animal kingdoms)
    Second,

    /// 3rd Density: Self-awareness (humanity)
    Third,

    /// 4th Density: Love/Understanding
    Fourth,

    /// 5th Density: Light/Wisdom
    Fifth,

    /// 6th Density: Unity/Equality
    Sixth,

    /// 7th Density: Gateway/Cycle
    Seventh,

    /// 8th Density: Return to Infinity
    Eighth,
}

impl TargetDensity {
    /// Get density number (1-8)
    pub fn number(&self) -> u8 {
        match self {
            TargetDensity::First => 1,
            TargetDensity::Second => 2,
            TargetDensity::Third => 3,
            TargetDensity::Fourth => 4,
            TargetDensity::Fifth => 5,
            TargetDensity::Sixth => 6,
            TargetDensity::Seventh => 7,
            TargetDensity::Eighth => 8,
        }
    }

    /// Get density name
    pub fn name(&self) -> &str {
        match self {
            TargetDensity::First => "1st Density - Awareness",
            TargetDensity::Second => "2nd Density - Growth",
            TargetDensity::Third => "3rd Density - Self-awareness",
            TargetDensity::Fourth => "4th Density - Love/Understanding",
            TargetDensity::Fifth => "5th Density - Light/Wisdom",
            TargetDensity::Sixth => "6th Density - Unity/Equality",
            TargetDensity::Seventh => "7th Density - Gateway",
            TargetDensity::Eighth => "8th Density - Return to Infinity",
        }
    }
}

impl fmt::Display for TargetDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AttractorField {
    /// Create a new attractor field
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the attractor field
    /// * `strength` - Strength of the attractor (0.0 to 1.0)
    /// * `orientation` - Orientation (STO, STS, or Balanced)
    /// * `target_density` - Target density this attractor leads toward
    pub fn new(
        name: String,
        strength: f64,
        orientation: Orientation,
        target_density: TargetDensity,
    ) -> Self {
        Self {
            name,
            strength: strength.clamp(0.0, 1.0),
            orientation,
            archetype_signature: [0.5; 22], // Default signature
            target_density: Some(target_density),
        }
    }

    /// Create an attractor field with custom archetype signature
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the attractor field
    /// * `strength` - Strength of the attractor (0.0 to 1.0)
    /// * `orientation` - Orientation (STO, STS, or Balanced)
    /// * `archetype_signature` - 22 archetype coefficients
    /// * `target_density` - Target density this attractor leads toward
    pub fn with_signature(
        name: String,
        strength: f64,
        orientation: Orientation,
        archetype_signature: [f64; 22],
        target_density: TargetDensity,
    ) -> Self {
        Self {
            name,
            strength: strength.clamp(0.0, 1.0),
            orientation,
            archetype_signature,
            target_density: Some(target_density),
        }
    }

    /// Calculate influence on an entity
    ///
    /// The attractor influence depends on:
    /// - Strength of the attractor
    /// - Alignment with entity's archetype signature
    /// - Entity's current spectrum position
    ///
    /// # Arguments
    ///
    /// * `entity_signature` - Entity's archetype signature
    /// * `entity_spectrum` - Entity's spectrum position (0.0 to ∞, 1.0 = veil)
    ///
    /// # Returns
    ///
    /// Influence value (0.0 to 1.0)
    pub fn calculate_influence(&self, entity_signature: &[f64; 22], entity_spectrum: f64) -> f64 {
        // Calculate archetype alignment
        let alignment = self.calculate_archetype_alignment(entity_signature);

        // Adjust for spectrum position
        // Entities closer to target density feel stronger influence
        let spectrum_factor = if let Some(target) = self.target_density {
            let target_spectrum = (target.number() as f64) * 10.0;
            let distance = (entity_spectrum - target_spectrum).abs();
            1.0 / (1.0 + distance / 10.0)
        } else {
            0.5
        };

        // Combine strength, alignment, and spectrum factor
        self.strength * alignment * spectrum_factor
    }

    /// Calculate archetype alignment
    ///
    /// Measures how well the entity's archetype signature aligns with
    /// the attractor's signature.
    ///
    /// # Arguments
    ///
    /// * `entity_signature` - Entity's archetype signature
    ///
    /// # Returns
    ///
    /// Alignment value (0.0 to 1.0, higher = better alignment)
    fn calculate_archetype_alignment(&self, entity_signature: &[f64; 22]) -> f64 {
        let mut sum = 0.0;
        for (a, e) in self.archetype_signature.iter().zip(entity_signature.iter()) {
            sum += 1.0 - (a - e).abs();
        }
        sum / 22.0
    }

    /// Get attractor name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get attractor strength
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Get attractor orientation
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Get target density
    pub fn target_density(&self) -> Option<TargetDensity> {
        self.target_density
    }
}

impl fmt::Display for AttractorField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AttractorField({}, strength: {:.2}, orientation: {}, target: {})",
            self.name,
            self.strength,
            self.orientation,
            self.target_density
                .map(|d| d.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}

/// Builder for creating TranscendInclude operations
///
/// Provides a fluent interface for creating TranscendInclude structures.
pub struct TranscendIncludeBuilder<L1, F, A> {
    includes: Option<L1>,
    transcends: Option<F>,
    evolves_into: Option<A>,
}

impl<L1, F, A> TranscendIncludeBuilder<L1, F, A> {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            includes: None,
            transcends: None,
            evolves_into: None,
        }
    }

    /// Set what is included from previous stage
    pub fn includes(mut self, includes: L1) -> Self {
        self.includes = Some(includes);
        self
    }

    /// Set what transcends (new development)
    pub fn transcends(mut self, transcends: F) -> Self {
        self.transcends = Some(transcends);
        self
    }

    /// Set what evolves into (attractor field)
    pub fn evolves_into(mut self, evolves_into: A) -> Self {
        self.evolves_into = Some(evolves_into);
        self
    }

    /// Build the TranscendInclude structure
    ///
    /// # Panics
    ///
    /// Panics if any field is not set
    pub fn build(self) -> TranscendInclude<L1, F, A> {
        TranscendInclude {
            includes: self.includes.expect("includes must be set"),
            transcends: self.transcends.expect("transcends must be set"),
            evolves_into: self.evolves_into.expect("evolves_into must be set"),
        }
    }
}

impl<L1, F, A> Default for TranscendIncludeBuilder<L1, F, A> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcend_include_creation() {
        let previous = "Violet Realm";
        let new_feature = Feature::FreeWill;
        let next_attractor = AttractorField::new(
            "Indigo Attractor".to_string(),
            0.8,
            Orientation::Balanced,
            TargetDensity::Second,
        );

        let transcend_include = TranscendInclude::apply(previous, new_feature, next_attractor);

        assert_eq!(transcend_include.includes, "Violet Realm");
    }

    #[test]
    fn test_transcend_include_generate_next_stage() {
        let previous = "Violet Realm";
        let new_feature = Feature::FreeWill;
        let next_attractor = AttractorField::new(
            "Indigo Attractor".to_string(),
            0.8,
            Orientation::Balanced,
            TargetDensity::Second,
        );

        let transcend_include = TranscendInclude::apply(previous, new_feature, next_attractor);

        let next_stage = transcend_include.generate_next_stage();
        assert_eq!(next_stage, "Violet Realm");
    }

    #[test]
    fn test_feature_name() {
        assert_eq!(Feature::FreeWill.name(), "Free Will");
        assert_eq!(Feature::Logos.name(), "Logos");
        assert_eq!(Feature::LightLove.name(), "Light/Love");
    }

    #[test]
    fn test_feature_transition_layer() {
        assert_eq!(
            Feature::FreeWill.transition_layer(),
            Some(LayerTransition::VioletToIndigo)
        );
        assert_eq!(
            Feature::Logos.transition_layer(),
            Some(LayerTransition::IndigoToBlue)
        );
        assert_eq!(
            Feature::Individuality.transition_layer(),
            Some(LayerTransition::RedToLayer7)
        );
    }

    #[test]
    fn test_layer_transition_display() {
        let transition = LayerTransition::VioletToIndigo;
        assert_eq!(transition.to_string(), "Violet → Indigo");

        let transition = LayerTransition::RedToLayer7;
        assert_eq!(transition.to_string(), "Red → Layer 7");
    }

    #[test]
    fn test_layer_transition_feature() {
        assert_eq!(LayerTransition::VioletToIndigo.feature(), Feature::FreeWill);
        assert_eq!(LayerTransition::IndigoToBlue.feature(), Feature::Logos);
        assert_eq!(LayerTransition::BlueToGreen.feature(), Feature::LightLove);
    }

    #[test]
    fn test_layer_transition_source_target() {
        assert_eq!(LayerTransition::VioletToIndigo.source_layer(), "Violet");
        assert_eq!(LayerTransition::VioletToIndigo.target_layer(), "Indigo");
    }

    #[test]
    fn test_attractor_field_creation() {
        let attractor = AttractorField::new(
            "Test Attractor".to_string(),
            0.75,
            Orientation::STO,
            TargetDensity::Fourth,
        );

        assert_eq!(attractor.name(), "Test Attractor");
        assert_eq!(attractor.strength(), 0.75);
        assert_eq!(attractor.orientation(), Orientation::STO);
        assert_eq!(attractor.target_density(), Some(TargetDensity::Fourth));
    }

    #[test]
    fn test_attractor_field_with_signature() {
        let signature = [1.0; 22];
        let attractor = AttractorField::with_signature(
            "Signature Attractor".to_string(),
            0.9,
            Orientation::STS,
            signature,
            TargetDensity::Fifth,
        );

        assert_eq!(attractor.archetype_signature, [1.0; 22]);
    }

    #[test]
    fn test_attractor_field_influence() {
        let signature = [0.5; 22];
        let attractor = AttractorField::with_signature(
            "Test Attractor".to_string(),
            1.0,
            Orientation::STO,
            signature,
            TargetDensity::Fourth,
        );

        let entity_signature = [0.5; 22];
        let influence = attractor.calculate_influence(&entity_signature, 40.0);

        // With perfect alignment, influence should equal strength
        assert!((influence - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_attractor_field_archetype_alignment() {
        let attractor = AttractorField::new(
            "Test Attractor".to_string(),
            1.0,
            Orientation::STO,
            TargetDensity::Fourth,
        );

        let perfect_match = [0.5; 22];
        let alignment = attractor.calculate_archetype_alignment(&perfect_match);
        assert!((alignment - 1.0).abs() < 0.01);

        let mismatched = [1.0; 22];
        let alignment = attractor.calculate_archetype_alignment(&mismatched);
        assert!(alignment < 1.0);
        assert!(alignment >= 0.0);
    }

    #[test]
    fn test_orientation_display() {
        assert_eq!(Orientation::STO.to_string(), "Service to Others");
        assert_eq!(Orientation::STS.to_string(), "Service to Self");
        assert_eq!(Orientation::Balanced.to_string(), "Balanced");
    }

    #[test]
    fn test_target_density_number() {
        assert_eq!(TargetDensity::First.number(), 1);
        assert_eq!(TargetDensity::Third.number(), 3);
        assert_eq!(TargetDensity::Eighth.number(), 8);
    }

    #[test]
    fn test_target_density_name() {
        assert_eq!(TargetDensity::First.name(), "1st Density - Awareness");
        assert_eq!(TargetDensity::Third.name(), "3rd Density - Self-awareness");
        assert_eq!(
            TargetDensity::Eighth.name(),
            "8th Density - Return to Infinity"
        );
    }

    #[test]
    fn test_transcend_include_builder() {
        let builder = TranscendIncludeBuilder::new()
            .includes("Previous")
            .transcends(Feature::Logos)
            .evolves_into(AttractorField::new(
                "Next Attractor".to_string(),
                0.8,
                Orientation::Balanced,
                TargetDensity::Third,
            ));

        let transcend_include = builder.build();

        assert_eq!(transcend_include.includes, "Previous");
    }
}
