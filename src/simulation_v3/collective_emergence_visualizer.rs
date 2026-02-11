// Collective Emergence Visualizer
//
// Phase 2 Implementation: Visualizes collective system emergence
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Densities are hierarchical material substrates, not individual evolutionary stages"
//
// This module implements:
// 1. Emergence event visualization
// 2. Density distribution visualization
// 3. Emergence progress tracking
// 4. Collective system state visualization
//
// IMPORTANT: This visualizes COLLECTIVE system emergence, not individual entity progression.
// Individual entities DO NOT change density - they progress within their density.

use crate::evolution_density_octave::density_octave::{Density, Density2SubLevel};
use crate::simulation_v3::collective_system::{
    CollectiveSystemManager, DensityDistribution, EmergenceEvent, EmergenceReadiness,
    EmergenceSummary,
};
use std::io::Write;

/// Collective Emergence Visualizer
///
/// Phase 2 Implementation: Visualizes collective system emergence.
///
/// This visualizer provides:
/// 1. Emergence event display
/// 2. Density distribution display
/// 3. Emergence progress display
/// 4. Collective system state display
pub struct CollectiveEmergenceVisualizer;

impl CollectiveEmergenceVisualizer {
    /// Visualize emergence summary
    pub fn visualize_emergence_summary<W: Write>(
        writer: &mut W,
        summary: &EmergenceSummary,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "COLLECTIVE SYSTEM EMERGENCE SUMMARY")?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        // Basic statistics
        writeln!(writer, "Current Step: {}", summary.current_step)?;
        writeln!(
            writer,
            "Emergence Progress: {:.2}%",
            summary.emergence_percentage
        )?;
        writeln!(
            writer,
            "Highest Density Present: {}",
            summary.highest_density
        )?;
        writeln!(
            writer,
            "Densities Emerged: {}/{}",
            summary.densities_emerged, summary.total_densities
        )?;

        // Scores
        writeln!(writer, "\nSystem Quality:")?;
        writeln!(
            writer,
            "  Complexity Score: {:.3}",
            summary.complexity_score
        )?;
        writeln!(writer, "  Coherence Score: {:.3}", summary.coherence_score)?;

        // Emergence progress bar
        writeln!(writer, "\nEmergence Progress:")?;
        Self::visualize_progress_bar(writer, summary.emergence_percentage)?;

        // Emergence events
        writeln!(writer, "\nEmergence Events:")?;
        for event in &summary.emergence_events {
            writeln!(
                writer,
                "  [Step {:4}] {}: {}",
                event.step, event.density, event.description
            )?;
        }

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize density distribution
    pub fn visualize_density_distribution<W: Write>(
        writer: &mut W,
        distribution: &DensityDistribution,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "DENSITY DISTRIBUTION")?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        let total = distribution.total_entities();
        writeln!(writer, "Total Entities: {}\n", total)?;

        // 1st Density
        writeln!(writer, "1st Density (Red Ray):")?;
        writeln!(
            writer,
            "  Quantum Realm:      {:5} ({:5.1}%)",
            distribution.first_density_quantum,
            Self::percentage(distribution.first_density_quantum, total)
        )?;
        writeln!(
            writer,
            "  Atomic Realm:       {:5} ({:5.1}%)",
            distribution.first_density_atomic,
            Self::percentage(distribution.first_density_atomic, total)
        )?;
        writeln!(
            writer,
            "  Molecular Realm:    {:5} ({:5.1}%)",
            distribution.first_density_molecular,
            Self::percentage(distribution.first_density_molecular, total)
        )?;
        writeln!(
            writer,
            "  Planetary Realm:    {:5} ({:5.1}%)",
            distribution.first_density_planetary,
            Self::percentage(distribution.first_density_planetary, total)
        )?;
        writeln!(
            writer,
            "  1st Density Total:  {:5} ({:5.1}%)",
            distribution.first_density_total(),
            Self::percentage(distribution.first_density_total(), total)
        )?;

