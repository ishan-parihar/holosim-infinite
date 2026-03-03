//! Atomic Level Visualization - Phase C.2
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.2:
//! "Design orbital visualization from archetype patterns, Implement attractor field display,
//! Show periodic table as archetype landscape, Visualize mass/charge derivation from archetypes"
//!
//! This module provides visualizations for:
//! - Electron orbitals (s, p, d, f) derived from archetype patterns
//! - Attractor field configurations showing elements as stable field states
//! - Particle signatures (proton/electron/neutron) as archetype patterns
//! - Mass/charge derivation flow animation
//! - Periodic table as attractor basin landscape
//!
//! Key design principles:
//! - Elements emerge as stable attractor fields (NOT fundamental particles)
//! - Mass/charge derive from archetype patterns (NOT hardcoded)
//! - Proton: Mind-complex dominant (A1-A7) → positive charge
//! - Electron: Spirit-complex dominant (A15-A21) → negative charge
//! - Neutron: Balanced Mind+Spirit → neutral
//! - The periodic table is a map of attractor basins in field space
//!
//! Uses f32 for all visualization types (converts from Float/f64 where needed)

use egui::{Color32, FontId, Pos2, Rect, Shape, Stroke, Vec2};
use std::f32::consts::{PI, TAU};

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::atomic_emergence::attractor_field::{
    AttractorBasin, AttractorField, AttractorStability, FieldConfiguration,
};
use crate::holographic_foundation::atomic_emergence::element_attractor::{
    ChargeConfiguration, ElementAttractorField, ElementIdentity,
};
use crate::holographic_foundation::atomic_emergence::particle_derivation::{
    DerivationFactors, ParticleArchetypePattern, ParticleProperties, ParticleType,
};
use crate::holographic_foundation::atomic_emergence::periodic_table_attractors::{
    Block, ElementPosition, PeriodicTableAttractors, ShellConfiguration,
};
use crate::holographic_foundation::quantum_consciousness::quantum_numbers::QuantumNumberSet;
use crate::types::Float;

/// Color palette for atomic visualizations
pub struct AtomicColors;

impl AtomicColors {
    /// Proton color (positive charge - red/orange)
    pub fn proton() -> Color32 {
        Color32::from_rgb(255, 100, 50)
    }

    /// Electron color (negative charge - blue/cyan)
    pub fn electron() -> Color32 {
        Color32::from_rgb(50, 150, 255)
    }

    /// Neutron color (neutral - gray/white)
    pub fn neutron() -> Color32 {
        Color32::from_rgb(200, 200, 200)
    }

    /// Positive charge glow
    pub fn positive_charge() -> Color32 {
        Color32::from_rgb(255, 80, 80)
    }

    /// Negative charge glow
    pub fn negative_charge() -> Color32 {
        Color32::from_rgb(80, 150, 255)
    }

    /// Neutral color
    pub fn neutral() -> Color32 {
        Color32::from_rgb(180, 180, 180)
    }

    /// Attractor basin depth color (deeper = more stable)
    pub fn basin_depth(depth: f32) -> Color32 {
        let intensity = (depth * 255.0).min(255.0) as u8;
        Color32::from_rgb(intensity / 2, intensity, intensity / 3)
    }

    /// Orbital color based on type
    pub fn orbital(orbital_type: &str) -> Color32 {
        match orbital_type {
            "s" => Color32::from_rgb(255, 200, 100), // Yellow
            "p" => Color32::from_rgb(100, 255, 150), // Green
            "d" => Color32::from_rgb(100, 150, 255), // Blue
            "f" => Color32::from_rgb(200, 100, 255), // Purple
            _ => Color32::GRAY,
        }
    }

    /// Element category colors
    pub fn element_category(category: &str) -> Color32 {
        match category {
            "noble_gas" => Color32::from_rgb(200, 150, 255), // Lavender
            "metal" => Color32::from_rgb(180, 140, 100),     // Bronze
            "nonmetal" => Color32::from_rgb(150, 200, 150),  // Sage
            "halogen" => Color32::from_rgb(255, 200, 100),   // Gold
            "alkali" => Color32::from_rgb(255, 100, 100),    // Red
            "alkaline" => Color32::from_rgb(255, 180, 100),  // Orange
            "transition" => Color32::from_rgb(150, 150, 200), // Slate
            "lanthanide" => Color32::from_rgb(200, 150, 200), // Pink
            "actinide" => Color32::from_rgb(255, 150, 150),  // Salmon
            _ => Color32::GRAY,
        }
    }

    /// Block color (s, p, d, f)
    pub fn block(block: Block) -> Color32 {
        match block {
            Block::S => Color32::from_rgb(255, 150, 100), // Orange
            Block::P => Color32::from_rgb(100, 200, 150), // Green
            Block::D => Color32::from_rgb(100, 150, 255), // Blue
            Block::F => Color32::from_rgb(200, 100, 255), // Purple
        }
    }
}

/// Orbital shape data for 3D visualization (projected to 2D)
#[derive(Debug, Clone)]
pub struct OrbitalShape {
    pub orbital_type: String,
    pub n: u32,
    pub l: u32,
    pub points: Vec<Pos2>,
    pub probability_density: Vec<f32>,
}

/// Orbital renderer - visualize electron orbitals from archetype patterns
#[derive(Debug, Clone)]
pub struct OrbitalRenderer {
    /// Show orbital lobes
    pub show_lobes: bool,
    /// Show probability density cloud
    pub show_probability_cloud: bool,
    /// Show quantum numbers
    pub show_quantum_numbers: bool,
    /// Animation time
    pub animation_time: f32,
    /// Rotation angle for 3D effect
    pub rotation: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for OrbitalRenderer {
    fn default() -> Self {
        Self {
            show_lobes: true,
            show_probability_cloud: true,
            show_quantum_numbers: true,
            animation_time: 0.0,
            rotation: 0.0,
            scale: 1.0,
        }
    }
}

impl OrbitalRenderer {
    /// Create a new orbital renderer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
        self.rotation += delta_time * 0.3;
    }

    /// Generate orbital shape points based on quantum numbers
    fn generate_orbital_shape(&self, n: u32, l: u32, center: Pos2, radius: f32) -> Vec<Pos2> {
        let mut points = Vec::new();
        let num_points = 60;

        match l {
            0 => {
                // s-orbital: spherical
                for i in 0..num_points {
                    let angle = (i as f32 / num_points as f32) * TAU;
                    let r = radius * (1.0 + (self.animation_time * 2.0 + angle * 3.0).sin() * 0.1);
                    points.push(Pos2::new(
                        center.x + r * angle.cos(),
                        center.y + r * angle.sin(),
                    ));
                }
            }
            1 => {
                // p-orbital: dumbbell (two lobes)
                for side in [-1.0, 1.0] {
                    for i in 0..(num_points / 2) {
                        let t = (i as f32 / (num_points / 2) as f32) * PI;
                        let lobe_radius = radius * 0.6 * t.sin();
                        let x_offset = side * radius * 0.5 * t.cos();
                        let rotation_factor = self.rotation.cos() * 0.3;
                        points.push(Pos2::new(
                            center.x + x_offset + lobe_radius * rotation_factor,
                            center.y + lobe_radius * t.sin() * 0.5,
                        ));
                    }
                }
            }
            2 => {
                // d-orbital: cloverleaf (four lobes)
                for quadrant in 0..4 {
                    let base_angle = (quadrant as f32) * PI / 2.0 + PI / 4.0;
                    for i in 0..15 {
                        let t = (i as f32 / 15.0) * PI / 2.0;
                        let r = radius * 0.4 * (1.0 + t.sin());
                        let angle = base_angle + t * 0.5;
                        points.push(Pos2::new(
                            center.x + r * angle.cos() * (1.0 + self.rotation.sin() * 0.2),
                            center.y + r * angle.sin() * (1.0 + self.rotation.cos() * 0.2),
                        ));
                    }
                }
            }
            3 => {
                // f-orbital: complex multi-lobed
                for i in 0..num_points {
                    let angle = (i as f32 / num_points as f32) * TAU;
                    let r = radius * 0.3 * (1.0 + (angle * 4.0).sin() * 0.5);
                    let x = center.x + r * angle.cos();
                    let y = center.y + r * angle.sin() * 0.7;
                    points.push(Pos2::new(x, y));
                }
            }
            _ => {
                // Fallback: circle
                for i in 0..num_points {
                    let angle = (i as f32 / num_points as f32) * TAU;
                    points.push(Pos2::new(
                        center.x + radius * 0.3 * angle.cos(),
                        center.y + radius * 0.3 * angle.sin(),
                    ));
                }
            }
        }

        points
    }

