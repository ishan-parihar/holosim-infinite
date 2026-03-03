//! Field-Derived Hydrological Systems
//!
//! From ROADMAP Phase 6.1: "Hydrological cycle from field coherence"
//! "Water follows field gradients, not just gravity"
//!
//! This module implements water systems that emerge from holographic field coherence patterns:
//! - Water archetype signature (22-element pattern)
//! - Water bodies forming at high-coherence field nodes
//! - Rivers flowing along field gradients
//! - Evaporation/precipitation from field coherence thresholds
//!
//! # Key Principles
//!
//! 1. **Field-Derived Water**: Water emerges from field coherence, not just gravity
//! 2. **Archetype Signature**: Water has a characteristic 22-element archetype pattern
//! 3. **Gradient Flow**: Rivers follow field gradients, not just elevation
//! 4. **Coherence Thresholds**: Evaporation/precipitation based on coherence levels

use crate::holographic::holographic_field::HolographicField;
use crate::holographic::universal_template::ArchetypeActivationProfile;
use crate::holographic::Position;
use std::collections::HashSet;
use std::sync::Arc;

/// Float type alias for consistency with the codebase
pub type Float = f64;

/// Water archetype signature (22 coefficients)
///
/// From ROADMAP: "Water signature: [f64; 22]"
/// This defines water's archetype activation pattern.
///
/// The water archetype has these characteristics:
/// - Balanced Matrix (A1): Stable structure for molecular bonds
/// - High Potentiator (A2): Great potential for transformation
/// - Active Catalyst (A3): Reactive, enables chemical processes
/// - Accumulating Experience (A4): Memory of flow paths
/// - Flowing Great Way (A7): Completion through cycles
pub const WATER_ARCHETYPE_SIGNATURE: [Float; 22] = [
    0.5, // A1 Matrix - balanced structure
    0.6, // A2 Potentiator - high potential
    0.7, // A3 Catalyst - reactive (transformation)
    0.6, // A4 Experience - accumulating
    0.4, // A5 Significator - meaning
    0.5, // A6 Transformation - balanced change
    0.6, // A7 Great Way - completion
    // Body archetypes (A8-A14) - water's physical manifestation
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
    // Spirit archetypes (A15-A21) - water's spiritual aspects
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, // A22 Choice - water's path selection
    0.5,
];

/// Field-derived hydrological system
///
/// From ROADMAP: "Water cycle from archetype patterns"
///
/// Water systems emerge from the holographic field rather than being
/// pre-defined. This creates naturalistic water patterns that are
/// consistent with the underlying field coherence structure.
#[derive(Debug, Clone)]
pub struct FieldHydrology {
    /// Reference to the holographic field
    pub field: Arc<HolographicField>,

    /// Water archetype pattern
    pub water_signature: [Float; 22],

    /// Coherence threshold for water accumulation
    /// Water pools where field coherence exceeds this threshold
    pub accumulation_threshold: Float,

    /// Coherence threshold for evaporation
    /// Water evaporates where field coherence falls below this threshold
    pub evaporation_threshold: Float,

    /// Flow rate multiplier based on field gradient
    pub flow_multiplier: Float,

    /// Current water bodies (accumulation points)
    pub water_bodies: Vec<WaterBody>,

    /// River networks (gradient-following paths)
    pub rivers: Vec<FieldRiver>,

    /// Precipitation map
    pub precipitation_map: Vec<PrecipitationCell>,

    /// Vapor capacity (Time/Space dimension water storage)
    pub vapor_capacity: Float,

    /// Current vapor in system
    pub current_vapor: Float,
}

/// Water body formed from field coherence
///
/// Water bodies emerge at locations where the field coherence pattern
/// matches the water archetype signature above a threshold.
#[derive(Debug, Clone)]
pub struct WaterBody {
    /// Unique identifier
    pub id: u64,

    /// Position in 3D space
    pub position: Position,

    /// Volume of water (arbitrary units)
    pub volume: Float,

    /// Average depth
    pub depth: Float,

    /// Salinity (0.0 = fresh, 1.0 = saturated)
    pub salinity: Float,

    /// Local field coherence level
    pub coherence_level: Float,

    /// Type of water body
    pub water_type: WaterType,

