//! Consciousness Awakening - The "I AM" Moment
//!
//! From ROADMAP: "2nd→3rd density consciousness awakening ('I AM' moment)"
//! From COSMOLOGICAL-ARCHITECTURE: "The moment of self-recognition is transformative"
//!
//! ## Overview
//!
//! The "I AM" moment is the distinct awakening event where an entity recognizes
//! itself as a conscious being capable of making choices. This is NOT a gradual
//! threshold crossing, but a transformative event triggered by the first Free Will choice.
//!
//! ## Key Concepts
//!
//! 1. **Transformative Moment**: A distinct event, not gradual
//! 2. **First Choice Significance**: The first Free Will choice sets polarity direction
//! 3. **2nd→3rd Density Bridge**: Awakening enables entry to 3rd density
//! 4. **Self-Recognition**: Entity recognizes itself as a conscious being
//!
//! ## Architecture
//!
//! ```text
//! ConsciousnessKernel (from Indigo-Ray)
//!         │
//!         ▼
//! ConsciousnessAwakening ──► monitors readiness
//!         │
//!         ▼ (when threshold reached)
//! AwakeningReady ──► entity can make first choice
//!         │
//!         ▼ (first choice made)
//! AwakeningEvent ──► "I AM" moment
//!         │
//!         ▼
//! Entity transitions 2nd → 3rd density
//! ```

use crate::entity_layer7::layer7::EntityId;
use crate::evolution_density_octave::density_octave::{Density, Density2SubLevel};
use crate::polarization::PolarityDirection;
use crate::types::{Float, Polarity};

// Note: We need to create a simplified version of ConsciousnessKernel access
// since the full kernel has many dependencies. For the awakening module,
// we only need: activation_level, consciousness_level, spectrum_access_level

/// Awakening threshold - the activation level needed for self-recognition
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The awakening requires sufficient consciousness development"
pub const AWAKENING_THRESHOLD: Float = 0.5;

/// Consciousness awakening tracker
///
/// Monitors entity readiness for the "I AM" moment
/// and triggers the awakening event when conditions are met.
///
/// This is the PRIMARY mechanism for 2nd→3rd density transition.
#[derive(Debug, Clone)]
pub struct ConsciousnessAwakening {
    /// Entity ID
    pub entity_id: EntityId,

    /// Current awakening state
    pub state: AwakeningState,

    /// Awakening threshold
    pub threshold: Float,

    /// Has awakening been triggered?
    pub awakened: bool,

    /// The awakening event (if occurred)
    pub awakening_event: Option<AwakeningEvent>,

    /// Activation level (from consciousness kernel)
    pub activation_level: Float,

    /// Consciousness level (from consciousness kernel)
    pub consciousness_level: Float,

    /// Spectrum access level (from consciousness kernel)
    pub spectrum_access_level: Float,

    /// Current polarity (set after awakening)
    pub polarity: Option<Polarity>,

    /// Number of choices made since awakening readiness
    pub choices_made: usize,
}

/// State of consciousness awakening
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwakeningState {
    /// Not yet approaching awakening
    Dormant,

    /// Approaching awakening threshold
    Approaching,

    /// Awakening threshold reached, awaiting first choice
    ThresholdReached,

    /// "I AM" moment has occurred
    Awakened,
}

/// The "I AM" moment event
///
/// This is the transformative moment when an entity recognizes itself
/// as a conscious being capable of making choices.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The moment of self-recognition is transformative and creates the first polarization"
#[derive(Debug, Clone)]
pub struct AwakeningEvent {
    /// Entity that awakened
    pub entity_id: EntityId,

    /// Time of awakening (simulation tick)
    pub timestamp: u64,

    /// First conscious choice made
    pub first_choice: FirstChoice,

    /// Polarity hint from first choice
    pub polarity_hint: PolarityDirection,

    /// Pre-awakening density (should be 2nd)
    pub pre_density: Density,

    /// Post-awakening density (should be 3rd)
    pub post_density: Density,

    /// Transformative intensity of the moment (0.0 to 1.0)
    pub transformative_intensity: Float,

