// Resonance Heatmap Visualizer (Phase 6)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 6:
// "Create resonance heatmap visualization"
//
// This module implements:
// 1. Resonance heatmap generation
// 2. Holographic pattern visualization
// 3. Resonance-based clustering visualization
// 4. Coherence tracking visualization

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::holographic_field::HolographicFieldManager;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Resonance Heatmap Visualizer
///
/// Visualizes resonance between entities as a heatmap.
pub struct ResonanceHeatmapVisualizer;

/// Resonance Heatmap Data
///
/// Contains the data needed to generate a resonance heatmap.
#[derive(Debug, Clone)]
pub struct ResonanceHeatmap {
    /// Entity IDs (ordered for matrix indexing)
    pub entity_ids: Vec<EntityId>,

    /// Resonance matrix (entity_index_a, entity_index_b) -> resonance_score
    pub resonance_matrix: Vec<Vec<Float>>,

    /// Resonance statistics
    pub statistics: ResonanceHeatmapStatistics,

    /// Heatmap configuration
    pub config: HeatmapConfig,
}

/// Heatmap Configuration
#[derive(Debug, Clone)]
pub struct HeatmapConfig {
    /// Color scheme for heatmap
    pub color_scheme: ColorScheme,

    /// Show values in cells
    pub show_values: bool,

    /// Show entity IDs
    pub show_entity_ids: bool,

    /// Threshold for highlighting (resonance > threshold)
    pub highlight_threshold: Float,
}

impl Default for HeatmapConfig {
    fn default() -> Self {
        HeatmapConfig {
            color_scheme: ColorScheme::Viridis,
            show_values: true,
            show_entity_ids: true,
            highlight_threshold: 0.8,
        }
    }
}

/// Color Scheme for Heatmap
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    /// Viridis (purple to yellow)
    Viridis,

    /// Plasma (purple to yellow)
    Plasma,

    /// Inferno (black to yellow)
    Inferno,

    /// Magma (black to white)
    Magma,

    /// Blue-Red (cold to hot)
    BlueRed,

    /// Grayscale (black to white)
    Grayscale,
}

/// Resonance Heatmap Statistics
#[derive(Debug, Clone, Default)]
pub struct ResonanceHeatmapStatistics {
    /// Total entities
    pub total_entities: usize,

    /// Total pairs
    pub total_pairs: usize,

    /// Average resonance
    pub average_resonance: Float,

    /// Maximum resonance
    pub max_resonance: Float,

    /// Minimum resonance
    pub min_resonance: Float,

    /// Resonance distribution
    pub distribution: ResonanceDistribution,
}

/// Resonance Distribution
#[derive(Debug, Clone, Default)]
pub struct ResonanceDistribution {
    /// High resonance (> 0.8) count
    pub high_count: usize,

    /// Medium resonance (0.5 to 0.8) count
    pub medium_count: usize,

    /// Low resonance (0.2 to 0.5) count
    pub low_count: usize,

    /// No resonance (< 0.2) count
    pub none_count: usize,
}

/// Holographic Pattern Visualization
#[derive(Debug, Clone)]
pub struct HolographicPatternVisualization {
    /// Pattern ID
    pub pattern_id: String,

    /// Pattern type
    pub pattern_type: HolographicPatternType,

    /// Entities in pattern
    pub entities: Vec<EntityId>,

    /// Pattern resonance score
    pub resonance_score: Float,

    /// Pattern coherence
    pub coherence: Float,

    /// Pattern complexity
    pub complexity: Float,
}

/// Holographic Pattern Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolographicPatternType {
    /// Resonant cluster (entities with high mutual resonance)
    ResonantCluster,

    /// Harmonic pattern (entities with harmonic relationships)
    HarmonicPattern,

    /// Interference pattern (constructive/destructive interference)
    InterferencePattern,

    /// Fractal pattern (self-similar structures)
    FractalPattern,
}

/// Coherence Visualization
#[derive(Debug, Clone)]
pub struct CoherenceVisualization {
    /// Entity coherence
    pub entity_coherence: HashMap<EntityId, Float>,

    /// Collective coherence
    pub collective_coherence: HashMap<EntityId, Float>,

    /// Global coherence
    pub global_coherence: Float,

    /// Coherence history
    pub coherence_history: Vec<CoherenceSnapshot>,
}

/// Coherence Snapshot
#[derive(Debug, Clone)]
pub struct CoherenceSnapshot {
    /// Simulation step
    pub step: u64,

    /// Global coherence
    pub global_coherence: Float,

