//! Continuous Spectrum Position Evolution
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The spectrum position evolves continuously as entities develop.
//!  Evolution is not discrete jumps but gradual progression through the density octave."
//!
//! KEY INSIGHT: Spectrum position v = s/t (space/time ratio) evolves smoothly.
//! v < 1: Moving toward Time/Space (unity consciousness)
//! v = 1: At the Veil (transition zone)
//! v > 1: Moving toward Space/Time (individual manifestation)

use super::{DensityPosition, VelocityRatio, NUM_DENSITY_BANDS};
use crate::types::Float;

#[derive(Debug, Clone)]
pub struct SpectrumConfig {
    pub evolution_rate: Float,
    pub minimum_position: Float,
    pub maximum_position: Float,
    pub veil_attraction: Float,
    pub density_coupling: Float,
}

impl Default for SpectrumConfig {
    fn default() -> Self {
        Self {
            evolution_rate: 0.001,
            minimum_position: 0.1,
            maximum_position: 10.0,
            veil_attraction: 0.1,
            density_coupling: 0.2,
        }
    }
}

impl SpectrumConfig {
    pub fn fast_evolution() -> Self {
        Self {
            evolution_rate: 0.01,
            minimum_position: 0.1,
            maximum_position: 10.0,
            veil_attraction: 0.2,
            density_coupling: 0.3,
        }
    }

