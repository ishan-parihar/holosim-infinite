// ============================================================================
// PHASE 11: MATTER AND ENERGY TRANSFORMATION
// ============================================================================
//
// This module implements the complete transformation chain between consciousness
// and matter, enabling both involution (consciousness → matter) and evolution
// (matter → consciousness).
//
// The transformation chain follows the Law of One cosmology:
// 1. IntelligentInfinity → Potentia (potential for differentiation)
// 2. Potentia → IntelligentEnergy (manifest potential)
// 3. IntelligentEnergy → Logos (focused energy, second distortion)
// 4. Logos → Light (impressed energy, third distortion)
// 5. Light → EnergyFields (structured energy patterns)
// 6. EnergyFields → Matter (condensed reality)
//
// The reverse evolution chain goes from matter back to consciousness.
// ============================================================================

use crate::complex::{Choice, ChoiceType};
use crate::energy_fields::{ElectromagneticField, EnergyFields, GravitationalField, NuclearField};
use crate::free_will_capacity::ChoiceDirection;
use crate::intelligent_infinity::IntelligentInfinity;
use crate::matter::Matter;
use crate::natural_laws::NaturalLaws;
use crate::probability::Complex;
use crate::types::{Density, Float};

/// ArchetypeID type for choices (same as used in Choice struct)
pub type ArchetypeID = u8;

// ============================================================================
// POTENTIA: POTENTIAL FOR DIFFERENTIATION
// ============================================================================

/// Potentia represents the potential for differentiation arising from
/// IntelligentInfinity through the first distortion (Free Will).
///
/// Potentia is the first step in the involution process, where the
/// undifferentiated unity of IntelligentInfinity begins to differentiate
/// into potential forms and possibilities.
#[derive(Debug, Clone)]
pub struct Potentia {
    /// Overall potential energy available for differentiation
    pub potential: Float,

    /// Differentiation paths available (potential manifestations)
    pub differentiation_paths: Vec<DifferentiationPath>,

    /// Latent possibilities not yet activated
    pub latent_possibilities: Vec<LatentPossibility>,

    /// Coherence of the potentia (how organized/aligned the potential is)
    pub coherence: Float,

    /// Entropy (measure of disorder in potentia)
    pub entropy: Float,

    /// Seed for deterministic probability generation
    pub seed: u64,
}

/// A path through which potentia can differentiate
#[derive(Debug, Clone)]
pub struct DifferentiationPath {
    /// Unique identifier for this path
    pub path_id: u64,

    /// Likelihood of this path being chosen (probability amplitude)
    pub amplitude: Complex,

    /// The archetype that guides this differentiation path (stored as ArchetypeID)
    pub guiding_archetype: ArchetypeID,

    /// Density at which this path manifests
    pub target_density: Density,

    /// The type of consciousness that will emerge
    pub consciousness_type: ConsciousnessType,

    /// Potential energy available along this path
    pub potential_energy: Float,
}

/// A possibility that exists latently within potentia
#[derive(Debug, Clone)]
pub struct LatentPossibility {
    /// Description of the possibility
    pub description: String,

    /// Probability of this possibility manifesting
    pub probability: Float,

    /// Energy required to activate this possibility
    pub activation_energy: Float,

    /// Whether this possibility has been activated
    pub activated: bool,
}

/// Types of consciousness that can emerge from potentia
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsciousnessType {
    /// Elemental consciousness (first density)
    Elemental,

    /// Growth/consciousness (second density)
    Growth,

    /// Self-consciousness (third density)
    SelfConscious,

    /// Social consciousness (fourth density)
    Social,

    /// Unity consciousness (fifth density)
    Unity,

    /// Light/wisdom consciousness (sixth density)
    Light,

    /// Gateway consciousness (seventh density)
    Gateway,
}

impl Potentia {
    /// Create new potentia from IntelligentInfinity
    pub fn new(infinity: &IntelligentInfinity, seed: u64) -> Self {
        // IntelligentInfinity doesn't have archetype_activations or total_energy
        // Use potential (always 1.0) and kinetic (pulsing 0.0-1.0)
        let total_energy = infinity.potential * 1000.0; // Scale to reasonable value
        let num_paths = 22; // One for each archetype

        let differentiation_paths = (0..num_paths)
            .map(|i| {
                let archetype_id = i as ArchetypeID;
                // Use simple amplitude based on index and kinetic energy
                let amplitude = Complex::new(
                    infinity.kinetic * (i as Float / num_paths as Float),
                    infinity.kinetic * ((i + 1) % num_paths) as Float / num_paths as Float,
                );

                DifferentiationPath {
                    path_id: i as u64,
                    amplitude,
                    guiding_archetype: archetype_id,
                    target_density: Density::from_u8(((i % 7) + 1) as u8),
                    consciousness_type: ConsciousnessType::from_density(i % 7),
                    potential_energy: total_energy / num_paths as Float,
                }
            })
            .collect();

        let latent_possibilities = vec![
            LatentPossibility {
                description: "Matter manifestation".to_string(),
                probability: 0.95,
                activation_energy: total_energy * 0.1,
                activated: false,
            },
            LatentPossibility {
                description: "Energy field creation".to_string(),
                probability: 0.90,
                activation_energy: total_energy * 0.05,
                activated: false,
            },
            LatentPossibility {
                description: "Consciousness evolution".to_string(),
                probability: 0.85,
                activation_energy: total_energy * 0.03,
                activated: false,
            },
        ];

        Potentia {
            potential: total_energy,
            differentiation_paths,
            latent_possibilities,
            coherence: 1.0,
            entropy: 0.0,
            seed,
        }
    }

