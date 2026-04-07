# HoloSim Infinite: Phase 1 Implementation Plan
## "Make Something Visible" — From Abstract Simulation to Interactive Reality

**Date**: April 8, 2026  
**Status**: Planning Complete → Ready for Execution  
**Branch**: `phase1-visible-simulation`  
**Goal**: A single entity in 2D space that you can see, click, and watch evolve — connected to real simulation data, not stubbed values.

---

## Executive Summary

The cosmological architecture (Violet→Layer 7, 22 archetypes, free will kernel) is **already implemented**. The holographic optimization framework is **designed but not integrated**. The critical gap is the **Observation Layer** — the bridge that translates consciousness states into visible, spatial, interactive game properties.

This plan focuses on **making the existing architecture visible** rather than adding new cosmological layers.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│  GOAL: This layer becomes functional                             │
│  ▼                                                              │
│  ┌─────────────┐    ┌─────────────────┐    ┌──────────────────┐ │
│  │  GUI Layer  │◄───│ Observation     │◄───│  HoloSim Engine  │ │
│  │  WGPU+egui  │    │ Bridge (NEW)    │    │  (EXISTING)      │ │
│  └─────────────┘    └─────────────────┘    └──────────────────┘ │
│       ↑                      ↑                      ↑            │
│   What you            Consciousness →          Existing:         │
│   SEE:                  Game Properties        - 22 archetypes   │
│   • Position            - position             - Free will       │
│   • Color               - velocity             - Density octave  │
│   • Size                - behavior_state       - Holographic fld │
│   • Movement            - intelligence         - Entity structs  │
│   • Click info          - appearance           - Involution      │
│                                                  - Evolution     │
└─────────────────────────────────────────────────────────────────┘
```

**Key Principle**: Consciousness drives the "why" (decisions, behavior), physics handles the "how" (movement, collisions). Hybrid approach.

---

## Implementation Tasks

### Task 1: Define ObservableProperties — The Game-Facing Entity Struct

**Why**: The GUI needs concrete, renderable properties. Currently entities have `consciousness_level`, `archetype_activations`, `density` — but no `position`, `velocity`, `color`, `size`, or `behavior_state`. This struct is the single source of truth for what games can observe about an entity.

**Files to create/modify**:
- **CREATE** `src/gui/observable_properties.rs` — new module
- **MODIFY** `src/gui/mod.rs` — add `pub mod observable_properties;`

**Exact specification**:

```rust
// src/gui/observable_properties.rs

