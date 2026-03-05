//! Manifestation Engine - Dynamic threshold system for quantum manifestation
//!
//! This module implements the ManifestationEngine according to V5 Phase 3 specifications.
//! The engine manages the transition from potential manifestations to collapsed states
//! through the act of observation, with dynamic thresholds based on density.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Observation creates the physical by collapsing the field into manifestation."
//! "The Veil creates the separation that allows observation to happen."
//!
//! Key Concepts:
//! - Dynamic Thresholds: Higher density = lower threshold = easier manifestation
//! - Density Modifiers: Each density has a different manifestation efficiency
//! - Observer Integration: Uses Observer.observe() to collapse potentials
//! - History Tracking: Records all manifestations for analysis
//!
//! This module implements:
//! - ManifestationEngine with dynamic threshold computation
//! - ManifestationRecord for tracking individual manifestations
//! - ManifestationStats for aggregation and analysis
//! - Batch processing for multiple potentials
//! - Integration with Observer system

use crate::foundation::observer::{CollapsedState, Observer, PotentialManifestation};
use crate::types::{Density, Float};
use std::collections::HashMap;

// ============================================================================
// MANIFESTATION RECORD
// ============================================================================

/// Record of a single manifestation event
///
/// Tracks when a potential manifestation was collapsed into a definite state.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestationRecord {
    /// Unique identifier for this record
    pub record_id: u64,
    /// The potential that was manifested
    pub potential: PotentialManifestation,
    /// The resulting collapsed state
    pub collapsed_state: CollapsedState,
    /// Density at which manifestation occurred
    pub density: Density,
    /// The threshold used for this manifestation
    pub threshold_used: Float,
    /// When this manifestation occurred (simulation step)
    pub timestamp: u64,
    /// Whether this was a successful manifestation
    pub success: bool,
}

impl ManifestationRecord {
    /// Create a new manifestation record
    pub fn new(
        potential: PotentialManifestation,
        collapsed_state: CollapsedState,
        density: Density,
        threshold_used: Float,
        timestamp: u64,
    ) -> Self {
        Self {
            record_id: rand::random(),
            potential,
            collapsed_state,
            density,
            threshold_used,
            timestamp,
            success: true,
        }
    }

    /// Create a failed manifestation record
    pub fn failed(
        potential: PotentialManifestation,
        density: Density,
        threshold_used: Float,
        timestamp: u64,
    ) -> Self {
        let scale_level = potential.scale_level;
        Self {
            record_id: rand::random(),
            potential,
            // Create a default collapsed state for failed manifestations
            collapsed_state: CollapsedState::new(
                crate::foundation::observer::FieldSignature::default_for_scale(scale_level),
                0.0,
                timestamp,
                density,
                0,
            ),
            density,
            threshold_used,
            timestamp,
            success: false,
        }
    }
}

// ============================================================================
// MANIFESTATION STATS
// ============================================================================

/// Statistics about manifestation activity
///
/// Provides aggregate information about manifestation success rates,
/// thresholds used, and density-specific performance.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestationStats {
    /// Total number of manifestations attempted
    pub total_attempts: usize,
    /// Number of successful manifestations
    pub successful_manifestations: usize,
    /// Success rate (0.0 to 1.0)
    pub success_rate: Float,
    /// Average threshold used across all manifestations
    pub average_threshold: Float,
    /// Manifestations per density
    pub manifestations_by_density: HashMap<Density, usize>,
    /// Average threshold by density
    pub average_threshold_by_density: HashMap<Density, Float>,
    /// Total manifestation strength across all successes
    pub total_manifestation_strength: Float,
    /// Average manifestation strength
    pub average_manifestation_strength: Float,
}