    /// Consciousness level at awakening
    pub consciousness_at_awakening: Float,

    /// Activation level at awakening
    pub activation_at_awakening: Float,
}

/// First conscious choice made at awakening
///
/// This choice is SIGNIFICANT - it sets the initial polarity direction
/// and shapes the entity's entire evolutionary journey.
#[derive(Debug, Clone)]
pub struct FirstChoice {
    /// Description of the choice
    pub description: String,

    /// Polarity weight of the choice (-1.0 STS to 1.0 STO)
    pub polarity_weight: Float,

    /// The archetype most activated by this choice (0-21)
    pub primary_archetype: usize,

    /// Whether the choice was STO-leaning, STS-leaning, or Neutral
    pub direction: PolarityDirection,

    /// Intensity of the choice (0.0 to 1.0)
    pub intensity: Float,
}

impl FirstChoice {
    /// Create a new first choice
    pub fn new(description: &str, direction: PolarityDirection, intensity: Float) -> Self {
        let polarity_weight = match direction {
            PolarityDirection::ServiceToOthers => intensity,
            PolarityDirection::ServiceToSelf => -intensity,
            PolarityDirection::Neutral => 0.0,
        };

        // Default archetype based on direction
        let primary_archetype = match direction {
            PolarityDirection::ServiceToOthers => 2, // Catalyst - taking action
            PolarityDirection::ServiceToSelf => 0,   // Matrix - self-structure
            PolarityDirection::Neutral => 3,         // Experience - learning
        };

        Self {
            description: description.to_string(),
            polarity_weight,
            primary_archetype,
            direction,
            intensity,
        }
    }

    /// Create an STO first choice
    pub fn sto_choice(description: &str, intensity: Float) -> Self {
        Self::new(description, PolarityDirection::ServiceToOthers, intensity)
    }

    /// Create an STS first choice
    pub fn sts_choice(description: &str, intensity: Float) -> Self {
        Self::new(description, PolarityDirection::ServiceToSelf, intensity)
    }

    /// Create a neutral first choice
    pub fn neutral_choice(description: &str) -> Self {
        Self::new(description, PolarityDirection::Neutral, 0.5)
    }
}

