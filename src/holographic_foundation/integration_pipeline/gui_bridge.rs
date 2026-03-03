//! GUI Bridge - Connects Holographic Foundation to Visualization System
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "GUI integration with new architecture"
//!
//! This module provides the bridge between:
//! - HolographicFoundation (field-first architecture)
//! - GUI visualization components
//! - Existing simulation components
//!
//! Key responsibilities:
//! - Convert field data to render-ready format
//! - Manage visualization state
//! - Handle user interactions
//! - Coordinate updates between field and visualization

use std::collections::HashMap;

use super::entity_lifecycle_transitions::LifecycleTransitionManager;
use super::extraction_pipeline::{
    EntityExtractionConfig, EntityExtractionPipeline, ExtractionResult,
};
use super::field_visualization::{
    FieldVisualizationConfig, FieldVisualizationData, VisualizationRenderData,
};
use super::performance_optimizer::{PerformanceConfig, PerformanceOptimizer};
use crate::holographic_foundation::field_state::Position3D;
use crate::holographic_foundation::HolographicFoundation;
use crate::types::Float;

/// Configuration for GUI integration
#[derive(Debug, Clone)]
pub struct GuiIntegrationConfig {
    /// Visualization configuration
    pub visualization: FieldVisualizationConfig,
    /// Extraction configuration
    pub extraction: EntityExtractionConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Update interval in frames
    pub update_interval: u64,
    /// Enable real-time updates
    pub real_time: bool,
    /// Show debug overlays
    pub debug_overlays: bool,
}

impl Default for GuiIntegrationConfig {
    fn default() -> Self {
        Self {
            visualization: FieldVisualizationConfig::default(),
            extraction: EntityExtractionConfig::default(),
            performance: PerformanceConfig::default(),
            update_interval: 1,
            real_time: true,
            debug_overlays: false,
        }
    }
}

/// Render command for the visualization system
#[derive(Debug, Clone)]
pub enum RenderCommand {
    /// Update heatmap data
    UpdateHeatmap {
        resolution: usize,
        data: Vec<[f32; 4]>,
    },
    /// Update entity positions
    UpdateEntities {
        positions: Vec<[f32; 3]>,
        colors: Vec<[f32; 4]>,
    },
    /// Update veil indicator
    UpdateVeilIndicator {
        transparency: Float,
        color: [f32; 4],
    },
    /// Update coherence meter
    UpdateCoherenceMeter { coherence: Float, color: [f32; 4] },
    /// Show transition effect
    ShowTransition {
        transition_id: u64,
        position: [f32; 3],
        transition_type: String,
        progress: Float,
    },
    /// Update statistics display
    UpdateStatistics { statistics: HashMap<String, String> },
    /// Set camera focus
    SetCameraFocus { position: [f32; 3], zoom: Float },
    /// Clear visualization
    Clear,
}

/// Holographic GUI state
#[derive(Debug, Clone)]
pub struct HolographicGuiState {
    /// Global coherence
    pub global_coherence: Float,
    /// Entity count
    pub entity_count: usize,
    /// Veil transparency
    pub veil_transparency: Float,
    /// Spectrum position
    pub spectrum_position: Float,
    /// FPS
    pub fps: Float,
    /// Active transitions
    pub active_transitions: usize,
    /// Is performance target met
    pub performance_ok: bool,
    /// Selected entity ID
    pub selected_entity: Option<u64>,
    /// Hovered entity ID
    pub hovered_entity: Option<u64>,
    /// Camera position
    pub camera_position: [Float; 3],
    /// Camera zoom
    pub camera_zoom: Float,
}

impl Default for HolographicGuiState {
    fn default() -> Self {
        Self {
            global_coherence: 0.0,
            entity_count: 0,
            veil_transparency: 0.0,
            spectrum_position: 0.5,
            fps: 60.0,
            active_transitions: 0,
            performance_ok: true,
            selected_entity: None,
            hovered_entity: None,
            camera_position: [0.5, 0.5, 1.0],
            camera_zoom: 1.0,
        }
    }
}

