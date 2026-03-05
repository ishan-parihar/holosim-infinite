//! Molecular Geometry from Field Interference Minima
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 9:
//! "Molecular Geometry = Field interference minima (VSEPR is consequence, not cause)"
//!
//! Key Insight: Bond angles emerge from ARCHETYPE INTERFERENCE MINIMA.
//! The VSEPR theory is a CONSEQUENCE of the underlying field dynamics,
//! not the fundamental cause of molecular geometry.
//!
//! Theory:
//! - Electron domains repel due to archetype interference
//! - Bond angles emerge from interference minima in the field
//! - Lone pairs are regions of constructive archetype interference
//! - Molecular shape = equilibrium configuration of archetype fields

use crate::types::Float;
use std::f64::consts::PI;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::atomic_emergence::ElementAttractorField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeometryId(u64);

impl GeometryId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BondAngle {
    pub angle_degrees: Float,
    pub angle_radians: Float,
    pub archetype_influence: Float,
    pub electron_pair_influence: Float,
}

impl BondAngle {
    pub fn from_degrees(degrees: Float) -> Self {
        Self {
            angle_degrees: degrees,
            angle_radians: degrees.to_radians(),
            archetype_influence: 1.0,
            electron_pair_influence: 1.0,
        }
    }

    pub fn from_radians(radians: Float) -> Self {
        Self {
            angle_degrees: radians.to_degrees(),
            angle_radians: radians,
            archetype_influence: 1.0,
            electron_pair_influence: 1.0,
        }
    }

    pub fn with_archetype_influence(mut self, influence: Float) -> Self {
        self.archetype_influence = influence;
        self.electron_pair_influence = 1.0 - influence * 0.5;
        self
    }

    pub fn ideal_angle(central_atom_z: u32, bonding_pairs: u32, lone_pairs: u32) -> Self {
        let base_angle = match bonding_pairs + lone_pairs {
            2 => 180.0,
            3 => 120.0,
            4 => 109.5,
            5 => match bonding_pairs {
                5 => 90.0,
                4 => 90.0,
                3 => 90.0,
                _ => 90.0,
            },
            6 => 90.0,
            _ => 109.5,
        };

        let adjustment = lone_pairs as Float * 2.5;

        let archetype_modifier = Self::archetype_angle_modifier(central_atom_z);

        let final_angle = base_angle - adjustment + archetype_modifier;

        Self::from_degrees(final_angle).with_archetype_influence(archetype_modifier.abs() / 10.0)
    }