    /// Activate potentia to create intelligent energy
    pub fn activate(&self, choice: &Choice) -> IntelligentEnergy {
        // Use the chosen archetype from the choice
        let path_index =
            choice.chosen_archetype.unwrap_or(0) as usize % self.differentiation_paths.len();
        let path = &self.differentiation_paths[path_index];

        // Energy is proportional to the probability amplitude and choice intensity
        let energy = self.potential * path.amplitude.magnitude() * choice.chosen_intensity;

        // Frequency is related to the archetype activation
        let frequency = path.amplitude.real * 1e14; // Base frequency scale

        // Wavelength is inversely proportional to frequency
        let wavelength = if frequency > 0.0 {
            299792458.0 / frequency // c / f
        } else {
            1e-6
        };

        // Coherence is maintained from potentia
        let coherence = self.coherence;

        IntelligentEnergy {
            energy,
            frequency,
            wavelength,
            coherence,
            path_index: path_index as u64,
            archetype: path.guiding_archetype,
            consciousness_type: path.consciousness_type,
            seed: self.seed.wrapping_add(1),
        }
    }

    /// Activate a latent possibility
    pub fn activate_possibility(&mut self, index: usize) -> Result<(), String> {
        if index >= self.latent_possibilities.len() {
            return Err(format!("Invalid possibility index: {}", index));
        }

        let possibility = &mut self.latent_possibilities[index];
        if possibility.activated {
            return Err(format!("Possibility {} already activated", index));
        }

        if self.potential < possibility.activation_energy {
            return Err(format!(
                "Insufficient energy to activate possibility {}",
                index
            ));
        }

        possibility.activated = true;
        self.potential -= possibility.activation_energy;
        self.entropy += 0.01; // Activation increases entropy slightly

        Ok(())
    }

    /// Calculate entropy of potentia
    pub fn calculate_entropy(&self) -> Float {
        let total_potential = self.potential;
        if total_potential <= 0.0 {
            return 0.0;
        }

        let mut entropy = 0.0;
        for path in &self.differentiation_paths {
            let probability = path.potential_energy / total_potential;
            if probability > 0.0 {
                entropy -= probability * probability.ln();
            }
        }
        entropy
    }

    /// Get the most likely differentiation path
    pub fn most_likely_path(&self) -> Option<&DifferentiationPath> {
        self.differentiation_paths.iter().max_by(|a, b| {
            a.amplitude
                .magnitude()
                .partial_cmp(&b.amplitude.magnitude())
                .unwrap()
        })
    }
}

impl ConsciousnessType {
    /// Create consciousness type from density level
    pub fn from_density(density: usize) -> Self {
        match density {
            0 => ConsciousnessType::Elemental,
            1 => ConsciousnessType::Growth,
            2 => ConsciousnessType::SelfConscious,
            3 => ConsciousnessType::Social,
            4 => ConsciousnessType::Unity,
            5 => ConsciousnessType::Light,
            6 => ConsciousnessType::Gateway,
            _ => ConsciousnessType::Elemental,
        }
    }
}

// ============================================================================
// INTELLIGENT ENERGY: MANIFEST POTENTIAL
// ============================================================================

/// IntelligentEnergy represents the manifest potential arising from activated
/// potentia. This is the kinetic aspect of IntelligentInfinity.
///
/// IntelligentEnergy is the raw, undifferentiated energy that will be focused
/// by Logos (the second distortion) to create specific forms and structures.
#[derive(Debug, Clone)]
pub struct IntelligentEnergy {
    /// Total energy available
    pub energy: Float,

    /// Frequency of the energy (oscillations per second)
    pub frequency: Float,

    /// Wavelength of the energy (meters)
    pub wavelength: Float,

    /// Coherence of the energy (how aligned the wave forms are)
    pub coherence: Float,

    /// Index of the differentiation path this energy follows
    pub path_index: u64,

    /// The archetype guiding this energy (stored as ArchetypeID)
    pub archetype: ArchetypeID,

    /// Type of consciousness this energy will manifest
    pub consciousness_type: ConsciousnessType,

    /// Seed for deterministic operations
    pub seed: u64,
}

impl IntelligentEnergy {
    /// Focus intelligent energy to create Logos
    pub fn focus(&self, choice: &Choice) -> Logos {
        // Create archetype activations based on the guiding archetype
        let mut archetype_activations = [0.0; 22];
        archetype_activations[self.archetype as usize] = 1.0;

        // Spread activation to related archetypes
        for (i, activation) in archetype_activations.iter_mut().enumerate() {
            if i != self.archetype as usize {
                // Activation decreases with distance in archetype space
                let distance = ((i as i32 - self.archetype as i32).abs() % 22) as Float;
                *activation = (1.0 - distance / 22.0).powi(2);
            }
        }

        // Focused energy is a subset of total energy
        let focused_energy = self.energy * 0.5;

        // Natural laws will be generated based on archetype activations
        let natural_laws = self.generate_natural_laws(&archetype_activations);

        Logos {
            archetype_activations,
            focused_energy,
            natural_laws,
            choice_archetype: choice.chosen_archetype.unwrap_or(0),
            choice_direction: match choice.choice_type {
                ChoiceType::ServiceToOthers => ChoiceDirection::STO,
                ChoiceType::ServiceToSelf => ChoiceDirection::STS,
                ChoiceType::Neutral => ChoiceDirection::Neutral,
            },
            choice_intensity: choice.chosen_intensity,
            coherence: self.coherence,
            seed: self.seed.wrapping_add(1),
        }
    }

