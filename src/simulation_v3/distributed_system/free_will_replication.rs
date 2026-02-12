//! Free Will Replication System
//!
//! This module implements the replication of Free Will choices across multiple peers,
//! with server-side validation and client reconciliation. Free Will is the 1st primal
//! distortion in the cosmological architecture - it's fundamental to the holographic system.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The first primal distortion is Free Will, which is
//! the focus of the Violet Realm and is the foundation for all other distortions."

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::Float;

// Re-export Density and Polarity from simulation_v3's density_mechanics
pub use crate::simulation_v3::density_mechanics::{Density, Polarity};

// Re-export ChoiceId from parent module
pub use super::ChoiceId;

// Type aliases for clarity
pub type RuleId = u64;
pub type LogEntryId = u64;
pub type ConflictId = u64;
pub type Timestamp = u64;
pub type PeerId = u64;
pub type FieldSignature = Vec<u8>;
pub type ErrorCode = String;
pub type EntityId = u64; // Simplified entity ID for this module

// Re-export EntityType
pub use crate::entity_layer7::layer7::EntityType;

// ============================================================================
// Choice Types and Data
// ============================================================================

/// Types of Free Will choices that entities can make
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will manifests as the ability to make
/// choices at any level of consciousness, from density transitions to daily actions."
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FreeWillType {
    /// Choose archetype path (Archetype 22 and sub-archetypes)
    ArchetypeChoice,
    /// Choose polarity (Service to Self or Service to Others)
    PolarityChoice,
    /// Transition to a new density
    DensityTransition,
    /// Align resonance pattern with others
    ResonanceAlignment,
    /// Choose a specific action
    ActionChoice,
    /// Choose how to interact with another entity
    InteractionChoice,
    /// Make an evolutionary leap
    EvolutionaryChoice,
}

impl Display for FreeWillType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FreeWillType::ArchetypeChoice => write!(f, "ArchetypeChoice"),
            FreeWillType::PolarityChoice => write!(f, "PolarityChoice"),
            FreeWillType::DensityTransition => write!(f, "DensityTransition"),
            FreeWillType::ResonanceAlignment => write!(f, "ResonanceAlignment"),
            FreeWillType::ActionChoice => write!(f, "ActionChoice"),
            FreeWillType::InteractionChoice => write!(f, "InteractionChoice"),
            FreeWillType::EvolutionaryChoice => write!(f, "EvolutionaryChoice"),
        }
    }
}

/// Data associated with a Free Will choice (varies by type)
#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceData {
    ArchetypeData {
        archetype_id: u64,
    },
    PolarityData {
        polarity: InternalPolarity,
    },
    DensityData {
        target_density: Density,
    },
    ResonanceData {
        target_pattern: ResonancePattern,
    },
    ActionData {
        action_type: ActionType,
        target_id: Option<u64>,
    },
    InteractionData {
        interaction_type: InteractionType,
        with_entity: u64,
    },
    EvolutionaryData {
        leap_type: LeapType,
    },
}

/// Helper to check if a density transition is valid
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Entities progress through densities through
/// evolutionary leaps, typically moving one density at a time."
pub fn can_transition_density(from: Density, to: Density) -> bool {
    let from_val = from.value() as i8;
    let to_val = to.value() as i8;
    // Can transition to next density or skip one density at most
    let diff = to_val - from_val;
    diff >= 1 && diff <= 2
}

/// Convert density_mechanics::Polarity to simplified internal form
fn polarity_to_internal(p: Polarity) -> InternalPolarity {
    match p {
        Polarity::ServiceToOthers => InternalPolarity::STO,
        Polarity::ServiceToSelf => InternalPolarity::STS,
        Polarity::Undecided | Polarity::Balanced => InternalPolarity::Unpolarized,
    }
}

/// Convert from internal form back to density_mechanics::Polarity
fn internal_to_polarity(p: InternalPolarity) -> Polarity {
    match p {
        InternalPolarity::STO => Polarity::ServiceToOthers,
        InternalPolarity::STS => Polarity::ServiceToSelf,
        InternalPolarity::Unpolarized => Polarity::Undecided,
    }
}

/// Internal simplified polarity representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InternalPolarity {
    /// Service to Self (negative polarity)
    STS,
    /// Service to Others (positive polarity)
    STO,
    /// Unpolarized or in transition
    Unpolarized,
}

/// Resonance pattern for alignment between entities
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Resonance represents the vibrational alignment
/// between entities, enabling collective consciousness formation."
#[derive(Debug, Clone, PartialEq)]
pub struct ResonancePattern {
    pub frequency: Float,
    pub phase: Float,
    pub amplitude: Float,
}

impl ResonancePattern {
    pub fn new(frequency: Float, phase: Float, amplitude: Float) -> Self {
        ResonancePattern {
            frequency,
            phase,
            amplitude,
        }
    }

    /// Calculate compatibility with another pattern
    pub fn compatibility_with(&self, other: &ResonancePattern) -> Float {
        let freq_diff = (self.frequency - other.frequency).abs();
        let phase_diff = (self.phase - other.phase).abs();
        let amp_diff = (self.amplitude - other.amplitude).abs();

        // Higher compatibility when values are closer
        1.0 - (freq_diff + phase_diff + amp_diff) / 3.0
    }
}

/// Action types for entity actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// Move to new location
    Move,
    /// Communicate with another entity
    Communicate,
    /// Create something new
    Create,
    /// Transform or change state
    Transform,
    /// Observe or perceive
    Observe,
    /// Meditate or reflect
    Meditate,
    /// Share energy with others
    Share,
    /// Accumulate energy
    Accumulate,
}

/// Interaction types for entity interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionType {
    /// Offer help or service
    Offer,
    /// Request help or service
    Request,
    /// Form a bond or relationship
    Bond,
    /// Dissolve a bond
    Release,
    /// Exchange information
    Exchange,
    /// Harmonize or synchronize
    Harmonize,
}

/// Evolutionary leap types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeapType {
    /// Density leap (major)
    DensityLeap,
    /// Archetype mastery (major)
    ArchetypeMastery,
    /// Resonance breakthrough (major)
    ResonanceBreakthrough,
    /// Consciousness expansion (minor)
    ConsciousnessExpansion,
    /// Service polarity integration (minor)
    PolarityIntegration,
}

/// Represents a player's Free Will choice
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will choices are fundamental operations
/// of the holographic system, affecting both individual and collective evolution."
#[derive(Debug, Clone, PartialEq)]
pub struct FreeWillChoice {
    pub choice_id: ChoiceId,
    pub entity_id: EntityId,
    pub player_id: PeerId,
    pub choice_type: FreeWillType,
    pub choice_data: ChoiceData,
    pub timestamp: Timestamp,
    pub choice_signature: FieldSignature,
    pub is_validated: bool,
}

impl FreeWillChoice {
    /// Create a new Free Will choice
    pub fn new(
        choice_id: ChoiceId,
        entity_id: EntityId,
        player_id: PeerId,
        choice_type: FreeWillType,
        choice_data: ChoiceData,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        FreeWillChoice {
            choice_id,
            entity_id,
            player_id,
            choice_type,
            choice_data,
            timestamp,
            choice_signature: Vec::new(), // To be signed
            is_validated: false,
        }
    }

    /// Validate the cryptographic signature
    pub fn validate(&self) -> bool {
        // In a real implementation, this would verify the signature
        // For now, we check if a signature exists
        !self.choice_signature.is_empty()
    }

    /// Check if this choice is compatible with another choice
    pub fn is_compatible_with(&self, other: &FreeWillChoice) -> bool {
        // Same entity and choice type = potential conflict
        if self.entity_id == other.entity_id && self.choice_type == other.choice_type {
            // Compatible if the choice data is the same
            self.choice_data == other.choice_data
        } else {
            // Different entities or choice types are generally compatible
            true
        }
    }

    /// Serialize the choice to bytes
    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Serialize choice_id
        bytes.extend_from_slice(&self.choice_id.to_le_bytes());

        // Serialize entity_id
        bytes.extend_from_slice(&self.entity_id.to_le_bytes());

        // Serialize player_id
        bytes.extend_from_slice(&self.player_id.to_le_bytes());

        // Serialize choice_type
        let type_byte = match self.choice_type {
            FreeWillType::ArchetypeChoice => 0u8,
            FreeWillType::PolarityChoice => 1,
            FreeWillType::DensityTransition => 2,
            FreeWillType::ResonanceAlignment => 3,
            FreeWillType::ActionChoice => 4,
            FreeWillType::InteractionChoice => 5,
            FreeWillType::EvolutionaryChoice => 6,
        };
        bytes.push(type_byte);

        // Serialize choice_data (simplified)
        match &self.choice_data {
            ChoiceData::ArchetypeData { archetype_id } => {
                bytes.extend_from_slice(&archetype_id.to_le_bytes());
            }
            ChoiceData::PolarityData { polarity } => {
                let pol_byte = match polarity {
                    InternalPolarity::STS => 0u8,
                    InternalPolarity::STO => 1,
                    InternalPolarity::Unpolarized => 2,
                };
                bytes.push(pol_byte);
            }
            ChoiceData::DensityData { target_density } => {
                bytes.push(target_density.value() as u8);
            }
            ChoiceData::ResonanceData { target_pattern } => {
                bytes.extend_from_slice(&target_pattern.frequency.to_le_bytes());
                bytes.extend_from_slice(&target_pattern.phase.to_le_bytes());
                bytes.extend_from_slice(&target_pattern.amplitude.to_le_bytes());
            }
            ChoiceData::ActionData {
                action_type,
                target_id,
            } => {
                let act_byte = match action_type {
                    ActionType::Move => 0u8,
                    ActionType::Communicate => 1,
                    ActionType::Create => 2,
                    ActionType::Transform => 3,
                    ActionType::Observe => 4,
                    ActionType::Meditate => 5,
                    ActionType::Share => 6,
                    ActionType::Accumulate => 7,
                };
                bytes.push(act_byte);
                if let Some(tid) = target_id {
                    bytes.extend_from_slice(&tid.to_le_bytes());
                }
            }
            ChoiceData::InteractionData {
                interaction_type,
                with_entity,
            } => {
                let int_byte = match interaction_type {
                    InteractionType::Offer => 0u8,
                    InteractionType::Request => 1,
                    InteractionType::Bond => 2,
                    InteractionType::Release => 3,
                    InteractionType::Exchange => 4,
                    InteractionType::Harmonize => 5,
                };
                bytes.push(int_byte);
                bytes.extend_from_slice(&with_entity.to_le_bytes());
            }
            ChoiceData::EvolutionaryData { leap_type } => {
                let leap_byte = match leap_type {
                    LeapType::DensityLeap => 0u8,
                    LeapType::ArchetypeMastery => 1,
                    LeapType::ResonanceBreakthrough => 2,
                    LeapType::ConsciousnessExpansion => 3,
                    LeapType::PolarityIntegration => 4,
                };
                bytes.push(leap_byte);
            }
        }

