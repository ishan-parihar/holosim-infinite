//! Catalyst-Driven Combat System
//!
//! Implements Week 94: Combat as Catalyst-Driven Conflict
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "Combat is NOT about winning/losing but about catalyst generation"
//! > "Catalyst is the friction that drives evolution (1st-6th densities need catalyst)"
//! > "Combat outcomes are determined by resonance interference, not damage formulas"
//!
//! This module treats combat as evolutionary friction rather than traditional damage mechanics.
//! Conflict generates catalyst (evolutionary energy) that drives entities toward higher densities.

use crate::simulation_v3::holographic_inventory::ResonancePattern;
use crate::types::{Density, Float, Polarity};
use std::collections::HashMap;

pub type EntityId = u64;
pub type ArchetypeId = u8;
pub type CombatId = u64;
pub type CatalystAmount = Float;
pub type Timestamp = u64;

/// Minimum participants required for combat
pub const MIN_COMBAT_PARTICIPANTS: usize = 2;

/// Maximum participants allowed in a single combat
pub const MAX_COMBAT_PARTICIPANTS: usize = 100;

/// Base catalyst generation multiplier
pub const BASE_CATALYST_MULTIPLIER: Float = 10.0;

/// Quality refinement threshold
pub const REFINEMENT_THRESHOLD: Float = 0.7;

/// Transformation threshold
pub const TRANSFORMATION_THRESHOLD: Float = 0.9;

/// Quality of catalyst determines its evolutionary potential
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalystQuality {
    /// Raw catalyst - freshly generated from conflict, less potent
    Raw,
    /// Refined catalyst - processed through resonance, more potent
    Refined,
    /// Transformed catalyst - fully processed, maximum potency
    Transformed,
}

impl CatalystQuality {
    /// Get the potency multiplier for this quality level
    pub fn potency_multiplier(&self) -> Float {
        match self {
            CatalystQuality::Raw => 1.0,
            CatalystQuality::Refined => 1.5,
            CatalystQuality::Transformed => 2.0,
        }
    }

    /// Get the next quality level, if any
    pub fn next_quality(&self) -> Option<CatalystQuality> {
        match self {
            CatalystQuality::Raw => Some(CatalystQuality::Refined),
            CatalystQuality::Refined => Some(CatalystQuality::Transformed),
            CatalystQuality::Transformed => None,
        }
    }
}

impl std::fmt::Display for CatalystQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalystQuality::Raw => write!(f, "Raw"),
            CatalystQuality::Refined => write!(f, "Refined"),
            CatalystQuality::Transformed => write!(f, "Transformed"),
        }
    }
}

/// Catalyst - The fundamental conflict energy that drives evolution
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Catalyst is the friction that drives evolution"
#[derive(Debug, Clone)]
pub struct Catalyst {
    /// Amount of catalyst energy
    pub amount: CatalystAmount,
    /// Quality level of the catalyst
    pub quality: CatalystQuality,
    /// Which archetypes generated this catalyst
    pub source_archetypes: Vec<ArchetypeId>,
    /// Which density this catalyst operates in
    pub density_level: Density,
    /// Polarity alignment of the catalyst
    pub polarity_alignment: Polarity,
    /// When this catalyst was generated
    pub timestamp: Timestamp,
}

impl Catalyst {
    /// Create a new catalyst with default values
    pub fn new(
        amount: CatalystAmount,
        density_level: Density,
        polarity_alignment: Polarity,
    ) -> Self {
        Self {
            amount,
            quality: CatalystQuality::Raw,
            source_archetypes: Vec::new(),
            density_level,
            polarity_alignment,
            timestamp: 0,
        }
    }

    /// Create a catalyst with specific archetype sources
    pub fn with_archetypes(
        amount: CatalystAmount,
        density_level: Density,
        polarity_alignment: Polarity,
        archetypes: Vec<ArchetypeId>,
    ) -> Self {
        Self {
            amount,
            quality: CatalystQuality::Raw,
            source_archetypes: archetypes,
            density_level,
            polarity_alignment,
            timestamp: 0,
        }
    }

    /// Transform raw catalyst into refined catalyst
    ///
    /// # Returns
    /// A new catalyst with improved quality if refinement is possible
    pub fn refine(&self) -> Option<Catalyst> {
        self.quality.next_quality().map(|new_quality| {
            let potency_loss = 0.1; // 10% loss during refinement
            Catalyst {
                amount: self.amount * (1.0 - potency_loss),
                quality: new_quality,
                source_archetypes: self.source_archetypes.clone(),
                density_level: self.density_level,
                polarity_alignment: self.polarity_alignment,
                timestamp: self.timestamp,
            }
        })
    }

    /// Combine two catalysts into a single catalyst
    ///
    /// The resulting catalyst takes the higher quality and combines amounts.
    /// Archetype sources are merged and deduplicated.
    pub fn combine(&self, other: &Catalyst) -> Catalyst {
        // Take the higher quality
        let new_quality = match (self.quality, other.quality) {
            (CatalystQuality::Transformed, _) | (_, CatalystQuality::Transformed) => {
                CatalystQuality::Transformed
            }
            (CatalystQuality::Refined, _) | (_, CatalystQuality::Refined) => {
                CatalystQuality::Refined
            }
            _ => CatalystQuality::Raw,
        };

        // Combine archetype sources
        let mut combined_archetypes = self.source_archetypes.clone();
        for archetype in &other.source_archetypes {
            if !combined_archetypes.contains(archetype) {
                combined_archetypes.push(*archetype);
            }
        }

        // Use the higher density (more evolved)
        let new_density = if self.density_level.as_u8() >= other.density_level.as_u8() {
            self.density_level
        } else {
            other.density_level
        };

        // Combine polarity (balanced if different)
        let new_polarity = if self.polarity_alignment == other.polarity_alignment {
            self.polarity_alignment
        } else {
            Polarity::Neutral
        };

        Catalyst {
            amount: self.amount + other.amount,
            quality: new_quality,
            source_archetypes: combined_archetypes,
            density_level: new_density,
            polarity_alignment: new_polarity,
            timestamp: self.timestamp.max(other.timestamp),
        }
    }

    /// Calculate the evolutionary potential of this catalyst
    ///
    /// Evolutionary potential = amount × quality_multiplier × density_factor
    pub fn get_evolutionary_potential(&self) -> Float {
        let quality_multiplier = self.quality.potency_multiplier();
        let density_factor = self.density_level.as_u8() as Float / 8.0;
        self.amount * quality_multiplier * density_factor
    }

    /// Check if this catalyst can be absorbed by an entity of a given density
    pub fn can_be_absorbed_by(&self, entity_density: Density) -> bool {
        let catalyst_density = self.density_level.as_u8();
        let entity_density_val = entity_density.as_u8();

        // Catalyst can be absorbed if it's within one density level
        // of the entity's current density
        catalyst_density <= entity_density_val + 1
    }
}

impl Default for Catalyst {
    fn default() -> Self {
        Self {
            amount: 0.0,
            quality: CatalystQuality::Raw,
            source_archetypes: Vec::new(),
            density_level: Density::First,
            polarity_alignment: Polarity::Neutral,
            timestamp: 0,
        }
    }
}

/// State of a combat instance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatState {
    /// Gathering participants, combat not yet started
    Preparation,
    /// Combat is active, catalyst being generated
    Active,
    /// Determining outcomes from generated catalyst
    Resolution,
    /// Combat finished, outcomes determined
    Complete,
}

impl std::fmt::Display for CombatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatState::Preparation => write!(f, "Preparation"),
            CombatState::Active => write!(f, "Active"),
            CombatState::Resolution => write!(f, "Resolution"),
            CombatState::Complete => write!(f, "Complete"),
        }
    }
}

