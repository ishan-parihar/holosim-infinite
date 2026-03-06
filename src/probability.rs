// Probability System - Phase 10 Implementation
// Models quantum superposition, probability amplitudes, and collapse
// Enables free will choice to collapse possibilities into actualities

use serde::{Deserialize, Serialize};
use std::fmt;

/// Float type for physical calculations
pub type Float = f64;

// ============================================================================
// COMPLEX NUMBERS (for probability amplitudes)
// ============================================================================

/// Complex number for probability amplitudes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub real: Float,
    pub imaginary: Float,
}

impl Complex {
    /// Create a complex number
    pub fn new(real: Float, imaginary: Float) -> Self {
        Complex { real, imaginary }
    }

    /// Zero complex number
    pub fn zero() -> Self {
        Complex {
            real: 0.0,
            imaginary: 0.0,
        }
    }

    /// One complex number
    pub fn one() -> Self {
        Complex {
            real: 1.0,
            imaginary: 0.0,
        }
    }

    /// Imaginary unit (i)
    pub fn i() -> Self {
        Complex {
            real: 0.0,
            imaginary: 1.0,
        }
    }

    /// Magnitude (absolute value)
    pub fn magnitude(&self) -> Float {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    /// Magnitude squared
    pub fn magnitude_squared(&self) -> Float {
        self.real * self.real + self.imaginary * self.imaginary
    }

    /// Phase (argument)
    pub fn phase(&self) -> Float {
        self.imaginary.atan2(self.real)
    }

    /// Complex conjugate
    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    /// Add two complex numbers
    pub fn add(&self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    /// Subtract two complex numbers
    pub fn sub(&self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }

    /// Multiply two complex numbers
    pub fn mul(&self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    /// Scale by real number
    pub fn scale(&self, scalar: Float) -> Complex {
        Complex {
            real: self.real * scalar,
            imaginary: self.imaginary * scalar,
        }
    }

    /// Normalize to unit magnitude
    pub fn normalize(&self) -> Option<Complex> {
        let mag = self.magnitude();
        if mag > 0.0 {
            Some(Complex {
                real: self.real / mag,
                imaginary: self.imaginary / mag,
            })
        } else {
            None
        }
    }

    /// Convert to polar form (magnitude, phase)
    pub fn to_polar(&self) -> (Float, Float) {
        (self.magnitude(), self.phase())
    }

    /// Create from polar form
    pub fn from_polar(magnitude: Float, phase: Float) -> Self {
        Complex {
            real: magnitude * phase.cos(),
            imaginary: magnitude * phase.sin(),
        }
    }
}

// ============================================================================
// PROBABILITY SPACE
// ============================================================================

/// Outcome in probability space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub id: usize,
    pub amplitude: Complex,
    pub probability: Float,
    pub description: Option<String>,
}

impl Outcome {
    /// Create an outcome
    pub fn new(id: usize, amplitude: Complex, description: Option<String>) -> Self {
        let probability = amplitude.magnitude_squared();
        Outcome {
            id,
            amplitude,
            probability,
            description,
        }
    }

    /// Update probability from amplitude
    pub fn update_probability(&mut self) {
        self.probability = self.amplitude.magnitude_squared();
    }
}

/// Probability space - contains all possible outcomes with amplitudes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilitySpace {
    /// Probability amplitudes for each outcome
    pub amplitudes: Vec<Complex>,
    /// Probability distribution (normalized)
    pub distribution: Vec<Float>,
    /// Collapsed outcome (if any)
    pub collapsed: Option<usize>,
    /// Coherence (0.0 = completely decoherent, 1.0 = fully coherent)
    pub coherence: Float,
    /// Outcomes
    pub outcomes: Vec<Outcome>,
    /// Random seed for deterministic collapse
    pub seed: u64,
}

impl Default for ProbabilitySpace {
    fn default() -> Self {
        Self::empty()
    }
}

impl ProbabilitySpace {
    /// Create empty probability space
    pub fn empty() -> Self {
        ProbabilitySpace {
            amplitudes: Vec::new(),
            distribution: Vec::new(),
            collapsed: None,
            coherence: 1.0,
            outcomes: Vec::new(),
            seed: 42,
        }
    }

