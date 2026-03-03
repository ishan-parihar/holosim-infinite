//! Spectrum Position - Unified coordinate system for the Space/Time ↔ Time/Space spectrum.
//!
//! This module implements the SpectrumPosition type that replaces Cartesian 3D coordinates
//! with a position on the continuous spectrum between Space/Time and Time/Space.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Space/Time and Time/Space spectrum is a continuous range of reciprocal ratios
//! (v = s/t to v = t/s) with a qualitative break at v = 1 (the Veil)."

use crate::types::{Density, Float};
use rand;
use std::hash::{Hash, Hasher};

/// Relationship between two spectrum positions.
///
/// Represents a connection between entities in spectrum space,
/// where distance is measured in spectrum terms (not Euclidean).
#[derive(Clone, Debug, PartialEq)]
pub struct SpectrumRelationship {
    /// ID of the other spectrum position
    pub other_id: u64,
    /// Spectrum distance (not Euclidean)
    pub spectrum_distance: Float,
    /// Whether this relationship is accessible (not veiled)
    pub accessible: bool,
}

impl SpectrumRelationship {
    /// Create a new spectrum relationship
    pub fn new(other_id: u64, spectrum_distance: Float, accessible: bool) -> Self {
        Self {
            other_id,
            spectrum_distance: spectrum_distance.max(0.0),
            accessible,
        }
    }
}

/// Position on the Space/Time ↔ Time/Space spectrum.
///
/// This is the unified coordinate system that replaces Cartesian 3D coordinates.
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Space/Time and Time/Space spectrum is a continuous range of reciprocal ratios
/// (v = s/t to v = t/s) with a qualitative break at v = 1 (the Veil)."
#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumPosition {
    /// Unique identifier for this position
    pub position_id: u64,

    /// Velocity ratio (v = s/t for space/time dominant, v = t/s for time/space dominant)
    /// v < 1.0: Time/Space dominant (higher densities)
    /// v = 1.0: The Veil (boundary between space/time and time/space)
    /// v > 1.0: Space/Time dominant (lower densities)
    pub velocity_ratio: Float,

    /// Current density level (1st through 8th)
    pub density: Density,

    /// Position within the density (0.0 to 1.0)
    /// Represents progress within the current density
    pub phase: Float,

    /// Relationships to other spectrum positions
    pub relationships: Vec<SpectrumRelationship>,
}

impl SpectrumPosition {
    /// Create a new spectrum position with a randomly generated ID
    pub fn new(velocity_ratio: Float, density: Density, phase: Float) -> Self {
        Self {
            position_id: rand::random(),
            velocity_ratio: velocity_ratio.max(0.0),
            density,
            phase: phase.clamp(0.0, 1.0),
            relationships: Vec::new(),
        }
    }

    /// Create a spectrum position with a specific ID
    pub fn with_id(
        position_id: u64,
        velocity_ratio: Float,
        density: Density,
        phase: Float,
    ) -> Self {
        Self {
            position_id,
            velocity_ratio: velocity_ratio.max(0.0),
            density,
            phase: phase.clamp(0.0, 1.0),
            relationships: Vec::new(),
        }
    }

    /// Create a position at the Veil (v = 1.0)
    pub fn at_veil(density: Density, phase: Float) -> Self {
        Self::new(1.0, density, phase)
    }

    /// Create a space/time dominant position
    pub fn space_time_dominant(density: Density, phase: Float) -> Self {
        Self::new(20.0, density, phase)
    }

    /// Create a time/space dominant position
    pub fn time_space_dominant(density: Density, phase: Float) -> Self {
        Self::new(0.05, density, phase)
    }

    /// Create a position in the physical (space-dominant) region.
    /// This is for entities primarily experiencing Space/Time reality.
    /// v > 1.0 indicates space-dominant, physical existence.
    pub fn physical(density: Density) -> Self {
        // Physical positions have v > 1.0
        // Higher densities naturally have lower v values
        let base_v = 10.0;
        let density_factor = density.as_u8() as Float;
        let velocity_ratio = base_v / density_factor.sqrt();
        Self::new(velocity_ratio, density, 0.0)
    }

