// Density Octave Implementation
//
// From COSMOLOGICAL-ARCHITECTURE.md (CORRECTED):
// "Densities are hierarchical material substrates, not individual evolutionary stages"
//
// Phase 0 Refactor: This module now implements:
// 1. Collective system emergence (when densities emerge in the collective system)
// 2. Individual entity progression (entities progress within their density)
// 3. Sub-density levels for 1st and 2nd Density
// 4. Characteristics of each density
// 5. Density transition mechanisms for individual entities (within their density)
//
// IMPORTANT: The entire concept of "density transition" for individual entities
// is cosmologically incorrect. Individual entities DO NOT change density.
// They progress WITHIN their density.
//
// The collective system develops new densities through emergence:
// - 1st Density materials create planets → 2nd Density life emerges
// - 2nd Density life develops → 3rd Density consciousness emerges

use crate::archetypes::cycles::ArchetypeChoice;
use crate::attractors::DensityAttractorFields;
use crate::entity_layer7::layer7::EntityState;
use crate::evolution_chain::EvolutionStep;

/// Density Octave
///
/// Phase 0 Refactor: Tracks BOTH collective system emergence and individual entity progression.
///
/// From COSMOLOGICAL-ARCHITECTURE.md (CORRECTED):
/// "Densities are hierarchical material substrates, not individual evolutionary stages"
///
/// This structure now tracks:
/// 1. **Collective System Emergence**: When densities emerge in the collective system
///    - When does 2nd Density emerge in the collective system?
///    - When does 3rd Density emerge in the collective system?
/// 2. **Individual Entity Progression**: Entities at each scale progress within their density
///    - 1st Density entities: Progress through physical interaction
///    - 2nd Density entities: Progress through growth and survival
///    - 3rd Density entities: Progress through polarization and choice
#[derive(Debug, Clone)]
pub struct DensityOctave {
    /// Current density level of the COLLECTIVE SYSTEM (not individual entities)
    pub collective_density: Density,

    /// Complete octave structure
    pub octave_structure: OctaveStructure,

    /// Collective system emergence progress
    pub collective_emergence: CollectiveEmergence,

    /// Density transition state of the collective system
    pub transition_state: TransitionState,

    /// Attractor-fields for each density transition (Phase 2)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each stage creates attractor-fields that pull toward the next level"
    /// "Attractor-fields are 'spiritual gravity' that pulls the previous stage toward the next"
    pub attractor_fields: DensityAttractorFields,
}

impl DensityOctave {
    /// Create a new density octave starting at 1st density (collective system)
    ///
    /// Phase 0 Refactor: Tracks collective system emergence, not individual entity progression.
    pub fn new() -> Self {
        DensityOctave {
            collective_density: Density::First(Density1SubLevel::Quantum),
            octave_structure: OctaveStructure::new(),
            collective_emergence: CollectiveEmergence::new(),
            transition_state: TransitionState::Stable,
            attractor_fields: DensityAttractorFields::new(),
        }
    }

    /// Create a new density octave starting at a specific density
    pub fn with_initial_density(initial_density: Density) -> Self {
        DensityOctave {
            collective_density: initial_density,
            octave_structure: OctaveStructure::new(),
            collective_emergence: CollectiveEmergence::new(),
            transition_state: TransitionState::Stable,
            attractor_fields: DensityAttractorFields::new(),
        }
    }

    /// Get current density characteristics of the collective system
    pub fn current_density_characteristics(&self) -> DensityCharacteristics {
        match &self.collective_density {
            Density::First(sub_level) => DensityCharacteristics::first_density(*sub_level),
            Density::Second(sub_level) => DensityCharacteristics::second_density(*sub_level),
            Density::Third => DensityCharacteristics::third_density(),
            Density::Fourth => DensityCharacteristics::fourth_density(),
            Density::Fifth => DensityCharacteristics::fifth_density(),
            Density::Sixth => DensityCharacteristics::sixth_density(),
            Density::Seventh => DensityCharacteristics::seventh_density(),
            Density::Eighth => DensityCharacteristics::eighth_density(),
        }
    }

    /// Advance collective system to next density (emergence)
    ///
    /// Phase 0 Refactor: This now tracks collective system emergence, not individual entity progression.
    /// Individual entities DO NOT change density - they progress within their density.
    ///
    /// Collective emergence thresholds:
    /// - 1st → 2nd: ~25% collective emergence progress
    /// - 2nd → 3rd: ~50% collective emergence progress
    /// - 3rd → 4th: ~75% collective emergence progress
    /// - 4th → 5th: ~85% collective emergence progress
    /// - 5th → 6th: ~95% collective emergence progress
    /// - 6th → 7th: ~99% collective emergence progress
    /// - 7th → 8th: ~100% collective emergence progress
    pub fn advance_collective_emergence(&mut self) -> DensityTransitionResult {
        match &self.collective_density {
            Density::First(sub_level) => self.advance_first_density_emergence(*sub_level),
            Density::Second(sub_level) => self.advance_second_density_emergence(*sub_level),
            Density::Third => self.advance_to_fourth_density_emergence(),
            Density::Fourth => self.advance_to_fifth_density_emergence(),
            Density::Fifth => self.advance_to_sixth_density_emergence(),
            Density::Sixth => self.advance_to_seventh_density_emergence(),
            Density::Seventh => self.advance_to_eighth_density_emergence(),
            Density::Eighth => DensityTransitionResult::AlreadyComplete,
        }
    }