impl ManifestationStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self {
            total_attempts: 0,
            successful_manifestations: 0,
            success_rate: 0.0,
            average_threshold: 0.0,
            manifestations_by_density: HashMap::new(),
            average_threshold_by_density: HashMap::new(),
            total_manifestation_strength: 0.0,
            average_manifestation_strength: 0.0,
        }
    }

    /// Add a manifestation record to stats
    pub fn add_record(&mut self, record: &ManifestationRecord) {
        self.total_attempts += 1;

        if record.success {
            self.successful_manifestations += 1;
            self.total_manifestation_strength += record.collapsed_state.manifestation;
        }

        // Update density-specific stats
        *self
            .manifestations_by_density
            .entry(record.density)
            .or_insert(0) += 1;

        self.recalculate();
    }

    /// Recalculate derived statistics
    fn recalculate(&mut self) {
        // Calculate success rate
        if self.total_attempts > 0 {
            self.success_rate =
                self.successful_manifestations as Float / self.total_attempts as Float;
        }

        // Calculate average manifestation strength
        if self.successful_manifestations > 0 {
            self.average_manifestation_strength =
                self.total_manifestation_strength / self.successful_manifestations as Float;
        }
    }

    /// Get manifestations for a specific density
    pub fn count_for_density(&self, density: Density) -> usize {
        *self.manifestations_by_density.get(&density).unwrap_or(&0)
    }

    /// Get average threshold for a specific density
    pub fn average_threshold_for_density(&self, density: Density) -> Float {
        *self
            .average_threshold_by_density
            .get(&density)
            .unwrap_or(&0.0)
    }
}

impl Default for ManifestationStats {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MANIFESTATION ENGINE
// ============================================================================

/// Manifestation Engine - Manages quantum manifestation with dynamic thresholds
///
/// From V5 Phase 3 specifications:
/// "The ManifestationEngine manages the transition from potential to collapsed state."
/// "Dynamic thresholds based on density - higher density = lower threshold = easier manifestation."
///
/// The engine:
/// - Computes dynamic thresholds based on density
/// - Evaluates whether potentials should manifest
/// - Uses Observer.observe() to collapse potentials into states
/// - Tracks history and statistics
/// - Supports batch processing
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestationEngine {
    /// Base threshold (default threshold before density modifiers)
    pub base_threshold: Float,
    /// Density modifiers for each density level
    pub density_modifiers: HashMap<Density, Float>,
    /// History of all manifestations
    pub history: Vec<ManifestationRecord>,
    /// Statistics about manifestations
    pub stats: ManifestationStats,
    /// Maximum history size (to prevent unbounded growth)
    pub max_history_size: usize,
}

impl ManifestationEngine {
    /// Create a new manifestation engine with default parameters
    pub fn new() -> Self {
        Self::with_base_threshold(0.5)
    }

    /// Create a new manifestation engine with a specific base threshold
    pub fn with_base_threshold(base_threshold: Float) -> Self {
        let mut engine = Self {
            base_threshold: base_threshold.clamp(0.0, 1.0),
            density_modifiers: HashMap::new(),
            history: Vec::new(),
            stats: ManifestationStats::new(),
            max_history_size: 10000,
        };

        // Initialize density modifiers
        // Higher density = lower modifier = easier manifestation
        engine.initialize_density_modifiers();
        engine
    }

    /// Initialize density modifiers
    ///
    /// From V5 Phase 3:
    /// "Higher density = lower threshold = easier manifestation"
    /// The modifier reduces the threshold for higher densities.
    fn initialize_density_modifiers(&mut self) {
        // Lower modifier = easier manifestation
        self.density_modifiers.insert(Density::First, 1.0); // Hardest to manifest
        self.density_modifiers.insert(Density::Second, 0.9);
        self.density_modifiers.insert(Density::Third, 0.8);
        self.density_modifiers.insert(Density::Fourth, 0.6); // Half difficulty
        self.density_modifiers.insert(Density::Fifth, 0.4);
        self.density_modifiers.insert(Density::Sixth, 0.2);
        self.density_modifiers.insert(Density::Seventh, 0.1); // Easiest to manifest
        self.density_modifiers.insert(Density::Eighth, 0.0); // No threshold limit
    }

    /// Compute the manifestation threshold for a given density
    ///
    /// From V5 Phase 3:
    /// "Dynamic thresholds based on density - higher density = lower threshold"
    ///
    /// Formula: threshold = base_threshold * density_modifier
    pub fn compute_threshold(&self, density: Density) -> Float {
        let modifier = *self.density_modifiers.get(&density).unwrap_or(&1.0);
        self.base_threshold * modifier
    }

    /// Determine if a potential should manifest
    ///
    /// A potential manifests if its amplitude exceeds the threshold for its density.
    pub fn should_manifest(&self, potential: &PotentialManifestation, density: Density) -> bool {
        let threshold = self.compute_threshold(density);
        potential.amplitude >= threshold
    }

