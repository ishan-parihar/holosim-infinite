//! Fractal-Holographic Structure Implementation
//!
//! This module implements the 7×7×7 = 343 state structure per entity,
//! following the fractal-holographic principles from the Law of One cosmology.
//!
//! # Structure
//! - Level 1: Primary Density (7 states) - D1 through D7
//! - Level 2: Sub-Density (7×7 = 49 states) - Each primary density has 7 sub-densities
//! - Level 3: Sub-Sub-Density (7×7×7 = 343 states) - Each sub-density has 7 sub-sub-densities
//!
//! # Holographic Principle
//! Each state contains information about all other states. Changes in one state
//! affect all other states through non-local influence.
//!
//! # Fractal Principle
//! Self-similarity at all levels. The same patterns repeat at different scales.

use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};
use crate::types::{Density, Float};
use lru::LruCache;
use std::sync::Arc;

/// Unique identifier for a state in the 7×7×7 structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateID {
    /// Primary density index (0-6, representing D1-D7)
    pub primary: u8,

    /// Sub-density index (0-6, representing sub-densities within a primary density)
    pub sub: u8,

    /// Sub-sub-density index (0-6, representing sub-sub-densities within a sub-density)
    pub sub_sub: u8,
}

impl StateID {
    /// Create a new StateID
    ///
    /// # Arguments
    /// * `primary` - Primary density index (0-6)
    /// * `sub` - Sub-density index (0-6)
    /// * `sub_sub` - Sub-sub-density index (0-6)
    ///
    /// # Returns
    /// * `Some(StateID)` if all indices are valid
    /// * `None` if any index is invalid
    pub fn new(primary: u8, sub: u8, sub_sub: u8) -> Option<Self> {
        if primary < 7 && sub < 7 && sub_sub < 7 {
            Some(Self {
                primary,
                sub,
                sub_sub,
            })
        } else {
            None
        }
    }

    /// Get the linear index of this state (0-342)
    pub fn linear_index(&self) -> usize {
        (self.primary as usize) * 49 + (self.sub as usize) * 7 + (self.sub_sub as usize)
    }

    /// Create a StateID from a linear index (0-342)
    pub fn from_linear_index(index: usize) -> Option<Self> {
        if index < 343 {
            let primary = (index / 49) as u8;
            let sub = ((index % 49) / 7) as u8;
            let sub_sub = (index % 7) as u8;
            Some(Self {
                primary,
                sub,
                sub_sub,
            })
        } else {
            None
        }
    }

    /// Get the Density enum for this state
    pub fn density(&self) -> Density {
        // primary 0-6 maps to Density 1-7 (D1-D7)
        Density::from_u8(self.primary + 1)
    }
}

impl std::fmt::Display for StateID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "D{}.{}.{}",
            self.primary + 1,
            self.sub + 1,
            self.sub_sub + 1
        )
    }
}

/// Primary density state (Level 1)
///
/// Represents one of the 7 primary densities (D1-D7)
#[derive(Debug, Clone)]
pub struct PrimaryDensityState {
    /// The density (D1-D7)
    pub density: Density,

    /// Consciousness level at this density
    pub consciousness_level: Float,

    /// Physical manifestation (if any)
    pub physical_manifestation: Option<PhysicalManifestation>,

    /// Archetype influence (22 archetypes)
    pub archetype_influence: [Float; 22],

    /// Lambda (spiritual mass) at this density
    pub lambda: Float,

    /// Overall activation level
    pub activation: Float,
}

impl PrimaryDensityState {
    /// Create a new primary density state
    pub fn new(density: Density) -> Self {
        Self {
            density,
            consciousness_level: 0.0,
            physical_manifestation: None,
            archetype_influence: [0.0; 22],
            lambda: 0.0,
            activation: 0.0,
        }
    }

    /// Initialize with default values for this density
    pub fn initialize(&mut self, base_consciousness: Float) {
        // Consciousness level increases with density
        let density_factor = self.density.as_u8() as Float / 7.0;
        self.consciousness_level = base_consciousness * density_factor;

        // Lambda increases with density
        self.lambda = density_factor;

        // Activation starts low
        self.activation = 0.1 * density_factor;
    }

    /// Get the density number (1-8)
    pub fn density_number(&self) -> u8 {
        self.density.as_u8()
    }
}

impl SubDensityCharacteristics {
    /// Create new sub-density characteristics
    pub fn new(sub_density: u8, primary_density: Density) -> Self {
        let (learning_focus, primary_lessons, typical_experiences) =
            Self::generate_characteristics(sub_density, primary_density);

        Self {
            developmental_stage: (sub_density + 1) as Float / 7.0,
            learning_focus,
            primary_lessons,
            typical_experiences,
        }
    }

    /// Generate characteristics based on sub-density and primary density
    fn generate_characteristics(
        sub_density: u8,
        primary_density: Density,
    ) -> (String, Vec<String>, Vec<String>) {
        match primary_density {
            Density::First => {
                // D1: Elemental awareness
                let learning_focus = match sub_density {
                    0 => "Elemental formation",
                    1 => "Basic awareness",
                    2 => "Energy movement",
                    3 => "Vibration patterns",
                    4 => "Sound resonance",
                    5 => "Light coherence",
                    _ => "Unity recognition",
                };
                (
                    learning_focus.to_string(),
                    vec!["Awareness of existence".to_string()],
                    vec!["Vibrating".to_string()],
                )
            }
            Density::Second => {
                // D2: Growth and movement
                let learning_focus = match sub_density {
                    0 => "Growth impulse",
                    1 => "Movement patterns",
                    2 => "Survival instincts",
                    3 => "Basic learning",
                    4 => "Social interaction",
                    5 => "Emotional development",
                    _ => "Group consciousness",
                };
                (
                    learning_focus.to_string(),
                    vec!["Growth and movement".to_string()],
                    vec!["Growing".to_string(), "Moving".to_string()],
                )
            }
            Density::Third => {
                // D3: Self-awareness and choice
                let learning_focus = match sub_density {
                    0 => "Self-awareness emergence",
                    1 => "Choice making",
                    2 => "Polarity choice",
                    3 => "Service learning",
                    4 => "Control learning",
                    5 => "Relationship dynamics",
                    _ => "Catalyst processing",
                };
                (
                    learning_focus.to_string(),
                    vec!["Self-awareness and choice".to_string()],
                    vec!["Choosing".to_string(), "Learning".to_string()],
                )
            }
            Density::Fourth => {
                // D4: Love and understanding
                let learning_focus = match sub_density {
                    0 => "Love activation",
                    1 => "Compassion development",
                    2 => "Understanding others",
                    3 => "Service to others",
                    4 => "Healing abilities",
                    5 => "Intuitive awareness",
                    _ => "Universal love",
                };
                (
                    learning_focus.to_string(),
                    vec!["Love and understanding".to_string()],
                    vec!["Loving".to_string(), "Understanding".to_string()],
                )
            }
            Density::Fifth => {
                // D5: Wisdom and light
                let learning_focus = match sub_density {
                    0 => "Wisdom accumulation",
                    1 => "Light body activation",
                    2 => "Telepathic communication",
                    3 => "Energy healing",
                    4 => "Time/space awareness",
                    5 => "Dimensional travel",
                    _ => "Light wisdom",
                };
                (
                    learning_focus.to_string(),
                    vec!["Wisdom and light".to_string()],
                    vec!["Wisdomizing".to_string(), "Illuminating".to_string()],
                )
            }
            Density::Sixth => {
                // D6: Unity and equality
                let learning_focus = match sub_density {
                    0 => "Unity consciousness",
                    1 => "Equality recognition",
                    2 => "Social memory complex",
                    3 => "Group harmony",
                    4 => "Collective wisdom",
                    5 => "Universal service",
                    _ => "Social memory",
                };
                (
                    learning_focus.to_string(),
                    vec!["Unity and equality".to_string()],
                    vec!["Uniting".to_string(), "Harmonizing".to_string()],
                )
            }
            Density::Seventh => {
                // D7: Gateway and completion
                let learning_focus = match sub_density {
                    0 => "Gateway preparation",
                    1 => "Time/space mastery",
                    2 => "Energy transmutation",
                    3 => "Universal awareness",
                    4 => "Creator recognition",
                    5 => "Completion integration",
                    _ => "Gateway passage",
                };
                (
                    learning_focus.to_string(),
                    vec!["Gateway and completion".to_string()],
                    vec!["Transcending".to_string(), "Completing".to_string()],
                )
            }
            Density::Eighth => {
                // D8: Completion and reintegration
                let learning_focus = match sub_density {
                    0 => "Universal reintegration",
                    1 => "Complete awareness",
                    2 => "Total unity",
                    3 => "Infinity recognition",
                    4 => "Creator realization",
                    5 => "Full completion",
                    _ => "Gateway passage",
                };
                (
                    learning_focus.to_string(),
                    vec!["Completion and reintegration".to_string()],
                    vec!["Reintegrating".to_string(), "Completing".to_string()],
                )
            }
        }
    }
}