    /// Advance within 1st density (collective system)
    fn advance_first_density_emergence(
        &mut self,
        sub_level: Density1SubLevel,
    ) -> DensityTransitionResult {
        match sub_level {
            Density1SubLevel::Quantum => {
                if self.collective_emergence.progress >= 0.06 {
                    self.collective_density = Density::First(Density1SubLevel::Atomic);
                    DensityTransitionResult::Advanced {
                        from_density: "1st Density - Quantum Realm".to_string(),
                        to_density: "1st Density - Atomic Realm".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "1st Density - Quantum Realm".to_string(),
                        required_progress: 6.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
            Density1SubLevel::Atomic => {
                if self.collective_emergence.progress >= 0.12 {
                    self.collective_density = Density::First(Density1SubLevel::Molecular);
                    DensityTransitionResult::Advanced {
                        from_density: "1st Density - Atomic Realm".to_string(),
                        to_density: "1st Density - Molecular Realm".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "1st Density - Atomic Realm".to_string(),
                        required_progress: 12.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
            Density1SubLevel::Molecular => {
                if self.collective_emergence.progress >= 0.18 {
                    self.collective_density = Density::First(Density1SubLevel::Planetary);
                    DensityTransitionResult::Advanced {
                        from_density: "1st Density - Molecular Realm".to_string(),
                        to_density: "1st Density - Planetary Realm".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "1st Density - Molecular Realm".to_string(),
                        required_progress: 18.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
            Density1SubLevel::Planetary => {
                if self.collective_emergence.progress >= 0.25 {
                    self.collective_density = Density::Second(Density2SubLevel::Cellular);
                    DensityTransitionResult::DensityTransition {
                        from_density: "1st Density".to_string(),
                        to_density: "2nd Density".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "1st Density - Planetary Realm".to_string(),
                        required_progress: 25.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
        }
    }

    /// Advance within 2nd density (collective system)
    fn advance_second_density_emergence(
        &mut self,
        sub_level: Density2SubLevel,
    ) -> DensityTransitionResult {
        match sub_level {
            Density2SubLevel::Cellular => {
                if self.collective_emergence.progress >= 0.31 {
                    self.collective_density = Density::Second(Density2SubLevel::SimpleLife);
                    DensityTransitionResult::Advanced {
                        from_density: "2nd Density - Cellular Realm".to_string(),
                        to_density: "2nd Density - Simple Life Realm".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "2nd Density - Cellular Realm".to_string(),
                        required_progress: 31.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
            Density2SubLevel::SimpleLife => {
                if self.collective_emergence.progress >= 0.40 {
                    self.collective_density = Density::Second(Density2SubLevel::ComplexLife);
                    DensityTransitionResult::Advanced {
                        from_density: "2nd Density - Simple Life Realm".to_string(),
                        to_density: "2nd Density - Complex Life Realm".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "2nd Density - Simple Life Realm".to_string(),
                        required_progress: 40.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
            Density2SubLevel::ComplexLife => {
                if self.collective_emergence.progress >= 0.50 {
                    self.collective_density = Density::Third;
                    DensityTransitionResult::DensityTransition {
                        from_density: "2nd Density".to_string(),
                        to_density: "3rd Density".to_string(),
                        progress_percentage: self.collective_emergence.calculate_percentage(),
                    }
                } else {
                    DensityTransitionResult::NotReady {
                        current_density: "2nd Density - Complex Life Realm".to_string(),
                        required_progress: 50.0,
                        current_progress: self.collective_emergence.calculate_percentage(),
                    }
                }
            }
        }
    }

    /// Advance from 3rd to 4th density (collective system)
    fn advance_to_fourth_density_emergence(&mut self) -> DensityTransitionResult {
        if self.collective_emergence.progress >= 0.75 {
            self.collective_density = Density::Fourth;
            self.transition_state = TransitionState::Transitioning;
            DensityTransitionResult::DensityTransition {
                from_density: "3rd Density".to_string(),
                to_density: "4th Density".to_string(),
                progress_percentage: self.collective_emergence.calculate_percentage(),
            }
        } else {
            DensityTransitionResult::NotReady {
                current_density: "3rd Density".to_string(),
                required_progress: 75.0,
                current_progress: self.collective_emergence.calculate_percentage(),
            }
        }
    }

    /// Advance from 4th to 5th density (collective system)
    fn advance_to_fifth_density_emergence(&mut self) -> DensityTransitionResult {
        if self.collective_emergence.progress >= 0.85 {
            self.collective_density = Density::Fifth;
            self.transition_state = TransitionState::Transitioning;
            DensityTransitionResult::DensityTransition {
                from_density: "4th Density".to_string(),
                to_density: "5th Density".to_string(),
                progress_percentage: self.collective_emergence.calculate_percentage(),
            }
        } else {
            DensityTransitionResult::NotReady {
                current_density: "4th Density".to_string(),
                required_progress: 85.0,
                current_progress: self.collective_emergence.calculate_percentage(),
            }
        }
    }

    /// Advance from 5th to 6th density (collective system)
    fn advance_to_sixth_density_emergence(&mut self) -> DensityTransitionResult {
        if self.collective_emergence.progress >= 0.95 {
            self.collective_density = Density::Sixth;
            self.transition_state = TransitionState::Transitioning;
            DensityTransitionResult::DensityTransition {
                from_density: "5th Density".to_string(),
                to_density: "6th Density".to_string(),
                progress_percentage: self.collective_emergence.calculate_percentage(),
            }
        } else {
            DensityTransitionResult::NotReady {
                current_density: "5th Density".to_string(),
                required_progress: 95.0,
                current_progress: self.collective_emergence.calculate_percentage(),
            }
        }
    }

    /// Advance from 6th to 7th density (collective system)
    fn advance_to_seventh_density_emergence(&mut self) -> DensityTransitionResult {
        if self.collective_emergence.progress >= 0.99 {
            self.collective_density = Density::Seventh;
            self.transition_state = TransitionState::Transitioning;
            DensityTransitionResult::DensityTransition {
                from_density: "6th Density".to_string(),
                to_density: "7th Density".to_string(),
                progress_percentage: self.collective_emergence.calculate_percentage(),
            }
        } else {
            DensityTransitionResult::NotReady {
                current_density: "6th Density".to_string(),
                required_progress: 99.0,
                current_progress: self.collective_emergence.calculate_percentage(),
            }
        }
    }

    /// Advance from 7th to 8th density (collective system)
    fn advance_to_eighth_density_emergence(&mut self) -> DensityTransitionResult {
        if self.collective_emergence.progress >= 1.0 {
            self.collective_density = Density::Eighth;
            self.transition_state = TransitionState::Complete;
            DensityTransitionResult::DensityTransition {
                from_density: "7th Density".to_string(),
                to_density: "8th Density".to_string(),
                progress_percentage: self.collective_emergence.calculate_percentage(),
            }
        } else {
            DensityTransitionResult::NotReady {
                current_density: "7th Density".to_string(),
                required_progress: 100.0,
                current_progress: self.collective_emergence.calculate_percentage(),
            }
        }
    }

    /// Update collective emergence progress
    ///
    /// Phase 0 Refactor: Collective emergence is tracked based on the overall state
    /// of the collective system, not individual entity progression.
    pub fn update_collective_emergence(&mut self, entity_state: &EntityState) {
        self.collective_emergence.update(entity_state);
    }

    /// Check if ready for density transition (collective system)
    pub fn check_collective_emergence_readiness(&self) -> TransitionReadiness {
        match &self.collective_density {
            Density::First(Density1SubLevel::Quantum) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.06,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 6.0,
                next_density: "1st Density - Atomic Realm".to_string(),
            },
            Density::First(Density1SubLevel::Atomic) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.12,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 12.0,
                next_density: "1st Density - Molecular Realm".to_string(),
            },
            Density::First(Density1SubLevel::Molecular) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.18,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 18.0,
                next_density: "1st Density - Planetary Realm".to_string(),
            },
            Density::First(Density1SubLevel::Planetary) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.25,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 25.0,
                next_density: "2nd Density".to_string(),
            },
            Density::Second(Density2SubLevel::Cellular) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.31,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 31.0,
                next_density: "2nd Density - Simple Life Realm".to_string(),
            },
            Density::Second(Density2SubLevel::SimpleLife) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.40,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 40.0,
                next_density: "2nd Density - Complex Life Realm".to_string(),
            },
            Density::Second(Density2SubLevel::ComplexLife) => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.50,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 50.0,
                next_density: "3rd Density".to_string(),
            },
            Density::Third => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.75,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 75.0,
                next_density: "4th Density".to_string(),
            },
            Density::Fourth => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.85,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 85.0,
                next_density: "5th Density".to_string(),
            },
            Density::Fifth => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.95,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 95.0,
                next_density: "6th Density".to_string(),
            },
            Density::Sixth => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 0.99,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 99.0,
                next_density: "7th Density".to_string(),
            },
            Density::Seventh => TransitionReadiness {
                is_ready: self.collective_emergence.progress >= 1.0,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 100.0,
                next_density: "8th Density".to_string(),
            },
            Density::Eighth => TransitionReadiness {
                is_ready: false,
                current_progress: self.collective_emergence.calculate_percentage(),
                required_progress: 100.0,
                next_density: "Complete".to_string(),
            },
        }
    }

    /// Get the current collective emergence progress
    ///
    /// Phase 1: Independent Evolution
    ///
    /// Returns the current progress through the density octave (0.0 to 1.0).
    /// This is used to determine when entities are ready to make evolutionary choices.
    pub fn get_progress(&self) -> f64 {
        self.collective_emergence.progress
    }
}

/// Density (1st through 8th)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density {
    First(Density1SubLevel),
    Second(Density2SubLevel),
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

/// 1st Density sub-levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density1SubLevel {
    Quantum,   // Quantum particles and fields
    Atomic,    // Atoms and galaxies
    Molecular, // Molecules and planets
    Planetary, // Planetary structures and Gaia consciousness precursors
}

/// 2nd Density sub-levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Density2SubLevel {
    Cellular,    // Prokaryotes and simple life
    SimpleLife,  // Plants and simple animals
    ComplexLife, // Eukaryotes and complex animals
}

/// Sub-Density - All sub-densities across the density octave
///
/// This enum consolidates all sub-densities from different density levels
/// for use in validations and holographic principle demonstrations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubDensity {
    // 1st Density sub-densities
    QuantumRealm,
    AtomicRealm,
    MolecularRealm,
    PlanetaryRealm,

    // 2nd Density sub-densities
    CellularRealm,
    SimpleLifeRealm,
    ComplexLifeRealm,

    // 3rd Density sub-densities
    ConsciousLifeRealm,
    SocietalRealm,
}

/// Valve State - The mechanism regulating flow between Body and Spirit
///
/// Mind acts as a valve regulating the flow between Body (up-pouring)
/// and Spirit (in-pouring).
///
/// From REFACTOR_ROADMAP_V3.md Phase 7:
/// "Mind acts as a valve regulating the flow between Body and Spirit"
/// "When Mind is balanced, the valve is open and Spirit can flow"
///
/// Migrated from: src/evolution_chain.rs (Migration 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValveState {
    /// Open - Mind is balanced, full access to Spirit
    Open,

    /// Restricted - Mind has some blockages, limited Spirit access
    Restricted,

    /// Closed - Mind is blocked, no Spirit access
    Closed,
}

/// Spiral Pattern - Non-linear development pattern
///
/// Evolution follows a spiral pattern where entities can make leaps
/// in consciousness via Free Will choices, even if lower centers
/// are not perfectly balanced.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Entities can make non-linear leaps in consciousness through Free Will"
/// "The spiral pattern represents the non-linear nature of consciousness expansion"
///
/// Migrated from: src/evolution_chain.rs (Migration 3)
#[derive(Debug, Clone)]
pub struct SpiralPattern {
    /// Spiral level (higher = more advanced)
    pub level: usize,

    /// Non-linear leap capability (0.0 to 1.0)
    pub leap_capacity: f64,

    /// Number of leaps made
    pub leap_count: usize,

    /// Leaps history
    pub leap_history: Vec<SpiralLeap>,
}

/// A Spiral Leap - Non-linear consciousness expansion
///
/// Represents a leap in consciousness made via Free Will choice.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Spiral leaps occur when entities make Free Will choices that activate
/// higher levels of holographic architecture"
///
/// Migrated from: src/evolution_chain.rs (Migration 3)
#[derive(Debug, Clone)]
pub struct SpiralLeap {
    /// From which step
    pub from_step: EvolutionStep,

    /// To which step
    pub to_step: EvolutionStep,

    /// Leap intensity (0.0 to 1.0)
    pub intensity: f64,

    /// Choice that triggered the leap
    pub choice: ArchetypeChoice,
}

impl SpiralPattern {
    /// Create a new Spiral Pattern
    pub fn new() -> Self {
        Self {
            level: 1,
            leap_capacity: 0.5,
            leap_count: 0,
            leap_history: Vec::new(),
        }
    }

    /// Apply a leap to the spiral pattern
    pub fn apply_leap(&mut self, leap: &SpiralLeap) {
        self.leap_count += 1;
        self.leap_history.push(leap.clone());

        // Increase leap capacity with each leap
        self.leap_capacity = (self.leap_capacity + leap.intensity * 0.1).min(1.0);

        // Increase level - use discriminant value
        self.level = match leap.to_step {
            crate::evolution_chain::EvolutionStep::RedRay => 1,
            crate::evolution_chain::EvolutionStep::OrangeRay => 2,
            crate::evolution_chain::EvolutionStep::YellowRay => 3,
            crate::evolution_chain::EvolutionStep::GreenRay => 4,
            crate::evolution_chain::EvolutionStep::BlueRay => 5,
            crate::evolution_chain::EvolutionStep::IndigoRay => 6,
            crate::evolution_chain::EvolutionStep::VioletRay => 7,
        };
    }
}

/// Octave structure
#[derive(Debug, Clone)]
pub struct OctaveStructure {
    /// All 8 densities
    pub densities: Vec<Density>,

    /// Density transition thresholds
    pub transition_thresholds: Vec<f64>,
}

impl OctaveStructure {
    fn new() -> Self {
        OctaveStructure {
            densities: vec![
                Density::First(Density1SubLevel::Quantum),
                Density::Second(Density2SubLevel::Cellular),
                Density::Third,
                Density::Fourth,
                Density::Fifth,
                Density::Sixth,
                Density::Seventh,
                Density::Eighth,
            ],
            transition_thresholds: vec![
                0.25, // 1st → 2nd
                0.50, // 2nd → 3rd
                0.75, // 3rd → 4th
                0.85, // 4th → 5th
                0.95, // 5th → 6th
                0.99, // 6th → 7th
                1.00, // 7th → 8th
            ],
        }
    }
}

/// Collective System Emergence
///
/// Phase 0 Refactor: Tracks when densities emerge in the collective system.
///
/// From COSMOLOGICAL-ARCHITECTURE.md (CORRECTED):
/// "Densities are hierarchical material substrates, not individual evolutionary stages"
///
/// Collective emergence is about the emergence of new levels of complexity in the
/// COLLECTIVE system, not about individual entities changing density.
///
/// Example:
/// - 1st Density materials create planets → 2nd Density life emerges
/// - 2nd Density life develops → 3rd Density consciousness emerges
#[derive(Debug, Clone)]
pub struct CollectiveEmergence {
    /// Progress through the collective emergence (0.0 to 1.0)
    pub progress: f64,

    /// Experience accumulation across the collective system
    pub experience_accumulation: f64,

    /// Learning progress across the collective system
    pub learning_progress: f64,

    /// Consciousness expansion across the collective system
    pub consciousness_expansion: f64,
}

impl CollectiveEmergence {
    fn new() -> Self {
        CollectiveEmergence {
            progress: 0.0,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
            consciousness_expansion: 0.0,
        }
    }

    fn update(&mut self, entity_state: &EntityState) {
        self.experience_accumulation = entity_state.experience_accumulation;
        self.learning_progress = entity_state.learning_progress;
        self.consciousness_expansion = entity_state.consciousness_level;

        // Calculate collective emergence progress based on TOTAL accumulated experience
        // This represents the collective system's journey through the entire octave
        // Progress is capped at 1.0 (100%)
        const MAX_EXPERIENCE_FOR_COMPLETE_OCTAVE: f64 = 250.0;
        self.progress =
            (self.experience_accumulation / MAX_EXPERIENCE_FOR_COMPLETE_OCTAVE).min(1.0);
    }

    fn calculate_percentage(&self) -> f64 {
        self.progress * 100.0
    }
}

/// Transition state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionState {
    Stable,
    Transitioning,
    Complete,
}

/// Density transition result
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionResult {
    Advanced {
        from_density: String,
        to_density: String,
        progress_percentage: f64,
    },
    DensityTransition {
        from_density: String,
        to_density: String,
        progress_percentage: f64,
    },
    NotReady {
        current_density: String,
        required_progress: f64,
        current_progress: f64,
    },
    AlreadyComplete,
}

/// Transition readiness
#[derive(Debug, Clone)]
pub struct TransitionReadiness {
    pub is_ready: bool,
    pub current_progress: f64,
    pub required_progress: f64,
    pub next_density: String,
}

/// Density characteristics
#[derive(Debug, Clone)]
pub struct DensityCharacteristics {
    pub density_name: String,
    pub ray_color: String,
    pub primary_focus: String,
    pub consciousness_level: f64,
    pub veil_transparency: f64,
    pub spectrum_access: f64,
    pub characteristics: Vec<String>,
}

/// Density Attractor Field
///
/// Phase 5: Organic Evolution
///
/// Attractor fields guide entities toward specific density levels by creating
/// "gravity wells" in the probability landscape. Each density level has an
/// attractor field that influences transition probability.
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 5:
/// "Implement attractor fields for each density level"
#[derive(Debug, Clone)]
pub struct DensityAttractorField {
    /// The density level this attractor field is associated with
    pub density_level: Density,

    /// How strongly the density pulls entities toward it (0.1 to 1.0)
    /// Higher values create stronger gravitational pull toward this density
    pub attractor_strength: f64,

    /// The range of experience levels where the attractor is active (0.0 to 1.0)
    /// Entities within this experience range feel the strongest pull
    pub attractor_range: f64,

    /// The minimum progress required to feel the attractor (0.0 to 1.0)
    /// Entities below this threshold are not influenced by the attractor
    pub attractor_threshold: f64,

    /// The resonant frequency of this density level
    /// Entities with similar resonant frequencies are more strongly attracted
    pub resonance_frequency: f64,

    /// The polarity bias of this attractor (STO positive, STS negative)
    /// Entities with similar polarity bias are more strongly attracted
    pub polarity_bias: f64,
}

impl DensityAttractorField {
    /// Create a new density attractor field
    pub fn new(
        density_level: Density,
        attractor_strength: f64,
        attractor_range: f64,
        attractor_threshold: f64,
        resonance_frequency: f64,
        polarity_bias: f64,
    ) -> Self {
        DensityAttractorField {
            density_level,
            attractor_strength,
            attractor_range,
            attractor_threshold,
            resonance_frequency,
            polarity_bias,
        }
    }

    /// Calculate the attractor influence on transition probability
    ///
    /// Returns a multiplier (0.0 to 2.0) that modifies the base transition probability.
    /// Higher values indicate stronger pull toward this density.
    pub fn calculate_attractor_influence(
        &self,
        entity_progress: f64,
        entity_resonance: f64,
        entity_polarity_bias: f64,
    ) -> f64 {
        // Check if entity is within attractor range
        if entity_progress < self.attractor_threshold {
            // Entity below threshold - no influence
            return 1.0;
        }

        // Calculate distance from attractor center
        // Attractor is strongest at the center of its range
        let distance_from_center =
            (entity_progress - (self.attractor_threshold + self.attractor_range / 2.0)).abs();
        let normalized_distance = distance_from_center / (self.attractor_range / 2.0);

        // Calculate resonance match (0.0 to 1.0)
        let resonance_match = 1.0 - (entity_resonance - self.resonance_frequency).abs();

        // Calculate polarity match (0.0 to 1.0)
        let polarity_match = 1.0 - (entity_polarity_bias - self.polarity_bias).abs() / 2.0;

        // Calculate influence (strength decreases with distance, increases with resonance and polarity match)
        let distance_factor = (1.0 - normalized_distance).max(0.0);
        let influence = self.attractor_strength
            * distance_factor
            * (0.5 + resonance_match * 0.3 + polarity_match * 0.2);

        // Return as multiplier (0.0 to 2.0)
        (1.0 + influence).min(2.0)
    }
}

impl Density {
    /// Convert Density to u8 (1-8)
    pub fn as_u8(&self) -> u8 {
        match self {
            Density::First(_) => 1,
            Density::Second(_) => 2,
            Density::Third => 3,
            Density::Fourth => 4,
            Density::Fifth => 5,
            Density::Sixth => 6,
            Density::Seventh => 7,
            Density::Eighth => 8,
        }
    }

    /// Convert Density to usize (1-8)
    pub fn as_usize(&self) -> usize {
        self.as_u8() as usize
    }

    /// Create Density from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Density::First(Density1SubLevel::Quantum)),
            2 => Some(Density::Second(Density2SubLevel::Cellular)),
            3 => Some(Density::Third),
            4 => Some(Density::Fourth),
            5 => Some(Density::Fifth),
            6 => Some(Density::Sixth),
            7 => Some(Density::Seventh),
            8 => Some(Density::Eighth),
            _ => None,
        }
    }

