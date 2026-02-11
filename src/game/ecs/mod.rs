//! Entity Component System

use crate::game::core::{HolographicEntity, ComponentStore};
use std::collections::HashMap;

pub use super::core::EntityId;

/// ECS World containing all entities
#[derive(Debug, Clone, Default)]
pub struct EcsWorld {
    pub entities: HashMap<EntityId, HolographicEntity>,
    pub next_entity_id: EntityId,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_entity(&mut self, holographic: crate::entity_layer7::layer7::SubSubLogos) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        
        let entity = HolographicEntity::new(holographic);
        self.entities.insert(id, entity);
        
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&HolographicEntity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut HolographicEntity> {
        self.entities.get_mut(&id)
    }
}
