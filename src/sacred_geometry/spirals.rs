//! Spiral Patterns
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 19-20:
//! - Spiral patterns in galaxy formation
//! - φ (1.618...) in spiral growth
//! - Golden spiral in spectrum trajectories
//! - Fibonacci spiral in entity movement
//!
//! Spiral patterns appear throughout sacred geometry:
//! - Golden Spiral: r = a × φ^(θ/90°)
//! - Fibonacci Spiral: Approximation using quarter circles
//! - Archimedean Spiral: r = a + bθ
//! - Logarithmic Spiral: r = a × e^(bθ)

use crate::types::Float;

/// Spiral Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiralType {
    /// Golden Spiral (growth factor = φ)
    Golden,
    /// Fibonacci Spiral (quarter circles with Fibonacci radii)
    Fibonacci,
    /// Archimedean Spiral (linear growth)
    Archimedean,
    /// Logarithmic Spiral (exponential growth)
    Logarithmic,
}

impl SpiralType {
    /// Get name of spiral type
    pub fn name(&self) -> &'static str {
        match self {
            SpiralType::Golden => "Golden Spiral",
            SpiralType::Fibonacci => "Fibonacci Spiral",
            SpiralType::Archimedean => "Archimedean Spiral",
            SpiralType::Logarithmic => "Logarithmic Spiral",
        }
    }
}

/// Spiral Point
///
/// A single point on a spiral path.
#[derive(Debug, Clone)]
pub struct SpiralPoint {
    x: Float,
    y: Float,
    radius: Float,
    angle: Float,
    index: usize,
}

impl SpiralPoint {
    /// Create a new spiral point
    pub fn new(x: Float, y: Float, radius: Float, angle: Float, index: usize) -> Self {
        SpiralPoint {
            x,
            y,
            radius,
            angle,
            index,
        }
    }

    /// Get x coordinate
    pub fn x(&self) -> Float {
        self.x
    }

    /// Get y coordinate
    pub fn y(&self) -> Float {
        self.y
    }

    /// Get radius from center
    pub fn radius(&self) -> Float {
        self.radius
    }

    /// Get angle in radians
    pub fn angle(&self) -> Float {
        self.angle
    }

    /// Get point index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get as tuple
    pub fn as_tuple(&self) -> (Float, Float) {
        (self.x, self.y)
    }
}

/// Spiral Pattern
///
/// A complete spiral with all calculated points.
#[derive(Debug, Clone)]
pub struct SpiralPattern {
    spiral_type: SpiralType,
    points: Vec<SpiralPoint>,
    center_x: Float,
    center_y: Float,
    total_turns: Float,
    growth_factor: Float,
}

impl SpiralPattern {
    /// Create a golden spiral
    pub fn golden_spiral(center_x: Float, center_y: Float, turns: Float) -> Self {
        let phi: Float = 1.6180339887498948482;
        let mut points = Vec::new();
        let steps = (turns * 100.0) as usize;
        
        for i in 0..steps {
            let t = i as Float / steps as Float;
            let angle = t * turns * 2.0 * std::f64::consts::PI;
            let radius = 0.1 * phi.powf(angle / (std::f64::consts::PI / 2.0));
            
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            
            points.push(SpiralPoint::new(x, y, radius, angle, i));
        }
        
        SpiralPattern {
            spiral_type: SpiralType::Golden,
            points,
            center_x,
            center_y,
            total_turns: turns,
            growth_factor: phi,
        }
    }

    /// Create a Fibonacci spiral
    pub fn fibonacci_spiral(center_x: Float, center_y: Float, generations: usize) -> Self {
        let mut points = Vec::new();
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        
        for i in 0..generations {
            let radius = b as Float * 0.1;
            let angle = i as Float * std::f64::consts::PI / 2.0;
            
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            
            points.push(SpiralPoint::new(x, y, radius, angle, i));
            
            let temp = a + b;
            a = b;
            b = temp;
        }
        
        SpiralPattern {
            spiral_type: SpiralType::Fibonacci,
            points,
            center_x,
            center_y,
            total_turns: generations as Float / 4.0,
            growth_factor: 1.618,
        }
    }

    /// Create an Archimedean spiral
    pub fn archimedean_spiral(center_x: Float, center_y: Float, turns: Float, a: Float, b: Float) -> Self {
        let mut points = Vec::new();
        let steps = (turns * 100.0) as usize;
        
        for i in 0..steps {
            let t = i as Float / steps as Float;
            let angle = t * turns * 2.0 * std::f64::consts::PI;
            let radius = a + b * angle;
            
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            
            points.push(SpiralPoint::new(x, y, radius, angle, i));
        }
        
        SpiralPattern {
            spiral_type: SpiralType::Archimedean,
            points,
            center_x,
            center_y,
            total_turns: turns,
            growth_factor: b,
        }
    }

