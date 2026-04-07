use crate::gui::simulation_adapter::SimulationRunnerAdapter;
use crate::simulation_v3::observation_layer::TerrainType;

#[derive(Debug, Clone)]
pub struct ObservationPanel {
    pub show_panel: bool,
}

impl Default for ObservationPanel {
    fn default() -> Self {
        Self { show_panel: true }
    }
}

impl ObservationPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.show_panel = !self.show_panel;
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        adapter: &SimulationRunnerAdapter,
        selected_entity_uuid: Option<&str>,
    ) {
        if !self.show_panel {
            return;
        }

        egui::Window::new("Observation Layer")
            .collapsible(true)
            .resizable(true)
            .default_width(380.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                let Some(uuid) = selected_entity_uuid else {
                    ui.label("Select an entity to view observations");
                    return;
                };

                let Some(obs) = adapter.get_entity_observation(uuid) else {
                    ui.label("No observation data for selected entity");
                    return;
                };

                self.show_physical(ui, &obs.physical);
                ui.separator();
                self.show_behavioral(ui, &obs.behavioral);
                ui.separator();
                self.show_environmental(ui, &obs.environment);
                ui.separator();
                self.show_events(ui, adapter);
            });
    }

    fn show_physical(
        &mut self,
        ui: &mut egui::Ui,
        physical: &crate::simulation_v3::observation_layer::PhysicalObservation,
    ) {
        ui.heading("Physical");

        ui.horizontal(|ui| {
            ui.label("Position:");
            ui.label(format!(
                "({:.2}, {:.2}, {:.2})",
                physical.position[0], physical.position[1], physical.position[2]
            ));
        });

        ui.horizontal(|ui| {
            ui.label("Velocity:");
            ui.label(format!(
                "({:.3}, {:.3}, {:.3})",
                physical.velocity[0], physical.velocity[1], physical.velocity[2]
            ));
        });

        ui.horizontal(|ui| {
            ui.label("Mass:");
            ui.label(format!("{:.3}", physical.mass));
        });

        ui.horizontal(|ui| {
            ui.label("Temperature:");
            ui.label(format!("{:.2}K", physical.temperature));
        });

        ui.horizontal(|ui| {
            ui.label("Charge:");
            ui.label(format!("{:.3}", physical.charge));
        });

        ui.horizontal(|ui| {
            ui.label("Energy:");
            ui.label(format!("{:.3}", physical.energy_level));
        });

        ui.horizontal(|ui| {
            ui.label("Health:");
            let health = physical.health.clamp(0.0, 1.0);
            ui.add(
                egui::ProgressBar::new(health as f32)
                    .fill(health_color(health))
                    .show_percentage(),
            );
        });
    }

    fn show_behavioral(
        &mut self,
        ui: &mut egui::Ui,
        behavioral: &crate::simulation_v3::observation_layer::BehavioralObservation,
    ) {
        ui.heading("Behavioral");

        let need_labels = ["Hunger", "Social", "Rest", "Exploration", "Growth"];
        for (i, label) in need_labels.iter().enumerate() {
            let value = behavioral.needs[i].clamp(0.0, 1.0);
            ui.horizontal(|ui| {
                ui.label(format!("{label}:"));
                ui.add(
                    egui::ProgressBar::new(value as f32)
                        .fill(need_color(i))
                        .show_percentage(),
                );
            });
        }

        ui.horizontal(|ui| {
            ui.label("Mood:");
            let mood = behavioral.mood.clamp(-1.0, 1.0);
            let mood_normalized = ((mood + 1.0) / 2.0) as f32;
            ui.add(
                egui::ProgressBar::new(mood_normalized)
                    .fill(mood_color(mood))
                    .text(format!("{mood:+.2}")),
            );
        });

        if let Some(ref goal) = behavioral.current_goal {
            ui.horizontal(|ui| {
                ui.label("Goal:");
                ui.label(goal);
            });
        } else {
            ui.label("Goal: None");
        }

        ui.horizontal(|ui| {
            ui.label("Stability:");
            ui.label(format!("{:.2}", behavioral.stability));
        });

        ui.horizontal(|ui| {
            ui.label("Novelty:");
            ui.label(format!("{:.2}", behavioral.novelty));
        });
    }

    fn show_environmental(
        &mut self,
        ui: &mut egui::Ui,
        env: &crate::simulation_v3::observation_layer::EnvironmentalObservation,
    ) {
        ui.heading("Environment");

        ui.horizontal(|ui| {
            ui.label("Terrain:");
            ui.label(terrain_label(env.terrain_type));
        });

        ui.horizontal(|ui| {
            ui.label("Weather:");
            let intensity = env.weather_intensity.clamp(0.0, 1.0);
            ui.add(egui::ProgressBar::new(intensity as f32).fill(weather_color(intensity)));
        });

        ui.horizontal(|ui| {
            ui.label("Resources:");
            let density = env.resource_density.clamp(0.0, 1.0);
            ui.add(egui::ProgressBar::new(density as f32).fill(resource_color(density)));
        });

        ui.horizontal(|ui| {
            ui.label("Danger:");
            let danger = env.danger_level.clamp(0.0, 1.0);
            ui.add(egui::ProgressBar::new(danger as f32).fill(danger_color(danger)));
        });

        ui.horizontal(|ui| {
            ui.label("Social density:");
            ui.label(format!("{:.2}", env.social_density));
        });
    }

    fn show_events(&mut self, ui: &mut egui::Ui, adapter: &SimulationRunnerAdapter) {
        ui.heading("Recent Events");

        let events = adapter.get_recent_events();
        if events.is_empty() {
            ui.label("No events yet");
            return;
        }

        let display_count = events.len().min(10);
        let recent = &events[events.len() - display_count..];

        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for event in recent.iter().rev() {
                    ui.horizontal(|ui| {
                        ui.label(format!("[t{}] ", event.timestamp));
                        let intensity_color = if event.intensity > 0.7 {
                            egui::Color32::RED
                        } else if event.intensity > 0.4 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::GREEN
                        };
                        ui.label(
                            egui::RichText::new(format!("{}", event.event_type))
                                .color(intensity_color),
                        );
                    });
                }
            });
    }
}

