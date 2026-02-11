# Holographic Architecture Simulation

A comprehensive simulation of the Law of One cosmological principles, demonstrating holographic architecture, density octave evolution, and the emergence of consciousness through free will.

## 🌟 Overview

This simulation implements an 8-phase comprehensive refactor that transforms the system from an algorithmic, serial system to an organic, individuated, holographic creation simulation based on the COSMOLOGICAL-ARCHITECTURE.md specification.

### Key Features

- ✅ **True Individualization**: Each entity evolves independently with unique spectrum configuration
- ✅ **Complete Visualization**: All portions of creation visible and understandable
- ✅ **Organic Emergence**: Probabilistic evolution through attractor fields and resonance
- ✅ **Physical Manifestation**: All density sub-levels with physical structures
- ✅ **Logos Hierarchy**: Galactic Logoi → Solar Logoi → Individual entities
- ✅ **Holographic Coherence**: Collectives form through resonance, not proximity
- ✅ **Spectrum Differentiation**: Clear space/time vs time/space differentiation with Veil effect
- ✅ **Comprehensive Testing**: 26 tests covering all phases

## 🎯 What's New (Version 9.0)

### All 8 Phases Complete

1. **Phase 1**: Foundation - True Individualization
2. **Phase 2**: Spectrum Visualization
3. **Phase 3**: Physical Manifestation System
4. **Phase 4**: Logos Hierarchy
5. **Phase 5**: Organic Evolution
6. **Phase 6**: Holographic Coherence
7. **Phase 7**: Space/Time vs Time/Space Differentiation
8. **Phase 8**: Integration and Testing

See [COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md](COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md) for details.

### Latest Results

- **Architecture Alignment**: 84.62%
- **Total Entities**: 188
- **Total Transitions**: 226
- **Execution Time**: 42.77s (128 entities, 100 steps)
- **All Phases**: ✅ Complete and Integrated

See [PHASE_8_VALIDATION_REPORT.md](PHASE_8_VALIDATION_REPORT.md) for full validation results.

### SDL2 GUI Integration (New!)

The simulation now includes a professional-grade SDL2 + WGPU visualization system with:

- **Real-time Rendering**: 4,000-5,000+ FPS (67-83x target performance)
- **Full Visualization**: Entity, spectrum, emergence, and collective dynamics
- **Interactive Controls**: Camera pan/zoom, visualization toggles
- **Cross-Platform**: Linux, Windows, macOS support
- **Wayland Ready**: Native Wayland support with X11 fallback

**Quick Start:**
```bash
# Build SDL2 GUI
cargo build --release --features sdl2 --bin holonic_sdl2

# Run (X11 backend recommended for Wayland)
SDL_VIDEO_DRIVER=x11 ./target/release/holonic_sdl2
```

**Performance:**
- Target: 60 FPS
- Actual: 4,000-5,000+ FPS
- Frame time: 0.20-0.25ms
- GPU: Tested with NVIDIA RTX 2060 SUPER

**Documentation:**
- See [SDL2_SETUP.md](SDL2_SETUP.md) for detailed setup instructions
- See [SDL2_MIGRATION_GUIDE.md](SDL2_MIGRATION_GUIDE.md) for migration from Winit
- See [SDL2_INTEGRATION_ROADMAP.md](SDL2_INTEGRATION_ROADMAP.md) for implementation details

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- 4GB RAM minimum (8GB recommended)
- For SDL2 GUI:
  - Linux: `libsdl2-dev`, `libsdl2-mixer-dev` (optional)
  - Windows: SDL2 development libraries
  - macOS: `sdl2` via Homebrew

### SDL2 GUI Requirements

The SDL2 GUI requires additional dependencies:

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install libsdl2-dev libsdl2-mixer-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install SDL2-devel SDL2_mixer-devel
```

**Linux (Arch):**
```bash
sudo pacman -S sdl2 sdl2_mixer
```

**macOS:**
```bash
brew install sdl2 sdl2_mixer
```

**Windows:**
Download SDL2 development libraries from https://www.libsdl.org/

### Known Issues: Wayland Support

**Issue:** WGPU surface configuration error on Wayland + Vulkan backend
```
wp_linux_drm_syncobj_manager_v1#94: error 0: surface already exists
```

**Workaround:** Force X11 backend with environment variable:
```bash
SDL_VIDEO_DRIVER=x11 ./target/release/holonic_sdl2
```

**Status:** WGPU 0.20.1 backend issue, documented for future resolution. X11 backend works perfectly with excellent performance (4000+ FPS).

**Performance:** The SDL2 GUI achieves 67-83x the target performance:
- Target: 60 FPS
- Actual: 4,000-5,000+ FPS
- Frame time: 0.20-0.25ms

### Installation

**CLI Simulation (No GUI):**
```bash
# Navigate to project directory
cd /home/ishanp/Documents/Knowledge-Base/0_DOMAINS/1_Source/2_Simulation

# Build the project
cargo build --release

# Run the simulation
cargo run --release --bin holonic_realms
```

**SDL2 GUI (Recommended):**
```bash
# Install SDL2 dependencies (see Prerequisites above)

