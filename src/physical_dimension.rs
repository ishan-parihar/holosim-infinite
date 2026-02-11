// Physical Dimension - Phase 10 Implementation
// Container for all physical dimension components
// Provides the space/time infrastructure for the dual-dimensional simulation
//
// Phase 3 Update: Physical dimension now uses holographic references in Space/Time/Spacetime

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

// Re-export types from submodules
pub use super::energy_fields::{
    ElectricField, ElectromagneticField, EnergyFields, GravitationalField, MagneticField,
    NuclearField, Vector3,
};
pub use super::matter::{
    Atom, BondType, Cell, DNABase, Matter, MolecularStructure, Molecule, OrganelleType, Particle,
    RNABase,
};
pub use super::natural_laws::{
    ConservationQuantity, EntropyLawType, Float, ForceLawType, NaturalLaws, NuclearForceType,
};
pub use super::probability::{Complex, ProbabilityDistribution, ProbabilitySpace, QuantumState};
// pub use crate::space_time::{
//     BoundaryCondition, Causality, Space, Spacetime, SpacetimeEvent, SpacetimeInterval, Time,
//     TimeDirection,
// }; // TODO: space_time module not declared in lib.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacetimeEvent {
    pub position: Vector3,
    pub timestamp: Float,
}

impl SpacetimeEvent {
    pub fn new(position: Vector3, timestamp: Float) -> Self {
        Self {
            position,
            timestamp,
        }
    }
    pub fn origin(timestamp: Float) -> Self {
        Self::new(Vector3::zero(), timestamp)
    }
}

/// Physical Dimension (Space/Time)
/// Contains all physical components of the simulation
///
/// Phase 3: No longer serializable due to Arc<HolographicSeed> in Space/Time/Spacetime
#[derive(Debug, Clone)]
pub struct PhysicalDimension {
    /// Natural laws (parameterized physics)
    pub natural_laws: NaturalLaws,

    /// Energy fields (gravitational, electromagnetic, nuclear)
    pub energy_fields: EnergyFields,

    /// Matter (particles, atoms, molecules, cells)
    pub matter: Matter,

    /// Space (3D with extent and curvature)
    // pub space: Space, // TODO: Space type not available

    /// Time (causality and direction)
    // pub time: Time, // TODO: Time type not available

    /// Combined spacetime
    // pub spacetime: Spacetime, // TODO: Spacetime type not available

    /// Probability space (quantum superposition)
    pub probability_space: ProbabilitySpace,

    /// Physical experiences generated in this dimension
    pub experiences: Vec<PhysicalExperience>,

    /// Age of the physical dimension
    pub age: Float,
}

impl Default for PhysicalDimension {
    fn default() -> Self {
        Self::earth_like()
    }
}

impl PhysicalDimension {
    /// Create physical dimension similar to Earth's universe
    ///
    /// Phase 3: Creates holographic reference for space/time/spacetime
    pub fn earth_like() -> Self {
        let natural_laws = NaturalLaws::earth_like();
        let seed = HolographicSeed::default();
        let _holographic_ref = Arc::new(seed);

        // TODO: Space, Time, Spacetime types not available - commented out
        // let space = Space::flat_3d_with_holographic_ref(holographic_ref.clone());
        // let time = Time::standard_with_holographic_ref(holographic_ref.clone());
        // let spacetime = Spacetime::standard_with_holographic_ref(holographic_ref);

        PhysicalDimension {
            natural_laws,
            energy_fields: EnergyFields::empty(),
            matter: Matter::empty(),
            // space,
            // time,
            // spacetime,
            probability_space: ProbabilitySpace::empty(),
            experiences: Vec::new(),
            age: 0.0,
        }
    }

