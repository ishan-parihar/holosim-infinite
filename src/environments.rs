//! Environments module - stub implementation

use crate::types::Float;

/// Environment placeholder
pub struct Environment {
    pub id: u64,
}

impl Environment {
    pub fn new(id: u64) -> Self {
        Environment { id }
    }
}
