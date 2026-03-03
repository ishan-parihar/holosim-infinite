//! Procedural Cosmos Engine
//!
//! This module implements Phase 2 of the V4 roadmap: Universe self-generates
//! from field coherence. Stars ignite. Planets form. Physics is FELT by entities.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The spectrum is configured at galactic and solar scales before physical matter
//! exists. Intelligent infinity organizes itself through coherence patterns."
//!
//! ## Phase 2 Goals
//! - No pre-scripted universe structures
//! - Stars emerge from field coherence peaks
//! - Planets form from accretion disks
//! - Physics is felt by entities (gravity, radiation, day/night, seasons)

pub mod cosmic_web;
pub mod cosmos_engine;
pub mod orbital_mechanics;
pub mod physics_experience;
pub mod planetary_formation;
pub mod stellar_physics;

// Re-export main types for convenience
pub use cosmic_web::{CosmicStructure, CosmicWeb, Filament, Void};
pub use cosmos_engine::{CosmosEngine, ProtoStellarRegion, StellarId, StellarSystem};
pub use orbital_mechanics::{Orbit, OrbitalElements};
pub use physics_experience::PhysicsExperience;
pub use planetary_formation::{Moon, Planet, PlanetType, Season, TerrainType};
pub use stellar_physics::{RadiationSpectrum, SpectralClass, Star, StellarEvolutionStage};
