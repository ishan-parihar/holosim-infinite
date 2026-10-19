# HOLOSIM INFINITE - MASTER CONTEXT DOCUMENT

**Last Updated**: 2026-03-02
**Version**: 9.0.0
**Purpose**: Complete context for AI assistants with no prior knowledge of this project

---

## PROJECT OVERVIEW

HoloSim Infinite is a Rust-based simulation of consciousness evolution based on the Law of One cosmological framework. It simulates:

- **Holographic architecture** where each part contains the whole
- **Density octave evolution** (8 densities of consciousness)
- **Free will** as the primary creative force
- **The Veil** - a forgetfulness mechanism at space/time boundary
- **Emergence** of matter, life, and consciousness from unified fields

### Core Cosmological Concepts

From `COSMOLOGICAL-ARCHITECTURE.md`:

1. **Three Primal Distortions**: Free Will, Love/Logos, Light
2. **"Transcend and Include"**: Each layer includes previous development
3. **Space/Time ↔ Time/Space Spectrum**: Continuum with Veil at v=1
4. **Density Octave**: 8 densities representing consciousness stages
5. **Holographic Principle**: Each entity contains the whole

---

## PROJECT STRUCTURE

```
/home/ishanp/Documents/GitHub/HoloSim_Infinite/
├── src/
│   ├── foundation/          # Layers 0-3 (Violet → Green): Three Primal Distortions
│   ├── spectrum/            # Layers 4-6 (Yellow → Red): Spectrum emergence
│   ├── entity_layer7/       # Layer 7: Individual entities
│   ├── evolution_density_octave/  # Density octave progression
│   ├── consciousness/       # Free Will and Archetype 22
│   ├── simulation_v3/       # Main simulation system
│   ├── civilization/        # Phase 5: Civilization system (NEW)
│   ├── gui/                 # Visualization system
│   │   ├── renderer/        # WGPU renderers
│   │   ├── camera/          # Camera controls
│   │   ├── ui/              # Egui panels
│   │   ├── visualization/   # Data visualization
│   │   └── simulation_adapter.rs  # Bridge between sim and GUI
│   ├── hpo/                 # Hyperparameter optimization
│   ├── holographic/         # Field systems
│   ├── biology/             # Biological emergence
│   ├── physics/             # Physics systems
│   └── lib.rs               # Module exports
├── Cargo.toml               # Dependencies
├── AGENTS.md                # Coding conventions for AI agents
└── docs/                    # Documentation
```

---

## IMPLEMENTED PHASES

### Phase 1: Camera Zoom Calibration
- **File**: `src/gui/camera/camera_extended.rs`
- Logarithmic zoom (10^-35m to 10^27m)
- Scale transitions between cosmic levels

### Phase 2: SimulationRunnerAdapter
- **File**: `src/gui/simulation_adapter.rs`
- Bridges `SimulationRunner` to GUI rendering
- Replaces simplified `HolographicSimulation` with full simulation

### Phase 3: Cosmos Visualization
- **Files**:
  - `src/gui/renderer/cosmos_renderer.rs`
  - `src/gui/renderer/shaders/cosmos.wgsl`
- Renders: Stars, planets, orbits, cosmic filaments
- `StarVertex`, `PlanetVertex`, `OrbitVertex`, `FilamentVertex`

### Phase 4: Planet Surface Visualization
- **Files**:
  - `src/gui/renderer/planet_renderer.rs`
  - `src/gui/renderer/shaders/planet.wgsl`
- Renders: Terrain, water, clouds, atmosphere, storms, settlements
- `TerrainVertex`, `WaterVertex`, `CloudVertex`, `StormVertex`, `SettlementVertex`

### Phase 5: Civilization System (CURRENT)
- **Files**:
  - `src/civilization/mod.rs` - CivilizationManager
  - `src/civilization/civilization.rs` - Civilization, TechnologyLevel
  - `src/civilization/population.rs` - Population, Individual
  - `src/civilization/settlement.rs` - Settlement, SettlementType
  - `src/civilization/culture.rs` - Culture, Ideology
  - `src/gui/renderer/civilization_renderer.rs`
  - `src/gui/renderer/shaders/civilization.wgsl`
- Features:
  - Technology levels (Kardashev 0-10)
  - Polarization (-1.0 STS to +1.0 STO)
  - Settlement hierarchy (Hamlet → Megalopolis)
  - Cultural traits and ideology

---

## KEY ARCHITECTURAL PATTERNS

### 1. SimulationRunnerAdapter Pattern

The adapter bridges deep simulation to GUI:

```rust
pub struct SimulationRunnerAdapter {
    runner: SimulationRunner,
    civilization_manager: CivilizationManager,
    cached_entities: Vec<EntityInstance>,
    cached_connections: Vec<HierarchyConnection>,
    // ...
}
```

Methods:
- `initialize()` - Run involution to create entities
- `step()` - Advance simulation
- `get_entities()` - For EntityRenderer
- `get_civilization_data()` - For CivilizationRenderer
- `get_cosmic_render_data()` - For CosmosRenderer

### 2. WGPU Renderer Pattern

Each renderer follows this structure:

```rust
pub struct SomeRenderer {
    // Pipelines
    pipeline: wgpu::RenderPipeline,
    
    // Uniforms
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    
    // Vertex buffers (recreated each frame)
    vertex_buffer: Option<wgpu::Buffer>,
    vertex_count: u32,
}
```

Key methods:
- `new(device, format)` - Create pipelines
- `update(device, queue, data)` - Update buffers
- `render(render_pass)` - Draw

### 3. Vertex Structure Pattern

