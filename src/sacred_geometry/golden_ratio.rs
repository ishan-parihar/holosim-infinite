//! Golden Ratio (φ) Proportions
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 17-18:
//! - Golden ratio (φ) in proportions
//! - φ = (1+√5)/2 ≈ 1.6180339887498948482
//! - Proportions between layers use φ
//! - Entity sizes are in φ proportions
//!
//! The golden ratio appears throughout sacred geometry:
//! - φ = (1+√5)/2 ≈ 1.618
//! - 1/φ = φ - 1 ≈ 0.618
//! - φ² = φ + 1 ≈ 2.618
//! - Golden angle: 360°/φ² ≈ 137.5°

use crate::types::Float;

/// Golden Ratio Constant
pub const GOLDEN_RATIO: Float = 1.6180339887498948482;

/// Golden Ratio Inverse: 1/φ
pub const GOLDEN_RATIO_INVERSE: Float = 0.6180339887498948482;

/// Golden Angle in radians: 360°/φ²
pub const GOLDEN_ANGLE: Float = 2.39996322972865332;

/// Golden Proportion
///
/// Provides calculations and utilities for working with golden ratio proportions.
#[derive(Debug, Clone, Copy)]
pub struct GoldenProportion {
    phi: Float,
    phi_inverse: Float,
    phi_squared: Float,
}

impl GoldenProportion {
    /// Create a new golden proportion with default φ
    pub fn new() -> Self {
        GoldenProportion {
            phi: GOLDEN_RATIO,
            phi_inverse: GOLDEN_RATIO_INVERSE,
            phi_squared: GOLDEN_RATIO + 1.0,
        }
    }

    /// Get φ value
    pub fn phi(&self) -> Float {
        self.phi
    }

    /// Get 1/φ value
    pub fn phi_inverse(&self) -> Float {
        self.phi_inverse
    }

    /// Get φ² value
    pub fn phi_squared(&self) -> Float {
        self.phi_squared
    }

    /// Divide a length by φ to get the smaller golden section
    pub fn golden_section(&self, length: Float) -> Float {
        length / self.phi
    }

    /// Multiply a length by φ to get the larger golden section
    pub fn golden_section_large(&self, length: Float) -> Float {
        length * self.phi
    }

    /// Check if two values are in golden proportion
    pub fn is_golden_proportion(&self, a: Float, b: Float, tolerance: Float) -> bool {
        let ratio = a.max(b) / a.min(b);
        (ratio - self.phi).abs() < tolerance
    }

    /// Calculate the golden ratio of two values
    pub fn calculate_ratio(&self, a: Float, b: Float) -> Float {
        if b == 0.0 {
            0.0
        } else {
            a / b
        }
    }

    /// Apply golden ratio to a value (multiply by φ)
    pub fn apply_phi(&self, value: Float) -> Float {
        value * self.phi
    }

    /// Apply inverse golden ratio to a value (multiply by 1/φ)
    pub fn apply_phi_inverse(&self, value: Float) -> Float {
        value * self.phi_inverse
    }

    /// Calculate golden angle for a given index
    pub fn golden_angle_for_index(&self, index: usize) -> Float {
        index as Float * GOLDEN_ANGLE
    }
}

impl Default for GoldenProportion {
    fn default() -> Self {
        Self::new()
    }
}

/// Golden Rectangle
///
/// A rectangle whose side lengths are in the golden ratio.
#[derive(Debug, Clone)]
pub struct GoldenRectangle {
    width: Float,
    height: Float,
    phi: Float,
}

impl GoldenRectangle {
    /// Create a golden rectangle with specified width
    pub fn from_width(width: Float) -> Self {
        let phi = GOLDEN_RATIO;
        let height = width / phi;
        GoldenRectangle { width, height, phi }
    }

    /// Create a golden rectangle with specified height
    pub fn from_height(height: Float) -> Self {
        let phi = GOLDEN_RATIO;
        let width = height * phi;
        GoldenRectangle { width, height, phi }
    }

    /// Create a golden rectangle with specified short side
    pub fn from_short_side(short: Float) -> Self {
        let phi = GOLDEN_RATIO;
        let long = short * phi;
        GoldenRectangle {
            width: long,
            height: short,
            phi,
        }
    }

    /// Get width
    pub fn width(&self) -> Float {
        self.width
    }

    /// Get height
    pub fn height(&self) -> Float {
        self.height
    }

    /// Get golden ratio
    pub fn phi(&self) -> Float {
        self.phi
    }

    /// Get area
    pub fn area(&self) -> Float {
        self.width * self.height
    }

    /// Get diagonal length
    pub fn diagonal(&self) -> Float {
        (self.width.powi(2) + self.height.powi(2)).sqrt()
    }

    /// Check if this is a valid golden rectangle
    pub fn is_valid(&self, tolerance: Float) -> bool {
        let ratio = self.width / self.height;
        (ratio - self.phi).abs() < tolerance
    }

