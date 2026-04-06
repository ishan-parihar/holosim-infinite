//! Density-Specific Mechanics - Unique gameplay mechanics for all 8 densities
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 6: Density-Based Gaming:
//! "Each density has unique mechanics, not just visual differences."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each density has its own unique experience, with its own laws, its own rules of physics,
//! and its own mode of consciousness."
//!
//! This module provides:
//! - DensityMechanics: Enum for all 8 density-specific gameplay systems
//! - Unique mechanics for each density (1st through 8th)
//! - Integration with ArchetypeActivationProfile for behavior derivation
//! - Integration with ArchetypeInterferenceCache for performance
//! - Perception filters, action spaces, and interaction rules per density

use crate::simulation_v3::archetype_basis::ArchetypeActivationProfile;
use crate::simulation_v3::archetype_interference_cache::ArchetypeInterferenceCache;
use crate::simulation_v3::archetype_interference_cache::ArchetypicalInterference;
use crate::types::Float;
use std::collections::HashMap;

/// Number of densities in the octave
pub const NUM_DENSITIES: usize = 8;

/// Maximum catalyst intensity (0.0 to 1.0)
pub const MAX_CATALYST_INTENSITY: Float = 1.0;

/// Minimum veil transparency (0.0 = opaque, 1.0 = fully transparent)
pub const MIN_VEIL_TRANSPARENCY: Float = 0.0;

/// Maximum veil transparency
pub const MAX_VEIL_TRANSPARENCY: Float = 1.0;

/// Density levels in the octave
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Density {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Eighth = 8,
}

impl Density {
    /// First density: Elements (Red Ray)
    pub const FIRST: Density = Density::First;
    /// Second density: Biological life (Orange Ray)
    pub const SECOND: Density = Density::Second;
    /// Third density: Self-aware consciousness (Yellow Ray)
    pub const THIRD: Density = Density::Third;
    /// Fourth density: Love/understanding (Green Ray)
    pub const FOURTH: Density = Density::Fourth;
    /// Fifth density: Light/wisdom (Blue Ray)
    pub const FIFTH: Density = Density::Fifth;
    /// Sixth density: Unity/one-with-all (Indigo Ray)
    pub const SIXTH: Density = Density::Sixth;
    /// Seventh density: Gateway to Intelligent Infinity (Violet Ray)
    pub const SEVENTH: Density = Density::Seventh;
    /// Eighth density: Return to Intelligent Infinity
    pub const EIGHTH: Density = Density::Eighth;

    /// Get density number
    pub fn value(&self) -> usize {
        *self as usize
    }

    /// Get veil transparency for this density
    pub fn veil_transparency(&self) -> Float {
        match self {
            Density::First => 0.0,
            Density::Second => 0.0,
            Density::Third => 0.1,
            Density::Fourth => 0.3,
            Density::Fifth => 0.6,
            Density::Sixth => 1.0,
            Density::Seventh => 1.0,
            Density::Eighth => 1.0,
        }
    }

    /// Get spectrum access level for this density
    pub fn spectrum_access(&self) -> Float {
        match self {
            Density::First => 0.05,
            Density::Second => 0.1,
            Density::Third => 0.2,
            Density::Fourth => 0.4,
            Density::Fifth => 0.7,
            Density::Sixth => 0.9,
            Density::Seventh => 0.95,
            Density::Eighth => 1.0,
        }
    }

    /// Get density name
    pub fn name(&self) -> &str {
        match self {
            Density::First => "First Density",
            Density::Second => "Second Density",
            Density::Third => "Third Density",
            Density::Fourth => "Fourth Density",
            Density::Fifth => "Fifth Density",
            Density::Sixth => "Sixth Density",
            Density::Seventh => "Seventh Density",
            Density::Eighth => "Eighth Density",
        }
    }

    /// Get density ray color
    pub fn ray_color(&self) -> &str {
        match self {
            Density::First => "Red",
            Density::Second => "Orange",
            Density::Third => "Yellow",
            Density::Fourth => "Green",
            Density::Fifth => "Blue",
            Density::Sixth => "Indigo",
            Density::Seventh => "Violet",
            Density::Eighth => "White",
        }
    }

    /// Get density from u8 value
    pub fn from_u8(value: u8) -> Density {
        match value {
            1 => Density::First,
            2 => Density::Second,
            3 => Density::Third,
            4 => Density::Fourth,
            5 => Density::Fifth,
            6 => Density::Sixth,
            7 => Density::Seventh,
            8 => Density::Eighth,
            _ => Density::Third,
        }
    }

    /// Create a new Density, clamped to valid range (1-8)
    pub fn new(value: u8) -> Self {
        Self::from_u8(value.clamp(1, 8))
    }

    /// Get the display name for this density (alias for name())
    pub fn display_name(&self) -> &str {
        self.name()
    }

    /// Get the density as usize for array indexing
    pub fn as_usize(&self) -> usize {
        self.value()
    }

    /// Check if this density is harvestable (can transition to next density)
    /// 3rd density is the density of choice - requires 51%+ STO or 95%+ STS
    pub fn is_harvestable(&self) -> bool {
        self.value() >= 3
    }
}

impl std::fmt::Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Perception filter - what the entity can perceive based on density
#[derive(Debug, Clone, PartialEq)]
pub struct PerceptionFilter {
    pub veil_transparency: Float,
    pub spectrum_access: Float,
    pub physical_dominant: bool,
    pub telepathy_enabled: bool,
    pub collective_awareness: Float,
    pub time_space_access: Float,
}

impl PerceptionFilter {
    /// Create new perception filter
    pub fn new(
        veil_transparency: Float,
        spectrum_access: Float,
        physical_dominant: bool,
        telepathy_enabled: bool,
        collective_awareness: Float,
        time_space_access: Float,
    ) -> Self {
        PerceptionFilter {
            veil_transparency,
            spectrum_access,
            physical_dominant,
            telepathy_enabled,
            collective_awareness,
            time_space_access,
        }
    }