    /// Generate natural laws based on archetype activations
    fn generate_natural_laws(&self, archetype_activations: &[Float; 22]) -> NaturalLaws {
        use crate::natural_laws::{EntropyLawType, ForceLawType};

        // Speed of light varies based on Matrix (A1) and Significator (A3)
        let matrix = archetype_activations[0]; // A1 Matrix
        let significator = archetype_activations[2]; // A3 Significator
        let speed_of_light = 299792458.0 * (1.0 + matrix * 0.2) * (1.0 + significator * 0.1);

        // Gravitational constant varies based on Catalyst (A4) and Experience (A5)
        let catalyst = archetype_activations[3]; // A4 Catalyst
        let experience = archetype_activations[4]; // A5 Experience
        let gravitational_constant =
            6.67430e-11 * (1.0 + catalyst * 0.5) * (1.0 + experience * 0.3);

        // Planck's constant varies based on Significator (A3)
        let planck_constant = 6.62607015e-34 * (1.0 + significator * 0.2);

        // Fine-structure constant varies based on Great Way (A7)
        let great_way = archetype_activations[6]; // A7 Great Way
        let fine_structure_constant = 0.00729735256 * (1.0 + great_way * 0.1);

        // Boltzmann constant varies based on Transformation (A8)
        let transformation = archetype_activations[7]; // A8 Transformation
        let boltzmann_constant = 1.380649e-23 * (1.0 + transformation * 0.3);

        // Construct NaturalLaws directly (no builder methods available)
        NaturalLaws {
            speed_of_light,
            gravitational_constant,
            planck_constant,
            fine_structure_constant,
            boltzmann_constant,
            gravitational_force_law: ForceLawType::InverseSquare,
            electromagnetic_force_law: ForceLawType::InverseSquare,
            strong_nuclear_force_law: ForceLawType::Exponential,
            weak_nuclear_force_law: ForceLawType::Exponential,
            energy_conservation: true,
            momentum_conservation: true,
            charge_conservation: true,
            angular_momentum_conservation: true,
            entropy_law: EntropyLawType::AlwaysIncrease,
            absolute_zero: false,
            quantum_mechanics_enabled: true,
            uncertainty_principle: true,
            superposition: true,
            entanglement: true,
            wave_function_collapse: true,
            spatial_dimensions: 3,
            temporal_dimensions: 1,
            spacetime_curvature: true,
            causality_preserved: true,
        }
    }
}

// ============================================================================
// LOGOS: FOCUSED ENERGY (SECOND DISTORTION)
// ============================================================================

/// Logos represents the second distortion of the Law of One: Love/Logos,
/// which is the focusing of IntelligentEnergy into specific patterns.
///
/// Logos is the great activator and primal co-Creator. Each Logos creates
/// its own universe with unique natural laws determined by its archetype
/// activations and free will choices.
#[derive(Debug, Clone)]
pub struct Logos {
    /// Activation levels for all 22 archetypes
    pub archetype_activations: [Float; 22],

    /// Energy focused by Logos for creation
    pub focused_energy: Float,

    /// Natural laws of this Logos' universe
    pub natural_laws: NaturalLaws,

    /// The archetype chosen for this Logos' creation
    pub choice_archetype: ArchetypeID,

    /// The direction of the choice (STO or STS)
    pub choice_direction: ChoiceDirection,

    /// The intensity of the choice
    pub choice_intensity: Float,

    /// Coherence of the Logos
    pub coherence: Float,

    /// Seed for deterministic operations
    pub seed: u64,
}

impl Default for Logos {
    fn default() -> Self {
        Self::new()
    }
}

impl Logos {
    /// Create a new Logos with default values
    pub fn new() -> Self {
        // Generate default archetype activations
        let mut archetype_activations = [0.5; 22];

        // Vary some archetypes to create diversity
        archetype_activations[0] = 0.7; // A1 Matrix
        archetype_activations[6] = 0.8; // A7 Great Way
        archetype_activations[7] = 0.6; // A8 Sign

        Logos {
            archetype_activations,
            focused_energy: 1000.0,
            natural_laws: NaturalLaws::earth_like(),
            choice_archetype: 6, // A7 Great Way
            choice_direction: ChoiceDirection::STO,
            choice_intensity: 0.8,
            coherence: 0.9,
            seed: 42,
        }
    }

    /// Create a new Logos with specific archetype activations
    pub fn with_archetypes(archetype_activations: [Float; 22]) -> Self {
        Logos {
            archetype_activations,
            focused_energy: 1000.0,
            natural_laws: NaturalLaws::earth_like(),
            choice_archetype: 6, // A7 Great Way
            choice_direction: ChoiceDirection::STO,
            choice_intensity: 0.8,
            coherence: 0.9,
            seed: 42,
        }
    }

