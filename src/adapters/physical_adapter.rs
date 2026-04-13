// Physical Adapter - Phase 4: Physical Manifestation Adapter
//
// This adapter connects the NEW architecture (V3.0) to the OLD physical system (V2.0).
// It translates entity states from the NEW architecture to physical representations
// that can be used by the OLD matter module.
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 4:
// "Create adapter that connects NEW architecture to OLD physical system"
//
// Key Principles:
// 1. Adapter layer should be thin and focused
// 2. Translation mappings should be well-documented
// 3. Backward compatibility maintained during migration
// 4. Eventually replaced by NEW physical system
//
// Translation Mappings:
// - Density levels → Physical scales (quantum → atomic → molecular → cellular)
// - Spectrum configurations → Physical properties (mass, charge, spin)
// - Vibrational states → Energy levels
// - Polarity states → Charge types

use crate::entity_layer7::layer7::{
    IndividualSpectrumConfiguration, PolarityState, SubSubLogos, VibrationalState,
};
use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::matter::{Coordinate3D, Particle, Vector3D};
use crate::types::Float;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert EntityId to u64 (simplified hash)
pub fn entity_id_to_u64(entity_id: &crate::entity_layer7::layer7::EntityId) -> u64 {
    let mut hasher = DefaultHasher::new();
    entity_id.uuid.hash(&mut hasher);
    entity_id.incarnation_number.hash(&mut hasher);
    hasher.finish()
}
use std::collections::HashMap;

// ============================================================================
// PHYSICAL SCALES
// ============================================================================

/// Physical scale corresponding to density level
///
/// Maps density levels to physical manifestation scales:
/// - 1st Density: Quantum scale (particles, wavefunctions)
/// - 2nd Density: Atomic scale (atoms, molecules)
/// - 3rd Density: Cellular scale (biological life)
/// - 4th Density: Planetary scale (conscious entities)
/// - 5th Density: Solar scale (wisdom beings)
/// - 6th Density: Galactic scale (unity/balance)
/// - 7th Density: Universal scale (completion)
/// - 8th Density: Infinite scale (return to source)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalScale {
    /// Quantum scale (subatomic particles, wavefunctions)
    Quantum,
    /// Atomic scale (atoms, molecules)
    Atomic,
    /// Molecular scale (complex molecules, crystals)
    Molecular,
    /// Cellular scale (biological life)
    Cellular,
    /// Planetary scale (conscious entities)
    Planetary,
    /// Solar scale (wisdom beings)
    Solar,
    /// Galactic scale (unity/balance)
    Galactic,
    /// Universal scale (completion)
    Universal,
    /// Infinite scale (return to source)
    Infinite,
}

impl PhysicalScale {
    /// Map density level to physical scale
    pub fn from_density(density: &Density) -> Self {
        match density {
            Density::First(sub_level) => match sub_level {
                Density1SubLevel::Quantum => PhysicalScale::Quantum,
                Density1SubLevel::Atomic => PhysicalScale::Atomic,
                Density1SubLevel::Molecular => PhysicalScale::Molecular,
                Density1SubLevel::Planetary => PhysicalScale::Planetary,
            },
            Density::Second(sub_level) => match sub_level {
                Density2SubLevel::Cellular => PhysicalScale::Cellular,
                Density2SubLevel::SimpleLife => PhysicalScale::Cellular,
                Density2SubLevel::ComplexLife => PhysicalScale::Cellular,
            },
            Density::Third => PhysicalScale::Molecular,
            Density::Fourth => PhysicalScale::Cellular,
            Density::Fifth => PhysicalScale::Planetary,
            Density::Sixth => PhysicalScale::Solar,
            Density::Seventh => PhysicalScale::Galactic,
            Density::Eighth => PhysicalScale::Universal,
        }
    }

    /// Get characteristic size for this scale (in meters)
    pub fn characteristic_size(&self) -> Float {
        match self {
            PhysicalScale::Quantum => 1e-15,  // femtometer
            PhysicalScale::Atomic => 1e-10,   // angstrom
            PhysicalScale::Molecular => 1e-9, // nanometer
            PhysicalScale::Cellular => 1e-6,  // micrometer
            PhysicalScale::Planetary => 1e7,  // Earth radius
            PhysicalScale::Solar => 1e9,      // Solar radius
            PhysicalScale::Galactic => 1e21,  // Galaxy radius
            PhysicalScale::Universal => 1e26, // Observable universe
            PhysicalScale::Infinite => Float::INFINITY,
        }
    }

    /// Get characteristic time for this scale (in seconds)
    pub fn characteristic_time(&self) -> Float {
        match self {
            PhysicalScale::Quantum => 1e-21,     // Planck time
            PhysicalScale::Atomic => 1e-16,      // Atomic time
            PhysicalScale::Molecular => 1e-12,   // Molecular vibration
            PhysicalScale::Cellular => 1e-3,     // Millisecond
            PhysicalScale::Planetary => 3.15e7,  // Year
            PhysicalScale::Solar => 3.15e11,     // 10,000 years
            PhysicalScale::Galactic => 3.15e16,  // Billion years
            PhysicalScale::Universal => 3.15e17, // Age of universe
            PhysicalScale::Infinite => Float::INFINITY,
        }
    }

    /// Get characteristic energy for this scale (in Joules)
    pub fn characteristic_energy(&self) -> Float {
        match self {
            PhysicalScale::Quantum => 1.6e-19, // Electron volt
            PhysicalScale::Atomic => 2.2e-18,  // Chemical bond
            PhysicalScale::Molecular => 1e-20, // Thermal energy
            PhysicalScale::Cellular => 1e-12,  // ATP hydrolysis
            PhysicalScale::Planetary => 1e32,  // Planetary rotation
            PhysicalScale::Solar => 1e44,      // Solar output
            PhysicalScale::Galactic => 1e52,   // Galaxy luminosity
            PhysicalScale::Universal => 1e69,  // Universe total
            PhysicalScale::Infinite => Float::INFINITY,
        }
    }
}

