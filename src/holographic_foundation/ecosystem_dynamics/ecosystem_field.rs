//! Ecosystem Field - Unified Ecosystem Field
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Ecosystems emerge as FIELD RESONANCE NETWORKS.
//!  Ecosystem field influencing entity evolution"
//!
//! # Key Insight
//!
//! An ecosystem is a unified field configuration containing:
//! - All species as field patterns
//! - All trophic relationships as field couplings
//! - All spatial structure as field geometry
//! - Ecosystem consciousness emerging from field coherence

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::ecosystem_dynamics::coevolution::CoevolutionSystem;
use crate::holographic_foundation::ecosystem_dynamics::population_dynamics::{Population, PopulationId};
use crate::holographic_foundation::ecosystem_dynamics::spatial_ecosystem::SpatialEcosystem;
use crate::holographic_foundation::ecosystem_dynamics::species_field::{Species, SpeciesId};
use crate::holographic_foundation::ecosystem_dynamics::trophic_coupling::TrophicNetwork;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EcosystemId(pub u64);

impl EcosystemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcosystemState {
    Emerging,
    Mature,
    Stressed,
    Disturbed,
    Recovering,
    Collapsed,
}

impl EcosystemState {
    pub fn from_health(health: Float) -> Self {
        if health < 0.2 {
            EcosystemState::Collapsed
        } else if health < 0.4 {
            EcosystemState::Disturbed
        } else if health < 0.6 {
            EcosystemState::Stressed
        } else if health < 0.8 {
            EcosystemState::Recovering
        } else if health < 0.95 {
            EcosystemState::Mature
        } else {
            EcosystemState::Emerging
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            EcosystemState::Emerging => "New ecosystem forming",
            EcosystemState::Mature => "Stable, diverse ecosystem",
            EcosystemState::Stressed => "Under moderate pressure",
            EcosystemState::Disturbed => "Major disruption occurring",
            EcosystemState::Recovering => "Rebuilding after disturbance",
            EcosystemState::Collapsed => "Ecosystem function lost",
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnergyBudget {
    pub total_input: Float,
    pub primary_production: Float,
    pub respiration: Float,
    pub export: Float,
    pub storage: Float,
    pub efficiency: Float,
}

impl EnergyBudget {
    pub fn new() -> Self {
        Self {
            total_input: 1000.0,
            primary_production: 500.0,
            respiration: 300.0,
            export: 50.0,
            storage: 150.0,
            efficiency: 0.5,
        }
    }

    pub fn net_production(&self) -> Float {
        self.primary_production - self.respiration
    }

    pub fn update(&mut self, dt: Float, production_factor: Float, respiration_factor: Float) {
        self.primary_production = self.total_input * self.efficiency * production_factor;
        self.respiration = self.primary_production * respiration_factor * 0.6;
        self.storage = (self.storage + self.net_production() * dt * 0.1).max(0.0);
        self.export = self.primary_production * 0.1;
    }

    pub fn energy_available_at_level(&self, level: usize) -> Float {
        let mut available = self.primary_production;
        for _l in 0..level {
            available *= 0.1;
        }
        available
    }
}

impl Default for EnergyBudget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EcosystemHealth {
    pub species_richness: Float,
    pub functional_diversity: Float,
    pub trophic_completeness: Float,
    pub spatial_integrity: Float,
    pub resilience: Float,
    pub redundancy: Float,
}

impl EcosystemHealth {
    pub fn new() -> Self {
        Self {
            species_richness: 0.8,
            functional_diversity: 0.75,
            trophic_completeness: 0.7,
            spatial_integrity: 0.8,
            resilience: 0.6,
            redundancy: 0.5,
        }
    }

    pub fn overall_health(&self) -> Float {
        (self.species_richness
            + self.functional_diversity
            + self.trophic_completeness
            + self.spatial_integrity
            + self.resilience
            + self.redundancy)
            / 6.0
    }

    pub fn vulnerability(&self) -> Float {
        1.0 - self.resilience
    }

    pub fn update_from_metrics(
        &mut self,
        richness: usize,
        functional_groups: usize,
        trophic_levels: usize,
        fragmentation: Float,
    ) {
        self.species_richness = (richness as Float / 100.0).min(1.0);
        self.functional_diversity = (functional_groups as Float / 20.0).min(1.0);
        self.trophic_completeness = (trophic_levels as Float / 5.0).min(1.0);
        self.spatial_integrity = 1.0 - fragmentation;

        self.resilience = (self.species_richness * 0.3
            + self.functional_diversity * 0.3
            + self.spatial_integrity * 0.2
            + self.redundancy * 0.2)
            .clamp(0.0, 1.0);
    }
}

impl Default for EcosystemHealth {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EcosystemField {
    pub id: EcosystemId,
    pub state: EcosystemState,
    pub age: Float,
    pub field_pattern: [Float; NUM_ARCHETYPES],
    pub field_coherence: Float,
    pub resonance_frequency: Float,
    pub species: HashMap<SpeciesId, Species>,
    pub populations: HashMap<PopulationId, Population>,
    pub trophic_network: TrophicNetwork,
    pub spatial_structure: SpatialEcosystem,
    pub coevolution: CoevolutionSystem,
    pub energy_budget: EnergyBudget,
    pub health: EcosystemHealth,
    pub consciousness_level: Float,
}

impl EcosystemField {
    pub fn new() -> Self {
        Self {
            id: EcosystemId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            state: EcosystemState::Emerging,
            age: 0.0,
            field_pattern: [0.5; NUM_ARCHETYPES],
            field_coherence: 0.8,
            resonance_frequency: 1.0,
            species: HashMap::new(),
            populations: HashMap::new(),
            trophic_network: TrophicNetwork::new(),
            spatial_structure: SpatialEcosystem::new(),
            coevolution: CoevolutionSystem::new(),
            energy_budget: EnergyBudget::new(),
            health: EcosystemHealth::new(),
            consciousness_level: 0.5,
        }
    }

    pub fn add_species(&mut self, species: Species) -> SpeciesId {
        let id = species.id;
        self.update_field_pattern_with_species(&species);
        self.species.insert(id, species);
        id
    }

    pub fn add_population(&mut self, population: Population) -> PopulationId {
        let id = population.id;
        self.populations.insert(id, population);
        id
    }

    fn update_field_pattern_with_species(&mut self, species: &Species) {
        let pattern = species.field_pattern.archetype_pattern();
        let weight = species.trophic_level as Float + 1.0;

        for (field_i, &pattern_i) in self.field_pattern.iter_mut().zip(pattern.iter()) {
            *field_i = (*field_i * self.age + pattern_i * weight) / (self.age + weight);
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.age += dt;

        let species_ids: Vec<SpeciesId> = self.populations.values().map(|p| p.species_id).collect();
        let mut competitor_counts: HashMap<SpeciesId, usize> = HashMap::new();
        let mut predator_counts: HashMap<SpeciesId, usize> = HashMap::new();

        for species_id in &species_ids {
            competitor_counts.insert(*species_id, self.competitor_count(species_id));
            predator_counts.insert(*species_id, self.predator_count(species_id));
        }

        let resources = self.energy_budget.energy_available_at_level(0);

        for population in self.populations.values_mut() {
            let competitors = competitor_counts.get(&population.species_id).unwrap_or(&0);
            let predators = predator_counts.get(&population.species_id).unwrap_or(&0);

            population.update(dt, resources, *competitors as Float, *predators as Float);
        }

        self.trophic_network.update(dt);
        self.spatial_structure.update(dt);
        self.coevolution.update(dt);

        self.energy_budget
            .update(dt, self.field_coherence, 1.0 - self.field_coherence);

        self.update_health();

        self.state = EcosystemState::from_health(self.health.overall_health());

        self.field_coherence = self.calculate_field_coherence();
        self.consciousness_level = self.calculate_ecosystem_consciousness();
    }

    fn competitor_count(&self, species_id: &SpeciesId) -> usize {
        self.populations
            .values()
            .filter(|p| {
                p.species_id != *species_id
                    && self.species.get(&p.species_id).is_some_and(|s| {
                        self.species
                            .get(species_id)
                            .is_some_and(|target| s.trophic_level == target.trophic_level)
                    })
            })
            .count()
    }

    fn predator_count(&self, species_id: &SpeciesId) -> usize {
        let target_level = self
            .species
            .get(species_id)
            .map(|s| s.trophic_level)
            .unwrap_or(0);

        self.populations
            .values()
            .filter(|p| {
                self.species
                    .get(&p.species_id)
                    .is_some_and(|s| s.trophic_level > target_level)
            })
            .count()
    }

    fn update_health(&mut self) {
        let richness = self.species.len();
        let functional_groups = self.count_functional_groups();
        let trophic_levels = self.count_trophic_levels();
        let fragmentation = self.spatial_structure.fragmentation_index;

        self.health
            .update_from_metrics(richness, functional_groups, trophic_levels, fragmentation);

        self.health.redundancy = self.calculate_redundancy();
    }

    fn count_functional_groups(&self) -> usize {
        self.species
            .values()
            .map(|s| s.species_type as usize)
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    fn count_trophic_levels(&self) -> usize {
        self.species
            .values()
            .map(|s| s.trophic_level)
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    fn calculate_redundancy(&self) -> Float {
        if self.species.is_empty() {
            return 0.0;
        }

        let mut trophic_counts: HashMap<usize, usize> = HashMap::new();
        for species in self.species.values() {
            *trophic_counts.entry(species.trophic_level).or_insert(0) += 1;
        }

        let avg_per_level: Float =
            trophic_counts.values().sum::<usize>() as Float / trophic_counts.len().max(1) as Float;

        (avg_per_level / 3.0).min(1.0)
    }

    fn calculate_field_coherence(&self) -> Float {
        if self.populations.is_empty() {
            return self.field_coherence;
        }

        let population_coherence: Float =
            self.populations.values().map(|p| p.fitness).sum::<Float>()
                / self.populations.len() as Float;

        let network_coherence = self.trophic_network.network_coherence;
        let spatial_coherence = self.spatial_structure.landscape_coherence;

        (population_coherence * 0.4 + network_coherence * 0.3 + spatial_coherence * 0.3)
            .clamp(0.0, 1.0)
    }

    fn calculate_ecosystem_consciousness(&self) -> Float {
        let base = self.field_coherence * self.health.overall_health();

        let diversity_factor = self.health.functional_diversity * 0.5 + 0.5;

        let resilience_factor = self.health.resilience * 0.3 + 0.7;

        base * diversity_factor * resilience_factor
    }

    pub fn species_count(&self) -> usize {
        self.species.len()
    }

    pub fn population_count(&self) -> usize {
        self.populations.len()
    }

    pub fn total_biomass(&self) -> Float {
        self.populations.values().map(|p| p.size).sum()
    }

    pub fn biodiversity_index(&self) -> Float {
        if self.populations.is_empty() {
            return 0.0;
        }

        let total: Float = self.populations.values().map(|p| p.size).sum();
        if total == 0.0 {
            return 0.0;
        }

        let mut shannon: Float = 0.0;
        for population in self.populations.values() {
            let p = population.size / total;
            if p > 0.0 {
                shannon -= p * p.ln();
            }
        }

        let max_shannon = (self.populations.len() as Float).ln();
        if max_shannon > 0.0 {
            shannon / max_shannon
        } else {
            0.0
        }
    }

    pub fn apply_disturbance(&mut self, intensity: Float, affected_level: Option<usize>) {
        match affected_level {
            Some(level) => {
                for population in self.populations.values_mut() {
                    if let Some(species) = self.species.get(&population.species_id) {
                        if species.trophic_level == level {
                            population.size *= 1.0 - intensity;
                        }
                    }
                }
            }
            None => {
                for population in self.populations.values_mut() {
                    population.size *= 1.0 - intensity * 0.5;
                }
            }
        }

        self.field_coherence *= 1.0 - intensity * 0.3;
    }

    pub fn resilience_score(&self) -> Float {
        self.health.resilience * self.field_coherence
    }

    pub fn is_stable(&self) -> bool {
        matches!(
            self.state,
            EcosystemState::Mature | EcosystemState::Emerging
        ) && self.resilience_score() > 0.5
    }
}

impl Default for EcosystemField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_id_creation() {
        let id = EcosystemId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_ecosystem_state_from_health() {
        assert_eq!(EcosystemState::from_health(0.99), EcosystemState::Emerging);
        assert_eq!(EcosystemState::from_health(0.85), EcosystemState::Mature);
        assert_eq!(EcosystemState::from_health(0.5), EcosystemState::Stressed);
        assert_eq!(EcosystemState::from_health(0.3), EcosystemState::Disturbed);
        assert_eq!(EcosystemState::from_health(0.1), EcosystemState::Collapsed);
    }

    #[test]
    fn test_energy_budget_creation() {
        let budget = EnergyBudget::new();
        assert!(budget.total_input > 0.0);
        assert!(budget.net_production() > 0.0);
    }

    #[test]
    fn test_energy_budget_level_availability() {
        let budget = EnergyBudget::new();
        let level0 = budget.energy_available_at_level(0);
        let level1 = budget.energy_available_at_level(1);
        assert!(level0 > level1);
    }

    #[test]
    fn test_ecosystem_health_creation() {
        let health = EcosystemHealth::new();
        assert!(health.overall_health() > 0.0);
    }

    #[test]
    fn test_ecosystem_health_vulnerability() {
        let health = EcosystemHealth::new();
        assert!(health.vulnerability() >= 0.0);
    }

    #[test]
    fn test_ecosystem_field_creation() {
        let ecosystem = EcosystemField::new();
        assert!(ecosystem.species.is_empty());
        assert!(ecosystem.populations.is_empty());
    }

    #[test]
    fn test_ecosystem_field_add_species() {
        let mut ecosystem = EcosystemField::new();
        let species = Species::new(
            "Test".to_string(),
            crate::holographic_foundation::ecosystem_dynamics::species_field::SpeciesType::Producer,
        );
        let id = ecosystem.add_species(species);

        assert_eq!(ecosystem.species_count(), 1);
        assert!(ecosystem.species.contains_key(&id));
    }

    #[test]
    fn test_ecosystem_field_add_population() {
        let mut ecosystem = EcosystemField::new();
        let population = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        let _id = ecosystem.add_population(population);

        assert_eq!(ecosystem.population_count(), 1);
    }

    #[test]
    fn test_ecosystem_field_update() {
        let mut ecosystem = EcosystemField::new();
        ecosystem.update(1.0);

        assert!(ecosystem.age > 0.0);
    }

    #[test]
    fn test_ecosystem_field_total_biomass() {
        let mut ecosystem = EcosystemField::new();
        let pop1 = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        let pop2 = Population::new(SpeciesId::new(2), 50.0, 500.0);
        ecosystem.add_population(pop1);
        ecosystem.add_population(pop2);

        assert_eq!(ecosystem.total_biomass(), 150.0);
    }

    #[test]
    fn test_ecosystem_field_apply_disturbance() {
        let mut ecosystem = EcosystemField::new();
        let population = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        ecosystem.add_population(population);

        let initial_biomass = ecosystem.total_biomass();
        ecosystem.apply_disturbance(0.5, None);

        assert!(ecosystem.total_biomass() < initial_biomass);
    }

    #[test]
    fn test_ecosystem_field_resilience() {
        let ecosystem = EcosystemField::new();
        assert!(ecosystem.resilience_score() >= 0.0);
    }

    #[test]
    fn test_ecosystem_biodiversity_index() {
        let mut ecosystem = EcosystemField::new();
        let pop1 = Population::new(SpeciesId::new(1), 50.0, 1000.0);
        let pop2 = Population::new(SpeciesId::new(2), 50.0, 1000.0);
        ecosystem.add_population(pop1);
        ecosystem.add_population(pop2);

        let biodiversity = ecosystem.biodiversity_index();
        assert!(biodiversity > 0.0 && biodiversity <= 1.0);
    }
}
