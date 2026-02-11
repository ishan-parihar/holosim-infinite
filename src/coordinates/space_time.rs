// Space/Time Coordinate - Physical realm coordinates
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "Space/Time: The physical realm where entities experience separation,
// limitation, and linear time"

use crate::types::Float;

/// Planetary system identifier
pub type PlanetarySystemId = u64;

/// Space/Time Coordinate - physical realm coordinates
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSpaceTimeCoord {
    /// Physical coordinates (3D + time)
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub t: Float,

    /// The corresponding Time/Space coordinates
    /// (accessible when veil is thin)
    pub time_space_reflection: Option<crate::coordinates::time_space::MetaphysicalTimeSpaceCoord>,

    /// Which solar system this coordinate is in
    pub planetary_system: PlanetarySystemId,
}

impl PhysicalSpaceTimeCoord {
    /// Create new Space/Time coordinate
    pub fn new(
        x: Float,
        y: Float,
        z: Float,
        t: Float,
        planetary_system: PlanetarySystemId,
    ) -> Self {
        PhysicalSpaceTimeCoord {
            x,
            y,
            z,
            t,
            time_space_reflection: None,
            planetary_system,
        }
    }

    /// Create Space/Time coordinate with Time/Space reflection
    pub fn with_reflection(
        x: Float,
        y: Float,
        z: Float,
        t: Float,
        planetary_system: PlanetarySystemId,
        time_space_reflection: crate::coordinates::time_space::MetaphysicalTimeSpaceCoord,
    ) -> Self {
        PhysicalSpaceTimeCoord {
            x,
            y,
            z,
            t,
            time_space_reflection: Some(time_space_reflection),
            planetary_system,
        }
    }

    /// Calculate distance to another Space/Time coordinate (spatial only)
    pub fn spatial_distance(&self, other: &PhysicalSpaceTimeCoord) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calculate spacetime interval to another coordinate
    pub fn spacetime_interval(&self, other: &PhysicalSpaceTimeCoord, c: Float) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dt = self.t - other.t;

        // Spacetime interval: s² = (c*dt)² - dx² - dy² - dz²
        let c_dt = c * dt;
        let interval_sq = c_dt * c_dt - (dx * dx + dy * dy + dz * dz);
        interval_sq.sqrt().abs()
    }

    /// Check if coordinates are causally connected (within light cone)
    pub fn is_causally_connected(&self, other: &PhysicalSpaceTimeCoord, c: Float) -> bool {
        let spatial_dist = self.spatial_distance(other);
        let time_dist = (self.t - other.t).abs();

        // Causally connected if spatial distance <= c * time distance
        spatial_dist <= c * time_dist
    }

    /// Set Time/Space reflection
    pub fn set_time_space_reflection(
        &mut self,
        reflection: crate::coordinates::time_space::MetaphysicalTimeSpaceCoord,
    ) {
        self.time_space_reflection = Some(reflection);
    }

    /// Clear Time/Space reflection
    pub fn clear_time_space_reflection(&mut self) {
        self.time_space_reflection = None;
    }

    /// Get position as tuple
    pub fn as_tuple(&self) -> (Float, Float, Float, Float) {
        (self.x, self.y, self.z, self.t)
    }

    /// Create origin (0, 0, 0, 0)
    pub fn origin(planetary_system: PlanetarySystemId) -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, planetary_system)
    }
}