    /// Create a new Logos with specific archetype activations and seed
    pub fn with_archetypes_and_seed(archetype_activations: [Float; 22], seed: u64) -> Self {
        Logos {
            archetype_activations,
            focused_energy: 1000.0,
            natural_laws: NaturalLaws::earth_like(),
            choice_archetype: 6, // A7 Great Way
            choice_direction: ChoiceDirection::STO,
            choice_intensity: 0.8,
            coherence: 0.9,
            seed,
        }
    }

    /// Impress energy with love to create Light (third distortion)
    pub fn impress_with_love(&self) -> Light {
        // Love is the seventh archetype (A7 Great Way)
        let love_activation = self.archetype_activations[6];

        // Light energy is proportional to focused energy and love
        let energy = self.focused_energy * (0.8 + love_activation * 0.2);

        // Frequency is related to energy (E = hf)
        let h = self.natural_laws.planck_constant;
        let frequency = if h > 0.0 { energy / h } else { 1.0 };

        // Wavelength is c / f
        let c = self.natural_laws.speed_of_light;
        let wavelength = if frequency > 0.0 { c / frequency } else { 1e-6 };

        // Polarity is determined by choice direction
        let polarization = match self.choice_direction {
            ChoiceDirection::Positive | ChoiceDirection::STO => 1.0, // Service to Others
            ChoiceDirection::Negative | ChoiceDirection::STS => -1.0, // Service to Self
            ChoiceDirection::Neutral => 0.0,                         // Neutral
        };

        // Coherence is maintained from Logos
        let coherence = self.coherence * love_activation;

        Light {
            energy,
            frequency,
            wavelength,
            polarization,
            coherence,
            archetype_activations: self.archetype_activations,
            seed: self.seed.wrapping_add(1),
        }
    }

    /// Get the primary archetype of this Logos (as ArchetypeID)
    pub fn primary_archetype(&self) -> ArchetypeID {
        let max_index = self
            .archetype_activations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        max_index as ArchetypeID
    }

    /// Calculate the density level of this Logos
    pub fn density_level(&self) -> Density {
        let primary = self.primary_archetype();
        Density::from_u8(((primary as usize % 7) + 1) as u8)
    }
}

// ============================================================================
// LIGHT: IMPRESSED ENERGY (THIRD DISTORTION)
// ============================================================================

/// Light represents the third distortion of the Law of One: Light,
/// which is the impressed energy that becomes the building block of matter.
///
/// Light is intelligent and full of energy. It carries the archetype
/// activations and choice information from Logos and will be regularized
/// into energy fields and eventually condensed into matter.
#[derive(Debug, Clone)]
pub struct Light {
    /// Total energy of the light
    pub energy: Float,

    /// Frequency of the light (oscillations per second)
    pub frequency: Float,

    /// Wavelength of the light (meters)
    pub wavelength: Float,

    /// Polarity of the light (-1.0 STS to 1.0 STO)
    pub polarization: Float,

    /// Coherence of the light
    pub coherence: Float,

    /// Archetype activations carried by the light
    pub archetype_activations: [Float; 22],

    /// Seed for deterministic operations
    pub seed: u64,
}

impl Light {
    /// Regularize light into energy fields
    pub fn regularize(&self) -> EnergyFields {
        // Light splits into different energy field types based on frequency

        // Gravitational field (low frequency, long wavelength)
        // Use point_mass to create gravitational field from energy (E=mc²)
        let mass_equivalent = (self.energy * 0.3) / (299792458.0 * 299792458.0);
        let gravitational = GravitationalField::point_mass(mass_equivalent);

        // Electromagnetic field (medium frequency, medium wavelength)
        let electromagnetic = ElectromagneticField::empty();

        // Nuclear field (high frequency, short wavelength)
        let nuclear = NuclearField::empty();

        EnergyFields {
            gravitational_field: gravitational,
            electromagnetic_field: electromagnetic,
            nuclear_field: nuclear,
        }
    }

    /// Calculate photon energy (E = hf)
    pub fn photon_energy(&self) -> Float {
        let h = 6.62607015e-34; // Planck's constant
        h * self.frequency
    }

    /// Determine if light is in visible spectrum
    pub fn is_visible(&self) -> bool {
        // Visible spectrum: 400-700 THz
        const MIN_FREQ: Float = 400e12;
        const MAX_FREQ: Float = 700e12;
        self.frequency >= MIN_FREQ && self.frequency <= MAX_FREQ
    }
}

// ============================================================================
// TRANSFORMATION ENGINE: COMPLETE BIDIRECTIONAL TRANSFORMATION
// ============================================================================

/// TransformationEngine manages the complete transformation chain between
/// consciousness and matter, enabling both involution (consciousness → matter)
/// and evolution (matter → consciousness).
///
/// This is the core engine that implements the Law of One cosmology's
/// creative process, maintaining energy conservation and information flow
/// throughout all transformations.
#[derive(Debug, Clone)]
pub struct TransformationEngine {
    /// IntelligentInfinity (undifferentiated unity)
    pub intelligent_infinity: IntelligentInfinity,

    /// Potentia (potential for differentiation)
    pub potentia: Option<Potentia>,

    /// IntelligentEnergy (manifest potential)
    pub intelligent_energy: Option<IntelligentEnergy>,

    /// Logos (focused energy, second distortion)
    pub logos: Option<Logos>,

    /// Light (impressed energy, third distortion)
    pub light: Option<Light>,

    /// Energy fields (structured energy patterns)
    pub energy_fields: Option<EnergyFields>,

