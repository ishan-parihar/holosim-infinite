//! Archetype Visualization - 22-Archetype Radar Chart and Interference Display
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.3:
//! "Design 22-archetype radar chart, Implement archetype interference display,
//!  Show archetype → property derivation chain"
//!
//! This module provides:
//! - 22-archetype radar chart visualization
//! - Complex-specific groupings (Mind/Body/Spirit)
//! - Interference pattern overlay
//! - Property derivation visualization

use egui::{Color32, FontId, Pos2, Rect, Shape, Stroke, Vec2};
use std::f32::consts::TAU;

/// Number of archetypes in the system
pub const NUM_ARCHETYPES: usize = 22;

/// Archetype indices by complex
pub const MIND_ARCHETYPES: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
pub const BODY_ARCHETYPES: [usize; 7] = [7, 8, 9, 10, 11, 12, 13];
pub const SPIRIT_ARCHETYPES: [usize; 7] = [14, 15, 16, 17, 18, 19, 20];
pub const CHOICE_ARCHETYPE: usize = 21;

/// Archetype names with Tarot correlations
pub const ARCHETYPE_NAMES: [&str; NUM_ARCHETYPES] = [
    // Mind Complex (A1-A7)
    "A1: Matrix of Mind",
    "A2: Potentiator of Mind",
    "A3: Catalyst of Mind",
    "A4: Experience of Mind",
    "A5: Significator of Mind",
    "A6: Transformation of Mind",
    "A7: Great Way of Mind",
    // Body Complex (A8-A14)
    "A8: Matrix of Body",
    "A9: Potentiator of Body",
    "A10: Catalyst of Body",
    "A11: Experience of Body",
    "A12: Significator of Body",
    "A13: Transformation of Body",
    "A14: Great Way of Body",
    // Spirit Complex (A15-A21)
    "A15: Matrix of Spirit",
    "A16: Potentiator of Spirit",
    "A17: Catalyst of Spirit",
    "A18: Experience of Spirit",
    "A19: Significator of Spirit",
    "A20: Transformation of Spirit",
    "A21: Great Way of Spirit",
    // The Choice (A22)
    "A22: The Choice",
];

/// Brief archetype names for display
pub const ARCHETYPE_SHORT_NAMES: [&str; NUM_ARCHETYPES] = [
    "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "A10", "A11", "A12", "A13", "A14", "A15",
    "A16", "A17", "A18", "A19", "A20", "A21", "A22",
];

/// Complex names
pub const COMPLEX_NAMES: [&str; 4] = [
    "Mind Complex (σA)",
    "Body Complex (σB)",
    "Spirit Complex (σC)",
    "The Choice (A22)",
];

/// Get archetype glow color (from existing entity_viz.rs)
pub fn get_archetype_color(archetype_idx: usize) -> Color32 {
    match archetype_idx {
        // Mind Complex - Purple/Blue tones
        0 => Color32::from_rgb(128, 0, 128), // A1: Matrix - Purple
        1 => Color32::from_rgb(180, 0, 60),  // A2: Potentiator - Rose
        2 => Color32::from_rgb(50, 180, 50), // A3: Catalyst - Green
        3 => Color32::from_rgb(0, 128, 200), // A4: Experience - Blue
        4 => Color32::from_rgb(200, 128, 0), // A5: Significator - Orange
        5 => Color32::from_rgb(0, 180, 180), // A6: Transformation - Cyan
        6 => Color32::from_rgb(180, 0, 180), // A7: Great Way - Magenta
        // Body Complex - Orange/Red tones
        7 => Color32::from_rgb(200, 100, 50), // A8: Matrix - Terracotta
        8 => Color32::from_rgb(220, 80, 40),  // A9: Potentiator - Rust
        9 => Color32::from_rgb(180, 140, 20), // A10: Catalyst - Gold
        10 => Color32::from_rgb(160, 120, 60), // A11: Experience - Bronze
        11 => Color32::from_rgb(200, 100, 100), // A12: Significator - Coral
        12 => Color32::from_rgb(180, 80, 100), // A13: Transformation - Rose
        13 => Color32::from_rgb(220, 60, 60), // A14: Great Way - Red
        // Spirit Complex - White/Silver tones
        14 => Color32::from_rgb(200, 200, 220), // A15: Matrix - Silver
        15 => Color32::from_rgb(180, 180, 200), // A16: Potentiator - Lavender
        16 => Color32::from_rgb(220, 220, 240), // A17: Catalyst - White
        17 => Color32::from_rgb(200, 220, 220), // A18: Experience - Pearl
        18 => Color32::from_rgb(220, 200, 240), // A19: Significator - Lilac
        19 => Color32::from_rgb(240, 240, 255), // A20: Transformation - Snow
        20 => Color32::from_rgb(255, 255, 255), // A21: Great Way - Pure White
        // The Choice
        21 => Color32::from_rgb(255, 215, 0), // A22: Choice - Gold
        _ => Color32::from_rgb(128, 128, 128), // Default - Gray
    }
}

