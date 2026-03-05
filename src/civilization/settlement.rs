//! Settlement - Cities, towns, villages on planet surface
//!
//! Settlements are the physical locations where populations gather.
//! They connect to the planet environment (lithosphere, hydrosphere, atmosphere)
//! and provide the context for individual activities.

use super::CivilizationId;

// ============================================================================
// ID Types
// ============================================================================

/// Unique identifier for a settlement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub struct SettlementId(pub u64);

impl SettlementId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}


// ============================================================================
// Settlement Type
// ============================================================================

/// Type of settlement based on population
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettlementType {
    /// Small group of dwellings (< 100 people)
    #[default]
    Hamlet,

    /// Small community (100 - 1,000)
    Village,

    /// Larger community (1,000 - 10,000)
    Town,

    /// Urban center (10,000 - 1,000,000)
    City,

    /// Major urban center (1,000,000 - 10,000,000)
    Metropolis,

    /// Massive urban area (> 10,000,000)
    Megalopolis,
}

impl SettlementType {
    /// Determine type from population
    pub fn from_population(population: u64) -> Self {
        match population {
            0..=99 => SettlementType::Hamlet,
            100..=999 => SettlementType::Village,
            1_000..=9_999 => SettlementType::Town,
            10_000..=999_999 => SettlementType::City,
            1_000_000..=9_999_999 => SettlementType::Metropolis,
            _ => SettlementType::Megalopolis,
        }
    }

    /// Get name string
    pub fn name(&self) -> &'static str {
        match self {
            SettlementType::Hamlet => "Hamlet",
            SettlementType::Village => "Village",
            SettlementType::Town => "Town",
            SettlementType::City => "City",
            SettlementType::Metropolis => "Metropolis",
            SettlementType::Megalopolis => "Megalopolis",
        }
    }

    /// Get typical population range
    pub fn population_range(&self) -> (u64, u64) {
        match self {
            SettlementType::Hamlet => (1, 99),
            SettlementType::Village => (100, 999),
            SettlementType::Town => (1_000, 9_999),
            SettlementType::City => (10_000, 999_999),
            SettlementType::Metropolis => (1_000_000, 9_999_999),
            SettlementType::Megalopolis => (10_000_000, u64::MAX),
        }
    }
}

// ============================================================================
// Settlement
// ============================================================================

/// A settlement on a planet surface
#[derive(Debug, Clone)]
pub struct Settlement {
    /// Unique identifier
    pub id: SettlementId,

    /// Name of the settlement
    pub name: String,

    /// Civilization this belongs to
    pub civilization_id: CivilizationId,

    /// Type of settlement
    pub settlement_type: SettlementType,

    /// Location on planet surface (latitude, longitude)
    pub location: (f64, f64),

    /// Population count
    pub population: u64,

    /// Technology level relative to civilization
    pub tech_factor: f64,

    /// Resource production
    pub resources: ResourceProduction,

    /// Buildings and infrastructure
    pub buildings: BuildingSummary,

    /// Age in years
    pub age: f64,

    /// Growth rate
    pub growth_rate: f64,
}

impl Settlement {
    /// Create a new settlement
    pub fn new(
        id: SettlementId,
        civilization_id: CivilizationId,
        location: (f64, f64),
        population: u64,
    ) -> Self {
        Self {
            id,
            name: format!("Settlement-{}", id.0),
            civilization_id,
            settlement_type: SettlementType::from_population(population),
            location,
            population,
            tech_factor: 1.0,
            resources: ResourceProduction::default(),
            buildings: BuildingSummary::default(),
            age: 0.0,
            growth_rate: 0.01,
        }
    }

    /// Tick settlement for one time step
    pub fn tick(&mut self, dt: f64) {
        // Population growth
        self.grow_population(dt);

        // Resource production
        self.produce_resources(dt);

        // Building construction
        self.construct_buildings(dt);

        // Update settlement type if population changed
        self.settlement_type = SettlementType::from_population(self.population);

        // Age
        self.age += dt;
    }

    fn grow_population(&mut self, dt: f64) {
        let growth = (self.population as f64 * self.growth_rate * dt / 365.25) as u64;
        self.population = self.population.saturating_add(growth);
    }

    fn produce_resources(&mut self, dt: f64) {
        let production_factor = self.population as f64 * 0.001 * dt;
        self.resources.food += production_factor * 0.5;
        self.resources.materials += production_factor * 0.3;
        self.resources.energy += production_factor * 0.2;
    }

    fn construct_buildings(&mut self, dt: f64) {
        // Add buildings as population grows
        let building_rate = self.population as f64 * 0.00001 * dt;
        self.buildings.residential += (building_rate * 0.5) as u32;
        self.buildings.commercial += (building_rate * 0.3) as u32;
        self.buildings.industrial += (building_rate * 0.2) as u32;
    }

    /// Get settlement summary for rendering
    pub fn summary(&self) -> SettlementSummary {
        SettlementSummary {
            id: self.id.0,
            name: self.name.clone(),
            settlement_type: self.settlement_type.name().to_string(),
            population: self.population,
            location: self.location,
            age: self.age,
        }
    }
}

/// Resource production summary
#[derive(Debug, Clone, Default)]
pub struct ResourceProduction {
    pub food: f64,
    pub materials: f64,
    pub energy: f64,
}

/// Building summary
#[derive(Debug, Clone, Default)]
pub struct BuildingSummary {
    pub residential: u32,
    pub commercial: u32,
    pub industrial: u32,
    pub public_services: u32,
}

/// Summary for rendering
#[derive(Debug, Clone)]
pub struct SettlementSummary {
    pub id: u64,
    pub name: String,
    pub settlement_type: String,
    pub population: u64,
    pub location: (f64, f64),
    pub age: f64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_creation() {
        let id = SettlementId::new(0);
        let civ_id = CivilizationId::new(0);
        let settlement = Settlement::new(id, civ_id, (0.0, 0.0), 1000);

        assert_eq!(settlement.id, id);
        assert_eq!(settlement.settlement_type, SettlementType::Village);
        assert_eq!(settlement.population, 1000);
    }

    #[test]
    fn test_settlement_type_from_population() {
        assert_eq!(SettlementType::from_population(50), SettlementType::Hamlet);
        assert_eq!(SettlementType::from_population(500), SettlementType::Village);
        assert_eq!(SettlementType::from_population(5000), SettlementType::Town);
        assert_eq!(SettlementType::from_population(50000), SettlementType::City);
        assert_eq!(
            SettlementType::from_population(5_000_000),
            SettlementType::Metropolis
        );
        assert_eq!(
            SettlementType::from_population(50_000_000),
            SettlementType::Megalopolis
        );
    }

    #[test]
    fn test_settlement_growth() {
        let id = SettlementId::new(0);
        let civ_id = CivilizationId::new(0);
        let mut settlement = Settlement::new(id, civ_id, (0.0, 0.0), 1000);
        settlement.growth_rate = 0.1;

        settlement.tick(365.25); // One year
        assert!(settlement.population > 1000);
    }
}