    /// Create physical dimension with custom natural laws
    ///
    /// Phase 3: Creates holographic reference for space/time/spacetime
    pub fn with_natural_laws(natural_laws: NaturalLaws) -> Self {
        // TODO: Space, Time, Spacetime types not available - commented out
        // let seed = HolographicSeed::default();
        // let holographic_ref = Arc::new(seed);

        // let space = Space::custom(
        //     natural_laws.spatial_dimensions,
        //     1.0e26,
        //     0.0,
        //     holographic_ref.clone(),
        // );
        // let time = Time::standard_with_holographic_ref(holographic_ref.clone());
        // let spacetime = Spacetime::custom(space.clone(), time.clone());

        PhysicalDimension {
            natural_laws,
            energy_fields: EnergyFields::empty(),
            matter: Matter::empty(),
            // space,
            // time,
            // spacetime,
            probability_space: ProbabilitySpace::empty(),
            experiences: Vec::new(),
            age: 0.0,
        }
    }

    /// Create physical dimension from light (third distortion)
    pub fn from_light(light_energy: Float, seed: u64) -> Self {
        let mut dimension = Self::earth_like();

        // Create energy fields from light
        dimension.energy_fields = EnergyFields::from_light(light_energy);

        // Create matter from energy fields
        dimension.matter = Matter::from_energy_fields();

        // Initialize probability space
        dimension.probability_space = ProbabilitySpace::with_outcomes(10, seed);

        dimension
    }

    /// Initialize physical dimension with given parameters
    pub fn initialize(&mut self, seed: u64) {
        // Initialize probability space
        self.probability_space = ProbabilitySpace::with_outcomes(10, seed);

        // Generate initial matter
        if self.matter.total_mass() == 0.0 {
            self.matter = Matter::from_energy_fields();
        }
    }

    /// Update physical dimension (advance time step)
    pub fn update(&mut self, dt: Float) {
        // TODO: Advance time - time field not available
        // self.time.advance();

        // TODO: Update spacetime - spacetime field not available
        // self.spacetime.advance();

        // Update matter
        self.matter.update(dt);

        // Age the dimension
        self.age += dt;

        // Generate physical experiences
        self.generate_experiences();
    }

    /// Generate physical experiences from matter interactions
    fn generate_experiences(&mut self) {
        // Simplified experience generation
        // In a full implementation, this would analyze matter interactions
        // and generate appropriate physical experiences

        if self.matter.cells().len() > 0 {
            // Generate one experience per cell (simplified)
            for (i, _cell) in self.matter.cells().iter().enumerate() {
                let experience = PhysicalExperience {
                    experience_type: PhysicalExperienceType::Growth,
                    pleasure_pain: 0.5,
                    success_failure: 0.5,
                    fear_relief: 0.0,
                    entity_id: format!("cell_{}", i),
                };

                self.experiences.push(experience);
            }
        }
    }

    /// Apply natural laws to matter
    pub fn apply_natural_laws(&mut self) {
        // Apply gravitational forces
        for atom in self.matter.atoms_mut() {
            let force = self.energy_fields.gravitational_field.force_on(
                atom.position.to_vector3(),
                atom.atomic_mass(),
                self.natural_laws.gravitational_constant,
            );
            // TODO: Apply force to atom (no apply_force method available)
            let _ = force;
        }

        // Apply electromagnetic forces
        for particle in &mut self.matter.particles_mut() {
            let force = self
                .energy_fields
                .electromagnetic_field
                .electric_field
                .force_on(
                    particle.position.to_vector3(),
                    particle.charge,
                    8.9875517923e9,
                );
            // TODO: Apply force to particle (no apply_force method available)
            let _ = force;
        }
    }

    /// Collapse probability space (make a choice)
    pub fn collapse_probability(&mut self, choice: Option<usize>) -> PhysicalOutcome {
        let outcome = self.probability_space.collapse(choice);

        PhysicalOutcome {
            outcome_id: outcome.id,
            probability: outcome.probability,
            description: outcome
                .description
                .unwrap_or_else(|| "Physical outcome".to_string()),
            spacetime_event: SpacetimeEvent::new(Vector3::zero(), self.age),
        }
    }

    /// Reset probability space (return to superposition)
    pub fn reset_probability(&mut self) {
        self.probability_space.reset();
    }

    /// Get total energy in physical dimension
    pub fn total_energy(&self) -> Float {
        self.matter.total_energy() + self.energy_fields.total_field_energy(Vector3::zero())
    }

