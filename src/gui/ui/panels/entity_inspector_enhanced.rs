//! Entity Inspector (Enhanced) - Complete Cosmological Data Display
//!
//! Phase 6: Interactive Inspection
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md Phase 6:
//! "Entity inspector panel (UI overlay) - display detailed entity information"
//!
//! This enhanced inspector displays:
//! - Complete 7-layer holonic architecture data
//! - 22 archetype activations with names and visual display
//! - Realm intensities with color-coded visualization
//! - Involution and evolutionary stages
//! - Consciousness, polarization, and spectrum data
//! - Energy and vibrational states
//! - Composition and hierarchy relationships

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::evolution_density_octave::density_octave::Density;
use egui::{Color32, Context, RichText};

/// Archetype names (22 archetypes from Law of One)
const ARCHETYPE_NAMES: [&str; 22] = [
    "1. The Magician (I) - Matrix of Mind",
    "2. The High Priestess (II) - Potentiator of Mind",
    "3. The Empress (III) - Catalyst of Mind",
    "4. The Emperor (IV) - Experience of Mind",
    "5. The Hierophant (V) - Significator of Mind",
    "6. The Lovers (VI) - Transformation of Mind",
    "7. The Chariot (VII) - Great Way of Mind",
    "8. Strength (VIII) - Matrix of Body",
    "9. The Hermit (IX) - Potentiator of Body",
    "10. Wheel of Fortune (X) - Catalyst of Body",
    "11. Justice (XI) - Experience of Body",
    "12. The Hanged Man (XII) - Significator of Body",
    "13. Death (XIII) - Transformation of Body",
    "14. Temperance (XIV) - Great Way of Body",
    "15. The Devil (XV) - Matrix of Spirit",
    "16. The Tower (XVI) - Potentiator of Spirit",
    "17. The Star (XVII) - Catalyst of Spirit",
    "18. The Moon (XVIII) - Experience of Spirit",
    "19. The Sun (XIX) - Significator of Spirit",
    "20. Judgement (XX) - Transformation of Spirit",
    "21. The World (XXI) - Great Way of Spirit",
    "22. The Choice (XXII) - Unifying Archetype",
];

/// Realm names with colors
const REALM_DATA: [(&str, Color32); 7] = [
    ("Violet - Infinity", Color32::from_rgb(148, 0, 211)),
    (
        "Indigo - Intelligent Infinity",
        Color32::from_rgb(75, 0, 130),
    ),
    ("Blue - Love/Logos", Color32::from_rgb(0, 0, 255)),
    ("Green - Light/Love", Color32::from_rgb(0, 128, 0)),
    (
        "Yellow - Dimensional Architecture",
        Color32::from_rgb(255, 255, 0),
    ),
    ("Orange - Galactic Spectrum", Color32::from_rgb(255, 165, 0)),
    ("Red - Solar Archetype", Color32::from_rgb(255, 0, 0)),
];

#[derive(Debug, Clone)]
pub struct EntityInspectorEnhanced {
    pub selected_entity_id: Option<EntityId>,
    pub show_inspector: bool,
    pub show_archetypes: bool,
    pub show_realms: bool,
    pub show_energy: bool,
}

impl EntityInspectorEnhanced {
    pub fn new() -> Self {
        EntityInspectorEnhanced {
            selected_entity_id: None,
            show_inspector: true,
            show_archetypes: true,
            show_realms: true,
            show_energy: true,
        }
    }

    pub fn select_entity(&mut self, entity_id: EntityId) {
        self.selected_entity_id = Some(entity_id);
    }

    pub fn deselect(&mut self) {
        self.selected_entity_id = None;
    }