# Build with SDL2 feature
cargo build --release --features sdl2 --bin holonic_sdl2

# Run with X11 backend (recommended for Wayland systems)
SDL_VIDEO_DRIVER=x11 ./target/release/holonic_sdl2

# Or run normally (may fail on Wayland systems)
./target/release/holonic_sdl2
```

### Basic Usage

**CLI Simulation:**
```bash
# Default (128 entities, 100 steps)
cargo run --release --bin holonic_realms

# Custom parameters
cargo run --release --bin holonic_realms -- --entities 256 --steps 1000

# Quick test
cargo run --release --bin holonic_realms -- --entities 32 --steps 50
```

**SDL2 GUI:**
```bash
# Run with X11 backend (recommended for Wayland systems)
SDL_VIDEO_DRIVER=x11 ./target/release/holonic_sdl2

# Run normally (may fail on Wayland systems)
./target/release/holonic_sdl2
```

**SDL2 GUI Controls:**
- WASD/Arrows: Pan camera
- +/-: Zoom in/out
- 1: Toggle Entity Visualization
- 2: Toggle Spectrum Visualization
- 3: Toggle Emergence Visualization
- 4: Toggle Collective Visualization
- P: Toggle Performance Display
- F11: Toggle Fullscreen
- ESC: Exit
- Shift+C: Copy entity info to clipboard
- Shift+V: Paste from clipboard

## 📊 Simulation Output

The simulation produces comprehensive visualizations including:

### 1. Density Distribution
Shows distribution across 8 densities (1st through 8th)

### 2. Spectrum Statistics
Shows space/time vs time/space distribution

### 3. Spectrum Ratio Histogram
Detailed continuum from deep time/space to deep space/time

### 4. Entity Type Distribution
Environmental, Solar Logos, Galactic Logos, Individual

### 5. Physical Structure Visualization
Quantum, atomic, molecular, planetary, cellular, life forms

### 6. Collective Dynamics
Resonance, collective consciousness, group behavior

### 7. Polarization Distribution
STO (Service-to-Others), STS (Service-to-Self), Unpolarized

## 📚 Documentation

### Core Documentation

- **[COSMOLOGICAL-ARCHITECTURE.md](COSMOLOGICAL-ARCHITECTURE.md)**: Architecture specification
- **[COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md](COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md)**: 8-phase refactor plan
- **[PHASE_8_VALIDATION_REPORT.md](PHASE_8_VALIDATION_REPORT.md)**: Validation report
- **[USER_GUIDE.md](USER_GUIDE.md)**: User guide with examples and troubleshooting

### Additional Documentation

- **[docs/](docs/)**: Additional documentation and technical notes
- **[tests/](tests/)**: Comprehensive test suite

## 🧪 Testing

```bash
# Run all tests
cargo test --release

# Run tests with output
cargo test --release -- --nocapture

# Run specific test
cargo test --release test_entity_has_evolution_clock
```

### Test Coverage

- **Phase 1 Tests**: 4 tests (True Individualization)
- **Phase 2 Tests**: 3 tests (Spectrum Visualization)
- **Phase 3 Tests**: 3 tests (Physical Manifestation)
- **Phase 4 Tests**: 3 tests (Logos Hierarchy)
- **Phase 5 Tests**: 3 tests (Organic Evolution)
- **Phase 6 Tests**: 3 tests (Holographic Coherence)
- **Phase 7 Tests**: 3 tests (Spectrum Differentiation)
- **Phase 8 Tests**: 4 tests (Integration)

**Total**: 26 tests

### SDL2 GUI Testing

```bash
# Build SDL2 GUI
cargo build --release --features sdl2 --bin holonic_sdl2

# Run cross-platform test suite
./test_sdl2_cross_platform.sh

