//! Culture - Cultural traits and evolution
//!
//! Culture defines the values, beliefs, and practices of a civilization.
//! It influences technology development, polarization drift, and
//! consciousness evolution.

use rand::Rng;

// ============================================================================
// Cultural Trait
// ============================================================================

/// A specific cultural trait
#[derive(Debug, Clone)]
pub struct CulturalTrait {
    /// Name of the trait
    pub name: String,

    /// Strength of the trait (0-1)
    pub strength: f64,

    /// Category of the trait
    pub category: TraitCategory,
}

impl CulturalTrait {
    pub fn new(name: &str, strength: f64, category: TraitCategory) -> Self {
        Self {
            name: name.to_string(),
            strength: strength.clamp(0.0, 1.0),
            category,
        }
    }
}

/// Category of cultural trait
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitCategory {
    /// Values and beliefs
    Values,
    /// Social organization
    Social,
    /// Economic practices
    Economic,
    /// Spiritual/religious
    Spiritual,
    /// Artistic expression
    Artistic,
}

// ============================================================================
// Ideology
// ============================================================================

/// Major ideological orientation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ideology {
    /// Service to Others (positive polarization)
    ServiceToOthers,

    /// Service to Self (negative polarization)
    ServiceToSelf,

    /// Mixed/Balanced
    Mixed,

    /// Spiritual focus
    Spiritual,

    /// Material focus
    Material,
}

impl Ideology {
    /// Get polarization drift for this ideology
    pub fn polarization_drift(&self) -> f64 {
        match self {
            Ideology::ServiceToOthers => 0.1,
            Ideology::ServiceToSelf => -0.1,
            Ideology::Mixed => 0.0,
            Ideology::Spiritual => 0.05,
            Ideology::Material => -0.02,
        }
    }

    /// Get consciousness orientation for this ideology
    pub fn consciousness_orientation(&self) -> f64 {
        match self {
            Ideology::ServiceToOthers => 1.2,
            Ideology::ServiceToSelf => 0.8,
            Ideology::Mixed => 1.0,
            Ideology::Spiritual => 1.5,
            Ideology::Material => 0.7,
        }
    }
}

// ============================================================================
// Culture
// ============================================================================

/// Complete culture of a civilization
#[derive(Debug, Clone)]
pub struct Culture {
    /// Dominant ideology
    pub ideology: Ideology,

    /// Cultural traits
    pub traits: Vec<CulturalTrait>,

    /// Orientation towards consciousness development
    pub consciousness_orientation: f64,

    /// Drift in polarization over time
    pub polarization_drift: f64,

    /// Value on education
    pub education_value: f64,

    /// Value on technology
    pub technology_value: f64,

    /// Value on art
    pub art_value: f64,

    /// Value on spirituality
    pub spirituality_value: f64,

    /// Cultural cohesion
    pub cohesion: f64,

    /// Cultural age
    pub age: f64,
}

impl Culture {
    /// Create a new culture with random traits
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        // Random ideology
        let ideology = match rng.gen_range(0..5) {
            0 => Ideology::ServiceToOthers,
            1 => Ideology::ServiceToSelf,
            2 => Ideology::Mixed,
            3 => Ideology::Spiritual,
            _ => Ideology::Material,
        };

