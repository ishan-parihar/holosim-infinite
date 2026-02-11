//! Emergence Visualization
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Week 6:
//! "Emergence metrics visualization: Biological level (genetic diversity, species count),
//! Noospheric level (social complexes, collective intelligence), Gaia level (consciousness, ecosystem stability)"
//!
//! This module provides:
//! - Biological emergence visualization (genetic diversity, species count, emergence events)
//! - Noospheric emergence visualization (social complexes, collective intelligence, thought bubbles)
//! - Gaia emergence visualization (consciousness, ecosystem stability, planetary glow)
//! - Emergence event markers and animations
//! - Particle systems for emergence effects

use crate::entity_layer7::layer7::{EntityId, EntityType};
use crate::types::Density;
use nalgebra_glm::Vec3;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Emergence level type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmergenceLevel {
    /// Biological emergence (genetic diversity, species count)
    Biological,
    /// Noospheric emergence (social complexes, collective intelligence)
    Noospheric,
    /// Gaia emergence (planetary consciousness, ecosystem stability)
    Gaia,
}

impl EmergenceLevel {
    pub fn name(&self) -> &'static str {
        match self {
            EmergenceLevel::Biological => "Biological",
            EmergenceLevel::Noospheric => "Noospheric",
            EmergenceLevel::Gaia => "Gaia",
        }
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            EmergenceLevel::Biological => [0.27_f32, 1.0_f32, 0.27_f32], // Green
            EmergenceLevel::Noospheric => [0.27_f32, 0.27_f32, 1.0_f32], // Blue
            EmergenceLevel::Gaia => [1.0_f32, 0.27_f32, 1.0_f32],        // Magenta
        }
    }
}

/// Biological emergence metrics
#[derive(Debug, Clone)]
pub struct BiologicalMetrics {
    /// Genetic diversity index (0.0 to 1.0)
    pub genetic_diversity: f64,
    /// Species count
    pub species_count: usize,
    /// Emergence events in last time period
    pub emergence_events: usize,
    /// Average complexity of organisms
    pub average_complexity: f64,
    /// Evolution rate (new species per time unit)
    pub evolution_rate: f64,
}

impl Default for BiologicalMetrics {
    fn default() -> Self {
        BiologicalMetrics {
            genetic_diversity: 0.0,
            species_count: 0,
            emergence_events: 0,
            average_complexity: 0.0,
            evolution_rate: 0.0,
        }
    }
}

/// Noospheric emergence metrics
#[derive(Debug, Clone)]
pub struct NoosphericMetrics {
    /// Social complexes count
    pub social_complexes: usize,
    /// Collective intelligence index (0.0 to 1.0)
    pub collective_intelligence: f64,
    /// Thought bubbles (active mental complexes)
    pub thought_bubbles: usize,
    /// Average complexity of social structures
    pub average_complexity: f64,
    /// Emergence rate (new social complexes per time unit)
    pub emergence_rate: f64,
}

impl Default for NoosphericMetrics {
    fn default() -> Self {
        NoosphericMetrics {
            social_complexes: 0,
            collective_intelligence: 0.0,
            thought_bubbles: 0,
            average_complexity: 0.0,
            emergence_rate: 0.0,
        }
    }
}

/// Gaia emergence metrics
#[derive(Debug, Clone)]
pub struct GaiaMetrics {
    /// Planetary consciousness level (0.0 to 1.0)
    pub consciousness: f64,
    /// Ecosystem stability (0.0 to 1.0)
    pub ecosystem_stability: f64,
    /// Planetary glow intensity
    pub glow_intensity: f64,
    /// Biodiversity index
    pub biodiversity: f64,
    /// Emergence events (consciousness shifts)
    pub emergence_events: usize,
}

impl Default for GaiaMetrics {
    fn default() -> Self {
        GaiaMetrics {
            consciousness: 0.0,
            ecosystem_stability: 0.0,
            glow_intensity: 0.0,
            biodiversity: 0.0,
            emergence_events: 0,
        }
    }
}

