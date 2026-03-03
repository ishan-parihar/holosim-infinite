//! Evolution Path - The Path of Consciousness Evolution
//!
//! From ROADMAP: "Implement evolution (ascent D1→D7) path"
//! From COSMOLOGICAL-ARCHITECTURE: "Transcend and Include applies to all density transitions"
//!
//! ## Overview
//!
//! The evolution path tracks an entity's journey through the density octave,
//! from 1st density through 8th density. Each transition "transcends and includes"
//! the previous development.
//!
//! ## Key Concepts
//!
//! 1. **Transcend and Include**: Each density includes all previous development
//! 2. **Non-linear Progress**: Entities can make leaps through Free Will
//! 3. **Polarization Requirement**: 3rd density and above require polarization
//! 4. **Harvest Points**: Major transition opportunities

use crate::entity_layer7::layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;
use crate::types::{Float, Polarity};
use std::collections::HashMap;

/// Evolution path tracking for an entity
///
/// Tracks the entity's position in the density octave and
/// the milestones achieved along the evolutionary journey.
#[derive(Debug, Clone)]
pub struct EvolutionPath {
    /// Entity ID
    pub entity_id: EntityId,

    /// Current density
    pub current_density: Density,

    /// Evolution milestones achieved
    pub milestones: Vec<EvolutionMilestone>,

    /// Total evolutionary progress (0.0 to 1.0)
    pub overall_progress: Float,

    /// Current polarity (if chosen)
    pub polarity: Option<Polarity>,

    /// Polarity intensity (0.0 to 1.0)
    pub polarity_intensity: Float,

    /// Number of major choices made
    pub major_choices_count: usize,

    /// Time spent in each density (simulation ticks)
    pub time_in_density: HashMap<u8, u64>,

    /// Catalyst experience accumulated in each density
    pub catalyst_by_density: HashMap<u8, Float>,

    /// Choices by polarity direction
    pub sto_choices: usize,

    /// STS choices count
    pub sts_choices: usize,

    /// Neutral choices count
    pub neutral_choices: usize,

    /// Total wisdom accumulated
    pub wisdom_accumulated: Float,

    /// Service rendered (for STO entities)
    pub service_rendered: Float,

    /// Self-enhancement (for STS entities)
    pub self_enhancement: Float,

    /// Whether the entity has awakened
    pub awakened: bool,

    /// Whether harvest-ready
    pub harvest_ready: bool,

    /// Number of harvest cycles experienced
    pub harvest_cycles: usize,
}

/// A milestone in the evolution path
#[derive(Debug, Clone)]
pub struct EvolutionMilestone {
    /// Type of milestone
    pub milestone_type: MilestoneType,

    /// Density at which milestone was achieved
    pub density: Density,

    /// Simulation tick when milestone was achieved
    pub tick: u64,

    /// Significance of the milestone (0.0 to 1.0)
    pub significance: Float,

    /// Optional description
    pub description: Option<String>,
}

/// Types of evolution milestones
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneType {
    /// First emergence as a distinct entity
    Emergence,

    /// Consciousness awakening (I AM moment)
    Awakening,

    /// First polarization choice
    FirstPolarization,

    /// Reached harvestable state
    HarvestReady,

    /// Successful density transition
    DensityTransition,

    /// Major wisdom insight
    WisdomInsight,

    /// Unity consciousness achievement
    UnityConsciousness,

    /// Completion of octave
    OctaveCompletion,

    /// Major service act (STO)
    MajorService,

    /// Major self-enhancement (STS)
    MajorSelfEnhancement,

    /// Catalyst processing complete
    CatalystProcessed,

    /// SMC formation
    SocialMemoryComplexFormed,

    /// Collective harvest
    CollectiveHarvest,
}

/// Progress summary for an evolution path
#[derive(Debug, Clone)]
pub struct EvolutionProgress {
    /// Current density number (1-8)
    pub density_number: u8,

    /// Progress within current density (0.0 to 1.0)
    pub density_progress: Float,

    /// Overall octave progress (0.0 to 1.0)
    pub octave_progress: Float,

    /// Polarization status
    pub polarization_status: String,

    /// Milestones achieved count
    pub milestones_achieved: usize,

    /// Estimated time to next transition
    pub estimated_transition: Option<u64>,

    /// Current evolution stage name
    pub stage_name: String,
}

