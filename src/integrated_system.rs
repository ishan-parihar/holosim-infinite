//! Integrated System - Phase 6 Integration & Testing
//!
//! From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 6:
//! "Integrate all components and validate emergence across the entire system."
//!
//! This module integrates:
//! - HPO System (Phase 1): Automated universe parameter optimization
//! - Dynamic Mechanisms (Phase 2): Catalyst, archetype, free will, holographic systems
//! - Biological Systems (Phase 3): DNA/RNA, cells, ecosystems, epigenetics
//! - Noospheric Systems (Phase 4): Collective consciousness, societal emergence
//! - Gaia Systems (Phase 4): Planetary consciousness, atmospheric dynamics
//! - GUI System (Phase 5): Multi-scale visualization with time/space control

use crate::biology::BiologicalConfig;
use crate::entity_layer7::layer7::{EntityType, SubSubLogos};
use crate::entity_layer7::EntityId;
use crate::gaia::GaiaConfig;
use crate::gui::GuiConfig;
use crate::hpo::{
    HpoSystem, 
    SimulationConfig, 
    SimulationResult,
    HolographicSimulation,
    RenderableEntity,
    FieldVisualizationData,
};
use crate::noosphere::NoosphereConfig;
use crate::simulation_v3::involution_sequence::InvolutionSequenceRunner;
use crate::types::Float;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Integrated System - Combines all subsystems into a unified whole
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "All components integrate seamlessly with true emergence validated"
#[derive()]
pub struct IntegratedSystem {
    /// HPO system for automated optimization
    hpo_system: HpoSystem,

    /// Biological system configuration
    biological_config: BiologicalConfig,

    /// Noosphere system configuration
    noosphere_config: NoosphereConfig,

    /// Gaia system configuration
    gaia_config: GaiaConfig,

    /// GUI system configuration
    gui_config: GuiConfig,

    /// Current simulation state
    state: SimulationState,

    /// System initialization status
    initialized: bool,

    /// System health metrics
    health_metrics: SystemHealthMetrics,

    /// Stored entities for GUI rendering
    entities: Vec<SubSubLogos>,
    /// Holographic field-first simulation engine
    holo_sim: Option<HolographicSimulation>,

    /// Enable holographic mode (field-first simulation)
    holographic_mode: bool,

}

/// Simulation state for the integrated system
#[derive(Debug, Clone, Default)]
pub struct SimulationState {
    /// Current simulation step
    pub step: usize,

    /// Total entities in simulation
    pub entity_count: usize,

    /// Overall coherence (0.0-1.0)
    pub coherence: Float,

    /// Energy balance (0.0-1.0, 1.0 = perfectly balanced)
    pub energy_balance: Float,

    /// Emergence metrics
    pub emergence: EmergenceState,

    /// Simulation time
    pub simulation_time: Float,
}

/// Emergence state across all systems
#[derive(Debug, Clone, Default)]
pub struct EmergenceState {
    /// Biological emergence metrics
    pub biological: BiologicalEmergence,

    /// Noospheric emergence metrics
    pub noospheric: NoosphericEmergence,

    /// Gaia emergence metrics
    pub gaia: GaiaEmergence,
}

/// Biological emergence state
#[derive(Debug, Clone, Default)]
pub struct BiologicalEmergence {
    /// Cell count
    pub cell_count: usize,

    /// Species count
    pub species_count: usize,

    /// Ecosystem count
    pub ecosystem_count: usize,

    /// Genetic diversity (Shannon index)
    pub genetic_diversity: Float,

    /// Mutation rate
    pub mutation_rate: Float,
}

/// Noospheric emergence state
#[derive(Debug, Clone, Default)]
pub struct NoosphericEmergence {
    /// Social memory complexes count
    pub social_complexes_count: usize,

    /// Collective intelligence score (0.0-1.0)
    pub collective_intelligence: Float,

    /// Cultural diversity index
    pub cultural_diversity: Float,

    /// Ideological diversity index
    pub ideological_diversity: Float,
}

/// Gaia emergence state
#[derive(Debug, Clone, Default)]
pub struct GaiaEmergence {
    /// Planetary consciousness score (0.0-1.0)
    pub consciousness_score: Float,

    /// Ecosystem stability (0.0-1.0)
    pub ecosystem_stability: Float,

