// Simulation Statistics Module (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md:
// "Implement statistics tracking system for NEW simulation"
//
// This module implements:
// 1. SimulationStatistics - comprehensive statistics tracking
// 2. StatisticsTracker - manager for collecting and updating statistics
// 3. Architecture metrics - validation of architecture alignment
// 4. Performance metrics - performance tracking

use crate::physical_manifestation::consciousness_to_matter::ConsciousnessToMatterStatistics;
use crate::simulation_v3::individual_variation_statistics::IndividualVariationStatistics;
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Type alias for spectrum accumulator data used in statistics calculations
/// (spectrum_ratios, space_time_access, time_space_access, count)
type SpectrumAccumulator = (Vec<Float>, Vec<Float>, Vec<Float>, usize);

// ============================================================================
// SIMULATION STATISTICS
// ============================================================================

/// Comprehensive simulation statistics
///
/// Tracks all aspects of the simulation including involution, evolution,
/// holographic field, and physical manifestation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulationStatistics {
    /// Involution phase statistics
    pub involution: InvolutionStatistics,

    /// Evolution phase statistics
    pub evolution: EvolutionStatistics,

    /// Holographic field statistics
    pub holographic: HolographicFieldStatistics,

    /// Physical manifestation statistics
    pub physical: PhysicalStatistics,

    /// Performance metrics
    pub performance: PerformanceMetrics,

    /// Architecture alignment metrics
    pub architecture: ArchitectureMetrics,

    /// Spectrum access statistics (Phase 3)
    pub spectrum_access: SpectrumAccessStatistics,

    /// Individual variation statistics (Phase 5)
    pub individual_variation: IndividualVariationStatistics,

    /// Phase 4: Emergent properties
    pub emergent_properties: EmergentProperties,

    /// Phase 7: Consciousness-to-matter statistics
    pub consciousness_to_matter: ConsciousnessToMatterStatistics,
}

/// Involution phase statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvolutionStatistics {
    /// Number of entities created
    pub entities_created: usize,

    /// Number of attractor fields created
    pub attractor_fields_created: usize,

    /// Number of stage transitions
    pub stage_transitions: usize,

    /// Involution execution time
    pub execution_time: Duration,

    /// Stage-by-stage details
    pub stage_details: Vec<StageDetail>,
}

/// Details for each involution stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDetail {
    /// Stage number (0-7)
    pub stage_number: usize,

    /// Stage name
    pub stage_name: String,

    /// Success flag
    pub success: bool,

    /// Duration
    pub duration: Duration,

    /// Number of entities created at this stage
    pub entities_created: usize,

    /// Number of attractor fields created at this stage
    pub attractor_fields_created: usize,
}

/// Evolution phase statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStatistics {
    /// Current simulation step
    pub current_step: u64,

    /// Total number of steps
    pub total_steps: u64,

    /// Number of entities
    pub num_entities: usize,

    /// Density distribution
    pub density_distribution: HashMap<String, usize>,

    /// Polarity distribution
    pub polarization_distribution: PolarizationDistribution,

    /// Number of density transitions
    pub density_transitions: usize,

    /// Evolution execution time
    pub execution_time: Duration,

    // Phase 3: Feature Completion metrics
    /// Total energy tapped from IntelligentInfinity
    pub total_tapped_energy: Float,

    /// Average energy tap strength
    pub average_tap_strength: Float,

    /// Spectrum access level (0.0 to 1.0)
    pub spectrum_access_level: Float,

    /// Veil thickness (0.0 to 1.0, where 1.0 is full thickness)
    pub veil_thickness: Float,

    /// Veil transparency (0.0 to 1.0, where 1.0 is fully transparent)
    pub veil_transparency: Float,

    /// Attractor-field activation level (0.0 to 1.0)
    pub attractor_field_activation: Float,

    // Phase 3: Polarity Diversity Index
    /// Polarity diversity index (0.0 to 1.0)
    /// 0.0 = no diversity (all unpolarized)
    /// 1.0 = maximum diversity (equal STO/STS)
    pub polarization_diversity_index: Float,
}

/// Polarity distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolarizationDistribution {
    /// Number of STO (Service-to-Others) entities
    pub sto: usize,

    /// Number of STS (Service-to-Self) entities
    pub sts: usize,

    /// Number of unpolarized entities
    pub unpolarized: usize,

    /// Average polarity bias
    pub average_bias: Float,
}

/// Spectrum access statistics (Phase 3)
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md:
/// "Track space/time vs time/space access separately"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumAccessStatistics {
    /// Spectrum ratio distribution (ratio -> count)
    /// Ratio > 1.0: space/time dominant
    /// Ratio < 1.0: time/space dominant
    /// Ratio = 1.0: at the Veil
    pub spectrum_ratio_distribution: HashMap<String, usize>,

    /// Number of space/time dominant entities (v = s/t >> 1)
    pub space_time_dominant_count: usize,

    /// Number of time/space dominant entities (v = t/s >> 1)
    pub time_space_dominant_count: usize,

    /// Number of balanced entities (near the Veil, v ≈ 1)
    pub balanced_count: usize,

    /// Number of entities with active Veil
    pub veil_active_count: usize,

    /// Number of entities with inactive Veil
    pub veil_inactive_count: usize,

    /// Average spectrum ratio
    pub average_spectrum_ratio: Float,

    /// Average space/time access (0.0 to 1.0)
    pub average_space_time_access: Float,

    /// Average time/space access (0.0 to 1.0)
    pub average_time_space_access: Float,

    /// Spectrum access by entity type
    pub by_entity_type: HashMap<String, SpectrumAccessStats>,

    /// Spectrum access by entity scale
    pub by_scale: HashMap<String, SpectrumAccessStats>,
}

/// Spectrum access statistics for a specific group (entity type or scale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumAccessStats {
    /// Number of entities in this group
    pub count: usize,

    /// Average spectrum ratio
    pub average_ratio: Float,

    /// Number with space/time dominance
    pub space_time_dominant: usize,

    /// Number with time/space dominance
    pub time_space_dominant: usize,

    /// Number balanced
    pub balanced: usize,

    /// Number with active Veil
    pub veil_active: usize,

    /// Average space/time access
    pub average_space_time_access: Float,

    /// Average time/space access
    pub average_time_space_access: Float,
}

