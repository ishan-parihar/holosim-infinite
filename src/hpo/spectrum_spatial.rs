//! Spectrum-Driven Space (Phase F2)
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V2.md:
//! "Implement v=s/t and v=t/s as the FUNDAMENTAL creators of spatial structure, not just abstract ratios."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Space/Time ↔ Time/Space spectrum is a CONTINUUM with the Veil at v=1 as a STRUCTURAL FEATURE"
//!
//! This module implements:
//! - Spectrum-Spatial Mapping: Spectrum position determines spatial dimensionality
//! - Veil as Spatial Boundary: The veil is an actual spatial phenomenon
//! - Coordinate Transformation: Entities near veil experience different coordinate systems
//!
//! KEY PRINCIPLE: Space is not a fixed container - it EMERGES from the spectrum.
//! Below v=1: 3D space with 1D time (classical physics)
//! Above v=1: 1D space with 3D time (spiritual experience)

use super::field_state::Float;
use super::spatial_field::{Position3D, SpatialBounds};

/// Spatial configuration derived from spectrum position
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > v = s/t creates 3D space with 1D time
/// > v = t/s creates 1D space with 3D time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialConfig {
    /// Number of spatial dimensions (1-3)
    pub spatial_dimensions: usize,

    /// Number of time dimensions (1-3)
    pub time_dimensionality: usize,

    /// Whether this is the physical reality (below veil)
    pub is_physical: bool,

    /// Whether this is the spiritual reality (above veil)
    pub is_spiritual: bool,
}

impl SpatialConfig {
    /// Create config for Space/Time (v < 1)
    pub fn space_time() -> Self {
        SpatialConfig {
            spatial_dimensions: 3,
            time_dimensionality: 1,
            is_physical: true,
            is_spiritual: false,
        }
    }

    /// Create config for Time/Space (v > 1)
    pub fn time_space() -> Self {
        SpatialConfig {
            spatial_dimensions: 1,
            time_dimensionality: 3,
            is_physical: false,
            is_spiritual: true,
        }
    }

    /// Derive spatial config from spectrum position
    ///
    /// v < 1: Space/Time dominant (3D space, 1D time)
    /// v = 1: At the Veil (transition)
    /// v > 1: Time/Space dominant (1D space, 3D time)
    pub fn from_spectrum_position(spectrum_position: Float) -> Self {
        if spectrum_position < 1.0 {
            // Below veil: 3D space with 1D time
            SpatialConfig {
                spatial_dimensions: 3,
                time_dimensionality: 1,
                is_physical: true,
                is_spiritual: false,
            }
        } else if spectrum_position > 1.0 {
            // Above veil: 1D space with 3D time
            SpatialConfig {
                spatial_dimensions: 1,
                time_dimensionality: 3,
                is_physical: false,
                is_spiritual: true,
            }
        } else {
            // At the veil: mixed state
            SpatialConfig {
                spatial_dimensions: 2,
                time_dimensionality: 2,
                is_physical: true,
                is_spiritual: true,
            }
        }
    }

    /// Create config from explicit spatial and time dimensions
    ///
    /// # Arguments
    /// * `spatial_dims` - Number of spatial dimensions (1-3)
    /// * `time_dims` - Number of time dimensions (1-3)
    pub fn from_spatial_dimensions(spatial_dims: usize, time_dims: usize) -> Self {
        let is_physical = spatial_dims >= time_dims;
        let is_spiritual = time_dims >= spatial_dims;
        SpatialConfig {
            spatial_dimensions: spatial_dims.clamp(1, 3),
            time_dimensionality: time_dims.clamp(1, 3),
            is_physical,
            is_spiritual,
        }
    }

    /// Get the effective dimension for a given axis
    pub fn effective_dimensions(&self) -> usize {
        self.spatial_dimensions + self.time_dimensionality
    }
}

/// The Veil as a Spatial Boundary
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > The Veil at v=1 is a STRUCTURAL FEATURE
///
/// The veil is not just a conceptual boundary - it has actual spatial extent
/// and determines how entities experience reality.
#[derive(Debug, Clone, Copy)]
pub struct VeilBoundary {
    /// Position on spectrum where veil exists (v = 1.0)
    pub position: Float,

