//! Emergent Attractors from Field Coherence Peaks
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.3:
//! "Create emergent attractors from coherence peaks"
//! "Attractors should emerge from field coherence peaks, not pre-defined formulas"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "The third distortion (Light) manifests as the field of potential"
//! > "Attractors emerge from coherence peaks in the holographic field"
//!
//! This module implements:
//! - Coherence peak detection in field
//! - Attractor spawning at coherence peaks
//! - Attractor strength based on coherence
//! - Spiritual gravity (Love/Logos as attractive force)

use crate::hpo::spatial_field::Position3D;
use std::collections::HashMap;

/// Configuration for emergent attractors
#[derive(Debug, Clone)]
pub struct EmergentAttractorsConfig {
    /// Minimum coherence to be considered a peak
    pub peak_threshold: f64,

    /// Minimum strength to spawn attractor
    pub spawn_threshold: f64,

    /// Maximum number of attractors
    pub max_attractors: usize,

    /// Coherence neighbor radius for peak detection
    pub neighbor_radius: f64,

    /// Enable spiritual gravity
    pub enable_spiritual_gravity: bool,

    /// Decay rate for attractors
    pub decay_rate: f64,
}

impl Default for EmergentAttractorsConfig {
    fn default() -> Self {
        EmergentAttractorsConfig {
            peak_threshold: 0.7,
            spawn_threshold: 0.3,
            max_attractors: 100,
            neighbor_radius: 10.0,
            enable_spiritual_gravity: true,
            decay_rate: 0.01,
        }
    }
}

/// A coherence peak in the field
#[derive(Debug, Clone)]
pub struct CoherencePeak {
    /// Position of the peak
    pub position: Position3D,

    /// Local coherence value
    pub coherence: f64,

    /// Strength relative to neighbors
    pub strength: f64,

    /// Time when peak was detected
    pub detection_time: usize,
}

/// Type of emergent attractor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergentAttractorType {
    /// Coherence peak attractor
    Coherence,
    /// Love/Logos as spiritual gravity
    SpiritualGravity,
    /// Evolution attractor (density progression)
    Evolution,
    /// Consciousness attractor
    Consciousness,
    /// Entity grouping attractor
    Collective,
}

/// Emergent attractor
#[derive(Debug, Clone)]
pub struct EmergentAttractor {
    /// Unique identifier
    pub id: u64,

    /// Position in space
    pub position: Position3D,

    /// Attractor strength
    pub strength: f64,

    /// Attractor type
    pub attractor_type: EmergentAttractorType,

    /// Range of influence
    pub influence_range: f64,

    /// Coherence at spawn time
    pub spawn_coherence: f64,

    /// Current decay factor
    pub decay_factor: f64,

    /// Is active
    pub active: bool,
}

/// Statistics for emergent attractors
#[derive(Debug, Clone, Default)]
pub struct EmergentAttractorStatistics {
    pub peaks_detected: usize,
    pub attractors_spawned: usize,
    pub attractors_active: usize,
    pub attractors_decayed: usize,
    pub average_strength: f64,
    pub coherence_distribution: Vec<usize>,
}

/// Emergent Attractors System
///
/// Spawns attractors at field coherence peaks instead of using formulas
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// > "Attractors emerge from field coherence peaks"
#[derive(Debug, Clone)]
pub struct EmergentAttractors {
    config: EmergentAttractorsConfig,
    attractors: HashMap<u64, EmergentAttractor>,
    next_id: u64,
    statistics: EmergentAttractorStatistics,
}

impl EmergentAttractors {
    pub fn new() -> Self {
        EmergentAttractors {
            config: EmergentAttractorsConfig::default(),
            attractors: HashMap::new(),
            next_id: 0,
            statistics: EmergentAttractorStatistics::default(),
        }
    }

    pub fn with_config(config: EmergentAttractorsConfig) -> Self {
        EmergentAttractors {
            config,
            attractors: HashMap::new(),
            next_id: 0,
            statistics: EmergentAttractorStatistics::default(),
        }
    }

    /// Detect coherence peaks in the field
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Field coherence creates attractor patterns"
    pub fn detect_coherence_peaks(
        &mut self,
        field_data: &[(Position3D, f64)], // (position, coherence)
        current_time: usize,
    ) -> Vec<CoherencePeak> {
        let mut peaks = Vec::new();

        for (i, (pos, coherence)) in field_data.iter().enumerate() {
            if *coherence < self.config.peak_threshold {
                continue;
            }

            // Check if local maximum
            let mut is_maximum = true;
            let mut neighbor_coherence_sum = 0.0;
            let mut neighbor_count = 0;

            for (j, (other_pos, other_coherence)) in field_data.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dx = pos.x - other_pos.x;
                let dy = pos.y - other_pos.y;
                let dz = pos.z - other_pos.z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();

                if distance <= self.config.neighbor_radius {
                    neighbor_coherence_sum += *other_coherence;
                    neighbor_count += 1;

                    if *other_coherence >= *coherence {
                        is_maximum = false;
                    }
                }
            }

            if is_maximum && neighbor_count > 0 {
                let avg_neighbor_coherence = neighbor_coherence_sum / neighbor_count as f64;
                let strength = *coherence - avg_neighbor_coherence;

                if strength >= self.config.spawn_threshold {
                    peaks.push(CoherencePeak {
                        position: *pos,
                        coherence: *coherence,
                        strength,
                        detection_time: current_time,
                    });
                }
            }
        }