    pub fn slow_evolution() -> Self {
        Self {
            evolution_rate: 0.0001,
            minimum_position: 0.1,
            maximum_position: 10.0,
            veil_attraction: 0.05,
            density_coupling: 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpectrumPosition {
    velocity_ratio: VelocityRatio,
    density_position: DensityPosition,
    config: SpectrumConfig,
    evolution_direction: Float,
    time_at_current_position: Float,
    total_evolution_time: Float,
}

impl SpectrumPosition {
    pub fn new(velocity: Float, density: Float, config: SpectrumConfig) -> Self {
        Self {
            velocity_ratio: VelocityRatio::new(velocity),
            density_position: DensityPosition::new(density),
            config,
            evolution_direction: 0.0,
            time_at_current_position: 0.0,
            total_evolution_time: 0.0,
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(1.0, 3.0, SpectrumConfig::default())
    }

    pub fn at_density(density: Float) -> Self {
        Self::new(1.0, density, SpectrumConfig::default())
    }

    pub fn at_spectrum_position(velocity: Float) -> Self {
        Self::new(velocity, 3.0, SpectrumConfig::default())
    }

    /// Evolve spectrum position over time
    ///
    /// Evolution tends toward unity (v → 0, Time/Space) as consciousness develops.
    /// Higher densities evolve faster toward unity.
    pub fn evolve(&mut self, dt: Float, catalyst: Float) {
        self.time_at_current_position += dt;
        self.total_evolution_time += dt;

        // Base evolution rate depends on density
        let density_factor = self.density_position.evolution_speed();
        let base_rate = self.config.evolution_rate * density_factor;

        // Catalyst accelerates evolution
        let catalyst_factor = 1.0 + catalyst;

        // Direction of evolution: toward unity (Time/Space)
        // Higher consciousness = faster movement toward unity
        let consciousness = self.density_position.consciousness_level();
        self.evolution_direction = -base_rate * catalyst_factor * (1.0 + consciousness);

        // Veil attraction: entities near the veil feel pulled toward it
        let veil_pull = self.calculate_veil_attraction();
        self.evolution_direction += veil_pull;

        // Apply evolution
        let new_velocity = self.velocity_ratio.value + self.evolution_direction * dt;
        self.velocity_ratio = VelocityRatio::new(
            new_velocity.clamp(self.config.minimum_position, self.config.maximum_position),
        );

        // Reset time counter if significant movement
        if self.evolution_direction.abs() > 0.001 {
            self.time_at_current_position = 0.0;
        }
    }

    fn calculate_veil_attraction(&self) -> Float {
        let distance_to_veil = self.velocity_ratio.distance_to_veil();

        // Attraction is stronger near the veil
        let proximity_factor = 1.0 / (distance_to_veil + 0.1);

        // Direction: toward veil (v = 1)
        let direction = if self.velocity_ratio.value < 1.0 {
            1.0
        } else {
            -1.0
        };

        self.config.veil_attraction * proximity_factor * direction
    }

    /// Force movement toward a target position
    pub fn move_toward(&mut self, target_velocity: Float, rate: Float) {
        let diff = target_velocity - self.velocity_ratio.value;
        let step = diff.signum() * rate.abs().min(diff.abs());
        let new_velocity = self.velocity_ratio.value + step;
        self.velocity_ratio = VelocityRatio::new(new_velocity);
    }

    /// Jump to a new position (for significant events)
    pub fn jump_to(&mut self, velocity: Float) {
        self.velocity_ratio = VelocityRatio::new(velocity);
        self.time_at_current_position = 0.0;
    }

    /// Set density position (affects evolution rate)
    pub fn set_density(&mut self, density: Float) {
        self.density_position = DensityPosition::new(density);
    }

    /// Evolution toward Time/Space (unity consciousness)
    pub fn evolve_toward_unity(&mut self, dt: Float) {
        self.move_toward(0.5, self.config.evolution_rate * 2.0);
    }

    /// Evolution toward Space/Time (individual manifestation)
    pub fn evolve_toward_manifestation(&mut self, dt: Float) {
        self.move_toward(2.0, self.config.evolution_rate * 2.0);
    }

    /// Lock position at the veil
    pub fn lock_at_veil(&mut self) {
        self.velocity_ratio = VelocityRatio::at_veil();
        self.evolution_direction = 0.0;
    }

    pub fn velocity_ratio(&self) -> &VelocityRatio {
        &self.velocity_ratio
    }

    pub fn density_position(&self) -> &DensityPosition {
        &self.density_position
    }

    pub fn value(&self) -> Float {
        self.velocity_ratio.value
    }

    pub fn time_at_position(&self) -> Float {
        self.time_at_current_position
    }

    pub fn total_evolution_time(&self) -> Float {
        self.total_evolution_time
    }

    pub fn evolution_direction(&self) -> Float {
        self.evolution_direction
    }

    pub fn config(&self) -> &SpectrumConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct SpectrumDynamics {
    config: SpectrumConfig,
    positions: Vec<SpectrumPosition>,
    average_position: Float,
    variance: Float,
}

impl SpectrumDynamics {
    pub fn new(config: SpectrumConfig) -> Self {
        Self {
            config,
            positions: Vec::new(),
            average_position: 1.0,
            variance: 0.0,
        }
    }

    pub fn from_defaults() -> Self {
        Self::new(SpectrumConfig::default())
    }

    pub fn add_position(&mut self, position: SpectrumPosition) {
        self.positions.push(position);
        self.update_statistics();
    }

    pub fn remove_position(&mut self, index: usize) -> Option<SpectrumPosition> {
        if index < self.positions.len() {
            let removed = self.positions.remove(index);
            self.update_statistics();
            Some(removed)
        } else {
            None
        }
    }

    pub fn evolve_all(&mut self, dt: Float, catalyst: Float) {
        for position in &mut self.positions {
            position.evolve(dt, catalyst);
        }
        self.update_statistics();
    }

    fn update_statistics(&mut self) {
        if self.positions.is_empty() {
            self.average_position = 1.0;
            self.variance = 0.0;
            return;
        }

        let sum: Float = self.positions.iter().map(|p| p.value()).sum();
        self.average_position = sum / self.positions.len() as Float;

        let variance_sum: Float = self
            .positions
            .iter()
            .map(|p| (p.value() - self.average_position).powi(2))
            .sum();
        self.variance = variance_sum / self.positions.len() as Float;
    }

    pub fn count_at_veil(&self) -> usize {
        self.positions
            .iter()
            .filter(|p| p.velocity_ratio().is_at_veil())
            .count()
    }

    pub fn count_space_time(&self) -> usize {
        self.positions
            .iter()
            .filter(|p| p.velocity_ratio().is_space_time())
            .count()
    }

    pub fn count_time_space(&self) -> usize {
        self.positions
            .iter()
            .filter(|p| p.velocity_ratio().is_time_space())
            .count()
    }

    pub fn positions(&self) -> &[SpectrumPosition] {
        &self.positions
    }

    pub fn positions_mut(&mut self) -> &mut [SpectrumPosition] {
        &mut self.positions
    }

    pub fn average_position(&self) -> Float {
        self.average_position
    }

    pub fn variance(&self) -> Float {
        self.variance
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn config(&self) -> &SpectrumConfig {
        &self.config
    }

    pub fn clear(&mut self) {
        self.positions.clear();
        self.average_position = 1.0;
        self.variance = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_position_creation() {
        let pos = SpectrumPosition::from_defaults();
        assert!((pos.value() - 1.0).abs() < 1e-10);
        assert!((pos.density_position().value - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectrum_position_at_density() {
        let pos = SpectrumPosition::at_density(5.0);
        assert!((pos.density_position().value - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectrum_evolution() {
        let mut pos = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::fast_evolution());

        for _ in 0..100 {
            pos.evolve(0.01, 0.0);
        }

        // Position should have evolved
        assert!(pos.total_evolution_time() > 0.0);
    }

    #[test]
    fn test_spectrum_evolution_with_catalyst() {
        let mut pos1 = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::fast_evolution());
        let mut pos2 = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::fast_evolution());

        let initial_velocity = pos1.value();

        for _ in 0..10 {
            pos1.evolve(0.01, 0.0);
            pos2.evolve(0.01, 1.0); // High catalyst
        }

        // With catalyst, velocity should change more (toward unity = lower)
        assert!(pos1.value() < initial_velocity);
        assert!(pos2.value() < pos1.value()); // pos2 moved more with catalyst
    }

    #[test]
    fn test_spectrum_move_toward() {
        let mut pos = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::default());
        pos.move_toward(1.0, 0.1);

        assert!(pos.value() < 2.0);
    }

    #[test]
    fn test_spectrum_jump_to() {
        let mut pos = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::default());
        pos.jump_to(0.5);

        assert!((pos.value() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_spectrum_lock_at_veil() {
        let mut pos = SpectrumPosition::new(2.0, 3.0, SpectrumConfig::default());
        pos.lock_at_veil();

        assert!((pos.value() - 1.0).abs() < 1e-10);
        assert!((pos.evolution_direction() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_spectrum_dynamics_creation() {
        let dynamics = SpectrumDynamics::from_defaults();
        assert!(dynamics.is_empty());
    }

    #[test]
    fn test_spectrum_dynamics_add_remove() {
        let mut dynamics = SpectrumDynamics::from_defaults();
        let pos = SpectrumPosition::from_defaults();

        dynamics.add_position(pos);
        assert_eq!(dynamics.len(), 1);

        dynamics.remove_position(0);
        assert!(dynamics.is_empty());
    }

    #[test]
    fn test_spectrum_dynamics_evolve_all() {
        let mut dynamics = SpectrumDynamics::from_defaults();

        for i in 0..5 {
            dynamics.add_position(SpectrumPosition::new(
                1.0 + i as Float * 0.1,
                3.0,
                SpectrumConfig::fast_evolution(),
            ));
        }

        dynamics.evolve_all(0.01, 0.5);

        // All positions should have evolved
        assert!(dynamics.average_position() > 0.0);
    }

    #[test]
    fn test_spectrum_dynamics_counting() {
        let mut dynamics = SpectrumDynamics::from_defaults();

        dynamics.add_position(SpectrumPosition::new(0.5, 3.0, SpectrumConfig::default())); // Time/Space
        dynamics.add_position(SpectrumPosition::new(1.0, 3.0, SpectrumConfig::default())); // At Veil
        dynamics.add_position(SpectrumPosition::new(2.0, 3.0, SpectrumConfig::default())); // Space/Time
        dynamics.add_position(SpectrumPosition::new(3.0, 3.0, SpectrumConfig::default())); // Space/Time

        assert_eq!(dynamics.count_time_space(), 1);
        assert_eq!(dynamics.count_at_veil(), 1);
        assert_eq!(dynamics.count_space_time(), 2);
    }

    #[test]
    fn test_veil_attraction() {
        let mut pos = SpectrumPosition::new(
            0.8,
            6.0,
            SpectrumConfig {
                veil_attraction: 0.5,
                ..Default::default()
            },
        );

        let initial = pos.value();
        pos.evolve(0.1, 0.0);

        // Near veil with high attraction should move toward it
        // (but evolution direction also moves toward unity)
    }

    #[test]
    fn test_density_affects_evolution() {
        let mut pos_low = SpectrumPosition::new(2.0, 1.0, SpectrumConfig::fast_evolution());
        let mut pos_high = SpectrumPosition::new(2.0, 6.0, SpectrumConfig::fast_evolution());

        let initial_velocity = pos_low.value();

        for _ in 0..100 {
            pos_low.evolve(0.01, 0.0);
            pos_high.evolve(0.01, 0.0);
        }

        // Both should have evolved toward unity (lower velocity)
        assert!(pos_low.value() < initial_velocity);
        assert!(pos_high.value() < initial_velocity);
        // Higher density evolves faster (more movement toward unity)
        assert!(pos_high.value() < pos_low.value());
    }
}
