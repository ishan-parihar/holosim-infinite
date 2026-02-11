# Reference Manual

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [Core Types](#core-types)
2. [Holographic Module](#holographic-module)
3. [Physics Module](#physics-module)
4. [Validation Module](#validation-module)
5. [Exploration Module](#exploration-module)
6. [Entity Module](#entity-module)
7. [Soul Stream Module](#soul-stream-module)
8. [Veil Module](#veil-module)
9. [Coordinate System Module](#coordinate-system-module)
10. [Multi-Scale Module](#multi-scale-module)
11. [Light Module](#light-module)
12. [Physical Dimension Module](#physical-dimension-module)
13. [Transformation Engine Module](#transformation-engine-module)
14. [Scale Architecture Module](#scale-architecture-module)
15. [Fractal-Holographic Structure Module](#fractal-holographic-structure-module)
16. [Decision Engine Module](#decision-engine-module)
17. [Enhanced Veil Module](#enhanced-veil-module)
18. [Organic Reality Generator Module](#organic-reality-generator-module)
19. [Dual-Dimensional Integration Module](#dual-dimensional-integration-module)
20. [Complete Simulation Module](#complete-simulation-module)

---

## Core Types

### Float

```rust
pub type Float = f64;
```

Floating-point type used throughout the simulator.

### Position

```rust
pub struct Position {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
```

3D spatial position.

**Methods**:
- `new(x: Float, y: Float, z: Float) -> Self`
- `origin() -> Self`
- `distance_to(&self, other: &Position) -> Float`
- `norm(&self) -> Float`
- `spatial_frequency(&self) -> Float`

### ComplexVector

```rust
pub struct ComplexVector {
    pub amplitude: Float,
    pub phase: Float,
}
```

Complex vector with amplitude and phase.

**Methods**:
- `new(amplitude: Float, phase: Float) -> Self`
- `zero() -> Self`
- `add(&self, other: &ComplexVector) -> ComplexVector`
- `subtract(&self, other: &ComplexVector) -> ComplexVector`
- `multiply(&self, other: &ComplexVector) -> ComplexVector`
- `scale(&self, scalar: Float) -> ComplexVector`
- `dot_product(&self, other: &ComplexVector) -> Float`
- `norm(&self) -> Float`
- `normalize(&self) -> ComplexVector`
- `conjugate(&self) -> ComplexVector`

### ComplexArchetype

```rust
pub struct ComplexArchetype {
    pub amplitude: Float,
    pub phase: Float,
}
```

Archetype represented as complex vector.

**Methods**:
- `new(amplitude: Float, phase: Float) -> Self`
- `from_archetype(archetype: &Archetype22) -> Self`
- `to_vector(&self) -> ComplexVector`

---

## Holographic Module

### InvolutionLayer

```rust
pub enum InvolutionLayer {
    Violet,
    Indigo,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
}
```

7 layers of involution.

**Methods**:
- `spatial_frequency(&self) -> Float`
- `aperture_size(&self) -> Float`
- `ev_wavelength(&self) -> Option<Float>`
- `ev_frequency(&self) -> Option<Float>`

### HolographicField

```rust
pub struct HolographicField {
    pub aperture_size: Float,
    pub spatial_frequency: Float,
    pub layer: InvolutionLayer,
}
```

Holographic field with interference patterns.

**Methods**:
- `new(aperture_size: Float, spatial_frequency: Float, layer: InvolutionLayer) -> Self`
- `with_default_aperture(layer: InvolutionLayer) -> Self`
- `with_default_spatial_frequency(layer: InvolutionLayer) -> Self`
- `set_aperture_size(&mut self, size: Float)`
- `set_spatial_frequency(&mut self, frequency: Float)`
- `get_resolution(&self) -> Float`
- `get_layer_info(&self) -> LayerInfo`

### HolographicEntity

```rust
pub struct HolographicEntity {
    pub id: String,
    pub archetype_encoding: [ComplexArchetype; 22],
    pub position: Position,
    pub layer: InvolutionLayer,
}
```

Entity with holographic encoding.

**Methods**:
- `new(id: String, archetype_encoding: [ComplexArchetype; 22]) -> Self`
- `with_position(&mut self, position: Position)`
- `with_layer(&mut self, layer: InvolutionLayer)`
- `get_mind_view(&self) -> MindView`
- `get_body_view(&self) -> BodyView`
- `get_spirit_view(&self) -> SpiritView`
- `calculate_resonance(&self, other: &HolographicEntity) -> Float`

### InterferencePattern

```rust
pub struct InterferencePattern {
    pub positions: Vec<Position>,
    pub intensities: Vec<Float>,
    pub phases: Vec<Float>,
}
```

Interference pattern from archetype interactions.

**Methods**:
- `new(archetypes: &[ComplexVector], positions: Vec<Position>) -> Self`
- `calculate_intensity(&self, position: &Position) -> Float`
- `find_constructive_nodes(&self, threshold: Float) -> Vec<Position>`
- `find_destructive_nodes(&self, threshold: Float) -> Vec<Position>`

### OscillatorNetwork

```rust
pub struct OscillatorNetwork {
    pub oscillators: Vec<ArchetypeOscillator>,
    pub coupling_strength: Float,
}
```

Oscillator network for consciousness modeling.

**Methods**:
- `new(coupling_strength: Float) -> Self`
- `add_oscillator(&mut self, oscillator: ArchetypeOscillator)`
- `synchronize(&mut self, time_step: Float) -> SynchronizationState`
- `calculate_phase_coherence(&self) -> Float`
- `get_synchronization_state(&self) -> SynchronizationState`

### HolographicMemory

```rust
pub struct HolographicMemory {
    pub experiences: Vec<Experience>,
    pub dimension: usize,
}
```

Holographic memory for Soul Stream.

**Methods**:
- `new(dimension: usize) -> Self`
- `store_experience(&mut self, experience: Experience)`
- `retrieve_similar(&self, query: &Hypervector, k: usize) -> Vec<Experience>`
- `associate(&mut self, hypervectors: Vec<Hypervector>) -> Hypervector`

### ConfigurationDiscoveryEngine

```rust
pub struct ConfigurationDiscoveryEngine {
    pub field: HolographicField,
    pub resonance_threshold: Float,
}
```

Discovers stable configurations.

**Methods**:
- `new(field: HolographicField, resonance_threshold: Float) -> Self`
- `discover_configurations(&self, archetypes: &[ComplexArchetype]) -> Vec<StableConfiguration>`
- `classify_configurations(&self, configs: &[StableConfiguration]) -> ConfigurationClassification`
- `calculate_stability(&self, config: &StableConfiguration) -> Float`

### LightCondensationEngine

```rust
pub struct LightCondensationEngine {
    pub field: HolographicField,
    pub condensation_threshold: Float,
}
```

Condenses light into matter.

**Methods**:
- `new(field: HolographicField, condensation_threshold: Float) -> Self`
- `condense_light(&self, photon: &Photon) -> CondensationResult`
- `calculate_condensation_probability(&self, photon: &Photon) -> Float`

---

## Physics Module

### PhysicsMode

```rust
pub enum PhysicsMode {
    Hardcoded,
    Holographic,
}
```

Physics mode enumeration.

### DualPhysicsSystem

```rust
pub struct DualPhysicsSystem {
    pub mode: PhysicsMode,
    pub hardcoded_engine: HardcodedPhysicsEngine,
    pub holographic_engine: HolographicPhysicsEngine,
}
```

Dual-mode physics system.

**Methods**:
- `new() -> Self`
- `set_mode(&mut self, mode: PhysicsMode)`
- `get_mode(&self) -> PhysicsMode`
- `calculate_force(&self, particle1: &Particle, particle2: &Particle) -> Float`
- `calculate_energy(&self, particle: &Particle) -> Float`
- `compare_modes(&self) -> ComparisonReport`

### HardcodedPhysicsEngine

```rust
pub struct HardcodedPhysicsEngine {
    pub gravitational_constant: Float,
    pub speed_of_light: Float,
    pub planck_constant: Float,
}
```

Hardcoded physics engine.

**Methods**:
- `new() -> Self`
- `calculate_gravitational_force(&self, m1: Float, m2: Float, distance: Float) -> Float`
- `calculate_electromagnetic_force(&self, q1: Float, q2: Float, distance: Float) -> Float`

### HolographicPhysicsEngine

```rust
pub struct HolographicPhysicsEngine {
    pub field: HolographicField,
}
```

Holographic physics engine.

**Methods**:
- `new(field: HolographicField) -> Self`
- `calculate_force(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> Float`
- `calculate_resonance(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> Float`

### ParticleProperties

```rust
pub struct ParticleProperties {
    pub mass: Float,
    pub charge: Float,
    pub spin: Float,
    pub lifetime: Option<Float>,
}
```

Properties of a particle.

---

## Validation Module

### ValidationSuite

```rust
pub struct ValidationSuite {
    pub architecture_tests: ArchitectureTests,
    pub physics_recovery_tests: PhysicsRecoveryTests,
    pub holographic_tests: HolographicTests,
}
```

Comprehensive validation suite.

**Methods**:
- `new() -> Self`
- `run_all_tests(&self) -> ValidationReport`
- `run_architecture_tests(&self) -> ArchitectureValidationResult`
- `run_physics_recovery_tests(&self) -> PhysicsRecoveryResult`
- `run_holographic_tests(&self) -> HolographicValidationResult`

### ArchitectureTests

```rust
pub struct ArchitectureTests {
    pub holographic_completeness_threshold: Float,
    pub phase_coherence_threshold: Float,
}
```

Architecture validation tests.

**Methods**:
- `new() -> Self`
- `test_holographic_completeness(&self, entity: &HolographicEntity) -> bool`
- `test_phase_coherence(&self, network: &OscillatorNetwork) -> bool`
- `test_spatial_frequency_reduction(&self, layer1: InvolutionLayer, layer2: InvolutionLayer) -> bool`

### PhysicsRecoveryTests

```rust
pub struct PhysicsRecoveryTests {
    pub accuracy_threshold: Float,
}
```

Physics recovery validation tests.

**Methods**:
- `new() -> Self`
- `test_mass_recovery(&self, entity: &HolographicEntity, expected_mass: Float) -> bool`
- `test_charge_recovery(&self, entity: &HolographicEntity, expected_charge: Float) -> bool`
- `test_spin_recovery(&self, entity: &HolographicEntity, expected_spin: Float) -> bool`

### HolographicTests

```rust
pub struct HolographicTests {
    pub resonance_threshold: Float,
}
```

Holographic validation tests.

**Methods**:
- `new() -> Self`
- `test_resonance_calculation(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> bool`
- `test_interference_pattern(&self, pattern: &InterferencePattern) -> bool`
- `test_configuration_discovery(&self, engine: &ConfigurationDiscoveryEngine) -> bool`

---

## Exploration Module

### UniverseExplorer

```rust
pub struct UniverseExplorer {
    pub base_architecture: LayerArchitecture,
    pub discovery_database: DiscoveryDatabase,
}
```

Explores variant holographic architectures.

**Methods**:
- `new(base_architecture: LayerArchitecture) -> Self`
- `with_default_architecture() -> Self`
- `explore_variant(&mut self, variant: ArchitectureVariant) -> ExplorationResult`
- `explore_multiple_variants(&mut self, variants: Vec<ArchitectureVariant>) -> Vec<ExplorationResult>`
- `compare_variants(&self, results: &[ExplorationResult]) -> VariantComparison`
- `get_discovery_database(&self) -> &DiscoveryDatabase`
- `get_statistics(&self) -> ExplorationStatistics`

### ArchitectureVariant

```rust
pub enum ArchitectureVariant {
    DifferentPhysicalLaws(PhysicalLawsVariant),
    DifferentPlanetaryBias(PlanetaryBiasVariant),
    DifferentSoulStream(SoulStreamVariant),
    DifferentSpatialFrequency(SpatialFrequencyVariant),
}
```

Variant architecture types.

### HypothesisGenerator

```rust
pub struct HypothesisGenerator {
    pub pattern_recognizer: PatternRecognizer,
    pub hypothesis_database: HypothesisDatabase,
}
```

Generates hypotheses from discoveries.

**Methods**:
- `new() -> Self`
- `generate_hypotheses(&self, discoveries: &[DiscoveryRecord]) -> Vec<Hypothesis>`
- `generate_testable_predictions(&self, hypotheses: &[Hypothesis]) -> Vec<TestablePrediction>`
- `recognize_patterns(&self, discoveries: &[DiscoveryRecord]) -> Vec<Pattern>`

### DiscoveryDatabase

```rust
pub struct DiscoveryDatabase {
    pub discoveries: Vec<DiscoveryRecord>,
}
```

Database for storing discoveries.

**Methods**:
- `new() -> Self`
- `add_discovery(&mut self, discovery: DiscoveryRecord)`
- `query(&self, query: DiscoveryQuery) -> Vec<DiscoveryRecord>`
- `get_statistics(&self) -> DatabaseStatistics`

---

## See Also

- [API Documentation](API.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Tutorial](TUTORIAL.md)
- [FAQ](FAQ.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Usage Examples](EXAMPLES.md)
- [Architecture Diagrams](ARCHITECTURE.md)

---

**Reference Manual Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator