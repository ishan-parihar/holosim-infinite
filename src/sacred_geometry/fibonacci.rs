//! Fibonacci Sequence and Growth Patterns
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 17-18:
//! - Fibonacci sequence in growth patterns
//! - Entity populations follow Fibonacci sequences
//! - Growth over time follows Fibonacci sequences
//!
//! The Fibonacci sequence appears throughout nature and sacred geometry:
//! - 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144...
//! - Each number is the sum of the two preceding numbers
//! - Ratio of consecutive numbers approaches φ (golden ratio)

use crate::types::Float;

/// Fibonacci Sequence
///
/// Stores the Fibonacci sequence up to a specified generation.
/// Uses lazy evaluation to generate numbers on demand.
#[derive(Debug, Clone)]
pub struct FibonacciSequence {
    sequence: Vec<u64>,
    max_generation: usize,
}

impl FibonacciSequence {
    /// Create a new Fibonacci sequence up to max_generation
    pub fn new(max_generation: usize) -> Self {
        let mut seq = FibonacciSequence {
            sequence: Vec::with_capacity(max_generation + 1),
            max_generation,
        };
        seq.generate();
        seq
    }

    /// Generate the Fibonacci sequence
    fn generate(&mut self) {
        if self.sequence.is_empty() {
            self.sequence.push(0); // F(0) = 0
        }

        while self.sequence.len() <= self.max_generation {
            let len = self.sequence.len();
            if len == 1 {
                self.sequence.push(1); // F(1) = 1
            } else {
                let next = self.sequence[len - 1] + self.sequence[len - 2];
                self.sequence.push(next);
            }
        }
    }

    /// Get the Fibonacci number at generation n
    pub fn get(&self, n: u64) -> u64 {
        let n = n as usize;
        if n < self.sequence.len() {
            self.sequence[n]
        } else {
            // Calculate on demand using Binet's formula for large n
            self.calculate_binet(n)
        }
    }

    /// Calculate Fibonacci number using Binet's formula
    fn calculate_binet(&self, n: usize) -> u64 {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let psi = (1.0 - 5.0_f64.sqrt()) / 2.0;

        let result = (phi.powi(n as i32) - psi.powi(n as i32)) / 5.0_f64.sqrt();
        result.round() as u64
    }

    /// Get the sequence as a slice
    pub fn as_slice(&self) -> &[u64] {
        &self.sequence
    }

    /// Get the ratio of F(n) to F(n-1)
    pub fn ratio(&self, n: u64) -> Float {
        if n == 0 {
            return 0.0;
        }
        let a = self.get(n) as Float;
        let b = self.get(n - 1) as Float;
        if b == 0.0 {
            0.0
        } else {
            a / b
        }
    }

    /// Get the length of the sequence
    pub fn len(&self) -> usize {
        self.sequence.len()
    }
}

/// Fibonacci Growth
///
/// Models growth patterns that follow the Fibonacci sequence.
/// Used for entity population growth, density transitions, and collective formation.
#[derive(Debug, Clone)]
pub struct FibonacciGrowth {
    generation: u64,
    population: u64,
    growth_rate: Float,
}

impl FibonacciGrowth {
    /// Create a new Fibonacci growth model
    pub fn new(initial_generation: u64) -> Self {
        let mut growth = FibonacciGrowth {
            generation: initial_generation,
            population: 0,
            growth_rate: 1.618, // φ
        };
        growth.update_population();
        growth
    }

    /// Update population based on current generation
    fn update_population(&mut self) {
        let seq = FibonacciSequence::new(self.generation as usize);
        self.population = seq.get(self.generation);
    }

    /// Advance to next generation
    pub fn advance(&mut self) {
        self.generation += 1;
        self.update_population();
    }

    /// Get current population
    pub fn population(&self) -> u64 {
        self.population
    }

    /// Get current generation
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Calculate growth at a specific generation
    pub fn growth_at_generation(&self, generation: u64) -> Float {
        let seq = FibonacciSequence::new(generation as usize);
        let fib = seq.get(generation) as Float;
        fib * self.growth_rate.powi(2 - generation as i32)
    }

