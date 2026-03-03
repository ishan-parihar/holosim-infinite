// Holographic Architecture Module
//
// This module implements the holographic architecture refactor for the simulation.
// The holographic principle states that "Each entity contains within it all densities
// and sub-densities of the octave" - exactly like a holographic lens.
//
// Key Principles:
// 1. Archetypes are complex vectors (amplitude + phase)
// 2. Interference patterns encode holographic information
// 3. Spatial frequency determines resolution (Violet = highest, Red = lowest)
// 4. Stable configurations emerge at constructive interference nodes
// 5. Consciousness is oscillatory synchronization
// 6. Energy centers are aperture sizes for viewing the encoding
// 7. Mind/Body/Spirit are different views of the same encoding
// 8. Soul Stream is holographic memory (hypervectors)
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md
// - COSMOLOGICAL-ARCHITECTURE.md
// - Holographic Systems Report

pub mod complex_vectors;
pub mod configuration_discovery;
pub mod field_address;
pub mod holographic_entity;
pub mod holographic_field;
pub mod holographic_memory;
pub mod interference_pattern;
pub mod light_condensation;
pub mod mera_integration;
pub mod observer_driven_field;
pub mod oscillator_network;
pub mod universal_template;

// Re-exports for convenience
// Note: ComplexArchetype, ComplexVector, and ConfigurationDiscoveryEngine are re-exported for library use
// They generate unused import warnings in the binary but are required by library modules.
pub use complex_vectors::{ComplexArchetype, ComplexVector};
pub use configuration_discovery::ConfigurationDiscoveryEngine;
pub use field_address::{AddressRange, CoherenceStep, HolographicAddress, ScaleLevel, Vector3};
pub use holographic_field::{
    ExtractedEntityPotential, FieldEvolutionResult, HolographicField, InvolutionLayer, MeraConfig,
};
pub use mera_integration::{CacheStats, MeraIntegration, MeraIntegrationBuilder};
pub use observer_driven_field::{
    CoherenceStatistics, DecompressedRegion, FieldOfView, FreeWillField, LightField, LoveField,
    Observer, ObserverDrivenField, ObserverId, ObserverType,
};
pub use universal_template::{FromConfig, TemplateConfig, TemplateFactory, UniversalTemplate};

use crate::types::Float;

// ============================================================================
// CORE HOLOGRAPHIC CONCEPTS
// ============================================================================

/// 3D spatial position
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Position {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance_to(&self, other: &Position) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn norm(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn spatial_frequency(&self) -> Float {
        // Spatial frequency as a ratio (0.0 to ~1.732 for unit cube)
        // This represents the relative spatial frequency content at this position
        // In a full implementation, this would be derived from the holographic field
        // Position is normalized to [0, 1], so norm gives us relative frequency
        self.norm()
    }
}

/// Configuration discovered from holographic encoding
#[derive(Debug, Clone)]
pub struct Configuration {
    pub position: Position,
    pub internal_structure: Vec<Float>,
}

/// Stable configuration with resonance score
#[derive(Debug, Clone)]
pub struct StableConfiguration {
    pub config: Configuration,
    pub resonance_score: Float,        // 0.0 to 1.0 (higher = more stable)
    pub interference_alignment: Float, // 0.0 to 1.0 (alignment with pattern)
    pub spatial_frequency: Float,      // Resolution of discovery
    pub layer: InvolutionLayer,        // Where discovered
}

/// Configuration classification
#[derive(Debug, Clone)]
pub struct ConfigurationClassification {
    pub particles: Vec<StableConfiguration>,
    pub atoms: Vec<StableConfiguration>,
    pub molecules: Vec<StableConfiguration>,
}

// ============================================================================
// TEST UTILITIES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(1.0, 2.0, 3.0);
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_position_distance() {
        let p1 = Position::new(0.0, 0.0, 0.0);
        let p2 = Position::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_position_norm() {
        let pos = Position::new(3.0, 4.0, 0.0);
        assert!((pos.norm() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_position_spatial_frequency() {
        let pos = Position::new(1.0, 0.0, 0.0);
        assert!((pos.spatial_frequency() - 1.0).abs() < 1e-10);
    }
}