    /// Average entity coherence
    pub average_entity_coherence: Float,

    /// Average collective coherence
    pub average_collective_coherence: Float,

    /// Coherence variance
    pub coherence_variance: Float,
}

// ============================================================================
// IMPLEMENTATIONS
// ============================================================================

impl ResonanceHeatmapVisualizer {
    /// Generate resonance heatmap
    pub fn generate_heatmap(
        holographic_manager: &HolographicFieldManager,
        config: Option<HeatmapConfig>,
    ) -> ResonanceHeatmap {
        let config = config.unwrap_or_default();

        // Get entity IDs
        let entity_ids: Vec<EntityId> = holographic_manager.entities.keys().cloned().collect();

        // Calculate resonance matrix
        let resonance_matrix = Self::calculate_resonance_matrix(holographic_manager, &entity_ids);

        // Calculate statistics
        let statistics = Self::calculate_heatmap_statistics(&resonance_matrix);

        ResonanceHeatmap {
            entity_ids,
            resonance_matrix,
            statistics,
            config,
        }
    }

    /// Calculate resonance matrix
    fn calculate_resonance_matrix(
        holographic_manager: &HolographicFieldManager,
        entity_ids: &[EntityId],
    ) -> Vec<Vec<Float>> {
        let n = entity_ids.len();
        let mut matrix = vec![vec![0.0; n]; n];

        let entities_clone = holographic_manager.entities.clone();

        for (i, entity_id_a) in entity_ids.iter().enumerate() {
            for (j, entity_id_b) in entity_ids.iter().enumerate() {
                if i == j {
                    matrix[i][j] = 1.0; // Self-resonance is always 1.0
                    continue;
                }

                if let Some(entity_a) = entities_clone.get(entity_id_a) {
                    if let Some(entity_b) = entities_clone.get(entity_id_b) {
                        let resonance = holographic_manager.calculate_resonance(entity_a, entity_b);
                        matrix[i][j] = resonance.resonance_score;
                    }
                }
            }
        }

        matrix
    }

