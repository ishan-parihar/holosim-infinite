//! Molecular Level Visualization - Phase C.3
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.3:
//! "Design molecular visualization from archetype patterns, Implement bond visualization
//! with type-based colors, Show molecular geometry as field interference minima,
//! Visualize functional groups as archetype signatures, Simultaneous molecular/planetary emergence"
//!
//! This module provides visualizations for:
//! - Chemical bonds derived from archetype interference patterns
//! - Molecular geometry showing VSEPR as field interference minima
//! - Functional group archetype pattern radar charts
//! - Reactivity indicators based on archetype profiles
//! - Simultaneous molecular and planetary emergence visualization
//!
//! Key design principles:
//! - Bonds form through ARCHETYPE RESONANCE (not just electronegativity)
//! - Molecular Geometry = Field interference minima (VSEPR is consequence, not cause)
//! - Bond types: Covalent (similar patterns), Ionic (complementary), Metallic (collective),
//!   Hydrogen (catalyst bridge), Van der Waals (weak field)
//! - Simultaneous emergence: molecules AND planets from same field
//!
//! Uses f32 for all visualization types (converts from Float/f64 where needed)

use egui::{Color32, FontId, Pos2, Rect, Shape, Stroke, Vec2};
use std::f32::consts::{PI, TAU};

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::molecular_emergence::bond_formation::{
    ArchetypeBond, BondOrder, BondType, MolecularInterferencePattern,
};
use crate::holographic_foundation::molecular_emergence::functional_groups::{
    FunctionalGroup, FunctionalGroupResonance, ReactivityProfile,
};
use crate::holographic_foundation::molecular_emergence::molecular_geometry::{
    GeometryPrediction, MolecularShape,
};
use crate::holographic_foundation::molecular_emergence::simultaneous_emergence::{
    MolecularManifestation, MolecularPlanetaryPair, MolecularPlanetarySystem, PlanetType,
    PlanetaryEmergence,
};

/// Color palette for molecular visualizations
pub struct MolecularColors;

impl MolecularColors {
    /// Covalent bond color (similar archetype patterns - green/blue)
    pub fn covalent() -> Color32 {
        Color32::from_rgb(100, 200, 150)
    }

    /// Ionic bond color (complementary patterns - purple/magenta)
    pub fn ionic() -> Color32 {
        Color32::from_rgb(200, 100, 200)
    }

    /// Metallic bond color (collective resonance - silver/gray)
    pub fn metallic() -> Color32 {
        Color32::from_rgb(180, 180, 200)
    }

    /// Hydrogen bond color (catalyst bridge - orange/yellow)
    pub fn hydrogen() -> Color32 {
        Color32::from_rgb(255, 180, 100)
    }

    /// Van der Waals bond color (weak field - light blue)
    pub fn van_der_waals() -> Color32 {
        Color32::from_rgb(150, 200, 255)
    }

    /// Coordinate bond color (donor-acceptor - cyan)
    pub fn coordinate() -> Color32 {
        Color32::from_rgb(100, 220, 220)
    }

    /// Aromatic bond color (cyclic resonance - pink)
    pub fn aromatic() -> Color32 {
        Color32::from_rgb(255, 150, 180)
    }

    /// Get color for bond type
    pub fn bond_type(bond_type: BondType) -> Color32 {
        match bond_type {
            BondType::Covalent => Self::covalent(),
            BondType::Ionic => Self::ionic(),
            BondType::Metallic => Self::metallic(),
            BondType::Hydrogen => Self::hydrogen(),
            BondType::VanDerWaals => Self::van_der_waals(),
            BondType::Coordinate => Self::coordinate(),
            BondType::Aromatic => Self::aromatic(),
        }
    }

    /// Bond order color (single=thin, double=medium, triple=thick)
    pub fn bond_order(order: BondOrder) -> f32 {
        match order {
            BondOrder::Partial => 1.0,
            BondOrder::Single => 2.0,
            BondOrder::Double => 3.5,
            BondOrder::Triple => 5.0,
            BondOrder::Aromatic => 4.0,
        }
    }

    /// Interference constructive region (positive - warm colors)
    pub fn constructive() -> Color32 {
        Color32::from_rgb(255, 150, 100)
    }

    /// Interference destructive region (negative - cool colors)
    pub fn destructive() -> Color32 {
        Color32::from_rgb(100, 150, 255)
    }

    /// Planet type colors
    pub fn planet_type(planet_type: PlanetType) -> Color32 {
        match planet_type {
            PlanetType::Rocky => Color32::from_rgb(180, 140, 100),
            PlanetType::GasGiant => Color32::from_rgb(255, 200, 150),
            PlanetType::IceGiant => Color32::from_rgb(150, 200, 255),
            PlanetType::Dwarf => Color32::from_rgb(160, 160, 160),
            PlanetType::Terrestrial => Color32::from_rgb(100, 180, 100),
            PlanetType::Ocean => Color32::from_rgb(50, 150, 255),
            PlanetType::Desert => Color32::from_rgb(255, 200, 100),
        }
    }

    /// Functional group reactivity colors
    pub fn electrophilic() -> Color32 {
        Color32::from_rgb(255, 100, 100)
    }

    pub fn nucleophilic() -> Color32 {
        Color32::from_rgb(100, 255, 150)
    }

    pub fn acidic() -> Color32 {
        Color32::from_rgb(255, 200, 100)
    }

    pub fn basic() -> Color32 {
        Color32::from_rgb(150, 150, 255)
    }
}

/// Bond visualization data for rendering
#[derive(Debug, Clone)]
pub struct BondRenderData {
    pub element1_symbol: String,
    pub element2_symbol: String,
    pub bond_type: BondType,
    pub bond_order: BondOrder,
    pub length: f32,
    pub energy: f32,
    pub archetype_similarity: f32,
    pub polarity: f32,
    pub stability: f32,
}