/// Result of combat for an entity
///
/// Tracks how the combat affected the entity's evolution
#[derive(Debug, Clone)]
pub struct EvolutionOutcome {
    /// Progress toward next density (0.0 to 1.0+)
    pub density_progression: Float,
    /// Shift in polarity alignment (-1.0 to 1.0)
    pub polarity_shift: Float,
    /// Which archetypes were activated during combat
    pub archetype_activations: Vec<ArchetypeId>,
    /// Amount of catalyst absorbed for evolution
    pub catalyst_absorbed: CatalystAmount,
    /// Amount of catalyst reflected (not absorbed)
    pub catalyst_reflected: CatalystAmount,
}

impl EvolutionOutcome {
    /// Create a new evolution outcome with default values
    pub fn new() -> Self {
        Self {
            density_progression: 0.0,
            polarity_shift: 0.0,
            archetype_activations: Vec::new(),
            catalyst_absorbed: 0.0,
            catalyst_reflected: 0.0,
        }
    }

    /// Calculate total catalyst involved in this outcome
    pub fn total_catalyst(&self) -> CatalystAmount {
        self.catalyst_absorbed + self.catalyst_reflected
    }

    /// Check if this outcome represents significant evolution
    pub fn is_significant(&self) -> bool {
        self.density_progression > 0.1 || self.catalyst_absorbed > 10.0
    }
}

impl Default for EvolutionOutcome {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a combat encounter
///
/// Combat generates catalyst through resonance interference between participants.
/// The catalyst is then distributed based on each entity's ability to absorb it.
#[derive(Debug, Clone)]
pub struct CombatInstance {
    /// Unique identifier for this combat
    pub instance_id: CombatId,
    /// Entities participating in combat
    pub participants: Vec<EntityId>,
    /// Total catalyst generated by this combat
    pub catalyst_generated: Catalyst,
    /// The interference pattern of all participants
    pub resonance_interference: ResonancePattern,
    /// Current state of the combat
    pub combat_state: CombatState,
    /// When combat started
    pub start_time: Timestamp,
    /// When combat was resolved (if complete)
    pub resolution_time: Option<Timestamp>,
    /// How much catalyst each entity received
    pub catalyst_distribution: HashMap<EntityId, CatalystAmount>,
    /// Evolution outcomes for each participant
    pub evolution_outcomes: HashMap<EntityId, EvolutionOutcome>,
}

impl CombatInstance {
    /// Create a new combat instance
    pub fn new(instance_id: CombatId, participants: Vec<EntityId>) -> Result<Self, CombatError> {
        if participants.len() < MIN_COMBAT_PARTICIPANTS {
            return Err(CombatError::InsufficientParticipants {
                required: MIN_COMBAT_PARTICIPANTS,
                actual: participants.len(),
            });
        }

        if participants.len() > MAX_COMBAT_PARTICIPANTS {
            return Err(CombatError::TooManyParticipants {
                max: MAX_COMBAT_PARTICIPANTS,
                actual: participants.len(),
            });
        }

        Ok(Self {
            instance_id,
            participants,
            catalyst_generated: Catalyst::default(),
            resonance_interference: ResonancePattern::new(),
            combat_state: CombatState::Preparation,
            start_time: 0,
            resolution_time: None,
            catalyst_distribution: HashMap::new(),
            evolution_outcomes: HashMap::new(),
        })
    }

    /// Add a participant to the combat
    ///
    /// Can only be done during Preparation state
    pub fn add_participant(&mut self, entity_id: EntityId) -> Result<(), CombatError> {
        if self.combat_state != CombatState::Preparation {
            return Err(CombatError::InvalidCombatState {
                current: self.combat_state,
                required: CombatState::Preparation,
            });
        }

        if self.participants.len() >= MAX_COMBAT_PARTICIPANTS {
            return Err(CombatError::TooManyParticipants {
                max: MAX_COMBAT_PARTICIPANTS,
                actual: self.participants.len() + 1,
            });
        }

        if self.participants.contains(&entity_id) {
            return Err(CombatError::DuplicateParticipant(entity_id));
        }

        self.participants.push(entity_id);
        Ok(())
    }

    /// Transition combat to Active state
    pub fn activate(&mut self) -> Result<(), CombatError> {
        if self.combat_state != CombatState::Preparation {
            return Err(CombatError::InvalidCombatState {
                current: self.combat_state,
                required: CombatState::Preparation,
            });
        }

        if self.participants.len() < MIN_COMBAT_PARTICIPANTS {
            return Err(CombatError::InsufficientParticipants {
                required: MIN_COMBAT_PARTICIPANTS,
                actual: self.participants.len(),
            });
        }

        self.combat_state = CombatState::Active;
        Ok(())
    }

    /// Transition combat to Resolution state
    pub fn begin_resolution(&mut self) -> Result<(), CombatError> {
        if self.combat_state != CombatState::Active {
            return Err(CombatError::InvalidCombatState {
                current: self.combat_state,
                required: CombatState::Active,
            });
        }

        self.combat_state = CombatState::Resolution;
        Ok(())
    }

    /// Complete the combat
    pub fn complete(&mut self, timestamp: Timestamp) -> Result<(), CombatError> {
        if self.combat_state != CombatState::Resolution {
            return Err(CombatError::InvalidCombatState {
                current: self.combat_state,
                required: CombatState::Resolution,
            });
        }

        self.combat_state = CombatState::Complete;
        self.resolution_time = Some(timestamp);
        Ok(())
    }

    /// Check if combat is complete
    pub fn is_complete(&self) -> bool {
        self.combat_state == CombatState::Complete
    }

    /// Get the number of participants
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
}

/// Error types for combat operations
#[derive(Debug, Clone, PartialEq)]
pub enum CombatError {
    /// Combat is in wrong state for the requested operation
    InvalidCombatState {
        current: CombatState,
        required: CombatState,
    },
    /// Not enough participants for combat
    InsufficientParticipants { required: usize, actual: usize },
    /// Too many participants
    TooManyParticipants { max: usize, actual: usize },
    /// Entity already in combat
    DuplicateParticipant(EntityId),
    /// Catalyst generation failed
    CatalystGenerationFailed,
    /// Combat already complete
    CombatAlreadyComplete,
    /// Invalid resolution parameters
    InvalidResolution,
    /// Combat not found
    CombatNotFound(CombatId),
    /// Entity not in combat
    EntityNotInCombat(EntityId),
}

impl std::fmt::Display for CombatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatError::InvalidCombatState { current, required } => {
                write!(
                    f,
                    "Invalid combat state: current is {}, required is {}",
                    current, required
                )
            }
            CombatError::InsufficientParticipants { required, actual } => {
                write!(
                    f,
                    "Insufficient participants: required {}, actual {}",
                    required, actual
                )
            }
            CombatError::TooManyParticipants { max, actual } => {
                write!(f, "Too many participants: max {}, actual {}", max, actual)
            }
            CombatError::DuplicateParticipant(id) => {
                write!(f, "Entity {} already in combat", id)
            }
            CombatError::CatalystGenerationFailed => {
                write!(f, "Failed to generate catalyst")
            }
            CombatError::CombatAlreadyComplete => {
                write!(f, "Combat is already complete")
            }
            CombatError::InvalidResolution => {
                write!(f, "Invalid resolution parameters")
            }
            CombatError::CombatNotFound(id) => {
                write!(f, "Combat {} not found", id)
            }
            CombatError::EntityNotInCombat(id) => {
                write!(f, "Entity {} not in combat", id)
            }
        }
    }
}

impl std::error::Error for CombatError {}

/// Creates catalyst from entity interactions
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Catalyst is generated through resonance interference between entities"
#[derive(Debug, Clone)]
pub struct CatalystGenerator;

