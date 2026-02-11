//! Emergence Dashboard
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Week 6:
//! "Emergence dashboard: Real-time graphs for each level, progress bars and indicators, event log for emergence events"
//!
//! This module provides:
//! - Real-time graphs for biological, noospheric, and Gaia emergence
//! - Progress bars and indicators for each level
//! - Event log for emergence events
//! - Interactive controls for emergence visualization

use crate::gui::visualization::emergence_viz::{
    EmergenceEvent, EmergenceEventType, EmergenceLevel, EmergenceMetricsData, EmergenceVisualizer,
};
use std::collections::HashMap;

/// Emergence dashboard
#[derive(Debug, Clone)]
pub struct EmergenceDashboard {
    /// Show dashboard
    pub show_dashboard: bool,
    /// Selected emergence level for detailed view
    pub selected_level: Option<EmergenceLevel>,
    /// Show biological graph
    pub show_biological_graph: bool,
    /// Show noospheric graph
    pub show_noospheric_graph: bool,
    /// Show Gaia graph
    pub show_gaia_graph: bool,
    /// Show event log
    pub show_event_log: bool,
    /// Show particles
    pub show_particles: bool,
    /// Show event markers
    pub show_event_markers: bool,
    /// Maximum event log entries
    pub max_event_log_entries: usize,
    /// Event log filter (by level)
    pub event_log_filter: Option<EmergenceLevel>,
    /// Graph time range (seconds)
    pub graph_time_range: f64,
    /// Auto-scroll event log
    pub auto_scroll_event_log: bool,
    /// Show statistics panel
    pub show_statistics: bool,
}

impl Default for EmergenceDashboard {
    fn default() -> Self {
        EmergenceDashboard {
            show_dashboard: true,
            selected_level: None,
            show_biological_graph: true,
            show_noospheric_graph: true,
            show_gaia_graph: true,
            show_event_log: true,
            show_particles: true,
            show_event_markers: true,
            max_event_log_entries: 100,
            event_log_filter: None,
            graph_time_range: 60.0,
            auto_scroll_event_log: true,
            show_statistics: true,
        }
    }
}

impl EmergenceDashboard {
    /// Create new emergence dashboard
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle dashboard visibility
    pub fn toggle_dashboard(&mut self) {
        self.show_dashboard = !self.show_dashboard;
    }

    /// Set selected level
    pub fn set_selected_level(&mut self, level: Option<EmergenceLevel>) {
        self.selected_level = level;
    }

    /// Toggle graph visibility for a level
    pub fn toggle_graph(&mut self, level: EmergenceLevel) {
        match level {
            EmergenceLevel::Biological => self.show_biological_graph = !self.show_biological_graph,
            EmergenceLevel::Noospheric => self.show_noospheric_graph = !self.show_noospheric_graph,
            EmergenceLevel::Gaia => self.show_gaia_graph = !self.show_gaia_graph,
        }
    }

    /// Get graph visibility for a level
    pub fn is_graph_visible(&self, level: EmergenceLevel) -> bool {
        match level {
            EmergenceLevel::Biological => self.show_biological_graph,
            EmergenceLevel::Noospheric => self.show_noospheric_graph,
            EmergenceLevel::Gaia => self.show_gaia_graph,
        }
    }

    /// Set event log filter
    pub fn set_event_log_filter(&mut self, filter: Option<EmergenceLevel>) {
        self.event_log_filter = filter;
    }