    /// Create probability space with given number of outcomes
    pub fn with_outcomes(num_outcomes: usize, seed: u64) -> Self {
        let mut space = ProbabilitySpace {
            amplitudes: vec![Complex::zero(); num_outcomes],
            distribution: vec![0.0; num_outcomes],
            collapsed: None,
            coherence: 1.0,
            outcomes: Vec::new(),
            seed,
        };

        // Initialize with equal amplitudes
        let equal_amplitude = Complex::from_polar(1.0 / (num_outcomes as Float).sqrt(), 0.0);
        for i in 0..num_outcomes {
            space.amplitudes[i] = equal_amplitude;
            space.outcomes.push(Outcome::new(i, equal_amplitude, None));
        }

        space.normalize();
        space
    }

    /// Create probability space with custom amplitudes
    pub fn with_amplitudes(amplitudes: Vec<Complex>, seed: u64) -> Self {
        let num_outcomes = amplitudes.len();
        let mut space = ProbabilitySpace {
            amplitudes: amplitudes.clone(),
            distribution: vec![0.0; num_outcomes],
            collapsed: None,
            coherence: 1.0,
            outcomes: Vec::new(),
            seed,
        };

        for (i, amplitude) in amplitudes.iter().enumerate() {
            space.outcomes.push(Outcome::new(i, *amplitude, None));
        }

        space.normalize();
        space
    }

    /// Normalize probability distribution
    pub fn normalize(&mut self) {
        let total_prob: Float = self.amplitudes.iter().map(|a| a.magnitude_squared()).sum();

        if total_prob > 0.0 {
            let norm_factor = 1.0 / total_prob.sqrt();
            for amplitude in &mut self.amplitudes {
                *amplitude = amplitude.scale(norm_factor);
            }
        }

        // Update probability distribution
        self.distribution = self
            .amplitudes
            .iter()
            .map(|a| a.magnitude_squared())
            .collect();

        // Update outcome probabilities
        for (i, outcome) in self.outcomes.iter_mut().enumerate() {
            if i < self.amplitudes.len() {
                outcome.amplitude = self.amplitudes[i];
                outcome.update_probability();
            }
        }
    }

    /// Collapse probability space to a specific outcome
    pub fn collapse(&mut self, choice: Option<usize>) -> Outcome {
        if let Some(idx) = self.collapsed {
            // Already collapsed, return existing outcome
            return self.outcomes[idx].clone();
        }

        let selected = if let Some(choice_idx) = choice {
            choice_idx
        } else {
            // Deterministic collapse using seed
            self.deterministic_collapse()
        };

        // Collapse to selected outcome
        self.collapsed = Some(selected);

        // Set all amplitudes to zero except selected
        for i in 0..self.amplitudes.len() {
            if i == selected {
                self.amplitudes[i] = Complex::one();
            } else {
                self.amplitudes[i] = Complex::zero();
            }
        }

        // Update distribution
        self.distribution = self
            .amplitudes
            .iter()
            .map(|a| a.magnitude_squared())
            .collect();

        self.outcomes[selected].clone()
    }

    /// Deterministic collapse using seeded randomness
    fn deterministic_collapse(&self) -> usize {
        let mut rng = seeded_rng(self.seed);

        // Weighted random selection based on probabilities
        let mut cumulative = 0.0;
        let random_value: Float = rng.next_f64();

        for (i, prob) in self.distribution.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return i;
            }
        }

