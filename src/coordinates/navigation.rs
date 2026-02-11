// Dual Realm Navigation - Phase 3: Green-Ray Realm
// Implements navigation between Space/Time and Time/Space realms
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "The Veil separates these realms, creating the illusion that they are separate"

use crate::coordinates::space_time::PhysicalSpaceTimeCoord;
use crate::coordinates::time_space::MetaphysicalTimeSpaceCoord;
use crate::coordinates::translation::RealmTranslation;
use crate::evolution_density_octave::density_octave::Density;
use crate::veil::density_variation::PolarizationState;
use crate::veil::VeilMechanism;
use std::collections::HashMap;

/// Entity identifier type
pub type EntityId = u64;

/// Navigation result
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationResult {
    /// Navigation successful
    Success,

    /// Veil too thick for translation
    VeilTooThick,

    /// Translation failed
    TranslationFailed,

    /// Entity not found
    EntityNotFound,
}

/// Space/Time Realm
#[derive(Debug, Clone, PartialEq)]
pub struct SpaceTimeRealm {
    /// Entity positions in Space/Time
    pub positions: HashMap<EntityId, PhysicalSpaceTimeCoord>,
}

impl SpaceTimeRealm {
    /// Create new Space/Time realm
    pub fn new() -> Self {
        SpaceTimeRealm {
            positions: HashMap::new(),
        }
    }

    /// Get entity position
    pub fn get_position(&self, entity_id: EntityId) -> Option<&PhysicalSpaceTimeCoord> {
        self.positions.get(&entity_id)
    }

    /// Place entity at position
    pub fn place_entity(&mut self, entity_id: EntityId, position: PhysicalSpaceTimeCoord) {
        self.positions.insert(entity_id, position);
    }

    /// Remove entity
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<PhysicalSpaceTimeCoord> {
        self.positions.remove(&entity_id)
    }

    /// Check if entity is in realm
    pub fn contains_entity(&self, entity_id: EntityId) -> bool {
        self.positions.contains_key(&entity_id)
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.positions.len()
    }
}

impl Default for SpaceTimeRealm {
    fn default() -> Self {
        Self::new()
    }
}

/// Time/Space Realm
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSpaceRealm {
    /// Entity positions in Time/Space
    pub positions: HashMap<EntityId, MetaphysicalTimeSpaceCoord>,
}

impl TimeSpaceRealm {
    /// Create new Time/Space realm
    pub fn new() -> Self {
        TimeSpaceRealm {
            positions: HashMap::new(),
        }
    }

    /// Get entity position
    pub fn get_position(&self, entity_id: EntityId) -> Option<&MetaphysicalTimeSpaceCoord> {
        self.positions.get(&entity_id)
    }

    /// Place entity at position
    pub fn place_entity(&mut self, entity_id: EntityId, position: MetaphysicalTimeSpaceCoord) {
        self.positions.insert(entity_id, position);
    }

    /// Remove entity
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<MetaphysicalTimeSpaceCoord> {
        self.positions.remove(&entity_id)
    }

    /// Check if entity is in realm
    pub fn contains_entity(&self, entity_id: EntityId) -> bool {
        self.positions.contains_key(&entity_id)
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.positions.len()
    }
}

impl Default for TimeSpaceRealm {
    fn default() -> Self {
        Self::new()
    }
}

/// Dual Realm Navigation System
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
#[derive(Debug, Clone, PartialEq)]
pub struct DualRealmNavigator {
    /// Space/Time realm
    pub space_time: SpaceTimeRealm,

    /// Time/Space realm
    pub time_space: TimeSpaceRealm,

    /// Translation mechanism
    pub translation: RealmTranslation,

    /// Veil mechanism
    pub veil_mechanism: VeilMechanism,
}

impl DualRealmNavigator {
    /// Create new dual realm navigator
    pub fn new() -> Self {
        DualRealmNavigator {
            space_time: SpaceTimeRealm::new(),
            time_space: TimeSpaceRealm::new(),
            translation: RealmTranslation::new(),
            veil_mechanism: VeilMechanism::new(),
        }
    }

