//! Field Visual Bridge - Connect Holographic Field State to GPU Rendering
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.1:
//! "Render phase coherence networks, Show interference patterns"
//!
//! This module provides:
//! - Bridge between HolographicFieldState and FieldVolumeData
//! - Sampling strategies for field visualization
//! - Coherence network extraction
//! - Interference pattern visualization

use crate::hpo::{Complex, HolographicFieldState};
use crate::types::Float;

use super::volume_texture::{FieldVolumeData, VolumeDimensions};

/// Bridge for converting field state to visualizable data
pub struct FieldVisualBridge {
    /// Volume dimensions for sampling
    dimensions: VolumeDimensions,
    /// Sample spacing
    #[allow(dead_code)]
    sample_spacing: Float,
    /// Minimum coherence threshold for visualization
    #[allow(dead_code)]
    coherence_threshold: Float,
    /// Enable interpolation between samples
    #[allow(dead_code)]
    interpolate: bool,
}

impl FieldVisualBridge {
    /// Create a new field visual bridge
    pub fn new(dimensions: VolumeDimensions) -> Self {
        let sample_spacing = 2.0
            / dimensions
                .width
                .max(dimensions.height)
                .max(dimensions.depth) as Float;

        Self {
            dimensions,
            sample_spacing,
            coherence_threshold: 0.01,
            interpolate: true,
        }
    }

    /// Sample field state into volume data
    pub fn sample_field(&self, field_state: &HolographicFieldState) -> FieldVolumeData {
        let mut data = FieldVolumeData::new(self.dimensions);
        let bounds = &field_state.root.bounds;

        for z in 0..self.dimensions.depth {
            for y in 0..self.dimensions.height {
                for x in 0..self.dimensions.width {
                    let idx = data.index(x, y, z);

                    // Convert grid position into actual simulation field bounds
                    let pos = self.grid_to_field_position_in_bounds(x, y, z, bounds);

                    // Sample from field state using octree traversal
                    if let Some(node) = self.get_node_at_position(field_state, &pos) {
                        let (coherence, amplitude, phase) = self.sample_node(&node.field_data);
                        data.coherence[idx] = coherence;
                        data.amplitude[idx] = amplitude;
                        data.phase[idx] = phase;

                        // Determine density band from spectrum position
                        data.density[idx] = self.determine_density_band(&node.field_data);
                    } else {
                        // Fill with default values if position is outside field bounds
                        data.coherence[idx] = 0.0;
                        data.amplitude[idx] = 0.0;
                        data.phase[idx] = 0.0;
                        data.density[idx] = 0;
                    }
                }
            }
        }

        data
    }