        // Fallback to last outcome
        self.distribution.len() - 1
    }

    /// Check if collapsed
    pub fn is_collapsed(&self) -> bool {
        self.collapsed.is_some()
    }

    /// Reset to superposition state
    pub fn reset(&mut self) {
        self.collapsed = None;
        self.coherence = 1.0;
        self.normalize();
    }

    /// Apply decoherence (reduce coherence)
    pub fn decohere(&mut self, amount: Float) {
        self.coherence = (self.coherence - amount).max(0.0);

        // Reduce off-diagonal terms (simplified)
        if self.coherence < 1.0 {
            for amplitude in &mut self.amplitudes {
                // Reduce imaginary part (decoherence)
                amplitude.imaginary *= self.coherence;
            }
            self.normalize();
        }
    }

    /// Interfere with another probability space
    pub fn interfere(&mut self, other: &ProbabilitySpace, phase_shift: Float) {
        if self.amplitudes.len() != other.amplitudes.len() {
            return;
        }

        let phase_factor = Complex::from_polar(1.0, phase_shift);

        for i in 0..self.amplitudes.len() {
            self.amplitudes[i] = self.amplitudes[i].add(other.amplitudes[i].mul(phase_factor));
        }

        self.normalize();
    }

    /// Get most probable outcome
    pub fn most_probable(&self) -> Option<&Outcome> {
        self.outcomes
            .iter()
            .max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
    }

    /// Get entropy (Shannon entropy)
    pub fn entropy(&self) -> Float {
        let mut entropy = 0.0;
        for prob in &self.distribution {
            if *prob > 0.0 {
                entropy -= prob * prob.log2();
            }
        }
        entropy
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Probability Space:\n\
             - Outcomes: {}\n\
             - Collapsed: {}\n\
             - Coherence: {:.2}\n\
             - Entropy: {:.2} bits\n\
             - Most Probable: {}",
            self.outcomes.len(),
            self.is_collapsed(),
            self.coherence,
            self.entropy(),
            self.most_probable()
                .map(|o| format!("{} ({:.2}%)", o.id, o.probability * 100.0))
                .unwrap_or_else(|| "None".to_string())
        )
    }
}

impl fmt::Display for ProbabilitySpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Simple seeded random number generator (for deterministic behavior)
struct SeededRng {
    state: u64,
}

impl SeededRng {
    fn new(seed: u64) -> Self {
        SeededRng {
            state: seed.wrapping_add(1),
        }
    }

    fn next(&mut self) -> u64 {
        // Simple XORShift algorithm
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_f64(&mut self) -> Float {
        (self.next() as Float) / (u64::MAX as Float)
    }
}

fn seeded_rng(seed: u64) -> SeededRng {
    SeededRng::new(seed)
}

// ============================================================================
// PROBABILITY DISTRIBUTION
// ============================================================================

/// Type of probability distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistributionType {
    /// Uniform distribution
    Uniform,
    /// Normal (Gaussian) distribution
    Normal,
    /// Exponential distribution
    Exponential,
    /// Poisson distribution
    Poisson,
    /// Custom distribution
    Custom,
}

/// Probability distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilityDistribution {
    pub distribution_type: DistributionType,
    pub probabilities: Vec<Float>,
    pub mean: Float,
    pub variance: Float,
}

impl ProbabilityDistribution {
    /// Create uniform distribution
    pub fn uniform(num_outcomes: usize) -> Self {
        let prob = 1.0 / num_outcomes as Float;
        ProbabilityDistribution {
            distribution_type: DistributionType::Uniform,
            probabilities: vec![prob; num_outcomes],
            mean: 0.5,
            variance: 1.0 / 12.0,
        }
    }

    /// Create normal distribution (approximated)
    pub fn normal(mean: Float, variance: Float, num_bins: usize) -> Self {
        let std_dev = variance.sqrt();
        let mut probabilities = Vec::with_capacity(num_bins);

        for i in 0..num_bins {
            let x = (i as Float - num_bins as Float / 2.0) / (num_bins as Float / 10.0);
            let prob = (-0.5 * ((x - mean) / std_dev).powi(2)).exp()
                / (std_dev * (2.0 * std::f64::consts::PI).sqrt());
            probabilities.push(prob);
        }

        // Normalize
        let total: Float = probabilities.iter().sum();
        for prob in &mut probabilities {
            *prob /= total;
        }

        ProbabilityDistribution {
            distribution_type: DistributionType::Normal,
            probabilities,
            mean,
            variance,
        }
    }

    /// Create custom distribution
    pub fn custom(probabilities: Vec<Float>) -> Self {
        let total: Float = probabilities.iter().sum();
        let normalized: Vec<Float> = probabilities.iter().map(|p| p / total).collect();

        let mean: Float = normalized
            .iter()
            .enumerate()
            .map(|(i, p)| i as Float * p)
            .sum();

        let variance: Float = normalized
            .iter()
            .enumerate()
            .map(|(i, p)| (i as Float - mean).powi(2) * p)
            .sum();

        ProbabilityDistribution {
            distribution_type: DistributionType::Custom,
            probabilities: normalized,
            mean,
            variance,
        }
    }