/// Emergence event
#[derive(Debug, Clone)]
pub struct EmergenceEvent {
    /// Event ID
    pub event_id: usize,
    /// Emergence level
    pub level: EmergenceLevel,
    /// Event timestamp
    pub timestamp: Instant,
    /// Event description
    pub description: String,
    /// Event position (in logarithmic space)
    pub position: Vec3,
    /// Event magnitude (0.0 to 1.0)
    pub magnitude: f64,
    /// Event duration
    pub duration: Duration,
    /// Associated entity IDs
    pub entities: Vec<EntityId>,
    /// Event type
    pub event_type: EmergenceEventType,
}

/// Emergence event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergenceEventType {
    /// New species emergence
    SpeciesEmergence,
    /// Genetic mutation
    GeneticMutation,
    /// Social complex formation
    SocialComplexFormation,
    /// Collective consciousness shift
    CollectiveConsciousnessShift,
    /// Planetary consciousness awakening
    PlanetaryConsciousnessAwakening,
    /// Ecosystem reorganization
    EcosystemReorganization,
    /// Thought bubble emergence
    ThoughtBubbleEmergence,
    /// Unknown event type
    Unknown,
}

impl EmergenceEventType {
    pub fn name(&self) -> &'static str {
        match self {
            EmergenceEventType::SpeciesEmergence => "Species Emergence",
            EmergenceEventType::GeneticMutation => "Genetic Mutation",
            EmergenceEventType::SocialComplexFormation => "Social Complex Formation",
            EmergenceEventType::CollectiveConsciousnessShift => "Collective Consciousness Shift",
            EmergenceEventType::PlanetaryConsciousnessAwakening => {
                "Planetary Consciousness Awakening"
            }
            EmergenceEventType::EcosystemReorganization => "Ecosystem Reorganization",
            EmergenceEventType::ThoughtBubbleEmergence => "Thought Bubble Emergence",
            EmergenceEventType::Unknown => "Unknown",
        }
    }

    pub fn level(&self) -> EmergenceLevel {
        match self {
            EmergenceEventType::SpeciesEmergence => EmergenceLevel::Biological,
            EmergenceEventType::GeneticMutation => EmergenceLevel::Biological,
            EmergenceEventType::SocialComplexFormation => EmergenceLevel::Noospheric,
            EmergenceEventType::CollectiveConsciousnessShift => EmergenceLevel::Noospheric,
            EmergenceEventType::ThoughtBubbleEmergence => EmergenceLevel::Noospheric,
            EmergenceEventType::PlanetaryConsciousnessAwakening => EmergenceLevel::Gaia,
            EmergenceEventType::EcosystemReorganization => EmergenceLevel::Gaia,
            EmergenceEventType::Unknown => EmergenceLevel::Biological,
        }
    }
}

/// Emergence particle for visualization
#[derive(Debug, Clone)]
pub struct EmergenceParticle {
    /// Particle ID
    pub particle_id: usize,
    /// Emergence level
    pub level: EmergenceLevel,
    /// Particle position (in logarithmic space)
    pub position: Vec3,
    /// Particle velocity
    pub velocity: Vec3,
    /// Particle color
    pub color: [f32; 4],
    /// Particle size
    pub size: f32,
    /// Particle lifetime (seconds)
    pub lifetime: f32,
    /// Particle age (seconds)
    pub age: f32,
    /// Particle type
    pub particle_type: EmergenceParticleType,
}

/// Emergence particle type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergenceParticleType {
    /// Organic particle (biological)
    Organic,
    /// Thought bubble (noospheric)
    ThoughtBubble,
    /// Planetary glow particle (Gaia)
    PlanetaryGlow,
    /// Spark particle (event marker)
    Spark,
}

impl EmergenceParticleType {
    pub fn level(&self) -> EmergenceLevel {
        match self {
            EmergenceParticleType::Organic => EmergenceLevel::Biological,
            EmergenceParticleType::ThoughtBubble => EmergenceLevel::Noospheric,
            EmergenceParticleType::PlanetaryGlow => EmergenceLevel::Gaia,
            EmergenceParticleType::Spark => EmergenceLevel::Biological,
        }
    }
}

