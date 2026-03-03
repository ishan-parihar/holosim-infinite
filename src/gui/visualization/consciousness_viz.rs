//! Consciousness Visualization Module
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase D.1: Consciousness Integration - Free Will Visualization
//!
//! This module implements:
//! 1. FreeWillVisualizer - visualize choice moments, decision weight distribution,
//!    seed-based reconstruction, free will capacity indicator
//! 2. TeleologicalPullView - render attractor fields as gravity wells,
//!    show spiritual gravity toward unity, visualize entity trajectory in consciousness space
//! 3. CausalInversionView - implement top-down causation visualization,
//!    show density hierarchy arrows, visualize higher → lower density influence
//! 4. DensityTransitionView - animate density progression, show polarity requirement thresholds,
//!    visualize harvest readiness indicators, display light body activation
//!
//! Key Design Principles:
//! - Free Will is NOT random - it's non-deterministic but CORRELATED (seed-based)
//! - Teleological pull is "spiritual gravity" toward unity
//! - Top-down causation: Higher densities influence lower densities
//! - Density progression: Entities progress within their density, not change density
//! - Polarity: STO (Service-to-Others) vs STS (Service-to-Self) as fundamental choice

use crate::consciousness::free_will::{
    ChoiceRecord, FreeWillKernel, FreeWillStatistics, PolarityPreference,
};
use crate::consciousness::kernel::{ConsciousnessKernel, ConsciousnessSignal};
use crate::consciousness::possibility_space::{OutcomeType, PossibilitySpace};
use crate::evolution_density_octave::density_octave::{
    Density, DensityCharacteristics, DensityOctave,
};
use crate::polarization::{PolarityDirection as PolDir, PolarizationProgress, PolarizationState};
use crate::types::Polarity;
use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};

/// Visualization types using f32 as specified
type Float = f32;

// ============================================================================
// FreeWillVisualizer
// ============================================================================

/// Free Will Visualizer
///
/// Visualizes choice moments, decision weight distribution, seed-based reconstruction,
/// and free will capacity indicator.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Free Will = controlled selection from possibility space"
/// "Free Will requires the ability to influence which possibility actualizes"
/// "Free Will is NOT random - it's non-deterministic but CORRELATED (seed-based)"
#[derive(Debug, Clone)]
pub struct FreeWillVisualizer {
    /// Show choice timeline
    pub show_timeline: bool,
    /// Show decision weight distribution
    pub show_weights: bool,
    /// Show capacity indicator
    pub show_capacity: bool,
    /// Selected entity ID for detailed view
    pub selected_entity_id: Option<u64>,
    /// Animation phase for choice moments
    pub animation_phase: Float,
    /// Historical choice records for visualization
    pub choice_history: Vec<ChoiceVisualization>,
}

impl Default for FreeWillVisualizer {
    fn default() -> Self {
        Self {
            show_timeline: true,
            show_weights: true,
            show_capacity: true,
            selected_entity_id: None,
            animation_phase: 0.0,
            choice_history: Vec::new(),
        }
    }
}

/// A single choice visualized
#[derive(Debug, Clone)]
pub struct ChoiceVisualization {
    /// Choice ID
    pub choice_id: u64,
    /// Timestamp/step
    pub step: u64,
    /// Possibility space size
    pub possibility_count: usize,
    /// Selected index
    pub selected_index: usize,
    /// Confidence level
    pub confidence: Float,
    /// STO alignment (-1.0 to 1.0)
    pub sto_alignment: Float,
    /// Possibility weights for distribution chart
    pub possibility_weights: Vec<Float>,
}

impl FreeWillVisualizer {
    /// Create a new free will visualizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with new choice data
    pub fn update_with_choice(&mut self, record: &ChoiceRecord, weights: Vec<Float>) {
        let viz = ChoiceVisualization {
            choice_id: record.choice_id,
            step: record.timestamp,
            possibility_count: record.possibility_space_size,
            selected_index: record.selected_index,
            confidence: record.confidence as Float,
            sto_alignment: record.sto_alignment as Float,
            possibility_weights: weights,
        };

        // Keep last 50 choices for visualization
        if self.choice_history.len() >= 50 {
            self.choice_history.remove(0);
        }
        self.choice_history.push(viz);
    }