/// Physical manifestation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalStatistics {
    /// Number of physical entities manifested
    pub num_physical_entities: usize,

    /// Physical scale distribution
    pub scale_distribution: HashMap<String, usize>,

    /// Energy level distribution
    pub energy_level_distribution: HashMap<String, usize>,

    /// Mass distribution (scale -> mass values)
    pub mass_distribution: HashMap<String, Vec<Float>>,

    /// Charge distribution (scale -> charge values)
    pub charge_distribution: HashMap<String, Vec<Float>>,

    /// Position distribution (all coordinates)
    pub position_distribution: Vec<crate::matter::Coordinate3D>,

    /// Total energy in the system
    pub total_energy: Float,

    /// Average kinetic energy
    pub average_kinetic_energy: Float,

    /// Total mass in the system
    pub total_mass: Float,

    /// Average mass
    pub average_mass: Float,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total simulation time
    pub total_time: Duration,

    /// Time per step (average)
    pub time_per_step: Duration,

    /// Entities per second
    pub entities_per_second: Float,

    /// Steps per second
    pub steps_per_second: Float,

    /// Peak memory usage (in MB) - simplified
    pub peak_memory_mb: usize,
}

/// Holographic field statistics
///
/// Simplified version for statistics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolographicFieldStatistics {
    /// Total number of entities
    pub entity_count: usize,

    /// Total number of connections
    pub connection_count: usize,

    /// Average connection strength
    pub average_connection_strength: Float,

    /// Number of resonant connections
    pub resonant_connections: usize,

    /// Phase 4: Number of harmonic connections
    pub harmonic_connections: usize,

    /// Number of entangled connections
    pub entangled_connections: usize,

    /// Phase 4: Number of antiphase connections
    pub antiphase_connections: usize,

    /// Phase 4: Number of weak connections
    pub weak_connections: usize,

    /// Phase 4: Average archetype similarity
    pub average_archetype_similarity: Float,

    /// Global phase coherence
    pub global_phase_coherence: Float,

    /// Number of interference patterns
    pub interference_pattern_count: usize,

    /// Number of resonance clusters
    pub resonance_cluster_count: usize,

    /// Phase 4: Holographic interaction score
    pub holographic_interaction_score: Float,
}

impl Default for HolographicFieldStatistics {
    fn default() -> Self {
        HolographicFieldStatistics {
            entity_count: 0,
            connection_count: 0,
            average_connection_strength: 0.0,
            resonant_connections: 0,
            harmonic_connections: 0,
            entangled_connections: 0,
            antiphase_connections: 0,
            weak_connections: 0,
            average_archetype_similarity: 0.0,
            global_phase_coherence: 0.0,
            interference_pattern_count: 0,
            resonance_cluster_count: 0,
            holographic_interaction_score: 0.0,
        }
    }
}

/// Architecture alignment metrics
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md:
/// "Architecture alignment is PRIMARY goal"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureMetrics {
    /// Three Primal Distortions correctly implemented
    pub three_primal_distortions: bool,

    /// Number of "transcend and include" stages implemented
    pub transcend_include_stages: usize,

    /// Total transcend and include stages expected
    pub transcend_include_stages_total: usize,

    /// Space/Time spectrum correctly modeled
    pub space_time_spectrum: bool,

    /// Veil correctly positioned at v=1
    pub veil_position: bool,

    /// Logos Hierarchy correctly structured
    pub logos_hierarchy: bool,

    /// Density Octave correctly implemented
    pub density_octave: bool,

    /// Holographic principle demonstrated
    pub holographic_principle: bool,

    /// Phase 4: Environmental interaction implemented
    pub environmental_interaction: bool,

    /// Phase 4: Collective influence implemented
    pub collective_influence: bool,

    /// Phase 4: Emergent properties detected
    pub emergent_properties: bool,

    /// Phase 3: Free Will / Polarity diversity detected
    pub polarization_diversity: bool,

    /// Phase 7: Consciousness-to-matter transitions implemented
    pub consciousness_to_matter_transitions: bool,

    /// Phase 7: Quantum energy pools created
    pub quantum_pools: bool,

    /// Overall alignment score (0.0 to 1.0)
    pub alignment_score: Float,

    /// Alignment percentage (0 to 100)
    pub alignment_percentage: f64,
}

// ============================================================================
// STATISTICS TRACKER
// ============================================================================

/// Statistics Tracker
///
/// Manager for collecting and updating simulation statistics.
#[derive(Debug, Clone)]
pub struct StatisticsTracker {
    /// Current statistics
    pub statistics: SimulationStatistics,

    /// Start time
    pub start_time: std::time::Instant,

    /// Step times (for calculating averages)
    pub step_times: Vec<Duration>,
}

impl Default for StatisticsTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticsTracker {
    /// Create a new statistics tracker
    pub fn new() -> Self {
        StatisticsTracker {
            statistics: SimulationStatistics::default(),
            start_time: std::time::Instant::now(),
            step_times: Vec::new(),
        }
    }

    /// Record involution phase completion
    pub fn record_involution_complete(
        &mut self,
        entities_created: usize,
        attractor_fields_created: usize,
        stage_transitions: usize,
        execution_time: Duration,
        stage_details: Vec<StageDetail>,
    ) {
        self.statistics.involution = InvolutionStatistics {
            entities_created,
            attractor_fields_created,
            stage_transitions,
            execution_time,
            stage_details,
        };

        // Update architecture metrics based on involution
        self.update_architecture_metrics();
    }

    /// Record evolution step
    #[allow(clippy::too_many_arguments)]
    pub fn record_evolution_step(
        &mut self,
        step: u64,
        num_entities: usize,
        density_distribution: HashMap<String, usize>,
        polarization_distribution: PolarizationDistribution,
        density_transitions: usize,
        // Phase 3: Feature Completion metrics
        total_tapped_energy: Float,
        average_tap_strength: Float,
        spectrum_access_level: Float,
        veil_thickness: Float,
        veil_transparency: Float,
        attractor_field_activation: Float,
    ) {
        self.statistics.evolution.current_step = step;
        self.statistics.evolution.num_entities = num_entities;
        self.statistics.evolution.density_distribution = density_distribution;
        self.statistics.evolution.polarization_distribution = polarization_distribution;
        self.statistics.evolution.density_transitions = density_transitions;

        // Phase 3: Record feature completion metrics
        self.statistics.evolution.total_tapped_energy = total_tapped_energy;
        self.statistics.evolution.average_tap_strength = average_tap_strength;
        self.statistics.evolution.spectrum_access_level = spectrum_access_level;
        self.statistics.evolution.veil_thickness = veil_thickness;
        self.statistics.evolution.veil_transparency = veil_transparency;
        self.statistics.evolution.attractor_field_activation = attractor_field_activation;

        // Record step time
        let _step_time = self.step_times.last().copied().unwrap_or(Duration::ZERO);
        // Note: This is simplified - in practice, we'd record actual step times
    }