    /// Get the attractor field for this density level
    ///
    /// Phase 5: Organic Evolution
    ///
    /// Returns the attractor field that guides entities toward this density.
    pub fn get_attractor_field(&self) -> DensityAttractorField {
        match self {
            Density::First(_) => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.3, // Weak attractor - many entities stay here
                attractor_range: 0.3,
                attractor_threshold: 0.0,
                resonance_frequency: 0.1,
                polarity_bias: 0.0,
            },
            Density::Second(_) => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.5, // Moderate attractor
                attractor_range: 0.4,
                attractor_threshold: 0.2,
                resonance_frequency: 0.2,
                polarity_bias: 0.0,
            },
            Density::Third => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.7, // Strong attractor - choice creates momentum
                attractor_range: 0.5,
                attractor_threshold: 0.4,
                resonance_frequency: 0.4,
                polarity_bias: 0.0,
            },
            Density::Fourth => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.9, // Very strong attractor - polarization drives evolution
                attractor_range: 0.6,
                attractor_threshold: 0.6,
                resonance_frequency: 0.7,
                polarity_bias: 0.0,
            },
            Density::Fifth => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.95, // Extremely strong attractor
                attractor_range: 0.7,
                attractor_threshold: 0.75,
                resonance_frequency: 0.85,
                polarity_bias: 0.0,
            },
            Density::Sixth => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.98, // Near-complete attraction
                attractor_range: 0.8,
                attractor_threshold: 0.85,
                resonance_frequency: 0.95,
                polarity_bias: 0.0,
            },
            Density::Seventh => DensityAttractorField {
                density_level: *self,
                attractor_strength: 0.99, // Almost complete attraction
                attractor_range: 0.9,
                attractor_threshold: 0.95,
                resonance_frequency: 0.99,
                polarity_bias: 0.0,
            },
            Density::Eighth => DensityAttractorField {
                density_level: *self,
                attractor_strength: 1.0, // Complete attraction
                attractor_range: 1.0,
                attractor_threshold: 1.0,
                resonance_frequency: 1.0,
                polarity_bias: 0.0,
            },
        }
    }

    /// Get all densities in the octave
    ///
    /// Returns a vector of all 8 densities in the density octave.
    pub fn all() -> Vec<Density> {
        vec![
            Density::First(Density1SubLevel::Quantum),
            Density::Second(Density2SubLevel::Cellular),
            Density::Third,
            Density::Fourth,
            Density::Fifth,
            Density::Sixth,
            Density::Seventh,
            Density::Eighth,
        ]
    }

    /// Get all sub-densities for this density
    ///
    /// Returns a vector of all sub-densities for this density level.
    /// Densities without sub-densities return an empty vector.
    pub fn sub_densities(&self) -> Vec<SubDensity> {
        match self {
            Density::First(_) => vec![
                SubDensity::QuantumRealm,
                SubDensity::AtomicRealm,
                SubDensity::MolecularRealm,
                SubDensity::PlanetaryRealm,
            ],
            Density::Second(_) => vec![
                SubDensity::CellularRealm,
                SubDensity::SimpleLifeRealm,
                SubDensity::ComplexLifeRealm,
            ],
            Density::Third => vec![SubDensity::ConsciousLifeRealm, SubDensity::SocietalRealm],
            _ => vec![], // Higher densities don't have sub-densities
        }
    }
}

