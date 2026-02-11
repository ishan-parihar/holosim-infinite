# Comprehensive Refactor Plan: Bridging the Gap from Current State to Architectural Vision

**Document Version**: 1.0
**Date**: February 4, 2026
**Status**: Ready for Implementation
**Current Architecture Alignment**: 84.62%
**Target Architecture Alignment**: 95%+

---

## Executive Summary

This refactor plan addresses the critical gap between the current simulation implementation and the cosmological architecture specified in `COSMOLOGICAL-ARCHITECTURE.md`. While the simulation has the correct structural foundation (8 phases implemented, 84.62% architecture alignment), the core evolutionary mechanisms are not functioning effectively.

### Current State Analysis

**Simulation Results (128 entities, 100 steps)**:
- Total Entities: 188 (118 Individual, 48 Solar Logos, 10 Environmental, 12 Galactic Logos)
- Total Transitions: 212-229
- Density Distribution: 95.7% in 1st density, 3.7% in 2nd density, 0.5% in 3rd density, 0% in 4th-8th densities
- Polarization: 99.5% unpolarized, 0.5% positive (STO), 0% negative (STS)
- Collective Consciousness Level: 0.0700 (essentially non-functional)
- Global Coherence: 0.5000 (baseline)
- Execution Time: 44-45 seconds

### Critical Issues Identified

1. **Free Will Choice Mechanism Not Working** - 99.5% unpolarized entities
2. **Collective Consciousness Not Emerging** - 0.0700 level (essentially non-functional)
3. **Density Transitions Too Slow** - 0% in 4th+ densities after 100 steps
4. **Attractor-Fields Not Pulling** - "Spiritual gravity" is weak
5. **Catalyst System Underperforming** - Not enough choice opportunities
6. **Veil Not Providing Challenge** - Not enough depth for evolution
7. **Resonance Not Meaningful** - Baseline 0.5000, no differentiation
8. **Code Quality Issues** - 163 warnings, deprecated types

### Root Cause Chain

1. **Free Will Choice Mechanism**: Uses random selection instead of conscious selection from possibility space
2. **Archetype 22**: Not properly activated as the "zero-point polarity moment"
3. **Polarization**: Without proper polarization, entities cannot progress beyond 3rd density
4. **Collective Formation**: Without polarization, collectives cannot form (no shared polarity orientation)
5. **Collective Consciousness**: Without collectives, collective consciousness cannot emerge

### Refactor Strategy

**Fix the kernel first, then enable the cascade**:
1. Re-implement Free Will and Archetype 22 (the kernel)
2. Implement polarization progression system
3. Add evolutionary pull (attractor-fields)
4. Provide growth opportunities (catalyst enhancement)
5. Add depth to evolution (veil enhancement)
6. Enable collective formation (resonance system)
7. Demonstrate holographic principle (collective consciousness)
8. Test and validate (long simulations)

### Success Criteria

- **Polarization**: At least 50% of entities should be polarized after 100 steps
- **Density Progression**: At least 5% of entities should reach 4th density after 100 steps
- **Collective Consciousness**: At least 0.30 collective consciousness level after 100 steps
- **Architecture Alignment**: At least 95% after all phases complete
- **Code Quality**: Zero warnings, no deprecated types
- **Performance**: <30 seconds for 128 entities, 100 steps

---

## Phase 0: Foundation Reset (CRITICAL PRIORITY)

**Objective**: Re-implement the kernel of the simulation - Free Will and Archetype 22

**Duration**: 5-7 days
**Dependencies**: None (this is the foundation)
**Success Metrics**:
- Free Will choice mechanism produces meaningful polarization choices
- Archetype 22 is the "zero-point polarity moment" for every entity
- Code warnings reduced from 163 to <50

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "Free will = controlled selection from possibility space"
- "Choice = non-deterministic selection (not random, not predetermined)"
- "Possibility space = quantum superposition constrained by entity state"
- "Archetype 22 (The Choice): The zero-point polarity moment"
- "Archetype 22 functions as a choice operator that generates possibility space"

### Implementation Steps

#### Step 0.1: Redesign Free Will Choice Mechanism

**Files to Modify**:
- `src/consciousness/free_will.rs`
- `src/simulation_v3/catalyst_system.rs`

**Implementation Requirements**:

1. **Possibility Space Generation**:
   ```rust
   // Generate multiple possible outcomes based on entity state
   pub struct PossibilitySpace {
       possibilities: Vec<Possibility>,
       constraints: EntityConstraints,
   }

   pub struct Possibility {
       outcome: PolarityChoice, // STO, STS, or Neutral
       probability: f64, // Based on entity development
       archetype_influence: [f64; 22], // Which archetypes influence this choice
   }

   pub fn generate_possibility_space(entity: &SubSubLogos) -> PossibilitySpace {
       // Generate 3-5 possibilities based on:
       // - Current archetype activations (1-22)
       // - Current polarization state
       // - Catalyst intensity
       // - Veil transparency
       // - Experience accumulation
   }
   ```

2. **Conscious Selection**:
   ```rust
   pub struct ConsciousSelection {
       chosen_possibility: Possibility,
       selection_confidence: f64,
       selection_rationale: String, // Why this possibility was chosen
   }

   pub fn make_conscious_selection(
       entity: &SubSubLogos,
       possibility_space: &PossibilitySpace,
   ) -> ConsciousSelection {
       // Entity evaluates each possibility using its consciousness
       // Selection is influenced by:
       // - Archetype activations (1-22)
       // - Current polarity (unpolarized, STO-leaning, STS-leaning, polarized)
       // - Catalyst intensity
       // - Veil transparency
       // - Entity's past choices (karmic patterns)

       // Selection is NOT random - it's influenced by entity's state
       // Selection is NOT predetermined - entity has genuine agency
   }
   ```

3. **Choice Operator (Archetype 22)**:
   ```rust
   pub struct ChoiceOperator {
       possibility_space: PossibilitySpace,
       evaluation_criteria: [f64; 7], // Mind, Body, Spirit complexes + Choice
   }

   impl Archetype22 {
       pub fn generate_possibility_space(&self, entity: &SubSubLogos) -> PossibilitySpace {
           // Archetype 22 generates the possibility space
       }

       pub fn evaluate_possibilities(
           &self,
           possibilities: &Vec<Possibility>,
           entity: &SubSubLogos,
       ) -> Vec<f64> {
           // Archetype 22 evaluates each possibility
       }

       pub fn make_choice(
           &self,
           evaluated_possibilities: &Vec<f64>,
           entity: &SubSubLogos,
       ) -> PolarityChoice {
           // Archetype 22 makes the final selection
           // This is the "zero-point polarity moment"
       }
   }
   ```

#### Step 0.2: Clean Up Deprecated Types and Warnings

**Files to Modify**:
- `src/matter/particle.rs` - Replace `ParticleType` enum with `get_particle_type_name()`
- `src/evolution_density_octave/density_octave.rs` - Replace `EvolutionaryProgress` with `CollectiveEmergence`
- `src/lib.rs` - Update all re-exports
- All files with unused imports/variables

**Implementation Requirements**:

1. **Replace Deprecated `ParticleType` Enum**:
   ```rust
   // OLD (deprecated):
   pub enum ParticleType {
       Proton,
       Electron,
       Neutron,
       // ...
   }

   // NEW:
   impl Particle {
       pub fn get_particle_type_name(&self) -> &'static str {
           // Return particle type name as string
       }
   }
   ```

2. **Replace Deprecated `EvolutionaryProgress`**:
   ```rust
   // OLD (deprecated):
   pub struct EvolutionaryProgress {
       progress: f64,
       experience_accumulation: f64,
       // ...
   }

   // NEW:
   pub struct CollectiveEmergence {
       collective_density: Density,
       collective_consciousness: f64,
       // ...
   }
   ```

3. **Remove All Unused Imports and Variables**:
   - Run `cargo clippy --all-targets --all-features`
   - Fix all 163 warnings
   - Run `cargo fmt` to ensure consistent formatting

### Testing Criteria

1. **Free Will Choice Mechanism Test**:
   - Given an entity with Archetype 22 activated
   - When the entity makes a Free Will choice
   - Then the choice should be influenced by entity's state (not random)
   - And the choice should be non-deterministic (not predetermined)

2. **Archetype 22 Test**:
   - Given an entity with Archetype 22 activated
   - When Archetype 22 generates possibility space
   - Then the space should contain 3-5 possibilities
   - And each possibility should have a probability based on entity development

