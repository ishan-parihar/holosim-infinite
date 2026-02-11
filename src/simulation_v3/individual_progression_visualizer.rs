// Individual Entity Progression Visualizer (Phase 3)
//
// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 3 Step 3.3:
// "Visualize individual entity progression"
//
// This module implements visualization for:
// 1. Individual entity progression over time
// 2. Polarity distribution (STO vs STS)
// 3. Evolutionary paths for different entities
// 4. Progression metrics specific to each density

use crate::evolution_density_octave::density_octave::Density;
use crate::simulation_v3::individual_progression::{
    DensitySpecificProgression, EntityPolarity, EntityPolarizationState, EntityProgressionState,
    IndividualProgressionManager, ProgressionStatistics,
};
use std::io::{self, Write};

/// Individual Progression Visualizer
///
/// Provides visualization for individual entity progression.
pub struct IndividualProgressionVisualizer;

impl IndividualProgressionVisualizer {
    /// Visualize entity progression
    pub fn visualize_entity_progression<W: Write>(
        writer: &mut W,
        entity_id: u64,
        progression_state: &EntityProgressionState,
    ) -> io::Result<()> {
        writeln!(
            writer,
            "\n╔══════════════════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║                    INDIVIDUAL ENTITY PROGRESSION                               ║"
        )?;
        writeln!(
            writer,
            "╚══════════════════════════════════════════════════════════════════════════════╝"
        )?;

        writeln!(writer, "\n📊 Entity Information:")?;
        writeln!(writer, "   Entity ID: {}", entity_id)?;
        writeln!(
            writer,
            "   Density: {}",
            Self::format_density(&progression_state.current_density)
        )?;
        writeln!(
            writer,
            "   Progression Score: {:.2}%",
            progression_state.progression_score * 100.0
        )?;
        writeln!(
            writer,
            "   Creation Time: Step {}",
            progression_state.creation_time
        )?;
        writeln!(
            writer,
            "   Last Update: Step {}",
            progression_state.last_update_time
        )?;

        // Visualize density-specific metrics
        writeln!(writer, "\n📈 Density-Specific Progression Metrics:")?;
        Self::visualize_density_metrics(writer, &progression_state.density_specific_metrics)?;

        // Visualize polarization for 3rd Density
        if let Some(polarization) = &progression_state.polarization {
            Self::visualize_polarization(writer, polarization)?;
        }

        writeln!(writer, "\n{}", "─".repeat(80))?;
        Ok(())
    }

