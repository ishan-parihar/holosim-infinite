// Realm Translation - Translation between Space/Time and Time/Space
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "The Veil separates these realms, creating the illusion that they are separate"

use crate::coordinates::time_space::MetaphysicalTimeSpaceCoord;
use crate::types::Float;

/// Threshold for translation - veil must be thin enough
pub const TRANSLATION_THRESHOLD: f64 = 0.4;

/// Realm Translation - handles translation between realms
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
#[derive(Debug, Clone, PartialEq)]
pub struct RealmTranslation {
    /// Speed of light for causality calculations
    speed_of_light: Float,

    /// Translation threshold (veil thickness below which translation is possible)
    translation_threshold: Float,
}

impl RealmTranslation {
    /// Create new realm translation
    pub fn new() -> Self {
        RealmTranslation {
            speed_of_light: 299792458.0, // c in m/s
            translation_threshold: TRANSLATION_THRESHOLD,
        }
    }

    /// Create realm translation with custom settings
    pub fn with_settings(speed_of_light: Float, translation_threshold: Float) -> Self {
        RealmTranslation {
            speed_of_light,
            translation_threshold: translation_threshold.clamp(0.0, 1.0),
        }
    }

    /// Translate from Space/Time to Time/Space
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `space_time_coord` - The Space/Time coordinate to translate
    /// * `veil_thickness` - The current veil thickness (0.0 = transparent, 1.0 = opaque)
    ///
    /// # Returns
    /// Some(MetaphysicalTimeSpaceCoord) if translation succeeds, None if veil is too thick
    pub fn translate_to_time_space(
        &self,
        space_time_coord: &crate::coordinates::space_time::PhysicalSpaceTimeCoord,
        veil_thickness: Float,
    ) -> Option<MetaphysicalTimeSpaceCoord> {
        // Check if veil is thin enough
        if veil_thickness > self.translation_threshold {
            return None;
        }

        // Create Time/Space reflection from Space/Time
        let time_space = MetaphysicalTimeSpaceCoord::from_space_time(space_time_coord);

        Some(time_space)
    }

    /// Translate from Time/Space to Space/Time
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `time_space_coord` - The Time/Space coordinate to translate
    ///
    /// # Returns
    /// PhysicalSpaceTimeCoord at origin with Time/Space reflection
    pub fn translate_to_space_time(
        &self,
        time_space_coord: &MetaphysicalTimeSpaceCoord,
    ) -> crate::coordinates::space_time::PhysicalSpaceTimeCoord {
        // Create Space/Space coordinate with Time/Space reflection
        crate::coordinates::space_time::PhysicalSpaceTimeCoord::with_reflection(
            0.0,
            0.0,
            0.0,
            0.0,
            0,
            time_space_coord.clone(),
        )
    }

    /// Check if translation is possible
    ///
    /// # Arguments
    /// * `veil_thickness` - The current veil thickness
    ///
    /// # Returns
    /// True if translation is possible
    pub fn can_translate(&self, veil_thickness: Float) -> bool {
        veil_thickness <= self.translation_threshold
    }

    /// Get translation threshold
    pub fn get_translation_threshold(&self) -> Float {
        self.translation_threshold
    }

