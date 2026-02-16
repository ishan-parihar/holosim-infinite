//! Vesica Piscis
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 19-20:
//! - Vesica Piscis in entity relationships
//! - Resonance creates Vesica Piscis patterns
//! - Intersection of two circles
//!
//! The Vesica Piscis is the shape formed by the intersection of two circles
//! of equal radius, where the center of each circle lies on the circumference
//! of the other. It's a fundamental sacred geometry pattern representing:
//! - The intersection of two realms or entities
//! - The birthplace of creation (the "womb" of the universe)
//! - Resonance and harmony between entities
//! - The doorway between dimensions

use crate::types::Float;

/// Vesica Piscis
///
/// The intersection of two circles of equal radius.
#[derive(Debug, Clone)]
pub struct VesicaPiscis {
    center_a: (Float, Float),
    center_b: (Float, Float),
    radius: Float,
    intersection_points: Vec<(Float, Float)>,
    width: Float,
    height: Float,
    area: Float,
}

impl VesicaPiscis {
    /// Create a new Vesica Piscis
    ///
    /// # Arguments
    /// * `center_a` - Center of first circle
    /// * `center_b` - Center of second circle
    /// * `radius` - Radius of both circles
    pub fn new(center_a: (Float, Float), center_b: (Float, Float), radius: Float) -> Self {
        let distance = ((center_b.0 - center_a.0).powi(2) + (center_b.1 - center_a.1).powi(2)).sqrt();
        
        // Calculate intersection points
        let intersection_points = Self::calculate_intersections(center_a, center_b, radius, distance);
        
        // Calculate dimensions
        let width = distance;
        let height = Self::calculate_height(radius, distance);
        let area = Self::calculate_area(radius, distance);
        
        VesicaPiscis {
            center_a,
            center_b,
            radius,
            intersection_points,
            width,
            height,
            area,
        }
    }

    /// Calculate intersection points of two circles
    fn calculate_intersections(
        center_a: (Float, Float),
        center_b: (Float, Float),
        radius: Float,
        distance: Float,
    ) -> Vec<(Float, Float)> {
        if distance > 2.0 * radius || distance == 0.0 {
            return Vec::new();
        }
        
        // Midpoint between centers
        let mid_x = (center_a.0 + center_b.0) / 2.0;
        let mid_y = (center_a.1 + center_b.1) / 2.0;
        
        // Distance from midpoint to intersection points
        let d = (radius.powi(2) - (distance / 2.0).powi(2)).sqrt();
        
        // Direction from center_a to center_b
        let dir_x = (center_b.0 - center_a.0) / distance;
        let dir_y = (center_b.1 - center_a.1) / distance;
        
        // Perpendicular direction
        let perp_x = -dir_y;
        let perp_y = dir_x;
        
        // Intersection points
        let p1 = (mid_x + perp_x * d, mid_y + perp_y * d);
        let p2 = (mid_x - perp_x * d, mid_y - perp_y * d);
        
        vec![p1, p2]
    }

    /// Calculate height of Vesica Piscis
    fn calculate_height(radius: Float, distance: Float) -> Float {
        2.0 * (radius.powi(2) - (distance / 2.0).powi(2)).sqrt()
    }

    /// Calculate area of Vesica Piscis
    fn calculate_area(radius: Float, distance: Float) -> Float {
        if distance >= 2.0 * radius {
            return 0.0;
        }
        
        let theta = 2.0 * (distance / (2.0 * radius)).acos();
        let segment_area = (radius.powi(2) / 2.0) * (theta - theta.sin());
        2.0 * segment_area
    }

    /// Get center of first circle
    pub fn center_a(&self) -> (Float, Float) {
        self.center_a
    }

    /// Get center of second circle
    pub fn center_b(&self) -> (Float, Float) {
        self.center_b
    }

    /// Get radius
    pub fn radius(&self) -> Float {
        self.radius
    }

    /// Get intersection points
    pub fn intersection_points(&self) -> &[(Float, Float)] {
        &self.intersection_points
    }