/// Physical manifestation at a density
#[derive(Debug, Clone)]
pub struct PhysicalManifestation {
    /// Type of manifestation
    pub manifestation_type: ManifestationType,

    /// Intensity of manifestation
    pub intensity: Float,

    /// Scale of manifestation
    pub scale: Float,
}

/// Type of physical manifestation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestationType {
    /// No physical manifestation (D1-D2)
    None,

    /// Light/energy manifestation (D3)
    Light,

    /// Chemical/biological manifestation (D4-D5)
    Chemical,

    /// Social/civilizational manifestation (D6)
    Social,

    /// Spiritual/universal manifestation (D7)
    Spiritual,
}

/// Sub-density characteristics
#[derive(Debug, Clone)]
pub struct SubDensityCharacteristics {
    /// Developmental stage within the primary density
    pub developmental_stage: Float,

    /// Learning focus
    pub learning_focus: String,

    /// Primary lessons
    pub primary_lessons: Vec<String>,

    /// Typical experiences
    pub typical_experiences: Vec<String>,
}

/// Sub-density state (Level 2)
///
/// Represents one of the 49 sub-densities (7 sub-densities per primary density)
#[derive(Debug, Clone)]
pub struct SubDensityState {
    /// Sub-density index (0-6)
    pub sub_density: u8,

    /// Characteristics of this sub-density
    pub characteristics: SubDensityCharacteristics,

    /// Experiences accumulated
    pub experiences: Vec<Experience>,

    /// Developmental progress (0.0 to 1.0)
    pub developmental_progress: Float,

    /// Activation level
    pub activation: Float,

    /// Lambda at this sub-density
    pub lambda: Float,
}

impl SubDensityState {
    /// Create a new sub-density state
    pub fn new(sub_density: u8, primary_density: Density) -> Self {
        let characteristics = SubDensityCharacteristics::new(sub_density, primary_density);

        Self {
            sub_density,
            characteristics,
            experiences: Vec::new(),
            developmental_progress: 0.0,
            activation: 0.0,
            lambda: 0.0,
        }
    }

    /// Initialize with default values
    pub fn initialize(&mut self, base_progress: Float) {
        // Developmental progress varies by sub-density
        let sub_factor = (self.sub_density + 1) as Float / 7.0;
        self.developmental_progress = base_progress * sub_factor;

        // Lambda varies by sub-density
        self.lambda = sub_factor * 0.5;

        // Activation starts low
        self.activation = 0.1 * sub_factor;
    }

    /// Add an experience
    pub fn add_experience(&mut self, experience: Experience) {
        self.experiences.push(experience);
    }

    /// Get developmental stage
    pub fn developmental_stage(&self) -> Float {
        self.characteristics.developmental_stage
    }
}

/// Experience at a sub-density
#[derive(Debug, Clone)]
pub struct Experience {
    /// Experience type
    pub experience_type: String,

    /// Intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Learning value (0.0 to 1.0)
    pub learning_value: Float,

    /// Timestamp
    pub timestamp: Float,
}

/// Specific experience at sub-sub-density
#[derive(Debug, Clone)]
pub struct SpecificExperience {
    /// Experience description
    pub description: String,

    /// Experience quality
    pub quality: ExperienceQuality,

    /// Intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Learning value (0.0 to 1.0)
    pub learning_value: Float,
}

/// Quality of experience
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperienceQuality {
    /// Positive experience
    Positive,

    /// Negative experience
    Negative,

    /// Neutral experience
    Neutral,

    /// Mixed experience
    Mixed,
}

/// Sub-sub-density state (Level 3)
///
/// Represents one of the 343 sub-sub-densities (7 sub-sub-densities per sub-density)
#[derive(Debug, Clone)]
pub struct SubSubDensityState {
    /// Sub-sub-density index (0-6)
    pub sub_sub_density: u8,

    /// Specific experience
    pub specific_experience: SpecificExperience,

    /// Activation level
    pub activation: Float,

    /// Lambda at this sub-sub-density
    pub lambda: Float,

    /// Connection strength to other states
    pub connection_strengths: [Float; 343],
}

impl SubSubDensityState {
    /// Create a new sub-sub-density state
    pub fn new(sub_sub_density: u8) -> Self {
        let specific_experience = SpecificExperience::new(sub_sub_density);

        Self {
            sub_sub_density,
            specific_experience,
            activation: 0.0,
            lambda: 0.0,
            connection_strengths: [0.0; 343],
        }
    }

    /// Initialize with default values
    pub fn initialize(&mut self, base_activation: Float) {
        let sub_sub_factor = (self.sub_sub_density + 1) as Float / 7.0;
        self.activation = base_activation * sub_sub_factor;
        self.lambda = sub_sub_factor * 0.3;

        // Initialize connection strengths (holographic principle)
        for i in 0..343 {
            let state_id = StateID::from_linear_index(i).unwrap();
            let distance = self.calculate_holographic_distance(&state_id);
            self.connection_strengths[i] = 1.0 / (1.0 + distance);
        }
    }

    /// Calculate holographic distance to another state
    fn calculate_holographic_distance(&self, other: &StateID) -> Float {
        // This is a placeholder - actual holographic distance calculation
        // would be more sophisticated
        let linear_self = self.sub_sub_density as Float;
        let linear_other = other.linear_index() as Float;
        (linear_self - linear_other).abs() / 342.0
    }

    /// Get connection strength to another state
    pub fn get_connection_strength(&self, state_id: &StateID) -> Float {
        self.connection_strengths[state_id.linear_index()]
    }

    /// Set connection strength to another state
    pub fn set_connection_strength(&mut self, state_id: &StateID, strength: Float) {
        self.connection_strengths[state_id.linear_index()] = strength;
    }
}

impl SpecificExperience {
    /// Create a new specific experience
    pub fn new(sub_sub_density: u8) -> Self {
        let (description, quality) = Self::generate_experience(sub_sub_density);

        Self {
            description,
            quality,
            intensity: 0.5,
            learning_value: 0.5,
        }
    }

    /// Generate experience based on sub-sub-density
    fn generate_experience(sub_sub_density: u8) -> (String, ExperienceQuality) {
        let description = format!("Experience at sub-sub-density {}", sub_sub_density + 1);
        let quality = match sub_sub_density % 4 {
            0 => ExperienceQuality::Positive,
            1 => ExperienceQuality::Negative,
            2 => ExperienceQuality::Neutral,
            _ => ExperienceQuality::Mixed,
        };
        (description, quality)
    }
}

/// Change in a state
#[derive(Debug, Clone)]
pub struct StateChange {
    /// Type of change
    pub change_type: ChangeType,

    /// Magnitude of change
    pub magnitude: Float,

    /// Affected attributes
    pub affected_attributes: Vec<String>,
}

/// Type of state change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    /// Activation change
    Activation,

    /// Lambda change
    Lambda,

    /// Experience addition
    Experience,

    /// Coherence change
    Coherence,

    /// Archetype influence change
    ArchetypeInfluence,
}

/// Information flow between states
#[derive(Debug, Clone)]
pub struct HolographicInformationFlow {
    /// Flow direction
    pub direction: HolographicFlowDirection,

    /// Information content
    pub content: Vec<u8>,

    /// Flow intensity
    pub intensity: Float,
}

/// Direction of information flow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolographicFlowDirection {
    /// Bidirectional flow
    Bidirectional,

    /// Unidirectional from state
    Outgoing,

    /// Unidirectional to state
    Incoming,
}

/// Fractal connection between states
#[derive(Debug, Clone)]
pub struct FractalConnection {
    /// From state
    pub from_state: StateID,

    /// To state
    pub to_state: StateID,

    /// Connection coherence
    pub coherence: Float,

    /// Information flow
    pub holographic_information_flow: HolographicInformationFlow,

    /// Connection type
    pub connection_type: ConnectionType,
}

/// Type of fractal connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Hierarchical connection (parent-child)
    Hierarchical,

    /// Peer connection (same level)
    Peer,

    /// Cross-level connection
    CrossLevel,

    /// Holographic connection (non-local)
    Holographic,
}

impl FractalConnection {
    /// Create a new fractal connection
    pub fn new(from_state: StateID, to_state: StateID, connection_type: ConnectionType) -> Self {
        Self {
            from_state,
            to_state,
            coherence: 0.5,
            holographic_information_flow: HolographicInformationFlow {
                direction: HolographicFlowDirection::Bidirectional,
                content: Vec::new(),
                intensity: 0.5,
            },
            connection_type,
        }
    }

