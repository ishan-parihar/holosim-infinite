//! Spectrum Dashboard Panel
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Week 5:
//! "Spectrum dashboard: Real-time spectrum graph, entity distribution histogram, Veil transparency indicator"
//!
//! This module provides:
//! - Real-time spectrum graph showing entity positions
//! - Entity distribution histogram by spectrum region
//! - Veil transparency indicator
//! - Interactive spectrum filtering controls

use crate::gui::visualization::spectrum_viz::{
    SpectrumPosition, SpectrumRegion, SpectrumVisualizer,
};
use crate::types::Density;
use egui::{Context, Ui};
use std::collections::HashMap;

/// Entity data for spectrum dashboard
#[derive(Debug, Clone)]
pub struct SpectrumEntityData {
    pub entity_id: u64,
    pub spectrum_position: f32,
    pub density: Density,
    pub polarity: f32,
    pub consciousness: f32,
}

/// Spectrum dashboard panel
#[derive(Debug, Clone)]
pub struct SpectrumDashboard {
    /// Show dashboard
    pub show_dashboard: bool,
    /// Spectrum visualizer for filtering
    visualizer: SpectrumVisualizer,
    /// Entity data for visualization
    entities: Vec<SpectrumEntityData>,
    /// Distribution histogram by spectrum region
    pub distribution: HashMap<SpectrumRegion, usize>,
    /// History of spectrum positions for graphs
    history: Vec<(f64, Vec<(f32, f32)>)>,
    /// Maximum history length
    max_history: usize,
    /// Show real-time graph
    pub show_graph: bool,
    /// Show distribution histogram
    pub show_histogram: bool,
    /// Show veil indicator
    pub show_veil_indicator: bool,
}

impl SpectrumDashboard {
    /// Create a new spectrum dashboard
    pub fn new() -> Self {
        SpectrumDashboard {
            show_dashboard: true,
            visualizer: SpectrumVisualizer::new(),
            entities: Vec::new(),
            distribution: HashMap::new(),
            history: Vec::new(),
            max_history: 1000,
            show_graph: true,
            show_histogram: true,
            show_veil_indicator: true,
        }
    }

    /// Update entity data
    pub fn update_entities(&mut self, entities: Vec<SpectrumEntityData>) {
        self.entities = entities;
        self.update_distribution();
    }

    /// Update distribution histogram
    fn update_distribution(&mut self) {
        self.distribution.clear();

        for entity in &self.entities {
            let region = SpectrumRegion::from_position(entity.spectrum_position as f64);
            *self.distribution.entry(region).or_insert(0) += 1;
        }
    }

    /// Add history entry
    pub fn add_history_entry(&mut self, timestamp: f64, positions: Vec<f32>) {
        let normalized_positions: Vec<(f32, f32)> = positions
            .iter()
            .enumerate()
            .map(|(i, pos)| (*pos as f32, i as f32))
            .collect();

        self.history.push((timestamp, normalized_positions));

        // Trim history if too long
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }

    /// Get spectrum visualizer
    pub fn visualizer(&self) -> &SpectrumVisualizer {
        &self.visualizer
    }

    /// Get mutable spectrum visualizer
    pub fn visualizer_mut(&mut self) -> &mut SpectrumVisualizer {
        &mut self.visualizer
    }

