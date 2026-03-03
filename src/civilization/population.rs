//! Population - Groups of individuals with demographics
//!
//! A population is a group of individuals sharing:
//! - Geographic location (within a settlement or region)
//! - Cultural identity
//! - Demographic characteristics
//!
//! Individuals within a population are consciousness entities
//! (SubSubLogos) with physical bodies and daily activities.

use super::SettlementId;

// ============================================================================
// Population
// ============================================================================

/// A population group within a civilization
#[derive(Debug, Clone)]
pub struct Population {
    /// Unique identifier within civilization
    pub id: u64,

    /// Number of individuals
    pub size: u64,

    /// Age distribution (10 buckets: 0-9, 10-19, ..., 90+)
    pub age_distribution: [f64; 10],

    /// Consciousness distribution (per density level 1-8)
    pub consciousness_distribution: [f64; 8],

    /// Polarization distribution (21 buckets: -1.0 to +1.0 in 0.1 steps)
    pub polarization_distribution: [f64; 21],

    /// Average health of population
    pub average_health: f64,

    /// Education level (0-1)
    pub education_level: f64,

    /// Settlement this population belongs to
    pub settlement_id: Option<SettlementId>,

    /// Birth rate
    pub birth_rate: f64,

    /// Death rate
    pub death_rate: f64,
}

impl Population {
    /// Create a new population
    pub fn new(id: u64, size: u64) -> Self {
        // Initialize with realistic age distribution
        let age_distribution = [
            0.12, // 0-9
            0.12, // 10-19
            0.14, // 20-29
            0.14, // 30-39
            0.12, // 40-49
            0.10, // 50-59
            0.08, // 60-69
            0.06, // 70-79
            0.04, // 80-89
            0.02, // 90+
        ];

        Self {
            id,
            size,
            age_distribution,
            consciousness_distribution: [0.125; 8], // Evenly distributed initially
            polarization_distribution: [0.0; 21],   // Centered at 0
            average_health: 0.8,
            education_level: 0.3,
            settlement_id: None,
            birth_rate: 0.02,
            death_rate: 0.01,
        }
    }

    /// Tick population for one time step
    pub fn tick(&mut self, dt: f64) {
        // Birth and death rates
        self.process_demographics(dt);

        // Age advancement
        self.advance_ages(dt);

        // Consciousness development
        self.evolve_consciousness(dt);

        // Education improvement
        self.evolve_education(dt);
    }

    fn process_demographics(&mut self, dt: f64) {
        // Calculate births and deaths
        let births = ((self.size as f64) * self.birth_rate * dt / 365.25) as u64;
        let deaths = ((self.size as f64) * self.death_rate * dt / 365.25) as u64;

        self.size = self.size.saturating_add(births).saturating_sub(deaths);
    }

    fn advance_ages(&mut self, dt: f64) {
        // Shift age distribution over time (simplified)
        // In a full implementation, this would track cohorts
        let _ = dt; // Suppress unused warning
    }

    fn evolve_consciousness(&mut self, dt: f64) {
        // Gradual consciousness evolution
        for i in 0..8 {
            self.consciousness_distribution[i] += 0.00001 * dt;
            // Normalize
            self.consciousness_distribution[i] =
                self.consciousness_distribution[i].min(1.0);
        }
    }

    fn evolve_education(&mut self, dt: f64) {
        // Education improves over time
        let improvement = 0.0001 * dt;
        self.education_level = (self.education_level + improvement).min(1.0);
    }

    /// Get average consciousness
    pub fn average_consciousness(&self) -> f64 {
        let total: f64 = self.consciousness_distribution.iter().sum();
        total / 8.0
    }

    /// Get average polarization
    pub fn average_polarization(&self) -> f64 {
        let mut weighted_sum = 0.0;
        for (i, &count) in self.polarization_distribution.iter().enumerate() {
            let polarization = -1.0 + (i as f64) * 0.1;
            weighted_sum += polarization * count;
        }
        let total: f64 = self.polarization_distribution.iter().sum();
        if total > 0.0 {
            weighted_sum / total
        } else {
            0.0
        }
    }
}

// ============================================================================
// Individual
// ============================================================================

/// An individual person within a civilization
#[derive(Debug, Clone)]
pub struct Individual {
    /// Unique identifier
    pub id: u64,

    /// Age in years
    pub age: f64,

    /// Gender
    pub gender: Gender,

    /// Current occupation
    pub occupation: Occupation,

    /// Location on planet surface (lat, lon)
    pub location: (f64, f64),

    /// Current activity
    pub activity: Activity,

    /// Personal consciousness level
    pub consciousness_level: f64,

