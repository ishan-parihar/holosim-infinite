//! Interactive Interface - Dwarf-Fortress-Level Observation
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
//! "Dwarf-Fortress-level depth for observing consciousness evolution"
//!
//! This module provides:
//! - ObserverMode: Different viewing perspectives (entity, collective, cosmic)
//! - ScaleController: Zoom through scale levels (quantum to cosmic)
//! - EventNarrator: Meaningful descriptions of simulation events
//! - EntityInspector: Detailed entity inspection with tabs
//! - InteractiveInterface: Unified observation interface

use crate::entity_layer7::layer7::EntityId;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Observer Mode
// ============================================================================

/// Observer viewing mode
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
/// "View from entity's perspective, collective view, or cosmic overview"
#[derive(Debug, Clone, PartialEq)]
pub enum ObserverMode {
    /// View from entity's perspective
    EntityView { entity_id: EntityId },
    /// View a group/SMC collectively
    CollectiveView { group_id: u64 },
    /// Cosmic overview
    CosmicView,
    /// Free camera navigation
    FreeCamera { position: [Float; 3] },
}

impl Default for ObserverMode {
    fn default() -> Self {
        ObserverMode::CosmicView
    }
}

// ============================================================================
// Scale Level
// ============================================================================

/// Scale level for observation
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
/// "Multi-scale navigation from quantum to cosmic"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleLevel {
    /// Quantum scale (10^-35 m) - Planck scale
    Quantum,
    /// Atomic scale (10^-15 m) - Subatomic particles
    Atomic,
    /// Cellular scale (10^-6 m) - Biological cells
    Cellular,
    /// Organism scale (10^0 m) - Life forms
    Organism,
    /// Planetary scale (10^7 m) - Planets
    Planetary,
    /// Stellar scale (10^13 m) - Stars
    Stellar,
    /// Galactic scale (10^21 m) - Galaxies
    Galactic,
    /// Cosmic scale (10^26 m) - Observable universe
    Cosmic,
}

impl ScaleLevel {
    /// Get the approximate scale in meters
    pub fn scale_in_meters(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1.0e-35,
            ScaleLevel::Atomic => 1.0e-15,
            ScaleLevel::Cellular => 1.0e-6,
            ScaleLevel::Organism => 1.0e0,
            ScaleLevel::Planetary => 1.0e7,
            ScaleLevel::Stellar => 1.0e13,
            ScaleLevel::Galactic => 1.0e21,
            ScaleLevel::Cosmic => 1.0e26,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            ScaleLevel::Quantum => "Quantum",
            ScaleLevel::Atomic => "Atomic",
            ScaleLevel::Cellular => "Cellular",
            ScaleLevel::Organism => "Organism",
            ScaleLevel::Planetary => "Planetary",
            ScaleLevel::Stellar => "Stellar",
            ScaleLevel::Galactic => "Galactic",
            ScaleLevel::Cosmic => "Cosmic",
        }
    }

    /// Get the next larger scale
    pub fn zoom_out(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => Some(ScaleLevel::Atomic),
            ScaleLevel::Atomic => Some(ScaleLevel::Cellular),
            ScaleLevel::Cellular => Some(ScaleLevel::Organism),
            ScaleLevel::Organism => Some(ScaleLevel::Planetary),
            ScaleLevel::Planetary => Some(ScaleLevel::Stellar),
            ScaleLevel::Stellar => Some(ScaleLevel::Galactic),
            ScaleLevel::Galactic => Some(ScaleLevel::Cosmic),
            ScaleLevel::Cosmic => None,
        }
    }

    /// Get the next smaller scale
    pub fn zoom_in(&self) -> Option<ScaleLevel> {
        match self {
            ScaleLevel::Quantum => None,
            ScaleLevel::Atomic => Some(ScaleLevel::Quantum),
            ScaleLevel::Cellular => Some(ScaleLevel::Atomic),
            ScaleLevel::Organism => Some(ScaleLevel::Cellular),
            ScaleLevel::Planetary => Some(ScaleLevel::Organism),
            ScaleLevel::Stellar => Some(ScaleLevel::Planetary),
            ScaleLevel::Galactic => Some(ScaleLevel::Stellar),
            ScaleLevel::Cosmic => Some(ScaleLevel::Galactic),
        }
    }
}