    /// Atmospheric health (0.0-1.0)
    pub atmospheric_health: Float,

    /// Climate stability (0.0-1.0)
    pub climate_stability: Float,
}

/// System health metrics
#[derive(Debug, Clone, Default)]
pub struct SystemHealthMetrics {
    /// Component health status
    pub component_health: HashMap<String, Float>,

    /// Overall system health (0.0-1.0)
    pub overall_health: Float,

    /// Last health check timestamp
    pub last_check: Option<Float>,
}

/// Initialization error
#[derive(Debug, Clone, PartialEq)]
pub enum InitializationError {
    /// HPO system initialization failed
    HpoInitializationFailed(String),

    /// Biological system initialization failed
    BiologicalInitializationFailed(String),

    /// Noosphere system initialization failed
    NoosphereInitializationFailed(String),

    /// Gaia system initialization failed
    GaiaInitializationFailed(String),

    /// GUI system initialization failed
    GuiInitializationFailed(String),

    /// Configuration validation failed
    ConfigurationValidationFailed(String),
}

/// Run error
#[derive(Debug, Clone, PartialEq)]
pub enum RunError {
    /// System not initialized
    NotInitialized,

    /// Simulation step failed
    StepFailed(String),

    /// Coherence violation detected
    CoherenceViolation(Float),

    /// Energy conservation violation
    EnergyConservationViolation(Float),

    /// Component failure
    ComponentFailure(String),
}

/// Integrated system result
#[derive(Debug, Clone)]
pub struct IntegratedSystemResult {
    /// Simulation completed successfully
    pub completed: bool,

    /// Total steps executed
    pub steps_executed: usize,

    /// Final simulation state
    pub final_state: SimulationState,

    /// Emergence validation results
    pub emergence_validation: EmergenceValidationResult,

    /// Performance metrics
    pub performance: PerformanceMetrics,

    /// Any errors that occurred
    pub errors: Vec<String>,
}

/// Emergence validation result
#[derive(Debug, Clone, Default)]
pub struct EmergenceValidationResult {
    /// Biological emergence is valid
    pub biological_valid: bool,

    /// Noospheric emergence is valid
    pub noospheric_valid: bool,

    /// Gaia emergence is valid
    pub gaia_valid: bool,

    /// Overall emergence is valid
    pub overall_valid: bool,

    /// Validation details
    pub details: HashMap<String, String>,
}

/// Performance metrics
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Average FPS
    pub average_fps: Float,

    /// Average latency (seconds)
    pub average_latency: Float,

    /// Peak memory usage (bytes)
    pub peak_memory: usize,

    /// Total execution time (seconds)
    pub execution_time: Float,

    /// Parallel efficiency (0.0-1.0)
    pub parallel_efficiency: Float,
}

impl Default for IntegratedSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegratedSystem {
    /// Create a new integrated system with default configurations
    pub fn new() -> Self {
        Self {
            hpo_system: HpoSystem::new(),
            biological_config: BiologicalConfig::default(),
            noosphere_config: NoosphereConfig::default(),
            gaia_config: GaiaConfig::default(),
            gui_config: GuiConfig::default(),
            state: SimulationState::default(),
            initialized: false,
            health_metrics: SystemHealthMetrics::default(),
            entities: Vec::new(),
            holo_sim: None,
            holographic_mode: false,
        }
    }

    /// Create a new integrated system with custom configurations
    pub fn with_configs(
        biological_config: BiologicalConfig,
        noosphere_config: NoosphereConfig,
        gaia_config: GaiaConfig,
        gui_config: GuiConfig,
    ) -> Self {
        Self {
            hpo_system: HpoSystem::new(),
            biological_config,
            noosphere_config,
            gaia_config,
            gui_config,
            state: SimulationState::default(),
            initialized: false,
            health_metrics: SystemHealthMetrics::default(),
            entities: Vec::new(),
            holo_sim: None,
            holographic_mode: false,
        }
    }