    /// Record evolution phase completion
    pub fn record_evolution_complete(&mut self, execution_time: Duration) {
        self.statistics.evolution.execution_time = execution_time;
    }

    /// Record holographic field statistics
    pub fn record_holographic_statistics(&mut self, holographic_stats: HolographicFieldStatistics) {
        self.statistics.holographic = holographic_stats;
    }

    /// Record physical manifestation statistics
    pub fn record_physical_statistics(&mut self, physical_stats: PhysicalStatistics) {
        self.statistics.physical = physical_stats;
    }

    /// Record physical scale data from entities (Phase 4)
    ///
    /// Collects physical properties from entities and updates statistics.
    pub fn record_physical_scale_data(&mut self, entities: &[crate::entity_layer7::SubSubLogos]) {
        let mut num_physical = 0;
        let mut scale_distribution: HashMap<String, usize> = HashMap::new();
        let mut mass_distribution: HashMap<String, Vec<Float>> = HashMap::new();
        let mut charge_distribution: HashMap<String, Vec<Float>> = HashMap::new();
        let mut position_distribution: Vec<crate::matter::Coordinate3D> = Vec::new();
        let mut total_energy = 0.0 as Float;
        let mut total_mass = 0.0 as Float;

        for entity in entities {
            // Always add to scale distribution (regardless of whether physical entity exists)
            // Scale is now derived from current density
            let scale_name = format!("{:?}", entity.evolutionary_attractor.current_density);
            *scale_distribution.entry(scale_name.clone()).or_insert(0) += 1;

            if let Some(physical) = &entity.physical_entity {
                num_physical += 1;

                // Extract physical properties based on type
                match physical {
                    crate::matter::Matter::Particle(p) => {
                        mass_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(p.mass);
                        charge_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(p.charge);
                        position_distribution.push(p.position);
                        total_energy += p.total_energy();
                        total_mass += p.mass;
                    }
                    crate::matter::Matter::Atom(a) => {
                        let mass = a.atomic_mass();
                        mass_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(mass);
                        charge_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(a.atomic_number as Float);
                        position_distribution.push(a.position);
                        total_energy += mass * 2.998e8 * 2.998e8; // E = mc²
                        total_mass += mass;
                    }
                    crate::matter::Matter::Molecule(m) => {
                        let mut molecule_mass = 0.0;
                        for atom in &m.atoms {
                            molecule_mass += atom.atomic_mass();
                            position_distribution.push(atom.position);
                        }
                        mass_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(molecule_mass);
                        total_energy += molecule_mass * 2.998e8 * 2.998e8;
                        total_mass += molecule_mass;
                    }
                    crate::matter::Matter::Cell(_) => {
                        // Cells have complex mass calculation - simplified
                        let cell_mass = 1.0e-12; // Typical cell mass
                        mass_distribution
                            .entry(scale_name.clone())
                            .or_default()
                            .push(cell_mass);
                        total_energy += cell_mass * 2.998e8 * 2.998e8;
                        total_mass += cell_mass;
                    }
                    crate::matter::Matter::Empty => {}
                }
            }
        }

        let average_mass = if num_physical > 0 {
            total_mass / num_physical as Float
        } else {
            0.0
        };

        let average_kinetic_energy = 0.0; // Simplified - would calculate from velocities

        self.statistics.physical = PhysicalStatistics {
            num_physical_entities: num_physical,
            scale_distribution,
            energy_level_distribution: HashMap::new(), // Would calculate from energy levels
            mass_distribution,
            charge_distribution,
            position_distribution,
            total_energy,
            average_kinetic_energy,
            total_mass,
            average_mass,
        };
    }

    /// Update performance metrics
    pub fn update_performance_metrics(&mut self) {
        let total_time = self.start_time.elapsed();
        self.statistics.performance.total_time = total_time;

        // Calculate time per step
        if self.statistics.evolution.total_steps > 0 {
            self.statistics.performance.time_per_step =
                total_time / self.statistics.evolution.total_steps as u32;
        }

        // Calculate steps per second
        if total_time.as_secs_f64() > 0.0 {
            self.statistics.performance.steps_per_second =
                self.statistics.evolution.total_steps as Float / total_time.as_secs_f64();
        }

        // Calculate entities per second
        if total_time.as_secs_f64() > 0.0 {
            self.statistics.performance.entities_per_second =
                self.statistics.evolution.num_entities as Float / total_time.as_secs_f64();
        }
    }

