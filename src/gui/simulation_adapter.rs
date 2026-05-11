//! SimulationRunnerAdapter - Bridges SimulationRunner to GUI rendering
//!
//! From HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md Phase 2:
//! "Connect SimulationRunner to GUI, implement multi-scale visualization"
//!
//! This module adapts the deep simulation (SimulationRunner) to the
//! visualization layer (EntityRenderer, ConnectionRenderer).
//!
//! ARCHITECTURE:
//! Before: IntegratedSystem → HolographicSimulation (simplified, 137 entities)
//! After:  SimulationRunnerAdapter → SimulationRunner (full simulation)
//!
//! PURPOSE:
//! - Replace the simplified HolographicSimulation with full SimulationRunner
//! - Stream entity data from deep simulation to renderers
//! - Provide multi-scale data access (universe, galaxy, planet, civilization)

use crate::civilization::CivilizationManager;
use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::observable_properties::ObservableProperties;
use crate::gui::observation_mapper::ObservationMapper;

// Re-export civilization types for GUI use
pub use crate::civilization::{CivilizationSummary, SettlementSummary};
use crate::gui::observable_properties::ObservableProperties;
use crate::gui::observation_mapper::ObservationMapper;
use crate::gui::renderer::entity_instance::EntityInstance;
use crate::gui::renderer::hierarchy_connection::HierarchyConnection;
use crate::hpo::{FieldVisualizationData, RenderableEntity, SimulationStatistics};
use crate::simulation_v3::observation_layer::{EntityObservation, ObservationLayer, TerrainType};
use crate::simulation_v3::simulation_runner::{SimulationParameters, SimulationRunner};
use crate::types::Float;
use std::collections::HashMap;

/// Information about an entity's position in the hierarchy
///
/// Used for UI display when navigating nested entity structures.
#[derive(Debug, Clone, Default)]
pub struct EntityHierarchyInfo {
    /// Entity ID
    pub entity_id: String,
    /// Entity type (GalacticLogos, SolarLogos, Individual, etc.)
    pub entity_type: String,
    /// Parent entity name (if any)
    pub parent_name: Option<String>,
    /// Number of child entities
    pub children_count: usize,
    /// Number of entities in composition
    pub composition_count: usize,
    /// Environment entity name (if any)
    pub environment_name: Option<String>,
    /// Whether this entity has internal structure to explore
    pub has_internal_structure: bool,
}

/// Adapter that connects SimulationRunner to GUI rendering
///
/// This is the primary bridge between the deep simulation and visualization.
/// It extracts entity and connection data from SimulationRunner and converts
/// it to GPU-friendly formats for rendering.
pub struct SimulationRunnerAdapter {
    /// The actual simulation runner
    runner: SimulationRunner,

    /// Civilization manager (Phase 5)
    civilization_manager: CivilizationManager,

    /// Cached entity instances for rendering
    cached_entities: Vec<EntityInstance>,

    /// Cached connections for hierarchy visualization
    cached_connections: Vec<HierarchyConnection>,

    /// Raw entity data from simulation
    raw_entities: HashMap<crate::entity_layer7::EntityId, SubSubLogos>,

    /// Last simulation step that was rendered
    last_update_step: u64,

    /// Is the simulation currently running?
    is_running: bool,

    /// Total entities created
    total_entities_created: usize,

    /// Simulation initialized flag
    initialized: bool,

    /// Cached simulation state for returning reference
    cached_state: crate::integrated_system::SimulationState,

    /// Cached holographic field state for visualization
    cached_field_state: crate::hpo::HolographicFieldState,

    /// Observation layer — translates simulation state into game-engine-consumable data
    observation_layer: ObservationLayer,

    /// Cached observations indexed by entity UUID string for GUI lookup
    cached_observations: HashMap<String, EntityObservation>,

    /// Recent simulation events for display
    recent_events: Vec<crate::simulation_v3::observation_layer::GameEvent>,
}

impl SimulationRunnerAdapter {
    /// Create a new adapter with default simulation parameters
    pub fn new() -> Self {
        let params = SimulationParameters {
            num_entities: 500,
            num_steps: 100000,
            run_involution: true,
            run_evolution: true,
            update_holographic_field: true,
            update_physical_manifestations: true,
            generate_detailed_reports: false,
            causal_inversion_mode: true,
            ..Default::default()
        };

        Self::with_params(params)
    }

    /// Create adapter with custom parameters
    pub fn with_params(params: SimulationParameters) -> Self {
        Self {
            runner: SimulationRunner::new(params),
            civilization_manager: CivilizationManager::new(),
            cached_entities: Vec::new(),
            cached_connections: Vec::new(),
            raw_entities: HashMap::new(),
            last_update_step: 0,
            is_running: false,
            total_entities_created: 0,
            initialized: false,
            cached_state: crate::integrated_system::SimulationState::default(),
            cached_field_state: crate::hpo::HolographicFieldState::with_defaults(),
            observation_layer: ObservationLayer::new(),
            cached_observations: HashMap::new(),
            recent_events: Vec::new(),
        }
    }

    /// Initialize the simulation (run involution to create entities)
    ///
    /// This must be called before stepping the simulation.
    /// It runs the involution sequence to create the initial entity population.
    pub fn initialize(&mut self) {
        if self.initialized {
            return;
        }

        println!("SimulationRunnerAdapter: Initializing simulation...");

        // Run involution directly using InvolutionSequenceRunner
        use crate::simulation_v3::involution_sequence::InvolutionSequenceRunner;

        let mut involution_runner = InvolutionSequenceRunner::new();

        match involution_runner.run_involution_sequence() {
            Ok(result) => {
                self.total_entities_created = result.entities.len();
                println!(
                    "SimulationRunnerAdapter: Involution complete, {} entities created",
                    self.total_entities_created
                );

                // Store entities locally
                for entity in result.entities {
                    self.raw_entities.insert(entity.entity_id.clone(), entity);
                }

                // Update caches
                self.update_caches();

                // Update cached state
                self.update_cached_state();

                self.initialized = true;
            }
            Err(e) => {
                eprintln!("SimulationRunnerAdapter: Involution failed: {:?}", e);
            }
        }
    }

    /// Step simulation forward by one tick
    ///
    /// For now, this increments the step counter. Full evolution
    /// integration will be added when SimulationRunner provides
    /// a public step-by-step interface.
    pub fn step(&mut self) {
        if !self.initialized {
            self.initialize();
        }

        // Increment step counter
        // Note: Full evolution integration pending public step API in SimulationRunner
        self.last_update_step = self.last_update_step.saturating_add(1);
        self.is_running = true;

        self.observation_layer.advance_tick();
        self.update_observations();

        // Update cached state
        self.update_cached_state();

        // For now, entities remain static after involution
        // Future: Add incremental evolution when available
    }

    /// Run a single simulation step (alias for step() for compatibility)
    ///
    /// This is an alias for step() to maintain compatibility with
    /// code that expects run_step() method.
    pub fn run_step(&mut self) {
        self.step();
    }

