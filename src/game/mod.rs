//! Holonic Gaming Engine Module
//! 
//! This module wraps the holographic simulation with gaming engine capabilities:
//! - ECS (Entity Component System)
//! - Physics & Interactions
//! - AI & Behavior
//! - Game Mechanics (inventory, crafting, building)
//! - World Management
//! - Networking

pub mod core;
pub mod ecs;
pub mod physics;
pub mod ai;
pub mod mechanics;
pub mod world;
pub mod systems;
pub mod networking;

pub use core::*;