        // Serialize timestamp
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());

        // Serialize signature length and signature
        bytes.extend_from_slice(&(self.choice_signature.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&self.choice_signature);

        // Serialize validation status
        bytes.push(self.is_validated as u8);

        bytes
    }

    /// Deserialize a choice from bytes
    pub fn deserialize(data: &[u8]) -> Result<Self, FreeWillReplicationError> {
        let mut offset = 0;

        // Read choice_id
        if data.len() < offset + 8 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for choice_id".to_string(),
            ));
        }
        let choice_id_val = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        let choice_id = ChoiceId(choice_id_val);
        offset += 8;

        // Read entity_id
        if data.len() < offset + 8 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for entity_id".to_string(),
            ));
        }
        let entity_id = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        // Read player_id
        if data.len() < offset + 8 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for player_id".to_string(),
            ));
        }
        let player_id = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        // Read choice_type
        if data.len() < offset + 1 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for choice_type".to_string(),
            ));
        }
        let choice_type = match data[offset] {
            0 => FreeWillType::ArchetypeChoice,
            1 => FreeWillType::PolarityChoice,
            2 => FreeWillType::DensityTransition,
            3 => FreeWillType::ResonanceAlignment,
            4 => FreeWillType::ActionChoice,
            5 => FreeWillType::InteractionChoice,
            6 => FreeWillType::EvolutionaryChoice,
            _ => {
                return Err(FreeWillReplicationError::DeserializationError(
                    "Invalid choice_type".to_string(),
                ))
            }
        };
        offset += 1;

        // Read choice_data based on type
        let choice_data = match choice_type {
            FreeWillType::ArchetypeChoice => {
                if data.len() < offset + 8 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for ArchetypeData".to_string(),
                    ));
                }
                let archetype_id = u64::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]);
                offset += 8;
                ChoiceData::ArchetypeData { archetype_id }
            }
            FreeWillType::PolarityChoice => {
                if data.len() < offset + 1 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for PolarityData".to_string(),
                    ));
                }
                let polarity = match data[offset] {
                    0 => InternalPolarity::STS,
                    1 => InternalPolarity::STO,
                    2 => InternalPolarity::Unpolarized,
                    _ => {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Invalid polarity".to_string(),
                        ))
                    }
                };
                offset += 1;
                ChoiceData::PolarityData { polarity }
            }
            FreeWillType::DensityTransition => {
                if data.len() < offset + 1 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for DensityData".to_string(),
                    ));
                }
                let density_val = data[offset];
                let target_density = match density_val {
                    1 => Density::First,
                    2 => Density::Second,
                    3 => Density::Third,
                    4 => Density::Fourth,
                    5 => Density::Fifth,
                    6 => Density::Sixth,
                    7 => Density::Seventh,
                    8 => Density::Eighth,
                    _ => {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Invalid density".to_string(),
                        ))
                    }
                };
                offset += 1;
                ChoiceData::DensityData { target_density }
            }
            FreeWillType::ResonanceAlignment => {
                if data.len() < offset + 24 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for ResonanceData".to_string(),
                    ));
                }
                let frequency = Float::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]);
                offset += 8;
                let phase = Float::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]);
                offset += 8;
                let amplitude = Float::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]);
                offset += 8;
                let target_pattern = ResonancePattern {
                    frequency,
                    phase,
                    amplitude,
                };
                ChoiceData::ResonanceData { target_pattern }
            }
            FreeWillType::ActionChoice => {
                if data.len() < offset + 1 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for ActionData".to_string(),
                    ));
                }
                let action_type = match data[offset] {
                    0 => ActionType::Move,
                    1 => ActionType::Communicate,
                    2 => ActionType::Create,
                    3 => ActionType::Transform,
                    4 => ActionType::Observe,
                    5 => ActionType::Meditate,
                    6 => ActionType::Share,
                    7 => ActionType::Accumulate,
                    _ => {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Invalid action_type".to_string(),
                        ))
                    }
                };
                offset += 1;

                let mut target_id = None;
                // Check if target_id is present (for some actions)
                if matches!(action_type, ActionType::Communicate | ActionType::Share) {
                    if data.len() < offset + 8 {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Insufficient data for target_id".to_string(),
                        ));
                    }
                    let tid = u64::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                        data[offset + 4],
                        data[offset + 5],
                        data[offset + 6],
                        data[offset + 7],
                    ]);
                    offset += 8;
                    target_id = Some(tid);
                }

                ChoiceData::ActionData {
                    action_type,
                    target_id,
                }
            }
            FreeWillType::InteractionChoice => {
                if data.len() < offset + 9 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for InteractionData".to_string(),
                    ));
                }
                let interaction_type = match data[offset] {
                    0 => InteractionType::Offer,
                    1 => InteractionType::Request,
                    2 => InteractionType::Bond,
                    3 => InteractionType::Release,
                    4 => InteractionType::Exchange,
                    5 => InteractionType::Harmonize,
                    _ => {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Invalid interaction_type".to_string(),
                        ))
                    }
                };
                offset += 1;
                let with_entity = u64::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                    data[offset + 4],
                    data[offset + 5],
                    data[offset + 6],
                    data[offset + 7],
                ]);
                offset += 8;
                ChoiceData::InteractionData {
                    interaction_type,
                    with_entity,
                }
            }
            FreeWillType::EvolutionaryChoice => {
                if data.len() < offset + 1 {
                    return Err(FreeWillReplicationError::DeserializationError(
                        "Insufficient data for EvolutionaryData".to_string(),
                    ));
                }
                let leap_type = match data[offset] {
                    0 => LeapType::DensityLeap,
                    1 => LeapType::ArchetypeMastery,
                    2 => LeapType::ResonanceBreakthrough,
                    3 => LeapType::ConsciousnessExpansion,
                    4 => LeapType::PolarityIntegration,
                    _ => {
                        return Err(FreeWillReplicationError::DeserializationError(
                            "Invalid leap_type".to_string(),
                        ))
                    }
                };
                offset += 1;
                ChoiceData::EvolutionaryData { leap_type }
            }
        };

        // Read timestamp
        if data.len() < offset + 8 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for timestamp".to_string(),
            ));
        }
        let timestamp = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        // Read signature length and signature
        if data.len() < offset + 2 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for signature length".to_string(),
            ));
        }
        let sig_len = u16::from_le_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;

        if data.len() < offset + sig_len {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for signature".to_string(),
            ));
        }
        let choice_signature = data[offset..offset + sig_len].to_vec();
        offset += sig_len;

        // Read validation status
        if data.len() < offset + 1 {
            return Err(FreeWillReplicationError::DeserializationError(
                "Insufficient data for validation status".to_string(),
            ));
        }
        let is_validated = data[offset] != 0;

        Ok(FreeWillChoice {
            choice_id,
            entity_id,
            player_id,
            choice_type,
            choice_data,
            timestamp,
            choice_signature,
            is_validated,
        })
    }

    /// Sign the choice with a signature (placeholder for real cryptographic signing)
    pub fn sign(&mut self, _private_key: &[u8]) {
        // In a real implementation, this would create a cryptographic signature
        // For now, we create a placeholder signature
        self.choice_signature = format!("signature_{}", self.choice_id).into_bytes();
    }
}

// ============================================================================
// Entity State (for validation)
// ============================================================================

/// Entity state for validation purposes
#[derive(Debug, Clone, PartialEq)]
pub struct EntityState {
    pub entity_id: EntityId,
    pub current_density: Density,
    pub current_polarity: InternalPolarity,
    pub resonance_pattern: ResonancePattern,
    pub last_choice_timestamp: Timestamp,
}

impl EntityState {
    pub fn new(entity_id: EntityId, current_density: Density) -> Self {
        EntityState {
            entity_id,
            current_density,
            current_polarity: InternalPolarity::Unpolarized,
            resonance_pattern: ResonancePattern::new(1.0, 0.0, 1.0),
            last_choice_timestamp: 0,
        }
    }

    pub fn with_polarity(mut self, polarity: InternalPolarity) -> Self {
        self.current_polarity = polarity;
        self
    }

    pub fn with_resonance(mut self, pattern: ResonancePattern) -> Self {
        self.resonance_pattern = pattern;
        self
    }
}

// ============================================================================
// Validation System
// ============================================================================

/// Types of validation rules
#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    DensityRequirement {
        min_density: Density,
        max_density: Density,
    },
    ResonanceRequirement {
        required_pattern: ResonancePattern,
    },
    CooldownConstraint {
        cooldown_duration: Timestamp,
    },
    FrequencyLimit {
        max_choices_per_period: u32,
        period_duration: Timestamp,
    },
    ArchetypeConstraint {
        allowed_archetypes: Vec<u64>,
    },
    PolarityConstraint {
        allowed_polarities: Vec<InternalPolarity>,
    },
    StateConsistency,
}

