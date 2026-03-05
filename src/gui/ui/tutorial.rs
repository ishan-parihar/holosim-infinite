//! Tutorial System
//!
//! Provides an interactive tutorial mode for new users with step-by-step
//! guided tours of the simulation interface and features.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Tutorial mode for new users"

use std::collections::HashMap;

/// A single step in the tutorial
#[derive(Debug, Clone)]
pub struct TutorialStep {
    /// Unique step identifier
    pub id: String,
    /// Step title
    pub title: String,
    /// Step description/instruction
    pub description: String,
    /// Target UI element (if applicable)
    pub target: Option<String>,
    /// Step position relative to target
    pub position: TooltipPosition,
    /// Required action to advance
    pub action: StepAction,
    /// Whether this step can be skipped
    pub skippable: bool,
    /// Screenshot or visual reference
    pub visual_hint: Option<String>,
    /// Additional tips
    pub tips: Vec<String>,
}

impl TutorialStep {
    /// Create a new tutorial step
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            target: None,
            position: TooltipPosition::Bottom,
            action: StepAction::ClickAnywhere,
            skippable: true,
            visual_hint: None,
            tips: Vec::new(),
        }
    }

    /// Set the target element
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Set the tooltip position
    pub fn with_position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the required action
    pub fn with_action(mut self, action: StepAction) -> Self {
        self.action = action;
        self
    }

    /// Set whether step can be skipped
    pub fn with_skippable(mut self, skippable: bool) -> Self {
        self.skippable = skippable;
        self
    }

    /// Add a visual hint
    pub fn with_visual_hint(mut self, hint: impl Into<String>) -> Self {
        self.visual_hint = Some(hint.into());
        self
    }

    /// Add a tip
    pub fn with_tip(mut self, tip: impl Into<String>) -> Self {
        self.tips.push(tip.into());
        self
    }
}

/// Position for tutorial tooltips
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    Center,
    Auto,
}

impl TooltipPosition {
    /// Get the opposite position
    pub fn opposite(&self) -> Self {
        match self {
            TooltipPosition::Top => TooltipPosition::Bottom,
            TooltipPosition::Bottom => TooltipPosition::Top,
            TooltipPosition::Left => TooltipPosition::Right,
            TooltipPosition::Right => TooltipPosition::Left,
            TooltipPosition::Center => TooltipPosition::Center,
            TooltipPosition::Auto => TooltipPosition::Auto,
        }
    }
}

/// Actions required to advance a tutorial step
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepAction {
    /// Click anywhere on screen
    ClickAnywhere,
    /// Click specific target element
    ClickTarget,
    /// Press specific key
    PressKey(String),
    /// Perform specific action
    PerformAction(String),
    /// Wait for event
    WaitForEvent(String),
    /// Automatic (no user action needed)
    Automatic,
}

/// Tutorial category/type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TutorialCategory {
    /// Getting started basics
    GettingStarted,
    /// Camera navigation
    Navigation,
    /// Entity interaction
    Interaction,
    /// Time controls
    TimeControls,
    /// UI panels overview
    Panels,
    /// Advanced features
    Advanced,
}

impl TutorialCategory {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            TutorialCategory::GettingStarted => "Getting Started",
            TutorialCategory::Navigation => "Navigation",
            TutorialCategory::Interaction => "Interaction",
            TutorialCategory::TimeControls => "Time Controls",
            TutorialCategory::Panels => "UI Panels",
            TutorialCategory::Advanced => "Advanced Features",
        }
    }

    /// Get icon
    pub fn icon(&self) -> &'static str {
        match self {
            TutorialCategory::GettingStarted => "🚀",
            TutorialCategory::Navigation => "🧭",
            TutorialCategory::Interaction => "👆",
            TutorialCategory::TimeControls => "⏱️",
            TutorialCategory::Panels => "🖥️",
            TutorialCategory::Advanced => "⚡",
        }
    }

    /// Get all categories
    pub fn all() -> &'static [TutorialCategory] {
        &[
            TutorialCategory::GettingStarted,
            TutorialCategory::Navigation,
            TutorialCategory::Interaction,
            TutorialCategory::TimeControls,
            TutorialCategory::Panels,
            TutorialCategory::Advanced,
        ]
    }
}