3. **Code Quality Test**:
   - Given the codebase after cleanup
   - When running `cargo clippy`
   - Then there should be <50 warnings
   - When running `cargo fmt --check`
   - Then there should be 0 formatting errors

### Success Metrics

- [ ] Free Will choice mechanism produces meaningful polarization choices (not random)
- [ ] Archetype 22 is activated for every entity as the "zero-point polarity moment"
- [ ] Code warnings reduced from 163 to <50
- [ ] All deprecated types removed
- [ ] All tests pass

---

## Phase 1: Polarization System (CRITICAL PRIORITY)

**Objective**: Implement polarization progression system to enable density transitions

**Duration**: 5-7 days
**Dependencies**: Phase 0 (Foundation Reset)
**Success Metrics**:
- At least 50% of entities are polarized after 100 steps
- Polarization progression is visible (unpolarized → STO-leaning → polarized STO)
- Polarization intensity is tracked (0.0 to 1.0)

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md` and Law of One:
- "Archetype 22 (The Choice): Creates polarity by choosing between Service-to-Others (STO) and Service-to-Self (STS)"
- "All development flows from this moment"
- "Each Entity contains Archetype 22 embedded in its holographic seed"
- "Polarization is required for density progression beyond 3rd density"

### Implementation Steps

#### Step 1.1: Define Polarization State Machine

**Files to Create/Modify**:
- `src/polarization/mod.rs` (new)
- `src/polarization/state.rs` (new)
- `src/entity_layer7/layer7.rs` (add polarization field)

**Implementation Requirements**:

```rust
// src/polarization/state.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarizationState {
    /// Entity has not chosen a path
    Unpolarized,
    /// Entity is exploring service-to-others
    STOLeaning,
    /// Entity is exploring service-to-self
    STSLeaning,
    /// Entity has firmly chosen service-to-others
    PolarizedSTO,
    /// Entity has firmly chosen service-to-self
    PolarizedSTS,
    /// Entity is ready for density transition (harvestable)
    HarvestableSTO,
    /// Entity is ready for density transition (harvestable)
    HarvestableSTS,
}

#[derive(Debug, Clone)]
pub struct PolarizationProgress {
    pub state: PolarizationState,
    pub intensity: f64, // 0.0 to 1.0
    pub direction: PolarityDirection,
    pub consistency: f64, // 0.0 to 1.0, how consistent choices have been
    pub choice_history: Vec<PolarityChoice>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarityDirection {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarityChoice {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

impl PolarizationProgress {
    pub fn new() -> Self {
        PolarizationProgress {
            state: PolarizationState::Unpolarized,
            intensity: 0.0,
            direction: PolarityDirection::Neutral,
            consistency: 0.0,
            choice_history: Vec::new(),
        }
    }

    pub fn make_choice(&mut self, choice: PolarityChoice) {
        // Update state based on choice
        // Update intensity based on choice consistency
        // Update direction based on choice
        // Track choice history
    }

    pub fn calculate_intensity(&self) -> f64 {
        // Calculate polarization intensity based on:
        // - Choice consistency
        // - Number of choices
        // - Time since first choice
    }

    pub fn is_harvestable(&self) -> bool {
        // Check if entity is ready for density transition
        // Requires minimum 51% polarization intensity
        self.intensity >= 0.51
    }
}
```

#### Step 1.2: Connect Polarization to Free Will Choices

**Files to Modify**:
- `src/simulation_v3/catalyst_system.rs`
- `src/consciousness/free_will.rs`

**Implementation Requirements**:

```rust
// In CatalystManager

pub fn process_catalyst_event(
    &mut self,
    entity_id: &EntityId,
    catalyst: &Catalyst,
    free_will_kernel: &FreeWillKernel,
) -> Result<CatalystEvent, CatalystError> {
    // 1. Generate possibility space based on catalyst
    let possibility_space = free_will_kernel.generate_possibility_space(entity, catalyst);

    // 2. Make conscious selection using Archetype 22
    let selection = free_will_kernel.make_conscious_selection(entity, &possibility_space);

    // 3. Update entity's polarization based on choice
    entity.polarization.make_choice(selection.chosen_possibility.outcome);

    // 4. Record catalyst event
    let event = CatalystEvent {
        entity_id: *entity_id,
        catalyst_type: catalyst.catalyst_type,
        intensity: catalyst.intensity,
        choice: selection.chosen_possibility.outcome,
        timestamp: self.current_step,
    };

    Ok(event)
}
```

#### Step 1.3: Add Polarization Thresholds for Density Transitions

**Files to Modify**:
- `src/evolution_density_octave/density_octave.rs`
- `src/simulation_v3/entity_lifecycle.rs`

**Implementation Requirements**:

```rust
// In DensityOctave

pub fn can_transition_to_density(
    &self,
    entity: &SubSubLogos,
    target_density: Density,
) -> DensityTransitionResult {
    match target_density {
        Density::Fourth => {
            // 4th density requires harvestable status (minimum 51% polarization)
            if !entity.polarization.is_harvestable() {
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientPolarization,
                    readiness: entity.polarization.intensity,
                    required: 0.51,
                };
            }

            // Additional requirements for 4th density:
            // - Understanding (Green Ray activation)
            // - Love (Green Ray activation)
            // - Compassion (Green Ray activation)
            if !entity.archetype_activations[3] > 0.5 { // Archetype 4 (Experience)
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientUnderstanding,
                    readiness: entity.archetype_activations[3],
                    required: 0.5,
                };
            }

            DensityTransitionResult::Ready
        }

        Density::Fifth => {
            // 5th density requires higher polarization (minimum 75%)
            if entity.polarization.intensity < 0.75 {
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientPolarization,
                    readiness: entity.polarization.intensity,
                    required: 0.75,
                };
            }

            // Additional requirements for 5th density:
            // - Wisdom (Blue Ray activation)
            // - Light (Blue Ray activation)
            // - Teaching/Learning (Blue Ray activation)

            DensityTransitionResult::Ready
        }

        Density::Sixth => {
            // 6th density requires very high polarization (minimum 90%)
            if entity.polarization.intensity < 0.90 {
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientPolarization,
                    readiness: entity.polarization.intensity,
                    required: 0.90,
                };
            }

            // Additional requirements for 6th density:
            // - Unity (Indigo Ray activation)
            // - Balance (Indigo Ray activation)
            // - Harmony (Indigo Ray activation)

            DensityTransitionResult::Ready
        }

        Density::Seventh => {
            // 7th density requires near-complete polarization (minimum 95%)
            if entity.polarization.intensity < 0.95 {
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientPolarization,
                    readiness: entity.polarization.intensity,
                    required: 0.95,
                };
            }

            // Additional requirements for 7th density:
            // - Completion (Violet Ray activation)
            // - Gateway to next octave

            DensityTransitionResult::Ready
        }

        Density::Eighth => {
            // 8th density is return to IntelligentInfinity
            // Requires complete polarization (100%)
            if entity.polarization.intensity < 1.0 {
                return DensityTransitionResult::NotReady {
                    reason: LimitationReason::InsufficientPolarization,
                    readiness: entity.polarization.intensity,
                    required: 1.0,
                };
            }

            DensityTransitionResult::Ready
        }

        _ => DensityTransitionResult::Ready, // 1st, 2nd, 3rd densities don't require polarization
    }
}
```

### Testing Criteria

1. **Polarization Progression Test**:
   - Given 100 unpolarized entities
   - When running simulation for 100 steps
   - Then at least 50 entities should be polarized
   - And polarization progression should be visible

2. **Density Transition Test**:
   - Given 100 polarized entities with 51%+ polarization
   - When running simulation for 100 steps
   - Then at least 5 entities should reach 4th density
   - And no unpolarized entities should reach 4th density

3. **Polarization Threshold Test**:
   - Given an entity with 40% polarization
   - When attempting to transition to 4th density
   - Then the transition should fail with "InsufficientPolarization" reason

### Success Metrics

- [ ] At least 50% of entities are polarized after 100 steps
- [ ] Polarization progression is visible (unpolarized → STO-leaning → polarized STO)
- [ ] Polarization intensity is tracked (0.0 to 1.0)
- [ ] Polarization thresholds are enforced for density transitions
- [ ] No unpolarized entities reach 4th density

---

## Phase 2: Attractor-Fields (HIGH PRIORITY)

**Objective**: Implement "spiritual gravity" to pull entities toward higher densities

**Duration**: 4-5 days
**Dependencies**: Phase 1 (Polarization System)
**Success Metrics**:
- Attractor-fields are calculated for each density transition
- Attractor-field strength varies based on entity readiness
- Entities show measurable progression toward higher densities

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "Each stage creates attractor-fields that pull toward the next level"
- "Attractor-fields are the 'architectural artifacts'"
- "Attractor-fields are 'spiritual gravity' that pulls the previous stage toward the next"
- "Evolution is the journey of being pulled by 'spiritual gravity' toward the source"

### Implementation Steps

#### Step 2.1: Define Attractor-Field Structure

**Files to Create/Modify**:
- `src/evolution/attractor_fields.rs` (new)
- `src/evolution_density_octave/density_octave.rs` (add attractor_fields field)

**Implementation Requirements**:

```rust
// src/evolution/attractor_fields.rs