impl std::fmt::Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Density::First(_) => write!(f, "1st Density"),
            Density::Second(_) => write!(f, "2nd Density"),
            Density::Third => write!(f, "3rd Density"),
            Density::Fourth => write!(f, "4th Density"),
            Density::Fifth => write!(f, "5th Density"),
            Density::Sixth => write!(f, "6th Density"),
            Density::Seventh => write!(f, "7th Density"),
            Density::Eighth => write!(f, "8th Density"),
        }
    }
}

impl DensityCharacteristics {
    fn first_density(sub_level: Density1SubLevel) -> Self {
        let (name, characteristics) = match sub_level {
            Density1SubLevel::Quantum => (
                "1st Density - Quantum Realm".to_string(),
                vec![
                    "Quantum particles and fields".to_string(),
                    "Consciousness is present but not self-aware".to_string(),
                    "Evolution through experience and growth".to_string(),
                ],
            ),
            Density1SubLevel::Atomic => (
                "1st Density - Atomic Realm".to_string(),
                vec![
                    "Atoms and galaxies".to_string(),
                    "Consciousness is present but not self-aware".to_string(),
                    "Evolution through experience and growth".to_string(),
                ],
            ),
            Density1SubLevel::Molecular => (
                "1st Density - Molecular Realm".to_string(),
                vec![
                    "Molecules and planets".to_string(),
                    "Consciousness is present but not self-aware".to_string(),
                    "Evolution through experience and growth".to_string(),
                ],
            ),
            Density1SubLevel::Planetary => (
                "1st Density - Planetary Realm".to_string(),
                vec![
                    "Planetary structures and Gaia consciousness precursors".to_string(),
                    "Consciousness is present but not self-aware".to_string(),
                    "Evolution through experience and growth".to_string(),
                ],
            ),
        };

        DensityCharacteristics {
            density_name: name,
            ray_color: "Red Ray".to_string(),
            primary_focus: "Physical manifestation".to_string(),
            consciousness_level: 0.05,
            veil_transparency: 0.05,
            spectrum_access: 0.05,
            characteristics,
        }
    }

