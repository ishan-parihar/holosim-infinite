//! Visualization Engine - Multi-scale rendering engine
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 5:
//! "Multi-Scale Visualization Engine:
//! - Quantum scale (10^-35 m)
//! - Atomic scale (10^-10 m)
//! - Molecular scale (10^-9 m)
//! - Cellular scale (10^-6 m)
//! - Organism scale (10^-3 m)
//! - Planetary scale (10^6 m)
//! - Stellar scale (10^9 m)
//! - Galactic scale (10^21 m)
//! - Universal scale (10^26 m)"
//!
//! This module provides:
//! - WGPU renderer integration
//! - Multi-scale camera system
//! - Scene graph management
//! - Involution and emergence visualization

use crate::entity_layer7::layer7::EntityId;
use crate::gui::renderer::{BufferManager, EntityRenderPipeline, WgpuContext};
use crate::gui::{Coordinate3D, GuiConfig, ScaleLevel};
use std::sync::Arc;

/// 3D camera with multi-scale capabilities
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position in 3D space
    pub position: Coordinate3D,

    /// Camera target (what it's looking at)
    pub target: Coordinate3D,

    /// Up vector (defines camera orientation)
    pub up: Coordinate3D,

    /// Field of view in degrees
    pub fov: f32,

    /// Near clipping plane
    pub near: f32,

    /// Far clipping plane
    pub far: f32,

    /// Current zoom level in meters
    pub zoom_level: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Coordinate3D::new(0.0, 0.0, 10.0),
            target: Coordinate3D::origin(),
            up: Coordinate3D::new(0.0, 1.0, 0.0),
            fov: 60.0,
            near: 0.1,
            far: 10000.0,
            zoom_level: 1.0e-6,
        }
    }
}

impl Camera {
    /// Create a new camera with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the position of the camera
    pub fn with_position(mut self, pos: Coordinate3D) -> Self {
        self.position = pos;
        self
    }

    /// Set the target of the camera
    pub fn with_target(mut self, target: Coordinate3D) -> Self {
        self.target = target;
        self
    }

    /// Set the zoom level in meters
    pub fn with_zoom_level(mut self, zoom: f64) -> Self {
        self.zoom_level = zoom;
        self
    }

    /// Move the camera to look at a specific entity
    pub fn look_at(&mut self, target: Coordinate3D) {
        self.target = target;
        // Adjust position to maintain current zoom level
        let offset = self
            .position
            .subtract(&target)
            .normalize()
            .scale(self.zoom_level);
        self.position = target.add(&offset);
    }

    /// Zoom in by a factor
    pub fn zoom_in(&mut self, factor: f64) {
        self.zoom_level = (self.zoom_level / factor).max(1.616255e-35); // Planck length
        self.update_position_for_zoom();
    }

    /// Zoom out by a factor
    pub fn zoom_out(&mut self, factor: f64) {
        self.zoom_level = (self.zoom_level * factor).min(8.8e26); // Observable universe
        self.update_position_for_zoom();
    }

    /// Set zoom level directly
    pub fn set_zoom_level(&mut self, zoom: f64) {
        self.zoom_level = zoom.clamp(1.616255e-35, 8.8e26);
        self.update_position_for_zoom();
    }

    /// Update camera position based on zoom level
    fn update_position_for_zoom(&mut self) {
        let direction = self.position.subtract(&self.target).normalize();
        self.position = self.target.add(&direction.scale(self.zoom_level));
    }

    /// Get the view direction
    pub fn view_direction(&self) -> Coordinate3D {
        self.target.subtract(&self.position).normalize()
    }

    /// Get the right vector
    pub fn right_vector(&self) -> Coordinate3D {
        let view = self.view_direction();
        let up = self.up;
        Coordinate3D::new(
            view.y * up.z - view.z * up.y,
            view.z * up.x - view.x * up.z,
            view.x * up.y - view.y * up.x,
        )
    }
}

