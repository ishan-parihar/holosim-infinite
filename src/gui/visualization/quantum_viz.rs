//! Quantum Level Visualization - Phase C.1
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.1:
//! "Design 22-archetype quantum probability cloud, Implement wavefunction collapse
//!  as Free Will visualization, Show quantum number derivation from archetype activations"
//!
//! This module provides visualizations for:
//! - Wavefunction probability clouds from QuantumNode data
//! - Quantum number derivation from archetype activations (n from magnitude, l from Mind, m from Body, s from Spirit)
//! - Collapse visualization showing Free Will moment (Archetype 22 choice)
//! - Entanglement as phase correlation networks
//! - Observer effect as cache invalidation
//!
//! Key design principles:
//! - Quantum numbers derive from archetype activations (NOT arbitrary labels)
//! - Wavefunction collapse is Free Will (Archetype 22), NOT random
//! - Entanglement is phase correlation, NOT mysterious action-at-a-distance
//! - Observer effect IS cache invalidation in the holographic field
//!
//! Uses f32 for all visualization types (converts from Float/f64 where needed)

use egui::{Color32, FontId, Pos2, Rect, Stroke, Vec2};
use std::f32::consts::TAU;

use crate::holographic_foundation::quantum_consciousness::archetype_collapse::{
    CollapseResult, CollapseType,
};
use crate::holographic_foundation::quantum_consciousness::entanglement_field::{
    EntanglementCorrelation, EntanglementField, EntanglementFieldStatistics,
};
use crate::holographic_foundation::quantum_consciousness::quantum_numbers::{
    ArchetypeToQuantumMapping, QuantumNumberSet,
};
use crate::holographic_foundation::quantum_consciousness::wavefunction::{
    QuantumNode, QuantumNodeId, QuantumWavefunction, WavefunctionState,
};

/// Color palette for quantum states (phase-colored probability clouds)
pub struct QuantumColors;

impl QuantumColors {
    /// Get color based on quantum phase (0 to 2π)
    pub fn from_phase(phase: f64) -> Color32 {
        let normalized = (phase % (2.0 * std::f64::consts::PI)) / (2.0 * std::f64::consts::PI);

        // Phase cycles through: Red → Yellow → Green → Cyan → Blue → Magenta → Red
        let (r, g, b) = if normalized < 1.0 / 6.0 {
            // Red to Yellow
            let t = (normalized * 6.0) as f32;
            (255, (255.0 * t) as u8, 0)
        } else if normalized < 2.0 / 6.0 {
            // Yellow to Green
            let t = ((normalized - 1.0 / 6.0) * 6.0) as f32;
            ((255.0 * (1.0 - t)) as u8, 255, 0)
        } else if normalized < 3.0 / 6.0 {
            // Green to Cyan
            let t = ((normalized - 2.0 / 6.0) * 6.0) as f32;
            (0, 255, (255.0 * t) as u8)
        } else if normalized < 4.0 / 6.0 {
            // Cyan to Blue
            let t = ((normalized - 3.0 / 6.0) * 6.0) as f32;
            (0, (255.0 * (1.0 - t)) as u8, 255)
        } else if normalized < 5.0 / 6.0 {
            // Blue to Magenta
            let t = ((normalized - 4.0 / 6.0) * 6.0) as f32;
            ((255.0 * t) as u8, 0, 255)
        } else {
            // Magenta to Red
            let t = ((normalized - 5.0 / 6.0) * 6.0) as f32;
            (255, 0, (255.0 * (1.0 - t)) as u8)
        };

        Color32::from_rgb(r, g, b)
    }

    /// Get color based on probability density (brighter = more probable)
    pub fn from_probability(probability: f64) -> Color32 {
        let intensity = (probability.sqrt() * 255.0).min(255.0) as u8;
        Color32::from_rgba_premultiplied(intensity, intensity, intensity, 200)
    }

    /// Get entanglement correlation color
    pub fn from_entanglement_strength(strength: f64) -> Color32 {
        // High strength = Gold, Low strength = Gray
        let r = (100.0 + strength * 155.0) as u8;
        let g = (100.0 + strength * 115.0) as u8;
        let b = (100.0 - strength * 100.0) as u8;
        Color32::from_rgb(r, g, b)
    }

    /// Spin up color (cyan)
    pub fn spin_up() -> Color32 {
        Color32::from_rgb(0, 200, 255)
    }

    /// Spin down color (orange)
    pub fn spin_down() -> Color32 {
        Color32::from_rgb(255, 150, 0)
    }

    /// Collapse event color (gold)
    pub fn collapse() -> Color32 {
        Color32::from_rgb(255, 215, 0)
    }

    /// Cache hit color (green)
    pub fn cache_hit() -> Color32 {
        Color32::from_rgb(50, 200, 50)
    }

    /// Cache miss color (red)
    pub fn cache_miss() -> Color32 {
        Color32::from_rgb(200, 50, 50)
    }
}

