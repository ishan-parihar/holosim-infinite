//! Spectrum Continuum Panel - Interactive Spectrum Visualization
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.2:
//! "Design spectrum slider UI, Implement space/time ↔ time/space indicator,
//!  Add veil transparency visualization, Show entity spectrum positions"
//!
//! This module provides:
//! - Interactive spectrum continuum slider
//! - Space/Time ↔ Time/Space mode indicator
//! - Enhanced veil visualization with thin spots
//! - Real-time spectrum statistics
//! - Integration with holographic field rendering

use egui::{Color32, Context, Pos2, Rect, Ui, Vec2};
use std::collections::HashMap;

use crate::holographic_foundation::spectrum::DensityPosition;
use crate::holographic_foundation::spectrum::SpectrumSide;
use crate::holographic_foundation::spectrum::VelocityRatio;

/// Spectrum Continuum Panel for Phase B.2
pub struct SpectrumContinuumPanel {
    /// Panel visibility
    pub visible: bool,

    /// Current viewer spectrum position (0.0 = pure time/space, 1.0 = pure space/time)
    pub viewer_spectrum_position: f32,

    /// Veil transparency (0.0 = opaque, 1.0 = fully transparent)
    pub veil_transparency: f32,

    /// Entity distribution across spectrum
    entity_distribution: HashMap<SpectrumRegion, usize>,

    /// Total entities tracked
    total_entities: usize,

    /// Show detailed statistics
    pub show_statistics: bool,

    /// Show entity list
    pub show_entity_list: bool,

    /// Animation time for effects
    animation_time: f32,

    /// Selected spectrum region for filtering
    selected_region: Option<SpectrumRegion>,

    /// Spectrum statistics
    statistics: SpectrumStatistics,
}

/// Spectrum statistics
#[derive(Debug, Clone, Default)]
pub struct SpectrumStatistics {
    pub average_position: f32,
    pub median_position: f32,
    pub std_deviation: f32,
    pub entities_at_veil: usize,
    pub entities_in_time_space: usize,
    pub entities_in_space_time: usize,
    pub average_veil_proximity: f32,
}

/// Spectrum regions for visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpectrumRegion {
    DeepTimeSpace,
    MidTimeSpace,
    NearVeilTimeSpace,
    TheVeil,
    NearVeilSpaceTime,
    MidSpaceTime,
    DeepSpaceTime,
}

impl SpectrumRegion {
    /// Get region from position (0.0 to 1.0)
    pub fn from_position(pos: f32) -> Self {
        match pos {
            p if p < 0.15 => SpectrumRegion::DeepTimeSpace,
            p if p < 0.35 => SpectrumRegion::MidTimeSpace,
            p if p < 0.48 => SpectrumRegion::NearVeilTimeSpace,
            p if p < 0.52 => SpectrumRegion::TheVeil,
            p if p < 0.65 => SpectrumRegion::NearVeilSpaceTime,
            p if p < 0.85 => SpectrumRegion::MidSpaceTime,
            _ => SpectrumRegion::DeepSpaceTime,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            SpectrumRegion::DeepTimeSpace => "Deep Time/Space",
            SpectrumRegion::MidTimeSpace => "Mid Time/Space",
            SpectrumRegion::NearVeilTimeSpace => "Near Veil (TS)",
            SpectrumRegion::TheVeil => "The Veil",
            SpectrumRegion::NearVeilSpaceTime => "Near Veil (ST)",
            SpectrumRegion::MidSpaceTime => "Mid Space/Time",
            SpectrumRegion::DeepSpaceTime => "Deep Space/Time",
        }
    }

    /// Get color for this region
    pub fn color(&self) -> Color32 {
        match self {
            SpectrumRegion::DeepTimeSpace => Color32::from_rgb(75, 0, 130), // Deep Indigo
            SpectrumRegion::MidTimeSpace => Color32::from_rgb(138, 43, 226), // Blue Violet
            SpectrumRegion::NearVeilTimeSpace => Color32::from_rgb(186, 85, 211), // Medium Orchid
            SpectrumRegion::TheVeil => Color32::from_rgb(255, 255, 255),    // White
            SpectrumRegion::NearVeilSpaceTime => Color32::from_rgb(255, 140, 0), // Dark Orange
            SpectrumRegion::MidSpaceTime => Color32::from_rgb(255, 69, 0),  // Orange Red
            SpectrumRegion::DeepSpaceTime => Color32::from_rgb(139, 0, 0),  // Dark Red
        }
    }

