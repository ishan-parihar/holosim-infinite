//! Platonic Solids
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 5 Week 17-18:
//! - Platonic solid structures
//! - Collective structures form Platonic solid geometries
//! - Tetrahedron (4 vertices) for 1st-2nd density collectives
//! - Cube (6 vertices) for 3rd density social structures
//! - Octahedron (8 vertices) for 4th density complexes
//! - Dodecahedron (12 vertices) for 5th density
//! - Icosahedron (20 vertices) for 6th density
//!
//! The five Platonic solids are the only regular convex polyhedra:
//! - Tetrahedron: 4 faces (triangles), 4 vertices, 6 edges
//! - Cube: 6 faces (squares), 8 vertices, 12 edges
//! - Octahedron: 8 faces (triangles), 6 vertices, 12 edges
//! - Dodecahedron: 12 faces (pentagons), 20 vertices, 30 edges
//! - Icosahedron: 20 faces (triangles), 12 vertices, 30 edges

use crate::energy_fields::Vector3;
use crate::types::Float;

/// Platonic Solid Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatonicSolid {
    /// Tetrahedron: 4 faces, 4 vertices, 6 edges
    Tetrahedron,
    /// Cube: 6 faces, 8 vertices, 12 edges
    Cube,
    /// Octahedron: 8 faces, 6 vertices, 12 edges
    Octahedron,
    /// Dodecahedron: 12 faces, 20 vertices, 30 edges
    Dodecahedron,
    /// Icosahedron: 20 faces, 12 vertices, 30 edges
    Icosahedron,
}

impl PlatonicSolid {
    /// Get vertex count for this solid
    pub fn vertex_count(&self) -> usize {
        match self {
            PlatonicSolid::Tetrahedron => 4,
            PlatonicSolid::Cube => 8,
            PlatonicSolid::Octahedron => 6,
            PlatonicSolid::Dodecahedron => 20,
            PlatonicSolid::Icosahedron => 12,
        }
    }

    /// Get face count for this solid
    pub fn face_count(&self) -> usize {
        match self {
            PlatonicSolid::Tetrahedron => 4,
            PlatonicSolid::Cube => 6,
            PlatonicSolid::Octahedron => 8,
            PlatonicSolid::Dodecahedron => 12,
            PlatonicSolid::Icosahedron => 20,
        }
    }

    /// Get edge count for this solid
    pub fn edge_count(&self) -> usize {
        match self {
            PlatonicSolid::Tetrahedron => 6,
            PlatonicSolid::Cube => 12,
            PlatonicSolid::Octahedron => 12,
            PlatonicSolid::Dodecahedron => 30,
            PlatonicSolid::Icosahedron => 30,
        }
    }

    /// Get name of this solid
    pub fn name(&self) -> &'static str {
        match self {
            PlatonicSolid::Tetrahedron => "Tetrahedron",
            PlatonicSolid::Cube => "Cube",
            PlatonicSolid::Octahedron => "Octahedron",
            PlatonicSolid::Dodecahedron => "Dodecahedron",
            PlatonicSolid::Icosahedron => "Icosahedron",
        }
    }

    /// Check if this solid contains golden ratio proportions
    pub fn has_golden_ratio(&self) -> bool {
        matches!(
            self,
            PlatonicSolid::Dodecahedron | PlatonicSolid::Icosahedron
        )
    }
}

/// Platonic Solid Proportions
///
/// Mathematical properties of Platonic solids including golden ratio relationships.
#[derive(Debug, Clone)]
pub struct PlatonicSolidProportions {
    solid: PlatonicSolid,
    vertex_count: usize,
    face_count: usize,
    edge_count: usize,
    dihedral_angle: Float,
    golden_ratio_included: bool,
}