    /// Render orbital visualization
    pub fn render(&self, ui: &mut egui::Ui, quantum_numbers: &QuantumNumberSet) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let base_radius = rect.width().min(rect.height()) * 0.35;

        // Animation pulse
        let pulse = 1.0 + (self.animation_time * 2.0).sin() * 0.05;
        let radius = base_radius * pulse;

        // Draw reference circles
        self.draw_reference_circles(&painter, center, radius);

        // Draw orbital shape
        let orbital_points =
            self.generate_orbital_shape(quantum_numbers.n, quantum_numbers.l, center, radius);

        if self.show_lobes && !orbital_points.is_empty() {
            self.draw_orbital_lobes(&painter, &orbital_points, quantum_numbers.l);
        }

        if self.show_probability_cloud {
            self.draw_probability_cloud(&painter, center, radius, quantum_numbers);
        }

        // Draw nucleus
        self.draw_nucleus(&painter, center);

        // Draw quantum numbers
        if self.show_quantum_numbers {
            self.draw_quantum_numbers(&painter, rect, quantum_numbers);
        }

        response
    }

    /// Draw reference circles
    fn draw_reference_circles(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 1..=4 {
            let ring_radius = radius * (i as f32 / 4.0);
            let alpha = 20 + (4 - i) * 10;
            painter.circle_stroke(
                center,
                ring_radius,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 150, 200, alpha)),
            );
        }

        // Center cross
        let cross_size = 5.0;
        painter.line_segment(
            [
                Pos2::new(center.x - cross_size, center.y),
                Pos2::new(center.x + cross_size, center.y),
            ],
            Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 150, 200, 80)),
        );
        painter.line_segment(
            [
                Pos2::new(center.x, center.y - cross_size),
                Pos2::new(center.x, center.y + cross_size),
            ],
            Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 150, 200, 80)),
        );
    }

    /// Draw orbital lobes
    fn draw_orbital_lobes(&self, painter: &egui::Painter, points: &[Pos2], l: u32) {
        let orbital_type = match l {
            0 => "s",
            1 => "p",
            2 => "d",
            3 => "f",
            _ => "?",
        };
        let color = AtomicColors::orbital(orbital_type);

        // Draw filled shape
        if points.len() >= 3 {
            painter.add(Shape::convex_polygon(
                points.to_vec(),
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 60),
                Stroke::new(2.0, color),
            ));
        }

        // Draw points
        for (i, point) in points.iter().enumerate().step_by(3) {
            let pulse = 1.0 + (self.animation_time * 3.0 + i as f32 * 0.1).sin() * 0.2;
            let size = 2.0 + pulse;
            painter.circle_filled(*point, size, color);
        }
    }

    /// Draw probability cloud
    fn draw_probability_cloud(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        quantum_numbers: &QuantumNumberSet,
    ) {
        // Draw probability density as concentric regions
        let n_factor = quantum_numbers.n as f32;
        let num_rings = 5;

        for i in 0..num_rings {
            let r = radius * (i as f32 + 1.0) / num_rings as f32;
            let probability = 1.0 / (n_factor.powi(2) * (i as f32 + 1.0));
            let alpha = (probability * 150.0) as u8;

            painter.circle_filled(
                center,
                r,
                Color32::from_rgba_premultiplied(100, 200, 255, alpha / 3),
            );
        }
    }

    /// Draw nucleus
    fn draw_nucleus(&self, painter: &egui::Painter, center: Pos2) {
        // Nucleus glow
        painter.circle_filled(center, 12.0, AtomicColors::proton());
        painter.circle_filled(center, 8.0, Color32::from_rgb(255, 200, 150));
        painter.circle_filled(center, 4.0, Color32::WHITE);

        // Label
        painter.text(
            center + Vec2::new(0.0, 20.0),
            egui::Align2::CENTER_TOP,
            "Nucleus",
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Draw quantum numbers
    fn draw_quantum_numbers(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        quantum_numbers: &QuantumNumberSet,
    ) {
        let qn = quantum_numbers;
        let info_y = rect.max.y - 60.0;

        let spin_str = if qn.s.is_up() { "↑" } else { "↓" };

        painter.text(
            Pos2::new(rect.min.x + 5.0, info_y),
            egui::Align2::LEFT_TOP,
            format!(
                "n={}, l={}, m={} {}",
                qn.n,
                qn.orbital_type(),
                qn.m,
                spin_str
            ),
            FontId::proportional(10.0),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(rect.min.x + 5.0, info_y + 15.0),
            egui::Align2::LEFT_TOP,
            format!("Shell: {}", qn.shell_name()),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Render multiple orbitals for an element
    pub fn render_element_orbitals(
        &self,
        ui: &mut egui::Ui,
        shell_config: &ShellConfiguration,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(350.0, 350.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let max_radius = rect.width().min(rect.height()) * 0.4;

        // Draw electron configuration
        let mut current_radius = max_radius * 0.2;

        for (i, (&n, &l)) in shell_config
            .principal
            .iter()
            .zip(shell_config.azimuthal.iter())
            .enumerate()
        {
            let radius_step = max_radius / shell_config.shell_count().max(1) as f32;
            current_radius = max_radius * (n as f32) / 7.0;

            // Draw shell ring
            painter.circle_stroke(
                center,
                current_radius,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 150, 200, 100)),
            );

            // Draw orbital for this subshell
            let orbital_points = self.generate_orbital_shape(n, l, center, current_radius * 0.8);
            if !orbital_points.is_empty() {
                self.draw_orbital_lobes(&painter, &orbital_points, l);
            }

            // Draw electron count
            let orbital_label = match l {
                0 => "s",
                1 => "p",
                2 => "d",
                3 => "f",
                _ => "?",
            };
            let label = format!("{}{}", n, orbital_label);
            let label_angle = self.rotation + (i as f32 * 0.5);
            let label_pos = Pos2::new(
                center.x + current_radius * label_angle.cos(),
                center.y + current_radius * label_angle.sin(),
            );

            painter.circle_filled(
                label_pos,
                12.0,
                Color32::from_rgba_premultiplied(50, 50, 70, 200),
            );
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                label,
                FontId::proportional(8.0),
                Color32::WHITE,
            );
        }

        // Draw nucleus
        self.draw_nucleus(&painter, center);

        // Draw configuration string
        painter.text(
            Pos2::new(rect.min.x + 5.0, rect.max.y - 25.0),
            egui::Align2::LEFT_TOP,
            shell_config.electron_configuration_string(),
            FontId::proportional(9.0),
            Color32::from_rgb(200, 200, 255),
        );

        response
    }
}

/// Attractor field visualization data
#[derive(Debug, Clone)]
pub struct AttractorFieldData {
    pub depth: f32,
    pub stability: AttractorStability,
    pub radius: f32,
    pub coherence: f32,
}

impl From<&AttractorBasin> for AttractorFieldData {
    fn from(basin: &AttractorBasin) -> Self {
        Self {
            depth: basin.depth as f32,
            stability: basin.stability,
            radius: basin.radius as f32,
            coherence: 0.7 + basin.depth as f32 * 0.3, // Approximate coherence from depth
        }
    }
}

/// Attractor field visualizer - show elements as attractor field configurations
#[derive(Debug, Clone)]
pub struct AttractorFieldVisualizer {
    /// Show basin depth
    pub show_depth: bool,
    /// Show stability indicator
    pub show_stability: bool,
    /// Show field lines
    pub show_field_lines: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for AttractorFieldVisualizer {
    fn default() -> Self {
        Self {
            show_depth: true,
            show_stability: true,
            show_field_lines: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl AttractorFieldVisualizer {
    /// Create a new visualizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render attractor field for an element
    pub fn render(&self, ui: &mut egui::Ui, element: &ElementAttractorField) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let base_radius = rect.width().min(rect.height()) * 0.35;

        let attractor = element.attractor();
        let basin = attractor.basin();
        let data = AttractorFieldData::from(basin);

        // Draw attractor basin
        if self.show_depth {
            self.draw_basin_depth(&painter, center, base_radius, &data);
        }

        // Draw field lines
        if self.show_field_lines {
            self.draw_field_lines(&painter, center, base_radius, &data);
        }

        // Draw stability indicator
        if self.show_stability {
            self.draw_stability_indicator(&painter, center, base_radius, &data);
        }

        // Draw element info
        self.draw_element_info(&painter, rect, element);

        response
    }

    /// Draw basin depth visualization
    fn draw_basin_depth(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &AttractorFieldData,
    ) {
        let depth_factor = data.depth.min(1.0);

        // Draw concentric depth rings
        for i in (1..=5).rev() {
            let r = radius * (i as f32 / 5.0);
            let depth_intensity = (depth_factor * 255.0) as u8;
            let alpha = (255 - i * 40) as u8;

            let color = Color32::from_rgba_premultiplied(
                (depth_intensity / 2) as u8,
                depth_intensity as u8,
                (depth_intensity / 3) as u8,
                alpha,
            );

            painter.circle_filled(center, r, color);
        }

        // Draw basin boundary
        let boundary_radius = radius * data.radius;
        painter.circle_stroke(
            center,
            boundary_radius,
            Stroke::new(2.0, Color32::from_rgb(100, 200, 255)),
        );
    }

    /// Draw field lines
    fn draw_field_lines(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &AttractorFieldData,
    ) {
        let num_lines = 8;
        let pulse = 1.0 + (self.animation_time * 1.5).sin() * 0.1;

        for i in 0..num_lines {
            let angle = (i as f32 / num_lines as f32) * TAU + self.animation_time * 0.2;
            let r = radius * data.coherence * pulse;

            let start = Pos2::new(
                center.x + r * 0.3 * angle.cos(),
                center.y + r * 0.3 * angle.sin(),
            );
            let end = Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin());

            let strength = (i as f32 / num_lines as f32) * 0.5 + 0.5;
            let color = Color32::from_rgba_premultiplied(
                (200.0 * strength) as u8,
                (220.0 * strength) as u8,
                255,
                (150.0 * strength) as u8,
            );

            painter.line_segment([start, end], Stroke::new(1.5, color));

            // Arrowhead
            let arrow_angle = angle + PI / 8.0;
            let arrow_r = r * 0.9;
            let arrow_end = Pos2::new(
                center.x + arrow_r * angle.cos(),
                center.y + arrow_r * angle.sin(),
            );
            let arrow_tip1 = Pos2::new(
                center.x + arrow_r * 0.85 * arrow_angle.cos(),
                center.y + arrow_r * 0.85 * arrow_angle.sin(),
            );
            painter.line_segment([arrow_tip1, arrow_end], Stroke::new(1.5, color));
        }
    }

    /// Draw stability indicator
    fn draw_stability_indicator(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &AttractorFieldData,
    ) {
        let (color, label) = match data.stability {
            AttractorStability::NobleGas => (Color32::from_rgb(200, 150, 255), "Noble Gas"),
            AttractorStability::Stable => (Color32::from_rgb(100, 255, 100), "Stable"),
            AttractorStability::Metastable => (Color32::from_rgb(255, 255, 100), "Metastable"),
            AttractorStability::Unstable => (Color32::from_rgb(255, 100, 100), "Unstable"),
        };

        let indicator_y = center.y + radius + 20.0;
        painter.circle_filled(Pos2::new(center.x - 50.0, indicator_y), 8.0, color);

        painter.text(
            Pos2::new(center.x - 35.0, indicator_y),
            egui::Align2::LEFT_CENTER,
            label,
            FontId::proportional(10.0),
            color,
        );

        // Coherence bar
        let bar_width = 100.0;
        let bar_height = 6.0;
        let bar_rect = Rect::from_min_size(
            Pos2::new(center.x - bar_width / 2.0, indicator_y + 20.0),
            Vec2::new(bar_width, bar_height),
        );

        painter.rect_stroke(bar_rect, 2.0, Stroke::new(1.0, Color32::GRAY));
        painter.rect_filled(
            Rect::from_min_size(
                bar_rect.min,
                Vec2::new(bar_width * data.coherence, bar_height),
            ),
            1.0,
            Color32::from_rgb(100, 200, 255),
        );
    }

    /// Draw element information
    fn draw_element_info(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        element: &ElementAttractorField,
    ) {
        let info_y = rect.max.y - 70.0;

        // Element symbol and name
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y),
            egui::Align2::LEFT_TOP,
            format!("{} - {}", element.symbol(), element.name()),
            FontId::proportional(12.0),
            Color32::WHITE,
        );

        // Properties
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 18.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Z={}  EN={:.2}",
                element.atomic_number(),
                element.electronegativity()
            ),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Category
        let category = if element.is_noble_gas() {
            "Noble Gas"
        } else if element.is_metal() {
            "Metal"
        } else if element.is_nonmetal() {
            "Nonmetal"
        } else {
            "Other"
        };

        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 32.0),
            egui::Align2::LEFT_TOP,
            format!("{}", category),
            FontId::proportional(9.0),
            AtomicColors::element_category(&category.to_lowercase().replace(' ', "_")),
        );
    }

    /// Render attractor basin comparison
    pub fn render_basin_comparison(
        &self,
        ui: &mut egui::Ui,
        basins: &[(String, AttractorFieldData)],
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 200.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let bar_width = (rect.width() - 40.0) / basins.len().max(1) as f32;
        let max_height = rect.height() - 60.0;

        // Find max depth for normalization
        let max_depth = basins.iter().map(|(_, d)| d.depth).fold(1.0f32, f32::max);

        for (i, (name, data)) in basins.iter().enumerate() {
            let x = rect.min.x + 20.0 + i as f32 * bar_width + bar_width * 0.1;
            let bar_height_actual = (data.depth / max_depth) * max_height * 0.8;
            let y = rect.max.y - 40.0 - bar_height_actual;

            let bar_rect = Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(bar_width * 0.8, bar_height_actual),
            );

            // Color based on stability
            let color = match data.stability {
                AttractorStability::NobleGas => AtomicColors::element_category("noble_gas"),
                AttractorStability::Stable => AtomicColors::element_category("nonmetal"),
                AttractorStability::Metastable => AtomicColors::element_category("halogen"),
                AttractorStability::Unstable => AtomicColors::element_category("alkali"),
            };

            painter.rect_filled(bar_rect, 2.0, color);
            painter.rect_stroke(bar_rect, 2.0, Stroke::new(1.0, Color32::WHITE));

            // Label
            painter.text(
                Pos2::new(x + bar_width * 0.4, rect.max.y - 35.0),
                egui::Align2::CENTER_TOP,
                name,
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );

            // Depth value
            painter.text(
                Pos2::new(x + bar_width * 0.4, y - 5.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{:.2}", data.depth),
                FontId::proportional(8.0),
                Color32::WHITE,
            );
        }

        // Title
        painter.text(
            Pos2::new(rect.center().x, rect.min.y + 10.0),
            egui::Align2::CENTER_TOP,
            "Attractor Basin Depth Comparison",
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        response
    }
}

