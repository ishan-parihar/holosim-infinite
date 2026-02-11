//! Involution Path Tracking System
//!
//! This module tracks each entity's descent path through densities during involution.
//! It records timestamps, density levels, catalyst encountered, and provides
//! visualization and analysis tools for understanding the involution journey.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;

/// Represents a single step in an entity's involution path
#[derive(Debug, Clone, PartialEq)]
pub struct InvolutionStep {
    /// The density at this step
    pub density: Density,
    /// Sub-density level (1-7), or 0 if not applicable
    pub sub_density: u8,
    /// Timestamp when this step was reached (Unix timestamp in seconds)
    pub timestamp: u64,
    /// Catalyst encountered at this step
    pub catalyst_encountered: Vec<String>,
    /// Energy level at this step
    pub energy_level: Float,
    /// Veil thickness at this step
    pub veil_thickness: Float,
    /// Memory retention at this step
    pub memory_retention: Float,
    /// Polarity intensity at this step
    pub polarization_intensity: Float,
}

impl InvolutionStep {
    /// Create a new involution step
    pub fn new(density: Density, sub_density: u8) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            density,
            sub_density,
            timestamp,
            catalyst_encountered: Vec::new(),
            energy_level: 1.0,
            veil_thickness: 0.0,
            memory_retention: 1.0,
            polarization_intensity: 0.0,
        }
    }

    /// Set the energy level for this step
    pub fn with_energy(mut self, energy: Float) -> Self {
        self.energy_level = energy;
        self
    }

    /// Set the veil thickness for this step
    pub fn with_veil_thickness(mut self, thickness: Float) -> Self {
        self.veil_thickness = thickness;
        self
    }

    /// Set the memory retention for this step
    pub fn with_memory_retention(mut self, retention: Float) -> Self {
        self.memory_retention = retention;
        self
    }

    /// Set the polarization intensity for this step
    pub fn with_polarization(mut self, intensity: Float) -> Self {
        self.polarization_intensity = intensity;
        self
    }

    /// Add a catalyst encountered at this step
    pub fn add_catalyst(&mut self, catalyst: String) {
        self.catalyst_encountered.push(catalyst);
    }
}

/// Analysis results for an entity's involution path
#[derive(Debug, Clone, PartialEq)]
pub struct PathAnalysis {
    /// Total number of steps in the path
    pub total_steps: usize,
    /// Starting density
    pub starting_density: Density,
    /// Current density
    pub current_density: Density,
    /// Maximum depth reached (D1 = deepest)
    pub max_depth: Density,
    /// Total duration of involution in seconds
    pub total_duration: u64,
    /// Total catalyst encountered
    pub total_catalyst: usize,
    /// Average energy level across path
    pub average_energy: Float,
    /// Average veil thickness across path
    pub average_veil_thickness: Float,
    /// Average memory retention across path
    pub average_memory_retention: Float,
    /// Final polarization intensity
    pub final_polarization: Float,
    /// Whether reversal point has been detected
    pub reversal_detected: bool,
    /// Index of reversal point in path (if detected)
    pub reversal_point_index: Option<usize>,
}

impl PathAnalysis {
    /// Calculate the involution depth (number of density levels descended)
    pub fn involution_depth(&self) -> u8 {
        let start = self.starting_density.as_u8();
        let current = self.current_density.as_u8();
        if start >= current {
            start - current
        } else {
            0
        }
    }

    /// Calculate the average descent rate (densities per hour)
    pub fn average_descent_rate(&self) -> Float {
        if self.total_duration == 0 || self.total_steps <= 1 {
            return 0.0;
        }
        let hours = self.total_duration as Float / 3600.0;
        let densities_descended = self.involution_depth() as Float;
        densities_descended / hours
    }
}

/// Reversal point information
#[derive(Debug, Clone, PartialEq)]
pub struct ReversalPoint {
    /// Index of the reversal point in the path
    pub step_index: usize,
    /// Density at reversal point
    pub density: Density,
    /// Timestamp when reversal was detected
    pub timestamp: u64,
    /// Polarity intensity at reversal
    pub polarization_intensity: Float,
    /// Total catalyst processed at reversal
    pub catalyst_processed: usize,
}

/// Involution Path Tracker
///
/// Records and analyzes each entity's descent path through densities.
/// Provides tools for visualization and analysis of the involution journey.
#[derive(Debug, Clone)]
pub struct InvolutionPathTracker {
    /// Maps entity IDs to their involution paths
    paths: HashMap<u64, Vec<InvolutionStep>>,
    /// Maps entity IDs to their reversal points
    reversal_points: HashMap<u64, ReversalPoint>,
}