    /// Create with custom settings
    pub fn with_settings(translation: RealmTranslation, veil_mechanism: VeilMechanism) -> Self {
        DualRealmNavigator {
            space_time: SpaceTimeRealm::new(),
            time_space: TimeSpaceRealm::new(),
            translation,
            veil_mechanism,
        }
    }

    /// Navigate from Space/Time to Time/Space
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The entity's density
    /// * `polarization` - The entity's polarization state
    ///
    /// # Returns
    /// Navigation result
    pub fn navigate_to_time_space(
        &mut self,
        entity_id: EntityId,
        density: Density,
        polarization: &PolarizationState,
    ) -> NavigationResult {
        // Calculate veil thickness
        let veil_thickness =
            self.veil_mechanism
                .calculate_veil_thickness(entity_id, &density, polarization);

        // Check if entity can translate
        if !self.translation.can_translate(veil_thickness) {
            return NavigationResult::VeilTooThick;
        }

        // Get entity's current position
        let space_time_pos = match self.space_time.get_position(entity_id) {
            Some(pos) => pos.clone(),
            None => return NavigationResult::EntityNotFound,
        };

        // Translate to Time/Space
        let time_space_pos = match self
            .translation
            .translate_to_time_space(&space_time_pos, veil_thickness)
        {
            Some(pos) => pos,
            None => return NavigationResult::TranslationFailed,
        };

        // Move entity to Time/Space
        self.time_space.place_entity(entity_id, time_space_pos);

        NavigationResult::Success
    }

    /// Navigate from Time/Space to Space/Time
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Navigation result
    pub fn navigate_to_space_time(&mut self, entity_id: EntityId) -> NavigationResult {
        // Get entity's current position in Time/Space
        let time_space_pos = match self.time_space.get_position(entity_id) {
            Some(pos) => pos.clone(),
            None => return NavigationResult::EntityNotFound,
        };

        // Translate back to Space/Space
        let space_time_pos = self.translation.translate_to_space_time(&time_space_pos);

        // Move entity to Space/Time
        self.space_time.place_entity(entity_id, space_time_pos);

        NavigationResult::Success
    }

    /// Add entity to Space/Time realm
    pub fn add_entity_to_space_time(
        &mut self,
        entity_id: EntityId,
        position: PhysicalSpaceTimeCoord,
    ) {
        self.space_time.place_entity(entity_id, position);
    }

    /// Add entity to Time/Space realm
    pub fn add_entity_to_time_space(
        &mut self,
        entity_id: EntityId,
        position: MetaphysicalTimeSpaceCoord,
    ) {
        self.time_space.place_entity(entity_id, position);
    }

    /// Remove entity from both realms
    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.space_time.remove_entity(entity_id);
        self.time_space.remove_entity(entity_id);
    }

    /// Get entity's current realm
    pub fn get_entity_realm(&self, entity_id: EntityId) -> Option<EntityRealm> {
        if self.space_time.contains_entity(entity_id) {
            Some(EntityRealm::SpaceTime)
        } else if self.time_space.contains_entity(entity_id) {
            Some(EntityRealm::TimeSpace)
        } else {
            None
        }
    }

    /// Check if entity can navigate to Time/Space
    pub fn can_navigate_to_time_space(
        &self,
        entity_id: EntityId,
        density: Density,
        polarization: &PolarizationState,
    ) -> bool {
        let veil_thickness =
            self.veil_mechanism
                .calculate_veil_thickness(entity_id, &density, polarization);
        self.translation.can_translate(veil_thickness)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> NavigationStatistics {
        NavigationStatistics {
            space_time_entity_count: self.space_time.entity_count(),
            time_space_entity_count: self.time_space.entity_count(),
            total_entities: self.space_time.entity_count() + self.time_space.entity_count(),
        }
    }
}

impl Default for DualRealmNavigator {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity realm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityRealm {
    /// Entity is in Space/Time
    SpaceTime,