    /// Get filtered events
    pub fn get_filtered_events<'a>(
        &self,
        visualizer: &'a EmergenceVisualizer,
    ) -> Vec<&'a EmergenceEvent> {
        if let Some(filter) = self.event_log_filter {
            visualizer
                .events
                .iter()
                .filter(|e| e.level == filter)
                .collect()
        } else {
            visualizer.events.iter().collect()
        }
    }

    /// Get recent events (limited by max_event_log_entries)
    pub fn get_recent_events<'a>(
        &self,
        visualizer: &'a EmergenceVisualizer,
    ) -> Vec<&'a EmergenceEvent> {
        let filtered = self.get_filtered_events(visualizer);
        let start = if filtered.len() > self.max_event_log_entries {
            filtered.len() - self.max_event_log_entries
        } else {
            0
        };
        filtered[start..].to_vec()
    }

    /// Get event statistics
    pub fn get_event_statistics(&self, visualizer: &EmergenceVisualizer) -> EventStatistics {
        let mut stats = EventStatistics::default();

        for event in &visualizer.events {
            stats.total_events += 1;

            match event.level {
                EmergenceLevel::Biological => {
                    stats.biological_events += 1;
                    match event.event_type {
                        EmergenceEventType::SpeciesEmergence => stats.species_emergence += 1,
                        EmergenceEventType::GeneticMutation => stats.genetic_mutations += 1,
                        _ => {}
                    }
                }
                EmergenceLevel::Noospheric => {
                    stats.noospheric_events += 1;
                    match event.event_type {
                        EmergenceEventType::SocialComplexFormation => {
                            stats.social_complex_formations += 1
                        }
                        EmergenceEventType::CollectiveConsciousnessShift => {
                            stats.collective_shifts += 1
                        }
                        EmergenceEventType::ThoughtBubbleEmergence => stats.thought_bubbles += 1,
                        _ => {}
                    }
                }
                EmergenceLevel::Gaia => {
                    stats.gaia_events += 1;
                    match event.event_type {
                        EmergenceEventType::PlanetaryConsciousnessAwakening => {
                            stats.planetary_awakenings += 1
                        }
                        EmergenceEventType::EcosystemReorganization => {
                            stats.ecosystem_reorganizations += 1
                        }
                        _ => {}
                    }
                }
            }

            stats.total_magnitude += event.magnitude;
        }

        if stats.total_events > 0 {
            stats.average_magnitude = stats.total_magnitude / stats.total_events as f64;
        }

        stats
    }

    /// Get emergence summary
    pub fn get_emergence_summary(&self, visualizer: &EmergenceVisualizer) -> EmergenceSummary {
        EmergenceSummary {
            biological_level: visualizer.get_metrics(EmergenceLevel::Biological),
            noospheric_level: visualizer.get_metrics(EmergenceLevel::Noospheric),
            gaia_level: visualizer.get_metrics(EmergenceLevel::Gaia),
            biological_particle_count: visualizer.get_particle_count(EmergenceLevel::Biological),
            noospheric_particle_count: visualizer.get_particle_count(EmergenceLevel::Noospheric),
            gaia_particle_count: visualizer.get_particle_count(EmergenceLevel::Gaia),
        }
    }

    /// Clear event log
    pub fn clear_event_log(&mut self) {
        // This would be called on the visualizer
    }

    /// Reset dashboard
    pub fn reset(&mut self) {
        self.selected_level = None;
        self.event_log_filter = None;
    }
}

/// Event statistics
#[derive(Debug, Clone, Default)]
pub struct EventStatistics {
    /// Total events
    pub total_events: usize,
    /// Biological events
    pub biological_events: usize,
    /// Noospheric events
    pub noospheric_events: usize,
    /// Gaia events
    pub gaia_events: usize,
    /// Species emergence events
    pub species_emergence: usize,
    /// Genetic mutations
    pub genetic_mutations: usize,
    /// Social complex formations
    pub social_complex_formations: usize,
    /// Collective consciousness shifts
    pub collective_shifts: usize,
    /// Thought bubbles
    pub thought_bubbles: usize,
    /// Planetary awakenings
    pub planetary_awakenings: usize,
    /// Ecosystem reorganizations
    pub ecosystem_reorganizations: usize,
    /// Total magnitude
    pub total_magnitude: f64,
    /// Average magnitude
    pub average_magnitude: f64,
}

/// Emergence summary
#[derive(Debug, Clone)]
pub struct EmergenceSummary {
    /// Biological level metrics
    pub biological_level: EmergenceMetricsData,
    /// Noospheric level metrics
    pub noospheric_level: EmergenceMetricsData,
    /// Gaia level metrics
    pub gaia_level: EmergenceMetricsData,
    /// Biological particle count
    pub biological_particle_count: usize,
    /// Noospheric particle count
    pub noospheric_particle_count: usize,
    /// Gaia particle count
    pub gaia_particle_count: usize,
}