// ============================================================================
// ENERGY LEVELS
// ============================================================================

/// Energy level corresponding to vibrational state
///
/// Vibrational states map to energy levels in the physical system:
/// - Low vibration → Low energy (ground states)
/// - Medium vibration → Medium energy (excited states)
/// - High vibration → High energy (ionized states)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyLevel {
    /// Ground state (lowest energy)
    Ground,
    /// Low energy state
    Low,
    /// Medium energy state
    Medium,
    /// High energy state
    High,
    /// Very high energy state
    VeryHigh,
    /// Ionized state
    Ionized,
    /// Infinite energy (return to source)
    Infinite,
}

impl EnergyLevel {
    /// Map vibrational state to energy level
    pub fn from_vibrational_state(vibrational_state: &VibrationalState) -> Self {
        // Use frequency as vibration level
        let vibration = vibrational_state.frequency;

        if vibration < 0.2 {
            EnergyLevel::Ground
        } else if vibration < 0.4 {
            EnergyLevel::Low
        } else if vibration < 0.6 {
            EnergyLevel::Medium
        } else if vibration < 0.8 {
            EnergyLevel::High
        } else if vibration < 0.95 {
            EnergyLevel::VeryHigh
        } else if vibration < 1.0 {
            EnergyLevel::Ionized
        } else {
            EnergyLevel::Infinite
        }
    }

    /// Get energy value for this level (in Joules)
    pub fn energy_value(&self) -> Float {
        match self {
            EnergyLevel::Ground => 0.0,
            EnergyLevel::Low => 1.6e-19,
            EnergyLevel::Medium => 3.2e-19,
            EnergyLevel::High => 6.4e-19,
            EnergyLevel::VeryHigh => 1.28e-18,
            EnergyLevel::Ionized => 2.56e-18,
            EnergyLevel::Infinite => Float::INFINITY,
        }
    }

    /// Get excitation level (0.0 to 1.0)
    pub fn excitation_level(&self) -> Float {
        match self {
            EnergyLevel::Ground => 0.0,
            EnergyLevel::Low => 0.2,
            EnergyLevel::Medium => 0.4,
            EnergyLevel::High => 0.6,
            EnergyLevel::VeryHigh => 0.8,
            EnergyLevel::Ionized => 0.95,
            EnergyLevel::Infinite => 1.0,
        }
    }
}

// ============================================================================
// CHARGE TYPES
// ============================================================================

/// Charge type corresponding to polarity state
///
/// Polarity states map to charge types:
/// - STO (Service to Others) → Positive charge (attractive)
/// - STS (Service to Self) → Negative charge (repulsive)
/// - Unpolarized → Neutral charge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChargeType {
    /// Positive charge (STO polarity - attractive)
    Positive,
    /// Negative charge (STS polarity - repulsive)
    Negative,
    /// Neutral charge (unpolarized)
    Neutral,
    /// Mixed charge (transitioning)
    Mixed,
}

impl ChargeType {
    /// Map polarity state to charge type
    pub fn from_polarity_state(polarity_state: &PolarityState) -> Self {
        // Use polarity_bias to determine charge type
        // Positive bias = STO = Positive charge
        // Negative bias = STS = Negative charge
        // Zero bias = Unpolarized = Neutral charge

        let bias = polarity_state.polarity_bias;
        let strength = polarity_state.polarization_strength;

        if bias > 0.3 && strength > 0.7 {
            ChargeType::Positive
        } else if bias < -0.3 && strength > 0.7 {
            ChargeType::Negative
        } else if strength < 0.3 {
            ChargeType::Neutral
        } else {
            ChargeType::Mixed
        }
    }

    /// Get charge value (in elementary charge units)
    pub fn charge_value(&self) -> Float {
        match self {
            ChargeType::Positive => 1.0,
            ChargeType::Negative => -1.0,
            ChargeType::Neutral => 0.0,
            ChargeType::Mixed => 0.5,
        }
    }

    /// Get interaction strength (0.0 to 1.0)
    pub fn interaction_strength(&self) -> Float {
        match self {
            ChargeType::Positive => 1.0,
            ChargeType::Negative => 1.0,
            ChargeType::Neutral => 0.0,
            ChargeType::Mixed => 0.5,
        }
    }
}

// ============================================================================
// PHYSICAL PROPERTIES
// ============================================================================

/// Physical properties derived from spectrum configuration
///
/// These properties emerge from the spectrum configuration and determine
/// how the entity manifests in physical reality.
#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumPhysicalProperties {
    /// Mass (in kg)
    pub mass: Float,

    /// Charge (in elementary charge units)
    pub charge: Float,

    /// Spin (in units of ħ)
    pub spin: Float,

    /// Magnetic moment (in Bohr magnetons)
    pub magnetic_moment: Float,

    /// Lifetime (in seconds, None for stable)
    pub lifetime: Option<Float>,

    /// Cross-section (in m²)
    pub cross_section: Float,

    /// Binding energy (in Joules)
    pub binding_energy: Float,
}

