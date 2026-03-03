# HOLOSIM INFINITE COMPLETION ROADMAP V4

## From Disconnected Subsystems to Living Cosmos

**Version:** 4.0
**Date:** February 18, 2026
**Status:** R&D Roadmap — Ready for Phase 1 Implementation
**Supersedes:** V1, V2, V3 roadmaps
**Reference Frameworks:**

- `COSMOLOGICAL-ARCHITECTURE.md`
- `HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md`
- `COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md`
- `HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md` (predecessor)

---

## TABLE OF CONTENTS

1. [Executive Summary](#executive-summary)
2. [Part I: Foundational Analysis](#part-i-foundational-analysis)
   - [Current Implementation Status](#current-implementation-status)
   - [Architectural Diagnosis](#architectural-diagnosis)
   - [The Integration Challenge](#the-integration-challenge)
3. [Part II: The 8-Phase Roadmap](#part-ii-the-8-phase-roadmap)
   - [Phase 1: Infinite Holographic Field Substrate](#phase-1-infinite-holographic-field-substrate-weeks-1-6)
   - [Phase 2: Procedural Cosmos Engine](#phase-2-procedural-cosmos-engine-weeks-7-12)
   - [Phase 3: Living Planets](#phase-3-living-planets-weeks-13-16)
   - [Phase 4: Biological Simulation Engine](#phase-4-biological-simulation-engine-weeks-17-22)
   - [Phase 5: Archetype-Driven Consciousness Engine](#phase-5-archetype-driven-consciousness-engine-weeks-23-26)
   - [Phase 6: Social Emergence & Civilization Engine](#phase-6-social-emergence--civilization-engine-weeks-27-32)
   - [Phase 7: Unified Simulation Loop](#phase-7-unified-simulation-loop-weeks-33-36)
   - [Phase 8: Interactive Experience Engine](#phase-8-interactive-experience-engine-weeks-37-40)
4. [Part III: R&D Investigation Areas](#part-iii-rd-investigation-areas)
5. [Part IV: Dependency Graph](#part-iv-dependency-graph)
6. [Part V: Success Criteria](#part-v-success-criteria)
7. [Part VI: Risk Analysis & Mitigations](#part-vi-risk-analysis--mitigations)
8. [Part VII: Performance Budget](#part-vii-performance-budget)
9. [Part VIII: Testing Strategy](#part-viii-testing-strategy)
10. [Conclusion](#conclusion)

---

## EXECUTIVE SUMMARY

HoloSim Infinite is a **335,727-line Rust codebase** spanning **446 source files** across
**102 module directories**. The simulation has substantial subsystems that **EXIST** but
are **DISCONNECTED**:

| Subsystem | Files | Lines | Status |
|-----------|-------|-------|--------|
| **Physics engine** (`scale_physics.rs` + `simulation_v3/`) | 73 | 102,070 | Not wired to entity experience |
| **Archetypes** (22 implementations) | 33 | 24,032 | Not driving entity behavior each tick |
| **Spectrum** (space/time ratio, veil) | 9 | 7,238 | Computed but not felt |
| **Entity Layer 7** (Sub-Sub-Logos) | 4 | 6,370 | Exists, but disconnected from matter chain |
| **Biology** (DNA, cells, ecosystems, epigenetics) | 5 | 3,556 | Described, not simulated each tick |
| **Veil mechanics** (density variation, piercing) | 5 | 3,856 | Implemented, not actively filtering experience |
| **Consciousness** (Archetype 22, free will, choice) | 5 | 3,115 | Exists, not orchestrated |
| **Sacred geometry** (Flower of Life, Fibonacci, etc.) | 8 | 3,936 | Decorative, not structural |
| **Compression / MERA** | 7 | 3,363 | Partial implementation |
| **Density evolution octave** | 5 | 3,494 | Transitions defined, not probabilistic |
| **Gaia** (atmosphere, consciousness) | 3 | 2,285 | Not connected to entities |
| **Matter** (particles) | 2 | 1,870 | Exists, no entity coupling |
| **Environment** (effects, bridge) | 3 | 870 | Stub implementation |
| **Game** (loop, mechanics) | 3 | 126 | Barely started |

**The core problem is simple: these are ISLANDS.** Each subsystem computes independently
but they don't feed into each other. The main simulation loop in `simulation_runner.rs`
(3,010 lines) orchestrates entity lifecycle, holographic field updates, and involution —
but it does NOT tick physics as felt experience, does NOT run biology each step, does NOT
let archetypes drive behavior, and does NOT make environments dynamic.

### The 15 Specific Gaps

| # | Gap | Severity | Phase to Fix |
|---|-----|----------|--------------|
| 1 | Bounded space (-1000..1000) instead of infinite holographic field | **Critical** | Phase 1 |
| 2 | Simulation runner not integrated with new modules | **Critical** | Phase 7 |
| 3 | Entities disconnected from matter chain (quantum→atomic→molecular→cellular) | **Critical** | Phase 4 |
| 4 | No observer-driven rendering / decompression | **High** | Phase 1 |
| 5 | Static environments (weather, terrain, atmosphere created once, never updated) | **High** | Phase 3 |
| 6 | Biology not alive (DNA, cells, ecosystems described but not running each tick) | **Critical** | Phase 4 |
| 7 | No gameplay loop / interactive depth / Dwarf-Fortress-level observation | **High** | Phase 8 |
| 8 | `UniversalTemplate` not applied as primary pattern across all components | **Medium** | Phase 1 |
| 9 | No inter-entity social dynamics (relationships, communication, groups) | **High** | Phase 6 |
| 10 | Physics not wired to entity experience (gravity, temperature, radiation unfelt) | **Critical** | Phase 2 |
| 11 | No real Time/Space qualitative experience difference from Space/Time | **High** | Phase 5 |
| 12 | Archetype system (22 implementations, 24K lines) not driving entity behavior | **Critical** | Phase 5 |
| 13 | No procedural universe generation (stars, planets, galaxies from field coherence) | **High** | Phase 2 |
| 14 | No energy economy (stellar → planetary → biological → entity energy flow) | **Medium** | Phase 3 |
| 15 | Wanderer/Social Memory Complex/Harvest mechanics missing or disconnected | **High** | Phase 6 |

### The Vision

Transform HoloSim Infinite from a collection of impressive but isolated modules into a
**Dwarf-Fortress-depth consciousness evolution simulator** where:

- Reality **self-generates** from Intelligent Infinity through the Three Primal Distortions
- Stars ignite, planets form, life emerges — **without scripting**
- Entities have **actual biological bodies** that metabolize, age, and die
- Behavior **emerges** from 22-archetype interference — no behavior trees
- Civilizations form, develop technology, discover governance
- Social Memory Complexes crystallize at 4th density
- Harvest occurs based on accumulated polarity
- All of this is **observable** at any zoom level, from quantum field to cosmic web

---

## PART I: FOUNDATIONAL ANALYSIS

### Current Implementation Status

#### Directory-by-Directory Audit

```
src/
├── adapters/              — Physical adapter layer (bridges old → new)
├── archetypes/            — 33 files, 24,032 lines
│   ├── archetype_1.rs through archetype_22.rs (22 individual archetypes)
│   ├── interference.rs    — Archetype interference patterns
│   ├── mind_body_spirit.rs — Complex decomposition
│   └── mod.rs             — Module coordination
├── attractor_fields/      — Spiritual gravity attractors
├── attractors/            — Attractor pattern system
├── bin/                   — Binary targets
├── biology/               — 5 files, 3,556 lines
│   ├── cellular_emergence.rs  — Cell formation from molecular substrate (842 lines)
│   ├── dna_system.rs          — DNA encoding and replication (844 lines)
│   ├── ecosystem_dynamics.rs  — Ecosystem interactions (850 lines)
│   ├── epigenetic_system.rs   — Epigenetic modification (868 lines)
│   └── mod.rs                 — Module exports
├── compression/           — 7 files, 3,363 lines (MERA network, fractal cache)
├── consciousness/         — 5 files, 3,115 lines
│   ├── archetype22.rs         — The Choice archetype (711 lines)
│   ├── choice_operator.rs     — Choice mechanics (695 lines)
│   ├── free_will.rs           — Free will kernel (742 lines)
│   ├── possibility_space.rs   — Branching possibilities (668 lines)
│   └── mod.rs
├── cosmological/          — Cosmological layer mapping
├── entity_layer7/         — 4 files, 6,370 lines
│   ├── layer7.rs              — Sub-Sub-Logos entity definition
│   ├── dna_encoding.rs        — DNA pattern encoding
│   ├── holographic_blueprint.rs — Entity holographic blueprint
│   └── mod.rs
├── environment/           — 3 files, 870 lines (STUB — barely implemented)
├── evolution/             — Teleological evolution, intelligent infinity
├── evolution_density_octave/ — 5 files, 3,494 lines (density transitions)
├── foundation/            — 6 files, 2,432 lines (Layers 0-3: Violet→Green)
├── gaia/                  — 3 files, 2,285 lines (atmospheric dynamics, Gaia consciousness)
├── game/                  — 3 files, 126 lines (EMPTY — placeholder only)
├── gui/                   — GUI rendering
├── holographic/           — Holographic field operations
├── light/                 — Light/Love field
├── matter/                — 2 files, 1,870 lines (particle emergence)
├── memory/                — Memory systems
├── multi_scale/           — Multi-scale operations
├── physics/               — 2 files, 939 lines (physics primitives)
├── sacred_geometry/       — 8 files, 3,936 lines (geometric patterns)
├── simulation_v3/         — 73 files, 102,070 lines (THE MAIN ENGINE)
│   ├── simulation_runner.rs   — Main loop (3,010 lines)
│   ├── scale_physics.rs       — Scale-specific physics (12,389 lines)
│   ├── holographic_field.rs   — Field management (2,929 lines)
│   ├── entity_lifecycle.rs    — Entity birth/death/evolution
│   ├── catalyst_system.rs     — Catalyst processing
│   ├── collective_dynamics.rs — Collective behavior
│   ├── density_mechanics.rs   — Density transitions
│   └── ... 65 more files
├── spatial/               — 3 files, 954 lines (spectrum spatial, veil transform)
├── spectrum/              — 9 files, 7,238 lines (Orange/Red/Yellow realms, archetype mind)
├── template/              — 4 files, 1,775 lines (UniversalTemplate, migration)
├── unified/               — 6 files, 950 lines (unified field, emergence views)
├── veil/                  — 5 files, 3,856 lines (mechanism, piercing, density variation)
│
├── physics_engine.rs      — 1,083 lines (standalone physics)
├── social_memory.rs       — 577 lines (Social Memory Complex)
├── veil_mechanics.rs      — 752 lines (veil filtering)
├── harvestability.rs      — 475 lines (harvest calculations)
├── entity.rs              — Core entity definition
├── energy_flow_system.rs  — Energy flow chains
├── energy_ray_centers.rs  — 7 energy center (chakra) system
└── ... 40+ more standalone modules
```

**Total: 446 files, 335,727 lines of Rust**

#### What Works Today

Running `cargo run --release --bin holonic_realms -- --entities 128 --steps 100`:

1. **Involution sequence** runs: Violet → Indigo → Blue → Green → Yellow → Orange → Red → Entity
2. **Entities are created** with EntityType, EntityId, density, spectrum configuration
3. **Holographic field** updates each tick (Free Will perturbation, Love/Light propagation)
4. **Entity lifecycle** manages birth, evolution steps, density transitions
5. **Scale physics** computes forces at each scale level
6. **Statistics** are tracked and reported

#### What Doesn't Work (The 15 Gaps in Detail)

**Gap 1: Bounded Space**

The current coordinate system uses `Coordinate3D` with hard bounds:

```rust
// CURRENT — bounded, non-holographic
pub struct Coordinate3D {
    pub x: f64, // Range: -1000.0 to 1000.0
    pub y: f64, // Range: -1000.0 to 1000.0
    pub z: f64, // Range: -1000.0 to 1000.0
}
```

This means the entire universe fits in a 2000×2000×2000 cube. Entities near the boundary
have asymmetric interactions. There is no way to represent galactic-scale structures
alongside molecular-scale detail. The holographic principle demands that space itself
emerges from observation, not that it pre-exists as a box.

**Gap 2: Disconnected Simulation Runner**

`simulation_runner.rs` coordinates a subset of the system:

```rust
// CURRENT — simulation_runner.rs orchestrates these:
// ✅ InvolutionSequenceRunner
// ✅ EntityLifecycleManager
// ✅ HolographicFieldManager
// ✅ PhysicalAdapter
// ✅ CatalystManager
// ✅ CollectiveDynamicsManager
// ✅ EnvironmentalInteractionManager
// ✅ InterScaleInteractionManager
//
// ❌ NOT orchestrated:
// ❌ Biology (CellEngine, DNA, Ecosystems)
// ❌ Gaia (AtmosphericDynamics, GaiaConsciousness)
// ❌ Physics-to-Experience bridge
// ❌ Archetype-driven behavior
// ❌ Social dynamics
// ❌ Harvest system
// ❌ Wanderer system
// ❌ Dynamic environment updates
```

**Gap 3: Entities Disconnected from Matter**

The matter emergence chain (quantum → atomic → molecular → cellular) exists in
`src/matter/particle.rs` and `src/biology/cellular_emergence.rs`, but entities don't
HAVE bodies made of these simulated particles. An entity's "physical form" is a set of
numeric attributes, not an actual composition of simulated cells.

**Gap 4: No Observer-Driven Decompression**

The MERA compression network exists in `src/compression/mera_network.rs` (partial), but
there is no observer that drives which regions to decompress. The entire simulation is
fully expanded at all times, wasting memory on regions no one is looking at.

**Gap 5: Static Environments**

`src/environment/environmental_effects.rs` defines environment types (temperature,
pressure, radiation) but they are set once during creation and never updated. Weather
doesn't change. Seasons don't cycle. Terrain doesn't erode.

**Gap 6: Biology Not Alive**

The biology module (`src/biology/`) has 3,556 lines defining DNA replication, cell
division, ecosystem dynamics, and epigenetics. But none of it runs each tick. Cells
don't divide. Organisms don't metabolize. Species don't evolve through selection.

**Gap 7: No Gameplay Loop**

The `src/game/` directory has 3 files totaling 126 lines — essentially empty stubs.
There is no interactive observation system, no entity inspection, no time controls,
no zoom levels, no event narration.

**Gap 8: UniversalTemplate Not Universal**

`src/template/` defines a `UniversalTemplate` pattern (1,775 lines), but most modules
don't use it. Entities, particles, planets, and stars each define their own structures
instead of being template instantiations.

**Gap 9: No Social Dynamics**

`src/social_memory.rs` (577 lines) defines Social Memory Complex structures, but there
is no relationship system, no communication model, no group formation, no civilization
emergence. Entities are solitary agents.

**Gap 10: Physics Unfelt**

`src/simulation_v3/scale_physics.rs` (12,389 lines) computes physics at every scale —
gravitational, electromagnetic, nuclear, quantum. But entities don't FEEL these forces.
There is no `PhysicsExperience` struct that translates computed forces into subjective
entity experience (temperature, pressure, day/night, seasons).

**Gap 11: No Time/Space Experience**

The veil module (`src/veil/`, 3,856 lines) defines veil mechanics, density variation,
and piercing — but there is no qualitative difference in how entities experience reality
above vs. below the Veil. In Space/Time, entities should have free spatial movement and
linear time. In Time/Space, entities should have fixed spatial locus and free temporal
navigation. This duality is defined but not experientially simulated.

**Gap 12: Archetypes Not Driving Behavior**

The archetype system (`src/archetypes/`, 24,032 lines) implements all 22 archetypes with
interference patterns, mind/body/spirit complex decomposition, and activation coefficients.
But entity behavior is NOT computed from archetype interference. Entities don't process
catalysts through the archetype cycle (Matrix → Potentiator → Catalyst → Experience →
Significator → Transformation → Great Way).

**Gap 13: No Procedural Universe**

Stars, planets, and galaxies are created by explicit construction, not by field coherence
producing gravitational collapse, stellar ignition, and planetary accretion. The universe
should self-generate.

**Gap 14: No Energy Economy**

There is no chain from stellar radiation → planetary surface irradiance → photosynthetic
capture → food web → entity energy availability. `src/energy_flow_system.rs` defines
some flow structures but doesn't implement the full star-to-entity energy pipeline.

**Gap 15: Wanderer/SMC/Harvest Disconnected**

`src/harvestability.rs` (475 lines) defines harvest calculations. `src/social_memory.rs`
(577 lines) defines Social Memory Complex structures. But these are standalone calculations
that don't integrate with the simulation loop, don't trigger actual density transitions,
and don't create experiential consequences.

---

### Architectural Diagnosis

#### The Core Issue: Subsystems Are Islands

The codebase resembles a **museum of simulation components**: each exhibit is individually
impressive, but they don't interact. The physics engine doesn't warm the entities. The
biology module doesn't feed the physics. The archetype system doesn't drive behavior.

The current data flow looks like this:

```
┌─────────────┐    ┌──────────────┐    ┌──────────────┐
│  Archetypes │    │   Physics    │    │   Biology    │
│  (24K lines)│    │  (12K lines) │    │  (3.5K lines)│
│             │    │              │    │              │
│  Computes   │    │  Computes    │    │  Defines     │
│  archetype  │    │  forces at   │    │  DNA, cells, │
│  coefficients    │  every scale │    │  ecosystems  │
│             │    │              │    │              │
│  OUTPUT: ── │    │  OUTPUT: ── │    │  OUTPUT: ── │
│  f64 array  │    │  force vecs  │    │  structs     │
│  (unused)   │    │  (unused by  │    │  (not ticked)│
│             │    │   entities)  │    │              │
└─────────────┘    └──────────────┘    └──────────────┘

         │                 │                   │
         │                 │                   │
         ▼                 ▼                   ▼
    ┌────────────────────────────────────────────────┐
    │          simulation_runner.rs (3K lines)       │
    │                                                │
    │  Ticks: involution, lifecycle, field, adapter   │
    │  Does NOT read: archetypes, physics-as-felt,   │
    │                 biology, environment, social    │
    └────────────────────────────────────────────────┘
```

**The target data flow should look like this:**

```
                    ┌─────────────────────┐
                    │  Intelligent Infinity │
                    │  (Three Primal       │
                    │   Distortions)       │
                    └──────────┬──────────┘
                               │
                    ┌──────────▼──────────┐
                    │  Holographic Field   │
                    │  (Observer-Driven    │
                    │   Decompression)     │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
    ┌─────────▼──────┐  ┌─────▼─────┐  ┌───────▼───────┐
    │  Cosmos Engine  │  │  Physics  │  │  Energy Flow  │
    │  Stars ignite,  │  │  Forces   │  │  Star→Planet  │
    │  planets form   │  │  computed │  │  →Bio→Entity  │
    └─────────┬──────┘  └─────┬─────┘  └───────┬───────┘
              │               │                 │
              │        ┌──────▼──────┐          │
              └───────►│  Planets    │◄─────────┘
                       │  Geology    │
                       │  Hydrology  │
                       │  Atmosphere │
                       │  Weather    │
                       └──────┬──────┘
                              │
                       ┌──────▼──────┐
                       │  Biology    │
                       │  Cells tick │
                       │  Organisms  │
                       │  Evolution  │
                       └──────┬──────┘
                              │
              ┌───────────────┼───────────────┐
              │               │               │
    ┌─────────▼──────┐ ┌─────▼─────┐  ┌──────▼──────┐
    │  Archetype     │ │  Embodied │  │  Physics    │
    │  Processing    │ │  Entity   │  │  Experience │
    │  22 archetypes │ │  body+mind│  │  felt forces│
    │  drive choice  │ │  coupling │  │  temperature│
    └─────────┬──────┘ └─────┬─────┘  └──────┬──────┘
              │              │                │
              └──────────────┼────────────────┘
                             │
                      ┌──────▼──────┐
                      │   Social    │
                      │ Dynamics    │
                      │ Civilization│
                      │ SMC/Harvest │
                      └──────┬──────┘
                             │
                      ┌──────▼──────┐
                      │  Observer   │
                      │  Camera     │
                      │  Multi-Scale│
                      │  Inspection │
                      └─────────────┘
```

### The Integration Challenge

Every subsystem needs to connect to at least 2-3 others. Here is the dependency web:

| Subsystem | Needs Input From | Provides Output To |
|-----------|------------------|--------------------|
| **Holographic Field** | Observer position, entity feedback | All subsystems (substrate) |
| **Cosmos Engine** | Field coherence peaks | Planets, Energy Flow |
| **Stellar Physics** | Field, nuclear physics | Planets (radiation), Energy Flow |
| **Planet Systems** | Star radiation, orbital mechanics | Biology, Entity experience |
| **Geology** | Internal heat, planet age | Terrain for biology, resources |
| **Hydrology** | Atmosphere, tidal forces | Biology (water), entity experience |
| **Atmosphere** | Solar radiation, biology (O2/CO2) | Weather, entity experience |
| **Biology** | Environment state, energy input | Entity bodies, ecosystem services |
| **Cell Engine** | Energy, environment | Organisms (constituent cells) |
| **Evolution** | Population dynamics, selection | Species diversity, new body plans |
| **Archetypes** | Catalyst events, entity state | Entity behavior, polarity shifts |
| **Consciousness** | Sensory input, archetype output | Choices, field feedback |
| **Veil** | Entity density | Memory filtering, perception limits |
| **Social** | Entity interactions | Groups, civilizations, SMC |
| **Harvest** | Entity polarity, cycle timing | Density transitions |
| **Energy Flow** | Star luminosity, planet albedo | Available energy for biology |
| **Observer** | User input | Field decompression regions |

The challenge is not building new subsystems — it's **wiring 335,000 lines of existing
code into a coherent data flow** where each tick propagates from field through cosmos
through planet through biology through consciousness through social and back to field.

---

## PART II: THE 8-PHASE ROADMAP

---

### PHASE 1: INFINITE HOLOGRAPHIC FIELD SUBSTRATE (Weeks 1-6)

**Goal:** Remove bounded coordinates. Space emerges from field coherence. Only observed
regions decompress. The `UniversalTemplate<T>` becomes the primary type for all components.

**Files to create/modify:**

- `src/holographic/field_address.rs` — New addressing system
- `src/holographic/observer_driven_field.rs` — Observer-driven decompression
- `src/holographic/mera_integration.rs` — MERA compression for field storage
- `src/template/universal_component.rs` — `HolographicComponent<T>` wrapper
- Migrate all entity creation to use `HolographicAddress` instead of `Coordinate3D`

**Estimated effort:** 8,000-12,000 new lines, 3,000-5,000 lines modified

---

#### 1.1 Field-Relative Addressing

Replace `Coordinate3D(-1000..1000)` with `HolographicAddress`:

```rust
/// Position is an address in the holographic field, not absolute coordinates.
/// Space doesn't exist until observed — it decompresses from field state.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Reality is a holographic projection. Each point contains the whole."
///
/// The address encodes a path through the MERA tensor network from the
/// coarsest (cosmic) scale down to the finest (quantum) scale. Only the
/// levels that have been decompressed by an observer have spatial extent.
pub struct HolographicAddress {
    /// Coherence gradient path from field origin.
    /// Each step narrows the address to a smaller region of the field.
    /// Think of it as a zip code that gets more specific with each digit.
    pub coherence_path: Vec<CoherenceStep>,

    /// Scale level this address resolves to.
    /// Quantum=0 (10^-35 m), Atomic=1, Molecular=2, Cellular=3,
    /// Biological=4, Planetary=5, Stellar=6, Cosmic=7
    pub scale: ScaleLevel,

    /// Local offset within the decompressed region.
    /// Only meaningful after decompression — within a decompressed region,
    /// entities can move freely using standard 3D vectors.
    pub local_offset: Vector3,
}

/// A single step in the coherence path — narrows the address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoherenceStep {
    /// Which density band this step addresses (1-8).
    /// Higher density bands contain "more" of the holographic information.
    pub density_band: u8,

    /// Octant selection (0-7) — which spatial subdivision at this level.
    /// This creates an octree-like addressing scheme, but the octants are
    /// defined by coherence gradients, not Cartesian axes.
    pub octant: u8,

    /// Depth within this density band (how many subdivisions deep).
    /// Maximum depth determines spatial resolution at this scale.
    pub depth: u16,
}

/// Scale level enumeration with physical extent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScaleLevel {
    Quantum = 0,    // 10^-35 m (Planck scale)
    Atomic = 1,     // 10^-10 m (Angstrom)
    Molecular = 2,  // 10^-9 m (Nanometer)
    Cellular = 3,   // 10^-5 m (Micrometer)
    Biological = 4, // 10^0 m (Meter)
    Planetary = 5,  // 10^7 m (Earth radius)
    Stellar = 6,    // 10^11 m (AU)
    Cosmic = 7,     // 10^22 m (Megaparsec)
}

impl HolographicAddress {
    /// Create an address at the cosmic root (most compressed level).
    pub fn cosmic_origin() -> Self {
        Self {
            coherence_path: vec![],
            scale: ScaleLevel::Cosmic,
            local_offset: Vector3::zero(),
        }
    }

    /// Refine this address to a more specific region.
    /// Returns a new address one level deeper in the coherence hierarchy.
    pub fn refine(&self, step: CoherenceStep) -> Self {
        let mut new_path = self.coherence_path.clone();
        new_path.push(step);
        Self {
            coherence_path: new_path,
            scale: self.scale.finer(),
            local_offset: Vector3::zero(),
        }
    }

    /// Compute distance between two addresses.
    /// Addresses in the same decompressed region: Euclidean distance.
    /// Addresses in different regions: coherence distance through MERA network.
    pub fn distance_to(&self, other: &HolographicAddress) -> f64 {
        // Find common prefix in coherence paths
        let common_depth = self.common_prefix_length(other);
        if common_depth == self.coherence_path.len()
            && common_depth == other.coherence_path.len()
        {
            // Same region: Euclidean distance
            (self.local_offset - other.local_offset).magnitude()
        } else {
            // Different regions: scale-aware distance through MERA
            let scale_factor = 10.0_f64.powi(
                (self.coherence_path.len() as i32 - common_depth as i32).abs(),
            );
            scale_factor * self.coherence_metric(other, common_depth)
        }
    }

    fn common_prefix_length(&self, other: &HolographicAddress) -> usize {
        self.coherence_path
            .iter()
            .zip(other.coherence_path.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }

    fn coherence_metric(&self, other: &HolographicAddress, common_depth: usize) -> f64 {
        // Coherence distance: how many MERA levels apart
        let self_remaining = self.coherence_path.len() - common_depth;
        let other_remaining = other.coherence_path.len() - common_depth;
        ((self_remaining + other_remaining) as f64).sqrt()
    }
}

impl ScaleLevel {
    /// Get the next finer scale level.
    pub fn finer(&self) -> Self {
        match self {
            Self::Cosmic => Self::Stellar,
            Self::Stellar => Self::Planetary,
            Self::Planetary => Self::Biological,
            Self::Biological => Self::Cellular,
            Self::Cellular => Self::Molecular,
            Self::Molecular => Self::Atomic,
            Self::Atomic => Self::Quantum,
            Self::Quantum => Self::Quantum, // Already at finest
        }
    }

    /// Get the characteristic length scale in meters.
    pub fn characteristic_length(&self) -> f64 {
        match self {
            Self::Quantum => 1.616e-35,
            Self::Atomic => 1.0e-10,
            Self::Molecular => 1.0e-9,
            Self::Cellular => 1.0e-5,
            Self::Biological => 1.0,
            Self::Planetary => 6.371e6,
            Self::Stellar => 1.496e11,
            Self::Cosmic => 3.086e22,
        }
    }
}
```

---

#### 1.2 Observer-Driven Decompression

Only decompress field regions where an observer is looking. Unobserved regions remain
compressed in the MERA network, consuming O(1) memory per region.

```rust
/// Observer-driven holographic field.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The holographic principle means the field is everywhere simultaneously,
/// but only manifests (decompresses) where consciousness observes."
///
/// This is the SUBSTRATE for all simulation. Everything exists as patterns
/// in this field. Stars, planets, entities, particles — all are coherence
/// structures within the field.
pub struct ObserverDrivenField {
    /// Compressed field state (MERA tensor network).
    /// The entire universe is encoded here in O(n log n) memory.
    /// Unobserved regions cost O(1) per region.
    compressed: MeraNetwork,

    /// Currently decompressed regions, keyed by observer ID.
    /// Only these regions have full spatial detail.
    active_regions: HashMap<ObserverId, DecompressedRegion>,

    /// All registered observers (entities with awareness + external camera).
    observers: Vec<Observer>,

    /// Three Primal Distortion field strengths (global).
    free_will_field: FreeWillField,
    love_field: LoveField,
    light_field: LightField,

    /// Field coherence statistics (for detecting emergent structures).
    coherence_stats: CoherenceStatistics,
}

/// A region of the field that has been decompressed for observation.
pub struct DecompressedRegion {
    /// The holographic address range this region covers.
    pub address_range: AddressRange,

    /// Scale level of decompression.
    pub scale: ScaleLevel,

    /// The decompressed field values within this region.
    /// Grid resolution depends on scale level.
    pub field_values: FieldGrid,

    /// Entities present in this region.
    pub entity_ids: Vec<EntityId>,

    /// Tick count since last access (for eviction).
    pub idle_ticks: u64,

    /// Maximum idle ticks before eviction.
    pub max_idle_ticks: u64,
}

/// An observer that drives field decompression.
pub struct Observer {
    pub id: ObserverId,
    pub address: HolographicAddress,
    pub field_of_view: FieldOfView,
    pub scale: ScaleLevel,
    pub observer_type: ObserverType,
}

pub enum ObserverType {
    /// External camera (player/user observation point)
    ExternalCamera,
    /// Self-aware entity (3rd density+) — observes surroundings
    SelfAwareEntity(EntityId),
    /// Collective consciousness — observes broader patterns
    CollectiveObserver(Vec<EntityId>),
}

pub struct FieldOfView {
    /// Angular extent of observation (radians)
    pub angular_extent: f64,
    /// Depth of observation (how many scale levels to decompress)
    pub depth: u8,
    /// Resolution (how fine-grained the decompression)
    pub resolution: u32,
}

impl ObserverDrivenField {
    /// Create a new field from initial conditions.
    pub fn new(config: FieldConfig) -> Self {
        let compressed = MeraNetwork::from_initial_conditions(&config);
        Self {
            compressed,
            active_regions: HashMap::new(),
            observers: vec![],
            free_will_field: FreeWillField::new(config.free_will_strength),
            love_field: LoveField::new(config.love_strength),
            light_field: LightField::new(config.light_strength),
            coherence_stats: CoherenceStatistics::default(),
        }
    }

    /// Main tick: update field state, manage decompression.
    pub fn tick(&mut self, dt: f64) {
        // 1. Apply Three Primal Distortions to compressed field
        self.compressed.apply_free_will(&self.free_will_field, dt);
        self.compressed.apply_love_attraction(&self.love_field, dt);
        self.compressed.apply_light_propagation(&self.light_field, dt);

        // 2. Evict regions no longer observed (compress them back)
        self.evict_unobserved();

        // 3. Decompress newly observed regions
        for observer in &self.observers {
            let address_range = observer.compute_address_range();
            if !self.is_region_active(&observer.id) {
                let decompressed = self.compressed.decompress_region(
                    &address_range,
                    observer.scale,
                    observer.field_of_view.resolution,
                );
                self.active_regions.insert(
                    observer.id,
                    DecompressedRegion {
                        address_range,
                        scale: observer.scale,
                        field_values: decompressed,
                        entity_ids: vec![],
                        idle_ticks: 0,
                        max_idle_ticks: 100,
                    },
                );
            }
        }

        // 4. Update coherence statistics (for emergent structure detection)
        self.coherence_stats = self.compressed.compute_coherence_stats();
    }

    /// Evict decompressed regions that haven't been observed recently.
    fn evict_unobserved(&mut self) {
        let to_evict: Vec<ObserverId> = self
            .active_regions
            .iter()
            .filter(|(_, region)| region.idle_ticks > region.max_idle_ticks)
            .map(|(id, _)| *id)
            .collect();

        for id in to_evict {
            if let Some(region) = self.active_regions.remove(&id) {
                // Compress region back into MERA network
                self.compressed.recompress_region(
                    &region.address_range,
                    &region.field_values,
                );
            }
        }
    }

    fn is_region_active(&self, observer_id: &ObserverId) -> bool {
        self.active_regions.contains_key(observer_id)
    }

    /// Detect coherence peaks in the field — these become particles, stars, etc.
    pub fn detect_coherence_peaks(
        &self,
        threshold: CoherenceThreshold,
    ) -> Vec<CoherencePeak> {
        self.compressed.find_peaks(threshold)
    }

    /// Register a new observer (entity gains self-awareness, camera moves).
    pub fn register_observer(&mut self, observer: Observer) {
        self.observers.push(observer);
    }

    /// Remove an observer (entity loses awareness, camera leaves).
    pub fn remove_observer(&mut self, observer_id: &ObserverId) {
        self.observers.retain(|o| o.id != *observer_id);
    }

    /// Get the decompressed field view for a specific observer.
    pub fn get_observer_view(&self, observer_id: &ObserverId) -> Option<&DecompressedRegion> {
        self.active_regions.get(observer_id)
    }

    /// Apply entity feedback to the field (consciousness modifies reality).
    pub fn apply_entity_feedback(
        &mut self,
        entity_id: EntityId,
        address: &HolographicAddress,
        feedback: FieldFeedback,
    ) {
        // Entity consciousness modifies the field at their location
        self.compressed.apply_perturbation(
            address,
            feedback.coherence_delta,
            feedback.polarity_influence,
        );
    }
}
```

---

#### 1.3 UniversalTemplate\<T\> as Primary Type

Every component in the simulation — entity, particle, planet, star, galaxy — wraps in the
same holographic template. This implements the holographic principle: each part contains
the whole architecture.

```rust
/// ALL components use this template. Entity, Particle, Planet, Star, Galaxy.
///
/// From COSMOLOGICAL-ARCHITECTURE.md section 2.6:
/// "Each entity contains the holographic blueprint with the complete
/// evolutionary trajectory. They 'transcend and include' all previous layers."
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Every component follows the same UniversalTemplate. We don't build each
/// from scratch — we instantiate the template with different parameters."
pub struct HolographicComponent<T: ComponentData> {
    /// Shared reference to the holographic field (Arc — O(1) memory per ref).
    /// Every component can read field state at its location.
    field: Arc<ObserverDrivenField>,

    /// Holographic address (replaces Coordinate3D).
    /// This IS the component's position — an address in the field, not
    /// an absolute coordinate in a pre-existing space.
    address: HolographicAddress,

    /// Spectrum configuration (from Layer 4 — Yellow Realm).
    /// Determines the space/time ↔ time/space ratio for this component.
    spectrum: SpectrumConfiguration,

    /// 22 archetype activation coefficients (from Layer 6 — Red Realm).
    /// These are the "DNA" of consciousness — they shape how this component
    /// processes catalyst and makes choices.
    archetypes: [f64; 22],

    /// Current density level (1st through 8th).
    /// Determines which experiences are available and what evolution means.
    density: Density,

    /// Free will seed (unique per component).
    /// Seeds the probabilistic evolution engine — no two components evolve
    /// identically even from the same initial conditions.
    free_will_seed: u64,

    /// Component-specific data (Entity, Particle, Star, Planet, etc.)
    data: T,
}

/// Trait that all component data types must implement.
pub trait ComponentData: Send + Sync + Clone + std::fmt::Debug {
    /// What kind of component this is.
    fn component_type(&self) -> ComponentType;

    /// Per-tick update. Receives the local field state and time delta.
    fn tick(&mut self, field_view: &FieldView, dt: f64);

    /// Density of this component (derived from its evolution state).
    fn current_density(&self) -> Density;

    /// Whether this component is an observer (affects field decompression).
    fn is_observer(&self) -> bool;

    /// Consciousness level (0.0 = no awareness, 1.0 = full self-awareness).
    fn consciousness_level(&self) -> f64;
}

/// Component types in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// Quantum field excitation
    Particle,
    /// Collection of particles (atom, molecule)
    MatterCluster,
    /// Living cell
    Cell,
    /// Multicellular organism
    Organism,
    /// Self-aware being (3rd density+)
    Entity,
    /// Moon, asteroid
    SmallBody,
    /// Planet or dwarf planet
    Planet,
    /// Star
    Star,
    /// Stellar system (star + planets)
    StellarSystem,
    /// Galaxy
    Galaxy,
    /// Social Memory Complex
    SocialMemoryComplex,
}

/// Local field state visible to a component at its address.
pub struct FieldView {
    /// Local coherence (how "real" space is here)
    pub coherence: f64,
    /// Local free will distortion strength
    pub free_will_strength: f64,
    /// Local love/attraction strength
    pub love_strength: f64,
    /// Local light/awareness strength
    pub light_strength: f64,
    /// Nearby coherence peaks (other components)
    pub nearby_peaks: Vec<NearbyPeak>,
    /// Current scale of observation
    pub observer_scale: ScaleLevel,
}

impl<T: ComponentData> HolographicComponent<T> {
    /// Create a new holographic component.
    pub fn new(
        field: Arc<ObserverDrivenField>,
        address: HolographicAddress,
        data: T,
        free_will_seed: u64,
    ) -> Self {
        Self {
            field,
            address,
            spectrum: SpectrumConfiguration::default(),
            archetypes: [0.0; 22],
            density: data.current_density(),
            free_will_seed,
            data,
        }
    }

    /// Main tick: update this component based on field state.
    pub fn tick(&mut self, dt: f64) {
        // 1. Read local field state
        let field_view = self.field.get_local_view(&self.address);

        // 2. Update component-specific logic
        self.data.tick(&field_view, dt);

        // 3. Update density (may have changed from evolution)
        self.density = self.data.current_density();

        // 4. If observer, ensure field is decompressed around us
        if self.data.is_observer() {
            // Observer registration handled by field manager
        }
    }

    /// Get this component's position.
    pub fn address(&self) -> &HolographicAddress {
        &self.address
    }

    /// Get this component's density.
    pub fn density(&self) -> &Density {
        &self.density
    }

    /// Get the archetype activation coefficients.
    pub fn archetypes(&self) -> &[f64; 22] {
        &self.archetypes
    }

    /// Set archetype activations (from Solar Logos configuration).
    pub fn configure_archetypes(&mut self, activations: [f64; 22]) {
        self.archetypes = activations;
    }
}
```

---

#### 1.4 Migration Strategy

The migration from `Coordinate3D` to `HolographicAddress` must be incremental to avoid
breaking the existing 335K-line codebase:

```rust
/// Compatibility layer: convert between old and new addressing.
/// This allows incremental migration — modules can switch one at a time.
impl From<Coordinate3D> for HolographicAddress {
    fn from(coord: Coordinate3D) -> Self {
        // Map the old bounded coordinates into the holographic address space.
        // The old (-1000..1000) range maps to a single decompressed region
        // at Planetary scale level.
        let octant = compute_octant(coord.x, coord.y, coord.z);
        HolographicAddress {
            coherence_path: vec![
                CoherenceStep { density_band: 3, octant, depth: 0 },
            ],
            scale: ScaleLevel::Planetary,
            local_offset: Vector3::new(coord.x, coord.y, coord.z),
        }
    }
}

impl From<HolographicAddress> for Coordinate3D {
    fn from(addr: HolographicAddress) -> Self {
        // Backwards compatibility: extract local offset as coordinates.
        // Only works for addresses that have been decompressed.
        Coordinate3D {
            x: addr.local_offset.x,
            y: addr.local_offset.y,
            z: addr.local_offset.z,
        }
    }
}

fn compute_octant(x: f64, y: f64, z: f64) -> u8 {
    let bx = if x >= 0.0 { 1u8 } else { 0 };
    let by = if y >= 0.0 { 1u8 } else { 0 };
    let bz = if z >= 0.0 { 1u8 } else { 0 };
    bx | (by << 1) | (bz << 2)
}
```

#### 1.5 Performance Targets

| Metric | Target | Mechanism |
|--------|--------|-----------|
| Addressable space | Infinite | Coherence path has no length limit |
| Unobserved region memory | O(1) per region | MERA compressed representation |
| Region lookup | O(log n) | Coherence path tree traversal |
| Decompression latency | O(log n) per region | MERA network partial decomposition |
| Entity placement | Via coherence address | No random coordinates |
| Observer count | 100+ simultaneous | Independent region management |

---

### PHASE 2: PROCEDURAL COSMOS ENGINE (Weeks 7-12)

**Goal:** Universe self-generates from field coherence. Stars ignite. Planets form.
Physics is FELT by entities. No pre-scripted universe structures.

**Files to create/modify:**

- `src/cosmos/cosmos_engine.rs` — Universe generation from field coherence
- `src/cosmos/stellar_physics.rs` — Star lifecycle simulation
- `src/cosmos/planetary_formation.rs` — Planet formation from accretion
- `src/cosmos/orbital_mechanics.rs` — Kepler orbits, n-body interactions
- `src/cosmos/cosmic_web.rs` — Large-scale structure
- `src/cosmos/physics_experience.rs` — Physics-to-entity experience bridge
- `src/cosmos/mod.rs`

**Estimated effort:** 10,000-15,000 new lines

---

#### 2.1 Cosmological Genesis Sequence

The universe generates itself from the holographic field. No explicit construction of
stars or planets — they emerge from coherence dynamics.

```rust
/// The Cosmos Engine generates and manages the universe.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Logos (Blue realm) establishes the fundamental creative principle.
/// Light/Love (Green realm) becomes the pattern of energy. Space/Time
/// dimensions emerge. Physical manifestation follows."
///
/// Stars don't get placed by construction. They ignite where the field
/// has sufficient coherence density for gravitational collapse.
pub struct CosmosEngine {
    /// Reference to the holographic field (shared with all subsystems).
    field: Arc<ObserverDrivenField>,

    /// Active stellar systems (only those near observers are fully simulated).
    stellar_systems: HashMap<StellarId, StellarSystem>,

    /// Cosmic-scale structure (filaments, voids, clusters).
    cosmic_web: CosmicWeb,

    /// Proto-stellar regions (coherence peaks not yet collapsed into stars).
    proto_stellar_regions: Vec<ProtoStellarRegion>,

    /// Time since cosmic genesis (in simulation seconds).
    cosmic_age: f64,
}

/// A stellar system: star + orbiting bodies.
pub struct StellarSystem {
    pub id: StellarId,
    pub star: Star,
    pub planets: Vec<Planet>,
    pub asteroid_belts: Vec<AsteroidBelt>,
    pub comets: Vec<Comet>,
    pub address: HolographicAddress,
}

/// A region of the field approaching stellar ignition.
pub struct ProtoStellarRegion {
    pub address: HolographicAddress,
    pub mass_accumulation: f64,
    pub temperature: f64,
    pub density: f64,
    pub angular_momentum: Vector3,
    pub collapse_progress: f64, // 0.0 = diffuse cloud, 1.0 = stellar ignition
}

impl CosmosEngine {
    /// Create a new cosmos engine from a holographic field.
    pub fn new(field: Arc<ObserverDrivenField>) -> Self {
        Self {
            field,
            stellar_systems: HashMap::new(),
            cosmic_web: CosmicWeb::new(),
            proto_stellar_regions: Vec::new(),
            cosmic_age: 0.0,
        }
    }

    /// Run the genesis sequence from field coherence.
    /// This is called at simulation start to bootstrap the universe.
    pub fn genesis(&mut self) {
        // 1. Detect coherence peaks in the field → matter formation sites
        let peaks = self.field.detect_coherence_peaks(
            CoherenceThreshold::StellarFormation,
        );

        // 2. Create proto-stellar regions at coherence peaks
        for peak in &peaks {
            self.proto_stellar_regions.push(ProtoStellarRegion {
                address: peak.address.clone(),
                mass_accumulation: peak.coherence_magnitude * SOLAR_MASS_CONSTANT,
                temperature: COSMIC_BACKGROUND_TEMPERATURE,
                density: peak.coherence_magnitude,
                angular_momentum: peak.gradient_curl(),
                collapse_progress: 0.0,
            });
        }

        // 3. Evolve proto-stellar regions to collapse
        self.evolve_proto_stellar_regions();

        // 4. Build cosmic web structure from collapsed regions
        self.cosmic_web.build_from_stellar_positions(
            &self.stellar_systems,
        );
    }

    /// Main tick: advance all cosmic processes.
    pub fn tick(&mut self, dt: f64) {
        self.cosmic_age += dt;

        // 1. Check for new coherence peaks (new star formation sites)
        let new_peaks = self.field.detect_coherence_peaks(
            CoherenceThreshold::StellarFormation,
        );
        for peak in new_peaks {
            if !self.has_stellar_system_near(&peak.address) {
                self.proto_stellar_regions.push(ProtoStellarRegion {
                    address: peak.address.clone(),
                    mass_accumulation: peak.coherence_magnitude * SOLAR_MASS_CONSTANT,
                    temperature: COSMIC_BACKGROUND_TEMPERATURE,
                    density: peak.coherence_magnitude,
                    angular_momentum: peak.gradient_curl(),
                    collapse_progress: 0.0,
                });
            }
        }

        // 2. Evolve proto-stellar regions
        self.evolve_proto_stellar_regions();

        // 3. Tick all active stellar systems
        for system in self.stellar_systems.values_mut() {
            system.tick(dt);
        }

        // 4. Update cosmic web
        self.cosmic_web.tick(dt);
    }

    /// Evolve proto-stellar regions toward gravitational collapse.
    fn evolve_proto_stellar_regions(&mut self) {
        let mut newly_ignited = Vec::new();

        for region in &mut self.proto_stellar_regions {
            // Gravitational collapse (Jeans instability)
            let jeans_mass = self.compute_jeans_mass(region.temperature, region.density);
            if region.mass_accumulation > jeans_mass {
                region.collapse_progress += 0.01; // Gradual collapse
                region.temperature *= 1.1; // Heating from compression
                region.density *= 1.05;
            }

            // Stellar ignition when core temperature reaches fusion threshold
            if region.temperature > HYDROGEN_FUSION_TEMPERATURE {
                // Star is born!
                let star = Star::ignite_from_proto_region(region);
                let planets = self.form_planets_from_disk(region);
                let system = StellarSystem {
                    id: StellarId::new(),
                    star,
                    planets,
                    asteroid_belts: vec![],
                    comets: vec![],
                    address: region.address.clone(),
                };
                newly_ignited.push(system);
            }
        }

        // Move ignited systems from proto-regions to active systems
        for system in newly_ignited {
            self.proto_stellar_regions
                .retain(|r| r.address != system.address);
            self.stellar_systems.insert(system.id, system);
        }
    }

    /// Form planets from the accretion disk around a proto-star.
    fn form_planets_from_disk(&self, region: &ProtoStellarRegion) -> Vec<Planet> {
        let mut planets = Vec::new();
        let disk_mass = region.mass_accumulation * 0.01; // 1% of stellar mass
        let num_planets = (disk_mass / EARTH_MASS).sqrt().min(12.0) as usize;

        for i in 0..num_planets {
            let orbital_radius = self.compute_orbital_radius(i, num_planets);
            let planet_mass = self.compute_planet_mass(
                disk_mass,
                orbital_radius,
                region.temperature,
            );
            let planet = Planet::form_from_accretion(
                orbital_radius,
                planet_mass,
                &region.address,
                &region.angular_momentum,
            );
            planets.push(planet);
        }

        planets
    }

    fn compute_jeans_mass(&self, temperature: f64, density: f64) -> f64 {
        // Jeans mass: M_J ∝ T^(3/2) / ρ^(1/2)
        let k_b = 1.38e-23; // Boltzmann constant
        let g = 6.674e-11; // Gravitational constant
        let m_h = 1.67e-27; // Hydrogen mass
        let pi = std::f64::consts::PI;
        (pi / 6.0) * (5.0 * k_b * temperature / (g * m_h)).powf(1.5) / density.sqrt()
    }

    fn compute_orbital_radius(&self, index: usize, total: usize) -> f64 {
        // Titius-Bode-like law: r_n = 0.4 + 0.3 * 2^n (AU)
        0.4 + 0.3 * 2.0_f64.powi(index as i32)
    }

    fn compute_planet_mass(
        &self,
        disk_mass: f64,
        orbital_radius: f64,
        stellar_temperature: f64,
    ) -> f64 {
        // Inner planets: rocky (small mass)
        // Outer planets: gas giants (large mass) beyond frost line
        let frost_line = 2.7 * (stellar_temperature / 5778.0).sqrt(); // AU
        if orbital_radius < frost_line {
            disk_mass * 0.001 * (1.0 / orbital_radius).sqrt() // Rocky
        } else {
            disk_mass * 0.1 * (orbital_radius / frost_line).sqrt() // Gas giant
        }
    }

    fn has_stellar_system_near(&self, address: &HolographicAddress) -> bool {
        self.stellar_systems
            .values()
            .any(|s| s.address.distance_to(address) < 1.0)
    }
}

const SOLAR_MASS_CONSTANT: f64 = 1.989e30; // kg
const COSMIC_BACKGROUND_TEMPERATURE: f64 = 2.725; // K
const HYDROGEN_FUSION_TEMPERATURE: f64 = 1.0e7; // K (10 million K)
const EARTH_MASS: f64 = 5.972e24; // kg
```

---

#### 2.2 Stellar Physics

Stars are living fusion reactors. They have lifecycles: main sequence → red giant →
white dwarf / neutron star / black hole. Their radiation determines habitability.

```rust
/// A star: a self-sustaining fusion reactor in the holographic field.
///
/// Stars are coherence peaks that have collapsed under gravitational
/// instability and ignited hydrogen fusion. They are the primary energy
/// source for all planetary and biological processes.
pub struct Star {
    pub id: StellarId,
    /// Spectral class determines temperature, color, luminosity
    pub spectral_class: SpectralClass,
    /// Mass in solar masses (determines everything else)
    pub mass: f64,
    /// Luminosity in solar luminosities
    pub luminosity: f64,
    /// Surface temperature in Kelvin
    pub temperature: f64,
    /// Radius in solar radii
    pub radius: f64,
    /// Age in years
    pub age: f64,
    /// Remaining hydrogen fuel fraction (0.0 = exhausted, 1.0 = full)
    pub fuel_remaining: f64,
    /// Full radiation spectrum output
    pub radiation_output: RadiationSpectrum,
    /// Stellar magnetic field (affects planetary magnetic fields)
    pub magnetic_field: MagneticField,
    /// Current evolutionary stage
    pub evolutionary_stage: StellarEvolutionStage,
    /// Metallicity (heavier elements from previous stellar generations)
    pub metallicity: f64,
}

/// Spectral classification: O (hottest) through M (coolest).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectralClass {
    O, // >30,000K — blue, rare, massive, short-lived
    B, // 10,000-30,000K — blue-white
    A, // 7,500-10,000K — white
    F, // 6,000-7,500K — yellow-white
    G, // 5,200-6,000K — yellow (Sun-like, habitable zone)
    K, // 3,700-5,200K — orange
    M, // 2,400-3,700K — red, common, long-lived
}

/// Stellar evolutionary stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StellarEvolutionStage {
    ProtoStar,      // Collapsing gas cloud
    MainSequence,   // Hydrogen fusion (stable)
    SubGiant,       // Core hydrogen exhausted
    RedGiant,       // Shell hydrogen fusion, expanding
    HorizontalBranch, // Core helium fusion
    AsymptoticGiant,  // Shell helium fusion
    PlanetaryNebula,  // Outer layers ejected
    WhiteDwarf,       // Degenerate remnant (< 1.4 M_sun)
    NeutronStar,      // Degenerate remnant (1.4-3 M_sun)
    BlackHole,        // Gravitational singularity (> 3 M_sun)
    Supernova,        // Catastrophic explosion
}

impl Star {
    /// Create a star from a proto-stellar region that has reached ignition.
    pub fn ignite_from_proto_region(region: &ProtoStellarRegion) -> Self {
        let mass = region.mass_accumulation / SOLAR_MASS_CONSTANT;
        let spectral_class = SpectralClass::from_mass(mass);
        let temperature = spectral_class.effective_temperature();
        let luminosity = mass.powf(3.5); // Mass-luminosity relation
        let radius = mass.powf(0.8); // Mass-radius relation (main sequence)

        Self {
            id: StellarId::new(),
            spectral_class,
            mass,
            luminosity,
            temperature,
            radius,
            age: 0.0,
            fuel_remaining: 1.0,
            radiation_output: RadiationSpectrum::blackbody(temperature),
            magnetic_field: MagneticField::from_rotation(
                region.angular_momentum.magnitude(),
            ),
            evolutionary_stage: StellarEvolutionStage::MainSequence,
            metallicity: 0.02, // Solar metallicity
        }
    }

    /// Tick the star's evolution.
    pub fn tick(&mut self, dt: f64) {
        self.age += dt;

        match self.evolutionary_stage {
            StellarEvolutionStage::MainSequence => {
                // Hydrogen fusion: consume fuel
                let fusion_rate = self.luminosity / self.main_sequence_lifetime();
                self.fuel_remaining -= fusion_rate * dt;

                // Update luminosity (brightens slightly over time)
                self.luminosity = self.mass.powf(3.5) * (1.0 + 0.4 * (1.0 - self.fuel_remaining));

                // Update radiation output
                self.radiation_output = RadiationSpectrum::blackbody(self.temperature);

                // Check for end of main sequence
                if self.fuel_remaining <= 0.1 {
                    self.evolutionary_stage = StellarEvolutionStage::SubGiant;
                }
            }
            StellarEvolutionStage::SubGiant => {
                // Core contracts, envelope expands
                self.radius *= 1.001; // Gradual expansion
                self.temperature *= 0.999; // Cooling surface
                self.luminosity *= 1.002; // Brightening

                if self.radius > self.mass.powf(0.8) * 5.0 {
                    self.evolutionary_stage = StellarEvolutionStage::RedGiant;
                }
            }
            StellarEvolutionStage::RedGiant => {
                // Massive expansion, shell fusion
                self.radius *= 1.01;
                self.temperature *= 0.995;
                self.luminosity *= 1.01;
                self.fuel_remaining -= 0.001 * dt;

                if self.fuel_remaining <= 0.0 {
                    self.evolve_off_main_sequence();
                }
            }
            StellarEvolutionStage::Supernova => {
                // Explosive energy release — one-tick event
                self.luminosity = 1.0e10; // Billion solar luminosities
                self.evolve_off_main_sequence();
            }
            _ => {
                // Remnant stages: slow cooling
                self.temperature *= 0.9999;
                self.luminosity *= 0.9999;
            }
        }
    }

    /// Compute radiation flux at a given distance from the star.
    pub fn radiation_at_distance(&self, distance_au: f64) -> f64 {
        // Inverse square law: F = L / (4π r²)
        let distance_m = distance_au * 1.496e11; // AU to meters
        let luminosity_w = self.luminosity * 3.828e26; // Solar luminosities to watts
        luminosity_w / (4.0 * std::f64::consts::PI * distance_m * distance_m)
    }

    /// Main sequence lifetime in years.
    fn main_sequence_lifetime(&self) -> f64 {
        // τ ∝ M / L ∝ M^(-2.5)
        1.0e10 * self.mass.powf(-2.5) // 10 billion years for 1 solar mass
    }

    /// Handle end-of-life stellar evolution.
    fn evolve_off_main_sequence(&mut self) {
        if self.mass < 0.5 {
            // Low mass: directly to white dwarf
            self.evolutionary_stage = StellarEvolutionStage::WhiteDwarf;
            self.radius = 0.01; // Earth-sized
            self.temperature = 10000.0;
        } else if self.mass < 8.0 {
            // Medium mass: planetary nebula → white dwarf
            self.evolutionary_stage = StellarEvolutionStage::WhiteDwarf;
            self.radius = 0.01;
            self.temperature = 20000.0;
        } else if self.mass < 25.0 {
            // High mass: supernova → neutron star
            self.evolutionary_stage = StellarEvolutionStage::NeutronStar;
            self.radius = 1.0e-5; // 10 km
            self.temperature = 1.0e6;
        } else {
            // Very high mass: supernova → black hole
            self.evolutionary_stage = StellarEvolutionStage::BlackHole;
            self.radius = 3.0 * self.mass * 2.95e3 / SOLAR_MASS_CONSTANT; // Schwarzschild
            self.luminosity = 0.0; // No radiation escapes
        }
    }
}

impl SpectralClass {
    pub fn from_mass(solar_masses: f64) -> Self {
        match solar_masses {
            m if m > 16.0 => Self::O,
            m if m > 2.1 => Self::B,
            m if m > 1.4 => Self::A,
            m if m > 1.04 => Self::F,
            m if m > 0.8 => Self::G,
            m if m > 0.45 => Self::K,
            _ => Self::M,
        }
    }

    pub fn effective_temperature(&self) -> f64 {
        match self {
            Self::O => 40000.0,
            Self::B => 20000.0,
            Self::A => 8500.0,
            Self::F => 6500.0,
            Self::G => 5778.0, // Sun
            Self::K => 4500.0,
            Self::M => 3000.0,
        }
    }
}
```

---

#### 2.3 Planetary Formation & Orbital Mechanics

Planets form from accretion disks. They orbit their stars following Keplerian mechanics.
Their axial tilt creates seasons. Their rotation creates day/night cycles. Their moons
create tides. **All of this is FELT by entities.**

```rust
/// A planet: a coherence structure orbiting a star.
///
/// Planets are not just positions with attributes — they are living worlds
/// with dynamic geology, hydrology, atmosphere, weather, and biospheres.
/// Every aspect feeds into entity experience.
pub struct Planet {
    pub id: PlanetId,
    // Orbital parameters
    pub orbital_radius: f64,         // AU
    pub orbital_period: f64,         // years (computed from Kepler's 3rd law)
    pub orbital_eccentricity: f64,   // 0.0 = circular, <1.0 = elliptical
    pub orbital_phase: f64,          // current position in orbit (radians)
    pub axial_tilt: f64,             // radians (creates seasons)
    pub rotation_period: f64,        // hours (creates day/night cycle)
    pub rotation_phase: f64,         // current rotation angle (radians)
    // Physical properties
    pub mass: f64,                   // kg
    pub radius: f64,                 // meters
    pub surface_gravity: f64,        // m/s² (computed from mass and radius)
    pub magnetic_field_strength: f64, // Tesla
    pub albedo: f64,                 // reflectivity (0.0-1.0)
    pub moons: Vec<Moon>,
    // Dynamic systems (Phase 3 fills these in)
    pub atmosphere: DynamicAtmosphere,
    pub hydrosphere: Hydrosphere,
    pub lithosphere: Lithosphere,
    pub biosphere: Biosphere,
    // Holographic address in the field
    pub address: HolographicAddress,
}

impl Planet {
    /// Create a planet from accretion disk parameters.
    pub fn form_from_accretion(
        orbital_radius: f64,
        mass: f64,
        star_address: &HolographicAddress,
        angular_momentum: &Vector3,
    ) -> Self {
        let radius = Self::compute_radius(mass);
        let surface_gravity = Self::compute_surface_gravity(mass, radius);
        let orbital_period = Self::compute_orbital_period(orbital_radius);

        Self {
            id: PlanetId::new(),
            orbital_radius,
            orbital_period,
            orbital_eccentricity: 0.02 + rand::random::<f64>() * 0.1,
            orbital_phase: rand::random::<f64>() * 2.0 * std::f64::consts::PI,
            axial_tilt: rand::random::<f64>() * 0.5, // 0 to ~28.6 degrees
            rotation_period: 10.0 + rand::random::<f64>() * 30.0, // 10-40 hours
            rotation_phase: 0.0,
            mass,
            radius,
            surface_gravity,
            magnetic_field_strength: Self::compute_magnetic_field(mass, angular_momentum),
            albedo: 0.3,
            moons: Vec::new(),
            atmosphere: DynamicAtmosphere::default(),
            hydrosphere: Hydrosphere::default(),
            lithosphere: Lithosphere::default(),
            biosphere: Biosphere::default(),
            address: star_address.refine(CoherenceStep {
                density_band: 5,
                octant: 0,
                depth: 1,
            }),
        }
    }

    /// Tick the planet: orbital mechanics + all subsystems.
    pub fn tick(&mut self, star: &Star, dt: f64) {
        // 1. Update orbital position (Kepler's equation)
        let dt_years = dt / 3.156e7; // seconds to years
        self.orbital_phase += (2.0 * std::f64::consts::PI / self.orbital_period) * dt_years;
        self.orbital_phase %= 2.0 * std::f64::consts::PI;

        // 2. Update rotation
        let dt_hours = dt / 3600.0;
        self.rotation_phase += (2.0 * std::f64::consts::PI / self.rotation_period) * dt_hours;
        self.rotation_phase %= 2.0 * std::f64::consts::PI;

        // 3. Compute stellar radiation received
        let current_distance = self.current_distance_from_star();
        let radiation = star.radiation_at_distance(current_distance);

        // 4. Compute solar angle (for day/night)
        let solar_angle = self.compute_solar_angle();

        // 5. Compute season from axial tilt and orbital position
        let season = self.compute_season();

        // 6. Compute tidal forces from moons
        let tidal_force = self.compute_tidal_forces();

        // 7. Update dynamic planetary systems (Phase 3 — detailed below)
        self.atmosphere.tick(radiation, solar_angle, season, dt);
        self.hydrosphere.tick(&self.atmosphere, tidal_force, dt);
        self.lithosphere.tick(dt);
        self.biosphere.tick(&self.atmosphere, &self.hydrosphere, radiation, dt);
    }

    /// Current distance from star (varies with eccentricity).
    fn current_distance_from_star(&self) -> f64 {
        // Elliptical orbit: r = a(1-e²) / (1 + e·cos(θ))
        let a = self.orbital_radius;
        let e = self.orbital_eccentricity;
        a * (1.0 - e * e) / (1.0 + e * self.orbital_phase.cos())
    }

    /// Compute the solar angle at a given latitude.
    fn compute_solar_angle(&self) -> f64 {
        // Simplified: angle depends on rotation phase and axial tilt
        (self.rotation_phase.sin() * self.axial_tilt.cos()).asin()
    }

    /// Compute current season from axial tilt and orbital position.
    fn compute_season(&self) -> Season {
        let seasonal_angle = (self.orbital_phase + std::f64::consts::FRAC_PI_2).sin()
            * self.axial_tilt.sin();
        if seasonal_angle > 0.3 {
            Season::Summer
        } else if seasonal_angle < -0.3 {
            Season::Winter
        } else if self.orbital_phase.cos() > 0.0 {
            Season::Spring
        } else {
            Season::Autumn
        }
    }

    fn compute_tidal_forces(&self) -> f64 {
        self.moons
            .iter()
            .map(|moon| moon.mass / moon.orbital_radius.powi(3))
            .sum()
    }

    fn compute_radius(mass: f64) -> f64 {
        // Empirical mass-radius relation for terrestrial planets
        let earth_mass = 5.972e24;
        let earth_radius = 6.371e6;
        earth_radius * (mass / earth_mass).powf(0.27)
    }

    fn compute_surface_gravity(mass: f64, radius: f64) -> f64 {
        let g = 6.674e-11;
        g * mass / (radius * radius)
    }

    fn compute_orbital_period(orbital_radius_au: f64) -> f64 {
        // Kepler's 3rd law: P² = a³ (in AU and years)
        orbital_radius_au.powf(1.5)
    }

    fn compute_magnetic_field(mass: f64, angular_momentum: &Vector3) -> f64 {
        // Simplified: magnetic field from core dynamo
        let earth_field = 5.0e-5; // Tesla
        let earth_mass = 5.972e24;
        earth_field * (mass / earth_mass).powf(0.5) * angular_momentum.magnitude().min(2.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Season { Spring, Summer, Autumn, Winter }
```

---

#### 2.4 Physics-Entity Experience Bridge

This is the critical missing link: entities **FEEL** physics. Gravity holds them.
Radiation warms them. Day and night cycle. Seasons change.

```rust
/// Entities FEEL physics. This struct translates computed physical forces
/// into subjective entity experience.
///
/// Gap #10 resolution: Physics is no longer just numbers — it's experience.
pub struct PhysicsExperience {
    /// Felt gravitational acceleration (direction + magnitude).
    /// On Earth: ~9.8 m/s² downward. In space: near zero.
    pub gravity: Vector3,

    /// Local temperature in Kelvin.
    /// Determined by: stellar radiation, atmospheric greenhouse, altitude,
    /// latitude, time of day, season, proximity to heat sources.
    pub temperature: f64,

    /// Radiation exposure from star (W/m²).
    /// Filtered by atmosphere. Affects biology (photosynthesis, sunburn).
    pub radiation_exposure: f64,

    /// Atmospheric pressure in Pascals.
    /// Determines: breathing capability, sound propagation, boiling point.
    pub pressure: f64,

    /// Day/night phase (0.0 = midnight, 0.5 = noon, 1.0 = midnight again).
    /// Affects: visibility, temperature, behavior patterns.
    pub day_night_phase: f64,

    /// Current season at this latitude.
    /// Affects: temperature, food availability, migration patterns.
    pub season: Season,

    /// Tidal force from moons (dimensionless, relative).
    /// Affects: ocean levels, biological rhythms.
    pub tidal_force: f64,

    /// Local magnetic field vector.
    /// Affects: navigation (magnetoreception), radiation shielding.
    pub magnetic_field: Vector3,

    /// Wind velocity at entity location.
    pub wind: Vector3,

    /// Precipitation at entity location (mm/hour equivalent).
    pub precipitation: f64,

    /// Humidity (0.0 = bone dry, 1.0 = saturated).
    pub humidity: f64,

    /// Terrain type at entity location.
    pub terrain: TerrainType,

    /// Altitude above sea level (meters).
    pub altitude: f64,

    /// Underwater depth (0.0 if on land, positive if submerged).
    pub water_depth: f64,
}

impl PhysicsExperience {
    /// Compute the physics experience for an entity on a planet.
    pub fn compute(
        entity_address: &HolographicAddress,
        planet: &Planet,
        star: &Star,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        let distance = planet.current_distance_from_star();
        let raw_radiation = star.radiation_at_distance(distance);
        let day_phase = Self::compute_day_phase(planet, longitude);
        let solar_angle = Self::compute_local_solar_angle(planet, latitude, day_phase);
        let atmosphere_filter = planet.atmosphere.transmittance(solar_angle);

        Self {
            gravity: Vector3::new(0.0, -planet.surface_gravity, 0.0),
            temperature: planet.atmosphere.temperature_at(latitude, day_phase, &planet.compute_season()),
            radiation_exposure: raw_radiation * atmosphere_filter * solar_angle.max(0.0),
            pressure: planet.atmosphere.pressure_at_altitude(0.0), // sea level
            day_night_phase: day_phase,
            season: planet.compute_season(),
            tidal_force: planet.compute_tidal_forces(),
            magnetic_field: Vector3::new(
                planet.magnetic_field_strength * latitude.cos(),
                planet.magnetic_field_strength * latitude.sin(),
                0.0,
            ),
            wind: planet.atmosphere.wind_at(latitude, longitude),
            precipitation: planet.atmosphere.precipitation_at(latitude, longitude),
            humidity: planet.atmosphere.humidity_at(latitude, longitude),
            terrain: planet.lithosphere.terrain_at(latitude, longitude),
            altitude: planet.lithosphere.altitude_at(latitude, longitude),
            water_depth: planet.hydrosphere.depth_at(latitude, longitude),
        }
    }

    fn compute_day_phase(planet: &Planet, longitude: f64) -> f64 {
        let phase = (planet.rotation_phase + longitude) / (2.0 * std::f64::consts::PI);
        phase.fract()
    }

    fn compute_local_solar_angle(planet: &Planet, latitude: f64, day_phase: f64) -> f64 {
        let hour_angle = (day_phase - 0.5) * 2.0 * std::f64::consts::PI;
        let declination = planet.axial_tilt * planet.orbital_phase.sin();
        (latitude.sin() * declination.sin()
            + latitude.cos() * declination.cos() * hour_angle.cos())
        .asin()
    }

    /// Comfort level for biological organisms (0.0 = lethal, 1.0 = ideal).
    pub fn habitability_score(&self) -> f64 {
        let temp_score = 1.0 - ((self.temperature - 293.0) / 50.0).abs().min(1.0); // Ideal ~20°C
        let pressure_score = 1.0 - ((self.pressure - 101325.0) / 50000.0).abs().min(1.0); // 1 atm
        let radiation_score = 1.0 - (self.radiation_exposure / 2000.0).min(1.0);
        (temp_score + pressure_score + radiation_score) / 3.0
    }
}
```

---

### PHASE 3: LIVING PLANETS (Weeks 13-16)

**Goal:** Planets are alive. Geology, hydrology, atmosphere, weather all run dynamically
each tick. The planet surface is a living system, not a static backdrop.

**Files to create/modify:**

- `src/planet/lithosphere.rs` — Dynamic geology, tectonics, volcanism
- `src/planet/hydrosphere.rs` — Dynamic hydrology, water cycle, ocean currents
- `src/planet/atmosphere.rs` — Dynamic atmosphere, weather, storms
- `src/planet/energy_flow.rs` — Star → planet → biology energy chain
- `src/planet/mod.rs`
- Integrate with `src/gaia/` (2,285 existing lines)

**Estimated effort:** 8,000-10,000 new lines, 1,500 lines modified

---

#### 3.1 Dynamic Geology (Lithosphere)

```rust
/// Dynamic lithosphere: tectonic plates, volcanism, seismicity, erosion.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Matter at higher complexity levels inherits and includes all lower levels."
///
/// The lithosphere interacts with the hydrosphere (erosion, sedimentation),
/// atmosphere (volcanic gases), and biosphere (soil formation).
pub struct Lithosphere {
    /// Active tectonic plates
    pub tectonic_plates: Vec<TectonicPlate>,
    /// Mantle convection cells (drive plate motion)
    pub mantle_convection: MantleConvection,
    /// Surface crust grid (elevation, composition, soil)
    pub crust: CrustGrid,
    /// Active and dormant volcanoes
    pub volcanoes: Vec<Volcano>,
    /// Seismic zones along plate boundaries
    pub earthquake_zones: Vec<SeismicZone>,
    /// Accumulated erosion/deposition map
    pub erosion_map: ErosionMap,
}

pub struct TectonicPlate {
    /// Plate identifier
    pub id: PlateId,
    /// Boundary type where this plate meets others
    pub boundary_type: BoundaryType,
    /// Plate velocity vector (cm/year, continental drift)
    pub velocity: Vector2,
    /// Average density of the plate
    pub density: f64,
    /// Average thickness (km)
    pub thickness: f64,
    /// Geological age (billions of years)
    pub age: f64,
    /// Whether this is oceanic or continental crust
    pub crust_type: CrustType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryType {
    /// Plates moving toward each other → subduction, mountains, volcanism
    Convergent,
    /// Plates moving apart → rifts, mid-ocean ridges, new crust
    Divergent,
    /// Plates sliding past each other → earthquakes
    Transform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrustType { Oceanic, Continental }

pub struct Volcano {
    pub position: (f64, f64), // latitude, longitude
    pub eruption_state: EruptionState,
    pub magma_chamber_pressure: f64,
    pub magma_chamber_volume: f64,
    pub eruption_history: Vec<EruptionEvent>,
    pub volcanic_type: VolcanicType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EruptionState { Dormant, Active, Erupting }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolcanicType { Shield, Stratovolcano, Caldera, Cinder }

impl Lithosphere {
    pub fn tick(&mut self, dt: f64) {
        // 1. Move tectonic plates (cm/year timescale)
        for plate in &mut self.tectonic_plates {
            plate.move_by(plate.velocity.scale(dt));
        }

        // 2. Detect plate boundary interactions
        let interactions = self.detect_boundary_interactions();
        for interaction in interactions {
            match interaction.boundary_type {
                BoundaryType::Convergent => {
                    // Subduction: denser plate dives under lighter one
                    // Results: mountain building, volcanism, deep-sea trenches
                    self.trigger_subduction(&interaction);
                }
                BoundaryType::Divergent => {
                    // Rifting: new crust forms from rising magma
                    // Results: mid-ocean ridges, rift valleys
                    self.create_new_crust(&interaction);
                }
                BoundaryType::Transform => {
                    // Shear: plates grind past each other
                    // Results: earthquakes, fault scarps
                    if self.should_trigger_earthquake(&interaction) {
                        self.trigger_earthquake(&interaction);
                    }
                }
            }
        }

        // 3. Volcanic activity
        for volcano in &mut self.volcanoes {
            volcano.tick(dt);
        }

        // 4. Erosion from wind and water (slow process)
        self.apply_erosion(dt);

        // 5. Mantle convection (drives plate motion)
        self.mantle_convection.tick(dt);
    }

    /// Get terrain type at a latitude/longitude.
    pub fn terrain_at(&self, lat: f64, lon: f64) -> TerrainType {
        let elevation = self.crust.elevation_at(lat, lon);
        let soil = self.crust.soil_type_at(lat, lon);
        TerrainType::from_elevation_and_soil(elevation, soil)
    }

    /// Get elevation at a latitude/longitude (meters above sea level).
    pub fn altitude_at(&self, lat: f64, lon: f64) -> f64 {
        self.crust.elevation_at(lat, lon)
    }

    fn detect_boundary_interactions(&self) -> Vec<BoundaryInteraction> {
        let mut interactions = Vec::new();
        for i in 0..self.tectonic_plates.len() {
            for j in (i + 1)..self.tectonic_plates.len() {
                let plate_a = &self.tectonic_plates[i];
                let plate_b = &self.tectonic_plates[j];
                if self.plates_are_adjacent(plate_a, plate_b) {
                    let relative_velocity = plate_a.velocity - plate_b.velocity;
                    let boundary_type = BoundaryType::classify(
                        &relative_velocity,
                        &self.boundary_normal(plate_a, plate_b),
                    );
                    interactions.push(BoundaryInteraction {
                        plate_a_id: plate_a.id,
                        plate_b_id: plate_b.id,
                        boundary_type,
                        relative_velocity,
                        stress: relative_velocity.magnitude(),
                    });
                }
            }
        }
        interactions
    }

    fn trigger_subduction(&mut self, interaction: &BoundaryInteraction) {
        // Mountain building: raise elevation along collision zone
        let collision_zone = self.find_collision_zone(interaction);
        for point in collision_zone {
            self.crust.raise_elevation(point.0, point.1, 0.001); // 1mm per tick
        }
        // Volcanism: create or pressurize volcanoes above subduction zone
        if rand::random::<f64>() < 0.001 * interaction.stress {
            let volcano_pos = self.subduction_volcano_position(interaction);
            self.volcanoes.push(Volcano::new_at(volcano_pos));
        }
    }

    fn create_new_crust(&mut self, interaction: &BoundaryInteraction) {
        // New ocean floor forms at divergent boundaries
        let rift_zone = self.find_rift_zone(interaction);
        for point in rift_zone {
            self.crust.set_crust_type(point.0, point.1, CrustType::Oceanic);
            self.crust.set_age(point.0, point.1, 0.0);
        }
    }

    fn trigger_earthquake(&mut self, interaction: &BoundaryInteraction) {
        let magnitude = 2.0 + interaction.stress.log10().abs() * 2.0;
        self.earthquake_zones.push(SeismicZone {
            epicenter: self.boundary_midpoint(interaction),
            magnitude: magnitude.min(9.5),
            depth: 10.0 + rand::random::<f64>() * 50.0,
            timestamp: 0.0, // current simulation time
        });
    }

    fn apply_erosion(&mut self, dt: f64) {
        // Simplified erosion: high elevations erode, low elevations accumulate
        self.crust.apply_diffusive_erosion(dt * 1e-6);
    }

    // Placeholder methods for boundary geometry
    fn plates_are_adjacent(&self, _a: &TectonicPlate, _b: &TectonicPlate) -> bool { true }
    fn boundary_normal(&self, _a: &TectonicPlate, _b: &TectonicPlate) -> Vector2 { Vector2::new(1.0, 0.0) }
    fn find_collision_zone(&self, _i: &BoundaryInteraction) -> Vec<(f64, f64)> { vec![] }
    fn find_rift_zone(&self, _i: &BoundaryInteraction) -> Vec<(f64, f64)> { vec![] }
    fn boundary_midpoint(&self, _i: &BoundaryInteraction) -> (f64, f64) { (0.0, 0.0) }
    fn subduction_volcano_position(&self, _i: &BoundaryInteraction) -> (f64, f64) { (0.0, 0.0) }
    fn should_trigger_earthquake(&self, i: &BoundaryInteraction) -> bool {
        rand::random::<f64>() < 0.01 * i.stress
    }
}
```

---

#### 3.2 Dynamic Hydrology (Hydrosphere)

```rust
/// Dynamic hydrosphere: oceans, rivers, lakes, glaciers, water cycle.
pub struct Hydrosphere {
    pub oceans: Vec<Ocean>,
    pub rivers: Vec<River>,
    pub lakes: Vec<Lake>,
    pub glaciers: Vec<Glacier>,
    pub water_cycle: WaterCycle,
    pub ocean_currents: Vec<OceanCurrent>,
    /// Water depth map (meters below sea level; negative = above)
    pub depth_map: DepthMap,
}

pub struct WaterCycle {
    pub evaporation_rate: f64,    // kg/m²/s
    pub precipitation_rate: f64,  // mm/hour
    pub runoff_rate: f64,         // m³/s
    pub groundwater_level: f64,   // meters below surface
    pub cloud_water_content: f64, // kg/m³
}

pub struct OceanCurrent {
    pub name: String,
    pub path: Vec<(f64, f64)>,    // latitude, longitude waypoints
    pub velocity: f64,            // m/s
    pub temperature: f64,         // °C
    pub salinity: f64,            // parts per thousand
    pub current_type: CurrentType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentType {
    Surface,        // Wind-driven (Gulf Stream, Kuroshio)
    DeepWater,      // Thermohaline (global conveyor belt)
    Upwelling,      // Cold deep water rising (nutrient-rich)
    Downwelling,    // Warm surface water sinking
}

impl Hydrosphere {
    pub fn tick(
        &mut self,
        atmosphere: &DynamicAtmosphere,
        tidal_force: f64,
        dt: f64,
    ) {
        // 1. Evaporation (temperature-driven: warmer → more evaporation)
        let evaporation = self.compute_evaporation(atmosphere.average_temperature());
        self.water_cycle.evaporation_rate = evaporation;
        self.water_cycle.cloud_water_content += evaporation * dt;

        // 2. Precipitation (when clouds saturate)
        let precipitation = atmosphere.compute_precipitation();
        self.water_cycle.precipitation_rate = precipitation;
        self.water_cycle.cloud_water_content -= precipitation * dt;

        // 3. River flow (gravity + terrain driven)
        for river in &mut self.rivers {
            river.flow(precipitation, dt);
        }

        // 4. Ocean currents (temperature differential + Coriolis effect)
        for current in &mut self.ocean_currents {
            current.update(atmosphere, tidal_force, dt);
        }

        // 5. Glacial advance/retreat (temperature-dependent)
        for glacier in &mut self.glaciers {
            glacier.update(atmosphere.average_temperature(), precipitation, dt);
        }

        // 6. Sea level changes (glacial melt, thermal expansion)
        self.update_sea_level(atmosphere.average_temperature(), dt);
    }

    /// Water depth at a given location (0.0 = dry land, positive = underwater).
    pub fn depth_at(&self, lat: f64, lon: f64) -> f64 {
        self.depth_map.depth_at(lat, lon).max(0.0)
    }

    fn compute_evaporation(&self, temperature: f64) -> f64 {
        // Clausius-Clapeyron: evaporation roughly doubles per 10°C
        let base_rate = 0.001; // kg/m²/s at 20°C
        base_rate * 2.0_f64.powf((temperature - 293.0) / 10.0)
    }

    fn update_sea_level(&mut self, temperature: f64, dt: f64) {
        // Thermal expansion + glacial melt
        let expansion = (temperature - 288.0) * 0.0001; // m per degree above 15°C
        let glacial_melt: f64 = self
            .glaciers
            .iter()
            .map(|g| g.melt_rate)
            .sum::<f64>() * dt;
        self.depth_map.adjust_sea_level(expansion + glacial_melt);
    }
}
```

---

#### 3.3 Dynamic Atmosphere & Weather

```rust
/// Dynamic atmosphere: composition, pressure, temperature, wind, clouds, storms.
pub struct DynamicAtmosphere {
    pub composition: AtmosphericComposition,
    pub pressure_field: PressureField,
    pub temperature_field: TemperatureField,
    pub wind_field: WindField,
    pub humidity_field: HumidityField,
    pub cloud_formations: Vec<CloudFormation>,
    pub weather_fronts: Vec<WeatherFront>,
    pub storms: Vec<Storm>,
}

pub struct AtmosphericComposition {
    pub nitrogen: f64,      // fraction (Earth: 0.78)
    pub oxygen: f64,        // fraction (Earth: 0.21)
    pub co2: f64,           // fraction (Earth: 0.0004)
    pub water_vapor: f64,   // fraction (variable)
    pub methane: f64,       // fraction
    pub noble_gases: f64,   // fraction
    /// Greenhouse factor (computed from composition)
    pub greenhouse_effect: f64,
}

pub struct Storm {
    pub storm_type: StormType,
    pub center: (f64, f64),       // latitude, longitude
    pub radius: f64,              // km
    pub intensity: f64,           // 0.0-1.0
    pub wind_speed: f64,          // m/s
    pub precipitation_rate: f64,  // mm/hour
    pub direction: Vector2,       // movement vector
    pub lifespan_remaining: f64,  // hours
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StormType {
    Thunderstorm,   // Convective, localized
    Hurricane,      // Tropical cyclone, large
    Tornado,        // Mesocyclone, intense
    Blizzard,       // Winter storm
    Sandstorm,      // Desert, wind-driven
}

impl DynamicAtmosphere {
    pub fn tick(
        &mut self,
        solar_radiation: f64,
        solar_angle: f64,
        season: Season,
        dt: f64,
    ) {
        // 1. Solar heating (differential by latitude and time of day)
        self.temperature_field.apply_solar_heating(
            solar_radiation,
            solar_angle,
            &self.composition,
        );

        // 2. Greenhouse effect
        let greenhouse_warming = solar_radiation * self.composition.greenhouse_effect * 0.1;
        self.temperature_field.apply_uniform_warming(greenhouse_warming);

        // 3. Pressure gradients from temperature differentials
        self.pressure_field.update_from_temperature(&self.temperature_field);

        // 4. Wind from pressure gradients + Coriolis effect
        self.wind_field.update_from_pressure(&self.pressure_field, dt);

        // 5. Humidity from evaporation and temperature
        self.humidity_field.update(&self.temperature_field);

        // 6. Cloud formation from humidity + temperature (dew point)
        self.update_cloud_formations();

        // 7. Storm genesis (warm moist air + cold front collision)
        self.check_storm_formation();

        // 8. Weather front movement
        for front in &mut self.weather_fronts {
            front.advance(&self.wind_field, dt);
        }

        // 9. Storm evolution
        self.storms.retain_mut(|storm| {
            storm.lifespan_remaining -= dt / 3600.0;
            storm.center.0 += storm.direction.x * dt / 111000.0; // approx deg/s
            storm.center.1 += storm.direction.y * dt / 111000.0;
            storm.lifespan_remaining > 0.0
        });

        // 10. Atmospheric chemistry (O2/CO2 balance with biosphere)
        self.composition.chemistry_tick(dt);
    }

    pub fn average_temperature(&self) -> f64 {
        self.temperature_field.global_average()
    }

    pub fn temperature_at(&self, lat: f64, day_phase: f64, season: &Season) -> f64 {
        self.temperature_field.at(lat, day_phase, season)
    }

    pub fn pressure_at_altitude(&self, altitude: f64) -> f64 {
        // Barometric formula: P = P0 * exp(-Mgh/RT)
        let p0 = 101325.0;
        let scale_height = 8500.0; // meters
        p0 * (-altitude / scale_height).exp()
    }

    pub fn wind_at(&self, lat: f64, lon: f64) -> Vector3 {
        self.wind_field.at(lat, lon)
    }

    pub fn precipitation_at(&self, lat: f64, lon: f64) -> f64 {
        // Sum precipitation from all storms affecting this location
        self.storms
            .iter()
            .filter(|s| {
                let dist = ((s.center.0 - lat).powi(2) + (s.center.1 - lon).powi(2)).sqrt();
                dist < s.radius / 111.0 // km to degrees
            })
            .map(|s| s.precipitation_rate)
            .sum::<f64>()
    }

    pub fn humidity_at(&self, lat: f64, lon: f64) -> f64 {
        self.humidity_field.at(lat, lon)
    }

    pub fn transmittance(&self, solar_angle: f64) -> f64 {
        // Atmospheric transmittance depends on air mass (path length through atmosphere)
        if solar_angle <= 0.0 { return 0.0; }
        let air_mass = 1.0 / solar_angle.sin().max(0.01);
        0.7_f64.powf(air_mass.powf(0.678)) // Meinel's formula
    }

    pub fn compute_precipitation(&self) -> f64 {
        self.storms.iter().map(|s| s.precipitation_rate).sum::<f64>()
            + self.weather_fronts.iter().map(|f| f.precipitation_rate).sum::<f64>()
    }

    fn update_cloud_formations(&mut self) {
        // Clouds form where humidity exceeds saturation at given temperature
        // Simplified: high humidity regions get clouds
        self.cloud_formations.retain(|c| c.humidity > 0.7);
    }

    fn check_storm_formation(&mut self) {
        // Storms form when warm moist air meets cold dry air
        // or when ocean temperature > 26°C (tropical cyclone genesis)
        if rand::random::<f64>() < 0.001 {
            let lat = (rand::random::<f64>() - 0.5) * 60.0; // ±30° latitude
            let lon = (rand::random::<f64>() - 0.5) * 360.0;
            self.storms.push(Storm {
                storm_type: StormType::Thunderstorm,
                center: (lat, lon),
                radius: 50.0 + rand::random::<f64>() * 200.0,
                intensity: 0.3 + rand::random::<f64>() * 0.7,
                wind_speed: 10.0 + rand::random::<f64>() * 50.0,
                precipitation_rate: 5.0 + rand::random::<f64>() * 20.0,
                direction: Vector2::new(
                    rand::random::<f64>() - 0.5,
                    rand::random::<f64>() - 0.5,
                ).normalize().scale(5.0),
                lifespan_remaining: 2.0 + rand::random::<f64>() * 24.0, // hours
            });
        }
    }
}

impl AtmosphericComposition {
    pub fn chemistry_tick(&mut self, dt: f64) {
        // Photosynthesis: CO2 → O2
        let photosynthesis_rate = self.co2 * 0.001 * dt;
        self.co2 -= photosynthesis_rate;
        self.oxygen += photosynthesis_rate;

        // Respiration: O2 → CO2
        let respiration_rate = self.oxygen * 0.0005 * dt;
        self.oxygen -= respiration_rate;
        self.co2 += respiration_rate;

        // Update greenhouse effect
        self.greenhouse_effect = self.co2 * 100.0 + self.methane * 25000.0 + self.water_vapor * 50.0;
    }
}
```

---

#### 3.4 Energy Flow: Star → Planet → Biology → Entity

```rust
/// Energy flow system: tracks energy from stellar source through all
/// planetary systems down to entity availability.
///
/// Gap #14 resolution: energy has a complete chain from star to entity.
pub struct EnergyFlowSystem {
    /// Solar energy input at top of atmosphere (W/m²)
    pub stellar_input: f64,
    /// Reflected back to space by albedo
    pub albedo_loss: f64,
    /// Absorbed by atmosphere (greenhouse heating)
    pub atmospheric_absorption: f64,
    /// Reaching the planetary surface
    pub surface_irradiance: f64,
    /// Captured by photosynthetic organisms
    pub biological_capture: f64,
    /// Available to heterotrophic organisms and entities
    pub entity_available_energy: f64,
    /// Geothermal energy from planetary interior
    pub geothermal_energy: f64,
}

impl EnergyFlowSystem {
    /// Recompute the energy flow chain.
    pub fn tick(&mut self, star: &Star, planet: &Planet) {
        let pi = std::f64::consts::PI;

        // Solar constant at planet's distance
        let distance_m = planet.current_distance_from_star() * 1.496e11;
        let luminosity_w = star.luminosity * 3.828e26;
        self.stellar_input = luminosity_w / (4.0 * pi * distance_m * distance_m);

        // Albedo loss
        self.albedo_loss = self.stellar_input * planet.albedo;

        // Atmospheric absorption (~23% on Earth)
        self.atmospheric_absorption =
            (self.stellar_input - self.albedo_loss) * 0.23;

        // Surface irradiance
        self.surface_irradiance =
            self.stellar_input - self.albedo_loss - self.atmospheric_absorption;

        // Photosynthetic capture (~1% of surface irradiance on Earth)
        self.biological_capture =
            self.surface_irradiance * planet.biosphere.photosynthetic_efficiency();

        // Food web efficiency (~10% per trophic level, average 2 levels)
        self.entity_available_energy =
            self.biological_capture * planet.biosphere.food_web_efficiency();

        // Geothermal (~0.1% of solar on Earth)
        self.geothermal_energy = 0.087; // W/m² (Earth average)
    }

    /// Total energy budget summary.
    pub fn summary(&self) -> String {
        format!(
            "Energy Flow: Stellar={:.1} W/m² → Surface={:.1} → Bio={:.4} → Entity={:.6}",
            self.stellar_input,
            self.surface_irradiance,
            self.biological_capture,
            self.entity_available_energy,
        )
    }
}
```

---

### PHASE 4: BIOLOGICAL SIMULATION ENGINE (Weeks 17-22)

**Goal:** Biology is ALIVE. Cells divide. Organisms grow. Species evolve through natural
selection. Ecosystems function. Entities have biological bodies.

**Files to create/modify:**

- `src/biology/cell_engine.rs` — Living cells that divide, metabolize, die
- `src/biology/organism_lifecycle.rs` — Multicellular organism simulation
- `src/biology/evolution_engine.rs` — Species-level evolution by natural selection
- `src/biology/body_coupling.rs` — Entity-body interface (consciousness + biology)
- Integrate with existing `src/biology/` (3,556 lines)

**Estimated effort:** 12,000-16,000 new lines, 2,000 lines modified

---

#### 4.1 Living Cell Engine

```rust
/// Living cell engine: cells that actually divide, metabolize, and die.
///
/// Gap #6 resolution: Biology is ALIVE, not described.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "2nd density entities exhibit growth, movement, and awareness of self
/// as distinct from environment."
pub struct CellEngine {
    /// All living cells (pooled, indexed by CellId)
    pub cells: Vec<LiveCell>,
    /// Cells queued for division this tick
    pub division_queue: VecDeque<CellId>,
    /// Cells queued for death (apoptosis or necrosis) this tick
    pub death_queue: VecDeque<CellId>,
    /// Next available cell ID
    next_id: u64,
    /// Total cells ever created (for statistics)
    pub total_created: u64,
    /// Total cells that have died (for statistics)
    pub total_died: u64,
}

/// A single living cell with full metabolic state.
pub struct LiveCell {
    pub id: CellId,
    /// Cell type (stem, nerve, muscle, epithelial, etc.)
    pub cell_type: CellType,
    /// DNA content (determines protein expression)
    pub dna: DNA,
    /// Current energy level (ATP equivalent)
    pub energy: f64,
    /// Health (0.0 = dead, 1.0 = perfect health)
    pub health: f64,
    /// Age in simulation time units
    pub age: f64,
    /// Number of times this cell has divided (Hayflick limit)
    pub division_count: u32,
    /// Maximum divisions before senescence
    pub max_divisions: u32,
    /// Position in the holographic field
    pub position: HolographicAddress,
    /// Metabolic rate (energy consumption per unit time)
    pub metabolic_rate: f64,
    /// Accumulated waste products (toxicity)
    pub waste_products: f64,
    /// Organism this cell belongs to (None = free-living)
    pub organism_id: Option<OrganismId>,
    /// Epigenetic state (gene expression modifiers)
    pub epigenetic_state: EpigeneticState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Stem,           // Undifferentiated, can become any type
    Neuron,         // Nerve cell (processing, signaling)
    Muscle,         // Contractile cell (movement)
    Epithelial,     // Covering cell (barrier, absorption)
    Connective,     // Structural cell (support, binding)
    Immune,         // Defense cell (pathogen response)
    Reproductive,   // Gamete or gamete precursor
    Photosynthetic, // Light-capturing cell (plants, algae)
    Secretory,      // Hormone/enzyme producing cell
}

impl CellEngine {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            division_queue: VecDeque::new(),
            death_queue: VecDeque::new(),
            next_id: 0,
            total_created: 0,
            total_died: 0,
        }
    }

    /// Main tick: process all cells.
    pub fn tick(&mut self, environment: &EnvironmentState, dt: f64) {
        // Phase 1: Update each cell's metabolic state
        for cell in &mut self.cells {
            // Metabolism: consume energy, produce waste
            let energy_consumed = cell.metabolic_rate * dt;
            cell.energy -= energy_consumed;
            cell.waste_products += energy_consumed * 0.3; // 30% waste

            // Energy intake from environment (simplified)
            let energy_intake = match cell.cell_type {
                CellType::Photosynthetic => {
                    environment.solar_radiation * 0.05 * dt // Photosynthesis
                }
                _ => {
                    environment.nutrient_density * 0.01 * dt // Heterotrophic
                }
            };
            cell.energy += energy_intake;

            // Aging
            cell.age += dt;

            // Health calculation
            cell.health = self.compute_cell_health(cell, environment);

            // Division check: enough energy, healthy, under Hayflick limit
            if cell.should_divide() {
                self.division_queue.push_back(cell.id);
            }

            // Death check: no energy, too much waste, too old, unhealthy
            if cell.should_die() {
                self.death_queue.push_back(cell.id);
            }
        }

        // Phase 2: Process divisions (with DNA replication and mutation)
        self.process_divisions();

        // Phase 3: Process deaths
        self.process_deaths();
    }

    fn compute_cell_health(&self, cell: &LiveCell, env: &EnvironmentState) -> f64 {
        let energy_factor = (cell.energy / 100.0).min(1.0); // Need energy
        let waste_factor = 1.0 - (cell.waste_products / 500.0).min(1.0); // Low waste
        let age_factor = 1.0 - (cell.age / cell.max_age()).min(1.0); // Youth
        let temp_factor = 1.0 - ((env.temperature - 310.0) / 20.0).abs().min(1.0); // ~37°C
        let radiation_factor = 1.0 - (env.radiation / 1000.0).min(1.0); // Low radiation

        (energy_factor * waste_factor * age_factor * temp_factor * radiation_factor).max(0.0)
    }

    fn process_divisions(&mut self) {
        let mut new_cells = Vec::new();

        while let Some(parent_id) = self.division_queue.pop_front() {
            if let Some(parent) = self.cells.iter_mut().find(|c| c.id == parent_id) {
                // Create daughter cell
                let mut daughter = LiveCell {
                    id: CellId(self.next_id),
                    cell_type: parent.cell_type,
                    dna: parent.dna.replicate_with_mutation(),
                    energy: parent.energy / 2.0, // Split energy
                    health: 1.0,
                    age: 0.0,
                    division_count: 0,
                    max_divisions: parent.max_divisions,
                    position: parent.position.clone(),
                    metabolic_rate: parent.metabolic_rate,
                    waste_products: 0.0,
                    organism_id: parent.organism_id,
                    epigenetic_state: parent.epigenetic_state.inherit(),
                };

                // Parent keeps half the energy, increments division count
                parent.energy /= 2.0;
                parent.division_count += 1;

                self.next_id += 1;
                self.total_created += 1;
                new_cells.push(daughter);
            }
        }

        self.cells.extend(new_cells);
    }

    fn process_deaths(&mut self) {
        let death_ids: Vec<CellId> = self.death_queue.drain(..).collect();
        for id in &death_ids {
            self.cells.retain(|c| c.id != *id);
            self.total_died += 1;
        }
    }
}

impl LiveCell {
    fn should_divide(&self) -> bool {
        self.energy > 200.0          // Enough energy
            && self.health > 0.7     // Healthy
            && self.division_count < self.max_divisions // Under Hayflick limit
            && self.age > 1.0        // Not brand new
    }

    fn should_die(&self) -> bool {
        self.energy <= 0.0           // No energy (starvation)
            || self.health <= 0.0    // Health collapsed
            || self.waste_products > 1000.0 // Toxicity
            || self.age > self.max_age()    // Senescence
    }

    fn max_age(&self) -> f64 {
        match self.cell_type {
            CellType::Neuron => 1000.0,       // Long-lived
            CellType::Muscle => 500.0,
            CellType::Epithelial => 50.0,     // Short-lived, high turnover
            CellType::Immune => 100.0,
            CellType::Stem => 2000.0,         // Very long-lived
            CellType::Photosynthetic => 200.0,
            _ => 300.0,
        }
    }
}
```

---

#### 4.2 Organism Lifecycle

```rust
/// A multicellular organism: a coherent collection of cells with
/// a body plan, behavior, and lifecycle.
pub struct Organism {
    pub id: OrganismId,
    pub species: SpeciesId,
    /// Constituent cells (references into CellEngine)
    pub cell_ids: Vec<CellId>,
    /// Morphological structure
    pub body_plan: BodyPlan,
    /// Neural complexity (0.0 = no nervous system, 1.0 = human-level)
    pub neural_complexity: f64,
    /// Energy reserves (fat/glucose equivalent)
    pub energy_store: f64,
    /// Age in simulation time
    pub age: f64,
    /// Whether sexually mature
    pub reproductive_maturity: bool,
    /// Number of offspring produced
    pub offspring_count: u32,
    /// Position in the field
    pub position: HolographicAddress,
    /// Current behavioral state
    pub behavior_state: BehaviorState,
    /// Genome (shared species template + individual variation)
    pub genome: Genome,
    /// Fitness score (computed from survival + reproduction success)
    pub fitness: f64,
}

pub struct BodyPlan {
    /// Body symmetry
    pub symmetry: Symmetry,
    /// Body segments (head, thorax, abdomen, limbs, etc.)
    pub segments: Vec<BodySegment>,
    /// How this organism moves
    pub locomotion: LocomotionType,
    /// Sensory capabilities
    pub sensory_organs: Vec<SensoryOrgan>,
    /// Digestive strategy
    pub digestive_type: DigestiveType,
    /// Body mass (kg)
    pub mass: f64,
    /// Body size (characteristic length, meters)
    pub size: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symmetry { Radial, Bilateral, Asymmetric }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocomotionType {
    Sessile,    // Fixed in place (plants, corals)
    Crawling,   // Surface locomotion (worms, snakes)
    Swimming,   // Aquatic (fish, cetaceans)
    Walking,    // Terrestrial bipedal or quadrupedal
    Flying,     // Aerial (birds, insects, bats)
    Burrowing,  // Underground (moles, earthworms)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorState {
    Resting,
    Foraging,
    Hunting,
    Fleeing,
    Mating,
    Parenting,
    Socializing,
    Exploring,
    Defending,
    Migrating,
}

impl Organism {
    pub fn tick(&mut self, environment: &EnvironmentState, dt: f64) {
        // 1. Metabolism: consume energy based on body mass and activity
        let metabolic_cost = self.compute_metabolic_cost(dt);
        self.energy_store -= metabolic_cost;

        // 2. Growth (if juvenile)
        if self.age < self.species_maturity_age() {
            self.grow(dt);
        }

        // 3. Aging
        self.age += dt;

        // 4. Behavior decision (driven by needs hierarchy)
        self.behavior_state = self.decide_behavior(environment);

        // 5. Reproductive maturity check
        if !self.reproductive_maturity && self.age >= self.species_maturity_age() {
            self.reproductive_maturity = true;
        }

        // 6. Fitness update
        self.fitness = self.compute_fitness(environment);
    }

    fn compute_metabolic_cost(&self, dt: f64) -> f64 {
        // Kleiber's law: metabolic rate ∝ mass^0.75
        let base_rate = 3.5 * self.body_plan.mass.powf(0.75); // watts
        let activity_multiplier = match self.behavior_state {
            BehaviorState::Resting => 1.0,
            BehaviorState::Foraging => 2.0,
            BehaviorState::Hunting => 4.0,
            BehaviorState::Fleeing => 5.0,
            BehaviorState::Flying => 6.0,
            BehaviorState::Mating => 3.0,
            _ => 1.5,
        };
        base_rate * activity_multiplier * dt
    }

    fn decide_behavior(&self, environment: &EnvironmentState) -> BehaviorState {
        // Maslow-like needs hierarchy for organisms:
        // 1. Survival (flee from predators)
        if environment.predator_nearby {
            return BehaviorState::Fleeing;
        }
        // 2. Energy (forage/hunt when hungry)
        if self.energy_store < self.body_plan.mass * 10.0 {
            return if self.body_plan.digestive_type == DigestiveType::Carnivore {
                BehaviorState::Hunting
            } else {
                BehaviorState::Foraging
            };
        }
        // 3. Reproduction (when mature and well-fed)
        if self.reproductive_maturity && self.energy_store > self.body_plan.mass * 50.0 {
            return BehaviorState::Mating;
        }
        // 4. Social (when all needs met)
        if self.neural_complexity > 0.3 {
            return BehaviorState::Socializing;
        }
        // 5. Rest
        BehaviorState::Resting
    }

    fn grow(&mut self, dt: f64) {
        let growth_rate = 0.01 * dt; // 1% per time unit
        self.body_plan.mass *= 1.0 + growth_rate;
        self.body_plan.size *= 1.0 + growth_rate * 0.33; // Cube root scaling
    }

    fn species_maturity_age(&self) -> f64 {
        // Larger organisms mature slower
        100.0 * self.body_plan.mass.powf(0.25) // Allometric scaling
    }

    fn compute_fitness(&self, environment: &EnvironmentState) -> f64 {
        let survival = self.health_score();
        let reproduction = self.offspring_count as f64 / (self.age + 1.0);
        let adaptation = environment.habitability_for(self);
        (survival + reproduction + adaptation) / 3.0
    }

    fn health_score(&self) -> f64 {
        let energy_score = (self.energy_store / (self.body_plan.mass * 100.0)).min(1.0);
        let age_score = 1.0 - (self.age / self.max_lifespan()).min(1.0);
        (energy_score + age_score) / 2.0
    }

    fn max_lifespan(&self) -> f64 {
        // Allometric: lifespan ∝ mass^0.25
        1000.0 * self.body_plan.mass.powf(0.25)
    }
}
```

---

#### 4.3 Species & Evolution Engine

```rust
/// Evolution engine: species-level dynamics with natural selection,
/// mutation, speciation, and extinction.
pub struct EvolutionEngine {
    pub species_registry: HashMap<SpeciesId, Species>,
    pub populations: HashMap<SpeciesId, Population>,
    pub extinction_log: Vec<ExtinctionEvent>,
    pub speciation_log: Vec<SpeciationEvent>,
    next_species_id: u64,
}

pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub genome_template: Genome,
    pub trophic_level: TrophicLevel,
    pub habitat: HabitatPreference,
    pub body_plan: BodyPlan,
    /// How close to self-awareness (0.0 = mineral, 1.0 = self-aware)
    pub consciousness_potential: f64,
    /// Density level (1st = mineral, 2nd = animal, 3rd = self-aware)
    pub density: Density,
    /// Time of origin (simulation time)
    pub origin_time: f64,
    /// Parent species (None if primordial)
    pub parent_species: Option<SpeciesId>,
}

pub struct Population {
    pub species_id: SpeciesId,
    pub count: usize,
    pub carrying_capacity: usize,
    pub growth_rate: f64,
    pub genetic_diversity: f64,     // 0.0 = clonal, 1.0 = maximally diverse
    pub average_fitness: f64,
    pub geographic_range: Vec<HolographicAddress>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrophicLevel {
    Producer,           // Autotroph (plants, algae)
    PrimaryConsumer,    // Herbivore
    SecondaryConsumer,  // Small predator
    TertiaryConsumer,   // Apex predator
    Decomposer,         // Detritovore (fungi, bacteria)
}

impl EvolutionEngine {
    pub fn tick(&mut self, dt: f64) {
        let species_ids: Vec<SpeciesId> = self.populations.keys().cloned().collect();

        for species_id in species_ids {
            if let Some(population) = self.populations.get_mut(&species_id) {
                // 1. Population dynamics (logistic growth)
                let n = population.count as f64;
                let k = population.carrying_capacity as f64;
                let r = population.growth_rate;
                let growth = r * n * (1.0 - n / k);
                population.count = (n + growth * dt).max(0.0).round() as usize;

                // 2. Genetic diversity drift
                if population.count > 0 {
                    // Diversity increases with large populations, decreases with small
                    let drift = if population.count > 100 {
                        0.001 * dt  // Large pop: slow diversification
                    } else {
                        -0.01 * dt  // Small pop: genetic drift (loss)
                    };
                    population.genetic_diversity =
                        (population.genetic_diversity + drift).clamp(0.0, 1.0);
                }

                // 3. Extinction check
                if population.count == 0 {
                    self.extinction_log.push(ExtinctionEvent {
                        species_id,
                        cause: ExtinctionCause::PopulationCollapse,
                        time: 0.0, // current simulation time
                    });
                }

                // 4. Speciation check (high genetic diversity + geographic isolation)
                if population.genetic_diversity > 0.8
                    && population.geographic_range.len() > 2
                {
                    self.attempt_speciation(species_id);
                }
            }
        }

        // 5. Remove extinct species from active populations
        let extinct: Vec<SpeciesId> = self
            .extinction_log
            .iter()
            .map(|e| e.species_id)
            .collect();
        for id in extinct {
            self.populations.remove(&id);
        }
    }

    fn attempt_speciation(&mut self, parent_id: SpeciesId) {
        if let Some(parent_species) = self.species_registry.get(&parent_id) {
            let new_id = SpeciesId(self.next_species_id);
            self.next_species_id += 1;

            let mut new_species = parent_species.clone();
            new_species.id = new_id;
            new_species.name = format!("{}_variant_{}", parent_species.name, new_id.0);
            new_species.parent_species = Some(parent_id);
            new_species.genome_template = parent_species.genome_template.mutate_significantly();

            // Split population
            if let Some(parent_pop) = self.populations.get_mut(&parent_id) {
                let split_count = parent_pop.count / 4; // 25% splits off
                parent_pop.count -= split_count;

                self.populations.insert(new_id, Population {
                    species_id: new_id,
                    count: split_count,
                    carrying_capacity: parent_pop.carrying_capacity / 2,
                    growth_rate: parent_pop.growth_rate * (0.8 + rand::random::<f64>() * 0.4),
                    genetic_diversity: 0.2, // Low initial diversity
                    average_fitness: parent_pop.average_fitness,
                    geographic_range: vec![], // New range
                });
            }

            self.species_registry.insert(new_id, new_species);
            self.speciation_log.push(SpeciationEvent {
                parent_id,
                child_id: new_id,
                time: 0.0,
            });
        }
    }
}
```

---

#### 4.4 Entity-Body Coupling

```rust
/// A 3rd density entity has an ACTUAL body made of simulated biology.
/// The consciousness and the body are COUPLED — each affects the other.
///
/// Gap #3 resolution: entities have real biological bodies.
pub struct EmbodiedEntity {
    /// Consciousness component (mind/body/spirit complex)
    pub consciousness: EntityConsciousness,
    /// Physical body (actual organism with cells)
    pub body: Organism,
    /// Body-mind interface (nervous system translates between domains)
    pub nervous_system: NervousSystem,
    /// Energy centers (chakras) mapped to body systems
    pub energy_centers: [EnergyCenterState; 7],
    /// Physics experience at current location
    pub physics_experience: PhysicsExperience,
    /// Veil status (how much pre-incarnative memory is accessible)
    pub veil: VeilMechanics,
}

pub struct NervousSystem {
    /// Neural complexity (determines sensory processing capability)
    pub complexity: f64,
    /// Current sensory input buffer
    pub sensory_buffer: Vec<SensoryInput>,
    /// Pain/pleasure signal (affects behavior)
    pub hedonic_signal: f64,
    /// Stress level (cumulative unresolved catalyst)
    pub stress: f64,
}

pub struct EnergyCenterState {
    /// Which energy center (Red=0, Orange=1, ... Violet=6)
    pub center: usize,
    /// Activation level (0.0 = blocked, 1.0 = fully activated)
    pub activation: f64,
    /// Blockage level (unresolved catalyst blocking this center)
    pub blockage: f64,
    /// Associated body system health
    pub body_system_health: f64,
}

impl EmbodiedEntity {
    /// Main tick: coupled consciousness-body simulation.
    pub fn tick(&mut self, environment: &EnvironmentState, dt: f64) {
        // 1. Body tick: metabolism, aging, health
        self.body.tick(environment, dt);

        // 2. Update physics experience at this location
        // (Populated by the planet tick before entity tick)

        // 3. Nervous system processes sensory input
        let sensory_input = self.nervous_system.process_senses(
            environment,
            &self.physics_experience,
        );

        // 4. Consciousness processes through archetype cycle
        let catalyst = Catalyst::from_sensory_input(sensory_input);
        let choice = self.consciousness.process_catalyst(&catalyst);

        // 5. Choice affects body (stress response, healing, energy)
        self.body.apply_consciousness_effect(&choice);

        // 6. Update energy centers based on experience
        self.update_energy_centers(&choice);

        // 7. Update nervous system stress
        self.nervous_system.stress += choice.unresolved_tension;
        self.nervous_system.stress -= self.nervous_system.stress * 0.01 * dt; // Natural recovery

        // 8. Body health affects consciousness (sick body → foggy mind)
        self.consciousness.clarity = self.body.health_score();
    }

    fn update_energy_centers(&mut self, choice: &ProcessedExperience) {
        // Energy center activation depends on the nature of the experience:
        // Red (0): Survival experiences
        // Orange (1): Personal identity, sexuality
        // Yellow (2): Social, power dynamics
        // Green (3): Love, compassion, acceptance
        // Blue (4): Communication, truth, expression
        // Indigo (5): Insight, intuition, will
        // Violet (6): Connection to Infinite, spiritual integration

        // Each choice activates certain centers and may clear or create blockages
        for i in 0..7 {
            let activation_delta = choice.energy_center_effect(i);
            self.energy_centers[i].activation =
                (self.energy_centers[i].activation + activation_delta).clamp(0.0, 1.0);

            // Clear blockage when center is activated with positive polarity
            if activation_delta > 0.0 && choice.polarity_shift > 0.0 {
                self.energy_centers[i].blockage *= 0.95; // 5% blockage reduction
            }
        }
    }
}
```

---

### PHASE 5: ARCHETYPE-DRIVEN CONSCIOUSNESS ENGINE (Weeks 23-26)

**Goal:** Entity behavior EMERGES from 22-archetype interference patterns. The Veil
creates qualitative experience differences. No behavior trees, no scripted AI.

**Files to create/modify:**

- `src/consciousness/archetype_processor.rs` — Archetype processing cycle
- `src/consciousness/dual_experience.rs` — Space/Time vs Time/Space experience
- `src/consciousness/veil_filter.rs` — Active veil filtering
- Integrate with existing `src/archetypes/` (24,032 lines)
- Integrate with existing `src/consciousness/` (3,115 lines)
- Integrate with existing `src/veil/` (3,856 lines)

**Estimated effort:** 6,000-8,000 new lines, 4,000 lines modified

---

#### 5.1 Archetype Processing Cycle

```rust
/// Archetype processor: the engine that drives entity behavior.
///
/// Gap #12 resolution: 24,032 lines of archetype code now DRIVE behavior.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Archetypical Mind is the architecture of consciousness.
/// Archetypes 1-7 = Mind, 8-14 = Body, 15-21 = Spirit, 22 = The Choice."
///
/// Each catalyst (experience) is processed through the full archetype cycle:
/// Matrix → Potentiator → Catalyst → Experience → Significator →
/// Transformation → Great Way
pub struct ArchetypeProcessor {
    /// The 22 archetype activation coefficients.
    /// These are set by the Solar Logos and inherited by the entity.
    pub activations: [f64; 22],

    /// Mind complex (Archetypes 1-7): how the entity THINKS
    pub mind: MindComplex,
    /// Body complex (Archetypes 8-14): how the entity ACTS
    pub body_complex: BodyArchetypeComplex,
    /// Spirit complex (Archetypes 15-21): how the entity WILLS
    pub spirit: SpiritComplex,
    /// The Choice (Archetype 22): STO vs STS polarization
    pub choice: ChoiceArchetype,

    /// Accumulated polarity (positive = STO, negative = STS)
    pub polarity: f64,
    /// Transformation threshold (how much catalyst needed for change)
    pub transformation_threshold: f64,
    /// Accumulated unprocessed catalyst (backlog)
    pub catalyst_backlog: f64,
}

pub struct MindComplex {
    pub matrix: f64,        // A1: The Magician — conscious mind, what you think you are
    pub potentiator: f64,   // A2: The High Priestess — unconscious, vast potential
    pub catalyst: f64,      // A3: The Empress — what triggers change
    pub experience: f64,    // A4: The Emperor — what you actually learn
    pub significator: f64,  // A5: The Hierophant — the whole mind, who you ARE
    pub transformation: f64,// A6: The Lovers — death/rebirth of old mental patterns
    pub great_way: f64,     // A7: The Chariot — environment in which mind operates
}

impl ArchetypeProcessor {
    /// Process a catalyst through the full archetype cycle.
    /// This is the core of consciousness simulation — how an entity
    /// turns raw experience into learning, growth, and polarity.
    pub fn process_catalyst(&mut self, catalyst: &Catalyst) -> ProcessedExperience {
        // ═══════════════════════════════════════════════════════
        // MIND COMPLEX PROCESSING (Archetypes 1-7)
        // ═══════════════════════════════════════════════════════

        // Step 1: MATRIX receives catalyst (what conscious mind perceives)
        // The Matrix is the surface mind — it sees what it's ready to see.
        let conscious_reception = self.mind.matrix * catalyst.intensity
            * (1.0 + catalyst.relevance_to_current_pattern(&self.mind));

        // Step 2: POTENTIATOR activates (unconscious patterns respond)
        // The Potentiator is the deep mind — it holds shadow material,
        // past-life patterns, and archetypal resonances.
        let unconscious_response = self.mind.potentiator
            * catalyst.resonance_with_shadow()
            * (1.0 + self.catalyst_backlog * 0.1); // Backlog amplifies

        // Step 3: CATALYST processing (how the challenge is engaged)
        // The Catalyst archetype determines whether the entity engages
        // or avoids the experience.
        let engagement = self.mind.catalyst
            * (conscious_reception + unconscious_response)
            * self.openness_factor();

        // Step 4: EXPERIENCE crystallized (what is actually learned)
        // The Experience archetype filters engagement into learning.
        let learning = self.mind.experience * engagement
            * self.learning_efficiency();

        // Step 5: SIGNIFICATOR integrates (whole mind shifts)
        // The Significator is who you ARE — it either accepts or rejects
        // the new learning based on consistency with self-concept.
        let integration = self.mind.significator * learning
            * self.consistency_with_self_concept(&catalyst);

        // Step 6: TRANSFORMATION occurs (old patterns die, new emerge)
        // Only happens when integration exceeds the threshold — not
        // every experience transforms. Most are filed away.
        let transformation = if integration > self.transformation_threshold {
            self.mind.transformation * integration
        } else {
            // Not enough to transform — add to backlog for later
            self.catalyst_backlog += integration * 0.5;
            0.0
        };

        // Step 7: GREAT WAY manifests (environment reflects change)
        // When transformation occurs, the entity's relationship with
        // its environment shifts.
        let environmental_effect = self.mind.great_way * transformation;

        // ═══════════════════════════════════════════════════════
        // BODY & SPIRIT COMPLEX (parallel processing)
        // ═══════════════════════════════════════════════════════

        let body_response = self.body_complex.process(catalyst, engagement);
        let spirit_response = self.spirit.process(catalyst, integration);

        // ═══════════════════════════════════════════════════════
        // THE CHOICE (Archetype 22)
        // ═══════════════════════════════════════════════════════

        let polarity_shift = self.choice.compute_polarity_shift(
            catalyst,
            learning,
            &body_response,
            &spirit_response,
        );
        self.polarity += polarity_shift;

        ProcessedExperience {
            learning,
            transformation,
            polarity_shift,
            environmental_effect,
            body_response,
            spirit_response,
            unresolved_tension: self.catalyst_backlog,
        }
    }

    fn openness_factor(&self) -> f64 {
        // How open the entity is to new experience.
        // Higher polarity (either direction) = more openness.
        0.5 + self.polarity.abs() * 0.5
    }

    fn learning_efficiency(&self) -> f64 {
        // Diminishing returns on repeated catalyst types.
        // Fresh experiences teach more than familiar ones.
        1.0 / (1.0 + self.catalyst_backlog * 0.01)
    }

    fn consistency_with_self_concept(&self, catalyst: &Catalyst) -> f64 {
        // How well this catalyst aligns with the entity's self-concept.
        // Challenging catalysts (low consistency) require more energy but
        // produce more growth.
        let alignment = catalyst.alignment_with_polarity(self.polarity);
        0.5 + alignment * 0.5
    }
}
```

---

#### 5.2 Dual Experience Engine (Space/Time vs Time/Space)

```rust
/// Dual experience engine: qualitative difference in how entities
/// experience reality above vs below the Veil.
///
/// Gap #11 resolution: Time/Space is experientially different from Space/Time.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "In Space/Time: 3 dimensions of space (freely navigable), 1 dimension
/// of time (linear, irreversible). In Time/Space: 1 dimension of space
/// (fixed locus), 3 dimensions of time (past/present/future accessible)."
pub struct DualExperienceEngine {
    /// Position on the spectrum: 0.0 = pure Space/Time, 1.0 = Veil, 2.0 = pure Time/Space
    pub spectrum_position: f64,
}

pub struct SubjectiveExperience {
    /// How freely the entity can move in space (0.0 = fixed, 1.0 = free)
    pub spatial_freedom: f64,
    /// How freely the entity can navigate time (0.0 = linear, 1.0 = free)
    pub temporal_freedom: f64,
    /// Access to pre-incarnative memory (0.0 = blocked, 1.0 = full)
    pub memory_access: f64,
    /// Perception of other entities as separate selves vs other-selves
    pub other_self_perception: f64,
    /// Whether the Veil is active for this entity
    pub veil_active: bool,
    /// Perceived separation from other entities (1.0 = fully separate)
    pub perceived_separation: f64,
}

impl DualExperienceEngine {
    /// Compute the subjective experience for a given spectrum position.
    pub fn compute_experience(&self, _raw_event: &Event) -> SubjectiveExperience {
        if self.spectrum_position < 1.0 {
            // ═══════════════════════════════════════════════
            // SPACE/TIME EXPERIENCE (below Veil)
            // ═══════════════════════════════════════════════
            // This is normal incarnate experience:
            // - 3D space: can move freely, explore, build
            // - 1D time: moves forward only, no going back
            // - Memory: pre-incarnative plans are FORGOTTEN
            // - Others: appear as SEPARATE beings
            // - Purpose: maximum catalyst, maximum free will
            SubjectiveExperience {
                spatial_freedom: 1.0,
                temporal_freedom: 0.0,
                memory_access: 0.0,
                other_self_perception: 0.0,
                veil_active: true,
                perceived_separation: 1.0 - self.spectrum_position,
            }
        } else {
            // ═══════════════════════════════════════════════
            // TIME/SPACE EXPERIENCE (above Veil)
            // ═══════════════════════════════════════════════
            // This is between-incarnation / higher-density experience:
            // - 1D space: fixed at a locus, cannot freely move
            // - 3D time: can review past, experience present, preview future
            // - Memory: full access to all incarnations
            // - Others: perceived as other-selves (part of same unity)
            // - Purpose: review, planning, healing
            let time_access = (self.spectrum_position - 1.0).min(1.0);
            SubjectiveExperience {
                spatial_freedom: 0.0,
                temporal_freedom: time_access,
                memory_access: time_access,
                other_self_perception: time_access,
                veil_active: false,
                perceived_separation: 0.0,
            }
        }
    }
}
```

---

#### 5.3 Veil Mechanics

```rust
/// Active veil mechanics: filters memory, perception, and awareness
/// based on entity density.
///
/// The Veil is THE critical mechanism for 3rd density experience.
/// Without it, entities would remember their cosmic nature and have
/// no impetus to make The Choice through faith alone.
pub struct VeilMechanics {
    /// Veil intensity (0.0 = fully transparent, 1.0 = fully opaque)
    pub intensity: f64,
}

impl VeilMechanics {
    /// Compute veil intensity for a given density.
    pub fn for_density(density: &Density) -> Self {
        Self {
            intensity: match density {
                Density::First => 0.0,    // No veil for minerals/elements
                Density::Second => 0.0,   // No veil for plants/animals
                Density::Third => 0.9,    // STRONG veil for self-aware beings
                Density::Fourth => 0.4,   // Thinning — telepathy begins
                Density::Fifth => 0.1,    // Nearly transparent — wisdom
                Density::Sixth => 0.0,    // Fully dissolved — unity
                Density::Seventh => 0.0,  // Gateway density
                Density::Eighth => 0.0,   // Return to Infinity
            },
        }
    }

    /// Filter pre-incarnative memory through the Veil.
    /// In 3rd density, the Veil blocks almost all memory of "who you really are."
    /// Occasional "breakthroughs" occur — dreams, déjà vu, mystical experiences.
    pub fn filter_memory(&self, memory: &PreIncarnativeMemory) -> AccessibleMemory {
        let breakthrough_chance = 1.0 - self.intensity;
        if rand::random::<f64>() < breakthrough_chance {
            // Veil breach — entity glimpses deeper reality
            AccessibleMemory::Accessible(memory.clone())
        } else {
            AccessibleMemory::Blocked
        }
    }

    /// Filter perception of other entities through the Veil.
    /// Below the Veil: others appear as separate beings.
    /// Above the Veil: others are recognized as other-selves.
    pub fn filter_perception(&self, other: &Entity) -> PerceivedEntity {
        if self.intensity > 0.5 {
            // Veil active: see physical appearance only
            PerceivedEntity::Separate(other.physical_appearance())
        } else {
            // Veil thin/absent: perceive consciousness directly
            PerceivedEntity::OtherSelf(other.consciousness_signature())
        }
    }

    /// Whether the Veil blocks awareness of cosmic unity.
    pub fn blocks_unity_awareness(&self) -> bool {
        self.intensity > 0.3
    }

    /// Whether the Veil permits telepathic communication.
    pub fn permits_telepathy(&self) -> bool {
        self.intensity < 0.5
    }
}

pub enum AccessibleMemory {
    Accessible(PreIncarnativeMemory),
    Blocked,
}

pub enum PerceivedEntity {
    /// Entity appears as a separate, physical being
    Separate(PhysicalAppearance),
    /// Entity is perceived as another aspect of the One
    OtherSelf(ConsciousnessSignature),
}
```

---

### PHASE 6: SOCIAL EMERGENCE & CIVILIZATION ENGINE (Weeks 27-32)

**Goal:** Entities form relationships, groups, and civilizations. Social Memory Complexes
emerge at 4th density. Harvest mechanics determine density transitions.

**Files to create/modify:**

- `src/social/relationship_web.rs` — Inter-entity relationships
- `src/social/communication.rs` — Density-appropriate communication
- `src/social/civilization.rs` — Group formation, governance, technology
- `src/social/smc_engine.rs` — Social Memory Complex formation
- `src/social/harvest.rs` — Harvest mechanics
- `src/social/wanderer.rs` — Wanderer incarnation system
- Integrate with existing `src/social_memory.rs` (577 lines)
- Integrate with existing `src/harvestability.rs` (475 lines)

**Estimated effort:** 10,000-14,000 new lines, 1,000 lines modified

---

#### 6.1 Relationship System

```rust
/// Inter-entity relationship web.
/// Gap #9 resolution: entities have social dynamics.
pub struct RelationshipWeb {
    pub relationships: HashMap<(EntityId, EntityId), Relationship>,
}

pub struct Relationship {
    pub relationship_type: RelationshipType,
    /// Strength: -1.0 (hostile) to 1.0 (loving)
    pub strength: f64,
    /// Trust: 0.0 (no trust) to 1.0 (complete trust)
    pub trust: f64,
    /// Shared experiences that shaped this relationship
    pub shared_experiences: Vec<SharedExperience>,
    /// Accumulated karmic balance between these entities
    pub karmic_balance: f64,
    /// How long this relationship has existed
    pub duration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipType {
    Parent, Child, Sibling, Mate, Friend, Rival,
    Teacher, Student, Leader, Follower, Stranger,
}

impl RelationshipWeb {
    pub fn update_from_interaction(
        &mut self,
        entity_a: EntityId,
        entity_b: EntityId,
        interaction: &Interaction,
    ) {
        let key = (entity_a.min(entity_b), entity_a.max(entity_b));
        let relationship = self.relationships
            .entry(key)
            .or_insert(Relationship::new_stranger());

        match interaction.interaction_type {
            InteractionType::Cooperative => {
                relationship.strength += 0.01;
                relationship.trust += 0.005;
            }
            InteractionType::Competitive => {
                relationship.strength -= 0.01;
                relationship.trust -= 0.005;
            }
            InteractionType::Teaching => {
                relationship.strength += 0.02;
                relationship.trust += 0.01;
                relationship.relationship_type = RelationshipType::Teacher;
            }
            InteractionType::Hostile => {
                relationship.strength -= 0.05;
                relationship.trust -= 0.02;
                relationship.relationship_type = RelationshipType::Rival;
            }
        }

        relationship.strength = relationship.strength.clamp(-1.0, 1.0);
        relationship.trust = relationship.trust.clamp(0.0, 1.0);
        relationship.shared_experiences.push(SharedExperience {
            interaction_type: interaction.interaction_type,
            time: interaction.time,
            impact: interaction.impact,
        });
    }
}
```

---

#### 6.2 Communication & Civilization

```rust
/// Communication types vary by density level.
pub enum CommunicationType {
    /// 1st Density: No communication (mineral/elemental)
    None,
    /// 2nd Density: Instinctual signaling (body language, pheromones)
    Instinctual { signal_type: InstinctualSignal },
    /// 3rd Density: Verbal language (symbolic, abstract, imprecise)
    Verbal {
        language: LanguageId,
        vocabulary_size: usize,
        abstraction_level: f64,
    },
    /// 4th Density: Telepathic (direct thought, honest, precise)
    Telepathic { bandwidth: f64, honesty: f64 },
    /// 5th+ Density: Direct consciousness sharing
    ConsciousnessShare { depth: f64 },
}

/// Civilization engine: tracks groups, governance, technology, culture.
pub struct CivilizationEngine {
    pub groups: HashMap<GroupId, SocialGroup>,
    pub civilizations: HashMap<CivId, Civilization>,
    pub technologies: HashMap<TechId, Technology>,
}

pub struct Civilization {
    pub id: CivId,
    pub name: String,
    pub population: usize,
    pub territory: Vec<HolographicAddress>,
    pub governance: GovernanceType,
    pub technology_level: f64,
    pub cultural_values: CulturalValues,
    pub average_polarity: f64,  // STO vs STS orientation
    pub consciousness_level: f64,
    pub age: f64,               // How long this civilization has existed
    pub resources: ResourcePool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceType {
    Tribal,           // Small group, elder-led
    Chiefdom,         // Hereditary leadership
    Kingdom,          // Centralized authority
    Republic,         // Representative governance
    Democracy,        // Direct participation
    Technocracy,      // Knowledge-based governance
    Theocracy,        // Cosmological-principle-based
    SocialMemory,     // 4th density — consensus through telepathy
}
```

---

#### 6.3 Social Memory Complex & Harvest

```rust
/// Social Memory Complex engine.
/// Gap #15 resolution: SMC formation mechanics.
pub struct SocialMemoryComplexEngine {
    pub forming_complexes: Vec<FormingSMC>,
    pub active_complexes: Vec<SocialMemoryComplex>,
}

pub struct SocialMemoryComplex {
    pub id: SmcId,
    pub member_entities: Vec<EntityId>,
    pub shared_memory: SharedMemoryField,
    pub collective_consciousness: f64,
    pub density: Density,
    pub polarity: Polarity,
    pub unified_purpose: String,
}

impl SocialMemoryComplexEngine {
    pub fn check_formation(&mut self, entities: &[EmbodiedEntity]) {
        // SMC forms when a group of entities meets ALL criteria:
        // 1. All at 4th density or above
        // 2. All same polarity direction (STO or STS)
        // 3. Sufficient mutual trust/love between ALL members
        // 4. Shared purpose/understanding
        let candidate_groups = self.find_candidate_groups(entities);
        for group in candidate_groups {
            let all_4th_plus = group.iter().all(|e| e.consciousness.density >= Density::Fourth);
            let same_polarity = self.check_polarity_alignment(&group);
            let sufficient_trust = self.check_mutual_trust(&group);
            let shared_purpose = self.check_shared_purpose(&group);

            if all_4th_plus && same_polarity && sufficient_trust && shared_purpose {
                let smc = SocialMemoryComplex {
                    id: SmcId::new(),
                    member_entities: group.iter().map(|e| e.body.id).collect(),
                    shared_memory: SharedMemoryField::merge(&group),
                    collective_consciousness: group.iter()
                        .map(|e| e.consciousness.level)
                        .sum::<f64>() / group.len() as f64,
                    density: Density::Fourth,
                    polarity: group[0].consciousness.dominant_polarity(),
                    unified_purpose: self.derive_purpose(&group),
                };
                self.active_complexes.push(smc);
            }
        }
    }

    fn find_candidate_groups(&self, entities: &[EmbodiedEntity]) -> Vec<Vec<&EmbodiedEntity>> {
        // Cluster entities by proximity, shared relationships, and polarity alignment
        // This is a simplified version — real implementation would use graph clustering
        vec![] // Placeholder for clustering algorithm
    }

    fn check_polarity_alignment(&self, group: &[&EmbodiedEntity]) -> bool {
        if group.is_empty() { return false; }
        let first_polarity = group[0].consciousness.dominant_polarity();
        group.iter().all(|e| e.consciousness.dominant_polarity() == first_polarity)
    }

    fn check_mutual_trust(&self, group: &[&EmbodiedEntity]) -> bool {
        // All pair-wise trust must exceed threshold
        true // Simplified
    }

    fn check_shared_purpose(&self, group: &[&EmbodiedEntity]) -> bool {
        true // Simplified
    }

    fn derive_purpose(&self, group: &[&EmbodiedEntity]) -> String {
        "Collective evolution and service".to_string()
    }
}

/// Harvest system: determines which entities are ready for density transition.
/// Gap #15 resolution: harvest mechanics integrated.
pub struct HarvestSystem {
    /// Length of the harvest cycle (e.g., ~75,000 years for 3rd density)
    pub harvest_cycle_length: f64,
    /// Current progress through the cycle
    pub current_cycle_progress: f64,
}

impl HarvestSystem {
    pub fn check_harvest(&self, entities: &[EmbodiedEntity]) -> HarvestResult {
        let mut harvestable = Vec::new();
        let mut not_ready = Vec::new();

        for entity in entities {
            match entity.consciousness.density {
                Density::Third => {
                    // 3rd → 4th harvest criteria:
                    // STO path: 51% or more service-to-others polarization
                    // STS path: 95% or more service-to-self polarization
                    let sto = entity.consciousness.polarity_sto();
                    let sts = entity.consciousness.polarity_sts();
                    if sto >= 0.51 || sts >= 0.95 {
                        harvestable.push(HarvestCandidate {
                            entity_id: entity.body.id,
                            target_density: Density::Fourth,
                            polarity: if sto >= 0.51 {
                                Polarity::ServiceToOthers
                            } else {
                                Polarity::ServiceToSelf
                            },
                            readiness: (sto.max(sts) * 100.0) as u32,
                        });
                    } else {
                        not_ready.push(entity.body.id);
                    }
                }
                Density::Fourth => {
                    // 4th → 5th: wisdom integration
                    if entity.consciousness.wisdom_score() >= 0.75 {
                        harvestable.push(HarvestCandidate {
                            entity_id: entity.body.id,
                            target_density: Density::Fifth,
                            polarity: entity.consciousness.dominant_polarity(),
                            readiness: 100,
                        });
                    }
                }
                _ => {} // Higher density harvests follow similar patterns
            }
        }

        HarvestResult { harvestable, not_ready }
    }
}

/// Wanderer system: higher-density entities incarnating in lower densities.
pub struct WandererSystem {
    pub active_wanderers: Vec<Wanderer>,
}

pub struct Wanderer {
    pub higher_self_id: EntityId,
    pub incarnation_id: EntityId,
    pub origin_density: Density,
    pub mission: WandererMission,
    pub veil_status: VeilMechanics,   // Wanderers forget too!
    pub awakening_progress: f64,      // 0.0 = fully veiled, 1.0 = fully awakened
}

#[derive(Debug, Clone, PartialEq)]
pub enum WandererMission {
    LightenPlanetaryVibration,
    TeachByExample,
    BalancePlanetaryKarma,
    PrepareForHarvest,
    SpecificService(String),
}
```

---

### PHASE 7: UNIFIED SIMULATION LOOP (Weeks 33-36)

**Goal:** All systems orchestrated in a single, layered tick system. The simulation
runner coordinates field → cosmos → planet → biology → consciousness → social.

**Files to create/modify:**

- `src/simulation_v3/unified_simulation.rs` — New unified tick system
- `src/simulation_v3/adaptive_time.rs` — Scale-adaptive time stepping
- Refactor `src/simulation_v3/simulation_runner.rs` (3,010 lines)

**Estimated effort:** 4,000-6,000 new lines, 3,000 lines modified

---

#### 7.1 Layered Tick Architecture

```rust
/// Unified simulation: all subsystems orchestrated in a single loop.
///
/// Gap #2 resolution: simulation runner integrates ALL subsystems.
pub struct UnifiedSimulation {
    // ── Substrates ──
    pub field: Arc<ObserverDrivenField>,
    pub cosmos: CosmosEngine,

    // ── Planet-level systems ──
    pub planets: HashMap<PlanetId, Planet>,
    pub energy_flows: HashMap<PlanetId, EnergyFlowSystem>,

    // ── Biology ──
    pub cell_engine: CellEngine,
    pub organisms: HashMap<OrganismId, Organism>,
    pub evolution: EvolutionEngine,

    // ── Consciousness ──
    pub entities: HashMap<EntityId, EmbodiedEntity>,
    pub archetype_processor: HashMap<EntityId, ArchetypeProcessor>,

    // ── Social ──
    pub relationships: RelationshipWeb,
    pub civilizations: CivilizationEngine,
    pub smc_engine: SocialMemoryComplexEngine,
    pub harvest: HarvestSystem,
    pub wanderers: WandererSystem,

    // ── Time management ──
    pub time: AdaptiveTimeManager,

    // ── Observer ──
    pub observer: ObserverCamera,

    // ── Events ──
    pub event_log: CosmicEventLog,

    // ── Statistics ──
    pub tick_count: u64,
    pub simulation_time: f64,
}

impl UnifiedSimulation {
    /// Main tick: cascading update from field to social and back.
    pub fn tick(&mut self) {
        let dt = self.time.compute_dt();
        self.tick_count += 1;
        self.simulation_time += dt;

        // ═══════════════════════════════════════════════════════
        // LAYER 1: FIELD TICK
        // Three Primal Distortions update the holographic field.
        // This is the substrate for everything else.
        // ═══════════════════════════════════════════════════════
        self.field.tick(dt);

        // ═══════════════════════════════════════════════════════
        // LAYER 2: COSMOS TICK
        // Stars evolve, orbital mechanics update, new stars may form.
        // ═══════════════════════════════════════════════════════
        self.cosmos.tick(dt);

        // ═══════════════════════════════════════════════════════
        // LAYER 3: PLANET TICKS
        // Geology, hydrology, atmosphere, weather update for each planet.
        // ═══════════════════════════════════════════════════════
        for (planet_id, planet) in &mut self.planets {
            let star = self.cosmos.get_star_for_planet(planet_id);
            planet.tick(star, dt);

            // Update energy flow for this planet
            if let Some(energy) = self.energy_flows.get_mut(planet_id) {
                energy.tick(star, planet);
            }
        }

        // ═══════════════════════════════════════════════════════
        // LAYER 4: BIOLOGY TICK
        // Cells divide, organisms metabolize, species evolve.
        // ═══════════════════════════════════════════════════════
        let env_state = self.compute_environment_state();
        self.cell_engine.tick(&env_state, dt);
        for organism in self.organisms.values_mut() {
            organism.tick(&env_state, dt);
        }
        self.evolution.tick(dt);

        // ═══════════════════════════════════════════════════════
        // LAYER 5: CONSCIOUSNESS TICK
        // Archetype processing, catalyst, choices, polarity evolution.
        // ═══════════════════════════════════════════════════════
        for (entity_id, entity) in &mut self.entities {
            // Compute physics experience at entity's location
            entity.physics_experience = self.compute_physics_for_entity(entity);
            // Run coupled consciousness-body tick
            entity.tick(&env_state, dt);
        }

        // ═══════════════════════════════════════════════════════
        // LAYER 6: SOCIAL TICK
        // Communication, relationships, groups, civilization, SMC.
        // ═══════════════════════════════════════════════════════
        self.update_relationships(dt);
        self.civilizations.tick(dt);
        let entities_vec: Vec<&EmbodiedEntity> = self.entities.values().collect();
        self.smc_engine.check_formation(
            &entities_vec.iter().map(|e| (*e).clone()).collect::<Vec<_>>(),
        );

        // ═══════════════════════════════════════════════════════
        // LAYER 7: FEEDBACK TICK
        // Entity consciousness modifies the holographic field.
        // This is the loop closure: consciousness → field → reality.
        // ═══════════════════════════════════════════════════════
        for (entity_id, entity) in &self.entities {
            let feedback = FieldFeedback {
                coherence_delta: entity.consciousness.field_influence(),
                polarity_influence: entity.consciousness.polarity,
            };
            self.field.apply_entity_feedback(
                *entity_id,
                &entity.body.position,
                feedback,
            );
        }

        // ═══════════════════════════════════════════════════════
        // LAYER 8: OBSERVER TICK
        // Decompress/compress field regions based on camera position.
        // ═══════════════════════════════════════════════════════
        self.field.update_observer_regions(&self.observer);

        // ═══════════════════════════════════════════════════════
        // HARVEST CHECK (periodic)
        // ═══════════════════════════════════════════════════════
        if self.tick_count % 1000 == 0 {
            let harvest_result = self.harvest.check_harvest(
                &self.entities.values().cloned().collect::<Vec<_>>(),
            );
            for candidate in harvest_result.harvestable {
                self.event_log.log(CosmicEvent::DensityTransition {
                    entity_id: candidate.entity_id,
                    from: Density::Third,
                    to: candidate.target_density,
                });
            }
        }
    }

    fn compute_environment_state(&self) -> EnvironmentState {
        // Aggregate environment from all active planets
        // (Simplified: use first planet's state)
        EnvironmentState::default()
    }

    fn compute_physics_for_entity(&self, entity: &EmbodiedEntity) -> PhysicsExperience {
        PhysicsExperience::default()
    }

    fn update_relationships(&mut self, dt: f64) {
        // Check for interactions between nearby entities
        // Update relationship web based on interactions
    }
}
```

---

#### 7.2 Scale-Adaptive Time

```rust
/// Adaptive time management: tick rate depends on observer scale.
///
/// When zoomed to cosmic scale, each tick covers billions of years.
/// When zoomed to cellular scale, each tick covers milliseconds.
pub struct AdaptiveTimeManager {
    /// Current observer scale (determines base dt)
    pub observer_scale: ScaleLevel,
    /// Speed multiplier (user-controlled)
    pub speed_multiplier: f64,
    /// Whether the simulation is paused
    pub paused: bool,
}

impl AdaptiveTimeManager {
    /// Compute the time delta for this tick based on observer scale.
    pub fn compute_dt(&self) -> f64 {
        if self.paused {
            return 0.0;
        }
        let base_dt = match self.observer_scale {
            ScaleLevel::Quantum => 1e-15,       // Femtosecond
            ScaleLevel::Atomic => 1e-9,         // Nanosecond
            ScaleLevel::Molecular => 1e-6,      // Microsecond
            ScaleLevel::Cellular => 60.0,       // Minute
            ScaleLevel::Biological => 3600.0,   // Hour
            ScaleLevel::Planetary => 86400.0,   // Day
            ScaleLevel::Stellar => 3.156e7,     // Year
            ScaleLevel::Cosmic => 3.156e16,     // Billion years
        };
        base_dt * self.speed_multiplier
    }
}
```

---

### PHASE 8: INTERACTIVE EXPERIENCE ENGINE (Weeks 37-40)

**Goal:** Dwarf-Fortress-depth interactive observation with multi-scale zoom, entity
following, inspection, event narration, and time controls.

**Files to create/modify:**

- `src/game/observer_camera.rs` — Multi-scale observation camera
- `src/game/entity_inspector.rs` — Detailed entity inspection
- `src/game/event_system.rs` — Cosmic event log and narration
- `src/game/time_controls.rs` — Time speed controls
- `src/game/mod.rs` — Game loop integration
- Expand existing `src/game/` (currently 126 lines)

**Estimated effort:** 6,000-8,000 new lines

---

#### 8.1 Multi-Scale Camera

```rust
/// Multi-scale observer camera with zoom from quantum to cosmic.
/// Gap #7 resolution: interactive depth.
pub struct ObserverCamera {
    pub position: HolographicAddress,
    pub scale: ScaleLevel,
    pub zoom: f64,
    pub following: Option<EntityId>,
    pub field_of_view: FieldOfView,
}

impl ObserverCamera {
    pub fn zoom_to_scale(&mut self, target: ScaleLevel) {
        self.scale = target;
    }

    pub fn follow_entity(&mut self, entity_id: EntityId) {
        self.following = Some(entity_id);
    }

    pub fn unfollow(&mut self) {
        self.following = None;
    }

    /// Get visible components at the current scale level.
    pub fn get_visible_components(
        &self,
        simulation: &UnifiedSimulation,
    ) -> VisibleScene {
        match self.scale {
            ScaleLevel::Cosmic => self.render_cosmic_web(simulation),
            ScaleLevel::Stellar => self.render_stellar_systems(simulation),
            ScaleLevel::Planetary => self.render_planet_surface(simulation),
            ScaleLevel::Biological => self.render_organisms(simulation),
            ScaleLevel::Cellular => self.render_cells(simulation),
            ScaleLevel::Molecular => self.render_molecules(simulation),
            ScaleLevel::Atomic => self.render_atoms(simulation),
            ScaleLevel::Quantum => self.render_field(simulation),
        }
    }

    fn render_cosmic_web(&self, sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene {
            scale: ScaleLevel::Cosmic,
            stellar_systems: sim.cosmos.stellar_systems.len(),
            visible_entities: 0,
            description: format!(
                "Cosmic web: {} stellar systems, age {:.2e} years",
                sim.cosmos.stellar_systems.len(),
                sim.cosmos.cosmic_age / 3.156e7,
            ),
        }
    }

    fn render_stellar_systems(&self, sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene {
            scale: ScaleLevel::Stellar,
            stellar_systems: sim.cosmos.stellar_systems.len(),
            visible_entities: 0,
            description: "Stellar system view".to_string(),
        }
    }

    fn render_planet_surface(&self, sim: &UnifiedSimulation) -> VisibleScene {
        let entity_count = sim.entities.len();
        VisibleScene {
            scale: ScaleLevel::Planetary,
            stellar_systems: 1,
            visible_entities: entity_count,
            description: format!("Planet surface: {} entities", entity_count),
        }
    }

    fn render_organisms(&self, sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene {
            scale: ScaleLevel::Biological,
            stellar_systems: 0,
            visible_entities: sim.organisms.len(),
            description: format!("Biological view: {} organisms", sim.organisms.len()),
        }
    }

    fn render_cells(&self, sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene {
            scale: ScaleLevel::Cellular,
            stellar_systems: 0,
            visible_entities: sim.cell_engine.cells.len(),
            description: format!("Cellular view: {} cells", sim.cell_engine.cells.len()),
        }
    }

    fn render_molecules(&self, _sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene { scale: ScaleLevel::Molecular, stellar_systems: 0, visible_entities: 0, description: "Molecular view".to_string() }
    }

    fn render_atoms(&self, _sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene { scale: ScaleLevel::Atomic, stellar_systems: 0, visible_entities: 0, description: "Atomic view".to_string() }
    }

    fn render_field(&self, _sim: &UnifiedSimulation) -> VisibleScene {
        VisibleScene { scale: ScaleLevel::Quantum, stellar_systems: 0, visible_entities: 0, description: "Quantum field view".to_string() }
    }
}
```

---

#### 8.2 Entity Inspector

```rust
/// Entity inspector: deep-dive into any entity's complete state.
pub struct EntityInspector {
    pub selected_entity: Option<EntityId>,
}

impl EntityInspector {
    pub fn inspect(&self, entity: &EmbodiedEntity) -> InspectorView {
        InspectorView {
            // Identity
            name: entity.body.genome.species_name.clone(),
            species: entity.body.species,
            density: entity.consciousness.density,
            polarity: entity.consciousness.polarity_summary(),

            // Consciousness
            archetype_activations: entity.consciousness.archetypes,
            current_catalyst: entity.consciousness.current_catalyst_description(),
            consciousness_level: entity.consciousness.level,
            veil_intensity: entity.veil.intensity,

            // Biology
            health: entity.body.health_score(),
            energy: entity.body.energy_store,
            age: entity.body.age,
            cell_count: entity.body.cell_ids.len(),
            body_mass: entity.body.body_plan.mass,

            // Physics
            temperature: entity.physics_experience.temperature,
            gravity: entity.physics_experience.gravity.magnitude(),
            day_phase: entity.physics_experience.day_night_phase,
            season: entity.physics_experience.season,

            // Social
            relationship_count: 0, // Would be computed from RelationshipWeb
            group_membership: vec![],

            // History
            incarnation_count: entity.consciousness.incarnation_count,
            density_transitions: entity.consciousness.density_history.clone(),
        }
    }
}

pub struct InspectorView {
    pub name: String,
    pub species: SpeciesId,
    pub density: Density,
    pub polarity: String,
    pub archetype_activations: [f64; 22],
    pub current_catalyst: String,
    pub consciousness_level: f64,
    pub veil_intensity: f64,
    pub health: f64,
    pub energy: f64,
    pub age: f64,
    pub cell_count: usize,
    pub body_mass: f64,
    pub temperature: f64,
    pub gravity: f64,
    pub day_phase: f64,
    pub season: Season,
    pub relationship_count: usize,
    pub group_membership: Vec<GroupId>,
    pub incarnation_count: u32,
    pub density_transitions: Vec<DensityTransitionRecord>,
}
```

---

#### 8.3 Event System & Narration

```rust
/// Cosmic event log: tracks every significant event in the simulation.
pub struct CosmicEventLog {
    pub events: Vec<CosmicEvent>,
    pub max_events: usize,
}

#[derive(Debug, Clone)]
pub enum CosmicEvent {
    StellarIgnition { star_id: StellarId, spectral_class: SpectralClass },
    PlanetaryFormation { planet_id: PlanetId, star_id: StellarId },
    FirstLife { planet_id: PlanetId, cell_type: CellType },
    FirstMulticellular { planet_id: PlanetId },
    FirstSelfAwareness { entity_id: EntityId, planet_id: PlanetId },
    CivilizationFounded { civ_id: CivId, population: usize },
    TechnologyDiscovered { civ_id: CivId, tech_name: String },
    FirstTelepathy { entity_id: EntityId },
    SocialMemoryFormed { smc_id: SmcId, member_count: usize },
    HarvestOccurred { planet_id: PlanetId, harvested: usize, remaining: usize },
    DensityTransition { entity_id: EntityId, from: Density, to: Density },
    WandererAwakening { wanderer_id: EntityId },
    SpeciesExtinction { species_id: SpeciesId, cause: ExtinctionCause },
    SupernovaExplosion { star_id: StellarId },
    MassExtinction { planet_id: PlanetId, species_lost: usize },
    ArchetypeTransformation { entity_id: EntityId, archetype: u8 },
}

impl CosmicEventLog {
    pub fn log(&mut self, event: CosmicEvent) {
        self.events.push(event);
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }

    pub fn narrate_recent(&self, count: usize) -> Vec<String> {
        self.events
            .iter()
            .rev()
            .take(count)
            .map(|e| Self::narrate_event(e))
            .collect()
    }

    fn narrate_event(event: &CosmicEvent) -> String {
        match event {
            CosmicEvent::StellarIgnition { spectral_class, .. } => {
                format!("A {:?}-class star ignites, flooding its region with light.", spectral_class)
            }
            CosmicEvent::PlanetaryFormation { .. } => {
                "A new world coalesces from the stellar accretion disk.".to_string()
            }
            CosmicEvent::FirstLife { .. } => {
                "Life stirs! The first living cells emerge from the molecular soup.".to_string()
            }
            CosmicEvent::FirstSelfAwareness { .. } => {
                "A being looks up at the stars and asks: 'Who am I?'".to_string()
            }
            CosmicEvent::SocialMemoryFormed { member_count, .. } => {
                format!("{} beings merge into a Social Memory Complex, thinking as one.", member_count)
            }
            CosmicEvent::HarvestOccurred { harvested, remaining, .. } => {
                format!("The Harvest: {} souls graduate to the next density. {} remain.", harvested, remaining)
            }
            CosmicEvent::DensityTransition { from, to, .. } => {
                format!("An entity transitions from {:?} to {:?} density.", from, to)
            }
            CosmicEvent::WandererAwakening { .. } => {
                "A Wanderer remembers who they truly are.".to_string()
            }
            CosmicEvent::SupernovaExplosion { .. } => {
                "A star explodes in supernova, seeding the cosmos with heavy elements!".to_string()
            }
            _ => format!("{:?}", event),
        }
    }
}
```

---

#### 8.4 Time Controls

```rust
/// Time speed controls for the observer.
pub struct TimeController {
    pub speed: TimeSpeed,
    pub paused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSpeed {
    Realtime,     // 1 second = 1 second
    Fast10x,      // 1 second = 10 seconds
    Fast100x,     // 1 second = 100 seconds
    Fast1000x,    // 1 second = ~17 minutes
    Hour,         // 1 second = 1 hour
    Day,          // 1 second = 1 day
    Year,         // 1 second = 1 year
    Century,      // 1 second = 100 years
    Millennium,   // 1 second = 1,000 years
    Epoch,        // 1 second = 1,000,000 years
    Eon,          // 1 second = 1,000,000,000 years
}

impl TimeSpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            Self::Realtime => 1.0,
            Self::Fast10x => 10.0,
            Self::Fast100x => 100.0,
            Self::Fast1000x => 1000.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
            Self::Year => 3.156e7,
            Self::Century => 3.156e9,
            Self::Millennium => 3.156e10,
            Self::Epoch => 3.156e13,
            Self::Eon => 3.156e16,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Realtime => "Realtime",
            Self::Fast10x => "10x",
            Self::Fast100x => "100x",
            Self::Fast1000x => "1000x",
            Self::Hour => "Hour/sec",
            Self::Day => "Day/sec",
            Self::Year => "Year/sec",
            Self::Century => "Century/sec",
            Self::Millennium => "Millennium/sec",
            Self::Epoch => "Epoch/sec",
            Self::Eon => "Eon/sec",
        }
    }
}
```

---

## PART III: R&D INVESTIGATION AREAS

These are open research questions that need investigation before or during implementation.

### Research Area 1: Archetype-to-DNA Mapping

**Question:** How do the 22 archetypes map to nucleotide sequences? What archetype
activation patterns produce carbon-based life vs silicon-based?

**Current state:** `src/entity_layer7/dna_encoding.rs` defines `DNAPattern` and
`src/biology/dna_system.rs` defines DNA replication. But there is no mapping between
archetype coefficients and DNA sequence generation.

**Hypothesis:** The 22 archetype coefficients can be treated as a 22-dimensional vector
that maps to a point in "body plan space." Different regions of this space correspond to
different biological chemistries:

```rust
/// Hypothetical archetype-to-chemistry mapping.
pub fn archetype_to_chemistry(archetypes: &[f64; 22]) -> Chemistry {
    let body_mind_ratio = archetypes[7..14].iter().sum::<f64>()
        / archetypes[0..7].iter().sum::<f64>().max(0.001);
    if body_mind_ratio > 2.0 {
        Chemistry::Silicon // Body-dominant: silicon-based
    } else {
        Chemistry::Carbon // Mind-dominant: carbon-based
    }
}
```

**Research needed:** Literature review of theoretical xenobiology. Mathematical framework
for mapping high-dimensional archetype space to biochemistry.

---

### Research Area 2: Consciousness-Field Coupling Constant

**Question:** What is the mathematical relationship between entity consciousness evolution
and field modification? How strong should the feedback be?

**Challenge:** Too strong → simulation destabilizes as entities rapidly modify the field.
Too weak → consciousness has no effect on reality (violates the cosmological architecture).

**Hypothesis:** The coupling constant should scale with density:

```rust
pub fn consciousness_field_coupling(density: &Density) -> f64 {
    match density {
        Density::First => 0.0,       // No coupling (no awareness)
        Density::Second => 0.0001,   // Minimal
        Density::Third => 0.001,     // Noticeable with sustained effort
        Density::Fourth => 0.01,     // Significant
        Density::Fifth => 0.1,       // Strong
        Density::Sixth => 0.5,       // Reality bends to will
        Density::Seventh => 1.0,     // Full creator
        Density::Eighth => f64::INFINITY, // Is the field
    }
}
```

**Research needed:** Stability analysis of coupled consciousness-field system. Phase
diagrams showing stable operating regimes.

---

### Research Area 3: Larson-Newtonian Unification

**Question:** How to implement Newtonian gravity (entities FEEL it) within Larson's
Reciprocal System where motion is primary and matter is secondary?

**Current state:** `src/simulation_v3/scale_physics.rs` (12,389 lines) implements
scale-specific physics but uses Newtonian formulations. The cosmological architecture
references Larson's reciprocal framework where space and time are reciprocal aspects of
motion.

**Hypothesis:** Newtonian gravity emerges as an EFFECTIVE THEORY within the Reciprocal
System. Entities experience Newtonian gravity at their scale, but the underlying mechanism
is reciprocal motion ratios:

```rust
/// Effective gravity from reciprocal motion ratios.
pub fn effective_gravity(
    space_time_ratio: f64,
    field_coherence: f64,
    mass_equivalent: f64,
) -> f64 {
    // In the Reciprocal System, gravity is the inward aspect of
    // the space/time progression. It manifests as acceleration
    // proportional to mass and inversely proportional to r².
    let g_constant = 6.674e-11;
    g_constant * mass_equivalent * field_coherence / space_time_ratio.powi(2)
}
```

**Research needed:** Formal derivation of Newtonian limit from reciprocal motion framework.

---

### Research Area 4: Scale-Bridging Physics

**Question:** How does quantum coherence at 10^-35m affect entity consciousness at 10^0m?
What is the inter-scale coupling through the MERA network?

**Current state:** `src/compression/mera_network.rs` defines the MERA (Multiscale
Entanglement Renormalization Ansatz) structure, but it's used only for data compression,
not for physics bridging.

**Hypothesis:** The MERA network naturally provides scale bridging. Information at fine
scales propagates to coarse scales through the isometry tensors. Quantum coherence at the
Planck scale creates subtle biases at biological scales that manifest as "intuition."

**Research needed:** Numerical experiments with MERA propagation. Can small quantum
effects at fine scales produce measurable bias at coarse scales?

---

### Research Area 5: Emergent Language

**Question:** Can language genuinely emerge from entity communication attempts? What are
the minimum conditions for symbolic communication to develop?

**Hypothesis:** Language emerges when entities need to coordinate complex behavior but
lack telepathy. The minimum conditions are:

1. Entities with 3+ different behavioral goals
2. Cooperative advantage from coordination
3. Vocal/gestural communication channel
4. Memory of past interactions
5. Theory of mind (ability to model other entity's state)

```rust
/// Emergent language formation check.
pub fn language_emergence_conditions(
    neural_complexity: f64,
    social_group_size: usize,
    coordination_need: f64,
) -> bool {
    neural_complexity > 0.6        // Sufficient brain
        && social_group_size > 5   // Need for group coordination
        && coordination_need > 0.3 // Advantage from communicating
}
```

**Research needed:** Agent-based modeling of language emergence. Literature review of
computational linguistics and the evolution of communication.

---

### Research Area 6: Social Memory Complex Dynamics

**Question:** What mathematical model best describes collective consciousness formation?
How do individual memories merge into a shared field?

**Hypothesis:** SMC formation follows a phase transition model. Individual consciousnesses
are like magnetic domains — when they align sufficiently (shared polarity, mutual trust),
they undergo spontaneous magnetization into a unified field:

```rust
/// Ising-model-inspired SMC formation.
pub fn smc_order_parameter(
    trust_matrix: &[Vec<f64>],  // N×N trust between entities
    polarity_alignment: f64,
) -> f64 {
    let n = trust_matrix.len();
    let avg_trust: f64 = trust_matrix.iter()
        .flat_map(|row| row.iter())
        .sum::<f64>() / (n * n) as f64;

    // Order parameter: product of alignment and coupling
    polarity_alignment * avg_trust
    // Phase transition at order_parameter ≈ 0.7
}
```

**Research needed:** Ising model analogies for consciousness. Phase transition dynamics
of collective awareness formation.

---

## PART IV: DEPENDENCY GRAPH

```
Phase 1 (Field Substrate)
    │
    ├──────────────────────────────────────────┐
    │                                          │
    ▼                                          ▼
Phase 2 (Cosmos Engine)                 Phase 5 (Consciousness)
    │                                          │
    ▼                                          │
Phase 3 (Living Planets)                       │
    │                                          │
    ▼                                          │
Phase 4 (Biology)                              │
    │                                          │
    ├──────────────────────────────────────────┘
    │
    ▼
Phase 6 (Social Emergence)
    │
    ▼
Phase 7 (Unified Simulation Loop)
    │
    ▼
Phase 8 (Interactive Experience)
```

**Critical path:** Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 7 → Phase 8

**Parallel track:** Phase 1 → Phase 5 (can be developed in parallel with Phases 2-4)

**Integration point:** Phase 6 requires BOTH Phase 4 (biology) AND Phase 5 (consciousness)

**Final integration:** Phase 7 wires everything together, Phase 8 makes it observable

---

## PART V: SUCCESS CRITERIA

Each criterion maps to a testable property of the running simulation:

| # | Criterion | Test | Target |
|---|-----------|------|--------|
| 1 | Universe self-generates from field coherence | Run genesis with random seed, verify stars form | Stars ignite without explicit placement |
| 2 | Stars ignite, planets form, life emerges WITHOUT scripting | Run simulation from genesis to first life | Life emergence in <10K cosmic ticks |
| 3 | Entities have biological bodies that metabolize, age, die | Check entity body has cells, energy, age | Cell count > 0, energy depletes, age increases |
| 4 | Entity behavior emerges from archetype interference | No behavior trees; behavior from archetype processing | BehaviorState derived from ArchetypeProcessor output |
| 5 | Civilizations form, develop technology, governance | Multiple entities → groups → civilization | CivId count > 0 after sufficient social ticks |
| 6 | Social Memory Complexes form at 4th density | 4th density entities with shared polarity → SMC | SmcId count > 0 when conditions met |
| 7 | Harvest occurs based on polarity | 51% STO or 95% STS → density transition | HarvestResult.harvestable non-empty for qualifying entities |
| 8 | Observable at any scale zoom level | Camera zoom from Quantum to Cosmic | VisibleScene rendered at all 8 ScaleLevels |
| 9 | Performance: 1000+ entities at full detail | Benchmark with 1000 embodied entities | >10 ticks/second on release build |
| 10 | Infinite addressable space with O(log n) lookup | Address resolution benchmark | <1μs lookup for any address depth |

---

## PART VI: RISK ANALYSIS & MITIGATIONS

| Risk | Severity | Probability | Mitigation |
|------|----------|-------------|------------|
| **Performance collapse** with full biology simulation | High | High | Hierarchical tick rates: cells tick at 1/100 of entity rate. LOD system for distant organisms. Pool allocator for cells. |
| **Field instability** from consciousness-field coupling | High | Medium | Coupling constant tuning. Stability analysis. Damping terms in field equation. Maximum perturbation per tick. |
| **Memory explosion** from cell-level simulation | High | High | Object pooling. Max cell count per organism. Compressed cell representation for background organisms. |
| **Complexity explosion** in 8-phase integration | Medium | High | Incremental integration tests. Each phase produces a running (if partial) simulation. CI/CD pipeline with regression tests. |
| **Behavioral degeneracy** (entities converge to same behavior) | Medium | Medium | Free will seed ensures unique evolution paths. Catalyst diversity from physics experience. Archetype coefficient variation. |
| **Speciation too fast/slow** | Low | Medium | Tunable speciation thresholds. Genetic diversity accumulation rate as parameter. Population genetics validation against real-world data. |
| **Observer-driven decompression latency** | Medium | Medium | Predictive loading (decompress regions the observer is moving toward). Background decompression thread. LRU cache for recently observed regions. |
| **Cosmic event flooding** | Low | Low | Event importance ranking. Only log events above threshold. Circular buffer with configurable size. |

---

## PART VII: PERFORMANCE BUDGET

### Per-Tick Compute Budget (targeting 10 ticks/second)

| Subsystem | Budget | Notes |
|-----------|--------|-------|
| Field tick | 10 ms | MERA operations on compressed network |
| Cosmos tick | 5 ms | Only active stellar systems (near observers) |
| Planet ticks | 15 ms | Atmosphere, hydrology, geology (1-3 planets active) |
| Biology tick | 30 ms | Cell engine (max 10K active cells), organisms |
| Consciousness tick | 20 ms | Archetype processing for 1000 entities |
| Social tick | 10 ms | Relationship updates, group dynamics |
| Feedback tick | 5 ms | Entity → field feedback |
| Observer tick | 5 ms | Decompression/compression management |
| **Total** | **100 ms** | **= 10 ticks/second** |

### Memory Budget (targeting 4 GB max)

| Component | Budget | Notes |
|-----------|--------|-------|
| Compressed field (MERA) | 500 MB | Entire universe compressed |
| Active decompressed regions | 500 MB | 3-5 regions at planetary scale |
| Cell pool | 500 MB | 10K cells × ~50 KB each |
| Organisms | 200 MB | 1K organisms × ~200 KB each |
| Entities (embodied) | 500 MB | 1K entities × ~500 KB each |
| Stellar systems | 200 MB | Active systems only |
| Relationship web | 100 MB | N² relationships, sparse storage |
| Event log | 100 MB | Circular buffer |
| Working memory | 400 MB | Temporaries, buffers, caches |
| **Total** | **3 GB** | **Under 4 GB target** |

---

## PART VIII: TESTING STRATEGY

### Unit Tests (per module)

```rust
#[cfg(test)]
mod field_tests {
    use super::*;

    #[test]
    fn test_holographic_address_distance() {
        let a = HolographicAddress::cosmic_origin();
        let b = a.refine(CoherenceStep { density_band: 3, octant: 0, depth: 0 });
        assert!(a.distance_to(&b) > 0.0);
    }

    #[test]
    fn test_observer_driven_decompression() {
        let field = ObserverDrivenField::new(FieldConfig::default());
        let observer = Observer {
            id: ObserverId(1),
            address: HolographicAddress::cosmic_origin(),
            field_of_view: FieldOfView { angular_extent: 1.0, depth: 3, resolution: 64 },
            scale: ScaleLevel::Planetary,
            observer_type: ObserverType::ExternalCamera,
        };
        // After registering observer, region should be decompressed
        assert!(field.get_observer_view(&observer.id).is_none()); // Before tick
    }

    #[test]
    fn test_cell_division() {
        let mut engine = CellEngine::new();
        // Create a cell with enough energy to divide
        let cell = LiveCell {
            id: CellId(0),
            cell_type: CellType::Stem,
            dna: DNA::default(),
            energy: 300.0,
            health: 1.0,
            age: 2.0,
            division_count: 0,
            max_divisions: 50,
            position: HolographicAddress::cosmic_origin(),
            metabolic_rate: 1.0,
            waste_products: 0.0,
            organism_id: None,
            epigenetic_state: EpigeneticState::default(),
        };
        engine.cells.push(cell);
        engine.tick(&EnvironmentState::default(), 1.0);
        // Should have divided
        assert!(engine.cells.len() >= 1);
    }
}
```

### Integration Tests (cross-module)

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_star_to_planet_radiation_chain() {
        let star = Star::ignite_from_proto_region(&ProtoStellarRegion::default());
        let planet = Planet::form_from_accretion(1.0, EARTH_MASS, &HolographicAddress::cosmic_origin(), &Vector3::unit_z());
        let radiation = star.radiation_at_distance(planet.orbital_radius);
        assert!(radiation > 0.0, "Planet should receive radiation from star");
        assert!(radiation < star.luminosity * 3.828e26, "Radiation should decrease with distance");
    }

    #[test]
    fn test_archetype_catalyst_processing() {
        let mut processor = ArchetypeProcessor::default();
        let catalyst = Catalyst::new(CatalystType::Encounter, 0.5);
        let result = processor.process_catalyst(&catalyst);
        assert!(result.learning >= 0.0, "Learning should be non-negative");
    }

    #[test]
    fn test_harvest_polarity_threshold() {
        let harvest = HarvestSystem { harvest_cycle_length: 75000.0, current_cycle_progress: 74999.0 };
        // Entity with 51% STO should be harvestable
        let sto_entity = create_test_entity_with_polarity(0.51, 0.0);
        // Entity with 50% STO should NOT be harvestable
        let not_ready = create_test_entity_with_polarity(0.50, 0.0);
        let result = harvest.check_harvest(&[sto_entity, not_ready]);
        assert_eq!(result.harvestable.len(), 1);
        assert_eq!(result.not_ready.len(), 1);
    }

    #[test]
    fn test_unified_tick_completes() {
        let mut sim = UnifiedSimulation::new_minimal();
        // Should not panic
        sim.tick();
        assert!(sim.tick_count == 1);
        assert!(sim.simulation_time > 0.0);
    }
}
```

### Benchmarks

```rust
// benches/simulation_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_field_tick(c: &mut Criterion) {
    let mut field = ObserverDrivenField::new(FieldConfig::default());
    c.bench_function("field_tick", |b| {
        b.iter(|| field.tick(0.001))
    });
}

fn bench_cell_engine_1000(c: &mut Criterion) {
    let mut engine = CellEngine::with_cells(1000);
    let env = EnvironmentState::default();
    c.bench_function("cell_engine_1000", |b| {
        b.iter(|| engine.tick(&env, 1.0))
    });
}

fn bench_archetype_processing(c: &mut Criterion) {
    let mut processor = ArchetypeProcessor::default();
    let catalyst = Catalyst::new(CatalystType::Encounter, 0.5);
    c.bench_function("archetype_process", |b| {
        b.iter(|| processor.process_catalyst(&catalyst))
    });
}

fn bench_unified_tick_100_entities(c: &mut Criterion) {
    let mut sim = UnifiedSimulation::with_entities(100);
    c.bench_function("unified_tick_100", |b| {
        b.iter(|| sim.tick())
    });
}

criterion_group!(
    benches,
    bench_field_tick,
    bench_cell_engine_1000,
    bench_archetype_processing,
    bench_unified_tick_100_entities,
);
criterion_main!(benches);
```

---

## CONCLUSION

HoloSim Infinite has an extraordinary foundation — **335,727 lines of Rust** implementing
physics engines, archetype systems, biological models, holographic fields, consciousness
mechanics, and more. The individual subsystems are substantial and well-structured.

The critical work is **INTEGRATION**: wiring disconnected islands into a living, breathing
cosmos where each tick cascades from field through cosmos through planet through biology
through consciousness through social and back to field.

The 8-phase roadmap transforms the simulation from a collection of impressive but isolated
modules into a **Dwarf-Fortress-depth consciousness evolution simulator** where:

1. **Reality emerges** from Intelligent Infinity through the Three Primal Distortions
2. **Stars ignite** from field coherence peaks, not from explicit construction
3. **Planets form** from accretion disks, with dynamic geology, hydrology, and atmosphere
4. **Life emerges** from chemistry, with cells that divide and species that evolve
5. **Consciousness develops** through the 22-archetype cycle, not behavior trees
6. **Civilizations rise** through social dynamics, governance evolution, and technology
7. **Social Memory Complexes** crystallize when groups achieve telepathic unity
8. **Harvest** determines which souls graduate to the next density of experience
9. **All of this** is observable at any zoom level, from quantum field to cosmic web
10. **The Veil** creates the dramatic tension of forgetting — making The Choice meaningful

The estimated timeline is **40 weeks** (10 months) for a solo developer, or **20 weeks**
(5 months) with a two-person team. The critical path runs through Phases 1→2→3→4→7→8,
with Phase 5 developable in parallel.

The foundation is built. The subsystems exist. **Now we wire them together.**

---

**Version:** 4.0
**Date:** February 18, 2026
**Status:** R&D Roadmap — Ready for Phase 1 Implementation
**Total estimated new code:** ~65,000-80,000 lines
**Total estimated modified code:** ~15,000-20,000 lines
**Existing codebase:** 335,727 lines across 446 files
