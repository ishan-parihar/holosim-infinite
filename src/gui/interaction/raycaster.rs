//! Raycasting System - 3D Entity Selection at All Scales
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 5 Week 10:
//! "Raycasting system - 3D picking for entities, Accurate selection at all scales, Hover highlighting"
//!
//! This module provides:
//! - Accurate 3D raycasting using camera matrices
//! - Multi-scale entity selection (61 orders of magnitude)
//! - Hover and selection highlighting
//! - Entity hit testing with scale-aware hit radii

use crate::entity_layer7::layer7::EntityId;
use crate::gui::camera::Camera2D as Camera3D;
use crate::gui::{Coordinate3D, ScreenPosition};
use nalgebra_glm::{Mat4, Vec3};

/// Result of a raycast operation
#[derive(Debug, Clone, PartialEq)]
pub enum RaycastResult {
    /// Hit an entity
    Entity {
        entity_id: EntityId,
        hit_distance: f32,
        world_position: Coordinate3D,
    },
    /// Hit empty space
    EmptySpace {
        world_position: Coordinate3D,
        ray_direction: Vec3,
    },
    /// Click was outside viewport
    OutOfBounds,
}

/// Ray in 3D space (origin and direction)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /// Ray origin in world space
    pub origin: Vec3,
    /// Ray direction (normalized)
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let normalized = nalgebra_glm::normalize(&direction);
        Ray {
            origin,
            direction: normalized,
        }
    }

    /// Get point at distance t along ray
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Intersect ray with sphere
    /// Returns distance to intersection point if hit
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - center;
        let a = nalgebra_glm::dot(&self.direction, &self.direction);
        let b = 2.0 * nalgebra_glm::dot(&oc, &self.direction);
        let c = nalgebra_glm::dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 >= 0.0 {
                Some(t1)
            } else if t2 >= 0.0 {
                Some(t2)
            } else {
                None
            }
        }
    }
}

/// 3D Raycaster for entity selection
pub struct Raycaster3D {
    /// Screen dimensions
    screen_width: u32,
    screen_height: u32,
    /// Current camera
    camera: Option<Camera3D>,
    /// View matrix
    view_matrix: Mat4,
    /// Projection matrix
    proj_matrix: Mat4,
    /// Inverse view-projection matrix for unprojection
    inv_view_proj: Option<Mat4>,
    /// Selection radius multiplier (for easier clicking)
    selection_radius_multiplier: f32,
    /// Enable debug output
    debug_mode: bool,
}

