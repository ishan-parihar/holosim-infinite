// Phase 2: Spectrum Distribution Visualization
//
// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md:
// "Phase 2: Spectrum Visualization"
//
// This module implements comprehensive spectrum visualization showing:
// 1. Space/Time ↔ Time/Space continuum (v = s/t ↔ v = t/s)
// 2. Each entity's position on the spectrum
// 3. What each entity is currently doing
// 4. The Veil's effect on entity perception
// 5. Density distribution

use crate::entity_layer7::layer7::{DensityLevel, EntityId, SubSubLogos};
use std::collections::HashMap;

/// Entity Activity - What is this entity currently doing?
#[derive(Debug, Clone)]
pub enum EntityActivity {
    /// Accumulating experience at current density
    AccumulatingExperience {
        density: String,
        experience_level: f64,
    },
    /// Seeking catalyst for evolution
    SeekingCatalyst {
        catalyst_type: String,
        intensity: f64,
    },
    /// Making evolutionary choice
    MakingChoice { choice_type: String, alignment: f64 },
}

impl EntityActivity {
    /// Get display name for this activity
    pub fn display_name(&self) -> &'static str {
        match self {
            EntityActivity::AccumulatingExperience { .. } => "Accumulating Experience",
            EntityActivity::SeekingCatalyst { .. } => "Seeking Catalyst",
            EntityActivity::MakingChoice { .. } => "Making Choice",
        }
    }

    /// Get activity description
    pub fn description(&self) -> String {
        match self {
            EntityActivity::AccumulatingExperience {
                density,
                experience_level,
            } => {
                format!(
                    "Accumulating experience at {} - Exp: {:.2}%",
                    density,
                    experience_level * 100.0
                )
            }
            EntityActivity::SeekingCatalyst {
                catalyst_type,
                intensity,
            } => format!(
                "Seeking {} catalyst (intensity: {:.2})",
                catalyst_type, intensity
            ),
            EntityActivity::MakingChoice {
                choice_type,
                alignment,
            } => format!(
                "Making {} choice (alignment: {:.2})",
                choice_type, alignment
            ),
        }
    }
}

/// Entity Spectrum Position
#[derive(Debug, Clone)]
pub struct EntitySpectrumPosition {
    /// Entity ID
    pub entity_id: EntityId,
    /// Spectrum ratio (v = s/t or v = t/s)
    /// Values > 1.0: space/time dominant
    /// Values < 1.0: time/space dominant
    /// Value of 1.0: at the Veil
    pub spectrum_ratio: f64,
    /// Position on continuum (0.0 to 1.0)
    /// 0.0: v = ∞ (time/space extreme)
    /// 0.5: v = 1.0 (Veil)
    /// 1.0: v = ∞ (space/time extreme)
    pub continuum_position: f64,
    /// Space/time dominance (0.0 to 1.0)
    pub space_time_dominance: f64,
    /// Time/space dominance (0.0 to 1.0)
    pub time_space_dominance: f64,
    /// Veil transparency (0.0 to 1.0)
    pub veil_transparency: f64,
    /// Current activity
    pub current_activity: EntityActivity,
}

impl EntitySpectrumPosition {
    /// Calculate spectrum position from entity
    pub fn from_entity(entity: &SubSubLogos) -> Self {
        let spectrum_ratio = entity.spectrum_access.ratio;

        // Calculate continuum position (logarithmic scale)
        let continuum_position = if spectrum_ratio >= 1.0 {
            0.5 + (spectrum_ratio.log10() / 10.0).min(0.5)
        } else {
            0.5 - ((1.0 / spectrum_ratio).log10() / 10.0).min(0.5)
        };

        // Calculate dominance
        let total_access =
            entity.spectrum_access.space_time_access + entity.spectrum_access.time_space_access;

        let space_time_dominance = if total_access > 0.0 {
            entity.spectrum_access.space_time_access / total_access
        } else {
            0.5
        };

        let time_space_dominance = if total_access > 0.0 {
            entity.spectrum_access.time_space_access / total_access
        } else {
            0.5
        };

        // Calculate veil transparency based on density level
        let veil_transparency = match entity.evolutionary_attractor.current_density {
            DensityLevel::First => 0.0,
            DensityLevel::Second => 0.1,
            DensityLevel::Third => 0.2,
            DensityLevel::Fourth => 0.4,
            DensityLevel::Fifth => 0.7,
            DensityLevel::Sixth => 1.0,
            DensityLevel::Seventh => 1.0,
            DensityLevel::Eighth => 1.0,
        };

        // Determine current activity
        let current_activity = Self::determine_activity(entity);

        EntitySpectrumPosition {
            entity_id: entity.entity_id.clone(),
            spectrum_ratio,
            continuum_position,
            space_time_dominance,
            time_space_dominance,
            veil_transparency,
            current_activity,
        }
    }

