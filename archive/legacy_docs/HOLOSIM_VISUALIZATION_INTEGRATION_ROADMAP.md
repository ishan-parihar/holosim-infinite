# HOLOSIM INFINITE: VISUALIZATION INTEGRATION ROADMAP

> **Document Purpose:** This roadmap provides complete implementation instructions for connecting the deep simulation architecture to the GUI visualization layer. Future AI agents should use this document to understand the architecture gap and implement the required changes.

> **Last Updated:** March 2, 2026

---

## EXECUTIVE SUMMARY

### The Problem

The codebase contains **two completely separate architectures** that are not connected:

1. **Deep Simulation (CLI)** - `SimulationRunner` with `CosmosEngine`, `Planet` systems, `LivingEnvironment`
2. **GUI Visualization** - `IntegratedSystem` with simplified `HolographicSimulation` showing colored circles

**Result:** GUI shows blank/purple screen with abstract entities, while the full universe/galaxy/planet simulation runs disconnected in CLI mode.

### The Solution

Connect `SimulationRunner` to GUI, implement multi-scale visualization (Universe → Galaxy → Solar System → Planet → Civilization → Individual), and create the missing civilization/population layer.

---

## ARCHITECTURE ANALYSIS

### Current State: Disconnected Systems

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        CLI SIMULATION (main.rs)                              │
│                                                                              │
│  SimulationRunner                                                            │
│  ├── InvolutionSequenceRunner (creates entities via involution)             │
│  ├── EntityLifecycleManager (entity evolution)                              │
│  ├── HolographicFieldManager (field dynamics)                               │
│  ├── PhysicalAdapter (scale transitions)                                    │
│  ├── CollectiveDynamicsManager (social memory complexes)                    │
│  ├── EmergenceManager (emergent behavior)                                   │
│  ├── CatalystManager (catalyst events)                                      │
│  ├── CausalInversionRunner (time/space inversion)                           │
│  ├── InterScaleInteractionManager (quantum↔cosmic)                          │
│  ├── EnvironmentalInteractionManager                                        │
│  ├── LivingEnvironment                                                      │
│  │   ├── Lithosphere (tectonics, volcanoes, terrain)                       │
│  │   ├── Hydrosphere (oceans, rivers, water cycle)                         │
│  │   ├── DynamicAtmosphere (weather, storms, climate)                      │
│  │   └── EnergyFlowSystem                                                   │
│  └── CosmosEngine                                                           │
│      ├── CosmicWeb (galaxy filaments, voids)                               │
│      ├── ProtoStellarRegion (star formation)                               │
│      ├── StellarSystem (stars with planets)                                │
│      └── Planet (full planetary dynamics)                                   │
│                                                                              │
│  STATUS: ✅ Fully Implemented, ❌ No Visualization                           │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ NOT CONNECTED
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                     GUI VISUALIZATION (holonic_gui_complete)                 │
│                                                                              │
│  IntegratedSystem                                                            │
│  ├── HpoSystem (parameter optimization)                                     │
│  ├── HolographicSimulation (simplified, 137 hardcoded entities)             │
│  │   ├── SpatialField (abstract positions)                                  │
│  │   ├── SpectrumSpatialDynamics                                            │
│  │   └── get_entities() → 85 RenderableEntity                               │
│  └── EntityRenderer                                                          │
│      └── Renders circles with realm colors                                  │
│                                                                              │
│  NOT CONNECTED:                                                              │
│  ❌ SimulationRunner                                                         │
│  ❌ CosmosEngine (universe/galaxy/star/planet structures)                   │
│  ❌ LivingEnvironment (lithosphere/hydrosphere/atmosphere)                  │
│  ❌ CollectiveDynamicsManager                                                │
│  ❌ Civilization/Population systems (NOT IMPLEMENTED)                       │
│                                                                              │
│  STATUS: ✅ Runs, ❌ Shows only abstract circles                            │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why Blank Purple Screen

1. **Initial zoom: `1e-6`** (extremely zoomed out - entities invisible)
   - File: `src/gui/application.rs:4296`
   - Fixed: Changed to `1.0`

2. **`holo_entities()` returns only 85 entities** from simplified `HolographicSimulation`
   - File: `src/integrated_system.rs:724`
   - These are abstract consciousness units, not cosmic structures

3. **No universe/galaxy/planet structures visualized**
   - `CosmosEngine` exists but not connected to GUI
   - File: `src/cosmos/cosmos_engine.rs`

4. **Entity positions in wrong scale**
   - Derived from abstract `SpatialField`, not cosmic coordinates
   - File: `src/hpo/holographic_simulation.rs:624`

---

## IMPLEMENTED BUT NOT VISUALIZED

| System | File | Lines | Status |
|--------|------|-------|--------|
| **CosmosEngine** | `src/cosmos/cosmos_engine.rs` | 569 | ✅ Implemented, ❌ Not visualized |
| **CosmicWeb** | `src/cosmos/cosmic_web.rs` | - | ✅ Implemented, ❌ Not visualized |
| **StellarPhysics** | `src/cosmos/stellar_physics.rs` | - | ✅ Implemented, ❌ Not visualized |
| **PlanetaryFormation** | `src/cosmos/planetary_formation.rs` | - | ✅ Implemented, ❌ Not visualized |
| **OrbitalMechanics** | `src/cosmos/orbital_mechanics.rs` | - | ✅ Implemented, ❌ Not visualized |
| **Lithosphere** | `src/planet/lithosphere.rs` | 780+ | ✅ Implemented, ❌ Not visualized |
| **Hydrosphere** | `src/planet/hydrosphere.rs` | 660+ | ✅ Implemented, ❌ Not visualized |
| **DynamicAtmosphere** | `src/planet/atmosphere.rs` | 710+ | ✅ Implemented, ❌ Not visualized |
| **EnergyFlowSystem** | `src/planet/energy_flow.rs` | - | ✅ Implemented, ❌ Not visualized |
| **LivingEnvironment** | `src/simulation_v3/living_environment.rs` | 1020+ | ✅ Implemented, ❌ Not visualized |
| **CollectiveDynamics** | `src/simulation_v3/collective_dynamics.rs` | - | ✅ Implemented, ❌ Not visualized |
| **Civilizations** | NOT IMPLEMENTED | - | ❌ Missing |
| **Populations** | NOT IMPLEMENTED | - | ❌ Missing |
| **Individuals** | Partial (`SubSubLogos`) | - | ⚠️ Abstract only |

---

## PHASE 1: FIX IMMEDIATE VISIBILITY

### 1.1 Camera Zoom Calibration

**File:** `src/gui/application.rs`

**Location:** Line ~4296 in `GuiConfig::default()`

**Change:**
```rust
// BEFORE:
initial_zoom: 1e-6,

// AFTER:
initial_zoom: 1.0,  // Fixed: was 1e-6 making entities invisible
```

**Status:** ✅ Completed

---

## PHASE 2: CONNECT DEEP SIMULATION TO GUI

### 2.1 Create SimulationRunnerAdapter

**New File:** `src/gui/simulation_adapter.rs`

```rust
//! SimulationRunnerAdapter - Bridges SimulationRunner to GUI rendering
//!
//! This module adapts the deep simulation (SimulationRunner) to the
//! visualization layer (EntityRenderer, ConnectionRenderer).
//!
//! ARCHITECTURE:
//! Before: IntegratedSystem → HolographicSimulation (simplified, 137 entities)
//! After:  SimulationRunnerAdapter → SimulationRunner (full simulation)
//!
//! PURPOSE:
//! - Replace the simplified HolographicSimulation with full SimulationRunner
//! - Stream entity data from deep simulation to renderers
//! - Provide multi-scale data access (universe, galaxy, planet, civilization)

use crate::simulation_v3::simulation_runner::{SimulationRunner, SimulationParameters, SimulationResult};
use crate::gui::renderer::entity_instance::EntityInstance;
use crate::gui::renderer::connection_renderer::ConnectionData;
use crate::hpo::RenderableEntity;
use crate::cosmos::CosmosEngine;
use crate::planet::Planet;

/// Adapter that connects SimulationRunner to GUI rendering
pub struct SimulationRunnerAdapter {
    /// The actual simulation runner
    runner: SimulationRunner,
    
    /// Cached entity instances for rendering
    cached_entities: Vec<EntityInstance>,
    
    /// Cached connection data for hierarchy visualization
    cached_connections: Vec<ConnectionData>,
    
    /// Last simulation step that was rendered
    last_update_step: u64,
    
    /// Is the simulation currently running?
    is_running: bool,
}

impl SimulationRunnerAdapter {
    /// Create a new adapter with default simulation parameters
    pub fn new() -> Self {
        let params = SimulationParameters {
            num_entities: 1000,
            num_steps: 100000,
            run_involution: true,
            run_evolution: true,
            update_holographic_field: true,
            update_physical_manifestations: true,
            generate_detailed_reports: false,
            ..Default::default()
        };
        
        Self {
            runner: SimulationRunner::new(params),
            cached_entities: Vec::new(),
            cached_connections: Vec::new(),
            last_update_step: 0,
            is_running: false,
        }
    }
    
    /// Create adapter with custom parameters
    pub fn with_params(params: SimulationParameters) -> Self {
        Self {
            runner: SimulationRunner::new(params),
            cached_entities: Vec::new(),
            cached_connections: Vec::new(),
            last_update_step: 0,
            is_running: false,
        }
    }
    
    /// Initialize the simulation (run involution)
    pub fn initialize(&mut self) {
        // Run involution to create initial entities
        // This populates the simulation with entities
    }
    
    /// Step simulation forward by one tick
    pub fn step(&mut self) {
        self.runner.step();
        self.update_caches();
    }
    
    /// Run simulation for multiple steps
    pub fn run_steps(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step();
        }
    }
    
    /// Check if simulation is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    /// Get current simulation step
    pub fn current_step(&self) -> u64 {
        self.runner.current_step()
    }
    
    /// Get entities for EntityRenderer
    pub fn get_entities(&self) -> &[EntityInstance] {
        &self.cached_entities
    }
    
    /// Get connections for ConnectionRenderer
    pub fn get_connections(&self) -> &[ConnectionData] {
        &self.cached_connections
    }
    
    /// Get RenderableEntity format for compatibility
    pub fn get_renderable_entities(&self) -> Vec<RenderableEntity> {
        // Convert EntityInstance to RenderableEntity
        self.cached_entities.iter().map(|e| e.to_renderable()).collect()
    }
    
    /// Get cosmic data for CosmosRenderer
    pub fn get_cosmic_data(&self) -> Option<CosmicRenderData> {
        // Extract from SimulationRunner -> CosmosEngine
        // Return stellar systems, planets, cosmic web data
        None // TODO: Implement after connecting to SimulationRunner
    }
    
    /// Get planet data for PlanetRenderer
    pub fn get_planet_data(&self, planet_id: u64) -> Option<PlanetRenderData> {
        // Extract from SimulationRunner -> LivingEnvironment -> Planet
        None // TODO: Implement
    }
    
    /// Get civilization data for CivilizationRenderer
    pub fn get_civilization_data(&self) -> Option<CivilizationRenderData> {
        // Extract from SimulationRunner -> CivilizationSystem
        None // TODO: Implement after civilization system is created
    }
    
    /// Update cached data from simulation
    fn update_caches(&mut self) {
        let current_step = self.runner.current_step();
        
        // Only update if simulation has advanced
        if current_step == self.last_update_step {
            return;
        }
        
        // Extract entity data from SimulationRunner
        self.cached_entities = self.extract_entity_instances();
        
        // Extract connection data from collective dynamics
        self.cached_connections = self.extract_connections();
        
        self.last_update_step = current_step;
    }
    
    /// Extract entity instances from simulation
    fn extract_entity_instances(&self) -> Vec<EntityInstance> {
        // TODO: Connect to SimulationRunner's entity data
        // Source: SimulationRunner -> EntityLifecycleManager -> entities
        Vec::new()
    }
    
    /// Extract connection data from collective dynamics
    fn extract_connections(&self) -> Vec<ConnectionData> {
        // TODO: Connect to SimulationRunner's collective dynamics
        // Source: SimulationRunner -> CollectiveDynamicsManager -> connections
        Vec::new()
    }
}

impl Default for SimulationRunnerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Data structure for cosmic rendering
#[derive(Debug, Clone)]
pub struct CosmicRenderData {
    /// Cosmic web filaments
    pub filaments: Vec<CosmicFilament>,
    
    /// Stellar systems in view
    pub stellar_systems: Vec<StellarSystemData>,
    
    /// Selected star for detail view
    pub selected_star: Option<u64>,
    
    /// Selected planet for detail view
    pub selected_planet: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct CosmicFilament {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub density: f64,
}

#[derive(Debug, Clone)]
pub struct StellarSystemData {
    pub id: u64,
    pub star_position: [f64; 3],
    pub star_color: [f32; 3],
    pub star_luminosity: f64,
    pub planets: Vec<PlanetOrbitData>,
}

#[derive(Debug, Clone)]
pub struct PlanetOrbitData {
    pub id: u64,
    pub orbit_radius: f64,
    pub orbit_period: f64,
    pub current_angle: f64,
    pub planet_type: String,
}

/// Data structure for planet surface rendering
#[derive(Debug, Clone)]
pub struct PlanetRenderData {
    pub planet_id: u64,
    
    /// Terrain heightmap
    pub terrain: Vec<Vec<f64>>,
    
    /// Water depth map
    pub water_depth: Vec<Vec<f64>>,
    
    /// Cloud coverage
    pub cloud_coverage: Vec<Vec<f64>>,
    
    /// Storm positions
    pub storms: Vec<StormData>,
    
    /// Settlements on surface
    pub settlements: Vec<SettlementData>,
}

#[derive(Debug, Clone)]
pub struct StormData {
    pub position: (f64, f64),
    pub intensity: f64,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct SettlementData {
    pub position: (f64, f64),
    pub population: u64,
    pub settlement_type: String,
}

/// Data structure for civilization rendering
#[derive(Debug, Clone)]
pub struct CivilizationRenderData {
    pub civilizations: Vec<CivilizationSummary>,
    pub trade_routes: Vec<TradeRoute>,
    pub population_density: Vec<PopulationDensity>,
}

#[derive(Debug, Clone)]
pub struct CivilizationSummary {
    pub id: u64,
    pub name: String,
    pub population: u64,
    pub tech_level: f64,
    pub polarization: f64,
}

#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub from_settlement: u64,
    pub to_settlement: u64,
    pub volume: f64,
}

#[derive(Debug, Clone)]
pub struct PopulationDensity {
    pub position: (f64, f64),
    pub density: f64,
}
```