/// Multi-scale camera that adapts to different scale levels
#[derive(Debug, Clone)]
pub struct MultiScaleCamera {
    /// Base camera
    pub camera: Camera,

    /// Current scale level
    pub current_scale: ScaleLevel,

    /// Target scale level (for smooth transitions)
    pub target_scale: ScaleLevel,

    /// Transition progress (0.0 to 1.0)
    pub transition_progress: f64,

    /// Whether a transition is in progress
    pub is_transitioning: bool,
}

impl Default for MultiScaleCamera {
    fn default() -> Self {
        MultiScaleCamera {
            camera: Camera::default(),
            current_scale: ScaleLevel::Cellular,
            target_scale: ScaleLevel::Cellular,
            transition_progress: 0.0,
            is_transitioning: false,
        }
    }
}

impl MultiScaleCamera {
    /// Create a new multi-scale camera
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the scale level
    pub fn set_scale(&mut self, scale: ScaleLevel) {
        if self.current_scale != scale {
            self.target_scale = scale;
            self.is_transitioning = true;
            self.transition_progress = 0.0;
        }
    }

    /// Update the transition
    pub fn update_transition(&mut self, delta_time: f64) {
        if self.is_transitioning {
            self.transition_progress += delta_time * 2.0; // 0.5 second transition
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.is_transitioning = false;
                self.current_scale = self.target_scale;
                self.camera
                    .set_zoom_level(self.target_scale.scale_in_meters());
            } else {
                // Interpolate zoom level
                let start_zoom = self.current_scale.scale_in_meters();
                let end_zoom = self.target_scale.scale_in_meters();
                let t = self.transition_progress;
                // Smooth interpolation
                let smooth_t = t * t * (3.0 - 2.0 * t);
                let current_zoom = start_zoom + (end_zoom - start_zoom) * smooth_t;
                self.camera.set_zoom_level(current_zoom);
            }
        }
    }

    /// Get the current effective scale level
    pub fn effective_scale(&self) -> ScaleLevel {
        if self.is_transitioning {
            ScaleLevel::from_meters(self.camera.zoom_level)
        } else {
            self.current_scale
        }
    }

    /// Focus on an entity at the current scale
    pub fn focus_on_entity(&mut self, _entity_id: EntityId, position: Coordinate3D) {
        self.camera.look_at(position);
    }
}

/// Renderable trait for objects that can be rendered
pub trait Renderable {
    /// Get the position of this renderable object
    fn position(&self) -> Coordinate3D;

    /// Get the scale of this object in meters
    fn scale(&self) -> f64;

    /// Get whether this object is visible at the given scale level
    fn is_visible_at(&self, scale: ScaleLevel) -> bool;

    /// Get the render priority (higher = rendered first)
    fn render_priority(&self) -> i32;
}

/// Renderable entity
#[derive(Debug, Clone)]
pub struct RenderableEntity {
    pub entity_id: EntityId,
    pub position: Coordinate3D,
    pub scale: f64,
    pub density: u8,
    pub polarity: i8,
    pub consciousness: f64,
    pub archetype_activations: Vec<(usize, f64)>,
}

impl Renderable for RenderableEntity {
    fn position(&self) -> Coordinate3D {
        self.position
    }

    fn scale(&self) -> f64 {
        self.scale
    }

    fn is_visible_at(&self, scale: ScaleLevel) -> bool {
        let entity_scale = ScaleLevel::from_meters(self.scale);
        let min_scale = scale.previous().unwrap_or(ScaleLevel::Quantum);
        (min_scale..=scale).contains(&entity_scale)
    }

    fn render_priority(&self) -> i32 {
        // Higher consciousness = higher priority
        (self.consciousness * 100.0) as i32
    }
}

/// Renderable star
#[derive(Debug, Clone)]
pub struct RenderableStar {
    pub star_id: u64,
    pub position: Coordinate3D,
    pub radius: f64,
    pub temperature: f64,
    pub luminosity: f64,
    pub spectral_type: String,
}

