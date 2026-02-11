# Usage Examples

**Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Holographic Architecture Examples](#holographic-architecture-examples)
3. [Physics System Examples](#physics-system-examples)
4. [Validation Framework Examples](#validation-framework-examples)
5. [Exploration System Examples](#exploration-system-examples)
6. [Entity System Examples](#entity-system-examples)
7. [Soul Stream Examples](#soul-stream-examples)
8. [Veil System Examples](#veil-system-examples)
9. [Coordinate System Examples](#coordinate-system-examples)
10. [Multi-Scale System Examples](#multi-scale-system-examples)
11. [Light Architecture Examples](#light-architecture-examples)
12. [Physical Dimension Examples](#physical-dimension-examples)
13. [Transformation Engine Examples](#transformation-engine-examples)
14. [Scale Architecture Examples](#scale-architecture-examples)
15. [Fractal-Holographic Structure Examples](#fractal-holographic-structure-examples)
16. [Decision Engine Examples](#decision-engine-examples)
17. [Enhanced Veil Examples](#enhanced-veil-examples)
18. [Organic Reality Generator Examples](#organic-reality-generator-examples)
19. [Dual-Dimensional Integration Examples](#dual-dimensional-integration-examples)
20. [Complete Simulation Examples](#complete-simulation-examples)

---

## Getting Started

### Example 1: Creating a Complex Vector

```rust
use holonic_realms::holographic::ComplexVector;

fn main() {
    // Create a complex vector
    let cv = ComplexVector::new(1.0, 0.0);

    // Scale the vector
    let scaled = cv.scale(2.0);

    // Calculate norm
    let norm = cv.norm();

    println!("Complex Vector: amplitude={}, phase={}", cv.amplitude, cv.phase);
    println!("Scaled: amplitude={}, phase={}", scaled.amplitude, scaled.phase);
    println!("Norm: {}", norm);
}
```

### Example 2: Creating a Complex Archetype

```rust
use holonic_realms::holographic::ComplexArchetype;

fn main() {
    // Create a complex archetype
    let archetype = ComplexArchetype::new(1.0, 0.0);

    // Convert to vector
    let vector = archetype.to_vector();

    println!("Archetype: amplitude={}, phase={}", archetype.amplitude, archetype.phase);
    println!("Vector: amplitude={}, phase={}", vector.amplitude, vector.phase);
}
```

### Example 3: Creating a Holographic Field

```rust
use holonic_realms::holographic::{HolographicField, InvolutionLayer};

fn main() {
    // Create a holographic field
    let field = HolographicField::new(
        1.0,  // aperture_size
        1000.0,  // spatial_frequency
        InvolutionLayer::Violet,
    );

    // Get resolution
    let resolution = field.get_resolution();

    // Get layer info
    let layer_info = field.get_layer_info();

    println!("Resolution: {}", resolution);
    println!("Layer: {:?}", layer_info.layer);
}
```

---

## Holographic Architecture Examples

### Example 1: Creating an Interference Pattern

```rust
use holonic_realms::holographic::{
    ComplexVector, InterferencePattern, Position,
};

fn main() {
    // Create complex vectors for archetypes
    let archetypes = vec![
        ComplexVector::new(1.0, 0.0),
        ComplexVector::new(1.0, std::f64::consts::PI / 2.0),
        ComplexVector::new(1.0, std::f64::consts::PI),
    ];

    // Create positions
    let positions = vec![
        Position::new(0.0, 0.0, 0.0),
        Position::new(1.0, 0.0, 0.0),
        Position::new(0.0, 1.0, 0.0),
        Position::new(1.0, 1.0, 0.0),
    ];

    // Create interference pattern
    let pattern = InterferencePattern::new(&archetypes, positions);

    // Calculate intensity at a position
    let position = Position::new(0.5, 0.5, 0.0);
    let intensity = pattern.calculate_intensity(&position);

    println!("Intensity at (0.5, 0.5, 0.0): {}", intensity);

    // Find constructive nodes
    let constructive_nodes = pattern.find_constructive_nodes(0.8);
    println!("Constructive nodes: {} found", constructive_nodes.len());

    // Find destructive nodes
    let destructive_nodes = pattern.find_destructive_nodes(0.2);
    println!("Destructive nodes: {} found", destructive_nodes.len());
}
```

### Example 2: Creating a Holographic Entity

```rust
use holonic_realms::holographic::{
    ComplexArchetype, HolographicEntity, InvolutionLayer, Position,
};

fn main() {
    // Create archetype encoding
    let mut archetype_encoding = [ComplexArchetype::new(0.0, 0.0); 22];
    for i in 0..22 {
        archetype_encoding[i] = ComplexArchetype::new(1.0, (i as f64) * 0.1);
    }

    // Create holographic entity
    let mut entity = HolographicEntity::new(
        "entity_001".to_string(),
        archetype_encoding,
    );

    // Set position
    entity.with_position(Position::new(0.0, 0.0, 0.0));

    // Set layer
    entity.with_layer(InvolutionLayer::Yellow);

    // Get views
    let mind_view = entity.get_mind_view();
    let body_view = entity.get_body_view();
    let spirit_view = entity.get_spirit_view();

    println!("Entity ID: {}", entity.id);
    println!("Layer: {:?}", entity.layer);
    println!("Mind View: {:?}", mind_view);
    println!("Body View: {:?}", body_view);
    println!("Spirit View: {:?}", spirit_view);

    // Calculate resonance with another entity
    let mut entity2 = HolographicEntity::new(
        "entity_002".to_string(),
        archetype_encoding,
    );
    entity2.with_position(Position::new(1.0, 0.0, 0.0));

    let resonance = entity.calculate_resonance(&entity2);
    println!("Resonance: {}", resonance);
}
```

### Example 3: Creating an Oscillator Network

```rust
use holonic_realms::holographic::{
    ArchetypeOscillator, OscillatorNetwork, SynchronizationState,
};

fn main() {
    // Create oscillator network
    let mut network = OscillatorNetwork::new(0.5);

    // Add oscillators
    for i in 0..22 {
        let oscillator = ArchetypeOscillator::new(
            i as f64,  // frequency
            (i as f64) * 0.1,  // phase
            1.0,  // amplitude
        );
        network.add_oscillator(oscillator);
    }

    // Synchronize network
    let sync_state = network.synchronize(0.1);

    println!("Synchronization State: {:?}", sync_state);

    // Calculate phase coherence
    let coherence = network.calculate_phase_coherence();
    println!("Phase Coherence: {}", coherence);

    // Get synchronization state
    let state = network.get_synchronization_state();
    println!("State: {:?}", state);
}
```

### Example 4: Creating Holographic Memory

```rust
use holonic_realms::holographic::{
    Experience, HolographicMemory, Hypervector,
};

fn main() {
    // Create holographic memory
    let mut memory = HolographicMemory::new(1000);

    // Create experience
    let experience = Experience {
        id: "experience_001".to_string(),
        timestamp: 1234567890,
        hypervector: Hypervector::new(1000),
        description: "First experience".to_string(),
    };

    // Store experience
    memory.store_experience(experience);

    // Retrieve similar experiences
    let query = Hypervector::new(1000);
    let similar = memory.retrieve_similar(&query, 5);

    println!("Similar experiences: {} found", similar.len());

    // Associate hypervectors
    let hypervectors = vec![
        Hypervector::new(1000),
        Hypervector::new(1000),
        Hypervector::new(1000),
    ];
    let associated = memory.associate(hypervectors);

    println!("Associated hypervector: {:?}", associated);
}
```

### Example 5: Discovering Configurations

```rust
use holonic_realms::holographic::{
    ComplexArchetype, ConfigurationDiscoveryEngine, HolographicField,
    InvolutionLayer,
};

fn main() {
    // Create holographic field
    let field = HolographicField::new(
        1.0,
        1000.0,
        InvolutionLayer::Yellow,
    );

    // Create configuration discovery engine
    let engine = ConfigurationDiscoveryEngine::new(field, 0.8);

    // Create archetypes
    let archetypes = vec![
        ComplexArchetype::new(1.0, 0.0),
        ComplexArchetype::new(1.0, std::f64::consts::PI / 2.0),
        ComplexArchetype::new(1.0, std::f64::consts::PI),
    ];

    // Discover configurations
    let configs = engine.discover_configurations(&archetypes);

    println!("Configurations discovered: {}", configs.len());

    // Classify configurations
    let classification = engine.classify_configurations(&configs);

    println!("Particles: {}", classification.particles.len());
    println!("Atoms: {}", classification.atoms.len());
    println!("Molecules: {}", classification.molecules.len());

    // Calculate stability
    if let Some(config) = configs.first() {
        let stability = engine.calculate_stability(config);
        println!("Stability: {}", stability);
    }
}
```

### Example 6: Condensing Light into Matter

```rust
use holonic_realms::holographic::{
    ComplexArchetype, HolographicField, InvolutionLayer, LightCondensationEngine,
    Photon, PhotonID, PhotonPosition, PhotonState,
};

fn main() {
    // Create holographic field
    let field = HolographicField::new(
        1.0,
        1000.0,
        InvolutionLayer::Yellow,
    );

    // Create light condensation engine
    let engine = LightCondensationEngine::new(field, 0.9);

    // Create photon
    let photon = Photon {
        id: PhotonID::new("photon_001"),
        position: PhotonPosition::new(0.0, 0.0, 0.0),
        state: PhotonState {
            amplitude: 1.0,
            phase: 0.0,
            archetype_encoding: vec![
                ComplexArchetype::new(1.0, 0.0),
                ComplexArchetype::new(1.0, std::f64::consts::PI / 2.0),
            ],
        },
    };

    // Condense light
    let result = engine.condense_light(&photon);

    println!("Condensed: {}", result.condensed);
    println!("Resonance: {:?}", result.resonance);
    println!("Configuration: {:?}", result.configuration);

    // Calculate condensation probability
    let probability = engine.calculate_condensation_probability(&photon);
    println!("Condensation Probability: {}", probability);
}
```

---

## Physics System Examples

### Example 1: Using Dual Physics System

```rust
use holonic_realms::physics::{
    DualPhysicsSystem, PhysicsMode, Particle, ParticleType,
};

fn main() {
    // Create dual physics system
    let mut system = DualPhysicsSystem::new();

    // Set mode to holographic
    system.set_mode(PhysicsMode::Holographic);

    // Create particles
    let particle1 = Particle::new(
        "particle_001".to_string(),
        ParticleType::Electron,
    );
    let particle2 = Particle::new(
        "particle_002".to_string(),
        ParticleType::Proton,
    );

    // Calculate force
    let force = system.calculate_force(&particle1, &particle2);
    println!("Force: {}", force);

    // Calculate energy
    let energy = system.calculate_energy(&particle1);
    println!("Energy: {}", energy);

    // Compare modes
    let comparison = system.compare_modes();
    println!("Comparison: {:?}", comparison);
}
```

### Example 2: Using Hardcoded Physics Engine

```rust
use holonic_realms::physics::HardcodedPhysicsEngine;

fn main() {
    // Create hardcoded physics engine
    let engine = HardcodedPhysicsEngine::new();

    // Calculate gravitational force
    let force = engine.calculate_gravitational_force(
        1.0,  // mass1
        2.0,  // mass2
        1.0,  // distance
    );
    println!("Gravitational Force: {}", force);

    // Calculate electromagnetic force
    let force = engine.calculate_electromagnetic_force(
        1.0,  // charge1
        -1.0,  // charge2
        1.0,  // distance
    );
    println!("Electromagnetic Force: {}", force);
}
```

### Example 3: Using Holographic Physics Engine

```rust
use holonic_realms::holographic::{
    ComplexArchetype, HolographicEntity, HolographicField, InvolutionLayer,
    Position,
};
use holonic_realms::physics::HolographicPhysicsEngine;

fn main() {
    // Create holographic field
    let field = HolographicField::new(
        1.0,
        1000.0,
        InvolutionLayer::Yellow,
    );

    // Create holographic physics engine
    let engine = HolographicPhysicsEngine::new(field);

    // Create holographic entities
    let mut archetype_encoding = [ComplexArchetype::new(0.0, 0.0); 22];
    for i in 0..22 {
        archetype_encoding[i] = ComplexArchetype::new(1.0, (i as f64) * 0.1);
    }

    let mut entity1 = HolographicEntity::new(
        "entity_001".to_string(),
        archetype_encoding,
    );
    entity1.with_position(Position::new(0.0, 0.0, 0.0));

    let mut entity2 = HolographicEntity::new(
        "entity_002".to_string(),
        archetype_encoding,
    );
    entity2.with_position(Position::new(1.0, 0.0, 0.0));

    // Calculate force
    let force = engine.calculate_force(&entity1, &entity2);
    println!("Force: {}", force);

    // Calculate resonance
    let resonance = engine.calculate_resonance(&entity1, &entity2);
    println!("Resonance: {}", resonance);
}
```

---

## Validation Framework Examples

### Example 1: Running All Validation Tests

```rust
use holonic_realms::validation::ValidationSuite;

fn main() {
    // Create validation suite
    let suite = ValidationSuite::new();

    // Run all tests
    let report = suite.run_all_tests();

    println!("Validation Report: {:?}", report);
}
```

### Example 2: Running Architecture Tests

```rust
use holonic_realms::holographic::{
    ComplexArchetype, HolographicEntity, InvolutionLayer, Position,
};
use holonic_realms::validation::ArchitectureTests;

fn main() {
    // Create architecture tests
    let tests = ArchitectureTests::new();

    // Create holographic entity
    let mut archetype_encoding = [ComplexArchetype::new(0.0, 0.0); 22];
    for i in 0..22 {
        archetype_encoding[i] = ComplexArchetype::new(1.0, (i as f64) * 0.1);
    }

    let mut entity = HolographicEntity::new(
        "entity_001".to_string(),
        archetype_encoding,
    );
    entity.with_position(Position::new(0.0, 0.0, 0.0));

    // Test holographic completeness
    let completeness = tests.test_holographic_completeness(&entity);
    println!("Holographic Completeness: {}", completeness);
}
```

### Example 3: Running Physics Recovery Tests

```rust
use holonic_realms::holographic::{
    ComplexArchetype, HolographicEntity, InvolutionLayer, Position,
};
use holonic_realms::validation::PhysicsRecoveryTests;

fn main() {
    // Create physics recovery tests
    let tests = PhysicsRecoveryTests::new();

    // Create holographic entity
    let mut archetype_encoding = [ComplexArchetype::new(0.0, 0.0); 22];
    for i in 0..22 {
        archetype_encoding[i] = ComplexArchetype::new(1.0, (i as f64) * 0.1);
    }

    let mut entity = HolographicEntity::new(
        "entity_001".to_string(),
        archetype_encoding,
    );
    entity.with_position(Position::new(0.0, 0.0, 0.0));

    // Test mass recovery
    let mass_recovery = tests.test_mass_recovery(&entity, 1.0);
    println!("Mass Recovery: {}", mass_recovery);

    // Test charge recovery
    let charge_recovery = tests.test_charge_recovery(&entity, 1.0);
    println!("Charge Recovery: {}", charge_recovery);

    // Test spin recovery
    let spin_recovery = tests.test_spin_recovery(&entity, 0.5);
    println!("Spin Recovery: {}", spin_recovery);
}
```

---

## Exploration System Examples

### Example 1: Exploring Variant Architectures

```rust
use holonic_realms::exploration::{
    ArchitectureVariant, PhysicalLawsVariant, UniverseExplorer,
};

fn main() {
    // Create universe explorer
    let explorer = UniverseExplorer::with_default_architecture();

    // Create physical laws variant
    let variant = ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant {
        gravity_modifier: 1.5,
        electromagnetic_force_modifier: 1.0,
        strong_force_modifier: 1.0,
        weak_force_modifier: 1.0,
        speed_of_light_modifier: 1.0,
        planck_constant_modifier: 1.0,
    });

    // Explore variant
    let result = explorer.explore_variant(variant);

    println!("Variant Description: {}", result.variant_description);
    println!("Novelty Score: {}", result.novelty_score);
    println!("Stability Score: {}", result.stability_score);
    println!("Configurations Discovered: {}", result.discovered_configurations.len());
}
```

### Example 2: Exploring Multiple Variants

```rust
use holonic_realms::exploration::{
    ArchitectureVariant, PhysicalLawsVariant, UniverseExplorer,
};

fn main() {
    // Create universe explorer
    let mut explorer = UniverseExplorer::with_default_architecture();

    // Create multiple variants
    let variants = vec![
        ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant {
            gravity_modifier: 1.5,
            electromagnetic_force_modifier: 1.0,
            strong_force_modifier: 1.0,
            weak_force_modifier: 1.0,
            speed_of_light_modifier: 1.0,
            planck_constant_modifier: 1.0,
        }),
        ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant {
            gravity_modifier: 1.0,
            electromagnetic_force_modifier: 1.5,
            strong_force_modifier: 1.0,
            weak_force_modifier: 1.0,
            speed_of_light_modifier: 1.0,
            planck_constant_modifier: 1.0,
        }),
    ];

    // Explore multiple variants
    let results = explorer.explore_multiple_variants(variants);

    println!("Variants Explored: {}", results.len());

    // Compare variants
    let comparison = explorer.compare_variants(&results);
    println!("Comparison: {:?}", comparison);
}
```

### Example 3: Generating Hypotheses

```rust
use holonic_realms::exploration::{
    DiscoveryDatabase, DiscoveryRecord, HypothesisGenerator,
};

fn main() {
    // Create hypothesis generator
    let generator = HypothesisGenerator::new();

    // Create discovery database
    let mut database = DiscoveryDatabase::new();

    // Add discovery records
    let discovery = DiscoveryRecord {
        id: "discovery_001".to_string(),
        description: "Novel particle discovered".to_string(),
        novelty_score: 0.9,
        stability_score: 0.8,
        timestamp: 1234567890,
    };
    database.add_discovery(discovery);

    // Generate hypotheses
    let discoveries = database.query(DiscoveryQuery::new());
    let hypotheses = generator.generate_hypotheses(&discoveries);

    println!("Hypotheses Generated: {}", hypotheses.len());

    // Generate testable predictions
    let predictions = generator.generate_testable_predictions(&hypotheses);
    println("Testable Predictions: {}", predictions.len());
}
```

### Example 4: Recognizing Patterns

```rust
use holonic_realms::exploration::{
    DiscoveryDatabase, DiscoveryRecord, HypothesisGenerator,
};

fn main() {
    // Create hypothesis generator
    let generator = HypothesisGenerator::new();

    // Create discovery database
    let mut database = DiscoveryDatabase::new();

    // Add discovery records
    for i in 0..10 {
        let discovery = DiscoveryRecord {
            id: format!("discovery_{}", i),
            description: format!("Discovery {}", i),
            novelty_score: 0.5 + (i as f64) * 0.05,
            stability_score: 0.5 + (i as f64) * 0.05,
            timestamp: 1234567890 + i,
        };
        database.add_discovery(discovery);
    }

    // Recognize patterns
    let discoveries = database.query(DiscoveryQuery::new());
    let patterns = generator.recognize_patterns(&discoveries);

    println!("Patterns Recognized: {}", patterns.len());
}
```

---

## Entity System Examples

### Example 1: Creating an Entity

```rust
use holonic_realms::entity::{ArchetypeState, Entity, VibrationalState};

fn main() {
    // Create entity
    let mut entity = Entity::new("entity_001".to_string());

    // Set archetype state
    entity.archetype_state = ArchetypeState::new();

    // Set vibrational state
    entity.vibrational_state = VibrationalState::new();

    println!("Entity ID: {}", entity.id);
}
```

### Example 2: Making Entity Choices

```rust
use holonic_realms::entity::{Catalyst, CatalystType, Choice, ChoiceContext, Entity};

fn main() {
    // Create entity
    let mut entity = Entity::new("entity_001".to_string());

    // Create choice context
    let context = ChoiceContext {
        catalyst: Catalyst {
            catalyst_type: CatalystType::Environmental,
            intensity: 0.8,
        },
        environment: Default::default(),
        polarization: Default::default(),
    };

    // Make choice
    let choice = entity.make_choice(context);

    println!("Choice: {:?}", choice);
}
```

---

## Soul Stream Examples

### Example 1: Creating a Soul Stream

```rust
use holonic_realms::soul_stream::{
    EvolutionaryGoal, KarmicPattern, PolarizationState, SoulStream,
};

fn main() {
    // Create soul stream
    let mut soul_stream = SoulStream::new();

    // Set karmic pattern
    soul_stream.karmic_pattern = KarmicPattern::new();

    // Set polarization state
    soul_stream.polarization_state = PolarizationState::ServiceToOthers(0.8);

    // Set evolutionary goal
    soul_stream.evolutionary_goal = EvolutionaryGoal::new("Ascension".to_string());

    println!("Soul Stream: {:?}", soul_stream);
}
```

---

## Veil System Examples

### Example 1: Creating a Veil Mechanism

```rust
use holonic_realms::veil::{
    DensityTransparency, PiercingEvent, PiercingEventType, PiercingLocation, VeilMechanism,
};

fn main() {
    // Create veil mechanism
    let mut veil = VeilMechanism::new();

    // Create piercing event
    let event = PiercingEvent {
        event_type: PiercingEventType::Catalyst,
        location: PiercingLocation::EnergyCenter,
        intensity: 0.9,
    };

    // Pierce veil
    let result = veil.pierce(event);

    println!("Piercing Result: {:?}", result);

    // Get transparency
    let transparency = veil.get_transparency(DensityTransparency::ThirdDensity);
    println!("Transparency: {}", transparency);
}
```

---

## Coordinate System Examples

### Example 1: Navigating Dual Realms

```rust
use holonic_realms::coordinates::{
    DualRealmNavigator, MetaphysicalTimeSpaceCoord, PhysicalSpaceTimeCoord, RealmTranslation,
};

fn main() {
    // Create navigator
    let mut navigator = DualRealmNavigator::new();

    // Create metaphysical coordinate
    let metaphysical = MetaphysicalTimeSpaceCoord {
        density: 3.0,
        polarization: 0.8,
    };

    // Create physical coordinate
    let physical = PhysicalSpaceTimeCoord {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        t: 0.0,
    };

    // Navigate to realm
    let result = navigator.navigate_to(RealmTranslation::MetaphysicalToPhysical);

    println!("Navigation Result: {:?}", result);
}
```

---

## Multi-Scale System Examples

### Example 1: Creating a Multi-Scale Hierarchy

```rust
use holonic_realms::multi_scale::{
    MultiScaleHierarchy, Scale, ScaleID, ScaleType,
};

fn main() {
    // Create multi-scale hierarchy
    let mut hierarchy = MultiScaleHierarchy::new();

    // Add scales
    let scale = Scale {
        scale_id: ScaleID::new("scale_001"),
        scale_type: ScaleType::Atomic,
    };
    hierarchy.add_scale(scale);

    println!("Scales: {}", hierarchy.scales.len());
}
```

---

## Light Architecture Examples

### Example 1: Creating a Photon

```rust
use holonic_realms::light::{
    Photon, PhotonID, PhotonPosition, PhotonState,
};

fn main() {
    // Create photon
    let photon = Photon {
        id: PhotonID::new("photon_001"),
        position: PhotonPosition::new(0.0, 0.0, 0.0),
        state: PhotonState {
            amplitude: 1.0,
            phase: 0.0,
            archetype_encoding: vec![],
        },
    };

    println!("Photon ID: {:?}", photon.id);
}
```

---

## Physical Dimension Examples

### Example 1: Creating a Physical Dimension

```rust
use holonic_realms::physical_dimension::{
    PhysicalDimension, PhysicalExperience, PhysicalExperienceType, PhysicalOutcome,
};

fn main() {
    // Create physical dimension
    let dimension = PhysicalDimension {
        space: Default::default(),
        time: Default::default(),
    };

    // Create physical experience
    let experience = PhysicalExperience {
        experience_type: PhysicalExperienceType::Sensation,
        outcome: PhysicalOutcome::Learning,
    };

    println!("Physical Dimension: {:?}", dimension);
    println!("Physical Experience: {:?}", experience);
}
```

---

## Transformation Engine Examples

### Example 1: Transforming Energy

```rust
use holonic_realms::transformation_engine::{
    ConsciousnessType, IntelligentEnergy, TransformationEngine, TransformationResult,
};

fn main() {
    // Create transformation engine
    let mut engine = TransformationEngine::new();

    // Create intelligent energy
    let energy = IntelligentEnergy {
        energy_type: ConsciousnessType::Logos,
        amplitude: 1.0,
    };

    // Transform energy
    let result = engine.transform(energy);

    println!("Transformation Result: {:?}", result);
}
```

---

## Scale Architecture Examples

### Example 1: Creating a Scale Hierarchy

```rust
use holonic_realms::scale_architecture::{
    Scale, ScaleHierarchy, ScaleID, ScaleType,
};

fn main() {
    // Create scale hierarchy
    let mut hierarchy = ScaleHierarchy::new();

    // Add scales
    let scale = Scale {
        scale_id: ScaleID::new("scale_001"),
        scale_type: ScaleType::Atomic,
    };
    hierarchy.add_scale(scale);

    println!("Scales: {}", hierarchy.scales.len());
}
```

---

## Fractal-Holographic Structure Examples

### Example 1: Creating a Holographic Container

```rust
use holonic_realms::fractal_holographic_structure::{
    HolographicContainer, SubDensityState,
};

fn main() {
    // Create holographic container
    let mut container = HolographicContainer::new();

    // Add sub-density states
    let state = SubDensityState::new();
    container.sub_density_states.push(state);

    println!("Sub-Density States: {}", container.sub_density_states.len());
}
```

---

## Decision Engine Examples

### Example 1: Making a Decision

```rust
use holonic_realms::decision_engine::{
    DecisionContext, DecisionEngine, DecisionExplanation, EntityID,
};

fn main() {
    // Create decision engine
    let engine = DecisionEngine::new(EntityID::new("entity_001"));

    // Create decision context
    let context = DecisionContext {
        contributing_factors: vec![],
        environmental_influences: vec![],
    };

    // Make decision
    let decision = engine.make_decision(context);

    println!("Decision: {:?}", decision.decision);
    println!("Explanation: {:?}", decision.explanation);
}
```

---

## Enhanced Veil Examples

### Example 1: Creating an Enhanced Veil

```rust
use holonic_realms::enhanced_veil::{FullAwareness, LimitedAwareness, Veil};

fn main() {
    // Create enhanced veil
    let veil = Veil {
        full_awareness: FullAwareness::new(),
        limited_awareness: LimitedAwareness::new(),
    };

    // Get illusion of separation
    let illusion = veil.get_illusion_of_separation();

    println!("Illusion of Separation: {:?}", illusion);
}
```

---

## Organic Reality Generator Examples

### Example 1: Generating an Organic Reality

```rust
use holonic_realms::organic_reality_generator::{
    NaturalLawsGenerator, OrganicReality, OrganicRealityGenerator,
};

fn main() {
    // Create organic reality generator
    let mut generator = OrganicRealityGenerator::new();

    // Generate reality
    let reality = generator.generate_reality();

    println!("Reality Characteristics: {:?}", reality.characteristics);
}
```

---

## Dual-Dimensional Integration Examples

### Example 1: Integrating Dual Dimensions

```rust
use holonic_realms::dual_dimensional_integration::{
    DualDimensionalIntegration, IntegrationResult, IntegrationState,
};

fn main() {
    // Create dual-dimensional integration
    let mut integration = DualDimensionalIntegration::new();

    // Integrate
    let result = integration.integrate();

    println!("Integration Result: {:?}", result);
}
```

---

## Complete Simulation Examples

### Example 1: Running a Complete Simulation

```rust
use holonic_realms::complete_simulation::{
    CompleteSimulation, SimulationResult, SimulationStatistics,
};

fn main() {
    // Create complete simulation
    let mut simulation = CompleteSimulation::new();

    // Run simulation
    let result = simulation.run();

    println!("Simulation Result: {:?}", result);

    // Get statistics
    let statistics = simulation.get_statistics();
    println!("Simulation Statistics: {:?}", statistics);
}
```

---

## Advanced Examples

### Example 1: Complete Workflow

```rust
use holonic_realms::holographic::{
    ComplexArchetype, ConfigurationDiscoveryEngine, HolographicEntity, HolographicField,
    InvolutionLayer, Position,
};
use holonic_realms::exploration::{
    ArchitectureVariant, PhysicalLawsVariant, UniverseExplorer,
};
use holonic_realms::physics::{DualPhysicsSystem, PhysicsMode};
use holonic_realms::validation::ValidationSuite;

fn main() {
    // Step 1: Create holographic field
    let field = HolographicField::new(
        1.0,
        1000.0,
        InvolutionLayer::Yellow,
    );

    // Step 2: Create holographic entity
    let mut archetype_encoding = [ComplexArchetype::new(0.0, 0.0); 22];
    for i in 0..22 {
        archetype_encoding[i] = ComplexArchetype::new(1.0, (i as f64) * 0.1);
    }

    let mut entity = HolographicEntity::new(
        "entity_001".to_string(),
        archetype_encoding,
    );
    entity.with_position(Position::new(0.0, 0.0, 0.0));

    // Step 3: Discover configurations
    let engine = ConfigurationDiscoveryEngine::new(field, 0.8);
    let configs = engine.discover_configurations(&archetype_encoding.to_vec());

    println!("Configurations Discovered: {}", configs.len());

    // Step 4: Explore variant architectures
    let mut explorer = UniverseExplorer::with_default_architecture();
    let variant = ArchitectureVariant::DifferentPhysicalLaws(PhysicalLawsVariant {
        gravity_modifier: 1.5,
        electromagnetic_force_modifier: 1.0,
        strong_force_modifier: 1.0,
        weak_force_modifier: 1.0,
        speed_of_light_modifier: 1.0,
        planck_constant_modifier: 1.0,
    });

    let result = explorer.explore_variant(variant);
    println!("Novelty Score: {}", result.novelty_score);

    // Step 5: Use dual physics system
    let mut physics = DualPhysicsSystem::new();
    physics.set_mode(PhysicsMode::Holographic);

    // Step 6: Run validation tests
    let suite = ValidationSuite::new();
    let report = suite.run_all_tests();

    println!("Validation Report: {:?}", report);
}
```

---

## See Also

- [API Documentation](API.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Tutorial](TUTORIAL.md)
- [Reference Manual](REFERENCE.md)
- [FAQ](FAQ.md)
- [Troubleshooting Guide](TROUBLESHOOTING.md)
- [Architecture Diagrams](ARCHITECTURE.md)

---

**Usage Examples Version**: 1.0.0
**Last Updated**: January 31, 2026
**Project**: Holographic Multi-Scale Cosmic Creation Simulator