/// Particle signature data for radar chart
#[derive(Debug, Clone)]
pub struct ParticleSignatureData {
    pub particle_type: ParticleType,
    pub archetype_pattern: [f32; NUM_ARCHETYPES],
    pub mind_dominance: f32,
    pub spirit_dominance: f32,
    pub charge: f32,
    pub mass: f32,
}

impl ParticleSignatureData {
    /// Create from particle properties
    pub fn from_properties(props: &ParticleProperties) -> Self {
        let pattern: [f32; NUM_ARCHETYPES] = props.archetype_contribution.map(|v| v as f32);

        Self {
            particle_type: props.particle_type,
            archetype_pattern: pattern,
            mind_dominance: props.derivation_factors.mind_dominance as f32,
            spirit_dominance: props.derivation_factors.spirit_dominance as f32,
            charge: props.charge as f32,
            mass: props.mass_factor as f32,
        }
    }

    /// Get particle name
    pub fn name(&self) -> &'static str {
        match self.particle_type {
            ParticleType::Proton => "Proton",
            ParticleType::Electron => "Electron",
            ParticleType::Neutron => "Neutron",
            ParticleType::Positron => "Positron",
            ParticleType::Antiproton => "Antiproton",
        }
    }

    /// Get particle color
    pub fn color(&self) -> Color32 {
        match self.particle_type {
            ParticleType::Proton => AtomicColors::proton(),
            ParticleType::Electron => AtomicColors::electron(),
            ParticleType::Neutron => AtomicColors::neutron(),
            ParticleType::Positron => AtomicColors::positive_charge(),
            ParticleType::Antiproton => AtomicColors::negative_charge(),
        }
    }
}

/// Particle signature view - show proton/electron/neutron archetype signatures
#[derive(Debug, Clone)]
pub struct ParticleSignatureView {
    /// Show complex breakdown (Mind/Body/Spirit)
    pub show_complex_breakdown: bool,
    /// Show charge indicator
    pub show_charge: bool,
    /// Show mass indicator
    pub show_mass: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for ParticleSignatureView {
    fn default() -> Self {
        Self {
            show_complex_breakdown: true,
            show_charge: true,
            show_mass: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl ParticleSignatureView {
    /// Create a new view
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render particle signature radar chart
    pub fn render(&self, ui: &mut egui::Ui, data: &ParticleSignatureData) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(280.0, 280.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.35;

        // Draw radar background
        self.draw_radar_background(&painter, center, radius);

        // Draw archetype pattern
        self.draw_archetype_radar(&painter, center, radius, data);

        // Draw complex breakdown
        if self.show_complex_breakdown {
            self.draw_complex_breakdown(&painter, rect, data);
        }

        // Draw particle info
        self.draw_particle_info(&painter, rect, data);

        response
    }

    /// Draw radar chart background
    fn draw_radar_background(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        // Concentric circles
        for i in 1..=4 {
            let r = radius * (i as f32 / 4.0);
            painter.circle_stroke(
                center,
                r,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 100)),
            );
        }

        // Axis lines
        let num_axes = 8;
        for i in 0..num_axes {
            let angle = (i as f32 / num_axes as f32) * TAU - PI / 2.0;
            let end = Pos2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            painter.line_segment(
                [center, end],
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 80)),
            );
        }
    }

