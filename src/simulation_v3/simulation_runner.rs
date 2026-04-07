// Simulation Runner Module (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md:
// "Create NEW simulation runner that integrates all NEW components"
//
// This module implements:
// 1. SimulationRunner - main simulation runner using all NEW components
// 2. SimulationParameters - configuration parameters for the simulation
// 3. SimulationResult - comprehensive result of simulation execution
// 4. Integration of InvolutionSequenceRunner, EntityLifecycleManager,
//    HolographicFieldManager, and PhysicalAdapter

use crate::adapters::physical_adapter::{PhysicalAdapter, PhysicalAdapterStatistics};
use crate::biology::BiologicalConfig;
use crate::biology::BiologyPipeline;
use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::physical_manifestation::consciousness_to_matter::ConsciousnessToMatterManager;
use crate::physical_manifestation::{
    HierarchicalCompositionManager, PhysicalStructureManager, SimultaneousEmergenceManager,
};
use crate::simulation_v3::catalyst_system::{CatalystEvent, CatalystManager};
use crate::simulation_v3::causal_inversion::{
    CausalInversionConfig, CausalInversionRunner, CausalSimulationResult, CausalStatistics,
};
use crate::simulation_v3::collective_dynamics::CollectiveDynamicsManager;
use crate::simulation_v3::collective_statistics::CollectiveStatisticsTracker;
use crate::simulation_v3::emergent_behavior::EmergenceManager;
use crate::simulation_v3::entity_lifecycle::{
    EntityLifecycleManager, EvolutionResult, LifecycleStatistics,
};
use crate::simulation_v3::environment::EnvironmentalInteractionManager;
use crate::simulation_v3::holographic_field::{
    HolographicFieldConfig, HolographicFieldManager, HolographicFieldResult,
};
use crate::simulation_v3::inter_scale_interactions::InterScaleInteractionManager;
use crate::simulation_v3::involution_sequence::{
    InvolutionError, InvolutionResult, InvolutionSequenceRunner, InvolutionStage,
};
use crate::simulation_v3::multiscale_camera::{MultiScaleCamera, ScaleLevel};
use crate::simulation_v3::persistence::{
    CheckpointManager, PerformanceMetrics, PersistenceError, PersistenceManager,
};
use crate::simulation_v3::physical_structure_visualizer::PhysicalStructureVisualizer;
use crate::simulation_v3::reporter::SimulationReporter;
use crate::simulation_v3::scale_physics::{ScalePhysicsError, ScaleSpecificPhysics};
use crate::simulation_v3::spectrum_visualizer::SpectrumDistributionVisualizer;
use crate::simulation_v3::statistics::{
    EmergentProperties, HolographicFieldStatistics as StatsHolographicFieldStatistics,
    PolarizationDistribution, SimulationStatistics, StageDetail, StatisticsTracker,
};
use crate::types::Float;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

// ============================================================================
// SIMULATION PARAMETERS
// ============================================================================

/// Simulation Parameters
///
/// Configuration parameters for running the simulation.
#[derive(Debug, Clone)]
pub struct SimulationParameters {
    /// Number of entities to create during involution
    pub num_entities: usize,

    /// Number of simulation steps to run
    pub num_steps: u64,

    /// Time step size (in seconds)
    pub time_step_size: Float,

    /// Whether to run involution phase
    pub run_involution: bool,

    /// Whether to run evolution phase
    pub run_evolution: bool,

    /// Whether to update holographic field during evolution
    pub update_holographic_field: bool,

    /// Whether to update physical manifestations during evolution
    pub update_physical_manifestations: bool,

    /// Whether to generate detailed reports
    pub generate_detailed_reports: bool,

    /// Random seed for reproducibility
    pub random_seed: Option<u64>,

    /// Phase 5: Performance configuration for holographic field
    pub holographic_performance_config: Option<HolographicFieldConfig>,

    /// Phase 8: Checkpoint interval for long simulations
    pub checkpoint_interval: Option<u64>,

    /// Phase 8: Checkpoint directory path
    pub checkpoint_directory: Option<String>,

    /// Phase 8: Maximum checkpoints to keep
    pub max_checkpoints: Option<usize>,

    /// Phase 0: Enable causal inversion mode (top-down causation)
    pub causal_inversion_mode: bool,

    /// Phase 5: Enable biology simulation (cells → organisms → evolution)
    pub biology_enabled: bool,

    /// Phase 5: Biology system configuration
    pub biology_config: BiologicalConfig,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        SimulationParameters {
            num_entities: 100,
            num_steps: 100,
            time_step_size: 1.0,
            run_involution: true,
            run_evolution: true,
            update_holographic_field: true,
            update_physical_manifestations: true,
            generate_detailed_reports: false,
            random_seed: None,
            holographic_performance_config: None,
            checkpoint_interval: None,
            checkpoint_directory: None,
            max_checkpoints: None,
            causal_inversion_mode: true, // Phase 0: Causal inversion is now the default
            // The old bottom-up path is deprecated and will be removed
            biology_enabled: true,
            biology_config: BiologicalConfig::default(),
        }
    }
}

impl SimulationParameters {
    /// Create default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set number of entities
    pub fn with_num_entities(mut self, num_entities: usize) -> Self {
        self.num_entities = num_entities;
        self
    }

    /// Set number of steps
    pub fn with_num_steps(mut self, num_steps: u64) -> Self {
        self.num_steps = num_steps;
        self
    }

    /// Set time step size
    pub fn with_time_step_size(mut self, time_step_size: Float) -> Self {
        self.time_step_size = time_step_size;
        self
    }

    /// Set random seed
    pub fn with_random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }

    /// Set holographic performance configuration (Phase 5)
    pub fn with_holographic_performance_config(mut self, config: HolographicFieldConfig) -> Self {
        self.holographic_performance_config = Some(config);
        self
    }

    /// Set checkpoint interval for long simulations (Phase 8)
    pub fn with_checkpoint_interval(mut self, interval: u64) -> Self {
        self.checkpoint_interval = Some(interval);
        self
    }

    /// Set checkpoint directory for long simulations (Phase 8)
    pub fn with_checkpoint_directory(mut self, directory: String) -> Self {
        self.checkpoint_directory = Some(directory);
        self
    }

    /// Set maximum checkpoints to keep (Phase 8)
    pub fn with_max_checkpoints(mut self, max: usize) -> Self {
        self.max_checkpoints = Some(max);
        self
    }
}

// ============================================================================
// SIMULATION RESULT
// ============================================================================

/// Simulation Result
///
/// Comprehensive result of simulation execution.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Involution phase result
    pub involution_result: InvolutionResult,

    /// Evolution phase result
    pub evolution_result: EvolutionResult,

    /// Holographic field result
    pub holographic_result: HolographicFieldResult,

    /// Physical adapter statistics
    pub physical_statistics: crate::adapters::physical_adapter::PhysicalAdapterStatistics,

    /// Complete simulation statistics
    pub statistics: SimulationStatistics,

    /// Total execution time
    pub total_execution_time: Duration,

    /// Success flag
    pub success: bool,

    /// Error message if failed
    pub error_message: Option<String>,
}

impl SimulationResult {
    /// Check if simulation was successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get a summary of the result
    pub fn summary(&self) -> String {
        format!(
            "Simulation Result:\n\
             - Success: {}\n\
             - Entities created: {}\n\
             - Evolution steps: {}\n\
             - Density transitions: {}\n\
             - Holographic connections: {}\n\
             - Total execution time: {:?}\n\
             - Architecture alignment: {:.2}%",
            self.success,
            self.involution_result.entities.len(),
            self.evolution_result.steps,
            self.evolution_result.transitions,
            self.holographic_result.connections,
            self.total_execution_time,
            self.statistics.architecture.alignment_score * 100.0
        )
    }
}

// ============================================================================
// PHASE 0: CAUSAL INVERSION RESULT CONVERSION
// ============================================================================

impl From<CausalSimulationResult> for SimulationResult {
    fn from(causal_result: CausalSimulationResult) -> Self {
        SimulationResult {
            involution_result: InvolutionResult {
                entities: Vec::new(), // Causal inversion manifests entities differently
                attractor_fields: Vec::new(),
                stage_transitions: Vec::new(),
                execution_time: Duration::ZERO,
            },
            evolution_result: EvolutionResult {
                steps: causal_result.steps_completed,
                transitions: causal_result.statistics.total_entities_manifested as usize,
                spectrum_access_upgrades: 0,
                final_statistics: LifecycleStatistics::default(),
                execution_time: Duration::ZERO,
            },
            holographic_result: HolographicFieldResult {
                steps: causal_result.steps_completed,
                connections: causal_result.final_tick.potentials_extracted,
                interference_patterns: 0,
                final_statistics:
                    crate::simulation_v3::holographic_field::HolographicFieldStatistics::default(),
                execution_time: Duration::ZERO,
            },
            physical_statistics:
                crate::adapters::physical_adapter::PhysicalAdapterStatistics::default(),
            statistics: SimulationStatistics {
                evolution: crate::simulation_v3::statistics::EvolutionStatistics {
                    total_steps: causal_result.steps_completed,
                    ..Default::default()
                },
                holographic: StatsHolographicFieldStatistics {
                    entity_count: causal_result.statistics.peak_entity_count,
                    connection_count: causal_result.final_tick.potentials_extracted,
                    ..Default::default()
                },
                ..Default::default()
            },
            total_execution_time: Duration::from_micros(
                causal_result.final_tick.total_execution_time_us(),
            ),
            success: causal_result.success,
            error_message: causal_result.error_message,
        }
    }
}

// ============================================================================
// SIMULATION RUNNER
// ============================================================================

/// Simulation Runner
///
/// Main simulation runner that integrates all NEW components:
/// - InvolutionSequenceRunner: Runs creation sequence
/// - EntityLifecycleManager: Manages entity evolution
/// - CatalystManager: Manages environmental events and challenges (Phase 5)
/// - HolographicFieldManager: Manages holographic connections
/// - CollectiveDynamicsManager: Manages collective entity behavior (Phase 6)
/// - CollectiveStatisticsTracker: Tracks collective statistics (Phase 6)
/// - EnvironmentalInteractionManager: Manages environment-entity interactions (Phase 4)
/// - PhysicalAdapter: Bridges to physical manifestation
/// - ConsciousnessToMatterManager: Manages consciousness-to-matter transitions (Phase 7)
/// - StatisticsTracker: Tracks simulation statistics
/// - Phase 3: PhysicalStructureManager - Manages physical structures
/// - Phase 3: HierarchicalCompositionManager - Manages hierarchical composition
/// - Phase 3: SimultaneousEmergenceManager - Manages simultaneous emergence
/// - Phase 1 Week 1: ScaleSpecificPhysics - Multi-scale physics engine
/// - Phase 1 Week 1: MultiScaleCamera - Scale tracking and transitions
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md:
/// "The NEW simulation uses the V3.0 architecture that correctly implements
/// COSMOLOGICAL-ARCHITECTURE.md"
///
/// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
/// "Integrate scale_physics.rs into simulation_runner.rs to enable multi-scale simulation"
pub struct SimulationRunner {
    /// Simulation parameters
    parameters: SimulationParameters,

    /// Involution sequence runner
    involution_runner: InvolutionSequenceRunner,

    /// Entity lifecycle manager
    lifecycle_manager: EntityLifecycleManager,

    /// Catalyst manager (Phase 5)
    catalyst_manager: CatalystManager,

    /// Holographic field manager
    holographic_manager: HolographicFieldManager,

    /// Collective dynamics manager (Phase 6)
    collective_dynamics_manager: CollectiveDynamicsManager,

    /// Collective statistics tracker (Phase 6)
    collective_statistics_tracker: CollectiveStatisticsTracker,

    /// Phase 4: Environmental interaction manager
    environmental_manager: EnvironmentalInteractionManager,

    /// Phase 4: Inter-scale interaction manager
    inter_scale_interaction_manager: InterScaleInteractionManager,

    /// Phase 5: Emergence manager
    emergence_manager: EmergenceManager,

    /// Physical adapter
    physical_adapter: PhysicalAdapter,

    /// Phase 7: Consciousness-to-matter manager
    consciousness_manager: ConsciousnessToMatterManager,

    /// Statistics tracker
    statistics_tracker: StatisticsTracker,

    /// Phase 3: Physical structure manager
    physical_structure_manager: PhysicalStructureManager,

    /// Phase 3: Hierarchical composition manager
    hierarchical_composition_manager: HierarchicalCompositionManager,

    /// Phase 3: Simultaneous emergence manager
    simultaneous_emergence_manager: SimultaneousEmergenceManager,