# Quick manual tests
SDL_VIDEO_DRIVER=x11 timeout 1 ./target/release/holonic_sdl2 2>&1 | grep "Window created"
SDL_VIDEO_DRIVER=x11 timeout 1 ./target/release/holonic_sdl2 2>&1 | grep "WGPU initialized"
SDL_VIDEO_DRIVER=x11 timeout 1 ./target/release/holonic_sdl2 2>&1 | grep "GPU adapter found"
```

**Test Results:**
- All 8 core tests: ✅ PASSED
- Window creation: ✅
- WGPU initialization: ✅
- GPU detection: ✅
- Surface configuration: ✅
- Entity system: ✅
- GUI startup: ✅
- Performance tracking: ✅

## 🏗️ Architecture

### Core Concepts

1. **Three Primal Distortions**: Free Will, Love/Logos, Light
2. **Space/Time and Time/Space Spectrum**: Unified continuum v = s/t ↔ v = t/s
3. **Density Octave**: 8 densities representing consciousness stages
4. **Logos Hierarchy**: Galactic → Solar → Individual
5. **Holographic Principle**: Each entity contains the whole
6. **Free Will**: Archetype 22 enables non-deterministic choice
7. **The Veil**: Structural feature at v=1 limiting Oneness access

### Project Structure

```
src/
├── simulation_v3/          # Main simulation system
│   ├── involution_sequence.rs
│   ├── entity_lifecycle.rs
│   ├── holographic_field.rs
│   ├── catalyst_system.rs
│   ├── collective_dynamics.rs
│   ├── environment.rs
│   ├── simulation_runner.rs
│   ├── statistics.rs
│   └── visualization.rs
├── entity_layer7/          # Layer 7: Individual entities
│   └── layer7.rs
├── evolution_density_octave/  # Density octave progression
│   ├── density_octave.rs
│   └── spectrum_access.rs
├── physical_manifestation/ # Physical reality generation
│   ├── structures.rs
│   └── hierarchy.rs
├── consciousness/          # Free Will, Archetype 22
├── memory/                 # Holographic memory, soul stream
├── holographic/            # Holographic reference systems
└── [266 total Rust files]
```

## 🎨 Visualizations

The simulation provides multiple visualizations:

### Spectrum Visualization
- Space/Time ↔ Time/Space continuum
- Entity positions on spectrum
- Veil indicator at v=1
- Activity dashboard

### Physical Structure Visualization
- Hierarchical composition (atoms → molecules → cells)
- Simultaneous emergence (atoms/galaxies, molecules/planets)
- Density and sub-level distribution

### Collective Dynamics Visualization
- Resonance heatmap
- Group consciousness metrics
- Collective behavior patterns

### Phase 7 Visualizations
- Detailed spectrum distribution
- Space/Time vs Time/Space breakdown
- Density progression over time

## 🔬 Key Metrics

### Success Criteria

| Criterion | Target | Current | Status |
|-----------|--------|---------|--------|
| Architecture Alignment | > 90% | 84.62% | ⚠️ Below Target |
| Entities Individuated | 100% | 100% | ✅ Met |
| Logos Hierarchy Manifested | Yes | Yes | ✅ Met |
| Spectrum Visualization | Complete | Complete | ✅ Met |
| Performance | <30s (128 entities) | 42.77s | ⚠️ Above Target |

### Note on Metrics

Many success criteria (polarization, collective consciousness, organic emergence) require longer simulation times (10,000+ steps) to manifest. Short simulations (100 steps) cannot show significant progress in these areas.

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Clean build
cargo clean && cargo build --release
```

### Running

```bash
# Debug run
cargo run --bin holonic_realms

# Release run (optimized)
cargo run --release --bin holonic_realms

# With parameters
cargo run --release --bin holonic_realms -- --entities 256 --steps 1000
```

### Testing

```bash
# All tests
cargo test

# Release tests
cargo test --release

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

## 🤝 Contributing

Contributions are welcome! Areas for contribution:

- Bug reports and fixes
- Performance optimizations
- New visualizations
- Documentation improvements
- Additional tests
- Feature requests

## 📖 User Guide

For detailed usage instructions, see [USER_GUIDE.md](USER_GUIDE.md):

- Getting started
- Running the simulation
- Understanding visualizations
- Simulation parameters
- Interpreting results
- Troubleshooting
- Advanced usage

## 📊 Phase-by-Phase Implementation

### Phase 1: Foundation - True Individualization ✅
- Parallel independent decision-making
- Entity evolution clocks
- Free Will-based choice mechanism

### Phase 2: Spectrum Visualization ✅
- Comprehensive spectrum visualization
- Entity position tracking
- Veil effect visualization

### Phase 3: Physical Manifestation System ✅
- Physical structures at each sub-level
- Hierarchical composition
- Simultaneous emergence

### Phase 4: Logos Hierarchy ✅
- Galactic-scale Logoi
- Solar-scale Logoi
- Hierarchical relationships

### Phase 5: Organic Evolution ✅
- Probabilistic transitions
- Attractor fields
- Non-linear development

### Phase 6: Holographic Coherence ✅
- Resonance mechanism
- Collective formation
- Coherence tracking

### Phase 7: Space/Time vs Time/Space Differentiation ✅
- Spectrum access tracking
- Veil transparency
- Spectrum position calculation

### Phase 8: Integration and Testing ✅
- All phases integrated
- Comprehensive testing
- Documentation complete

## 🔮 Future Work

- Interactive GUI visualization
- Save/load simulation state
- Real-time visualization
- Performance optimizations
- Additional cosmological scenarios
- User-configurable parameters

## 📄 License

This project is part of the Holographic Architecture project and follows the project's license.

## 🙏 Acknowledgments

Based on the Law of One material (Ra sessions), which describes the cosmological architecture of the universe through the lens of holographic principles, density octave evolution, and the primacy of free will.

## 📞 Support

For questions, issues, or suggestions:

1. Review the [USER_GUIDE.md](USER_GUIDE.md)
2. Check [PHASE_8_VALIDATION_REPORT.md](PHASE_8_VALIDATION_REPORT.md)
3. Review the test suite in [tests/](tests/)
4. Check existing documentation in [docs/](docs/)

---

**Version:** 9.0
**Last Updated:** February 4, 2026
**Status:** All 8 Phases Complete ✅
**Architecture Alignment:** 84.62%