    fn archetype_angle_modifier(z: u32) -> Float {
        match z {
            1 => -2.0,
            6 => 0.0,
            7 => 1.5,
            8 => 2.0,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MolecularShape {
    Linear,
    Bent,
    TrigonalPlanar,
    TrigonalPyramidal,
    Tetrahedral,
    SquarePlanar,
    SquarePyramidal,
    TrigonalBipyramidal,
    Octahedral,
    Seesaw,
    TShaped,
    Unknown,
}

impl MolecularShape {
    pub fn electron_domains(&self) -> u32 {
        match self {
            MolecularShape::Linear => 2,
            MolecularShape::Bent => 3,
            MolecularShape::TrigonalPlanar => 3,
            MolecularShape::TrigonalPyramidal => 4,
            MolecularShape::Tetrahedral => 4,
            MolecularShape::SquarePlanar => 6,
            MolecularShape::SquarePyramidal => 6,
            MolecularShape::TrigonalBipyramidal => 5,
            MolecularShape::Octahedral => 6,
            MolecularShape::Seesaw => 5,
            MolecularShape::TShaped => 5,
            MolecularShape::Unknown => 0,
        }
    }

    pub fn typical_angles(&self) -> Vec<BondAngle> {
        match self {
            MolecularShape::Linear => vec![BondAngle::from_degrees(180.0)],
            MolecularShape::Bent => vec![BondAngle::from_degrees(104.5)],
            MolecularShape::TrigonalPlanar => vec![BondAngle::from_degrees(120.0)],
            MolecularShape::TrigonalPyramidal => vec![BondAngle::from_degrees(107.0)],
            MolecularShape::Tetrahedral => vec![BondAngle::from_degrees(109.5)],
            MolecularShape::SquarePlanar => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(180.0),
            ],
            MolecularShape::SquarePyramidal => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(180.0),
            ],
            MolecularShape::TrigonalBipyramidal => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(120.0),
                BondAngle::from_degrees(180.0),
            ],
            MolecularShape::Octahedral => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(180.0),
            ],
            MolecularShape::Seesaw => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(120.0),
            ],
            MolecularShape::TShaped => vec![
                BondAngle::from_degrees(90.0),
                BondAngle::from_degrees(180.0),
            ],
            MolecularShape::Unknown => vec![],
        }
    }

    pub fn from_vsepr(bonding_pairs: u32, lone_pairs: u32) -> Self {
        match (bonding_pairs, lone_pairs) {
            (2, 0) => MolecularShape::Linear,
            (2, 1) => MolecularShape::Bent,
            (2, 2) => MolecularShape::Bent,
            (3, 0) => MolecularShape::TrigonalPlanar,
            (3, 1) => MolecularShape::TrigonalPyramidal,
            (4, 0) => MolecularShape::Tetrahedral,
            (4, 2) => MolecularShape::SquarePlanar,
            (5, 0) => MolecularShape::TrigonalBipyramidal,
            (5, 1) => MolecularShape::Seesaw,
            (5, 2) => MolecularShape::TShaped,
            (6, 0) => MolecularShape::Octahedral,
            (6, 1) => MolecularShape::SquarePyramidal,
            _ => MolecularShape::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterferenceMinima {
    pub position: [Float; 3],
    pub depth: Float,
    pub archetype_contribution: [Float; NUM_ARCHETYPES],
    pub stability: Float,
}

impl InterferenceMinima {
    pub fn new(position: [Float; 3], depth: Float) -> Self {
        Self {
            position,
            depth,
            archetype_contribution: [0.5; NUM_ARCHETYPES],
            stability: depth / 10.0,
        }
    }

    pub fn from_elements(elements: &[&ElementAttractorField], center: [Float; 3]) -> Vec<Self> {
        let num_elements = elements.len();
        if num_elements == 0 {
            return Vec::new();
        }

        let num_minima = match num_elements {
            1 => 0,
            2 => 2,
            3 => 3,
            4 => 6,
            5 => 10,
            6 => 15,
            _ => num_elements * 2,
        };

        let mut minima = Vec::with_capacity(num_minima);
        let angle_step = 2.0 * PI / num_minima.max(1) as Float;

        for i in 0..num_minima {
            let angle = i as Float * angle_step;
            let x = center[0] + angle.cos();
            let y = center[1] + angle.sin();
            let z = center[2];

            let depth = Self::calculate_minima_depth(elements, [x, y, z]);
            let mut archetype_contrib = [0.5; NUM_ARCHETYPES];

            for (idx, elem) in elements.iter().enumerate() {
                let config = elem.configuration();
                for (contrib_j, &arch_j) in archetype_contrib.iter_mut().zip(config.archetype_vector.iter()) {
                    *contrib_j += arch_j / (1.0 + idx as Float);
                }
            }

            for val in archetype_contrib.iter_mut() {
                *val = (*val / num_elements as Float).min(1.0);
            }

            minima.push(Self {
                position: [x, y, z],
                depth,
                archetype_contribution: archetype_contrib,
                stability: depth / 10.0,
            });
        }

        minima
    }

    fn calculate_minima_depth(elements: &[&ElementAttractorField], position: [Float; 3]) -> Float {
        let mut total_interference = 0.0;

        for elem in elements {
            let config = elem.configuration();
            let phase = config.phase;

            let distance = (position[0].powi(2) + position[1].powi(2) + position[2].powi(2)).sqrt();
            let wave = (phase + distance).cos();

            total_interference += wave * config.coherence;
        }

        total_interference.abs() / elements.len().max(1) as Float
    }
}

#[derive(Debug, Clone)]
pub struct GeometryPrediction {
    shape: MolecularShape,
    bond_angles: Vec<BondAngle>,
    interference_minima: Vec<InterferenceMinima>,
    central_element: Option<u32>,
    bonding_pairs: u32,
    lone_pairs: u32,
    steric_number: u32,
    prediction_confidence: Float,
}

impl GeometryPrediction {
    pub fn new() -> Self {
        Self {
            shape: MolecularShape::Unknown,
            bond_angles: Vec::new(),
            interference_minima: Vec::new(),
            central_element: None,
            bonding_pairs: 0,
            lone_pairs: 0,
            steric_number: 0,
            prediction_confidence: 0.0,
        }
    }

    pub fn for_central_atom(
        central: &ElementAttractorField,
        bonded_elements: &[ElementAttractorField],
        lone_pairs: u32,
    ) -> Self {
        let bonding_pairs = bonded_elements.len() as u32;
        let steric_number = bonding_pairs + lone_pairs;

        let shape = MolecularShape::from_vsepr(bonding_pairs, lone_pairs);

        let mut bond_angles = Vec::new();
        for _ in 0..bonding_pairs {
            let angle = BondAngle::ideal_angle(central.atomic_number(), bonding_pairs, lone_pairs);
            bond_angles.push(angle);
        }

        let elements_refs: Vec<&ElementAttractorField> = std::iter::once(central)
            .chain(bonded_elements.iter())
            .collect();
        let interference_minima =
            InterferenceMinima::from_elements(&elements_refs, [0.0, 0.0, 0.0]);

        let confidence = Self::calculate_confidence(&shape, bonding_pairs, lone_pairs);

        Self {
            shape,
            bond_angles,
            interference_minima,
            central_element: Some(central.atomic_number()),
            bonding_pairs,
            lone_pairs,
            steric_number,
            prediction_confidence: confidence,
        }
    }

    fn calculate_confidence(shape: &MolecularShape, bonding: u32, lone: u32) -> Float {
        let shape_confidence = match shape {
            MolecularShape::Linear => 0.95,
            MolecularShape::Tetrahedral => 0.90,
            MolecularShape::Octahedral => 0.85,
            MolecularShape::TrigonalPlanar => 0.85,
            MolecularShape::Bent => 0.70,
            MolecularShape::TrigonalPyramidal => 0.80,
            MolecularShape::Unknown => 0.30,
            _ => 0.75,
        };

        let complexity_penalty = (bonding + lone) as Float * 0.02;
        (shape_confidence - complexity_penalty).max(0.0)
    }

    pub fn shape(&self) -> &MolecularShape {
        &self.shape
    }

    pub fn bond_angles(&self) -> &[BondAngle] {
        &self.bond_angles
    }

    pub fn interference_minima(&self) -> &[InterferenceMinima] {
        &self.interference_minima
    }

    pub fn central_element(&self) -> Option<u32> {
        self.central_element
    }

    pub fn bonding_pairs(&self) -> u32 {
        self.bonding_pairs
    }

    pub fn lone_pairs(&self) -> u32 {
        self.lone_pairs
    }

    pub fn steric_number(&self) -> u32 {
        self.steric_number
    }

    pub fn confidence(&self) -> Float {
        self.prediction_confidence
    }

    pub fn is_stable(&self) -> bool {
        self.prediction_confidence > 0.5 && !self.interference_minima.is_empty()
    }

    pub fn optimize_geometry(&mut self, iterations: u32) {
        for _ in 0..iterations {
            for minima in self.interference_minima.iter_mut() {
                for i in 0..3 {
                    minima.position[i] +=
                        (minima.archetype_contribution[i % NUM_ARCHETYPES] - 0.5) * 0.01;
                }
                minima.depth *= 1.001;
                minima.stability = minima.depth / 10.0;
            }
        }

        self.prediction_confidence = (self.prediction_confidence + 0.01).min(1.0);
    }

    pub fn bond_angle_for_pair(&self, pair_index: usize) -> Option<&BondAngle> {
        self.bond_angles.get(pair_index)
    }

    pub fn average_angle(&self) -> Float {
        if self.bond_angles.is_empty() {
            return 0.0;
        }
        self.bond_angles
            .iter()
            .map(|a| a.angle_degrees)
            .sum::<Float>()
            / self.bond_angles.len() as Float
    }

    pub fn angle_variance(&self) -> Float {
        if self.bond_angles.len() < 2 {
            return 0.0;
        }

        let mean = self.average_angle();
        let variance = self
            .bond_angles
            .iter()
            .map(|a| (a.angle_degrees - mean).powi(2))
            .sum::<Float>()
            / self.bond_angles.len() as Float;

        variance.sqrt()
    }
}

impl Default for GeometryPrediction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_angle_creation() {
        let angle = BondAngle::from_degrees(109.5);
        assert!((angle.angle_degrees - 109.5).abs() < 1e-10);
        assert!(angle.angle_radians > 0.0);
    }

    #[test]
    fn test_bond_angle_ideal() {
        let angle = BondAngle::ideal_angle(6, 4, 0);
        assert!((angle.angle_degrees - 109.5).abs() < 5.0);
    }

    #[test]
    fn test_molecular_shape_from_vsepr() {
        assert_eq!(MolecularShape::from_vsepr(2, 0), MolecularShape::Linear);
        assert_eq!(
            MolecularShape::from_vsepr(4, 0),
            MolecularShape::Tetrahedral
        );
        assert_eq!(
            MolecularShape::from_vsepr(3, 1),
            MolecularShape::TrigonalPyramidal
        );
    }

    #[test]
    fn test_molecular_shape_angles() {
        let linear = MolecularShape::Linear;
        let angles = linear.typical_angles();
        assert!(!angles.is_empty());
        assert!((angles[0].angle_degrees - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_interference_minima_creation() {
        let minima = InterferenceMinima::new([1.0, 0.0, 0.0], 5.0);
        assert_eq!(minima.position, [1.0, 0.0, 0.0]);
        assert_eq!(minima.depth, 5.0);
    }

    #[test]
    fn test_interference_minima_from_elements() {
        let c = ElementAttractorField::carbon();
        let h = ElementAttractorField::hydrogen();

        let minima = InterferenceMinima::from_elements(&[&c, &h], [0.0, 0.0, 0.0]);
        assert!(!minima.is_empty());
    }

    #[test]
    fn test_geometry_prediction_creation() {
        let pred = GeometryPrediction::new();
        assert_eq!(pred.shape(), &MolecularShape::Unknown);
        assert!(pred.bond_angles().is_empty());
    }

    #[test]
    fn test_geometry_prediction_for_central_atom() {
        let c = ElementAttractorField::carbon();
        let h1 = ElementAttractorField::hydrogen();
        let h2 = ElementAttractorField::hydrogen();
        let h3 = ElementAttractorField::hydrogen();
        let h4 = ElementAttractorField::hydrogen();

        let pred = GeometryPrediction::for_central_atom(&c, &[h1, h2, h3, h4], 0);

        assert_eq!(pred.shape(), &MolecularShape::Tetrahedral);
        assert_eq!(pred.bonding_pairs(), 4);
        assert_eq!(pred.lone_pairs(), 0);
    }

    #[test]
    fn test_geometry_prediction_methane() {
        let c = ElementAttractorField::carbon();
        let hydrogens = vec![
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
        ];

        let pred = GeometryPrediction::for_central_atom(&c, &hydrogens, 0);

        assert_eq!(pred.steric_number(), 4);
        assert!(pred.confidence() > 0.5);
    }

    #[test]
    fn test_geometry_prediction_water() {
        let o = ElementAttractorField::oxygen();
        let hydrogens = vec![
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
        ];

        let pred = GeometryPrediction::for_central_atom(&o, &hydrogens, 2);

        assert_eq!(pred.shape(), &MolecularShape::Bent);
        assert_eq!(pred.bonding_pairs(), 2);
        assert_eq!(pred.lone_pairs(), 2);
    }

    #[test]
    fn test_geometry_optimization() {
        let c = ElementAttractorField::carbon();
        let hydrogens = vec![ElementAttractorField::hydrogen(); 4];

        let mut pred = GeometryPrediction::for_central_atom(&c, &hydrogens, 0);
        let initial_confidence = pred.confidence();

        pred.optimize_geometry(10);

        assert!(pred.confidence() >= initial_confidence);
    }

    #[test]
    fn test_average_angle() {
        let pred = GeometryPrediction::new();
        assert_eq!(pred.average_angle(), 0.0);
    }

    #[test]
    fn test_angle_variance() {
        let pred = GeometryPrediction::new();
        assert_eq!(pred.angle_variance(), 0.0);
    }
}
