/// Involution Sequence Module
///
/// This module implements the complete involution sequence that creates entities
/// from the source (Violet) through the Three Primal Distortions and down to
/// Layer 7 (individual entities).
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Reality is not constructed; it is Unfolded from a Pre-Existing Whole."
///
/// The involution sequence is the Compiler Phase—the process by which Infinity
/// manifests as the Light/Love architecture that forms the basis of all creation.
///
/// ## The Sequence
///
/// 1. **Violet-Ray (Layer 0)**: Infinity - undifferentiated unity
/// 2. **Indigo-Ray (Layer 1)**: IntelligentInfinity - First Distortion (Free Will)
/// 3. **Blue-Ray (Layer 2)**: Love/Light - Second Distortion (Love/Logos)
/// 4. **Green-Ray (Layer 3)**: Light/Love - Third Distortion (Light)
/// 5. **Yellow-Ray (Layer 4)**: Space/Time and Time/Space Spectrum - Dimensional actualization
/// 6. **Orange-Ray (Layer 5)**: Galactic-scale spectrum configuration
/// 7. **Red-Ray (Layer 6)**: Solar-scale spectrum configuration + archetypical mind
/// 8. **Layer 7**: Individual entity inheritance with holographic blueprint
///
/// Each step follows the "transcend and include" universal constant:
/// - INCLUDE: Retains all development from previous stages
/// - TRANSCEND: Adds new development that transcends the previous
/// - EVOLVES INTO: Creates attractor-fields that pull toward the next stage
use crate::entity_layer7::{IndividualSpectrumConfiguration, SubSubLogos};
use crate::foundation::{
    AttractorField, Feature, Field, HolographicPattern, IntelligentInfinity, LightLoveField, Logos,
    Rhythm, VioletRealm,
};
use crate::spectrum::{OrangeRealm, RedRealm, SpectrumRatio as EntitySpectrumRatio, YellowRealm};
use std::time::Duration;

/// Errors that can occur during involution sequence
#[derive(Debug, Clone, PartialEq)]
pub enum InvolutionError {
    /// Stage transition failed
    StageTransitionFailed {
        stage: InvolutionStage,
        reason: String,
    },
    /// Attractor field creation failed
    AttractorFieldCreationFailed {
        stage: InvolutionStage,
        reason: String,
    },
    /// Entity creation failed
    EntityCreationFailed { reason: String },
}

impl std::fmt::Display for InvolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvolutionError::StageTransitionFailed { stage, reason } => {
                write!(f, "Stage transition failed at {:?}: {}", stage, reason)
            }
            InvolutionError::AttractorFieldCreationFailed { stage, reason } => {
                write!(
                    f,
                    "Attractor field creation failed at {:?}: {}",
                    stage, reason
                )
            }
            InvolutionError::EntityCreationFailed { reason } => {
                write!(f, "Entity creation failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for InvolutionError {}

/// Involution Stage - represents each layer in the involution sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvolutionStage {
    /// Layer 0: Violet-Ray - Infinity as undifferentiated unity
    Violet,
    /// Layer 1: Indigo-Ray - IntelligentInfinity (First Distortion: Free Will)
    Indigo,
    /// Layer 2: Blue-Ray - Love/Light (Second Distortion: Love/Logos)
    Blue,
    /// Layer 3: Green-Ray - Light/Love field (Third Distortion: Light)
    Green,
    /// Layer 4: Yellow-Ray - Space/Time and Time/Space Spectrum
    Yellow,
    /// Layer 5: Orange-Ray - Galactic-scale spectrum configuration
    Orange,
    /// Layer 6: Red-Ray - Solar-scale spectrum configuration + archetypical mind
    Red,
    /// Layer 7: Individual entity inheritance with holographic blueprint
    Layer7,
}

impl InvolutionStage {
    /// Get the layer number (0-7)
    pub fn layer_number(&self) -> usize {
        match self {
            InvolutionStage::Violet => 0,
            InvolutionStage::Indigo => 1,
            InvolutionStage::Blue => 2,
            InvolutionStage::Green => 3,
            InvolutionStage::Yellow => 4,
            InvolutionStage::Orange => 5,
            InvolutionStage::Red => 6,
            InvolutionStage::Layer7 => 7,
        }
    }

    /// Get the ray color
    pub fn ray_color(&self) -> &'static str {
        match self {
            InvolutionStage::Violet => "Violet-Ray",
            InvolutionStage::Indigo => "Indigo-Ray",
            InvolutionStage::Blue => "Blue-Ray",
            InvolutionStage::Green => "Green-Ray",
            InvolutionStage::Yellow => "Yellow-Ray",
            InvolutionStage::Orange => "Orange-Ray",
            InvolutionStage::Red => "Red-Ray",
            InvolutionStage::Layer7 => "Beyond Red-Ray",
        }
    }

    /// Get the description of this stage
    pub fn description(&self) -> &'static str {
        match self {
            InvolutionStage::Violet => {
                "Infinity as undifferentiated unity. The first known thing in creation."
            }
            InvolutionStage::Indigo => {
                "IntelligentInfinity. Infinity becomes aware through Free Will (First Distortion)."
            }
            InvolutionStage::Blue => {
                "Love/Light. Logos emerges through Love (Second Distortion). The Creative Principle."
            }
            InvolutionStage::Green => {
                "Light/Love. Light manifests through Light (Third Distortion). The field of potential."
            }
            InvolutionStage::Yellow => {
                "The Great Mystery. Space/Time and Time/Time Space Spectrum emerges. Dimensions form."
            }
            InvolutionStage::Orange => {
                "Galactic-scale Logoi. Galactic-scale spectrum configuration. Patterns for galaxies."
            }
            InvolutionStage::Red => {
                "Solar-scale Logoi. Solar-system creation and archetypical mind development."
            }
            InvolutionStage::Layer7 => {
                "Sub-Sub-Logos. Individual entity inheritance with holographic blueprint."
            }
        }
    }
}

impl std::fmt::Display for InvolutionStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Layer {})", self.ray_color(), self.layer_number())
    }
}

/// Result of running the involution sequence
#[derive(Debug, Clone)]
pub struct InvolutionResult {
    /// The entities created from the involution sequence
    pub entities: Vec<SubSubLogos>,
    /// The attractor-fields created at each transition
    pub attractor_fields: Vec<AttractorField>,
    /// The stage transitions that occurred
    pub stage_transitions: Vec<StageTransitionRecord>,
    /// Time taken to run the involution sequence
    pub execution_time: Duration,
}

/// Record of a stage transition
#[derive(Debug, Clone)]
pub struct StageTransitionRecord {
    /// The stage this transition is from
    pub from_stage: InvolutionStage,
    /// The stage this transition is to
    pub to_stage: InvolutionStage,
    /// The feature that was added (transcends)
    pub feature: Feature,
    /// The attractor-field that was created (evolves into)
    pub attractor_field: AttractorField,
}

/// Involution Sequence Runner
///
/// This struct manages the complete involution sequence from Violet (Infinity)
/// to Layer 7 (individual entities).
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The involution sequence is the Compiler Phase—the process by which Infinity
/// manifests as the Light/Love architecture that forms the basis of all creation."
///
/// The runner applies the Three Primal Distortions at the correct stages and
/// creates attractor-fields at each transition, following the "transcend and
/// include" universal constant.
pub struct InvolutionSequenceRunner {
    /// Current stage in the involution sequence
    current_stage: InvolutionStage,

    /// Layer 0: Violet Realm (Infinity)
    violet_realm: Option<VioletRealm>,

    /// Layer 1: IntelligentInfinity (after First Distortion)
    intelligent_infinity: Option<IntelligentInfinity>,

    /// Layer 2: Logos (after Second Distortion)
    logos: Option<Logos>,

    /// Layer 3: Light/Love field (after Third Distortion)
    light_love_field: Option<LightLoveField>,

    /// Layer 4: Yellow Realm (Space/Time spectrum)
    yellow_realm: Option<YellowRealm>,

    /// Layer 5: Orange Realm (Galactic-scale configuration)
    orange_realm: Option<OrangeRealm>,

    /// Layer 6: Red Realm (Solar-scale configuration + archetypical mind)
    red_realm: Option<RedRealm>,

    /// Layer 7: Individual entities
    entities: Vec<SubSubLogos>,

    /// Attractor-fields created at each transition
    attractor_fields: Vec<AttractorField>,

    /// Stage transition records
    stage_transitions: Vec<StageTransitionRecord>,

    /// Configuration for entity creation
    num_galaxies: usize,
    num_solar_systems: usize,
    num_planets: usize,
}

impl Default for InvolutionSequenceRunner {
    fn default() -> Self {
        InvolutionSequenceRunner::new()
    }
}

impl InvolutionSequenceRunner {
    /// Create a new Involution Sequence Runner
    ///
    /// This creates a runner with default configuration:
    /// - 3 galaxies
    /// - 2 solar systems per galaxy
    /// - 3 planets per solar system
    pub fn new() -> Self {
        InvolutionSequenceRunner {
            current_stage: InvolutionStage::Violet,
            violet_realm: None,
            intelligent_infinity: None,
            logos: None,
            light_love_field: None,
            yellow_realm: None,
            orange_realm: None,
            red_realm: None,
            entities: Vec::new(),
            attractor_fields: Vec::new(),
            stage_transitions: Vec::new(),
            num_galaxies: 3,
            num_solar_systems: 2,
            num_planets: 3,
        }
    }

    /// Create a new Involution Sequence Runner with custom configuration
    pub fn with_config(
        _entity_count: usize,
        num_galaxies: usize,
        num_solar_systems: usize,
        num_planets: usize,
    ) -> Self {
        InvolutionSequenceRunner {
            current_stage: InvolutionStage::Violet,
            violet_realm: None,
            intelligent_infinity: None,
            logos: None,
            light_love_field: None,
            yellow_realm: None,
            orange_realm: None,
            red_realm: None,
            entities: Vec::new(),
            attractor_fields: Vec::new(),
            stage_transitions: Vec::new(),
            num_galaxies,
            num_solar_systems,
            num_planets,
        }
    }

    /// Run the complete involution sequence
    ///
    /// This runs through all stages from Violet (Infinity) to Layer 7 (entities),
    /// applying the Three Primal Distortions and creating attractor-fields at
    /// each transition.
    ///
    /// # Returns
    ///
    /// Returns an `InvolutionResult` containing:
    /// - The entities created
    /// - The attractor-fields created
    /// - The stage transition records
    /// - The execution time
    ///
    /// # Errors
    ///
    /// Returns an `InvolutionError` if any stage transition fails.
    pub fn run_involution_sequence(&mut self) -> Result<InvolutionResult, InvolutionError> {
        let start_time = std::time::Instant::now();

        // Stage 0: Initialize Violet Realm
        self.initialize_violet_realm()?;

        // Stage 1: Apply First Distortion (Free Will)
        self.apply_first_distortion()?;

        // Stage 2: Apply Second Distortion (Love)
        self.apply_second_distortion()?;

        // Stage 3: Apply Third Distortion (Light)
        self.apply_third_distortion()?;

        // Stage 4: Dimensional actualization (Yellow-Ray)
        self.create_dimensional_architecture()?;

        // Stage 5: Galactic-scale configuration (Orange-Ray)
        self.configure_galactic_spectrum()?;

        // Stage 6: Solar-scale configuration (Red-Ray)
        self.configure_solar_spectrum()?;

        // Stage 7: Create individual entities (Layer 7)
        self.create_individual_entities()?;

        let execution_time = start_time.elapsed();

        Ok(InvolutionResult {
            entities: self.entities.clone(),
            attractor_fields: self.attractor_fields.clone(),
            stage_transitions: self.stage_transitions.clone(),
            execution_time,
        })
    }

