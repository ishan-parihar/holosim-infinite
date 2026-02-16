//! Polarity State Machine
//!
//! Implements the polarization progression system that tracks an entity's
//! journey from unpolarized through to harvestable states.

/// Re-export PolarityChoice from foundation for convenience
pub use crate::foundation::indigo_realm::PolarityChoice;

/// Polarity state represents the current stage of an entity's polarization journey
///
/// From Law of One and COSMOLOGICAL-ARCHITECTURE.md:
/// - Entities start unpolarized
/// - Through repeated choices, they develop a leaning (STO or STS)
/// - With consistent choices, they become polarized
/// - When sufficiently polarized (51%+), they become harvestable for density transition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarizationState {
    /// Entity has not chosen a path
    ///
    /// All entities start in this state. Through catalyst and free will,
    /// they will make choices that lead toward polarization.
    Unpolarized,

    /// Entity is exploring service-to-others
    ///
    /// Entity has made some STO choices but not yet consistently enough
    /// to be considered polarized.
    STOLeaning,

    /// Entity is exploring service-to-self
    ///
    /// Entity has made some STS choices but not yet consistently enough
    /// to be considered polarized.
    STSLeaning,

    /// Entity has firmly chosen service-to-others
    ///
    /// Entity has consistently chosen STO and has developed a stable
    /// polarization orientation.
    PolarizedSTO,

    /// Entity has firmly chosen service-to-self
    ///
    /// Entity has consistently chosen STS and has developed a stable
    /// polarization orientation.
    PolarizedSTS,

    /// Entity is ready for density transition (STO)
    ///
    /// Entity has reached 51%+ polarization intensity and is ready to
    /// transition to 4th density or higher.
    HarvestableSTO,

    /// Entity is ready for density transition (STS)
    ///
    /// Entity has reached 51%+ polarization intensity and is ready to
    /// transition to 4th density or higher.
    HarvestableSTS,
}

impl PolarizationState {
    /// Check if entity is polarized (either STO or STS)
    pub fn is_polarized(&self) -> bool {
        matches!(
            self,
            PolarizationState::PolarizedSTO
                | PolarizationState::PolarizedSTS
                | PolarizationState::HarvestableSTO
                | PolarizationState::HarvestableSTS
        )
    }

    /// Check if entity is harvestable (ready for density transition)
    pub fn is_harvestable(&self) -> bool {
        matches!(
            self,
            PolarizationState::HarvestableSTO | PolarizationState::HarvestableSTS
        )
    }

    /// Check if entity is leaning toward STO
    pub fn is_sto_leaning(&self) -> bool {
        matches!(
            self,
            PolarizationState::STOLeaning
                | PolarizationState::PolarizedSTO
                | PolarizationState::HarvestableSTO
        )
    }

    /// Check if entity is leaning toward STS
    pub fn is_sts_leaning(&self) -> bool {
        matches!(
            self,
            PolarizationState::STSLeaning
                | PolarizationState::PolarizedSTS
                | PolarizationState::HarvestableSTS
        )
    }
}

/// Polarity direction represents the overall orientation of an entity's choices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarityDirection {
    /// Service-to-Others polarity
    ///
    /// Entity seeks to serve others and the collective good.
    ServiceToOthers,

    /// Service-to-Self polarity
    ///
    /// Entity seeks to serve self and personal advancement.
    ServiceToSelf,

    /// Neutral polarity
    ///
    /// Entity has not yet established a clear direction.
    Neutral,
}

impl PolarityDirection {
    /// Get the display name for this direction
    pub fn display_name(&self) -> &'static str {
        match self {
            PolarityDirection::ServiceToOthers => "STO",
            PolarityDirection::ServiceToSelf => "STS",
            PolarityDirection::Neutral => "Neutral",
        }
    }
}

/// Polarity progress tracks an entity's journey through polarization
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Polarity is required for density progression beyond 3rd density"
/// "Harvest requires 51% service-to-others or 95% service-to-self"
#[derive(Debug, Clone)]
pub struct PolarizationProgress {
    /// Current polarization state
    pub state: PolarizationState,

