//! Sacred Geometry Module
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 (Weeks 17-20):
//! - Fibonacci sequence in growth patterns
//! - Golden ratio (φ) in proportions
//! - Platonic solid structures
//! - Self-similar patterns at all scales
//! - Spiral patterns in galaxy formation
//! - Vesica Piscis in entity relationships
//! - Flower of Life in layer interactions
//! - Frequency-based resonance (not just spectrum ratios)
//! - Musical harmonic series
//! - Overtone patterns
//! - Standing wave formations
//!
//! This module implements the mathematical patterns that underlie consciousness
//! and creation in the cosmological architecture. Sacred geometry provides the
//! true harmonics (not statistical averages) that guide entity evolution and
//! cosmological structure formation.
//!
//! Key Concepts:
//! - Fibonacci Growth: Entity populations follow Fibonacci sequences
//! - Golden Ratio (φ): Proportions between layers and structures use φ = (1+√5)/2 ≈ 1.618
//! - Platonic Solids: Collective structures form Platonic solid geometries
//! - Spirals: Galaxy formation and spectrum trajectories follow φ spirals
//! - Vesica Piscis: Entity resonance creates Vesica Piscis patterns
//! - Flower of Life: Layer interactions create Flower of Life patterns
//! - Harmonic Resonance: Entities resonate when frequencies align (NOT statistical averaging)
//! - Musical Harmonics: Overtones at integer multiples (2f, 3f, 4f, ...)
//! - Standing Waves: Interference patterns when frequencies align

pub mod fibonacci;
pub mod flower_of_life;
pub mod golden_ratio;
pub mod harmonic_resonance;
pub mod platonic_solids;
pub mod spirals;
pub mod vesica_piscis;

pub use fibonacci::{
    fibonacci_number, fibonacci_sequence, FibonacciGrowth, FibonacciSequence, FibonacciSpiral,
};
pub use flower_of_life::{
    flower_of_life_circles, flower_of_life_pattern, FlowerLayer, FlowerOfLife, FlowerPattern,
};
pub use golden_ratio::{
    GoldenAngle, GoldenProportion, GoldenRectangle, GOLDEN_RATIO, GOLDEN_RATIO_INVERSE,
};
pub use harmonic_resonance::{
    FrequencyResonance, HarmonicSeries, InterferenceType, MusicalNote, OvertonePattern,
    StandingWave,
};
pub use platonic_solids::{
    PlatonicSolid, PlatonicSolidProportions, PlatonicStructure, PlatonicVertex,
};
pub use spirals::{
    spiral_point, ArchimedeanSpiral, FibonacciSpiral as FibSpiral, GoldenSpiral, LogarithmicSpiral,
    SpiralPattern, SpiralType,
};
pub use vesica_piscis::{
    vesica_piscis_intersection, EntityVesicaPiscis, VesicaPiscis, VesicaPiscisPattern,
};

use crate::types::Float;

/// Sacred Geometry Constants
///
/// Fundamental constants used throughout sacred geometry calculations.
pub mod constants {
    use crate::types::Float;

    /// Golden Ratio: φ = (1+√5)/2 ≈ 1.6180339887498948482
    pub const GOLDEN_RATIO: Float = 1.6180339887498948482;

    /// Golden Ratio Inverse: 1/φ = φ - 1 ≈ 0.6180339887498948482
    pub const GOLDEN_RATIO_INVERSE: Float = 0.6180339887498948482;

    /// Golden Angle: 360°/φ² ≈ 137.5077640500378546463° (in radians: ≈ 2.399963)
    pub const GOLDEN_ANGLE: Float = 2.39996322972865332;

    /// Phi squared: φ² = φ + 1 ≈ 2.6180339887498948482
    pub const GOLDEN_RATIO_SQUARED: Float = 2.6180339887498948482;

    /// Phi cubed: φ³ = 2φ + 1 ≈ 4.2360679774997896964
    pub const GOLDEN_RATIO_CUBED: Float = 4.2360679774997896964;

    /// √5 used in golden ratio calculations
    pub const SQRT_5: Float = 2.2360679774997896964;

    /// π for geometric calculations
    pub const PI: Float = std::f64::consts::PI;
}

