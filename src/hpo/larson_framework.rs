//! Larson Reciprocal Framework (Phase C)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The Larson reciprocal framework defines v = s/t (Space/Time dominant)
//! and v = t/s (Time/Space dominant) with v = 1 representing the Veil as a structural feature where
//! the qualitative nature of reality transforms from Many-ness to Oneness."
//!
//! Key concepts:
//! - v = s/t: Three-dimensional space with one-dimensional time (physical reality)
//! - v = t/s: One-dimensional space with three-dimensional time (spiritual reality)
//! - The Veil at v = 1: Boundary where consciousness experiences reality shift
//! - Below v = 1: Many-ness (separation consciousness)
//! - Above v = 1: Oneness (unity consciousness)

use super::field_state::{Float, OctreeNode};
use std::collections::HashMap;

/// The Larson velocity ratio - determines Space/Time vs Time/Space dominance
/// v = s/t (Space/Time) or v = t/s (Time/Space)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LarsonMode {
    /// Space/Time dominant (v < 1)
    /// Entities experience classical time flow, spatial navigation
    SpaceTime,

    /// At the Veil (v ≈ 1)
    /// Entities experience transformation between modes
    AtVeil,

    /// Time/Space dominant (v > 1)
    /// Entities experience time as navigable dimensions
    TimeSpace,
}

impl LarsonMode {
    pub fn from_velocity_ratio(v: Float) -> Self {
        if v < 0.9 {
            LarsonMode::SpaceTime
        } else if v > 1.1 {
            LarsonMode::TimeSpace
        } else {
            LarsonMode::AtVeil
        }
    }

    pub fn velocity_ratio(&self) -> Float {
        match self {
            LarsonMode::SpaceTime => 0.5,
            LarsonMode::AtVeil => 1.0,
            LarsonMode::TimeSpace => 2.0,
        }
    }
}

/// Configuration for the Larson framework
#[derive(Debug, Clone)]
pub struct LarsonConfig {
    /// Veil position (v = 1.0)
    pub veil_position: Float,

    /// Veil thickness (region of transformation)
    pub veil_thickness: Float,

    /// Rate of time flow in Space/Time mode
    pub space_time_rate: Float,

    /// Rate of time flow in Time/Space mode
    pub time_space_rate: Float,

    /// Strength of the Veil as barrier
    pub veil_barrier_strength: Float,

    /// Enable veil crossing events
    pub enable_veil_crossing: bool,
}

impl Default for LarsonConfig {
    fn default() -> Self {
        LarsonConfig {
            veil_position: 1.0,
            veil_thickness: 0.2,
            space_time_rate: 1.0,
            time_space_rate: 3.0,
            veil_barrier_strength: 0.5,
            enable_veil_crossing: true,
        }
    }
}

/// Entity's relationship to the Veil
/// Determines how they experience time and space
#[derive(Debug, Clone)]
pub struct VeilRelationship {
    /// Current velocity ratio (s/t or t/s)
    pub velocity_ratio: Float,

    /// Current mode (Space/Time, AtVeil, Time/Space)
    pub mode: LarsonMode,

    /// Perception of time flow (0 = static, 1 = flowing)
    pub time_perception: Float,

    /// Perception of space (0 = fixed, 1 = fluid)
    pub space_perception: Float,

    /// Access to past/future (only in Time/Space)
    pub temporal_access: Float,

    /// Number of veil crossings
    pub veil_crossings: usize,

    /// Time spent at each side
    pub time_below_veil: Float,
    pub time_at_veil: Float,
    pub time_above_veil: Float,
}

impl Default for VeilRelationship {
    fn default() -> Self {
        Self::new()
    }
}

impl VeilRelationship {
    pub fn new() -> Self {
        VeilRelationship {
            velocity_ratio: 0.5, // Start in Space/Time
            mode: LarsonMode::SpaceTime,
            time_perception: 1.0,  // Full time flow
            space_perception: 0.0, // Fixed space
            temporal_access: 0.0,  // No access to other times
            veil_crossings: 0,
            time_below_veil: 0.0,
            time_at_veil: 0.0,
            time_above_veil: 0.0,
        }
    }