    fn second_density(sub_level: Density2SubLevel) -> Self {
        let (name, characteristics) = match sub_level {
            Density2SubLevel::Cellular => (
                "2nd Density - Cellular Realm".to_string(),
                vec![
                    "Prokaryotes and simple life".to_string(),
                    "Biological consciousness emerges".to_string(),
                    "Growth through instinct and survival".to_string(),
                ],
            ),
            Density2SubLevel::SimpleLife => (
                "2nd Density - Simple Life Realm".to_string(),
                vec![
                    "Plants and simple animals".to_string(),
                    "Biological consciousness emerges".to_string(),
                    "Growth through instinct and survival".to_string(),
                ],
            ),
            Density2SubLevel::ComplexLife => (
                "2nd Density - Complex Life Realm".to_string(),
                vec![
                    "Eukaryotes and complex animals".to_string(),
                    "Biological consciousness emerges".to_string(),
                    "Growth through instinct and survival".to_string(),
                ],
            ),
        };

        DensityCharacteristics {
            density_name: name,
            ray_color: "Orange Ray".to_string(),
            primary_focus: "Biological growth".to_string(),
            consciousness_level: 0.15,
            veil_transparency: 0.1,
            spectrum_access: 0.15,
            characteristics,
        }
    }

    pub fn third_density() -> Self {
        DensityCharacteristics {
            density_name: "3rd Density".to_string(),
            ray_color: "Yellow Ray".to_string(),
            primary_focus: "Self-awareness and choice".to_string(),
            consciousness_level: 0.3,
            veil_transparency: 0.1,
            spectrum_access: 0.25,
            characteristics: vec![
                "Self-aware consciousness emerges".to_string(),
                "The Veil is fully active".to_string(),
                "The choice of polarity (service-to-self vs service-to-others)".to_string(),
                "Societal development and cultural evolution".to_string(),
            ],
        }
    }