    /// Show dashboard panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.show_dashboard {
            return;
        }

        egui::Window::new("Spectrum Dashboard")
            .collapsible(true)
            .resizable(true)
            .default_width(400.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                self.show_dashboard_content(ui);
            });
    }

    /// Show dashboard content
    fn show_dashboard_content(&mut self, ui: &mut Ui) {
        // Veil transparency indicator
        if self.show_veil_indicator {
            self.show_veil_transparency(ui);
            ui.separator();
        }

        // Spectrum graph
        if self.show_graph {
            self.show_spectrum_graph(ui);
            ui.separator();
        }

        // Distribution histogram
        if self.show_histogram {
            self.show_distribution_histogram(ui);
            ui.separator();
        }

        // Filter controls
        self.show_filter_controls(ui);
    }

    /// Show veil transparency indicator
    fn show_veil_transparency(&mut self, ui: &mut Ui) {
        ui.heading("The Veil");

        let veil_transparency = self.visualizer.veil_transparency();

        ui.horizontal(|ui| {
            ui.label("Transparency:");
            ui.add(
                egui::Slider::new(&mut self.visualizer_mut().veil_transparency(), 0.0..=1.0)
                    .text("")
                    .step_by(0.01),
            );
        });

        // Visual indicator
        let veil_color = [1.0_f32, 1.0_f32, 1.0_f32, veil_transparency];
        let rect = ui.allocate_space(egui::vec2(ui.available_width(), 30.0)).1;
        let painter = ui.painter();

        // Draw gradient representing veil
        painter.rect_filled(
            rect,
            0.0,
            egui::Color32::from_rgba_premultiplied(
                (veil_color[0] * 255.0) as u8,
                (veil_color[1] * 255.0) as u8,
                (veil_color[2] * 255.0) as u8,
                (veil_color[3] * 255.0) as u8,
            ),
        );

        // Draw veil label
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("Veil (v=1) - {:.0}% transparent", veil_transparency * 100.0),
            egui::FontId::default(),
            egui::Color32::BLACK,
        );

        ui.label(format!(
            "Veil limits access to oneness. Higher transparency = more time/space access."
        ));
    }

    /// Show spectrum graph
    fn show_spectrum_graph(&mut self, ui: &mut Ui) {
        ui.heading("Spectrum Graph");

        let rect = ui.allocate_space(egui::vec2(ui.available_width(), 200.0)).1;
        let painter = ui.painter();

        // Draw background gradient
        let gradient_colors = self.visualizer.get_gradient_colors();
        let segment_width = rect.width() / gradient_colors.len() as f32;

        for (i, color) in gradient_colors.iter().enumerate() {
            let x = rect.min.x + i as f32 * segment_width;
            let segment_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.min.y),
                egui::vec2(segment_width, rect.height()),
            );

            painter.rect_filled(
                segment_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(
                    (color[0] * 255.0) as u8,
                    (color[1] * 255.0) as u8,
                    (color[2] * 255.0) as u8,
                    (color[3] * 255.0) as u8,
                ),
            );
        }

        // Draw Veil line
        let veil_x = rect.min.x + rect.width() * 0.5;
        painter.line_segment(
            [
                egui::pos2(veil_x, rect.min.y),
                egui::pos2(veil_x, rect.max.y),
            ],
            (2.0, egui::Color32::WHITE),
        );

        // Draw entity positions
        for entity in &self.entities {
            if !self.visualizer.passes_filter(entity.spectrum_position) {
                continue;
            }

            let x = rect.min.x + entity.spectrum_position as f32 * rect.width();
            let y = rect.min.y + (1.0 - entity.consciousness) * rect.height();

            // Color based on density
            let density_color = Self::density_color(entity.density);

            painter.circle_filled(
                egui::pos2(x, y),
                4.0 + entity.density.as_u8() as f32,
                egui::Color32::from_rgba_premultiplied(
                    (density_color[0] * 255.0) as u8,
                    (density_color[1] * 255.0) as u8,
                    (density_color[2] * 255.0) as u8,
                    (density_color[3] * 255.0) as u8,
                ),
            );
        }

        // Draw labels
        painter.text(
            egui::pos2(rect.min.x + 5.0, rect.max.y - 5.0),
            egui::Align2::LEFT_BOTTOM,
            "Deep Time/Space",
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        painter.text(
            egui::pos2(rect.max.x - 5.0, rect.max.y - 5.0),
            egui::Align2::RIGHT_BOTTOM,
            "Deep Space/Time",
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        painter.text(
            egui::pos2(veil_x + 5.0, rect.min.y + 5.0),
            egui::Align2::LEFT_TOP,
            "The Veil (v=1)",
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );

        ui.label(format!("Total entities: {}", self.entities.len()));
    }

    /// Show distribution histogram
    fn show_distribution_histogram(&mut self, ui: &mut Ui) {
        ui.heading("Entity Distribution");

        let rect = ui.allocate_space(egui::vec2(ui.available_width(), 150.0)).1;
        let painter = ui.painter();

        let regions = [
            SpectrumRegion::DeepTimeSpace,
            SpectrumRegion::MidTimeSpace,
            SpectrumRegion::NearVeilTimeSpace,
            SpectrumRegion::TheVeil,
            SpectrumRegion::NearVeilSpaceTime,
            SpectrumRegion::MidSpaceTime,
            SpectrumRegion::DeepSpaceTime,
        ];

        // Find maximum count for scaling
        let max_count = self.distribution.values().copied().max().unwrap_or(1);
        let bar_width = rect.width() / regions.len() as f32;

        for (i, region) in regions.iter().enumerate() {
            let count = *self.distribution.get(region).unwrap_or(&0);
            let bar_height = if max_count > 0 {
                (count as f32 / max_count as f32) * rect.height()
            } else {
                0.0
            };

            let x = rect.min.x + i as f32 * bar_width;
            let bar_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.max.y - bar_height),
                egui::vec2(bar_width - 2.0, bar_height),
            );

            // Color based on region
            let region_color = region.color();
            painter.rect_filled(
                bar_rect,
                0.0,
                egui::Color32::from_rgba_premultiplied(
                    (region_color[0] * 255.0) as u8,
                    (region_color[1] * 255.0) as u8,
                    (region_color[2] * 255.0) as u8,
                    (region_color[3] * 255.0) as u8,
                ),
            );

            // Draw count label
            painter.text(
                egui::pos2(x + bar_width / 2.0, bar_rect.min.y - 5.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{}", count),
                egui::FontId::proportional(10.0),
                egui::Color32::WHITE,
            );

            // Draw region label
            painter.text(
                egui::pos2(x + bar_width / 2.0, rect.max.y + 5.0),
                egui::Align2::CENTER_TOP,
                region.name(),
                egui::FontId::proportional(8.0),
                egui::Color32::LIGHT_GRAY,
            );
        }
    }

    /// Show filter controls
    fn show_filter_controls(&mut self, ui: &mut Ui) {
        ui.heading("Spectrum Filter");

        // Show current filter range
        let (min, max) = self.visualizer.filter_range();
        ui.label(format!("Current range: {:.2} - {:.2}", min, max));

        // Quick filter buttons
        ui.horizontal(|ui| {
            if ui.button("All").clicked() {
                self.visualizer.clear_filter();
            }
            if ui.button("Time/Space").clicked() {
                self.visualizer.set_filter_range(0.0, 0.5);
            }
            if ui.button("Space/Time").clicked() {
                self.visualizer.set_filter_range(0.5, 1.0);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Deep TS").clicked() {
                self.visualizer.select_region(SpectrumRegion::DeepTimeSpace);
            }
            if ui.button("Mid TS").clicked() {
                self.visualizer.select_region(SpectrumRegion::MidTimeSpace);
            }
            if ui.button("Near Veil TS").clicked() {
                self.visualizer
                    .select_region(SpectrumRegion::NearVeilTimeSpace);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Near Veil ST").clicked() {
                self.visualizer
                    .select_region(SpectrumRegion::NearVeilSpaceTime);
            }
            if ui.button("Mid ST").clicked() {
                self.visualizer.select_region(SpectrumRegion::MidSpaceTime);
            }
            if ui.button("Deep ST").clicked() {
                self.visualizer.select_region(SpectrumRegion::DeepSpaceTime);
            }
        });

        // Custom range slider
        ui.horizontal(|ui| {
            ui.label("Custom range:");
            let mut min_val = min;
            let mut max_val = max;
            ui.add(egui::Slider::new(&mut min_val, 0.0..=1.0).text("Min"));
            ui.add(egui::Slider::new(&mut max_val, 0.0..=1.0).text("Max"));
            if ui.button("Apply").clicked() && min_val < max_val {
                self.visualizer.set_filter_range(min_val, max_val);
            }
        });

        // Toggle controls
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_graph, "Show Graph");
            ui.checkbox(&mut self.show_histogram, "Show Histogram");
            ui.checkbox(&mut self.show_veil_indicator, "Show Veil");
        });
    }

    /// Get density color
    fn density_color(density: Density) -> [f32; 4] {
        match density {
            Density::First => [1.0_f32, 0.27_f32, 0.27_f32, 1.0_f32],
            Density::Second => [1.0_f32, 0.53_f32, 0.27_f32, 1.0_f32],
            Density::Third => [1.0_f32, 0.8_f32, 0.27_f32, 1.0_f32],
            Density::Fourth => [0.27_f32, 1.0_f32, 0.27_f32, 1.0_f32],
            Density::Fifth => [0.27_f32, 1.0_f32, 1.0_f32, 1.0_f32],
            Density::Sixth => [0.27_f32, 0.27_f32, 1.0_f32, 1.0_f32],
            Density::Seventh => [0.53_f32, 0.27_f32, 1.0_f32, 1.0_f32],
            Density::Eighth => [1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32],
        }
    }

    /// Clear entity data
    pub fn clear_entities(&mut self) {
        self.entities.clear();
        self.distribution.clear();
    }

    /// Get filtered entities count
    pub fn filtered_entity_count(&self) -> usize {
        self.entities
            .iter()
            .filter(|e| self.visualizer.passes_filter(e.spectrum_position))
            .count()
    }

    /// Get total entities count
    pub fn total_entity_count(&self) -> usize {
        self.entities.len()
    }
}