    /// Initialize Violet Realm (Layer 0)
    ///
    /// This creates the source state of all creation: Infinity as undifferentiated unity.
    fn initialize_violet_realm(&mut self) -> Result<(), InvolutionError> {
        self.current_stage = InvolutionStage::Violet;
        self.violet_realm = Some(VioletRealm::new());
        Ok(())
    }

    /// Apply First Distortion (Free Will) to transition to Indigo-Ray (Layer 1)
    ///
    /// This is the first movement away from perfect unity that creates the
    /// possibility of self-knowledge.
    ///
    /// Creates:
    /// - IntelligentInfinity (Infinity becomes aware)
    /// - Archetype 22 (The Choice) - attractor-field
    fn apply_first_distortion(&mut self) -> Result<(), InvolutionError> {
        let violet = self
            .violet_realm
            .as_ref()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Violet,
                reason: "Violet realm not initialized".to_string(),
            })?;

        // Apply First Distortion
        let (violet_included, first_distortion, archetype22) = violet.apply_first_distortion();

        // Create IntelligentInfinity
        let intelligent = IntelligentInfinity::from_violet(violet_included);

        // Record attractor-field
        self.attractor_fields.push(archetype22.clone());

        // Record stage transition
        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Violet,
            to_stage: InvolutionStage::Indigo,
            feature: first_distortion,
            attractor_field: archetype22,
        });

        // Update state
        self.current_stage = InvolutionStage::Indigo;
        self.intelligent_infinity = Some(intelligent);

        Ok(())
    }

    /// Apply Second Distortion (Love) to transition to Blue-Ray (Layer 2)
    ///
    /// This focuses Infinity as an aware or conscious principle, creating the Logos.
    ///
    /// Creates:
    /// - Logos (the Creative Principle)
    /// - Universal Archetypical Patterns - attractor-field
    fn apply_second_distortion(&mut self) -> Result<(), InvolutionError> {
        let intelligent =
            self.intelligent_infinity
                .as_ref()
                .ok_or(InvolutionError::StageTransitionFailed {
                    stage: InvolutionStage::Indigo,
                    reason: "IntelligentInfinity not initialized".to_string(),
                })?;

        // Apply Second Distortion
        let (intelligent_included, second_distortion, universal_patterns) =
            intelligent.apply_second_distortion();

        // Create Logos
        let logos = Logos::from_intelligent_infinity(intelligent_included);

        // Record attractor-field
        self.attractor_fields.push(universal_patterns.clone());

        // Record stage transition
        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Indigo,
            to_stage: InvolutionStage::Blue,
            feature: second_distortion,
            attractor_field: universal_patterns,
        });

        // Update state
        self.current_stage = InvolutionStage::Blue;
        self.logos = Some(logos);

        Ok(())
    }

    /// Apply Third Distortion (Light) to transition to Green-Ray (Layer 3)
    ///
    /// This is the third primal distortion where Light impressed with love
    /// begins to manifest, creating the Light/Love field of potential.
    ///
    /// Creates:
    /// - Light/Love field (manifestation field)
    /// - Holographic patterns
    /// - Rhythms
    /// - Fields
    fn apply_third_distortion(&mut self) -> Result<(), InvolutionError> {
        let logos = self
            .logos
            .as_ref()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Blue,
                reason: "Logos not initialized".to_string(),
            })?;

        // Apply Third Distortion
        let (logos_included, third_distortion, light_love_field) = logos.apply_third_distortion();

        // Create Light/Love field
        let mut field = LightLoveField::from_logos(logos_included);

        // Add spectrum conditions (required for mysterious emergence)
        // Add holographic patterns
        field.add_holographic_pattern(HolographicPattern::new(0.8, [1.0, 0.8, 0.6], 0.9));
        field.add_holographic_pattern(HolographicPattern::new(0.7, [0.6, 0.8, 1.0], 0.85));
        field.add_holographic_pattern(HolographicPattern::new(0.75, [0.9, 0.7, 0.5], 0.88));

        // Add rhythms
        field.add_rhythm(Rhythm::new(0.5, 0.8, 0.0));
        field.add_rhythm(Rhythm::new(0.6, 0.75, 0.1));
        field.add_rhythm(Rhythm::new(0.55, 0.85, 0.05));

        // Add fields
        field.add_field(Field::new(0.9, 0.85, "Light Field"));
        field.add_field(Field::new(0.88, 0.82, "Love Field"));
        field.add_field(Field::new(0.92, 0.87, "Manifestation Field"));

        // Record attractor-field
        self.attractor_fields.push(light_love_field.clone());

        // Record stage transition
        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Blue,
            to_stage: InvolutionStage::Green,
            feature: third_distortion,
            attractor_field: light_love_field,
        });

        // Update state
        self.current_stage = InvolutionStage::Green;
        self.light_love_field = Some(field);

        Ok(())
    }

    /// Create dimensional architecture to transition to Yellow-Ray (Layer 4)
    ///
    /// This is the mysterious actualization where energy patterns regularize
    /// into dimensions and universes, creating the Space/Time and Time/Space
    /// spectrum with the Veil at v=1.
    ///
    /// Creates:
    /// - Dimensional architecture (Space/Time spectrum)
    /// - Galactic-scale spectrum configuration - attractor-field
    fn create_dimensional_architecture(&mut self) -> Result<(), InvolutionError> {
        let field = self
            .light_love_field
            .take()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Green,
                reason: "Light/Love field not initialized".to_string(),
            })?;

        // Apply mysterious emergence to create Yellow Realm
        let mut yellow = YellowRealm::new(field);

        yellow.apply_mysterious_emergence().map_err(|e| {
            InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Green,
                reason: format!("Failed to apply mysterious emergence: {}", e),
            }
        })?;

        // Get attractor-field
        let attractor_field = yellow.attractor_field.clone();

        // Record attractor-field
        self.attractor_fields.push(attractor_field.clone());

        // Record stage transition
        let mysterious_emergence = Feature::new(
            "The Mysterious Emergence",
            "Space/Time and Time/Space Spectrum emerges",
            1.0,
        );

        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Green,
            to_stage: InvolutionStage::Yellow,
            feature: mysterious_emergence,
            attractor_field,
        });

        // Update state
        self.current_stage = InvolutionStage::Yellow;
        self.yellow_realm = Some(yellow);

        Ok(())
    }

    /// Configure galactic-scale spectrum to transition to Orange-Ray (Layer 5)
    ///
    /// Galactic-scale Logoi configure the spectrum at galactic scale, creating
    /// the patterns that galaxies will follow when physical matter forms.
    ///
    /// Creates:
    /// - Galactic-scale spectrum configuration
    /// - Solar-scale spectrum configuration + archetypical mind - attractor-field
    fn configure_galactic_spectrum(&mut self) -> Result<(), InvolutionError> {
        let yellow = self
            .yellow_realm
            .take()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Yellow,
                reason: "Yellow realm not initialized".to_string(),
            })?;

        // Apply galactic configuration
        let mut orange = OrangeRealm::new(yellow);

        orange.apply_galactic_configuration().map_err(|e| {
            InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Yellow,
                reason: format!("Failed to apply galactic configuration: {}", e),
            }
        })?;

        // Create solar systems
        orange.create_solar_systems(self.num_solar_systems);

        // Get attractor-field
        let attractor_field = orange.attractor_field.clone();

        // Record attractor-field
        self.attractor_fields.push(attractor_field.clone());

        // Record stage transition
        let galactic_configuration = Feature::new(
            "Galactic-scale Spectrum Configuration",
            "Galactic-scale Logoi configure spectrum at galactic scale",
            1.0,
        );

        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Yellow,
            to_stage: InvolutionStage::Orange,
            feature: galactic_configuration,
            attractor_field,
        });

        // Update state
        self.current_stage = InvolutionStage::Orange;
        self.orange_realm = Some(orange);

        Ok(())
    }

    /// Configure solar-scale spectrum to transition to Red-Ray (Layer 6)
    ///
    /// Solar-scale Logoi configure the spectrum at solar scale and develop
    /// specific archetypical mind systems (10 or 22 archetypes).
    ///
    /// Creates:
    /// - Solar-scale spectrum configuration
    /// - Archetypical mind system
    /// - Individual entity inheritance with holographic blueprint - attractor-field
    fn configure_solar_spectrum(&mut self) -> Result<(), InvolutionError> {
        let orange = self
            .orange_realm
            .take()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Orange,
                reason: "Orange realm not initialized".to_string(),
            })?;

        // Apply solar configuration
        let mut red = RedRealm::new(orange);

        red.apply_solar_configuration()
            .map_err(|e| InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Orange,
                reason: format!("Failed to apply solar configuration: {}", e),
            })?;

        // Create planets
        red.create_planets(self.num_planets);

        // Activate archetypical minds
        red.activate_archetypical_minds();

        // Get attractor-field
        let attractor_field = red.attractor_field.clone();

        // Record attractor-field
        self.attractor_fields.push(attractor_field.clone());

        // Record stage transition
        let solar_configuration = Feature::new(
            "Solar-scale Spectrum Configuration + Archetypical Mind",
            "Solar-scale Logoi configure spectrum and develop archetypical mind systems",
            1.0,
        );

        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Orange,
            to_stage: InvolutionStage::Red,
            feature: solar_configuration,
            attractor_field,
        });

        // Update state
        self.current_stage = InvolutionStage::Red;
        self.red_realm = Some(red);

        Ok(())
    }

    /// Create individual entities to transition to Layer 7
    ///
    /// Phase 4 Refactor: Logos Hierarchy Implementation
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Galactic-scale Logoi → Solar-scale Logoi → Individual entities"
    ///
    /// This method creates the complete Logos hierarchy:
    /// 1. Galactic-scale Logoi entities (Orange-Ray) - create galaxy patterns
    /// 2. Solar-scale Logoi entities (Red-Ray) - children of Galactic Logoi
    /// 3. Individual entities (Layer 7) - children of Solar Logoi
    /// 4. Environmental entities (galaxies, stars, planets) - created from 1st Density materials
    /// 5. Hierarchical composition relationships (parent/child)
    /// 6. Entity-environment relationships
    ///
    /// Creates:
    /// - Galactic-scale Logoi entities
    /// - Solar-scale Logoi entities as children of Galactic Logoi
    /// - Individual entities as children of Solar Logoi
    /// - Environmental entities (galaxies, stars, planets)
    /// - Hierarchical composition relationships
    /// - Entity-environment relationships
    fn create_individual_entities(&mut self) -> Result<(), InvolutionError> {
        let red = self
            .red_realm
            .as_ref()
            .ok_or(InvolutionError::StageTransitionFailed {
                stage: InvolutionStage::Red,
                reason: "Red realm not initialized".to_string(),
            })?;

        // ========================================================================
        // Phase 4 Refactor: Logos Hierarchy
        // ========================================================================
        // 1. Create Galactic-scale Logoi entities (Orange-Ray)
        // 2. Create Solar-scale Logoi entities as children of Galactic Logoi (Red-Ray)
        // 3. Create individual entities as children of Solar Logoi (Layer 7)
        // 4. Create environmental entities (galaxies, stars, planets)
        // 5. Create hierarchical composition relationships (many:1)
        // 6. Create entity-environment relationships
        // ========================================================================

        // ========================================================================
        // Step 1: Create Galactic-scale Logoi entities (Orange-Ray)
        // ========================================================================
        // Galactic-scale Logoi configure the spectrum at galactic scale, creating
        // the patterns that galaxies will follow when physical matter forms.
        // ========================================================================

        let mut galactic_logos_ids = Vec::new();
        for i in 0..self.num_galaxies {
            let galactic_logos_id =
                crate::entity_layer7::EntityId::new(format!("galactic-logos-{}", i));
            let galactic_ratio = EntitySpectrumRatio::space_time(1.0, 1.0);
            let galactic_spectrum_config = IndividualSpectrumConfiguration::new(galactic_ratio);

            let mut galactic_logos = SubSubLogos::new(
                galactic_logos_id.clone(),
                crate::entity_layer7::EntityType::GalacticLogos,
                None,       // parent_id (Galactic Logoi have no parent)
                Vec::new(), // composition (will be populated with Solar Logoi)
                None,       // environment_id (Galactic Logoi have no environment)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                galactic_spectrum_config,
            );

            // Galactic Logoi are at a higher density level (they configure spectrum)
            galactic_logos.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::First;
            galactic_logos_ids.push(galactic_logos_id.clone());
            self.entities.push(galactic_logos);
        }

        // ========================================================================
        // Step 2: Create Solar-scale Logoi entities as children of Galactic Logoi
        // ========================================================================
        // Solar-scale Logoi configure the spectrum at solar-system scale and develop
        // specific archetypical mind systems. They are children of Galactic Logoi.
        // ========================================================================

        let mut solar_logos_ids = Vec::new();
        for (galaxy_idx, galactic_logos_id) in galactic_logos_ids.iter().enumerate() {
            for solar_idx in 0..self.num_solar_systems {
                let solar_logos_id = crate::entity_layer7::EntityId::new(format!(
                    "solar-logos-{}-{}",
                    galaxy_idx, solar_idx
                ));
                let solar_ratio = EntitySpectrumRatio::space_time(1.0, 1.0);
                let solar_spectrum_config = IndividualSpectrumConfiguration::new(solar_ratio);

                let mut solar_logos = SubSubLogos::new(
                    solar_logos_id.clone(),
                    crate::entity_layer7::EntityType::SolarLogos,
                    Some(galactic_logos_id.clone()), // parent_id (Galactic Logos)
                    Vec::new(), // composition (will be populated with individual entities)
                    None,       // environment_id (Solar Logoi have no environment)
                    red.orange_realm
                        .yellow_realm
                        .light_love_field
                        .logos
                        .intelligent_infinity
                        .violet_realm
                        .clone(),
                    red.orange_realm
                        .yellow_realm
                        .light_love_field
                        .logos
                        .intelligent_infinity
                        .clone(),
                    red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                    red.orange_realm.yellow_realm.light_love_field.clone(),
                    red.orange_realm.yellow_realm.clone(),
                    red.orange_realm.clone(),
                    red.clone(),
                    solar_spectrum_config,
                );

                // Solar Logoi are at a higher density level (they configure spectrum and archetypical minds)
                solar_logos.evolutionary_attractor.current_density =
                    crate::entity_layer7::layer7::DensityLevel::First;
                solar_logos_ids.push(solar_logos_id.clone());
                self.entities.push(solar_logos);

                // Update Galactic Logos' children to include this Solar Logos
                if let Some(galactic_entity) = self
                    .entities
                    .iter_mut()
                    .find(|e| e.entity_id == *galactic_logos_id)
                {
                    galactic_entity.add_child(solar_logos_id.clone());
                }
            }
        }

        // ========================================================================
        // Step 3: Create Environmental Entities (1st Density materials create environment)
        // ========================================================================
        // Environmental entities provide the substrate for higher densities to exist
        // Without environment, higher densities cannot emerge
        // ========================================================================

        // Create 1 Galaxy environmental entity
        let galaxy_id = crate::entity_layer7::EntityId::new("environment-galaxy-0".to_string());
        let galaxy_ratio = EntitySpectrumRatio::space_time(10.0, 1.0);
        let galaxy_spectrum_config = IndividualSpectrumConfiguration::new(galaxy_ratio);

        let mut galaxy = SubSubLogos::new(
            galaxy_id.clone(),
            crate::entity_layer7::EntityType::Environmental,
            None,       // parent_id
            Vec::new(), // composition (will be populated with stars)
            None,       // environment_id (galaxies have no environment)
            red.orange_realm
                .yellow_realm
                .light_love_field
                .logos
                .intelligent_infinity
                .violet_realm
                .clone(),
            red.orange_realm
                .yellow_realm
                .light_love_field
                .logos
                .intelligent_infinity
                .clone(),
            red.orange_realm.yellow_realm.light_love_field.logos.clone(),
            red.orange_realm.yellow_realm.light_love_field.clone(),
            red.orange_realm.yellow_realm.clone(),
            red.orange_realm.clone(),
            red.clone(),
            galaxy_spectrum_config,
        );

        galaxy.evolutionary_attractor.current_density =
            crate::entity_layer7::layer7::DensityLevel::First;
        let _ = galaxy.create_physical_entity();
        self.entities.push(galaxy);

        // Create 3 Star environmental entities (in the galaxy)
        let mut star_ids = Vec::new();
        for i in 0..3 {
            let star_id = crate::entity_layer7::EntityId::new(format!("environment-star-{}", i));
            let star_ratio = EntitySpectrumRatio::space_time(8.0 + (i as f64), 1.0);
            let star_spectrum_config = IndividualSpectrumConfiguration::new(star_ratio);

            let mut star = SubSubLogos::new(
                star_id.clone(),
                crate::entity_layer7::EntityType::Environmental,
                Some(galaxy_id.clone()), // parent_id (in the galaxy)
                Vec::new(),              // composition (will be populated with planets)
                Some(galaxy_id.clone()), // environment_id (in the galaxy)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                star_spectrum_config,
            );

            star.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::First;
            let _ = star.create_physical_entity();
            star_ids.push(star_id.clone());
            self.entities.push(star);
        }

        // Update galaxy's composition to include stars
        if let Some(galaxy_entity) = self.entities.iter_mut().find(|e| e.entity_id == galaxy_id) {
            galaxy_entity.composition = star_ids.clone();
            galaxy_entity.children = star_ids.clone();
        }

        // Create 6 Planet environmental entities (2 per star)
        let mut planet_ids = Vec::new();
        for (star_idx, star_id) in star_ids.iter().enumerate() {
            for planet_idx in 0..2 {
                let planet_id = crate::entity_layer7::EntityId::new(format!(
                    "environment-planet-{}-{}",
                    star_idx, planet_idx
                ));
                let planet_ratio =
                    EntitySpectrumRatio::space_time(5.0 + (planet_idx as f64 * 0.5), 1.0);
                let planet_spectrum_config = IndividualSpectrumConfiguration::new(planet_ratio);

                let mut planet = SubSubLogos::new(
                    planet_id.clone(),
                    crate::entity_layer7::EntityType::Environmental,
                    Some(star_id.clone()), // parent_id (in the star system)
                    Vec::new(), // composition (planets are not composed of other entities in this simple model)
                    Some(star_id.clone()), // environment_id (in the star system)
                    red.orange_realm
                        .yellow_realm
                        .light_love_field
                        .logos
                        .intelligent_infinity
                        .violet_realm
                        .clone(),
                    red.orange_realm
                        .yellow_realm
                        .light_love_field
                        .logos
                        .intelligent_infinity
                        .clone(),
                    red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                    red.orange_realm.yellow_realm.light_love_field.clone(),
                    red.orange_realm.yellow_realm.clone(),
                    red.orange_realm.clone(),
                    red.clone(),
                    planet_spectrum_config,
                );

                planet.evolutionary_attractor.current_density =
                    crate::entity_layer7::layer7::DensityLevel::First;
                let _ = planet.create_physical_entity();
                planet_ids.push(planet_id.clone());
                self.entities.push(planet);
            }
        }

        // Update stars' composition to include planets
        for (star_idx, star_id) in star_ids.iter().enumerate() {
            let star_planets: Vec<crate::entity_layer7::EntityId> = planet_ids
                .iter()
                .filter(|pid| pid.uuid.contains(&format!("{}-", star_idx)))
                .cloned()
                .collect();

            if let Some(star_entity) = self.entities.iter_mut().find(|e| e.entity_id == *star_id) {
                star_entity.composition = star_planets.clone();
                star_entity.children = star_planets;
            }
        }

        // ========================================================================
        // Step 4: Create Individual Entities as children of Solar Logoi
        // ========================================================================
        // Material Complexity Hierarchy:
        // - 80 Quantum particles → form 20 Atoms
        // - 20 Atoms → form 10 Molecules
        // - 10 Molecules → form 5 Cells
        // - 5 Cells → form 2 Organisms
        // - 2 Organisms → support 1 Self-aware being
        // ========================================================================

        let mut quantum_particle_ids = Vec::new();
        let mut atom_ids = Vec::new();
        let mut molecule_ids = Vec::new();
        let mut cell_ids = Vec::new();
        let mut organism_ids = Vec::new();

        // Assign individual entities to the first Solar Logos for simplicity
        let parent_solar_logos_id = solar_logos_ids.first().cloned();
        let environment_id = planet_ids.first().cloned();

        // Create 80 Quantum particles (1st Density)
        for i in 0..80 {
            let quantum_id =
                crate::entity_layer7::EntityId::new(format!("individual-quantum-{}", i));
            let quantum_ratio = EntitySpectrumRatio::space_time(1.0 + (i as f64 * 0.001), 1.0);
            let quantum_spectrum_config = IndividualSpectrumConfiguration::new(quantum_ratio);

            let mut quantum = SubSubLogos::new(
                quantum_id.clone(),
                crate::entity_layer7::EntityType::Individual,
                parent_solar_logos_id.clone(), // parent_id (Solar Logos)
                Vec::new(),                    // composition (quantum particles are fundamental)
                environment_id.clone(),        // environment_id (on the first planet)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                quantum_spectrum_config,
            );

            quantum.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::First;
            let _ = quantum.create_physical_entity();
            quantum_particle_ids.push(quantum_id.clone());
            self.entities.push(quantum);
        }

        // Create 20 Atoms (1st Density) - each composed of 4 quantum particles
        for i in 0..20 {
            let atom_id = crate::entity_layer7::EntityId::new(format!("individual-atomic-{}", i));
            let atom_ratio = EntitySpectrumRatio::space_time(1.5 + (i as f64 * 0.01), 1.0);
            let atom_spectrum_config = IndividualSpectrumConfiguration::new(atom_ratio);

            // Assign 4 quantum particles to this atom
            let start_idx = i * 4;
            let atom_quantum_ids: Vec<crate::entity_layer7::EntityId> = quantum_particle_ids
                [start_idx..(start_idx + 4).min(quantum_particle_ids.len())]
                .to_vec();

            let mut atom = SubSubLogos::new(
                atom_id.clone(),
                crate::entity_layer7::EntityType::Individual,
                parent_solar_logos_id.clone(), // parent_id (Solar Logos)
                atom_quantum_ids.clone(),      // composition (composed of quantum particles)
                environment_id.clone(),        // environment_id (on the first planet)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                atom_spectrum_config,
            );

            atom.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::First;
            let _ = atom.create_physical_entity();
            atom_ids.push(atom_id.clone());
            self.entities.push(atom);

            // Update quantum particles' parent_id to point to this atom
            for quantum_id in &atom_quantum_ids {
                if let Some(quantum_entity) = self
                    .entities
                    .iter_mut()
                    .find(|e| e.entity_id == *quantum_id)
                {
                    quantum_entity.parent_id = Some(atom_id.clone());
                }
            }
        }

        // Create 10 Molecules (1st Density) - each composed of 2 atoms
        for i in 0..10 {
            let molecule_id =
                crate::entity_layer7::EntityId::new(format!("individual-molecular-{}", i));
            let molecule_ratio = EntitySpectrumRatio::space_time(2.0 + (i as f64 * 0.01), 1.0);
            let molecule_spectrum_config = IndividualSpectrumConfiguration::new(molecule_ratio);

            // Assign 2 atoms to this molecule
            let start_idx = i * 2;
            let molecule_atom_ids: Vec<crate::entity_layer7::EntityId> =
                atom_ids[start_idx..(start_idx + 2).min(atom_ids.len())].to_vec();

            let mut molecule = SubSubLogos::new(
                molecule_id.clone(),
                crate::entity_layer7::EntityType::Individual,
                parent_solar_logos_id.clone(), // parent_id (Solar Logos)
                molecule_atom_ids.clone(),     // composition (composed of atoms)
                environment_id.clone(),        // environment_id (on the first planet)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                molecule_spectrum_config,
            );

            molecule.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::First;
            let _ = molecule.create_physical_entity();
            molecule_ids.push(molecule_id.clone());
            self.entities.push(molecule);

            // Update atoms' parent_id to point to this molecule
            for atom_id in &molecule_atom_ids {
                if let Some(atom_entity) =
                    self.entities.iter_mut().find(|e| e.entity_id == *atom_id)
                {
                    atom_entity.parent_id = Some(molecule_id.clone());
                }
            }
        }

        // Create 5 Cells (2nd Density) - each composed of 2 molecules
        for i in 0..5 {
            let cell_id = crate::entity_layer7::EntityId::new(format!("individual-cellular-{}", i));
            let cell_ratio = EntitySpectrumRatio::space_time(2.5 + (i as f64 * 0.01), 1.0);
            let cell_spectrum_config = IndividualSpectrumConfiguration::new(cell_ratio);

            // Assign 2 molecules to this cell
            let start_idx = i * 2;
            let cell_molecule_ids: Vec<crate::entity_layer7::EntityId> =
                molecule_ids[start_idx..(start_idx + 2).min(molecule_ids.len())].to_vec();

            let mut cell = SubSubLogos::new(
                cell_id.clone(),
                crate::entity_layer7::EntityType::Individual,
                parent_solar_logos_id.clone(), // parent_id (Solar Logos)
                cell_molecule_ids.clone(),     // composition (composed of molecules)
                environment_id.clone(),        // environment_id (on the first planet)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                cell_spectrum_config,
            );

            cell.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::Second;
            let _ = cell.create_physical_entity();
            cell_ids.push(cell_id.clone());
            self.entities.push(cell);

            // Update molecules' parent_id to point to this cell
            for molecule_id in &cell_molecule_ids {
                if let Some(molecule_entity) = self
                    .entities
                    .iter_mut()
                    .find(|e| e.entity_id == *molecule_id)
                {
                    molecule_entity.parent_id = Some(cell_id.clone());
                }
            }
        }

        // Create 2 Organisms (2nd Density) - each composed of 2-3 cells
        for i in 0..2 {
            let organism_id =
                crate::entity_layer7::EntityId::new(format!("individual-organism-{}", i));
            let organism_ratio = EntitySpectrumRatio::space_time(3.0 + (i as f64 * 0.01), 1.0);
            let organism_spectrum_config = IndividualSpectrumConfiguration::new(organism_ratio);

            // Assign 2-3 cells to this organism
            let start_idx = i * 2;
            let end_idx = (start_idx + 3).min(cell_ids.len());
            let organism_cell_ids: Vec<crate::entity_layer7::EntityId> =
                cell_ids[start_idx..end_idx].to_vec();

            let mut organism = SubSubLogos::new(
                organism_id.clone(),
                crate::entity_layer7::EntityType::Individual,
                parent_solar_logos_id.clone(), // parent_id (Solar Logos)
                organism_cell_ids.clone(),     // composition (composed of cells)
                environment_id.clone(),        // environment_id (on the first planet)
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .violet_realm
                    .clone(),
                red.orange_realm
                    .yellow_realm
                    .light_love_field
                    .logos
                    .intelligent_infinity
                    .clone(),
                red.orange_realm.yellow_realm.light_love_field.logos.clone(),
                red.orange_realm.yellow_realm.light_love_field.clone(),
                red.orange_realm.yellow_realm.clone(),
                red.orange_realm.clone(),
                red.clone(),
                organism_spectrum_config,
            );

            organism.evolutionary_attractor.current_density =
                crate::entity_layer7::layer7::DensityLevel::Second;
            let _ = organism.create_physical_entity();
            organism_ids.push(organism_id.clone());
            self.entities.push(organism);

            // Update cells' parent_id to point to this organism
            for cell_id in &organism_cell_ids {
                if let Some(cell_entity) =
                    self.entities.iter_mut().find(|e| e.entity_id == *cell_id)
                {
                    cell_entity.parent_id = Some(organism_id.clone());
                }
            }
        }

        // Create 1 Self-aware being (3rd Density) - composed of 2 organisms
        let being_id = crate::entity_layer7::EntityId::new("individual-being-0".to_string());
        let being_ratio = EntitySpectrumRatio::space_time(4.0, 1.0);
        let being_spectrum_config = IndividualSpectrumConfiguration::new(being_ratio);

        let mut being = SubSubLogos::new(
            being_id.clone(),
            crate::entity_layer7::EntityType::Individual,
            parent_solar_logos_id.clone(), // parent_id (Solar Logos)
            organism_ids.clone(),          // composition (composed of organisms)
            environment_id.clone(),        // environment_id (on the first planet)
            red.orange_realm
                .yellow_realm
                .light_love_field
                .logos
                .intelligent_infinity
                .violet_realm
                .clone(),
            red.orange_realm
                .yellow_realm
                .light_love_field
                .logos
                .intelligent_infinity
                .clone(),
            red.orange_realm.yellow_realm.light_love_field.logos.clone(),
            red.orange_realm.yellow_realm.light_love_field.clone(),
            red.orange_realm.yellow_realm.clone(),
            red.orange_realm.clone(),
            red.clone(),
            being_spectrum_config,
        );

        being.evolutionary_attractor.current_density =
            crate::entity_layer7::layer7::DensityLevel::Third;
        let _ = being.create_physical_entity();
        self.entities.push(being);

        // Update organisms' parent_id to point to this being
        for organism_id in &organism_ids {
            if let Some(organism_entity) = self
                .entities
                .iter_mut()
                .find(|e| e.entity_id == *organism_id)
            {
                organism_entity.parent_id = Some(being_id.clone());
            }
        }

        // Update Solar Logos' children to include individual entities
        if let Some(solar_logos_id) = parent_solar_logos_id {
            if let Some(solar_entity) = self
                .entities
                .iter_mut()
                .find(|e| e.entity_id == solar_logos_id)
            {
                // Add all individual entities as children of this Solar Logos
                solar_entity.children.extend(quantum_particle_ids.clone());
                solar_entity.children.extend(atom_ids.clone());
                solar_entity.children.extend(molecule_ids.clone());
                solar_entity.children.extend(cell_ids.clone());
                solar_entity.children.extend(organism_ids.clone());
                solar_entity.children.push(being_id.clone());
            }
        }

        // ========================================================================
        // Record stage transition
        // ========================================================================
        let entity_creation = Feature::new(
            "Phase 4 Refactor: Logos Hierarchy",
            "Galactic-scale Logoi → Solar-scale Logoi → Individual entities. \
             Entities at all scales exist simultaneously with hierarchical composition relationships. \
             Environmental entities (galaxies, stars, planets) are created from 1st Density materials \
             and provide the substrate for higher densities to exist.",
            1.0,
        );

        let attractor_field = AttractorField::new(
            "Evolutionary Attractor-Fields",
            1.0,
            "Individual entities progress within their density (not to a different density). \
             Collective system emergence tracks when new densities emerge in the collective system.",
        );

        self.stage_transitions.push(StageTransitionRecord {
            from_stage: InvolutionStage::Red,
            to_stage: InvolutionStage::Layer7,
            feature: entity_creation,
            attractor_field: attractor_field.clone(),
        });

        self.attractor_fields.push(attractor_field);

        // Update state
        self.current_stage = InvolutionStage::Layer7;

        Ok(())
    }

    /// Get the current stage
    pub fn current_stage(&self) -> InvolutionStage {
        self.current_stage
    }

    /// Get the number of entities created
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Get the number of attractor-fields created
    pub fn attractor_field_count(&self) -> usize {
        self.attractor_fields.len()
    }

    /// Get the number of stage transitions
    pub fn stage_transition_count(&self) -> usize {
        self.stage_transitions.len()
    }
}

