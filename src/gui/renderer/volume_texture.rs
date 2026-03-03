//! Volume Texture - 3D Texture Infrastructure for Holographic Field Rendering
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.1:
//! "Design field visualization shader, Implement 3D volume rendering for field"
//!
//! This module provides:
//! - 3D texture creation and management
//! - Volumetric data upload
//! - Sampling configuration for raymarching

use crate::types::Float;
use wgpu::*;

/// Volume texture dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeDimensions {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl VolumeDimensions {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn cube(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            depth: size,
        }
    }

    pub fn total_voxels(&self) -> u64 {
        self.width as u64 * self.height as u64 * self.depth as u64
    }

    pub fn total_bytes(&self, bytes_per_voxel: u64) -> u64 {
        self.total_voxels() * bytes_per_voxel
    }
}

impl Default for VolumeDimensions {
    fn default() -> Self {
        Self::cube(64) // Default 64x64x64 volume
    }
}

/// Volume texture format options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeFormat {
    /// Single channel, 8-bit (for density/coherence maps)
    R8Unorm,
    /// Two channels, 8-bit (amplitude + phase)
    Rg8Unorm,
    /// Four channels, 8-bit (RGBA for colored fields)
    Rgba8Unorm,
    /// Single channel, 16-bit float (high precision amplitude)
    R16Float,
    /// Two channels, 16-bit float (amplitude + phase)
    Rg16Float,
    /// Four channels, 16-bit float (RGBA high precision)
    Rgba16Float,
    /// Single channel, 32-bit float (maximum precision)
    R32Float,
    /// Four channels, 32-bit float (RGBA maximum precision)
    Rgba32Float,
}

impl VolumeFormat {
    pub fn wgpu_format(&self) -> TextureFormat {
        match self {
            VolumeFormat::R8Unorm => TextureFormat::R8Unorm,
            VolumeFormat::Rg8Unorm => TextureFormat::Rg8Unorm,
            VolumeFormat::Rgba8Unorm => TextureFormat::Rgba8Unorm,
            VolumeFormat::R16Float => TextureFormat::R16Float,
            VolumeFormat::Rg16Float => TextureFormat::Rg16Float,
            VolumeFormat::Rgba16Float => TextureFormat::Rgba16Float,
            VolumeFormat::R32Float => TextureFormat::R32Float,
            VolumeFormat::Rgba32Float => TextureFormat::Rgba32Float,
        }
    }

    pub fn bytes_per_voxel(&self) -> u64 {
        match self {
            VolumeFormat::R8Unorm => 1,
            VolumeFormat::Rg8Unorm => 2,
            VolumeFormat::Rgba8Unorm => 4,
            VolumeFormat::R16Float => 2,
            VolumeFormat::Rg16Float => 4,
            VolumeFormat::Rgba16Float => 8,
            VolumeFormat::R32Float => 4,
            VolumeFormat::Rgba32Float => 16,
        }
    }

    pub fn num_channels(&self) -> u32 {
        match self {
            VolumeFormat::R8Unorm | VolumeFormat::R16Float | VolumeFormat::R32Float => 1,
            VolumeFormat::Rg8Unorm | VolumeFormat::Rg16Float => 2,
            VolumeFormat::Rgba8Unorm | VolumeFormat::Rgba16Float | VolumeFormat::Rgba32Float => 4,
        }
    }
}

/// Volume texture for 3D field data
pub struct VolumeTexture {
    /// The 3D texture
    pub texture: Texture,
    /// Texture view for sampling
    pub view: TextureView,
    /// Sampler for volumetric access
    pub sampler: Sampler,
    /// Dimensions
    pub dimensions: VolumeDimensions,
    /// Format
    pub format: VolumeFormat,
}