### 2.2 Modify GuiApplication

**File:** `src/gui/application.rs`

**Changes Required:**

```rust
// ============================================================================
// AT THE TOP OF THE FILE
// ============================================================================

// REPLACE:
// use crate::integrated_system::IntegratedSystem;

// WITH:
use crate::gui::simulation_adapter::SimulationRunnerAdapter;

// ============================================================================
// IN GuiApplication STRUCT
// ============================================================================

pub struct GuiApplication {
    // ... existing fields ...
    
    // REMOVE:
    // simulation: IntegratedSystem,
    
    // ADD:
    simulation: SimulationRunnerAdapter,
    
    // ADD: Scale navigation
    current_scale: ScaleLevel,
    selected_cosmic_object: Option<CosmicObjectId>,
}

// ============================================================================
// ADD SCALE LEVEL ENUM
// ============================================================================

/// Navigation scale levels for multi-scale visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleLevel {
    /// View cosmic web, galaxy clusters (10^26 m)
    Universe,
    
    /// View stellar systems within a galaxy (10^21 m)
    Galaxy,
    
    /// View star and planets (10^13 m)
    SolarSystem,
    
    /// View planet surface (10^7 m)
    Planet,
    
    /// View cities, populations (10^5 m)
    Civilization,
    
    /// View individual entities (10^0 m)
    Individual,
}

impl Default for ScaleLevel {
    fn default() -> Self {
        ScaleLevel::Planet  // Start at planet scale for best experience
    }
}

/// Identifier for selected cosmic object
#[derive(Debug, Clone, Copy)]
pub enum CosmicObjectId {
    Galaxy(u64),
    Star(u64),
    Planet(u64),
    Civilization(u64),
    Settlement(u64),
    Individual(u64),
}

// ============================================================================
// IN GuiApplication::new()
// ============================================================================

// REPLACE IntegratedSystem::new() with SimulationRunnerAdapter::new()

// ============================================================================
// IN render() METHOD
// ============================================================================

fn render(&mut self) -> Result<(), GuiError> {
    // Get entities from SimulationRunnerAdapter
    let entities = self.simulation.get_entities();
    
    // Update renderer
    if let Some(ref mut renderer) = self.entity_renderer {
        renderer.update_entities(&ctx.queue, entities);
    }
    
    // Render based on current scale
    match self.current_scale {
        ScaleLevel::Universe => self.render_universe_view(&mut encoder, &view),
        ScaleLevel::Galaxy => self.render_galaxy_view(&mut encoder, &view),
        ScaleLevel::SolarSystem => self.render_solar_system_view(&mut encoder, &view),
        ScaleLevel::Planet => self.render_planet_view(&mut encoder, &view),
        ScaleLevel::Civilization => self.render_civilization_view(&mut encoder, &view),
        ScaleLevel::Individual => self.render_individual_view(&mut encoder, &view),
    }
    
    Ok(())
}
```

---

## PHASE 3: COSMOS VISUALIZATION

### 3.1 Create CosmosRenderer

**New File:** `src/gui/renderer/cosmos_renderer.rs`

```rust
//! CosmosRenderer - Visualizes universe, galaxies, stars, planets
//!
//! Renders the cosmic hierarchy at multiple scales:
//! - Universe scale: Cosmic web filaments, voids, galaxy clusters
//! - Galaxy scale: Spiral arms, globular clusters, nebulae
//! - Solar system scale: Star with planetary orbits
//! - Planet scale: Surface, atmosphere, clouds
//!
//! CONNECTS TO:
//! - CosmosEngine (src/cosmos/cosmos_engine.rs)
//! - StellarPhysics (src/cosmos/stellar_physics.rs)
//! - PlanetaryFormation (src/cosmos/planetary_formation.rs)

use wgpu::*;
use crate::cosmos::{CosmosEngine, CosmicWeb, StellarSystem, Star};
use crate::planet::Planet;

pub struct CosmosRenderer {
    device: Device,
    
    // Pipeline for cosmic web filaments
    filament_pipeline: RenderPipeline,
    filament_buffer: Buffer,
    
    // Pipeline for stars (point sprites)
    star_pipeline: RenderPipeline,
    star_buffer: Buffer,
    
    // Pipeline for planets (spheres)
    planet_pipeline: RenderPipeline,
    planet_buffer: Buffer,
    
    // Pipeline for orbital paths
    orbit_pipeline: RenderPipeline,
    orbit_buffer: Buffer,
    
    // Bind groups
    camera_bind_group: BindGroup,
    camera_buffer: Buffer,
}

impl CosmosRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        // Load cosmos shader
        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Cosmos Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/cosmos.wgsl").into()),
        });
        
        // Create camera bind group layout
        let camera_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Cosmos Camera Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(64),
                    },
                    count: None,
                },
            ],
        });
        
        // Create pipelines for each render type
        let filament_pipeline = Self::create_filament_pipeline(device, &camera_layout, &shader, surface_config);
        let star_pipeline = Self::create_star_pipeline(device, &camera_layout, &shader, surface_config);
        let planet_pipeline = Self::create_planet_pipeline(device, &camera_layout, &shader, surface_config);
        let orbit_pipeline = Self::create_orbit_pipeline(device, &camera_layout, &shader, surface_config);
        
        // Create buffers
        let camera_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Cosmos Camera Buffer"),
            size: 64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let filament_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Filament Buffer"),
            size: 1024 * 1024,  // 1MB for filament data
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let star_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Star Buffer"),
            size: 1024 * 1024,  // 1MB for star data
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let planet_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Planet Buffer"),
            size: 1024 * 1024,  // 1MB for planet data
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let orbit_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Orbit Buffer"),
            size: 1024 * 1024,  // 1MB for orbit data
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Cosmos Camera Bind Group"),
            layout: &camera_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
            ],
        });
        
        Self {
            device: device.clone(),
            filament_pipeline,
            star_pipeline,
            planet_pipeline,
            orbit_pipeline,
            filament_buffer,
            star_buffer,
            planet_buffer,
            orbit_buffer,
            camera_bind_group,
            camera_buffer,
        }
    }
    
    /// Update cosmic web data
    pub fn update_cosmic_web(&mut self, queue: &Queue, web: &CosmicWeb) {
        // Convert CosmicWeb to vertex data
        // Update filament_buffer with filament positions
    }
    
    /// Update stellar system data
    pub fn update_stellar_system(&mut self, queue: &Queue, system: &StellarSystem) {
        // Update star_buffer with star position, color, luminosity
        // Update planet_buffer with planet positions and sizes
        // Update orbit_buffer with orbital path vertices
    }
    
    /// Render cosmic web (universe scale)
    pub fn render_cosmic_web(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Cosmic Web Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
        });
        
        render_pass.set_pipeline(&self.filament_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.filament_buffer.slice(..));
        render_pass.draw(0..1000, 0..1);  // Draw filaments
    }
    
    /// Render stellar system (galaxy/solar scale)
    pub fn render_stellar_system(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Render orbital paths first (behind planets)
        // Render star (central point with glow)
        // Render planets at current positions
    }
    
    /// Update camera uniform
    pub fn update_camera(&self, queue: &Queue, view_proj: [[f32; 4]; 4]) {
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&view_proj));
    }
    
    fn create_filament_pipeline(
        device: &Device,
        layout: &BindGroupLayout,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        // Create pipeline for rendering cosmic web filaments
        // Lines with varying thickness based on density
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Filament Pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Filament Pipeline Layout"),
                bind_group_layouts: &[layout],
                push_constant_ranges: &[],
            })),
            vertex: VertexState {
                module: shader,
                entry_point: "vs_filament",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: shader,
                entry_point: "fs_filament",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }
    
    fn create_star_pipeline(
        device: &Device,
        layout: &BindGroupLayout,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        // Create pipeline for rendering stars as point sprites
        // With glow effect based on luminosity
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Star Pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Star Pipeline Layout"),
                bind_group_layouts: &[layout],
                push_constant_ranges: &[],
            })),
            vertex: VertexState {
                module: shader,
                entry_point: "vs_star",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: shader,
                entry_point: "fs_star",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ADDITIVE_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::PointList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }
    
    fn create_planet_pipeline(
        device: &Device,
        layout: &BindGroupLayout,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        // Create pipeline for rendering planets as spheres
        // With textures and lighting from star
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Planet Pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Planet Pipeline Layout"),
                bind_group_layouts: &[layout],
                push_constant_ranges: &[],
            })),
            vertex: VertexState {
                module: shader,
                entry_point: "vs_planet",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: shader,
                entry_point: "fs_planet",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }
    
    fn create_orbit_pipeline(
        device: &Device,
        layout: &BindGroupLayout,
        shader: &ShaderModule,
        config: &SurfaceConfiguration,
    ) -> RenderPipeline {
        // Create pipeline for rendering orbital paths
        // As thin lines
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Orbit Pipeline"),
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Orbit Pipeline Layout"),
                bind_group_layouts: &[layout],
                push_constant_ranges: &[],
            })),
            vertex: VertexState {
                module: shader,
                entry_point: "vs_orbit",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: shader,
                entry_point: "fs_orbit",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::LineStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        })
    }
}
```

### 3.2 Create Cosmos Shader

**New File:** `src/gui/renderer/shaders/cosmos.wgsl`