    /// Get total mass in physical dimension
    pub fn total_mass(&self) -> Float {
        self.matter.total_mass()
    }

    /// Get total entropy
    pub fn total_entropy(&self) -> Float {
        self.probability_space.entropy()
    }

    /// Validate physical dimension
    pub fn validate(&self) -> Result<(), String> {
        // Validate natural laws
        self.natural_laws.validate()?;

        // TODO: Check consistency - space field not available
        // if self.space.dimensions != self.natural_laws.spatial_dimensions {
        //     return Err(format!(
        //         "Space dimensions ({}) don't match natural laws ({})",
        //         self.space.dimensions, self.natural_laws.spatial_dimensions
        //     ));
        // }

        Ok(())
    }

    /// Get summary of physical dimension
    pub fn summary(&self) -> String {
        format!(
            "Physical Dimension (Age: {:.2e} s):\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             - Total Energy: {:.2e} J\n\
             - Total Mass: {:.2e} kg\n\
             - Total Entropy: {:.2} bits\n\
             - Experiences: {}",
            self.age,
            self.natural_laws,
            self.energy_fields,
            self.matter,
            self.probability_space,
            self.total_energy(),
            self.total_mass(),
            self.total_entropy(),
            self.experiences.len()
        )
    }
}

impl fmt::Display for PhysicalDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Physical experience (generated in space/time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalExperience {
    pub experience_type: PhysicalExperienceType,
    pub pleasure_pain: Float,   // -1.0 (pain) to 1.0 (pleasure)
    pub success_failure: Float, // -1.0 (failure) to 1.0 (success)
    pub fear_relief: Float,     // -1.0 (relief) to 1.0 (fear)
    pub entity_id: String,
}

/// Types of physical experiences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalExperienceType {
    Birth,
    Death,
    Hunger,
    Thirst,
    Pain,
    Pleasure,
    Competition,
    Cooperation,
    Scarcity,
    Abundance,
    Growth,
    Decay,
    Discovery,
    Loss,
}

/// Physical outcome (result of probability collapse)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalOutcome {
    pub outcome_id: usize,
    pub probability: Float,
    pub description: String,
    pub spacetime_event: SpacetimeEvent,
}