impl SpectrumPhysicalProperties {
    /// Create physical properties from spectrum configuration
    pub fn from_spectrum_configuration(
        spectrum_configuration: &IndividualSpectrumConfiguration,
    ) -> Self {
        // Calculate properties based on spectrum ratios
        let ratio = &spectrum_configuration.ratio;
        let oneness_ratio = ratio.calculate_ratio();
        let separation_ratio = 1.0 / oneness_ratio;

        // Mass emerges from total spectrum activation
        let mass = Self::calculate_mass(oneness_ratio, separation_ratio);

        // Charge emerges from imbalance between oneness and separation
        let charge = Self::calculate_charge(oneness_ratio, separation_ratio);

        // Spin emerges from rotational asymmetry in spectrum
        let spin = Self::calculate_spin(oneness_ratio);

        // Magnetic moment emerges from spin and charge
        let magnetic_moment = Self::calculate_magnetic_moment(spin, charge);

        // Lifetime emerges from stability of spectrum configuration
        let lifetime = Self::calculate_lifetime(oneness_ratio);

        // Cross-section emerges from interaction probability
        let cross_section = Self::calculate_cross_section(mass, charge);

        // Binding energy emerges from total spectrum energy
        let binding_energy = Self::calculate_binding_energy(oneness_ratio, separation_ratio);

        SpectrumPhysicalProperties {
            mass,
            charge,
            spin,
            magnetic_moment,
            lifetime,
            cross_section,
            binding_energy,
        }
    }

    /// Calculate mass from spectrum ratios
    fn calculate_mass(oneness_ratio: Float, separation_ratio: Float) -> Float {
        // Mass is proportional to total spectrum activation
        let total_activation = oneness_ratio + separation_ratio;

        // Base mass scale (electron mass)
        const ELECTRON_MASS: Float = 9.10938356e-31;

        // Mass scales with total activation
        ELECTRON_MASS * total_activation * 1000.0
    }

    /// Calculate charge from spectrum ratios
    fn calculate_charge(oneness_ratio: Float, separation_ratio: Float) -> Float {
        // Charge emerges from imbalance
        let imbalance = oneness_ratio - separation_ratio;

        // Charge is proportional to imbalance
        // Positive imbalance → positive charge (STO)
        // Negative imbalance → negative charge (STS)
        // Clamp to reasonable range (-10 to 10 elementary charges)
        (imbalance * 10.0).clamp(-10.0, 10.0)
    }

    /// Calculate spin from oneness ratio
    fn calculate_spin(oneness_ratio: Float) -> Float {
        // Spin emerges from rotational asymmetry
        // Oneness ratio determines asymmetry
        let asymmetry = 1.0 - oneness_ratio;

        // Spin is half-integer for fermions, integer for bosons
        if asymmetry > 0.5 {
            0.5 // Fermion
        } else if asymmetry > 0.0 {
            1.0 // Boson
        } else {
            0.0 // Scalar
        }
    }

    /// Calculate magnetic moment from spin and charge
    fn calculate_magnetic_moment(spin: Float, charge: Float) -> Float {
        // Magnetic moment = g-factor * spin * charge
        const G_FACTOR: Float = 2.0;
        G_FACTOR * spin * charge.abs()
    }

    /// Calculate lifetime from oneness ratio
    fn calculate_lifetime(oneness_ratio: Float) -> Option<Float> {
        // High oneness ratio → stable (infinite lifetime)
        // Low oneness ratio → unstable (finite lifetime)
        if oneness_ratio > 0.8 {
            None // Stable
        } else {
            // Lifetime scales inversely with instability
            let instability = 1.0 - oneness_ratio;
            Some(1e-15 / instability) // Very short lifetime
        }
    }

    /// Calculate cross-section from mass and charge
    fn calculate_cross_section(mass: Float, charge: Float) -> Float {
        // Cross-section emerges from interaction probability
        // Larger mass and charge → larger cross-section
        let mass_factor = (mass / 1e-30).ln();
        let charge_factor = charge.abs().ln() + 1.0;
        1e-28 * mass_factor * charge_factor
    }

    /// Calculate binding energy from spectrum ratios
    fn calculate_binding_energy(oneness_ratio: Float, separation_ratio: Float) -> Float {
        // Binding energy emerges from total spectrum energy
        const BASE_ENERGY: Float = 1.6e-19; // Electron volt
        let total_activation = oneness_ratio + separation_ratio;
        BASE_ENERGY * total_activation * 10.0
    }
}

impl From<&SpectrumPhysicalProperties> for crate::physics::archetype_physics::PhysicalProperties {
    fn from(spectrum: &SpectrumPhysicalProperties) -> Self {
        Self {
            mass: spectrum.mass,
            charge: spectrum.charge,
            spin: spectrum.spin,
            damping: 0.0,
            moment_of_inertia: 0.4 * spectrum.mass,
            force_vector: crate::physics::archetype_physics::Force3D::new(0.0, 0.0, 0.0),
        }
    }
}

// ============================================================================
// PHYSICAL ENTITY
// ============================================================================

/// Physical entity representation for OLD architecture
///
/// This is the physical manifestation of a NEW architecture entity,
/// translated into a format that the OLD matter module can use.
#[derive(Debug, Clone)]
pub struct PhysicalEntity {
    /// Entity ID (from NEW architecture)
    pub entity_id: u64, // Simplified for adapter

    /// Physical scale (quantum, atomic, molecular, etc.)
    pub physical_scale: PhysicalScale,

    /// Physical properties (mass, charge, spin, etc.)
    pub physical_properties: SpectrumPhysicalProperties,

    /// Energy level (ground, low, medium, high, etc.)
    pub energy_level: EnergyLevel,

    /// Charge type (positive, negative, neutral, mixed)
    pub charge_type: ChargeType,

    /// Position in 3D space
    pub position: Coordinate3D,

    /// Velocity vector
    pub velocity: Vector3D,

    /// Particle representation (for OLD matter module)
    pub particle: Option<Particle>,
}