    /// Update architecture alignment metrics
    pub fn update_architecture_metrics(&mut self) {
        // Check Three Primal Distortions
        let three_primal_distortions = self.statistics.involution.stage_details.len() >= 4;

        // Check "transcend and include" stages
        let transcend_include_stages = self.statistics.involution.stage_details.len();
        let transcend_include_stages_total = 8;

        // Check Space/Time spectrum
        let space_time_spectrum = !self
            .statistics
            .spectrum_access
            .spectrum_ratio_distribution
            .is_empty();

        // Check Veil position
        let veil_position = self.statistics.spectrum_access.veil_active_count > 0
            || self.statistics.spectrum_access.veil_inactive_count > 0;

        // Check Logos Hierarchy
        let logos_hierarchy = self.statistics.involution.attractor_fields_created > 0;

        // Check Density Octave
        let density_octave = !self.statistics.evolution.density_distribution.is_empty()
            && self.statistics.evolution.density_distribution.len() > 1;

        // Check Holographic principle
        let holographic_principle = self.statistics.holographic.connection_count > 0
            && self.statistics.holographic.global_phase_coherence > 0.0;

        // Phase 4: Check Environmental interaction
        let environmental_interaction = self
            .statistics
            .emergent_properties
            .environmental_integration
            > 0.0;

        // Phase 4: Check Collective influence
        let collective_influence = self
            .statistics
            .emergent_properties
            .collective_consciousness_level
            > 0.0;

        // Phase 4: Check Emergent properties
        let emergent_properties = self.statistics.emergent_properties.emergence_score > 0.3;

        // Phase 3: Check Free Will / Polarity
        // Check if there are polarized entities (STO or STS)
        let total_polarized = self.statistics.evolution.polarization_distribution.sto
            + self.statistics.evolution.polarization_distribution.sts;
        let total_entities = total_polarized
            + self
                .statistics
                .evolution
                .polarization_distribution
                .unpolarized;
        let polarization_diversity = total_entities > 0 && total_polarized > 0;

        // Phase 7: Check Quantum pools
        let quantum_pools = self.statistics.consciousness_to_matter.total_pools > 0;

        // Phase 7: Check Consciousness-to-matter transitions
        let consciousness_to_matter_transitions =
            self.statistics.consciousness_to_matter.active_transitions > 0
                || self
                    .statistics
                    .consciousness_to_matter
                    .completed_transitions
                    > 0;

        // Calculate alignment score (Phase 3: Now includes Phase 3 features)
        let total_checks = 13;
        let passed_checks = [
            three_primal_distortions,
            space_time_spectrum,
            veil_position,
            logos_hierarchy,
            density_octave,
            holographic_principle,
            environmental_interaction,
            collective_influence,
            emergent_properties,
            polarization_diversity, // Phase 3
            quantum_pools,
            consciousness_to_matter_transitions,
        ]
        .iter()
        .filter(|&&x| x)
        .count() as Float;

        // Add bonus for high emergence score
        let emergence_bonus = if self.statistics.emergent_properties.emergence_score > 0.5 {
            0.05
        } else {
            0.0
        };

        // Add bonus for high transition success rate (Phase 7)
        let transition_bonus = if self.statistics.consciousness_to_matter.total_pools > 0 {
            let success_rate = self
                .statistics
                .consciousness_to_matter
                .completed_transitions as Float
                / self.statistics.consciousness_to_matter.total_pools as Float;
            if success_rate > 0.5 {
                0.05
            } else {
                0.0
            }
        } else {
            0.0
        };

        let alignment_score =
            ((passed_checks / total_checks as Float) + emergence_bonus + transition_bonus)
                .clamp(0.0, 1.0);

        let alignment_percentage = alignment_score * 100.0;

        let metrics = ArchitectureMetrics {
            three_primal_distortions,
            transcend_include_stages,
            transcend_include_stages_total,
            space_time_spectrum,
            veil_position,
            logos_hierarchy,
            density_octave,
            holographic_principle,
            environmental_interaction,
            collective_influence,
            emergent_properties,
            polarization_diversity,
            quantum_pools,
            consciousness_to_matter_transitions,
            alignment_score,
            alignment_percentage,
        };

        self.statistics.architecture = metrics;
    }

