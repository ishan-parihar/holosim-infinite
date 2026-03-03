//! Field-to-Entity Extraction Pipeline
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "Field-to-entity extraction pipeline"
//!
//! This module implements the extraction of entities from the holographic field.
//! Entities are not created but MANIFESTED from pre-existing field configurations.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Reality is not constructed; it is Unfolded from a Pre-Existing Whole.
//!  The Entity does not 'become' something it was not—it REMEMBERS what it always was."
//!
//! The extraction pipeline:
//! 1. Identifies coherence peaks in the field (entity potentials)
//! 2. Evaluates archetype activation patterns
//! 3. Determines density and spectrum configuration
//! 4. Manifests entities with derived properties

use std::collections::HashMap;

use super::super::field_state::{CoherencePeak, HolographicFieldState, Position3D};
use super::super::scale_level::ScaleLevel;
use super::super::template::Density;
use crate::holographic_foundation::spectrum::SpectrumState;
use crate::types::Float;

/// Configuration for entity extraction
#[derive(Debug, Clone)]
pub struct EntityExtractionConfig {
    /// Minimum coherence threshold for extraction
    pub coherence_threshold: Float,
    /// Maximum entities to extract per pass
    pub max_entities_per_pass: usize,
    /// Minimum archetype activation for extraction
    pub min_archetype_activation: Float,
    /// Enable density-based extraction
    pub density_based: bool,
    /// Spectrum extraction mode
    pub spectrum_extraction: bool,
}

impl Default for EntityExtractionConfig {
    fn default() -> Self {
        Self {
            coherence_threshold: 0.3,
            max_entities_per_pass: 1000,
            min_archetype_activation: 0.1,
            density_based: true,
            spectrum_extraction: true,
        }
    }
}

/// Result of entity extraction
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// Extracted entities
    pub entities: Vec<ExtractedEntity>,
    /// Number of potentials evaluated
    pub potentials_evaluated: usize,
    /// Extraction time in microseconds
    pub extraction_time_us: u64,
    /// Field snapshot at extraction time
    pub field_snapshot: FieldSnapshot,
}

/// Snapshot of field state for visualization
#[derive(Debug, Clone)]
pub struct FieldSnapshot {
    /// Global coherence
    pub global_coherence: Float,
    /// Number of coherence peaks
    pub peak_count: usize,
    /// Average peak amplitude
    pub avg_amplitude: Float,
    /// Density distribution
    pub density_distribution: HashMap<u8, usize>,
    /// Spectrum position average
    pub avg_spectrum_position: Float,
    /// Veil transparency average
    pub avg_veil_transparency: Float,
    /// Timestamp
    pub timestamp: u64,
}

/// An extracted entity with all derived properties
#[derive(Debug, Clone)]
pub struct ExtractedEntity {
    /// Unique entity identifier
    pub entity_id: u64,
    /// Position in the field
    pub position: Position3D,
    /// Coherence at extraction point
    pub coherence: Float,
    /// Amplitude at extraction point
    pub amplitude: Float,
    /// Derived density
    pub density: Density,
    /// Spectrum state
    pub spectrum_state: SpectrumState,
    /// Archetype activation vector
    pub archetype_vector: [Float; 22],
    /// Dominant archetype index
    pub dominant_archetype: usize,
    /// Extraction timestamp
    pub extraction_time: u64,
}

/// Field-to-Entity Extraction Pipeline
pub struct EntityExtractionPipeline {
    /// Configuration
    config: EntityExtractionConfig,
    /// Next entity ID
    next_entity_id: u64,
    /// Extraction statistics
    statistics: ExtractionStatistics,
    /// Cached coherence peaks
    cached_peaks: Vec<CoherencePeak>,
}

#[derive(Debug, Clone, Default)]
struct ExtractionStatistics {
    total_extractions: u64,
    total_potentials_evaluated: u64,
    average_extraction_time_us: f64,
    density_distribution: HashMap<u8, u64>,
}

impl EntityExtractionPipeline {
    /// Create a new extraction pipeline
    pub fn new(config: EntityExtractionConfig) -> Self {
        Self {
            config,
            next_entity_id: 1,
            statistics: ExtractionStatistics::default(),
            cached_peaks: Vec::new(),
        }
    }

