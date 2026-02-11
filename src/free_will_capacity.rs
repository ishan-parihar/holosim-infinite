// Free Will Capacity
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Free Will is the first primal distortion"
// "Free Will is the capacity to choose"
//
// This module implements:
// 1. Free Will capacity measurement
// 2. Choice mechanisms
// 3. Choice direction tracking

use crate::types::Float;

/// Free Will Capacity
///
/// Measures the capacity of an entity to exercise free will.
#[derive(Debug, Clone, PartialEq)]
pub struct FreeWillCapacity {
    /// Capacity level (0.0 to 1.0)
    pub capacity: Float,

    /// Choice history
    pub choice_history: Vec<Choice>,
}

impl Default for FreeWillCapacity {
    fn default() -> Self {
        Self {
            capacity: 0.5,
            choice_history: Vec::new(),
        }
    }
}

/// Choice
///
/// Represents a choice made by an entity.
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    /// Choice ID
    pub choice_id: u64,

    /// Choice direction
    pub direction: ChoiceDirection,

    /// Choice intensity (0.0 to 1.0)
    pub intensity: Float,

    /// Chosen intensity (0.0 to 1.0)
    pub chosen_intensity: Float,

    /// Chosen archetype
    pub chosen_archetype: Option<u8>,

    /// Timestamp
    pub timestamp: u64,
}

impl Choice {
    /// Creates a new choice with default values
    pub fn new() -> Self {
        Choice {
            choice_id: 0,
            direction: ChoiceDirection::Neutral,
            intensity: 0.5,
            chosen_intensity: 0.5,
            chosen_archetype: None,
            timestamp: 0,
        }
    }
}

/// Choice Direction
///
/// The direction of the choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceDirection {
    /// Positive polarity (Service-to-Others)
    Positive,

    /// Service-to-Others
    STO,

    /// Negative polarity (Service-to-Self)
    Negative,

    /// Service-to-Self
    STS,

    /// Neutral
    Neutral,
}