```rust
//! Cosmos Shader - Renders cosmic structures at multiple scales
//!
//! SCALES:
//! - Universe: Cosmic web filaments, voids, galaxy clusters
//! - Galaxy: Stars, nebulae, spiral arms
//! - Solar System: Central star, planetary orbits, planets

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
    time: f32,
    scale_level: u32,  // 0=Universe, 1=Galaxy, 2=Solar, 3=Planet
    _pad: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// ============================================================================
// FILAMENT RENDERING (Cosmic Web)
// ============================================================================

struct FilamentVertex {
    @location(0) position: vec3<f32>,
    @location(1) density: f32,
    @location(2) color: vec3<f32>,
};

struct FilamentOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) density: f32,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_filament(vertex: FilamentVertex) -> FilamentOutput {
    var output: FilamentOutput;
    output.position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    output.density = vertex.density;
    output.color = vertex.color;
    return output;
}

@fragment
fn fs_filament(input: FilamentOutput) -> @location(0) vec4<f32> {
    // Cosmic web color: dark purple to bright blue based on density
    let base_color = vec3<f32>(0.1, 0.0, 0.2);
    let bright_color = vec3<f32>(0.3, 0.5, 1.0);
    let color = mix(base_color, bright_color, input.density);
    
    // Add subtle animation
    let pulse = sin(camera.time * 0.5 + input.density * 3.14159) * 0.1 + 0.9;
    
    return vec4<f32>(color * pulse, input.density * 0.5);
}

// ============================================================================
// STAR RENDERING (Point Sprites with Glow)
// ============================================================================

struct StarVertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,      // Spectral class color (OBAFGKM)
    @location(2) luminosity: f32,
    @location(3) radius: f32,
};

struct StarOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) luminosity: f32,
};

@vertex
fn vs_star(vertex: StarVertex) -> StarOutput {
    var output: StarOutput;
    
    // Project to clip space
    let clip_pos = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    
    // Scale point size by luminosity and distance
    let distance = length(camera.position - vertex.position);
    let point_size = vertex.radius * vertex.luminosity * 100.0 / max(distance, 0.1);
    
    // Expand to point sprite quad
    output.position = clip_pos;
    output.position.xy += vec2<f32>(
        (input.instance_id % 2u == 0u) ? -point_size : point_size,
        (input.instance_id / 2u % 2u == 0u) ? -point_size : point_size
    ) * 0.01;
    
    output.color = vertex.color;
    output.uv = vec2<f32>(
        f32(input.instance_id % 2u),
        f32(input.instance_id / 2u % 2u)
    ) * 2.0 - 1.0;
    output.luminosity = vertex.luminosity;
    
    return output;
}

@fragment
fn fs_star(input: StarOutput) -> @location(0) vec4<f32> {
    // Distance from center of point sprite
    let dist = length(input.uv);
    
    // Gaussian glow falloff
    let glow = exp(-dist * dist * 4.0);
    
    // Core brightness
    let core = exp(-dist * dist * 20.0);
    
    // Combine glow and core
    let intensity = (glow * 0.5 + core * 1.5) * input.luminosity;
    
    // Add spectral color variation
    let color = input.color * intensity;
    
    // Add twinkling effect
    let twinkle = sin(camera.time * 3.0 + input.uv.x * 10.0) * 0.1 + 0.9;
    
    return vec4<f32>(color * twinkle, glow);
}

// ============================================================================
// PLANET RENDERING (Spheres with Lighting)
// ============================================================================

struct PlanetVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) planet_type: f32,  // 0=rocky, 1=gas, 2=ice, 3=ocean
    @location(4) color_hint: vec3<f32>,
};

struct PlanetOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) planet_type: f32,
    @location(3) color_hint: vec3<f32>,
    @location(4) world_pos: vec3<f32>,
};

@vertex
fn vs_planet(vertex: PlanetVertex) -> PlanetOutput {
    var output: PlanetOutput;
    output.position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    output.normal = vertex.normal;
    output.uv = vertex.uv;
    output.planet_type = vertex.planet_type;
    output.color_hint = vertex.color_hint;
    output.world_pos = vertex.position;
    return output;
}

@fragment
fn fs_planet(input: PlanetOutput) -> @location(0) vec4<f32> {
    // Direction to "sun" (simplified - should come from uniform)
    let light_dir = normalize(vec3<f32>(1.0, 0.5, 0.5));
    
    // Diffuse lighting
    let diffuse = max(0.0, dot(input.normal, light_dir));
    
    // Ambient
    let ambient = 0.1;
    
    // Base color from planet type
    var base_color: vec3<f32>;
    if (input.planet_type < 0.5) {
        // Rocky planet
        base_color = mix(
            vec3<f32>(0.5, 0.4, 0.3),  // Brown
            vec3<f32>(0.7, 0.6, 0.5),  // Tan
            input.uv.y
        );
    } else if (input.planet_type < 1.5) {
        // Gas giant
        base_color = mix(
            vec3<f32>(0.8, 0.7, 0.5),  // Yellow
            vec3<f32>(0.9, 0.6, 0.3),  // Orange
            sin(input.uv.y * 10.0) * 0.5 + 0.5
        );
    } else if (input.planet_type < 2.5) {
        // Ice world
        base_color = vec3<f32>(0.8, 0.9, 1.0);
    } else {
        // Ocean world
        base_color = mix(
            vec3<f32>(0.0, 0.2, 0.5),  // Deep blue
            vec3<f32>(0.2, 0.5, 0.7),  // Light blue
            input.uv.y
        );
    }
    
    // Apply color hint
    base_color = mix(base_color, input.color_hint, 0.3);
    
    // Combine lighting
    let color = base_color * (ambient + diffuse * 0.9);
    
    // Add atmospheric glow at edges
    let rim = 1.0 - max(0.0, dot(input.normal, normalize(camera.position - input.world_pos)));
    let atmosphere = pow(rim, 3.0) * vec3<f32>(0.3, 0.5, 1.0);
    
    return vec4<f32>(color + atmosphere * 0.3, 1.0);
}

// ============================================================================
// ORBIT RENDERING (Lines)
// ============================================================================

struct OrbitVertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) alpha: f32,
};

struct OrbitOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) alpha: f32,
};

@vertex
fn vs_orbit(vertex: OrbitVertex) -> OrbitOutput {
    var output: OrbitOutput;
    output.position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    output.color = vertex.color;
    output.alpha = vertex.alpha;
    return output;
}

@fragment
fn fs_orbit(input: OrbitOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, input.alpha * 0.5);
}
```

---

## PHASE 4: PLANET SURFACE VISUALIZATION

### 4.1 Create PlanetRenderer

**New File:** `src/gui/renderer/planet_renderer.rs`

```rust
//! PlanetRenderer - Renders planetary surfaces with dynamic systems
//!
//! Visualizes the four planetary subsystems:
//! - Lithosphere: Tectonic plates, volcanoes, terrain, earthquakes
//! - Hydrosphere: Oceans, rivers, lakes, glaciers, water cycle
//! - Atmosphere: Clouds, storms, weather fronts, wind
//! - EnergyFlow: Solar radiation, day/night, seasons
//!
//! CONNECTS TO:
//! - Lithosphere (src/planet/lithosphere.rs)
//! - Hydrosphere (src/planet/hydrosphere.rs)
//! - DynamicAtmosphere (src/planet/atmosphere.rs)
//! - EnergyFlowSystem (src/planet/energy_flow.rs)

use wgpu::*;
use crate::planet::{Lithosphere, Hydrosphere, DynamicAtmosphere, EnergyFlowSystem};

pub struct PlanetRenderer {
    device: Device,
    
    // Terrain mesh (from Lithosphere)
    terrain_pipeline: RenderPipeline,
    terrain_vertex_buffer: Buffer,
    terrain_index_buffer: Buffer,
    terrain_texture: Texture,
    
    // Water surface (from Hydrosphere)
    water_pipeline: RenderPipeline,
    water_vertex_buffer: Buffer,
    
    // Cloud layer (from Atmosphere)
    cloud_pipeline: RenderPipeline,
    cloud_texture: Texture,
    cloud_buffer: Buffer,
    
    // Atmosphere glow
    atmosphere_pipeline: RenderPipeline,
    
    // Weather systems (storms, fronts)
    storm_pipeline: RenderPipeline,
    storm_buffer: Buffer,
    
    // Camera
    camera_bind_group: BindGroup,
    camera_buffer: Buffer,
    
    // Planet uniform (rotation, lighting)
    planet_bind_group: BindGroup,
    planet_buffer: Buffer,
}

/// Uniform data for planet rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct PlanetUniform {
    /// View-projection matrix
    view_proj: [[f32; 4]; 4],
    
    /// Planet rotation matrix
    rotation: [[f32; 4]; 4],
    
    /// Time for animations
    time: f32,
    
    /// Axial tilt in radians
    axial_tilt: f32,
    
    /// Angle to star (day/night)
    solar_angle: f32,
    
    /// Planet radius
    radius: f32,
}

impl PlanetRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        // Load planet shader
        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Planet Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/planet.wgsl").into()),
        });
        
        // Create bind group layouts
        // Create pipelines
        // Create buffers
        
        // ... implementation similar to CosmosRenderer
        
        Self {
            device: device.clone(),
            terrain_pipeline: Self::create_terrain_pipeline(device, &shader, surface_config),
            water_pipeline: Self::create_water_pipeline(device, &shader, surface_config),
            cloud_pipeline: Self::create_cloud_pipeline(device, &shader, surface_config),
            atmosphere_pipeline: Self::create_atmosphere_pipeline(device, &shader, surface_config),
            storm_pipeline: Self::create_storm_pipeline(device, &shader, surface_config),
            terrain_vertex_buffer: Self::create_terrain_buffers(device),
            terrain_index_buffer: device.create_buffer(&BufferDescriptor::default()),
            water_vertex_buffer: device.create_buffer(&BufferDescriptor::default()),
            cloud_buffer: device.create_buffer(&BufferDescriptor::default()),
            storm_buffer: device.create_buffer(&BufferDescriptor::default()),
            terrain_texture: Self::create_terrain_texture(device),
            cloud_texture: Self::create_cloud_texture(device),
            camera_bind_group: device.create_bind_group(&BindGroupDescriptor::default()),
            camera_buffer: device.create_buffer(&BufferDescriptor::default()),
            planet_bind_group: device.create_bind_group(&BindGroupDescriptor::default()),
            planet_buffer: device.create_buffer(&BufferDescriptor::default()),
        }
    }
    
    /// Update from simulation state
    pub fn update(
        &mut self,
        queue: &Queue,
        lithosphere: &Lithosphere,
        hydrosphere: &Hydrosphere,
        atmosphere: &DynamicAtmosphere,
        energy_flow: &EnergyFlowSystem,
    ) {
        // Update terrain heights from lithosphere.crust
        self.update_terrain(queue, lithosphere);
        
        // Update water level from hydrosphere.oceans
        self.update_water(queue, hydrosphere);
        
        // Update cloud texture from atmosphere.clouds
        self.update_clouds(queue, atmosphere);
        
        // Update storm positions
        self.update_storms(queue, atmosphere);
    }
    
    fn update_terrain(&mut self, queue: &Queue, lithosphere: &Lithosphere) {
        // Generate heightmap from lithosphere.crust
        // Apply tectonic plate boundaries
        // Add volcanic peaks
        // Apply erosion
        
        // Write to terrain_vertex_buffer
    }
    
    fn update_water(&mut self, queue: &Queue, hydrosphere: &Hydrosphere) {
        // Update ocean depth
        // Update river paths
        // Update lake levels
        // Update glacier positions
        
        // Write to water_vertex_buffer
    }
    
    fn update_clouds(&mut self, queue: &Queue, atmosphere: &DynamicAtmosphere) {
        // Generate cloud coverage from atmosphere state
        // Update cloud texture
        
        // Write to cloud_texture
    }
    
    fn update_storms(&mut self, queue: &Queue, atmosphere: &DynamicAtmosphere) {
        // Extract storm positions and intensities
        // Write to storm_buffer
    }
    
    /// Render planet surface
    pub fn render(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Render in order: terrain, water, clouds, atmosphere, storms
        
        // 1. Render terrain mesh with height-based coloring
        self.render_terrain(encoder, view);
        
        // 2. Render water surface with transparency and reflections
        self.render_water(encoder, view);
        
        // 3. Render cloud layer with rotation
        self.render_clouds(encoder, view);
        
        // 4. Render atmosphere glow
        self.render_atmosphere(encoder, view);
        
        // 5. Render weather systems (storms, fronts)
        self.render_storms(encoder, view);
    }
    
    fn render_terrain(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Terrain render pass
    }
    
    fn render_water(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Water render pass
    }
    
    fn render_clouds(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Cloud render pass
    }
    
    fn render_atmosphere(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Atmosphere render pass
    }
    
    fn render_storms(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Storm render pass
    }
    
    // Pipeline creation helpers...
    fn create_terrain_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        // Implementation
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_water_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_cloud_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_atmosphere_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_storm_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_terrain_buffers(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor::default())
    }
    
    fn create_terrain_texture(device: &Device) -> Texture {
        device.create_texture(&TextureDescriptor::default())
    }
    
    fn create_cloud_texture(device: &Device) -> Texture {
        device.create_texture(&TextureDescriptor::default())
    }
}
```

### 4.2 Planet Surface Shader

**New File:** `src/gui/renderer/shaders/planet.wgsl`