impl Default for ScaleLevel {
    fn default() -> Self {
        ScaleLevel::Organism
    }
}

// ============================================================================
// Scale Controller
// ============================================================================

/// Scale zoom controller
///
/// Manages smooth transitions between scale levels.
#[derive(Debug, Clone)]
pub struct ScaleController {
    /// Current scale level
    pub current_scale: ScaleLevel,
    /// Target scale for transition
    pub target_scale: Option<ScaleLevel>,
    /// Transition progress (0.0 to 1.0)
    pub transition_progress: Float,
    /// Transition speed (progress per second)
    pub transition_speed: Float,
}

impl Default for ScaleController {
    fn default() -> Self {
        ScaleController {
            current_scale: ScaleLevel::Organism,
            target_scale: None,
            transition_progress: 0.0,
            transition_speed: 2.0,
        }
    }
}

impl ScaleController {
    /// Create a new scale controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a zoom transition to a new scale
    pub fn zoom_to_scale(&mut self, target: ScaleLevel) {
        if self.current_scale != target {
            self.target_scale = Some(target);
            self.transition_progress = 0.0;
        }
    }

    /// Zoom in one level
    pub fn zoom_in(&mut self) -> bool {
        if let Some(smaller) = self.current_scale.zoom_in() {
            self.zoom_to_scale(smaller);
            true
        } else {
            false
        }
    }

    /// Zoom out one level
    pub fn zoom_out(&mut self) -> bool {
        if let Some(larger) = self.current_scale.zoom_out() {
            self.zoom_to_scale(larger);
            true
        } else {
            false
        }
    }

    /// Update transition progress
    pub fn update(&mut self, delta_time: Float) {
        if let Some(target) = self.target_scale {
            self.transition_progress += self.transition_speed * delta_time;

            if self.transition_progress >= 1.0 {
                self.current_scale = target;
                self.target_scale = None;
                self.transition_progress = 0.0;
            }
        }
    }

    /// Check if currently transitioning
    pub fn is_transitioning(&self) -> bool {
        self.target_scale.is_some()
    }

    /// Get interpolated scale in meters for smooth transitions
    pub fn current_scale_meters(&self) -> Float {
        if let Some(target) = self.target_scale {
            let from = self.current_scale.scale_in_meters();
            let to = target.scale_in_meters();
            // Smooth interpolation using smoothstep
            let t = self.transition_progress;
            let smooth_t = t * t * (3.0 - 2.0 * t);
            from * (to / from).powf(smooth_t)
        } else {
            self.current_scale.scale_in_meters()
        }
    }
}

// ============================================================================
// Event Narrator
// ============================================================================

/// Simulation event for narration
///
/// These are significant events in the simulation that warrant
/// human-readable descriptions.
#[derive(Debug, Clone)]
pub enum SimulationEvent {
    /// An entity made a polarity choice
    EntityChoice {
        entity_id: EntityId,
        polarity: Float,
    },
    /// A Social Memory Complex formed
    SMCFormation { group_id: u64, member_count: usize },
    /// An entity was harvested to a higher density
    HarvestComplete {
        entity_id: EntityId,
        next_density: u8,
    },
    /// A wanderer incarnated
    WandererIncarnation { entity_id: EntityId },
    /// A civilization emerged
    CivilizationEmergence { civilization_id: u64 },
    /// An entity died
    EntityDeath { entity_id: EntityId, cause: String },
    /// Major archetype activation
    ArchetypeActivation {
        entity_id: EntityId,
        archetype: u8,
        level: Float,
    },
    /// Catalyst processed
    CatalystProcessed {
        entity_id: EntityId,
        catalyst_type: String,
        outcome: String,
    },
}

/// Event narrator for meaningful descriptions
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
/// "Events are narrated meaningfully"
#[derive(Debug, Clone)]
pub struct EventNarrator {
    /// Recorded events
    pub events: Vec<SimulationEvent>,
    /// Maximum events to store
    pub max_events: usize,
}