    /// Update connection coherence
    pub fn update_coherence(&mut self, delta: Float) {
        self.coherence = (self.coherence + delta).clamp(0.0, 1.0);
    }

    /// Get connection strength
    pub fn strength(&self) -> Float {
        self.coherence * self.holographic_information_flow.intensity
    }
}

/// Holographic container for all 343 states
///
/// This is the main structure that implements the fractal-holographic architecture.
/// It contains all 7×7×7 = 343 states and manages holographic propagation.
#[derive(Debug, Clone)]
pub struct HolographicContainer {
    /// Level 1: Primary Density (7 states)
    pub primary_density: Vec<PrimaryDensityState>,

    /// Level 2: Sub-Density (7×7 = 49 states)
    pub sub_density: Vec<Vec<SubDensityState>>,

    /// Level 3: Sub-Sub-Density (7×7×7 = 343 states)
    pub sub_sub_density: Vec<Vec<Vec<SubSubDensityState>>>,

    /// Holographic coherence (0.0 to 1.0)
    pub coherence: Float,

    /// Fractal connections
    pub fractal_connections: Vec<FractalConnection>,

    /// State activation levels (7×7×7)
    pub activation_levels: Vec<Vec<Vec<Float>>>,
}

impl HolographicContainer {
    /// Create a new holographic container
    pub fn new() -> Self {
        let mut primary_density = Vec::new();
        for i in 0..7 {
            let density = Density::from_u8((i + 1) as u8);
            primary_density.push(PrimaryDensityState::new(density));
        }

        let mut sub_density = Vec::new();
        for primary in 0..7 {
            let mut row = Vec::new();
            let density = Density::from_u8((primary + 1) as u8);
            for sub in 0..7 {
                row.push(SubDensityState::new(sub as u8, density));
            }
            sub_density.push(row);
        }

        let mut sub_sub_density = Vec::new();
        for _primary in 0..7 {
            let mut plane = Vec::new();
            for _sub in 0..7 {
                let mut row = Vec::new();
                for sub_sub in 0..7 {
                    row.push(SubSubDensityState::new(sub_sub as u8));
                }
                plane.push(row);
            }
            sub_sub_density.push(plane);
        }

        let mut activation_levels = Vec::new();
        for _primary in 0..7 {
            let mut plane = Vec::new();
            for _sub in 0..7 {
                let row = vec![0.0; 7];
                plane.push(row);
            }
            activation_levels.push(plane);
        }

        let mut container = Self {
            primary_density,
            sub_density,
            sub_sub_density,
            coherence: 1.0,
            fractal_connections: Vec::new(),
            activation_levels,
        };

        container.initialize();
        container
    }

    /// Initialize all states
    fn initialize(&mut self) {
        // Initialize primary densities
        for (_i, state) in self.primary_density.iter_mut().enumerate() {
            state.initialize(1.0);
        }

        // Initialize sub-densities
        for primary in 0..7usize {
            for sub in 0..7usize {
                self.sub_density[primary][sub].initialize(0.5);
            }
        }

        // Initialize sub-sub-densities
        for primary in 0..7usize {
            for sub in 0..7usize {
                for sub_sub in 0..7usize {
                    self.sub_sub_density[primary][sub][sub_sub].initialize(0.3);
                }
            }
        }

        // Initialize activation levels
        for primary in 0..7usize {
            for sub in 0..7usize {
                for sub_sub in 0..7usize {
                    self.activation_levels[primary][sub][sub_sub] =
                        self.sub_sub_density[primary][sub][sub_sub].activation;
                }
            }
        }

        // Create fractal connections
        self.create_fractal_connections();
    }

    /// Create fractal connections between all states
    fn create_fractal_connections(&mut self) {
        self.fractal_connections.clear();

        // Create hierarchical connections
        for primary in 0..7u8 {
            for sub in 0..7u8 {
                for sub_sub in 0..7u8 {
                    let state_id = StateID::new(primary, sub, sub_sub).unwrap();

                    // Connect to parent sub-density
                    if sub > 0 {
                        let parent_id = StateID::new(primary, sub - 1, sub_sub).unwrap();
                        let connection = FractalConnection::new(
                            state_id,
                            parent_id,
                            ConnectionType::Hierarchical,
                        );
                        self.fractal_connections.push(connection);
                    }

                    // Connect to parent primary density
                    if primary > 0 {
                        let parent_id = StateID::new(primary - 1, sub, sub_sub).unwrap();
                        let connection = FractalConnection::new(
                            state_id,
                            parent_id,
                            ConnectionType::Hierarchical,
                        );
                        self.fractal_connections.push(connection);
                    }

                    // Connect to peer states (same sub-density)
                    for peer_sub_sub in 0..7u8 {
                        if peer_sub_sub != sub_sub {
                            let peer_id = StateID::new(primary, sub, peer_sub_sub).unwrap();
                            let connection =
                                FractalConnection::new(state_id, peer_id, ConnectionType::Peer);
                            self.fractal_connections.push(connection);
                        }
                    }

                    // Create holographic connections to random states
                    // (simulating non-local influence)
                    for _ in 0..3 {
                        let random_primary = (state_id.linear_index() * 7) % 7;
                        let random_sub = (state_id.linear_index() * 13) % 7;
                        let random_sub_sub = (state_id.linear_index() * 17) % 7;
                        let holographic_id = StateID::new(
                            random_primary as u8,
                            random_sub as u8,
                            random_sub_sub as u8,
                        )
                        .unwrap();
                        let connection = FractalConnection::new(
                            state_id,
                            holographic_id,
                            ConnectionType::Holographic,
                        );
                        self.fractal_connections.push(connection);
                    }
                }
            }
        }
    }

    /// Get a primary density state
    pub fn get_primary_density(&self, primary: u8) -> Option<&PrimaryDensityState> {
        if primary < 7 {
            Some(&self.primary_density[primary as usize])
        } else {
            None
        }
    }

    /// Get a sub-density state
    pub fn get_sub_density(&self, primary: u8, sub: u8) -> Option<&SubDensityState> {
        if primary < 7 && sub < 7 {
            Some(&self.sub_density[primary as usize][sub as usize])
        } else {
            None
        }
    }

    /// Get a sub-sub-density state
    pub fn get_sub_sub_density(
        &self,
        primary: u8,
        sub: u8,
        sub_sub: u8,
    ) -> Option<&SubSubDensityState> {
        if primary < 7 && sub < 7 && sub_sub < 7 {
            Some(&self.sub_sub_density[primary as usize][sub as usize][sub_sub as usize])
        } else {
            None
        }
    }