/// Sacred Geometry System
///
/// Main container for all sacred geometry calculations and patterns.
#[derive(Debug, Clone)]
pub struct SacredGeometrySystem {
    /// Fibonacci sequence up to generation 50
    fibonacci: FibonacciSequence,

    /// Golden ratio calculations
    golden_ratio: GoldenProportion,

    /// Platonic solid structures
    platonic_solids: Vec<PlatonicStructure>,

    /// Active spiral patterns
    spirals: Vec<SpiralPattern>,

    /// Harmonic resonance calculations
    harmonic_resonance: HarmonicResonance,
}

/// Harmonic Resonance System
///
/// Container for harmonic resonance calculations including musical harmonics,
/// overtone patterns, and standing wave formations.
#[derive(Debug, Clone)]
pub struct HarmonicResonance {
    /// Base frequency scale (Hz)
    base_frequency_scale: Float,
    /// Maximum overtones to consider
    max_overtones: usize,
    /// Harmonic tolerance for matching
    harmonic_tolerance: Float,
}

impl HarmonicResonance {
    /// Create a new harmonic resonance system
    pub fn new() -> Self {
        HarmonicResonance {
            base_frequency_scale: 432.0, // A=432Hz (sacred frequency)
            max_overtones: 12,
            harmonic_tolerance: 0.01,
        }
    }

    /// Calculate resonance between two entity frequencies
    pub fn calculate_entity_resonance(
        &self,
        freq_a: Float,
        freq_b: Float,
        coherence_a: Float,
        coherence_b: Float,
    ) -> FrequencyResonance {
        FrequencyResonance::calculate_from_entities(freq_a, freq_b, coherence_a, coherence_b)
    }

    /// Create a harmonic series from entity frequency
    pub fn create_harmonic_series(&self, entity_frequency: Float) -> HarmonicSeries {
        HarmonicSeries::from_entity_frequency(entity_frequency)
    }

    /// Create a musical note from entity frequency
    pub fn create_musical_note(&self, entity_frequency: Float) -> MusicalNote {
        MusicalNote::from_entity_frequency(entity_frequency)
    }

    /// Create an overtone pattern for an archetype
    pub fn create_overtone_pattern(&self, archetype: usize) -> OvertonePattern {
        OvertonePattern::archetype(archetype)
    }

    /// Calculate standing wave between two frequencies
    pub fn calculate_standing_wave(&self, freq_a: Float, freq_b: Float) -> StandingWave {
        StandingWave::calculate(freq_a, freq_b)
    }
}

impl Default for HarmonicResonance {
    fn default() -> Self {
        Self::new()
    }
}

impl SacredGeometrySystem {
    /// Create a new sacred geometry system with default configurations
    pub fn new() -> Self {
        SacredGeometrySystem {
            fibonacci: FibonacciSequence::new(50),
            golden_ratio: GoldenProportion::default(),
            platonic_solids: Self::initialize_platonic_solids(),
            spirals: Vec::new(),
            harmonic_resonance: HarmonicResonance::new(),
        }
    }

    /// Initialize all five Platonic solids
    fn initialize_platonic_solids() -> Vec<PlatonicStructure> {
        vec![
            PlatonicStructure::tetrahedron(1.0),
            PlatonicStructure::cube(1.0),
            PlatonicStructure::octahedron(1.0),
            PlatonicStructure::dodecahedron(1.0),
            PlatonicStructure::icosahedron(1.0),
        ]
    }

    /// Get Fibonacci number at specified generation
    pub fn fibonacci_number(&self, generation: u64) -> u64 {
        self.fibonacci.get(generation)
    }

    /// Calculate Fibonacci growth at specified generation
    pub fn fibonacci_growth(&self, generation: u64) -> Float {
        let fib = self.fibonacci_number(generation);
        fib as Float * constants::GOLDEN_RATIO.powi(2 - generation as i32)
    }

    /// Get golden ratio value
    pub fn golden_ratio(&self) -> Float {
        constants::GOLDEN_RATIO
    }

    /// Check if two values are in golden ratio proportion
    pub fn is_golden_proportion(&self, a: Float, b: Float, tolerance: Float) -> bool {
        let ratio = a.max(b) / a.min(b);
        (ratio - constants::GOLDEN_RATIO).abs() < tolerance
    }