    /// Create default perception filter for a density
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First => PerceptionFilter {
                veil_transparency: 0.0,
                spectrum_access: 0.05,
                physical_dominant: true,
                telepathy_enabled: false,
                collective_awareness: 0.0,
                time_space_access: 0.0,
            },
            Density::Second => PerceptionFilter {
                veil_transparency: 0.0,
                spectrum_access: 0.1,
                physical_dominant: true,
                telepathy_enabled: false,
                collective_awareness: 0.1,
                time_space_access: 0.0,
            },
            Density::Third => PerceptionFilter {
                veil_transparency: 0.1,
                spectrum_access: 0.2,
                physical_dominant: true,
                telepathy_enabled: false,
                collective_awareness: 0.2,
                time_space_access: 0.05,
            },
            Density::Fourth => PerceptionFilter {
                veil_transparency: 0.3,
                spectrum_access: 0.4,
                physical_dominant: false,
                telepathy_enabled: true,
                collective_awareness: 0.6,
                time_space_access: 0.3,
            },
            Density::Fifth => PerceptionFilter {
                veil_transparency: 0.6,
                spectrum_access: 0.7,
                physical_dominant: false,
                telepathy_enabled: true,
                collective_awareness: 0.8,
                time_space_access: 0.6,
            },
            Density::Sixth => PerceptionFilter {
                veil_transparency: 1.0,
                spectrum_access: 0.9,
                physical_dominant: false,
                telepathy_enabled: true,
                collective_awareness: 1.0,
                time_space_access: 0.9,
            },
            Density::Seventh => PerceptionFilter {
                veil_transparency: 1.0,
                spectrum_access: 0.95,
                physical_dominant: false,
                telepathy_enabled: true,
                collective_awareness: 1.0,
                time_space_access: 0.95,
            },
            Density::Eighth => PerceptionFilter {
                veil_transparency: 1.0,
                spectrum_access: 1.0,
                physical_dominant: false,
                telepathy_enabled: true,
                collective_awareness: 1.0,
                time_space_access: 1.0,
            },
        }
    }
}

/// Action space - what actions are available to the entity
#[derive(Debug, Clone, PartialEq)]
pub struct ActionSpace {
    pub physical_actions: Vec<String>,
    pub mental_actions: Vec<String>,
    pub spiritual_actions: Vec<String>,
    pub collective_actions: Vec<String>,
}

impl ActionSpace {
    /// Create new action space
    pub fn new(
        physical_actions: Vec<String>,
        mental_actions: Vec<String>,
        spiritual_actions: Vec<String>,
        collective_actions: Vec<String>,
    ) -> Self {
        ActionSpace {
            physical_actions,
            mental_actions,
            spiritual_actions,
            collective_actions,
        }
    }

    /// Create default action space for a density
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First => ActionSpace {
                physical_actions: vec![
                    "form_bond".to_string(),
                    "release_bond".to_string(),
                    "stabilize_matter".to_string(),
                ],
                mental_actions: vec![],
                spiritual_actions: vec![],
                collective_actions: vec![],
            },
            Density::Second => ActionSpace {
                physical_actions: vec![
                    "move".to_string(),
                    "consume".to_string(),
                    "reproduce".to_string(),
                    "evolve".to_string(),
                ],
                mental_actions: vec!["instinct_response".to_string()],
                spiritual_actions: vec![],
                collective_actions: vec!["group_herd".to_string()],
            },
            Density::Third => ActionSpace {
                physical_actions: vec![
                    "move".to_string(),
                    "craft".to_string(),
                    "build".to_string(),
                    "trade".to_string(),
                ],
                mental_actions: vec![
                    "think".to_string(),
                    "choose_polarity".to_string(),
                    "process_catalyst".to_string(),
                ],
                spiritual_actions: vec!["meditate".to_string(), "seek_meaning".to_string()],
                collective_actions: vec!["form_group".to_string(), "cooperate".to_string()],
            },
            Density::Fourth => ActionSpace {
                physical_actions: vec!["manifest_light_body".to_string()],
                mental_actions: vec![
                    "think".to_string(),
                    "integrate_polarities".to_string(),
                    "understand_love".to_string(),
                ],
                spiritual_actions: vec![
                    "channel_love".to_string(),
                    "practice_compassion".to_string(),
                ],
                collective_actions: vec![
                    "telepathic_communication".to_string(),
                    "form_collective".to_string(),
                    "merge_consciousness".to_string(),
                ],
            },
            Density::Fifth => ActionSpace {
                physical_actions: vec![
                    "manifest_light_body".to_string(),
                    "travel_instant".to_string(),
                ],
                mental_actions: vec![
                    "teach".to_string(),
                    "learn".to_string(),
                    "create_wisdom".to_string(),
                ],
                spiritual_actions: vec![
                    "channel_light".to_string(),
                    "activate_wisdom_center".to_string(),
                ],
                collective_actions: vec![
                    "telepathic_communication".to_string(),
                    "form_social_memory".to_string(),
                    "teach_collective".to_string(),
                ],
            },
            Density::Sixth => ActionSpace {
                physical_actions: vec![
                    "manifest_light_body".to_string(),
                    "travel_instant".to_string(),
                    "shape_shift".to_string(),
                ],
                mental_actions: vec!["balance_polarities".to_string(), "unify_all".to_string()],
                spiritual_actions: vec![
                    "channel_unity".to_string(),
                    "activate_all_centers".to_string(),
                ],
                collective_actions: vec![
                    "merge_into_social_memory".to_string(),
                    "access_all_experiences".to_string(),
                    "create_collective_wisdom".to_string(),
                ],
            },
            Density::Seventh => ActionSpace {
                physical_actions: vec![
                    "manifest_violet_body".to_string(),
                    "transcend_form".to_string(),
                ],
                mental_actions: vec![
                    "prepare_gateway".to_string(),
                    "integrate_all_densities".to_string(),
                ],
                spiritual_actions: vec![
                    "activate_violet_ray".to_string(),
                    "access_intelligent_infinity".to_string(),
                ],
                collective_actions: vec![
                    "unify_all_social_memories".to_string(),
                    "prepare_next_octave".to_string(),
                ],
            },
            Density::Eighth => ActionSpace {
                physical_actions: vec!["transcend_all_form".to_string()],
                mental_actions: vec!["merge_with_infinity".to_string()],
                spiritual_actions: vec!["become_intelligent_infinity".to_string()],
                collective_actions: vec!["unify_all_octaves".to_string()],
            },
        }
    }
}