impl EvolutionPath {
    /// Create a new evolution path starting at 1st density
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            current_density: Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            milestones: Vec::new(),
            overall_progress: 0.0,
            polarity: None,
            polarity_intensity: 0.0,
            major_choices_count: 0,
            time_in_density: HashMap::new(),
            catalyst_by_density: HashMap::new(),
            sto_choices: 0,
            sts_choices: 0,
            neutral_choices: 0,
            wisdom_accumulated: 0.0,
            service_rendered: 0.0,
            self_enhancement: 0.0,
            awakened: false,
            harvest_ready: false,
            harvest_cycles: 0,
        }
    }

    /// Create an evolution path starting at a specific density
    pub fn with_density(entity_id: EntityId, density: Density) -> Self {
        Self {
            entity_id,
            current_density: density,
            milestones: Vec::new(),
            overall_progress: 0.0,
            polarity: None,
            polarity_intensity: 0.0,
            major_choices_count: 0,
            time_in_density: HashMap::new(),
            catalyst_by_density: HashMap::new(),
            sto_choices: 0,
            sts_choices: 0,
            neutral_choices: 0,
            wisdom_accumulated: 0.0,
            service_rendered: 0.0,
            self_enhancement: 0.0,
            awakened: false,
            harvest_ready: false,
            harvest_cycles: 0,
        }
    }

    /// Record a milestone achievement
    pub fn record_milestone(&mut self, milestone: EvolutionMilestone) {
        self.milestones.push(milestone);
        self.recalculate_progress();
    }

    /// Record awakening milestone
    pub fn record_awakening(&mut self, tick: u64) {
        self.awakened = true;
        let milestone = EvolutionMilestone {
            milestone_type: MilestoneType::Awakening,
            density: self.current_density.clone(),
            tick,
            significance: 0.3,
            description: Some("Consciousness awakening - I AM moment".to_string()),
        };
        self.record_milestone(milestone);
    }

    /// Record density transition
    pub fn record_density_transition(&mut self, new_density: Density, tick: u64) {
        let milestone = EvolutionMilestone {
            milestone_type: MilestoneType::DensityTransition,
            density: new_density.clone(),
            tick,
            significance: 0.2,
            description: Some(format!("Transitioned to {}", new_density)),
        };
        self.current_density = new_density;
        self.record_milestone(milestone);
    }

    /// Set polarity
    pub fn set_polarity(&mut self, polarity: Polarity, intensity: Float) {
        self.polarity = Some(polarity);
        self.polarity_intensity = intensity;

        // Record first polarization if this is the first
        if !self
            .milestones
            .iter()
            .any(|m| m.milestone_type == MilestoneType::FirstPolarization)
        {
            let milestone = EvolutionMilestone {
                milestone_type: MilestoneType::FirstPolarization,
                density: self.current_density.clone(),
                tick: 0, // Would be set by simulation
                significance: 0.15,
                description: Some(format!("Chose {} polarity", polarity)),
            };
            self.record_milestone(milestone);
        }
    }

    /// Record a major choice
    pub fn record_major_choice(&mut self, sto_aligned: Option<bool>) {
        self.major_choices_count += 1;

        match sto_aligned {
            Some(true) => {
                self.sto_choices += 1;
                self.service_rendered += 0.05;
            }
            Some(false) => {
                self.sts_choices += 1;
                self.self_enhancement += 0.05;
            }
            None => {
                self.neutral_choices += 1;
            }
        }
    }

    /// Record catalyst experience
    pub fn record_catalyst(&mut self, intensity: Float) {
        let density_num = self.current_density.as_u8();
        *self.catalyst_by_density.entry(density_num).or_insert(0.0) += intensity;
    }

    /// Record time in current density
    pub fn record_time(&mut self, ticks: u64) {
        let density_num = self.current_density.as_u8();
        *self.time_in_density.entry(density_num).or_insert(0) += ticks;
    }

    /// Record wisdom gain
    pub fn record_wisdom(&mut self, amount: Float) {
        self.wisdom_accumulated = (self.wisdom_accumulated + amount).min(1.0);

        // Check for wisdom insight milestone
        if self.wisdom_accumulated >= 0.5
            && !self
                .milestones
                .iter()
                .any(|m| m.milestone_type == MilestoneType::WisdomInsight)
        {
            let milestone = EvolutionMilestone {
                milestone_type: MilestoneType::WisdomInsight,
                density: self.current_density.clone(),
                tick: 0,
                significance: 0.1,
                description: Some("Achieved significant wisdom".to_string()),
            };
            self.record_milestone(milestone);
        }
    }

    /// Mark as harvest ready
    pub fn set_harvest_ready(&mut self, tick: u64) {
        self.harvest_ready = true;
        self.harvest_cycles += 1;

        let milestone = EvolutionMilestone {
            milestone_type: MilestoneType::HarvestReady,
            density: self.current_density.clone(),
            tick,
            significance: 0.25,
            description: Some("Reached harvestable state".to_string()),
        };
        self.record_milestone(milestone);
    }

    /// Record SMC formation
    pub fn record_smc_formation(&mut self, tick: u64) {
        let milestone = EvolutionMilestone {
            milestone_type: MilestoneType::SocialMemoryComplexFormed,
            density: self.current_density.clone(),
            tick,
            significance: 0.2,
            description: Some("Joined Social Memory Complex".to_string()),
        };
        self.record_milestone(milestone);
    }

    /// Recalculate overall progress
    fn recalculate_progress(&mut self) {
        // Base progress from density
        let density_progress = match self.current_density {
            Density::First(_) => 0.0,
            Density::Second(_) => 0.125,
            Density::Third => 0.25,
            Density::Fourth => 0.375,
            Density::Fifth => 0.5,
            Density::Sixth => 0.625,
            Density::Seventh => 0.75,
            Density::Eighth => 0.875,
        };

        // Add milestone contributions
        let milestone_progress: Float = self
            .milestones
            .iter()
            .map(|m| m.significance * 0.1)
            .sum::<Float>()
            .min(0.125);

        self.overall_progress = (density_progress + milestone_progress).min(1.0);
    }

    /// Check if entity is at or above a specific density
    pub fn is_at_least(&self, density: &Density) -> bool {
        self.current_density.as_u8() >= density.as_u8()
    }

    /// Get the number of densities completed
    pub fn densities_completed(&self) -> usize {
        (self.current_density.as_u8() as usize).saturating_sub(1)
    }

    /// Check if entity has achieved a specific milestone
    pub fn has_milestone(&self, milestone_type: MilestoneType) -> bool {
        self.milestones
            .iter()
            .any(|m| m.milestone_type == milestone_type)
    }

    /// Get progress summary
    pub fn get_progress_summary(&self) -> EvolutionProgress {
        let density_number = self.current_density.as_u8();

        // Calculate density progress from time spent and milestones
        let time_in_current = self
            .time_in_density
            .get(&density_number)
            .copied()
            .unwrap_or(0);
        let density_progress = (time_in_current as Float / 1000.0).min(1.0);

        // Polarization status
        let polarization_status = match self.polarity {
            Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                format!("STO {:.0}%", self.polarity_intensity * 100.0)
            }
            Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => {
                format!("STS {:.0}%", self.polarity_intensity * 100.0)
            }
            _ => "Unpolarized".to_string(),
        };

        // Stage name
        let stage_name = match density_number {
            1 => "Awareness",
            2 => "Growth",
            3 => "Self-Awareness",
            4 => "Love/Understanding",
            5 => "Light/Wisdom",
            6 => "Unity",
            7 => "Completion",
            8 => "Return",
            _ => "Unknown",
        }
        .to_string();

        EvolutionProgress {
            density_number,
            density_progress,
            octave_progress: self.overall_progress,
            polarization_status,
            milestones_achieved: self.milestones.len(),
            estimated_transition: None, // Would require simulation context
            stage_name,
        }
    }

    /// Get total catalyst experienced
    pub fn total_catalyst(&self) -> Float {
        self.catalyst_by_density.values().sum()
    }

    /// Get polarity ratio (STO vs STS choices)
    pub fn polarity_ratio(&self) -> Float {
        let total = self.sto_choices + self.sts_choices;
        if total == 0 {
            return 0.5; // Neutral
        }
        self.sto_choices as Float / total as Float
    }

    /// Check if entity meets harvest requirements
    pub fn is_harvest_eligible(&self) -> bool {
        match self.polarity {
            Some(Polarity::STO) | Some(Polarity::ServiceToOthers) => {
                self.polarity_intensity >= 0.51
            }
            Some(Polarity::STS) | Some(Polarity::ServiceToSelf) => self.polarity_intensity >= 0.95,
            _ => false,
        }
    }
}