    pub fn show(&mut self, ctx: &Context, entity: Option<&SubSubLogos>) {
        if !self.show_inspector {
            return;
        }

        egui::Window::new("Entity Inspector (Enhanced)")
            .collapsible(true)
            .resizable(true)
            .default_width(400.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                if let Some(entity) = entity {
                    self.show_entity_info(ui, entity);
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(RichText::new("No entity selected").size(16.0));
                        ui.add_space(10.0);
                        ui.label("Click on an entity to inspect it");
                        ui.label("Press 'I' to toggle this panel");
                        ui.add_space(20.0);
                        ui.label("Keyboard Shortcuts:");
                        ui.label("  1-6: Switch views");
                        ui.label("  F: Focus on selected entity");
                        ui.label("  ESC: Deselect / Exit focus");
                    });
                }
            });
    }

    fn show_entity_info(&mut self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new(format!("Entity {}", entity.entity_id)).size(18.0));
            ui.label(
                RichText::new(entity.entity_type.display_name().to_string())
                    .color(Color32::LIGHT_BLUE),
            );
        });
        ui.separator();

        // Scrollable content
        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                // Section 1: Basic Information
                self.show_basic_info(ui, entity);

                // Section 2: Consciousness & Polarization
                self.show_consciousness(ui, entity);

                // Section 3: Spectrum
                self.show_spectrum(ui, entity);

                // Section 4: Realm Intensities (7-layer architecture)
                if self.show_realms {
                    self.show_realms_section(ui, entity);
                }

                // Section 5: 22 Archetypes
                if self.show_archetypes {
                    self.show_archetypes_section(ui, entity);
                }

                // Section 6: Energy & Vibrational State
                if self.show_energy {
                    self.show_energy_section(ui, entity);
                }

                // Section 7: Evolution & Density
                self.show_evolution_section(ui, entity);

                // Section 8: Composition & Hierarchy
                self.show_hierarchy_section(ui, entity);
            });
    }

    fn show_basic_info(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("📋 Basic Information", |ui| {
            ui.label(format!("Type: {}", entity.entity_type.display_name()));
            ui.label(format!("ID: {}", entity.entity_id));

            // Position info - simplified
            ui.label("Position: Available in EntityRenderer (world space)");
        });
        ui.add_space(5.0);
    }

    fn show_consciousness(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("🧠 Consciousness & Polarization", |ui| {
            // Consciousness level
            ui.label("Consciousness Level:");
            ui.add(
                egui::ProgressBar::new((entity.consciousness_level as f32).min(1.0))
                    .show_percentage(),
            );
            ui.label(format!("  {:.4}", entity.consciousness_level));

            ui.add_space(5.0);

            // Experience
            ui.label("Experience Accumulation:");
            ui.add(egui::ProgressBar::new(
                ((entity.experience_accumulation % 100.0) as f32) / 100.0,
            ));
            ui.label(format!("  {:.2}", entity.experience_accumulation));

            ui.add_space(5.0);

            // Learning
            ui.label("Learning Progress:");
            ui.add(egui::ProgressBar::new(entity.learning_progress as f32));
            ui.label(format!("  {:.4}", entity.learning_progress));

            ui.add_space(5.0);

            // Polarity
            let polarity_bias = entity.polarization.polarity_bias();
            let polarity_text = if polarity_bias > 0.1 {
                format!("STO (+{:.4})", polarity_bias)
            } else if polarity_bias < -0.1 {
                format!("STS ({:.4})", polarity_bias)
            } else {
                "Neutral".to_string()
            };

            ui.label(format!("Polarity Orientation: {}", polarity_text));

            ui.label("Polarization Strength:");
            ui.add(egui::ProgressBar::new(
                (entity.polarization.intensity as f32).min(1.0),
            ));
            ui.label(format!("  {:.4}", entity.polarization.intensity));
        });
        ui.add_space(5.0);
    }

    fn show_spectrum(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("🌈 Spectrum Configuration", |ui| {
            ui.label(format!(
                "Spectrum Position: {:.4}",
                entity.spectrum_position()
            ));
            ui.label(format!("Space/Time Ratio: {:.4}", entity.space_time_ratio));
            ui.label(format!("Time/Space Ratio: {:.4}", entity.time_space_ratio));
            ui.label(format!(
                "Veil Active: {}",
                entity.spectrum_access.veil_active
            ));
        });
        ui.add_space(5.0);
    }

    fn show_realms_section(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("🔮 7-Layer Holonic Architecture", |ui| {
            ui.label("Realm Intensities (Violet → Red):");

            // Extract realm intensities from entity realm objects
            let realm_intensities = [
                (entity.violet_realm.rhythmic_flow * entity.violet_realm.mystery) as f32,
                entity.indigo_realm.awareness as f32,
                entity.blue_realm.focusing_strength as f32,
                entity.green_realm.potential_strength as f32,
                entity.yellow_realm.attractor_field.strength as f32,
                (entity
                    .yellow_realm
                    .dimensional_architecture
                    .dimensions
                    .len() as f32
                    / 10.0)
                    .min(1.0),
                (entity.archetype_activations.iter().sum::<f64>() / 22.0) as f32,
            ];

            for (i, (name, color)) in REALM_DATA.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("{}:", name.split(" - ").next().unwrap())).size(11.0),
                    );
                    ui.add_sized(
                        [150.0, 15.0],
                        egui::ProgressBar::new(realm_intensities[i].min(1.0))
                            .fill(color.gamma_multiply(0.7)),
                    );
                    ui.label(format!("{:.3}", realm_intensities[i]));
                });
            }
        });
        ui.add_space(5.0);
    }

    fn show_archetypes_section(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("⭐ 22 Archetypes (Archetypical Mind)", |ui| {
            let active_count = entity
                .archetype_activations
                .iter()
                .filter(|&&v| v > 0.1)
                .count();

            ui.label(format!("Active Archetypes: {}/22", active_count));

            ui.add_space(5.0);

            // Show all 22 archetypes sorted by activation
            let mut archetype_list: Vec<(usize, f64, &str)> = entity
                .archetype_activations
                .iter()
                .enumerate()
                .map(|(i, &v)| (i, v, ARCHETYPE_NAMES[i]))
                .collect();

            // Sort by activation (descending)
            archetype_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            for (archetype_idx, activation, name) in archetype_list {
                if activation > 0.01 {
                    // Only show significant activations
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("#{}", archetype_idx + 1)).size(10.0));
                        ui.add_sized(
                            [200.0, 12.0],
                            egui::ProgressBar::new(activation as f32)
                                .fill(Color32::from_rgb(255, 215, 0)),
                        );
                        ui.label(RichText::new(format!("{:.3}", activation)).size(10.0));
                    });
                    ui.label(RichText::new(name).size(10.0).italics());
                    ui.add_space(3.0);
                }
            }
        });
        ui.add_space(5.0);
    }

    fn show_energy_section(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("⚡ Energy & Vibrational State", |ui| {
            ui.label("Potential Energy:");
            ui.add(egui::ProgressBar::new(
                ((entity.potential_energy % 100.0) as f32) / 100.0,
            ));
            ui.label(format!("  {:.2}", entity.potential_energy));

            ui.add_space(5.0);

            ui.label("Kinetic Energy:");
            ui.add(egui::ProgressBar::new(
                ((entity.kinetic_energy % 100.0) as f32) / 100.0,
            ));
            ui.label(format!("  {:.2}", entity.kinetic_energy));

            ui.add_space(5.0);

            ui.label(format!(
                "Total Energy: {:.2}",
                entity.potential_energy + entity.kinetic_energy
            ));

            ui.add_space(10.0);

            // Vibrational state
            ui.label("Vibrational Frequency:");
            ui.add(egui::ProgressBar::new(
                ((entity.current_state.vibrational_state.frequency % 10.0) as f32) / 10.0,
            ));
            ui.label(format!(
                "  {:.2} Hz",
                entity.current_state.vibrational_state.frequency
            ));

            ui.add_space(5.0);

            ui.label("Vibrational Amplitude:");
            ui.add(egui::ProgressBar::new(
                entity.current_state.vibrational_state.amplitude as f32,
            ));
            ui.label(format!(
                "  {:.2}",
                entity.current_state.vibrational_state.amplitude
            ));

            ui.add_space(5.0);

            ui.label("Vibrational Coherence:");
            ui.add(egui::ProgressBar::new(
                entity.current_state.vibrational_state.coherence as f32,
            ));
            ui.label(format!(
                "  {:.2}",
                entity.current_state.vibrational_state.coherence
            ));
        });
        ui.add_space(5.0);
    }

    fn show_evolution_section(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("🔄 Evolution & Density", |ui| {
            ui.label(format!("Evolution Clock: {:.2}", entity.evolution_clock));
            ui.label(format!(
                "Evolutionary Rate: {:.2}x",
                entity.evolutionary_rate
            ));

            ui.add_space(5.0);

            ui.label(format!(
                "Current Density: {}",
                format_density(entity.current_density)
            ));

            ui.add_space(5.0);

            ui.label("Evolution Progress:");
            let progress = (entity.evolution_clock % 100.0) / 100.0;
            ui.add(egui::ProgressBar::new(progress as f32));
            ui.label(format!("  {:.4}", progress));

            ui.add_space(5.0);

            ui.label(format!("Karmic Patterns: {}", entity.karmic_patterns.len()));
        });
        ui.add_space(5.0);
    }

    fn show_hierarchy_section(&self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.collapsing("🌳 Composition & Hierarchy", |ui| {
            ui.label(format!("Components: {}", entity.composition.len()));
            ui.label(format!("Children: {}", entity.children.len()));

            ui.add_space(5.0);

            if let Some(parent_id) = &entity.parent_id {
                ui.label(format!("Parent: {}", parent_id));
            } else {
                ui.label("Parent: None (Root entity)");
            }

            ui.add_space(5.0);

            if let Some(env_id) = &entity.environment_id {
                ui.label(format!("Environment: {}", env_id));
            } else {
                ui.label("Environment: None (Direct environment)");
            }
        });
    }
}