    pub fn fourth_density() -> Self {
        DensityCharacteristics {
            density_name: "4th Density".to_string(),
            ray_color: "Green Ray".to_string(),
            primary_focus: "Understanding, love, compassion".to_string(),
            consciousness_level: 0.6,
            veil_transparency: 0.4,
            spectrum_access: 0.5,
            characteristics: vec![
                "The Veil begins to thin".to_string(),
                "Entities access more of the spectrum".to_string(),
                "Service-to-others polarity becomes more accessible".to_string(),
                "Group consciousness and telepathy".to_string(),
            ],
        }
    }

    fn fifth_density() -> Self {
        DensityCharacteristics {
            density_name: "5th Density".to_string(),
            ray_color: "Blue Ray".to_string(),
            primary_focus: "Wisdom, light, teaching/learning".to_string(),
            consciousness_level: 0.8,
            veil_transparency: 0.7,
            spectrum_access: 0.75,
            characteristics: vec![
                "The Veil mostly dissolved".to_string(),
                "Entities access even more of the spectrum".to_string(),
                "Teachers and learners share wisdom".to_string(),
                "Light bodies and energy manifestation".to_string(),
            ],
        }
    }

    fn sixth_density() -> Self {
        DensityCharacteristics {
            density_name: "6th Density".to_string(),
            ray_color: "Indigo-Ray".to_string(),
            primary_focus: "Unity, balance, harmony".to_string(),
            consciousness_level: 0.95,
            veil_transparency: 1.0,
            spectrum_access: 1.0,
            characteristics: vec![
                "The Veil completely dissolved".to_string(),
                "Entities access the entire spectrum".to_string(),
                "Both Space/Time and Time/Space dissolve into pure conscious light".to_string(),
                "Unity consciousness and oneness".to_string(),
            ],
        }
    }

