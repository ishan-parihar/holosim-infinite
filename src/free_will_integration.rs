// Free Will Integration
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Free Will is integrated into all aspects of existence"
//
// This module implements:
// 1. Free will integration mechanisms
// 2. Polarity tracking
// 3. Integration states

use crate::types::Float;

/// Free Will Integration Polarity
///
/// The polarity of free will integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeWillIntegrationPolarity {
    /// Positive polarity
    Positive,

    /// Negative polarity
    Negative,

    /// Neutral
    Neutral,
}

/// Free Will Integration
///
/// Tracks how free will is integrated into an entity.
#[derive(Debug, Clone, PartialEq)]
pub struct FreeWillIntegration {
    /// Integration level (0.0 to 1.0)
    pub integration_level: Float,

    /// Polarity
    pub polarity: FreeWillIntegrationPolarity,
}

impl Default for FreeWillIntegration {
    fn default() -> Self {
        Self {
            integration_level: 0.5,
            polarity: FreeWillIntegrationPolarity::Neutral,
        }
    }
}
