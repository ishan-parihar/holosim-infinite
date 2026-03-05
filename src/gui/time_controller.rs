//! Time Controller - Temporal rate control and time dilation
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 5:
//! "Time Controller:
//! - Temporal rate adjustment (0.1x to 1000x)
//! - Pause/resume/step-through
//! - Focus-based time dilation (where you look, time flows differently)
//! - Timeline navigation (past/present/future)"
//!
//! This module provides:
//! - Time rate control (0.1x to 1000x)
//! - Pause/resume functionality
//! - Focus-based time dilation
//! - Timeline navigation

use crate::entity_layer7::layer7::EntityId;
use crate::gui::GuiConfig;
use std::collections::HashMap;

/// Simulation time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimulationTime {
    /// Total simulation time in seconds
    pub total_seconds: u64,

    /// Current step number
    pub step_number: u64,
}

impl SimulationTime {
    /// Create a new simulation time
    pub fn new() -> Self {
        SimulationTime {
            total_seconds: 0,
            step_number: 0,
        }
    }

    /// Advance time by a number of seconds
    pub fn advance(&mut self, seconds: u64) {
        self.total_seconds += seconds;
        self.step_number += 1;
    }

    /// Get the time in a human-readable format
    pub fn format(&self) -> String {
        let days = self.total_seconds / 86400;
        let hours = (self.total_seconds % 86400) / 3600;
        let minutes = (self.total_seconds % 3600) / 60;
        let seconds = self.total_seconds % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Focus-based time dilation
#[derive(Debug, Clone, PartialEq)]
pub struct FocusDilation {
    /// Entity ID being focused on
    pub entity_id: Option<EntityId>,

    /// Dilation factor (1.0 = normal, < 1.0 = slower, > 1.0 = faster)
    pub dilation_factor: f64,

    /// Dilation radius in meters
    pub dilation_radius: f64,

    /// Enable focus-based dilation
    pub enabled: bool,
}

impl Default for FocusDilation {
    fn default() -> Self {
        FocusDilation {
            entity_id: None,
            dilation_factor: 1.0,
            dilation_radius: 1.0e6, // 1000 km default
            enabled: false,
        }
    }
}

impl FocusDilation {
    /// Create a new focus dilation
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the focused entity
    pub fn set_focus(&mut self, entity_id: EntityId, factor: f64) {
        self.entity_id = Some(entity_id);
        self.dilation_factor = factor;
    }

    /// Clear the focus
    pub fn clear_focus(&mut self) {
        self.entity_id = None;
        self.dilation_factor = 1.0;
    }

    /// Enable or disable focus-based dilation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Calculate the effective time dilation for an entity
    pub fn calculate_dilation(&self, entity_id: EntityId) -> f64 {
        if !self.enabled {
            return 1.0;
        }

        if let Some(ref focused_id) = self.entity_id {
            if focused_id == &entity_id {
                return self.dilation_factor;
            }
        }

        1.0
    }

    /// Set the dilation radius
    pub fn set_radius(&mut self, radius: f64) {
        self.dilation_radius = radius;
    }
}

/// Timeline state for navigation
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineState {
    /// Current time
    pub current_time: SimulationTime,

    /// Minimum time (start of timeline)
    pub min_time: SimulationTime,

    /// Maximum time (end of timeline)
    pub max_time: SimulationTime,

    /// Timeline history (for navigation back)
    pub history: Vec<SimulationTime>,

    /// Timeline future (for navigation forward)
    pub future: Vec<SimulationTime>,

    /// Maximum history size
    pub max_history_size: usize,
}

impl Default for TimelineState {
    fn default() -> Self {
        TimelineState {
            current_time: SimulationTime::new(),
            min_time: SimulationTime::new(),
            max_time: SimulationTime::new(),
            history: Vec::new(),
            future: Vec::new(),
            max_history_size: 1000,
        }
    }
}

impl TimelineState {
    /// Create a new timeline state
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance to the next time
    pub fn advance(&mut self, time: SimulationTime) {
        // Save current time to history
        self.history.push(self.current_time);

        // Trim history if too large
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }

        // Clear future (we've diverged)
        self.future.clear();

        // Update current time
        self.current_time = time;

        // Update max time
        if time > self.max_time {
            self.max_time = time;
        }
    }

    /// Navigate back in time
    pub fn navigate_back(&mut self) -> Option<SimulationTime> {
        if let Some(prev_time) = self.history.pop() {
            // Save current time to future
            self.future.push(self.current_time);
            self.current_time = prev_time;
            Some(self.current_time)
        } else {
            None
        }
    }

    /// Navigate forward in time
    pub fn navigate_forward(&mut self) -> Option<SimulationTime> {
        if let Some(next_time) = self.future.pop() {
            // Save current time to history
            self.history.push(self.current_time);
            self.current_time = next_time;
            Some(self.current_time)
        } else {
            None
        }
    }

    /// Jump to a specific time
    pub fn jump_to(&mut self, time: SimulationTime) -> bool {
        if time >= self.min_time && time <= self.max_time {
            // Save current time to history
            self.history.push(self.current_time);
            self.current_time = time;
            self.future.clear();
            true
        } else {
            false
        }
    }

    /// Check if we can navigate back
    pub fn can_navigate_back(&self) -> bool {
        !self.history.is_empty()
    }

    /// Check if we can navigate forward
    pub fn can_navigate_forward(&self) -> bool {
        !self.future.is_empty()
    }

    /// Get the timeline progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.max_time.total_seconds == 0 {
            0.0
        } else {
            self.current_time.total_seconds as f64 / self.max_time.total_seconds as f64
        }
    }

    /// Set the maximum history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        // Trim history if necessary
        while self.history.len() > self.max_history_size {
            self.history.remove(0);
        }
    }
}

