//! Quantum Panel - Quantum Level Visualization Panel
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase C.1:
//! "Design 22-archetype quantum probability cloud, Implement wavefunction collapse
//!  as Free Will visualization, Show quantum number derivation from archetype activations,
//!  Implement entanglement as phase correlation networks, Show observer effect as cache invalidation"
//!
//! This module provides:
//! - Tabbed interface for different quantum views
//! - Wavefunction probability cloud visualization
//! - Quantum number derivation from archetype activations
//! - Collapse visualization (Free Will moment)
//! - Entanglement as phase correlation networks
//! - Observer effect as cache invalidation
//! - Entity integration for quantum number display

use egui::{Color32, Context, Ui, Vec2};
use std::collections::HashMap;

use crate::gui::visualization::quantum_viz::{
    CollapseVisualization, EntanglementVisualization, ObserverEffectVisualization, QuantumColors,
    QuantumNumberDerivationDisplay, QuantumNumberDerivationView, WavefunctionRenderer,
};
use crate::holographic_foundation::field_state::HolographicFieldState;
use crate::holographic_foundation::quantum_consciousness::entanglement_field::EntanglementField;
use crate::holographic_foundation::quantum_consciousness::wavefunction::QuantumWavefunction;

use crate::entity_layer7::layer7::SubSubLogos;

/// View mode for quantum panel tabs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuantumViewMode {
    /// Wavefunction probability cloud
    Wavefunction,
    /// Quantum numbers derivation from archetypes
    QuantumNumbers,
    /// Wavefunction collapse (Free Will)
    Collapse,
    /// Entanglement phase correlation networks
    Entanglement,
    /// Observer effect (cache invalidation)
    ObserverEffect,
}

impl Default for QuantumViewMode {
    fn default() -> Self {
        Self::Wavefunction
    }
}

/// Quantum Panel for Phase C.1
///
/// Provides integrated quantum visualization with tabbed interface for:
/// - Wavefunction: Probability cloud from quantum nodes
/// - Quantum Numbers: n, l, m, s derivation from archetypes
/// - Collapse: Free Will (A22) moment visualization
/// - Entanglement: Phase correlation network
/// - Observer Effect: Cache invalidation visualization
pub struct QuantumPanel {
    /// Panel visibility toggle
    pub visible: bool,

    /// Current tab/view mode
    pub view_mode: QuantumViewMode,

    /// Wavefunction renderer
    wavefunction_renderer: WavefunctionRenderer,

    /// Quantum number derivation view
    derivation_view: QuantumNumberDerivationView,

    /// Collapse visualization
    collapse_viz: CollapseVisualization,

    /// Entanglement visualization
    entanglement_viz: EntanglementVisualization,

    /// Observer effect visualization
    observer_effect_viz: ObserverEffectVisualization,

    /// Current entity being displayed
    current_entity: Option<SubSubLogos>,

    /// Current archetype vector for quantum derivation
    archetype_vector: [f64; 22],

    /// Quantum number derivation cache
    quantum_derivation: Option<QuantumNumberDerivationDisplay>,

    /// Display controls
    pub show_controls: bool,

    /// Animation time accumulator
    animation_time: f32,

    /// Entity selection map (id -> entity)
    entity_cache: HashMap<u64, SubSubLogos>,

    /// Selected entity ID
    selected_entity_id: Option<u64>,

    /// Collapse result history for display
    last_collapse_result: Option<CollapseDisplayData>,

    /// Field-derived wavefunction cache
    field_wavefunction: Option<QuantumWavefunction>,

    /// Field-derived entanglement cache
    field_entanglement: Option<EntanglementField>,
}

/// Display data for collapse results
#[derive(Debug, Clone)]
pub struct CollapseDisplayData {
    pub collapsed_notation: String,
    pub collapse_type: String,
    pub choice_strength: f64,
    pub pre_entropy: f64,
    pub post_entropy: f64,
}