    /// Get a state by StateID
    pub fn get_state(&self, state_id: &StateID) -> Option<StateReference<'_>> {
        if state_id.primary < 7 && state_id.sub < 7 && state_id.sub_sub < 7 {
            Some(StateReference {
                primary: &self.primary_density[state_id.primary as usize],
                sub: &self.sub_density[state_id.primary as usize][state_id.sub as usize],
                sub_sub: &self.sub_sub_density[state_id.primary as usize][state_id.sub as usize]
                    [state_id.sub_sub as usize],
            })
        } else {
            None
        }
    }

    /// Propagate a change holographically to all other states
    ///
    /// This implements the holographic principle: changes in one state
    /// affect all other states through non-local influence.
    pub fn propagate_change(&mut self, state_id: StateID, change: StateChange) {
        // Apply change to the target state
        self.apply_change_to_state(state_id, &change);

        // Propagate to all other states
        for primary in 0..7u8 {
            for sub in 0..7u8 {
                for sub_sub in 0..7u8 {
                    let other_id = StateID::new(primary, sub, sub_sub).unwrap();
                    if other_id != state_id {
                        // Calculate influence based on holographic distance
                        let influence = self.calculate_holographic_influence(&state_id, &other_id);

                        // Apply propagated change
                        let propagated_change = StateChange {
                            change_type: change.change_type,
                            magnitude: change.magnitude * influence * self.coherence,
                            affected_attributes: change.affected_attributes.clone(),
                        };
                        self.apply_change_to_state(other_id, &propagated_change);
                    }
                }
            }
        }

        // Update activation levels
        self.update_activation_levels();
    }

    /// Apply a change to a specific state
    fn apply_change_to_state(&mut self, state_id: StateID, change: &StateChange) {
        let primary = state_id.primary as usize;
        let sub = state_id.sub as usize;
        let sub_sub = state_id.sub_sub as usize;

        match change.change_type {
            ChangeType::Activation => {
                self.sub_sub_density[primary][sub][sub_sub].activation =
                    (self.sub_sub_density[primary][sub][sub_sub].activation + change.magnitude)
                        .clamp(0.0, 1.0);
            }
            ChangeType::Lambda => {
                self.sub_sub_density[primary][sub][sub_sub].lambda =
                    (self.sub_sub_density[primary][sub][sub_sub].lambda + change.magnitude)
                        .clamp(0.0, 1.0);
            }
            ChangeType::Experience => {
                // Experience changes are handled at sub-density level
                self.sub_density[primary][sub].developmental_progress =
                    (self.sub_density[primary][sub].developmental_progress
                        + change.magnitude * 0.1)
                        .clamp(0.0, 1.0);
            }
            ChangeType::Coherence => {
                // Coherence changes affect the entire container
                self.coherence = (self.coherence + change.magnitude * 0.01).clamp(0.0, 1.0);
            }
            ChangeType::ArchetypeInfluence => {
                // Archetype influence changes are handled at primary density level
                for i in 0..22 {
                    self.primary_density[primary].archetype_influence[i] =
                        (self.primary_density[primary].archetype_influence[i]
                            + change.magnitude * 0.05)
                            .clamp(0.0, 1.0);
                }
            }
        }
    }

    /// Calculate holographic influence between two states
    fn calculate_holographic_influence(&self, from: &StateID, to: &StateID) -> Float {
        // Calculate influence based on:
        // 1. Hierarchical distance
        // 2. Connection strength
        // 3. Coherence

        let hierarchical_distance = self.calculate_hierarchical_distance(from, to);
        let connection_strength = self.get_connection_strength(from, to);

        // Influence decreases with distance
        let distance_factor = 1.0 / (1.0 + hierarchical_distance);

        // Influence increases with connection strength
        let connection_factor = connection_strength;

        // Influence increases with coherence
        let coherence_factor = self.coherence;

        distance_factor * connection_factor * coherence_factor
    }

    /// Calculate hierarchical distance between two states
    fn calculate_hierarchical_distance(&self, from: &StateID, to: &StateID) -> Float {
        // Distance based on level differences
        let primary_diff = (from.primary as Float - to.primary as Float).abs();
        let sub_diff = (from.sub as Float - to.sub as Float).abs();
        let sub_sub_diff = (from.sub_sub as Float - to.sub_sub as Float).abs();

        // Weighted distance (primary level is most significant)
        primary_diff * 3.0 + sub_diff * 2.0 + sub_sub_diff * 1.0
    }

    /// Get connection strength between two states
    fn get_connection_strength(&self, from: &StateID, to: &StateID) -> Float {
        // Check if there's a direct connection
        for connection in &self.fractal_connections {
            if connection.from_state == *from && connection.to_state == *to {
                return connection.strength();
            }
            if connection.from_state == *to && connection.to_state == *from {
                return connection.strength();
            }
        }

        // If no direct connection, use holographic connection strength
        self.sub_sub_density[from.primary as usize][from.sub as usize][from.sub_sub as usize]
            .get_connection_strength(to)
    }

    /// Update activation levels based on current state
    fn update_activation_levels(&mut self) {
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    self.activation_levels[primary][sub][sub_sub] =
                        self.sub_sub_density[primary][sub][sub_sub].activation;
                }
            }
        }
    }

    /// Maintain holographic coherence
    ///
    /// This method ensures that all states remain coherent with each other.
    /// It adjusts activation levels and connection strengths as needed.
    pub fn maintain_coherence(&mut self) {
        // Calculate average activation
        let mut total_activation = 0.0;
        let mut count = 0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    total_activation += self.sub_sub_density[primary][sub][sub_sub].activation;
                    count += 1;
                }
            }
        }
        let average_activation = total_activation / count as Float;

        // Adjust coherence based on activation variance
        let mut variance = 0.0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    let diff =
                        self.sub_sub_density[primary][sub][sub_sub].activation - average_activation;
                    variance += diff * diff;
                }
            }
        }
        variance /= count as Float;

        // Coherence decreases with high variance
        let target_coherence = 1.0 / (1.0 + variance * 10.0);
        self.coherence = self.coherence * 0.9 + target_coherence * 0.1;

        // Adjust connection strengths based on coherence
        for connection in &mut self.fractal_connections {
            connection.update_coherence((self.coherence - 0.5) * 0.1);
        }

        // Update activation levels
        self.update_activation_levels();
    }

    /// Get total number of states
    pub fn total_states(&self) -> usize {
        343
    }

    /// Get number of active states
    pub fn active_states(&self) -> usize {
        let mut count = 0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if self.sub_sub_density[primary][sub][sub_sub].activation > 0.1 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Get number of fractal connections
    pub fn total_connections(&self) -> usize {
        self.fractal_connections.len()
    }

    /// Get average activation level
    pub fn average_activation(&self) -> Float {
        let mut total = 0.0;
        let mut count = 0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    total += self.sub_sub_density[primary][sub][sub_sub].activation;
                    count += 1;
                }
            }
        }
        total / count as Float
    }

    /// Get coherence level
    pub fn get_coherence(&self) -> Float {
        self.coherence
    }

    /// Reset all states to initial values
    pub fn reset(&mut self) {
        self.initialize();
        self.coherence = 1.0;
    }

    /// Get state statistics
    pub fn get_statistics(&self) -> HolographicStatistics {
        let total_states = self.total_states();
        let active_states = self.active_states();
        let average_activation = self.average_activation();
        let coherence = self.get_coherence();
        let total_connections = self.total_connections();

        // Calculate average lambda
        let mut total_lambda = 0.0;
        let mut lambda_count = 0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    total_lambda += self.sub_sub_density[primary][sub][sub_sub].lambda;
                    lambda_count += 1;
                }
            }
        }
        let average_lambda = total_lambda / lambda_count as Float;

        HolographicStatistics {
            total_states,
            active_states,
            inactive_states: total_states - active_states,
            average_activation,
            average_lambda,
            coherence,
            total_connections,
            average_connection_strength: if total_connections > 0 {
                let mut total_strength = 0.0;
                for connection in &self.fractal_connections {
                    total_strength += connection.strength();
                }
                total_strength / total_connections as Float
            } else {
                0.0
            },
        }
    }
}

/// Reference to a state at all three levels
#[derive(Debug, Clone)]
pub struct StateReference<'a> {
    pub primary: &'a PrimaryDensityState,
    pub sub: &'a SubDensityState,
    pub sub_sub: &'a SubSubDensityState,
}

/// Statistics for the holographic container
#[derive(Debug, Clone)]
pub struct HolographicStatistics {
    pub total_states: usize,
    pub active_states: usize,
    pub inactive_states: usize,
    pub average_activation: Float,
    pub average_lambda: Float,
    pub coherence: Float,
    pub total_connections: usize,
    pub average_connection_strength: Float,
}

impl Default for HolographicContainer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// LAZY LOADING IMPLEMENTATION
// ============================================================================

/// Lazy-loading holographic container with LRU cache
///
/// This variant uses lazy initialization to avoid creating all 343 states upfront.
/// States are initialized on first access and cached using an LRU policy.
#[derive(Debug, Clone)]
pub struct LazyHolographicContainer {
    /// Primary density states (initialized lazily)
    primary_density: [Option<PrimaryDensityState>; 7],

    /// Sub-density states (initialized lazily)
    sub_density: [[Option<SubDensityState>; 7]; 7],

    /// Sub-sub-density states (initialized lazily)
    sub_sub_density: [[[Option<SubSubDensityState>; 7]; 7]; 7],

    /// LRU cache for frequently accessed sub-sub-density states
    cache: LruCache<StateID, SubSubDensityState>,

    /// Holographic coherence (0.0 to 1.0)
    coherence: Float,

    /// Fractal connections (created lazily)
    fractal_connections: Option<Vec<FractalConnection>>,

    /// State activation levels (7×7×7)
    activation_levels: [[[Float; 7]; 7]; 7],
}

impl LazyHolographicContainer {
    /// Create a new lazy holographic container
    ///
    /// This is much faster than creating a regular HolographicContainer
    /// because it doesn't initialize all 343 states upfront.
    pub fn new() -> Self {
        // Initialize all states as None (lazy)
        let primary_density: [Option<PrimaryDensityState>; 7] = Default::default();
        let sub_density: [[Option<SubDensityState>; 7]; 7] = Default::default();
        let sub_sub_density: [[[Option<SubSubDensityState>; 7]; 7]; 7] = Default::default();

        // Initialize activation levels to 0.0
        let activation_levels = [[[0.0; 7]; 7]; 7];

        // Create LRU cache with capacity for 100 states
        let cache = LruCache::new(std::num::NonZeroUsize::new(100).unwrap());

        Self {
            primary_density,
            sub_density,
            sub_sub_density,
            cache,
            coherence: 1.0,
            fractal_connections: None,
            activation_levels,
        }
    }