impl Default for EventNarrator {
    fn default() -> Self {
        EventNarrator {
            events: Vec::new(),
            max_events: 1000,
        }
    }
}

impl EventNarrator {
    /// Create a new event narrator
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom max events
    pub fn with_max_events(max: usize) -> Self {
        EventNarrator {
            events: Vec::new(),
            max_events: max,
        }
    }

    /// Add an event
    pub fn add_event(&mut self, event: SimulationEvent) {
        self.events.push(event);
        // Trim if over limit
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }

    /// Generate a human-readable narration for an event
    pub fn narrate(&self, event: &SimulationEvent) -> String {
        match event {
            SimulationEvent::EntityChoice {
                entity_id,
                polarity,
            } => {
                let direction = if *polarity > 0.0 {
                    "Service-to-Others"
                } else if *polarity < 0.0 {
                    "Service-to-Self"
                } else {
                    "Neutral"
                };
                format!(
                    "Entity {} made a {} choice (polarity: {:.2})",
                    entity_id, direction, polarity
                )
            }
            SimulationEvent::SMCFormation {
                group_id,
                member_count,
            } => {
                format!(
                    "A Social Memory Complex formed (Group #{}) with {} members, \
                     their minds now resonate as one",
                    group_id, member_count
                )
            }
            SimulationEvent::HarvestComplete {
                entity_id,
                next_density,
            } => {
                let density_name = match next_density {
                    1 => "1st",
                    2 => "2nd",
                    3 => "3rd",
                    4 => "4th",
                    5 => "5th",
                    6 => "6th",
                    7 => "7th",
                    8 => "8th",
                    _ => "unknown",
                };
                format!(
                    "Entity {} has completed their harvest and graduates to {} density",
                    entity_id, density_name
                )
            }
            SimulationEvent::WandererIncarnation { entity_id } => {
                format!(
                    "A Wanderer from higher densities has incarnated as {} to assist in planetary healing",
                    entity_id
                )
            }
            SimulationEvent::CivilizationEmergence { civilization_id } => {
                format!(
                    "A new civilization has emerged (ID: {}), marking a collective step in consciousness",
                    civilization_id
                )
            }
            SimulationEvent::EntityDeath { entity_id, cause } => {
                format!(
                    "Entity {} has passed from physical existence. Cause: {}",
                    entity_id, cause
                )
            }
            SimulationEvent::ArchetypeActivation {
                entity_id,
                archetype,
                level,
            } => {
                let archetype_name = Self::archetype_name(*archetype);
                format!(
                    "Entity {} activated Archetype {} ({}) at {:.1}% intensity",
                    entity_id,
                    archetype,
                    archetype_name,
                    level * 100.0
                )
            }
            SimulationEvent::CatalystProcessed {
                entity_id,
                catalyst_type,
                outcome,
            } => {
                format!(
                    "Entity {} processed {} catalyst with outcome: {}",
                    entity_id, catalyst_type, outcome
                )
            }
        }
    }