// ============================================================================
// MIGRATION 10: Involution-Evolution Engine
// ============================================================================
//
// This section implements the simultaneous involution and evolution process.
//
// Key Principles:
// - Involution and evolution happen simultaneously (not sequentially)
// - Balance shifts based on density (lower = more involution, higher = more evolution)
// - Both processes are continuous and never-ending
// - Entities exhibit both specificity and awareness
//
// From involution_evolution.rs (Migrated February 5, 2026)
//
// Note: This is a COMPLEMENTARY system to the involution_sequence above:
// - involution_sequence: Initial creation of entities (ONE-TIME process from source)
// - involution_evolution: Ongoing simultaneous involution-evolution (CONTINUOUS process)
// ============================================================================

use crate::evolution_density_octave::density_octave::Density;
use crate::types::{Float, HolonID};
use std::collections::HashMap;

// ============================================================================
// Flow Direction
// ============================================================================

/// Flow Direction
///
/// Represents the direction of the involution-evolution flow.
/// Note: Both involution and evolution happen simultaneously.
/// This represents the dominant direction at a given moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowDirection {
    /// Spirit descending into matter (becoming more specific)
    Involution,
    /// Matter ascending to spirit (becoming more aware)
    Evolution,
    /// Balanced flow (equal involution and evolution)
    Balanced,
}