impl EvolutionMilestone {
    /// Create a new milestone
    pub fn new(
        milestone_type: MilestoneType,
        density: Density,
        tick: u64,
        significance: Float,
    ) -> Self {
        Self {
            milestone_type,
            density,
            tick,
            significance,
            description: None,
        }
    }

    /// Create with description
    pub fn with_description(
        milestone_type: MilestoneType,
        density: Density,
        tick: u64,
        significance: Float,
        description: &str,
    ) -> Self {
        Self {
            milestone_type,
            density,
            tick,
            significance,
            description: Some(description.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};

    #[test]
    fn test_evolution_path_creation() {
        let path = EvolutionPath::new(EntityId::new("test".to_string()));

        assert!(matches!(path.current_density, Density::First(_)));
        assert_eq!(path.milestones.len(), 0);
        assert_eq!(path.polarity, None);
        assert!(!path.awakened);
    }

    #[test]
    fn test_record_awakening() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.record_awakening(100);

        assert!(path.awakened);
        assert!(path.has_milestone(MilestoneType::Awakening));
        assert_eq!(path.milestones.len(), 1);
    }

    #[test]
    fn test_record_density_transition() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.record_density_transition(Density::Third, 200);

        assert_eq!(path.current_density, Density::Third);
        assert!(path.has_milestone(MilestoneType::DensityTransition));
    }

