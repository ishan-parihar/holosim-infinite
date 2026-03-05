// Complex Vectors for Holographic Encoding
//
// In holography, you need both INTENSITY and PHASE information.
// The phase relationships encode the holographic information.
// Phase coherence = Unity (Law of One).
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Section "Complex Vectors for Archetypes"
// - "Phase coherence = Unity"

use crate::types::Float;

// ============================================================================
// COMPLEX VECTOR
// ============================================================================

/// Complex vector representing amplitude and phase
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexVector {
    pub real: Float, // Amplitude * cos(phase)
    pub imag: Float, // Amplitude * sin(phase)
}

impl ComplexVector {
    /// Create complex vector from amplitude and phase
    pub fn from_amplitude_phase(amplitude: Float, phase: Float) -> Self {
        ComplexVector {
            real: amplitude * phase.cos(),
            imag: amplitude * phase.sin(),
        }
    }

    /// Get amplitude (magnitude)
    pub fn amplitude(&self) -> Float {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    /// Get phase angle (0 to 2π)
    pub fn phase(&self) -> Float {
        self.imag.atan2(self.real)
    }

    /// Get squared norm (amplitude squared)
    pub fn norm_squared(&self) -> Float {
        self.real.powi(2) + self.imag.powi(2)
    }

    /// Add two complex vectors
    pub fn add(&self, other: &ComplexVector) -> ComplexVector {
        ComplexVector {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    /// Subtract two complex vectors
    pub fn sub(&self, other: &ComplexVector) -> ComplexVector {
        ComplexVector {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    /// Multiply two complex vectors
    pub fn multiply(&self, other: &ComplexVector) -> ComplexVector {
        ComplexVector {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    /// Scale complex vector by real number
    pub fn scale(&self, factor: Float) -> ComplexVector {
        ComplexVector {
            real: self.real * factor,
            imag: self.imag * factor,
        }
    }

    /// Get complex conjugate
    pub fn conjugate(&self) -> ComplexVector {
        ComplexVector {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Multiply by conjugate of other
    pub fn multiply_conjugate(&self, other: &ComplexVector) -> ComplexVector {
        self.multiply(&other.conjugate())
    }

    /// Normalize complex vector (set amplitude to 1.0)
    pub fn normalize(&self) -> Option<ComplexVector> {
        let amplitude = self.amplitude();
        if amplitude > 0.0 {
            Some(self.scale(1.0 / amplitude))
        } else {
            None
        }
    }

    /// Rotate by phase angle
    pub fn rotate(&self, phase: Float) -> ComplexVector {
        let rotation = ComplexVector::from_amplitude_phase(1.0, phase);
        self.multiply(&rotation)
    }

    /// Calculate phase coherence with another complex vector
    /// Returns 1.0 for perfect coherence (same phase), 0.0 for orthogonal, -1.0 for opposite
    pub fn phase_coherence(&self, other: &ComplexVector) -> Float {
        let self_normalized = self.normalize().unwrap_or(ComplexVector {
            real: 0.0,
            imag: 0.0,
        });
        let other_normalized = other.normalize().unwrap_or(ComplexVector {
            real: 0.0,
            imag: 0.0,
        });

        let cos_phase_diff = self_normalized.real * other_normalized.real
            + self_normalized.imag * other_normalized.imag;
        cos_phase_diff.clamp(-1.0, 1.0)
    }
}

impl Default for ComplexVector {
    fn default() -> Self {
        ComplexVector {
            real: 1.0,
            imag: 0.0,
        }
    }
}

// ============================================================================
// COMPLEX ARCHETYPE
// ============================================================================

/// Complex archetype with amplitude and phase
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexArchetype {
    pub amplitude: Float, // Activation level (0.0 to 1.0)
    pub phase: Float,     // Phase angle (0 to 2π)
}

impl ComplexArchetype {
    /// Create complex archetype
    pub fn new(amplitude: Float, phase: Float) -> Self {
        ComplexArchetype {
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: phase % (2.0 * std::f64::consts::PI as Float),
        }
    }

    /// Convert to complex vector
    pub fn to_complex_vector(&self) -> ComplexVector {
        ComplexVector::from_amplitude_phase(self.amplitude, self.phase)
    }

    /// Set amplitude (clamped to 0.0-1.0)
    pub fn with_amplitude(mut self, amplitude: Float) -> Self {
        self.amplitude = amplitude.clamp(0.0, 1.0);
        self
    }

    /// Set phase (wrapped to 0-2π)
    pub fn with_phase(mut self, phase: Float) -> Self {
        self.phase = phase % (2.0 * std::f64::consts::PI as Float);
        self
    }
}

impl Default for ComplexArchetype {
    fn default() -> Self {
        ComplexArchetype {
            amplitude: 0.5,
            phase: 0.0,
        }
    }
}

// ============================================================================
// TEST ARCHETYPES
// ============================================================================

#[cfg(test)]
pub fn test_archetypes() -> [ComplexArchetype; 22] {
    let mut archetypes = [ComplexArchetype::default(); 22];

    // Create test archetypes with varying amplitudes and phases
    for (i, archetype) in archetypes.iter_mut().enumerate() {
        let amplitude = 0.5 + (i as Float / 22.0) * 0.5;
        let phase = (i as Float / 22.0) * 2.0 * std::f64::consts::PI as Float;
        *archetype = ComplexArchetype::new(amplitude, phase);
    }

    archetypes
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_vector_amplitude() {
        let cv = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        assert!((cv.amplitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_phase() {
        let cv = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        assert!((cv.phase() - std::f64::consts::PI as Float / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_add() {
        let a = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let b = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let c = a.add(&b);
        assert!((c.amplitude() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_multiply() {
        let a = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        let b = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        let c = a.multiply(&b);
        assert!((c.amplitude() - 1.0).abs() < 1e-10);
        assert!((c.phase() - std::f64::consts::PI as Float / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_conjugate() {
        let cv = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        let conj = cv.conjugate();
        assert!((conj.phase() + std::f64::consts::PI as Float / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_normalize() {
        let cv = ComplexVector::from_amplitude_phase(2.0, std::f64::consts::PI as Float / 4.0);
        let normalized = cv.normalize().unwrap();
        assert!((normalized.amplitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_rotate() {
        let cv = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let rotated = cv.rotate(std::f64::consts::PI as Float / 2.0);
        assert!((rotated.phase() - std::f64::consts::PI as Float / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_phase_coherence() {
        let cv1 = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let cv2 = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let coherence = cv1.phase_coherence(&cv2);
        assert!((coherence - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_phase_coherence_opposite() {
        let cv1 = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let cv2 = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float);
        let coherence = cv1.phase_coherence(&cv2);
        assert!((coherence - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_complex_vector_phase_coherence_orthogonal() {
        let cv1 = ComplexVector::from_amplitude_phase(1.0, 0.0);
        let cv2 = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 2.0);
        let coherence = cv1.phase_coherence(&cv2);
        assert!(coherence.abs() < 1e-10);
    }

    #[test]
    fn test_complex_archetype_creation() {
        let ca = ComplexArchetype::new(0.8, std::f64::consts::PI as Float / 2.0);
        assert_eq!(ca.amplitude, 0.8);
        assert!((ca.phase - std::f64::consts::PI as Float / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_archetype_clamping() {
        let ca = ComplexArchetype::new(1.5, 0.0);
        assert_eq!(ca.amplitude, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_complex_archetype_phase_wrapping() {
        let ca = ComplexArchetype::new(0.5, 3.0 * std::f64::consts::PI as Float);
        assert!((ca.phase - std::f64::consts::PI as Float).abs() < 1e-10); // Wrapped
    }

    #[test]
    fn test_complex_archetype_to_complex_vector() {
        let ca = ComplexArchetype::new(0.5, std::f64::consts::PI as Float / 4.0);
        let cv = ca.to_complex_vector();
        assert!((cv.amplitude() - 0.5).abs() < 1e-10);
        assert!((cv.phase() - std::f64::consts::PI as Float / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_archetype_with_amplitude() {
        let ca = ComplexArchetype::default();
        let ca = ca.with_amplitude(0.8);
        assert_eq!(ca.amplitude, 0.8);
    }

    #[test]
    fn test_complex_archetype_with_phase() {
        let ca = ComplexArchetype::default();
        let ca = ca.with_phase(std::f64::consts::PI as Float / 2.0);
        assert!((ca.phase - std::f64::consts::PI as Float / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiply_conjugate() {
        let cv1 = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 4.0);
        let cv2 = ComplexVector::from_amplitude_phase(1.0, std::f64::consts::PI as Float / 2.0);
        let result = cv1.multiply_conjugate(&cv2);

        // Result should have phase = (π/4 - π/2) = -π/4
        assert!((result.phase() + std::f64::consts::PI as Float / 4.0).abs() < 1e-10);
    }
}