impl CatalystGenerator {
    /// Create a new catalyst generator
    pub fn new() -> Self {
        Self
    }

    /// Generate catalyst from a resonance clash between two entities
    ///
    /// The amount of catalyst is proportional to the interference between
    /// the two resonance patterns. Higher interference = more catalyst.
    pub fn generate_catalyst_from_resonance_clash(
        entity_a: EntityId,
        entity_b: EntityId,
        resonance_a: &ResonancePattern,
        resonance_b: &ResonancePattern,
    ) -> Catalyst {
        // Calculate interference - higher difference = more catalyst
        let interference = resonance_a.compute_interference(resonance_b);
        let clash_intensity = 1.0 - interference; // Invert: less similarity = more clash

        // Base amount from clash intensity
        let base_amount = clash_intensity * BASE_CATALYST_MULTIPLIER;

        // Quality depends on stability of patterns
        let avg_stability = (resonance_a.stability + resonance_b.stability) / 2.0;
        let quality = if avg_stability > TRANSFORMATION_THRESHOLD {
            CatalystQuality::Transformed
        } else if avg_stability > REFINEMENT_THRESHOLD {
            CatalystQuality::Refined
        } else {
            CatalystQuality::Raw
        };

        let mut catalyst = Catalyst {
            amount: base_amount,
            quality,
            source_archetypes: Vec::new(),
            density_level: Density::Third, // Default to 3rd density
            polarity_alignment: Polarity::Neutral,
            timestamp: 0,
        };

        // Store entity IDs in archetype sources for tracking
        // Using entity IDs mod 22 as archetype references
        catalyst.source_archetypes.push((entity_a % 22) as u8);
        catalyst.source_archetypes.push((entity_b % 22) as u8);

        catalyst
    }

    /// Generate catalyst from archetype interference
    ///
    /// When multiple archetypes interact, they generate catalyst based on
    /// their compatibility. Complementary archetypes generate more catalyst.
    pub fn generate_catalyst_from_archetype_interference(archetypes: &[ArchetypeId]) -> Catalyst {
        if archetypes.is_empty() {
            return Catalyst::default();
        }

        // Calculate interference based on archetype differences
        let mut total_interference = 0.0;
        let mut count = 0;

        for i in 0..archetypes.len() {
            for j in (i + 1)..archetypes.len() {
                let diff = (archetypes[i] as i16 - archetypes[j] as i16).abs() as Float;
                // Maximum difference is 21 (22 archetypes, 0-21)
                let normalized_diff = diff / 21.0;
                total_interference += normalized_diff;
                count += 1;
            }
        }

        let avg_interference = if count > 0 {
            total_interference / count as Float
        } else {
            0.0
        };

        let base_amount = avg_interference * BASE_CATALYST_MULTIPLIER * archetypes.len() as Float;

        // Quality based on number of archetypes involved
        let quality = if archetypes.len() >= 5 {
            CatalystQuality::Transformed
        } else if archetypes.len() >= 3 {
            CatalystQuality::Refined
        } else {
            CatalystQuality::Raw
        };

        Catalyst {
            amount: base_amount,
            quality,
            source_archetypes: archetypes.to_vec(),
            density_level: Density::from_u8((archetypes.len() as u8).min(8).max(1)),
            polarity_alignment: Polarity::Neutral,
            timestamp: 0,
        }
    }

    /// Generate catalyst from polarity conflict
    ///
    /// Opposing polarities (STS vs STO) generate the most catalyst.
    /// Same polarities generate minimal catalyst.
    pub fn generate_catalyst_from_polarity_conflict(
        polarity_a: Polarity,
        polarity_b: Polarity,
    ) -> Catalyst {
        let conflict_intensity = match (polarity_a, polarity_b) {
            // Opposing polarities create maximum conflict
            (Polarity::ServiceToSelf, Polarity::ServiceToOthers)
            | (Polarity::ServiceToOthers, Polarity::ServiceToSelf)
            | (Polarity::STS, Polarity::STO)
            | (Polarity::STO, Polarity::STS) => 1.0,
            // Same polarities create minimal conflict
            (p1, p2) if p1 == p2 => 0.1,
            // Neutral with anything creates moderate conflict
            _ => 0.5,
        };

        let base_amount = conflict_intensity * BASE_CATALYST_MULTIPLIER;

        // Quality based on conflict intensity
        let quality = if conflict_intensity > 0.9 {
            CatalystQuality::Transformed
        } else if conflict_intensity > 0.4 {
            CatalystQuality::Refined
        } else {
            CatalystQuality::Raw
        };

        // Polarity alignment of catalyst depends on the conflict
        let catalyst_polarity = match (polarity_a, polarity_b) {
            (Polarity::ServiceToSelf, Polarity::ServiceToOthers)
            | (Polarity::STS, Polarity::STO) => Polarity::Neutral, // Balanced conflict
            _ => polarity_a, // Inherit from first polarity
        };

        Catalyst {
            amount: base_amount,
            quality,
            source_archetypes: Vec::new(),
            density_level: Density::Third,
            polarity_alignment: catalyst_polarity,
            timestamp: 0,
        }
    }

    /// Generate catalyst from density difference
    ///
    /// Entities of different densities generate catalyst proportional
    /// to their density gap. Larger gaps = more catalyst.
    pub fn generate_catalyst_from_density_difference(
        density_a: Density,
        density_b: Density,
    ) -> Catalyst {
        let density_diff = (density_a.as_u8() as i16 - density_b.as_u8() as i16).abs() as Float;
        let normalized_diff = density_diff / 7.0; // Max difference is 7 (8th - 1st)

        let base_amount = normalized_diff * BASE_CATALYST_MULTIPLIER * 2.0;

        // Quality based on density difference
        let quality = if density_diff >= 3.0 {
            CatalystQuality::Transformed
        } else if density_diff >= 1.0 {
            CatalystQuality::Refined
        } else {
            CatalystQuality::Raw
        };

        // Use the higher density
        let higher_density = if density_a.as_u8() > density_b.as_u8() {
            density_a
        } else {
            density_b
        };

        Catalyst {
            amount: base_amount,
            quality,
            source_archetypes: Vec::new(),
            density_level: higher_density,
            polarity_alignment: Polarity::Neutral,
            timestamp: 0,
        }
    }

    /// Generate catalyst from multiple sources combined
    pub fn generate_combined_catalyst(
        &self,
        resonance_pairs: &[(EntityId, EntityId, ResonancePattern, ResonancePattern)],
        archetypes: &[ArchetypeId],
        polarities: &[(Polarity, Polarity)],
        densities: &[(Density, Density)],
    ) -> Catalyst {
        let mut combined = Catalyst::default();

        // Generate from resonance clashes
        for (entity_a, entity_b, res_a, res_b) in resonance_pairs {
            let catalyst =
                Self::generate_catalyst_from_resonance_clash(*entity_a, *entity_b, res_a, res_b);
            combined = combined.combine(&catalyst);
        }

        // Generate from archetype interference
        if !archetypes.is_empty() {
            let archetype_catalyst =
                Self::generate_catalyst_from_archetype_interference(archetypes);
            combined = combined.combine(&archetype_catalyst);
        }

        // Generate from polarity conflicts
        for (pol_a, pol_b) in polarities {
            let polarity_catalyst = Self::generate_catalyst_from_polarity_conflict(*pol_a, *pol_b);
            combined = combined.combine(&polarity_catalyst);
        }

        // Generate from density differences
        for (den_a, den_b) in densities {
            let density_catalyst = Self::generate_catalyst_from_density_difference(*den_a, *den_b);
            combined = combined.combine(&density_catalyst);
        }

        combined
    }
}

impl Default for CatalystGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// How conflicts resolve into evolution
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Combat outcomes are determined by resonance interference, not damage formulas"
pub struct CatalystResolution;