/// Field to GUI bridge
pub struct FieldToGuiBridge {
    /// Configuration
    config: GuiIntegrationConfig,
    /// Extraction pipeline
    extraction_pipeline: EntityExtractionPipeline,
    /// Transition manager
    transition_manager: LifecycleTransitionManager,
    /// Performance optimizer
    performance_optimizer: PerformanceOptimizer,
    /// Visualization data
    visualization_data: FieldVisualizationData,
    /// GUI state
    gui_state: HolographicGuiState,
    /// Current frame
    frame: u64,
    /// Last extraction result
    last_extraction: Option<ExtractionResult>,
    /// Pending render commands
    render_commands: Vec<RenderCommand>,
}

impl FieldToGuiBridge {
    /// Create a new GUI bridge
    pub fn new(config: GuiIntegrationConfig) -> Self {
        let extraction_pipeline = EntityExtractionPipeline::new(config.extraction.clone());
        let transition_manager = LifecycleTransitionManager::new(
            super::entity_lifecycle_transitions::LifecycleTransitionConfig::default(),
        );
        let performance_optimizer = PerformanceOptimizer::new(config.performance.clone());
        let visualization_data = FieldVisualizationData::new(config.visualization.grid_resolution);

        Self {
            config,
            extraction_pipeline,
            transition_manager,
            performance_optimizer,
            visualization_data,
            gui_state: HolographicGuiState::default(),
            frame: 0,
            last_extraction: None,
            render_commands: Vec::new(),
        }
    }

    /// Initialize from holographic foundation
    pub fn initialize(&mut self, foundation: &HolographicFoundation) {
        let field_state = foundation.field_state();
        let timestamp = 0;

        let extraction = self.extraction_pipeline.extract(field_state, timestamp);
        self.last_extraction = Some(extraction.clone());

        for entity in &extraction.entities {
            self.performance_optimizer.register_entity(
                entity.entity_id,
                &entity.position,
                entity.coherence,
            );
        }

        self.visualization_data
            .update(field_state, &extraction.field_snapshot, timestamp);

        self.gui_state.global_coherence = extraction.field_snapshot.global_coherence;
        self.gui_state.entity_count = extraction.entities.len();
        self.gui_state.veil_transparency = extraction.field_snapshot.avg_veil_transparency;
        self.gui_state.spectrum_position = extraction.field_snapshot.avg_spectrum_position;

        self.generate_render_commands();
    }

    /// Update from holographic foundation
    pub fn update(&mut self, foundation: &mut HolographicFoundation, dt: Float) {
        self.frame += 1;
        self.performance_optimizer.begin_frame();

        let update_start = std::time::Instant::now();

        if self.frame % self.config.update_interval == 0 {
            foundation.evolve(dt);

            let field_state = foundation.field_state();
            let extraction = self.extraction_pipeline.extract(field_state, self.frame);
            self.last_extraction = Some(extraction.clone());

            self.update_entity_registry(&extraction);

            let completed = self.transition_manager.update(self.frame);
            for transition_id in completed {
                self.render_commands.push(RenderCommand::ShowTransition {
                    transition_id,
                    position: [0.5, 0.5, 0.5],
                    transition_type: "completed".to_string(),
                    progress: 1.0,
                });
            }

            self.visualization_data
                .update(field_state, &extraction.field_snapshot, self.frame);

            self.gui_state.global_coherence = extraction.field_snapshot.global_coherence;
            self.gui_state.entity_count = extraction.entities.len();
            self.gui_state.veil_transparency = extraction.field_snapshot.avg_veil_transparency;
            self.gui_state.spectrum_position = extraction.field_snapshot.avg_spectrum_position;
            self.gui_state.active_transitions = self.transition_manager.active_transitions().len();

            self.generate_render_commands();
        }

        let update_time = update_start.elapsed().as_micros() as u64;
        self.performance_optimizer.end_frame(update_time, 0);

        self.gui_state.fps = self.performance_optimizer.metrics().fps;
        self.gui_state.performance_ok = self.performance_optimizer.is_performance_target_met();
    }