    /// Determine current activity of entity
    fn determine_activity(entity: &SubSubLogos) -> EntityActivity {
        // Check if entity is ready to evolve
        if entity.is_ready_to_evolve() {
            let polarity_bias = entity.current_state.polarity_state.polarity_bias;

            return EntityActivity::MakingChoice {
                choice_type: if polarity_bias > 0.0 {
                    "STO Density Transition".to_string()
                } else if polarity_bias < 0.0 {
                    "STS Density Transition".to_string()
                } else {
                    "Neutral Evolution".to_string()
                },
                alignment: polarity_bias,
            };
        }

        // Get current density for activity determination
        let current_density = format!("{:?}", entity.evolutionary_attractor.current_density);

        // Check if entity is seeking catalyst
        if entity.current_state.experience_accumulation < 0.7 {
            return EntityActivity::SeekingCatalyst {
                catalyst_type: if entity.current_state.polarity_state.polarity_bias > 0.0 {
                    "STO".to_string()
                } else if entity.current_state.polarity_state.polarity_bias < 0.0 {
                    "STS".to_string()
                } else {
                    "Neutral".to_string()
                },
                intensity: 1.0 - entity.current_state.experience_accumulation,
            };
        }

        // Default: accumulating experience
        EntityActivity::AccumulatingExperience {
            density: current_density,
            experience_level: entity.current_state.experience_accumulation,
        }
    }
}

/// Spectrum Distribution Visualizer - Phase 2
///
/// Visualizes the complete spectrum continuum showing where each entity
/// is located and what they are doing.
pub struct SpectrumDistributionVisualizer;

impl SpectrumDistributionVisualizer {
    /// Generate comprehensive spectrum visualization
    pub fn visualize_spectrum_distribution(entities: &HashMap<EntityId, SubSubLogos>) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              SPECTRUM DISTRIBUTION VISUALIZATION                  ║\n");
        output.push_str("║         Space/Time ↔ Time/Space Continuum (v = s/t ↔ v = t/s)     ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Calculate entity spectrum positions
        let positions: Vec<EntitySpectrumPosition> = entities
            .values()
            .map(EntitySpectrumPosition::from_entity)
            .collect();

        // Display spectrum continuum map
        output.push_str(&Self::visualize_spectrum_continuum(&positions));

        // Display Veil indicator
        output.push_str(&Self::visualize_veil_effect(&positions));

        // Display entity activities
        output.push_str(&Self::visualize_entity_activities(&positions));

        // Display density distribution
        output.push_str(&Self::visualize_density_distribution(entities));

        // Display spectrum statistics
        output.push_str(&Self::visualize_spectrum_statistics(&positions));

        output
    }