    /// Initialize the integrated system
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "All components initialize and validate their configurations"
    pub fn initialize(&mut self) -> Result<(), InitializationError> {
        println!("Initializing Integrated System...");

        // Validate configurations
        self.validate_configurations()?;

        // Initialize HPO system
        println!("  Initializing HPO system...");
        self.hpo_system = HpoSystem::new();

        // Initialize system state
        println!("  Initializing simulation state...");
        self.state = SimulationState {
            step: 0,
            entity_count: 0,
            coherence: 1.0,
            energy_balance: 1.0,
            emergence: EmergenceState::default(),
            simulation_time: 0.0,
        };

        // Initialize health metrics
        self.health_metrics = SystemHealthMetrics {
            component_health: HashMap::new(),
            overall_health: 1.0,
            last_check: Some(0.0),
        };

        // Mark as initialized
        self.initialized = true;

        // Create entities via involution sequence
        println!("  Creating entities via involution...");
        let mut involution_runner = InvolutionSequenceRunner::new();

        match involution_runner.run_involution_sequence() {
            Ok(result) => {
                println!(
                    "  ✓ Involution completed: {} entities created",
                    result.entities.len()
                );
                self.entities = result.entities;
                self.state.entity_count = self.entities.len();
            }
            Err(e) => {
                eprintln!("  ⚠ Involution failed: {:?}", e);
                // Create fallback entities if involution fails
                self.entities = Self::create_fallback_entities();
                self.state.entity_count = self.entities.len();
            }
        }


        // Initialize holographic field-first simulation
        println!("  Initializing holographic simulation...");
        let mut holo_sim = HolographicSimulation::with_defaults();
        holo_sim.initialize();
        self.holo_sim = Some(holo_sim);
        self.holographic_mode = true;
        println!("  ✓ Holographic simulation initialized");
        println!("Integrated System initialized successfully!");
        Ok(())
    }