    /// Process a single potential manifestation
    ///
    /// This method:
    /// 1. Checks if the potential should manifest (based on threshold)
    /// 2. If yes, uses the observer to collapse the potential
    /// 3. Records the manifestation in history
    /// 4. Updates statistics
    ///
    /// Returns the collapsed state if manifestation occurred, None otherwise.
    pub fn process(
        &mut self,
        potential: &PotentialManifestation,
        observer: &mut Observer,
        density: Density,
        timestamp: u64,
    ) -> Option<CollapsedState> {
        let threshold = self.compute_threshold(density);

        // Check if potential should manifest
        if !self.should_manifest(potential, density) {
            // Record failed manifestation
            let failed_record =
                ManifestationRecord::failed(potential.clone(), density, threshold, timestamp);
            self.add_to_history(failed_record);
            return None;
        }

        // Create field signature from potential
        let field_signature = crate::foundation::observer::FieldSignature::new(
            potential.potential_id,
            "Potential Manifestation".to_string(),
            potential.amplitude,
            0.0,                 // phase
            potential.amplitude, // coherence = amplitude for simplicity
            potential.scale_level,
        );

        // Create focus target
        let focus_target = crate::foundation::observer::FocusTarget::new(
            crate::foundation::observer::FocusTargetType::PossibilitySpace,
            potential.potential_id,
        );

        // Use observer to collapse the potential
        let collapsed_state = observer.observe(&focus_target, &field_signature, timestamp);

        if let Some(state) = collapsed_state {
            // Record successful manifestation
            let record = ManifestationRecord::new(
                potential.clone(),
                state.clone(),
                density,
                threshold,
                timestamp,
            );
            self.add_to_history(record);
            Some(state)
        } else {
            // Observer failed to collapse - record as failed
            let failed_record =
                ManifestationRecord::failed(potential.clone(), density, threshold, timestamp);
            self.add_to_history(failed_record);
            None
        }
    }

    /// Process multiple potential manifestations in batch
    ///
    /// Efficiently processes multiple potentials, returning a vector of collapsed states
    /// for those that successfully manifested.
    pub fn batch_process(
        &mut self,
        potentials: &[PotentialManifestation],
        observer: &mut Observer,
        density: Density,
        timestamp: u64,
    ) -> Vec<CollapsedState> {
        let mut results = Vec::new();

        for potential in potentials {
            if let Some(collapsed) = self.process(potential, observer, density, timestamp) {
                results.push(collapsed);
            }
        }

        results
    }

    /// Get current statistics
    pub fn stats(&self) -> &ManifestationStats {
        &self.stats
    }

    /// Get mutable statistics
    pub fn stats_mut(&mut self) -> &mut ManifestationStats {
        &mut self.stats
    }

    /// Get manifestation history
    pub fn history(&self) -> &[ManifestationRecord] {
        &self.history
    }

    /// Get recent manifestations (last n records)
    pub fn recent_history(&self, n: usize) -> Vec<&ManifestationRecord> {
        let len = self.history.len();
        let start = len.saturating_sub(n);
        self.history[start..].iter().collect()
    }

    /// Get manifestations for a specific density
    pub fn history_for_density(&self, density: Density) -> Vec<&ManifestationRecord> {
        self.history
            .iter()
            .filter(|record| record.density == density)
            .collect()
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.stats = ManifestationStats::new();
    }

    /// Set the maximum history size
    pub fn set_max_history_size(&mut self, max_size: usize) {
        self.max_history_size = max_size;
        self.trim_history();
    }

    /// Update the base threshold
    pub fn set_base_threshold(&mut self, threshold: Float) {
        self.base_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set a density modifier
    pub fn set_density_modifier(&mut self, density: Density, modifier: Float) {
        self.density_modifiers
            .insert(density, modifier.clamp(0.0, 1.0));
    }

    /// Add a record to history and update stats
    fn add_to_history(&mut self, record: ManifestationRecord) {
        self.stats.add_record(&record);
        self.history.push(record);
        self.trim_history();
    }

    /// Trim history to max size
    fn trim_history(&mut self) {
        if self.history.len() > self.max_history_size {
            // Remove oldest records
            let excess = self.history.len() - self.max_history_size;
            self.history.drain(0..excess);
        }
    }

    /// Recalculate statistics from history
    pub fn recalculate_stats(&mut self) {
        self.stats = ManifestationStats::new();
        for record in &self.history {
            self.stats.add_record(record);
        }

        // Calculate average thresholds by density
        let mut threshold_sums: HashMap<Density, (Float, usize)> = HashMap::new();

        for record in &self.history {
            let entry = threshold_sums.entry(record.density).or_insert((0.0, 0));
            entry.0 += record.threshold_used;
            entry.1 += 1;
        }

        for (density, (sum, count)) in threshold_sums {
            if count > 0 {
                self.stats
                    .average_threshold_by_density
                    .insert(density, sum / count as Float);
            }
        }

        // Calculate overall average threshold
        if !self.history.is_empty() {
            let total_threshold: Float = self.history.iter().map(|r| r.threshold_used).sum();
            self.stats.average_threshold = total_threshold / self.history.len() as Float;
        }
    }
}

impl Default for ManifestationEngine {
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
    use crate::foundation::observer::FieldSignature;
    use crate::holographic::field_address::ScaleLevel;