impl PhysicalEntity {
    /// Create a physical entity from a NEW architecture entity
    pub fn from_entity(entity: &SubSubLogos) -> Self {
        // Convert EntityId to u64 (use hash for simplicity)
        let entity_id_u64 = entity_id_to_u64(&entity.entity_id);

        // Get entity state
        let current_state = &entity.current_state;

        // Use a default density since EntityState doesn't have current_density field
        // In a real implementation, this would come from the entity's evolutionary state
        let default_density = Density::Third; // Default to molecular scale

        // Map density to physical scale
        let physical_scale = PhysicalScale::from_density(&default_density);

        // Map spectrum configuration to physical properties
        let physical_properties =
            SpectrumPhysicalProperties::from_spectrum_configuration(&entity.spectrum_configuration);

        // Map vibrational state to energy level
        let energy_level = EnergyLevel::from_vibrational_state(&current_state.vibrational_state);

        // Map polarity state to charge type
        let charge_type = ChargeType::from_polarity_state(&current_state.polarity_state);

        // Initialize position at origin
        let position = Coordinate3D::origin();

        // Initialize velocity as zero
        let velocity = Vector3D::zero();

        // Create particle representation (for OLD matter module)
        let particle = Self::create_particle_representation(
            entity_id_u64,
            &physical_properties,
            &charge_type,
            position,
        );

        PhysicalEntity {
            entity_id: entity_id_u64,
            physical_scale,
            physical_properties,
            energy_level,
            charge_type,
            position,
            velocity,
            particle,
        }
    }

    /// Convert EntityId to u64 (simplified hash)
    pub fn entity_id_to_u64(entity_id: &crate::entity_layer7::layer7::EntityId) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        entity_id.uuid.hash(&mut hasher);
        entity_id.incarnation_number.hash(&mut hasher);
        hasher.finish()
    }

    /// Create particle representation for OLD matter module
    fn create_particle_representation(
        entity_id: u64,
        physical_properties: &SpectrumPhysicalProperties,
        charge_type: &ChargeType,
        position: Coordinate3D,
    ) -> Option<Particle> {
        // Create archetype activation pattern from physical properties
        let archetype_activation =
            Self::physical_properties_to_archetype_activation(physical_properties, charge_type);

        // Create particle
        let particle = Particle::from_archetype_activation(
            entity_id as crate::matter::ParticleID,
            archetype_activation,
            position,
        );

        Some(particle)
    }

    /// Convert physical properties to archetype activation pattern
    fn physical_properties_to_archetype_activation(
        physical_properties: &SpectrumPhysicalProperties,
        charge_type: &ChargeType,
    ) -> [Float; 22] {
        let mut activation = [0.0; 22];

        // Mass influences Matrix, Potentiator, Catalyst archetypes
        let mass_factor = (physical_properties.mass / 1e-30).ln().clamp(0.0, 1.0);
        activation[0] = mass_factor; // A1: Matrix
        activation[7] = mass_factor; // A8: Matrix
        activation[14] = mass_factor; // A15: Matrix

        // Charge influences Catalyst archetypes
        let charge_factor = physical_properties.charge.abs().clamp(0.0, 1.0);
        activation[2] = charge_factor * charge_type.charge_value().signum(); // A3: Catalyst
        activation[9] = charge_factor * charge_type.charge_value().signum(); // A10: Catalyst
        activation[16] = charge_factor * charge_type.charge_value().signum(); // A17: Catalyst

        // Spin influences Significator archetypes
        let spin_factor = physical_properties.spin;
        activation[4] = spin_factor; // A5: Significator
        activation[11] = spin_factor; // A12: Significator
        activation[18] = spin_factor; // A19: Significator

        // Great Way archetypes (stability)
        activation[6] = 0.95; // A7: Great Way
        activation[13] = 0.95; // A14: Great Way
        activation[20] = 0.95; // A21: Great Way

        activation
    }

    /// Update physical entity state
    pub fn update(&mut self, dt: Float) {
        // Update position based on velocity
        self.position.x += self.velocity.vx * dt;
        self.position.y += self.velocity.vy * dt;
        self.position.z += self.velocity.vz * dt;

        // Update particle if present
        if let Some(ref mut particle) = self.particle {
            particle.update_position(dt);
        }
    }

    /// Get total energy
    pub fn total_energy(&self) -> Float {
        const C: Float = 2.998e8; // Speed of light in m/s

        // Rest energy: E = mc²
        let rest_energy = self.physical_properties.mass * C * C;

        // Kinetic energy: E = ½mv²
        let velocity_squared = self.velocity.vx * self.velocity.vx
            + self.velocity.vy * self.velocity.vy
            + self.velocity.vz * self.velocity.vz;
        let kinetic_energy = 0.5 * self.physical_properties.mass * velocity_squared;

        // Excitation energy
        let excitation_energy = self.energy_level.energy_value();

        // Total energy
        rest_energy + kinetic_energy + excitation_energy
    }

    /// Get momentum
    pub fn momentum(&self) -> Vector3D {
        Vector3D {
            vx: self.physical_properties.mass * self.velocity.vx,
            vy: self.physical_properties.mass * self.velocity.vy,
            vz: self.physical_properties.mass * self.velocity.vz,
        }
    }
}

// ============================================================================
// TRANSLATION MAPPINGS
// ============================================================================

/// Translation mappings between NEW and OLD architectures
///
/// These mappings define how to translate between the two architectures.
/// They are used by the PhysicalAdapter to convert entity states.
#[derive(Debug, Clone)]
pub struct TranslationMappings {
    /// Map spectrum configurations to physical properties
    pub spectrum_to_physical: HashMap<String, SpectrumPhysicalProperties>,
}

impl TranslationMappings {
    /// Create default translation mappings
    pub fn new() -> Self {
        TranslationMappings {
            spectrum_to_physical: HashMap::new(),
        }
    }

    /// Add spectrum configuration mapping
    pub fn add_spectrum_mapping(
        &mut self,
        spectrum_config: &IndividualSpectrumConfiguration,
        physical_properties: SpectrumPhysicalProperties,
    ) {
        let ratio = &spectrum_config.ratio;
        let key = format!(
            "{:.2}_{:.2}",
            ratio.calculate_ratio(),
            1.0 / ratio.calculate_ratio()
        );
        self.spectrum_to_physical.insert(key, physical_properties);
    }
}

