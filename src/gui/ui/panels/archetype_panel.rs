//! Archetype Panel - 22-Archetype System Visualization
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.3:
//! "Design 22-archetype radar chart, Implement archetype interference display,
//!  Show archetype → property derivation chain, Add archetype profile comparison,
//!  Implement archetype evolution animation"
//!
//! This module provides:
//! - Interactive 22-archetype radar chart
//! - Interference pattern visualization
//! - Property derivation chain display
//! - Profile comparison between entities
//! - Evolution animation controls

use egui::{Color32, Context, Ui};

use crate::gui::visualization::archetype_viz::{
    get_archetype_color, get_complex_color, ArchetypeProfileData, ArchetypeRadarChart, InterferenceDisplay, PropertyDerivation, ARCHETYPE_NAMES,
    ARCHETYPE_SHORT_NAMES, BODY_ARCHETYPES, CHOICE_ARCHETYPE, MIND_ARCHETYPES, NUM_ARCHETYPES,
    SPIRIT_ARCHETYPES,
};

use crate::entity_layer7::layer7::SubSubLogos;

/// Archetype Panel for Phase B.3
pub struct ArchetypePanel {
    /// Panel visibility
    pub visible: bool,

    /// Radar chart renderer
    radar_chart: ArchetypeRadarChart,

    /// Current profile being displayed
    current_profile: ArchetypeProfileData,

    /// Comparison profile (for side-by-side comparison)
    comparison_profile: Option<ArchetypeProfileData>,

    /// Show comparison mode
    pub show_comparison: bool,

    /// Show interference display
    pub show_interference: bool,

    /// Show property derivation
    pub show_derivation: bool,

    /// Show evolution animation
    pub show_evolution: bool,

    /// Evolution animation time
    evolution_time: f32,

    /// Evolution speed
    evolution_speed: f32,

    /// Evolution history (for animation)
    evolution_history: Vec<ArchetypeProfileData>,

    /// Maximum history length
    max_history: usize,

    /// Selected archetype for detail view
    selected_archetype: Option<usize>,

    /// View mode
    pub view_mode: ArchetypeViewMode,

    /// Entity ID being inspected
    current_entity_id: Option<u64>,
}

/// View mode for archetype panel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchetypeViewMode {
    /// Full radar chart
    RadarChart,
    /// Complex breakdown
    ComplexBreakdown,
    /// Individual archetype detail
    ArchetypeDetail,
    /// Evolution timeline
    EvolutionTimeline,
}

impl ArchetypePanel {
    /// Create a new archetype panel
    pub fn new() -> Self {
        Self {
            visible: true,
            radar_chart: ArchetypeRadarChart::new(),
            current_profile: ArchetypeProfileData::default_profile(),
            comparison_profile: None,
            show_comparison: false,
            show_interference: true,
            show_derivation: true,
            show_evolution: false,
            evolution_time: 0.0,
            evolution_speed: 1.0,
            evolution_history: Vec::new(),
            max_history: 100,
            selected_archetype: None,
            view_mode: ArchetypeViewMode::RadarChart,
            current_entity_id: None,
        }
    }

    /// Update from entity
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        // Convert entity's archetype activations
        let mut activations = [0.0f32; NUM_ARCHETYPES];
        for (i, activation) in entity.archetype_activations.iter().enumerate() {
            if i < NUM_ARCHETYPES {
                activations[i] = *activation as f32;
            }
        }

        self.current_profile = ArchetypeProfileData::new(activations);
        self.current_entity_id = Some(entity.entity_id.as_u64());

