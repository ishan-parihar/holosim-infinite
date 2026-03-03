//! Dynamical Spectrum System
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 6.1:
//! "The spectrum is NOT static - it's a dynamical system"
//! "Entities move along the spectrum based on their evolution"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The spectrum is configured at galactic and solar scales before physical matter exists"
//! "v = s/t creates 3D space with 1D time"
//! "v = t/s creates 1D space with 3D time"
//!
//! This module implements:
//! - Spectrum evolution based on entity choices
//! - Integration with veil dynamics
//! - Density progression along the spectrum

use crate::evolution_density_octave::density_octave::Density;
use std::collections::HashMap;

/// Polarity orientation for spectrum evolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityOrientation {
    /// Service to Others - moves toward Time/Space (Oneness)
    ServiceToOthers,
    /// Service to Self - moves toward Space/Time (Many-ness)
    ServiceToSelf,
    /// Unpolarized
    Unpolarized,
}

/// Entity choice record
#[derive(Debug, Clone)]
pub struct EntityChoice {
    /// Entity ID
    pub entity_id: u64,
    /// Choice orientation
    pub orientation: PolarityOrientation,
    /// Choice magnitude (0-1)
    pub magnitude: f64,
    /// Step when choice was made
    pub step: usize,
}

/// Spectrum trajectory for an entity
#[derive(Debug, Clone)]
pub struct SpectrumTrajectory {
    /// Entity ID
    pub entity_id: u64,
    /// Current spectrum position
    pub current_position: f64,
    /// Velocity along spectrum
    pub velocity: f64,
    /// Target position (attractor)
    pub target_position: Option<f64>,
    /// Choice history
    pub choice_history: Vec<EntityChoice>,
    /// Movement history (for smoothing)
    pub position_history: Vec<f64>,
    /// Max history size
    pub max_history: usize,
}

impl SpectrumTrajectory {
    pub fn new(entity_id: u64, initial_position: f64) -> Self {
        SpectrumTrajectory {
            entity_id,
            current_position: initial_position,
            velocity: 0.0,
            target_position: None,
            choice_history: Vec::new(),
            position_history: vec![initial_position],
            max_history: 100,
        }
    }

    /// Record a choice and update trajectory
    pub fn record_choice(&mut self, orientation: PolarityOrientation, magnitude: f64, step: usize) {
        let choice = EntityChoice {
            entity_id: self.entity_id,
            orientation,
            magnitude,
            step,
        };

        self.choice_history.push(choice);

        // Update velocity based on choice
        let polarity_shift = match orientation {
            PolarityOrientation::ServiceToOthers => magnitude,
            PolarityOrientation::ServiceToSelf => -magnitude,
            PolarityOrientation::Unpolarized => 0.0,
        };

        self.velocity += polarity_shift * 0.1;

        // Clamp velocity
        self.velocity = self.velocity.clamp(-0.5, 0.5);
    }

    /// Update position based on velocity
    pub fn update(&mut self, dt: f64) {
        // Apply velocity with smoothing
        let new_position = self.current_position + self.velocity * dt;
        self.current_position = new_position.clamp(0.0, 2.0); // Clamp to valid range

        // Add to history
        self.position_history.push(self.current_position);
        while self.position_history.len() > self.max_history {
            self.position_history.remove(0);
        }

        // Apply friction to velocity
        self.velocity *= 0.95;
    }

    /// Get average position from history (smoothed)
    pub fn get_smoothed_position(&self) -> f64 {
        if self.position_history.is_empty() {
            return self.current_position;
        }

        // Weighted average (recent positions weighted more)
        let len = self.position_history.len();
        let mut sum = 0.0;
        let mut weight_sum = 0.0;

        for (i, pos) in self.position_history.iter().enumerate() {
            let weight = (i + 1) as f64;
            sum += pos * weight;
            weight_sum += weight;
        }

        sum / weight_sum
    }
}