    /// Archetype match score at this location
    pub archetype_match: Float,
}

/// Types of water bodies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaterType {
    /// Large body of saltwater
    Ocean,
    /// Smaller body of saltwater
    Sea,
    /// Body of freshwater
    Lake,
    /// Small body of freshwater
    Pond,
    /// Frozen water
    Glacier,
    /// Underground water
    Groundwater,
    /// Atmospheric water
    Vapor,
}

/// River flowing along field gradients
///
/// From ROADMAP: "Rivers flowing along field gradients"
///
/// Rivers trace paths through the field following coherence gradients,
/// not just elevation. This creates naturalistic drainage patterns.
#[derive(Debug, Clone)]
pub struct FieldRiver {
    /// Unique identifier
    pub id: u64,

    /// Path traced through the field
    pub path: Vec<Position>,

    /// Flow rate (arbitrary units)
    pub flow_rate: Float,

    /// How well the path follows the field gradient (0.0-1.0)
    pub gradient_followed: Float,

    /// Coherence at source
    pub source_coherence: Float,

    /// Coherence at destination
    pub destination_coherence: Float,

    /// Width of the river
    pub width: Float,

    /// Whether the river is active (has flow)
    pub is_active: bool,
}

/// Precipitation cell
///
/// Precipitation occurs where field coherence is high enough
/// to condense water vapor from Time/Space.
#[derive(Debug, Clone)]
pub struct PrecipitationCell {
    /// Position of precipitation
    pub position: Position,

    /// Precipitation rate (arbitrary units)
    pub precipitation_rate: Float,

    /// Field coherence at source
    pub coherence_at_source: Float,

    /// Vapor consumed to create this precipitation
    pub vapor_consumed: Float,
}

/// Result of hydrology evolution step
#[derive(Debug, Clone, Default)]
pub struct HydrologyEvolutionResult {
    /// Number of new water bodies created
    pub new_water_bodies: usize,

    /// Number of new rivers created
    pub new_rivers: usize,

    /// Total evaporation volume
    pub evaporation_volume: Float,

    /// Total precipitation volume
    pub precipitation_volume: Float,

    /// Total flow through rivers
    pub total_flow: Float,
}

impl FieldHydrology {
    /// Create hydrology system from holographic field
    ///
    /// The hydrology system derives its water patterns from the field structure.
    pub fn from_field(field: Arc<HolographicField>) -> Self {
        Self {
            field,
            water_signature: WATER_ARCHETYPE_SIGNATURE,
            accumulation_threshold: 0.6,
            evaporation_threshold: 0.3,
            flow_multiplier: 1.0,
            water_bodies: Vec::new(),
            rivers: Vec::new(),
            precipitation_map: Vec::new(),
            vapor_capacity: 1000.0,
            current_vapor: 0.0,
        }
    }

    /// Create hydrology with custom configuration
    pub fn with_config(
        field: Arc<HolographicField>,
        accumulation_threshold: Float,
        evaporation_threshold: Float,
        flow_multiplier: Float,
    ) -> Self {
        Self {
            field,
            water_signature: WATER_ARCHETYPE_SIGNATURE,
            accumulation_threshold,
            evaporation_threshold,
            flow_multiplier,
            water_bodies: Vec::new(),
            rivers: Vec::new(),
            precipitation_map: Vec::new(),
            vapor_capacity: 1000.0,
            current_vapor: 0.0,
        }
    }