/// Interaction rules - how entities interact based on density
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionRules {
    pub can_harm: bool,
    pub can_heal: bool,
    pub can_communicate: bool,
    pub telepathic_communication: bool,
    pub collective_merge: bool,
    pub polarity_conflict: bool,
}

impl InteractionRules {
    /// Create new interaction rules
    pub fn new(
        can_harm: bool,
        can_heal: bool,
        can_communicate: bool,
        telepathic_communication: bool,
        collective_merge: bool,
        polarity_conflict: bool,
    ) -> Self {
        InteractionRules {
            can_harm,
            can_heal,
            can_communicate,
            telepathic_communication,
            collective_merge,
            polarity_conflict,
        }
    }

    /// Create default interaction rules for a density
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First => InteractionRules {
                can_harm: false,
                can_heal: false,
                can_communicate: false,
                telepathic_communication: false,
                collective_merge: false,
                polarity_conflict: false,
            },
            Density::Second => InteractionRules {
                can_harm: true,
                can_heal: false,
                can_communicate: false,
                telepathic_communication: false,
                collective_merge: false,
                polarity_conflict: false,
            },
            Density::Third => InteractionRules {
                can_harm: true,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: false,
                collective_merge: false,
                polarity_conflict: true,
            },
            Density::Fourth => InteractionRules {
                can_harm: false,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: true,
                collective_merge: true,
                polarity_conflict: false,
            },
            Density::Fifth => InteractionRules {
                can_harm: false,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: true,
                collective_merge: true,
                polarity_conflict: false,
            },
            Density::Sixth => InteractionRules {
                can_harm: false,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: true,
                collective_merge: true,
                polarity_conflict: false,
            },
            Density::Seventh => InteractionRules {
                can_harm: false,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: true,
                collective_merge: true,
                polarity_conflict: false,
            },
            Density::Eighth => InteractionRules {
                can_harm: false,
                can_heal: true,
                can_communicate: true,
                telepathic_communication: true,
                collective_merge: true,
                polarity_conflict: false,
            },
        }
    }
}

/// Polarity choice (Service-to-Others or Service-to-Self)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    ServiceToOthers,
    ServiceToSelf,
    Undecided,
    /// Balanced: Neither strongly STO nor STS (added for compatibility with advanced_game_mechanics)
    Balanced,
}

impl Polarity {
    /// Check if this is Service-to-Others polarity
    pub fn is_sto(&self) -> bool {
        matches!(self, Polarity::ServiceToOthers)
    }

    /// Check if this is Service-to-Self polarity
    pub fn is_sts(&self) -> bool {
        matches!(self, Polarity::ServiceToSelf)
    }

    /// Check if this is balanced polarity
    pub fn is_balanced(&self) -> bool {
        matches!(self, Polarity::Balanced) || matches!(self, Polarity::Undecided)
    }

    /// Get the polarity value (-1.0 for STS, 0.0 for Balanced/Undecided, 1.0 for STO)
    pub fn as_f64(&self) -> f64 {
        match self {
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => -1.0,
            Polarity::Balanced | Polarity::Undecided => 0.0,
        }
    }

    /// Get the display name for this polarity
    pub fn display_name(&self) -> &str {
        match self {
            Polarity::ServiceToOthers => "Service-to-Others",
            Polarity::ServiceToSelf => "Service-to-Self",
            Polarity::Balanced => "Balanced",
            Polarity::Undecided => "Undecided",
        }
    }

    /// Check if two polarities are compatible
    /// STO and STS are incompatible, Balanced/Undecided is compatible with both
    pub fn is_compatible_with(&self, other: &Polarity) -> bool {
        !matches!(
            (self, other),
            (Polarity::ServiceToOthers, Polarity::ServiceToSelf)
                | (Polarity::ServiceToSelf, Polarity::ServiceToOthers)
        )
    }

    /// Calculate compatibility score between two polarities
    /// Returns 1.0 for same polarity, 0.5 for balanced/undecided, 0.0 for opposing
    pub fn compatibility_with(&self, other: &Polarity) -> f64 {
        match (self, other) {
            (Polarity::ServiceToOthers, Polarity::ServiceToOthers) => 1.0,
            (Polarity::ServiceToSelf, Polarity::ServiceToSelf) => 1.0,
            (Polarity::Balanced, _) | (_, Polarity::Balanced) => 0.5,
            (Polarity::Undecided, _) | (_, Polarity::Undecided) => 0.5,
            _ => 0.0,
        }
    }
}

impl std::fmt::Display for Polarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Catalyst for evolution
#[derive(Debug, Clone, PartialEq)]
pub struct Catalyst {
    pub intensity: Float,
    pub polarity_affinity: Option<Polarity>,
    pub source: String,
}

impl Catalyst {
    /// Create new catalyst
    pub fn new(intensity: Float, polarity_affinity: Option<Polarity>, source: String) -> Self {
        Catalyst {
            intensity: intensity.clamp(0.0, MAX_CATALYST_INTENSITY),
            polarity_affinity,
            source,
        }
    }

    /// Create random catalyst
    pub fn random(intensity: Float) -> Self {
        Catalyst {
            intensity: intensity.clamp(0.0, MAX_CATALYST_INTENSITY),
            polarity_affinity: None,
            source: "random".to_string(),
        }
    }
}