/// Get complex color
pub fn get_complex_color(complex_idx: usize) -> Color32 {
    match complex_idx {
        0 => Color32::from_rgb(138, 43, 226),  // Mind - Blue Violet
        1 => Color32::from_rgb(255, 140, 0),   // Body - Dark Orange
        2 => Color32::from_rgb(220, 220, 255), // Spirit - Silver White
        3 => Color32::from_rgb(255, 215, 0),   // Choice - Gold
        _ => Color32::GRAY,
    }
}

/// Archetype profile data for visualization
#[derive(Debug, Clone)]
pub struct ArchetypeProfileData {
    /// Activation values for all 22 archetypes (0.0 to 1.0)
    pub activations: [f32; NUM_ARCHETYPES],
    /// Computed coherence (0.0 = chaotic, 1.0 = coherent)
    pub coherence: f32,
    /// Computed entropy (0.0 = ordered, higher = chaotic)
    pub entropy: f32,
    /// Dominant archetype indices (sorted by activation, highest first)
    pub dominant_indices: [usize; 5],
}

impl ArchetypeProfileData {
    /// Create from activation array
    pub fn new(activations: [f32; NUM_ARCHETYPES]) -> Self {
        let coherence = Self::compute_coherence(&activations);
        let entropy = Self::compute_entropy(&activations);
        let dominant_indices = Self::find_dominant(&activations);

        Self {
            activations,
            coherence,
            entropy,
            dominant_indices,
        }
    }

    /// Create default profile
    pub fn default_profile() -> Self {
        let mut activations = [0.5f32; NUM_ARCHETYPES];
        activations[CHOICE_ARCHETYPE] = 0.8; // Higher choice activation
        Self::new(activations)
    }

    /// Compute coherence from variance
    fn compute_coherence(activations: &[f32; NUM_ARCHETYPES]) -> f32 {
        let mean: f32 = activations.iter().sum::<f32>() / NUM_ARCHETYPES as f32;
        let variance: f32 =
            activations.iter().map(|a| (a - mean).powi(2)).sum::<f32>() / NUM_ARCHETYPES as f32;
        1.0 / (1.0 + variance.sqrt() * 2.0)
    }

    /// Compute entropy using Shannon formula
    fn compute_entropy(activations: &[f32; NUM_ARCHETYPES]) -> f32 {
        let total: f32 = activations.iter().sum();
        if total < 1e-10 {
            return 0.0;
        }

        let mut entropy = 0.0;
        for &a in activations.iter() {
            let p = a / total;
            if p > 1e-10 {
                entropy -= p * p.ln();
            }
        }
        entropy / (NUM_ARCHETYPES as f32).ln() // Normalized
    }

    /// Find 5 dominant archetypes
    fn find_dominant(activations: &[f32; NUM_ARCHETYPES]) -> [usize; 5] {
        let mut indexed: Vec<(usize, f32)> = activations
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut result = [0usize; 5];
        for (i, (idx, _)) in indexed.iter().take(5).enumerate() {
            result[i] = *idx;
        }
        result
    }