```rust
//! Planet Surface Shader
//!
//! Renders planetary surfaces with:
//! - Terrain (from Lithosphere)
//! - Water (from Hydrosphere)
//! - Clouds (from Atmosphere)
//! - Weather (from Atmosphere)
//! - Day/night cycle (from EnergyFlow)

struct PlanetUniform {
    view_proj: mat4x4<f32>,
    rotation: mat4x4<f32>,
    time: f32,
    axial_tilt: f32,
    solar_angle: f32,
    radius: f32,
    camera_pos: vec3<f32>,
    _pad: f32,
};

@group(0) @binding(0)
var<uniform> planet: PlanetUniform;

// Terrain heightmap texture
@group(1) @binding(0)
var terrain_heightmap: texture_2d<f32>;

@group(1) @binding(1)
var terrain_sampler: sampler;

// Cloud texture
@group(2) @binding(0)
var cloud_texture: texture_2d<f32>;

@group(2) @binding(1)
var cloud_sampler: sampler;

// ============================================================================
// TERRAIN TYPES (from lithosphere)
// ============================================================================

const TERRAIN_DEEP_OCEAN: f32 = -1.0;
const TERRAIN_OCEAN: f32 = -0.5;
const TERRAIN_COASTAL: f32 = 0.0;
const TERRAIN_BEACH: f32 = 0.05;
const TERRAIN_PLAINS: f32 = 0.2;
const TERRAIN_HILLS: f32 = 0.4;
const TERRAIN_MOUNTAINS: f32 = 0.6;
const TERRAIN_HIGH_PEAKS: f32 = 0.8;
const TERRAIN_VOLCANIC: f32 = 0.9;
const TERRAIN_ICE: f32 = 1.0;

// ============================================================================
// TERRAIN VERTEX
// ============================================================================

struct TerrainVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) plate_id: f32,
    @location(4) volcanic_activity: f32,
};

struct TerrainOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) volcanic_activity: f32,
    @location(4) latitude: f32,
};

@vertex
fn vs_terrain(vertex: TerrainVertex) -> TerrainOutput {
    var output: TerrainOutput;
    
    // Apply planet rotation
    let rotated_pos = planet.rotation * vec4<f32>(vertex.position, 1.0);
    
    // Scale by planet radius
    let scaled_pos = rotated_pos.xyz * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.normal = (planet.rotation * vec4<f32>(vertex.normal, 0.0)).xyz;
    output.uv = vertex.uv;
    output.world_pos = scaled_pos;
    output.volcanic_activity = vertex.volcanic_activity;
    output.latitude = asin(normalize(scaled_pos).y);
    
    return output;
}

@fragment
fn fs_terrain(input: TerrainOutput) -> @location(0) vec4<f32> {
    // Sample heightmap
    let height = textureSample(terrain_heightmap, terrain_sampler, input.uv).r;
    
    // Determine terrain type color
    var color: vec3<f32>;
    
    if (height < TERRAIN_COASTAL) {
        // Ocean - handled by water pass
        color = vec3<f32>(0.0, 0.1, 0.3);
    } else if (height < TERRAIN_BEACH) {
        // Beach/coastal
        color = vec3<f32>(0.76, 0.7, 0.5);
    } else if (height < TERRAIN_HILLS) {
        // Plains to hills - grassland to forest
        let t = (height - TERRAIN_PLAINS) / (TERRAIN_HILLS - TERRAIN_PLAINS);
        color = mix(
            vec3<f32>(0.3, 0.6, 0.2),  // Grassland
            vec3<f32>(0.15, 0.4, 0.15),  // Forest
            t
        );
    } else if (height < TERRAIN_HIGH_PEAKS) {
        // Mountains
        let t = (height - TERRAIN_MOUNTAINS) / (TERRAIN_HIGH_PEAKS - TERRAIN_MOUNTAINS);
        color = mix(
            vec3<f32>(0.4, 0.35, 0.3),  // Rock
            vec3<f32>(0.95, 0.95, 1.0),  // Snow
            t
        );
    } else {
        // High peaks / ice
        color = vec3<f32>(0.95, 0.95, 1.0);
    }
    
    // Add volcanic coloring
    if (input.volcanic_activity > 0.5) {
        let glow = sin(planet.time * 2.0) * 0.1 + 0.9;
        color = mix(color, vec3<f32>(0.8, 0.2, 0.0), input.volcanic_activity * glow);
    }
    
    // Lighting from star
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let diffuse = max(0.0, dot(normalize(input.normal), normalize(light_dir)));
    let ambient = 0.15;
    
    // Latitude-based ice caps
    let ice_factor = smoothstep(0.7, 0.9, abs(input.latitude) / 1.5708);
    color = mix(color, vec3<f32>(0.95, 0.95, 1.0), ice_factor * 0.8);
    
    // Final color
    let final_color = color * (ambient + diffuse * 0.85);
    
    return vec4<f32>(final_color, 1.0);
}

// ============================================================================
// WATER VERTEX
// ============================================================================

struct WaterVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) depth: f32,  // From hydrosphere
};

struct WaterOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_pos: vec3<f32>,
    @location(3) depth: f32,
};

@vertex
fn vs_water(vertex: WaterVertex) -> WaterOutput {
    var output: WaterOutput;
    
    // Apply wave animation
    let wave_height = sin(vertex.uv.x * 20.0 + planet.time * 2.0) * 0.001;
    let wave_height2 = sin(vertex.uv.y * 15.0 + planet.time * 1.5) * 0.0008;
    
    var pos = vertex.position;
    pos = pos + vertex.normal * (wave_height + wave_height2);
    
    let rotated_pos = planet.rotation * vec4<f32>(pos, 1.0);
    let scaled_pos = rotated_pos.xyz * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.normal = (planet.rotation * vec4<f32>(vertex.normal, 0.0)).xyz;
    output.uv = vertex.uv;
    output.world_pos = scaled_pos;
    output.depth = vertex.depth;
    
    return output;
}

@fragment
fn fs_water(input: WaterOutput) -> @location(0) vec4<f32> {
    // Water color based on depth
    let shallow_color = vec3<f32>(0.1, 0.5, 0.6);
    let deep_color = vec3<f32>(0.0, 0.15, 0.3);
    
    let depth_factor = smoothstep(0.0, 1.0, input.depth);
    let water_color = mix(shallow_color, deep_color, depth_factor);
    
    // Specular highlight from star
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let view_dir = normalize(planet.camera_pos - input.world_pos);
    let half_vec = normalize(light_dir + view_dir);
    let specular = pow(max(0.0, dot(input.normal, half_vec)), 64.0);
    
    // Fresnel effect
    let fresnel = pow(1.0 - max(0.0, dot(input.normal, view_dir)), 3.0);
    
    // Final color
    let color = water_color + specular * 0.5 + fresnel * vec3<f32>(0.3, 0.5, 0.7) * 0.3;
    
    return vec4<f32>(color, 0.85);
}

// ============================================================================
// CLOUD VERTEX
// ============================================================================

struct CloudVertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct CloudOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vs_cloud(vertex: CloudVertex) -> CloudOutput {
    var output: CloudOutput;
    
    // Clouds slightly above surface
    let cloud_height = 0.02;
    let pos = vertex.position * (1.0 + cloud_height);
    
    // Rotate clouds independently (faster than planet)
    let cloud_rotation = planet.time * 0.1;
    let rotated = vec3<f32>(
        pos.x * cos(cloud_rotation) - pos.z * sin(cloud_rotation),
        pos.y,
        pos.x * sin(cloud_rotation) + pos.z * cos(cloud_rotation)
    );
    
    let scaled_pos = rotated * planet.radius;
    
    output.position = planet.view_proj * vec4<f32>(scaled_pos, 1.0);
    output.uv = vertex.uv + vec2<f32>(planet.time * 0.01, 0.0);
    output.world_pos = scaled_pos;
    
    return output;
}

@fragment
fn fs_cloud(input: CloudOutput) -> @location(0) vec4<f32> {
    // Sample cloud texture
    let cloud = textureSample(cloud_texture, cloud_sampler, input.uv);
    
    // Cloud density
    let density = cloud.r;
    
    // Only render where there are clouds
    if (density < 0.1) {
        discard;
    }
    
    // Cloud color (white to gray)
    let cloud_color = mix(
        vec3<f32>(0.9, 0.9, 0.95),
        vec3<f32>(0.6, 0.6, 0.65),
        density
    );
    
    // Lighting
    let light_dir = vec3<f32>(cos(planet.solar_angle), 0.3, sin(planet.solar_angle));
    let normal = normalize(input.world_pos);
    let diffuse = max(0.0, dot(normal, light_dir));
    
    let color = cloud_color * (0.3 + diffuse * 0.7);
    
    return vec4<f32>(color, density * 0.7);
}

// ============================================================================
// ATMOSPHERE GLOW
// ============================================================================

struct AtmosphereOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

@fragment
fn fs_atmosphere(input: AtmosphereOutput) -> @location(0) vec4<f32> {
    // Rayleigh scattering approximation
    let view_dir = normalize(planet.camera_pos - input.world_pos);
    
    // Scattering color (blue sky)
    let scatter_color = vec3<f32>(0.3, 0.5, 1.0);
    
    // Sun direction
    let light_dir = normalize(vec3<f32>(cos(planet.solar_angle), 0.0, sin(planet.solar_angle)));
    
    // Mie scattering (sun glow)
    let sun_glow = pow(max(0.0, dot(view_dir, light_dir)), 32.0) * vec3<f32>(1.0, 0.9, 0.7);
    
    // Rayleigh scattering (blue sky)
    let rayleigh = pow(1.0 - max(0.0, dot(view_dir, input.normal)), 4.0);
    
    // Combine
    let color = scatter_color * rayleigh + sun_glow;
    
    return vec4<f32>(color, rayleigh * 0.3);
}

// ============================================================================
// STORM RENDERING
// ============================================================================

struct StormVertex {
    @location(0) position: vec3<f32>,  // On sphere surface
    @location(1) size: f32,
    @location(2) intensity: f32,
    @location(3) rotation: f32,
};

struct StormOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) intensity: f32,
};

@vertex
fn vs_storm(vertex: StormVertex) -> StormOutput {
    var output: StormOutput;
    
    // Position storm on planet surface
    let pos = vertex.position * planet.radius * 1.001;  // Slightly above surface
    
    output.position = planet.view_proj * vec4<f32>(pos, 1.0);
    output.uv = vec2<f32>(0.5, 0.5);  // Center of spiral
    output.intensity = vertex.intensity;
    
    return output;
}

@fragment
fn fs_storm(input: StormOutput) -> @location(0) vec4<f32> {
    // Spiral pattern for hurricane
    let dist = length(input.uv - vec2<f32>(0.5, 0.5));
    let angle = atan2(input.uv.y - 0.5, input.uv.x - 0.5);
    
    // Spiral arms
    let spiral = sin(angle * 3.0 - dist * 20.0 + planet.time * 2.0) * 0.5 + 0.5;
    
    // Storm color (white/gray)
    let color = vec3<f32>(0.8, 0.8, 0.85) * spiral;
    
    // Fade at edges
    let alpha = (1.0 - dist * 2.0) * input.intensity;
    
    return vec4<f32>(color, max(0.0, alpha));
}
```

---

## PHASE 5: CIVILIZATION SYSTEM

### 5.1 Civilization Module Structure

**New Directory:** `src/civilization/`

```
src/civilization/
├── mod.rs              # Module exports and documentation
├── civilization.rs     # Civilization struct and logic
├── population.rs       # Population groups and individuals
├── settlement.rs       # Cities, towns, villages
├── culture.rs          # Cultural traits and evolution
└── civilization_bridge.rs  # Bridge to planet environment
```

**New File:** `src/civilization/mod.rs`

```rust
//! Civilization Module - Societies, cultures, populations
//!
//! This module implements the civilization layer between planets and individuals.
//! It fills the gap in the simulation hierarchy:
//!
//! SIMULATION HIERARCHY:
//! 1. Universe (CosmosEngine)
//! 2. Galaxy (CosmicWeb)
//! 3. Stellar System (StellarSystem)
//! 4. Planet (Planet with Lithosphere/Hydrosphere/Atmosphere)
//! 5. CIVILIZATION (this module) ← NEW
//! 6. Population (groups of individuals)
//! 7. Individual (consciousness entities)
//!
//! FROM COSMOLOGICAL-ARCHITECTURE.md:
//! "Civilizations emerge on habitable planets when consciousness
//! reaches sufficient density. The social memory complex is the
//! collective consciousness of the civilization."

pub mod civilization;
pub mod population;
pub mod settlement;
pub mod culture;
pub mod civilization_bridge;

pub use civilization::{Civilization, CivilizationId, TechnologyLevel};
pub use population::{Population, Individual, Demographics};
pub use settlement::{Settlement, SettlementId, SettlementType};
pub use culture::{Culture, CulturalTrait, Ideology};
pub use civilization_bridge::CivilizationPlanetBridge;

use crate::planet::Planet;

/// Manager for all civilizations in simulation
pub struct CivilizationManager {
    civilizations: Vec<Civilization>,
    next_id: u64,
}

impl CivilizationManager {
    pub fn new() -> Self {
        Self {
            civilizations: Vec::new(),
            next_id: 0,
        }
    }
    
    /// Create a new civilization on a planet
    pub fn create_civilization(&mut self, planet: &Planet) -> CivilizationId {
        let id = CivilizationId(self.next_id);
        self.next_id += 1;
        
        let civilization = Civilization::new(planet, id);
        self.civilizations.push(civilization);
        
        id
    }
    
    /// Get civilization by ID
    pub fn get_civilization(&self, id: CivilizationId) -> Option<&Civilization> {
        self.civilizations.iter().find(|c| c.id == id)
    }
    
    /// Get mutable civilization by ID
    pub fn get_civilization_mut(&mut self, id: CivilizationId) -> Option<&mut Civilization> {
        self.civilizations.iter_mut().find(|c| c.id == id)
    }
    
    /// Get all civilizations
    pub fn all_civilizations(&self) -> &[Civilization] {
        &self.civilizations
    }
    
    /// Tick all civilizations
    pub fn tick(&mut self, planets: &[Planet], dt: f64) {
        for civilization in &mut self.civilizations {
            if let Some(planet) = planets.iter().find(|p| p.id == civilization.planet_id) {
                civilization.tick(planet, dt);
            }
        }
    }
}

impl Default for CivilizationManager {
    fn default() -> Self {
        Self::new()
    }
}
```

### 5.2 Civilization Implementation

**New File:** `src/civilization/civilization.rs`