    /// Sample from distribution (deterministic)
    pub fn sample(&self, seed: u64) -> usize {
        let mut rng = seeded_rng(seed);
        let random_value: Float = rng.next_f64();

        let mut cumulative = 0.0;
        for (i, prob) in self.probabilities.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return i;
            }
        }

        self.probabilities.len() - 1
    }

    /// Get entropy
    pub fn entropy(&self) -> Float {
        let mut entropy = 0.0;
        for prob in &self.probabilities {
            if *prob > 0.0 {
                entropy -= prob * prob.log2();
            }
        }
        entropy
    }
}

// ============================================================================
// QUANTUM STATE (for quantum mechanics)
// ============================================================================

/// Quantum state (wave function)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// Probability amplitudes for basis states
    pub amplitudes: Vec<Complex>,
    /// Collapsed state
    pub collapsed: Option<usize>,
    /// Entanglement partners
    pub entangled_with: Vec<usize>,
    /// Random seed
    pub seed: u64,
}

impl QuantumState {
    /// Create quantum state with given basis states
    pub fn new(num_basis_states: usize, seed: u64) -> Self {
        let equal_amplitude = Complex::from_polar(1.0 / (num_basis_states as Float).sqrt(), 0.0);

        QuantumState {
            amplitudes: vec![equal_amplitude; num_basis_states],
            collapsed: None,
            entangled_with: Vec::new(),
            seed,
        }
    }

    /// Measure (collapse) quantum state
    pub fn measure(&mut self) -> usize {
        if let Some(state) = self.collapsed {
            return state;
        }

        let selected = self.deterministic_measurement();
        self.collapsed = Some(selected);

        // Collapse to selected state
        for i in 0..self.amplitudes.len() {
            if i == selected {
                self.amplitudes[i] = Complex::one();
            } else {
                self.amplitudes[i] = Complex::zero();
            }
        }

        selected
    }

    /// Deterministic measurement
    fn deterministic_measurement(&self) -> usize {
        let mut rng = seeded_rng(self.seed);

        let probabilities: Vec<Float> = self
            .amplitudes
            .iter()
            .map(|a| a.magnitude_squared())
            .collect();

        let mut cumulative = 0.0;
        let random_value: Float = rng.next_f64();

        for (i, prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return i;
            }
        }

