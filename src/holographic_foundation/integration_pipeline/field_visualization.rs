//! Field Visualization Components
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 6:
//! "Field heatmaps (consciousness density), Veil transparency indicators, Coherence meters"
//!
//! This module provides visualization components that reveal the holographic architecture:
//! - FieldHeatmap: Consciousness density visualization
//! - VeilTransparencyIndicator: Shows veil transparency state
//! - CoherenceMeter: Displays local and global coherence
//!
//! These components bridge the holographic field to the visualization layer.

use std::collections::HashMap;

use super::extraction_pipeline::FieldSnapshot;
use crate::holographic_foundation::field_state::{HolographicFieldState, Position3D};
use crate::types::Float;

/// Configuration for field visualization
#[derive(Debug, Clone)]
pub struct FieldVisualizationConfig {
    /// Grid resolution for heatmaps
    pub grid_resolution: usize,
    /// Color gradient start (low coherence)
    pub color_gradient_start: [f32; 4],
    /// Color gradient end (high coherence)
    pub color_gradient_end: [f32; 4],
    /// Update frequency (frames)
    pub update_frequency: u64,
    /// Enable density overlay
    pub density_overlay: bool,
    /// Enable spectrum overlay
    pub spectrum_overlay: bool,
    /// Maximum entities to render
    pub max_entities_render: usize,
}

impl Default for FieldVisualizationConfig {
    fn default() -> Self {
        Self {
            grid_resolution: 32,
            color_gradient_start: [0.0, 0.0, 0.2, 1.0], // Dark blue
            color_gradient_end: [1.0, 1.0, 1.0, 1.0],   // White
            update_frequency: 1,
            density_overlay: true,
            spectrum_overlay: true,
            max_entities_render: 10000,
        }
    }
}

/// Field heatmap data
#[derive(Debug, Clone)]
pub struct FieldHeatmap {
    /// Grid resolution
    pub resolution: usize,
    /// Coherence values (2D slice at z=0.5)
    pub coherence_grid: Vec<Vec<Float>>,
    /// Amplitude values (2D slice at z=0.5)
    pub amplitude_grid: Vec<Vec<Float>>,
    /// Density values (2D slice at z=0.5)
    pub density_grid: Vec<Vec<u8>>,
    /// Maximum coherence value
    pub max_coherence: Float,
    /// Minimum coherence value
    pub min_coherence: Float,
    /// Average coherence
    pub avg_coherence: Float,
    /// Timestamp of last update
    pub last_update: u64,
}

impl FieldHeatmap {
    /// Create an empty heatmap
    pub fn new(resolution: usize) -> Self {
        let coherence_grid = vec![vec![0.0; resolution]; resolution];
        let amplitude_grid = vec![vec![0.0; resolution]; resolution];
        let density_grid = vec![vec![0; resolution]; resolution];

        Self {
            resolution,
            coherence_grid,
            amplitude_grid,
            density_grid,
            max_coherence: 0.0,
            min_coherence: 0.0,
            avg_coherence: 0.0,
            last_update: 0,
        }
    }

    /// Generate heatmap from field state
    pub fn generate(&mut self, field_state: &HolographicFieldState, timestamp: u64) {
        let mut total_coherence = 0.0;
        let mut count = 0;

        for i in 0..self.resolution {
            for j in 0..self.resolution {
                let x = (i as Float) / (self.resolution - 1) as Float;
                let y = (j as Float) / (self.resolution - 1) as Float;
                let position = Position3D::new(x, y, 0.5);

                if let Some(node) = field_state.get_node_at(&position) {
                    self.coherence_grid[i][j] = node.coherence;
                    self.amplitude_grid[i][j] = node.total_amplitude();

                    let scale = node.dominant_scale();
                    self.density_grid[i][j] = scale.to_density();

                    total_coherence += node.coherence;
                    count += 1;

                    if node.coherence > self.max_coherence {
                        self.max_coherence = node.coherence;
                    }
                    if self.min_coherence == 0.0 || node.coherence < self.min_coherence {
                        self.min_coherence = node.coherence;
                    }
                }
            }
        }

        self.avg_coherence = if count > 0 {
            total_coherence / count as Float
        } else {
            0.0
        };
        self.last_update = timestamp;
    }