impl CatalystResolution {
    /// Resolve a complete combat instance
    ///
    /// Distributes catalyst among participants and determines evolution outcomes
    /// based on each entity's resonance with the interference pattern.
    pub fn resolve_combat(
        instance: &mut CombatInstance,
        entity_resonances: &HashMap<EntityId, ResonancePattern>,
        entity_densities: &HashMap<EntityId, Density>,
    ) -> Result<HashMap<EntityId, EvolutionOutcome>, CombatError> {
        if instance.combat_state != CombatState::Resolution {
            return Err(CombatError::InvalidCombatState {
                current: instance.combat_state,
                required: CombatState::Resolution,
            });
        }

        // Calculate catalyst distribution
        let distribution =
            Self::calculate_catalyst_distribution(instance, entity_resonances, entity_densities)?;

        // Determine evolution outcomes
        let outcomes = Self::determine_evolution_outcomes(
            &distribution,
            &instance.resonance_interference,
            entity_resonances,
            entity_densities,
        );

        // Store results in instance
        instance.catalyst_distribution = distribution;
        instance.evolution_outcomes = outcomes.clone();

        Ok(outcomes)
    }

    /// Calculate how much catalyst each entity receives
    ///
    /// Distribution is based on:
    /// - Resonance with the interference pattern (higher = more absorption)
    /// - Current density (higher densities absorb more efficiently)
    /// - Catalyst compatibility
    pub fn calculate_catalyst_distribution(
        instance: &CombatInstance,
        entity_resonances: &HashMap<EntityId, ResonancePattern>,
        entity_densities: &HashMap<EntityId, Density>,
    ) -> Result<HashMap<EntityId, CatalystAmount>, CombatError> {
        let total_catalyst = instance.catalyst_generated.amount;
        let mut distribution = HashMap::new();

        if total_catalyst <= 0.0 || instance.participants.is_empty() {
            // Return zero distribution for all participants
            for entity_id in &instance.participants {
                distribution.insert(*entity_id, 0.0);
            }
            return Ok(distribution);
        }

        // Calculate absorption capacity for each entity
        let mut total_absorption = 0.0;
        let mut absorption_scores: Vec<(EntityId, Float)> = Vec::new();

        for entity_id in &instance.participants {
            let resonance = entity_resonances.get(entity_id);
            let density = entity_densities.get(entity_id);

            let absorption_score = match (resonance, density) {
                (Some(res), Some(den)) => {
                    // Resonance with interference pattern
                    let resonance_match = instance.resonance_interference.compute_interference(res);
                    // Density efficiency (higher = more efficient)
                    let density_efficiency = den.as_u8() as Float / 8.0;
                    // Catalyst compatibility
                    let compatibility = if instance.catalyst_generated.can_be_absorbed_by(*den) {
                        1.0
                    } else {
                        0.3 // Reduced absorption for incompatible catalyst
                    };

                    resonance_match * density_efficiency * compatibility
                }
                _ => 0.1, // Minimal absorption without resonance/density data
            };

            absorption_scores.push((*entity_id, absorption_score));
            total_absorption += absorption_score;
        }

        // Distribute catalyst proportionally
        for (entity_id, score) in absorption_scores {
            let proportion = if total_absorption > 0.0 {
                score / total_absorption
            } else {
                1.0 / instance.participants.len() as Float
            };
            let amount = total_catalyst * proportion;
            distribution.insert(entity_id, amount);
        }

        Ok(distribution)
    }

    /// Determine evolution outcomes from catalyst distribution
    ///
    /// Calculates how the absorbed catalyst affects each entity's evolution
    pub fn determine_evolution_outcomes(
        distribution: &HashMap<EntityId, CatalystAmount>,
        interference: &ResonancePattern,
        entity_resonances: &HashMap<EntityId, ResonancePattern>,
        entity_densities: &HashMap<EntityId, Density>,
    ) -> HashMap<EntityId, EvolutionOutcome> {
        let mut outcomes = HashMap::new();

        for (entity_id, catalyst_amount) in distribution {
            let mut outcome = EvolutionOutcome::new();

            // Catalyst absorbed vs reflected
            let resonance = entity_resonances.get(entity_id);
            let density = entity_densities.get(entity_id);

            let absorption_rate = match resonance {
                Some(res) => {
                    let match_score = interference.compute_interference(res);
                    0.3 + match_score * 0.7 // Base 30% + up to 70% from resonance
                }
                None => 0.5, // Default 50% absorption
            };

            outcome.catalyst_absorbed = catalyst_amount * absorption_rate;
            outcome.catalyst_reflected = catalyst_amount * (1.0 - absorption_rate);

            // Density progression
            let density_factor = density.map(|d| d.as_u8() as Float).unwrap_or(3.0);
            let catalyst_potency = outcome.catalyst_absorbed / 100.0; // Normalize
            outcome.density_progression = catalyst_potency * (9.0 - density_factor) / 8.0;

            // Polarity shift based on catalyst alignment
            // This is a simplified model - real implementation would be more complex
            outcome.polarity_shift = 0.0; // Neutral for now

            // Archetype activations
            // Entities activate archetypes that resonate with the interference
            if let Some(res) = resonance {
                for i in 0..8 {
                    if res.pattern[i] > 0.5 {
                        outcome.archetype_activations.push(i as u8);
                    }
                }
            }

            outcomes.insert(*entity_id, outcome);
        }

        outcomes
    }
}

/// Main combat system managing catalyst-driven conflicts
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "Combat is NOT about winning/losing but about catalyst generation"
#[derive(Debug)]
pub struct CatalystCombat {
    /// Currently active combats
    active_combats: HashMap<CombatId, CombatInstance>,
    /// History of completed combats
    combat_history: Vec<CombatInstance>,
    /// Catalyst generator for creating catalyst
    catalyst_generator: CatalystGenerator,
    /// Next combat ID
    next_combat_id: CombatId,
}

impl CatalystCombat {
    /// Create a new catalyst combat system
    pub fn new() -> Self {
        Self {
            active_combats: HashMap::new(),
            combat_history: Vec::new(),
            catalyst_generator: CatalystGenerator::new(),
            next_combat_id: 1,
        }
    }

    /// Initiate a new combat with the given participants
    ///
    /// # Arguments
    /// * `participants` - Vector of entity IDs to participate in combat
    ///
    /// # Returns
    /// The created CombatInstance or an error
    pub fn initiate_combat(
        &mut self,
        participants: Vec<EntityId>,
    ) -> Result<&CombatInstance, CombatError> {
        let combat_id = self.next_combat_id;
        self.next_combat_id += 1;

        let instance = CombatInstance::new(combat_id, participants)?;
        self.active_combats.insert(combat_id, instance);

        Ok(self.active_combats.get(&combat_id).unwrap())
    }

    /// Add a participant to an existing combat
    ///
    /// Can only be done during the Preparation phase
    pub fn add_participant(
        &mut self,
        combat_id: CombatId,
        entity_id: EntityId,
    ) -> Result<(), CombatError> {
        let instance = self
            .active_combats
            .get_mut(&combat_id)
            .ok_or(CombatError::CombatNotFound(combat_id))?;

        instance.add_participant(entity_id)
    }

