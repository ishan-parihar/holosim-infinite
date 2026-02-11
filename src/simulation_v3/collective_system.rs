// Collective System Manager
//
// Phase 2 Implementation: Tracks collective system emergence
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Densities are hierarchical material substrates, not individual evolutionary stages"
//
// This module implements:
// 1. Collective system state tracking
// 2. Density distribution tracking
// 3. Emergence detection (when 2nd/3rd density emerges)
// 4. Emergence conditions and thresholds
//
// IMPORTANT: This tracks COLLECTIVE system emergence, not individual entity progression.
// Individual entities DO NOT change density - they progress within their density.
//
// The collective system develops new densities through emergence:
// - 1st Density materials create planets → 2nd Density life emerges
// - 2nd Density life develops → 3rd Density consciousness emerges

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};

/// Collective System Manager
///
/// Phase 2 Implementation: Tracks the overall state of the collective system.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Densities are hierarchical material substrates, not individual evolutionary stages"
///
/// This manager tracks:
/// 1. **Density Distribution**: How many entities at each density level
/// 2. **Emergence Detection**: When new densities emerge in the collective system
/// 3. **Emergence Conditions**: Under what conditions densities emerge
/// 4. **Collective State**: Overall state of the collective system
///
/// Example:
/// - Track when 2nd Density emerges in the collective system
/// - Track when 3rd Density emerges in the collective system
/// - Show density distribution at each step
/// - Track how the collective system evolves
#[derive(Debug, Clone)]
pub struct CollectiveSystemManager {
    /// Density distribution: count of entities at each density
    pub density_distribution: DensityDistribution,

    /// Emergence history: when each density emerged
    pub emergence_history: EmergenceHistory,

    /// Current collective system state
    pub collective_state: CollectiveSystemState,

    /// Emergence configuration and thresholds
    pub emergence_config: EmergenceConfig,
}

impl CollectiveSystemManager {
    /// Create a new collective system manager
    pub fn new() -> Self {
        CollectiveSystemManager {
            density_distribution: DensityDistribution::new(),
            emergence_history: EmergenceHistory::new(),
            collective_state: CollectiveSystemState::new(),
            emergence_config: EmergenceConfig::default(),
        }
    }

    /// Update the collective system state based on current entities
    ///
    /// This analyzes all entities and updates:
    /// - Density distribution (count entities at each density)
    /// - Emergence detection (check if new densities emerged)
    /// - Collective system state
    pub fn update<F>(&mut self, get_entity_fn: &F)
    where
        F: Fn(EntityId) -> Option<&'static SubSubLogos>,
    {
        // Update density distribution
        self.update_density_distribution(get_entity_fn);

        // Check for emergence events
        self.check_emergence();

        // Update collective system state
        self.update_collective_state();
    }

    /// Update density distribution based on current entities
    fn update_density_distribution<F>(&mut self, _get_entity_fn: &F)
    where
        F: Fn(EntityId) -> Option<&'static SubSubLogos>,
    {
        // Reset counts
        self.density_distribution.reset();

        // Count entities at each density
        // Note: In the actual implementation, we would iterate over all entity IDs
        // For now, this is a placeholder that would be integrated with the entity store
    }