impl VolumeTexture {
    /// Create a new volume texture
    pub fn new(
        device: &Device,
        dimensions: VolumeDimensions,
        format: VolumeFormat,
        label: Option<&str>,
    ) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label,
            size: Extent3d {
                width: dimensions.width,
                height: dimensions.height,
                depth_or_array_layers: dimensions.depth,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D3,
            format: format.wgpu_format(),
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor {
            label: Some("volume_texture_view"),
            format: Some(format.wgpu_format()),
            dimension: Some(TextureViewDimension::D3),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("volume_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        });

        Self {
            texture,
            view,
            sampler,
            dimensions,
            format,
        }
    }

    /// Upload data to the volume texture
    pub fn upload(&self, queue: &Queue, data: &[u8]) {
        let expected_size = self.dimensions.total_bytes(self.format.bytes_per_voxel()) as usize;
        assert_eq!(
            data.len(),
            expected_size,
            "Data size {} doesn't match expected {} for {:?} volume",
            data.len(),
            expected_size,
            self.dimensions
        );

        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
                aspect: TextureAspect::All,
            },
            data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.dimensions.width * self.format.bytes_per_voxel() as u32),
                rows_per_image: Some(self.dimensions.height),
            },
            Extent3d {
                width: self.dimensions.width,
                height: self.dimensions.height,
                depth_or_array_layers: self.dimensions.depth,
            },
        );
    }

    /// Create a bind group layout entry for this volume texture
    pub fn bind_group_layout_entry(binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Texture {
                view_dimension: TextureViewDimension::D3,
                sample_type: TextureSampleType::Float { filterable: true },
                multisampled: false,
            },
            count: None,
        }
    }

    /// Create a bind group layout entry for the sampler
    pub fn sampler_bind_group_layout_entry(
        binding: u32,
        visibility: ShaderStages,
    ) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
        }
    }

    /// Create a bind group for this volume texture
    pub fn create_bind_group(
        &self,
        device: &Device,
        layout: &BindGroupLayout,
        texture_binding: u32,
        sampler_binding: u32,
        label: Option<&str>,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label,
            layout,
            entries: &[
                BindGroupEntry {
                    binding: texture_binding,
                    resource: BindingResource::TextureView(&self.view),
                },
                BindGroupEntry {
                    binding: sampler_binding,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        })
    }
}

/// Field data for volumetric rendering
#[derive(Debug, Clone)]
pub struct FieldVolumeData {
    /// Coherence values (0.0 to 1.0)
    pub coherence: Vec<Float>,
    /// Amplitude magnitude values
    pub amplitude: Vec<Float>,
    /// Phase values (0.0 to 2π)
    pub phase: Vec<Float>,
    /// Density band values (0-7 for 8 densities)
    pub density: Vec<u8>,
    /// Dimensions
    pub dimensions: VolumeDimensions,
}

impl FieldVolumeData {
    /// Create empty field data
    pub fn new(dimensions: VolumeDimensions) -> Self {
        let size = dimensions.total_voxels() as usize;
        Self {
            coherence: vec![0.0; size],
            amplitude: vec![0.0; size],
            phase: vec![0.0; size],
            density: vec![0; size],
            dimensions,
        }
    }

    /// Create from existing field data
    pub fn from_field_state(
        field_state: &crate::holographic_foundation::field_state::HolographicFieldState,
        dimensions: VolumeDimensions,
    ) -> Self {
        let mut data = Self::new(dimensions);

        // Sample field state into volume grid
        for z in 0..dimensions.depth {
            for y in 0..dimensions.height {
                for x in 0..dimensions.width {
                    let idx = data.index(x, y, z);

                    // Convert grid position to field position
                    let pos = crate::holographic_foundation::field_state::Position3D {
                        x: (x as Float / dimensions.width as Float) * 2.0 - 1.0,
                        y: (y as Float / dimensions.height as Float) * 2.0 - 1.0,
                        z: (z as Float / dimensions.depth as Float) * 2.0 - 1.0,
                    };

                    // Sample from field state
                    if let Some((coherence, amplitude, phase)) =
                        Self::sample_field_at(&field_state, &pos)
                    {
                        data.coherence[idx] = coherence;
                        data.amplitude[idx] = amplitude;
                        data.phase[idx] = phase;
                    }
                }
            }
        }

        data
    }

