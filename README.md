# HoloSim Infinite

**A holographic universe simulation engine — the substrate for emergent games.**

> **The Problem**: Traditional game engines (Unity, Unreal) are "physics-first," simulating matter and struggling to produce emergent intelligence. This limits the scale and variety of behavior possible in simulation, as intelligence is usually bolted on via rigid behavior trees or scripted events rather than emerging from the system itself.

## Engineering Highlights

### Holographic Optimization Framework
To enable simulation at a cosmic scale, I implemented a universal template (`UniversalTemplate<T>`) where every entity, from a quantum particle to a galaxy, follows the same structural pattern. To manage the resulting data, I utilized **MERA (Multi-scale Entanglement Renormalization Ansatz) tensor compression**, reducing memory scaling from $O(n)$ to $O(n^{2/3})$. This allows the simulation of massive entity counts with a fraction of the traditional memory footprint.

### Consciousness Architecture & Archetype Interference
Instead of traditional AI, I built a system based on **Archetype Interference**. Behavior emerges from the interaction of 22 orthogonal basis vectors (representing Mind, Body, Spirit, and Choice). This is coupled with a **Free Will Kernel** that ensures non-deterministic but reproducible choice-making for every entity, allowing for true emergence and authentic behavioral variety.

### Fractal Multi-Scale Simulation
I implemented a seamless simulation across 9 scale levels, from the Quantum ($10^{-35}m$) to the Universal ($10^{26}m$). By using **Fractal Caching**, the engine enables instant zoom across 61 orders of magnitude without loading screens. The entire universe is treated as a single, continuous holographic field where scale transitions are merely parameter changes rather than system switches.

### Emerging Systems
- **Biology Pipeline** — Quantum → atoms → molecules → cells → organisms (19 files, ~40% functional)
- **Physics (Dual-Mode)** — Archetype-derived forces + Rapier3d rigid-body (8+ files, ~50% functional)
- **Civilization** — Settlements, culture, population dynamics, trade routes
- **Chemistry** — Elements as stable attractor fields, molecular bonding

---

## Quick Start

### Prerequisites

- **Rust 1.70+**
- **GPU with WGPU support** (Vulkan, OpenGL, DirectX 12, or Metal)
- **8GB RAM recommended** (4GB minimum)

### Clone & Build

```bash
git clone https://github.com/ishan-parihar/holosim-infinite.git
cd holosim-infinite
cargo build --release
```

### Run the Simulation

```bash
# Terminal simulation (default: 128 entities, 100 steps)
cargo run --release --bin holonic_realms

# Custom parameters
cargo run --release --bin holonic_realms -- --entities 256 --steps 1000
```

### Run the GUI

```bash
# Primary GUI (WGPU + EGUI)
cargo run --release --bin holonic_gui_complete

# If WGPU fails on your system, force OpenGL:
WGPU_BACKEND=gl cargo run --release --bin holonic_gui_complete

# Alternative: SDL2 GUI (requires SDL2 system libraries)
cargo run --release --bin holonic_sdl2 --features sdl2
```

**SDL2 system dependencies:**
```bash
# Debian/Ubuntu
sudo apt install libsdl2-dev libsdl2-mixer-dev

# Fedora
sudo dnf install SDL2-devel SDL2_mixer-devel

# Arch
sudo pacman -S sdl2 sdl2_mixer

# macOS
brew install sdl2 sdl2_mixer
```

---

## Project Structure

```
src/
├── simulation_v3/            # Main simulation engine (82 files)
│   ├── simulation_runner.rs  # Orchestration
│   ├── entity_lifecycle.rs   # Per-entity evolution
│   ├── holographic_field.rs  # Interference network
│   ├── mera_network.rs       # MERA tensor compression
│   ├── collective_dynamics.rs # Group behavior
│   └── ...                   # 77 more files
├── holographic_foundation/   # Unified field system (97 files)
│   ├── distortions/          # FreeWill, Love, Light field terms
│   ├── intelligent_infinity/ # Source, feedback, patterns
│   ├── spectrum/             # Density bands, veil crossing
│   ├── cellular_emergence/   # Cell manifestation
│   ├── ecosystem_dynamics/   # Species, trophic networks
│   └── ...                   # 10 more sub-modules
├── gui/                      # Multi-scale visualization (91 files)
│   ├── application.rs        # Main GUI coordinator
│   ├── renderer/             # WGPU renderers (17 files)
│   ├── visualization/        # Scale-specific viz (15 files)
│   ├── ui/                   # EGUI panels (8 files)
│   └── simulation_adapter.rs # Bridges sim → GUI
├── archetypes/               # 22-archetype system (33 files)
├── biology/                  # Biological emergence (19 files)
├── foundation/               # Violet → Green realms (12 files)
├── spectrum/                 # Yellow → Red realms (9 files)
├── consciousness/            # Free Will kernel (7 files)
├── physics/                  # Dual physics system (5 files)
├── physics_rapier/           # Rapier3d integration (3 files)
├── quantum/                  # Entanglement, collapse (4 files)
├── civilization/             # Settlements, culture (5 files)
├── chemistry/                # Elements as attractors (5 files)
├── holographic/              # Core holographic mechanisms (13 files)
├── compression/              # Tensor networks, MERA (5 files)
└── [58 total modules, 650+ .rs files]
```

**Binary targets:**
| Binary | Purpose | Dependencies |
|--------|---------|-------------|
| `holonic_realms` | Terminal simulation | None |
| `holonic_gui_complete` | Primary WGPU GUI | wgpu, egui, winit, tokio |
| `holonic_sdl2` | Alternative SDL2 GUI | SDL2 (system lib) |
| `physical_manifestation_demo` | Physics demo | None |