    fn seventh_density() -> Self {
        DensityCharacteristics {
            density_name: "7th Density".to_string(),
            ray_color: "Violet-Ray".to_string(),
            primary_focus: "Completion, gateway to next octave".to_string(),
            consciousness_level: 0.99,
            veil_transparency: 1.0,
            spectrum_access: 1.0,
            characteristics: vec![
                "Entities transcend the spectrum entirely".to_string(),
                "Pure unity consciousness".to_string(),
                "Gateway to the next octave of experience".to_string(),
                "Preparation for return to IntelligentInfinity".to_string(),
            ],
        }
    }

    fn eighth_density() -> Self {
        DensityCharacteristics {
            density_name: "8th Density".to_string(),
            ray_color: "White Light".to_string(),
            primary_focus: "Return to IntelligentInfinity".to_string(),
            consciousness_level: 1.0,
            veil_transparency: 1.0,
            spectrum_access: 1.0,
            characteristics: vec![
                "Completion of the octave".to_string(),
                "Return to the source".to_string(),
                "Preparation for the next cycle".to_string(),
                "Merging with IntelligentInfinity".to_string(),
            ],
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::{
        EntitySpectrumAccess, EntityState, PolarityState, SpectrumAccessLevel, VibrationalState,
    };

    fn create_test_entity_state() -> EntityState {
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: crate::evolution_density_octave::density_octave::Density::First(
                    Density1SubLevel::Quantum,
                ),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.0,
            },
            consciousness_level: 0.1,
            experience_accumulation: 20.0,
            learning_progress: 10.0,
        }
    }

    fn create_test_spectrum_access(level: SpectrumAccessLevel) -> EntitySpectrumAccess {
        EntitySpectrumAccess {
            space_time_access: 1.0
                - match level {
                    SpectrumAccessLevel::ThirdDensity => 0.1,
                    SpectrumAccessLevel::FourthDensity => 0.4,
                    SpectrumAccessLevel::FifthDensity => 0.7,
                    SpectrumAccessLevel::SixthDensity => 1.0,
                    SpectrumAccessLevel::SeventhDensity => 1.0,
                },
            oneness_access: match level {
                SpectrumAccessLevel::ThirdDensity => 0.1,
                SpectrumAccessLevel::FourthDensity => 0.4,
                SpectrumAccessLevel::FifthDensity => 0.7,
                SpectrumAccessLevel::SixthDensity => 1.0,
                SpectrumAccessLevel::SeventhDensity => 1.0,
            },
            veil_transparency: match level {
                SpectrumAccessLevel::ThirdDensity => 0.1,
                SpectrumAccessLevel::FourthDensity => 0.4,
                SpectrumAccessLevel::FifthDensity => 0.7,
                SpectrumAccessLevel::SixthDensity => 1.0,
                SpectrumAccessLevel::SeventhDensity => 1.0,
            },
            evolutionary_level: level,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
        }
    }