impl Default for EntityInspectorEnhanced {
    fn default() -> Self {
        Self::new()
    }
}

fn format_density(density: Density) -> String {
    match density {
        Density::First(sublevel) => format!("1st Density ({:?})", sublevel),
        Density::Second(sublevel) => format!("2nd Density ({:?})", sublevel),
        Density::Third => "3rd Density".to_string(),
        Density::Fourth => "4th Density".to_string(),
        Density::Fifth => "5th Density".to_string(),
        Density::Sixth => "6th Density".to_string(),
        Density::Seventh => "7th Density".to_string(),
        Density::Eighth => "8th Density".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_names() {
        assert_eq!(ARCHETYPE_NAMES.len(), 22);
        assert_eq!(ARCHETYPE_NAMES[0], "1. The Magician (I) - Matrix of Mind");
        assert_eq!(
            ARCHETYPE_NAMES[21],
            "22. The Choice (XXII) - Unifying Archetype"
        );
    }

    #[test]
    fn test_realm_data() {
        assert_eq!(REALM_DATA.len(), 7);
        assert_eq!(REALM_DATA[0].0, "Violet - Infinity");
        assert_eq!(REALM_DATA[6].0, "Red - Solar Archetype");
    }

    #[test]
    fn test_inspector_creation() {
        let inspector = EntityInspectorEnhanced::new();
        assert!(inspector.selected_entity_id.is_none());
        assert!(inspector.show_inspector);
        assert!(inspector.show_archetypes);
        assert!(inspector.show_realms);
        assert!(inspector.show_energy);
    }

    #[test]
    fn test_inspector_default() {
        let inspector = EntityInspectorEnhanced::default();
        assert!(inspector.show_inspector);
    }
}
