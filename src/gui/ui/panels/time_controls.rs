//! Time Controls Panel - GUI Panel for Simulation Time Control
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 5 Week 10:
//! "Time controls panel - Play/pause simulation, Time rate slider (0.1x to 1000x), Focus dilation toggle, Timeline scrubber"
//!
//! This module provides:
//! - Play/pause controls
//! - Time rate adjustment
//! - Timeline navigation
//! - Step-through mode
//! - Focus-based time dilation controls

use crate::entity_layer7::layer7::EntityId;
use crate::gui::time_controller::{SimulationTime, TimeController};

/// Time controls panel for EGUI integration
pub struct TimeControlsPanel {
    /// Reference to time controller
    time_controller: TimeController,
    /// Show the panel
    pub visible: bool,
    /// Panel position
    pub position: [f32; 2],
    /// Panel size
    pub size: [f32; 2],
    /// Current time rate slider value
    rate_slider_value: f32,
    /// Timeline scrubber value (0.0 to 1.0)
    timeline_scrubber: f32,
    /// Show advanced controls
    show_advanced: bool,
    /// Step count for step-through
    step_count: usize,
    /// Enable focus dilation
    focus_dilation_enabled: bool,
    /// Selected entity for focus dilation
    focus_entity: Option<EntityId>,
    /// Focus dilation factor
    focus_factor: f32,
    /// Custom rate presets
    rate_presets: Vec<f32>,
    /// Selected preset index
    selected_preset: Option<usize>,
}

