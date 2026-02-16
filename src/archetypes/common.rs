//! Common types and traits for archetypes

use crate::types::{Float, Octant, Polarity, Rung};

/// Common archetype placeholder
pub struct ArchetypeCommon {
    pub id: u8,
}

impl ArchetypeCommon {
    pub fn new(id: u8) -> Self {
        ArchetypeCommon { id }
    }
}

/// Archetype Complex - Mind/Body/Spirit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeComplex {
    Mind,
    Body,
    Spirit,

    /// Unified complex (all archetypes integrated)
    Unified,
}

/// Archetype Role - Matrix/Potentiator/Catalyst/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeRole {
    Matrix,
    Potentiator,
    Catalyst,
    Experience,
    Significator,
    Transformation,
    GreatWay,

    /// Archetype 22: Choice (Free Will)
    Choice,
}

/// Developmental Position - Input/Catalyst/Experience/Significator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevelopmentalPosition {
    Input,
    Catalyst,
    Experience,
    Significator,
}

impl DevelopmentalPosition {
    pub fn new(position: u8) -> Self {
        match position {
            0 => DevelopmentalPosition::Input,
            1 => DevelopmentalPosition::Catalyst,
            2 => DevelopmentalPosition::Experience,
            3 => DevelopmentalPosition::Significator,
            _ => DevelopmentalPosition::Input,
        }
    }

    pub fn new_with_octant_rung(octant: Octant, rung: u8) -> Self {
        match rung % 4 {
            0 => DevelopmentalPosition::Input,
            1 => DevelopmentalPosition::Catalyst,
            2 => DevelopmentalPosition::Experience,
            3 => DevelopmentalPosition::Significator,
            _ => DevelopmentalPosition::Input,
        }
    }

    pub fn rung_level(&self) -> u8 {
        match self {
            DevelopmentalPosition::Input => 0,
            DevelopmentalPosition::Catalyst => 1,
            DevelopmentalPosition::Experience => 2,
            DevelopmentalPosition::Significator => 3,
        }
    }

    /// Get the actual rung number (1-7) from the position type
    /// This is used for activation purposes where the original rung matters
    pub fn actual_rung(&self) -> u8 {
        match self {
            DevelopmentalPosition::Input => 1,
            DevelopmentalPosition::Catalyst => 2,
            DevelopmentalPosition::Experience => 3,
            DevelopmentalPosition::Significator => 4,
        }
    }
}

/// Functional Pair - Matrix/Process/Experience/Significator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionalPair {
    MatrixPair,
    ProcessPair,
    ExperiencePair,
    SignificatorPair,
    IdentityPair,
    StructurePair,
    TransformationSingleton,
}

/// Health Status - Balanced/Imbalanced
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Balanced,
    Warning,
    Pathological,
    PathologicalLow,
    PathologicalHigh,
    Imbalanced,
    Degraded,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Balanced => write!(f, "Balanced"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Pathological => write!(f, "Pathological"),
            HealthStatus::PathologicalLow => write!(f, "Pathological Low"),
            HealthStatus::PathologicalHigh => write!(f, "Pathological High"),
            HealthStatus::Imbalanced => write!(f, "Imbalanced"),
            HealthStatus::Degraded => write!(f, "Degraded"),
        }
    }
}

/// Holonic Level - Individual/Collective
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolonicLevel {
    Micro,
    Meso,
    Macro,
    Meta,
}

/// Lambda Measurement - measurement of archetype activity
#[derive(Debug, Clone)]
pub struct LambdaMeasurement {
    pub value: Float,
    pub measurement_type: LambdaMeasurementType,
    pub healthy_min: Float,
    pub healthy_max: Float,
}

impl LambdaMeasurement {
    pub fn new(value: Float, measurement_type: LambdaMeasurementType) -> Self {
        LambdaMeasurement {
            value,
            measurement_type,
            healthy_min: 0.3,
            healthy_max: 0.7,
        }
    }

    pub fn lambda(&self) -> Float {
        self.value
    }