```rust
//! Civilization - Large-scale societal structures
//!
//! A civilization is a collection of populations, settlements, and culture
//! existing on a habitable planet. Civilizations evolve over time based on:
//! - Environmental conditions (planet state)
//! - Technology progression
//! - Cultural development
//! - Consciousness evolution
//!
//! FROM COSMOLOGICAL-ARCHITECTURE.md:
//! "Third-density civilizations are the training ground for
//! social memory complex formation. The polarization of the
//! civilization determines its harvestability."

use super::{Population, Settlement, SettlementId, Culture};
use crate::planet::Planet;
use std::collections::HashMap;

/// Unique identifier for a civilization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CivilizationId(pub u64);

impl CivilizationId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Technology level on a modified Kardashev scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechnologyLevel {
    /// Pre-agricultural hunter-gatherer
    PreAgricultural = 0,
    
    /// Agricultural society
    Agricultural = 1,
    
    /// Industrial society
    Industrial = 2,
    
    /// Atomic/Digital society
    Atomic = 3,
    
    /// Information age
    Digital = 4,
    
    /// Fusion power
    Fusion = 5,
    
    /// Stellar civilization
    Stellar = 6,
    
    /// Galactic civilization
    Galactic = 7,
    
    /// Intergalactic civilization
    Intergalactic = 8,
    
    /// Dimensional access
    Dimensional = 9,
    
    /// Infinite intelligence
    Infinite = 10,
}

impl Default for TechnologyLevel {
    fn default() -> Self {
        TechnologyLevel::PreAgricultural
    }
}

impl TechnologyLevel {
    /// Convert to numeric value for comparison
    pub fn level(&self) -> u8 {
        *self as u8
    }
    
    /// Get technology level from numeric value
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => TechnologyLevel::PreAgricultural,
            1 => TechnologyLevel::Agricultural,
            2 => TechnologyLevel::Industrial,
            3 => TechnologyLevel::Atomic,
            4 => TechnologyLevel::Digital,
            5 => TechnologyLevel::Fusion,
            6 => TechnologyLevel::Stellar,
            7 => TechnologyLevel::Galactic,
            8 => TechnologyLevel::Intergalactic,
            9 => TechnologyLevel::Dimensional,
            10 => TechnologyLevel::Infinite,
            _ => TechnologyLevel::Infinite,
        }
    }
}

/// A civilization on a planet
#[derive(Debug, Clone)]
pub struct Civilization {
    /// Unique identifier
    pub id: CivilizationId,
    
    /// Name of the civilization
    pub name: String,
    
    /// Planet this civilization exists on
    pub planet_id: u64,
    
    /// Population groups
    pub populations: Vec<Population>,
    
    /// Settlements (cities, towns, villages)
    pub settlements: HashMap<SettlementId, Settlement>,
    
    /// Cultural traits
    pub culture: Culture,
    
    /// Technology level
    pub technology_level: TechnologyLevel,
    
    /// Years since civilization began
    pub age: f64,
    
    /// Service-to-Others vs Service-to-Self polarization
    /// -1.0 = purely STS, 0.0 = mixed, +1.0 = purely STO
    pub polarization: f64,
    
    /// Average consciousness level of population
    pub average_consciousness: f64,
    
    /// Total population count
    pub total_population: u64,
    
    /// Is this civilization harvested?
    pub is_harvested: bool,
    
    /// Harvest readiness (consciousness and polarization criteria met)
    pub harvest_readiness: f64,
}

impl Civilization {
    /// Create a new civilization on a planet
    pub fn new(planet: &Planet, id: CivilizationId) -> Self {
        Self {
            id,
            name: format!("Civilization-{}", id.0),
            planet_id: planet.id,
            populations: Vec::new(),
            settlements: HashMap::new(),
            culture: Culture::random(),
            technology_level: TechnologyLevel::PreAgricultural,
            age: 0.0,
            polarization: 0.0,
            average_consciousness: 0.3,
            total_population: 0,
            is_harvested: false,
            harvest_readiness: 0.0,
        }
    }
    
    /// Create a seeded civilization with initial population
    pub fn seeded(planet: &Planet, id: CivilizationId, initial_population: u64) -> Self {
        let mut civ = Self::new(planet, id);
        
        // Create initial population
        let population = Population::new(0, initial_population);
        civ.populations.push(population);
        civ.total_population = initial_population;
        
        // Create initial settlement
        let settlement_id = SettlementId::new(0);
        let settlement = Settlement::new(
            settlement_id,
            id,
            (0.0, 0.0),  // Start at equator/prime meridian
            initial_population,
        );
        civ.settlements.insert(settlement_id, settlement);
        
        civ
    }
    
    /// Evolve civilization one tick
    pub fn tick(&mut self, planet: &Planet, dt: f64) {
        if self.is_harvested {
            return;
        }
        
        // Update populations
        for population in &mut self.populations {
            population.tick(planet, dt);
        }
        
        // Update settlements
        for settlement in self.settlements.values_mut() {
            settlement.tick(planet, dt);
        }
        
        // Technology advancement
        self.advance_technology(planet, dt);
        
        // Cultural evolution
        self.culture.evolve(dt);
        
        // Consciousness development
        self.evolve_consciousness(dt);
        
        // Polarization dynamics
        self.evolve_polarization(dt);
        
        // Check harvest readiness
        self.check_harvest_readiness();
        
        // Update total population
        self.total_population = self.populations.iter().map(|p| p.size).sum();
        
        self.age += dt;
    }
    
    /// Advance technology based on conditions
    fn advance_technology(&mut self, planet: &Planet, dt: f64) {
        // Technology advancement rate depends on:
        // - Population size (more minds = more innovation)
        // - Education level
        // - Resource availability
        // - Stability
        // - Current tech level (diminishing returns)
        
        let population_factor = (self.total_population as f64).ln() / 10.0;
        let education_factor = self.average_education();
        let stability_factor = self.calculate_stability();
        
        let advancement_rate = population_factor * education_factor * stability_factor * 0.001;
        
        // Random chance of breakthrough
        if rand::random::<f64>() < advancement_rate * dt {
            let current_level = self.technology_level.level();
            if current_level < 10 {
                self.technology_level = TechnologyLevel::from_level(current_level + 1);
            }
        }
    }
    
    /// Evolve consciousness of civilization
    fn evolve_consciousness(&mut self, dt: f64) {
        // Consciousness evolution depends on:
        // - Catalyst events
        // - Cultural values
        // - Technology level (can help or hinder)
        // - Meditation/contemplation practices
        
        let base_rate = 0.0001;
        let culture_factor = self.culture.consciousness_orientation;
        let tech_factor = if self.technology_level.level() >= 4 {
            1.1  // Digital age helps
        } else if self.technology_level.level() >= 2 {
            0.9  // Industrial age hinders
        } else {
            1.0  // Neutral
        };
        
        let evolution = base_rate * culture_factor * tech_factor * dt;
        self.average_consciousness = (self.average_consciousness + evolution).min(1.0);
    }
    
    /// Evolve polarization of civilization
    fn evolve_polarization(&mut self, dt: f64) {
        // Polarization drifts based on:
        // - Cultural values
        // - Resource scarcity
        // - External threats
        // - Catalyst events
        
        let drift = self.culture.polarization_drift * dt * 0.0001;
        self.polarization = (self.polarization + drift).clamp(-1.0, 1.0);
    }
    
    /// Check if civilization is ready for harvest
    fn check_harvest_readiness(&mut self) {
        // Harvest criteria (from cosmological architecture):
        // 1. Consciousness >= 0.5 (ready for fourth density)
        // 2. Polarization >= 0.75 (STO) or <= -0.75 (STS)
        // 3. Civilization age >= 75000 years (typical cycle length)
        
        let consciousness_ready = self.average_consciousness >= 0.5;
        let polarization_ready = self.polarization.abs() >= 0.75;
        let age_ready = self.age >= 75000.0;
        
        // Calculate readiness score
        self.harvest_readiness = [
            if consciousness_ready { 1.0 } else { self.average_consciousness / 0.5 },
            if polarization_ready { 1.0 } else { self.polarization.abs() / 0.75 },
            if age_ready { 1.0 } else { self.age / 75000.0 },
        ].iter().sum::<f64>() / 3.0;
        
        // Check if harvested
        if consciousness_ready && polarization_ready {
            self.is_harvested = true;
        }
    }
    
    /// Calculate average education level
    fn average_education(&self) -> f64 {
        if self.populations.is_empty() {
            return 0.0;
        }
        
        self.populations.iter().map(|p| p.education_level).sum::<f64>() 
            / self.populations.len() as f64
    }
    
    /// Calculate stability of civilization
    fn calculate_stability(&self) -> f64 {
        // Stability based on:
        // - Resource satisfaction
        // - Social cohesion
        // - Absence of major conflicts
        
        // Simplified calculation
        0.7 + 0.3 * (1.0 - self.polarization.abs())
    }
    
    /// Add a new settlement
    pub fn add_settlement(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id, settlement);
    }
    
    /// Get settlement by ID
    pub fn get_settlement(&self, id: SettlementId) -> Option<&Settlement> {
        self.settlements.get(&id)
    }
    
    /// Get all settlements
    pub fn all_settlements(&self) -> Vec<&Settlement> {
        self.settlements.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_civilization_creation() {
        let planet = Planet::default();
        let id = CivilizationId::new(0);
        let civ = Civilization::new(&planet, id);
        
        assert_eq!(civ.id, id);
        assert_eq!(civ.technology_level, TechnologyLevel::PreAgricultural);
    }
    
    #[test]
    fn test_technology_levels() {
        assert_eq!(TechnologyLevel::PreAgricultural.level(), 0);
        assert_eq!(TechnologyLevel::Digital.level(), 4);
        assert_eq!(TechnologyLevel::Infinite.level(), 10);
    }
}
```

### 5.3 Population Implementation

**New File:** `src/civilization/population.rs`

```rust
//! Population - Groups of individuals with demographics
//!
//! A population is a group of individuals sharing:
//! - Geographic location (within a settlement or region)
//! - Cultural identity
//! - Demographic characteristics
//!
//! Individuals within a population are consciousness entities
//! (SubSubLogos) with physical bodies and daily activities.

use crate::entity_layer7::layer7::SubSubLogos;
use crate::planet::Planet;

/// A population group within a civilization
#[derive(Debug, Clone)]
pub struct Population {
    /// Unique identifier within civilization
    pub id: u64,
    
    /// Number of individuals
    pub size: u64,
    
    /// Age distribution (10 buckets: 0-9, 10-19, ..., 90+)
    pub age_distribution: [f64; 10],
    
    /// Consciousness distribution (per density level)
    pub consciousness_distribution: [f64; 8],
    
    /// Polarization distribution (21 buckets: -1.0 to +1.0 in 0.1 steps)
    pub polarization_distribution: [f64; 21],
    
    /// Average health of population
    pub average_health: f64,
    
    /// Education level (0-1)
    pub education_level: f64,
    
    /// Settlement this population belongs to
    pub settlement_id: Option<super::SettlementId>,
    
    /// Representative individuals (for detailed simulation)
    /// Not all individuals are simulated in detail
    pub sampled_individuals: Vec<Individual>,
}

impl Population {
    /// Create a new population
    pub fn new(id: u64, size: u64) -> Self {
        // Initialize with realistic age distribution
        let age_distribution = [
            0.12,  // 0-9
            0.12,  // 10-19
            0.14,  // 20-29
            0.14,  // 30-39
            0.12,  // 40-49
            0.10,  // 50-59
            0.08,  // 60-69
            0.06,  // 70-79
            0.04,  // 80-89
            0.02,  // 90+
        ];
        
        Self {
            id,
            size,
            age_distribution,
            consciousness_distribution: [0.0; 8],
            polarization_distribution: [0.0; 21],
            average_health: 0.8,
            education_level: 0.3,
            settlement_id: None,
            sampled_individuals: Vec::new(),
        }
    }
    
    /// Tick population for one time step
    pub fn tick(&mut self, planet: &Planet, dt: f64) {
        // Birth and death rates
        self.process_demographics(planet, dt);
        
        // Age advancement
        self.advance_ages(dt);
        
        // Consciousness development
        self.evolve_consciousness(planet, dt);
        
        // Migration (simplified)
        // ...
    }
    
    fn process_demographics(&mut self, planet: &Planet, dt: f64) {
        // Birth rate depends on:
        // - Planet habitability
        // - Technology level
        // - Cultural factors
        
        // Death rate depends on:
        // - Health
        // - Age distribution
        // - Environmental factors
        
        // Simplified model
        let birth_rate = 0.02 * planet.habitability_index();
        let death_rate = 0.01;
        
        let births = ((self.size as f64) * birth_rate * dt) as u64;
        let deaths = ((self.size as f64) * death_rate * dt) as u64;
        
        self.size = self.size.saturating_add(births).saturating_sub(deaths);
    }
    
    fn advance_ages(&mut self, dt: f64) {
        // Shift age distribution each year
        // Simplified: just advance time
    }
    
    fn evolve_consciousness(&mut self, planet: &Planet, dt: f64) {
        // Gradual consciousness evolution
        for i in 0..8 {
            self.consciousness_distribution[i] += 0.0001 * dt;
        }
    }
    
    /// Get average consciousness
    pub fn average_consciousness(&self) -> f64 {
        let total: f64 = self.consciousness_distribution.iter().sum();
        total / 8.0
    }
    
    /// Get average polarization
    pub fn average_polarization(&self) -> f64 {
        let mut weighted_sum = 0.0;
        for (i, &count) in self.polarization_distribution.iter().enumerate() {
            let polarization = -1.0 + (i as f64) * 0.1;
            weighted_sum += polarization * count;
        }
        let total: f64 = self.polarization_distribution.iter().sum();
        if total > 0.0 { weighted_sum / total } else { 0.0 }
    }
}

/// An individual person within a civilization
#[derive(Debug, Clone)]
pub struct Individual {
    /// Unique identifier
    pub id: u64,
    
    /// Core consciousness entity
    pub entity: SubSubLogos,
    
    /// Physical body state
    pub body: PhysicalBody,
    
    /// Age in years
    pub age: f64,
    
    /// Gender
    pub gender: Gender,
    
    /// Current occupation
    pub occupation: Occupation,
    
    /// Location on planet surface (lat, lon)
    pub location: (f64, f64),
    
    /// Current activity
    pub activity: Activity,
    
    /// Personal consciousness level
    pub consciousness_level: f64,
    
    /// Personal polarization
    pub polarization: f64,
}

/// Physical body state
#[derive(Debug, Clone)]
pub struct PhysicalBody {
    /// Health (0-1)
    pub health: f64,
    
    /// Energy (0-1)
    pub energy: f64,
    
    /// Nutrition (0-1)
    pub nutrition: f64,
    
    /// Rest (0-1)
    pub rest: f64,
    
    /// Age-related decline
    pub age_factor: f64,
}

impl Default for PhysicalBody {
    fn default() -> Self {
        Self {
            health: 1.0,
            energy: 1.0,
            nutrition: 0.8,
            rest: 1.0,
            age_factor: 1.0,
        }
    }
}

/// Gender for biological simulation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

/// Occupation types
#[derive(Debug, Clone, PartialEq)]
pub enum Occupation {
    Child,
    Student,
    Farmer,
    Artisan,
    Merchant,
    Scholar,
    Leader,
    Healer,
    Artist,
    Warrior,
    Mystic,
    Retired,
}

/// Current activity
#[derive(Debug, Clone, PartialEq)]
pub enum Activity {
    Sleeping,
    Eating,
    Working,
    Learning,
    Socializing,
    Meditating,
    Traveling,
    Resting,
    /// Engaged with catalyst event
    CatalystExperience,
}

impl Individual {
    /// Create a new individual
    pub fn new(id: u64, entity: SubSubLogos) -> Self {
        Self {
            id,
            entity,
            body: PhysicalBody::default(),
            age: 0.0,
            gender: Gender::Other,
            occupation: Occupation::Child,
            location: (0.0, 0.0),
            activity: Activity::Sleeping,
            consciousness_level: 0.3,
            polarization: 0.0,
        }
    }
    
    /// Tick individual for one time step
    pub fn tick(&mut self, planet: &Planet, dt: f64) {
        // Update body state
        self.body.tick(dt);
        
        // Age
        self.age += dt / 365.25;  // Convert days to years
        
        // Determine activity based on time of day
        self.determine_activity(planet);
        
        // Consciousness evolution
        self.evolve_consciousness(dt);
    }
    
    fn determine_activity(&mut self, planet: &Planet) {
        // Simplified activity cycle
        // In a full implementation, this would use planetary time
    }
    
    fn evolve_consciousness(&mut self, dt: f64) {
        // Gradual consciousness development
        // Influenced by catalysts, meditation, learning
    }
}

impl PhysicalBody {
    /// Tick body state
    pub fn tick(&mut self, dt: f64) {
        // Energy depletes over time
        self.energy = (self.energy - 0.01 * dt).max(0.0);
        
        // Rest recovers during sleep
        // Nutrition depletes
        // Health changes based on factors
        
        // Age factor decline
        self.age_factor = (self.age_factor - 0.00001 * dt).max(0.0);
    }
}

/// Demographics summary
#[derive(Debug, Clone, Default)]
pub struct Demographics {
    pub total_population: u64,
    pub birth_rate: f64,
    pub death_rate: f64,
    pub median_age: f64,
    pub life_expectancy: f64,
}
```