    /// Entity is in Time/Space
    TimeSpace,
}

/// Navigation statistics
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationStatistics {
    /// Number of entities in Space/Time
    pub space_time_entity_count: usize,

    /// Number of entities in Time/Space
    pub time_space_entity_count: usize,

    /// Total entities
    pub total_entities: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Polarity;

    // ===== SpaceTimeRealm Tests =====

    #[test]
    fn test_space_time_realm_new() {
        let realm = SpaceTimeRealm::new();
        assert_eq!(realm.entity_count(), 0);
    }

    #[test]
    fn test_space_time_realm_place_entity() {
        let mut realm = SpaceTimeRealm::new();
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        realm.place_entity(1, pos);
        assert_eq!(realm.entity_count(), 1);
        assert!(realm.contains_entity(1));
    }

    #[test]
    fn test_space_time_realm_get_position() {
        let mut realm = SpaceTimeRealm::new();
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        realm.place_entity(1, pos.clone());
        let retrieved = realm.get_position(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().x, 1.0);
    }

    #[test]
    fn test_space_time_realm_remove_entity() {
        let mut realm = SpaceTimeRealm::new();
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        realm.place_entity(1, pos);
        realm.remove_entity(1);
        assert_eq!(realm.entity_count(), 0);
        assert!(!realm.contains_entity(1));
    }

    #[test]
    fn test_space_time_realm_default() {
        let realm = SpaceTimeRealm::default();
        assert_eq!(realm.entity_count(), 0);
    }

    // ===== TimeSpaceRealm Tests =====

    #[test]
    fn test_time_space_realm_new() {
        let realm = TimeSpaceRealm::new();
        assert_eq!(realm.entity_count(), 0);
    }

    #[test]
    fn test_time_space_realm_place_entity() {
        let mut realm = TimeSpaceRealm::new();
        let pos = MetaphysicalTimeSpaceCoord::new(
            crate::coordinates::ExperienceVector::new(),
            crate::coordinates::IncarnationPlan::new(),
            crate::coordinates::BroaderConsciousnessAccess::new(),
        );
        realm.place_entity(1, pos);
        assert_eq!(realm.entity_count(), 1);
        assert!(realm.contains_entity(1));
    }

    #[test]
    fn test_time_space_realm_default() {
        let realm = TimeSpaceRealm::default();
        assert_eq!(realm.entity_count(), 0);
    }

    // ===== DualRealmNavigator Tests =====

    #[test]
    fn test_dual_realm_navigator_new() {
        let navigator = DualRealmNavigator::new();
        assert_eq!(navigator.space_time.entity_count(), 0);
        assert_eq!(navigator.time_space.entity_count(), 0);
    }

    #[test]
    fn test_navigate_to_time_space_success() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Add entity to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        // Navigate to Time/Space
        // Base: 0.8, Polarity: 1.0 * 0.3 = 0.3, Total: 0.5
        // Still above 0.4, so need to use D4 which has base 0.6
        let result = navigator.navigate_to_time_space(entity_id, Density::Fourth, &polarization);
        assert_eq!(result, NavigationResult::Success);
        assert!(navigator.time_space.contains_entity(entity_id));
    }

    #[test]
    fn test_navigate_to_time_space_veil_too_thick() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::neutral(); // No thinning