impl Default for SpectrumDashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = SpectrumDashboard::new();
        assert!(dashboard.show_dashboard);
        assert!(dashboard.show_graph);
        assert!(dashboard.show_histogram);
        assert!(dashboard.show_veil_indicator);
    }

    #[test]
    fn test_update_entities() {
        let mut dashboard = SpectrumDashboard::new();

        let entities = vec![
            SpectrumEntityData {
                entity_id: 1,
                spectrum_position: 0.2,
                density: Density::Fourth,
                polarity: 0.5,
                consciousness: 0.7,
            },
            SpectrumEntityData {
                entity_id: 2,
                spectrum_position: 0.8,
                density: Density::Sixth,
                polarity: -0.3,
                consciousness: 0.9,
            },
        ];

        dashboard.update_entities(entities.clone());
        assert_eq!(dashboard.entities.len(), 2);
        assert_eq!(dashboard.total_entity_count(), 2);
    }

    #[test]
    fn test_distribution() {
        let mut dashboard = SpectrumDashboard::new();

        let entities = vec![
            SpectrumEntityData {
                entity_id: 1,
                spectrum_position: 0.1,
                density: Density::Fourth,
                polarity: 0.5,
                consciousness: 0.7,
            },
            SpectrumEntityData {
                entity_id: 2,
                spectrum_position: 0.1,
                density: Density::Sixth,
                polarity: -0.3,
                consciousness: 0.9,
            },
            SpectrumEntityData {
                entity_id: 3,
                spectrum_position: 0.9,
                density: Density::Eighth,
                polarity: 0.8,
                consciousness: 1.0,
            },
        ];

        dashboard.update_entities(entities);

        let deep_ts_count = *dashboard
            .distribution
            .get(&SpectrumRegion::DeepTimeSpace)
            .unwrap_or(&0);
        let deep_st_count = *dashboard
            .distribution
            .get(&SpectrumRegion::DeepSpaceTime)
            .unwrap_or(&0);

        assert_eq!(deep_ts_count, 2);
        assert_eq!(deep_st_count, 1);
    }

    #[test]
    fn test_filter() {
        let mut dashboard = SpectrumDashboard::new();

        let entities = vec![
            SpectrumEntityData {
                entity_id: 1,
                spectrum_position: 0.2,
                density: Density::Fourth,
                polarity: 0.5,
                consciousness: 0.7,
            },
            SpectrumEntityData {
                entity_id: 2,
                spectrum_position: 0.8,
                density: Density::Sixth,
                polarity: -0.3,
                consciousness: 0.9,
            },
        ];

        dashboard.update_entities(entities);

        // Filter to space/time side
        dashboard.visualizer_mut().set_filter_range(0.5, 1.0);
        assert_eq!(dashboard.filtered_entity_count(), 1);

        // Clear filter
        dashboard.visualizer_mut().clear_filter();
        assert_eq!(dashboard.filtered_entity_count(), 2);
    }

    #[test]
    fn test_history() {
        let mut dashboard = SpectrumDashboard::new();

        dashboard.add_history_entry(0.0, vec![0.2, 0.5, 0.8]);
        dashboard.add_history_entry(1.0, vec![0.3, 0.6, 0.9]);

        assert_eq!(dashboard.history.len(), 2);
    }

    #[test]
    fn test_history_trim() {
        let mut dashboard = SpectrumDashboard::new();
        dashboard.max_history = 5;

        for i in 0..10 {
            dashboard.add_history_entry(i as f64, vec![0.5]);
        }

        assert_eq!(dashboard.history.len(), 5);
    }

    #[test]
    fn test_clear_entities() {
        let mut dashboard = SpectrumDashboard::new();

        let entities = vec![SpectrumEntityData {
            entity_id: 1,
            spectrum_position: 0.5,
            density: Density::Fourth,
            polarity: 0.0,
            consciousness: 0.5,
        }];

        dashboard.update_entities(entities);
        assert_eq!(dashboard.total_entity_count(), 1);

        dashboard.clear_entities();
        assert_eq!(dashboard.total_entity_count(), 0);
    }

    #[test]
    fn test_density_color() {
        let red = SpectrumDashboard::density_color(Density::First);
        assert!(red[0] > 0.9_f32 && red[1] < 0.5_f32 && red[2] < 0.5_f32);

        let white = SpectrumDashboard::density_color(Density::Eighth);
        assert!(white[0] > 0.9_f32 && white[1] > 0.9_f32 && white[2] > 0.9_f32);
    }
}
