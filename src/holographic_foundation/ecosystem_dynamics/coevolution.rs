//! Co-evolution Through Field Co-adaptation
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Co-evolution through field co-adaptation
//!  Ecosystem field influencing entity evolution"
//!
//! # Key Insight
//!
//! Co-evolution occurs when species field patterns influence each other:
//! - Adaptation = field pattern adjustment
//! - Co-adaptation = mutual field pattern adjustment
//! - Arms race = reciprocal field pattern escalation
//! - Mutualism = field pattern alignment

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::ecosystem_dynamics::species_field::SpeciesId;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoevolutionId(pub u64);

impl CoevolutionId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoevolutionRelationship {
    PredatorPrey,
    ParasiteHost,
    Mutualist,
    Competitor,
    Mimicry,
    Pollinator,
    Disperser,
}

impl CoevolutionRelationship {
    pub fn directionality(&self) -> (bool, bool) {
        match self {
            CoevolutionRelationship::PredatorPrey => (true, true),
            CoevolutionRelationship::ParasiteHost => (true, true),
            CoevolutionRelationship::Mutualist => (true, true),
            CoevolutionRelationship::Competitor => (true, true),
            CoevolutionRelationship::Mimicry => (true, false),
            CoevolutionRelationship::Pollinator => (true, true),
            CoevolutionRelationship::Disperser => (true, true),
        }
    }