/// 1st Density Mechanics - Matter Awareness
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "1st Density: Awareness as matter - no veil, particle physics, atomic bonding"
#[derive(Debug, Clone, PartialEq)]
pub struct FirstDensityMechanics {
    pub atomic_stability: Float,
    pub bonding_energy: Float,
    pub elemental_composition: HashMap<String, Float>,
}

impl FirstDensityMechanics {
    /// Create new 1st density mechanics
    pub fn new() -> Self {
        FirstDensityMechanics {
            atomic_stability: 1.0,
            bonding_energy: 0.5,
            elemental_composition: HashMap::new(),
        }
    }

    /// Form atomic bond
    pub fn form_bond(
        &mut self,
        element_a: &str,
        element_b: &str,
        energy: Float,
    ) -> Result<(), DensityMechanicsError> {
        if energy < self.bonding_energy {
            return Err(DensityMechanicsError::InsufficientEnergy(
                energy,
                self.bonding_energy,
            ));
        }
        *self
            .elemental_composition
            .entry(element_a.to_string())
            .or_insert(0.0) += 1.0;
        *self
            .elemental_composition
            .entry(element_b.to_string())
            .or_insert(0.0) += 1.0;
        Ok(())
    }

    /// Stabilize matter
    pub fn stabilize_matter(&mut self) {
        self.atomic_stability = 1.0;
    }

    /// Get elemental composition
    pub fn get_composition(&self) -> &HashMap<String, Float> {
        &self.elemental_composition
    }
}

impl Default for FirstDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 2nd Density Mechanics - Instinctual Growth
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "2nd Density: Instinctual growth - no veil, evolution, food chains, survival"
#[derive(Debug, Clone, PartialEq)]
pub struct SecondDensityMechanics {
    pub survival_instinct: Float,
    pub reproductive_drive: Float,
    pub evolutionary_progress: Float,
    food_chain_position: usize,
}

impl SecondDensityMechanics {
    /// Create new 2nd density mechanics
    pub fn new(food_chain_position: usize) -> Self {
        SecondDensityMechanics {
            survival_instinct: 0.8,
            reproductive_drive: 0.7,
            evolutionary_progress: 0.0,
            food_chain_position,
        }
    }

    /// Process survival instinct
    pub fn process_survival(&self, threat_level: Float) -> String {
        if threat_level > self.survival_instinct {
            "flee".to_string()
        } else {
            "stay".to_string()
        }
    }

    /// Evolve
    pub fn evolve(&mut self, catalyst_intensity: Float) {
        self.evolutionary_progress += catalyst_intensity * 0.1;
        self.evolutionary_progress = self.evolutionary_progress.min(1.0);
    }

    /// Get evolutionary progress
    pub fn evolutionary_progress(&self) -> Float {
        self.evolutionary_progress
    }

    /// Get food chain position
    pub fn food_chain_position(&self) -> usize {
        self.food_chain_position
    }
}

impl Default for SecondDensityMechanics {
    fn default() -> Self {
        Self::new(1)
    }
}

/// 3rd Density Mechanics - Self-Aware with Veil
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "3rd Density: Self-aware + veil - polarity choice, catalyst for evolution"
#[derive(Debug, Clone, PartialEq)]
pub struct ThirdDensityMechanics {
    pub polarity: Polarity,
    pub polarization_strength: Float,
    pub veil_intensity: Float,
    pub catalyst_accumulated: Float,
}

impl ThirdDensityMechanics {
    /// Create new 3rd density mechanics
    pub fn new() -> Self {
        ThirdDensityMechanics {
            polarity: Polarity::Undecided,
            polarization_strength: 0.0,
            veil_intensity: 0.9,
            catalyst_accumulated: 0.0,
        }
    }

    /// Choose polarity
    pub fn choose_polarity(&mut self, polarity: Polarity) {
        self.polarity = polarity;
        self.polarization_strength = 0.1;
    }

    /// Process catalyst
    pub fn process_catalyst(&mut self, catalyst: &Catalyst) -> Result<(), DensityMechanicsError> {
        self.catalyst_accumulated += catalyst.intensity;

        if self.polarity == Polarity::Undecided {
            if let Some(catalyst_polarity) = catalyst.polarity_affinity {
                if self.catalyst_accumulated > 0.5 {
                    self.polarity = catalyst_polarity;
                    self.polarization_strength = 0.1;
                }
            }
        } else {
            self.polarization_strength += catalyst.intensity * 0.1;
            self.polarization_strength = self.polarization_strength.min(1.0);
        }

        Ok(())
    }

    /// Get polarization strength
    pub fn polarization_strength(&self) -> Float {
        self.polarization_strength
    }

    /// Check if polarized
    pub fn is_polarized(&self) -> bool {
        self.polarity != Polarity::Undecided
    }

    /// Get polarity
    pub fn polarity(&self) -> Polarity {
        self.polarity
    }
}

impl Default for ThirdDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 4th Density Mechanics - Love/Understanding
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "4th Density: Love/understanding - veil thinning, telepathy, collective consciousness"
#[derive(Debug, Clone, PartialEq)]
pub struct FourthDensityMechanics {
    pub love_capacity: Float,
    pub understanding_depth: Float,
    pub telepathy_strength: Float,
    pub collective_awareness: Float,
    pub polarity_integration: Float,
}

impl FourthDensityMechanics {
    /// Create new 4th density mechanics
    pub fn new() -> Self {
        FourthDensityMechanics {
            love_capacity: 0.5,
            understanding_depth: 0.5,
            telepathy_strength: 0.3,
            collective_awareness: 0.3,
            polarity_integration: 0.0,
        }
    }

    /// Channel love
    pub fn channel_love(&mut self, intensity: Float) {
        self.love_capacity += intensity * 0.1;
        self.love_capacity = self.love_capacity.min(1.0);
    }

    /// Practice compassion
    pub fn practice_compassion(&mut self, target_polarity: Polarity) {
        self.understanding_depth += 0.1;
        self.understanding_depth = self.understanding_depth.min(1.0);

        if target_polarity == Polarity::ServiceToOthers
            || target_polarity == Polarity::ServiceToSelf
        {
            self.polarity_integration += 0.1;
            self.polarity_integration = self.polarity_integration.min(1.0);
        }
    }