    /// Update based on spectrum position
    pub fn update_from_spectrum(&mut self, spectrum_position: Float) {
        // Map spectrum position to velocity ratio
        // spectrum < 0.5 => v < 1 (Space/Time)
        // spectrum > 0.5 => v > 1 (Time/Space)
        self.velocity_ratio = if spectrum_position < 0.5 {
            0.5 + spectrum_position // Range: 0.5 to 1.0
        } else {
            1.0 + (spectrum_position - 0.5) * 2.0 // Range: 1.0 to 2.0
        };

        // Determine mode
        self.mode = LarsonMode::from_velocity_ratio(self.velocity_ratio);

        // Update perceptions based on mode
        match self.mode {
            LarsonMode::SpaceTime => {
                // Time flows forward, space is navigable
                self.time_perception = 1.0;
                self.space_perception = 0.2;
                self.temporal_access = 0.0;
                self.time_below_veil += 1.0;
            }
            LarsonMode::AtVeil => {
                // Transformation zone
                self.time_perception = 0.5;
                self.space_perception = 0.5;
                self.temporal_access = 0.3;
                self.time_at_veil += 1.0;
            }
            LarsonMode::TimeSpace => {
                // Time becomes dimensions, space becomes fixed locus
                self.time_perception = 0.3;
                self.space_perception = 1.0;
                self.temporal_access = 1.0;
                self.time_above_veil += 1.0;
            }
        }
    }

    /// Get the effective time step for this entity
    pub fn get_effective_time_step(&self, base_dt: Float, time_space_rate: Float) -> Float {
        match self.mode {
            LarsonMode::SpaceTime => base_dt * self.time_perception * 1.0,
            LarsonMode::AtVeil => base_dt * 0.5,
            LarsonMode::TimeSpace => base_dt * self.time_perception * time_space_rate,
        }
    }
}

/// Complete Larson framework system
pub struct LarsonFramework {
    /// Configuration
    pub config: LarsonConfig,

    /// Veil position in the field
    pub veil_position: Float,

    /// Current veil transparency (how permeable)
    pub veil_transparency: Float,

    /// Entity veil relationships
    pub entity_relationships: HashMap<usize, VeilRelationship>,

