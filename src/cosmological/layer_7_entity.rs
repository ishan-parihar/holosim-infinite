//! Layer 7: Sub-Sub-Logos (Individual Entities)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "All creation is alive, everything is a sub-sub-Logos down to the limits of observation"
//!
//! This module implements individual entities that emerge from the top-down cosmological flow.

use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::entity_layer7::layer7::{
    DensityLevel, EntityExperience, EntityId, EntitySpectrumAccess, EntityState, EntityType,
    IndividualSpectrumConfiguration, SpectrumAccess, SpectrumAccessLevel, SubSubLogos,
    VibrationalState,
};
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::foundation::blue_realm::Logos;
use crate::foundation::green_realm::LightLoveField;
use crate::foundation::indigo_realm::IntelligentInfinity;
use crate::foundation::violet_realm::VioletRealm;
use crate::spectrum::larson_framework::SpectrumRatio;
use crate::spectrum::orange_realm::OrangeRealm;
use crate::spectrum::red_realm::RedRealm;
use crate::spectrum::yellow_realm::YellowRealm;
use crate::template::transcend_include::{
    AttractorField, Feature, LayerTransition, Orientation, TargetDensity, TranscendInclude,
};

use std::sync::Arc;

/// Individual entity (Sub-Sub-Logos) that emerges from cosmological layers
///
/// Each entity contains all previous layers (holographic principle) and has
/// individual Free Will to choose its evolutionary path.
#[derive(Debug, Clone)]
pub struct Entity {
    /// The underlying SubSubLogos
    pub entity: SubSubLogos,

    /// INCLUDES: All previous cosmological layers (holographic principle)
    pub cosmological_inheritance: CosmologicalInheritance,

    /// TRANSCENDS: Individual consciousness features
    pub individual_transcendence: IndividualTranscendence,

    /// EVOLVES INTO: Attractor for next density
    pub evolutionary_attractor: Option<AttractorField>,
}

/// All previous cosmological layers contained within this entity
#[derive(Debug, Clone)]
pub struct CosmologicalInheritance {
    pub violet: VioletRealm,
    pub indigo: IntelligentInfinity,
    pub blue: Logos,
    pub green: LightLoveField,
    pub yellow: YellowRealm,
    pub orange: OrangeRealm,
    pub red: RedRealm,
}

/// Individual features that transcend previous layers
#[derive(Debug, Clone)]
pub struct IndividualTranscendence {
    pub free_will_exercised: bool,
    pub consciousness_level: f64,
    pub experience_accumulated: f64,
    pub evolutionary_trajectory: EvolutionaryTrajectory,
    pub current_density: Density,
}

impl Entity {
    /// Create a new entity from cosmological layers (TOP-DOWN flow)
    pub fn from_cosmological_flow(
        cosmological_inheritance: CosmologicalInheritance,
        individual_features: IndividualTranscendence,
        evolutionary_attractor: Option<AttractorField>,
    ) -> Self {
        // Create spectrum configuration from the cosmological flow
        let spectrum = create_spectrum_from_layers(&cosmological_inheritance);

        // Create entity ID
        let entity_id = EntityId::new(format!("entity-{:x}", rand_simple()));

        // Create the underlying SubSubLogos with all cosmological layers
        let entity = SubSubLogos::new(
            entity_id,
            EntityType::Individual,
            None,       // parent_id
            Vec::new(), // composition
            None,       // environment_id
            cosmological_inheritance.violet.clone(),
            cosmological_inheritance.indigo.clone(),
            cosmological_inheritance.blue.clone(),
            cosmological_inheritance.green.clone(),
            cosmological_inheritance.yellow.clone(),
            cosmological_inheritance.orange.clone(),
            cosmological_inheritance.red.clone(),
            spectrum,
        );

        Self {
            entity,
            cosmological_inheritance,
            individual_transcendence: individual_features,
            evolutionary_attractor,
        }
    }

    /// Get entity ID
    pub fn entity_id(&self) -> &EntityId {
        &self.entity.entity_id
    }

    /// Get entity type
    pub fn entity_type(&self) -> EntityType {
        self.entity.entity_type
    }

    /// Get consciousness level
    pub fn consciousness_level(&self) -> f64 {
        self.individual_transcendence.consciousness_level
    }

    /// Get current density
    pub fn current_density(&self) -> Density {
        self.individual_transcendence.current_density
    }

    /// Exercise Free Will
    pub fn exercise_free_will(&mut self, choices: &[String]) -> String {
        self.individual_transcendence.free_will_exercised = true;

        if choices.is_empty() {
            return "default".to_string();
        }

        let index = (rand_simple() as usize) % choices.len();
        choices[index].clone()
    }

    /// Progress through density octave
    pub fn progress_density(&mut self) -> Option<Density> {
        let current = self.individual_transcendence.current_density;

        let next = match current {
            Density::First(_) => Some(Density::Second(Density2SubLevel::Cellular)),
            Density::Second(_) => Some(Density::Third),
            Density::Third => Some(Density::Fourth),
            Density::Fourth => Some(Density::Fifth),
            Density::Fifth => Some(Density::Sixth),
            Density::Sixth => Some(Density::Seventh),
            Density::Seventh => Some(Density::Eighth),
            Density::Eighth => None,
        };

        if let Some(new_density) = next {
            self.individual_transcendence.current_density = new_density;
            Some(new_density)
        } else {
            None
        }
    }