    /// Thickness of the veil transition region
    pub thickness: Float,

    /// How permeable the veil is (0 = opaque, 1 = fully transparent)
    pub transparency: Float,

    /// Current state of the veil (how active the crossing is)
    pub activity: Float,
}

impl VeilBoundary {
    pub fn new() -> Self {
        VeilBoundary {
            position: 1.0,
            thickness: 0.2,
            transparency: 0.5,
            activity: 0.0,
        }
    }

    pub fn with_config(position: Float, thickness: Float, transparency: Float) -> Self {
        VeilBoundary {
            position,
            thickness,
            transparency,
            activity: 0.0,
        }
    }

    /// Check if a spectrum position is within the veil region
    pub fn is_in_veil(&self, spectrum_position: Float) -> bool {
        let dist = (spectrum_position - self.position).abs();
        dist <= self.thickness
    }

    /// Get the veil factor (0 = outside veil, 1 = at center of veil)
    pub fn veil_factor(&self, spectrum_position: Float) -> Float {
        if !self.is_in_veil(spectrum_position) {
            return 0.0;
        }

        let dist = (spectrum_position - self.position).abs();
        1.0 - (dist / self.thickness)
    }

    /// Get effective transparency at a position
    pub fn effective_transparency(&self, spectrum_position: Float) -> Float {
        if !self.is_in_veil(spectrum_position) {
            return if spectrum_position < self.position {
                1.0
            } else {
                self.transparency
            };
        }

        // Within veil: interpolation
        let factor = self.veil_factor(spectrum_position);
        self.transparency * factor + (1.0 - factor) * 0.5
    }

    /// Update veil activity (for animation/visualization)
    pub fn update_activity(&mut self, dt: Float) {
        self.activity = (self.activity + dt * 0.1).min(1.0);
    }
}

impl Default for VeilBoundary {
    fn default() -> Self {
        Self::new()
    }
}

/// Coordinate system type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoordinateType {
    /// Standard 3D space + 1D time: (x, y, z, t)
    Standard,
    /// Time-space: (t1, t2, t3, x) - time as navigable dimensions
    TimeSpace,
    /// At the veil: mixed coordinate system
    Transitional,
}

/// Transformed coordinates based on spectrum position
///
/// Entities near the veil experience coordinate transformation:
/// - Below veil: (x, y, z, t) - standard
/// - Above veil: (t1, t2, t3, x) - time as dimensions
#[derive(Debug, Clone)]
pub struct TransformedCoordinates {
    /// The actual spatial position
    pub spatial: Position3D,

    /// Time coordinates (1-3 depending on mode)
    pub time: [Float; 3],

    /// Coordinate system type
    pub coordinate_type: CoordinateType,

    /// How much transformation has occurred (0 = no transform, 1 = full)
    pub transformation_factor: Float,

    /// Original spectrum position
    pub spectrum_position: Float,
}

impl TransformedCoordinates {
    /// Create transformed coordinates from position and spectrum
    pub fn new(position: Position3D, spectrum_position: Float, veil: &VeilBoundary) -> Self {
        let config = SpatialConfig::from_spectrum_position(spectrum_position);
        let transformation_factor = veil.veil_factor(spectrum_position);

        let coordinate_type = if transformation_factor > 0.5 {
            CoordinateType::Transitional
        } else if config.is_spiritual {
            CoordinateType::TimeSpace
        } else {
            CoordinateType::Standard
        };

        // Initialize time coordinates based on mode
        let time = if config.is_spiritual {
            // Above veil: time becomes the spatial dimensions
            [position.x, position.y, position.z]
        } else {
            // Below veil: single time dimension
            [0.0, 0.0, 0.0]
        };

        TransformedCoordinates {
            spatial: position,
            time,
            coordinate_type,
            transformation_factor,
            spectrum_position,
        }
    }

