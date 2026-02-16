use crate::foundation::blue_realm::Logos;
/// Layer 3: Green-Ray Realm - Light/Love field of potential
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Third Distortion: Light"
/// "The first distortion of intelligent infinity, the building block of matter"
/// "Intelligent and full of energy, vibrational distortion of infinity"
/// "Holographic patterns of energy appearing as entire creation in all directions"
/// "Light/Love — Light first, then Love (manifestation)"
///
/// Action: Third Distortion — Light
///
/// Result: Light/Love field of potential
///
/// Architectural Artifact: Conditions for Space/Time and Time/Space Spectrum
///
/// INCLUDES: All previous layers (Violet + Indigo + Blue)
///
/// TRANSCENDS: Adds Light/Love manifestation field
///
/// EVOLVES INTO: Attractor-field for Yellow-Ray (Dimensions/Spectrum/Veil)
///
/// Deep Logic:
/// - The Light, now architected with universal patterns, manifests as the field of potential
/// - This is the Green-Ray Realm—the realm where Light impressed with Love creates the
///   conditions for manifestation
/// - Light is the "enabler and power giver"—the building block of matter and the foundation
///   of all physical reality
///
/// The Process:
/// 1. The architected Light (from Blue-Ray) begins to manifest in various modes of knowing
/// 2. Holographic patterns of energy appear as entire creation in all directions
/// 3. Energy patterns regularize into rhythms and fields
/// 4. These patterns create the field of potential for dimensions and universes
///
/// What Emerges at Green Resolution:
/// - **Holographic Patterns**: Energy patterns appearing as entire creation in all directions
/// - **Rhythms and Fields**: Energy regularized into stable configurations
/// - **The Conditions for Spectrum**: The field contains the potential for the Space/Time
///   and Time/Space spectrum
/// - **The Field of Potential**: The infinite playground where dimensions can manifest
use crate::foundation::transcend_include::{AttractorField, Feature};
use std::fmt;

/// Holographic Pattern - Energy pattern appearing as entire creation
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Holographic patterns of energy appearing as entire creation in all directions"
///
/// These are the energy patterns that emerge at Green-Ray, where Light impressed
/// with Love creates the conditions for manifestation.
#[derive(Debug, Clone, PartialEq)]
pub struct HolographicPattern {
    /// The pattern energy level (0.0 to 1.0)
    pub energy_level: f64,
    /// The pattern direction (3D vector placeholder)
    pub direction: [f64; 3],
    /// The pattern complexity (0.0 to 1.0)
    pub complexity: f64,
}

impl HolographicPattern {
    /// Create a new holographic pattern
    pub fn new(energy_level: f64, direction: [f64; 3], complexity: f64) -> Self {
        assert!(
            energy_level >= 0.0 && energy_level <= 1.0,
            "Energy level must be between 0.0 and 1.0"
        );
        assert!(
            complexity >= 0.0 && complexity <= 1.0,
            "Complexity must be between 0.0 and 1.0"
        );
        HolographicPattern {
            energy_level,
            direction,
            complexity,
        }
    }

    /// Get the energy level
    pub fn energy_level(&self) -> f64 {
        self.energy_level
    }

    /// Get the direction
    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    /// Get the complexity
    pub fn complexity(&self) -> f64 {
        self.complexity
    }
}

/// Rhythm - Energy regularized into stable configuration
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Energy patterns regularize into rhythms and fields"
///
/// These are the rhythms that emerge as energy patterns stabilize into
/// regular configurations.
#[derive(Debug, Clone, PartialEq)]
pub struct Rhythm {
    /// The rhythm frequency (0.0 to 1.0)
    pub frequency: f64,
    /// The rhythm amplitude (0.0 to 1.0)
    pub amplitude: f64,
    /// The rhythm phase (0.0 to 2π)
    pub phase: f64,
}

impl Rhythm {
    /// Create a new rhythm
    pub fn new(frequency: f64, amplitude: f64, phase: f64) -> Self {
        assert!(
            frequency >= 0.0 && frequency <= 1.0,
            "Frequency must be between 0.0 and 1.0"
        );
        assert!(
            amplitude >= 0.0 && amplitude <= 1.0,
            "Amplitude must be between 0.0 and 1.0"
        );
        assert!(
            phase >= 0.0 && phase <= 2.0 * std::f64::consts::PI,
            "Phase must be between 0.0 and 2π"
        );
        Rhythm {
            frequency,
            amplitude,
            phase,
        }
    }

    /// Get the frequency
    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    /// Get the amplitude
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    /// Get the phase
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// Calculate the rhythm value at a given time
    pub fn value_at(&self, time: f64) -> f64 {
        self.amplitude * (2.0 * std::f64::consts::PI * self.frequency * time + self.phase).sin()
    }
}

