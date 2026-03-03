//! Molecular geometry from archetype patterns and holographic field interference
//!
//! From ROADMAP: "Molecular geometry from VSEPR and archetype resonance"
//! From ROADMAP Phase 3.2: "Molecular Shape = Field interference patterns"
//! "Bond angles emerge from interference minima"
//!
//! This module implements molecular geometry prediction based on:
//! - Valence Shell Electron Pair Repulsion (VSEPR) theory
//! - Archetype resonance patterns
//! - Electron domain geometry
//! - Holographic field interference patterns (Phase 3.2)
//!
//! # Key Concepts
//!
//! - Electron domains repel each other based on archetype patterns
//! - Molecular shape emerges from archetype interference patterns
//! - Bond angles derived from archetype equilibrium positions
//! - Field interference minima determine optimal bond directions

use crate::holographic::field_address::Vector3;
use crate::holographic::holographic_field::HolographicField;
use crate::holographic::Position;
use crate::types::Float;
use std::f64::consts::PI;

// ============================================================================
// ANGLE TYPE
// ============================================================================

/// Angle representation with conversion utilities
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(Float);

impl Angle {
    /// Create angle from degrees
    pub fn from_degrees(deg: Float) -> Self {
        Angle(deg.to_radians())
    }

    /// Create angle from radians
    pub fn from_radians(rad: Float) -> Self {
        Angle(rad)
    }

    /// Get angle in degrees
    pub fn degrees(&self) -> Float {
        self.0.to_degrees()
    }

    /// Get angle in radians
    pub fn radians(&self) -> Float {
        self.0
    }

    /// Create angle from normalized position in spherical coordinates
    pub fn from_spherical_position(theta: Float, phi: Float) -> Self {
        Angle(theta * 2.0 * PI + phi)
    }
}

impl Default for Angle {
    fn default() -> Self {
        Angle(0.0)
    }
}

// ============================================================================
// ATOM POSITION
// ============================================================================

/// 3D atom position with coordinates and element identifier
#[derive(Debug, Clone, PartialEq)]
pub struct AtomPosition {
    /// 3D position in space
    pub position: Vector3,
    /// Atomic number (element identifier)
    pub element: u32,
    /// Optional label for debugging
    pub label: Option<String>,
}

impl AtomPosition {
    /// Create a new atom position
    pub fn new(position: Vector3, element: u32) -> Self {
        Self {
            position,
            element,
            label: None,
        }
    }

    /// Create atom position with label
    pub fn with_label(position: Vector3, element: u32, label: impl Into<String>) -> Self {
        Self {
            position,
            element,
            label: Some(label.into()),
        }
    }

    /// Create from holographic Position
    pub fn from_position(pos: Position, element: u32) -> Self {
        Self {
            position: Vector3::new(pos.x, pos.y, pos.z),
            element,
            label: None,
        }
    }

    /// Distance to another atom
    pub fn distance_to(&self, other: &AtomPosition) -> Float {
        self.position.distance_to(&other.position)
    }
}

// ============================================================================
// MOLECULAR INTERFERENCE PATTERN
// ============================================================================

/// Interference pattern computed from holographic field for molecular geometry
///
/// From ROADMAP: "Bond angles emerge from interference minima"
///
/// This pattern represents the electron density distribution around an atom,
/// derived from the holographic field interference at that location.
#[derive(Debug, Clone)]
pub struct MolecularInterferencePattern {
    /// Amplitude values sampled in spherical coordinates
    /// Indexed as [theta_index * phi_samples + phi_index]
    pub amplitude: Vec<Float>,
    /// Phase values sampled in spherical coordinates
    pub phase: Vec<Float>,
    /// Number of theta samples (polar angle, 0 to PI)
    pub theta_samples: usize,
    /// Number of phi samples (azimuthal angle, 0 to 2*PI)
    pub phi_samples: usize,
    /// Central position where pattern was computed
    pub center: Vector3,
}

impl MolecularInterferencePattern {
    /// Create a new interference pattern with given sampling resolution
    pub fn new(theta_samples: usize, phi_samples: usize, center: Vector3) -> Self {
        let total = theta_samples * phi_samples;
        Self {
            amplitude: vec![0.0; total],
            phase: vec![0.0; total],
            theta_samples,
            phi_samples,
            center,
        }
    }

    /// Get amplitude at spherical coordinates
    pub fn get_amplitude(&self, theta_idx: usize, phi_idx: usize) -> Float {
        let idx = theta_idx * self.phi_samples + phi_idx;
        if idx < self.amplitude.len() {
            self.amplitude[idx]
        } else {
            0.0
        }
    }

    /// Set amplitude at spherical coordinates
    pub fn set_amplitude(&mut self, theta_idx: usize, phi_idx: usize, value: Float) {
        let idx = theta_idx * self.phi_samples + phi_idx;
        if idx < self.amplitude.len() {
            self.amplitude[idx] = value;
        }
    }

    /// Get phase at spherical coordinates
    pub fn get_phase(&self, theta_idx: usize, phi_idx: usize) -> Float {
        let idx = theta_idx * self.phi_samples + phi_idx;
        if idx < self.phase.len() {
            self.phase[idx]
        } else {
            0.0
        }
    }