impl FlowDirection {
    /// Convert to string representation
    pub fn as_str(&self) -> &str {
        match self {
            FlowDirection::Involution => "Involution",
            FlowDirection::Evolution => "Evolution",
            FlowDirection::Balanced => "Balanced",
        }
    }

    /// Get the numeric value for calculations
    /// Returns -1.0 for involution, 1.0 for evolution, 0.0 for balanced
    pub fn as_float(&self) -> Float {
        match self {
            FlowDirection::Involution => -1.0,
            FlowDirection::Evolution => 1.0,
            FlowDirection::Balanced => 0.0,
        }
    }

    /// Create from balance value
    /// Returns Involution if balance < -0.3, Evolution if balance > 0.3, Balanced otherwise
    pub fn from_balance(balance: Float) -> Self {
        if balance < -0.3 {
            FlowDirection::Involution
        } else if balance > 0.3 {
            FlowDirection::Evolution
        } else {
            FlowDirection::Balanced
        }
    }
}

// ============================================================================
// Entity State for Involution-Evolution
// ============================================================================

/// Entity State for Involution-Evolution Processing
///
/// Tracks the entity's current state in the involution-evolution process.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityInvolutionEvolutionState {
    /// Entity ID
    pub entity_id: HolonID,

    // Involution metrics
    /// How deep into matter (0.0 to 1.0)
    pub involution_depth: Float,
    /// How specific/individuated (0.0 to 1.0)
    pub specificity: Float,
    /// Current veil thickness (0.0 to 1.0)
    pub veil_thickness: Float,

    // Evolution metrics
    /// How high in awareness (0.0 to 1.0)
    pub evolution_height: Float,
    /// How aware of unity (0.0 to 1.0)
    pub unity_awareness: Float,

    // Current density
    pub density: Density,

    // Polarity (affects involution-evolution balance)
    /// Polarity intensity (0.0 to 1.0)
    pub polarization_intensity: Float,
    /// Polarity direction (-1.0 STS to 1.0 STO)
    pub polarization_direction: Float,
}