    /// Get coherence at position
    pub fn get_coherence_at(&self, x: usize, y: usize) -> Float {
        if x < self.resolution && y < self.resolution {
            self.coherence_grid[x][y]
        } else {
            0.0
        }
    }

    /// Convert to color data for rendering
    pub fn to_color_data(&self, config: &FieldVisualizationConfig) -> Vec<[f32; 4]> {
        let mut colors = Vec::with_capacity(self.resolution * self.resolution);

        for i in 0..self.resolution {
            for j in 0..self.resolution {
                let coherence = self.coherence_grid[i][j];
                let t = coherence.clamp(0.0, 1.0) as f32;

                let r = config.color_gradient_start[0]
                    + (config.color_gradient_end[0] - config.color_gradient_start[0]) * t;
                let g = config.color_gradient_start[1]
                    + (config.color_gradient_end[1] - config.color_gradient_start[1]) * t;
                let b = config.color_gradient_start[2]
                    + (config.color_gradient_end[2] - config.color_gradient_start[2]) * t;
                let a = config.color_gradient_start[3]
                    + (config.color_gradient_end[3] - config.color_gradient_start[3]) * t;

                colors.push([r, g, b, a]);
            }
        }

        colors
    }
}

/// Veil transparency indicator
#[derive(Debug, Clone)]
pub struct VeilTransparencyIndicator {
    /// Current veil transparency (0.0 = opaque, 1.0 = transparent)
    pub transparency: Float,
    /// Spectrum position (0.0 = pure time/space, 1.0+ = space/time dominant)
    pub spectrum_position: Float,
    /// Veil crossing events
    pub crossing_events: u64,
    /// Transparency history
    pub transparency_history: Vec<Float>,
    /// Maximum history length
    pub max_history: usize,
    /// Current veil state
    pub state: VeilState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VeilState {
    /// Veil is opaque (3rd density and below)
    Opaque,
    /// Veil is thinning (approaching 4th density)
    Thinning,
    /// Veil is transparent (4th density and above)
    Transparent,
    /// Entity is crossing the veil
    Crossing,
}

impl Default for VeilTransparencyIndicator {
    fn default() -> Self {
        Self {
            transparency: 0.0,
            spectrum_position: 0.5,
            crossing_events: 0,
            transparency_history: Vec::new(),
            max_history: 100,
            state: VeilState::Opaque,
        }
    }
}

impl VeilTransparencyIndicator {
    /// Create a new veil indicator
    pub fn new() -> Self {
        Self::default()
    }

    /// Update from field snapshot
    pub fn update(&mut self, snapshot: &FieldSnapshot) {
        self.transparency = snapshot.avg_veil_transparency;
        self.spectrum_position = snapshot.avg_spectrum_position;

        self.transparency_history.push(self.transparency);
        if self.transparency_history.len() > self.max_history {
            self.transparency_history.remove(0);
        }

        self.state = if self.transparency < 0.3 {
            VeilState::Opaque
        } else if self.transparency < 0.7 {
            VeilState::Thinning
        } else {
            VeilState::Transparent
        };
    }

    /// Record a veil crossing event
    pub fn record_crossing(&mut self) {
        self.crossing_events += 1;
        self.state = VeilState::Crossing;
    }