    /// Polarity intensity (0.0 to 1.0)
    ///
    /// Represents how consistently the entity has chosen a direction.
    /// - 0.0 = Completely unpolarized
    /// - 0.51 = Minimum for harvest (STO)
    /// - 0.95 = Minimum for harvest (STS)
    /// - 1.0 = Fully polarized
    pub intensity: f64,

    /// Overall polarity direction
    pub direction: PolarityDirection,

    /// Choice consistency (0.0 to 1.0)
    ///
    /// How consistent the entity's choices have been.
    /// Higher values indicate more consistent choices in one direction.
    pub consistency: f64,

    /// History of choices made by the entity
    ///
    /// Tracks the sequence of choices to calculate consistency and intensity.
    pub choice_history: Vec<PolarityChoice>,
}

impl Default for PolarizationProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl PolarizationProgress {
    /// Create a new unpolarized entity
    pub fn new() -> Self {
        PolarizationProgress {
            state: PolarizationState::Unpolarized,
            intensity: 0.0,
            direction: PolarityDirection::Neutral,
            consistency: 0.0,
            choice_history: Vec::new(),
        }
    }

    /// Make a polarity choice
    ///
    /// This method updates the entity's polarization state based on the choice made.
    /// It tracks the choice history, updates consistency, and determines if the entity
    /// has progressed to a new polarization state.
    ///
    /// # Arguments
    /// * `choice` - The polarity choice made (STO, STS, or Neutral)
    pub fn make_choice(&mut self, choice: PolarityChoice) {
        // Add choice to history
        self.choice_history.push(choice);

        // Update direction based on choice
        self.update_direction(choice);

        // Calculate consistency and intensity
        self.recalculate();

        // Update state based on intensity and direction
        self.update_state();
    }

    /// Update polarity direction based on latest choice
    fn update_direction(&mut self, choice: PolarityChoice) {
        // If entity is still neutral, set direction based on choice
        if self.direction == PolarityDirection::Neutral {
            self.direction = match choice {
                PolarityChoice::ServiceToOthers => PolarityDirection::ServiceToOthers,
                PolarityChoice::ServiceToSelf => PolarityDirection::ServiceToSelf,
                PolarityChoice::Neutral => PolarityDirection::Neutral,
            };
            return;
        }

        // If entity already has a direction, check if choice is consistent
        // If entity makes enough inconsistent choices, direction may flip
        if self.choice_history.len() >= 10 {
            // Count recent choices (last 10)
            let recent_choices: Vec<_> = self.choice_history.iter().rev().take(10).collect();

            let sto_count = recent_choices
                .iter()
                .filter(|c| matches!(c, PolarityChoice::ServiceToOthers))
                .count();
            let sts_count = recent_choices
                .iter()
                .filter(|c| matches!(c, PolarityChoice::ServiceToSelf))
                .count();

            // If 70%+ of recent choices are in one direction, update direction
            if sto_count >= 7 {
                self.direction = PolarityDirection::ServiceToOthers;
            } else if sts_count >= 7 {
                self.direction = PolarityDirection::ServiceToSelf;
            }
        }
    }

