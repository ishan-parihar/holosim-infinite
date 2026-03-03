//! Space/Time ↔ Time/Space Coordinate Transformation
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "At the Veil (v=1), a coordinate transformation occurs:
//!  Space/Time (3D space, 1D time) ↔ Time/Space (1D space, 3D time)
//!
//!  This is not just a mathematical transformation but a fundamental shift in
//!  how reality is experienced. In Space/Time, entities experience linear time
//!  and navigate 3D space. In Time/Space, time becomes navigable and space
//!  becomes more 'linear' or unified."
//!
//! KEY INSIGHT: The transformation at v=1 is a REALITY PHASE TRANSITION.
//! It's not just coordinate rotation - it's a change in the structure of experience.
//!
//! Mathematical basis (from Larson's Reciprocal System):
//! - Space/Time: s³/t (3D space, 1D time)
//! - Time/Space: t³/s (3D time, 1D space)
//! - At v=1: s/t = t/s = 1, both aspects are equally present

use super::{SpectrumSide, VelocityRatio, VEIL_POSITION};
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpaceTimeCoordinates {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub t: Float,
}

impl SpaceTimeCoordinates {
    pub fn new(x: Float, y: Float, z: Float, t: Float) -> Self {
        Self { x, y, z, t }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn at_time(t: Float) -> Self {
        Self::new(0.0, 0.0, 0.0, t)
    }

    pub fn at_position(x: Float, y: Float, z: Float) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn spatial_magnitude(&self) -> Float {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn spatial_distance(&self, other: &Self) -> Float {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn temporal_distance(&self, other: &Self) -> Float {
        (self.t - other.t).abs()
    }

    pub fn spacetime_interval(&self, other: &Self) -> Float {
        let spatial = self.spatial_distance(other);
        let temporal = self.temporal_distance(other);

        // Minkowski-like interval: ds² = dx² + dy² + dz² - c²dt²
        // We use c = 1 for simplicity
        spatial.powi(2) - temporal.powi(2)
    }

    pub fn is_timelike(&self, other: &Self) -> bool {
        self.spacetime_interval(other) < 0.0
    }

    pub fn is_spacelike(&self, other: &Self) -> bool {
        self.spacetime_interval(other) > 0.0
    }

    pub fn is_lightlike(&self, other: &Self) -> bool {
        (self.spacetime_interval(other)).abs() < 1e-10
    }
}

impl Default for SpaceTimeCoordinates {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpaceCoordinates {
    pub s: Float,
    pub tx: Float,
    pub ty: Float,
    pub tz: Float,
}

impl TimeSpaceCoordinates {
    pub fn new(s: Float, tx: Float, ty: Float, tz: Float) -> Self {
        Self { s, tx, ty, tz }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn at_space(s: Float) -> Self {
        Self::new(s, 0.0, 0.0, 0.0)
    }

    pub fn at_time(tx: Float, ty: Float, tz: Float) -> Self {
        Self::new(0.0, tx, ty, tz)
    }

    pub fn temporal_magnitude(&self) -> Float {
        (self.tx.powi(2) + self.ty.powi(2) + self.tz.powi(2)).sqrt()
    }

    pub fn spatial_distance(&self, other: &Self) -> Float {
        (self.s - other.s).abs()
    }

    pub fn temporal_distance(&self, other: &Self) -> Float {
        ((self.tx - other.tx).powi(2) + (self.ty - other.ty).powi(2) + (self.tz - other.tz).powi(2))
            .sqrt()
    }
}

impl Default for TimeSpaceCoordinates {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone)]
pub struct CoordinateTransform {
    velocity_ratio: VelocityRatio,
    transformation_factor: Float,
    blend_weight: Float,
}

impl CoordinateTransform {
    pub fn new(velocity_ratio: VelocityRatio) -> Self {
        let transformation_factor = Self::calculate_transformation_factor(&velocity_ratio);
        let blend_weight = Self::calculate_blend_weight(&velocity_ratio);

        Self {
            velocity_ratio,
            transformation_factor,
            blend_weight,
        }
    }

    fn calculate_transformation_factor(v: &VelocityRatio) -> Float {
        // Transformation factor peaks at v=1 and falls off on either side
        let distance_from_veil = v.distance_to_veil();
        (-distance_from_veil.powi(2) / 0.5).exp()
    }

    fn calculate_blend_weight(v: &VelocityRatio) -> Float {
        // Blend weight: 0.0 = pure Space/Time, 1.0 = pure Time/Space
        if v.value <= VEIL_POSITION {
            v.time_space_factor()
        } else {
            1.0 - v.space_time_factor() * 0.5
        }
    }

    /// Transform from Space/Time to Time/Space coordinates
    ///
    /// At v=1, the transformation is:
    /// (x, y, z, t) → (s, tx, ty, tz)
    ///
    /// The exact mapping depends on the transformation_factor.
    /// Near the veil, there's a smooth blend between the two coordinate systems.
    pub fn to_time_space(&self, st: &SpaceTimeCoordinates) -> TimeSpaceCoordinates {
        match self.velocity_ratio.side() {
            SpectrumSide::SpaceTime => {
                // v > 1: Mostly Space/Time, partial transformation
                self.partial_to_time_space(st, 1.0 - self.blend_weight)
            }
            SpectrumSide::AtVeil => {
                // v ≈ 1: Full transformation active
                self.full_to_time_space(st)
            }
            SpectrumSide::TimeSpace => {
                // v < 1: Already Time/Space dominant
                self.full_to_time_space(st)
            }
        }
    }

    /// Transform from Time/Space to Space/Time coordinates
    pub fn to_space_time(&self, ts: &TimeSpaceCoordinates) -> SpaceTimeCoordinates {
        match self.velocity_ratio.side() {
            SpectrumSide::SpaceTime => {
                // v > 1: Mostly Space/Time, partial transformation
                self.full_to_space_time(ts)
            }
            SpectrumSide::AtVeil => {
                // v ≈ 1: Full transformation active
                self.full_to_space_time(ts)
            }
            SpectrumSide::TimeSpace => {
                // v < 1: Time/Space dominant
                self.partial_to_space_time(ts, self.blend_weight)
            }
        }
    }

    fn full_to_time_space(&self, st: &SpaceTimeCoordinates) -> TimeSpaceCoordinates {
        // Full transformation: (x, y, z, t) → (s, tx, ty, tz)
        // The key insight: spatial dimensions become temporal and vice versa
        TimeSpaceCoordinates {
            s: st.t,  // Time becomes the single spatial dimension
            tx: st.x, // x becomes a temporal dimension
            ty: st.y, // y becomes a temporal dimension
            tz: st.z, // z becomes a temporal dimension
        }
    }

    fn partial_to_time_space(
        &self,
        st: &SpaceTimeCoordinates,
        factor: Float,
    ) -> TimeSpaceCoordinates {
        // Partial transformation based on factor (0.0 to 1.0)
        let ts = self.full_to_time_space(st);
        TimeSpaceCoordinates {
            s: st.t * factor + st.spatial_magnitude() * (1.0 - factor) * 0.1,
            tx: st.x * factor + st.t * (1.0 - factor) * 0.1,
            ty: st.y * factor + st.t * (1.0 - factor) * 0.1,
            tz: st.z * factor + st.t * (1.0 - factor) * 0.1,
        }
    }

    fn full_to_space_time(&self, ts: &TimeSpaceCoordinates) -> SpaceTimeCoordinates {
        // Full transformation: (s, tx, ty, tz) → (x, y, z, t)
        SpaceTimeCoordinates {
            x: ts.tx, // Temporal dimension becomes spatial
            y: ts.ty, // Temporal dimension becomes spatial
            z: ts.tz, // Temporal dimension becomes spatial
            t: ts.s,  // Single spatial dimension becomes temporal
        }
    }

    fn partial_to_space_time(
        &self,
        ts: &TimeSpaceCoordinates,
        factor: Float,
    ) -> SpaceTimeCoordinates {
        // Partial transformation based on factor (0.0 to 1.0)
        let st = self.full_to_space_time(ts);
        SpaceTimeCoordinates {
            x: ts.tx * factor + ts.s * (1.0 - factor) * 0.1,
            y: ts.ty * factor + ts.s * (1.0 - factor) * 0.1,
            z: ts.tz * factor + ts.s * (1.0 - factor) * 0.1,
            t: ts.s * factor + ts.temporal_magnitude() * (1.0 - factor) * 0.1,
        }
    }

    /// Get the perception modifier for Space/Time experience
    ///
    /// Returns how "Space/Time-like" the experience is (0.0 to 1.0)
    pub fn space_time_perception(&self) -> Float {
        match self.velocity_ratio.side() {
            SpectrumSide::SpaceTime => 1.0 - self.blend_weight,
            SpectrumSide::AtVeil => 0.5,
            SpectrumSide::TimeSpace => self.blend_weight,
        }
    }

    /// Get the perception modifier for Time/Space experience
    ///
    /// Returns how "Time/Space-like" the experience is (0.0 to 1.0)
    pub fn time_space_perception(&self) -> Float {
        1.0 - self.space_time_perception()
    }

    /// Calculate the "temporal access" level
    ///
    /// In Time/Space, entities can access multiple timelines
    pub fn temporal_access(&self) -> Float {
        self.time_space_perception() * self.velocity_ratio.time_space_factor()
    }

    /// Calculate the "spatial access" level
    ///
    /// In Space/Time, entities can navigate 3D space
    pub fn spatial_access(&self) -> Float {
        self.space_time_perception() * self.velocity_ratio.space_time_factor()
    }

    /// Apply veil distortion to coordinates
    ///
    /// Near the veil, coordinates become "fuzzy" or uncertain
    pub fn apply_veil_distortion(&self, st: &SpaceTimeCoordinates) -> SpaceTimeCoordinates {
        let distortion = self.transformation_factor * 0.1;

        SpaceTimeCoordinates {
            x: st.x + distortion * st.x.sin(),
            y: st.y + distortion * st.y.sin(),
            z: st.z + distortion * st.z.sin(),
            t: st.t + distortion * st.t.sin(),
        }
    }

    pub fn velocity_ratio(&self) -> &VelocityRatio {
        &self.velocity_ratio
    }

    pub fn transformation_factor(&self) -> Float {
        self.transformation_factor
    }

    pub fn blend_weight(&self) -> Float {
        self.blend_weight
    }

    pub fn update(&mut self, velocity_ratio: VelocityRatio) {
        self.velocity_ratio = velocity_ratio;
        self.transformation_factor = Self::calculate_transformation_factor(&self.velocity_ratio);
        self.blend_weight = Self::calculate_blend_weight(&self.velocity_ratio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_time_coordinates() {
        let st = SpaceTimeCoordinates::new(1.0, 2.0, 3.0, 4.0);
        assert!((st.spatial_magnitude() - 14.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_space_time_distance() {
        let a = SpaceTimeCoordinates::new(0.0, 0.0, 0.0, 0.0);
        let b = SpaceTimeCoordinates::new(3.0, 4.0, 0.0, 0.0);

        assert!((a.spatial_distance(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_spacetime_interval() {
        let a = SpaceTimeCoordinates::origin();
        let b = SpaceTimeCoordinates::new(3.0, 0.0, 0.0, 5.0);

        let interval = a.spacetime_interval(&b);
        // ds² = 9 - 25 = -16 (timelike)
        assert!((interval - (-16.0)).abs() < 1e-10);
        assert!(a.is_timelike(&b));
    }

    #[test]
    fn test_time_space_coordinates() {
        let ts = TimeSpaceCoordinates::new(1.0, 2.0, 3.0, 4.0);
        assert!((ts.temporal_magnitude() - 29.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_transform_creation() {
        let transform = CoordinateTransform::new(VelocityRatio::at_veil());
        assert!(transform.transformation_factor() > 0.5);
    }

    #[test]
    fn test_full_transformation() {
        let transform = CoordinateTransform::new(VelocityRatio::at_veil());
        let st = SpaceTimeCoordinates::new(1.0, 2.0, 3.0, 10.0);

        let ts = transform.to_time_space(&st);

        // Time should become spatial
        assert!((ts.s - 10.0).abs() < 1e-10);
        assert!((ts.tx - 1.0).abs() < 1e-10);
        assert!((ts.ty - 2.0).abs() < 1e-10);
        assert!((ts.tz - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_reverse_transformation() {
        let transform = CoordinateTransform::new(VelocityRatio::at_veil());
        let st = SpaceTimeCoordinates::new(1.0, 2.0, 3.0, 10.0);

        let ts = transform.to_time_space(&st);
        let st_back = transform.to_space_time(&ts);

        // Should be reversible
        assert!((st_back.x - st.x).abs() < 1e-10);
        assert!((st_back.y - st.y).abs() < 1e-10);
        assert!((st_back.z - st.z).abs() < 1e-10);
        assert!((st_back.t - st.t).abs() < 1e-10);
    }

    #[test]
    fn test_transformation_factor() {
        let at_veil = CoordinateTransform::new(VelocityRatio::at_veil());
        let far_st = CoordinateTransform::new(VelocityRatio::space_time_dominant());
        let far_ts = CoordinateTransform::new(VelocityRatio::time_space_dominant());

        // At veil should have highest transformation factor
        assert!(at_veil.transformation_factor() > far_st.transformation_factor());
        assert!(at_veil.transformation_factor() > far_ts.transformation_factor());
    }

    #[test]
    fn test_perception_levels() {
        let st_transform = CoordinateTransform::new(VelocityRatio::space_time_dominant());
        let ts_transform = CoordinateTransform::new(VelocityRatio::time_space_dominant());

        // Space/Time dominant should have high spatial access
        assert!(st_transform.spatial_access() > ts_transform.spatial_access());

        // Time/Space dominant should have high temporal access
        assert!(ts_transform.temporal_access() > st_transform.temporal_access());
    }

    #[test]
    fn test_veil_distortion() {
        let transform = CoordinateTransform::new(VelocityRatio::at_veil());
        let st = SpaceTimeCoordinates::new(1.0, 2.0, 3.0, 4.0);

        let distorted = transform.apply_veil_distortion(&st);

        // Distorted should be close to original but slightly modified
        assert!((distorted.x - st.x).abs() < 0.5);
        assert!((distorted.y - st.y).abs() < 0.5);
    }

    #[test]
    fn test_partial_transformation() {
        let transform = CoordinateTransform::new(VelocityRatio::new(1.5));
        let st = SpaceTimeCoordinates::new(1.0, 2.0, 3.0, 10.0);

        let ts = transform.to_time_space(&st);

        // Should be a blend, not full transformation
        // The single spatial dimension should have some contribution from t
        assert!(ts.s > 0.0);
    }

    #[test]
    fn test_update_transform() {
        let mut transform = CoordinateTransform::new(VelocityRatio::space_time_dominant());
        let initial_factor = transform.transformation_factor();

        transform.update(VelocityRatio::at_veil());

        // At veil should have higher transformation factor
        assert!(transform.transformation_factor() > initial_factor);
    }
}