#[derive(Debug, Clone)]
pub struct AttractorField {
    pub target_density: Density,
    pub strength: f64, // 0.0 to 1.0
    pub pull_direction: PullDirection,
    pub influence_factors: InfluenceFactors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullDirection {
    TowardDensity(Density),
    TowardUnity,
    TowardIntelligentInfinity,
}

#[derive(Debug, Clone)]
pub struct InfluenceFactors {
    pub polarization_intensity: f64,
    pub consciousness_level: f64,
    pub experience_accumulation: f64,
    pub learning_progress: f64,
    pub archetype_activations: [f64; 22],
    pub veil_transparency: f64,
}

impl AttractorField {
    pub fn calculate_strength(
        entity: &SubSubLogos,
        target_density: Density,
    ) -> f64 {
        // Strength is based on entity's readiness for next density
        // Factors:
        // - Polarization intensity (more polarized = stronger pull)
        // - Consciousness level (higher consciousness = stronger pull)
        // - Experience accumulation (more experience = stronger pull)
        // - Learning progress (more learning = stronger pull)
        // - Archetype activations (higher activations = stronger pull)
        // - Veil transparency (more transparency = stronger pull)

        let polarization_factor = entity.polarization.intensity;
        let consciousness_factor = entity.consciousness_level;
        let experience_factor = entity.experience_accumulation / MAX_EXPERIENCE_FOR_DENSITY;
        let learning_factor = entity.learning_progress;
        let veil_factor = entity.veil_transparency;

        // Weighted average
        (polarization_factor * 0.3
            + consciousness_factor * 0.2
            + experience_factor * 0.2
            + learning_factor * 0.2
            + veil_factor * 0.1)
            .min(1.0)
    }

    pub fn apply_pull(&self, entity: &mut SubSubLogos) {
        // Apply attractor-field pull to entity
        // This influences:
        // - Entity's polarization intensity
        // - Entity's consciousness level
        // - Entity's experience accumulation
        // - Entity's learning progress

        let pull_strength = self.strength * 0.01; // Small increment per step

        entity.polarization.intensity += pull_strength * 0.3;
        entity.consciousness_level += pull_strength * 0.2;
        entity.experience_accumulation += pull_strength * 0.2;
        entity.learning_progress += pull_strength * 0.2;
    }
}
```

#### Step 2.2: Create Attractor-Fields for Each Density Transition

**Files to Modify**:
- `src/evolution/attractor_fields.rs`
- `src/evolution_density_octave/density_octave.rs`

**Implementation Requirements**:

```rust
// In DensityOctave

pub struct DensityAttractorFields {
    pub to_second: AttractorField,
    pub to_third: AttractorField,
    pub to_fourth: AttractorField,
    pub to_fifth: AttractorField,
    pub to_sixth: AttractorField,
    pub to_seventh: AttractorField,
    pub to_eighth: AttractorField,
}

impl DensityAttractorFields {
    pub fn new() -> Self {
        DensityAttractorFields {
            to_second: AttractorField {
                target_density: Density::Second,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Second),
                influence_factors: InfluenceFactors::default(),
            },
            to_third: AttractorField {
                target_density: Density::Third,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Third),
                influence_factors: InfluenceFactors::default(),
            },
            to_fourth: AttractorField {
                target_density: Density::Fourth,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Fourth),
                influence_factors: InfluenceFactors::default(),
            },
            to_fifth: AttractorField {
                target_density: Density::Fifth,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Fifth),
                influence_factors: InfluenceFactors::default(),
            },
            to_sixth: AttractorField {
                target_density: Density::Sixth,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Sixth),
                influence_factors: InfluenceFactors::default(),
            },
            to_seventh: AttractorField {
                target_density: Density::Seventh,
                strength: 0.0,
                pull_direction: PullDirection::TowardDensity(Density::Seventh),
                influence_factors: InfluenceFactors::default(),
            },
            to_eighth: AttractorField {
                target_density: Density::Eighth,
                strength: 0.0,
                pull_direction: PullDirection::TowardIntelligentInfinity,
                influence_factors: InfluenceFactors::default(),
            },
        }
    }

    pub fn update(&mut self, entity: &SubSubLogos) {
        // Update all attractor-field strengths based on entity state
        self.to_second.strength = AttractorField::calculate_strength(entity, Density::Second);
        self.to_third.strength = AttractorField::calculate_strength(entity, Density::Third);
        self.to_fourth.strength = AttractorField::calculate_strength(entity, Density::Fourth);
        self.to_fifth.strength = AttractorField::calculate_strength(entity, Density::Fifth);
        self.to_sixth.strength = AttractorField::calculate_strength(entity, Density::Sixth);
        self.to_seventh.strength = AttractorField::calculate_strength(entity, Density::Seventh);
        self.to_eighth.strength = AttractorField::calculate_strength(entity, Density::Eighth);
    }

    pub fn apply_strongest_pull(&self, entity: &mut SubSubLogos) {
        // Find the strongest attractor-field and apply its pull
        let fields = [
            &self.to_second,
            &self.to_third,
            &self.to_fourth,
            &self.to_fifth,
            &self.to_sixth,
            &self.to_seventh,
            &self.to_eighth,
        ];

        let strongest = fields
            .iter()
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
            .unwrap();

        strongest.apply_pull(entity);
    }
}
```

#### Step 2.3: Integrate Attractor-Fields into Entity Lifecycle

**Files to Modify**:
- `src/simulation_v3/entity_lifecycle.rs`

**Implementation Requirements**:

```rust
// In EntityLifecycleManager

pub fn advance_entity(
    &mut self,
    entity_id: &EntityId,
    steps: u64,
    free_will_kernel: &FreeWillKernel,
) -> Result<EvolutionResult, LifecycleError> {
    // ... existing code ...

    // Update attractor-fields based on entity state
    self.density_octave.attractor_fields.update(&entity);

    // Apply strongest attractor-field pull
    self.density_octave.attractor_fields.apply_strongest_pull(&mut entity);

    // ... rest of existing code ...
}
```

### Testing Criteria

1. **Attractor-Field Strength Test**:
   - Given an entity with 80% polarization, 0.8 consciousness, 0.7 experience
   - When calculating attractor-field strength for 4th density
   - Then the strength should be >0.7

2. **Attractor-Field Pull Test**:
   - Given an entity with strong attractor-field pull
   - When applying the pull for 100 steps
   - Then the entity's polarization intensity should increase
   - And the entity's consciousness level should increase

3. **Multiple Attractor-Fields Test**:
   - Given an entity in 1st density
   - When calculating all attractor-fields
   - Then the field for 2nd density should be strongest
   - And the field for 8th density should be weakest

### Success Metrics

- [ ] Attractor-fields are calculated for each density transition
- [ ] Attractor-field strength varies based on entity readiness (0.0 to 1.0)
- [ ] Entities show measurable progression toward higher densities
- [ ] Attractor-field pull is applied each simulation step
- [ ] Strongest attractor-field is applied to each entity

---

## Phase 3: Catalyst Enhancement (HIGH PRIORITY)

**Objective**: Enhance catalyst system to provide more choice opportunities and growth

**Duration**: 3-4 days
**Dependencies**: Phase 1 (Polarization System)
**Success Metrics**:
- At least 1 catalyst event per entity every 5 steps
- Catalyst variety is implemented (emotional, intellectual, physical, spiritual)
- Catalyst intensity varies based on entity readiness

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "Catalyst provides contrast, limitation, challenge, choice, and growth"
- "The Veil creates the depth required for evolution"
- "Catalyst is essential for polarization choices"

### Implementation Steps

#### Step 3.1: Define Catalyst Varieties

**Files to Modify**:
- `src/simulation_v3/catalyst_system.rs`

**Implementation Requirements**:

```rust
// In CatalystManager

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalystVariety {
    /// Challenges entity's emotional responses
    Emotional,
    /// Challenges entity's understanding
    Intellectual,
    /// Challenges entity's physical experience
    Physical,
    /// Challenges entity's spiritual growth
    Spiritual,
}