    /// Update the cached simulation state
    fn update_cached_state(&mut self) {
        self.cached_state = crate::integrated_system::SimulationState {
            step: self.current_step() as usize,
            entity_count: self.cached_entities.len(),
            coherence: self.calculate_coherence(),
            energy_balance: self.calculate_energy_balance(),
            emergence: crate::integrated_system::EmergenceState::default(),
            simulation_time: self.current_step() as Float,
        };

        self.cached_field_state.time = self.current_step() as Float;
        self.cached_field_state.average_coherence = self.calculate_coherence();
        self.cached_field_state.total_energy = self.calculate_energy_balance();
        self.cached_field_state.active_node_count = self.cached_entities.len().max(1);
        self.cached_field_state.leaf_node_count = self.cached_entities.len().max(1);
        self.cached_field_state.statistics.avg_magnitude = self.calculate_coherence();
    }

    /// Run simulation for multiple steps
    pub fn run_steps(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step();
        }
    }

    /// Check if simulation is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Check if simulation is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get current simulation step
    pub fn current_step(&self) -> u64 {
        self.last_update_step
    }

    /// Get total entities created
    pub fn total_entities(&self) -> usize {
        self.total_entities_created
    }

    /// Get entities for EntityRenderer
    pub fn get_entities(&self) -> &[EntityInstance] {
        &self.cached_entities
    }

    /// Get connections for ConnectionRenderer
    pub fn get_connections(&self) -> &[HierarchyConnection] {
        &self.cached_connections
    }

    /// Get mutable access to cached entities (for updates)
    pub fn get_entities_mut(&mut self) -> &mut Vec<EntityInstance> {
        &mut self.cached_entities
    }

    /// Get RenderableEntity format for compatibility with existing code
    pub fn get_renderable_entities(&self) -> Vec<RenderableEntity> {
        // Convert EntityInstance to RenderableEntity for legacy compatibility
        self.cached_entities
            .iter()
            .enumerate()
            .map(|(i, instance)| RenderableEntity {
                position: [
                    instance.position[0] as Float,
                    instance.position[1] as Float,
                    instance.position[2] as Float,
                ],
                raw_position: [
                    instance.position[0] as Float,
                    instance.position[1] as Float,
                    instance.position[2] as Float,
                ],
                // Use Blue for 3rd density entities (Animal/Human level)
                density_band: crate::hpo::DensityBand::Blue,
                consciousness: instance.consciousness_level as Float,
                in_collective: false,
                entity_id: i,
                // Use space_time config for physical reality
                spatial_config: crate::hpo::SpatialConfig::space_time(),
                veil_distance: (1.0 - instance.veil_transparency) as Float,
            })
            .collect()
    }

    /// Get raw SubSubLogos entities
    pub fn get_raw_entities(&self) -> &HashMap<crate::entity_layer7::EntityId, SubSubLogos> {
        &self.raw_entities
    }

    /// Get observable properties for ALL entities.
    ///
    /// This bridges HoloSim consciousness data (archetype activations,
    /// density, polarization, spectrum position) to game-facing
    /// ObservableProperties via ObservationMapper.
    pub fn get_observable_properties(&self) -> Vec<ObservableProperties> {
        self.raw_entities
            .values()
            .enumerate()
            .map(|(i, entity)| {
                let archetype_activations = entity.archetype_activations;
                let consciousness_level = entity.consciousness_level;
                let density = Self::density_to_u8(&entity.current_density);
                let spectrum_position = entity.spectrum_position;
                let polarization = entity.polarization.polarity_bias();

                ObservationMapper::map(
                    &archetype_activations,
                    consciousness_level,
                    density,
                    spectrum_position,
                    polarization,
                    i,
                )
            })
            .collect()
    }

    pub fn get_entity_observation(&self, entity_uuid: &str) -> Option<&EntityObservation> {
        self.cached_observations.get(entity_uuid)
    }

    pub fn get_all_observations(&self) -> &HashMap<String, EntityObservation> {
        &self.cached_observations
    }

    pub fn get_recent_events(&self) -> &[crate::simulation_v3::observation_layer::GameEvent] {
        &self.recent_events
    }

    pub fn terrain_type_display(terrain: TerrainType) -> &'static str {
        match terrain {
            TerrainType::Quantum => "Quantum Realm",
            TerrainType::Biological => "Biological Realm",
            TerrainType::Planetary => "Planetary Realm",
            TerrainType::Stellar => "Stellar Realm",
            TerrainType::Cosmic => "Cosmic Realm",
        }
    }

    /// Get cosmic data for CosmosRenderer
    ///
    /// Delegates to get_cosmic_render_data() which extracts stellar systems,
    /// planets, and cosmic web filaments from entity distribution.
    pub fn get_cosmic_data(&self) -> Option<CosmicRenderData> {
        Some(self.get_cosmic_render_data())
    }

    /// Get planet data for PlanetRenderer
    ///
    /// Delegates to get_planet_render_data() which generates terrain,
    /// water, cloud, storm, and settlement data from entity distribution.
    pub fn get_planet_data(&self, planet_id: u64) -> Option<PlanetRenderData> {
        Some(self.get_planet_render_data(planet_id))
    }

    /// Get civilization data for CivilizationRenderer
    ///
    /// Phase 5: Returns civilization data from CivilizationManager.
    pub fn get_civilization_data(&self) -> Option<CivilizationRenderData> {
        let summaries = self.civilization_manager.get_all_summaries();

        if summaries.is_empty() {
            return None;
        }

        // Generate trade routes from civilization relationships
        let trade_routes = self.generate_trade_routes();

        // Generate population density from settlements
        let population_density = self.generate_population_density();

        Some(CivilizationRenderData {
            civilizations: summaries,
            trade_routes,
            population_density,
        })
    }

    /// Generate trade routes between civilization settlements
    fn generate_trade_routes(&self) -> Vec<TradeRoute> {
        let mut routes = Vec::new();

        // Get all settlements from all civilizations
        let all_settlements = self.civilization_manager.get_all_settlement_summaries();

        // Create trade routes between nearby settlements
        for (i, s1) in all_settlements.iter().enumerate() {
            for s2 in all_settlements.iter().skip(i + 1) {
                // Calculate distance between settlements
                let dx = s1.location.0 - s2.location.0;
                let dy = s1.location.1 - s2.location.1;
                let distance = (dx * dx + dy * dy).sqrt();

                // If close enough, create a trade route
                if distance < 0.5 {
                    let volume = (s1.population.min(s2.population) as f64 * 0.00001).min(1.0);
                    routes.push(TradeRoute {
                        from_settlement: s1.id,
                        to_settlement: s2.id,
                        volume,
                    });
                }

                // Limit routes for performance
                if routes.len() >= 200 {
                    return routes;
                }
            }
        }

        routes
    }

    /// Generate population density data for visualization
    fn generate_population_density(&self) -> Vec<PopulationDensity> {
        self.civilization_manager
            .get_all_settlement_summaries()
            .iter()
            .map(|s| {
                let density = (s.population as f64).log10() / 8.0; // Normalize to 0-1
                PopulationDensity {
                    position: s.location,
                    density: density.min(1.0),
                }
            })
            .collect()
    }

    /// Get access to the CivilizationManager
    pub fn civilization_manager(&self) -> &CivilizationManager {
        &self.civilization_manager
    }

    /// Get mutable access to the CivilizationManager
    pub fn civilization_manager_mut(&mut self) -> &mut CivilizationManager {
        &mut self.civilization_manager
    }

    /// Get the underlying SimulationRunner for direct access
    pub fn runner(&self) -> &SimulationRunner {
        &self.runner
    }

    /// Get mutable access to the SimulationRunner
    pub fn runner_mut(&mut self) -> &mut SimulationRunner {
        &mut self.runner
    }

    /// Update cached data from simulation
    fn update_caches(&mut self) {
        let current_step = self.runner.current_step();

        // Only update if simulation has advanced
        if current_step == self.last_update_step && !self.cached_entities.is_empty() {
            return;
        }

        // Extract entity data from SimulationRunner
        self.cached_entities = self.extract_entity_instances();

        // Extract connection data from hierarchy
        self.cached_connections = self.extract_connections();

        self.last_update_step = current_step;
    }

    fn update_observations(&mut self) {
        self.cached_observations.clear();

        for (entity_id, entity) in &self.raw_entities {
            let id_numeric = Self::entity_id_to_u64(entity_id);
            let archetype_profile = entity.archetype_activations;
            let spectrum_position = entity.spectrum_position;
            let density = Self::density_to_u8(&entity.current_density);
            let position = EntityInstance::position_from_entity(entity);
            let velocity = [
                entity.kinetic_energy * 0.01,
                entity.kinetic_energy * 0.005,
                entity.kinetic_energy * 0.008,
            ];
            let mass = entity.energy.max(0.1);
            let energy = entity.energy.clamp(0.0, 1.0);
            let coherence = entity.current_state.vibrational_state.coherence;

            let observation = self.observation_layer.observe_entity(
                id_numeric,
                &archetype_profile,
                spectrum_position,
                density,
                [position[0] as f64, position[1] as f64, position[2] as f64],
                velocity,
                mass,
                energy,
                coherence,
            );

            self.cached_observations
                .insert(entity_id.uuid.clone(), observation);
        }

        self.recent_events = self.observation_layer.take_events();
        if self.recent_events.len() > 50 {
            self.recent_events = self.recent_events.split_off(self.recent_events.len() - 50);
        }
    }

    fn density_to_u8(density: &crate::evolution_density_octave::density_octave::Density) -> u8 {
        match density {
            crate::evolution_density_octave::density_octave::Density::First(_) => 1,
            crate::evolution_density_octave::density_octave::Density::Second(_) => 2,
            crate::evolution_density_octave::density_octave::Density::Third => 3,
            crate::evolution_density_octave::density_octave::Density::Fourth => 4,
            crate::evolution_density_octave::density_octave::Density::Fifth => 5,
            crate::evolution_density_octave::density_octave::Density::Sixth => 6,
            crate::evolution_density_octave::density_octave::Density::Seventh => 7,
            crate::evolution_density_octave::density_octave::Density::Eighth => 8,
        }
    }

    fn entity_id_to_u64(entity_id: &crate::entity_layer7::EntityId) -> u64 {
        entity_id.uuid.parse::<u64>().unwrap_or(0)
    }

    /// Extract entity instances from simulation
    fn extract_entity_instances(&self) -> Vec<EntityInstance> {
        // Get entities from SimulationRunner's lifecycle manager
        // For now, convert from raw_entities
        self.raw_entities
            .values()
            .enumerate()
            .map(|(i, entity)| EntityInstance::from_entity(entity, i))
            .collect()
    }

    /// Extract connection data from entity hierarchy
    fn extract_connections(&self) -> Vec<HierarchyConnection> {
        let mut connections = Vec::new();

        // Generate connections from entity hierarchy
        for entity in self.raw_entities.values() {
            let entity_pos = EntityInstance::position_from_entity(entity);

            // Parent-child connections
            if let Some(parent_id) = &entity.parent_id {
                if let Some(parent) = self.raw_entities.get(parent_id) {
                    let parent_pos = EntityInstance::position_from_entity(parent);
                    connections.push(HierarchyConnection::parent_child(
                        parent_pos, entity_pos, 0.8,
                    ));
                }
            }

            // Environment connections
            if let Some(env_id) = &entity.environment_id {
                if let Some(env) = self.raw_entities.get(env_id) {
                    let env_pos = EntityInstance::position_from_entity(env);
                    connections.push(HierarchyConnection::environment(env_pos, entity_pos, 0.5));
                }
            }
        }

        connections
    }

    /// Get simulation statistics summary
    pub fn get_statistics_summary(&self) -> String {
        format!(
            "SimulationRunnerAdapter Statistics:\n\
             - Initialized: {}\n\
             - Total Entities: {}\n\
             - Cached Entities: {}\n\
             - Cached Connections: {}\n\
             - Current Step: {}\n\
             - Running: {}",
            self.initialized,
            self.total_entities_created,
            self.cached_entities.len(),
            self.cached_connections.len(),
            self.current_step(),
            self.is_running
        )
    }

    // ========================================================================
    // IntegratedSystem Compatibility Methods
    // These methods provide the same interface as IntegratedSystem for
    // drop-in replacement in GuiApplication
    // ========================================================================

    /// Get all entities as SubSubLogos (legacy compatibility)
    ///
    /// Returns a clone of the raw entity data for GUI rendering.
    pub fn entities(&self) -> Vec<SubSubLogos> {
        self.raw_entities.values().cloned().collect()
    }

    /// Get all entities as ObservableProperties for the rendering pipeline.
    ///
    /// This bridges the raw consciousness data (archetype activations, density,
    /// spectrum position, polarization) into game-facing properties (position,
    /// color, size, shape, behavior) via ObservationMapper.
    pub fn get_observable_properties(&self) -> Vec<ObservableProperties> {
        self.raw_entities
            .values()
            .enumerate()
            .map(|(i, entity)| {
                ObservationMapper::map(
                    &entity.archetype_activations,
                    entity.consciousness_level,
                    Self::density_to_u8(&entity.current_density),
                    entity.spectrum_position,
                    entity.polarization.polarity_bias(),
                    i,
                )
            })
            .collect()
    }

    /// Get holographic simulation entities (field-derived)
    ///
    /// Returns entities in RenderableEntity format for compatibility
    /// with existing rendering code.
    pub fn holo_entities(&self) -> Vec<RenderableEntity> {
        self.get_renderable_entities()
    }

    /// Get the holographic field state for visualization
    ///
    /// Returns the cached field state derived from entity coherence,
    /// energy, and distribution data.
    pub fn get_holographic_field_state(&self) -> Option<&crate::hpo::HolographicFieldState> {
        Some(&self.cached_field_state)
    }

    /// Get simulation state
    ///
    /// Returns a reference to the cached SimulationState.
    pub fn state(&self) -> &crate::integrated_system::SimulationState {
        &self.cached_state
    }

    /// Check if holographic mode is enabled
    ///
    /// Always returns true as SimulationRunner uses field-based simulation.
    pub fn is_holographic_mode(&self) -> bool {
        true
    }

    /// Get holographic simulation statistics
    ///
    /// Returns statistics compatible with the HPO system.
    pub fn get_holo_statistics(&self) -> Option<SimulationStatistics> {
        // Create statistics from cached entity data
        let entity_count = self.cached_entities.len();

        // Calculate average consciousness
        let avg_consciousness = if entity_count > 0 {
            self.cached_entities
                .iter()
                .map(|e| e.consciousness_level as Float)
                .sum::<Float>()
                / entity_count as Float
        } else {
            0.0
        };

        // Calculate average polarization
        let _avg_polarization = if entity_count > 0 {
            self.cached_entities
                .iter()
                .map(|e| e.polarization as Float)
                .sum::<Float>()
                / entity_count as Float
        } else {
            0.0
        };

        Some(SimulationStatistics {
            entity_count,
            average_coherence: self.calculate_coherence(),
            average_spectrum_position: avg_consciousness * 0.5, // Derive from consciousness
            veil_transparency: self.calculate_avg_veil_transparency(),
            fps: 60.0, // Default FPS
            ..Default::default()
        })
    }

    /// Get field visualization data
    ///
    /// Returns field bounds and coherence data for rendering.
    pub fn get_field_visualization(&self) -> Option<FieldVisualizationData> {
        // Calculate field bounds from entity positions
        let positions: Vec<[Float; 3]> = self
            .cached_entities
            .iter()
            .map(|e| {
                [
                    e.position[0] as Float,
                    e.position[1] as Float,
                    e.position[2] as Float,
                ]
            })
            .collect();

        if positions.is_empty() {
            return None;
        }

        // Calculate bounds
        let mut min = [Float::MAX, Float::MAX, Float::MAX];
        let mut max = [Float::MIN, Float::MIN, Float::MIN];

        for pos in &positions {
            for i in 0..3 {
                min[i] = min[i].min(pos[i]);
                max[i] = max[i].max(pos[i]);
            }
        }

        Some(FieldVisualizationData {
            bounds: [min, max],
            coherence: self.calculate_coherence(),
            energy: self.calculate_energy_balance(),
            veil_transparency: self.calculate_avg_veil_transparency(),
            spectrum_position: 0.5, // Default middle of spectrum
        })
    }

    // ========================================================================
    // Hierarchy Navigation Methods
    // ========================================================================

    /// Get children of a specific entity
    ///
    /// Returns entities that have this entity as their parent.
    pub fn get_children(&self, entity_id: &str) -> Vec<SubSubLogos> {
        self.raw_entities
            .values()
            .filter(|e| {
                e.parent_id
                    .as_ref()
                    .map(|p| p.uuid.as_str() == entity_id)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Get composition of a specific entity
    ///
    /// Returns entities that this entity is composed of.
    pub fn get_composition(&self, entity_id: &str) -> Vec<SubSubLogos> {
        if let Some(entity) = self
            .raw_entities
            .values()
            .find(|e| e.entity_id.uuid.as_str() == entity_id)
        {
            entity
                .composition
                .iter()
                .filter_map(|comp_id| self.raw_entities.get(comp_id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get entities in an environment
    ///
    /// Returns entities that exist within this environment.
    pub fn get_entities_in_environment(&self, environment_id: &str) -> Vec<SubSubLogos> {
        self.raw_entities
            .values()
            .filter(|e| {
                e.environment_id
                    .as_ref()
                    .map(|env| env.uuid.as_str() == environment_id)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Get entities visible at a hierarchy level
    ///
    /// When drilling down, we only want to show relevant entities:
    /// - Root level: Show galaxies, galactic logoi
    /// - Galactic level: Show stars, solar logoi
    /// - Stellar level: Show planets
    /// - Planetary level: Show lifeforms
    /// - Entity level: Show biological components
    pub fn get_entities_at_hierarchy_level(
        &self,
        focus_entity_id: Option<&str>,
        depth: usize,
    ) -> Vec<SubSubLogos> {
        match (focus_entity_id, depth) {
            // Root level - show top-level entities (galaxies, galactic logoi)
            (None, 0) => self
                .raw_entities
                .values()
                .filter(|e| {
                    matches!(
                        e.entity_type,
                        crate::entity_layer7::layer7::EntityType::GalacticLogos
                    ) || e.parent_id.is_none()
                })
                .cloned()
                .collect(),

            // Inside an entity - show its children/composition
            (Some(focus_id), _) => {
                let mut result = Vec::new();

                // Add direct children
                result.extend(self.get_children(focus_id));

                // Add composition
                result.extend(self.get_composition(focus_id));

                // Add entities in this environment
                result.extend(self.get_entities_in_environment(focus_id));

                // Also include the focus entity itself (centered)
                if let Some(focus) = self
                    .raw_entities
                    .values()
                    .find(|e| e.entity_id.uuid.as_str() == focus_id)
                {
                    result.push(focus.clone());
                }

                result
            }

            _ => self.raw_entities.values().cloned().collect(),
        }
    }

    /// Get entity hierarchy info for UI display
    ///
    /// Returns information about an entity's position in the hierarchy.
    pub fn get_entity_hierarchy_info(&self, entity_id: &str) -> EntityHierarchyInfo {
        if let Some(entity) = self
            .raw_entities
            .values()
            .find(|e| e.entity_id.uuid.as_str() == entity_id)
        {
            let parent_name = entity
                .parent_id
                .as_ref()
                .and_then(|p| self.raw_entities.get(p))
                .map(|p| p.entity_id.uuid.clone());

            let children_count = entity.children.len();
            let composition_count = entity.composition.len();

            let environment_name = entity
                .environment_id
                .as_ref()
                .and_then(|env| self.raw_entities.get(env))
                .map(|env| env.entity_id.uuid.clone());

            EntityHierarchyInfo {
                entity_id: entity.entity_id.uuid.clone(),
                entity_type: format!("{:?}", entity.entity_type),
                parent_name,
                children_count,
                composition_count,
                environment_name,
                has_internal_structure: !entity.composition.is_empty()
                    || !entity.children.is_empty(),
            }
        } else {
            EntityHierarchyInfo::default()
        }
    }

    /// Get entity hierarchy info by EntityId
    pub fn get_entity_hierarchy_info_by_id(
        &self,
        entity_id: &crate::entity_layer7::EntityId,
    ) -> EntityHierarchyInfo {
        self.get_entity_hierarchy_info(entity_id.uuid.as_str())
    }

    /// Get children of a focus entity for hierarchy navigation
    ///
    /// Returns entities that are:
    /// - Direct children of the focus
    /// - In the composition of the focus
    /// - In the environment of the focus (if focus is an environment)
    pub fn get_children_of_focus(
        &self,
        focus_entity_id: Option<&crate::entity_layer7::EntityId>,
    ) -> Vec<SubSubLogos> {
        match focus_entity_id {
            None => {
                // At root level - show all top-level entities (those without parents)
                self.raw_entities
                    .values()
                    .filter(|e| {
                        e.parent_id.is_none()
                            && e.entity_type != crate::entity_layer7::layer7::EntityType::Individual
                    })
                    .cloned()
                    .collect()
            }
            Some(focus_id) => {
                let mut result = Vec::new();

                // Get the focus entity
                if let Some(focus) = self.raw_entities.get(focus_id) {
                    // Add direct children
                    for child_id in &focus.children {
                        if let Some(child) = self.raw_entities.get(child_id) {
                            result.push(child.clone());
                        }
                    }

                    // Add composition entities
                    for comp_id in &focus.composition {
                        if let Some(comp) = self.raw_entities.get(comp_id) {
                            result.push(comp.clone());
                        }
                    }
                }

                // Also add entities that have this focus as their parent
                for entity in self.raw_entities.values() {
                    if let Some(ref parent_id) = entity.parent_id {
                        if parent_id == focus_id
                            && !result.iter().any(|e| e.entity_id == entity.entity_id)
                        {
                            result.push(entity.clone());
                        }
                    }
                }

                result
            }
        }
    }

    /// Get entity by ID (using EntityId type)
    pub fn get_entity(&self, entity_id: &crate::entity_layer7::EntityId) -> Option<&SubSubLogos> {
        self.raw_entities.get(entity_id)
    }

    /// Get entity by ID string (convenience method)
    pub fn get_entity_by_uuid(&self, uuid: &str) -> Option<&SubSubLogos> {
        self.raw_entities
            .values()
            .find(|e| e.entity_id.uuid.as_str() == uuid)
    }

    /// Get entity instances with hierarchical positioning
    ///
    /// Returns EntityInstance objects with positions calculated relative to the current focus.
    pub fn get_hierarchical_entity_instances(
        &self,
        focus_entity_id: Option<&crate::entity_layer7::EntityId>,
        depth: usize,
    ) -> Vec<EntityInstance> {
        let entities =
            self.get_entities_at_hierarchy_level(focus_entity_id.map(|id| id.uuid.as_str()), depth);

        entities
            .iter()
            .enumerate()
            .map(|(i, entity)| {
                let mut instance = EntityInstance::from_entity(entity, i);
                // Update position with hierarchy-aware calculation
                instance.position = EntityInstance::position_with_hierarchy(
                    entity,
                    focus_entity_id,
                    &self.raw_entities,
                    depth,
                );
                instance
            })
            .collect()
    }

    /// Shutdown the simulation
    ///
    /// Stops the simulation and clears caches.
    pub fn shutdown(&mut self) {
        println!("SimulationRunnerAdapter: Shutting down...");
        self.is_running = false;
        self.cached_entities.clear();
        self.cached_connections.clear();
        self.raw_entities.clear();
        self.initialized = false;
        println!("SimulationRunnerAdapter: Shutdown complete");
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Calculate overall coherence from entity data
    fn calculate_coherence(&self) -> Float {
        if self.cached_entities.is_empty() {
            return 1.0;
        }

        // Average consciousness level as proxy for coherence
        let sum: f32 = self
            .cached_entities
            .iter()
            .map(|e| e.consciousness_level)
            .sum();
        (sum / self.cached_entities.len() as f32) as Float
    }

    /// Calculate energy balance from entity data
    fn calculate_energy_balance(&self) -> Float {
        if self.cached_entities.is_empty() {
            return 1.0;
        }

        // Balance between space/time and time/space ratios
        let sum: f32 = self
            .cached_entities
            .iter()
            .map(|e| {
                let ratio = e.space_time_ratio / (e.time_space_ratio + 0.001);
                1.0 - (ratio - 1.0).abs().min(1.0)
            })
            .sum();
        (sum / self.cached_entities.len() as f32) as Float
    }

    /// Calculate average veil transparency
    fn calculate_avg_veil_transparency(&self) -> Float {
        if self.cached_entities.is_empty() {
            return 0.0;
        }

        let sum: f32 = self
            .cached_entities
            .iter()
            .map(|e| e.veil_transparency)
            .sum();
        (sum / self.cached_entities.len() as f32) as Float
    }

    // ========================================================================
    // Phase 3: Cosmos Data Bridging
    // ========================================================================

    /// Get cosmic render data for visualization
    ///
    /// This method bridges the CosmosEngine data to the rendering layer.
    /// Returns stellar systems, planets, and cosmic web filaments.
    pub fn get_cosmic_render_data(&self) -> CosmicRenderData {
        // For now, generate procedural cosmic data based on entity distribution
        // In future, this will connect to SimulationRunner's CosmosEngine

        // Generate stellar systems from entity clusters
        let stellar_systems = self.generate_stellar_systems_from_entities();

        // Generate cosmic filaments from entity connections
        let filaments = self.generate_filaments_from_connections();

        CosmicRenderData {
            filaments,
            stellar_systems,
            selected_star: None,
            selected_planet: None,
        }
    }

    /// Generate stellar systems from entity positions
    ///
    /// Entities with high consciousness become stars,
    /// entities in their vicinity become planets.
    fn generate_stellar_systems_from_entities(&self) -> Vec<StellarSystemData> {
        let mut systems = Vec::new();

        // Find high-consciousness entities to use as stars
        let stars: Vec<(usize, &EntityInstance)> = self
            .cached_entities
            .iter()
            .enumerate()
            .filter(|(_, e)| e.consciousness_level > 0.7)
            .take(20) // Limit to 20 stars for performance
            .collect();

        for (idx, star_entity) in stars {
            // Determine star color based on consciousness level
            let color = self.consciousness_to_star_color(star_entity.consciousness_level);

            // Determine luminosity based on consciousness level
            let luminosity = (star_entity.consciousness_level * 100.0 + 0.1) as f64;

            // Find nearby entities as planets
            let planets = self.find_planets_near_star(&star_entity.position);

            systems.push(StellarSystemData {
                id: idx as u64,
                star_position: [
                    star_entity.position[0] as f64,
                    star_entity.position[1] as f64,
                    star_entity.position[2] as f64,
                ],
                star_color: color,
                star_luminosity: luminosity,
                planets,
            });
        }

        systems
    }

    /// Convert consciousness level to star color
    fn consciousness_to_star_color(&self, consciousness: f32) -> [f32; 3] {
        // High consciousness = hot blue star
        // Low consciousness = cool red star
        if consciousness > 0.9 {
            [0.7, 0.8, 1.0] // O/B class - blue-white
        } else if consciousness > 0.8 {
            [0.9, 0.9, 1.0] // A class - white
        } else if consciousness > 0.7 {
            [1.0, 1.0, 0.9] // F class - yellow-white
        } else if consciousness > 0.6 {
            [1.0, 1.0, 0.7] // G class - yellow (Sun-like)
        } else if consciousness > 0.5 {
            [1.0, 0.8, 0.5] // K class - orange
        } else {
            [1.0, 0.6, 0.4] // M class - red
        }
    }

    /// Find entities that can be rendered as planets near a star
    fn find_planets_near_star(&self, star_pos: &[f32; 3]) -> Vec<PlanetOrbitData> {
        let mut planets = Vec::new();

        for (idx, entity) in self.cached_entities.iter().enumerate() {
            // Skip if this entity is a star (high consciousness)
            if entity.consciousness_level > 0.7 {
                continue;
            }

            // Calculate distance to star
            let dx = entity.position[0] - star_pos[0];
            let dy = entity.position[1] - star_pos[1];
            let dz = entity.position[2] - star_pos[2];
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();

            // If within orbital range (scaled for visualization)
            if distance > 0.1 && distance < 2.0 {
                // Calculate orbital parameters from entity properties
                let orbit_radius = distance as f64;
                let orbit_period = orbit_radius.powi(3).sqrt() * 365.0; // Kepler's 3rd law
                let current_angle = (entity.position[0].atan2(entity.position[2])) as f64;

                // Determine planet type from density level
                let planet_type = self.density_band_to_planet_type(entity.density_level as u32);

                planets.push(PlanetOrbitData {
                    id: idx as u64,
                    orbit_radius,
                    orbit_period,
                    current_angle,
                    planet_type,
                });
            }

            // Limit planets per star
            if planets.len() >= 12 {
                break;
            }
        }

        planets
    }

    /// Convert density band to planet type string
    fn density_band_to_planet_type(&self, band: u32) -> String {
        match band {
            0 => "ProtoPlanet".to_string(),
            1 => "GasGiant".to_string(),
            2 => "IceGiant".to_string(),
            3 => "Terrestrial".to_string(),
            4 => "Desert".to_string(),
            5 => "Ocean".to_string(),
            6 => "Ice".to_string(),
            7 => "Dwarf".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Generate cosmic filaments from entity connections
    fn generate_filaments_from_connections(&self) -> Vec<CosmicFilament> {
        let mut filaments = Vec::new();

        // Use hierarchy connections as filaments - positions are stored directly
        for conn in &self.cached_connections {
            filaments.push(CosmicFilament {
                start: [
                    conn.from_position[0] as f64,
                    conn.from_position[1] as f64,
                    conn.from_position[2] as f64,
                ],
                end: [
                    conn.to_position[0] as f64,
                    conn.to_position[1] as f64,
                    conn.to_position[2] as f64,
                ],
                density: conn.intensity as f64,
            });

            // Limit filaments for performance
            if filaments.len() >= 100 {
                break;
            }
        }

        filaments
    }

    /// Get star vertices for CosmosRenderer
    pub fn get_star_vertices(&self) -> Vec<crate::gui::renderer::cosmos_renderer::StarVertex> {
        use crate::gui::renderer::cosmos_renderer::StarVertex;

        let cosmic_data = self.get_cosmic_render_data();

        cosmic_data
            .stellar_systems
            .iter()
            .map(|system| StarVertex {
                position: [
                    system.star_position[0] as f32,
                    system.star_position[1] as f32,
                    system.star_position[2] as f32,
                ],
                color: system.star_color,
                luminosity: system.star_luminosity as f32,
                temperature: 5778.0 + system.star_luminosity as f32 * 100.0, // Scale temperature with luminosity
                radius: (system.star_luminosity as f32).sqrt() * 0.1,
                stage: 1, // Main sequence
                _padding: [0.0, 0.0],
            })
            .collect()
    }

    /// Get planet vertices for CosmosRenderer
    pub fn get_planet_vertices(&self) -> Vec<crate::gui::renderer::cosmos_renderer::PlanetVertex> {
        use crate::gui::renderer::cosmos_renderer::PlanetVertex;

        let mut vertices = Vec::new();
        let cosmic_data = self.get_cosmic_render_data();

        for system in &cosmic_data.stellar_systems {
            for planet in &system.planets {
                // Calculate planet position from orbital parameters
                let x = system.star_position[0] + planet.orbit_radius * planet.current_angle.cos();
                let z = system.star_position[2] + planet.orbit_radius * planet.current_angle.sin();
                let y = system.star_position[1]; // Assume flat orbital plane for now

                vertices.push(PlanetVertex {
                    position: [x as f32, y as f32, z as f32],
                    normal: [0.0, 1.0, 0.0], // Simple up normal
                    uv: [0.5, 0.5],
                    planet_type: self.planet_type_to_u32(&planet.planet_type),
                    orbital_radius: planet.orbit_radius as f32,
                    radius: 0.02,       // Small planet size
                    temperature: 288.0, // Earth-like temperature
                    _padding: [0.0],
                });
            }
        }

        vertices
    }

    /// Convert planet type string to u32 for rendering
    fn planet_type_to_u32(&self, planet_type: &str) -> u32 {
        match planet_type {
            "Terrestrial" => 0,
            "GasGiant" => 1,
            "IceGiant" => 2,
            "Dwarf" => 3,
            "Desert" => 0,
            "Ocean" => 0,
            "Ice" => 2,
            _ => 0,
        }
    }

    /// Get orbit vertices for CosmosRenderer
    pub fn get_orbit_vertices(&self) -> Vec<crate::gui::renderer::cosmos_renderer::OrbitVertex> {
        use crate::gui::renderer::cosmos_renderer::OrbitVertex;

        let mut vertices = Vec::new();
        let cosmic_data = self.get_cosmic_render_data();

        for system in &cosmic_data.stellar_systems {
            for planet in &system.planets {
                // Generate circle points for orbit
                let segments = 64;
                for i in 0..segments {
                    let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                    let _angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

                    let x1 =
                        system.star_position[0] as f32 + planet.orbit_radius as f32 * angle1.cos();
                    let z1 =
                        system.star_position[2] as f32 + planet.orbit_radius as f32 * angle1.sin();
                    let y1 = system.star_position[1] as f32;

                    vertices.push(OrbitVertex {
                        position: [x1, y1, z1],
                        color: [0.3, 0.3, 0.4], // Dim blue-gray for orbits
                        alpha: 0.3,
                        _padding: [0.0],
                    });
                }
            }
        }

        vertices
    }

    /// Get filament vertices for CosmosRenderer
    pub fn get_filament_vertices(
        &self,
    ) -> Vec<crate::gui::renderer::cosmos_renderer::FilamentVertex> {
        use crate::gui::renderer::cosmos_renderer::FilamentVertex;

        let cosmic_data = self.get_cosmic_render_data();

        cosmic_data
            .filaments
            .iter()
            .flat_map(|filament| {
                // Create two vertices per filament (line)
                vec![
                    FilamentVertex {
                        position: [
                            filament.start[0] as f32,
                            filament.start[1] as f32,
                            filament.start[2] as f32,
                        ],
                        color: [0.5, 0.3, 0.8], // Purple for cosmic filaments
                        density: filament.density as f32,
                        _padding: [0.0],
                    },
                    FilamentVertex {
                        position: [
                            filament.end[0] as f32,
                            filament.end[1] as f32,
                            filament.end[2] as f32,
                        ],
                        color: [0.5, 0.3, 0.8],
                        density: filament.density as f32,
                        _padding: [0.0],
                    },
                ]
            })
            .collect()
    }

    // ========================================================================
    // Phase 4: Planet Surface Data Bridging
    // ========================================================================

    /// Get planet surface render data for a specific planet
    ///
    /// This bridges Lithosphere, Hydrosphere, Atmosphere, and EnergyFlow
    /// to the PlanetRenderer.
    pub fn get_planet_render_data(&self, planet_id: u64) -> PlanetRenderData {
        // Generate procedural planet data based on entity distribution
        // In a full implementation, this would extract from actual Planet struct

        let mut terrain: Vec<Vec<f64>> = vec![vec![0.0; 128]; 256];
        let mut water_depth: Vec<Vec<f64>> = vec![vec![0.0; 128]; 256];
        let mut cloud_coverage: Vec<Vec<f64>> = vec![vec![0.0; 128]; 256];

        // Generate terrain heightmap from entity consciousness levels
        for entity in &self.cached_entities {
            let lat = ((entity.position[1] + 1.0) / 2.0 * 127.0) as usize;
            let lon = ((entity.position[0] + 1.0) / 2.0 * 255.0) as usize;

            if lat < 128 && lon < 256 {
                // Height based on consciousness level
                let height = entity.consciousness_level as f64 * 0.5;
                terrain[lon][lat] = terrain[lon][lat].max(height);

                // Water based on veil transparency (lower = more water)
                if entity.veil_transparency < 0.3 {
                    water_depth[lon][lat] = (0.3 - entity.veil_transparency as f64) * 2.0;
                }

                // Clouds based on archetype activations
                cloud_coverage[lon][lat] = entity.density_level as f64 * 0.1;
            }
        }

        // Generate storms from high-intensity entities
        let storms = self
            .cached_entities
            .iter()
            .filter(|e| e.consciousness_level > 0.8)
            .take(10)
            .map(|e| StormData {
                position: (e.position[0] as f64, e.position[1] as f64),
                intensity: e.consciousness_level as f64,
                radius: 0.1,
            })
            .collect();

        // Generate settlements from mid-consciousness entities (civilizations)
        let settlements = self
            .cached_entities
            .iter()
            .filter(|e| e.consciousness_level > 0.4 && e.consciousness_level < 0.7)
            .take(50)
            .map(|e| SettlementData {
                position: (e.position[0] as f64, e.position[1] as f64),
                population: (e.consciousness_level as f64 * 100000.0) as u64,
                settlement_type: if e.consciousness_level > 0.6 {
                    "City"
                } else {
                    "Town"
                }
                .to_string(),
            })
            .collect();

        PlanetRenderData {
            planet_id,
            terrain,
            water_depth,
            cloud_coverage,
            storms,
            settlements,
        }
    }

    /// Get terrain vertices for PlanetRenderer
    pub fn get_terrain_vertices(
        &self,
    ) -> Vec<crate::gui::renderer::planet_renderer::TerrainVertex> {
        use crate::gui::renderer::planet_renderer::TerrainVertex;

        let mut vertices = Vec::new();
        let segments = 64;

        // Generate sphere mesh for terrain
        for lat in 0..segments {
            for lon in 0..segments {
                let lat_angle = (lat as f32 / segments as f32) * std::f32::consts::PI;
                let lon_angle = (lon as f32 / segments as f32) * std::f32::consts::TAU;

                let y = lat_angle.cos();
                let x = lat_angle.sin() * lon_angle.cos();
                let z = lat_angle.sin() * lon_angle.sin();

                // Calculate UV
                let u = lon as f32 / segments as f32;
                let v = lat as f32 / segments as f32;

                // Get height from entity data
                let height = self.get_terrain_height_at(u, v);

                vertices.push(TerrainVertex {
                    position: [
                        x * (1.0 + height * 0.02),
                        y * (1.0 + height * 0.02),
                        z * (1.0 + height * 0.02),
                    ],
                    normal: [x, y, z], // Simple outward normal
                    uv: [u, v],
                    plate_id: (lat % 7) as f32, // 7 tectonic plates
                    volcanic_activity: if height > 0.6 { 0.5 } else { 0.0 },
                    _padding: [0.0, 0.0],
                });
            }
        }

        vertices
    }

    /// Get terrain height at UV coordinates
    fn get_terrain_height_at(&self, u: f32, v: f32) -> f32 {
        // Sample from entity data
        let mut max_height = 0.0f32;

        for entity in &self.cached_entities {
            let entity_u = (entity.position[0] + 1.0) / 2.0;
            let entity_v = (entity.position[1] + 1.0) / 2.0;

            let dist = ((entity_u - u).powi(2) + (entity_v - v).powi(2)).sqrt();
            if dist < 0.1 {
                max_height = max_height.max(entity.consciousness_level * (1.0 - dist * 10.0));
            }
        }

        max_height
    }

    /// Get water vertices for PlanetRenderer
    pub fn get_water_vertices(&self) -> Vec<crate::gui::renderer::planet_renderer::WaterVertex> {
        use crate::gui::renderer::planet_renderer::WaterVertex;

        let mut vertices = Vec::new();
        let segments = 64;

        // Generate sphere mesh for water surface
        for lat in 0..segments {
            for lon in 0..segments {
                let lat_angle = (lat as f32 / segments as f32) * std::f32::consts::PI;
                let lon_angle = (lon as f32 / segments as f32) * std::f32::consts::TAU;

                let y = lat_angle.cos();
                let x = lat_angle.sin() * lon_angle.cos();
                let z = lat_angle.sin() * lon_angle.sin();

                let u = lon as f32 / segments as f32;
                let v = lat as f32 / segments as f32;

                // Water depth from entity veil transparency
                let depth = self.get_water_depth_at(u, v);

                // Only add water where there is water
                if depth > 0.0 {
                    vertices.push(WaterVertex {
                        position: [x * 1.001, y * 1.001, z * 1.001], // Slightly above terrain
                        normal: [x, y, z],
                        uv: [u, v],
                        depth,
                        _padding: [0.0],
                    });
                }
            }
        }

        vertices
    }

    /// Get water depth at UV coordinates
    fn get_water_depth_at(&self, u: f32, v: f32) -> f32 {
        let mut max_depth = 0.0f32;

        for entity in &self.cached_entities {
            if entity.veil_transparency < 0.3 {
                let entity_u = (entity.position[0] + 1.0) / 2.0;
                let entity_v = (entity.position[1] + 1.0) / 2.0;

                let dist = ((entity_u - u).powi(2) + (entity_v - v).powi(2)).sqrt();
                if dist < 0.1 {
                    let depth = (0.3 - entity.veil_transparency) * (1.0 - dist * 10.0);
                    max_depth = max_depth.max(depth);
                }
            }
        }

        max_depth
    }

    /// Get cloud vertices for PlanetRenderer
    pub fn get_cloud_vertices(&self) -> Vec<crate::gui::renderer::planet_renderer::CloudVertex> {
        use crate::gui::renderer::planet_renderer::CloudVertex;

        let mut vertices = Vec::new();
        let segments = 48; // Lower resolution for clouds

        // Generate sphere mesh for cloud layer
        for lat in 0..segments {
            for lon in 0..segments {
                let lat_angle = (lat as f32 / segments as f32) * std::f32::consts::PI;
                let lon_angle = (lon as f32 / segments as f32) * std::f32::consts::TAU;

                let y = lat_angle.cos();
                let x = lat_angle.sin() * lon_angle.cos();
                let z = lat_angle.sin() * lon_angle.sin();

                let u = lon as f32 / segments as f32;
                let v = lat as f32 / segments as f32;

                // Cloud density from entity polarization
                let density = self.get_cloud_density_at(u, v);

                // Only add clouds where there are clouds
                if density > 0.1 {
                    vertices.push(CloudVertex {
                        position: [x, y, z],
                        uv: [u, v],
                        density,
                        _padding: [0.0, 0.0],
                    });
                }
            }
        }

        vertices
    }

    /// Get cloud density at UV coordinates
    fn get_cloud_density_at(&self, u: f32, v: f32) -> f32 {
        let mut max_density = 0.0f32;

        for entity in &self.cached_entities {
            let entity_u = (entity.position[0] + 1.0) / 2.0;
            let entity_v = (entity.position[1] + 1.0) / 2.0;

            let dist = ((entity_u - u).powi(2) + (entity_v - v).powi(2)).sqrt();
            if dist < 0.15 {
                let density = entity.polarization * (1.0 - dist * 7.0);
                max_density = max_density.max(density);
            }
        }

        max_density
    }

    /// Get storm vertices for PlanetRenderer
    pub fn get_storm_vertices(&self) -> Vec<crate::gui::renderer::planet_renderer::StormVertex> {
        use crate::gui::renderer::planet_renderer::StormVertex;

        self.cached_entities
            .iter()
            .filter(|e| e.consciousness_level > 0.8)
            .take(20)
            .map(|e| StormVertex {
                position: [e.position[0], e.position[1], e.position[2]],
                size: e.consciousness_level * 0.05,
                intensity: e.consciousness_level,
                rotation: e.space_time_ratio * std::f32::consts::TAU,
                _padding: [0.0, 0.0],
            })
            .collect()
    }

    /// Get settlement vertices for PlanetRenderer
    pub fn get_settlement_vertices(
        &self,
    ) -> Vec<crate::gui::renderer::planet_renderer::SettlementVertex> {
        use crate::gui::renderer::planet_renderer::SettlementVertex;

        self.cached_entities
            .iter()
            .filter(|e| e.consciousness_level > 0.3 && e.consciousness_level < 0.8)
            .take(100)
            .map(|e| {
                let settlement_type = if e.consciousness_level > 0.6 {
                    3.0 // City
                } else if e.consciousness_level > 0.5 {
                    2.0 // Town
                } else if e.consciousness_level > 0.4 {
                    1.0 // Village
                } else {
                    0.0 // Hamlet
                };

                SettlementVertex {
                    position: [e.position[0], e.position[1], e.position[2]],
                    size: e.consciousness_level * 0.02,
                    tech_level: e.consciousness_level,
                    settlement_type,
                    _padding: [0.0, 0.0],
                }
            })
            .collect()
    }
}

impl Default for SimulationRunnerAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DATA STRUCTURES FOR MULTI-SCALE RENDERING
// ============================================================================

/// Data structure for cosmic rendering (Phase 3)
#[derive(Debug, Clone)]
pub struct CosmicRenderData {
    /// Cosmic web filaments
    pub filaments: Vec<CosmicFilament>,

    /// Stellar systems in view
    pub stellar_systems: Vec<StellarSystemData>,

    /// Selected star for detail view
    pub selected_star: Option<u64>,

    /// Selected planet for detail view
    pub selected_planet: Option<u64>,
}

/// Cosmic web filament for rendering
#[derive(Debug, Clone)]
pub struct CosmicFilament {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub density: f64,
}

/// Stellar system data for rendering
#[derive(Debug, Clone)]
pub struct StellarSystemData {
    pub id: u64,
    pub star_position: [f64; 3],
    pub star_color: [f32; 3],
    pub star_luminosity: f64,
    pub planets: Vec<PlanetOrbitData>,
}

/// Planet orbit data for rendering
#[derive(Debug, Clone)]
pub struct PlanetOrbitData {
    pub id: u64,
    pub orbit_radius: f64,
    pub orbit_period: f64,
    pub current_angle: f64,
    pub planet_type: String,
}

/// Data structure for planet surface rendering (Phase 4)
#[derive(Debug, Clone)]
pub struct PlanetRenderData {
    pub planet_id: u64,

    /// Terrain heightmap
    pub terrain: Vec<Vec<f64>>,

    /// Water depth map
    pub water_depth: Vec<Vec<f64>>,

    /// Cloud coverage
    pub cloud_coverage: Vec<Vec<f64>>,

    /// Storm positions
    pub storms: Vec<StormData>,

    /// Settlements on surface (Phase 6)
    pub settlements: Vec<SettlementData>,
}

/// Storm data for planet visualization
#[derive(Debug, Clone)]
pub struct StormData {
    pub position: (f64, f64),
    pub intensity: f64,
    pub radius: f64,
}

/// Settlement data for planet surface
#[derive(Debug, Clone)]
pub struct SettlementData {
    pub position: (f64, f64),
    pub population: u64,
    pub settlement_type: String,
}

/// Data structure for civilization rendering (Phase 6)
#[derive(Debug, Clone)]
pub struct CivilizationRenderData {
    pub civilizations: Vec<CivilizationSummary>,
    pub trade_routes: Vec<TradeRoute>,
    pub population_density: Vec<PopulationDensity>,
}

/// Trade route between settlements
#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub from_settlement: u64,
    pub to_settlement: u64,
    pub volume: f64,
}

/// Population density data point
#[derive(Debug, Clone)]
pub struct PopulationDensity {
    pub position: (f64, f64),
    pub density: f64,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let adapter = SimulationRunnerAdapter::new();
        assert!(!adapter.is_initialized());
        assert_eq!(adapter.current_step(), 0);
    }

    #[test]
    fn test_adapter_initialization() {
        let mut adapter = SimulationRunnerAdapter::new();
        adapter.initialize();
        assert!(adapter.is_initialized());
        // After initialization, we should have entities
    }

    #[test]
    fn test_adapter_step() {
        let mut adapter = SimulationRunnerAdapter::new();
        adapter.initialize();
        let initial_step = adapter.current_step();
        adapter.step();
        assert!(adapter.current_step() >= initial_step);
    }

    #[test]
    fn test_entity_extraction() {
        let mut adapter = SimulationRunnerAdapter::new();
        adapter.initialize();

        let entities = adapter.get_entities();
        // Should have extracted some entities
        // The exact count depends on the involution process
        println!("Extracted {} entities", entities.len());
    }

    #[test]
    fn test_get_observable_properties_returns_correct_count() {
        let adapter = SimulationRunnerAdapter::new();
        let props = adapter.get_observable_properties();
        assert_eq!(props.len(), adapter.raw_entities.len());
    }

    #[test]
    fn test_get_observable_properties_are_unique() {
        let mut adapter = SimulationRunnerAdapter::new();
        adapter.initialize();
        let props = adapter.get_observable_properties();
        assert!(!props.is_empty());
        let all_at_origin = props.iter().all(|p| p.position == [0.0, 0.0]);
        assert!(!all_at_origin, "Entities should have spread positions");
        let all_default_color = props.iter().all(|p| p.color == [0.5, 0.5, 0.5]);
        assert!(
            !all_default_color,
            "Entities should have unique archetype-based colors"
        );
    }

    #[test]
    fn test_get_observable_properties_valid_ranges() {
        let mut adapter = SimulationRunnerAdapter::new();
        adapter.initialize();
        let props = adapter.get_observable_properties();
        for p in &props {
            assert!(
                p.density >= 1 && p.density <= 8,
                "Density {} out of range",
                p.density
            );
            assert!(
                p.glow >= 0.0 && p.glow <= 1.0,
                "Glow {} out of range",
                p.glow
            );
            assert!(p.size > 0.0, "Size should be positive");
            assert!(
                p.intelligence >= 0.0 && p.intelligence <= 1.0,
                "Intelligence {} out of range",
                p.intelligence
            );
        }
    }
}