    /// Get the "perceived" position based on coordinate type
    pub fn perceived_position(&self) -> Position3D {
        match self.coordinate_type {
            CoordinateType::Standard => self.spatial,
            CoordinateType::TimeSpace => {
                // Time dimensions become perceived as space
                Position3D::new(self.time[0], self.time[1], self.time[2])
            }
            CoordinateType::Transitional => {
                // Interpolate based on transformation factor
                let t = self.transformation_factor;
                Position3D::new(
                    self.spatial.x * (1.0 - t) + self.time[0] * t,
                    self.spatial.y * (1.0 - t) + self.time[1] * t,
                    self.spatial.z * (1.0 - t) + self.time[2] * t,
                )
            }
        }
    }
}

/// Spectrum-Spatial Dynamics - The core of Phase F2
///
/// This implements the principle that the spectrum (v=s/t) is not just a label
/// but actually DETERMINES the spatial structure of reality.
pub struct SpectrumSpatialDynamics {
    /// Configuration
    pub config: SpectrumSpatialConfig,

    /// The veil boundary
    pub veil: VeilBoundary,

    /// Statistics
    pub statistics: SpectrumSpatialStatistics,
}

impl SpectrumSpatialDynamics {
    pub fn new(config: SpectrumSpatialConfig) -> Self {
        SpectrumSpatialDynamics {
            veil: VeilBoundary::with_config(
                config.veil_position,
                config.veil_thickness,
                config.veil_transparency,
            ),
            config,
            statistics: SpectrumSpatialStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(SpectrumSpatialConfig::default())
    }

    /// Derive spatial configuration from spectrum position
    ///
    /// This is the KEY FUNCTION that implements spectrum-driven space
    pub fn derive_spatial_structure(&self, spectrum_position: Float) -> SpatialConfig {
        SpatialConfig::from_spectrum_position(spectrum_position)
    }

    /// Get spatial bounds modified by spectrum position
    ///
    /// Below veil: Full 3D bounds
    /// At veil: Compressed bounds (reality is "thinner")
    /// Above veil: Extended bounds in time dimensions
    pub fn derive_spatial_bounds(
        &self,
        base_bounds: &SpatialBounds,
        spectrum_position: Float,
    ) -> SpatialBounds {
        let config = self.derive_spatial_structure(spectrum_position);
        let veil_factor = self.veil.veil_factor(spectrum_position);

        let mut extent = base_bounds.extent;

        if config.is_spiritual {
            // Above veil: expand "time" dimensions (which become spatial-like)
            extent.x *= 3.0;
        } else if veil_factor > 0.0 {
            // At veil: compress space
            let compression = 1.0 - veil_factor * 0.5;
            extent.x *= compression;
            extent.y *= compression;
            extent.z *= compression;
        }

        SpatialBounds {
            center: base_bounds.center,
            extent,
        }
    }

    /// Transform coordinates based on spectrum position
    pub fn transform_coordinates(
        &self,
        position: Position3D,
        spectrum_position: Float,
    ) -> TransformedCoordinates {
        TransformedCoordinates::new(position, spectrum_position, &self.veil)
    }

    /// Check if an entity can "cross" the veil
    pub fn can_cross_veil(&self, spectrum_position: Float) -> bool {
        self.veil.is_in_veil(spectrum_position)
            && self.veil.effective_transparency(spectrum_position) > 0.3
    }

    /// Get the "perceived" position for an entity based on its spectrum position
    ///
    /// This is what the entity actually "sees" as its position
    pub fn get_perceived_position(
        &self,
        position: Position3D,
        spectrum_position: Float,
    ) -> Position3D {
        let transformed = self.transform_coordinates(position, spectrum_position);
        transformed.perceived_position()
    }

    /// Calculate how far a position is from the veil (in spectrum space)
    pub fn distance_to_veil(&self, spectrum_position: Float) -> Float {
        (spectrum_position - self.veil.position).abs()
    }

    /// Update the dynamics
    pub fn step(&mut self, dt: Float) {
        self.veil.update_activity(dt);

        // Update statistics
        self.statistics.veil_activity = self.veil.activity;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &SpectrumSpatialStatistics {
        &self.statistics
    }
}

/// Configuration for spectrum-spatial dynamics
#[derive(Debug, Clone)]
pub struct SpectrumSpatialConfig {
    /// Veil position (v = 1.0)
    pub veil_position: Float,

    /// Veil thickness
    pub veil_thickness: Float,

    /// Base veil transparency
    pub veil_transparency: Float,

    /// Enable coordinate transformation
    pub enable_transformation: bool,

    /// Enable veil crossing events
    pub enable_veil_crossing: bool,
}

impl Default for SpectrumSpatialConfig {
    fn default() -> Self {
        SpectrumSpatialConfig {
            veil_position: 1.0,
            veil_thickness: 0.2,
            veil_transparency: 0.5,
            enable_transformation: true,
            enable_veil_crossing: true,
        }
    }
}

/// Statistics for spectrum-spatial dynamics
#[derive(Debug, Clone, Default)]
pub struct SpectrumSpatialStatistics {
    pub veil_activity: Float,
    pub entities_below_veil: usize,
    pub entities_at_veil: usize,
    pub entities_above_veil: usize,
    pub veil_crossings: usize,
}

/// Spatial projection helpers for rendering
pub struct SpatialProjection;

impl SpatialProjection {
    /// Project a position to 2D screen coordinates
    /// Takes into account spectrum-driven space
    pub fn project_3d_to_2d(
        position: &Position3D,
        _view_matrix: &[Float; 16],
        screen_width: Float,
        screen_height: Float,
    ) -> (Float, Float, Float) {
        // Simple perspective projection
        let fov = 60.0 * std::f64::consts::PI / 180.0;
        let aspect = screen_width / screen_height;
        let z_near = 0.1;
        let _z_far = 10000.0;

        // Apply view transform (simplified)
        let x = position.x;
        let y = position.y;
        let z = position.z;

        // Perspective divide
        let perspective_z = z.max(z_near);
        let scale = 1.0 / (perspective_z.tan() * 0.5 * fov);

        let screen_x = (x * scale / aspect + 1.0) * screen_width / 2.0;
        let screen_y = (1.0 - (y * scale + 1.0)) * screen_height / 2.0;
        let depth = z;

        (screen_x, screen_y, depth)
    }

    /// Get the effective spatial extent based on spectrum
    pub fn effective_extent(spectrum_position: Float, base_extent: Float) -> Float {
        if spectrum_position < 1.0 {
            // Below veil: normal space
            base_extent
        } else if spectrum_position > 1.0 {
            // Above veil: time dimensions expand
            base_extent * 3.0
        } else {
            // At veil: mixed
            base_extent * 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_config_space_time() {
        let config = SpatialConfig::from_spatial_dimensions(3, 1);
        assert_eq!(config.spatial_dimensions, 3);
        assert_eq!(config.time_dimensionality, 1);
        assert!(config.is_physical);
        assert!(!config.is_spiritual);
    }

    #[test]
    fn test_spatial_config_from_spectrum() {
        let config_below = SpatialConfig::from_spectrum_position(0.5);
        assert!(config_below.is_physical);

        let config_above = SpatialConfig::from_spectrum_position(1.5);
        assert!(config_above.is_spiritual);
    }

    #[test]
    fn test_veil_boundaries() {
        let veil = VeilBoundary::new();
        assert!(!veil.is_in_veil(0.5));
        assert!(veil.is_in_veil(1.0));
        assert!(!veil.is_in_veil(1.5));

        assert!((veil.veil_factor(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_coordinate_transformation() {
        let veil = VeilBoundary::new();
        let pos = Position3D::new(10.0, 20.0, 30.0);

        // Below veil: standard coordinates
        let transformed_below = TransformedCoordinates::new(pos, 0.5, &veil);
        assert_eq!(transformed_below.coordinate_type, CoordinateType::Standard);

        // Above veil: time-space coordinates
        let transformed_above = TransformedCoordinates::new(pos, 1.5, &veil);
        assert_eq!(transformed_above.coordinate_type, CoordinateType::TimeSpace);
    }

    #[test]
    fn test_spectrum_spatial_dynamics() {
        let dynamics = SpectrumSpatialDynamics::with_defaults();

        let config = dynamics.derive_spatial_structure(0.5);
        assert!(config.is_physical);

        let config2 = dynamics.derive_spatial_structure(1.5);
        assert!(config2.is_spiritual);
    }
}
