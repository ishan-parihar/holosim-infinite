//! Involution Flow - Causal Chain from Intelligent Infinity to Entities
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 3:
//! "Implement top-down causal flow from Intelligent Infinity through the
//!  Logos hierarchy to entities."
//!
//! The Causal Chain:
//! ```text
//! IntelligentInfinity (Source)
//!     ↓ First Distortion (Free Will)
//! LOGOS (Blue-Ray)
//!     ↓ Second Distortion (Love)
//! SubLogos (Galactic/Solar)
//!     ↓ Third Distortion (Light)
//! SubSubLogos (Planetary)
//!     ↓ Blueprint Encoding
//! Entity Manifestation
//! ```
//!
//! KEY PRINCIPLE: "Transcend and Include"
//! - Each level includes all previous development
//! - Higher levels impose boundary conditions on lower levels
//! - Lower levels inherit and refine patterns from above

mod cosmic_hierarchy;
mod entity_manifestation;
mod logos_config;
mod propagation;

pub use cosmic_hierarchy::{CosmicHierarchy, HierarchyPath};
pub use entity_manifestation::{EntityManifestation, EntitySeed};
pub use logos_config::{
    GalacticLogosConfig, ManifestationParameters, PlanetaryLogosConfig, PrimaryLogosConfig,
    SolarLogosConfig,
};
pub use propagation::{FieldConfiguration, InvolutionFlow, PropagationResult};

use crate::types::Float;

pub const NUM_HIERARCHY_LEVELS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HierarchyLevel {
    Primary,
    Galactic,
    Solar,
    Planetary,
}

impl HierarchyLevel {
    pub fn depth(&self) -> usize {
        match self {
            HierarchyLevel::Primary => 0,
            HierarchyLevel::Galactic => 1,
            HierarchyLevel::Solar => 2,
            HierarchyLevel::Planetary => 3,
        }
    }

    pub fn from_depth(depth: usize) -> Option<Self> {
        match depth {
            0 => Some(HierarchyLevel::Primary),
            1 => Some(HierarchyLevel::Galactic),
            2 => Some(HierarchyLevel::Solar),
            3 => Some(HierarchyLevel::Planetary),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            HierarchyLevel::Primary => "Primary Logos (Universal)",
            HierarchyLevel::Galactic => "Galactic Logos",
            HierarchyLevel::Solar => "Solar Logos",
            HierarchyLevel::Planetary => "Planetary Logos",
        }
    }

    pub fn distortion(&self) -> &'static str {
        match self {
            HierarchyLevel::Primary => "First Distortion (Free Will)",
            HierarchyLevel::Galactic => "Second Distortion (Love)",
            HierarchyLevel::Solar => "Third Distortion (Light)",
            HierarchyLevel::Planetary => "Blueprint Encoding",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PropagationMode {
    Strict,
    Probabilistic,
    Resonant,
}

impl Default for PropagationMode {
    fn default() -> Self {
        PropagationMode::Resonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_level_depth() {
        assert_eq!(HierarchyLevel::Primary.depth(), 0);
        assert_eq!(HierarchyLevel::Galactic.depth(), 1);
        assert_eq!(HierarchyLevel::Solar.depth(), 2);
        assert_eq!(HierarchyLevel::Planetary.depth(), 3);
    }

    #[test]
    fn test_hierarchy_level_from_depth() {
        assert_eq!(HierarchyLevel::from_depth(0), Some(HierarchyLevel::Primary));
        assert_eq!(HierarchyLevel::from_depth(4), None);
    }

    #[test]
    fn test_hierarchy_distortions() {
        assert!(HierarchyLevel::Primary.distortion().contains("Free Will"));
        assert!(HierarchyLevel::Galactic.distortion().contains("Love"));
        assert!(HierarchyLevel::Solar.distortion().contains("Light"));
        assert!(HierarchyLevel::Planetary.distortion().contains("Blueprint"));
    }
}
