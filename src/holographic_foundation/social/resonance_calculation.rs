//! Resonance Calculation - Phase alignment algorithms for resonance
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Phase alignment dot product - High resonance = strong connection potential"
//!
//! KEY INSIGHT: Resonance is computed from field configuration similarity,
//! NOT spatial proximity. Entities across the universe can resonate strongly.

use super::{ConnectionType, MIN_RESONANCE_THRESHOLD};
use crate::holographic_foundation::distortions::{DensityAmplitude, FieldState};
use crate::holographic_foundation::evolution::PatternSignature;
use crate::holographic_foundation::spectrum::DensityPosition;
use crate::types::Float;
use std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct PhaseAlignment {
    pub density_phases: [Float; 8],
    pub archetype_phases: [Float; 22],
    pub spectrum_phase: Float,
}

impl PhaseAlignment {
    pub fn new() -> Self {
        Self {
            density_phases: [0.0; 8],
            archetype_phases: [0.0; 22],
            spectrum_phase: 0.0,
        }
    }

    pub fn from_field_state(field: &FieldState) -> Self {
        let density_phases = field.density_amplitudes.map(|amp| amp.phase());
        Self {
            density_phases,
            archetype_phases: [0.0; 22],
            spectrum_phase: field.spectrum_position,
        }
    }

    pub fn from_density_amplitudes(amplitudes: &[DensityAmplitude; 8]) -> Self {
        Self {
            density_phases: amplitudes.map(|amp| amp.phase()),
            archetype_phases: [0.0; 22],
            spectrum_phase: 0.5,
        }
    }

    pub fn with_archetype_phases(mut self, phases: [Float; 22]) -> Self {
        self.archetype_phases = phases;
        self
    }

    pub fn with_spectrum_phase(mut self, phase: Float) -> Self {
        self.spectrum_phase = phase;
        self
    }

    pub fn phase_coherence(&self) -> Float {
        let mean: Float = self.density_phases.iter().sum::<Float>() / 8.0;
        let variance: Float = self
            .density_phases
            .iter()
            .map(|p| (p - mean).powi(2))
            .sum::<Float>()
            / 8.0;
        1.0 - (variance / TAU).min(1.0)
    }
}

impl Default for PhaseAlignment {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ResonanceResult {
    pub total_resonance: Float,
    pub density_resonance: Float,
    pub archetype_resonance: Float,
    pub spectrum_resonance: Float,
    pub polarity_alignment: Float,
    pub connection_type: ConnectionType,
}

impl ResonanceResult {
    pub fn new() -> Self {
        Self {
            total_resonance: 0.0,
            density_resonance: 0.0,
            archetype_resonance: 0.0,
            spectrum_resonance: 0.0,
            polarity_alignment: 0.0,
            connection_type: ConnectionType::Resonance,
        }
    }

    pub fn is_significant(&self) -> bool {
        self.total_resonance >= MIN_RESONANCE_THRESHOLD
    }

    pub fn is_strong(&self) -> bool {
        self.total_resonance >= 0.7
    }

    pub fn dominant_factor(&self) -> ConnectionType {
        if self.density_resonance >= self.archetype_resonance
            && self.density_resonance >= self.spectrum_resonance
        {
            ConnectionType::Density
        } else if self.archetype_resonance >= self.spectrum_resonance {
            ConnectionType::Archetype
        } else {
            ConnectionType::Resonance
        }
    }
}

impl Default for ResonanceResult {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResonanceCalculation {
    density_weight: Float,
    archetype_weight: Float,
    spectrum_weight: Float,
    polarity_weight: Float,
}

impl Default for ResonanceCalculation {
    fn default() -> Self {
        Self {
            density_weight: 0.4,
            archetype_weight: 0.3,
            spectrum_weight: 0.2,
            polarity_weight: 0.1,
        }
    }
}

impl ResonanceCalculation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_weights(
        density: Float,
        archetype: Float,
        spectrum: Float,
        polarity: Float,
    ) -> Self {
        let total = density + archetype + spectrum + polarity;
        Self {
            density_weight: density / total,
            archetype_weight: archetype / total,
            spectrum_weight: spectrum / total,
            polarity_weight: polarity / total,
        }
    }

    pub fn compute_resonance(
        &self,
        alignment_a: &PhaseAlignment,
        alignment_b: &PhaseAlignment,
    ) -> ResonanceResult {
        let density_resonance = self
            .compute_density_resonance(&alignment_a.density_phases, &alignment_b.density_phases);
        let archetype_resonance = self.compute_archetype_resonance(
            &alignment_a.archetype_phases,
            &alignment_b.archetype_phases,
        );
        let spectrum_resonance =
            self.compute_spectrum_resonance(alignment_a.spectrum_phase, alignment_b.spectrum_phase);

        let total_resonance = density_resonance * self.density_weight
            + archetype_resonance * self.archetype_weight
            + spectrum_resonance * self.spectrum_weight;

        ResonanceResult {
            total_resonance: total_resonance.clamp(0.0, 1.0),
            density_resonance,
            archetype_resonance,
            spectrum_resonance,
            polarity_alignment: 0.0,
            connection_type: ConnectionType::Resonance,
        }
    }

    pub fn compute_resonance_from_fields(
        &self,
        field_a: &FieldState,
        field_b: &FieldState,
    ) -> ResonanceResult {
        let alignment_a = PhaseAlignment::from_field_state(field_a);
        let alignment_b = PhaseAlignment::from_field_state(field_b);
        self.compute_resonance(&alignment_a, &alignment_b)
    }