### 5.4 Settlement Implementation

**New File:** `src/civilization/settlement.rs`

```rust
//! Settlement - Cities, towns, villages on planet surface
//!
//! Settlements are the physical locations where populations gather.
//! They connect to the planet environment (lithosphere, hydrosphere, atmosphere)
//! and provide the context for individual activities.

use super::CivilizationId;
use crate::planet::Planet;

/// Unique identifier for a settlement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SettlementId(pub u64);

impl SettlementId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Type of settlement based on population
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementType {
    /// Small group of dwellings (< 100 people)
    Hamlet,
    
    /// Small community (100 - 1,000)
    Village,
    
    /// Larger community (1,000 - 10,000)
    Town,
    
    /// Urban center (10,000 - 1,000,000)
    City,
    
    /// Major urban center (1,000,000 - 10,000,000)
    Metropolis,
    
    /// Massive urban area (> 10,000,000)
    Megalopolis,
}

impl SettlementType {
    /// Determine type from population
    pub fn from_population(population: u64) -> Self {
        match population {
            0..=99 => SettlementType::Hamlet,
            100..=999 => SettlementType::Village,
            1_000..=9_999 => SettlementType::Town,
            10_000..=999_999 => SettlementType::City,
            1_000_000..=9_999_999 => SettlementType::Metropolis,
            _ => SettlementType::Megalopolis,
        }
    }
}

/// A settlement on a planet surface
#[derive(Debug, Clone)]
pub struct Settlement {
    /// Unique identifier
    pub id: SettlementId,
    
    /// Name of the settlement
    pub name: String,
    
    /// Civilization this belongs to
    pub civilization_id: CivilizationId,
    
    /// Type of settlement
    pub settlement_type: SettlementType,
    
    /// Position on planet surface (latitude, longitude)
    pub position: (f64, f64),
    
    /// Population count
    pub population: u64,
    
    /// Buildings and structures
    pub structures: Vec<Structure>,
    
    /// Local resources
    pub resources: LocalResources,
    
    /// Local technology level
    pub tech_level: f64,
    
    /// Settlement age in years
    pub age: f64,
}

/// A building or structure in a settlement
#[derive(Debug, Clone)]
pub struct Structure {
    /// Type of structure
    pub structure_type: StructureType,
    
    /// Position relative to settlement center
    pub local_position: (f64, f64),
    
    /// Size of structure
    pub size: f64,
    
    /// Condition (0-1)
    pub condition: f64,
    
    /// Age in years
    pub age: f64,
}

/// Types of structures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StructureType {
    /// Residential dwelling
    Dwelling,
    
    /// Agricultural facility
    Farm,
    
    /// Manufacturing facility
    Workshop,
    
    /// Trading center
    Market,
    
    /// Religious/spiritual center
    Temple,
    
    /// Educational facility
    School,
    
    /// Government building
    Government,
    
    /// Defensive structure
    Defense,
    
    /// Infrastructure (roads, bridges, etc.)
    Infrastructure,
}

/// Local resources available to settlement
#[derive(Debug, Clone, Default)]
pub struct LocalResources {
    /// Food production capacity
    pub food_production: f64,
    
    /// Water availability
    pub water_availability: f64,
    
    /// Mineral resources
    pub mineral_resources: f64,
    
    /// Energy production
    pub energy_production: f64,
    
    /// Trade value
    pub trade_value: f64,
}

impl Settlement {
    /// Create a new settlement
    pub fn new(
        id: SettlementId,
        civilization_id: CivilizationId,
        position: (f64, f64),
        initial_population: u64,
    ) -> Self {
        let settlement_type = SettlementType::from_population(initial_population);
        
        Self {
            id,
            name: format!("Settlement-{}", id.0),
            civilization_id,
            settlement_type,
            position,
            population: initial_population,
            structures: Vec::new(),
            resources: LocalResources::default(),
            tech_level: 0.1,
            age: 0.0,
        }
    }
    
    /// Tick settlement for one time step
    pub fn tick(&mut self, planet: &Planet, dt: f64) {
        // Update resources from planet
        self.update_resources(planet);
        
        // Population growth
        self.grow_population(dt);
        
        // Build structures
        self.build_structures(dt);
        
        // Age structures
        self.age_structures(dt);
        
        // Update settlement type if population changed
        self.settlement_type = SettlementType::from_population(self.population);
        
        self.age += dt / 365.25;
    }
    
    fn update_resources(&mut self, planet: &Planet) {
        // Get terrain at settlement position
        // Calculate food, water, minerals based on planet state
        // This connects to lithosphere, hydrosphere, atmosphere
    }
    
    fn grow_population(&mut self, dt: f64) {
        // Population growth based on resources
        let growth_rate = self.resources.food_availability() * 0.01;
        let growth = (self.population as f64 * growth_rate * dt) as u64;
        self.population = self.population.saturating_add(growth);
    }
    
    fn build_structures(&mut self, dt: f64) {
        // Build new structures based on population needs
        let structures_needed = self.population / 5;  // Rough ratio
        
        if self.structures.len() < structures_needed as usize {
            // Add a dwelling
            self.structures.push(Structure {
                structure_type: StructureType::Dwelling,
                local_position: (
                    (rand::random::<f64>() - 0.5) * 0.01,
                    (rand::random::<f64>() - 0.5) * 0.01,
                ),
                size: 1.0,
                condition: 1.0,
                age: 0.0,
            });
        }
    }
    
    fn age_structures(&mut self, dt: f64) {
        for structure in &mut self.structures {
            structure.age += dt / 365.25;
            structure.condition = (structure.condition - 0.0001 * dt).max(0.0);
        }
        
        // Remove collapsed structures
        self.structures.retain(|s| s.condition > 0.0);
    }
    
    /// Get settlement center position
    pub fn center_position(&self) -> (f64, f64) {
        self.position
    }
    
    /// Calculate settlement radius (approximate)
    pub fn radius(&self) -> f64 {
        // Rough approximation: sqrt(population) * base unit
        (self.population as f64).sqrt() * 0.001
    }
}

impl LocalResources {
    /// Calculate overall food availability
    pub fn food_availability(&self) -> f64 {
        (self.food_production + self.trade_value * 0.1).min(1.0)
    }
}
```

### 5.5 Culture Implementation

**New File:** `src/civilization/culture.rs`

```rust
//! Culture - Cultural traits and evolution
//!
//! Culture represents the shared values, beliefs, and practices
//! of a civilization. It influences:
//! - Polarization drift
//! - Consciousness development
//! - Technology direction
//! - Social structure

use std::collections::HashMap;

/// Cultural identity of a civilization
#[derive(Debug, Clone)]
pub struct Culture {
    /// Name of the culture
    pub name: String,
    
    /// Core values (value name -> importance 0-1)
    pub values: HashMap<String, f64>,
    
    /// Dominant ideology
    pub ideology: Ideology,
    
    /// Consciousness orientation (affects evolution rate)
    pub consciousness_orientation: f64,
    
    /// Polarization drift (tends toward STO or STS)
    pub polarization_drift: f64,
    
    /// Technology preference
    pub tech_preference: TechPreference,
    
    /// Cultural traits
    pub traits: Vec<CulturalTrait>,
}

/// Dominant ideology of a culture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ideology {
    /// Collectivist, harmony-focused
    Communal,
    
    /// Individualist, competition-focused
    Individualist,
    
    /// Spiritual, transcendence-focused
    Spiritual,
    
    /// Materialist, progress-focused
    Materialist,
    
    /// Balanced, adaptive
    Syncretic,
}

impl Default for Ideology {
    fn default() -> Self {
        Ideology::Syncretic
    }
}

/// Technology development preference
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TechPreference {
    /// Focus on biological/organic technology
    Organic,
    
    /// Focus on mechanical/industrial technology
    Mechanical,
    
    /// Focus on electronic/digital technology
    Electronic,
    
    /// Focus on consciousness/psychic technology
    Psychic,
    
    /// Balanced development
    Balanced,
}

impl Default for TechPreference {
    fn default() -> Self {
        TechPreference::Balanced
    }
}

/// A specific cultural trait
#[derive(Debug, Clone)]
pub struct CulturalTrait {
    /// Name of the trait
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Strength of the trait (0-1)
    pub strength: f64,
    
    /// How the trait affects civilization
    pub effects: Vec<CulturalEffect>,
}

/// Effects a cultural trait can have
#[derive(Debug, Clone)]
pub enum CulturalEffect {
    /// Modifies consciousness evolution rate
    ConsciousnessRate(f64),
    
    /// Modifies polarization drift
    PolarizationDrift(f64),
    
    /// Modifies technology advancement rate
    TechRate(f64),
    
    /// Modifies population growth rate
    PopulationGrowth(f64),
    
    /// Modifies resource efficiency
    ResourceEfficiency(f64),
    
    /// Modifies stability
    Stability(f64),
}

impl Culture {
    /// Create a random culture
    pub fn random() -> Self {
        let mut values = HashMap::new();
        values.insert("harmony".to_string(), rand::random());
        values.insert("knowledge".to_string(), rand::random());
        values.insert("freedom".to_string(), rand::random());
        values.insert("community".to_string(), rand::random());
        values.insert("progress".to_string(), rand::random());
        
        Self {
            name: "Primary Culture".to_string(),
            values,
            ideology: Ideology::Syncretic,
            consciousness_orientation: 0.5,
            polarization_drift: 0.0,
            tech_preference: TechPreference::Balanced,
            traits: Vec::new(),
        }
    }
    
    /// Create a culture with specific orientation
    pub fn oriented(consciousness_orientation: f64, polarization_drift: f64) -> Self {
        let mut culture = Self::random();
        culture.consciousness_orientation = consciousness_orientation;
        culture.polarization_drift = polarization_drift;
        culture
    }
    
    /// Evolve culture over time
    pub fn evolve(&mut self, dt: f64) {
        // Cultural values slowly shift
        for value in self.values.values_mut() {
            let drift = (rand::random::<f64>() - 0.5) * 0.0001 * dt;
            *value = (*value + drift).clamp(0.0, 1.0);
        }
        
        // Consciousness orientation tends to increase
        self.consciousness_orientation = (self.consciousness_orientation + 0.00001 * dt).min(1.0);
        
        // Traits can strengthen or weaken
        for trait_item in &mut self.traits {
            let change = (rand::random::<f64>() - 0.5) * 0.0001 * dt;
            trait_item.strength = (trait_item.strength + change).clamp(0.0, 1.0);
        }
        
        // Remove weak traits
        self.traits.retain(|t| t.strength > 0.1);
    }
    
    /// Get a cultural value
    pub fn get_value(&self, name: &str) -> f64 {
        *self.values.get(name).unwrap_or(&0.5)
    }
    
    /// Set a cultural value
    pub fn set_value(&mut self, name: &str, value: f64) {
        self.values.insert(name.to_string(), value.clamp(0.0, 1.0));
    }
    
    /// Calculate overall effect on consciousness rate
    pub fn consciousness_rate_modifier(&self) -> f64 {
        let mut modifier = 1.0;
        
        for trait_item in &self.traits {
            for effect in &trait_item.effects {
                if let CulturalEffect::ConsciousnessRate(r) = effect {
                    modifier *= 1.0 + r * trait_item.strength;
                }
            }
        }
        
        modifier
    }
    
    /// Calculate overall effect on polarization drift
    pub fn polarization_modifier(&self) -> f64 {
        let mut modifier = self.polarization_drift;
        
        for trait_item in &self.traits {
            for effect in &trait_item.effects {
                if let CulturalEffect::PolarizationDrift(d) = effect {
                    modifier += d * trait_item.strength;
                }
            }
        }
        
        modifier
    }
}
```