/// A complete tutorial with multiple steps
#[derive(Debug, Clone)]
pub struct Tutorial {
    /// Tutorial unique identifier
    pub id: String,
    /// Tutorial title
    pub title: String,
    /// Tutorial description
    pub description: String,
    /// Tutorial category
    pub category: TutorialCategory,
    /// Tutorial steps
    pub steps: Vec<TutorialStep>,
    /// Estimated duration in minutes
    pub estimated_duration: u32,
    /// Whether tutorial has been completed
    pub completed: bool,
    /// Completion timestamp
    pub completed_at: Option<std::time::SystemTime>,
    /// Prerequisites (other tutorial IDs)
    pub prerequisites: Vec<String>,
}

impl Tutorial {
    /// Create a new tutorial
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        category: TutorialCategory,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: String::new(),
            category,
            steps: Vec::new(),
            estimated_duration: 5,
            completed: false,
            completed_at: None,
            prerequisites: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Add a step
    pub fn with_step(mut self, step: TutorialStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Set estimated duration
    pub fn with_duration(mut self, minutes: u32) -> Self {
        self.estimated_duration = minutes;
        self
    }

    /// Add prerequisite
    pub fn with_prerequisite(mut self, prereq: impl Into<String>) -> Self {
        self.prerequisites.push(prereq.into());
        self
    }

    /// Get number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Mark as completed
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.completed_at = Some(std::time::SystemTime::now());
    }

    /// Reset completion status
    pub fn reset(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }
}

/// Manages tutorial state and progression
pub struct TutorialManager {
    /// Available tutorials
    tutorials: HashMap<String, Tutorial>,
    /// Currently active tutorial
    active_tutorial: Option<String>,
    /// Current step index
    current_step_index: usize,
    /// Whether tutorial mode is enabled
    pub enabled: bool,
    /// Whether to show tutorials on startup
    pub show_on_startup: bool,
    /// Completed tutorial IDs
    completed_tutorials: Vec<String>,
    /// Tutorial progress callbacks
    progress_callbacks: Vec<Box<dyn Fn(&str, usize, usize) + Send>>,
    /// Completion callbacks
    completion_callbacks: Vec<Box<dyn Fn(&str) + Send>>,
    /// Whether user has dismissed tutorials
    pub dismissed: bool,
}

impl Default for TutorialManager {
    fn default() -> Self {
        let mut manager = Self {
            tutorials: HashMap::new(),
            active_tutorial: None,
            current_step_index: 0,
            enabled: true,
            show_on_startup: true,
            completed_tutorials: Vec::new(),
            progress_callbacks: Vec::new(),
            completion_callbacks: Vec::new(),
            dismissed: false,
        };

        // Register default tutorials
        manager.register_default_tutorials();

        manager
    }
}