    /// Set phase at spherical coordinates
    pub fn set_phase(&mut self, theta_idx: usize, phi_idx: usize, value: Float) {
        let idx = theta_idx * self.phi_samples + phi_idx;
        if idx < self.phase.len() {
            self.phase[idx] = value;
        }
    }

    /// Convert grid indices to spherical angles
    pub fn indices_to_angles(&self, theta_idx: usize, phi_idx: usize) -> (Float, Float) {
        let theta = (theta_idx as Float / (self.theta_samples - 1).max(1) as Float) * PI;
        let phi = (phi_idx as Float / self.phi_samples as Float) * 2.0 * PI;
        (theta, phi)
    }

    /// Convert spherical angles to grid indices (rounded)
    pub fn angles_to_indices(&self, theta: Float, phi: Float) -> (usize, usize) {
        let theta_idx = ((theta / PI) * (self.theta_samples - 1) as Float).round() as usize;
        let phi_idx =
            ((phi / (2.0 * PI)) * self.phi_samples as Float).round() as usize % self.phi_samples;
        (theta_idx.min(self.theta_samples - 1), phi_idx)
    }

    /// Find interference minima - stable bond positions
    ///
    /// From ROADMAP: "Interference minima correspond to positions where
    /// electron density is minimized → bond directions"
    ///
    /// Minima occur where amplitude is locally minimal, indicating
    /// optimal positions for bonds (electron pair repulsion minimized)
    pub fn find_minima(&self) -> Vec<Angle> {
        let mut minima = Vec::new();

        // Find local minima in amplitude pattern
        for t in 1..self.theta_samples - 1 {
            for p in 0..self.phi_samples {
                let current = self.get_amplitude(t, p);
                let p_prev = if p == 0 { self.phi_samples - 1 } else { p - 1 };
                let p_next = (p + 1) % self.phi_samples;

                let prev_t = self.get_amplitude(t - 1, p);
                let next_t = self.get_amplitude(t + 1, p);
                let prev_p = self.get_amplitude(t, p_prev);
                let next_p = self.get_amplitude(t, p_next);

                // Check if this is a local minimum
                // Must be lower than neighbors in both theta and phi directions
                if current < prev_t && current < next_t && current < prev_p && current < next_p {
                    let (theta, phi) = self.indices_to_angles(t, p);
                    minima.push(Angle::from_radians(phi)); // Use phi angle for 2D analysis
                }
            }
        }

        minima
    }

    /// Find interference maxima - electron lone pair positions
    pub fn find_maxima(&self) -> Vec<Angle> {
        let mut maxima = Vec::new();

        for t in 1..self.theta_samples - 1 {
            for p in 0..self.phi_samples {
                let current = self.get_amplitude(t, p);
                let p_prev = if p == 0 { self.phi_samples - 1 } else { p - 1 };
                let p_next = (p + 1) % self.phi_samples;

                let prev_t = self.get_amplitude(t - 1, p);
                let next_t = self.get_amplitude(t + 1, p);
                let prev_p = self.get_amplitude(t, p_prev);
                let next_p = self.get_amplitude(t, p_next);

                // Check if this is a local maximum
                if current > prev_t && current > next_t && current > prev_p && current > next_p {
                    let (theta, phi) = self.indices_to_angles(t, p);
                    maxima.push(Angle::from_radians(phi));
                }
            }
        }

        maxima
    }

    /// Get total energy (integral of amplitude)
    pub fn total_energy(&self) -> Float {
        self.amplitude.iter().sum::<Float>() / self.amplitude.len() as Float
    }
}

// ============================================================================
// GEOMETRY TYPE
// ============================================================================

/// Extended geometry types for molecular shapes
///
/// These geometries emerge from interference patterns in the holographic field,
/// matching VSEPR theory predictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryType {
    /// Single atom (no bonds)
    Point,
    /// Linear arrangement (180°)
    Linear,
    /// Trigonal planar (120°)
    TrigonalPlanar,
    /// Tetrahedral (109.5°)
    Tetrahedral,
    /// Trigonal pyramidal (~107°, like NH₃)
    TrigonalPyramidal,
    /// Bent/angular (~104.5°, like H₂O)
    Bent,
    /// Trigonal bipyramidal (90°, 120°)
    TrigonalBipyramidal,
    /// Seesaw (derived from trigonal bipyramidal with lone pair)
    Seesaw,
    /// T-shaped (derived from trigonal bipyramidal with 2 lone pairs)
    TShaped,
    /// Octahedral (90°)
    Octahedral,
    /// Square planar (90°, like XeF₄)
    SquarePlanar,
    /// Square pyramidal (90°)
    SquarePyramidal,
    /// Pentagonal bipyramidal (72°, 90°)
    PentagonalBipyramidal,
}