    /// Get Platonic solid by type
    pub fn get_platonic_solid(&self, solid_type: PlatonicSolid) -> Option<&PlatonicStructure> {
        self.platonic_solids
            .iter()
            .find(|s| s.solid_type() == solid_type)
    }

    /// Create a golden spiral
    pub fn create_golden_spiral(&mut self, center: (Float, Float), turns: Float) -> SpiralPattern {
        let spiral = SpiralPattern::golden_spiral(center.0, center.1, turns);
        self.spirals.push(spiral.clone());
        spiral
    }

    /// Calculate Vesica Piscis intersection for two entities
    pub fn vesica_piscis(
        &self,
        center_a: (Float, Float),
        center_b: (Float, Float),
        radius: Float,
    ) -> VesicaPiscis {
        VesicaPiscis::new(center_a, center_b, radius)
    }

    /// Create Flower of Life pattern
    pub fn flower_of_life(
        &self,
        center: (Float, Float),
        radius: Float,
        layers: usize,
    ) -> FlowerOfLife {
        FlowerOfLife::new(center.0, center.1, radius, layers)
    }

    /// Calculate harmonic resonance between two entities
    pub fn calculate_harmonic_resonance(
        &self,
        freq_a: Float,
        freq_b: Float,
        coherence_a: Float,
        coherence_b: Float,
    ) -> FrequencyResonance {
        self.harmonic_resonance
            .calculate_entity_resonance(freq_a, freq_b, coherence_a, coherence_b)
    }

    /// Get the harmonic resonance subsystem
    pub fn harmonic_resonance(&self) -> &HarmonicResonance {
        &self.harmonic_resonance
    }

    // ========================================================================
    // Phase 8.2: Active Intelligence Integration Methods
    // ========================================================================

    /// Scale a value by φ^level (golden ratio scaling)
    ///
    /// From ROADMAP Phase 8.2:
    /// "Golden ratio scaling throughout"
    ///
    /// This method applies the golden ratio (φ ≈ 1.618) raised to the given level
    /// to scale values according to sacred geometry principles.
    ///
    /// # Arguments
    ///
    /// * `value` - The base value to scale
    /// * `level` - The power to raise φ to (can be negative for shrinking)
    ///
    /// # Returns
    ///
    /// value × φ^level
    ///
    /// # Examples
    ///
    /// ```
    /// # use holonic_realms::sacred_geometry::SacredGeometrySystem;
    /// let sg = SacredGeometrySystem::new();
    /// let scaled = sg.phi_scale(1.0, 1);  // ≈ 1.618
    /// let scaled2 = sg.phi_scale(1.0, 2); // ≈ 2.618
    /// let shrunk = sg.phi_scale(1.0, -1); // ≈ 0.618
    /// ```
    pub fn phi_scale(&self, value: Float, level: i32) -> Float {
        value * constants::GOLDEN_RATIO.powi(level)
    }

    /// Calculate a point on a Fibonacci spiral at the given index
    ///
    /// From ROADMAP Phase 8.2:
    /// "Fibonacci spiral patterns"
    ///
    /// The Fibonacci spiral uses quarter circles with radii following the
    /// Fibonacci sequence: 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
    ///
    /// # Arguments
    ///
    /// * `index` - Index along the spiral (0, 1, 2, ...)
    ///
    /// # Returns
    ///
    /// (x, y) coordinates of the spiral point
    ///
    /// # Examples
    ///
    /// ```
    /// # use holonic_realms::sacred_geometry::SacredGeometrySystem;
    /// let sg = SacredGeometrySystem::new();
    /// let (x, y) = sg.fibonacci_spiral_point(5);
    /// ```
    pub fn fibonacci_spiral_point(&self, index: usize) -> (Float, Float) {
        // Get Fibonacci number at this index for radius scaling
        let fib = self.fibonacci_number(index as u64) as Float;

        // Golden angle determines the angle increment
        let angle = index as Float * constants::GOLDEN_ANGLE;

        // Scale radius by Fibonacci number
        let radius = fib * 0.1;

        // Convert polar to Cartesian
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        (x, y)
    }

