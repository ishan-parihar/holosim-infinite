//! Core gaming engine infrastructure

use crate::entity_layer7::layer7::{SubSubLogos, EntityId, Density};

/// Wrapper that preserves holographic architecture while adding game features
#[derive(Debug, Clone)]
pub struct HolographicEntity {
    /// The underlying holographic entity (preserved)
    pub inner: SubSubLogos,
    /// ECS components (game layer)
    pub components: ComponentStore,
}

/// Store for ECS components
#[derive(Debug, Clone, Default)]
pub struct ComponentStore {
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
    pub health: Option<Health>,
    pub inventory: Option<Inventory>,
    // More components to be added
}

/// 3D position component
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Velocity component
#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

/// Health component
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: f64,
    pub max: f64,
}

/// Inventory component
#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

/// Inventory item
#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quantity: u32,
}

impl HolographicEntity {
    pub fn new(inner: SubSubLogos) -> Self {
        Self {
            inner,
            components: ComponentStore::default(),
        }
    }
}