    /// Generate catalyst for a combat
    ///
    /// Transitions combat from Preparation to Active state
    pub fn generate_catalyst(
        &mut self,
        combat_id: CombatId,
        entity_resonances: &HashMap<EntityId, ResonancePattern>,
        entity_polarities: &HashMap<EntityId, Polarity>,
        entity_densities: &HashMap<EntityId, Density>,
    ) -> Result<Catalyst, CombatError> {
        let instance = self
            .active_combats
            .get_mut(&combat_id)
            .ok_or(CombatError::CombatNotFound(combat_id))?;

        // Activate combat
        instance.activate()?;

        // Generate catalyst from all participant pairs
        let mut resonance_pairs: Vec<(EntityId, EntityId, ResonancePattern, ResonancePattern)> =
            Vec::new();
        let mut polarities: Vec<(Polarity, Polarity)> = Vec::new();
        let mut densities: Vec<(Density, Density)> = Vec::new();

        for i in 0..instance.participants.len() {
            for j in (i + 1)..instance.participants.len() {
                let entity_a = instance.participants[i];
                let entity_b = instance.participants[j];

                // Get resonances
                if let (Some(res_a), Some(res_b)) = (
                    entity_resonances.get(&entity_a),
                    entity_resonances.get(&entity_b),
                ) {
                    resonance_pairs.push((entity_a, entity_b, res_a.clone(), res_b.clone()));
                }

                // Get polarities
                if let (Some(pol_a), Some(pol_b)) = (
                    entity_polarities.get(&entity_a),
                    entity_polarities.get(&entity_b),
                ) {
                    polarities.push((*pol_a, *pol_b));
                }

                // Get densities
                if let (Some(den_a), Some(den_b)) = (
                    entity_densities.get(&entity_a),
                    entity_densities.get(&entity_b),
                ) {
                    densities.push((*den_a, *den_b));
                }
            }
        }

        // Collect all archetypes
        let archetypes: Vec<ArchetypeId> = instance
            .participants
            .iter()
            .map(|e| (*e % 22) as u8)
            .collect();

        // Generate combined catalyst
        let catalyst = self.catalyst_generator.generate_combined_catalyst(
            &resonance_pairs,
            &archetypes,
            &polarities,
            &densities,
        );

        // Store catalyst and update interference pattern
        instance.catalyst_generated = catalyst.clone();

        // Calculate collective resonance pattern
        if let Some(first_res) = entity_resonances.get(&instance.participants[0]) {
            let mut collective = first_res.clone();
            for i in 1..instance.participants.len() {
                if let Some(res) = entity_resonances.get(&instance.participants[i]) {
                    // Average the patterns
                    for j in 0..8 {
                        collective.pattern[j] = (collective.pattern[j] + res.pattern[j]) / 2.0;
                    }
                    collective.stability = (collective.stability + res.stability) / 2.0;
                }
            }
            instance.resonance_interference = collective;
        }

        Ok(catalyst)
    }

    /// Resolve a combat and determine evolution outcomes
    ///
    /// Transitions combat through Resolution to Complete state
    pub fn resolve_combat(
        &mut self,
        combat_id: CombatId,
        entity_resonances: &HashMap<EntityId, ResonancePattern>,
        entity_densities: &HashMap<EntityId, Density>,
        timestamp: Timestamp,
    ) -> Result<HashMap<EntityId, EvolutionOutcome>, CombatError> {
        let instance = self
            .active_combats
            .get_mut(&combat_id)
            .ok_or(CombatError::CombatNotFound(combat_id))?;

        // Begin resolution
        instance.begin_resolution()?;

        // Resolve combat
        let outcomes =
            CatalystResolution::resolve_combat(instance, entity_resonances, entity_densities)?;

        // Complete combat
        instance.complete(timestamp)?;

        // Move to history
        let completed_instance = self.active_combats.remove(&combat_id).unwrap();
        self.combat_history.push(completed_instance);

        Ok(outcomes)
    }

    /// Get the outcome of a completed combat
    pub fn get_combat_outcome(
        &self,
        combat_id: CombatId,
    ) -> Option<&HashMap<EntityId, EvolutionOutcome>> {
        // Check active combats
        if let Some(instance) = self.active_combats.get(&combat_id) {
            if instance.is_complete() {
                return Some(&instance.evolution_outcomes);
            }
        }

        // Check history
        for instance in &self.combat_history {
            if instance.instance_id == combat_id {
                return Some(&instance.evolution_outcomes);
            }
        }

        None
    }

    /// Get combat history for a specific entity
    pub fn get_entity_combat_history(&self, entity_id: EntityId) -> Vec<&CombatInstance> {
        let mut history = Vec::new();

        // Check active combats
        for instance in self.active_combats.values() {
            if instance.participants.contains(&entity_id) {
                history.push(instance);
            }
        }

        // Check completed combats
        for instance in &self.combat_history {
            if instance.participants.contains(&entity_id) {
                history.push(instance);
            }
        }

        history
    }

    /// Calculate collective catalyst for a combat
    ///
    /// Returns the total catalyst generated by the combat
    pub fn calculate_collective_catalyst(
        &self,
        combat_id: CombatId,
    ) -> Result<Catalyst, CombatError> {
        // Check active combats
        if let Some(instance) = self.active_combats.get(&combat_id) {
            return Ok(instance.catalyst_generated.clone());
        }

        // Check history
        for instance in &self.combat_history {
            if instance.instance_id == combat_id {
                return Ok(instance.catalyst_generated.clone());
            }
        }

        Err(CombatError::CombatNotFound(combat_id))
    }

    /// Get an active combat instance
    pub fn get_combat(&self, combat_id: CombatId) -> Option<&CombatInstance> {
        self.active_combats.get(&combat_id)
    }

    /// Get a mutable reference to an active combat
    pub fn get_combat_mut(&mut self, combat_id: CombatId) -> Option<&mut CombatInstance> {
        self.active_combats.get_mut(&combat_id)
    }

    /// Get all active combats
    pub fn get_active_combats(&self) -> &HashMap<CombatId, CombatInstance> {
        &self.active_combats
    }

    /// Get combat history
    pub fn get_combat_history(&self) -> &[CombatInstance] {
        &self.combat_history
    }

    /// Get the number of active combats
    pub fn active_combat_count(&self) -> usize {
        self.active_combats.len()
    }

    /// Get the total number of combats (active + history)
    pub fn total_combat_count(&self) -> usize {
        self.active_combats.len() + self.combat_history.len()
    }
}