    /// Render the choice timeline
    pub fn render_timeline(&self, ui: &mut egui::Ui, rect: Rect) {
        if self.choice_history.is_empty() {
            ui.label("No choices recorded yet");
            return;
        }

        let painter = ui.painter();
        let height = rect.height();
        let width = rect.width();

        // Draw timeline background
        let timeline_rect = Rect::from_min_size(rect.min, Vec2::new(width, height * 0.3));
        painter.rect_filled(
            timeline_rect,
            2.0,
            Color32::from_rgba_unmultiplied(30, 30, 40, 200),
        );

        // Draw timeline line
        let y = timeline_rect.center().y;
        painter.line_segment(
            [
                Pos2::new(timeline_rect.min.x, y),
                Pos2::new(timeline_rect.max.x, y),
            ],
            Stroke::new(2.0, Color32::from_rgb(100, 150, 200)),
        );

        // Draw choice points
        let num_choices = self.choice_history.len();
        for (i, choice) in self.choice_history.iter().enumerate() {
            let x = timeline_rect.min.x + (i as Float / (num_choices - 1).max(1) as Float) * width;

            // Color based on STO alignment
            let color = if choice.sto_alignment > 0.0 {
                Color32::from_rgb(100, 200, 100) // STO - green
            } else if choice.sto_alignment < 0.0 {
                Color32::from_rgb(200, 100, 100) // STS - red
            } else {
                Color32::from_rgb(150, 150, 150) // Neutral - gray
            };

            // Size based on confidence
            let radius = 3.0 + choice.confidence * 5.0;

            painter.circle_filled(Pos2::new(x, y), radius, color);
        }

        // Labels
        let label_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, timeline_rect.max.y + 10.0),
            Vec2::new(width, height * 0.15),
        );

        let painter = ui.painter_at(label_rect);
        painter.text(
            Pos2::new(label_rect.min.x, label_rect.min.y),
            egui::Align2::LEFT_TOP,
            "Choice Timeline (green=STO, red=STS)",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }

    /// Render decision weight distribution
    pub fn render_weight_distribution(&self, ui: &mut egui::Ui, rect: Rect) {
        if self.choice_history.is_empty() {
            ui.label("No choice data available");
            return;
        }

        let painter = ui.painter();
        let height = rect.height();
        let width = rect.width();

        // Get most recent choice for weight visualization
        let latest = self.choice_history.last().unwrap();

        if latest.possibility_weights.is_empty() {
            return;
        }

        // Draw bar chart for possibility weights
        let bar_width = width / latest.possibility_weights.len() as Float;
        let max_weight = latest
            .possibility_weights
            .iter()
            .cloned()
            .fold(0.0f32, Float::max);

        for (i, &weight) in latest.possibility_weights.iter().enumerate() {
            let x = rect.min.x + i as Float * bar_width;
            let bar_height = if max_weight > 0.0 {
                (weight / max_weight) * height * 0.8
            } else {
                0.0
            };

            // Color based on whether this was selected
            let color = if i == latest.selected_index {
                Color32::from_rgb(255, 200, 50) // Selected - gold
            } else {
                Color32::from_rgb(80, 120, 180) // Not selected - blue
            };

            let bar_rect = Rect::from_min_size(
                Pos2::new(x + 2.0, rect.max.y - bar_height),
                Vec2::new(bar_width - 4.0, bar_height),
            );
            painter.rect_filled(bar_rect, 1.0, color);
        }

        // Labels
        let label_text = format!(
            "Decision Weights - Selected: {} (conf: {:.2})",
            latest.selected_index, latest.confidence
        );
        painter.text(
            Pos2::new(rect.min.x, rect.min.y),
            egui::Align2::LEFT_TOP,
            label_text,
            egui::FontId::default(),
            Color32::from_white_alpha(200),
        );
    }

    /// Render free will capacity indicator
    pub fn render_capacity_indicator(
        &self,
        ui: &mut egui::Ui,
        rect: Rect,
        stats: &FreeWillStatistics,
    ) {
        let painter = ui.painter();

        // Background circle
        painter.circle_filled(
            rect.center(),
            rect.width().min(rect.height()) * 0.4,
            Color32::from_rgba_unmultiplied(40, 40, 50, 200),
        );

        // Capacity arc (full circle represents 1.0)
        let capacity = stats.capacity as f64;
        let activation = stats.activation_level as f64;
        let consciousness = stats.consciousness_level as f64;

        // Draw capacity arc (outer ring)
        Self::draw_arc(
            painter,
            rect.center(),
            (rect.width().min(rect.height()) * 0.45) as f64,
            0.0,
            capacity * std::f64::consts::TAU,
            Color32::from_rgb(100, 150, 200),
        );

        // Draw activation arc (middle ring)
        Self::draw_arc(
            painter,
            rect.center(),
            (rect.width().min(rect.height()) * 0.35) as f64,
            0.0,
            activation * std::f64::consts::TAU,
            Color32::from_rgb(150, 200, 100),
        );

        // Draw consciousness arc (inner ring)
        Self::draw_arc(
            painter,
            rect.center(),
            (rect.width().min(rect.height()) * 0.25) as f64,
            0.0,
            consciousness * std::f64::consts::TAU,
            Color32::from_rgb(200, 150, 50),
        );

        // Center text
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{:.0}", stats.consciousness_level * 100.0),
            egui::FontId::monospace(24.0),
            Color32::WHITE,
        );

        // Labels
        let label_y = rect.max.y - 20.0;
        painter.text(
            Pos2::new(rect.center().x, label_y),
            egui::Align2::CENTER_TOP,
            "Free Will Capacity",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }

    /// Helper to draw arcs
    fn draw_arc(
        painter: &Painter,
        center: Pos2,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Color32,
    ) {
        let segments = 32;
        let d_angle = (end_angle - start_angle) / segments as f64;

        for i in 0..segments {
            let a1 = start_angle + i as f64 * d_angle;
            let a2 = start_angle + (i + 1) as f64 * d_angle;

            let p1 = Pos2::new(
                center.x + (radius * a1.cos()) as Float,
                center.y - (radius * a1.sin()) as Float,
            );
            let p2 = Pos2::new(
                center.x + (radius * a2.cos()) as Float,
                center.y - (radius * a2.sin()) as Float,
            );

            painter.line_segment([p1, p2], Stroke::new(4.0, color));
        }
    }

    /// Render seed-based reconstruction info
    pub fn render_seed_info(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        // Background
        painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(30, 30, 40, 200));

        // Title
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Seed-Based Reconstruction",
            egui::FontId::default(),
            Color32::from_rgb(200, 150, 50),
        );

        // Info text
        let info_y = rect.min.y + 35.0;
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y),
            egui::Align2::LEFT_TOP,
            "Free Will is non-deterministic but CORRELATED:",
            egui::FontId::default(),
            Color32::from_white_alpha(200),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 20.0),
            egui::Align2::LEFT_TOP,
            "• Choices are influenced by consciousness state",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 40.0),
            egui::Align2::LEFT_TOP,
            "• Polarity preference creates correlation",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 60.0),
            egui::Align2::LEFT_TOP,
            "• Not random (predetermined) or chaotic",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 80.0),
            egui::Align2::LEFT_TOP,
            "• Seed-based: reproducible with same state",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }
}