impl TutorialManager {
    /// Create a new tutorial manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Register default tutorials
    fn register_default_tutorials(&mut self) {
        // Getting Started Tutorial
        let getting_started = Tutorial::new(
            "getting_started",
            "Welcome to Holonic Realms",
            TutorialCategory::GettingStarted,
        )
        .with_description("Learn the basics of navigating and interacting with the simulation.")
        .with_duration(3)
        .with_step(
            TutorialStep::new(
                "welcome",
                "Welcome!",
                "Welcome to Holonic Realms, a cosmological simulation of consciousness evolution through the density octave.",
            )
            .with_position(TooltipPosition::Center)
            .with_action(StepAction::ClickAnywhere),
        )
        .with_step(
            TutorialStep::new(
                "interface_overview",
                "Interface Overview",
                "The main view shows entities evolving through 8 densities. Use the panels on the sides to inspect details and control the simulation.",
            )
            .with_position(TooltipPosition::Bottom)
            .with_action(StepAction::ClickAnywhere)
            .with_tip("You can collapse panels by clicking their headers"),
        )
        .with_step(
            TutorialStep::new(
                "entities",
                "Understanding Entities",
                "Each colored sphere represents an entity. Colors indicate their density level: Red (1st) through White (8th).",
            )
            .with_position(TooltipPosition::Bottom)
            .with_action(StepAction::ClickAnywhere)
            .with_tip("Hover over entities to see their basic information"),
        )
        .with_step(
            TutorialStep::new(
                "navigation",
                "Navigation",
                "Use arrow keys to pan, +/- to zoom, and mouse drag to rotate the view. Press Home to reset the camera.",
            )
            .with_position(TooltipPosition::Bottom)
            .with_action(StepAction::PressKey("Home".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "complete",
                "You're Ready!",
                "You've learned the basics! Try the other tutorials to learn more about specific features.",
            )
            .with_position(TooltipPosition::Center)
            .with_action(StepAction::ClickAnywhere),
        );

        // Navigation Tutorial
        let navigation = Tutorial::new(
            "navigation",
            "Camera Navigation",
            TutorialCategory::Navigation,
        )
        .with_description("Master the camera controls to explore the simulation at all scales.")
        .with_duration(5)
        .with_prerequisite("getting_started")
        .with_step(
            TutorialStep::new(
                "panning",
                "Panning",
                "Use the arrow keys or hold middle mouse button to pan the camera view.",
            )
            .with_action(StepAction::PerformAction("pan".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "zooming",
                "Zooming",
                "Use +/- keys or scroll wheel to zoom in and out. The simulation spans 61 orders of magnitude!",
            )
            .with_action(StepAction::PerformAction("zoom".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "bookmarks",
                "Camera Bookmarks",
                "Press 1, 2, or 3 to quickly jump to saved camera positions. Ctrl+1/2/3 to save current view.",
            )
            .with_action(StepAction::PressKey("1".to_string())),
        );

        // Interaction Tutorial
        let interaction = Tutorial::new(
            "interaction",
            "Entity Interaction",
            TutorialCategory::Interaction,
        )
        .with_description("Learn how to select, inspect, and track entities.")
        .with_duration(4)
        .with_prerequisite("getting_started")
        .with_step(
            TutorialStep::new(
                "selecting",
                "Selecting Entities",
                "Click on any entity to select it. The Entity Inspector panel will show detailed information.",
            )
            .with_target("entity_inspector")
            .with_action(StepAction::ClickTarget),
        )
        .with_step(
            TutorialStep::new(
                "tracking",
                "Tracking Entities",
                "Press 'T' to track a selected entity. The camera will follow it as it evolves.",
            )
            .with_action(StepAction::PressKey("t".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "tab_navigation",
                "Tab Navigation",
                "Press Tab to cycle through entities, Shift+Tab to go backwards.",
            )
            .with_action(StepAction::PressKey("Tab".to_string())),
        );

        // Time Controls Tutorial
        let time_controls = Tutorial::new(
            "time_controls",
            "Time Controls",
            TutorialCategory::TimeControls,
        )
        .with_description("Master the simulation time controls.")
        .with_duration(3)
        .with_step(
            TutorialStep::new(
                "play_pause",
                "Play/Pause",
                "Press Space to pause or resume the simulation.",
            )
            .with_action(StepAction::PressKey("Space".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "time_rate",
                "Time Rate",
                "Use > and < keys to speed up or slow down time. Try 1000x speed to see evolution!",
            )
            .with_action(StepAction::PressKey(">".to_string())),
        )
        .with_step(
            TutorialStep::new(
                "stepping",
                "Step-by-Step",
                "Use . and , keys to advance or rewind one step at a time.",
            )
            .with_action(StepAction::PressKey(".".to_string())),
        );

        // Register all tutorials
        self.register_tutorial(getting_started);
        self.register_tutorial(navigation);
        self.register_tutorial(interaction);
        self.register_tutorial(time_controls);
    }

    /// Register a tutorial
    pub fn register_tutorial(&mut self, tutorial: Tutorial) {
        self.tutorials.insert(tutorial.id.clone(), tutorial);
    }

    /// Get a tutorial by ID
    pub fn get_tutorial(&self, id: &str) -> Option<&Tutorial> {
        self.tutorials.get(id)
    }

    /// Get all tutorials
    pub fn get_all_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials.values().collect()
    }

    /// Get tutorials by category
    pub fn get_tutorials_by_category(&self, category: TutorialCategory) -> Vec<&Tutorial> {
        self.tutorials
            .values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Get available tutorials (prerequisites met)
    pub fn get_available_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials
            .values()
            .filter(|t| {
                t.prerequisites
                    .iter()
                    .all(|prereq| self.completed_tutorials.contains(prereq))
            })
            .collect()
    }

    /// Start a tutorial
    pub fn start_tutorial(&mut self, tutorial_id: impl Into<String>) -> Result<(), String> {
        let id = tutorial_id.into();

        if let Some(tutorial) = self.tutorials.get(&id) {
            // Check prerequisites
            for prereq in &tutorial.prerequisites {
                if !self.completed_tutorials.contains(prereq) {
                    return Err(format!(
                        "Prerequisite not met: {}. Please complete it first.",
                        prereq
                    ));
                }
            }

            self.active_tutorial = Some(id);
            self.current_step_index = 0;
            Ok(())
        } else {
            Err(format!("Tutorial '{}' not found", id))
        }
    }

    /// Get the currently active tutorial
    pub fn get_active_tutorial(&self) -> Option<&Tutorial> {
        self.active_tutorial
            .as_ref()
            .and_then(|id| self.tutorials.get(id))
    }

    /// Get the current step
    pub fn get_current_step(&self) -> Option<&TutorialStep> {
        self.get_active_tutorial()
            .and_then(|t| t.steps.get(self.current_step_index))
    }

    /// Advance to the next step
    pub fn next_step(&mut self) -> bool {
        let tutorial_id = self.get_active_tutorial().map(|t| t.id.clone());
        let total_steps = self
            .get_active_tutorial()
            .map(|t| t.steps.len())
            .unwrap_or(0);

        if let Some(tutorial) = self.get_active_tutorial() {
            if self.current_step_index < tutorial.steps.len() - 1 {
                self.current_step_index += 1;

                // Notify progress callbacks
                if let Some(ref id) = tutorial_id {
                    for callback in &self.progress_callbacks {
                        callback(id, self.current_step_index, total_steps);
                    }
                }

                return true;
            } else {
                // Tutorial complete
                self.complete_tutorial();
                return false;
            }
        }
        false
    }

    /// Go to previous step
    pub fn previous_step(&mut self) -> bool {
        if self.current_step_index > 0 {
            self.current_step_index -= 1;

            if let Some(tutorial) = self.get_active_tutorial() {
                for callback in &self.progress_callbacks {
                    callback(&tutorial.id, self.current_step_index, tutorial.steps.len());
                }
            }

            return true;
        }
        false
    }

    /// Skip current step
    pub fn skip_step(&mut self) -> bool {
        if let Some(step) = self.get_current_step() {
            if step.skippable {
                return self.next_step();
            }
        }
        false
    }

    /// Complete the active tutorial
    pub fn complete_tutorial(&mut self) {
        if let Some(id) = self.active_tutorial.take() {
            if let Some(tutorial) = self.tutorials.get_mut(&id) {
                tutorial.mark_completed();
            }

            if !self.completed_tutorials.contains(&id) {
                self.completed_tutorials.push(id.clone());
            }

            // Notify completion callbacks
            for callback in &self.completion_callbacks {
                callback(&id);
            }
        }

        self.current_step_index = 0;
    }

    /// Stop the active tutorial without completing
    pub fn stop_tutorial(&mut self) {
        self.active_tutorial = None;
        self.current_step_index = 0;
    }

    /// Check if a tutorial is active
    pub fn is_active(&self) -> bool {
        self.active_tutorial.is_some()
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f32 {
        if let Some(tutorial) = self.get_active_tutorial() {
            if tutorial.steps.is_empty() {
                1.0
            } else {
                self.current_step_index as f32 / tutorial.steps.len() as f32
            }
        } else {
            0.0
        }
    }

    /// Register a progress callback
    pub fn on_progress<F>(&mut self, callback: F)
    where
        F: Fn(&str, usize, usize) + Send + 'static,
    {
        self.progress_callbacks.push(Box::new(callback));
    }

    /// Register a completion callback
    pub fn on_complete<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + 'static,
    {
        self.completion_callbacks.push(Box::new(callback));
    }

    /// Mark a tutorial as completed
    pub fn mark_completed(&mut self, tutorial_id: impl Into<String>) {
        let id = tutorial_id.into();
        if let Some(tutorial) = self.tutorials.get_mut(&id) {
            tutorial.mark_completed();
        }
        if !self.completed_tutorials.contains(&id) {
            self.completed_tutorials.push(id);
        }
    }

    /// Check if a tutorial is completed
    pub fn is_completed(&self, tutorial_id: &str) -> bool {
        self.completed_tutorials.iter().any(|id| id == tutorial_id)
    }

    /// Get completion percentage for all tutorials
    pub fn get_overall_progress(&self) -> f32 {
        if self.tutorials.is_empty() {
            1.0
        } else {
            self.completed_tutorials.len() as f32 / self.tutorials.len() as f32
        }
    }

    /// Reset all tutorial progress
    pub fn reset_all_progress(&mut self) {
        self.completed_tutorials.clear();
        for tutorial in self.tutorials.values_mut() {
            tutorial.reset();
        }
    }

    /// Dismiss tutorials
    pub fn dismiss(&mut self) {
        self.dismissed = true;
        self.stop_tutorial();
    }

    /// Enable tutorials
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable tutorials
    pub fn disable(&mut self) {
        self.enabled = false;
        self.stop_tutorial();
    }

    /// Export tutorial progress
    pub fn export_progress(&self) -> TutorialProgressExport {
        TutorialProgressExport {
            completed: self.completed_tutorials.clone(),
            dismissed: self.dismissed,
        }
    }

    /// Import tutorial progress
    pub fn import_progress(&mut self, progress: &TutorialProgressExport) {
        self.completed_tutorials = progress.completed.clone();
        self.dismissed = progress.dismissed;

        // Update tutorial completion status
        for id in &self.completed_tutorials {
            if let Some(tutorial) = self.tutorials.get_mut(id) {
                tutorial.mark_completed();
            }
        }
    }
}

/// Export format for tutorial progress
#[derive(Debug, Clone)]
pub struct TutorialProgressExport {
    pub completed: Vec<String>,
    pub dismissed: bool,
}

/// Overlay renderer for tutorial tooltips
#[derive(Debug)]
pub struct TutorialOverlay {
    /// Whether overlay is visible
    pub visible: bool,
    /// Current step being displayed
    current_step: Option<TutorialStep>,
    /// Overlay opacity (0.0 to 1.0)
    opacity: f32,
    /// Highlight target element
    #[allow(dead_code)]
    highlight_target: Option<String>,
}

impl Default for TutorialOverlay {
    fn default() -> Self {
        Self {
            visible: false,
            current_step: None,
            opacity: 1.0,
            highlight_target: None,
        }
    }
}

impl TutorialOverlay {
    /// Create a new tutorial overlay
    pub fn new() -> Self {
        Self::default()
    }

    /// Show the overlay with a step
    pub fn show(&mut self, step: TutorialStep) {
        self.current_step = Some(step);
        self.visible = true;
        self.opacity = 1.0;
    }

    /// Hide the overlay
    pub fn hide(&mut self) {
        self.visible = false;
        self.current_step = None;
    }

    /// Set opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Get current step
    pub fn get_step(&self) -> Option<&TutorialStep> {
        self.current_step.as_ref()
    }

    /// Check if overlay is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get opacity
    pub fn opacity(&self) -> f32 {
        self.opacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_step_creation() {
        let step = TutorialStep::new("step1", "Title", "Description")
            .with_target("button1")
            .with_position(TooltipPosition::Top)
            .with_action(StepAction::ClickTarget)
            .with_tip("This is a tip");

        assert_eq!(step.id, "step1");
        assert_eq!(step.title, "Title");
        assert_eq!(step.target, Some("button1".to_string()));
        assert_eq!(step.position, TooltipPosition::Top);
        assert_eq!(step.action, StepAction::ClickTarget);
        assert_eq!(step.tips.len(), 1);
    }

    #[test]
    fn test_tooltip_position_opposite() {
        assert_eq!(TooltipPosition::Top.opposite(), TooltipPosition::Bottom);
        assert_eq!(TooltipPosition::Bottom.opposite(), TooltipPosition::Top);
        assert_eq!(TooltipPosition::Left.opposite(), TooltipPosition::Right);
        assert_eq!(TooltipPosition::Right.opposite(), TooltipPosition::Left);
    }

    #[test]
    fn test_tutorial_category() {
        assert_eq!(
            TutorialCategory::GettingStarted.display_name(),
            "Getting Started"
        );
        assert_eq!(TutorialCategory::Navigation.icon(), "🧭");
    }

    #[test]
    fn test_tutorial_creation() {
        let tutorial = Tutorial::new("test", "Test Tutorial", TutorialCategory::GettingStarted)
            .with_description("A test tutorial")
            .with_duration(10)
            .with_step(TutorialStep::new("s1", "Step 1", "Description"))
            .with_prerequisite("other_tutorial");

        assert_eq!(tutorial.id, "test");
        assert_eq!(tutorial.title, "Test Tutorial");
        assert_eq!(tutorial.step_count(), 1);
        assert_eq!(tutorial.estimated_duration, 10);
        assert_eq!(tutorial.prerequisites.len(), 1);
    }

    #[test]
    fn test_tutorial_completion() {
        let mut tutorial = Tutorial::new("test", "Test", TutorialCategory::GettingStarted);
        assert!(!tutorial.completed);

        tutorial.mark_completed();
        assert!(tutorial.completed);
        assert!(tutorial.completed_at.is_some());

        tutorial.reset();
        assert!(!tutorial.completed);
        assert!(tutorial.completed_at.is_none());
    }

    #[test]
    fn test_tutorial_manager_registration() {
        let mut manager = TutorialManager::new();

        let tutorial = Tutorial::new("custom", "Custom", TutorialCategory::Advanced);
        manager.register_tutorial(tutorial);

        assert!(manager.get_tutorial("custom").is_some());
        assert_eq!(manager.get_all_tutorials().len(), 5); // 4 default + 1 custom
    }

    #[test]
    fn test_tutorial_manager_start() {
        let mut manager = TutorialManager::new();

        assert!(manager.start_tutorial("getting_started").is_ok());
        assert!(manager.is_active());
        assert!(manager.get_active_tutorial().is_some());
        assert!(manager.get_current_step().is_some());
    }

    #[test]
    fn test_tutorial_manager_prerequisites() {
        let mut manager = TutorialManager::new();

        // Navigation requires getting_started
        assert!(manager.start_tutorial("navigation").is_err());

        // Complete getting_started first
        manager.mark_completed("getting_started");
        assert!(manager.start_tutorial("navigation").is_ok());
    }

    #[test]
    fn test_tutorial_progression() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        let initial_step = manager.current_step_index;
        assert!(manager.next_step());
        assert_eq!(manager.current_step_index, initial_step + 1);

        assert!(manager.previous_step());
        assert_eq!(manager.current_step_index, initial_step);
    }

    #[test]
    fn test_tutorial_completion_flow() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let mut manager = TutorialManager::new();

        // Track completion using Arc<AtomicBool> for thread-safe interior mutability
        let completed: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let completed_clone = completed.clone();

        manager.on_complete(move |_id| {
            completed_clone.store(true, Ordering::SeqCst);
        });

        manager.start_tutorial("getting_started").unwrap();

        // Complete all steps
        while manager.next_step() {}

        assert!(completed.load(Ordering::SeqCst));
        assert!(manager.is_completed("getting_started"));
        assert!(!manager.is_active());
    }

    #[test]
    fn test_tutorial_progress_callback() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let mut manager = TutorialManager::new();

        // Track progress updates using thread-safe counter
        let update_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let update_count_clone = update_count.clone();

        manager.on_progress(move |_id, _current, _total| {
            update_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        manager.start_tutorial("getting_started").unwrap();
        manager.next_step();

        assert!(update_count.load(Ordering::SeqCst) > 0);
    }

    #[test]
    fn test_tutorial_skip() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        let initial_index = manager.current_step_index;
        assert!(manager.skip_step());
        assert_eq!(manager.current_step_index, initial_index + 1);
    }

    #[test]
    fn test_tutorial_stop() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        assert!(manager.is_active());
        manager.stop_tutorial();
        assert!(!manager.is_active());
    }

    #[test]
    fn test_tutorial_progress_calculation() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        // Getting started has 5 steps
        assert_eq!(manager.get_progress(), 0.0);

        manager.next_step();
        assert_eq!(manager.get_progress(), 0.2);

        manager.next_step();
        assert_eq!(manager.get_progress(), 0.4);
    }

    #[test]
    fn test_tutorial_overall_progress() {
        let mut manager = TutorialManager::new();

        let initial_progress = manager.get_overall_progress();
        assert_eq!(initial_progress, 0.0);

        manager.mark_completed("getting_started");
        let progress = manager.get_overall_progress();
        assert!(progress > 0.0);
    }

    #[test]
    fn test_tutorial_reset() {
        let mut manager = TutorialManager::new();

        manager.mark_completed("getting_started");
        assert!(manager.is_completed("getting_started"));

        manager.reset_all_progress();
        assert!(!manager.is_completed("getting_started"));
    }

    #[test]
    fn test_tutorial_dismiss() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        manager.dismiss();
        assert!(manager.dismissed);
        assert!(!manager.is_active());
    }

    #[test]
    fn test_tutorial_enable_disable() {
        let mut manager = TutorialManager::new();
        manager.start_tutorial("getting_started").unwrap();

        manager.disable();
        assert!(!manager.enabled);
        assert!(!manager.is_active());

        manager.enable();
        assert!(manager.enabled);
    }

    #[test]
    fn test_tutorial_export_import() {
        let mut manager = TutorialManager::new();
        manager.mark_completed("getting_started");
        manager.dismiss();

        let export = manager.export_progress();
        assert!(export.completed.contains(&"getting_started".to_string()));
        assert!(export.dismissed);

        let mut new_manager = TutorialManager::new();
        new_manager.import_progress(&export);
        assert!(new_manager.is_completed("getting_started"));
        assert!(new_manager.dismissed);
    }

    #[test]
    fn test_tutorial_overlay() {
        let mut overlay = TutorialOverlay::new();
        assert!(!overlay.is_visible());

        let step = TutorialStep::new("s1", "Title", "Description");
        overlay.show(step);

        assert!(overlay.is_visible());
        assert!(overlay.get_step().is_some());

        overlay.hide();
        assert!(!overlay.is_visible());
    }

    #[test]
    fn test_tutorial_available_tutorials() {
        let mut manager = TutorialManager::new();

        // Initially, only getting_started should be available
        let available = manager.get_available_tutorials();
        assert!(available.iter().any(|t| t.id == "getting_started"));
        assert!(!available.iter().any(|t| t.id == "navigation"));

        // After completing getting_started, navigation becomes available
        manager.mark_completed("getting_started");
        let available = manager.get_available_tutorials();
        assert!(available.iter().any(|t| t.id == "navigation"));
    }
}
