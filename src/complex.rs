//! Complex types for the Holonic Realms simulation
//!
//! This module defines complex data structures used throughout the simulation,
//! including experience storage, choice mechanisms, and cycle states.

use crate::types::Float;

/// Experience - a unit of experience stored by an entity
#[derive(Debug, Clone)]
pub struct Experience {
    pub intensity: Float,
    pub timestamp: u64,
    pub polarized: bool,
}

impl Experience {
    pub fn new(intensity: Float, timestamp: u64, polarized: bool) -> Self {
        Experience {
            intensity,
            timestamp,
            polarized,
        }
    }
}

/// Choice - a choice made by an entity
#[derive(Debug, Clone)]
pub struct Choice {
    pub choice_type: ChoiceType,
    pub intensity: Float,
    pub timestamp: u64,
    pub chosen_archetype: Option<u8>,
    pub chosen_intensity: Float,
}

/// Choice type - the type of choice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceType {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

impl Choice {
    pub fn new(choice_type: ChoiceType, intensity: Float, timestamp: u64) -> Self {
        Choice {
            choice_type,
            intensity,
            timestamp,
            chosen_archetype: None,
            chosen_intensity: intensity,
        }
    }
}

/// Lesser Cycle State - state within a lesser cycle
#[derive(Debug, Clone)]
pub struct LesserCycleState {
    pub cycle_number: u32,
    pub position_in_cycle: Float,
    pub accumulated_experience: Float,
    pub microcosmic_tension: Float,
    pub experience_output: Float,
}

impl LesserCycleState {
    pub fn new(cycle_number: u32, position_in_cycle: Float) -> Self {
        LesserCycleState {
            cycle_number,
            position_in_cycle,
            accumulated_experience: 0.0,
            microcosmic_tension: 0.0,
            experience_output: 0.0,
        }
    }

    pub fn advance(&mut self, delta: Float) {
        self.position_in_cycle = (self.position_in_cycle + delta).min(1.0);
        self.accumulated_experience += delta;
        self.experience_output += delta;
    }

    pub fn is_complete(&self) -> bool {
        self.position_in_cycle >= 1.0
    }
}

/// Complex - a complex of archetypes
#[derive(Debug, Clone)]
pub struct Complex {
    pub complex_type: crate::types::ComplexType,
    pub archetypes: Vec<u8>,
}

impl Complex {
    pub fn new(complex_type: crate::types::ComplexType) -> Self {
        Complex {
            complex_type,
            archetypes: Vec::new(),
        }
    }

    pub fn add_archetype(&mut self, archetype_id: u8) {
        self.archetypes.push(archetype_id);
    }
}