---

## PHASE 6: CIVILIZATION VISUALIZATION

### 6.1 Create CivilizationRenderer

**New File:** `src/gui/renderer/civilization_renderer.rs`

```rust
//! CivilizationRenderer - Visualizes settlements, populations, activities
//!
//! Renders the civilization layer on planet surfaces:
//! - Settlement markers (circles scaled by population)
//! - Population density heatmap
//! - Activity indicators (day/night cycle, work patterns)
//! - Trade routes and connections

use wgpu::*;
use crate::civilization::{Civilization, Settlement, CivilizationId};

pub struct CivilizationRenderer {
    device: Device,
    
    // Settlement markers
    settlement_pipeline: RenderPipeline,
    settlement_buffer: Buffer,
    
    // Population density overlay
    population_heatmap_pipeline: RenderPipeline,
    population_texture: Texture,
    
    // Connection lines (trade routes)
    connection_pipeline: RenderPipeline,
    connection_buffer: Buffer,
    
    // Bind groups
    camera_bind_group: BindGroup,
    camera_buffer: Buffer,
}

impl CivilizationRenderer {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
        // Load civilization shader
        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Civilization Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/civilization.wgsl").into()),
        });
        
        // Create pipelines and buffers
        // ... similar to other renderers
        
        Self {
            device: device.clone(),
            settlement_pipeline: Self::create_settlement_pipeline(device, &shader, surface_config),
            settlement_buffer: device.create_buffer(&BufferDescriptor {
                label: Some("Settlement Buffer"),
                size: 1024 * 1024,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            population_heatmap_pipeline: Self::create_heatmap_pipeline(device, &shader, surface_config),
            population_texture: device.create_texture(&TextureDescriptor {
                label: Some("Population Heatmap"),
                size: Extent3d { width: 1024, height: 512, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                view_formats: &[],
            }),
            connection_pipeline: Self::create_connection_pipeline(device, &shader, surface_config),
            connection_buffer: device.create_buffer(&BufferDescriptor {
                label: Some("Connection Buffer"),
                size: 1024 * 1024,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            camera_bind_group: device.create_bind_group(&BindGroupDescriptor::default()),
            camera_buffer: device.create_buffer(&BufferDescriptor::default()),
        }
    }
    
    /// Update from civilization state
    pub fn update(&mut self, queue: &Queue, civilization: &Civilization, planet_radius: f64) {
        // Update settlement positions and sizes
        self.update_settlements(queue, civilization, planet_radius);
        
        // Update population density texture
        self.update_population_heatmap(queue, civilization);
        
        // Update trade route connections
        self.update_connections(queue, civilization);
    }
    
    fn update_settlements(&mut self, queue: &Queue, civilization: &Civilization, planet_radius: f64) {
        // Convert settlements to vertex data
        // Position on planet sphere surface
        // Size based on population
        // Color based on tech level
    }
    
    fn update_population_heatmap(&mut self, queue: &Queue, civilization: &Civilization) {
        // Generate 2D heatmap of population density
        // Write to population_texture
    }
    
    fn update_connections(&mut self, queue: &Queue, civilization: &Civilization) {
        // Create lines between major settlements
        // Width based on trade volume
    }
    
    /// Render civilization layer
    pub fn render(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Render in order: heatmap, connections, settlements
        
        // 1. Population density as heat map overlay
        self.render_heatmap(encoder, view);
        
        // 2. Trade/migration connections
        self.render_connections(encoder, view);
        
        // 3. Settlement markers (circles with size = population)
        self.render_settlements(encoder, view);
    }
    
    fn render_heatmap(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Heatmap render pass
    }
    
    fn render_connections(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Connection render pass
    }
    
    fn render_settlements(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        // Settlement render pass
    }
    
    // Pipeline creation helpers...
    fn create_settlement_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_heatmap_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
    
    fn create_connection_pipeline(device: &Device, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor::default())
    }
}
```

### 6.2 Civilization Shader

**New File:** `src/gui/renderer/shaders/civilization.wgsl`

```rust
//! Civilization Visualization Shader
//!
//! Renders:
//! - Settlements as markers on planet surface
//! - Population density as heatmap overlay
//! - Trade routes as connection lines
//! - Activity indicators

struct CivilizationUniform {
    view_proj: mat4x4<f32>,
    time: f32,
    day_phase: f32,    // 0-1 for day/night cycle
    planet_radius: f32,
    _pad: f32,
};

@group(0) @binding(0)
var<uniform> civ: CivilizationUniform;

// Population heatmap texture
@group(1) @binding(0)
var population_heatmap: texture_2d<f32>;

@group(1) @binding(1)
var heatmap_sampler: sampler;

// ============================================================================
// SETTLEMENT TYPES
// ============================================================================

const SETTLEMENT_HAMLET: u32 = 0u;
const SETTLEMENT_VILLAGE: u32 = 1u;
const SETTLEMENT_TOWN: u32 = 2u;
const SETTLEMENT_CITY: u32 = 3u;
const SETTLEMENT_METROPOLIS: u32 = 4u;
const SETTLEMENT_MEGALOPOLIS: u32 = 5u;

// ============================================================================
// SETTLEMENT VERTEX
// ============================================================================

struct SettlementVertex {
    @location(0) position: vec3<f32>,  // On planet surface (normalized)
    @location(1) population: f32,
    @location(2) settlement_type: u32,
    @location(3) tech_level: f32,
    @location(4) consciousness_avg: f32,
    @location(5) polarization: f32,
};

struct SettlementOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) local_pos: vec2<f32>,
    @location(1) population: f32,
    @location(2) settlement_type: u32,
    @location(3) tech_level: f32,
    @location(4) consciousness_avg: f32,
    @location(5) polarization: f32,
};

@vertex
fn vs_settlement(vertex: SettlementVertex) -> SettlementOutput {
    var output: SettlementOutput;
    
    // Position on planet surface, slightly elevated
    let elevated = vertex.position * (civ.planet_radius * 1.001);
    
    output.position = civ.view_proj * vec4<f32>(elevated, 1.0);
    output.local_pos = vec2<f32>(0.0, 0.0);  // Will be set per-instance
    output.population = vertex.population;
    output.settlement_type = vertex.settlement_type;
    output.tech_level = vertex.tech_level;
    output.consciousness_avg = vertex.consciousness_avg;
    output.polarization = vertex.polarization;
    
    return output;
}

@fragment
fn fs_settlement(input: SettlementOutput) -> @location(0) vec4<f32> {
    // Distance from center of marker
    let dist = length(input.local_pos);
    
    // Discard outside circle
    if (dist > 1.0) {
        discard;
    }
    
    // Base color from technology level
    let tech_color = mix(
        vec3<f32>(0.4, 0.3, 0.2),  // Low tech - brown
        vec3<f32>(0.3, 0.8, 1.0),  // High tech - cyan
        input.tech_level
    );
    
    // Consciousness glow
    let glow = input.consciousness_avg * 0.4;
    
    // Polarization tint (STO = blue, STS = red)
    let polarization_tint = vec3<f32>(
        max(0.0, -input.polarization) * 0.5,  // Red for STS
        0.0,
        max(0.0, input.polarization) * 0.5    // Blue for STO
    );
    
    // Day/night effect - settlements light up at night
    let night_brightness = 1.0 - civ.day_phase;
    let light_intensity = 0.5 + night_brightness * 1.5;
    
    // Settlement type affects size indicator
    let type_indicator = 1.0 - dist * 0.8;
    
    // Combine colors
    let color = (tech_color + polarization_tint) * light_intensity 
              + vec3<f32>(glow) * (1.0 - dist);
    
    // Edge glow
    let edge_glow = smoothstep(0.8, 1.0, dist) * 0.3;
    
    return vec4<f32>(color * type_indicator, 1.0 - edge_glow);
}

// ============================================================================
// POPULATION HEATMAP
// ============================================================================

struct HeatmapVertex {
    @location(0) position: vec2<f32>,  // Screen quad
    @location(1) uv: vec2<f32>,
};

struct HeatmapOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_heatmap(vertex: HeatmapVertex) -> HeatmapOutput {
    var output: HeatmapOutput;
    output.position = vec4<f32>(vertex.position, 0.0, 1.0);
    output.uv = vertex.uv;
    return output;
}

@fragment
fn fs_heatmap(input: HeatmapOutput) -> @location(0) vec4<f32> {
    // Sample population density
    let density = textureSample(population_heatmap, heatmap_sampler, input.uv).r;
    
    // Heat map coloring: low = green, medium = yellow, high = red
    var color: vec3<f32>;
    if (density < 0.33) {
        color = mix(
            vec3<f32>(0.0, 0.2, 0.0),   // Dark green
            vec3<f32>(0.0, 0.8, 0.0),   // Green
            density * 3.0
        );
    } else if (density < 0.66) {
        color = mix(
            vec3<f32>(0.0, 0.8, 0.0),   // Green
            vec3<f32>(1.0, 1.0, 0.0),   // Yellow
            (density - 0.33) * 3.0
        );
    } else {
        color = mix(
            vec3<f32>(1.0, 1.0, 0.0),   // Yellow
            vec3<f32>(1.0, 0.0, 0.0),   // Red
            (density - 0.66) * 3.0
        );
    }
    
    return vec4<f32>(color, density * 0.5);
}

// ============================================================================
// TRADE ROUTE CONNECTIONS
// ============================================================================

struct ConnectionVertex {
    @location(0) position: vec3<f32>,
    @location(1) volume: f32,
    @location(2) start_color: vec3<f32>,
    @location(3) end_color: vec3<f32>,
};

struct ConnectionOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) volume: f32,
    @location(1) color: vec3<f32>,
    @location(2) t: f32,
};

@vertex
fn vs_connection(vertex: ConnectionVertex) -> ConnectionOutput {
    var output: ConnectionOutput;
    output.position = civ.view_proj * vec4<f32>(vertex.position, 1.0);
    output.volume = vertex.volume;
    output.color = vertex.start_color;
    output.t = 0.0;
    return output;
}

@fragment
fn fs_connection(input: ConnectionOutput) -> @location(0) vec4<f32> {
    // Animated flow effect
    let flow = sin(civ.time * 2.0 + input.t * 10.0) * 0.3 + 0.7;
    
    return vec4<f32>(input.color * flow, input.volume * 0.5);
}
```

---

## PHASE 7: SCALE NAVIGATION SYSTEM

### 7.1 Create Scale Navigator

**New File:** `src/gui/scale_navigator.rs`