/// Game-facing entity properties — what any game engine can observe.
/// This is the bridge between HoloSim's consciousness architecture and
/// observable, renderable, interactive game properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObservableProperties {
    // === Spatial ===
    /// 2D position in world space (meters)
    pub position: [f32; 2],
    /// 2D velocity (meters/second)
    pub velocity: [f32; 2],
    /// Mass for physics calculations (kg, normalized 0.0–1.0)
    pub mass: f32,

    // === Appearance ===
    /// RGB color derived from archetype activation profile
    pub color: [f32; 3],
    /// Visual size (radius in world units), derived from density level
    pub size: f32,
    /// Glow intensity (0.0–1.0), derived from consciousness level
    pub glow: f32,
    /// Shape enum for visual differentiation
    pub shape: EntityShape,

    // === Behavior ===
    /// Current behavioral state (driven by archetype interference)
    pub behavior_state: BehaviorState,
    /// Intelligence/awareness level (0.0–1.0), maps to consciousness_level
    pub intelligence: f32,
    /// Service orientation: -1.0 (STS) to +1.0 (STO)
    pub service_orientation: f32,

    // === Identity ===
    /// Which density (1st–8th) this entity is evolving through
    pub density: u8,
    /// Top 3 active archetype indices (0–21) for debug visualization
    pub top_archetypes: [u8; 3],
    /// Entity is currently evolving (not dormant/dead)
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityShape {
    Circle,     // 1st-2nd density: reactive
    Triangle,   // 3rd density: self-aware
    Square,     // 4th density: loving
    Diamond,    // 5th density: wise
    Hexagon,    // 6th density: unified
    Star,       // 7th-8th density: transcendent
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorState {
    Idle,           // No strong archetype activation
    Seeking,        // High Catalyst archetype → exploring
    Fleeing,        // High Tower archetype → avoiding
    Socializing,    // High Lovers/Hierophant → connecting
    Contemplating,  // High Hermit/High Priestess → processing
    Leading,        // High Emperor/Chariot → directing
}

impl Default for ObservableProperties {
    fn default() -> Self {
        ObservableProperties {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            mass: 0.5,
            color: [0.5, 0.5, 0.5],
            size: 1.0,
            glow: 0.0,
            shape: EntityShape::Circle,
            behavior_state: BehaviorState::Idle,
            intelligence: 0.5,
            service_orientation: 0.0,
            density: 3,
            top_archetypes: [0, 0, 0],
            is_active: true,
        }
    }
}
```

**Tests required**:
- `test_observable_properties_default()` — defaults are sensible
- `test_entity_shape_from_density()` — density 1-2→Circle, 3→Triangle, 4→Square, 5→Diamond, 6→Hexagon, 7-8→Star
- `test_behavior_state_from_archetypes()` — specific archetype patterns produce correct behavior states

**Success criteria**:
- ✅ File compiles with zero warnings
- ✅ All 3 tests pass
- ✅ Module exported from `src/gui/mod.rs`
- ✅ No dependencies on simulation code (pure data types)

---

### Task 2: Implement Consciousness-to-Observation Mapper

**Why**: `ObservableProperties` is just a struct. We need a function that takes an entity's actual consciousness data (archetype activations, density, consciousness_level, spectrum_position) and maps it to those properties. This is the core "translation" logic.

**Files to create/modify**:
- **CREATE** `src/gui/observation_mapper.rs` — new module
- **MODIFY** `src/gui/mod.rs` — add `pub mod observation_mapper;`

**Exact specification**:

```rust
// src/gui/observation_mapper.rs

use crate::gui::observable_properties::*;

/// Maps consciousness state → observable game properties.
/// This is the core bridge: archetype interference drives behavior,
/// consciousness drives appearance, density drives capabilities.
pub struct ObservationMapper;

impl ObservationMapper {
    /// Map a single entity's consciousness state to ObservableProperties.
    ///
    /// # Arguments
    /// * `archetype_activations` — 22-element array of archetype coefficients [0.0–1.0]
    /// * `consciousness_level` — overall consciousness [0.0–1.0]
    /// * `density` — density level (1–8)
    /// * `spectrum_position` — position on space/time ↔ time/space spectrum [0.0–1.0]
    /// * `polarization` — service orientation [-1.0 (STS) to +1.0 (STO)]
    /// * `entity_index` — index in entity list (used for initial position spread)
    ///
    /// # Returns
    /// Fully populated ObservableProperties ready for rendering.
    pub fn map(
        archetype_activations: &[f64; 22],
        consciousness_level: f64,
        density: u8,
        spectrum_position: f64,
        polarization: f64,
        entity_index: usize,
    ) -> ObservableProperties {
        let mut props = ObservableProperties::default();

        // === Appearance: color from archetype interference ===
        props.color = Self::archetypes_to_color(archetype_activations);

        // === Appearance: size from density ===
        props.size = Self::density_to_size(density);

        // === Appearance: shape from density ===
        props.shape = Self::density_to_shape(density);

        // === Appearance: glow from consciousness ===
        props.glow = consciousness_level as f32;

        // === Behavior: state from top archetypes ===
        props.behavior_state = Self::archetypes_to_behavior(archetype_activations);

        // === Identity: intelligence from consciousness ===
        props.intelligence = consciousness_level as f32;

        // === Identity: service orientation ===
        props.service_orientation = polarization as f32;

        // === Identity: density ===
        props.density = density;

        // === Identity: top 3 archetypes ===
        props.top_archetypes = Self::top_n_archetypes(archetype_activations, 3);

        // === Spatial: initial position spread in a circle ===
        props.position = Self::initial_position(entity_index, 100);

        // === Spatial: velocity from behavior state ===
        props.velocity = Self::behavior_to_velocity(&props.behavior_state, spectrum_position);

        props
    }

    /// Convert 22 archetype activations to an RGB color.
    /// Mind archetypes (0-6) → Red channel
    /// Body archetypes (7-13) → Green channel
    /// Spirit archetypes (14-20) → Blue channel
    /// Choice archetype (21) → brightness modifier
    fn archetypes_to_color(activations: &[f64; 22]) -> [f32; 3] {
        let mind_avg: f64 = activations[0..7].iter().sum::<f64>() / 7.0;
        let body_avg: f64 = activations[7..14].iter().sum::<f64>() / 7.0;
        let spirit_avg: f64 = activations[14..21].iter().sum::<f64>() / 7.0;
        let choice = activations[21];

        let brightness = 0.5 + choice * 0.5; // 0.5–1.0 based on choice activation
        [
            (mind_avg * brightness) as f32,
            (body_avg * brightness) as f32,
            (spirit_avg * brightness) as f32,
        ]
    }

    /// Map density to visual size. Higher density = slightly larger (more presence).
    fn density_to_size(density: u8) -> f32 {
        match density {
            1 => 0.5,   // 1st density: smallest (elemental)
            2 => 0.7,   // 2nd density: small (biological)
            3 => 1.0,   // 3rd density: normal (self-aware)
            4 => 1.2,   // 4th density: larger (loving presence)
            5 => 1.4,   // 5th density: larger (wise presence)
            6 => 1.6,   // 6th density: large (unified presence)
            7 => 1.8,   // 7th density: very large (transcendent)
            8 => 2.0,   // 8th density: largest (return to source)
            _ => 1.0,
        }
    }

    /// Map density to visual shape.
    fn density_to_shape(density: u8) -> EntityShape {
        match density {
            1..=2 => EntityShape::Circle,
            3 => EntityShape::Triangle,
            4 => EntityShape::Square,
            5 => EntityShape::Diamond,
            6 => EntityShape::Hexagon,
            7..=8 => EntityShape::Star,
            _ => EntityShape::Circle,
        }
    }

    /// Map archetype activation pattern to behavioral state.
    /// Uses the highest-activated archetype to determine behavior:
    /// - Catalyst (A3, A10, A17) → Seeking
    /// - Tower (A11) → Fleeing
    /// - Lovers (A6), Hierophant (A5) → Socializing
    /// - Hermit (A9), High Priestess (A2) → Contemplating
    /// - Emperor (A4), Chariot (A7) → Leading
    fn archetypes_to_behavior(activations: &[f64; 22]) -> BehaviorState {
        let mut max_idx = 0usize;
        let mut max_val = activations[0];
        for (i, &val) in activations.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        match max_idx {
            3 | 10 | 17 => BehaviorState::Seeking,     // Catalyst
            11 => BehaviorState::Fleeing,               // Tower
            6 | 5 => BehaviorState::Socializing,        // Lovers, Hierophant
            9 | 2 => BehaviorState::Contemplating,      // Hermit, High Priestess
            4 | 7 => BehaviorState::Leading,            // Emperor, Chariot
            _ => BehaviorState::Idle,
        }
    }

    /// Get indices of top N most activated archetypes.
    fn top_n_archetypes(activations: &[f64; 22], n: usize) -> [u8; 3] {
        let mut indexed: Vec<(usize, f64)> = activations.iter().enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut result = [0u8; 3];
        for i in 0..n.min(3) {
            result[i] = indexed[i].0 as u8;
        }
        result
    }

    /// Generate initial position in a circular spread.
    fn initial_position(index: usize, radius: usize) -> [f32; 2] {
        let count = (index as f64 + 1.0).max(1.0);
        let angle = (count * 0.3) % std::f64::consts::TAU;
        let r = (radius as f64) * 0.3;
        [
            (angle.cos() * r) as f32,
            (angle.sin() * r) as f32,
        ]
    }

    /// Convert behavior state + spectrum position to velocity.
    /// Spectrum position affects movement style:
    ///   Low (space/time dominant) → deterministic, linear movement
    ///   High (time/space dominant) → probabilistic, erratic movement
    fn behavior_to_velocity(behavior: &BehaviorState, spectrum_position: f64) -> [f32; 2] {
        let speed = match behavior {
            BehaviorState::Seeking => 2.0,
            BehaviorState::Fleeing => 3.0,
            BehaviorState::Socializing => 1.0,
            BehaviorState::Contemplating => 0.2,
            BehaviorState::Leading => 1.5,
            BehaviorState::Idle => 0.0,
        };

        // Direction: use spectrum_position as a seed for direction
        let angle = spectrum_position * std::f64::consts::TAU;
        let randomness = spectrum_position as f32 * 0.5; // More time/space = more erratic

        [
            (angle.cos() as f32 * speed) + randomness - 0.25,
            (angle.sin() as f32 * speed) + randomness - 0.25,
        ]
    }
}
```

**Tests required**:
- `test_color_from_mind_dominant()` — mind archetypes high → reddish color
- `test_color_from_spirit_dominant()` — spirit archetypes high → bluish color
- `test_density_to_size()` — density 1→0.5, density 3→1.0, density 8→2.0
- `test_density_to_shape()` — density 3→Triangle, density 5→Diamond, density 7→Star
- `test_behavior_from_catalyst()` — A3 high → Seeking
- `test_behavior_from_tower()` — A11 high → Fleeing
- `test_behavior_from_hermit()` — A9 high → Contemplating
- `test_top_archetypes()` — returns correct sorted indices
- `test_initial_position_spread()` — entities spread in circle, not stacked
- `test_map_full_integration()` — full map() call produces non-default, non-zero properties

**Success criteria**:
- ✅ File compiles with zero warnings
- ✅ All 10 tests pass
- ✅ Module exported from `src/gui/mod.rs`
- ✅ No runtime panics on edge cases (empty activations, density 0, etc.)

---

### Task 3: Build EntityRenderer — Draw ObservableProperties with WGPU

**Why**: We have `ObservableProperties` and `ObservationMapper`. Now we need to actually render them. The existing `EntityRenderer` uses `EntityInstance` which has broken rendering. We build a new, clean renderer that draws `ObservableProperties` directly using instanced WGPU rendering.

**Files to create/modify**:
- **CREATE** `src/gui/renderer/observable_entity_renderer.rs` — new renderer
- **MODIFY** `src/gui/renderer/mod.rs` — add module export
- **MODIFY** `src/gui/renderer/entity_instance.rs` — no changes, we work alongside it

**Exact specification**:

The renderer must:
1. Accept `&[ObservableProperties]` as input
2. Use WGPU instanced rendering (one draw call for all entities)
3. Draw different shapes based on `EntityShape` enum
4. Apply `color`, `size`, and `glow` properties
5. Handle camera transformation (world → screen coordinates)
6. Support entity selection via mouse click hit-testing

```rust
// src/gui/renderer/observable_entity_renderer.rs

//! ObservableEntityRenderer — draws entities from ObservableProperties using WGPU.
//! Uses instanced rendering for performance (single draw call for all entities).

use crate::gui::observable_properties::*;
use crate::gui::camera::Camera;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct InstanceData {
    position: [f32; 2],
    size: f32,
    color: [f32; 3],
    glow: f32,
    shape: u32, // 0=Circle, 1=Triangle, 2=Square, 3=Diamond, 4=Hexagon, 5=Star
}

pub struct ObservableEntityRenderer {
    render_pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_count: usize,
    camera_bind_group: wgpu::BindGroup,
    // ... WGPU resources
}

impl ObservableEntityRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, camera_bind_group: wgpu::BindGroup) -> Self {
        // Create render pipeline with entity shader
        // Create instance buffer with MAP_WRITE | COPY_DST
        // ...
    }

    /// Update instance buffer with current entity data
    pub fn update(&mut self, queue: &wgpu::Queue, entities: &[ObservableProperties]) {
        let instances: Vec<InstanceData> = entities.iter().map(|e| InstanceData {
            position: e.position,
            size: e.size,
            color: e.color,
            glow: e.glow,
            shape: e.shape as u32,
        }).collect();

        self.instance_count = instances.len();
        let bytes = bytemuck::cast_slice(&instances);
        queue.write_buffer(&self.instance_buffer, 0, bytes);
    }

    /// Render all entities
    pub fn render(&self, render_pass: &mut wgpu::RenderPass<'_>, camera: &Camera) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.draw(0..6, 0..self.instance_count as u32); // 6 vertices per instance
    }

    /// Hit-test: find entity at screen position. Returns entity index or None.
    pub fn hit_test(&self, screen_pos: [f32; 2], camera: &Camera, entities: &[ObservableProperties]) -> Option<usize> {
        let world_pos = camera.screen_to_world(screen_pos);
        for (i, entity) in entities.iter().enumerate() {
            let dx = world_pos[0] - entity.position[0];
            let dy = world_pos[1] - entity.position[1];
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < entity.size {
                return Some(i);
            }
        }
        None
    }
}
```

**Shader** (`src/gui/shaders/entity.wgsl`):
```wgsl
struct CameraUniforms {
    view_proj: mat4x4f,
}
@group(0) @binding(0) var<uniform> camera: CameraUniforms;