    /// Get brief description
    pub fn description(&self) -> &'static str {
        match self {
            SpectrumRegion::DeepTimeSpace => "Unity consciousness, 3D time access",
            SpectrumRegion::MidTimeSpace => "Transitional consciousness, partial time access",
            SpectrumRegion::NearVeilTimeSpace => "Approaching the veil from unity side",
            SpectrumRegion::TheVeil => "The barrier between manifestations",
            SpectrumRegion::NearVeilSpaceTime => "Just beyond the veil, individual awareness",
            SpectrumRegion::MidSpaceTime => "Physical manifestation, linear time",
            SpectrumRegion::DeepSpaceTime => "Pure physicality, strong separation",
        }
    }
}

impl SpectrumContinuumPanel {
    /// Create a new spectrum continuum panel
    pub fn new() -> Self {
        Self {
            visible: true,
            viewer_spectrum_position: 0.7, // Default to space/time (human perspective)
            veil_transparency: 0.3,
            entity_distribution: HashMap::new(),
            total_entities: 0,
            show_statistics: true,
            show_entity_list: false,
            animation_time: 0.0,
            selected_region: None,
            statistics: SpectrumStatistics::default(),
        }
    }

    /// Update entity distribution
    pub fn update_distribution(&mut self, positions: &[f32]) {
        self.entity_distribution.clear();
        self.total_entities = positions.len();

        if positions.is_empty() {
            self.statistics = SpectrumStatistics::default();
            return;
        }

        // Calculate distribution
        for &pos in positions {
            let region = SpectrumRegion::from_position(pos);
            *self.entity_distribution.entry(region).or_insert(0) += 1;
        }

        // Calculate statistics
        let sum: f32 = positions.iter().sum();
        let count = positions.len() as f32;
        self.statistics.average_position = sum / count;

        // Median
        let mut sorted = positions.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        self.statistics.median_position = sorted[sorted.len() / 2];

        // Standard deviation
        let mean = self.statistics.average_position;
        let variance: f32 = positions.iter().map(|p| (p - mean).powi(2)).sum::<f32>() / count;
        self.statistics.std_deviation = variance.sqrt();

        // Count by side
        self.statistics.entities_in_time_space = positions.iter().filter(|p| **p < 0.5).count();
        self.statistics.entities_in_space_time = positions.iter().filter(|p| **p >= 0.5).count();
        self.statistics.entities_at_veil =
            positions.iter().filter(|p| (**p - 0.5).abs() < 0.1).count();

        // Average veil proximity
        self.statistics.average_veil_proximity = positions
            .iter()
            .map(|p| 1.0 - (p - 0.5).abs() * 2.0)
            .sum::<f32>()
            / count;
    }