impl Renderable for RenderableStar {
    fn position(&self) -> Coordinate3D {
        self.position
    }

    fn scale(&self) -> f64 {
        self.radius
    }

    fn is_visible_at(&self, scale: ScaleLevel) -> bool {
        scale >= ScaleLevel::Stellar
    }

    fn render_priority(&self) -> i32 {
        (self.luminosity.log10() * 10.0) as i32
    }
}

/// Renderable planet
#[derive(Debug, Clone)]
pub struct RenderablePlanet {
    pub planet_id: u64,
    pub position: Coordinate3D,
    pub radius: f64,
    pub has_life: bool,
    pub atmosphere_composition: Vec<(String, f64)>,
    pub ecosystem_health: f64,
}

impl Renderable for RenderablePlanet {
    fn position(&self) -> Coordinate3D {
        self.position
    }

    fn scale(&self) -> f64 {
        self.radius
    }

    fn is_visible_at(&self, scale: ScaleLevel) -> bool {
        scale >= ScaleLevel::Planetary
    }

    fn render_priority(&self) -> i32 {
        if self.has_life {
            50
        } else {
            10
        }
    }
}

/// Renderable solar system
#[derive(Debug, Clone)]
pub struct RenderableSystem {
    pub system_id: u64,
    pub position: Coordinate3D,
    pub stars: Vec<RenderableStar>,
    pub planets: Vec<RenderablePlanet>,
}

impl Renderable for RenderableSystem {
    fn position(&self) -> Coordinate3D {
        self.position
    }

    fn scale(&self) -> f64 {
        // System scale based on outermost planet
        let mut max_dist = 1.0e11; // Default to 1 AU
        for planet in &self.planets {
            let dist = planet.position.distance_to(&self.position);
            if dist > max_dist {
                max_dist = dist;
            }
        }
        max_dist
    }

    fn is_visible_at(&self, scale: ScaleLevel) -> bool {
        scale >= ScaleLevel::Stellar
    }

    fn render_priority(&self) -> i32 {
        if self.stars.is_empty() {
            10
        } else {
            20 + (self.stars.len() as i32) * 5
        }
    }
}

/// Scene graph for organizing renderable objects
#[derive(Debug, Clone, Default)]
pub struct SceneGraph {
    /// Entities
    pub entities: Vec<RenderableEntity>,

    /// Stars
    pub stars: Vec<RenderableStar>,

    /// Planets
    pub planets: Vec<RenderablePlanet>,

    /// Solar systems
    pub systems: Vec<RenderableSystem>,
}

impl SceneGraph {
    /// Create a new empty scene graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entity to the scene
    pub fn add_entity(&mut self, entity: RenderableEntity) {
        self.entities.push(entity);
    }

    /// Add a star to the scene
    pub fn add_star(&mut self, star: RenderableStar) {
        self.stars.push(star);
    }

    /// Add a planet to the scene
    pub fn add_planet(&mut self, planet: RenderablePlanet) {
        self.planets.push(planet);
    }

    /// Add a system to the scene
    pub fn add_system(&mut self, system: RenderableSystem) {
        self.systems.push(system);
    }

    /// Get all renderable objects visible at a given scale
    pub fn get_visible_objects(&self, scale: ScaleLevel) -> Vec<RenderableObject> {
        let mut objects = Vec::new();

        // Add visible entities
        for entity in &self.entities {
            if entity.is_visible_at(scale) {
                objects.push(RenderableObject::Entity(entity.clone()));
            }
        }

        // Add visible stars
        for star in &self.stars {
            if star.is_visible_at(scale) {
                objects.push(RenderableObject::Star(star.clone()));
            }
        }

        // Add visible planets
        for planet in &self.planets {
            if planet.is_visible_at(scale) {
                objects.push(RenderableObject::Planet(planet.clone()));
            }
        }

        // Add visible systems
        for system in &self.systems {
            if system.is_visible_at(scale) {
                objects.push(RenderableObject::System(system.clone()));
            }
        }

        // Sort by render priority (descending)
        objects.sort_by_key(|b| std::cmp::Reverse(b.render_priority()));

        objects
    }