    /// Get growth rate (golden ratio)
    pub fn growth_rate(&self) -> Float {
        self.growth_rate
    }
}

/// Fibonacci Spiral
///
/// A spiral that follows the Fibonacci sequence for its growth rate.
/// The radius increases according to Fibonacci numbers.
#[derive(Debug, Clone)]
pub struct FibonacciSpiral {
    points: Vec<(Float, Float)>,
    generations: usize,
}

impl FibonacciSpiral {
    /// Create a new Fibonacci spiral with specified number of generations
    pub fn new(generations: usize) -> Self {
        let mut spiral = FibonacciSpiral {
            points: Vec::with_capacity(generations),
            generations,
        };
        spiral.generate();
        spiral
    }

    /// Generate the spiral points
    fn generate(&mut self) {
        let seq = FibonacciSequence::new(self.generations);
        let mut angle: Float = 0.0;
        let angle_increment = std::f64::consts::PI / 2.0; // 90 degrees

        for i in 0..=self.generations {
            let radius = seq.get(i as u64) as Float;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            self.points.push((x, y));
            angle += angle_increment;
        }
    }

    /// Get the spiral points
    pub fn points(&self) -> &[(Float, Float)] {
        &self.points
    }

    /// Get a point at a specific generation
    pub fn point_at(&self, generation: usize) -> Option<(Float, Float)> {
        self.points.get(generation).copied()
    }
}

/// Get the nth Fibonacci number
pub fn fibonacci_number(n: u64) -> u64 {
    let seq = FibonacciSequence::new(n as usize);
    seq.get(n)
}

/// Get a Fibonacci sequence up to n
pub fn fibonacci_sequence(n: usize) -> Vec<u64> {
    let seq = FibonacciSequence::new(n);
    seq.as_slice().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence_first_10() {
        let seq = FibonacciSequence::new(10);

        assert_eq!(seq.get(0), 0);
        assert_eq!(seq.get(1), 1);
        assert_eq!(seq.get(2), 1);
        assert_eq!(seq.get(3), 2);
        assert_eq!(seq.get(4), 3);
        assert_eq!(seq.get(5), 5);
        assert_eq!(seq.get(6), 8);
        assert_eq!(seq.get(7), 13);
        assert_eq!(seq.get(8), 21);
        assert_eq!(seq.get(9), 34);
        assert_eq!(seq.get(10), 55);
    }

    #[test]
    fn test_fibonacci_ratio_converges_to_phi() {
        let seq = FibonacciSequence::new(20);

        // Ratio should approach φ = 1.618...
        let ratio_10 = seq.ratio(10);
        let ratio_20 = seq.ratio(20);

        // Higher generation should be closer to φ
        assert!((ratio_20 - 1.618).abs() < (ratio_10 - 1.618).abs());
    }

    #[test]
    fn test_fibonacci_growth() {
        let mut growth = FibonacciGrowth::new(0);

        // Initial population should be F(0) = 0
        assert_eq!(growth.population(), 0);

        growth.advance();
        assert_eq!(growth.population(), 1);

        growth.advance();
        assert_eq!(growth.population(), 1);

        growth.advance();
        assert_eq!(growth.population(), 2);

        growth.advance();
        assert_eq!(growth.population(), 3);

        growth.advance();
        assert_eq!(growth.population(), 5);
    }

    #[test]
    fn test_fibonacci_spiral_points() {
        let spiral = FibonacciSpiral::new(5);

        // Should have 6 points (0 through 5)
        assert_eq!(spiral.points().len(), 6);

        // First point should be at origin
        assert_eq!(spiral.point_at(0), Some((0.0, 0.0)));
    }

    #[test]
    fn test_fibonacci_number_function() {
        assert_eq!(fibonacci_number(0), 0);
        assert_eq!(fibonacci_number(1), 1);
        assert_eq!(fibonacci_number(2), 1);
        assert_eq!(fibonacci_number(10), 55);
        assert_eq!(fibonacci_number(20), 6765);
    }

    #[test]
    fn test_fibonacci_sequence_function() {
        let seq = fibonacci_sequence(10);

        assert_eq!(seq.len(), 11); // 0 through 10
        assert_eq!(seq[10], 55);
    }
}