    /// Extract entities from a holographic field
    pub fn extract(
        &mut self,
        field_state: &HolographicFieldState,
        timestamp: u64,
    ) -> ExtractionResult {
        let start = std::time::Instant::now();

        let peaks = field_state.extract_coherence_peaks(self.config.coherence_threshold);
        self.cached_peaks = peaks.clone();

        let field_snapshot = self.create_field_snapshot(field_state, &peaks, timestamp);

        let mut entities = Vec::new();
        let mut extracted_count = 0;

        for peak in &peaks {
            if extracted_count >= self.config.max_entities_per_pass {
                break;
            }

            if peak.coherence < self.config.coherence_threshold {
                continue;
            }

            let max_archetype = peak
                .archetype_vector
                .iter()
                .cloned()
                .fold(0.0_f64, |max, val| max.max(val));
            if max_archetype < self.config.min_archetype_activation {
                continue;
            }

            let entity = self.extract_entity_from_peak(peak, timestamp);
            entities.push(entity);
            extracted_count += 1;
        }

        let extraction_time_us = start.elapsed().as_micros() as u64;

        self.statistics.total_extractions += entities.len() as u64;
        self.statistics.total_potentials_evaluated += peaks.len() as u64;
        self.statistics.average_extraction_time_us = (self.statistics.average_extraction_time_us
            * (self.statistics.total_extractions - entities.len() as u64) as f64
            + extraction_time_us as f64)
            / self.statistics.total_extractions as f64;

        for entity in &entities {
            let density = match entity.density {
                Density::First(_) => 1,
                Density::Second(_) => 2,
                Density::Third => 3,
                Density::Fourth => 4,
                Density::Fifth => 5,
                Density::Sixth => 6,
                Density::Seventh => 7,
                Density::Eighth => 8,
            };
            *self
                .statistics
                .density_distribution
                .entry(density)
                .or_insert(0) += 1;
        }

        ExtractionResult {
            entities,
            potentials_evaluated: peaks.len(),
            extraction_time_us,
            field_snapshot,
        }
    }

    /// Extract a single entity from a coherence peak
    fn extract_entity_from_peak(
        &mut self,
        peak: &CoherencePeak,
        timestamp: u64,
    ) -> ExtractedEntity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        let density = self.derive_density_from_scale(peak.dominant_scale);

        let spectrum_state = self.derive_spectrum_state(peak);

        let dominant_archetype = self.find_dominant_archetype(&peak.archetype_vector);