    /// Update entity registry with new extraction
    fn update_entity_registry(&mut self, extraction: &ExtractionResult) {
        let existing_ids: std::collections::HashSet<u64> =
            extraction.entities.iter().map(|e| e.entity_id).collect();

        let current_ids: std::collections::HashSet<u64> = self
            .performance_optimizer
            .query_radius(&Position3D::new(0.0, 0.0, 0.0), 2.0)
            .into_iter()
            .collect();

        for entity in &extraction.entities {
            if !current_ids.contains(&entity.entity_id) {
                self.performance_optimizer.register_entity(
                    entity.entity_id,
                    &entity.position,
                    entity.coherence,
                );
            } else {
                self.performance_optimizer
                    .update_entity_position(entity.entity_id, &entity.position);
            }
        }
    }

    /// Generate render commands from current state
    fn generate_render_commands(&mut self) {
        let render_data = VisualizationRenderData::from_visualization(
            &self.visualization_data,
            &self.config.visualization,
        );

        self.render_commands.push(RenderCommand::UpdateHeatmap {
            resolution: self.config.visualization.grid_resolution,
            data: render_data.heatmap_colors,
        });

        self.render_commands
            .push(RenderCommand::UpdateVeilIndicator {
                transparency: self.gui_state.veil_transparency,
                color: render_data.veil_color,
            });

        self.render_commands
            .push(RenderCommand::UpdateCoherenceMeter {
                coherence: self.gui_state.global_coherence,
                color: render_data.coherence_color,
            });

        self.render_commands.push(RenderCommand::UpdateStatistics {
            statistics: render_data.statistics_text,
        });
    }

    /// Get pending render commands
    pub fn take_render_commands(&mut self) -> Vec<RenderCommand> {
        std::mem::take(&mut self.render_commands)
    }

    /// Get current GUI state
    pub fn gui_state(&self) -> &HolographicGuiState {
        &self.gui_state
    }

    /// Get mutable GUI state
    pub fn gui_state_mut(&mut self) -> &mut HolographicGuiState {
        &mut self.gui_state
    }

    /// Handle entity selection
    pub fn select_entity(&mut self, entity_id: Option<u64>) {
        self.gui_state.selected_entity = entity_id;

        if let Some(id) = entity_id {
            if let Some(extraction) = &self.last_extraction {
                if let Some(entity) = extraction.entities.iter().find(|e| e.entity_id == id) {
                    self.render_commands.push(RenderCommand::SetCameraFocus {
                        position: [
                            entity.position.x as f32,
                            entity.position.y as f32,
                            entity.position.z as f32,
                        ],
                        zoom: 0.1,
                    });
                }
            }
        }
    }

    /// Handle camera movement
    pub fn move_camera(&mut self, delta: [Float; 3]) {
        self.gui_state.camera_position[0] += delta[0];
        self.gui_state.camera_position[1] += delta[1];
        self.gui_state.camera_position[2] += delta[2];

        self.gui_state.camera_position[0] = self.gui_state.camera_position[0].clamp(0.0, 1.0);
        self.gui_state.camera_position[1] = self.gui_state.camera_position[1].clamp(0.0, 1.0);
        self.gui_state.camera_position[2] = self.gui_state.camera_position[2].clamp(0.0, 1.0);
    }

    /// Handle zoom
    pub fn zoom(&mut self, factor: Float) {
        self.gui_state.camera_zoom *= factor;
        self.gui_state.camera_zoom = self.gui_state.camera_zoom.clamp(0.01, 100.0);
    }

    /// Get last extraction result
    pub fn last_extraction(&self) -> Option<&ExtractionResult> {
        self.last_extraction.as_ref()
    }

