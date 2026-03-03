//! Civilization - Large-scale societal structures
//!
//! A civilization is a collection of populations, settlements, and culture
//! existing on a habitable planet. Civilizations evolve over time based on:
//! - Environmental conditions (planet state)
//! - Technology progression
//! - Cultural development
//! - Consciousness evolution
//!
//! FROM COSMOLOGICAL-ARCHITECTURE.md:
//! "Third-density civilizations are the training ground for
//! social memory complex formation. The polarization of the
//! civilization determines its harvestability."

use super::{Culture, Population, Settlement, SettlementId};
use std::collections::HashMap;

// ============================================================================
// ID Types
// ============================================================================

/// Unique identifier for a civilization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CivilizationId(pub u64);

impl CivilizationId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for CivilizationId {
    fn default() -> Self {
        Self(0)
    }
}

// ============================================================================
// Technology Level
// ============================================================================

/// Technology level on a modified Kardashev scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechnologyLevel {
    /// Pre-agricultural hunter-gatherer
    PreAgricultural = 0,

    /// Agricultural society
    Agricultural = 1,

    /// Industrial society
    Industrial = 2,

    /// Atomic/Digital society
    Atomic = 3,

    /// Information age
    Digital = 4,

    /// Fusion power
    Fusion = 5,

    /// Stellar civilization
    Stellar = 6,

    /// Galactic civilization
    Galactic = 7,

    /// Intergalactic civilization
    Intergalactic = 8,

    /// Dimensional access
    Dimensional = 9,

    /// Infinite intelligence
    Infinite = 10,
}

impl Default for TechnologyLevel {
    fn default() -> Self {
        TechnologyLevel::PreAgricultural
    }
}

impl TechnologyLevel {
    /// Convert to numeric value for comparison
    pub fn level(&self) -> u8 {
        *self as u8
    }

    /// Get technology level from numeric value
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => TechnologyLevel::PreAgricultural,
            1 => TechnologyLevel::Agricultural,
            2 => TechnologyLevel::Industrial,
            3 => TechnologyLevel::Atomic,
            4 => TechnologyLevel::Digital,
            5 => TechnologyLevel::Fusion,
            6 => TechnologyLevel::Stellar,
            7 => TechnologyLevel::Galactic,
            8 => TechnologyLevel::Intergalactic,
            9 => TechnologyLevel::Dimensional,
            10 | _ => TechnologyLevel::Infinite,
        }
    }

    /// Get name string
    pub fn name(&self) -> &'static str {
        match self {
            TechnologyLevel::PreAgricultural => "Pre-Agricultural",
            TechnologyLevel::Agricultural => "Agricultural",
            TechnologyLevel::Industrial => "Industrial",
            TechnologyLevel::Atomic => "Atomic",
            TechnologyLevel::Digital => "Digital",
            TechnologyLevel::Fusion => "Fusion",
            TechnologyLevel::Stellar => "Stellar",
            TechnologyLevel::Galactic => "Galactic",
            TechnologyLevel::Intergalactic => "Intergalactic",
            TechnologyLevel::Dimensional => "Dimensional",
            TechnologyLevel::Infinite => "Infinite",
        }
    }
}

// ============================================================================
// Civilization
// ============================================================================

/// A civilization on a planet
#[derive(Debug, Clone)]
pub struct Civilization {
    /// Unique identifier
    pub id: CivilizationId,

    /// Name of the civilization
    pub name: String,

    /// Planet this civilization exists on
    pub planet_id: u64,

    /// Population groups
    pub populations: Vec<Population>,

    /// Settlements (cities, towns, villages)
    pub settlements: HashMap<SettlementId, Settlement>,

    /// Cultural traits
    pub culture: Culture,

    /// Technology level
    pub technology_level: TechnologyLevel,

    /// Years since civilization began
    pub age: f64,

    /// Service-to-Others vs Service-to-Self polarization
    /// -1.0 = purely STS, 0.0 = mixed, +1.0 = purely STO
    pub polarization: f64,

    /// Average consciousness level of population
    pub average_consciousness: f64,

    /// Total population count
    pub total_population: u64,

    /// Is this civilization harvested?
    pub is_harvested: bool,

    /// Harvest readiness (consciousness and polarization criteria met)
    pub harvest_readiness: f64,

    /// Technology progress (0-1 towards next level)
    pub technology_progress: f64,
}

impl Civilization {
    /// Create a new civilization on a planet
    pub fn new(planet_id: u64, id: CivilizationId) -> Self {
        Self {
            id,
            name: format!("Civilization-{}", id.0),
            planet_id,
            populations: Vec::new(),
            settlements: HashMap::new(),
            culture: Culture::random(),
            technology_level: TechnologyLevel::PreAgricultural,
            age: 0.0,
            polarization: 0.0,
            average_consciousness: 0.3,
            total_population: 0,
            is_harvested: false,
            harvest_readiness: 0.0,
            technology_progress: 0.0,
        }
    }

    /// Evolve civilization one tick
    pub fn tick(&mut self, dt: f64) {
        if self.is_harvested {
            return;
        }

        // Update populations
        for population in &mut self.populations {
            population.tick(dt);
        }

        // Update settlements
        for settlement in self.settlements.values_mut() {
            settlement.tick(dt);
        }

        // Technology advancement
        self.advance_technology(dt);

        // Cultural evolution
        self.culture.evolve(dt);

        // Consciousness development
        self.evolve_consciousness(dt);

        // Polarization dynamics
        self.evolve_polarization(dt);

        // Check harvest readiness
        self.check_harvest_readiness();

        // Update total population
        self.total_population = self.populations.iter().map(|p| p.size).sum();

        self.age += dt;
    }