    #[test]
    fn test_manifestation_engine_creation() {
        let engine = ManifestationEngine::new();

        assert_eq!(engine.base_threshold, 0.5);
        assert_eq!(engine.history.len(), 0);
        assert_eq!(engine.stats.total_attempts, 0);
    }

    #[test]
    fn test_manifestation_engine_with_base_threshold() {
        let engine = ManifestationEngine::with_base_threshold(0.7);

        assert_eq!(engine.base_threshold, 0.7);
    }

    #[test]
    fn test_density_modifiers_initialization() {
        let engine = ManifestationEngine::new();

        // Check that all densities have modifiers
        assert_eq!(engine.density_modifiers.len(), 8);

        // Higher density should have lower modifier
        let first_mod = engine.density_modifiers[&Density::First];
        let fourth_mod = engine.density_modifiers[&Density::Fourth];
        let seventh_mod = engine.density_modifiers[&Density::Seventh];

        assert!(first_mod > fourth_mod);
        assert!(fourth_mod > seventh_mod);
    }

    #[test]
    fn test_compute_threshold() {
        let engine = ManifestationEngine::new();

        // First density: 0.5 * 1.0 = 0.5
        let threshold_1st = engine.compute_threshold(Density::First);
        assert_eq!(threshold_1st, 0.5);

        // Fourth density: 0.5 * 0.6 = 0.3
        let threshold_4th = engine.compute_threshold(Density::Fourth);
        assert_eq!(threshold_4th, 0.3);

        // Seventh density: 0.5 * 0.1 = 0.05
        let threshold_7th = engine.compute_threshold(Density::Seventh);
        assert_eq!(threshold_7th, 0.05);
    }

    #[test]
    fn test_compute_threshold_with_custom_base() {
        let engine = ManifestationEngine::with_base_threshold(0.8);

        // First density: 0.8 * 1.0 = 0.8
        let threshold_1st = engine.compute_threshold(Density::First);
        assert_eq!(threshold_1st, 0.8);

        // Fourth density: 0.8 * 0.6 = 0.48
        let threshold_4th = engine.compute_threshold(Density::Fourth);
        assert_eq!(threshold_4th, 0.48);
    }

    #[test]
    fn test_should_manifest() {
        let engine = ManifestationEngine::new();

        // Potential with amplitude 0.6 at First density (threshold 0.5)
        let potential1 = PotentialManifestation::new(0.6, 0.5, None, ScaleLevel::Molecular);
        assert!(engine.should_manifest(&potential1, Density::First));

        // Potential with amplitude 0.4 at First density (threshold 0.5)
        let potential2 = PotentialManifestation::new(0.4, 0.5, None, ScaleLevel::Molecular);
        assert!(!engine.should_manifest(&potential2, Density::First));

        // Same potential at Seventh density (threshold 0.05) - should manifest
        assert!(engine.should_manifest(&potential2, Density::Seventh));
    }

    #[test]
    fn test_should_manifest_at_different_densities() {
        let engine = ManifestationEngine::new();

        // Potential with moderate amplitude (0.3)
        let potential = PotentialManifestation::new(0.3, 0.5, None, ScaleLevel::Molecular);

        // Should NOT manifest at low densities
        assert!(!engine.should_manifest(&potential, Density::First)); // threshold 0.5
        assert!(!engine.should_manifest(&potential, Density::Second)); // threshold 0.45

        // Should manifest at medium densities
        assert!(!engine.should_manifest(&potential, Density::Third)); // threshold 0.4
        assert!(engine.should_manifest(&potential, Density::Fourth)); // threshold 0.3
        assert!(engine.should_manifest(&potential, Density::Fifth)); // threshold 0.2

        // Should definitely manifest at high densities
        assert!(engine.should_manifest(&potential, Density::Sixth)); // threshold 0.1
        assert!(engine.should_manifest(&potential, Density::Seventh)); // threshold 0.05
    }

