# Phase 6: Physics Unification — Architecture Document

## Overview

Connect archetype-derived forces → Rapier rigid-body physics → WGPU rendering into a unified physics loop, with bidirectional feedback to the holographic field.

## Existing Foundation

The codebase already has significant physics infrastructure:

### PhysicalObservation (observation_layer.rs:125-134)
```rust
pub struct PhysicalObservation {
    pub entity_id: u64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub mass: f64,
    pub temperature: f64,
    pub charge: f64,
    pub health: f64,
    pub energy_level: f64,
}
```

### Existing Archetype → Physics Mappings
- **`compute_mass(base, archetypes, density)`** — Body archetypes (A7-A13) amplify mass; density multiplier
- **`compute_charge(archetypes)`** — Body archetype asymmetry (A7-A10 vs A11-A13), range [-1, 1]
- **`compute_temperature(spectrum, energy)`** — Quantum spectrum → higher temperature

### Archetype Ranges
| Domain | Indices | Count | Physics Role |
|--------|---------|-------|-------------|
| MIND | A0-A6 (0-7) | 7 | Cognitive forces (attention, memory, reasoning) |
| BODY | A7-A13 (7-14) | 7 | Physical mass, charge, inertia |
| SPIRIT | A14-A20 (14-21) | 7 | Resonance, cohesion, field interactions |
| FREE WILL | A21 | 1 | Stochastic perturbation source |

### holosim_bevy Integration
- `PhysicsComponent` already exists in colony_demo.rs with position, velocity, mass
- Bevy 0.18 ECS with ButtonInput for player control
- 60-entity colony demo with archetype-colored entities

## Architecture Design

### Data Flow

```
Archetype Activations [22]     Spectrum Position
         │                          │
         ▼                          ▼
┌─────────────────────────────────────────────────┐
│          ArchetypePhysicsMapper (6.2)            │
│  compute_force_vector(archetypes) → Force3D      │
│  compute_mass(archetypes, density) → f64         │
│  compute_charge(archetypes) → f64                │
│  compute_moment_of_inertia(archetypes) → f64     │
│  compute_damping(archetypes) → f64               │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│          RapierPhysicsWorld (6.3)               │
│  PhysicsWorld: rapier3d::prelude::PhysicsPipeline│
│  EntityBody: { rigid_body, collider, force }    │
│  apply_archetype_forces() → ExternalImpulse      │
│  step_simulation(dt) → positions, velocities     │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│          PhysicsObservation (6.4)               │
│  Extract positions, velocities, collisions       │
│  Update PhysicalObservation.position/.velocity   │
│  Feed into ObservationLayer                      │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│          Feedback Loop (6.6)                     │
│  Observed position → holographic field update    │
│  Velocity changes → archetype activation shift   │
│  Collision events → free will catalyst events    │
└─────────────────────────────────────────────────┘
```

### Module Organization

```
src/physics/
├── mod.rs                    // Module declaration, re-exports
├── archetype_physics.rs      // 6.2: Archetype → physical property mappings
├── physics_world.rs          // 6.3: Rapier physics world wrapper
├── integrator.rs             // 6.5: RK4 integration (replaces Euler)
├── feedback.rs               // 6.6: Physics → holographic field feedback
└── tests.rs                  // 6.7: Physics integration tests
```

### New Dependencies (Cargo.toml)
```toml
rapier3d = "0.19"    # 3D physics engine
rapier3d-math = "0.19"  # Math utilities (if needed)
```

## Detailed Design by Sub-Phase

### 6.2: Archetype Coefficients → Physical Properties

**File**: `src/physics/archetype_physics.rs`

```rust
pub struct ArchetypePhysics {
    /// Force vectors for each archetype (direction + magnitude)
    pub archetype_forces: [Force3D; 22],
    /// Global physics constants
    pub gravitational_constant: f64,
    pub electromagnetic_constant: f64,
    pub damping_coefficient: f64,
}

pub struct Force3D {
    pub fx: f64,  // x-component of force
    pub fy: f64,  // y-component of force
    pub fz: f64,  // z-component of force
}

impl ArchetypePhysics {
    /// Compute total force vector from archetype activation profile.
    /// Each archetype exerts a characteristic force direction:
    /// - A0-A6 (MIND): subtle cognitive forces (small magnitude, long range)
    /// - A7-A13 (BODY): strong physical forces (large magnitude, short range)
    /// - A14-A20 (SPIRIT): resonant forces (medium magnitude, field-wide)
    /// - A21 (FREE WILL): stochastic perturbation
    pub fn compute_force_vector(&self, archetype_profile: &[f64; 22]) -> Force3D;

    /// Compute mass from body archetype activation and density level.
    /// Extends existing ObservationLayer::compute_mass with physics-specific scaling.
    pub fn compute_mass(&self, archetype_profile: &[f64; 22], density: u8) -> f64;

    /// Compute electric charge from body archetype asymmetry.
    pub fn compute_charge(&self, archetype_profile: &[f64; 22]) -> f64;

    /// Compute moment of inertia for rotational dynamics.
    pub fn compute_moment_of_inertia(&self, archetype_profile: &[f64; 22]) -> f64;

    /// Compute damping coefficient from spirit archetype activation.
    /// Higher spirit = more damping (spiritual entities resist physical motion).
    pub fn compute_damping(&self, archetype_profile: &[f64; 22]) -> f64;
}
```

**Force Direction Mapping** (from cosmological architecture):

