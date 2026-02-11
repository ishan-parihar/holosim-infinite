# AGENTS.md - Guide for Agentic Coding Assistants

This guide helps coding assistants (AI agents, Cursor, Copilot, etc.) work effectively with this Rust codebase.

---

## Build, Lint, and Test Commands

### Building
```bash
cargo build                    # Debug build
cargo build --release          # Optimized build
cargo clean && cargo build --release  # Clean build
```

### Testing
```bash
cargo test --release                           # All tests
cargo test --release -- --nocapture            # With output
cargo test --release test_entity_has_evolution_clock  # Single test
cargo test --release phase1_tests              # Module tests
cargo test --no-run                            # Check syntax only
```

### Linting
```bash
cargo clippy --all-targets --all-features      # Run linter
cargo clippy --fix --allow-dirty               # Auto-fix
cargo fmt                                      # Format code
cargo fmt -- --check                           # Check formatting
```

### Running Simulation
```bash
cargo run --release --bin holonic_realms                    # Default (128 entities, 100 steps)
cargo run --release --bin holonic_realms -- --entities 256 --steps 1000  # Custom
```

---

## Code Style Guidelines

### Naming Conventions
- **Types**: `PascalCase` (`EntityLifecycleManager`, `EntityType`)
- **Functions/Methods**: `snake_case` (`create_entity`, `advance_entity`)
- **Variables/Fields**: `snake_case` (`entity_id`, `current_state`)
- **Constants**: `SCREAMING_SNAKE_CASE` (`MAX_ENTITIES`)
- **Files**: `snake_case` (`entity_lifecycle.rs`)

### Imports (ordered: std → external → internal)
```rust
use std::collections::HashMap;
use rand::Rng;
use crate::foundation::violet_realm::VioletRealm;
use crate::entity_layer7::layer7::{EntityId, EntityType};
```

### Documentation
```rust
//! Module docs with //!
/// Item docs with ///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Quote source"
pub struct Entity { pub entity_id: EntityId; }
```

### Type System
```rust
pub type Float = f64;
pub type EntityId = u64;
pub fn create_entity() -> Result<Entity, CreationError> { }
#[derive(Debug, Clone, PartialEq)] pub struct Entity { }
```

### Error Handling
```rust
#[derive(Debug, Clone, PartialEq)] pub enum InvolutionError { InvalidStage(String), }
pub fn run_simulation() -> Result<SimulationResult, SimulationError> {
    let entities = create_entities()?;
    Ok(process_entities(entities)?)
}
let entity = entities.get(&id).ok_or_else(|| EntityError::NotFound(id))?;
```

### Structs
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Entity { pub entity_id: EntityId, pub current_state: EntityState, }
impl Default for Entity { fn default() -> Self { Entity { entity_id: 0, current_state: EntityState::Initial } } }
```

### Functions
```rust
pub fn calculate_transition_probability(entity: &Entity) -> f64 { }
pub fn process_entity(entity: &Entity) -> Result<ProcessedEntity, Error> { }
let sum: f64 = entities.iter().map(|e| e.value).sum();
```

### Testing
```rust
#[cfg(test)] mod phase1_tests { use super::*;
    #[test] fn test_entity_has_evolution_clock() { assert!(entity.evolution_clock >= 0.0); }
}
```

---

## Project-Specific Guidelines

### Cosmological Architecture
- **Three Primal Distortions**: Free Will, Love/Logos, Light
- **"Transcend and Include"**: Each layer includes previous development
- **Space/Time ↔ Time/Space Spectrum**: Continuum with Veil at v=1
- **Density Octave**: 8 densities (1st → 8th) representing consciousness stages
- **Holographic Principle**: Each entity contains the whole

### Layer Organization
- `src/foundation/` - Layers 0-3 (Violet → Green): Three Primal Distortions
- `src/spectrum/` - Layers 4-6 (Yellow → Red): Spectrum emergence
- `src/entity_layer7/` - Layer 7: Individual entities
- `src/evolution_density_octave/` - Density octave progression
- `src/consciousness/` - Free Will and Archetype 22
- `src/simulation_v3/` - Main simulation system

### Common Patterns
```rust
// Entity creation
let entity = SubSubLogos::builder().with_entity_id(EntityId::new(id)).with_entity_type(EntityType::Individual).build()?;
// Evolution step
lifecycle.advance_entity(&entity_id, 1, &free_will_kernel)?;
// Spectrum access
let access = EntitySpectrumAccess { space_time_ratio: 1.5, time_space_ratio: 0.67, spectrum_position: 0.6, veil_transparency: 0.0 };
```

### Common Pitfalls
1. **Don't mutate spectrum configuration** - Original must be preserved
2. **Don't use deterministic transitions** - Use probabilistic evolution (Phase 5)
3. **Don't create collectives by proximity** - Use resonance (Phase 6)
4. **Don't ignore the Veil** - It's a structural feature at v=1
5. **Don't assume linear evolution** - Entities can "leap" to higher densities

---

## Key Files

**Documentation**: `README.md`, `USER_GUIDE.md`, `COSMOLOGICAL-ARCHITECTURE.md`, `COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md`

**Source Code**: `src/simulation_v3/simulation_runner.rs`, `src/entity_layer7/layer7.rs`, `src/simulation_v3/entity_lifecycle.rs`, `tests/comprehensive_test_suite.rs`

**Architecture References**: Always cite source documents in code comments: `/// From COSMOLOGICAL-ARCHITECTURE.md: "The spectrum is configured at galactic and solar scales before physical matter exists."`