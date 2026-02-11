//! Scale Architecture Module
//!
//! This module implements the 9-scale hierarchy with fractal containment
//! according to the Law of One cosmology. Each scale is holographically
//! connected to all other scales and contains all smaller scales.
//!
//! Scale Hierarchy:
//! - Scale 1: Sub-Atomic (Quantum realm, particles)
//! - Scale 2: Atomic (Atoms, electron configurations)
//! - Scale 3: Molecular (Chemical bonds, molecules)
//! - Scale 4: Cellular (Biological structures, DNA)
//! - Scale 5: Organismic (Metabolism, reproduction, evolution)
//! - Scale 6: Planetary (Planets, ecosystems)
//! - Scale 7: Stellar (Stars, solar systems)
//! - Scale 8: Galactic (Galaxies, star systems)
//! - Scale 9: Universal (Entire universe)

use std::collections::HashMap;
use std::hash::Hash;

use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};
use crate::natural_laws::NaturalLaws;
use crate::physical_dimension::PhysicalDimension;
use crate::types::Density;
use crate::types::Float;

/// Unique identifier for a scale instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleID(u64);

impl ScaleID {
    pub fn new(id: u64) -> Self {
        ScaleID(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Scale types in the 9-scale hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaleType {
    /// Scale 1: Sub-Atomic (Quantum realm, particles)
    SubAtomic = 1,

    /// Scale 2: Atomic (Atoms, electron configurations)
    Atomic = 2,

    /// Scale 3: Molecular (Chemical bonds, molecules)
    Molecular = 3,

    /// Scale 4: Cellular (Biological structures, DNA)
    Cellular = 4,

    /// Scale 5: Organismic (Metabolism, reproduction, evolution)
    Organismic = 5,

    /// Scale 6: Planetary (Planets, ecosystems)
    Planetary = 6,

    /// Scale 7: Stellar (Stars, solar systems)
    Stellar = 7,

    /// Scale 8: Galactic (Galaxies, star systems)
    Galactic = 8,

    /// Scale 9: Universal (Entire universe)
    Universal = 9,
}

impl ScaleType {
    /// Get scale number (1-9)
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Get scale name
    pub fn name(&self) -> &str {
        match self {
            ScaleType::SubAtomic => "Sub-Atomic",
            ScaleType::Atomic => "Atomic",
            ScaleType::Molecular => "Molecular",
            ScaleType::Cellular => "Cellular",
            ScaleType::Organismic => "Organismic",
            ScaleType::Planetary => "Planetary",
            ScaleType::Stellar => "Stellar",
            ScaleType::Galactic => "Galactic",
            ScaleType::Universal => "Universal",
        }
    }

    /// Get scale description
    pub fn description(&self) -> &str {
        match self {
            ScaleType::SubAtomic => "Quantum realm, particles, uncertainty principle",
            ScaleType::Atomic => "Atoms, electron configurations, nuclear structure",
            ScaleType::Molecular => "Chemical bonds, molecules, chemical reactions",
            ScaleType::Cellular => "Biological structures, DNA, protein synthesis",
            ScaleType::Organismic => "Metabolism, reproduction, evolution, self-awareness",
            ScaleType::Planetary => "Planets, geology, atmosphere, ecosystems",
            ScaleType::Stellar => "Stars, nuclear fusion, stellar evolution, solar systems",
            ScaleType::Galactic => "Galaxies, dark matter, galactic dynamics",
            ScaleType::Universal => "Entire universe, cosmology, universal constants",
        }
    }

    /// Get primary density associated with this scale
    pub fn primary_density(&self) -> Density {
        match self {
            ScaleType::SubAtomic => Density::First,
            ScaleType::Atomic => Density::First,
            ScaleType::Molecular => Density::Second,
            ScaleType::Cellular => Density::Second,
            ScaleType::Organismic => Density::Third,
            ScaleType::Planetary => Density::Fourth,
            ScaleType::Stellar => Density::Fifth,
            ScaleType::Galactic => Density::Sixth,
            ScaleType::Universal => Density::Seventh,
        }
    }

    /// Get scale-specific physics type
    pub fn physics_type(&self) -> ScalePhysics {
        match self {
            ScaleType::SubAtomic => ScalePhysics::QuantumMechanics,
            ScaleType::Atomic => ScalePhysics::QuantumChemistry,
            ScaleType::Molecular => ScalePhysics::Thermodynamics,
            ScaleType::Cellular => ScalePhysics::Biochemistry,
            ScaleType::Organismic => ScalePhysics::Biology,
            ScaleType::Planetary => ScalePhysics::Geophysics,
            ScaleType::Stellar => ScalePhysics::StellarPhysics,
            ScaleType::Galactic => ScalePhysics::GalacticDynamics,
            ScaleType::Universal => ScalePhysics::Cosmology,
        }
    }

    /// Check if this scale contains another scale
    pub fn contains(&self, other: &ScaleType) -> bool {
        self.as_u8() > other.as_u8()
    }

    /// Get all scales contained by this scale
    pub fn contained_scales(&self) -> Vec<ScaleType> {
        let mut contained = Vec::new();
        for i in 1..self.as_u8() {
            match i {
                1 => contained.push(ScaleType::SubAtomic),
                2 => contained.push(ScaleType::Atomic),
                3 => contained.push(ScaleType::Molecular),
                4 => contained.push(ScaleType::Cellular),
                5 => contained.push(ScaleType::Organismic),
                6 => contained.push(ScaleType::Planetary),
                7 => contained.push(ScaleType::Stellar),
                8 => contained.push(ScaleType::Galactic),
                _ => {}
            }
        }
        contained
    }

    /// Get all scales that contain this scale
    pub fn containing_scales(&self) -> Vec<ScaleType> {
        let mut containing = Vec::new();
        for i in (self.as_u8() + 1)..=9 {
            match i {
                2 => containing.push(ScaleType::Atomic),
                3 => containing.push(ScaleType::Molecular),
                4 => containing.push(ScaleType::Cellular),
                5 => containing.push(ScaleType::Organismic),
                6 => containing.push(ScaleType::Planetary),
                7 => containing.push(ScaleType::Stellar),
                8 => containing.push(ScaleType::Galactic),
                9 => containing.push(ScaleType::Universal),
                _ => {}
            }
        }
        containing
    }
}

/// Scale-specific physics types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalePhysics {
    QuantumMechanics,
    QuantumChemistry,
    Thermodynamics,
    Biochemistry,
    Biology,
    Geophysics,
    StellarPhysics,
    GalacticDynamics,
    Cosmology,
}

impl ScalePhysics {
    pub fn name(&self) -> &str {
        match self {
            ScalePhysics::QuantumMechanics => "Quantum Mechanics",
            ScalePhysics::QuantumChemistry => "Quantum Chemistry",
            ScalePhysics::Thermodynamics => "Thermodynamics",
            ScalePhysics::Biochemistry => "Biochemistry",
            ScalePhysics::Biology => "Biology",
            ScalePhysics::Geophysics => "Geophysics",
            ScalePhysics::StellarPhysics => "Stellar Physics",
            ScalePhysics::GalacticDynamics => "Galactic Dynamics",
            ScalePhysics::Cosmology => "Cosmology",
        }
    }
}

/// Holographic connection between scales
#[derive(Debug, Clone)]
pub struct HolographicConnection {
    /// Source scale
    pub from_scale: ScaleID,

    /// Target scale
    pub to_scale: ScaleID,

    /// Coherence level (0.0 to 1.0)
    pub coherence: Float,

    /// Information flow direction
    pub information_flow: InformationFlow,

    /// Connection strength
    pub strength: Float,
}

impl HolographicConnection {
    pub fn new(
        from_scale: ScaleID,
        to_scale: ScaleID,
        coherence: Float,
        information_flow: InformationFlow,
    ) -> Self {
        HolographicConnection {
            from_scale,
            to_scale,
            coherence: coherence.clamp(0.0, 1.0),
            information_flow,
            strength: coherence,
        }
    }

    /// Check if connection is bidirectional
    pub fn is_bidirectional(&self) -> bool {
        matches!(self.information_flow, InformationFlow::Bidirectional)
    }
}

/// Information flow direction in holographic connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InformationFlow {
    /// Information flows both ways
    Bidirectional,

    /// Information flows upward (smaller to larger scale)
    UpwardOnly,

    /// Information flows downward (larger to smaller scale)
    DownwardOnly,
}

/// A scale in the 9-scale hierarchy
#[derive(Debug, Clone)]
pub struct Scale {
    /// Unique identifier
    pub scale_id: ScaleID,