impl Default for TranslationMappings {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// POLARITY TO CHARGE MAPPING
// ============================================================================

/// Detailed mapping from polarity state to charge type
///
/// This provides more detailed information about how polarity maps to charge.
#[derive(Debug, Clone)]
pub struct PolarityToChargeMapping {
    /// Polarity state
    pub polarity_state: PolarityState,

    /// Resulting charge type
    pub charge_type: ChargeType,

    /// Charge value (in elementary charge units)
    pub charge_value: Float,

    /// Interaction strength (0.0 to 1.0)
    pub interaction_strength: Float,
}

impl PolarityToChargeMapping {
    /// Create mapping from polarity state
    pub fn from_polarity_state(polarity_state: &PolarityState) -> Self {
        let charge_type = ChargeType::from_polarity_state(polarity_state);
        let charge_value = charge_type.charge_value();
        let interaction_strength = charge_type.interaction_strength();

        PolarityToChargeMapping {
            polarity_state: polarity_state.clone(),
            charge_type,
            charge_value,
            interaction_strength,
        }
    }
}

// ============================================================================
// PHYSICAL ADAPTER
// ============================================================================

/// Physical Adapter - bridges NEW architecture to OLD physical system
///
/// This adapter translates entity states from the NEW architecture (V3.0)
/// to physical representations that can be used by the OLD matter module (V2.0).
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 4:
/// "Create adapter that connects NEW architecture to OLD physical system"
///
/// # Example
///
/// ```rust
/// use crate::adapters::physical_adapter::PhysicalAdapter;
/// use crate::entity_layer7::layer7::SubSubLogos;
///
/// // Create adapter
/// let mut adapter = PhysicalAdapter::new();
///
/// // Add entities from NEW architecture
/// adapter.add_entity(entity1);
/// adapter.add_entity(entity2);
///
/// // Connect to OLD physical system
/// adapter.connect_to_physical_system();
///
/// // Update physical manifestations
/// adapter.update_physical_manifestations(1e-15);
///
/// // Get translation mappings
/// let mappings = adapter.get_translation_mappings();
/// ```
#[derive(Debug, Clone)]
pub struct PhysicalAdapter {
    /// Entity storage (NEW architecture) - indexed by u64 hash of EntityId
    pub entities: HashMap<u64, SubSubLogos>,

    /// Physical manifestations (OLD architecture)
    pub physical_manifestations: HashMap<u64, PhysicalEntity>,

    /// Translation mappings
    pub translation_mappings: TranslationMappings,

    /// Connection status
    pub connected: bool,

    /// Statistics
    pub statistics: PhysicalAdapterStatistics,
}

/// Statistics for physical adapter
#[derive(Debug, Clone, Default)]
pub struct PhysicalAdapterStatistics {
    /// Total entities
    pub total_entities: usize,

    /// Total physical manifestations
    pub total_manifestations: usize,

    /// Translation cache hits
    pub cache_hits: usize,

    /// Translation cache misses
    pub cache_misses: usize,

    /// Average translation time (in nanoseconds)
    pub avg_translation_time: u64,
}

impl PhysicalAdapter {
    /// Create a new physical adapter
    pub fn new() -> Self {
        PhysicalAdapter {
            entities: HashMap::new(),
            physical_manifestations: HashMap::new(),
            translation_mappings: TranslationMappings::new(),
            connected: false,
            statistics: PhysicalAdapterStatistics::default(),
        }
    }

    /// Add an entity from NEW architecture
    pub fn add_entity(&mut self, entity: SubSubLogos) {
        // Convert EntityId to u64 hash
        let entity_id_u64 = entity_id_to_u64(&entity.entity_id);

        // Store entity
        self.entities.insert(entity_id_u64, entity.clone());

        // Create physical manifestation
        let physical_entity = PhysicalEntity::from_entity(&entity);

        // Cache spectrum configuration mapping
        let physical_properties = physical_entity.physical_properties.clone();
        self.translation_mappings
            .add_spectrum_mapping(&entity.spectrum_configuration, physical_properties);

        // Store physical manifestation
        self.physical_manifestations
            .insert(entity_id_u64, physical_entity);

        // Update statistics
        self.statistics.total_entities = self.entities.len();
        self.statistics.total_manifestations = self.physical_manifestations.len();
    }

    /// Translate entity to physical representation
    pub fn translate_entity_to_physical(&self, entity: &SubSubLogos) -> PhysicalEntity {
        PhysicalEntity::from_entity(entity)
    }

    /// Translate entity state to physical properties
    pub fn translate_entity_state_to_properties(
        &self,
        entity: &SubSubLogos,
    ) -> SpectrumPhysicalProperties {
        SpectrumPhysicalProperties::from_spectrum_configuration(&entity.spectrum_configuration)
    }

    /// Translate density to physical scale
    pub fn translate_density_to_physical(&self, density: &Density) -> PhysicalScale {
        PhysicalScale::from_density(density)
    }

    /// Translate spectrum configuration to physical properties
    pub fn translate_spectrum_to_physical(
        &self,
        spectrum_config: &IndividualSpectrumConfiguration,
    ) -> SpectrumPhysicalProperties {
        SpectrumPhysicalProperties::from_spectrum_configuration(spectrum_config)
    }

    /// Translate vibrational state to energy level
    pub fn translate_vibrational_to_energy(
        &self,
        vibrational_state: &VibrationalState,
    ) -> EnergyLevel {
        EnergyLevel::from_vibrational_state(vibrational_state)
    }

    /// Translate polarity state to charge type
    pub fn translate_polarity_to_charge(&self, polarity_state: &PolarityState) -> ChargeType {
        ChargeType::from_polarity_state(polarity_state)
    }