impl TimeControlsPanel {
    /// Create a new time controls panel
    pub fn new(time_controller: TimeController) -> Self {
        let current_rate = time_controller.temporal_rate as f32;

        TimeControlsPanel {
            time_controller,
            visible: true,
            position: [10.0, 10.0],
            size: [300.0, 400.0],
            rate_slider_value: current_rate.log10(),
            timeline_scrubber: 0.0,
            show_advanced: false,
            step_count: 1,
            focus_dilation_enabled: false,
            focus_entity: None,
            focus_factor: 1.0,
            rate_presets: vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 50.0, 100.0, 500.0, 1000.0],
            selected_preset: None,
        }
    }

    /// Show the panel (EGUI integration)
    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.visible {
            return;
        }

        let panel = egui::Window::new("Time Controls")
            .default_pos(self.position)
            .default_size(self.size)
            .collapsible(true)
            .resizable(true);

        panel.show(ctx, |ui| {
            self.render_contents(ui);
        });
    }

    /// Render the panel contents
    fn render_contents(&mut self, ui: &mut egui::Ui) {
        // Main controls section
        ui.heading("Playback");
        ui.separator();

        // Play/Pause button
        self.render_play_pause_controls(ui);

        ui.add_space(10.0);

        // Time rate slider
        self.render_rate_controls(ui);

        ui.add_space(10.0);

        // Timeline scrubber
        self.render_timeline_controls(ui);

        ui.add_space(10.0);

        // Step controls
        self.render_step_controls(ui);

        ui.add_space(10.0);

        // Advanced controls toggle
        ui.collapsing("Advanced", |ui| {
            self.render_advanced_controls(ui);
        });

        ui.add_space(10.0);

        // Focus dilation controls
        ui.collapsing("Focus Dilation", |ui| {
            self.render_focus_dilation_controls(ui);
        });

        ui.add_space(10.0);

        // Time display
        self.render_time_display(ui);
    }

    /// Render play/pause controls
    fn render_play_pause_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Play/Pause button
            let button_text = if self.time_controller.is_paused() {
                "▶ Play"
            } else {
                "⏸ Pause"
            };

            if ui.button(button_text).clicked() {
                self.time_controller.toggle_pause();
            }

            // Stop button
            if ui.button("⏹ Stop").clicked() {
                self.time_controller.pause();
                self.time_controller.reset();
                self.timeline_scrubber = 0.0;
            }

            // Reset button
            if ui.button("↺ Reset").clicked() {
                self.time_controller.reset();
                self.timeline_scrubber = 0.0;
            }
        });
    }

    /// Render time rate controls
    fn render_rate_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Time Rate:");

        // Current rate display
        let current_rate = self.time_controller.temporal_rate;
        ui.label(format!("{:.1}x", current_rate));

        // Rate slider (logarithmic scale: 0.1 to 1000)
        let min_rate = self.time_controller.min_rate.max(0.1);
        let max_rate = self.time_controller.max_rate.min(1000.0);

        let min_log = min_rate.log10() as f32;
        let max_log = max_rate.log10() as f32;

        let mut slider_value = current_rate.log10() as f32;
        let slider_response = ui.add(
            egui::Slider::new(&mut slider_value, min_log..=max_log)
                .show_value(false)
                .text("Speed"),
        );

        if slider_response.changed() {
            let new_rate = 10f32.powf(slider_value) as f64;
            self.time_controller.set_rate(new_rate);
        }

        // Rate presets
        ui.horizontal(|ui| {
            ui.label("Presets:");
            for (idx, &preset) in self.rate_presets.iter().enumerate() {
                let text = if preset >= 1.0 {
                    format!("{:.0}x", preset)
                } else {
                    format!("{:.1}x", preset)
                };

                let is_selected = self.selected_preset == Some(idx);
                let button = if is_selected {
                    ui.button(egui::RichText::new(text).strong())
                } else {
                    ui.button(text)
                };

                if button.clicked() {
                    self.time_controller.set_rate(preset as f64);
                    self.selected_preset = Some(idx);
                }
            }
        });
    }

    /// Render timeline controls
    fn render_timeline_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Timeline:");

        let current = self.time_controller.current_time();
        let max_time = self.time_controller.timeline.max_time.total_seconds;

        // Calculate progress
        let progress = if max_time > 0 {
            current.total_seconds as f32 / max_time as f32
        } else {
            0.0
        };

        // Timeline scrubber
        let mut scrubber_value = progress;
        let slider_response = ui.add(
            egui::Slider::new(&mut scrubber_value, 0.0..=1.0)
                .show_value(false)
                .text("Position"),
        );

        if slider_response.changed() {
            // Jump to position
            let target_seconds = (scrubber_value * max_time as f32) as u64;
            let target_time = SimulationTime {
                total_seconds: target_seconds,
                step_number: current.step_number,
            };
            self.time_controller.navigate_timeline(target_time);
        }

        // Navigation buttons
        ui.horizontal(|ui| {
            if ui.button("⏮").clicked() && self.time_controller.timeline.can_navigate_back() {
                self.time_controller.navigate_back();
            }

            if ui.button("⏭").clicked() && self.time_controller.timeline.can_navigate_forward() {
                self.time_controller.navigate_forward();
            }

            ui.label(format!("{:.1}%", progress * 100.0));
        });
    }

    /// Render step controls
    fn render_step_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Step Through:");

        ui.horizontal(|ui| {
            // Step count input
            ui.add(
                egui::DragValue::new(&mut self.step_count)
                    .speed(1)
                    .clamp_range(1..=100),
            );
            ui.label("steps");

            // Step forward button
            if ui.button("Step Forward").clicked() {
                self.time_controller.step_forward_multiple(self.step_count);
            }

            // Step backward button
            if ui.button("Step Backward").clicked()
                && self.time_controller.timeline.can_navigate_back()
            {
                // Navigate back multiple times
                for _ in 0..self.step_count {
                    if !self.time_controller.timeline.can_navigate_back() {
                        break;
                    }
                    self.time_controller.navigate_back();
                }
            }
        });

        // Show step-through status
        if self.time_controller.step_through {
            ui.label(format!(
                "Stepping: {} remaining",
                self.time_controller.steps_remaining
            ));
        }
    }

    /// Render advanced controls
    fn render_advanced_controls(&mut self, ui: &mut egui::Ui) {
        // Min/Max rate controls
        ui.horizontal(|ui| {
            ui.label("Min Rate:");
            let mut min_rate = self.time_controller.min_rate as f32;
            if ui
                .add(
                    egui::DragValue::new(&mut min_rate)
                        .speed(0.1)
                        .clamp_range(0.01..=10.0),
                )
                .changed()
            {
                self.time_controller.min_rate = min_rate as f64;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Max Rate:");
            let mut max_rate = self.time_controller.max_rate as f32;
            if ui
                .add(
                    egui::DragValue::new(&mut max_rate)
                        .speed(1.0)
                        .clamp_range(10.0..=10000.0),
                )
                .changed()
            {
                self.time_controller.max_rate = max_rate as f64;
            }
        });

        // History size
        ui.horizontal(|ui| {
            ui.label("Timeline History:");
            let mut history_size = self.time_controller.timeline.max_history_size;
            if ui
                .add(
                    egui::DragValue::new(&mut history_size)
                        .speed(10)
                        .clamp_range(10..=10000),
                )
                .changed()
            {
                self.time_controller
                    .timeline
                    .set_max_history_size(history_size);
            }
        });

        // Rate presets editor
        ui.collapsing("Edit Presets", |ui| {
            let presets_to_edit = self.rate_presets.clone();
            for (idx, preset) in presets_to_edit.iter().enumerate() {
                let mut value = *preset;
                ui.horizontal(|ui| {
                    ui.label(format!("Preset {}:", idx + 1));
                    if ui
                        .add(
                            egui::DragValue::new(&mut value)
                                .speed(0.1)
                                .clamp_range(0.01..=10000.0),
                        )
                        .changed()
                    {
                        if let Some(p) = self.rate_presets.get_mut(idx) {
                            *p = value;
                        }
                    }
                });
            }
        });
    }

    /// Render focus dilation controls
    fn render_focus_dilation_controls(&mut self, ui: &mut egui::Ui) {
        // Enable/disable focus dilation
        ui.checkbox(&mut self.focus_dilation_enabled, "Enable Focus Dilation");

        if self.focus_dilation_enabled != self.time_controller.focus_dilation.enabled {
            self.time_controller
                .set_focus_dilation_enabled(self.focus_dilation_enabled);
        }

        if self.focus_dilation_enabled {
            // Focus entity selection
            ui.label("Focus Entity:");
            if let Some(ref entity_id) = self.focus_entity {
                ui.label(format!("Selected: {}", entity_id));
            } else {
                ui.label("None selected");
            }

            // Dilation factor
            ui.horizontal(|ui| {
                ui.label("Dilation Factor:");
                ui.add(
                    egui::Slider::new(&mut self.focus_factor, 0.1..=5.0)
                        .logarithmic(true)
                        .text("x"),
                );
            });

            // Set focus button
            if ui.button("Set Focus").clicked() {
                if let Some(ref entity_id) = self.focus_entity {
                    self.time_controller
                        .set_focus_dilation(entity_id.clone(), self.focus_factor as f64);
                }
            }

            // Clear focus button
            if ui.button("Clear Focus").clicked() {
                self.time_controller.clear_focus_dilation();
                self.focus_entity = None;
            }
        }
    }

    /// Render time display
    fn render_time_display(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        let current = self.time_controller.current_time();

        ui.horizontal(|ui| {
            ui.label("Current Time:");
            ui.label(egui::RichText::new(current.format()).strong().size(14.0));
        });

        ui.horizontal(|ui| {
            ui.label("Step:");
            ui.label(current.step_number.to_string());
        });

        ui.horizontal(|ui| {
            ui.label("Status:");
            let status = if self.time_controller.is_paused() {
                "⏸ Paused"
            } else if self.time_controller.step_through {
                "🚶 Stepping"
            } else {
                "▶ Running"
            };
            ui.label(status);
        });

        ui.horizontal(|ui| {
            ui.label("Rate:");
            ui.label(format!("{:.2}x", self.time_controller.temporal_rate));
        });
    }

    /// Update the panel (call each frame)
    pub fn update(&mut self, delta_time: f64) {
        self.time_controller.update(delta_time);

        // Update timeline scrubber
        let current = self.time_controller.current_time();
        let max_time = self.time_controller.timeline.max_time.total_seconds;

        if max_time > 0 {
            self.timeline_scrubber = current.total_seconds as f32 / max_time as f32;
        }
    }

    /// Get the time controller
    pub fn time_controller(&self) -> &TimeController {
        &self.time_controller
    }

    /// Get mutable time controller
    pub fn time_controller_mut(&mut self) -> &mut TimeController {
        &mut self.time_controller
    }

    /// Set the focus entity
    pub fn set_focus_entity(&mut self, entity_id: Option<EntityId>) {
        self.focus_entity = entity_id.clone();

        if self.focus_dilation_enabled {
            if let Some(id) = entity_id {
                self.time_controller
                    .set_focus_dilation(id, self.focus_factor as f64);
            } else {
                self.time_controller.clear_focus_dilation();
            }
        }
    }

    /// Toggle panel visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Set panel position
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = [x, y];
    }

    /// Set panel size
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size = [width, height];
    }

    /// Check if currently paused
    pub fn is_paused(&self) -> bool {
        self.time_controller.is_paused()
    }

    /// Get current simulation time
    pub fn current_time(&self) -> SimulationTime {
        self.time_controller.current_time()
    }
}