        // 2nd Density
        writeln!(writer, "\n2nd Density (Orange Ray):")?;
        writeln!(
            writer,
            "  Cellular Realm:     {:5} ({:5.1}%)",
            distribution.second_density_cellular,
            Self::percentage(distribution.second_density_cellular, total)
        )?;
        writeln!(
            writer,
            "  Simple Life Realm:  {:5} ({:5.1}%)",
            distribution.second_density_simple_life,
            Self::percentage(distribution.second_density_simple_life, total)
        )?;
        writeln!(
            writer,
            "  Complex Life Realm: {:5} ({:5.1}%)",
            distribution.second_density_complex_life,
            Self::percentage(distribution.second_density_complex_life, total)
        )?;
        writeln!(
            writer,
            "  2nd Density Total:  {:5} ({:5.1}%)",
            distribution.second_density_total(),
            Self::percentage(distribution.second_density_total(), total)
        )?;

        // Higher densities
        writeln!(writer, "\nHigher Densities:")?;
        writeln!(
            writer,
            "  3rd Density (Yellow Ray): {:5} ({:5.1}%)",
            distribution.third_density,
            Self::percentage(distribution.third_density, total)
        )?;
        writeln!(
            writer,
            "  4th Density (Green Ray):  {:5} ({:5.1}%)",
            distribution.fourth_density,
            Self::percentage(distribution.fourth_density, total)
        )?;
        writeln!(
            writer,
            "  5th Density (Blue Ray):   {:5} ({:5.1}%)",
            distribution.fifth_density,
            Self::percentage(distribution.fifth_density, total)
        )?;
        writeln!(
            writer,
            "  6th Density (Indigo-Ray): {:5} ({:5.1}%)",
            distribution.sixth_density,
            Self::percentage(distribution.sixth_density, total)
        )?;
        writeln!(
            writer,
            "  7th Density (Violet-Ray): {:5} ({:5.1}%)",
            distribution.seventh_density,
            Self::percentage(distribution.seventh_density, total)
        )?;
        writeln!(
            writer,
            "  8th Density (White Light):{:5} ({:5.1}%)",
            distribution.eighth_density,
            Self::percentage(distribution.eighth_density, total)
        )?;

        // Visual representation
        writeln!(writer, "\nVisual Representation:")?;
        Self::visualize_density_distribution_bar(writer, distribution)?;

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize emergence readiness
    pub fn visualize_emergence_readiness<W: Write>(
        writer: &mut W,
        target_density: Density,
        readiness: &EmergenceReadiness,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "EMERGENCE READINESS: {:?}", target_density)?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        writeln!(
            writer,
            "Status: {}",
            if readiness.is_ready {
                "READY"
            } else {
                "NOT READY"
            }
        )?;
        writeln!(
            writer,
            "Current Progress: {:.2}% (Required: {:.2}%)",
            readiness.current_percentage, readiness.required_percentage
        )?;

        // Progress bar
        writeln!(writer, "\nProgress:")?;
        Self::visualize_progress_bar(writer, readiness.current_percentage)?;

        // Conditions met
        if !readiness.conditions_met.is_empty() {
            writeln!(writer, "\n✓ Conditions Met:")?;
            for condition in &readiness.conditions_met {
                writeln!(writer, "  ✓ {}", condition)?;
            }
        }

        // Conditions missing
        if !readiness.conditions_missing.is_empty() {
            writeln!(writer, "\n✗ Conditions Missing:")?;
            for condition in &readiness.conditions_missing {
                writeln!(writer, "  ✗ {}", condition)?;
            }
        }

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize emergence event
    pub fn visualize_emergence_event<W: Write>(
        writer: &mut W,
        event: &EmergenceEvent,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "EMERGENCE EVENT DETECTED")?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        writeln!(writer, "Density: {}", event.density)?;
        writeln!(writer, "Step: {}", event.step)?;
        writeln!(writer, "Description: {}", event.description)?;

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize progress bar
    fn visualize_progress_bar<W: Write>(writer: &mut W, percentage: f64) -> std::io::Result<()> {
        let bar_width = 50;
        let filled = (percentage / 100.0 * bar_width as f64).round() as usize;
        let empty = bar_width - filled;

        write!(writer, "  [")?;
        for _ in 0..filled {
            write!(writer, "█")?;
        }
        for _ in 0..empty {
            write!(writer, "░")?;
        }
        writeln!(writer, "] {:.2}%", percentage)?;

        Ok(())
    }

