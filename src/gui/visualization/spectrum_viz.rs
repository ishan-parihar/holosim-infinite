//! Spectrum Visualization Overlay
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Week 5:
//! "Spectrum visualization overlay: Space/time vs time/space continuum, entity positions on spectrum, Veil indicator at v=1"
//!
//! This module provides:
//! - Space/time vs time/space continuum visualization
//! - Entity positions on spectrum
//! - Veil indicator at v=1
//! - Gradient background for spectrum regions
//! - Interactive spectrum filtering

use crate::types::Density;
use nalgebra_glm::Vec3;

/// Position on the space/time spectrum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectrumPosition {
    /// Spectrum ratio (v = s/t or v = t/s)
    /// Values > 1.0 indicate space/time dominance
    /// Values < 1.0 indicate time/space dominance
    /// Value of 1.0 is at the Veil
    pub ratio: f64,
    /// Space/time component (0.0 to 1.0)
    pub space_time: f64,
    /// Time/space component (0.0 to 1.0)
    pub time_space: f64,
    /// Is entity on the space/time side of the Veil?
    pub is_space_time: bool,
}

impl SpectrumPosition {
    /// Create spectrum position from ratio
    pub fn from_ratio(ratio: f64) -> Self {
        let is_space_time = ratio >= 1.0;

        // Normalize components
        let (space_time, time_space) = if is_space_time {
            // Space/time side: ratio maps to [0.5, 1.0] space_time
            let normalized = (ratio - 1.0) / (100.0 - 1.0); // Normalize [1, 100] to [0, 1]
            (0.5 + normalized * 0.5, 0.5 - normalized * 0.5)
        } else {
            // Time/space side: inverse ratio maps to [0.5, 1.0] time_space
            let inv_ratio = 1.0 / ratio;
            let normalized = (inv_ratio - 1.0) / (100.0 - 1.0);
            (0.5 - normalized * 0.5, 0.5 + normalized * 0.5)
        };

        SpectrumPosition {
            ratio,
            space_time: space_time.max(0.0).min(1.0),
            time_space: time_space.max(0.0).min(1.0),
            is_space_time,
        }
    }

    /// Create spectrum position from components
    pub fn from_components(space_time: f64, time_space: f64) -> Self {
        let ratio = if space_time >= time_space {
            // Space/time side
            if space_time <= 0.5 {
                1.0
            } else {
                1.0 + (space_time - 0.5) / 0.5 * 99.0
            }
        } else {
            // Time/space side
            if time_space <= 0.5 {
                1.0
            } else {
                1.0 / (1.0 + (time_space - 0.5) / 0.5 * 99.0)
            }
        };

        SpectrumPosition::from_ratio(ratio)
    }

    /// Get the normalized position on spectrum (0.0 = deep time/space, 1.0 = deep space/time)
    pub fn normalized_position(&self) -> f64 {
        if self.ratio >= 1.0 {
            // Space/time side: [0.5, 1.0]
            0.5 + (self.ratio.log10() / 2.0) * 0.5
        } else {
            // Time/space side: [0.0, 0.5]
            0.5 - ((1.0 / self.ratio).log10() / 2.0) * 0.5
        }
    }

    /// Get distance from the Veil (v=1)
    pub fn distance_from_veil(&self) -> f64 {
        (self.ratio.log10().abs()) / 2.0
    }

    /// Check if entity is at the Veil
    pub fn is_at_veil(&self) -> bool {
        self.ratio.abs() - 1.0 < 0.1
    }

    /// Get the spectrum region name
    pub fn region_name(&self) -> &'static str {
        if self.is_at_veil() {
            "The Veil"
        } else if self.is_space_time {
            match self.distance_from_veil() {
                d if d < 0.2 => "Near Veil (Space/Time)",
                d if d < 0.5 => "Mid Space/Time",
                _ => "Deep Space/Time",
            }
        } else {
            match self.distance_from_veil() {
                d if d < 0.2 => "Near Veil (Time/Space)",
                d if d < 0.5 => "Mid Time/Space",
                _ => "Deep Time/Space",
            }
        }
    }
}