    /// Generate vertices for a Platonic solid
    ///
    /// From ROADMAP Phase 8.2:
    /// "Platonic solid templates"
    ///
    /// Returns the vertices for one of the five Platonic solids scaled
    /// to the specified size.
    ///
    /// # Arguments
    ///
    /// * `solid_type` - Which Platonic solid to generate
    /// * `scale` - Scale factor for the solid
    ///
    /// # Returns
    ///
    /// Vector of 3D vertices (as tuples for easy use)
    ///
    /// # Examples
    ///
    /// ```
    /// # use holonic_realms::sacred_geometry::{SacredGeometrySystem, PlatonicSolid};
    /// let sg = SacredGeometrySystem::new();
    /// let vertices = sg.platonic_solid(PlatonicSolid::Tetrahedron, 1.0);
    /// assert_eq!(vertices.len(), 4);
    /// ```
    pub fn platonic_solid(
        &self,
        solid_type: PlatonicSolid,
        scale: Float,
    ) -> Vec<(Float, Float, Float)> {
        match solid_type {
            PlatonicSolid::Tetrahedron => vec![
                (scale, scale, scale),
                (-scale, -scale, scale),
                (-scale, scale, -scale),
                (scale, -scale, -scale),
            ],
            PlatonicSolid::Cube => vec![
                (-scale, -scale, -scale),
                (scale, -scale, -scale),
                (scale, scale, -scale),
                (-scale, scale, -scale),
                (-scale, -scale, scale),
                (scale, -scale, scale),
                (scale, scale, scale),
                (-scale, scale, scale),
            ],
            PlatonicSolid::Octahedron => vec![
                (0.0, 0.0, scale),
                (0.0, 0.0, -scale),
                (scale, 0.0, 0.0),
                (-scale, 0.0, 0.0),
                (0.0, scale, 0.0),
                (0.0, -scale, 0.0),
            ],
            PlatonicSolid::Dodecahedron => {
                // Dodecahedron vertices involve the golden ratio
                let phi = constants::GOLDEN_RATIO;
                let inv_phi = constants::GOLDEN_RATIO_INVERSE;

                vec![
                    // Cube vertices scaled by inv_phi
                    (1.0, 1.0, 1.0),
                    (1.0, 1.0, -1.0),
                    (1.0, -1.0, 1.0),
                    (1.0, -1.0, -1.0),
                    (-1.0, 1.0, 1.0),
                    (-1.0, 1.0, -1.0),
                    (-1.0, -1.0, 1.0),
                    (-1.0, -1.0, -1.0),
                    // Additional vertices from golden ratio
                    (0.0, inv_phi, phi),
                    (0.0, inv_phi, -phi),
                    (0.0, -inv_phi, phi),
                    (0.0, -inv_phi, -phi),
                    (inv_phi, phi, 0.0),
                    (inv_phi, -phi, 0.0),
                    (-inv_phi, phi, 0.0),
                    (-inv_phi, -phi, 0.0),
                    (phi, 0.0, inv_phi),
                    (phi, 0.0, -inv_phi),
                    (-phi, 0.0, inv_phi),
                    (-phi, 0.0, -inv_phi),
                ]
                .iter()
                .map(|&(x, y, z)| (x * scale, y * scale, z * scale))
                .collect()
            }
            PlatonicSolid::Icosahedron => {
                // Icosahedron vertices involve the golden ratio
                let phi = constants::GOLDEN_RATIO;

                vec![
                    (0.0, 1.0, phi),
                    (0.0, 1.0, -phi),
                    (0.0, -1.0, phi),
                    (0.0, -1.0, -phi),
                    (1.0, phi, 0.0),
                    (1.0, -phi, 0.0),
                    (-1.0, phi, 0.0),
                    (-1.0, -phi, 0.0),
                    (phi, 0.0, 1.0),
                    (phi, 0.0, -1.0),
                    (-phi, 0.0, 1.0),
                    (-phi, 0.0, -1.0),
                ]
                .iter()
                .map(|&(x, y, z)| (x * scale, y * scale, z * scale))
                .collect()
            }
        }
    }