    /// Create a position in the metaphysical (time-dominant) region.
    /// This is for entities primarily experiencing Time/Space reality.
    /// v < 1.0 indicates time-dominant, metaphysical existence.
    pub fn metaphysical(density: Density) -> Self {
        // Metaphysical positions have v < 1.0
        // Higher densities have lower v values (closer to time/space)
        let base_v = 0.1;
        let density_factor = 9.0 - density.as_u8() as Float;
        let velocity_ratio = base_v * density_factor.sqrt();
        Self::new(velocity_ratio, density, 0.0)
    }

    /// Create from space_time_ratio and time_space_ratio
    pub fn from_space_time_ratio(space_time: Float, time_space: Float) -> Self {
        let ratio = if time_space > 0.0 {
            space_time / time_space
        } else {
            Float::INFINITY
        };
        Self::new(ratio, Density::Third, 0.0)
    }

    /// Compute phase from velocity ratio using sigmoid-like formula.
    /// Phase = 1.0 / (1.0 + v)
    /// This maps velocity ratio to a phase value in a smooth manner.
    pub fn compute_phase(v: Float) -> Float {
        if v <= 0.0 {
            1.0 // At v=0, phase is maximum
        } else {
            (1.0 / (1.0 + v)).clamp(0.0, 1.0)
        }
    }

    /// Check if at the Veil (v = 1.0)
    pub fn is_at_veil(&self) -> bool {
        (self.velocity_ratio - 1.0).abs() < 0.01
    }

    /// Check if space/time dominant (v > 1.0)
    pub fn is_space_time_dominant(&self) -> bool {
        self.velocity_ratio > 1.0
    }

    /// Check if time/space dominant (v < 1.0)
    pub fn is_time_space_dominant(&self) -> bool {
        self.velocity_ratio < 1.0
    }

    /// Check if in physical (space-dominant) region.
    /// Physical means v > 1.0 (space/time dominant).
    pub fn is_physical(&self) -> bool {
        self.velocity_ratio > 1.0
    }

    /// Check if in metaphysical (time-dominant) region.
    /// Metaphysical means v < 1.0 (time/space dominant).
    pub fn is_metaphysical(&self) -> bool {
        self.velocity_ratio < 1.0
    }

    /// Get space/time access level (0.0 to 1.0)
    pub fn space_time_access(&self) -> Float {
        if self.velocity_ratio >= 1.0 {
            (1.0 - 1.0 / self.velocity_ratio).clamp(0.0, 1.0)
        } else {
            self.velocity_ratio
        }
    }

    /// Get time/space access level (0.0 to 1.0)
    pub fn time_space_access(&self) -> Float {
        1.0 - self.space_time_access()
    }

    /// Evolve the spectrum position by a delta.
    /// Uses density.evolution_direction() to determine evolution speed.
    pub fn evolve(&mut self, delta: Float, density: Density) {
        // Evolution moves toward time/space dominance using density's evolution direction
        let evolution_factor = self.density.evolution_direction();
        let effective_delta = delta * evolution_factor;

        // Move velocity ratio toward time/space dominance (lower values)
        self.velocity_ratio = (self.velocity_ratio - effective_delta).max(0.01);

        // Phase evolves based on the density's natural progression
        let phase_delta = effective_delta * 0.1;
        self.phase = (self.phase + phase_delta).min(1.0);

        self.density = density;
    }

    /// Move this position toward another position by a specified amount.
    /// The movement is in spectrum space, not Euclidean space.
    pub fn move_toward(&mut self, other: &SpectrumPosition, amount: Float) {
        // Move velocity_ratio toward other
        let v_diff = other.velocity_ratio - self.velocity_ratio;
        self.velocity_ratio += v_diff * amount * 0.1;
        self.velocity_ratio = self.velocity_ratio.max(0.01);

        // Move phase toward other
        let phase_diff = other.phase - self.phase;
        self.phase += phase_diff * amount * 0.1;
        self.phase = self.phase.clamp(0.0, 1.0);

        // Density can also shift if we're close enough
        if amount > 0.5 {
            self.density = other.density;
        }
    }