/// Graph data point
#[derive(Debug, Clone, Copy)]
pub struct GraphDataPoint {
    /// Time (seconds)
    pub time: f64,
    /// Value
    pub value: f64,
}

/// Graph data for a level
#[derive(Debug, Clone)]
pub struct GraphData {
    /// Emergence level
    pub level: EmergenceLevel,
    /// Data points
    pub points: Vec<GraphDataPoint>,
    /// Min value
    pub min_value: f64,
    /// Max value
    pub max_value: f64,
}

impl GraphData {
    /// Create graph data from history
    pub fn from_history(level: EmergenceLevel, history: &[(f64, f64)], time_range: f64) -> Self {
        let current_time = history.last().map(|(t, _)| *t).unwrap_or(0.0);
        let start_time = (current_time - time_range).max(0.0);

        let points: Vec<GraphDataPoint> = history
            .iter()
            .filter(|(t, _)| *t >= start_time)
            .map(|(t, v)| GraphDataPoint {
                time: *t,
                value: *v,
            })
            .collect();

        let min_value = points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
        let max_value = points
            .iter()
            .map(|p| p.value)
            .fold(f64::NEG_INFINITY, f64::max);

        GraphData {
            level,
            points,
            min_value: if min_value.is_finite() {
                min_value
            } else {
                0.0
            },
            max_value: if max_value.is_finite() {
                max_value
            } else {
                1.0
            },
        }
    }