    /// Statistics
    pub statistics: LarsonStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct LarsonStatistics {
    pub entities_below_veil: usize,
    pub entities_at_veil: usize,
    pub entities_above_veil: usize,
    pub total_veil_crossings: usize,
    pub average_velocity_ratio: Float,
    pub veil_transparency: Float,
}

impl LarsonFramework {
    pub fn new(config: LarsonConfig) -> Self {
        LarsonFramework {
            config: config.clone(),
            veil_position: config.veil_position,
            veil_transparency: 0.5,
            entity_relationships: HashMap::new(),
            statistics: LarsonStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(LarsonConfig::default())
    }

    /// Initialize relationships for entities
    pub fn initialize_entities(&mut self, entity_count: usize) {
        self.entity_relationships.clear();

        for i in 0..entity_count {
            // Entities start at various positions relative to the veil
            let relationship = VeilRelationship::new();
            self.entity_relationships.insert(i, relationship);
        }
    }

    /// Update entity veil relationships
    pub fn update_entity(&mut self, entity_id: usize, spectrum_position: Float) {
        let relationship = self
            .entity_relationships
            .entry(entity_id)
            .or_default();

        let old_mode = relationship.mode;
        relationship.update_from_spectrum(spectrum_position);

        // Track veil crossings
        if old_mode != relationship.mode
            && (old_mode == LarsonMode::AtVeil || relationship.mode == LarsonMode::AtVeil) {
                relationship.veil_crossings += 1;
            }
    }

    /// Update veil transparency based on field coherence
    pub fn update_veil(&mut self, coherence: Float) {
        // Higher coherence = more transparent veil
        // This represents clearer perception across the boundary
        self.veil_transparency = (self.veil_transparency * 0.95 + coherence * 0.05).clamp(0.0, 1.0);
        self.statistics.veil_transparency = self.veil_transparency;
    }

    /// Calculate the veil barrier effect
    /// Returns how strongly entities are pulled back at the veil
    pub fn get_veil_barrier(&self, entity_id: usize) -> Float {
        if let Some(relationship) = self.entity_relationships.get(&entity_id) {
            if relationship.mode == LarsonMode::AtVeil {
                return self.config.veil_barrier_strength * (1.0 - self.veil_transparency);
            }
        }
        0.0
    }

    /// Apply Space/Time ↔ Time/Space transformation to position
    pub fn transform_position(
        &self,
        position: &[Float; 3],
        entity_id: usize,
        dt: Float,
    ) -> [Float; 3] {
        let relationship = match self.entity_relationships.get(&entity_id) {
            Some(r) => r,
            None => return *position,
        };

        match relationship.mode {
            LarsonMode::SpaceTime => {
                // Time flows forward, space navigable
                // Standard position update
                [
                    position[0],
                    position[1],
                    position[2] + dt * self.config.space_time_rate,
                ]
            }
            LarsonMode::AtVeil => {
                // Transformation zone - position fluctuates
                let wave = (dt * 10.0).sin() * 0.1 * self.veil_transparency;
                [position[0] + wave, position[1] + wave, position[2]]
            }
            LarsonMode::TimeSpace => {
                // Time becomes navigable dimensions
                // Position in 3D space represents time coordinates
                // We can access past/future through spatial movement
                let time_access = relationship.temporal_access;

                [
                    position[0],                          // Fixed locus
                    position[1] + dt * time_access * 0.5, // Can move in "time space"
                    position[2] + dt * time_access * 0.3, // Access to other times
                ]
            }
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> LarsonStatistics {
        let mut stats = self.statistics.clone();

        stats.entities_below_veil = 0;
        stats.entities_at_veil = 0;
        stats.entities_above_veil = 0;
        stats.total_veil_crossings = 0;

        let mut total_ratio = 0.0;

        for relationship in self.entity_relationships.values() {
            match relationship.mode {
                LarsonMode::SpaceTime => stats.entities_below_veil += 1,
                LarsonMode::AtVeil => stats.entities_at_veil += 1,
                LarsonMode::TimeSpace => stats.entities_above_veil += 1,
            }
            stats.total_veil_crossings += relationship.veil_crossings;
            total_ratio += relationship.velocity_ratio;
        }

        let count = self.entity_relationships.len() as Float;
        if count > 0.0 {
            stats.average_velocity_ratio = total_ratio / count;
        }

        stats
    }

    /// Get mode description for an entity
    pub fn get_entity_mode_description(&self, entity_id: usize) -> String {
        if let Some(relationship) = self.entity_relationships.get(&entity_id) {
            match relationship.mode {
                LarsonMode::SpaceTime => format!(
                    "Space/Time dominant (v={:.2}) - Sequential time, navigable space",
                    relationship.velocity_ratio
                ),
                LarsonMode::AtVeil => {
                    "At the Veil (v≈1) - Transformation zone - Crossing threshold".to_string()
                }
                LarsonMode::TimeSpace => format!(
                    "Time/Space dominant (v={:.2}) - Time as dimensions, fixed locus",
                    relationship.velocity_ratio
                ),
            }
        } else {
            "Unknown".to_string()
        }
    }
}

/// Apply veil dynamics to field node
pub fn apply_veil_to_field(node: &mut OctreeNode, veil_position: Float, veil_transparency: Float) {
    // Get the node's spectrum position (stored in field_data)
    let spectrum = node.field_data.spectrum_position;

    // Calculate distance from veil
    let distance_from_veil = (spectrum - veil_position).abs();

    // If close to veil, apply barrier effect
    if distance_from_veil < 0.2 {
        // Veil barrier effect - reduces movement across veil
        let barrier_strength = (1.0 - veil_transparency) * 0.1;

        // Dampen coherence near veil (unless very transparent)
        node.field_data.coherence *= 1.0 - barrier_strength;

        // Modify energy near veil
        node.field_data.energy *= 1.0 - barrier_strength * 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larson_mode() {
        assert_eq!(LarsonMode::from_velocity_ratio(0.5), LarsonMode::SpaceTime);
        assert_eq!(LarsonMode::from_velocity_ratio(1.0), LarsonMode::AtVeil);
        assert_eq!(LarsonMode::from_velocity_ratio(2.0), LarsonMode::TimeSpace);
    }

    #[test]
    fn test_veil_relationship() {
        let mut relationship = VeilRelationship::new();

        // Test Space/Time
        relationship.update_from_spectrum(0.3);
        assert_eq!(relationship.mode, LarsonMode::SpaceTime);

        // Test At Veil
        relationship.update_from_spectrum(0.55); // Near v=1
        assert!(relationship.mode != LarsonMode::SpaceTime);

        // Test Time/Space
        relationship.update_from_spectrum(0.8);
        assert_eq!(relationship.mode, LarsonMode::TimeSpace);
    }

    #[test]
    fn test_framework() {
        let mut framework = LarsonFramework::with_defaults();
        framework.initialize_entities(10);

        // Update some entities
        framework.update_entity(0, 0.3); // Below veil
        framework.update_entity(1, 0.55); // At veil
        framework.update_entity(2, 0.8); // Above veil

        let stats = framework.get_statistics();

        assert!(stats.entities_below_veil > 0);
        assert!(stats.entities_above_veil > 0);
    }
}
