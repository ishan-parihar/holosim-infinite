//! Working GUI - Actually Renders Entities
//!
//! This creates a minimal working GUI that actually shows the simulation.

use holonic_realms::simulation_v3::simulation_runner::{
    SimulationRunner, SimulationParameters,
};
use miniquad::{conf::*, *};
use std::time::Instant;

struct GuiState {
    runner: SimulationRunner,
    frame_count: u64,
    last_update: Instant,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
}

impl GuiState {
    fn new() -> Self {
        let parameters = SimulationParameters {
            num_entities: 50,
            num_steps: 1000,
            run_involution: true,
            run_evolution: true,
            update_holographic_field: true,
            update_physical_manifestations: true,
            generate_detailed_reports: false,
            holographic_performance_config: None,
            ..Default::default()
        };
        
        let runner = SimulationRunner::new(parameters);
        
        GuiState {
            runner,
            frame_count: 0,
            last_update: Instant::now(),
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl EventHandler for GuiState {
    fn update(&mut self, _ctx: &mut Context) {
        // Run simulation step
        if let Err(e) = self.runner.run_simulation() {
            eprintln!("Simulation error: {:?}", e);
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // Clear screen
        clear(BLACK, ctx);
        
        // Draw title
        let scale = self.zoom * ctx.screen_size().0 as f32 / 1920.0;
        let text_scale = scale * 30.0;
        
        ctx.draw_text(
            "HOLONIC REALMS - Working GUI",
            TextParams {
                x: ctx.screen_size().0 as f32 / 2.0,
                y: ctx.screen_size().1 as f32 * 0.1,
                rotation: 0.0,
                font_size: text_scale,
                font_id: FontId::default(),
                color: WHITE,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Top,
            },
        );
        
        // Draw simulation info
        let state = self.runner.get_state();
        let info = format!(
            "Step: {} | Coherence: {:.3} | Zoom: {:.2}",
            self.frame_count,
            state.coherence,
            self.zoom
        );
        
        ctx.draw_text(
            &info,
            TextParams {
                x: ctx.screen_size().0 as f32 / 2.0,
                y: ctx.screen_size().1 as f32 * 0.9,
                rotation: 0.0,
                font_size: text_scale * 0.6,
                font_id: FontId::default(),
                color: WHITE,
                horizontal_alignment: HorizontalAlignment::Center,
                vertical_alignment: VerticalAlignment::Bottom,
            },
        );
        
        // Draw entities as circles
        let screen_center_x = ctx.screen_size().0 as f32 / 2.0;
        let screen_center_y = ctx.screen_size().1 as f32 / 2.0;
        let entity_scale = scale * 10.0;
        
        // Get entities from simulation
        if let Some(result) = self.runner.get_last_result() {
            for (i, entity) in result.involution_result.entities.iter().enumerate() {
                // Calculate position based on entity properties
                let angle = (i as f32 * 137.5) * std::f32::consts::PI / 180.0;
                let radius = 100.0 + (entity.current_state.consciousness_level * 200.0);
                
                let x = screen_center_x + self.offset_x + radius * angle.cos();
                let y = screen_center_y + self.offset_y + radius * angle.sin();
                
                // Color based on entity type
                let color = match entity.entity_type {
                    holonic_realms::entity_layer7::layer7::EntityType::Individual => BLUE,
                    holonic_realms::entity_layer7::layer7::EntityType::SolarLogos => YELLOW,
                    holonic_realms::entity_layer7::layer7::EntityType::GalacticLogos => PURPLE,
                    _ => GREEN,
                };
                
                // Draw entity
                ctx.draw_circle(
                    x,
                    y,
                    entity_scale * (1.0 + entity.current_state.consciousness_level),
                    color,
                );
                
                // Draw consciousness level indicator
                let consciousness_level = entity.current_state.consciousness_level;
                ctx.draw_circle(
                    x,
                    y,
                    entity_scale * consciousness_level * 0.5,
                    WHITE,
                );
            }
        }
        
        // Draw controls
        ctx.draw_text(
            "Controls: Mouse Wheel = Zoom | Drag = Pan",
            TextParams {
                x: 20.0,
                y: ctx.screen_size().1 as f32 - 20.0,
                rotation: 0.0,
                font_size: text_scale * 0.4,
                font_id: FontId::default(),
                color: GRAY,
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Bottom,
            },
        );
        
        self.frame_count += 1;
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y > 0.0 {
            self.zoom *= 1.1;
        } else {
            self.zoom *= 0.9;
        }
        self.zoom = self.zoom.max(0.1).min(10.0);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        // Simple pan
        self.offset_x += x;
        self.offset_y += y;
    }
}

fn main() {
    Window::new(
        conf::Window {
            title: String::from("Holonic Realms - Working GUI"),
            window_width: 1920,
            window_height: 1080,
            window_resizable: true,
            ..Default::default()
        },
        |_| GuiState::new(),
    );
}
