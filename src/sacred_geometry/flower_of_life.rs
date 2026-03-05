//! Flower of Life
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 19-20:
//! - Flower of Life in layer interactions
//! - Overlapping circles pattern
//! - Sacred geometry of creation
//!
//! The Flower of Life is a sacred geometric pattern consisting of multiple
//! evenly-spaced, overlapping circles arranged in a flower-like pattern with
//! six-fold symmetry. It represents:
//! - The fundamental pattern of creation
//! - The interconnectedness of all life
//! - The unity of consciousness
//! - The holographic nature of reality

use crate::types::Float;

/// Flower of Life
///
/// A pattern of overlapping circles arranged in a flower-like pattern.
#[derive(Debug, Clone)]
pub struct FlowerOfLife {
    center: (Float, Float),
    radius: Float,
    circles: Vec<((Float, Float), Float)>,
    layers: usize,
}

impl FlowerOfLife {
    /// Create a new Flower of Life pattern
    ///
    /// # Arguments
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    /// * `radius` - Radius of each circle
    /// * `layers` - Number of layers (rings of circles)
    pub fn new(center_x: Float, center_y: Float, radius: Float, layers: usize) -> Self {
        let mut flower = FlowerOfLife {
            center: (center_x, center_y),
            radius,
            circles: Vec::new(),
            layers,
        };

        flower.generate();
        flower
    }

    /// Generate the Flower of Life pattern
    fn generate(&mut self) {
        // Add center circle
        self.circles.push((self.center, self.radius));

        // Add circles in layers around center
        for layer in 1..=self.layers {
            self.add_layer(layer);
        }
    }

    /// Add a layer of circles
    fn add_layer(&mut self, layer: usize) {
        let circles_in_layer = if layer == 1 { 6 } else { layer * 6 };
        let angle_step = 2.0 * std::f64::consts::PI / circles_in_layer as Float;
        let distance = layer as Float * self.radius;

        for i in 0..circles_in_layer {
            let angle = i as Float * angle_step;
            let x = self.center.0 + distance * angle.cos();
            let y = self.center.1 + distance * angle.sin();
            self.circles.push(((x, y), self.radius));
        }
    }

    /// Get center
    pub fn center(&self) -> (Float, Float) {
        self.center
    }

    /// Get radius
    pub fn radius(&self) -> Float {
        self.radius
    }

    /// Get circles
    pub fn circles(&self) -> &[((Float, Float), Float)] {
        &self.circles
    }

    /// Get number of layers
    pub fn layers(&self) -> usize {
        self.layers
    }

    /// Get number of circles
    pub fn circle_count(&self) -> usize {
        self.circles.len()
    }

    /// Get circle at index
    pub fn circle_at(&self, index: usize) -> Option<((Float, Float), Float)> {
        self.circles.get(index).copied()
    }

    /// Get connections between circles (overlaps)
    pub fn connections(&self) -> Vec<(usize, usize)> {
        let mut connections = Vec::new();

        for i in 0..self.circles.len() {
            for j in (i + 1)..self.circles.len() {
                let (center_a, radius_a) = self.circles[i];
                let (center_b, radius_b) = self.circles[j];

                let distance =
                    ((center_b.0 - center_a.0).powi(2) + (center_b.1 - center_a.1).powi(2)).sqrt();

                // Circles overlap if distance < sum of radii
                if distance < radius_a + radius_b && distance > 0.001 {
                    connections.push((i, j));
                }
            }
        }

        connections
    }
}

/// Flower Pattern
///
/// A more complex flower pattern with multiple layers and variations.
#[derive(Debug, Clone)]
pub struct FlowerPattern {
    pattern_type: FlowerType,
    center: (Float, Float),
    radius: Float,
    circles: Vec<((Float, Float), Float)>,
    connections: Vec<(usize, usize)>,
    layers: Vec<FlowerLayer>,
}

impl FlowerPattern {
    /// Create a new flower pattern
    pub fn new(
        pattern_type: FlowerType,
        center: (Float, Float),
        radius: Float,
        layers: usize,
    ) -> Self {
        let mut pattern = FlowerPattern {
            pattern_type,
            center,
            radius,
            circles: Vec::new(),
            connections: Vec::new(),
            layers: Vec::new(),
        };

        pattern.generate(layers);
        pattern
    }