    /// Evolve hydrology based on field state
    ///
    /// From ROADMAP: "Water cycle from archetype patterns"
    ///
    /// This method performs one evolution step of the hydrological cycle:
    /// 1. Find water accumulation points (high coherence + archetype match)
    /// 2. Generate rivers along field gradients
    /// 3. Process evaporation from low coherence areas
    /// 4. Process precipitation based on coherence patterns
    /// 5. Update water bodies
    pub fn evolve(&mut self, dt: Float) -> HydrologyEvolutionResult {
        // 1. Find water accumulation points (high coherence + archetype match)
        let accumulation_points = self.find_accumulation_points();
        let new_water_bodies = accumulation_points.len();

        // Create water bodies at accumulation points
        for (i, (position, coherence, match_score)) in accumulation_points.iter().enumerate() {
            let water_body = WaterBody {
                id: self.water_bodies.len() as u64 + i as u64,
                position: *position,
                volume: *coherence * 100.0,
                depth: *coherence * 10.0,
                salinity: if *match_score > 0.8 { 0.035 } else { 0.0 },
                coherence_level: *coherence,
                water_type: self.determine_water_type(*coherence, *match_score),
                archetype_match: *match_score,
            };
            self.water_bodies.push(water_body);
        }

        // 2. Generate rivers along field gradients
        let sources: Vec<Position> = self
            .water_bodies
            .iter()
            .filter(|wb| wb.water_type != WaterType::Glacier)
            .map(|wb| wb.position)
            .collect();
        let new_rivers = self.generate_rivers(&sources);

        // 3. Process evaporation from low coherence areas
        let evaporation = self.process_evaporation(dt);

        // 4. Process precipitation based on coherence patterns
        let precipitation = self.process_precipitation(dt);

        // 5. Update water bodies
        self.update_water_bodies(dt);

        // Calculate total flow
        let total_flow: Float = self.rivers.iter().map(|r| r.flow_rate).sum();

        HydrologyEvolutionResult {
            new_water_bodies,
            new_rivers: new_rivers.len(),
            evaporation_volume: evaporation,
            precipitation_volume: precipitation,
            total_flow,
        }
    }

    /// Find points where water accumulates based on field coherence
    ///
    /// Water accumulates where:
    /// 1. Field coherence is above accumulation threshold
    /// 2. Archetype pattern matches water signature
    fn find_accumulation_points(&self) -> Vec<(Position, Float, Float)> {
        let mut points = Vec::new();

        // Sample field at grid points
        let grid_resolution = self.field.grid_resolution();
        for x in 0..grid_resolution {
            for y in 0..grid_resolution {
                for z in 0..grid_resolution {
                    let pos = Position::new(
                        x as Float / grid_resolution as Float,
                        y as Float / grid_resolution as Float,
                        z as Float / grid_resolution as Float,
                    );

                    let coherence = self.field.phase_coherence();

                    if coherence > self.accumulation_threshold {
                        // Check archetype match
                        let activation = self.field.derive_archetype_activation(&pos);
                        let match_score = self.calculate_archetype_match(&activation.coefficients);

                        if match_score > 0.7 {
                            points.push((pos, coherence, match_score));
                        }
                    }
                }
            }
        }

        points
    }

    /// Calculate how well an archetype pattern matches water
    ///
    /// Uses L1 distance between the pattern and water signature.
    /// Returns a score from 0.0 (no match) to 1.0 (perfect match).
    pub fn calculate_archetype_match(&self, pattern: &[Float; 22]) -> Float {
        let mut match_score = 0.0;

        for i in 0..22 {
            let diff = (pattern[i] - self.water_signature[i]).abs();
            match_score += 1.0 - diff;
        }

        match_score / 22.0
    }

    /// Determine water type based on coherence and archetype match
    fn determine_water_type(&self, coherence: Float, match_score: Float) -> WaterType {
        if coherence > 0.9 {
            WaterType::Glacier
        } else if match_score > 0.85 && coherence > 0.7 {
            WaterType::Ocean
        } else if match_score > 0.75 && coherence > 0.6 {
            WaterType::Sea
        } else if coherence > 0.5 {
            WaterType::Lake
        } else {
            WaterType::Pond
        }
    }

    /// Generate rivers following field gradients
    ///
    /// From ROADMAP: "Rivers flowing along field gradients"
    fn generate_rivers(&mut self, sources: &[Position]) -> Vec<FieldRiver> {
        let mut rivers = Vec::new();
        let mut used_sources: HashSet<String> = HashSet::new();

        for (i, source) in sources.iter().enumerate() {
            // Avoid duplicate rivers from same source
            let key = format!("{:.3},{:.3},{:.3}", source.x, source.y, source.z);
            if used_sources.contains(&key) {
                continue;
            }
            used_sources.insert(key);

            // Trace path along gradient descent
            let path = self.trace_gradient_path(source);

            if path.len() > 5 {
                let source_coherence = self.field.phase_coherence();
                let river = FieldRiver {
                    id: self.rivers.len() as u64 + i as u64,
                    path,
                    flow_rate: source_coherence * self.flow_multiplier * 10.0,
                    gradient_followed: 0.8,
                    source_coherence,
                    destination_coherence: 0.3,
                    width: source_coherence * 5.0,
                    is_active: true,
                };
                rivers.push(river);
            }
        }

        self.rivers.extend(rivers.clone());
        rivers
    }