/// Time controller - manages temporal rate and time dilation
#[derive(Debug, Clone)]
pub struct TimeController {
    /// Temporal rate (1.0 = normal speed)
    pub temporal_rate: f64,

    /// Minimum temporal rate
    pub min_rate: f64,

    /// Maximum temporal rate
    pub max_rate: f64,

    /// Paused state
    pub paused: bool,

    /// Focus-based time dilation
    pub focus_dilation: FocusDilation,

    /// Timeline state
    pub timeline: TimelineState,

    /// Per-entity time dilation factors
    pub entity_dilation: HashMap<EntityId, f64>,

    /// Step-through mode (advance one step at a time)
    pub step_through: bool,

    /// Steps remaining in step-through mode
    pub steps_remaining: usize,
}

impl Default for TimeController {
    fn default() -> Self {
        TimeController {
            temporal_rate: 1.0,
            min_rate: 0.1,
            max_rate: 1000.0,
            paused: false,
            focus_dilation: FocusDilation::default(),
            timeline: TimelineState::default(),
            entity_dilation: HashMap::new(),
            step_through: false,
            steps_remaining: 0,
        }
    }
}

impl TimeController {
    /// Create a new time controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the temporal rate
    pub fn set_rate(&mut self, rate: f64) {
        self.temporal_rate = rate.max(self.min_rate).min(self.max_rate);
    }

    /// Get the current temporal rate
    pub fn get_rate(&self) -> f64 {
        self.temporal_rate
    }

    /// Pause the simulation
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume the simulation
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Step forward one step
    pub fn step_forward(&mut self) {
        self.steps_remaining = 1;
        self.step_through = true;
        self.paused = true;
    }

    /// Step forward multiple steps
    pub fn step_forward_multiple(&mut self, steps: usize) {
        self.steps_remaining = steps;
        self.step_through = true;
        self.paused = true;
    }

    /// Set focus-based time dilation
    pub fn set_focus_dilation(&mut self, entity_id: EntityId, dilation: f64) {
        self.focus_dilation.set_focus(entity_id, dilation);
    }

    /// Clear focus-based time dilation
    pub fn clear_focus_dilation(&mut self) {
        self.focus_dilation.clear_focus();
    }

    /// Enable or disable focus-based dilation
    pub fn set_focus_dilation_enabled(&mut self, enabled: bool) {
        self.focus_dilation.set_enabled(enabled);
    }

    /// Set entity-specific time dilation
    pub fn set_entity_dilation(&mut self, entity_id: EntityId, dilation: f64) {
        self.entity_dilation.insert(entity_id, dilation);
    }

    /// Remove entity-specific time dilation
    pub fn remove_entity_dilation(&mut self, entity_id: &EntityId) {
        self.entity_dilation.remove(entity_id);
    }