    /// Sample field state at a position
    fn sample_field_at(
        field_state: &crate::holographic_foundation::field_state::HolographicFieldState,
        pos: &crate::holographic_foundation::field_state::Position3D,
    ) -> Option<(Float, Float, Float)> {
        // Find the node at this position
        let node = field_state.get_node_at(pos)?;

        // Get average amplitude and coherence
        let coherence = node.coherence;
        let avg_amplitude: Float = node
            .amplitudes
            .iter()
            .map(|a| (a.real * a.real + a.imaginary * a.imaginary).sqrt())
            .sum::<Float>()
            / node.amplitudes.len() as Float;

        // Get phase from first amplitude
        let phase = node.amplitudes[0].phase();

        Some((coherence, avg_amplitude, phase))
    }

    /// Get linear index for 3D coordinates
    pub fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (z as usize * self.dimensions.height as usize * self.dimensions.width as usize)
            + (y as usize * self.dimensions.width as usize)
            + x as usize
    }

    /// Convert to RGBA texture data for GPU upload
    /// Encodes: R=coherence, G=amplitude, B=phase, A=density
    pub fn to_rgba_f32(&self) -> Vec<[f32; 4]> {
        self.coherence
            .iter()
            .zip(self.amplitude.iter())
            .zip(self.phase.iter())
            .zip(self.density.iter())
            .map(|(((c, a), p), d)| {
                [
                    *c as f32,
                    *a as f32,
                    (*p as f32) / (std::f32::consts::TAU), // Normalize phase to 0-1
                    *d as f32 / 7.0,                       // Normalize density to 0-1
                ]
            })
            .collect()
    }

    /// Convert to bytes for Rgba16Float format
    pub fn to_rgba16_bytes(&self) -> Vec<u8> {
        let rgba = self.to_rgba_f32();
        let mut bytes = Vec::with_capacity(rgba.len() * 8);

        for pixel in rgba {
            for channel in pixel {
                let half = half::f16::from_f32(channel);
                bytes.extend_from_slice(&half.to_bits().to_le_bytes());
            }
        }

        bytes
    }

    /// Convert to bytes for Rgba8Unorm format (lower precision but faster)
    pub fn to_rgba8_bytes(&self) -> Vec<u8> {
        self.coherence
            .iter()
            .zip(self.amplitude.iter())
            .zip(self.phase.iter())
            .zip(self.density.iter())
            .flat_map(|(((c, a), p), d)| {
                vec![
                    (c.clamp(0.0, 1.0) * 255.0) as u8,
                    (a.clamp(0.0, 1.0) * 255.0) as u8,
                    ((p.clamp(0.0, std::f32::consts::TAU as Float)
                        / std::f32::consts::TAU as Float)
                        * 255.0) as u8,
                    ((*d).min(7) * 32) as u8, // Encode density in 3 bits
                ]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_dimensions() {
        let dim = VolumeDimensions::cube(64);
        assert_eq!(dim.width, 64);
        assert_eq!(dim.height, 64);
        assert_eq!(dim.depth, 64);
        assert_eq!(dim.total_voxels(), 64 * 64 * 64);
    }

    #[test]
    fn test_volume_format_bytes() {
        assert_eq!(VolumeFormat::R8Unorm.bytes_per_voxel(), 1);
        assert_eq!(VolumeFormat::Rgba8Unorm.bytes_per_voxel(), 4);
        assert_eq!(VolumeFormat::Rgba16Float.bytes_per_voxel(), 8);
        assert_eq!(VolumeFormat::Rgba32Float.bytes_per_voxel(), 16);
    }

    #[test]
    fn test_field_volume_data() {
        let dim = VolumeDimensions::cube(8);
        let data = FieldVolumeData::new(dim);

        assert_eq!(data.coherence.len(), 8 * 8 * 8);
        assert_eq!(data.amplitude.len(), 8 * 8 * 8);

        let idx = data.index(1, 2, 3);
        assert_eq!(idx, 3 * 64 + 2 * 8 + 1);
    }
}