impl Raycaster3D {
    /// Create a new 3D raycaster
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Raycaster3D {
            screen_width,
            screen_height,
            camera: None,
            view_matrix: Mat4::identity(),
            proj_matrix: Mat4::identity(),
            inv_view_proj: None,
            selection_radius_multiplier: 2.0,
            debug_mode: false,
        }
    }

    /// Set screen dimensions
    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.screen_width = width;
        self.screen_height = height;
        self.update_inverse_matrix();
    }

    /// Set camera and update matrices
    pub fn set_camera(&mut self, camera: &Camera3D) {
        self.camera = Some(camera.clone());
        self.view_matrix = camera.view_matrix();
        self.proj_matrix = camera.projection_matrix();
        self.update_inverse_matrix();
    }

    /// Update inverse view-projection matrix
    fn update_inverse_matrix(&mut self) {
        let view_proj = self.proj_matrix * self.view_matrix;
        self.inv_view_proj = view_proj.try_inverse();
    }

    /// Set selection radius multiplier
    pub fn set_selection_radius_multiplier(&mut self, multiplier: f32) {
        self.selection_radius_multiplier = multiplier.clamp(0.5, 5.0);
    }

    /// Enable/disable debug mode
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    /// Generate a ray from screen position
    pub fn screen_to_ray(&self, screen_pos: ScreenPosition) -> Option<Ray> {
        // Validate screen position
        if screen_pos.x < 0.0
            || screen_pos.x >= self.screen_width as f32
            || screen_pos.y < 0.0
            || screen_pos.y >= self.screen_height as f32
        {
            return None;
        }

        // Get camera position
        let _camera_pos = match &self.camera {
            Some(cam) => cam.position,
            None => return None,
        };

        // Convert screen coordinates to normalized device coordinates (NDC)
        // NDC: x, y in [-1, 1], z in [0, 1] for OpenGL-style, but we use [-1, 1]
        let ndc_x = (2.0 * screen_pos.x / self.screen_width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_pos.y / self.screen_height as f32); // Flip Y

        // Unproject two points on the ray: near and far planes
        let inv_view_proj = self.inv_view_proj?;

        // Near plane point (z = -1 in NDC)
        let near_ndc = Vec3::new(ndc_x, ndc_y, -1.0);
        let near_world = Self::unproject(&inv_view_proj, &near_ndc);

        // Far plane point (z = 1 in NDC)
        let far_ndc = Vec3::new(ndc_x, ndc_y, 1.0);
        let far_world = Self::unproject(&inv_view_proj, &far_ndc);

        // Ray direction from near to far
        let direction = nalgebra_glm::normalize(&(far_world - near_world));

        Some(Ray::new(near_world, direction))
    }

    /// Unproject NDC coordinates to world space
    fn unproject(inv_view_proj: &Mat4, ndc: &Vec3) -> Vec3 {
        let clip = Vec3::new(ndc.x, ndc.y, ndc.z);
        let world_h = inv_view_proj * nalgebra_glm::Vec4::new(clip.x, clip.y, clip.z, 1.0);

        // Perspective divide
        if world_h.w != 0.0 {
            Vec3::new(
                world_h.x / world_h.w,
                world_h.y / world_h.w,
                world_h.z / world_h.w,
            )
        } else {
            Vec3::new(world_h.x, world_h.y, world_h.z)
        }
    }

    /// Cast a ray and find the closest entity hit
    pub fn cast_ray(
        &self,
        screen_pos: ScreenPosition,
        entities: &[(EntityId, Coordinate3D, f64)], // (id, position, scale)
    ) -> RaycastResult {
        // Check bounds
        if screen_pos.x < 0.0
            || screen_pos.x >= self.screen_width as f32
            || screen_pos.y < 0.0
            || screen_pos.y >= self.screen_height as f32
        {
            return RaycastResult::OutOfBounds;
        }

        // Generate ray
        let ray = match self.screen_to_ray(screen_pos) {
            Some(r) => r,
            None => {
                return RaycastResult::EmptySpace {
                    world_position: Coordinate3D::origin(),
                    ray_direction: Vec3::new(0.0, 0.0, -1.0),
                }
            }
        };

        if self.debug_mode {
            println!("Raycasting from ({}, {})", screen_pos.x, screen_pos.y);
            println!("  Origin: {:?}", ray.origin);
            println!("  Direction: {:?}", ray.direction);
        }

        // Test against all entities
        let mut closest_hit: Option<(EntityId, f32, Coordinate3D)> = None;

        for (entity_id, position, scale) in entities {
            // Convert to Vec3
            let entity_pos = Vec3::new(position.x as f32, position.y as f32, position.z as f32);

            // Calculate hit radius based on entity scale
            // In logarithmic space, visual size varies with log10(scale)
            let visual_scale = scale.log10().abs() as f32;
            let hit_radius = (visual_scale * 0.5).max(0.1) * self.selection_radius_multiplier;

            // Test intersection
            if let Some(distance) = ray.intersect_sphere(entity_pos, hit_radius) {
                // Check if this is the closest hit
                if closest_hit.is_none() || distance < closest_hit.as_ref().unwrap().1 {
                    let hit_point = ray.at(distance);
                    closest_hit = Some((
                        entity_id.clone(),
                        distance,
                        Coordinate3D::new(
                            hit_point.x as f64,
                            hit_point.y as f64,
                            hit_point.z as f64,
                        ),
                    ));
                }
            }
        }

        match closest_hit {
            Some((entity_id, distance, hit_pos)) => RaycastResult::Entity {
                entity_id,
                hit_distance: distance,
                world_position: hit_pos,
            },
            None => {
                // Calculate approximate world position at fixed distance
                let distance = 100.0; // Arbitrary distance for empty space click
                let world_pos = ray.at(distance);
                RaycastResult::EmptySpace {
                    world_position: Coordinate3D::new(
                        world_pos.x as f64,
                        world_pos.y as f64,
                        world_pos.z as f64,
                    ),
                    ray_direction: ray.direction,
                }
            }
        }
    }

    /// Get the ray from a screen position
    pub fn get_ray(&self, screen_pos: ScreenPosition) -> Option<Ray> {
        self.screen_to_ray(screen_pos)
    }

    /// Project a 3D world point to screen coordinates
    pub fn world_to_screen(&self, world_pos: &Coordinate3D) -> Option<ScreenPosition> {
        let view_proj = self.proj_matrix * self.view_matrix;
        let world_vec = nalgebra_glm::Vec4::new(
            world_pos.x as f32,
            world_pos.y as f32,
            world_pos.z as f32,
            1.0,
        );
        let clip = view_proj * world_vec;

        // Perspective divide
        if clip.w == 0.0 {
            return None;
        }

        let ndc_x = clip.x / clip.w;
        let ndc_y = clip.y / clip.w;
        let ndc_z = clip.z / clip.w;

        // Check if behind camera
        if !(-1.0..=1.0).contains(&ndc_z) {
            return None;
        }

        // Convert NDC to screen coordinates
        let screen_x = ((ndc_x + 1.0) * 0.5 * self.screen_width as f32).max(0.0);
        let screen_y = ((1.0 - ndc_y) * 0.5 * self.screen_height as f32).max(0.0);

        Some(ScreenPosition::new(screen_x, screen_y))
    }

    /// Check if an entity is visible on screen
    pub fn is_entity_visible(&self, position: &Coordinate3D, scale: f64) -> bool {
        match self.world_to_screen(position) {
            Some(screen_pos) => {
                // Check if on screen
                let margin = (scale.log10().abs() as f32 * 10.0).max(10.0);
                screen_pos.x >= -margin
                    && screen_pos.x < (self.screen_width as f32 + margin)
                    && screen_pos.y >= -margin
                    && screen_pos.y < (self.screen_height as f32 + margin)
            }
            None => false,
        }
    }

    /// Calculate distance from screen position to entity center
    pub fn distance_to_entity(
        &self,
        screen_pos: ScreenPosition,
        entity_pos: &Coordinate3D,
    ) -> Option<f32> {
        let entity_screen = self.world_to_screen(entity_pos)?;
        let dx = screen_pos.x - entity_screen.x;
        let dy = screen_pos.y - entity_screen.y;
        Some((dx * dx + dy * dy).sqrt())
    }

    /// Get screen bounds
    pub fn get_screen_size(&self) -> (u32, u32) {
        (self.screen_width, self.screen_height)
    }
}

