use std::f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LODLevel {
    Level0, // Full detail (closest)
    Level1, // 75% detail
    Level2, // 50% detail
    Level3, // 25% detail
    Level4, // 12.5% detail
    Level5, // Point/sprite (furthest)
}

impl LODLevel {
    pub fn to_float(&self) -> f32 {
        match self {
            LODLevel::Level0 => 1.0,
            LODLevel::Level1 => 0.75,
            LODLevel::Level2 => 0.5,
            LODLevel::Level3 => 0.25,
            LODLevel::Level4 => 0.125,
            LODLevel::Level5 => 0.0,
        }
    }

    pub fn vertex_multiplier(&self) -> f32 {
        match self {
            LODLevel::Level0 => 1.0,
            LODLevel::Level1 => 0.75,
            LODLevel::Level2 => 0.5,
            LODLevel::Level3 => 0.25,
            LODLevel::Level4 => 0.125,
            LODLevel::Level5 => 0.0625,
        }
    }
}

pub struct LODSystem {
    lod_thresholds: [f32; 6], // Distance thresholds for each LOD level
    transition_duration: f32, // Seconds for smooth transitions
}

impl Default for LODSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl LODSystem {
    pub fn new() -> Self {
        Self {
            lod_thresholds: [
                1.0,   // Level 0: < 1.0 units
                5.0,   // Level 1: 1.0 - 5.0 units
                15.0,  // Level 2: 5.0 - 15.0 units
                30.0,  // Level 3: 15.0 - 30.0 units
                60.0,  // Level 4: 30.0 - 60.0 units
                100.0, // Level 5: > 60.0 units
            ],
            transition_duration: 0.5,
        }
    }

    pub fn with_thresholds(mut self, thresholds: [f32; 6]) -> Self {
        self.lod_thresholds = thresholds;
        self
    }

    pub fn with_transition_duration(mut self, duration: f32) -> Self {
        self.transition_duration = duration;
        self
    }

    pub fn calculate_lod(&self, distance: f32) -> LODLevel {
        for (i, &threshold) in self.lod_thresholds.iter().enumerate() {
            if distance < threshold {
                return match i {
                    0 => LODLevel::Level0,
                    1 => LODLevel::Level1,
                    2 => LODLevel::Level2,
                    3 => LODLevel::Level3,
                    4 => LODLevel::Level4,
                    _ => LODLevel::Level5,
                };
            }
        }
        LODLevel::Level5
    }

    pub fn calculate_lod_interpolated(
        &self,
        distance: f32,
        previous_lod: LODLevel,
        _delta_time: f32,
    ) -> (LODLevel, f32) {
        let target_lod = self.calculate_lod(distance);
        let interpolation_factor = if previous_lod == target_lod {
            1.0
        } else {
            0.5 // Smooth transition default
        };
        (target_lod, interpolation_factor)
    }

    pub fn get_vertex_count(&self, base_vertices: u32, lod: LODLevel) -> u32 {
        let multiplier = lod.vertex_multiplier();
        ((base_vertices as f32) * multiplier).max(3.0) as u32
    }

    pub fn get_rendering_quality(&self, lod: LODLevel) -> (bool, bool, bool) {
        match lod {
            LODLevel::Level0 => (true, true, true), // Shadows, reflections, post-process
            LODLevel::Level1 => (true, true, false), // Shadows, reflections
            LODLevel::Level2 => (true, false, false), // Shadows only
            LODLevel::Level3 => (false, false, false),
            LODLevel::Level4 => (false, false, false),
            LODLevel::Level5 => (false, false, false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod_levels() {
        let lod_system = LODSystem::new();
        assert_eq!(lod_system.calculate_lod(0.5), LODLevel::Level0);
        assert_eq!(lod_system.calculate_lod(2.0), LODLevel::Level1);
        assert_eq!(lod_system.calculate_lod(10.0), LODLevel::Level2);
        assert_eq!(lod_system.calculate_lod(20.0), LODLevel::Level3);
        assert_eq!(lod_system.calculate_lod(45.0), LODLevel::Level4);
        assert_eq!(lod_system.calculate_lod(80.0), LODLevel::Level5);
    }

    #[test]
    fn test_vertex_multiplier() {
        assert_eq!(LODLevel::Level0.vertex_multiplier(), 1.0);
        assert_eq!(LODLevel::Level2.vertex_multiplier(), 0.5);
        assert_eq!(LODLevel::Level5.vertex_multiplier(), 0.0625);
    }

    #[test]
    fn test_vertex_count() {
        let lod_system = LODSystem::new();
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level0), 100);
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level1), 75);
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level2), 50);
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level3), 25);
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level4), 12);
        assert_eq!(lod_system.get_vertex_count(100, LODLevel::Level5), 6);
    }
}