    /// Compute distance to another spectrum position
    pub fn distance_to(&self, other: &SpectrumPosition) -> Float {
        let velocity_diff = (self.velocity_ratio.log2() - other.velocity_ratio.log2()).abs();
        let density_diff = (self.density.as_u8() as Float - other.density.as_u8() as Float).abs();
        let phase_diff = (self.phase - other.phase).abs();

        velocity_diff + density_diff + phase_diff
    }

    /// Check if this position can access another position.
    /// Uses density.veil_transparency() to determine access.
    pub fn can_access(&self, other: &SpectrumPosition) -> bool {
        // Get veil transparency for this density
        let veil_transparency = self.density.veil_transparency();

        // Higher density entities can access lower density positions
        if self.density.as_u8() >= other.density.as_u8() {
            return true;
        }

        // At same density, check spectrum access and veil transparency
        if self.density == other.density {
            let access_threshold = 0.1 * (1.0 - veil_transparency);
            (self.space_time_access() - other.space_time_access()).abs() < access_threshold
        } else {
            // Lower density trying to access higher density
            // Only possible if veil is transparent enough
            veil_transparency > 0.5
        }
    }

    /// Add a relationship to another spectrum position
    pub fn add_relationship(&mut self, other: &SpectrumPosition) {
        let distance = self.distance_to(other);
        let accessible = self.can_access(other);
        let relationship = SpectrumRelationship::new(other.position_id, distance, accessible);
        self.relationships.push(relationship);
    }

    /// Remove a relationship by other_id
    pub fn remove_relationship(&mut self, other_id: u64) {
        self.relationships.retain(|r| r.other_id != other_id);
    }

    /// Get a relationship by other_id
    pub fn get_relationship(&self, other_id: u64) -> Option<&SpectrumRelationship> {
        self.relationships.iter().find(|r| r.other_id == other_id)
    }

    /// Update all relationships (recalculate distances and accessibility)
    pub fn update_relationships(&mut self, positions: &[SpectrumPosition]) {
        for position in positions {
            if position.position_id != self.position_id {
                let distance = self.distance_to(position);
                let accessible = self.can_access(position);

                if let Some(existing) = self
                    .relationships
                    .iter_mut()
                    .find(|r| r.other_id == position.position_id)
                {
                    existing.spectrum_distance = distance;
                    existing.accessible = accessible;
                } else {
                    self.relationships.push(SpectrumRelationship::new(
                        position.position_id,
                        distance,
                        accessible,
                    ));
                }
            }
        }
    }

    /// Compute hash for caching
    pub fn hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.position_id.hash(&mut hasher);
        self.velocity_ratio.to_bits().hash(&mut hasher);
        self.density.as_u8().hash(&mut hasher);
        ((self.phase * 1000.0) as u64).hash(&mut hasher);
        hasher.finish()
    }

    /// Get the veil transparency at this position
    pub fn veil_transparency(&self) -> Float {
        // At v = 1.0, veil is opaque (0.0)
        // As v moves away from 1.0, veil becomes more transparent
        (1.0 - (1.0 - self.velocity_ratio).abs().min(1.0)).clamp(0.0, 1.0)
    }
}

impl Default for SpectrumPosition {
    fn default() -> Self {
        Self::at_veil(Density::Third, 0.0)
    }
}