/// Validation condition for a rule
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationCondition {
    ChoiceTypeEquals(FreeWillType),
    DensityInRange(Density, Density),
    ResonanceAboveThreshold(Float),
    ChoiceCountLessThan(u32),
    AlwaysTrue,
    NeverTrue,
}

impl ValidationCondition {
    pub fn evaluate(&self, _choice: &FreeWillChoice, _entity_state: &EntityState) -> bool {
        match self {
            ValidationCondition::AlwaysTrue => true,
            ValidationCondition::NeverTrue => false,
            ValidationCondition::ChoiceTypeEquals(expected_type) => {
                _choice.choice_type == *expected_type
            }
            ValidationCondition::DensityInRange(min, max) => {
                let d = _entity_state.current_density as i8;
                d >= *min as i8 && d <= *max as i8
            }
            ValidationCondition::ResonanceAboveThreshold(threshold) => {
                _entity_state.resonance_pattern.frequency >= *threshold
            }
            ValidationCondition::ChoiceCountLessThan(_max) => {
                // Would need access to choice history
                true
            }
        }
    }
}

/// A validation rule for Free Will choices
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationRule {
    pub rule_id: RuleId,
    pub rule_type: RuleType,
    pub condition: ValidationCondition,
    pub is_required: bool,
    pub error_message: String,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new(
        rule_id: RuleId,
        rule_type: RuleType,
        condition: ValidationCondition,
        error_message: String,
    ) -> Self {
        ValidationRule {
            rule_id,
            rule_type,
            condition,
            is_required: false,
            error_message,
        }
    }

    /// Create a required validation rule
    pub fn required(
        rule_id: RuleId,
        rule_type: RuleType,
        condition: ValidationCondition,
        error_message: String,
    ) -> Self {
        ValidationRule {
            rule_id,
            rule_type,
            condition,
            is_required: true,
            error_message,
        }
    }

    /// Evaluate the rule against a choice and entity state
    pub fn evaluate(&self, choice: &FreeWillChoice, entity_state: &EntityState) -> bool {
        self.condition.evaluate(choice, entity_state)
    }

    /// Get the error message for this rule
    pub fn get_error_message(&self) -> String {
        self.error_message.clone()
    }
}

/// Result of validating a choice
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub choice_id: ChoiceId,
    pub validation_timestamp: Timestamp,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success(choice_id: ChoiceId) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ValidationResult {
            is_valid: true,
            choice_id,
            validation_timestamp: timestamp,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a failed validation result
    pub fn failure(choice_id: ChoiceId, errors: Vec<ValidationError>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ValidationResult {
            is_valid: false,
            choice_id,
            validation_timestamp: timestamp,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn add_warning(mut self, warning: ValidationWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Get a summary of the validation result
    pub fn get_summary(&self) -> String {
        if self.is_valid {
            let warning_count = self.warnings.len();
            if warning_count > 0 {
                format!("Valid with {} warning(s)", warning_count)
            } else {
                "Valid".to_string()
            }
        } else {
            format!("Invalid: {} error(s)", self.errors.len())
        }
    }
}

/// A validation error
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub error_code: ErrorCode,
    pub message: String,
    pub rule_id: Option<RuleId>,
}

impl ValidationError {
    pub fn new(error_code: ErrorCode, message: String) -> Self {
        ValidationError {
            error_code,
            message,
            rule_id: None,
        }
    }

    pub fn with_rule_id(mut self, rule_id: RuleId) -> Self {
        self.rule_id = Some(rule_id);
        self
    }
}

/// A validation warning
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationWarning {
    pub warning_code: ErrorCode,
    pub message: String,
}

impl ValidationWarning {
    pub fn new(warning_code: ErrorCode, message: String) -> Self {
        ValidationWarning {
            warning_code,
            message,
        }
    }
}

/// Compatibility check between two choices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    FullyCompatible,
    PartiallyCompatible,
    Incompatible,
}

/// Server-side validator for Free Will choices
#[derive(Debug, Clone)]
pub struct FreeWillValidator {
    validation_rules: Vec<ValidationRule>,
    choice_log: ChoiceLog,
}

impl FreeWillValidator {
    /// Create a new validator with default rules
    pub fn new() -> Self {
        let mut validator = FreeWillValidator {
            validation_rules: Vec::new(),
            choice_log: ChoiceLog::new(),
        };

        // Add default validation rules
        validator.add_default_rules();

        validator
    }

    /// Add default validation rules
    fn add_default_rules(&mut self) {
        // Density transition rule
        let density_rule = ValidationRule::required(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::ChoiceTypeEquals(FreeWillType::DensityTransition),
            "Density transition must be within valid range (1-8)".to_string(),
        );
        self.validation_rules.push(density_rule);

        // Frequency limit rule
        let frequency_rule = ValidationRule::new(
            2,
            RuleType::FrequencyLimit {
                max_choices_per_period: 100,
                period_duration: 3600, // 1 hour
            },
            ValidationCondition::AlwaysTrue,
            "Choice frequency limit: 100 per hour".to_string(),
        );
        self.validation_rules.push(frequency_rule);
    }

    /// Validate a choice against all rules
    pub fn validate_choice(
        &self,
        choice: &FreeWillChoice,
        entity_state: &EntityState,
    ) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check signature
        if !choice.validate() {
            errors.push(ValidationError::new(
                "SIGNATURE_INVALID".to_string(),
                "Choice signature is invalid or missing".to_string(),
            ));
        }

        // Check entity ID matches
        if choice.entity_id != entity_state.entity_id {
            errors.push(ValidationError::new(
                "ENTITY_MISMATCH".to_string(),
                "Choice entity ID does not match entity state".to_string(),
            ));
        }

        // Apply validation rules
        for rule in &self.validation_rules {
            if rule.evaluate(choice, entity_state) {
                // Rule passed
                continue;
            }

            if rule.is_required {
                errors.push(
                    ValidationError::new("RULE_VIOLATION".to_string(), rule.get_error_message())
                        .with_rule_id(rule.rule_id),
                );
            } else {
                warnings.push(ValidationWarning::new(
                    "RULE_WARNING".to_string(),
                    rule.get_error_message(),
                ));
            }
        }

        if errors.is_empty() {
            ValidationResult::success(choice.choice_id).add_warning(ValidationWarning::new(
                "WARN".to_string(),
                "Warning".to_string(),
            ))
        } else {
            ValidationResult::failure(choice.choice_id, errors).add_warning(ValidationWarning::new(
                "WARN".to_string(),
                "Warning".to_string(),
            ))
        }
    }

    /// Add a validation rule
    pub fn add_validation_rule(&mut self, rule: ValidationRule) {
        self.validation_rules.push(rule);
    }

    /// Remove a validation rule
    pub fn remove_validation_rule(&mut self, rule_id: RuleId) {
        self.validation_rules.retain(|r| r.rule_id != rule_id);
    }

    /// Get validation history for an entity
    pub fn get_validation_history(&self, entity_id: EntityId) -> Vec<&ValidationResult> {
        self.choice_log
            .get_entity_history(entity_id)
            .iter()
            .map(|entry| &entry.validation_result)
            .collect()
    }

    /// Check compatibility between two choices
    pub fn check_choice_compatibility(
        &self,
        choice_a: &FreeWillChoice,
        choice_b: &FreeWillChoice,
    ) -> Compatibility {
        if choice_a.is_compatible_with(choice_b) {
            Compatibility::FullyCompatible
        } else if choice_a.entity_id == choice_b.entity_id {
            Compatibility::PartiallyCompatible
        } else {
            Compatibility::Incompatible
        }
    }
}

impl Default for FreeWillValidator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Client-Server Reconciliation
// ============================================================================

/// Reconciliation strategies for resolving client-server conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconciliationStrategy {
    /// Server choices take precedence
    ServerWins,
    /// Client choices take precedence
    ClientWins,
    /// Merge both choices where possible
    Merge,
    /// Require user input for conflicts
    Interactive,
}

/// Types of conflicts between client and server choices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictType {
    /// Both made choices for the same entity
    ConcurrentChoice,
    /// Choices contradict each other
    ContradictoryChoice,
    /// Choice inconsistent with entity state
    StateInconsistency,
    /// Choice failed validation
    ValidationFailure,
}

/// Resolution type for a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionType {
    ServerChosen,
    ClientChosen,
    BothDiscarded,
    Merged,
    PendingUserInput,
}

/// Resolution of a conflict
#[derive(Debug, Clone, PartialEq)]
pub struct ConflictResolution {
    pub resolution_type: ResolutionType,
    pub chosen_choice: Option<FreeWillChoice>,
    pub rationale: String,
}

impl ConflictResolution {
    pub fn server_choice(choice: FreeWillChoice) -> Self {
        ConflictResolution {
            resolution_type: ResolutionType::ServerChosen,
            chosen_choice: Some(choice),
            rationale: "Server choice takes precedence".to_string(),
        }
    }

    pub fn client_choice(choice: FreeWillChoice) -> Self {
        ConflictResolution {
            resolution_type: ResolutionType::ClientChosen,
            chosen_choice: Some(choice),
            rationale: "Client choice takes precedence".to_string(),
        }
    }

    pub fn both_discarded() -> Self {
        ConflictResolution {
            resolution_type: ResolutionType::BothDiscarded,
            chosen_choice: None,
            rationale: "Both choices discarded due to conflict".to_string(),
        }
    }
}

/// Conflict between client and server choices
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceConflict {
    pub conflict_id: ConflictId,
    pub client_choice: FreeWillChoice,
    pub server_choice: FreeWillChoice,
    pub conflict_type: ConflictType,
    pub resolution: Option<ConflictResolution>,
}

impl ChoiceConflict {
    pub fn new(
        conflict_id: ConflictId,
        client_choice: FreeWillChoice,
        server_choice: FreeWillChoice,
        conflict_type: ConflictType,
    ) -> Self {
        ChoiceConflict {
            conflict_id,
            client_choice,
            server_choice,
            conflict_type,
            resolution: None,
        }
    }