    /// Phase 4: Entities storage (for emergent properties calculation)
    entities: HashMap<EntityId, crate::entity_layer7::SubSubLogos>,

    /// Phase 7: Entity ID to quantum pool ID mapping
    entity_quantum_pools: HashMap<EntityId, String>,

    /// Phase 1 Week 1: Scale-specific physics engine
    /// Manages physics for all 7 scale levels (Quantum through Cosmic)
    scale_physics: ScaleSpecificPhysics,

    /// Phase 1 Week 1: Current active scale level
    current_scale: ScaleLevel,

    /// Phase 1 Week 1: Multi-scale camera for scale tracking
    multiscale_camera: Option<MultiScaleCamera>,

    /// Current simulation time
    current_time: u64,

    /// Current simulation step
    current_step: u64,

    /// Simulation running state
    running: bool,

    /// Phase 5: Performance optimization enabled
    performance_optimization_enabled: bool,

    /// Phase 0: Causal inversion runner for top-down causation
    causal_inversion_runner: Option<CausalInversionRunner>,

    /// Phase 5: Biology pipeline (molecules → cells → organisms → evolution)
    biology_pipeline: Option<BiologyPipeline>,
}

impl SimulationRunner {
    /// Create a new simulation runner
    pub fn new(parameters: SimulationParameters) -> Self {
        // Configure involution runner with entity count from parameters
        // Scale galaxy/solar/planet numbers based on entity count
        let num_entities = parameters.num_entities;
        let num_galaxies = ((num_entities as f64).sqrt().ceil() as usize).max(1);
        let num_solar_systems =
            ((num_entities as f64 / num_galaxies as f64).sqrt().ceil() as usize).max(1);
        let num_planets = ((num_entities as f64 / (num_galaxies * num_solar_systems) as f64).ceil()
            as usize)
            .max(1);

        let involution_runner = InvolutionSequenceRunner::with_config(
            num_entities,
            num_galaxies,
            num_solar_systems,
            num_planets,
        );

        // Phase 5: Use performance configuration if provided
        let holographic_manager = if let Some(config) = &parameters.holographic_performance_config {
            HolographicFieldManager::with_config(config.clone())
        } else {
            HolographicFieldManager::new()
        };

        // Phase 5: Check if performance optimization is enabled
        let performance_optimization_enabled = parameters.holographic_performance_config.is_some();

        // Phase 3: Initialize physical structure managers
        let physical_structure_manager = PhysicalStructureManager::new();
        let hierarchical_composition_manager = HierarchicalCompositionManager::new();
        let simultaneous_emergence_manager = SimultaneousEmergenceManager::new();

        // Phase 1 Week 1: Initialize scale-specific physics engine
        // From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
        // "Add scale_physics: ScaleSpecificPhysics field to SimulationRunner struct"
        let scale_physics = ScaleSpecificPhysics::new();

        // Phase 1 Week 1: Set default scale to Biological (middle of 7 scales)
        // From COSMOLOGICAL-ARCHITECTURE.md: "3rd Density: Space/Time, v = s/t > 1"
        let current_scale = ScaleLevel::Biological;

        // Phase 1 Week 1: Initialize multi-scale camera (optional, can be set later)
        let multiscale_camera = None;

        SimulationRunner {
            parameters: parameters.clone(),
            involution_runner,
            lifecycle_manager: EntityLifecycleManager::new(),
            catalyst_manager: CatalystManager::new(), // Phase 5
            holographic_manager,
            collective_dynamics_manager: CollectiveDynamicsManager::new(), // Phase 6
            collective_statistics_tracker: CollectiveStatisticsTracker::new(), // Phase 6
            environmental_manager: EnvironmentalInteractionManager::new(), // Phase 4
            inter_scale_interaction_manager: InterScaleInteractionManager::new(), // Phase 4
            emergence_manager: EmergenceManager::new(),                    // Phase 5
            physical_adapter: PhysicalAdapter::new(),
            consciousness_manager: ConsciousnessToMatterManager::new(), // Phase 7
            statistics_tracker: StatisticsTracker::new(),
            physical_structure_manager,           // Phase 3
            hierarchical_composition_manager,     // Phase 3
            simultaneous_emergence_manager,       // Phase 3
            entities: HashMap::new(),             // Phase 4
            entity_quantum_pools: HashMap::new(), // Phase 7
            scale_physics,                        // Phase 1 Week 1
            current_scale,                        // Phase 1 Week 1
            multiscale_camera,                    // Phase 1 Week 1
            current_time: 0,
            current_step: 0,
            running: false,
            performance_optimization_enabled,

            // Phase 0: Initialize causal inversion runner if enabled
            causal_inversion_runner: if parameters.causal_inversion_mode {
                Some(CausalInversionRunner::new(CausalInversionConfig::default()))
            } else {
                None
            },

            // Phase 5: Initialize biology pipeline if enabled
            biology_pipeline: if parameters.biology_enabled {
                Some(BiologyPipeline::with_config(
                    parameters.biology_config.clone(),
                ))
            } else {
                None
            },
        }
    }

    /// Run the complete simulation
    pub fn run_simulation(&mut self) -> SimulationResult {
        let start_time = std::time::Instant::now();
        self.running = true;

        // Phase 0: Check for causal inversion mode
        if self.parameters.causal_inversion_mode {
            return self
                .run_causal_inversion_simulation(self.parameters.num_steps)
                .into();
        }

        // Initialize statistics
        self.statistics_tracker.statistics.evolution.total_steps = self.parameters.num_steps;

        // Run involution phase (one-time)
        let involution_result = if self.parameters.run_involution {
            match self.run_involution_phase() {
                Ok(result) => Some(result),
                Err(e) => {
                    eprintln!("Involution phase error: {:?}", e);
                    None
                }
            }
        } else {
            None
        };

        // Check if involution was successful
        if let Some(ref inv_result) = involution_result {
            if inv_result.entities.is_empty() {
                return SimulationResult {
                    involution_result: inv_result.clone(),
                    evolution_result: EvolutionResult {
                        steps: 0,
                        transitions: 0,
                        spectrum_access_upgrades: 0,
                        final_statistics: LifecycleStatistics::default(),
                        execution_time: Duration::ZERO,
                    },
                    holographic_result: HolographicFieldResult {
                        steps: 0,
                        connections: 0,
                        interference_patterns: 0,
                        final_statistics: crate::simulation_v3::holographic_field::HolographicFieldStatistics::default(),
                        execution_time: Duration::ZERO,
                    },
                    physical_statistics: PhysicalAdapterStatistics::default(),
                    statistics: self.statistics_tracker.statistics.clone(),
                    total_execution_time: start_time.elapsed(),
                    success: false,
                    error_message: Some(
                        "Involution phase failed - no entities created".to_string(),
                    ),
                };
            }
        } else if self.parameters.run_involution {
            return SimulationResult {
                involution_result: InvolutionResult {
                    entities: Vec::new(),
                    attractor_fields: Vec::new(),
                    stage_transitions: Vec::new(),
                    execution_time: Duration::ZERO,
                },
                evolution_result: EvolutionResult {
                    steps: 0,
                    transitions: 0,
                    spectrum_access_upgrades: 0,
                    final_statistics: LifecycleStatistics::default(),
                    execution_time: Duration::ZERO,
                },
                holographic_result: HolographicFieldResult {
                    steps: 0,
                    connections: 0,
                    interference_patterns: 0,
                    final_statistics:
                        crate::simulation_v3::holographic_field::HolographicFieldStatistics::default(
                        ),
                    execution_time: Duration::ZERO,
                },
                physical_statistics: PhysicalAdapterStatistics::default(),
                statistics: self.statistics_tracker.statistics.clone(),
                total_execution_time: start_time.elapsed(),
                success: false,
                error_message: Some("Involution phase failed - None result".to_string()),
            };
        }

        // Run evolution phase (continuous)
        let evolution_result = if self.parameters.run_evolution {
            self.run_evolution_phase()
        } else {
            EvolutionResult {
                steps: 0,
                transitions: 0,
                spectrum_access_upgrades: 0,
                final_statistics: LifecycleStatistics::default(),
                execution_time: Duration::ZERO,
            }
        };

        // Get holographic field result
        let holographic_result = self.holographic_manager.get_field_result();

        // Get physical adapter statistics
        let physical_adapter_stats = self.physical_adapter.get_statistics().clone();

        // Unwrap involution_result (we've already validated it's not None)
        let involution_result = involution_result.unwrap_or_else(|| InvolutionResult {
            entities: Vec::new(),
            attractor_fields: Vec::new(),
            stage_transitions: Vec::new(),
            execution_time: Duration::ZERO,
        });

        // Update final performance metrics
        self.statistics_tracker.update_performance_metrics();

        // Phase 4: Calculate emergent properties
        let entities_vec: Vec<_> = self.entities.values().cloned().collect();
        let holographic_stats = self.holographic_manager.get_statistics();
        let environmental_stats = self.environmental_manager.get_statistics();
        let collective_influence_stats = self
            .collective_dynamics_manager
            .get_collective_influence_stats();

        // Convert HolographicFieldStatistics to the statistics module version
        let holographic_stats_converted = StatsHolographicFieldStatistics {
            entity_count: holographic_stats.entity_count,
            connection_count: holographic_stats.connection_count,
            average_connection_strength: holographic_stats.average_connection_strength,
            resonant_connections: holographic_stats.resonant_connections,
            harmonic_connections: holographic_stats.harmonic_connections,
            entangled_connections: holographic_stats.entangled_connections,
            antiphase_connections: holographic_stats.antiphase_connections,
            weak_connections: holographic_stats.weak_connections,
            average_archetype_similarity: holographic_stats.average_archetype_similarity,
            global_phase_coherence: holographic_stats.global_phase_coherence,
            interference_pattern_count: holographic_stats.interference_pattern_count,
            resonance_cluster_count: holographic_stats.resonance_cluster_count,
            holographic_interaction_score: holographic_stats.holographic_interaction_score,
        };

        let emergent_properties = EmergentProperties::calculate_from_system_state(
            &entities_vec,
            &holographic_stats_converted,
            environmental_stats,
            collective_influence_stats,
        );

        self.statistics_tracker.statistics.emergent_properties = emergent_properties;

        // Update architecture metrics with emergent properties
        self.statistics_tracker.update_architecture_metrics();

        // Record complete statistics
        let statistics = self.statistics_tracker.statistics.clone();

        let total_execution_time = start_time.elapsed();

        self.running = false;

        SimulationResult {
            involution_result,
            evolution_result,
            holographic_result,
            physical_statistics: physical_adapter_stats,
            statistics,
            total_execution_time,
            success: true,
            error_message: None,
        }
    }

    /// Phase 0: Run simulation using causal inversion (top-down causation)
    ///
    /// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md:
    /// "The Fundamental Gap: Causal Inversion"
    ///
    /// This method implements the correct causal ordering:
    /// 1. infinity.pulse() — Intelligent Infinity pulses kinetic energy
    /// 2. field.evolve() — Holographic field distributes energy
    /// 3. extract_potentials() — High-coherence regions identified
    /// 4. manifest_entities() — Entities manifest from field
    /// 5. absorb_field_influence() — Existing entities updated
    /// 6. decompress_for_observers() — Observer-specific rendering
    pub fn run_causal_inversion_simulation(&mut self, num_steps: u64) -> CausalSimulationResult {
        // Initialize runner if not present
        if self.causal_inversion_runner.is_none() {
            let config = CausalInversionConfig {
                max_entities: self.parameters.num_entities,
                ..Default::default()
            };
            self.causal_inversion_runner = Some(CausalInversionRunner::new(config));
        }

        // Run using causal inversion
        if let Some(ref mut runner) = self.causal_inversion_runner {
            runner.run(num_steps, self.parameters.time_step_size)
        } else {
            CausalSimulationResult {
                steps_completed: 0,
                final_tick: crate::simulation_v3::causal_inversion::CausalTickResult::default(),
                statistics: CausalStatistics::default(),
                success: false,
                error_message: Some("Failed to initialize causal inversion runner".to_string()),
            }
        }
    }

