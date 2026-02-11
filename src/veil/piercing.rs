// Veil Piercing - Events and accumulation of veil thinning
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "Veil thins as entities polarize and ascend"
// "The Entity's journey is about making the Mind transparent enough
// to let the Spirit shine through"

use crate::evolution_density_octave::density_octave::Density;
use crate::types::Float;
use crate::veil::density_variation::PolarizationState;

/// Type of veil-piercing event
///
/// Knowledge Base Reference: Healing.json
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PiercingEventType {
    /// Meditation - quieting the mind
    Meditation,

    /// Spiritual work - conscious development
    SpiritualWork,

    /// Catalyst processing - working with challenges
    CatalystProcessing,

    /// Love-based action - acting from the heart
    LoveBasedAction,
}

/// Location where piercing occurs (in the mind/body/spirit complex)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PiercingLocation {
    /// Mind complex (archetypes 1-7)
    Mind,

    /// Body complex (archetypes 8-14)
    Body,

    /// Spirit complex (archetypes 15-21)
    Spirit,

    /// Across all complexes (green-ray bridge)
    All,
}

/// Veil-piercing event
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
#[derive(Debug, Clone, PartialEq)]
pub struct PiercingEvent {
    /// Type of piercing event
    pub event_type: PiercingEventType,

    /// Location in the mind/body/spirit complex
    pub location: PiercingLocation,

    /// Duration of the event (for meditation)
    pub duration: Float,

    /// Intensity of the event (for spiritual work)
    pub intensity: Float,

    /// Depth of processing (for catalyst processing)
    pub depth: Float,

    /// Purity of love (for love-based action)
    pub purity: Float,

    /// Density at which piercing occurred
    pub density: Density,

    /// Polarity state at time of piercing
    pub polarization: PolarizationState,

    /// Timestamp of event
    pub timestamp: Float,
}

impl PiercingEvent {
    /// Create new piercing event
    pub fn new(
        event_type: PiercingEventType,
        location: PiercingLocation,
        density: Density,
        polarization: PolarizationState,
    ) -> Self {
        PiercingEvent {
            event_type,
            location,
            duration: 0.0,
            intensity: 0.0,
            depth: 0.0,
            purity: 0.0,
            density,
            polarization,
            timestamp: 0.0,
        }
    }

    /// Create meditation piercing event
    pub fn meditation(
        duration: Float,
        location: PiercingLocation,
        density: Density,
        polarization: PolarizationState,
    ) -> Self {
        PiercingEvent {
            event_type: PiercingEventType::Meditation,
            location,
            duration: duration.max(0.0),
            intensity: 0.0,
            depth: 0.0,
            purity: 0.0,
            density,
            polarization,
            timestamp: 0.0,
        }
    }

    /// Create spiritual work piercing event
    pub fn spiritual_work(
        intensity: Float,
        location: PiercingLocation,
        density: Density,
        polarization: PolarizationState,
    ) -> Self {
        PiercingEvent {
            event_type: PiercingEventType::SpiritualWork,
            location,
            duration: 0.0,
            intensity: intensity.max(0.0).min(1.0),
            depth: 0.0,
            purity: 0.0,
            density,
            polarization,
            timestamp: 0.0,
        }
    }

    /// Create catalyst processing piercing event
    pub fn catalyst_processing(
        depth: Float,
        location: PiercingLocation,
        density: Density,
        polarization: PolarizationState,
    ) -> Self {
        PiercingEvent {
            event_type: PiercingEventType::CatalystProcessing,
            location,
            duration: 0.0,
            intensity: 0.0,
            depth: depth.max(0.0).min(1.0),
            purity: 0.0,
            density,
            polarization,
            timestamp: 0.0,
        }
    }

    /// Create love-based action piercing event
    pub fn love_based_action(
        purity: Float,
        location: PiercingLocation,
        density: Density,
        polarization: PolarizationState,
    ) -> Self {
        PiercingEvent {
            event_type: PiercingEventType::LoveBasedAction,
            location,
            duration: 0.0,
            intensity: 0.0,
            depth: 0.0,
            purity: purity.max(0.0).min(1.0),
            density,
            polarization,
            timestamp: 0.0,
        }
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: Float) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Calculate piercing strength for this event
    ///
    /// Knowledge Base Reference: Healing.json
    pub fn calculate_strength(&self) -> Float {
        match self.event_type {
            PiercingEventType::Meditation => 0.01 * self.duration,
            PiercingEventType::SpiritualWork => 0.02 * self.intensity,
            PiercingEventType::CatalystProcessing => 0.015 * self.depth,
            PiercingEventType::LoveBasedAction => 0.025 * self.purity,
        }
    }
}