    pub fn adaptation_rate(&self) -> Float {
        match self {
            CoevolutionRelationship::PredatorPrey => 0.1,
            CoevolutionRelationship::ParasiteHost => 0.15,
            CoevolutionRelationship::Mutualist => 0.05,
            CoevolutionRelationship::Competitor => 0.08,
            CoevolutionRelationship::Mimicry => 0.12,
            CoevolutionRelationship::Pollinator => 0.03,
            CoevolutionRelationship::Disperser => 0.02,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CoevolutionRelationship::PredatorPrey => "Arms race between predator and prey",
            CoevolutionRelationship::ParasiteHost => "Parasite virulence vs host resistance",
            CoevolutionRelationship::Mutualist => "Reciprocal benefit optimization",
            CoevolutionRelationship::Competitor => "Niche differentiation race",
            CoevolutionRelationship::Mimicry => "Model-mimic pattern matching",
            CoevolutionRelationship::Pollinator => "Flower-pollinator specialization",
            CoevolutionRelationship::Disperser => "Fruit-disperser co-dependence",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptationDirection {
    Escalate,
    Deescalate,
    Specialize,
    Generalize,
    Resist,
    Overcome,
    Cooperate,
}

impl AdaptationDirection {
    pub fn fitness_effect(&self) -> Float {
        match self {
            AdaptationDirection::Escalate => 0.1,
            AdaptationDirection::Deescalate => -0.05,
            AdaptationDirection::Specialize => 0.15,
            AdaptationDirection::Generalize => 0.05,
            AdaptationDirection::Resist => 0.12,
            AdaptationDirection::Overcome => 0.1,
            AdaptationDirection::Cooperate => 0.08,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FitnessLandscape {
    pub dimensions: usize,
    pub peaks: Vec<Vec<Float>>,
    pub fitness_values: Vec<Float>,
    pub ruggedness: Float,
    pub neutrality: Float,
}

impl FitnessLandscape {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            peaks: Vec::new(),
            fitness_values: Vec::new(),
            ruggedness: 0.5,
            neutrality: 0.2,
        }
    }

    pub fn add_peak(&mut self, position: Vec<Float>, fitness: Float) {
        self.peaks.push(position);
        self.fitness_values.push(fitness);
    }

    pub fn fitness_at(&self, position: &[Float]) -> Float {
        if self.peaks.is_empty() {
            return 0.5;
        }

        let mut max_fitness: Float = 0.0;

        for (i, peak) in self.peaks.iter().enumerate() {
            let distance: Float = position
                .iter()
                .zip(peak.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<Float>()
                .sqrt();

            let peak_fitness = self.fitness_values[i];
            let fitness = peak_fitness * (-distance * self.ruggedness).exp();

            max_fitness = max_fitness.max(fitness);
        }

        max_fitness + self.neutrality * 0.5
    }

    pub fn gradient_at(&self, position: &[Float]) -> Vec<Float> {
        let epsilon = 0.01;
        let mut gradient = Vec::with_capacity(position.len());

        for i in 0..position.len() {
            let mut pos_plus = position.to_vec();
            let mut pos_minus = position.to_vec();

            pos_plus[i] += epsilon;
            pos_minus[i] -= epsilon;

            let fitness_plus = self.fitness_at(&pos_plus);
            let fitness_minus = self.fitness_at(&pos_minus);

            gradient.push((fitness_plus - fitness_minus) / (2.0 * epsilon));
        }

        gradient
    }
}

#[derive(Debug, Clone)]
pub struct CoevolutionPair {
    pub id: CoevolutionId,
    pub species_a: SpeciesId,
    pub species_b: SpeciesId,
    pub relationship: CoevolutionRelationship,
    pub adaptation_state_a: Float,
    pub adaptation_state_b: Float,
    pub coevolution_strength: Float,
    pub generations: u64,
    pub fitness_history_a: Vec<Float>,
    pub fitness_history_b: Vec<Float>,
}

impl CoevolutionPair {
    pub fn new(
        species_a: SpeciesId,
        species_b: SpeciesId,
        relationship: CoevolutionRelationship,
    ) -> Self {
        Self {
            id: CoevolutionId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            species_a,
            species_b,
            relationship,
            adaptation_state_a: 0.5,
            adaptation_state_b: 0.5,
            coevolution_strength: 1.0,
            generations: 0,
            fitness_history_a: Vec::new(),
            fitness_history_b: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: Float) {
        let rate = self.relationship.adaptation_rate() * self.coevolution_strength;

        let (a_adapts, b_adapts) = self.relationship.directionality();

        if a_adapts && b_adapts {
            let escalation = rate * dt * 0.1;

            self.adaptation_state_a = (self.adaptation_state_a + escalation).min(1.0);
            self.adaptation_state_b = (self.adaptation_state_b + escalation).min(1.0);
        }

        let fitness_a = self.calculate_fitness_a();
        let fitness_b = self.calculate_fitness_b();

        self.fitness_history_a.push(fitness_a);
        self.fitness_history_b.push(fitness_b);

        if self.fitness_history_a.len() > 100 {
            self.fitness_history_a.remove(0);
            self.fitness_history_b.remove(0);
        }

        self.generations += 1;
    }

    fn calculate_fitness_a(&self) -> Float {
        match self.relationship {
            CoevolutionRelationship::PredatorPrey => {
                0.5 + (self.adaptation_state_a - self.adaptation_state_b) * 0.3
            }
            CoevolutionRelationship::Mutualist => {
                0.5 + (self.adaptation_state_a + self.adaptation_state_b) * 0.15
            }
            _ => 0.5 + self.adaptation_state_a * 0.2,
        }
    }

    fn calculate_fitness_b(&self) -> Float {
        match self.relationship {
            CoevolutionRelationship::PredatorPrey => {
                0.5 + (self.adaptation_state_b - self.adaptation_state_a) * 0.3
            }
            CoevolutionRelationship::Mutualist => {
                0.5 + (self.adaptation_state_a + self.adaptation_state_b) * 0.15
            }
            _ => 0.5 + self.adaptation_state_b * 0.2,
        }
    }

    pub fn arms_race_intensity(&self) -> Float {
        match self.relationship {
            CoevolutionRelationship::PredatorPrey | CoevolutionRelationship::ParasiteHost => {
                (self.adaptation_state_a + self.adaptation_state_b) / 2.0
            }
            _ => 0.0,
        }
    }

    pub fn mutualism_strength(&self) -> Float {
        match self.relationship {
            CoevolutionRelationship::Mutualist | CoevolutionRelationship::Pollinator => {
                (self.adaptation_state_a + self.adaptation_state_b) / 2.0
            }
            _ => 0.0,
        }
    }

    pub fn coevolutionary_tempo(&self) -> Float {
        if self.fitness_history_a.len() < 10 {
            return 0.0;
        }

        let recent_a: Float = self.fitness_history_a.iter().rev().take(10).sum::<Float>() / 10.0;
        let older_a: Float = self.fitness_history_a.iter().take(10).sum::<Float>() / 10.0;

        (recent_a - older_a).abs()
    }
}

#[derive(Debug, Clone)]
pub struct CoevolutionSystem {
    pub pairs: HashMap<CoevolutionId, CoevolutionPair>,
    pub species_pair_map: HashMap<(SpeciesId, SpeciesId), CoevolutionId>,
    pub landscape: FitnessLandscape,
    pub global_adaptation_rate: Float,
    pub red_queen_effect: Float,
}

impl CoevolutionSystem {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
            species_pair_map: HashMap::new(),
            landscape: FitnessLandscape::new(NUM_ARCHETYPES),
            global_adaptation_rate: 0.01,
            red_queen_effect: 0.0,
        }
    }

    pub fn add_pair(&mut self, pair: CoevolutionPair) -> CoevolutionId {
        let id = pair.id;
        let key = (pair.species_a, pair.species_b);

        self.species_pair_map.insert(key, id);
        self.pairs.insert(id, pair);

        id
    }

    pub fn get_pair(
        &self,
        species_a: &SpeciesId,
        species_b: &SpeciesId,
    ) -> Option<&CoevolutionPair> {
        let key = (*species_a, *species_b);
        self.species_pair_map
            .get(&key)
            .and_then(|id| self.pairs.get(id))
            .or_else(|| {
                let reverse_key = (*species_b, *species_a);
                self.species_pair_map
                    .get(&reverse_key)
                    .and_then(|id| self.pairs.get(id))
            })
    }

    pub fn update(&mut self, dt: Float) {
        let mut total_arms_race = 0.0;
        let mut arms_race_count = 0;

        for pair in self.pairs.values_mut() {
            pair.update(dt * self.global_adaptation_rate);

            let intensity = pair.arms_race_intensity();
            if intensity > 0.0 {
                total_arms_race += intensity;
                arms_race_count += 1;
            }
        }

        if arms_race_count > 0 {
            self.red_queen_effect = total_arms_race / arms_race_count as Float;
        }

        self.global_adaptation_rate = 0.01 * (1.0 + self.red_queen_effect);
    }

    pub fn species_coevolution_partners(&self, species_id: &SpeciesId) -> Vec<SpeciesId> {
        self.pairs
            .values()
            .filter(|p| p.species_a == *species_id || p.species_b == *species_id)
            .map(|p| {
                if p.species_a == *species_id {
                    p.species_b
                } else {
                    p.species_a
                }
            })
            .collect()
    }

    pub fn calculate_adaptation_pressure(&self, species_id: &SpeciesId) -> Float {
        let partners = self.species_coevolution_partners(species_id);

        if partners.is_empty() {
            return 0.0;
        }

        let mut total_pressure = 0.0;

        for partner_id in &partners {
            if let Some(pair) = self.get_pair(species_id, partner_id) {
                let pressure = match pair.relationship {
                    CoevolutionRelationship::PredatorPrey
                    | CoevolutionRelationship::ParasiteHost => pair.arms_race_intensity() * 0.5,
                    CoevolutionRelationship::Competitor => pair.coevolution_strength * 0.3,
                    _ => 0.0,
                };
                total_pressure += pressure;
            }
        }

        total_pressure / partners.len() as Float
    }

    pub fn ecosystem_adaptation_state(&self) -> Float {
        if self.pairs.is_empty() {
            return 0.5;
        }

        let total: Float = self
            .pairs
            .values()
            .map(|p| p.adaptation_state_a + p.adaptation_state_b)
            .sum();
        total / (self.pairs.len() * 2) as Float
    }

    pub fn coevolutionary_hotspots(&self) -> Vec<CoevolutionId> {
        self.pairs
            .values()
            .filter(|p| p.coevolutionary_tempo() > 0.1)
            .map(|p| p.id)
            .collect()
    }
}

impl Default for CoevolutionSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coevolution_id_creation() {
        let id = CoevolutionId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_coevolution_relationship_directionality() {
        let (a, b) = CoevolutionRelationship::PredatorPrey.directionality();
        assert!(a && b);
    }

    #[test]
    fn test_coevolution_relationship_adaptation_rate() {
        let rate = CoevolutionRelationship::PredatorPrey.adaptation_rate();
        assert!(rate > 0.0);
    }

    #[test]
    fn test_fitness_landscape_creation() {
        let landscape = FitnessLandscape::new(10);
        assert_eq!(landscape.dimensions, 10);
    }

    #[test]
    fn test_fitness_landscape_add_peak() {
        let mut landscape = FitnessLandscape::new(3);
        landscape.add_peak(vec![0.5, 0.5, 0.5], 1.0);
        assert_eq!(landscape.peaks.len(), 1);
    }

    #[test]
    fn test_fitness_landscape_fitness_at() {
        let mut landscape = FitnessLandscape::new(3);
        landscape.add_peak(vec![0.5, 0.5, 0.5], 1.0);

        let fitness = landscape.fitness_at(&[0.5, 0.5, 0.5]);
        assert!(fitness > 0.9);

        let fitness_far = landscape.fitness_at(&[1.0, 1.0, 1.0]);
        assert!(fitness_far < fitness);
    }

    #[test]
    fn test_fitness_landscape_gradient() {
        let mut landscape = FitnessLandscape::new(3);
        landscape.add_peak(vec![0.5, 0.5, 0.5], 1.0);

        let gradient = landscape.gradient_at(&[0.3, 0.3, 0.3]);
        assert_eq!(gradient.len(), 3);
    }

    #[test]
    fn test_coevolution_pair_creation() {
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::PredatorPrey,
        );
        assert_eq!(pair.relationship, CoevolutionRelationship::PredatorPrey);
    }

    #[test]
    fn test_coevolution_pair_update() {
        let mut pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::PredatorPrey,
        );
        pair.update(1.0);

        assert!(pair.generations > 0);
        assert!(!pair.fitness_history_a.is_empty());
    }

    #[test]
    fn test_coevolution_pair_arms_race() {
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::PredatorPrey,
        );
        let intensity = pair.arms_race_intensity();
        assert!(intensity >= 0.0);
    }

    #[test]
    fn test_coevolution_system_creation() {
        let system = CoevolutionSystem::new();
        assert!(system.pairs.is_empty());
    }

    #[test]
    fn test_coevolution_system_add_pair() {
        let mut system = CoevolutionSystem::new();
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::Mutualist,
        );
        system.add_pair(pair);

        assert_eq!(system.pairs.len(), 1);
    }

    #[test]
    fn test_coevolution_system_get_pair() {
        let mut system = CoevolutionSystem::new();
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::Mutualist,
        );
        system.add_pair(pair);

        let found = system.get_pair(&SpeciesId::new(1), &SpeciesId::new(2));
        assert!(found.is_some());
    }

    #[test]
    fn test_coevolution_system_species_partners() {
        let mut system = CoevolutionSystem::new();
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::Mutualist,
        );
        system.add_pair(pair);

        let partners = system.species_coevolution_partners(&SpeciesId::new(1));
        assert!(partners.contains(&SpeciesId::new(2)));
    }

    #[test]
    fn test_coevolution_system_update() {
        let mut system = CoevolutionSystem::new();
        let pair = CoevolutionPair::new(
            SpeciesId::new(1),
            SpeciesId::new(2),
            CoevolutionRelationship::PredatorPrey,
        );
        system.add_pair(pair);

        system.update(1.0);
        assert!(system.red_queen_effect >= 0.0);
    }
}