#[derive(Debug, Clone)]
pub struct Catalyst {
    pub catalyst_type: CatalystType,
    pub variety: CatalystVariety,
    pub intensity: f64, // 0.0 to 1.0
    pub description: String,
    pub challenge_level: ChallengeLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChallengeLevel {
    Low,    // Easy challenges, minor growth
    Medium, // Moderate challenges, moderate growth
    High,   // Difficult challenges, major growth
}

impl CatalystManager {
    pub fn generate_catalyst(&self, entity: &SubSubLogos) -> Catalyst {
        // Generate catalyst based on entity's readiness
        // - Variety is chosen based on entity's current development
        // - Intensity is based on entity's readiness (more ready = higher intensity)
        // - Challenge level is based on entity's readiness

        let variety = self.select_catalyst_variety(entity);
        let intensity = self.calculate_catalyst_intensity(entity);
        let challenge_level = self.determine_challenge_level(intensity);

        Catalyst {
            catalyst_type: self.select_catalyst_type(variety),
            variety,
            intensity,
            description: self.generate_catalyst_description(variety, intensity),
            challenge_level,
        }
    }

    fn select_catalyst_variety(&self, entity: &SubSubLogos) -> CatalystVariety {
        // Select variety based on entity's current development
        // - If entity is weak in emotional development, generate emotional catalyst
        // - If entity is weak in intellectual development, generate intellectual catalyst
        // - If entity is weak in physical development, generate physical catalyst
        // - If entity is weak in spiritual development, generate spiritual catalyst

        let emotional_strength = entity.archetype_activations[3]; // Experience
        let intellectual_strength = entity.archetype_activations[4]; // Significator
        let physical_strength = entity.archetype_activations[11]; // Body Experience
        let spiritual_strength = entity.archetype_activations[18]; // Spirit Experience

        let weakest = [
            ("emotional", emotional_strength, CatalystVariety::Emotional),
            ("intellectual", intellectual_strength, CatalystVariety::Intellectual),
            ("physical", physical_strength, CatalystVariety::Physical),
            ("spiritual", spiritual_strength, CatalystVariety::Spiritual),
        ]
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

        weakest.2
    }

    fn calculate_catalyst_intensity(&self, entity: &SubSubLogos) -> f64 {
        // Intensity is based on entity's readiness
        // - More polarized = higher intensity
        // - Higher consciousness = higher intensity
        // - More experience = higher intensity

        let polarization_factor = entity.polarization.intensity;
        let consciousness_factor = entity.consciousness_level;
        let experience_factor = entity.experience_accumulation / MAX_EXPERIENCE_FOR_DENSITY;

        (polarization_factor * 0.4
            + consciousness_factor * 0.3
            + experience_factor * 0.3)
            .min(1.0)
            .max(0.1) // Minimum intensity
    }

    fn determine_challenge_level(&self, intensity: f64) -> ChallengeLevel {
        if intensity < 0.33 {
            ChallengeLevel::Low
        } else if intensity < 0.66 {
            ChallengeLevel::Medium
        } else {
            ChallengeLevel::High
        }
    }
}
```

#### Step 3.2: Increase Catalyst Event Frequency

**Files to Modify**:
- `src/simulation_v3/simulation_runner.rs`

**Implementation Requirements**:

```rust
// In SimulationRunner

pub fn run_evolution_step(&mut self, step: u64) -> Result<StepResult, SimulationError> {
    // ... existing code ...

    // Generate catalyst events for each entity
    // Target: At least 1 catalyst event per entity every 5 steps
    for entity_id in self.entity_ids.clone() {
        if step % 5 == 0 {
            // Generate catalyst event every 5 steps
            let entity = self.get_entity(&entity_id)?;
            let catalyst = self.catalyst_manager.generate_catalyst(&entity);

            let event = self.catalyst_manager.process_catalyst_event(
                &entity_id,
                &catalyst,
                &self.free_will_kernel,
            )?;

            self.catalyst_events.push(event);
        }
    }

    // ... rest of existing code ...
}
```

#### Step 3.3: Connect Catalyst to Polarization Choices

**Files to Modify**:
- `src/simulation_v3/catalyst_system.rs`

**Implementation Requirements**:

```rust
// In CatalystManager

pub fn process_catalyst_event(
    &mut self,
    entity_id: &EntityId,
    catalyst: &Catalyst,
    free_will_kernel: &FreeWillKernel,
) -> Result<CatalystEvent, CatalystError> {
    // 1. Generate possibility space based on catalyst
    let possibility_space = free_will_kernel.generate_possibility_space(entity, catalyst);

    // 2. Make conscious selection using Archetype 22
    let selection = free_will_kernel.make_conscious_selection(entity, &possibility_space);

    // 3. Update entity's polarization based on choice
    let entity = self.get_entity_mut(entity_id)?;
    entity.polarization.make_choice(selection.chosen_possibility.outcome);

    // 4. Provide growth based on catalyst intensity and choice
    let growth = self.calculate_growth(catalyst, &selection.chosen_possibility);
    entity.experience_accumulation += growth.experience;
    entity.learning_progress += growth.learning;

    // 5. Record catalyst event
    let event = CatalystEvent {
        entity_id: *entity_id,
        catalyst_type: catalyst.catalyst_type,
        catalyst_variety: catalyst.variety,
        intensity: catalyst.intensity,
        challenge_level: catalyst.challenge_level,
        choice: selection.chosen_possibility.outcome,
        growth,
        timestamp: self.current_step,
    };

    Ok(event)
}

#[derive(Debug, Clone)]
pub struct CatalystGrowth {
    pub experience: f64,
    pub learning: f64,
    pub polarization_change: f64,
}

impl CatalystManager {
    fn calculate_growth(
        &self,
        catalyst: &Catalyst,
        selection: &ConsciousSelection,
    ) -> CatalystGrowth {
        // Growth is based on:
        // - Catalyst intensity (higher intensity = more growth)
        // - Choice alignment (better alignment = more growth)
        // - Selection confidence (higher confidence = more growth)

        let intensity_factor = catalyst.intensity;
        let alignment_factor = selection.selection_confidence;

        let experience = intensity_factor * alignment_factor * 0.1;
        let learning = intensity_factor * alignment_factor * 0.1;
        let polarization_change = intensity_factor * alignment_factor * 0.05;

        CatalystGrowth {
            experience,
            learning,
            polarization_change,
        }
    }
}
```

### Testing Criteria

1. **Catalyst Event Frequency Test**:
   - Given 100 entities running for 100 steps
   - When the simulation completes
   - Then each entity should have received at least 20 catalyst events

2. **Catalyst Variety Test**:
   - Given 100 entities with varying development levels
   - When generating catalysts
   - Then catalyst varieties should be distributed based on entity weaknesses

3. **Catalyst Intensity Test**:
   - Given an entity with 80% polarization, 0.8 consciousness
   - When generating catalyst
   - Then the catalyst intensity should be >0.6

### Success Metrics

- [ ] At least 1 catalyst event per entity every 5 steps
- [ ] Catalyst variety is implemented (emotional, intellectual, physical, spiritual)
- [ ] Catalyst intensity varies based on entity readiness (0.1 to 1.0)
- [ ] Catalyst events are connected to polarization choices
- [ ] Catalyst events provide measurable growth

---

## Phase 4: Veil Enhancement (HIGH PRIORITY)

**Objective**: Enhance veil mechanism to provide proper access control and depth for evolution

**Duration**: 3-4 days
**Dependencies**: Phase 1 (Polarization System)
**Success Metrics**:
- Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th)
- Veil limits access to Time/Space based on transparency
- Veil provides contrast, limitation, challenge, choice, and growth

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "The Veil limits ACCESS to the Oneness side of the spectrum"
- "The Veil creates the illusion of separation"
- "The Veil provides contrast, limitation, challenge, choice, and growth"
- "The Veil is a structural feature of dimensional architecture"

### Implementation Steps

#### Step 4.1: Define Veil Transparency by Density

**Files to Modify**:
- `src/veil/mod.rs`
- `src/spectrum/veil.rs`

**Implementation Requirements**:

```rust
// In Veil

pub struct Veil {
    pub transparency: f64, // 0.0 to 1.0
    pub access_control: VeilAccessControl,
    pub illusion_strength: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone)]