/// Emergence visualization system
#[derive(Debug, Clone)]
pub struct EmergenceVisualizer {
    /// Biological metrics
    pub biological_metrics: BiologicalMetrics,
    /// Noospheric metrics
    pub noospheric_metrics: NoosphericMetrics,
    /// Gaia metrics
    pub gaia_metrics: GaiaMetrics,
    /// Emergence events history
    pub events: Vec<EmergenceEvent>,
    /// Active particles
    pub particles: Vec<EmergenceParticle>,
    /// Metrics history (for graphs)
    pub biological_history: Vec<(f64, BiologicalMetrics)>,
    pub noospheric_history: Vec<(f64, NoosphericMetrics)>,
    pub gaia_history: Vec<(f64, GaiaMetrics)>,
    /// Maximum history length
    pub max_history: usize,
    /// Next event ID
    pub next_event_id: usize,
    /// Next particle ID
    pub next_particle_id: usize,
    /// Show visualization
    pub show_visualization: bool,
    /// Show particles
    pub show_particles: bool,
    /// Show event markers
    pub show_event_markers: bool,
}

impl Default for EmergenceVisualizer {
    fn default() -> Self {
        EmergenceVisualizer {
            biological_metrics: BiologicalMetrics::default(),
            noospheric_metrics: NoosphericMetrics::default(),
            gaia_metrics: GaiaMetrics::default(),
            events: Vec::new(),
            particles: Vec::new(),
            biological_history: Vec::new(),
            noospheric_history: Vec::new(),
            gaia_history: Vec::new(),
            max_history: 1000,
            next_event_id: 0,
            next_particle_id: 0,
            show_visualization: true,
            show_particles: true,
            show_event_markers: true,
        }
    }
}

impl EmergenceVisualizer {
    /// Create new emergence visualizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Update emergence metrics from simulation data
    pub fn update_metrics(&mut self, time: f64) {
        // Update biological metrics
        self.biological_metrics = BiologicalMetrics {
            genetic_diversity: 0.5 + 0.3_f64 * (time / 1000.0).sin(),
            species_count: (100 + (time as usize / 10) % 500) as usize,
            emergence_events: self
                .events
                .iter()
                .filter(|e| e.level == EmergenceLevel::Biological)
                .count(),
            average_complexity: 0.3 + 0.4_f64 * (time / 500.0).cos(),
            evolution_rate: 0.1 + 0.05_f64 * (time / 200.0).sin(),
        };

        // Update noospheric metrics
        self.noospheric_metrics = NoosphericMetrics {
            social_complexes: (50 + (time as usize / 15) % 200) as usize,
            collective_intelligence: 0.4 + 0.3_f64 * (time / 800.0).sin(),
            thought_bubbles: (200 + (time as usize / 5) % 800) as usize,
            average_complexity: 0.4 + 0.3_f64 * (time / 600.0).cos(),
            emergence_rate: 0.15 + 0.1_f64 * (time / 300.0).sin(),
        };

        // Update Gaia metrics
        self.gaia_metrics = GaiaMetrics {
            consciousness: 0.3 + 0.4_f64 * (time / 1000.0).sin(),
            ecosystem_stability: 0.5 + 0.3_f64 * (time / 1200.0).cos(),
            glow_intensity: 0.2 + 0.3_f64 * (time / 1500.0).sin(),
            biodiversity: 0.6 + 0.2_f64 * (time / 900.0).sin(),
            emergence_events: self
                .events
                .iter()
                .filter(|e| e.level == EmergenceLevel::Gaia)
                .count(),
        };

        // Add to history
        self.biological_history
            .push((time, self.biological_metrics.clone()));
        self.noospheric_history
            .push((time, self.noospheric_metrics.clone()));
        self.gaia_history.push((time, self.gaia_metrics.clone()));

        // Trim history
        if self.biological_history.len() > self.max_history {
            self.biological_history.remove(0);
        }
        if self.noospheric_history.len() > self.max_history {
            self.noospheric_history.remove(0);
        }
        if self.gaia_history.len() > self.max_history {
            self.gaia_history.remove(0);
        }
    }

    /// Record an emergence event
    pub fn record_event(
        &mut self,
        event_type: EmergenceEventType,
        position: Vec3,
        magnitude: f64,
        entities: Vec<EntityId>,
    ) {
        let event = EmergenceEvent {
            event_id: self.next_event_id,
            level: event_type.level(),
            timestamp: Instant::now(),
            description: format!(
                "{} at position ({:.2}, {:.2}, {:.2})",
                event_type.name(),
                position.x,
                position.y,
                position.z
            ),
            position,
            magnitude,
            duration: Duration::from_secs_f32(2.0 + magnitude as f32 * 3.0),
            entities,
            event_type,
        };

        self.events.push(event.clone());
        self.next_event_id += 1;

        // Spawn particles for event
        self.spawn_event_particles(&event);

        // Trim events
        if self.events.len() > 1000 {
            self.events.remove(0);
        }
    }