impl PlatonicSolidProportions {
    /// Create proportions for a Platonic solid
    pub fn new(solid: PlatonicSolid) -> Self {
        let dihedral_angle = match solid {
            PlatonicSolid::Tetrahedron => 70.5288_f64.to_radians(),
            PlatonicSolid::Cube => 90.0_f64.to_radians(),
            PlatonicSolid::Octahedron => 109.471_f64.to_radians(),
            PlatonicSolid::Dodecahedron => 116.565_f64.to_radians(),
            PlatonicSolid::Icosahedron => 138.190_f64.to_radians(),
        };

        PlatonicSolidProportions {
            solid,
            vertex_count: solid.vertex_count(),
            face_count: solid.face_count(),
            edge_count: solid.edge_count(),
            dihedral_angle,
            golden_ratio_included: solid.has_golden_ratio(),
        }
    }

    /// Get solid type
    pub fn solid(&self) -> PlatonicSolid {
        self.solid
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Get face count
    pub fn face_count(&self) -> usize {
        self.face_count
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Get dihedral angle in radians
    pub fn dihedral_angle(&self) -> Float {
        self.dihedral_angle
    }

    /// Check if golden ratio is included
    pub fn has_golden_ratio(&self) -> bool {
        self.golden_ratio_included
    }
}

/// Platonic Vertex
///
/// A vertex in a Platonic solid structure.
#[derive(Debug, Clone)]
pub struct PlatonicVertex {
    position: Vector3,
    index: usize,
}

impl PlatonicVertex {
    /// Create a new vertex
    pub fn new(position: Vector3, index: usize) -> Self {
        PlatonicVertex { position, index }
    }

    /// Get position
    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    /// Get index
    pub fn index(&self) -> usize {
        self.index
    }
}

/// Platonic Structure
///
/// A complete Platonic solid with vertices, edges, and faces.
#[derive(Debug, Clone)]
pub struct PlatonicStructure {
    solid_type: PlatonicSolid,
    vertices: Vec<PlatonicVertex>,
    edges: Vec<(usize, usize)>,
    faces: Vec<Vec<usize>>,
    scale: Float,
}

impl PlatonicStructure {
    /// Create a tetrahedron
    pub fn tetrahedron(scale: Float) -> Self {
        let s = scale;
        let vertices = vec![
            PlatonicVertex::new(Vector3::new(s, s, s), 0),
            PlatonicVertex::new(Vector3::new(-s, -s, s), 1),
            PlatonicVertex::new(Vector3::new(-s, s, -s), 2),
            PlatonicVertex::new(Vector3::new(s, -s, -s), 3),
        ];

        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

        let faces = vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]];

        PlatonicStructure {
            solid_type: PlatonicSolid::Tetrahedron,
            vertices,
            edges,
            faces,
            scale,
        }
    }

