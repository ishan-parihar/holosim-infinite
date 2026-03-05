//! Consciousness Panel - Phase D: Consciousness Integration Panel
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase D:
//! "Consciousness Integration: Free Will Kernel, Teleological Pull, Causal Inversion, Density Transitions"
//!
//! This module provides:
//! - Tabbed interface for different consciousness views
//! - Free Will Kernel visualization (Archetype 22 activation)
//! - Teleological Pull (Omega Point attraction)
//! - Causal Inversion (future→past influence)
//! - Density Transition visualization (1st→8th density progression)

use egui::{Color32, Context, Ui};

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::visualization::consciousness_viz::{
    CausalInversionView, DensityTransitionView, FreeWillVisualizer, TeleologicalPullView,
};

/// View mode for consciousness panel tabs
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ConsciousnessViewMode {
    /// Free Will Kernel visualization
    #[default]
    FreeWill,
    /// Teleological Pull (Omega Point attraction)
    TeleologicalPull,
    /// Causal Inversion (future→past influence)
    CausalInversion,
    /// Density Transition visualization
    DensityTransition,
}


/// Consciousness Panel for Phase D
///
/// Provides integrated consciousness visualization with tabbed interface for:
/// - FreeWill: Free Will Kernel visualization (A22 activation)
/// - TeleologicalPull: Omega Point attraction
/// - CausalInversion: Future→past influence
/// - DensityTransition: 1st→8th density progression
pub struct ConsciousnessPanel {
    /// Panel visibility toggle
    pub visible: bool,

    /// Current tab/view mode
    pub view_mode: ConsciousnessViewMode,

    /// Free Will visualizer for A22 kernel
    free_will_viz: FreeWillVisualizer,

    /// Teleological pull view for Omega Point attraction
    teleological_pull_viz: TeleologicalPullView,

    /// Causal inversion view for future→past influence
    causal_inversion_viz: CausalInversionView,

    /// Density transition view for density progression
    density_transition_viz: DensityTransitionView,

    /// Animation time accumulator
    pub animation_time: f32,

    /// Display controls
    pub show_controls: bool,

    /// Current entity being displayed
    current_entity: Option<SubSubLogos>,

    /// Selected entity ID
    selected_entity_id: Option<u64>,

    /// Free Will data cache
    cached_free_will_data: Option<FreeWillPanelData>,

    /// Teleological pull data cache
    cached_teleological_data: Option<TeleologicalPanelData>,

    /// Causal inversion data cache
    cached_causal_data: Option<CausalPanelData>,

    /// Density transition data cache
    cached_density_data: Option<DensityPanelData>,
}

/// Free Will panel rendering data
#[derive(Debug, Clone)]
struct FreeWillPanelData {
    pub archetype_22_activation: f32,
    pub choice_capacity: f32,
    pub free_will_strength: f32,
    pub karmic_patterns: Vec<f32>,
    pub choice_moments: usize,
}

/// Teleological pull panel data
#[derive(Debug, Clone)]
struct TeleologicalPanelData {
    pub omega_distance: f32,
    pub attraction_strength: f32,
    pub purpose_alignment: f32,
    pub evolutionary_direction: f32,
    pub telos_coordinates: [f32; 3],
}

/// Causal inversion panel data
#[derive(Debug, Clone)]
struct CausalPanelData {
    pub inversion_strength: f32,
    pub future_influence: f32,
    pub past_resonance: f32,
    pub causal_loops: usize,
    pub temporal_coherence: f32,
}

/// Density panel data
#[derive(Debug, Clone)]
struct DensityPanelData {
    pub current_density: u8,
    pub density_progress: [f32; 8],
    pub transition_readiness: f32,
    pub density_characteristics: Vec<DensityCharacteristic>,
}

/// Density characteristic descriptor
#[derive(Debug, Clone)]
struct DensityCharacteristic {
    pub name: String,
    pub description: String,
    pub activation: f32,
}

