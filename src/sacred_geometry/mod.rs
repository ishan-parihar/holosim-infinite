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
pub mod golden_ratio;
pub mod platonic_solids;
pub mod spirals;
pub mod vesica_piscis;
pub mod flower_of_life;
pub mod harmonic_resonance;

pub use fibonacci::{
    FibonacciSequence, FibonacciGrowth, FibonacciSpiral, 
    fibonacci_sequence, fibonacci_number
};
pub use golden_ratio::{
    GOLDEN_RATIO, GOLDEN_RATIO_INVERSE, GoldenProportion, 
    GoldenRectangle, GoldenAngle
};
pub use platonic_solids::{
    PlatonicSolid, PlatonicStructure, PlatonicSolidProportions,
    PlatonicVertex
};
pub use spirals::{
    SpiralType, SpiralPattern, GoldenSpiral, FibonacciSpiral as FibSpiral,
    ArchimedeanSpiral, LogarithmicSpiral, spiral_point
};
pub use vesica_piscis::{
    VesicaPiscis, VesicaPiscisPattern, EntityVesicaPiscis,
    vesica_piscis_intersection
};
pub use flower_of_life::{
    FlowerOfLife, FlowerPattern, FlowerLayer,
    flower_of_life_circles, flower_of_life_pattern
};
pub use harmonic_resonance::{
    HarmonicSeries, MusicalNote, OvertonePattern, StandingWave,
    FrequencyResonance, InterferenceType
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
        self.platonic_solids.iter()
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
    pub fn flower_of_life(&self, center: (Float, Float), radius: Float, layers: usize) -> FlowerOfLife {
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
        self.harmonic_resonance.calculate_entity_resonance(freq_a, freq_b, coherence_a, coherence_b)
    }
    
    /// Get the harmonic resonance subsystem
    pub fn harmonic_resonance(&self) -> &HarmonicResonance {
        &self.harmonic_resonance
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
        assert!(system.is_golden_proportion(constants::GOLDEN_RATIO_SQUARED, constants::GOLDEN_RATIO, 0.01));
    }

    #[test]
    fn test_platonic_solids_initialized() {
        let system = SacredGeometrySystem::new();
        
        // All 5 Platonic solids should be initialized
        assert_eq!(system.platonic_solids.len(), 5);
        
        // Check each solid type exists
        assert!(system.get_platonic_solid(PlatonicSolid::Tetrahedron).is_some());
        assert!(system.get_platonic_solid(PlatonicSolid::Cube).is_some());
        assert!(system.get_platonic_solid(PlatonicSolid::Octahedron).is_some());
        assert!(system.get_platonic_solid(PlatonicSolid::Dodecahedron).is_some());
        assert!(system.get_platonic_solid(PlatonicSolid::Icosahedron).is_some());
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
        let wave = system.harmonic_resonance().calculate_standing_wave(440.0, 880.0);
        assert!(wave.center_amplitude > 0.0);
    }
}
