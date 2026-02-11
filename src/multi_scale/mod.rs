// Multi-Scale Entity System - Phase 3: Green-Ray Realm
// Implements the holographic principle across scales
//
// Knowledge Base Reference: Cosmology.json
// "Each entity contains within it all densities and sub-densities of the octave"
// "The creation is holographic - any portion contains the whole"

pub mod entities;
pub mod holographic_containment;
pub mod scales;

pub use entities::{
    AtomID, AtomicPosition, CellType, CollectiveType, Electron, GalacticType, MolecularBond,
    MultiScaleAtom, MultiScaleCell, MultiScaleCollective, MultiScaleCosmic, MultiScaleGalactic,
    MultiScaleMolecule, MultiScaleOrganism, MultiScalePlanetary, MultiScaleQuantumState,
    MultiScaleSolar, OrganismType, ParticleID, PlanetaryType, QuantumParticle, QuantumPosition,
    SolarType,
};
pub use holographic_containment::HolographicReference;
pub use scales::{
    EntityId as ScaleEntityId, EntityScale, MultiScaleEntity, MultiScaleHierarchy as ScaleHierarchy,
};
