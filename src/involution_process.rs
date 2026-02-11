// Involution Process
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Involution is the process of spirit descending into matter"
//
// This module implements:
// 1. Involution process management
// 2. Stage tracking
// 3. Progress measurement

use crate::types::Float;

/// Involution Process
///
/// Tracks the involution process of spirit descending into matter.
#[derive(Debug, Clone, PartialEq)]
pub struct InvolutionProcess {
    /// Current stage (0.0 to 1.0)
    pub current_stage: Float,

    /// Progress (0.0 to 1.0)
    pub progress: Float,

    /// Is complete
    pub is_complete: bool,
}

impl Default for InvolutionProcess {
    fn default() -> Self {
        Self {
            current_stage: 0.0,
            progress: 0.0,
            is_complete: false,
        }
    }
}

impl InvolutionProcess {
    /// Create a new involution process
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance the involution process
    pub fn advance(&mut self, amount: Float) {
        self.progress = (self.progress + amount).min(1.0);
        self.current_stage = self.progress;
        if self.progress >= 1.0 {
            self.is_complete = true;
        }
    }
}