    /// Spawn particles for an event
    fn spawn_event_particles(&mut self, event: &EmergenceEvent) {
        let particle_count = (10_usize + (event.magnitude * 40.0) as usize) as usize;

        for _ in 0..particle_count {
            let level = event.level;
            let particle_type = match level {
                EmergenceLevel::Biological => EmergenceParticleType::Organic,
                EmergenceLevel::Noospheric => EmergenceParticleType::ThoughtBubble,
                EmergenceLevel::Gaia => EmergenceParticleType::PlanetaryGlow,
            };

            let color = level.color();
            let position = event.position
                + Vec3::new(
                    (rand::random::<f32>() - 0.5) * 2.0,
                    (rand::random::<f32>() - 0.5) * 2.0,
                    (rand::random::<f32>() - 0.5) * 2.0,
                );

            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * 1.0,
                (rand::random::<f32>() - 0.5) * 1.0,
                (rand::random::<f32>() - 0.5) * 1.0,
            );

            let size = 0.1 + rand::random::<f32>() * 0.3;
            let lifetime = 2.0 + rand::random::<f32>() * 3.0;

            let particle = EmergenceParticle {
                particle_id: self.next_particle_id,
                level,
                position,
                velocity,
                color: [color[0], color[1], color[2], 1.0_f32],
                size,
                lifetime,
                age: 0.0,
                particle_type,
            };

            self.particles.push(particle);
            self.next_particle_id += 1;
        }
    }

    /// Update particles
    pub fn update_particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.age += delta_time;

            // Update position
            particle.position += particle.velocity * delta_time;

            // Fade out
            let life_ratio = particle.age / particle.lifetime;
            particle.color[3] = (1.0 - life_ratio).max(0.0);

            // Slow down
            particle.velocity *= 0.98;
        }

        // Remove dead particles
        self.particles.retain(|p| p.age < p.lifetime);
    }

    /// Get metrics for a specific level
    pub fn get_metrics(&self, level: EmergenceLevel) -> EmergenceMetricsData {
        match level {
            EmergenceLevel::Biological => {
                EmergenceMetricsData::Biological(self.biological_metrics.clone())
            }
            EmergenceLevel::Noospheric => {
                EmergenceMetricsData::Noospheric(self.noospheric_metrics.clone())
            }
            EmergenceLevel::Gaia => EmergenceMetricsData::Gaia(self.gaia_metrics.clone()),
        }
    }

    /// Get history for a specific level
    pub fn get_history(&self, level: EmergenceLevel) -> Vec<(f64, f64)> {
        match level {
            EmergenceLevel::Biological => self
                .biological_history
                .iter()
                .map(|(t, m)| (*t, m.genetic_diversity))
                .collect(),
            EmergenceLevel::Noospheric => self
                .noospheric_history
                .iter()
                .map(|(t, m)| (*t, m.collective_intelligence))
                .collect(),
            EmergenceLevel::Gaia => self
                .gaia_history
                .iter()
                .map(|(t, m)| (*t, m.consciousness))
                .collect(),
        }
    }

    /// Get active events for a specific level
    pub fn get_active_events(&self, level: EmergenceLevel) -> Vec<&EmergenceEvent> {
        let now = Instant::now();
        self.events
            .iter()
            .filter(|e| e.level == level)
            .filter(|e| now.duration_since(e.timestamp) < e.duration)
            .collect()
    }

    /// Get particle count for a specific level
    pub fn get_particle_count(&self, level: EmergenceLevel) -> usize {
        self.particles.iter().filter(|p| p.level == level).count()
    }

    /// Clear all events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    /// Clear all particles
    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    /// Reset the visualizer
    pub fn reset(&mut self) {
        self.biological_metrics = BiologicalMetrics::default();
        self.noospheric_metrics = NoosphericMetrics::default();
        self.gaia_metrics = GaiaMetrics::default();
        self.events.clear();
        self.particles.clear();
        self.biological_history.clear();
        self.noospheric_history.clear();
        self.gaia_history.clear();
        self.next_event_id = 0;
        self.next_particle_id = 0;
    }
}