    /// Connect to OLD physical system
    pub fn connect_to_physical_system(&mut self) {
        // In a real implementation, this would establish connections
        // to the OLD matter module and physical systems.

        // For now, we just mark as connected
        self.connected = true;
    }

    /// Disconnect from OLD physical system
    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    /// Check if connected to OLD physical system
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Update physical manifestations
    pub fn update_physical_manifestations(&mut self, dt: Float) {
        for physical_entity in self.physical_manifestations.values_mut() {
            physical_entity.update(dt);
        }
    }

    /// Get physical entity by ID
    pub fn get_physical_entity(&self, entity_id: u64) -> Option<&PhysicalEntity> {
        self.physical_manifestations.get(&entity_id)
    }

    /// Get physical entity by ID (mutable)
    pub fn get_physical_entity_mut(&mut self, entity_id: u64) -> Option<&mut PhysicalEntity> {
        self.physical_manifestations.get_mut(&entity_id)
    }

    /// Get all physical entities
    pub fn get_all_physical_entities(&self) -> Vec<&PhysicalEntity> {
        self.physical_manifestations.values().collect()
    }

    /// Get translation mappings
    pub fn get_translation_mappings(&self) -> &TranslationMappings {
        &self.translation_mappings
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &PhysicalAdapterStatistics {
        &self.statistics
    }

    /// Get summary
    pub fn get_summary(&self) -> PhysicalAdapterSummary {
        PhysicalAdapterSummary {
            total_entities: self.statistics.total_entities,
            total_manifestations: self.statistics.total_manifestations,
            connected: self.connected,
            cache_hit_rate: if self.statistics.cache_hits + self.statistics.cache_misses > 0 {
                self.statistics.cache_hits as Float
                    / (self.statistics.cache_hits + self.statistics.cache_misses) as Float
            } else {
                0.0
            },
        }
    }
}

impl Default for PhysicalAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PHYSICAL ADAPTER SUMMARY
// ============================================================================

/// Summary of physical adapter state
#[derive(Debug, Clone)]
pub struct PhysicalAdapterSummary {
    /// Total entities
    pub total_entities: usize,

    /// Total physical manifestations
    pub total_manifestations: usize,

    /// Connection status
    pub connected: bool,

    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: Float,
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::IndividualSpectrumConfiguration;
    use crate::evolution_density_octave::density_octave::{
        Density, Density1SubLevel, Density2SubLevel,
    };

    // Helper function to create a test entity
    fn create_test_entity(entity_id: u64) -> SubSubLogos {
        use crate::entity_layer7::layer7::EntityId;
        use crate::foundation::{
            IntelligentInfinity as IndigoRealm, LightLoveField as GreenRealm, Logos as BlueRealm,
            VioletRealm,
        };
        use crate::spectrum::{OrangeRealm, RedRealm, YellowRealm};

        // Create all required realms (simplified)
        let violet_realm = VioletRealm::new();
        let indigo_realm = IndigoRealm::new();
        let blue_realm = BlueRealm::new();
        let green_realm = GreenRealm::new();

        // Yellow Realm: Space/Time and Time/Space spectrum
        let yellow_realm = YellowRealm::new(green_realm.clone());

        // Orange Realm: Galactic-scale configuration
        let orange_realm = OrangeRealm::new(yellow_realm.clone());

        // Red Realm: Solar-scale configuration
        let red_realm = RedRealm::new(orange_realm.clone());

        // Create spectrum configuration
        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.6, SpectrumSide::SpaceTime);
        let spectrum_configuration = IndividualSpectrumConfiguration::new(spectrum_ratios);

        // Create entity
        use crate::entity_layer7::layer7::EntityType;
        SubSubLogos::new(
            EntityId::new(format!("entity-{}", entity_id)),
            EntityType::Individual,
            None,       // parent_id
            Vec::new(), // composition
            None,       // environment_id
            violet_realm,
            indigo_realm,
            blue_realm,
            green_realm,
            yellow_realm,
            orange_realm,
            red_realm,
            spectrum_configuration,
        )
    }

    // ============================================================================
    // PhysicalScale Tests
    // ============================================================================

    #[test]
    fn test_physical_scale_from_density() {
        let density = Density::First(Density1SubLevel::Quantum);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Quantum);

        let density = Density::First(Density1SubLevel::Atomic);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Atomic);