    /// Visualize spectrum continuum map
    fn visualize_spectrum_continuum(positions: &[EntitySpectrumPosition]) -> String {
        let mut output = String::new();

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SPECTRUM CONTINUUM MAP\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push('\n');
        output.push_str(
            "  v = ∞ (Time/Space)                                    v = ∞ (Space/Time)\n",
        );
        output.push_str(
            "  │ Oneness Dominant                                    │ Many-ness Dominant\n",
        );
        output.push_str("  │ Metaphysical                                         │ Physical\n");
        output.push_str(
            "  │ Inner Experience                                     │ Outer Experience\n",
        );
        output.push('\n');

        // Create spectrum bar
        let bar_width = 60;
        let veil_position = bar_width / 2;

        output.push_str("  ");
        for i in 0..bar_width {
            if i < veil_position {
                output.push('🔵'); // Time/space side
            } else if i == veil_position {
                output.push('📍'); // Veil position
            } else {
                output.push('🟢'); // Space/time side
            }
        }
        output.push('\n');

        output.push_str("  ");
        for i in 0..bar_width {
            if i < veil_position - 2 {
                output.push('─');
            } else if i < veil_position + 2 {
                output.push('│'); // Veil indicator
            } else {
                output.push('─');
            }
        }
        output.push('\n');

        output.push_str("  ");
        output.push('│');
        output.push_str(&" ".repeat(veil_position - 2));
        output.push_str("Veil");
        output.push_str(&" ".repeat(veil_position - 2));
        output.push_str("│\n");

        output.push_str("  ");
        output.push('│');
        output.push_str(&" ".repeat(veil_position - 5));
        output.push_str("(v = 1)");
        output.push_str(&" ".repeat(veil_position - 5));
        output.push_str("│\n\n");

        // Display entity positions
        output.push_str("Entity Positions:\n");
        output.push_str("  ");

        // Sort entities by continuum position
        let mut sorted_positions = positions.to_vec();
        sorted_positions.sort_by(|a, b| {
            a.continuum_position
                .partial_cmp(&b.continuum_position)
                .unwrap()
        });

        // Show entity markers on spectrum
        let mut entity_bar = vec![' '; bar_width];
        for pos in &sorted_positions {
            let bar_index = (pos.continuum_position * bar_width as f64) as usize;
            let bar_index = bar_index.min(bar_width - 1);

            if entity_bar[bar_index] == ' ' {
                if pos.continuum_position < 0.5 {
                    entity_bar[bar_index] = '●'; // Time/space entity
                } else if pos.continuum_position > 0.5 {
                    entity_bar[bar_index] = '○'; // Space/time entity
                } else {
                    entity_bar[bar_index] = '◐'; // Near Veil
                }
            } else if entity_bar[bar_index] != '●'
                && entity_bar[bar_index] != '○'
                && entity_bar[bar_index] != '◐'
            {
                entity_bar[bar_index] = '▓';
            }
        }

        output.push_str("  ");
        for c in entity_bar {
            output.push(c);
        }
        output.push('\n');

        output.push_str("  ");
        output.push('├');
        for i in 1..bar_width - 1 {
            if i == veil_position {
                output.push('┤');
            } else {
                output.push('─');
            }
        }
        output.push('┤');
        output.push('\n');

        output.push_str("  ");
        output.push('│');
        output.push_str(&" ".repeat(veil_position - 10));
        output.push_str("Time/Space");
        output.push_str(&" ".repeat(veil_position - 8));
        output.push_str("Space/Time");
        output.push_str(&" ".repeat(veil_position - 10));
        output.push_str("│\n\n");

        // Legend
        output.push_str("Legend:\n");
        output.push_str("  ● = Entity in Time/Space (v < 1)\n");
        output.push_str("  ◐ = Entity near Veil (v ≈ 1)\n");
        output.push_str("  ○ = Entity in Space/Time (v > 1)\n");
        output.push_str("  ▓ = Multiple entities at same position\n");
        output.push_str("  📍 = Veil position (v = 1)\n\n");

        output
    }