    /// Visualize density distribution as a horizontal bar
    fn visualize_density_distribution_bar<W: Write>(
        writer: &mut W,
        distribution: &DensityDistribution,
    ) -> std::io::Result<()> {
        let total = distribution.total_entities();
        if total == 0 {
            writeln!(writer, "  (No entities)")?;
            return Ok(());
        }

        let max_bar_width = 40;

        // 1st Density
        Self::write_density_bar(
            writer,
            "1st",
            distribution.first_density_total(),
            total,
            max_bar_width,
        )?;

        // 2nd Density
        Self::write_density_bar(
            writer,
            "2nd",
            distribution.second_density_total(),
            total,
            max_bar_width,
        )?;

        // Higher densities
        Self::write_density_bar(
            writer,
            "3rd",
            distribution.third_density,
            total,
            max_bar_width,
        )?;
        Self::write_density_bar(
            writer,
            "4th",
            distribution.fourth_density,
            total,
            max_bar_width,
        )?;
        Self::write_density_bar(
            writer,
            "5th",
            distribution.fifth_density,
            total,
            max_bar_width,
        )?;
        Self::write_density_bar(
            writer,
            "6th",
            distribution.sixth_density,
            total,
            max_bar_width,
        )?;
        Self::write_density_bar(
            writer,
            "7th",
            distribution.seventh_density,
            total,
            max_bar_width,
        )?;
        Self::write_density_bar(
            writer,
            "8th",
            distribution.eighth_density,
            total,
            max_bar_width,
        )?;

        Ok(())
    }

    /// Write a single density bar
    fn write_density_bar<W: Write>(
        writer: &mut W,
        label: &str,
        count: usize,
        total: usize,
        max_width: usize,
    ) -> std::io::Result<()> {
        let percentage = Self::percentage(count, total);
        let bar_length = (percentage / 100.0 * max_width as f64).round() as usize;

        write!(writer, "  {:3}: [", label)?;
        for _ in 0..bar_length {
            write!(writer, "█")?;
        }
        for _ in bar_length..max_width {
            write!(writer, "░")?;
        }
        writeln!(writer, "] {:5} ({:5.1}%)", count, percentage)?;

        Ok(())
    }

    /// Calculate percentage
    fn percentage(count: usize, total: usize) -> f64 {
        if total == 0 {
            0.0
        } else {
            (count as f64 / total as f64) * 100.0
        }
    }

    /// Visualize collective system state
    pub fn visualize_collective_state<W: Write>(
        writer: &mut W,
        manager: &CollectiveSystemManager,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "COLLECTIVE SYSTEM STATE")?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        let state = manager.collective_state();

