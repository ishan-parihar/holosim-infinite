// Individual Variation Visualizer (Phase 5)
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 5:
// "Add visualization for individual variation"
//
// This module implements visualization for individual variation:
// 1. Evolutionary rate distribution
// 2. Karmic pattern statistics
// 3. Catalyst event statistics
// 4. Holographic interaction statistics
// 5. Polarity diversity

use crate::simulation_v3::individual_variation_statistics::IndividualVariationStatistics;
use crate::types::Float;

/// Individual Variation Visualizer
///
/// Visualizes individual variation statistics from the simulation.
pub struct IndividualVariationVisualizer {
    statistics: IndividualVariationStatistics,
}

impl IndividualVariationVisualizer {
    /// Create a new Individual Variation Visualizer
    pub fn new(statistics: IndividualVariationStatistics) -> Self {
        IndividualVariationVisualizer { statistics }
    }

    /// Generate comprehensive visualization of individual variation
    pub fn generate_variation_visualization(&self) -> String {
        let mut output = String::new();

        output
            .push_str("╔══════════════════════════════════════════════════════════════════════╗\n");
        output
            .push_str("║     INDIVIDUAL VARIATION VISUALIZATION (Phase 5)                    ║\n");
        output
            .push_str("╚══════════════════════════════════════════════════════════════════════╝\n");
        output.push('\n');

        // Evolutionary Rate Distribution
        output.push_str(&self.visualize_evolutionary_rate_distribution());

        // Karmic Pattern Statistics
        output.push_str(&self.visualize_karmic_patterns());

        // Catalyst Event Statistics
        output.push_str(&self.visualize_catalyst_events());

        // Holographic Interaction Statistics
        output.push_str(&self.visualize_holographic_interactions());

        // Polarity Diversity
        output.push_str(&self.visualize_polarization_diversity());

        output.push('\n');
        output
            .push_str("╔══════════════════════════════════════════════════════════════════════╗\n");
        output
            .push_str("║                    ORGANIC EMERGENCE METRICS                         ║\n");
        output
            .push_str("╚══════════════════════════════════════════════════════════════════════╝\n");
        output.push_str(&self.visualize_organic_emergence());

        output
    }

    /// Visualize evolutionary rate distribution
    fn visualize_evolutionary_rate_distribution(&self) -> String {
        let mut output = String::new();

        output
            .push_str("┌─ EVOLUTIONARY RATE DISTRIBUTION ─────────────────────────────────────┐\n");
        output.push_str("│\n");
        output.push_str(&format!(
            "│ Average Evolutionary Rate: {:.3}x\n",
            self.statistics.average_evolutionary_rate
        ));
        output.push_str(&format!(
            "│ Min Evolutionary Rate:     {:.3}x\n",
            self.statistics.min_evolutionary_rate
        ));
        output.push_str(&format!(
            "│ Max Evolutionary Rate:     {:.3}x\n",
            self.statistics.max_evolutionary_rate
        ));
        output.push_str("│\n");

        // Distribution buckets - Phase 2: Updated to cover full range (0.3x-1.7x)
        output.push_str("│ Distribution:\n");
        for (range, count) in &self.statistics.evolutionary_rate_distribution {
            output.push_str(&format!("│   {}: ", range));
            output.push_str(&Self::create_bar(*count, 50));
            output.push_str(&format!(" ({})\n", count));
        }

        output.push_str("│\n");
        output.push_str(
            "└────────────────────────────────────────────────────────────────────────┘\n",
        );
        output.push('\n');

        output
    }

    /// Visualize karmic patterns
    fn visualize_karmic_patterns(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "┌─ KARMIC PATTERN STATISTICS ────────────────────────────────────────────┐\n",
        );
        output.push_str("│\n");
        output.push_str(&format!(
            "│ Total Karmic Patterns:       {}\n",
            self.statistics.karmic_patterns.total_patterns
        ));
        output.push_str(&format!(
            "│ Average Intensity:           {:.3}\n",
            self.statistics.karmic_patterns.average_intensity
        ));
        output.push_str(&format!(
            "│ Avg Patterns per Entity:     {:.2}\n",
            self.statistics.karmic_patterns.average_patterns_per_entity
        ));
        output.push_str("│\n");

        output.push_str("│ By Resolution Status:\n");
        for (status, count) in &self.statistics.karmic_patterns.by_resolution_status {
            output.push_str(&format!("│   {}: {}\n", status, count));
        }