impl InvolutionPathTracker {
    /// Create a new involution path tracker
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            reversal_points: HashMap::new(),
        }
    }

    /// Record a descent step for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `density` - The density being entered
    /// * `sub_density` - Sub-density level (1-7), or 0 if not applicable
    ///
    /// # Returns
    /// The index of the recorded step in the path
    pub fn record_descent_step(
        &mut self,
        entity_id: u64,
        density: Density,
        sub_density: u8,
    ) -> usize {
        let step = InvolutionStep::new(density, sub_density);
        let path = self.paths.entry(entity_id).or_insert_with(Vec::new);
        let index = path.len();
        path.push(step);
        index
    }

    /// Record a descent step with additional context
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `step` - The involution step to record
    ///
    /// # Returns
    /// The index of the recorded step in the path
    pub fn record_step(&mut self, entity_id: u64, step: InvolutionStep) -> usize {
        let path = self.paths.entry(entity_id).or_insert_with(Vec::new);
        let index = path.len();
        path.push(step);
        index
    }

    /// Get the complete involution path for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// A vector of involution steps, or None if entity not found
    pub fn get_involution_path(&self, entity_id: u64) -> Option<&[InvolutionStep]> {
        self.paths.get(&entity_id).map(|path| path.as_slice())
    }

    /// Get the current step for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The most recent step, or None if entity not found or has no steps
    pub fn get_current_step(&self, entity_id: u64) -> Option<&InvolutionStep> {
        self.paths.get(&entity_id)?.last()
    }

    /// Get the starting density for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The starting density, or None if entity not found or has no steps
    pub fn get_starting_density(&self, entity_id: u64) -> Option<Density> {
        self.paths.get(&entity_id)?.first().map(|step| step.density)
    }

    /// Get the current density for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// The current density, or None if entity not found or has no steps
    pub fn get_current_density(&self, entity_id: u64) -> Option<Density> {
        self.paths.get(&entity_id)?.last().map(|step| step.density)
    }

    /// Find the reversal point for an entity
    ///
    /// The reversal point is detected when:
    /// - The entity has reached minimum density (D1 or D2)
    /// - The entity has processed significant catalyst
    /// - The entity has established polarization
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `min_density_threshold` - Minimum density to consider for reversal (D1 or D2)
    /// * `min_catalyst_threshold` - Minimum catalyst processed to consider for reversal
    /// * `min_polarization_threshold` - Minimum polarization intensity to consider for reversal
    ///
    /// # Returns
    /// The reversal point if found, or None
    pub fn find_reversal_point(
        &self,
        entity_id: u64,
        min_density_threshold: Density,
        min_catalyst_threshold: usize,
        min_polarization_threshold: Float,
    ) -> Option<ReversalPoint> {
        // Check if we already have a reversal point
        if let Some(reversal) = self.reversal_points.get(&entity_id) {
            return Some(reversal.clone());
        }

        // Find the reversal point
        let path = self.paths.get(&entity_id)?;
        let total_catalyst: usize = path
            .iter()
            .map(|step| step.catalyst_encountered.len())
            .sum();

        // Iterate from the end to find the first step that meets criteria
        for (index, step) in path.iter().enumerate().rev() {
            let density_value = step.density.as_u8();
            let threshold_value = min_density_threshold.as_u8();

            // Check if at or below minimum density
            if density_value <= threshold_value {
                // Check catalyst threshold
                if total_catalyst >= min_catalyst_threshold {
                    // Check polarization threshold
                    if step.polarization_intensity >= min_polarization_threshold {
                        let reversal = ReversalPoint {
                            step_index: index,
                            density: step.density,
                            timestamp: step.timestamp,
                            polarization_intensity: step.polarization_intensity,
                            catalyst_processed: total_catalyst,
                        };
                        return Some(reversal);
                    }
                }
            }
        }

        None
    }

    /// Mark the reversal point for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    /// * `reversal_point` - The reversal point to mark
    pub fn mark_reversal_point(&mut self, entity_id: u64, reversal_point: ReversalPoint) {
        self.reversal_points.insert(entity_id, reversal_point);
    }

    /// Analyze the involution path for an entity
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// Path analysis results, or None if entity not found or has no steps
    pub fn analyze_path(&self, entity_id: u64) -> Option<PathAnalysis> {
        let path = self.paths.get(&entity_id)?;

        if path.is_empty() {
            return None;
        }

        let total_steps = path.len();
        let starting_density = path.first()?.density;
        let current_density = path.last()?.density;
        let max_depth = path
            .iter()
            .map(|step| step.density.as_u8())
            .min()
            .and_then(|d| Density::from_u8(d))
            .unwrap_or(current_density);

        let first_timestamp = path.first()?.timestamp;
        let last_timestamp = path.last()?.timestamp;
        let total_duration = last_timestamp - first_timestamp;

        let total_catalyst: usize = path
            .iter()
            .map(|step| step.catalyst_encountered.len())
            .sum();

        let average_energy =
            path.iter().map(|step| step.energy_level).sum::<Float>() / total_steps as Float;
        let average_veil_thickness =
            path.iter().map(|step| step.veil_thickness).sum::<Float>() / total_steps as Float;
        let average_memory_retention =
            path.iter().map(|step| step.memory_retention).sum::<Float>() / total_steps as Float;

        let final_polarization = path.last()?.polarization_intensity;

        let reversal_detected = self.reversal_points.contains_key(&entity_id);
        let reversal_point_index = self.reversal_points.get(&entity_id).map(|rp| rp.step_index);

        Some(PathAnalysis {
            total_steps,
            starting_density,
            current_density,
            max_depth,
            total_duration,
            total_catalyst,
            average_energy,
            average_veil_thickness,
            average_memory_retention,
            final_polarization,
            reversal_detected,
            reversal_point_index,
        })
    }

    /// Generate a visualization of the involution path
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    ///
    /// # Returns
    /// A string representation of the path visualization
    pub fn visualize_path(&self, entity_id: u64) -> Option<String> {
        let path = self.paths.get(&entity_id)?;

        if path.is_empty() {
            return None;
        }

        let mut visualization = String::new();
        visualization.push_str("Involution Path Visualization:\n");
        visualization.push_str("=============================\n");

        for (index, step) in path.iter().enumerate() {
            visualization.push_str(&format!(
                "Step {}: D{}.{} | Time: {}s | Energy: {:.2} | Veil: {:.2} | Memory: {:.2} | Pol: {:.2} | Catalyst: {}\n",
                index,
                step.density.as_u8(),
                step.sub_density,
                step.timestamp,
                step.energy_level,
                step.veil_thickness,
                step.memory_retention,
                step.polarization_intensity,
                step.catalyst_encountered.len()
            ));
        }

        // Add reversal point marker if detected
        if let Some(reversal) = self.reversal_points.get(&entity_id) {
            visualization.push_str("\n");
            visualization.push_str(&format!(
                "🔄 REVERSAL POINT at Step {}: D{} | Pol: {:.2} | Catalyst: {}\n",
                reversal.step_index,
                reversal.density.as_u8(),
                reversal.polarization_intensity,
                reversal.catalyst_processed
            ));
        }

        Some(visualization)
    }

    /// Remove an entity from the tracker
    ///
    /// # Arguments
    /// * `entity_id` - Unique identifier for the entity
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.paths.remove(&entity_id);
        self.reversal_points.remove(&entity_id);
    }

    /// Clear all tracked paths
    pub fn clear_all(&mut self) {
        self.paths.clear();
        self.reversal_points.clear();
    }

    /// Get the number of tracked entities
    pub fn entity_count(&self) -> usize {
        self.paths.len()
    }

    /// Get all entity IDs being tracked
    pub fn get_all_entity_ids(&self) -> Vec<u64> {
        self.paths.keys().copied().collect()
    }
}