/// Configuration for dynamical spectrum
#[derive(Debug, Clone)]
pub struct DynamicalSpectrumConfig {
    /// Base evolution rate
    pub evolution_rate: f64,
    /// Choice influence factor
    pub choice_influence: f64,
    /// Velocity damping
    pub damping: f64,
    /// Minimum spectrum position
    pub min_position: f64,
    /// Maximum spectrum position  
    pub max_position: f64,
    /// Veil position (spectrum = 1.0)
    pub veil_position: f64,
}

impl Default for DynamicalSpectrumConfig {
    fn default() -> Self {
        DynamicalSpectrumConfig {
            evolution_rate: 0.01,
            choice_influence: 0.1,
            damping: 0.95,
            min_position: 0.0,
            max_position: 2.0,
            veil_position: 1.0,
        }
    }
}

/// Dynamical Spectrum System
///
/// Evolves spectrum positions based on entity choices
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Entities move along the spectrum based on their evolution"
#[derive(Debug, Clone)]
pub struct DynamicalSpectrum {
    config: DynamicalSpectrumConfig,
    trajectories: HashMap<u64, SpectrumTrajectory>,
    statistics: DynamicalSpectrumStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct DynamicalSpectrumStatistics {
    pub total_entities: usize,
    pub average_position: f64,
    pub average_velocity: f64,
    pub below_veil_count: usize,
    pub above_veil_count: usize,
    pub transitions_to_sto: usize,
    pub transitions_to_sts: usize,
}

impl DynamicalSpectrum {
    pub fn new() -> Self {
        DynamicalSpectrum {
            config: DynamicalSpectrumConfig::default(),
            trajectories: HashMap::new(),
            statistics: DynamicalSpectrumStatistics::default(),
        }
    }

    pub fn with_config(config: DynamicalSpectrumConfig) -> Self {
        DynamicalSpectrum {
            config,
            trajectories: HashMap::new(),
            statistics: DynamicalSpectrumStatistics::default(),
        }
    }

    /// Initialize entity in spectrum
    pub fn init_entity(&mut self, entity_id: u64, initial_position: f64) {
        let trajectory = SpectrumTrajectory::new(entity_id, initial_position);
        self.trajectories.insert(entity_id, trajectory);
    }