    /// Get width (distance between centers)
    pub fn width(&self) -> Float {
        self.width
    }

    /// Get height
    pub fn height(&self) -> Float {
        self.height
    }

    /// Get area
    pub fn area(&self) -> Float {
        self.area
    }

    /// Get aspect ratio (width/height)
    pub fn aspect_ratio(&self) -> Float {
        if self.height == 0.0 {
            0.0
        } else {
            self.width / self.height
        }
    }

    /// Check if circles intersect
    pub fn has_intersection(&self) -> bool {
        !self.intersection_points.is_empty()
    }

    /// Check if this is a perfect Vesica Piscis (centers on each other's circumference)
    pub fn is_perfect(&self) -> bool {
        let distance = ((self.center_b.0 - self.center_a.0).powi(2) + (self.center_b.1 - self.center_a.1).powi(2)).sqrt();
        (distance - self.radius).abs() < 0.001
    }
}

/// Vesica Piscis Pattern
///
/// A pattern of multiple Vesica Piscis arrangements.
#[derive(Debug, Clone)]
pub struct VesicaPiscisPattern {
    vesica_piscis: Vec<VesicaPiscis>,
    circles: Vec<((Float, Float), Float)>,
}

impl VesicaPiscisPattern {
    /// Create a new Vesica Piscis pattern
    pub fn new() -> Self {
        VesicaPiscisPattern {
            vesica_piscis: Vec::new(),
            circles: Vec::new(),
        }
    }

    /// Add a Vesica Piscis to the pattern
    pub fn add(&mut self, vesica: VesicaPiscis) {
        // Get values before moving vesica
        let center_a = vesica.center_a();
        let center_b = vesica.center_b();
        let radius = vesica.radius();
        
        self.vesica_piscis.push(vesica);
        
        // Add circles if not already present
        let circles: [((Float, Float), Float); 2] = [
            (center_a, radius),
            (center_b, radius),
        ];
        
        for circle in circles {
            if !self.circles.contains(&circle) {
                self.circles.push(circle);
            }
        }
    }

    /// Get all Vesica Piscis
    pub fn vesica_piscis(&self) -> &[VesicaPiscis] {
        &self.vesica_piscis
    }

    /// Get all circles
    pub fn circles(&self) -> &[((Float, Float), Float)] {
        &self.circles
    }

    /// Get total area of all Vesica Piscis
    pub fn total_area(&self) -> Float {
        self.vesica_piscis.iter().map(|v| v.area()).sum()
    }
}

impl Default for VesicaPiscisPattern {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity Vesica Piscis
///
/// Represents the Vesica Piscis pattern formed by two resonating entities.
#[derive(Debug, Clone)]
pub struct EntityVesicaPiscis {
    entity_a: u64,
    entity_b: u64,
    vesica: VesicaPiscis,
    overlap_ratio: Float,
    resonance: Float,
}

impl EntityVesicaPiscis {
    /// Create a new entity Vesica Piscis
    pub fn new(entity_a: u64, entity_b: u64, center_a: (Float, Float), center_b: (Float, Float), radius: Float) -> Self {
        let vesica = VesicaPiscis::new(center_a, center_b, radius);
        let overlap_ratio = if radius > 0.0 { vesica.area() / (std::f64::consts::PI * radius.powi(2)) } else { 0.0 };
        let resonance = overlap_ratio; // Resonance equals overlap ratio
        
        EntityVesicaPiscis {
            entity_a,
            entity_b,
            vesica,
            overlap_ratio,
            resonance,
        }
    }

    /// Get entity A ID
    pub fn entity_a(&self) -> u64 {
        self.entity_a
    }

    /// Get entity B ID
    pub fn entity_b(&self) -> u64 {
        self.entity_b
    }

    /// Get Vesica Piscis
    pub fn vesica(&self) -> &VesicaPiscis {
        &self.vesica
    }