    /// Update animation time
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("🌌 Spectrum Continuum")
            .collapsible(true)
            .resizable(true)
            .default_width(450.0)
            .show(ctx, |ui| {
                self.show_content(ui);
            });
    }

    /// Show panel content
    fn show_content(&mut self, ui: &mut Ui) {
        // Main spectrum slider
        self.show_spectrum_slider(ui);
        ui.add_space(10.0);

        // Space/Time ↔ Time/Space indicator
        self.show_mode_indicator(ui);
        ui.add_space(10.0);

        // Veil visualization
        self.show_veil_visualization(ui);
        ui.add_space(10.0);

        // Distribution histogram
        self.show_distribution_histogram(ui);
        ui.add_space(10.0);

        // Statistics
        if self.show_statistics {
            self.show_statistics(ui);
        }
    }

    /// Show interactive spectrum slider
    fn show_spectrum_slider(&mut self, ui: &mut Ui) {
        ui.heading("📊 Spectrum Position");
        ui.label("v = s/t (velocity ratio)");

        // Get available width
        let slider_width = ui.available_width() - 80.0;

        // Draw spectrum continuum background
        let rect = ui.allocate_space(Vec2::new(slider_width, 50.0)).1;
        let painter = ui.painter();

        // Draw gradient background
        let regions = [
            SpectrumRegion::DeepTimeSpace,
            SpectrumRegion::MidTimeSpace,
            SpectrumRegion::NearVeilTimeSpace,
            SpectrumRegion::TheVeil,
            SpectrumRegion::NearVeilSpaceTime,
            SpectrumRegion::MidSpaceTime,
            SpectrumRegion::DeepSpaceTime,
        ];

        let segment_width = rect.width() / regions.len() as f32;
        for (i, region) in regions.iter().enumerate() {
            let x = rect.min.x + i as f32 * segment_width;
            let segment_rect = Rect::from_min_size(
                Pos2::new(x, rect.min.y),
                Vec2::new(segment_width, rect.height()),
            );
            painter.rect_filled(segment_rect, 0.0, region.color());
        }

        // Draw veil line
        let veil_x = rect.min.x + rect.width() * 0.5;
        painter.line_segment(
            [Pos2::new(veil_x, rect.min.y), Pos2::new(veil_x, rect.max.y)],
            (3.0, Color32::WHITE),
        );

        // Draw viewer position marker
        let viewer_x = rect.min.x + self.viewer_spectrum_position * rect.width();
        let marker_y = rect.min.y + rect.height() / 2.0;

        // Animated glow effect
        let glow_size = 8.0 + (self.animation_time * 2.0).sin() * 2.0;
        painter.circle_filled(
            Pos2::new(viewer_x, marker_y),
            glow_size,
            Color32::from_rgba_premultiplied(255, 255, 255, 100),
        );
        painter.circle_filled(Pos2::new(viewer_x, marker_y), 6.0, Color32::YELLOW);

        // Labels
        painter.text(
            rect.min,
            egui::Align2::LEFT_TOP,
            "Time/Space\n(Unity)",
            egui::FontId::proportional(10.0),
            Color32::WHITE,
        );
        painter.text(
            Pos2::new(rect.max.x, rect.min.y),
            egui::Align2::RIGHT_TOP,
            "Space/Time\n(Individual)",
            egui::FontId::proportional(10.0),
            Color32::WHITE,
        );

        // Slider control
        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Position:");
            let mut pos = self.viewer_spectrum_position;
            ui.add(
                egui::Slider::new(&mut pos, 0.0..=1.0)
                    .text("")
                    .step_by(0.01),
            );
            self.viewer_spectrum_position = pos;

            // Quick position buttons
            if ui.small_button("Unity").clicked() {
                self.viewer_spectrum_position = 0.1;
            }
            if ui.small_button("Veil").clicked() {
                self.viewer_spectrum_position = 0.5;
            }
            if ui.small_button("Physical").clicked() {
                self.viewer_spectrum_position = 0.9;
            }
        });
    }

    /// Show Space/Time ↔ Time/Space mode indicator
    fn show_mode_indicator(&mut self, ui: &mut Ui) {
        ui.heading("🔄 Experience Mode");

        let side = if self.viewer_spectrum_position < 0.48 {
            SpectrumSide::TimeSpace
        } else if self.viewer_spectrum_position > 0.52 {
            SpectrumSide::SpaceTime
        } else {
            SpectrumSide::AtVeil
        };

        let (mode_name, mode_color, mode_desc) = match side {
            SpectrumSide::TimeSpace => (
                "TIME/SPACE DOMINANT",
                Color32::from_rgb(138, 43, 226),
                "3D time, 1D space\nUnity consciousness\nAccess to all timelines",
            ),
            SpectrumSide::AtVeil => (
                "AT THE VEIL",
                Color32::WHITE,
                "Transition zone\nWhere polarities meet\nTransformation in progress",
            ),
            SpectrumSide::SpaceTime => (
                "SPACE/TIME DOMINANT",
                Color32::from_rgb(255, 140, 0),
                "3D space, 1D time\nIndividual consciousness\nLinear time perception",
            ),
        };

        // Draw mode indicator box
        let rect = ui.allocate_space(Vec2::new(ui.available_width(), 60.0)).1;
        let painter = ui.painter();

        // Background
        painter.rect_filled(rect, 5.0, mode_color);
        painter.rect_stroke(rect, 5.0, (2.0, Color32::WHITE));

        // Mode name
        painter.text(
            rect.center_top() + Vec2::new(0.0, 10.0),
            egui::Align2::CENTER_TOP,
            mode_name,
            egui::FontId::proportional(16.0),
            Color32::BLACK,
        );

        // Description
        painter.text(
            rect.center_bottom() - Vec2::new(0.0, 5.0),
            egui::Align2::CENTER_BOTTOM,
            mode_desc,
            egui::FontId::proportional(10.0),
            Color32::BLACK,
        );
    }

    /// Show veil visualization
    fn show_veil_visualization(&mut self, ui: &mut Ui) {
        ui.heading("🌫️ The Veil (v=1)");

        // Veil transparency slider
        ui.horizontal(|ui| {
            ui.label("Transparency:");
            ui.add(
                egui::Slider::new(&mut self.veil_transparency, 0.0..=1.0)
                    .text("")
                    .step_by(0.01),
            );
            ui.label(format!("{:.0}%", self.veil_transparency * 100.0));
        });

        // Visual veil representation
        let rect = ui.allocate_space(Vec2::new(ui.available_width(), 40.0)).1;
        let painter = ui.painter();

        // Draw veil with varying opacity
        let veil_alpha = ((1.0 - self.veil_transparency) * 255.0) as u8;

        // Animated veil effect
        let wave_offset = (self.animation_time * 2.0).sin() * 5.0;

        for i in 0..rect.width() as i32 {
            let x = rect.min.x + i as f32;
            let wave = (i as f32 * 0.1 + self.animation_time * 3.0).sin() * wave_offset;
            let y = rect.center().y + wave;

            let alpha = veil_alpha.saturating_sub((wave.abs() * 5.0) as u8);
            painter.circle_filled(
                Pos2::new(x, y),
                2.0,
                Color32::from_rgba_premultiplied(200, 200, 220, alpha),
            );
        }

        // Veil status text
        let status = if self.veil_transparency < 0.3 {
            "Veil is OPAQUE - Limited access to oneness"
        } else if self.veil_transparency < 0.7 {
            "Veil is THINNING - Partial access to unity"
        } else {
            "Veil is TRANSPARENT - Full access to oneness"
        };

        ui.label(status);

        // Density-based veil info
        ui.collapsing("Density & Veil", |ui| {
            ui.label("Veil transparency by density:");
            ui.label("  1st-2nd: 0-10% (Veil fully active)");
            ui.label("  3rd: 30% (Human experience)");
            ui.label("  4th: 50% (Veil begins to thin)");
            ui.label("  5th: 70% (Mostly dissolved)");
            ui.label("  6th-8th: 90-100% (Fully transparent)");
        });
    }

    /// Show distribution histogram
    fn show_distribution_histogram(&mut self, ui: &mut Ui) {
        ui.heading("📈 Entity Distribution");

        if self.total_entities == 0 {
            ui.label("No entities to display");
            return;
        }

        let rect = ui.allocate_space(Vec2::new(ui.available_width(), 100.0)).1;
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

        let max_count = self
            .entity_distribution
            .values()
            .copied()
            .max()
            .unwrap_or(1);
        let bar_width = rect.width() / regions.len() as f32;

        for (i, region) in regions.iter().enumerate() {
            let count = *self.entity_distribution.get(region).unwrap_or(&0);
            let bar_height = if max_count > 0 {
                (count as f32 / max_count as f32) * rect.height() * 0.8
            } else {
                0.0
            };

            let x = rect.min.x + i as f32 * bar_width;
            let bar_rect = Rect::from_min_size(
                Pos2::new(x, rect.max.y - bar_height),
                Vec2::new(bar_width - 2.0, bar_height),
            );

            // Highlight selected region
            let color = if self.selected_region == Some(*region) {
                Color32::YELLOW
            } else {
                region.color()
            };

            painter.rect_filled(bar_rect, 2.0, color);

            // Count label
            if count > 0 {
                painter.text(
                    Pos2::new(x + bar_width / 2.0, bar_rect.min.y - 5.0),
                    egui::Align2::CENTER_BOTTOM,
                    format!("{}", count),
                    egui::FontId::proportional(9.0),
                    Color32::WHITE,
                );
            }
        }

        // Summary
        ui.label(format!(
            "Total: {} | Time/Space: {} | Space/Time: {} | At Veil: {}",
            self.total_entities,
            self.statistics.entities_in_time_space,
            self.statistics.entities_in_space_time,
            self.statistics.entities_at_veil
        ));
    }

    /// Show statistics
    fn show_statistics(&mut self, ui: &mut Ui) {
        ui.collapsing("📊 Statistics", |ui| {
            ui.label(format!(
                "Average position: {:.3}",
                self.statistics.average_position
            ));
            ui.label(format!(
                "Median position: {:.3}",
                self.statistics.median_position
            ));
            ui.label(format!(
                "Std deviation: {:.3}",
                self.statistics.std_deviation
            ));
            ui.label(format!(
                "Average veil proximity: {:.1}%",
                self.statistics.average_veil_proximity * 100.0
            ));

            // Position interpretation
            if self.statistics.average_position < 0.4 {
                ui.label("Population tends toward UNITY consciousness");
            } else if self.statistics.average_position > 0.6 {
                ui.label("Population tends toward INDIVIDUAL consciousness");
            } else {
                ui.label("Population balanced near THE VEIL");
            }
        });
    }

    /// Get viewer spectrum position
    pub fn viewer_position(&self) -> f32 {
        self.viewer_spectrum_position
    }

    /// Get veil transparency
    pub fn transparency(&self) -> f32 {
        self.veil_transparency
    }

    /// Get current spectrum side
    pub fn current_side(&self) -> SpectrumSide {
        if self.viewer_spectrum_position < 0.48 {
            SpectrumSide::TimeSpace
        } else if self.viewer_spectrum_position > 0.52 {
            SpectrumSide::SpaceTime
        } else {
            SpectrumSide::AtVeil
        }
    }
}