    /// Calculate sacred geometry resonance between two patterns
    ///
    /// Used by ActiveIntelligenceEngine for pattern optimization.
    pub fn calculate_pattern_resonance(&self, pattern_a: &[Float], pattern_b: &[Float]) -> Float {
        if pattern_a.len() != pattern_b.len() || pattern_a.is_empty() {
            return 0.0;
        }

        // Calculate dot product
        let dot: Float = pattern_a
            .iter()
            .zip(pattern_b.iter())
            .map(|(a, b)| a * b)
            .sum();

        // Calculate magnitudes
        let mag_a: Float = pattern_a.iter().map(|x| x * x).sum::<Float>().sqrt();
        let mag_b: Float = pattern_b.iter().map(|x| x * x).sum::<Float>().sqrt();

        if mag_a > 0.0 && mag_b > 0.0 {
            // Cosine similarity scaled to [0, 1]
            let cosine = dot / (mag_a * mag_b);
            (cosine + 1.0) / 2.0
        } else {
            0.0
        }
    }

    /// Get phi powers array for quick lookup
    ///
    /// Returns pre-computed φ^0 through φ^19
    pub fn phi_powers(&self) -> [Float; 20] {
        let mut powers = [0.0; 20];
        for i in 0..20 {
            powers[i] = constants::GOLDEN_RATIO.powi(i as i32);
        }
        powers
    }
}

impl Default for SacredGeometrySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_ratio_constant() {
        let phi = constants::GOLDEN_RATIO;
        let phi_inverse = constants::GOLDEN_RATIO_INVERSE;

        // φ × 1/φ should equal 1
        assert!((phi * phi_inverse - 1.0).abs() < 1e-10);

