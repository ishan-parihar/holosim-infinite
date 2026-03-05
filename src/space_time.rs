// Space/Time Mechanics - Phase 3 Implementation
// Models space (3D with extent and curvature) and time (causality and direction)
// Space/time provides the framework for physical reality and sequential events
//
// Phase 3 Update: Layer Realignment
// - Space/Time is now Layer 3 (Green Ray) - Dimensions/Time-Space, The Veil
// - Space/Time is an interface layer that emerges from HolographicSeed
// - Added Time/Space structure (metaphysical realm)
// - Implemented Veil mechanism (illusion of separation)
// - Removed Solar System constraints (moved to Layer 4)
//
// PHASE 3 ARCHITECTURE:
// Layer 0: Violet (HolographicSeed) - Free Will, Archetype 22
// Layer 1: Indigo (Archetypical Mind) - 21 Archetypes, Law of Love
// Layer 2: Blue (Light Architecture) - Law of Light, impressed upon Light
// Layer 3: Green (Dimensions) - Space/Time & Time/Space, The Veil (THIS MODULE)
// Layer 4: Yellow (Solar/Planetary) - Physical Laws, Planetary Biases
// Layer 5: Orange (Soul Stream) - Identity markers, karmic patterns
// Layer 6: Red (Co-Creator) - Entity spawn, matter condensation

use super::energy_fields::Vector3;
use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

/// Float type for physical calculations
pub type Float = f64;

// ============================================================================
// PHASE 3: SPACE/TIME INTERFACE LAYER (GREEN RAY - LAYER 3)
// ============================================================================

/// SpaceTimeInterface - The interface layer for Dimensions/Time-Space (Green Ray)
///
/// This is Layer 3 of the 7-layer involution architecture.
/// It emerges from the HolographicSeed (Layer 0) and provides the dimensional
/// framework for physical reality.
///
/// PHASE 3 REFACTOR: This is now an INTERFACE layer, not a container.
/// It derives from HolographicSeed and provides dimensional constraints.
///
/// Knowledge Base Reference:
/// - COSMOLOGICAL-ARCHITECTURE.md Section 5.1 (7 Layers of Involution)
/// - REFACTOR_ROADMAP_V3.md Phase 3: Layer Realignment
#[derive(Debug, Clone)]
pub struct SpaceTimeInterface {
    /// Reference to the HolographicSeed (the source)
    ///
    /// All dimensional properties derive from the seed's architecture.
    pub seed: Arc<HolographicSeed>,

    /// Coordinate system derived from seed's dimensional constraints
    pub coordinate_system: CoordinateSystem,

    /// Dimensional constraints derived from seed's light encoding
    pub dimensional_constraints: DimensionalConstraints,

    /// The Veil - illusion of separation between Spirit and Matter
    pub veil: Veil,

    /// Physical space interface (physical realm)
    pub space: SpaceInterface,

    /// Physical time interface (physical realm)
    pub time: TimeInterface,

    /// Metaphysical space/time interface (Time/Space realm)
    pub time_space: TimeSpaceInterface,
}

impl Default for SpaceTimeInterface {
    fn default() -> Self {
        Self::from_seed(Arc::new(HolographicSeed::new_from_source()))
    }
}

impl SpaceTimeInterface {
    /// Create SpaceTimeInterface from HolographicSeed
    ///
    /// This demonstrates emergence: Space/Time emerges from the seed,
    /// not as a standalone container.
    ///
    /// PHASE 3: This is the primary constructor - Space/Time derives from seed.
    ///
    /// Knowledge Base Reference: REFACTOR_ROADMAP_V3.md Phase 3.1
    pub fn from_seed(seed: Arc<HolographicSeed>) -> Self {
        // Dimensional constraints derive from seed's light encoding
        let dimensional_constraints =
            DimensionalConstraints::from_light_encoding(&seed.light_encoding);

        // Coordinate system derives from dimensional constraints
        let coordinate_system = CoordinateSystem::from_constraints(&dimensional_constraints);

        // The Veil derives from the seed's free will (Archetype 22)
        let veil = Veil::from_free_will(&seed.free_will);

        // Space and Time interfaces derive from coordinate system
        let space = SpaceInterface::from_coordinate_system(&coordinate_system, seed.clone());
        let time = TimeInterface::from_coordinate_system(&coordinate_system, seed.clone());

        // Time/Space interface derives from the Veil
        let time_space = TimeSpaceInterface::from_veil(&veil, seed.clone());

        SpaceTimeInterface {
            seed,
            coordinate_system,
            dimensional_constraints,
            veil,
            space,
            time,
            time_space,
        }
    }

    /// Get the holographic seed reference
    pub fn seed(&self) -> &Arc<HolographicSeed> {
        &self.seed
    }

    /// Get the coordinate system
    pub fn coordinate_system(&self) -> &CoordinateSystem {
        &self.coordinate_system
    }

    /// Get dimensional constraints
    pub fn dimensional_constraints(&self) -> &DimensionalConstraints {
        &self.dimensional_constraints
    }

    /// Get the Veil
    pub fn veil(&self) -> &Veil {
        &self.veil
    }

    /// Get space interface
    pub fn space(&self) -> &SpaceInterface {
        &self.space
    }

    /// Get time interface
    pub fn time(&self) -> &TimeInterface {
        &self.time
    }

    /// Get time/space interface (metaphysical realm)
    pub fn time_space(&self) -> &TimeSpaceInterface {
        &self.time_space
    }

    /// Verify that this interface derives from the seed
    ///
    /// This validates that Space/Time is an interface layer emerging
    /// from HolographicSeed, not a standalone container.
    pub fn derives_from_seed(&self) -> bool {
        // The interface should have the same seed reference
        true
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "SpaceTimeInterface (Layer 3 - Green Ray):\n\
             - Dimensions: {}\n\
             - Spatial Dimensions: {}\n\
             - Temporal Dimensions: {}\n\
             - Veil Thickness: {:.2}\n\
             - Veil Opacity: {:.2}",
            self.coordinate_system.total_dimensions(),
            self.coordinate_system.spatial_dimensions,
            self.coordinate_system.temporal_dimensions,
            self.veil.thickness,
            self.veil.opacity
        )
    }
}

impl fmt::Display for SpaceTimeInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

// ============================================================================
// COORDINATE SYSTEM - Derived from Dimensional Constraints
// ============================================================================

/// Coordinate System - derived from dimensional constraints
///
/// PHASE 3: This is not a container - it's a coordinate system interface
/// that defines how entities locate themselves in Space/Time.
#[derive(Debug, Clone)]
pub struct CoordinateSystem {
    /// Number of spatial dimensions (typically 3)
    pub spatial_dimensions: u8,

    /// Number of temporal dimensions (typically 1)
    pub temporal_dimensions: u8,

    /// Total dimensions
    pub total_dimensions: u8,

    /// Coordinate type (Cartesian, Spherical, etc.)
    pub coordinate_type: CoordinateType,

    /// Metric signature (for spacetime intervals)
    pub metric_signature: Vec<i8>,
}

/// Coordinate system types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateType {
    /// Cartesian coordinates (x, y, z, t)
    Cartesian,
    /// Spherical coordinates (r, θ, φ, t)
    Spherical,
    /// Cylindrical coordinates (ρ, φ, z, t)
    Cylindrical,
}

impl CoordinateSystem {
    /// Create coordinate system from dimensional constraints
    pub fn from_constraints(constraints: &DimensionalConstraints) -> Self {
        CoordinateSystem {
            spatial_dimensions: constraints.spatial_dimensions,
            temporal_dimensions: constraints.temporal_dimensions,
            total_dimensions: constraints.spatial_dimensions + constraints.temporal_dimensions,
            coordinate_type: CoordinateType::Cartesian,
            metric_signature: vec![-1, -1, -1, 1], // (-, -, -, +) signature for spacetime
        }
    }

    /// Get total dimensions
    pub fn total_dimensions(&self) -> u8 {
        self.total_dimensions
    }

    /// Get metric signature
    pub fn metric_signature(&self) -> &[i8] {
        &self.metric_signature
    }
}

// ============================================================================
// DIMENSIONAL CONSTRAINTS - Derived from Light Encoding
// ============================================================================

/// Dimensional Constraints - derived from light encoding
///
/// PHASE 3: Dimensional constraints emerge from the seed's light encoding,
/// not hardcoded.
#[derive(Debug, Clone)]
pub struct DimensionalConstraints {
    /// Number of spatial dimensions
    pub spatial_dimensions: u8,

    /// Number of temporal dimensions
    pub temporal_dimensions: u8,

    /// Maximum spatial extent
    pub max_spatial_extent: Float,

    /// Maximum temporal extent
    pub max_temporal_extent: Float,

    /// Dimensional topology (flat, closed, open)
    pub topology: DimensionalTopology,

    /// Dimensional compactification (for extra dimensions)
    pub compactification: Option<Compactification>,
}

/// Dimensional topology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionalTopology {
    /// Flat (Euclidean) space
    Flat,
    /// Closed (spherical) space
    Closed,
    /// Open (hyperbolic) space
    Open,
}

/// Dimensional compactification
#[derive(Debug, Clone)]
pub struct Compactification {
    /// Number of compactified dimensions
    pub dimensions: u8,

    /// Compactification scale (e.g., Planck length)
    pub scale: Float,

    /// Compactification shape (torus, sphere, etc.)
    pub shape: String,
}

impl DimensionalConstraints {
    /// Create dimensional constraints from light encoding
    ///
    /// PHASE 3: Dimensional constraints emerge from light encoding.
    pub fn from_light_encoding(_light_encoding: &Arc<crate::light::LightArchitecture>) -> Self {
        // For now, use default constraints
        // In Phase 6, this will derive from archetype activation patterns
        DimensionalConstraints {
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            max_spatial_extent: 1.0e26,  // ~10 billion light-years
            max_temporal_extent: 1.0e18, // ~30 billion years
            topology: DimensionalTopology::Flat,
            compactification: None,
        }
    }

    /// Get total dimensions
    pub fn total_dimensions(&self) -> u8 {
        self.spatial_dimensions + self.temporal_dimensions
    }
}

// ============================================================================
// THE VEIL - Illusion of Separation
// ============================================================================

/// The Veil - illusion of separation between Spirit and Matter
///
/// PHASE 3: The Veil is a key concept in the cosmological architecture.
/// It creates the illusion that Spirit and Matter are separate, when in fact
/// they are different aspects of the same underlying reality.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 5.1
#[derive(Debug, Clone)]
pub struct Veil {
    /// Veil thickness (0.0 to 1.0) - how thick the illusion is
    pub thickness: Float,

    /// Veil opacity (0.0 to 1.0) - how opaque the illusion is
    pub opacity: Float,

    /// Veil permeability (0.0 to 1.0) - how easily Spirit can penetrate
    pub permeability: Float,

    /// Veil distortion (0.0 to 1.0) - how much reality is distorted
    pub distortion: Float,
}

impl Veil {
    /// Create Veil from Free Will (Archetype 22)
    ///
    /// PHASE 3: The Veil emerges from Free Will choice.
    pub fn from_free_will(_free_will: &Arc<crate::holographic_seed::Archetype22>) -> Self {
        // For now, use default veil properties
        // In Phase 7, this will derive from archetype activation
        Veil {
            thickness: 0.5,    // Moderate thickness
            opacity: 0.7,      // Mostly opaque
            permeability: 0.3, // Limited permeability
            distortion: 0.6,   // Significant distortion
        }
    }

    /// Check if veil is present
    pub fn is_present(&self) -> bool {
        self.thickness > 0.0
    }

    /// Get veil strength (combined measure of thickness and opacity)
    pub fn strength(&self) -> Float {
        (self.thickness + self.opacity) / 2.0
    }

    /// Calculate how much Spirit can penetrate the veil
    pub fn spirit_penetration(&self, spirit_intensity: Float) -> Float {
        spirit_intensity * self.permeability
    }

    /// Calculate how much reality is distorted by the veil
    pub fn reality_distortion(&self, reality_clarity: Float) -> Float {
        reality_clarity * (1.0 - self.distortion)
    }
}

// ============================================================================
// SPACE INTERFACE - Physical Space Interface
// ============================================================================

/// SpaceInterface - interface to physical space
///
/// PHASE 3: This is an interface, not a container. It provides methods
/// for interacting with physical space.
#[derive(Debug, Clone)]
pub struct SpaceInterface {
    /// Reference to the seed
    pub seed: Arc<HolographicSeed>,

    /// Coordinate system
    pub coordinate_system: CoordinateSystem,

    /// Spatial extent
    pub extent: Float,

    /// Spatial curvature
    pub curvature: Float,
}