impl ConsciousnessPanel {
    /// Create a new consciousness panel
    pub fn new() -> Self {
        Self {
            visible: true,
            view_mode: ConsciousnessViewMode::FreeWill,
            free_will_viz: FreeWillVisualizer::new(),
            teleological_pull_viz: TeleologicalPullView::new(),
            causal_inversion_viz: CausalInversionView::new(),
            density_transition_viz: DensityTransitionView::new(),
            animation_time: 0.0,
            show_controls: true,
            current_entity: None,
            selected_entity_id: None,
            cached_free_will_data: None,
            cached_teleological_data: None,
            cached_causal_data: None,
            cached_density_data: None,
        }
    }

    /// Update from entity - extract consciousness-relevant data
    pub fn update_from_entity(&mut self, entity: &SubSubLogos) {
        self.current_entity = Some(entity.clone());
        self.selected_entity_id = Some(entity.entity_id.as_u64());

        // Generate cached data from entity
        self.generate_cached_data_from_entity(entity);
    }

    /// Generate cached data from entity
    fn generate_cached_data_from_entity(&mut self, entity: &SubSubLogos) {
        // Generate Free Will data
        let archetype_22 = entity.archetype_activations.get(21).copied().unwrap_or(0.5) as f32;
        let karmic_patterns: Vec<f32> = entity
            .archetype_activations
            .iter()
            .take(21)
            .map(|&x| x as f32)
            .collect();

        self.cached_free_will_data = Some(FreeWillPanelData {
            archetype_22_activation: archetype_22,
            choice_capacity: archetype_22,
            free_will_strength: archetype_22 * 0.8,
            karmic_patterns,
            choice_moments: 5,
        });

        // Generate Teleological Pull data
        let purpose_alignment = entity.archetype_activations.iter().sum::<f64>() / 22.0;
        let omega_distance = 1.0 - purpose_alignment as f32;

        self.cached_teleological_data = Some(TeleologicalPanelData {
            omega_distance,
            attraction_strength: 1.0 - omega_distance,
            purpose_alignment: purpose_alignment as f32,
            evolutionary_direction: purpose_alignment as f32,
            telos_coordinates: [purpose_alignment as f32, archetype_22, omega_distance],
        });

        // Generate Causal Inversion data
        let inversion_strength = archetype_22 * 0.7;
        let future_influence = archetype_22 * 0.6;

        self.cached_causal_data = Some(CausalPanelData {
            inversion_strength,
            future_influence,
            past_resonance: 0.5 + (archetype_22 * 0.3),
            causal_loops: 3,
            temporal_coherence: 0.75,
        });

        // Generate Density Transition data
        let density_progress: [f32; 8] = {
            let mut arr = [0.0f32; 8];
            for (i, val) in arr.iter_mut().enumerate() {
                *val = 0.3 + (i as f32 * 0.1) + (archetype_22 * 0.05);
            }
            arr
        };

        self.cached_density_data = Some(DensityPanelData {
            current_density: 3,
            density_progress,
            transition_readiness: 0.65,
            density_characteristics: self.generate_density_characteristics(),
        });
    }

    /// Generate density characteristics
    fn generate_density_characteristics(&self) -> Vec<DensityCharacteristic> {
        vec![
            DensityCharacteristic {
                name: "1st Density".to_string(),
                description: "Physical awareness".to_string(),
                activation: 0.9,
            },
            DensityCharacteristic {
                name: "2nd Density".to_string(),
                description: "Biological consciousness".to_string(),
                activation: 0.8,
            },
            DensityCharacteristic {
                name: "3rd Density".to_string(),
                description: "Self-awareness".to_string(),
                activation: 0.7,
            },
            DensityCharacteristic {
                name: "4th Density".to_string(),
                description: "Love/understanding".to_string(),
                activation: 0.5,
            },
            DensityCharacteristic {
                name: "5th Density".to_string(),
                description: "Wisdom/light".to_string(),
                activation: 0.4,
            },
            DensityCharacteristic {
                name: "6th Density".to_string(),
                description: "Unity consciousness".to_string(),
                activation: 0.3,
            },
            DensityCharacteristic {
                name: "7th Density".to_string(),
                description: "Gateway to octave".to_string(),
                activation: 0.2,
            },
            DensityCharacteristic {
                name: "8th Density".to_string(),
                description: "Octave completion".to_string(),
                activation: 0.1,
            },
        ]
    }