struct InstanceData {
    position: vec2f,
    size: f32,
    color: vec3f,
    glow: f32,
    shape: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec3f,
    @location(1) glow: f32,
}

@vertex
fn vs_main(
    @location(0) vertex: vec2f,
    @location(1) @builtin(instance_index) instance_idx: u32,
) -> VertexOutput {
    // Each instance has its own data, vertex provides local offset
    // Shape generation happens here or via texture lookup
    ...
}
```

**Tests required**:
- `test_instance_data_layout()` — InstanceData is Pod and Zeroable
- `test_hit_test_finds_entity()` — clicking on entity returns correct index
- `test_hit_test_misses_entity()` — clicking far away returns None
- `test_update_writes_correct_data()` — buffer contains expected bytes after update

**Success criteria**:
- ✅ File compiles with zero warnings
- ✅ All 4 tests pass
- ✅ Shader compiles via `naga` validation
- ✅ Can render 1000 entities at 60fps (benchmark)

---

### Task 4: Connect SimulationRunnerAdapter → ObservationMapper → ObservableEntityRenderer

**Why**: Tasks 1-3 create isolated components. Task 4 wires them together so the GUI can display REAL simulation data. The `SimulationRunnerAdapter` already has `raw_entities: HashMap<EntityId, SubSubLogos>` — we extract archetype activations, consciousness, density, etc., map them through `ObservationMapper`, and feed the results to `ObservableEntityRenderer`.

**Files to modify**:
- **MODIFY** `src/gui/simulation_adapter.rs` — add `get_observable_properties()` method
- **MODIFY** `src/gui/application.rs` — use ObservableEntityRenderer alongside or instead of current entity renderer
- **MODIFY** `src/gui/mod.rs` — ensure all modules visible

**Exact specification**:

Add to `SimulationRunnerAdapter`:
```rust
use crate::gui::observation_mapper::ObservationMapper;
use crate::gui::observable_properties::ObservableProperties;