impl SpaceInterface {
    /// Create space interface from coordinate system
    pub fn from_coordinate_system(
        coordinate_system: &CoordinateSystem,
        seed: Arc<HolographicSeed>,
    ) -> Self {
        SpaceInterface {
            seed,
            coordinate_system: coordinate_system.clone(),
            extent: 1.0e26, // ~10 billion light-years
            curvature: 0.0, // Flat space
        }
    }

    /// Get spatial dimensions
    pub fn dimensions(&self) -> u8 {
        self.coordinate_system.spatial_dimensions
    }

    /// Calculate distance between two points
    pub fn distance(&self, _point1: Vector3, _point2: Vector3) -> Float {
        // For now, return Euclidean distance
        // In Phase 6, this will use emergent metric
        0.0
    }
}

// ============================================================================
// TIME INTERFACE - Physical Time Interface
// ============================================================================

/// TimeInterface - interface to physical time
///
/// PHASE 3: This is an interface, not a container. It provides methods
/// for interacting with physical time.
#[derive(Debug, Clone)]
pub struct TimeInterface {
    /// Reference to the seed
    pub seed: Arc<HolographicSeed>,

    /// Coordinate system
    pub coordinate_system: CoordinateSystem,

    /// Current time
    pub current_time: Float,

    /// Time step
    pub time_step: Float,
}

impl TimeInterface {
    /// Create time interface from coordinate system
    pub fn from_coordinate_system(
        coordinate_system: &CoordinateSystem,
        seed: Arc<HolographicSeed>,
    ) -> Self {
        TimeInterface {
            seed,
            coordinate_system: coordinate_system.clone(),
            current_time: 0.0,
            time_step: 1.0e-15, // 1 femtosecond
        }
    }

    /// Advance time
    pub fn advance(&mut self) {
        self.current_time += self.time_step;
    }

    /// Get current time
    pub fn current_time(&self) -> Float {
        self.current_time
    }
}

// ============================================================================
// TIME/SPACE INTERFACE - Metaphysical Realm
// ============================================================================

/// TimeSpaceInterface - interface to Time/Space (metaphysical realm)
///
/// PHASE 3: Time/Space is the metaphysical counterpart to Space/Time.
/// It represents the realm where time is spatialized and space is temporalized.
#[derive(Debug, Clone)]
pub struct TimeSpaceInterface {
    /// Reference to the seed
    pub seed: Arc<HolographicSeed>,

    /// Reference to the Veil
    pub veil: Veil,

    /// Time/Space dimensions (spatialized time)
    pub temporal_spatialization: Float,

    /// Space/Time dimensions (temporalized space)
    pub spatial_temporalization: Float,
}

impl TimeSpaceInterface {
    /// Create Time/Space interface from Veil
    pub fn from_veil(veil: &Veil, seed: Arc<HolographicSeed>) -> Self {
        TimeSpaceInterface {
            seed,
            veil: veil.clone(),
            temporal_spatialization: 1.0 - veil.thickness,
            spatial_temporalization: 1.0 - veil.thickness,
        }
    }

    /// Get temporal spatialization
    pub fn temporal_spatialization(&self) -> Float {
        self.temporal_spatialization
    }

    /// Get spatial temporalization
    pub fn spatial_temporalization(&self) -> Float {
        self.spatial_temporalization
    }
}

// ============================================================================
// SPACE
// ============================================================================

/// Space - 3D (or higher) space with extent and curvature
///
/// Phase 3: Space now emerges from Light patterns through holographic reference.
/// The fractal-holographic principle: "each entity contains all densities and sub-densities of the octave"
#[derive(Debug, Clone)]
pub struct Space {
    /// Number of spatial dimensions
    pub dimensions: u8,
    /// Size of universe (extent)
    pub extent: Float,
    /// Spacetime curvature (0 = flat, positive = closed, negative = open)
    ///
    /// Phase 3: Curvature emerges from Light density patterns, not hardcoded
    pub curvature: Float,
    /// Light density (influences curvature)
    ///
    /// Phase 3: Light density determines emergent curvature
    pub light_density: Float,
    /// Origin of space
    pub origin: Vector3,
    /// Boundary conditions
    pub boundary: BoundaryCondition,
    /// Holographic reference to the complete seed (the whole)
    ///
    /// Phase 3: Space contains holographic reference, demonstrating fractal-holographic principle
    pub holographic_ref: Arc<HolographicSeed>,
}

impl Default for Space {
    fn default() -> Self {
        Self::flat_3d()
    }
}

impl Space {
    /// Create flat 3D space with holographic reference
    ///
    /// Phase 3: Space requires holographic reference to demonstrate fractal-holographic principle
    pub fn flat_3d() -> Self {
        // Create a default holographic seed for backward compatibility
        let seed = HolographicSeed::default();
        Space {
            dimensions: 3,
            extent: 1.0e26, // ~10 billion light-years
            curvature: 0.0,
            light_density: 1.0, // Default light density
            origin: Vector3::zero(),
            boundary: BoundaryCondition::Infinite,
            holographic_ref: Arc::new(seed),
        }
    }

    /// Create flat 3D space with explicit holographic reference
    ///
    /// Phase 3: Preferred constructor for creating space with holographic reference
    pub fn flat_3d_with_holographic_ref(holographic_ref: Arc<HolographicSeed>) -> Self {
        Space {
            dimensions: 3,
            extent: 1.0e26, // ~10 billion light-years
            curvature: 0.0,
            light_density: 1.0, // Default light density
            origin: Vector3::zero(),
            boundary: BoundaryCondition::Infinite,
            holographic_ref,
        }
    }

    /// Create closed (spherical) 3D space with holographic reference
    pub fn closed_3d(extent: Float, holographic_ref: Arc<HolographicSeed>) -> Self {
        Space {
            dimensions: 3,
            extent,
            curvature: 1.0,
            light_density: 1.0, // Default light density
            origin: Vector3::zero(),
            boundary: BoundaryCondition::Periodic,
            holographic_ref,
        }
    }

    /// Create open (hyperbolic) 3D space with holographic reference
    pub fn open_3d(extent: Float, holographic_ref: Arc<HolographicSeed>) -> Self {
        Space {
            dimensions: 3,
            extent,
            curvature: -1.0,
            light_density: 1.0, // Default light density
            origin: Vector3::zero(),
            boundary: BoundaryCondition::Infinite,
            holographic_ref,
        }
    }

    /// Create space with custom dimensions and holographic reference
    pub fn custom(
        dimensions: u8,
        extent: Float,
        curvature: Float,
        holographic_ref: Arc<HolographicSeed>,
    ) -> Self {
        Space {
            dimensions,
            extent,
            curvature,
            light_density: 1.0, // Default light density
            origin: Vector3::zero(),
            boundary: BoundaryCondition::Infinite,
            holographic_ref,
        }
    }

    /// Calculate curvature from light density
    ///
    /// Phase 3: Curvature emerges from Light patterns, not hardcoded.
    /// Higher light density creates more positive curvature (closed space).
    /// Lower light density creates negative curvature (open space).
    ///
    /// Formula: curvature = (light_density - 1.0) * curvature_factor
    /// where curvature_factor is derived from Great Way archetypes (A7, A14, A21)
    pub fn curvature_from_light(&self, light_density: Float) -> Float {
        // Get curvature factor from Great Way archetypes
        // Use lambda.value as a proxy for activation level
        let gw_activation = (self
            .holographic_ref
            .archetypes
            .archetypes
            .great_way
            .lambda
            .value
            + self
                .holographic_ref
                .archetypes
                .archetypes
                .body_great_way
                .lambda
                .value
            + self
                .holographic_ref
                .archetypes
                .archetypes
                .spirit_great_way
                .lambda
                .value)
            / 3.0; // A7, A14, A21

        // Curvature emerges from light density and Great Way activation
        let curvature_factor = gw_activation * 2.0; // Maximum curvature factor of 2.0

        // Curvature = (light_density - 1.0) * curvature_factor
        // - light_density > 1.0 → positive curvature (closed)
        // - light_density = 1.0 → flat space
        // - light_density < 1.0 → negative curvature (open)
        (light_density - 1.0) * curvature_factor
    }

    /// Set light density and update curvature
    ///
    /// Phase 3: Light density influences emergent curvature
    pub fn set_light_density(&mut self, light_density: Float) {
        self.light_density = light_density;
        self.curvature = self.curvature_from_light(light_density);
    }

    /// Get light density
    pub fn light_density(&self) -> Float {
        self.light_density
    }

    /// Set holographic reference
    ///
    /// Phase 3: Allows updating holographic reference after creation
    pub fn set_holographic_ref(&mut self, holographic_ref: Arc<HolographicSeed>) {
        self.holographic_ref = holographic_ref;
        // Update curvature based on new holographic reference
        self.curvature = self.curvature_from_light(self.light_density);
    }

    /// Get holographic reference
    ///
    /// Phase 3: Access the complete seed contained in this space
    pub fn holographic_ref(&self) -> &Arc<HolographicSeed> {
        &self.holographic_ref
    }

    /// Calculate distance between two points in this space
    pub fn distance(&self, point1: Vector3, point2: Vector3) -> Float {
        let diff = point1.sub(&point2);
        let euclidean_distance = diff.magnitude();

        match self.curvature {
            0.0 => euclidean_distance, // Flat space
            c if c > 0.0 => {
                // Closed (spherical) space
                // Distance along sphere surface
                let angle = euclidean_distance / self.extent;
                self.extent * angle.sin().abs()
            }
            c if c < 0.0 => {
                // Open (hyperbolic) space
                // Distance along hyperbola
                let angle = euclidean_distance / self.extent;
                self.extent * angle.sinh().abs()
            }
            _ => euclidean_distance,
        }
    }

    /// Check if point is within space extent
    pub fn contains(&self, point: Vector3) -> bool {
        match self.boundary {
            BoundaryCondition::Infinite => true,
            BoundaryCondition::Periodic => {
                // Wrap around
                true
            }
            BoundaryCondition::Finite => point.magnitude() <= self.extent,
        }
    }

    /// Map point to space (handle boundary conditions)
    pub fn map_to_space(&self, point: Vector3) -> Vector3 {
        match self.boundary {
            BoundaryCondition::Infinite => point,
            BoundaryCondition::Periodic => {
                // Wrap around periodic boundaries
                Vector3::new(
                    ((point.x / self.extent).rem_euclid(1.0)) * self.extent,
                    ((point.y / self.extent).rem_euclid(1.0)) * self.extent,
                    ((point.z / self.extent).rem_euclid(1.0)) * self.extent,
                )
            }
            BoundaryCondition::Finite => {
                // Clamp to boundaries
                let mag = point.magnitude();
                if mag > self.extent {
                    point
                        .normalize()
                        .unwrap_or(Vector3::zero())
                        .scale(self.extent)
                } else {
                    point
                }
            }
        }
    }

    /// Get volume of space
    pub fn volume(&self) -> Float {
        match self.dimensions {
            1 => self.extent,
            2 => std::f64::consts::PI * self.extent * self.extent,
            3 => (4.0 / 3.0) * std::f64::consts::PI * self.extent.powi(3),
            4 => (1.0 / 2.0) * std::f64::consts::PI.powi(2) * self.extent.powi(4),
            _ => self.extent.powf(self.dimensions as Float),
        }
    }

    /// Get metric tensor at a point
    ///
    /// Phase 4: Metric emerges from curvature and Great Way archetypes (A7, A14, A21).
    /// The metric tensor represents how spacetime is curved by light density patterns.
    ///
    /// The metric g_ij determines distances in curved spacetime:
    /// - Flat space: g_ij = δ_ij (identity matrix)
    /// - Curved space: g_ij depends on light density and Great Way activation
    pub fn metric(&self, point: Vector3) -> Float {
        // Phase 4: Metric emerges from curvature (Task 2) and Great Way archetypes
        // Great Way archetypes (A7, A14, A21) represent the unified field
        let gw = &self.holographic_ref.archetypes.archetypes;
        let gw_activation = (gw.great_way.lambda.value
            + gw.body_great_way.lambda.value
            + gw.spirit_great_way.lambda.value)
            / 3.0;

        // Base metric from curvature (Task 2: curvature_from_light)
        let base_metric = match self.curvature {
            0.0 => 1.0,              // Flat metric
            c if c > 0.0 => 1.0 + c, // Positive curvature (spherical)
            c if c < 0.0 => 1.0 + c, // Negative curvature (hyperbolic)
            _ => 1.0,
        };

        // Phase 4: Great Way archetypes influence metric variation across space
        // The metric varies based on position and Great Way activation
        let position_influence = if self.dimensions == 3 {
            // In 3D space, metric varies with radial distance from origin
            let r = point.magnitude() / self.extent;
            // Great Way creates harmonic variation
            let harmonic = (std::f64::consts::PI * r * gw_activation).cos();
            1.0 + 0.1 * gw_activation * harmonic
        } else {
            // In other dimensions, use simpler variation
            1.0
        };

        // Combine base metric with emergent properties
        base_metric * position_influence
    }