    fn compute_density_resonance(&self, phases_a: &[Float; 8], phases_b: &[Float; 8]) -> Float {
        let mut dot_product = 0.0;
        let mut mag_a = 0.0;
        let mut mag_b = 0.0;

        for i in 0..8 {
            let cos_a = phases_a[i].cos();
            let sin_a = phases_a[i].sin();
            let cos_b = phases_b[i].cos();
            let sin_b = phases_b[i].sin();

            dot_product += cos_a * cos_b + sin_a * sin_b;
            mag_a += cos_a * cos_a + sin_a * sin_a;
            mag_b += cos_b * cos_b + sin_b * sin_b;
        }

        if mag_a > 0.0 && mag_b > 0.0 {
            (dot_product / (mag_a.sqrt() * mag_b.sqrt())).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn compute_archetype_resonance(&self, phases_a: &[Float; 22], phases_b: &[Float; 22]) -> Float {
        let mut dot_product = 0.0;
        let mut mag_a = 0.0;
        let mut mag_b = 0.0;

        for i in 0..22 {
            dot_product += (phases_a[i] - phases_b[i]).cos();
            mag_a += 1.0;
            mag_b += 1.0;
        }

        (dot_product / 22.0).clamp(0.0, 1.0)
    }

    fn compute_spectrum_resonance(&self, phase_a: Float, phase_b: Float) -> Float {
        let diff = (phase_a - phase_b).abs();
        (1.0 - diff.min(1.0)).powi(2)
    }

    pub fn compute_density_similarity(
        &self,
        density_a: &DensityPosition,
        density_b: &DensityPosition,
    ) -> Float {
        let diff = (density_a.value - density_b.value).abs();
        1.0 / (1.0 + diff)
    }

    pub fn compute_pattern_resonance(
        &self,
        pattern_a: &PatternSignature,
        pattern_b: &PatternSignature,
    ) -> Float {
        let density_resonance: Float = pattern_a
            .density_weights
            .iter()
            .zip(pattern_b.density_weights.iter())
            .map(|(a, b)| 1.0 - (a - b).abs())
            .sum::<Float>()
            / 8.0;

        let phase_resonance: Float = pattern_a
            .phase_pattern
            .iter()
            .zip(pattern_b.phase_pattern.iter())
            .map(|(a, b)| (a - b).cos())
            .sum::<Float>()
            / 8.0;

        (density_resonance * 0.6 + phase_resonance * 0.4).clamp(0.0, 1.0)
    }

    pub fn compute_polarity_alignment(&self, polarity_a: Float, polarity_b: Float) -> Float {
        let same_sign = polarity_a * polarity_b >= 0.0;
        let magnitude_similarity = 1.0 - (polarity_a.abs() - polarity_b.abs()).abs();

        if same_sign {
            magnitude_similarity.clamp(0.5, 1.0)
        } else {
            (1.0 - magnitude_similarity) * 0.3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_alignment_creation() {
        let alignment = PhaseAlignment::new();
        assert_eq!(alignment.density_phases, [0.0; 8]);
    }

    #[test]
    fn test_phase_alignment_from_field() {
        let field = FieldState::new();
        let alignment = PhaseAlignment::from_field_state(&field);

        for phase in alignment.density_phases.iter() {
            assert!(*phase >= -std::f64::consts::PI && *phase <= std::f64::consts::PI);
        }
    }

    #[test]
    fn test_resonance_result_significance() {
        let mut result = ResonanceResult::new();
        result.total_resonance = 0.5;

        assert!(result.is_significant());
        assert!(!result.is_strong());
    }

    #[test]
    fn test_compute_resonance() {
        let calc = ResonanceCalculation::new();
        let alignment_a = PhaseAlignment::new();
        let alignment_b = PhaseAlignment::new();

        let result = calc.compute_resonance(&alignment_a, &alignment_b);

        assert!(result.total_resonance >= 0.0);
    }

    #[test]
    fn test_density_resonance_identical() {
        let calc = ResonanceCalculation::new();
        let phases = [0.5; 8];

        let resonance = calc.compute_density_resonance(&phases, &phases);

        assert!((resonance - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_spectrum_resonance() {
        let calc = ResonanceCalculation::new();

        let same = calc.compute_spectrum_resonance(0.5, 0.5);
        let close = calc.compute_spectrum_resonance(0.5, 0.6);
        let far = calc.compute_spectrum_resonance(0.5, 1.5);

        assert!(same > close);
        assert!(close > far);
    }

    #[test]
    fn test_density_similarity() {
        let calc = ResonanceCalculation::new();

        let same =
            calc.compute_density_similarity(&DensityPosition::new(3.0), &DensityPosition::new(3.0));
        let close =
            calc.compute_density_similarity(&DensityPosition::new(3.0), &DensityPosition::new(3.5));
        let far =
            calc.compute_density_similarity(&DensityPosition::new(1.0), &DensityPosition::new(7.0));

        assert!(same > close);
        assert!(close > far);
    }

    #[test]
    fn test_polarity_alignment() {
        let calc = ResonanceCalculation::new();

        let sto_sto = calc.compute_polarity_alignment(1.0, 1.0);
        let sts_sts = calc.compute_polarity_alignment(-1.0, -1.0);
        let sto_sts = calc.compute_polarity_alignment(1.0, -1.0);

        assert!(sto_sto > sto_sts);
        assert!(sts_sts > sto_sts);
    }

    #[test]
    fn test_phase_coherence() {
        let coherent = PhaseAlignment {
            density_phases: [0.1; 8],
            archetype_phases: [0.0; 22],
            spectrum_phase: 0.5,
        };

        let incoherent = PhaseAlignment {
            density_phases: [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0],
            archetype_phases: [0.0; 22],
            spectrum_phase: 0.5,
        };

        assert!(coherent.phase_coherence() > incoherent.phase_coherence());
    }
}