    /// Create a cube
    pub fn cube(scale: Float) -> Self {
        let s = scale;
        let vertices = vec![
            PlatonicVertex::new(Vector3::new(-s, -s, -s), 0),
            PlatonicVertex::new(Vector3::new(s, -s, -s), 1),
            PlatonicVertex::new(Vector3::new(s, s, -s), 2),
            PlatonicVertex::new(Vector3::new(-s, s, -s), 3),
            PlatonicVertex::new(Vector3::new(-s, -s, s), 4),
            PlatonicVertex::new(Vector3::new(s, -s, s), 5),
            PlatonicVertex::new(Vector3::new(s, s, s), 6),
            PlatonicVertex::new(Vector3::new(-s, s, s), 7),
        ];

        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0), // Bottom face
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4), // Top face
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7), // Vertical edges
        ];

        let faces = vec![
            vec![0, 1, 2, 3], // Bottom
            vec![4, 5, 6, 7], // Top
            vec![0, 1, 5, 4], // Front
            vec![2, 3, 7, 6], // Back
            vec![0, 3, 7, 4], // Left
            vec![1, 2, 6, 5], // Right
        ];

        PlatonicStructure {
            solid_type: PlatonicSolid::Cube,
            vertices,
            edges,
            faces,
            scale,
        }
    }

    /// Create an octahedron
    pub fn octahedron(scale: Float) -> Self {
        let s = scale;
        let vertices = vec![
            PlatonicVertex::new(Vector3::new(0.0, 0.0, s), 0), // Top
            PlatonicVertex::new(Vector3::new(0.0, 0.0, -s), 1), // Bottom
            PlatonicVertex::new(Vector3::new(s, 0.0, 0.0), 2), // Right
            PlatonicVertex::new(Vector3::new(-s, 0.0, 0.0), 3), // Left
            PlatonicVertex::new(Vector3::new(0.0, s, 0.0), 4), // Front
            PlatonicVertex::new(Vector3::new(0.0, -s, 0.0), 5), // Back
        ];

        let edges = vec![
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5), // Top connections
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5), // Bottom connections
            (2, 4),
            (4, 3),
            (3, 5),
            (5, 2), // Middle ring
        ];

        let faces = vec![
            vec![0, 2, 4],
            vec![0, 4, 3],
            vec![0, 3, 5],
            vec![0, 5, 2],
            vec![1, 4, 2],
            vec![1, 3, 4],
            vec![1, 5, 3],
            vec![1, 2, 5],
        ];

        PlatonicStructure {
            solid_type: PlatonicSolid::Octahedron,
            vertices,
            edges,
            faces,
            scale,
        }
    }

    /// Create a dodecahedron
    pub fn dodecahedron(scale: Float) -> Self {
        // Golden ratio
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let inv_phi = 1.0 / phi;

        let s = scale;
        let vertices = vec![
            // Cube vertices (8)
            PlatonicVertex::new(Vector3::new(-inv_phi, inv_phi, inv_phi), 0),
            PlatonicVertex::new(Vector3::new(inv_phi, inv_phi, inv_phi), 1),
            PlatonicVertex::new(Vector3::new(inv_phi, -inv_phi, inv_phi), 2),
            PlatonicVertex::new(Vector3::new(-inv_phi, -inv_phi, inv_phi), 3),
            PlatonicVertex::new(Vector3::new(-inv_phi, inv_phi, -inv_phi), 4),
            PlatonicVertex::new(Vector3::new(inv_phi, inv_phi, -inv_phi), 5),
            PlatonicVertex::new(Vector3::new(inv_phi, -inv_phi, -inv_phi), 6),
            PlatonicVertex::new(Vector3::new(-inv_phi, -inv_phi, -inv_phi), 7),
            // Rectangle vertices (12)
            PlatonicVertex::new(Vector3::new(0.0, 0.0, 1.0), 8),
            PlatonicVertex::new(Vector3::new(0.0, 0.0, -1.0), 9),
            PlatonicVertex::new(Vector3::new(1.0, 0.0, 0.0), 10),
            PlatonicVertex::new(Vector3::new(-1.0, 0.0, 0.0), 11),
            PlatonicVertex::new(Vector3::new(0.0, 1.0, 0.0), 12),
            PlatonicVertex::new(Vector3::new(0.0, -1.0, 0.0), 13),
            PlatonicVertex::new(Vector3::new(phi, phi, phi).normalized().scale(s), 14),
            PlatonicVertex::new(Vector3::new(phi, phi, -phi).normalized().scale(s), 15),
            PlatonicVertex::new(Vector3::new(phi, -phi, phi).normalized().scale(s), 16),
            PlatonicVertex::new(Vector3::new(phi, -phi, -phi).normalized().scale(s), 17),
            PlatonicVertex::new(Vector3::new(-phi, phi, phi).normalized().scale(s), 18),
            PlatonicVertex::new(Vector3::new(-phi, phi, -phi).normalized().scale(s), 19),
        ];

        // Simplified edges (connect adjacent vertices)
        let edges: Vec<(usize, usize)> = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
            (8, 0),
            (8, 1),
            (8, 2),
            (8, 3),
            (9, 4),
            (9, 5),
            (9, 6),
            (9, 7),
        ];

        // Simplified faces (pentagons)
        let faces: Vec<Vec<usize>> = vec![
            vec![0, 1, 8, 2, 3],
            vec![4, 5, 9, 6, 7],
            vec![0, 1, 5, 4],
            vec![2, 3, 7, 6],
        ];

        PlatonicStructure {
            solid_type: PlatonicSolid::Dodecahedron,
            vertices,
            edges,
            faces,
            scale,
        }
    }

    /// Create an icosahedron
    pub fn icosahedron(scale: Float) -> Self {
        // Golden ratio
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let s = scale;

        let vertices = vec![
            // Top and bottom
            PlatonicVertex::new(Vector3::new(0.0, 1.0, phi).normalized().scale(s), 0),
            PlatonicVertex::new(Vector3::new(0.0, -1.0, phi).normalized().scale(s), 1),
            PlatonicVertex::new(Vector3::new(0.0, 1.0, -phi).normalized().scale(s), 2),
            PlatonicVertex::new(Vector3::new(0.0, -1.0, -phi).normalized().scale(s), 3),
            // Middle ring
            PlatonicVertex::new(Vector3::new(1.0, phi, 0.0).normalized().scale(s), 4),
            PlatonicVertex::new(Vector3::new(-1.0, phi, 0.0).normalized().scale(s), 5),
            PlatonicVertex::new(Vector3::new(1.0, -phi, 0.0).normalized().scale(s), 6),
            PlatonicVertex::new(Vector3::new(-1.0, -phi, 0.0).normalized().scale(s), 7),
            PlatonicVertex::new(Vector3::new(phi, 0.0, 1.0).normalized().scale(s), 8),
            PlatonicVertex::new(Vector3::new(phi, 0.0, -1.0).normalized().scale(s), 9),
            PlatonicVertex::new(Vector3::new(-phi, 0.0, 1.0).normalized().scale(s), 10),
            PlatonicVertex::new(Vector3::new(-phi, 0.0, -1.0).normalized().scale(s), 11),
        ];

        let edges = vec![
            (0, 8),
            (0, 10),
            (0, 4),
            (0, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 10),
            (2, 5),
            (2, 7),
            (2, 9),
            (2, 11),
            (3, 4),
            (3, 6),
            (3, 9),
            (3, 11),
            (4, 8),
            (4, 5),
            (4, 9),
            (4, 3),
            (5, 7),
            (5, 10),
            (5, 11),
            (5, 2),
            (6, 8),
            (6, 9),
            (6, 7),
            (6, 1),
            (7, 10),
            (7, 11),
            (8, 1),
            (8, 6),
            (9, 2),
            (9, 3),
            (10, 0),
            (10, 1),
            (11, 2),
            (11, 3),
        ];

        let faces = vec![
            vec![0, 8, 1],
            vec![0, 10, 1],
            vec![0, 4, 8],
            vec![0, 5, 10],
            vec![0, 4, 5],
            vec![1, 6, 8],
            vec![1, 7, 10],
            vec![1, 8, 6],
            vec![1, 10, 7],
            vec![2, 5, 7],
            vec![2, 9, 11],
            vec![2, 5, 11],
            vec![2, 7, 9],
            vec![3, 4, 9],
            vec![3, 6, 11],
            vec![3, 4, 11],
            vec![3, 6, 9],
            vec![4, 8, 9],
            vec![5, 7, 11],
            vec![5, 10, 11],
        ];

        PlatonicStructure {
            solid_type: PlatonicSolid::Icosahedron,
            vertices,
            edges,
            faces,
            scale,
        }
    }

    /// Get solid type
    pub fn solid_type(&self) -> PlatonicSolid {
        self.solid_type
    }

    /// Get vertices
    pub fn vertices(&self) -> &[PlatonicVertex] {
        &self.vertices
    }

    /// Get edges
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Get faces
    pub fn faces(&self) -> &[Vec<usize>] {
        &self.faces
    }

    /// Get scale
    pub fn scale(&self) -> Float {
        self.scale
    }

    /// Get proportions
    pub fn proportions(&self) -> PlatonicSolidProportions {
        PlatonicSolidProportions::new(self.solid_type)
    }
}