    /// Calculate the effective time rate for an entity
    pub fn calculate_effective_rate(&self, entity_id: EntityId) -> f64 {
        if self.paused && self.steps_remaining == 0 {
            return 0.0;
        }

        let mut rate = self.temporal_rate;

        // Apply focus-based dilation
        rate *= self.focus_dilation.calculate_dilation(entity_id.clone());

        // Apply entity-specific dilation
        if let Some(&dilation) = self.entity_dilation.get(&entity_id) {
            rate *= dilation;
        }

        rate
    }

    /// Navigate the timeline
    pub fn navigate_timeline(&mut self, time: SimulationTime) -> bool {
        self.timeline.jump_to(time)
    }

    /// Navigate back in timeline
    pub fn navigate_back(&mut self) -> Option<SimulationTime> {
        self.timeline.navigate_back()
    }

    /// Navigate forward in timeline
    pub fn navigate_forward(&mut self) -> Option<SimulationTime> {
        self.timeline.navigate_forward()
    }

    /// Get the current simulation time
    pub fn current_time(&self) -> SimulationTime {
        self.timeline.current_time
    }

    /// Update the controller
    pub fn update(&mut self, _delta_time: f64) {
        // Handle step-through mode
        if self.step_through && self.steps_remaining > 0 {
            self.steps_remaining -= 1;
            if self.steps_remaining == 0 {
                self.step_through = false;
            }
        }
    }

    /// Advance simulation time
    pub fn advance_time(&mut self, base_delta: f64) {
        if !self.paused || self.steps_remaining > 0 {
            let effective_delta = base_delta * self.temporal_rate;
            let seconds = effective_delta.floor() as u64;
            if seconds > 0 {
                self.timeline.advance(SimulationTime {
                    total_seconds: self.timeline.current_time.total_seconds + seconds,
                    step_number: self.timeline.current_time.step_number + 1,
                });
            }
        }
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.temporal_rate = 1.0;
        self.paused = false;
        self.focus_dilation = FocusDilation::default();
        self.timeline = TimelineState::default();
        self.entity_dilation.clear();
        self.step_through = false;
        self.steps_remaining = 0;
    }
}

/// Builder for TimeController
#[derive(Debug, Clone, Default)]
pub struct TimeControllerBuilder {
    min_rate: f64,
    max_rate: f64,
    initial_rate: f64,
    enable_focus_dilation: bool,
}

impl TimeControllerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the rate range
    pub fn with_rate_range(mut self, min: f64, max: f64) -> Self {
        self.min_rate = min;
        self.max_rate = max;
        self
    }

    /// Set the initial rate
    pub fn with_initial_rate(mut self, rate: f64) -> Self {
        self.initial_rate = rate;
        self
    }

    /// Enable focus-based dilation
    pub fn with_focus_dilation(mut self, enable: bool) -> Self {
        self.enable_focus_dilation = enable;
        self
    }

    /// Build the time controller
    pub fn build(self) -> TimeController {
        let mut controller = TimeController::new();
        controller.min_rate = self.min_rate;
        controller.max_rate = self.max_rate;
        controller.temporal_rate = self.initial_rate;
        controller
            .focus_dilation
            .set_enabled(self.enable_focus_dilation);
        controller
    }
}

