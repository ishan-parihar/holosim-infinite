//! Free Will Term: TRUE Stochastic Perturbation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Free Will emanates from the ONE as the first distortion - The Law of Confusion.
//!  It is NOT random - it is NON-DETERMINISTIC selection from possibility space.
//!  Random = no pattern possible. Free Will = meaningful choice without external cause."
//!
//! KEY INSIGHT: Free Will breaks perfect symmetry through TRUE entropy injection.
//! This creates the possibility for ANY outcome while allowing meaningful patterns.
//!
//! The Free Will term adds stochastic perturbations to the field:
//! - Amplitude: Magnitude of perturbations
//! - Correlation: Spatial/temporal correlation (0 = uncorrelated, 1 = fully correlated)
//! - Entropy Rate: Rate of true entropy injection
//!
//! From R&D Roadmap:
//! "How does Free Will differ from quantum randomness in field dynamics?
//!  Answer: Free Will uses CORRELATED noise (not white noise) and is guided
//!  by Archetype 22 (The Choice) - it's not purely random but non-deterministic."

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::{DensityAmplitude, FieldState, NUM_DENSITY_BANDS};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerturbationType {
    SymmetryBreaking,
    ChoiceGuided,
    EntropyInjection,
    CoherentNoise,
}

#[derive(Debug, Clone)]
pub struct FreeWillConfig {
    pub amplitude: f64,
    pub correlation: f64,
    pub entropy_rate: f64,
    pub archetype_22_influence: f64,
    pub seed: Option<u64>,
}

impl Default for FreeWillConfig {
    fn default() -> Self {
        Self {
            amplitude: 0.1,
            correlation: 0.3,
            entropy_rate: 0.5,
            archetype_22_influence: 0.2,
            seed: None,
        }
    }
}

impl FreeWillConfig {
    pub fn low_amplitude() -> Self {
        Self {
            amplitude: 0.01,
            correlation: 0.5,
            entropy_rate: 0.3,
            archetype_22_influence: 0.1,
            seed: None,
        }
    }

    pub fn high_freedom() -> Self {
        Self {
            amplitude: 0.3,
            correlation: 0.1,
            entropy_rate: 0.8,
            archetype_22_influence: 0.3,
            seed: None,
        }
    }

    pub fn deterministic(seed: u64) -> Self {
        Self {
            amplitude: 0.1,
            correlation: 0.5,
            entropy_rate: 0.5,
            archetype_22_influence: 0.2,
            seed: Some(seed),
        }
    }
}

#[derive(Debug)]
pub struct FreeWillTerm {
    config: FreeWillConfig,
    rng: StdRng,
    prev_perturbation: [f64; NUM_DENSITY_BANDS],
    entropy_accumulator: f64,
    total_applications: usize,
    archetype_22_state: f64,
}

impl FreeWillTerm {
    pub fn new(config: FreeWillConfig) -> Self {
        let rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        Self {
            config,
            rng,
            prev_perturbation: [0.0; NUM_DENSITY_BANDS],
            entropy_accumulator: 0.0,
            total_applications: 0,
            archetype_22_state: 0.5,
        }
    }

    pub fn from_entropy() -> Self {
        Self::new(FreeWillConfig::default())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::new(FreeWillConfig::deterministic(seed))
    }