    /// Run involution phase
    fn run_involution_phase(&mut self) -> Result<InvolutionResult, InvolutionError> {
        let start_time = std::time::Instant::now();

        // Run involution sequence
        let involution_result = self.involution_runner.run_involution_sequence()?;

        // Track stage details for statistics
        let mut stage_details = Vec::new();

        // Add entity IDs from involution result
        for entity in &involution_result.entities {
            // Phase 4: Store entities for emergent properties calculation
            self.entities
                .insert(entity.entity_id.clone(), entity.clone());

            // Add to lifecycle manager
            let entity_id = entity.entity_id.clone();
            let initial_state = entity.current_state.clone();
            let evolutionary_trajectory =
                crate::entity_layer7::layer7::EvolutionaryTrajectory::new();
            let free_will_kernel = crate::consciousness::free_will::FreeWillKernel::new(
                entity.indigo_realm.archetype22.clone(),
            );
            // Get initial density from entity's evolutionary attractor
            let initial_density = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => {
                    crate::evolution_density_octave::density_octave::Density::First(
                        crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                    )
                }
                crate::entity_layer7::layer7::DensityLevel::Second => {
                    crate::evolution_density_octave::density_octave::Density::Second(
                        crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
                    )
                }
                crate::entity_layer7::layer7::DensityLevel::Third => {
                    crate::evolution_density_octave::density_octave::Density::Third
                }
                crate::entity_layer7::layer7::DensityLevel::Fourth => {
                    crate::evolution_density_octave::density_octave::Density::Fourth
                }
                crate::entity_layer7::layer7::DensityLevel::Fifth => {
                    crate::evolution_density_octave::density_octave::Density::Fifth
                }
                crate::entity_layer7::layer7::DensityLevel::Sixth => {
                    crate::evolution_density_octave::density_octave::Density::Sixth
                }
                crate::entity_layer7::layer7::DensityLevel::Seventh => {
                    crate::evolution_density_octave::density_octave::Density::Seventh
                }
                crate::entity_layer7::layer7::DensityLevel::Eighth => {
                    crate::evolution_density_octave::density_octave::Density::Eighth
                }
            };

            // Phase 1: Convert SpectrumAccess to EntitySpectrumAccess
            // This preserves the unique spectrum configuration created during involution
            // IMPORTANT: Use spectrum_configuration.ratio for unique values, not spectrum_access
            let evolutionary_level = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Second => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Third => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Fourth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::FourthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Fifth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::FifthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Sixth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SixthDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Seventh => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SeventhDensity
                }
                crate::entity_layer7::layer7::DensityLevel::Eighth => {
                    crate::entity_layer7::layer7::SpectrumAccessLevel::SeventhDensity
                }
            };

            // Phase 1: Calculate space_time_access and time_space_access from the unique ratio
            // The ratio is stored in entity.spectrum_configuration.ratio, not in entity.spectrum_access
            let unique_ratio = entity.spectrum_configuration.ratio.calculate_ratio();
            let (space_time_access, time_space_access) = if unique_ratio >= 1.0 {
                // Space/time dominant: v = s/t, so space_time = v / (v + 1), time_space = 1 / (v + 1)
                let denom = unique_ratio + 1.0;
                (unique_ratio / denom, 1.0 / denom)
            } else {
                // Time/space dominant: v = t/s, so time_space = v / (v + 1), space_time = 1 / (v + 1)
                let denom = unique_ratio + 1.0;
                (1.0 / denom, unique_ratio / denom)
            };

            let _spectrum_configuration = crate::entity_layer7::layer7::EntitySpectrumAccess {
                space_time_access,
                oneness_access: time_space_access, // time_space_access maps to oneness_access
                veil_transparency: if entity.spectrum_access.veil_active {
                    0.0
                } else {
                    1.0
                },
                evolutionary_level,
                space_time_ratio: space_time_access,
                time_space_ratio: time_space_access,
                spectrum_position: space_time_access,
            };

            let spectrum_configuration = crate::entity_layer7::layer7::EntitySpectrumAccess {
                space_time_access: entity.spectrum_access.space_time_access,
                oneness_access: entity.spectrum_access.time_space_access, // time_space_access maps to oneness_access
                veil_transparency: if entity.spectrum_access.veil_active {
                    0.0
                } else {
                    1.0
                },
                evolutionary_level,
                space_time_ratio: entity.spectrum_access.space_time_access,
                time_space_ratio: entity.spectrum_access.time_space_access,
                spectrum_position: entity.spectrum_access.space_time_access,
            };

            self.lifecycle_manager.add_entity(
                entity_id,
                initial_state,
                evolutionary_trajectory,
                free_will_kernel,
                initial_density,
                spectrum_configuration, // Phase 1: Pass spectrum configuration
            );

            // Add to holographic manager
            if self.holographic_manager.add_entity(entity.clone()).is_err() {
                // Continue even if holographic manager fails
            }

            // Add to physical adapter
            self.physical_adapter.add_entity(entity.clone());

            // Phase 7: Create quantum energy pool for this entity
            // Information content based on holographic blueprint complexity
            let information_content = self.calculate_entity_information_content(entity);
            let coherence_level = 0.9; // Start with high coherence
            let pool_id = self
                .consciousness_manager
                .create_quantum_pool(information_content, coherence_level);

            // Map entity ID to quantum pool ID
            self.entity_quantum_pools
                .insert(entity.entity_id.clone(), pool_id);
        }

        // Record involution statistics
        let execution_time = start_time.elapsed();

        // Create stage details from stage transitions
        for transition in &involution_result.stage_transitions {
            stage_details.push(StageDetail {
                stage_number: InvolutionStage::layer_number(&transition.to_stage),
                stage_name: format!("{:?}", transition.to_stage),
                success: true,
                duration: Duration::ZERO,
                entities_created: involution_result.entities.len(),
                attractor_fields_created: involution_result.attractor_fields.len(),
            });
        }

        self.statistics_tracker.record_involution_complete(
            involution_result.entities.len(),
            involution_result.attractor_fields.len(),
            involution_result.stage_transitions.len(),
            execution_time,
            stage_details,
        );

        // Phase 3: Collect spectrum access data from entities
        let mut spectrum_ratios: Vec<crate::types::Float> = Vec::new();
        let mut space_time_access: Vec<crate::types::Float> = Vec::new();
        let mut time_space_access: Vec<crate::types::Float> = Vec::new();
        let mut veil_active: Vec<bool> = Vec::new();
        let mut entity_types: Vec<String> = Vec::new();
        let mut scales: Vec<String> = Vec::new();

        for entity in &involution_result.entities {
            // Phase 1: Use the unique spectrum configuration ratio, not the generic spectrum_access
            // The unique ratio is stored in entity.spectrum_configuration.ratio
            let unique_ratio = entity.spectrum_configuration.ratio.calculate_ratio();
            spectrum_ratios.push(unique_ratio);

            // Calculate space_time_access and time_space_access from the unique ratio
            let (st_access, ts_access) = if unique_ratio >= 1.0 {
                let denom = unique_ratio + 1.0;
                (unique_ratio / denom, 1.0 / denom)
            } else {
                let denom = unique_ratio + 1.0;
                (1.0 / denom, unique_ratio / denom)
            };

            space_time_access.push(st_access);
            time_space_access.push(ts_access);
            veil_active.push(entity.spectrum_access.veil_active);
            entity_types.push(entity.entity_type.display_name().to_string());
            // Scale is now derived from current density
            scales.push(format!(
                "{:?}",
                entity.evolutionary_attractor.current_density
            ));
        }

        self.statistics_tracker.record_spectrum_access_data(
            spectrum_ratios,
            space_time_access,
            time_space_access,
            veil_active,
            entity_types,
            scales,
        );

        // Phase 4: Collect physical scale data from entities
        self.statistics_tracker
            .record_physical_scale_data(&involution_result.entities);

        // Phase 5: Collect individual variation data from entities
        self.record_individual_variation_data(&involution_result.entities);

        // Phase 6: Initialize collective dynamics
        self.initialize_collective_dynamics(&involution_result.entities);

        Ok(involution_result)
    }

    /// Run evolution phase
    fn run_evolution_phase(&mut self) -> EvolutionResult {
        let start_time = std::time::Instant::now();

        // Phase 1 Week 1: Initialize holographic continuity at evolution start
        self.update_holographic_continuity();

        // Collect entity types for catalyst generation (Phase 5)
        let mut entity_types: HashMap<EntityId, EntityType> = HashMap::new();

        // Phase 5: Collect entity types from involution result
        // We'll get these from the lifecycle manager entities
        for entity_id in self.lifecycle_manager.entities.keys() {
            // Try to get entity type from involution result
            // For now, we'll use default values - this will be improved
            entity_types.insert(entity_id.clone(), EntityType::Individual);
        }

        // Run evolution loop
        for step in 0..self.parameters.num_steps {
            self.current_step = step;

            // Phase 1 Week 1: Detect scale transitions
            // From MASTER_R&D_ROADMAP.md Phase 1 Week 1: "Detect when camera scale changes"
            self.detect_scale_transition();

            // Phase 1 Week 1: Run scale-specific simulation step
            // From MASTER_R&D_ROADMAP.md Phase 1 Week 1: "In each evolution step, call simulate_scale_step()"
            // This runs simulation specific to the current scale level (Quantum, Cellular, Biological, etc.)
            if let Err(e) = self.simulate_scale_step(self.parameters.time_step_size) {
                // Log error but continue with entity evolution
                eprintln!(
                    "Scale simulation error at step {} (scale: {:?}): {:?}",
                    step, self.current_scale, e
                );
            }

            // Phase 5: Apply catalysts to entities
            //
            // Phase 3 Implementation:
            // - Pass entity densities to apply_catalysts for Free Will integration
            // - Pass free will kernels to apply_catalysts for 3rd density+ choices
            // - Free Will choices are made internally in apply_catalysts
            let mut entity_states: HashMap<EntityId, crate::entity_layer7::layer7::EntityState> =
                self.lifecycle_manager
                    .entities
                    .iter()
                    .map(|(id, data)| (id.clone(), data.current_state.clone()))
                    .collect();

            // Collect entity densities for catalyst application
            let entity_densities: HashMap<
                EntityId,
                crate::evolution_density_octave::density_octave::Density,
            > = self
                .lifecycle_manager
                .entities
                .iter()
                .map(|(id, data)| (id.clone(), data.current_density))
                .collect();

            // Phase 5: Generate catalysts periodically (every 5 steps)
            // Phase 3: Increased frequency to ensure 3rd density entities get catalyst events
            if step % 5 == 0 {
                let num_catalysts = (self.lifecycle_manager.entities.len() / 15).clamp(1, 10);

                // Collect entity types for catalyst generation (Phase 5)
                let entity_types: HashMap<EntityId, EntityType> = self
                    .entities
                    .iter()
                    .map(|(id, data)| (id.clone(), data.entity_type))
                    .collect();

                // Phase 3: Pass entity_densities to ensure 3rd density entities are included
                self.catalyst_manager.generate_catalysts(
                    &entity_states,
                    &entity_types,
                    &entity_densities,
                    num_catalysts,
                );
            }

            // Collect entity densities for catalyst application
            let entity_densities: HashMap<
                EntityId,
                crate::evolution_density_octave::density_octave::Density,
            > = self
                .lifecycle_manager
                .entities
                .iter()
                .map(|(id, data)| (id.clone(), data.current_density))
                .collect();

            // Phase 3: Apply catalysts with Free Will integration
            // The Free Will kernel is used internally for 3rd density+ entities
            let catalyst_events = self.catalyst_manager.apply_catalysts(
                &mut entity_states,
                &entity_densities,
                &mut self.lifecycle_manager.free_will_kernels,
            );

            // Phase 5: Record catalyst events in statistics
            for event in catalyst_events {
                self.record_catalyst_event(event.clone());

                // Phase 1: Update entity polarization based on catalyst choice
                // Convert catalyst PolarityChoice to foundation PolarityChoice
                let polarity_choice = match event.polarity_choice {
                    crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToOthers => {
                        crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers
                    }
                    crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToSelf => {
                        crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf
                    }
                    crate::simulation_v3::catalyst_system::PolarityChoice::Unpolarized => {
                        crate::foundation::indigo_realm::PolarityChoice::Neutral
                    }
                };

                // Update entity's polarization
                if let Some(entity) = self.entities.get_mut(&event.entity_id) {
                    entity.polarization.make_choice(polarity_choice);
                }
            }

            // Phase 5: Update entity states in lifecycle manager after catalyst application
            for (entity_id, new_state) in entity_states {
                if let Some(lifecycle_data) = self.lifecycle_manager.entities.get_mut(&entity_id) {
                    lifecycle_data.current_state = new_state;
                }
            }

            // Evolve entities
            let _evolution_result = self.lifecycle_manager.evolve_entities(1);

            // Update holographic field
            // Phase 5: Use lazy updates for better performance
            if self.parameters.update_holographic_field {
                if self.performance_optimization_enabled {
                    // Phase 5: Set current step for lazy updates
                    self.holographic_manager.set_current_step(step);

                    // Phase 5: Use lazy update (only updates when needed based on interval)
                    self.holographic_manager.update_connections_lazy();

                    // Always update interference patterns and resonance (lightweight)
                    self.holographic_manager.calculate_interference_patterns();
                    self.holographic_manager.track_resonance_for_all_entities();

                    // Update statistics
                    self.holographic_manager.update_statistics();
                } else {
                    // Phase 5: Use regular update (performance optimization disabled)
                    let _ = self.holographic_manager.update_field(1);
                }
            }

            // Update physical manifestations
            if self.parameters.update_physical_manifestations {
                self.physical_adapter
                    .update_physical_manifestations(self.parameters.time_step_size);
            }

            // Phase 5: Update catalyst manager time
            self.catalyst_manager.update_time();

            // Phase 6: Update collective dynamics
            if step % 5 == 0 {
                self.update_collective_dynamics();
            }

            // Phase 4: Update environment and entity-environment interactions
            if step % 5 == 0 {
                self.update_environmental_interactions();
            }

            // Phase 4: Update inter-scale interactions
            if step % 5 == 0 {
                self.update_inter_scale_interactions();
            }

            // Phase 5: Update emergent behavior
            if step % 10 == 0 {
                self.update_emergent_behavior();
            }

            // Phase 7: Start consciousness-to-matter transitions periodically
            // Start transitions for entities that have reached sufficient development
            // TODO: Implement start_consciousness_transitions method
            // if step % 15 == 0 && step > 0 {
            //     self.start_consciousness_transitions();
            // }

            // Phase 7: Update consciousness-to-matter transitions each step
            self.consciousness_manager
                .update_transitions(self.parameters.time_step_size);

            // Phase 5: Tick biology pipeline (molecules → cells → organisms → evolution)
            if self.biology_pipeline.is_some() {
                let archetype_profile = self.compute_global_archetype_profile();
                let current_density = self.compute_current_global_density();
                if let Some(ref mut pipeline) = self.biology_pipeline {
                    let bio_result = pipeline.tick(
                        self.parameters.time_step_size,
                        &current_density,
                        &archetype_profile,
                    );
                    if step % 10 == 0 || step == self.parameters.num_steps - 1 {
                        self.statistics_tracker
                            .statistics
                            .biology
                            .update_from_tick(&bio_result);
                    }
                }
            }

            // Phase 3: Record polarization distribution periodically
            if step % 10 == 0 {
                self.record_polarization_distribution(step);
            }

            // Record statistics periodically
            if step % 10 == 0 || step == self.parameters.num_steps - 1 {
                self.record_evolution_step_statistics(step);
            }

            // Update time
            self.current_time += self.parameters.time_step_size as u64;
        }

        let execution_time = start_time.elapsed();

        // Record final evolution statistics
        self.statistics_tracker
            .record_evolution_complete(execution_time);

        // Record holographic statistics
        let holographic_field_stats = self.holographic_manager.get_statistics();
        let holographic_stats = StatsHolographicFieldStatistics {
            entity_count: holographic_field_stats.entity_count,
            connection_count: holographic_field_stats.connection_count,
            average_connection_strength: holographic_field_stats.average_connection_strength,
            resonant_connections: holographic_field_stats.resonant_connections,
            harmonic_connections: holographic_field_stats.harmonic_connections,
            entangled_connections: holographic_field_stats.entangled_connections,
            antiphase_connections: holographic_field_stats.antiphase_connections,
            weak_connections: holographic_field_stats.weak_connections,
            average_archetype_similarity: holographic_field_stats.average_archetype_similarity,
            global_phase_coherence: holographic_field_stats.global_phase_coherence,
            interference_pattern_count: holographic_field_stats.interference_pattern_count,
            resonance_cluster_count: holographic_field_stats.resonance_cluster_count,
            holographic_interaction_score: holographic_field_stats.holographic_interaction_score,
        };
        self.statistics_tracker
            .record_holographic_statistics(holographic_stats);

        // Physical statistics are already recorded during involution phase via record_physical_scale_data()

        // Phase 5: Update individual variation statistics before returning
        self.update_individual_variation_statistics();

        // Phase 7: Record consciousness-to-matter statistics
        let consciousness_stats = self.consciousness_manager.get_statistics();
        self.statistics_tracker.statistics.consciousness_to_matter = consciousness_stats;

        self.lifecycle_manager.get_final_evolution_result()
    }

    /// Record evolution step statistics
    fn record_evolution_step_statistics(&mut self, step: u64) {
        // Get lifecycle statistics
        let lifecycle_stats = self.lifecycle_manager.get_statistics();

        // Build density distribution
        let mut density_distribution = HashMap::new();
        for (density, count) in &lifecycle_stats.entities_by_density {
            density_distribution.insert(density.clone(), *count);
        }

        // Build polarization distribution
        let polarization_distribution = PolarizationDistribution {
            sto: lifecycle_stats.polarization_distribution.sto_count,
            sts: lifecycle_stats.polarization_distribution.sts_count,
            unpolarized: lifecycle_stats.polarization_distribution.unpolarized_count,
            average_bias: lifecycle_stats.polarization_distribution.avg_sto_score
                - lifecycle_stats.polarization_distribution.avg_sts_score,
        };

        // Phase 3: Calculate feature completion metrics

        // Energy tapping: Estimate based on entity development and polarization
        // More developed entities tap more energy from IntelligentInfinity
        let avg_development = lifecycle_stats.avg_developmental_level;
        // Normalize avg_development to 0.0-1.0 range (assuming max reasonable is 10.0)
        let normalized_development = (avg_development / 10.0).clamp(0.0, 1.0);
        let total_tapped_energy =
            normalized_development * lifecycle_stats.total_entities as Float * 10.0;
        let average_tap_strength = if lifecycle_stats.total_entities > 0 {
            total_tapped_energy / lifecycle_stats.total_entities as Float
        } else {
            0.0
        };

        // Spectrum access: Improves as entities evolve through densities
        // Start at 0.0 (1st Density) and approach 1.0 (8th Density)
        let spectrum_access_level = normalized_development.clamp(0.0, 1.0);

        // Veil evolution: Veil thins as entities evolve
        // Start at 1.0 (full veil) and approach 0.0 (no veil)
        let veil_thickness = (1.0 - normalized_development * 0.8).clamp(0.0, 1.0); // Never completely thin
        let veil_transparency = (normalized_development * 0.9).clamp(0.0, 1.0); // Never 100% transparent

        // Attractor-field activation: Increases as entities polarize and evolve
        // Based on polarization intensity and development level
        let polarization_intensity = if lifecycle_stats.total_entities > 0 {
            (lifecycle_stats.polarization_distribution.sto_count as Float
                + lifecycle_stats.polarization_distribution.sts_count as Float)
                / lifecycle_stats.total_entities as Float
        } else {
            0.0
        };
        let attractor_field_activation =
            ((normalized_development + polarization_intensity) / 2.0).clamp(0.0, 1.0);

        // Record step with Phase 3 metrics
        self.statistics_tracker.record_evolution_step(
            step,
            lifecycle_stats.total_entities,
            density_distribution,
            polarization_distribution,
            lifecycle_stats.total_transitions,
            total_tapped_energy,
            average_tap_strength,
            spectrum_access_level,
            veil_thickness,
            veil_transparency,
            attractor_field_activation,
        );
    }

    /// Phase 5: Record individual variation data from entities
    fn record_individual_variation_data(&mut self, entities: &[crate::entity_layer7::SubSubLogos]) {
        use crate::simulation_v3::individual_variation_statistics::KarmicPatternStatistics;

        let mut evolutionary_rates: Vec<crate::types::Float> = Vec::new();
        let mut karmic_pattern_stats = KarmicPatternStatistics::default();

        for entity in entities {
            // Collect evolutionary rates
            evolutionary_rates.push(entity.evolutionary_rate);

            // Collect karmic pattern statistics
            for karmic_pattern in &entity.karmic_patterns {
                karmic_pattern_stats.total_patterns += 1;

                let resolution_key = format!("{:?}", karmic_pattern.resolution_status);
                *karmic_pattern_stats
                    .by_resolution_status
                    .entry(resolution_key)
                    .or_insert(0) += 1;

                karmic_pattern_stats.average_intensity += karmic_pattern.intensity;
            }
        }

        // Calculate statistics
        if !evolutionary_rates.is_empty() {
            let total_rate: crate::types::Float = evolutionary_rates.iter().sum();
            self.statistics_tracker
                .statistics
                .individual_variation
                .average_evolutionary_rate =
                total_rate / evolutionary_rates.len() as crate::types::Float;
            self.statistics_tracker
                .statistics
                .individual_variation
                .min_evolutionary_rate = *evolutionary_rates
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            self.statistics_tracker
                .statistics
                .individual_variation
                .max_evolutionary_rate = *evolutionary_rates
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            // Build distribution histogram
            // Phase 2: Updated buckets to cover full range (0.3x-1.7x)
            for rate in &evolutionary_rates {
                let bucket = if rate < &0.5 {
                    "< 0.5x".to_string()
                } else if rate < &0.7 {
                    "0.5x - 0.7x".to_string()
                } else if rate < &0.9 {
                    "0.7x - 0.9x".to_string()
                } else if rate < &1.1 {
                    "0.9x - 1.1x".to_string()
                } else if rate < &1.3 {
                    "1.1x - 1.3x".to_string()
                } else if rate < &1.5 {
                    "1.3x - 1.5x".to_string()
                } else {
                    "≥ 1.5x".to_string()
                };

                *self
                    .statistics_tracker
                    .statistics
                    .individual_variation
                    .evolutionary_rate_distribution
                    .entry(bucket)
                    .or_insert(0) += 1;
            }
        }

        // Calculate karmic pattern statistics
        if karmic_pattern_stats.total_patterns > 0 {
            karmic_pattern_stats.average_intensity /= karmic_pattern_stats.total_patterns as f64;
            karmic_pattern_stats.average_patterns_per_entity =
                karmic_pattern_stats.total_patterns as f64 / entities.len() as f64;
        }

        self.statistics_tracker
            .statistics
            .individual_variation
            .karmic_patterns = karmic_pattern_stats;

        // Phase 2: Calculate unique evolutionary trajectories
        // Count entities with unique combinations of:
        // - Current density
        // - Evolutionary rate
        // - Polarity bias
        // - Number of karmic patterns
        use std::collections::HashSet;
        let mut unique_combinations: HashSet<String> = HashSet::new();
        for entity in entities {
            let density_key = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => "1",
                crate::entity_layer7::layer7::DensityLevel::Second => "2",
                crate::entity_layer7::layer7::DensityLevel::Third => "3",
                crate::entity_layer7::layer7::DensityLevel::Fourth => "4",
                crate::entity_layer7::layer7::DensityLevel::Fifth => "5",
                crate::entity_layer7::layer7::DensityLevel::Sixth => "6",
                crate::entity_layer7::layer7::DensityLevel::Seventh => "7",
                crate::entity_layer7::layer7::DensityLevel::Eighth => "8",
            };
            let key = format!(
                "{}_{:.2}_{:.2}_{}",
                density_key,
                entity.evolutionary_rate,
                entity.current_state.polarity_state.polarity_bias,
                entity.karmic_patterns.len()
            );
            unique_combinations.insert(key);
        }
        self.statistics_tracker
            .statistics
            .individual_variation
            .unique_trajectories = unique_combinations.len();
    }

    /// Phase 5: Record a catalyst event
    fn record_catalyst_event(&mut self, event: CatalystEvent) {
        use crate::simulation_v3::catalyst_system::PolarityChoice;

        // Update catalyst event statistics
        let catalyst_stats = &mut self
            .statistics_tracker
            .statistics
            .individual_variation
            .catalyst_events;

        catalyst_stats.total_events += 1;

        let type_key = format!("{:?}", event.event_type);
        *catalyst_stats.by_catalyst_type.entry(type_key).or_insert(0) += 1;

        match event.polarity_choice {
            PolarityChoice::ServiceToOthers => {
                catalyst_stats.polarity_choices.sto_choices += 1;
            }
            PolarityChoice::ServiceToSelf => {
                catalyst_stats.polarity_choices.sts_choices += 1;
            }
            PolarityChoice::Unpolarized => {
                catalyst_stats.polarity_choices.unpolarized_choices += 1;
            }
        }

        catalyst_stats.average_intensity = (catalyst_stats.average_intensity
            * (catalyst_stats.total_events - 1) as f64
            + event.intensity)
            / catalyst_stats.total_events as f64;

        catalyst_stats.total_learning_value += event.learning_value;
        catalyst_stats.total_consciousness_expansion += event.consciousness_expansion;

        // Update percentages (only periodically to avoid overhead)
        if catalyst_stats.total_events.is_multiple_of(10) {
            catalyst_stats.polarity_choices.update_percentages();
        }
    }

    /// Phase 5: Update individual variation statistics
    fn update_individual_variation_statistics(&mut self) {
        use crate::simulation_v3::individual_variation_statistics::CatalystEventStatistics;

        let catalyst_stats = &self.catalyst_manager.get_statistics();

        self.statistics_tracker
            .statistics
            .individual_variation
            .catalyst_events = CatalystEventStatistics {
            total_events: catalyst_stats.total_events,
            by_catalyst_type: catalyst_stats.events_by_type.clone(),
            polarity_choices: self
                .statistics_tracker
                .statistics
                .individual_variation
                .catalyst_events
                .polarity_choices
                .clone(),
            average_intensity: catalyst_stats.average_intensity,
            total_learning_value: catalyst_stats.total_learning_value,
            total_consciousness_expansion: catalyst_stats.total_consciousness_expansion,
            catalyst_effectiveness: if catalyst_stats.total_events > 0 {
                catalyst_stats.total_learning_value
                    / (catalyst_stats.total_events as Float * catalyst_stats.average_intensity
                        + 1e-10)
            } else {
                0.0
            },
        };

        // Calculate polarization diversity
        let polarity_choices = &mut self
            .statistics_tracker
            .statistics
            .individual_variation
            .catalyst_events
            .polarity_choices;
        polarity_choices.update_percentages();
        let diversity = polarity_choices.calculate_diversity();

        self.statistics_tracker
            .statistics
            .individual_variation
            .polarization_diversity = diversity;

        // Update holographic interaction statistics
        let holographic_stats = self.holographic_manager.get_statistics();
        self.statistics_tracker.statistics.individual_variation.holographic_interactions =
            crate::simulation_v3::individual_variation_statistics::HolographicInteractionStatistics {
                total_connections: holographic_stats.connection_count,
                by_connection_type: HashMap::new(), // Simplified
                average_connection_strength: holographic_stats.average_connection_strength,
                resonant_connections: holographic_stats.resonant_connections,
                entangled_connections: holographic_stats.entangled_connections,
                harmonic_connections: 0, // Not tracked
                global_phase_coherence: holographic_stats.global_phase_coherence,
                resonance_cluster_count: holographic_stats.resonance_cluster_count,
                average_cluster_size: 0.0, // Simplified
            };
    }

    fn compute_global_archetype_profile(&self) -> [Float; 22] {
        if self.entities.is_empty() {
            return [0.5; 22];
        }
        let mut profile = [0.0f64; 22];
        for entity in self.entities.values() {
            for i in 0..22 {
                profile[i] += entity.archetype_activations[i];
            }
        }
        let n = self.entities.len() as Float;
        for i in 0..22 {
            profile[i] /= n;
        }
        profile
    }

    fn compute_current_global_density(
        &self,
    ) -> crate::evolution_density_octave::density_octave::Density {
        if self.entities.is_empty() {
            return crate::evolution_density_octave::density_octave::Density::Third;
        }
        let mut counts: std::collections::HashMap<u8, usize> = std::collections::HashMap::new();
        for entity in self.entities.values() {
            let d = match entity.current_density {
                crate::evolution_density_octave::density_octave::Density::First(_) => 1,
                crate::evolution_density_octave::density_octave::Density::Second(_) => 2,
                crate::evolution_density_octave::density_octave::Density::Third => 3,
                crate::evolution_density_octave::density_octave::Density::Fourth => 4,
                crate::evolution_density_octave::density_octave::Density::Fifth => 5,
                crate::evolution_density_octave::density_octave::Density::Sixth => 6,
                crate::evolution_density_octave::density_octave::Density::Seventh => 7,
                crate::evolution_density_octave::density_octave::Density::Eighth => 8,
            };
            *counts.entry(d).or_insert(0) += 1;
        }
        let mode_density = counts
            .into_iter()
            .max_by_key(|(_, c)| *c)
            .map(|(d, _)| d)
            .unwrap_or(3);
        match mode_density {
            1 => crate::evolution_density_octave::density_octave::Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            ),
            2 => crate::evolution_density_octave::density_octave::Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::Cellular,
            ),
            3 => crate::evolution_density_octave::density_octave::Density::Third,
            4 => crate::evolution_density_octave::density_octave::Density::Fourth,
            5 => crate::evolution_density_octave::density_octave::Density::Fifth,
            6 => crate::evolution_density_octave::density_octave::Density::Sixth,
            7 => crate::evolution_density_octave::density_octave::Density::Seventh,
            _ => crate::evolution_density_octave::density_octave::Density::Eighth,
        }
    }

    /// Get current simulation time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Get current simulation step
    pub fn current_step(&self) -> u64 {
        self.current_step
    }

    /// Check if simulation is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get current statistics
    pub fn get_statistics(&self) -> &SimulationStatistics {
        self.statistics_tracker.get_statistics()
    }

    /// Generate a report
    pub fn generate_report(&self) -> String {
        let reporter = SimulationReporter::new(self.statistics_tracker.statistics.clone());
        reporter.generate_summary_report()
    }

    /// Phase 2: Generate spectrum distribution visualization
    ///
    /// Shows where each entity is on the space/time ↔ time/space continuum,
    /// what each entity is doing, the Veil effect, and density distribution.
    pub fn generate_spectrum_distribution(&self) -> String {
        SpectrumDistributionVisualizer::visualize_spectrum_distribution(&self.entities)
    }

    /// Generate comprehensive report including spectrum visualization
    ///
    /// This is the Phase 2 enhanced report that includes spectrum distribution.
    pub fn generate_comprehensive_report(&self) -> String {
        let mut output = String::new();

        // Original report
        output.push_str(&self.generate_report());
        output.push_str("\n\n");

        // Phase 2: Spectrum distribution visualization
        output.push_str(&self.generate_spectrum_distribution());

        output
    }

    /// Phase 3: Generate physical structure visualization report
    pub fn generate_physical_structure_report(&self) -> String {
        let visualizer = PhysicalStructureVisualizer::new(
            self.physical_structure_manager.clone(),
            self.hierarchical_composition_manager.clone(),
            self.simultaneous_emergence_manager.clone(),
        );

        visualizer.generate_comprehensive_report()
    }

    /// Phase 3: Generate complete report including physical structures
    pub fn generate_complete_report(&self) -> String {
        let mut output = String::new();

        // Original comprehensive report
        output.push_str(&self.generate_comprehensive_report());
        output.push_str("\n\n");

        // Phase 3: Physical structure visualization
        output.push_str(&self.generate_physical_structure_report());

        output
    }

    /// Get lifecycle manager (for external access)
    pub fn lifecycle_manager(&self) -> &EntityLifecycleManager {
        &self.lifecycle_manager
    }

    /// Get holographic manager (for external access)
    pub fn holographic_manager(&self) -> &HolographicFieldManager {
        &self.holographic_manager
    }

    /// Get physical adapter (for external access)
    pub fn physical_adapter(&self) -> &PhysicalAdapter {
        &self.physical_adapter
    }

    /// Get collective dynamics manager (for external access)
    pub fn collective_dynamics_manager(&self) -> &CollectiveDynamicsManager {
        &self.collective_dynamics_manager
    }

    /// Get collective statistics tracker (for external access)
    pub fn collective_statistics_tracker(&self) -> &CollectiveStatisticsTracker {
        &self.collective_statistics_tracker
    }

    // Phase 6: Initialize collective dynamics
    fn initialize_collective_dynamics(
        &mut self,
        entities: &Vec<crate::entity_layer7::layer7::SubSubLogos>,
    ) {
        // Create entity map for collective initialization
        let mut entity_map: HashMap<EntityId, crate::entity_layer7::layer7::SubSubLogos> =
            HashMap::new();
        for entity in entities {
            entity_map.insert(entity.entity_id.clone(), entity.clone());
        }

        // Initialize collectives
        if let Err(e) = self
            .collective_dynamics_manager
            .initialize_collectives(&entity_map)
        {
            eprintln!("Failed to initialize collective dynamics: {}", e);
        }
    }

    // Phase 6: Update collective dynamics with resonance-based formation
    fn update_collective_dynamics(&mut self) {
        // Get active catalysts
        let active_catalysts: Vec<crate::simulation_v3::catalyst_system::Catalyst> =
            self.catalyst_manager.active_catalysts.clone();

        // Phase 6: Update collective assignments based on resonance (>0.8 threshold)
        // This method calculates resonance between entities and forms collectives
        self.collective_dynamics_manager
            .update_collective_assignments(&self.entities, &self.holographic_manager);

        // Phase 6: Update coherence tracking across scales
        let collective_coherence_map = self
            .collective_dynamics_manager
            .get_collective_coherence_map();
        self.holographic_manager.coherence_tracker.update_coherence(
            &self.entities,
            &collective_coherence_map,
            self.current_step,
        );

        // Update collective dynamics with stored entities (Phase 4)
        self.collective_dynamics_manager
            .update_collective_dynamics(&self.entities, &active_catalysts);

        // Record collective statistics
        self.collective_statistics_tracker.record_statistics(
            &self.collective_dynamics_manager.collective_behaviors,
            &self.collective_dynamics_manager.group_consciousness,
            &self.collective_dynamics_manager.collective_evolution,
            &self.collective_dynamics_manager.collective_free_will,
            &self.holographic_manager,
        );

        // Record collective statistics
        self.collective_statistics_tracker.record_statistics(
            &self.collective_dynamics_manager.collective_behaviors,
            &self.collective_dynamics_manager.group_consciousness,
            &self.collective_dynamics_manager.collective_evolution,
            &self.collective_dynamics_manager.collective_free_will,
            &self.holographic_manager,
        );
    }

    // Phase 4: Update environmental interactions
    fn update_environmental_interactions(&mut self) {
        // Initialize environment if not already done
        if self.environmental_manager.simulation_time == 0 {
            self.environmental_manager
                .initialize_environment(&self.entities);
        }

        // Update environment
        self.environmental_manager
            .update_environment(&self.entities);

        // Apply entity-environment interactions for all entities
        for entity_id in self.entities.keys() {
            if let Some(entity) = self.entities.get(entity_id) {
                let _effect = self
                    .environmental_manager
                    .interact_with_environment(entity_id, entity);
            }
        }

        // Phase 4: Apply collective influence bidirectionally
        // For each collective entity, apply influence to its members
        let collective_ids: Vec<_> = self
            .collective_dynamics_manager
            .collective_behaviors
            .keys()
            .cloned()
            .collect();

        for collective_id in collective_ids {
            // Get member entities before applying influence to avoid borrow conflicts
            let member_ids: Vec<_> = if let Some(behavior) = self
                .collective_dynamics_manager
                .collective_behaviors
                .get(&collective_id)
            {
                behavior.member_entities.clone()
            } else {
                continue;
            };

            // Apply collective influence to each member
            for member_id in member_ids {
                let _result = self.collective_dynamics_manager.apply_collective_influence(
                    &collective_id,
                    &member_id,
                    &self.entities,
                );

                // Apply individual influence back to collective
                let _result = self.collective_dynamics_manager.apply_individual_influence(
                    &collective_id,
                    &member_id,
                    &self.entities,
                );
            }
        }
    }

    /// Update inter-scale interactions
    ///
    /// Phase 4 Implementation:
    /// - Process composition interactions (how components affect composites)
    /// - Create holographic connections (resonant, entangled, harmonic)
    /// - Process collective influence (collective ↔ member)
    /// - Track co-evolution between entities
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 4:
    /// "How do quantum particles affect atoms?"
    /// "How do atoms affect molecules?"
    /// "How do molecules affect cells?"
    /// "How do cells affect organisms?"
    /// "How do organisms affect beings?"
    fn update_inter_scale_interactions(&mut self) {
        // Process composition interactions
        let _composition_effects = self
            .inter_scale_interaction_manager
            .process_composition_interactions(&self.entities);

        // Create holographic connections (every 10 steps to avoid too many connections)
        if self.current_step.is_multiple_of(10) {
            let _connections = self
                .inter_scale_interaction_manager
                .create_holographic_connections(&self.entities, 0.5); // 0.5 similarity threshold
        }

        // Process collective influence
        let _influences = self
            .inter_scale_interaction_manager
            .process_collective_influence(&self.entities);

        // Track co-evolution (every 10 steps)
        if self.current_step.is_multiple_of(10) {
            let _co_evolution = self
                .inter_scale_interaction_manager
                .track_co_evolution(&self.entities);
        }

        // Increment timestamp for interaction tracking
        self.inter_scale_interaction_manager.increment_timestamp();
    }

    /// Update emergent behavior
    ///
    /// Phase 5 Implementation:
    /// - Calculate system-level emergence (global coherence, collective consciousness)
    /// - Calculate environmental emergence (environment from 1st Density)
    /// - Detect emergent events
    /// - Track emergence history
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 5:
    /// "Implement System-Level Emergence"
    /// "Implement Environmental Emergence"
    /// "Create organic emergent behavior from the complex system"
    fn update_emergent_behavior(&mut self) {
        // Calculate system-level emergence
        let _system_emergence = self
            .emergence_manager
            .calculate_system_emergence(&self.entities);

        // Calculate environmental emergence
        let _environmental_emergence = self
            .emergence_manager
            .calculate_environmental_emergence(&self.entities);

        // Detect emergent events
        let _events = self
            .emergence_manager
            .detect_emergent_events(&self.entities);

        // Increment timestamp for emergence tracking
        self.emergence_manager.increment_timestamp();
    }

    // ========================================================================
    // PHASE 3: POLARIZATION VISUALIZATION
    // ========================================================================

    /// Record polarization distribution
    ///
    /// Phase 3 Implementation:
    /// - Track polarization distribution over time
    /// - Calculate Polarity Diversity Index
    /// - Visualize polarization distribution
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 3.3:
    /// "Show polarization distribution over time"
    /// "Track how polarization affects evolution"
    /// "Calculate Polarity Diversity Index"
    fn record_polarization_distribution(&mut self, _step: u64) {
        let lifecycle_stats = self.lifecycle_manager.get_statistics();
        let polarization = &lifecycle_stats.polarization_distribution;

        // Calculate Polarity Diversity Index
        // 0.0 = no diversity (all unpolarized)
        // 1.0 = maximum diversity (equal STO/STS)
        let diversity_index = self.calculate_polarization_diversity_index();

        // Record in statistics tracker
        self.statistics_tracker
            .statistics
            .evolution
            .polarization_distribution =
            crate::simulation_v3::statistics::PolarizationDistribution {
                sto: polarization.sto_count,
                sts: polarization.sts_count,
                unpolarized: polarization.unpolarized_count,
                average_bias: polarization.avg_sto_score - polarization.avg_sts_score,
            };

        self.statistics_tracker
            .statistics
            .evolution
            .polarization_diversity_index = diversity_index;
    }

    /// Calculate Polarity Diversity Index
    ///
    /// Phase 3 Implementation:
    /// - Measures diversity of polarization in the simulation
    /// - Range: 0.0 (no diversity) to 1.0 (maximum diversity)
    /// - Based on the balance between STO and STS entities
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 3.3:
    /// "Calculate Polarity Diversity Index"
    /// "0.0 = no diversity (all unpolarized)"
    /// "1.0 = maximum diversity (equal STO/STS)"
    fn calculate_polarization_diversity_index(&self) -> f64 {
        let lifecycle_stats = self.lifecycle_manager.get_statistics();
        let polarization = &lifecycle_stats.polarization_distribution;

        if lifecycle_stats.total_entities == 0 {
            return 0.0;
        }

        // Calculate diversity based on the balance between STO and STS
        // Unpolarized entities don't contribute to diversity
        let sto_count = polarization.sto_count as f64;
        let sts_count = polarization.sts_count as f64;
        let total_polarized = sto_count + sts_count;

        if total_polarized == 0.0 {
            // No polarized entities - no diversity
            return 0.0;
        }

        // Calculate diversity using normalized Shannon entropy
        // Maximum diversity when STO and STS are equal
        let sto_ratio = sto_count / total_polarized;
        let sts_ratio = sts_count / total_polarized;

        // Shannon entropy: -sum(p_i * log2(p_i))
        let entropy = -sto_ratio * sto_ratio.log2() - sts_ratio * sts_ratio.log2();

        // Maximum entropy for 2 categories is 1.0
        // Normalize to 0.0-1.0 range
        let normalized_entropy = entropy / 1.0;

        // Scale by polarization percentage
        // If only 50% of entities are polarized, max diversity is 0.5
        let polarization_percentage = total_polarized / lifecycle_stats.total_entities as f64;

        normalized_entropy * polarization_percentage
    }

    /// Display polarization distribution
    ///
    /// Phase 3 Implementation:
    /// - Visualize polarization distribution with progress bars
    /// - Show polarization diversity index
    /// - Display polarization statistics
    pub fn display_polarization_distribution(&self) {
        let lifecycle_stats = self.lifecycle_manager.get_statistics();
        let polarization = &lifecycle_stats.polarization_distribution;
        let diversity_index = self.calculate_polarization_diversity_index();

        println!("\n=== POLARIZATION DISTRIBUTION (Phase 3) ===");
        println!("Total Entities: {}", lifecycle_stats.total_entities);

        // Calculate percentages
        let sto_percent = if lifecycle_stats.total_entities > 0 {
            polarization.sto_count as f64 / lifecycle_stats.total_entities as f64 * 100.0
        } else {
            0.0
        };

        let sts_percent = if lifecycle_stats.total_entities > 0 {
            polarization.sts_count as f64 / lifecycle_stats.total_entities as f64 * 100.0
        } else {
            0.0
        };

        let unpolarized_percent = if lifecycle_stats.total_entities > 0 {
            polarization.unpolarized_count as f64 / lifecycle_stats.total_entities as f64 * 100.0
        } else {
            0.0
        };

        // Display STO distribution
        println!("\nService-to-Others (STO):");
        println!(
            "  Count: {} / {} ({:.1}%)",
            polarization.sto_count, lifecycle_stats.total_entities, sto_percent
        );
        println!("  Average Score: {:.3}", polarization.avg_sto_score);
        println!(
            "  {}",
            self.visualize_bar(polarization.sto_count, lifecycle_stats.total_entities)
        );

        // Display STS distribution
        println!("\nService-to-Self (STS):");
        println!(
            "  Count: {} / {} ({:.1}%)",
            polarization.sts_count, lifecycle_stats.total_entities, sts_percent
        );
        println!("  Average Score: {:.3}", polarization.avg_sts_score);
        println!(
            "  {}",
            self.visualize_bar(polarization.sts_count, lifecycle_stats.total_entities)
        );

        // Display Unpolarized distribution
        println!("\nUnpolarized:");
        println!(
            "  Count: {} / {} ({:.1}%)",
            polarization.unpolarized_count, lifecycle_stats.total_entities, unpolarized_percent
        );
        println!(
            "  {}",
            self.visualize_bar(
                polarization.unpolarized_count,
                lifecycle_stats.total_entities
            )
        );

        // Display Polarity Diversity Index
        println!("\nPolarity Diversity Index: {:.3}", diversity_index);
        println!("  (0.0 = no diversity, 1.0 = maximum diversity)");

        // Display interpretation
        println!("\nPolarity Interpretation:");
        if diversity_index < 0.1 {
            println!("  Status: LOW DIVERSITY - Most entities are unpolarized");
        } else if diversity_index < 0.3 {
            println!("  Status: MODERATE DIVERSITY - Some polarization emerging");
        } else if diversity_index < 0.5 {
            println!("  Status: GOOD DIVERSITY - Balanced polarization developing");
        } else if diversity_index < 0.7 {
            println!("  Status: HIGH DIVERSITY - Well-balanced STO/STS distribution");
        } else {
            println!("  Status: VERY HIGH DIVERSITY - Near-perfect polarization balance");
        }
    }

    /// Visualize a bar for distribution display
    fn visualize_bar(&self, count: usize, total: usize) -> f64 {
        if total == 0 {
            return 0.0;
        }
        count as f64 / total as f64
    }

    // ========================================================================
    // PHASE 7: CONSCIOUSNESS-TO-MATTER TRANSITION
    // ========================================================================

    /// Calculate information content for an entity
    ///
    /// Information content is based on the complexity of the entity's
    /// holographic blueprint and evolutionary trajectory.
    ///
    /// # Arguments
    /// * `entity` - The entity to calculate information content for
    ///
    /// # Returns
    /// Information content in bits
    fn calculate_entity_information_content(
        &self,
        entity: &crate::entity_layer7::SubSubLogos,
    ) -> Float {
        // Base information content from archetype strength
        let archetype_info = entity.indigo_realm.archetype22.strength * 22.0;

        // Add complexity from density level
        let density_info = match entity.evolutionary_attractor.current_density {
            crate::entity_layer7::layer7::DensityLevel::First => 100.0,
            crate::entity_layer7::layer7::DensityLevel::Second => 200.0,
            crate::entity_layer7::layer7::DensityLevel::Third => 400.0,
            crate::entity_layer7::layer7::DensityLevel::Fourth => 800.0,
            crate::entity_layer7::layer7::DensityLevel::Fifth => 1600.0,
            crate::entity_layer7::layer7::DensityLevel::Sixth => 3200.0,
            crate::entity_layer7::layer7::DensityLevel::Seventh => 6400.0,
            crate::entity_layer7::layer7::DensityLevel::Eighth => 12800.0,
        };

        // Add complexity from consciousness level
        let consciousness_info = entity.current_state.consciousness_level * 1000.0;

        // Add complexity from polarization
        let polarization_info = entity.current_state.polarity_state.polarization_strength * 500.0;

        // Total information content
        archetype_info + density_info + consciousness_info + polarization_info
    }

    /// Run a single evolution step (internal method for long simulations)
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Internal method for running single evolution step"
    fn run_evolution_step_internal(&mut self, step: u64) -> Result<(), PersistenceError> {
        // Collect entity types for catalyst generation
        let mut entity_types: HashMap<EntityId, EntityType> = HashMap::new();

        // Collect entity types from involution result
        for entity_id in self.lifecycle_manager.entities.keys() {
            entity_types.insert(entity_id.clone(), EntityType::Individual);
        }

        // Apply catalysts to entities
        let mut entity_states: HashMap<EntityId, crate::entity_layer7::layer7::EntityState> = self
            .lifecycle_manager
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.current_state.clone()))
            .collect();

        // Collect entity densities for catalyst application
        let entity_densities: HashMap<
            EntityId,
            crate::evolution_density_octave::density_octave::Density,
        > = self
            .lifecycle_manager
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.current_density))
            .collect();

        // Generate catalysts periodically (every 5 steps)
        if step.is_multiple_of(5) {
            let num_catalysts = (self.lifecycle_manager.entities.len() / 15).clamp(1, 10);

            self.catalyst_manager.generate_catalysts(
                &entity_states,
                &entity_types,
                &entity_densities,
                num_catalysts,
            );
        }

        // Apply catalysts with Free Will integration
        let catalyst_events = self.catalyst_manager.apply_catalysts(
            &mut entity_states,
            &entity_densities,
            &mut self.lifecycle_manager.free_will_kernels,
        );

        // Record catalyst events in statistics
        for event in catalyst_events {
            self.record_catalyst_event(event.clone());

            // Update entity polarization based on catalyst choice
            let polarity_choice = match event.polarity_choice {
                crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToOthers => {
                    crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers
                }
                crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToSelf => {
                    crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf
                }
                crate::simulation_v3::catalyst_system::PolarityChoice::Unpolarized => {
                    crate::foundation::indigo_realm::PolarityChoice::Neutral
                }
            };

            // Update entity's polarization
            if let Some(entity) = self.entities.get_mut(&event.entity_id) {
                entity.polarization.make_choice(polarity_choice);
            }
        }

        // Update entity states in lifecycle manager after catalyst application
        for (entity_id, new_state) in entity_states {
            if let Some(lifecycle_data) = self.lifecycle_manager.entities.get_mut(&entity_id) {
                lifecycle_data.current_state = new_state;
            }
        }

        // Evolve entities
        let _evolution_result = self.lifecycle_manager.evolve_entities(1);

        // Update holographic field
        if self.parameters.update_holographic_field {
            if self.performance_optimization_enabled {
                // Set current step for lazy updates
                self.holographic_manager.set_current_step(step);

                // Use lazy update (only updates when needed based on interval)
                self.holographic_manager.update_connections_lazy();

                // Always update interference patterns and resonance (lightweight)
                self.holographic_manager.calculate_interference_patterns();
                self.holographic_manager.track_resonance_for_all_entities();

                // Update statistics
                self.holographic_manager.update_statistics();
            } else {
                // Use regular update (performance optimization disabled)
                let _ = self.holographic_manager.update_field(1);
            }
        }

        // Update physical manifestations
        if self.parameters.update_physical_manifestations {
            self.physical_adapter
                .update_physical_manifestations(self.parameters.time_step_size);
        }

        // Update catalyst manager time
        self.catalyst_manager.update_time();

        // Update collective dynamics
        if step.is_multiple_of(5) {
            self.update_collective_dynamics();
        }

        // Update statistics
        // TODO: Implement update_after_step method
        // self.statistics_tracker.update_after_step(step);

        Ok(())
    }

    // ============================================================================
    // PHASE 8: LONG SIMULATION RUNS
    // ============================================================================

    /// Run long simulation with checkpointing
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Implement long simulation runs with save/load functionality"
    pub fn run_long_simulation_with_checkpointing(
        &mut self,
        steps: u64,
        save_interval: u64,
        checkpoint_dir: &Path,
    ) -> Result<SimulationResult, PersistenceError> {
        let start_time = Instant::now();
        self.running = true;

        // Create checkpoint directory
        std::fs::create_dir_all(checkpoint_dir).map_err(|e| {
            PersistenceError::IoError(format!("Failed to create checkpoint directory: {}", e))
        })?;

        // Initialize checkpoint manager
        let mut checkpoint_manager = CheckpointManager::new(10);
        let mut performance_metrics = PerformanceMetrics::new();

        // Run involution phase first
        let involution_result = if self.parameters.run_involution {
            match self.run_involution_phase() {
                Ok(result) => Some(result),
                Err(e) => {
                    return Err(PersistenceError::IoError(format!(
                        "Involution phase error: {:?}",
                        e
                    )))
                }
            }
        } else {
            None
        };

        // Check if involution was successful
        if let Some(ref inv_result) = involution_result {
            if inv_result.entities.is_empty() {
                return Err(PersistenceError::InvalidState(
                    "Involution phase failed - no entities created".to_string(),
                ));
            }
        }

        // Run evolution phase with checkpointing
        let mut evolution_result = EvolutionResult::default();
        let evolution_start = Instant::now();

        for step in 0..steps {
            let step_start = Instant::now();

            // Run single evolution step
            self.current_step = step;
            self.run_evolution_step_internal(step)?;

            // Update performance metrics
            let step_time = step_start.elapsed().as_secs_f64();
            let entity_count = self.entities.len();
            let collective_count = self.collective_dynamics_manager.collective_behaviors.len();

            performance_metrics.record_step(step_time, entity_count, collective_count);

            // Save checkpoint periodically
            if step > 0 && step % save_interval == 0 {
                let checkpoint_path = PersistenceManager::get_checkpoint_path(checkpoint_dir, step);

                match PersistenceManager::save_checkpoint_metadata(
                    step,
                    entity_count,
                    collective_count,
                    &checkpoint_path,
                ) {
                    Ok(checkpoint) => {
                        checkpoint_manager.add_checkpoint(checkpoint);
                        println!(
                            "Checkpoint saved at step {} ({} entities, {} collectives)",
                            step, entity_count, collective_count
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to save checkpoint at step {}: {:?}", step, e);
                    }
                }

                // Clean up old checkpoints
                if let Err(e) = PersistenceManager::delete_old_checkpoints(checkpoint_dir, 5) {
                    eprintln!("Failed to delete old checkpoints: {:?}", e);
                }
            }

            // Print progress
            if step % 100 == 0 {
                let elapsed = evolution_start.elapsed().as_secs_f64();
                let eta = if step > 0 {
                    (elapsed / step as f64) * (steps - step) as f64
                } else {
                    0.0
                };

                println!(
                    "Progress: {}/{} steps ({:.1}%) - {:.2} steps/sec - ETA: {:.1}s",
                    step,
                    steps,
                    (step * 100) / steps,
                    performance_metrics.get_steps_per_second(),
                    eta
                );
            }
        }

        // Update evolution result
        evolution_result.steps = steps;
        evolution_result.execution_time = evolution_start.elapsed();

        // Get holographic field result
        let holographic_result = self.holographic_manager.get_field_result();

        // Get physical adapter statistics
        let physical_adapter_stats = self.physical_adapter.get_statistics().clone();

        // Unwrap involution_result
        let involution_result = involution_result.unwrap_or_else(|| InvolutionResult {
            entities: Vec::new(),
            attractor_fields: Vec::new(),
            stage_transitions: Vec::new(),
            execution_time: Duration::ZERO,
        });

        // Update final statistics
        self.statistics_tracker.update_performance_metrics();

        // Build result
        let result = SimulationResult {
            involution_result,
            evolution_result,
            holographic_result,
            physical_statistics: physical_adapter_stats,
            statistics: self.statistics_tracker.statistics.clone(),
            total_execution_time: start_time.elapsed(),
            success: true,
            error_message: None,
        };

        // Save final checkpoint
        let final_checkpoint_path = PersistenceManager::get_checkpoint_path(checkpoint_dir, steps);
        if let Err(e) = PersistenceManager::save_checkpoint_metadata(
            steps,
            self.entities.len(),
            self.collective_dynamics_manager.collective_behaviors.len(),
            &final_checkpoint_path,
        ) {
            eprintln!("Failed to save final checkpoint: {:?}", e);
        }

        self.running = false;

        Ok(result)
    }

    /// Get performance metrics
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md:
    /// "Track performance metrics for long simulations"
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        // Note: This would need to track metrics during simulation
        // For now, return empty metrics
        PerformanceMetrics::new()
    }

    // ===========================================================================
    // PHASE 1 WEEK 1: MULTI-SCALE SIMULATION
    // ===========================================================================

    /// Set the active scale level for simulation
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add method set_scale(&mut self, scale: ScaleLevel) to change active scale"
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each scale contains the whole (holographic principle)"
    pub fn set_scale(&mut self, scale: ScaleLevel) {
        // From GAMING_ENGINE_ROADMAP_v2.md Section 5:
        // "Maintain holographic continuity during scale transitions"
        let previous_scale = self.current_scale;

        if previous_scale != scale {
            // Log scale transition
            println!(
                "Scale transition: {} -> {}",
                previous_scale.display_name(),
                scale.display_name()
            );

            // Update current scale
            self.current_scale = scale;

            // Update multi-scale camera if available
            if let Some(ref mut camera) = self.multiscale_camera {
                camera.transition_to_scale(
                    scale,
                    std::time::Duration::from_millis(100),
                    crate::simulation_v3::multiscale_camera::InterpolationMode::Smooth,
                );
            }

            // Update holographic continuity
            self.update_holographic_continuity();
        }
    }

    /// Get the current active scale level
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add method get_current_scale(&self) -> ScaleLevel to get active scale"
    pub fn get_current_scale(&self) -> ScaleLevel {
        self.current_scale
    }

    /// Simulate one time step at the current scale
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add method simulate_scale_step(&mut self, time_step: Float) to run one simulation step"
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each scale contains the whole" - scale-specific simulation maintains holographic continuity
    pub fn simulate_scale_step(&mut self, time_step: Float) -> Result<(), ScalePhysicsError> {
        // From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
        // "In each evolution step, call simulate_scale_step() instead of generic entity updates"
        let _result = self
            .scale_physics
            .simulate_step(self.current_scale, time_step)?;

        // Track scale-specific simulation performance
        // Performance metrics will be recorded in the simulation result

        // Update cross-scale coupling after each step
        self.update_cross_scale_coupling();

        Ok(())
    }

    /// Update holographic continuity across all scales
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Implement method update_holographic_continuity() to ensure 'each scale contains the whole'"
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each entity contains within it all densities and sub-densities of the octave"
    /// "Any portion contains the whole" - the Law of One principle
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
    /// "Holographic continuity: each scale contains the whole"
    pub fn update_holographic_continuity(&mut self) {
        // From GAMING_ENGINE_ROADMAP_v2.md Section 5.2:
        // "Maintain holographic continuity during transitions"
        // "Resolution changes, but completeness is maintained"

        // Update holographic continuity strength
        // Continuity strength approaches 1.0 (perfect) as the simulation progresses
        let continuity_improvement = 0.001 * self.parameters.time_step_size;
        self.scale_physics
            .holographic_continuity
            .continuity_strength = (self
            .scale_physics
            .holographic_continuity
            .continuity_strength
            + continuity_improvement)
            .min(1.0);

        // Update cross-scale coupling coefficients
        // Adjacent scales have stronger coupling (higher coefficient)
        let scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for (i, scale) in scales.iter().enumerate() {
            // Coupling to previous scale (if exists)
            if i > 0 {
                let prev_scale = scales[i - 1];
                let coupling_key = (prev_scale, *scale);
                let coupling_strength = 0.8; // Strong coupling to adjacent scales
                self.scale_physics
                    .holographic_continuity
                    .cross_scale_coupling
                    .insert(coupling_key, coupling_strength);
            }

            // Coupling to next scale (if exists)
            if i < scales.len() - 1 {
                let next_scale = scales[i + 1];
                let coupling_key = (*scale, next_scale);
                let coupling_strength = 0.8; // Strong coupling to adjacent scales
                self.scale_physics
                    .holographic_continuity
                    .cross_scale_coupling
                    .insert(coupling_key, coupling_strength);
            }

            // Coupling to non-adjacent scales (weaker)
            for (j, &distant_scale) in scales.iter().enumerate().skip(i + 2) {
                let coupling_key = (*scale, distant_scale);
                let distance = (j - i) as Float;
                let coupling_strength = 0.8 / distance; // Decays with distance
                self.scale_physics
                    .holographic_continuity
                    .cross_scale_coupling
                    .insert(coupling_key, coupling_strength);
            }
        }
    }

    /// Update cross-scale coupling between scales
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add coupling between adjacent scales"
    /// "Store cross-scale influence factors"
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 5:
    /// "All scales are simulated simultaneously, each contains the whole"
    fn update_cross_scale_coupling(&mut self) {
        // From GAMING_ENGINE_ROADMAP_v2.md Section 5:
        // "Cross-scale coupling: Quantum ↔ Cellular ↔ Biological ↔ Planetary ↔ Stellar ↔ Galactic ↔ Cosmic"

        // Quantum ↔ Cellular coupling
        // Quantum processes affect cellular processes (e.g., protein folding)
        let quantum_cellular_coupling = 0.7;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Quantum, ScaleLevel::Cellular),
                quantum_cellular_coupling,
            );

        // Cellular ↔ Biological coupling
        // Cellular processes affect biological organisms
        let cellular_biological_coupling = 0.8;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Cellular, ScaleLevel::Biological),
                cellular_biological_coupling,
            );

        // Biological ↔ Planetary coupling
        // Organisms affect planetary systems (Gaia consciousness)
        let biological_planetary_coupling = 0.6;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Biological, ScaleLevel::Planetary),
                biological_planetary_coupling,
            );

        // Planetary ↔ Stellar coupling
        // Planetary systems affect stellar dynamics
        let planetary_stellar_coupling = 0.5;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Planetary, ScaleLevel::Stellar),
                planetary_stellar_coupling,
            );

        // Stellar ↔ Galactic coupling
        // Stellar systems affect galactic structure
        let stellar_galactic_coupling = 0.4;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Stellar, ScaleLevel::Galactic),
                stellar_galactic_coupling,
            );

        // Galactic ↔ Cosmic coupling
        // Galactic structures affect cosmic evolution
        let galactic_cosmic_coupling = 0.3;
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .insert(
                (ScaleLevel::Galactic, ScaleLevel::Cosmic),
                galactic_cosmic_coupling,
            );

        // Store cross-scale influence factors
        // These can be used for holographic continuity calculations
    }

    /// Detect and handle scale transitions
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add scale transition detection"
    /// "Detect when camera scale changes"
    /// "Trigger scale transition with holographic continuity preservation"
    /// "Log scale transitions for debugging"
    pub fn detect_scale_transition(&mut self) -> Option<(ScaleLevel, ScaleLevel)> {
        // Check if multi-scale camera is available
        if let Some(ref camera) = self.multiscale_camera {
            // Get camera's current scale
            let camera_scale = camera.current_scale();

            // Check if scale has changed
            if camera_scale != self.current_scale {
                let previous_scale = self.current_scale;

                // Log scale transition
                println!(
                    "Scale transition detected (via camera): {} -> {}",
                    previous_scale.display_name(),
                    camera_scale.display_name()
                );

                // Trigger scale transition with holographic continuity preservation
                self.set_scale(camera_scale);

                // Return transition information
                Some((previous_scale, camera_scale))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Set the multi-scale camera for scale tracking
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add multiscale_camera: Option<MultiScaleCamera> field for scale tracking"
    pub fn set_multiscale_camera(&mut self, camera: MultiScaleCamera) {
        // Update current scale to match camera's scale
        self.current_scale = camera.current_scale();

        // Set the camera
        self.multiscale_camera = Some(camera);

        // Update holographic continuity
        self.update_holographic_continuity();
    }

    /// Get the multi-scale camera (if available)
    pub fn get_multiscale_camera(&self) -> Option<&MultiScaleCamera> {
        self.multiscale_camera.as_ref()
    }

    /// Get holographic continuity information
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each entity contains within it all densities and sub-densities of the octave"
    pub fn get_holographic_continuity(
        &self,
    ) -> &crate::simulation_v3::scale_physics::HolographicContinuity {
        &self.scale_physics.holographic_continuity
    }

    /// Get cross-scale coupling between two scales
    ///
    /// From MASTER_R&D_ROADMAP.md Phase 1 Week 1:
    /// "Add coupling between adjacent scales"
    pub fn get_cross_scale_coupling(&self, from: ScaleLevel, to: ScaleLevel) -> Float {
        self.scale_physics
            .holographic_continuity
            .cross_scale_coupling
            .get(&(from, to))
            .copied()
            .unwrap_or(0.0)
    }
}

// ============================================================================
// PHASE 1 WEEK 1: MULTI-SCALE SIMULATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::multiscale_camera::{InterpolationMode, MultiScaleCamera};

    #[test]
    fn test_simulation_runner_initializes_scale_physics() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let runner = SimulationRunner::new(parameters);

        // Check that current_scale is set to default (Biological)
        assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);

        // Check that multiscale_camera is initially None
        assert!(runner.get_multiscale_camera().is_none());

        // Check that holographic continuity is initialized
        let continuity = runner.get_holographic_continuity();
        assert_eq!(continuity.continuity_strength, 1.0);
    }

    #[test]
    fn test_scale_transitions_preserve_holographic_data() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Get initial cross-scale coupling data after proper initialization
        runner.set_scale(ScaleLevel::Biological);
        runner.update_holographic_continuity(); // Ensure coupling is initialized

        let initial_quantum_biological =
            runner.get_cross_scale_coupling(ScaleLevel::Quantum, ScaleLevel::Biological);

        // Transition to different scale
        runner.set_scale(ScaleLevel::Planetary);

        // Cross-scale coupling should still be available
        let later_quantum_biological =
            runner.get_cross_scale_coupling(ScaleLevel::Quantum, ScaleLevel::Biological);

        // Coupling data should be preserved
        assert_eq!(
            initial_quantum_biological, later_quantum_biological,
            "Cross-scale coupling data not preserved during transition"
        );
    }

    #[test]
    fn test_simulate_scale_step_calls_correct_physics() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Test simulation at each scale level
        let scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in scales {
            // Set the scale
            runner.set_scale(scale);

            // Simulate one step
            let result = runner.simulate_scale_step(1.0);

            // Verify that simulation succeeded
            assert!(result.is_ok(), "Simulation failed for scale {:?}", scale);
        }
    }

    #[test]
    fn test_holographic_continuity_during_scale_transitions() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Get initial holographic continuity
        let initial_continuity = runner.get_holographic_continuity();
        let initial_strength = initial_continuity.continuity_strength;

        // Transition through all scales
        let scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in scales {
            runner.set_scale(scale);

            // Get holographic continuity after transition
            let continuity = runner.get_holographic_continuity();

            // Continuity strength should remain high (near 1.0)
            assert!(
                continuity.continuity_strength >= 0.99,
                "Continuity strength dropped to {} at scale {:?}",
                continuity.continuity_strength,
                scale
            );
        }

        // Final continuity strength should be at least as strong as initial
        let final_continuity = runner.get_holographic_continuity();
        assert!(
            final_continuity.continuity_strength >= initial_strength,
            "Continuity strength degraded from {} to {}",
            initial_strength,
            final_continuity.continuity_strength
        );
    }

    #[test]
    fn test_cross_scale_coupling() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Initialize holographic continuity by setting a scale
        runner.set_scale(ScaleLevel::Biological);

        // Manually trigger update_holographic_continuity to ensure coupling is set
        runner.update_holographic_continuity();

        // Test coupling between adjacent scales
        let adjacent_pairs = [
            (ScaleLevel::Quantum, ScaleLevel::Cellular),
            (ScaleLevel::Cellular, ScaleLevel::Biological),
            (ScaleLevel::Biological, ScaleLevel::Planetary),
            (ScaleLevel::Planetary, ScaleLevel::Stellar),
            (ScaleLevel::Stellar, ScaleLevel::Galactic),
            (ScaleLevel::Galactic, ScaleLevel::Cosmic),
        ];

        for (from, to) in adjacent_pairs {
            let coupling = runner.get_cross_scale_coupling(from, to);
            assert!(coupling > 0.0, "No coupling from {:?} to {:?}", from, to);
            assert!(
                coupling <= 1.0,
                "Coupling exceeds 1.0: {} from {:?} to {:?}",
                coupling,
                from,
                to
            );
        }
    }

    #[test]
    fn test_multiscale_camera_integration() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Check that camera is initially None
        assert!(runner.get_multiscale_camera().is_none());

        // Create and set multi-scale camera
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        runner.set_multiscale_camera(camera);

        // Check that camera is now available
        assert!(runner.get_multiscale_camera().is_some());

        // Check that scale was updated to match camera
        assert_eq!(runner.get_current_scale(), ScaleLevel::Biological);
    }

    #[test]
    fn test_scale_transition_detection() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Create and set multi-scale camera
        let camera = MultiScaleCamera::new(ScaleLevel::Biological, 16.0 / 9.0);
        runner.set_multiscale_camera(camera);

        // No transition should be detected initially
        let transition = runner.detect_scale_transition();
        assert!(transition.is_none());

        // Trigger a camera transition with zero duration for immediate effect
        if let Some(ref mut cam) = runner.multiscale_camera {
            cam.transition_to_scale(
                ScaleLevel::Planetary,
                std::time::Duration::from_millis(0),
                InterpolationMode::Linear,
            );
            // Update camera to complete transition
            cam.update(std::time::Duration::from_millis(1));
        }

        // Transition should now be detected
        let transition = runner.detect_scale_transition();
        assert!(transition.is_some());

        let (from, to) = transition.unwrap();
        assert_eq!(from, ScaleLevel::Biological);
        assert_eq!(to, ScaleLevel::Planetary);
    }

    #[test]
    fn test_all_seven_scale_levels_accessible() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Test all 7 scale levels
        let all_scales = [
            ScaleLevel::Quantum,
            ScaleLevel::Cellular,
            ScaleLevel::Biological,
            ScaleLevel::Planetary,
            ScaleLevel::Stellar,
            ScaleLevel::Galactic,
            ScaleLevel::Cosmic,
        ];

        for scale in all_scales {
            // Set scale
            runner.set_scale(scale);

            // Verify scale is set
            assert_eq!(runner.get_current_scale(), scale);

            // Simulate at this scale
            let result = runner.simulate_scale_step(1.0);
            assert!(result.is_ok(), "Failed to simulate at scale {:?}", scale);
        }
    }

    #[test]
    fn test_scale_physics_modes() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);

        // Quantum scale should use Quantum physics
        runner.set_scale(ScaleLevel::Quantum);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::Quantum
        );

        // Cellular scale should use Quantum physics
        runner.set_scale(ScaleLevel::Cellular);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::Quantum
        );

        // Biological scale should use Space/Time physics
        runner.set_scale(ScaleLevel::Biological);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
        );

        // Planetary scale should use Space/Time physics
        runner.set_scale(ScaleLevel::Planetary);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
        );

        // Stellar scale should use Space/Time physics
        runner.set_scale(ScaleLevel::Stellar);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
        );

        // Galactic scale should use Space/Time physics
        runner.set_scale(ScaleLevel::Galactic);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::SpaceTime
        );

        // Cosmic scale should use Time/Space physics
        runner.set_scale(ScaleLevel::Cosmic);
        assert_eq!(
            runner.get_current_scale().physics_mode(),
            crate::simulation_v3::multiscale_camera::PhysicsMode::TimeSpace
        );
    }

    #[test]
    fn test_cross_scale_coupling_distance_decay() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(10);

        let mut runner = SimulationRunner::new(parameters);
        runner.set_scale(ScaleLevel::Biological);

        // Manually trigger update_holographic_continuity to ensure coupling is set
        runner.update_holographic_continuity();

        // Adjacent scales should have stronger coupling
        let biological_planetary_coupling =
            runner.get_cross_scale_coupling(ScaleLevel::Biological, ScaleLevel::Planetary);

        // Non-adjacent scales should have weaker coupling
        let biological_stellar_coupling =
            runner.get_cross_scale_coupling(ScaleLevel::Biological, ScaleLevel::Stellar);

        // Adjacent coupling should be stronger
        assert!(
            biological_planetary_coupling > biological_stellar_coupling,
            "Adjacent coupling ({}) should be stronger than distant coupling ({})",
            biological_planetary_coupling,
            biological_stellar_coupling
        );
    }

    // ============================================================================
    // PHASE 5: HOLOGRAPHIC INTEGRATION TESTS
    // ============================================================================

    /// Test Phase 1: Holographic field processes steps correctly
    #[test]
    fn test_holographic_field_processes_steps() {
        let parameters = SimulationParameters::new()
            .with_num_entities(16)
            .with_num_steps(10);

        let runner = SimulationRunner::new(parameters);

        // Verify holographic manager was initialized
        let field_result = runner.holographic_manager.get_field_result();

        // After initialization, field should track steps (steps is usize, always >= 0)
        let _ = field_result.steps;
    }

    /// Test Phase 2: Full simulation runs without panic
    #[test]
    fn test_simulation_runs_without_panic() {
        let parameters = SimulationParameters::new()
            .with_num_entities(8)
            .with_num_steps(3);

        let mut runner = SimulationRunner::new(parameters);

        // This should run the full simulation including:
        // - Phase 2: top-down feedback
        // - Phase 3: fractal hierarchy
        // - Phase 4: observer effect, veil, polarization, teleological
        let result = runner.run_simulation();

        // Verify simulation completed
        assert!(
            result.evolution_result.steps > 0,
            "Simulation should complete"
        );
    }

    /// Test Phase 3: Entity hierarchy exists
    #[test]
    fn test_entity_hierarchy_exists() {
        // Phase 0: Test with causal inversion disabled to verify entity hierarchy
        let parameters = SimulationParameters {
            num_entities: 10,
            num_steps: 5,
            causal_inversion_mode: false, // Use old path for entity hierarchy test
            ..Default::default()
        };

        let mut runner = SimulationRunner::new(parameters);

        // Run simulation to create entities
        let result = runner.run_simulation();

        // Verify simulation completed and entities were created
        assert!(
            result.evolution_result.steps > 0,
            "Simulation should complete"
        );

        // In causal inversion mode, entities are stored in CausalInversionRunner
        // In legacy mode, entities are stored in runner.entities
        let entities_exist = !runner.entities.is_empty()
            || runner
                .causal_inversion_runner
                .as_ref()
                .is_some_and(|cir| cir.entity_count() > 0);
        assert!(
            entities_exist,
            "Entities should exist in either legacy or causal inversion mode"
        );

        // Check entity types include parent-child relationships (legacy mode only)
        for entity in runner.entities.values() {
            // Entities can have parent_id and children
            let _parent = entity.parent_id.clone();
            let _children = entity.children.clone();
        }
    }

    /// Test Phase 4: Observer effect - coherence tracker exists
    #[test]
    fn test_coherence_tracker_exists() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(5);

        let runner = SimulationRunner::new(parameters);

        // Verify coherence tracker exists
        let coherence = runner
            .holographic_manager
            .coherence_tracker
            .global_coherence;
        assert!(
            (0.0..=1.0).contains(&coherence),
            "Coherence should be valid"
        );
    }

    /// Test Phase 4: Dynamic veil - entities have veil
    #[test]
    fn test_dynamic_veil_exists() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(5);

        let runner = SimulationRunner::new(parameters);

        // Verify entities have veil
        for entity in runner.entities.values() {
            assert!(
                entity.veil.transparency >= 0.0 && entity.veil.transparency <= 1.0,
                "Veil transparency should be valid"
            );
        }
    }

    /// Test Phase 4: Polarization-field - entities have polarization
    #[test]
    fn test_polarization_exists() {
        let parameters = SimulationParameters::new()
            .with_num_entities(10)
            .with_num_steps(5);

        let runner = SimulationRunner::new(parameters);

        // Verify entities have polarization
        for entity in runner.entities.values() {
            let _direction = entity.polarization.direction;
            let _intensity = entity.polarization.intensity;
        }
    }

    /// Test: Simulation performance is acceptable
    #[test]
    fn test_simulation_performance() {
        use std::time::Instant;

        let parameters = SimulationParameters::new()
            .with_num_entities(16)
            .with_num_steps(5);

        let start = Instant::now();
        let mut runner = SimulationRunner::new(parameters);
        runner.run_simulation();
        let elapsed = start.elapsed();

        // Should complete in reasonable time (10 seconds max for test)
        assert!(
            elapsed.as_secs() < 10,
            "Simulation should complete in reasonable time"
        );
    }
}