impl TimeController {
    /// Create a time controller from configuration
    pub fn from_config(config: &GuiConfig) -> Self {
        TimeControllerBuilder::new()
            .with_rate_range(config.min_time_rate, config.max_time_rate)
            .with_initial_rate(config.initial_time_rate)
            .with_focus_dilation(config.enable_focus_dilation)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_time_default() {
        let time = SimulationTime::new();
        assert_eq!(time.total_seconds, 0);
        assert_eq!(time.step_number, 0);
    }

    #[test]
    fn test_simulation_time_advance() {
        let mut time = SimulationTime::new();
        time.advance(10);
        assert_eq!(time.total_seconds, 10);
        assert_eq!(time.step_number, 1);
    }

    #[test]
    fn test_simulation_time_format() {
        let mut time = SimulationTime::new();
        time.total_seconds = 3661; // 1h 1m 1s
        assert_eq!(time.format(), "1h 1m 1s");

        time.total_seconds = 86461; // 1d 0h 1m 1s
        assert_eq!(time.format(), "1d 0h 1m 1s");

        time.total_seconds = 61; // 1m 1s
        assert_eq!(time.format(), "1m 1s");

        time.total_seconds = 1; // 1s
        assert_eq!(time.format(), "1s");
    }

    #[test]
    fn test_focus_dilation_default() {
        let dilation = FocusDilation::new();
        assert!(dilation.entity_id.is_none());
        assert_eq!(dilation.dilation_factor, 1.0);
        assert_eq!(dilation.dilation_radius, 1.0e6);
        assert!(!dilation.enabled);
    }

    #[test]
    fn test_focus_dilation_set_focus() {
        let mut dilation = FocusDilation::new();
        dilation.set_focus(EntityId::new("1".to_string()), 2.0);

        assert_eq!(dilation.entity_id, Some(EntityId::new("1".to_string())));
        assert_eq!(dilation.dilation_factor, 2.0);
    }

    #[test]
    fn test_focus_dilation_calculate() {
        let mut dilation = FocusDilation::new();
        dilation.set_enabled(true);
        dilation.set_focus(EntityId::new("1".to_string()), 2.0);

        assert_eq!(
            dilation.calculate_dilation(EntityId::new("1".to_string())),
            2.0
        );
        assert_eq!(
            dilation.calculate_dilation(EntityId::new("2".to_string())),
            1.0
        );

        dilation.set_enabled(false);
        assert_eq!(
            dilation.calculate_dilation(EntityId::new("1".to_string())),
            1.0
        );
    }

    #[test]
    fn test_timeline_state_default() {
        let timeline = TimelineState::new();
        assert_eq!(timeline.current_time.total_seconds, 0);
        assert!(timeline.history.is_empty());
        assert!(timeline.future.is_empty());
    }

    #[test]
    fn test_timeline_state_advance() {
        let mut timeline = TimelineState::new();
        let time1 = SimulationTime {
            total_seconds: 10,
            step_number: 1,
        };
        let time2 = SimulationTime {
            total_seconds: 20,
            step_number: 2,
        };

        timeline.advance(time1);
        assert_eq!(timeline.current_time, time1);
        assert_eq!(timeline.history.len(), 1);

        timeline.advance(time2);
        assert_eq!(timeline.current_time, time2);
        assert_eq!(timeline.history.len(), 2);
    }

    #[test]
    fn test_timeline_state_navigate_back() {
        let mut timeline = TimelineState::new();
        let time1 = SimulationTime {
            total_seconds: 10,
            step_number: 1,
        };
        let time2 = SimulationTime {
            total_seconds: 20,
            step_number: 2,
        };

        timeline.advance(time1);
        timeline.advance(time2);

        let prev = timeline.navigate_back();
        assert_eq!(prev, Some(time1));
        assert_eq!(timeline.current_time, time1);
        assert_eq!(timeline.future.len(), 1);
    }

    #[test]
    fn test_timeline_state_navigate_forward() {
        let mut timeline = TimelineState::new();
        let time1 = SimulationTime {
            total_seconds: 10,
            step_number: 1,
        };
        let time2 = SimulationTime {
            total_seconds: 20,
            step_number: 2,
        };

        timeline.advance(time1);
        timeline.advance(time2);
        timeline.navigate_back();

        let next = timeline.navigate_forward();
        assert_eq!(next, Some(time2));
        assert_eq!(timeline.current_time, time2);
    }

    #[test]
    fn test_time_controller_default() {
        let controller = TimeController::new();
        assert_eq!(controller.temporal_rate, 1.0);
        assert_eq!(controller.min_rate, 0.1);
        assert_eq!(controller.max_rate, 1000.0);
        assert!(!controller.paused);
    }

    #[test]
    fn test_time_controller_set_rate() {
        let mut controller = TimeController::new();
        controller.set_rate(500.0);
        assert_eq!(controller.temporal_rate, 500.0);

        controller.set_rate(0.05); // Below minimum
        assert_eq!(controller.temporal_rate, 0.1);

        controller.set_rate(2000.0); // Above maximum
        assert_eq!(controller.temporal_rate, 1000.0);
    }

    #[test]
    fn test_time_controller_pause_resume() {
        let mut controller = TimeController::new();
        assert!(!controller.is_paused());

        controller.pause();
        assert!(controller.is_paused());

        controller.resume();
        assert!(!controller.is_paused());

        controller.toggle_pause();
        assert!(controller.is_paused());

        controller.toggle_pause();
        assert!(!controller.is_paused());
    }

    #[test]
    fn test_time_controller_step_forward() {
        let mut controller = TimeController::new();
        controller.step_forward();

        assert!(controller.is_paused());
        assert!(controller.step_through);
        assert_eq!(controller.steps_remaining, 1);
    }

    #[test]
    fn test_time_controller_step_forward_multiple() {
        let mut controller = TimeController::new();
        controller.step_forward_multiple(5);

        assert!(controller.is_paused());
        assert!(controller.step_through);
        assert_eq!(controller.steps_remaining, 5);
    }

    #[test]
    fn test_time_controller_focus_dilation() {
        let mut controller = TimeController::new();
        controller.set_focus_dilation_enabled(true);
        controller.set_focus_dilation(EntityId::new("1".to_string()), 2.0);

        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("1".to_string())),
            2.0
        );
        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("2".to_string())),
            1.0
        );

        controller.clear_focus_dilation();
        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("1".to_string())),
            1.0
        );
    }

    #[test]
    fn test_time_controller_entity_dilation() {
        let mut controller = TimeController::new();
        controller.set_entity_dilation(EntityId::new("1".to_string()), 3.0);

        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("1".to_string())),
            3.0
        );
        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("2".to_string())),
            1.0
        );

        controller.remove_entity_dilation(&EntityId::new("1".to_string()));
        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("1".to_string())),
            1.0
        );
    }

    #[test]
    fn test_time_controller_navigate_timeline() {
        let mut controller = TimeController::new();

        // Advance time to set max_time
        controller.advance_time(10.0);

        // Navigate to a time within the valid range
        let time = SimulationTime {
            total_seconds: 5,
            step_number: 1,
        };
        assert!(controller.navigate_timeline(time));
        assert_eq!(controller.current_time(), time);

        // Navigate to the current maximum time
        let max_time = SimulationTime {
            total_seconds: 10,
            step_number: 1,
        };
        assert!(controller.navigate_timeline(max_time));
        assert_eq!(controller.current_time(), max_time);
    }

    #[test]
    fn test_time_controller_advance_time() {
        let mut controller = TimeController::new();
        controller.advance_time(1.0);

        assert_eq!(controller.current_time().total_seconds, 1);
        assert_eq!(controller.current_time().step_number, 1);
    }

    #[test]
    fn test_time_controller_reset() {
        let mut controller = TimeController::new();
        controller.set_rate(500.0);
        controller.pause();
        controller.set_focus_dilation_enabled(true);
        controller.set_entity_dilation(EntityId::new("1".to_string()), 2.0);

        controller.reset();

        assert_eq!(controller.temporal_rate, 1.0);
        assert!(!controller.paused);
        assert!(!controller.focus_dilation.enabled);
        assert!(controller.entity_dilation.is_empty());
    }

    #[test]
    fn test_time_controller_from_config() {
        let config = GuiConfig::new()
            .with_time_rate_range(0.5, 500.0)
            .with_initial_rate(2.0)
            .with_focus_dilation(true);

        let controller = TimeController::from_config(&config);

        assert_eq!(controller.min_rate, 0.5);
        assert_eq!(controller.max_rate, 500.0);
        assert_eq!(controller.temporal_rate, 2.0);
        assert!(controller.focus_dilation.enabled);
    }

    #[test]
    fn test_time_controller_builder() {
        let controller = TimeControllerBuilder::new()
            .with_rate_range(0.5, 500.0)
            .with_initial_rate(2.0)
            .with_focus_dilation(true)
            .build();

        assert_eq!(controller.min_rate, 0.5);
        assert_eq!(controller.max_rate, 500.0);
        assert_eq!(controller.temporal_rate, 2.0);
        assert!(controller.focus_dilation.enabled);
    }

    #[test]
    fn test_time_controller_update_step_through() {
        let mut controller = TimeController::new();
        controller.step_forward_multiple(3);

        assert_eq!(controller.steps_remaining, 3);

        controller.update(0.1);
        assert_eq!(controller.steps_remaining, 2);

        controller.update(0.1);
        assert_eq!(controller.steps_remaining, 1);

        controller.update(0.1);
        assert_eq!(controller.steps_remaining, 0);
        assert!(!controller.step_through);
    }

    #[test]
    fn test_time_controller_combined_dilation() {
        let mut controller = TimeController::new();
        controller.set_rate(2.0);
        controller.set_focus_dilation_enabled(true);
        controller.set_focus_dilation(EntityId::new("1".to_string()), 3.0);
        controller.set_entity_dilation(EntityId::new("1".to_string()), 4.0);

        // Combined: 2.0 * 3.0 * 4.0 = 24.0
        assert_eq!(
            controller.calculate_effective_rate(EntityId::new("1".to_string())),
            24.0
        );
    }
}
