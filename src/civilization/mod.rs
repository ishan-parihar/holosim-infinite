//! Civilization Module
//!
//! This module implements the civilization layer between planets and individuals.
//! It fills the gap in the simulation hierarchy:
//!
//! SIMULATION HIERARCHY:
//! 1. Universe (CosmosEngine)
//! 2. Galaxy (CosmicWeb)
//! 3. Stellar System (StellarSystem)
//! 4. Planet (Planet with Lithosphere/Hydrosphere/Atmosphere)
//! 5. CIVILIZATION (this module) ← NEW
//! 6. Population (groups of individuals)
//! 7. Individual (consciousness entities)
//!
//! FROM COSMOLOGICAL-ARCHITECTURE.md:
//! "Civilizations emerge on habitable planets when consciousness
//! reaches sufficient density. The social memory complex is the
//! collective consciousness of the civilization."

#[allow(clippy::module_inception)]
pub mod civilization;
pub mod culture;
pub mod population;
pub mod settlement;

pub use civilization::{Civilization, CivilizationId, TechnologyLevel};
pub use culture::{Culture, CulturalTrait, Ideology};
pub use population::{Demographics, Individual, Population};
pub use settlement::{Settlement, SettlementId, SettlementType};

// Re-export SettlementSummary from settlement module
pub use settlement::SettlementSummary;

// ============================================================================
// Summary Types for Visualization
// ============================================================================

/// Civilization summary for visualization
#[derive(Debug, Clone)]
pub struct CivilizationSummary {
    pub id: u64,
    pub name: String,
    pub population: u64,
    pub tech_level: f64,
    pub polarization: f64,
}

// ============================================================================
// Civilization Manager
// ============================================================================

/// Manager for all civilizations in simulation
pub struct CivilizationManager {
    civilizations: Vec<Civilization>,
    next_id: u64,
}

impl CivilizationManager {
    pub fn new() -> Self {
        Self {
            civilizations: Vec::new(),
            next_id: 0,
        }
    }

    /// Create a new civilization on a planet
    pub fn create_civilization(&mut self, planet_id: u64) -> CivilizationId {
        let id = CivilizationId(self.next_id);
        self.next_id += 1;

        let civilization = Civilization::new(planet_id, id);
        self.civilizations.push(civilization);

        id
    }

    /// Create a seeded civilization with initial population
    pub fn create_seeded_civilization(
        &mut self,
        planet_id: u64,
        initial_population: u64,
    ) -> CivilizationId {
        let id = CivilizationId(self.next_id);
        self.next_id += 1;

        let mut civilization = Civilization::new(planet_id, id);
        
        // Create initial population
        let population = Population::new(0, initial_population);
        civilization.populations.push(population);
        civilization.total_population = initial_population;

        // Create initial settlement
        let settlement_id = SettlementId::new(0);
        let settlement = Settlement::new(settlement_id, id, (0.0, 0.0), initial_population);
        civilization.settlements.insert(settlement_id, settlement);

        self.civilizations.push(civilization);
        id
    }

    /// Get civilization by ID
    pub fn get_civilization(&self, id: CivilizationId) -> Option<&Civilization> {
        self.civilizations.iter().find(|c| c.id == id)
    }

    /// Get mutable civilization by ID
    pub fn get_civilization_mut(&mut self, id: CivilizationId) -> Option<&mut Civilization> {
        self.civilizations.iter_mut().find(|c| c.id == id)
    }

    /// Get all civilizations
    pub fn all_civilizations(&self) -> &[Civilization] {
        &self.civilizations
    }

    /// Get civilizations on a specific planet
    pub fn civilizations_on_planet(&self, planet_id: u64) -> Vec<&Civilization> {
        self.civilizations
            .iter()
            .filter(|c| c.planet_id == planet_id)
            .collect()
    }

    /// Tick all civilizations
    pub fn tick(&mut self, dt: f64) {
        for civilization in &mut self.civilizations {
            civilization.tick(dt);
        }
    }

    /// Get total population across all civilizations
    pub fn total_population(&self) -> u64 {
        self.civilizations.iter().map(|c| c.total_population).sum()
    }

    /// Get civilization count
    pub fn count(&self) -> usize {
        self.civilizations.len()
    }

    /// Get summaries of all civilizations for rendering
    pub fn get_all_summaries(&self) -> Vec<CivilizationSummary> {
        self.civilizations
            .iter()
            .map(|c| CivilizationSummary {
                id: c.id.0,
                name: c.name.clone(),
                population: c.total_population,
                tech_level: c.technology_level.level() as f64 / 10.0, // Normalize to 0-1
                polarization: c.polarization,
            })
            .collect()
    }

    /// Get all settlement summaries across all civilizations
    pub fn get_all_settlement_summaries(&self) -> Vec<SettlementSummary> {
        self.civilizations
            .iter()
            .flat_map(|c| c.settlements.values().map(|s| s.summary()))
            .collect()
    }
}

impl Default for CivilizationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civilization_manager_creation() {
        let manager = CivilizationManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_create_civilization() {
        let mut manager = CivilizationManager::new();
        let id = manager.create_civilization(0);
        assert_eq!(manager.count(), 1);
        assert!(manager.get_civilization(id).is_some());
    }

    #[test]
    fn test_seeded_civilization() {
        let mut manager = CivilizationManager::new();
        let id = manager.create_seeded_civilization(0, 1000);
        let civ = manager.get_civilization(id).unwrap();
        assert_eq!(civ.total_population, 1000);
        assert_eq!(civ.settlements.len(), 1);
    }
}