impl From<&ArchetypeBond> for BondRenderData {
    fn from(bond: &ArchetypeBond) -> Self {
        Self {
            element1_symbol: bond.element1.symbol().to_string(),
            element2_symbol: bond.element2.symbol().to_string(),
            bond_type: bond.bond_type,
            bond_order: bond.bond_order,
            length: bond.length as f32,
            energy: bond.energy as f32,
            archetype_similarity: bond.archetype_similarity as f32,
            polarity: bond.polarity() as f32,
            stability: bond.bond_strength() as f32,
        }
    }
}

/// Bond renderer - visualize chemical bonds from archetype interference patterns
#[derive(Debug, Clone)]
pub struct BondRenderer {
    /// Show bond type colors
    pub show_bond_types: bool,
    /// Show bond order as thickness
    pub show_bond_order: bool,
    /// Show archetype similarity
    pub show_similarity: bool,
    /// Show interference pattern
    pub show_interference: bool,
    /// Animation time for pulse effects
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for BondRenderer {
    fn default() -> Self {
        Self {
            show_bond_types: true,
            show_bond_order: true,
            show_similarity: true,
            show_interference: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl BondRenderer {
    /// Create a new bond renderer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render a single bond
    pub fn render(&self, ui: &mut egui::Ui, data: &BondRenderData) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(200.0, 120.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Draw bond line
        let bond_color = MolecularColors::bond_type(data.bond_type);
        let stroke_width = MolecularColors::bond_order(data.bond_order);

        // Animated pulse
        let pulse = 1.0 + (self.animation_time * 2.0).sin() * 0.1;
        let actual_stroke = stroke_width * pulse;

        // Draw main bond line
        let bond_y = center.y;
        let bond_left = rect.min.x + 30.0;
        let bond_right = rect.max.x - 30.0;

        painter.line_segment(
            [Pos2::new(bond_left, bond_y), Pos2::new(bond_right, bond_y)],
            Stroke::new(actual_stroke, bond_color),
        );

        // Draw element circles at ends
        let elem_radius = 15.0;
        painter.circle_filled(
            Pos2::new(bond_left, bond_y),
            elem_radius,
            Color32::from_rgba_premultiplied(50, 50, 70, 200),
        );
        painter.circle_stroke(
            Pos2::new(bond_left, bond_y),
            elem_radius,
            Stroke::new(2.0, bond_color),
        );

        painter.circle_filled(
            Pos2::new(bond_right, bond_y),
            elem_radius,
            Color32::from_rgba_premultiplied(50, 50, 70, 200),
        );
        painter.circle_stroke(
            Pos2::new(bond_right, bond_y),
            elem_radius,
            Stroke::new(2.0, bond_color),
        );

        // Draw element symbols
        painter.text(
            Pos2::new(bond_left, bond_y),
            egui::Align2::CENTER_CENTER,
            &data.element1_symbol,
            FontId::proportional(10.0),
            Color32::WHITE,
        );
        painter.text(
            Pos2::new(bond_right, bond_y),
            egui::Align2::CENTER_CENTER,
            &data.element2_symbol,
            FontId::proportional(10.0),
            Color32::WHITE,
        );

        // Draw bond type label
        let bond_type_name = match data.bond_type {
            BondType::Covalent => "Covalent",
            BondType::Ionic => "Ionic",
            BondType::Metallic => "Metallic",
            BondType::Hydrogen => "Hydrogen",
            BondType::VanDerWaals => "Van der Waals",
            BondType::Coordinate => "Coordinate",
            BondType::Aromatic => "Aromatic",
        };

        painter.text(
            Pos2::new(center.x, rect.max.y - 30.0),
            egui::Align2::CENTER_TOP,
            format!("{} ({})", bond_type_name, data.bond_order.numeric()),
            FontId::proportional(9.0),
            bond_color,
        );

        // Draw similarity indicator
        if self.show_similarity {
            let sim_bar_width = 80.0;
            let sim_bar_x = center.x - sim_bar_width / 2.0;
            let sim_bar_y = rect.max.y - 18.0;

            let sim_rect = Rect::from_min_size(
                Pos2::new(sim_bar_x, sim_bar_y),
                Vec2::new(sim_bar_width, 6.0),
            );
            painter.rect_stroke(sim_rect, 1.0, Stroke::new(1.0, Color32::GRAY));

            let sim_fill_width = sim_bar_width * data.archetype_similarity;
            painter.rect_filled(
                Rect::from_min_size(sim_rect.min, Vec2::new(sim_fill_width, 6.0)),
                1.0,
                bond_color,
            );

            painter.text(
                Pos2::new(sim_bar_x + sim_bar_width + 5.0, sim_bar_y),
                egui::Align2::LEFT_CENTER,
                format!("{:.0}%", data.archetype_similarity * 100.0),
                FontId::proportional(7.0),
                Color32::LIGHT_GRAY,
            );
        }

        response
    }

    /// Render interference pattern visualization
    pub fn render_interference(
        &self,
        ui: &mut egui::Ui,
        interference: &MolecularInterferencePattern,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(250.0, 200.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.35;

        // Draw radar background
        for i in 1..=4 {
            let r = radius * (i as f32 / 4.0);
            painter.circle_stroke(
                center,
                r,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 80)),
            );
        }

        // Draw archetype interference
        let points: Vec<Pos2> = (0..8)
            .map(|i| {
                let idx = (i * 22) / 8;
                let interference_val = interference.archetype_interference[idx.min(21)] as f32;
                let angle = (i as f32 / 8.0) * TAU - PI / 2.0;
                let r = radius * (0.2 + interference_val.abs() * 0.8);
                Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect();

        if points.len() >= 3 {
            let color = if interference.net_interference > 0.0 {
                MolecularColors::constructive()
            } else {
                MolecularColors::destructive()
            };
            painter.add(Shape::convex_polygon(
                points.clone(),
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 60),
                Stroke::new(2.0, color),
            ));
        }

        // Draw info
        painter.text(
            Pos2::new(rect.min.x + 5.0, rect.max.y - 35.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Net: {:.2} | Phase: {:.2}",
                interference.net_interference as f32, interference.phase_alignment as f32
            ),
            FontId::proportional(9.0),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(rect.min.x + 5.0, rect.max.y - 20.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Constructive: {} | Destructive: {}",
                interference.constructive_regions.len(),
                interference.destructive_regions.len()
            ),
            FontId::proportional(8.0),
            Color32::LIGHT_GRAY,
        );

        response
    }

    /// Render multiple bonds comparison
    pub fn render_bond_comparison(
        &self,
        ui: &mut egui::Ui,
        bonds: &[BondRenderData],
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 150.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let bond_width = (rect.width() - 40.0) / bonds.len().max(1) as f32;

        for (i, bond) in bonds.iter().enumerate() {
            let x = rect.min.x + 20.0 + i as f32 * bond_width;
            let center_x = x + bond_width * 0.5;

            // Bond line
            let bond_color = MolecularColors::bond_type(bond.bond_type);
            let stroke_width = MolecularColors::bond_order(bond.bond_order);
            let y = rect.min.y + 50.0;

            painter.line_segment(
                [Pos2::new(x + 10.0, y), Pos2::new(x + bond_width - 10.0, y)],
                Stroke::new(stroke_width, bond_color),
            );

            // Elements
            painter.circle_filled(
                Pos2::new(x + 10.0, y),
                10.0,
                Color32::from_rgba_premultiplied(50, 50, 70, 200),
            );
            painter.circle_filled(
                Pos2::new(x + bond_width - 10.0, y),
                10.0,
                Color32::from_rgba_premultiplied(50, 50, 70, 200),
            );

            // Labels
            painter.text(
                Pos2::new(x + 10.0, y),
                egui::Align2::CENTER_CENTER,
                &bond.element1_symbol,
                FontId::proportional(7.0),
                Color32::WHITE,
            );
            painter.text(
                Pos2::new(x + bond_width - 10.0, y),
                egui::Align2::CENTER_CENTER,
                &bond.element2_symbol,
                FontId::proportional(7.0),
                Color32::WHITE,
            );

            // Energy bar
            let bar_y = rect.max.y - 30.0;
            let bar_width = bond_width * 0.6;
            let energy_height = (bond.energy / 600.0 * 20.0).min(20.0);

            let bar_rect = Rect::from_min_size(
                Pos2::new(center_x - bar_width / 2.0, bar_y - energy_height),
                Vec2::new(bar_width, energy_height),
            );
            painter.rect_filled(bar_rect, 2.0, bond_color);

            painter.text(
                Pos2::new(center_x, bar_y + 3.0),
                egui::Align2::CENTER_TOP,
                format!("{:.0}", bond.energy),
                FontId::proportional(7.0),
                Color32::LIGHT_GRAY,
            );
        }

        response
    }
}

/// Helper to get constructive color
/// Molecular geometry visualization data
#[derive(Debug, Clone)]
pub struct GeometryRenderData {
    pub shape: MolecularShape,
    pub bond_angles: Vec<f32>,
    pub steric_number: u32,
    pub bonding_pairs: u32,
    pub lone_pairs: u32,
    pub confidence: f32,
    pub interference_minima_count: usize,
}

impl From<&GeometryPrediction> for GeometryRenderData {
    fn from(pred: &GeometryPrediction) -> Self {
        Self {
            shape: *pred.shape(),
            bond_angles: pred
                .bond_angles()
                .iter()
                .map(|a| a.angle_degrees as f32)
                .collect(),
            steric_number: pred.steric_number(),
            bonding_pairs: pred.bonding_pairs(),
            lone_pairs: pred.lone_pairs(),
            confidence: pred.confidence() as f32,
            interference_minima_count: pred.interference_minima().len(),
        }
    }
}

/// Molecular geometry view - show VSEPR as field interference minima
#[derive(Debug, Clone)]
pub struct MolecularGeometryView {
    /// Show interference minima
    pub show_interference_minima: bool,
    /// Show bond angles
    pub show_bond_angles: bool,
    /// Show shape name
    pub show_shape_name: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for MolecularGeometryView {
    fn default() -> Self {
        Self {
            show_interference_minima: true,
            show_bond_angles: true,
            show_shape_name: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl MolecularGeometryView {
    /// Create a new geometry view
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render molecular geometry
    pub fn render(&self, ui: &mut egui::Ui, data: &GeometryRenderData) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let base_radius = rect.width().min(rect.height()) * 0.3;

        // Draw central atom
        let central_radius = 20.0;
        painter.circle_filled(center, central_radius, Color32::from_rgb(100, 100, 150));
        painter.circle_stroke(center, central_radius, Stroke::new(2.0, Color32::WHITE));

        // Draw bonded atoms based on geometry
        let num_bonds = data.bonding_pairs.min(6);
        let angles = self.calculate_geometry_angles(&data.shape, num_bonds);

        for (i, &angle) in angles.iter().enumerate() {
            let rad = angle.to_radians();
            let x = center.x + base_radius * rad.cos();
            let y = center.y + base_radius * rad.sin() * 0.7; // Flatten for 2D

            // Bond line
            let bond_color = MolecularColors::covalent();
            painter.line_segment([center, Pos2::new(x, y)], Stroke::new(3.0, bond_color));

            // Atom circle
            painter.circle_filled(
                Pos2::new(x, y),
                12.0,
                Color32::from_rgba_premultiplied(50, 80, 120, 200),
            );
            painter.circle_stroke(Pos2::new(x, y), 12.0, Stroke::new(1.5, bond_color));

            // Atom label
            painter.text(
                Pos2::new(x, y),
                egui::Align2::CENTER_CENTER,
                format!("X{}", i + 1),
                FontId::proportional(7.0),
                Color32::WHITE,
            );

            // Draw angle if available
            if self.show_bond_angles && i < data.bond_angles.len() {
                let angle_text = format!("{:.0}°", data.bond_angles[i]);
                let angle_offset = 25.0;
                let angle_x = center.x + angle_offset * rad.cos();
                let angle_y = center.y + angle_offset * rad.sin() * 0.7;

                painter.text(
                    Pos2::new(angle_x, angle_y),
                    egui::Align2::CENTER_CENTER,
                    angle_text,
                    FontId::proportional(7.0),
                    Color32::from_rgb(200, 200, 100),
                );
            }
        }

        // Draw lone pairs as smaller circles
        for i in 0..data.lone_pairs {
            let angle = (i as f32 * 72.0 + 180.0).to_radians();
            let x = center.x + base_radius * 0.7 * angle.cos();
            let y = center.y + base_radius * 0.7 * angle.sin() * 0.7;

            painter.circle_filled(
                Pos2::new(x, y),
                8.0,
                Color32::from_rgba_premultiplied(255, 200, 100, 150),
            );
        }

        // Draw interference minima visualization
        if self.show_interference_minima {
            self.draw_interference_minima(&painter, center, base_radius, data);
        }

        // Draw shape name and info
        if self.show_shape_name {
            let shape_name = match data.shape {
                MolecularShape::Linear => "Linear",
                MolecularShape::Bent => "Bent",
                MolecularShape::TrigonalPlanar => "Trigonal Planar",
                MolecularShape::TrigonalPyramidal => "Trigonal Pyramidal",
                MolecularShape::Tetrahedral => "Tetrahedral",
                MolecularShape::SquarePlanar => "Square Planar",
                MolecularShape::SquarePyramidal => "Square Pyramidal",
                MolecularShape::TrigonalBipyramidal => "Trigonal Bipyramidal",
                MolecularShape::Octahedral => "Octahedral",
                MolecularShape::Seesaw => "Seesaw",
                MolecularShape::TShaped => "T-Shaped",
                MolecularShape::Unknown => "Unknown",
            };

            painter.text(
                Pos2::new(rect.min.x + 10.0, rect.max.y - 45.0),
                egui::Align2::LEFT_TOP,
                shape_name,
                FontId::proportional(12.0),
                Color32::WHITE,
            );

            painter.text(
                Pos2::new(rect.min.x + 10.0, rect.max.y - 28.0),
                egui::Align2::LEFT_TOP,
                format!(
                    "Steric: {} | Bonds: {} | LP: {} | Conf: {:.0}%",
                    data.steric_number,
                    data.bonding_pairs,
                    data.lone_pairs,
                    data.confidence * 100.0
                ),
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );

            // Interference minima indicator
            painter.text(
                Pos2::new(rect.min.x + 10.0, rect.max.y - 15.0),
                egui::Align2::LEFT_TOP,
                format!("Interference Minima: {}", data.interference_minima_count),
                FontId::proportional(8.0),
                Color32::from_rgb(150, 200, 255),
            );
        }

        response
    }

    /// Calculate geometry angles based on shape
    fn calculate_geometry_angles(&self, shape: &MolecularShape, num_bonds: u32) -> Vec<f32> {
        let angles: Vec<f32> = match shape {
            MolecularShape::Linear => vec![0.0, 180.0],
            MolecularShape::Bent => vec![0.0, 180.0], // Simplified
            MolecularShape::TrigonalPlanar => vec![0.0, 120.0, 240.0],
            MolecularShape::TrigonalPyramidal => vec![0.0, 109.5, 219.5],
            MolecularShape::Tetrahedral => vec![0.0, 109.5, 180.0, 250.5],
            MolecularShape::TrigonalBipyramidal => vec![0.0, 90.0, 180.0, 270.0, 360.0],
            MolecularShape::Octahedral => (0..6).map(|i| i as f32 * 60.0).collect(),
            _ => (0..num_bonds)
                .map(|i| i as f32 * 360.0 / num_bonds as f32)
                .collect(),
        };
        angles
    }

    /// Draw interference minima visualization
    fn draw_interference_minima(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        data: &GeometryRenderData,
    ) {
        let num_minima = data.interference_minima_count.min(12);

        for i in 0..num_minima {
            let angle = (i as f32 / num_minima as f32) * TAU + self.animation_time * 0.2;
            let r = radius * 1.3;
            let x = center.x + r * angle.cos();
            let y = center.y + r * angle.sin() * 0.7;

            let pulse = 1.0 + (self.animation_time * 3.0 + i as f32 * 0.5).sin() * 0.3;
            let size = 4.0 * pulse;

            // Draw as small interference points
            let is_constructive = i % 3 != 0;
            let color = if is_constructive {
                MolecularColors::constructive()
            } else {
                MolecularColors::destructive()
            };

            painter.circle_filled(Pos2::new(x, y), size, color);
        }
    }
}

/// Functional group visualization data
#[derive(Debug, Clone)]
pub struct FunctionalGroupRenderData {
    pub group_name: String,
    pub formula: String,
    pub archetype_pattern: [f32; NUM_ARCHETYPES],
    pub reactivity: ReactivityRenderData,
}

#[derive(Debug, Clone)]
pub struct ReactivityRenderData {
    pub electrophilicity: f32,
    pub nucleophilicity: f32,
    pub acidity: f32,
    pub basicity: f32,
    pub dominant_type: String,
}

impl From<&ReactivityProfile> for ReactivityRenderData {
    fn from(profile: &ReactivityProfile) -> Self {
        Self {
            electrophilicity: profile.electrophilicity as f32,
            nucleophilicity: profile.nucleophilicity as f32,
            acidity: profile.acidity as f32,
            basicity: profile.basicity as f32,
            dominant_type: profile.dominant_reactivity().to_string(),
        }
    }
}

/// Functional group visualizer - show archetype patterns for functional groups
#[derive(Debug, Clone)]
pub struct FunctionalGroupVisualizer {
    /// Show radar chart
    pub show_radar: bool,
    /// Show reactivity bars
    pub show_reactivity: bool,
    /// Show archetype labels
    pub show_archetype_labels: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for FunctionalGroupVisualizer {
    fn default() -> Self {
        Self {
            show_radar: true,
            show_reactivity: true,
            show_archetype_labels: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl FunctionalGroupVisualizer {
    /// Create a new functional group visualizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render functional group with archetype pattern
    pub fn render(&self, ui: &mut egui::Ui, data: &FunctionalGroupRenderData) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(350.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;

        // Draw group name and formula
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 10.0),
            egui::Align2::LEFT_TOP,
            &data.group_name,
            FontId::proportional(14.0),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.min.y + 28.0),
            egui::Align2::LEFT_TOP,
            format!("Formula: {}", data.formula),
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );

        // Draw radar chart
        if self.show_radar {
            let radar_center = Pos2::new(rect.min.x + 120.0, rect.min.y + 120.0);
            let radar_radius = 70.0;
            self.draw_radar_chart(
                &painter,
                radar_center,
                radar_radius,
                &data.archetype_pattern,
            );
        }

        // Draw reactivity bars
        if self.show_reactivity {
            self.draw_reactivity_bars(&painter, rect, &data.reactivity);
        }

        response
    }

    /// Draw archetype radar chart
    fn draw_radar_chart(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        pattern: &[f32; NUM_ARCHETYPES],
    ) {
        // Background circles
        for i in 1..=4 {
            let r = radius * (i as f32 / 4.0);
            painter.circle_stroke(
                center,
                r,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(80, 80, 120, 60)),
            );
        }

        // Axis lines (sample every 3 archetypes)
        let num_axes = 8;
        for i in 0..num_axes {
            let angle = (i as f32 / num_axes as f32) * TAU - PI / 2.0;
            let end = Pos2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            painter.line_segment(
                [center, end],
                Stroke::new(1.0, Color32::from_rgba_premultiplied(80, 80, 120, 50)),
            );
        }

        // Pattern polygon
        let points: Vec<Pos2> = (0..num_axes)
            .map(|i| {
                let idx = (i * 22) / 8;
                let value = pattern[idx.min(21)];
                let angle = (i as f32 / num_axes as f32) * TAU - PI / 2.0;
                let r = radius * (0.2 + value * 0.8);
                Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect();

        if points.len() >= 3 {
            painter.add(Shape::convex_polygon(
                points.clone(),
                Color32::from_rgba_premultiplied(100, 200, 255, 50),
                Stroke::new(2.0, Color32::from_rgb(100, 200, 255)),
            ));
        }

        // Points
        for (i, point) in points.iter().enumerate() {
            let idx = (i * 22) / 8;
            let value = pattern[idx.min(21)];
            let size = 2.0 + value * 4.0;
            painter.circle_filled(*point, size, Color32::from_rgb(150, 220, 255));
        }

        // Label
        painter.text(
            Pos2::new(center.x, center.y + radius + 15.0),
            egui::Align2::CENTER_TOP,
            "Archetype Pattern",
            FontId::proportional(8.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Draw reactivity bars
    fn draw_reactivity_bars(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        reactivity: &ReactivityRenderData,
    ) {
        let bar_x = rect.max.x - 120.0;
        let bar_width = 100.0;
        let bar_height = 12.0;
        let start_y = rect.min.y + 60.0;
        let spacing = 25.0;

        let reactivities = [
            (
                "Electrophilic",
                reactivity.electrophilicity,
                MolecularColors::electrophilic(),
            ),
            (
                "Nucleophilic",
                reactivity.nucleophilicity,
                MolecularColors::nucleophilic(),
            ),
            ("Acidic", reactivity.acidity, MolecularColors::acidic()),
            ("Basic", reactivity.basicity, MolecularColors::basic()),
        ];

        for (i, (name, value, color)) in reactivities.iter().enumerate() {
            let y = start_y + i as f32 * spacing;

            // Background
            let bg_rect =
                Rect::from_min_size(Pos2::new(bar_x, y), Vec2::new(bar_width, bar_height));
            painter.rect_filled(
                bg_rect,
                2.0,
                Color32::from_rgba_premultiplied(40, 40, 60, 200),
            );
            painter.rect_stroke(bg_rect, 1.0, Stroke::new(1.0, Color32::GRAY));

            // Fill
            let fill_width = bar_width * value;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(bar_x, y), Vec2::new(fill_width, bar_height)),
                2.0,
                *color,
            );

            // Label
            painter.text(
                Pos2::new(bar_x - 5.0, y + bar_height / 2.0),
                egui::Align2::RIGHT_CENTER,
                name,
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );

            // Value
            painter.text(
                Pos2::new(bar_x + bar_width + 5.0, y + bar_height / 2.0),
                egui::Align2::LEFT_CENTER,
                format!("{:.0}", value * 100.0),
                FontId::proportional(7.0),
                *color,
            );
        }

        // Dominant type
        painter.text(
            Pos2::new(bar_x + bar_width / 2.0, start_y + 4.0 * spacing),
            egui::Align2::CENTER_TOP,
            format!("Dominant: {}", reactivity.dominant_type),
            FontId::proportional(9.0),
            Color32::WHITE,
        );
    }

    /// Render multiple functional groups comparison
    pub fn render_group_comparison(
        &self,
        ui: &mut egui::Ui,
        groups: &[FunctionalGroupRenderData],
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(450.0, 200.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let group_width = (rect.width() - 20.0) / groups.len().max(1) as f32;

        for (i, group) in groups.iter().enumerate() {
            let x = rect.min.x + 10.0 + i as f32 * group_width;

            // Group box
            let box_rect = Rect::from_min_size(
                Pos2::new(x, rect.min.y + 20.0),
                Vec2::new(group_width - 10.0, rect.height() - 40.0),
            );
            painter.rect_filled(
                box_rect,
                4.0,
                Color32::from_rgba_premultiplied(40, 50, 70, 150),
            );
            painter.rect_stroke(
                box_rect,
                1.0,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 150, 200, 100)),
            );

            // Name
            painter.text(
                Pos2::new(x + group_width / 2.0 - 5.0, rect.min.y + 30.0),
                egui::Align2::CENTER_TOP,
                &group.group_name,
                FontId::proportional(9.0),
                Color32::WHITE,
            );

            // Formula
            painter.text(
                Pos2::new(x + group_width / 2.0 - 5.0, rect.min.y + 45.0),
                egui::Align2::CENTER_TOP,
                &group.formula,
                FontId::proportional(7.0),
                Color32::LIGHT_GRAY,
            );

            // Mini reactivity bars
            let bar_x = x + 10.0;
            let bar_y = rect.min.y + 65.0;
            let bar_w = group_width - 25.0;
            let bar_h = 6.0;

            let reactivities = [
                (
                    "E",
                    group.reactivity.electrophilicity,
                    MolecularColors::electrophilic(),
                ),
                (
                    "N",
                    group.reactivity.nucleophilicity,
                    MolecularColors::nucleophilic(),
                ),
                ("A", group.reactivity.acidity, MolecularColors::acidic()),
                ("B", group.reactivity.basicity, MolecularColors::basic()),
            ];

            for (j, (label, value, color)) in reactivities.iter().enumerate() {
                let y = bar_y + j as f32 * 12.0;

                // Background
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(bar_x, y), Vec2::new(bar_w, bar_h)),
                    1.0,
                    Color32::from_rgba_premultiplied(30, 30, 50, 200),
                );

                // Fill
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(bar_x, y), Vec2::new(bar_w * value, bar_h)),
                    1.0,
                    *color,
                );

                // Label
                painter.text(
                    Pos2::new(bar_x - 3.0, y + bar_h / 2.0),
                    egui::Align2::RIGHT_CENTER,
                    label,
                    FontId::proportional(6.0),
                    Color32::LIGHT_GRAY,
                );
            }
        }

        response
    }
}

/// Molecular manifestation view - simultaneous molecular/planetary emergence
#[derive(Debug, Clone)]
pub struct MolecularManifestationView {
    /// Show molecule
    pub show_molecule: bool,
    /// Show planet
    pub show_planet: bool,
    /// Show resonance connection
    pub show_resonance: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
}

impl Default for MolecularManifestationView {
    fn default() -> Self {
        Self {
            show_molecule: true,
            show_planet: true,
            show_resonance: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

impl MolecularManifestationView {
    /// Create a new manifestation view
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render molecular manifestation with planet correspondence
    pub fn render_molecule_planet_pair(
        &self,
        ui: &mut egui::Ui,
        pair: &MolecularPlanetaryPair,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(450.0, 250.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Draw molecule on left
        if self.show_molecule {
            let mol_center = Pos2::new(rect.min.x + 80.0, center.y);
            self.draw_molecule(&painter, mol_center, &pair.molecule);
        }

        // Draw planet on right
        if self.show_planet {
            let planet_center = Pos2::new(rect.max.x - 80.0, center.y);
            self.draw_planet(&painter, planet_center, &pair.planet);
        }

        // Draw resonance connection
        if self.show_resonance {
            self.draw_resonance_connection(&painter, rect);
        }

        // Draw info
        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.max.y - 35.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Resonance: {:.1}% | Alignment: {:.1}%",
                pair.resonance * 100.0,
                pair.archetype_alignment * 100.0
            ),
            FontId::proportional(10.0),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(rect.min.x + 10.0, rect.max.y - 18.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Scale Ratio: {:.0e} | Coupled: {}",
                pair.scale_ratio,
                pair.is_strongly_coupled()
            ),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        response
    }

    /// Draw molecule representation
    fn draw_molecule(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        molecule: &MolecularManifestation,
    ) {
        // Molecule label
        painter.text(
            Pos2::new(center.x, center.y - 60.0),
            egui::Align2::CENTER_TOP,
            "Molecule",
            FontId::proportional(11.0),
            Color32::from_rgb(150, 200, 255),
        );

        // Draw atoms
        let num_atoms = molecule.atom_count().min(6);
        let atom_positions = self.calculate_molecule_positions(center, num_atoms);

        // Draw bonds first
        for bond in &molecule.bonds {
            let bond_color = MolecularColors::bond_type(bond.bond_type);
            let stroke_width = MolecularColors::bond_order(bond.bond_order);

            if let (Some(pos1), Some(pos2)) = (
                atom_positions.first(),
                atom_positions.get(bond.element2.atomic_number() as usize % num_atoms.max(1)),
            ) {
                painter.line_segment([*pos1, *pos2], Stroke::new(stroke_width, bond_color));
            }
        }

        // Draw atoms
        for (i, pos) in atom_positions.iter().enumerate() {
            let color = if i == 0 {
                Color32::from_rgb(100, 150, 200)
            } else {
                Color32::from_rgb(200, 200, 200)
            };

            painter.circle_filled(
                *pos,
                12.0,
                Color32::from_rgba_premultiplied(50, 50, 70, 200),
            );
            painter.circle_stroke(*pos, 12.0, Stroke::new(1.5, color));

            // Atom symbol
            let symbol = if i == 0 { "C" } else { "X" };
            painter.text(
                *pos,
                egui::Align2::CENTER_CENTER,
                symbol,
                FontId::proportional(7.0),
                Color32::WHITE,
            );
        }

        // Info
        painter.text(
            Pos2::new(center.x, center.y + 55.0),
            egui::Align2::CENTER_TOP,
            format!(
                "Atoms: {} | Bonds: {}",
                molecule.atom_count(),
                molecule.bond_count()
            ),
            FontId::proportional(8.0),
            Color32::LIGHT_GRAY,
        );

        painter.text(
            Pos2::new(center.x, center.y + 68.0),
            egui::Align2::CENTER_TOP,
            format!(
                "Energy: {:.0} | Coherence: {:.0}",
                molecule.total_energy,
                molecule.coherence * 100.0
            ),
            FontId::proportional(7.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Calculate molecule atom positions
    fn calculate_molecule_positions(&self, center: Pos2, num_atoms: usize) -> Vec<Pos2> {
        let mut positions = Vec::new();
        let base_radius = 35.0;

        for i in 0..num_atoms {
            let angle = if num_atoms == 1 {
                0.0
            } else {
                (i as f32 / num_atoms as f32) * TAU - PI / 2.0
            };
            let x = center.x + base_radius * angle.cos();
            let y = center.y + base_radius * angle.sin();
            positions.push(Pos2::new(x, y));
        }

        positions
    }

    /// Draw planet representation
    fn draw_planet(&self, painter: &egui::Painter, center: Pos2, planet: &PlanetaryEmergence) {
        let planet_color = MolecularColors::planet_type(planet.planet_type);
        let pulse = 1.0 + (self.animation_time * 1.5).sin() * 0.05;
        let radius = 30.0 * pulse;

        // Planet label
        painter.text(
            Pos2::new(center.x, center.y - 60.0),
            egui::Align2::CENTER_TOP,
            "Planet",
            FontId::proportional(11.0),
            Color32::from_rgb(255, 200, 150),
        );

        // Planet glow
        painter.circle_filled(
            center,
            radius * 1.3,
            Color32::from_rgba_premultiplied(
                planet_color.r(),
                planet_color.g(),
                planet_color.b(),
                50,
            ),
        );

        // Planet body
        painter.circle_filled(center, radius, planet_color);
        painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::WHITE));

        // Surface detail (simplified)
        let detail_angle = self.animation_time * 0.5;
        for i in 0..3 {
            let angle = detail_angle + i as f32 * TAU / 3.0;
            let x = center.x + radius * 0.5 * angle.cos();
            let y = center.y + radius * 0.4 * angle.sin();
            painter.circle_filled(
                Pos2::new(x, y),
                5.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 80),
            );
        }

        // Planet type name
        let type_name = match planet.planet_type {
            PlanetType::Rocky => "Rocky",
            PlanetType::GasGiant => "Gas Giant",
            PlanetType::IceGiant => "Ice Giant",
            PlanetType::Dwarf => "Dwarf",
            PlanetType::Terrestrial => "Terrestrial",
            PlanetType::Ocean => "Ocean",
            PlanetType::Desert => "Desert",
        };

        painter.text(
            Pos2::new(center.x, center.y + 45.0),
            egui::Align2::CENTER_TOP,
            type_name,
            FontId::proportional(9.0),
            planet_color,
        );

        // Info
        painter.text(
            Pos2::new(center.x, center.y + 60.0),
            egui::Align2::CENTER_TOP,
            format!("Mass: {:.1} M⊕", planet.mass_earth_masses),
            FontId::proportional(8.0),
            Color32::LIGHT_GRAY,
        );

        painter.text(
            Pos2::new(center.x, center.y + 73.0),
            egui::Align2::CENTER_TOP,
            format!("Radius: {:.0} km", planet.radius_km),
            FontId::proportional(7.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Draw resonance connection between molecule and planet
    fn draw_resonance_connection(&self, painter: &egui::Painter, rect: Rect) {
        let start = Pos2::new(rect.min.x + 130.0, rect.center().y);
        let end = Pos2::new(rect.max.x - 130.0, rect.center().y);

        // Animated dashed line
        let num_dashes = 10;
        let dash_length = 8.0;
        let gap_length = 8.0;
        let _total_length = end.x - start.x;
        let segment_length = dash_length + gap_length;

        let offset = (self.animation_time * 30.0) % segment_length;

        for i in 0..num_dashes {
            let x_start = start.x + i as f32 * segment_length + offset;
            let x_end = (x_start + dash_length).min(end.x);

            if x_start < end.x {
                painter.line_segment(
                    [Pos2::new(x_start, start.y), Pos2::new(x_end, start.y)],
                    Stroke::new(2.0, Color32::from_rgb(200, 150, 255)),
                );
            }
        }

        // Resonance symbol in center
        let center = Pos2::new(rect.center().x, rect.center().y);
        let pulse = 1.0 + (self.animation_time * 2.0).sin() * 0.2;

        painter.circle_stroke(
            center,
            15.0 * pulse,
            Stroke::new(2.0, Color32::from_rgb(200, 150, 255)),
        );

        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            "∼",
            FontId::proportional(16.0),
            Color32::from_rgb(200, 150, 255),
        );
    }

    /// Render system overview
    pub fn render_system_overview(
        &self,
        ui: &mut egui::Ui,
        system: &MolecularPlanetarySystem,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(400.0, 180.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;

        // Title
        painter.text(
            Pos2::new(rect.center().x, rect.min.y + 10.0),
            egui::Align2::CENTER_TOP,
            "Molecular-Planetary Emergence System",
            FontId::proportional(12.0),
            Color32::WHITE,
        );

        // Statistics
        let stats_y = rect.min.y + 40.0;
        let stat_spacing = 30.0;

        let stats = [
            ("Total Molecules", system.total_molecules() as f32),
            ("Total Planets", system.total_planets() as f32),
            (
                "Strongly Coupled",
                system.strongly_coupled_pairs().len() as f32,
            ),
            ("Avg Resonance", system.average_resonance() as f32 * 100.0),
        ];

        for (i, (label, value)) in stats.iter().enumerate() {
            let x = rect.min.x + 20.0 + (i as f32 % 2.0) * 180.0;
            let y = stats_y + (i as f32 / 2.0).floor() * stat_spacing;

            painter.text(
                Pos2::new(x, y),
                egui::Align2::LEFT_TOP,
                label,
                FontId::proportional(10.0),
                Color32::LIGHT_GRAY,
            );

            let value_str = if *label == "Avg Resonance" {
                format!("{:.1}%", value)
            } else {
                format!("{}", *value as u32)
            };

            painter.text(
                Pos2::new(x + 120.0, y),
                egui::Align2::LEFT_TOP,
                value_str,
                FontId::proportional(10.0),
                Color32::WHITE,
            );
        }

        // Planet type distribution
        let dist = system.planet_type_distribution();
        let dist_y = rect.max.y - 50.0;

        painter.text(
            Pos2::new(rect.min.x + 10.0, dist_y),
            egui::Align2::LEFT_TOP,
            "Planet Type Distribution:",
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        let mut x_offset = 10.0;
        for (planet_type, count) in &dist {
            let type_name = match planet_type {
                PlanetType::Rocky => "Rocky",
                PlanetType::GasGiant => "Gas",
                PlanetType::IceGiant => "Ice",
                PlanetType::Terrestrial => "Terr",
                PlanetType::Ocean => "Ocean",
                PlanetType::Desert => "Desert",
                PlanetType::Dwarf => "Dwarf",
            };

            let color = MolecularColors::planet_type(*planet_type);
            let bar_width = 30.0 + *count as f32 * 10.0;

            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(rect.min.x + x_offset, dist_y + 15.0),
                    Vec2::new(bar_width, 8.0),
                ),
                2.0,
                color,
            );

            painter.text(
                Pos2::new(rect.min.x + x_offset + bar_width / 2.0, dist_y + 25.0),
                egui::Align2::CENTER_TOP,
                format!("{}:{}", type_name, count),
                FontId::proportional(7.0),
                Color32::LIGHT_GRAY,
            );

            x_offset += bar_width + 10.0;
        }

        response
    }
}

/// Create bond render data from archetype bond reference
pub fn create_bond_render_data(bond: &ArchetypeBond) -> BondRenderData {
    BondRenderData::from(bond)
}

/// Create geometry render data from geometry prediction
pub fn create_geometry_render_data(geometry: &GeometryPrediction) -> GeometryRenderData {
    GeometryRenderData::from(geometry)
}

/// Create functional group render data from functional group and resonance
pub fn create_functional_group_render_data(
    group: FunctionalGroup,
    resonance: &FunctionalGroupResonance,
) -> Option<FunctionalGroupRenderData> {
    let _confidence = resonance.get_confidence(group)?;

    let reactivity = ReactivityProfile::for_group(group);

    Some(FunctionalGroupRenderData {
        group_name: group.name().to_string(),
        formula: group.formula().to_string(),
        archetype_pattern: group.archetype_signature().map(|v| v as f32),
        reactivity: ReactivityRenderData::from(&reactivity),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecular_colors_bond_type() {
        let covalent_color = MolecularColors::bond_type(BondType::Covalent);
        assert!(covalent_color != Color32::BLACK);
    }

    #[test]
    fn test_bond_order_stroke() {
        let single_stroke = MolecularColors::bond_order(BondOrder::Single);
        let triple_stroke = MolecularColors::bond_order(BondOrder::Triple);
        assert!(triple_stroke > single_stroke);
    }

    #[test]
    fn test_bond_renderer_default() {
        let renderer = BondRenderer::default();
        assert!(renderer.show_bond_types);
        assert!(renderer.show_bond_order);
    }

    #[test]
    fn test_molecular_geometry_view_default() {
        let view = MolecularGeometryView::default();
        assert!(view.show_interference_minima);
        assert!(view.show_bond_angles);
    }

    #[test]
    fn test_functional_group_visualizer_default() {
        let viz = FunctionalGroupVisualizer::default();
        assert!(viz.show_radar);
        assert!(viz.show_reactivity);
    }

    #[test]
    fn test_molecular_manifestation_view_default() {
        let view = MolecularManifestationView::default();
        assert!(view.show_molecule);
        assert!(view.show_planet);
        assert!(view.show_resonance);
    }

    #[test]
    fn test_planet_colors() {
        let rocky = MolecularColors::planet_type(PlanetType::Rocky);
        let ocean = MolecularColors::planet_type(PlanetType::Ocean);
        assert!(rocky != ocean);
    }

    #[test]
    fn test_reactivity_colors() {
        let electrophilic = MolecularColors::electrophilic();
        let nucleophilic = MolecularColors::nucleophilic();
        assert!(electrophilic != nucleophilic);
    }
}