        output.push_str("│\n");
        output.push_str(
            "└────────────────────────────────────────────────────────────────────────┘\n",
        );
        output.push('\n');

        output
    }

    /// Visualize catalyst events
    fn visualize_catalyst_events(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "┌─ CATALYST EVENT STATISTICS ─────────────────────────────────────────────┐\n",
        );
        output.push_str("│\n");
        output.push_str(&format!(
            "│ Total Catalyst Events:        {}\n",
            self.statistics.catalyst_events.total_events
        ));
        output.push_str(&format!(
            "│ Average Catalyst Intensity:  {:.3}\n",
            self.statistics.catalyst_events.average_intensity
        ));
        output.push_str(&format!(
            "│ Total Learning Value:        {:.3}\n",
            self.statistics.catalyst_events.total_learning_value
        ));
        output.push_str(&format!(
            "│ Total Consciousness Exp:     {:.3}\n",
            self.statistics
                .catalyst_events
                .total_consciousness_expansion
        ));
        output.push_str(&format!(
            "│ Catalyst Effectiveness:      {:.3}\n",
            self.statistics.catalyst_events.catalyst_effectiveness
        ));
        output.push_str("│\n");

        output.push_str("│ By Catalyst Type:\n");
        for (catalyst_type, count) in &self.statistics.catalyst_events.by_catalyst_type {
            output.push_str(&format!("│   {}: {}\n", catalyst_type, count));
        }

        output.push_str("│\n");

        // Polarity choices
        output.push_str("│ Polarity Choices:\n");
        output.push_str(&format!(
            "│   Service-to-Others (STO):   {} ({:.1}%)\n",
            self.statistics.catalyst_events.polarity_choices.sto_choices,
            self.statistics
                .catalyst_events
                .polarity_choices
                .sto_percentage
        ));
        output.push_str(&format!(
            "│   Service-to-Self (STS):     {} ({:.1}%)\n",
            self.statistics.catalyst_events.polarity_choices.sts_choices,
            self.statistics
                .catalyst_events
                .polarity_choices
                .sts_percentage
        ));
        output.push_str(&format!(
            "│   Unpolarized:              {} ({:.1}%)\n",
            self.statistics
                .catalyst_events
                .polarity_choices
                .unpolarized_choices,
            self.statistics
                .catalyst_events
                .polarity_choices
                .unpolarized_percentage
        ));

        output.push_str("│\n");
        output.push_str(
            "└────────────────────────────────────────────────────────────────────────┘\n",
        );
        output.push('\n');

        output
    }

    /// Visualize holographic interactions
    fn visualize_holographic_interactions(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "┌─ HOLOGRAPHIC INTERACTION STATISTICS ──────────────────────────────────┐\n",
        );
        output.push_str("│\n");
        output.push_str(&format!(
            "│ Total Connections:            {}\n",
            self.statistics.holographic_interactions.total_connections
        ));
        output.push_str(&format!(
            "│ Avg Connection Strength:     {:.3}\n",
            self.statistics
                .holographic_interactions
                .average_connection_strength
        ));
        output.push_str(&format!(
            "│ Global Phase Coherence:      {:.3}\n",
            self.statistics
                .holographic_interactions
                .global_phase_coherence
        ));
        output.push_str("│\n");

        output.push_str("│ By Connection Type:\n");
        for (conn_type, count) in &self.statistics.holographic_interactions.by_connection_type {
            output.push_str(&format!("│   {}: {}\n", conn_type, count));
        }

        output.push_str("│\n");

        output.push_str(&format!(
            "│ Resonant Connections:        {}\n",
            self.statistics
                .holographic_interactions
                .resonant_connections
        ));
        output.push_str(&format!(
            "│ Entangled Connections:       {}\n",
            self.statistics
                .holographic_interactions
                .entangled_connections
        ));
        output.push_str(&format!(
            "│ Harmonic Connections:       {}\n",
            self.statistics
                .holographic_interactions
                .harmonic_connections
        ));
        output.push_str("│\n");

        output.push_str(&format!(
            "│ Resonance Clusters:          {}\n",
            self.statistics
                .holographic_interactions
                .resonance_cluster_count
        ));
        output.push_str(&format!(
            "│ Avg Cluster Size:           {:.2}\n",
            self.statistics
                .holographic_interactions
                .average_cluster_size
        ));

        output.push_str("│\n");
        output.push_str(
            "└────────────────────────────────────────────────────────────────────────┘\n",
        );
        output.push('\n');

        output
    }

    /// Visualize polarization diversity
    fn visualize_polarization_diversity(&self) -> String {
        let mut output = String::new();

        output.push_str(
            "┌─ POLARIZATION DIVERSITY ───────────────────────────────────────────────┐\n",
        );
        output.push_str("│\n");
        output.push_str(&format!(
            "│ Polarity Diversity Index: {:.3} / 1.0\n",
            self.statistics.polarization_diversity
        ));
        output.push_str("│\n");

        // Diversity interpretation
        let diversity_level = if self.statistics.polarization_diversity < 0.3 {
            "LOW (Mostly uniform)"
        } else if self.statistics.polarization_diversity < 0.6 {
            "MODERATE (Some diversity)"
        } else {
            "HIGH (Diverse polarization)"
        };

        output.push_str(&format!("│ Diversity Level: {}\n", diversity_level));
        output.push_str("│\n");

        // Visualization bar
        let bar_length = (self.statistics.polarization_diversity * 50.0) as usize;
        output.push_str("│ Diversity: [");
        for i in 0..50 {
            if i < bar_length {
                output.push('█');
            } else {
                output.push('░');
            }
        }
        output.push_str("]\n");

        output.push_str("│\n");
        output.push_str(
            "└────────────────────────────────────────────────────────────────────────┘\n",
        );
        output.push('\n');

        output
    }

    /// Visualize organic emergence metrics
    fn visualize_organic_emergence(&self) -> String {
        let mut output = String::new();

        output.push_str("│\n");
        output.push_str(&format!(
            "│ Unique Evolutionary Trajectories:  {}\n",
            self.statistics.unique_trajectories
        ));
        output.push_str(&format!(
            "│ Polarity Diversity:            {:.3}\n",
            self.statistics.polarization_diversity
        ));
        output.push_str(&format!(
            "│ Catalyst Effectiveness:            {:.3}\n",
            self.statistics.catalyst_events.catalyst_effectiveness
        ));
        output.push_str(&format!(
            "│ Holographic Coherence:             {:.3}\n",
            self.statistics
                .holographic_interactions
                .global_phase_coherence
        ));
        output.push_str("│\n");

        // Organic emergence score
        let organic_score = (self.statistics.polarization_diversity * 0.3
            + self.statistics.catalyst_events.catalyst_effectiveness * 0.3
            + self
                .statistics
                .holographic_interactions
                .global_phase_coherence
                * 0.2
            + (self.statistics.average_evolutionary_rate - 0.5) / 1.0 * 0.2)
            .clamp(0.0, 1.0);

        output.push_str(&format!(
            "│ Organic Emergence Score:           {:.3} / 1.0\n",
            organic_score
        ));

        // Organic emergence level
        let organic_level = if organic_score < 0.3 {
            "DETERMINISTIC (Low variation)"
        } else if organic_score < 0.6 {
            "EMERGING (Some variation)"
        } else {
            "ORGANIC (High variation and emergence)"
        };

        output.push_str(&format!(
            "│ Emergence Level:                  {}\n",
            organic_level
        ));
        output.push_str("│\n");

        output
    }

    /// Create a bar visualization
    fn create_bar(value: usize, max_length: usize) -> String {
        let bar_length = (value as Float / (max_length as Float) * 50.0) as usize;
        let bar_length = bar_length.min(50);

        let mut bar = String::new();
        for _ in 0..bar_length {
            bar.push('█');
        }
        for _ in bar_length..50 {
            bar.push('░');
        }
        bar
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_variation_visualizer() {
        let statistics = IndividualVariationStatistics::default();
        let visualizer = IndividualVariationVisualizer::new(statistics);
        let output = visualizer.generate_variation_visualization();

        assert!(output.contains("INDIVIDUAL VARIATION VISUALIZATION"));
        assert!(output.contains("EVOLUTIONARY RATE DISTRIBUTION"));
        assert!(output.contains("KARMIC PATTERN STATISTICS"));
        assert!(output.contains("CATALYST EVENT STATISTICS"));
        assert!(output.contains("HOLOGRAPHIC INTERACTION STATISTICS"));
        assert!(output.contains("POLARIZATION DIVERSITY"));
    }
}