// ============================================================================
// TeleologicalPullView
// ============================================================================

/// Teleological Pull View
///
/// Renders attractor fields as gravity wells, shows spiritual gravity toward unity,
/// and visualizes entity trajectory in consciousness space.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each stage creates attractor-fields that pull toward the next level"
/// "Attractor-fields are 'spiritual gravity' that pulls the previous stage toward the next"
#[derive(Debug, Clone)]
pub struct TeleologicalPullView {
    /// Show attractor fields as gravity wells
    pub show_attractors: bool,
    /// Show entity trajectory
    pub show_trajectory: bool,
    /// Consciousness space dimensions
    pub space_bounds: (Float, Float),
    /// Entity positions in consciousness space
    pub entity_positions: Vec<ConsciousnessPosition>,
    /// Attractor field centers
    pub attractor_centers: Vec<AttractorField>,
    /// Trajectory history
    pub trajectory_history: Vec<TrajectoryPoint>,
}

/// A position in consciousness space
#[derive(Debug, Clone, Copy)]
pub struct ConsciousnessPosition {
    pub x: Float,
    pub y: Float,
    pub density: Float,
    pub polarity: Float, // -1.0 to 1.0
}

/// An attractor field (gravity well)
#[derive(Debug, Clone, Copy)]
pub struct AttractorField {
    pub x: Float,
    pub y: Float,
    pub strength: Float,
    pub density_level: u8,
    pub polarity_bias: Float,
}

/// A point on the entity's trajectory
#[derive(Debug, Clone, Copy)]
pub struct TrajectoryPoint {
    pub x: Float,
    pub y: Float,
    pub step: u64,
    pub velocity: Float,
}

impl Default for TeleologicalPullView {
    fn default() -> Self {
        Self {
            show_attractors: true,
            show_trajectory: true,
            space_bounds: (100.0, 100.0),
            entity_positions: Vec::new(),
            attractor_centers: Vec::new(),
            trajectory_history: Vec::new(),
        }
    }
}

impl TeleologicalPullView {
    /// Create a new teleological pull view
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entity position
    pub fn add_entity_position(&mut self, x: Float, y: Float, density: Float, polarity: Float) {
        self.entity_positions.push(ConsciousnessPosition {
            x,
            y,
            density,
            polarity,
        });
    }

    /// Add an attractor field
    pub fn add_attractor(
        &mut self,
        x: Float,
        y: Float,
        strength: Float,
        density_level: u8,
        polarity_bias: Float,
    ) {
        self.attractor_centers.push(AttractorField {
            x,
            y,
            strength,
            density_level,
            polarity_bias,
        });
    }

    /// Add trajectory point
    pub fn add_trajectory_point(&mut self, x: Float, y: Float, step: u64, velocity: Float) {
        // Keep last 100 points
        if self.trajectory_history.len() >= 100 {
            self.trajectory_history.remove(0);
        }
        self.trajectory_history.push(TrajectoryPoint {
            x,
            y,
            step,
            velocity,
        });
    }