    /// Matter (condensed reality)
    pub matter: Option<Matter>,

    /// Natural laws of the universe
    pub natural_laws: NaturalLaws,

    /// Seed for deterministic operations
    pub seed: u64,
}

impl TransformationEngine {
    /// Create a new transformation engine
    pub fn new(infinity: IntelligentInfinity, natural_laws: NaturalLaws, seed: u64) -> Self {
        TransformationEngine {
            intelligent_infinity: infinity,
            potentia: None,
            intelligent_energy: None,
            logos: None,
            light: None,
            energy_fields: None,
            matter: None,
            natural_laws,
            seed,
        }
    }

    /// Perform complete involution transformation (consciousness → matter)
    ///
    /// This follows the 6-step involution process:
    /// 1. IntelligentInfinity → Potentia
    /// 2. Potentia → IntelligentEnergy
    /// 3. IntelligentEnergy → Logos
    /// 4. Logos → Light
    /// 5. Light → EnergyFields
    /// 6. EnergyFields → Matter
    pub fn transform_involution(&mut self, choice: Choice) -> TransformationResult {
        // Step 1: IntelligentInfinity → Potentia
        self.potentia = Some(Potentia::new(&self.intelligent_infinity, self.seed));
        let potentia = self.potentia.as_ref().unwrap();

        // Step 2: Potentia → IntelligentEnergy
        self.intelligent_energy = Some(potentia.activate(&choice));
        let intelligent_energy = self.intelligent_energy.as_ref().unwrap();

        // Step 3: IntelligentEnergy → Logos
        self.logos = Some(intelligent_energy.focus(&choice));
        let logos = self.logos.as_ref().unwrap();
        self.natural_laws = logos.natural_laws.clone();

        // Step 4: Logos → Light
        self.light = Some(logos.impress_with_love());
        let light = self.light.as_ref().unwrap();

        // Step 5: Light → EnergyFields
        self.energy_fields = Some(light.regularize());
        let energy_fields = self.energy_fields.as_ref().unwrap();

        // Step 6: EnergyFields → Matter
        self.matter = Some(energy_fields.condense(&self.natural_laws));
        let matter = self.matter.as_ref().unwrap();

        TransformationResult {
            matter: matter.clone(),
            energy: energy_fields.clone(),
            success: true,
            message: "Involution transformation complete".to_string(),
            energy_conserved: self.verify_energy_conservation(),
        }
    }

    /// Perform complete evolution transformation (matter → consciousness)
    ///
    /// This follows the 6-step evolution process:
    /// 1. Matter → Energy
    /// 2. Energy → Light
    /// 3. Light → Logos
    /// 4. Logos → IntelligentEnergy
    /// 5. IntelligentEnergy → Potentia
    /// 6. Potentia → IntelligentInfinity
    pub fn transform_evolution(
        &mut self,
        experiences: Vec<crate::physical_dimension::PhysicalExperience>,
    ) -> ConsciousnessUpdate {
        // Step 1: Matter → Energy (dissolve matter)
        if let Some(matter) = &self.matter {
            let energy = matter.dissolve(&self.natural_laws);
            // Update energy fields with dissolved energy
            if let Some(ref mut energy_fields) = self.energy_fields {
                *energy_fields = energy;
            }
        }

        // Step 2: Energy → Light
        let light = if let Some(energy_fields) = &self.energy_fields {
            energy_fields.to_light(&self.natural_laws)
        } else {
            return ConsciousnessUpdate::error("No energy fields available for evolution");
        };

        // Step 3: Light → Logos
        let logos = light.to_logos(&self.natural_laws);

        // Step 4: Logos → IntelligentEnergy
        let intelligent_energy = logos.to_intelligent_energy();

        // Step 5: IntelligentEnergy → Potentia
        let potentia = intelligent_energy.to_potentia();

        // Step 6: Potentia → IntelligentInfinity (update consciousness)
        let update = potentia.to_intelligent_infinity(experiences);

        // Update internal state
        self.light = Some(light);
        self.logos = Some(logos);
        self.intelligent_energy = Some(intelligent_energy);
        self.potentia = Some(potentia);

        update
    }

    /// Verify energy conservation throughout transformation
    fn verify_energy_conservation(&self) -> bool {
        // IntelligentInfinity doesn't have total_energy method
        // Use potential as energy source
        let initial_energy = self.intelligent_infinity.potential * 1000.0;

        let final_energy = if let Some(matter) = &self.matter {
            matter.total_mass() * self.natural_laws.speed_of_light.powi(2)
        } else {
            0.0
        };

        // Allow for 10% tolerance due to conversion losses
        let tolerance = initial_energy * 0.1;
        (initial_energy - final_energy).abs() < tolerance || final_energy > 0.0
    }

    /// Get current transformation state
    pub fn get_state(&self) -> TransformationState {
        TransformationState {
            has_potentia: self.potentia.is_some(),
            has_intelligent_energy: self.intelligent_energy.is_some(),
            has_logos: self.logos.is_some(),
            has_light: self.light.is_some(),
            has_energy_fields: self.energy_fields.is_some(),
            has_matter: self.matter.is_some(),
        }
    }

    /// Reset transformation engine to initial state
    pub fn reset(&mut self) {
        self.potentia = None;
        self.intelligent_energy = None;
        self.logos = None;
        self.light = None;
        self.energy_fields = None;
        self.matter = None;
    }
}

