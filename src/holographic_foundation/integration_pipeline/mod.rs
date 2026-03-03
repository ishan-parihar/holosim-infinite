//! Phase 6: Integration + Visualization Pipeline
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "Integrate all components and create visualizations that reveal the holographic architecture"
//!
//! This module provides:
//! - Field-to-entity extraction pipeline
//! - Entity lifecycle transitions (birth, merge, split, death)
//! - Field visualization components (heatmaps, coherence meters, veil indicators)
//! - Performance optimization for 10,000+ entities
//! - GUI integration bridge
//!
//! Key Deliverables:
//! - Field heatmaps (consciousness density)
//! - Veil transparency indicators
//! - Coherence meters
//! - Entity rendering with orientation/polarity/density
//!
//! Success Criteria:
//! - 60 FPS with 10,000 entities
//! - Smooth transitions in entity lifecycle
//! - Field structures visible in visualization

pub mod entity_lifecycle_transitions;
pub mod extraction_pipeline;
pub mod field_visualization;
pub mod gui_bridge;
pub mod performance_optimizer;

pub use entity_lifecycle_transitions::{
    EntityBirth, EntityDeath, EntityLifecycleTransition, EntityMerge, EntitySplit,
    LifecycleTransitionManager, TransitionType,
};
pub use extraction_pipeline::{
    EntityExtractionConfig, EntityExtractionPipeline, ExtractedEntity, ExtractionResult,
    FieldSnapshot,
};
pub use field_visualization::{
    CoherenceMeter, FieldHeatmap, FieldVisualizationConfig, FieldVisualizationData,
    VeilTransparencyIndicator, VisualizationRenderData,
};
pub use gui_bridge::{FieldToGuiBridge, GuiIntegrationConfig, HolographicGuiState, RenderCommand};
pub use performance_optimizer::{
    EntityBatch, PerformanceConfig, PerformanceMetrics, PerformanceOptimizer, SpatialPartitioning,
    UpdateStrategy,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _config = EntityExtractionConfig::default();
        let _lifecycle_config = entity_lifecycle_transitions::LifecycleTransitionConfig::default();
        let _viz_config = FieldVisualizationConfig::default();
        let _perf_config = PerformanceConfig::default();
    }
}