impl SimulationRunnerAdapter {
    /// Get observable properties for ALL entities.
    /// This is the bridge: extracts consciousness data → maps to game properties.
    pub fn get_observable_properties(&self) -> Vec<ObservableProperties> {
        self.raw_entities
            .values()
            .enumerate()
            .map(|(i, entity)| {
                let archetype_activations = entity.archetype_activations;
                let consciousness_level = entity.current_state.vibrational_state.coherence;
                let density = Self::density_to_u8(&entity.current_density);
                let spectrum_position = entity.spectrum_position;
                let polarization = entity.current_state.vibrational_state.polarity;

                ObservationMapper::map(
                    &archetype_activations,
                    consciousness_level,
                    density,
                    spectrum_position,
                    polarization,
                    i,
                )
            })
            .collect()
    }
}
```

In `GuiApplication::update()`:
```rust
// Each frame:
self.simulation.step(); // advance simulation
let entities = self.simulation.get_observable_properties(); // get game-facing data
self.entity_renderer.update(&self.queue, &entities); // send to GPU
// Render frame...
```

**Tests required**:
- Integration test: Create SimulationRunnerAdapter, call `get_observable_properties()`, verify:
  - Returns same count as `raw_entities.len()`
  - Each entity has non-zero position, non-gray color, non-zero size
  - Properties differ between entities (not all identical)

**Success criteria**:
- ✅ GUI shows entities as colored shapes (not just text stats)
- ✅ Each entity has unique color based on its archetype profile
- ✅ Entity shapes differ by density
- ✅ Clicking an entity highlights it (selection works)
- ✅ Entities move each frame (velocity from behavior state)
- ✅ 60fps with 100+ entities

---

### Task 5: Entity Detail Panel — Click to Inspect Consciousness State

**Why**: Being able to see entities is good. Being able to click one and see its full consciousness state (archetype activations, density, polarity, spectrum position, behavior state) is what makes this a simulation tool, not just a screensaver.

**Files to create/modify**:
- **CREATE** `src/gui/ui/entity_detail_panel.rs` — new EGUI panel
- **MODIFY** `src/gui/application.rs` — integrate panel, wire click → show panel

**Exact specification**:

The panel shows when an entity is selected:
```
┌── Entity Detail Panel ────────────────────────┐
│ Entity #42                                    │
│ Density: 3rd (Self-Aware)                     │
│ Shape: ▲ Triangle                             │
│                                               │
│ Consciousness: ████████░░ 82%                 │
│ Service:   ███████░░░ STO +67%               │
│ Behavior:  Seeking                            │
│                                               │
│ Top Archetypes:                               │
│   A3 (Catalyst):     █████████░ 91%          │
│   A10 (Catalyst):    ███████░░░ 72%          │
│   A17 (Catalyst):    ██████░░░░ 58%          │
│                                               │
│ Space/Time Ratio: 0.67 (Space-dominant)       │
│ Position: [12.3, -4.5]                        │
│ Velocity: [1.8, 0.3]                          │
│                                               │
│ [Close]                                       │
└───────────────────────────────────────────────┘
```

**Tests required**:
- Manual testing only (EGUI panels are UI)
- Verify: clicking entity opens panel with correct data
- Verify: panel shows all 22 archetype activation bars (collapsed to top 5 by default)
- Verify: "Close" button dismisses panel

**Success criteria**:
- ✅ Clicking an entity opens detail panel
- ✅ Panel shows all key consciousness metrics
- ✅ Archetype bars are color-coded (Mind=red, Body=green, Spirit=blue)
- ✅ Panel closes on [Close] button or clicking background

---

## Task Dependencies

```
Task 1 (ObservableProperties)  ──┐
                                  ├─── Task 2 (ObservationMapper) ──┐