        let density = Density::First(Density1SubLevel::Molecular);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Molecular);

        let density = Density::Second(Density2SubLevel::Cellular);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Cellular);
    }

    #[test]
    fn test_physical_scale_characteristic_size() {
        let quantum_size = PhysicalScale::Quantum.characteristic_size();
        assert!(quantum_size < 1e-14, "Quantum scale should be ~femtometer");

        let atomic_size = PhysicalScale::Atomic.characteristic_size();
        assert!(
            atomic_size > 1e-11 && atomic_size < 1e-9,
            "Atomic scale should be ~angstrom"
        );

        let cellular_size = PhysicalScale::Cellular.characteristic_size();
        assert!(
            cellular_size > 1e-7 && cellular_size < 1e-5,
            "Cellular scale should be ~micrometer"
        );
    }

    #[test]
    fn test_physical_scale_characteristic_energy() {
        let quantum_energy = PhysicalScale::Quantum.characteristic_energy();
        assert!(
            quantum_energy > 1e-20 && quantum_energy < 1e-18,
            "Quantum energy should be ~eV"
        );

        let cellular_energy = PhysicalScale::Cellular.characteristic_energy();
        assert!(
            cellular_energy > 1e-13 && cellular_energy < 1e-11,
            "Cellular energy should be ~pJ"
        );
    }

    // ============================================================================
    // EnergyLevel Tests
    // ============================================================================

    #[test]
    fn test_energy_level_from_vibrational_state() {
        use crate::evolution_density_octave::density_octave::Density;
        let vibrational_state = VibrationalState {
            frequency: 0.1,
            amplitude: 0.5,
            coherence: 0.5,
            density: Density::Third,
            kinetic_energy: 0.1,
            potential_energy: 0.1,
        };
        let energy_level = EnergyLevel::from_vibrational_state(&vibrational_state);
        assert_eq!(energy_level, EnergyLevel::Ground);

        let vibrational_state = VibrationalState {
            frequency: 0.5,
            amplitude: 0.5,
            coherence: 0.5,
            density: Density::Third,
            kinetic_energy: 0.5,
            potential_energy: 0.5,
        };
        let energy_level = EnergyLevel::from_vibrational_state(&vibrational_state);
        assert_eq!(energy_level, EnergyLevel::Medium);

        let vibrational_state = VibrationalState {
            frequency: 0.9,
            amplitude: 0.5,
            coherence: 0.5,
            density: Density::Third,
            kinetic_energy: 0.9,
            potential_energy: 0.9,
        };
        let energy_level = EnergyLevel::from_vibrational_state(&vibrational_state);
        assert_eq!(energy_level, EnergyLevel::VeryHigh);
    }

    #[test]
    fn test_energy_level_excitation() {
        let excitation = EnergyLevel::Ground.excitation_level();
        assert_eq!(excitation, 0.0);

        let excitation = EnergyLevel::Medium.excitation_level();
        assert_eq!(excitation, 0.4);

        let excitation = EnergyLevel::Ionized.excitation_level();
        assert_eq!(excitation, 0.95);
    }

    // ============================================================================
    // ChargeType Tests
    // ============================================================================

    #[test]
    fn test_charge_type_from_polarity_state() {
        let polarity_state = PolarityState {
            polarity_bias: 0.8,
            polarization_strength: 0.9,
        };
        let charge_type = ChargeType::from_polarity_state(&polarity_state);
        assert_eq!(charge_type, ChargeType::Positive);

        let polarity_state = PolarityState {
            polarity_bias: -0.8,
            polarization_strength: 0.9,
        };
        let charge_type = ChargeType::from_polarity_state(&polarity_state);
        assert_eq!(charge_type, ChargeType::Negative);

        let polarity_state = PolarityState {
            polarity_bias: 0.0,
            polarization_strength: 0.1,
        };
        let charge_type = ChargeType::from_polarity_state(&polarity_state);
        assert_eq!(charge_type, ChargeType::Neutral);
    }

    #[test]
    fn test_charge_type_interaction_strength() {
        let strength = ChargeType::Positive.interaction_strength();
        assert_eq!(strength, 1.0);

        let strength = ChargeType::Neutral.interaction_strength();
        assert_eq!(strength, 0.0);

        let strength = ChargeType::Mixed.interaction_strength();
        assert_eq!(strength, 0.5);
    }

    // ============================================================================
    // SpectrumPhysicalProperties Tests
    // ============================================================================

    #[test]
    fn test_physical_properties_from_spectrum_configuration() {
        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.6, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(spectrum_ratios);

        let properties = SpectrumPhysicalProperties::from_spectrum_configuration(&spectrum_config);

        assert!(properties.mass > 0.0, "Mass should be positive");
        assert!(
            properties.charge.abs() <= 10.0,
            "Charge should be reasonable"
        );
        assert!(properties.spin >= 0.0, "Spin should be non-negative");
    }

    #[test]
    fn test_physical_properties_binding_energy() {
        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.8, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(spectrum_ratios);

        let properties = SpectrumPhysicalProperties::from_spectrum_configuration(&spectrum_config);

        assert!(
            properties.binding_energy > 0.0,
            "Binding energy should be positive"
        );
        assert!(
            properties.binding_energy > 1e-18,
            "Binding energy should be significant"
        );
    }

    #[test]
    fn test_physical_properties_binding_energy_alternative_constructor() {
        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.8, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(spectrum_ratios);

        let properties = SpectrumPhysicalProperties::from_spectrum_configuration(&spectrum_config);

        assert!(
            properties.binding_energy > 0.0,
            "Binding energy should be positive"
        );
        assert!(
            properties.binding_energy > 1e-18,
            "Binding energy should be significant"
        );
    }

    // ============================================================================
    // PhysicalEntity Tests
    // ============================================================================

    #[test]
    fn test_physical_entity_from_entity() {
        let entity = create_test_entity(1);
        let physical_entity = PhysicalEntity::from_entity(&entity);

        // entity_id is a hash, not the original ID
        assert!(physical_entity.entity_id > 0);
        assert!(
            physical_entity.particle.is_some(),
            "Particle should be created"
        );
    }

    #[test]
    fn test_physical_entity_update() {
        let entity = create_test_entity(1);
        let mut physical_entity = PhysicalEntity::from_entity(&entity);

        let initial_position = physical_entity.position;
        physical_entity.velocity = Vector3D::new(1.0, 2.0, 3.0);

        physical_entity.update(1.0);

        assert_ne!(physical_entity.position, initial_position);
        assert_eq!(physical_entity.position.x, 1.0);
        assert_eq!(physical_entity.position.y, 2.0);
        assert_eq!(physical_entity.position.z, 3.0);
    }

    #[test]
    fn test_physical_entity_total_energy() {
        let entity = create_test_entity(1);
        let physical_entity = PhysicalEntity::from_entity(&entity);

        let total_energy = physical_entity.total_energy();
        assert!(total_energy > 0.0, "Total energy should be positive");
    }

    // ============================================================================
    // TranslationMappings Tests
    // ============================================================================

    #[test]
    fn test_translation_mappings_new() {
        let mappings = TranslationMappings::new();

        assert!(mappings.spectrum_to_physical.is_empty());
    }

    #[test]
    fn test_translation_mappings_add_spectrum_mapping() {
        let mut mappings = TranslationMappings::new();

        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.6, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(spectrum_ratios);
        let properties = SpectrumPhysicalProperties::from_spectrum_configuration(&spectrum_config);

        mappings.add_spectrum_mapping(&spectrum_config, properties);

        assert_eq!(mappings.spectrum_to_physical.len(), 1);
    }

    #[test]
    fn test_translation_mappings_add_spectrum_mapping_with_ratio() {
        let mut mappings = TranslationMappings::new();

        use crate::spectrum::larson_framework::{SpectrumRatio, SpectrumSide};
        let spectrum_ratios = SpectrumRatio::new(0.6, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(spectrum_ratios);
        let properties = SpectrumPhysicalProperties::from_spectrum_configuration(&spectrum_config);

        mappings.add_spectrum_mapping(&spectrum_config, properties);

        assert_eq!(mappings.spectrum_to_physical.len(), 1);
    }

    #[test]
    fn test_physical_scale_from_density_alternative_types() {
        let density = Density::First(Density1SubLevel::Quantum);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Quantum);

        let density = Density::Second(Density2SubLevel::Cellular);
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Cellular);

        let density = Density::Third;
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Molecular);

        let density = Density::Fourth;
        let scale = PhysicalScale::from_density(&density);
        assert_eq!(scale, PhysicalScale::Cellular);
    }

    // ============================================================================
    // PhysicalAdapter Tests
    // ============================================================================

    #[test]
    fn test_physical_adapter_new() {
        let adapter = PhysicalAdapter::new();

        assert!(!adapter.is_connected());
        assert_eq!(adapter.get_statistics().total_entities, 0);
    }

    #[test]
    fn test_physical_adapter_add_entity() {
        let mut adapter = PhysicalAdapter::new();
        let entity = create_test_entity(1);

        adapter.add_entity(entity);

        assert_eq!(adapter.get_statistics().total_entities, 1);
        assert_eq!(adapter.get_statistics().total_manifestations, 1);
    }

    #[test]
    fn test_physical_adapter_translate_entity_to_physical() {
        let adapter = PhysicalAdapter::new();
        let entity = create_test_entity(1);

        let physical_entity = adapter.translate_entity_to_physical(&entity);

        // entity_id will be a hash, not 1
        assert!(physical_entity.entity_id > 0);
    }

    #[test]
    fn test_physical_adapter_connect_to_physical_system() {
        let mut adapter = PhysicalAdapter::new();

        adapter.connect_to_physical_system();

        assert!(adapter.is_connected());
    }

    #[test]
    fn test_physical_adapter_disconnect() {
        let mut adapter = PhysicalAdapter::new();
        adapter.connect_to_physical_system();

        adapter.disconnect();

        assert!(!adapter.is_connected());
    }

    #[test]
    fn test_physical_adapter_update_physical_manifestations() {
        let mut adapter = PhysicalAdapter::new();
        let entity = create_test_entity(1);
        adapter.add_entity(entity);

        adapter.update_physical_manifestations(1e-15);
        // Test passes if no panic occurred
    }

    #[test]
    fn test_physical_adapter_get_physical_entity() {
        let mut adapter = PhysicalAdapter::new();
        let entity = create_test_entity(1);
        adapter.add_entity(entity.clone());

        // Get the physical entity ID from the created physical entity
        let entities = adapter.get_all_physical_entities();
        let physical_entity = entities.first().unwrap();
        let entity_id_hash = physical_entity.entity_id;

        let retrieved_entity = adapter.get_physical_entity(entity_id_hash);

        assert!(retrieved_entity.is_some());
        assert_eq!(retrieved_entity.unwrap().entity_id, entity_id_hash);
    }

    #[test]
    fn test_physical_adapter_get_summary() {
        let mut adapter = PhysicalAdapter::new();
        let entity = create_test_entity(1);
        adapter.add_entity(entity);

        let summary = adapter.get_summary();

        assert_eq!(summary.total_entities, 1);
        assert_eq!(summary.total_manifestations, 1);
        assert!(!summary.connected);
    }

    #[test]
    fn test_physical_adapter_multiple_entities() {
        let mut adapter = PhysicalAdapter::new();

        for i in 1..=10 {
            let entity = create_test_entity(i);
            adapter.add_entity(entity);
        }

        assert_eq!(adapter.get_statistics().total_entities, 10);
        assert_eq!(adapter.get_statistics().total_manifestations, 10);

        let all_entities = adapter.get_all_physical_entities();
        assert_eq!(all_entities.len(), 10);
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_full_translation_workflow() {
        // Create adapter
        let mut adapter = PhysicalAdapter::new();

        // Add entity
        let entity = create_test_entity(1);
        adapter.add_entity(entity);

        // Connect to physical system
        adapter.connect_to_physical_system();

        // Verify translation by getting the first physical entity
        let physical_entities = adapter.get_all_physical_entities();
        assert!(!physical_entities.is_empty());
        let physical_entity = physical_entities.first().unwrap();
        assert!(physical_entity.entity_id > 0);
        assert!(physical_entity.particle.is_some());

        // Update physical manifestations
        adapter.update_physical_manifestations(1e-15);

        // Get summary
        let summary = adapter.get_summary();
        assert_eq!(summary.total_entities, 1);
        assert!(summary.connected);
    }

    #[test]
    fn test_polarity_to_charge_mapping() {
        // PolarityState is a struct, not an enum - create with STO bias
        let polarity_state = PolarityState {
            polarity_bias: 0.8, // STO bias
            polarization_strength: 0.8,
        };
        let mapping = PolarityToChargeMapping::from_polarity_state(&polarity_state);

        assert_eq!(mapping.charge_type, ChargeType::Positive);
        assert_eq!(mapping.charge_value, 1.0);
        assert_eq!(mapping.interaction_strength, 1.0);
    }
}