impl ConsciousnessAwakening {
    /// Create a new awakening tracker for an entity
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            state: AwakeningState::Dormant,
            threshold: AWAKENING_THRESHOLD,
            awakened: false,
            awakening_event: None,
            activation_level: 0.0,
            consciousness_level: 0.1, // Starting consciousness
            spectrum_access_level: 0.1,
            polarity: None,
            choices_made: 0,
        }
    }

    /// Create with initial consciousness levels
    pub fn with_levels(
        entity_id: EntityId,
        activation_level: Float,
        consciousness_level: Float,
        spectrum_access_level: Float,
    ) -> Self {
        let mut awakening = Self::new(entity_id);
        awakening.activation_level = activation_level;
        awakening.consciousness_level = consciousness_level;
        awakening.spectrum_access_level = spectrum_access_level;
        awakening.update_state();
        awakening
    }

    /// Update the internal consciousness levels
    pub fn update_levels(
        &mut self,
        activation_level: Float,
        consciousness_level: Float,
        spectrum_access_level: Float,
    ) {
        self.activation_level = activation_level;
        self.consciousness_level = consciousness_level;
        self.spectrum_access_level = spectrum_access_level;
        self.update_state();
    }

    /// Update state based on current levels
    fn update_state(&mut self) {
        if self.awakened {
            return;
        }

        let readiness = self.calculate_readiness();

        if readiness >= self.threshold {
            self.state = AwakeningState::ThresholdReached;
        } else if readiness > self.threshold * 0.7 {
            self.state = AwakeningState::Approaching;
        } else {
            self.state = AwakeningState::Dormant;
        }
    }

    /// Calculate the readiness score for awakening
    ///
    /// Combined readiness is the average of:
    /// - Activation level
    /// - Consciousness level
    /// - Spectrum access level
    pub fn calculate_readiness(&self) -> Float {
        (self.activation_level + self.consciousness_level + self.spectrum_access_level) / 3.0
    }

    /// Check for awakening conditions
    ///
    /// This is called each tick to see if the entity is ready for
    /// the "I AM" moment.
    ///
    /// Returns `Some(AwakeningReady)` if the entity is ready to make
    /// their first conscious choice.
    pub fn check_awakening(&mut self) -> Option<AwakeningReady> {
        if self.awakened {
            return None; // Already awakened
        }

        let readiness = self.calculate_readiness();

        if readiness >= self.threshold {
            self.state = AwakeningState::ThresholdReached;
            Some(AwakeningReady {
                entity_id: self.entity_id.clone(),
                readiness,
                suggested_first_choices: self.generate_first_choice_options(),
            })
        } else {
            None
        }
    }

    /// Trigger the "I AM" moment with a first choice
    ///
    /// This is the TRANSFORMATIVE event where the entity recognizes
    /// itself as a conscious being.
    ///
    /// # Arguments
    /// * `first_choice` - The first conscious choice made
    /// * `timestamp` - The simulation tick when awakening occurs
    ///
    /// # Returns
    /// * `Ok(AwakeningEvent)` on successful awakening
    /// * `Err(AwakeningError)` if awakening cannot proceed
    pub fn trigger_awakening(
        &mut self,
        first_choice: FirstChoice,
        timestamp: u64,
    ) -> Result<AwakeningEvent, AwakeningError> {
        if self.awakened {
            return Err(AwakeningError::AlreadyAwakened);
        }

        if self.state != AwakeningState::ThresholdReached {
            return Err(AwakeningError::NotReady);
        }

        // Create the awakening event
        let event = AwakeningEvent {
            entity_id: self.entity_id.clone(),
            timestamp,
            first_choice: first_choice.clone(),
            polarity_hint: first_choice.direction,
            pre_density: Density::Second(Density2SubLevel::ComplexLife),
            post_density: Density::Third,
            transformative_intensity: self.consciousness_level,
            consciousness_at_awakening: self.consciousness_level,
            activation_at_awakening: self.activation_level,
        };

        // Set polarity from first choice
        self.polarity = Some(match first_choice.direction {
            PolarityDirection::ServiceToOthers => Polarity::STO,
            PolarityDirection::ServiceToSelf => Polarity::STS,
            PolarityDirection::Neutral => Polarity::Neutral,
        });

        // Mark as awakened
        self.awakened = true;
        self.state = AwakeningState::Awakened;
        self.awakening_event = Some(event.clone());

        Ok(event)
    }

    /// Generate possible first choices for the entity
    ///
    /// These are suggested first choices based on the Law of One cosmology.
    fn generate_first_choice_options(&self) -> Vec<FirstChoiceOption> {
        vec![
            FirstChoiceOption {
                description: "Choose to help another being".to_string(),
                direction: PolarityDirection::ServiceToOthers,
                polarity_weight: 0.3,
                archetype: 2, // Catalyst - taking action
            },
            FirstChoiceOption {
                description: "Choose to help yourself first".to_string(),
                direction: PolarityDirection::ServiceToSelf,
                polarity_weight: -0.3,
                archetype: 0, // Matrix - self-structure
            },
            FirstChoiceOption {
                description: "Choose to observe and understand".to_string(),
                direction: PolarityDirection::Neutral,
                polarity_weight: 0.0,
                archetype: 3, // Experience - learning
            },
        ]
    }

    /// Check if entity has achieved self-hood (awakened)
    ///
    /// This replaces the simple threshold check in ConsciousnessKernel::has_self_hood()
    pub fn has_self_hood(&self) -> bool {
        self.awakened
    }

    /// Get the awakening progress (0.0 to 1.0)
    ///
    /// Returns the progress toward awakening threshold.
    pub fn awakening_progress(&self) -> Float {
        if self.awakened {
            return 1.0;
        }

        let readiness = self.calculate_readiness();
        (readiness / self.threshold).min(1.0)
    }

    /// Get the current polarity (only set after awakening)
    pub fn get_polarity(&self) -> Option<Polarity> {
        self.polarity
    }

    /// Check if the entity is ready for awakening
    pub fn is_ready(&self) -> bool {
        self.state == AwakeningState::ThresholdReached
    }

    /// Increment choices made counter
    pub fn record_choice(&mut self) {
        self.choices_made += 1;
    }
}