impl Default for InvolutionPathTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_involution_step_creation() {
        let step = InvolutionStep::new(Density::Fourth, 3);
        assert_eq!(step.density, Density::Fourth);
        assert_eq!(step.sub_density, 3);
        assert_eq!(step.energy_level, 1.0);
        assert_eq!(step.veil_thickness, 0.0);
        assert_eq!(step.memory_retention, 1.0);
        assert_eq!(step.polarization_intensity, 0.0);
    }

    #[test]
    fn test_involution_step_with_energy() {
        let step = InvolutionStep::new(Density::Third, 2)
            .with_energy(0.8)
            .with_veil_thickness(0.6)
            .with_memory_retention(0.4)
            .with_polarization(0.5);

        assert_eq!(step.energy_level, 0.8);
        assert_eq!(step.veil_thickness, 0.6);
        assert_eq!(step.memory_retention, 0.4);
        assert_eq!(step.polarization_intensity, 0.5);
    }

    #[test]
    fn test_involution_step_add_catalyst() {
        let mut step = InvolutionStep::new(Density::Second(Density2SubLevel::Cellular), 1);
        step.add_catalyst("Emotional catalyst".to_string());
        step.add_catalyst("Mental catalyst".to_string());

        assert_eq!(step.catalyst_encountered.len(), 2);
        assert_eq!(step.catalyst_encountered[0], "Emotional catalyst");
        assert_eq!(step.catalyst_encountered[1], "Mental catalyst");
    }

    #[test]
    fn test_record_descent_step() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 123;

        let index1 = tracker.record_descent_step(entity_id, Density::Seventh, 0);
        let index2 = tracker.record_descent_step(entity_id, Density::Sixth, 1);
        let index3 = tracker.record_descent_step(entity_id, Density::Fifth, 2);

        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(index3, 2);

        let path = tracker.get_involution_path(entity_id).unwrap();
        assert_eq!(path.len(), 3);
    }

    #[test]
    fn test_get_current_step() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 456;

        tracker.record_descent_step(entity_id, Density::Sixth, 1);
        tracker.record_descent_step(entity_id, Density::Fifth, 2);

        let current = tracker.get_current_step(entity_id).unwrap();
        assert_eq!(current.density, Density::Fifth);
        assert_eq!(current.sub_density, 2);
    }

    #[test]
    fn test_get_starting_density() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 789;

        tracker.record_descent_step(entity_id, Density::Seventh, 0);
        tracker.record_descent_step(entity_id, Density::Sixth, 1);

        let starting = tracker.get_starting_density(entity_id).unwrap();
        assert_eq!(starting, Density::Seventh);
    }

    #[test]
    fn test_get_current_density() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 101;

        tracker.record_descent_step(entity_id, Density::Fifth, 3);
        tracker.record_descent_step(entity_id, Density::Fourth, 1);

        let current = tracker.get_current_density(entity_id).unwrap();
        assert_eq!(current, Density::Fourth);
    }

    #[test]
    fn test_find_reversal_point() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 202;

        // Record descent path with catalyst and polarization
        let mut step1 = InvolutionStep::new(Density::Fourth, 1).with_polarization(0.2);
        step1.add_catalyst("Catalyst 1".to_string());
        tracker.record_step(entity_id, step1);

        let mut step2 = InvolutionStep::new(Density::Third, 2).with_polarization(0.4);
        step2.add_catalyst("Catalyst 2".to_string());
        tracker.record_step(entity_id, step2);

        let mut step3 = InvolutionStep::new(Density::Second(Density2SubLevel::Cellular), 3)
            .with_polarization(0.7);
        step3.add_catalyst("Catalyst 3".to_string());
        tracker.record_step(entity_id, step3);

        // Find reversal point with thresholds
        let reversal = tracker.find_reversal_point(
            entity_id,
            Density::Second(Density2SubLevel::Cellular),
            2,
            0.5,
        );
        assert!(reversal.is_some());
        assert_eq!(
            reversal.unwrap().density,
            Density::Second(Density2SubLevel::Cellular)
        );

        // Should not find reversal with higher threshold
        let reversal = tracker.find_reversal_point(
            entity_id,
            Density::First(Density1SubLevel::Quantum),
            2,
            0.5,
        );
        assert!(reversal.is_none());
    }

    #[test]
    fn test_mark_reversal_point() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 303;

        tracker.record_descent_step(entity_id, Density::Second(Density2SubLevel::Cellular), 5);

        let reversal = ReversalPoint {
            step_index: 0,
            density: Density::Second(Density2SubLevel::Cellular),
            timestamp: 12345,
            polarization_intensity: 0.8,
            catalyst_processed: 10,
        };

        tracker.mark_reversal_point(entity_id, reversal);

        let found = tracker.find_reversal_point(
            entity_id,
            Density::Second(Density2SubLevel::Cellular),
            5,
            0.5,
        );
        assert!(found.is_some());
        assert_eq!(found.unwrap().polarization_intensity, 0.8);
    }

    #[test]
    fn test_analyze_path() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 404;

        let step1 = InvolutionStep::new(Density::Fifth, 1)
            .with_energy(1.0)
            .with_veil_thickness(0.3)
            .with_memory_retention(0.7)
            .with_polarization(0.2);

        let step2 = InvolutionStep::new(Density::Fourth, 2)
            .with_energy(0.9)
            .with_veil_thickness(0.45)
            .with_memory_retention(0.55)
            .with_polarization(0.4);

        tracker.record_step(entity_id, step1);
        tracker.record_step(entity_id, step2);

        let analysis = tracker.analyze_path(entity_id).unwrap();
        assert_eq!(analysis.total_steps, 2);
        assert_eq!(analysis.starting_density, Density::Fifth);
        assert_eq!(analysis.current_density, Density::Fourth);
        assert_eq!(analysis.max_depth, Density::Fourth);
        assert_eq!(analysis.final_polarization, 0.4);
    }

    #[test]
    fn test_path_analysis_involution_depth() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 505;

        tracker.record_descent_step(entity_id, Density::Sixth, 0);
        tracker.record_descent_step(entity_id, Density::Fifth, 1);
        tracker.record_descent_step(entity_id, Density::Fourth, 2);

        let analysis = tracker.analyze_path(entity_id).unwrap();
        assert_eq!(analysis.involution_depth(), 2); // D6 to D4
    }

    #[test]
    fn test_path_analysis_average_descent_rate() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 606;

        let mut step1 = InvolutionStep::new(Density::Fifth, 0);
        step1.timestamp = 1000;
        tracker.record_step(entity_id, step1);

        let mut step2 = InvolutionStep::new(Density::Third, 0);
        step2.timestamp = 10000; // 9000 seconds later
        tracker.record_step(entity_id, step2);

        let analysis = tracker.analyze_path(entity_id).unwrap();
        // 2 densities in 2.5 hours = 0.8 densities/hour
        let rate = analysis.average_descent_rate();
        assert!(rate > 0.0);
    }

    #[test]
    fn test_visualize_path() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 707;

        let step = InvolutionStep::new(Density::Fourth, 3)
            .with_energy(0.85)
            .with_veil_thickness(0.45)
            .with_memory_retention(0.55)
            .with_polarization(0.6);
        tracker.record_step(entity_id, step);

        let visualization = tracker.visualize_path(entity_id);
        assert!(visualization.is_some());
        let viz = visualization.unwrap();
        assert!(viz.contains("Involution Path Visualization"));
        assert!(viz.contains("D4.3"));
        assert!(viz.contains("Energy: 0.85"));
    }

    #[test]
    fn test_visualize_path_with_reversal() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 808;

        tracker.record_descent_step(entity_id, Density::Second(Density2SubLevel::Cellular), 5);

        let reversal = ReversalPoint {
            step_index: 0,
            density: Density::Second(Density2SubLevel::Cellular),
            timestamp: 12345,
            polarization_intensity: 0.8,
            catalyst_processed: 10,
        };
        tracker.mark_reversal_point(entity_id, reversal);

        let visualization = tracker.visualize_path(entity_id).unwrap();
        assert!(visualization.contains("🔄 REVERSAL POINT"));
        assert!(visualization.contains("Pol: 0.80"));
    }

    #[test]
    fn test_remove_entity() {
        let mut tracker = InvolutionPathTracker::new();
        let entity_id = 909;

        tracker.record_descent_step(entity_id, Density::Fifth, 1);
        assert_eq!(tracker.entity_count(), 1);

        tracker.remove_entity(entity_id);
        assert_eq!(tracker.entity_count(), 0);
        assert!(tracker.get_involution_path(entity_id).is_none());
    }

    #[test]
    fn test_clear_all() {
        let mut tracker = InvolutionPathTracker::new();

        tracker.record_descent_step(111, Density::Fourth, 1);
        tracker.record_descent_step(222, Density::Third, 2);
        assert_eq!(tracker.entity_count(), 2);

        tracker.clear_all();
        assert_eq!(tracker.entity_count(), 0);
    }

    #[test]
    fn test_get_all_entity_ids() {
        let mut tracker = InvolutionPathTracker::new();

        tracker.record_descent_step(333, Density::Fourth, 1);
        tracker.record_descent_step(444, Density::Third, 2);
        tracker.record_descent_step(555, Density::Fifth, 0);

        let ids = tracker.get_all_entity_ids();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&333));
        assert!(ids.contains(&444));
        assert!(ids.contains(&555));
    }

    #[test]
    fn test_default() {
        let tracker = InvolutionPathTracker::default();
        assert_eq!(tracker.entity_count(), 0);
    }

    #[test]
    fn test_multiple_entities_independent_paths() {
        let mut tracker = InvolutionPathTracker::new();

        tracker.record_descent_step(111, Density::Sixth, 0);
        tracker.record_descent_step(222, Density::Fifth, 1);

        let path1 = tracker.get_involution_path(111).unwrap();
        let path2 = tracker.get_involution_path(222).unwrap();

        assert_eq!(path1.len(), 1);
        assert_eq!(path2.len(), 1);
        assert_ne!(path1[0].density, path2[0].density);
    }
}