    /// Develop telepathy
    pub fn develop_telepathy(&mut self, practice_time: Float) {
        self.telepathy_strength += practice_time * 0.01;
        self.telepathy_strength = self.telepathy_strength.min(1.0);
    }

    /// Form collective
    pub fn form_collective(&mut self, member_count: usize) {
        self.collective_awareness += member_count as Float * 0.01;
        self.collective_awareness = self.collective_awareness.min(1.0);
    }

    /// Get love capacity
    pub fn love_capacity(&self) -> Float {
        self.love_capacity
    }

    /// Get telepathy strength
    pub fn telepathy_strength(&self) -> Float {
        self.telepathy_strength
    }
}

impl Default for FourthDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 5th Density Mechanics - Wisdom/Light
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "5th Density: Wisdom/light - veil mostly dissolved, teaching/learning, light bodies"
#[derive(Debug, Clone, PartialEq)]
pub struct FifthDensityMechanics {
    pub wisdom_level: Float,
    pub light_body_activation: Float,
    pub teaching_skill: Float,
    pub learning_capacity: Float,
}

impl FifthDensityMechanics {
    /// Create new 5th density mechanics
    pub fn new() -> Self {
        FifthDensityMechanics {
            wisdom_level: 0.5,
            light_body_activation: 0.5,
            teaching_skill: 0.5,
            learning_capacity: 0.5,
        }
    }

    /// Teach
    pub fn teach(&mut self, intensity: Float) {
        self.teaching_skill += intensity * 0.05;
        self.teaching_skill = self.teaching_skill.min(1.0);
        self.wisdom_level += intensity * 0.02;
        self.wisdom_level = self.wisdom_level.min(1.0);
    }

    /// Learn
    pub fn learn(&mut self, intensity: Float) {
        self.learning_capacity += intensity * 0.05;
        self.learning_capacity = self.learning_capacity.min(1.0);
        self.wisdom_level += intensity * 0.03;
        self.wisdom_level = self.wisdom_level.min(1.0);
    }

    /// Activate light body
    pub fn activate_light_body(&mut self, activation_level: Float) {
        self.light_body_activation += activation_level * 0.1;
        self.light_body_activation = self.light_body_activation.min(1.0);
    }

    /// Get wisdom level
    pub fn wisdom_level(&self) -> Float {
        self.wisdom_level
    }

    /// Get light body activation
    pub fn light_body_activation(&self) -> Float {
        self.light_body_activation
    }
}

impl Default for FifthDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 6th Density Mechanics - Unity/Balance
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "6th Density: Unity/balance - veil completely dissolved, social memory complexes"
#[derive(Debug, Clone, PartialEq)]
pub struct SixthDensityMechanics {
    pub unity_level: Float,
    pub balance_mastery: Float,
    pub social_memory_access: Float,
    pub all_polarities_integrated: bool,
}

impl SixthDensityMechanics {
    /// Create new 6th density mechanics
    pub fn new() -> Self {
        SixthDensityMechanics {
            unity_level: 0.5,
            balance_mastery: 0.5,
            social_memory_access: 0.5,
            all_polarities_integrated: false,
        }
    }

    /// Balance polarities
    pub fn balance_polarities(&mut self) {
        self.balance_mastery += 0.1;
        self.balance_mastery = self.balance_mastery.min(1.0);

        if self.balance_mastery >= 0.9 {
            self.all_polarities_integrated = true;
        }
    }

    /// Access social memory
    pub fn access_social_memory(&mut self, depth: Float) {
        self.social_memory_access += depth * 0.1;
        self.social_memory_access = self.social_memory_access.min(1.0);
    }

    /// Merge into social memory complex
    pub fn merge_into_social_memory(&mut self) {
        self.unity_level = 1.0;
        self.social_memory_access = 1.0;
    }

    /// Get unity level
    pub fn unity_level(&self) -> Float {
        self.unity_level
    }

    /// Check if all polarities integrated
    pub fn all_polarities_integrated(&self) -> bool {
        self.all_polarities_integrated
    }
}

impl Default for SixthDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 7th Density Mechanics - Gateway
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "7th Density: Gateway - transcends veil, violet-ray activation, prepare for next octave"
#[derive(Debug, Clone, PartialEq)]
pub struct SeventhDensityMechanics {
    pub violet_ray_activation: Float,
    pub gateway_readiness: Float,
    pub all_densities_integrated: bool,
    pub intelligent_infinity_access: Float,
}

impl SeventhDensityMechanics {
    /// Create new 7th density mechanics
    pub fn new() -> Self {
        SeventhDensityMechanics {
            violet_ray_activation: 0.5,
            gateway_readiness: 0.0,
            all_densities_integrated: false,
            intelligent_infinity_access: 0.0,
        }
    }

    /// Activate violet ray
    pub fn activate_violet_ray(&mut self, intensity: Float) {
        self.violet_ray_activation += intensity * 0.1;
        self.violet_ray_activation = self.violet_ray_activation.min(1.0);

        if self.violet_ray_activation >= 0.9 {
            self.all_densities_integrated = true;
            self.gateway_readiness += 0.1;
        }
    }

    /// Access intelligent infinity
    pub fn access_intelligent_infinity(&mut self) {
        if self.all_densities_integrated {
            self.intelligent_infinity_access += 0.1;
            self.intelligent_infinity_access = self.intelligent_infinity_access.min(1.0);
        }
    }

    /// Prepare for gateway
    pub fn prepare_gateway(&mut self) {
        if self.violet_ray_activation >= 0.9 && self.all_densities_integrated {
            self.gateway_readiness = 1.0;
        }
    }

    /// Get gateway readiness
    pub fn gateway_readiness(&self) -> Float {
        self.gateway_readiness
    }