    /// Resolve the conflict with a resolution
    pub fn resolve(&mut self, resolution: ConflictResolution) {
        self.resolution = Some(resolution);
    }
}

/// Result of reconciliation
#[derive(Debug, Clone, PartialEq)]
pub struct ReconciliationResult {
    pub reconciled_choices: Vec<FreeWillChoice>,
    pub conflicts_resolved: Vec<ChoiceConflict>,
    pub choices_discarded: Vec<ChoiceId>,
    pub reconciliation_timestamp: Timestamp,
}

impl ReconciliationResult {
    pub fn new(
        reconciled_choices: Vec<FreeWillChoice>,
        conflicts_resolved: Vec<ChoiceConflict>,
        choices_discarded: Vec<ChoiceId>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ReconciliationResult {
            reconciled_choices,
            conflicts_resolved,
            choices_discarded,
            reconciliation_timestamp: timestamp,
        }
    }
}

/// Summary of reconciliation
#[derive(Debug, Clone)]
pub struct ReconciliationSummary {
    pub total_client_choices: usize,
    pub total_server_choices: usize,
    pub conflicts_detected: usize,
    pub conflicts_resolved: usize,
    pub choices_reconciled: usize,
    pub choices_discarded: usize,
}

/// Client-side reconciliation of choices with server
#[derive(Debug, Clone)]
pub struct ClientReconciliation {
    client_choices: HashMap<ChoiceId, FreeWillChoice>,
    server_choices: HashMap<ChoiceId, FreeWillChoice>,
    reconciliation_strategy: ReconciliationStrategy,
}

impl ClientReconciliation {
    /// Create a new reconciliation with a strategy
    pub fn new(strategy: ReconciliationStrategy) -> Self {
        ClientReconciliation {
            client_choices: HashMap::new(),
            server_choices: HashMap::new(),
            reconciliation_strategy: strategy,
        }
    }

    /// Add a client choice
    pub fn add_client_choice(&mut self, choice: FreeWillChoice) {
        self.client_choices.insert(choice.choice_id, choice);
    }

    /// Add a server choice
    pub fn add_server_choice(&mut self, choice: FreeWillChoice) {
        self.server_choices.insert(choice.choice_id, choice);
    }

    /// Detect conflicts between client and server choices
    pub fn detect_conflicts(&self) -> Vec<ChoiceConflict> {
        let mut conflicts = Vec::new();
        let mut conflict_id = 0u64;

        // Find choices with same ID (both sides made a choice)
        for choice_id in self.client_choices.keys() {
            if let Some(server_choice) = self.server_choices.get(choice_id) {
                let client_choice = self.client_choices.get(choice_id).unwrap();

                if client_choice != server_choice {
                    // Same choice ID but different data = conflict
                    let conflict_type = if client_choice.entity_id == server_choice.entity_id {
                        ConflictType::ConcurrentChoice
                    } else {
                        ConflictType::ContradictoryChoice
                    };

                    conflicts.push(ChoiceConflict::new(
                        conflict_id,
                        client_choice.clone(),
                        server_choice.clone(),
                        conflict_type,
                    ));
                    conflict_id += 1;
                }
            }
        }

        conflicts
    }

    /// Reconcile client and server choices
    pub fn reconcile(&mut self) -> Result<ReconciliationResult, FreeWillReplicationError> {
        let mut reconciled_choices = Vec::new();
        let mut conflicts_resolved = Vec::new();
        let mut choices_discarded = Vec::new();

        // Detect conflicts
        let conflicts = self.detect_conflicts();

        // Resolve conflicts based on strategy
        for mut conflict in conflicts {
            match self.reconciliation_strategy {
                ReconciliationStrategy::ServerWins => {
                    let resolution =
                        ConflictResolution::server_choice(conflict.server_choice.clone());
                    reconciled_choices.push(conflict.server_choice.clone());
                    conflict.resolve(resolution);
                    conflicts_resolved.push(conflict);
                }
                ReconciliationStrategy::ClientWins => {
                    let resolution =
                        ConflictResolution::client_choice(conflict.client_choice.clone());
                    reconciled_choices.push(conflict.client_choice.clone());
                    conflict.resolve(resolution);
                    conflicts_resolved.push(conflict);
                }
                ReconciliationStrategy::Merge => {
                    // For now, just pick server (could implement smarter merging)
                    let resolution =
                        ConflictResolution::server_choice(conflict.server_choice.clone());
                    reconciled_choices.push(conflict.server_choice.clone());
                    conflict.resolve(resolution);
                    conflicts_resolved.push(conflict);
                }
                ReconciliationStrategy::Interactive => {
                    // For non-interactive mode, pick server
                    let resolution =
                        ConflictResolution::server_choice(conflict.server_choice.clone());
                    reconciled_choices.push(conflict.server_choice.clone());
                    conflict.resolve(resolution);
                    conflicts_resolved.push(conflict);
                }
            }
        }

        // Add non-conflicting choices
        for (choice_id, client_choice) in &self.client_choices {
            if !self.server_choices.contains_key(choice_id) {
                // Client-only choice
                if self.reconciliation_strategy == ReconciliationStrategy::ClientWins {
                    reconciled_choices.push(client_choice.clone());
                }
            }
        }

        for (choice_id, server_choice) in &self.server_choices {
            if !self.client_choices.contains_key(choice_id) {
                // Server-only choice
                reconciled_choices.push(server_choice.clone());
            } else {
                // Both sides had this choice (already handled)
            }
        }

        Ok(ReconciliationResult::new(
            reconciled_choices,
            conflicts_resolved,
            choices_discarded,
        ))
    }

    /// Get a summary of the reconciliation
    pub fn get_reconciliation_summary(&self) -> ReconciliationSummary {
        let conflicts = self.detect_conflicts();

        ReconciliationSummary {
            total_client_choices: self.client_choices.len(),
            total_server_choices: self.server_choices.len(),
            conflicts_detected: conflicts.len(),
            conflicts_resolved: 0,
            choices_reconciled: 0,
            choices_discarded: 0,
        }
    }
}

// ============================================================================
// Choice Log (Audit Trail)
// ============================================================================

/// Entry in the choice log
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceLogEntry {
    pub entry_id: LogEntryId,
    pub choice_id: ChoiceId,
    pub entity_id: EntityId,
    pub player_id: PeerId,
    pub choice_snapshot: FreeWillChoice,
    pub validation_result: ValidationResult,
    pub timestamp: Timestamp,
}

impl ChoiceLogEntry {
    pub fn new(
        entry_id: LogEntryId,
        choice: &FreeWillChoice,
        validation_result: ValidationResult,
    ) -> Self {
        ChoiceLogEntry {
            entry_id,
            choice_id: choice.choice_id,
            entity_id: choice.entity_id,
            player_id: choice.player_id,
            choice_snapshot: choice.clone(),
            validation_result,
            timestamp: choice.timestamp,
        }
    }
}

/// Audit trail of all choices
#[derive(Debug, Clone)]
pub struct ChoiceLog {
    entries: Vec<ChoiceLogEntry>,
    next_entry_id: LogEntryId,
}

impl ChoiceLog {
    /// Create a new choice log
    pub fn new() -> Self {
        ChoiceLog {
            entries: Vec::new(),
            next_entry_id: 1,
        }
    }

    /// Log a choice and its validation result
    pub fn log_choice(&mut self, choice: &FreeWillChoice, result: &ValidationResult) {
        let entry = ChoiceLogEntry::new(self.next_entry_id, choice, result.clone());
        self.entries.push(entry);
        self.next_entry_id += 1;
    }

    /// Get history for an entity
    pub fn get_entity_history(&self, entity_id: EntityId) -> Vec<&ChoiceLogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.entity_id == entity_id)
            .collect()
    }

    /// Get history for a player
    pub fn get_player_history(&self, player_id: PeerId) -> Vec<&ChoiceLogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.player_id == player_id)
            .collect()
    }

    /// Get replay sequence for an entity within a time range
    pub fn get_replay_sequence(
        &self,
        entity_id: EntityId,
        start: Timestamp,
        end: Timestamp,
    ) -> Vec<FreeWillChoice> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.entity_id == entity_id && entry.timestamp >= start && entry.timestamp <= end
            })
            .map(|entry| entry.choice_snapshot.clone())
            .collect()
    }

    /// Get all entries
    pub fn get_all_entries(&self) -> &[ChoiceLogEntry] {
        &self.entries
    }
}

impl Default for ChoiceLog {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Statistics
// ============================================================================

/// Statistics about choices
#[derive(Debug, Clone)]
pub struct ChoiceStatistics {
    pub total_choices_submitted: u64,
    pub total_choices_validated: u64,
    pub total_choices_rejected: u64,
    pub total_conflicts_resolved: u64,
    pub choices_by_type: HashMap<FreeWillType, u64>,
    pub validation_time_avg: Float,
    pub reconciliation_time_avg: Float,
}

impl ChoiceStatistics {
    pub fn new() -> Self {
        ChoiceStatistics {
            total_choices_submitted: 0,
            total_choices_validated: 0,
            total_choices_rejected: 0,
            total_conflicts_resolved: 0,
            choices_by_type: HashMap::new(),
            validation_time_avg: 0.0,
            reconciliation_time_avg: 0.0,
        }
    }

    /// Record a submitted choice
    pub fn record_submitted(&mut self, choice_type: FreeWillType) {
        self.total_choices_submitted += 1;
        *self.choices_by_type.entry(choice_type).or_insert(0) += 1;
    }

    /// Record a validated choice
    pub fn record_validated(&mut self) {
        self.total_choices_validated += 1;
    }

    /// Record a rejected choice
    pub fn record_rejected(&mut self) {
        self.total_choices_rejected += 1;
    }

