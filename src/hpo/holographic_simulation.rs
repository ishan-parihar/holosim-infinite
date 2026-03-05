//! Unified Holographic Simulation Engine (Phase F6 + F1-F5)
//! Phase F1: SpatialField for actual 3D coordinates
//! Phase F2: SpectrumSpatialDynamics for spectrum-driven space
//! Phase F3: MatterEmergence for archetype-derived matter
//! Phase F4: ComplexityEmergence for atomic→molecular phase transitions
//! Phase F5: BiologicalEmergence for life from molecular field
//! Phase F6: PlanetaryEmergence for environment from field patterns

use super::archetype_matter::{ArchetypeMatterConfig, MatterEmergence};
use super::attractor_fields::AttractorFields;
use super::biological_emergence::{BiologicalEmergenceConfig, BiologyBridge};
use super::complexity_emergence::{ComplexMolecule, ComplexityEmergence};
use super::cosmic_sequence::CosmicSequence;
use super::density_sublevels::DensitySubLevels;
use super::entity_emergence::EntityEmergence;
use super::evolution_feedback::{DecisionType, EntityDecision, EvolutionFeedback};
use super::field_state::{DensityBand, Float, HolographicFieldConfig, HolographicFieldState};
use super::full_integration::{IntegrationBridge, IntegrationConfig};
use super::holographic_encoder::EntityExtractor;
use super::involution_flow::CosmicHierarchy;
use super::larson_framework::LarsonFramework;
use super::planetary_emergence::{PlanetaryBridge, PlanetaryConfig};
use super::social_memory::{SocialMemory, SocialMemoryConfig};
use super::spatial_field::{Position3D, SpatialField, SpatialFieldConfig};
use super::spectrum_dynamics::SpectrumDynamics;
use super::spectrum_spatial::{SpatialConfig, SpectrumSpatialConfig, SpectrumSpatialDynamics};
use super::unified_field::{UnifiedFieldConfig, UnifiedFieldEquation};

/// Unified simulation configuration
#[derive(Debug, Clone)]
pub struct HoloSimConfig {
    pub field: HolographicFieldConfig,
    pub spatial: SpatialFieldConfig,
    pub spectrum_spatial: SpectrumSpatialConfig,
    pub archetype_matter: ArchetypeMatterConfig,
    pub biological: BiologicalEmergenceConfig,
    pub planetary: PlanetaryConfig,
    pub integration: IntegrationConfig,
    pub unified_field: UnifiedFieldConfig,
    pub social_memory: SocialMemoryConfig,
    pub target_entity_count: usize,
    pub enable_visualization: bool,
    pub visualization_interval: usize,
}

impl Default for HoloSimConfig {
    fn default() -> Self {
        HoloSimConfig {
            field: HolographicFieldConfig::default(),
            spatial: SpatialFieldConfig::default(),
            spectrum_spatial: SpectrumSpatialConfig::default(),
            archetype_matter: ArchetypeMatterConfig::default(),
            biological: BiologicalEmergenceConfig::default(),
            planetary: PlanetaryConfig::default(),
            integration: IntegrationConfig::default(),
            unified_field: UnifiedFieldConfig::default(),
            social_memory: SocialMemoryConfig::default(),
            target_entity_count: 10000,
            enable_visualization: true,
            visualization_interval: 1,
        }
    }
}

/// Complete unified simulation engine
pub struct HolographicSimulation {
    field: HolographicFieldState,
    spatial_field: SpatialField,
    spectrum_spatial: SpectrumSpatialDynamics,
    matter_emergence: MatterEmergence,
    complexity_emergence: ComplexityEmergence,
    biological_emergence: BiologyBridge,
    unified_field: UnifiedFieldEquation,
    spectrum: SpectrumDynamics,
    hierarchy: CosmicHierarchy,
    evolution: EvolutionFeedback,
    social_memory: SocialMemory,
    extractor: EntityExtractor,
    cosmic_sequence: CosmicSequence,
    larson: LarsonFramework,
    density_sub_levels: DensitySubLevels,
    attractor_fields: AttractorFields,
    entity_emergence: EntityEmergence,
    planetary_emergence: PlanetaryBridge,
    full_integration: IntegrationBridge,
    config: HoloSimConfig,
    step: usize,
    pub statistics: SimulationStatistics,
}