    /// Record spectrum access data from entities (Phase 3)
    pub fn record_spectrum_access_data(
        &mut self,
        spectrum_ratios: Vec<Float>,
        space_time_access: Vec<Float>,
        time_space_access: Vec<Float>,
        veil_active: Vec<bool>,
        entity_types: Vec<String>,
        scales: Vec<String>,
    ) {
        let num_entities = spectrum_ratios.len();

        if num_entities == 0 {
            return;
        }

        // Calculate overall statistics
        let total_ratio: Float = spectrum_ratios.iter().sum();
        let total_space_time: Float = space_time_access.iter().sum();
        let total_time_space: Float = time_space_access.iter().sum();

        self.statistics.spectrum_access.average_spectrum_ratio =
            total_ratio / num_entities as Float;
        self.statistics.spectrum_access.average_space_time_access =
            total_space_time / num_entities as Float;
        self.statistics.spectrum_access.average_time_space_access =
            total_time_space / num_entities as Float;

        // Count space/time dominant, time/space dominant, and balanced entities
        for ratio in &spectrum_ratios {
            if ratio > &1.5 {
                self.statistics.spectrum_access.space_time_dominant_count += 1;
            } else if ratio < &0.67 {
                self.statistics.spectrum_access.time_space_dominant_count += 1;
            } else {
                self.statistics.spectrum_access.balanced_count += 1;
            }
        }

        // Count veil activity
        for active in &veil_active {
            if *active {
                self.statistics.spectrum_access.veil_active_count += 1;
            } else {
                self.statistics.spectrum_access.veil_inactive_count += 1;
            }
        }

        // Build spectrum ratio distribution histogram
        for ratio in &spectrum_ratios {
            let bucket = if ratio > &100.0 {
                "100+".to_string()
            } else if ratio > &50.0 {
                "50-100".to_string()
            } else if ratio > &20.0 {
                "20-50".to_string()
            } else if ratio > &10.0 {
                "10-20".to_string()
            } else if ratio > &5.0 {
                "5-10".to_string()
            } else if ratio > &2.0 {
                "2-5".to_string()
            } else if ratio > &1.5 {
                "1.5-2".to_string()
            } else if ratio > &1.0 {
                "1.0-1.5".to_string()
            } else if ratio > &0.67 {
                "0.67-1.0".to_string()
            } else if ratio > &0.5 {
                "0.5-0.67".to_string()
            } else if ratio > &0.1 {
                "0.1-0.5".to_string()
            } else if ratio > &0.01 {
                "0.01-0.1".to_string()
            } else {
                "0-0.01".to_string()
            };

            *self
                .statistics
                .spectrum_access
                .spectrum_ratio_distribution
                .entry(bucket)
                .or_insert(0) += 1;
        }

        // Calculate statistics by entity type
        let mut by_type: HashMap<String, SpectrumAccumulator> = HashMap::new();

        for i in 0..num_entities {
            let entity_type = entity_types[i].clone();
            let entry = by_type
                .entry(entity_type.clone())
                .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new(), 0));
            entry.0.push(spectrum_ratios[i]);
            entry.1.push(space_time_access[i]);
            entry.2.push(time_space_access[i]);
            entry.3 += 1;
        }

        for (entity_type, (ratios, st_access, ts_access, count)) in by_type {
            let total_ratio: Float = ratios.iter().sum();
            let total_st: Float = st_access.iter().sum();
            let total_ts: Float = ts_access.iter().sum();

            let mut stats = SpectrumAccessStats {
                count,
                average_ratio: total_ratio / count as Float,
                average_space_time_access: total_st / count as Float,
                average_time_space_access: total_ts / count as Float,
                ..Default::default()
            };

            // Count dominance types
            for ratio in &ratios {
                if ratio > &1.5 {
                    stats.space_time_dominant += 1;
                } else if ratio < &0.67 {
                    stats.time_space_dominant += 1;
                } else {
                    stats.balanced += 1;
                }
            }

            self.statistics
                .spectrum_access
                .by_entity_type
                .insert(entity_type, stats);
        }

        // Calculate statistics by scale
        let mut by_scale: HashMap<String, SpectrumAccumulator> = HashMap::new();

        for i in 0..num_entities {
            let scale = scales[i].clone();
            let entry = by_scale
                .entry(scale.clone())
                .or_insert_with(|| (Vec::new(), Vec::new(), Vec::new(), 0));
            entry.0.push(spectrum_ratios[i]);
            entry.1.push(space_time_access[i]);
            entry.2.push(time_space_access[i]);
            entry.3 += 1;
        }

        for (scale, (ratios, st_access, ts_access, count)) in by_scale {
            let total_ratio: Float = ratios.iter().sum();
            let total_st: Float = st_access.iter().sum();
            let total_ts: Float = ts_access.iter().sum();

            let mut stats = SpectrumAccessStats {
                count,
                average_ratio: total_ratio / count as Float,
                average_space_time_access: total_st / count as Float,
                average_time_space_access: total_ts / count as Float,
                ..Default::default()
            };

            // Count dominance types
            for ratio in &ratios {
                if ratio > &1.5 {
                    stats.space_time_dominant += 1;
                } else if ratio < &0.67 {
                    stats.time_space_dominant += 1;
                } else {
                    stats.balanced += 1;
                }
            }

            self.statistics
                .spectrum_access
                .by_scale
                .insert(scale, stats);
        }
    }

    /// Get current statistics
    pub fn get_statistics(&self) -> &SimulationStatistics {
        &self.statistics
    }

    /// Get a summary report
    pub fn get_summary(&self) -> String {
        let stats = &self.statistics;

        format!(
            "=== Simulation Statistics Summary ===\n\
             \n\
             Involution Phase:\n\
             - Entities created: {}\n\
             - Attractor fields created: {}\n\
             - Stage transitions: {}\n\
             - Execution time: {:?}\n\
             \n\
             Evolution Phase:\n\
             - Current step: {}\n\
             - Total steps: {}\n\
             - Entities: {}\n\
             - Density transitions: {}\n\
             - Execution time: {:?}\n\
             \n\
             Polarity Distribution:\n\
             - STO: {}\n\
             - STS: {}\n\
             - Unpolarized: {}\n\
             \n\
             Holographic Field:\n\
             - Connections: {}\n\
             - Global coherence: {:.4}\n\
             \n\
             Physical Manifestation:\n\
             - Physical entities: {}\n\
             - Total energy: {:.2e} J\n\
             \n\
             Performance:\n\
             - Total time: {:?}\n\
             - Steps per second: {:.2}\n\
             - Entities per second: {:.2}\n\
             \n\
             Architecture Alignment:\n\
             - Alignment score: {:.2}%\n\
             - Three Primal Distortions: {}\n\
             - Density Octave: {}\n\
             - Holographic principle: {}",
            stats.involution.entities_created,
            stats.involution.attractor_fields_created,
            stats.involution.stage_transitions,
            stats.involution.execution_time,
            stats.evolution.current_step,
            stats.evolution.total_steps,
            stats.evolution.num_entities,
            stats.evolution.density_transitions,
            stats.evolution.execution_time,
            stats.evolution.polarization_distribution.sto,
            stats.evolution.polarization_distribution.sts,
            stats.evolution.polarization_distribution.unpolarized,
            stats.holographic.connection_count,
            stats.holographic.global_phase_coherence,
            stats.physical.num_physical_entities,
            stats.physical.total_energy,
            stats.performance.total_time,
            stats.performance.steps_per_second,
            stats.performance.entities_per_second,
            stats.architecture.alignment_score,
            if stats.architecture.three_primal_distortions {
                "✓"
            } else {
                "✗"
            },
            if stats.architecture.density_octave {
                "✓"
            } else {
                "✗"
            },
            if stats.architecture.holographic_principle {
                "✓"
            } else {
                "✗"
            }
        )
    }
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS
// ============================================================================

impl Default for InvolutionStatistics {
    fn default() -> Self {
        InvolutionStatistics {
            entities_created: 0,
            attractor_fields_created: 0,
            stage_transitions: 0,
            execution_time: Duration::ZERO,
            stage_details: Vec::new(),
        }
    }
}

impl Default for EvolutionStatistics {
    fn default() -> Self {
        EvolutionStatistics {
            current_step: 0,
            total_steps: 0,
            num_entities: 0,
            density_distribution: HashMap::new(),
            polarization_distribution: PolarizationDistribution::default(),
            density_transitions: 0,
            execution_time: Duration::ZERO,
            // Phase 3: Feature Completion metrics
            total_tapped_energy: 0.0,
            average_tap_strength: 0.0,
            spectrum_access_level: 0.0,
            veil_thickness: 1.0,    // Start with full veil
            veil_transparency: 0.0, // Start with no transparency
            attractor_field_activation: 0.0,
            // Phase 3: Polarity Diversity Index
            polarization_diversity_index: 0.0,
        }
    }
}

impl Default for PolarizationDistribution {
    fn default() -> Self {
        PolarizationDistribution {
            sto: 0,
            sts: 0,
            unpolarized: 0,
            average_bias: 0.0,
        }
    }
}

impl Default for PhysicalStatistics {
    fn default() -> Self {
        PhysicalStatistics {
            num_physical_entities: 0,
            scale_distribution: HashMap::new(),
            energy_level_distribution: HashMap::new(),
            mass_distribution: HashMap::new(),
            charge_distribution: HashMap::new(),
            position_distribution: Vec::new(),
            total_energy: 0.0,
            average_kinetic_energy: 0.0,
            total_mass: 0.0,
            average_mass: 0.0,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            total_time: Duration::ZERO,
            time_per_step: Duration::ZERO,
            entities_per_second: 0.0,
            steps_per_second: 0.0,
            peak_memory_mb: 0,
        }
    }
}

impl Default for ArchitectureMetrics {
    fn default() -> Self {
        ArchitectureMetrics {
            three_primal_distortions: false,
            transcend_include_stages: 0,
            transcend_include_stages_total: 8,
            space_time_spectrum: false,
            veil_position: false,
            logos_hierarchy: false,
            density_octave: false,
            holographic_principle: false,
            environmental_interaction: false,
            collective_influence: false,
            emergent_properties: false,
            polarization_diversity: false, // Phase 3
            consciousness_to_matter_transitions: false,
            quantum_pools: false,
            alignment_score: 0.0,
            alignment_percentage: 0.0,
        }
    }
}

