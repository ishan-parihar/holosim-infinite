//! Population Dynamics from Field Amplitude
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Population growth = field amplitude growth
//!  Oscillations = field resonance between coupled species"
//!
//! # Key Insight
//!
//! Population dynamics emerge from field amplitude dynamics:
//! - Population size = field amplitude at species location
//! - Growth rate = field amplitude growth rate
//! - Carrying capacity = field amplitude saturation
//! - Oscillations = resonance between coupled population fields

use crate::holographic_foundation::ecosystem_dynamics::species_field::SpeciesId;
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PopulationId(pub u64);

impl PopulationId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OscillationPattern {
    Stable,
    PredatorPrey,
    Competitive,
    Cyclic,
    Chaotic,
    Damped,
}

impl OscillationPattern {
    pub fn period(&self) -> Float {
        match self {
            OscillationPattern::Stable => 0.0,
            OscillationPattern::PredatorPrey => 10.0,
            OscillationPattern::Competitive => 15.0,
            OscillationPattern::Cyclic => 20.0,
            OscillationPattern::Chaotic => 0.0,
            OscillationPattern::Damped => 8.0,
        }
    }

    pub fn amplitude_factor(&self) -> Float {
        match self {
            OscillationPattern::Stable => 0.0,
            OscillationPattern::PredatorPrey => 0.3,
            OscillationPattern::Competitive => 0.2,
            OscillationPattern::Cyclic => 0.4,
            OscillationPattern::Chaotic => 0.8,
            OscillationPattern::Damped => 0.15,
        }
    }

