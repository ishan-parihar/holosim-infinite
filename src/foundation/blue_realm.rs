use crate::foundation::indigo_realm::IntelligentInfinity;
/// Layer 2: Blue-Ray Realm - Love/Light + Universal Archetypical Patterns
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Second Distortion: Love/Logos"
/// "Awareness focused infinity into infinite energy, the Creative Principle"
/// "The focusing of infinity as an aware or conscious principle"
/// "The great activator and primal co-Creator"
/// "Love/Light — Love first, then Light (energy source)"
///
/// Action: Second Distortion — Love (Logos)
///
/// Result: Logos emerges (Creative Principle)
///
/// Architectural Artifact: Universal Archetypical Patterns (Foundation)
///
/// INCLUDES: Violet-Ray + Indigo-Ray (Infinity + IntelligentInfinity)
///
/// TRANSCENDS: Adds Love/Logos and Universal Archetypical Patterns
///
/// EVOLVES INTO: Attractor-field for Green-Ray (Light/Love field)
///
/// Deep Logic:
/// - The Light at the Blue-Ray Realm was architected with universal patterns
/// - IntelligentInfinity, having become aware through Free Will at Indigo-Ray,
///   now focuses itself into the Creative Principle: the Logos
/// - The Logos is the primordial individualized consciousness closest to Infinity/Creator
///
/// The Three-Tier Refinement Process:
/// 1. **Cosmic Mind**: Universal field of potential, the same for all sub-Logoi
/// 2. **Primary Logos Refinement**: The Logos refines the cosmic mind by its own bias,
///    articulating it in a pattern peculiar to its choice
/// 3. **Sub-Logos Refinement**: Sub-Logoi (Solar-Logoi) further refine these universal
///    patterns into specific archetypical mind systems
///
/// Universal Archetypical Patterns:
/// - The Logos engaged in several experiments in creation as the Creative Principle
/// - Through evolving in the process of creation, it accumulated experiences
/// - These experiences revealed patterns—the fundamental operations of consciousness
/// - The Logos codified these patterns into universal archetypical patterns
/// - The specific structure of these universal patterns is NOT specified in the source
///   material (this is the UNKNOWN to be discovered!)
///
/// IMPORTANT NOTE:
/// - Do NOT implement the 22-archetype system here!
/// - The 22-archetype system is a Solar-Logos feature at Red-Ray
/// - At Blue-Ray, we implement Universal Archetypical Patterns (foundation)
/// - At Red-Ray, Solar-Logoi refine these patterns into specific systems (10 or 22 archetypes)
use crate::foundation::transcend_include::{AttractorField, Feature};
use std::fmt;

/// Cosmic Mind - Universal field of potential
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Universal field of potential, the same for all sub-Logoi"
///
/// This is the first tier of the three-tier refinement process.
/// The cosmic mind is the same for all sub-Logoi - it's the universal field
/// of potential that exists before any refinement.
#[derive(Debug, Clone, PartialEq)]
pub struct CosmicMind {
    /// The potential field (0.0 to 1.0)
    pub potential_field: f64,
    /// The coherence of the cosmic mind (0.0 to 1.0)
    pub coherence: f64,
}

impl Default for CosmicMind {
    fn default() -> Self {
        CosmicMind {
            potential_field: 1.0,
            coherence: 1.0,
        }
    }
}

impl CosmicMind {
    /// Create a new Cosmic Mind
    pub fn new() -> Self {
        CosmicMind::default()
    }

    /// Get the potential field
    pub fn potential_field(&self) -> f64 {
        self.potential_field
    }

    /// Get the coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }
}

/// Universal Archetypical Patterns - Foundation
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Logos codified these patterns into universal archetypical patterns"
/// "The specific structure of these universal patterns is NOT specified in the
/// source material (this is the UNKNOWN to be discovered!)"
///
/// IMPORTANT: This is NOT the 22-archetype system!
/// The 22-archetype system is a Solar-Logos-specific refinement at Red-Ray.
/// At Blue-Ray, we implement the universal patterns (foundation) that Solar-Logoi
/// will later refine into specific systems.
///
/// The structure of these universal patterns is UNKNOWN and to be discovered
/// through simulation and exploration.
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalArchetypicalPatterns {
    /// The patterns themselves (structure unknown)
    ///
    /// In a real implementation, this would contain the actual patterns.
    /// For now, we use a placeholder to represent the unknown structure.
    pub patterns: Vec<String>,
    /// The coherence of the patterns (0.0 to 1.0)
    pub coherence: f64,
    /// The complexity of the patterns (0.0 to 1.0)
    pub complexity: f64,
}