| Archetype | Force Direction | Physical Interpretation |
|-----------|----------------|------------------------|
| A0 (Memory) | (-1, 0, 0) | Pull toward past states |
| A1 (Perception) | (1, 0, 0) | Push toward novelty |
| A2 (Attention) | (0, 1, 0) | Focus force (vertical) |
| A3 (Reasoning) | (0, -1, 0) | Grounding force |
| A4 (Language) | (0, 0, 1) | Forward propagation |
| A5 (Logic) | (0, 0, -1) | Backward constraint |
| A6 (Pattern) | (1, 1, 0) | Spiral pattern force |
| A7 (Physical Form) | (0, 0, 0) | Rest mass (no force, only mass) |
| A8 (Vitality) | (0, 1, 0) | Upward life force |
| A9 (Motion) | (1, 0, 0) | Kinetic impulse |
| A10 (Sensation) | (-1, 0, 0) | Reactive force |
| A11 (Rest) | (0, -1, 0) | Damping force |
| A12 (Growth) | (0, 0, 1) | Expansion force |
| A13 (Decay) | (0, 0, -1) | Contraction force |
| A14 (Love/Connection) | (1, 1, 1) | Attractive force |
| A15 (Wisdom) | (-1, -1, -1) | Repulsive force |
| A16 (Harmony) | (0, 0, 0) | Equilibrium (zero force, stabilizes) |
| A17 (Transcendence) | (0, 1, 0) | Ascension force |
| A18 (Unity) | (0, 0, 0) | Cohesion (pulls toward center of mass) |
| A19 (Service) | (1, 0, 0) | Outward flow |
| A20 (Integration) | (-1, 0, 0) | Inward flow |
| A21 (Free Will) | stochastic | Random perturbation |

### 6.3: Rapier Rigid Body Integration

**File**: `src/physics/physics_world.rs`

```rust
use rapier3d::prelude::*;

pub struct PhysicsWorld {
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
}

pub struct PhysicsEntity {
    pub entity_id: u64,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub archetype_forces: Force3D,
}

impl PhysicsWorld {
    pub fn new() -> Self;
    pub fn add_entity(&mut self, entity_id: u64, position: [f64; 3], mass: f64, charge: f64) -> PhysicsEntity;
    pub fn remove_entity(&mut self, entity_id: u64);
    pub fn apply_archetype_force(&mut self, entity_id: u64, force: Force3D);
    pub fn apply_electromagnetic_force(&mut self, entity_id_a: u64, entity_id_b: u64, charge_a: f64, charge_b: f64);
    pub fn step(&mut self, dt: f64);
    pub fn get_entity_state(&self, entity_id: u64) -> Option<EntityPhysicsState>;
}

pub struct EntityPhysicsState {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub angular_velocity: [f64; 3],
    pub linear_acceleration: [f64; 3],
}
```

### 6.4: Physics → Rendering Pipeline

The rendering pipeline already exists in holosim_bevy:
- `PhysicsComponent` with position, velocity → Bevy `Transform`
- `HoloSimPlugin` syncs simulation state → Bevy ECS

Phase 6.4 adds:
- Extract positions/velocities from Rapier → update `PhysicalObservation`
- Feed updated `PhysicalObservation` into existing Bevy rendering pipeline
- Add collision event visualization (flash/particle effects)

### 6.5: RK4 Integration

Replace Rapier's default semi-implicit Euler with custom RK4 for archetype force integration:

```rust
pub fn rk4_step(
    state: &EntityPhysicsState,
    forces: &Force3D,
    mass: f64,
    dt: f64,
) -> EntityPhysicsState {
    // k1 = f(t, y)
    // k2 = f(t + dt/2, y + k1*dt/2)
    // k3 = f(t + dt/2, y + k2*dt/2)
    // k4 = f(t + dt, y + k3*dt)
    // y_new = y + (k1 + 2*k2 + 2*k3 + k4) * dt / 6
}
```

### 6.6: Feedback Loop

```
Rapier positions → update holographic field entity positions
                  → recalculate holographic connections
                  → modify archetype activations based on:
                     - Proximity to similar entities (resonance boost)
                     - Collision events (free will catalyst)
                     - Velocity changes (evolutionary rate modifier)
```

### 6.7: Physics Integration Tests

- **Mass positivity**: All entities have mass > 0
- **Energy conservation**: Closed system conserves total energy
- **Force symmetry**: Archetype forces produce bounded, stable trajectories
- **Collision response**: Entities bounce/merge correctly
- **Feedback stability**: Physics → holographic → archetype loop doesn't diverge
- **RK4 accuracy**: RK4 matches analytical solution within tolerance

## Implementation Order

1. **6.2 FIRST** — ArchetypePhysicsMapper with unit tests (no Rapier dependency)
2. **6.3 SECOND** — PhysicsWorld wrapper with Rapier (add rapier3d to Cargo.toml)
3. **6.4 THIRD** — Observation Layer sync (wire Rapier output → PhysicalObservation)
4. **6.5 FOURTH** — RK4 integration (optional, can use Rapier's default first)
5. **6.6 FIFTH** — Feedback loop (physics → holographic → archetype)
6. **6.7 LAST** — Integration tests

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Rapier 3D is heavy dependency | Medium | Start with rapier2d, upgrade to 3D later |
| Physics diverges from consciousness model | High | Keep archetype→physics mapping configurable |
| Performance degradation with 10K+ entities | High | Use Rapier's sleeping bodies, spatial hashing |
| RK4 complexity vs benefit | Low | Start with Rapier's default integrator, add RK4 later |