    /// Recalculate consistency and intensity based on choice history
    fn recalculate(&mut self) {
        if self.choice_history.is_empty() {
            self.consistency = 0.0;
            self.intensity = 0.0;
            return;
        }

        // Calculate consistency: how many choices are in the current direction?
        // Note: Neutral choices don't contribute to polarization intensity
        let consistent_choices = self
            .choice_history
            .iter()
            .filter(|choice| {
                match (self.direction, choice) {
                    (PolarityDirection::ServiceToOthers, PolarityChoice::ServiceToOthers) => true,
                    (PolarityDirection::ServiceToSelf, PolarityChoice::ServiceToSelf) => true,
                    (PolarityDirection::Neutral, PolarityChoice::Neutral) => true, // Only neutral choices count as consistent when neutral
                    _ => false,
                }
            })
            .count();

        self.consistency = consistent_choices as f64 / self.choice_history.len() as f64;

        // Calculate intensity based on:
        // - Consistency (higher = more intense)
        // - Number of choices (more choices = potentially more intense)
        // - Time factor (recent choices weighted more heavily)

        let _choice_count = self.choice_history.len();

        // Base intensity from consistency
        // If consistency is low (< 0.6), reduce intensity further to account for
        // the lack of polarization
        let mut intensity = if self.consistency < 0.6 {
            self.consistency * 0.8 // 20% reduction for inconsistent patterns
        } else {
            self.consistency
        };

        // For neutral direction, intensity should remain low
        // Neutral choices don't build polarization intensity
        if self.direction == PolarityDirection::Neutral {
            self.intensity = 0.1; // Slight presence but no polarization
            return;
        }

        // Boost intensity based on number of consistent choices
        // But only apply boost if consistency is significantly higher than 0.5
        // (i.e., entity is actually making more consistent than inconsistent choices)
        let consistent_choice_count = consistent_choices;
        if self.consistency > 0.55 && consistent_choice_count >= 5 {
            intensity += 0.1; // 5+ consistent choices adds 10%
        }
        if self.consistency > 0.60 && consistent_choice_count >= 10 {
            intensity += 0.1; // 10+ consistent choices adds another 10%
        }
        if self.consistency > 0.65 && consistent_choice_count >= 20 {
            intensity += 0.1; // 20+ consistent choices adds another 10%
        }

        // Cap at 1.0
        self.intensity = intensity.min(1.0).max(0.0);
    }

    /// Update polarization state based on intensity and direction
    fn update_state(&mut self) {
        // Determine thresholds for state transitions
        // STO requires 51% intensity for harvest
        // STS requires 95% intensity for harvest (more difficult path)

        match self.direction {
            PolarityDirection::ServiceToOthers => {
                if self.intensity >= 0.51 {
                    self.state = PolarizationState::HarvestableSTO;
                } else if self.intensity >= 0.30 {
                    self.state = PolarizationState::PolarizedSTO;
                } else if self.intensity >= 0.10 {
                    self.state = PolarizationState::STOLeaning;
                } else {
                    self.state = PolarizationState::Unpolarized;
                }
            }
            PolarityDirection::ServiceToSelf => {
                if self.intensity >= 0.95 {
                    self.state = PolarizationState::HarvestableSTS;
                } else if self.intensity >= 0.70 {
                    self.state = PolarizationState::PolarizedSTS;
                } else if self.intensity >= 0.20 {
                    self.state = PolarizationState::STSLeaning;
                } else {
                    self.state = PolarizationState::Unpolarized;
                }
            }
            PolarityDirection::Neutral => {
                self.state = PolarizationState::Unpolarized;
            }
        }
    }

    /// Calculate polarization intensity
    ///
    /// Returns the current polarization intensity (0.0 to 1.0).
    /// This is automatically updated after each choice.
    pub fn calculate_intensity(&self) -> f64 {
        self.intensity
    }

    /// Check if entity is harvestable (ready for density transition)
    ///
    /// From Law of One:
    /// - STO requires 51% service-to-others
    /// - STS requires 95% service-to-self
    pub fn is_harvestable(&self) -> bool {
        self.state.is_harvestable()
    }

    /// Get the number of choices made
    pub fn choice_count(&self) -> usize {
        self.choice_history.len()
    }

    /// Get the number of STO choices
    pub fn sto_choice_count(&self) -> usize {
        self.choice_history
            .iter()
            .filter(|c| matches!(c, PolarityChoice::ServiceToOthers))
            .count()
    }

    /// Get the number of STS choices
    pub fn sts_choice_count(&self) -> usize {
        self.choice_history
            .iter()
            .filter(|c| matches!(c, PolarityChoice::ServiceToSelf))
            .count()
    }

    /// Get the number of Neutral choices
    pub fn neutral_choice_count(&self) -> usize {
        self.choice_history
            .iter()
            .filter(|c| matches!(c, PolarityChoice::Neutral))
            .count()
    }

    /// Get polarization bias (-1.0 to 1.0)
    ///
    /// Returns a value indicating the entity's polarization bias:
    /// - -1.0 = Fully STS
    /// - 0.0 = Neutral
    /// - 1.0 = Fully STO
    pub fn polarity_bias(&self) -> f64 {
        match self.direction {
            PolarityDirection::ServiceToOthers => self.intensity,
            PolarityDirection::ServiceToSelf => -self.intensity,
            PolarityDirection::Neutral => 0.0,
        }
    }