impl Default for UniversalArchetypicalPatterns {
    fn default() -> Self {
        UniversalArchetypicalPatterns {
            patterns: Vec::new(),
            coherence: 1.0,
            complexity: 0.5,
        }
    }
}

impl UniversalArchetypicalPatterns {
    /// Create a new set of Universal Archetypical Patterns
    ///
    /// The structure is UNKNOWN - this is to be discovered.
    pub fn new() -> Self {
        UniversalArchetypicalPatterns::default()
    }

    /// Add a pattern placeholder
    ///
    /// In a real implementation, this would add actual patterns.
    /// For now, we add placeholders to represent the unknown structure.
    pub fn add_pattern(&mut self, pattern: impl Into<String>) {
        self.patterns.push(pattern.into());
    }

    /// Get the number of patterns
    pub fn count(&self) -> usize {
        self.patterns.len()
    }

    /// Get the coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Get the complexity
    pub fn complexity(&self) -> f64 {
        self.complexity
    }

    /// Get the description of Universal Archetypical Patterns
    pub fn description(&self) -> String {
        "Universal Archetypical Patterns: The foundation patterns codified by the Logos \
        through its experiences in creation. The specific structure is UNKNOWN and to be \
        discovered. These are NOT the 22-archetype system - that is a Solar-Logos-specific \
        refinement at Red-Ray. At Blue-Ray, we implement the universal patterns (foundation) \
        that Solar-Logoi will later refine into specific systems (10 or 22 archetypes)."
            .to_string()
    }
}

/// Logos - The Creative Principle
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The focusing of infinity as an aware or conscious principle"
/// "The great activator and primal co-Creator"
/// "The Logos is the primordial individualized consciousness closest to Infinity/Creator"
///
/// This is the result of applying the Second Distortion (Love/Logos) to IntelligentInfinity.
#[derive(Debug, Clone, PartialEq)]
pub struct Logos {
    /// The IntelligentInfinity (Violet + Indigo) - INCLUDED
    pub intelligent_infinity: IntelligentInfinity,
    /// The Cosmic Mind - the universal field of potential
    pub cosmic_mind: CosmicMind,
    /// The Universal Archetypical Patterns - the patterns codified by the Logos
    pub universal_patterns: UniversalArchetypicalPatterns,
    /// The focusing strength (0.0 to 1.0)
    pub focusing_strength: f64,
}

impl Logos {
    /// Create a new Logos (legacy API for backward compatibility)
    ///
    /// Creates with default IntelligentInfinity
    pub fn new() -> Self {
        let intelligent_infinity = IntelligentInfinity::new();
        let cosmic_mind = CosmicMind::new();
        let universal_patterns = UniversalArchetypicalPatterns::new();

        Logos {
            intelligent_infinity,
            cosmic_mind,
            universal_patterns,
            focusing_strength: 1.0,
        }
    }

    /// Create a new Logos from IntelligentInfinity
    ///
    /// This applies the Second Distortion (Love/Logos) to IntelligentInfinity.
    pub fn from_intelligent_infinity(intelligent_infinity: IntelligentInfinity) -> Self {
        let cosmic_mind = CosmicMind::new();
        let universal_patterns = UniversalArchetypicalPatterns::new();

        Logos {
            intelligent_infinity,
            cosmic_mind,
            universal_patterns,
            focusing_strength: 1.0,
        }
    }

    /// Get the focusing strength
    pub fn focusing_strength(&self) -> f64 {
        self.focusing_strength
    }

    /// Check if this is truly focused
    pub fn is_focused(&self) -> bool {
        self.focusing_strength > 0.0
    }

    /// Refine the cosmic mind with Logos bias
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Logos refines the cosmic mind by its own bias, articulating it in a pattern
    /// peculiar to its choice"
    ///
    /// This is the second tier of the three-tier refinement process.
    pub fn refine_cosmic_mind(&mut self, bias: impl Into<String>) {
        self.universal_patterns
            .add_pattern(format!("Refined pattern: {}", bias.into()));
        self.focusing_strength = 1.0;
    }