        // φ - 1 should equal 1/φ
        assert!((phi - 1.0 - phi_inverse).abs() < 1e-10);
    }

    #[test]
    fn test_fibonacci_sequence() {
        let system = SacredGeometrySystem::new();

        // First 10 Fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
        assert_eq!(system.fibonacci_number(0), 0);
        assert_eq!(system.fibonacci_number(1), 1);
        assert_eq!(system.fibonacci_number(2), 1);
        assert_eq!(system.fibonacci_number(3), 2);
        assert_eq!(system.fibonacci_number(4), 3);
        assert_eq!(system.fibonacci_number(5), 5);
        assert_eq!(system.fibonacci_number(6), 8);
        assert_eq!(system.fibonacci_number(7), 13);
        assert_eq!(system.fibonacci_number(8), 21);
        assert_eq!(system.fibonacci_number(9), 34);
        assert_eq!(system.fibonacci_number(10), 55);
    }

    #[test]
    fn test_golden_proportion_check() {
        let system = SacredGeometrySystem::new();

        // φ and 1 are in golden ratio
        assert!(system.is_golden_proportion(constants::GOLDEN_RATIO, 1.0, 0.01));

        // 1.618 and 1 are in golden ratio
        assert!(system.is_golden_proportion(1.618, 1.0, 0.01));

        // 2.618 and 1.618 are in golden ratio (φ² and φ)
        assert!(system.is_golden_proportion(
            constants::GOLDEN_RATIO_SQUARED,
            constants::GOLDEN_RATIO,
            0.01
        ));
    }

    #[test]
    fn test_platonic_solids_initialized() {
        let system = SacredGeometrySystem::new();

        // All 5 Platonic solids should be initialized
        assert_eq!(system.platonic_solids.len(), 5);

        // Check each solid type exists
        assert!(system
            .get_platonic_solid(PlatonicSolid::Tetrahedron)
            .is_some());
        assert!(system.get_platonic_solid(PlatonicSolid::Cube).is_some());
        assert!(system
            .get_platonic_solid(PlatonicSolid::Octahedron)
            .is_some());
        assert!(system
            .get_platonic_solid(PlatonicSolid::Dodecahedron)
            .is_some());
        assert!(system
            .get_platonic_solid(PlatonicSolid::Icosahedron)
            .is_some());
    }

    #[test]
    fn test_spiral_creation() {
        let mut system = SacredGeometrySystem::new();
        let spiral = system.create_golden_spiral((0.0, 0.0), 5.0);

        assert_eq!(system.spirals.len(), 1);
        assert!(matches!(spiral.spiral_type(), SpiralType::Golden));
    }

    #[test]
    fn test_vesica_piscis_creation() {
        let system = SacredGeometrySystem::new();
        let vesica = system.vesica_piscis((0.0, 0.0), (1.0, 0.0), 1.0);

        // Should have intersection points
        assert!(!vesica.intersection_points().is_empty());
    }

    #[test]
    fn test_flower_of_life_creation() {
        let system = SacredGeometrySystem::new();
        let flower = system.flower_of_life((0.0, 0.0), 1.0, 3);

        // Center circle exists
        assert!(!flower.circles().is_empty());

        // With 3 layers, should have multiple circles
        assert!(flower.circles().len() > 1);
    }

    #[test]
    fn test_harmonic_resonance_system() {
        let system = SacredGeometrySystem::new();

        // Test harmonic resonance calculation
        let resonance = system.calculate_harmonic_resonance(0.5, 0.5, 0.8, 0.8);
        assert!(resonance.resonance_strength > 0.0);

        // Test harmonic series creation
        let series = system.harmonic_resonance().create_harmonic_series(0.5);
        assert!(series.fundamental > 0.0);

        // Test musical note creation
        let note = system.harmonic_resonance().create_musical_note(0.5);
        assert!(note.frequency > 0.0);

        // Test overtone pattern creation
        let pattern = system.harmonic_resonance().create_overtone_pattern(22);
        assert!(!pattern.strengths.is_empty());

        // Test standing wave calculation
        let wave = system
            .harmonic_resonance()
            .calculate_standing_wave(440.0, 880.0);
        assert!(wave.center_amplitude > 0.0);
    }

    // ========================================================================
    // Phase 8.2 Tests: Active Intelligence Integration
    // ========================================================================

    #[test]
    fn test_phi_scale_positive() {
        let system = SacredGeometrySystem::new();

        // φ^0 = 1
        assert!((system.phi_scale(1.0, 0) - 1.0).abs() < 0.001);

        // φ^1 ≈ 1.618
        assert!((system.phi_scale(1.0, 1) - constants::GOLDEN_RATIO).abs() < 0.001);

        // φ^2 ≈ 2.618
        assert!((system.phi_scale(1.0, 2) - constants::GOLDEN_RATIO_SQUARED).abs() < 0.001);

        // Scaling a value: 2 × φ^1 ≈ 3.236
        let scaled = system.phi_scale(2.0, 1);
        assert!((scaled - 2.0 * constants::GOLDEN_RATIO).abs() < 0.001);
    }

    #[test]
    fn test_phi_scale_negative() {
        let system = SacredGeometrySystem::new();

        // φ^-1 = 1/φ ≈ 0.618
        assert!((system.phi_scale(1.0, -1) - constants::GOLDEN_RATIO_INVERSE).abs() < 0.001);

        // φ^-2 ≈ 0.382
        let neg2 = system.phi_scale(1.0, -2);
        assert!(neg2 > 0.0 && neg2 < 0.5);
    }

    #[test]
    fn test_fibonacci_spiral_point() {
        let system = SacredGeometrySystem::new();

        // Point at index 0
        let (x0, y0) = system.fibonacci_spiral_point(0);
        // Should be at origin (F(0) = 0)
        assert!(x0.abs() < 0.001);
        assert!(y0.abs() < 0.001);

        // Point at index 1
        let (x1, y1) = system.fibonacci_spiral_point(1);
        // Should have non-zero coordinates
        assert!(x1.abs() > 0.0 || y1.abs() > 0.0);

        // Points should spiral outward
        let (x5, y5) = system.fibonacci_spiral_point(5);
        let r5 = (x5 * x5 + y5 * y5).sqrt();
        let (x10, y10) = system.fibonacci_spiral_point(10);
        let r10 = (x10 * x10 + y10 * y10).sqrt();

        // Later points should be further from origin
        assert!(r10 > r5);
    }

    #[test]
    fn test_platonic_solid_tetrahedron() {
        let system = SacredGeometrySystem::new();

        let vertices = system.platonic_solid(PlatonicSolid::Tetrahedron, 1.0);

        // Tetrahedron has 4 vertices
        assert_eq!(vertices.len(), 4);

        // All vertices should be at distance sqrt(3) from origin (for scale=1)
        for (x, y, z) in &vertices {
            let dist = (x * x + y * y + z * z).sqrt();
            assert!((dist - 3.0_f64.sqrt()).abs() < 0.001);
        }
    }

    #[test]
    fn test_platonic_solid_cube() {
        let system = SacredGeometrySystem::new();

        let vertices = system.platonic_solid(PlatonicSolid::Cube, 1.0);

        // Cube has 8 vertices
        assert_eq!(vertices.len(), 8);

        // All coordinates should be ±1
        for (x, y, z) in &vertices {
            assert!(x.abs() <= 1.0);
            assert!(y.abs() <= 1.0);
            assert!(z.abs() <= 1.0);
        }
    }

    #[test]
    fn test_platonic_solid_octahedron() {
        let system = SacredGeometrySystem::new();

        let vertices = system.platonic_solid(PlatonicSolid::Octahedron, 1.0);

        // Octahedron has 6 vertices
        assert_eq!(vertices.len(), 6);

        // All vertices should be at distance 1 from origin
        for (x, y, z) in &vertices {
            let dist = (x * x + y * y + z * z).sqrt();
            assert!((dist - 1.0).abs() < 0.001);
        }
    }

    #[test]
    fn test_platonic_solid_dodecahedron() {
        let system = SacredGeometrySystem::new();

        let vertices = system.platonic_solid(PlatonicSolid::Dodecahedron, 1.0);

        // Dodecahedron has 20 vertices
        assert_eq!(vertices.len(), 20);

        // Dodecahedron includes golden ratio
        // Some vertices should have coordinates involving φ
        let has_phi = vertices.iter().any(|(x, y, z)| {
            let phi = constants::GOLDEN_RATIO;
            (x - phi).abs() < 0.001
                || (y - phi).abs() < 0.001
                || (z - phi).abs() < 0.001
                || (x + phi).abs() < 0.001
                || (y + phi).abs() < 0.001
                || (z + phi).abs() < 0.001
        });
        assert!(has_phi);
    }

    #[test]
    fn test_platonic_solid_icosahedron() {
        let system = SacredGeometrySystem::new();

        let vertices = system.platonic_solid(PlatonicSolid::Icosahedron, 1.0);

        // Icosahedron has 12 vertices
        assert_eq!(vertices.len(), 12);

        // Icosahedron includes golden ratio
        let has_phi = vertices.iter().any(|(x, y, z)| {
            let phi = constants::GOLDEN_RATIO;
            (x - phi).abs() < 0.001 || (y - phi).abs() < 0.001 || (z - phi).abs() < 0.001
        });
        assert!(has_phi);
    }

    #[test]
    fn test_platonic_solid_scaling() {
        let system = SacredGeometrySystem::new();

        let v1 = system.platonic_solid(PlatonicSolid::Tetrahedron, 1.0);
        let v2 = system.platonic_solid(PlatonicSolid::Tetrahedron, 2.0);

        // Larger scale should have larger coordinates
        for ((x1, y1, z1), (x2, y2, z2)) in v1.iter().zip(v2.iter()) {
            assert!((x2 - 2.0 * x1).abs() < 0.001);
            assert!((y2 - 2.0 * y1).abs() < 0.001);
            assert!((z2 - 2.0 * z1).abs() < 0.001);
        }
    }

    #[test]
    fn test_pattern_resonance() {
        let system = SacredGeometrySystem::new();

        // Identical patterns should have resonance = 1.0
        let pattern = vec![0.5, 0.5, 0.5, 0.5];
        let resonance = system.calculate_pattern_resonance(&pattern, &pattern);
        assert!((resonance - 1.0).abs() < 0.001);

        // Orthogonal patterns should have resonance = 0.5
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let ortho_resonance = system.calculate_pattern_resonance(&a, &b);
        assert!((ortho_resonance - 0.5).abs() < 0.001);

        // Empty patterns should return 0.0
        let empty: Vec<Float> = vec![];
        assert_eq!(system.calculate_pattern_resonance(&empty, &empty), 0.0);
    }

    #[test]
    fn test_phi_powers() {
        let system = SacredGeometrySystem::new();
        let powers = system.phi_powers();

        // Should have 20 elements
        assert_eq!(powers.len(), 20);

        // φ^0 = 1
        assert!((powers[0] - 1.0).abs() < 0.001);

        // φ^1 ≈ 1.618
        assert!((powers[1] - constants::GOLDEN_RATIO).abs() < 0.001);

        // φ^2 ≈ 2.618
        assert!((powers[2] - constants::GOLDEN_RATIO_SQUARED).abs() < 0.001);

        // Each power should be φ times the previous
        for i in 1..20 {
            assert!((powers[i] / powers[i - 1] - constants::GOLDEN_RATIO).abs() < 0.001);
        }
    }
}