    /// Clear all objects from the scene
    pub fn clear(&mut self) {
        self.entities.clear();
        self.stars.clear();
        self.planets.clear();
        self.systems.clear();
    }
}

/// Enum for all renderable object types
#[derive(Debug, Clone)]
pub enum RenderableObject {
    Entity(RenderableEntity),
    Star(RenderableStar),
    Planet(RenderablePlanet),
    System(RenderableSystem),
}

impl Renderable for RenderableObject {
    fn position(&self) -> Coordinate3D {
        match self {
            RenderableObject::Entity(e) => e.position(),
            RenderableObject::Star(s) => s.position(),
            RenderableObject::Planet(p) => p.position(),
            RenderableObject::System(sys) => sys.position(),
        }
    }

    fn scale(&self) -> f64 {
        match self {
            RenderableObject::Entity(e) => e.scale(),
            RenderableObject::Star(s) => s.scale(),
            RenderableObject::Planet(p) => p.scale(),
            RenderableObject::System(sys) => sys.scale(),
        }
    }

    fn is_visible_at(&self, scale: ScaleLevel) -> bool {
        match self {
            RenderableObject::Entity(e) => e.is_visible_at(scale),
            RenderableObject::Star(s) => s.is_visible_at(scale),
            RenderableObject::Planet(p) => p.is_visible_at(scale),
            RenderableObject::System(sys) => sys.is_visible_at(scale),
        }
    }

    fn render_priority(&self) -> i32 {
        match self {
            RenderableObject::Entity(e) => e.render_priority(),
            RenderableObject::Star(s) => s.render_priority(),
            RenderableObject::Planet(p) => p.render_priority(),
            RenderableObject::System(sys) => sys.render_priority(),
        }
    }
}

/// Involution state for visualization
#[derive(Debug, Clone)]
pub struct InvolutionState {
    /// Current involution layer (0-7)
    pub current_layer: u8,

    /// Progress through current layer (0.0 to 1.0)
    pub layer_progress: f64,

    /// Total involution progress (0.0 to 1.0)
    pub total_progress: f64,

    /// Active primal distortions
    pub active_distortions: Vec<String>,
}

impl Default for InvolutionState {
    fn default() -> Self {
        InvolutionState {
            current_layer: 0,
            layer_progress: 0.0,
            total_progress: 0.0,
            active_distortions: vec!["Free Will".to_string()],
        }
    }
}

/// Emergence state for visualization
#[derive(Debug, Clone)]
pub struct EmergenceState {
    /// Biological emergence level (0.0 to 1.0)
    pub biological_level: f64,

    /// Noospheric emergence level (0.0 to 1.0)
    pub noospheric_level: f64,

    /// Gaia emergence level (0.0 to 1.0)
    pub gaia_level: f64,

    /// Active emergence events
    pub active_events: Vec<String>,
}

impl Default for EmergenceState {
    fn default() -> Self {
        EmergenceState {
            biological_level: 0.0,
            noospheric_level: 0.0,
            gaia_level: 0.0,
            active_events: Vec::new(),
        }
    }
}

/// Entity render data for rendering
#[derive(Debug, Clone)]
pub struct EntityRenderData {
    pub entity_id: EntityId,
    pub position: Coordinate3D,
    pub scale: f64,
    pub color: [f32; 4],
    pub archetype_colors: Vec<[f32; 4]>,
    pub consciousness_indicator: f32,
    pub polarity_indicator: f32,
}