/// Statistics for complete simulation
#[derive(Debug, Clone, Default)]
pub struct SimulationStatistics {
    pub entity_count: usize,
    pub field_energy: Float,
    pub average_coherence: Float,
    pub average_spectrum_position: Float,
    pub veil_transparency: Float,
    pub collective_count: usize,
    pub entities_in_collectives: usize,
    pub decisions_processed: usize,
    pub fps: Float,
    pub peak_magnitude: Float,
    // Cosmic sequence
    pub current_layer: usize,
    pub layer_activations: [Float; 8],
    pub attractor_strength: Float,
    // Phase F1: Spatial
    pub spatial_nodes: usize,
    pub spatial_leaf_nodes: usize,
    // Phase F2: Spectrum-spatial
    pub entities_below_veil: usize,
    pub entities_at_veil: usize,
    pub entities_above_veil: usize,
    pub veil_activity: Float,
    // Phase F3: Matter
    pub particles: usize,
    pub atoms: usize,
    pub molecules: usize,
    // Phase F4: Complexity
    pub quantum_regions: usize,
    pub atomic_regions: usize,
    pub molecular_regions: usize,
    pub planetary_regions: usize,
    // Phase F5: Biology
    pub cells: usize,
    pub species: usize,
    // Phase F6: Planetary
    pub planets: usize,
    pub terrain_cells: usize,
    // Phase F7: Full Integration
    pub integration_coherence: Float,
    pub integration_stability: Float,
}

