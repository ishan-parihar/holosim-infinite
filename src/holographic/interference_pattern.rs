// Interference Patterns for Holographic Encoding
//
// The interference pattern is the result of 22 archetype beams interacting.
// It contains CROSS-TERMS that encode the holographic information.
// Stable configurations emerge at constructive interference nodes (bright fringes).
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Section "Interference Patterns"
// - "The cross-terms encode the holographic information"

use crate::holographic::{ComplexVector, Position};
use crate::types::Float;

// ============================================================================
// INTERFERENCE PATTERN
// ============================================================================

/// Interference pattern from archetype interactions
#[derive(Debug, Clone, Default)]
pub struct InterferencePattern {
    /// Intensity field (bright/dark fringes)
    pub intensity_field: IntensityField,
    /// Phase field (phase relationships)
    pub phase_field: PhaseField,
    /// Cross-terms (the holographic encoding)
    pub cross_terms: CrossTermMatrix,
    /// Constructive nodes (stable configurations)
    pub constructive_nodes: Vec<Position>,
    /// Destructive nodes (unstable configurations)
    pub destructive_nodes: Vec<Position>,
}

impl InterferencePattern {
    /// Create interference pattern from archetype complex vectors
    pub fn from_archetypes(archetypes: &[ComplexVector; 22]) -> Self {
        let mut pattern = Self::default();

        // Calculate cross-terms (the holographic encoding)
        for i in 0..22 {
            for j in 0..22 {
                if i != j {
                    // Cross-term: A_i * A_j^*
                    pattern.cross_terms.matrix[i][j] =
                        archetypes[i].multiply_conjugate(&archetypes[j]);
                }
            }
        }

        // Calculate total field
        let total_field: ComplexVector = archetypes
            .iter()
            .fold(ComplexVector::default(), |acc, cv| acc.add(cv));

        // Calculate intensity field
        pattern.intensity_field = IntensityField::from_complex(total_field);

        // Calculate phase field
        pattern.phase_field = PhaseField::from_phase(total_field.phase());

        // Find constructive and destructive nodes
        pattern.find_interference_nodes();

        pattern
    }

    /// Find constructive and destructive interference nodes
    fn find_interference_nodes(&mut self) {
        // Constructive nodes = local maxima in intensity field
        self.constructive_nodes = self.intensity_field.find_local_maxima();

        // Destructive nodes = local minima in intensity field
        self.destructive_nodes = self.intensity_field.find_local_minima();
    }

    /// Find constructive interference nodes
    pub fn find_constructive_nodes(&self) -> Vec<Position> {
        self.intensity_field.find_local_maxima()
    }

    /// Find destructive interference nodes
    pub fn find_destructive_nodes(&self) -> Vec<Position> {
        self.intensity_field.find_local_minima()
    }

    /// Returns the phase coherence of the interference pattern.
    ///
    /// Phase coherence measures how aligned the phases of the cross-terms are.
    /// Higher coherence = more unity (Law of One).
    ///
    /// # Returns
    ///
    /// Phase coherence (0.0 to 1.0, where 1.0 is perfect coherence)
    pub fn phase_coherence(&self) -> Float {
        self.cross_terms.phase_coherence()
    }
}

// ============================================================================
// INTENSITY FIELD
// ============================================================================

/// Intensity field (bright/dark fringes)
#[derive(Debug, Clone)]
pub struct IntensityField {
    /// Grid of intensity values
    pub grid: Vec<Float>,
    /// Grid dimensions
    pub dimensions: (usize, usize, usize),
}

impl IntensityField {
    /// Create intensity field from complex vector
    pub fn from_complex(cv: ComplexVector) -> Self {
        let intensity = cv.norm_squared();

        // Create a simple 10x10x10 grid
        let size = 10;
        let mut grid = vec![0.0; size * size * size];

        // Fill grid with intensity pattern (simplified)
        for i in 0..size {
            for j in 0..size {
                for k in 0..size {
                    let idx = i * size * size + j * size + k;
                    // Create interference pattern
                    let x = i as Float / size as Float;
                    let y = j as Float / size as Float;
                    let z = k as Float / size as Float;
                    grid[idx] =
                        intensity * (std::f64::consts::PI as Float * (x + y + z)).cos().powi(2);
                }
            }
        }

        IntensityField {
            grid,
            dimensions: (size, size, size),
        }
    }