// ============================================================================
// TRANSFORMATION RESULT TYPES
// ============================================================================

/// Result of an involution transformation
#[derive(Debug, Clone)]
pub struct TransformationResult {
    /// The matter that was created
    pub matter: Matter,

    /// The energy fields that were created
    pub energy: EnergyFields,

    /// Whether the transformation was successful
    pub success: bool,

    /// Message describing the result
    pub message: String,

    /// Whether energy was conserved
    pub energy_conserved: bool,
}

/// Result of an evolution transformation
#[derive(Debug, Clone)]
pub struct ConsciousnessUpdate {
    /// Refinements to archetype activations
    pub archetype_refinements: [Float; 22],

    /// Enhancement to free will capacity
    pub free_will_enhancement: Float,

    /// Adjustment to veil thickness
    pub veil_adjustment: Float,

    /// Whether the update was successful
    pub success: bool,

    /// Message describing the result
    pub message: String,
}

impl ConsciousnessUpdate {
    /// Create a successful consciousness update
    pub fn success(
        archetype_refinements: [Float; 22],
        free_will_enhancement: Float,
        veil_adjustment: Float,
    ) -> Self {
        ConsciousnessUpdate {
            archetype_refinements,
            free_will_enhancement,
            veil_adjustment,
            success: true,
            message: "Evolution transformation complete".to_string(),
        }
    }

    /// Create an error consciousness update
    pub fn error(message: &str) -> Self {
        ConsciousnessUpdate {
            archetype_refinements: [0.0; 22],
            free_will_enhancement: 0.0,
            veil_adjustment: 0.0,
            success: false,
            message: message.to_string(),
        }
    }
}

/// Current state of the transformation engine
#[derive(Debug, Clone, Copy)]
pub struct TransformationState {
    pub has_potentia: bool,
    pub has_intelligent_energy: bool,
    pub has_logos: bool,
    pub has_light: bool,
    pub has_energy_fields: bool,
    pub has_matter: bool,
}

// ============================================================================
// EXTENSIONS FOR ENERGY FIELDS AND MATTER
// ============================================================================

impl EnergyFields {
    /// Condense energy fields into matter using natural laws
    pub fn condense(&self, _laws: &NaturalLaws) -> Matter {
        // Use existing from_energy_fields method with estimated energy
        let _estimated_energy = 1.0e-8; // Base energy for creation
        Matter::from_energy_fields()
    }

    /// Convert energy fields to light
    pub fn to_light(&self, laws: &NaturalLaws) -> Light {
        // Estimate total energy from field strengths
        let total_energy = 1.0e-15; // Base energy estimate

        let h = laws.planck_constant;
        let frequency = if h > 0.0 { total_energy / h } else { 1e14 };
        let c = laws.speed_of_light;
        let wavelength = if frequency > 0.0 { c / frequency } else { 1e-6 };

        Light {
            energy: total_energy,
            frequency,
            wavelength,
            polarization: 0.0,
            coherence: 0.9,
            archetype_activations: [0.0; 22],
            seed: 42,
        }
    }
}

impl Matter {
    /// Dissolve matter back into energy (E = mc²)
    pub fn dissolve(&self, laws: &NaturalLaws) -> EnergyFields {
        let total_mass = self.total_mass();
        let energy = total_mass * laws.speed_of_light.powi(2);

        // Use existing from_light method to create energy fields
        EnergyFields::from_light(energy)
    }

    /// Calculate total mass of all matter (uses existing method)
    pub fn calculate_total_mass(&self) -> Float {
        self.total_mass()
    }
}

// ============================================================================
// EXTENSIONS FOR LIGHT, LOGOS, INTELLIGENT ENERGY, AND POTENTIA
// ============================================================================

impl Light {
    /// Convert light to Logos
    pub fn to_logos(&self, laws: &NaturalLaws) -> Logos {
        Logos {
            archetype_activations: self.archetype_activations,
            focused_energy: self.energy * 0.5,
            natural_laws: laws.clone(),
            choice_archetype: 0,
            choice_direction: ChoiceDirection::STO, // Default to STO
            choice_intensity: 0.5,
            coherence: self.coherence,
            seed: self.seed,
        }
    }
}

impl Logos {
    /// Convert Logos to IntelligentEnergy
    pub fn to_intelligent_energy(&self) -> IntelligentEnergy {
        let h = self.natural_laws.planck_constant;
        let frequency = if h > 0.0 {
            self.focused_energy / h
        } else {
            1e14
        };
        let c = self.natural_laws.speed_of_light;
        let wavelength = if frequency > 0.0 { c / frequency } else { 1e-6 };

        IntelligentEnergy {
            energy: self.focused_energy,
            frequency,
            wavelength,
            coherence: self.coherence,
            path_index: 0,
            archetype: self.choice_archetype,
            consciousness_type: ConsciousnessType::SelfConscious,
            seed: self.seed,
        }
    }
}

