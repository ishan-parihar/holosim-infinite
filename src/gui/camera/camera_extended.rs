//! Camera Extended - Smooth Transition Support
//!
//! Extension methods for Camera to support smooth view transitions
//! in the multi-level view system.

use crate::gui::camera::camera::Camera;
use nalgebra_glm::Vec3;

impl Camera {
    /// Smoothly transition camera to target state
    ///
    /// # Arguments
    /// * `target_position` - Target camera position
    /// * `target_zoom` - Target zoom level
    /// * `target_rotation` - Target rotation angle
    /// * `progress` - Transition progress (0.0 to 1.0)
    /// * `start_position` - Starting camera position
    /// * `start_zoom` - Starting zoom level
    /// * `start_rotation` - Starting rotation angle
    pub fn smooth_transition_to(
        &mut self,
        target_position: Vec3,
        target_zoom: f32,
        target_rotation: f32,
        progress: f32,
        start_position: Vec3,
        start_zoom: f32,
        start_rotation: f32,
    ) {
        self.position = lerp_vec3(&start_position, &target_position, progress);
        self.zoom = lerp(start_zoom, target_zoom, progress);
        self.rotation = lerp_angle(start_rotation, target_rotation, progress);
    }

    /// Get current camera state as tuple
    pub fn get_state(&self) -> (Vec3, f32, f32) {
        (self.position, self.zoom, self.rotation)
    }
}

/// Linear interpolation between two values
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linear interpolation between two Vec3 values
fn lerp_vec3(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
    Vec3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t))
}

/// Linear interpolation between two angles (handles wrapping)
fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let diff = b - a;
    let normalized_diff =
        ((diff + std::f32::consts::PI) % (2.0 * std::f32::consts::PI)) - std::f32::consts::PI;
    a + normalized_diff * t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::camera::camera::Camera;

    #[test]
    fn test_camera_smooth_transition() {
        let mut camera = Camera::new(16.0 / 9.0);

        let start_pos = Vec3::new(0.0, 0.0, 0.0);
        let target_pos = Vec3::new(10.0, 20.0, 30.0);

        camera.smooth_transition_to(
            target_pos,
            2.0,
            std::f32::consts::FRAC_PI_2,
            0.5,
            start_pos,
            1.0,
            0.0,
        );

        assert!((camera.position.x - 5.0).abs() < 0.001);
        assert!((camera.position.y - 10.0).abs() < 0.001);
        assert!((camera.position.z - 15.0).abs() < 0.001);
        assert!((camera.zoom - 1.5).abs() < 0.001);
        assert!((camera.rotation - std::f32::consts::FRAC_PI_4).abs() < 0.001);
    }

    #[test]
    fn test_camera_get_state() {
        let camera = Camera::new(16.0 / 9.0);
        let (pos, zoom, rotation) = camera.get_state();

        assert_eq!(pos, camera.position);
        assert_eq!(zoom, camera.zoom);
        assert_eq!(rotation, camera.rotation);
    }
}