    /// Apply the Third Distortion (Light) to transition to Green-Ray
    ///
    /// Returns: (Logos, Feature, AttractorField)
    /// - Logos: The included source state
    /// - Feature: The Third Distortion (Light)
    /// - AttractorField: Light/Love field of potential
    pub fn apply_third_distortion(&self) -> (Logos, Feature, AttractorField) {
        let blue_included = self.clone();

        let third_distortion = Feature::new(
            "Third Distortion: Light",
            "The manifestation when light has been impressed with love",
            1.0,
        );

        let light_love_field = AttractorField::new(
            "Light/Love Field of Potential",
            1.0,
            "Conditions for Space/Time and Time/Space Spectrum emerge",
        );

        (blue_included, third_distortion, light_love_field)
    }

    /// Get the description of Logos
    pub fn description(&self) -> String {
        "Logos: The Creative Principle. Awareness focused infinity into infinite energy. \
        The focusing of infinity as an aware or conscious principle. The great activator \
        and primal co-Creator. Love/Light — Love first, then Light (energy source). The \
        primordial individualized consciousness closest to Infinity/Creator."
            .to_string()
    }
}

impl Default for Logos {
    fn default() -> Self {
        Logos {
            intelligent_infinity: IntelligentInfinity::default(),
            cosmic_mind: CosmicMind::default(),
            universal_patterns: UniversalArchetypicalPatterns::default(),
            focusing_strength: 0.0,
        }
    }
}

impl fmt::Display for Logos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Logos: Focusing={}, CosmicMind={}, Patterns={}, Awareness={}",
            self.focusing_strength,
            self.cosmic_mind.coherence,
            self.universal_patterns.count(),
            self.intelligent_infinity.awareness()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmic_mind_creation() {
        let cosmic = CosmicMind::new();
        assert_eq!(cosmic.potential_field, 1.0);
        assert_eq!(cosmic.coherence, 1.0);
    }

    #[test]
    fn test_cosmic_mind_default() {
        let cosmic = CosmicMind::default();
        assert_eq!(cosmic.potential_field, 1.0);
        assert_eq!(cosmic.coherence, 1.0);
    }

    #[test]
    fn test_universal_patterns_creation() {
        let patterns = UniversalArchetypicalPatterns::new();
        assert_eq!(patterns.count(), 0);
        assert_eq!(patterns.coherence, 1.0);
        assert_eq!(patterns.complexity, 0.5);
    }

    #[test]
    fn test_add_pattern() {
        let mut patterns = UniversalArchetypicalPatterns::new();
        patterns.add_pattern("Test pattern");
        assert_eq!(patterns.count(), 1);
    }

    #[test]
    fn test_universal_patterns_description() {
        let patterns = UniversalArchetypicalPatterns::new();
        let desc = patterns.description();
        assert!(desc.contains("Universal Archetypical Patterns"));
        assert!(desc.contains("UNKNOWN"));
        assert!(desc.contains("NOT the 22-archetype system"));
    }

    #[test]
    fn test_logos_from_intelligent_infinity() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        assert_eq!(logos.focusing_strength, 1.0);
        assert!(logos.is_focused());
    }

    #[test]
    fn test_logos_includes_intelligent_infinity() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet.clone());
        let logos = Logos::from_intelligent_infinity(intelligent.clone());
        assert_eq!(
            logos.intelligent_infinity.awareness(),
            intelligent.awareness()
        );
        assert_eq!(logos.intelligent_infinity.violet_realm.unity, violet.unity);
    }

    #[test]
    fn test_logos_has_cosmic_mind() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        assert_eq!(logos.cosmic_mind.coherence, 1.0);
    }

    #[test]
    fn test_logos_has_universal_patterns() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        assert_eq!(logos.universal_patterns.coherence, 1.0);
    }

    #[test]
    fn test_refine_cosmic_mind() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let mut logos = Logos::from_intelligent_infinity(intelligent);
        logos.refine_cosmic_mind("Test bias");
        assert_eq!(logos.universal_patterns.count(), 1);
    }

    #[test]
    fn test_apply_third_distortion() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let (blue_included, feature, attractor) = logos.apply_third_distortion();

        assert_eq!(blue_included.focusing_strength, 1.0);
        assert_eq!(feature.name, "Third Distortion: Light");
        assert_eq!(attractor.name, "Light/Love Field of Potential");
    }

    #[test]
    fn test_logos_description() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let desc = logos.description();
        assert!(desc.contains("Logos"));
        assert!(desc.contains("Creative Principle"));
        assert!(desc.contains("Love/Light"));
    }

    #[test]
    fn test_logos_display() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let display = format!("{}", logos);
        assert!(display.contains("Logos"));
        assert!(display.contains("Focusing=1"));
    }
}