    /// Draw archetype pattern on radar
    fn draw_archetype_radar(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &ParticleSignatureData,
    ) {
        // Sample every ~3 archetypes to fit on 8 axes
        let points: Vec<Pos2> = (0..8)
            .map(|i| {
                let idx = (i * 22) / 8; // Map to archetype index
                let activation = data.archetype_pattern[idx.min(21)];
                let angle = (i as f32 / 8.0) * TAU - PI / 2.0;
                let r = radius * (0.2 + activation * 0.8);
                Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect();

        if points.len() >= 3 {
            let color = data.color();
            painter.add(Shape::convex_polygon(
                points.clone(),
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 60),
                Stroke::new(2.0, color),
            ));
        }

        // Draw points
        for (i, point) in points.iter().enumerate() {
            let idx = (i * 22) / 8;
            let activation = data.archetype_pattern[idx.min(21)];
            let size = 2.0 + activation * 4.0;
            painter.circle_filled(*point, size, data.color());
        }
    }

    /// Draw complex breakdown
    fn draw_complex_breakdown(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        data: &ParticleSignatureData,
    ) {
        let breakdown_y = rect.max.y - 60.0;
        let bar_width = 60.0;
        let bar_height = 40.0;

        let complexes = [
            ("Mind", data.mind_dominance, Color32::from_rgb(138, 43, 226)),
            ("Body", 0.5, Color32::from_rgb(255, 140, 0)), // Neutral body
            (
                "Spirit",
                data.spirit_dominance,
                Color32::from_rgb(220, 220, 255),
            ),
        ];

        for (i, (name, value, color)) in complexes.iter().enumerate() {
            let x = rect.min.x + 20.0 + i as f32 * (bar_width + 10.0);
            let y = breakdown_y;

            // Background bar
            let bg_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_width, bar_height));
            painter.rect_filled(
                bg_rect,
                2.0,
                Color32::from_rgba_premultiplied(50, 50, 70, 200),
            );

            // Value bar
            let value_height = bar_height * value.min(1.0);
            let value_rect = Rect::from_min_size(
                Pos2::new(x, y + bar_height - value_height),
                Vec2::new(bar_width, value_height),
            );
            painter.rect_filled(value_rect, 2.0, *color);

            // Label
            painter.text(
                Pos2::new(x + bar_width / 2.0, y + bar_height + 5.0),
                egui::Align2::CENTER_TOP,
                *name,
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );

            // Value
            painter.text(
                Pos2::new(x + bar_width / 2.0, y + bar_height / 2.0),
                egui::Align2::CENTER_CENTER,
                format!("{:.2}", value),
                FontId::proportional(9.0),
                Color32::WHITE,
            );
        }
    }

    /// Draw particle information
    fn draw_particle_info(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        data: &ParticleSignatureData,
    ) {
        let info_y = rect.max.y - 20.0;

        // Particle name and properties
        let charge_str = if data.charge > 0.1 {
            format!("+{:.1}", data.charge)
        } else if data.charge < -0.1 {
            format!("{:.1}", data.charge)
        } else {
            "0".to_string()
        };

        let info_text = if self.show_charge && self.show_mass {
            format!(
                "{} | Charge: {} | Mass: {:.1}",
                data.name(),
                charge_str,
                data.mass
            )
        } else if self.show_charge {
            format!("{} | Charge: {}", data.name(), charge_str)
        } else {
            data.name().to_string()
        };

        painter.text(
            Pos2::new(rect.center().x, info_y),
            egui::Align2::CENTER_TOP,
            info_text,
            FontId::proportional(10.0),
            data.color(),
        );
    }

    /// Render comparison of multiple particles
    pub fn render_particle_comparison(
        &self,
        ui: &mut egui::Ui,
        particles: &[ParticleSignatureData],
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = Pos2::new(rect.min.x + 140.0, rect.center().y);
        let radius = 100.0;

        // Draw radar background
        self.draw_radar_background(&painter, center, radius);

        // Draw each particle
        for (i, particle) in particles.iter().enumerate() {
            let offset_angle = (i as f32 / particles.len() as f32) * PI / 8.0;
            self.draw_comparison_radar(&painter, center, radius, particle, offset_angle);
        }

        // Legend
        let legend_x = rect.max.x - 140.0;
        let legend_y = rect.min.y + 30.0;

        painter.text(
            Pos2::new(legend_x, legend_y),
            egui::Align2::LEFT_TOP,
            "Particle Signatures",
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        for (i, particle) in particles.iter().enumerate() {
            let y = legend_y + 25.0 + i as f32 * 20.0;
            painter.circle_filled(Pos2::new(legend_x, y), 6.0, particle.color());
            painter.text(
                Pos2::new(legend_x + 12.0, y),
                egui::Align2::LEFT_CENTER,
                particle.name(),
                FontId::proportional(9.0),
                Color32::LIGHT_GRAY,
            );
        }

        response
    }

    /// Draw comparison radar with offset
    fn draw_comparison_radar(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &ParticleSignatureData,
        offset_angle: f32,
    ) {
        let points: Vec<Pos2> = (0..8)
            .map(|i| {
                let idx = (i * 22) / 8;
                let activation = data.archetype_pattern[idx.min(21)];
                let angle = (i as f32 / 8.0) * TAU - PI / 2.0 + offset_angle;
                let r = radius * (0.2 + activation * 0.7);
                Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect();

        if points.len() >= 3 {
            let color = data.color();
            painter.add(Shape::convex_polygon(
                points.clone(),
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 40),
                Stroke::new(1.5, color),
            ));
        }
    }
}

/// Mass/charge derivation flow data
#[derive(Debug, Clone)]
pub struct MassChargeDerivationData {
    pub mind_dominance: f32,
    pub spirit_dominance: f32,
    pub coherence_depth: f32,
    pub derived_mass: f32,
    pub derived_charge: f32,
    pub derivation_step: usize,
}

impl MassChargeDerivationData {
    /// Create from derivation factors
    pub fn from_factors(factors: &DerivationFactors) -> Self {
        Self {
            mind_dominance: factors.mind_dominance as f32,
            spirit_dominance: factors.spirit_dominance as f32,
            coherence_depth: factors.coherence_depth as f32,
            derived_mass: 1.0, // Will be calculated during animation
            derived_charge: factors.charge_polarity as f32,
            derivation_step: 0,
        }
    }
}

/// Mass/charge derivation - animate mass/charge derivation from archetypes
#[derive(Debug, Clone)]
pub struct MassChargeDerivation {
    /// Animation progress (0.0 to 1.0)
    pub animation_progress: f32,
    /// Current step in derivation
    pub current_step: usize,
    /// Show flow arrows
    pub show_flow: bool,
    /// Scale factor
    pub scale: f32,
}

impl Default for MassChargeDerivation {
    fn default() -> Self {
        Self {
            animation_progress: 0.0,
            current_step: 0,
            show_flow: true,
            scale: 1.0,
        }
    }
}

impl MassChargeDerivation {
    /// Create a new derivation visualizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_progress += delta_time * 0.5;
        if self.animation_progress >= 1.0 {
            self.animation_progress = 0.0;
            self.current_step = (self.current_step + 1) % 4;
        }
    }

    /// Reset animation
    pub fn reset(&mut self) {
        self.animation_progress = 0.0;
        self.current_step = 0;
    }

    /// Render mass/charge derivation flow
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        data: &MassChargeDerivationData,
        particle_type: ParticleType,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(450.0, 250.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Draw derivation flow
        self.draw_derivation_flow(&painter, rect, center, data, particle_type);

        // Draw result
        self.draw_derivation_result(&painter, rect, data, particle_type);

        response
    }

    /// Draw derivation flow diagram
    fn draw_derivation_flow(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        center: Pos2,
        data: &MassChargeDerivationData,
        particle_type: ParticleType,
    ) {
        // Source boxes (Mind and Spirit complexes)
        let mind_box = Rect::from_min_size(
            Pos2::new(rect.min.x + 20.0, rect.min.y + 40.0),
            Vec2::new(80.0, 50.0),
        );
        let spirit_box = Rect::from_min_size(
            Pos2::new(rect.min.x + 20.0, rect.min.y + 140.0),
            Vec2::new(80.0, 50.0),
        );

        // Draw source boxes
        painter.rect_filled(
            mind_box,
            3.0,
            Color32::from_rgba_premultiplied(138, 43, 226, 100),
        );
        painter.rect_stroke(
            mind_box,
            3.0,
            Stroke::new(2.0, Color32::from_rgb(138, 43, 226)),
        );
        painter.text(
            mind_box.center(),
            egui::Align2::CENTER_CENTER,
            "Mind\nComplex",
            FontId::proportional(10.0),
            Color32::WHITE,
        );

        painter.rect_filled(
            spirit_box,
            3.0,
            Color32::from_rgba_premultiplied(220, 220, 255, 100),
        );
        painter.rect_stroke(
            spirit_box,
            3.0,
            Stroke::new(2.0, Color32::from_rgb(220, 220, 255)),
        );
        painter.text(
            spirit_box.center(),
            egui::Align2::CENTER_CENTER,
            "Spirit\nComplex",
            FontId::proportional(10.0),
            Color32::from_rgb(50, 50, 80),
        );

        // Coherence depth box
        let coherence_box = Rect::from_min_size(
            Pos2::new(center.x - 40.0, center.y - 25.0),
            Vec2::new(80.0, 50.0),
        );
        painter.rect_filled(
            coherence_box,
            3.0,
            Color32::from_rgba_premultiplied(100, 200, 150, 100),
        );
        painter.rect_stroke(
            coherence_box,
            3.0,
            Stroke::new(2.0, Color32::from_rgb(100, 200, 150)),
        );
        painter.text(
            coherence_box.center(),
            egui::Align2::CENTER_CENTER,
            format!("Coherence\n{:.2}", data.coherence_depth),
            FontId::proportional(9.0),
            Color32::WHITE,
        );

        // Result box
        let result_box = Rect::from_min_size(
            Pos2::new(rect.max.x - 100.0, center.y - 35.0),
            Vec2::new(80.0, 70.0),
        );
        let result_color = match particle_type {
            ParticleType::Proton => AtomicColors::proton(),
            ParticleType::Electron => AtomicColors::electron(),
            ParticleType::Neutron => AtomicColors::neutron(),
            ParticleType::Positron => AtomicColors::positive_charge(),
            ParticleType::Antiproton => AtomicColors::negative_charge(),
        };
        painter.rect_filled(
            result_box,
            3.0,
            Color32::from_rgba_premultiplied(
                result_color.r(),
                result_color.g(),
                result_color.b(),
                150,
            ),
        );
        painter.rect_stroke(result_box, 3.0, Stroke::new(2.0, result_color));

        // Flow arrows with animation
        if self.show_flow {
            let progress = self.animation_progress;

            // Mind to coherence
            self.draw_animated_arrow(
                painter,
                Pos2::new(mind_box.max.x, mind_box.center().y),
                Pos2::new(coherence_box.min.x, coherence_box.center().y - 10.0),
                progress,
                Color32::from_rgb(138, 43, 226),
            );

            // Spirit to coherence
            self.draw_animated_arrow(
                painter,
                Pos2::new(spirit_box.max.x, spirit_box.center().y),
                Pos2::new(coherence_box.min.x, coherence_box.center().y + 10.0),
                progress,
                Color32::from_rgb(180, 180, 220),
            );

            // Coherence to result
            self.draw_animated_arrow(
                painter,
                Pos2::new(coherence_box.max.x, coherence_box.center().y),
                Pos2::new(result_box.min.x, result_box.center().y),
                progress,
                Color32::from_rgb(100, 200, 150),
            );
        }
    }

    /// Draw animated arrow
    fn draw_animated_arrow(
        &self,
        painter: &egui::Painter,
        start: Pos2,
        end: Pos2,
        progress: f32,
        color: Color32,
    ) {
        // Main line
        painter.line_segment([start, end], Stroke::new(2.0, color));

        // Animated dot
        let dot_pos = Pos2::new(
            start.x + (end.x - start.x) * progress,
            start.y + (end.y - start.y) * progress,
        );
        painter.circle_filled(dot_pos, 4.0, color);

        // Arrowhead
        let angle = (end.y - start.y).atan2(end.x - start.x);
        let arrow_angle1 = angle - PI / 6.0;
        let arrow_angle2 = angle + PI / 6.0;
        let arrow_length = 10.0;

        let tip1 = Pos2::new(
            end.x - arrow_length * arrow_angle1.cos(),
            end.y - arrow_length * arrow_angle1.sin(),
        );
        let tip2 = Pos2::new(
            end.x - arrow_length * arrow_angle2.cos(),
            end.y - arrow_length * arrow_angle2.sin(),
        );

        painter.line_segment([tip1, end], Stroke::new(2.0, color));
        painter.line_segment([tip2, end], Stroke::new(2.0, color));
    }

    /// Draw derivation result
    fn draw_derivation_result(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        data: &MassChargeDerivationData,
        particle_type: ParticleType,
    ) {
        let result_x = rect.max.x - 100.0;
        let result_y = rect.center().y - 35.0;

        let particle_name = match particle_type {
            ParticleType::Proton => "Proton",
            ParticleType::Electron => "Electron",
            ParticleType::Neutron => "Neutron",
            ParticleType::Positron => "Positron",
            ParticleType::Antiproton => "Antiproton",
        };

        // Particle name
        painter.text(
            Pos2::new(result_x + 40.0, result_y + 15.0),
            egui::Align2::CENTER_TOP,
            particle_name,
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        // Charge
        let charge_str = if data.derived_charge > 0.1 {
            format!("Charge: +{:.1}", data.derived_charge)
        } else if data.derived_charge < -0.1 {
            format!("Charge: {:.1}", data.derived_charge)
        } else {
            "Charge: 0".to_string()
        };
        painter.text(
            Pos2::new(result_x + 40.0, result_y + 35.0),
            egui::Align2::CENTER_TOP,
            charge_str,
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Mass
        painter.text(
            Pos2::new(result_x + 40.0, result_y + 50.0),
            egui::Align2::CENTER_TOP,
            format!("Mass: {:.1}", data.derived_mass),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Explanation
        let explanation = match particle_type {
            ParticleType::Proton => "Mind-dominant → Positive",
            ParticleType::Electron => "Spirit-dominant → Negative",
            ParticleType::Neutron => "Balanced → Neutral",
            ParticleType::Positron => "Inverted Spirit → Positive",
            ParticleType::Antiproton => "Inverted Mind → Negative",
        };

        painter.text(
            Pos2::new(rect.center().x, rect.max.y - 15.0),
            egui::Align2::CENTER_TOP,
            explanation,
            FontId::proportional(9.0),
            Color32::from_rgba_premultiplied(200, 200, 200, 150),
        );
    }

    /// Render derivation comparison between particle types
    pub fn render_derivation_comparison(
        &self,
        ui: &mut egui::Ui,
        particles: &[(ParticleType, MassChargeDerivationData)],
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(500.0, 200.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let bar_width = (rect.width() - 60.0) / particles.len().max(1) as f32;

        // Find max mass for normalization
        let max_mass = particles
            .iter()
            .map(|(_, d)| d.derived_mass)
            .fold(1.0f32, f32::max);

        for (i, (particle_type, data)) in particles.iter().enumerate() {
            let x = rect.min.x + 30.0 + i as f32 * bar_width;
            let center_x = x + bar_width * 0.5;

            let particle_name = match particle_type {
                ParticleType::Proton => "Proton",
                ParticleType::Electron => "Electron",
                ParticleType::Neutron => "Neutron",
                ParticleType::Positron => "Positron",
                ParticleType::Antiproton => "Antiproton",
            };

            let color = match particle_type {
                ParticleType::Proton => AtomicColors::proton(),
                ParticleType::Electron => AtomicColors::electron(),
                ParticleType::Neutron => AtomicColors::neutron(),
                ParticleType::Positron => AtomicColors::positive_charge(),
                ParticleType::Antiproton => AtomicColors::negative_charge(),
            };

            // Mass bar
            let mass_height = (data.derived_mass / max_mass) * 100.0;
            let mass_rect = Rect::from_min_size(
                Pos2::new(center_x - 15.0, rect.max.y - 60.0 - mass_height),
                Vec2::new(30.0, mass_height),
            );
            painter.rect_filled(mass_rect, 2.0, color);
            painter.rect_stroke(mass_rect, 2.0, Stroke::new(1.0, Color32::WHITE));

            // Charge indicator
            let charge_y = rect.max.y - 50.0;
            let charge_color = if data.derived_charge > 0.1 {
                AtomicColors::positive_charge()
            } else if data.derived_charge < -0.1 {
                AtomicColors::negative_charge()
            } else {
                AtomicColors::neutral()
            };

            painter.circle_filled(Pos2::new(center_x, charge_y), 8.0, charge_color);
            let charge_str = if data.derived_charge > 0.1 {
                "+"
            } else if data.derived_charge < -0.1 {
                "-"
            } else {
                "0"
            };
            painter.text(
                Pos2::new(center_x, charge_y),
                egui::Align2::CENTER_CENTER,
                charge_str,
                FontId::proportional(10.0),
                Color32::WHITE,
            );

            // Label
            painter.text(
                Pos2::new(center_x, rect.max.y - 25.0),
                egui::Align2::CENTER_TOP,
                particle_name,
                FontId::proportional(9.0),
                Color32::LIGHT_GRAY,
            );
        }

        // Title
        painter.text(
            Pos2::new(rect.center().x, rect.min.y + 10.0),
            egui::Align2::CENTER_TOP,
            "Mass/Charge Derivation from Archetype Patterns",
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        response
    }
}

/// Periodic table element display data
#[derive(Debug, Clone)]
pub struct ElementDisplayData {
    pub atomic_number: u32,
    pub symbol: String,
    pub name: String,
    pub electronegativity: f32,
    pub block: Block,
    pub period: u32,
    pub group: u32,
}

impl From<&ElementAttractorField> for ElementDisplayData {
    fn from(element: &ElementAttractorField) -> Self {
        Self {
            atomic_number: element.atomic_number(),
            symbol: element.symbol().to_string(),
            name: element.name().to_string(),
            electronegativity: element.electronegativity() as f32,
            block: Block::S, // Will be updated from position
            period: 1,
            group: 1,
        }
    }
}

/// Periodic table landscape - show periodic table as archetype landscape
#[derive(Debug, Clone)]
pub struct PeriodicTableLandscape {
    /// Show attractor basins
    pub show_basins: bool,
    /// Show element properties
    pub show_properties: bool,
    /// Color by category
    pub color_by_category: bool,
    /// Selected element
    pub selected_element: Option<u32>,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for PeriodicTableLandscape {
    fn default() -> Self {
        Self {
            show_basins: true,
            show_properties: true,
            color_by_category: true,
            selected_element: None,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl PeriodicTableLandscape {
    /// Create a new periodic table landscape
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Get element color based on properties
    fn element_color(&self, element: &ElementAttractorField) -> Color32 {
        if self.color_by_category {
            if element.is_noble_gas() {
                AtomicColors::element_category("noble_gas")
            } else if element.is_metal() {
                AtomicColors::element_category("metal")
            } else if element.is_nonmetal() {
                AtomicColors::element_category("nonmetal")
            } else if element.identity().is_halogen() {
                AtomicColors::element_category("halogen")
            } else {
                AtomicColors::element_category("other")
            }
        } else {
            // Color by electronegativity
            let en = element.electronegativity() as f32;
            let t = (en / 4.0).min(1.0);
            Color32::from_rgb(
                (100.0 + t * 100.0) as u8,
                (100.0 + t * 100.0) as u8,
                (200.0 - t * 100.0) as u8,
            )
        }
    }

    /// Render periodic table as archetype landscape
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        periodic_table: &PeriodicTableAttractors,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(700.0, 400.0) * self.scale, egui::Sense::click());

        let rect = response.rect;
        let cell_width = (rect.width() - 40.0) / 18.0; // 18 groups
        let cell_height = (rect.height() - 60.0) / 7.0; // 7 periods

        // Draw grid
        self.draw_periodic_grid(&painter, rect, periodic_table, cell_width, cell_height);

        // Draw lanthanides and actinides
        self.draw_separated_series(&painter, rect, periodic_table, cell_width, cell_height);

        // Draw title
        painter.text(
            Pos2::new(rect.center().x, rect.min.y + 10.0),
            egui::Align2::CENTER_TOP,
            "Periodic Table as Attractor Basin Landscape",
            FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Draw legend
        self.draw_legend(&painter, rect);

        response
    }

    /// Draw main periodic table grid
    fn draw_periodic_grid(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        periodic_table: &PeriodicTableAttractors,
        cell_width: f32,
        cell_height: f32,
    ) {
        let start_x = rect.min.x + 20.0;
        let start_y = rect.min.y + 40.0;

        for z in 1..=118 {
            if let Some(element) = periodic_table.get(z) {
                if let Some(position) = periodic_table.position(z) {
                    // Skip lanthanides (57-71) and actinides (89-103) - they go below
                    if (57..=71).contains(&z) || (89..=103).contains(&z) {
                        continue;
                    }

                    let x = start_x + (position.group - 1) as f32 * cell_width;
                    let y = start_y + (position.period - 1) as f32 * cell_height;

                    let cell_rect = Rect::from_min_size(
                        Pos2::new(x, y),
                        Vec2::new(cell_width - 2.0, cell_height - 2.0),
                    );

                    self.draw_element_cell(painter, cell_rect, element, z);
                }
            }
        }

        // Draw period labels
        for period in 1..=7 {
            let y = start_y + (period - 1) as f32 * cell_height + cell_height / 2.0;
            painter.text(
                Pos2::new(rect.min.x + 5.0, y),
                egui::Align2::RIGHT_CENTER,
                format!("{}", period),
                FontId::proportional(9.0),
                Color32::GRAY,
            );
        }

        // Draw group labels
        for group in 1..=18 {
            let x = start_x + (group - 1) as f32 * cell_width + cell_width / 2.0;
            painter.text(
                Pos2::new(x, start_y - 8.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{}", group),
                FontId::proportional(9.0),
                Color32::GRAY,
            );
        }
    }

    /// Draw separated lanthanide and actinide series
    fn draw_separated_series(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        periodic_table: &PeriodicTableAttractors,
        cell_width: f32,
        cell_height: f32,
    ) {
        let start_x = rect.min.x + 20.0 + 2.0 * cell_width; // Start at group 3
        let lanthanide_y = rect.min.y + 40.0 + 5.0 * cell_height + 20.0;
        let actinide_y = lanthanide_y + cell_height + 5.0;

        // Label
        painter.text(
            Pos2::new(rect.min.x + 20.0, lanthanide_y + cell_height / 2.0),
            egui::Align2::LEFT_CENTER,
            "Ln",
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );
        painter.text(
            Pos2::new(rect.min.x + 20.0, actinide_y + cell_height / 2.0),
            egui::Align2::LEFT_CENTER,
            "An",
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );

        // Lanthanides (57-71)
        for (i, z) in (57..=71).enumerate() {
            if let Some(element) = periodic_table.get(z) {
                let x = start_x + i as f32 * cell_width;
                let cell_rect = Rect::from_min_size(
                    Pos2::new(x, lanthanide_y),
                    Vec2::new(cell_width - 2.0, cell_height - 2.0),
                );
                self.draw_element_cell(painter, cell_rect, element, z);
            }
        }

        // Actinides (89-103)
        for (i, z) in (89..=103).enumerate() {
            if let Some(element) = periodic_table.get(z) {
                let x = start_x + i as f32 * cell_width;
                let cell_rect = Rect::from_min_size(
                    Pos2::new(x, actinide_y),
                    Vec2::new(cell_width - 2.0, cell_height - 2.0),
                );
                self.draw_element_cell(painter, cell_rect, element, z);
            }
        }
    }

    /// Draw single element cell
    fn draw_element_cell(
        &self,
        painter: &egui::Painter,
        cell_rect: Rect,
        element: &ElementAttractorField,
        z: u32,
    ) {
        let color = self.element_color(element);

        // Attractor basin effect
        if self.show_basins {
            let attractor = element.attractor();
            let depth = attractor.basin().depth as f32;
            let pulse = 1.0 + (self.animation_time * 2.0 + z as f32 * 0.1).sin() * 0.05;

            // Basin glow
            let glow_radius = 3.0 + depth * 5.0 * pulse;
            painter.rect_filled(
                Rect::from_min_size(
                    cell_rect.min - Vec2::new(glow_radius, glow_radius),
                    cell_rect.size() + Vec2::new(glow_radius * 2.0, glow_radius * 2.0),
                ),
                3.0,
                Color32::from_rgba_premultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    (50.0 * depth) as u8,
                ),
            );
        }

        // Cell background
        painter.rect_filled(cell_rect, 2.0, color);

        // Border (highlight if selected)
        let border_color = if self.selected_element == Some(z) {
            Color32::WHITE
        } else {
            Color32::from_rgba_premultiplied(0, 0, 0, 100)
        };
        let border_width = if self.selected_element == Some(z) {
            2.0
        } else {
            1.0
        };
        painter.rect_stroke(cell_rect, 2.0, Stroke::new(border_width, border_color));

        // Atomic number
        painter.text(
            cell_rect.min + Vec2::new(3.0, 3.0),
            egui::Align2::LEFT_TOP,
            format!("{}", z),
            FontId::proportional(7.0),
            Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );

        // Symbol
        painter.text(
            cell_rect.center(),
            egui::Align2::CENTER_CENTER,
            element.symbol(),
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        // Properties
        if self.show_properties && cell_rect.width() > 25.0 {
            painter.text(
                cell_rect.max - Vec2::new(3.0, 3.0),
                egui::Align2::RIGHT_BOTTOM,
                format!("{:.1}", element.electronegativity()),
                FontId::proportional(6.0),
                Color32::from_rgba_premultiplied(255, 255, 255, 200),
            );
        }
    }

    /// Draw legend
    fn draw_legend(&self, painter: &egui::Painter, rect: Rect) {
        let legend_y = rect.max.y - 35.0;
        let items = [
            ("Noble Gas", AtomicColors::element_category("noble_gas")),
            ("Metal", AtomicColors::element_category("metal")),
            ("Nonmetal", AtomicColors::element_category("nonmetal")),
            ("Halogen", AtomicColors::element_category("halogen")),
        ];

        let item_width = 80.0;
        let start_x = rect.min.x + 20.0;

        for (i, (label, color)) in items.iter().enumerate() {
            let x = start_x + i as f32 * item_width;

            painter.rect_filled(
                Rect::from_min_size(Pos2::new(x, legend_y), Vec2::new(12.0, 12.0)),
                2.0,
                *color,
            );
            painter.text(
                Pos2::new(x + 16.0, legend_y + 6.0),
                egui::Align2::LEFT_CENTER,
                *label,
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );
        }
    }

    /// Render element detail view
    pub fn render_element_detail(
        &self,
        ui: &mut egui::Ui,
        element: &ElementAttractorField,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 350.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Element card
        let color = self.element_color(element);
        let card_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            Vec2::new(rect.width() - 20.0, rect.height() - 20.0),
        );

        // Background
        painter.rect_filled(
            card_rect,
            5.0,
            Color32::from_rgba_premultiplied(40, 40, 50, 250),
        );
        painter.rect_stroke(card_rect, 5.0, Stroke::new(2.0, color));

        // Header
        painter.text(
            Pos2::new(card_rect.center().x, card_rect.min.y + 20.0),
            egui::Align2::CENTER_TOP,
            format!("{} ({})", element.name(), element.symbol()),
            FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Atomic number
        painter.text(
            Pos2::new(card_rect.center().x, card_rect.min.y + 45.0),
            egui::Align2::CENTER_TOP,
            format!("Atomic Number: {}", element.atomic_number()),
            FontId::proportional(11.0),
            Color32::LIGHT_GRAY,
        );

        // Properties
        let info_y = card_rect.min.y + 80.0;
        let properties = [
            format!("Electronegativity: {:.2}", element.electronegativity()),
            format!("Atomic Radius: {:.1} pm", element.atomic_radius()),
            format!("Ionization Energy: {:.2} eV", element.ionization_energy()),
            format!("Formation Energy: {:.2} eV", element.formation_energy()),
        ];

        for (i, prop) in properties.iter().enumerate() {
            painter.text(
                Pos2::new(card_rect.min.x + 15.0, info_y + i as f32 * 20.0),
                egui::Align2::LEFT_TOP,
                prop,
                FontId::proportional(9.0),
                Color32::LIGHT_GRAY,
            );
        }

        // Category
        let category = if element.is_noble_gas() {
            "Noble Gas"
        } else if element.is_metal() {
            "Metal"
        } else if element.is_nonmetal() {
            "Nonmetal"
        } else {
            "Other"
        };

        painter.text(
            Pos2::new(card_rect.center().x, card_rect.max.y - 30.0),
            egui::Align2::CENTER_BOTTOM,
            format!("Category: {}", category),
            FontId::proportional(10.0),
            color,
        );

        // Charge info
        let charge = element.charge();
        let charge_str = if charge.is_neutral() {
            "Neutral"
        } else if charge.is_cation() {
            "Cation (+)"
        } else {
            "Anion (-)"
        };

        painter.text(
            Pos2::new(card_rect.center().x, card_rect.max.y - 15.0),
            egui::Align2::CENTER_BOTTOM,
            format!("Charge: {}", charge_str),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        response
    }

    /// Render block comparison view
    pub fn render_block_comparison(
        &self,
        ui: &mut egui::Ui,
        periodic_table: &PeriodicTableAttractors,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 200.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let blocks = [Block::S, Block::P, Block::D, Block::F];
        let bar_width = (rect.width() - 60.0) / 4.0;

        for (i, block) in blocks.iter().enumerate() {
            let elements = periodic_table.elements_by_block(*block);
            let count = elements.len() as f32;
            let avg_en: f32 = elements
                .iter()
                .map(|e| e.electronegativity() as f32)
                .sum::<f32>()
                / count.max(1.0);

            let x = rect.min.x + 30.0 + i as f32 * bar_width;
            let bar_height = (count / 50.0) * (rect.height() - 60.0);
            let y = rect.max.y - 40.0 - bar_height;

            let color = AtomicColors::block(*block);

            // Bar
            let bar_rect =
                Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_width * 0.8, bar_height));
            painter.rect_filled(bar_rect, 2.0, color);
            painter.rect_stroke(bar_rect, 2.0, Stroke::new(1.0, Color32::WHITE));

            // Block label
            painter.text(
                Pos2::new(x + bar_width * 0.4, rect.max.y - 35.0),
                egui::Align2::CENTER_TOP,
                format!("{:?}", block),
                FontId::proportional(11.0),
                color,
            );

            // Count
            painter.text(
                Pos2::new(x + bar_width * 0.4, y - 5.0),
                egui::Align2::CENTER_BOTTOM,
                format!("{:.0}", count),
                FontId::proportional(9.0),
                Color32::WHITE,
            );

            // Avg EN
            painter.text(
                Pos2::new(x + bar_width * 0.4, y + bar_height / 2.0),
                egui::Align2::CENTER_CENTER,
                format!("{:.1}", avg_en),
                FontId::proportional(8.0),
                Color32::from_rgba_premultiplied(0, 0, 0, 150),
            );
        }

        // Title
        painter.text(
            Pos2::new(rect.center().x, rect.min.y + 10.0),
            egui::Align2::CENTER_TOP,
            "Element Distribution by Block",
            FontId::proportional(12.0),
            Color32::WHITE,
        );

        response
    }
}

/// Complete atomic visualization panel
#[derive(Debug, Clone)]
pub struct AtomicVisualizationPanel {
    pub orbital_renderer: OrbitalRenderer,
    pub attractor_visualizer: AttractorFieldVisualizer,
    pub particle_signature_view: ParticleSignatureView,
    pub mass_charge_derivation: MassChargeDerivation,
    pub periodic_table_landscape: PeriodicTableLandscape,
}

impl Default for AtomicVisualizationPanel {
    fn default() -> Self {
        Self {
            orbital_renderer: OrbitalRenderer::new(),
            attractor_visualizer: AttractorFieldVisualizer::new(),
            particle_signature_view: ParticleSignatureView::new(),
            mass_charge_derivation: MassChargeDerivation::new(),
            periodic_table_landscape: PeriodicTableLandscape::new(),
        }
    }
}

impl AtomicVisualizationPanel {
    /// Create a new panel
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all visualizations
    pub fn update(&mut self, delta_time: f32) {
        self.orbital_renderer.update(delta_time);
        self.attractor_visualizer.update(delta_time);
        self.particle_signature_view.update(delta_time);
        self.mass_charge_derivation.update(delta_time);
        self.periodic_table_landscape.update(delta_time);
    }

    /// Render the complete panel
    pub fn render(&self, ui: &mut egui::Ui, periodic_table: &PeriodicTableAttractors) {
        ui.heading("Atomic Level Visualization - Phase C.2");
        ui.label("Elements as stable attractor fields, mass/charge derived from archetypes");
        ui.separator();

        // Orbital visualization
        ui.collapsing("Electron Orbitals", |ui| {
            if let Some(element) = periodic_table.get(6) {
                if let Some(shells) = periodic_table.shell_configuration(6) {
                    self.orbital_renderer.render_element_orbitals(ui, shells);
                }
            }
        });

        ui.separator();

        // Attractor field visualization
        ui.collapsing("Attractor Field Configuration", |ui| {
            ui.horizontal(|ui| {
                if let Some(element) = periodic_table.get(8) {
                    self.attractor_visualizer.render(ui, element);
                }
                if let Some(element) = periodic_table.get(26) {
                    self.attractor_visualizer.render(ui, element);
                }
            });
        });

        ui.separator();

        // Particle signatures
        ui.collapsing("Particle Archetype Signatures", |ui| {
            let proton_pattern = ParticleArchetypePattern::proton_pattern();
            let proton_props =
                ParticleProperties::derive_from_archetype(ParticleType::Proton, proton_pattern);
            let proton_data = ParticleSignatureData::from_properties(&proton_props);

            let electron_pattern = ParticleArchetypePattern::electron_pattern();
            let electron_props =
                ParticleProperties::derive_from_archetype(ParticleType::Electron, electron_pattern);
            let electron_data = ParticleSignatureData::from_properties(&electron_props);

            let neutron_pattern = ParticleArchetypePattern::neutron_pattern();
            let neutron_props =
                ParticleProperties::derive_from_archetype(ParticleType::Neutron, neutron_pattern);
            let neutron_data = ParticleSignatureData::from_properties(&neutron_props);

            let particles = vec![proton_data, electron_data, neutron_data];
            self.particle_signature_view
                .render_particle_comparison(ui, &particles);
        });

        ui.separator();

        // Mass/charge derivation
        ui.collapsing("Mass/Charge Derivation from Archetypes", |ui| {
            let proton_factors = DerivationFactors {
                mind_dominance: 0.85,
                spirit_dominance: 0.25,
                body_balance: 0.55,
                coherence_depth: 0.75,
                charge_polarity: 0.6,
            };
            let proton_data = MassChargeDerivationData::from_factors(&proton_factors);

            let electron_factors = DerivationFactors {
                mind_dominance: 0.25,
                spirit_dominance: 0.90,
                body_balance: 0.40,
                coherence_depth: 0.45,
                charge_polarity: -0.65,
            };
            let electron_data = MassChargeDerivationData::from_factors(&electron_factors);

            let neutron_factors = DerivationFactors {
                mind_dominance: 0.55,
                spirit_dominance: 0.55,
                body_balance: 0.60,
                coherence_depth: 0.55,
                charge_polarity: 0.0,
            };
            let neutron_data = MassChargeDerivationData::from_factors(&neutron_factors);

            let particles = vec![
                (ParticleType::Proton, proton_data),
                (ParticleType::Electron, electron_data),
                (ParticleType::Neutron, neutron_data),
            ];
            self.mass_charge_derivation
                .render_derivation_comparison(ui, &particles);
        });

        ui.separator();

        // Periodic table landscape
        ui.collapsing("Periodic Table as Attractor Landscape", |ui| {
            self.periodic_table_landscape.render(ui, periodic_table);
        });

        ui.separator();

        // Key principles
        ui.collapsing("Design Principles", |ui| {
            ui.label("• Elements emerge as stable attractor fields (NOT fundamental particles)");
            ui.label("• Mass/charge derive from archetype patterns (NOT hardcoded)");
            ui.label("• Proton: Mind-complex dominant (A1-A7) → positive charge");
            ui.label("• Electron: Spirit-complex dominant (A15-A21) → negative charge");
            ui.label("• Neutron: Balanced Mind+Spirit → neutral charge");
            ui.label("• The periodic table is a map of attractor basins in field space");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_colors() {
        assert!(AtomicColors::proton().r() > 200); // Proton is reddish
        assert!(AtomicColors::electron().b() > 150); // Electron is blueish
        assert!(AtomicColors::neutron().r() > 150); // Neutron is grayish
    }

    #[test]
    fn test_orbital_renderer() {
        let mut renderer = OrbitalRenderer::new();
        renderer.update(0.1);
        assert!(renderer.animation_time > 0.0);
    }

    #[test]
    fn test_attractor_visualizer() {
        let mut viz = AttractorFieldVisualizer::new();
        viz.update(0.1);
        assert!(viz.animation_time > 0.0);
    }

    #[test]
    fn test_particle_signature_data() {
        let proton_pattern = ParticleArchetypePattern::proton_pattern();
        let props = ParticleProperties::derive_from_archetype(ParticleType::Proton, proton_pattern);
        let data = ParticleSignatureData::from_properties(&props);

        assert_eq!(data.particle_type, ParticleType::Proton);
        assert!(data.charge > 0.0); // Proton has positive charge
    }

    #[test]
    fn test_mass_charge_derivation() {
        let mut derivation = MassChargeDerivation::new();
        derivation.update(0.1);
        assert!(derivation.animation_progress >= 0.0);
    }

    #[test]
    fn test_periodic_table_landscape() {
        let mut landscape = PeriodicTableLandscape::new();
        landscape.update(0.1);
        assert!(landscape.animation_time > 0.0);
    }

    #[test]
    fn test_particle_comparison_colors() {
        let proton = ParticleSignatureData {
            particle_type: ParticleType::Proton,
            archetype_pattern: [0.5; NUM_ARCHETYPES],
            mind_dominance: 0.8,
            spirit_dominance: 0.2,
            charge: 1.0,
            mass: 1836.0,
        };
        let electron = ParticleSignatureData {
            particle_type: ParticleType::Electron,
            archetype_pattern: [0.5; NUM_ARCHETYPES],
            mind_dominance: 0.2,
            spirit_dominance: 0.8,
            charge: -1.0,
            mass: 1.0,
        };

        assert!(proton.charge > 0.0);
        assert!(electron.charge < 0.0);
        assert!(proton.mind_dominance > proton.spirit_dominance);
        assert!(electron.spirit_dominance > electron.mind_dominance);
    }

    #[test]
    fn test_panel_update() {
        let mut panel = AtomicVisualizationPanel::new();
        panel.update(0.1);
        assert!(panel.orbital_renderer.animation_time > 0.0);
    }
}