    /// Get Mind complex activations
    pub fn mind_complex(&self) -> [f32; 7] {
        let mut result = [0.0f32; 7];
        for (i, &idx) in MIND_ARCHETYPES.iter().enumerate() {
            result[i] = self.activations[idx];
        }
        result
    }

    /// Get Body complex activations
    pub fn body_complex(&self) -> [f32; 7] {
        let mut result = [0.0f32; 7];
        for (i, &idx) in BODY_ARCHETYPES.iter().enumerate() {
            result[i] = self.activations[idx];
        }
        result
    }

    /// Get Spirit complex activations
    pub fn spirit_complex(&self) -> [f32; 7] {
        let mut result = [0.0f32; 7];
        for (i, &idx) in SPIRIT_ARCHETYPES.iter().enumerate() {
            result[i] = self.activations[idx];
        }
        result
    }

    /// Get complex averages
    pub fn complex_averages(&self) -> [f32; 4] {
        [
            self.mind_complex().iter().sum::<f32>() / 7.0,
            self.body_complex().iter().sum::<f32>() / 7.0,
            self.spirit_complex().iter().sum::<f32>() / 7.0,
            self.activations[CHOICE_ARCHETYPE],
        ]
    }

    /// Get dominant archetype name
    pub fn dominant_name(&self) -> &'static str {
        ARCHETYPE_NAMES[self.dominant_indices[0]]
    }
}

impl Default for ArchetypeProfileData {
    fn default() -> Self {
        Self::default_profile()
    }
}

/// Archetype radar chart configuration
#[derive(Debug, Clone)]
pub struct ArchetypeRadarConfig {
    /// Show archetype labels
    pub show_labels: bool,
    /// Show complex outline rings
    pub show_complex_rings: bool,
    /// Show interference pattern overlay
    pub show_interference: bool,
    /// Show A22 at center
    pub center_choice: bool,
    /// Animate the chart
    pub animate: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor for display
    pub scale: f32,
}

impl Default for ArchetypeRadarConfig {
    fn default() -> Self {
        Self {
            show_labels: true,
            show_complex_rings: true,
            show_interference: true,
            center_choice: true,
            animate: true,
            animation_time: 0.0,
            scale: 1.0,
        }
    }
}

/// Archetype radar chart renderer
pub struct ArchetypeRadarChart {
    /// Configuration
    config: ArchetypeRadarConfig,
    /// Cached vertex positions for 22 archetypes
    vertex_positions: [Pos2; NUM_ARCHETYPES],
    /// Cached complex vertex positions
    complex_positions: [[Pos2; 7]; 3],
}

impl ArchetypeRadarChart {
    /// Create a new radar chart renderer
    pub fn new() -> Self {
        Self {
            config: ArchetypeRadarConfig::default(),
            vertex_positions: [Pos2::ZERO; NUM_ARCHETYPES],
            complex_positions: [[Pos2::ZERO; 7]; 3],
        }
    }

    /// Set configuration
    pub fn with_config(mut self, config: ArchetypeRadarConfig) -> Self {
        self.config = config;
        self
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        if self.config.animate {
            self.config.animation_time += delta_time;
        }
    }

    /// Render the radar chart into a rect
    pub fn render(&self, ui: &mut egui::Ui, profile: &ArchetypeProfileData) -> egui::Response {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(300.0, 300.0) * self.config.scale,
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let center = rect.center();
        let base_radius = rect.width().min(rect.height()) * 0.4;

        // Animation effect
        let pulse = if self.config.animate {
            1.0 + (self.config.animation_time * 2.0).sin() * 0.02
        } else {
            1.0
        };
        let radius = base_radius * pulse;

        // Draw background rings
        self.draw_rings(&painter, center, radius);

        // Draw complex outlines
        if self.config.show_complex_rings {
            self.draw_complex_outlines(&painter, center, radius * 0.7, profile);
        }

        // Draw archetype vertices and connections
        self.draw_archetype_shape(&painter, center, radius, profile);

        // Draw labels
        if self.config.show_labels {
            self.draw_labels(&painter, center, radius);
        }

        // Draw A22 at center
        if self.config.center_choice {
            self.draw_choice_center(&painter, center, profile);
        }

        response
    }

