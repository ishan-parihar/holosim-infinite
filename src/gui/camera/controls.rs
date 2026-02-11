use super::Camera2D as Camera;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, NamedKey},
};

#[derive(Debug, Clone)]
pub struct CameraControls {
    pub is_dragging: bool,
    pub last_mouse_pos: Option<PhysicalPosition<f64>>,
    pub zoom_sensitivity: f32,
    pub pan_sensitivity: f32,
}

impl CameraControls {
    pub fn new() -> Self {
        CameraControls {
            is_dragging: false,
            last_mouse_pos: None,
            zoom_sensitivity: 1.0,
            pan_sensitivity: 1.0,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, camera: &mut Camera) {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                if button == &MouseButton::Left {
                    match state {
                        ElementState::Pressed => {
                            self.is_dragging = true;
                        }
                        ElementState::Released => {
                            self.is_dragging = false;
                            self.last_mouse_pos = None;
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.is_dragging {
                    if let Some(last_pos) = self.last_mouse_pos {
                        let dx = (position.x - last_pos.x) as f32 * self.pan_sensitivity;
                        let dy = (position.y - last_pos.y) as f32 * self.pan_sensitivity;
                        camera.pan(dx, dy);
                    }
                    self.last_mouse_pos = Some(*position);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_delta = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                };
                camera.zoom(zoom_delta * self.zoom_sensitivity);
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state, logical_key, ..
                },
                ..
            } => {
                if state == &ElementState::Pressed {
                    self.handle_keyboard(logical_key, camera);
                }
            }
            _ => {}
        }
    }

    fn handle_keyboard(&self, logical_key: &Key, camera: &mut Camera) {
        let pan_speed = 0.1 * self.pan_sensitivity;
        let zoom_speed = 0.1 * self.zoom_sensitivity;
        let rotate_speed = 0.1;

        match logical_key {
            Key::Named(NamedKey::ArrowUp) => {
                camera.pan(0.0, pan_speed);
            }
            Key::Named(NamedKey::ArrowDown) => {
                camera.pan(0.0, -pan_speed);
            }
            Key::Named(NamedKey::ArrowLeft) => {
                camera.pan(-pan_speed, 0.0);
            }
            Key::Named(NamedKey::ArrowRight) => {
                camera.pan(pan_speed, 0.0);
            }
            Key::Named(NamedKey::PageUp) => {
                camera.zoom(zoom_speed);
            }
            Key::Named(NamedKey::PageDown) => {
                camera.zoom(-zoom_speed);
            }
            Key::Named(NamedKey::Home) => {
                camera.zoom = 1.0;
                camera.position = nalgebra_glm::Vec3::new(0.0, 0.0, 0.0);
                camera.rotation = 0.0;
            }
            Key::Character(c) => match c.as_str() {
                "q" => {
                    camera.rotate(rotate_speed);
                }
                "e" => {
                    camera.rotate(-rotate_speed);
                }
                "+" => {
                    camera.zoom(zoom_speed);
                }
                "-" => {
                    camera.zoom(-zoom_speed);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl Default for CameraControls {
    fn default() -> Self {
        Self::new()
    }
}