    /// Generate the flower pattern
    fn generate(&mut self, layers: usize) {
        match self.pattern_type {
            FlowerType::Standard => self.generate_standard(layers),
            FlowerType::SeedOfLife => self.generate_seed_of_life(),
            FlowerType::FruitOfLife => self.generate_fruit_of_life(),
            FlowerType::MetatronsCube => self.generate_metatrons_cube(),
        }

        // Calculate connections
        self.connections = self.calculate_connections();

        // Create layers
        self.create_layers(layers);
    }

    /// Generate standard Flower of Life
    fn generate_standard(&mut self, layers: usize) {
        let flower = FlowerOfLife::new(self.center.0, self.center.1, self.radius, layers);
        self.circles = flower.circles().to_vec();
    }

    /// Generate Seed of Life (center circle + 6 surrounding circles)
    fn generate_seed_of_life(&mut self) {
        let flower = FlowerOfLife::new(self.center.0, self.center.1, self.radius, 1);
        self.circles = flower.circles().to_vec();
    }

    /// Generate Fruit of Life (Seed of Life + additional circles)
    fn generate_fruit_of_life(&mut self) {
        let flower = FlowerOfLife::new(self.center.0, self.center.1, self.radius, 2);
        self.circles = flower.circles().to_vec();

        // Add 13 additional circles for complete Fruit of Life
        let angle_step = 2.0 * std::f64::consts::PI / 6.0;
        for i in 0..6 {
            let angle = i as Float * angle_step;
            let x = self.center.0 + 2.0 * self.radius * angle.cos();
            let y = self.center.1 + 2.0 * self.radius * angle.sin();
            self.circles.push(((x, y), self.radius));
        }
    }

    /// Generate Metatron's Cube (Fruit of Life + connecting lines)
    fn generate_metatrons_cube(&mut self) {
        self.generate_fruit_of_life();
        // Metatron's Cube has the same circles as Fruit of Life
        // The connections form the cube and other Platonic solids
    }

    /// Calculate connections between circles
    fn calculate_connections(&self) -> Vec<(usize, usize)> {
        let mut connections = Vec::new();

        for i in 0..self.circles.len() {
            for j in (i + 1)..self.circles.len() {
                let (center_a, radius_a) = self.circles[i];
                let (center_b, radius_b) = self.circles[j];

                let distance =
                    ((center_b.0 - center_a.0).powi(2) + (center_b.1 - center_a.1).powi(2)).sqrt();

                // Circles overlap if distance < sum of radii
                if distance < radius_a + radius_b && distance > 0.001 {
                    connections.push((i, j));
                }
            }
        }

        connections
    }

    /// Create layers from circles
    fn create_layers(&mut self, layers: usize) {
        for layer in 0..=layers {
            let circles_in_layer = if layer == 0 { 1 } else { layer * 6 };
            let start = if layer == 0 { 0 } else { 1 + (layer - 1) * 6 };
            let end = (start + circles_in_layer).min(self.circles.len());

            self.layers.push(FlowerLayer {
                layer_index: layer,
                circles: self.circles[start..end].to_vec(),
            });
        }
    }

    /// Get pattern type
    pub fn pattern_type(&self) -> FlowerType {
        self.pattern_type
    }

    /// Get circles
    pub fn circles(&self) -> &[((Float, Float), Float)] {
        &self.circles
    }

    /// Get connections
    pub fn connections(&self) -> &[(usize, usize)] {
        &self.connections
    }

    /// Get layers
    pub fn layers(&self) -> &[FlowerLayer] {
        &self.layers
    }
}

/// Flower Type
///
/// Different types of flower patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowerType {
    /// Standard Flower of Life
    Standard,
    /// Seed of Life (7 circles)
    SeedOfLife,
    /// Fruit of Life (13 circles)
    FruitOfLife,
    /// Metatron's Cube (13 circles + connections)
    MetatronsCube,
}

impl FlowerType {
    /// Get name of flower type
    pub fn name(&self) -> &'static str {
        match self {
            FlowerType::Standard => "Flower of Life",
            FlowerType::SeedOfLife => "Seed of Life",
            FlowerType::FruitOfLife => "Fruit of Life",
            FlowerType::MetatronsCube => "Metatron's Cube",
        }
    }
}

/// Flower Layer
///
/// A single layer of the Flower of Life pattern.
#[derive(Debug, Clone)]
pub struct FlowerLayer {
    layer_index: usize,
    circles: Vec<((Float, Float), Float)>,
}

impl FlowerLayer {
    /// Create a new flower layer
    pub fn new(layer_index: usize, circles: Vec<((Float, Float), Float)>) -> Self {
        FlowerLayer {
            layer_index,
            circles,
        }
    }

    /// Get layer index
    pub fn layer_index(&self) -> usize {
        self.layer_index
    }