/// Spectrum visualization data for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpectrumVisualizationData {
    /// Entity position on spectrum (0.0 to 1.0)
    pub spectrum_position: f32,
    /// Entity color (rgba)
    pub color: [f32; 4],
    /// Size of entity marker on spectrum
    pub size: f32,
    /// Density level (1-8)
    pub density: f32,
    /// Is entity selected
    pub selected: u32,
    /// Entity ID
    pub entity_id: u64,
}

/// Spectrum visualizer for rendering the space/time continuum
#[derive(Debug, Clone)]
pub struct SpectrumVisualizer {
    /// Veil transparency (0.0 = opaque, 1.0 = transparent)
    veil_transparency: f32,
    /// Show spectrum overlay
    show_overlay: bool,
    /// Spectrum filter range (min, max)
    filter_range: (f32, f32),
    /// Selected spectrum region for filtering
    selected_region: Option<SpectrumRegion>,
}

/// Regions on the spectrum for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpectrumRegion {
    /// Deep time/space (0.0 to 0.2)
    DeepTimeSpace,
    /// Mid time/space (0.2 to 0.4)
    MidTimeSpace,
    /// Near Veil time/space (0.4 to 0.5)
    NearVeilTimeSpace,
    /// The Veil (0.5)
    TheVeil,
    /// Near Veil space/time (0.5 to 0.6)
    NearVeilSpaceTime,
    /// Mid space/time (0.6 to 0.8)
    MidSpaceTime,
    /// Deep space/time (0.8 to 1.0)
    DeepSpaceTime,
}

impl SpectrumRegion {
    /// Get region from normalized spectrum position
    pub fn from_position(position: f64) -> Self {
        match position {
            p if p < 0.2 => SpectrumRegion::DeepTimeSpace,
            p if p < 0.4 => SpectrumRegion::MidTimeSpace,
            p if p < 0.48 => SpectrumRegion::NearVeilTimeSpace,
            p if p < 0.52 => SpectrumRegion::TheVeil,
            p if p < 0.6 => SpectrumRegion::NearVeilSpaceTime,
            p if p < 0.8 => SpectrumRegion::MidSpaceTime,
            _ => SpectrumRegion::DeepSpaceTime,
        }
    }

    /// Get position range for this region
    pub fn range(&self) -> (f64, f64) {
        match self {
            SpectrumRegion::DeepTimeSpace => (0.0, 0.2),
            SpectrumRegion::MidTimeSpace => (0.2, 0.4),
            SpectrumRegion::NearVeilTimeSpace => (0.4, 0.48),
            SpectrumRegion::TheVeil => (0.48, 0.52),
            SpectrumRegion::NearVeilSpaceTime => (0.52, 0.6),
            SpectrumRegion::MidSpaceTime => (0.6, 0.8),
            SpectrumRegion::DeepSpaceTime => (0.8, 1.0),
        }
    }

    /// Get region name
    pub fn name(&self) -> &'static str {
        match self {
            SpectrumRegion::DeepTimeSpace => "Deep Time/Space",
            SpectrumRegion::MidTimeSpace => "Mid Time/Space",
            SpectrumRegion::NearVeilTimeSpace => "Near Veil (Time/Space)",
            SpectrumRegion::TheVeil => "The Veil",
            SpectrumRegion::NearVeilSpaceTime => "Near Veil (Space/Time)",
            SpectrumRegion::MidSpaceTime => "Mid Space/Time",
            SpectrumRegion::DeepSpaceTime => "Deep Space/Time",
        }
    }

    /// Get region color for visualization
    pub fn color(&self) -> [f32; 4] {
        match self {
            SpectrumRegion::DeepTimeSpace => [0.3_f32, 0.0_f32, 0.5_f32, 0.8_f32], // Deep purple
            SpectrumRegion::MidTimeSpace => [0.5_f32, 0.0_f32, 0.8_f32, 0.8_f32],  // Purple
            SpectrumRegion::NearVeilTimeSpace => [0.7_f32, 0.5_f32, 1.0_f32, 0.8_f32], // Lavender
            SpectrumRegion::TheVeil => [1.0_f32, 1.0_f32, 1.0_f32, 0.9_f32],       // White
            SpectrumRegion::NearVeilSpaceTime => [1.0_f32, 0.7_f32, 0.5_f32, 0.8_f32], // Orange-white
            SpectrumRegion::MidSpaceTime => [1.0_f32, 0.5_f32, 0.0_f32, 0.8_f32],      // Orange
            SpectrumRegion::DeepSpaceTime => [0.8_f32, 0.3_f32, 0.0_f32, 0.8_f32],     // Red-orange
        }
    }
}