impl QuantumPanel {
    /// Create a new quantum panel
    pub fn new() -> Self {
        Self {
            visible: true,
            view_mode: QuantumViewMode::Wavefunction,
            wavefunction_renderer: WavefunctionRenderer::new(),
            derivation_view: QuantumNumberDerivationView::new(),
            collapse_viz: CollapseVisualization::new(),
            entanglement_viz: EntanglementVisualization::new(),
            observer_effect_viz: ObserverEffectVisualization::new(),
            current_entity: None,
            archetype_vector: [0.0; 22],
            quantum_derivation: None,
            show_controls: true,
            animation_time: 0.0,
            entity_cache: HashMap::new(),
            selected_entity_id: None,
            last_collapse_result: None,
            field_wavefunction: None,
            field_entanglement: None,
        }
    }

    /// Update from entity - extract quantum-relevant data
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        self.current_entity = Some(entity.clone());
        self.selected_entity_id = Some(entity.entity_id.as_u64());

        // Cache entity
        self.entity_cache
            .insert(entity.entity_id.as_u64(), entity.clone());

        // Extract archetype activations for quantum derivation
        for (i, activation) in entity.archetype_activations.iter().enumerate() {
            if i < 22 {
                self.archetype_vector[i] = *activation as f64;
            }
        }