    /// Render attractor fields as gravity wells
    pub fn render_gravity_wells(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        for attractor in &self.attractor_centers {
            // Map attractor position to screen coordinates
            let screen_x = rect.min.x + (attractor.x / self.space_bounds.0) * rect.width();
            let screen_y = rect.min.y + (attractor.y / self.space_bounds.1) * rect.height();
            let center = Pos2::new(screen_x, screen_y);

            // Draw concentric circles representing gravity well
            let max_radius = rect.width().min(rect.height()) * 0.3 * attractor.strength;

            for i in (1..=5).rev() {
                let radius = max_radius * (i as Float / 5.0);
                let alpha = (30 + (i as u8) * 40) as u8;

                // Color based on density level
                let color = match attractor.density_level {
                    1 => Color32::from_rgba_unmultiplied(200, 80, 80, alpha), // 1st - Red
                    2 => Color32::from_rgba_unmultiplied(200, 150, 50, alpha), // 2nd - Orange
                    3 => Color32::from_rgba_unmultiplied(200, 200, 50, alpha), // 3rd - Yellow
                    4 => Color32::from_rgba_unmultiplied(80, 200, 80, alpha), // 4th - Green
                    5 => Color32::from_rgba_unmultiplied(50, 150, 200, alpha), // 5th - Blue
                    6 => Color32::from_rgba_unmultiplied(100, 50, 200, alpha), // 6th - Indigo
                    7 => Color32::from_rgba_unmultiplied(150, 50, 200, alpha), // 7th - Violet
                    _ => Color32::from_rgba_unmultiplied(200, 200, 200, alpha), // 8th - White
                };

                painter.circle_stroke(center, radius, Stroke::new(2.0, color));
            }

            // Center label
            let label = format!("D{}", attractor.density_level);
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::default(),
                Color32::from_white_alpha(200),
            );
        }
    }

    /// Render entity trajectory
    pub fn render_trajectory(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        if self.trajectory_history.is_empty() {
            return;
        }

        // Draw trajectory path
        for i in 1..self.trajectory_history.len() {
            let p1 = &self.trajectory_history[i - 1];
            let p2 = &self.trajectory_history[i];

            let screen_x1 = rect.min.x + (p1.x / self.space_bounds.0) * rect.width();
            let screen_y1 = rect.min.y + (p1.y / self.space_bounds.1) * rect.height();
            let screen_x2 = rect.min.x + (p2.x / self.space_bounds.0) * rect.width();
            let screen_y2 = rect.min.y + (p2.y / self.space_bounds.1) * rect.height();

            // Color based on velocity (faster = brighter)
            let alpha = (100.0 + p1.velocity * 155.0) as u8;
            let color = Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

            painter.line_segment(
                [
                    Pos2::new(screen_x1, screen_y1),
                    Pos2::new(screen_x2, screen_y2),
                ],
                Stroke::new(2.0, color),
            );
        }

        // Draw current position
        if let Some(current) = self.trajectory_history.last() {
            let screen_x = rect.min.x + (current.x / self.space_bounds.0) * rect.width();
            let screen_y = rect.min.y + (current.y / self.space_bounds.1) * rect.height();

            painter.circle_filled(
                Pos2::new(screen_x, screen_y),
                6.0,
                Color32::from_rgb(255, 200, 50),
            );
        }
    }

    /// Render spiritual gravity legend
    pub fn render_legend(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(30, 30, 40, 200));

        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Spiritual Gravity Wells",
            egui::FontId::default(),
            Color32::from_rgb(200, 150, 50),
        );

        let info_y = rect.min.y + 35.0;
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y),
            egui::Align2::LEFT_TOP,
            "Attractor fields pull entities toward",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 20.0),
            egui::Align2::LEFT_TOP,
            "higher density consciousness (unity).",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        // Show density levels
        for i in 0..8 {
            let y = info_y + 50.0 + i as f32 * 18.0;
            let color = match i {
                0 => Color32::from_rgb(200, 80, 80),
                1 => Color32::from_rgb(200, 150, 50),
                2 => Color32::from_rgb(200, 200, 50),
                3 => Color32::from_rgb(80, 200, 80),
                4 => Color32::from_rgb(50, 150, 200),
                5 => Color32::from_rgb(100, 50, 200),
                6 => Color32::from_rgb(150, 50, 200),
                _ => Color32::from_rgb(200, 200, 200),
            };

            painter.circle_filled(Pos2::new(rect.min.x + 20.0, y), 5.0, color);
            painter.text(
                Pos2::new(rect.min.x + 35.0, y - 6.0),
                egui::Align2::LEFT_CENTER,
                format!("{} Density", Self::density_name(i + 1)),
                egui::FontId::default(),
                Color32::from_white_alpha(200),
            );
        }
    }

    /// Helper to get density name
    fn density_name(level: u8) -> &'static str {
        match level {
            1 => "1st",
            2 => "2nd",
            3 => "3rd",
            4 => "4th",
            5 => "5th",
            6 => "6th",
            7 => "7th",
            8 => "8th",
            _ => "Unknown",
        }
    }
}

// ============================================================================
// CausalInversionView
// ============================================================================

/// Causal Inversion View
///
/// Implements top-down causation visualization, shows density hierarchy arrows,
/// visualizes higher → lower density influence.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Higher densities influence lower densities (top-down causation)"
/// "This is 'causal inversion' - consciousness directs matter, not the reverse"
#[derive(Debug, Clone)]
pub struct CausalInversionView {
    /// Show density hierarchy
    pub show_hierarchy: bool,
    /// Show influence arrows
    pub show_arrows: bool,
    /// Density influence strengths
    pub influence_matrix: [[Float; 8]; 8],
    /// Current active influences
    pub active_influences: Vec<ActiveInfluence>,
}

/// An active influence between densities
#[derive(Debug, Clone)]
pub struct ActiveInfluence {
    pub from_density: u8,
    pub to_density: u8,
    pub strength: Float,
}

impl Default for CausalInversionView {
    fn default() -> Self {
        let mut influence_matrix = [[0.0; 8]; 8];

        // Higher densities influence lower densities (top-down)
        // Fill in the influence matrix: influence_matrix[from][to]
        for from in 0..8 {
            for to in 0..8 {
                if from > to {
                    // Higher density influences lower density
                    influence_matrix[from][to] = ((from - to) as Float) * 0.3;
                }
            }
        }

        Self {
            show_hierarchy: true,
            show_arrows: true,
            influence_matrix,
            active_influences: Vec::new(),
        }
    }
}

impl CausalInversionView {
    /// Create a new causal inversion view
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an active influence
    pub fn add_influence(&mut self, from_density: u8, to_density: u8, strength: Float) {
        self.active_influences.push(ActiveInfluence {
            from_density,
            to_density,
            strength,
        });
    }

    /// Render density hierarchy with influence arrows
    pub fn render_hierarchy(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        let density_height = rect.height() / 9.0; // 8 densities + margin

        // Draw density levels from top (8th) to bottom (1st)
        for i in 0..8 {
            let density_level = 8 - i; // 8th at top
            let y = rect.min.y + i as Float * density_height;
            let level_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, y),
                Vec2::new(rect.width() * 0.4, density_height - 2.0),
            );

            // Color based on ray color
            let color = Self::density_color(density_level);

            painter.rect_filled(level_rect, 4.0, color);