```rust
//! Scale Navigator - Handles zooming between simulation scales
//!
//! Provides seamless navigation from universe to individual:
//! Universe → Galaxy → Solar System → Planet → Civilization → Individual
//!
//! SCALE LEVELS:
//! - Universe:  View cosmic web, galaxy clusters (10^26 m)
//! - Galaxy:    View stellar systems within a galaxy (10^21 m)
//! - Solar:     View star and planets (10^13 m)
//! - Planet:    View planet surface (10^7 m)
//! - Civilization: View cities, populations (10^5 m)
//! - Individual: View single entity (10^0 m)

/// Navigation scale levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaleLevel {
    /// Universe scale: Cosmic web, galaxy clusters
    /// Scale factor: ~10^26 meters
    Universe,
    
    /// Galaxy scale: Stellar systems within galaxy
    /// Scale factor: ~10^21 meters (100,000 light years)
    Galaxy,
    
    /// Solar system scale: Star with orbiting planets
    /// Scale factor: ~10^13 meters (100 AU)
    SolarSystem,
    
    /// Planet scale: Planetary surface
    /// Scale factor: ~10^7 meters (10,000 km)
    Planet,
    
    /// Civilization scale: Cities, populations
    /// Scale factor: ~10^5 meters (100 km)
    Civilization,
    
    /// Individual scale: Single entities
    /// Scale factor: ~10^0 meters (1 meter)
    Individual,
}

impl Default for ScaleLevel {
    fn default() -> Self {
        ScaleLevel::Planet
    }
}

impl ScaleLevel {
    /// Get the scale factor in meters
    pub fn scale_factor(&self) -> f64 {
        match self {
            ScaleLevel::Universe => 8.8e26,
            ScaleLevel::Galaxy => 1e21,
            ScaleLevel::SolarSystem => 1e13,
            ScaleLevel::Planet => 1e7,
            ScaleLevel::Civilization => 1e5,
            ScaleLevel::Individual => 1e0,
        }
    }
    
    /// Get the name of this scale level
    pub fn name(&self) -> &'static str {
        match self {
            ScaleLevel::Universe => "Universe",
            ScaleLevel::Galaxy => "Galaxy",
            ScaleLevel::SolarSystem => "Solar System",
            ScaleLevel::Planet => "Planet",
            ScaleLevel::Civilization => "Civilization",
            ScaleLevel::Individual => "Individual",
        }
    }
    
    /// Get the next larger scale (zoom out)
    pub fn zoom_out(&self) -> ScaleLevel {
        match self {
            ScaleLevel::Universe => ScaleLevel::Universe,
            ScaleLevel::Galaxy => ScaleLevel::Universe,
            ScaleLevel::SolarSystem => ScaleLevel::Galaxy,
            ScaleLevel::Planet => ScaleLevel::SolarSystem,
            ScaleLevel::Civilization => ScaleLevel::Planet,
            ScaleLevel::Individual => ScaleLevel::Civilization,
        }
    }
    
    /// Get the next smaller scale (zoom in)
    pub fn zoom_in(&self) -> ScaleLevel {
        match self {
            ScaleLevel::Universe => ScaleLevel::Galaxy,
            ScaleLevel::Galaxy => ScaleLevel::SolarSystem,
            ScaleLevel::SolarSystem => ScaleLevel::Planet,
            ScaleLevel::Planet => ScaleLevel::Civilization,
            ScaleLevel::Civilization => ScaleLevel::Individual,
            ScaleLevel::Individual => ScaleLevel::Individual,
        }
    }
    
    /// Can zoom in further?
    pub fn can_zoom_in(&self) -> bool {
        *self != ScaleLevel::Individual
    }
    
    /// Can zoom out further?
    pub fn can_zoom_out(&self) -> bool {
        *self != ScaleLevel::Universe
    }
}

/// Identifier for selected objects at different scales
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CosmicObjectId {
    Galaxy(u64),
    Star(u64),
    Planet(u64),
    Civilization(u64),
    Settlement(u64),
    Individual(u64),
}

/// Scale navigation state
#[derive(Debug, Clone)]
pub struct ScaleNavigator {
    /// Current scale level
    current_scale: ScaleLevel,
    
    /// Target scale for transitions
    target_scale: ScaleLevel,
    
    /// Transition progress (0.0 - 1.0)
    transition_progress: f64,
    
    /// Transition speed (progress per second)
    transition_speed: f64,
    
    /// Currently selected object at each scale
    selected: SelectionState,
    
    /// Camera interpolation state
    camera_state: CameraInterpolation,
}

/// Selected objects at different scales
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub selected_galaxy: Option<u64>,
    pub selected_star: Option<u64>,
    pub selected_planet: Option<u64>,
    pub selected_civilization: Option<u64>,
    pub selected_settlement: Option<u64>,
    pub selected_individual: Option<u64>,
}

/// Camera interpolation for smooth transitions
#[derive(Debug, Clone)]
pub struct CameraInterpolation {
    /// Starting position
    from_position: [f64; 3],
    
    /// Target position
    to_position: [f64; 3],
    
    /// Starting zoom
    from_zoom: f64,
    
    /// Target zoom
    to_zoom: f64,
    
    /// Starting rotation
    from_rotation: f64,
    
    /// Target rotation
    to_rotation: f64,
}

impl Default for CameraInterpolation {
    fn default() -> Self {
        Self {
            from_position: [0.0, 0.0, 0.0],
            to_position: [0.0, 0.0, 0.0],
            from_zoom: 1.0,
            to_zoom: 1.0,
            from_rotation: 0.0,
            to_rotation: 0.0,
        }
    }
}

impl ScaleNavigator {
    /// Create a new scale navigator
    pub fn new() -> Self {
        Self {
            current_scale: ScaleLevel::default(),
            target_scale: ScaleLevel::default(),
            transition_progress: 1.0,
            transition_speed: 0.5,  // Complete transition in 2 seconds
            selected: SelectionState::default(),
            camera_state: CameraInterpolation::default(),
        }
    }
    
    /// Get current scale level
    pub fn current_scale(&self) -> ScaleLevel {
        self.current_scale
    }
    
    /// Get target scale level
    pub fn target_scale(&self) -> ScaleLevel {
        self.target_scale
    }
    
    /// Is a transition in progress?
    pub fn is_transitioning(&self) -> bool {
        self.transition_progress < 1.0
    }
    
    /// Navigate to a specific scale
    pub fn navigate_to(&mut self, scale: ScaleLevel) {
        if scale == self.current_scale {
            return;
        }
        
        self.target_scale = scale;
        self.transition_progress = 0.0;
        
        // Set up camera interpolation
        self.setup_camera_transition();
    }
    
    /// Zoom in one level
    pub fn zoom_in(&mut self) {
        if self.current_scale.can_zoom_in() {
            self.navigate_to(self.current_scale.zoom_in());
        }
    }
    
    /// Zoom out one level
    pub fn zoom_out(&mut self) {
        if self.current_scale.can_zoom_out() {
            self.navigate_to(self.current_scale.zoom_out());
        }
    }
    
    /// Select an object and navigate to its scale
    pub fn select_and_navigate(&mut self, id: CosmicObjectId) {
        let scale = match id {
            CosmicObjectId::Galaxy(_) => ScaleLevel::Galaxy,
            CosmicObjectId::Star(_) => ScaleLevel::SolarSystem,
            CosmicObjectId::Planet(_) => ScaleLevel::Planet,
            CosmicObjectId::Civilization(_) => ScaleLevel::Civilization,
            CosmicObjectId::Settlement(_) => ScaleLevel::Civilization,
            CosmicObjectId::Individual(_) => ScaleLevel::Individual,
        };
        
        // Update selection
        match id {
            CosmicObjectId::Galaxy(id) => self.selected.selected_galaxy = Some(id),
            CosmicObjectId::Star(id) => self.selected.selected_star = Some(id),
            CosmicObjectId::Planet(id) => self.selected.selected_planet = Some(id),
            CosmicObjectId::Civilization(id) => self.selected.selected_civilization = Some(id),
            CosmicObjectId::Settlement(id) => self.selected.selected_settlement = Some(id),
            CosmicObjectId::Individual(id) => self.selected.selected_individual = Some(id),
        }
        
        self.navigate_to(scale);
    }
    
    /// Update transition progress
    pub fn tick(&mut self, dt: f64) {
        if self.transition_progress < 1.0 {
            self.transition_progress += dt * self.transition_speed;
            
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.current_scale = self.target_scale;
            }
        }
    }
    
    /// Get interpolated camera state
    pub fn get_camera_state(&self) -> ([f64; 3], f64, f64) {
        let t = self.ease_in_out(self.transition_progress);
        
        let position = [
            self.camera_state.from_position[0] + (self.camera_state.to_position[0] - self.camera_state.from_position[0]) * t,
            self.camera_state.from_position[1] + (self.camera_state.to_position[1] - self.camera_state.from_position[1]) * t,
            self.camera_state.from_position[2] + (self.camera_state.to_position[2] - self.camera_state.from_position[2]) * t,
        ];
        
        let zoom = self.camera_state.from_zoom + (self.camera_state.to_zoom - self.camera_state.from_zoom) * t;
        let rotation = self.camera_state.from_rotation + (self.camera_state.to_rotation - self.camera_state.from_rotation) * t;
        
        (position, zoom, rotation)
    }
    
    /// Get render scale (interpolated during transition)
    pub fn get_render_scale(&self) -> f64 {
        let from = self.current_scale.scale_factor();
        let to = self.target_scale.scale_factor();
        let t = self.ease_in_out(self.transition_progress);
        
        from * (to / from).powf(t)
    }
    
    /// Get current selection
    pub fn selection(&self) -> &SelectionState {
        &self.selected
    }
    
    fn setup_camera_transition(&mut self) {
        // Set up from/to camera states based on scale change
        let from_scale = self.current_scale.scale_factor();
        let to_scale = self.target_scale.scale_factor();
        
        self.camera_state.from_zoom = from_scale;
        self.camera_state.to_zoom = to_scale;
        
        // Keep position centered on selected object if any
        // This would need the actual object positions from simulation
    }
    
    fn ease_in_out(&self, t: f64) -> f64 {
        // Smooth step interpolation
        t * t * (3.0 - 2.0 * t)
    }
}

impl Default for ScaleNavigator {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## PHASE 8: INTEGRATION POINTS

### 8.1 Modify lib.rs

**File:** `src/lib.rs`

```rust
// Add to module declarations:

pub mod civilization;

// Add to re-exports:
pub use civilization::{
    Civilization, CivilizationId, CivilizationManager,
    Population, Individual, Demographics,
    Settlement, SettlementId, SettlementType,
    Culture, Ideology,
};
```

### 8.2 Modify gui/mod.rs

**File:** `src/gui/mod.rs`

```rust
// Add modules:
pub mod simulation_adapter;
pub mod scale_navigator;

// In renderer/mod.rs, add:
pub mod cosmos_renderer;
pub mod planet_renderer;
pub mod civilization_renderer;
```

### 8.3 Integration in SimulationRunner

**File:** `src/simulation_v3/simulation_runner.rs`

```rust
// Add field to SimulationRunner:

pub struct SimulationRunner {
    // ... existing fields ...
    
    /// Civilization manager
    civilization_manager: Option<CivilizationManager>,
    
    /// Cosmos engine for universe/galaxy/star visualization
    cosmos_engine: Option<CosmosEngine>,
}

// Add methods:

impl SimulationRunner {
    /// Get civilization data for rendering
    pub fn get_civilizations(&self) -> &[Civilization] {
        self.civilization_manager
            .as_ref()
            .map(|m| m.all_civilizations())
            .unwrap_or(&[])
    }
    
    /// Get cosmos engine for rendering
    pub fn get_cosmos_engine(&self) -> Option<&CosmosEngine> {
        self.cosmos_engine.as_ref()
    }
    
    /// Get living environment for planet rendering
    pub fn get_living_environment(&self) -> Option<&LivingEnvironment> {
        // Return reference to living environment
        None  // TODO: Connect
    }
}
```

---

## FILE CREATION CHECKLIST

### New Files to Create

```
src/gui/simulation_adapter.rs              [Phase 2] - Connect SimulationRunner to GUI
src/gui/scale_navigator.rs                 [Phase 7] - Multi-scale navigation

src/gui/renderer/cosmos_renderer.rs        [Phase 3] - Universe/galaxy/star visualization
src/gui/renderer/planet_renderer.rs        [Phase 4] - Planet surface visualization
src/gui/renderer/civilization_renderer.rs  [Phase 6] - Settlement/population visualization

src/gui/renderer/shaders/cosmos.wgsl       [Phase 3] - Cosmos shaders
src/gui/renderer/shaders/planet.wgsl       [Phase 4] - Planet shaders
src/gui/renderer/shaders/civilization.wgsl [Phase 6] - Civilization shaders

src/civilization/mod.rs                    [Phase 5] - Module exports
src/civilization/civilization.rs           [Phase 5] - Civilization struct
src/civilization/population.rs             [Phase 5] - Population/Individual
src/civilization/settlement.rs             [Phase 5] - Settlement struct
src/civilization/culture.rs                [Phase 5] - Cultural traits
src/civilization/civilization_bridge.rs    [Phase 5] - Planet integration
```

### Files to Modify

```
src/lib.rs                                 - Export civilization module
src/gui/mod.rs                             - Export new modules
src/gui/renderer/mod.rs                    - Export new renderers
src/gui/application.rs                     - Replace IntegratedSystem
src/simulation_v3/simulation_runner.rs     - Add civilization/cosmos accessors
```

---

## IMPLEMENTATION PRIORITY ORDER

| Priority | Phase | Description | Dependencies |
|----------|-------|-------------|--------------|
| **P0** | 1.1 | Fix initial zoom | None |
| **P1** | 2.1-2.2 | SimulationRunnerAdapter | P0 |
| **P2** | 3.1-3.2 | CosmosRenderer + Shader | P1 |
| **P3** | 4.1-4.2 | PlanetRenderer + Shader | P1 |
| **P4** | 7.1 | ScaleNavigator | P1 |
| **P5** | 5.1-5.5 | Civilization system | P1 |
| **P6** | 6.1-6.2 | CivilizationRenderer + Shader | P5 |
| **P7** | 8.1-8.3 | Integration points | P1-P6 |

---

## TESTING APPROACH

After each phase, verify:

1. **Phase 2**: SimulationRunner connected, entities rendering
2. **Phase 3**: Stars/planets visible at Universe/Galaxy scales
3. **Phase 4**: Planet surfaces with terrain/water/clouds
4. **Phase 5**: Civilizations emerging on habitable planets
5. **Phase 6**: Settlements visible on planet surfaces
6. **Phase 7**: Smooth transitions between scales

---

## END OF DOCUMENT

This roadmap provides complete implementation instructions. Each phase builds on the previous. Start with Phase 2 (SimulationRunnerAdapter) as the foundation, then proceed through visualization layers.

**Key insight:** The deep simulation already exists. The work is connecting it to visualization and filling the civilization gap in the hierarchy.