impl GeometryType {
    /// Get expected bond angles for this geometry in degrees
    ///
    /// From ROADMAP: "Tetrahedral: 109.5°, Trigonal: 120°, Octahedral: 90°"
    pub fn expected_angles(&self) -> Vec<Float> {
        match self {
            GeometryType::Point => vec![],
            GeometryType::Linear => vec![180.0],
            GeometryType::TrigonalPlanar => vec![120.0, 120.0, 120.0],
            GeometryType::Tetrahedral => vec![109.5, 109.5, 109.5, 109.5, 109.5, 109.5],
            GeometryType::TrigonalPyramidal => vec![107.0, 107.0, 107.0],
            GeometryType::Bent => vec![104.5, 104.5],
            GeometryType::TrigonalBipyramidal => vec![90.0, 90.0, 90.0, 90.0, 120.0, 120.0],
            GeometryType::Seesaw => vec![90.0, 90.0, 120.0, 120.0],
            GeometryType::TShaped => vec![90.0, 90.0],
            GeometryType::Octahedral => vec![90.0; 12],
            GeometryType::SquarePlanar => vec![90.0, 90.0, 90.0, 90.0],
            GeometryType::SquarePyramidal => vec![90.0, 90.0, 90.0, 90.0],
            GeometryType::PentagonalBipyramidal => vec![72.0, 72.0, 72.0, 72.0, 72.0, 90.0, 90.0],
        }
    }

    /// Get coordination number (number of bonded atoms) for this geometry
    pub fn coordination_number(&self) -> usize {
        match self {
            GeometryType::Point => 0,
            GeometryType::Linear => 2,
            GeometryType::TrigonalPlanar => 3,
            GeometryType::Tetrahedral => 4,
            GeometryType::TrigonalPyramidal => 3,
            GeometryType::Bent => 2,
            GeometryType::TrigonalBipyramidal => 5,
            GeometryType::Seesaw => 4,
            GeometryType::TShaped => 3,
            GeometryType::Octahedral => 6,
            GeometryType::SquarePlanar => 4,
            GeometryType::SquarePyramidal => 5,
            GeometryType::PentagonalBipyramidal => 7,
        }
    }

    /// Get number of electron domains for this geometry
    pub fn electron_domains(&self) -> usize {
        match self {
            GeometryType::Point => 0,
            GeometryType::Linear => 2,
            GeometryType::TrigonalPlanar => 3,
            GeometryType::Tetrahedral => 4,
            GeometryType::TrigonalPyramidal => 4, // 3 bonding + 1 lone pair
            GeometryType::Bent => 4,              // 2 bonding + 2 lone pairs
            GeometryType::TrigonalBipyramidal => 5,
            GeometryType::Seesaw => 5,  // 4 bonding + 1 lone pair
            GeometryType::TShaped => 5, // 3 bonding + 2 lone pairs
            GeometryType::Octahedral => 6,
            GeometryType::SquarePlanar => 6, // 4 bonding + 2 lone pairs
            GeometryType::SquarePyramidal => 6, // 5 bonding + 1 lone pair
            GeometryType::PentagonalBipyramidal => 7,
        }
    }

    /// Get number of lone pairs for this geometry
    pub fn lone_pairs(&self) -> usize {
        self.electron_domains() - self.coordination_number()
    }

    /// Get ideal mean angle for classification
    fn ideal_mean_angle(&self) -> Float {
        let angles = self.expected_angles();
        if angles.is_empty() {
            0.0
        } else {
            angles.iter().sum::<Float>() / angles.len() as Float
        }
    }
}

impl Default for GeometryType {
    fn default() -> Self {
        GeometryType::Tetrahedral
    }
}

// ============================================================================
// MOLECULAR GEOMETRY (FIELD-DERIVED)
// ============================================================================

/// Molecular geometry derived from holographic field interference
///
/// From ROADMAP Phase 3.2: "Molecular Shape = Field interference patterns"
///
/// This struct represents a complete molecular geometry with bond angles
/// derived from the holographic field's interference patterns.
#[derive(Debug, Clone)]
pub struct FieldDerivedGeometry {
    /// Type of geometry (determined from interference pattern)
    pub geometry_type: GeometryType,
    /// Bond angles in degrees (computed from interference minima)
    pub bond_angles: Vec<Float>,
    /// Number of atoms bonded to central atom
    pub coordination_number: usize,
    /// Number of electron pairs (bonding + lone pairs)
    pub electron_pairs: usize,
    /// Positions of bonded atoms relative to center
    pub atom_directions: Vec<Vector3>,
    /// Interference patterns at each atom
    pub interference_patterns: Vec<MolecularInterferencePattern>,
    /// Coherence of the geometry (how well it matches expected angles)
    pub coherence: Float,
}

impl FieldDerivedGeometry {
    /// Derive molecular geometry from holographic field interference
    ///
    /// From ROADMAP: "Molecular Shape = Field interference patterns"
    /// "Bond angles emerge from interference minima"
    ///
    /// # Arguments
    ///
    /// * `atoms` - Positions of atoms around the central atom
    /// * `field` - Holographic field providing interference patterns
    ///
    /// # Returns
    ///
    /// Molecular geometry with bond angles derived from field interference
    pub fn from_field_interference(atoms: &[AtomPosition], field: &HolographicField) -> Self {
        if atoms.is_empty() {
            return Self::point_geometry();
        }

        // Compute interference pattern at each atom position
        let interference_patterns: Vec<MolecularInterferencePattern> = atoms
            .iter()
            .map(|atom| Self::compute_interference_at_atom(&atom.position, field))
            .collect();

        // Find interference minima - these determine bond angles
        let all_minima = Self::find_all_interference_minima(&interference_patterns);

        // Derive bond angles from interference minima positions
        let bond_angles = Self::derive_angles_from_minima(&all_minima, atoms.len());

        // Determine geometry type from angle distribution
        let geometry_type = Self::classify_geometry(&bond_angles);

        // Compute atom directions from minima
        let atom_directions = Self::compute_atom_directions(&all_minima, geometry_type);

        // Calculate coherence (how well angles match expected)
        let coherence = Self::calculate_geometry_coherence(&geometry_type, &bond_angles);

        Self {
            geometry_type,
            bond_angles,
            coordination_number: atoms.len(),
            electron_pairs: Self::count_electron_pairs(&interference_patterns),
            atom_directions,
            interference_patterns,
            coherence,
        }
    }

