//! Unified Archetype Activation Profile
//!
//! This is the CANONICAL archetype activation definition that provides:
//! - 22 archetype coefficients representing the full archetype system
//! - Mind/Body/Spirit complex accessors
//! - Coherence and entropy calculations
//! - Compression through archetype basis projection
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The 22-Archetype System:
//!  - A1-A7: Mind complex (Matrix, Potentiator, Catalyst, Experience, Significator, Transformation, Great Way)
//!  - A8-A14: Body complex (same 7-archetype structure)
//!  - A15-A21: Spirit complex (same 7-archetype structure)
//!  - A22: The Choice - culmination of the system, the fundamental choice of polarity"
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "The 22 archetypes form an orthogonal basis set. Any archetype activation profile
//!  is a linear combination. Compression: Store 22 floats (88 bytes), Reconstruct from
//!  22 multiply-add operations. Original pattern: Thousands of floats. Compression ratio: ~100x"

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::types::Float;

pub const NUM_ARCHETYPES: usize = 22;
pub const PROFILE_SIZE_BYTES: usize = NUM_ARCHETYPES * std::mem::size_of::<Float>();

#[derive(Debug, Clone)]
pub struct ArchetypeActivationProfile {
    coefficients: [Float; NUM_ARCHETYPES],
}

impl PartialEq for ArchetypeActivationProfile {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..NUM_ARCHETYPES {
            if (self.coefficients[i] - other.coefficients[i]).abs() >= 1e-10 {
                return false;
            }
        }
        true
    }
}

impl Eq for ArchetypeActivationProfile {}

impl Hash for ArchetypeActivationProfile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for coeff in &self.coefficients {
            coeff.to_bits().hash(state);
        }
    }
}

impl Default for ArchetypeActivationProfile {
    fn default() -> Self {
        Self {
            coefficients: [0.5; NUM_ARCHETYPES],
        }
    }
}

impl ArchetypeActivationProfile {
    pub fn new(coefficients: [Float; NUM_ARCHETYPES]) -> Self {
        Self {
            coefficients: coefficients.map(|c| c.clamp(0.0, 1.0)),
        }
    }

    /// Create an initial profile (alias for default for backward compatibility)
    pub fn initial() -> Self {
        Self::default()
    }

    pub fn zero() -> Self {
        Self {
            coefficients: [0.0; NUM_ARCHETYPES],
        }
    }

    pub fn unity() -> Self {
        Self {
            coefficients: [1.0; NUM_ARCHETYPES],
        }
    }

    pub fn coefficients(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.coefficients
    }

    pub fn coefficients_mut(&mut self) -> &mut [Float; NUM_ARCHETYPES] {
        &mut self.coefficients
    }

    pub fn get_activation(&self, archetype_number: usize) -> Option<Float> {
        if (1..=22).contains(&archetype_number) {
            Some(self.coefficients[archetype_number - 1])
        } else {
            None
        }
    }

    pub fn set_activation(&mut self, archetype_number: usize, activation: Float) {
        if (1..=22).contains(&archetype_number) {
            self.coefficients[archetype_number - 1] = activation.clamp(0.0, 1.0);
        }
    }

    pub fn get(&self, index: usize) -> Option<Float> {
        self.coefficients.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: Float) -> Result<(), ArchetypeProfileError> {
        if index >= NUM_ARCHETYPES {
            return Err(ArchetypeProfileError::InvalidIndex(index));
        }
        self.coefficients[index] = value.clamp(0.0, 1.0);
        Ok(())
    }

    pub fn mind_complex(&self) -> [Float; 7] {
        [
            self.coefficients[0],
            self.coefficients[1],
            self.coefficients[2],
            self.coefficients[3],
            self.coefficients[4],
            self.coefficients[5],
            self.coefficients[6],
        ]
    }