    #[test]
    fn test_set_polarity() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.set_polarity(Polarity::STO, 0.6);

        assert_eq!(path.polarity, Some(Polarity::STO));
        assert!((path.polarity_intensity - 0.6).abs() < 0.001);
        assert!(path.has_milestone(MilestoneType::FirstPolarization));
    }

    #[test]
    fn test_record_major_choice() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        path.record_major_choice(Some(true)); // STO
        path.record_major_choice(Some(true)); // STO
        path.record_major_choice(Some(false)); // STS

        assert_eq!(path.sto_choices, 2);
        assert_eq!(path.sts_choices, 1);
        assert_eq!(path.major_choices_count, 3);
    }

    #[test]
    fn test_record_catalyst() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        path.record_catalyst(0.1);
        path.record_catalyst(0.2);

        assert!((path.total_catalyst() - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_record_wisdom() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        path.record_wisdom(0.3);
        assert!((path.wisdom_accumulated - 0.3).abs() < 0.001);

        path.record_wisdom(0.3);
        assert!(path.has_milestone(MilestoneType::WisdomInsight));
    }

    #[test]
    fn test_is_at_least() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.current_density = Density::Third;

        assert!(path.is_at_least(&Density::Third));
        assert!(path.is_at_least(&Density::Second(Density2SubLevel::Cellular)));
        assert!(!path.is_at_least(&Density::Fourth));
    }

    #[test]
    fn test_densities_completed() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        path.current_density = Density::First(Density1SubLevel::Quantum);
        assert_eq!(path.densities_completed(), 0);

        path.current_density = Density::Third;
        assert_eq!(path.densities_completed(), 2);

        path.current_density = Density::Eighth;
        assert_eq!(path.densities_completed(), 7);
    }

    #[test]
    fn test_overall_progress() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        // Starting progress
        let initial = path.overall_progress;

        // Add milestones
        path.record_awakening(100);
        assert!(path.overall_progress > initial);

        // Transition density
        path.record_density_transition(Density::Third, 200);
        assert!(path.overall_progress > initial);
    }

    #[test]
    fn test_polarity_ratio() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));

        // Neutral when no choices
        assert!((path.polarity_ratio() - 0.5).abs() < 0.001);

        path.record_major_choice(Some(true)); // STO
        path.record_major_choice(Some(true)); // STO
        path.record_major_choice(Some(false)); // STS

        // 2 STO, 1 STS = 66.7% STO
        assert!((path.polarity_ratio() - 0.667).abs() < 0.01);
    }

    #[test]
    fn test_harvest_eligibility_sto() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.current_density = Density::Third;

        // Below threshold
        path.set_polarity(Polarity::STO, 0.4);
        assert!(!path.is_harvest_eligible());

        // At threshold
        path.set_polarity(Polarity::STO, 0.51);
        assert!(path.is_harvest_eligible());

        // Above threshold
        path.set_polarity(Polarity::STO, 0.7);
        assert!(path.is_harvest_eligible());
    }

    #[test]
    fn test_harvest_eligibility_sts() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.current_density = Density::Third;

        // Below threshold
        path.set_polarity(Polarity::STS, 0.9);
        assert!(!path.is_harvest_eligible());

        // At threshold
        path.set_polarity(Polarity::STS, 0.95);
        assert!(path.is_harvest_eligible());
    }

    #[test]
    fn test_get_progress_summary() {
        let mut path = EvolutionPath::new(EntityId::new("test".to_string()));
        path.set_polarity(Polarity::STO, 0.6);
        path.record_awakening(100);

        let summary = path.get_progress_summary();

        assert_eq!(summary.density_number, 1);
        assert!(summary.polarization_status.contains("STO"));
        assert!(summary.milestones_achieved > 0);
    }

    #[test]
    fn test_milestone_with_description() {
        let milestone = EvolutionMilestone::with_description(
            MilestoneType::Awakening,
            Density::Third,
            100,
            0.5,
            "Test description",
        );

        assert!(milestone.description.is_some());
        assert!(milestone.description.unwrap().contains("Test"));
    }
}