    /// Get or initialize a primary density state
    pub fn get_primary_density(&mut self, primary: u8) -> Option<&PrimaryDensityState> {
        if primary >= 7 {
            return None;
        }

        // Initialize if needed
        if self.primary_density[primary as usize].is_none() {
            let density = Density::from_u8(primary + 1);
            let mut state = PrimaryDensityState::new(density);
            state.initialize(1.0);
            self.primary_density[primary as usize] = Some(state);
        }

        self.primary_density[primary as usize].as_ref()
    }

    /// Get or initialize a sub-density state
    pub fn get_sub_density(&mut self, primary: u8, sub: u8) -> Option<&SubDensityState> {
        if primary >= 7 || sub >= 7 {
            return None;
        }

        // Initialize if needed
        if self.sub_density[primary as usize][sub as usize].is_none() {
            let density = Density::from_u8(primary + 1);
            let mut state = SubDensityState::new(sub, density);
            state.initialize(0.5);
            self.sub_density[primary as usize][sub as usize] = Some(state);
        }

        self.sub_density[primary as usize][sub as usize].as_ref()
    }

    /// Get or initialize a sub-sub-density state
    pub fn get_sub_sub_density(
        &mut self,
        primary: u8,
        sub: u8,
        sub_sub: u8,
    ) -> Option<&SubSubDensityState> {
        if primary >= 7 || sub >= 7 || sub_sub >= 7 {
            return None;
        }

        let state_id = StateID::new(primary, sub, sub_sub)?;

        // Check if already initialized in the storage
        if self.sub_sub_density[primary as usize][sub as usize][sub_sub as usize].is_some() {
            // Try to get from cache first
            if self.cache.contains(&state_id) {
                return self.cache.get(&state_id);
            } else {
                // Not in cache but in storage, add to cache
                if let Some(state) =
                    self.sub_sub_density[primary as usize][sub as usize][sub_sub as usize].clone()
                {
                    self.cache.put(state_id, state);
                    return self.cache.get(&state_id);
                }
            }
        }

        // Initialize new state
        let mut state = SubSubDensityState::new(sub_sub);
        state.initialize(0.3);

        // Initialize connection strengths
        for i in 0..343 {
            let other_id = StateID::from_linear_index(i).unwrap();
            let distance = Self::calculate_holographic_distance_static(sub_sub, &other_id);
            state.connection_strengths[i] = 1.0 / (1.0 + distance);
        }

        // Store in both cache and storage
        self.sub_sub_density[primary as usize][sub as usize][sub_sub as usize] =
            Some(state.clone());
        self.cache.put(state_id, state);

        // Return from cache
        self.cache.get(&state_id)
    }

    /// Get a state by StateID
    pub fn get_state(&mut self, state_id: &StateID) -> Option<StateReferenceLazy> {
        if state_id.primary >= 7 || state_id.sub >= 7 || state_id.sub_sub >= 7 {
            return None;
        }

        // Get all three levels (will initialize if needed)
        let primary = self.get_primary_density(state_id.primary)?.clone();
        let sub = self
            .get_sub_density(state_id.primary, state_id.sub)?
            .clone();
        let sub_sub = self
            .get_sub_sub_density(state_id.primary, state_id.sub, state_id.sub_sub)?
            .clone();

        Some(StateReferenceLazy {
            primary: Arc::new(primary),
            sub: Arc::new(sub),
            sub_sub: Arc::new(sub_sub),
        })
    }

    /// Propagate a change holographically to all other states
    ///
    /// This implements lazy propagation: only initialized states are affected.
    pub fn propagate_change(&mut self, state_id: StateID, change: StateChange) {
        // Apply change to the target state
        self.apply_change_to_state(state_id, &change);

        // Propagate to all initialized states
        for primary in 0..7u8 {
            for sub in 0..7u8 {
                for sub_sub in 0..7u8 {
                    let other_id = StateID::new(primary, sub, sub_sub).unwrap();
                    if other_id != state_id {
                        // Only propagate if the state is initialized
                        if self.sub_sub_density[primary as usize][sub as usize][sub_sub as usize]
                            .is_some()
                        {
                            let influence =
                                self.calculate_holographic_influence(&state_id, &other_id);

                            let propagated_change = StateChange {
                                change_type: change.change_type,
                                magnitude: change.magnitude * influence * self.coherence,
                                affected_attributes: change.affected_attributes.clone(),
                            };
                            self.apply_change_to_state(other_id, &propagated_change);
                        }
                    }
                }
            }
        }

        // Update activation levels
        self.update_activation_levels();
    }

    /// Apply a change to a specific state
    fn apply_change_to_state(&mut self, state_id: StateID, change: &StateChange) {
        let primary = state_id.primary as usize;
        let sub = state_id.sub as usize;
        let sub_sub = state_id.sub_sub as usize;

        // Ensure the state is initialized
        if self.sub_sub_density[primary][sub][sub_sub].is_none() {
            let _ = self.get_sub_sub_density(state_id.primary, state_id.sub, state_id.sub_sub);
        }

        // Apply the change
        match change.change_type {
            ChangeType::Activation => {
                if let Some(state) = &mut self.sub_sub_density[primary][sub][sub_sub] {
                    state.activation = (state.activation + change.magnitude).clamp(0.0, 1.0);
                    self.activation_levels[primary][sub][sub_sub] = state.activation;

                    // Update cache if present
                    if self.cache.contains(&state_id) {
                        self.cache.put(state_id, state.clone());
                    }
                }
            }
            ChangeType::Lambda => {
                if let Some(state) = &mut self.sub_sub_density[primary][sub][sub_sub] {
                    state.lambda = (state.lambda + change.magnitude).clamp(0.0, 1.0);

                    // Update cache if present
                    if self.cache.contains(&state_id) {
                        self.cache.put(state_id, state.clone());
                    }
                }
            }
            ChangeType::Experience => {
                if let Some(state) = &mut self.sub_density[primary][sub] {
                    state.developmental_progress =
                        (state.developmental_progress + change.magnitude * 0.1).clamp(0.0, 1.0);
                }
            }
            ChangeType::Coherence => {
                self.coherence = (self.coherence + change.magnitude * 0.01).clamp(0.0, 1.0);
            }
            ChangeType::ArchetypeInfluence => {
                if let Some(state) = &mut self.primary_density[primary] {
                    for i in 0..22 {
                        state.archetype_influence[i] = (state.archetype_influence[i]
                            + change.magnitude * 0.05)
                            .clamp(0.0, 1.0);
                    }
                }
            }
        }
    }

    /// Calculate holographic influence between two states
    fn calculate_holographic_influence(&self, from: &StateID, to: &StateID) -> Float {
        let hierarchical_distance = self.calculate_hierarchical_distance(from, to);
        let connection_strength = self.get_connection_strength(from, to);

        let distance_factor = 1.0 / (1.0 + hierarchical_distance);
        let connection_factor = connection_strength;
        let coherence_factor = self.coherence;

        distance_factor * connection_factor * coherence_factor
    }

    /// Calculate hierarchical distance between two states
    fn calculate_hierarchical_distance(&self, from: &StateID, to: &StateID) -> Float {
        let primary_diff = (from.primary as Float - to.primary as Float).abs();
        let sub_diff = (from.sub as Float - to.sub as Float).abs();
        let sub_sub_diff = (from.sub_sub as Float - to.sub_sub as Float).abs();

        primary_diff * 3.0 + sub_diff * 2.0 + sub_sub_diff * 1.0
    }

    /// Static helper to calculate holographic distance
    fn calculate_holographic_distance_static(sub_sub: u8, other: &StateID) -> Float {
        let linear_self = sub_sub as Float;
        let linear_other = other.linear_index() as Float;
        (linear_self - linear_other).abs() / 342.0
    }

    /// Get connection strength between two states
    fn get_connection_strength(&self, from: &StateID, to: &StateID) -> Float {
        // Check if states are initialized
        let from_state = match &self.sub_sub_density[from.primary as usize][from.sub as usize]
            [from.sub_sub as usize]
        {
            Some(state) => state,
            None => return 0.5, // Default strength for uninitialized states
        };

        from_state.get_connection_strength(to)
    }