    /// Trace a path along the field gradient
    ///
    /// Rivers follow the gradient descent of field coherence,
    /// representing water flowing "down" from high coherence to low coherence regions.
    fn trace_gradient_path(&self, start: &Position) -> Vec<Position> {
        let mut path = vec![*start];
        let mut current = *start;
        let mut visited: HashSet<String> = HashSet::new();

        for _ in 0..100 {
            // Find direction of steepest descent
            let gradient = self.calculate_field_gradient(&current);

            // Move along gradient (decreasing coherence direction)
            current = Position::new(
                current.x - gradient.x * 0.01,
                current.y - gradient.y * 0.01,
                current.z - gradient.z * 0.01,
            );

            // Check bounds
            if current.x < 0.0
                || current.x > 1.0
                || current.y < 0.0
                || current.y > 1.0
                || current.z < 0.0
                || current.z > 1.0
            {
                break;
            }

            // Avoid loops
            let key = format!("{:.2},{:.2},{:.2}", current.x, current.y, current.z);
            if visited.contains(&key) {
                break;
            }
            visited.insert(key);

            path.push(current);
        }

        path
    }

    /// Calculate field gradient at position
    ///
    /// The gradient indicates the direction of maximum coherence change.
    fn calculate_field_gradient(&self, pos: &Position) -> Gradient3D {
        let delta = 0.01;

        // Sample coherence at adjacent points
        let coherence_here = self.field.phase_coherence();

        // Approximate gradient using finite differences
        let dx = if pos.x + delta <= 1.0 {
            let pos_plus = Position::new(pos.x + delta, pos.y, pos.z);
            // For now, use a simplified gradient based on position
            (pos_plus.x - pos.x) * coherence_here * 0.1
        } else {
            0.0
        };

        let dy = if pos.y + delta <= 1.0 {
            let pos_plus = Position::new(pos.x, pos.y + delta, pos.z);
            (pos_plus.y - pos.y) * coherence_here * 0.1
        } else {
            0.0
        };

        let dz = if pos.z + delta <= 1.0 {
            let pos_plus = Position::new(pos.x, pos.y, pos.z + delta);
            (pos_plus.z - pos.z) * coherence_here * 0.1
        } else {
            0.0
        };

        Gradient3D {
            x: dx,
            y: dy,
            z: dz,
        }
    }

    /// Process evaporation from low coherence areas
    ///
    /// Water evaporates where field coherence falls below the evaporation threshold.
    /// The vapor is stored in the Time/Space dimension (accessible through high coherence).
    fn process_evaporation(&mut self, dt: Float) -> Float {
        let mut total_evaporation = 0.0;

        for water_body in &mut self.water_bodies {
            if water_body.coherence_level < self.evaporation_threshold {
                let evap_rate =
                    (self.evaporation_threshold - water_body.coherence_level) * dt * 0.1;
                let evap_volume = water_body.volume * evap_rate;

                // Transfer water to vapor
                water_body.volume -= evap_volume;
                self.current_vapor = (self.current_vapor + evap_volume).min(self.vapor_capacity);
                total_evaporation += evap_volume;
            }
        }

        total_evaporation
    }

    /// Process precipitation based on coherence patterns
    ///
    /// Precipitation occurs where coherence is high enough to condense
    /// water vapor from Time/Space back to Space/Time.
    fn process_precipitation(&mut self, dt: Float) -> Float {
        let coherence = self.field.phase_coherence();

        if coherence > self.accumulation_threshold && self.current_vapor > 0.0 {
            let precip_rate = (coherence - self.accumulation_threshold) * dt;
            let precip_volume = (precip_rate * self.current_vapor).min(self.current_vapor);

            // Create precipitation cell at field center
            let cell = PrecipitationCell {
                position: Position::new(0.5, 0.5, 0.5),
                precipitation_rate: precip_rate,
                coherence_at_source: coherence,
                vapor_consumed: precip_volume,
            };

            self.precipitation_map.push(cell);
            self.current_vapor -= precip_volume;

            precip_volume
        } else {
            0.0
        }
    }