    /// Find local maxima (constructive nodes)
    pub fn find_local_maxima(&self) -> Vec<Position> {
        let mut maxima = Vec::new();
        let (nx, ny, nz) = self.dimensions;

        for i in 1..nx - 1 {
            for j in 1..ny - 1 {
                for k in 1..nz - 1 {
                    let current = self.get(i, j, k);
                    let prev_x = self.get(i - 1, j, k);
                    let next_x = self.get(i + 1, j, k);
                    let prev_y = self.get(i, j - 1, k);
                    let next_y = self.get(i, j + 1, k);
                    let prev_z = self.get(i, j, k - 1);
                    let next_z = self.get(i, j, k + 1);

                    if current > prev_x
                        && current > next_x
                        && current > prev_y
                        && current > next_y
                        && current > prev_z
                        && current > next_z
                    {
                        // Convert grid indices to position
                        let x = i as Float / nx as Float;
                        let y = j as Float / ny as Float;
                        let z = k as Float / nz as Float;
                        maxima.push(Position::new(x, y, z));
                    }
                }
            }
        }

        maxima
    }

    /// Find local minima (destructive nodes)
    pub fn find_local_minima(&self) -> Vec<Position> {
        let mut minima = Vec::new();
        let (nx, ny, nz) = self.dimensions;

        for i in 1..nx - 1 {
            for j in 1..ny - 1 {
                for k in 1..nz - 1 {
                    let current = self.get(i, j, k);
                    let prev_x = self.get(i - 1, j, k);
                    let next_x = self.get(i + 1, j, k);
                    let prev_y = self.get(i, j - 1, k);
                    let next_y = self.get(i, j + 1, k);
                    let prev_z = self.get(i, j, k - 1);
                    let next_z = self.get(i, j, k + 1);

                    if current < prev_x
                        && current < next_x
                        && current < prev_y
                        && current < next_y
                        && current < prev_z
                        && current < next_z
                    {
                        // Convert grid indices to position
                        let x = i as Float / nx as Float;
                        let y = j as Float / ny as Float;
                        let z = k as Float / nz as Float;
                        minima.push(Position::new(x, y, z));
                    }
                }
            }
        }

        minima
    }

    /// Get intensity at grid position
    fn get(&self, i: usize, j: usize, k: usize) -> Float {
        let (nx, ny, nz) = self.dimensions;
        if i < nx && j < ny && k < nz {
            self.grid[i * ny * nz + j * nz + k]
        } else {
            0.0
        }
    }
}

impl Default for IntensityField {
    fn default() -> Self {
        IntensityField {
            grid: vec![0.0; 1000],
            dimensions: (10, 10, 10),
        }
    }
}

// ============================================================================
// PHASE FIELD
// ============================================================================

/// Phase field (phase relationships)
#[derive(Debug, Clone)]
pub struct PhaseField {
    /// Grid of phase values
    pub grid: Vec<Float>,
    /// Grid dimensions
    pub dimensions: (usize, usize, usize),
}

impl PhaseField {
    /// Create phase field from phase value
    pub fn from_phase(phase: Float) -> Self {
        let size = 10;
        let mut grid = vec![0.0; size * size * size];

        // Fill grid with phase pattern (simplified)
        for i in 0..size {
            for j in 0..size {
                for k in 0..size {
                    let idx = i * size * size + j * size + k;
                    // Create phase pattern
                    let x = i as Float / size as Float;
                    let y = j as Float / size as Float;
                    let z = k as Float / size as Float;
                    grid[idx] = (phase + std::f64::consts::PI as Float * (x + y + z))
                        % (2.0 * std::f64::consts::PI as Float);
                }
            }
        }

        PhaseField {
            grid,
            dimensions: (size, size, size),
        }
    }
}