    pub fn body_complex(&self) -> [Float; 7] {
        [
            self.coefficients[7],
            self.coefficients[8],
            self.coefficients[9],
            self.coefficients[10],
            self.coefficients[11],
            self.coefficients[12],
            self.coefficients[13],
        ]
    }

    pub fn spirit_complex(&self) -> [Float; 7] {
        [
            self.coefficients[14],
            self.coefficients[15],
            self.coefficients[16],
            self.coefficients[17],
            self.coefficients[18],
            self.coefficients[19],
            self.coefficients[20],
        ]
    }

    pub fn choice(&self) -> Float {
        self.coefficients[21]
    }

    pub fn set_choice(&mut self, activation: Float) {
        self.coefficients[21] = activation.clamp(0.0, 1.0);
    }

    pub fn coherence(&self) -> Float {
        let mean: Float = self.coefficients.iter().sum::<Float>() / NUM_ARCHETYPES as Float;
        let variance: Float = self
            .coefficients
            .iter()
            .map(|c| (c - mean).powi(2))
            .sum::<Float>()
            / NUM_ARCHETYPES as Float;
        1.0 - variance.sqrt()
    }

    pub fn normalize(&mut self) {
        let sum: Float = self.coefficients.iter().sum();
        if sum > 1e-10 {
            for coeff in self.coefficients.iter_mut() {
                *coeff /= sum;
            }
        }
    }

    pub fn is_normalized(&self, tolerance: Float) -> bool {
        let sum: Float = self.coefficients.iter().sum();
        (sum - 1.0).abs() < tolerance
    }

    pub fn entropy(&self) -> Float {
        let mut entropy = 0.0;
        for &coeff in self.coefficients.iter() {
            if coeff > 1e-10 {
                entropy -= coeff * coeff.ln();
            }
        }
        entropy
    }

    pub fn dominant_archetype(&self) -> (usize, Float) {
        self.coefficients
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, &v)| (i + 1, v))
            .unwrap_or((1, 0.5))
    }

    pub fn mind_dominance(&self) -> Float {
        let mind: Float = self.mind_complex().iter().sum();
        let total: Float = self.coefficients.iter().sum();
        if total > 1e-10 {
            mind / total
        } else {
            0.0
        }
    }

    pub fn body_dominance(&self) -> Float {
        let body: Float = self.body_complex().iter().sum();
        let total: Float = self.coefficients.iter().sum();
        if total > 1e-10 {
            body / total
        } else {
            0.0
        }
    }

    pub fn spirit_dominance(&self) -> Float {
        let spirit: Float = self.spirit_complex().iter().sum();
        let total: Float = self.coefficients.iter().sum();
        if total > 1e-10 {
            spirit / total
        } else {
            0.0
        }
    }

    pub fn polarization(&self) -> Float {
        let choice = self.choice();
        if choice > 0.5 {
            (choice - 0.5) * 2.0
        } else {
            (0.5 - choice) * -2.0
        }
    }

    pub fn size_bytes(&self) -> usize {
        PROFILE_SIZE_BYTES
    }

    pub fn archetype_name(index: usize) -> Option<&'static str> {
        match index {
            0 => Some("Matrix (Mind)"),
            1 => Some("Potentiator (Mind)"),
            2 => Some("Catalyst (Mind)"),
            3 => Some("Experience (Mind)"),
            4 => Some("Significator (Mind)"),
            5 => Some("Transformation (Mind)"),
            6 => Some("Great Way (Mind)"),
            7 => Some("Matrix (Body)"),
            8 => Some("Potentiator (Body)"),
            9 => Some("Catalyst (Body)"),
            10 => Some("Experience (Body)"),
            11 => Some("Significator (Body)"),
            12 => Some("Transformation (Body)"),
            13 => Some("Great Way (Body)"),
            14 => Some("Matrix (Spirit)"),
            15 => Some("Potentiator (Spirit)"),
            16 => Some("Catalyst (Spirit)"),
            17 => Some("Experience (Spirit)"),
            18 => Some("Significator (Spirit)"),
            19 => Some("Transformation (Spirit)"),
            20 => Some("Great Way (Spirit)"),
            21 => Some("The Choice"),
            _ => None,
        }
    }
}