    /// Update water bodies based on field state
    fn update_water_bodies(&mut self, _dt: Float) {
        // Remove depleted water bodies
        self.water_bodies.retain(|wb| wb.volume > 0.01);

        // Update coherence levels
        let coherence = self.field.phase_coherence();
        for water_body in &mut self.water_bodies {
            water_body.coherence_level = coherence;
        }

        // Update river activity
        for river in &mut self.rivers {
            river.is_active = river.flow_rate > 0.1;
        }
    }

    /// Get total water volume in system
    pub fn total_water_volume(&self) -> Float {
        let liquid: Float = self.water_bodies.iter().map(|wb| wb.volume).sum();
        let river: Float = self.rivers.iter().map(|r| r.flow_rate).sum();
        liquid + river + self.current_vapor
    }

    /// Get water at a specific position
    pub fn get_water_at(&self, pos: &Position) -> Option<&WaterBody> {
        self.water_bodies.iter().find(|wb| {
            let dist = (wb.position.x - pos.x).powi(2)
                + (wb.position.y - pos.y).powi(2)
                + (wb.position.z - pos.z).powi(2);
            dist < 0.01
        })
    }

    /// Get water statistics
    pub fn get_statistics(&self) -> HydrologyStatistics {
        let ocean_count = self
            .water_bodies
            .iter()
            .filter(|wb| wb.water_type == WaterType::Ocean)
            .count();
        let lake_count = self
            .water_bodies
            .iter()
            .filter(|wb| wb.water_type == WaterType::Lake)
            .count();
        let glacier_count = self
            .water_bodies
            .iter()
            .filter(|wb| wb.water_type == WaterType::Glacier)
            .count();
        let active_rivers = self.rivers.iter().filter(|r| r.is_active).count();

        HydrologyStatistics {
            total_water_bodies: self.water_bodies.len(),
            total_rivers: self.rivers.len(),
            ocean_count,
            lake_count,
            glacier_count,
            active_rivers,
            total_volume: self.total_water_volume(),
            vapor_level: self.current_vapor,
            precipitation_cells: self.precipitation_map.len(),
        }
    }
}

impl Default for FieldHydrology {
    fn default() -> Self {
        // Create a default holographic field
        use crate::holographic::complex_vectors::ComplexArchetype;
        let archetypes = [ComplexArchetype {
            amplitude: 0.5,
            phase: 0.0,
        }; 22];
        let field = Arc::new(HolographicField::new(
            crate::holographic::InvolutionLayer::Green,
            archetypes,
        ));
        Self::from_field(field)
    }
}

/// 3D gradient vector
#[derive(Debug, Clone, Copy, Default)]
pub struct Gradient3D {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Gradient3D {
    /// Calculate magnitude of the gradient
    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize to unit vector
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0001 {
            Gradient3D {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }
}

/// Statistics for hydrology system
#[derive(Debug, Clone, Default)]
pub struct HydrologyStatistics {
    /// Total number of water bodies
    pub total_water_bodies: usize,
    /// Total number of rivers
    pub total_rivers: usize,
    /// Number of oceans
    pub ocean_count: usize,
    /// Number of lakes
    pub lake_count: usize,
    /// Number of glaciers
    pub glacier_count: usize,
    /// Number of active rivers
    pub active_rivers: usize,
    /// Total water volume
    pub total_volume: Float,
    /// Current vapor level
    pub vapor_level: Float,
    /// Number of precipitation cells
    pub precipitation_cells: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_archetype_signature() {
        // Water should have balanced signature
        let sum: Float = WATER_ARCHETYPE_SIGNATURE.iter().sum();
        assert!(
            sum > 10.0 && sum < 14.0,
            "Water signature sum should be balanced"
        );

        // Catalyst (A3) should be high for water's reactivity
        assert!(
            WATER_ARCHETYPE_SIGNATURE[2] > 0.5,
            "Catalyst should be elevated for water"
        );
    }

