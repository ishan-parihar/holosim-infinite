use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use egui::Context;

#[derive(Debug, Clone)]
pub struct EntityInspector {
    pub selected_entity_id: Option<EntityId>,
    pub show_inspector: bool,
}

impl EntityInspector {
    pub fn new() -> Self {
        EntityInspector {
            selected_entity_id: None,
            show_inspector: true,
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

        egui::Window::new("Entity Inspector")
            .collapsible(true)
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                if let Some(entity) = entity {
                    self.show_entity_info(ui, entity);
                } else {
                    ui.label("No entity selected");
                    ui.label("Click on an entity to inspect it");
                }
            });
    }

    fn show_entity_info(&mut self, ui: &mut egui::Ui, entity: &SubSubLogos) {
        ui.heading(format!("Entity {}", entity.entity_id));

        ui.separator();

        ui.label(format!("Type: {}", entity.entity_type.display_name()));

        ui.separator();
        ui.label("Density:");
        ui.label(format!(
            "  Current: {}",
            format_density(entity.current_density)
        ));

        ui.separator();
        ui.label("Consciousness:");
        ui.label(format!("  Level: {:.2}", entity.consciousness_level));
        ui.label(format!(
            "  Experience: {:.2}",
            entity.experience_accumulation
        ));
        ui.label(format!("  Learning: {:.2}", entity.learning_progress));

        ui.separator();
        ui.label("Polarity:");
        let polarity_bias = entity.current_state.polarity_state.polarity_bias;
        let polarity_text = if polarity_bias > 0.1 {
            format!("STO (+{:.2})", polarity_bias)
        } else if polarity_bias < -0.1 {
            format!("STS ({:.2})", polarity_bias)
        } else {
            "Neutral".to_string()
        };
        ui.label(format!("  Orientation: {}", polarity_text));
        ui.label(format!(
            "  Strength: {:.2}",
            entity.current_state.polarity_state.polarization_strength
        ));

        ui.separator();
        ui.label("Spectrum:");
        ui.label(format!("  Position: {:.2}", entity.spectrum_position));
        ui.label(format!("  Space/Time: {:.2}", entity.space_time_ratio));
        ui.label(format!("  Time/Space: {:.2}", entity.time_space_ratio));
        ui.label(format!(
            "  Veil Transparency: {:.2}",
            entity.veil_transparency
        ));

        ui.separator();
        ui.label("Energy:");
        ui.label(format!("  Potential: {:.2}", entity.potential_energy));
        ui.label(format!("  Kinetic: {:.2}", entity.kinetic_energy));
        ui.label(format!("  Total: {:.2}", entity.energy));

        ui.separator();
        ui.label("Vibrational State:");
        ui.label(format!(
            "  Frequency: {:.2}",
            entity.current_state.vibrational_state.frequency
        ));
        ui.label(format!(
            "  Amplitude: {:.2}",
            entity.current_state.vibrational_state.amplitude
        ));
        ui.label(format!(
            "  Coherence: {:.2}",
            entity.current_state.vibrational_state.coherence
        ));

        ui.separator();
        ui.label("Evolution:");
        ui.label(format!("  Clock: {:.2}", entity.evolution_clock));
        ui.label(format!("  Rate: {:.2}x", entity.evolutionary_rate));
        ui.label(format!(
            "  Karmic Patterns: {}",
            entity.karmic_patterns.len()
        ));

        ui.separator();
        ui.label("Composition:");
        ui.label(format!("  Components: {}", entity.composition.len()));
        ui.label(format!("  Children: {}", entity.children.len()));
        if let Some(parent_id) = &entity.parent_id {
            ui.label(format!("  Parent: {}", parent_id));
        } else {
            ui.label("  Parent: None");
        }
        if let Some(env_id) = &entity.environment_id {
            ui.label(format!("  Environment: {}", env_id));
        } else {
            ui.label("  Environment: None");
        }

        ui.separator();
        ui.label("Archetypes:");
        let active_archetypes: Vec<(usize, &f64)> = entity
            .archetype_activations
            .iter()
            .enumerate()
            .filter(|(_, &val)| val > 0.1)
            .collect();
        if active_archetypes.is_empty() {
            ui.label("  None active");
        } else {
            for (archetype, level) in active_archetypes {
                ui.label(format!("  #{}: {:.2}", archetype + 1, level));
            }
        }
    }
}

impl Default for EntityInspector {
    fn default() -> Self {
        Self::new()
    }
}

fn format_density(density: crate::evolution_density_octave::density_octave::Density) -> String {
    match density {
        crate::evolution_density_octave::density_octave::Density::First(sublevel) => {
            format!("1st ({:?})", sublevel)
        }
        crate::evolution_density_octave::density_octave::Density::Second(sublevel) => {
            format!("2nd ({:?})", sublevel)
        }
        crate::evolution_density_octave::density_octave::Density::Third => "3rd".to_string(),
        crate::evolution_density_octave::density_octave::Density::Fourth => "4th".to_string(),
        crate::evolution_density_octave::density_octave::Density::Fifth => "5th".to_string(),
        crate::evolution_density_octave::density_octave::Density::Sixth => "6th".to_string(),
        crate::evolution_density_octave::density_octave::Density::Seventh => "7th".to_string(),
        crate::evolution_density_octave::density_octave::Density::Eighth => "8th".to_string(),
    }
}