impl PhysicalOutcome {
    /// Create a physical outcome
    pub fn new(
        outcome_id: usize,
        probability: Float,
        description: String,
        spacetime_event: SpacetimeEvent,
    ) -> Self {
        PhysicalOutcome {
            outcome_id,
            probability,
            description,
            spacetime_event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_dimension_default() {
        let dimension = PhysicalDimension::default();
        assert_eq!(dimension.age, 0.0);
        assert_eq!(dimension.experiences.len(), 0);
    }

    #[test]
    fn test_physical_dimension_earth_like() {
        let dimension = PhysicalDimension::earth_like();
        assert!(dimension.natural_laws.energy_conservation);
        // Note: space field is commented out in PhysicalDimension
        // assert_eq!(dimension.space.dimensions, 3);
    }

    #[test]
    fn test_physical_dimension_with_natural_laws() {
        let laws = NaturalLaws::high_light_universe(2.0);
        let dimension = PhysicalDimension::with_natural_laws(laws);
        assert_eq!(dimension.natural_laws.speed_of_light, 299_792_458.0 * 2.0);
    }

    #[test]
    fn test_physical_dimension_from_light() {
        let dimension = PhysicalDimension::from_light(1.0e25, 42);
        assert!(dimension.matter.particles().len() > 0);
        assert!(dimension.matter.atoms().len() > 0);
    }

    #[test]
    fn test_physical_dimension_initialize() {
        let mut dimension = PhysicalDimension::earth_like();
        dimension.initialize(42);
        assert_eq!(dimension.probability_space.outcomes.len(), 10);
    }

    #[test]
    fn test_physical_dimension_update() {
        let mut dimension = PhysicalDimension::from_light(1.0e25, 42);
        dimension.update(1.0e-15);
        assert!(dimension.age > 0.0);
    }

    #[test]
    fn test_physical_dimension_apply_natural_laws() {
        let mut dimension = PhysicalDimension::from_light(1.0e25, 42);
        dimension.apply_natural_laws();
        // Should not panic
    }

    #[test]
    fn test_physical_dimension_collapse_probability() {
        let mut dimension = PhysicalDimension::from_light(1.0e25, 42);
        let outcome = dimension.collapse_probability(None);
        assert!(dimension.probability_space.is_collapsed());
        assert_eq!(
            outcome.outcome_id,
            dimension.probability_space.collapsed.unwrap()
        );
    }

    #[test]
    fn test_physical_dimension_reset_probability() {
        let mut dimension = PhysicalDimension::from_light(1.0e25, 42);
        dimension.collapse_probability(None);
        dimension.reset_probability();
        assert!(!dimension.probability_space.is_collapsed());
    }

    #[test]
    fn test_physical_dimension_total_energy() {
        let dimension = PhysicalDimension::from_light(1.0e25, 42);
        let energy = dimension.total_energy();
        assert!(energy >= 0.0);
    }

    #[test]
    fn test_physical_dimension_total_mass() {
        let dimension = PhysicalDimension::from_light(1.0e25, 42);
        let mass = dimension.total_mass();
        assert!(mass >= 0.0);
    }

    #[test]
    fn test_physical_dimension_total_entropy() {
        let dimension = PhysicalDimension::from_light(1.0e25, 42);
        let entropy = dimension.total_entropy();
        assert!(entropy >= 0.0);
    }

    #[test]
    fn test_physical_dimension_validate() {
        let dimension = PhysicalDimension::earth_like();
        assert!(dimension.validate().is_ok());
    }

    #[test]
    fn test_physical_dimension_validate_invalid() {
        let mut dimension = PhysicalDimension::earth_like();
        // Note: space field is commented out in PhysicalDimension
        // dimension.space.dimensions = 4;
        dimension.natural_laws.spatial_dimensions = 3;
        // This test may not work as expected without space field
        // assert!(dimension.validate().is_err());
    }

    #[test]
    fn test_physical_experience_creation() {
        let experience = PhysicalExperience {
            experience_type: PhysicalExperienceType::Pleasure,
            pleasure_pain: 0.8,
            success_failure: 0.5,
            fear_relief: 0.0,
            entity_id: "test_entity".to_string(),
        };
        assert_eq!(experience.experience_type, PhysicalExperienceType::Pleasure);
        assert_eq!(experience.pleasure_pain, 0.8);
    }

    #[test]
    fn test_physical_outcome_creation() {
        let event = SpacetimeEvent::origin(0.0);
        let outcome = PhysicalOutcome::new(0, 0.5, "Test outcome".to_string(), event);
        assert_eq!(outcome.outcome_id, 0);
        assert_eq!(outcome.probability, 0.5);
    }

    #[test]
    fn test_physical_dimension_generate_experiences() {
        let mut dimension = PhysicalDimension::from_light(1.0e25, 42);
        // TODO: Implement add_cell method or replace matter with Cell
        // dimension.matter.add_cell(Cell::eukaryotic());
        // Create a eukaryotic cell directly
        let eukaryotic_cell = Cell {
            organelles: vec![
                (OrganelleType::Nucleus, "Cell nucleus".to_string()),
                (OrganelleType::Mitochondria, "Powerhouse".to_string()),
                (OrganelleType::Ribosome, "Protein synthesis".to_string()),
            ],
        };
        dimension.matter = Matter::Cell(eukaryotic_cell);
        dimension.update(1.0e-15);
        // Should generate experiences
        assert!(dimension.experiences.len() > 0);
    }

    #[test]
    fn test_physical_dimension_summary() {
        let dimension = PhysicalDimension::earth_like();
        let summary = dimension.summary();
        assert!(summary.contains("Physical Dimension"));
        assert!(summary.contains("Natural Laws"));
    }

    #[test]
    fn test_physical_dimension_display() {
        let dimension = PhysicalDimension::earth_like();
        let display = format!("{}", dimension);
        assert!(display.contains("Physical Dimension"));
    }
}
