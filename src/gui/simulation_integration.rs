//! Simulation-GUI Integration Layer
//!
//! Bridges the simulation backend with GUI visualization systems.
//! Handles data flow, event propagation, and state synchronization.
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md: "Connect all visualization systems to simulation backend"

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::entity_layer7::{EntityId, EntityType};
use crate::integrated_system::{GuiCollective, GuiEntity, IntegratedSystem};

/// Integration bridge between simulation and GUI
///
/// Provides:
/// - Entity data access for visualization
/// - Event propagation between systems
/// - State synchronization
/// - Metric collection
pub struct SimulationGuiIntegration {
    /// Reference to simulation system
    simulation: Arc<RwLock<IntegratedSystem>>,

    /// Cached entity data for rendering
    entity_cache: HashMap<EntityId, EntityRenderData>,

    /// Cached collective data
    collective_cache: Vec<CollectiveRenderData>,

    /// Cached emergence metrics
    emergence_cache: EmergenceMetrics,

    /// Cached spectrum data
    spectrum_cache: SpectrumData,

    /// Event queue for GUI notifications
    event_queue: Vec<SimulationEvent>,

    /// Last update timestamp
    last_update: std::time::Instant,

    /// Update interval
    update_interval: Duration,

    /// Whether cache needs refresh
    dirty: bool,
}

/// Render-optimized entity data
#[derive(Debug, Clone)]
pub struct EntityRenderData {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: [f32; 3],
    pub scale: f32,
    pub density: u8,
    pub polarity: f32,
    pub consciousness: f32,
    pub color: [f32; 4],
    pub archetype_activations: [f32; 22],
    pub is_active: bool,
    pub emergence_level: f32,
}

/// Render-optimized collective data
#[derive(Debug, Clone)]
pub struct CollectiveRenderData {
    pub id: u64,
    pub center_position: [f32; 3],
    pub radius: f32,
    pub member_count: usize,
    pub coherence: f32,
    pub consciousness_level: f32,
    pub density: u8,
    pub color: [f32; 4],
}

/// Emergence metrics for visualization
#[derive(Debug, Clone, Default)]
pub struct EmergenceMetrics {
    pub biological_level: f32,
    pub noospheric_level: f32,
    pub gaia_level: f32,
    pub species_count: u32,
    pub social_complex_count: u32,
    pub global_consciousness: f32,
    pub ecosystem_stability: f32,
}

/// Spectrum data for visualization
#[derive(Debug, Clone, Default)]
pub struct SpectrumData {
    pub space_time_entities: Vec<EntityId>,
    pub time_space_entities: Vec<EntityId>,
    pub veil_position: f32,
    pub veil_transparency: f32,
    pub distribution: [u32; 10], // Distribution across spectrum
}

/// Simulation events for GUI
#[derive(Debug, Clone)]
pub enum SimulationEvent {
    EntityCreated(EntityId),
    EntityDestroyed(EntityId),
    EntityEvolved(EntityId, u8, u8), // old_density, new_density
    CollectiveFormed(u64),
    CollectiveDissolved(u64),
    EmergenceEvent(String, f32),
    DensityTransition(u8),
    CatalystEvent(String),
}

impl SimulationGuiIntegration {
    /// Create new integration layer
    pub fn new(simulation: IntegratedSystem) -> Self {
        Self {
            simulation: Arc::new(RwLock::new(simulation)),
            entity_cache: HashMap::new(),
            collective_cache: Vec::new(),
            emergence_cache: EmergenceMetrics::default(),
            spectrum_cache: SpectrumData::default(),
            event_queue: Vec::new(),
            last_update: std::time::Instant::now(),
            update_interval: Duration::from_millis(16), // ~60 FPS
            dirty: true,
        }
    }