        // Add to evolution history
        if self.show_evolution {
            self.evolution_history.push(self.current_profile.clone());
            if self.evolution_history.len() > self.max_history {
                self.evolution_history.remove(0);
            }
        }
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) {
        self.radar_chart.update(delta_time);

        if self.show_evolution {
            self.evolution_time += delta_time * self.evolution_speed;
        }
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("🔮 Archetype System")
            .collapsible(true)
            .resizable(true)
            .default_width(500.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                self.show_content(ui);
            });
    }

    /// Show panel content
    fn show_content(&mut self, ui: &mut Ui) {
        // Mode selector
        self.show_mode_selector(ui);
        ui.separator();

        match self.view_mode {
            ArchetypeViewMode::RadarChart => {
                self.show_radar_chart_mode(ui);
            }
            ArchetypeViewMode::ComplexBreakdown => {
                self.show_complex_breakdown(ui);
            }
            ArchetypeViewMode::ArchetypeDetail => {
                self.show_archetype_detail(ui);
            }
            ArchetypeViewMode::EvolutionTimeline => {
                self.show_evolution_timeline(ui);
            }
        }
    }

    /// Show mode selector
    fn show_mode_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.view_mode,
                ArchetypeViewMode::RadarChart,
                "📊 Radar",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ArchetypeViewMode::ComplexBreakdown,
                "🔄 Complexes",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ArchetypeViewMode::ArchetypeDetail,
                "🔍 Detail",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ArchetypeViewMode::EvolutionTimeline,
                "📈 Evolution",
            );
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_interference, "Interference");
            ui.checkbox(&mut self.show_derivation, "Derivation");
            ui.checkbox(&mut self.show_comparison, "Compare");
            ui.checkbox(&mut self.show_evolution, "Animate");
        });
    }

    /// Show radar chart mode
    fn show_radar_chart_mode(&mut self, ui: &mut Ui) {
        // Radar chart
        self.radar_chart.render(ui, &self.current_profile);

        // Dominant archetype info
        ui.add_space(10.0);
        ui.label(format!(
            "Dominant: {}",
            self.current_profile.dominant_name()
        ));
        ui.label(format!(
            "Coherence: {:.2} | Entropy: {:.2}",
            self.current_profile.coherence, self.current_profile.entropy
        ));

        // Interference display
        if self.show_interference {
            ui.collapsing("Interference Pattern", |ui| {
                let interference = InterferenceDisplay::from_profile(&self.current_profile);
                interference.render(ui);
            });
        }

        // Property derivation
        if self.show_derivation {
            ui.collapsing("Property Derivation", |ui| {
                let derivation = PropertyDerivation::from_profile(&self.current_profile);
                derivation.render(ui);
            });
        }

        // Comparison mode
        if self.show_comparison {
            if let Some(ref comparison) = self.comparison_profile {
                ui.collapsing("Comparison", |ui| {
                    self.show_profile_comparison(ui, &self.current_profile, comparison);
                });
            } else {
                ui.label("No comparison profile set");
            }
        }
    }

    /// Show complex breakdown
    fn show_complex_breakdown(&mut self, ui: &mut Ui) {
        ui.heading("Complex Breakdown");

        let complexes = [
            (
                "Mind Complex (σA)",
                MIND_ARCHETYPES,
                self.current_profile.mind_complex(),
            ),
            (
                "Body Complex (σB)",
                BODY_ARCHETYPES,
                self.current_profile.body_complex(),
            ),
            (
                "Spirit Complex (σC)",
                SPIRIT_ARCHETYPES,
                self.current_profile.spirit_complex(),
            ),
        ];

        for (name, indices, activations) in complexes {
            ui.collapsing(name, |ui| {
                let avg: f32 = activations.iter().sum::<f32>() / 7.0;
                ui.label(format!("Average Activation: {:.2}", avg));

                for (i, &idx) in indices.iter().enumerate() {
                    let activation = activations[i];
                    ui.horizontal(|ui| {
                        ui.add_sized([150.0, 16.0], egui::Label::new(ARCHETYPE_SHORT_NAMES[idx]));
                        ui.add_sized(
                            [100.0, 16.0],
                            egui::ProgressBar::new(activation)
                                .fill(get_archetype_color(idx))
                                .text(format!("{:.0}%", activation * 100.0)),
                        );
                    });
                }
            });
        }

        // A22 (The Choice)
        ui.collapsing("The Choice (A22)", |ui| {
            let choice = self.current_profile.activations[CHOICE_ARCHETYPE];
            ui.add_sized(
                [100.0, 20.0],
                egui::ProgressBar::new(choice)
                    .fill(get_archetype_color(CHOICE_ARCHETYPE))
                    .text(format!("{:.0}%", choice * 100.0)),
            );
            ui.label("A22 is the gateway where infinity becomes accessible.");
            ui.label("It emerges from Free Will and unifies all other archetypes.");
        });

        // Complex averages visualization
        ui.add_space(10.0);
        ui.heading("Complex Balance");

        let averages = self.current_profile.complex_averages();
        let rect = ui.allocate_space(egui::vec2(ui.available_width(), 80.0)).1;
        let painter = ui.painter();

        let bar_width = rect.width() / 4.0 - 10.0;
        let max_height = rect.height() - 20.0;

        for (i, &avg) in averages.iter().enumerate() {
            let bar_height = avg * max_height;
            let x = rect.min.x + i as f32 * (bar_width + 10.0);

            let color = get_complex_color(i);
            let bar_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.max.y - bar_height),
                egui::vec2(bar_width, bar_height),
            );

            painter.rect_filled(bar_rect, 3.0, color);

            // Label
            painter.text(
                egui::pos2(x + bar_width / 2.0, rect.min.y),
                egui::Align2::CENTER_TOP,
                ["Mind", "Body", "Spirit", "Choice"][i],
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );
        }
    }

    /// Show archetype detail view
    fn show_archetype_detail(&mut self, ui: &mut Ui) {
        ui.heading("Archetype Detail View");

        // Archetype selector
        ui.horizontal(|ui| {
            ui.label("Select Archetype:");
            egui::ComboBox::new("archetype_selector", "")
                .selected_text(
                    self.selected_archetype
                        .map(|i| ARCHETYPE_SHORT_NAMES[i])
                        .unwrap_or("None"),
                )
                .show_ui(ui, |ui| {
                    for (i, name) in ARCHETYPE_SHORT_NAMES.iter().enumerate().take(NUM_ARCHETYPES) {
                        let selected = self.selected_archetype == Some(i);
                        if ui.selectable_label(selected, *name).clicked() {
                            self.selected_archetype = Some(i);
                        }
                    }
                });
        });

        if let Some(idx) = self.selected_archetype {
            self.show_single_archetype_detail(ui, idx);
        } else {
            // Show top 5 dominant
            ui.label("Top 5 Dominant Archetypes:");
            for &idx in &self.current_profile.dominant_indices {
                self.show_single_archetype_detail(ui, idx);
                ui.separator();
            }
        }
    }

    /// Show single archetype detail
    fn show_single_archetype_detail(&self, ui: &mut Ui, idx: usize) {
        let activation = self.current_profile.activations[idx];
        let color = get_archetype_color(idx);

        // Header with color
        ui.horizontal(|ui| {
            let (r, g, b) = (color.r(), color.g(), color.b());
            ui.add_sized(
                [20.0, 20.0],
                egui::Label::new(egui::RichText::new("■").color(Color32::from_rgb(r, g, b))),
            );
            ui.strong(ARCHETYPE_SHORT_NAMES[idx]);
        });

        ui.label(ARCHETYPE_NAMES[idx]);

        // Activation bar
        ui.add_sized(
            [200.0, 20.0],
            egui::ProgressBar::new(activation)
                .fill(color)
                .text(format!("{:.1}%", activation * 100.0)),
        );

        // Complex classification
        let complex = if MIND_ARCHETYPES.contains(&idx) {
            "Mind Complex"
        } else if BODY_ARCHETYPES.contains(&idx) {
            "Body Complex"
        } else if SPIRIT_ARCHETYPES.contains(&idx) {
            "Spirit Complex"
        } else {
            "The Choice"
        };
        ui.label(format!("Belongs to: {}", complex));

        // Role description
        let role = Self::get_archetype_role(idx);
        ui.label(role);
    }

    /// Get archetype role description
    fn get_archetype_role(idx: usize) -> &'static str {
        match idx {
            0 => "Creates the foundation of mental perception",
            1 => "Enhances and amplifies mental patterns",
            2 => "Triggers transformation through mental challenge",
            3 => "Accumulates wisdom through mental experience",
            4 => "Assigns meaning to mental phenomena",
            5 => "Converts mental patterns to new forms",
            6 => "Integrates all mental archetypes into wholeness",
            7 => "Creates the foundation of physical experience",
            8 => "Enhances and amplifies bodily sensations",
            9 => "Triggers transformation through physical challenge",
            10 => "Accumulates wisdom through bodily experience",
            11 => "Assigns meaning to physical phenomena",
            12 => "Converts bodily patterns to new forms",
            13 => "Integrates all body archetypes into wholeness",
            14 => "Creates the foundation of spiritual connection",
            15 => "Enhances and amplifies spiritual insight",
            16 => "Triggers transformation through spiritual challenge",
            17 => "Accumulates wisdom through spiritual experience",
            18 => "Assigns meaning to spiritual phenomena",
            19 => "Converts spiritual patterns to new forms",
            20 => "Integrates all spirit archetypes into wholeness",
            21 => "The gateway where infinity becomes accessible - Free Will",
            _ => "Unknown archetype",
        }
    }

    /// Show evolution timeline
    fn show_evolution_timeline(&mut self, ui: &mut Ui) {
        ui.heading("Archetype Evolution");

        if self.evolution_history.len() < 2 {
            ui.label("Not enough evolution history. Keep observing an entity.");
            return;
        }

        // Animation controls
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut self.evolution_speed, 0.1..=5.0).text(""));
            if ui.button("Reset").clicked() {
                self.evolution_history.clear();
            }
        });

        // Evolution graph
        let rect = ui.allocate_space(egui::vec2(ui.available_width(), 150.0)).1;
        let painter = ui.painter();

        // Draw evolution lines for dominant archetypes
        let dominant_indices = self.current_profile.dominant_indices;

        for &arch_idx in &dominant_indices {
            let color = get_archetype_color(arch_idx);
            let points: Vec<egui::Pos2> = self
                .evolution_history
                .iter()
                .enumerate()
                .map(|(i, profile)| {
                    let x = rect.min.x
                        + (i as f32 / self.evolution_history.len().max(1) as f32) * rect.width();
                    let y = rect.max.y - profile.activations[arch_idx] * rect.height();
                    egui::pos2(x, y)
                })
                .collect();

            if points.len() > 1 {
                for i in 0..points.len() - 1 {
                    painter.line_segment(
                        [points[i], points[i + 1]],
                        (
                            2.0,
                            Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 200),
                        ),
                    );
                }
            }
        }

        // Labels
        painter.text(
            rect.min,
            egui::Align2::LEFT_TOP,
            "Evolution Timeline",
            egui::FontId::proportional(10.0),
            Color32::WHITE,
        );

        // Statistics
        ui.add_space(10.0);
        ui.label(format!(
            "History length: {} frames",
            self.evolution_history.len()
        ));

        // Change from start to now
        if let Some(first) = self.evolution_history.first() {
            ui.label("Changes from start:");
            for &idx in &dominant_indices[..3] {
                let change = self.current_profile.activations[idx] - first.activations[idx];
                let change_str = if change > 0.0 {
                    format!("+{:.2}", change)
                } else {
                    format!("{:.2}", change)
                };
                ui.colored_label(
                    get_archetype_color(idx),
                    format!("{}: {}", ARCHETYPE_SHORT_NAMES[idx], change_str),
                );
            }
        }
    }

    /// Show profile comparison
    fn show_profile_comparison(
        &self,
        ui: &mut Ui,
        profile1: &ArchetypeProfileData,
        profile2: &ArchetypeProfileData,
    ) {
        ui.label("Comparing profiles:");

        for (i, (a1, a2)) in profile1.activations.iter().zip(profile2.activations.iter()).enumerate().take(NUM_ARCHETYPES) {
            let diff = a1 - a2;
            if diff.abs() > 0.1 {
                ui.horizontal(|ui| {
                    ui.add_sized([40.0, 16.0], egui::Label::new(ARCHETYPE_SHORT_NAMES[i]));
                    let diff_text = if diff > 0.0 {
                        format!("+{:.2}", diff)
                    } else {
                        format!("{:.2}", diff)
                    };
                    let color = if diff > 0.0 {
                        Color32::GREEN
                    } else {
                        Color32::RED
                    };
                    ui.colored_label(color, diff_text);
                });
            }
        }

        ui.add_space(10.0);
        ui.label(format!(
            "Coherence diff: {:.3}",
            profile1.coherence - profile2.coherence
        ));
        ui.label(format!(
            "Entropy diff: {:.3}",
            profile1.entropy - profile2.entropy
        ));
    }

    /// Set comparison profile
    pub fn set_comparison(&mut self, profile: ArchetypeProfileData) {
        self.comparison_profile = Some(profile);
    }

    /// Get current profile
    pub fn current_profile(&self) -> &ArchetypeProfileData {
        &self.current_profile
    }
}

impl Default for ArchetypePanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_panel_creation() {
        let panel = ArchetypePanel::new();
        assert!(panel.visible);
        assert_eq!(panel.view_mode, ArchetypeViewMode::RadarChart);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = ArchetypePanel::new();
        panel.view_mode = ArchetypeViewMode::ComplexBreakdown;
        assert_eq!(panel.view_mode, ArchetypeViewMode::ComplexBreakdown);
    }

    #[test]
    fn test_archetype_role() {
        let role = ArchetypePanel::get_archetype_role(0);
        assert!(role.contains("mental"));

        let choice_role = ArchetypePanel::get_archetype_role(CHOICE_ARCHETYPE);
        assert!(choice_role.contains("Free Will"));
    }
}