    /// Update animations
    pub fn update(&mut self, delta_time: f32) {
        self.animation_time += delta_time;
    }

    /// Show the panel
    pub fn show(&mut self, ctx: &Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("🧠 Consciousness")
            .collapsible(true)
            .resizable(true)
            .default_width(600.0)
            .default_height(700.0)
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
        ui.separator();

        // Render current tab content
        match self.view_mode {
            ConsciousnessViewMode::FreeWill => {
                self.show_free_will_tab(ui);
            }
            ConsciousnessViewMode::TeleologicalPull => {
                self.show_teleological_pull_tab(ui);
            }
            ConsciousnessViewMode::CausalInversion => {
                self.show_causal_inversion_tab(ui);
            }
            ConsciousnessViewMode::DensityTransition => {
                self.show_density_transition_tab(ui);
            }
        }
    }

    /// Show entity selector
    fn show_entity_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Entity:");

            // Build entity list
            if let Some(id) = self.selected_entity_id {
                ui.label(format!("Entity {}", id));
            } else {
                ui.label("No entity selected (using demo data)");
            }
        });
    }

    /// Show tab navigation
    fn show_tab_navigation(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.view_mode,
                ConsciousnessViewMode::FreeWill,
                "⚡ Free Will",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ConsciousnessViewMode::TeleologicalPull,
                "🧲 Teleological",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ConsciousnessViewMode::CausalInversion,
                "🔄 Causal Inv",
            );
            ui.selectable_value(
                &mut self.view_mode,
                ConsciousnessViewMode::DensityTransition,
                "📊 Density",
            );
        });
    }

    /// Show Free Will tab
    fn show_free_will_tab(&mut self, ui: &mut Ui) {
        ui.heading("Free Will Kernel (Archetype 22)");

        // Controls
        if self.show_controls {
            self.show_free_will_controls(ui);
            ui.separator();
        }

        // Render Free Will visualization
        self.render_free_will_visualization(ui);

        // Show Free Will explanation
        ui.add_space(10.0);
        ui.label("Archetype 22 - The Choice Operator:");
        ui.label("• Represents the capacity for genuine choice");
        ui.label("• Modulates karmic pattern resolution");
        ui.label("• Enables evolutionary leaps beyond deterministic progression");
        ui.label("• From COSMOLOGICAL-ARCHITECTURE.md: 'Free Will is the primal distortion that allows choice'");
    }

    /// Show Free Will controls
    fn show_free_will_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Free Will Controls", |ui| {
            ui.checkbox(
                &mut self.free_will_viz.show_timeline,
                "Show Choice Timeline",
            );
            ui.checkbox(
                &mut self.free_will_viz.show_weights,
                "Show Decision Weights",
            );
            ui.checkbox(
                &mut self.free_will_viz.show_capacity,
                "Show Capacity Indicator",
            );
        });
    }

    /// Render Free Will visualization
    fn render_free_will_visualization(&self, ui: &mut Ui) {
        let data = self.cached_free_will_data.as_ref();

        if let Some(free_will) = data {
            // A22 Activation
            ui.label("Archetype 22 Activation:");
            ui.add(
                egui::ProgressBar::new(free_will.archetype_22_activation)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(255, 99, 71)), // Tomato red
            );
            ui.label(format!("{:.3}", free_will.archetype_22_activation));

            ui.separator();

            // Choice Capacity
            ui.horizontal(|ui| {
                ui.label("Choice Capacity:");
                ui.add(
                    egui::ProgressBar::new(free_will.choice_capacity)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(255, 140, 0)), // Dark orange
                );
                ui.label(format!("{:.2}", free_will.choice_capacity));
            });

            ui.horizontal(|ui| {
                ui.label("Free Will Strength:");
                ui.add(
                    egui::ProgressBar::new(free_will.free_will_strength)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(255, 165, 0)), // Orange
                );
                ui.label(format!("{:.2}", free_will.free_will_strength));
            });

            ui.separator();

            // Karmic patterns visualization
            ui.label(egui::RichText::new("Karmic Patterns (A1-A21)").strong());
            self.render_karmic_patterns(ui, free_will);

            ui.separator();

            // Choice moments
            ui.label(format!(
                "Choice Moments Available: {}",
                free_will.choice_moments
            ));
        } else {
            ui.label("No Free Will data available");
        }
    }

    /// Render karmic patterns
    fn render_karmic_patterns(&self, ui: &mut Ui, data: &FreeWillPanelData) {
        let bar_width = 12.0;

        egui::Grid::new("karmic_patterns_grid")
            .num_columns(7)
            .spacing([3.0, 3.0])
            .show(ui, |ui| {
                for (i, &pattern) in data.karmic_patterns.iter().enumerate() {
                    if i > 0 && i % 7 == 0 {
                        ui.end_row();
                    }

                    let color = match i {
                        0..=6 => Color32::from_rgb(100, 149, 237),  // Mind - Blue
                        7..=13 => Color32::from_rgb(34, 139, 34),   // Body - Green
                        14..=20 => Color32::from_rgb(218, 165, 32), // Spirit - Gold
                        _ => Color32::GRAY,
                    };

                    ui.horizontal(|ui| {
                        ui.add(
                            egui::ProgressBar::new(pattern)
                                .desired_width(bar_width)
                                .fill(color),
                        );
                        ui.label(format!("A{}", i + 1));
                    });
                }
            });
    }

    /// Show Teleological Pull tab
    fn show_teleological_pull_tab(&mut self, ui: &mut Ui) {
        ui.heading("Teleological Pull (Omega Point)");

        // Controls
        if self.show_controls {
            self.show_teleological_controls(ui);
            ui.separator();
        }

        // Render Teleological Pull visualization
        self.render_teleological_visualization(ui);

        // Show Teleological explanation
        ui.add_space(10.0);
        ui.label("Omega Point Attraction:");
        ui.label("• The universe evolves toward a final cause (telos)");
        ui.label("• Purpose alignment increases as entity approaches Omega");
        ui.label("• Evolutionary direction emerges from future→past pull");
        ui.label("• From COSMOLOGICAL-ARCHITECTURE.md: 'Teleological pull is the logos returning to source'");
    }

    /// Show Teleological controls
    fn show_teleological_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Teleological Controls", |ui| {
            ui.checkbox(
                &mut self.teleological_pull_viz.show_attractors,
                "Show Attractor Fields",
            );
            ui.checkbox(
                &mut self.teleological_pull_viz.show_trajectory,
                "Show Entity Trajectory",
            );
        });
    }

    /// Render Teleological visualization
    fn render_teleological_visualization(&self, ui: &mut Ui) {
        let data = self.cached_teleological_data.as_ref();

        if let Some(telos) = data {
            // Omega Distance
            ui.label("Distance to Omega Point:");
            ui.add(
                egui::ProgressBar::new(telos.omega_distance)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(147, 112, 219)), // Purple
            );
            ui.label(format!("{:.3}", telos.omega_distance));

            ui.separator();

            // Attraction metrics
            ui.horizontal(|ui| {
                ui.label("Attraction Strength:");
                ui.add(
                    egui::ProgressBar::new(telos.attraction_strength)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(138, 43, 226)), // Blue violet
                );
                ui.label(format!("{:.2}", telos.attraction_strength));
            });

            ui.horizontal(|ui| {
                ui.label("Purpose Alignment:");
                ui.add(
                    egui::ProgressBar::new(telos.purpose_alignment)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(148, 0, 211)), // Dark violet
                );
                ui.label(format!("{:.2}", telos.purpose_alignment));
            });

            ui.horizontal(|ui| {
                ui.label("Evolutionary Direction:");
                ui.add(
                    egui::ProgressBar::new(telos.evolutionary_direction)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(153, 50, 204)), // Dark orchid
                );
                ui.label(format!("{:.2}", telos.evolutionary_direction));
            });

            ui.separator();

            // Telos coordinates
            ui.label(egui::RichText::new("Telos Coordinates").strong());
            ui.horizontal(|ui| {
                ui.label("X (Alignment):");
                ui.label(format!("{:.2}", telos.telos_coordinates[0]));
                ui.label("Y (A22):");
                ui.label(format!("{:.2}", telos.telos_coordinates[1]));
                ui.label("Z (Distance):");
                ui.label(format!("{:.2}", telos.telos_coordinates[2]));
            });
        } else {
            ui.label("No Teleological data available");
        }
    }

    /// Show Causal Inversion tab
    fn show_causal_inversion_tab(&mut self, ui: &mut Ui) {
        ui.heading("Causal Inversion (Future→Past)");

        // Controls
        if self.show_controls {
            self.show_causal_inversion_controls(ui);
            ui.separator();
        }

        // Render Causal Inversion visualization
        self.render_causal_inversion_visualization(ui);

        // Show Causal Inversion explanation
        ui.add_space(10.0);
        ui.label("Temporal Causality Reversal:");
        ui.label("• Future states influence present decisions");
        ui.label("• Omega Point exerts pull through time");
        ui.label("• Causal loops enable learning from future outcomes");
        ui.label("• From COSMOLOGICAL-ARCHITECTURE.md: 'Causal inversion allows the end to shape the beginning'");
    }

    /// Show Causal Inversion controls
    fn show_causal_inversion_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Causal Inversion Controls", |ui| {
            ui.checkbox(
                &mut self.causal_inversion_viz.show_hierarchy,
                "Show Density Hierarchy",
            );
            ui.checkbox(
                &mut self.causal_inversion_viz.show_arrows,
                "Show Influence Arrows",
            );
        });
    }

    /// Render Causal Inversion visualization
    fn render_causal_inversion_visualization(&self, ui: &mut Ui) {
        let data = self.cached_causal_data.as_ref();

        if let Some(causal) = data {
            // Inversion Strength
            ui.label("Causal Inversion Strength:");
            ui.add(
                egui::ProgressBar::new(causal.inversion_strength)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(0, 206, 209)), // Dark turquoise
            );
            ui.label(format!("{:.3}", causal.inversion_strength));

            ui.separator();

            // Temporal metrics
            ui.horizontal(|ui| {
                ui.label("Future Influence:");
                ui.add(
                    egui::ProgressBar::new(causal.future_influence)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(32, 178, 170)), // Light sea green
                );
                ui.label(format!("{:.2}", causal.future_influence));
            });

            ui.horizontal(|ui| {
                ui.label("Past Resonance:");
                ui.add(
                    egui::ProgressBar::new(causal.past_resonance)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(46, 139, 87)), // Sea green
                );
                ui.label(format!("{:.2}", causal.past_resonance));
            });

            ui.horizontal(|ui| {
                ui.label("Temporal Coherence:");
                ui.add(
                    egui::ProgressBar::new(causal.temporal_coherence)
                        .desired_width(150.0)
                        .fill(Color32::from_rgb(60, 179, 113)), // Medium sea green
                );
                ui.label(format!("{:.2}", causal.temporal_coherence));
            });

            ui.separator();

            // Causal loops
            ui.label(format!("Active Causal Loops: {}", causal.causal_loops));

            // Causal loop visualization
            ui.label(egui::RichText::new("Causal Loop Diagram").strong());
            self.render_causal_loops(ui, causal);
        } else {
            ui.label("No Causal Inversion data available");
        }
    }

    /// Render causal loops
    fn render_causal_loops(&self, ui: &mut Ui, _data: &CausalPanelData) {
        // Simple visualization of causal loop structure
        let (response, painter) =
            ui.allocate_painter(egui::vec2(200.0, 100.0), egui::Sense::hover());

        let center = response.rect.center();
        let radius = 40.0;

        // Draw loop nodes
        for i in 0..3 {
            let angle = (i as f32) * std::f32::consts::TAU / 3.0;
            let x = center.x + angle.cos() * radius;
            let y = center.y + angle.sin() * radius;
            let pos = egui::pos2(x, y);

            painter.circle_filled(pos, 8.0, Color32::from_rgb(0, 206, 209));

            // Draw arrows between nodes
            let next_angle = ((i + 1) as f32) * std::f32::consts::TAU / 3.0;
            let next_x = center.x + next_angle.cos() * radius;
            let next_y = center.y + next_angle.sin() * radius;

            painter.line_segment(
                [pos, egui::pos2(next_x, next_y)],
                (2.0, Color32::from_rgb(0, 206, 209)),
            );
        }

        // Center point (present)
        painter.circle_filled(center, 10.0, Color32::from_rgb(255, 99, 71));
    }

    /// Show Density Transition tab
    fn show_density_transition_tab(&mut self, ui: &mut Ui) {
        ui.heading("Density Transition (1st→8th)");

        // Controls
        if self.show_controls {
            self.show_density_transition_controls(ui);
            ui.separator();
        }

        // Render Density Transition visualization
        self.render_density_transition_visualization(ui);

        // Show Density Transition explanation
        ui.add_space(10.0);
        ui.label("Density Octave Progression:");
        ui.label("• 1st Density: Physical awareness (mineral)");
        ui.label("• 2nd Density: Biological consciousness (plant/animal)");
        ui.label("• 3rd Density: Self-awareness (human current)");
        ui.label("• 4th Density: Love/understanding");
        ui.label("• 5th Density: Wisdom/light");
        ui.label("• 6th Density: Unity consciousness");
        ui.label("• 7th Density: Gateway to octave");
        ui.label("• 8th Density: Octave completion (return to 1st at higher spiral)");
    }

    /// Show Density Transition controls
    fn show_density_transition_controls(&mut self, ui: &mut Ui) {
        ui.collapsing("Density Transition Controls", |ui| {
            ui.checkbox(
                &mut self.density_transition_viz.show_animation,
                "Show Progression Animation",
            );
            ui.checkbox(
                &mut self.density_transition_viz.show_thresholds,
                "Show Polarity Thresholds",
            );
            ui.checkbox(
                &mut self.density_transition_viz.show_harvest,
                "Show Harvest Indicator",
            );
            ui.checkbox(
                &mut self.density_transition_viz.show_light_body,
                "Show Light Body Activation",
            );
        });
    }

    /// Render Density Transition visualization
    fn render_density_transition_visualization(&self, ui: &mut Ui) {
        let data = self.cached_density_data.as_ref();

        if let Some(density) = data {
            // Current Density
            ui.label(
                egui::RichText::new(format!("Current Density: {}rd", density.current_density))
                    .strong(),
            );

            ui.separator();

            // Transition Readiness
            ui.label("Transition Readiness:");
            ui.add(
                egui::ProgressBar::new(density.transition_readiness)
                    .desired_width(200.0)
                    .fill(Color32::from_rgb(255, 215, 0)), // Gold
            );
            ui.label(format!("{:.3}", density.transition_readiness));

            ui.separator();

            // Density progress bars
            ui.label(egui::RichText::new("Density Progress (1st→8th)").strong());
            self.render_density_progress(ui, density);

            ui.separator();

            // Density characteristics
            ui.label(egui::RichText::new("Density Characteristics").strong());
            self.render_density_characteristics(ui, density);
        } else {
            ui.label("No Density Transition data available");
        }
    }

    /// Render density progress
    fn render_density_progress(&self, ui: &mut Ui, data: &DensityPanelData) {
        let density_names = ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th"];

        let density_colors = [
            Color32::from_rgb(139, 69, 19),   // Brown - 1st
            Color32::from_rgb(34, 139, 34),   // Green - 2nd
            Color32::from_rgb(255, 215, 0),   // Gold - 3rd
            Color32::from_rgb(255, 105, 180), // Pink - 4th
            Color32::from_rgb(100, 149, 237), // Blue - 5th
            Color32::from_rgb(147, 112, 219), // Purple - 6th
            Color32::from_rgb(255, 255, 255), // White - 7th
            Color32::from_rgb(255, 223, 186), // Peach - 8th
        ];

        for (i, &progress) in data.density_progress.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{} Density:", density_names[i]));
                ui.add(
                    egui::ProgressBar::new(progress.min(1.0))
                        .desired_width(150.0)
                        .fill(density_colors[i]),
                );
                ui.label(format!("{:.1}%", progress.min(1.0) * 100.0));
            });
        }
    }

    /// Render density characteristics
    fn render_density_characteristics(&self, ui: &mut Ui, data: &DensityPanelData) {
        egui::Grid::new("density_characteristics_grid")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                for characteristic in &data.density_characteristics {
                    let color = match characteristic.activation {
                        x if x > 0.7 => Color32::from_rgb(50, 205, 50), // High - Green
                        x if x > 0.4 => Color32::from_rgb(255, 165, 0), // Medium - Orange
                        _ => Color32::from_rgb(200, 200, 200),          // Low - Gray
                    };

                    let rect_min = ui.cursor().min;
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(rect_min, egui::vec2(12.0, 12.0)),
                        2.0,
                        color,
                    );

                    ui.label(&characteristic.name);
                    ui.label(&characteristic.description);
                    ui.end_row();
                }
            });
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