impl Default for SpectrumContinuumPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = SpectrumContinuumPanel::new();
        assert!(panel.visible);
        assert_eq!(panel.viewer_spectrum_position, 0.7);
    }

    #[test]
    fn test_spectrum_region_from_position() {
        assert_eq!(
            SpectrumRegion::from_position(0.0),
            SpectrumRegion::DeepTimeSpace
        );
        assert_eq!(SpectrumRegion::from_position(0.5), SpectrumRegion::TheVeil);
        assert_eq!(
            SpectrumRegion::from_position(1.0),
            SpectrumRegion::DeepSpaceTime
        );
    }

    #[test]
    fn test_update_distribution() {
        let mut panel = SpectrumContinuumPanel::new();
        let positions = vec![0.1, 0.2, 0.5, 0.5, 0.8, 0.9];

        panel.update_distribution(&positions);

        assert_eq!(panel.total_entities, 6);
        assert_eq!(panel.statistics.entities_at_veil, 2);
        assert_eq!(panel.statistics.entities_in_time_space, 2);
        assert_eq!(panel.statistics.entities_in_space_time, 4);
    }

    #[test]
    fn test_current_side() {
        let mut panel = SpectrumContinuumPanel::new();

        panel.viewer_spectrum_position = 0.3;
        assert_eq!(panel.current_side(), SpectrumSide::TimeSpace);

        panel.viewer_spectrum_position = 0.5;
        assert_eq!(panel.current_side(), SpectrumSide::AtVeil);

        panel.viewer_spectrum_position = 0.8;
        assert_eq!(panel.current_side(), SpectrumSide::SpaceTime);
    }
}
