//! EntityDetailPanel — Shows ObservableProperties for a selected entity.
//!
//! Unlike EntityInspector (which shows raw SubSubLogos data), this panel
//! shows the game-facing mapped properties: color, shape, behavior, etc.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The 22 archetypes form the orthogonal
//! basis vectors from which all behavior emerges."

use crate::gui::observable_properties::*;
use egui::Context;

/// Archetype names for display (22 archetypes of the Tarot system)
const ARCHETYPE_NAMES: [&str; 22] = [
    "0: The Fool",
    "1: The Magician",
    "2: The High Priestess",
    "3: The Empress",
    "4: The Emperor",
    "5: The Hierophant",
    "6: The Lovers",
    "7: The Chariot",
    "8: Strength",
    "9: The Hermit",
    "10: Wheel of Fortune",
    "11: Justice",
    "12: The Hanged Man",
    "13: Death",
    "14: Temperance",
    "15: The Devil",
    "16: The Tower",
    "17: The Star",
    "18: The Moon",
    "19: The Sun",
    "20: Judgement",
    "21: The World",
];

pub struct EntityDetailPanel {
    pub show_panel: bool,
    pub selected_entity_index: Option<usize>,
    pub collapse_archetypes: bool,
    pub selected_entity_activations: Option<[f64; 22]>,
}

impl Default for EntityDetailPanel {
    fn default() -> Self {
        Self {
            show_panel: true,
            selected_entity_index: None,
            collapse_archetypes: true,
            selected_entity_activations: None,
        }
    }
}

impl EntityDetailPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, index: usize, activations: Option<[f64; 22]>) {
        self.selected_entity_index = Some(index);
        self.selected_entity_activations = activations;
    }

    pub fn deselect(&mut self) {
        self.selected_entity_index = None;
    }

    /// Show the panel. Pass the ObservableProperties for the selected entity.
    pub fn show(&mut self, ctx: &Context, entity: Option<&ObservableProperties>) {
        if !self.show_panel {
            return;
        }

        egui::Window::new("Entity Detail")
            .collapsible(true)
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| {
                if let Some(entity) = entity {
                    self.show_entity_detail(ui, entity);
                } else {
                    ui.label("No entity selected");
                    ui.label("Click on an entity to view details");
                }
            });
    }

    fn show_entity_detail(&mut self, ui: &mut egui::Ui, entity: &ObservableProperties) {
        ui.heading(format!(
            "Entity #{}",
            self.selected_entity_index.unwrap_or(0)
        ));

        ui.separator();
        ui.label(format!(
            "Density: {} ({})",
            entity.density,
            Self::density_name(entity.density)
        ));
        ui.label(format!("Shape: {:?}", entity.shape));
        ui.label(format!("Behavior: {:?}", entity.behavior_state));
        ui.label(format!(
            "Active: {}",
            if entity.is_active { "Yes" } else { "No" }
        ));

        ui.separator();
        ui.label("Consciousness:");
        ui.add(egui::ProgressBar::new(entity.intelligence).show_percentage());

        let sto_pct = ((entity.service_orientation + 1.0) / 2.0 * 100.0).clamp(0.0, 100.0);
        ui.label(format!(
            "Service: {:.0}% STO / {:.0}% STS",
            sto_pct,
            100.0 - sto_pct
        ));
        ui.add(egui::ProgressBar::new(sto_pct / 100.0).text("STO"));

        ui.separator();
        ui.label(format!(
            "Position: [{:.1}, {:.1}]",
            entity.position[0], entity.position[1]
        ));
        ui.label(format!(
            "Velocity: [{:.1}, {:.1}]",
            entity.velocity[0], entity.velocity[1]
        ));
        ui.label(format!("Size: {:.1}", entity.size));
        ui.label(format!("Mass: {:.2}", entity.mass));
        ui.label(format!("Glow: {:.0}%", entity.glow * 100.0));

        ui.separator();
        ui.collapsing("Archetype Activations", |ui| {
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    if let Some(ref acts) = self.selected_entity_activations {
                        for (i, name) in ARCHETYPE_NAMES.iter().enumerate() {
                            let activation = acts[i].clamp(0.0, 1.0) as f32;
                            let color = match i {
                                0..=6 => egui::Color32::from_rgb(200, 80, 80),
                                7..=13 => egui::Color32::from_rgb(80, 180, 80),
                                14..=20 => egui::Color32::from_rgb(80, 80, 200),
                                _ => egui::Color32::from_rgb(160, 80, 160),
                            };
                            ui.horizontal(|ui| {
                                ui.add_space(4.0);
                                ui.add(
                                    egui::ProgressBar::new(activation)
                                        .fill(color)
                                        .desired_height(12.0),
                                );
                                ui.label(format!("{:.0}% ", activation * 100.0));
                                ui.label(*name);
                            });
                        }
                    } else {
                        ui.label("Activation data not available");
                    }
                });
        });

        ui.separator();
        let color_rect = egui::Color32::from_rgb(
            (entity.color[0] * 255.0) as u8,
            (entity.color[1] * 255.0) as u8,
            (entity.color[2] * 255.0) as u8,
        );
        ui.horizontal(|ui| {
            ui.label("Entity Color:");
            ui.colored_label(color_rect, "\u{25A0}\u{25A0}\u{25A0}");
        });

        ui.separator();
        if ui.button("Close").clicked() {
            self.deselect();
        }
    }

    fn density_name(density: u8) -> &'static str {
        match density {
            1 => "Elemental",
            2 => "Biological",
            3 => "Self-Aware",
            4 => "Loving",
            5 => "Wise",
            6 => "Unified",
            7 => "Transcendent",
            8 => "Source Return",
            _ => "Unknown",
        }
    }
}
