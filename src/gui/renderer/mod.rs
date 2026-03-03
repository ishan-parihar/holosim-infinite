//! Renderer Module - Actual WGPU rendering implementation
//!
//! This module provides real GPU rendering infrastructure for the Holonic Realms simulation.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 1 Week 1:
//! "Implement actual WGPU initialization and basic entity rendering"
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 8:
//! "Instanced rendering implementation - Refactor entity rendering for instancing, Instance buffers for entity data, Reduce draw calls from 1000+ to ~10"

pub mod buffers;
pub mod instanced_renderer;
pub mod logarithmic_depth;
pub mod pipeline;
pub mod post_process;
pub mod shaders;
pub mod wgpu_context;

pub use buffers::{BufferManager, CircleBuffers, Entity3DInstanceData, InstanceBufferManager};
pub use instanced_renderer::{CameraUniforms, InstancedRenderer, LightUniforms};
pub use logarithmic_depth::{LOD_SHADER, LOGARITHMIC_DEPTH_SHADER};
pub use pipeline::EntityRenderPipeline;
pub use post_process::{
    PostProcessConfig, PostProcessPipeline, VisualEffects, POST_PROCESS_SHADER,
};
pub use wgpu_context::WgpuContext;

// Phase 1: Entity rendering
pub mod entity_instance;
pub mod entity_renderer;

pub use entity_instance::EntityInstance;
pub use entity_renderer::EntityRenderer;

// Phase 4: Hierarchy visualization
pub mod connection_renderer;
pub mod hierarchy_connection;

pub use connection_renderer::ConnectionRenderer;
pub use hierarchy_connection::{generate_connections, ConnectionType, HierarchyConnection};

// Phase B.1: Holographic field volume rendering
pub mod field_visual_bridge;
pub mod holographic_field_renderer;
pub mod volume_texture;

pub use field_visual_bridge::{FieldVisualBridge, PhaseCoherenceEdge};
pub use holographic_field_renderer::{HolographicFieldConfig, HolographicFieldRenderer};
pub use volume_texture::{FieldVolumeData, VolumeDimensions, VolumeFormat, VolumeTexture};

// Phase 3: Cosmos visualization
pub mod cosmos_renderer;

pub use cosmos_renderer::{
    CosmosCameraUniform, CosmosRenderer, FilamentVertex, OrbitVertex, PlanetVertex as CosmosPlanetVertex, StarVertex,
};

// Phase 4: Planet surface visualization
pub mod planet_renderer;

pub use planet_renderer::{
    CloudVertex, PlanetRenderer, PlanetUniform, SettlementVertex, StormVertex,
    TerrainVertex, WaterVertex,
};

// Phase 5: Civilization visualization
pub mod civilization_renderer;

pub use civilization_renderer::{
    BoundaryVertex, CivilizationRenderer, CivilizationUniforms, DensityVertex,
    SettlementVertex as CivSettlementVertex, TradeRouteVertex,
    create_boundary_vertices, create_density_vertices, create_settlement_vertices,
    create_trade_route_vertices,
};