/// Wavefunction renderer - visualize probability clouds from QuantumNode data
#[derive(Debug, Clone)]
pub struct WavefunctionRenderer {
    /// Show probability density
    pub show_probability: bool,
    /// Show phase coloring
    pub show_phase: bool,
    /// Show quantum numbers on nodes
    pub show_quantum_numbers: bool,
    /// Animation time
    pub animation_time: f32,
    /// Scale factor
    pub scale: f32,
    /// Selected node (for detailed view)
    pub selected_node: Option<QuantumNodeId>,
}

impl Default for WavefunctionRenderer {
    fn default() -> Self {
        Self {
            show_probability: true,
            show_phase: true,
            show_quantum_numbers: true,
            animation_time: 0.0,
            scale: 1.0,
            selected_node: None,
        }
    }
}

impl WavefunctionRenderer {
    /// Create a new renderer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Render wavefunction as probability cloud
    pub fn render(&self, ui: &mut egui::Ui, wavefunction: &QuantumWavefunction) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0) * self.scale, egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let base_radius = rect.width().min(rect.height()) * 0.4;

        // Animation pulse
        let pulse = 1.0 + (self.animation_time * 2.0).sin() * 0.03;
        let radius = base_radius * pulse;

        // Draw reference circles
        self.draw_reference_circles(&painter, center, radius);

        // Collect nodes sorted by probability (draw highest first)
        let mut nodes: Vec<&QuantumNode> = wavefunction.nodes().collect();
        nodes.sort_by(|a, b| {
            b.probability_density()
                .partial_cmp(&a.probability_density())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Draw probability clouds
        for node in &nodes {
            self.draw_node_cloud(&painter, center, radius, node);
        }

        // Draw quantum state info
        self.draw_state_info(&painter, rect, wavefunction);

        response
    }

    /// Draw reference circles
    fn draw_reference_circles(&self, painter: &egui::Painter, center: Pos2, radius: f32) {
        for i in 1..=4 {
            let ring_radius = radius * (i as f32 / 4.0);
            let alpha = 25 + (4 - i) * 10;
            painter.circle_stroke(
                center,
                ring_radius,
                Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, alpha)),
            );
        }

        // Center cross
        let cross_size = 5.0;
        painter.line_segment(
            [
                Pos2::new(center.x - cross_size, center.y),
                Pos2::new(center.x + cross_size, center.y),
            ],
            Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 100)),
        );
        painter.line_segment(
            [
                Pos2::new(center.x, center.y - cross_size),
                Pos2::new(center.x, center.y + cross_size),
            ],
            Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 100)),
        );
    }

    /// Draw a single node's probability cloud
    fn draw_node_cloud(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        node: &QuantumNode,
    ) {
        // Position based on quantum numbers (n determines distance from center)
        let n = node.quantum_numbers.n as f32;
        let l = node.quantum_numbers.l as f32;
        let m = node.quantum_numbers.m as f32;

        // Position: n determines radial distance, m determines angle
        let radial_factor = n.max(1.0) / 7.0; // Normalize to max n=7
        let r = radius * (0.2 + radial_factor * 0.8);

        let angle = if l > 0.0 { (m / l) * TAU / 4.0 } else { 0.0 };

        let pos = Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin());

        // Probability determines size
        let probability = node.probability_density() as f32;
        let base_size = 5.0 + probability * 20.0;
        let size = base_size * (1.0 + (self.animation_time + probability * 10.0).sin() * 0.1);

        // Color based on phase or probability
        let color = if self.show_phase {
            QuantumColors::from_phase(node.phase())
        } else {
            QuantumColors::from_probability(node.probability_density())
        };

        // Draw glow
        painter.circle_filled(
            pos,
            size * 1.5,
            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 50),
        );

        // Draw main cloud
        painter.circle_filled(pos, size, color);

        // Draw core
        painter.circle_filled(pos, size * 0.3, Color32::WHITE);

        // Draw quantum numbers if enabled
        if self.show_quantum_numbers {
            let qn = &node.quantum_numbers;
            let label = format!("{}{}", qn.n, qn.orbital_type());
            painter.text(
                pos + Vec2::new(size + 2.0, -size / 2.0),
                egui::Align2::LEFT_CENTER,
                label,
                FontId::proportional(8.0),
                Color32::LIGHT_GRAY,
            );
        }
    }

    /// Draw quantum state information
    fn draw_state_info(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        wavefunction: &QuantumWavefunction,
    ) {
        let stats = wavefunction.statistics();

        let info_y = rect.max.y - 60.0;

        // State type
        let state_text: String = match wavefunction.state() {
            WavefunctionState::Pure => "Pure State".to_string(),
            WavefunctionState::Superposition { num_components } => {
                format!("Superposition ({} components)", num_components)
            }
            WavefunctionState::Mixed { purity } => format!("Mixed (purity: {:.2})", purity),
            WavefunctionState::Collapsed { basis_state } => {
                format!("Collapsed to {}", basis_state.to_spectroscopic_notation())
            }
        };

        painter.text(
            Pos2::new(rect.min.x + 5.0, info_y),
            egui::Align2::LEFT_TOP,
            state_text,
            FontId::proportional(10.0),
            Color32::WHITE,
        );

        // Coherence
        painter.text(
            Pos2::new(rect.min.x + 5.0, info_y + 12.0),
            egui::Align2::LEFT_TOP,
            format!("Coherence: {:.2}", stats.coherence),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Node count
        painter.text(
            Pos2::new(rect.min.x + 5.0, info_y + 24.0),
            egui::Align2::LEFT_TOP,
            format!("Nodes: {}", stats.node_count),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Get selected node info
    pub fn get_node_info(&self, wavefunction: &QuantumWavefunction) -> Option<String> {
        let selected = self.selected_node.as_ref()?;
        let node = wavefunction.get_node(selected)?;

        Some(format!(
            "n={}, l={}, m={}, s={}\nPhase: {:.3}\nProbability: {:.4}",
            node.quantum_numbers.n,
            node.quantum_numbers.l,
            node.quantum_numbers.m,
            if node.quantum_numbers.s.is_up() {
                "↑"
            } else {
                "↓"
            },
            node.phase(),
            node.probability_density()
        ))
    }
}

/// Quantum number derivation view - show n,l,m,s derivation from archetype activations
#[derive(Debug, Clone)]
pub struct QuantumNumberDerivationView {
    /// Show derivation chain
    pub show_chain: bool,
    /// Show archetype source
    pub show_archetype_source: bool,
    /// Show confidence indicator
    pub show_confidence: bool,
}

impl Default for QuantumNumberDerivationView {
    fn default() -> Self {
        Self {
            show_chain: true,
            show_archetype_source: true,
            show_confidence: true,
        }
    }
}

impl QuantumNumberDerivationView {
    /// Create a new view
    pub fn new() -> Self {
        Self::default()
    }

    /// Derive quantum numbers from archetype activations
    pub fn derive_from_archetypes(archetype_vector: &[f64; 22]) -> QuantumNumberDerivationDisplay {
        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(archetype_vector);

        // Calculate source values
        let total: f64 = archetype_vector.iter().sum();
        let mind: f64 = archetype_vector[0..7].iter().sum::<f64>() / 7.0;
        let body: f64 = archetype_vector[7..14].iter().sum::<f64>() / 7.0;
        let spirit: f64 = archetype_vector[14..21].iter().sum::<f64>() / 7.0;
        let choice = archetype_vector[21];

        // Calculate confidence (based on coherence of archetype vector)
        let mean = total / 22.0;
        let variance: f64 = archetype_vector
            .iter()
            .map(|a| (a - mean).powi(2))
            .sum::<f64>()
            / 22.0;
        let confidence = (1.0 - variance.sqrt()).clamp(0.0, 1.0);

        QuantumNumberDerivationDisplay {
            quantum_numbers: qn,
            n_source: total / 22.0,
            l_source: mind,
            m_source: body,
            s_source: spirit,
            choice_activation: choice,
            confidence,
        }
    }

    /// Render the derivation view
    pub fn render(&self, ui: &mut egui::Ui, derivation: &QuantumNumberDerivationDisplay) {
        ui.heading("Quantum Number Derivation");
        ui.separator();

        let qn = &derivation.quantum_numbers;

        // Main quantum numbers display
        ui.label("Principal Quantum Number (n):");
        ui.label(format!(
            "  n = {} ← From total archetype activation magnitude",
            qn.n
        ));

        ui.label("\nAngular Momentum Quantum Number (l):");
        ui.label(format!("  l = {} ← From Mind Complex (A1-A7)", qn.l));

        ui.label("\nMagnetic Quantum Number (m):");
        ui.label(format!("  m = {} ← From Body Complex (A8-A14)", qn.m));

        ui.label("\nSpin Quantum Number (s):");
        let spin_str = if qn.s.is_up() {
            "↑ (+1/2)"
        } else {
            "↓ (-1/2)"
        };
        ui.label(format!(
            "  s = {} ← From Spirit Complex (A15-A21)",
            spin_str
        ));

        ui.separator();

        // Spectroscopic notation
        ui.label("Spectroscopic Notation:");
        ui.label(format!("  {}", qn.to_spectroscopic_notation()));

        // Shell name
        ui.label("Shell:");
        ui.label(format!("  {}", qn.shell_name()));

        ui.separator();

        // Source values
        if self.show_archetype_source {
            ui.label("Archetype Source Values:");
            ui.label(format!(
                "  n source: {:.3} (total activation)",
                derivation.n_source
            ));
            ui.label(format!(
                "  l source: {:.3} (Mind average)",
                derivation.l_source
            ));
            ui.label(format!(
                "  m source: {:.3} (Body average)",
                derivation.m_source
            ));
            ui.label(format!(
                "  s source: {:.3} (Spirit average)",
                derivation.s_source
            ));
            ui.label(format!(
                "  A22 (Choice): {:.3}",
                derivation.choice_activation
            ));
        }

        // Confidence
        if self.show_confidence {
            ui.separator();
            ui.label("Derivation Confidence:");
            let confidence_color = if derivation.confidence > 0.7 {
                Color32::GREEN
            } else if derivation.confidence > 0.4 {
                Color32::YELLOW
            } else {
                Color32::RED
            };
            ui.colored_label(
                confidence_color,
                format!("  {:.1}%", derivation.confidence * 100.0),
            );
        }
    }
}

/// Display data for quantum number derivation
#[derive(Debug, Clone)]
pub struct QuantumNumberDerivationDisplay {
    pub quantum_numbers: QuantumNumberSet,
    pub n_source: f64,
    pub l_source: f64,
    pub m_source: f64,
    pub s_source: f64,
    pub choice_activation: f64,
    pub confidence: f64,
}

/// Collapse visualization - show Free Will moment (Archetype 22 choice)
#[derive(Debug, Clone)]
pub struct CollapseVisualization {
    /// Animation time
    pub animation_time: f32,
    /// Show alternative possibilities
    pub show_alternatives: bool,
    /// Collapse in progress
    pub collapse_progress: f32,
    /// Show choice operator state
    pub show_choice_state: bool,
}

impl Default for CollapseVisualization {
    fn default() -> Self {
        Self {
            animation_time: 0.0,
            show_alternatives: true,
            collapse_progress: 0.0,
            show_choice_state: true,
        }
    }
}

impl CollapseVisualization {
    /// Create a new visualization
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
        if self.collapse_progress < 1.0 {
            self.collapse_progress = (self.collapse_progress + delta_time * 2.0).min(1.0);
        }
    }

    /// Render collapse visualization
    pub fn render(&self, ui: &mut egui::Ui, result: &CollapseResult) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(350.0, 250.0), egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Title
        painter.text(
            Pos2::new(center.x, rect.min.y + 15.0),
            egui::Align2::CENTER_TOP,
            "Wavefunction Collapse: Free Will (A22)",
            FontId::proportional(14.0),
            QuantumColors::collapse(),
        );

        // Draw collapse animation
        let collapse_center = Pos2::new(center.x, center.y - 20.0);
        self.draw_collapse_animation(&painter, collapse_center, result);

        // Draw choice info
        self.draw_choice_info(&painter, rect, result);

        // Draw alternatives if enabled
        if self.show_alternatives {
            self.draw_alternatives(&painter, rect, result);
        }

        response
    }

    /// Draw collapse animation
    fn draw_collapse_animation(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        result: &CollapseResult,
    ) {
        // Expanding ring animation
        let ring_radius = 20.0 + self.collapse_progress * 60.0;
        let alpha = ((1.0 - self.collapse_progress) * 255.0) as u8;

        painter.circle_stroke(
            center,
            ring_radius,
            Stroke::new(2.0, Color32::from_rgba_premultiplied(255, 215, 0, alpha)),
        );

        // Central collapsed state
        let collapsed_color = QuantumColors::collapse();
        let core_radius = 8.0 + self.collapse_progress * 4.0;

        painter.circle_filled(center, core_radius, collapsed_color);

        // Gold core
        painter.circle_filled(center, 3.0, Color32::WHITE);

        // Quantum number label
        let qn = &result.collapsed_quantum_numbers;
        let label = qn.to_spectroscopic_notation();
        painter.text(
            center + Vec2::new(0.0, core_radius + 10.0),
            egui::Align2::CENTER_TOP,
            label,
            FontId::proportional(10.0),
            collapsed_color,
        );
    }

    /// Draw choice information
    fn draw_choice_info(&self, painter: &egui::Painter, rect: Rect, result: &CollapseResult) {
        let info_y = rect.max.y - 80.0;

        // Collapse type
        let type_str = match result.collapse_type {
            CollapseType::ChoiceGuided => "Choice-Guided (Free Will)",
            CollapseType::Spontaneous => "Spontaneous",
            CollapseType::MeasurementInduced => "Measurement-Induced",
            CollapseType::EntanglementTriggered => "Entanglement-Triggered",
            CollapseType::DecoherenceDriven => "Decoherence-Driven",
        };

        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y),
            egui::Align2::LEFT_TOP,
            format!("Type: {}", type_str),
            FontId::proportional(10.0),
            if result.collapse_type == CollapseType::ChoiceGuided {
                QuantumColors::collapse()
            } else {
                Color32::LIGHT_GRAY
            },
        );

        // Choice strength
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 15.0),
            egui::Align2::LEFT_TOP,
            format!("Choice Strength: {:.2}", result.choice_strength),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Entropy change
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 30.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Entropy: {:.3} → {:.3}",
                result.pre_collapse_entropy, result.post_collapse_entropy
            ),
            FontId::proportional(9.0),
            Color32::LIGHT_GRAY,
        );

        // Archetype 22 note
        painter.text(
            Pos2::new(rect.min.x + 10.0, info_y + 45.0),
            egui::Align2::LEFT_TOP,
            "Collapse is NOT random - it is Free Will",
            FontId::proportional(8.0),
            Color32::from_rgba_premultiplied(150, 150, 150, 180),
        );
    }

    /// Draw alternative possibilities
    fn draw_alternatives(&self, painter: &egui::Painter, rect: Rect, result: &CollapseResult) {
        let alt_x = rect.max.x - 80.0;
        let mut alt_y = rect.min.y + 40.0;

        painter.text(
            Pos2::new(alt_x, alt_y),
            egui::Align2::RIGHT_TOP,
            "Alternatives:",
            FontId::proportional(9.0),
            Color32::GRAY,
        );

        alt_y += 12.0;
        for (_node_id, prob) in result.alternative_possibilities.iter().take(5) {
            let label = format!("{:.1}%", prob * 100.0);
            painter.text(
                Pos2::new(alt_x, alt_y),
                egui::Align2::RIGHT_TOP,
                label,
                FontId::proportional(8.0),
                Color32::from_rgba_premultiplied(150, 150, 150, 200),
            );
            alt_y += 10.0;
        }
    }

    /// Reset animation for new collapse
    pub fn reset_collapse(&mut self) {
        self.collapse_progress = 0.0;
    }

    /// Trigger new collapse
    pub fn trigger_collapse(&mut self) {
        self.collapse_progress = 0.0;
    }
}