/// Field - Energy regularized into field configuration
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Energy patterns regularize into rhythms and fields"
///
/// These are the fields that emerge as energy patterns stabilize into
/// field configurations.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The field strength (0.0 to 1.0)
    pub strength: f64,
    /// The field coherence (0.0 to 1.0)
    pub coherence: f64,
    /// The field type
    pub field_type: String,
}

impl Field {
    /// Create a new field
    pub fn new(strength: f64, coherence: f64, field_type: impl Into<String>) -> Self {
        assert!(
            strength >= 0.0 && strength <= 1.0,
            "Strength must be between 0.0 and 1.0"
        );
        assert!(
            coherence >= 0.0 && coherence <= 1.0,
            "Coherence must be between 0.0 and 1.0"
        );
        Field {
            strength,
            coherence,
            field_type: field_type.into(),
        }
    }

    /// Get the strength
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Get the coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Get the field type
    pub fn field_type(&self) -> &str {
        &self.field_type
    }
}

/// Light/Love Field of Potential - The conditions for manifestation
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The field of potential for dimensions and universes"
/// "The infinite playground where dimensions can manifest"
///
/// This is the result of applying the Third Distortion (Light) to the Logos.
/// It contains the conditions for the Space/Time and Time/Space spectrum.
#[derive(Debug, Clone, PartialEq)]
pub struct LightLoveField {
    /// The Logos (Violet + Indigo + Blue) - INCLUDED
    pub logos: Logos,
    /// The holographic patterns
    pub holographic_patterns: Vec<HolographicPattern>,
    /// The rhythms
    pub rhythms: Vec<Rhythm>,
    /// The fields
    pub fields: Vec<Field>,
    /// The potential strength (0.0 to 1.0)
    pub potential_strength: f64,
}

impl LightLoveField {
    /// Create a new Light/Love Field (legacy API for backward compatibility)
    ///
    /// Creates with default Logos
    pub fn new() -> Self {
        let logos = Logos::new();
        LightLoveField {
            logos,
            holographic_patterns: Vec::new(),
            rhythms: Vec::new(),
            fields: Vec::new(),
            potential_strength: 1.0,
        }
    }

    /// Create a new Light/Love Field from Logos
    ///
    /// This applies the Third Distortion (Light) to the Logos.
    pub fn from_logos(logos: Logos) -> Self {
        LightLoveField {
            logos,
            holographic_patterns: Vec::new(),
            rhythms: Vec::new(),
            fields: Vec::new(),
            potential_strength: 1.0,
        }
    }

    /// Get the potential strength
    pub fn potential_strength(&self) -> f64 {
        self.potential_strength
    }

    /// Add a holographic pattern
    pub fn add_holographic_pattern(&mut self, pattern: HolographicPattern) {
        self.holographic_patterns.push(pattern);
    }

    /// Add a rhythm
    pub fn add_rhythm(&mut self, rhythm: Rhythm) {
        self.rhythms.push(rhythm);
    }

    /// Add a field
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Get the number of holographic patterns
    pub fn holographic_pattern_count(&self) -> usize {
        self.holographic_patterns.len()
    }

    /// Get the number of rhythms
    pub fn rhythm_count(&self) -> usize {
        self.rhythms.len()
    }

    /// Get the number of fields
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    /// Check if the field has conditions for spectrum
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The field contains the potential for the Space/Time and Time/Space spectrum"
    pub fn has_spectrum_conditions(&self) -> bool {
        self.potential_strength > 0.0
            && !self.holographic_patterns.is_empty()
            && !self.rhythms.is_empty()
            && !self.fields.is_empty()
    }

    /// Apply the mysterious emergence to transition to Yellow-Ray
    ///
    /// Returns: (LightLoveField, Feature, AttractorField)
    /// - LightLoveField: The included source state
    /// - Feature: The Mysterious Emergence
    /// - AttractorField: Dimensions/Spectrum/Veil
    pub fn apply_mysterious_emergence(&self) -> (LightLoveField, Feature, AttractorField) {
        let green_included = self.clone();

        let mysterious_emergence = Feature::new(
            "The Mysterious Emergence",
            "The emergence of Space/Time and Time/Space Spectrum from Light/Love field",
            1.0,
        );

        let dimensions_spectrum_veil = AttractorField::new(
            "Dimensions/Spectrum/Veil",
            1.0,
            "Space/Time and Time/Space Spectrum emerges",
        );

        (
            green_included,
            mysterious_emergence,
            dimensions_spectrum_veil,
        )
    }

    /// Get the description of Light/Love Field
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Third Distortion: Light"
    /// "The first distortion of intelligent infinity, the building block of matter"
    pub fn description(&self) -> String {
        "Light/Love Field of Potential: The manifestation when light has been impressed with love. \
        Third Distortion: Light - The first distortion of intelligent infinity, the building block of matter. \
        Intelligent and full of energy, vibrational distortion of infinity. Holographic patterns of energy appearing \
        as entire creation in all directions. Energy patterns regularize into rhythms and fields. \
        The field contains the potential for the Space/Time and Time/Space spectrum. The infinite \
        playground where dimensions can manifest.".to_string()
    }
}