pub struct VeilAccessControl {
    pub time_space_access: f64, // 0.0 to 1.0
    pub holographic_connection_access: f64, // 0.0 to 1.0
    pub higher_consciousness_access: f64, // 0.0 to 1.0
}

impl Veil {
    pub fn from_density(density: Density) -> Self {
        // Veil transparency varies based on density:
        // - 3rd Density: Veil is fully active (0.0 transparency)
        // - 4th Density: Veil starts to thin (0.2 transparency)
        // - 5th Density: Veil mostly dissolved (0.5 transparency)
        // - 6th Density: Veil mostly gone (0.8 transparency)
        // - 7th Density: Veil fully dissolved (1.0 transparency)

        let transparency = match density {
            Density::First => 0.0,
            Density::Second => 0.0,
            Density::Third => 0.0,
            Density::Fourth => 0.2,
            Density::Fifth => 0.5,
            Density::Sixth => 0.8,
            Density::Seventh => 1.0,
            Density::Eighth => 1.0,
        };

        // Access control is based on transparency
        let time_space_access = transparency;
        let holographic_connection_access = transparency * 0.8;
        let higher_consciousness_access = transparency * 0.6;

        // Illusion strength is inverse of transparency
        let illusion_strength = 1.0 - transparency;

        Veil {
            transparency,
            access_control: VeilAccessControl {
                time_space_access,
                holographic_connection_access,
                higher_consciousness_access,
            },
            illusion_strength,
        }
    }

    pub fn can_access_time_space(&self, required_access: f64) -> bool {
        self.access_control.time_space_access >= required_access
    }

    pub fn can_access_holographic_connections(&self, required_access: f64) -> bool {
        self.access_control.holographic_connection_access >= required_access
    }

    pub fn can_access_higher_consciousness(&self, required_access: f64) -> bool {
        self.access_control.higher_consciousness_access >= required_access
    }

    pub fn thin(&mut self, amount: f64) {
        // Thin the veil by increasing transparency
        self.transparency = (self.transparency + amount).min(1.0);
        self.access_control.time_space_access = self.transparency;
        self.access_control.holographic_connection_access = self.transparency * 0.8;
        self.access_control.higher_consciousness_access = self.transparency * 0.6;
        self.illusion_strength = 1.0 - self.transparency;
    }
}
```

#### Step 4.2: Implement Veil Access Control

**Files to Modify**:
- `src/simulation_v3/holographic_field.rs`
- `src/simulation_v3/collective_dynamics.rs`

**Implementation Requirements**:

```rust
// In HolographicFieldManager

pub fn create_holographic_connection(
    &mut self,
    entity_a: &EntityId,
    entity_b: &EntityId,
) -> Result<(), HolographicFieldError> {
    // Check if entities can access holographic connections based on veil
    let entity_a = self.get_entity(entity_a)?;
    let entity_b = self.get_entity(entity_b)?;

    let veil_a = &entity_a.veil;
    let veil_b = &entity_b.veil;

    // Required access for holographic connections
    let required_access = 0.3;

    if !veil_a.can_access_holographic_connections(required_access)
        || !veil_b.can_access_holographic_connections(required_access)
    {
        return Err(HolographicFieldError::VeilBlocksConnection);
    }

    // ... existing code to create connection ...
}
```

#### Step 4.3: Connect Veil to Spectrum Access

**Files to Modify**:
- `src/spectrum/spectrum_access.rs`
- `src/entity_layer7/layer7.rs`

**Implementation Requirements**:

```rust
// In EntitySpectrumAccess

pub fn calculate_from_entity(entity: &SubSubLogos) -> Self {
    let veil = &entity.veil;

    // Space/Time access is influenced by veil transparency
    // More transparent veil = more Time/Space access
    // Less transparent veil = more Space/Time access
    let space_time_ratio = if veil.transparency < 0.5 {
        // Veil is thick, more Space/Time dominant
        1.0 + (1.0 - veil.transparency) * 9.0 // 1.0 to 10.0
    } else {
        // Veil is thin, more balanced or Time/Space dominant
        1.0 / (1.0 + veil.transparency * 9.0) // 1.0 to 0.1
    };

    let time_space_ratio = 1.0 / space_time_ratio;

    EntitySpectrumAccess {
        space_time_ratio,
        time_space_ratio,
        spectrum_position: self.calculate_spectrum_position(space_time_ratio),
        veil_transparency: veil.transparency,
    }
}
```

### Testing Criteria

1. **Veil Transparency Test**:
   - Given entities in different densities
   - When calculating veil transparency
   - Then 3rd density entities should have 0.0 transparency
   - And 7th density entities should have 1.0 transparency

2. **Veil Access Control Test**:
   - Given a 3rd density entity (0.0 transparency)
   - When attempting to access holographic connections
   - Then the access should be blocked

3. **Veil Thinning Test**:
   - Given a 3rd density entity
   - When thinning the veil by 0.2
   - Then the transparency should be 0.2
   *And access control should be updated

### Success Metrics

- [ ] Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th)
- [ ] Veil limits access to Time/Space based on transparency
- [ ] Veil limits access to holographic connections based on transparency
- [ ] Veil limits access to higher consciousness based on transparency
- [ ] Veil provides contrast, limitation, challenge, choice, and growth

---

## Phase 5: Resonance System (MEDIUM PRIORITY)

**Objective**: Implement meaningful resonance calculation to enable collective formation

**Duration**: 4-5 days
**Dependencies**: Phase 1 (Polarization System), Phase 4 (Veil Enhancement)
**Success Metrics**:
- Resonance varies based on entity state (0.0 to 1.0)
- Resonance is not a baseline value (not 0.5000 for all entities)
- Resonance enables collective formation based on shared polarity

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "Resonance measures how well entities align in spectrum configuration, holographic patterns, polarity orientation"
- "Collectives form based on resonance (not proximity)"
- "High resonance leads to collective intelligence emergence"

### Implementation Steps

#### Step 5.1: Redefine Resonance Calculation

**Files to Modify**:
- `src/simulation_v3/holographic_field.rs`

**Implementation Requirements**:

```rust
// In HolographicFieldManager

pub fn calculate_resonance(
    entity_a: &SubSubLogos,
    entity_b: &SubSubLogos,
) -> f64 {
    // Resonance measures alignment in:
    // 1. Spectrum configuration (space/time ratio similarity)
    // 2. Holographic patterns (archetype activation similarity)
    // 3. Polarity orientation (same polarity direction)

    let spectrum_resonance = calculate_spectrum_resonance(entity_a, entity_b);
    let holographic_resonance = calculate_holographic_resonance(entity_a, entity_b);
    let polarity_resonance = calculate_polarity_resonance(entity_a, entity_b);

    // Weighted average
    (spectrum_resonance * 0.3
        + holographic_resonance * 0.4
        + polarity_resonance * 0.3)
        .min(1.0)
}

fn calculate_spectrum_resonance(entity_a: &SubSubLogos, entity_b: &SubSubLogos) -> f64 {
    // Spectrum resonance is based on space/time ratio similarity
    let ratio_a = entity_a.spectrum_access.space_time_ratio;
    let ratio_b = entity_b.spectrum_access.space_time_ratio;

    // Calculate similarity (1.0 = identical, 0.0 = completely different)
    let difference = (ratio_a - ratio_b).abs();
    let max_difference = 10.0; // Maximum possible difference

    1.0 - (difference / max_difference).min(1.0)
}