            // Label
            painter.text(
                level_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{} Density", Self::density_name(density_level)),
                egui::FontId::default(),
                Color32::WHITE,
            );
        }

        // Draw influence arrows
        if self.show_arrows {
            for influence in &self.active_influences {
                let from_y = rect.min.y
                    + (8 - influence.from_density) as Float * density_height
                    + density_height / 2.0;
                let to_y = rect.min.y
                    + (8 - influence.to_density) as Float * density_height
                    + density_height / 2.0;

                let arrow_start = Pos2::new(rect.min.x + rect.width() * 0.45, from_y);
                let arrow_end = Pos2::new(rect.min.x + rect.width() * 0.45, to_y);

                // Arrow color based on strength
                let alpha = (100.0 + influence.strength * 155.0) as u8;
                let arrow_color = Color32::from_rgba_unmultiplied(100, 200, 255, alpha);

                // Draw arrow line
                painter.line_segment(
                    [arrow_start, arrow_end],
                    Stroke::new(2.0 + influence.strength * 3.0, arrow_color),
                );

                // Draw arrow head
                let arrow_head_size = 8.0 + influence.strength * 4.0;
                let head_y = if influence.from_density > influence.to_density {
                    to_y - arrow_head_size
                } else {
                    to_y + arrow_head_size
                };

                painter.line_segment(
                    [
                        arrow_end,
                        Pos2::new(arrow_end.x - arrow_head_size / 2.0, head_y),
                    ],
                    Stroke::new(2.0, arrow_color),
                );
                painter.line_segment(
                    [
                        arrow_end,
                        Pos2::new(arrow_end.x + arrow_head_size / 2.0, head_y),
                    ],
                    Stroke::new(2.0, arrow_color),
                );
            }
        }
    }

    /// Render top-down causation explanation
    pub fn render_explanation(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(30, 30, 40, 200));

        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Top-Down Causation",
            egui::FontId::default(),
            Color32::from_rgb(200, 150, 50),
        );

        let info_y = rect.min.y + 35.0;
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y),
            egui::Align2::LEFT_TOP,
            "Higher densities influence lower densities:",
            egui::FontId::default(),
            Color32::from_white_alpha(200),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 20.0),
            egui::Align2::LEFT_TOP,
            "• 8th → 7th → 6th → ... → 1st",
            egui::FontId::monospace(14.0),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 40.0),
            egui::Align2::LEFT_TOP,
            "Consciousness directs matter, not reverse.",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 60.0),
            egui::Align2::LEFT_TOP,
            "This is 'causal inversion' - the spiritual",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );

        painter.text(
            Pos2::new(rect.min.x + 20.0, info_y + 80.0),
            egui::Align2::LEFT_TOP,
            "governs the material.",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }

    /// Get density color
    fn density_color(level: u8) -> Color32 {
        match level {
            1 => Color32::from_rgb(180, 60, 60),   // Red
            2 => Color32::from_rgb(180, 120, 40),  // Orange
            3 => Color32::from_rgb(180, 180, 40),  // Yellow
            4 => Color32::from_rgb(60, 180, 60),   // Green
            5 => Color32::from_rgb(40, 120, 180),  // Blue
            6 => Color32::from_rgb(80, 40, 180),   // Indigo
            7 => Color32::from_rgb(120, 40, 180),  // Violet
            8 => Color32::from_rgb(220, 220, 220), // White
            _ => Color32::GRAY,
        }
    }

    /// Get density name
    fn density_name(level: u8) -> &'static str {
        match level {
            1 => "1st",
            2 => "2nd",
            3 => "3rd",
            4 => "4th",
            5 => "5th",
            6 => "6th",
            7 => "7th",
            8 => "8th",
            _ => "Unknown",
        }
    }
}

// ============================================================================
// DensityTransitionView
// ============================================================================

/// Density Transition View
///
/// Animates density progression, shows polarity requirement thresholds,
/// visualizes harvest readiness indicators, displays light body activation.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Densities are hierarchical material substrates, not individual evolutionary stages"
/// "Entities progress WITHIN their density, not change density"
#[derive(Debug, Clone)]
pub struct DensityTransitionView {
    /// Show progression animation
    pub show_animation: bool,
    /// Current animation phase
    pub animation_phase: Float,
    /// Show polarity thresholds
    pub show_thresholds: bool,
    /// Show harvest indicators
    pub show_harvest: bool,
    /// Show light body activation
    pub show_light_body: bool,
    /// Entity density progress
    pub density_progress: Float,
    /// Polarity state
    pub polarity_state: PolarizationState,
    /// Polarity intensity
    pub polarity_intensity: Float,
    /// Light body activation level
    pub light_body_level: Float,
    /// Harvest readiness
    pub harvest_ready: bool,
    /// Current density
    pub current_density: u8,
}

impl Default for DensityTransitionView {
    fn default() -> Self {
        Self {
            show_animation: true,
            animation_phase: 0.0,
            show_thresholds: true,
            show_harvest: true,
            show_light_body: true,
            density_progress: 0.0,
            polarity_state: PolarizationState::Unpolarized,
            polarity_intensity: 0.0,
            light_body_level: 0.0,
            harvest_ready: false,
            current_density: 3,
        }
    }
}

impl DensityTransitionView {
    /// Create a new density transition view
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with entity state
    pub fn update(
        &mut self,
        progress: &PolarizationProgress,
        density: Density,
        light_body_level: Float,
        harvest_ready: bool,
    ) {
        self.polarity_state = progress.state;
        self.polarity_intensity = progress.intensity as Float;
        self.light_body_level = light_body_level;
        self.harvest_ready = harvest_ready;

        match density {
            Density::First(_) => self.current_density = 1,
            Density::Second(_) => self.current_density = 2,
            Density::Third => self.current_density = 3,
            Density::Fourth => self.current_density = 4,
            Density::Fifth => self.current_density = 5,
            Density::Sixth => self.current_density = 6,
            Density::Seventh => self.current_density = 7,
            Density::Eighth => self.current_density = 8,
        }

        // Update animation
        self.animation_phase = (self.animation_phase + 0.02) % 1.0;
    }

    /// Render density progression animation
    pub fn render_progression(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        // Draw progression bar
        let bar_height = 30.0;
        let bar_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.center().y - bar_height / 2.0),
            Vec2::new(rect.width(), bar_height),
        );

        // Background
        painter.rect_filled(
            bar_rect,
            4.0,
            Color32::from_rgba_unmultiplied(40, 40, 50, 200),
        );

        // Draw density segments
        let segment_width = rect.width() / 8.0;
        for i in 0..8 {
            let seg_x = rect.min.x + i as Float * segment_width;
            let seg_rect = Rect::from_min_size(
                Pos2::new(seg_x, bar_rect.min.y),
                Vec2::new(segment_width - 2.0, bar_height),
            );

            let color = CausalInversionView::density_color((i + 1) as u8);

            // Highlight current density
            if i + 1 == self.current_density {
                painter.rect_filled(seg_rect, 2.0, color);
            } else {
                painter.rect_stroke(seg_rect, 0.0, Stroke::new(1.0, color));
            }

            // Density label
            painter.text(
                seg_rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{}", i + 1),
                egui::FontId::default(),
                if i + 1 == self.current_density {
                    Color32::WHITE
                } else {
                    Color32::from_white_alpha(150)
                },
            );
        }

        // Current progress indicator
        if self.show_animation {
            let progress_x = rect.min.x + self.animation_phase * rect.width();

            // Animated particle
            let particle_color = Color32::from_rgb(255, 220, 100);
            painter.circle_filled(
                Pos2::new(progress_x, bar_rect.center().y),
                5.0,
                particle_color,
            );
        }

        // Label
        painter.text(
            Pos2::new(rect.min.x, bar_rect.max.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Density Progression (entities progress WITHIN density)",
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }

    /// Render polarity thresholds
    pub fn render_polarity_thresholds(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        // Background
        painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(30, 30, 40, 200));

        // Title
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Polarity Thresholds",
            egui::FontId::default(),
            Color32::from_rgb(200, 150, 50),
        );

        // STO threshold (51%)
        let sto_y = rect.min.y + 50.0;
        let sto_threshold = 0.51;

        painter.text(
            Pos2::new(rect.min.x + 20.0, sto_y),
            egui::Align2::LEFT_TOP,
            "STO Harvest: 51%+",
            egui::FontId::default(),
            Color32::from_rgb(80, 200, 80),
        );

        // STO progress bar
        let sto_bar_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 20.0, sto_y + 20.0),
            Vec2::new(rect.width() - 40.0, 15.0),
        );
        painter.rect_filled(
            sto_bar_rect,
            2.0,
            Color32::from_rgba_unmultiplied(50, 50, 50, 200),
        );

        let sto_fill_width = if self.polarity_state.is_sto_leaning() {
            (self.polarity_intensity / sto_threshold).min(1.0) * sto_bar_rect.width()
        } else {
            0.0
        };

        if sto_fill_width > 0.0 {
            let sto_fill_rect = Rect::from_min_size(
                sto_bar_rect.min,
                Vec2::new(sto_fill_width, sto_bar_rect.height()),
            );
            painter.rect_filled(sto_fill_rect, 2.0, Color32::from_rgb(80, 200, 80));
        }

        // Current STO value
        painter.text(
            Pos2::new(sto_bar_rect.max.x, sto_y + 20.0),
            egui::Align2::RIGHT_TOP,
            format!("{:.0}%", self.polarity_intensity * 100.0),
            egui::FontId::monospace(12.0),
            if self.polarity_state.is_sto_leaning() {
                Color32::from_rgb(80, 200, 80)
            } else {
                Color32::from_white_alpha(100)
            },
        );

        // STS threshold (95%)
        let sts_y = sto_y + 60.0;
        let sts_threshold = 0.95;

        painter.text(
            Pos2::new(rect.min.x + 20.0, sts_y),
            egui::Align2::LEFT_TOP,
            "STS Harvest: 95%+",
            egui::FontId::default(),
            Color32::from_rgb(200, 80, 80),
        );

        // STS progress bar
        let sts_bar_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 20.0, sts_y + 20.0),
            Vec2::new(rect.width() - 40.0, 15.0),
        );
        painter.rect_filled(
            sts_bar_rect,
            2.0,
            Color32::from_rgba_unmultiplied(50, 50, 50, 200),
        );

        let sts_fill_width = if self.polarity_state.is_sts_leaning() {
            (self.polarity_intensity / sts_threshold).min(1.0) * sts_bar_rect.width()
        } else {
            0.0
        };

        if sts_fill_width > 0.0 {
            let sts_fill_rect = Rect::from_min_size(
                sts_bar_rect.min,
                Vec2::new(sts_fill_width, sts_bar_rect.height()),
            );
            painter.rect_filled(sts_fill_rect, 2.0, Color32::from_rgb(200, 80, 80));
        }

        // Current STS value
        painter.text(
            Pos2::new(sts_bar_rect.max.x, sts_y + 20.0),
            egui::Align2::RIGHT_TOP,
            format!("{:.0}%", self.polarity_intensity * 100.0),
            egui::FontId::monospace(12.0),
            if self.polarity_state.is_sts_leaning() {
                Color32::from_rgb(200, 80, 80)
            } else {
                Color32::from_white_alpha(100)
            },
        );

        // Current polarity indicator
        let polarity_label = match self.polarity_state {
            PolarizationState::Unpolarized => "Unpolarized",
            PolarizationState::STOLeaning => "STO Leaning",
            PolarizationState::STSLeaning => "STS Leaning",
            PolarizationState::PolarizedSTO => "Polarized STO",
            PolarizationState::PolarizedSTS => "Polarized STS",
            PolarizationState::HarvestableSTO => "HARVESTABLE (STO)",
            PolarizationState::HarvestableSTS => "HARVESTABLE (STS)",
        };

        let polarity_color = if self.polarity_state.is_sto_leaning() {
            Color32::from_rgb(80, 200, 80)
        } else if self.polarity_state.is_sts_leaning() {
            Color32::from_rgb(200, 80, 80)
        } else {
            Color32::from_white_alpha(150)
        };

        painter.text(
            Pos2::new(rect.min.x + rect.width() / 2.0, rect.max.y - 25.0),
            egui::Align2::CENTER_TOP,
            polarity_label,
            egui::FontId::default(),
            polarity_color,
        );
    }

    /// Render harvest readiness indicator
    pub fn render_harvest_indicator(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        // Background circle
        let radius = rect.width().min(rect.height()) * 0.4;
        painter.circle_filled(
            rect.center(),
            radius,
            Color32::from_rgba_unmultiplied(40, 40, 50, 200),
        );

        // Harvest readiness ring
        let ring_color = if self.harvest_ready {
            Color32::from_rgb(255, 200, 50) // Gold when ready
        } else {
            Color32::from_rgb(100, 100, 100) // Gray when not ready
        };

        painter.circle_stroke(rect.center(), radius + 5.0, Stroke::new(4.0, ring_color));

        // Center icon/text
        let center_text = if self.harvest_ready {
            "READY"
        } else {
            "NOT READY"
        };

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            center_text,
            egui::FontId::monospace(18.0),
            if self.harvest_ready {
                Color32::from_rgb(255, 220, 100)
            } else {
                Color32::from_white_alpha(150)
            },
        );

        // Subtitle
        let subtitle = if self.harvest_ready {
            "Ready for density transition"
        } else {
            "Keep making choices"
        };

        painter.text(
            Pos2::new(rect.center().x, rect.center().y + radius * 0.6),
            egui::Align2::CENTER_TOP,
            subtitle,
            egui::FontId::default(),
            Color32::from_white_alpha(180),
        );
    }

    /// Render light body activation
    pub fn render_light_body(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();

        // Background
        painter.rect_filled(rect, 4.0, Color32::from_rgba_unmultiplied(30, 30, 40, 200));

        // Title
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            "Light Body Activation",
            egui::FontId::default(),
            Color32::from_rgb(200, 150, 50),
        );

        // Activation level visualization
        let level = self.light_body_level;
        let num_chakras = 7;
        let chakra_spacing = rect.width() / (num_chakras + 1) as Float;

        for i in 0..num_chakras {
            let x = rect.min.x + (i + 1) as Float * chakra_spacing;
            let y = rect.center().y;

            // Chakra activation threshold (each chakra activates at different levels)
            let threshold = (i + 1) as Float / num_chakras as Float;
            let activated = level >= threshold;

            let color = if activated {
                Self::chakra_color(i)
            } else {
                Color32::from_rgba_unmultiplied(60, 60, 60, 200)
            };

            let radius = 15.0 + (i as Float * 2.0);
            painter.circle_filled(Pos2::new(x, y), radius, color);

            // Chakra label
            painter.text(
                Pos2::new(x, y + radius + 10.0),
                egui::Align2::CENTER_TOP,
                Self::chakra_name(i),
                egui::FontId::monospace(10.0),
                if activated {
                    Color32::WHITE
                } else {
                    Color32::from_white_alpha(100)
                },
            );
        }

        // Overall level percentage
        painter.text(
            Pos2::new(rect.max.x - 10.0, rect.min.y + 10.0),
            egui::Align2::RIGHT_TOP,
            format!("{:.0}%", level * 100.0),
            egui::FontId::monospace(16.0),
            Color32::from_rgb(255, 200, 100),
        );
    }

    /// Get chakra color
    fn chakra_color(index: usize) -> Color32 {
        match index {
            0 => Color32::from_rgb(255, 100, 100), // Root - Red
            1 => Color32::from_rgb(255, 150, 50),  // Sacral - Orange
            2 => Color32::from_rgb(255, 220, 50),  // Solar - Yellow
            3 => Color32::from_rgb(100, 220, 100), // Heart - Green
            4 => Color32::from_rgb(100, 180, 220), // Throat - Blue
            5 => Color32::from_rgb(150, 100, 220), // Third Eye - Indigo
            6 => Color32::from_rgb(200, 100, 220), // Crown - Violet
            _ => Color32::WHITE,
        }
    }

    /// Get chakra name
    fn chakra_name(index: usize) -> &'static str {
        match index {
            0 => "Root",
            1 => "Sacral",
            2 => "Solar",
            3 => "Heart",
            4 => "Throat",
            5 => "3rd Eye",
            6 => "Crown",
            _ => "Unknown",
        }
    }
}

