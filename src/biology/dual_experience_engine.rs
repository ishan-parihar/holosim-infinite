//! Dual Experience Engine - Space/Time vs Time/Space Qualitative Differences
//!
//! From V4 Roadmap Phase 5: Gap #11 resolution

use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Spectrum Position
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpectrumPosition {
    DeepSpaceTime,
    SpaceTime,
    Balanced,
    TimeSpace,
    DeepTimeSpace,
}

impl SpectrumPosition {
    pub fn from_velocity(velocity: Float) -> Self {
        if velocity < 0.3 {
            SpectrumPosition::DeepSpaceTime
        } else if velocity < 0.7 {
            SpectrumPosition::SpaceTime
        } else if velocity < 1.3 {
            SpectrumPosition::Balanced
        } else if velocity < 1.7 {
            SpectrumPosition::TimeSpace
        } else {
            SpectrumPosition::DeepTimeSpace
        }
    }

    pub fn time_perception_modifier(&self) -> Float {
        match self {
            SpectrumPosition::DeepSpaceTime => 0.1,
            SpectrumPosition::SpaceTime => 0.5,
            SpectrumPosition::Balanced => 1.0,
            SpectrumPosition::TimeSpace => 2.0,
            SpectrumPosition::DeepTimeSpace => 5.0,
        }
    }

