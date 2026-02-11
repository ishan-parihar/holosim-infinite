use nalgebra_glm::{Mat4, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub zoom: f32,
    pub rotation: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub aspect_ratio: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            zoom: 1.0,
            rotation: 0.0,
            min_zoom: 0.01,
            max_zoom: 100.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            zoom: 1.0,
            rotation: 0.0,
            min_zoom: 0.01,
            max_zoom: 100.0,
            aspect_ratio,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let scale = 1.0 / self.zoom;
        self.position.x += dx * scale;
        self.position.y += dy * scale;
    }

    pub fn zoom(&mut self, delta: f32) {
        let zoom_factor = 1.0 + delta * 0.1;
        self.zoom = (self.zoom * zoom_factor).clamp(self.min_zoom, self.max_zoom);
    }

    pub fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
    }

    pub fn view_matrix(&self) -> Mat4 {
        let mut view = Mat4::identity();
        view = nalgebra_glm::rotate_z(&view, -self.rotation);
        view = nalgebra_glm::translate(&view, &-self.position);
        view
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let height = 2.0;
        let width = height * self.aspect_ratio;
        let left = -width / 2.0 / self.zoom;
        let right = width / 2.0 / self.zoom;
        let bottom = -height / 2.0 / self.zoom;
        let top = height / 2.0 / self.zoom;
        nalgebra_glm::ortho(left, right, bottom, top, -1.0, 1.0)
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> Vec3 {
        let aspect_ratio = self.aspect_ratio;
        let height = 2.0;
        let width = height * aspect_ratio;

        let ndc_x = (screen_x / width) * 2.0 - 1.0;
        let ndc_y = (screen_y / height) * 2.0 - 1.0;

        let world_x = ndc_x * width / 2.0 / self.zoom + self.position.x;
        let world_y = -ndc_y * height / 2.0 / self.zoom + self.position.y;

        Vec3::new(world_x, world_y, 0.0)
    }

    pub fn world_to_screen(&self, world_pos: &Vec3) -> (f32, f32) {
        let pos4 = nalgebra_glm::vec4(world_pos.x, world_pos.y, world_pos.z, 1.0);
        let transformed = self.view_projection_matrix() * pos4;

        let aspect_ratio = self.aspect_ratio;
        let height = 2.0;
        let width = height * aspect_ratio;

        let screen_x = (transformed.x + 1.0) / 2.0 * width;
        let screen_y = (1.0 - transformed.y) / 2.0 * height;

        (screen_x, screen_y)
    }
}