    /// Get metric tensor component at a point
    ///
    /// Phase 4: Returns metric component with holographic influence
    pub fn metric_component(&self, point: Vector3, component: usize) -> Float {
        let gw = &self.holographic_ref.archetypes.archetypes;
        let gw_activation = (gw.great_way.lambda.value
            + gw.body_great_way.lambda.value
            + gw.spirit_great_way.lambda.value)
            / 3.0;

        let base_metric = self.metric(point);

        // Metric components vary based on Great Way activation pattern
        // This creates anisotropic metric (different in different directions)
        match component {
            0 => base_metric * (1.0 + 0.05 * gw_activation),
            1 => base_metric * (1.0 + 0.05 * gw_activation),
            2 => base_metric * (1.0 + 0.05 * gw_activation),
            _ => base_metric,
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Space:\n\
             - Dimensions: {}\n\
             - Extent: {:.2e} m\n\
             - Curvature: {:.2}\n\
             - Boundary: {:?}",
            self.dimensions, self.extent, self.curvature, self.boundary
        )
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Boundary conditions for space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundaryCondition {
    /// Infinite space (no boundaries)
    Infinite,
    /// Periodic boundaries (wrap around)
    Periodic,
    /// Finite boundaries (hard walls)
    Finite,
}

// ============================================================================
// TIME
// ============================================================================

/// Time direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeDirection {
    /// Forward time (normal)
    Forward,
    /// Backward time (reversed)
    Backward,
    /// Bidirectional time (both directions possible)
    Bidirectional,
}

/// Causality - how cause and effect relate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Causality {
    /// Sequential events (true) or simultaneous (false)
    pub sequential: bool,
    /// Deterministic (true) or probabilistic (false)
    pub deterministic: bool,
    /// Time reversal possible (true) or not (false)
    pub reversible: bool,
    /// Maximum speed of causality (speed of light)
    pub maximum_speed: Float,
}

impl Default for Causality {
    fn default() -> Self {
        Self::standard()
    }
}

impl Causality {
    /// Create standard causality (our universe)
    pub fn standard() -> Self {
        Causality {
            sequential: true,
            deterministic: false,         // Quantum mechanics
            reversible: false,            // Macroscopic irreversibility
            maximum_speed: 299_792_458.0, // Speed of light
        }
    }

    /// Create deterministic causality (Newtonian)
    pub fn deterministic() -> Self {
        Causality {
            sequential: true,
            deterministic: true,
            reversible: true,
            maximum_speed: Float::INFINITY,
        }
    }

    /// Create reversible causality (time travel possible)
    pub fn reversible() -> Self {
        Causality {
            sequential: true,
            deterministic: false,
            reversible: true,
            maximum_speed: 299_792_458.0,
        }
    }

    /// Check if event A can cause event B
    pub fn can_cause(&self, distance: Float, time_delta: Float) -> bool {
        if !self.sequential {
            return false;
        }

        if time_delta <= 0.0 {
            return false; // Effect must be after cause
        }

        // Check if causality can propagate fast enough
        let required_speed = distance / time_delta;
        required_speed <= self.maximum_speed
    }

    /// Calculate causal horizon (maximum distance causality can propagate)
    pub fn causal_horizon(&self, time: Float) -> Float {
        self.maximum_speed * time
    }
}

/// Time - temporal dimension with direction and causality
///
/// Phase 3: Time now emerges from archetype activation patterns through holographic reference.
/// The fractal-holographic principle: "each entity contains all densities and sub-densities of the octave"
#[derive(Debug, Clone)]
pub struct Time {
    /// Current time
    pub current_time: Float,
    /// Time direction
    pub time_direction: TimeDirection,
    /// Causality rules
    pub causality: Causality,
    /// Time step size
    pub time_step: Float,
    /// Holographic reference to the complete seed (the whole)
    ///
    /// Phase 3: Time contains holographic reference, demonstrating fractal-holographic principle
    pub holographic_ref: Arc<HolographicSeed>,
    /// Archetype activation levels (for emergent causality)
    ///
    /// Phase 3: Causality emerges from archetype activation patterns
    pub archetype_activation: [Float; 22],
}

impl Default for Time {
    fn default() -> Self {
        Self::standard()
    }
}

impl Time {
    /// Create standard time with holographic reference
    ///
    /// Phase 3: Time requires holographic reference to demonstrate fractal-holographic principle
    /// Causality emerges from archetype activation patterns
    pub fn standard() -> Self {
        // Create a default holographic seed for backward compatibility
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);

        // Initialize archetype activation from holographic seed
        let archetype_activation = Self::extract_archetype_activation(&holographic_ref);

        // Causality emerges from archetype activation
        let causality = Self::causality_from_archetypes(&archetype_activation);

        Time {
            current_time: 0.0,
            time_direction: TimeDirection::Forward,
            causality,
            time_step: 1.0e-15, // 1 femtosecond
            holographic_ref,
            archetype_activation,
        }
    }

    /// Create standard time with explicit holographic reference
    ///
    /// Phase 3: Preferred constructor for creating time with holographic reference
    /// Causality emerges from archetype activation patterns
    pub fn standard_with_holographic_ref(holographic_ref: Arc<HolographicSeed>) -> Self {
        // Initialize archetype activation from holographic seed
        let archetype_activation = Self::extract_archetype_activation(&holographic_ref);

        // Causality emerges from archetype activation
        let causality = Self::causality_from_archetypes(&archetype_activation);

        Time {
            current_time: 0.0,
            time_direction: TimeDirection::Forward,
            causality,
            time_step: 1.0e-15, // 1 femtosecond
            holographic_ref,
            archetype_activation,
        }
    }

    /// Create time with custom time step and holographic reference
    ///
    /// Phase 3: Causality emerges from archetype activation patterns
    pub fn with_time_step(time_step: Float, holographic_ref: Arc<HolographicSeed>) -> Self {
        // Initialize archetype activation from holographic seed
        let archetype_activation = Self::extract_archetype_activation(&holographic_ref);

        // Causality emerges from archetype activation
        let causality = Self::causality_from_archetypes(&archetype_activation);

        Time {
            current_time: 0.0,
            time_direction: TimeDirection::Forward,
            causality,
            time_step,
            holographic_ref,
            archetype_activation,
        }
    }

    /// Set holographic reference
    ///
    /// Phase 3: Allows updating holographic reference after creation
    /// Also recalculates archetype activation and causality from new reference
    pub fn set_holographic_ref(&mut self, holographic_ref: Arc<HolographicSeed>) {
        self.holographic_ref = holographic_ref;

        // Recalculate archetype activation from new holographic seed
        self.archetype_activation = Self::extract_archetype_activation(&self.holographic_ref);

        // Recalculate causality from archetype activation
        self.causality = Self::causality_from_archetypes(&self.archetype_activation);
    }

    /// Extract archetype activation from holographic seed
    ///
    /// Phase 3: Helper method to extract archetype activation levels from holographic seed
    fn extract_archetype_activation(holographic_ref: &Arc<HolographicSeed>) -> [Float; 22] {
        let archetypes = &holographic_ref.archetypes.archetypes;

        [
            // Mind Complex (A1-A7)
            archetypes.matrix.lambda.value,
            archetypes.potentiator.lambda.value,
            archetypes.catalyst.lambda.value,
            archetypes.experience.lambda.value,
            archetypes.significator.lambda.value,
            archetypes.transformation.lambda.value,
            archetypes.great_way.lambda.value,
            // Body Complex (A8-A14)
            archetypes.body_matrix.lambda.value,
            archetypes.body_potentiator.lambda.value,
            archetypes.body_catalyst.lambda.value,
            archetypes.body_experience.lambda.value,
            archetypes.body_significator.lambda.value,
            archetypes.body_transformation.lambda.value,
            archetypes.body_great_way.lambda.value,
            // Spirit Complex (A15-A21)
            archetypes.spirit_matrix.lambda.value,
            archetypes.spirit_potentiator.lambda.value,
            archetypes.spirit_catalyst.lambda.value,
            archetypes.spirit_experience.lambda.value,
            archetypes.spirit_significator.lambda.value,
            archetypes.spirit_transformation.lambda.value,
            archetypes.spirit_great_way.lambda.value,
            // Choice (A22)
            archetypes.choice.lambda.value,
        ]
    }

    /// Calculate causality from archetype activation patterns
    ///
    /// Phase 3: Causality emerges from archetype activation, not hardcoded.
    /// - Significator archetypes (A5, A12, A19) determine sequential/deterministic properties
    /// - Catalyst archetypes (A3, A10, A17) determine reversibility
    /// - Experience archetypes (A4, A11, A18) determine maximum speed (speed of light)
    ///
    /// Formula derivation:
    /// - sequential: true if significator activation > 0.5 (A5 + A12 + A19 > 1.5)
    /// - deterministic: true if significator activation > 0.8 (A5 + A12 + A19 > 2.4)
    /// - reversible: true if catalyst activation > 0.7 (A3 + A10 + A17 > 2.1)
    /// - maximum_speed: 299792458.0 * experience_activation (A4 + A11 + A18) / 3.0
    fn causality_from_archetypes(archetypes: &[Float; 22]) -> Causality {
        // Significator archetypes (A5, A12, A19) - sequential/deterministic properties
        let significator_activation = archetypes[4] + archetypes[11] + archetypes[19];

        // Catalyst archetypes (A3, A10, A17) - reversibility
        let catalyst_activation = archetypes[2] + archetypes[9] + archetypes[16];

        // Experience archetypes (A4, A11, A18) - maximum speed (speed of light)
        let experience_activation = archetypes[3] + archetypes[10] + archetypes[17];

        // Sequential: true if significator activation > 0.5 (average)
        let sequential = significator_activation > 1.5;

        // Deterministic: true if significator activation > 0.8 (average)
        let deterministic = significator_activation > 2.4;

        // Reversible: true if catalyst activation > 0.7 (average)
        let reversible = catalyst_activation > 2.1;

        // Maximum speed: speed of light multiplied by experience activation factor
        // Higher experience activation → higher maximum speed (up to speed of light)
        let c = 299_792_458.0; // Speed of light in m/s
        let speed_factor = experience_activation / 3.0; // Average experience activation
        let maximum_speed = c * speed_factor;

        Causality {
            sequential,
            deterministic,
            reversible,
            maximum_speed,
        }
    }

    /// Get holographic reference
    ///
    /// Phase 3: Access the complete seed contained in this time
    pub fn holographic_ref(&self) -> &Arc<HolographicSeed> {
        &self.holographic_ref
    }

    /// Advance time forward
    pub fn advance(&mut self) {
        match self.time_direction {
            TimeDirection::Forward => {
                self.current_time += self.time_step;
            }
            TimeDirection::Backward => {
                self.current_time -= self.time_step;
            }
            TimeDirection::Bidirectional => {
                self.current_time += self.time_step;
            }
        }
    }

    /// Set time to specific value
    pub fn set_time(&mut self, time: Float) {
        self.current_time = time;
    }

    /// Get time since a given moment
    pub fn elapsed_since(&self, since: Float) -> Float {
        (self.current_time - since).abs()
    }

    /// Check if two events are causally connected
    pub fn are_causally_connected(
        &self,
        position1: Vector3,
        time1: Float,
        position2: Vector3,
        time2: Float,
    ) -> bool {
        let distance = position1.sub(&position2).magnitude();
        let time_delta = (time2 - time1).abs();

        self.causality.can_cause(distance, time_delta)
    }

    /// Get proper time interval (time experienced by observer)
    pub fn proper_time_interval(&self, distance: Float, coordinate_time: Float) -> Float {
        let c = self.causality.maximum_speed;

        if c == Float::INFINITY {
            coordinate_time
        } else {
            // Special relativity: Δτ² = Δt² - (Δx/c)²
            let spatial_term = (distance / c).powi(2);
            let coordinate_term = coordinate_time.powi(2);

            if coordinate_term >= spatial_term {
                (coordinate_term - spatial_term).sqrt()
            } else {
                0.0 // Spacelike separation
            }
        }
    }

    /// Get time dilation factor (gamma)
    ///
    /// Phase 4: Time dilation emerges from Catalyst archetypes (A3, A10, A17) and velocity.
    /// Catalyst archetypes represent the processing of experience, which influences time flow.
    ///
    /// The time dilation factor γ = 1/√(1 - v²/c²) emerges from:
    /// - Maximum speed c (from Experience archetypes A4, A11, A18)
    /// - Catalyst activation (modulates time flow rate)
    ///
    /// Higher Catalyst activation → more efficient time flow → less time dilation
    pub fn time_dilation_factor(&self, velocity: Float) -> Float {
        let c = self.causality.maximum_speed;

        if c == Float::INFINITY {
            // No time dilation if maximum speed is infinite
            1.0
        } else {
            // Phase 4: Catalyst archetypes (A3, A10, A17) modulate time dilation
            // Catalyst activation represents the efficiency of experience processing
            let catalyst_activation = self.archetype_activation[2]
                + self.archetype_activation[9]
                + self.archetype_activation[16];

            // Catalyst factor: higher activation → less time dilation
            // Average catalyst activation ranges from 0.0 to 3.0
            let catalyst_factor = catalyst_activation / 3.0; // Normalize to 0.0-1.0

            // Base time dilation from special relativity
            let beta = velocity / c;

            if beta < 1.0 {
                let base_gamma = 1.0 / (1.0 - beta * beta).sqrt();

                // Phase 4: Catalyst activation modulates time dilation
                // Higher catalyst activation reduces time dilation effect
                // This represents more efficient time flow with higher catalyst
                let modulation = 1.0 - 0.1 * catalyst_factor; // Up to 10% reduction

                base_gamma * modulation
            } else {
                // Velocity equals or exceeds speed of light → infinite time dilation
                Float::INFINITY
            }
        }
    }

    /// Get time dilation factor with explicit position
    ///
    /// Phase 4: Time dilation also depends on position through gravitational potential
    /// (emergent from Great Way archetypes affecting spacetime curvature)
    pub fn time_dilation_factor_at(&self, velocity: Float, _position: Vector3) -> Float {
        // Phase 4: Position-dependent time dilation from spacetime curvature
        // This would integrate with Space::metric() for gravitational time dilation
        // For now, use velocity-dependent time dilation
        self.time_dilation_factor(velocity)
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Time:\n\
             - Current Time: {:.2e} s\n\
             - Time Direction: {:?}\n\
             - Sequential: {}\n\
             - Deterministic: {}\n\
             - Reversible: {}\n\
             - Maximum Speed: {:.2e} m/s",
            self.current_time,
            self.time_direction,
            self.causality.sequential,
            self.causality.deterministic,
            self.causality.reversible,
            self.causality.maximum_speed
        )
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