impl EntityInvolutionEvolutionState {
    /// Create a new entity state
    pub fn new(entity_id: HolonID, density: Density) -> Self {
        Self {
            entity_id,
            involution_depth: 0.0,
            specificity: 0.0,
            veil_thickness: density.base_veil_thickness(),
            evolution_height: 0.0,
            unity_awareness: 0.0,
            density,
            polarization_intensity: 0.0,
            polarization_direction: 0.0,
        }
    }

    /// Create a new entity state with custom values
    pub fn with_values(
        entity_id: HolonID,
        density: Density,
        involution_depth: Float,
        specificity: Float,
        veil_thickness: Float,
        evolution_height: Float,
        unity_awareness: Float,
        polarization_intensity: Float,
        polarization_direction: Float,
    ) -> Self {
        Self {
            entity_id,
            involution_depth: involution_depth.clamp(0.0, 1.0),
            specificity: specificity.clamp(0.0, 1.0),
            veil_thickness: veil_thickness.clamp(0.0, 1.0),
            evolution_height: evolution_height.clamp(0.0, 1.0),
            unity_awareness: unity_awareness.clamp(0.0, 1.0),
            density,
            polarization_intensity: polarization_intensity.clamp(0.0, 1.0),
            polarization_direction: polarization_direction.clamp(-1.0, 1.0),
        }
    }

    /// Increase specificity (involution)
    pub fn increase_specificity(&mut self, amount: Float) {
        self.specificity = (self.specificity + amount).clamp(0.0, 1.0);
        // Higher specificity increases involution depth
        self.involution_depth = (self.involution_depth + amount * 0.5).clamp(0.0, 1.0);
    }

    /// Increase individuation (involution)
    pub fn increase_individuation(&mut self, amount: Float) {
        self.specificity = (self.specificity + amount).clamp(0.0, 1.0);
        self.involution_depth = (self.involution_depth + amount * 0.5).clamp(0.0, 1.0);
    }

    /// Thicken veil (involution)
    pub fn thicken_veil(&mut self, amount: Float) {
        self.veil_thickness = (self.veil_thickness + amount).clamp(0.0, 1.0);
    }

    /// Increase awareness (evolution)
    pub fn increase_awareness(&mut self, amount: Float) {
        self.evolution_height = (self.evolution_height + amount).clamp(0.0, 1.0);
        // Higher awareness increases unity awareness
        self.unity_awareness = (self.unity_awareness + amount * 0.5).clamp(0.0, 1.0);
    }

    /// Increase unity awareness (evolution)
    pub fn increase_unity_awareness(&mut self, amount: Float) {
        self.unity_awareness = (self.unity_awareness + amount).clamp(0.0, 1.0);
        // Higher unity awareness increases evolution height
        self.evolution_height = (self.evolution_height + amount * 0.5).clamp(0.0, 1.0);
    }

    /// Thin veil (evolution)
    pub fn thin_veil(&mut self, amount: Float) {
        self.veil_thickness = (self.veil_thickness - amount).clamp(0.0, 1.0);
    }

    /// Set density
    pub fn set_density(&mut self, density: Density) {
        self.density = density;
        // Update base veil thickness
        self.veil_thickness = density.base_veil_thickness();
    }

    /// Calculate overall involution-evolution balance
    /// Returns -1.0 (pure involution) to 1.0 (pure evolution)
    ///
    /// Note: Veil thickness is NOT included in balance calculation to avoid feedback loops.
    /// The veil is a result of involution/evolution, not a cause of it.
    pub fn calculate_balance(&self) -> Float {
        let involution_score = (self.involution_depth + self.specificity) / 2.0;
        let evolution_score = (self.evolution_height + self.unity_awareness) / 2.0;
        evolution_score - involution_score
    }

    /// Get flow direction
    pub fn get_flow_direction(&self) -> FlowDirection {
        FlowDirection::from_balance(self.calculate_balance())
    }
}

// ============================================================================
// Involution-Evolution Engine
// ============================================================================

/// Involution-Evolution Engine
///
/// Manages the simultaneous involution and evolution process across all entities.
#[derive(Debug, Clone)]
pub struct InvolutionEvolutionEngine {
    /// Current flow direction
    pub flow_direction: FlowDirection,
    /// Flow intensity (0.0 to 1.0)
    pub flow_intensity: Float,

    // Global metrics
    /// Average involution depth across all entities
    pub involution_depth: Float,
    /// Average specificity across all entities
    pub specificity: Float,
    /// Average evolution height across all entities
    pub evolution_height: Float,
    /// Average unity awareness across all entities
    pub unity_awareness: Float,

    // Balance
    /// Involution-evolution balance (-1.0 pure involution to 1.0 pure evolution)
    pub involution_evolution_balance: Float,

    // Entity states
    /// Per-entity involution-evolution state
    pub entity_states: HashMap<HolonID, EntityInvolutionEvolutionState>,

    // Configuration
    /// Base flow intensity
    pub base_flow_intensity: Float,
    /// Balance sensitivity (how quickly balance shifts)
    pub balance_sensitivity: Float,
}

impl InvolutionEvolutionEngine {
    /// Create a new involution-evolution engine
    pub fn new() -> Self {
        Self {
            flow_direction: FlowDirection::Balanced,
            flow_intensity: 0.5,
            involution_depth: 0.0,
            specificity: 0.0,
            evolution_height: 0.0,
            unity_awareness: 0.0,
            involution_evolution_balance: 0.0,
            entity_states: HashMap::new(),
            base_flow_intensity: 0.5,
            balance_sensitivity: 0.1,
        }
    }

    /// Create a new involution-evolution engine with custom settings
    pub fn with_settings(base_flow_intensity: Float, balance_sensitivity: Float) -> Self {
        Self {
            flow_direction: FlowDirection::Balanced,
            flow_intensity: base_flow_intensity,
            involution_depth: 0.0,
            specificity: 0.0,
            evolution_height: 0.0,
            unity_awareness: 0.0,
            involution_evolution_balance: 0.0,
            entity_states: HashMap::new(),
            base_flow_intensity,
            balance_sensitivity,
        }
    }

    /// Register an entity for involution-evolution processing
    pub fn register_entity(&mut self, entity_id: HolonID, density: Density) {
        let state = EntityInvolutionEvolutionState::new(entity_id, density);
        self.entity_states.insert(entity_id, state);
        self.recalculate_global_metrics();
    }

    /// Register an entity with custom state
    pub fn register_entity_with_state(&mut self, state: EntityInvolutionEvolutionState) {
        self.entity_states.insert(state.entity_id, state);
        self.recalculate_global_metrics();
    }

    /// Process entity through involution-evolution
    ///
    /// Both involution and evolution happen simultaneously.
    /// The balance shifts based on entity's density and polarization.
    pub fn process(&mut self, entity_id: HolonID) -> Option<&EntityInvolutionEvolutionState> {
        let state = self.entity_states.get_mut(&entity_id)?;

        // Calculate balance based on density
        // Lower densities: More involution (becoming specific)
        // Higher densities: More evolution (becoming aware)
        let density_factor = state.density.as_float();
        let density_balance = (density_factor - 0.5) * 2.0; // -1.0 to 1.0

        // Modify balance based on polarization
        // STS polarization tends toward involution (more self-focused)
        // STO polarization tends toward evolution (more unity-focused)
        let polarization_modifier =
            state.polarization_direction * state.polarization_intensity * 0.3;

        // Calculate target balance
        let target_balance = density_balance + polarization_modifier;
        let target_balance = target_balance.clamp(-1.0, 1.0);

        // Shift balance gradually
        let current_balance = state.calculate_balance();
        let balance_shift = (target_balance - current_balance) * self.balance_sensitivity;
        self.involution_evolution_balance = (current_balance + balance_shift).clamp(-1.0, 1.0);

        // Update flow direction
        self.flow_direction = FlowDirection::from_balance(self.involution_evolution_balance);

        // Calculate flow intensity
        // Higher intensity = faster involution-evolution
        self.flow_intensity =
            self.base_flow_intensity * (1.0 - self.involution_evolution_balance.abs() * 0.5);

        // Apply involution (if balance < 0)
        if self.involution_evolution_balance < 0.0 {
            let involution_strength = self.involution_evolution_balance.abs() * self.flow_intensity;
            state.increase_specificity(involution_strength * 0.1);
            state.increase_individuation(involution_strength * 0.1);
            state.thicken_veil(involution_strength * 0.05);
        }

        // Apply evolution (if balance >= 0)
        if self.involution_evolution_balance >= 0.0 {
            let evolution_strength =
                self.involution_evolution_balance.max(0.1) * self.flow_intensity;
            state.increase_awareness(evolution_strength * 0.1);
            state.increase_unity_awareness(evolution_strength * 0.1);
            state.thin_veil(evolution_strength * 0.05);
        }

        // Both processes happen simultaneously (even when balanced)
        // Small amounts of both involution and evolution always occur
        let base_involution = self.flow_intensity * 0.02; // 2% base involution
        let base_evolution = self.flow_intensity * 0.02; // 2% base evolution

        state.increase_specificity(base_involution);
        state.increase_awareness(base_evolution);

        // Recalculate global metrics
        self.recalculate_global_metrics();

        self.entity_states.get(&entity_id)
    }

    /// Process all entities
    pub fn process_all(&mut self) {
        let entity_ids: Vec<HolonID> = self.entity_states.keys().copied().collect();

        for entity_id in entity_ids {
            self.process(entity_id);
        }
    }