fn calculate_holographic_resonance(entity_a: &SubSubLogos, entity_b: &SubSubLogos) -> f64 {
    // Holographic resonance is based on archetype activation similarity
    let activations_a = &entity_a.archetype_activations;
    let activations_b = &entity_b.archetype_activations;

    // Calculate cosine similarity
    let dot_product: f64 = activations_a
        .iter()
        .zip(activations_b.iter())
        .map(|(a, b)| a * b)
        .sum();

    let magnitude_a: f64 = activations_a.iter().map(|a| a * a).sum::<f64>().sqrt();
    let magnitude_b: f64 = activations_b.iter().map(|b| b * b).sum::<f64>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

fn calculate_polarity_resonance(entity_a: &SubSubLogos, entity_b: &SubSubLogos) -> f64 {
    // Polarity resonance is based on polarity direction
    // Same polarity = high resonance
    // Different polarity = low resonance
    // Unpolarized = neutral resonance

    match (
        entity_a.polarization.direction,
        entity_b.polarization.direction,
    ) {
        (PolarityDirection::ServiceToOthers, PolarityDirection::ServiceToOthers) => 1.0,
        (PolarityDirection::ServiceToSelf, PolarityDirection::ServiceToSelf) => 1.0,
        (PolarityDirection::Neutral, PolarityDirection::Neutral) => 0.5,
        (PolarityDirection::Neutral, _) => 0.3,
        (_, PolarityDirection::Neutral) => 0.3,
        _ => 0.0, // Different polarities
    }
}
```

#### Step 5.2: Implement Resonance-Based Collective Formation

**Files to Modify**:
- `src/simulation_v3/collective_dynamics.rs`

**Implementation Requirements**:

```rust
// In CollectiveDynamicsManager

pub fn form_collectives(&mut self, entities: &HashMap<EntityId, SubSubLogos>) {
    // Form collectives based on resonance (not proximity)
    // Entities with high resonance and same polarity form collectives

    let resonance_threshold = 0.7; // Minimum resonance for collective formation

    // Clear existing collectives
    self.collectives.clear();

    // Find high resonance pairs
    let entity_ids: Vec<EntityId> = entities.keys().cloned().collect();
    let mut resonance_pairs: Vec<(EntityId, EntityId, f64)> = Vec::new();

    for i in 0..entity_ids.len() {
        for j in (i + 1)..entity_ids.len() {
            let entity_a = entities.get(&entity_ids[i]).unwrap();
            let entity_b = entities.get(&entity_ids[j]).unwrap();

            let resonance = calculate_resonance(entity_a, entity_b);

            if resonance >= resonance_threshold {
                // Check if they have same polarity
                if entity_a.polarization.direction == entity_b.polarization.direction {
                    resonance_pairs.push((entity_ids[i], entity_ids[j], resonance));
                }
            }
        }
    }

    // Form collectives from resonance pairs
    let mut processed_entities: HashSet<EntityId> = HashSet::new();

    for (entity_a, entity_b, resonance) in resonance_pairs {
        if processed_entities.contains(&entity_a) || processed_entities.contains(&entity_b) {
            continue;
        }

        // Create new collective
        let collective_id = EntityId::new(self.collectives.len() as u64);
        let collective = Collective {
            collective_id,
            member_ids: vec![entity_a, entity_b],
            average_resonance: resonance,
            collective_consciousness: 0.0, // Will be calculated later
            polarity: entities.get(&entity_a).unwrap().polarization.direction,
        };

        self.collectives.insert(collective_id, collective);
        processed_entities.insert(entity_a);
        processed_entities.insert(entity_b);
    }

    // Update collective statistics
    self.update_statistics(entities);
}
```

### Testing Criteria

1. **Resonance Calculation Test**:
   - Given two entities with identical spectrum ratios, archetype activations, and polarity
   - When calculating resonance
   - Then the resonance should be 1.0

2. **Resonance Variation Test**:
   - Given 100 entities with varying states
   - When calculating resonance for all pairs
   *Then resonance values should vary (not all 0.5000)

3. **Collective Formation Test**:
   - Given 100 entities with high resonance pairs
   - When forming collectives
   *Then collectives should form based on resonance and polarity

### Success Metrics

- [ ] Resonance varies based on entity state (0.0 to 1.0)
- [ ] Resonance is not a baseline value (not 0.5000 for all entities)
- [ ] Resonance enables collective formation based on shared polarity
- [ ] Collectives form based on resonance (not proximity)
- [ ] High resonance leads to collective intelligence emergence

---

## Phase 6: Collective Consciousness (MEDIUM PRIORITY)

**Objective**: Implement collective consciousness to demonstrate "the whole is more than the sum of parts"

**Duration**: 4-5 days
**Dependencies**: Phase 5 (Resonance System)
**Success Metrics**:
- Collective consciousness level is >0.30 after 100 steps
- Collective consciousness emerges from high resonance collectives
- "The whole is more than the sum of parts" is demonstrated

### Requirements

From `COSMOLOGICAL-ARCHITECTURE.md`:
- "The whole is more than the sum of parts"
- "Collective entities exhibit collective consciousness"
- "Self-organization from complex interactions"

### Implementation Steps

#### Step 6.1: Define Collective Consciousness Calculation

**Files to Modify**:
- `src/simulation_v3/collective_dynamics.rs`

**Implementation Requirements**:

```rust
// In CollectiveDynamicsManager

pub fn calculate_collective_consciousness(
    collective: &Collective,
    entities: &HashMap<EntityId, SubSubLogos>,
) -> f64 {
    // Collective consciousness emerges when:
    // 1. Entities have high resonance
    // 2. Holographic connections synchronize
    // 3. Collective intelligence emerges

    let members: Vec<&SubSubLogos> = collective
        .member_ids
        .iter()
        .filter_map(|id| entities.get(id))
        .collect();

    if members.is_empty() {
        return 0.0;
    }

    // Calculate average member consciousness
    let average_member_consciousness: f64 = members
        .iter()
        .map(|e| e.consciousness_level)
        .sum::<f64>()
        / members.len() as f64;

    // Calculate resonance coherence
    let resonance_coherence = collective.average_resonance;

    // Calculate holographic synchronization
    let holographic_synchronization = calculate_holographic_synchronization(&members);

    // Calculate synergy factor (the "more than sum of parts" factor)
    let synergy_factor = calculate_synergy_factor(&members);

    // Collective consciousness is weighted average of all factors
    (average_member_consciousness * 0.3
        + resonance_coherence * 0.2
        + holographic_synchronization * 0.3
        + synergy_factor * 0.2)
        .min(1.0)
}

fn calculate_holographic_synchronization(members: &[&SubSubLogos]) -> f64 {
    // Calculate how synchronized the holographic patterns are
    // This is based on archetype activation similarity

    if members.len() < 2 {
        return 0.0;
    }

    // Calculate pairwise holographic resonance
    let mut total_resonance = 0.0;
    let mut pair_count = 0;

    for i in 0..members.len() {
        for j in (i + 1)..members.len() {
            let resonance = calculate_holographic_resonance(members[i], members[j]);
            total_resonance += resonance;
            pair_count += 1;
        }
    }

    if pair_count == 0 {
        return 0.0;
    }

    total_resonance / pair_count as f64
}

fn calculate_synergy_factor(members: &[&SubSubLogos]) -> f64 {
    // Calculate the "more than sum of parts" factor
    // This is based on:
    // - Number of members (more members = higher synergy potential)
    // - Diversity of members (more diversity = higher synergy potential)
    // - Alignment of members (more alignment = higher synergy potential)

    if members.len() < 2 {
        return 0.0;
    }

    let member_count_factor = (members.len() as f64).ln() / 10.0; // Logarithmic scaling

    // Calculate diversity (standard deviation of archetype activations)
    let archetype_means: Vec<f64> = (0..22)
        .map(|i| {
            members
                .iter()
                .map(|e| e.archetype_activations[i])
                .sum::<f64>()
                / members.len() as f64
        })
        .collect();

    let archetype_variance: f64 = (0..22)
        .map(|i| {
            members
                .iter()
                .map(|e| {
                    let deviation = e.archetype_activations[i] - archetype_means[i];
                    deviation * deviation
                })
                .sum::<f64>()
                / members.len() as f64
        })
        .sum::<f64>()
        / 22.0;

    let diversity_factor = archetype_variance.sqrt() / 2.0;

    // Calculate alignment (average holographic resonance)
    let alignment_factor = calculate_holographic_synchronization(members);

    // Synergy is weighted average of all factors
    (member_count_factor * 0.3 + diversity_factor * 0.3 + alignment_factor * 0.4).min(1.0)
}
```

#### Step 6.2: Integrate Collective Consciousness into Simulation

**Files to Modify**:
- `src/simulation_v3/simulation_runner.rs`
- `src/simulation_v3/collective_statistics.rs`

**Implementation Requirements**:

```rust
// In SimulationRunner

pub fn run_evolution_step(&mut self, step: u64) -> Result<StepResult, SimulationError> {
    // ... existing code ...

    // Form collectives based on resonance
    self.collective_manager.form_collectives(&self.entities);

    // Calculate collective consciousness for each collective
    for (collective_id, collective) in self.collective_manager.collectives.iter_mut() {
        collective.collective_consciousness =
            self.collective_manager.calculate_collective_consciousness(collective, &self.entities);
    }

    // Update collective statistics
    self.collective_statistics.update(&self.collective_manager.collectives, &self.entities);

    // ... rest of existing code ...
}
```

### Testing Criteria

1. **Collective Consciousness Test**:
   - Given a collective with 5 high resonance members
   - When calculating collective consciousness
   *Then the collective consciousness should be > the average member consciousness

2. **Synergy Factor Test**:
   *Given collectives with varying member counts
   *When calculating synergy factor
   *Then larger collectives should have higher synergy potential

3. **"More Than Sum of Parts" Test**:
   *Given a collective with diverse, aligned members
   *When calculating collective consciousness
   *Then the collective consciousness should demonstrate "more than sum of parts"

### Success Metrics

- [ ] Collective consciousness level is >0.30 after 100 steps
- [ ] Collective consciousness emerges from high resonance collectives
- [ ] "The whole is more than the sum of parts" is demonstrated
- [ ] Collective consciousness varies based on member count, diversity, and alignment
- [ ] Collective consciousness is calculated and tracked

---

## Phase 7: Integration Testing (MEDIUM PRIORITY)

**Objective**: Test all systems together and verify end-to-end functionality

**Duration**: 5-7 days
**Dependencies**: Phases 0-6
**Success Metrics**:
- All tests pass
- Simulation produces expected results
- Architecture alignment is ≥95%

### Testing Plan

#### Test 7.1: Polarization Emergence Test

**Objective**: Verify polarization emerges from Free Will choices

**Test Steps**:
1. Create 100 unpolarized entities
2. Run simulation for 100 steps
3. Verify at least 50 entities are polarized
4. Verify polarization progression is visible

**Success Criteria**:
- [ ] At least 50% of entities are polarized
- [ ] Polarization progression is visible (unpolarized → STO-leaning → polarized)
- [ ] Polarization intensity is tracked (0.0 to 1.0)

#### Test 7.2: Density Progression Test

**Objective**: Verify density progression based on polarization

**Test Steps**:
1. Create 100 polarized entities with varying polarization intensities
2. Run simulation for 100 steps
3. Verify entities with >51% polarization reach 4th density
4. Verify entities with <51% polarization do not reach 4th density

**Success Criteria**:
- [ ] At least 5% of entities reach 4th density
- [ ] No unpolarized entities reach 4th density
- [ ] Density transitions follow polarization thresholds

#### Test 7.3: Collective Formation Test

**Objective**: Verify collectives form based on resonance

**Test Steps**:
1. Create 100 entities with varying archetype activations
2. Run simulation for 100 steps
3. Verify collectives form based on resonance and polarity
4. Verify collectives do not form based on proximity

**Success Criteria**:
- [ ] Collectives form based on resonance
- [ ] Collectives require same polarity orientation
- [ ] Collective consciousness emerges (>0.30)

#### Test 7.4: Architecture Alignment Test

**Objective**: Verify simulation aligns with cosmological architecture

**Test Steps**:
1. Run simulation for 100 steps
2. Verify Three Primal Distortions are implemented
3. Verify "transcend and include" is implemented across all layers
4. Verify Space/Time and Time/Space spectrum is implemented
5. Verify Logos Hierarchy is implemented
6. Verify Density Octave is implemented
7. Verify Holographic Principle is implemented

**Success Criteria**:
- [ ] Three Primal Distortions: ✅ Implemented
- [ ] Transcend and Include: 8/8 stages
- [ ] Space/Time Spectrum: ✅ Implemented
- [ ] Logos Hierarchy: ✅ Implemented
- [ ] Density Octave: ✅ Implemented
- [ ] Holographic Principle: ✅ Implemented
- [ ] Architecture Alignment: ≥95%

#### Test 7.5: Long Simulation Test

**Objective**: Verify simulation stability over long runs

**Test Steps**:
1. Create 256 entities
2. Run simulation for 1000 steps
3. Verify no crashes or errors
4. Verify polarization continues to emerge
5. Verify density progression continues
6. Verify collective consciousness continues to emerge

**Success Criteria**:
- [ ] No crashes or errors
- [ ] Polarization continues to emerge
- [ ] Density progression continues
- [ ] Collective consciousness continues to emerge
- [ ] Execution time is reasonable (<5 minutes)

### Success Metrics

- [ ] All tests pass
- [ ] Simulation produces expected results
- [ ] Architecture alignment is ≥95%
- [ ] No crashes or errors in long simulation runs

---

## Phase 8: Long Simulation Runs (LOW PRIORITY)

**Objective**: Optimize for 10,000+ step simulations and add save/load functionality

**Duration**: 5-7 days
**Dependencies**: Phase 7 (Integration Testing)
**Success Metrics**:
- Simulation can run for 10,000+ steps without crashes
- Save/load functionality is implemented
- Performance is optimized for long runs

### Implementation Steps

#### Step 8.1: Implement Save/Load Functionality

**Files to Create/Modify**:
- `src/simulation_v3/persistence.rs` (new)
- `src/simulation_v3/simulation_runner.rs`

**Implementation Requirements**:

```rust
// src/simulation_v3/persistence.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub entities: HashMap<EntityId, SubSubLogos>,
    pub collectives: HashMap<EntityId, Collective>,
    pub current_step: u64,
    pub simulation_parameters: SimulationParameters,
    pub statistics: SimulationStatistics,
}