    /// Get normalized value (0.0 to 1.0)
    pub fn normalize_value(&self, value: f64) -> f64 {
        if self.max_value > self.min_value {
            (value - self.min_value) / (self.max_value - self.min_value)
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = EmergenceDashboard::new();
        assert!(dashboard.show_dashboard);
        assert!(dashboard.show_biological_graph);
        assert!(dashboard.show_noospheric_graph);
        assert!(dashboard.show_gaia_graph);
        assert!(dashboard.show_event_log);
    }

    #[test]
    fn test_toggle_dashboard() {
        let mut dashboard = EmergenceDashboard::new();
        assert!(dashboard.show_dashboard);
        dashboard.toggle_dashboard();
        assert!(!dashboard.show_dashboard);
        dashboard.toggle_dashboard();
        assert!(dashboard.show_dashboard);
    }

    #[test]
    fn test_toggle_graph() {
        let mut dashboard = EmergenceDashboard::new();
        assert!(dashboard.show_biological_graph);
        dashboard.toggle_graph(EmergenceLevel::Biological);
        assert!(!dashboard.show_biological_graph);
        dashboard.toggle_graph(EmergenceLevel::Biological);
        assert!(dashboard.show_biological_graph);
    }

    #[test]
    fn test_is_graph_visible() {
        let dashboard = EmergenceDashboard::new();
        assert!(dashboard.is_graph_visible(EmergenceLevel::Biological));
        assert!(dashboard.is_graph_visible(EmergenceLevel::Noospheric));
        assert!(dashboard.is_graph_visible(EmergenceLevel::Gaia));
    }

    #[test]
    fn test_set_selected_level() {
        let mut dashboard = EmergenceDashboard::new();
        assert!(dashboard.selected_level.is_none());
        dashboard.set_selected_level(Some(EmergenceLevel::Biological));
        assert_eq!(dashboard.selected_level, Some(EmergenceLevel::Biological));
        dashboard.set_selected_level(None);
        assert!(dashboard.selected_level.is_none());
    }

    #[test]
    fn test_set_event_log_filter() {
        let mut dashboard = EmergenceDashboard::new();
        assert!(dashboard.event_log_filter.is_none());
        dashboard.set_event_log_filter(Some(EmergenceLevel::Biological));
        assert_eq!(dashboard.event_log_filter, Some(EmergenceLevel::Biological));
    }

    #[test]
    fn test_get_filtered_events() {
        let mut dashboard = EmergenceDashboard::new();
        let mut visualizer = EmergenceVisualizer::new();

        use nalgebra_glm::Vec3;
        visualizer.record_event(
            EmergenceEventType::SpeciesEmergence,
            Vec3::new(0.0, 0.0, 0.0),
            0.5,
            vec![],
        );
        visualizer.record_event(
            EmergenceEventType::SocialComplexFormation,
            Vec3::new(1.0, 0.0, 0.0),
            0.5,
            vec![],
        );

        // No filter
        assert_eq!(dashboard.get_filtered_events(&visualizer).len(), 2);

        // Biological filter
        dashboard.set_event_log_filter(Some(EmergenceLevel::Biological));
        assert_eq!(dashboard.get_filtered_events(&visualizer).len(), 1);

        // Noospheric filter
        dashboard.set_event_log_filter(Some(EmergenceLevel::Noospheric));
        assert_eq!(dashboard.get_filtered_events(&visualizer).len(), 1);
    }

    #[test]
    fn test_get_recent_events() {
        let mut dashboard = EmergenceDashboard::new();
        dashboard.max_event_log_entries = 2;

        let mut visualizer = EmergenceVisualizer::new();
        use nalgebra_glm::Vec3;

        for i in 0..5 {
            visualizer.record_event(
                EmergenceEventType::SpeciesEmergence,
                Vec3::new(i as f32, 0.0, 0.0),
                0.5,
                vec![],
            );
        }

        let recent = dashboard.get_recent_events(&visualizer);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_get_event_statistics() {
        let dashboard = EmergenceDashboard::new();
        let mut visualizer = EmergenceVisualizer::new();
        use nalgebra_glm::Vec3;

        visualizer.record_event(
            EmergenceEventType::SpeciesEmergence,
            Vec3::new(0.0, 0.0, 0.0),
            0.5,
            vec![],
        );
        visualizer.record_event(
            EmergenceEventType::SocialComplexFormation,
            Vec3::new(1.0, 0.0, 0.0),
            0.7,
            vec![],
        );
        visualizer.record_event(
            EmergenceEventType::PlanetaryConsciousnessAwakening,
            Vec3::new(2.0, 0.0, 0.0),
            0.9,
            vec![],
        );

        let stats = dashboard.get_event_statistics(&visualizer);
        assert_eq!(stats.total_events, 3);
        assert_eq!(stats.biological_events, 1);
        assert_eq!(stats.noospheric_events, 1);
        assert_eq!(stats.gaia_events, 1);
        assert_eq!(stats.species_emergence, 1);
        assert_eq!(stats.social_complex_formations, 1);
        assert_eq!(stats.planetary_awakenings, 1);
        // Use approximate comparison for floating point
        assert!((stats.average_magnitude - 0.7).abs() < 1e-6);
    }

    #[test]
    fn test_get_emergence_summary() {
        let dashboard = EmergenceDashboard::new();
        let mut visualizer = EmergenceVisualizer::new();
        visualizer.update_metrics(0.0);

        let summary = dashboard.get_emergence_summary(&visualizer);
        assert!(matches!(
            summary.biological_level,
            EmergenceMetricsData::Biological(_)
        ));
        assert!(matches!(
            summary.noospheric_level,
            EmergenceMetricsData::Noospheric(_)
        ));
        assert!(matches!(summary.gaia_level, EmergenceMetricsData::Gaia(_)));
    }

    #[test]
    fn test_graph_data_from_history() {
        let history = vec![(0.0, 0.5), (10.0, 0.6), (20.0, 0.7), (30.0, 0.8)];

        let graph_data = GraphData::from_history(EmergenceLevel::Biological, &history, 30.0);
        assert_eq!(graph_data.points.len(), 4);
        assert_eq!(graph_data.min_value, 0.5);
        assert_eq!(graph_data.max_value, 0.8);

        // Test with time range
        let graph_data_limited =
            GraphData::from_history(EmergenceLevel::Biological, &history, 15.0);
        assert_eq!(graph_data_limited.points.len(), 2);
    }

    #[test]
    fn test_normalize_value() {
        let history = vec![(0.0, 0.0), (10.0, 1.0)];

        let graph_data = GraphData::from_history(EmergenceLevel::Biological, &history, 30.0);
        assert_eq!(graph_data.normalize_value(0.0), 0.0);
        assert_eq!(graph_data.normalize_value(0.5), 0.5);
        assert_eq!(graph_data.normalize_value(1.0), 1.0);
    }
}