impl fmt::Display for ArchetypeActivationProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Archetype Activation Profile:")?;
        writeln!(f, "  Mind Complex:   {:?}", self.mind_complex())?;
        writeln!(f, "  Body Complex:   {:?}", self.body_complex())?;
        writeln!(f, "  Spirit Complex: {:?}", self.spirit_complex())?;
        writeln!(f, "  The Choice:     {:.3}", self.choice())?;
        writeln!(f, "  Coherence:      {:.3}", self.coherence())?;
        writeln!(f, "  Polarization:   {:.3}", self.polarization())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArchetypeProfileError {
    InvalidIndex(usize),
    InvalidCoefficient(Float),
}

impl fmt::Display for ArchetypeProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchetypeProfileError::InvalidIndex(i) => write!(f, "Invalid archetype index: {}", i),
            ArchetypeProfileError::InvalidCoefficient(v) => {
                write!(f, "Invalid coefficient value: {}", v)
            }
        }
    }
}

impl std::error::Error for ArchetypeProfileError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeComplex {
    Mind,
    Body,
    Spirit,
    Choice,
}

impl ArchetypeComplex {
    pub fn archetype_range(&self) -> std::ops::Range<usize> {
        match self {
            ArchetypeComplex::Mind => 0..7,
            ArchetypeComplex::Body => 7..14,
            ArchetypeComplex::Spirit => 14..21,
            ArchetypeComplex::Choice => 21..22,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile() {
        let profile = ArchetypeActivationProfile::initial();
        assert_eq!(profile.coherence(), 1.0);
    }

    #[test]
    fn test_complex_accessors() {
        let mut profile = ArchetypeActivationProfile::zero();
        profile.set_activation(1, 0.8);
        profile.set_activation(8, 0.6);
        profile.set_activation(15, 0.4);
        profile.set_activation(22, 0.9);

        assert_eq!(profile.mind_complex()[0], 0.8);
        assert_eq!(profile.body_complex()[0], 0.6);
        assert_eq!(profile.spirit_complex()[0], 0.4);
        assert_eq!(profile.choice(), 0.9);
    }

    #[test]
    fn test_coherence_calculation() {
        let profile = ArchetypeActivationProfile::new([1.0; 22]);
        assert!((profile.coherence() - 1.0).abs() < 0.01);

        let varied = ArchetypeActivationProfile::new([
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.1, 0.2, 0.3,
            0.4, 0.5, 0.6, 0.7, 0.5,
        ]);
        assert!(varied.coherence() < 1.0);
    }

    #[test]
    fn test_dominant_archetype() {
        let mut profile = ArchetypeActivationProfile::initial();
        profile.set_activation(22, 1.0);
        let (index, value) = profile.dominant_archetype();
        assert_eq!(index, 22);
        assert_eq!(value, 1.0);
    }

    #[test]
    fn test_polarization() {
        let mut profile = ArchetypeActivationProfile::initial();
        profile.set_choice(1.0);
        assert!((profile.polarization() - 1.0).abs() < 0.01);

        profile.set_choice(0.0);
        assert!((profile.polarization() - (-1.0)).abs() < 0.01);

        profile.set_choice(0.5);
        assert!(profile.polarization().abs() < 0.01);
    }

    #[test]
    fn test_archetype_names() {
        assert_eq!(
            ArchetypeActivationProfile::archetype_name(0),
            Some("Matrix (Mind)")
        );
        assert_eq!(
            ArchetypeActivationProfile::archetype_name(21),
            Some("The Choice")
        );
        assert_eq!(ArchetypeActivationProfile::archetype_name(22), None);
    }
}