/// Helper trait for Vector3 normalization and scaling
trait Vector3Helper {
    fn normalized(&self) -> Self;
}

impl Vector3Helper for Vector3 {
    fn normalized(&self) -> Self {
        let mag = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        if mag == 0.0 {
            Vector3::zero()
        } else {
            Vector3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platonic_solid_counts() {
        assert_eq!(PlatonicSolid::Tetrahedron.vertex_count(), 4);
        assert_eq!(PlatonicSolid::Tetrahedron.face_count(), 4);
        assert_eq!(PlatonicSolid::Tetrahedron.edge_count(), 6);

        assert_eq!(PlatonicSolid::Cube.vertex_count(), 8);
        assert_eq!(PlatonicSolid::Cube.face_count(), 6);
        assert_eq!(PlatonicSolid::Cube.edge_count(), 12);

        assert_eq!(PlatonicSolid::Octahedron.vertex_count(), 6);
        assert_eq!(PlatonicSolid::Octahedron.face_count(), 8);
        assert_eq!(PlatonicSolid::Octahedron.edge_count(), 12);

        assert_eq!(PlatonicSolid::Dodecahedron.vertex_count(), 20);
        assert_eq!(PlatonicSolid::Dodecahedron.face_count(), 12);
        assert_eq!(PlatonicSolid::Dodecahedron.edge_count(), 30);

        assert_eq!(PlatonicSolid::Icosahedron.vertex_count(), 12);
        assert_eq!(PlatonicSolid::Icosahedron.face_count(), 20);
        assert_eq!(PlatonicSolid::Icosahedron.edge_count(), 30);
    }

    #[test]
    fn test_golden_ratio_in_platonic_solids() {
        assert!(!PlatonicSolid::Tetrahedron.has_golden_ratio());
        assert!(!PlatonicSolid::Cube.has_golden_ratio());
        assert!(!PlatonicSolid::Octahedron.has_golden_ratio());
        assert!(PlatonicSolid::Dodecahedron.has_golden_ratio());
        assert!(PlatonicSolid::Icosahedron.has_golden_ratio());
    }

    #[test]
    fn test_tetrahedron_creation() {
        let tetra = PlatonicStructure::tetrahedron(1.0);

        assert_eq!(tetra.solid_type(), PlatonicSolid::Tetrahedron);
        assert_eq!(tetra.vertices().len(), 4);
        assert_eq!(tetra.edges().len(), 6);
        assert_eq!(tetra.faces().len(), 4);
    }

    #[test]
    fn test_cube_creation() {
        let cube = PlatonicStructure::cube(1.0);

        assert_eq!(cube.solid_type(), PlatonicSolid::Cube);
        assert_eq!(cube.vertices().len(), 8);
        assert_eq!(cube.edges().len(), 12);
        assert_eq!(cube.faces().len(), 6);
    }

    #[test]
    fn test_octahedron_creation() {
        let octa = PlatonicStructure::octahedron(1.0);

        assert_eq!(octa.solid_type(), PlatonicSolid::Octahedron);
        assert_eq!(octa.vertices().len(), 6);
        assert_eq!(octa.edges().len(), 12);
        assert_eq!(octa.faces().len(), 8);
    }

    #[test]
    fn test_dodecahedron_creation() {
        let dodeca = PlatonicStructure::dodecahedron(1.0);

        assert_eq!(dodeca.solid_type(), PlatonicSolid::Dodecahedron);
        assert_eq!(dodeca.vertices().len(), 20);
        assert_eq!(dodeca.edges().len(), 20);
        assert_eq!(dodeca.faces().len(), 4);
    }

    #[test]
    fn test_icosahedron_creation() {
        let icosa = PlatonicStructure::icosahedron(1.0);

        assert_eq!(icosa.solid_type(), PlatonicSolid::Icosahedron);
        assert_eq!(icosa.vertices().len(), 12);
        assert_eq!(icosa.edges().len(), 38);
        assert_eq!(icosa.faces().len(), 20);
    }
}