fn health_color(health: f64) -> egui::Color32 {
    if health > 0.7 {
        egui::Color32::GREEN
    } else if health > 0.4 {
        egui::Color32::YELLOW
    } else {
        egui::Color32::RED
    }
}

fn need_color(index: usize) -> egui::Color32 {
    match index {
        0 => egui::Color32::from_rgb(255, 140, 50),
        1 => egui::Color32::from_rgb(100, 180, 255),
        2 => egui::Color32::from_rgb(180, 130, 255),
        3 => egui::Color32::from_rgb(100, 220, 150),
        4 => egui::Color32::from_rgb(255, 200, 80),
        _ => egui::Color32::GRAY,
    }
}

fn mood_color(mood: f64) -> egui::Color32 {
    if mood > 0.3 {
        egui::Color32::GREEN
    } else if mood > -0.3 {
        egui::Color32::YELLOW
    } else {
        egui::Color32::RED
    }
}

fn terrain_label(terrain: TerrainType) -> &'static str {
    SimulationRunnerAdapter::terrain_type_display(terrain)
}

fn weather_color(intensity: f64) -> egui::Color32 {
    if intensity > 0.7 {
        egui::Color32::from_rgb(100, 100, 200)
    } else if intensity > 0.3 {
        egui::Color32::from_rgb(180, 180, 220)
    } else {
        egui::Color32::from_rgb(220, 220, 240)
    }
}

fn resource_color(density: f64) -> egui::Color32 {
    if density > 0.7 {
        egui::Color32::from_rgb(80, 200, 80)
    } else if density > 0.3 {
        egui::Color32::from_rgb(200, 200, 80)
    } else {
        egui::Color32::from_rgb(200, 120, 80)
    }
}

fn danger_color(danger: f64) -> egui::Color32 {
    if danger > 0.7 {
        egui::Color32::RED
    } else if danger > 0.3 {
        egui::Color32::from_rgb(255, 160, 50)
    } else {
        egui::Color32::from_rgb(80, 200, 80)
    }
}