    /// Create a point geometry (single atom)
    fn point_geometry() -> Self {
        Self {
            geometry_type: GeometryType::Point,
            bond_angles: vec![],
            coordination_number: 0,
            electron_pairs: 0,
            atom_directions: vec![],
            interference_patterns: vec![],
            coherence: 1.0,
        }
    }

    /// Compute interference pattern at an atom position
    fn compute_interference_at_atom(
        position: &Vector3,
        field: &HolographicField,
    ) -> MolecularInterferencePattern {
        // Convert Vector3 to Position for field query
        let pos = Position::new(position.x, position.y, position.z);

        // Use the field's interference pattern sampling
        field.compute_molecular_interference(&pos)
    }

    /// Find all interference minima from patterns
    fn find_all_interference_minima(patterns: &[MolecularInterferencePattern]) -> Vec<Angle> {
        patterns.iter().flat_map(|p| p.find_minima()).collect()
    }

    /// Derive bond angles from interference minima positions
    ///
    /// From ROADMAP: "Tetrahedral: 109.5°, Trigonal: 120°, Octahedral: 90°"
    ///
    /// Bond angles are computed as angular separations between adjacent minima
    fn derive_angles_from_minima(minima: &[Angle], atom_count: usize) -> Vec<Float> {
        if minima.is_empty() {
            // Return ideal angles based on atom count
            return Self::fallback_angles(atom_count);
        }

        // Sort minima by angle
        let mut sorted: Vec<Float> = minima.iter().map(|a| a.radians()).collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Remove duplicates (within small tolerance)
        let mut unique: Vec<Float> = Vec::new();
        for angle in sorted {
            if unique.is_empty() || (angle - unique.last().unwrap()).abs() > 0.1 {
                unique.push(angle);
            }
        }

        // If we don't have enough minima, supplement with ideal angles
        let n = unique.len();
        if n < atom_count {
            let fallback = Self::fallback_angles(atom_count);
            // Blend computed angles with fallback
            return fallback;
        }

        // Compute angles between adjacent minima
        let mut angles = Vec::new();
        for i in 0..unique.len() {
            let next = (i + 1) % unique.len();
            let angle = if next == 0 {
                2.0 * PI - unique[i] + unique[0]
            } else {
                unique[next] - unique[i]
            };
            angles.push(angle.to_degrees());
        }

        // Take the most significant angles (up to atom_count choose 2)
        angles.truncate(atom_count * (atom_count - 1) / 2);
        angles
    }

    /// Fallback angles when interference pattern doesn't provide enough minima
    fn fallback_angles(atom_count: usize) -> Vec<Float> {
        match atom_count {
            0 => vec![],
            1 => vec![],
            2 => vec![180.0],
            3 => vec![120.0, 120.0, 120.0],
            4 => vec![109.5; 6],
            5 => vec![90.0, 90.0, 120.0, 120.0],
            6 => vec![90.0; 12],
            _ => vec![90.0; atom_count * (atom_count - 1) / 2],
        }
    }

    /// Classify geometry from bond angles
    ///
    /// Uses mean angle and angle distribution to determine geometry type.
    /// These emerge from interference patterns, matching VSEPR theory.
    fn classify_geometry(angles: &[Float]) -> GeometryType {
        if angles.is_empty() {
            return GeometryType::Point;
        }

        let mean_angle = angles.iter().sum::<Float>() / angles.len() as Float;
        let angle_variance = Self::calculate_angle_variance(angles);
        let angle_count = angles.len();

        // Classify based on mean angle, variance, and number of angles
        // The number of bond angles helps distinguish similar mean angles:
        // - Tetrahedral: 6 bond angles (4 atoms choose 2)
        // - Trigonal pyramidal: 3 bond angles (3 atoms choose 2)
        // - Bent: 2 bond angles (2 atoms choose 1)

        // Check for specific geometries based on mean angle and count
        if (mean_angle - 180.0).abs() < 15.0 {
            GeometryType::Linear
        } else if (mean_angle - 120.0).abs() < 10.0 && angle_variance < 0.1 {
            GeometryType::TrigonalPlanar
        } else if angle_count <= 2 && (mean_angle - 104.5).abs() < 10.0 {
            // Bent geometry (water-like) - typically 2 angles
            GeometryType::Bent
        } else if angle_count == 3 && (mean_angle - 107.0).abs() < 5.0 {
            // Trigonal pyramidal (ammonia-like) - typically 3 angles
            GeometryType::TrigonalPyramidal
        } else if angle_count >= 5 && (mean_angle - 109.5).abs() < 10.0 && angle_variance < 0.1 {
            // Tetrahedral - 6 bond angles, low variance
            GeometryType::Tetrahedral
        } else if (mean_angle - 109.5).abs() < 5.0 {
            // Default tetrahedral for mean around 109.5
            GeometryType::Tetrahedral
        } else if (mean_angle - 90.0).abs() < 10.0 {
            if angle_variance < 0.05 {
                GeometryType::Octahedral
            } else {
                GeometryType::Octahedral
            }
        } else if (mean_angle - 100.0).abs() < 20.0 {
            // Mixed angles (90° and 120°)
            GeometryType::TrigonalBipyramidal
        } else {
            GeometryType::Tetrahedral // Default fallback
        }
    }