    /// Subdivide into a square and a smaller golden rectangle
    pub fn subdivide(&self) -> (GoldenRectangle, Float) {
        let short = self.height;
        let long = self.width;
        let square_side = short;
        let remaining = long - short;

        let new_rectangle = GoldenRectangle::from_short_side(remaining);
        (new_rectangle, square_side)
    }
}

/// Golden Angle
///
/// The angle that divides a circle in golden ratio: ≈ 137.5°
#[derive(Debug, Clone, Copy)]
pub struct GoldenAngle {
    angle_radians: Float,
    angle_degrees: Float,
}

impl GoldenAngle {
    /// Create a new golden angle
    pub fn new() -> Self {
        GoldenAngle {
            angle_radians: GOLDEN_ANGLE,
            angle_degrees: GOLDEN_ANGLE.to_degrees(),
        }
    }

    /// Get angle in radians
    pub fn radians(&self) -> Float {
        self.angle_radians
    }

    /// Get angle in degrees
    pub fn degrees(&self) -> Float {
        self.angle_degrees
    }

    /// Calculate point on circle using golden angle
    pub fn point_on_circle(&self, radius: Float, index: usize) -> (Float, Float) {
        let angle = index as Float * self.angle_radians;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        (x, y)
    }

    /// Generate multiple points on circle using golden angle
    pub fn points_on_circle(&self, radius: Float, count: usize) -> Vec<(Float, Float)> {
        (0..count)
            .map(|i| self.point_on_circle(radius, i))
            .collect()
    }
}

impl Default for GoldenAngle {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate golden ratio (for convenience)
pub fn golden_ratio() -> Float {
    GOLDEN_RATIO
}

/// Calculate golden section of a length
pub fn golden_section(length: Float) -> Float {
    length / GOLDEN_RATIO
}

/// Check if two values are in golden proportion
pub fn is_golden_proportion(a: Float, b: Float, tolerance: Float) -> bool {
    let ratio = a.max(b) / a.min(b);
    (ratio - GOLDEN_RATIO).abs() < tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_proportion_values() {
        let gp = GoldenProportion::new();

        // φ × 1/φ should equal 1
        assert!((gp.phi() * gp.phi_inverse() - 1.0).abs() < 1e-10);

        // φ - 1 should equal 1/φ
        assert!((gp.phi() - 1.0 - gp.phi_inverse()).abs() < 1e-10);

        // φ² should equal φ + 1
        assert!((gp.phi_squared() - (gp.phi() + 1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_golden_rectangle() {
        let rect = GoldenRectangle::from_width(10.0);

        // Should be valid golden rectangle
        assert!(rect.is_valid(0.01));

        // Height should be width / φ
        assert!((rect.height() - (rect.width() / GOLDEN_RATIO)).abs() < 0.01);
    }

    #[test]
    fn test_golden_rectangle_from_height() {
        let rect = GoldenRectangle::from_height(10.0);

        // Should be valid golden rectangle
        assert!(rect.is_valid(0.01));

        // Width should be height × φ
        assert!((rect.width() - (rect.height() * GOLDEN_RATIO)).abs() < 0.01);
    }

    #[test]
    fn test_golden_rectangle_subdivision() {
        let rect = GoldenRectangle::from_width(10.0);
        let (new_rect, square_side) = rect.subdivide();

        // Square side should equal original height
        assert!((square_side - rect.height()).abs() < 0.01);

        // New rectangle should also be golden
        assert!(new_rect.is_valid(0.01));
    }

    #[test]
    fn test_golden_angle() {
        let ga = GoldenAngle::new();

        // Golden angle should be ≈ 137.5°
        assert!((ga.degrees() - 137.5).abs() < 0.1);

        // In radians, should be ≈ 2.4
        assert!((ga.radians() - 2.4).abs() < 0.01);
    }

    #[test]
    fn test_golden_angle_points() {
        let ga = GoldenAngle::new();
        let points = ga.points_on_circle(1.0, 10);

        // Should have 10 points
        assert_eq!(points.len(), 10);

        // All points should be on unit circle
        for (x, y) in points {
            assert!((x.powi(2) + y.powi(2) - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_is_golden_proportion() {
        // φ and 1 are in golden ratio
        assert!(is_golden_proportion(GOLDEN_RATIO, 1.0, 0.01));

        // 1.618 and 1 are in golden ratio
        assert!(is_golden_proportion(1.618, 1.0, 0.01));

        // φ² and φ are in golden ratio
        assert!(is_golden_proportion(GOLDEN_RATIO + 1.0, GOLDEN_RATIO, 0.01));

        // 2 and 1 are NOT in golden ratio
        assert!(!is_golden_proportion(2.0, 1.0, 0.01));
    }

    #[test]
    fn test_golden_section() {
        let length = 10.0;
        let section = golden_section(length);

        // Should be length / φ
        assert!((section - (length / GOLDEN_RATIO)).abs() < 0.01);

        // Should be less than original length
        assert!(section < length);
    }
}
