//! Spatial Module - Phase 4: Entity-Environment Coupling & Spatial Emergence
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md:
//! "Replace golden ratio positions with spectrum-derived positions"
//! "Positions emerge from field coherence gradients, NOT mathematical formulas"
//!
//! This module implements:
//! - SpectrumSpatialDynamics: Derives positions from field coherence gradients
//! - VeilTransform: Dynamical veil transformation
//! - EmergentPositions: Position emergence from field structure

use std::collections::HashMap;

/// Re-export for convenience
pub mod spectrum_spatial;
pub mod veil_transform;

pub use spectrum_spatial::*;
pub use veil_transform::*;

// ============================================================================
// COHERENCE GRADIENT DERIVED POSITIONS
// ============================================================================

/// Position derived from field coherence gradients
/// 
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > Space is not a fixed container - it EMERGES from the spectrum
/// > Positions should emerge from field coherence gradients, NOT mathematical formulas
#[derive(Debug, Clone)]
pub struct CoherenceDerivedPosition {
    /// The 3D position in space
    pub position: [f64; 3],
    
    /// Spectrum position (v = s/t) for this entity
    pub spectrum_position: f64,
    
    /// Local coherence at this position
    pub coherence: f64,
    
    /// Coherence gradient (direction of increasing coherence)
    pub coherence_gradient: [f64; 3],
    
    /// Which perspective: Space/Time or Time/Space
    pub perspective: SpatialPerspective,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpatialPerspective {
    /// Below veil: 3D space, 1D time
    SpaceTime,
    /// At veil: transition zone
    Transitional,
    /// Above veil: 1D space, 3D time
    TimeSpace,
}

impl CoherenceDerivedPosition {
    pub fn new(position: [f64; 3], spectrum_position: f64, coherence: f64, gradient: [f64; 3]) -> Self {
        let perspective = if spectrum_position < 1.0 {
            SpatialPerspective::SpaceTime
        } else if spectrum_position > 1.0 {
            SpatialPerspective::TimeSpace
        } else {
            SpatialPerspective::Transitional
        };
        
        CoherenceDerivedPosition {
            position,
            spectrum_position,
            coherence,
            coherence_gradient: gradient,
            perspective,
        }
    }
}

/// Spatial statistics for the coherence-derived position system
#[derive(Debug, Clone, Default)]
pub struct SpatialStatistics {
    pub total_positions: usize,
    pub average_coherence: f64,
    pub coherence_gradient_magnitude: f64,
    pub space_time_count: usize,
    pub time_space_count: usize,
    pub transitional_count: usize,
}
