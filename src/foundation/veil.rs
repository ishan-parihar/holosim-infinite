use crate::types::{Density, Float};

/// The Veil - structural feature of dimensional architecture.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Veil is NOT a barrier separating two realms—it is a STRUCTURAL FEATURE
///  of dimensional architecture formed at Yellow-Ray by the fundamental opposition
///  between Time/Space Oneness and Space/Time Many-ness."
///
/// The Veil:
/// - Limits ACCESS to the Oneness side of the spectrum
/// - Creates the ILLUSION of separation
/// - Provides contrast, limitation, challenge, choice, growth
/// - Implements Free Will (First Distortion becomes meaningful)
#[derive(Debug, Clone)]
pub struct Veil {
    /// Position on spectrum (always at v = 1)
    #[allow(dead_code)]
    position: Float,

    /// Thickness varies by density
    thickness: Float,

    /// Thin spots (accumulated experience creates holes)
    thin_spots: Vec<ThinSpot>,
}

/// A thin spot in the veil - allows glimpses through
#[derive(Debug, Clone)]
pub struct ThinSpot {
    /// Location in experience-space
    pub trigger: CatalystTrigger,

    /// Size of the thin spot
    pub size: Float,

    /// How much can be seen through
    pub transparency: Float,
}

/// What triggers a thin spot in the veil
#[derive(Debug, Clone, PartialEq)]
pub enum CatalystTrigger {
    IntenseEmotion,
    NearDeath,
    DeepMeditation,
    Trauma,
    LoveOverflow,
    WisdomThreshold,
}

/// Content that may pass through the veil
#[derive(Debug, Clone)]
pub struct Content {
    pub relevance: Float,
    pub emotional_intensity: Float,
}

/// Content filtered by the veil
#[derive(Debug, Clone)]
pub struct FilteredContent {
    pub content: Content,
    pub filtered_intensity: Float,
}

/// Situation that may trigger thin spot access
#[derive(Debug, Clone, PartialEq)]
pub enum Situation {
    Emotional,
    LifeThreatening,
    Meditative,
    Traumatic,
    Loving,
    WisdomSeeking,
}

impl Veil {
    /// Create a new veil for given density
    pub fn new(density: Density) -> Self {
        Self {
            position: 1.0,
            thickness: density.veil_thickness(),
            thin_spots: Vec::new(),
        }
    }

    /// Check if something can pass through the veil
    pub fn can_pass(&self, transparency: Float, thin_spot: Option<&ThinSpot>) -> bool {
        let effective_thickness = match thin_spot {
            Some(spot) => self.thickness * (1.0 - spot.transparency),
            None => self.thickness,
        };

        transparency > effective_thickness
    }

    /// Filter content based on veil transparency
    pub fn filter_content(&self, content: &Content, density: Density) -> Option<FilteredContent> {
        let transparency = density.veil_transparency();

        let relevance = content.relevance;
        let emotional_intensity = content.emotional_intensity;

        let pass_prob = transparency * relevance * emotional_intensity;

        if pass_prob > self.thickness {
            Some(FilteredContent {
                content: content.clone(),
                filtered_intensity: pass_prob,
            })
        } else {
            None
        }
    }

    /// Create a thin spot from accumulated experience
    pub fn create_thin_spot(&mut self, trigger: CatalystTrigger, intensity: Float) {
        let thin_spot = ThinSpot {
            trigger,
            size: intensity * 0.1,
            transparency: intensity * 0.5,
        };

        self.thin_spots.push(thin_spot);
    }

    /// Get applicable thin spot for a situation
    pub fn get_thin_spot(&self, situation: &Situation) -> Option<&ThinSpot> {
        self.thin_spots
            .iter()
            .find(|spot| matches!((&spot.trigger, situation),
                (CatalystTrigger::IntenseEmotion, Situation::Emotional) |
                (CatalystTrigger::NearDeath, Situation::LifeThreatening) |
                (CatalystTrigger::DeepMeditation, Situation::Meditative) |
                (CatalystTrigger::Trauma, Situation::Traumatic) |
                (CatalystTrigger::LoveOverflow, Situation::Loving) |
                (CatalystTrigger::WisdomThreshold, Situation::WisdomSeeking)))
    }

    /// Get veil thickness
    pub fn thickness(&self) -> Float {
        self.thickness
    }

    /// Get number of thin spots
    pub fn thin_spot_count(&self) -> usize {
        self.thin_spots.len()
    }
}

impl Default for Veil {
    fn default() -> Self {
        Self::new(Density::Third)
    }
}

// Add comprehensive tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veil_creation() {
        let veil = Veil::new(Density::Third);
        assert_eq!(veil.position, 1.0);
        assert_eq!(veil.thickness(), 0.7);
    }

    #[test]
    fn test_veil_can_pass() {
        let veil = Veil::new(Density::Third);

        // High transparency passes
        assert!(veil.can_pass(0.8, None));

        // Low transparency doesn't pass
        assert!(!veil.can_pass(0.5, None));
    }

    #[test]
    fn test_veil_can_pass_with_thin_spot() {
        let mut veil = Veil::new(Density::Third);
        veil.create_thin_spot(CatalystTrigger::DeepMeditation, 0.8);

        let thin_spot = veil.get_thin_spot(&Situation::Meditative);
        assert!(thin_spot.is_some(), "thin spot should exist");
        // With transparency 0.8, effective thickness = 0.7 * 0.2 = 0.14
        // 0.3 > 0.14 = true
        assert!(veil.can_pass(0.3, thin_spot), "should pass with thin spot");
    }

    #[test]
    fn test_veil_filter_content() {
        let veil = Veil::new(Density::Third);
        let content = Content {
            relevance: 0.8,
            emotional_intensity: 0.9,
        };

        // Sixth density has high transparency, so should pass
        let filtered = veil.filter_content(&content, Density::Sixth);
        assert!(filtered.is_some(), "high density should pass through");
    }

    #[test]
    fn test_veil_filter_content_blocked() {
        let veil = Veil::new(Density::Third);
        // First density has very low transparency
        let content = Content {
            relevance: 0.3,
            emotional_intensity: 0.2,
        };

        let filtered = veil.filter_content(&content, Density::First);
        // With low relevance and emotional intensity, should be blocked
        assert!(filtered.is_none(), "low relevance should be blocked");
    }

    #[test]
    fn test_thin_spot_creation() {
        let mut veil = Veil::new(Density::Third);
        veil.create_thin_spot(CatalystTrigger::LoveOverflow, 0.7);

        assert_eq!(veil.thin_spot_count(), 1);
    }

    #[test]
    fn test_thin_spot_matching() {
        let mut veil = Veil::new(Density::Third);
        veil.create_thin_spot(CatalystTrigger::IntenseEmotion, 0.8);
        veil.create_thin_spot(CatalystTrigger::DeepMeditation, 0.7);

        let spot = veil.get_thin_spot(&Situation::Emotional);
        assert!(spot.is_some());
        assert_eq!(spot.unwrap().trigger, CatalystTrigger::IntenseEmotion);
    }
}