All vertex structs use `bytemuck` for GPU compatibility:

```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SomeVertex {
    pub position: [f32; 3],
    // ... other fields
    pub _padding: [f32; N], // Always pad to 16-byte alignment
}
```

### 4. Module Organization Pattern

```rust
// In mod.rs:
pub mod submodule;
pub use submodule::{PublicType};

// Re-exports for convenience
pub use self::SummaryType;
```

---

## WGPU VERSION COMPATIBILITY

**Critical**: This project uses an older WGPU version. Pipeline descriptors must NOT include:

```rust
// ❌ DO NOT USE (not in this WGPU version):
entry_point: Some("vs_main"),  // Use: entry_point: "vs_main"
compilation_options: ...,
cache: None,

// ✅ CORRECT:
vertex: wgpu::VertexState {
    module: &shader,
    entry_point: "vs_main",  // Direct &str
    buffers: &[Vertex::desc()],
},
```

---

## TYPE CONVENTIONS

```rust
pub type Float = f64;
pub type EntityId = u64;

// In GUI (f32 for GPU):
pub struct Vertex {
    position: [f32; 3],  // GPU uses f32
}

// In simulation (f64 for precision):
pub struct SimulationData {
    value: Float,  // f64
}
```

---

## ROADMAP REFERENCE

From `HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md`:

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Camera zoom calibration | ✅ Complete |
| 2 | SimulationRunnerAdapter | ✅ Complete |
| 3 | Cosmos visualization | ✅ Complete |
| 4 | Planet surface visualization | ✅ Complete |
| 5 | Civilization system | ✅ Complete |
| 6 | Social memory complex | 🔲 Pending |
| 7 | Consciousness visualization | 🔲 Pending |
| 8 | Full integration | 🔲 Pending |

---

## COMMON COMMANDS

```bash
# Build
cargo build --release

# Check (fast)
cargo check --message-format=short

# Test
cargo test --release

# Run simulation
cargo run --release --bin holonic_realms

# Format
cargo fmt

# Lint
cargo clippy --all-targets --all-features
```

---

## CRITICAL FILES TO READ FIRST

When starting a new session, read these in order:

1. `AGENTS.md` - Coding conventions
2. `COSMOLOGICAL-ARCHITECTURE.md` - Domain concepts
3. `HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md` - Implementation plan
4. `src/lib.rs` - Module structure
5. `src/gui/simulation_adapter.rs` - Main adapter
6. `src/gui/renderer/mod.rs` - Renderer exports

---

## DEPENDENCIES (Cargo.toml)

Key dependencies:
- `wgpu` - GPU rendering
- `egui` / `eframe` - Immediate mode GUI
- `bytemuck` - GPU memory layout
- `nalgebra` - Linear algebra
- `rand` - Random number generation
- `rayon` - Parallelism

---

## ERROR PATTERNS TO AVOID

### 1. WGPU API Mismatch
```
error[E0599]: no method named `pipeline_compilation_options`
```
**Fix**: Remove `compilation_options` and `cache` fields

### 2. Type Mismatch f64/f32
```
error[E0277]: cannot multiply `f64` by `f32`
```
**Fix**: Add explicit casts: `value as f32` or `value as f64`

### 3. Private Import
```
error[E0603]: struct import is private
```
**Fix**: Add `pub use` in module's `mod.rs`

### 4. Duplicate Definition
```
error[E0255]: the name is defined multiple times
```
**Fix**: Remove one definition or use re-export

---

## ENTITY HIERARCHY

```
Universe
├── Galaxy
│   ├── StellarSystem
│   │   ├── Star
│   │   └── Planets
│   │       ├── Lithosphere
│   │       ├── Hydrosphere
│   │       ├── Atmosphere
│   │       └── Civilization
│   │           ├── Populations
│   │           ├── Settlements
│   │           └── Culture
│   └── Individual (EntityLayer7)
└── Consciousness Field
```

---

## CONSCIOUSNESS EVOLUTION

1. **1st Density**: Awareness (minerals, elements)
2. **2nd Density**: Growth (plants, animals)
3. **3rd Density**: Self-awareness (humans) ← Current human level
4. **4th Density**: Love/understanding
5. **5th Density**: Light/wisdom
6. **6th Density**: Unity
7. **7th Density**: Gateway
8. **8th Density**: Octave transition

---

## POLARIZATION SYSTEM

```
STS (-1.0) ←─────────────────────────────────→ STO (+1.0)
    │                                              │
    ├── Service to Self                           ├── Service to Others
    ├── Control, separation                       ├── Love, unity
    ├── Negative polarization                     ├── Positive polarization
    └── "Harvestable" at 95% STS                  └── "Harvestable" at 51% STO
```

---

## TESTING APPROACH

Tests are organized by module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code
    }
}
```

Run specific test:
```bash
cargo test --release test_name
```

---

## NEXT PHASE IMPLEMENTATION

When implementing a new phase:

1. Read the roadmap section for that phase
2. Create module in appropriate directory
3. Create renderer in `src/gui/renderer/`
4. Create WGSL shader in `src/gui/renderer/shaders/`
5. Add exports to `src/gui/renderer/mod.rs`
6. Integrate with `SimulationRunnerAdapter`
7. Run `cargo check` to validate
8. Run frontend-tester agent for validation

---

## COMMIT MESSAGE STYLE

```
Phase X: Feature description

- Created file1.rs with component A
- Created file2.rs with component B
- Integrated with SimulationRunnerAdapter
- Added exports to mod.rs

All compilation checks passed.
```

---

This document should provide sufficient context for any AI assistant to continue work on HoloSim Infinite without prior session knowledge.