    /// Get circles in this layer
    pub fn circles(&self) -> &[((Float, Float), Float)] {
        &self.circles
    }

    /// Get number of circles in this layer
    pub fn circle_count(&self) -> usize {
        self.circles.len()
    }
}

/// Get circles for Flower of Life pattern
///
/// Convenience function for creating Flower of Life circles.
pub fn flower_of_life_circles(
    center: (Float, Float),
    radius: Float,
    layers: usize,
) -> Vec<((Float, Float), Float)> {
    let flower = FlowerOfLife::new(center.0, center.1, radius, layers);
    flower.circles().to_vec()
}

/// Get complete Flower of Life pattern
///
/// Convenience function for creating a complete Flower of Life pattern.
pub fn flower_of_life_pattern(
    center: (Float, Float),
    radius: Float,
    layers: usize,
) -> FlowerPattern {
    FlowerPattern::new(FlowerType::Standard, center, radius, layers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flower_of_life_creation() {
        let flower = FlowerOfLife::new(0.0, 0.0, 1.0, 2);

        assert_eq!(flower.center(), (0.0, 0.0));
        assert_eq!(flower.radius(), 1.0);
        assert_eq!(flower.layers(), 2);
        assert!(flower.circle_count() > 1);
    }

    #[test]
    fn test_flower_of_life_single_layer() {
        let flower = FlowerOfLife::new(0.0, 0.0, 1.0, 1);

        // Should have 7 circles (1 center + 6 surrounding)
        assert_eq!(flower.circle_count(), 7);
    }

    #[test]
    fn test_flower_of_life_two_layers() {
        let flower = FlowerOfLife::new(0.0, 0.0, 1.0, 2);

        // Should have 19 circles (1 + 6 + 12)
        assert_eq!(flower.circle_count(), 19);
    }

    #[test]
    fn test_flower_of_life_connections() {
        let flower = FlowerOfLife::new(0.0, 0.0, 1.0, 1);

        // Should have connections between overlapping circles
        let connections = flower.connections();
        assert!(!connections.is_empty());
    }

    #[test]
    fn test_flower_pattern_standard() {
        let pattern = FlowerPattern::new(FlowerType::Standard, (0.0, 0.0), 1.0, 2);

        assert_eq!(pattern.pattern_type(), FlowerType::Standard);
        assert!(!pattern.circles().is_empty());
        assert!(!pattern.connections().is_empty());
    }

    #[test]
    fn test_flower_pattern_seed_of_life() {
        let pattern = FlowerPattern::new(FlowerType::SeedOfLife, (0.0, 0.0), 1.0, 1);

        assert_eq!(pattern.pattern_type(), FlowerType::SeedOfLife);
        assert_eq!(pattern.circles().len(), 7);
    }

    #[test]
    fn test_flower_pattern_fruit_of_life() {
        let pattern = FlowerPattern::new(FlowerType::FruitOfLife, (0.0, 0.0), 1.0, 2);

        assert_eq!(pattern.pattern_type(), FlowerType::FruitOfLife);
        assert!(pattern.circles().len() > 7);
    }

    #[test]
    fn test_flower_pattern_metatrons_cube() {
        let pattern = FlowerPattern::new(FlowerType::MetatronsCube, (0.0, 0.0), 1.0, 2);

        assert_eq!(pattern.pattern_type(), FlowerType::MetatronsCube);
    }

    #[test]
    fn test_flower_type_names() {
        assert_eq!(FlowerType::Standard.name(), "Flower of Life");
        assert_eq!(FlowerType::SeedOfLife.name(), "Seed of Life");
        assert_eq!(FlowerType::FruitOfLife.name(), "Fruit of Life");
        assert_eq!(FlowerType::MetatronsCube.name(), "Metatron's Cube");
    }

    #[test]
    fn test_flower_layer() {
        let circles = vec![((0.0, 0.0), 1.0), ((1.0, 0.0), 1.0)];
        let layer = FlowerLayer::new(0, circles);

        assert_eq!(layer.layer_index(), 0);
        assert_eq!(layer.circle_count(), 2);
    }

    #[test]
    fn test_flower_of_life_circles_function() {
        let circles = flower_of_life_circles((0.0, 0.0), 1.0, 1);

        assert!(!circles.is_empty());
        assert_eq!(circles.len(), 7);
    }

    #[test]
    fn test_flower_of_life_pattern_function() {
        let pattern = flower_of_life_pattern((0.0, 0.0), 1.0, 1);

        assert!(!pattern.circles().is_empty());
    }
}