    /// Check if ready for next octave
    pub fn ready_for_next_octave(&self) -> bool {
        self.gateway_readiness >= 1.0
    }
}

impl Default for SeventhDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// 8th Density Mechanics - Intelligent Infinity
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "8th Density: Return to source - merge with intelligent infinity, complete octave"
#[derive(Debug, Clone, PartialEq)]
pub struct EighthDensityMechanics {
    pub merged_with_infinity: bool,
    pub octave_complete: bool,
    pub next_octave_preparation: Float,
}

impl EighthDensityMechanics {
    /// Create new 8th density mechanics
    pub fn new() -> Self {
        EighthDensityMechanics {
            merged_with_infinity: false,
            octave_complete: false,
            next_octave_preparation: 0.0,
        }
    }

    /// Merge with intelligent infinity
    pub fn merge_with_infinity(&mut self) {
        self.merged_with_infinity = true;
    }

    /// Complete octave
    pub fn complete_octave(&mut self) {
        if self.merged_with_infinity {
            self.octave_complete = true;
        }
    }

    /// Prepare for next octave
    pub fn prepare_next_octave(&mut self, preparation_level: Float) {
        if self.octave_complete {
            self.next_octave_preparation += preparation_level * 0.1;
            self.next_octave_preparation = self.next_octave_preparation.min(1.0);
        }
    }

    /// Check if merged
    pub fn is_merged(&self) -> bool {
        self.merged_with_infinity
    }

    /// Check if octave complete
    pub fn is_octave_complete(&self) -> bool {
        self.octave_complete
    }
}

impl Default for EighthDensityMechanics {
    fn default() -> Self {
        Self::new()
    }
}

/// Density-Specific Mechanics - Main enum for all 8 densities
#[derive(Debug, Clone, PartialEq)]
pub enum DensityMechanics {
    First(FirstDensityMechanics),
    Second(SecondDensityMechanics),
    Third(ThirdDensityMechanics),
    Fourth(FourthDensityMechanics),
    Fifth(FifthDensityMechanics),
    Sixth(SixthDensityMechanics),
    Seventh(SeventhDensityMechanics),
    Eighth(EighthDensityMechanics),
}

impl DensityMechanics {
    /// Create density mechanics for a specific density
    pub fn from_density(density: Density) -> Self {
        match density {
            Density::First => DensityMechanics::First(FirstDensityMechanics::new()),
            Density::Second => DensityMechanics::Second(SecondDensityMechanics::default()),
            Density::Third => DensityMechanics::Third(ThirdDensityMechanics::new()),
            Density::Fourth => DensityMechanics::Fourth(FourthDensityMechanics::new()),
            Density::Fifth => DensityMechanics::Fifth(FifthDensityMechanics::new()),
            Density::Sixth => DensityMechanics::Sixth(SixthDensityMechanics::new()),
            Density::Seventh => DensityMechanics::Seventh(SeventhDensityMechanics::new()),
            Density::Eighth => DensityMechanics::Eighth(EighthDensityMechanics::new()),
        }
    }

    /// Get density
    pub fn density(&self) -> Density {
        match self {
            DensityMechanics::First(_) => Density::First,
            DensityMechanics::Second(_) => Density::Second,
            DensityMechanics::Third(_) => Density::Third,
            DensityMechanics::Fourth(_) => Density::Fourth,
            DensityMechanics::Fifth(_) => Density::Fifth,
            DensityMechanics::Sixth(_) => Density::Sixth,
            DensityMechanics::Seventh(_) => Density::Seventh,
            DensityMechanics::Eighth(_) => Density::Eighth,
        }
    }

    /// Get perception filter
    pub fn perception_filter(&self) -> PerceptionFilter {
        PerceptionFilter::from_density(self.density())
    }

    /// Get action space
    pub fn action_space(&self) -> ActionSpace {
        ActionSpace::from_density(self.density())
    }

    /// Get interaction rules
    pub fn interaction_rules(&self) -> InteractionRules {
        InteractionRules::from_density(self.density())
    }

    /// Process archetype activation profile
    pub fn process_archetype_profile(
        &mut self,
        profile: &ArchetypeActivationProfile,
        cache: Option<&mut ArchetypeInterferenceCache>,
    ) -> Result<ArchetypicalInterference, DensityMechanicsError> {
        if let Some(cache) = cache {
            let interference = cache.get(profile);
            return Ok(interference);
        }

        let interference = self.compute_archetype_interference(profile)?;
        Ok(interference)
    }

    /// Compute archetype interference (internal method)
    fn compute_archetype_interference(
        &self,
        profile: &ArchetypeActivationProfile,
    ) -> Result<ArchetypicalInterference, DensityMechanicsError> {
        let coefficients = profile.coefficients();
        let pattern = coefficients.to_vec();
        Ok(ArchetypicalInterference::new(pattern))
    }
}

/// Density mechanics error
#[derive(Debug, Clone, PartialEq)]
pub enum DensityMechanicsError {
    InsufficientEnergy(Float, Float),
    InvalidDensity(usize),
    InvalidAction(String),
    CatalystProcessingError(String),
    ArchetypeInterferenceError(String),
}

/// Density mechanics statistics
#[derive(Debug, Clone, PartialEq)]
pub struct DensityMechanicsStatistics {
    pub total_entities_by_density: [usize; NUM_DENSITIES],
    pub average_catalyst_intensity_by_density: [Float; NUM_DENSITIES],
    pub average_polarization_by_density: [Float; NUM_DENSITIES],
    pub transitions_between_densities: usize,
}