impl Default for Raycaster3D {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Selection state for highlighting
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionState {
    /// Not selected
    None,
    /// Hovered (mouse over)
    Hovered,
    /// Selected (clicked)
    Selected,
    /// Tracked (camera following)
    Tracked,
}

/// Selection manager for entity highlighting
#[derive(Clone)]
pub struct SelectionManager {
    /// Currently hovered entity
    pub hovered_entity: Option<EntityId>,
    /// Currently selected entity
    pub selected_entity: Option<EntityId>,
    /// Currently tracked entity
    pub tracked_entity: Option<EntityId>,
    /// Selection history
    pub selection_history: Vec<EntityId>,
    /// Maximum history size
    pub max_history_size: usize,
    /// Enable hover highlighting
    pub hover_highlighting: bool,
    /// Enable selection effects
    pub selection_effects: bool,
}

impl SelectionManager {
    /// Create a new selection manager
    pub fn new() -> Self {
        SelectionManager {
            hovered_entity: None,
            selected_entity: None,
            tracked_entity: None,
            selection_history: Vec::new(),
            max_history_size: 100,
            hover_highlighting: true,
            selection_effects: true,
        }
    }

    /// Set hovered entity
    pub fn set_hovered(&mut self, entity_id: Option<EntityId>) {
        self.hovered_entity = entity_id;
    }