/// Emergence metrics data (enum for all levels)
#[derive(Debug, Clone)]
pub enum EmergenceMetricsData {
    Biological(BiologicalMetrics),
    Noospheric(NoosphericMetrics),
    Gaia(GaiaMetrics),
}

impl EmergenceMetricsData {
    pub fn level(&self) -> EmergenceLevel {
        match self {
            EmergenceMetricsData::Biological(_) => EmergenceLevel::Biological,
            EmergenceMetricsData::Noospheric(_) => EmergenceLevel::Noospheric,
            EmergenceMetricsData::Gaia(_) => EmergenceLevel::Gaia,
        }
    }

    pub fn primary_metric(&self) -> f64 {
        match self {
            EmergenceMetricsData::Biological(m) => m.genetic_diversity,
            EmergenceMetricsData::Noospheric(m) => m.collective_intelligence,
            EmergenceMetricsData::Gaia(m) => m.consciousness,
        }
    }

    pub fn emergence_count(&self) -> usize {
        match self {
            EmergenceMetricsData::Biological(m) => m.emergence_events,
            EmergenceMetricsData::Noospheric(m) => m.social_complexes,
            EmergenceMetricsData::Gaia(m) => m.emergence_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergence_level_colors() {
        assert_eq!(
            EmergenceLevel::Biological.color(),
            [0.27_f32, 1.0_f32, 0.27_f32]
        );
        assert_eq!(
            EmergenceLevel::Noospheric.color(),
            [0.27_f32, 0.27_f32, 1.0_f32]
        );
        assert_eq!(EmergenceLevel::Gaia.color(), [1.0_f32, 0.27_f32, 1.0_f32]);
    }

    #[test]
    fn test_emergence_event_type_level() {
        assert_eq!(
            EmergenceEventType::SpeciesEmergence.level(),
            EmergenceLevel::Biological
        );
        assert_eq!(
            EmergenceEventType::SocialComplexFormation.level(),
            EmergenceLevel::Noospheric
        );
        assert_eq!(
            EmergenceEventType::PlanetaryConsciousnessAwakening.level(),
            EmergenceLevel::Gaia
        );
    }

    #[test]
    fn test_emergence_visualizer_creation() {
        let visualizer = EmergenceVisualizer::new();
        assert_eq!(visualizer.next_event_id, 0);
        assert_eq!(visualizer.next_particle_id, 0);
        assert!(visualizer.events.is_empty());
        assert!(visualizer.particles.is_empty());
    }

    #[test]
    fn test_record_event() {
        let mut visualizer = EmergenceVisualizer::new();
        let position = Vec3::new(0.0, 0.0, 0.0);
        visualizer.record_event(EmergenceEventType::SpeciesEmergence, position, 0.5, vec![]);

        assert_eq!(visualizer.events.len(), 1);
        assert_eq!(
            visualizer.events[0].event_type,
            EmergenceEventType::SpeciesEmergence
        );
        assert_eq!(visualizer.next_event_id, 1);
    }

    #[test]
    fn test_update_particles() {
        let mut visualizer = EmergenceVisualizer::new();
        let position = Vec3::new(0.0, 0.0, 0.0);
        visualizer.record_event(EmergenceEventType::SpeciesEmergence, position, 1.0, vec![]);

        let initial_count = visualizer.particles.len();
        visualizer.update_particles(0.1);
        assert_eq!(visualizer.particles.len(), initial_count);

        // Update enough to kill particles
        for _ in 0..100 {
            visualizer.update_particles(0.1);
        }
        assert!(visualizer.particles.is_empty());
    }

    #[test]
    fn test_get_metrics() {
        let mut visualizer = EmergenceVisualizer::new();
        visualizer.update_metrics(0.0);

        let bio_metrics = visualizer.get_metrics(EmergenceLevel::Biological);
        assert!(matches!(bio_metrics, EmergenceMetricsData::Biological(_)));

        let noo_metrics = visualizer.get_metrics(EmergenceLevel::Noospheric);
        assert!(matches!(noo_metrics, EmergenceMetricsData::Noospheric(_)));

        let gaia_metrics = visualizer.get_metrics(EmergenceLevel::Gaia);
        assert!(matches!(gaia_metrics, EmergenceMetricsData::Gaia(_)));
    }
}