    /// Get the cosmological inheritance
    pub fn cosmological_inheritance(&self) -> &CosmologicalInheritance {
        &self.cosmological_inheritance
    }

    /// Get the evolutionary attractor
    pub fn evolutionary_attractor(&self) -> Option<&AttractorField> {
        self.evolutionary_attractor.as_ref()
    }
}

/// Create spectrum configuration from cosmological layers
fn create_spectrum_from_layers(
    inheritance: &CosmologicalInheritance,
) -> IndividualSpectrumConfiguration {
    // Get spectrum from yellow realm continuum
    let continuum = inheritance.yellow.spectrum_continuum();

    let ratio = if let Some(first) = continuum.first() {
        first.calculate_ratio()
    } else {
        1.0
    };

    IndividualSpectrumConfiguration::new(SpectrumRatio::space_time(ratio, 1.0))
}

/// Simple random number
fn rand_simple() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

/// Evolutionary trajectory
#[derive(Debug, Clone)]
pub struct EvolutionaryTrajectory {
    pub stage: Stage,
    pub progress: f64,
    pub polarity: Polarity,
    pub experience: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    FirstDensity,
    SecondDensity,
    ThirdDensity,
    FourthDensity,
    FifthDensity,
    SixthDensity,
    SeventhDensity,
    EighthDensity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    ServiceToOthers,
    ServiceToSelf,
    Neutral,
}

impl EvolutionaryTrajectory {
    pub fn new() -> Self {
        Self {
            stage: Stage::FirstDensity,
            progress: 0.0,
            polarity: Polarity::Neutral,
            experience: 0.0,
        }
    }

    pub fn add_experience(&mut self, points: f64) {
        self.experience += points;
        let threshold = match self.stage {
            Stage::FirstDensity => 100.0,
            Stage::SecondDensity => 500.0,
            Stage::ThirdDensity => 1000.0,
            Stage::FourthDensity => 2500.0,
            Stage::FifthDensity => 5000.0,
            Stage::SixthDensity => 10000.0,
            Stage::SeventhDensity => 25000.0,
            Stage::EighthDensity => f64::INFINITY,
        };

        self.progress = (self.experience / threshold).min(1.0);
    }
}

impl Default for EvolutionaryTrajectory {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the complete cosmological flow from Violet to Layer 7
pub fn build_cosmological_flow(num_entities: usize) -> Vec<Entity> {
    // Step 1: Initialize Violet Realm
    let violet = VioletRealm::new();

    // Step 2: Apply First Distortion → Indigo
    let (violet_included, _, _) = violet.apply_first_distortion();
    let indigo = IntelligentInfinity::from_violet(violet_included);

    // Step 3: Apply Second Distortion → Blue
    let (indigo_included, _, _) = indigo.apply_second_distortion();
    let blue = Logos::from_intelligent_infinity(indigo_included);

    // Step 4: Apply Third Distortion → Green
    let (blue_included, _, _) = blue.apply_third_distortion();
    let mut green = LightLoveField::from_logos(blue_included);
    green.add_holographic_pattern(crate::foundation::green_realm::HolographicPattern::new(
        0.8,
        [1.0, 0.0, 0.0],
        0.5,
    ));

    // Step 5: Apply Mysterious Emergence → Yellow
    let (green_included, _, _) = green.apply_mysterious_emergence();
    let mut yellow = YellowRealm::new(green_included);
    yellow.apply_mysterious_emergence().unwrap();

    // Step 6: Create Orange Realm (clone yellow for entity inheritance)
    let orange = OrangeRealm::new(yellow.clone());

    // Step 7: Create Red Realm (clone orange for entity inheritance)
    let red = RedRealm::new(orange.clone());

    // Step 8: Create entities from the cosmological flow
    let mut entities = Vec::with_capacity(num_entities);

    for i in 0..num_entities {
        let inheritance = CosmologicalInheritance {
            violet: violet.clone(),
            indigo: indigo.clone(),
            blue: blue.clone(),
            green: green.clone(),
            yellow: yellow.clone(),
            orange: orange.clone(),
            red: red.clone(),
        };

        let individual = IndividualTranscendence {
            free_will_exercised: false,
            consciousness_level: 0.1 + (i as f64 * 0.01),
            experience_accumulated: 0.0,
            evolutionary_trajectory: EvolutionaryTrajectory::new(),
            current_density: Density::First(Density1SubLevel::Quantum),
        };

        let attractor = AttractorField::new(
            format!("Entity {} Attractor", i),
            0.7,
            Orientation::Balanced,
            TargetDensity::Third,
        );

        let entity = Entity::from_cosmological_flow(inheritance, individual, Some(attractor));
        entities.push(entity);
    }

    entities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_cosmological_flow_creates_entities() {
        let entities = build_cosmological_flow(10);
        assert_eq!(entities.len(), 10);
    }

    #[ignore]
    #[test]
    fn test_entity_contains_all_layers() {
        let entities = build_cosmological_flow(1);
        let entity = &entities[0];

        let inheritance = &entity.cosmological_inheritance;
        assert_eq!(inheritance.violet.unity, 1.0);
        assert!(inheritance.indigo.is_aware());
    }

    #[ignore]
    #[test]
    fn test_entity_density_progression() {
        let entities = build_cosmological_flow(1);
        let mut entity = entities[0].clone();

        assert_eq!(
            entity.current_density(),
            Density::First(Density1SubLevel::Quantum)
        );

        for _ in 0..10 {
            entity.progress_density();
        }
    }
}