    /// Create a logarithmic spiral
    pub fn logarithmic_spiral(center_x: Float, center_y: Float, turns: Float, a: Float, b: Float) -> Self {
        let mut points = Vec::new();
        let steps = (turns * 100.0) as usize;
        
        for i in 0..steps {
            let t = i as Float / steps as Float;
            let angle = t * turns * 2.0 * std::f64::consts::PI;
            let radius = a * (b * angle).exp();
            
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            
            points.push(SpiralPoint::new(x, y, radius, angle, i));
        }
        
        SpiralPattern {
            spiral_type: SpiralType::Logarithmic,
            points,
            center_x,
            center_y,
            total_turns: turns,
            growth_factor: b.exp(),
        }
    }

    /// Get spiral type
    pub fn spiral_type(&self) -> SpiralType {
        self.spiral_type
    }

    /// Get points
    pub fn points(&self) -> &[SpiralPoint] {
        &self.points
    }

    /// Get center x
    pub fn center_x(&self) -> Float {
        self.center_x
    }

    /// Get center y
    pub fn center_y(&self) -> Float {
        self.center_y
    }

    /// Get total turns
    pub fn total_turns(&self) -> Float {
        self.total_turns
    }

    /// Get growth factor
    pub fn growth_factor(&self) -> Float {
        self.growth_factor
    }

    /// Get point at index
    pub fn point_at(&self, index: usize) -> Option<&SpiralPoint> {
        self.points.get(index)
    }

    /// Get last point
    pub fn last_point(&self) -> Option<&SpiralPoint> {
        self.points.last()
    }
}

/// Golden Spiral
///
/// A spiral that grows by the golden ratio φ for each quarter turn.
#[derive(Debug, Clone)]
pub struct GoldenSpiral {
    pattern: SpiralPattern,
    phi: Float,
}

impl GoldenSpiral {
    /// Create a new golden spiral
    pub fn new(center_x: Float, center_y: Float, turns: Float) -> Self {
        let pattern = SpiralPattern::golden_spiral(center_x, center_y, turns);
        GoldenSpiral {
            pattern,
            phi: 1.6180339887498948482,
        }
    }

    /// Get pattern
    pub fn pattern(&self) -> &SpiralPattern {
        &self.pattern
    }

    /// Get φ value
    pub fn phi(&self) -> Float {
        self.phi
    }

    /// Calculate radius at angle
    pub fn radius_at_angle(&self, angle: Float) -> Float {
        0.1 * self.phi.powf(angle / (std::f64::consts::PI / 2.0))
    }
}

/// Fibonacci Spiral (alias for FibSpiral)
pub type FibSpiral = FibonacciSpiral;

/// Fibonacci Spiral
///
/// A spiral approximated using quarter circles with Fibonacci radii.
#[derive(Debug, Clone)]
pub struct FibonacciSpiral {
    pattern: SpiralPattern,
    sequence: Vec<u64>,
}

impl FibonacciSpiral {
    /// Create a new Fibonacci spiral
    pub fn new(center_x: Float, center_y: Float, generations: usize) -> Self {
        let pattern = SpiralPattern::fibonacci_spiral(center_x, center_y, generations);
        let mut sequence = vec![0, 1];
        for _ in 2..=generations {
            let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
            sequence.push(next);
        }
        
        FibonacciSpiral {
            pattern,
            sequence,
        }
    }

    /// Get pattern
    pub fn pattern(&self) -> &SpiralPattern {
        &self.pattern
    }

    /// Get Fibonacci sequence
    pub fn sequence(&self) -> &[u64] {
        &self.sequence
    }

    /// Get Fibonacci number at generation
    pub fn fibonacci_number(&self, generation: usize) -> u64 {
        if generation < self.sequence.len() {
            self.sequence[generation]
        } else {
            0
        }
    }
}

/// Archimedean Spiral
///
/// A spiral with linear growth: r = a + bθ
#[derive(Debug, Clone)]
pub struct ArchimedeanSpiral {
    pattern: SpiralPattern,
    a: Float,
    b: Float,
}

impl ArchimedeanSpiral {
    /// Create a new Archimedean spiral
    pub fn new(center_x: Float, center_y: Float, turns: Float, a: Float, b: Float) -> Self {
        let pattern = SpiralPattern::archimedean_spiral(center_x, center_y, turns, a, b);
        ArchimedeanSpiral {
            pattern,
            a,
            b,
        }
    }

    /// Get pattern
    pub fn pattern(&self) -> &SpiralPattern {
        &self.pattern
    }

    /// Get a parameter
    pub fn a(&self) -> Float {
        self.a
    }

    /// Get b parameter
    pub fn b(&self) -> Float {
        self.b
    }

    /// Calculate radius at angle
    pub fn radius_at_angle(&self, angle: Float) -> Float {
        self.a + self.b * angle
    }
}

/// Logarithmic Spiral
///
/// A spiral with exponential growth: r = a × e^(bθ)
#[derive(Debug, Clone)]
pub struct LogarithmicSpiral {
    pattern: SpiralPattern,
    a: Float,
    b: Float,
}