    /// Scale type
    pub scale_type: ScaleType,

    /// Physical dimension for this scale
    pub physical_dimension: PhysicalDimension,

    /// Natural laws specific to this scale
    pub natural_laws: NaturalLaws,

    /// Scale-specific physics variations
    pub scale_physics: ScalePhysics,

    /// Scales contained by this scale (fractal containment)
    pub contains: Vec<ScaleID>,

    /// Scale that contains this scale (if any)
    pub contained_by: Option<ScaleID>,

    /// Holographic connections to other scales
    pub holographic_connections: Vec<HolographicConnection>,

    /// Coherence level of this scale (0.0 to 1.0)
    pub coherence: Float,

    /// Is this scale alive (conscious)?
    pub is_alive: bool,

    /// Scale-specific consciousness level
    pub consciousness_level: Float,
}

impl Scale {
    /// Create a new scale
    pub fn new(
        scale_id: ScaleID,
        scale_type: ScaleType,
        physical_dimension: PhysicalDimension,
        natural_laws: NaturalLaws,
    ) -> Self {
        let scale_physics = scale_type.physics_type();

        Scale {
            scale_id,
            scale_type,
            physical_dimension,
            natural_laws,
            scale_physics,
            contains: Vec::new(),
            contained_by: None,
            holographic_connections: Vec::new(),
            coherence: 1.0,
            is_alive: true,
            consciousness_level: scale_type.primary_density().as_u8() as Float / 7.0,
        }
    }