    /// Get transparency trend (positive = thinning, negative = thickening)
    pub fn transparency_trend(&self) -> Float {
        if self.transparency_history.len() < 2 {
            return 0.0;
        }

        let window_len = 10.min(self.transparency_history.len());
        let older_len = self.transparency_history.len().saturating_sub(10);

        let recent: Float = self.transparency_history
            [self.transparency_history.len().saturating_sub(window_len)..]
            .iter()
            .sum::<Float>()
            / window_len as Float;

        let older: Float = if older_len > 0 {
            self.transparency_history[0..older_len]
                .iter()
                .sum::<Float>()
                / older_len as Float
        } else {
            0.0
        };

        recent - older
    }

    /// Get color for veil visualization
    pub fn to_color(&self) -> [f32; 4] {
        match self.state {
            VeilState::Opaque => [0.2, 0.2, 0.4, 0.9],
            VeilState::Thinning => [0.4, 0.4, 0.6, 0.7],
            VeilState::Transparent => [0.6, 0.6, 0.8, 0.3],
            VeilState::Crossing => [1.0, 1.0, 0.5, 0.5],
        }
    }
}

/// Coherence meter
#[derive(Debug, Clone)]
pub struct CoherenceMeter {
    /// Global coherence
    pub global_coherence: Float,
    /// Local coherence at focus point
    pub local_coherence: Float,
    /// Coherence by density
    pub coherence_by_density: HashMap<u8, Float>,
    /// Coherence history
    pub coherence_history: Vec<Float>,
    /// Maximum history length
    pub max_history: usize,
    /// Coherence variance
    pub variance: Float,
    /// Coherence stability (0.0 to 1.0)
    pub stability: Float,
}

impl Default for CoherenceMeter {
    fn default() -> Self {
        Self {
            global_coherence: 0.0,
            local_coherence: 0.0,
            coherence_by_density: HashMap::new(),
            coherence_history: Vec::new(),
            max_history: 100,
            variance: 0.0,
            stability: 1.0,
        }
    }
}

impl CoherenceMeter {
    /// Create a new coherence meter
    pub fn new() -> Self {
        Self::default()
    }

    /// Update from field snapshot
    pub fn update(&mut self, snapshot: &FieldSnapshot) {
        self.global_coherence = snapshot.global_coherence;

        self.coherence_history.push(self.global_coherence);
        if self.coherence_history.len() > self.max_history {
            self.coherence_history.remove(0);
        }

        self.calculate_variance();
        self.calculate_stability();
    }

    /// Update local coherence at a specific position
    pub fn update_local(&mut self, coherence: Float) {
        self.local_coherence = coherence;
    }

    /// Update coherence for a specific density
    pub fn update_density_coherence(&mut self, density: u8, coherence: Float) {
        self.coherence_by_density.insert(density, coherence);
    }

    /// Calculate variance from history
    fn calculate_variance(&mut self) {
        if self.coherence_history.len() < 2 {
            self.variance = 0.0;
            return;
        }

        let mean: Float =
            self.coherence_history.iter().sum::<Float>() / self.coherence_history.len() as Float;

        let variance: Float = self
            .coherence_history
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<Float>()
            / self.coherence_history.len() as Float;

        self.variance = variance;
    }

    /// Calculate stability from variance
    fn calculate_stability(&mut self) {
        self.stability = 1.0 - (self.variance * 10.0).min(1.0);
    }

    /// Get average coherence over recent history
    pub fn average_recent(&self, window: usize) -> Float {
        let window = window.min(self.coherence_history.len());
        if window == 0 {
            return 0.0;
        }

        self.coherence_history[self.coherence_history.len().saturating_sub(window)..]
            .iter()
            .sum::<Float>()
            / window as Float
    }

    /// Get coherence trend (positive = increasing, negative = decreasing)
    pub fn coherence_trend(&self) -> Float {
        if self.coherence_history.len() < 2 {
            return 0.0;
        }

        let recent_avg = self.average_recent(10);
        let older_avg = self.average_recent(50);

        recent_avg - older_avg
    }

    /// Convert to display value (0-100%)
    pub fn to_display_value(&self) -> u8 {
        (self.global_coherence * 100.0).min(100.0) as u8
    }