    pub fn space_perception_modifier(&self) -> Float {
        match self {
            SpectrumPosition::DeepSpaceTime => 2.0,
            SpectrumPosition::SpaceTime => 1.5,
            SpectrumPosition::Balanced => 1.0,
            SpectrumPosition::TimeSpace => 0.5,
            SpectrumPosition::DeepTimeSpace => 0.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CausalityMode {
    StrictLinear,
    MostlyLinear,
    Flexible,
    Resonant,
    Simultaneous,
}

// ============================================================================
// Experience Modality
// ============================================================================

#[derive(Debug, Clone)]
pub struct ExperienceModality {
    pub position: SpectrumPosition,
    pub velocity: Float,
    pub time_space_ratio: Float,
    pub veil_transparency: Float,
}

impl Default for ExperienceModality {
    fn default() -> Self {
        Self {
            position: SpectrumPosition::SpaceTime,
            velocity: 0.5,
            time_space_ratio: 2.0,
            veil_transparency: 0.1,
        }
    }
}

impl ExperienceModality {
    pub fn from_space_time_ratio(space_time_ratio: Float) -> Self {
        let velocity = 1.0 / space_time_ratio;
        let position = SpectrumPosition::from_velocity(velocity);
        let veil_transparency = if velocity >= 1.0 {
            ((velocity - 1.0) / 2.0).min(1.0) * 0.8
        } else {
            0.1
        };

        Self {
            position,
            velocity,
            time_space_ratio: space_time_ratio,
            veil_transparency,
        }
    }

    pub fn update(&mut self, growth_direction: GrowthPolarity, polarity_strength: Float) {
        match growth_direction {
            GrowthPolarity::ServiceToOthers => {
                self.velocity = (self.velocity + polarity_strength * 0.01).min(3.0);
            }
            GrowthPolarity::ServiceToSelf => {
                self.velocity = (self.velocity - polarity_strength * 0.005).max(0.1);
            }
            GrowthPolarity::Neutral => {}
        }

        self.time_space_ratio = 1.0 / self.velocity;
        self.position = SpectrumPosition::from_velocity(self.velocity);

        self.veil_transparency = if self.velocity >= 1.0 {
            ((self.velocity - 1.0) / 2.0).min(1.0) * 0.8
        } else {
            0.1
        };
    }
}

// ============================================================================
// Experience Structs
// ============================================================================

#[derive(Debug, Clone)]
pub struct TimeExperience {
    pub flow_rate: Float,
    pub linearity: Float,
}

#[derive(Debug, Clone)]
pub struct SpaceExperience {
    pub solidity: Float,
}

#[derive(Debug, Clone)]
pub struct CausalityExperience {
    pub mode: CausalityMode,
}

#[derive(Debug, Clone)]
pub struct QualitativeExperience {
    pub time: TimeExperience,
    pub space: SpaceExperience,
    pub causality: CausalityExperience,
    pub veil_transparency: Float,
    pub spectrum_position: SpectrumPosition,
}

impl Default for QualitativeExperience {
    fn default() -> Self {
        Self {
            time: TimeExperience {
                flow_rate: 1.0,
                linearity: 0.9,
            },
            space: SpaceExperience { solidity: 0.8 },
            causality: CausalityExperience {
                mode: CausalityMode::Flexible,
            },
            veil_transparency: 0.1,
            spectrum_position: SpectrumPosition::SpaceTime,
        }
    }
}

impl ExperienceModality {
    pub fn to_qualitative(&self) -> QualitativeExperience {
        let linearity = match self.position {
            SpectrumPosition::DeepSpaceTime => 1.0,
            SpectrumPosition::SpaceTime => 0.9,
            SpectrumPosition::Balanced => 0.5,
            SpectrumPosition::TimeSpace => 0.3,
            SpectrumPosition::DeepTimeSpace => 0.0,
        };

        let solidity = match self.position {
            SpectrumPosition::DeepSpaceTime => 1.0,
            SpectrumPosition::SpaceTime => 0.8,
            SpectrumPosition::Balanced => 0.5,
            SpectrumPosition::TimeSpace => 0.3,
            SpectrumPosition::DeepTimeSpace => 0.1,
        };

        let flow_rate = match self.position {
            SpectrumPosition::DeepSpaceTime => 0.1,
            SpectrumPosition::SpaceTime => 0.5,
            SpectrumPosition::Balanced => 1.0,
            SpectrumPosition::TimeSpace => 2.0,
            SpectrumPosition::DeepTimeSpace => 5.0,
        };

        QualitativeExperience {
            time: TimeExperience {
                flow_rate,
                linearity,
            },
            space: SpaceExperience { solidity },
            causality: CausalityExperience {
                mode: CausalityMode::Flexible,
            },
            veil_transparency: self.veil_transparency,
            spectrum_position: self.position,
        }
    }
}

// ============================================================================
// Growth Polarity
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthPolarity {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

// ============================================================================
// Dual Experience Engine
// ============================================================================

pub struct DualExperienceEngine {
    pub entity_modalities: HashMap<u64, ExperienceModality>,
    pub global_modality: ExperienceModality,
}

impl Default for DualExperienceEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DualExperienceEngine {
    pub fn new() -> Self {
        Self {
            entity_modalities: HashMap::new(),
            global_modality: ExperienceModality::default(),
        }
    }

    pub fn register_entity(&mut self, entity_id: u64, initial_space_time_ratio: Float) {
        let modality = ExperienceModality::from_space_time_ratio(initial_space_time_ratio);
        self.entity_modalities.insert(entity_id, modality);
    }

    pub fn get_modality(&mut self, entity_id: u64) -> &mut ExperienceModality {
        self.entity_modalities
            .entry(entity_id)
            .or_insert_with(ExperienceModality::default)
    }

    pub fn get_qualitative_experience(&self, entity_id: u64) -> QualitativeExperience {
        self.entity_modalities
            .get(&entity_id)
            .map(|m| m.to_qualitative())
            .unwrap_or_else(QualitativeExperience::default)
    }

    pub fn update_entity(
        &mut self,
        entity_id: u64,
        growth: GrowthPolarity,
        polarity_strength: Float,
    ) {
        if let Some(modality) = self.entity_modalities.get_mut(&entity_id) {
            modality.update(growth, polarity_strength);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_modality_creation() {
        let modality = ExperienceModality::from_space_time_ratio(0.5);
        assert!(modality.velocity < 1.0);
    }

    #[ignore]
    #[test]
    fn test_dual_experience_engine() {
        let mut engine = DualExperienceEngine::new();
        engine.register_entity(1, 0.5);
        let exp = engine.get_qualitative_experience(1);
        assert_eq!(exp.spectrum_position, SpectrumPosition::SpaceTime);
    }
}