    /// Get visualization data
    pub fn visualization_data(&self) -> &FieldVisualizationData {
        &self.visualization_data
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &super::performance_optimizer::PerformanceMetrics {
        self.performance_optimizer.metrics()
    }

    /// Query entities near a position
    pub fn query_entities_near(&self, position: &Position3D, radius: Float) -> Vec<u64> {
        self.performance_optimizer.query_radius(position, radius)
    }

    /// Get entity details
    pub fn get_entity_details(&self, entity_id: u64) -> Option<EntityDetails> {
        let extraction = self.last_extraction.as_ref()?;
        let entity = extraction
            .entities
            .iter()
            .find(|e| e.entity_id == entity_id)?;

        Some(EntityDetails {
            entity_id: entity.entity_id,
            position: entity.position,
            coherence: entity.coherence,
            amplitude: entity.amplitude,
            density: format!("{:?}", entity.density),
            dominant_archetype: entity.dominant_archetype,
            archetype_vector: entity.archetype_vector,
        })
    }

    /// Enable/disable debug overlays
    pub fn set_debug_overlays(&mut self, enabled: bool) {
        self.config.debug_overlays = enabled;
    }

    /// Set real-time mode
    pub fn set_real_time(&mut self, enabled: bool) {
        self.config.real_time = enabled;
    }
}

/// Details about a specific entity
#[derive(Debug, Clone)]
pub struct EntityDetails {
    pub entity_id: u64,
    pub position: Position3D,
    pub coherence: Float,
    pub amplitude: Float,
    pub density: String,
    pub dominant_archetype: usize,
    pub archetype_vector: [Float; 22],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gui_integration_config_default() {
        let config = GuiIntegrationConfig::default();
        assert_eq!(config.update_interval, 1);
        assert!(config.real_time);
    }

    #[test]
    fn test_holographic_gui_state_default() {
        let state = HolographicGuiState::default();
        assert_eq!(state.global_coherence, 0.0);
        assert_eq!(state.fps, 60.0);
    }

    #[test]
    fn test_field_to_gui_bridge_creation() {
        let config = GuiIntegrationConfig::default();
        let bridge = FieldToGuiBridge::new(config);
        assert_eq!(bridge.frame, 0);
    }

    #[test]
    fn test_bridge_initialize() {
        let config = GuiIntegrationConfig::default();
        let mut bridge = FieldToGuiBridge::new(config);
        let foundation = HolographicFoundation::unity();

        bridge.initialize(&foundation);

        assert!(bridge.gui_state.entity_count >= 0);
        assert!(bridge.gui_state.global_coherence > 0.0);
    }

    #[test]
    fn test_bridge_render_commands() {
        let config = GuiIntegrationConfig::default();
        let mut bridge = FieldToGuiBridge::new(config);
        let foundation = HolographicFoundation::unity();

        bridge.initialize(&foundation);
        let commands = bridge.take_render_commands();

        assert!(!commands.is_empty());
    }

    #[test]
    fn test_bridge_select_entity() {
        let config = GuiIntegrationConfig::default();
        let mut bridge = FieldToGuiBridge::new(config);

        bridge.select_entity(Some(42));
        assert_eq!(bridge.gui_state.selected_entity, Some(42));
    }

    #[test]
    fn test_bridge_camera_movement() {
        let config = GuiIntegrationConfig::default();
        let mut bridge = FieldToGuiBridge::new(config);

        bridge.move_camera([0.1, 0.1, 0.0]);
        assert!(bridge.gui_state.camera_position[0] > 0.5);
    }

    #[test]
    fn test_bridge_zoom() {
        let config = GuiIntegrationConfig::default();
        let mut bridge = FieldToGuiBridge::new(config);

        bridge.zoom(2.0);
        assert_eq!(bridge.gui_state.camera_zoom, 2.0);
    }
}