    /// Visualize Veil effect
    fn visualize_veil_effect(positions: &[EntitySpectrumPosition]) -> String {
        let mut output = String::new();

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("VEIL EFFECT ANALYSIS\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push('\n');

        // Count entities on each side of Veil
        let time_space_count = positions
            .iter()
            .filter(|p| p.continuum_position < 0.5)
            .count();

        let veil_count = positions
            .iter()
            .filter(|p| (p.continuum_position - 0.5).abs() < 0.1)
            .count();

        let space_time_count = positions
            .iter()
            .filter(|p| p.continuum_position > 0.5)
            .count();

        let total = positions.len();
        let total_f64 = total as f64;

        if total > 0 {
            output.push_str(&format!(
                "Time/Space Entities  (v < 1): {:>3} ({:>5.1}%)\n",
                time_space_count,
                (time_space_count as f64 / total_f64) * 100.0
            ));
            output.push_str(&format!(
                "Near Veil Entities   (v ≈ 1): {:>3} ({:>5.1}%)\n",
                veil_count,
                (veil_count as f64 / total_f64) * 100.0
            ));
            output.push_str(&format!(
                "Space/Time Entities (v > 1): {:>3} ({:>5.1}%)\n",
                space_time_count,
                (space_time_count as f64 / total_f64) * 100.0
            ));
            output.push('\n');

            // Calculate average veil transparency
            let avg_veil_transparency: f64 =
                positions.iter().map(|p| p.veil_transparency).sum::<f64>() / total_f64;

            output.push_str(&format!(
                "Average Veil Transparency: {:.2} (0.0 = thick veil, 1.0 = no veil)\n",
                avg_veil_transparency
            ));
            output.push('\n');
        }

        // Veil effect explanation
        output.push_str("Veil Effect:\n");
        output.push_str("  • The Veil (v=1) creates illusion of separation\n");
        output.push_str("  • Limits access to oneness side of spectrum\n");
        output.push_str("  • Allows forgetting and learning experience\n");
        output.push_str("  • Thins as entities evolve to higher densities\n");
        output.push_str("  • Completely dissolved in 6th density\n\n");

        output
    }

    /// Visualize entity activities
    fn visualize_entity_activities(positions: &[EntitySpectrumPosition]) -> String {
        let mut output = String::new();

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("ENTITY ACTIVITIES\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push('\n');

        // Count activities
        let mut activity_counts: HashMap<&str, usize> = HashMap::new();
        for pos in positions {
            let activity_name = pos.current_activity.display_name();
            *activity_counts.entry(activity_name).or_insert(0) += 1;
        }

        // Display activity distribution
        output.push_str("Activity Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let activities = vec![
            "Accumulating Experience",
            "Seeking Catalyst",
            "Making Choice",
        ];

        let total = positions.len() as f64;

        for activity in activities {
            let count = *activity_counts.get(activity).unwrap_or(&0);
            let percentage = if total > 0.0 {
                (count as f64 / total) * 100.0
            } else {
                0.0
            };

            let icon = match activity {
                "Accumulating Experience" => "📚",
                "Seeking Catalyst" => "🔍",
                "Making Choice" => "🤔",
                _ => "•",
            };

            output.push_str(&format!(
                "│ {} {:<20}: {:>3} ({:>5.1}%) {:<15} │\n",
                icon,
                activity,
                count,
                percentage,
                Self::activity_bar(percentage)
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Show sample entity activities (first 10)
        output.push_str("Sample Entity Activities (First 10):\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        for (i, pos) in positions.iter().take(10).enumerate() {
            let short_id = format!("{:.6}", pos.entity_id.uuid);
            output.push_str(&format!(
                "│ {:>3}. [{:<8}] {:<55} │\n",
                i + 1,
                short_id,
                pos.current_activity.description()
            ));
        }

        if positions.len() > 10 {
            output.push_str(&format!(
                "│ ... and {} more entities                                          │\n",
                positions.len() - 10
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output
    }

    /// Visualize density distribution
    fn visualize_density_distribution(entities: &HashMap<EntityId, SubSubLogos>) -> String {
        let mut output = String::new();

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("DENSITY DISTRIBUTION\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push('\n');

        // Count entities by density
        let mut density_counts: HashMap<String, usize> = HashMap::new();

        for entity in entities.values() {
            let density_key = format!("{:?}", entity.evolutionary_attractor.current_density);
            *density_counts.entry(density_key).or_insert(0) += 1;
        }

        let total = entities.len() as f64;

        // Display density distribution
        output.push_str("Density Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let densities = vec![
            "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth",
        ];

        for density in densities {
            let count = *density_counts.get(density).unwrap_or(&0);
            let percentage = if total > 0.0 {
                (count as f64 / total) * 100.0
            } else {
                0.0
            };

            let icon = match density {
                "First" => "🔴",
                "Second" => "🟠",
                "Third" => "🟡",
                "Fourth" => "🟢",
                "Fifth" => "🔵",
                "Sixth" => "🟣",
                "Seventh" => "⚪",
                "Eighth" => "🌟",
                _ => "•",
            };

            output.push_str(&format!(
                "│ {} {:<15}: {:>3} ({:>5.1}%) {:<15} │\n",
                icon,
                format!("{} Density", density),
                count,
                percentage,
                Self::activity_bar(percentage)
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output
    }

    /// Visualize spectrum statistics
    fn visualize_spectrum_statistics(positions: &[EntitySpectrumPosition]) -> String {
        let mut output = String::new();

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SPECTRUM STATISTICS\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push('\n');

        if positions.is_empty() {
            output.push_str("No entities to analyze.\n\n");
            return output;
        }

        // Calculate statistics
        let avg_spectrum_ratio: f64 =
            positions.iter().map(|p| p.spectrum_ratio).sum::<f64>() / positions.len() as f64;

        let avg_space_time_dominance: f64 = positions
            .iter()
            .map(|p| p.space_time_dominance)
            .sum::<f64>()
            / positions.len() as f64;

        let avg_time_space_dominance: f64 = positions
            .iter()
            .map(|p| p.time_space_dominance)
            .sum::<f64>()
            / positions.len() as f64;

        let avg_continuum_position: f64 =
            positions.iter().map(|p| p.continuum_position).sum::<f64>() / positions.len() as f64;

        output.push_str(&format!(
            "Average Spectrum Ratio: {:.4}\n",
            avg_spectrum_ratio
        ));
        output.push_str(&format!(
            "Average Continuum Position: {:.4}\n",
            avg_continuum_position
        ));
        output.push_str(&format!(
            "Average Space/Time Dominance: {:.1}%\n",
            avg_space_time_dominance * 100.0
        ));
        output.push_str(&format!(
            "Average Time/Space Dominance: {:.1}%\n",
            avg_time_space_dominance * 100.0
        ));
        output.push('\n');

        // Spectrum balance
        let balance = (avg_space_time_dominance - avg_time_space_dominance).abs();
        output.push_str(&format!(
            "Spectrum Balance: {:.1}% (0% = perfect balance)\n",
            balance * 100.0
        ));
        output.push('\n');

        output
    }

    /// Generate activity bar
    fn activity_bar(percentage: f64) -> String {
        let width = 15;
        let filled = (percentage / 100.0 * width as f64).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push('[');
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in filled..width {
            bar.push('░');
        }
        bar.push(']');

        bar
    }
}