impl Default for PhaseField {
    fn default() -> Self {
        PhaseField {
            grid: vec![0.0; 1000],
            dimensions: (10, 10, 10),
        }
    }
}

// ============================================================================
// CROSS-TERM MATRIX
// ============================================================================

/// Cross-term matrix (encodes holographic information)
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct CrossTermMatrix {
    /// Matrix of cross-terms between all archetype pairs
    pub matrix: [[ComplexVector; 22]; 22],
}

impl CrossTermMatrix {
    /// Calculate total cross-term magnitude
    pub fn total_magnitude(&self) -> Float {
        let mut total = 0.0;

        for i in 0..22 {
            for j in 0..22 {
                if i != j {
                    total += self.matrix[i][j].amplitude();
                }
            }
        }

        total
    }

    /// Calculate phase coherence of cross-terms
    pub fn phase_coherence(&self) -> Float {
        let mut coherence = 0.0;
        let mut count = 0;

        for i in 0..22 {
            for j in 0..22 {
                if i != j {
                    let cross_term = self.matrix[i][j];
                    let normalized = cross_term.normalize().unwrap_or_default();
                    coherence += normalized.real; // cos(phase)
                    count += 1;
                }
            }
        }

        if count > 0 {
            // Normalize to [0, 1] range since average of cos values can be negative
            (coherence / count as Float).abs()
        } else {
            0.0
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic::ComplexVector;

    #[test]
    fn test_interference_pattern_from_archetypes() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        assert_eq!(pattern.cross_terms.matrix.len(), 22);
        assert!(!pattern.constructive_nodes.is_empty());
        assert!(!pattern.destructive_nodes.is_empty());
    }

    #[test]
    fn test_cross_term_calculation() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        // Check that cross-terms are calculated
        for i in 0..22 {
            for j in 0..22 {
                if i != j {
                    let cross_term = pattern.cross_terms.matrix[i][j];
                    // Cross-term should not be zero
                    assert!(cross_term.norm_squared() > 0.0);
                }
            }
        }
    }

    #[test]
    fn test_cross_term_total_magnitude() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        let magnitude = pattern.cross_terms.total_magnitude();
        assert!(magnitude > 0.0);
    }

    #[test]
    fn test_cross_term_phase_coherence() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        let coherence = pattern.cross_terms.phase_coherence();
        assert!((-1.0..=1.0).contains(&coherence));
    }

    #[test]
    fn test_intensity_field() {
        let cv = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let field = IntensityField::from_complex(cv);

        assert_eq!(field.dimensions, (10, 10, 10));
        assert_eq!(field.grid.len(), 1000);
    }

    #[test]
    fn test_intensity_field_local_maxima() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        let maxima = pattern.intensity_field.find_local_maxima();
        assert!(!maxima.is_empty());
    }

    #[test]
    fn test_intensity_field_local_minima() {
        let archetypes = create_test_archetypes();
        let pattern = InterferencePattern::from_archetypes(&archetypes);

        let minima = pattern.intensity_field.find_local_minima();
        assert!(!minima.is_empty());
    }

    #[test]
    fn test_phase_field() {
        let phase = std::f64::consts::PI as Float / 2.0;
        let field = PhaseField::from_phase(phase);

        assert_eq!(field.dimensions, (10, 10, 10));
        assert_eq!(field.grid.len(), 1000);
    }

    fn create_test_archetypes() -> [ComplexVector; 22] {
        let mut archetypes = [ComplexVector::default(); 22];

        for (i, archetype) in archetypes.iter_mut().enumerate() {
            let amplitude = 0.5 + (i as Float / 22.0) * 0.5;
            let phase = (i as Float / 22.0) * 2.0 * std::f64::consts::PI as Float;
            *archetype = ComplexVector::from_amplitude_phase(amplitude, phase);
        }

        archetypes
    }
}