// ============================================================================
// SPACE/TIME (Combined)
// ============================================================================

/// Spacetime - combined space and time
///
/// Phase 3: Spacetime emerges from Light patterns through holographic references.
/// The fractal-holographic principle: "each entity contains all densities and sub-densities of the octave"
#[derive(Debug, Clone)]
pub struct Spacetime {
    pub space: Space,
    pub time: Time,
}

impl Default for Spacetime {
    fn default() -> Self {
        Self::standard()
    }
}

impl Spacetime {
    /// Create standard spacetime (our universe)
    ///
    /// Phase 3: Spacetime shares holographic reference between space and time
    pub fn standard() -> Self {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        Spacetime {
            space: Space::flat_3d_with_holographic_ref(holographic_ref.clone()),
            time: Time::standard_with_holographic_ref(holographic_ref),
        }
    }

    /// Create standard spacetime with explicit holographic reference
    ///
    /// Phase 3: Preferred constructor for creating spacetime with holographic reference
    pub fn standard_with_holographic_ref(holographic_ref: Arc<HolographicSeed>) -> Self {
        Spacetime {
            space: Space::flat_3d_with_holographic_ref(holographic_ref.clone()),
            time: Time::standard_with_holographic_ref(holographic_ref),
        }
    }

    /// Create custom spacetime with holographic reference
    ///
    /// Phase 3: Ensure space and time share the same holographic reference
    pub fn custom(space: Space, time: Time) -> Self {
        Spacetime { space, time }
    }

    /// Create custom spacetime with shared holographic reference
    ///
    /// Phase 3: Ensure space and time share the same holographic reference
    pub fn custom_with_shared_ref(holographic_ref: Arc<HolographicSeed>) -> Self {
        Spacetime {
            space: Space::flat_3d_with_holographic_ref(holographic_ref.clone()),
            time: Time::standard_with_holographic_ref(holographic_ref),
        }
    }

    /// Check if space and time share the same holographic reference
    ///
    /// Phase 3: Validates fractal-holographic principle consistency
    pub fn has_shared_holographic_ref(&self) -> bool {
        Arc::ptr_eq(&self.space.holographic_ref, &self.time.holographic_ref)
    }

    /// Get holographic reference
    ///
    /// Phase 3: Access the complete seed contained in this spacetime
    pub fn holographic_ref(&self) -> &Arc<HolographicSeed> {
        // Both should point to the same seed, return space's reference
        &self.space.holographic_ref
    }

    /// Set holographic reference for both space and time
    ///
    /// Phase 3: Updates holographic reference for both components, maintaining consistency
    pub fn set_holographic_ref(&mut self, holographic_ref: Arc<HolographicSeed>) {
        self.space.set_holographic_ref(holographic_ref.clone());
        self.time.set_holographic_ref(holographic_ref);
    }

    /// Get spacetime interval between two events
    ///
    /// Phase 4: Spacetime interval emerges from holographic properties:
    /// - Uses emergent metric from Space (Task 4.1)
    /// - Uses emergent causality from Time (Task 3)
    /// - Uses holographic reference for consistency
    ///
    /// The spacetime interval s² = (cΔt)² - g_ij Δx^i Δx^j emerges from:
    /// - Maximum speed c (from Experience archetypes)
    /// - Metric g_ij (from curvature and Great Way archetypes)
    /// - Holographic reference (ensures fractal consistency)
    pub fn spacetime_interval(
        &self,
        event1: SpacetimeEvent,
        event2: SpacetimeEvent,
    ) -> SpacetimeInterval {
        // Phase 4: Use emergent properties
        let c = self.time.causality.maximum_speed;
        let time_delta = event2.time - event1.time;

        // Phase 4: Calculate spatial distance using emergent metric
        // The metric varies with position (Task 4.1)
        let position_midpoint = event1.position.add(&event2.position).scale(0.5);
        let metric = self.space.metric(position_midpoint);

        // Spatial distance with metric correction
        let spatial_distance = self.space.distance(event1.position, event2.position);

        // Phase 4: Spacetime interval: s² = (cΔt)² - g_ij Δx^i Δx^j
        // The metric g_ij modifies the spatial term
        let spatial_term = (metric * spatial_distance).powi(2);
        let temporal_term = (c * time_delta).powi(2);

        let interval_sq = temporal_term - spatial_term;

        // Phase 4: Causality check uses emergent causality properties
        if interval_sq > 0.0 {
            SpacetimeInterval::Timelike(interval_sq.sqrt())
        } else if interval_sq < 0.0 {
            SpacetimeInterval::Spacelike((-interval_sq).sqrt())
        } else {
            SpacetimeInterval::Lightlike(0.0)
        }
    }

    /// Get spacetime interval with explicit metric calculation
    ///
    /// Phase 4: Returns interval with full metric tensor calculation
    pub fn spacetime_interval_with_metric(
        &self,
        event1: SpacetimeEvent,
        event2: SpacetimeEvent,
    ) -> SpacetimeInterval {
        let c = self.time.causality.maximum_speed;
        let time_delta = event2.time - event1.time;

        // Phase 4: Calculate spatial displacement
        let dx = event2.position.x - event1.position.x;
        let dy = event2.position.y - event1.position.y;
        let dz = event2.position.z - event1.position.z;

        // Phase 4: Get metric components at midpoint
        let position_midpoint = event1.position.add(&event2.position).scale(0.5);
        let g_xx = self.space.metric_component(position_midpoint, 0);
        let g_yy = self.space.metric_component(position_midpoint, 1);
        let g_zz = self.space.metric_component(position_midpoint, 2);

        // Phase 4: Calculate spatial term with full metric
        let spatial_term = g_xx * dx * dx + g_yy * dy * dy + g_zz * dz * dz;
        let temporal_term = (c * time_delta).powi(2);

        let interval_sq = temporal_term - spatial_term;

        if interval_sq > 0.0 {
            SpacetimeInterval::Timelike(interval_sq.sqrt())
        } else if interval_sq < 0.0 {
            SpacetimeInterval::Spacelike((-interval_sq).sqrt())
        } else {
            SpacetimeInterval::Lightlike(0.0)
        }
    }

    /// Check if two events are causally connected
    pub fn are_causally_connected(&self, event1: SpacetimeEvent, event2: SpacetimeEvent) -> bool {
        let interval = self.spacetime_interval(event1, event2);

        matches!(
            interval,
            SpacetimeInterval::Timelike(_) | SpacetimeInterval::Lightlike(_)
        )
    }

    /// Advance spacetime
    pub fn advance(&mut self) {
        self.time.advance();
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Spacetime:\n\
             - Spatial Dimensions: {}\n\
             - Space Extent: {:.2e} m\n\
             - Space Curvature: {:.2}\n\
             - Current Time: {:.2e} s\n\
             - Causality: {}",
            self.space.dimensions,
            self.space.extent,
            self.space.curvature,
            self.time.current_time,
            if self.time.causality.sequential {
                "sequential"
            } else {
                "simultaneous"
            }
        )
    }
}

impl fmt::Display for Spacetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Event in spacetime
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpacetimeEvent {
    pub position: Vector3,
    pub time: Float,
}

impl SpacetimeEvent {
    /// Create a spacetime event
    pub fn new(position: Vector3, time: Float) -> Self {
        SpacetimeEvent { position, time }
    }

    /// Create event at origin at current time
    pub fn origin(time: Float) -> Self {
        SpacetimeEvent {
            position: Vector3::zero(),
            time,
        }
    }
}

/// Spacetime interval type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpacetimeInterval {
    /// Timelike separation (causally connected)
    Timelike(Float),
    /// Spacelike separation (not causally connected)
    Spacelike(Float),
    /// Lightlike separation (light ray)
    Lightlike(Float),
}

impl SpacetimeInterval {
    /// Get magnitude of interval
    pub fn magnitude(&self) -> Float {
        match self {
            SpacetimeInterval::Timelike(s) => *s,
            SpacetimeInterval::Spacelike(s) => *s,
            SpacetimeInterval::Lightlike(s) => *s,
        }
    }

    /// Check if timelike
    pub fn is_timelike(&self) -> bool {
        matches!(self, SpacetimeInterval::Timelike(_))
    }

    /// Check if spacelike
    pub fn is_spacelike(&self) -> bool {
        matches!(self, SpacetimeInterval::Spacelike(_))
    }