impl EntityRenderData {
    /// Create new entity render data
    pub fn new(entity_id: EntityId, position: Coordinate3D, scale: f64) -> Self {
        EntityRenderData {
            entity_id,
            position,
            scale,
            color: [1.0, 1.0, 1.0, 1.0], // White by default
            archetype_colors: Vec::new(),
            consciousness_indicator: 0.0,
            polarity_indicator: 0.0,
        }
    }
}

/// WGPU renderer - Actual GPU rendering implementation
#[derive(Debug)]
pub struct WgpuRenderer {
    /// WGPU context
    pub context: Option<WgpuContext>,

    /// Buffer manager
    pub buffer_manager: Option<BufferManager>,

    /// Render pipeline
    pub render_pipeline: Option<EntityRenderPipeline>,

    /// Camera zoom level
    pub zoom: f32,

    /// Camera offset X
    pub offset_x: f32,

    /// Camera offset Y
    pub offset_y: f32,
}

impl WgpuRenderer {
    /// Create a new WGPU renderer
    pub fn new() -> Self {
        WgpuRenderer {
            context: None,
            buffer_manager: None,
            render_pipeline: None,
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Initialize the renderer with a surface
    pub async fn initialize(&mut self, window: Arc<winit::window::Window>) -> Result<(), String> {
        let context = WgpuContext::new(window).await?;

        let buffer_manager = BufferManager::new(
            &context.device,
            context.surface_config.width,
            context.surface_config.height,
        );

        let render_pipeline = EntityRenderPipeline::new(&context.device, &context.surface_config);

        self.context = Some(context);
        self.buffer_manager = Some(buffer_manager);
        self.render_pipeline = Some(render_pipeline);

        Ok(())
    }

    /// Resize the surface
    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(context) = &mut self.context {
            context.resize(width, height);
        }
        if let (Some(buffer_manager), Some(context)) = (&self.buffer_manager, &self.context) {
            buffer_manager.update_uniforms(
                &context.queue,
                width,
                height,
                self.zoom,
                self.offset_x,
                self.offset_y,
            );
        }
    }

    /// Render a frame
    pub fn render_frame(&mut self, _scene: &SceneGraph, _camera: &Camera) -> Result<(), String> {
        if let (Some(context), Some(buffer_manager), Some(render_pipeline)) =
            (&self.context, &self.buffer_manager, &self.render_pipeline)
        {
            let surface = context
                .surface
                .as_ref()
                .ok_or_else(|| "Surface not initialized".to_string())?;
            let output = surface
                .get_current_texture()
                .map_err(|e| format!("Failed to get current texture: {}", e))?;
            let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let bind_group = context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Uniform Bind Group"),
                    layout: &render_pipeline.pipeline.get_bind_group_layout(0),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer_manager.uniform_buffer().as_entire_binding(),
                    }],
                });

            let mut encoder =
                context
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&render_pipeline.pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.set_vertex_buffer(0, buffer_manager.vertex_buffer().slice(..));
                render_pass.set_index_buffer(
                    buffer_manager.index_buffer().slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..buffer_manager.num_indices(), 0, 0..1);
            }

            context.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
        Ok(())
    }

    /// Get WGPU context
    pub fn context(&self) -> Option<&WgpuContext> {
        self.context.as_ref()
    }

    /// Get mutable WGPU context
    pub fn context_mut(&mut self) -> Option<&mut WgpuContext> {
        self.context.as_mut()
    }
}

impl Default for WgpuRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Visualization engine - main entry point for rendering
#[derive(Debug)]
pub struct VisualizationEngine {
    /// WGPU renderer
    renderer: WgpuRenderer,

    /// Multi-scale camera
    camera: MultiScaleCamera,

    /// Scene graph
    scene_graph: SceneGraph,

    /// Configuration
    config: GuiConfig,

    /// Involution state
    involution_state: InvolutionState,

    /// Emergence state
    emergence_state: EmergenceState,

    /// Last frame time
    last_frame_time: Option<f64>,
}