/// Entanglement visualization - render phase correlation networks
#[derive(Debug, Clone)]
pub struct EntanglementVisualization {
    /// Animation time
    pub animation_time: f32,
    /// Show entanglement clusters
    pub show_clusters: bool,
    /// Show phase correlations
    pub show_phase: bool,
    /// Node positions (for rendering)
    pub node_positions: Vec<(QuantumNodeId, Pos2)>,
    /// Selected entanglement
    pub selected_entanglement: Option<usize>,
}

impl Default for EntanglementVisualization {
    fn default() -> Self {
        Self {
            animation_time: 0.0,
            show_clusters: true,
            show_phase: true,
            node_positions: Vec::new(),
            selected_entanglement: None,
        }
    }
}

impl EntanglementVisualization {
    /// Create a new visualization
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Calculate node positions from wavefunction
    pub fn calculate_positions(
        &mut self,
        wavefunction: &QuantumWavefunction,
        center: Pos2,
        radius: f32,
    ) {
        self.node_positions.clear();

        let nodes: Vec<&QuantumNode> = wavefunction.nodes().collect();
        let count = nodes.len();

        if count == 0 {
            return;
        }

        for (i, node) in nodes.iter().enumerate() {
            // Position in a circle
            let angle = (i as f32 / count as f32) * TAU;
            let n_factor = node.quantum_numbers.n as f32 / 7.0;
            let r = radius * (0.3 + n_factor * 0.7);

            let pos = Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin());

            self.node_positions.push((node.id, pos));
        }
    }

    /// Render entanglement network
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        entanglement_field: &EntanglementField,
    ) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(350.0, 300.0), egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.35;

        // Title
        painter.text(
            Pos2::new(center.x, rect.min.y + 15.0),
            egui::Align2::CENTER_TOP,
            "Entanglement Network (Phase Correlation)",
            FontId::proportional(12.0),
            Color32::from_rgb(180, 180, 255),
        );

        // Get statistics
        let stats = entanglement_field.statistics();

        // Draw entanglement connections using fallback visualization
        // (since we can't access private correlations field directly)
        self.draw_fallback_entanglements(&painter, center, radius, stats.total_entanglements);

        // Draw statistics
        self.draw_statistics(&painter, rect, &stats);

        // Explanation
        self.draw_explanation(&painter, rect);

        response
    }

    /// Draw fallback entanglement visualization
    fn draw_fallback_entanglements(
        &self,
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        count: usize,
    ) {
        // Draw nodes in a circle based on count
        let display_count = count.clamp(1, 12);
        let mut positions: Vec<Pos2> = Vec::new();

        for i in 0..display_count {
            let angle = (i as f32 / display_count as f32) * TAU;
            let r = radius * (0.4 + (i as f32 * 0.05));
            let pos = Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin());
            positions.push(pos);

            // Draw node
            painter.circle_filled(pos, 5.0, Color32::from_rgb(150, 150, 200));
        }

        // Draw entanglement lines connecting nodes
        for i in 0..positions.len() {
            let next_i = (i + 1) % positions.len();

            // Animated pulse
            let pulse = (self.animation_time * 2.0 + i as f32 * 0.5).sin() * 0.3 + 0.7;
            let line_alpha = (150.0 * pulse) as u8;

            painter.line_segment(
                [positions[i], positions[next_i]],
                Stroke::new(
                    2.0,
                    Color32::from_rgba_premultiplied(200, 180, 100, line_alpha),
                ),
            );
        }
    }

    /// Draw entanglement connections from stored positions
    #[allow(dead_code)]
    fn draw_entanglement_connections(
        &self,
        painter: &egui::Painter,
        correlations: &[&EntanglementCorrelation],
    ) {
        for corr in correlations {
            let pos1 = self
                .node_positions
                .iter()
                .find(|(id, _)| *id == corr.node1_id)
                .map(|(_, p)| *p);

            let pos2 = self
                .node_positions
                .iter()
                .find(|(id, _)| *id == corr.node2_id)
                .map(|(_, p)| *p);

            if let (Some(p1), Some(p2)) = (pos1, pos2) {
                let strength = corr.entanglement_strength() as f32;
                let color = QuantumColors::from_entanglement_strength(corr.entanglement_strength());

                // Animated pulse
                let pulse = (self.animation_time * 3.0).sin() * 0.2 + 0.8;

                painter.line_segment(
                    [p1, p2],
                    Stroke::new(
                        1.0 + strength * 2.0,
                        Color32::from_rgba_premultiplied(
                            color.r(),
                            color.g(),
                            color.b(),
                            (50.0 + strength * 150.0 * pulse) as u8,
                        ),
                    ),
                );
            }
        }

        // Draw nodes
        for (_, pos) in &self.node_positions {
            painter.circle_filled(*pos, 4.0, Color32::from_rgb(100, 150, 255));
            painter.circle_filled(*pos, 2.0, Color32::WHITE);
        }
    }

    /// Draw entanglement statistics
    fn draw_statistics(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        stats: &EntanglementFieldStatistics,
    ) {
        let stats_y = rect.max.y - 70.0;

        painter.text(
            Pos2::new(rect.min.x + 10.0, stats_y),
            egui::Align2::LEFT_TOP,
            format!("Entanglements: {}", stats.total_entanglements),
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );

        painter.text(
            Pos2::new(rect.min.x + 10.0, stats_y + 15.0),
            egui::Align2::LEFT_TOP,
            format!("Global Entanglement: {:.2}", stats.global_entanglement),
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );

        painter.text(
            Pos2::new(rect.min.x + 10.0, stats_y + 30.0),
            egui::Align2::LEFT_TOP,
            format!("Clusters: {}", stats.num_clusters),
            FontId::proportional(10.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Draw explanation
    fn draw_explanation(&self, painter: &egui::Painter, rect: Rect) {
        let exp_y = rect.max.y - 20.0;

        painter.text(
            Pos2::new(rect.min.x + 10.0, exp_y),
            egui::Align2::LEFT_TOP,
            "Entanglement = Phase Correlation (not action-at-a-distance)",
            FontId::proportional(8.0),
            Color32::from_rgba_premultiplied(150, 150, 150, 180),
        );
    }
}

/// Observer effect visualization - show cache invalidation on observation
#[derive(Debug, Clone)]
pub struct ObserverEffectVisualization {
    /// Animation time
    pub animation_time: f32,
    /// Observation progress (0 to 1)
    pub observation_progress: f32,
    /// Cache state
    pub cache_state: CacheState,
    /// Last observation result
    pub last_observation: Option<ObservationResult>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CacheState {
    Valid,
    Invalidating,
    Invalid,
    Refreshing,
}

impl Default for ObserverEffectVisualization {
    fn default() -> Self {
        Self {
            animation_time: 0.0,
            observation_progress: 0.0,
            cache_state: CacheState::Valid,
            last_observation: None,
        }
    }
}

impl ObserverEffectVisualization {
    /// Create a new visualization
    pub fn new() -> Self {
        Self::default()
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;

        // Animate observation
        if self.observation_progress < 1.0 {
            self.observation_progress = (self.observation_progress + delta_time * 3.0).min(1.0);

            // Update cache state based on progress
            if self.observation_progress <= 0.3 {
                self.cache_state = CacheState::Valid;
            } else if self.observation_progress <= 0.7 {
                self.cache_state = CacheState::Invalidating;
            } else if self.observation_progress < 1.0 {
                self.cache_state = CacheState::Invalid;
            } else {
                self.cache_state = CacheState::Refreshing;
            }
        }
    }

    /// Trigger observation
    pub fn observe(&mut self, collapsed: bool) {
        self.observation_progress = 0.0;
        self.cache_state = CacheState::Valid;
        self.last_observation = Some(ObservationResult {
            timestamp: self.animation_time,
            collapsed_state: collapsed,
            cache_hit: false,
        });
    }

    /// Render observer effect visualization
    pub fn render(&self, ui: &mut egui::Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 180.0), egui::Sense::hover());

        let rect = response.rect;
        let center = rect.center();

        // Title
        painter.text(
            Pos2::new(center.x, rect.min.y + 15.0),
            egui::Align2::CENTER_TOP,
            "Observer Effect = Cache Invalidation",
            FontId::proportional(12.0),
            Color32::from_rgb(200, 180, 255),
        );

        // Draw cache representation
        self.draw_cache(&painter, center);

        // Draw observation wave
        self.draw_observation_wave(&painter, center);

        // Draw state label
        self.draw_state_label(&painter, rect);

        // Draw explanation
        self.draw_explanation(&painter, rect);

        response
    }

    /// Draw cache representation
    fn draw_cache(&self, painter: &egui::Painter, center: Pos2) {
        let cache_pos = Pos2::new(center.x - 60.0, center.y - 10.0);
        let cache_size = Vec2::new(80.0, 60.0);

        // Cache box
        let cache_color = match self.cache_state {
            CacheState::Valid => QuantumColors::cache_hit(),
            CacheState::Invalidating => Color32::from_rgb(200, 150, 50),
            CacheState::Invalid => QuantumColors::cache_miss(),
            CacheState::Refreshing => Color32::from_rgb(100, 150, 200),
        };

        painter.rect_filled(
            Rect::from_min_size(cache_pos, cache_size),
            3.0,
            Color32::from_rgba_premultiplied(50, 50, 70, 200),
        );

        painter.rect_stroke(
            Rect::from_min_size(cache_pos, cache_size),
            3.0,
            Stroke::new(2.0, cache_color),
        );

        // Cache label
        painter.text(
            cache_pos + Vec2::new(cache_size.x / 2.0, 8.0),
            egui::Align2::CENTER_TOP,
            "Holographic",
            FontId::proportional(8.0),
            Color32::GRAY,
        );
        painter.text(
            cache_pos + Vec2::new(cache_size.x / 2.0, 20.0),
            egui::Align2::CENTER_TOP,
            "Cache",
            FontId::proportional(8.0),
            Color32::GRAY,
        );

        // Status indicator
        let status_text = match self.cache_state {
            CacheState::Valid => "HIT",
            CacheState::Invalidating => "INVALIDATING...",
            CacheState::Invalid => "MISS",
            CacheState::Refreshing => "REFRESHING...",
        };

        let status_color = match self.cache_state {
            CacheState::Valid => QuantumColors::cache_hit(),
            CacheState::Invalidating => Color32::YELLOW,
            CacheState::Invalid => QuantumColors::cache_miss(),
            CacheState::Refreshing => Color32::from_rgb(100, 150, 255),
        };

        painter.text(
            cache_pos + Vec2::new(cache_size.x / 2.0, 45.0),
            egui::Align2::CENTER_TOP,
            status_text,
            FontId::proportional(10.0),
            status_color,
        );
    }

    /// Draw observation wave
    fn draw_observation_wave(&self, painter: &egui::Painter, center: Pos2) {
        let wave_origin = Pos2::new(center.x + 50.0, center.y);

        // Draw expanding wave
        let wave_radius = self.observation_progress * 60.0;
        let alpha = ((1.0 - self.observation_progress) * 200.0) as u8;

        painter.circle_stroke(
            wave_origin,
            wave_radius,
            Stroke::new(2.0, Color32::from_rgba_premultiplied(200, 200, 255, alpha)),
        );

        // Draw observer eye
        let eye_pos = wave_origin + Vec2::new(30.0, 0.0);

        // Eye outline
        painter.circle_stroke(
            eye_pos,
            12.0,
            Stroke::new(2.0, Color32::from_rgb(200, 200, 255)),
        );

        // Pupil (changes with observation)
        let pupil_size = 4.0 + self.observation_progress * 4.0;
        painter.circle_filled(eye_pos, pupil_size, Color32::from_rgb(150, 150, 255));

        // Label
        painter.text(
            eye_pos + Vec2::new(0.0, 20.0),
            egui::Align2::CENTER_TOP,
            "Observer",
            FontId::proportional(8.0),
            Color32::LIGHT_GRAY,
        );
    }

    /// Draw state label
    fn draw_state_label(&self, painter: &egui::Painter, rect: Rect) {
        let label_y = rect.max.y - 45.0;

        let (text, color) = match self.cache_state {
            CacheState::Valid => ("Observation: Cache Valid (State Unchanged)", Color32::GREEN),
            CacheState::Invalidating => ("Observation: Cache Being Invalidated", Color32::YELLOW),
            CacheState::Invalid => (
                "Observation: Cache Miss (Wavefunction Collapsed)",
                Color32::RED,
            ),
            CacheState::Refreshing => (
                "Observation: Cache Refreshing",
                Color32::from_rgb(100, 150, 255),
            ),
        };

        painter.text(
            Pos2::new(rect.min.x + 10.0, label_y),
            egui::Align2::LEFT_TOP,
            text,
            FontId::proportional(9.0),
            color,
        );
    }

    /// Draw explanation
    fn draw_explanation(&self, painter: &egui::Painter, rect: Rect) {
        let exp_y = rect.max.y - 25.0;

        painter.text(
            Pos2::new(rect.min.x + 10.0, exp_y),
            egui::Align2::LEFT_TOP,
            "Observer effect IS cache invalidation in the holographic field",
            FontId::proportional(7.0),
            Color32::from_rgba_premultiplied(150, 150, 150, 180),
        );
    }

    /// Reset for new observation
    pub fn reset(&mut self) {
        self.observation_progress = 0.0;
        self.cache_state = CacheState::Valid;
    }
}

/// Result of an observation
#[derive(Debug, Clone)]
pub struct ObservationResult {
    pub timestamp: f32,
    pub collapsed_state: bool,
    pub cache_hit: bool,
}

/// Complete quantum visualization panel data
#[derive(Debug, Clone)]
pub struct QuantumVisualizationPanel {
    pub wavefunction_renderer: WavefunctionRenderer,
    pub derivation_view: QuantumNumberDerivationView,
    pub collapse_viz: CollapseVisualization,
    pub entanglement_viz: EntanglementVisualization,
    pub observer_effect_viz: ObserverEffectVisualization,
}

impl Default for QuantumVisualizationPanel {
    fn default() -> Self {
        Self {
            wavefunction_renderer: WavefunctionRenderer::new(),
            derivation_view: QuantumNumberDerivationView::new(),
            collapse_viz: CollapseVisualization::new(),
            entanglement_viz: EntanglementVisualization::new(),
            observer_effect_viz: ObserverEffectVisualization::new(),
        }
    }
}

impl QuantumVisualizationPanel {
    /// Create a new panel
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all visualizations
    pub fn update(&mut self, delta_time: f32) {
        self.wavefunction_renderer.update(delta_time);
        self.collapse_viz.update(delta_time);
        self.entanglement_viz.update(delta_time);
        self.observer_effect_viz.update(delta_time);
    }

    /// Render the complete panel
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        wavefunction: &QuantumWavefunction,
        entanglement_field: &EntanglementField,
        archetype_vector: &[f64; 22],
        collapse_result: Option<&CollapseResult>,
    ) {
        ui.heading("Quantum Level Visualization");
        ui.separator();

        // Wavefunction
        ui.label("Wavefunction Probability Cloud");
        self.wavefunction_renderer.render(ui, wavefunction);
        ui.separator();

        // Quantum number derivation
        let derivation = QuantumNumberDerivationView::derive_from_archetypes(archetype_vector);
        self.derivation_view.render(ui, &derivation);
        ui.separator();

        // Collapse visualization
        if let Some(result) = collapse_result {
            ui.label("Wavefunction Collapse");
            self.collapse_viz.render(ui, result);
            ui.separator();
        }

        // Entanglement
        ui.label("Entanglement Network");
        self.entanglement_viz.render(ui, entanglement_field);
        ui.separator();

        // Observer effect
        ui.label("Observer Effect");
        self.observer_effect_viz.render(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_colors_from_phase() {
        let color = QuantumColors::from_phase(0.0);
        assert!(color.r() > 0 || color.g() > 0 || color.b() > 0);

        let color_pi = QuantumColors::from_phase(std::f64::consts::PI);
        assert!(color_pi.r() > 0 || color_pi.g() > 0 || color_pi.b() > 0);
    }

    #[test]
    fn test_quantum_colors_from_probability() {
        let color = QuantumColors::from_probability(0.5);
        assert!(color.r() > 0 || color.g() > 0 || color.b() > 0);

        let color_high = QuantumColors::from_probability(1.0);
        assert!(color_high.r() > 200);
    }

    #[test]
    fn test_entanglement_colors() {
        let color = QuantumColors::from_entanglement_strength(0.8);
        assert!(color.r() > 150); // Should be bright gold
    }

    #[test]
    fn test_wavefunction_renderer() {
        let mut renderer = WavefunctionRenderer::new();
        renderer.update(0.1);
        assert!(renderer.animation_time > 0.0);
    }

    #[test]
    fn test_quantum_derivation() {
        let archetype = [0.5f64; 22];
        let derivation = QuantumNumberDerivationView::derive_from_archetypes(&archetype);

        assert!(derivation.quantum_numbers.n >= 1);
        assert!(derivation.confidence >= 0.0 && derivation.confidence <= 1.0);
    }

    #[test]
    fn test_collapse_visualization() {
        let mut viz = CollapseVisualization::new();
        viz.update(0.1);
        assert!(viz.animation_time > 0.0);
    }

    #[test]
    fn test_entanglement_visualization() {
        let viz = EntanglementVisualization::new();
        assert!(viz.show_clusters);
    }

    #[test]
    fn test_observer_effect() {
        let mut viz = ObserverEffectVisualization::new();

        // Initial state should be Valid
        assert_eq!(viz.cache_state, CacheState::Valid);

        // After observe, progress should be 0
        viz.observe(true);
        assert_eq!(viz.observation_progress, 0.0);

        // After very small update, should still be Valid (< 0.3 threshold)
        viz.update(0.05);
        assert_eq!(viz.cache_state, CacheState::Valid);
    }

    #[test]
    fn test_panel_update() {
        let mut panel = QuantumVisualizationPanel::new();
        panel.update(0.1);

        assert!(panel.wavefunction_renderer.animation_time > 0.0);
    }
}