Task 1 types needed here ────────┘                                  │
                                                                    ├─── Task 4 (Wiring) ──── Task 5 (Detail Panel)
Task 3 (EntityRenderer) ────────────────────────────────────────────┘
```

**Execution order**:
1. Task 1 + Task 3 can start in parallel (no dependency between them — Task 3 uses the types from Task 1 but they're simple data types that can be stubbed)
2. Task 2 depends on Task 1 types being defined
3. Task 4 depends on Tasks 1, 2, 3 all being complete
4. Task 5 depends on Task 4 (needs real data flow)

**Recommended parallel dispatch**:
- **Batch 1**: Task 1 + Task 3 (parallel)
- **Batch 2**: Task 2 (after Task 1)
- **Batch 3**: Task 4 (after Tasks 2+3)
- **Batch 4**: Task 5 (after Task 4)

---

## Verification Checklist

After all tasks complete:

### Functional Requirements
- [ ] GUI window opens and shows a dark canvas
- [ ] 100+ entities visible as colored shapes
- [ ] Each entity has unique color based on archetype profile
- [ ] Entity shapes differ by density (Circle, Triangle, Square, etc.)
- [ ] Entities move each frame with behavior-appropriate velocities
- [ ] Clicking an entity highlights it
- [ ] Entity detail panel opens on click with correct data
- [ ] Panel shows consciousness level, polarity, top archetypes
- [ ] 60fps with 100 entities

### Technical Requirements
- [ ] `cargo build --release --bin holonic_gui_complete` succeeds
- [ ] `cargo test --release` passes all new tests (17+ tests)
- [ ] `cargo clippy --all-targets --all-features` has no new warnings
- [ ] No new `unimplemented!()`, `todo!()`, or `panic!()` calls
- [ ] Code follows existing patterns (naming, imports, documentation)

### Integration Requirements
- [ ] ObservableProperties is populated from REAL SubSubLogos data (not stubbed)
- [ ] ObservationMapper correctly maps archetype activations to color
- [ ] Behavior state correctly reflects dominant archetype
- [ ] SimulationRunnerAdapter.get_observable_properties() works end-to-end

---

## Out of Scope (NOT in this phase)

These are explicitly excluded to maintain focus:
- ❌ Rapier3d physics integration (next phase)
- ❌ Multi-scale rendering (cosmic, planet views)
- ❌ Holographic optimization framework implementation
- ❌ Bevy ECS integration
- ❌ Civilization/settlement rendering
- ❌ Biology pipeline fixes
- ❌ Physics unification
- ❌ Network/multiplayer
- ❌ Sound/audio

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| WGPU version incompatibility | Medium | High | Use existing wgpu 0.19 that's already in Cargo.toml |
| Shader compilation fails | Medium | High | Start with simple circle shader, add shapes incrementally |
| EntityRenderer conflicts with existing EntityInstance renderer | Low | Medium | Use separate renderer instance, don't modify existing code |
| SubSubLogos archetype_activations type mismatch | Low | Medium | Check actual type before implementing mapper |
| GUI fails to compile due to dependency issues | Low | High | Build after each task, not all at once |