    /// Record a resolved conflict
    pub fn record_conflict_resolved(&mut self) {
        self.total_conflicts_resolved += 1;
    }
}

impl Default for ChoiceStatistics {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Main Replication System
// ============================================================================

/// Free Will Replication System
///
/// From COSMOLOGICAL-ARCHITECTURE.md: "Free Will choices are fundamental operations
/// that must be validated and replicated across the holographic system to ensure
/// consistency and integrity of evolutionary progress."
#[derive(Debug, Clone)]
pub struct FreeWillReplicationSystem {
    validator: FreeWillValidator,
    client_reconciliation: ClientReconciliation,
    choice_log: ChoiceLog,
    pending_choices: HashMap<ChoiceId, FreeWillChoice>,
    validated_choices: HashMap<ChoiceId, FreeWillChoice>,
    statistics: ChoiceStatistics,
    next_choice_id: ChoiceId,
}

impl FreeWillReplicationSystem {
    /// Create a new Free Will replication system
    pub fn new() -> Self {
        FreeWillReplicationSystem {
            validator: FreeWillValidator::new(),
            client_reconciliation: ClientReconciliation::new(ReconciliationStrategy::ServerWins),
            choice_log: ChoiceLog::new(),
            pending_choices: HashMap::new(),
            validated_choices: HashMap::new(),
            statistics: ChoiceStatistics::new(),
            next_choice_id: ChoiceId(1),
        }
    }

    /// Submit a choice for validation
    pub fn submit_choice(
        &mut self,
        choice: FreeWillChoice,
    ) -> Result<ValidationResult, FreeWillReplicationError> {
        // Check if choice already exists
        if self.validated_choices.contains_key(&choice.choice_id) {
            return Err(FreeWillReplicationError::ChoiceAlreadyExists(
                choice.choice_id,
            ));
        }

        // Move to pending
        self.pending_choices
            .insert(choice.choice_id, choice.clone());

        // Record in statistics
        self.statistics.record_submitted(choice.choice_type.clone());

        // Create a placeholder validation result (real validation requires entity state)
        let result = ValidationResult::success(choice.choice_id);
        Ok(result)
    }

    /// Validate a choice with entity state
    pub fn validate_choice(
        &mut self,
        choice: &FreeWillChoice,
        entity_state: &EntityState,
    ) -> Result<ValidationResult, FreeWillReplicationError> {
        // Check signature
        if !choice.validate() {
            self.statistics.record_rejected();
            return Err(FreeWillReplicationError::SignatureInvalid);
        }

        // Validate using validator
        let result = self.validator.validate_choice(choice, entity_state);

        // Log the choice
        self.choice_log.log_choice(choice, &result);

        // Update statistics
        if result.is_valid {
            self.statistics.record_validated();
        } else {
            self.statistics.record_rejected();
        }

        // Move from pending to validated if successful
        if result.is_valid {
            self.pending_choices.remove(&choice.choice_id);
            self.validated_choices
                .insert(choice.choice_id, choice.clone());
        }

        Ok(result)
    }

    /// Replicate a validated choice to other peers
    pub fn replicate_choice(
        &mut self,
        choice: &FreeWillChoice,
    ) -> Result<(), FreeWillReplicationError> {
        // Check if choice is validated
        if !choice.is_validated {
            return Err(FreeWillReplicationError::ChoiceValidationFailed(
                "Choice must be validated before replication".to_string(),
            ));
        }

        // Check if choice exists in validated choices
        if !self.validated_choices.contains_key(&choice.choice_id) {
            return Err(FreeWillReplicationError::ChoiceNotFound(choice.choice_id));
        }

        // In a real implementation, this would replicate to other peers
        // For now, we just log it as replicated
        Ok(())
    }

    /// Reconcile with server choices
    pub fn reconcile_with_server(
        &mut self,
        server_choices: Vec<FreeWillChoice>,
    ) -> Result<ReconciliationResult, FreeWillReplicationError> {
        // Clear client reconciliation and add client choices
        self.client_reconciliation = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        for choice in self.validated_choices.values() {
            self.client_reconciliation.add_client_choice(choice.clone());
        }

        // Add server choices
        for choice in server_choices {
            self.client_reconciliation.add_server_choice(choice);
        }

        // Perform reconciliation
        let result = self.client_reconciliation.reconcile()?;

        // Update statistics
        self.statistics.total_conflicts_resolved += result.conflicts_resolved.len() as u64;

        // Update validated choices
        self.validated_choices.clear();
        for choice in &result.reconciled_choices {
            self.validated_choices
                .insert(choice.choice_id, choice.clone());
        }

        Ok(result)
    }

    /// Get all choices for an entity
    pub fn get_entity_choices(&self, entity_id: EntityId) -> Vec<&FreeWillChoice> {
        self.validated_choices
            .values()
            .filter(|choice| choice.entity_id == entity_id)
            .collect()
    }

    /// Get all choices for a player
    pub fn get_player_choices(&self, player_id: PeerId) -> Vec<&FreeWillChoice> {
        self.validated_choices
            .values()
            .filter(|choice| choice.player_id == player_id)
            .collect()
    }

    /// Get choice history by ID
    pub fn get_choice_history(&self, choice_id: ChoiceId) -> Option<&FreeWillChoice> {
        self.validated_choices.get(&choice_id)
    }

    /// Get system statistics
    pub fn get_statistics(&self) -> ChoiceStatistics {
        self.statistics.clone()
    }

    /// Get the validator
    pub fn get_validator(&self) -> &FreeWillValidator {
        &self.validator
    }

    /// Get mutable validator
    pub fn get_validator_mut(&mut self) -> &mut FreeWillValidator {
        &mut self.validator
    }

    /// Create a new choice ID
    pub fn create_choice_id(&mut self) -> ChoiceId {
        let id = self.next_choice_id;
        self.next_choice_id += 1;
        id
    }
}

impl Default for FreeWillReplicationSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur in Free Will replication
#[derive(Debug, Clone, PartialEq)]
pub enum FreeWillReplicationError {
    InvalidChoice(String),
    ChoiceValidationFailed(String),
    ChoiceAlreadyExists(ChoiceId),
    ChoiceNotFound(ChoiceId),
    ReconciliationFailed(String),
    SignatureInvalid,
    SerializationError(String),
    DeserializationError(String),
}

impl Display for FreeWillReplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FreeWillReplicationError::InvalidChoice(msg) => write!(f, "Invalid choice: {}", msg),
            FreeWillReplicationError::ChoiceValidationFailed(msg) => {
                write!(f, "Choice validation failed: {}", msg)
            }
            FreeWillReplicationError::ChoiceAlreadyExists(id) => {
                write!(f, "Choice already exists: {}", id)
            }
            FreeWillReplicationError::ChoiceNotFound(id) => write!(f, "Choice not found: {}", id),
            FreeWillReplicationError::ReconciliationFailed(msg) => {
                write!(f, "Reconciliation failed: {}", msg)
            }
            FreeWillReplicationError::SignatureInvalid => write!(f, "Signature is invalid"),
            FreeWillReplicationError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            FreeWillReplicationError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for FreeWillReplicationError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test choice
    fn create_test_choice(
        choice_id: ChoiceId,
        entity_id: EntityId,
        player_id: PeerId,
        choice_type: FreeWillType,
        choice_data: ChoiceData,
    ) -> FreeWillChoice {
        let mut choice =
            FreeWillChoice::new(choice_id, entity_id, player_id, choice_type, choice_data);
        choice.sign(b"test_key");
        choice.is_validated = true;
        choice
    }

    // Helper function to create a test entity state
    fn create_test_entity_state(entity_id: EntityId) -> EntityState {
        EntityState::new(entity_id, Density::Third)
    }

    // --------------------------------------------------------------------------
    // FreeWillChoice Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_create_free_will_choice() {
        let choice_data = ChoiceData::ArchetypeData { archetype_id: 22 };
        let choice = FreeWillChoice::new(1, 100, 200, FreeWillType::ArchetypeChoice, choice_data);

        assert_eq!(choice.choice_id, 1);
        assert_eq!(choice.entity_id, 100);
        assert_eq!(choice.player_id, 200);
        assert_eq!(choice.choice_type, FreeWillType::ArchetypeChoice);
        assert!(!choice.is_validated);
    }

    #[test]
    fn test_choice_validate() {
        let mut choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        assert!(choice.validate());
    }

    #[test]
    fn test_choice_validate_without_signature() {
        let choice = FreeWillChoice::new(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        assert!(!choice.validate());
    }

    #[test]
    fn test_choice_compatibility_same() {
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = choice1.clone();
        assert!(choice1.is_compatible_with(&choice2));
    }

    #[test]
    fn test_choice_compatibility_different_entity() {
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            101,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        assert!(choice1.is_compatible_with(&choice2));
    }

    #[test]
    fn test_choice_compatibility_conflict() {
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );
        assert!(!choice1.is_compatible_with(&choice2));
    }

    #[test]
    fn test_choice_serialize_deserialize() {
        let original = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let bytes = original.serialize();
        let deserialized = FreeWillChoice::deserialize(&bytes).unwrap();

        assert_eq!(original.choice_id, deserialized.choice_id);
        assert_eq!(original.entity_id, deserialized.entity_id);
        assert_eq!(original.player_id, deserialized.player_id);
        assert_eq!(original.choice_type, deserialized.choice_type);
        assert_eq!(original.is_validated, deserialized.is_validated);
    }

    #[test]
    fn test_choice_serialize_deserialize_polarity() {
        let original = create_test_choice(
            1,
            100,
            200,
            FreeWillType::PolarityChoice,
            ChoiceData::PolarityData {
                polarity: InternalPolarity::STO,
            },
        );
        let bytes = original.serialize();
        let deserialized = FreeWillChoice::deserialize(&bytes).unwrap();

        assert_eq!(original.choice_type, deserialized.choice_type);
        match &deserialized.choice_data {
            ChoiceData::PolarityData { polarity } => {
                assert_eq!(*polarity, InternalPolarity::STO);
            }
            _ => panic!("Wrong choice data type"),
        }
    }