impl LogarithmicSpiral {
    /// Create a new logarithmic spiral
    pub fn new(center_x: Float, center_y: Float, turns: Float, a: Float, b: Float) -> Self {
        let pattern = SpiralPattern::logarithmic_spiral(center_x, center_y, turns, a, b);
        LogarithmicSpiral {
            pattern,
            a,
            b,
        }
    }

    /// Get pattern
    pub fn pattern(&self) -> &SpiralPattern {
        &self.pattern
    }

    /// Get a parameter
    pub fn a(&self) -> Float {
        self.a
    }

    /// Get b parameter
    pub fn b(&self) -> Float {
        self.b
    }

    /// Calculate radius at angle
    pub fn radius_at_angle(&self, angle: Float) -> Float {
        self.a * (self.b * angle).exp()
    }
}

/// Calculate a point on a spiral
///
/// Generic function for calculating a point on any type of spiral.
pub fn spiral_point(
    spiral_type: SpiralType,
    center_x: Float,
    center_y: Float,
    angle: Float,
    params: (Float, Float),
) -> (Float, Float) {
    let (a, b) = params;
    let radius = match spiral_type {
        SpiralType::Golden => {
            let phi: Float = 1.6180339887498948482;
            a * phi.powf(angle / (std::f64::consts::PI / 2.0))
        }
        SpiralType::Archimedean => a + b * angle,
        SpiralType::Logarithmic => a * (b * angle).exp(),
        SpiralType::Fibonacci => {
            // Approximate using golden ratio
            let phi: Float = 1.6180339887498948482;
            a * phi.powf(angle / (std::f64::consts::PI / 2.0))
        }
    };
    
    let x = center_x + radius * angle.cos();
    let y = center_y + radius * angle.sin();
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_spiral_creation() {
        let spiral = SpiralPattern::golden_spiral(0.0, 0.0, 3.0);
        
        assert_eq!(spiral.spiral_type(), SpiralType::Golden);
        assert!(!spiral.points().is_empty());
        assert_eq!(spiral.center_x(), 0.0);
        assert_eq!(spiral.center_y(), 0.0);
        assert_eq!(spiral.total_turns(), 3.0);
        assert!((spiral.growth_factor() - 1.618).abs() < 0.01);
    }

    #[test]
    fn test_fibonacci_spiral_creation() {
        let spiral = SpiralPattern::fibonacci_spiral(0.0, 0.0, 10);
        
        assert_eq!(spiral.spiral_type(), SpiralType::Fibonacci);
        assert_eq!(spiral.points().len(), 10);
    }

    #[test]
    fn test_archimedean_spiral_creation() {
        let spiral = SpiralPattern::archimedean_spiral(0.0, 0.0, 2.0, 1.0, 0.5);
        
        assert_eq!(spiral.spiral_type(), SpiralType::Archimedean);
        assert_eq!(spiral.growth_factor(), 0.5);
    }

    #[test]
    fn test_logarithmic_spiral_creation() {
        let spiral = SpiralPattern::logarithmic_spiral(0.0, 0.0, 2.0, 1.0, 0.1);
        
        assert_eq!(spiral.spiral_type(), SpiralType::Logarithmic);
    }

    #[test]
    fn test_spiral_points_on_path() {
        let spiral = SpiralPattern::golden_spiral(0.0, 0.0, 1.0);
        
        // Points should spiral outward
        let first = spiral.point_at(0).unwrap();
        let last = spiral.last_point().unwrap();
        
        assert!(last.radius() > first.radius());
    }

    #[test]
    fn test_golden_spiral_struct() {
        let golden = GoldenSpiral::new(0.0, 0.0, 2.0);
        
        assert!((golden.phi() - 1.618).abs() < 0.01);
        assert!(!golden.pattern().points().is_empty());
    }

    #[test]
    fn test_fibonacci_spiral_struct() {
        let fib = FibonacciSpiral::new(0.0, 0.0, 10);
        
        assert_eq!(fib.sequence().len(), 11);
        assert_eq!(fib.fibonacci_number(0), 0);
        assert_eq!(fib.fibonacci_number(1), 1);
        assert_eq!(fib.fibonacci_number(10), 55);
    }

    #[test]
    fn test_spiral_point_function() {
        let (x, y) = spiral_point(SpiralType::Golden, 0.0, 0.0, std::f64::consts::PI, (1.0, 0.0));
        
        // Point should be on the spiral
        assert!(!x.is_nan());
        assert!(!y.is_nan());
    }

    #[test]
    fn test_spiral_type_names() {
        assert_eq!(SpiralType::Golden.name(), "Golden Spiral");
        assert_eq!(SpiralType::Fibonacci.name(), "Fibonacci Spiral");
        assert_eq!(SpiralType::Archimedean.name(), "Archimedean Spiral");
        assert_eq!(SpiralType::Logarithmic.name(), "Logarithmic Spiral");
    }
}