    /// Check for emergence events (new densities emerging)
    fn check_emergence(&mut self) {
        // Check if 2nd Density emerged
        if !self.emergence_history.second_density_emerged
            && self.density_distribution.has_second_density_entities()
        {
            self.emergence_history.second_density_emerged = true;
            self.emergence_history.second_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 3rd Density emerged
        if !self.emergence_history.third_density_emerged
            && self.density_distribution.has_third_density_entities()
        {
            self.emergence_history.third_density_emerged = true;
            self.emergence_history.third_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 4th Density emerged
        if !self.emergence_history.fourth_density_emerged
            && self.density_distribution.has_fourth_density_entities()
        {
            self.emergence_history.fourth_density_emerged = true;
            self.emergence_history.fourth_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 5th Density emerged
        if !self.emergence_history.fifth_density_emerged
            && self.density_distribution.has_fifth_density_entities()
        {
            self.emergence_history.fifth_density_emerged = true;
            self.emergence_history.fifth_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 6th Density emerged
        if !self.emergence_history.sixth_density_emerged
            && self.density_distribution.has_sixth_density_entities()
        {
            self.emergence_history.sixth_density_emerged = true;
            self.emergence_history.sixth_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 7th Density emerged
        if !self.emergence_history.seventh_density_emerged
            && self.density_distribution.has_seventh_density_entities()
        {
            self.emergence_history.seventh_density_emerged = true;
            self.emergence_history.seventh_density_emergence_step =
                Some(self.collective_state.current_step);
        }

        // Check if 8th Density emerged
        if !self.emergence_history.eighth_density_emerged
            && self.density_distribution.has_eighth_density_entities()
        {
            self.emergence_history.eighth_density_emerged = true;
            self.emergence_history.eighth_density_emergence_step =
                Some(self.collective_state.current_step);
        }
    }

    /// Update collective system state
    fn update_collective_state(&mut self) {
        // Update current step
        self.collective_state.current_step += 1;

        // Update highest density present
        self.collective_state.highest_density_present =
            self.density_distribution.get_highest_density();

        // Calculate emergence percentage
        self.collective_state.emergence_percentage = self.calculate_emergence_percentage();

        // Update complexity score
        self.collective_state.complexity_score =
            self.density_distribution.calculate_complexity_score();

        // Update coherence score
        self.collective_state.coherence_score = self.calculate_coherence_score();
    }

    /// Calculate emergence percentage
    fn calculate_emergence_percentage(&self) -> f64 {
        let max_density_level = 8.0;
        let current_highest_level = match self.collective_state.highest_density_present {
            Density::First(..) => 1.0,
            Density::Second(..) => 2.0,
            Density::Third => 3.0,
            Density::Fourth => 4.0,
            Density::Fifth => 5.0,
            Density::Sixth => 6.0,
            Density::Seventh => 7.0,
            Density::Eighth => 8.0,
        };

        (current_highest_level / max_density_level) * 100.0
    }

    /// Calculate coherence score based on density distribution
    fn calculate_coherence_score(&self) -> f64 {
        // Coherence is based on:
        // 1. Balance between densities (not too skewed)
        // 2. Hierarchical consistency (higher densities require sufficient lower densities)
        // 3. Stability of the system

        let total_entities = self.density_distribution.total_entities();
        if total_entities == 0 {
            return 0.0;
        }

        // Simple coherence calculation
        // Higher densities should have fewer entities (hierarchical)
        let first_density_count = self.density_distribution.first_density_total();
        let third_density_count = self.density_distribution.third_density;

        if third_density_count > 0 {
            // Should have many more 1st density than 3rd density
            let ratio = first_density_count as f64 / third_density_count as f64;
            let expected_ratio = 10.0;
            let divergence = (ratio - expected_ratio).abs() / expected_ratio;
            (1.0 - divergence.min(1.0)) * 0.8 + 0.2
        } else {
            0.5
        }
    }

    /// Get density distribution
    pub fn density_distribution(&self) -> &DensityDistribution {
        &self.density_distribution
    }

    /// Get emergence history
    pub fn emergence_history(&self) -> &EmergenceHistory {
        &self.emergence_history
    }

    /// Get collective system state
    pub fn collective_state(&self) -> &CollectiveSystemState {
        &self.collective_state
    }

    /// Get emergence configuration
    pub fn emergence_config(&self) -> &EmergenceConfig {
        &self.emergence_config
    }

    /// Check if emergence conditions are met for a density
    pub fn check_emergence_conditions(&self, target_density: Density) -> EmergenceReadiness {
        match target_density {
            Density::First(..) => EmergenceReadiness {
                is_ready: true,
                current_percentage: self.collective_state.emergence_percentage,
                required_percentage: 0.0,
                conditions_met: vec!["1st Density is the starting point".to_string()],
                conditions_missing: vec![],
            },
            Density::Second(..) => self.check_second_density_emergence_conditions(),
            Density::Third => self.check_third_density_emergence_conditions(),
            Density::Fourth => self.check_fourth_density_emergence_conditions(),
            Density::Fifth => self.check_fifth_density_emergence_conditions(),
            Density::Sixth => self.check_sixth_density_emergence_conditions(),
            Density::Seventh => self.check_seventh_density_emergence_conditions(),
            Density::Eighth => self.check_eighth_density_emergence_conditions(),
        }
    }

    /// Check if 2nd Density emergence conditions are met
    fn check_second_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: Sufficient 1st Density materials (planets)
        let has_planetary = self.density_distribution.first_density_planetary
            >= self.emergence_config.min_planetary_for_second_density;
        if has_planetary {
            conditions_met.push(
                "Sufficient planetary structures (1st Density - Planetary Realm)".to_string(),
            );
        } else {
            conditions_missing.push(format!(
                "Need at least {} planetary structures (currently: {})",
                self.emergence_config.min_planetary_for_second_density,
                self.density_distribution.first_density_planetary
            ));
        }

        // Condition 2: Sufficient 1st Density complexity
        let first_density_total = self.density_distribution.first_density_total();
        let has_complexity =
            first_density_total >= self.emergence_config.min_first_density_for_emergence;
        if has_complexity {
            conditions_met.push("Sufficient 1st Density complexity".to_string());
        } else {
            conditions_missing.push(format!(
                "Need at least {} 1st Density entities (currently: {})",
                self.emergence_config.min_first_density_for_emergence, first_density_total
            ));
        }

        // Condition 3: Minimum coherence
        let has_coherence = self.collective_state.coherence_score
            >= self.emergence_config.min_coherence_for_emergence;
        if has_coherence {
            conditions_met.push("Sufficient system coherence".to_string());
        } else {
            conditions_missing.push(format!(
                "Need coherence >= {:.2} (currently: {:.2})",
                self.emergence_config.min_coherence_for_emergence,
                self.collective_state.coherence_score
            ));
        }

        let is_ready = has_planetary && has_complexity && has_coherence;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 25.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 3rd Density emergence conditions are met
    fn check_third_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 2nd Density must have emerged
        if self.emergence_history.second_density_emerged {
            conditions_met.push("2nd Density has emerged".to_string());
        } else {
            conditions_missing.push("2nd Density must emerge first".to_string());
        }

        // Condition 2: Sufficient 2nd Density complexity (complex life)
        let has_complex_life = self.density_distribution.second_density_complex_life
            >= self.emergence_config.min_complex_life_for_third_density;
        if has_complex_life {
            conditions_met
                .push("Sufficient complex life (2nd Density - Complex Life Realm)".to_string());
        } else {
            conditions_missing.push(format!(
                "Need at least {} complex life entities (currently: {})",
                self.emergence_config.min_complex_life_for_third_density,
                self.density_distribution.second_density_complex_life
            ));
        }

        // Condition 3: Sufficient 2nd Density total
        let second_density_total = self.density_distribution.second_density_total();
        let has_sufficiency =
            second_density_total >= self.emergence_config.min_second_density_for_emergence;
        if has_sufficiency {
            conditions_met.push("Sufficient 2nd Density complexity".to_string());
        } else {
            conditions_missing.push(format!(
                "Need at least {} 2nd Density entities (currently: {})",
                self.emergence_config.min_second_density_for_emergence, second_density_total
            ));
        }

        let is_ready =
            self.emergence_history.second_density_emerged && has_complex_life && has_sufficiency;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 50.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 4th Density emergence conditions are met
    fn check_fourth_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 3rd Density must have emerged
        if self.emergence_history.third_density_emerged {
            conditions_met.push("3rd Density has emerged".to_string());
        } else {
            conditions_missing.push("3rd Density must emerge first".to_string());
        }

        // Condition 2: Sufficient 3rd Density polarization
        let has_polarization = self.density_distribution.third_density
            >= self.emergence_config.min_third_density_for_emergence;
        if has_polarization {
            conditions_met.push("Sufficient 3rd Density complexity".to_string());
        } else {
            conditions_missing.push(format!(
                "Need at least {} 3rd Density entities (currently: {})",
                self.emergence_config.min_third_density_for_emergence,
                self.density_distribution.third_density
            ));
        }

        let is_ready = self.emergence_history.third_density_emerged && has_polarization;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 75.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 5th Density emergence conditions are met
    fn check_fifth_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 4th Density must have emerged
        if self.emergence_history.fourth_density_emerged {
            conditions_met.push("4th Density has emerged".to_string());
        } else {
            conditions_missing.push("4th Density must emerge first".to_string());
        }

        let is_ready = self.emergence_history.fourth_density_emerged;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 85.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 6th Density emergence conditions are met
    fn check_sixth_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 5th Density must have emerged
        if self.emergence_history.fifth_density_emerged {
            conditions_met.push("5th Density has emerged".to_string());
        } else {
            conditions_missing.push("5th Density must emerge first".to_string());
        }

        let is_ready = self.emergence_history.fifth_density_emerged;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 95.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 7th Density emergence conditions are met
    fn check_seventh_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 6th Density must have emerged
        if self.emergence_history.sixth_density_emerged {
            conditions_met.push("6th Density has emerged".to_string());
        } else {
            conditions_missing.push("6th Density must emerge first".to_string());
        }

        let is_ready = self.emergence_history.sixth_density_emerged;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 99.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Check if 8th Density emergence conditions are met
    fn check_eighth_density_emergence_conditions(&self) -> EmergenceReadiness {
        let mut conditions_met = vec![];
        let mut conditions_missing = vec![];

        // Condition 1: 7th Density must have emerged
        if self.emergence_history.seventh_density_emerged {
            conditions_met.push("7th Density has emerged".to_string());
        } else {
            conditions_missing.push("7th Density must emerge first".to_string());
        }

        let is_ready = self.emergence_history.seventh_density_emerged;

        EmergenceReadiness {
            is_ready,
            current_percentage: self.collective_state.emergence_percentage,
            required_percentage: 100.0,
            conditions_met,
            conditions_missing,
        }
    }

    /// Get emergence summary
    pub fn emergence_summary(&self) -> EmergenceSummary {
        EmergenceSummary {
            current_step: self.collective_state.current_step,
            emergence_percentage: self.collective_state.emergence_percentage,
            highest_density: format!("{:?}", self.collective_state.highest_density_present),
            densities_emerged: self.count_densities_emerged(),
            total_densities: 8,
            emergence_events: self.get_emergence_events(),
            complexity_score: self.collective_state.complexity_score,
            coherence_score: self.collective_state.coherence_score,
        }
    }

    /// Count how many densities have emerged
    fn count_densities_emerged(&self) -> usize {
        let mut count = 0;
        if self.emergence_history.first_density_emerged {
            count += 1;
        }
        if self.emergence_history.second_density_emerged {
            count += 1;
        }
        if self.emergence_history.third_density_emerged {
            count += 1;
        }
        if self.emergence_history.fourth_density_emerged {
            count += 1;
        }
        if self.emergence_history.fifth_density_emerged {
            count += 1;
        }
        if self.emergence_history.sixth_density_emerged {
            count += 1;
        }
        if self.emergence_history.seventh_density_emerged {
            count += 1;
        }
        if self.emergence_history.eighth_density_emerged {
            count += 1;
        }
        count
    }

    /// Get emergence events in chronological order
    fn get_emergence_events(&self) -> Vec<EmergenceEvent> {
        let mut events = vec![];

        // 1st Density (starting point)
        events.push(EmergenceEvent {
            density: "1st Density".to_string(),
            step: 0,
            description: "System initialization".to_string(),
        });

        // 2nd Density
        if let Some(step) = self.emergence_history.second_density_emergence_step {
            events.push(EmergenceEvent {
                density: "2nd Density".to_string(),
                step,
                description: "Biological life emerges from planetary structures".to_string(),
            });
        }

        // 3rd Density
        if let Some(step) = self.emergence_history.third_density_emergence_step {
            events.push(EmergenceEvent {
                density: "3rd Density".to_string(),
                step,
                description: "Self-aware consciousness emerges from complex life".to_string(),
            });
        }

        // 4th Density
        if let Some(step) = self.emergence_history.fourth_density_emergence_step {
            events.push(EmergenceEvent {
                density: "4th Density".to_string(),
                step,
                description: "Understanding and love consciousness emerges".to_string(),
            });
        }

        // 5th Density
        if let Some(step) = self.emergence_history.fifth_density_emergence_step {
            events.push(EmergenceEvent {
                density: "5th Density".to_string(),
                step,
                description: "Wisdom and light consciousness emerges".to_string(),
            });
        }

        // 6th Density
        if let Some(step) = self.emergence_history.sixth_density_emergence_step {
            events.push(EmergenceEvent {
                density: "6th Density".to_string(),
                step,
                description: "Unity and balance consciousness emerges".to_string(),
            });
        }

        // 7th Density
        if let Some(step) = self.emergence_history.seventh_density_emergence_step {
            events.push(EmergenceEvent {
                density: "7th Density".to_string(),
                step,
                description: "Completion consciousness emerges".to_string(),
            });
        }

        // 8th Density
        if let Some(step) = self.emergence_history.eighth_density_emergence_step {
            events.push(EmergenceEvent {
                density: "8th Density".to_string(),
                step,
                description: "Return to Intelligent Infinity".to_string(),
            });
        }

        events.sort_by_key(|e| e.step);
        events
    }
}

/// Density Distribution
///
/// Tracks the count of entities at each density level.
#[derive(Debug, Clone, Default)]
pub struct DensityDistribution {
    // 1st Density sub-levels
    pub first_density_quantum: usize,
    pub first_density_atomic: usize,
    pub first_density_molecular: usize,
    pub first_density_planetary: usize,

    // 2nd Density sub-levels
    pub second_density_cellular: usize,
    pub second_density_simple_life: usize,
    pub second_density_complex_life: usize,

    // Higher densities
    pub third_density: usize,
    pub fourth_density: usize,
    pub fifth_density: usize,
    pub sixth_density: usize,
    pub seventh_density: usize,
    pub eighth_density: usize,
}

impl DensityDistribution {
    fn new() -> Self {
        DensityDistribution::default()
    }

    fn reset(&mut self) {
        *self = DensityDistribution::default();
    }

    pub fn total_entities(&self) -> usize {
        self.first_density_total()
            + self.second_density_total()
            + self.third_density
            + self.fourth_density
            + self.fifth_density
            + self.sixth_density
            + self.seventh_density
            + self.eighth_density
    }

    pub fn first_density_total(&self) -> usize {
        self.first_density_quantum
            + self.first_density_atomic
            + self.first_density_molecular
            + self.first_density_planetary
    }

    pub fn second_density_total(&self) -> usize {
        self.second_density_cellular
            + self.second_density_simple_life
            + self.second_density_complex_life
    }

    pub fn has_second_density_entities(&self) -> bool {
        self.second_density_total() > 0
    }

    pub fn has_third_density_entities(&self) -> bool {
        self.third_density > 0
    }

    pub fn has_fourth_density_entities(&self) -> bool {
        self.fourth_density > 0
    }

    pub fn has_fifth_density_entities(&self) -> bool {
        self.fifth_density > 0
    }

    pub fn has_sixth_density_entities(&self) -> bool {
        self.sixth_density > 0
    }

    pub fn has_seventh_density_entities(&self) -> bool {
        self.seventh_density > 0
    }

    pub fn has_eighth_density_entities(&self) -> bool {
        self.eighth_density > 0
    }

    pub fn get_highest_density(&self) -> Density {
        if self.eighth_density > 0 {
            Density::Eighth
        } else if self.seventh_density > 0 {
            Density::Seventh
        } else if self.sixth_density > 0 {
            Density::Sixth
        } else if self.fifth_density > 0 {
            Density::Fifth
        } else if self.fourth_density > 0 {
            Density::Fourth
        } else if self.third_density > 0 {
            Density::Third
        } else if self.second_density_total() > 0 {
            Density::Second(Density2SubLevel::Cellular)
        } else if self.first_density_planetary > 0 {
            Density::First(Density1SubLevel::Planetary)
        } else if self.first_density_molecular > 0 {
            Density::First(Density1SubLevel::Molecular)
        } else if self.first_density_atomic > 0 {
            Density::First(Density1SubLevel::Atomic)
        } else {
            Density::First(Density1SubLevel::Quantum)
        }
    }

    pub fn calculate_complexity_score(&self) -> f64 {
        // Complexity is based on:
        // 1. Total number of entities
        // 2. Distribution across densities
        // 3. Hierarchical depth

        let total = self.total_entities() as f64;
        if total == 0.0 {
            return 0.0;
        }

        let mut complexity = 0.0;

        // Base complexity from total entities (logarithmic)
        complexity += (total.log10() / 10.0).min(1.0) * 0.3;

        // Complexity from density distribution
        let density_variety = self.count_density_varieties() as f64 / 11.0; // 11 sub-levels total
        complexity += density_variety * 0.4;

        // Complexity from hierarchical depth
        let depth_score = match self.get_highest_density() {
            Density::First(..) => 0.1,
            Density::Second(..) => 0.3,
            Density::Third => 0.5,
            Density::Fourth => 0.7,
            Density::Fifth => 0.8,
            Density::Sixth => 0.9,
            Density::Seventh => 0.95,
            Density::Eighth => 1.0,
        };
        complexity += depth_score * 0.3;

        complexity.min(1.0)
    }

    fn count_density_varieties(&self) -> usize {
        let mut count = 0;
        if self.first_density_quantum > 0 {
            count += 1;
        }
        if self.first_density_atomic > 0 {
            count += 1;
        }
        if self.first_density_molecular > 0 {
            count += 1;
        }
        if self.first_density_planetary > 0 {
            count += 1;
        }
        if self.second_density_cellular > 0 {
            count += 1;
        }
        if self.second_density_simple_life > 0 {
            count += 1;
        }
        if self.second_density_complex_life > 0 {
            count += 1;
        }
        if self.third_density > 0 {
            count += 1;
        }
        if self.fourth_density > 0 {
            count += 1;
        }
        if self.fifth_density > 0 {
            count += 1;
        }
        if self.sixth_density > 0 {
            count += 1;
        }
        if self.seventh_density > 0 {
            count += 1;
        }
        if self.eighth_density > 0 {
            count += 1;
        }
        count
    }
}

/// Emergence History
///
/// Tracks when each density emerged in the collective system.
#[derive(Debug, Clone, Default)]
pub struct EmergenceHistory {
    pub first_density_emerged: bool,
    pub second_density_emerged: bool,
    pub third_density_emerged: bool,
    pub fourth_density_emerged: bool,
    pub fifth_density_emerged: bool,
    pub sixth_density_emerged: bool,
    pub seventh_density_emerged: bool,
    pub eighth_density_emerged: bool,

    pub second_density_emergence_step: Option<usize>,
    pub third_density_emergence_step: Option<usize>,
    pub fourth_density_emergence_step: Option<usize>,
    pub fifth_density_emergence_step: Option<usize>,
    pub sixth_density_emergence_step: Option<usize>,
    pub seventh_density_emergence_step: Option<usize>,
    pub eighth_density_emergence_step: Option<usize>,
}

impl EmergenceHistory {
    fn new() -> Self {
        EmergenceHistory {
            first_density_emerged: true, // 1st Density is the starting point
            ..Default::default()
        }
    }
}

/// Collective System State
///
/// Tracks the overall state of the collective system.
#[derive(Debug, Clone)]
pub struct CollectiveSystemState {
    pub current_step: usize,
    pub highest_density_present: Density,
    pub emergence_percentage: f64,
    pub complexity_score: f64,
    pub coherence_score: f64,
}

impl CollectiveSystemState {
    fn new() -> Self {
        CollectiveSystemState {
            current_step: 0,
            highest_density_present: Density::First(Density1SubLevel::Quantum),
            emergence_percentage: 12.5, // 1st Density = 1/8 * 100
            complexity_score: 0.0,
            coherence_score: 0.0,
        }
    }
}

/// Emergence Configuration
///
/// Configuration for emergence thresholds and conditions.
#[derive(Debug, Clone)]
pub struct EmergenceConfig {
    pub min_planetary_for_second_density: usize,
    pub min_first_density_for_emergence: usize,
    pub min_complex_life_for_third_density: usize,
    pub min_second_density_for_emergence: usize,
    pub min_third_density_for_emergence: usize,
    pub min_coherence_for_emergence: f64,
}

impl Default for EmergenceConfig {
    fn default() -> Self {
        EmergenceConfig {
            min_planetary_for_second_density: 1,
            min_first_density_for_emergence: 50,
            min_complex_life_for_third_density: 5,
            min_second_density_for_emergence: 20,
            min_third_density_for_emergence: 10,
            min_coherence_for_emergence: 0.5,
        }
    }
}

/// Emergence Readiness
///
/// Information about whether emergence conditions are met.
#[derive(Debug, Clone)]
pub struct EmergenceReadiness {
    pub is_ready: bool,
    pub current_percentage: f64,
    pub required_percentage: f64,
    pub conditions_met: Vec<String>,
    pub conditions_missing: Vec<String>,
}

/// Emergence Event
///
/// A record of a density emergence event.
#[derive(Debug, Clone)]
pub struct EmergenceEvent {
    pub density: String,
    pub step: usize,
    pub description: String,
}

/// Emergence Summary
///
/// A summary of the collective system emergence.
#[derive(Debug, Clone)]
pub struct EmergenceSummary {
    pub current_step: usize,
    pub emergence_percentage: f64,
    pub highest_density: String,
    pub densities_emerged: usize,
    pub total_densities: usize,
    pub emergence_events: Vec<EmergenceEvent>,
    pub complexity_score: f64,
    pub coherence_score: f64,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_system_manager_creation() {
        let manager = CollectiveSystemManager::new();

        assert_eq!(manager.collective_state.current_step, 0);
        assert_eq!(manager.emergence_history.first_density_emerged, true);
        assert_eq!(manager.emergence_history.second_density_emerged, false);
        assert_eq!(manager.emergence_history.third_density_emerged, false);
    }

    #[test]
    fn test_density_distribution() {
        let dist = DensityDistribution::default();

        assert_eq!(dist.total_entities(), 0);
        assert_eq!(dist.first_density_total(), 0);
        assert_eq!(dist.second_density_total(), 0);
    }

    #[test]
    fn test_emergence_history() {
        let history = EmergenceHistory::new();

        assert_eq!(history.first_density_emerged, true);
        assert_eq!(history.second_density_emerged, false);
        assert_eq!(history.third_density_emerged, false);
    }

    #[test]
    fn test_collective_system_state() {
        let state = CollectiveSystemState::new();

        assert_eq!(state.current_step, 0);
        assert_eq!(state.emergence_percentage, 12.5); // 1st Density = 1/8 * 100
    }

    #[test]
    fn test_emergence_config_default() {
        let config = EmergenceConfig::default();

        assert_eq!(config.min_planetary_for_second_density, 1);
        assert_eq!(config.min_first_density_for_emergence, 50);
        assert_eq!(config.min_complex_life_for_third_density, 5);
    }
}