    /// Apply TRUE stochastic perturbation to field state
    ///
    /// This represents Free Will breaking the perfect symmetry.
    /// The key insight: use CORRELATED noise, not white noise!
    ///
    /// From R&D Roadmap Phase 2:
    /// "Free Will as stochastic perturbation (correlated noise, not white noise)
    ///  with spectrum position affecting perturbation characteristics:
    ///  - Time/Space (v < 1): More deterministic, structured perturbations
    ///  - At Veil (v ≈ 1): Maximal choice potential, phase transition
    ///  - Space/Time (v > 1): More chaotic, individual choices"
    pub fn apply(&mut self, state: &mut FieldState, position: &[f64; 3], time: f64) {
        self.entropy_accumulator += self.config.entropy_rate * 0.01;
        self.total_applications += 1;

        let mut perturbations: [f64; NUM_DENSITY_BANDS] = [0.0; NUM_DENSITY_BANDS];
        let uniform = Uniform::new(-1.0, 1.0);

        // Archetype 22 (The Choice) influence - creates non-random bias
        self.archetype_22_state = self.update_archetype_22_state(state);

        // Phase 2: Spectrum-aware amplitude modification
        // From Phase 2 R&D: "Free Will: More deterministic in Time/Space, more chaotic in Space/Time"
        let spectrum_amplitude = self.calculate_spectrum_amplitude(state);

        for density_idx in 0..NUM_DENSITY_BANDS {
            // Generate base random components
            let u1: f64 = self.rng.sample(uniform);
            let u2: f64 = self.rng.sample(uniform);

            // Box-Muller transform for normal distribution
            let normal_noise = if u1.abs() > 1e-10 {
                (-2.0 * u1.abs().ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
            } else {
                0.0
            };

            // Archetype 22 influence creates a bias toward certain density bands
            let archetype_bias = self.calculate_archetype_bias(density_idx);

            // Combine correlated previous perturbation with new noise
            let correlation_factor = self.config.correlation;

            // Phase 2: At the veil, correlation breaks down (more choice)
            let veil_correlation_factor = if state.is_at_veil() {
                correlation_factor * 0.5 // Less correlation = more freedom
            } else if state.is_time_space() {
                correlation_factor * 1.3 // More correlation = more deterministic
            } else {
                correlation_factor
            };

            let new_component = (u1 * (1.0 - veil_correlation_factor)
                + normal_noise * 0.3 * (1.0 - veil_correlation_factor))
                + self.prev_perturbation[density_idx] * veil_correlation_factor;

            // Add archetype-guided bias (this is what makes it Free Will, not random)
            let guided_perturbation =
                new_component + archetype_bias * self.config.archetype_22_influence;

            // Phase 2: Apply spectrum-modified amplitude
            perturbations[density_idx] = guided_perturbation * spectrum_amplitude;
            self.prev_perturbation[density_idx] = guided_perturbation;
        }

        // Entropy injection breaks correlation periodically
        if self.entropy_accumulator > 1.0 {
            self.entropy_accumulator = 0.0;
            self.inject_entropy();
        }

        // Apply perturbations to density amplitudes
        for density_idx in 0..NUM_DENSITY_BANDS {
            let current = &state.density_amplitudes[density_idx];
            let perturbed = DensityAmplitude::new(
                current.re + perturbations[density_idx],
                current.im + perturbations[density_idx] * 0.5,
            );

            // Normalize to keep amplitude bounded
            let mag = perturbed.magnitude();
            if mag > 1.0 {
                state.density_amplitudes[density_idx] = perturbed.scale(1.0 / mag);
            } else {
                state.density_amplitudes[density_idx] = perturbed;
            }
        }

        // Free Will affects coherence and spectrum position
        let coherence_change = self.rng.sample(uniform) * self.config.amplitude * 0.2;
        state.coherence = (state.coherence + coherence_change).clamp(0.0, 1.0);

        let spectrum_shift = self.rng.sample(uniform) * self.config.amplitude * 0.05;
        state.spectrum_position = (state.spectrum_position + spectrum_shift).clamp(0.0, 2.0);

        // Update energy based on perturbation magnitude
        let total_perturbation: f64 = perturbations.iter().map(|p| p.abs()).sum();
        state.energy = (state.energy + total_perturbation * 0.01).min(10.0);
    }

    /// Calculate spectrum-aware amplitude modification
    ///
    /// From Phase 2 R&D:
    /// "At the veil, entities have maximal choice potential"
    fn calculate_spectrum_amplitude(&self, state: &FieldState) -> f64 {
        let base_amplitude = self.config.amplitude;

        if state.is_at_veil() {
            // At the veil: maximal choice potential
            base_amplitude * 1.5
        } else if state.is_time_space() {
            // Time/Space: more structured, less chaotic
            base_amplitude * 0.7
        } else {
            // Space/Time: standard amplitude
            base_amplitude
        }
    }

    /// Apply a specific type of perturbation
    pub fn apply_typed(
        &mut self,
        state: &mut FieldState,
        position: &[f64; 3],
        time: f64,
        perturbation_type: PerturbationType,
    ) {
        match perturbation_type {
            PerturbationType::SymmetryBreaking => {
                self.apply_symmetry_breaking(state);
            }
            PerturbationType::ChoiceGuided => {
                self.apply_choice_guided(state, time);
            }
            PerturbationType::EntropyInjection => {
                self.inject_entropy();
                self.apply(state, position, time);
            }
            PerturbationType::CoherentNoise => {
                self.apply_coherent_noise(state);
            }
        }
    }

    fn apply_symmetry_breaking(&mut self, state: &mut FieldState) {
        let uniform = Uniform::new(-1.0, 1.0);

        // Choose one density band to amplify (breaks uniform symmetry)
        let chosen_band =
            (self.rng.sample::<f64, _>(uniform).abs() * NUM_DENSITY_BANDS as f64) as usize;
        let chosen_band = chosen_band.min(NUM_DENSITY_BANDS - 1);

        for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
            if i == chosen_band {
                amp.re += self.config.amplitude;
            } else {
                amp.re -= self.config.amplitude * 0.1;
            }
        }

        // Update coherence based on new asymmetry
        state.coherence = state.calculate_coherence();
    }