    /// Set translation threshold
    pub fn set_translation_threshold(&mut self, threshold: Float) {
        self.translation_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Get speed of light
    pub fn get_speed_of_light(&self) -> Float {
        self.speed_of_light
    }

    /// Set speed of light
    pub fn set_speed_of_light(&mut self, c: Float) {
        self.speed_of_light = c;
    }
}

impl Default for RealmTranslation {
    fn default() -> Self {
        Self::new()
    }
}

// Extension trait for PhysicalSpaceTimeCoord to add translation methods
pub trait SpaceTimeTranslation {
    /// Translate to Time/Space coordinates (requires thin veil)
    fn translate_to_time_space(&self, veil_thickness: Float) -> Option<MetaphysicalTimeSpaceCoord>;
}

impl SpaceTimeTranslation for crate::coordinates::space_time::PhysicalSpaceTimeCoord {
    /// Translate to Time/Space coordinates (requires thin veil)
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `veil_thickness` - The current veil thickness (0.0 = transparent, 1.0 = opaque)
    ///
    /// # Returns
    /// Some(MetaphysicalTimeSpaceCoord) if translation succeeds, None if veil is too thick
    fn translate_to_time_space(&self, veil_thickness: Float) -> Option<MetaphysicalTimeSpaceCoord> {
        let translator = RealmTranslation::new();
        translator.translate_to_time_space(self, veil_thickness)
    }
}

// Extension trait for MetaphysicalTimeSpaceCoord to add translation methods
pub trait TimeSpaceTranslation {
    /// Translate back to Space/Time coordinates
    fn translate_to_space_time(&self) -> crate::coordinates::space_time::PhysicalSpaceTimeCoord;
}

impl TimeSpaceTranslation for MetaphysicalTimeSpaceCoord {
    /// Translate back to Space/Time coordinates
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Returns
    /// PhysicalSpaceTimeCoord with this Time/Space as reflection
    fn translate_to_space_time(&self) -> crate::coordinates::space_time::PhysicalSpaceTimeCoord {
        let translator = RealmTranslation::new();
        translator.translate_to_space_time(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== RealmTranslation Tests =====

    #[test]
    fn test_realm_translation_new() {
        let translator = RealmTranslation::new();
        assert_eq!(translator.speed_of_light, 299792458.0);
        assert_eq!(translator.translation_threshold, TRANSLATION_THRESHOLD);
    }

    #[test]
    fn test_realm_translation_with_settings() {
        let translator = RealmTranslation::with_settings(3.0e8, 0.3);
        assert_eq!(translator.speed_of_light, 3.0e8);
        assert_eq!(translator.translation_threshold, 0.3);
    }

    #[test]
    fn test_realm_translation_threshold_clamped() {
        let translator = RealmTranslation::with_settings(3.0e8, 1.5);
        assert_eq!(translator.translation_threshold, 1.0); // Clamped to 1.0

        let translator = RealmTranslation::with_settings(3.0e8, -0.5);
        assert_eq!(translator.translation_threshold, 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_translate_to_time_space_success() {
        let translator = RealmTranslation::new();
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Thin veil (below threshold)
        let time_space = translator.translate_to_time_space(&space_time, 0.3);
        assert!(time_space.is_some());
    }

    #[test]
    fn test_translate_to_time_space_failure() {
        let translator = RealmTranslation::new();
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Thick veil (above threshold)
        let time_space = translator.translate_to_time_space(&space_time, 0.6);
        assert!(time_space.is_none());
    }

    #[test]
    fn test_translate_to_time_space_exactly_at_threshold() {
        let translator = RealmTranslation::new();
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // At threshold (should succeed)
        let time_space = translator.translate_to_time_space(&space_time, TRANSLATION_THRESHOLD);
        assert!(time_space.is_some());
    }

    #[test]
    fn test_translate_to_space_time() {
        let translator = RealmTranslation::new();
        let time_space = MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::time_space::ExperienceVector::with_values(0.5, 0.6, 0.7),
            crate::coordinates::time_space::IncarnationPlan::new(),
            crate::coordinates::time_space::BroaderConsciousnessAccess::new(),
        );

        let space_time = translator.translate_to_space_time(&time_space);
        assert_eq!(space_time.x, 0.0);
        assert_eq!(space_time.y, 0.0);
        assert_eq!(space_time.z, 0.0);
        assert_eq!(space_time.t, 0.0);
        assert!(space_time.time_space_reflection.is_some());
    }

    #[test]
    fn test_can_translate_true() {
        let translator = RealmTranslation::new();
        assert!(translator.can_translate(0.3));
        assert!(translator.can_translate(0.4)); // At threshold
    }

    #[test]
    fn test_can_translate_false() {
        let translator = RealmTranslation::new();
        assert!(!translator.can_translate(0.5));
        assert!(!translator.can_translate(1.0));
    }

    #[test]
    fn test_get_set_translation_threshold() {
        let mut translator = RealmTranslation::new();
        translator.set_translation_threshold(0.3);
        assert_eq!(translator.get_translation_threshold(), 0.3);
    }

    #[test]
    fn test_set_translation_threshold_clamped() {
        let mut translator = RealmTranslation::new();
        translator.set_translation_threshold(1.5);
        assert_eq!(translator.get_translation_threshold(), 1.0); // Clamped to 1.0

        translator.set_translation_threshold(-0.5);
        assert_eq!(translator.get_translation_threshold(), 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_get_set_speed_of_light() {
        let mut translator = RealmTranslation::new();
        translator.set_speed_of_light(3.0e8);
        assert_eq!(translator.get_speed_of_light(), 3.0e8);
    }

    #[test]
    fn test_realm_translation_default() {
        let translator = RealmTranslation::default();
        assert_eq!(translator.speed_of_light, 299792458.0);
    }

    // ===== SpaceTimeTranslation Trait Tests =====

    #[test]
    fn test_space_time_translation_trait_success() {
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Thin veil
        let time_space = space_time.translate_to_time_space(0.3);
        assert!(time_space.is_some());
    }

    #[test]
    fn test_space_time_translation_trait_failure() {
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Thick veil
        let time_space = space_time.translate_to_time_space(0.6);
        assert!(time_space.is_none());
    }

    // ===== TimeSpaceTranslation Trait Tests =====

    #[test]
    fn test_time_space_translation_trait() {
        let time_space = MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::time_space::ExperienceVector::with_values(0.5, 0.6, 0.7),
            crate::coordinates::time_space::IncarnationPlan::new(),
            crate::coordinates::time_space::BroaderConsciousnessAccess::new(),
        );

        let space_time = time_space.translate_to_space_time();
        assert_eq!(space_time.x, 0.0);
        assert!(space_time.time_space_reflection.is_some());
    }

    // ===== Integration Tests =====

    #[test]
    fn test_round_trip_translation() {
        let translator = RealmTranslation::new();
        let original_space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Space/Time -> Time/Space
        let time_space = translator
            .translate_to_time_space(&original_space_time, 0.3)
            .unwrap();

        // Time/Space -> Space/Time
        let back_to_space_time = translator.translate_to_space_time(&time_space);

        assert!(back_to_space_time.time_space_reflection.is_some());
    }

    #[test]
    fn test_custom_translation_threshold() {
        let translator = RealmTranslation::with_settings(3.0e8, 0.2);
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);

        // Above custom threshold (0.25 > 0.2)
        let time_space = translator.translate_to_time_space(&space_time, 0.25);
        assert!(time_space.is_none());

        // Below custom threshold (0.15 < 0.2)
        let time_space = translator.translate_to_time_space(&space_time, 0.15);
        assert!(time_space.is_some());
    }

    #[test]
    fn test_realm_translation_clone() {
        let translator1 = RealmTranslation::with_settings(3.0e8, 0.3);
        let translator2 = translator1.clone();
        assert_eq!(translator1, translator2);
    }

    #[test]
    fn test_realm_translation_partial_eq() {
        let translator1 = RealmTranslation::with_settings(3.0e8, 0.3);
        let translator2 = RealmTranslation::with_settings(3.0e8, 0.3);
        let translator3 = RealmTranslation::with_settings(3.0e8, 0.4);

        assert_eq!(translator1, translator2);
        assert_ne!(translator1, translator3);
    }
}