impl SpectrumVisualizer {
    /// Create a new spectrum visualizer
    pub fn new() -> Self {
        SpectrumVisualizer {
            veil_transparency: 0.5,
            show_overlay: true,
            filter_range: (0.0, 1.0),
            selected_region: None,
        }
    }

    /// Set veil transparency
    pub fn set_veil_transparency(&mut self, transparency: f32) {
        self.veil_transparency = transparency.max(0.0).min(1.0);
    }

    /// Get veil transparency
    pub fn veil_transparency(&self) -> f32 {
        self.veil_transparency
    }

    /// Show or hide spectrum overlay
    pub fn set_show_overlay(&mut self, show: bool) {
        self.show_overlay = show;
    }

    /// Is spectrum overlay visible?
    pub fn is_overlay_visible(&self) -> bool {
        self.show_overlay
    }

    /// Set spectrum filter range
    pub fn set_filter_range(&mut self, min: f32, max: f32) {
        self.filter_range = (min.max(0.0), max.min(1.0));
    }

    /// Get spectrum filter range
    pub fn filter_range(&self) -> (f32, f32) {
        self.filter_range
    }

    /// Select spectrum region for filtering
    pub fn select_region(&mut self, region: SpectrumRegion) {
        let (min, max) = region.range();
        self.filter_range = (min as f32, max as f32);
        self.selected_region = Some(region);
    }

    /// Clear spectrum filter
    pub fn clear_filter(&mut self) {
        self.filter_range = (0.0, 1.0);
        self.selected_region = None;
    }

    /// Get selected region
    pub fn selected_region(&self) -> Option<SpectrumRegion> {
        self.selected_region
    }

    /// Check if entity passes spectrum filter
    pub fn passes_filter(&self, spectrum_position: f32) -> bool {
        spectrum_position >= self.filter_range.0 && spectrum_position <= self.filter_range.1
    }

    /// Convert entity spectrum position to visualization data
    pub fn entity_to_spectrum_data(
        &self,
        entity_id: u64,
        spectrum_position: SpectrumPosition,
        density: Density,
        color: [f32; 4],
        selected: bool,
    ) -> SpectrumVisualizationData {
        let normalized_pos = spectrum_position.normalized_position() as f32;

        // Size based on density
        let size = match density {
            Density::First => 3.0_f32,
            Density::Second => 4.0_f32,
            Density::Third => 5.0_f32,
            Density::Fourth => 6.0_f32,
            Density::Fifth => 7.0_f32,
            Density::Sixth => 8.0_f32,
            Density::Seventh => 9.0_f32,
            Density::Eighth => 10.0_f32,
        };

        // Apply filter opacity
        let passes_filter = self.passes_filter(normalized_pos);
        let alpha = if passes_filter {
            color[3]
        } else {
            color[3] * 0.3_f32
        };

        SpectrumVisualizationData {
            spectrum_position: normalized_pos,
            color: [color[0], color[1], color[2], alpha],
            size,
            density: density.as_u8() as f32,
            selected: if selected { 1 } else { 0 },
            entity_id,
        }
    }

    /// Get spectrum overlay colors for gradient rendering
    pub fn get_gradient_colors(&self) -> Vec<[f32; 4]> {
        vec![
            [0.3_f32, 0.0_f32, 0.5_f32, 0.8_f32], // Deep time/space (purple)
            [0.5_f32, 0.0_f32, 0.8_f32, 0.8_f32], // Mid time/space
            [0.7_f32, 0.5_f32, 1.0_f32, 0.8_f32], // Near Veil time/space
            [1.0_f32, 1.0_f32, 1.0_f32, 0.9_f32], // The Veil (white)
            [1.0_f32, 0.7_f32, 0.5_f32, 0.8_f32], // Near Veil space/time
            [1.0_f32, 0.5_f32, 0.0_f32, 0.8_f32], // Mid space/time
            [0.8_f32, 0.3_f32, 0.0_f32, 0.8_f32], // Deep space/time
        ]
    }

    /// Get Veil position on spectrum
    pub fn veil_position(&self) -> f32 {
        0.5
    }