impl Default for LightLoveField {
    fn default() -> Self {
        LightLoveField {
            logos: Logos::default(),
            holographic_patterns: Vec::new(),
            rhythms: Vec::new(),
            fields: Vec::new(),
            potential_strength: 0.0,
        }
    }
}

impl fmt::Display for LightLoveField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Light/Love Field: Potential={}, Patterns={}, Rhythms={}, Fields={}, Logos={}",
            self.potential_strength,
            self.holographic_pattern_count(),
            self.rhythm_count(),
            self.field_count(),
            self.logos.focusing_strength()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_pattern_creation() {
        let pattern = HolographicPattern::new(0.8, [1.0, 0.0, 0.0], 0.7);
        assert_eq!(pattern.energy_level, 0.8);
        assert_eq!(pattern.direction, [1.0, 0.0, 0.0]);
        assert_eq!(pattern.complexity, 0.7);
    }

    #[test]
    fn test_rhythm_creation() {
        let rhythm = Rhythm::new(0.5, 0.8, std::f64::consts::PI);
        assert_eq!(rhythm.frequency, 0.5);
        assert_eq!(rhythm.amplitude, 0.8);
        assert_eq!(rhythm.phase, std::f64::consts::PI);
    }

    #[test]
    fn test_rhythm_value_at() {
        let rhythm = Rhythm::new(0.5, 0.8, 0.0);
        let value = rhythm.value_at(1.0);
        // At t=1.0, the value should be amplitude * sin(2π * frequency * time + phase)
        // = 0.8 * sin(2π * 0.5 * 1.0 + 0.0) = 0.8 * sin(π) = 0.0
        assert!((value - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_field_creation() {
        let field = Field::new(0.9, 0.85, "Test Field");
        assert_eq!(field.strength, 0.9);
        assert_eq!(field.coherence, 0.85);
        assert_eq!(field.field_type, "Test Field");
    }

    #[test]
    fn test_light_love_field_from_logos() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let field = LightLoveField::from_logos(logos);
        assert_eq!(field.potential_strength, 1.0);
    }

    #[test]
    fn test_light_love_field_includes_logos() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent =
            crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet.clone());
        let logos = Logos::from_intelligent_infinity(intelligent.clone());
        let field = LightLoveField::from_logos(logos.clone());
        assert_eq!(field.logos.focusing_strength(), logos.focusing_strength());
        assert_eq!(
            field.logos.intelligent_infinity.awareness(),
            intelligent.awareness()
        );
        assert_eq!(
            field.logos.intelligent_infinity.violet_realm.unity,
            violet.unity
        );
    }

    #[test]
    fn test_add_holographic_pattern() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);
        field.add_holographic_pattern(HolographicPattern::new(0.8, [1.0, 0.0, 0.0], 0.7));
        assert_eq!(field.holographic_pattern_count(), 1);
    }

    #[test]
    fn test_add_rhythm() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);
        field.add_rhythm(Rhythm::new(0.5, 0.8, 0.0));
        assert_eq!(field.rhythm_count(), 1);
    }

    #[test]
    fn test_add_field() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);
        field.add_field(Field::new(0.9, 0.85, "Test Field"));
        assert_eq!(field.field_count(), 1);
    }

    #[test]
    fn test_has_spectrum_conditions() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let mut field = LightLoveField::from_logos(logos);

        // Initially, no spectrum conditions
        assert!(!field.has_spectrum_conditions());

        // Add patterns, rhythms, and fields
        field.add_holographic_pattern(HolographicPattern::new(0.8, [1.0, 0.0, 0.0], 0.7));
        field.add_rhythm(Rhythm::new(0.5, 0.8, 0.0));
        field.add_field(Field::new(0.9, 0.85, "Test Field"));

        // Now has spectrum conditions
        assert!(field.has_spectrum_conditions());
    }

    #[test]
    fn test_apply_mysterious_emergence() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let field = LightLoveField::from_logos(logos);
        let (green_included, feature, attractor) = field.apply_mysterious_emergence();

        assert_eq!(green_included.potential_strength, 1.0);
        assert_eq!(feature.name, "The Mysterious Emergence");
        assert_eq!(attractor.name, "Dimensions/Spectrum/Veil");
    }

    #[test]
    fn test_light_love_field_description() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let field = LightLoveField::from_logos(logos);
        let desc = field.description();
        assert!(desc.contains("Light/Love Field of Potential"));
        assert!(desc.contains("Third Distortion"));
        assert!(desc.contains("Space/Time and Time/Space spectrum"));
    }

    #[test]
    fn test_light_love_field_display() {
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let intelligent = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet);
        let logos = Logos::from_intelligent_infinity(intelligent);
        let field = LightLoveField::from_logos(logos);
        let display = format!("{}", field);
        assert!(display.contains("Light/Love Field"));
        assert!(display.contains("Potential=1"));
    }
}