    /// Add a holographic connection to another scale
    pub fn add_holographic_connection(&mut self, connection: HolographicConnection) {
        self.holographic_connections.push(connection);
    }

    /// Check if this scale contains another scale
    pub fn contains_scale(&self, other_scale_id: ScaleID) -> bool {
        self.contains.contains(&other_scale_id)
    }

    /// Get scale-specific physics description
    pub fn get_physics_description(&self) -> String {
        format!(
            "{} Scale: {} (Density {})",
            self.scale_type.name(),
            self.scale_physics.name(),
            self.scale_type.primary_density().as_u8()
        )
    }

    /// Update coherence level
    pub fn update_coherence(&mut self, delta: Float) {
        self.coherence = (self.coherence + delta).clamp(0.0, 1.0);
    }

    /// Calculate holographic influence on another scale
    pub fn calculate_influence(&self, target_type: ScaleType) -> Float {
        // Influence based on scale difference and coherence
        let scale_diff = (self.scale_type.as_u8() as Float - target_type.as_u8() as Float).abs();
        let base_influence = self.coherence / (1.0 + scale_diff);
        base_influence
    }
}

/// Scale hierarchy manager
#[derive(Debug, Clone)]
pub struct ScaleHierarchy {
    /// All scales indexed by ID
    pub scales: HashMap<ScaleID, Scale>,

    /// Root scale (Universal scale)
    pub root_scale: ScaleID,

    /// Next scale ID to assign
    next_scale_id: u64,

    /// Overall hierarchy coherence
    pub hierarchy_coherence: Float,
}

impl ScaleHierarchy {
    /// Create a new scale hierarchy
    pub fn new() -> Self {
        ScaleHierarchy {
            scales: HashMap::new(),
            root_scale: ScaleID(0), // Will be set during creation
            next_scale_id: 1,
            hierarchy_coherence: 1.0,
        }
    }