    pub fn is_healthy(&self) -> bool {
        self.value >= self.healthy_min && self.value <= self.healthy_max
    }

    pub fn health_status(&self) -> HealthStatus {
        if self.value < self.healthy_min {
            HealthStatus::PathologicalLow
        } else if self.value > self.healthy_max {
            HealthStatus::PathologicalHigh
        } else {
            HealthStatus::Healthy
        }
    }

    pub fn is_pathological_low(&self) -> bool {
        self.value < self.healthy_min
    }

    pub fn is_pathological_high(&self) -> bool {
        self.value > self.healthy_max
    }

    /// Adjust the lambda value by a delta amount
    /// Clamps the result to valid range [0.0, 1.0]
    pub fn adjust(&mut self, delta: Float) {
        self.value = (self.value + delta).clamp(0.0, 1.0);
    }
}

/// Lambda Measurement Type - Capacity/Intensity/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LambdaMeasurementType {
    Capacity,
    Intensity,
    Activation,
    TransformationVelocity,
    ExperienceDepth,
    SignificatorCoherence,
    GreatWayClarity,
    CatalystProcessingRate,
    MatrixRigidity,
    PotentiatorAccessibility,
    ChoicePolarity,
}

/// Sigma Axis - Mind/Body/Spirit capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaAxis {
    SigmaA,
    SigmaB,
    SigmaC,
    MindCapacity,
    BodyCapacity,
    SpiritCapacity,
}

/// Tarot Correlation - Tarot card for this archetype
#[derive(Debug, Clone)]
pub struct TarotCorrelation {
    pub card: String,
}

impl TarotCorrelation {
    pub fn new(card: String) -> Self {
        TarotCorrelation { card }
    }
}

/// Archetype Trait - base trait for archetypes
pub trait ArchetypeTrait {
    fn activate(&mut self, intensity: Float);
    fn deactivate(&mut self);
    fn is_active(&self) -> bool;
    fn role(&self) -> ArchetypeRole;
    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition);
    fn sigma_axis(&self) -> SigmaAxis;
    fn tarot_correlation(&self) -> TarotCorrelation;
    fn update_lambda(&mut self, value: Float);
    fn archetype_id(&self) -> u8;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn complex(&self) -> ArchetypeComplex;
    fn functional_pair(&self) -> FunctionalPair;
    fn lambda(&self) -> &LambdaMeasurement;
    fn is_healthy(&self) -> bool;
    fn health_status(&self) -> HealthStatus;
}

/// Developmental Trait - for archetypes that can develop
pub trait Developmental {
    fn develop(&mut self, catalyst: Float);
    fn regress(&mut self, resistance: Float);
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn rung_activation_level(&self, rung: Rung) -> Float;
    fn activated_rungs(&self) -> Vec<Rung>;
    fn developmental_position(&self) -> DevelopmentalPosition;
}

/// Holonic Trait - for archetypes with holonic levels
pub trait Holonic {
    fn get_holonic_level(&self) -> HolonicLevel;
    fn set_holonic_level(&mut self, level: HolonicLevel);
    fn transcend(&mut self) -> bool;
    fn include(&mut self, other: &dyn Holonic) -> bool;
    fn holonic_level(&self) -> HolonicLevel;
    fn integration_capacity(&self) -> Float;
}

/// Paired Trait - for archetypes that are paired
pub trait Paired {
    fn get_pair(&self) -> Option<u8>;
    fn lambda(&self) -> &LambdaMeasurement;
    fn paired_archetype_id(&self) -> Option<u8>;
    fn calculate_pair_tension(&self, paired: &dyn Paired) -> Float;
    fn calculate_pair_balance(&self, paired: &dyn Paired) -> Float;
}

/// Lambda Measurable Trait - for archetypes with lambda measurements
pub trait LambdaMeasurable {
    fn get_lambda(&self) -> Float;
    fn set_lambda(&mut self, value: Float);
    fn calculate_lambda(&self) -> Float;
    fn pathological_indicators(&self) -> Vec<String>;
    fn healthy_range(&self) -> (Float, Float);
}