    /// Update cached data from simulation
    pub fn update(&mut self) {
        let now = std::time::Instant::now();

        // Throttle updates
        if now.duration_since(self.last_update) < self.update_interval && !self.dirty {
            return;
        }

        self.last_update = now;
        self.dirty = false;

        // Acquire read lock and collect data
        let (entities, collectives, emergence_metrics, veil_transparency, events) = {
            let simulation = match self.simulation.read() {
                Ok(s) => s,
                Err(_) => return, // Skip update if lock is poisoned
            };

            // Clone necessary data
            let entities = simulation.gui_entities().clone();
            let collectives = simulation.collectives().clone();
            let emergence_metrics = simulation.emergence_metrics().clone();
            let veil_transparency = simulation.veil_transparency();
            let events = simulation.events().clone();

            (
                entities,
                collectives,
                emergence_metrics,
                veil_transparency,
                events,
            )
        };

        // Update caches with collected data
        self.update_entity_cache_from_data(&entities);
        self.update_collective_cache_from_data(&collectives);
        self.emergence_cache = EmergenceMetrics {
            biological_level: emergence_metrics.biological_level,
            noospheric_level: emergence_metrics.noospheric_level,
            gaia_level: emergence_metrics.gaia_level,
            species_count: emergence_metrics.species_count,
            social_complex_count: emergence_metrics.social_complex_count,
            global_consciousness: emergence_metrics.global_consciousness,
            ecosystem_stability: emergence_metrics.ecosystem_stability,
        };
        self.spectrum_cache.veil_transparency = veil_transparency as f32;

        // Collect events
        self.event_queue.clear();
        for _event in events {
            continue;
        }
    }

    /// Update entity cache from collected data
    fn update_entity_cache_from_data(&mut self, entities: &[GuiEntity]) {
        self.entity_cache.clear();

        for entity in entities {
            let render_data = self.convert_entity_to_render_data(entity);
            self.entity_cache.insert(entity.id.clone(), render_data);
        }
    }

    /// Update collective cache from collected data
    fn update_collective_cache_from_data(&mut self, collectives: &[GuiCollective]) {
        self.collective_cache.clear();

        for collective in collectives {
            let render_data = CollectiveRenderData {
                id: collective.id,
                center_position: [
                    collective.center.x.log10() as f32,
                    collective.center.y.log10() as f32,
                    collective.center.z.log10() as f32,
                ],
                radius: collective.radius.log10() as f32,
                member_count: collective.members.len(),
                coherence: collective.coherence as f32,
                consciousness_level: collective.consciousness_level as f32,
                density: collective.density.as_u8(),
                color: self.get_collective_color(collective),
            };

            self.collective_cache.push(render_data);
        }
    }

    /// Convert simulation entity to render data
    fn convert_entity_to_render_data(&self, entity: &GuiEntity) -> EntityRenderData {
        // Convert position to logarithmic space
        let log_position = [
            entity.position.x.log10() as f32,
            entity.position.y.log10() as f32,
            entity.position.z.log10() as f32,
        ];

        // Calculate scale in logarithmic space
        let log_scale = entity.scale.log10() as f32;

        // Get color based on density
        let color = self.get_density_color(entity.density.as_u8());

        // Get archetype activations
        let mut archetype_activations = [0.0f32; 22];
        for (idx, activation) in entity.archetype_activations.iter().enumerate() {
            if idx < 22 {
                archetype_activations[idx] = *activation as f32;
            }
        }

        EntityRenderData {
            id: entity.id.clone(),
            entity_type: entity.entity_type,
            position: log_position,
            scale: log_scale,
            density: entity.density.as_u8(),
            polarity: entity.polarity as f32,
            consciousness: entity.consciousness as f32,
            color,
            archetype_activations,
            is_active: entity.is_active,
            emergence_level: entity.emergence_level as f32,
        }
    }

    /// Get color for density level
    fn get_density_color(&self, density: u8) -> [f32; 4] {
        match density {
            1 => [1.0, 0.27, 0.27, 1.0], // Red
            2 => [1.0, 0.53, 0.27, 1.0], // Orange
            3 => [1.0, 0.8, 0.27, 1.0],  // Yellow
            4 => [0.27, 1.0, 0.27, 1.0], // Green
            5 => [0.27, 1.0, 1.0, 1.0],  // Cyan
            6 => [0.27, 0.27, 1.0, 1.0], // Blue
            7 => [0.53, 0.27, 1.0, 1.0], // Violet
            8 => [1.0, 1.0, 1.0, 1.0],   // White
            _ => [0.5, 0.5, 0.5, 1.0],   // Gray
        }
    }