    /// Get archetype name by number
    fn archetype_name(num: u8) -> &'static str {
        match num {
            1 => "Matrix of the Mind",
            2 => "Potentiator of the Mind",
            3 => "Catalyst of the Mind",
            4 => "Experience of the Mind",
            5 => "Significator of the Mind",
            6 => "Matrix of the Body",
            7 => "Potentiator of the Body",
            8 => "Catalyst of the Body",
            9 => "Experience of the Body",
            10 => "Significator of the Body",
            11 => "Matrix of the Spirit",
            12 => "Potentiator of the Spirit",
            13 => "Catalyst of the Spirit",
            14 => "Experience of the Spirit",
            15 => "Significator of the Spirit",
            16 => "Choice",
            17 => "Matrix of the Mind/Body/Spirit",
            18 => "Potentiator of the Mind/Body/Spirit",
            19 => "Catalyst of the Mind/Body/Spirit",
            20 => "Experience of the Mind/Body/Spirit",
            21 => "Significator of the Mind/Body/Spirit",
            22 => "The Great Way",
            _ => "Unknown",
        }
    }

    /// Get recent events as narrated strings
    pub fn narrate_recent(&self, count: usize) -> Vec<String> {
        self.events
            .iter()
            .rev()
            .take(count)
            .map(|e| self.narrate(e))
            .collect()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

// ============================================================================
// Entity Inspector
// ============================================================================

/// Inspector tab
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectorTab {
    /// Physical body status
    Body,
    /// Consciousness state
    Consciousness,
    /// Sensory input
    SensoryField,
    /// Experience history
    ExperienceHistory,
    /// Social relationships
    Relationships,
    /// Evolutionary trajectory
    EvolutionaryTrajectory,
}

impl Default for InspectorTab {
    fn default() -> Self {
        InspectorTab::Body
    }
}

impl InspectorTab {
    /// Get tab display name
    pub fn name(&self) -> &'static str {
        match self {
            InspectorTab::Body => "Body",
            InspectorTab::Consciousness => "Consciousness",
            InspectorTab::SensoryField => "Sensory Field",
            InspectorTab::ExperienceHistory => "Experience History",
            InspectorTab::Relationships => "Relationships",
            InspectorTab::EvolutionaryTrajectory => "Evolutionary Trajectory",
        }
    }

    /// Get all tabs
    pub fn all() -> [InspectorTab; 6] {
        [
            InspectorTab::Body,
            InspectorTab::Consciousness,
            InspectorTab::SensoryField,
            InspectorTab::ExperienceHistory,
            InspectorTab::Relationships,
            InspectorTab::EvolutionaryTrajectory,
        ]
    }
}

/// Entity inspector for detailed observation
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
/// "Can inspect any entity in detail"
#[derive(Debug, Clone)]
pub struct EntityInspector {
    /// Entity being inspected
    pub entity_id: EntityId,
    /// Currently active tab
    pub active_tab: InspectorTab,
    /// Inspector history (previously inspected entities)
    pub history: Vec<EntityId>,
    /// Maximum history size
    pub max_history: usize,
}

impl EntityInspector {
    /// Create a new inspector for an entity
    pub fn new(entity_id: EntityId) -> Self {
        EntityInspector {
            entity_id,
            active_tab: InspectorTab::default(),
            history: Vec::new(),
            max_history: 50,
        }
    }

    /// Switch to a different tab
    pub fn switch_tab(&mut self, tab: InspectorTab) {
        self.active_tab = tab;
    }

    /// Navigate to inspect a different entity
    pub fn navigate_to(&mut self, entity_id: EntityId) {
        // Save current to history
        self.history.push(self.entity_id.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        self.entity_id = entity_id;
    }

    /// Go back to previous entity
    pub fn go_back(&mut self) -> Option<EntityId> {
        if let Some(prev_id) = self.history.pop() {
            self.entity_id = prev_id.clone();
            Some(prev_id)
        } else {
            None
        }
    }
}

// ============================================================================
// Bookmark
// ============================================================================

/// Bookmark for saved observation positions
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// Bookmark name
    pub name: String,
    /// Observer mode when bookmarked
    pub observer_mode: ObserverMode,
    /// Scale level when bookmarked
    pub scale: ScaleLevel,
    /// Timestamp when created
    pub created_at: u64,
}

impl Bookmark {
    /// Create a new bookmark
    pub fn new(name: String, observer_mode: ObserverMode, scale: ScaleLevel) -> Self {
        Bookmark {
            name,
            observer_mode,
            scale,
            created_at: 0, // Would be set by interface
        }
    }
}

// ============================================================================
// Interactive Interface
// ============================================================================