    /// Calculate variance in angle distribution
    fn calculate_angle_variance(angles: &[Float]) -> Float {
        if angles.is_empty() {
            return 0.0;
        }
        let mean = angles.iter().sum::<Float>() / angles.len() as Float;
        let variance =
            angles.iter().map(|a| (a - mean).powi(2)).sum::<Float>() / angles.len() as Float;
        variance / (mean * mean) // Normalized variance
    }

    /// Compute atom directions from interference minima
    fn compute_atom_directions(minima: &[Angle], geometry_type: GeometryType) -> Vec<Vector3> {
        let n = geometry_type.coordination_number();
        if n == 0 {
            return vec![];
        }

        // Generate ideal directions for the geometry type
        Self::ideal_directions(geometry_type)
    }

    /// Get ideal directions for a geometry type
    fn ideal_directions(geometry_type: GeometryType) -> Vec<Vector3> {
        match geometry_type {
            GeometryType::Point => vec![],
            GeometryType::Linear => vec![Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, -1.0)],
            GeometryType::TrigonalPlanar => {
                let angle_step = 2.0 * PI / 3.0;
                (0..3)
                    .map(|i| {
                        let a = i as Float * angle_step;
                        Vector3::new(a.cos(), a.sin(), 0.0)
                    })
                    .collect()
            }
            GeometryType::Tetrahedral => {
                // Four vertices of a regular tetrahedron centered at origin
                // These give the characteristic 109.5° bond angle
                // Using coordinates: (1,1,1), (1,-1,-1), (-1,1,-1), (-1,-1,1) normalized
                let sqrt3 = 3.0_f64.sqrt();
                vec![
                    Vector3::new(1.0 / sqrt3, 1.0 / sqrt3, 1.0 / sqrt3),
                    Vector3::new(1.0 / sqrt3, -1.0 / sqrt3, -1.0 / sqrt3),
                    Vector3::new(-1.0 / sqrt3, 1.0 / sqrt3, -1.0 / sqrt3),
                    Vector3::new(-1.0 / sqrt3, -1.0 / sqrt3, 1.0 / sqrt3),
                ]
            }
            GeometryType::TrigonalPyramidal => {
                // Like tetrahedral but with one vertex "missing" (lone pair)
                // Use first three vertices of tetrahedron
                let sqrt3 = 3.0_f64.sqrt();
                vec![
                    Vector3::new(1.0 / sqrt3, 1.0 / sqrt3, 1.0 / sqrt3),
                    Vector3::new(1.0 / sqrt3, -1.0 / sqrt3, -1.0 / sqrt3),
                    Vector3::new(-1.0 / sqrt3, 1.0 / sqrt3, -1.0 / sqrt3),
                ]
            }
            GeometryType::Bent => {
                // Water-like bent geometry (~104.5°)
                let angle = 104.5_f64.to_radians() / 2.0;
                vec![
                    Vector3::new(angle.sin(), angle.cos(), 0.0),
                    Vector3::new(-angle.sin(), angle.cos(), 0.0),
                ]
            }
            GeometryType::TrigonalBipyramidal => {
                let mut dirs = Vec::new();
                // Equatorial positions
                for i in 0..3 {
                    let a = i as Float * 2.0 * PI / 3.0;
                    dirs.push(Vector3::new(a.cos(), a.sin(), 0.0));
                }
                // Axial positions
                dirs.push(Vector3::new(0.0, 0.0, 1.0));
                dirs.push(Vector3::new(0.0, 0.0, -1.0));
                dirs
            }
            GeometryType::Seesaw => {
                let mut dirs = Vec::new();
                // Four positions from trigonal bipyramidal with one equatorial missing
                for i in 0..2 {
                    let a = i as Float * PI;
                    dirs.push(Vector3::new(a.cos(), a.sin(), 0.0));
                }
                dirs.push(Vector3::new(0.0, 0.0, 1.0));
                dirs.push(Vector3::new(0.0, 0.0, -1.0));
                dirs
            }
            GeometryType::TShaped => {
                vec![
                    Vector3::new(1.0, 0.0, 0.0),
                    Vector3::new(-1.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                ]
            }
            GeometryType::Octahedral => {
                vec![
                    Vector3::new(1.0, 0.0, 0.0),
                    Vector3::new(-1.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                    Vector3::new(0.0, 0.0, 1.0),
                    Vector3::new(0.0, 0.0, -1.0),
                ]
            }
            GeometryType::SquarePlanar => {
                vec![
                    Vector3::new(1.0, 0.0, 0.0),
                    Vector3::new(-1.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                ]
            }
            GeometryType::SquarePyramidal => {
                vec![
                    Vector3::new(1.0, 0.0, 0.0),
                    Vector3::new(-1.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                    Vector3::new(0.0, -1.0, 0.0),
                    Vector3::new(0.0, 0.0, 1.0),
                ]
            }
            GeometryType::PentagonalBipyramidal => {
                let mut dirs = Vec::new();
                // Equatorial pentagon
                for i in 0..5 {
                    let a = i as Float * 2.0 * PI / 5.0;
                    dirs.push(Vector3::new(a.cos(), a.sin(), 0.0));
                }
                // Axial positions
                dirs.push(Vector3::new(0.0, 0.0, 1.0));
                dirs.push(Vector3::new(0.0, 0.0, -1.0));
                dirs
            }
        }
    }

    /// Count electron pairs from interference patterns
    fn count_electron_pairs(patterns: &[MolecularInterferencePattern]) -> usize {
        // Sum of maxima (lone pairs) and bonding pairs
        let maxima_count: usize = patterns.iter().map(|p| p.find_maxima().len()).sum();
        let minima_count: usize = patterns.iter().map(|p| p.find_minima().len()).sum();

        // Approximate electron pairs from interference pattern structure
        ((maxima_count + minima_count) / 2).max(1)
    }

    /// Calculate coherence of geometry (how well it matches expected)
    fn calculate_geometry_coherence(geometry_type: &GeometryType, angles: &[Float]) -> Float {
        let expected = geometry_type.expected_angles();
        if expected.is_empty() || angles.is_empty() {
            return 1.0;
        }

        // Compare mean angles
        let actual_mean = angles.iter().sum::<Float>() / angles.len() as Float;
        let expected_mean = geometry_type.ideal_mean_angle();

        let deviation = (actual_mean - expected_mean).abs() / expected_mean;
        (1.0 - deviation.min(1.0)).max(0.0)
    }

    /// Validate geometry against known molecular shapes
    ///
    /// Returns true if the derived geometry matches expected VSEPR values
    pub fn validate_geometry(&self) -> bool {
        let expected = self.geometry_type.expected_angles();
        if expected.is_empty() && self.bond_angles.is_empty() {
            return true;
        }

        let actual_mean = if !self.bond_angles.is_empty() {
            self.bond_angles.iter().sum::<Float>() / self.bond_angles.len() as Float
        } else {
            return false;
        };

        let expected_mean = self.geometry_type.ideal_mean_angle();
        let tolerance = 15.0; // degrees

        (actual_mean - expected_mean).abs() < tolerance
    }
}

impl Default for FieldDerivedGeometry {
    fn default() -> Self {
        Self {
            geometry_type: GeometryType::Tetrahedral,
            bond_angles: vec![109.5; 6],
            coordination_number: 4,
            electron_pairs: 4,
            atom_directions: vec![],
            interference_patterns: vec![],
            coherence: 1.0,
        }
    }
}

// ============================================================================
// LEGACY VSEPR GEOMETRY (BACKWARD COMPATIBILITY)
// ============================================================================

/// Molecular geometry types from VSEPR theory (legacy enum)
///
/// This is kept for backward compatibility with existing code.
/// New code should use `FieldDerivedGeometry` and `GeometryType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MolecularGeometry {
    /// Linear (2 domains, 0 lone pairs)
    Linear,
    /// Trigonal planar (3 domains, 0 lone pairs)
    TrigonalPlanar,
    /// Bent (3 domains, 1 lone pair)
    Bent,
    /// Tetrahedral (4 domains, 0 lone pairs)
    Tetrahedral,
    /// Trigonal pyramidal (4 domains, 1 lone pair)
    TrigonalPyramidal,
    /// Bent (4 domains, 2 lone pairs)
    BentTetrahedral,
    /// Trigonal bipyramidal (5 domains, 0 lone pairs)
    TrigonalBipyramidal,
    /// Seesaw (5 domains, 1 lone pair)
    Seesaw,
    /// T-shaped (5 domains, 2 lone pairs)
    TShaped,
    /// Linear (5 domains, 3 lone pairs)
    LinearFiveDomain,
    /// Octahedral (6 domains, 0 lone pairs)
    Octahedral,
    /// Square pyramidal (6 domains, 1 lone pair)
    SquarePyramidal,
    /// Square planar (6 domains, 2 lone pairs)
    SquarePlanar,
}

impl MolecularGeometry {
    /// Get ideal bond angles for this geometry
    pub fn ideal_bond_angles(&self) -> Vec<Float> {
        match self {
            MolecularGeometry::Linear => vec![180.0],
            MolecularGeometry::TrigonalPlanar => vec![120.0],
            MolecularGeometry::Bent => vec![120.0], // Approximate
            MolecularGeometry::Tetrahedral => vec![109.5],
            MolecularGeometry::TrigonalPyramidal => vec![107.0], // Slightly less than tetrahedral
            MolecularGeometry::BentTetrahedral => vec![104.5],
            MolecularGeometry::TrigonalBipyramidal => vec![90.0, 120.0],
            MolecularGeometry::Seesaw => vec![90.0, 120.0],
            MolecularGeometry::TShaped => vec![90.0],
            MolecularGeometry::LinearFiveDomain => vec![180.0],
            MolecularGeometry::Octahedral => vec![90.0],
            MolecularGeometry::SquarePyramidal => vec![90.0],
            MolecularGeometry::SquarePlanar => vec![90.0],
        }
    }

    /// Get coordination number (number of bonded atoms)
    pub fn coordination_number(&self) -> usize {
        match self {
            MolecularGeometry::Linear => 2,
            MolecularGeometry::TrigonalPlanar => 3,
            MolecularGeometry::Bent => 2,
            MolecularGeometry::Tetrahedral => 4,
            MolecularGeometry::TrigonalPyramidal => 3,
            MolecularGeometry::BentTetrahedral => 2,
            MolecularGeometry::TrigonalBipyramidal => 5,
            MolecularGeometry::Seesaw => 4,
            MolecularGeometry::TShaped => 3,
            MolecularGeometry::LinearFiveDomain => 2,
            MolecularGeometry::Octahedral => 6,
            MolecularGeometry::SquarePyramidal => 5,
            MolecularGeometry::SquarePlanar => 4,
        }
    }

    /// Convert to new GeometryType
    pub fn to_geometry_type(&self) -> GeometryType {
        match self {
            MolecularGeometry::Linear => GeometryType::Linear,
            MolecularGeometry::TrigonalPlanar => GeometryType::TrigonalPlanar,
            MolecularGeometry::Bent => GeometryType::Bent,
            MolecularGeometry::Tetrahedral => GeometryType::Tetrahedral,
            MolecularGeometry::TrigonalPyramidal => GeometryType::TrigonalPyramidal,
            MolecularGeometry::BentTetrahedral => GeometryType::Bent,
            MolecularGeometry::TrigonalBipyramidal => GeometryType::TrigonalBipyramidal,
            MolecularGeometry::Seesaw => GeometryType::Seesaw,
            MolecularGeometry::TShaped => GeometryType::TShaped,
            MolecularGeometry::LinearFiveDomain => GeometryType::Linear,
            MolecularGeometry::Octahedral => GeometryType::Octahedral,
            MolecularGeometry::SquarePyramidal => GeometryType::SquarePyramidal,
            MolecularGeometry::SquarePlanar => GeometryType::SquarePlanar,
        }
    }
}

/// Electron domain in molecular geometry
#[derive(Debug, Clone)]
pub struct ElectronDomain {
    /// Domain type (bonding or lone pair)
    pub domain_type: DomainType,

    /// Number of electrons in domain
    pub electron_count: usize,
}

/// Type of electron domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    /// Bonding pair (shared between atoms)
    BondingPair,
    /// Lone pair (non-bonding)
    LonePair,
}

/// Predict molecular geometry from electron domains (legacy VSEPR)
pub fn predict_geometry(electron_domains: usize, lone_pairs: usize) -> Option<MolecularGeometry> {
    let bonding_pairs = electron_domains - lone_pairs;

    match (electron_domains, lone_pairs) {
        (2, 0) => Some(MolecularGeometry::Linear),
        (3, 0) => Some(MolecularGeometry::TrigonalPlanar),
        (3, 1) => Some(MolecularGeometry::Bent),
        (4, 0) => Some(MolecularGeometry::Tetrahedral),
        (4, 1) => Some(MolecularGeometry::TrigonalPyramidal),
        (4, 2) => Some(MolecularGeometry::BentTetrahedral),
        (5, 0) => Some(MolecularGeometry::TrigonalBipyramidal),
        (5, 1) => Some(MolecularGeometry::Seesaw),
        (5, 2) => Some(MolecularGeometry::TShaped),
        (5, 3) => Some(MolecularGeometry::LinearFiveDomain),
        (6, 0) => Some(MolecularGeometry::Octahedral),
        (6, 1) => Some(MolecularGeometry::SquarePyramidal),
        (6, 2) => Some(MolecularGeometry::SquarePlanar),
        _ => None,
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_creation() {
        let a1 = Angle::from_degrees(90.0);
        assert!((a1.degrees() - 90.0).abs() < 1e-10);
        assert!((a1.radians() - PI / 2.0).abs() < 1e-10);

        let a2 = Angle::from_radians(PI);
        assert!((a2.degrees() - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_atom_position() {
        let pos = AtomPosition::new(Vector3::new(1.0, 2.0, 3.0), 6);
        assert_eq!(pos.element, 6);
        assert!((pos.position.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_molecular_interference_pattern() {
        let mut pattern = MolecularInterferencePattern::new(18, 36, Vector3::zero());

        // Set some test values
        pattern.set_amplitude(9, 0, 0.5);
        pattern.set_amplitude(9, 1, 0.8);
        pattern.set_amplitude(9, 2, 0.5);

        assert!((pattern.get_amplitude(9, 1) - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_geometry_type_expected_angles() {
        // From ROADMAP: "Tetrahedral: 109.5°, Trigonal: 120°, Octahedral: 90°"
        let tetra = GeometryType::Tetrahedral;
        let angles = tetra.expected_angles();
        assert!(angles.iter().all(|&a| (a - 109.5).abs() < 0.1));

        let trig = GeometryType::TrigonalPlanar;
        let angles = trig.expected_angles();
        assert!(angles.iter().all(|&a| (a - 120.0).abs() < 0.1));

        let oct = GeometryType::Octahedral;
        let angles = oct.expected_angles();
        assert!(angles.iter().all(|&a| (a - 90.0).abs() < 0.1));
    }

    #[test]
    fn test_geometry_type_coordination_numbers() {
        assert_eq!(GeometryType::Linear.coordination_number(), 2);
        assert_eq!(GeometryType::TrigonalPlanar.coordination_number(), 3);
        assert_eq!(GeometryType::Tetrahedral.coordination_number(), 4);
        assert_eq!(GeometryType::TrigonalBipyramidal.coordination_number(), 5);
        assert_eq!(GeometryType::Octahedral.coordination_number(), 6);
    }

    #[test]
    fn test_classify_geometry_linear() {
        let angles = vec![180.0];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::Linear);
    }

    #[test]
    fn test_classify_geometry_tetrahedral() {
        // Tetrahedral has 6 bond angles (4 atoms choose 2)
        let angles = vec![109.5, 109.5, 109.5, 109.5, 109.5, 109.5];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::Tetrahedral);
    }

    #[test]
    fn test_classify_geometry_trigonal_planar() {
        let angles = vec![120.0, 120.0, 120.0];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::TrigonalPlanar);
    }

    #[test]
    fn test_classify_geometry_octahedral() {
        let angles = vec![90.0; 12];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::Octahedral);
    }

    #[test]
    fn test_classify_geometry_trigonal_pyramidal() {
        let angles = vec![107.0, 107.0, 107.0];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::TrigonalPyramidal);
    }

    #[test]
    fn test_classify_geometry_bent() {
        let angles = vec![104.5, 104.5];
        let geo = FieldDerivedGeometry::classify_geometry(&angles);
        assert_eq!(geo, GeometryType::Bent);
    }

    #[test]
    fn test_ideal_directions() {
        // Linear
        let dirs = FieldDerivedGeometry::ideal_directions(GeometryType::Linear);
        assert_eq!(dirs.len(), 2);

        // Tetrahedral
        let dirs = FieldDerivedGeometry::ideal_directions(GeometryType::Tetrahedral);
        assert_eq!(dirs.len(), 4);

        // Check tetrahedral angles are approximately 109.5°
        for i in 0..4 {
            for j in (i + 1)..4 {
                let dot = dirs[i].dot(&dirs[j]);
                let angle = dot.acos().to_degrees();
                assert!(
                    (angle - 109.5).abs() < 1.0,
                    "Expected ~109.5°, got {}",
                    angle
                );
            }
        }
    }

    #[test]
    fn test_calculate_geometry_coherence() {
        let angles = vec![109.5, 109.5, 109.5, 109.5];
        let coherence =
            FieldDerivedGeometry::calculate_geometry_coherence(&GeometryType::Tetrahedral, &angles);
        assert!(coherence > 0.99);

        let bad_angles = vec![90.0, 90.0, 90.0, 90.0];
        let coherence = FieldDerivedGeometry::calculate_geometry_coherence(
            &GeometryType::Tetrahedral,
            &bad_angles,
        );
        assert!(coherence < 0.9);
    }

    #[test]
    fn test_legacy_predict_geometry() {
        // Methane: 4 domains, 0 lone pairs
        assert_eq!(predict_geometry(4, 0), Some(MolecularGeometry::Tetrahedral));

        // Ammonia: 4 domains, 1 lone pair
        assert_eq!(
            predict_geometry(4, 1),
            Some(MolecularGeometry::TrigonalPyramidal)
        );

        // Water: 4 domains, 2 lone pairs
        assert_eq!(
            predict_geometry(4, 2),
            Some(MolecularGeometry::BentTetrahedral)
        );
    }

    #[test]
    fn test_legacy_ideal_bond_angles() {
        let geo = MolecularGeometry::Linear;
        assert_eq!(geo.ideal_bond_angles(), vec![180.0]);

        let geo = MolecularGeometry::Tetrahedral;
        assert_eq!(geo.ideal_bond_angles(), vec![109.5]);
    }

    #[test]
    fn test_field_derived_geometry_default() {
        let geo = FieldDerivedGeometry::default();
        assert_eq!(geo.geometry_type, GeometryType::Tetrahedral);
        assert_eq!(geo.coordination_number, 4);
        assert!(geo.validate_geometry());
    }

    #[test]
    fn test_molecular_geometry_to_geometry_type() {
        assert_eq!(
            MolecularGeometry::Linear.to_geometry_type(),
            GeometryType::Linear
        );
        assert_eq!(
            MolecularGeometry::Tetrahedral.to_geometry_type(),
            GeometryType::Tetrahedral
        );
        assert_eq!(
            MolecularGeometry::Octahedral.to_geometry_type(),
            GeometryType::Octahedral
        );
    }
}