    /// Get node at a position by traversing the octree
    fn get_node_at_position<'a>(
        &self,
        field_state: &'a HolographicFieldState,
        pos: &[Float; 3],
    ) -> Option<&'a crate::hpo::OctreeNode> {
        

        // Check if position is within field bounds
        if !field_state.root.bounds.contains(pos) {
            return None;
        }

        // Traverse to find the leaf node at this position
        let mut current = &field_state.root;

        while current.subdivided {
            if let Some(ref children) = current.children {
                let index = current.child_index(pos);
                current = &children[index];
            } else {
                break;
            }
        }

        Some(current)
    }

    /// Create procedural field data for testing
    pub fn create_test_field(&self) -> FieldVolumeData {
        let mut data = FieldVolumeData::new(self.dimensions);

        let center_x = self.dimensions.width as Float / 2.0;
        let center_y = self.dimensions.height as Float / 2.0;
        let center_z = self.dimensions.depth as Float / 2.0;

        let max_dist = (center_x * center_x + center_y * center_y + center_z * center_z).sqrt();

        for z in 0..self.dimensions.depth {
            for y in 0..self.dimensions.height {
                for x in 0..self.dimensions.width {
                    let idx = data.index(x, y, z);

                    // Create spherical coherence pattern
                    let dx = x as Float - center_x;
                    let dy = y as Float - center_y;
                    let dz = z as Float - center_z;
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

                    // Radial coherence gradient
                    data.coherence[idx] = 1.0 - (dist / max_dist).min(1.0);

                    // Sinusoidal amplitude pattern
                    let angle = dist * 0.1;
                    data.amplitude[idx] = (angle.sin() * 0.5 + 0.5) * data.coherence[idx];

                    // Radial phase pattern
                    data.phase[idx] = (dist * 0.2) % std::f64::consts::TAU as Float;

                    // Density band based on distance
                    data.density[idx] = ((dist / max_dist) * 7.0).min(7.0) as u8;
                }
            }
        }

        data
    }

    /// Create field data showing the 7 involution layers
    pub fn create_layered_field(&self) -> FieldVolumeData {
        let mut data = FieldVolumeData::new(self.dimensions);

        // Spatial frequencies for each layer (from COSMOLOGICAL-ARCHITECTURE.md)
        // Violet: 5600 lines/mm → Red: 100 lines/mm
        let layer_frequencies: [Float; 7] = [
            5600.0, // Violet
            2800.0, // Indigo
            1400.0, // Blue
            700.0,  // Green
            350.0,  // Yellow
            175.0,  // Orange
            100.0,  // Red
        ];

        let scale = 0.01; // Scale factor for visualization
        let tau = std::f64::consts::TAU as Float;

        for z in 0..self.dimensions.depth {
            for y in 0..self.dimensions.height {
                for x in 0..self.dimensions.width {
                    let idx = data.index(x, y, z);

                    // Normalized position
                    let nx = x as Float / self.dimensions.width as Float;
                    let ny = y as Float / self.dimensions.height as Float;
                    let nz = z as Float / self.dimensions.depth as Float;

                    // Sum contributions from all layers
                    let mut total_coherence = 0.0;
                    let mut total_amplitude = 0.0;
                    let mut total_phase = 0.0;

                    for (i, &freq) in layer_frequencies.iter().enumerate() {
                        let layer_weight = 1.0 / (i + 1) as Float;
                        let f = freq * scale;

                        // 3D sinusoidal pattern
                        let pattern =
                            (nx * f * tau).sin() * (ny * f * tau).sin() * (nz * f * tau).cos();

                        total_coherence += pattern.abs() * layer_weight;
                        total_amplitude += pattern * layer_weight;
                        total_phase +=
                            (pattern.atan2(0.0) + std::f64::consts::PI as Float) * layer_weight;
                    }

                    data.coherence[idx] = (total_coherence / 7.0).clamp(0.0, 1.0);
                    data.amplitude[idx] = (total_amplitude / 7.0).clamp(0.0, 1.0);
                    data.phase[idx] = (total_phase / 7.0) % tau;

                    // Determine dominant layer
                    let radial =
                        ((nx - 0.5).powi(2) + (ny - 0.5).powi(2) + (nz - 0.5).powi(2)).sqrt();
                    data.density[idx] = (radial * 14.0).min(7.0) as u8;
                }
            }
        }

        data
    }

    /// Grid coordinates to field position in explicit field bounds
    fn grid_to_field_position_in_bounds(
        &self,
        x: u32,
        y: u32,
        z: u32,
        bounds: &crate::hpo::FieldBounds,
    ) -> [Float; 3] {
        let nx = if self.dimensions.width > 1 {
            x as Float / (self.dimensions.width - 1) as Float
        } else {
            0.5
        };
        let ny = if self.dimensions.height > 1 {
            y as Float / (self.dimensions.height - 1) as Float
        } else {
            0.5
        };
        let nz = if self.dimensions.depth > 1 {
            z as Float / (self.dimensions.depth - 1) as Float
        } else {
            0.5
        };

        [
            bounds.min[0] + (bounds.max[0] - bounds.min[0]) * nx,
            bounds.min[1] + (bounds.max[1] - bounds.min[1]) * ny,
            bounds.min[2] + (bounds.max[2] - bounds.min[2]) * nz,
        ]
    }

    /// Sample coherence, amplitude, and phase from field node data
    fn sample_node(&self, field_data: &crate::hpo::FieldNodeData) -> (Float, Float, Float) {
        let coherence = field_data.coherence;

        // Average amplitude across all density bands
        let avg_amplitude: Float = field_data
            .density_amplitudes
            .iter()
            .map(|c| c.magnitude())
            .sum::<Float>()
            / field_data.density_amplitudes.len() as Float;

        // Phase from first density amplitude (simplified)
        let phase = field_data
            .density_amplitudes.first()
            .map(|c| c.phase())
            .unwrap_or(0.0);

        (coherence, avg_amplitude, phase)
    }

    /// Determine density band from spectrum position
    fn determine_density_band(&self, field_data: &crate::hpo::FieldNodeData) -> u8 {
        // Map spectrum position (0.0 - 1.0) to density bands (1-8)
        let band = (field_data.spectrum_position * 8.0) as u8;
        band.min(7)
    }

    /// Extract phase coherence network edges
    pub fn extract_phase_network(
        &self,
        field_state: &HolographicFieldState,
        threshold: Float,
    ) -> Vec<PhaseCoherenceEdge> {
        let mut edges = Vec::new();
        let bounds = &field_state.root.bounds;

        // Sample at regular intervals
        let step = 8u32;

        for z in (0..self.dimensions.depth).step_by(step as usize) {
            for y in (0..self.dimensions.height).step_by(step as usize) {
                for x in (0..self.dimensions.width).step_by(step as usize) {
                    let pos = self.grid_to_field_position_in_bounds(x, y, z, bounds);

                    if let Some(node) = self.get_node_at_position(field_state, &pos) {
                        if node.field_data.coherence < threshold {
                            continue;
                        }

                        // Check neighbors in each direction
                        let offsets: [(i32, i32, i32); 6] = [
                            (step as i32, 0, 0),
                            (-(step as i32), 0, 0),
                            (0, step as i32, 0),
                            (0, -(step as i32), 0),
                            (0, 0, step as i32),
                            (0, 0, -(step as i32)),
                        ];

                        for (dx, dy, dz) in offsets {
                            let nx = (x as i32 + dx) as u32;
                            let ny = (y as i32 + dy) as u32;
                            let nz = (z as i32 + dz) as u32;

                            if nx >= self.dimensions.width
                                || ny >= self.dimensions.height
                                || nz >= self.dimensions.depth
                            {
                                continue;
                            }

                            let neighbor_pos =
                                self.grid_to_field_position_in_bounds(nx, ny, nz, bounds);

                            if let Some(neighbor) =
                                self.get_node_at_position(field_state, &neighbor_pos)
                            {
                                if neighbor.field_data.coherence < threshold {
                                    continue;
                                }

                                // Calculate phase coherence
                                let phase_diff = Self::phase_difference(
                                    &node.field_data.density_amplitudes,
                                    &neighbor.field_data.density_amplitudes,
                                );

                                if phase_diff > 0.7 {
                                    edges.push(PhaseCoherenceEdge {
                                        start: [pos[0] as f32, pos[1] as f32, pos[2] as f32],
                                        end: [
                                            neighbor_pos[0] as f32,
                                            neighbor_pos[1] as f32,
                                            neighbor_pos[2] as f32,
                                        ],
                                        coherence: phase_diff,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        edges
    }

    /// Calculate phase difference between two sets of amplitudes
    fn phase_difference(amps1: &[Complex; 8], amps2: &[Complex; 8]) -> Float {
        if amps1.is_empty() || amps2.is_empty() {
            return 0.0;
        }

        let mut total_diff = 0.0;
        let mut count = 0usize;

        for (a1, a2) in amps1.iter().zip(amps2.iter()) {
            let diff = (a1.phase() - a2.phase()).abs();
            total_diff += diff.min(2.0 * std::f64::consts::PI as Float - diff);
            count += 1;
        }

        if count > 0 {
            let avg_diff = total_diff / count as Float;
            // Normalize to 0.0 - 1.0 (0 = no coherence, 1 = perfect coherence)
            let coherence = 1.0 - (avg_diff / std::f64::consts::PI as Float);
            coherence.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Edge representing phase coherence between two field points
#[derive(Debug, Clone)]
pub struct PhaseCoherenceEdge {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub coherence: Float,
}
