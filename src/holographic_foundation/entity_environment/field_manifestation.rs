//! Entity Position from Field Manifestation
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Entity Position = WHERE entity's field configuration manifests
//!  Terrain Interaction = Field energy exchange with landscape field"
//!
//! # Key Insight
//!
//! Entity position is NOT arbitrary - it emerges from where the entity's
//! field configuration resonates most strongly with the environmental field.
//! Position is a MANIFESTATION POINT, not a coordinate assignment.

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionStability {
    Unstable,
    Fluctuating,
    Stable,
    Anchored,
    Permanent,
}

impl PositionStability {
    pub fn from_coherence(coherence: Float) -> Self {
        if coherence < 0.3 {
            PositionStability::Unstable
        } else if coherence < 0.5 {
            PositionStability::Fluctuating
        } else if coherence < 0.7 {
            PositionStability::Stable
        } else if coherence < 0.9 {
            PositionStability::Anchored
        } else {
            PositionStability::Permanent
        }
    }

    pub fn drift_rate(&self) -> Float {
        match self {
            PositionStability::Unstable => 1.0,
            PositionStability::Fluctuating => 0.5,
            PositionStability::Stable => 0.1,
            PositionStability::Anchored => 0.01,
            PositionStability::Permanent => 0.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PositionStability::Unstable => "Position rapidly shifting",
            PositionStability::Fluctuating => "Position varies noticeably",
            PositionStability::Stable => "Position maintains generally",
            PositionStability::Anchored => "Position strongly fixed",
            PositionStability::Permanent => "Position immutable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestationConditions {
    Optimal,
    Favorable,
    Neutral,
    Unfavorable,
    Hostile,
    Impossible,
}

impl ManifestationConditions {
    pub fn field_resonance_threshold(&self) -> Float {
        match self {
            ManifestationConditions::Optimal => 0.9,
            ManifestationConditions::Favorable => 0.7,
            ManifestationConditions::Neutral => 0.5,
            ManifestationConditions::Unfavorable => 0.3,
            ManifestationConditions::Hostile => 0.1,
            ManifestationConditions::Impossible => 0.0,
        }
    }

    pub fn manifestation_probability(&self) -> Float {
        match self {
            ManifestationConditions::Optimal => 1.0,
            ManifestationConditions::Favorable => 0.8,
            ManifestationConditions::Neutral => 0.5,
            ManifestationConditions::Unfavorable => 0.2,
            ManifestationConditions::Hostile => 0.05,
            ManifestationConditions::Impossible => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldManifestationPoint {
    pub position: Position3D,
    pub field_resonance: Float,
    pub environmental_coherence: Float,
    pub stability: PositionStability,
    pub manifestation_strength: Float,
    pub attractor_basin_id: Option<u64>,
}

impl FieldManifestationPoint {
    pub fn new(position: Position3D, field_resonance: Float) -> Self {
        let stability = PositionStability::from_coherence(field_resonance);
        let environmental_coherence = field_resonance * 0.9;
        let manifestation_strength = field_resonance * environmental_coherence;

        Self {
            position,
            field_resonance,
            environmental_coherence,
            stability,
            manifestation_strength,
            attractor_basin_id: None,
        }
    }

    pub fn with_attractor(mut self, basin_id: u64) -> Self {
        self.attractor_basin_id = Some(basin_id);
        self.stability = PositionStability::Anchored;
        self
    }

    pub fn check_conditions(&self) -> ManifestationConditions {
        let resonance = self.field_resonance;

        if resonance >= 0.9 {
            ManifestationConditions::Optimal
        } else if resonance >= 0.7 {
            ManifestationConditions::Favorable
        } else if resonance >= 0.5 {
            ManifestationConditions::Neutral
        } else if resonance >= 0.3 {
            ManifestationConditions::Unfavorable
        } else if resonance >= 0.1 {
            ManifestationConditions::Hostile
        } else {
            ManifestationConditions::Impossible
        }
    }

    pub fn can_manifest(&self) -> bool {
        self.check_conditions().manifestation_probability() > 0.0
    }

    pub fn drift_position(&mut self, dt: Float, random_factor: Float) {
        let drift_rate = self.stability.drift_rate();
        let drift = drift_rate * dt * random_factor * 0.1;

        self.position.x += drift * (random_factor - 0.5).sin();
        self.position.y += drift * (random_factor * 2.0 - 1.0).cos();
        self.position.z += drift * (random_factor * 0.5).sin();
    }

    pub fn update_resonance(&mut self, new_resonance: Float) {
        self.field_resonance = new_resonance;
        self.environmental_coherence = new_resonance * 0.9;
        self.stability = PositionStability::from_coherence(new_resonance);
        self.manifestation_strength = self.field_resonance * self.environmental_coherence;
    }
}

#[derive(Debug, Clone)]
pub struct ManifestationResult {
    pub success: bool,
    pub position: Option<Position3D>,
    pub conditions: ManifestationConditions,
    pub resonance_achieved: Float,
    pub energy_cost: Float,
}

impl ManifestationResult {
    pub fn success(point: FieldManifestationPoint) -> Self {
        Self {
            success: true,
            position: Some(point.position),
            conditions: point.check_conditions(),
            resonance_achieved: point.field_resonance,
            energy_cost: 1.0 - point.field_resonance,
        }
    }

    pub fn failure(required_resonance: Float) -> Self {
        Self {
            success: false,
            position: None,
            conditions: ManifestationConditions::Impossible,
            resonance_achieved: 0.0,
            energy_cost: required_resonance * 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityGrounding {
    pub entity_id: u64,
    pub manifestation_point: FieldManifestationPoint,
    pub grounding_strength: Float,
    pub history: Vec<Position3D>,
    pub time_at_position: Float,
    pub movement_frequency: Float,
}

impl EntityGrounding {
    pub fn new(entity_id: u64, position: Position3D, resonance: Float) -> Self {
        let manifestation_point = FieldManifestationPoint::new(position, resonance);

        Self {
            entity_id,
            manifestation_point,
            grounding_strength: resonance,
            history: vec![position],
            time_at_position: 0.0,
            movement_frequency: 0.0,
        }
    }

    pub fn position(&self) -> Position3D {
        self.manifestation_point.position
    }

    pub fn stability(&self) -> PositionStability {
        self.manifestation_point.stability
    }

    pub fn update(&mut self, dt: Float, environmental_resonance: Float, random_factor: Float) {
        let resonance_change =
            (environmental_resonance - self.manifestation_point.field_resonance) * 0.1 * dt;
        let new_resonance =
            (self.manifestation_point.field_resonance + resonance_change).clamp(0.0, 1.0);

        self.manifestation_point.update_resonance(new_resonance);

        if self.manifestation_point.stability.drift_rate() > 0.0 {
            self.manifestation_point.drift_position(dt, random_factor);
        }

        let new_position = self.manifestation_point.position;
        let last_position = self.history.last().copied().unwrap_or(new_position);

        if new_position.distance(&last_position) > 0.01 {
            self.history.push(new_position);
            if self.history.len() > 100 {
                self.history.remove(0);
            }
            self.time_at_position = 0.0;
            self.movement_frequency = 1.0 / (self.time_at_position + 0.1);
        } else {
            self.time_at_position += dt;
        }

        self.grounding_strength = self.manifestation_point.manifestation_strength;
    }

    pub fn reassign_position(&mut self, new_position: Position3D, new_resonance: Float) {
        self.manifestation_point = FieldManifestationPoint::new(new_position, new_resonance);
        self.history.push(new_position);
        self.time_at_position = 0.0;
    }

    pub fn distance_traveled(&self) -> Float {
        if self.history.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.history.len() {
            total += self.history[i].distance(&self.history[i - 1]);
        }
        total
    }

    pub fn average_position(&self) -> Position3D {
        if self.history.is_empty() {
            return self.manifestation_point.position;
        }

        let n = self.history.len() as Float;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for pos in &self.history {
            x += pos.x;
            y += pos.y;
            z += pos.z;
        }

        Position3D::new(x / n, y / n, z / n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_stability_from_coherence() {
        assert_eq!(
            PositionStability::from_coherence(0.2),
            PositionStability::Unstable
        );
        assert_eq!(
            PositionStability::from_coherence(0.4),
            PositionStability::Fluctuating
        );
        assert_eq!(
            PositionStability::from_coherence(0.6),
            PositionStability::Stable
        );
        assert_eq!(
            PositionStability::from_coherence(0.8),
            PositionStability::Anchored
        );
        assert_eq!(
            PositionStability::from_coherence(0.95),
            PositionStability::Permanent
        );
    }

    #[test]
    fn test_position_stability_drift_rate() {
        assert!(PositionStability::Unstable.drift_rate() > PositionStability::Stable.drift_rate());
        assert_eq!(PositionStability::Permanent.drift_rate(), 0.0);
    }

    #[test]
    fn test_manifestation_conditions_probability() {
        assert_eq!(
            ManifestationConditions::Optimal.manifestation_probability(),
            1.0
        );
        assert_eq!(
            ManifestationConditions::Impossible.manifestation_probability(),
            0.0
        );
    }

    #[test]
    fn test_field_manifestation_point_creation() {
        let point = FieldManifestationPoint::new(Position3D::new(1.0, 2.0, 3.0), 0.8);
        assert!(point.field_resonance > 0.0);
        assert!(point.can_manifest());
    }

    #[test]
    fn test_field_manifestation_point_conditions() {
        let optimal = FieldManifestationPoint::new(Position3D::new(0.0, 0.0, 0.0), 0.95);
        assert_eq!(optimal.check_conditions(), ManifestationConditions::Optimal);

        let hostile = FieldManifestationPoint::new(Position3D::new(0.0, 0.0, 0.0), 0.15);
        assert_eq!(hostile.check_conditions(), ManifestationConditions::Hostile);
    }

    #[test]
    fn test_manifestation_result_success() {
        let point = FieldManifestationPoint::new(Position3D::new(0.0, 0.0, 0.0), 0.8);
        let result = ManifestationResult::success(point);
        assert!(result.success);
        assert!(result.position.is_some());
    }

    #[test]
    fn test_manifestation_result_failure() {
        let result = ManifestationResult::failure(0.9);
        assert!(!result.success);
        assert!(result.position.is_none());
    }

    #[test]
    fn test_entity_grounding_creation() {
        let grounding = EntityGrounding::new(1, Position3D::new(0.0, 0.0, 0.0), 0.8);
        assert_eq!(grounding.entity_id, 1);
        assert!(grounding.grounding_strength > 0.0);
    }

    #[test]
    fn test_entity_grounding_update() {
        let mut grounding = EntityGrounding::new(1, Position3D::new(0.0, 0.0, 0.0), 0.8);
        grounding.update(1.0, 0.85, 0.5);
        assert!(grounding.time_at_position > 0.0);
    }

    #[test]
    fn test_entity_grounding_reassign() {
        let mut grounding = EntityGrounding::new(1, Position3D::new(0.0, 0.0, 0.0), 0.8);
        grounding.reassign_position(Position3D::new(5.0, 5.0, 5.0), 0.9);
        assert_eq!(grounding.history.len(), 2);
    }

    #[test]
    fn test_entity_grounding_distance() {
        let mut grounding = EntityGrounding::new(1, Position3D::new(0.0, 0.0, 0.0), 0.8);
        grounding.history.push(Position3D::new(1.0, 0.0, 0.0));
        grounding.history.push(Position3D::new(1.0, 1.0, 0.0));
        let distance = grounding.distance_traveled();
        assert!(distance > 0.0);
    }

    #[test]
    fn test_entity_grounding_average_position() {
        let grounding = EntityGrounding::new(1, Position3D::new(0.0, 0.0, 0.0), 0.8);
        let avg = grounding.average_position();
        assert_eq!(avg.x, 0.0);
    }
}