    /// Update activation levels based on current state
    fn update_activation_levels(&mut self) {
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if let Some(state) = &self.sub_sub_density[primary][sub][sub_sub] {
                        self.activation_levels[primary][sub][sub_sub] = state.activation;
                    }
                }
            }
        }
    }

    /// Maintain holographic coherence
    pub fn maintain_coherence(&mut self) {
        // Calculate average activation of initialized states
        let mut total_activation = 0.0;
        let mut count = 0;

        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if let Some(state) = &self.sub_sub_density[primary][sub][sub_sub] {
                        total_activation += state.activation;
                        count += 1;
                    }
                }
            }
        }

        if count == 0 {
            return;
        }

        let average_activation = total_activation / count as Float;

        // Calculate variance
        let mut variance = 0.0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if let Some(state) = &self.sub_sub_density[primary][sub][sub_sub] {
                        let diff = state.activation - average_activation;
                        variance += diff * diff;
                    }
                }
            }
        }
        variance /= count as Float;

        // Update coherence
        let target_coherence = 1.0 / (1.0 + variance * 10.0);
        self.coherence = self.coherence * 0.9 + target_coherence * 0.1;

        // Update activation levels
        self.update_activation_levels();
    }

    /// Get total number of states
    pub fn total_states(&self) -> usize {
        343
    }

    /// Get number of initialized states
    pub fn active_states(&self) -> usize {
        let mut count = 0;
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if self.sub_sub_density[primary][sub][sub_sub].is_some() {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Get number of fractal connections
    pub fn total_connections(&self) -> usize {
        if let Some(connections) = &self.fractal_connections {
            connections.len()
        } else {
            0
        }
    }

    /// Get average activation level
    pub fn average_activation(&self) -> Float {
        let mut total = 0.0;
        let mut count = 0;

        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    if let Some(state) = &self.sub_sub_density[primary][sub][sub_sub] {
                        total += state.activation;
                        count += 1;
                    }
                }
            }
        }

        if count == 0 {
            0.0
        } else {
            total / count as Float
        }
    }

    /// Get coherence level
    pub fn get_coherence(&self) -> Float {
        self.coherence
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            cache_len: self.cache.len(),
            cache_capacity: self.cache.cap().get(),
            initialized_states: self.active_states(),
        }
    }
}

/// Reference to a state at all three levels (lazy version)
#[derive(Debug, Clone)]
pub struct StateReferenceLazy {
    pub primary: Arc<PrimaryDensityState>,
    pub sub: Arc<SubDensityState>,
    pub sub_sub: Arc<SubSubDensityState>,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cache_len: usize,
    pub cache_capacity: usize,
    pub initialized_states: usize,
}