/// The unified interactive interface
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 8:
/// "Dwarf-Fortress-level depth for observing consciousness evolution"
///
/// This integrates:
/// - ObserverMode for perspective
/// - ScaleController for zoom
/// - EntityInspector for details
/// - EventNarrator for history
/// - Bookmarks for navigation
/// - Time control
pub struct InteractiveInterface {
    /// Current observer mode
    pub observer_mode: ObserverMode,
    /// Scale controller
    pub scale_controller: ScaleController,
    /// Entity inspector (when inspecting)
    pub entity_inspector: Option<EntityInspector>,
    /// Event narrator
    pub event_narrator: EventNarrator,
    /// Saved bookmarks
    pub bookmarks: Vec<Bookmark>,
    /// Time paused
    pub time_paused: bool,
    /// Time rate (1.0 = normal)
    pub time_rate: Float,
    /// Minimum time rate
    pub min_time_rate: Float,
    /// Maximum time rate
    pub max_time_rate: Float,
    /// Current simulation tick
    pub current_tick: u64,
}

impl Default for InteractiveInterface {
    fn default() -> Self {
        InteractiveInterface {
            observer_mode: ObserverMode::CosmicView,
            scale_controller: ScaleController::new(),
            entity_inspector: None,
            event_narrator: EventNarrator::new(),
            bookmarks: Vec::new(),
            time_paused: false,
            time_rate: 1.0,
            min_time_rate: 0.1,
            max_time_rate: 1000.0,
            current_tick: 0,
        }
    }
}

impl InteractiveInterface {
    /// Create a new interactive interface
    pub fn new() -> Self {
        Self::default()
    }

    // ========================================================================
    // Observer Mode
    // ========================================================================

    /// Observe a specific entity
    pub fn observe_entity(&mut self, entity_id: EntityId) {
        self.observer_mode = ObserverMode::EntityView {
            entity_id: entity_id.clone(),
        };
        // Create inspector if not already inspecting this entity
        match &self.entity_inspector {
            Some(inspector) if inspector.entity_id == entity_id => {}
            _ => {
                self.entity_inspector = Some(EntityInspector::new(entity_id));
            }
        }
    }

    /// Observe a collective/group
    pub fn observe_collective(&mut self, group_id: u64) {
        self.observer_mode = ObserverMode::CollectiveView { group_id };
        self.entity_inspector = None;
    }

    /// Switch to cosmic overview
    pub fn observe_cosmic(&mut self) {
        self.observer_mode = ObserverMode::CosmicView;
        self.entity_inspector = None;
    }

    /// Set free camera position
    pub fn set_camera_position(&mut self, position: [Float; 3]) {
        self.observer_mode = ObserverMode::FreeCamera { position };
    }

    // ========================================================================
    // Scale Control
    // ========================================================================

    /// Zoom to a specific scale
    pub fn zoom_to_scale(&mut self, scale: ScaleLevel) {
        self.scale_controller.zoom_to_scale(scale);
    }

    /// Zoom in one level
    pub fn zoom_in(&mut self) -> bool {
        self.scale_controller.zoom_in()
    }

    /// Zoom out one level
    pub fn zoom_out(&mut self) -> bool {
        self.scale_controller.zoom_out()
    }

    // ========================================================================
    // Time Control
    // ========================================================================

    /// Pause simulation time
    pub fn pause_time(&mut self) {
        self.time_paused = true;
    }

    /// Resume simulation time
    pub fn resume_time(&mut self) {
        self.time_paused = false;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.time_paused = !self.time_paused;
    }

    /// Set time rate
    pub fn set_time_rate(&mut self, rate: Float) {
        self.time_rate = rate.max(self.min_time_rate).min(self.max_time_rate);
    }

    // ========================================================================
    // Events
    // ========================================================================

    /// Record an event
    pub fn record_event(&mut self, event: SimulationEvent) {
        self.event_narrator.add_event(event);
    }

    /// Get recent event narrations
    pub fn narrate_recent_events(&self, count: usize) -> Vec<String> {
        self.event_narrator.narrate_recent(count)
    }

    // ========================================================================
    // Bookmarks
    // ========================================================================