    /// Visualize density-specific metrics
    fn visualize_density_metrics<W: Write>(
        writer: &mut W,
        metrics: &DensitySpecificProgression,
    ) -> io::Result<()> {
        match metrics {
            DensitySpecificProgression::FirstDensity {
                physical_interaction_score,
                bonding_formation_count,
                stability_score,
                energy_integration_score,
            } => {
                writeln!(
                    writer,
                    "   🌟 1st Density: Physical Interaction Progression"
                )?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(
                    writer,
                    "   Physical Interaction",
                    *physical_interaction_score,
                )?;
                Self::visualize_progress_bar(writer, "   Stability", *stability_score)?;
                Self::visualize_progress_bar(
                    writer,
                    "   Energy Integration",
                    *energy_integration_score,
                )?;
                writeln!(
                    writer,
                    "   │ Bonding Formations: {}                                                │",
                    bonding_formation_count
                )?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::SecondDensity {
                growth_score,
                survival_score,
                reproduction_count,
                adaptation_score,
            } => {
                writeln!(writer, "   🌱 2nd Density: Growth and Survival Progression")?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Growth", *growth_score)?;
                Self::visualize_progress_bar(writer, "   Survival", *survival_score)?;
                Self::visualize_progress_bar(writer, "   Adaptation", *adaptation_score)?;
                writeln!(
                    writer,
                    "   │ Reproductions: {}                                                       │",
                    reproduction_count
                )?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::ThirdDensity {
                polarization_percentage,
                catalyst_events_processed,
                wisdom_score,
                free_will_exercises,
            } => {
                writeln!(writer, "   🧠 3rd Density: Polarity and Choice Progression")?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Wisdom", *wisdom_score)?;
                writeln!(
                    writer,
                    "   │ Polarity: {:.1}% ({})                                        │",
                    polarization_percentage.abs(),
                    if *polarization_percentage >= 0.0 {
                        "STO"
                    } else {
                        "STS"
                    }
                )?;
                writeln!(
                    writer,
                    "   │ Catalyst Events Processed: {}                                         │",
                    catalyst_events_processed
                )?;
                writeln!(
                    writer,
                    "   │ Free Will Exercises: {}                                               │",
                    free_will_exercises
                )?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::FourthDensity {
                service_score,
                social_memory_complex_score,
                wisdom_score,
                love_understanding_score,
            } => {
                writeln!(
                    writer,
                    "   💚 4th Density: Service and Social Memory Complex Progression"
                )?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Service", *service_score)?;
                Self::visualize_progress_bar(
                    writer,
                    "   Social Memory Complex",
                    *social_memory_complex_score,
                )?;
                Self::visualize_progress_bar(writer, "   Wisdom", *wisdom_score)?;
                Self::visualize_progress_bar(
                    writer,
                    "   Love Understanding",
                    *love_understanding_score,
                )?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::FifthDensity {
                unity_score,
                light_score,
                wisdom_score,
                teaching_ability_score,
            } => {
                writeln!(writer, "   ✨ 5th Density: Unity and Wisdom Progression")?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Unity", *unity_score)?;
                Self::visualize_progress_bar(writer, "   Light", *light_score)?;
                Self::visualize_progress_bar(writer, "   Wisdom", *wisdom_score)?;
                Self::visualize_progress_bar(
                    writer,
                    "   Teaching Ability",
                    *teaching_ability_score,
                )?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::SixthDensity {
                balance_score,
                equality_score,
                harmony_score,
                integration_score,
            } => {
                writeln!(
                    writer,
                    "   ⚖️  6th Density: Balance and Equality Progression"
                )?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Balance", *balance_score)?;
                Self::visualize_progress_bar(writer, "   Equality", *equality_score)?;
                Self::visualize_progress_bar(writer, "   Harmony", *harmony_score)?;
                Self::visualize_progress_bar(writer, "   Integration", *integration_score)?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::SeventhDensity {
                completion_score,
                review_score,
                preparation_score,
                mastery_score,
            } => {
                writeln!(
                    writer,
                    "   🏆 7th Density: Completion and Review Progression"
                )?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Completion", *completion_score)?;
                Self::visualize_progress_bar(writer, "   Review", *review_score)?;
                Self::visualize_progress_bar(writer, "   Preparation", *preparation_score)?;
                Self::visualize_progress_bar(writer, "   Mastery", *mastery_score)?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }

            DensitySpecificProgression::EighthDensity {
                return_score,
                unity_with_creator_score,
                transcendence_score,
                completion_score,
            } => {
                writeln!(writer, "   🌌 8th Density: Return to Creator Progression")?;
                writeln!(
                    writer,
                    "   ┌─────────────────────────────────────────────────────────────────┐"
                )?;
                Self::visualize_progress_bar(writer, "   Return", *return_score)?;
                Self::visualize_progress_bar(
                    writer,
                    "   Unity with Creator",
                    *unity_with_creator_score,
                )?;
                Self::visualize_progress_bar(writer, "   Transcendence", *transcendence_score)?;
                Self::visualize_progress_bar(writer, "   Completion", *completion_score)?;
                writeln!(
                    writer,
                    "   └─────────────────────────────────────────────────────────────────┘"
                )?;
            }
        }

        Ok(())
    }

    /// Visualize polarization state
    fn visualize_polarization<W: Write>(
        writer: &mut W,
        polarization: &EntityPolarizationState,
    ) -> io::Result<()> {
        writeln!(writer, "\n🎭 Polarity State (3rd Density):")?;
        writeln!(
            writer,
            "   ┌─────────────────────────────────────────────────────────────────┐"
        )?;

        // STO alignment
        let sto_bars = (polarization.sto_alignment * 40.0) as usize;
        let sto_bar = "█".repeat(sto_bars) + &"░".repeat(40 - sto_bars);
        writeln!(
            writer,
            "   │ STO Alignment: [{}] {:.1}%                                    │",
            sto_bar,
            polarization.sto_alignment * 100.0
        )?;

        // STS alignment
        let sts_bars = (polarization.sts_alignment * 40.0) as usize;
        let sts_bar = "█".repeat(sts_bars) + &"░".repeat(40 - sts_bars);
        writeln!(
            writer,
            "   │ STS Alignment: [{}] {:.1}%                                    │",
            sts_bar,
            polarization.sts_alignment * 100.0
        )?;

        // Net polarization
        writeln!(
            writer,
            "   ├─────────────────────────────────────────────────────────────────┤"
        )?;
        let polarity_symbol = match polarization.current_polarity {
            Some(EntityPolarity::STO) => "💚 STO",
            Some(EntityPolarity::STS) => "🔴 STS",
            Some(EntityPolarity::Unpolarized) => "⚪ Unpolarized",
            None => "⚪ Unpolarized",
        };
        writeln!(
            writer,
            "   │ Current Polarity: {} (Net: {:.1}%)                        │",
            polarity_symbol, polarization.polarization_percentage
        )?;

        // Harvest readiness
        let harvest_status = if polarization.harvest_ready {
            "✅ READY"
        } else {
            "⏳ NOT READY (requires 51%+)"
        };
        writeln!(
            writer,
            "   │ Harvest Readiness: {}                                                  │",
            harvest_status
        )?;

        writeln!(
            writer,
            "   │ Catalyst Events: {}                                                       │",
            polarization.catalyst_events.len()
        )?;
        writeln!(
            writer,
            "   │ Polarity Choices: {}                                                      │",
            polarization.polarity_choices.len()
        )?;
        writeln!(
            writer,
            "   └─────────────────────────────────────────────────────────────────┘"
        )?;

        Ok(())
    }

    /// Visualize polarization distribution across all 3rd Density entities
    pub fn visualize_polarization_distribution<W: Write>(
        writer: &mut W,
        statistics: &ProgressionStatistics,
    ) -> io::Result<()> {
        writeln!(
            writer,
            "\n╔══════════════════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║                    POLARIZATION DISTRIBUTION (3rd Density)                    ║"
        )?;
        writeln!(
            writer,
            "╚══════════════════════════════════════════════════════════════════════════════╝"
        )?;

        let total = statistics.sto_count + statistics.sts_count + statistics.unpolarized_count;

        if total == 0 {
            writeln!(writer, "\n   No 3rd Density entities tracked.")?;
            return Ok(());
        }

        writeln!(writer, "\n📊 Polarity Distribution:")?;
        writeln!(
            writer,
            "   ┌─────────────────────────────────────────────────────────────────┐"
        )?;

        // STO percentage
        let sto_pct = if total > 0 {
            (statistics.sto_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sto_bars = (sto_pct / 2.5) as usize;
        let sto_bar = "💚".repeat(sto_bars) + &"░".repeat(40 - sto_bars);
        writeln!(
            writer,
            "   │ STO (Service-to-Others): [{}] {} ({:.1}%)                    │",
            sto_bar, statistics.sto_count, sto_pct
        )?;

        // STS percentage
        let sts_pct = if total > 0 {
            (statistics.sts_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sts_bars = (sts_pct / 2.5) as usize;
        let sts_bar = "🔴".repeat(sts_bars) + &"░".repeat(40 - sts_bars);
        writeln!(
            writer,
            "   │ STS (Service-to-Self):   [{}] {} ({:.1}%)                    │",
            sts_bar, statistics.sts_count, sts_pct
        )?;

        // Unpolarized percentage
        let unpolarized_pct = if total > 0 {
            (statistics.unpolarized_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let unpolarized_bars = (unpolarized_pct / 2.5) as usize;
        let unpolarized_bar = "⚪".repeat(unpolarized_bars) + &"░".repeat(40 - unpolarized_bars);
        writeln!(
            writer,
            "   │ Unpolarized:            [{}] {} ({:.1}%)                    │",
            unpolarized_bar, statistics.unpolarized_count, unpolarized_pct
        )?;

        writeln!(
            writer,
            "   └─────────────────────────────────────────────────────────────────┘"
        )?;

        writeln!(writer, "\n   Total 3rd Density Entities: {}", total)?;
        writeln!(
            writer,
            "   Total Polarity Choices Made: {}",
            statistics.total_polarity_choices
        )?;

        writeln!(writer, "\n{}", "─".repeat(80))?;
        Ok(())
    }

    /// Visualize progression by density
    pub fn visualize_density_progression<W: Write>(
        writer: &mut W,
        statistics: &ProgressionStatistics,
    ) -> io::Result<()> {
        writeln!(
            writer,
            "\n╔══════════════════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║                    PROGRESSION BY DENSITY                                   ║"
        )?;
        writeln!(
            writer,
            "╚══════════════════════════════════════════════════════════════════════════════╝"
        )?;

        writeln!(writer, "\n📊 Density Distribution:")?;
        writeln!(
            writer,
            "   ┌─────────────────────────────────────────────────────────────────┐"
        )?;

        for density_name in [
            "1st Density",
            "2nd Density",
            "3rd Density",
            "4th Density",
            "5th Density",
            "6th Density",
            "7th Density",
            "8th Density",
        ] {
            let count = statistics
                .entities_by_density
                .get(density_name)
                .unwrap_or(&0);
            let pct = if statistics.total_entities > 0 {
                (*count as f64 / statistics.total_entities as f64) * 100.0
            } else {
                0.0
            };
            let bars = (pct / 2.5) as usize;
            let bar = "█".repeat(bars) + &"░".repeat(40 - bars);

            writeln!(
                writer,
                "   │ {:14}: [{}] {} ({:.1}%)                    │",
                density_name, bar, count, pct
            )?;
        }

        writeln!(
            writer,
            "   └─────────────────────────────────────────────────────────────────┘"
        )?;

        writeln!(writer, "\n   Total Entities: {}", statistics.total_entities)?;
        writeln!(
            writer,
            "   Average Progression Score: {:.2}%",
            statistics.average_progression_score * 100.0
        )?;

        writeln!(writer, "\n{}", "─".repeat(80))?;
        Ok(())
    }

    /// Visualize full progression summary
    pub fn visualize_progression_summary<W: Write>(
        writer: &mut W,
        manager: &IndividualProgressionManager,
    ) -> io::Result<()> {
        writeln!(
            writer,
            "\n╔══════════════════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║                    INDIVIDUAL PROGRESSION SUMMARY                           ║"
        )?;
        writeln!(
            writer,
            "╚══════════════════════════════════════════════════════════════════════════════╝"
        )?;

        let statistics = manager.get_statistics();

        writeln!(writer, "\n📊 Overall Statistics:")?;
        writeln!(writer, "   Total Entities: {}", statistics.total_entities)?;
        writeln!(
            writer,
            "   Average Progression Score: {:.2}%",
            statistics.average_progression_score * 100.0
        )?;
        writeln!(
            writer,
            "   Total Polarity Choices: {}",
            statistics.total_polarity_choices
        )?;

        // Visualize density progression
        Self::visualize_density_progression(writer, statistics)?;

        // Visualize polarization distribution
        Self::visualize_polarization_distribution(writer, statistics)?;

        writeln!(writer, "\n{}", "─".repeat(80))?;
        Ok(())
    }

    /// Format density for display
    fn format_density(density: &Density) -> String {
        match density {
            Density::First(sub_level) => format!("1st Density ({:?})", sub_level),
            Density::Second(sub_level) => format!("2nd Density ({:?})", sub_level),
            Density::Third => "3rd Density".to_string(),
            Density::Fourth => "4th Density".to_string(),
            Density::Fifth => "5th Density".to_string(),
            Density::Sixth => "6th Density".to_string(),
            Density::Seventh => "7th Density".to_string(),
            Density::Eighth => "8th Density".to_string(),
        }
    }

    /// Visualize progress bar
    fn visualize_progress_bar<W: Write>(writer: &mut W, label: &str, value: f64) -> io::Result<()> {
        let bars = (value * 40.0) as usize;
        let bar = "█".repeat(bars) + &"░".repeat(40 - bars);
        writeln!(
            writer,
            "   │ {:20}: [{}] {:.1}%                                      │",
            label,
            bar,
            value * 100.0
        )?;
        Ok(())
    }
}