impl Default for ConsciousnessPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_panel_creation() {
        let panel = ConsciousnessPanel::new();
        assert!(!panel.visible);
        assert_eq!(panel.view_mode, ConsciousnessViewMode::FreeWill);
    }

    #[test]
    fn test_view_mode_switching() {
        let mut panel = ConsciousnessPanel::new();
        panel.view_mode = ConsciousnessViewMode::TeleologicalPull;
        assert_eq!(panel.view_mode, ConsciousnessViewMode::TeleologicalPull);

        panel.view_mode = ConsciousnessViewMode::CausalInversion;
        assert_eq!(panel.view_mode, ConsciousnessViewMode::CausalInversion);

        panel.view_mode = ConsciousnessViewMode::DensityTransition;
        assert_eq!(panel.view_mode, ConsciousnessViewMode::DensityTransition);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut panel = ConsciousnessPanel::new();
        assert!(!panel.is_visible());

        panel.toggle_visibility();
        assert!(panel.is_visible());

        panel.set_visible(false);
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_cached_data_generation() {
        let mut panel = ConsciousnessPanel::new();

        // Create a mock entity using the correct constructor
        // Use proper constructors for each realm type
        let violet = crate::foundation::violet_realm::VioletRealm::new();
        let indigo = crate::foundation::indigo_realm::IntelligentInfinity::from_violet(violet.clone());
        let blue = crate::foundation::blue_realm::Logos::from_intelligent_infinity(indigo.clone());
        let mut green = crate::foundation::green_realm::LightLoveField::from_logos(blue.clone());

        // Add spectrum conditions to green field
        green.add_holographic_pattern(crate::foundation::green_realm::HolographicPattern::new(
            1.0,
            [1.0, 0.0, 0.0],
            1.0,
        ));
        green.add_rhythm(crate::foundation::green_realm::Rhythm::new(0.5, 1.0, 0.0));
        green.add_field(crate::foundation::green_realm::Field::new(1.0, 1.0, "test"));

        let yellow = crate::spectrum::yellow_realm::YellowRealm::new(green.clone());
        let orange = crate::spectrum::orange_realm::OrangeRealm::new(yellow.clone());
        let red = crate::spectrum::red_realm::RedRealm::new(orange.clone());

        let ratio =
            crate::spectrum::SpectrumRatio::new(1.5, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_config =
            crate::entity_layer7::layer7::IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            crate::entity_layer7::layer7::EntityId::new("test-entity-1".to_string()),
            crate::entity_layer7::layer7::EntityType::Individual,
            None,
            vec![],
            None,
            violet,
            indigo,
            blue,
            green,
            yellow,
            orange,
            red,
            spectrum_config,
        );

        panel.update_from_entity(&entity);

        // Check that cached data was generated
        assert!(panel.cached_free_will_data.is_some());
        assert!(panel.cached_teleological_data.is_some());
        assert!(panel.cached_causal_data.is_some());
        assert!(panel.cached_density_data.is_some());
    }

    #[test]
    fn test_all_view_modes_render() {
        let mut panel = ConsciousnessPanel::new();
        panel.visible = true;

        let modes = [
            ConsciousnessViewMode::FreeWill,
            ConsciousnessViewMode::TeleologicalPull,
            ConsciousnessViewMode::CausalInversion,
            ConsciousnessViewMode::DensityTransition,
        ];

        for mode in modes {
            panel.view_mode = mode;
            assert_eq!(panel.view_mode, mode);
        }
    }
}