    #[test]
    fn test_density_octave_creation() {
        let octave = DensityOctave::new();

        assert_eq!(
            octave.collective_density,
            Density::First(Density1SubLevel::Quantum)
        );
        assert_eq!(octave.octave_structure.densities.len(), 8);
        assert_eq!(octave.octave_structure.transition_thresholds.len(), 7);
    }

    #[test]
    fn test_first_density_advancement() {
        let mut octave = DensityOctave::new();

        // Quantum -> Atomic: needs 6% (15 experience)
        let mut entity_state = create_test_entity_state();
        entity_state.experience_accumulation = 15.0;
        octave.update_collective_emergence(&entity_state);
        let result = octave.advance_collective_emergence();
        assert!(matches!(result, DensityTransitionResult::Advanced { .. }));

        // Atomic -> Molecular: needs 12% (30 experience)
        entity_state.experience_accumulation = 30.0;
        octave.update_collective_emergence(&entity_state);
        let result = octave.advance_collective_emergence();
        assert!(matches!(result, DensityTransitionResult::Advanced { .. }));

        // Molecular -> Planetary: needs 18% (45 experience)
        entity_state.experience_accumulation = 45.0;
        octave.update_collective_emergence(&entity_state);
        let result = octave.advance_collective_emergence();
        assert!(matches!(result, DensityTransitionResult::Advanced { .. }));

        // Try to advance to 2nd density without enough progress
        entity_state.experience_accumulation = 45.0; // Still 18% < 25%
        octave.update_collective_emergence(&entity_state);
        let result = octave.advance_collective_emergence();
        assert!(matches!(result, DensityTransitionResult::NotReady { .. }));
    }

    #[test]
    fn test_density_characteristics() {
        let quantum = DensityCharacteristics::first_density(Density1SubLevel::Quantum);
        assert_eq!(quantum.ray_color, "Red Ray");
        assert_eq!(quantum.consciousness_level, 0.05);

        let third = DensityCharacteristics::third_density();
        assert_eq!(third.ray_color, "Yellow Ray");
        assert_eq!(third.consciousness_level, 0.3);
        assert!(third
            .characteristics
            .contains(&"The Veil is fully active".to_string()));

        let fourth = DensityCharacteristics::fourth_density();
        assert_eq!(fourth.ray_color, "Green Ray");
        assert_eq!(fourth.consciousness_level, 0.6);
        assert!(fourth
            .characteristics
            .contains(&"The Veil begins to thin".to_string()));

        let sixth = DensityCharacteristics::sixth_density();
        assert_eq!(sixth.ray_color, "Indigo-Ray");
        assert_eq!(sixth.veil_transparency, 1.0);
        assert!(sixth
            .characteristics
            .contains(&"The Veil completely dissolved".to_string()));
    }

    #[test]
    fn test_evolutionary_progress_update() {
        let mut octave = DensityOctave::new();
        let mut entity_state = create_test_entity_state();
        entity_state.consciousness_level = 0.5;
        entity_state.experience_accumulation = 10.0;
        entity_state.learning_progress = 5.0;
        let _spectrum_access = create_test_spectrum_access(SpectrumAccessLevel::FourthDensity);

        octave.update_collective_emergence(&entity_state);

        // Use collective_emergence instead of deprecated evolutionary_progress
        assert_eq!(octave.collective_emergence.consciousness_expansion, 0.5);
        assert_eq!(octave.collective_emergence.experience_accumulation, 10.0);
        assert_eq!(octave.collective_emergence.learning_progress, 5.0);
        assert_eq!(octave.collective_emergence.progress, 0.04); // 10.0 / 250.0 = 0.04
    }

    #[test]
    fn test_transition_readiness() {
        let mut octave = DensityOctave::new();
        let _entity_state = create_test_entity_state();

        // Check readiness at 1st density - Quantum Realm
        let readiness = octave.check_collective_emergence_readiness();
        assert_eq!(readiness.next_density, "1st Density - Atomic Realm");
        assert!(!readiness.is_ready);

        // Update progress to 10% (25 experience)
        let mut entity_state = create_test_entity_state();
        entity_state.experience_accumulation = 25.0;
        octave.update_collective_emergence(&entity_state);

        let readiness = octave.check_collective_emergence_readiness();
        // Still at Quantum, need 6% to advance to Atomic
        assert!(readiness.is_ready);
        assert_eq!(readiness.current_progress, 10.0); // 25.0 / 250.0 * 100 = 10.0
        assert_eq!(readiness.required_progress, 6.0);
    }

    #[test]
    fn test_density_transition_thresholds() {
        let octave = DensityOctave::new();

        assert_eq!(octave.octave_structure.transition_thresholds[0], 0.25); // 1st → 2nd
        assert_eq!(octave.octave_structure.transition_thresholds[1], 0.50); // 2nd → 3rd
        assert_eq!(octave.octave_structure.transition_thresholds[2], 0.75); // 3rd → 4th
        assert_eq!(octave.octave_structure.transition_thresholds[3], 0.85); // 4th → 5th
        assert_eq!(octave.octave_structure.transition_thresholds[4], 0.95); // 5th → 6th
        assert_eq!(octave.octave_structure.transition_thresholds[5], 0.99); // 6th → 7th
        assert_eq!(octave.octave_structure.transition_thresholds[6], 1.00); // 7th → 8th
    }
}