    /// Get overlap ratio
    pub fn overlap_ratio(&self) -> Float {
        self.overlap_ratio
    }

    /// Get resonance
    pub fn resonance(&self) -> Float {
        self.resonance
    }

    /// Check if entities are in resonance
    pub fn is_resonant(&self, threshold: Float) -> bool {
        self.resonance >= threshold
    }
}

/// Calculate Vesica Piscis intersection
///
/// Convenience function for creating a Vesica Piscis.
pub fn vesica_piscis_intersection(
    center_a: (Float, Float),
    center_b: (Float, Float),
    radius: Float,
) -> VesicaPiscis {
    VesicaPiscis::new(center_a, center_b, radius)
}

/// Calculate resonance between two entities based on their positions and radii
pub fn calculate_entity_resonance(
    pos_a: (Float, Float),
    pos_b: (Float, Float),
    radius: Float,
) -> Float {
    let vesica = VesicaPiscis::new(pos_a, pos_b, radius);
    if radius > 0.0 {
        vesica.area() / (std::f64::consts::PI * radius.powi(2))
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vesica_piscis_creation() {
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        
        assert_eq!(vesica.center_a(), (0.0, 0.0));
        assert_eq!(vesica.center_b(), (1.0, 0.0));
        assert_eq!(vesica.radius(), 1.0);
        assert!(vesica.has_intersection());
    }

    #[test]
    fn test_vesica_piscis_perfect() {
        // Perfect Vesica Piscis: distance = radius
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        
        assert!(vesica.is_perfect());
    }

    #[test]
    fn test_vesica_piscis_no_intersection() {
        // Circles too far apart
        let vesica = VesicaPiscis::new((0.0, 0.0), (3.0, 0.0), 1.0);
        
        assert!(!vesica.has_intersection());
    }

    #[test]
    fn test_vesica_piscis_intersection_points() {
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        
        // Should have 2 intersection points
        assert_eq!(vesica.intersection_points().len(), 2);
    }

    #[test]
    fn test_vesica_piscis_area() {
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        
        // Area should be positive
        assert!(vesica.area() > 0.0);
    }

    #[test]
    fn test_vesica_piscis_pattern() {
        let mut pattern = VesicaPiscisPattern::new();
        
        let vesica1 = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        let vesica2 = VesicaPiscis::new((1.0, 0.0), (2.0, 0.0), 1.0);
        
        pattern.add(vesica1);
        pattern.add(vesica2);
        
        assert_eq!(pattern.vesica_piscis().len(), 2);
        assert!(pattern.total_area() > 0.0);
    }

    #[test]
    fn test_entity_vesica_piscis() {
        let entity_vesica = EntityVesicaPiscis::new(1, 2, (0.0, 0.0), (1.0, 0.0), 1.0);
        
        assert_eq!(entity_vesica.entity_a(), 1);
        assert_eq!(entity_vesica.entity_b(), 2);
        assert!(entity_vesica.overlap_ratio() > 0.0);
        assert!(entity_vesica.resonance() > 0.0);
    }

    #[test]
    fn test_entity_resonance_check() {
        let entity_vesica = EntityVesicaPiscis::new(1, 2, (0.0, 0.0), (1.0, 0.0), 1.0);
        
        // Should be resonant with low threshold
        assert!(entity_vesica.is_resonant(0.1));
        
        // May not be resonant with high threshold
        assert!(!entity_vesica.is_resonant(0.9));
    }

    #[test]
    fn test_calculate_entity_resonance() {
        let resonance = calculate_entity_resonance((0.0, 0.0), (1.0, 0.0), 1.0);
        
        // Resonance should be between 0 and 1
        assert!(resonance >= 0.0 && resonance <= 1.0);
    }

    #[test]
    fn test_vesica_piscis_aspect_ratio() {
        let vesica = VesicaPiscis::new((0.0, 0.0), (1.0, 0.0), 1.0);
        
        // Aspect ratio should be positive
        assert!(vesica.aspect_ratio() > 0.0);
    }
}