impl Default for SpectrumAccessStatistics {
    fn default() -> Self {
        SpectrumAccessStatistics {
            spectrum_ratio_distribution: HashMap::new(),
            space_time_dominant_count: 0,
            time_space_dominant_count: 0,
            balanced_count: 0,
            veil_active_count: 0,
            veil_inactive_count: 0,
            average_spectrum_ratio: 0.0,
            average_space_time_access: 0.0,
            average_time_space_access: 0.0,
            by_entity_type: HashMap::new(),
            by_scale: HashMap::new(),
        }
    }
}

impl Default for SpectrumAccessStats {
    fn default() -> Self {
        SpectrumAccessStats {
            count: 0,
            average_ratio: 0.0,
            space_time_dominant: 0,
            time_space_dominant: 0,
            balanced: 0,
            veil_active: 0,
            average_space_time_access: 0.0,
            average_time_space_access: 0.0,
        }
    }
}

// ============================================================================
// EMERGENT PROPERTIES (Phase 4)
// ============================================================================

/// Emergent Properties
///
/// Properties that emerge from the whole system that are not present
/// in individual components. These are true emergent phenomena.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentProperties {
    /// Global coherence (0.0 to 1.0)
    /// Measures the overall coherence of the system
    pub global_coherence: Float,

    /// Collective consciousness level (0.0 to 1.0)
    /// Measures the level of collective consciousness across all entities
    pub collective_consciousness_level: Float,

    /// System entropy (0.0 to 1.0)
    /// Measures the disorder/chaos in the system
    pub system_entropy: Float,

    /// Emergence score (0.0 to 1.0)
    /// Overall measure of emergent behavior in the system
    pub emergence_score: Float,

    /// Organic emergence score (0.0 to 1.0)
    /// Measures how organic/natural the emergence is
    pub organic_emergence_score: Float,

    /// Unique evolutionary trajectories count
    /// Number of unique evolutionary paths in the system
    pub unique_evolutionary_trajectories: usize,

    /// Holographic coherence (0.0 to 1.0)
    /// Measures how well the holographic principle is demonstrated
    pub holographic_coherence: Float,

    /// Environmental integration (0.0 to 1.0)
    /// Measures how well entities are integrated with the environment
    pub environmental_integration: Float,

    /// Phase 4: Holographic interaction diversity (0.0 to 1.0)
    /// Measures the diversity of holographic connection types
    pub holographic_interaction_diversity: Float,

    /// Phase 4: Collective-individual co-evolution (0.0 to 1.0)
    /// Measures how well collectives and individuals co-evolve
    pub collective_individual_coevolution: Float,

    /// Phase 4: Environmental resonance (0.0 to 1.0)
    /// Measures how well entities resonate with their environment
    pub environmental_resonance: Float,

    /// Phase 6: Resonance-based collective formation score (0.0 to 1.0)
    /// Measures how well collectives form through resonance
    pub resonance_formation_score: Float,

    /// Phase 6: Average entity resonance (0.0 to 1.0)
    /// Average resonance score across all entity pairs
    pub average_entity_resonance: Float,

    /// Phase 6: Resonance diversity (0.0 to 1.0)
    /// Measures diversity of resonance levels
    pub resonance_diversity: Float,

    /// Phase 7: Average spectrum position (0.0 to 1.0)
    /// Average position on v = s/t ↔ v = t/s continuum
    pub average_spectrum_position: Float,

    /// Phase 7: Space/Time dominant ratio (0.0 to 1.0)
    /// Percentage of entities space/time dominant (spectrum_position < 0.5)
    pub space_time_dominant_ratio: Float,

    /// Phase 7: Time/Space dominant ratio (0.0 to 1.0)
    /// Percentage of entities time/space dominant (spectrum_position > 0.5)
    pub time_space_dominant_ratio: Float,

    /// Phase 7: Average veil transparency (0.0 to 1.0)
    /// Average veil transparency across all entities
    pub average_veil_transparency: Float,
}

impl Default for EmergentProperties {
    fn default() -> Self {
        EmergentProperties {
            global_coherence: 0.0,
            collective_consciousness_level: 0.0,
            system_entropy: 1.0, // Start with maximum entropy
            emergence_score: 0.0,
            organic_emergence_score: 0.0,
            unique_evolutionary_trajectories: 0,
            holographic_coherence: 0.0,
            environmental_integration: 0.0,
            // Phase 4
            holographic_interaction_diversity: 0.0,
            collective_individual_coevolution: 0.0,
            environmental_resonance: 0.0,
            // Phase 6
            resonance_formation_score: 0.0,
            average_entity_resonance: 0.0,
            resonance_diversity: 0.0,
            // Phase 7
            average_spectrum_position: 0.5,
            space_time_dominant_ratio: 1.0,
            time_space_dominant_ratio: 0.0,
            average_veil_transparency: 0.0,
        }
    }
}