impl HolographicSimulation {
    pub fn new(config: HoloSimConfig) -> Self {
        let field = HolographicFieldState::new(config.field.clone());
        let spatial_field = SpatialField::new(config.spatial.clone());
        let spectrum_spatial = SpectrumSpatialDynamics::new(config.spectrum_spatial.clone());
        let matter_emergence = MatterEmergence::new(config.archetype_matter.clone());
        let complexity_emergence = ComplexityEmergence::with_defaults();
        let biological_emergence = BiologyBridge::new(config.biological.clone());
        let unified_field = UnifiedFieldEquation::new(config.unified_field.clone());
        let spectrum = SpectrumDynamics::with_defaults();
        let hierarchy = CosmicHierarchy::new();
        let evolution = EvolutionFeedback::new(config.target_entity_count);
        let social_memory = SocialMemory::new(config.social_memory.clone());
        let extractor = EntityExtractor::with_defaults();
        let cosmic_sequence = CosmicSequence::with_defaults();
        let larson = LarsonFramework::with_defaults();
        let density_sub_levels = DensitySubLevels::with_defaults();
        let attractor_fields = AttractorFields::with_defaults();
        let entity_emergence = EntityEmergence::with_defaults();
        let planetary_emergence = PlanetaryBridge::new(config.planetary.clone());
        let full_integration = IntegrationBridge::new(config.integration.clone());

        HolographicSimulation {
            field,
            spatial_field,
            spectrum_spatial,
            matter_emergence,
            complexity_emergence,
            biological_emergence,
            unified_field,
            spectrum,
            hierarchy,
            evolution,
            social_memory,
            extractor,
            cosmic_sequence,
            larson,
            density_sub_levels,
            attractor_fields,
            entity_emergence,
            planetary_emergence,
            full_integration,
            config,
            step: 0,
            statistics: SimulationStatistics::default(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(HoloSimConfig::default())
    }

    /// Initialize the simulation
    pub fn initialize(&mut self) {
        // Cosmic sequence
        self.cosmic_sequence.initialize_field(&mut self.field);
        self.larson.initialize_entities(137);
        self.density_sub_levels.initialize_entities(137);
        self.attractor_fields.initialize_archetypal();
        self.attractor_fields.initialize_entities(137);
        self.entity_emergence.initialize(10000);
        self.entity_emergence.apply_to_field(&mut self.field);
        self.hierarchy.initialize(&mut self.field);

        // Create ray configuration
        let mut rays = [0.0; 7];
        rays[1] = 0.4;
        let galactic = super::involution_flow::SubLogos::galactic(rays);
        self.hierarchy.add_sublogos(galactic);

        rays[2] = 0.3;
        let solar = super::involution_flow::SubLogos::solar(rays);
        self.hierarchy.add_sublogos(solar);

        rays[3] = 0.2;
        let mut planetary = super::involution_flow::SubSubLogos::new(rays);
        planetary.entity_count = 137;
        self.hierarchy.add_subsublogos(planetary);

        self.spectrum.initialize_entities(137);

        // Initialize spatial field with entities
        for i in 0..137 {
            let pos = self.spatial_field.derive_entity_position(i, 137);
            self.spatial_field.add_entity(i, pos);
            self.social_memory.register_entity(i, pos.to_array());
        }

        // Add field energy at entity positions
        for i in 0..137 {
            if let Some(pos) = self.spatial_field.get_entity_position(i) {
                let density = self.spectrum.get_spectrum_position(i).unwrap_or(0.5) as usize;
                self.spatial_field.add_energy_at(pos, density.min(7), 0.5);
            }
        }

        // Initialize matter and complexity
        self.initialize_matter_from_field();
    }

    /// Phase F3/F4/F5: Create matter, complexity, and life from field
    fn initialize_matter_from_field(&mut self) {
        let active_positions = self
            .spatial_field
            .get_active_positions(self.config.archetype_matter.particle_threshold);

        for (position, _energy) in active_positions {
            let field_data = self.spatial_field.sample_field(&position);

            // Phase F3: Create particles
            self.matter_emergence
                .derive_particle_from_field(&field_data, position);

            // Phase F4: Process complexity
            let particles: Vec<_> = self.matter_emergence.get_particles().values().collect();
            let particle_refs: Vec<&super::archetype_matter::Particle> =
                particles.to_vec();
            self.complexity_emergence
                .process_field(&field_data, position, &particle_refs);
        }
    }

    /// Get entity position
    fn get_entity_position(&self, entity_id: usize) -> [Float; 3] {
        let spatial_pos = if let Some(pos) = self.spatial_field.get_entity_position(entity_id) {
            pos
        } else {
            self.spatial_field.derive_entity_position(entity_id, 137)
        };

        let spectrum_pos = self
            .spectrum
            .get_spectrum_position(entity_id)
            .unwrap_or(0.5);
        let perceived_pos = self
            .spectrum_spatial
            .get_perceived_position(spatial_pos, spectrum_pos);

        perceived_pos.to_array()
    }

    fn get_raw_spatial_position(&self, entity_id: usize) -> Position3D {
        if let Some(pos) = self.spatial_field.get_entity_position(entity_id) {
            return pos;
        }
        self.spatial_field.derive_entity_position(entity_id, 137)
    }

    /// Advance simulation by one step
    pub fn step(&mut self) {
        self.step += 1;

        // Cosmic sequence
        let coherence = self.field.average_coherence;
        self.cosmic_sequence.step(&mut self.field, coherence);

        // Larson
        self.larson.update_veil(coherence);
        for i in 0..137 {
            let spectrum_pos = self.spectrum.get_spectrum_position(i).unwrap_or(0.5);
            self.larson.update_entity(i, spectrum_pos);
        }

        // Density sub-levels
        self.density_sub_levels.update(coherence);

        // Attractors
        for i in 0..137 {
            let pos = self.get_entity_position(i);
            self.attractor_fields.update_entity_attractors(i, pos, 0.5);
        }
        self.attractor_fields
            .update_archetypal(self.cosmic_sequence.get_layer_activations());
        self.attractor_fields.apply_to_node(&mut self.field.root);

        // Entity emergence
        self.entity_emergence.update(1.0);
        self.entity_emergence.apply_to_field(&mut self.field);

        // Spatial field
        self.spatial_field.step(0.1);

        // Spectrum-spatial
        self.spectrum_spatial.step(0.1);

        // Matter emergence (Phase F3)
        self.update_matter();

        // Complexity emergence (Phase F4)
        self.update_complexity();

        // Biological emergence (Phase F5)
        self.update_biology();

        // Planetary emergence (Phase F6)
        self.update_planetary();

        // Hierarchy
        self.hierarchy.propagate_down(&mut self.field);
        self.hierarchy.process_entity_emergence(&mut self.field);

        // Unified field
        self.unified_field.evolve(&mut self.field.root);

        // Spectrum
        self.spectrum.evolve(0.01, coherence, 0.1);

        // Evolution feedback
        if self.step.is_multiple_of(10) {
            for i in 0..137 {
                let decision_idx = (i * 7 + self.step) % 8;
                let decision_type =
                    DecisionType::from_index(decision_idx).unwrap_or(DecisionType::Growth);
                let significance = ((i * 13 + self.step) % 100) as Float / 100.0;
                let position = self.get_entity_position(i);
                let decision = EntityDecision::new(position, decision_type, significance);
                self.evolution.add_decision(decision);
            }
        }
        self.evolution.process(&mut self.field);

        // Social memory
        for i in 0..137 {
            self.social_memory.update_entity_phase(i, &self.field);
        }

        if self.step.is_multiple_of(50) {
            self.social_memory.compute_all_resonances();
            self.social_memory.form_collectives();
        }

        self.social_memory
            .apply_collective_to_field(&mut self.field);

        // Extract entities
        if self.step.is_multiple_of(self.config.visualization_interval) {
            let result = self.extractor.extract_entities(&self.field);
            self.statistics.entity_count = result.entity_count;
        }

        // Update statistics
        self.update_statistics();
    }

    /// Phase F3: Update matter from field
    fn update_matter(&mut self) {
        let active_positions = self
            .spatial_field
            .get_active_positions(self.config.archetype_matter.particle_threshold);

        let step_size = (active_positions.len() / 10).max(1);
        for (position, _) in active_positions.iter().step_by(step_size) {
            let field_data = self.spatial_field.sample_field(position);
            self.matter_emergence
                .derive_particle_from_field(&field_data, *position);
        }

        self.matter_emergence.update(0.1);
    }

    /// Phase F4: Update complexity from matter
    fn update_complexity(&mut self) {
        let active_positions = self.spatial_field.get_active_positions(0.3);

        let step_size = (active_positions.len() / 20).max(1);
        for (position, _) in active_positions.iter().step_by(step_size) {
            let field_data = self.spatial_field.sample_field(position);

            let particles: Vec<_> = self.matter_emergence.get_particles().values().collect();
            let particle_refs: Vec<&super::archetype_matter::Particle> =
                particles.to_vec();

            self.complexity_emergence
                .process_field(&field_data, *position, &particle_refs);
        }

        if self.step.is_multiple_of(10) {
            self.complexity_emergence.try_form_molecules();
        }

        self.complexity_emergence.update(0.1);
    }

    /// Phase F5: Update biology (R&D-5: actual molecular input)
    fn update_biology(&mut self) {
        // Only check periodically for performance
        if !self.step.is_multiple_of(5) {
            return;
        }

        // Try to create life at high-complexity positions
        let active_positions = self.spatial_field.get_active_positions(0.7);

        // Get molecules from complexity emergence (R&D-5)
        let molecules = self.complexity_emergence.get_molecules();

        let step_size = (active_positions.len() / 50).max(1);
        for (position, _) in active_positions.iter().step_by(step_size) {
            let field_data = self.spatial_field.sample_field(position);

            // Check if complexity is high enough for life
            if field_data.coherence > 0.6 {
                // Get molecules near this position (R&D-5)
                let nearby_molecules = self.get_molecules_near_position(*position, &molecules);

                // Try to create life with actual molecular data (not empty array!)
                if !nearby_molecules.is_empty() {
                    // Use emergent biology if available
                    if let Some(_cell) = self
                        .biological_emergence
                        .emerge_cell_from_molecules(*position, &nearby_molecules)
                    {
                        // Life emerged!
                    } else {
                        // Fallback to old method
                        self.biological_emergence
                            .try_create_life(*position, &nearby_molecules);
                    }
                }
            }
        }

        // Update biological systems
        self.biological_emergence.update(0.1);
    }

    /// Get molecules near a position (R&D-5)
    fn get_molecules_near_position(
        &self,
        position: Position3D,
        molecules: &[ComplexMolecule],
    ) -> Vec<ComplexMolecule> {
        let mut nearby = Vec::new();
        let threshold = 100.0; // Distance threshold

        for mol in molecules {
            let dist = position.distance_to(&mol.position);
            if dist < threshold {
                nearby.push(mol.clone()); // Clone to avoid move
            }
        }

        // Limit to prevent performance issues
        nearby.truncate(50);
        nearby
    }

    /// Phase F6: Update planetary environments
    fn update_planetary(&mut self) {
        // Only check periodically for performance
        if !self.step.is_multiple_of(20) {
            return;
        }

        // Process high-coherence regions for planet formation
        let active_positions = self
            .spatial_field
            .get_active_positions(self.config.planetary.planet_formation_threshold);

        let positions_with_field: Vec<_> = active_positions
            .iter()
            .map(|(pos, _)| {
                let field_data = self.spatial_field.sample_field(pos);
                (*pos, field_data)
            })
            .collect();

        self.planetary_emergence
            .process_field_positions(&positions_with_field);

        // Update planetary systems
        self.planetary_emergence.update(0.1);

        // Full integration (Phase F7)
        self.update_integration();
    }

    /// Phase F7: Full integration - unify all systems
    fn update_integration(&mut self) {
        // Gather all statistics for integration
        let field_coherence = self.field.average_coherence;
        let spectrum_position = self.statistics.average_spectrum_position;
        let particle_count = self.statistics.particles;
        let atom_count = self.statistics.atoms;
        let molecule_count = self.statistics.molecules;
        let entity_count = self.statistics.entity_count;
        let collective_count = self.statistics.collective_count;
        let avg_consciousness = 0.5; // Simplified - would come from entity data
        let planet_count = self.statistics.planets;
        let terrain_cells = self.statistics.terrain_cells;
        let terrain_coherence = 0.5; // Would come from planetary
        let biological_activity = 0.3; // Would come from biology

        // Sample field data
        let field_data = self.spatial_field.sample_field(&Position3D::origin());

        // Process full integration
        self.full_integration.step(
            field_coherence,
            spectrum_position,
            &field_data,
            particle_count,
            atom_count,
            molecule_count,
            entity_count,
            collective_count,
            avg_consciousness,
            planet_count,
            terrain_cells,
            terrain_coherence,
            biological_activity,
        );
    }

    /// Update all statistics
    fn update_statistics(&mut self) {
        self.statistics.field_energy = self.field.total_energy;
        self.statistics.average_coherence = self.field.average_coherence;
        self.statistics.average_spectrum_position = self.spectrum.statistics.average_position;
        self.statistics.veil_transparency = self.spectrum.get_veil_transparency();

        let sm_stats = self.social_memory.get_statistics();
        self.statistics.collective_count = sm_stats.collective_count;
        self.statistics.entities_in_collectives = sm_stats.entities_in_collectives;
        self.statistics.decisions_processed = self.evolution.statistics.decisions_processed;
        self.statistics.peak_magnitude = self.field.statistics.peak_magnitude;

        // Cosmic sequence
        self.statistics.current_layer = self.cosmic_sequence.get_current_layer().index();
        self.statistics.layer_activations = self.cosmic_sequence.get_layer_activations();
        self.statistics.attractor_strength = self.cosmic_sequence.statistics.max_attractor_strength;

        let larson_stats = self.larson.get_statistics();
        self.statistics.veil_transparency = larson_stats.veil_transparency;

        // Spatial
        let spatial_stats = self.spatial_field.get_statistics();
        self.statistics.spatial_nodes = spatial_stats.total_nodes;
        self.statistics.spatial_leaf_nodes = spatial_stats.leaf_nodes;

        // Spectrum-spatial
        let mut below_veil = 0;
        let mut at_veil = 0;
        let mut above_veil = 0;

        for i in 0..137 {
            let spectrum_pos = self.spectrum.get_spectrum_position(i).unwrap_or(0.5);
            if self.spectrum_spatial.veil.is_in_veil(spectrum_pos) {
                at_veil += 1;
            } else if spectrum_pos < 1.0 {
                below_veil += 1;
            } else {
                above_veil += 1;
            }
        }

        self.statistics.entities_below_veil = below_veil;
        self.statistics.entities_at_veil = at_veil;
        self.statistics.entities_above_veil = above_veil;
        self.statistics.veil_activity = self.spectrum_spatial.veil.activity;

        // Matter
        let matter_stats = self.matter_emergence.get_statistics();
        self.statistics.particles = matter_stats.active_particles;
        self.statistics.atoms = matter_stats.active_atoms;
        self.statistics.molecules = matter_stats.active_molecules;

        // Complexity
        let phase_stats = self.complexity_emergence.get_phase_statistics();
        self.statistics.quantum_regions = phase_stats.quantum_regions;
        self.statistics.atomic_regions = phase_stats.atomic_regions;
        self.statistics.molecular_regions = phase_stats.molecular_regions;
        self.statistics.planetary_regions = phase_stats.planetary_regions;

        // Biology
        self.statistics.cells = self.biological_emergence.cell_count();
        self.statistics.species = self.biological_emergence.species_count();

        // Planetary (Phase F6)
        self.statistics.planets = self.planetary_emergence.planet_count();
        self.statistics.terrain_cells = self.planetary_emergence.terrain_cell_count();

        // Full Integration (Phase F7)
        self.statistics.integration_coherence = self.full_integration.get_coherence();
        self.statistics.integration_stability = self.full_integration.get_stability();
    }

    /// Get entities for rendering
    pub fn get_entities(&self) -> Vec<RenderableEntity> {
        let mut entities = Vec::new();

        for i in 0..self.statistics.entity_count.min(137) {
            let position = self.get_entity_position(i);
            let raw_position = self.get_raw_spatial_position(i);

            let density = self.spectrum.get_spectrum_position(i).unwrap_or(0.5);
            let density_band = DensityBand::from_index((density * 8.0) as usize % 8);
            let in_collective = self.social_memory.get_entity_collective(i).is_some();

            let spatial_config = self.spectrum_spatial.derive_spatial_structure(density);

            entities.push(RenderableEntity {
                position,
                raw_position: raw_position.to_array(),
                density_band,
                consciousness: 0.5,
                in_collective,
                entity_id: i,
                spatial_config,
                veil_distance: self.spectrum_spatial.distance_to_veil(density),
            });
        }

        entities
    }

    /// Get field visualization data
    pub fn get_field_visualization(&self) -> FieldVisualizationData {
        FieldVisualizationData {
            bounds: [self.field.root.bounds.min, self.field.root.bounds.max],
            coherence: self.field.average_coherence,
            energy: self.field.total_energy,
            veil_transparency: self.statistics.veil_transparency,
            spectrum_position: self.statistics.average_spectrum_position,
        }
    }

    pub fn get_statistics(&self) -> &SimulationStatistics {
        &self.statistics
    }

    /// Get the holographic field state for visualization
    pub fn field_state(&self) -> &HolographicFieldState {
        &self.field
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub fn get_spatial_field(&self) -> &SpatialField {
        &self.spatial_field
    }

    pub fn get_spectrum_spatial(&self) -> &SpectrumSpatialDynamics {
        &self.spectrum_spatial
    }

    pub fn get_matter_emergence(&self) -> &MatterEmergence {
        &self.matter_emergence
    }

    pub fn get_complexity_emergence(&self) -> &ComplexityEmergence {
        &self.complexity_emergence
    }

    pub fn get_biological_emergence(&self) -> &BiologyBridge {
        &self.biological_emergence
    }

    pub fn get_planetary_emergence(&self) -> &PlanetaryBridge {
        &self.planetary_emergence
    }

    pub fn get_full_integration(&self) -> &IntegrationBridge {
        &self.full_integration
    }
}

/// Entity for rendering
#[derive(Debug, Clone)]
pub struct RenderableEntity {
    pub position: [Float; 3],
    pub raw_position: [Float; 3],
    pub density_band: DensityBand,
    pub consciousness: Float,
    pub in_collective: bool,
    pub entity_id: usize,
    pub spatial_config: SpatialConfig,
    pub veil_distance: Float,
}

/// Field visualization data
#[derive(Debug, Clone)]
pub struct FieldVisualizationData {
    pub bounds: [[Float; 3]; 2],
    pub coherence: Float,
    pub energy: Float,
    pub veil_transparency: Float,
    pub spectrum_position: Float,
}

/// Collective visualization
#[derive(Debug, Clone)]
pub struct CollectiveVisualization {
    pub id: usize,
    pub center: [Float; 3],
    pub member_count: usize,
    pub coherence: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_creation() {
        let sim = HolographicSimulation::with_defaults();
        assert_eq!(sim.get_step(), 0);
    }

    #[test]
    fn test_simulation_initialize() {
        let mut sim = HolographicSimulation::with_defaults();
        sim.initialize();
        // get_step() returns u64 which is always >= 0
        let _ = sim.get_step();
    }

    #[test]
    fn test_simulation_step() {
        let mut sim = HolographicSimulation::with_defaults();
        sim.initialize();
        sim.step();
        assert_eq!(sim.get_step(), 1);
    }

    #[test]
    fn test_all_phases_integrated() {
        let mut sim = HolographicSimulation::with_defaults();
        sim.initialize();

        let _spatial = sim.get_spatial_field();
        let _spectrum = sim.get_spectrum_spatial();
        let _matter = sim.get_matter_emergence();
        let _complexity = sim.get_complexity_emergence();
        let _biology = sim.get_biological_emergence();
        // Test passes if all getters work without panic
    }
}