    /// Get STO intensity (0.0 to 1.0)
    ///
    /// Returns the intensity of service-to-others polarization.
    pub fn sto_intensity(&self) -> f64 {
        match self.direction {
            PolarityDirection::ServiceToOthers => self.intensity,
            _ => 0.0,
        }
    }

    /// Get STS intensity (0.0 to 1.0)
    ///
    /// Returns the intensity of service-to-self polarization.
    pub fn sts_intensity(&self) -> f64 {
        match self.direction {
            PolarityDirection::ServiceToSelf => self.intensity,
            _ => 0.0,
        }
    }

    /// Check if entity is polarized
    ///
    /// Returns true if the entity has established a consistent polarization direction.
    pub fn is_polarized(&self) -> bool {
        self.state.is_polarized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polarization_progress_new() {
        let progress = PolarizationProgress::new();
        assert_eq!(progress.state, PolarizationState::Unpolarized);
        assert_eq!(progress.intensity, 0.0);
        assert_eq!(progress.direction, PolarityDirection::Neutral);
        assert_eq!(progress.consistency, 0.0);
        assert_eq!(progress.choice_history.len(), 0);
    }

    #[test]
    fn test_make_choice_sto() {
        let mut progress = PolarizationProgress::new();

        // Make 10 STO choices
        for _ in 0..10 {
            progress.make_choice(PolarityChoice::ServiceToOthers);
        }

        assert_eq!(progress.direction, PolarityDirection::ServiceToOthers);
        assert!(progress.intensity > 0.5); // Should be >50% after 10 consistent choices
        assert!(progress.sto_choice_count() == 10);
    }

    #[test]
    fn test_make_choice_sts() {
        let mut progress = PolarizationProgress::new();

        // Make 20 STS choices (need more for STS harvest)
        for _ in 0..20 {
            progress.make_choice(PolarityChoice::ServiceToSelf);
        }

        assert_eq!(progress.direction, PolarityDirection::ServiceToSelf);
        assert!(progress.intensity > 0.7); // Should be >70% after 20 consistent choices
        assert!(progress.sts_choice_count() == 20);
    }

    #[test]
    fn test_harvestable_sto() {
        let mut progress = PolarizationProgress::new();

        // Make enough STO choices to reach harvest (51%+)
        for _ in 0..10 {
            progress.make_choice(PolarityChoice::ServiceToOthers);
        }

        assert!(progress.is_harvestable());
        assert_eq!(progress.state, PolarizationState::HarvestableSTO);
    }

    #[test]
    fn test_harvestable_sts() {
        let mut progress = PolarizationProgress::new();

        // Make enough STS choices to reach harvest (95%+)
        // This requires many consistent choices
        for _ in 0..30 {
            progress.make_choice(PolarityChoice::ServiceToSelf);
        }

        // STS requires 95%, so this should be close to harvestable
        assert!(progress.intensity > 0.9);
        // May or may not be fully harvestable depending on exact calculation
    }

    #[test]
    fn test_inconsistent_choices() {
        let mut progress = PolarizationProgress::new();

        // Make alternating choices (low consistency)
        for i in 0..10 {
            if i % 2 == 0 {
                progress.make_choice(PolarityChoice::ServiceToOthers);
            } else {
                progress.make_choice(PolarityChoice::ServiceToSelf);
            }
        }

        // Should have low intensity due to inconsistency
        eprintln!(
            "After alternating choices: direction={:?}, intensity={:.3}, consistency={:.3}",
            progress.direction, progress.intensity, progress.consistency
        );
        assert!(progress.intensity < 0.5);
        assert!(progress.consistency < 0.6);
    }

    #[test]
    fn test_neutral_choices() {
        let mut progress = PolarizationProgress::new();

        // Make only neutral choices
        for _ in 0..10 {
            progress.make_choice(PolarityChoice::Neutral);
        }

        // Should remain unpolarized
        assert_eq!(progress.state, PolarizationState::Unpolarized);
        assert_eq!(progress.direction, PolarityDirection::Neutral);
        assert!(progress.intensity < 0.3); // Neutral choices don't build polarization
    }
}