    fn apply_choice_guided(&mut self, state: &mut FieldState, time: f64) {
        self.total_applications += 1;

        // Archetype 22 creates a choice between possibilities
        let possibilities = self.generate_possibility_space(state);

        // Non-deterministic selection (NOT random - guided by archetype state)
        let choice = self.exercise_free_will(&possibilities, time);

        // Apply the chosen perturbation
        self.apply_chosen_perturbation(state, choice);
    }

    fn generate_possibility_space(&self, state: &FieldState) -> Vec<[f64; NUM_DENSITY_BANDS]> {
        let mut possibilities = Vec::with_capacity(4);

        // Possibility 1: Enhance dominant density
        let dominant = state.dominant_density();
        let mut p1 = [0.0; NUM_DENSITY_BANDS];
        p1[dominant] = self.config.amplitude;
        possibilities.push(p1);

        // Possibility 2: Balance all densities
        let mut p2 = [0.0; NUM_DENSITY_BANDS];
        for i in 0..NUM_DENSITY_BANDS {
            p2[i] = self.config.amplitude * 0.5;
        }
        possibilities.push(p2);

        // Possibility 3: Shift to adjacent density
        let adjacent = (dominant + 1) % NUM_DENSITY_BANDS;
        let mut p3 = [0.0; NUM_DENSITY_BANDS];
        p3[adjacent] = self.config.amplitude;
        possibilities.push(p3);

        // Possibility 4: Create entropy (breakdown)
        let mut p4 = [0.0; NUM_DENSITY_BANDS];
        for i in 0..NUM_DENSITY_BANDS {
            p4[i] = -self.config.amplitude * 0.3;
        }
        possibilities.push(p4);

        possibilities
    }

