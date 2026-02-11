//! Collective Dynamics Dashboard
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 3 Week 7:
//! "Create resonance metrics, group size distribution, coherence indicator"
//!
//! This module provides an EGUI-based dashboard for visualizing collective dynamics
//! with resonance metrics, group distributions, and coherence indicators.

use crate::gui::visualization::collective_viz::{
    BehaviorPattern, CollectiveGroup, CollectiveLevel, CollectiveMetrics, CollectiveVisualizer,
    ResonanceType,
};
use egui::{Color32, Stroke, Ui, Vec2};
use std::collections::HashMap;

/// Collective dashboard configuration
pub struct CollectiveDashboard {
    /// Show dashboard
    pub show_dashboard: bool,
    /// Selected level
    pub selected_level: Option<CollectiveLevel>,
    /// Selected resonance type
    pub selected_resonance: Option<ResonanceType>,
    /// Show resonance field
    pub show_resonance_field: bool,
    /// Show group list
    pub show_group_list: bool,
    /// Show metrics
    pub show_metrics: bool,
    /// Show graphs
    pub show_graphs: bool,
    /// Graph time range
    pub graph_time_range: f64,
    /// Min resonance threshold
    pub min_resonance_threshold: f64,
    /// Show behavior patterns
    pub show_behavior_patterns: bool,
    /// Show heat map
    pub show_heat_map: bool,
    /// Heat map resolution
    pub heat_map_resolution: usize,
}

impl CollectiveDashboard {
    /// Create new collective dashboard
    pub fn new() -> Self {
        CollectiveDashboard {
            show_dashboard: true,
            selected_level: None,
            selected_resonance: None,
            show_resonance_field: true,
            show_group_list: true,
            show_metrics: true,
            show_graphs: true,
            graph_time_range: 60.0,
            min_resonance_threshold: 0.3,
            show_behavior_patterns: true,
            show_heat_map: true,
            heat_map_resolution: 32,
        }
    }

    /// Render the dashboard
    pub fn render(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        if !self.show_dashboard {
            return;
        }

        egui::Window::new("Collective Dynamics Dashboard")
            .resizable(true)
            .default_size(Vec2::new(400.0, 500.0))
            .show(ui.ctx(), |ui| {
                // Controls
                self.render_controls(ui);

                ui.separator();

                // Metrics summary
                self.render_metrics_summary(ui, visualizer);

                ui.separator();

                // Level distribution
                self.render_level_distribution(ui, visualizer);

                ui.separator();

                // Resonance distribution
                self.render_resonance_distribution(ui, visualizer);

                ui.separator();

                // Group list
                if self.show_group_list {
                    self.render_group_list(ui, visualizer);
                }

                ui.separator();

                // Graphs
                if self.show_graphs {
                    self.render_graphs(ui, visualizer);
                }
            });
    }