    /// Create fractal hierarchy with 9 scales
    pub fn create_fractal_hierarchy(&mut self, base_seed: u64) {
        // Clear existing hierarchy
        self.scales.clear();
        self.next_scale_id = 1;
        self.hierarchy_coherence = 1.0;

        // Create all 9 scales
        let scale_ids = self.create_all_scales(base_seed);

        // Set root scale (Universal)
        self.root_scale = scale_ids[8]; // Index 8 = Universal

        // Establish fractal containment
        self.establish_fractal_containment(&scale_ids);

        // Establish holographic connections
        self.establish_holographic_connections(&scale_ids);

        // Update hierarchy coherence
        self.update_hierarchy_coherence();
    }

    /// Create all 9 scales
    fn create_all_scales(&mut self, base_seed: u64) -> Vec<ScaleID> {
        let mut scale_ids = Vec::new();

        let scale_types = [
            ScaleType::SubAtomic,
            ScaleType::Atomic,
            ScaleType::Molecular,
            ScaleType::Cellular,
            ScaleType::Organismic,
            ScaleType::Planetary,
            ScaleType::Stellar,
            ScaleType::Galactic,
            ScaleType::Universal,
        ];

        for scale_type in scale_types {
            let scale_id = self.next_scale_id();
            scale_ids.push(scale_id);

            // Create scale-specific components
            let physical_dimension = self.create_physical_dimension(scale_type, base_seed);
            let natural_laws = self.create_natural_laws(scale_type, base_seed);

            let scale = Scale::new(scale_id, scale_type, physical_dimension, natural_laws);

            self.scales.insert(scale_id, scale);
        }

        scale_ids
    }

    /// Create physical dimension for a scale
    fn create_physical_dimension(&self, _scale_type: ScaleType, _seed: u64) -> PhysicalDimension {
        // Create scale-specific physical dimension
        // In a full implementation, this would vary based on scale type
        // For now, use earth_like as a placeholder
        PhysicalDimension::earth_like()
    }

    /// Create natural laws for a scale
    fn create_natural_laws(&self, _scale_type: ScaleType, _seed: u64) -> NaturalLaws {
        // Create scale-specific natural laws
        // In a full implementation, this would vary based on scale type
        // For now, use earth_like as a placeholder
        NaturalLaws::earth_like()
    }

    /// Establish fractal containment relationships
    fn establish_fractal_containment(&mut self, scale_ids: &[ScaleID]) {
        for (i, &larger_scale_id) in scale_ids.iter().enumerate() {
            for (j, &smaller_scale_id) in scale_ids.iter().enumerate() {
                if i > j {
                    // Larger scale contains smaller scale
                    if let Some(larger_scale) = self.scales.get_mut(&larger_scale_id) {
                        larger_scale.contains.push(smaller_scale_id);
                    }

                    if let Some(smaller_scale) = self.scales.get_mut(&smaller_scale_id) {
                        smaller_scale.contained_by = Some(larger_scale_id);
                    }
                }
            }
        }
    }

    /// Establish holographic connections between scales
    fn establish_holographic_connections(&mut self, scale_ids: &[ScaleID]) {
        for (i, &from_id) in scale_ids.iter().enumerate() {
            for (j, &to_id) in scale_ids.iter().enumerate() {
                if i != j {
                    // Every scale connects to every other scale
                    // Determine information flow
                    let information_flow = if i < j {
                        InformationFlow::UpwardOnly
                    } else {
                        InformationFlow::DownwardOnly
                    };

                    // Calculate coherence based on scale difference
                    let scale_diff = (i as Float - j as Float).abs();
                    let coherence = 1.0 / (1.0 + scale_diff * 0.1);

                    let connection =
                        HolographicConnection::new(from_id, to_id, coherence, information_flow);

                    if let Some(scale) = self.scales.get_mut(&from_id) {
                        scale.add_holographic_connection(connection);
                    }
                }
            }
        }
    }