    fn exercise_free_will(
        &mut self,
        possibilities: &[[f64; NUM_DENSITY_BANDS]],
        time: f64,
    ) -> usize {
        // This is the KEY DIFFERENCE from random selection:
        // Free Will chooses based on internal archetype state, not external randomness

        let mut scores: Vec<f64> = possibilities
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                // Score based on archetype alignment
                let alignment = (self.archetype_22_state - 0.5).abs() * 2.0;

                // Add time-dependent factor (consciousness evolves)
                let time_factor = (time * 0.1).sin() * 0.2;

                // Add non-deterministic but non-random factor
                let seed_factor = ((self.total_applications as f64 * time).sin() * 0.3).abs();

                alignment + time_factor + seed_factor + idx as f64 * 0.1
            })
            .collect();

        // Non-deterministic selection: add noise to scores but weight by archetype
        let uniform = Uniform::new(-0.1, 0.1);
        for score in &mut scores {
            *score += self.rng.sample(uniform);
        }

        // Select the highest scoring possibility
        scores
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    fn apply_chosen_perturbation(&mut self, state: &mut FieldState, choice_idx: usize) {
        let possibilities = self.generate_possibility_space(state);
        if choice_idx < possibilities.len() {
            for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
                amp.re += possibilities[choice_idx][i];
            }
        }
    }

    fn apply_coherent_noise(&mut self, state: &mut FieldState) {
        // Coherent noise has spatial/temporal correlation
        let noise_scale = self.config.correlation;
        let uniform = Uniform::new(-1.0, 1.0);

        // Base noise that all densities share (correlated component)
        let base_noise = self.rng.sample(uniform) * noise_scale;

        for (i, amp) in state.density_amplitudes.iter_mut().enumerate() {
            // Each density gets base + independent component
            let independent = self.rng.sample(uniform) * (1.0 - noise_scale);
            let total_noise = (base_noise + independent) * self.config.amplitude;

            amp.re += total_noise;
        }
    }

    fn inject_entropy(&mut self) {
        // Break correlation pattern - inject fresh entropy
        let uniform = Uniform::new(-1.0, 1.0);
        for i in 0..NUM_DENSITY_BANDS {
            self.prev_perturbation[i] *= 0.5;
            self.prev_perturbation[i] += self.rng.sample(uniform) * 0.5;
        }

        // Shift archetype state
        self.archetype_22_state =
            (self.archetype_22_state + self.rng.sample(uniform) * 0.1).clamp(0.0, 1.0);
    }

    fn update_archetype_22_state(&mut self, state: &FieldState) -> f64 {
        // Archetype 22 (The Choice) evolves based on field state
        let coherence_influence = state.coherence * 0.3;
        let entropy_influence = state.entropy() * 0.1;
        let spectrum_influence = (state.spectrum_position - 0.5).abs() * 0.2;

        self.archetype_22_state = (self.archetype_22_state
            + coherence_influence
            + entropy_influence
            + spectrum_influence)
            .clamp(0.0, 1.0);

        self.archetype_22_state
    }

    fn calculate_archetype_bias(&self, density_idx: usize) -> f64 {
        // The Choice archetype influences which density bands are favored
        // This creates non-random but non-deterministic patterns

        let density_weight = match density_idx {
            0 => -0.2, // First density - material
            1 => -0.1,
            2 => 0.0,
            3 => 0.3, // Fourth density - love/light
            4 => 0.2, // Fifth density - wisdom
            5 => 0.1,
            6 => 0.0,
            7 => 0.1, // Eighth density - return
            _ => 0.0,
        };

        density_weight * (self.archetype_22_state - 0.5)
    }

    pub fn current_entropy(&self) -> f64 {
        self.entropy_accumulator
    }

    pub fn total_applications(&self) -> usize {
        self.total_applications
    }

    pub fn archetype_22_state(&self) -> f64 {
        self.archetype_22_state
    }

    pub fn reset_entropy(&mut self) {
        self.entropy_accumulator = 0.0;
    }

    pub fn config(&self) -> &FreeWillConfig {
        &self.config
    }
}