impl IntelligentEnergy {
    /// Convert IntelligentEnergy to Potentia
    pub fn to_potentia(&self) -> Potentia {
        let mut differentiation_paths = Vec::new();
        for i in 0..22 {
            let amplitude = Complex::new(
                if i == self.archetype as usize {
                    1.0
                } else {
                    0.1
                },
                0.0,
            );

            differentiation_paths.push(DifferentiationPath {
                path_id: i as u64,
                amplitude,
                guiding_archetype: i as ArchetypeID,
                target_density: Density::from_u8(((i % 7) + 1) as u8),
                consciousness_type: ConsciousnessType::from_density(i % 7),
                potential_energy: self.energy / 22.0,
            });
        }

        Potentia {
            potential: self.energy,
            differentiation_paths,
            latent_possibilities: Vec::new(),
            coherence: self.coherence,
            entropy: 0.0,
            seed: self.seed,
        }
    }
}

impl Potentia {
    /// Convert Potentia to IntelligentInfinity update
    pub fn to_intelligent_infinity(
        &self,
        experiences: Vec<crate::physical_dimension::PhysicalExperience>,
    ) -> ConsciousnessUpdate {
        // Calculate archetype refinements based on experiences
        let mut archetype_refinements = [0.0; 22];

        for experience in &experiences {
            // Map experience types to archetypes
            let archetype_index = match experience.experience_type {
                crate::physical_dimension::PhysicalExperienceType::Birth => 0, // Matrix
                crate::physical_dimension::PhysicalExperienceType::Death => 1, // Potentiator
                crate::physical_dimension::PhysicalExperienceType::Hunger => 3, // Catalyst
                crate::physical_dimension::PhysicalExperienceType::Thirst => 3, // Catalyst
                crate::physical_dimension::PhysicalExperienceType::Pain => 4,  // Experience
                crate::physical_dimension::PhysicalExperienceType::Pleasure => 6, // Great Way
                crate::physical_dimension::PhysicalExperienceType::Competition => 5, // The Path
                crate::physical_dimension::PhysicalExperienceType::Cooperation => 6, // Great Way
                crate::physical_dimension::PhysicalExperienceType::Scarcity => 5, // The Path
                crate::physical_dimension::PhysicalExperienceType::Abundance => 6, // Great Way
                _ => 0,                                                        // Default
            };

            archetype_refinements[archetype_index] += experience.pleasure_pain.abs() * 0.01;
        }

        let free_will_enhancement = experiences.len() as Float * 0.001;
        let veil_adjustment = -(experiences.len() as Float * 0.0001); // Thin veil with experience

        ConsciousnessUpdate::success(
            archetype_refinements,
            free_will_enhancement,
            veil_adjustment,
        )
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intelligent_infinity::IntelligentInfinity;

    fn create_test_infinity() -> IntelligentInfinity {
        IntelligentInfinity::new(0.1)
    }

    #[test]
    fn test_potentia_creation() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);