    /// Get color for collective
    fn get_collective_color(&self, collective: &GuiCollective) -> [f32; 4] {
        // Base color on density with coherence affecting alpha
        let mut color = match collective.density {
            crate::types::Density::First => [1.0, 0.27, 0.27, 1.0],
            crate::types::Density::Second => [1.0, 0.53, 0.27, 1.0],
            crate::types::Density::Third => [1.0, 0.8, 0.27, 1.0],
            crate::types::Density::Fourth => [0.27, 1.0, 0.27, 1.0],
            crate::types::Density::Fifth => [0.27, 1.0, 1.0, 1.0],
            crate::types::Density::Sixth => [0.27, 0.27, 1.0, 1.0],
            crate::types::Density::Seventh => [0.53, 0.27, 1.0, 1.0],
            crate::types::Density::Eighth => [1.0, 1.0, 1.0, 1.0],
        };

        // Adjust alpha based on coherence
        color[3] = collective.coherence as f32;

        color
    }

    /// Get all entity render data
    pub fn get_entities(&self) -> &HashMap<EntityId, EntityRenderData> {
        &self.entity_cache
    }

    /// Get specific entity render data
    pub fn get_entity(&self, id: &EntityId) -> Option<&EntityRenderData> {
        self.entity_cache.get(id)
    }

    /// Get all collective render data
    pub fn get_collectives(&self) -> &[CollectiveRenderData] {
        &self.collective_cache
    }

    /// Get emergence metrics
    pub fn get_emergence_metrics(&self) -> &EmergenceMetrics {
        &self.emergence_cache
    }

    /// Get spectrum data
    pub fn get_spectrum_data(&self) -> &SpectrumData {
        &self.spectrum_cache
    }

    /// Get and clear event queue
    pub fn take_events(&mut self) -> Vec<SimulationEvent> {
        std::mem::take(&mut self.event_queue)
    }

    /// Mark cache as dirty
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.entity_cache.len()
    }

    /// Get collective count
    pub fn collective_count(&self) -> usize {
        self.collective_cache.len()
    }

    /// Access simulation for advanced operations
    pub fn with_simulation<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&IntegratedSystem) -> R,
    {
        let simulation = self.simulation.read().unwrap();
        f(&simulation)
    }

    /// Mutate simulation for advanced operations
    pub fn with_simulation_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut IntegratedSystem) -> R,
    {
        let mut simulation = self.simulation.write().unwrap();
        f(&mut simulation)
    }
}

/// Event bridge for real-time event streaming
pub struct EventBridge {
    /// Event senders
    subscribers: Vec<Box<dyn Fn(&SimulationEvent) + Send + Sync>>,
}

impl EventBridge {
    /// Create new event bridge
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    /// Subscribe to events
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&SimulationEvent) + Send + Sync + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    /// Publish event to all subscribers
    pub fn publish(&self, event: &SimulationEvent) {
        for subscriber in &self.subscribers {
            subscriber(event);
        }
    }
}

impl Default for EventBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergence_metrics_default() {
        let metrics = EmergenceMetrics::default();
        assert_eq!(metrics.biological_level, 0.0);
        assert_eq!(metrics.species_count, 0);
        assert_eq!(metrics.global_consciousness, 0.0);
    }

    #[test]
    fn test_spectrum_data_default() {
        let data = SpectrumData::default();
        assert!(data.space_time_entities.is_empty());
        assert!(data.time_space_entities.is_empty());
        assert_eq!(data.veil_position, 0.0);
    }

    #[test]
    fn test_density_colors() {
        let integration = SimulationGuiIntegration {
            simulation: Arc::new(RwLock::new(IntegratedSystem::new())),
            entity_cache: HashMap::new(),
            collective_cache: Vec::new(),
            emergence_cache: EmergenceMetrics::default(),
            spectrum_cache: SpectrumData::default(),
            event_queue: Vec::new(),
            last_update: std::time::Instant::now(),
            update_interval: Duration::from_millis(16),
            dirty: false,
        };

        // Test each density color
        let red = integration.get_density_color(1);
        assert!(red[0] > 0.9); // High red
        assert!(red[1] < 0.3); // Low green

        let blue = integration.get_density_color(6);
        assert!(blue[2] > 0.9); // High blue

        let white = integration.get_density_color(8);
        assert!(white[0] > 0.9 && white[1] > 0.9 && white[2] > 0.9);
    }

    #[test]
    fn test_event_bridge() {
        let mut bridge = EventBridge::new();
        let received = Arc::new(RwLock::new(false));

        let received_clone = received.clone();
        bridge.subscribe(move |_event| {
            *received_clone.write().unwrap() = true;
        });

        let event = SimulationEvent::EntityCreated(EntityId::new("test".to_string()));
        bridge.publish(&event);

        assert!(*received.read().unwrap());
    }
}