pub struct PersistenceManager;

impl PersistenceManager {
    pub fn save_simulation(
        state: &SimulationState,
        path: &Path,
    ) -> Result<(), PersistenceError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, state)?;
        Ok(())
    }

    pub fn load_simulation(path: &Path) -> Result<SimulationState, PersistenceError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let state: SimulationState = serde_json::from_reader(reader)?;
        Ok(state)
    }
}
```

#### Step 8.2: Optimize Performance for Long Runs

**Files to Modify**:
- `src/simulation_v3/simulation_runner.rs`
- `src/simulation_v3/holographic_field.rs`

**Implementation Requirements**:

```rust
// In SimulationRunner

pub fn run_long_simulation(&mut self, steps: u64) -> Result<SimulationResult, SimulationError> {
    let save_interval = 1000; // Save every 1000 steps

    for step in 0..steps {
        self.current_step = step;

        // Run evolution step
        self.run_evolution_step(step)?;

        // Save state periodically
        if step % save_interval == 0 && step > 0 {
            let save_path = format!("simulation_state_step_{}.json", step);
            let state = self.get_simulation_state();
            PersistenceManager::save_simulation(&state, Path::new(&save_path))?;
        }

        // Print progress
        if step % 100 == 0 {
            println!("Progress: {}/{} steps ({}%)", step, steps, (step * 100) / steps);
        }
    }

    Ok(self.get_simulation_result())
}
```

### Testing Criteria

1. **Save/Load Test**:
   - Given a running simulation
   - When saving the state
   - Then the state should be saved to disk
   - When loading the state
   *Then the simulation should resume from the saved state

2. **Long Simulation Test**:
   *Given 256 entities
   *When running simulation for 10,000 steps
   *Then the simulation should complete without crashes
   *And the results should be meaningful

### Success Metrics

- [ ] Simulation can run for 10,000+ steps without crashes
- [ ] Save/load functionality is implemented
- [ ] Performance is optimized for long runs
- [ ] Progress is printed periodically

---

## Phase 9: Visualization Enhancement (LOW PRIORITY)

**Objective**: Enhance visualization with better dashboard and real-time visualization

**Duration**: 3-4 days
**Dependencies**: Phase 7 (Integration Testing)
**Success Metrics**:
- Better dashboard with more metrics
- Real-time visualization of simulation state
- Interactive exploration of simulation results

### Implementation Steps

#### Step 9.1: Enhance Dashboard

**Files to Modify**:
- `src/simulation_v3/visualization.rs`

**Implementation Requirements**:

```rust
// In RealTimeDashboard

pub fn display_comprehensive_dashboard(&self, statistics: &SimulationStatistics) {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║          COMPREHENSIVE SIMULATION DASHBOARD                         ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    self.display_polarization_dashboard(statistics);
    self.display_density_dashboard(statistics);
    self.display_collective_dashboard(statistics);
    self.display_spectrum_dashboard(statistics);
    self.display_attractor_field_dashboard(statistics);
    self.display_catalyst_dashboard(statistics);
    self.display_veil_dashboard(statistics);
}