    /// Calculate heatmap statistics
    fn calculate_heatmap_statistics(matrix: &[Vec<Float>]) -> ResonanceHeatmapStatistics {
        let n = matrix.len();
        if n == 0 {
            return ResonanceHeatmapStatistics::default();
        }

        let mut all_values = Vec::new();
        let mut distribution = ResonanceDistribution::default();

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let value = matrix[i][j];
                    all_values.push(value);

                    // Update distribution
                    if value > 0.8 {
                        distribution.high_count += 1;
                    } else if value > 0.5 {
                        distribution.medium_count += 1;
                    } else if value > 0.2 {
                        distribution.low_count += 1;
                    } else {
                        distribution.none_count += 1;
                    }
                }
            }
        }

        if all_values.is_empty() {
            return ResonanceHeatmapStatistics {
                total_entities: n,
                total_pairs: 0,
                average_resonance: 0.0,
                max_resonance: 0.0,
                min_resonance: 0.0,
                distribution,
            };
        }

        let average = all_values.iter().sum::<Float>() / all_values.len() as Float;
        let max = *all_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let min = *all_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);

        ResonanceHeatmapStatistics {
            total_entities: n,
            total_pairs: all_values.len(),
            average_resonance: average,
            max_resonance: max,
            min_resonance: min,
            distribution,
        }
    }

    /// Visualize resonance heatmap as ASCII
    pub fn visualize_ascii(heatmap: &ResonanceHeatmap) -> String {
        let mut output = String::new();

        output.push_str("=== Resonance Heatmap ===\n\n");

        // Header
        if heatmap.config.show_entity_ids {
            output.push_str("      ");
            for (i, _entity_id) in heatmap.entity_ids.iter().enumerate() {
                if i < 10 {
                    output.push_str(&format!(" {:02} ", i));
                } else {
                    output.push_str(&format!(" {:02} ", i));
                }
            }
            output.push_str("\n");
        }

        // Matrix
        for (i, row) in heatmap.resonance_matrix.iter().enumerate() {
            if heatmap.config.show_entity_ids {
                output.push_str(&format!("{:02}  ", i));
            }

            for value in row {
                let char = Self::resonance_to_char(*value, &heatmap.config.color_scheme);
                output.push(char);
                output.push(' ');

                if heatmap.config.show_values {
                    output.push_str(&format!("{:.2}", value));
                    output.push(' ');
                }
            }
            output.push('\n');
        }

        // Statistics
        output.push_str("\n=== Statistics ===\n");
        output.push_str(&format!(
            "Total Entities: {}\n",
            heatmap.statistics.total_entities
        ));
        output.push_str(&format!(
            "Total Pairs: {}\n",
            heatmap.statistics.total_pairs
        ));
        output.push_str(&format!(
            "Average Resonance: {:.3}\n",
            heatmap.statistics.average_resonance
        ));
        output.push_str(&format!(
            "Max Resonance: {:.3}\n",
            heatmap.statistics.max_resonance
        ));
        output.push_str(&format!(
            "Min Resonance: {:.3}\n",
            heatmap.statistics.min_resonance
        ));

        output.push_str("\n=== Distribution ===\n");
        output.push_str(&format!(
            "High (> 0.8): {}\n",
            heatmap.statistics.distribution.high_count
        ));
        output.push_str(&format!(
            "Medium (0.5-0.8): {}\n",
            heatmap.statistics.distribution.medium_count
        ));
        output.push_str(&format!(
            "Low (0.2-0.5): {}\n",
            heatmap.statistics.distribution.low_count
        ));
        output.push_str(&format!(
            "None (< 0.2): {}\n",
            heatmap.statistics.distribution.none_count
        ));

        // Legend
        output.push_str("\n=== Legend ===\n");
        output.push_str(&Self::get_legend(&heatmap.config.color_scheme));

        output
    }

    /// Convert resonance value to ASCII character
    fn resonance_to_char(value: Float, color_scheme: &ColorScheme) -> char {
        match color_scheme {
            ColorScheme::Viridis => {
                // Purple to yellow
                if value < 0.2 {
                    '░'
                } else if value < 0.4 {
                    '▒'
                } else if value < 0.6 {
                    '▓'
                } else if value < 0.8 {
                    '▔'
                } else {
                    '█'
                }
            }
            ColorScheme::Plasma => {
                // Purple to yellow (similar to Viridis)
                if value < 0.2 {
                    '░'
                } else if value < 0.4 {
                    '▒'
                } else if value < 0.6 {
                    '▓'
                } else if value < 0.8 {
                    '▔'
                } else {
                    '█'
                }
            }
            ColorScheme::Inferno => {
                // Black to yellow
                if value < 0.2 {
                    '·'
                } else if value < 0.4 {
                    '░'
                } else if value < 0.6 {
                    '▒'
                } else if value < 0.8 {
                    '▓'
                } else {
                    '█'
                }
            }
            ColorScheme::Magma => {
                // Black to white
                if value < 0.2 {
                    '·'
                } else if value < 0.4 {
                    '░'
                } else if value < 0.6 {
                    '▒'
                } else if value < 0.8 {
                    '▓'
                } else {
                    '█'
                }
            }
            ColorScheme::BlueRed => {
                // Blue (cold) to red (hot)
                if value < 0.2 {
                    '░'
                }
                // Blue
                else if value < 0.4 {
                    '▒'
                }
                // Light blue
                else if value < 0.6 {
                    '▓'
                }
                // White
                else if value < 0.8 {
                    '▔'
                }
                // Light red
                else {
                    '█'
                } // Red
            }
            ColorScheme::Grayscale => {
                // Black to white
                if value < 0.2 {
                    '░'
                } else if value < 0.4 {
                    '▒'
                } else if value < 0.6 {
                    '▓'
                } else if value < 0.8 {
                    '▔'
                } else {
                    '█'
                }
            }
        }
    }

    /// Get color scheme legend
    fn get_legend(color_scheme: &ColorScheme) -> String {
        match color_scheme {
            ColorScheme::Viridis | ColorScheme::Plasma => {
                "░ (0.0-0.2) Purple\n▒ (0.2-0.4) Blue\n▓ (0.4-0.6) Green\n▔ (0.6-0.8) Yellow\n█ (0.8-1.0) Yellow\n".to_string()
            }
            ColorScheme::Inferno | ColorScheme::Magma => {
                "· (0.0-0.2) Black\n░ (0.2-0.4) Dark Red\n▒ (0.4-0.6) Red\n▓ (0.6-0.8) Orange\n█ (0.8-1.0) Yellow\n".to_string()
            }
            ColorScheme::BlueRed => {
                "░ (0.0-0.2) Blue\n▒ (0.2-0.4) Light Blue\n▓ (0.4-0.6) White\n▔ (0.6-0.8) Light Red\n█ (0.8-1.0) Red\n".to_string()
            }
            ColorScheme::Grayscale => {
                "░ (0.0-0.2) Light Gray\n▒ (0.2-0.4) Medium Gray\n▓ (0.4-0.6) Dark Gray\n▔ (0.6-0.8) Very Dark Gray\n█ (0.8-1.0) Black\n".to_string()
            }
        }
    }

    /// Identify holographic patterns
    pub fn identify_patterns(
        holographic_manager: &HolographicFieldManager,
    ) -> Vec<HolographicPatternVisualization> {
        let mut patterns = Vec::new();

        // Find resonant clusters
        let high_resonance_pairs = holographic_manager.find_high_resonance_pairs(0.7);

        // Group entities into clusters
        let mut clusters: Vec<Vec<EntityId>> = Vec::new();
        let mut clustered: std::collections::HashSet<EntityId> = std::collections::HashSet::new();

        for (entity_a, entity_b, _) in &high_resonance_pairs {
            let mut found_cluster = false;

            for cluster in &mut clusters {
                if cluster.contains(entity_a) || cluster.contains(entity_b) {
                    if !cluster.contains(entity_a) {
                        cluster.push(entity_a.clone());
                        clustered.insert(entity_a.clone());
                    }
                    if !cluster.contains(entity_b) {
                        cluster.push(entity_b.clone());
                        clustered.insert(entity_b.clone());
                    }
                    found_cluster = true;
                    break;
                }
            }

            if !found_cluster {
                let new_cluster = vec![entity_a.clone(), entity_b.clone()];
                clusters.push(new_cluster);
                clustered.insert(entity_a.clone());
                clustered.insert(entity_b.clone());
            }
        }

        // Create pattern visualizations for clusters
        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.len() < 2 {
                continue;
            }

            // Calculate cluster resonance
            let entities_clone = holographic_manager.entities.clone();
            let mut total_resonance = 0.0;
            let mut pair_count = 0;

            for j in 0..cluster.len() {
                for k in (j + 1)..cluster.len() {
                    if let Some(entity_a) = entities_clone.get(&cluster[j]) {
                        if let Some(entity_b) = entities_clone.get(&cluster[k]) {
                            let resonance =
                                holographic_manager.calculate_resonance(entity_a, entity_b);
                            total_resonance += resonance.resonance_score;
                            pair_count += 1;
                        }
                    }
                }
            }

            let avg_resonance = if pair_count > 0 {
                total_resonance / pair_count as Float
            } else {
                0.0
            };

            // Calculate coherence
            let mut coherence = 0.0;
            for entity_id in cluster {
                if let Some(entity) = entities_clone.get(entity_id) {
                    coherence += entity.current_state.vibrational_state.coherence;
                }
            }
            coherence /= cluster.len() as Float;

            // Calculate complexity (based on cluster size and resonance diversity)
            let complexity = (cluster.len() as Float / 10.0).min(1.0) * avg_resonance;

            patterns.push(HolographicPatternVisualization {
                pattern_id: format!("cluster-{}", i),
                pattern_type: HolographicPatternType::ResonantCluster,
                entities: cluster.clone(),
                resonance_score: avg_resonance,
                coherence,
                complexity,
            });
        }

        patterns
    }

    /// Visualize holographic patterns
    pub fn visualize_patterns(patterns: &[HolographicPatternVisualization]) -> String {
        let mut output = String::new();

        output.push_str("=== Holographic Patterns ===\n\n");

        if patterns.is_empty() {
            output.push_str("No holographic patterns detected.\n");
            return output;
        }

        for (i, pattern) in patterns.iter().enumerate() {
            output.push_str(&format!("Pattern {}: {}\n", i + 1, pattern.pattern_id));
            output.push_str(&format!("  Type: {:?}\n", pattern.pattern_type));
            output.push_str(&format!("  Entities: {}\n", pattern.entities.len()));
            output.push_str(&format!(
                "  Resonance Score: {:.3}\n",
                pattern.resonance_score
            ));
            output.push_str(&format!("  Coherence: {:.3}\n", pattern.coherence));
            output.push_str(&format!("  Complexity: {:.3}\n", pattern.complexity));
            output.push('\n');
        }

        output
    }

    /// Create coherence visualization
    pub fn create_coherence_visualization(
        entity_coherence: &HashMap<EntityId, Float>,
        collective_coherence: &HashMap<EntityId, Float>,
        global_coherence: Float,
        coherence_history: &[crate::simulation_v3::holographic_field::CoherenceSnapshot],
    ) -> CoherenceVisualization {
        let history = coherence_history
            .iter()
            .map(|snap| CoherenceSnapshot {
                step: snap.step,
                global_coherence: snap.global_coherence,
                average_entity_coherence: snap.average_entity_coherence,
                average_collective_coherence: snap.average_collective_coherence,
                coherence_variance: snap.coherence_variance,
            })
            .collect();

        CoherenceVisualization {
            entity_coherence: entity_coherence.clone(),
            collective_coherence: collective_coherence.clone(),
            global_coherence,
            coherence_history: history,
        }
    }

    /// Visualize coherence
    pub fn visualize_coherence(coherence_viz: &CoherenceVisualization) -> String {
        let mut output = String::new();

        output.push_str("=== Coherence Visualization ===\n\n");

        output.push_str(&format!(
            "Global Coherence: {:.3}\n\n",
            coherence_viz.global_coherence
        ));

        // Entity coherence
        output.push_str("Entity Coherence:\n");
        if !coherence_viz.entity_coherence.is_empty() {
            let mut entity_coherence_vec: Vec<_> = coherence_viz.entity_coherence.iter().collect();
            entity_coherence_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

            for (entity_id, coherence) in entity_coherence_vec.iter().take(10) {
                output.push_str(&format!("  {}: {:.3}\n", entity_id.uuid, coherence));
            }
        }
        output.push('\n');

        // Collective coherence
        output.push_str("Collective Coherence:\n");
        if !coherence_viz.collective_coherence.is_empty() {
            let mut collective_coherence_vec: Vec<_> =
                coherence_viz.collective_coherence.iter().collect();
            collective_coherence_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

            for (collective_id, coherence) in collective_coherence_vec.iter().take(10) {
                output.push_str(&format!("  {}: {:.3}\n", collective_id.uuid, coherence));
            }
        }
        output.push('\n');

        // Coherence history
        if !coherence_viz.coherence_history.is_empty() {
            output.push_str("Coherence History (last 10 steps):\n");
            for snap in coherence_viz.coherence_history.iter().rev().take(10) {
                output.push_str(&format!(
                    "  Step {}: Global={:.3}, Entity Avg={:.3}, Collective Avg={:.3}, Variance={:.3}\n",
                    snap.step,
                    snap.global_coherence,
                    snap.average_entity_coherence,
                    snap.average_collective_coherence,
                    snap.coherence_variance
                ));
            }
        }

        output
    }

    /// Generate comprehensive resonance report
    pub fn generate_resonance_report(holographic_manager: &HolographicFieldManager) -> String {
        let mut output = String::new();

        output.push_str("=== Holographic Resonance Report ===\n\n");

        // Generate heatmap
        let heatmap = Self::generate_heatmap(holographic_manager, None);
        output.push_str(&Self::visualize_ascii(&heatmap));
        output.push_str("\n");

        // Identify patterns
        let patterns = Self::identify_patterns(holographic_manager);
        output.push_str(&Self::visualize_patterns(&patterns));
        output.push_str("\n");

        // Coherence visualization
        let coherence_viz = Self::create_coherence_visualization(
            &holographic_manager.coherence_tracker.entity_coherence,
            &holographic_manager.coherence_tracker.collective_coherence,
            holographic_manager.coherence_tracker.global_coherence,
            &holographic_manager.coherence_tracker.coherence_history,
        );
        output.push_str(&Self::visualize_coherence(&coherence_viz));
        output.push_str("\n");

        // Resonance statistics
        let resonance_stats = holographic_manager.get_resonance_statistics();
        output.push_str("=== Resonance Statistics ===\n");
        output.push_str(&format!("Total Pairs: {}\n", resonance_stats.total_pairs));
        output.push_str(&format!(
            "Average Resonance: {:.3}\n",
            resonance_stats.average_resonance
        ));
        output.push_str(&format!(
            "Max Resonance: {:.3}\n",
            resonance_stats.max_resonance
        ));
        output.push_str(&format!(
            "Min Resonance: {:.3}\n",
            resonance_stats.min_resonance
        ));
        output.push_str(&format!(
            "Resonance Variance: {:.3}\n",
            resonance_stats.resonance_variance
        ));
        output.push_str(&format!(
            "High Resonance Pairs (> 0.8): {}\n",
            resonance_stats.high_resonance_count
        ));
        output.push_str(&format!(
            "Medium Resonance Pairs (0.5-0.8): {}\n",
            resonance_stats.medium_resonance_count
        ));
        output.push_str(&format!(
            "Low Resonance Pairs (0.2-0.5): {}\n",
            resonance_stats.low_resonance_count
        ));
        output.push_str(&format!(
            "No Resonance Pairs (< 0.2): {}\n",
            resonance_stats.no_resonance_count
        ));

        output
    }
}