    #[test]
    fn test_choice_serialize_deserialize_density() {
        let original = create_test_choice(
            1,
            100,
            200,
            FreeWillType::DensityTransition,
            ChoiceData::DensityData {
                target_density: Density::Fourth,
            },
        );
        let bytes = original.serialize();
        let deserialized = FreeWillChoice::deserialize(&bytes).unwrap();

        match &deserialized.choice_data {
            ChoiceData::DensityData { target_density } => {
                assert_eq!(target_density.value(), Density::Fourth.value());
            }
            _ => panic!("Wrong choice data type"),
        }
    }

    #[test]
    fn test_choice_serialize_deserialize_resonance() {
        let pattern = ResonancePattern::new(440.0, 0.5, 1.0);
        let original = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ResonanceAlignment,
            ChoiceData::ResonanceData {
                target_pattern: pattern,
            },
        );
        let bytes = original.serialize();
        let deserialized = FreeWillChoice::deserialize(&bytes).unwrap();

        match &deserialized.choice_data {
            ChoiceData::ResonanceData { target_pattern } => {
                assert_eq!(target_pattern.frequency, 440.0);
                assert_eq!(target_pattern.phase, 0.5);
                assert_eq!(target_pattern.amplitude, 1.0);
            }
            _ => panic!("Wrong choice data type"),
        }
    }

    // --------------------------------------------------------------------------
    // FreeWillType Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_free_will_type_display() {
        assert_eq!(
            format!("{}", FreeWillType::ArchetypeChoice),
            "ArchetypeChoice"
        );
        assert_eq!(
            format!("{}", FreeWillType::PolarityChoice),
            "PolarityChoice"
        );
        assert_eq!(
            format!("{}", FreeWillType::DensityTransition),
            "DensityTransition"
        );
    }

    // --------------------------------------------------------------------------
    // ChoiceData Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_choice_data_archetype() {
        let data = ChoiceData::ArchetypeData { archetype_id: 22 };
        assert_eq!(matches!(data, ChoiceData::ArchetypeData { .. }), true);
    }

    #[test]
    fn test_choice_data_polarity() {
        let data = ChoiceData::PolarityData {
            polarity: InternalPolarity::STO,
        };
        assert_eq!(matches!(data, ChoiceData::PolarityData { .. }), true);
    }

    // --------------------------------------------------------------------------
    // Density Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_density_value() {
        assert_eq!(Density::First.value(), 1);
        assert_eq!(Density::Third.value(), 3);
        assert_eq!(Density::Eighth.value(), 8);
    }

    #[test]
    fn test_density_can_transition_to() {
        assert!(can_transition_density(Density::Third, Density::Fourth));
        assert!(can_transition_density(Density::Third, Density::Fifth));
        assert!(!can_transition_density(Density::Third, Density::Sixth));
        assert!(!can_transition_density(Density::Third, Density::Third));
    }

    // --------------------------------------------------------------------------
    // ResonancePattern Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_resonance_pattern_new() {
        let pattern = ResonancePattern::new(440.0, 0.5, 1.0);
        assert_eq!(pattern.frequency, 440.0);
        assert_eq!(pattern.phase, 0.5);
        assert_eq!(pattern.amplitude, 1.0);
    }

    #[test]
    fn test_resonance_pattern_compatibility() {
        let pattern1 = ResonancePattern::new(440.0, 0.5, 1.0);
        let pattern2 = ResonancePattern::new(440.0, 0.5, 1.0);
        assert_eq!(pattern1.compatibility_with(&pattern2), 1.0);
    }

    #[test]
    fn test_resonance_pattern_compatibility_different() {
        let pattern1 = ResonancePattern::new(440.0, 0.5, 1.0);
        let pattern2 = ResonancePattern::new(880.0, 0.0, 0.5);
        let compat = pattern1.compatibility_with(&pattern2);
        assert!(compat < 1.0);
        assert!(compat > 0.0);
    }

    // --------------------------------------------------------------------------
    // Polarity Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_polarity_eq() {
        assert_eq!(InternalPolarity::STS, InternalPolarity::STS);
        assert_eq!(InternalPolarity::STO, InternalPolarity::STO);
        assert_ne!(InternalPolarity::STS, InternalPolarity::STO);
    }

    // --------------------------------------------------------------------------
    // ValidationRule Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_validation_rule_new() {
        let rule = ValidationRule::new(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::AlwaysTrue,
            "Test rule".to_string(),
        );
        assert_eq!(rule.rule_id, 1);
        assert!(!rule.is_required);
    }

    #[test]
    fn test_validation_rule_required() {
        let rule = ValidationRule::required(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::AlwaysTrue,
            "Test rule".to_string(),
        );
        assert!(rule.is_required);
    }

    #[test]
    fn test_validation_rule_evaluate_always_true() {
        let rule = ValidationRule::new(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::AlwaysTrue,
            "Test rule".to_string(),
        );
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);
        assert!(rule.evaluate(&choice, &entity_state));
    }

    #[test]
    fn test_validation_rule_evaluate_never_true() {
        let rule = ValidationRule::new(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::NeverTrue,
            "Test rule".to_string(),
        );
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);
        assert!(!rule.evaluate(&choice, &entity_state));
    }

    #[test]
    fn test_validation_rule_evaluate_choice_type() {
        let rule = ValidationRule::new(
            1,
            RuleType::DensityRequirement {
                min_density: Density::First,
                max_density: Density::Eighth,
            },
            ValidationCondition::ChoiceTypeEquals(FreeWillType::ArchetypeChoice),
            "Test rule".to_string(),
        );
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);
        assert!(rule.evaluate(&choice, &entity_state));

        let choice2 = create_test_choice(
            2,
            100,
            200,
            FreeWillType::PolarityChoice,
            ChoiceData::PolarityData {
                polarity: InternalPolarity::STO,
            },
        );
        assert!(!rule.evaluate(&choice2, &entity_state));
    }

    #[test]
    fn test_validation_rule_evaluate_density_range() {
        let rule = ValidationRule::new(
            1,
            RuleType::DensityRequirement {
                min_density: Density::Third,
                max_density: Density::Fifth,
            },
            ValidationCondition::DensityInRange(Density::Third, Density::Fifth),
            "Test rule".to_string(),
        );
        let entity_state = EntityState::new(100, Density::Fourth);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::DensityTransition,
            ChoiceData::DensityData {
                target_density: Density::Fifth,
            },
        );
        assert!(rule.evaluate(&choice, &entity_state));

        let entity_state2 = EntityState::new(100, Density::Second);
        assert!(!rule.evaluate(&choice, &entity_state2));
    }

    // --------------------------------------------------------------------------
    // ValidationResult Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_validation_result_with_warning() {
        let result = ValidationResult::success(1).add_warning(ValidationWarning::new(
            "WARN".to_string(),
            "Warning".to_string(),
        ));
        assert!(result.is_valid);
        assert!(result.has_warnings());
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_failure() {
        let errors = vec![ValidationError::new(
            "TEST_ERROR".to_string(),
            "Test error message".to_string(),
        )];
        let result = ValidationResult::failure(1, errors);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_validation_result_has_errors() {
        let result = ValidationResult::success(1);
        assert!(!result.has_errors());

        let errors = vec![ValidationError::new(
            "TEST_ERROR".to_string(),
            "Test error".to_string(),
        )];
        let result = ValidationResult::failure(1, errors);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_result_get_summary() {
        let result = ValidationResult::success(1);
        assert_eq!(result.get_summary(), "Valid");

        let result = ValidationResult::success(1).with_warning(ValidationWarning::new(
            "WARN".to_string(),
            "Warning".to_string(),
        ));
        assert_eq!(result.get_summary(), "Valid with 1 warning(s)");

        let errors = vec![ValidationError::new("ERR".to_string(), "Error".to_string())];
        let result = ValidationResult::failure(1, errors);
        assert_eq!(result.get_summary(), "Invalid: 1 error(s)");
    }

    // --------------------------------------------------------------------------
    // FreeWillValidator Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_validator_new() {
        let validator = FreeWillValidator::new();
        assert!(!validator.validation_rules.is_empty());
    }

    #[test]
    fn test_validator_validate_choice_success() {
        let validator = FreeWillValidator::new();
        let mut choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        choice.sign(b"test_key");
        let entity_state = create_test_entity_state(100);

        let result = validator.validate_choice(&choice, &entity_state);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validator_validate_choice_invalid_signature() {
        let validator = FreeWillValidator::new();
        let choice = FreeWillChoice::new(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);

        let result = validator.validate_choice(&choice, &entity_state);
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.error_code == "SIGNATURE_INVALID"));
    }

    #[test]
    fn test_validator_validate_choice_entity_mismatch() {
        let validator = FreeWillValidator::new();
        let mut choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        choice.sign(b"test_key");
        let entity_state = create_test_entity_state(999); // Different entity ID

        let result = validator.validate_choice(&choice, &entity_state);
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.error_code == "ENTITY_MISMATCH"));
    }

    #[test]
    fn test_validator_add_validation_rule() {
        let mut validator = FreeWillValidator::new();
        let initial_count = validator.validation_rules.len();

        let new_rule = ValidationRule::new(
            100,
            RuleType::ArchetypeConstraint {
                allowed_archetypes: vec![22, 23],
            },
            ValidationCondition::AlwaysTrue,
            "Test rule".to_string(),
        );
        validator.add_validation_rule(new_rule);

        assert_eq!(validator.validation_rules.len(), initial_count + 1);
    }

    #[test]
    fn test_validator_remove_validation_rule() {
        let mut validator = FreeWillValidator::new();
        let initial_count = validator.validation_rules.len();

        validator.remove_validation_rule(1);
        assert_eq!(validator.validation_rules.len(), initial_count - 1);
    }

    #[test]
    fn test_validator_check_choice_compatibility() {
        let validator = FreeWillValidator::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            101,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );

        let compat = validator.check_choice_compatibility(&choice1, &choice2);
        assert_eq!(compat, Compatibility::FullyCompatible);
    }

    // --------------------------------------------------------------------------
    // ClientReconciliation Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_client_reconciliation_new() {
        let recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        assert_eq!(
            recon.reconciliation_strategy,
            ReconciliationStrategy::ServerWins
        );
    }

    #[test]
    fn test_client_reconciliation_add_choices() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );

        recon.add_client_choice(choice.clone());
        assert_eq!(recon.client_choices.len(), 1);

        recon.add_server_choice(choice);
        assert_eq!(recon.server_choices.len(), 1);
    }

    #[test]
    fn test_client_reconciliation_detect_conflicts() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        recon.add_client_choice(client_choice);
        recon.add_server_choice(server_choice);

        let conflicts = recon.detect_conflicts();
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn test_client_reconciliation_server_wins() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        recon.add_client_choice(client_choice);
        recon.add_server_choice(server_choice);

        let result = recon.reconcile().unwrap();
        assert_eq!(result.conflicts_resolved.len(), 1);
        assert_eq!(result.reconciled_choices.len(), 1);
        assert_eq!(
            result.reconciled_choices[0].player_id,
            201 // Server's player ID
        );
    }

    #[test]
    fn test_client_reconciliation_client_wins() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ClientWins);
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        recon.add_client_choice(client_choice);
        recon.add_server_choice(server_choice);

        let result = recon.reconcile().unwrap();
        assert_eq!(result.reconciled_choices[0].player_id, 200); // Client's player ID
    }

    #[test]
    fn test_client_reconciliation_no_conflicts() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            2,
            101,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        recon.add_client_choice(client_choice);
        recon.add_server_choice(server_choice);

        let conflicts = recon.detect_conflicts();
        assert_eq!(conflicts.len(), 0);
    }

    #[test]
    fn test_client_reconciliation_get_summary() {
        let mut recon = ClientReconciliation::new(ReconciliationStrategy::ServerWins);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );

        recon.add_client_choice(choice.clone());
        recon.add_server_choice(choice);

        let summary = recon.get_reconciliation_summary();
        assert_eq!(summary.total_client_choices, 1);
        assert_eq!(summary.total_server_choices, 1);
        assert_eq!(summary.conflicts_detected, 0);
    }

    // --------------------------------------------------------------------------
    // ChoiceConflict Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_choice_conflict_new() {
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        let conflict = ChoiceConflict::new(
            0,
            client_choice,
            server_choice,
            ConflictType::ConcurrentChoice,
        );

        assert_eq!(conflict.conflict_id, 0);
        assert!(conflict.resolution.is_none());
    }

    #[test]
    fn test_choice_conflict_resolve() {
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        let mut conflict = ChoiceConflict::new(
            0,
            client_choice,
            server_choice,
            ConflictType::ConcurrentChoice,
        );

        let resolution = ConflictResolution::server_choice(server_choice.clone());
        conflict.resolve(resolution);

        assert!(conflict.resolution.is_some());
    }

    // --------------------------------------------------------------------------
    // ChoiceLog Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_choice_log_new() {
        let log = ChoiceLog::new();
        assert!(log.entries.is_empty());
        assert_eq!(log.next_entry_id, 1);
    }

    #[test]
    fn test_choice_log_log_choice() {
        let mut log = ChoiceLog::new();
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let result = ValidationResult::success(1);

        log.log_choice(&choice, &result);

        assert_eq!(log.entries.len(), 1);
        assert_eq!(log.next_entry_id, 2);
    }

    #[test]
    fn test_choice_log_get_entity_history() {
        let mut log = ChoiceLog::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            101,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );
        let result = ValidationResult::success(1);

        log.log_choice(&choice1, &result);
        log.log_choice(&choice2, &result);

        let history = log.get_entity_history(100);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].entity_id, 100);
    }

    #[test]
    fn test_choice_log_get_player_history() {
        let mut log = ChoiceLog::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            101,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );
        let result = ValidationResult::success(1);

        log.log_choice(&choice1, &result);
        log.log_choice(&choice2, &result);

        let history = log.get_player_history(200);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].player_id, 200);
    }

    #[test]
    fn test_choice_log_get_replay_sequence() {
        let mut log = ChoiceLog::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let result = ValidationResult::success(1);

        log.log_choice(&choice1, &result);

        let sequence = log.get_replay_sequence(100, 0, u64::MAX);
        assert_eq!(sequence.len(), 1);
    }

    // --------------------------------------------------------------------------
    // ChoiceStatistics Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_choice_statistics_new() {
        let stats = ChoiceStatistics::new();
        assert_eq!(stats.total_choices_submitted, 0);
        assert_eq!(stats.total_choices_validated, 0);
        assert_eq!(stats.total_choices_rejected, 0);
    }

    #[test]
    fn test_choice_statistics_record_submitted() {
        let mut stats = ChoiceStatistics::new();
        stats.record_submitted(FreeWillType::ArchetypeChoice);

        assert_eq!(stats.total_choices_submitted, 1);
        assert_eq!(
            *stats
                .choices_by_type
                .get(&FreeWillType::ArchetypeChoice)
                .unwrap(),
            1
        );
    }

    #[test]
    fn test_choice_statistics_record_validated() {
        let mut stats = ChoiceStatistics::new();
        stats.record_validated();

        assert_eq!(stats.total_choices_validated, 1);
    }

    #[test]
    fn test_choice_statistics_record_rejected() {
        let mut stats = ChoiceStatistics::new();
        stats.record_rejected();

        assert_eq!(stats.total_choices_rejected, 1);
    }

    #[test]
    fn test_choice_statistics_record_conflict_resolved() {
        let mut stats = ChoiceStatistics::new();
        stats.record_conflict_resolved();

        assert_eq!(stats.total_conflicts_resolved, 1);
    }

    // --------------------------------------------------------------------------
    // FreeWillReplicationSystem Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_replication_system_new() {
        let system = FreeWillReplicationSystem::new();
        assert!(!system.validated_choices.is_empty());
        assert_eq!(system.next_choice_id, 1);
    }

    #[test]
    fn test_replication_system_submit_choice() {
        let mut system = FreeWillReplicationSystem::new();
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );

        let result = system.submit_choice(choice).unwrap();
        assert!(result.is_valid);
        assert_eq!(system.statistics.total_choices_submitted, 1);
    }

    #[test]
    fn test_replication_system_submit_choice_duplicate() {
        let mut system = FreeWillReplicationSystem::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );

        system.submit_choice(choice1.clone()).unwrap();

        let result = system.submit_choice(choice1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FreeWillReplicationError::ChoiceAlreadyExists(_)
        ));
    }

    #[test]
    fn test_replication_system_validate_choice() {
        let mut system = FreeWillReplicationSystem::new();
        let mut choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        choice.sign(b"test_key");
        let entity_state = create_test_entity_state(100);

        let result = system.validate_choice(&choice, &entity_state).unwrap();
        assert!(result.is_valid);
        assert_eq!(system.statistics.total_choices_validated, 1);
    }

    #[test]
    fn test_replication_system_validate_choice_invalid_signature() {
        let mut system = FreeWillReplicationSystem::new();
        let choice = FreeWillChoice::new(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);

        let result = system.validate_choice(&choice, &entity_state);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FreeWillReplicationError::SignatureInvalid
        ));
    }

    #[test]
    fn test_replication_system_replicate_choice() {
        let mut system = FreeWillReplicationSystem::new();
        let mut choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        choice.is_validated = true;
        system
            .validated_choices
            .insert(choice.choice_id, choice.clone());

        let result = system.replicate_choice(&choice);
        assert!(result.is_ok());
    }

    #[test]
    fn test_replication_system_replicate_choice_not_validated() {
        let mut system = FreeWillReplicationSystem::new();
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        choice.is_validated = false;

        let result = system.replicate_choice(&choice);
        assert!(result.is_err());
    }

    #[test]
    fn test_replication_system_reconcile_with_server() {
        let mut system = FreeWillReplicationSystem::new();
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        system.validated_choices.insert(1, client_choice);

        let server_choices = vec![create_test_choice(
            2,
            101,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        )];

        let result = system.reconcile_with_server(server_choices).unwrap();
        assert_eq!(result.reconciled_choices.len(), 2);
    }

    #[test]
    fn test_replication_system_get_entity_choices() {
        let mut system = FreeWillReplicationSystem::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            100,
            200,
            FreeWillType::PolarityChoice,
            ChoiceData::PolarityData {
                polarity: InternalPolarity::STO,
            },
        );
        let choice3 = create_test_choice(
            3,
            101,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        system.validated_choices.insert(1, choice1);
        system.validated_choices.insert(2, choice2);
        system.validated_choices.insert(3, choice3);

        let choices = system.get_entity_choices(100);
        assert_eq!(choices.len(), 2);
    }

    #[test]
    fn test_replication_system_get_player_choices() {
        let mut system = FreeWillReplicationSystem::new();
        let choice1 = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let choice2 = create_test_choice(
            2,
            101,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );
        let choice3 = create_test_choice(
            3,
            102,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 24 },
        );

        system.validated_choices.insert(1, choice1);
        system.validated_choices.insert(2, choice2);
        system.validated_choices.insert(3, choice3);

        let choices = system.get_player_choices(200);
        assert_eq!(choices.len(), 2);
    }

    #[test]
    fn test_replication_system_get_choice_history() {
        let mut system = FreeWillReplicationSystem::new();
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        system.validated_choices.insert(1, choice.clone());

        let history = system.get_choice_history(1);
        assert!(history.is_some());
        assert_eq!(history.unwrap().choice_id, 1);
    }

    #[test]
    fn test_replication_system_get_choice_history_not_found() {
        let system = FreeWillReplicationSystem::new();
        let history = system.get_choice_history(999);
        assert!(history.is_none());
    }

    #[test]
    fn test_replication_system_get_statistics() {
        let mut system = FreeWillReplicationSystem::new();
        system
            .statistics
            .record_submitted(FreeWillType::ArchetypeChoice);
        system.statistics.record_validated();

        let stats = system.get_statistics();
        assert_eq!(stats.total_choices_submitted, 1);
        assert_eq!(stats.total_choices_validated, 1);
    }

    #[test]
    fn test_replication_system_create_choice_id() {
        let mut system = FreeWillReplicationSystem::new();
        let id1 = system.create_choice_id();
        let id2 = system.create_choice_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_replication_system_get_validator() {
        let system = FreeWillReplicationSystem::new();
        let validator = system.get_validator();
        assert!(!validator.validation_rules.is_empty());
    }

    // --------------------------------------------------------------------------
    // FreeWillReplicationError Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_error_display() {
        let err = FreeWillReplicationError::InvalidChoice("test".to_string());
        assert_eq!(format!("{}", err), "Invalid choice: test");

        let err = FreeWillReplicationError::ChoiceAlreadyExists(123);
        assert_eq!(format!("{}", err), "Choice already exists: 123");
    }

    #[test]
    fn test_error_choice_not_found() {
        let err = FreeWillReplicationError::ChoiceNotFound(999);
        assert_eq!(format!("{}", err), "Choice not found: 999");
    }

    #[test]
    fn test_validation_result_get_summary() {
        let result = ValidationResult::success(1);
        assert_eq!(result.get_summary(), "Valid");

        let result = ValidationResult::success(1).add_warning(ValidationWarning::new(
            "WARN".to_string(),
            "Warning".to_string(),
        ));
        assert_eq!(result.get_summary(), "Valid with 1 warning(s)");

        let errors = vec![ValidationError::new("ERR".to_string(), "Error".to_string())];
        let result = ValidationResult::failure(1, errors);
        assert_eq!(result.get_summary(), "Invalid: 1 error(s)");
    }

    // --------------------------------------------------------------------------
    // ValidationCondition Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_validation_condition_evaluate_choice_type() {
        let cond = ValidationCondition::ChoiceTypeEquals(FreeWillType::ArchetypeChoice);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);

        assert!(cond.evaluate(&choice, &entity_state));
    }

    #[test]
    fn test_validation_condition_evaluate_density_range() {
        let cond = ValidationCondition::DensityInRange(Density::Third, Density::Fifth);
        let entity_state = EntityState::new(100, Density::Fourth);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::DensityTransition,
            ChoiceData::DensityData {
                target_density: Density::Fifth,
            },
        );

        assert!(cond.evaluate(&choice, &entity_state));
    }

    #[test]
    fn test_validation_condition_evaluate_resonance_threshold() {
        let cond = ValidationCondition::ResonanceAboveThreshold(0.5);
        let mut entity_state = EntityState::new(100, Density::Third);
        entity_state.resonance_pattern = ResonancePattern::new(1.0, 0.0, 1.0);
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ResonanceAlignment,
            ChoiceData::ResonanceData {
                target_pattern: ResonancePattern::new(1.0, 0.0, 1.0),
            },
        );

        assert!(cond.evaluate(&choice, &entity_state));
    }

    // --------------------------------------------------------------------------
    // ConflictResolution Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_conflict_resolution_server_choice() {
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let resolution = ConflictResolution::server_choice(choice.clone());

        assert_eq!(resolution.resolution_type, ResolutionType::ServerChosen);
        assert!(resolution.chosen_choice.is_some());
        assert_eq!(resolution.chosen_choice.as_ref().unwrap().player_id, 200);
    }

    #[test]
    fn test_conflict_resolution_client_choice() {
        let choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let resolution = ConflictResolution::client_choice(choice.clone());

        assert_eq!(resolution.resolution_type, ResolutionType::ClientChosen);
    }

    #[test]
    fn test_conflict_resolution_both_discarded() {
        let resolution = ConflictResolution::both_discarded();

        assert_eq!(resolution.resolution_type, ResolutionType::BothDiscarded);
        assert!(resolution.chosen_choice.is_none());
    }

    // --------------------------------------------------------------------------
    // EntityState Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_entity_state_new() {
        let state = EntityState::new(100, Density::Third);
        assert_eq!(state.entity_id, 100);
        assert_eq!(state.current_density, Density::Third);
        assert_eq!(state.current_polarity, InternalPolarity::Unpolarized);
    }

    #[test]
    fn test_entity_state_with_polarity() {
        let state = EntityState::new(100, Density::Third).with_polarity(InternalPolarity::STO);
        assert_eq!(state.current_polarity, InternalPolarity::STO);
    }

    #[test]
    fn test_entity_state_with_resonance() {
        let pattern = ResonancePattern::new(440.0, 0.5, 1.0);
        let state = EntityState::new(100, Density::Third).with_resonance(pattern);
        assert_eq!(state.resonance_pattern.frequency, 440.0);
    }

    // --------------------------------------------------------------------------
    // Additional Integration Tests
    // --------------------------------------------------------------------------

    #[test]
    fn test_full_choice_workflow() {
        let mut system = FreeWillReplicationSystem::new();

        // Create and submit a choice
        let choice_id = system.create_choice_id();
        let mut choice = FreeWillChoice::new(
            choice_id,
            100,
            200,
            FreeWillType::PolarityChoice,
            ChoiceData::PolarityData {
                polarity: InternalPolarity::STO,
            },
        );
        choice.sign(b"test_key");

        system.submit_choice(choice.clone()).unwrap();

        // Validate the choice
        let entity_state = create_test_entity_state(100);
        let result = system.validate_choice(&choice, &entity_state).unwrap();
        assert!(result.is_valid);

        // Check that choice is in validated choices
        assert!(system.validated_choices.contains_key(&choice_id));

        // Replicate the choice
        choice.is_validated = true;
        let replicate_result = system.replicate_choice(&choice);
        assert!(replicate_result.is_ok());

        // Get statistics
        let stats = system.get_statistics();
        assert_eq!(stats.total_choices_submitted, 1);
        assert_eq!(stats.total_choices_validated, 1);
    }

    #[test]
    fn test_multiple_choice_types() {
        let mut system = FreeWillReplicationSystem::new();
        let entity_state = create_test_entity_state(100);

        // Create choices of different types
        let types = vec![
            (
                FreeWillType::ArchetypeChoice,
                ChoiceData::ArchetypeData { archetype_id: 22 },
            ),
            (
                FreeWillType::PolarityChoice,
                ChoiceData::PolarityData {
                    polarity: InternalPolarity::STO,
                },
            ),
            (
                FreeWillType::DensityTransition,
                ChoiceData::DensityData {
                    target_density: Density::Fourth,
                },
            ),
        ];

        for (choice_type, choice_data) in types {
            let choice_id = system.create_choice_id();
            let mut choice = FreeWillChoice::new(choice_id, 100, 200, choice_type, choice_data);
            choice.sign(b"test_key");

            system.submit_choice(choice.clone()).unwrap();
            system.validate_choice(&choice, &entity_state).unwrap();
        }

        let stats = system.get_statistics();
        assert_eq!(stats.total_choices_submitted, 3);
        assert_eq!(stats.total_choices_validated, 3);
    }

    #[test]
    fn test_reconciliation_with_conflicts() {
        let mut system = FreeWillReplicationSystem::new();

        // Add client choices
        let client_choice = create_test_choice(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        system.validated_choices.insert(1, client_choice);

        // Add server choices with conflict (same ID, different data)
        let server_choice = create_test_choice(
            1,
            100,
            201,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 23 },
        );

        let result = system.reconcile_with_server(vec![server_choice]).unwrap();

        assert_eq!(result.conflicts_resolved.len(), 1);
        assert_eq!(result.reconciled_choices.len(), 1);
        // Server wins by default
        assert_eq!(result.reconciled_choices[0].player_id, 201);
    }

    #[test]
    fn test_choice_log_audit_trail() {
        let mut log = ChoiceLog::new();

        // Log multiple choices
        for i in 1..=5 {
            let choice = create_test_choice(
                i,
                100 + i,
                200,
                FreeWillType::ArchetypeChoice,
                ChoiceData::ArchetypeData {
                    archetype_id: 22 + i,
                },
            );
            let result = ValidationResult::success(i);
            log.log_choice(&choice, &result);
        }

        assert_eq!(log.entries.len(), 5);

        // Get entity history
        let history = log.get_entity_history(102);
        assert_eq!(history.len(), 1);

        // Get player history
        let player_history = log.get_player_history(200);
        assert_eq!(player_history.len(), 5);

        // Get replay sequence
        let replay = log.get_replay_sequence(102, 0, u64::MAX);
        assert_eq!(replay.len(), 1);
    }

    #[test]
    fn test_error_handling_invalid_choice() {
        let mut system = FreeWillReplicationSystem::new();

        // Try to validate a choice without signature
        let choice = FreeWillChoice::new(
            1,
            100,
            200,
            FreeWillType::ArchetypeChoice,
            ChoiceData::ArchetypeData { archetype_id: 22 },
        );
        let entity_state = create_test_entity_state(100);

        let result = system.validate_choice(&choice, &entity_state);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FreeWillReplicationError::SignatureInvalid
        ));
    }

    #[test]
    fn test_statistics_by_type() {
        let mut stats = ChoiceStatistics::new();

        stats.record_submitted(FreeWillType::ArchetypeChoice);
        stats.record_submitted(FreeWillType::PolarityChoice);
        stats.record_submitted(FreeWillType::ArchetypeChoice);

        assert_eq!(stats.total_choices_submitted, 3);
        assert_eq!(
            *stats
                .choices_by_type
                .get(&FreeWillType::ArchetypeChoice)
                .unwrap(),
            2
        );
        assert_eq!(
            *stats
                .choices_by_type
                .get(&FreeWillType::PolarityChoice)
                .unwrap(),
            1
        );
    }
}