    #[test]
    fn test_manifestation_record_creation() {
        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed_state = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );

        let record = ManifestationRecord::new(potential, collapsed_state, Density::Third, 0.4, 100);

        assert!(record.success);
        assert_eq!(record.density, Density::Third);
        assert_eq!(record.threshold_used, 0.4);
        assert_eq!(record.timestamp, 100);
    }

    #[test]
    fn test_manifestation_record_failed() {
        let potential = PotentialManifestation::new(0.2, 0.5, None, ScaleLevel::Molecular);

        let record = ManifestationRecord::failed(potential, Density::First, 0.5, 100);

        assert!(!record.success);
        assert_eq!(record.density, Density::First);
        assert_eq!(record.threshold_used, 0.5);
        assert_eq!(record.collapsed_state.manifestation, 0.0);
    }

    #[test]
    fn test_manifestation_stats_creation() {
        let stats = ManifestationStats::new();

        assert_eq!(stats.total_attempts, 0);
        assert_eq!(stats.successful_manifestations, 0);
        assert_eq!(stats.success_rate, 0.0);
    }

    #[test]
    fn test_manifestation_stats_add_record() {
        let mut stats = ManifestationStats::new();

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed_state = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );

        let record = ManifestationRecord::new(potential, collapsed_state, Density::Third, 0.4, 100);

        stats.add_record(&record);

        assert_eq!(stats.total_attempts, 1);
        assert_eq!(stats.successful_manifestations, 1);
        assert_eq!(stats.success_rate, 1.0);
        assert_eq!(stats.total_manifestation_strength, 0.5);
        assert_eq!(stats.average_manifestation_strength, 0.5);
    }

    #[test]
    fn test_manifestation_stats_success_rate() {
        let mut stats = ManifestationStats::new();

        // Add successful record
        let potential1 = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed1 = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let record1 = ManifestationRecord::new(potential1, collapsed1, Density::Third, 0.4, 100);
        stats.add_record(&record1);

        // Add failed record
        let potential2 = PotentialManifestation::new(0.2, 0.5, None, ScaleLevel::Molecular);
        let record2 = ManifestationRecord::failed(potential2, Density::First, 0.5, 100);
        stats.add_record(&record2);

        assert_eq!(stats.total_attempts, 2);
        assert_eq!(stats.successful_manifestations, 1);
        assert_eq!(stats.success_rate, 0.5);
    }

    #[test]
    fn test_process_successful_manifestation() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        // High amplitude potential at Third density
        let potential = PotentialManifestation::new(0.9, 0.8, None, ScaleLevel::Molecular);

        let result = engine.process(&potential, &mut observer, Density::Third, 100);