/// Result of a veil-piercing event
#[derive(Debug, Clone, PartialEq)]
pub struct PiercingResult {
    /// New veil thickness after piercing
    pub new_thickness: Float,

    /// Whether a thin spot was created
    pub thin_spot_created: bool,

    /// Strength of the piercing
    pub piercing_strength: Float,
}

impl PiercingResult {
    /// Create new piercing result
    pub fn new(new_thickness: Float, thin_spot_created: bool, piercing_strength: Float) -> Self {
        PiercingResult {
            new_thickness: new_thickness.max(0.0).min(1.0),
            thin_spot_created,
            piercing_strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::veil::THIN_SPOT_THRESHOLD;

    // ===== PiercingEventType Tests =====

    #[test]
    fn test_piercing_event_type_values() {
        assert_eq!(PiercingEventType::Meditation, PiercingEventType::Meditation);
        assert_eq!(
            PiercingEventType::SpiritualWork,
            PiercingEventType::SpiritualWork
        );
        assert_eq!(
            PiercingEventType::CatalystProcessing,
            PiercingEventType::CatalystProcessing
        );
        assert_eq!(
            PiercingEventType::LoveBasedAction,
            PiercingEventType::LoveBasedAction
        );
    }

    // ===== PiercingLocation Tests =====

    #[test]
    fn test_piercing_location_values() {
        assert_eq!(PiercingLocation::Mind, PiercingLocation::Mind);
        assert_eq!(PiercingLocation::Body, PiercingLocation::Body);
        assert_eq!(PiercingLocation::Spirit, PiercingLocation::Spirit);
        assert_eq!(PiercingLocation::All, PiercingLocation::All);
    }

    // ===== PiercingEvent Tests =====

    #[test]
    fn test_piercing_event_new() {
        let event = PiercingEvent::new(
            PiercingEventType::Meditation,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        assert_eq!(event.event_type, PiercingEventType::Meditation);
        assert_eq!(event.location, PiercingLocation::Mind);
        assert_eq!(event.density, Density::Third);
        assert_eq!(event.duration, 0.0);
        assert_eq!(event.intensity, 0.0);
        assert_eq!(event.depth, 0.0);
        assert_eq!(event.purity, 0.0);
    }

    #[test]
    fn test_piercing_event_meditation() {
        let event = PiercingEvent::meditation(
            30.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        assert_eq!(event.event_type, PiercingEventType::Meditation);
        assert_eq!(event.duration, 30.0);
        assert_eq!(event.location, PiercingLocation::Mind);
    }

    #[test]
    fn test_piercing_event_meditation_duration_clamped() {
        let event = PiercingEvent::meditation(
            -10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        assert_eq!(event.duration, 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_piercing_event_spiritual_work() {
        let event = PiercingEvent::spiritual_work(
            0.7,
            PiercingLocation::Spirit,
            Density::Fourth,
            PolarizationState::sts(0.6),
        );

        assert_eq!(event.event_type, PiercingEventType::SpiritualWork);
        assert_eq!(event.intensity, 0.7);
        assert_eq!(event.location, PiercingLocation::Spirit);
    }

    #[test]
    fn test_piercing_event_spiritual_work_intensity_clamped() {
        let event = PiercingEvent::spiritual_work(
            1.5,
            PiercingLocation::Spirit,
            Density::Fourth,
            PolarizationState::sts(0.6),
        );

        assert_eq!(event.intensity, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_piercing_event_catalyst_processing() {
        let event = PiercingEvent::catalyst_processing(
            0.8,
            PiercingLocation::Body,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        assert_eq!(event.event_type, PiercingEventType::CatalystProcessing);
        assert_eq!(event.depth, 0.8);
        assert_eq!(event.location, PiercingLocation::Body);
    }

    #[test]
    fn test_piercing_event_catalyst_processing_depth_clamped() {
        let event = PiercingEvent::catalyst_processing(
            1.5,
            PiercingLocation::Body,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        assert_eq!(event.depth, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_piercing_event_love_based_action() {
        let event = PiercingEvent::love_based_action(
            0.9,
            PiercingLocation::All,
            Density::Fourth,
            PolarizationState::sto(0.8),
        );

        assert_eq!(event.event_type, PiercingEventType::LoveBasedAction);
        assert_eq!(event.purity, 0.9);
        assert_eq!(event.location, PiercingLocation::All);
    }

    #[test]
    fn test_piercing_event_love_based_action_purity_clamped() {
        let event = PiercingEvent::love_based_action(
            1.5,
            PiercingLocation::All,
            Density::Fourth,
            PolarizationState::sto(0.8),
        );

        assert_eq!(event.purity, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_piercing_event_with_timestamp() {
        let event = PiercingEvent::meditation(
            30.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        )
        .with_timestamp(100.0);

        assert_eq!(event.timestamp, 100.0);
    }

    // ===== Piercing Strength Tests =====

    #[test]
    fn test_piercing_strength_meditation() {
        let event = PiercingEvent::meditation(
            30.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let strength = event.calculate_strength();
        assert_eq!(strength, 0.01 * 30.0); // 0.3
    }

    #[test]
    fn test_piercing_strength_spiritual_work() {
        let event = PiercingEvent::spiritual_work(
            0.7,
            PiercingLocation::Spirit,
            Density::Fourth,
            PolarizationState::sts(0.6),
        );

        let strength = event.calculate_strength();
        assert_eq!(strength, 0.02 * 0.7); // 0.014
    }

    #[test]
    fn test_piercing_strength_catalyst_processing() {
        let event = PiercingEvent::catalyst_processing(
            0.8,
            PiercingLocation::Body,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let strength = event.calculate_strength();
        assert_eq!(strength, 0.015 * 0.8); // 0.012
    }

    #[test]
    fn test_piercing_strength_love_based_action() {
        let event = PiercingEvent::love_based_action(
            0.9,
            PiercingLocation::All,
            Density::Fourth,
            PolarizationState::sto(0.8),
        );

        let strength = event.calculate_strength();
        assert_eq!(strength, 0.025 * 0.9); // 0.0225
    }

    #[test]
    fn test_piercing_strength_comparison() {
        let meditation = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );
        let spiritual = PiercingEvent::spiritual_work(
            0.5,
            PiercingLocation::Spirit,
            Density::Fourth,
            PolarizationState::sto(0.5),
        );
        let catalyst = PiercingEvent::catalyst_processing(
            0.5,
            PiercingLocation::Body,
            Density::Third,
            PolarizationState::sto(0.5),
        );
        let love = PiercingEvent::love_based_action(
            0.5,
            PiercingLocation::All,
            Density::Fourth,
            PolarizationState::sto(0.5),
        );

        let med_strength = meditation.calculate_strength(); // 0.1
        let spir_strength = spiritual.calculate_strength(); // 0.01
        let cat_strength = catalyst.calculate_strength(); // 0.0075
        let love_strength = love.calculate_strength(); // 0.0125

        // Love-based action has highest coefficient (0.025)
        assert!(love_strength > spir_strength);
        assert!(love_strength > cat_strength);

        // Meditation can be strong with long duration
        assert!(med_strength > spir_strength);
    }

    // ===== PiercingResult Tests =====

    #[test]
    fn test_piercing_result_new() {
        let result = PiercingResult::new(0.7, true, 0.1);

        assert_eq!(result.new_thickness, 0.7);
        assert_eq!(result.thin_spot_created, true);
        assert_eq!(result.piercing_strength, 0.1);
    }

    #[test]
    fn test_piercing_result_thickness_clamped() {
        let result_low = PiercingResult::new(-0.5, false, 0.1);
        assert_eq!(result_low.new_thickness, 0.0);

        let result_high = PiercingResult::new(1.5, false, 0.1);
        assert_eq!(result_high.new_thickness, 1.0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_meditation_creates_thin_spot() {
        // Long meditation should create thin spot
        let event = PiercingEvent::meditation(
            10.0, // Strength: 0.1
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let strength = event.calculate_strength();
        assert!(strength > THIN_SPOT_THRESHOLD);
    }

    #[test]
    fn test_short_meditation_no_thin_spot() {
        // Short meditation should not create thin spot
        let event = PiercingEvent::meditation(
            2.0, // Strength: 0.02
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let strength = event.calculate_strength();
        assert!(strength < THIN_SPOT_THRESHOLD);
    }

    #[test]
    fn test_love_based_action_strongest() {
        let love = PiercingEvent::love_based_action(
            0.8,
            PiercingLocation::All,
            Density::Fourth,
            PolarizationState::sto(0.8),
        );
        let spiritual = PiercingEvent::spiritual_work(
            0.8,
            PiercingLocation::Spirit,
            Density::Fourth,
            PolarizationState::sto(0.8),
        );

        let love_strength = love.calculate_strength();
        let spir_strength = spiritual.calculate_strength();

        assert!(love_strength > spir_strength);
    }

    #[test]
    fn test_piercing_event_clone() {
        let event = PiercingEvent::meditation(
            30.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        )
        .with_timestamp(100.0);

        let cloned = event.clone();
        assert_eq!(event, cloned);
    }
}