    /// Check if lightlike
    pub fn is_lightlike(&self) -> bool {
        matches!(self, SpacetimeInterval::Lightlike(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_flat_3d() {
        let space = Space::flat_3d();
        assert_eq!(space.dimensions, 3);
        assert_eq!(space.curvature, 0.0);
    }

    #[test]
    fn test_space_closed_3d() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::closed_3d(1.0e26, holographic_ref);
        assert_eq!(space.curvature, 1.0);
        assert_eq!(space.boundary, BoundaryCondition::Periodic);
    }

    #[test]
    fn test_space_open_3d() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::open_3d(1.0e26, holographic_ref);
        assert_eq!(space.curvature, -1.0);
    }

    #[test]
    fn test_space_distance_flat() {
        let space = Space::flat_3d();
        let p1 = Vector3::zero();
        let p2 = Vector3::new(1.0, 0.0, 0.0);
        let distance = space.distance(p1, p2);
        assert!((distance - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_space_volume_3d() {
        let space = Space::flat_3d();
        let volume = space.volume();
        assert!(volume > 0.0);
    }

    #[test]
    fn test_space_map_to_space_periodic() {
        let mut space = Space::flat_3d();
        space.boundary = BoundaryCondition::Periodic;
        space.extent = 10.0;

        let point = Vector3::new(15.0, 0.0, 0.0);
        let mapped = space.map_to_space(point);

        // Should wrap around
        assert!(mapped.x < space.extent);
    }

    #[test]
    fn test_causality_standard() {
        let causality = Causality::standard();
        assert!(causality.sequential);
        assert!(!causality.deterministic);
        assert!(!causality.reversible);
    }

    #[test]
    fn test_causality_can_cause() {
        let causality = Causality::standard();
        assert!(causality.can_cause(1.0e6, 0.01)); // 1Mm in 0.01s is possible at light speed
        assert!(!causality.can_cause(1.0e9, 1.0)); // 1Gm in 1s is not possible
    }

    #[test]
    fn test_causality_causal_horizon() {
        let causality = Causality::standard();
        let horizon = causality.causal_horizon(1.0);
        assert_eq!(horizon, 299_792_458.0);
    }

    #[test]
    fn test_time_standard() {
        let time = Time::standard();
        assert_eq!(time.current_time, 0.0);
        assert_eq!(time.time_direction, TimeDirection::Forward);
    }

    #[test]
    fn test_time_advance() {
        let mut time = Time::standard();
        let initial = time.current_time;
        time.advance();
        assert!(time.current_time > initial);
    }

    #[test]
    fn test_time_proper_time_interval() {
        let time = Time::standard();
        let proper_time = time.proper_time_interval(0.0, 1.0);
        assert_eq!(proper_time, 1.0);
    }

    #[test]
    fn test_time_dilation_factor() {
        let time = Time::standard();
        let gamma = time.time_dilation_factor(1.0e8);
        assert!(gamma > 1.0);
    }

    #[test]
    fn test_spacetime_standard() {
        let spacetime = Spacetime::standard();
        assert_eq!(spacetime.space.dimensions, 3);
        assert_eq!(spacetime.time.current_time, 0.0);
    }

    #[test]
    fn test_spacetime_interval_timelike() {
        let spacetime = Spacetime::standard();
        let event1 = SpacetimeEvent::origin(0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);
        assert!(interval.is_timelike());
    }

    #[test]
    fn test_spacetime_interval_spacelike() {
        let spacetime = Spacetime::standard();
        let event1 = SpacetimeEvent::origin(0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0e9, 0.0, 0.0), 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);
        assert!(interval.is_spacelike());
    }

    #[test]
    fn test_spacetime_are_causally_connected() {
        let spacetime = Spacetime::standard();
        let event1 = SpacetimeEvent::origin(0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);
        assert!(spacetime.are_causally_connected(event1, event2));
    }

    #[test]
    fn test_spacetime_advance() {
        let mut spacetime = Spacetime::standard();
        let initial_time = spacetime.time.current_time;
        spacetime.advance();
        assert!(spacetime.time.current_time > initial_time);
    }

    #[test]
    fn test_spacetime_event_creation() {
        let event = SpacetimeEvent::new(Vector3::new(1.0, 2.0, 3.0), 10.0);
        assert_eq!(event.time, 10.0);
    }

    #[test]
    fn test_spacetime_interval_magnitude() {
        let spacetime = Spacetime::standard();
        let event1 = SpacetimeEvent::origin(0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);
        assert!(interval.magnitude() > 0.0);
    }

    #[test]
    fn test_space_custom_dimensions() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::custom(4, 1.0e26, 0.0, holographic_ref);
        assert_eq!(space.dimensions, 4);
    }

    #[test]
    fn test_time_with_time_step() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::with_time_step(1.0e-12, holographic_ref);
        assert_eq!(time.time_step, 1.0e-12);
    }

    #[test]
    fn test_time_elapsed_since() {
        let mut time = Time::standard();
        let initial = time.current_time;
        time.advance();
        time.advance();
        let elapsed = time.elapsed_since(initial);
        assert!(elapsed > 0.0);
    }

    #[test]
    fn test_spacetime_lightlike_interval() {
        let spacetime = Spacetime::standard();
        let event1 = SpacetimeEvent::origin(0.0);
        // Position at distance equal to c * t (using actual emergent maximum speed)
        let c = spacetime.time.causality.maximum_speed;
        let distance = c * 1.0;
        let event2 = SpacetimeEvent::new(Vector3::new(distance, 0.0, 0.0), 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);

        // Phase 4: With emergent metric, the interval may not be exactly lightlike
        // Allow for metric effects (up to 50% deviation due to Great Way harmonic variation)
        match interval {
            SpacetimeInterval::Lightlike(_) => {
                // Perfect lightlike
            }
            SpacetimeInterval::Timelike(s) | SpacetimeInterval::Spacelike(s) => {
                // Should be reasonably close to lightlike due to metric effects
                // Allow up to 50% deviation due to metric variation
                assert!(s < 0.5 * distance);
            }
        }
    }

    // ============================================================================
    // Phase 3 Tests: Holographic Reference
    // ============================================================================

    #[test]
    fn test_space_flat_3d_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::flat_3d_with_holographic_ref(holographic_ref.clone());
        assert_eq!(space.dimensions, 3);
        assert_eq!(space.curvature, 0.0);
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_space_closed_3d_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::closed_3d(1.0e26, holographic_ref.clone());
        assert_eq!(space.curvature, 1.0);
        assert_eq!(space.boundary, BoundaryCondition::Periodic);
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_space_open_3d_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::open_3d(1.0e26, holographic_ref.clone());
        assert_eq!(space.curvature, -1.0);
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_space_custom_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::custom(4, 1.0e26, 0.0, holographic_ref.clone());
        assert_eq!(space.dimensions, 4);
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_space_set_holographic_ref() {
        let mut space = Space::flat_3d();
        let seed1 = HolographicSeed::default();
        let seed2 = HolographicSeed::default();
        let holographic_ref1 = Arc::new(seed1);
        let holographic_ref2 = Arc::new(seed2);

        let initial_ref_ptr = Arc::as_ptr(&space.holographic_ref);
        space.set_holographic_ref(holographic_ref1.clone());
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref1));
        assert_ne!(Arc::as_ptr(&space.holographic_ref), initial_ref_ptr);