    /// Recalculate global metrics from entity states
    fn recalculate_global_metrics(&mut self) {
        if self.entity_states.is_empty() {
            self.involution_depth = 0.0;
            self.specificity = 0.0;
            self.evolution_height = 0.0;
            self.unity_awareness = 0.0;
            return;
        }

        let count = self.entity_states.len() as Float;

        let involution_depth_sum: Float = self
            .entity_states
            .values()
            .map(|s| s.involution_depth)
            .sum();
        let specificity_sum: Float = self.entity_states.values().map(|s| s.specificity).sum();
        let evolution_height_sum: Float = self
            .entity_states
            .values()
            .map(|s| s.evolution_height)
            .sum();
        let unity_awareness_sum: Float =
            self.entity_states.values().map(|s| s.unity_awareness).sum();

        self.involution_depth = involution_depth_sum / count;
        self.specificity = specificity_sum / count;
        self.evolution_height = evolution_height_sum / count;
        self.unity_awareness = unity_awareness_sum / count;
    }

    /// Get entity state
    pub fn get_entity_state(&self, entity_id: HolonID) -> Option<&EntityInvolutionEvolutionState> {
        self.entity_states.get(&entity_id)
    }

    /// Get mutable entity state
    pub fn get_entity_state_mut(
        &mut self,
        entity_id: HolonID,
    ) -> Option<&mut EntityInvolutionEvolutionState> {
        self.entity_states.get_mut(&entity_id)
    }

    /// Update entity polarization
    pub fn update_entity_polarization(
        &mut self,
        entity_id: HolonID,
        intensity: Float,
        direction: Float,
    ) {
        if let Some(state) = self.entity_states.get_mut(&entity_id) {
            state.polarization_intensity = intensity.clamp(0.0, 1.0);
            state.polarization_direction = direction.clamp(-1.0, 1.0);
        }
    }

    /// Update entity density
    pub fn update_entity_density(&mut self, entity_id: HolonID, density: Density) {
        if let Some(state) = self.entity_states.get_mut(&entity_id) {
            state.set_density(density);
            self.recalculate_global_metrics();
        }
    }

    /// Get number of registered entities
    pub fn get_entity_count(&self) -> usize {
        self.entity_states.len()
    }

    /// Clear all entity states
    pub fn clear_entities(&mut self) {
        self.entity_states.clear();
        self.recalculate_global_metrics();
    }

    /// Reset global metrics
    pub fn reset_metrics(&mut self) {
        self.involution_depth = 0.0;
        self.specificity = 0.0;
        self.evolution_height = 0.0;
        self.unity_awareness = 0.0;
        self.involution_evolution_balance = 0.0;
        self.flow_direction = FlowDirection::Balanced;
        self.flow_intensity = self.base_flow_intensity;
    }

    /// Get engine statistics
    pub fn get_stats(&self) -> InvolutionEvolutionStats {
        InvolutionEvolutionStats {
            entity_count: self.get_entity_count(),
            flow_direction: self.flow_direction,
            flow_intensity: self.flow_intensity,
            involution_depth: self.involution_depth,
            specificity: self.specificity,
            evolution_height: self.evolution_height,
            unity_awareness: self.unity_awareness,
            involution_evolution_balance: self.involution_evolution_balance,
        }
    }
}

// ============================================================================
// Statistics
// ============================================================================

/// Involution-Evolution Statistics
///
/// Snapshot of the engine's current state.
#[derive(Debug, Clone, PartialEq)]
pub struct InvolutionEvolutionStats {
    pub entity_count: usize,
    pub flow_direction: FlowDirection,
    pub flow_intensity: Float,
    pub involution_depth: Float,
    pub specificity: Float,
    pub evolution_height: Float,
    pub unity_awareness: Float,
    pub involution_evolution_balance: Float,
}

// ============================================================================
// Density Extension
// ============================================================================

/// Extension for Density to provide base veil thickness
pub trait DensityExt {
    /// Get base veil thickness for this density
    fn base_veil_thickness(&self) -> Float;

    /// Get density as float (0.0 to 1.0)
    fn as_float(&self) -> Float;
}

impl DensityExt for Density {
    fn base_veil_thickness(&self) -> Float {
        match self {
            Density::First(_) => 0.0,  // No veil (elemental consciousness)
            Density::Second(_) => 0.2, // Thin veil (plant/animal)
            Density::Third => 0.8,     // Thick veil (self-conscious, choice point)
            Density::Fourth => 0.6,    // Medium-thick (understanding love)
            Density::Fifth => 0.4,     // Medium (wisdom density)
            Density::Sixth => 0.2,     // Thin (unity density)
            Density::Seventh => 0.0,   // No veil (completion, gateway)
            Density::Eighth => 0.0,    // No veil (complete reintegration)
        }
    }

    fn as_float(&self) -> Float {
        match self {
            Density::First(_) => 0.0,
            Density::Second(_) => 0.1666667, // 1/6
            Density::Third => 0.3333333,     // 2/6
            Density::Fourth => 0.5,          // 3/6
            Density::Fifth => 0.6666667,     // 4/6
            Density::Sixth => 0.8333333,     // 5/6
            Density::Seventh => 1.0,         // 6/6
            Density::Eighth => 1.0,          // 7/7
        }
    }
}

// ============================================================================
// MIGRATION 10: Tests
// ============================================================================

#[cfg(test)]
mod involution_evolution_tests {
    use super::*;
    use crate::evolution_density_octave::density_octave::{Density1SubLevel, Density2SubLevel};