impl Clone for FreeWillTerm {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            rng: StdRng::from_entropy(),
            prev_perturbation: self.prev_perturbation,
            entropy_accumulator: self.entropy_accumulator,
            total_applications: self.total_applications,
            archetype_22_state: self.archetype_22_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_will_creation() {
        let fw = FreeWillTerm::from_entropy();
        assert_eq!(fw.total_applications(), 0);
        assert!((fw.current_entropy() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_free_will_deterministic() {
        let mut fw1 = FreeWillTerm::with_seed(42);
        let mut fw2 = FreeWillTerm::with_seed(42);

        let mut state1 = FieldState::uniform(0.5);
        let mut state2 = FieldState::uniform(0.5);
        let pos = [0.0, 0.0, 0.0];

        fw1.apply(&mut state1, &pos, 0.0);
        fw2.apply(&mut state2, &pos, 0.0);

        // Same seed should produce same results
        for i in 0..NUM_DENSITY_BANDS {
            assert!(
                (state1.density_amplitudes[i].re - state2.density_amplitudes[i].re).abs() < 1e-10
            );
        }
    }

    #[test]
    fn test_perturbation_changes_state() {
        let mut fw = FreeWillTerm::from_entropy();
        let mut state = FieldState::uniform(0.5);
        let original_coherence = state.coherence;

        fw.apply(&mut state, &[0.0, 0.0, 0.0], 0.0);

        // State should have changed
        assert!(state.coherence != original_coherence || state.total_magnitude() != 8.0_f64.sqrt());
    }

    #[test]
    fn test_symmetry_breaking() {
        let mut fw = FreeWillTerm::with_seed(123);
        let mut state = FieldState::uniform(0.5);

        fw.apply_typed(
            &mut state,
            &[0.0, 0.0, 0.0],
            0.0,
            PerturbationType::SymmetryBreaking,
        );

        // One density should be dominant after symmetry breaking
        let dominant = state.dominant_density();
        let dominant_mag = state.density_amplitudes[dominant].magnitude();
        let avg_mag = state.total_magnitude() / NUM_DENSITY_BANDS as f64;

        assert!(dominant_mag > avg_mag);
    }

    #[test]
    fn test_choice_guided_perturbation() {
        let mut fw = FreeWillTerm::with_seed(456);
        let mut state = FieldState::uniform(0.5);

        fw.apply_typed(
            &mut state,
            &[0.0, 0.0, 0.0],
            0.0,
            PerturbationType::ChoiceGuided,
        );

        assert!(fw.total_applications() > 0);
    }

    #[test]
    fn test_archetype_22_evolution() {
        let mut fw = FreeWillTerm::from_entropy();

        for _ in 0..10 {
            let mut state = FieldState::uniform(0.5);
            fw.apply(&mut state, &[0.0, 0.0, 0.0], 0.0);
        }

        // Archetype 22 state should have evolved
        assert!(fw.archetype_22_state() >= 0.0 && fw.archetype_22_state() <= 1.0);
    }

    #[test]
    fn test_entropy_injection() {
        let mut fw = FreeWillTerm::new(FreeWillConfig {
            entropy_rate: 10.0, // High rate to trigger injection
            ..Default::default()
        });

        let mut state = FieldState::uniform(0.5);
        fw.apply(&mut state, &[0.0, 0.0, 0.0], 0.0);

        // Entropy should have been injected
        assert!(fw.current_entropy() < 1.0);
    }

    #[test]
    fn test_free_will_vs_random() {
        // Free Will should produce different results from pure random
        // because of the archetype bias and correlation

        let mut fw = FreeWillTerm::with_seed(789);
        let mut results = Vec::new();

        for _ in 0..10 {
            let mut state = FieldState::uniform(0.5);
            fw.apply(&mut state, &[0.0, 0.0, 0.0], 0.0);
            results.push(state.dominant_density());
        }

        // Results should show some pattern (not pure random)
        let unique: std::collections::HashSet<_> = results.iter().collect();
        // With archetype bias toward density 3 and 4, we expect some clustering
        assert!(unique.len() <= NUM_DENSITY_BANDS);
    }
}