    /// Personal polarization
    pub polarization: f64,

    /// Physical body state
    pub body: PhysicalBody,
}

impl Individual {
    /// Create a new individual
    pub fn new(id: u64) -> Self {
        Self {
            id,
            body: PhysicalBody::default(),
            age: 0.0,
            gender: Gender::Other,
            occupation: Occupation::Child,
            location: (0.0, 0.0),
            activity: Activity::Sleeping,
            consciousness_level: 0.3,
            polarization: 0.0,
        }
    }

    /// Create an individual with specific age
    pub fn with_age(id: u64, age: f64) -> Self {
        let mut individual = Self::new(id);
        individual.age = age;
        individual.occupation = if age < 5.0 {
            Occupation::Child
        } else if age < 18.0 {
            Occupation::Student
        } else {
            Occupation::Farmer // Default adult occupation
        };
        individual
    }

    /// Tick individual for one time step
    pub fn tick(&mut self, dt: f64) {
        // Update body state
        self.body.tick(dt);

        // Age
        self.age += dt / 365.25; // Convert days to years

        // Update occupation based on age
        self.update_occupation();

        // Consciousness evolution
        self.evolve_consciousness(dt);
    }

    fn update_occupation(&mut self) {
        self.occupation = match self.age {
            a if a < 5.0 => Occupation::Child,
            a if a < 18.0 => Occupation::Student,
            a if a < 65.0 => self.occupation.clone(), // Keep current occupation
            _ => Occupation::Retired,
        };
    }

    fn evolve_consciousness(&mut self, dt: f64) {
        // Gradual consciousness development
        let evolution = 0.00001 * dt;
        self.consciousness_level = (self.consciousness_level + evolution).min(1.0);
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Physical body state
#[derive(Debug, Clone)]
pub struct PhysicalBody {
    /// Health (0-1)
    pub health: f64,

    /// Energy (0-1)
    pub energy: f64,

    /// Nutrition (0-1)
    pub nutrition: f64,

    /// Rest (0-1)
    pub rest: f64,

    /// Age-related decline
    pub age_factor: f64,
}

impl Default for PhysicalBody {
    fn default() -> Self {
        Self {
            health: 1.0,
            energy: 1.0,
            nutrition: 0.8,
            rest: 1.0,
            age_factor: 1.0,
        }
    }
}

impl PhysicalBody {
    /// Tick body state
    pub fn tick(&mut self, dt: f64) {
        // Energy depletes over time
        self.energy = (self.energy - 0.001 * dt).max(0.0);

        // Nutrition depletes
        self.nutrition = (self.nutrition - 0.0005 * dt).max(0.0);

        // Health changes based on factors
        if self.nutrition < 0.3 {
            self.health = (self.health - 0.001 * dt).max(0.0);
        }

        // Age factor decline
        self.age_factor = (self.age_factor - 0.000001 * dt).max(0.0);
    }
}

/// Gender for biological simulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

/// Occupation types
#[derive(Debug, Clone, PartialEq)]
pub enum Occupation {
    Child,
    Student,
    Farmer,
    Artisan,
    Merchant,
    Scholar,
    Leader,
    Healer,
    Artist,
    Warrior,
    Mystic,
    Retired,
}

/// Current activity
#[derive(Debug, Clone, PartialEq)]
pub enum Activity {
    Sleeping,
    Eating,
    Working,
    Learning,
    Socializing,
    Meditating,
    Traveling,
    Resting,
    /// Engaged with catalyst event
    CatalystExperience,
}

// ============================================================================
// Demographics
// ============================================================================

/// Demographics summary
#[derive(Debug, Clone, Default)]
pub struct Demographics {
    pub total_population: u64,
    pub birth_rate: f64,
    pub death_rate: f64,
    pub median_age: f64,
    pub life_expectancy: f64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_creation() {
        let pop = Population::new(0, 1000);
        assert_eq!(pop.size, 1000);
        assert_eq!(pop.id, 0);
    }

    #[test]
    fn test_population_tick() {
        let mut pop = Population::new(0, 1000);
        pop.tick(365.25); // One year
        // Size changes based on birth/death rates
        assert!(pop.size > 900 && pop.size < 1100);
    }

    #[test]
    fn test_individual_creation() {
        let ind = Individual::new(0);
        assert_eq!(ind.id, 0);
        assert_eq!(ind.occupation, Occupation::Child);
    }

    #[test]
    fn test_individual_aging() {
        let mut ind = Individual::with_age(0, 5.0);
        ind.tick(365.25 * 13.0); // Age 13 years
        assert_eq!(ind.occupation, Occupation::Student);
    }
}