    /// Draw background reference rings
    fn draw_rings(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 1..=4 {
            let ring_radius = radius * (i as f32 / 4.0);
            let alpha = 30 + (4 - i) * 10;
            painter.circle_stroke(
                center,
                ring_radius,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(128, 128, 128, alpha)),
            );
        }
    }

    /// Draw complex-specific outlines (Mind/Body/Spirit as heptagons)
    fn draw_complex_outlines(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        profile: &ArchetypeProfileData,
    ) {
        let complexes = [
            (
                MIND_ARCHETYPES,
                get_complex_color(0),
                profile.mind_complex(),
            ),
            (
                BODY_ARCHETYPES,
                get_complex_color(1),
                profile.body_complex(),
            ),
            (
                SPIRIT_ARCHETYPES,
                get_complex_color(2),
                profile.spirit_complex(),
            ),
        ];

        for (indices, color, activations) in complexes {
            // Draw heptagon outline with activations
            let points: Vec<Pos2> = indices
                .iter()
                .enumerate()
                .map(|(i, &idx)| {
                    let angle = (idx as f32 / 21.0) * TAU - TAU / 4.0; // Start from top
                    let activation = activations[i];
                    let r = radius * (0.3 + activation * 0.7);
                    Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
                })
                .collect();

            // Fill
            painter.add(Shape::convex_polygon(
                points.clone(),
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 30),
                Stroke::new(
                    2.0,
                    Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 128),
                ),
            ));
        }
    }

    /// Draw the main archetype shape
    fn draw_archetype_shape(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        profile: &ArchetypeProfileData,
    ) {
        // Calculate vertex positions
        let vertices: Vec<Pos2> = (0..21) // A1-A21 around the circle
            .map(|i| {
                let angle = (i as f32 / 21.0) * TAU - TAU / 4.0;
                let activation = profile.activations[i];
                let r = radius * (0.2 + activation * 0.8);
                Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
            })
            .collect();

        // Draw filled shape
        let fill_color = Color32::from_rgba_premultiplied(
            (profile.coherence * 200.0) as u8,
            (100.0 + profile.coherence * 100.0) as u8,
            (200.0 - profile.entropy * 100.0) as u8,
            80,
        );

        painter.add(Shape::convex_polygon(
            vertices.clone(),
            fill_color,
            Stroke::new(2.0, Color32::from_rgba_premultiplied(255, 255, 255, 150)),
        ));

        // Draw vertex points
        for (i, &pos) in vertices.iter().enumerate() {
            let activation = profile.activations[i];
            let point_radius = 3.0 + activation * 4.0;
            let color = get_archetype_color(i);

            // Glow for dominant
            let is_dominant = profile.dominant_indices.contains(&i);
            if is_dominant {
                painter.circle_filled(
                    pos,
                    point_radius + 3.0,
                    Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 100),
                );
            }

            painter.circle_filled(pos, point_radius, color);
        }
    }

    /// Draw archetype labels
    fn draw_labels(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 0..21 {
            let angle = (i as f32 / 21.0) * TAU - TAU / 4.0;
            let label_radius = radius * 1.1;
            let pos = Pos2::new(
                center.x + label_radius * angle.cos(),
                center.y + label_radius * angle.sin(),
            );

            let align = if angle.cos() > 0.1 {
                egui::Align2::LEFT_CENTER
            } else if angle.cos() < -0.1 {
                egui::Align2::RIGHT_CENTER
            } else if angle.sin() > 0.0 {
                egui::Align2::CENTER_BOTTOM
            } else {
                egui::Align2::CENTER_TOP
            };

            painter.text(
                pos,
                align,
                ARCHETYPE_SHORT_NAMES[i],
                FontId::proportional(10.0),
                Color32::LIGHT_GRAY,
            );
        }
    }

    /// Draw A22 (The Choice) at center
    fn draw_choice_center(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        profile: &ArchetypeProfileData,
    ) {
        let choice_activation = profile.activations[CHOICE_ARCHETYPE];
        let pulse = if self.config.animate {
            1.0 + (self.config.animation_time * 3.0).sin() * 0.1
        } else {
            1.0
        };

        let center_radius = 15.0 + choice_activation * 10.0;

        // Outer glow
        painter.circle_filled(
            center,
            center_radius * pulse * 1.5,
            Color32::from_rgba_premultiplied(255, 215, 0, 50),
        );

        // Inner circle
        painter.circle_filled(
            center,
            center_radius * pulse,
            get_archetype_color(CHOICE_ARCHETYPE),
        );

        // Center dot
        painter.circle_filled(center, 3.0, Color32::WHITE);

        // Label
        painter.text(
            center + Vec2::new(0.0, center_radius + 15.0),
            egui::Align2::CENTER_TOP,
            "A22",
            FontId::proportional(12.0),
            Color32::GOLD,
        );
    }
}