        ExtractedEntity {
            entity_id,
            position: peak.position,
            coherence: peak.coherence,
            amplitude: peak.amplitude,
            density,
            spectrum_state,
            archetype_vector: peak.archetype_vector,
            dominant_archetype,
            extraction_time: timestamp,
        }
    }

    /// Derive density from scale level
    fn derive_density_from_scale(&self, scale: ScaleLevel) -> Density {
        match scale {
            ScaleLevel::Quantum => Density::First(1),
            ScaleLevel::Atomic => Density::First(2),
            ScaleLevel::Molecular => Density::First(3),
            ScaleLevel::Cellular => Density::Second(1),
            ScaleLevel::Biological => Density::Third,
            ScaleLevel::Planetary => Density::Fourth,
            ScaleLevel::Stellar => Density::Fifth,
            ScaleLevel::Cosmic => Density::Eighth,
        }
    }

    /// Derive spectrum state from peak
    fn derive_spectrum_state(&self, peak: &CoherencePeak) -> SpectrumState {
        let coherence_factor = peak.coherence;

        let mut density_amplitudes = [num_complex::Complex::new(0.0, 0.0); 8];
        for i in 0..8 {
            let phase = (peak.archetype_vector[i * 2 % 22] * std::f64::consts::TAU).cos();
            let magnitude = coherence_factor * (1.0 - (i as f64 * 0.1));
            density_amplitudes[i] = num_complex::Complex::new(magnitude, phase);
        }

        let spectrum_position = match peak.dominant_scale {
            ScaleLevel::Quantum | ScaleLevel::Atomic | ScaleLevel::Molecular => 0.2,
            ScaleLevel::Cellular | ScaleLevel::Biological => 0.5,
            ScaleLevel::Planetary | ScaleLevel::Stellar => 0.8,
            ScaleLevel::Cosmic => 1.0,
        };

        let veil_transparency = coherence_factor * 0.5;

        let mut state = SpectrumState::new();
        state.density_amplitudes = density_amplitudes;
        state.spectrum_position =
            crate::holographic_foundation::spectrum::VelocityRatio::new(spectrum_position);
        state.veil_transparency = veil_transparency;
        state
    }

    /// Find dominant archetype index
    fn find_dominant_archetype(&self, archetype_vector: &[Float; 22]) -> usize {
        archetype_vector
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Create a field snapshot for visualization
    fn create_field_snapshot(
        &self,
        field_state: &HolographicFieldState,
        peaks: &[CoherencePeak],
        timestamp: u64,
    ) -> FieldSnapshot {
        let global_coherence = field_state.global_coherence();

        let avg_amplitude = if peaks.is_empty() {
            0.0
        } else {
            peaks.iter().map(|p| p.amplitude).sum::<Float>() / peaks.len() as Float
        };

        let mut density_distribution = HashMap::new();
        for peak in peaks {
            let density = peak.dominant_scale.to_density();
            *density_distribution.entry(density).or_insert(0) += 1;
        }

        let avg_spectrum_position = if peaks.is_empty() {
            0.5
        } else {
            peaks
                .iter()
                .map(|p| match p.dominant_scale {
                    ScaleLevel::Quantum | ScaleLevel::Atomic | ScaleLevel::Molecular => 0.2,
                    ScaleLevel::Cellular | ScaleLevel::Biological => 0.5,
                    ScaleLevel::Planetary | ScaleLevel::Stellar => 0.8,
                    ScaleLevel::Cosmic => 1.0,
                })
                .sum::<Float>()
                / peaks.len() as Float
        };

        let avg_veil_transparency = if peaks.is_empty() {
            0.0
        } else {
            peaks.iter().map(|p| p.coherence * 0.5).sum::<Float>() / peaks.len() as Float
        };

        FieldSnapshot {
            global_coherence,
            peak_count: peaks.len(),
            avg_amplitude,
            density_distribution,
            avg_spectrum_position,
            avg_veil_transparency,
            timestamp,
        }
    }

    /// Get extraction statistics
    pub fn statistics(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        stats.insert(
            "total_extractions".to_string(),
            self.statistics.total_extractions as f64,
        );
        stats.insert(
            "average_extraction_time_us".to_string(),
            self.statistics.average_extraction_time_us,
        );
        stats.insert(
            "total_potentials_evaluated".to_string(),
            self.statistics.total_potentials_evaluated as f64,
        );
        stats
    }

    /// Get cached peaks for visualization
    pub fn cached_peaks(&self) -> &[CoherencePeak] {
        &self.cached_peaks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_config_default() {
        let config = EntityExtractionConfig::default();
        assert_eq!(config.coherence_threshold, 0.3);
        assert_eq!(config.max_entities_per_pass, 1000);
    }

    #[test]
    fn test_extraction_pipeline_creation() {
        let config = EntityExtractionConfig::default();
        let pipeline = EntityExtractionPipeline::new(config);
        let stats = pipeline.statistics();
        assert_eq!(stats.get("total_extractions"), Some(&0.0));
    }

    #[test]
    fn test_derive_density_from_scale() {
        let config = EntityExtractionConfig::default();
        let pipeline = EntityExtractionPipeline::new(config);

        assert!(matches!(
            pipeline.derive_density_from_scale(ScaleLevel::Quantum),
            Density::First(_)
        ));
        assert!(matches!(
            pipeline.derive_density_from_scale(ScaleLevel::Biological),
            Density::Third
        ));
        assert!(matches!(
            pipeline.derive_density_from_scale(ScaleLevel::Cosmic),
            Density::Eighth
        ));
    }

    #[test]
    fn test_find_dominant_archetype() {
        let config = EntityExtractionConfig::default();
        let pipeline = EntityExtractionPipeline::new(config);

        let archetype_vector = [0.1; 22];
        let dominant = pipeline.find_dominant_archetype(&archetype_vector);
        assert!(dominant < 22);

        let mut specific_vector = [0.1; 22];
        specific_vector[10] = 0.9;
        let dominant = pipeline.find_dominant_archetype(&specific_vector);
        assert_eq!(dominant, 10);
    }

    #[test]
    fn test_field_snapshot_creation() {
        let config = EntityExtractionConfig::default();
        let pipeline = EntityExtractionPipeline::new(config);
        let field_state = HolographicFieldState::new(1.0);

        let snapshot = pipeline.create_field_snapshot(&field_state, &[], 0);

        assert_eq!(snapshot.peak_count, 0);
        assert_eq!(snapshot.timestamp, 0);
    }
}