impl Default for TimeControlsPanel {
    fn default() -> Self {
        Self::new(TimeController::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_controls_panel_new() {
        let controller = TimeController::new();
        let panel = TimeControlsPanel::new(controller);

        assert!(panel.visible);
        assert!(!panel.show_advanced);
        assert_eq!(panel.step_count, 1);
        assert!(!panel.focus_dilation_enabled);
    }

    #[test]
    fn test_time_controls_panel_toggle_visibility() {
        let controller = TimeController::new();
        let mut panel = TimeControlsPanel::new(controller);

        assert!(panel.visible);
        panel.toggle_visibility();
        assert!(!panel.visible);
        panel.toggle_visibility();
        assert!(panel.visible);
    }

    #[test]
    fn test_time_controls_panel_set_position() {
        let controller = TimeController::new();
        let mut panel = TimeControlsPanel::new(controller);

        panel.set_position(100.0, 200.0);
        assert_eq!(panel.position, [100.0, 200.0]);
    }

    #[test]
    fn test_time_controls_panel_set_size() {
        let controller = TimeController::new();
        let mut panel = TimeControlsPanel::new(controller);

        panel.set_size(400.0, 300.0);
        assert_eq!(panel.size, [400.0, 300.0]);
    }

    #[test]
    fn test_time_controls_panel_current_time() {
        let controller = TimeController::new();
        let panel = TimeControlsPanel::new(controller);

        let time = panel.current_time();
        assert_eq!(time.total_seconds, 0);
        assert_eq!(time.step_number, 0);
    }

    #[test]
    fn test_time_controls_panel_is_paused() {
        let controller = TimeController::new();
        let panel = TimeControlsPanel::new(controller);

        assert!(!panel.is_paused());
    }

    #[test]
    fn test_time_controls_panel_rate_presets() {
        let controller = TimeController::new();
        let panel = TimeControlsPanel::new(controller);

        assert!(!panel.rate_presets.is_empty());
        assert!(panel.rate_presets.contains(&1.0));
        assert!(panel.rate_presets.contains(&10.0));
        assert!(panel.rate_presets.contains(&100.0));
    }

    #[test]
    fn test_time_controls_panel_set_focus_entity() {
        let controller = TimeController::new();
        let mut panel = TimeControlsPanel::new(controller);

        let entity_id = EntityId::new("1".to_string());
        panel.set_focus_entity(Some(entity_id.clone()));

        assert_eq!(panel.focus_entity, Some(entity_id));
    }

    #[test]
    fn test_time_controls_panel_update() {
        let controller = TimeController::new();
        let mut panel = TimeControlsPanel::new(controller);

        // Should not panic
        panel.update(0.016);
    }
}