// ============================================================================
// Unified Consciousness Visualization Panel
// ============================================================================

/// Unified consciousness visualization for rendering all consciousness-related views
#[derive(Debug, Clone)]
pub struct ConsciousnessVisualization {
    /// Free will visualizer
    pub free_will: FreeWillVisualizer,
    /// Teleological pull view
    pub teleological: TeleologicalPullView,
    /// Causal inversion view
    pub causal: CausalInversionView,
    /// Density transition view
    pub density: DensityTransitionView,
}

impl Default for ConsciousnessVisualization {
    fn default() -> Self {
        Self {
            free_will: FreeWillVisualizer::new(),
            teleological: TeleologicalPullView::new(),
            causal: CausalInversionView::new(),
            density: DensityTransitionView::new(),
        }
    }
}

impl ConsciousnessVisualization {
    /// Create a new consciousness visualization
    pub fn new() -> Self {
        Self::default()
    }

    /// Update with consciousness kernel data
    pub fn update_with_kernel(&mut self, kernel: &ConsciousnessKernel, density: Density) {
        // Update free will statistics
        let stats = kernel.free_will.get_statistics();

        // Update density transition view
        self.density.update(
            &kernel.polarization_progress,
            density,
            kernel.source_resonance as Float,
            kernel.is_harvest_ready(),
        );
    }