impl Default for CatalystCombat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Catalyst Tests
    // =========================================================================

    #[test]
    fn test_catalyst_creation() {
        let catalyst = Catalyst::new(100.0, Density::Third, Polarity::ServiceToOthers);
        assert_eq!(catalyst.amount, 100.0);
        assert_eq!(catalyst.quality, CatalystQuality::Raw);
        assert_eq!(catalyst.density_level, Density::Third);
        assert_eq!(catalyst.polarity_alignment, Polarity::ServiceToOthers);
    }

    #[test]
    fn test_catalyst_with_archetypes() {
        let archetypes = vec![1, 5, 10];
        let catalyst =
            Catalyst::with_archetypes(50.0, Density::Fourth, Polarity::Neutral, archetypes.clone());
        assert_eq!(catalyst.amount, 50.0);
        assert_eq!(catalyst.source_archetypes, archetypes);
    }

    #[test]
    fn test_catalyst_refinement() {
        let catalyst = Catalyst::new(100.0, Density::Third, Polarity::Neutral);
        assert_eq!(catalyst.quality, CatalystQuality::Raw);

        let refined = catalyst.refine().unwrap();
        assert_eq!(refined.quality, CatalystQuality::Refined);
        assert!(refined.amount < catalyst.amount); // 10% loss

        let transformed = refined.refine().unwrap();
        assert_eq!(transformed.quality, CatalystQuality::Transformed);

        // Cannot refine beyond Transformed
        assert!(transformed.refine().is_none());
    }

    #[test]
    fn test_catalyst_combine() {
        let catalyst_a =
            Catalyst::with_archetypes(50.0, Density::Third, Polarity::ServiceToOthers, vec![1, 2]);
        let catalyst_b =
            Catalyst::with_archetypes(30.0, Density::Fourth, Polarity::ServiceToOthers, vec![2, 3]);

        let combined = catalyst_a.combine(&catalyst_b);
        assert_eq!(combined.amount, 80.0); // 50 + 30
        assert_eq!(combined.quality, CatalystQuality::Raw); // Same quality
        assert_eq!(combined.source_archetypes.len(), 3); // 1, 2, 3 (deduplicated)
        assert_eq!(combined.density_level, Density::Fourth); // Higher density
    }

    #[test]
    fn test_catalyst_combine_different_qualities() {
        let mut catalyst_a = Catalyst::new(50.0, Density::Third, Polarity::Neutral);
        catalyst_a.quality = CatalystQuality::Refined;

        let catalyst_b = Catalyst::new(30.0, Density::Third, Polarity::Neutral);
        // Raw quality

        let combined = catalyst_a.combine(&catalyst_b);
        assert_eq!(combined.quality, CatalystQuality::Refined); // Higher quality
    }

    #[test]
    fn test_catalyst_combine_different_polarities() {
        let catalyst_a = Catalyst::new(50.0, Density::Third, Polarity::ServiceToOthers);
        let catalyst_b = Catalyst::new(30.0, Density::Third, Polarity::ServiceToSelf);

        let combined = catalyst_a.combine(&catalyst_b);
        assert_eq!(combined.polarity_alignment, Polarity::Neutral); // Balanced
    }

    #[test]
    fn test_catalyst_evolutionary_potential() {
        let raw = Catalyst::new(100.0, Density::Fourth, Polarity::Neutral);
        let raw_potential = raw.get_evolutionary_potential();

        let mut refined = raw.clone();
        refined.quality = CatalystQuality::Refined;
        let refined_potential = refined.get_evolutionary_potential();

        let mut transformed = raw.clone();
        transformed.quality = CatalystQuality::Transformed;
        let transformed_potential = transformed.get_evolutionary_potential();

        assert!(transformed_potential > refined_potential);
        assert!(refined_potential > raw_potential);
    }

    #[test]
    fn test_catalyst_can_be_absorbed() {
        let catalyst = Catalyst::new(100.0, Density::Third, Polarity::Neutral);

        assert!(catalyst.can_be_absorbed_by(Density::Third));
        assert!(catalyst.can_be_absorbed_by(Density::Fourth));
        assert!(!catalyst.can_be_absorbed_by(Density::First));

        let high_catalyst = Catalyst::new(100.0, Density::Seventh, Polarity::Neutral);
        assert!(high_catalyst.can_be_absorbed_by(Density::Eighth));
        assert!(!high_catalyst.can_be_absorbed_by(Density::Fifth));
    }

    #[test]
    fn test_catalyst_quality_potency_multiplier() {
        assert_eq!(CatalystQuality::Raw.potency_multiplier(), 1.0);
        assert_eq!(CatalystQuality::Refined.potency_multiplier(), 1.5);
        assert_eq!(CatalystQuality::Transformed.potency_multiplier(), 2.0);
    }

    #[test]
    fn test_catalyst_quality_next() {
        assert_eq!(
            CatalystQuality::Raw.next_quality(),
            Some(CatalystQuality::Refined)
        );
        assert_eq!(
            CatalystQuality::Refined.next_quality(),
            Some(CatalystQuality::Transformed)
        );
        assert_eq!(CatalystQuality::Transformed.next_quality(), None);
    }

    // =========================================================================
    // Combat Instance Tests
    // =========================================================================

    #[test]
    fn test_combat_instance_creation() {
        let participants = vec![1, 2];
        let instance = CombatInstance::new(1, participants).unwrap();

        assert_eq!(instance.instance_id, 1);
        assert_eq!(instance.participants.len(), 2);
        assert_eq!(instance.combat_state, CombatState::Preparation);
        assert!(!instance.is_complete());
    }

    #[test]
    fn test_combat_instance_insufficient_participants() {
        let participants = vec![1];
        let result = CombatInstance::new(1, participants);

        assert!(matches!(
            result,
            Err(CombatError::InsufficientParticipants {
                required: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn test_combat_instance_too_many_participants() {
        let participants: Vec<EntityId> = (0..101).map(|i| i as u64).collect();
        let result = CombatInstance::new(1, participants);

        assert!(matches!(
            result,
            Err(CombatError::TooManyParticipants {
                max: 100,
                actual: 101
            })
        ));
    }

    #[test]
    fn test_combat_instance_add_participant() {
        let participants = vec![1, 2];
        let mut instance = CombatInstance::new(1, participants).unwrap();

        instance.add_participant(3).unwrap();
        assert_eq!(instance.participants.len(), 3);
    }

    #[test]
    fn test_combat_instance_add_duplicate_participant() {
        let participants = vec![1, 2];
        let mut instance = CombatInstance::new(1, participants).unwrap();

        let result = instance.add_participant(1);
        assert!(matches!(result, Err(CombatError::DuplicateParticipant(1))));
    }

    #[test]
    fn test_combat_instance_add_participant_wrong_state() {
        let participants = vec![1, 2];
        let mut instance = CombatInstance::new(1, participants).unwrap();

        instance.activate().unwrap();

        let result = instance.add_participant(3);
        assert!(matches!(
            result,
            Err(CombatError::InvalidCombatState {
                current: CombatState::Active,
                required: CombatState::Preparation
            })
        ));
    }

    #[test]
    fn test_combat_instance_state_transitions() {
        let participants = vec![1, 2];
        let mut instance = CombatInstance::new(1, participants).unwrap();

        assert_eq!(instance.combat_state, CombatState::Preparation);

        instance.activate().unwrap();
        assert_eq!(instance.combat_state, CombatState::Active);

        instance.begin_resolution().unwrap();
        assert_eq!(instance.combat_state, CombatState::Resolution);

        instance.complete(100).unwrap();
        assert_eq!(instance.combat_state, CombatState::Complete);
        assert!(instance.is_complete());
        assert_eq!(instance.resolution_time, Some(100));
    }

    #[test]
    fn test_combat_instance_invalid_state_transitions() {
        let participants = vec![1, 2];
        let mut instance = CombatInstance::new(1, participants).unwrap();

        // Cannot complete from Preparation
        let result = instance.complete(100);
        assert!(matches!(
            result,
            Err(CombatError::InvalidCombatState {
                current: CombatState::Preparation,
                required: CombatState::Resolution
            })
        ));

        // Cannot begin resolution from Preparation
        let result = instance.begin_resolution();
        assert!(matches!(
            result,
            Err(CombatError::InvalidCombatState {
                current: CombatState::Preparation,
                required: CombatState::Active
            })
        ));
    }

    // =========================================================================
    // Catalyst Generator Tests
    // =========================================================================

    #[test]
    fn test_catalyst_generator_new() {
        let generator = CatalystGenerator::new();
        // Just verify it creates successfully
        assert!(true);
    }

    #[test]
    fn test_generate_catalyst_from_resonance_clash() {
        let mut res_a = ResonancePattern::new();
        res_a.pattern = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        res_a.stability = 0.9;

        let mut res_b = ResonancePattern::new();
        res_b.pattern = [0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1];
        res_b.stability = 0.5;

        let catalyst =
            CatalystGenerator::generate_catalyst_from_resonance_clash(1, 2, &res_a, &res_b);

        assert!(catalyst.amount > 0.0);
        assert_eq!(catalyst.source_archetypes.len(), 2);
    }

    #[test]
    fn test_generate_catalyst_from_similar_resonance() {
        let res_a = ResonancePattern::new();
        let res_b = ResonancePattern::new();

        let catalyst =
            CatalystGenerator::generate_catalyst_from_resonance_clash(1, 2, &res_a, &res_b);

        // Similar patterns generate less catalyst
        assert!(catalyst.amount >= 0.0);
    }

    #[test]
    fn test_generate_catalyst_from_archetype_interference() {
        let archetypes = vec![1, 5, 10, 15, 20];
        let catalyst =
            CatalystGenerator::generate_catalyst_from_archetype_interference(&archetypes);

        assert!(catalyst.amount > 0.0);
        assert_eq!(catalyst.source_archetypes, archetypes);
        assert_eq!(catalyst.quality, CatalystQuality::Transformed); // 5+ archetypes
    }

    #[test]
    fn test_generate_catalyst_from_few_archetypes() {
        let archetypes = vec![1, 2];
        let catalyst =
            CatalystGenerator::generate_catalyst_from_archetype_interference(&archetypes);

        assert_eq!(catalyst.quality, CatalystQuality::Raw); // 2 archetypes
    }

    #[test]
    fn test_generate_catalyst_from_empty_archetypes() {
        let archetypes: Vec<ArchetypeId> = vec![];
        let catalyst =
            CatalystGenerator::generate_catalyst_from_archetype_interference(&archetypes);

        assert_eq!(catalyst.amount, 0.0);
    }

    #[test]
    fn test_generate_catalyst_from_polarity_conflict_opposing() {
        let catalyst = CatalystGenerator::generate_catalyst_from_polarity_conflict(
            Polarity::ServiceToSelf,
            Polarity::ServiceToOthers,
        );

        assert!(catalyst.amount > BASE_CATALYST_MULTIPLIER * 0.9); // Near max
        assert_eq!(catalyst.quality, CatalystQuality::Transformed);
        assert_eq!(catalyst.polarity_alignment, Polarity::Neutral);
    }

    #[test]
    fn test_generate_catalyst_from_polarity_conflict_same() {
        let catalyst = CatalystGenerator::generate_catalyst_from_polarity_conflict(
            Polarity::ServiceToSelf,
            Polarity::ServiceToSelf,
        );

        assert!(catalyst.amount < BASE_CATALYST_MULTIPLIER * 0.2); // Minimal
        assert_eq!(catalyst.quality, CatalystQuality::Raw);
    }

    #[test]
    fn test_generate_catalyst_from_polarity_conflict_neutral() {
        let catalyst = CatalystGenerator::generate_catalyst_from_polarity_conflict(
            Polarity::Neutral,
            Polarity::ServiceToOthers,
        );

        assert!(catalyst.amount >= BASE_CATALYST_MULTIPLIER * 0.4);
        assert!(catalyst.amount <= BASE_CATALYST_MULTIPLIER * 0.6);
    }

    #[test]
    fn test_generate_catalyst_from_density_difference_large() {
        let catalyst = CatalystGenerator::generate_catalyst_from_density_difference(
            Density::First,
            Density::Eighth,
        );

        assert!(catalyst.amount > BASE_CATALYST_MULTIPLIER); // Large difference
        assert_eq!(catalyst.quality, CatalystQuality::Transformed);
        assert_eq!(catalyst.density_level, Density::Eighth);
    }

    #[test]
    fn test_generate_catalyst_from_density_difference_small() {
        let catalyst = CatalystGenerator::generate_catalyst_from_density_difference(
            Density::Third,
            Density::Fourth,
        );

        assert!(catalyst.amount < BASE_CATALYST_MULTIPLIER); // Small difference
        assert_eq!(catalyst.quality, CatalystQuality::Refined);
    }

    #[test]
    fn test_generate_catalyst_from_same_density() {
        let catalyst = CatalystGenerator::generate_catalyst_from_density_difference(
            Density::Third,
            Density::Third,
        );

        assert_eq!(catalyst.amount, 0.0);
        assert_eq!(catalyst.quality, CatalystQuality::Raw);
    }

    #[test]
    fn test_generate_combined_catalyst() {
        let generator = CatalystGenerator::new();

        let res_a = ResonancePattern::new();
        let res_b = ResonancePattern::new();

        let resonance_pairs = vec![(1u64, 2u64, res_a, res_b)];
        let archetypes = vec![1u8, 2, 3];
        let polarities = vec![(Polarity::ServiceToSelf, Polarity::ServiceToOthers)];
        let densities = vec![(Density::Third, Density::Fifth)];

        let catalyst = generator.generate_combined_catalyst(
            &resonance_pairs,
            &archetypes,
            &polarities,
            &densities,
        );

        assert!(catalyst.amount > 0.0);
    }

    // =========================================================================
    // Evolution Outcome Tests
    // =========================================================================

    #[test]
    fn test_evolution_outcome_new() {
        let outcome = EvolutionOutcome::new();
        assert_eq!(outcome.density_progression, 0.0);
        assert_eq!(outcome.polarity_shift, 0.0);
        assert!(outcome.archetype_activations.is_empty());
        assert_eq!(outcome.catalyst_absorbed, 0.0);
        assert_eq!(outcome.catalyst_reflected, 0.0);
    }

    #[test]
    fn test_evolution_outcome_total_catalyst() {
        let mut outcome = EvolutionOutcome::new();
        outcome.catalyst_absorbed = 50.0;
        outcome.catalyst_reflected = 30.0;
        assert_eq!(outcome.total_catalyst(), 80.0);
    }

    #[test]
    fn test_evolution_outcome_is_significant() {
        let mut outcome = EvolutionOutcome::new();
        assert!(!outcome.is_significant());

        outcome.density_progression = 0.15;
        assert!(outcome.is_significant());

        outcome.density_progression = 0.0;
        outcome.catalyst_absorbed = 15.0;
        assert!(outcome.is_significant());
    }

    // =========================================================================
    // Catalyst Resolution Tests
    // =========================================================================

    #[test]
    fn test_calculate_catalyst_distribution() {
        let mut instance = CombatInstance::new(1, vec![1, 2]).unwrap();
        instance.activate().unwrap();
        instance.catalyst_generated = Catalyst::new(100.0, Density::Third, Polarity::Neutral);

        let mut entity_resonances = HashMap::new();
        entity_resonances.insert(1, ResonancePattern::new());
        entity_resonances.insert(2, ResonancePattern::new());

        let mut entity_densities = HashMap::new();
        entity_densities.insert(1, Density::Third);
        entity_densities.insert(2, Density::Fourth);

        let distribution = CatalystResolution::calculate_catalyst_distribution(
            &instance,
            &entity_resonances,
            &entity_densities,
        )
        .unwrap();

        assert_eq!(distribution.len(), 2);
        let total: Float = distribution.values().sum();
        assert!((total - 100.0).abs() < 0.01); // Should sum to total catalyst
    }

    #[test]
    fn test_calculate_catalyst_distribution_no_catalyst() {
        let instance = CombatInstance::new(1, vec![1, 2]).unwrap();

        let entity_resonances = HashMap::new();
        let entity_densities = HashMap::new();

        let distribution = CatalystResolution::calculate_catalyst_distribution(
            &instance,
            &entity_resonances,
            &entity_densities,
        )
        .unwrap();

        assert_eq!(distribution.get(&1), Some(&0.0));
        assert_eq!(distribution.get(&2), Some(&0.0));
    }

    #[test]
    fn test_determine_evolution_outcomes() {
        let mut distribution = HashMap::new();
        distribution.insert(1, 50.0);
        distribution.insert(2, 50.0);

        let interference = ResonancePattern::new();

        let mut entity_resonances = HashMap::new();
        entity_resonances.insert(1, ResonancePattern::new());
        entity_resonances.insert(2, ResonancePattern::new());

        let mut entity_densities = HashMap::new();
        entity_densities.insert(1, Density::Third);
        entity_densities.insert(2, Density::Fourth);

        let outcomes = CatalystResolution::determine_evolution_outcomes(
            &distribution,
            &interference,
            &entity_resonances,
            &entity_densities,
        );

        assert_eq!(outcomes.len(), 2);

        let outcome_1 = outcomes.get(&1).unwrap();
        assert!(outcome_1.catalyst_absorbed > 0.0);
        assert!(outcome_1.catalyst_reflected >= 0.0);
    }

    #[test]
    fn test_resolve_combat() {
        let mut instance = CombatInstance::new(1, vec![1, 2]).unwrap();
        instance.activate().unwrap();
        instance.catalyst_generated = Catalyst::new(100.0, Density::Third, Polarity::Neutral);
        instance.begin_resolution().unwrap();

        let mut entity_resonances = HashMap::new();
        entity_resonances.insert(1, ResonancePattern::new());
        entity_resonances.insert(2, ResonancePattern::new());

        let mut entity_densities = HashMap::new();
        entity_densities.insert(1, Density::Third);
        entity_densities.insert(2, Density::Fourth);

        let outcomes = CatalystResolution::resolve_combat(
            &mut instance,
            &entity_resonances,
            &entity_densities,
        )
        .unwrap();

        assert_eq!(outcomes.len(), 2);
        assert!(!instance.catalyst_distribution.is_empty());
    }

    #[test]
    fn test_resolve_combat_wrong_state() {
        let mut instance = CombatInstance::new(1, vec![1, 2]).unwrap();
        // Still in Preparation state

        let entity_resonances = HashMap::new();
        let entity_densities = HashMap::new();

        let result = CatalystResolution::resolve_combat(
            &mut instance,
            &entity_resonances,
            &entity_densities,
        );

        assert!(matches!(
            result,
            Err(CombatError::InvalidCombatState {
                current: CombatState::Preparation,
                required: CombatState::Resolution
            })
        ));
    }

    // =========================================================================
    // CatalystCombat System Tests
    // =========================================================================

    #[test]
    fn test_catalyst_combat_new() {
        let system = CatalystCombat::new();
        assert_eq!(system.active_combat_count(), 0);
        assert_eq!(system.total_combat_count(), 0);
    }

    #[test]
    fn test_initiate_combat() {
        let mut system = CatalystCombat::new();
        let instance = system.initiate_combat(vec![1, 2]).unwrap();

        assert_eq!(instance.instance_id, 1);
        assert_eq!(instance.participants, vec![1, 2]);
        assert_eq!(system.active_combat_count(), 1);
    }

    #[test]
    fn test_initiate_combat_insufficient_participants() {
        let mut system = CatalystCombat::new();
        let result = system.initiate_combat(vec![1]);

        assert!(matches!(
            result,
            Err(CombatError::InsufficientParticipants {
                required: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn test_add_participant() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();

        system.add_participant(1, 3).unwrap();

        let instance = system.get_combat(1).unwrap();
        assert_eq!(instance.participants.len(), 3);
    }

    #[test]
    fn test_add_participant_not_found() {
        let mut system = CatalystCombat::new();
        let result = system.add_participant(999, 3);

        assert!(matches!(result, Err(CombatError::CombatNotFound(999))));
    }

    #[test]
    fn test_generate_catalyst() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();

        let mut resonances = HashMap::new();
        resonances.insert(1, ResonancePattern::new());
        resonances.insert(2, ResonancePattern::new());

        let polarities = HashMap::new();
        let densities = HashMap::new();

        let catalyst = system
            .generate_catalyst(1, &resonances, &polarities, &densities)
            .unwrap();

        assert!(catalyst.amount >= 0.0);

        let instance = system.get_combat(1).unwrap();
        assert_eq!(instance.combat_state, CombatState::Active);
    }

    #[test]
    fn test_resolve_combat_full_flow() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();

        let mut resonances = HashMap::new();
        resonances.insert(1, ResonancePattern::new());
        resonances.insert(2, ResonancePattern::new());

        let polarities = HashMap::new();
        let mut densities = HashMap::new();
        densities.insert(1, Density::Third);
        densities.insert(2, Density::Fourth);

        // Generate catalyst
        system
            .generate_catalyst(1, &resonances, &polarities, &densities)
            .unwrap();

        // Resolve combat
        let outcomes = system
            .resolve_combat(1, &resonances, &densities, 100)
            .unwrap();

        assert_eq!(outcomes.len(), 2);
        assert_eq!(system.active_combat_count(), 0);
        assert_eq!(system.total_combat_count(), 1);

        // Check outcome retrieval
        let retrieved_outcome = system.get_combat_outcome(1);
        assert!(retrieved_outcome.is_some());
    }

    #[test]
    fn test_get_combat_outcome_incomplete() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();

        // Combat not complete yet
        let outcome = system.get_combat_outcome(1);
        assert!(outcome.is_none());
    }

    #[test]
    fn test_get_entity_combat_history() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();
        system.initiate_combat(vec![1, 3]).unwrap();
        system.initiate_combat(vec![2, 3]).unwrap();

        let history = system.get_entity_combat_history(1);
        assert_eq!(history.len(), 2); // Entity 1 in combats 1 and 2
    }

    #[test]
    fn test_calculate_collective_catalyst() {
        let mut system = CatalystCombat::new();
        system.initiate_combat(vec![1, 2]).unwrap();

        let mut resonances = HashMap::new();
        resonances.insert(1, ResonancePattern::new());
        resonances.insert(2, ResonancePattern::new());

        let polarities = HashMap::new();
        let densities = HashMap::new();

        system
            .generate_catalyst(1, &resonances, &polarities, &densities)
            .unwrap();

        let catalyst = system.calculate_collective_catalyst(1).unwrap();
        assert!(catalyst.amount >= 0.0);
    }

    #[test]
    fn test_calculate_collective_catalyst_not_found() {
        let system = CatalystCombat::new();
        let result = system.calculate_collective_catalyst(999);

        assert!(matches!(result, Err(CombatError::CombatNotFound(999))));
    }

    #[test]
    fn test_multiple_combats() {
        let mut system = CatalystCombat::new();

        // Create multiple combats
        for i in 0..5 {
            let participants = vec![i as u64 * 2, i as u64 * 2 + 1];
            system.initiate_combat(participants).unwrap();
        }

        assert_eq!(system.active_combat_count(), 5);

        // Verify different IDs
        for i in 1..=5 {
            assert!(system.get_combat(i).is_some());
        }
    }

    #[test]
    fn test_combat_state_display() {
        assert_eq!(format!("{}", CombatState::Preparation), "Preparation");
        assert_eq!(format!("{}", CombatState::Active), "Active");
        assert_eq!(format!("{}", CombatState::Resolution), "Resolution");
        assert_eq!(format!("{}", CombatState::Complete), "Complete");
    }

    #[test]
    fn test_catalyst_quality_display() {
        assert_eq!(format!("{}", CatalystQuality::Raw), "Raw");
        assert_eq!(format!("{}", CatalystQuality::Refined), "Refined");
        assert_eq!(format!("{}", CatalystQuality::Transformed), "Transformed");
    }

    #[test]
    fn test_combat_error_display() {
        let error = CombatError::InvalidCombatState {
            current: CombatState::Active,
            required: CombatState::Preparation,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid combat state"));
        assert!(msg.contains("Active"));
        assert!(msg.contains("Preparation"));
    }

    #[test]
    fn test_default_implementations() {
        let catalyst: Catalyst = Default::default();
        assert_eq!(catalyst.amount, 0.0);

        let outcome: EvolutionOutcome = Default::default();
        assert_eq!(outcome.density_progression, 0.0);

        let generator: CatalystGenerator = Default::default();
        assert!(true); // Just verify it creates

        let system: CatalystCombat = Default::default();
        assert_eq!(system.active_combat_count(), 0);
    }
}