    // -------------------------------------------------------------------------
    // FlowDirection Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_flow_direction_as_str() {
        assert_eq!(FlowDirection::Involution.as_str(), "Involution");
        assert_eq!(FlowDirection::Evolution.as_str(), "Evolution");
        assert_eq!(FlowDirection::Balanced.as_str(), "Balanced");
    }

    #[test]
    fn test_density_base_veil_thickness() {
        assert_eq!(
            Density::First(Density1SubLevel::Quantum).base_veil_thickness(),
            0.0
        );
        assert_eq!(
            Density::Second(Density2SubLevel::SimpleLife).base_veil_thickness(),
            0.2
        );
        assert_eq!(Density::Third.base_veil_thickness(), 0.8);
        assert_eq!(Density::Fourth.base_veil_thickness(), 0.6);
        assert_eq!(Density::Fifth.base_veil_thickness(), 0.4);
        assert_eq!(Density::Sixth.base_veil_thickness(), 0.2);
        assert_eq!(Density::Seventh.base_veil_thickness(), 0.0);
    }

    #[test]
    fn test_density_as_float() {
        assert_eq!(Density::First(Density1SubLevel::Quantum).as_float(), 0.0);
        assert!(
            (Density::Second(Density2SubLevel::SimpleLife).as_float() - 0.1666667).abs() < 0.0001
        );
        assert!((Density::Third.as_float() - 0.3333333).abs() < 0.0001);
        assert_eq!(Density::Fourth.as_float(), 0.5);
        assert!((Density::Fifth.as_float() - 0.6666667).abs() < 0.0001);
        assert!((Density::Sixth.as_float() - 0.8333333).abs() < 0.0001);
        assert_eq!(Density::Seventh.as_float(), 1.0);
    }

    #[test]
    fn test_flow_direction_from_balance() {
        assert_eq!(FlowDirection::from_balance(-0.5), FlowDirection::Involution);
        assert_eq!(FlowDirection::from_balance(0.5), FlowDirection::Evolution);
        assert_eq!(FlowDirection::from_balance(0.0), FlowDirection::Balanced);
        assert_eq!(FlowDirection::from_balance(0.2), FlowDirection::Balanced);
        assert_eq!(FlowDirection::from_balance(-0.2), FlowDirection::Balanced);
    }

    // -------------------------------------------------------------------------
    // EntityInvolutionEvolutionState Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_entity_state_creation() {
        let entity_id = 1;
        let state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        assert_eq!(state.entity_id, entity_id);
        assert_eq!(state.density, Density::Third);
        assert_eq!(state.involution_depth, 0.0);
        assert_eq!(state.specificity, 0.0);
        assert_eq!(state.veil_thickness, 0.8); // D3 base veil thickness
        assert_eq!(state.evolution_height, 0.0);
        assert_eq!(state.unity_awareness, 0.0);
    }

    #[test]
    fn test_entity_state_with_values() {
        let entity_id = 1;
        let state = EntityInvolutionEvolutionState::with_values(
            entity_id,
            Density::Third,
            0.5,
            0.7,
            0.6,
            0.4,
            0.3,
            0.8,
            0.9,
        );

        assert_eq!(state.entity_id, entity_id);
        assert_eq!(state.density, Density::Third);
        assert_eq!(state.involution_depth, 0.5);
        assert_eq!(state.specificity, 0.7);
        assert_eq!(state.veil_thickness, 0.6);
        assert_eq!(state.evolution_height, 0.4);
        assert_eq!(state.unity_awareness, 0.3);
        assert_eq!(state.polarization_intensity, 0.8);
        assert_eq!(state.polarization_direction, 0.9);
    }

    #[test]
    fn test_entity_state_increase_specificity() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        state.increase_specificity(0.3);

        assert_eq!(state.specificity, 0.3);
        assert_eq!(state.involution_depth, 0.15); // 0.3 * 0.5
    }

    #[test]
    fn test_entity_state_get_flow_direction() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        // Initially balanced (D3 has thick veil, but no involution/evolution yet)
        let initial_balance = state.calculate_balance();
        let initial_direction = state.get_flow_direction();
        assert_eq!(
            initial_direction,
            FlowDirection::Balanced,
            "Initial direction should be Balanced, got {:?} (balance: {})",
            initial_direction,
            initial_balance
        );

        // Increase involution significantly
        state.increase_specificity(0.9);
        state.increase_individuation(0.9);
        state.thicken_veil(0.2); // Thicken veil to help involution
        let involution_balance = state.calculate_balance();
        let involution_direction = state.get_flow_direction();
        assert_eq!(
            involution_direction,
            FlowDirection::Involution,
            "After involution, direction should be Involution, got {:?} (balance: {})",
            involution_direction,
            involution_balance
        );

        // Reset and test evolution
        let mut state2 = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        // Increase evolution significantly
        state2.increase_awareness(1.0);
        state2.increase_unity_awareness(1.0);
        state2.thin_veil(0.5); // Thin veil significantly to help evolution
        let evolution_balance = state2.calculate_balance();
        let evolution_direction = state2.get_flow_direction();
        assert_eq!(
            evolution_direction,
            FlowDirection::Evolution,
            "After evolution, direction should be Evolution, got {:?} (balance: {})",
            evolution_direction,
            evolution_balance
        );
    }

    #[test]
    fn test_entity_state_thicken_veil() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        state.thicken_veil(0.1);

        assert_eq!(state.veil_thickness, 0.9); // 0.8 + 0.1
    }

    #[test]
    fn test_entity_state_increase_awareness() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        state.increase_awareness(0.3);

        assert_eq!(state.evolution_height, 0.3);
        assert_eq!(state.unity_awareness, 0.15); // 0.3 * 0.5
    }

    #[test]
    fn test_entity_state_increase_unity_awareness() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        state.increase_unity_awareness(0.4);

        assert_eq!(state.unity_awareness, 0.4);
        assert_eq!(state.evolution_height, 0.2); // 0.4 * 0.5
    }

    #[test]
    fn test_entity_state_thin_veil() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        state.thin_veil(0.2);

        // Use approximate comparison for floating point
        assert!((state.veil_thickness - 0.6).abs() < 1e-9); // 0.8 - 0.2
    }

    #[test]
    fn test_entity_state_calculate_balance() {
        let entity_id = 1;
        let state = EntityInvolutionEvolutionState::with_values(
            entity_id,
            Density::Third,
            0.8, // involution_depth
            0.8, // specificity
            0.8, // veil_thickness
            0.2, // evolution_height
            0.2, // unity_awareness
            0.0,
            0.0,
        );

        // High involution, low evolution -> negative balance
        let balance = state.calculate_balance();
        assert!(balance < 0.0);
    }

    #[test]
    fn test_entity_state_clamping() {
        let entity_id = 1;
        let mut state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);

        // Test upper bounds
        state.increase_specificity(2.0);
        assert_eq!(state.specificity, 1.0);

        state.increase_awareness(2.0);
        assert_eq!(state.evolution_height, 1.0);

        // Test veil thickness bounds
        state.thicken_veil(1.0);
        assert_eq!(state.veil_thickness, 1.0);

        state.thin_veil(2.0);
        assert_eq!(state.veil_thickness, 0.0);
    }

    // -------------------------------------------------------------------------
    // InvolutionEvolutionEngine Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_engine_creation() {
        let engine = InvolutionEvolutionEngine::new();

        assert_eq!(engine.flow_direction, FlowDirection::Balanced);
        assert_eq!(engine.flow_intensity, 0.5);
        assert_eq!(engine.get_entity_count(), 0);
    }

    #[test]
    fn test_engine_with_settings() {
        let engine = InvolutionEvolutionEngine::with_settings(0.7, 0.2);

        assert_eq!(engine.flow_intensity, 0.7);
        assert_eq!(engine.balance_sensitivity, 0.2);
    }

    #[test]
    fn test_engine_register_entity() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        assert_eq!(engine.get_entity_count(), 1);
        assert!(engine.get_entity_state(entity_id).is_some());
    }

    #[test]
    fn test_engine_register_entity_with_state() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        let state = EntityInvolutionEvolutionState::new(entity_id, Density::Third);
        engine.register_entity_with_state(state);

        assert_eq!(engine.get_entity_count(), 1);
    }

    #[test]
    fn test_engine_get_entity_state() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        let state = engine.get_entity_state(entity_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().entity_id, entity_id);
    }

    #[test]
    fn test_engine_get_entity_state_mut() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        let state = engine.get_entity_state_mut(entity_id);
        assert!(state.is_some());
    }

    #[test]
    fn test_engine_process_entity() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        let result = engine.process(entity_id);
        assert!(result.is_some());
    }

    #[test]
    fn test_engine_process_all() {
        let mut engine = InvolutionEvolutionEngine::new();

        engine.register_entity(1, Density::Third);
        engine.register_entity(2, Density::Fourth);
        engine.register_entity(3, Density::Fifth);

        assert_eq!(engine.get_entity_count(), 3);

        engine.process_all();

        // All entities should have been processed
        assert_eq!(engine.get_entity_count(), 3);
    }

    #[test]
    fn test_engine_update_entity_polarization() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);
        engine.update_entity_polarization(entity_id, 0.8, 0.9);

        let state = engine.get_entity_state(entity_id).unwrap();
        assert_eq!(state.polarization_intensity, 0.8);
        assert_eq!(state.polarization_direction, 0.9);
    }

    #[test]
    fn test_engine_update_entity_density() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);
        engine.update_entity_density(entity_id, Density::Fourth);

        let state = engine.get_entity_state(entity_id).unwrap();
        assert_eq!(state.density, Density::Fourth);
        assert_eq!(state.veil_thickness, 0.6); // D4 base veil thickness
    }

    #[test]
    fn test_engine_clear_entities() {
        let mut engine = InvolutionEvolutionEngine::new();

        engine.register_entity(1, Density::Third);
        engine.register_entity(2, Density::Fourth);

        assert_eq!(engine.get_entity_count(), 2);

        engine.clear_entities();

        assert_eq!(engine.get_entity_count(), 0);
    }

    #[test]
    fn test_engine_reset_metrics() {
        let mut engine = InvolutionEvolutionEngine::new();

        engine.register_entity(1, Density::Third);
        engine.process(1);

        assert!(engine.involution_depth > 0.0 || engine.evolution_height > 0.0);

        engine.reset_metrics();

        assert_eq!(engine.involution_depth, 0.0);
        assert_eq!(engine.specificity, 0.0);
        assert_eq!(engine.evolution_height, 0.0);
        assert_eq!(engine.unity_awareness, 0.0);
        assert_eq!(engine.involution_evolution_balance, 0.0);
    }

    #[test]
    fn test_engine_get_stats() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        let stats = engine.get_stats();

        assert_eq!(stats.entity_count, 1);
        assert_eq!(stats.flow_direction, FlowDirection::Balanced);
        assert_eq!(stats.flow_intensity, 0.5);
    }

    // -------------------------------------------------------------------------
    // DensityExt Tests
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Integration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_density_affects_balance() {
        let mut engine = InvolutionEvolutionEngine::new();

        // D3 (middle density) has thickest veil (0.8), should favor involution
        // The veil thickens during involution, making D3 the most individuated
        engine.register_entity(1, Density::Third);
        for _ in 0..10 {
            engine.process(1);
        }

        let state_d3 = engine.get_entity_state(1).unwrap();
        let entity_balance_d3 = state_d3.calculate_balance();
        // D3 should have negative or balanced balance due to thick veil
        assert!(
            entity_balance_d3 <= 0.3,
            "D3 entity should have involution or balanced balance, got {}",
            entity_balance_d3
        );

        // D1 (low density) has thin veil (0.0), more aware of unity
        engine.clear_entities();
        engine.register_entity(2, Density::First(Density1SubLevel::Quantum));
        for _ in 0..10 {
            engine.process(2);
        }

        let state_d1 = engine.get_entity_state(2).unwrap();
        let entity_balance_d1 = state_d1.calculate_balance();
        // D1 thin veil gives evolution boost
        assert!(
            entity_balance_d1 >= -0.3,
            "D1 entity should have balanced or evolution balance, got {}",
            entity_balance_d1
        );

        // D7 (high density) has thin veil (0.0), most aware of unity
        engine.clear_entities();
        engine.register_entity(3, Density::Seventh);
        for _ in 0..10 {
            engine.process(3);
        }

        let state_d7 = engine.get_entity_state(3).unwrap();
        let entity_balance_d7 = state_d7.calculate_balance();
        // D7 should have evolution or balanced balance
        assert!(
            entity_balance_d7 >= -0.3,
            "D7 entity should have balanced or evolution balance, got {}",
            entity_balance_d7
        );
    }

    #[test]
    fn test_polarization_affects_balance() {
        let mut engine = InvolutionEvolutionEngine::with_settings(0.5, 0.5); // Higher sensitivity

        // Test STS polarization (should favor involution)
        let entity_sts = 1;
        engine.register_entity(entity_sts, Density::Fourth); // Middle density
        engine.update_entity_polarization(entity_sts, 0.95, -0.95); // Maximum polarization
        for _ in 0..30 {
            engine.process(entity_sts);
        }

        let state_sts = engine.get_entity_state(entity_sts).unwrap();
        let entity_balance_sts = state_sts.calculate_balance();
        assert!(
            entity_balance_sts < 0.0,
            "STS polarization should favor involution, got {}",
            entity_balance_sts
        );

        // Test STO polarization (should favor evolution)
        let entity_sto = 2;
        engine.register_entity(entity_sto, Density::Fourth); // Same density
        engine.update_entity_polarization(entity_sto, 0.95, 0.95); // Maximum polarization
        for _ in 0..30 {
            engine.process(entity_sto);
        }

        let state_sto = engine.get_entity_state(entity_sto).unwrap();
        let entity_balance_sto = state_sto.calculate_balance();
        assert!(
            entity_balance_sto > 0.0,
            "STO polarization should favor evolution, got {}",
            entity_balance_sto
        );

        // Verify STS balance is more negative than STO balance
        assert!(
            entity_balance_sts < entity_balance_sto,
            "STS should have more negative balance than STO"
        );
    }

    #[test]
    fn test_simultaneous_involution_evolution() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Fourth);

        // Process multiple times
        for _ in 0..10 {
            engine.process(entity_id);
        }

        let state = engine.get_entity_state(entity_id).unwrap();

        // Both involution and evolution metrics should increase
        // (base involution and evolution always occur)
        assert!(state.specificity > 0.0);
        assert!(state.evolution_height > 0.0);
        assert!(state.involution_depth > 0.0 || state.unity_awareness > 0.0);
    }

    #[test]
    fn test_balance_shifting_over_time() {
        let mut engine = InvolutionEvolutionEngine::new();
        let entity_id = 1;

        engine.register_entity(entity_id, Density::Third);

        let mut balances = Vec::new();

        // Process multiple times and track balance
        for _ in 0..20 {
            engine.process(entity_id);
            balances.push(engine.involution_evolution_balance);
        }

        // Balance should shift gradually (not jump)
        for i in 1..balances.len() {
            let change = (balances[i] - balances[i - 1]).abs();
            assert!(change < 0.5); // Should not jump too much
        }
    }

    #[test]
    fn test_global_metrics_calculation() {
        let mut engine = InvolutionEvolutionEngine::new();

        engine.register_entity(1, Density::Third);
        engine.register_entity(2, Density::Fourth);
        engine.register_entity(3, Density::Fifth);

        engine.process_all();

        let stats = engine.get_stats();

        // Global metrics should be averages of entity states
        assert!(stats.involution_depth >= 0.0 && stats.involution_depth <= 1.0);
        assert!(stats.specificity >= 0.0 && stats.specificity <= 1.0);
        assert!(stats.evolution_height >= 0.0 && stats.evolution_height <= 1.0);
        assert!(stats.unity_awareness >= 0.0 && stats.unity_awareness <= 1.0);
    }

    #[test]
    fn test_veil_dynamics() {
        let mut engine = InvolutionEvolutionEngine::with_settings(0.5, 1.0); // Very high sensitivity

        // Test 1: Veil thickens during involution
        let entity_involution = 1;
        engine.register_entity(entity_involution, Density::Third);
        let initial_veil_involution = engine
            .get_entity_state(entity_involution)
            .unwrap()
            .veil_thickness;

        engine.update_entity_polarization(entity_involution, 0.8, -0.8); // STS polarization
        for _ in 0..15 {
            engine.process(entity_involution);
        }

        let veil_after_involution = engine
            .get_entity_state(entity_involution)
            .unwrap()
            .veil_thickness;
        assert!(
            veil_after_involution > initial_veil_involution,
            "Veil should thicken during involution: {} > {}",
            veil_after_involution,
            initial_veil_involution
        );

        // Test 2: Veil dynamics work correctly (test that veil changes based on balance)
        // Use a fresh entity at D6 (thin veil base, positive density balance)
        let entity_evolution = 2;
        engine.register_entity(entity_evolution, Density::Sixth); // D6 has thin veil (0.2) and positive balance
        let initial_veil_evolution = engine
            .get_entity_state(entity_evolution)
            .unwrap()
            .veil_thickness;

        // Apply STO polarization (should favor evolution)
        engine.update_entity_polarization(entity_evolution, 0.95, 0.95); // Strong STO polarization
        for _ in 0..50 {
            engine.process(entity_evolution);
        }

        let veil_after_evolution = engine
            .get_entity_state(entity_evolution)
            .unwrap()
            .veil_thickness;
        let state_after_evolution = engine.get_entity_state(entity_evolution).unwrap();

        // At D6 with strong STO polarization, balance should be positive (evolution)
        // This means the veil should thin (or at least not thicken)
        let balance = state_after_evolution.calculate_balance();
        println!(
            "D6 + STO: initial_veil={}, final_veil={}, balance={}",
            initial_veil_evolution, veil_after_evolution, balance
        );

        // With positive balance, veil should not thicken significantly
        assert!(
            veil_after_evolution <= initial_veil_evolution + 0.1,
            "With positive balance, veil should not thicken significantly: {} <= {}",
            veil_after_evolution,
            initial_veil_evolution + 0.1
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_involution_stage_properties() {
        // Test stage properties
        let violet = InvolutionStage::Violet;
        assert_eq!(violet.layer_number(), 0);
        assert_eq!(violet.ray_color(), "Violet-Ray");

        let indigo = InvolutionStage::Indigo;
        assert_eq!(indigo.layer_number(), 1);
        assert_eq!(indigo.ray_color(), "Indigo-Ray");

        let layer7 = InvolutionStage::Layer7;
        assert_eq!(layer7.layer_number(), 7);
        assert_eq!(layer7.ray_color(), "Beyond Red-Ray");

        // Test stage ordering
        assert!(violet < indigo);
        assert!(indigo < InvolutionStage::Blue);
        assert!(InvolutionStage::Blue < InvolutionStage::Green);
        assert!(InvolutionStage::Green < InvolutionStage::Yellow);
        assert!(InvolutionStage::Yellow < InvolutionStage::Orange);
        assert!(InvolutionStage::Orange < InvolutionStage::Red);
        assert!(InvolutionStage::Red < InvolutionStage::Layer7);
    }

    #[test]
    fn test_involution_sequence_runner_creation() {
        let runner = InvolutionSequenceRunner::new();
        assert_eq!(runner.current_stage(), InvolutionStage::Violet);
        assert_eq!(runner.entity_count(), 0);
    }

    #[test]
    fn test_involution_sequence_runner_with_config() {
        let runner = InvolutionSequenceRunner::with_config(50, 2, 3, 4);
        assert_eq!(runner.current_stage(), InvolutionStage::Violet);
        assert_eq!(runner.entity_count(), 0);
    }

    #[test]
    fn test_involution_sequence_violet_realm() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        assert_eq!(runner.current_stage(), InvolutionStage::Violet);
        assert!(runner.violet_realm.is_some());
    }

    #[test]
    fn test_involution_sequence_first_distortion() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Indigo);
        assert!(runner.intelligent_infinity.is_some());
        assert_eq!(runner.attractor_field_count(), 1);
        assert_eq!(runner.stage_transition_count(), 1);
    }

    #[test]
    fn test_involution_sequence_second_distortion() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Blue);
        assert!(runner.logos.is_some());
        assert_eq!(runner.attractor_field_count(), 2);
        assert_eq!(runner.stage_transition_count(), 2);
    }

    #[test]
    fn test_involution_sequence_third_distortion() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();
        runner.apply_third_distortion().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Green);
        assert!(runner.light_love_field.is_some());
        assert_eq!(runner.attractor_field_count(), 3);
        assert_eq!(runner.stage_transition_count(), 3);
    }

    #[test]
    fn test_involution_sequence_dimensional_architecture() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();
        runner.apply_third_distortion().unwrap();
        runner.create_dimensional_architecture().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Yellow);
        assert!(runner.yellow_realm.is_some());
        assert_eq!(runner.attractor_field_count(), 4);
        assert_eq!(runner.stage_transition_count(), 4);
    }

    #[test]
    fn test_involution_sequence_galactic_configuration() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();
        runner.apply_third_distortion().unwrap();
        runner.create_dimensional_architecture().unwrap();
        runner.configure_galactic_spectrum().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Orange);
        assert!(runner.orange_realm.is_some());
        assert_eq!(runner.attractor_field_count(), 5);
        assert_eq!(runner.stage_transition_count(), 5);
    }

    #[test]
    fn test_involution_sequence_solar_configuration() {
        let mut runner = InvolutionSequenceRunner::new();
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();
        runner.apply_third_distortion().unwrap();
        runner.create_dimensional_architecture().unwrap();
        runner.configure_galactic_spectrum().unwrap();
        runner.configure_solar_spectrum().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Red);
        assert!(runner.red_realm.is_some());
        assert_eq!(runner.attractor_field_count(), 6);
        assert_eq!(runner.stage_transition_count(), 6);
    }

    #[test]
    fn test_involution_sequence_entity_creation() {
        let mut runner = InvolutionSequenceRunner::with_config(130, 1, 1, 1);
        runner.initialize_violet_realm().unwrap();
        runner.apply_first_distortion().unwrap();
        runner.apply_second_distortion().unwrap();
        runner.apply_third_distortion().unwrap();
        runner.create_dimensional_architecture().unwrap();
        runner.configure_galactic_spectrum().unwrap();
        runner.configure_solar_spectrum().unwrap();
        runner.create_individual_entities().unwrap();

        assert_eq!(runner.current_stage(), InvolutionStage::Layer7);
        // Total entities created with num_galaxies=1, num_solar_systems=1, num_planets=1:
        // 1 Galactic Logos + 1 Solar Logos + 1 Galaxy + 3 Stars + 6 Planets +
        // 80 Quantum particles + 20 Atoms + 10 Molecules + 5 Cells + 2 Organisms + 1 Individual = 130
        assert_eq!(runner.entity_count(), 130);
        assert_eq!(runner.attractor_field_count(), 7);
        assert_eq!(runner.stage_transition_count(), 7);
    }

    #[test]
    fn test_involution_sequence_complete() {
        let mut runner = InvolutionSequenceRunner::with_config(130, 1, 1, 1);
        let result = runner.run_involution_sequence().unwrap();

        // Total entities created with num_galaxies=1, num_solar_systems=1, num_planets=1:
        // 1 Galactic Logos + 1 Solar Logos + 1 Galaxy + 3 Stars + 6 Planets +
        // 80 Quantum particles + 20 Atoms + 10 Molecules + 5 Cells + 2 Organisms + 1 Individual = 130
        assert_eq!(result.entities.len(), 130);
        assert_eq!(result.attractor_fields.len(), 7);
        assert_eq!(result.stage_transitions.len(), 7);
        assert_eq!(runner.current_stage(), InvolutionStage::Layer7);
    }

    #[test]
    fn test_involution_sequence_complete_default_config() {
        let mut runner = InvolutionSequenceRunner::new();
        let result = runner.run_involution_sequence().unwrap();

        // Total entities created with default config (num_galaxies=3, num_solar_systems=2, num_planets=3):
        // 3 Galactic Logoi + 6 Solar Logoi + 1 Galaxy + 3 Stars + 6 Planets +
        // 80 Quantum particles + 20 Atoms + 10 Molecules + 5 Cells + 2 Organisms + 1 Individual = 137
        // Note: Only 1 Galaxy, 3 Stars, and 6 Planets are created (hardcoded), not scaled by config
        assert_eq!(result.entities.len(), 137);
        assert_eq!(result.attractor_fields.len(), 7);
        assert_eq!(result.stage_transitions.len(), 7);
        assert_eq!(runner.current_stage(), InvolutionStage::Layer7);
    }

    #[test]
    fn test_involution_sequence_three_primal_distortions() {
        let mut runner = InvolutionSequenceRunner::new();
        let result = runner.run_involution_sequence().unwrap();

        // Verify that Three Primal Distortions were applied
        let feature_names: Vec<&str> = result
            .stage_transitions
            .iter()
            .map(|t| t.feature.name.as_str())
            .collect();

        assert!(feature_names.iter().any(|n| n.contains("First Distortion")));
        assert!(feature_names
            .iter()
            .any(|n| n.contains("Second Distortion")));
        assert!(feature_names.iter().any(|n| n.contains("Third Distortion")));
    }

    #[test]
    fn test_involution_sequence_transcend_include() {
        let mut runner = InvolutionSequenceRunner::new();
        let result = runner.run_involution_sequence().unwrap();

        // Verify that each transition includes, transcends, and evolves
        assert_eq!(result.stage_transitions.len(), 7);

        for transition in &result.stage_transitions {
            // Each transition has a feature (transcends) and attractor-field (evolves)
            assert!(!transition.feature.name.is_empty());
            assert!(!transition.attractor_field.name.is_empty());
            assert!(!transition.attractor_field.target.is_empty());
        }
    }

    #[test]
    fn test_involution_sequence_attractor_fields() {
        let mut runner = InvolutionSequenceRunner::new();
        let result = runner.run_involution_sequence().unwrap();

        // Verify attractor-fields
        assert_eq!(result.attractor_fields.len(), 7);

        // Check specific attractor-fields
        let attractor_names: Vec<&str> = result
            .attractor_fields
            .iter()
            .map(|a| a.name.as_str())
            .collect();

        assert!(attractor_names.iter().any(|n| n.contains("Archetype 22")));
        assert!(attractor_names
            .iter()
            .any(|n| n.contains("Universal Archetypical Patterns")));
        assert!(attractor_names
            .iter()
            .any(|n| n.contains("Light/Love Field")));
    }

    #[test]
    fn test_involution_sequence_entities_holographic_completeness() {
        let mut runner = InvolutionSequenceRunner::with_config(3, 1, 1, 1);
        runner.run_involution_sequence().unwrap();

        // Verify that entities have holographic completeness
        for entity in &runner.entities {
            let report = entity.verify_holographic_completeness();
            assert_eq!(report.completeness_percentage, 100.0);
        }
    }

    #[test]
    fn test_involution_error_stage_transition_failed() {
        let mut runner = InvolutionSequenceRunner::new();

        // Try to apply first distortion without initializing violet realm
        let result = runner.apply_first_distortion();

        assert!(result.is_err());
        match result {
            Err(InvolutionError::StageTransitionFailed { stage, .. }) => {
                assert_eq!(stage, InvolutionStage::Violet);
            }
            _ => panic!("Expected StageTransitionFailed error"),
        }
    }

    #[test]
    fn test_involution_stage_description() {
        assert!(InvolutionStage::Violet.description().contains("Infinity"));
        assert!(InvolutionStage::Indigo
            .description()
            .contains("IntelligentInfinity"));
        assert!(InvolutionStage::Blue.description().contains("Logos"));
        assert!(InvolutionStage::Green.description().contains("Light/Love"));
        assert!(InvolutionStage::Yellow.description().contains("Spectrum"));
        assert!(InvolutionStage::Orange.description().contains("Galactic"));
        assert!(InvolutionStage::Red.description().contains("Solar"));
        assert!(InvolutionStage::Layer7
            .description()
            .contains("Sub-Sub-Logos"));
    }

    #[test]
    fn test_involution_stage_display() {
        let violet = InvolutionStage::Violet;
        assert_eq!(format!("{}", violet), "Violet-Ray (Layer 0)");

        let layer7 = InvolutionStage::Layer7;
        assert_eq!(format!("{}", layer7), "Beyond Red-Ray (Layer 7)");
    }

    #[test]
    fn test_involution_error_display() {
        let error = InvolutionError::StageTransitionFailed {
            stage: InvolutionStage::Violet,
            reason: "Test reason".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Stage transition failed"));
        assert!(display.contains("Violet"));
    }
}
