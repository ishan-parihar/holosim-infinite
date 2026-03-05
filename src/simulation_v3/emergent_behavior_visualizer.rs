//! Emergent Behavior Visualizer
//!
//! This module provides visualization tools for emergent behavior,
//! including system-level emergence and environmental emergence.

use crate::simulation_v3::emergent_behavior::{
    EmergenceHistory, EmergenceManager, EmergenceStatistics, EmergentEvent, EnvironmentalEmergence,
    SystemEmergence,
};
use std::io::Write;

/// Visualizer for emergent behavior
pub struct EmergentBehaviorVisualizer;

impl EmergentBehaviorVisualizer {
    /// Visualize system emergence
    pub fn visualize_system_emergence<W: Write>(
        writer: &mut W,
        emergence: &SystemEmergence,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "                 SYSTEM-LEVEL EMERGENCE")?;
        writeln!(
            writer,
            "═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "Timestamp: {}", emergence.timestamp)?;
        writeln!(
            writer,
            "───────────────────────────────────────────────────────────────"
        )?;

        // Visualize global coherence
        writeln!(writer, "\n🌐 GLOBAL COHERENCE")?;
        writeln!(writer, "   Value: {:.4}", emergence.global_coherence)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.global_coherence, "█", "░")?;

        // Visualize collective consciousness
        writeln!(writer, "\n🧠 COLLECTIVE CONSCIOUSNESS")?;
        writeln!(
            writer,
            "   Strength: {:.4}",
            emergence.collective_consciousness_strength
        )?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.collective_consciousness_strength,
            "█",
            "░",
        )?;

        // Visualize system resonance
        writeln!(writer, "\n🎵 SYSTEM RESONANCE")?;
        writeln!(writer, "   Value: {:.4}", emergence.system_resonance)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.system_resonance, "█", "░")?;

        // Visualize emergent intelligence
        writeln!(writer, "\n💡 EMERGENT INTELLIGENCE")?;
        writeln!(writer, "   Level: {:.4}", emergence.emergent_intelligence)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.emergent_intelligence, "█", "░")?;

        // Visualize system complexity
        writeln!(writer, "\n🔮 SYSTEM COMPLEXITY")?;
        writeln!(writer, "   Entropy: {:.4}", emergence.system_complexity)?;
        Self::visualize_bar(writer, 0.0, 4.0, emergence.system_complexity, "█", "░")?;

        // Visualize self-organization
        writeln!(writer, "\n🔄 SELF-ORGANIZATION")?;
        writeln!(writer, "   Level: {:.4}", emergence.self_organization)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.self_organization, "█", "░")?;

        // Visualize resilience
        writeln!(writer, "\n🛡️  RESILIENCE")?;
        writeln!(writer, "   Level: {:.4}", emergence.resilience)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.resilience, "█", "░")?;

        // Visualize criticality
        writeln!(writer, "\n⚡ CRITICALITY")?;
        writeln!(writer, "   Level: {:.4}", emergence.criticality)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.criticality, "█", "░")?;

        // Interpretation
        writeln!(writer, "\n📊 INTERPRETATION")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;

        if emergence.global_coherence >= 0.8 {
            writeln!(writer, "   ✓ System is highly coherent and aligned")?;
        } else if emergence.global_coherence >= 0.5 {
            writeln!(writer, "   ~ System shows moderate coherence")?;
        } else {
            writeln!(writer, "   ✗ System has low coherence")?;
        }

        if emergence.criticality >= 0.8 {
            writeln!(writer, "   ⚠ System is near critical phase transition")?;
        } else if emergence.criticality >= 0.5 {
            writeln!(writer, "   ~ System is moderately critical")?;
        } else {
            writeln!(writer, "   ✓ System is stable")?;
        }

        if emergence.resilience >= 0.7 {
            writeln!(writer, "   ✓ System has high resilience")?;
        } else {
            writeln!(writer, "   ~ System resilience could be improved")?;
        }

        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;

        Ok(())
    }

    /// Visualize environmental emergence
    pub fn visualize_environmental_emergence<W: Write>(
        writer: &mut W,
        emergence: &EnvironmentalEmergence,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "               ENVIRONMENTAL EMERGENCE")?;
        writeln!(
            writer,
            "═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "Timestamp: {}", emergence.timestamp)?;
        writeln!(
            writer,
            "───────────────────────────────────────────────────────────────"
        )?;

        // Visualize environment emergence level
        writeln!(writer, "\n🌍 ENVIRONMENT EMERGENCE LEVEL")?;
        writeln!(
            writer,
            "   Value: {:.4}",
            emergence.environment_emergence_level
        )?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.environment_emergence_level,
            "█",
            "░",
        )?;

        // Visualize planetary formation
        writeln!(writer, "\n🪐 PLANETARY FORMATION")?;
        writeln!(writer, "   Level: {:.4}", emergence.planetary_formation)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.planetary_formation, "█", "░")?;

        // Visualize galactic formation
        writeln!(writer, "\n🌌 GALACTIC FORMATION")?;
        writeln!(writer, "   Level: {:.4}", emergence.galactic_formation)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.galactic_formation, "█", "░")?;

        // Visualize environmental stability
        writeln!(writer, "\n⚖️  ENVIRONMENTAL STABILITY")?;
        writeln!(writer, "   Level: {:.4}", emergence.environmental_stability)?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.environmental_stability,
            "█",
            "░",
        )?;

        // Visualize environmental diversity
        writeln!(writer, "\n🎨 ENVIRONMENTAL DIVERSITY")?;
        writeln!(writer, "   Level: {:.4}", emergence.environmental_diversity)?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.environmental_diversity,
            "█",
            "░",
        )?;

        // Visualize entity-environment integration
        writeln!(writer, "\n🔗 ENTITY-ENVIRONMENT INTEGRATION")?;
        writeln!(
            writer,
            "   Level: {:.4}",
            emergence.entity_environment_integration
        )?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.entity_environment_integration,
            "█",
            "░",
        )?;

        // Visualize environmental influence
        writeln!(writer, "\n🌊 ENVIRONMENTAL INFLUENCE")?;
        writeln!(writer, "   Level: {:.4}", emergence.environmental_influence)?;
        Self::visualize_bar(
            writer,
            0.0,
            1.0,
            emergence.environmental_influence,
            "█",
            "░",
        )?;

        // Visualize co-evolution strength
        writeln!(writer, "\n🔄 CO-EVOLUTION STRENGTH")?;
        writeln!(writer, "   Level: {:.4}", emergence.co_evolution_strength)?;
        Self::visualize_bar(writer, 0.0, 1.0, emergence.co_evolution_strength, "█", "░")?;

        // Interpretation
        writeln!(writer, "\n📊 INTERPRETATION")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;

        if emergence.environment_emergence_level >= 0.7 {
            writeln!(
                writer,
                "   ✓ Environment has significantly emerged from 1st Density"
            )?;
        } else if emergence.environment_emergence_level >= 0.4 {
            writeln!(writer, "   ~ Environment is partially emerged")?;
        } else {
            writeln!(writer, "   ✗ Environment emergence is still early")?;
        }

        if emergence.planetary_formation >= 0.7 {
            writeln!(writer, "   ✓ Planetary structures are well-formed")?;
        } else {
            writeln!(writer, "   ~ Planetary formation is in progress")?;
        }

        if emergence.co_evolution_strength >= 0.7 {
            writeln!(
                writer,
                "   ✓ Strong co-evolution between environment and entities"
            )?;
        } else {
            writeln!(writer, "   ~ Co-evolution is developing")?;
        }

        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;

        Ok(())
    }

    /// Visualize emergent event
    pub fn visualize_emergent_event<W: Write>(
        writer: &mut W,
        event: &EmergentEvent,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "                  EMERGENT EVENT")?;
        writeln!(
            writer,
            "═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "Timestamp: {}", event.timestamp)?;
        writeln!(
            writer,
            "───────────────────────────────────────────────────────────────"
        )?;

        // Event type
        writeln!(writer, "\n🎯 EVENT TYPE")?;
        writeln!(writer, "   {:?}", event.event_type)?;

        // Description
        writeln!(writer, "\n📝 DESCRIPTION")?;
        writeln!(writer, "   {}", event.description)?;

        // Magnitude
        writeln!(writer, "\n📊 MAGNITUDE")?;
        writeln!(writer, "   {:.4}", event.magnitude)?;
        Self::visualize_bar(writer, 0.0, 1.0, event.magnitude, "█", "░")?;

        // Affected entities
        writeln!(writer, "\n🔗 AFFECTED ENTITIES")?;
        writeln!(writer, "   Count: {}", event.affected_entities.len())?;

        // System state change
        writeln!(writer, "\n🔄 SYSTEM STATE CHANGE")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;
        writeln!(writer, "   Before:")?;
        writeln!(
            writer,
            "      Coherence: {:.4}",
            event.system_state_before.global_coherence
        )?;
        writeln!(
            writer,
            "      Criticality: {:.4}",
            event.system_state_before.criticality
        )?;
        writeln!(writer, "   After:")?;
        writeln!(
            writer,
            "      Coherence: {:.4}",
            event.system_state_after.global_coherence
        )?;
        writeln!(
            writer,
            "      Criticality: {:.4}",
            event.system_state_after.criticality
        )?;

        // Environmental state change (if applicable)
        if let (Some(before), Some(after)) = (
            &event.environmental_state_before,
            &event.environmental_state_after,
        ) {
            writeln!(writer, "\n🌍 ENVIRONMENTAL STATE CHANGE")?;
            writeln!(
                writer,
                "   ─────────────────────────────────────────────────────"
            )?;
            writeln!(writer, "   Before:")?;
            writeln!(
                writer,
                "      Emergence: {:.4}",
                before.environment_emergence_level
            )?;
            writeln!(
                writer,
                "      Co-evolution: {:.4}",
                before.co_evolution_strength
            )?;
            writeln!(writer, "   After:")?;
            writeln!(
                writer,
                "      Emergence: {:.4}",
                after.environment_emergence_level
            )?;
            writeln!(
                writer,
                "      Co-evolution: {:.4}",
                after.co_evolution_strength
            )?;
        }

        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;

        Ok(())
    }

    /// Visualize emergence history
    pub fn visualize_emergence_history<W: Write>(
        writer: &mut W,
        history: &EmergenceHistory,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "              EMERGENCE HISTORY OVERVIEW")?;
        writeln!(
            writer,
            "═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(
            writer,
            "───────────────────────────────────────────────────────────────"
        )?;

        // Summary statistics
        writeln!(writer, "\n📊 SUMMARY STATISTICS")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;
        writeln!(
            writer,
            "   System measurements: {}",
            history.system_history.len()
        )?;
        writeln!(
            writer,
            "   Environmental measurements: {}",
            history.environmental_history.len()
        )?;
        writeln!(writer, "   Emergent events: {}", history.events.len())?;

        // Global coherence trend
        if !history.system_history.is_empty() {
            writeln!(writer, "\n🌈 GLOBAL COHERENCE TREND")?;
            writeln!(
                writer,
                "   ─────────────────────────────────────────────────────"
            )?;

            let first = history.system_history.first().unwrap().global_coherence;
            let last = history.system_history.last().unwrap().global_coherence;

            writeln!(writer, "   First: {:.4}", first)?;
            writeln!(writer, "   Last:  {:.4}", last)?;
            writeln!(writer, "   Change: {:+.4}", last - first)?;

            if last > first {
                writeln!(writer, "   ✓ Coherence is increasing")?;
            } else if last < first {
                writeln!(writer, "   ✗ Coherence is decreasing")?;
            } else {
                writeln!(writer, "   ~ Coherence is stable")?;
            }
        }

        // Criticality trend
        if !history.system_history.is_empty() {
            writeln!(writer, "\n⚡ CRITICALITY TREND")?;
            writeln!(
                writer,
                "   ─────────────────────────────────────────────────────"
            )?;

            let first = history.system_history.first().unwrap().criticality;
            let last = history.system_history.last().unwrap().criticality;

            writeln!(writer, "   First: {:.4}", first)?;
            writeln!(writer, "   Last:  {:.4}", last)?;
            writeln!(writer, "   Change: {:+.4}", last - first)?;

            if last > first {
                writeln!(writer, "   ⚠ System is becoming more critical")?;
            } else if last < first {
                writeln!(writer, "   ✓ System is becoming more stable")?;
            } else {
                writeln!(writer, "   ~ System criticality is stable")?;
            }
        }

        // Environmental emergence trend
        if !history.environmental_history.is_empty() {
            writeln!(writer, "\n🌍 ENVIRONMENTAL EMERGENCE TREND")?;
            writeln!(
                writer,
                "   ─────────────────────────────────────────────────────"
            )?;

            let first = history
                .environmental_history
                .first()
                .unwrap()
                .environment_emergence_level;
            let last = history
                .environmental_history
                .last()
                .unwrap()
                .environment_emergence_level;

            writeln!(writer, "   First: {:.4}", first)?;
            writeln!(writer, "   Last:  {:.4}", last)?;
            writeln!(writer, "   Change: {:+.4}", last - first)?;

            if last > first {
                writeln!(writer, "   ✓ Environment is emerging")?;
            } else if last < first {
                writeln!(writer, "   ✗ Environment emergence is declining")?;
            } else {
                writeln!(writer, "   ~ Environment emergence is stable")?;
            }
        }

        // Recent events
        if !history.events.is_empty() {
            writeln!(writer, "\n🎯 RECENT EMERGENT EVENTS")?;
            writeln!(
                writer,
                "   ─────────────────────────────────────────────────────"
            )?;

            let recent_count = 5.min(history.events.len());
            for event in history.events.iter().rev().take(recent_count) {
                writeln!(writer, "   [{}] {:?}", event.timestamp, event.event_type)?;
                writeln!(writer, "      {}", event.description)?;
            }
        }

        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;

        Ok(())
    }

    /// Visualize emergence statistics
    pub fn visualize_emergence_statistics<W: Write>(
        writer: &mut W,
        stats: &EmergenceStatistics,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(writer, "              EMERGENCE STATISTICS")?;
        writeln!(
            writer,
            "═══════════════════════════════════════════════════════════════"
        )?;
        writeln!(
            writer,
            "───────────────────────────────────────────────────────────────"
        )?;

        // System-level statistics
        writeln!(writer, "\n🌐 SYSTEM-LEVEL STATISTICS")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;
        writeln!(
            writer,
            "   Average Global Coherence: {:.4}",
            stats.avg_global_coherence
        )?;
        writeln!(
            writer,
            "   Peak Global Coherence:   {:.4}",
            stats.peak_global_coherence
        )?;
        writeln!(
            writer,
            "   Avg Collective Consciousness: {:.4}",
            stats.avg_collective_consciousness
        )?;
        writeln!(
            writer,
            "   Complexity Growth Rate:    {:.6}",
            stats.complexity_growth_rate
        )?;

        // Event statistics
        writeln!(writer, "\n🎯 EMERGENT EVENT STATISTICS")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;
        writeln!(writer, "   Total Events: {}", stats.total_events)?;

        if !stats.events_by_type.is_empty() {
            writeln!(writer, "\n   Events by Type:")?;
            for (event_type, count) in &stats.events_by_type {
                writeln!(writer, "      {:?}: {}", event_type, count)?;
            }
        }

        // Environmental statistics
        writeln!(writer, "\n🌍 ENVIRONMENTAL STATISTICS")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;
        writeln!(
            writer,
            "   Final Environmental Emergence: {:.4}",
            stats.final_environment_emergence
        )?;
        writeln!(
            writer,
            "   Final Co-evolution Strength:   {:.4}",
            stats.final_co_evolution_strength
        )?;

        // Interpretation
        writeln!(writer, "\n📊 INTERPRETATION")?;
        writeln!(
            writer,
            "   ─────────────────────────────────────────────────────"
        )?;

        if stats.avg_global_coherence >= 0.7 {
            writeln!(writer, "   ✓ System maintained high coherence")?;
        } else {
            writeln!(writer, "   ~ System coherence was moderate")?;
        }

        if stats.complexity_growth_rate > 0.0 {
            writeln!(writer, "   ✓ System complexity grew over time")?;
        } else {
            writeln!(writer, "   ~ System complexity was stable")?;
        }

        if stats.total_events > 0 {
            writeln!(writer, "   ✓ System experienced emergent events")?;
        } else {
            writeln!(writer, "   ~ No significant emergent events recorded")?;
        }

        writeln!(
            writer,
            "\n═══════════════════════════════════════════════════════════════"
        )?;

        Ok(())
    }

    /// Visualize full emergence report
    pub fn visualize_full_report<W: Write>(
        writer: &mut W,
        manager: &EmergenceManager,
    ) -> std::io::Result<()> {
        // System emergence
        Self::visualize_system_emergence(writer, manager.current_system_emergence())?;

        // Environmental emergence
        Self::visualize_environmental_emergence(writer, manager.current_environmental_emergence())?;

        // History
        Self::visualize_emergence_history(writer, manager.history())?;

        // Statistics
        let stats = manager.calculate_statistics();
        Self::visualize_emergence_statistics(writer, &stats)?;

        Ok(())
    }

    /// Visualize a bar chart
    fn visualize_bar<W: Write>(
        writer: &mut W,
        min: f64,
        max: f64,
        value: f64,
        filled: &str,
        empty: &str,
    ) -> std::io::Result<()> {
        let range = max - min;
        if range <= 0.0 {
            return Ok(());
        }

        let normalized = ((value - min) / range).clamp(0.0, 1.0);
        let bar_length = 40;
        let filled_chars = (normalized * bar_length as f64) as usize;
        let empty_chars = bar_length - filled_chars;

        write!(writer, "   [")?;
        for _ in 0..filled_chars {
            write!(writer, "{}", filled)?;
        }
        for _ in 0..empty_chars {
            write!(writer, "{}", empty)?;
        }
        writeln!(writer, "] {:.2}%", normalized * 100.0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_exists() {
        // Test that visualizer compiles
        let mut buffer = Vec::new();
        let emergence = SystemEmergence::default();
        let result =
            EmergentBehaviorVisualizer::visualize_system_emergence(&mut buffer, &emergence);
        assert!(result.is_ok());
    }
}