impl EmergentProperties {
    /// Calculate emergent properties from system state
    pub fn calculate_from_system_state(
        entities: &[crate::entity_layer7::SubSubLogos],
        holographic_stats: &HolographicFieldStatistics,
        environmental_stats: &crate::simulation_v3::EnvironmentalStatistics,
        collective_influence_stats: &crate::simulation_v3::CollectiveInfluenceStatistics,
    ) -> Self {
        // Calculate global coherence
        let global_coherence = Self::calculate_global_coherence(entities);

        // Calculate collective consciousness level
        let collective_consciousness_level = Self::calculate_collective_consciousness(entities);

        // Calculate system entropy
        let system_entropy = Self::calculate_system_entropy(entities);

        // Calculate holographic coherence
        let holographic_coherence = holographic_stats.global_phase_coherence;

        // Calculate environmental integration
        let environmental_integration = environmental_stats.environmental_coherence;

        // Calculate unique evolutionary trajectories
        let unique_evolutionary_trajectories = Self::count_unique_trajectories(entities);

        // Phase 4: Calculate holographic interaction diversity
        // Based on the number of different connection types present
        let connection_types = [
            holographic_stats.resonant_connections,
            holographic_stats.harmonic_connections,
            holographic_stats.entangled_connections,
            holographic_stats.antiphase_connections,
            holographic_stats.weak_connections,
        ];
        let non_zero_types = connection_types.iter().filter(|&&c| c > 0).count() as Float;
        let holographic_interaction_diversity = non_zero_types / 5.0;

        // Phase 4: Calculate collective-individual co-evolution
        let collective_individual_coevolution = collective_influence_stats.co_evolution_score;

        // Phase 4: Calculate environmental resonance
        // Measures how well entities resonate with their environment
        let environmental_resonance = (environmental_stats.average_vibrational_frequency
            + environmental_stats.environmental_coherence)
            / 2.0;

        // Phase 6: Calculate resonance-based collective formation score
        // Based on the proportion of high resonance connections
        let total_connections = holographic_stats.connection_count;
        let high_resonance_connections = holographic_stats.resonant_connections;
        let resonance_formation_score = if total_connections > 0 {
            high_resonance_connections as Float / total_connections as Float
        } else {
            0.0
        };

        // Phase 6: Calculate average entity resonance
        let average_entity_resonance = holographic_stats.average_archetype_similarity;

        // Phase 6: Calculate resonance diversity
        // Based on variance in resonance scores
        let resonance_diversity = if holographic_stats.average_connection_strength > 0.0 {
            let variance = holographic_stats.average_connection_strength
                * (1.0 - holographic_stats.average_connection_strength);
            variance.sqrt()
        } else {
            0.0
        };

        // Phase 7: Calculate spectrum position statistics
        let mut total_spectrum_position = 0.0;
        let mut space_time_dominant_count = 0;
        let mut time_space_dominant_count = 0;
        let mut total_veil_transparency = 0.0;

        for entity in entities {
            // Calculate spectrum position from spectrum_access
            // spectrum_position = space_time_ratio / (space_time_ratio + time_space_ratio)
            // space_time_ratio = space_time_access (v = s/t)
            // time_space_ratio = time_space_access (v = t/s)
            // Veil is at position 0.5 (v = 1)
            let space_time_ratio = entity.spectrum_access.space_time_access;
            let time_space_ratio = entity.spectrum_access.time_space_access;
            let spectrum_position =
                space_time_ratio / (space_time_ratio + time_space_ratio + 0.0001);

            total_spectrum_position += spectrum_position;
            total_veil_transparency += if entity.spectrum_access.veil_active {
                0.0
            } else {
                1.0
            };

            // Determine dominance based on spectrum position
            // Veil is at position 0.5 (v = 1)
            if spectrum_position < 0.5 {
                space_time_dominant_count += 1;
            } else if spectrum_position > 0.5 {
                time_space_dominant_count += 1;
            }
        }

        let entity_count = entities.len() as Float;
        let average_spectrum_position = if entity_count > 0.0 {
            total_spectrum_position / entity_count
        } else {
            0.5
        };
        let _space_time_dominant_ratio = if entity_count > 0.0 {
            space_time_dominant_count as Float / entity_count
        } else {
            1.0
        };
        let _time_space_dominant_ratio = if entity_count > 0.0 {
            time_space_dominant_count as Float / entity_count
        } else {
            0.0
        };
        let _average_veil_transparency = if entity_count > 0.0 {
            total_veil_transparency / entity_count
        } else {
            0.0
        };
        let _space_time_dominant_ratio = if entity_count > 0.0 {
            space_time_dominant_count as Float / entity_count
        } else {
            1.0
        };
        let _time_space_dominant_ratio = if entity_count > 0.0 {
            time_space_dominant_count as Float / entity_count
        } else {
            0.0
        };
        let _average_veil_transparency = if entity_count > 0.0 {
            total_veil_transparency / entity_count
        } else {
            0.0
        };
        let space_time_dominant_ratio = if entity_count > 0.0 {
            space_time_dominant_count as Float / entity_count
        } else {
            1.0
        };
        let time_space_dominant_ratio = if entity_count > 0.0 {
            time_space_dominant_count as Float / entity_count
        } else {
            0.0
        };
        let average_veil_transparency = if entity_count > 0.0 {
            total_veil_transparency / entity_count
        } else {
            0.0
        };

        // Calculate emergence score (overall) - Phase 6 enhanced
        // Now includes resonance-based collective formation
        let emergence_score = global_coherence * 0.10
            + collective_consciousness_level * 0.10
            + holographic_coherence * 0.10
            + environmental_integration * 0.10
            + collective_influence_stats.co_evolution_score * 0.12
            + holographic_interaction_diversity * 0.12
            + environmental_resonance * 0.10
            + holographic_stats.average_archetype_similarity * 0.08
            + resonance_formation_score * 0.10  // Phase 6
            + resonance_diversity * 0.08; // Phase 6

        // Calculate organic emergence score
        // Organic emergence requires:
        // 1. Significant emergence score (> 0.5)
        // 2. High unique evolutionary trajectories (> 50)
        // 3. Balanced system entropy (not too low, not too high)
        // 4. Good holographic interaction diversity (> 0.4)
        // 5. Good resonance formation (> 0.3) (Phase 6)
        let entropy_balance = 1.0 - (system_entropy - 0.5).abs();
        let trajectory_score = (unique_evolutionary_trajectories as Float / 100.0).min(1.0);
        let interaction_diversity_score = holographic_interaction_diversity;
        let resonance_formation_score_weighted = resonance_formation_score; // Phase 6
        let organic_emergence_score = emergence_score
            * entropy_balance
            * trajectory_score
            * interaction_diversity_score
            * (0.5 + 0.5 * resonance_formation_score_weighted); // Phase 6

        EmergentProperties {
            global_coherence,
            collective_consciousness_level,
            system_entropy,
            emergence_score,
            organic_emergence_score,
            unique_evolutionary_trajectories,
            holographic_coherence,
            environmental_integration,
            // Phase 4
            holographic_interaction_diversity,
            collective_individual_coevolution,
            environmental_resonance,
            // Phase 6
            resonance_formation_score,
            average_entity_resonance,
            resonance_diversity,
            // Phase 7
            average_spectrum_position,
            space_time_dominant_ratio,
            time_space_dominant_ratio,
            average_veil_transparency,
        }
    }

    /// Calculate global coherence
    fn calculate_global_coherence(entities: &[crate::entity_layer7::SubSubLogos]) -> Float {
        if entities.is_empty() {
            return 0.0;
        }

        // Calculate average vibrational coherence
        let total_coherence: Float = entities
            .iter()
            .map(|e| e.current_state.vibrational_state.coherence)
            .sum();

        total_coherence / entities.len() as Float
    }