        Self {
            ideology,
            traits: Self::generate_random_traits(),
            consciousness_orientation: ideology.consciousness_orientation(),
            polarization_drift: ideology.polarization_drift(),
            education_value: rng.gen_range(0.3..0.9),
            technology_value: rng.gen_range(0.3..0.9),
            art_value: rng.gen_range(0.2..0.8),
            spirituality_value: rng.gen_range(0.2..0.9),
            cohesion: 0.7,
            age: 0.0,
        }
    }

    /// Create a culture with specific ideology
    pub fn with_ideology(ideology: Ideology) -> Self {
        Self {
            ideology,
            traits: Self::generate_random_traits(),
            consciousness_orientation: ideology.consciousness_orientation(),
            polarization_drift: ideology.polarization_drift(),
            education_value: 0.5,
            technology_value: 0.5,
            art_value: 0.5,
            spirituality_value: 0.5,
            cohesion: 0.7,
            age: 0.0,
        }
    }

    fn generate_random_traits() -> Vec<CulturalTrait> {
        let mut rng = rand::thread_rng();
        let mut traits = Vec::new();

        // Add some random traits
        let possible_traits = [
            ("Collectivism", TraitCategory::Social),
            ("Individualism", TraitCategory::Social),
            ("Innovation", TraitCategory::Values),
            ("Tradition", TraitCategory::Values),
            ("Mysticism", TraitCategory::Spiritual),
            ("Rationalism", TraitCategory::Values),
            ("Militarism", TraitCategory::Social),
            ("Pacifism", TraitCategory::Values),
            ("Trade focus", TraitCategory::Economic),
            ("Self-sufficiency", TraitCategory::Economic),
            ("Artistic expression", TraitCategory::Artistic),
            ("Practical minimalism", TraitCategory::Artistic),
        ];

        // Select 3-5 random traits
        let num_traits = rng.gen_range(3..=5);
        let mut indices: Vec<usize> = (0..possible_traits.len()).collect();
        for _ in 0..num_traits {
            if indices.is_empty() {
                break;
            }
            let idx = rng.gen_range(0..indices.len());
            let trait_idx = indices.remove(idx);
            let (name, category) = possible_traits[trait_idx];
            let strength = rng.gen_range(0.3..1.0);
            traits.push(CulturalTrait::new(name, strength, category));
        }

        traits
    }

    /// Evolve culture over time
    pub fn evolve(&mut self, dt: f64) {
        // Culture evolves slowly
        self.age += dt;

        // Slight drift in values
        let drift_rate = 0.00001 * dt;

        // Cohesion tends towards average
        self.cohesion = 0.7 + (self.cohesion - 0.7) * 0.999;

        // Values evolve based on ideology
        match self.ideology {
            Ideology::ServiceToOthers => {
                self.spirituality_value = (self.spirituality_value + drift_rate).min(1.0);
                self.education_value = (self.education_value + drift_rate * 0.5).min(1.0);
            }
            Ideology::ServiceToSelf => {
                self.technology_value = (self.technology_value + drift_rate).min(1.0);
                self.art_value = (self.art_value - drift_rate * 0.3).max(0.1);
            }
            Ideology::Spiritual => {
                self.spirituality_value = (self.spirituality_value + drift_rate * 2.0).min(1.0);
                self.technology_value = (self.technology_value - drift_rate * 0.2).max(0.1);
            }
            Ideology::Material => {
                self.technology_value = (self.technology_value + drift_rate).min(1.0);
                self.spirituality_value = (self.spirituality_value - drift_rate * 0.3).max(0.1);
            }
            Ideology::Mixed => {
                // Slow drift towards balance
                self.education_value = 0.5 + (self.education_value - 0.5) * 0.99;
                self.technology_value = 0.5 + (self.technology_value - 0.5) * 0.99;
            }
        }
    }

    /// Get dominant trait in a category
    pub fn dominant_trait_in(&self, category: TraitCategory) -> Option<&CulturalTrait> {
        self.traits
            .iter()
            .filter(|t| t.category == category)
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
    }
}

impl Default for Culture {
    fn default() -> Self {
        Self::random()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_culture_creation() {
        let culture = Culture::random();
        assert!(!culture.traits.is_empty());
    }

    #[test]
    fn test_ideology_polarization() {
        assert!(Ideology::ServiceToOthers.polarization_drift() > 0.0);
        assert!(Ideology::ServiceToSelf.polarization_drift() < 0.0);
        assert_eq!(Ideology::Mixed.polarization_drift(), 0.0);
    }

    #[test]
    fn test_culture_evolution() {
        let mut culture = Culture::with_ideology(Ideology::Spiritual);
        let initial_spirituality = culture.spirituality_value;

        culture.evolve(1000.0);
        assert!(culture.spirituality_value >= initial_spirituality);
    }
}