    /// Render controls
    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_resonance_field, "Resonance Field");
            ui.checkbox(&mut self.show_group_list, "Groups");
            ui.checkbox(&mut self.show_metrics, "Metrics");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_graphs, "Graphs");
            ui.checkbox(&mut self.show_behavior_patterns, "Behavior");
            ui.checkbox(&mut self.show_heat_map, "Heat Map");
        });

        ui.separator();

        // Level selector
        ui.label("Select Level:");
        ui.horizontal_wrapped(|ui| {
            for level in [
                CollectiveLevel::Local,
                CollectiveLevel::Community,
                CollectiveLevel::Regional,
                CollectiveLevel::Planetary,
                CollectiveLevel::SocialComplex,
            ] {
                if ui
                    .selectable_label(self.selected_level == Some(level), level.name())
                    .clicked()
                {
                    self.selected_level = Some(level);
                }
            }
        });

        ui.separator();

        // Resonance type selector
        ui.label("Select Resonance Type:");
        ui.horizontal_wrapped(|ui| {
            for resonance in [
                ResonanceType::Density,
                ResonanceType::Polarity,
                ResonanceType::Archetype,
                ResonanceType::Purpose,
                ResonanceType::Geographic,
                ResonanceType::Temporal,
            ] {
                if ui
                    .selectable_label(self.selected_resonance == Some(resonance), resonance.name())
                    .clicked()
                {
                    self.selected_resonance = Some(resonance);
                }
            }
        });

        ui.separator();

        // Graph time range
        ui.horizontal(|ui| {
            ui.label("Graph Time Range:");
            ui.add(egui::Slider::new(&mut self.graph_time_range, 10.0..=600.0).step_by(10.0));
        });

        // Min resonance threshold
        ui.horizontal(|ui| {
            ui.label("Min Resonance:");
            ui.add(egui::Slider::new(&mut self.min_resonance_threshold, 0.0..=1.0).step_by(0.05));
        });

        ui.separator();
    }

    /// Render metrics summary
    fn render_metrics_summary(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        ui.heading("Collective Metrics");

        let metrics = &visualizer.metrics;

        ui.label(format!("Total Collectives: {}", metrics.total_collectives));
        ui.label(format!(
            "Entities in Collectives: {}",
            metrics.entities_in_collectives
        ));
        ui.label(format!(
            "Avg Group Size: {:.1}",
            metrics.average_collective_size
        ));
        ui.label(format!("Avg Resonance: {:.2}", metrics.average_resonance));
        ui.label(format!("Avg Coherence: {:.2}", metrics.average_coherence));

        // Health index with color coding
        let health_color = health_to_color(metrics.health_index);
        ui.horizontal(|ui| {
            ui.label("Health Index:");
            ui.colored_label(health_color, format!("{:.2}", metrics.health_index));
        });

        ui.label(format!(
            "Largest Group: {}",
            metrics.largest_collective_size
        ));

        if let Some(most_common) = metrics.most_common_resonance {
            ui.label(format!("Most Common Resonance: {}", most_common.name()));
        }
    }

    /// Render level distribution
    fn render_level_distribution(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        ui.heading("Level Distribution");

        let metrics = &visualizer.metrics;

        if metrics.level_distribution.is_empty() {
            ui.label("No collectives yet");
            return;
        }

        ui.columns(2, |columns| {
            for (i, (level, count)) in metrics.level_distribution.iter().enumerate() {
                let col = &mut columns[i % 2];
                col.horizontal(|ui| {
                    // Level color indicator
                    let color = Color32::from_rgba_premultiplied(
                        (level.color()[0] * 255.0) as u8,
                        (level.color()[1] * 255.0) as u8,
                        (level.color()[2] * 255.0) as u8,
                        255,
                    );

                    ui.colored_label(color, "●");
                    ui.label(level.name());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", count));
                    });
                });
            }
        });
    }

    /// Render resonance distribution
    fn render_resonance_distribution(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        ui.heading("Resonance Distribution");

        let metrics = &visualizer.metrics;

        if metrics.resonance_distribution.is_empty() {
            ui.label("No collectives yet");
            return;
        }

        ui.columns(2, |columns| {
            for (i, (resonance, count)) in metrics.resonance_distribution.iter().enumerate() {
                let col = &mut columns[i % 2];
                col.horizontal(|ui| {
                    // Resonance color indicator
                    let color = Color32::from_rgba_premultiplied(
                        (resonance.color()[0] * 255.0) as u8,
                        (resonance.color()[1] * 255.0) as u8,
                        (resonance.color()[2] * 255.0) as u8,
                        255,
                    );

                    ui.colored_label(color, "●");
                    ui.label(resonance.name());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", count));
                    });
                });
            }
        });
    }

    /// Render group list
    fn render_group_list(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        ui.heading("Collective Groups");

        let mut groups_to_show: Vec<&CollectiveGroup> = visualizer.groups.iter().collect();

        // Filter by selected level
        if let Some(selected_level) = self.selected_level {
            groups_to_show.retain(|g| g.level == selected_level);
        }

        // Filter by selected resonance type
        if let Some(selected_resonance) = self.selected_resonance {
            groups_to_show.retain(|g| g.resonance_type == selected_resonance);
        }

        if groups_to_show.is_empty() {
            ui.label("No matching groups");
            return;
        }

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for group in groups_to_show {
                    self.render_group_item(ui, group, visualizer);
                }
            });
    }

    /// Render a single group item
    fn render_group_item(
        &mut self,
        ui: &mut Ui,
        group: &CollectiveGroup,
        visualizer: &CollectiveVisualizer,
    ) {
        ui.horizontal(|ui| {
            // Level color
            let level_color = Color32::from_rgba_premultiplied(
                (group.level.color()[0] * 255.0) as u8,
                (group.level.color()[1] * 255.0) as u8,
                (group.level.color()[2] * 255.0) as u8,
                255,
            );

            ui.colored_label(level_color, "●");

            // Group info
            ui.label(format!("{}: {} members", group.level.name(), group.size()));

            // Resonance strength
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let resonance_color = resonance_to_color(group.resonance_strength);
                ui.colored_label(
                    resonance_color,
                    format!("R:{:.2}", group.resonance_strength),
                );
            });

            // Coherence
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let coherence_color = coherence_to_color(group.coherence);
                ui.colored_label(coherence_color, format!("C:{:.2}", group.coherence));
            });
        });
    }

    /// Render graphs
    fn render_graphs(&mut self, ui: &mut Ui, visualizer: &CollectiveVisualizer) {
        ui.heading("Historical Metrics");

        if visualizer.metrics_history.is_empty() {
            ui.label("No history yet");
            return;
        }

        let current_time = visualizer.metrics_history.last().unwrap().0;
        let min_time = current_time - self.graph_time_range;

        // Filter history by time range
        let filtered_history: Vec<(f64, &CollectiveMetrics)> = visualizer
            .metrics_history
            .iter()
            .filter(|(time, _)| *time >= min_time)
            .map(|(time, metrics)| (*time, metrics))
            .collect();

        if filtered_history.is_empty() {
            ui.label("No data in time range");
            return;
        }

        // Show latest values as progress bars
        let latest = filtered_history.last().unwrap().1;

        ui.label("Latest Metrics:");
        ui.horizontal(|ui| {
            ui.label("Total Collectives:");
            ui.add(
                egui::ProgressBar::new((latest.total_collectives as f32 / 100.0).min(1.0))
                    .text(format!("{}", latest.total_collectives)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Avg Resonance:");
            ui.add(
                egui::ProgressBar::new(latest.average_resonance as f32)
                    .text(format!("{:.2}", latest.average_resonance)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Avg Coherence:");
            ui.add(
                egui::ProgressBar::new(latest.average_coherence as f32)
                    .text(format!("{:.2}", latest.average_coherence)),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Health Index:");
            let health_color = health_to_color(latest.health_index);
            ui.add_sized(
                [200.0, 15.0],
                egui::ProgressBar::new(latest.health_index as f32)
                    .fill(health_color)
                    .text(format!("{:.2}", latest.health_index)),
            );
        });
    }

    /// Get metrics summary
    pub fn get_metrics_summary(&self, visualizer: &CollectiveVisualizer) -> String {
        let metrics = &visualizer.metrics;

        format!(
            "Collectives: {} | Entities: {} | Avg Size: {:.1} | Resonance: {:.2} | Coherence: {:.2}",
            metrics.total_collectives,
            metrics.entities_in_collectives,
            metrics.average_collective_size,
            metrics.average_resonance,
            metrics.average_coherence
        )
    }
}

impl Default for CollectiveDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert health value to color
fn health_to_color(health: f64) -> Color32 {
    if health < 0.3 {
        Color32::RED
    } else if health < 0.6 {
        Color32::YELLOW
    } else if health < 0.8 {
        Color32::from_rgb(100, 200, 100)
    } else {
        Color32::GREEN
    }
}

/// Convert resonance value to color
fn resonance_to_color(resonance: f64) -> Color32 {
    if resonance < 0.3 {
        Color32::from_rgb(150, 100, 100)
    } else if resonance < 0.6 {
        Color32::from_rgb(200, 150, 100)
    } else if resonance < 0.8 {
        Color32::from_rgb(150, 200, 100)
    } else {
        Color32::from_rgb(100, 200, 200)
    }
}

/// Convert coherence value to color
fn coherence_to_color(coherence: f64) -> Color32 {
    if coherence < 0.3 {
        Color32::from_rgb(100, 100, 150)
    } else if coherence < 0.6 {
        Color32::from_rgb(100, 150, 200)
    } else if coherence < 0.8 {
        Color32::from_rgb(150, 200, 150)
    } else {
        Color32::from_rgb(200, 200, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_dashboard_new() {
        let dashboard = CollectiveDashboard::new();
        assert!(dashboard.show_dashboard);
        assert!(dashboard.show_resonance_field);
        assert!(dashboard.show_group_list);
        assert_eq!(dashboard.graph_time_range, 60.0);
        assert_eq!(dashboard.min_resonance_threshold, 0.3);
    }

    #[test]
    fn test_health_to_color() {
        assert_eq!(health_to_color(0.2), Color32::RED);
        assert_eq!(health_to_color(0.5), Color32::YELLOW);
        assert_eq!(health_to_color(0.9), Color32::GREEN);
    }

    #[test]
    fn test_resonance_to_color() {
        assert_eq!(resonance_to_color(0.2), Color32::from_rgb(150, 100, 100));
        assert_eq!(resonance_to_color(0.9), Color32::from_rgb(100, 200, 200));
    }

    #[test]
    fn test_coherence_to_color() {
        assert_eq!(coherence_to_color(0.2), Color32::from_rgb(100, 100, 150));
        assert_eq!(coherence_to_color(0.9), Color32::from_rgb(200, 200, 100));
    }

    #[test]
    fn test_get_metrics_summary() {
        let dashboard = CollectiveDashboard::new();
        let visualizer = CollectiveVisualizer::new();

        let summary = dashboard.get_metrics_summary(&visualizer);
        assert_eq!(
            summary,
            "Collectives: 0 | Entities: 0 | Avg Size: 0.0 | Resonance: 0.00 | Coherence: 0.00"
        );
    }

    #[test]
    fn test_collective_dashboard_default() {
        let dashboard = CollectiveDashboard::default();
        assert!(dashboard.show_dashboard);
        assert_eq!(dashboard.graph_time_range, 60.0);
    }
}