    /// Calculate collective consciousness level
    fn calculate_collective_consciousness(entities: &[crate::entity_layer7::SubSubLogos]) -> Float {
        if entities.is_empty() {
            return 0.0;
        }

        // Calculate average consciousness level
        let total_consciousness: Float = entities
            .iter()
            .map(|e| e.current_state.consciousness_level)
            .sum();

        let avg_consciousness = total_consciousness / entities.len() as Float;

        // Factor in polarization (polarized entities have higher collective consciousness)
        let polarized_count = entities
            .iter()
            .filter(|e| {
                e.current_state.polarity_state.polarization_strength > 0.2
                    && e.current_state.polarity_state.polarity_bias.abs() > 0.2
            })
            .count();

        let polarization_factor = polarized_count as Float / entities.len() as Float;

        (avg_consciousness * 0.7 + polarization_factor * 0.3).clamp(0.0, 1.0)
    }

    /// Calculate system entropy using Shannon entropy
    fn calculate_system_entropy(entities: &[crate::entity_layer7::SubSubLogos]) -> Float {
        if entities.is_empty() {
            return 1.0; // Maximum entropy
        }

        // Calculate density distribution entropy
        let mut density_counts: std::collections::HashMap<u8, usize> =
            std::collections::HashMap::new();

        for entity in entities {
            let density_index = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => 0,
                crate::entity_layer7::layer7::DensityLevel::Second => 1,
                crate::entity_layer7::layer7::DensityLevel::Third => 2,
                crate::entity_layer7::layer7::DensityLevel::Fourth => 3,
                crate::entity_layer7::layer7::DensityLevel::Fifth => 4,
                crate::entity_layer7::layer7::DensityLevel::Sixth => 5,
                crate::entity_layer7::layer7::DensityLevel::Seventh => 6,
                crate::entity_layer7::layer7::DensityLevel::Eighth => 7,
            };
            *density_counts.entry(density_index).or_insert(0) += 1;
        }

        let total = entities.len() as Float;
        let mut entropy = 0.0;

        for count in density_counts.values() {
            let probability = *count as Float / total;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }

        // Normalize entropy (maximum entropy is log2(8) = 3.0 for 8 densities)
        let max_entropy = (density_counts.len() as Float).log2().max(1.0);
        (entropy / max_entropy).clamp(0.0, 1.0)
    }

    /// Count unique evolutionary trajectories
    fn count_unique_trajectories(entities: &[crate::entity_layer7::SubSubLogos]) -> usize {
        // Count entities with unique combinations of:
        // - Current density
        // - Evolutionary rate
        // - Polarity bias
        // - Number of karmic patterns

        let mut unique_combinations: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for entity in entities {
            let density_key = match entity.evolutionary_attractor.current_density {
                crate::entity_layer7::layer7::DensityLevel::First => "1".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Second => "2".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Third => "3".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Fourth => "4".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Fifth => "5".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Sixth => "6".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Seventh => "7".to_string(),
                crate::entity_layer7::layer7::DensityLevel::Eighth => "8".to_string(),
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

        unique_combinations.len()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_tracker_creation() {
        let tracker = StatisticsTracker::new();
        assert_eq!(tracker.statistics.involution.entities_created, 0);
        assert_eq!(tracker.statistics.evolution.current_step, 0);
    }

    #[test]
    fn test_record_involution_complete() {
        let mut tracker = StatisticsTracker::new();

        let stage_details = vec![StageDetail {
            stage_number: 0,
            stage_name: "Violet".to_string(),
            success: true,
            duration: Duration::from_millis(10),
            entities_created: 100,
            attractor_fields_created: 1,
        }];

        tracker.record_involution_complete(100, 8, 8, Duration::from_millis(100), stage_details);

        assert_eq!(tracker.statistics.involution.entities_created, 100);
        assert_eq!(tracker.statistics.involution.attractor_fields_created, 8);
        assert_eq!(tracker.statistics.involution.stage_transitions, 8);
    }

    #[test]
    fn test_record_evolution_step() {
        let mut tracker = StatisticsTracker::new();

        let mut density_dist = HashMap::new();
        density_dist.insert("Third".to_string(), 100);

        let polarization_dist = PolarizationDistribution {
            sto: 50,
            sts: 30,
            unpolarized: 20,
            average_bias: 0.1,
        };

        tracker.record_evolution_step(
            10,  // step
            100, // num_entities
            density_dist,
            polarization_dist,
            5,      // density_transitions
            1000.0, // total_tapped_energy
            10.0,   // average_tap_strength
            0.5,    // spectrum_access_level
            0.8,    // veil_thickness
            0.2,    // veil_transparency
            0.6,    // attractor_field_activation
        );

        assert_eq!(tracker.statistics.evolution.current_step, 10);
        assert_eq!(tracker.statistics.evolution.num_entities, 100);
        assert_eq!(tracker.statistics.evolution.density_transitions, 5);
    }

    #[test]
    fn test_update_performance_metrics() {
        let mut tracker = StatisticsTracker::new();

        tracker.statistics.evolution.total_steps = 100;
        tracker.statistics.evolution.num_entities = 50;

        // Simulate some time passing
        std::thread::sleep(Duration::from_millis(10));

        tracker.update_performance_metrics();

        assert!(tracker.statistics.performance.total_time.as_millis() >= 10);
    }

    #[test]
    fn test_get_summary() {
        let mut tracker = StatisticsTracker::new();

        // Set up some basic statistics
        tracker.statistics.involution.entities_created = 100;
        tracker.statistics.evolution.current_step = 50;
        tracker.statistics.evolution.total_steps = 100;
        tracker.statistics.evolution.num_entities = 100;

        let summary = tracker.get_summary();

        assert!(summary.contains("Simulation Statistics Summary"));
        assert!(summary.contains("Entities created: 100"));
        assert!(summary.contains("Current step: 50"));
    }

    #[test]
    fn test_polarization_distribution_default() {
        let dist = PolarizationDistribution::default();
        assert_eq!(dist.sto, 0);
        assert_eq!(dist.sts, 0);
        assert_eq!(dist.unpolarized, 0);
        assert_eq!(dist.average_bias, 0.0);
    }

    #[test]
    fn test_architecture_metrics_default() {
        let metrics = ArchitectureMetrics::default();
        assert!(!metrics.three_primal_distortions);
        assert_eq!(metrics.transcend_include_stages, 0);
        assert_eq!(metrics.alignment_score, 0.0);
    }
}