impl Default for LazyHolographicContainer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_id_creation() {
        let state_id = StateID::new(0, 0, 0);
        assert!(state_id.is_some());
        assert_eq!(state_id.unwrap().primary, 0);
        assert_eq!(state_id.unwrap().sub, 0);
        assert_eq!(state_id.unwrap().sub_sub, 0);
    }

    #[test]
    fn test_state_id_invalid() {
        let state_id = StateID::new(7, 0, 0);
        assert!(state_id.is_none());

        let state_id = StateID::new(0, 7, 0);
        assert!(state_id.is_none());

        let state_id = StateID::new(0, 0, 7);
        assert!(state_id.is_none());
    }

    #[test]
    fn test_state_id_linear_index() {
        let state_id = StateID::new(0, 0, 0).unwrap();
        assert_eq!(state_id.linear_index(), 0);

        let state_id = StateID::new(6, 6, 6).unwrap();
        assert_eq!(state_id.linear_index(), 342);

        let state_id = StateID::new(1, 2, 3).unwrap();
        assert_eq!(state_id.linear_index(), 1 * 49 + 2 * 7 + 3);
    }

    #[test]
    fn test_state_id_from_linear_index() {
        let state_id = StateID::from_linear_index(0);
        assert!(state_id.is_some());
        assert_eq!(state_id.unwrap().primary, 0);
        assert_eq!(state_id.unwrap().sub, 0);
        assert_eq!(state_id.unwrap().sub_sub, 0);

        let state_id = StateID::from_linear_index(342);
        assert!(state_id.is_some());
        assert_eq!(state_id.unwrap().primary, 6);
        assert_eq!(state_id.unwrap().sub, 6);
        assert_eq!(state_id.unwrap().sub_sub, 6);

        let state_id = StateID::from_linear_index(343);
        assert!(state_id.is_none());
    }

    #[test]
    fn test_state_id_density() {
        let state_id = StateID::new(0, 0, 0).unwrap();
        assert_eq!(state_id.density(), Density::First);

        let state_id = StateID::new(3, 0, 0).unwrap();
        assert_eq!(state_id.density(), Density::Fourth);

        let state_id = StateID::new(6, 0, 0).unwrap();
        assert_eq!(state_id.density(), Density::Seventh);
    }

    #[test]
    fn test_primary_density_state_creation() {
        let state = PrimaryDensityState::new(Density::Fourth);
        assert_eq!(state.density, Density::Fourth);
        assert_eq!(state.consciousness_level, 0.0);
        assert_eq!(state.lambda, 0.0);
    }

    #[test]
    fn test_primary_density_state_initialize() {
        let mut state = PrimaryDensityState::new(Density::Fourth);
        state.initialize(1.0);
        assert!(state.consciousness_level > 0.0);
        assert!(state.lambda > 0.0);
        assert!(state.activation > 0.0);
    }

    #[test]
    fn test_sub_density_state_creation() {
        let state = SubDensityState::new(3, Density::Fourth);
        assert_eq!(state.sub_density, 3);
        assert_eq!(state.developmental_progress, 0.0);
    }

    #[test]
    fn test_sub_density_state_initialize() {
        let mut state = SubDensityState::new(3, Density::Fourth);
        state.initialize(0.5);
        assert!(state.developmental_progress > 0.0);
        assert!(state.lambda > 0.0);
        assert!(state.activation > 0.0);
    }

    #[test]
    fn test_sub_density_state_add_experience() {
        let mut state = SubDensityState::new(3, Density::Fourth);
        let experience = Experience {
            experience_type: "Test".to_string(),
            intensity: 0.5,
            learning_value: 0.7,
            timestamp: 100.0,
        };
        state.add_experience(experience);
        assert_eq!(state.experiences.len(), 1);
    }

    #[test]
    fn test_sub_sub_density_state_creation() {
        let state = SubSubDensityState::new(3);
        assert_eq!(state.sub_sub_density, 3);
        assert_eq!(state.connection_strengths.len(), 343);
    }

    #[test]
    fn test_sub_sub_density_state_initialize() {
        let mut state = SubSubDensityState::new(3);
        state.initialize(0.3);
        assert!(state.activation > 0.0);
        assert!(state.lambda > 0.0);
    }

    #[test]
    fn test_sub_sub_density_state_connection_strength() {
        let mut state = SubSubDensityState::new(0);
        state.initialize(0.3);
        let other_id = StateID::new(1, 1, 1).unwrap();
        let strength = state.get_connection_strength(&other_id);
        assert!(strength >= 0.0 && strength <= 1.0);
    }

    #[test]
    fn test_fractal_connection_creation() {
        let from = StateID::new(0, 0, 0).unwrap();
        let to = StateID::new(1, 1, 1).unwrap();
        let connection = FractalConnection::new(from, to, ConnectionType::Hierarchical);
        assert_eq!(connection.from_state, from);
        assert_eq!(connection.to_state, to);
    }

    #[test]
    fn test_fractal_connection_update_coherence() {
        let from = StateID::new(0, 0, 0).unwrap();
        let to = StateID::new(1, 1, 1).unwrap();
        let mut connection = FractalConnection::new(from, to, ConnectionType::Hierarchical);
        let old_coherence = connection.coherence;
        connection.update_coherence(0.2);
        assert_ne!(connection.coherence, old_coherence);
    }

    #[test]
    fn test_fractal_connection_strength() {
        let from = StateID::new(0, 0, 0).unwrap();
        let to = StateID::new(1, 1, 1).unwrap();
        let connection = FractalConnection::new(from, to, ConnectionType::Hierarchical);
        let strength = connection.strength();
        assert!(strength >= 0.0 && strength <= 1.0);
    }

    #[test]
    fn test_holographic_container_creation() {
        let container = HolographicContainer::new();
        assert_eq!(container.total_states(), 343);
        assert_eq!(container.primary_density.len(), 7);
    }

    #[test]
    fn test_holographic_container_initialization() {
        let container = HolographicContainer::new();
        // Check that all states are initialized
        for primary in 0..7 {
            assert!(container.primary_density[primary].consciousness_level > 0.0);
            for sub in 0..7 {
                assert!(container.sub_density[primary][sub].developmental_progress >= 0.0);
                for sub_sub in 0..7 {
                    assert!(container.sub_sub_density[primary][sub][sub_sub].activation >= 0.0);
                }
            }
        }
    }

    #[test]
    fn test_holographic_container_get_primary_density() {
        let container = HolographicContainer::new();
        let state = container.get_primary_density(3);
        assert!(state.is_some());
        assert_eq!(state.unwrap().density, Density::Fourth);

        let state = container.get_primary_density(7);
        assert!(state.is_none());
    }

    #[test]
    fn test_holographic_container_get_sub_density() {
        let container = HolographicContainer::new();
        let state = container.get_sub_density(3, 2);
        assert!(state.is_some());
        assert_eq!(state.unwrap().sub_density, 2);

        let state = container.get_sub_density(7, 0);
        assert!(state.is_none());
    }

    #[test]
    fn test_holographic_container_get_sub_sub_density() {
        let container = HolographicContainer::new();
        let state = container.get_sub_sub_density(3, 2, 1);
        assert!(state.is_some());
        assert_eq!(state.unwrap().sub_sub_density, 1);

        let state = container.get_sub_sub_density(7, 0, 0);
        assert!(state.is_none());
    }

    #[test]
    fn test_holographic_container_get_state() {
        let container = HolographicContainer::new();
        let state_id = StateID::new(3, 2, 1).unwrap();
        let state = container.get_state(&state_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().primary.density, Density::Fourth);
    }

    #[test]
    fn test_holographic_container_propagate_change() {
        let mut container = HolographicContainer::new();
        let state_id = StateID::new(3, 2, 1).unwrap();
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.5,
            affected_attributes: vec!["activation".to_string()],
        };

        let old_activation = container.sub_sub_density[3][2][1].activation;
        container.propagate_change(state_id, change);
        let new_activation = container.sub_sub_density[3][2][1].activation;

        assert!(new_activation > old_activation);
    }

    #[test]
    fn test_holographic_container_maintain_coherence() {
        let mut container = HolographicContainer::new();
        let old_coherence = container.coherence;
        container.maintain_coherence();
        // Coherence should be close to 1.0 after initialization
        assert!((container.coherence - old_coherence).abs() < 0.1);
    }

    #[test]
    fn test_holographic_container_total_states() {
        let container = HolographicContainer::new();
        assert_eq!(container.total_states(), 343);
    }

    #[test]
    fn test_holographic_container_active_states() {
        let container = HolographicContainer::new();
        let active = container.active_states();
        assert!(active > 0 && active <= 343);
    }

    #[test]
    fn test_holographic_container_average_activation() {
        let container = HolographicContainer::new();
        let avg = container.average_activation();
        assert!(avg >= 0.0 && avg <= 1.0);
    }

    #[test]
    fn test_holographic_container_get_coherence() {
        let container = HolographicContainer::new();
        let coherence = container.get_coherence();
        assert!(coherence >= 0.0 && coherence <= 1.0);
    }

    #[test]
    fn test_holographic_container_reset() {
        let mut container = HolographicContainer::new();
        // Modify some states
        let state_id = StateID::new(3, 2, 1).unwrap();
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.5,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(state_id, change);

        // Reset
        container.reset();

        // Check that coherence is back to 1.0
        assert_eq!(container.coherence, 1.0);
    }

    #[test]
    fn test_holographic_container_get_statistics() {
        let container = HolographicContainer::new();
        let stats = container.get_statistics();
        assert_eq!(stats.total_states, 343);
        assert!(stats.active_states > 0);
        assert!(stats.inactive_states >= 0);
        assert!(stats.average_activation >= 0.0 && stats.average_activation <= 1.0);
        assert!(stats.average_lambda >= 0.0);
        assert!(stats.coherence >= 0.0 && stats.coherence <= 1.0);
        assert!(stats.total_connections > 0);
    }

    #[test]
    fn test_343_states_created() {
        let container = HolographicContainer::new();
        assert_eq!(container.total_states(), 343);

        // Verify all states exist
        for primary in 0..7 {
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    let state_id = StateID::new(primary as u8, sub as u8, sub_sub as u8).unwrap();
                    let state = container.get_state(&state_id);
                    assert!(state.is_some(), "State {} should exist", state_id);
                }
            }
        }
    }

    #[test]
    fn test_holographic_propagation() {
        let mut container = HolographicContainer::new();

        // Get initial activation of a distant state
        let source_id = StateID::new(0, 0, 0).unwrap();
        let target_id = StateID::new(6, 6, 6).unwrap();
        let initial_activation = container.sub_sub_density[6][6][6].activation;

        // Apply change to source
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 1.0,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(source_id, change);

        // Check that target was affected (though less strongly)
        let final_activation = container.sub_sub_density[6][6][6].activation;
        // The target should be affected due to holographic propagation
        // (though the effect will be smaller due to distance)
        assert!(
            (final_activation - initial_activation).abs() > 0.0,
            "Target state should be affected by holographic propagation"
        );
    }

    #[test]
    fn test_coherence_maintenance() {
        let mut container = HolographicContainer::new();

        // Apply multiple changes to create variance
        for i in 0..10 {
            let state_id = StateID::new(i % 7, (i * 2) % 7, (i * 3) % 7).unwrap();
            let change = StateChange {
                change_type: ChangeType::Activation,
                magnitude: 0.5,
                affected_attributes: vec!["activation".to_string()],
            };
            container.propagate_change(state_id, change);
        }

        // Maintain coherence
        container.maintain_coherence();

        // Coherence should still be reasonable
        assert!(container.coherence > 0.5);
    }

    #[test]
    fn test_self_similarity() {
        let container = HolographicContainer::new();

        // Check that patterns repeat at different levels
        // For example, activation should increase with density level
        let mut previous_avg = 0.0;
        for primary in 0..7 {
            let mut sum = 0.0;
            let mut count = 0;
            for sub in 0..7 {
                for sub_sub in 0..7 {
                    sum += container.sub_sub_density[primary][sub][sub_sub].activation;
                    count += 1;
                }
            }
            let avg = sum / count as Float;
            // Higher densities should have higher activation
            assert!(avg >= previous_avg * 0.9); // Allow some variance
            previous_avg = avg;
        }
    }

    #[test]
    fn test_fractal_connections_created() {
        let container = HolographicContainer::new();
        assert!(container.total_connections() > 0);

        // Verify connections have valid from/to states
        for connection in &container.fractal_connections {
            assert!(connection.from_state.primary < 7);
            assert!(connection.from_state.sub < 7);
            assert!(connection.from_state.sub_sub < 7);
            assert!(connection.to_state.primary < 7);
            assert!(connection.to_state.sub < 7);
            assert!(connection.to_state.sub_sub < 7);
        }
    }

    #[test]
    fn test_activation_levels_updated() {
        let mut container = HolographicContainer::new();

        // Apply a change
        let state_id = StateID::new(3, 2, 1).unwrap();
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.5,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(state_id, change);

        // Check that activation levels were updated
        let expected_activation = container.sub_sub_density[3][2][1].activation;
        let actual_activation = container.activation_levels[3][2][1];
        assert_eq!(expected_activation, actual_activation);
    }

    #[test]
    fn test_hierarchical_distance_calculation() {
        let container = HolographicContainer::new();
        let id1 = StateID::new(0, 0, 0).unwrap();
        let id2 = StateID::new(1, 1, 1).unwrap();
        let id3 = StateID::new(6, 6, 6).unwrap();

        let dist12 = container.calculate_hierarchical_distance(&id1, &id2);
        let dist13 = container.calculate_hierarchical_distance(&id1, &id3);

        // Distance should increase with separation
        assert!(dist13 > dist12);
    }

    #[test]
    fn test_information_flow() {
        let flow = HolographicInformationFlow {
            direction: HolographicFlowDirection::Bidirectional,
            content: vec![1, 2, 3],
            intensity: 0.7,
        };

        assert_eq!(flow.direction, HolographicFlowDirection::Bidirectional);
        assert_eq!(flow.intensity, 0.7);
        assert_eq!(flow.content.len(), 3);
    }

    #[test]
    fn test_experience_quality() {
        let quality1 = ExperienceQuality::Positive;
        let quality2 = ExperienceQuality::Negative;
        let quality3 = ExperienceQuality::Neutral;
        let quality4 = ExperienceQuality::Mixed;

        assert_ne!(quality1, quality2);
        assert_ne!(quality2, quality3);
        assert_ne!(quality3, quality4);
    }

    #[test]
    fn test_connection_type() {
        let ct1 = ConnectionType::Hierarchical;
        let ct2 = ConnectionType::Peer;
        let ct3 = ConnectionType::CrossLevel;
        let ct4 = ConnectionType::Holographic;

        assert_ne!(ct1, ct2);
        assert_ne!(ct2, ct3);
        assert_ne!(ct3, ct4);
    }

    #[test]
    fn test_change_type() {
        let ct1 = ChangeType::Activation;
        let ct2 = ChangeType::Lambda;
        let ct3 = ChangeType::Experience;
        let ct4 = ChangeType::Coherence;
        let ct5 = ChangeType::ArchetypeInfluence;

        assert_ne!(ct1, ct2);
        assert_ne!(ct2, ct3);
        assert_ne!(ct3, ct4);
        assert_ne!(ct4, ct5);
    }

    #[test]
    fn test_manifestation_type() {
        let mt1 = ManifestationType::None;
        let mt2 = ManifestationType::Light;
        let mt3 = ManifestationType::Chemical;
        let mt4 = ManifestationType::Social;
        let mt5 = ManifestationType::Spiritual;

        assert_ne!(mt1, mt2);
        assert_ne!(mt2, mt3);
        assert_ne!(mt3, mt4);
        assert_ne!(mt4, mt5);
    }

    // ============================================================================
    // LAZY LOADING TESTS
    // ============================================================================

    #[test]
    fn test_lazy_holographic_container_creation() {
        let container = LazyHolographicContainer::new();
        assert_eq!(container.total_states(), 343);
        assert_eq!(container.active_states(), 0); // No states initialized yet
    }

    #[test]
    fn test_lazy_primary_density_initialization() {
        let mut container = LazyHolographicContainer::new();

        // Initially, no states are initialized
        assert_eq!(container.active_states(), 0);

        // Access a primary density state
        let state = container.get_primary_density(3);
        assert!(state.is_some());
        assert_eq!(state.unwrap().density, Density::Fourth);

        // Still no sub-sub-density states initialized
        assert_eq!(container.active_states(), 0);
    }

    #[test]
    fn test_lazy_sub_density_initialization() {
        let mut container = LazyHolographicContainer::new();

        // Initially, no states are initialized
        assert_eq!(container.active_states(), 0);

        // Access a sub-density state
        let state = container.get_sub_density(3, 2);
        assert!(state.is_some());
        assert_eq!(state.unwrap().sub_density, 2);

        // Still no sub-sub-density states initialized
        assert_eq!(container.active_states(), 0);
    }

    #[test]
    fn test_lazy_sub_sub_density_initialization() {
        let mut container = LazyHolographicContainer::new();

        // Initially, no states are initialized
        assert_eq!(container.active_states(), 0);

        // Access a sub-sub-density state
        let state = container.get_sub_sub_density(3, 2, 1);
        assert!(state.is_some());
        assert_eq!(state.unwrap().sub_sub_density, 1);

        // Now one state is initialized
        assert_eq!(container.active_states(), 1);
    }

    #[test]
    fn test_lazy_state_access_multiple() {
        let mut container = LazyHolographicContainer::new();

        // Access multiple states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(1, 1, 1);
        let _ = container.get_sub_sub_density(2, 2, 2);

        assert_eq!(container.active_states(), 3);
    }

    #[test]
    fn test_lazy_state_reuse() {
        let mut container = LazyHolographicContainer::new();

        // Access the same state twice
        let state1 = container.get_sub_sub_density(3, 2, 1);
        assert!(state1.is_some());

        // Drop the first reference before accessing again
        drop(state1);

        let state2 = container.get_sub_sub_density(3, 2, 1);
        assert!(state2.is_some());

        // Should still only have one initialized state
        assert_eq!(container.active_states(), 1);
    }

    #[test]
    fn test_lazy_get_state() {
        let mut container = LazyHolographicContainer::new();

        let state_id = StateID::new(3, 2, 1).unwrap();
        let state = container.get_state(&state_id);

        assert!(state.is_some());
        let state = state.unwrap();
        assert_eq!(state.primary.density, Density::Fourth);
        assert_eq!(state.sub.sub_density, 2);
        assert_eq!(state.sub_sub.sub_sub_density, 1);

        // Should initialize the sub-sub-density state
        assert_eq!(container.active_states(), 1);
    }

    #[test]
    fn test_lazy_propagate_change() {
        let mut container = LazyHolographicContainer::new();

        // Initialize a few states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(1, 1, 1);
        let _ = container.get_sub_sub_density(2, 2, 2);

        assert_eq!(container.active_states(), 3);

        // Propagate a change to one of them
        let state_id = StateID::new(0, 0, 0).unwrap();
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.5,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(state_id, change);

        // Should still only have 3 initialized states (no new ones created)
        assert_eq!(container.active_states(), 3);
    }

    #[test]
    fn test_lazy_maintain_coherence() {
        let mut container = LazyHolographicContainer::new();

        // Initialize a few states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(1, 1, 1);
        let _ = container.get_sub_sub_density(2, 2, 2);

        let old_coherence = container.get_coherence();
        container.maintain_coherence();

        // Coherence should be close to 1.0
        assert!((container.get_coherence() - old_coherence).abs() < 0.1);
    }

    #[test]
    fn test_lazy_average_activation() {
        let mut container = LazyHolographicContainer::new();

        // Initially, average activation is 0.0 (no states initialized)
        assert_eq!(container.average_activation(), 0.0);

        // Initialize some states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(1, 1, 1);
        let _ = container.get_sub_sub_density(2, 2, 2);

        // Average activation should now be > 0.0
        assert!(container.average_activation() > 0.0);
    }

    #[test]
    fn test_lazy_cache_stats() {
        let mut container = LazyHolographicContainer::new();

        let stats = container.cache_stats();
        assert_eq!(stats.cache_len, 0);
        assert_eq!(stats.initialized_states, 0);

        // Access some states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(1, 1, 1);
        let _ = container.get_sub_sub_density(2, 2, 2);

        let stats = container.cache_stats();
        assert_eq!(stats.cache_len, 3);
        assert_eq!(stats.initialized_states, 3);
    }

    #[test]
    fn test_lazy_vs_eager_consistency() {
        // Create both containers
        let mut lazy_container = LazyHolographicContainer::new();
        let eager_container = HolographicContainer::new();

        // Initialize same state in lazy container
        let state_id = StateID::new(3, 2, 1).unwrap();
        let lazy_state = lazy_container.get_state(&state_id).unwrap();
        let eager_state = eager_container.get_state(&state_id).unwrap();

        // Check that they have the same density
        assert_eq!(lazy_state.primary.density, eager_state.primary.density);
        assert_eq!(lazy_state.sub.sub_density, eager_state.sub.sub_density);
        assert_eq!(
            lazy_state.sub_sub.sub_sub_density,
            eager_state.sub_sub.sub_sub_density
        );
    }

    #[test]
    fn test_lazy_invalid_access() {
        let mut container = LazyHolographicContainer::new();

        // Try to access invalid states
        assert!(container.get_primary_density(7).is_none());
        assert!(container.get_sub_density(7, 0).is_none());
        assert!(container.get_sub_density(0, 7).is_none());
        assert!(container.get_sub_sub_density(7, 0, 0).is_none());
        assert!(container.get_sub_sub_density(0, 7, 0).is_none());
        assert!(container.get_sub_sub_density(0, 0, 7).is_none());
    }

    #[test]
    fn test_lazy_performance_correctness() {
        let mut container = LazyHolographicContainer::new();

        // Initialize 10 states
        for i in 0..10 {
            let primary = (i % 7) as u8;
            let sub = ((i / 7) % 7) as u8;
            let sub_sub = ((i / 49) % 7) as u8;
            let _ = container.get_sub_sub_density(primary, sub, sub_sub);
        }

        assert_eq!(container.active_states(), 10);

        // Apply changes to all initialized states
        for i in 0..10 {
            let primary = (i % 7) as u8;
            let sub = ((i / 7) % 7) as u8;
            let sub_sub = ((i / 49) % 7) as u8;
            let state_id = StateID::new(primary, sub, sub_sub).unwrap();
            let change = StateChange {
                change_type: ChangeType::Activation,
                magnitude: 0.1,
                affected_attributes: vec!["activation".to_string()],
            };
            container.propagate_change(state_id, change);
        }

        // Should still have only 10 initialized states
        assert_eq!(container.active_states(), 10);

        // Average activation should have increased
        assert!(container.average_activation() > 0.0);
    }

    #[test]
    fn test_lazy_activation_levels() {
        let mut container = LazyHolographicContainer::new();

        // Initialize a state
        let state_id = StateID::new(3, 2, 1).unwrap();
        let initial_state = container.get_sub_sub_density(3, 2, 1).unwrap();
        let initial_activation = initial_state.activation;

        // Apply a change
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 0.5,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(state_id, change);

        // Get the state again
        let updated_state = container.get_sub_sub_density(3, 2, 1).unwrap();
        let updated_activation = updated_state.activation;

        // Activation should have increased
        assert!(updated_activation > initial_activation);
    }

    #[test]
    fn test_lazy_holographic_propagation_limited() {
        let mut container = LazyHolographicContainer::new();

        // Initialize only a few states
        let _ = container.get_sub_sub_density(0, 0, 0);
        let _ = container.get_sub_sub_density(6, 6, 6);

        assert_eq!(container.active_states(), 2);

        // Propagate a change from state (0,0,0)
        let source_id = StateID::new(0, 0, 0).unwrap();
        let change = StateChange {
            change_type: ChangeType::Activation,
            magnitude: 1.0,
            affected_attributes: vec!["activation".to_string()],
        };
        container.propagate_change(source_id, change);

        // Should still only have 2 initialized states
        // (propagation doesn't create new states, only affects initialized ones)
        assert_eq!(container.active_states(), 2);
    }
}