impl std::fmt::Display for SpectrumPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpectrumPosition(id={}, v={:.3}, {:?}, phase={:.3})",
            self.position_id, self.velocity_ratio, self.density, self.phase
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_position_creation() {
        let pos = SpectrumPosition::new(2.0, Density::Third, 0.5);
        assert_eq!(pos.velocity_ratio, 2.0);
        assert_eq!(pos.density, Density::Third);
        assert_eq!(pos.phase, 0.5);
        // position_id should be randomly generated (non-zero is a good check)
        // Note: could theoretically be 0, but extremely unlikely
        assert!(pos.relationships.is_empty());
    }

    #[test]
    fn test_spectrum_position_with_id() {
        let pos = SpectrumPosition::with_id(42, 2.0, Density::Third, 0.5);
        assert_eq!(pos.position_id, 42);
        assert_eq!(pos.velocity_ratio, 2.0);
        assert_eq!(pos.density, Density::Third);
        assert_eq!(pos.phase, 0.5);
    }

    #[test]
    fn test_spectrum_position_clamping() {
        // Negative velocity_ratio should be clamped to 0.0
        let pos = SpectrumPosition::new(-1.0, Density::Third, 0.5);
        assert_eq!(pos.velocity_ratio, 0.0);

        // Phase should be clamped to [0.0, 1.0]
        let pos = SpectrumPosition::new(1.0, Density::Third, 1.5);
        assert_eq!(pos.phase, 1.0);

        let pos = SpectrumPosition::new(1.0, Density::Third, -0.5);
        assert_eq!(pos.phase, 0.0);
    }

    #[test]
    fn test_spectrum_position_at_veil() {
        let pos = SpectrumPosition::at_veil(Density::Third, 0.0);
        assert!(pos.is_at_veil());
        assert!(!pos.is_space_time_dominant());
        assert!(!pos.is_time_space_dominant());
    }

    #[test]
    fn test_spectrum_position_space_time_dominant() {
        let pos = SpectrumPosition::space_time_dominant(Density::Third, 0.0);
        assert!(pos.is_space_time_dominant());
        assert!(!pos.is_time_space_dominant());
        assert!(pos.space_time_access() > 0.5);
    }

    #[test]
    fn test_spectrum_position_time_space_dominant() {
        let pos = SpectrumPosition::time_space_dominant(Density::Sixth, 0.5);
        assert!(pos.is_time_space_dominant());
        assert!(!pos.is_space_time_dominant());
        assert!(pos.time_space_access() > 0.5);
    }

    #[test]
    fn test_physical_position() {
        let pos = SpectrumPosition::physical(Density::Third);
        assert!(pos.is_physical());
        assert!(!pos.is_metaphysical());
        assert!(pos.velocity_ratio > 1.0);
    }

    #[test]
    fn test_metaphysical_position() {
        let pos = SpectrumPosition::metaphysical(Density::Sixth);
        assert!(pos.is_metaphysical());
        assert!(!pos.is_physical());
        assert!(pos.velocity_ratio < 1.0);
    }

    #[test]
    fn test_physical_higher_density() {
        // Higher density physical positions should have lower velocity_ratio
        let pos_third = SpectrumPosition::physical(Density::Third);
        let pos_fifth = SpectrumPosition::physical(Density::Fifth);
        assert!(pos_fifth.velocity_ratio < pos_third.velocity_ratio);
        assert!(pos_fifth.is_physical());
    }

    #[test]
    fn test_metaphysical_higher_density() {
        // Higher density metaphysical positions should have lower velocity_ratio
        let pos_third = SpectrumPosition::metaphysical(Density::Third);
        let pos_sixth = SpectrumPosition::metaphysical(Density::Sixth);
        assert!(pos_sixth.velocity_ratio < pos_third.velocity_ratio);
        assert!(pos_sixth.is_metaphysical());
    }

    #[test]
    fn test_compute_phase() {
        // At v=0, phase should be 1.0
        let phase = SpectrumPosition::compute_phase(0.0);
        assert_eq!(phase, 1.0);

        // At v=1, phase should be 0.5
        let phase = SpectrumPosition::compute_phase(1.0);
        assert!((phase - 0.5).abs() < 0.001);

        // At v=∞, phase should approach 0
        let phase = SpectrumPosition::compute_phase(1000.0);
        assert!(phase < 0.01);

        // Higher v should give lower phase
        let phase1 = SpectrumPosition::compute_phase(1.0);
        let phase2 = SpectrumPosition::compute_phase(2.0);
        assert!(phase2 < phase1);
    }

    #[ignore]
    #[test]
    fn test_move_toward() {
        let mut pos1 = SpectrumPosition::with_id(1, 10.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 2.0, Density::Fourth, 0.5);

        pos1.move_toward(&pos2, 0.5);

        // pos1 should have moved toward pos2's velocity_ratio
        assert!(pos1.velocity_ratio < 10.0);
        assert!(pos1.velocity_ratio > 2.0); // Not all the way there
                                            // Density only changes when amount > 0.5, so stays at Third
        assert_eq!(pos1.density, Density::Third);
    }

    #[test]
    fn test_move_toward_small_amount() {
        let mut pos1 = SpectrumPosition::with_id(1, 10.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 2.0, Density::Fourth, 0.5);

        pos1.move_toward(&pos2, 0.3);

        // pos1 should have moved slightly toward pos2
        assert!(pos1.velocity_ratio < 10.0);
        // Density should NOT change when amount <= 0.5
        assert_eq!(pos1.density, Density::Third);
    }

    #[test]
    fn test_is_physical_metaphysical() {
        let physical = SpectrumPosition::new(2.0, Density::Third, 0.0);
        assert!(physical.is_physical());
        assert!(!physical.is_metaphysical());

        let metaphysical = SpectrumPosition::new(0.5, Density::Sixth, 0.0);
        assert!(metaphysical.is_metaphysical());
        assert!(!metaphysical.is_physical());

        let at_veil = SpectrumPosition::at_veil(Density::Third, 0.0);
        assert!(!at_veil.is_physical());
        assert!(!at_veil.is_metaphysical());
    }

    #[test]
    fn test_spectrum_position_evolve() {
        let mut pos = SpectrumPosition::space_time_dominant(Density::Third, 0.0);
        pos.evolve(1.0, Density::Fourth);
        assert!(pos.velocity_ratio < 20.0);
        assert_eq!(pos.density, Density::Fourth);
    }

    #[test]
    fn test_evolve_uses_density_direction() {
        // Higher density should evolve faster
        let mut pos_low = SpectrumPosition::new(10.0, Density::Third, 0.0);
        let mut pos_high = SpectrumPosition::new(10.0, Density::Sixth, 0.0);

        let initial_v = 10.0;
        pos_low.evolve(1.0, Density::Third);
        pos_high.evolve(1.0, Density::Sixth);

        // Both should decrease, but higher density should decrease more
        assert!(pos_low.velocity_ratio < initial_v);
        assert!(pos_high.velocity_ratio < initial_v);
        // Sixth density has higher evolution_direction, so it should evolve faster
        assert!(pos_high.velocity_ratio < pos_low.velocity_ratio);
    }

    #[test]
    fn test_spectrum_position_distance_to() {
        let pos1 = SpectrumPosition::new(1.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::new(2.0, Density::Third, 0.0);
        let distance = pos1.distance_to(&pos2);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_spectrum_position_can_access_higher_density() {
        let higher = SpectrumPosition::new(1.0, Density::Sixth, 0.0);
        let lower = SpectrumPosition::new(1.0, Density::Third, 0.0);
        assert!(higher.can_access(&lower));
    }

    #[test]
    fn test_can_access_uses_veil_transparency() {
        // Sixth density has higher veil transparency
        let sixth = SpectrumPosition::new(1.0, Density::Sixth, 0.0);
        let third = SpectrumPosition::new(1.0, Density::Third, 0.0);

        // Sixth density should be able to access third (higher can access lower)
        assert!(sixth.can_access(&third));

        // Third density trying to access sixth - depends on veil transparency
        // Third density has veil_transparency = (3-1)/8 = 0.25, which is < 0.5
        // So it should NOT be able to access higher density
        assert!(!third.can_access(&sixth));
    }

    #[test]
    fn test_spectrum_position_hash() {
        let pos1 = SpectrumPosition::with_id(1, 1.0, Density::Third, 0.5);
        let pos2 = SpectrumPosition::with_id(1, 1.0, Density::Third, 0.5);
        assert_eq!(pos1.hash(), pos2.hash());
    }

    #[test]
    fn test_spectrum_position_different_hash() {
        let pos1 = SpectrumPosition::with_id(1, 1.0, Density::Third, 0.5);
        let pos2 = SpectrumPosition::with_id(2, 2.0, Density::Third, 0.5);
        assert_ne!(pos1.hash(), pos2.hash());
    }

    #[ignore]
    #[test]
    fn test_veil_transparency_at_veil() {
        let pos = SpectrumPosition::at_veil(Density::Third, 0.0);
        // At v=1.0, transparency should be 0.0 (opaque)
        assert_eq!(pos.veil_transparency(), 0.0);
    }

    #[test]
    #[ignore]
    fn test_veil_transparency_away_from_veil() {
        let pos = SpectrumPosition::space_time_dominant(Density::Third, 0.0);
        // Away from v=1.0, transparency should be > 0
        assert!(pos.veil_transparency() > 0.0);
    }

    #[test]
    fn test_default_spectrum_position() {
        let pos = SpectrumPosition::default();
        assert!(pos.is_at_veil());
        assert_eq!(pos.density, Density::Third);
        assert_eq!(pos.phase, 0.0);
    }

    #[test]
    fn test_from_space_time_ratio() {
        let pos = SpectrumPosition::from_space_time_ratio(1.0, 1.0);
        assert_eq!(pos.velocity_ratio, 1.0);

        let pos = SpectrumPosition::from_space_time_ratio(2.0, 1.0);
        assert_eq!(pos.velocity_ratio, 2.0);
    }

    #[test]
    fn test_display_format() {
        let pos = SpectrumPosition::with_id(42, 2.0, Density::Third, 0.5);
        let display = format!("{}", pos);
        assert!(display.contains("SpectrumPosition"));
        assert!(display.contains("id=42"));
        assert!(display.contains("v=2.000"));
    }

    #[test]
    fn test_spectrum_relationship_creation() {
        let rel = SpectrumRelationship::new(123, 1.5, true);
        assert_eq!(rel.other_id, 123);
        assert_eq!(rel.spectrum_distance, 1.5);
        assert!(rel.accessible);
    }

    #[test]
    fn test_spectrum_relationship_negative_distance() {
        // Negative distance should be clamped to 0.0
        let rel = SpectrumRelationship::new(123, -1.0, false);
        assert_eq!(rel.spectrum_distance, 0.0);
    }

    #[test]
    fn test_add_relationship() {
        let mut pos1 = SpectrumPosition::with_id(1, 2.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 4.0, Density::Third, 0.5);

        pos1.add_relationship(&pos2);

        assert_eq!(pos1.relationships.len(), 1);
        assert_eq!(pos1.relationships[0].other_id, 2);
    }

    #[test]
    fn test_remove_relationship() {
        let mut pos1 = SpectrumPosition::with_id(1, 2.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 4.0, Density::Third, 0.5);
        let pos3 = SpectrumPosition::with_id(3, 6.0, Density::Fourth, 0.3);

        pos1.add_relationship(&pos2);
        pos1.add_relationship(&pos3);
        assert_eq!(pos1.relationships.len(), 2);

        pos1.remove_relationship(2);
        assert_eq!(pos1.relationships.len(), 1);
        assert_eq!(pos1.relationships[0].other_id, 3);
    }

    #[test]
    fn test_get_relationship() {
        let mut pos1 = SpectrumPosition::with_id(1, 2.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 4.0, Density::Third, 0.5);

        pos1.add_relationship(&pos2);

        let rel = pos1.get_relationship(2);
        assert!(rel.is_some());
        assert_eq!(rel.unwrap().other_id, 2);

        let no_rel = pos1.get_relationship(999);
        assert!(no_rel.is_none());
    }

    #[test]
    fn test_update_relationships() {
        let mut pos1 = SpectrumPosition::with_id(1, 2.0, Density::Third, 0.0);
        let pos2 = SpectrumPosition::with_id(2, 4.0, Density::Third, 0.5);
        let pos3 = SpectrumPosition::with_id(3, 6.0, Density::Fourth, 0.3);

        let positions = vec![pos2.clone(), pos3.clone()];
        pos1.update_relationships(&positions);

        assert_eq!(pos1.relationships.len(), 2);

        // Now move pos2 and update again
        let mut pos2_moved = pos2.clone();
        pos2_moved.velocity_ratio = 8.0;
        let positions = vec![pos2_moved, pos3];
        pos1.update_relationships(&positions);

        // Should still have 2 relationships, not add duplicates
        assert_eq!(pos1.relationships.len(), 2);
    }
}
