//! Holon module for the Holonic Realms simulation
//!
//! This module defines the Holon type and related functionality
//! for the holonic architecture of the simulation.

use crate::types::{Float, HolonID};
use std::collections::HashMap;

/// A Holon - a fundamental unit in the holonic architecture
#[derive(Debug, Clone)]
pub struct Holon {
    pub id: HolonID,
    pub level: u8,
    pub emergence: Float,
    pub properties: HashMap<String, Float>,
}

impl Holon {
    pub fn new(id: HolonID, level: u8) -> Self {
        Holon {
            id,
            level,
            emergence: 0.0,
            properties: HashMap::new(),
        }
    }

    pub fn set_emergence(&mut self, emergence: Float) {
        self.emergence = emergence;
    }

    pub fn get_emergence(&self) -> Float {
        self.emergence
    }

    pub fn set_property(&mut self, key: String, value: Float) {
        self.properties.insert(key, value);
    }

    pub fn get_property(&self, key: &str) -> Option<Float> {
        self.properties.get(key).copied()
    }
}