        probabilities.len() - 1
    }

    /// Apply unitary operation
    pub fn apply_operation(&mut self, operation: &[Vec<Complex>]) {
        let mut new_amplitudes = vec![Complex::zero(); self.amplitudes.len()];

        for (i, new_amp) in new_amplitudes.iter_mut().enumerate() {
            for (j, &amp_j) in self.amplitudes.iter().enumerate() {
                *new_amp = new_amp.add(operation[i][j].mul(amp_j));
            }
        }

        self.amplitudes = new_amplitudes;
    }

    /// Check if entangled
    pub fn is_entangled(&self) -> bool {
        !self.entangled_with.is_empty()
    }

    /// Entangle with another state
    pub fn entangle_with(&mut self, other_id: usize) {
        if !self.entangled_with.contains(&other_id) {
            self.entangled_with.push(other_id);
        }
    }

    /// Get expectation value of observable
    pub fn expectation_value(&self, observable: &[Vec<Float>]) -> Float {
        let mut expectation = 0.0;

        for (i, amp_i) in self.amplitudes.iter().enumerate() {
            for (j, amp_j) in self.amplitudes.iter().enumerate() {
                expectation += observable[i][j] * amp_i.conjugate().mul(*amp_j).real;
            }
        }

        expectation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_creation() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c.real, 1.0);
        assert_eq!(c.imaginary, 2.0);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(1.0, 2.0);
        let conj = c.conjugate();
        assert_eq!(conj.real, 1.0);
        assert_eq!(conj.imaginary, -2.0);
    }

    #[test]
    fn test_complex_add() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(3.0, 4.0);
        let sum = c1.add(c2);
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imaginary, 6.0);
    }

    #[test]
    fn test_complex_multiply() {
        let c1 = Complex::new(1.0, 0.0);
        let c2 = Complex::new(0.0, 1.0);
        let product = c1.mul(c2);
        assert_eq!(product.real, 0.0);
        assert_eq!(product.imaginary, 1.0);
    }

    #[test]
    fn test_complex_normalize() {
        let c = Complex::new(3.0, 4.0);
        let normalized = c.normalize().unwrap();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_space_empty() {
        let space = ProbabilitySpace::empty();
        assert_eq!(space.amplitudes.len(), 0);
    }

    #[test]
    fn test_probability_space_with_outcomes() {
        let space = ProbabilitySpace::with_outcomes(3, 42);
        assert_eq!(space.amplitudes.len(), 3);
        assert_eq!(space.outcomes.len(), 3);
    }

    #[test]
    fn test_probability_space_normalize() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        space.normalize();

        let total_prob: Float = space.distribution.iter().sum();
        assert!((total_prob - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_space_collapse() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        let outcome = space.collapse(None);
        assert_eq!(outcome.id, space.collapsed.unwrap());
        assert!(space.is_collapsed());
    }

    #[test]
    fn test_probability_space_collapse_with_choice() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        let outcome = space.collapse(Some(1));
        assert_eq!(outcome.id, 1);
        assert_eq!(space.collapsed, Some(1));
    }

    #[test]
    fn test_probability_space_reset() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        space.collapse(None);
        space.reset();
        assert!(!space.is_collapsed());
    }

    #[test]
    fn test_probability_space_decohere() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        space.decohere(0.5);
        assert!(space.coherence < 1.0);
    }

    #[test]
    fn test_probability_space_entropy() {
        let space = ProbabilitySpace::with_outcomes(3, 42);
        let entropy = space.entropy();
        assert!(entropy > 0.0);
    }

    #[test]
    fn test_probability_distribution_uniform() {
        let dist = ProbabilityDistribution::uniform(4);
        assert_eq!(dist.probabilities.len(), 4);
        for prob in &dist.probabilities {
            assert!((prob - 0.25).abs() < 1e-10);
        }
    }

    #[test]
    fn test_probability_distribution_normal() {
        let dist = ProbabilityDistribution::normal(0.0, 1.0, 100);
        assert_eq!(dist.probabilities.len(), 100);
        assert!((dist.mean - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_distribution_custom() {
        let dist = ProbabilityDistribution::custom(vec![0.25, 0.5, 0.25]);
        assert_eq!(dist.probabilities.len(), 3);
        assert!((dist.probabilities[1] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_quantum_state_creation() {
        let state = QuantumState::new(3, 42);
        assert_eq!(state.amplitudes.len(), 3);
    }

    #[test]
    fn test_quantum_state_measure() {
        let mut state = QuantumState::new(3, 42);
        let measured = state.measure();
        assert!(measured < 3);
        assert!(state.collapsed.is_some());
    }

    #[test]
    fn test_quantum_state_entangle() {
        let mut state = QuantumState::new(3, 42);
        state.entangle_with(5);
        assert!(state.is_entangled());
        assert!(state.entangled_with.contains(&5));
    }

    #[test]
    fn test_seeded_rng_consistency() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(42);

        let val1 = rng1.next();
        let val2 = rng2.next();

        assert_eq!(val1, val2);
    }

    #[test]
    fn test_complex_from_polar() {
        let c = Complex::from_polar(1.0, std::f64::consts::PI / 2.0);
        assert!((c.real - 0.0).abs() < 1e-10);
        assert!((c.imaginary - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_to_polar() {
        let c = Complex::new(1.0, 1.0);
        let (mag, phase) = c.to_polar();
        assert!((mag - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((phase - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_space_most_probable() {
        let mut space = ProbabilitySpace::with_outcomes(3, 42);
        // Make outcome 0 more probable
        space.amplitudes[0] = Complex::from_polar(0.8, 0.0);
        space.amplitudes[1] = Complex::from_polar(0.4, 0.0);
        space.amplitudes[2] = Complex::from_polar(0.2, 0.0);
        space.normalize();

        let most_probable = space.most_probable();
        assert_eq!(most_probable.unwrap().id, 0);
    }

    #[test]
    fn test_quantum_state_expectation_value() {
        let mut state = QuantumState::new(2, 42);
        // Set to state |0⟩
        state.amplitudes[0] = Complex::one();
        state.amplitudes[1] = Complex::zero();

        // Observable = Pauli Z
        let observable = vec![vec![1.0, 0.0], vec![0.0, -1.0]];

        let expectation = state.expectation_value(&observable);
        assert!((expectation - 1.0).abs() < 1e-10);
    }
}