    /// Select an entity
    pub fn select(&mut self, entity_id: EntityId) {
        // Save previous selection to history
        if let Some(prev) = &self.selected_entity {
            if prev != &entity_id {
                self.selection_history.push(prev.clone());
                if self.selection_history.len() > self.max_history_size {
                    self.selection_history.remove(0);
                }
            }
        }
        self.selected_entity = Some(entity_id);
    }

    /// Deselect current entity
    pub fn deselect(&mut self) {
        self.selected_entity = None;
    }

    /// Track an entity
    pub fn track(&mut self, entity_id: EntityId) {
        self.tracked_entity = Some(entity_id);
    }

    /// Stop tracking
    pub fn stop_tracking(&mut self) {
        self.tracked_entity = None;
    }

    /// Get selection state for an entity
    pub fn get_state(&self, entity_id: &EntityId) -> SelectionState {
        if self.tracked_entity.as_ref() == Some(entity_id) {
            SelectionState::Tracked
        } else if self.selected_entity.as_ref() == Some(entity_id) {
            SelectionState::Selected
        } else if self.hovered_entity.as_ref() == Some(entity_id) {
            SelectionState::Hovered
        } else {
            SelectionState::None
        }
    }

    /// Check if an entity is selected
    pub fn is_selected(&self, entity_id: &EntityId) -> bool {
        self.selected_entity.as_ref() == Some(entity_id)
    }

    /// Check if an entity is hovered
    pub fn is_hovered(&self, entity_id: &EntityId) -> bool {
        self.hovered_entity.as_ref() == Some(entity_id)
    }

    /// Check if an entity is tracked
    pub fn is_tracked(&self, entity_id: &EntityId) -> bool {
        self.tracked_entity.as_ref() == Some(entity_id)
    }

    /// Navigate back to previous selection
    pub fn select_previous(&mut self) -> Option<EntityId> {
        if let Some(prev) = self.selection_history.pop() {
            self.selected_entity = Some(prev.clone());
            Some(prev)
        } else {
            None
        }
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.hovered_entity = None;
        self.selected_entity = None;
        self.tracked_entity = None;
        self.selection_history.clear();
    }

    /// Get the currently selected entity
    pub fn get_selected(&self) -> Option<&EntityId> {
        self.selected_entity.as_ref()
    }

    /// Get the currently hovered entity
    pub fn get_hovered(&self) -> Option<&EntityId> {
        self.hovered_entity.as_ref()
    }

    /// Get the currently tracked entity
    pub fn get_tracked(&self) -> Option<&EntityId> {
        self.tracked_entity.as_ref()
    }
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert!((ray.direction - direction).magnitude() < 0.001);
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let point = ray.at(5.0);

        assert!((point.x - 0.0).abs() < 0.001);
        assert!((point.y - 0.0).abs() < 0.001);
        assert!((point.z - (-5.0)).abs() < 0.001);
    }

    #[test]
    fn test_ray_intersect_sphere_hit() {
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 1.0;

        let hit = ray.intersect_sphere(center, radius);
        assert!(hit.is_some());

        let distance = hit.unwrap();
        assert!(distance > 3.0 && distance < 5.0);
    }