impl Default for ArchetypeRadarChart {
    fn default() -> Self {
        Self::new()
    }
}

/// Archetype interference display
#[derive(Debug, Clone)]
pub struct InterferenceDisplay {
    /// Interference pattern components
    pub components: Vec<f32>,
    /// Coherence value
    pub coherence: f32,
    /// Entropy value
    pub entropy: f32,
    /// Resonance value
    pub resonance: f32,
}

impl InterferenceDisplay {
    /// Create from archetype profile
    pub fn from_profile(profile: &ArchetypeProfileData) -> Self {
        // Compute interference between adjacent archetypes
        let mut components = Vec::with_capacity(NUM_ARCHETYPES);

        for i in 0..NUM_ARCHETYPES {
            let next_i = (i + 1) % NUM_ARCHETYPES;
            // Interference = product of adjacent activations
            let interference = profile.activations[i] * profile.activations[next_i];
            components.push(interference);
        }

        // Resonance = sum of interference
        let resonance: f32 = components.iter().sum();

        Self {
            components,
            coherence: profile.coherence,
            entropy: profile.entropy,
            resonance,
        }
    }

    /// Render as a bar chart
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label(format!("Coherence: {:.2}", self.coherence));
        ui.label(format!("Entropy: {:.2}", self.entropy));
        ui.label(format!("Resonance: {:.2}", self.resonance));

        // Draw interference bars
        let rect = ui.allocate_space(Vec2::new(ui.available_width(), 50.0)).1;
        let painter = ui.painter();

        let bar_width = rect.width() / self.components.len() as f32;
        let max_val = self
            .components
            .iter()
            .cloned()
            .fold(0.0_f32, f32::max)
            .max(0.01);

        for (i, &val) in self.components.iter().enumerate() {
            let bar_height = (val / max_val) * rect.height();
            let x = rect.min.x + i as f32 * bar_width;
            let bar_rect = Rect::from_min_size(
                Pos2::new(x, rect.max.y - bar_height),
                Vec2::new(bar_width - 1.0, bar_height),
            );

            let color = get_archetype_color(i % NUM_ARCHETYPES);
            painter.rect_filled(bar_rect, 0.0, color);
        }
    }
}

/// Property derivation chain display
#[derive(Debug, Clone)]
pub struct PropertyDerivation {
    /// Derived quantum numbers (n, l, m, s)
    pub quantum_numbers: (i32, i32, i32, f32),
    /// Derived mass
    pub mass: f32,
    /// Derived charge
    pub charge: f32,
    /// Derived spin
    pub spin: f32,
    /// Stability classification
    pub stability: &'static str,
}