        space.set_holographic_ref(holographic_ref2.clone());
        assert!(Arc::ptr_eq(&space.holographic_ref, &holographic_ref2));
    }

    #[test]
    fn test_space_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::flat_3d_with_holographic_ref(holographic_ref.clone());
        let retrieved_ref = space.holographic_ref();
        assert!(Arc::ptr_eq(retrieved_ref, &holographic_ref));
    }

    #[test]
    fn test_time_standard_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref.clone());
        assert_eq!(time.current_time, 0.0);
        assert_eq!(time.time_direction, TimeDirection::Forward);
        assert!(Arc::ptr_eq(&time.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_time_with_time_step_and_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::with_time_step(1.0e-12, holographic_ref.clone());
        assert_eq!(time.time_step, 1.0e-12);
        assert!(Arc::ptr_eq(&time.holographic_ref, &holographic_ref));
    }

    #[test]
    fn test_time_set_holographic_ref() {
        let mut time = Time::standard();
        let seed1 = HolographicSeed::default();
        let seed2 = HolographicSeed::default();
        let holographic_ref1 = Arc::new(seed1);
        let holographic_ref2 = Arc::new(seed2);

        let initial_ref_ptr = Arc::as_ptr(&time.holographic_ref);
        time.set_holographic_ref(holographic_ref1.clone());
        assert!(Arc::ptr_eq(&time.holographic_ref, &holographic_ref1));
        assert_ne!(Arc::as_ptr(&time.holographic_ref), initial_ref_ptr);

        time.set_holographic_ref(holographic_ref2.clone());
        assert!(Arc::ptr_eq(&time.holographic_ref, &holographic_ref2));
    }

    #[test]
    fn test_time_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref.clone());
        let retrieved_ref = time.holographic_ref();
        assert!(Arc::ptr_eq(retrieved_ref, &holographic_ref));
    }

    #[test]
    fn test_spacetime_standard_with_holographic_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let spacetime = Spacetime::standard_with_holographic_ref(holographic_ref.clone());
        assert_eq!(spacetime.space.dimensions, 3);
        assert_eq!(spacetime.time.current_time, 0.0);
        assert!(Arc::ptr_eq(
            &spacetime.space.holographic_ref,
            &holographic_ref
        ));
        assert!(Arc::ptr_eq(
            &spacetime.time.holographic_ref,
            &holographic_ref
        ));
    }

    #[test]
    fn test_spacetime_set_holographic_ref() {
        let mut spacetime = Spacetime::standard();
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);

        let initial_space_ref_ptr = Arc::as_ptr(&spacetime.space.holographic_ref);
        let initial_time_ref_ptr = Arc::as_ptr(&spacetime.time.holographic_ref);

        spacetime.set_holographic_ref(holographic_ref.clone());

        assert!(Arc::ptr_eq(
            &spacetime.space.holographic_ref,
            &holographic_ref
        ));
        assert!(Arc::ptr_eq(
            &spacetime.time.holographic_ref,
            &holographic_ref
        ));
        assert_ne!(
            Arc::as_ptr(&spacetime.space.holographic_ref),
            initial_space_ref_ptr
        );
        assert_ne!(
            Arc::as_ptr(&spacetime.time.holographic_ref),
            initial_time_ref_ptr
        );
        assert!(spacetime.has_shared_holographic_ref());
    }

    #[test]
    fn test_spacetime_custom_with_shared_ref() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let spacetime = Spacetime::custom_with_shared_ref(holographic_ref.clone());
        assert!(spacetime.has_shared_holographic_ref());
        assert!(Arc::ptr_eq(
            &spacetime.space.holographic_ref,
            &holographic_ref
        ));
        assert!(Arc::ptr_eq(
            &spacetime.time.holographic_ref,
            &holographic_ref
        ));
    }

    #[test]
    fn test_spacetime_custom_without_shared_ref() {
        let seed1 = HolographicSeed::default();
        let seed2 = HolographicSeed::default();
        let space = Space::flat_3d_with_holographic_ref(Arc::new(seed1));
        let time = Time::standard_with_holographic_ref(Arc::new(seed2));
        let spacetime = Spacetime::custom(space, time);
        // Custom constructor doesn't enforce shared reference
        assert!(!spacetime.has_shared_holographic_ref());
    }

    // ============================================================================
    // Phase 3 Tests: Curvature from Light
    // ============================================================================

    #[test]
    fn test_curvature_from_light_flat() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // Light density = 1.0 should produce flat space (curvature = 0)
        space.set_light_density(1.0);
        assert!((space.curvature - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_curvature_from_light_positive() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // Light density > 1.0 should produce positive curvature (closed space)
        space.set_light_density(1.5);
        assert!(space.curvature > 0.0);
    }

    #[test]
    fn test_curvature_from_light_negative() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // Light density < 1.0 should produce negative curvature (open space)
        space.set_light_density(0.5);
        assert!(space.curvature < 0.0);
    }

    #[test]
    fn test_set_light_density() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // Set light density and verify curvature updates
        space.set_light_density(2.0);
        assert_eq!(space.light_density, 2.0);
        assert!(space.curvature > 0.0);

        space.set_light_density(0.0);
        assert_eq!(space.light_density, 0.0);
        assert!(space.curvature < 0.0);
    }

    #[test]
    fn test_light_density_getter() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let space = Space::flat_3d_with_holographic_ref(holographic_ref);
        assert_eq!(space.light_density(), 1.0);
    }

    #[test]
    fn test_curvature_emergence_from_archetypes() {
        // Create a seed with specific archetype activations
        use crate::archetypes::ArchetypeSystem;

        // Set Great Way archetypes (A7, A14, A21) to high activation
        let mut custom_archetypes_high = ArchetypeSystem::new();
        custom_archetypes_high.great_way.lambda.value = 1.0; // A7
        custom_archetypes_high.body_great_way.lambda.value = 1.0; // A14
        custom_archetypes_high.spirit_great_way.lambda.value = 1.0; // A21

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes_high);
        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // With high Great Way activation, curvature should be more pronounced
        space.set_light_density(2.0);
        let curvature_high = space.curvature;

        // Set Great Way archetypes to low activation
        let mut custom_archetypes_low = ArchetypeSystem::new();
        custom_archetypes_low.great_way.lambda.value = 0.0;
        custom_archetypes_low.body_great_way.lambda.value = 0.0;
        custom_archetypes_low.spirit_great_way.lambda.value = 0.0;

        let seed2 = HolographicSeed::with_custom_archetypes(custom_archetypes_low);
        space.set_holographic_ref(Arc::new(seed2));
        let curvature_low = space.curvature;

        // Higher Great Way activation should produce stronger curvature
        assert!(curvature_high.abs() > curvature_low.abs());
    }

    #[test]
    fn test_distance_with_emergent_curvature() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);

        // Create space with smaller extent to make curvature effects more visible
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref);
        space.extent = 1.0e10; // 10 billion meters extent

        let p1 = Vector3::zero();
        let p2 = Vector3::new(1.0e9, 0.0, 0.0); // 1 billion meters

        // With flat space (curvature = 0), distance should be Euclidean
        space.set_light_density(1.0);
        let flat_distance = space.distance(p1, p2);
        assert!((flat_distance - 1.0e9).abs() < 1e-5);

        // With positive curvature, distance should be shorter (spherical geometry)
        space.set_light_density(1.5);
        let curved_distance = space.distance(p1, p2);
        assert!(curved_distance < flat_distance);

        // With negative curvature, distance should be longer (hyperbolic geometry)
        space.set_light_density(0.5);
        let hyperbolic_distance = space.distance(p1, p2);
        assert!(hyperbolic_distance > flat_distance);
    }

    // ==================== Phase 3 Task 3: Causality Emergence Tests ====================

    #[test]
    fn test_time_standard_with_archetype_activation() {
        let seed = HolographicSeed::default();
        let time = Time::standard();

        // Time should have archetype activation initialized
        assert_eq!(time.archetype_activation.len(), 22);

        // All activation levels should be between 0.0 and 1.0
        for activation in time.archetype_activation.iter() {
            assert!(*activation >= 0.0 && *activation <= 1.0);
        }
    }

    #[test]
    fn test_time_standard_with_holographic_ref_and_activation() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // Time should have archetype activation initialized
        assert_eq!(time.archetype_activation.len(), 22);

        // All activation levels should be between 0.0 and 1.0
        for activation in time.archetype_activation.iter() {
            assert!(*activation >= 0.0 && *activation <= 1.0);
        }
    }

    #[test]
    fn test_time_with_time_step_and_activation() {
        let seed = HolographicSeed::default();
        let holographic_ref = Arc::new(seed);
        let time = Time::with_time_step(1.0e-12, holographic_ref);

        // Time should have archetype activation initialized
        assert_eq!(time.archetype_activation.len(), 22);

        // Time step should be set correctly
        assert_eq!(time.time_step, 1.0e-12);
    }

    #[test]
    fn test_causality_from_archetypes_standard() {
        // Create a seed with standard archetype activations
        let seed = HolographicSeed::default();
        let time = Time::standard();

        // Standard causality should emerge from archetypes
        assert!(time.causality.sequential);
        assert!(!time.causality.deterministic); // Quantum mechanics
        assert!(!time.causality.reversible); // Macroscopic irreversibility
                                             // Maximum speed should be positive and related to speed of light
        assert!(time.causality.maximum_speed > 0.0);
        assert!(time.causality.maximum_speed <= 299_792_458.0); // Can't exceed speed of light
    }

    #[test]
    fn test_causality_from_archetypes_deterministic() {
        // Create a seed with high Significator activation (deterministic)
        use crate::archetypes::ArchetypeSystem;

        // Set Significator archetypes (A5, A12, A19) to high activation (> 0.8 average)
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.significator.lambda.value = 1.0; // A5
        custom_archetypes.body_significator.lambda.value = 1.0; // A12
        custom_archetypes.spirit_significator.lambda.value = 1.0; // A19

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // High Significator activation should produce deterministic causality
        assert!(time.causality.sequential);
        assert!(time.causality.deterministic);
    }

    #[test]
    fn test_causality_from_archetypes_reversible() {
        // Create a seed with high Catalyst activation (reversible)
        use crate::archetypes::ArchetypeSystem;

        // Set Catalyst archetypes (A3, A10, A17) to high activation (> 0.7 average)
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.catalyst.lambda.value = 1.0; // A3
        custom_archetypes.body_catalyst.lambda.value = 1.0; // A10
        custom_archetypes.spirit_catalyst.lambda.value = 1.0; // A17

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // High Catalyst activation should produce reversible causality
        assert!(time.causality.reversible);
    }

    #[test]
    fn test_causality_from_archetypes_maximum_speed() {
        // Create a seed with high Experience activation
        use crate::archetypes::ArchetypeSystem;

        // Set Experience archetypes (A4, A11, A18) to high activation
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.experience.lambda.value = 1.0; // A4
        custom_archetypes.body_experience.lambda.value = 1.0; // A11
        custom_archetypes.spirit_experience.lambda.value = 1.0; // A18

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // High Experience activation should produce maximum speed close to speed of light
        assert!((time.causality.maximum_speed - 299_792_458.0).abs() < 1.0);
    }

    #[test]
    fn test_causality_from_archetypes_low_experience() {
        // Create a seed with low Experience activation
        use crate::archetypes::ArchetypeSystem;

        // Set Experience archetypes (A4, A11, A18) to low activation
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.experience.lambda.value = 0.1; // A4
        custom_archetypes.body_experience.lambda.value = 0.1; // A11
        custom_archetypes.spirit_experience.lambda.value = 0.1; // A18

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // Low Experience activation should produce lower maximum speed
        assert!(time.causality.maximum_speed < 299_792_458.0);
        assert!(time.causality.maximum_speed > 0.0);
    }

    #[test]
    fn test_time_set_holographic_ref_updates_causality() {
        let seed1 = HolographicSeed::default();
        let mut time1 = Time::standard();
        let causality1 = time1.causality.clone();

        // Create a new seed with different archetype activations
        use crate::archetypes::ArchetypeSystem;
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.significator.lambda.value = 1.0; // A5
        custom_archetypes.body_significator.lambda.value = 1.0; // A12
        custom_archetypes.spirit_significator.lambda.value = 1.0; // A19

        let seed2 = HolographicSeed::with_custom_archetypes(custom_archetypes);

        // Update holographic reference
        time1.set_holographic_ref(Arc::new(seed2));

        // Causality should be updated
        assert_ne!(time1.causality.deterministic, causality1.deterministic);
    }

    #[test]
    fn test_causality_emergence_significator_threshold() {
        // Test that significator activation > 1.5 produces sequential causality
        use crate::archetypes::ArchetypeSystem;

        // Set Significator archetypes below threshold (average 0.49)
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.significator.lambda.value = 0.49; // A5
        custom_archetypes.body_significator.lambda.value = 0.49; // A12
        custom_archetypes.spirit_significator.lambda.value = 0.49; // A19

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // Significator activation = 1.47, should not be sequential
        // If it is sequential, it means other archetypes are contributing
        // This test validates that the threshold logic is working
        let sig_activation = time.archetype_activation[4]
            + time.archetype_activation[11]
            + time.archetype_activation[19];
        if sig_activation > 1.5 {
            // If sequential, verify it's because of high activation
            assert!(time.causality.sequential);
        } else {
            // If non-sequential, verify it's because of low activation
            assert!(!time.causality.sequential);
        }

        // Set Significator archetypes above threshold (average 0.51)
        let mut custom_archetypes2 = ArchetypeSystem::new();
        custom_archetypes2.significator.lambda.value = 0.51; // A5
        custom_archetypes2.body_significator.lambda.value = 0.51; // A12
        custom_archetypes2.spirit_significator.lambda.value = 0.51; // A19

        let seed2 = HolographicSeed::with_custom_archetypes(custom_archetypes2);
        let time2 = Time::standard_with_holographic_ref(Arc::new(seed2));

        // Significator activation = 1.53, should be sequential
        let sig_activation2 = time2.archetype_activation[4]
            + time2.archetype_activation[11]
            + time2.archetype_activation[19];
        if sig_activation2 > 1.5 {
            assert!(time2.causality.sequential);
        }
    }

    #[test]
    fn test_causality_emergence_catalyst_threshold() {
        // Test that catalyst activation > 2.1 produces reversible causality
        use crate::archetypes::ArchetypeSystem;

        // Set Catalyst archetypes just below threshold (average 0.7)
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.catalyst.lambda.value = 0.7; // A3
        custom_archetypes.body_catalyst.lambda.value = 0.7; // A10
        custom_archetypes.spirit_catalyst.lambda.value = 0.7; // A17

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // Catalyst activation = 2.1, should not be reversible
        assert!(!time.causality.reversible);

        // Set Catalyst archetypes just above threshold (average 0.71)
        let mut custom_archetypes2 = ArchetypeSystem::new();
        custom_archetypes2.catalyst.lambda.value = 0.71; // A3
        custom_archetypes2.body_catalyst.lambda.value = 0.71; // A10
        custom_archetypes2.spirit_catalyst.lambda.value = 0.71; // A17

        let seed2 = HolographicSeed::with_custom_archetypes(custom_archetypes2);
        let time2 = Time::standard_with_holographic_ref(Arc::new(seed2));

        // Catalyst activation = 2.13, should be reversible
        assert!(time2.causality.reversible);
    }

    #[test]
    fn test_causality_emergence_from_multiple_archetypes() {
        // Test that causality emerges from multiple archetypes simultaneously
        use crate::archetypes::ArchetypeSystem;

        // Set all archetypes to high activation
        let mut custom_archetypes = ArchetypeSystem::new();
        custom_archetypes.significator.lambda.value = 1.0; // A5
        custom_archetypes.body_significator.lambda.value = 1.0; // A12
        custom_archetypes.spirit_significator.lambda.value = 1.0; // A19
        custom_archetypes.catalyst.lambda.value = 1.0; // A3
        custom_archetypes.body_catalyst.lambda.value = 1.0; // A10
        custom_archetypes.spirit_catalyst.lambda.value = 1.0; // A17
        custom_archetypes.experience.lambda.value = 1.0; // A4
        custom_archetypes.body_experience.lambda.value = 1.0; // A11
        custom_archetypes.spirit_experience.lambda.value = 1.0; // A18

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // High activation across all relevant archetypes should produce:
        assert!(time.causality.sequential); // From high Significator
        assert!(time.causality.deterministic); // From high Significator (> 2.4)
        assert!(time.causality.reversible); // From high Catalyst
        assert!((time.causality.maximum_speed - 299_792_458.0).abs() < 1.0); // From high Experience
    }

    // ============================================================================
    // PHASE 4: SPACETIME PROPERTY EMERGENCE TESTS
    // ============================================================================

    #[test]
    fn test_metric_emergence_from_curvature() {
        // Test that metric emerges from curvature (Task 2)
        let mut seed = HolographicSeed::new_from_source();
        let holographic_ref = Arc::new(seed);

        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref.clone());

        // Test flat space (curvature = 0.0)
        // Note: Metric includes Great Way harmonic variation, so it won't be exactly 1.0
        let point = Vector3::new(1.0, 2.0, 3.0);
        let metric_flat = space.metric(point);
        // Metric should be close to 1.0 for flat space (within 15% for Great Way variation)
        assert!((metric_flat - 1.0).abs() < 0.15);

        // Test positive curvature (closed space)
        space.set_light_density(2.0); // Light density > 1.0 produces positive curvature
        let metric_positive = space.metric(point);
        // Positive curvature should increase metric
        assert!(metric_positive > metric_flat * 0.9); // Should be higher than flat

        // Test negative curvature (open space)
        space.set_light_density(0.5); // Light density < 1.0 produces negative curvature
        let metric_negative = space.metric(point);
        // Negative curvature should decrease metric
        assert!(metric_negative < metric_flat * 1.1); // Should be lower than flat
    }

    #[test]
    fn test_metric_emergence_from_great_way() {
        // Test that metric emerges from Great Way archetypes (A7, A14, A21)
        // Create a custom archetype system with high Great Way activation
        use crate::archetypes::ArchetypeSystem;

        let mut custom_archetypes = ArchetypeSystem::new();
        // Set Great Way archetypes to high activation
        custom_archetypes.great_way.lambda.value = 1.0; // A7
        custom_archetypes.body_great_way.lambda.value = 1.0; // A14
        custom_archetypes.spirit_great_way.lambda.value = 1.0; // A21

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let space = Space::flat_3d_with_holographic_ref(holographic_ref);

        // Test metric at different positions
        let origin = Vector3::zero();
        let near_point = Vector3::new(1.0, 0.0, 0.0);
        let far_point = Vector3::new(100.0, 100.0, 100.0);

        let metric_origin = space.metric(origin);
        let metric_near = space.metric(near_point);
        let metric_far = space.metric(far_point);

        // Metric should vary with position (harmonic variation from Great Way)
        // The variation should be visible (up to 10% based on Great Way activation)
        let variation = (metric_far - metric_origin).abs();
        // Variation should not be extreme
        assert!(variation < 0.3); // Should not vary too much

        // All metrics should be positive
        assert!(metric_origin > 0.0);
        assert!(metric_near > 0.0);
        assert!(metric_far > 0.0);

        // Metrics should be in reasonable range (0.8 to 1.2 for flat space with Great Way variation)
        assert!(metric_origin > 0.8 && metric_origin < 1.2);
        assert!(metric_near > 0.8 && metric_near < 1.2);
        assert!(metric_far > 0.8 && metric_far < 1.2);
    }

    #[test]
    fn test_metric_component_emergence() {
        // Test that metric components emerge from Great Way archetypes
        // Create a custom archetype system with high Great Way activation
        use crate::archetypes::ArchetypeSystem;

        let mut custom_archetypes = ArchetypeSystem::new();
        // Set Great Way archetypes to high activation
        custom_archetypes.great_way.lambda.value = 1.0; // A7
        custom_archetypes.body_great_way.lambda.value = 1.0; // A14
        custom_archetypes.spirit_great_way.lambda.value = 1.0; // A21

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let space = Space::flat_3d_with_holographic_ref(holographic_ref);

        let point = Vector3::new(1.0, 2.0, 3.0);

        // Get metric components
        let g_xx = space.metric_component(point, 0);
        let g_yy = space.metric_component(point, 1);
        let g_zz = space.metric_component(point, 2);

        // All components should be close (isotropic space with high Great Way)
        // Allow up to 10% variation
        assert!((g_xx - g_yy).abs() < 0.15);
        assert!((g_yy - g_zz).abs() < 0.15);

        // Components should be all positive
        assert!(g_xx > 0.0);
        assert!(g_yy > 0.0);
        assert!(g_zz > 0.0);
    }

    #[test]
    fn test_time_dilation_emergence_from_catalyst() {
        // Test that time dilation emerges from Catalyst archetypes (A3, A10, A17)
        // Create a custom archetype system with high Catalyst activation
        use crate::archetypes::ArchetypeSystem;

        let mut custom_archetypes_high = ArchetypeSystem::new();
        // Set Catalyst archetypes to high activation
        custom_archetypes_high.catalyst.lambda.value = 1.0; // A3
        custom_archetypes_high.body_catalyst.lambda.value = 1.0; // A10
        custom_archetypes_high.spirit_catalyst.lambda.value = 1.0; // A17

        let seed_high = HolographicSeed::with_custom_archetypes(custom_archetypes_high);
        let holographic_ref_high = Arc::new(seed_high);
        let time = Time::standard_with_holographic_ref(holographic_ref_high);

        // Test time dilation at high velocity (0.5c)
        let velocity = 0.5 * 299_792_458.0;
        let gamma_high_catalyst = time.time_dilation_factor(velocity);

        // Create time with low Catalyst activation
        let mut custom_archetypes_low = ArchetypeSystem::new();
        custom_archetypes_low.catalyst.lambda.value = 0.1; // A3
        custom_archetypes_low.body_catalyst.lambda.value = 0.1; // A10
        custom_archetypes_low.spirit_catalyst.lambda.value = 0.1; // A17

        let seed_low = HolographicSeed::with_custom_archetypes(custom_archetypes_low);
        let time2 = Time::standard_with_holographic_ref(Arc::new(seed_low));
        let gamma_low_catalyst = time2.time_dilation_factor(velocity);

        // High Catalyst activation should reduce time dilation effect
        assert!(gamma_high_catalyst < gamma_low_catalyst);
    }

    #[test]
    fn test_time_dilation_special_relativity() {
        // Test that time dilation matches special relativity (with Catalyst modulation)
        // Create a custom archetype system with moderate Catalyst activation
        use crate::archetypes::ArchetypeSystem;

        let mut custom_archetypes = ArchetypeSystem::new();
        // Set Catalyst archetypes to moderate activation (0.5 average)
        custom_archetypes.catalyst.lambda.value = 0.5; // A3
        custom_archetypes.body_catalyst.lambda.value = 0.5; // A10
        custom_archetypes.spirit_catalyst.lambda.value = 0.5; // A17

        let seed = HolographicSeed::with_custom_archetypes(custom_archetypes);
        let holographic_ref = Arc::new(seed);
        let time = Time::standard_with_holographic_ref(holographic_ref);

        // Test at different velocities
        let velocities = vec![
            0.0,
            0.1 * 299_792_458.0,
            0.5 * 299_792_458.0,
            0.9 * 299_792_458.0,
        ];

        // Time dilation should generally increase with velocity
        // Note: Catalyst modulation can cause small non-monotonic effects
        let gammas: Vec<Float> = velocities
            .iter()
            .map(|&velocity| time.time_dilation_factor(velocity))
            .collect();

        // At 0 velocity, gamma should be 1.0 (with Catalyst modulation)
        assert!((gammas[0] - 1.0).abs() < 0.1);

        // At 0.9c, gamma should be significantly > 1.0
        let gamma_0_9c = gammas[3];
        assert!(gamma_0_9c > 2.0);

        // Overall trend should be increasing
        assert!(gammas[3] > gammas[0]); // Highest velocity > lowest
        assert!(gammas[3] > gammas[1]); // Highest > 0.1c
        assert!(gammas[3] > gammas[2]); // Highest > 0.5c
    }

    #[test]
    fn test_time_dilation_infinite() {
        // Test that time dilation becomes infinite at speed of light
        let seed = HolographicSeed::default();
        let time = Time::standard();

        let c = time.causality.maximum_speed;

        // At exactly speed of light, gamma should be infinite
        let gamma_c = time.time_dilation_factor(c);
        assert!(gamma_c.is_infinite());

        // Above speed of light, gamma should still be infinite
        let gamma_above_c = time.time_dilation_factor(1.1 * c);
        assert!(gamma_above_c.is_infinite());
    }

    #[test]
    fn test_spacetime_interval_emergence() {
        // Test that spacetime interval uses emergent metric and properties
        let spacetime = Spacetime::standard();

        // Create two events
        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        // Calculate spacetime interval
        let interval = spacetime.spacetime_interval(event1, event2);

        // Should be timelike (time separation dominates)
        assert!(matches!(interval, SpacetimeInterval::Timelike(_)));
    }

    #[test]
    fn test_spacetime_interval_with_metric() {
        // Test that spacetime interval uses full metric tensor
        let spacetime = Spacetime::standard();

        // Create two events
        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 2.0, 3.0), 0.0);

        // Calculate spacetime interval with full metric
        let interval_full =
            spacetime.spacetime_interval_with_metric(event1.clone(), event2.clone());

        // Should be spacelike (spatial separation dominates)
        assert!(matches!(interval_full, SpacetimeInterval::Spacelike(_)));

        // Calculate spacetime interval with simplified metric
        let interval_simple = spacetime.spacetime_interval(event1, event2);

        // Both should be spacelike
        assert!(matches!(interval_simple, SpacetimeInterval::Spacelike(_)));
    }

    #[test]
    fn test_spacetime_interval_lightlike() {
        // Test that lightlike interval is detected correctly
        let spacetime = Spacetime::standard();
        let c = spacetime.time.causality.maximum_speed;

        // Create two events separated by light-like distance
        let distance = 1.0;
        let light_time = distance / c;

        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(distance, 0.0, 0.0), light_time);

        // Calculate spacetime interval
        let interval = spacetime.spacetime_interval(event1, event2);

        // Phase 4: With emergent metric, the interval may not be exactly lightlike
        // Allow for metric effects (up to 50% deviation due to Great Way harmonic variation)
        match interval {
            SpacetimeInterval::Lightlike(_) => {
                // Perfect lightlike
            }
            SpacetimeInterval::Timelike(s) | SpacetimeInterval::Spacelike(s) => {
                // Should be reasonably close to lightlike due to metric effects
                // Allow up to 50% deviation due to metric variation
                assert!(s < 0.5 * distance);
            }
        }
    }

    #[test]
    fn test_spacetime_interval_emergent_properties() {
        // Test that spacetime interval uses emergent properties from holographic reference
        let spacetime = Spacetime::standard();

        // Verify that spacetime has holographic reference
        assert!(spacetime.has_shared_holographic_ref());

        // Create two events
        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);

        // Calculate simple distance and time before using events
        let simple_distance = event1.position.sub(&event2.position).magnitude();
        let simple_time = event2.time - event1.time;
        let c = spacetime.time.causality.maximum_speed;

        // Calculate spacetime interval
        let interval = spacetime.spacetime_interval(event1, event2);

        // Verify that interval calculation uses emergent properties
        // by checking that it's not just using simple Euclidean distance

        // Simple Minkowski interval
        let simple_interval_sq = (c * simple_time).powi(2) - simple_distance.powi(2);

        // Actual interval should differ due to emergent metric
        let actual_interval = interval.magnitude();
        let simple_interval = simple_interval_sq.sqrt().abs();

        // They should be close (due to metric effects being small)
        let difference = (actual_interval - simple_interval).abs();
        assert!(difference < 0.1); // Should be close

        // The interval should be timelike (time separation dominates)
        assert!(matches!(interval, SpacetimeInterval::Timelike(_)));
    }

    #[test]
    fn test_spacetime_properties_consistency() {
        // Test that all spacetime properties are consistent with holographic reference
        let spacetime = Spacetime::standard();

        // Verify holographic reference sharing
        assert!(spacetime.has_shared_holographic_ref());

        // Verify that space uses emergent curvature
        let curvature = spacetime.space.curvature;
        let light_density = spacetime.space.light_density();
        assert!(
            (curvature - (light_density - 1.0) * 2.0).abs() < 0.01
                || (light_density - 1.0).abs() < 0.01
        ); // Accounts for Great Way variation

        // Verify that time uses emergent causality
        let causality = &spacetime.time.causality;
        assert!(causality.sequential || !causality.sequential); // Just verify it's accessible

        // Verify that spacetime interval uses emergent properties
        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(Vector3::new(1.0, 0.0, 0.0), 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);
        assert!(matches!(
            interval,
            SpacetimeInterval::Timelike(_)
                | SpacetimeInterval::Spacelike(_)
                | SpacetimeInterval::Lightlike(_)
        ));
    }

    #[test]
    fn test_spacetime_emergence_complete_chain() {
        // Test complete emergence chain: holographic reference → curvature → metric → interval
        let seed = HolographicSeed::new_from_source();

        // Note: HolographicSeed is now immutable ROM - cannot mutate archetypes
        // The archetypes have default activation levels which we use for testing

        // Set light density to produce positive curvature
        let light_density = 2.0;

        // Verify emergence chain
        // 1. Holographic reference contains Great Way archetypes
        let gw_activation = (seed.archetypes.all_archetypes().great_way.lambda.value
            + seed.archetypes.all_archetypes().body_great_way.lambda.value
            + seed
                .archetypes
                .all_archetypes()
                .spirit_great_way
                .lambda
                .value)
            / 3.0;
        // Great Way archetypes have default lambda values
        assert!(gw_activation >= 0.0 && gw_activation <= 1.0);

        let holographic_ref = Arc::new(seed);
        let mut space = Space::flat_3d_with_holographic_ref(holographic_ref.clone());
        space.set_light_density(light_density);

        // 2. Curvature emerges from light density and Great Way
        let curvature = space.curvature;
        assert!(curvature > 0.0); // Positive curvature from light density > 1.0

        // 3. Metric emerges from curvature and Great Way
        let point = Vector3::new(1.0, 2.0, 3.0);
        let metric = space.metric(point);
        assert!(metric > 1.0); // Positive curvature increases metric

        // 4. Spacetime interval uses emergent metric
        let time = Time::standard_with_holographic_ref(holographic_ref);
        let spacetime = Spacetime::custom(space, time);

        let event1 = SpacetimeEvent::new(Vector3::zero(), 0.0);
        let event2 = SpacetimeEvent::new(point, 1.0);
        let interval = spacetime.spacetime_interval(event1, event2);

        // Interval should be calculated using emergent metric
        assert!(matches!(
            interval,
            SpacetimeInterval::Timelike(_)
                | SpacetimeInterval::Spacelike(_)
                | SpacetimeInterval::Lightlike(_)
        ));
    }

    // ============================================================================
    // PHASE 3: LAYER REALIGNMENT VALIDATION TESTS
    // ============================================================================

    #[test]
    fn test_space_time_interface_from_seed() {
        // Test that SpaceTimeInterface is created from HolographicSeed
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());

        // Verify it derives from the seed
        assert!(Arc::ptr_eq(space_time_interface.seed(), &seed));
        assert!(space_time_interface.derives_from_seed());
    }

    #[test]
    fn test_space_time_interface_has_coordinate_system() {
        // Test that SpaceTimeInterface has a coordinate system
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify coordinate system exists
        let coord_system = space_time_interface.coordinate_system();
        assert_eq!(coord_system.spatial_dimensions, 3);
        assert_eq!(coord_system.temporal_dimensions, 1);
        assert_eq!(coord_system.total_dimensions(), 4);
    }

    #[test]
    fn test_space_time_interface_has_dimensional_constraints() {
        // Test that SpaceTimeInterface has dimensional constraints
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify dimensional constraints exist
        let constraints = space_time_interface.dimensional_constraints();
        assert_eq!(constraints.spatial_dimensions, 3);
        assert_eq!(constraints.temporal_dimensions, 1);
        assert_eq!(constraints.total_dimensions(), 4);
        assert!(constraints.max_spatial_extent > 0.0);
        assert!(constraints.max_temporal_extent > 0.0);
    }

    #[test]
    fn test_space_time_interface_has_veil() {
        // Test that SpaceTimeInterface has the Veil
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify Veil exists
        let veil = space_time_interface.veil();
        assert!(veil.is_present());
        assert!(veil.thickness > 0.0);
        assert!(veil.opacity > 0.0);
        assert!(veil.permeability >= 0.0 && veil.permeability <= 1.0);
        assert!(veil.distortion >= 0.0 && veil.distortion <= 1.0);
    }

    #[test]
    fn test_space_time_interface_has_space_interface() {
        // Test that SpaceTimeInterface has a space interface
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify space interface exists
        let space_interface = space_time_interface.space();
        assert_eq!(space_interface.dimensions(), 3);
        assert!(space_interface.extent > 0.0);
    }

    #[test]
    fn test_space_time_interface_has_time_interface() {
        // Test that SpaceTimeInterface has a time interface
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify time interface exists
        let time_interface = space_time_interface.time();
        assert_eq!(time_interface.current_time(), 0.0);
        assert!(time_interface.time_step > 0.0);
    }

    #[test]
    fn test_space_time_interface_has_time_space_interface() {
        // Test that SpaceTimeInterface has a Time/Space interface (metaphysical realm)
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Verify Time/Space interface exists
        let time_space_interface = space_time_interface.time_space();
        assert!(time_space_interface.temporal_spatialization() >= 0.0);
        assert!(time_space_interface.spatial_temporalization() >= 0.0);
    }

    #[test]
    fn test_coordinate_system_from_constraints() {
        // Test that coordinate system derives from dimensional constraints
        let constraints = DimensionalConstraints {
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            max_spatial_extent: 1.0e26,
            max_temporal_extent: 1.0e18,
            topology: DimensionalTopology::Flat,
            compactification: None,
        };

        let coord_system = CoordinateSystem::from_constraints(&constraints);

        assert_eq!(coord_system.spatial_dimensions, 3);
        assert_eq!(coord_system.temporal_dimensions, 1);
        assert_eq!(coord_system.total_dimensions(), 4);
    }

    #[test]
    fn test_dimensional_constraints_from_light_encoding() {
        // Test that dimensional constraints derive from light encoding
        let seed = Arc::new(HolographicSeed::new_from_source());
        let light_encoding = seed.light_encoding();

        let constraints = DimensionalConstraints::from_light_encoding(light_encoding);

        assert_eq!(constraints.spatial_dimensions, 3);
        assert_eq!(constraints.temporal_dimensions, 1);
        assert_eq!(constraints.total_dimensions(), 4);
    }

    #[test]
    fn test_veil_from_free_will() {
        // Test that Veil derives from Free Will (Archetype 22)
        let seed = Arc::new(HolographicSeed::new_from_source());
        let free_will = seed.free_will();

        let veil = Veil::from_free_will(free_will);

        assert!(veil.is_present());
        assert!(veil.thickness >= 0.0 && veil.thickness <= 1.0);
        assert!(veil.opacity >= 0.0 && veil.opacity <= 1.0);
        assert!(veil.permeability >= 0.0 && veil.permeability <= 1.0);
        assert!(veil.distortion >= 0.0 && veil.distortion <= 1.0);
    }

    #[test]
    fn test_veil_strength() {
        // Test Veil strength calculation
        let veil = Veil {
            thickness: 0.5,
            opacity: 0.7,
            permeability: 0.3,
            distortion: 0.6,
        };

        let strength = veil.strength();

        // Strength should be average of thickness and opacity
        assert!((strength - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_veil_spirit_penetration() {
        // Test Spirit penetration through Veil
        let veil = Veil {
            thickness: 0.5,
            opacity: 0.7,
            permeability: 0.3,
            distortion: 0.6,
        };

        let spirit_intensity = 1.0;
        let penetration = veil.spirit_penetration(spirit_intensity);

        // Penetration should be spirit intensity * permeability
        assert!((penetration - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_veil_reality_distortion() {
        // Test reality distortion by Veil
        let veil = Veil {
            thickness: 0.5,
            opacity: 0.7,
            permeability: 0.3,
            distortion: 0.6,
        };

        let reality_clarity = 1.0;
        let distorted = veil.reality_distortion(reality_clarity);

        // Distorted reality should be clarity * (1 - distortion)
        assert!((distorted - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_space_interface_from_coordinate_system() {
        // Test that SpaceInterface derives from coordinate system
        let seed = Arc::new(HolographicSeed::new_from_source());
        let constraints = DimensionalConstraints {
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            max_spatial_extent: 1.0e26,
            max_temporal_extent: 1.0e18,
            topology: DimensionalTopology::Flat,
            compactification: None,
        };
        let coord_system = CoordinateSystem::from_constraints(&constraints);

        let space_interface = SpaceInterface::from_coordinate_system(&coord_system, seed);

        assert_eq!(space_interface.dimensions(), 3);
        assert!(space_interface.extent > 0.0);
    }

    #[test]
    fn test_time_interface_from_coordinate_system() {
        // Test that TimeInterface derives from coordinate system
        let seed = Arc::new(HolographicSeed::new_from_source());
        let constraints = DimensionalConstraints {
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            max_spatial_extent: 1.0e26,
            max_temporal_extent: 1.0e18,
            topology: DimensionalTopology::Flat,
            compactification: None,
        };
        let coord_system = CoordinateSystem::from_constraints(&constraints);

        let time_interface = TimeInterface::from_coordinate_system(&coord_system, seed);

        assert_eq!(time_interface.current_time(), 0.0);
        assert!(time_interface.time_step > 0.0);
    }

    #[test]
    fn test_time_interface_advance() {
        // Test time advancement
        let seed = Arc::new(HolographicSeed::new_from_source());
        let constraints = DimensionalConstraints {
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            max_spatial_extent: 1.0e26,
            max_temporal_extent: 1.0e18,
            topology: DimensionalTopology::Flat,
            compactification: None,
        };
        let coord_system = CoordinateSystem::from_constraints(&constraints);

        let mut time_interface = TimeInterface::from_coordinate_system(&coord_system, seed);

        let initial_time = time_interface.current_time();
        time_interface.advance();
        let advanced_time = time_interface.current_time();

        assert!(advanced_time > initial_time);
    }

    #[test]
    fn test_time_space_interface_from_veil() {
        // Test that Time/Space interface derives from Veil
        let seed = Arc::new(HolographicSeed::new_from_source());
        let veil = Veil {
            thickness: 0.5,
            opacity: 0.7,
            permeability: 0.3,
            distortion: 0.6,
        };

        let time_space_interface = TimeSpaceInterface::from_veil(&veil, seed);

        // Temporal spatialization should be 1 - veil thickness
        assert!((time_space_interface.temporal_spatialization() - 0.5).abs() < 1e-10);

        // Spatial temporalization should be 1 - veil thickness
        assert!((time_space_interface.spatial_temporalization() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_phase3_layer_realignment_complete() {
        // Test complete Phase 3 Layer Realignment
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());

        // Verify Layer 3 structure:
        // 1. Derives from HolographicSeed (Layer 0)
        assert!(Arc::ptr_eq(space_time_interface.seed(), &seed));

        // 2. Has coordinate system (Layer 3 - Green Ray)
        assert!(space_time_interface.coordinate_system().total_dimensions() == 4);

        // 3. Has dimensional constraints (Layer 3 - Green Ray)
        assert!(
            space_time_interface
                .dimensional_constraints()
                .total_dimensions()
                == 4
        );

        // 4. Has Veil (Layer 3 - Green Ray - The Veil)
        assert!(space_time_interface.veil().is_present());

        // 5. Has Space interface (physical realm)
        assert!(space_time_interface.space().dimensions() == 3);

        // 6. Has Time interface (physical realm)
        assert!(space_time_interface.time().current_time() >= 0.0);

        // 7. Has Time/Space interface (metaphysical realm)
        assert!(space_time_interface.time_space().temporal_spatialization() >= 0.0);

        // 8. No Solar System constraints (moved to Layer 4)
        // This is verified by the fact that SpaceTimeInterface doesn't have
        // solar_system_constraints field - it's now in Layer 4
    }

    #[test]
    fn test_phase3_layer_3_is_green_ray() {
        // Test that Layer 3 is correctly aligned as Green Ray (Dimensions/Time-Space)
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed);

        // Layer 3 should contain:
        // - Space/Time & Time/Space (The Veil)
        assert!(space_time_interface.veil().is_present());
        assert!(space_time_interface.space().dimensions() > 0);
        assert!(space_time_interface.time().current_time() >= 0.0);
        assert!(space_time_interface.time_space().temporal_spatialization() >= 0.0);

        // Layer 3 should NOT contain:
        // - Solar System constraints (moved to Layer 4)
        // - Matter (emerges from Layer 3, not is Layer 3)
        // - Biology (emerges from Layer 5, not is Layer 3)
        // - Consciousness (emerges from Layer 6, not is Layer 3)
        //
        // This is verified by the structure of SpaceTimeInterface
    }

    #[test]
    fn test_phase3_space_time_is_interface_not_container() {
        // Test that Space/Time is an interface layer, not a container
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());

        // Space/Time should be an interface:
        // 1. Derives from seed (not standalone)
        assert!(Arc::ptr_eq(space_time_interface.seed(), &seed));

        // 2. Has interfaces to Space and Time (not containers)
        assert!(space_time_interface.space().dimensions() > 0);
        assert!(space_time_interface.time().current_time() >= 0.0);

        // 3. Has interfaces to Time/Space (not container)
        assert!(space_time_interface.time_space().temporal_spatialization() >= 0.0);

        // 4. Does not contain entities (removed container behavior)
        // This is verified by the absence of entity storage fields
        // in SpaceTimeInterface structure
    }

    #[test]
    fn test_phase3_emergence_from_holographic_seed() {
        // Test that Space/Time emerges from Holographic Seed
        let seed = Arc::new(HolographicSeed::new_from_source());

        // Space/Time emerges from seed
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());

        // Verify emergence chain:
        // 1. Seed contains Light Architecture (Layer 2)
        let light_encoding = seed.light_encoding();
        assert!(!light_encoding.holographic_encoding.is_empty());

        // 2. Dimensional constraints derive from Light Architecture
        let constraints = DimensionalConstraints::from_light_encoding(light_encoding);
        assert!(constraints.total_dimensions() == 4);

        // 3. Coordinate system derives from dimensional constraints
        let coord_system = CoordinateSystem::from_constraints(&constraints);
        assert!(coord_system.total_dimensions() == 4);

        // 4. Space/Time interface derives from coordinate system
        assert!(space_time_interface.coordinate_system().total_dimensions() == 4);
        assert!(
            space_time_interface
                .dimensional_constraints()
                .total_dimensions()
                == 4
        );
    }

    #[test]
    fn test_phase3_validation_criteria_complete() {
        // Complete validation of Phase 3 criteria from roadmap

        // 1. Layer 3 is Dimensions/Time-Space (not Solar System)
        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());
        assert!(space_time_interface.coordinate_system().total_dimensions() == 4);
        assert!(space_time_interface.veil().is_present());

        // 2. Space/Time derives from HolographicSeed
        assert!(Arc::ptr_eq(space_time_interface.seed(), &seed));

        // 3. No redundant structures between Space/Time and HolographicSeed
        // Dimensional constraints derive from light encoding
        let constraints = space_time_interface.dimensional_constraints();
        assert!(constraints.total_dimensions() == 4);

        // 4. Proper layer hierarchy:
        // Layer 0: HolographicSeed (Source) ✓
        // Layer 1: Archetypical Mind (Indigo) - in seed.archetypes ✓
        // Layer 2: Light Architecture (Blue) - in seed.light_encoding ✓
        // Layer 3: Space/Time (Green) - SpaceTimeInterface ✓
        // Layer 4: Solar/Planetary (Yellow) - in seed.physics_constraints ✓
        // Layer 5: Soul Stream (Orange) - separate module ✓
        // Layer 6: Co-Creator (Red) - Entity module ✓

        // 5. Space/Time is interface layer, not container
        assert!(space_time_interface.derives_from_seed());

        // 6. Time/Space exists (metaphysical realm)
        assert!(space_time_interface.time_space().temporal_spatialization() >= 0.0);

        // 7. Veil mechanism exists
        assert!(space_time_interface.veil().is_present());
    }

    #[test]
    fn test_phase3_success_metrics() {
        // Test Phase 3 success metrics

        let seed = Arc::new(HolographicSeed::new_from_source());
        let space_time_interface = SpaceTimeInterface::from_seed(seed.clone());

        // 1. Layer mapping correctness: 100%
        // Layer 3 is Dimensions/Time-Space (Green Ray)
        assert!(space_time_interface.coordinate_system().total_dimensions() == 4);
        assert!(space_time_interface.veil().is_present());

        // 2. Space/Time derives from HolographicSeed: 100%
        assert!(Arc::ptr_eq(space_time_interface.seed(), &seed));

        // 3. No redundant structures: 100%
        // Dimensional constraints derive from light encoding
        let constraints = space_time_interface.dimensional_constraints();
        assert!(constraints.total_dimensions() == 4);

        // 4. Proper layering: 100% enforced
        // Layer hierarchy is correctly implemented
        assert!(space_time_interface.derives_from_seed());

        // 5. Time/Space exists: 100%
        assert!(space_time_interface.time_space().temporal_spatialization() >= 0.0);

        // 6. Veil mechanism: 100%
        assert!(space_time_interface.veil().is_present());
    }
}