    /// Advance technology based on conditions
    fn advance_technology(&mut self, dt: f64) {
        // Technology advancement rate depends on:
        // - Population size (more minds = more innovation)
        // - Education level
        // - Stability

        let population_factor = if self.total_population > 0 {
            (self.total_population as f64).ln() / 20.0
        } else {
            0.0
        };
        let education_factor = self.average_education();
        let stability_factor = self.calculate_stability();

        let advancement_rate = population_factor * education_factor * stability_factor * 0.001;

        // Accumulate progress
        self.technology_progress += advancement_rate * dt;

        // Check for breakthrough
        if self.technology_progress >= 1.0 {
            let current_level = self.technology_level.level();
            if current_level < 10 {
                self.technology_level = TechnologyLevel::from_level(current_level + 1);
                self.technology_progress = 0.0;
            }
        }
    }

    /// Evolve consciousness of civilization
    fn evolve_consciousness(&mut self, dt: f64) {
        // Consciousness evolution depends on:
        // - Catalyst events
        // - Cultural values
        // - Technology level (can help or hinder)

        let base_rate = 0.0001;
        let culture_factor = self.culture.consciousness_orientation;
        let tech_factor = if self.technology_level.level() >= 4 {
            1.1 // Digital age helps
        } else if self.technology_level.level() >= 2 {
            0.9 // Industrial age hinders
        } else {
            1.0 // Neutral
        };

        let evolution = base_rate * culture_factor * tech_factor * dt;
        self.average_consciousness = (self.average_consciousness + evolution).min(1.0);
    }

    /// Evolve polarization of civilization
    fn evolve_polarization(&mut self, dt: f64) {
        // Polarization drifts based on cultural values
        let drift = self.culture.polarization_drift * dt * 0.0001;
        self.polarization = (self.polarization + drift).clamp(-1.0, 1.0);
    }

    /// Check if civilization is ready for harvest
    fn check_harvest_readiness(&mut self) {
        // Harvest criteria (from cosmological architecture):
        // 1. Consciousness >= 0.5 (ready for fourth density)
        // 2. Polarization >= 0.75 (STO) or <= -0.75 (STS)
        // 3. Civilization age >= 75000 years (typical cycle length)

        let consciousness_ready = self.average_consciousness >= 0.5;
        let polarization_ready = self.polarization.abs() >= 0.75;
        let age_ready = self.age >= 75000.0;

        // Calculate readiness score
        self.harvest_readiness = [
            if consciousness_ready {
                1.0
            } else {
                self.average_consciousness / 0.5
            },
            if polarization_ready {
                1.0
            } else {
                self.polarization.abs() / 0.75
            },
            if age_ready { 1.0 } else { self.age / 75000.0 },
        ]
        .iter()
        .sum::<f64>()
            / 3.0;

        // Check if harvested
        if consciousness_ready && polarization_ready {
            self.is_harvested = true;
        }
    }

    /// Calculate average education level
    fn average_education(&self) -> f64 {
        if self.populations.is_empty() {
            return 0.0;
        }

        self.populations
            .iter()
            .map(|p| p.education_level)
            .sum::<f64>()
            / self.populations.len() as f64
    }

    /// Calculate stability of civilization
    fn calculate_stability(&self) -> f64 {
        // Stability based on polarization balance
        0.7 + 0.3 * (1.0 - self.polarization.abs())
    }

    /// Add a new settlement
    pub fn add_settlement(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id, settlement);
    }

    /// Get settlement by ID
    pub fn get_settlement(&self, id: SettlementId) -> Option<&Settlement> {
        self.settlements.get(&id)
    }

    /// Get all settlements
    pub fn all_settlements(&self) -> Vec<&Settlement> {
        self.settlements.values().collect()
    }

    /// Get summary for rendering
    pub fn summary(&self) -> CivilizationSummary {
        CivilizationSummary {
            id: self.id.0,
            name: self.name.clone(),
            population: self.total_population,
            tech_level: self.technology_level.level() as f64,
            polarization: self.polarization,
            consciousness: self.average_consciousness,
            settlement_count: self.settlements.len() as u64,
            age: self.age,
            harvested: self.is_harvested,
        }
    }
}

/// Summary of civilization for rendering
#[derive(Debug, Clone)]
pub struct CivilizationSummary {
    pub id: u64,
    pub name: String,
    pub population: u64,
    pub tech_level: f64,
    pub polarization: f64,
    pub consciousness: f64,
    pub settlement_count: u64,
    pub age: f64,
    pub harvested: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civilization_creation() {
        let id = CivilizationId::new(0);
        let civ = Civilization::new(0, id);

        assert_eq!(civ.id, id);
        assert_eq!(civ.technology_level, TechnologyLevel::PreAgricultural);
        assert_eq!(civ.total_population, 0);
    }

    #[test]
    fn test_technology_levels() {
        assert_eq!(TechnologyLevel::PreAgricultural.level(), 0);
        assert_eq!(TechnologyLevel::Digital.level(), 4);
        assert_eq!(TechnologyLevel::Infinite.level(), 10);
    }

    #[test]
    fn test_civilization_tick() {
        let id = CivilizationId::new(0);
        let mut civ = Civilization::new(0, id);
        
        // Add a population
        civ.populations.push(Population::new(0, 100));
        civ.total_population = 100;

        civ.tick(1.0);
        assert!(civ.age > 0.0);
    }

    #[test]
    fn test_harvest_readiness() {
        let id = CivilizationId::new(0);
        let mut civ = Civilization::new(0, id);

        // Set conditions for harvest
        civ.average_consciousness = 0.6;
        civ.polarization = 0.8;
        civ.age = 80000.0;

        civ.check_harvest_readiness();
        assert!(civ.harvest_readiness > 0.9);
    }
}