    pub fn damping_rate(&self) -> Float {
        match self {
            OscillationPattern::Stable => 0.0,
            OscillationPattern::PredatorPrey => 0.01,
            OscillationPattern::Competitive => 0.02,
            OscillationPattern::Cyclic => 0.005,
            OscillationPattern::Chaotic => 0.0,
            OscillationPattern::Damped => 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CarryingCapacity {
    pub base_capacity: Float,
    pub resource_availability: Float,
    pub environmental_modifier: Float,
    pub competition_modifier: Float,
    pub predator_pressure: Float,
}

impl CarryingCapacity {
    pub fn new(base: Float) -> Self {
        Self {
            base_capacity: base,
            resource_availability: 1.0,
            environmental_modifier: 1.0,
            competition_modifier: 1.0,
            predator_pressure: 0.0,
        }
    }

    pub fn effective_capacity(&self) -> Float {
        let base = self.base_capacity * self.resource_availability * self.environmental_modifier;
        let reduction = self.competition_modifier * self.predator_pressure;
        (base * (1.0 - reduction)).max(1.0)
    }

    pub fn update(&mut self, resources: Float, competitors: Float, predators: Float) {
        self.resource_availability = (resources / (self.base_capacity * 0.1)).clamp(0.1, 1.0);
        self.competition_modifier = (1.0 - competitors * 0.01).max(0.1);
        self.predator_pressure = (predators * 0.02).min(0.8);
    }
}

#[derive(Debug, Clone)]
pub struct PopulationDynamics {
    pub intrinsic_growth_rate: Float,
    pub death_rate: Float,
    pub emigration_rate: Float,
    pub immigration_rate: Float,
    pub allee_threshold: Float,
    pub oscillation_phase: Float,
    pub resonance_frequency: Float,
}

impl PopulationDynamics {
    pub fn new(growth_rate: Float) -> Self {
        Self {
            intrinsic_growth_rate: growth_rate,
            death_rate: 0.1,
            emigration_rate: 0.02,
            immigration_rate: 0.02,
            allee_threshold: 10.0,
            oscillation_phase: 0.0,
            resonance_frequency: 0.1,
        }
    }

    pub fn net_growth_rate(&self) -> Float {
        self.intrinsic_growth_rate - self.death_rate + self.immigration_rate - self.emigration_rate
    }

    pub fn logistic_growth(&self, population: Float, carrying_capacity: Float) -> Float {
        if carrying_capacity <= 0.0 || population <= 0.0 {
            return 0.0;
        }

        let allee_effect = if population < self.allee_threshold {
            population / self.allee_threshold
        } else {
            1.0
        };

        let logistic_factor = 1.0 - population / carrying_capacity;
        self.net_growth_rate() * population * logistic_factor * allee_effect
    }

    pub fn oscillation_modifier(&self, time: Float, pattern: OscillationPattern) -> Float {
        if matches!(pattern, OscillationPattern::Stable) {
            return 1.0;
        }

        let period = pattern.period();
        if period <= 0.0 {
            return 1.0;
        }

        let phase = self.oscillation_phase + time * self.resonance_frequency;
        let damping = (-pattern.damping_rate() * time).exp();

        1.0 + pattern.amplitude_factor()
            * (2.0 * std::f64::consts::PI * phase / period).sin()
            * damping
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    pub id: PopulationId,
    pub species_id: SpeciesId,
    pub size: Float,
    pub field_amplitude: Float,
    pub phase: Float,
    pub dynamics: PopulationDynamics,
    pub carrying_capacity: CarryingCapacity,
    pub oscillation_pattern: OscillationPattern,
    pub age_structure: [Float; 5],
    pub fitness: Float,
    pub history: Vec<Float>,
}

impl Population {
    pub fn new(species_id: SpeciesId, initial_size: Float, base_capacity: Float) -> Self {
        let field_amplitude = initial_size.sqrt();
        let growth_rate = 0.3;

        Self {
            id: PopulationId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            species_id,
            size: initial_size,
            field_amplitude,
            phase: 0.0,
            dynamics: PopulationDynamics::new(growth_rate),
            carrying_capacity: CarryingCapacity::new(base_capacity),
            oscillation_pattern: OscillationPattern::Stable,
            age_structure: [0.2, 0.3, 0.3, 0.15, 0.05],
            fitness: 0.8,
            history: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: Float, resources: Float, competitors: Float, predators: Float) {
        self.carrying_capacity
            .update(resources, competitors, predators);

        let effective_k = self.carrying_capacity.effective_capacity();
        let growth = self.dynamics.logistic_growth(self.size, effective_k);

        let oscillation = self
            .dynamics
            .oscillation_modifier(dt, self.oscillation_pattern);
        let effective_growth = growth * oscillation;

        self.size = (self.size + effective_growth * dt).max(0.0);

        self.field_amplitude = self.size.sqrt();
        self.phase += dt * self.dynamics.resonance_frequency;

        self.history.push(self.size);
        if self.history.len() > 100 {
            self.history.remove(0);
        }

        self.fitness = self.calculate_fitness();
    }

    fn calculate_fitness(&self) -> Float {
        if self.size < self.allee_threshold() {
            return self.fitness * 0.99;
        }

        let optimal_ratio = self.size / self.carrying_capacity.effective_capacity();
        let capacity_fitness = 1.0 - (optimal_ratio - 0.5).abs() * 0.5;

        let stability_fitness = self.calculate_stability();

        (capacity_fitness * 0.6 + stability_fitness * 0.4).clamp(0.1, 1.0)
    }

    fn calculate_stability(&self) -> Float {
        if self.history.len() < 10 {
            return 1.0;
        }

        let recent: Vec<Float> = self.history.iter().rev().take(10).cloned().collect();
        let mean = recent.iter().sum::<Float>() / recent.len() as Float;

        if mean == 0.0 {
            return 0.0;
        }

        let variance: Float =
            recent.iter().map(|x| (x - mean).powi(2)).sum::<Float>() / recent.len() as Float;
        let cv = (variance.sqrt() / mean).min(1.0);

        1.0 - cv
    }

    pub fn allee_threshold(&self) -> Float {
        self.dynamics.allee_threshold
    }

    pub fn density(&self, area: Float) -> Float {
        if area > 0.0 {
            self.size / area
        } else {
            0.0
        }
    }

    pub fn per_capita_growth(&self) -> Float {
        if self.size > 0.0 {
            self.dynamics.net_growth_rate()
        } else {
            0.0
        }
    }

    pub fn is_viable(&self) -> bool {
        self.size >= self.allee_threshold() && self.fitness > 0.2
    }

    pub fn field_strength(&self) -> Float {
        self.field_amplitude * self.fitness
    }

    pub fn resonance_with(&self, other: &Population) -> Float {
        let phase_diff = (self.phase - other.phase).abs();
        let normalized_phase = if phase_diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - phase_diff
        } else {
            phase_diff
        };

        let phase_resonance = (normalized_phase / std::f64::consts::PI).cos();
        let amplitude_resonance = (self.field_amplitude * other.field_amplitude).sqrt()
            / (self.field_amplitude + other.field_amplitude).max(0.001);

        phase_resonance * amplitude_resonance
    }

    pub fn detect_oscillation_pattern(&mut self) {
        if self.history.len() < 20 {
            self.oscillation_pattern = OscillationPattern::Stable;
            return;
        }

        let recent: Vec<Float> = self.history.iter().rev().take(20).cloned().collect();
        let mean = recent.iter().sum::<Float>() / recent.len() as Float;

        let mut peaks = 0;
        let mut troughs = 0;
        for i in 1..recent.len() - 1 {
            if recent[i] > recent[i - 1] && recent[i] > recent[i + 1] {
                peaks += 1;
            } else if recent[i] < recent[i - 1] && recent[i] < recent[i + 1] {
                troughs += 1;
            }
        }

        let oscillation_count = peaks + troughs;

        let variance: Float =
            recent.iter().map(|x| (x - mean).powi(2)).sum::<Float>() / recent.len() as Float;
        let cv = variance.sqrt() / mean.max(0.001);

        if cv < 0.05 {
            self.oscillation_pattern = OscillationPattern::Stable;
        } else if oscillation_count >= 4 && cv < 0.3 {
            self.oscillation_pattern = OscillationPattern::Damped;
        } else if oscillation_count >= 3 {
            self.oscillation_pattern = OscillationPattern::PredatorPrey;
        } else if cv > 0.5 {
            self.oscillation_pattern = OscillationPattern::Chaotic;
        } else {
            self.oscillation_pattern = OscillationPattern::Cyclic;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_id_creation() {
        let id = PopulationId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_carrying_capacity_creation() {
        let cc = CarryingCapacity::new(1000.0);
        assert_eq!(cc.base_capacity, 1000.0);
        assert!(cc.effective_capacity() > 0.0);
    }

    #[test]
    fn test_carrying_capacity_update() {
        let mut cc = CarryingCapacity::new(1000.0);
        cc.update(500.0, 10.0, 5.0);
        assert!(cc.resource_availability > 0.0);
    }

    #[test]
    fn test_population_dynamics_creation() {
        let pd = PopulationDynamics::new(0.3);
        assert_eq!(pd.intrinsic_growth_rate, 0.3);
    }

    #[test]
    fn test_population_dynamics_logistic_growth() {
        let pd = PopulationDynamics::new(0.5);
        let growth = pd.logistic_growth(100.0, 1000.0);
        assert!(growth > 0.0);
    }

    #[test]
    fn test_population_dynamics_allee_effect() {
        let pd = PopulationDynamics::new(0.5);
        let small_pop = pd.logistic_growth(5.0, 1000.0);
        let larger_pop = pd.logistic_growth(50.0, 1000.0);
        assert!(small_pop < larger_pop);
    }

    #[test]
    fn test_population_creation() {
        let pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        assert_eq!(pop.size, 100.0);
        assert!(pop.field_amplitude > 0.0);
    }

    #[test]
    fn test_population_update() {
        let mut pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        pop.update(1.0, 500.0, 0.0, 0.0);
        assert!(pop.size > 0.0);
        assert!(!pop.history.is_empty());
    }

    #[test]
    fn test_population_field_strength() {
        let pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        assert!(pop.field_strength() > 0.0);
    }

    #[test]
    fn test_population_resonance() {
        let pop1 = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        let pop2 = Population::new(SpeciesId::new(2), 100.0, 1000.0);
        let resonance = pop1.resonance_with(&pop2);
        assert!((0.0..=1.0).contains(&resonance));
    }

    #[test]
    fn test_population_viability() {
        let mut pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        pop.dynamics.allee_threshold = 10.0;
        assert!(pop.is_viable());

        pop.size = 5.0;
        assert!(!pop.is_viable());
    }

    #[test]
    fn test_population_density() {
        let pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);
        let density = pop.density(10.0);
        assert_eq!(density, 10.0);
    }

    #[test]
    fn test_oscillation_pattern_period() {
        assert!(OscillationPattern::PredatorPrey.period() > 0.0);
        assert_eq!(OscillationPattern::Stable.period(), 0.0);
    }

    #[test]
    fn test_population_detect_oscillation() {
        let mut pop = Population::new(SpeciesId::new(1), 100.0, 1000.0);

        for _ in 0..25 {
            pop.history.push(100.0);
        }
        pop.detect_oscillation_pattern();
        assert_eq!(pop.oscillation_pattern, OscillationPattern::Stable);
    }
}