    /// Add a bookmark
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        // Set timestamp
        let mut bookmark = bookmark;
        bookmark.created_at = self.current_tick;
        self.bookmarks.push(bookmark);
    }

    /// Get a bookmark by name
    pub fn get_bookmark(&self, name: &str) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| b.name == name)
    }

    /// Remove a bookmark
    pub fn remove_bookmark(&mut self, name: &str) -> bool {
        let len = self.bookmarks.len();
        self.bookmarks.retain(|b| b.name != name);
        self.bookmarks.len() != len
    }

    /// Jump to a bookmark
    pub fn jump_to_bookmark(&mut self, name: &str) -> bool {
        // Find and clone bookmark data to avoid borrow issues
        let bookmark_data = self
            .bookmarks
            .iter()
            .find(|b| b.name == name)
            .map(|b| (b.observer_mode.clone(), b.scale));

        if let Some((mode, scale)) = bookmark_data {
            self.observer_mode = mode;
            self.scale_controller.zoom_to_scale(scale);
            true
        } else {
            false
        }
    }

    // ========================================================================
    // Inspector
    // ========================================================================

    /// Switch inspector tab
    pub fn switch_inspector_tab(&mut self, tab: InspectorTab) {
        if let Some(ref mut inspector) = self.entity_inspector {
            inspector.switch_tab(tab);
        }
    }

    /// Navigate inspector to another entity
    pub fn navigate_inspector_to(&mut self, entity_id: EntityId) {
        if let Some(ref mut inspector) = self.entity_inspector {
            inspector.navigate_to(entity_id);
        } else {
            self.entity_inspector = Some(EntityInspector::new(entity_id));
        }
    }

    /// Go back in inspector history
    pub fn inspector_go_back(&mut self) -> Option<EntityId> {
        if let Some(ref mut inspector) = self.entity_inspector {
            inspector.go_back()
        } else {
            None
        }
    }

    // ========================================================================
    // Update
    // ========================================================================

    /// Update the interface (called each tick)
    pub fn update(&mut self, delta_time: Float) {
        // Update scale transitions
        self.scale_controller.update(delta_time);

        // Advance tick
        if !self.time_paused {
            self.current_tick += 1;
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_mode_default() {
        let mode = ObserverMode::default();
        assert!(matches!(mode, ObserverMode::CosmicView));
    }

    #[test]
    fn test_scale_level_ordering() {
        assert!(ScaleLevel::Quantum < ScaleLevel::Atomic);
        assert!(ScaleLevel::Atomic < ScaleLevel::Cellular);
        assert!(ScaleLevel::Cellular < ScaleLevel::Organism);
        assert!(ScaleLevel::Organism < ScaleLevel::Planetary);
        assert!(ScaleLevel::Planetary < ScaleLevel::Stellar);
        assert!(ScaleLevel::Stellar < ScaleLevel::Galactic);
        assert!(ScaleLevel::Galactic < ScaleLevel::Cosmic);
    }

    #[test]
    fn test_scale_level_zoom() {
        assert_eq!(ScaleLevel::Quantum.zoom_in(), None);
        assert_eq!(ScaleLevel::Quantum.zoom_out(), Some(ScaleLevel::Atomic));

        assert_eq!(ScaleLevel::Cosmic.zoom_out(), None);
        assert_eq!(ScaleLevel::Cosmic.zoom_in(), Some(ScaleLevel::Galactic));
    }

    #[test]
    fn test_scale_controller_new() {
        let controller = ScaleController::new();
        assert_eq!(controller.current_scale, ScaleLevel::Organism);
        assert!(controller.target_scale.is_none());
        assert_eq!(controller.transition_progress, 0.0);
    }

    #[test]
    fn test_scale_controller_zoom() {
        let mut controller = ScaleController::new();
        controller.zoom_to_scale(ScaleLevel::Quantum);

        assert_eq!(controller.target_scale, Some(ScaleLevel::Quantum));
        assert!(controller.is_transitioning());

        // Complete transition
        controller.update(2.0);
        assert_eq!(controller.current_scale, ScaleLevel::Quantum);
        assert!(!controller.is_transitioning());
    }

    #[test]
    fn test_event_narrator_new() {
        let narrator = EventNarrator::new();
        assert!(narrator.events.is_empty());
        assert_eq!(narrator.max_events, 1000);
    }

    #[test]
    fn test_event_narrator_add_event() {
        let mut narrator = EventNarrator::new();
        narrator.add_event(SimulationEvent::EntityChoice {
            entity_id: EntityId::new("test".to_string()),
            polarity: 0.5,
        });

        assert_eq!(narrator.events.len(), 1);
    }

    #[test]
    fn test_event_narrator_narrate_choice() {
        let narrator = EventNarrator::new();

        let event = SimulationEvent::EntityChoice {
            entity_id: EntityId::new("alice".to_string()),
            polarity: 0.8,
        };

        let narration = narrator.narrate(&event);
        assert!(narration.contains("alice"));
        assert!(narration.contains("Service-to-Others"));
    }

    #[test]
    fn test_event_narrator_narrate_harvest() {
        let narrator = EventNarrator::new();

        let event = SimulationEvent::HarvestComplete {
            entity_id: EntityId::new("bob".to_string()),
            next_density: 4,
        };

        let narration = narrator.narrate(&event);
        assert!(narration.contains("bob"));
        assert!(narration.contains("harvest"));
        assert!(narration.contains("4th"));
    }

    #[test]
    fn test_entity_inspector_new() {
        let entity_id = EntityId::new("test".to_string());
        let inspector = EntityInspector::new(entity_id.clone());

        assert_eq!(inspector.entity_id, entity_id);
        assert_eq!(inspector.active_tab, InspectorTab::Body);
    }

    #[test]
    fn test_entity_inspector_switch_tab() {
        let entity_id = EntityId::new("test".to_string());
        let mut inspector = EntityInspector::new(entity_id);

        inspector.switch_tab(InspectorTab::Consciousness);
        assert_eq!(inspector.active_tab, InspectorTab::Consciousness);
    }

    #[test]
    fn test_interactive_interface_new() {
        let interface = InteractiveInterface::new();

        assert!(matches!(interface.observer_mode, ObserverMode::CosmicView));
        assert!(interface.entity_inspector.is_none());
        assert!(!interface.time_paused);
        assert_eq!(interface.time_rate, 1.0);
    }

    #[test]
    fn test_interactive_interface_observe_entity() {
        let mut interface = InteractiveInterface::new();
        let entity_id = EntityId::new("test".to_string());

        interface.observe_entity(entity_id.clone());

        assert!(matches!(
            interface.observer_mode,
            ObserverMode::EntityView { .. }
        ));
        assert!(interface.entity_inspector.is_some());
    }

    #[test]
    fn test_interactive_interface_time_control() {
        let mut interface = InteractiveInterface::new();

        interface.pause_time();
        assert!(interface.time_paused);

        interface.resume_time();
        assert!(!interface.time_paused);

        interface.set_time_rate(500.0);
        assert_eq!(interface.time_rate, 500.0);

        // Clamp to max
        interface.set_time_rate(2000.0);
        assert_eq!(interface.time_rate, 1000.0);

        // Clamp to min
        interface.set_time_rate(0.01);
        assert_eq!(interface.time_rate, 0.1);
    }

    #[test]
    fn test_bookmark() {
        let bookmark = Bookmark::new(
            "Test".to_string(),
            ObserverMode::CosmicView,
            ScaleLevel::Galactic,
        );

        assert_eq!(bookmark.name, "Test");
        assert_eq!(bookmark.scale, ScaleLevel::Galactic);
    }

    #[test]
    fn test_interactive_interface_bookmarks() {
        let mut interface = InteractiveInterface::new();

        interface.add_bookmark(Bookmark::new(
            "Home".to_string(),
            ObserverMode::CosmicView,
            ScaleLevel::Organism,
        ));

        assert_eq!(interface.bookmarks.len(), 1);
        assert!(interface.get_bookmark("Home").is_some());

        interface.remove_bookmark("Home");
        assert!(interface.get_bookmark("Home").is_none());
    }

    #[test]
    fn test_inspector_tabs() {
        let tabs = InspectorTab::all();

        assert_eq!(tabs.len(), 6);
        assert_eq!(tabs[0].name(), "Body");
        assert_eq!(tabs[1].name(), "Consciousness");
    }
}