impl VisualizationEngine {
    /// Create a new visualization engine
    pub fn new(config: GuiConfig) -> Self {
        VisualizationEngine {
            renderer: WgpuRenderer::new(),
            camera: MultiScaleCamera::new(),
            scene_graph: SceneGraph::new(),
            config,
            involution_state: InvolutionState::default(),
            emergence_state: EmergenceState::default(),
            last_frame_time: None,
        }
    }

    /// Initialize the engine
    pub async fn initialize(
        &mut self,
        window: Option<Arc<winit::window::Window>>,
    ) -> Result<(), String> {
        if let Some(window) = window {
            self.renderer.initialize(window).await?;
        }

        // Set initial camera zoom
        self.camera.camera.set_zoom_level(self.config.initial_zoom);

        Ok(())
    }

    /// Update the engine
    pub fn update(&mut self, delta_time: f64) {
        // Update camera transition
        self.camera.update_transition(delta_time);

        // Update involution state
        self.update_involution(delta_time);

        // Update emergence state
        self.update_emergence(delta_time);

        self.last_frame_time = Some(delta_time);
    }

    /// Render a frame
    pub fn render_frame(&mut self) -> Result<(), String> {
        self.renderer
            .render_frame(&self.scene_graph, &self.camera.camera)
    }

    /// Set the scale level
    pub fn set_scale(&mut self, scale: ScaleLevel) {
        self.camera.set_scale(scale);
    }

    /// Focus on an entity
    pub fn focus_on_entity(&mut self, entity_id: EntityId, position: Coordinate3D) {
        self.camera.focus_on_entity(entity_id, position);
    }

    /// Render involution state
    pub fn render_involution(&self) -> &InvolutionState {
        &self.involution_state
    }

    /// Render emergence state
    pub fn render_emergence(&self) -> &EmergenceState {
        &self.emergence_state
    }

    /// Get the scene graph
    pub fn scene_graph(&self) -> &SceneGraph {
        &self.scene_graph
    }

    /// Get mutable scene graph
    pub fn scene_graph_mut(&mut self) -> &mut SceneGraph {
        &mut self.scene_graph
    }

    /// Get the camera
    pub fn camera(&self) -> &MultiScaleCamera {
        &self.camera
    }

    /// Get mutable camera
    pub fn camera_mut(&mut self) -> &mut MultiScaleCamera {
        &mut self.camera
    }

    /// Handle window resize
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    /// Update involution state
    fn update_involution(&mut self, delta_time: f64) {
        self.involution_state.layer_progress += delta_time * 0.1;
        if self.involution_state.layer_progress >= 1.0 {
            self.involution_state.layer_progress = 0.0;
            self.involution_state.current_layer = (self.involution_state.current_layer + 1).min(7);
        }

        self.involution_state.total_progress = (self.involution_state.current_layer as f64
            + self.involution_state.layer_progress)
            / 8.0;
    }

    /// Update emergence state
    fn update_emergence(&mut self, delta_time: f64) {
        // Gradually increase emergence levels based on simulation state
        self.emergence_state.biological_level =
            (self.emergence_state.biological_level + delta_time * 0.01).min(1.0);
        self.emergence_state.noospheric_level =
            (self.emergence_state.noospheric_level + delta_time * 0.005).min(1.0);
        self.emergence_state.gaia_level =
            (self.emergence_state.gaia_level + delta_time * 0.003).min(1.0);
    }
}

impl Default for VisualizationEngine {
    fn default() -> Self {
        Self::new(GuiConfig::default())
    }
}

/// Builder for VisualizationEngine
#[derive(Debug, Clone, Default)]
pub struct VisualizationEngineBuilder {
    config: GuiConfig,
}

