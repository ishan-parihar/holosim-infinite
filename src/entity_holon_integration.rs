// Entity-Holon Integration Module
// Phase 3: Integration of Holon and Entity Systems
//
// This module provides integration between the legacy entity system
// and the new holon architecture, enabling entities to use the
// correct 22-archetype architecture.

use crate::holon::Holon;
use crate::types::Float;
use crate::types::HolonID;
use std::collections::HashMap;

/// Holon Emergence System
/// Tracks holon emergence levels and state
#[derive(Debug, Clone)]
pub struct HolonEmergence {
    /// Emergence level
    pub emergence_level: Float,
    /// Whether holons have emerged
    pub is_emerged: bool,
}

impl HolonEmergence {
    /// Get total number of holons
    pub fn get_total_holons(&self) -> usize {
        if self.is_emerged {
            1
        } else {
            0
        }
    }
}

/// Entity-Holon Integration System
/// Manages the integration between legacy entities and new holons
///
/// Note: Clone is not implemented because HolonEmergence doesn't implement Clone.
#[derive(Debug)]
pub struct EntityHolonIntegration {
    /// Holon storage
    holons: HashMap<HolonID, Holon>,

    /// Entity to Holon mapping
    entity_to_holon: HashMap<usize, HolonID>,

    /// Holon to Entity mapping
    holon_to_entity: HashMap<HolonID, usize>,
}

impl EntityHolonIntegration {
    /// Create a new Entity-Holon Integration system
    pub fn new() -> Self {
        EntityHolonIntegration {
            holons: HashMap::new(),
            entity_to_holon: HashMap::new(),
            holon_to_entity: HashMap::new(),
        }
    }

    /// Get holon by entity ID
    pub fn get_holon_by_entity(&self, entity_id: usize) -> Option<&Holon> {
        if let Some(holon_id) = self.entity_to_holon.get(&entity_id) {
            self.holons.get(holon_id)
        } else {
            None
        }
    }

    /// Get mutable holon by entity ID
    pub fn get_holon_by_entity_mut(&mut self, entity_id: usize) -> Option<&mut Holon> {
        let holon_id = self.entity_to_holon.get(&entity_id).copied()?;
        self.holons.get_mut(&holon_id)
    }

    /// Get entity by holon ID
    pub fn get_entity_by_holon(&self, holon_id: HolonID) -> Option<usize> {
        self.holon_to_entity.get(&holon_id).copied()
    }

    /// Process all holons
    pub fn process_all_holons(&mut self, _coupling_coefficient: Float) {
        for (_id, _holon) in self.holons.iter_mut() {
            // TODO: Implement holon processing
            // holon.process_step(coupling_coefficient, &holon.position);
        }
    }

    /// Get holon emergence system
    pub fn get_holon_emergence(&self) -> &HolonEmergence {
        static DEFAULT_EMERGENCE: HolonEmergence = HolonEmergence {
            emergence_level: 0.0,
            is_emerged: false,
        };
        &DEFAULT_EMERGENCE
    }

    /// Get mutable holon emergence system
    pub fn get_holon_emergence_mut(&mut self) -> &mut HolonEmergence {
        static mut DEFAULT_EMERGENCE: HolonEmergence = HolonEmergence {
            emergence_level: 0.0,
            is_emerged: false,
        };
        #[allow(static_mut_refs)]
        unsafe {
            &mut DEFAULT_EMERGENCE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = EntityHolonIntegration::new();
        assert_eq!(integration.get_holon_emergence().get_total_holons(), 0);
    }
}