impl Default for PhysicalSpaceTimeCoord {
    fn default() -> Self {
        Self::origin(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_time_coord_new() {
        let coord = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        assert_eq!(coord.x, 1.0);
        assert_eq!(coord.y, 2.0);
        assert_eq!(coord.z, 3.0);
        assert_eq!(coord.t, 4.0);
        assert_eq!(coord.planetary_system, 1);
        assert!(coord.time_space_reflection.is_none());
    }

    #[test]
    fn test_space_time_coord_with_reflection() {
        let time_space = crate::coordinates::time_space::MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::time_space::ExperienceVector::new(),
            crate::coordinates::time_space::IncarnationPlan::new(),
            crate::coordinates::time_space::BroaderConsciousnessAccess::new(),
        );
        let coord = PhysicalSpaceTimeCoord::with_reflection(1.0, 2.0, 3.0, 4.0, 1, time_space);
        assert!(coord.time_space_reflection.is_some());
    }

    #[test]
    fn test_spatial_distance() {
        let coord1 = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1);
        let coord2 = PhysicalSpaceTimeCoord::new(3.0, 4.0, 0.0, 0.0, 1);
        let distance = coord1.spatial_distance(&coord2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_spacetime_interval() {
        let coord1 = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1);
        let coord2 = PhysicalSpaceTimeCoord::new(3.0, 0.0, 0.0, 2.0, 1);
        let c = 3.0; // Speed of light (simplified)
        let interval = coord1.spacetime_interval(&coord2, c);
        // s² = (3*2)² - 3² = 36 - 9 = 27, s = sqrt(27) ≈ 5.196
        assert!((interval - 5.196).abs() < 0.01);
    }

    #[test]
    fn test_is_causally_connected_true() {
        let coord1 = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1);
        let coord2 = PhysicalSpaceTimeCoord::new(3.0, 0.0, 0.0, 2.0, 1);
        let c = 3.0;
        // Spatial distance: 3, Time distance: 2, c * time: 6
        // 3 <= 6, so causally connected
        assert!(coord1.is_causally_connected(&coord2, c));
    }

    #[test]
    fn test_is_causally_connected_false() {
        let coord1 = PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1);
        let coord2 = PhysicalSpaceTimeCoord::new(10.0, 0.0, 0.0, 2.0, 1);
        let c = 3.0;
        // Spatial distance: 10, Time distance: 2, c * time: 6
        // 10 > 6, so not causally connected
        assert!(!coord1.is_causally_connected(&coord2, c));
    }

    #[test]
    fn test_set_time_space_reflection() {
        let mut coord = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let time_space = crate::coordinates::time_space::MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::time_space::ExperienceVector::new(),
            crate::coordinates::time_space::IncarnationPlan::new(),
            crate::coordinates::time_space::BroaderConsciousnessAccess::new(),
        );
        coord.set_time_space_reflection(time_space);
        assert!(coord.time_space_reflection.is_some());
    }

    #[test]
    fn test_clear_time_space_reflection() {
        let time_space = crate::coordinates::time_space::MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::time_space::ExperienceVector::new(),
            crate::coordinates::time_space::IncarnationPlan::new(),
            crate::coordinates::time_space::BroaderConsciousnessAccess::new(),
        );
        let mut coord = PhysicalSpaceTimeCoord::with_reflection(1.0, 2.0, 3.0, 4.0, 1, time_space);
        assert!(coord.time_space_reflection.is_some());
        coord.clear_time_space_reflection();
        assert!(coord.time_space_reflection.is_none());
    }

    #[test]
    fn test_as_tuple() {
        let coord = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let tuple = coord.as_tuple();
        assert_eq!(tuple, (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_origin() {
        let origin = PhysicalSpaceTimeCoord::origin(1);
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);
        assert_eq!(origin.t, 0.0);
        assert_eq!(origin.planetary_system, 1);
    }

    #[test]
    fn test_default() {
        let coord = PhysicalSpaceTimeCoord::default();
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
        assert_eq!(coord.z, 0.0);
        assert_eq!(coord.t, 0.0);
        assert_eq!(coord.planetary_system, 0);
    }

    #[test]
    fn test_space_time_coord_clone() {
        let coord1 = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let coord2 = coord1.clone();
        assert_eq!(coord1, coord2);
    }

    #[test]
    fn test_space_time_coord_partial_eq() {
        let coord1 = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let coord2 = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let coord3 = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 2);

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }
}