impl DensityMechanicsStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        DensityMechanicsStatistics {
            total_entities_by_density: [0; NUM_DENSITIES],
            average_catalyst_intensity_by_density: [0.0; NUM_DENSITIES],
            average_polarization_by_density: [0.0; NUM_DENSITIES],
            transitions_between_densities: 0,
        }
    }

    /// Record entity in density
    pub fn record_entity(
        &mut self,
        density: Density,
        catalyst_intensity: Float,
        polarization: Float,
    ) {
        let idx = density.value() - 1;
        self.total_entities_by_density[idx] += 1;
        self.average_catalyst_intensity_by_density[idx] += catalyst_intensity;
        self.average_polarization_by_density[idx] += polarization;
    }

    /// Finalize averages
    pub fn finalize(&mut self) {
        for i in 0..NUM_DENSITIES {
            if self.total_entities_by_density[i] > 0 {
                let count = self.total_entities_by_density[i] as Float;
                self.average_catalyst_intensity_by_density[i] /= count;
                self.average_polarization_by_density[i] /= count;
            }
        }
    }

    /// Record density transition
    pub fn record_transition(&mut self) {
        self.transitions_between_densities += 1;
    }
}

impl Default for DensityMechanicsStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_values() {
        assert_eq!(Density::First.value(), 1);
        assert_eq!(Density::Second.value(), 2);
        assert_eq!(Density::Third.value(), 3);
        assert_eq!(Density::Fourth.value(), 4);
        assert_eq!(Density::Fifth.value(), 5);
        assert_eq!(Density::Sixth.value(), 6);
        assert_eq!(Density::Seventh.value(), 7);
        assert_eq!(Density::Eighth.value(), 8);
    }

    #[test]
    fn test_density_veil_transparency() {
        assert_eq!(Density::First.veil_transparency(), 0.0);
        assert_eq!(Density::Second.veil_transparency(), 0.0);
        assert_eq!(Density::Third.veil_transparency(), 0.1);
        assert_eq!(Density::Fourth.veil_transparency(), 0.3);
        assert_eq!(Density::Fifth.veil_transparency(), 0.6);
        assert_eq!(Density::Sixth.veil_transparency(), 1.0);
        assert_eq!(Density::Seventh.veil_transparency(), 1.0);
        assert_eq!(Density::Eighth.veil_transparency(), 1.0);
    }

    #[test]
    fn test_density_spectrum_access() {
        assert!(Density::First.spectrum_access() < Density::Second.spectrum_access());
        assert!(Density::Second.spectrum_access() < Density::Third.spectrum_access());
        assert!(Density::Third.spectrum_access() < Density::Fourth.spectrum_access());
        assert!(Density::Fourth.spectrum_access() < Density::Fifth.spectrum_access());
        assert!(Density::Fifth.spectrum_access() < Density::Sixth.spectrum_access());
        assert!(Density::Sixth.spectrum_access() < Density::Seventh.spectrum_access());
        assert!(Density::Seventh.spectrum_access() < Density::Eighth.spectrum_access());
        assert_eq!(Density::Eighth.spectrum_access(), 1.0);
    }

    #[test]
    fn test_perception_filter_from_density() {
        let filter = PerceptionFilter::from_density(Density::Third);
        assert_eq!(filter.veil_transparency, 0.1);
        assert_eq!(filter.spectrum_access, 0.2);
        assert!(filter.physical_dominant);
        assert!(!filter.telepathy_enabled);

        let filter = PerceptionFilter::from_density(Density::Fourth);
        assert_eq!(filter.veil_transparency, 0.3);
        assert_eq!(filter.spectrum_access, 0.4);
        assert!(!filter.physical_dominant);
        assert!(filter.telepathy_enabled);
    }

    #[test]
    fn test_action_space_from_density() {
        let actions = ActionSpace::from_density(Density::Third);
        assert!(actions.physical_actions.contains(&"move".to_string()));
        assert!(actions.mental_actions.contains(&"think".to_string()));
        assert!(actions.spiritual_actions.contains(&"meditate".to_string()));

        let actions = ActionSpace::from_density(Density::Fourth);
        assert!(actions
            .collective_actions
            .contains(&"telepathic_communication".to_string()));
        assert!(actions
            .mental_actions
            .contains(&"integrate_polarities".to_string()));
    }

    #[test]
    fn test_interaction_rules_from_density() {
        let rules = InteractionRules::from_density(Density::Third);
        assert!(rules.can_harm);
        assert!(rules.can_heal);
        assert!(rules.polarity_conflict);
        assert!(!rules.telepathic_communication);

        let rules = InteractionRules::from_density(Density::Fourth);
        assert!(!rules.can_harm);
        assert!(rules.can_heal);
        assert!(!rules.polarity_conflict);
        assert!(rules.telepathic_communication);
    }

    #[test]
    fn test_first_density_mechanics() {
        let mut mechanics = FirstDensityMechanics::new();
        assert_eq!(mechanics.atomic_stability, 1.0);

        mechanics.form_bond("H", "O", 0.6).unwrap();
        assert_eq!(mechanics.get_composition().get("H"), Some(&1.0));
        assert_eq!(mechanics.get_composition().get("O"), Some(&1.0));

        let result = mechanics.form_bond("Na", "Cl", 0.3);
        assert!(result.is_err());
    }

    #[test]
    fn test_second_density_mechanics() {
        let mut mechanics = SecondDensityMechanics::new(5);
        assert_eq!(mechanics.food_chain_position(), 5);

        let action = mechanics.process_survival(0.9);
        assert_eq!(action, "flee");

        let action = mechanics.process_survival(0.5);
        assert_eq!(action, "stay");

        mechanics.evolve(0.5);
        assert!(mechanics.evolutionary_progress() > 0.0);
    }

    #[test]
    fn test_third_density_mechanics() {
        let mut mechanics = ThirdDensityMechanics::new();
        assert_eq!(mechanics.polarity(), Polarity::Undecided);
        assert!(!mechanics.is_polarized());

        mechanics.choose_polarity(Polarity::ServiceToOthers);
        assert_eq!(mechanics.polarity(), Polarity::ServiceToOthers);
        assert!(mechanics.is_polarized());

        let catalyst = Catalyst::new(0.8, Some(Polarity::ServiceToOthers), "test".to_string());
        mechanics.process_catalyst(&catalyst).unwrap();
        assert!(mechanics.polarization_strength() > 0.1);
    }

    #[test]
    fn test_fourth_density_mechanics() {
        let mut mechanics = FourthDensityMechanics::new();
        assert_eq!(mechanics.love_capacity(), 0.5);

        mechanics.channel_love(0.5);
        assert!(mechanics.love_capacity() > 0.5);

        mechanics.practice_compassion(Polarity::ServiceToOthers);
        assert!(mechanics.understanding_depth > 0.5);

        mechanics.develop_telepathy(10.0);
        assert!(mechanics.telepathy_strength() > 0.3);
    }

    #[test]
    fn test_fifth_density_mechanics() {
        let mut mechanics = FifthDensityMechanics::new();
        assert_eq!(mechanics.wisdom_level(), 0.5);

        mechanics.teach(0.5);
        assert!(mechanics.wisdom_level() > 0.5);
        assert!(mechanics.teaching_skill > 0.5);

        mechanics.learn(0.5);
        assert!(mechanics.wisdom_level() > 0.5);
        assert!(mechanics.learning_capacity > 0.5);

        mechanics.activate_light_body(0.5);
        assert!(mechanics.light_body_activation() > 0.5);
    }

    #[test]
    fn test_sixth_density_mechanics() {
        let mut mechanics = SixthDensityMechanics::new();
        assert_eq!(mechanics.unity_level(), 0.5);

        mechanics.balance_polarities();
        assert!(mechanics.balance_mastery > 0.5);

        for _ in 0..10 {
            mechanics.balance_polarities();
        }
        assert!(mechanics.all_polarities_integrated());

        mechanics.access_social_memory(0.5);
        assert!(mechanics.social_memory_access > 0.5);
    }

    #[test]
    fn test_seventh_density_mechanics() {
        let mut mechanics = SeventhDensityMechanics::new();
        assert_eq!(mechanics.violet_ray_activation, 0.5);

        for _ in 0..10 {
            mechanics.activate_violet_ray(0.5);
        }
        assert!(mechanics.all_densities_integrated);
        assert!(mechanics.gateway_readiness() > 0.0);

        mechanics.access_intelligent_infinity();
        assert!(mechanics.intelligent_infinity_access > 0.0);
    }

    #[test]
    fn test_eighth_density_mechanics() {
        let mut mechanics = EighthDensityMechanics::new();
        assert!(!mechanics.is_merged());
        assert!(!mechanics.is_octave_complete());

        mechanics.merge_with_infinity();
        assert!(mechanics.is_merged());

        mechanics.complete_octave();
        assert!(mechanics.is_octave_complete());

        mechanics.prepare_next_octave(0.5);
        assert!(mechanics.next_octave_preparation > 0.0);
    }

    #[test]
    fn test_density_mechanics_from_density() {
        let mechanics = DensityMechanics::from_density(Density::Third);
        assert_eq!(mechanics.density(), Density::Third);

        let mechanics = DensityMechanics::from_density(Density::Sixth);
        assert_eq!(mechanics.density(), Density::Sixth);
    }

    #[test]
    fn test_density_mechanics_getters() {
        let mechanics = DensityMechanics::from_density(Density::Fourth);

        let filter = mechanics.perception_filter();
        assert_eq!(filter.veil_transparency, 0.3);

        let actions = mechanics.action_space();
        assert!(!actions.physical_actions.is_empty());

        let rules = mechanics.interaction_rules();
        assert!(!rules.can_harm);
    }

    #[test]
    fn test_density_mechanics_statistics() {
        let mut stats = DensityMechanicsStatistics::new();

        stats.record_entity(Density::Third, 0.5, 0.8);
        stats.record_entity(Density::Third, 0.7, 0.6);
        stats.finalize();

        assert_eq!(stats.total_entities_by_density[2], 2);
        assert!((stats.average_catalyst_intensity_by_density[2] - 0.6).abs() < 0.01);
        assert!((stats.average_polarization_by_density[2] - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_catalyst_creation() {
        let catalyst = Catalyst::new(0.8, Some(Polarity::ServiceToOthers), "test".to_string());
        assert_eq!(catalyst.intensity, 0.8);
        assert_eq!(catalyst.polarity_affinity, Some(Polarity::ServiceToOthers));

        let random_catalyst = Catalyst::random(0.5);
        assert_eq!(random_catalyst.intensity, 0.5);
        assert_eq!(random_catalyst.polarity_affinity, None);
    }

    #[test]
    fn test_catalyst_intensity_clamping() {
        let catalyst = Catalyst::new(1.5, None, "test".to_string());
        assert_eq!(catalyst.intensity, MAX_CATALYST_INTENSITY);

        let catalyst = Catalyst::new(-0.5, None, "test".to_string());
        assert_eq!(catalyst.intensity, 0.0);
    }

    #[test]
    fn test_polarity_equality() {
        assert_eq!(Polarity::ServiceToOthers, Polarity::ServiceToOthers);
        assert_eq!(Polarity::ServiceToSelf, Polarity::ServiceToSelf);
        assert_ne!(Polarity::ServiceToOthers, Polarity::ServiceToSelf);
        assert_eq!(Polarity::Undecided, Polarity::Undecided);
    }

    #[test]
    fn test_density_names() {
        assert_eq!(Density::First.name(), "First Density");
        assert_eq!(Density::Third.name(), "Third Density");
        assert_eq!(Density::Eighth.name(), "Eighth Density");
    }

    #[test]
    fn test_density_ray_colors() {
        assert_eq!(Density::First.ray_color(), "Red");
        assert_eq!(Density::Second.ray_color(), "Orange");
        assert_eq!(Density::Third.ray_color(), "Yellow");
        assert_eq!(Density::Fourth.ray_color(), "Green");
        assert_eq!(Density::Fifth.ray_color(), "Blue");
        assert_eq!(Density::Sixth.ray_color(), "Indigo");
        assert_eq!(Density::Seventh.ray_color(), "Violet");
        assert_eq!(Density::Eighth.ray_color(), "White");
    }
}