    /// Record entity choice and update spectrum position
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Choices affect spectrum movement"
    pub fn record_choice(
        &mut self,
        entity_id: u64,
        orientation: PolarityOrientation,
        magnitude: f64,
        step: usize,
    ) {
        if let Some(trajectory) = self.trajectories.get_mut(&entity_id) {
            let previous_orientation = trajectory.choice_history.last().map(|c| c.orientation);

            trajectory.record_choice(orientation, magnitude, step);

            // Track polarity transitions
            if let Some(prev) = previous_orientation {
                if prev != orientation {
                    match orientation {
                        PolarityOrientation::ServiceToOthers => {
                            self.statistics.transitions_to_sto += 1
                        }
                        PolarityOrientation::ServiceToSelf => {
                            self.statistics.transitions_to_sts += 1
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Update all entity spectrum positions
    pub fn update(&mut self, dt: f64) {
        for trajectory in self.trajectories.values_mut() {
            trajectory.update(dt);
        }

        self.update_statistics();
    }

    /// Get spectrum position for entity
    pub fn get_position(&self, entity_id: u64) -> Option<f64> {
        self.trajectories
            .get(&entity_id)
            .map(|t| t.current_position)
    }

    /// Get smoothed spectrum position for entity
    pub fn get_smoothed_position(&self, entity_id: u64) -> Option<f64> {
        self.trajectories
            .get(&entity_id)
            .map(|t| t.get_smoothed_position())
    }

    /// Get veil transparency for entity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Below veil (v < 1): opaque"
    /// > "Above veil (v > 1): transparency increases"
    pub fn get_veil_transparency(&self, entity_id: u64) -> f64 {
        let position = self.get_position(entity_id).unwrap_or(0.0);
        self.calculate_transparency(position)
    }

    /// Calculate transparency from spectrum position
    fn calculate_transparency(&self, position: f64) -> f64 {
        if position < self.config.veil_position {
            // Below veil - opaque
            0.0
        } else {
            // Above veil - transparency increases
            let above_veil = position - self.config.veil_position;
            (above_veil / (self.config.max_position - self.config.veil_position)).min(1.0)
        }
    }

    /// Get density from spectrum position
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Each density corresponds to spectrum range"
    pub fn get_density(&self, entity_id: u64) -> Option<Density> {
        let position = self.get_position(entity_id)?;

        let density = match position as usize {
            0..=10 => Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            11..=20 => Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
            ),
            21..=30 => Density::Third,
            31..=40 => Density::Fourth,
            41..=50 => Density::Fifth,
            51..=60 => Density::Sixth,
            61..=70 => Density::Seventh,
            _ => Density::Eighth,
        };

        Some(density)
    }

    /// Check if entity is ready for density transition
    pub fn check_density_transition(&self, entity_id: u64, current_density: Density) -> bool {
        if let Some(target_density) = self.get_density(entity_id) {
            target_density != current_density && self.is_ready_for_transition(entity_id)
        } else {
            false
        }
    }

    /// Check if entity is ready for transition
    fn is_ready_for_transition(&self, entity_id: u64) -> bool {
        if let Some(trajectory) = self.trajectories.get(&entity_id) {
            // Ready if stable at new position
            trajectory.velocity.abs() < 0.01
        } else {
            false
        }
    }

    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_entities = self.trajectories.len();

        if self.trajectories.is_empty() {
            return;
        }

        let mut total_pos = 0.0;
        let mut total_vel = 0.0;
        let mut below_veil = 0;
        let mut above_veil = 0;

        for trajectory in self.trajectories.values() {
            total_pos += trajectory.current_position;
            total_vel += trajectory.velocity.abs();

            if trajectory.current_position < self.config.veil_position {
                below_veil += 1;
            } else {
                above_veil += 1;
            }
        }

        let count = self.trajectories.len() as f64;
        self.statistics.average_position = total_pos / count;
        self.statistics.average_velocity = total_vel / count;
        self.statistics.below_veil_count = below_veil;
        self.statistics.above_veil_count = above_veil;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &DynamicalSpectrumStatistics {
        &self.statistics
    }

    /// Remove entity
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.trajectories.remove(&entity_id);
    }
}

impl Default for DynamicalSpectrum {
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
    fn test_entity_initialization() {
        let mut spectrum = DynamicalSpectrum::new();

        spectrum.init_entity(1, 0.5);

        let pos = spectrum.get_position(1);
        assert!(pos.is_some());
        assert_eq!(pos.unwrap(), 0.5);
    }

    #[test]
    fn test_choice_affects_position() {
        let mut spectrum = DynamicalSpectrum::new();

        spectrum.init_entity(1, 0.5);

        // Record Service-to-Others choice
        spectrum.record_choice(1, PolarityOrientation::ServiceToOthers, 1.0, 0);

        // Update
        spectrum.update(1.0);

        // Position should have moved toward Time/Space (higher)
        let pos = spectrum.get_position(1);
        assert!(pos.is_some());
        assert!(pos.unwrap() > 0.5);
    }

    #[test]
    fn test_veil_transparency() {
        let spectrum = DynamicalSpectrum::new();

        // Below veil - should be opaque
        let below = spectrum.calculate_transparency(0.5);
        assert_eq!(below, 0.0);

        // Above veil - should increase
        let above = spectrum.calculate_transparency(1.5);
        assert!(above > 0.0);
    }

    #[test]
    fn test_density_mapping() {
        let mut spectrum = DynamicalSpectrum::new();

        spectrum.init_entity(1, 0.5);

        let density = spectrum.get_density(1);
        assert!(density.is_some());
    }
}