/// Signal that entity is ready for awakening
#[derive(Debug, Clone)]
pub struct AwakeningReady {
    /// Entity that is ready
    pub entity_id: EntityId,

    /// Readiness score at threshold
    pub readiness: Float,

    /// Suggested first choices
    pub suggested_first_choices: Vec<FirstChoiceOption>,
}

/// Option for first choice
#[derive(Debug, Clone)]
pub struct FirstChoiceOption {
    /// Description of the choice
    pub description: String,

    /// Polarity direction
    pub direction: PolarityDirection,

    /// Polarity weight (-1.0 STS to 1.0 STO)
    pub polarity_weight: Float,

    /// Primary archetype activated
    pub archetype: usize,
}

/// Error during awakening
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwakeningError {
    /// Entity has already awakened
    AlreadyAwakened,

    /// Entity is not ready for awakening
    NotReady,

    /// Invalid choice provided
    InvalidChoice,
}

impl std::fmt::Display for AwakeningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AwakeningError::AlreadyAwakened => write!(f, "Entity has already awakened"),
            AwakeningError::NotReady => write!(f, "Entity is not ready for awakening"),
            AwakeningError::InvalidChoice => write!(f, "Invalid choice provided"),
        }
    }
}

impl std::error::Error for AwakeningError {}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awakening_creation() {
        let awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));

        assert!(!awakening.awakened);
        assert_eq!(awakening.state, AwakeningState::Dormant);
        assert_eq!(awakening.polarity, None);
    }

    #[test]
    fn test_awakening_threshold_constant() {
        const { assert!(AWAKENING_THRESHOLD > 0.0); }
        const { assert!(AWAKENING_THRESHOLD < 1.0); }
        assert!((AWAKENING_THRESHOLD - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_check_awakening_dormant() {
        let awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));

        let readiness = awakening.calculate_readiness();
        assert!(readiness < AWAKENING_THRESHOLD);
    }

    #[test]
    fn test_check_awakening_approaching() {
        let mut awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));
        // Set levels to be approaching (70-99% of threshold)
        awakening.update_levels(0.4, 0.4, 0.4);

        let readiness = awakening.calculate_readiness();
        assert!(readiness > AWAKENING_THRESHOLD * 0.7);
        assert!(readiness < AWAKENING_THRESHOLD);
        assert_eq!(awakening.state, AwakeningState::Approaching);
    }

    #[test]
    fn test_check_awakening_threshold_reached() {
        let mut awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));
        // Set levels above threshold
        awakening.update_levels(0.6, 0.6, 0.6);

        let ready = awakening.check_awakening();
        assert!(ready.is_some());
        assert_eq!(awakening.state, AwakeningState::ThresholdReached);

        let ready = ready.unwrap();
        assert!(!ready.suggested_first_choices.is_empty());
    }

    #[test]
    fn test_trigger_awakening_sto() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        // First check should return ready
        let ready = awakening.check_awakening();
        assert!(ready.is_some());

        // Trigger awakening with STO choice
        let first_choice = FirstChoice::sto_choice("Help another being", 0.5);
        let result = awakening.trigger_awakening(first_choice, 100);

        assert!(result.is_ok());
        assert!(awakening.awakened);
        assert_eq!(awakening.state, AwakeningState::Awakened);
        assert_eq!(awakening.polarity, Some(Polarity::STO));
    }

    #[test]
    fn test_trigger_awakening_sts() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        awakening.check_awakening();

        // Trigger awakening with STS choice
        let first_choice = FirstChoice::sts_choice("Help myself first", 0.5);
        let result = awakening.trigger_awakening(first_choice, 100);

        assert!(result.is_ok());
        assert_eq!(awakening.polarity, Some(Polarity::STS));
    }

    #[test]
    fn test_trigger_awakening_already_awakened() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        awakening.check_awakening();
        let _ = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);

        // Try to awaken again
        let result = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 200);
        assert!(matches!(result, Err(AwakeningError::AlreadyAwakened)));
    }

    #[test]
    fn test_trigger_awakening_not_ready() {
        let mut awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));
        // Levels are low (default)

        // Try to awaken without being ready
        let result = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);
        assert!(matches!(result, Err(AwakeningError::NotReady)));
    }

    #[test]
    fn test_has_self_hood() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        // Before awakening
        assert!(!awakening.has_self_hood());

        // Trigger awakening
        awakening.check_awakening();
        let _ = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);

        // After awakening
        assert!(awakening.has_self_hood());
    }

    #[test]
    fn test_awakening_progress() {
        let mut awakening = ConsciousnessAwakening::new(EntityId::new("test".to_string()));

        // Initial progress should be low
        let initial_progress = awakening.awakening_progress();
        assert!(initial_progress < 0.5);

        // Update to approaching
        awakening.update_levels(0.4, 0.4, 0.4);
        let approaching_progress = awakening.awakening_progress();
        assert!(approaching_progress > initial_progress);
        assert!(approaching_progress < 1.0);

        // Awaken
        awakening.update_levels(0.6, 0.6, 0.6);
        awakening.check_awakening();
        let _ = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);
        assert_eq!(awakening.awakening_progress(), 1.0);
    }

    #[test]
    fn test_awakening_event_properties() {
        let mut awakening = ConsciousnessAwakening::with_levels(
            EntityId::new("test-entity".to_string()),
            0.65,
            0.55,
            0.60,
        );

        awakening.check_awakening();
        let first_choice = FirstChoice::sto_choice("Help another being", 0.7);
        let result = awakening.trigger_awakening(first_choice, 42);

        let event = result.unwrap();
        assert_eq!(event.entity_id.uuid, "test-entity");
        assert_eq!(event.timestamp, 42);
        assert_eq!(event.polarity_hint, PolarityDirection::ServiceToOthers);
        assert!(matches!(event.pre_density, Density::Second(_)));
        assert_eq!(event.post_density, Density::Third);
        assert!((event.transformative_intensity - 0.55).abs() < 0.001);
    }

    #[test]
    fn test_first_choice_creation() {
        let sto = FirstChoice::sto_choice("Help others", 0.5);
        assert_eq!(sto.direction, PolarityDirection::ServiceToOthers);
        assert!((sto.polarity_weight - 0.5).abs() < 0.001);
        assert_eq!(sto.primary_archetype, 2);

        let sts = FirstChoice::sts_choice("Help self", 0.6);
        assert_eq!(sts.direction, PolarityDirection::ServiceToSelf);
        assert!((sts.polarity_weight - (-0.6)).abs() < 0.001);
        assert_eq!(sts.primary_archetype, 0);

        let neutral = FirstChoice::neutral_choice("Observe");
        assert_eq!(neutral.direction, PolarityDirection::Neutral);
        assert!((neutral.polarity_weight - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_readiness() {
        let awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.5, 0.6, 0.7);

        let readiness = awakening.calculate_readiness();
        let expected = (0.5 + 0.6 + 0.7) / 3.0;
        assert!((readiness - expected).abs() < 0.001);
    }

    #[test]
    fn test_polarity_set_on_awakening() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        awakening.check_awakening();
        let _ = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);

        assert!(awakening.polarity.is_some());
        assert_eq!(awakening.get_polarity(), Some(Polarity::STO));
    }

    #[test]
    fn test_awakening_event_stored() {
        let mut awakening =
            ConsciousnessAwakening::with_levels(EntityId::new("test".to_string()), 0.6, 0.6, 0.6);

        awakening.check_awakening();
        let result = awakening.trigger_awakening(FirstChoice::sto_choice("Test", 0.5), 100);

        assert!(awakening.awakening_event.is_some());
        let stored_event = awakening.awakening_event.as_ref().unwrap();
        assert_eq!(stored_event.entity_id, result.as_ref().unwrap().entity_id);
    }
}