        // Add entity to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        // Navigate to Time/Space (should fail due to thick veil)
        let result = navigator.navigate_to_time_space(entity_id, Density::Third, &polarization);
        assert_eq!(result, NavigationResult::VeilTooThick);
    }

    #[test]
    fn test_navigate_to_time_space_entity_not_found() {
        let mut navigator = DualRealmNavigator::new();
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Try to navigate non-existent entity
        let result = navigator.navigate_to_time_space(999, Density::Fourth, &polarization);
        assert_eq!(result, NavigationResult::EntityNotFound);
    }

    #[test]
    fn test_navigate_to_space_time_success() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Add entity to Space/Time first
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos.clone());

        // Navigate to Time/Space (use D4 with base 0.6)
        navigator.navigate_to_time_space(entity_id, Density::Fourth, &polarization);

        // Navigate back to Space/Time
        let result = navigator.navigate_to_space_time(entity_id);
        assert_eq!(result, NavigationResult::Success);
        assert!(navigator.space_time.contains_entity(entity_id));
    }

    #[test]
    fn test_navigate_to_space_time_entity_not_found() {
        let mut navigator = DualRealmNavigator::new();

        // Try to navigate non-existent entity
        let result = navigator.navigate_to_space_time(999);
        assert_eq!(result, NavigationResult::EntityNotFound);
    }

    #[test]
    fn test_get_entity_realm() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;

        // Add to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        let realm = navigator.get_entity_realm(entity_id);
        assert_eq!(realm, Some(EntityRealm::SpaceTime));
    }

    #[test]
    fn test_can_navigate_to_time_space_true() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Add entity to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        // Check if can navigate (D4 with base 0.6, full polarization = 0.3, total = 0.3 < 0.4)
        let can_navigate =
            navigator.can_navigate_to_time_space(entity_id, Density::Fourth, &polarization);
        assert!(can_navigate);
    }

    #[test]
    fn test_can_navigate_to_time_space_false() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::neutral(); // No thinning

        // Add entity to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        // Check if can navigate
        let can_navigate =
            navigator.can_navigate_to_time_space(entity_id, Density::Third, &polarization);
        assert!(!can_navigate);
    }

    #[test]
    fn test_get_statistics() {
        let mut navigator = DualRealmNavigator::new();

        // Add entities to Space/Time
        navigator.add_entity_to_space_time(1, PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1));
        navigator.add_entity_to_space_time(2, PhysicalSpaceTimeCoord::new(0.0, 0.0, 0.0, 0.0, 1));

        // Add entities to Time/Space
        navigator.add_entity_to_time_space(
            3,
            MetaphysicalTimeSpaceCoord::new(
                crate::coordinates::ExperienceVector::new(),
                crate::coordinates::IncarnationPlan::new(),
                crate::coordinates::BroaderConsciousnessAccess::new(),
            ),
        );

        let stats = navigator.get_statistics();
        assert_eq!(stats.space_time_entity_count, 2);
        assert_eq!(stats.time_space_entity_count, 1);
        assert_eq!(stats.total_entities, 3);
    }

    #[test]
    fn test_remove_entity() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Add entity to Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);

        // Navigate to Time/Space (use D4 with base 0.6)
        navigator.navigate_to_time_space(entity_id, Density::Fourth, &polarization);

        // Remove entity
        navigator.remove_entity(entity_id);

        assert!(!navigator.space_time.contains_entity(entity_id));
        assert!(!navigator.time_space.contains_entity(entity_id));
    }

    #[test]
    fn test_dual_realm_navigator_default() {
        let navigator = DualRealmNavigator::default();
        assert_eq!(navigator.space_time.entity_count(), 0);
    }

    #[test]
    fn test_dual_realm_navigator_clone() {
        let navigator1 = DualRealmNavigator::new();
        let navigator2 = navigator1.clone();
        assert_eq!(navigator1, navigator2);
    }

    #[test]
    fn test_dual_realm_navigator_partial_eq() {
        let navigator1 = DualRealmNavigator::new();
        let navigator2 = DualRealmNavigator::new();
        let navigator3 = DualRealmNavigator::with_settings(
            RealmTranslation::with_settings(3.0e8, 0.3),
            VeilMechanism::new(),
        );

        assert_eq!(navigator1, navigator2);
        assert_ne!(navigator1, navigator3);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_full_navigation_cycle() {
        let mut navigator = DualRealmNavigator::new();
        let entity_id = 1;
        let polarization = PolarizationState::new(Polarity::STO, 1.0); // Full polarization

        // Start in Space/Time
        let pos = PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        navigator.add_entity_to_space_time(entity_id, pos);
        assert_eq!(
            navigator.get_entity_realm(entity_id),
            Some(EntityRealm::SpaceTime)
        );

        // Navigate to Time/Space (use D4 with base 0.6)
        let result1 = navigator.navigate_to_time_space(entity_id, Density::Fourth, &polarization);
        assert_eq!(result1, NavigationResult::Success);
        assert_eq!(
            navigator.get_entity_realm(entity_id),
            Some(EntityRealm::TimeSpace)
        );

        // Navigate back to Space/Time
        let result2 = navigator.navigate_to_space_time(entity_id);
        assert_eq!(result2, NavigationResult::Success);
        assert_eq!(
            navigator.get_entity_realm(entity_id),
            Some(EntityRealm::SpaceTime)
        );
    }
}