        // Compute quantum derivation from archetypes
        self.quantum_derivation = Some(QuantumNumberDerivationView::derive_from_archetypes(
            &self.archetype_vector,
        ));
    }

    /// Update animations
    pub fn update(&mut self, delta_time: f32) {
        self.update_with_field(delta_time, None);
    }

    /// Update animations and optionally refresh from field context
    pub fn update_with_field(
        &mut self,
        delta_time: f32,
        field_state: Option<&HolographicFieldState>,
    ) {
        self.animation_time += delta_time;

        if let Some(field_state) = field_state {
            self.refresh_from_field(field_state);
        }

        // Update all visualization components
        self.wavefunction_renderer.update(delta_time);
        self.collapse_viz.update(delta_time);
        self.entanglement_viz.update(delta_time);
        self.observer_effect_viz.update(delta_time);
    }

    fn refresh_from_field(&mut self, field_state: &HolographicFieldState) {
        let wavefunction = QuantumWavefunction::from_holographic_field(field_state);
        let nodes: Vec<_> = wavefunction.nodes().cloned().collect();

        let mut entanglement = EntanglementField::new();
        if !nodes.is_empty() {
            entanglement.detect_entanglements(&nodes);
            self.field_entanglement = Some(entanglement);
        } else {
            self.field_entanglement = None;
        }

        self.field_wavefunction = Some(wavefunction);
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("⚛️ Quantum Level")
            .collapsible(true)
            .resizable(true)
            .default_width(550.0)
            .default_height(650.0)
            .show(ctx, |ui| {
                self.show_content(ui);
            });
    }

    /// Show panel content
    fn show_content(&mut self, ui: &mut Ui) {
        // Entity selector
        self.show_entity_selector(ui);
        ui.separator();

        // Tab navigation
        self.show_tab_navigation(ui);
        ui.separator();

        // Show controls toggle
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_controls, "Show Controls");
        });

        // Render current tab content
        match self.view_mode {
            QuantumViewMode::Wavefunction => {
                self.show_wavefunction_view(ui);
            }
            QuantumViewMode::QuantumNumbers => {
                self.show_quantum_numbers_view(ui);
            }
            QuantumViewMode::Collapse => {
                self.show_collapse_view(ui);
            }
            QuantumViewMode::Entanglement => {
                self.show_entanglement_view(ui);
            }
            QuantumViewMode::ObserverEffect => {
                self.show_observer_effect_view(ui);
            }
        }
    }

    /// Show entity selector
    fn show_entity_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Entity:");

            // Build entity list
            let entity_ids: Vec<u64> = self.entity_cache.keys().cloned().collect();

            if entity_ids.is_empty() {
                ui.label("No entity selected");
                return;
            }

            // Simple selector using radio buttons
            for &id in &entity_ids {
                let label = format!("Entity {}", id);
                ui.selectable_value(&mut self.selected_entity_id, Some(id), &label);
            }
        });
    }

    /// Show tab navigation
    fn show_tab_navigation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.view_mode,
                QuantumViewMode::Wavefunction,
                "🌊 Wavefunction",
            );
            ui.selectable_value(
                &mut self.view_mode,
                QuantumViewMode::QuantumNumbers,
                "🔢 Quantum #",
            );
            ui.selectable_value(
                &mut self.view_mode,
                QuantumViewMode::Collapse,
                "💫 Collapse",
            );
            ui.selectable_value(
                &mut self.view_mode,
                QuantumViewMode::Entanglement,
                "🔗 Entangle",
            );
            ui.selectable_value(
                &mut self.view_mode,
                QuantumViewMode::ObserverEffect,
                "👁️ Observer",
            );
        });
    }

    /// Show wavefunction view (probability cloud)
    fn show_wavefunction_view(&mut self, ui: &mut Ui) {
        ui.heading("Wavefunction Probability Cloud");

        // Controls
        if self.show_controls {
            self.show_wavefunction_controls(ui);
            ui.separator();
        }

        // Render wavefunction visualization
        // For demo, create a minimal wavefunction from archetype data
        let fallback_wavefunction;
        let wavefunction = if let Some(field_wavefunction) = self.field_wavefunction.as_ref() {
            field_wavefunction
        } else {
            fallback_wavefunction = self.create_demo_wavefunction();
            &fallback_wavefunction
        };

        self.wavefunction_renderer.render(ui, &wavefunction);

        // Show legend
        self.show_phase_legend(ui);

        // Show current quantum state info
        if let Some(ref derivation) = self.quantum_derivation {
            ui.add_space(10.0);
            ui.label(format!(
                "Primary State: {}",
                derivation.quantum_numbers.to_spectroscopic_notation()
            ));
        }
    }

    /// Show wavefunction controls
    fn show_wavefunction_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Wavefunction Controls", |ui| {
            ui.checkbox(
                &mut self.wavefunction_renderer.show_probability,
                "Probability Density",
            );
            ui.checkbox(&mut self.wavefunction_renderer.show_phase, "Phase Coloring");
            ui.checkbox(
                &mut self.wavefunction_renderer.show_quantum_numbers,
                "Show Quantum Numbers",
            );

            ui.label("Scale:");
            ui.add(egui::Slider::new(&mut self.wavefunction_renderer.scale, 0.5..=2.0).text(""));
        });
    }

    /// Show quantum numbers derivation view
    fn show_quantum_numbers_view(&mut self, ui: &mut Ui) {
        ui.heading("Quantum Number Derivation");

        // Controls
        if self.show_controls {
            ui.collapsing("Derivation Controls", |ui| {
                ui.checkbox(
                    &mut self.derivation_view.show_chain,
                    "Show Derivation Chain",
                );
                ui.checkbox(
                    &mut self.derivation_view.show_archetype_source,
                    "Archetype Source",
                );
                ui.checkbox(
                    &mut self.derivation_view.show_confidence,
                    "Confidence Indicator",
                );
            });
            ui.separator();
        }

        // Render derivation
        if let Some(ref derivation) = self.quantum_derivation {
            self.derivation_view.render(ui, derivation);
        } else {
            ui.label("Select an entity to see quantum number derivation");
        }
    }

    /// Show collapse view (Free Will moment)
    fn show_collapse_view(&mut self, ui: &mut Ui) {
        ui.heading("Wavefunction Collapse: Free Will (A22)");

        // Controls
        if self.show_controls {
            ui.collapsing("Collapse Controls", |ui| {
                ui.checkbox(
                    &mut self.collapse_viz.show_alternatives,
                    "Show Alternatives",
                );
                ui.checkbox(
                    &mut self.collapse_viz.show_choice_state,
                    "Show Choice State",
                );

                if ui.button("Trigger Collapse").clicked() {
                    self.collapse_viz.trigger_collapse();
                    // Simulate a collapse result for display
                    if let Some(ref derivation) = self.quantum_derivation {
                        self.last_collapse_result = Some(CollapseDisplayData {
                            collapsed_notation: derivation
                                .quantum_numbers
                                .to_spectroscopic_notation(),
                            collapse_type: "Choice-Guided (Free Will)".to_string(),
                            choice_strength: derivation.choice_activation,
                            pre_entropy: 0.8,
                            post_entropy: 0.2,
                        });
                    }
                }

                if ui.button("Reset Animation").clicked() {
                    self.collapse_viz.reset_collapse();
                }
            });
            ui.separator();
        }

        // Render collapse visualization
        let collapse_result = self.create_demo_collapse_result();
        self.collapse_viz.render(ui, &collapse_result);

        // Show additional info
        if let Some(ref result) = self.last_collapse_result {
            ui.add_space(10.0);
            ui.separator();
            ui.label("Last Collapse Result:");
            ui.label(format!("  State: {}", result.collapsed_notation));
            ui.label(format!("  Type: {}", result.collapse_type));
            ui.label(format!("  Choice Strength: {:.2}", result.choice_strength));
            ui.label(format!(
                "  Entropy: {:.3} → {:.3}",
                result.pre_entropy, result.post_entropy
            ));
        }
    }

    /// Show entanglement view (phase correlation networks)
    fn show_entanglement_view(&mut self, ui: &mut Ui) {
        ui.heading("Entanglement Network (Phase Correlation)");

        // Controls
        if self.show_controls {
            ui.collapsing("Entanglement Controls", |ui| {
                ui.checkbox(&mut self.entanglement_viz.show_clusters, "Show Clusters");
                ui.checkbox(&mut self.entanglement_viz.show_phase, "Show Phase");
            });
            ui.separator();
        }

        // Render entanglement visualization
        let fallback_entanglement_field;
        let entanglement_field = if let Some(field_entanglement) = self.field_entanglement.as_ref()
        {
            field_entanglement
        } else {
            fallback_entanglement_field = self.create_demo_entanglement_field();
            &fallback_entanglement_field
        };
        self.entanglement_viz.render(ui, &entanglement_field);

        // Show explanation
        ui.add_space(10.0);
        ui.label("Entanglement is phase correlation, NOT action-at-a-distance.");
        ui.label("Correlated quantum states share phase relationships through");
        ui.label("the holographic field - information is not transmitted, it's shared.");
    }

    /// Show observer effect view (cache invalidation)
    fn show_observer_effect_view(&mut self, ui: &mut Ui) {
        ui.heading("Observer Effect = Cache Invalidation");

        // Controls
        if self.show_controls {
            ui.collapsing("Observer Controls", |ui| {
                if ui.button("Observe").clicked() {
                    self.observer_effect_viz.observe(true);
                }
                if ui.button("Reset").clicked() {
                    self.observer_effect_viz.reset();
                }
            });
            ui.separator();
        }

        // Render observer effect visualization
        self.observer_effect_viz.render(ui);

        // Show explanation
        ui.add_space(10.0);
        ui.label("In the holographic field, observation causes cache invalidation.");
        ui.label("The observer's attention 'refreshes' the local field representation,");
        ui.label("causing the wavefunction to collapse into a definite state.");
    }

    /// Show phase color legend
    fn show_phase_legend(&self, ui: &mut Ui) {
        ui.collapsing("Phase Color Legend", |ui| {
            let phases = [
                (0.0, "0 (Red)"),
                (std::f64::consts::PI / 3.0, "π/3 (Yellow)"),
                (std::f64::consts::PI * 2.0 / 3.0, "2π/3 (Green)"),
                (std::f64::consts::PI, "π (Cyan)"),
                (std::f64::consts::PI * 4.0 / 3.0, "4π/3 (Blue)"),
                (std::f64::consts::PI * 5.0 / 3.0, "5π/3 (Magenta)"),
            ];

            for (phase, label) in phases {
                let color = QuantumColors::from_phase(phase);
                ui.horizontal(|ui| {
                    let (r, g, b) = (color.r(), color.g(), color.b());
                    ui.add_sized(
                        [20.0, 16.0],
                        egui::Label::new(
                            egui::RichText::new("■").color(Color32::from_rgb(r, g, b)),
                        ),
                    );
                    ui.label(label);
                });
            }
        });
    }

    /// Create demo wavefunction from archetype data
    fn create_demo_wavefunction(
        &self,
    ) -> crate::holographic_foundation::quantum_consciousness::wavefunction::QuantumWavefunction
    {
        use crate::holographic_foundation::quantum_consciousness::wavefunction::QuantumWavefunction;

        // Create a basic wavefunction - the visualization will show what it can
        QuantumWavefunction::new()
    }

    /// Create demo collapse result
    fn create_demo_collapse_result(
        &self,
    ) -> crate::holographic_foundation::quantum_consciousness::archetype_collapse::CollapseResult
    {
        use crate::holographic_foundation::quantum_consciousness::archetype_collapse::{
            CollapseResult, CollapseType, EntanglementEffect,
        };
        use crate::holographic_foundation::quantum_consciousness::quantum_numbers::{
            QuantumNumberSet, Spin,
        };
        use crate::holographic_foundation::quantum_consciousness::wavefunction::QuantumNodeId;

        let qn = self
            .quantum_derivation
            .as_ref()
            .map(|d| d.quantum_numbers.clone())
            .unwrap_or_else(|| QuantumNumberSet::new(1, 0, 0, Spin::Up));

        let choice_strength = self
            .quantum_derivation
            .as_ref()
            .map(|d| d.choice_activation)
            .unwrap_or(0.5);

        // Create a default collapsed node id
        let collapsed_node_id = QuantumNodeId::new(0);

        CollapseResult {
            collapsed_node_id,
            collapsed_quantum_numbers: qn,
            collapse_type: CollapseType::ChoiceGuided,
            choice_strength,
            pre_collapse_entropy: 0.8,
            post_collapse_entropy: 0.1,
            alternative_possibilities: Vec::new(),
            entanglement_effects: Vec::new(),
        }
    }

    /// Create demo entanglement field
    fn create_demo_entanglement_field(
        &self,
    ) -> crate::holographic_foundation::quantum_consciousness::entanglement_field::EntanglementField
    {
        use crate::holographic_foundation::quantum_consciousness::entanglement_field::EntanglementField;

        EntanglementField::new()
    }

    /// Toggle visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get current entity
    pub fn current_entity(&self) -> Option<&SubSubLogos> {
        self.current_entity.as_ref()
    }
}

impl Default for QuantumPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_panel_creation() {
        let panel = QuantumPanel::new();
        assert!(panel.visible);
        assert_eq!(panel.view_mode, QuantumViewMode::Wavefunction);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = QuantumPanel::new();
        panel.view_mode = QuantumViewMode::Collapse;
        assert_eq!(panel.view_mode, QuantumViewMode::Collapse);

        panel.view_mode = QuantumViewMode::Entanglement;
        assert_eq!(panel.view_mode, QuantumViewMode::Entanglement);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut panel = QuantumPanel::new();
        assert!(panel.is_visible());

        panel.toggle_visibility();
        assert!(!panel.is_visible());

        panel.set_visible(true);
        assert!(panel.is_visible());
    }

    #[test]
    fn test_animation_update() {
        let mut panel = QuantumPanel::new();
        panel.update(0.1);
        assert!(panel.animation_time > 0.0);
    }
}