        self.statistics.peaks_detected = peaks.len();
        peaks
    }

    /// Spawn attractor at coherence peak
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Love/Logos as attractive force (spiritual gravity)"
    pub fn spawn_attractor(&mut self, peak: &CoherencePeak) -> Option<u64> {
        // Check max attractors
        if self.attractors.len() >= self.config.max_attractors {
            return None;
        }

        let attractor_type = self.determine_attractor_type(peak.coherence);

        let attractor = EmergentAttractor {
            id: self.next_id,
            position: peak.position,
            strength: peak.strength,
            attractor_type,
            influence_range: peak.strength * 20.0, // Scale influence by strength
            spawn_coherence: peak.coherence,
            decay_factor: 1.0,
            active: true,
        };

        self.attractors.insert(self.next_id, attractor);
        self.next_id += 1;
        self.statistics.attractors_spawned += 1;

        Some(self.next_id - 1)
    }

    /// Determine attractor type based on coherence
    fn determine_attractor_type(&self, coherence: f64) -> EmergentAttractorType {
        match coherence {
            c if c >= 0.9 => EmergentAttractorType::Consciousness,
            c if c >= 0.8 => EmergentAttractorType::SpiritualGravity,
            c if c >= 0.7 => EmergentAttractorType::Evolution,
            _ => EmergentAttractorType::Coherence,
        }
    }

    /// Update attractors (decay)
    pub fn update(&mut self, dt: f64) {
        let mut to_remove = Vec::new();

        for (id, attractor) in &mut self.attractors {
            if !attractor.active {
                continue;
            }

            // Apply decay
            attractor.decay_factor -= self.config.decay_rate * dt;

            if attractor.decay_factor <= 0.0 {
                attractor.active = false;
                to_remove.push(*id);
                self.statistics.attractors_decayed += 1;
            } else {
                // Update strength based on decay
                attractor.strength = attractor.spawn_coherence * attractor.decay_factor;
            }
        }

        // Remove decayed attractors
        for id in to_remove {
            self.attractors.remove(&id);
        }

        self.update_statistics();
    }

    /// Get active attractors
    pub fn get_active_attractors(&self) -> Vec<&EmergentAttractor> {
        self.attractors.values().filter(|a| a.active).collect()
    }

    /// Get attractor force at position
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Spiritual gravity - Love/Logos as attractive force"
    pub fn get_force_at(&self, position: Position3D) -> (Position3D, f64) {
        let mut total_force = Position3D::new(0.0, 0.0, 0.0);
        let mut total_strength = 0.0;

        for attractor in self.get_active_attractors() {
            let dx = attractor.position.x - position.x;
            let dy = attractor.position.y - position.y;
            let dz = attractor.position.z - position.z;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            if distance < attractor.influence_range && distance > 0.1 {
                // Inverse square falloff
                let force_magnitude = attractor.strength / (distance * distance);

                total_force.x += (dx / distance) * force_magnitude;
                total_force.y += (dy / distance) * force_magnitude;
                total_force.z += (dz / distance) * force_magnitude;

                total_strength += attractor.strength;
            }
        }

        (total_force, total_strength)
    }

    /// Update statistics
    fn update_statistics(&mut self) {
        let active: Vec<_> = self.attractors.values().filter(|a| a.active).collect();

        self.statistics.attractors_active = active.len();

        if !active.is_empty() {
            let total_strength: f64 = active.iter().map(|a| a.strength).sum();
            self.statistics.average_strength = total_strength / active.len() as f64;
        }

        // Coherence distribution
        self.statistics.coherence_distribution = vec![0; 10];
        for attractor in &active {
            let idx = ((attractor.spawn_coherence * 10.0) as usize).min(9);
            self.statistics.coherence_distribution[idx] += 1;
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &EmergentAttractorStatistics {
        &self.statistics
    }

    /// Clear all attractors
    pub fn clear(&mut self) {
        self.attractors.clear();
        self.update_statistics();
    }
}

impl Default for EmergentAttractors {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherence_peak_detection() {
        let mut attractors = EmergentAttractors::new();

        // Create field data with clear peak
        let field_data = vec![
            (Position3D::new(0.0, 0.0, 0.0), 0.5), // Low
            (Position3D::new(1.0, 0.0, 0.0), 0.8), // Peak
            (Position3D::new(2.0, 0.0, 0.0), 0.5), // Low
        ];

        let peaks = attractors.detect_coherence_peaks(&field_data, 0);

        // Should detect the peak at (1.0, 0.0, 0.0)
        assert!(!peaks.is_empty());
    }

    #[test]
    fn test_attractor_spawning() {
        let mut attractors = EmergentAttractors::new();

        let peak = CoherencePeak {
            position: Position3D::new(0.0, 0.0, 0.0),
            coherence: 0.8,
            strength: 0.3,
            detection_time: 0,
        };

        let id = attractors.spawn_attractor(&peak);

        assert!(id.is_some());
        assert_eq!(attractors.attractors.len(), 1);
    }

    #[test]
    fn test_force_calculation() {
        let mut attractors = EmergentAttractors::new();

        // Spawn attractor
        let peak = CoherencePeak {
            position: Position3D::new(10.0, 10.0, 10.0),
            coherence: 0.9,
            strength: 1.0,
            detection_time: 0,
        };
        attractors.spawn_attractor(&peak);

        // Calculate force at origin
        let (force, strength) = attractors.get_force_at(Position3D::new(0.0, 0.0, 0.0));

        // Should have some force pointing toward attractor
        assert!(strength > 0.0);
    }
}