    /// Get Veil color
    pub fn veil_color(&self) -> [f32; 4] {
        [1.0_f32, 1.0_f32, 1.0_f32, self.veil_transparency]
    }
}

impl Default for SpectrumVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Spectrum overlay for rendering
#[derive(Debug, Clone)]
pub struct SpectrumOverlay {
    /// Overlay position (screen coordinates)
    pub position: (f32, f32),
    /// Overlay size (width, height)
    pub size: (f32, f32),
    /// Orientation (horizontal or vertical)
    pub orientation: SpectrumOrientation,
    /// Show labels
    pub show_labels: bool,
}

/// Orientation of spectrum overlay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectrumOrientation {
    /// Horizontal spectrum (left to right)
    Horizontal,
    /// Vertical spectrum (bottom to top)
    Vertical,
}

impl SpectrumOverlay {
    /// Create a new spectrum overlay
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        SpectrumOverlay {
            position: (x, y),
            size: (width, height),
            orientation: if width > height {
                SpectrumOrientation::Horizontal
            } else {
                SpectrumOrientation::Vertical
            },
            show_labels: true,
        }
    }

    /// Convert spectrum position to screen position
    pub fn spectrum_to_screen(&self, spectrum_pos: f32) -> (f32, f32) {
        match self.orientation {
            SpectrumOrientation::Horizontal => {
                let x = self.position.0 + spectrum_pos * self.size.0;
                let y = self.position.1 + self.size.1 / 2.0;
                (x, y)
            }
            SpectrumOrientation::Vertical => {
                let x = self.position.0 + self.size.0 / 2.0;
                let y = self.position.1 + spectrum_pos * self.size.1;
                (x, y)
            }
        }
    }

    /// Convert screen position to spectrum position
    pub fn screen_to_spectrum(&self, screen_x: f32, screen_y: f32) -> Option<f32> {
        match self.orientation {
            SpectrumOrientation::Horizontal => {
                if screen_x < self.position.0 || screen_x > self.position.0 + self.size.0 {
                    return None;
                }
                Some((screen_x - self.position.0) / self.size.0)
            }
            SpectrumOrientation::Vertical => {
                if screen_y < self.position.1 || screen_y > self.position.1 + self.size.1 {
                    return None;
                }
                Some((screen_y - self.position.1) / self.size.1)
            }
        }
    }

    /// Set orientation
    pub fn set_orientation(&mut self, orientation: SpectrumOrientation) {
        self.orientation = orientation;
    }

    /// Show or hide labels
    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    /// Are labels visible?
    pub fn labels_visible(&self) -> bool {
        self.show_labels
    }
}