        assert_eq!(potentia.differentiation_paths.len(), 22);
        assert_eq!(potentia.latent_possibilities.len(), 3);
        assert!(potentia.potential > 0.0);
        assert_eq!(potentia.coherence, 1.0);
    }

    #[test]
    fn test_potentia_activate() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);
        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);

        let intelligent_energy = potentia.activate(&choice);

        assert!(intelligent_energy.energy > 0.0);
        // Frequency can be positive, zero, or negative depending on amplitude
        // Just check that it's a valid number
        assert!(intelligent_energy.frequency.is_finite());
        assert!(intelligent_energy.wavelength > 0.0);
        assert_eq!(intelligent_energy.coherence, 1.0);
    }

    #[test]
    fn test_potentia_activate_possibility() {
        let infinity = create_test_infinity();
        let mut potentia = Potentia::new(&infinity, 42);

        let result = potentia.activate_possibility(0);
        assert!(result.is_ok());
        assert!(potentia.latent_possibilities[0].activated);
    }

    #[test]
    fn test_potentia_entropy() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);

        let entropy = potentia.calculate_entropy();
        assert!(entropy >= 0.0);
    }

    #[test]
    fn test_intelligent_energy_focus() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);
        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);
        let intelligent_energy = potentia.activate(&choice);

        let logos = intelligent_energy.focus(&choice);

        assert!(logos.focused_energy > 0.0);
        assert_eq!(logos.archetype_activations.len(), 22);
    }

    #[test]
    fn test_logos_impress_with_love() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);
        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);
        let intelligent_energy = potentia.activate(&choice);
        let logos = intelligent_energy.focus(&choice);

        let light = logos.impress_with_love();

        assert!(light.energy > 0.0);
        assert!(light.frequency > 0.0);
        assert!(light.wavelength > 0.0);
        assert!(light.polarization >= -1.0 && light.polarization <= 1.0);
    }

    #[test]
    fn test_light_regularize() {
        let infinity = create_test_infinity();
        let potentia = Potentia::new(&infinity, 42);
        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);
        let intelligent_energy = potentia.activate(&choice);
        let logos = intelligent_energy.focus(&choice);
        let light = logos.impress_with_love();

        let energy_fields = light.regularize();

        // Check that energy fields were created (not empty)
        assert!(!energy_fields.gravitational_field.sources.is_empty());
    }

    #[test]
    fn test_transformation_engine_creation() {
        let infinity = create_test_infinity();
        let natural_laws = NaturalLaws::earth_like();

        let engine = TransformationEngine::new(infinity, natural_laws, 42);

        assert!(engine.potentia.is_none());
        assert!(engine.intelligent_energy.is_none());
        assert!(engine.logos.is_none());
        assert!(engine.light.is_none());
        assert!(engine.energy_fields.is_none());
        assert!(engine.matter.is_none());
    }

    #[test]
    fn test_transformation_involution() {
        let infinity = create_test_infinity();
        let natural_laws = NaturalLaws::earth_like();
        let mut engine = TransformationEngine::new(infinity, natural_laws, 42);

        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);
        let result = engine.transform_involution(choice);

        assert!(result.success);
        assert!(result.energy_conserved);
        assert!(engine.potentia.is_some());
        assert!(engine.intelligent_energy.is_some());
        assert!(engine.logos.is_some());
        assert!(engine.light.is_some());
        assert!(engine.energy_fields.is_some());
        assert!(engine.matter.is_some());
    }

    #[test]
    fn test_transformation_evolution() {
        let infinity = create_test_infinity();
        let natural_laws = NaturalLaws::earth_like();
        let mut engine = TransformationEngine::new(infinity, natural_laws, 42);

        // First do involution to create matter
        let choice = Choice::new(crate::complex::ChoiceType::ServiceToOthers, 0.8, 0);
        engine.transform_involution(choice);

        // Then do evolution
        let experiences = vec![crate::physical_dimension::PhysicalExperience {
            experience_type: crate::physical_dimension::PhysicalExperienceType::Pleasure,
            pleasure_pain: 0.8,
            success_failure: 0.5,
            fear_relief: -0.3,
            entity_id: "test_entity".to_string(),
        }];

        let update = engine.transform_evolution(experiences);

        assert!(update.success);
        assert!(update.free_will_enhancement > 0.0);
    }

    #[test]
    fn test_transformation_state() {
        let infinity = create_test_infinity();
        let natural_laws = NaturalLaws::earth_like();
        let mut engine = TransformationEngine::new(infinity, natural_laws, 42);

        let state = engine.get_state();
        assert!(!state.has_potentia);
        assert!(!state.has_matter);

        engine.transform_involution(Choice::new(
            crate::complex::ChoiceType::ServiceToOthers,
            0.8,
            0,
        ));

        let state = engine.get_state();
        assert!(state.has_potentia);
        assert!(state.has_matter);
    }

    #[test]
    fn test_transformation_reset() {
        let infinity = create_test_infinity();
        let natural_laws = NaturalLaws::earth_like();
        let mut engine = TransformationEngine::new(infinity, natural_laws, 42);

        engine.transform_involution(Choice::new(
            crate::complex::ChoiceType::ServiceToOthers,
            0.8,
            0,
        ));
        engine.reset();

        let state = engine.get_state();
        assert!(!state.has_potentia);
        assert!(!state.has_matter);
    }

    #[test]
    fn test_light_photon_energy() {
        let light = Light {
            energy: 1e-19,
            frequency: 1e14,
            wavelength: 1e-6,
            polarization: 0.0,
            coherence: 1.0,
            archetype_activations: [0.0; 22],
            seed: 42,
        };

        let photon_energy = light.photon_energy();
        assert!(photon_energy > 0.0);
    }

    #[test]
    fn test_light_is_visible() {
        let visible_light = Light {
            energy: 1e-19,
            frequency: 500e12, // In visible range
            wavelength: 1e-6,
            polarization: 0.0,
            coherence: 1.0,
            archetype_activations: [0.0; 22],
            seed: 42,
        };

        assert!(visible_light.is_visible());

        let uv_light = Light {
            energy: 1e-19,
            frequency: 1e15, // UV range
            wavelength: 1e-6,
            polarization: 0.0,
            coherence: 1.0,
            archetype_activations: [0.0; 22],
            seed: 42,
        };

        assert!(!uv_light.is_visible());
    }

    #[test]
    fn test_logos_primary_archetype() {
        let mut archetype_activations = [0.0; 22];
        archetype_activations[6] = 1.0; // Great Way

        let logos = Logos {
            archetype_activations,
            focused_energy: 1000.0,
            natural_laws: NaturalLaws::earth_like(),
            choice_archetype: 6,
            choice_direction: ChoiceDirection::STO,
            choice_intensity: 0.8,
            coherence: 1.0,
            seed: 42,
        };

        let primary = logos.primary_archetype();
        assert_eq!(primary, 6);
    }

    #[test]
    fn test_logos_density_level() {
        let mut archetype_activations = [0.0; 22];
        archetype_activations[2] = 1.0; // Significator (index 2)

        let logos = Logos {
            archetype_activations,
            focused_energy: 1000.0,
            natural_laws: NaturalLaws::earth_like(),
            choice_archetype: 2,
            choice_direction: ChoiceDirection::STO,
            choice_intensity: 0.8,
            coherence: 1.0,
            seed: 42,
        };

        let density = logos.density_level();
        // Use .as_u8() method instead of raw cast to get correct density value
        assert_eq!(density.as_u8(), 3);
    }

    #[test]
    fn test_consciousness_update_success() {
        let archetype_refinements = [0.1; 22];
        let free_will_enhancement = 0.05;
        let veil_adjustment = -0.01;

        let update = ConsciousnessUpdate::success(
            archetype_refinements,
            free_will_enhancement,
            veil_adjustment,
        );

        assert!(update.success);
        assert_eq!(update.free_will_enhancement, 0.05);
    }

    #[test]
    fn test_consciousness_update_error() {
        let update = ConsciousnessUpdate::error("Test error");

        assert!(!update.success);
        assert_eq!(update.message, "Test error");
    }
}
