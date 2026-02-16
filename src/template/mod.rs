//! Template Module - Phase 1 Week 3-4: Connect Existing Systems
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 1 (Weeks 3-4):
//! "Migrate SubSubLogos to use UniversalTemplate<EntityData>"
//! "Migrate EntityLifecycleManager to template methods"
//! "Migrate HolographicFieldManager to shared references"
//! "Preserve all existing functionality"
//!
//! This module provides:
//! - EntityData struct for entity-specific data
//! - Type alias: Entity = UniversalTemplate<EntityData>
//! - FromConfig implementation for EntityData
//! - Migration utilities for SubSubLogos → Entity conversion

pub mod entity_data;
pub mod migration;

pub use entity_data::{Entity, EntityData};
pub use migration::{SubSubLogosAdapter, convert_subsublogos_to_entity, convert_entity_to_subsublogos};