    /// Get color for coherence visualization
    pub fn to_color(&self) -> [f32; 4] {
        let t = self.global_coherence.clamp(0.0, 1.0) as f32;

        [
            1.0 - t, // Red decreases with coherence
            t,       // Green increases with coherence
            0.2,     // Blue constant
            1.0,
        ]
    }
}

/// Combined field visualization data
#[derive(Debug, Clone)]
pub struct FieldVisualizationData {
    /// Heatmap
    pub heatmap: FieldHeatmap,
    /// Veil indicator
    pub veil_indicator: VeilTransparencyIndicator,
    /// Coherence meter
    pub coherence_meter: CoherenceMeter,
    /// Entity count
    pub entity_count: usize,
    /// Timestamp
    pub timestamp: u64,
}

impl FieldVisualizationData {
    /// Create empty visualization data
    pub fn new(resolution: usize) -> Self {
        Self {
            heatmap: FieldHeatmap::new(resolution),
            veil_indicator: VeilTransparencyIndicator::new(),
            coherence_meter: CoherenceMeter::new(),
            entity_count: 0,
            timestamp: 0,
        }
    }

    /// Update all components from field state and snapshot
    pub fn update(
        &mut self,
        field_state: &HolographicFieldState,
        snapshot: &FieldSnapshot,
        timestamp: u64,
    ) {
        self.heatmap.generate(field_state, timestamp);
        self.veil_indicator.update(snapshot);
        self.coherence_meter.update(snapshot);
        self.entity_count = snapshot.peak_count;
        self.timestamp = timestamp;
    }
}

/// Render-ready visualization data
#[derive(Debug, Clone)]
pub struct VisualizationRenderData {
    /// Heatmap color data
    pub heatmap_colors: Vec<[f32; 4]>,
    /// Veil color
    pub veil_color: [f32; 4],
    /// Coherence color
    pub coherence_color: [f32; 4],
    /// Entity positions (normalized)
    pub entity_positions: Vec<[f32; 3]>,
    /// Entity colors (based on density/polarity)
    pub entity_colors: Vec<[f32; 4]>,
    /// Statistics text
    pub statistics_text: HashMap<String, String>,
}

impl VisualizationRenderData {
    /// Create render data from visualization data
    pub fn from_visualization(
        viz_data: &FieldVisualizationData,
        config: &FieldVisualizationConfig,
    ) -> Self {
        let heatmap_colors = viz_data.heatmap.to_color_data(config);
        let veil_color = viz_data.veil_indicator.to_color();
        let coherence_color = viz_data.coherence_meter.to_color();

        let mut statistics_text = HashMap::new();
        statistics_text.insert(
            "global_coherence".to_string(),
            format!("{:.2}%", viz_data.coherence_meter.global_coherence * 100.0),
        );
        statistics_text.insert(
            "veil_transparency".to_string(),
            format!("{:.2}%", viz_data.veil_indicator.transparency * 100.0),
        );
        statistics_text.insert(
            "entity_count".to_string(),
            viz_data.entity_count.to_string(),
        );
        statistics_text.insert(
            "coherence_trend".to_string(),
            format!("{:+.3}", viz_data.coherence_meter.coherence_trend()),
        );
        statistics_text.insert(
            "stability".to_string(),
            format!("{:.2}%", viz_data.coherence_meter.stability * 100.0),
        );

        Self {
            heatmap_colors,
            veil_color,
            coherence_color,
            entity_positions: Vec::new(),
            entity_colors: Vec::new(),
            statistics_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization_config_default() {
        let config = FieldVisualizationConfig::default();
        assert_eq!(config.grid_resolution, 32);
        assert!(config.density_overlay);
    }

    #[test]
    fn test_field_heatmap_creation() {
        let heatmap = FieldHeatmap::new(32);
        assert_eq!(heatmap.resolution, 32);
        assert_eq!(heatmap.coherence_grid.len(), 32);
        assert_eq!(heatmap.coherence_grid[0].len(), 32);
    }

    #[test]
    fn test_heatmap_get_coherence() {
        let heatmap = FieldHeatmap::new(32);
        assert_eq!(heatmap.get_coherence_at(0, 0), 0.0);
        assert_eq!(heatmap.get_coherence_at(31, 31), 0.0);
    }

    #[test]
    fn test_heatmap_to_color_data() {
        let config = FieldVisualizationConfig::default();
        let heatmap = FieldHeatmap::new(16);
        let colors = heatmap.to_color_data(&config);
        assert_eq!(colors.len(), 16 * 16);
    }

    #[test]
    fn test_veil_indicator_default() {
        let indicator = VeilTransparencyIndicator::default();
        assert_eq!(indicator.transparency, 0.0);
        assert_eq!(indicator.state, VeilState::Opaque);
    }

    #[test]
    fn test_veil_indicator_update() {
        let mut indicator = VeilTransparencyIndicator::new();
        let snapshot = FieldSnapshot {
            global_coherence: 0.5,
            peak_count: 10,
            avg_amplitude: 1.0,
            density_distribution: HashMap::new(),
            avg_spectrum_position: 0.6,
            avg_veil_transparency: 0.5,
            timestamp: 0,
        };

        indicator.update(&snapshot);
        assert_eq!(indicator.transparency, 0.5);
        assert_eq!(indicator.state, VeilState::Thinning);
    }

    #[test]
    fn test_veil_indicator_crossing() {
        let mut indicator = VeilTransparencyIndicator::new();
        indicator.record_crossing();
        assert_eq!(indicator.crossing_events, 1);
        assert_eq!(indicator.state, VeilState::Crossing);
    }

    #[test]
    fn test_veil_indicator_to_color() {
        let indicator = VeilTransparencyIndicator::default();
        let color = indicator.to_color();
        assert_eq!(color[0], 0.2); // Opaque state
    }

    #[test]
    fn test_coherence_meter_default() {
        let meter = CoherenceMeter::default();
        assert_eq!(meter.global_coherence, 0.0);
        assert_eq!(meter.stability, 1.0);
    }

    #[test]
    fn test_coherence_meter_update() {
        let mut meter = CoherenceMeter::new();
        let snapshot = FieldSnapshot {
            global_coherence: 0.75,
            peak_count: 10,
            avg_amplitude: 1.0,
            density_distribution: HashMap::new(),
            avg_spectrum_position: 0.5,
            avg_veil_transparency: 0.3,
            timestamp: 0,
        };

        meter.update(&snapshot);
        assert_eq!(meter.global_coherence, 0.75);
        assert_eq!(meter.coherence_history.len(), 1);
    }

    #[test]
    fn test_coherence_meter_display_value() {
        let mut meter = CoherenceMeter::new();
        meter.global_coherence = 0.75;
        assert_eq!(meter.to_display_value(), 75);
    }

    #[test]
    fn test_coherence_meter_average_recent() {
        let mut meter = CoherenceMeter::new();
        for i in 0..20 {
            meter.coherence_history.push(i as Float / 20.0);
        }
        let avg = meter.average_recent(10);
        assert!(avg > 0.4);
    }

    #[test]
    fn test_field_visualization_data() {
        let viz_data = FieldVisualizationData::new(32);
        assert_eq!(viz_data.heatmap.resolution, 32);
        assert_eq!(viz_data.timestamp, 0);
    }

    #[test]
    fn test_render_data_from_visualization() {
        let config = FieldVisualizationConfig::default();
        let viz_data = FieldVisualizationData::new(16);
        let render_data = VisualizationRenderData::from_visualization(&viz_data, &config);

        assert_eq!(render_data.heatmap_colors.len(), 16 * 16);
        assert!(render_data.statistics_text.contains_key("global_coherence"));
    }
}