impl VisualizationEngineBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the configuration
    pub fn with_config(mut self, config: GuiConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the visualization engine
    pub fn build(self) -> VisualizationEngine {
        VisualizationEngine::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::default();
        assert_eq!(camera.position, Coordinate3D::new(0.0, 0.0, 10.0));
        assert_eq!(camera.target, Coordinate3D::origin());
        assert_eq!(camera.fov, 60.0);
        assert_eq!(camera.zoom_level, 1.0e-6);
    }

    #[test]
    fn test_camera_zoom() {
        let mut camera = Camera::default();
        let initial_zoom = camera.zoom_level;

        camera.zoom_in(2.0);
        assert!(camera.zoom_level < initial_zoom);

        camera.zoom_out(2.0);
        assert!((camera.zoom_level - initial_zoom).abs() < 0.001);

        camera.set_zoom_level(1.0e-3);
        assert_eq!(camera.zoom_level, 1.0e-3);
    }

    #[test]
    fn test_camera_look_at() {
        let mut camera = Camera::default();
        let target = Coordinate3D::new(5.0, 3.0, 2.0);
        camera.look_at(target);

        assert_eq!(camera.target, target);
    }

    #[test]
    fn test_multi_scale_camera_default() {
        let camera = MultiScaleCamera::default();
        assert_eq!(camera.current_scale, ScaleLevel::Cellular);
        assert_eq!(camera.target_scale, ScaleLevel::Cellular);
        assert!(!camera.is_transitioning);
    }

    #[test]
    fn test_multi_scale_camera_set_scale() {
        let mut camera = MultiScaleCamera::default();
        camera.set_scale(ScaleLevel::Planetary);

        assert_eq!(camera.target_scale, ScaleLevel::Planetary);
        assert!(camera.is_transitioning);
    }

    #[test]
    fn test_multi_scale_camera_update_transition() {
        let mut camera = MultiScaleCamera::default();
        camera.set_scale(ScaleLevel::Planetary);

        // Update transition to completion
        camera.update_transition(1.0);

        assert_eq!(camera.current_scale, ScaleLevel::Planetary);
        assert!(!camera.is_transitioning);
        assert_eq!(camera.transition_progress, 1.0);
    }

    #[test]
    fn test_renderable_entity() {
        let entity = RenderableEntity {
            entity_id: EntityId::new("1".to_string()),
            position: Coordinate3D::new(1.0, 2.0, 3.0),
            scale: 1.0e-6,
            density: 3,
            polarity: 1,
            consciousness: 0.5,
            archetype_activations: vec![(1, 0.8)],
        };

        assert_eq!(entity.position(), Coordinate3D::new(1.0, 2.0, 3.0));
        assert_eq!(entity.scale(), 1.0e-6);
        assert!(entity.is_visible_at(ScaleLevel::Cellular));
        assert!(!entity.is_visible_at(ScaleLevel::Atomic));
    }

    #[test]
    fn test_scene_graph() {
        let mut scene = SceneGraph::new();

        let entity = RenderableEntity {
            entity_id: EntityId::new("1".to_string()),
            position: Coordinate3D::origin(),
            scale: 1.0e-6,
            density: 3,
            polarity: 1,
            consciousness: 0.5,
            archetype_activations: vec![],
        };

        scene.add_entity(entity);

        assert_eq!(scene.entities.len(), 1);

        let visible = scene.get_visible_objects(ScaleLevel::Cellular);
        assert_eq!(visible.len(), 1);

        scene.clear();
        assert_eq!(scene.entities.len(), 0);
    }

    #[test]
    fn test_involution_state_default() {
        let state = InvolutionState::default();
        assert_eq!(state.current_layer, 0);
        assert_eq!(state.layer_progress, 0.0);
        assert_eq!(state.total_progress, 0.0);
        assert_eq!(state.active_distortions.len(), 1);
        assert_eq!(state.active_distortions[0], "Free Will");
    }

    #[test]
    fn test_emergence_state_default() {
        let state = EmergenceState::default();
        assert_eq!(state.biological_level, 0.0);
        assert_eq!(state.noospheric_level, 0.0);
        assert_eq!(state.gaia_level, 0.0);
        assert!(state.active_events.is_empty());
    }

    #[test]
    fn test_visualization_engine_default() {
        let engine = VisualizationEngine::default();
        assert_eq!(engine.camera.current_scale, ScaleLevel::Cellular);
        assert_eq!(engine.last_frame_time, None);
    }

    #[test]
    fn test_visualization_engine_update() {
        let mut engine = VisualizationEngine::default();
        engine.update(0.1);

        assert!(engine.last_frame_time.is_some());
        assert_eq!(engine.last_frame_time.unwrap(), 0.1);
        assert!(engine.involution_state.layer_progress > 0.0);
    }

    #[test]
    fn test_visualization_engine_set_scale() {
        let mut engine = VisualizationEngine::default();
        engine.set_scale(ScaleLevel::Planetary);

        assert_eq!(engine.camera.target_scale, ScaleLevel::Planetary);
        assert!(engine.camera.is_transitioning);
    }

    #[test]
    fn test_visualization_engine_focus_on_entity() {
        let mut engine = VisualizationEngine::default();
        let entity_id = EntityId::new("1".to_string());
        let position = Coordinate3D::new(5.0, 3.0, 2.0);

        engine.focus_on_entity(entity_id, position);

        assert_eq!(engine.camera.camera.target, position);
    }

    #[test]
    fn test_visualization_engine_builder() {
        let config = GuiConfig::new().with_window_size(1280, 720);
        let engine = VisualizationEngineBuilder::new()
            .with_config(config)
            .build();

        assert_eq!(engine.config.window_width, 1280);
        assert_eq!(engine.config.window_height, 720);
    }

    #[test]
    fn test_entity_render_data() {
        let data = EntityRenderData::new(
            EntityId::new("1".to_string()),
            Coordinate3D::origin(),
            1.0e-6,
        );

        assert_eq!(data.entity_id, EntityId::new("1".to_string()));
        assert_eq!(data.position, Coordinate3D::origin());
        assert_eq!(data.scale, 1.0e-6);
        assert_eq!(data.color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_renderable_star() {
        let star = RenderableStar {
            star_id: 1,
            position: Coordinate3D::origin(),
            radius: 6.96e8, // Solar radius
            temperature: 5778.0,
            luminosity: 3.828e26,
            spectral_type: "G2V".to_string(),
        };

        assert!(star.is_visible_at(ScaleLevel::Stellar));
        assert!(!star.is_visible_at(ScaleLevel::Planetary));
    }

    #[test]
    fn test_renderable_planet() {
        let planet = RenderablePlanet {
            planet_id: 1,
            position: Coordinate3D::new(1.496e11, 0.0, 0.0), // 1 AU
            radius: 6.371e6,                                 // Earth radius
            has_life: true,
            atmosphere_composition: vec![("N2".to_string(), 0.78), ("O2".to_string(), 0.21)],
            ecosystem_health: 0.8,
        };

        assert!(planet.is_visible_at(ScaleLevel::Planetary));
        assert!(!planet.is_visible_at(ScaleLevel::Organism));
        assert_eq!(planet.render_priority(), 50); // Has life
    }

    #[test]
    fn test_renderable_system() {
        let system = RenderableSystem {
            system_id: 1,
            position: Coordinate3D::origin(),
            stars: vec![RenderableStar {
                star_id: 1,
                position: Coordinate3D::origin(),
                radius: 6.96e8,
                temperature: 5778.0,
                luminosity: 3.828e26,
                spectral_type: "G2V".to_string(),
            }],
            planets: vec![RenderablePlanet {
                planet_id: 1,
                position: Coordinate3D::new(1.496e11, 0.0, 0.0),
                radius: 6.371e6,
                has_life: true,
                atmosphere_composition: vec![],
                ecosystem_health: 0.8,
            }],
        };

        assert!(system.is_visible_at(ScaleLevel::Stellar));
        assert!(!system.is_visible_at(ScaleLevel::Planetary));
    }
}