impl Default for SpectrumOverlay {
    fn default() -> Self {
        SpectrumOverlay::new(0.0, 0.0, 800.0, 50.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_position_from_ratio() {
        let pos = SpectrumPosition::from_ratio(1.0);
        assert!(pos.is_at_veil());
        assert_eq!(pos.normalized_position(), 0.5);

        let st_pos = SpectrumPosition::from_ratio(100.0);
        assert!(st_pos.is_space_time);
        assert!(st_pos.normalized_position() > 0.5);

        let ts_pos = SpectrumPosition::from_ratio(0.01);
        assert!(!ts_pos.is_space_time);
        assert!(ts_pos.normalized_position() < 0.5);
    }

    #[test]
    fn test_spectrum_position_from_components() {
        let pos = SpectrumPosition::from_components(0.8, 0.2);
        assert!(pos.is_space_time);

        let pos = SpectrumPosition::from_components(0.2, 0.8);
        assert!(!pos.is_space_time);

        let pos = SpectrumPosition::from_components(0.5, 0.5);
        assert!(pos.is_at_veil());
    }

    #[test]
    fn test_spectrum_region_from_position() {
        assert_eq!(
            SpectrumRegion::from_position(0.1),
            SpectrumRegion::DeepTimeSpace
        );
        assert_eq!(
            SpectrumRegion::from_position(0.3),
            SpectrumRegion::MidTimeSpace
        );
        assert_eq!(SpectrumRegion::from_position(0.5), SpectrumRegion::TheVeil);
        assert_eq!(
            SpectrumRegion::from_position(0.7),
            SpectrumRegion::MidSpaceTime
        );
        assert_eq!(
            SpectrumRegion::from_position(0.9),
            SpectrumRegion::DeepSpaceTime
        );
    }

    #[test]
    fn test_spectrum_region_range() {
        let veil_range = SpectrumRegion::TheVeil.range();
        assert_eq!(veil_range, (0.48, 0.52));

        let deep_st_range = SpectrumRegion::DeepSpaceTime.range();
        assert_eq!(deep_st_range, (0.8, 1.0));
    }

    #[test]
    fn test_spectrum_visualizer_creation() {
        let visualizer = SpectrumVisualizer::new();
        assert_eq!(visualizer.veil_transparency(), 0.5);
        assert!(visualizer.is_overlay_visible());
        assert_eq!(visualizer.filter_range(), (0.0, 1.0));
    }

    #[test]
    fn test_spectrum_filter() {
        let mut visualizer = SpectrumVisualizer::new();

        // Filter to mid space/time region
        visualizer.select_region(SpectrumRegion::MidSpaceTime);

        assert!(visualizer.passes_filter(0.7));
        assert!(!visualizer.passes_filter(0.2));
        assert!(!visualizer.passes_filter(0.9));
    }

    #[test]
    fn test_spectrum_filter_clear() {
        let mut visualizer = SpectrumVisualizer::new();

        visualizer.select_region(SpectrumRegion::MidSpaceTime);
        assert_eq!(
            visualizer.selected_region(),
            Some(SpectrumRegion::MidSpaceTime)
        );

        visualizer.clear_filter();
        assert_eq!(visualizer.selected_region(), None);
        assert_eq!(visualizer.filter_range(), (0.0, 1.0));
    }

    #[test]
    fn test_spectrum_overlay() {
        let overlay = SpectrumOverlay::new(100.0, 500.0, 800.0, 50.0);
        assert_eq!(overlay.orientation, SpectrumOrientation::Horizontal);

        let screen_pos = overlay.spectrum_to_screen(0.5);
        assert_eq!(screen_pos, (500.0, 525.0));

        let spectrum_pos = overlay.screen_to_spectrum(500.0, 525.0);
        assert_eq!(spectrum_pos, Some(0.5));
    }

    #[test]
    fn test_veil_position() {
        let visualizer = SpectrumVisualizer::new();
        assert_eq!(visualizer.veil_position(), 0.5);
    }

    #[test]
    fn test_entity_to_spectrum_data() {
        let visualizer = SpectrumVisualizer::new();

        let spectrum_pos = SpectrumPosition::from_ratio(1.0);
        let color = [1.0_f32, 0.0_f32, 0.0_f32, 1.0_f32];

        let data =
            visualizer.entity_to_spectrum_data(123, spectrum_pos, Density::Fourth, color, false);

        assert_eq!(data.entity_id, 123);
        assert_eq!(data.density, 4.0_f32);
        assert_eq!(data.spectrum_position, 0.5);
        assert_eq!(data.selected, 0);
    }

    #[test]
    fn test_gradient_colors() {
        let visualizer = SpectrumVisualizer::new();
        let colors = visualizer.get_gradient_colors();
        assert_eq!(colors.len(), 7);

        // Veil color should be white
        let veil_color = colors[3];
        assert!(veil_color[0] > 0.9_f32 && veil_color[1] > 0.9_f32 && veil_color[2] > 0.9_f32);
    }

    #[test]
    fn test_distance_from_veil() {
        let veil_pos = SpectrumPosition::from_ratio(1.0);
        assert_eq!(veil_pos.distance_from_veil(), 0.0);

        let far_pos = SpectrumPosition::from_ratio(100.0);
        assert!(far_pos.distance_from_veil() > 0.0);
    }

    #[test]
    fn test_region_names() {
        assert_eq!(SpectrumRegion::TheVeil.name(), "The Veil");
        assert_eq!(SpectrumRegion::DeepTimeSpace.name(), "Deep Time/Space");
        assert_eq!(SpectrumRegion::DeepSpaceTime.name(), "Deep Space/Time");
    }

    #[test]
    fn test_region_colors() {
        let veil_color = SpectrumRegion::TheVeil.color();
        assert!(veil_color[0] > 0.9_f32 && veil_color[1] > 0.9_f32 && veil_color[2] > 0.9_f32);

        let deep_ts_color = SpectrumRegion::DeepTimeSpace.color();
        assert!(deep_ts_color[2] > 0.4_f32); // Purple/blue
    }
}