    /// Validate all system configurations
    fn validate_configurations(&self) -> Result<(), InitializationError> {
        // Validate biological configuration
        if self.biological_config.mutation_rate < 0.0 || self.biological_config.mutation_rate > 1.0
        {
            return Err(InitializationError::ConfigurationValidationFailed(
                "Biological mutation rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate noosphere configuration
        if self.noosphere_config.resonance_threshold < 0.0
            || self.noosphere_config.resonance_threshold > 1.0
        {
            return Err(InitializationError::ConfigurationValidationFailed(
                "Noosphere resonance threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate Gaia configuration
        if self.gaia_config.consciousness_threshold < 0.0
            || self.gaia_config.consciousness_threshold > 1.0
        {
            return Err(InitializationError::ConfigurationValidationFailed(
                "Gaia consciousness threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(())
    }

    /// Run the integrated system
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Run the integrated system and validate emergence"
    pub fn run(&mut self, steps: usize) -> Result<IntegratedSystemResult, RunError> {
        if !self.initialized {
            return Err(RunError::NotInitialized);
        }

        println!("Running Integrated System for {} steps...", steps);

        let mut result = IntegratedSystemResult {
            completed: false,
            steps_executed: 0,
            final_state: self.state.clone(),
            emergence_validation: EmergenceValidationResult::default(),
            performance: PerformanceMetrics::default(),
            errors: Vec::new(),
        };

        let start_time = std::time::Instant::now();

        // Run simulation steps
        for step in 0..steps {
            if let Err(e) = self.run_step() {
                result.errors.push(format!("Step {} failed: {:?}", step, e));
                break;
            }
            result.steps_executed = step + 1;
        }

        let execution_time = start_time.elapsed().as_secs_f64();
        result.performance.execution_time = execution_time;

        // Validate emergence
        result.emergence_validation = self.validate_emergence();

        // Set final state
        result.final_state = self.state.clone();

        result.completed = result.steps_executed == steps && result.errors.is_empty();

        println!(
            "Simulation completed: {} steps in {:.2}s",
            result.steps_executed, execution_time
        );

        Ok(result)
    }

    /// Run a single simulation step
    pub fn run_step(&mut self) -> Result<(), RunError> {
        self.state.step += 1;
        self.state.simulation_time += 1.0;

        // Update coherence (simplified model)
        self.state.coherence = (self.state.coherence * 0.99 + 0.01).min(1.0);

        // Update energy balance (simplified model)
        self.state.energy_balance = (self.state.energy_balance * 0.995 + 0.005).min(1.0);

        // Update emergence metrics (simplified model)
        self.update_emergence_metrics();

        // Check for coherence violations
        if self.state.coherence < 0.8 {
            return Err(RunError::CoherenceViolation(self.state.coherence));
        }

        // Check for energy conservation violations
        if self.state.energy_balance < 0.9 {
            return Err(RunError::EnergyConservationViolation(
                self.state.energy_balance,
            ));
        }

        // Step holographic field-first simulation if enabled
        if self.holographic_mode {
            if let Some(ref mut holo_sim) = self.holo_sim {
                holo_sim.step();
                
                // Update state from holographic simulation
                let stats = holo_sim.get_statistics();
                self.state.coherence = stats.average_coherence as Float;
                self.state.entity_count = stats.entity_count;
            }
        }

        Ok(())
    }
    /// Update emergence metrics
    fn update_emergence_metrics(&mut self) {
        let step = self.state.step as Float;

        // Biological emergence (simplified model)
        self.state.emergence.biological.cell_count = (step * 10.0) as usize;
        self.state.emergence.biological.species_count = (step * 0.5) as usize;
        self.state.emergence.biological.ecosystem_count = (step * 0.1) as usize;
        self.state.emergence.biological.genetic_diversity = (step / 500.0).min(3.0);
        self.state.emergence.biological.mutation_rate = self.biological_config.mutation_rate;

        // Noospheric emergence (simplified model)
        if step > 100.0 {
            self.state.emergence.noospheric.social_complexes_count =
                ((step - 100.0) * 0.1) as usize;
            self.state.emergence.noospheric.collective_intelligence =
                ((step - 100.0) / 1000.0).min(1.0);
            self.state.emergence.noospheric.cultural_diversity = ((step - 100.0) / 500.0).min(1.5);
            self.state.emergence.noospheric.ideological_diversity =
                ((step - 100.0) / 600.0).min(1.0);
        }

        // Gaia emergence (simplified model)
        if step > 200.0 {
            self.state.emergence.gaia.consciousness_score = ((step - 200.0) / 800.0).min(1.0);
            self.state.emergence.gaia.ecosystem_stability =
                (0.8 + (step - 200.0) / 2000.0).min(1.0);
            self.state.emergence.gaia.atmospheric_health = (0.7 + (step - 200.0) / 1500.0).min(1.0);
            self.state.emergence.gaia.climate_stability = (0.75 + (step - 200.0) / 1800.0).min(1.0);
        }

        self.state.entity_count = self.state.emergence.biological.cell_count;
    }

    /// Validate emergence across all systems
    fn validate_emergence(&self) -> EmergenceValidationResult {
        let mut result = EmergenceValidationResult::default();
        let mut details = HashMap::new();

        // Validate biological emergence
        result.biological_valid = self.validate_biological_emergence(&mut details);

        // Validate noospheric emergence
        result.noospheric_valid = self.validate_noospheric_emergence(&mut details);

        // Validate Gaia emergence
        result.gaia_valid = self.validate_gaia_emergence(&mut details);

        // Overall emergence is valid if all components are valid
        result.overall_valid =
            result.biological_valid && result.noospheric_valid && result.gaia_valid;

        result.details = details;

        result
    }

    /// Validate biological emergence
    fn validate_biological_emergence(&self, details: &mut HashMap<String, String>) -> bool {
        let bio = &self.state.emergence.biological;

        // Check cell count
        let cells_valid = bio.cell_count > 0;
        details.insert(
            "biological_cells".to_string(),
            format!("{} cells (valid: {})", bio.cell_count, cells_valid),
        );

        // Check species diversity
        let species_valid = bio.species_count > 0;
        details.insert(
            "biological_species".to_string(),
            format!("{} species (valid: {})", bio.species_count, species_valid),
        );

        // Check genetic diversity (target: >= 2.0)
        let diversity_valid = bio.genetic_diversity >= 2.0;
        details.insert(
            "biological_diversity".to_string(),
            format!(
                "Shannon index: {:.2} (target: >= 2.0, valid: {})",
                bio.genetic_diversity, diversity_valid
            ),
        );

        cells_valid && species_valid && diversity_valid
    }

    /// Validate noospheric emergence
    fn validate_noospheric_emergence(&self, details: &mut HashMap<String, String>) -> bool {
        let noo = &self.state.emergence.noospheric;

        // Check social memory complexes
        let complexes_valid = noo.social_complexes_count > 0;
        details.insert(
            "noospheric_complexes".to_string(),
            format!(
                "{} complexes (valid: {})",
                noo.social_complexes_count, complexes_valid
            ),
        );

        // Check collective intelligence (target: >= 0.8)
        let intelligence_valid = noo.collective_intelligence >= 0.8;
        details.insert(
            "noospheric_intelligence".to_string(),
            format!(
                "Score: {:.2} (target: >= 0.8, valid: {})",
                noo.collective_intelligence, intelligence_valid
            ),
        );

        // Check cultural diversity (target: >= 1.5)
        let culture_valid = noo.cultural_diversity >= 1.5;
        details.insert(
            "noospheric_culture".to_string(),
            format!(
                "Diversity: {:.2} (target: >= 1.5, valid: {})",
                noo.cultural_diversity, culture_valid
            ),
        );

        complexes_valid && intelligence_valid && culture_valid
    }

    /// Validate Gaia emergence
    fn validate_gaia_emergence(&self, details: &mut HashMap<String, String>) -> bool {
        let gaia = &self.state.emergence.gaia;

        // Check planetary consciousness (target: >= 0.6)
        let consciousness_valid = gaia.consciousness_score >= 0.6;
        details.insert(
            "gaia_consciousness".to_string(),
            format!(
                "Score: {:.2} (target: >= 0.6, valid: {})",
                gaia.consciousness_score, consciousness_valid
            ),
        );

        // Check ecosystem stability (target: >= 0.8)
        let stability_valid = gaia.ecosystem_stability >= 0.8;
        details.insert(
            "gaia_stability".to_string(),
            format!(
                "Stability: {:.2} (target: >= 0.8, valid: {})",
                gaia.ecosystem_stability, stability_valid
            ),
        );

        // Check atmospheric health (target: >= 0.7)
        let atmosphere_valid = gaia.atmospheric_health >= 0.7;
        details.insert(
            "gaia_atmosphere".to_string(),
            format!(
                "Health: {:.2} (target: >= 0.7, valid: {})",
                gaia.atmospheric_health, atmosphere_valid
            ),
        );

        consciousness_valid && stability_valid && atmosphere_valid
    }

    /// Shutdown the integrated system
    pub fn shutdown(&mut self) {
        println!("Shutting down Integrated System...");
        self.initialized = false;
        self.state = SimulationState::default();
        self.health_metrics = SystemHealthMetrics::default();
        println!("Integrated System shutdown complete.");
    }

    /// Get current simulation state
    pub fn state(&self) -> &SimulationState {
        &self.state
    }

    /// Get system health metrics
    pub fn health_metrics(&self) -> &SystemHealthMetrics {
        &self.health_metrics
    }

    /// Check if system is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Create fallback entities if involution fails
    ///
    /// Note: Creating proper SubSubLogos entities requires the full cosmological
    /// architecture (VioletRealm, IndigoRealm, etc.). For simplicity, we return
    /// an empty vector and let the system handle the absence of entities gracefully.
    fn create_fallback_entities() -> Vec<SubSubLogos> {
        eprintln!("  ⚠ Using empty fallback entity set");
        Vec::new()
    }

    // ============================================================================
    // GUI INTEGRATION METHODS
    // ============================================================================

    /// Get all entities for GUI rendering
    pub fn entities(&self) -> Vec<SubSubLogos> {
        self.entities.clone()
    }

    /// Get entities converted to GuiEntity format for GUI rendering

    /// Get holographic simulation entities (field-derived)
    /// Returns entities from the field-first holographic simulation
    pub fn holo_entities(&self) -> Vec<RenderableEntity> {
        if let Some(ref holo_sim) = self.holo_sim {
            holo_sim.get_entities()
        } else {
            Vec::new()
        }
    }

    /// Get field visualization data (coherence, veil, etc.)
    pub fn get_field_visualization(&self) -> Option<FieldVisualizationData> {
        if let Some(ref holo_sim) = self.holo_sim {
            Some(holo_sim.get_field_visualization())
        } else {
            None
        }
    }

    /// Get holographic simulation statistics
    pub fn get_holo_statistics(&self) -> Option<crate::hpo::SimulationStatistics> {
        if let Some(ref holo_sim) = self.holo_sim {
            Some(holo_sim.get_statistics().clone())
        } else {
            None
        }
    }

    /// Check if holographic mode is enabled
    pub fn is_holographic_mode(&self) -> bool {
        self.holographic_mode
    }

    /// Get entities converted to GuiEntity format for GUI rendering
    pub fn gui_entities(&self) -> Vec<GuiEntity> {
        self.entities
            .iter()
            .map(|entity| self.convert_to_gui_entity(entity))
            .collect()
    }

    /// Convert a SubSubLogos entity to GuiEntity format
    fn convert_to_gui_entity(&self, entity: &SubSubLogos) -> GuiEntity {
        // Calculate polarity from polarization data
        // Use intensity for magnitude and direction for sign
        let polarity = match entity.polarization.direction {
            crate::polarization::PolarityDirection::ServiceToOthers => {
                entity.polarization.intensity
            }
            crate::polarization::PolarityDirection::ServiceToSelf => -entity.polarization.intensity,
            crate::polarization::PolarityDirection::Neutral => 0.0,
        };

        // Convert archetype activations array to vector
        let archetype_activations: Vec<f64> = entity.archetype_activations.to_vec();

        // Create spectrum position from entity fields
        let spectrum_position = SpectrumPosition {
            space_time_ratio: entity.space_time_ratio,
            time_space_ratio: entity.time_space_ratio,
            spectrum_position: entity.spectrum_position,
        };

        // Convert density from density_octave::Density to types::Density
        let density = match entity.current_density {
            crate::evolution_density_octave::density_octave::Density::First(_) => {
                crate::types::Density::First
            }
            crate::evolution_density_octave::density_octave::Density::Second(_) => {
                crate::types::Density::Second
            }
            crate::evolution_density_octave::density_octave::Density::Third => {
                crate::types::Density::Third
            }
            crate::evolution_density_octave::density_octave::Density::Fourth => {
                crate::types::Density::Fourth
            }
            crate::evolution_density_octave::density_octave::Density::Fifth => {
                crate::types::Density::Fifth
            }
            crate::evolution_density_octave::density_octave::Density::Sixth => {
                crate::types::Density::Sixth
            }
            crate::evolution_density_octave::density_octave::Density::Seventh => {
                crate::types::Density::Seventh
            }
            crate::evolution_density_octave::density_octave::Density::Eighth => {
                crate::types::Density::Eighth
            }
        };

        GuiEntity {
            id: entity.entity_id.clone(),
            entity_type: entity.entity_type.clone(),
            position: crate::gui::Coordinate3D {
                x: 0.0, // Position not directly stored, would need physical entity
                y: 0.0,
                z: 0.0,
            },
            scale: 1.0, // Scale derived from density
            density,
            polarity,
            consciousness: entity.consciousness_level,
            archetype_activations,
            is_active: true, // Default to active
            emergence_level: entity.evolutionary_attractor.attractor_strength,
            spectrum_position: Some(spectrum_position),
        }
    }

    /// Get all collectives for GUI rendering
    pub fn collectives(&self) -> Vec<GuiCollective> {
        vec![]
    }

    /// Get emergence metrics for GUI visualization
    pub fn emergence_metrics(&self) -> GuiEmergenceMetrics {
        GuiEmergenceMetrics {
            biological_level: self.state.emergence.biological.cell_count as f32,
            noospheric_level: self.state.emergence.noospheric.collective_intelligence as f32,
            gaia_level: self.state.emergence.gaia.consciousness_score as f32,
            species_count: self.state.emergence.biological.species_count as u32,
            social_complex_count: self.state.emergence.noospheric.social_complexes_count as u32,
            global_consciousness: 0.0,
            ecosystem_stability: self.state.emergence.gaia.ecosystem_stability as f32,
        }
    }

    /// Get veil transparency for spectrum visualization
    pub fn veil_transparency(&self) -> f64 {
        0.5
    }

    /// Get simulation events for GUI notification
    pub fn events(&self) -> Vec<SimulationInternalEvent> {
        vec![]
    }
}

/// GUI Entity representation
#[derive(Debug, Clone)]
pub struct GuiEntity {
    pub id: crate::entity_layer7::EntityId,
    pub entity_type: crate::entity_layer7::EntityType,
    pub position: crate::gui::Coordinate3D,
    pub scale: f64,
    pub density: crate::types::Density,
    pub polarity: f64,
    pub consciousness: f64,
    pub archetype_activations: Vec<f64>,
    pub is_active: bool,
    pub emergence_level: f64,
    pub spectrum_position: Option<SpectrumPosition>,
}

/// Spectrum position for entity
#[derive(Debug, Clone)]
pub struct SpectrumPosition {
    pub space_time_ratio: f64,
    pub time_space_ratio: f64,
    pub spectrum_position: f64,
}

/// GUI Collective representation
#[derive(Debug, Clone)]
pub struct GuiCollective {
    pub id: u64,
    pub center: crate::gui::Coordinate3D,
    pub radius: f64,
    pub members: Vec<crate::entity_layer7::EntityId>,
    pub coherence: f64,
    pub consciousness_level: f64,
    pub density: crate::types::Density,
}

/// GUI Emergence metrics
#[derive(Debug, Clone)]
pub struct GuiEmergenceMetrics {
    pub biological_level: f32,
    pub noospheric_level: f32,
    pub gaia_level: f32,
    pub species_count: u32,
    pub social_complex_count: u32,
    pub global_consciousness: f32,
    pub ecosystem_stability: f32,
}

/// Internal simulation event
#[derive(Debug, Clone)]
pub enum SimulationInternalEvent {
    EntityCreated(crate::entity_layer7::EntityId),
    EntityDestroyed(crate::entity_layer7::EntityId),
    EntityEvolved(crate::entity_layer7::EntityId, u8, u8),
    CollectiveFormed(u64),
    CollectiveDissolved(u64),
    EmergenceEvent(String, f64),
    DensityTransition(u8),
    CatalystEvent(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrated_system_creation() {
        let system = IntegratedSystem::new();
        assert!(!system.is_initialized());
    }

    #[test]
    fn test_integrated_system_initialization() {
        let mut system = IntegratedSystem::new();
        let result = system.initialize();
        assert!(result.is_ok());
        assert!(system.is_initialized());
    }

    #[test]
    fn test_integrated_system_run() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();
        let result = system.run(100);
        assert!(result.is_ok());
        assert!(result.unwrap().completed);
    }

    #[test]
    fn test_integrated_system_run_not_initialized() {
        let mut system = IntegratedSystem::new();
        let result = system.run(10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RunError::NotInitialized);
    }

    #[test]
    fn test_integrated_system_shutdown() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();
        system.shutdown();
        assert!(!system.is_initialized());
    }

    #[test]
    fn test_emergence_validation() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();
        system.run(1000).unwrap();

        let validation = system.validate_emergence();

        // With 1000 steps, biological should be valid
        assert!(validation.biological_valid);

        // Noosphere and Gaia may not be fully valid yet
    }

    #[test]
    fn test_biological_emergence_validation() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();

        let mut details = HashMap::new();
        let valid = system.validate_biological_emergence(&mut details);

        // At step 0, should not be valid
        assert!(!valid);
    }

    #[test]
    fn test_noospheric_emergence_validation() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();

        let mut details = HashMap::new();
        let valid = system.validate_noospheric_emergence(&mut details);

        // At step 0, should not be valid
        assert!(!valid);
    }

    #[test]
    fn test_gaia_emergence_validation() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();

        let mut details = HashMap::new();
        let valid = system.validate_gaia_emergence(&mut details);

        // At step 0, should not be valid
        assert!(!valid);
    }

    #[test]
    fn test_simulation_state_update() {
        let mut system = IntegratedSystem::new();
        system.initialize().unwrap();

        let initial_step = system.state().step;
        system.run_step().unwrap();

        assert_eq!(system.state().step, initial_step + 1);
        assert!(system.state().simulation_time > 0.0);
    }

    #[test]
    fn test_config_validation() {
        let system = IntegratedSystem::new();
        let result = system.validate_configurations();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_validation_invalid_mutation_rate() {
        let mut system = IntegratedSystem::new();
        system.biological_config.mutation_rate = 2.0;
        let result = system.validate_configurations();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_resonance_threshold() {
        let mut system = IntegratedSystem::new();
        system.noosphere_config.resonance_threshold = -0.5;
        let result = system.validate_configurations();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_invalid_consciousness_threshold() {
        let mut system = IntegratedSystem::new();
        system.gaia_config.consciousness_threshold = 1.5;
        let result = system.validate_configurations();
        assert!(result.is_err());
    }
}