fn display_polarization_dashboard(&self, statistics: &SimulationStatistics) {
    println!("📊 POLARIZATION DASHBOARD:");
    println!("  Unpolarized: {} ({:.1}%)",
        statistics.polarization_distribution.unpolarized,
        statistics.polarization_distribution.unpolarized_percentage);
    println!("  Positive (STO): {} ({:.1}%)",
        statistics.polarization_distribution.positive,
        statistics.polarization_distribution.positive_percentage);
    println!("  Negative (STS): {} ({:.1}%)",
        statistics.polarization_distribution.negative,
        statistics.polarization_distribution.negative_percentage);
    println!("  Average Polarization Intensity: {:.2}", statistics.average_polarization_intensity);
    println!();
}
```

#### Step 9.2: Implement Interactive Exploration

**Files to Create/Modify**:
- `src/simulation_v3/interactive_explorer.rs` (new)

**Implementation Requirements**:

```rust
// src/simulation_v3/interactive_explorer.rs

pub struct InteractiveExplorer {
    simulation_runner: SimulationRunner,
}

impl InteractiveExplorer {
    pub fn new(simulation_runner: SimulationRunner) -> Self {
        InteractiveExplorer { simulation_runner }
    }

    pub fn run(&mut self) {
        loop {
            println!("\n=== Interactive Simulation Explorer ===");
            println!("1. View entity details");
            println!("2. View collective details");
            println!("3. View spectrum distribution");
            println!("4. View polarization distribution");
            println!("5. Run simulation for N steps");
            println!("6. Save simulation state");
            println!("7. Load simulation state");
            println!("8. Exit");

            let choice = self.get_user_input("Enter choice: ");

            match choice.as_str() {
                "1" => self.view_entity_details(),
                "2" => self.view_collective_details(),
                "3" => self.view_spectrum_distribution(),
                "4" => self.view_polarization_distribution(),
                "5" => self.run_simulation_steps(),
                "6" => self.save_simulation_state(),
                "7" => self.load_simulation_state(),
                "8" => break,
                _ => println!("Invalid choice"),
            }
        }
    }

    fn view_entity_details(&self) {
        let entity_id = self.get_user_input("Enter entity ID: ");
        let entity_id = entity_id.parse::<u64>().unwrap();
        let entity_id = EntityId::new(entity_id);

        if let Some(entity) = self.simulation_runner.get_entity(&entity_id) {
            println!("\n=== Entity Details ===");
            println!("Entity ID: {:?}", entity_id);
            println!("Entity Type: {:?}", entity.entity_type);
            println!("Current Density: {:?}", entity.current_density);
            println!("Polarization State: {:?}", entity.polarization.state);
            println!("Polarization Intensity: {:.2}", entity.polarization.intensity);
            println!("Consciousness Level: {:.2}", entity.consciousness_level);
            println!("Experience Accumulation: {:.2}", entity.experience_accumulation);
            println!("Veil Transparency: {:.2}", entity.veil.transparency);
            println!("Space/Time Ratio: {:.2}", entity.spectrum_access.space_time_ratio);
            println!("Time/Space Ratio: {:.2}", entity.spectrum_access.time_space_ratio);
        } else {
            println!("Entity not found");
        }
    }
}
```

### Testing Criteria

1. **Dashboard Test**:
   - Given a running simulation
   *When displaying the dashboard
   *Then all metrics should be displayed correctly

2. **Interactive Exploration Test**:
   *Given an interactive explorer
   *When exploring entity details
   *Then entity details should be displayed correctly

### Success Metrics

- [ ] Better dashboard with more metrics
- [ ] Real-time visualization of simulation state
- [ ] Interactive exploration of simulation results
- [ ] All visualizations are accurate and informative

---

## Summary and Timeline

### Overall Timeline

| Phase | Duration | Dependencies | Priority | Expected Impact |
|-------|----------|--------------|----------|-----------------|
| Phase 0: Foundation Reset | 5-7 days | None | CRITICAL | Fixes Free Will and Archetype 22 (the kernel) |
| Phase 1: Polarization System | 5-7 days | Phase 0 | CRITICAL | Enables density transitions |
| Phase 2: Attractor-Fields | 4-5 days | Phase 1 | HIGH | Adds evolutionary pull |
| Phase 3: Catalyst Enhancement | 3-4 days | Phase 1 | HIGH | Provides growth opportunities |
| Phase 4: Veil Enhancement | 3-4 days | Phase 1 | HIGH | Adds depth to evolution |
| Phase 5: Resonance System | 4-5 days | Phase 1, 4 | MEDIUM | Enables collective formation |
| Phase 6: Collective Consciousness | 4-5 days | Phase 5 | MEDIUM | Demonstrates holographic principle |
| Phase 7: Integration Testing | 5-7 days | Phases 0-6 | MEDIUM | Verifies end-to-end functionality |
| Phase 8: Long Simulation Runs | 5-7 days | Phase 7 | LOW | Enables 10,000+ step simulations |
| Phase 9: Visualization Enhancement | 3-4 days | Phase 7 | LOW | Improves user experience |

**Total Duration**: 41-56 days (approximately 6-8 weeks)

### Critical Path

The critical path for the refactor is:
1. Phase 0 (Foundation Reset) - MUST be done first
2. Phase 1 (Polarization System) - MUST be done second
3. Phase 7 (Integration Testing) - MUST verify all previous phases

Phases 2-6 can be done in parallel after Phase 1 is complete.

### Expected Results

After completing all phases, the simulation should:

1. **Polarization**: At least 50% of entities polarized after 100 steps
2. **Density Progression**: At least 5% of entities reach 4th density after 100 steps
3. **Collective Consciousness**: At least 0.30 collective consciousness level after 100 steps
4. **Architecture Alignment**: At least 95% after all phases complete
5. **Code Quality**: Zero warnings, no deprecated types
6. **Performance**: <30 seconds for 128 entities, 100 steps
7. **Long Simulation**: Can run for 10,000+ steps without crashes
8. **User Experience**: Better dashboard and interactive exploration

### Key Architectural Principles to Maintain

Throughout the refactor, maintain these key architectural principles:

1. **"Transcend and Include"**: Each layer includes all previous development and transcends by adding new capabilities
2. **Free Will**: Non-deterministic selection from possibility space (not random, not predetermined)
3. **Archetype 22**: The kernel of the simulation, creates polarity
4. **Holographic Principle**: Each entity contains the whole
5. **Logos Hierarchy**: Galactic → Solar → Individual
6. **Spectrum Configuration**: Pre-existing patterns guide physical formation
7. **Consciousness-First Cosmology**: Spectrum patterns exist before physical matter

### Risk Mitigation

**Risk 1: Polarization Still Not Emerging After Phase 1**
- **Mitigation**: Increase catalyst event frequency, adjust Free Will choice mechanism
- **Fallback**: Simplify polarization model temporarily, iterate later

**Risk 2: Performance Degradation After Enhancements**
- **Mitigation**: Profile performance bottlenecks, optimize critical paths
- **Fallback**: Reduce entity count or step count for testing

**Risk 3: Code Complexity Becomes Unmanageable**
- **Mitigation**: Maintain clear module boundaries, add comprehensive tests
- **Fallback**: Refactor into smaller, more focused modules

**Risk 4: Timeline Slippage**
- **Mitigation**: Prioritize critical phases (0, 1, 7), defer low-priority phases (8, 9)
- **Fallback**: Reduce scope of individual phases, focus on core functionality

---

## Conclusion

This refactor plan provides a comprehensive roadmap to bridge the gap from the current simulation state (84.62% architecture alignment, non-functional evolutionary mechanisms) to the desired state (95%+ architecture alignment, fully functional evolutionary mechanisms).

The plan is structured in 9 phases, with clear priorities, dependencies, success criteria, and testing requirements. By following this plan, the simulation will:
- Properly implement Free Will and Archetype 22 (the kernel)
- Enable meaningful polarization progression
- Add evolutionary pull through attractor-fields
- Provide growth opportunities through catalyst enhancement
- Add depth to evolution through veil enhancement
- Enable collective formation through resonance
- Demonstrate the holographic principle through collective consciousness
- Support long simulation runs for deeper exploration
- Provide better visualization and user experience

The refactor is expected to take 41-56 days (6-8 weeks) and will result in a simulation that accurately models the Law of One cosmological principles as specified in `COSMOLOGICAL-ARCHITECTURE.md`.