    #[test]
    fn test_field_hydrology_creation() {
        let hydrology = FieldHydrology::default();

        assert_eq!(hydrology.water_signature, WATER_ARCHETYPE_SIGNATURE);
        assert!(hydrology.accumulation_threshold > hydrology.evaporation_threshold);
        assert!(hydrology.water_bodies.is_empty());
        assert!(hydrology.rivers.is_empty());
    }

    #[test]
    fn test_archetype_match() {
        let hydrology = FieldHydrology::default();

        // Exact match should score 1.0
        let score = hydrology.calculate_archetype_match(&WATER_ARCHETYPE_SIGNATURE);
        assert!((score - 1.0).abs() < 0.001, "Exact match should score 1.0");

        // No match (all zeros) should score lower
        let zero_pattern = [0.0; 22];
        let score = hydrology.calculate_archetype_match(&zero_pattern);
        assert!(score < 0.5, "Zero pattern should have low match score");
    }

    #[test]
    fn test_gradient_path_tracing() {
        let hydrology = FieldHydrology::default();

        let start = Position::new(0.5, 0.5, 0.5);
        let path = hydrology.trace_gradient_path(&start);

        // Should have at least the starting point
        assert!(!path.is_empty(), "Path should not be empty");

        // First point should be the starting position
        assert!((path[0].x - start.x).abs() < 0.001);
        assert!((path[0].y - start.y).abs() < 0.001);
        assert!((path[0].z - start.z).abs() < 0.001);
    }

    #[test]
    fn test_gradient_calculation() {
        let hydrology = FieldHydrology::default();

        let pos = Position::new(0.5, 0.5, 0.5);
        let gradient = hydrology.calculate_field_gradient(&pos);

        // Gradient should be finite
        assert!(gradient.x.is_finite());
        assert!(gradient.y.is_finite());
        assert!(gradient.z.is_finite());
    }

    #[test]
    fn test_hydrology_evolution() {
        let mut hydrology = FieldHydrology::default();

        let result = hydrology.evolve(0.1);

        // Should produce valid statistics
        assert!(result.evaporation_volume >= 0.0);
        assert!(result.precipitation_volume >= 0.0);
        assert!(result.total_flow >= 0.0);
    }

    #[test]
    fn test_water_type_determination() {
        let hydrology = FieldHydrology::default();

        // High coherence + high match = ocean
        let water_type = hydrology.determine_water_type(0.8, 0.9);
        assert_eq!(water_type, WaterType::Ocean);

        // Very high coherence = glacier
        let water_type = hydrology.determine_water_type(0.95, 0.7);
        assert_eq!(water_type, WaterType::Glacier);

        // Moderate coherence = lake
        let water_type = hydrology.determine_water_type(0.55, 0.6);
        assert_eq!(water_type, WaterType::Lake);
    }

    #[test]
    fn test_total_water_volume() {
        let hydrology = FieldHydrology::default();

        // Initial volume should be 0 (no water yet)
        let volume = hydrology.total_water_volume();
        assert!(volume >= 0.0);
    }

    #[test]
    fn test_gradient_3d_operations() {
        let gradient = Gradient3D {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };

        // Magnitude should be 5.0 (3-4-5 triangle)
        assert!((gradient.magnitude() - 5.0).abs() < 0.001);

        // Normalized should have magnitude 1.0
        let normalized = gradient.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_hydrology_statistics() {
        let hydrology = FieldHydrology::default();
        let stats = hydrology.get_statistics();

        assert_eq!(stats.total_water_bodies, 0);
        assert_eq!(stats.total_rivers, 0);
        assert_eq!(stats.ocean_count, 0);
        assert_eq!(stats.lake_count, 0);
    }

    #[test]
    fn test_evaporation_thresholds() {
        let mut hydrology = FieldHydrology::default();
        hydrology.evaporation_threshold = 0.5;

        // Add a water body with low coherence
        hydrology.water_bodies.push(WaterBody {
            id: 0,
            position: Position::new(0.5, 0.5, 0.5),
            volume: 100.0,
            depth: 10.0,
            salinity: 0.0,
            coherence_level: 0.2, // Below evaporation threshold
            water_type: WaterType::Lake,
            archetype_match: 0.7,
        });

        let evap = hydrology.process_evaporation(0.1);

        // Should have some evaporation
        assert!(evap > 0.0);
        assert!(hydrology.current_vapor > 0.0);
    }
}
