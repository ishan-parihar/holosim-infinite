# API Documentation

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [Overview](#overview)
2. [Holographic Architecture API](#holographic-architecture-api)
3. [Physics System API](#physics-system-api)
4. [Validation Framework API](#validation-framework-api)
5. [Exploration System API](#exploration-system-api)
6. [Entity System API](#entity-system-api)
7. [Soul Stream API](#soul-stream-api)
8. [Veil System API](#veil-system-api)
9. [Coordinate System API](#coordinate-system-api)
10. [Multi-Scale System API](#multi-scale-system-api)
11. [Light Architecture API](#light-architecture-api)
12. [Physical Dimension API](#physical-dimension-api)
13. [Transformation Engine API](#transformation-engine-api)
14. [Scale Architecture API](#scale-architecture-api)
15. [Fractal-Holographic Structure API](#fractal-holographic-structure-api)
16. [Decision Engine API](#decision-engine-api)
17. [Enhanced Veil API](#enhanced-veil-api)
18. [Organic Reality Generator API](#organic-reality-generator-api)
19. [Dual-Dimensional Integration API](#dual-dimensional-integration-api)
20. [Complete Simulation API](#complete-simulation-api)

---

## Overview

The Holographic Multi-Scale Cosmic Creation Simulator is a comprehensive simulation framework that models the holographic architecture of reality based on the Law of One principles. The simulation is organized into multiple modules, each representing different aspects of the cosmic creation process.

### Core Principles

1. **Holographic Completeness**: Each entity contains the complete encoding of the entire universe
2. **Spatial Frequency**: Resolution decreases from Violet to Red layers
3. **Resonance-Based Discovery**: Stable configurations emerge at constructive interference nodes
4. **Oscillatory Synchronization**: Consciousness is modeled as oscillator synchronization
5. **Phase Coherence**: Unity is achieved through phase coherence of oscillators

### Module Organization

The simulation is organized into 7 layers of involution:
- **Violet-Ray Realm**: Intelligent Infinity + Free Will
- **Indigo-Ray Realm**: Archetypical Mind + Wisdom
- **Blue-Ray Realm**: Holographic Light Architecture
- **Green-Ray Realm**: The Veil + Multi-Scale Systems
- **Yellow-Ray Realm**: Environmental Constraints + Physics
- **Orange-Ray Realm**: Individual Identity + Body
- **Red-Ray Realm**: Physical Manifestation + Matter

---

## Holographic Architecture API

### Module: `holographic`

The holographic module implements the core holographic architecture principles.

#### Core Types

##### `ComplexVector`

Represents a complex vector with amplitude and phase components.

```rust
pub struct ComplexVector {
    pub amplitude: Float,
    pub phase: Float,
}
```

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

##### `ComplexArchetype`

Represents an archetype as a complex vector with holographic encoding.

```rust
pub struct ComplexArchetype {
    pub amplitude: Float,
    pub phase: Float,
}
```

**Methods**:
- `new(amplitude: Float, phase: Float) -> Self`
- `from_archetype(archetype: &Archetype22) -> Self`
- `to_vector(&self) -> ComplexVector`

##### `HolographicField`

Represents a holographic field with interference patterns.

```rust
pub struct HolographicField {
    pub aperture_size: Float,
    pub spatial_frequency: Float,
    pub layer: InvolutionLayer,
}
```

**Methods**:
- `new(aperture_size: Float, spatial_frequency: Float, layer: InvolutionLayer) -> Self`
- `with_default_aperture(layer: InvolutionLayer) -> Self`
- `with_default_spatial_frequency(layer: InvolutionLayer) -> Self`
- `set_aperture_size(&mut self, size: Float)`
- `set_spatial_frequency(&mut self, frequency: Float)`
- `get_resolution(&self) -> Float`
- `get_layer_info(&self) -> LayerInfo`

##### `InvolutionLayer`

Represents the 7 layers of involution.

```rust
pub enum InvolutionLayer {
    Violet,   // Highest spatial frequency
    Indigo,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,      // Lowest spatial frequency
}
```

**Methods**:
- `spatial_frequency(&self) -> Float`
- `aperture_size(&self) -> Float`
- `ev_wavelength(&self) -> Option<Float>`
- `ev_frequency(&self) -> Option<Float>`

##### `InterferencePattern`

Represents interference patterns from archetype interactions.

```rust
pub struct InterferencePattern {
    pub positions: Vec<Position>,
    pub intensities: Vec<Float>,
    pub phases: Vec<Float>,
}
```

**Methods**:
- `new(archetypes: &[ComplexVector], positions: Vec<Position>) -> Self`
- `calculate_intensity(&self, position: &Position) -> Float`
- `find_constructive_nodes(&self, threshold: Float) -> Vec<Position>`
- `find_destructive_nodes(&self, threshold: Float) -> Vec<Position>`

##### `HolographicEntity`

Represents an entity with holographic encoding.

```rust
pub struct HolographicEntity {
    pub id: String,
    pub archetype_encoding: [ComplexArchetype; 22],
    pub position: Position,
    pub layer: InvolutionLayer,
}
```

**Methods**:
- `new(id: String, archetype_encoding: [ComplexArchetype; 22]) -> Self`
- `with_position(&mut self, position: Position)`
- `with_layer(&mut self, layer: InvolutionLayer)`
- `get_mind_view(&self) -> MindView`
- `get_body_view(&self) -> BodyView`
- `get_spirit_view(&self) -> SpiritView`
- `calculate_resonance(&self, other: &HolographicEntity) -> Float`

##### `OscillatorNetwork`

Represents oscillator network for consciousness modeling.

```rust
pub struct OscillatorNetwork {
    pub oscillators: Vec<ArchetypeOscillator>,
    pub coupling_strength: Float,
}
```

**Methods**:
- `new(coupling_strength: Float) -> Self`
- `add_oscillator(&mut self, oscillator: ArchetypeOscillator)`
- `synchronize(&mut self, time_step: Float) -> SynchronizationState`
- `calculate_phase_coherence(&self) -> Float`
- `get_synchronization_state(&self) -> SynchronizationState`

##### `HolographicMemory`

Represents holographic memory for Soul Stream.

```rust
pub struct HolographicMemory {
    pub experiences: Vec<Experience>,
    pub dimension: usize,
}
```

**Methods**:
- `new(dimension: usize) -> Self`
- `store_experience(&mut self, experience: Experience)`
- `retrieve_similar(&self, query: &Hypervector, k: usize) -> Vec<Experience>`
- `associate(&mut self, hypervectors: Vec<Hypervector>) -> Hypervector`

##### `ConfigurationDiscoveryEngine`

Discovers stable configurations from holographic encoding.

```rust
pub struct ConfigurationDiscoveryEngine {
    pub field: HolographicField,
    pub resonance_threshold: Float,
}
```

**Methods**:
- `new(field: HolographicField, resonance_threshold: Float) -> Self`
- `discover_configurations(&self, archetypes: &[ComplexArchetype]) -> Vec<StableConfiguration>`
- `classify_configurations(&self, configs: &[StableConfiguration]) -> ConfigurationClassification`
- `calculate_stability(&self, config: &StableConfiguration) -> Float`

##### `LightCondensationEngine`

Condenses light into matter through holographic encoding.

```rust
pub struct LightCondensationEngine {
    pub field: HolographicField,
    pub condensation_threshold: Float,
}
```

**Methods**:
- `new(field: HolographicField, condensation_threshold: Float) -> Self`
- `condense_light(&self, photon: &Photon) -> CondensationResult`
- `calculate_condensation_probability(&self, photon: &Photon) -> Float`

---

## Physics System API

### Module: `physics`

The physics module implements a dual-mode physics system.

#### Core Types

##### `DualPhysicsSystem`

Dual-mode physics system supporting both hardcoded and holographic physics.

```rust
pub struct DualPhysicsSystem {
    pub mode: PhysicsMode,
    pub hardcoded_engine: HardcodedPhysicsEngine,
    pub holographic_engine: HolographicPhysicsEngine,
}
```

**Methods**:
- `new() -> Self`
- `set_mode(&mut self, mode: PhysicsMode)`
- `get_mode(&self) -> PhysicsMode`
- `calculate_force(&self, particle1: &Particle, particle2: &Particle) -> Float`
- `calculate_energy(&self, particle: &Particle) -> Float`
- `compare_modes(&self) -> ComparisonReport`

##### `PhysicsMode`

Physics mode enumeration.

```rust
pub enum PhysicsMode {
    Hardcoded,
    Holographic,
}
```

##### `HardcodedPhysicsEngine`

Hardcoded physics engine (legacy).

```rust
pub struct HardcodedPhysicsEngine {
    pub gravitational_constant: Float,
    pub speed_of_light: Float,
    pub planck_constant: Float,
}
```

**Methods**:
- `new() -> Self`
- `calculate_gravitational_force(&self, m1: Float, m2: Float, distance: Float) -> Float`
- `calculate_electromagnetic_force(&self, q1: Float, q2: Float, distance: Float) -> Float`

##### `HolographicPhysicsEngine`

Holographic physics engine (emergent).

```rust
pub struct HolographicPhysicsEngine {
    pub field: HolographicField,
}
```

**Methods**:
- `new(field: HolographicField) -> Self`
- `calculate_force(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> Float`
- `calculate_resonance(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> Float`

##### `ParticleProperties`

Properties of a particle.

```rust
pub struct ParticleProperties {
    pub mass: Float,
    pub charge: Float,
    pub spin: Float,
    pub lifetime: Option<Float>,
}
```

---

## Validation Framework API

### Module: `validation`

The validation module provides comprehensive validation framework.

#### Core Types

##### `ValidationSuite`

Comprehensive validation suite.

```rust
pub struct ValidationSuite {
    pub architecture_tests: ArchitectureTests,
    pub physics_recovery_tests: PhysicsRecoveryTests,
    pub holographic_tests: HolographicTests,
}
```

**Methods**:
- `new() -> Self`
- `run_all_tests(&self) -> ValidationReport`
- `run_architecture_tests(&self) -> ArchitectureValidationResult`
- `run_physics_recovery_tests(&self) -> PhysicsRecoveryResult`
- `run_holographic_tests(&self) -> HolographicValidationResult`

##### `ArchitectureTests`

Architecture validation tests.

```rust
pub struct ArchitectureTests {
    pub holographic_completeness_threshold: Float,
    pub phase_coherence_threshold: Float,
}
```

**Methods**:
- `new() -> Self`
- `test_holographic_completeness(&self, entity: &HolographicEntity) -> bool`
- `test_phase_coherence(&self, network: &OscillatorNetwork) -> bool`
- `test_spatial_frequency_reduction(&self, layer1: InvolutionLayer, layer2: InvolutionLayer) -> bool`

##### `PhysicsRecoveryTests`

Physics recovery validation tests.

```rust
pub struct PhysicsRecoveryTests {
    pub accuracy_threshold: Float,
}
```

**Methods**:
- `new() -> Self`
- `test_mass_recovery(&self, entity: &HolographicEntity, expected_mass: Float) -> bool`
- `test_charge_recovery(&self, entity: &HolographicEntity, expected_charge: Float) -> bool`
- `test_spin_recovery(&self, entity: &HolographicEntity, expected_spin: Float) -> bool`

##### `HolographicTests`

Holographic validation tests.

```rust
pub struct HolographicTests {
    pub resonance_threshold: Float,
}
```

**Methods**:
- `new() -> Self`
- `test_resonance_calculation(&self, entity1: &HolographicEntity, entity2: &HolographicEntity) -> bool`
- `test_interference_pattern(&self, pattern: &InterferencePattern) -> bool`
- `test_configuration_discovery(&self, engine: &ConfigurationDiscoveryEngine) -> bool`

---

## Exploration System API

### Module: `exploration`

The exploration module provides universe exploration and hypothesis generation.

#### Core Types

##### `UniverseExplorer`

Explores variant holographic architectures.

```rust
pub struct UniverseExplorer {
    pub base_architecture: LayerArchitecture,
    pub discovery_database: DiscoveryDatabase,
}
```

**Methods**:
- `new(base_architecture: LayerArchitecture) -> Self`
- `with_default_architecture() -> Self`
- `explore_variant(&mut self, variant: ArchitectureVariant) -> ExplorationResult`
- `explore_multiple_variants(&mut self, variants: Vec<ArchitectureVariant>) -> Vec<ExplorationResult>`
- `compare_variants(&self, results: &[ExplorationResult]) -> VariantComparison`
- `get_discovery_database(&self) -> &DiscoveryDatabase`
- `get_statistics(&self) -> ExplorationStatistics`

##### `ArchitectureVariant`

Variant architecture types.

```rust
pub enum ArchitectureVariant {
    DifferentPhysicalLaws(PhysicalLawsVariant),
    DifferentPlanetaryBias(PlanetaryBiasVariant),
    DifferentSoulStream(SoulStreamVariant),
    DifferentSpatialFrequency(SpatialFrequencyVariant),
}
```

##### `PhysicalLawsVariant`

Physical laws variant.

```rust
pub struct PhysicalLawsVariant {
    pub gravity_modifier: Float,
    pub electromagnetic_force_modifier: Float,
    pub strong_force_modifier: Float,
    pub weak_force_modifier: Float,
    pub speed_of_light_modifier: Float,
    pub planck_constant_modifier: Float,
}
```

##### `PlanetaryBiasVariant`

Planetary bias variant.

```rust
pub struct PlanetaryBiasVariant {
    pub atmosphere_modifier: Float,
    pub em_field_modifier: Float,
    pub mineral_composition_modifier: Float,
    pub radiation_modifier: Float,
}
```

##### `SoulStreamVariant`

Soul stream variant.

```rust
pub struct SoulStreamVariant {
    pub karmic_pattern_modifier: Float,
    pub polarization_modifier: Float,
    pub evolutionary_goal_modifier: Float,
}
```

##### `SpatialFrequencyVariant`

Spatial frequency variant.

```rust
pub struct SpatialFrequencyVariant {
    pub violet_frequency_modifier: Float,
    pub indigo_frequency_modifier: Float,
    pub blue_frequency_modifier: Float,
    pub green_frequency_modifier: Float,
    pub yellow_frequency_modifier: Float,
    pub orange_frequency_modifier: Float,
    pub red_frequency_modifier: Float,
}
```

##### `ExplorationResult`

Result of exploration.

```rust
pub struct ExplorationResult {
    pub variant_description: String,
    pub discovered_configurations: Vec<StableConfiguration>,
    pub novelty_score: Float,
    pub stability_score: Float,
    pub physics_discovered: Option<PhysicsLaws>,
}
```

##### `HypothesisGenerator`

Generates hypotheses from discoveries.

```rust
pub struct HypothesisGenerator {
    pub pattern_recognizer: PatternRecognizer,
    pub hypothesis_database: HypothesisDatabase,
}
```

**Methods**:
- `new() -> Self`
- `generate_hypotheses(&self, discoveries: &[DiscoveryRecord]) -> Vec<Hypothesis>`
- `generate_testable_predictions(&self, hypotheses: &[Hypothesis]) -> Vec<TestablePrediction>`
- `recognize_patterns(&self, discoveries: &[DiscoveryRecord]) -> Vec<Pattern>`

##### `PatternRecognizer`

Recognizes patterns in discoveries.

```rust
pub struct PatternRecognizer {
    pub confidence_threshold: Float,
}
```

**Methods**:
- `new() -> Self`
- `recognize_mass_pattern(&self, discoveries: &[DiscoveryRecord]) -> Option<Pattern>`
- `recognize_charge_pattern(&self, discoveries: &[DiscoveryRecord]) -> Option<Pattern>`
- `recognize_spin_pattern(&self, discoveries: &[DiscoveryRecord]) -> Option<Pattern>`
- `recognize_resonance_pattern(&self, discoveries: &[DiscoveryRecord]) -> Option<Pattern>`
- `recognize_stability_pattern(&self, discoveries: &[DiscoveryRecord]) -> Option<Pattern>`

##### `DiscoveryDatabase`

Database for storing discoveries.

```rust
pub struct DiscoveryDatabase {
    pub discoveries: Vec<DiscoveryRecord>,
}
```

**Methods**:
- `new() -> Self`
- `add_discovery(&mut self, discovery: DiscoveryRecord)`
- `query(&self, query: DiscoveryQuery) -> Vec<DiscoveryRecord>`
- `get_statistics(&self) -> DatabaseStatistics`

##### `DiscoveryQuery`

Query for discovery database.

```rust
pub struct DiscoveryQuery {
    pub min_novelty_score: Option<Float>,
    pub min_stability_score: Option<Float>,
    pub layer_filter: Option<InvolutionLayer>,
    pub variant_type_filter: Option<String>,
}
```

**Methods**:
- `new() -> Self`
- `with_min_novelty_score(mut self, score: Float) -> Self`
- `with_min_stability_score(mut self, score: Float) -> Self`
- `with_layer_filter(mut self, layer: InvolutionLayer) -> Self`
- `with_variant_type_filter(mut self, variant_type: String) -> Self`

---

## Entity System API

### Module: `entity`

The entity module provides entity modeling.

#### Core Types

##### `Entity`

Represents an entity in the simulation.

```rust
pub struct Entity {
    pub id: String,
    pub archetype_state: ArchetypeState,
    pub vibrational_state: VibrationalState,
}
```

**Methods**:
- `new(id: String) -> Self`
- `make_choice(&mut self, context: ChoiceContext) -> Choice`
- `update_state(&mut self, catalyst: Catalyst) -> TransitionResult`

##### `ChoiceContext`

Context for entity choices.

```rust
pub struct ChoiceContext {
    pub catalyst: Catalyst,
    pub environment: Environment,
    pub polarization: Polarization,
}
```

##### `ChoiceModifier`

Modifiers for entity choices.

```rust
pub enum ChoiceModifier {
    FreeWillCapacity(Float),
    Harvestability(Harvestability),
    GrowthPrinciple(GrowthPrinciple),
}
```

---

## Soul Stream API

### Module: `soul_stream`

The soul stream module provides soul stream modeling.

#### Core Types

##### `SoulStream`

Represents the soul stream.

```rust
pub struct SoulStream {
    pub karmic_pattern: KarmicPattern,
    pub polarization_state: PolarizationState,
    pub evolutionary_goal: EvolutionaryGoal,
}
```

**Methods**:
- `new() -> Self`
- `process_experience(&mut self, experience: Experience) -> KarmaResult`
- `make_choice(&self, context: ChoiceContext) -> SoulChoice`

##### `KarmicPattern`

Karmic pattern.

```rust
pub struct KarmicPattern {
    pub distortions: Vec<Distortion>,
}
```

##### `PolarizationState`

Polarization state.

```rust
pub enum PolarizationState {
    ServiceToOthers(Float),
    ServiceToSelf(Float),
}
```

##### `EvolutionaryGoal`

Evolutionary goal.

```rust
pub struct EvolutionaryGoal {
    pub goal: String,
    pub progress: GoalProgress,
}
```

---

## Veil System API

### Module: `veil`

The veil module provides the veil mechanism.

#### Core Types

##### `VeilMechanism`

The veil mechanism.

```rust
pub struct VeilMechanism {
    pub state: VeilState,
    pub thin_spots: Vec<ThinSpot>,
}
```

**Methods**:
- `new() -> Self`
- `pierce(&mut self, event: PiercingEvent) -> PiercingResult`
- `get_transparency(&self, density: DensityTransparency) -> Float`
- `get_statistics(&self) -> VeilMechanismStatistics`

##### `VeilState`

Veil state.

```rust
pub struct VeilState {
    pub polarization_access: PolarizationAccess,
    pub veil_thickness: Float,
}
```

##### `PiercingEvent`

Piercing event.

```rust
pub struct PiercingEvent {
    pub event_type: PiercingEventType,
    pub location: PiercingLocation,
    pub intensity: Float,
}
```

---

## Coordinate System API

### Module: `coordinates`

The coordinates module provides coordinate systems.

#### Core Types

##### `MetaphysicalTimeSpaceCoord`

Metaphysical time-space coordinate.

```rust
pub struct MetaphysicalTimeSpaceCoord {
    pub density: Float,
    pub polarization: Float,
}
```

##### `PhysicalSpaceTimeCoord`

Physical space-time coordinate.

```rust
pub struct PhysicalSpaceTimeCoord {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub t: Float,
}
```

##### `DualRealmNavigator`

Navigator for dual realms.

```rust
pub struct DualRealmNavigator {
    pub current_realm: RealmTranslation,
}
```

**Methods**:
- `new() -> Self`
- `navigate_to(&mut self, realm: RealmTranslation) -> NavigationResult`

---

## Multi-Scale System API

### Module: `multi_scale`

The multi-scale module provides multi-scale modeling.

#### Core Types

##### `MultiScaleHierarchy`

Multi-scale hierarchy.

```rust
pub struct MultiScaleHierarchy {
    pub scales: Vec<Scale>,
}
```

**Methods**:
- `new() -> Self`
- `add_scale(&mut self, scale: Scale)`
- `get_scale(&self, scale_id: ScaleID) -> Option<&Scale>`

##### `MultiScaleAtom`

Multi-scale atom.

```rust
pub struct MultiScaleAtom {
    pub nucleus: Vec<QuantumParticle>,
    pub electrons: Vec<Electron>,
}
```

---

## Light Architecture API

### Module: `light`

The light module provides light architecture modeling.

#### Core Types

##### `Photon`

Represents a photon.

```rust
pub struct Photon {
    pub id: PhotonID,
    pub position: PhotonPosition,
    pub state: PhotonState,
}
```

##### `LightArchitecture`

Light architecture.

```rust
pub struct LightArchitecture {
    pub encoding: LightHolographicEncoding,
}
```

---

## Physical Dimension API

### Module: `physical_dimension`

The physical dimension module provides physical dimension modeling.

#### Core Types

##### `PhysicalDimension`

Physical dimension.

```rust
pub struct PhysicalDimension {
    pub space: Space,
    pub time: Time,
}
```

##### `PhysicalExperience`

Physical experience.

```rust
pub struct PhysicalExperience {
    pub experience_type: PhysicalExperienceType,
    pub outcome: PhysicalOutcome,
}
```

---

## Transformation Engine API

### Module: `transformation_engine`

The transformation engine module provides transformation engine.

#### Core Types

##### `TransformationEngine`

Transformation engine.

```rust
pub struct TransformationEngine {
    pub state: TransformationState,
}
```

**Methods**:
- `new() -> Self`
- `transform(&mut self, input: IntelligentEnergy) -> TransformationResult`

##### `IntelligentEnergy`

Intelligent energy.

```rust
pub struct IntelligentEnergy {
    pub energy_type: ConsciousnessType,
    pub amplitude: Float,
}
```

---

## Scale Architecture API

### Module: `scale_architecture`

The scale architecture module provides scale architecture modeling.

#### Core Types

##### `ScaleHierarchy`

Scale hierarchy.

```rust
pub struct ScaleHierarchy {
    pub scales: Vec<Scale>,
}
```

##### `Scale`

Scale.

```rust
pub struct Scale {
    pub scale_id: ScaleID,
    pub scale_type: ScaleType,
}
```

---

## Fractal-Holographic Structure API

### Module: `fractal_holographic_structure`

The fractal-holographic structure module provides fractal-holographic modeling.

#### Core Types

##### `HolographicContainer`

Holographic container.

```rust
pub struct HolographicContainer {
    pub sub_density_states: Vec<SubDensityState>,
}
```

##### `FractalConnection`

Fractal connection.

```rust
pub struct FractalConnection {
    pub connection_type: ConnectionType,
    pub flow_direction: HolographicFlowDirection,
}
```

---

## Decision Engine API

### Module: `decision_engine`

The decision engine module provides decision engine.

#### Core Types

##### `DecisionEngine`

Decision engine.

```rust
pub struct DecisionEngine {
    pub entity_id: EntityID,
}
```

**Methods**:
- `new(entity_id: EntityID) -> Self`
- `make_decision(&self, context: DecisionContext) -> Decision`

##### `Choice`

Choice.

```rust
pub struct Choice {
    pub decision: String,
    pub explanation: DecisionExplanation,
}
```

---

## Enhanced Veil API

### Module: `enhanced_veil`

The enhanced veil module provides enhanced veil mechanism.

#### Core Types

##### `Veil`

Enhanced veil.

```rust
pub struct Veil {
    pub full_awareness: FullAwareness,
    pub limited_awareness: LimitedAwareness,
}
```

**Methods**:
- `new() -> Self`
- `get_illusion_of_separation(&self) -> SeparationIllusion`
- `get_statistics(&self) -> VeilStatistics`

---

## Organic Reality Generator API

### Module: `organic_reality_generator`

The organic reality generator module provides organic reality generation.

#### Core Types

##### `OrganicRealityGenerator`

Organic reality generator.

```rust
pub struct OrganicRealityGenerator {
    pub natural_laws_generator: NaturalLawsGenerator,
}
```

**Methods**:
- `new() -> Self`
- `generate_reality(&mut self) -> OrganicReality`
- `compare_realities(&self, reality1: &OrganicReality, reality2: &OrganicReality) -> RealityComparison`

---

## Dual-Dimensional Integration API

### Module: `dual_dimensional_integration`

The dual-dimensional integration module provides dual-dimensional integration.

#### Core Types

##### `DualDimensionalIntegration`

Dual-dimensional integration.

```rust
pub struct DualDimensionalIntegration {
    pub state: IntegrationState,
}
```

**Methods**:
- `new() -> Self`
- `integrate(&mut self) -> IntegrationResult`
- `validate(&self) -> ValidationResult`

---

## Complete Simulation API

### Module: `complete_simulation`

The complete simulation module provides complete simulation.

#### Core Types

##### `CompleteSimulation`

Complete simulation.

```rust
pub struct CompleteSimulation {
    pub entities: Vec<EntityEvolution>,
}
```

**Methods**:
- `new() -> Self`
- `run(&mut self) -> SimulationResult`
- `get_statistics(&self) -> SimulationStatistics`

##### `EntityEvolution`

Entity evolution.

```rust
pub struct EntityEvolution {
    pub steps: Vec<SimulationEvolutionStep>,
}
```

---

## Type Definitions

### Common Types

```rust
pub type Float = f64;
pub type Position = holographic::Position;
pub type ComplexVector = holographic::ComplexVector;
pub type ComplexArchetype = holographic::ComplexArchetype;
pub type HolographicField = holographic::HolographicField;
pub type InvolutionLayer = holographic::InvolutionLayer;
pub type HolographicEntity = holographic::HolographicEntity;
pub type InterferencePattern = holographic::InterferencePattern;
pub type OscillatorNetwork = holographic::OscillatorNetwork;
pub type HolographicMemory = holographic::HolographicMemory;
pub type ConfigurationDiscoveryEngine = holographic::ConfigurationDiscoveryEngine;
pub type LightCondensationEngine = holographic::LightCondensationEngine;
pub type DualPhysicsSystem = physics::DualPhysicsSystem;
pub type PhysicsMode = physics::PhysicsMode;
pub type HardcodedPhysicsEngine = physics::HardcodedPhysicsEngine;
pub type HolographicPhysicsEngine = physics::HolographicPhysicsEngine;
pub type ParticleProperties = physics::ParticleProperties;
pub type ValidationSuite = validation::ValidationSuite;
pub type ArchitectureTests = validation::ArchitectureTests;
pub type PhysicsRecoveryTests = validation::PhysicsRecoveryTests;
pub type HolographicTests = validation::HolographicTests;
pub type UniverseExplorer = exploration::UniverseExplorer;
pub type HypothesisGenerator = exploration::HypothesisGenerator;
pub type PatternRecognizer = exploration::PatternRecognizer;
pub type DiscoveryDatabase = exploration::DiscoveryDatabase;
pub type Entity = entity::Entity;
pub type SoulStream = soul_stream::SoulStream;
pub type VeilMechanism = veil::VeilMechanism;
pub type DualRealmNavigator = coordinates::DualRealmNavigator;
pub type MultiScaleHierarchy = multi_scale::MultiScaleHierarchy;
pub type Photon = light::Photon;
pub type TransformationEngine = transformation_engine::TransformationEngine;
pub type ScaleHierarchy = scale_architecture::ScaleHierarchy;
pub type HolographicContainer = fractal_holographic_structure::HolographicContainer;
pub type DecisionEngine = decision_engine::DecisionEngine;
pub type Veil = enhanced_veil::Veil;
pub type OrganicRealityGenerator = organic_reality_generator::OrganicRealityGenerator;
pub type DualDimensionalIntegration = dual_dimensional_integration::DualDimensionalIntegration;
pub type CompleteSimulation = complete_simulation::CompleteSimulation;
```

---

## Error Handling

### Common Errors

```rust
pub enum SimulationError {
    InvalidConfiguration(String),
    CalculationError(String),
    DatabaseError(String),
    ValidationError(String),
}
```

---

## Performance Considerations

### Optimization Tips

1. **Use appropriate spatial frequency** for your layer
2. **Cache interference patterns** for repeated calculations
3. **Use resonance threshold** to filter unstable configurations
4. **Batch operations** for multiple entities
5. **Use memory pools** for frequent allocations

---

## Best Practices

1. **Always initialize** holographic fields before use
2. **Set appropriate aperture sizes** for your use case
3. **Use spatial frequency** as primary mechanism (not eV values)
4. **Discover configurations** through resonance (not derivation)
5. **Maintain phase information** in complex vectors
6. **Use holographic completeness** for entity encoding
7. **Calculate phase coherence** for unity verification
8. **Test with validation suite** before deployment

---

## See Also

- [Getting Started Guide](GETTING_STARTED.md)
- [Tutorial](TUTORIAL.md)
- [Reference Manual](REFERENCE.md)
- [FAQ](FAQ.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Architecture Diagrams](ARCHITECTURE.md)
- [Usage Examples](EXAMPLES.md)

---

**API Documentation Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator