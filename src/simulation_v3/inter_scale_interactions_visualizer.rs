// Inter-Scale Interactions Visualizer
//
// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md Phase 4:
// "Visualize inter-scale interactions"
//
// This module provides visualization for:
// 1. Composition interactions between entities at different scales
// 2. Holographic connections (resonant, entangled, harmonic)
// 3. Collective influence between collectives and members
// 4. Co-evolution tracking

use crate::simulation_v3::inter_scale_interactions::{
    CoEvolutionTracking, CollectiveInfluence, CompositionEffect, EntityInteraction,
    HolographicConnection, HolographicConnectionType, HolographicSimilarity,
    InterScaleInteractionManager, InteractionStatistics, InteractionType,
};
use std::io::Write;

/// Inter-Scale Interactions Visualizer
///
/// Provides visualization for all inter-scale interactions.
pub struct InterScaleInteractionsVisualizer;

impl InterScaleInteractionsVisualizer {
    /// Visualize composition effects for an entity
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "How do quantum particles affect atoms?"
    /// "How do atoms affect molecules?"
    /// "How do molecules affect cells?"
    /// "How do cells affect organisms?"
    /// "How do organisms affect beings?"
    pub fn visualize_composition_effects<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        effects: &[&CompositionEffect],
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== Composition Effects for Entity {:?} ===",
            entity_id
        )?;
        writeln!(writer, "Total Effects: {}", effects.len())?;

        if effects.is_empty() {
            writeln!(writer, "No composition effects found.")?;
            return Ok(());
        }

        writeln!(
            writer,
            "\n{:<40} {:<15} {:<15} {:<15}",
            "Component", "Influence", "Stability", "Energy"
        )?;
        writeln!(writer, "{:-<90}", "")?;

        for effect in effects {
            let component_id = format!("{:?}", effect.component_id);
            let influence = format!("{:.2}", effect.influence * 100.0);
            let stability = format!("{:.2}", effect.stability_contribution * 100.0);
            let energy = format!("{:.2}", effect.energy_contribution * 100.0);

            writeln!(
                writer,
                "{:<40} {:<15}% {:<15}% {:<15}%",
                component_id, influence, stability, energy
            )?;
        }

        // Calculate averages
        let avg_influence: f64 =
            effects.iter().map(|e| e.influence).sum::<f64>() / effects.len() as f64;
        let avg_stability: f64 = effects
            .iter()
            .map(|e| e.stability_contribution)
            .sum::<f64>()
            / effects.len() as f64;
        let avg_energy: f64 =
            effects.iter().map(|e| e.energy_contribution).sum::<f64>() / effects.len() as f64;

        writeln!(writer, "\n{:-<90}", "")?;
        writeln!(
            writer,
            "{:<40} {:<15}% {:<15}% {:<15}%",
            "AVERAGE",
            format!("{:.2}", avg_influence * 100.0),
            format!("{:.2}", avg_stability * 100.0),
            format!("{:.2}", avg_energy * 100.0)
        )?;

        Ok(())
    }

    /// Visualize holographic similarity between entities
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Each scale contains the pattern of the whole"
    /// "Universal, galactic, and individual scales are holographically related"
    pub fn visualize_holographic_similarity<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        similarities: Vec<&HolographicSimilarity>,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== Holographic Similarity for Entity {:?} ===",
            entity_id
        )?;
        writeln!(writer, "Total Similarities: {}", similarities.len())?;

        if similarities.is_empty() {
            writeln!(writer, "No holographic similarities found.")?;
            return Ok(());
        }

        // Sort by overall similarity (descending)
        let mut sorted_similarities = similarities.clone();
        sorted_similarities.sort_by(|a, b| {
            b.overall_similarity
                .partial_cmp(&a.overall_similarity)
                .unwrap()
        });

        writeln!(
            writer,
            "\n{:<40} {:<15} {:<15} {:<20} {:<15}",
            "Entity", "Archetype", "Spectrum", "Holographic", "Overall"
        )?;
        writeln!(writer, "{:-<130}", "")?;

        for similarity in sorted_similarities {
            let other_id = if similarity.entity_id_1 == *entity_id {
                &similarity.entity_id_2
            } else {
                &similarity.entity_id_1
            };

            let entity_str = format!("{:?}", other_id);
            let archetype = format!("{:.2}", similarity.archetype_similarity * 100.0);
            let spectrum = format!("{:.2}", similarity.spectrum_similarity * 100.0);
            let holographic = format!("{:.2}", similarity.holographic_similarity * 100.0);
            let overall = format!("{:.2}", similarity.overall_similarity * 100.0);

            writeln!(
                writer,
                "{:<40} {:<15}% {:<15}% {:<20}% {:<15}%",
                entity_str, archetype, spectrum, holographic, overall
            )?;
        }

        Ok(())
    }

    /// Visualize holographic connections for an entity
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Create resonant connections"
    /// "Create entangled connections"
    /// "Create harmonic connections"
    pub fn visualize_holographic_connections<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        connections: Vec<&HolographicConnection>,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== Holographic Connections for Entity {:?} ===",
            entity_id
        )?;
        writeln!(writer, "Total Connections: {}", connections.len())?;

        if connections.is_empty() {
            writeln!(writer, "No holographic connections found.")?;
            return Ok(());
        }

        // Group by connection type
        let mut resonant: Vec<&HolographicConnection> = Vec::new();
        let mut entangled: Vec<&HolographicConnection> = Vec::new();
        let mut harmonic: Vec<&HolographicConnection> = Vec::new();

        for connection in &connections {
            match connection.connection_type {
                HolographicConnectionType::Resonant => resonant.push(connection),
                HolographicConnectionType::Entangled => entangled.push(connection),
                HolographicConnectionType::Harmonic => harmonic.push(connection),
            }
        }

        // Visualize each type
        Self::visualize_connection_type(writer, "Resonant", &resonant)?;
        Self::visualize_connection_type(writer, "Entangled", &entangled)?;
        Self::visualize_connection_type(writer, "Harmonic", &harmonic)?;

        Ok(())
    }

    /// Visualize connections of a specific type
    fn visualize_connection_type<W: Write>(
        writer: &mut W,
        connection_type: &str,
        connections: &[&HolographicConnection],
    ) -> std::io::Result<()> {
        if connections.is_empty() {
            return Ok(());
        }

        writeln!(
            writer,
            "\n--- {} Connections ({}) ---",
            connection_type,
            connections.len()
        )?;
        writeln!(
            writer,
            "{:<40} {:<15} {:<15} {:<15}",
            "Connected Entity", "Strength", "Archetype", "Frequency"
        )?;
        writeln!(writer, "{:-<100}", "")?;

        for connection in connections {
            let connected_id = format!("{:?}", connection.entity_id_1);
            let strength = format!("{:.2}", connection.strength * 100.0);
            let archetype = format!("{:.2}", connection.archetype_alignment * 100.0);
            let frequency = format!("{:.2}", connection.frequency_alignment * 100.0);

            writeln!(
                writer,
                "{:<40} {:<15}% {:<15}% {:<15}%",
                connected_id, strength, archetype, frequency
            )?;
        }

        Ok(())
    }

    /// Visualize collective influences for an entity
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Collective entities influence their members"
    /// "Individual entities influence their collectives"
    pub fn visualize_collective_influences<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        influences: Vec<&CollectiveInfluence>,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== Collective Influences for Entity {:?} ===",
            entity_id
        )?;
        writeln!(writer, "Total Influences: {}", influences.len())?;

        if influences.is_empty() {
            writeln!(writer, "No collective influences found.")?;
            return Ok(());
        }

        // Separate into collective → member and member → collective
        let mut to_member: Vec<&CollectiveInfluence> = Vec::new();
        let mut to_collective: Vec<&CollectiveInfluence> = Vec::new();

        for influence in &influences {
            if influence.collective_to_member {
                to_member.push(influence);
            } else {
                to_collective.push(influence);
            }
        }

        // Visualize collective → member influences
        if !to_member.is_empty() {
            writeln!(
                writer,
                "\n--- Collective → Member Influences ({}) ---",
                to_member.len()
            )?;
            writeln!(
                writer,
                "{:<40} {:<15} {:<20}",
                "From Collective", "Strength", "Type"
            )?;
            writeln!(writer, "{:-<80}", "")?;

            for influence in &to_member {
                let collective_id = format!("{:?}", influence.collective_id);
                let strength = format!("{:.2}", influence.strength * 100.0);
                let influence_type = format!("{:?}", influence.influence_type);

                writeln!(
                    writer,
                    "{:<40} {:<15}% {:<20}",
                    collective_id, strength, influence_type
                )?;
            }
        }

        // Visualize member → collective influences
        if !to_collective.is_empty() {
            writeln!(
                writer,
                "\n--- Member → Collective Influences ({}) ---",
                to_collective.len()
            )?;
            writeln!(
                writer,
                "{:<40} {:<15} {:<20}",
                "To Collective", "Strength", "Type"
            )?;
            writeln!(writer, "{:-<80}", "")?;

            for influence in &to_collective {
                let collective_id = format!("{:?}", influence.collective_id);
                let strength = format!("{:.2}", influence.strength * 100.0);
                let influence_type = format!("{:?}", influence.influence_type);

                writeln!(
                    writer,
                    "{:<40} {:<15}% {:<20}",
                    collective_id, strength, influence_type
                )?;
            }
        }

        Ok(())
    }

    /// Visualize co-evolution tracking for an entity
    ///
    /// From CURRENT_SIMULATION_ANALYSIS_FEB3_2026.md:
    /// "Co-evolution of different scales"
    pub fn visualize_co_evolution<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        co_evolution: Option<&CoEvolutionTracking>,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== Co-Evolution Tracking for Entity {:?} ===",
            entity_id
        )?;

        let co_evolution = match co_evolution {
            Some(c) => c,
            None => {
                writeln!(writer, "No co-evolution tracking found.")?;
                return Ok(());
            }
        };

        writeln!(
            writer,
            "Related Entities: {}",
            co_evolution.related_entities.len()
        )?;
        writeln!(
            writer,
            "Co-Evolution Rate: {:.2}",
            co_evolution.co_evolution_rate
        )?;
        writeln!(
            writer,
            "Alignment Score: {:.2}",
            co_evolution.alignment_score * 100.0
        )?;

        if !co_evolution.related_entities.is_empty() {
            writeln!(writer, "\nRelated Entities:")?;
            for (i, related_id) in co_evolution.related_entities.iter().enumerate() {
                writeln!(writer, "  {}. {:?}", i + 1, related_id)?;
            }
        }

        // Visualize alignment score
        let alignment_bar_len = (co_evolution.alignment_score * 50.0) as usize;
        let alignment_bar = "█".repeat(alignment_bar_len);
        let alignment_empty = "░".repeat(50 - alignment_bar_len);
        writeln!(
            writer,
            "\nAlignment Score: |{}{}| {:.2}%",
            alignment_bar,
            alignment_empty,
            co_evolution.alignment_score * 100.0
        )?;

        Ok(())
    }

    /// Visualize all interactions for an entity
    pub fn visualize_all_interactions<W: Write>(
        writer: &mut W,
        entity_id: &crate::entity_layer7::layer7::EntityId,
        interactions: &[&EntityInteraction],
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n=== All Interactions for Entity {:?} ===",
            entity_id
        )?;
        writeln!(writer, "Total Interactions: {}", interactions.len())?;

        if interactions.is_empty() {
            writeln!(writer, "No interactions found.")?;
            return Ok(());
        }

        // Group by interaction type
        let mut composition: Vec<&EntityInteraction> = Vec::new();
        let mut resonant: Vec<&EntityInteraction> = Vec::new();
        let mut entangled: Vec<&EntityInteraction> = Vec::new();
        let mut harmonic: Vec<&EntityInteraction> = Vec::new();
        let mut environmental: Vec<&EntityInteraction> = Vec::new();
        let mut collective: Vec<&EntityInteraction> = Vec::new();
        let mut member: Vec<&EntityInteraction> = Vec::new();

        for interaction in interactions {
            match interaction.interaction_type {
                InteractionType::Composition => composition.push(interaction),
                InteractionType::Resonant => resonant.push(interaction),
                InteractionType::Entangled => entangled.push(interaction),
                InteractionType::Harmonic => harmonic.push(interaction),
                InteractionType::Environmental => environmental.push(interaction),
                InteractionType::Collective => collective.push(interaction),
                InteractionType::Member => member.push(interaction),
            }
        }

        writeln!(writer, "\nInteraction Summary:")?;
        writeln!(writer, "  Composition: {}", composition.len())?;
        writeln!(writer, "  Resonant: {}", resonant.len())?;
        writeln!(writer, "  Entangled: {}", entangled.len())?;
        writeln!(writer, "  Harmonic: {}", harmonic.len())?;
        writeln!(writer, "  Environmental: {}", environmental.len())?;
        writeln!(writer, "  Collective: {}", collective.len())?;
        writeln!(writer, "  Member: {}", member.len())?;

        // Visualize recent interactions
        let recent_interactions: Vec<&EntityInteraction> =
            interactions.iter().take(10).cloned().collect();

        if !recent_interactions.is_empty() {
            writeln!(writer, "\nRecent Interactions (last 10):")?;
            writeln!(
                writer,
                "{:<15} {:<40} {:<20} {:<15} {:<10}",
                "Timestamp", "With Entity", "Type", "Strength", "Direction"
            )?;
            writeln!(writer, "{:-<120}", "")?;

            for interaction in recent_interactions {
                let other_id = if interaction.source_id == *entity_id {
                    &interaction.target_id
                } else {
                    &interaction.source_id
                };

                let timestamp = interaction.timestamp;
                let entity_str = format!("{:?}", other_id);
                let interaction_type = format!("{:?}", interaction.interaction_type);
                let strength = format!("{:.2}", interaction.strength.0 * 100.0);

                let direction = if interaction.direction {
                    "Bidirectional"
                } else {
                    "Directional"
                };

                writeln!(
                    writer,
                    "{:<15} {:<40} {:<20} {:<15}% {:<10}",
                    timestamp, entity_str, interaction_type, strength, direction
                )?;
            }
        }

        Ok(())
    }

    /// Visualize interaction statistics
    pub fn visualize_interaction_statistics<W: Write>(
        writer: &mut W,
        stats: &InteractionStatistics,
    ) -> std::io::Result<()> {
        writeln!(writer, "\n=== Interaction Statistics ===")?;

        writeln!(writer, "\nTotal Counts:")?;
        writeln!(writer, "  Total Interactions: {}", stats.total_interactions)?;
        writeln!(
            writer,
            "  Composition Effects: {}",
            stats.total_composition_effects
        )?;
        writeln!(
            writer,
            "  Holographic Connections: {}",
            stats.total_holographic_connections
        )?;
        writeln!(
            writer,
            "  Collective Influences: {}",
            stats.total_collective_influences
        )?;
        writeln!(
            writer,
            "  Co-Evolution Tracking: {}",
            stats.total_co_evolution_tracking
        )?;

        // Visualize interaction types
        if !stats.interaction_counts.is_empty() {
            writeln!(writer, "\nInteractions by Type:")?;
            writeln!(writer, "{:<20} {:<15}", "Type", "Count")?;
            writeln!(writer, "{:-<40}", "")?;

            let interaction_types = vec![
                InteractionType::Composition,
                InteractionType::Resonant,
                InteractionType::Entangled,
                InteractionType::Harmonic,
                InteractionType::Environmental,
                InteractionType::Collective,
                InteractionType::Member,
            ];

            for interaction_type in interaction_types {
                let count = stats
                    .interaction_counts
                    .get(&interaction_type)
                    .unwrap_or(&0);
                writeln!(
                    writer,
                    "{:<20} {:<15}",
                    format!("{:?}", interaction_type),
                    count
                )?;
            }
        }

        // Visualize connection types
        if !stats.connection_counts.is_empty() {
            writeln!(writer, "\nHolographic Connections by Type:")?;
            writeln!(writer, "{:<20} {:<15}", "Type", "Count")?;
            writeln!(writer, "{:-<40}", "")?;

            let connection_types = vec![
                HolographicConnectionType::Resonant,
                HolographicConnectionType::Entangled,
                HolographicConnectionType::Harmonic,
            ];

            for connection_type in connection_types {
                let count = stats.connection_counts.get(&connection_type).unwrap_or(&0);
                writeln!(
                    writer,
                    "{:<20} {:<15}",
                    format!("{:?}", connection_type),
                    count
                )?;
            }
        }

        Ok(())
    }

    /// Visualize full inter-scale interactions report
    pub fn visualize_full_report<W: Write>(
        writer: &mut W,
        manager: &InterScaleInteractionManager,
        entity_id: &crate::entity_layer7::layer7::EntityId,
    ) -> std::io::Result<()> {
        writeln!(
            writer,
            "\n╔════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║     INTER-SCALE INTERACTIONS FULL REPORT                         ║"
        )?;
        writeln!(
            writer,
            "╚════════════════════════════════════════════════════════════════╝"
        )?;

        // Get all data for the entity
        let composition_effects = manager.get_composition_effects(entity_id);
        let holographic_connections = manager.get_holographic_connections(entity_id);
        let collective_influences = manager.get_collective_influences(entity_id);
        let interactions = manager.get_entity_interactions(entity_id);

        // Get holographic similarities
        let mut holographic_similarities: Vec<
            &crate::simulation_v3::inter_scale_interactions::HolographicSimilarity,
        > = Vec::new();
        for ((id1, id2), similarity) in &manager.holographic_similarities {
            if id1 == entity_id || id2 == entity_id {
                holographic_similarities.push(similarity);
            }
        }

        // Get co-evolution tracking
        let co_evolution = manager
            .co_evolution_tracking
            .iter()
            .find(|c| &c.entity_id == entity_id);

        // Visualize all sections
        Self::visualize_composition_effects(writer, entity_id, &composition_effects)?;
        Self::visualize_holographic_similarity(writer, entity_id, holographic_similarities)?;
        Self::visualize_holographic_connections(writer, entity_id, holographic_connections)?;
        Self::visualize_collective_influences(writer, entity_id, collective_influences)?;
        Self::visualize_co_evolution(writer, entity_id, co_evolution)?;
        Self::visualize_all_interactions(writer, entity_id, &interactions)?;

        // Visualize statistics
        let stats = manager.get_interaction_statistics();
        Self::visualize_interaction_statistics(writer, &stats)?;

        writeln!(
            writer,
            "\n╔════════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            writer,
            "║     END OF INTER-SCALE INTERACTIONS REPORT                      ║"
        )?;
        writeln!(
            writer,
            "╚════════════════════════════════════════════════════════════════╝"
        )?;

        Ok(())
    }
}