        // Should manifest (amplitude 0.9 > threshold 0.4)
        assert!(result.is_some());
        assert_eq!(engine.history.len(), 1);
        assert_eq!(engine.stats.total_attempts, 1);
        assert_eq!(engine.stats.successful_manifestations, 1);
    }

    #[test]
    fn test_process_failed_manifestation() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        // Low amplitude potential at Third density
        let potential = PotentialManifestation::new(0.2, 0.5, None, ScaleLevel::Molecular);

        let result = engine.process(&potential, &mut observer, Density::Third, 100);

        // Should not manifest (amplitude 0.2 < threshold 0.4)
        assert!(result.is_none());
        assert_eq!(engine.history.len(), 1);
        assert_eq!(engine.stats.total_attempts, 1);
        assert_eq!(engine.stats.successful_manifestations, 0);
    }

    #[test]
    fn test_batch_process() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Fourth);

        let potentials = vec![
            PotentialManifestation::new(0.9, 0.8, None, ScaleLevel::Molecular), // Should manifest
            PotentialManifestation::new(0.2, 0.5, None, ScaleLevel::Molecular), // Should not manifest
            PotentialManifestation::new(0.7, 0.6, None, ScaleLevel::Molecular), // Should manifest
            PotentialManifestation::new(0.3, 0.5, None, ScaleLevel::Molecular), // Should not manifest
        ];

        let results = engine.batch_process(&potentials, &mut observer, Density::Fourth, 100);

        // Threshold at Fourth density: 0.5 * 0.6 = 0.3
        // Potentials 0, 1, 2 should manifest (0.9, 0.7, 0.3 >= 0.3)
        // Potential 3 should not manifest (0.2 < 0.3)
        assert!(results.len() >= 2); // At least 2 should manifest
        assert_eq!(engine.stats.total_attempts, 4);
    }

    #[test]
    fn test_recent_history() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        // Add multiple manifestations
        for i in 0..5 {
            let potential = PotentialManifestation::new(
                0.8 + i as Float * 0.01,
                0.5,
                None,
                ScaleLevel::Molecular,
            );
            engine.process(&potential, &mut observer, Density::Third, i);
        }

        let recent = engine.recent_history(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_history_for_density() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        // Process at different densities
        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        engine.process(&potential, &mut observer, Density::Third, 100);
        engine.process(&potential, &mut observer, Density::Fourth, 101);
        engine.process(&potential, &mut observer, Density::Third, 102);

        let third_density_history = engine.history_for_density(Density::Third);
        assert_eq!(third_density_history.len(), 2);

        let fourth_density_history = engine.history_for_density(Density::Fourth);
        assert_eq!(fourth_density_history.len(), 1);
    }

    #[test]
    fn test_clear_history() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        engine.process(&potential, &mut observer, Density::Third, 100);

        assert_eq!(engine.history.len(), 1);
        assert_eq!(engine.stats.total_attempts, 1);

        engine.clear_history();

        assert_eq!(engine.history.len(), 0);
        assert_eq!(engine.stats.total_attempts, 0);
    }

    #[test]
    fn test_set_base_threshold() {
        let mut engine = ManifestationEngine::new();

        engine.set_base_threshold(0.8);
        assert_eq!(engine.base_threshold, 0.8);

        // Threshold should be clamped
        engine.set_base_threshold(1.5);
        assert_eq!(engine.base_threshold, 1.0);

        engine.set_base_threshold(-0.5);
        assert_eq!(engine.base_threshold, 0.0);
    }

    #[test]
    fn test_set_density_modifier() {
        let mut engine = ManifestationEngine::new();

        engine.set_density_modifier(Density::Third, 0.5);
        assert_eq!(engine.density_modifiers[&Density::Third], 0.5);

        // New threshold should use updated modifier
        let threshold = engine.compute_threshold(Density::Third);
        assert_eq!(threshold, 0.25); // 0.5 * 0.5
    }

    #[test]
    fn test_set_max_history_size() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        engine.set_max_history_size(5);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);

        // Add 10 manifestations
        for i in 0..10 {
            engine.process(&potential, &mut observer, Density::Third, i);
        }

        // Should only keep last 5
        assert_eq!(engine.history.len(), 5);
    }

    #[test]
    fn test_recalculate_stats() {
        let mut engine = ManifestationEngine::new();
        let mut observer = Observer::for_density(Density::Third);

        // Add some manifestations
        let potential1 = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        engine.process(&potential1, &mut observer, Density::Third, 100);

        let potential2 = PotentialManifestation::new(0.2, 0.5, None, ScaleLevel::Molecular);
        engine.process(&potential2, &mut observer, Density::Third, 101);

        // Manually clear stats
        engine.stats = ManifestationStats::new();
        assert_eq!(engine.stats.total_attempts, 0);

        // Recalculate from history
        engine.recalculate_stats();

        assert_eq!(engine.stats.total_attempts, 2);
    }

    #[test]
    fn test_default_implementations() {
        let engine = ManifestationEngine::default();
        let stats = ManifestationStats::default();

        assert_eq!(engine.base_threshold, 0.5);
        assert_eq!(stats.total_attempts, 0);
    }

    #[test]
    fn test_density_threshold_monotonic() {
        let engine = ManifestationEngine::new();

        // Thresholds should decrease as density increases
        let thresholds = vec![
            engine.compute_threshold(Density::First),
            engine.compute_threshold(Density::Second),
            engine.compute_threshold(Density::Third),
            engine.compute_threshold(Density::Fourth),
            engine.compute_threshold(Density::Fifth),
            engine.compute_threshold(Density::Sixth),
            engine.compute_threshold(Density::Seventh),
        ];

        for i in 0..thresholds.len() - 1 {
            assert!(
                thresholds[i] >= thresholds[i + 1],
                "Threshold should decrease with density: {} vs {}",
                thresholds[i],
                thresholds[i + 1]
            );
        }
    }
}