        writeln!(writer, "Current Step: {}", state.current_step)?;
        writeln!(
            writer,
            "Highest Density: {:?}",
            state.highest_density_present
        )?;
        writeln!(
            writer,
            "Emergence Percentage: {:.2}%",
            state.emergence_percentage
        )?;
        writeln!(writer, "Complexity Score: {:.3}", state.complexity_score)?;
        writeln!(writer, "Coherence Score: {:.3}", state.coherence_score)?;

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize emergence history
    pub fn visualize_emergence_history<W: Write>(
        writer: &mut W,
        manager: &CollectiveSystemManager,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n{}", "=".repeat(80))?;
        writeln!(writer, "EMERGENCE HISTORY")?;
        writeln!(writer, "{}\n", "=".repeat(80))?;

        let history = manager.emergence_history();

        writeln!(
            writer,
            "1st Density: {}",
            if history.first_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "2nd Density: {}",
            if history.second_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "3rd Density: {}",
            if history.third_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "4th Density: {}",
            if history.fourth_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "5th Density: {}",
            if history.fifth_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "6th Density: {}",
            if history.sixth_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "7th Density: {}",
            if history.seventh_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;
        writeln!(
            writer,
            "8th Density: {}",
            if history.eighth_density_emerged {
                "✓ Emerged"
            } else {
                "✗ Not Emerged"
            }
        )?;

        // Emergence steps
        writeln!(writer, "\nEmergence Steps:")?;
        if let Some(step) = history.second_density_emergence_step {
            writeln!(writer, "  2nd Density: Step {}", step)?;
        }
        if let Some(step) = history.third_density_emergence_step {
            writeln!(writer, "  3rd Density: Step {}", step)?;
        }
        if let Some(step) = history.fourth_density_emergence_step {
            writeln!(writer, "  4th Density: Step {}", step)?;
        }
        if let Some(step) = history.fifth_density_emergence_step {
            writeln!(writer, "  5th Density: Step {}", step)?;
        }
        if let Some(step) = history.sixth_density_emergence_step {
            writeln!(writer, "  6th Density: Step {}", step)?;
        }
        if let Some(step) = history.seventh_density_emergence_step {
            writeln!(writer, "  7th Density: Step {}", step)?;
        }
        if let Some(step) = history.eighth_density_emergence_step {
            writeln!(writer, "  8th Density: Step {}", step)?;
        }

        writeln!(writer, "{}\n", "=".repeat(80))?;

        Ok(())
    }

    /// Visualize full collective emergence report
    pub fn visualize_full_report<W: Write>(
        writer: &mut W,
        manager: &CollectiveSystemManager,
    ) -> std::io::Result<()> {
        // Emergence summary
        let summary = manager.emergence_summary();
        Self::visualize_emergence_summary(writer, &summary)?;

        // Density distribution
        Self::visualize_density_distribution(writer, manager.density_distribution())?;

        // Collective state
        Self::visualize_collective_state(writer, manager)?;

        // Emergence history
        Self::visualize_emergence_history(writer, manager)?;

        // Check readiness for next density
        let current_highest = manager.collective_state().highest_density_present.clone();
        let next_density = match current_highest {
            Density::First(..) => Density::Second(Density2SubLevel::Cellular),
            Density::Second(..) => Density::Third,
            Density::Third => Density::Fourth,
            Density::Fourth => Density::Fifth,
            Density::Fifth => Density::Sixth,
            Density::Sixth => Density::Seventh,
            Density::Seventh => Density::Eighth,
            Density::Eighth => {
                writeln!(
                    writer,
                    "\nAll densities have emerged. The octave is complete."
                )?;
                return Ok(());
            }
        };

        let readiness = manager.check_emergence_conditions(next_density.clone());
        Self::visualize_emergence_readiness(writer, next_density, &readiness)?;

        Ok(())
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_percentage() {
        assert_eq!(CollectiveEmergenceVisualizer::percentage(0, 100), 0.0);
        assert_eq!(CollectiveEmergenceVisualizer::percentage(50, 100), 50.0);
        assert_eq!(CollectiveEmergenceVisualizer::percentage(100, 100), 100.0);
        assert_eq!(CollectiveEmergenceVisualizer::percentage(0, 0), 0.0);
    }

    #[test]
    fn test_visualize_emergence_event() {
        let event = EmergenceEvent {
            density: "2nd Density".to_string(),
            step: 10,
            description: "Biological life emerges from planetary structures".to_string(),
        };

        let mut buffer = Vec::new();
        CollectiveEmergenceVisualizer::visualize_emergence_event(&mut buffer, &event).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("2nd Density"));
        assert!(output.contains("Step 10"));
        assert!(output.contains("Biological life emerges"));
    }
}