    /// Update overall hierarchy coherence
    fn update_hierarchy_coherence(&mut self) {
        let total_coherence: Float = self.scales.values().map(|scale| scale.coherence).sum();

        self.hierarchy_coherence = total_coherence / self.scales.len() as Float;
    }

    /// Get a scale by ID
    pub fn get_scale(&self, scale_id: ScaleID) -> Option<&Scale> {
        self.scales.get(&scale_id)
    }

    /// Get a mutable scale by ID
    pub fn get_scale_mut(&mut self, scale_id: ScaleID) -> Option<&mut Scale> {
        self.scales.get_mut(&scale_id)
    }

    /// Get scale by type
    pub fn get_scale_by_type(&self, scale_type: ScaleType) -> Option<&Scale> {
        self.scales
            .values()
            .find(|scale| scale.scale_type == scale_type)
    }

    /// Get all scales contained by a given scale
    pub fn get_contained_scales(&self, scale_id: ScaleID) -> Vec<&Scale> {
        if let Some(scale) = self.scales.get(&scale_id) {
            scale
                .contains
                .iter()
                .filter_map(|id| self.scales.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all scales that contain a given scale
    pub fn get_containing_scales(&self, scale_id: ScaleID) -> Vec<&Scale> {
        if let Some(scale) = self.scales.get(&scale_id) {
            if let Some(contained_by) = scale.contained_by {
                vec![self.scales.get(&contained_by).unwrap()]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    /// Add a scale to the hierarchy
    pub fn add_scale(&mut self, scale: Scale) -> ScaleID {
        let scale_id = scale.scale_id;
        self.scales.insert(scale_id, scale);
        scale_id
    }

    /// Get root scale (Universal)
    pub fn get_root_scale(&self) -> Option<&Scale> {
        self.scales.get(&self.root_scale)
    }

    /// Get all scale IDs
    pub fn get_all_scale_ids(&self) -> Vec<ScaleID> {
        self.scales.keys().copied().collect()
    }

    /// Get number of scales
    pub fn scale_count(&self) -> usize {
        self.scales.len()
    }

    /// Get hierarchy statistics
    pub fn get_statistics(&self) -> HierarchyStatistics {
        let mut total_connections = 0;
        let mut total_coherence = 0.0;

        for scale in self.scales.values() {
            total_connections += scale.holographic_connections.len();
            total_coherence += scale.coherence;
        }

        HierarchyStatistics {
            total_scales: self.scales.len(),
            total_connections,
            average_coherence: total_coherence / self.scales.len() as Float,
            hierarchy_coherence: self.hierarchy_coherence,
        }
    }

    /// Generate next scale ID
    fn next_scale_id(&mut self) -> ScaleID {
        let id = self.next_scale_id;
        self.next_scale_id += 1;
        ScaleID(id)
    }
}

impl Default for ScaleHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

/// Hierarchy statistics
#[derive(Debug, Clone)]
pub struct HierarchyStatistics {
    pub total_scales: usize,
    pub total_connections: usize,
    pub average_coherence: Float,
    pub hierarchy_coherence: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_type_properties() {
        // Test scale type properties
        assert_eq!(ScaleType::SubAtomic.as_u8(), 1);
        assert_eq!(ScaleType::Universal.as_u8(), 9);

        assert_eq!(ScaleType::SubAtomic.primary_density(), Density::First);
        assert_eq!(ScaleType::Organismic.primary_density(), Density::Third);
        assert_eq!(ScaleType::Universal.primary_density(), Density::Seventh);

        assert_eq!(
            ScaleType::SubAtomic.physics_type(),
            ScalePhysics::QuantumMechanics
        );
        assert_eq!(
            ScaleType::Stellar.physics_type(),
            ScalePhysics::StellarPhysics
        );
    }

    #[test]
    fn test_scale_containment() {
        // Test that larger scales contain smaller scales
        assert!(ScaleType::Universal.contains(&ScaleType::Galactic));
        assert!(ScaleType::Universal.contains(&ScaleType::SubAtomic));
        assert!(ScaleType::Organismic.contains(&ScaleType::Cellular));
        assert!(ScaleType::Organismic.contains(&ScaleType::SubAtomic));

        // Test that smaller scales don't contain larger scales
        assert!(!ScaleType::SubAtomic.contains(&ScaleType::Universal));
        assert!(!ScaleType::Atomic.contains(&ScaleType::Galactic));
    }

    #[test]
    fn test_contained_scales() {
        // Test contained scales
        let universal_contained = ScaleType::Universal.contained_scales();
        assert_eq!(universal_contained.len(), 8);

        let organismic_contained = ScaleType::Organismic.contained_scales();
        assert_eq!(organismic_contained.len(), 4);

        let subatomic_contained = ScaleType::SubAtomic.contained_scales();
        assert_eq!(subatomic_contained.len(), 0);
    }

    #[test]
    fn test_containing_scales() {
        // Test containing scales
        let subatomic_containing = ScaleType::SubAtomic.containing_scales();
        assert_eq!(subatomic_containing.len(), 8);

        let organismic_containing = ScaleType::Organismic.containing_scales();
        assert_eq!(organismic_containing.len(), 4);

        let universal_containing = ScaleType::Universal.containing_scales();
        assert_eq!(universal_containing.len(), 0);
    }

    #[test]
    fn test_scale_creation() {
        // Test scale creation
        let scale_id = ScaleID::new(1);
        let scale_type = ScaleType::Organismic;

        let physical_dimension = PhysicalDimension::earth_like();
        let natural_laws = NaturalLaws::earth_like();

        let scale = Scale::new(scale_id, scale_type, physical_dimension, natural_laws);

        assert_eq!(scale.scale_id, scale_id);
        assert_eq!(scale.scale_type, scale_type);
        assert_eq!(scale.scale_physics, ScalePhysics::Biology);
        assert_eq!(scale.is_alive, true);
        assert!(scale.coherence > 0.0);
    }

    #[test]
    fn test_scale_hierarchy_creation() {
        // Test hierarchy creation
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        // Should have 9 scales
        assert_eq!(hierarchy.scale_count(), 9);

        // Should have root scale
        assert!(hierarchy.get_root_scale().is_some());
        assert_eq!(
            hierarchy.get_root_scale().unwrap().scale_type,
            ScaleType::Universal
        );
    }

    #[test]
    fn test_fractal_containment() {
        // Test fractal containment
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        // Get universal scale
        let universal = hierarchy.get_scale_by_type(ScaleType::Universal).unwrap();
        assert_eq!(universal.contains.len(), 8);

        // Get organismic scale
        let organismic = hierarchy.get_scale_by_type(ScaleType::Organismic).unwrap();
        assert_eq!(organismic.contains.len(), 4);

        // Get subatomic scale
        let subatomic = hierarchy.get_scale_by_type(ScaleType::SubAtomic).unwrap();
        assert_eq!(subatomic.contains.len(), 0);
    }

    #[test]
    fn test_holographic_connections() {
        // Test holographic connections
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        // Each scale should have connections to all other scales
        for scale in hierarchy.scales.values() {
            // Should have 8 connections (to all other scales)
            assert_eq!(scale.holographic_connections.len(), 8);
        }
    }

    #[test]
    fn test_get_scale_by_type() {
        // Test getting scale by type
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        assert!(hierarchy.get_scale_by_type(ScaleType::SubAtomic).is_some());
        assert!(hierarchy.get_scale_by_type(ScaleType::Atomic).is_some());
        assert!(hierarchy.get_scale_by_type(ScaleType::Universal).is_some());
    }

    #[test]
    fn test_get_contained_scales() {
        // Test getting contained scales
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        let universal_id = hierarchy
            .get_scale_by_type(ScaleType::Universal)
            .unwrap()
            .scale_id;
        let contained = hierarchy.get_contained_scales(universal_id);
        assert_eq!(contained.len(), 8);

        let organismic_id = hierarchy
            .get_scale_by_type(ScaleType::Organismic)
            .unwrap()
            .scale_id;
        let contained = hierarchy.get_contained_scales(organismic_id);
        assert_eq!(contained.len(), 4);
    }

    #[test]
    fn test_hierarchy_statistics() {
        // Test hierarchy statistics
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        let stats = hierarchy.get_statistics();
        assert_eq!(stats.total_scales, 9);
        assert_eq!(stats.total_connections, 9 * 8); // 9 scales, 8 connections each
        assert!(stats.average_coherence > 0.0);
        assert!(stats.hierarchy_coherence > 0.0);
    }

    #[test]
    fn test_scale_physics_description() {
        // Test physics description
        let scale_id = ScaleID::new(1);
        let scale_type = ScaleType::Stellar;

        let physical_dimension = PhysicalDimension::earth_like();
        let natural_laws = NaturalLaws::earth_like();

        let scale = Scale::new(scale_id, scale_type, physical_dimension, natural_laws);

        let description = scale.get_physics_description();
        assert!(description.contains("Stellar"));
        assert!(description.contains("Density 5"));
        assert!(description.contains("Stellar Physics"));
    }

    #[test]
    fn test_scale_coherence_update() {
        // Test coherence update
        let scale_id = ScaleID::new(1);
        let scale_type = ScaleType::Molecular;

        let physical_dimension = PhysicalDimension::earth_like();
        let natural_laws = NaturalLaws::earth_like();

        let mut scale = Scale::new(scale_id, scale_type, physical_dimension, natural_laws);

        let initial_coherence = scale.coherence;
        scale.update_coherence(0.1);
        assert_eq!(scale.coherence, (initial_coherence + 0.1).min(1.0));

        scale.update_coherence(-0.2);
        assert_eq!(
            scale.coherence,
            ((initial_coherence + 0.1).min(1.0) - 0.2).max(0.0)
        );
    }

    #[test]
    fn test_scale_influence_calculation() {
        // Test influence calculation
        let scale_id = ScaleID::new(1);
        let scale_type = ScaleType::Planetary;

        let physical_dimension = PhysicalDimension::earth_like();
        let natural_laws = NaturalLaws::earth_like();

        let scale = Scale::new(scale_id, scale_type, physical_dimension, natural_laws);

        // Influence on closer scales should be higher
        let influence_nearby = scale.calculate_influence(ScaleType::Organismic);
        let influence_far = scale.calculate_influence(ScaleType::SubAtomic);

        assert!(influence_nearby > influence_far);
    }

    #[test]
    fn test_holographic_connection() {
        // Test holographic connection
        let from_id = ScaleID::new(1);
        let to_id = ScaleID::new(2);

        let connection =
            HolographicConnection::new(from_id, to_id, 0.8, InformationFlow::Bidirectional);

        assert_eq!(connection.from_scale, from_id);
        assert_eq!(connection.to_scale, to_id);
        assert_eq!(connection.coherence, 0.8);
        assert_eq!(connection.strength, 0.8);
        assert!(connection.is_bidirectional());
    }

    #[test]
    fn test_scale_id() {
        // Test scale ID
        let scale_id = ScaleID::new(42);
        assert_eq!(scale_id.as_u64(), 42);
    }

    #[test]
    fn test_all_scale_types_present() {
        // Test that all scale types are present in hierarchy
        let mut hierarchy = ScaleHierarchy::new();
        hierarchy.create_fractal_hierarchy(42);

        let all_types = vec![
            ScaleType::SubAtomic,
            ScaleType::Atomic,
            ScaleType::Molecular,
            ScaleType::Cellular,
            ScaleType::Organismic,
            ScaleType::Planetary,
            ScaleType::Stellar,
            ScaleType::Galactic,
            ScaleType::Universal,
        ];

        for scale_type in all_types {
            assert!(hierarchy.get_scale_by_type(scale_type).is_some());
        }
    }
}