    /// Render the complete consciousness panel
    pub fn render_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("Consciousness Visualization");

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Free Will Section
                ui.collapsing("Free Will", |ui| {
                    ui.label("Non-deterministic choice visualization");

                    let rect = ui.available_rect_before_wrap();
                    let timeline_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, rect.min.y),
                        Vec2::new(rect.width(), 150.0),
                    );
                    self.free_will.render_timeline(ui, timeline_rect);

                    ui.add_space(10.0);

                    let weights_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, timeline_rect.max.y + 10.0),
                        Vec2::new(rect.width(), 150.0),
                    );
                    self.free_will.render_weight_distribution(ui, weights_rect);

                    ui.add_space(10.0);

                    let capacity_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, weights_rect.max.y + 10.0),
                        Vec2::new(150.0, 150.0),
                    );
                    // Note: would need stats here for real rendering
                    // self.free_will.render_capacity_indicator(ui, capacity_rect, &stats);

                    ui.add_space(10.0);

                    let seed_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x + 160.0, weights_rect.max.y + 10.0),
                        Vec2::new(rect.width() - 160.0, 150.0),
                    );
                    self.free_will.render_seed_info(ui, seed_rect);
                });

                // Teleological Pull Section
                ui.collapsing("Teleological Pull", |ui| {
                    ui.label("Spiritual gravity toward unity");

                    let rect = ui.available_rect_before_wrap();
                    let wells_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, rect.min.y),
                        Vec2::new(rect.width() * 0.6, 300.0),
                    );
                    self.teleological.render_gravity_wells(ui, wells_rect);

                    let legend_rect = Rect::from_min_size(
                        Pos2::new(wells_rect.max.x + 10.0, rect.min.y),
                        Vec2::new(rect.width() * 0.35, 300.0),
                    );
                    self.teleological.render_legend(ui, legend_rect);

                    if !self.teleological.trajectory_history.is_empty() {
                        ui.add_space(10.0);
                        let traj_rect = Rect::from_min_size(
                            Pos2::new(rect.min.x, wells_rect.max.y + 10.0),
                            Vec2::new(rect.width(), 150.0),
                        );
                        self.teleological.render_trajectory(ui, traj_rect);
                    }
                });

                // Causal Inversion Section
                ui.collapsing("Top-Down Causation", |ui| {
                    ui.label("Higher densities influence lower densities");

                    let rect = ui.available_rect_before_wrap();
                    let hierarchy_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, rect.min.y),
                        Vec2::new(rect.width() * 0.6, 400.0),
                    );
                    self.causal.render_hierarchy(ui, hierarchy_rect);

                    let explain_rect = Rect::from_min_size(
                        Pos2::new(hierarchy_rect.max.x + 10.0, rect.min.y),
                        Vec2::new(rect.width() * 0.35, 200.0),
                    );
                    self.causal.render_explanation(ui, explain_rect);
                });

                // Density Transition Section
                ui.collapsing("Density & Polarity", |ui| {
                    ui.label("Density progression and harvest readiness");

                    let rect = ui.available_rect_before_wrap();

                    let prog_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, rect.min.y),
                        Vec2::new(rect.width(), 100.0),
                    );
                    self.density.render_progression(ui, prog_rect);

                    ui.add_space(10.0);

                    let thresh_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, prog_rect.max.y + 10.0),
                        Vec2::new(rect.width() * 0.5, 200.0),
                    );
                    self.density.render_polarity_thresholds(ui, thresh_rect);

                    let harvest_rect = Rect::from_min_size(
                        Pos2::new(thresh_rect.max.x + 10.0, prog_rect.max.y + 10.0),
                        Vec2::new(rect.width() * 0.45, 200.0),
                    );
                    self.density.render_harvest_indicator(ui, harvest_rect);

                    ui.add_space(10.0);

                    let body_rect = Rect::from_min_size(
                        Pos2::new(rect.min.x, thresh_rect.max.y + 10.0),
                        Vec2::new(rect.width(), 120.0),
                    );
                    self.density.render_light_body(ui, body_rect);
                });
            });
        });
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_will_visualizer_creation() {
        let viz = FreeWillVisualizer::new();
        assert!(viz.choice_history.is_empty());
        assert!(viz.show_timeline);
        assert!(viz.show_weights);
        assert!(viz.show_capacity);
    }

    #[test]
    fn test_teleological_pull_view_creation() {
        let view = TeleologicalPullView::new();
        assert!(view.show_attractors);
        assert!(view.show_trajectory);
        assert!(view.entity_positions.is_empty());
    }

    #[test]
    fn test_causal_inversion_view_creation() {
        let view = CausalInversionView::new();
        assert!(view.show_hierarchy);
        assert!(view.show_arrows);

        // Check influence matrix is set up for top-down causation
        // Higher density (7) should influence lower density (0)
        assert!(view.influence_matrix[7][0] > 0.0);
        // Same density should have no influence
        assert!(view.influence_matrix[3][3] == 0.0);
    }

    #[test]
    fn test_density_transition_view_creation() {
        let view = DensityTransitionView::new();
        assert!(view.show_animation);
        assert!(view.show_thresholds);
        assert!(view.show_harvest);
        assert!(view.show_light_body);
        assert!(!view.harvest_ready);
        assert_eq!(view.current_density, 3); // Default to 3rd density
    }

    #[test]
    fn test_consciousness_visualization_creation() {
        let viz = ConsciousnessVisualization::new();
        // All sub-views should be initialized
        assert!(viz.free_will.show_timeline);
        assert!(viz.teleological.show_attractors);
        assert!(viz.causal.show_hierarchy);
        assert!(viz.density.show_animation);
    }
}