impl PropertyDerivation {
    /// Derive properties from archetype profile
    pub fn from_profile(profile: &ArchetypeProfileData) -> Self {
        let activations = &profile.activations;

        // Quantum numbers
        let n = (activations.iter().sum::<f32>() * 5.0 + 1.0) as i32;
        let l = ((profile.mind_complex().iter().sum::<f32>() / 7.0) * 4.0) as i32;
        let m = ((profile.body_complex().iter().sum::<f32>() / 7.0) * 4.0 - 2.0) as i32;
        let s = ((profile.spirit_complex().iter().sum::<f32>() / 7.0) - 0.5) * 2.0;

        // Mass from coherence
        let mass = (0.5 + profile.coherence * 1.5) * (1.0 + profile.entropy);

        // Charge from Mind-Body balance
        let charge = (profile.mind_complex().iter().sum::<f32>()
            - profile.body_complex().iter().sum::<f32>())
            / 7.0;

        // Spin from Spirit
        let spin = s;

        // Stability
        let stability = if profile.coherence > 0.8 {
            "Stable"
        } else if profile.coherence > 0.5 {
            "Long-lived"
        } else if profile.coherence > 0.3 {
            "Short-lived"
        } else {
            "Unstable"
        };

        Self {
            quantum_numbers: (n.max(1), l.min(n - 1).max(0), m, s),
            mass,
            charge,
            spin,
            stability,
        }
    }

    /// Render property derivation display
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.heading("Property Derivation");
        ui.separator();

        ui.label("Quantum Numbers:");
        let (n, l, m, s) = self.quantum_numbers;
        ui.label(format!("  n = {} (Principal)", n));
        ui.label(format!("  l = {} (Angular)", l));
        ui.label(format!("  m = {} (Magnetic)", m));
        ui.label(format!("  s = {:.2} (Spin)", s));

        ui.separator();
        ui.label("Derived Properties:");
        ui.label(format!("  Mass: {:.3}", self.mass));
        ui.label(format!("  Charge: {:.3}", self.charge));
        ui.label(format!("  Spin: {:.3}", self.spin));
        ui.label(format!("  Stability: {}", self.stability));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_profile_creation() {
        let profile = ArchetypeProfileData::default_profile();
        assert_eq!(profile.activations.len(), NUM_ARCHETYPES);
        assert!(profile.coherence > 0.0);
    }

    #[test]
    fn test_archetype_colors() {
        let color = get_archetype_color(0);
        assert!(color.r() > 0 || color.g() > 0 || color.b() > 0);

        let choice_color = get_archetype_color(CHOICE_ARCHETYPE);
        assert!(choice_color.r() > 200); // Should be gold
    }

    #[test]
    fn test_dominant_archetypes() {
        let mut activations = [0.3f32; NUM_ARCHETYPES];
        activations[5] = 0.9; // Make A6 dominant
        activations[10] = 0.8;

        let profile = ArchetypeProfileData::new(activations);
        assert_eq!(profile.dominant_indices[0], 5);
        assert_eq!(profile.dominant_indices[1], 10);
    }

    #[test]
    fn test_complex_averages() {
        let mut activations = [0.5f32; NUM_ARCHETYPES];
        // Higher mind activation
        for i in 0..7 {
            activations[i] = 0.8;
        }

        let profile = ArchetypeProfileData::new(activations);
        let averages = profile.complex_averages();

        assert!(averages[0] > averages[1]); // Mind > Body
    }

    #[test]
    fn test_property_derivation() {
        let profile = ArchetypeProfileData::default_profile();
        let derivation = PropertyDerivation::from_profile(&profile);

        assert!(derivation.mass > 0.0);
        assert!(derivation.quantum_numbers.0 >= 1);
    }

    #[test]
    fn test_interference_display() {
        let profile = ArchetypeProfileData::default_profile();
        let interference = InterferenceDisplay::from_profile(&profile);

        assert_eq!(interference.components.len(), NUM_ARCHETYPES);
        assert!(interference.resonance > 0.0);
    }
}