    #[test]
    fn test_ray_intersect_sphere_miss() {
        let ray = Ray::new(Vec3::new(5.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 1.0;

        let hit = ray.intersect_sphere(center, radius);
        assert!(hit.is_none());
    }

    #[test]
    fn test_raycaster_new() {
        let raycaster = Raycaster3D::new(1920, 1080);
        assert_eq!(raycaster.screen_width, 1920);
        assert_eq!(raycaster.screen_height, 1080);
    }

    #[test]
    fn test_raycaster_set_screen_size() {
        let mut raycaster = Raycaster3D::new(1920, 1080);
        raycaster.set_screen_size(1280, 720);
        assert_eq!(raycaster.screen_width, 1280);
        assert_eq!(raycaster.screen_height, 720);
    }

    #[test]
    fn test_raycaster_out_of_bounds() {
        let raycaster = Raycaster3D::new(1920, 1080);
        let entities = vec![];

        let result = raycaster.cast_ray(ScreenPosition::new(-1.0, 500.0), &entities);
        assert_eq!(result, RaycastResult::OutOfBounds);

        let result = raycaster.cast_ray(ScreenPosition::new(2000.0, 500.0), &entities);
        assert_eq!(result, RaycastResult::OutOfBounds);
    }

    #[test]
    fn test_selection_manager_new() {
        let manager = SelectionManager::new();
        assert!(manager.hovered_entity.is_none());
        assert!(manager.selected_entity.is_none());
        assert!(manager.tracked_entity.is_none());
        assert!(manager.hover_highlighting);
        assert!(manager.selection_effects);
    }

    #[test]
    fn test_selection_manager_select() {
        let mut manager = SelectionManager::new();
        let entity_id = EntityId::new("1".to_string());

        manager.select(entity_id.clone());
        assert_eq!(manager.selected_entity, Some(entity_id));
    }

    #[test]
    fn test_selection_manager_hover() {
        let mut manager = SelectionManager::new();
        let entity_id = EntityId::new("1".to_string());

        manager.set_hovered(Some(entity_id.clone()));
        assert_eq!(manager.hovered_entity, Some(entity_id));
    }

    #[test]
    fn test_selection_manager_track() {
        let mut manager = SelectionManager::new();
        let entity_id = EntityId::new("1".to_string());

        manager.track(entity_id.clone());
        assert_eq!(manager.tracked_entity, Some(entity_id));
    }

    #[test]
    fn test_selection_manager_get_state() {
        let mut manager = SelectionManager::new();
        let entity1 = EntityId::new("1".to_string());
        let entity2 = EntityId::new("2".to_string());
        let entity3 = EntityId::new("3".to_string());

        manager.set_hovered(Some(entity1.clone()));
        manager.select(entity2.clone());
        manager.track(entity3.clone());

        assert_eq!(manager.get_state(&entity1), SelectionState::Hovered);
        assert_eq!(manager.get_state(&entity2), SelectionState::Selected);
        assert_eq!(manager.get_state(&entity3), SelectionState::Tracked);

        let entity4 = EntityId::new("4".to_string());
        assert_eq!(manager.get_state(&entity4), SelectionState::None);
    }

    #[test]
    fn test_selection_manager_history() {
        let mut manager = SelectionManager::new();
        let entity1 = EntityId::new("1".to_string());
        let entity2 = EntityId::new("2".to_string());

        manager.select(entity1.clone());
        manager.select(entity2.clone());

        assert_eq!(manager.selection_history.len(), 1);
        assert_eq!(manager.selection_history[0], entity1);

        let prev = manager.select_previous();
        assert_eq!(prev, Some(entity1.clone()));
        assert_eq!(manager.selected_entity, Some(entity1));
    }

    #[test]
    fn test_selection_manager_clear() {
        let mut manager = SelectionManager::new();
        let entity_id = EntityId::new("1".to_string());

        manager.select(entity_id.clone());
        manager.set_hovered(Some(entity_id.clone()));
        manager.track(entity_id.clone());

        manager.clear();

        assert!(manager.selected_entity.is_none());
        assert!(manager.hovered_entity.is_none());
        assert!(manager.tracked_entity.is_none());
        assert!(manager.selection_history.is_empty());
    }
}