---

## Architecture Overview

### Creation Cascade (Top-Down Involution)

```
Intelligent Infinity (Source)
    │
    ▼ First Distortion: Free Will
Violet Realm — Undifferentiated Unity
    │
    ▼ Second Distortion: Love/Logos
Indigo Realm — IntelligentInfinity + Archetype 22 (The Choice)
    │
    ▼ Third Distortion: Light
Blue Realm — Logos + Universal Archetypical Patterns
    │
    ▼
Green Realm — Light/Love Field of Potential
    │
    ▼ Mysterious Emergence
Yellow Realm — Dimensions + Spectrum + Veil at v=1
    │
    ▼ Galactic Configuration
Orange Realm — Galactic-Scale Spectrum
    │
    ▼ Solar Configuration
Red Realm — Solar-Scale Spectrum
    │
    ▼
Layer 7 — Individual Entities (SubSubLogos)
    │
    ▼ Evolution through 8 Densities
```

### The Holographic Template

Every element in the simulation follows the same template:

```rust
UniversalTemplate<T> {
    field: Arc<HolographicField>,           // Shared across all instances
    spectrum: SpectrumConfiguration,        // Space/Time ↔ Time/Space ratio
    archetype_activation: [f64; 22],        // 22 archetype coefficients
    density: Density,                       // 1st → 8th density
    free_will_seed: u64,                    // Non-deterministic choice seed
    component_data: T,                      // Entity, Particle, World, Star, etc.
}
```

The first 5 fields are **identical** for everything. Only `component_data` varies. This means:
- Implement holographic logic **once** → it applies to everything
- Behavior emerges from archetype interference, not behavior trees
- Scale transitions are just parameter changes, not system switches

---

## Current State

### What Works
- Cosmological architecture: **fully implemented** (Violet → Layer 7, 8 densities, 22 archetypes)
- Holographic optimization: **implemented** (UniversalTemplate, MERA, FractalCache, multi-scale field)
- Free Will system: **extensively implemented** (10,000+ lines across 12 files)
- Archetype interference: **implemented** (basis, cache, emergent behavior engine)
- Biology pipeline: **partial** (quantum → organisms, ~40% functional)
- Physics: **partial** (dual-mode, archetype-derived + Rapier, not yet unified)
- GUI infrastructure: **extensive code** (91 files), improving functionality

### What's In Progress
- GUI visibility fixes (WGPU multi-backend fallback, entity rendering)
- Simulation adapter data pipes (cosmic, planet, holographic field data)
- Observation Layer design (mapping archetypes → observable physical properties)

### What's Planned
- Bevy ECS integration (game-facing API layer)
- GPU acceleration (MERA tensors → compute shaders)
- Physics unification (archetype forces → Rapier → rendering)
- Fluid dynamics (SPH), thermodynamics engine
- World editor, serialization, networking

See the full analysis in [STRATEGIC_INTELLIGENCE_REPORT_2026.md](STRATEGIC_INTELLIGENCE_REPORT_2026.md) and [GAMING_ENGINE_ROADMAP_v2.md](GAMING_ENGINE_ROADMAP_v2.md).

---

## Testing

```bash
# All tests
cargo test --release

# With output
cargo test --release -- --nocapture

# Specific test
cargo test --release test_entity_has_evolution_clock
```

---

## Documentation

| Document | Purpose |
|----------|---------|
| [STRATEGIC_INTELLIGENCE_REPORT_2026.md](STRATEGIC_INTELLIGENCE_REPORT_2026.md) | Competitive landscape, tech trends, roadmap |
| [GAMING_ENGINE_ROADMAP_v2.md](GAMING_ENGINE_ROADMAP_v2.md) | Fractal-holographic engine roadmap |
| [HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md](docs/roadmaps/HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md) | MERA compression, template architecture |
| [COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md](COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md) | 8-phase refactor plan |
| [SIMULATION_AUDIT_REPORT.md](SIMULATION_AUDIT_REPORT.md) | Brutal reality assessment |
| [IMPLEMENTATION_GAP_REPORT.md](IMPLEMENTATION_GAP_REPORT.md) | Gap analysis |
| [HOLOSIM_MASTER_CONTEXT.md](HOLOSIM_MASTER_CONTEXT.md) | Master context document |
| [AGENTS.md](AGENTS.md) | AI assistant coding guide |
| [QUICK_START_GUIDE.md](QUICK_START_GUIDE.md) | Quick start guide |
| `docs/` | API, architecture, benchmarks, phase findings |

---

## Development

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Lint
cargo clippy --all-targets --all-features

# Format
cargo fmt

# Clean build
cargo clean && cargo build --release
```

---

## Philosophy

> *"Transcend and Include"* — Each layer of the simulation includes all previous layers while transcending them. An organism includes cells, which include molecules, which include atoms, which include quantum fields. But unlike traditional simulation, each layer also contains the **whole** — the holographic principle.

This is not just a narrative device. It's a **computational optimization strategy**:
- Store surface representation, reconstruct volume on-demand (holographic principle)
- Compress data using MERA tensor networks (multi-scale entanglement renormalization)
- Share references across layers instead of copying data (transcend and include)
- Generate behavior from 22 archetype coefficients instead of behavior trees (emergence)

The result: exponential improvements in memory, computation, storage, and load times — while maintaining true emergence.

---

**Version:** 10.0  
**Language:** Rust 2021 Edition  
**License:** See [LICENSE](LICENSE)  
**Based on:** The Law of One material (Ra sessions)

---

Developed by [Ishan Parihar](https://github.com/ishanparihar) — If you find this useful, [consider supporting](https://rzp.io/rzp/ishan-parihar)
