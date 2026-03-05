//! Buffer Management - Vertex, index, and uniform buffers
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md:
//! "Implement buffer management for circle geometry and screen parameters"

use crate::entity_layer7::layer7::{DensityLevel, SubSubLogos};
use wgpu::{util::DeviceExt, Buffer, BufferDescriptor, BufferUsages, Device};

/// Buffer manager for all GPU buffers
#[derive(Debug)]
pub struct BufferManager {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    uniform_buffer: Buffer,
    num_indices: u32,
}

impl BufferManager {
    /// Create new buffer manager
    pub fn new(device: &Device, screen_width: u32, screen_height: u32) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(device);
        let index_buffer = Self::create_index_buffer(device);
        let uniform_buffer = Self::create_uniform_buffer(device, screen_width, screen_height);

        BufferManager {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            num_indices: 6, // Two triangles for quad
        }
    }

    /// Create vertex buffer for circle geometry
    fn create_vertex_buffer(device: &Device) -> Buffer {
        let vertices = vec![
            // Position (x, y), Color (r, g, b, a)
            [-1.0f32, -1.0, 1.0, 1.0, 1.0, 1.0], // Bottom-left
            [1.0f32, -1.0, 1.0, 1.0, 1.0, 1.0],  // Bottom-right
            [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0],   // Top-right
            [-1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0],  // Top-left
        ];

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        })
    }

    /// Create index buffer for circle geometry
    fn create_index_buffer(device: &Device) -> Buffer {
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        })
    }

    /// Create uniform buffer for screen parameters
    fn create_uniform_buffer(device: &Device, width: u32, height: u32) -> Buffer {
        let uniforms = [width as f32, height as f32, 1.0f32, 0.0f32, 0.0f32];

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&uniforms),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        })
    }

    /// Update uniform buffer with new screen size and camera parameters
    pub fn update_uniforms(
        &self,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        zoom: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        let uniforms = [
            width as f32,
            height as f32,
            zoom,
            offset_x,
            offset_y,
            0.0f32,
        ];
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&uniforms));
    }

    /// Get vertex buffer
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    /// Get index buffer
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    /// Get uniform buffer
    pub fn uniform_buffer(&self) -> &Buffer {
        &self.uniform_buffer
    }

    /// Get number of indices
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    /// Update instance data for a single entity
    pub fn update_instance_data(
        &self,
        queue: &wgpu::Queue,
        _offset: u32,
        x: f32,
        y: f32,
        radius: f32,
        color: [f32; 4],
    ) {
        let instance_data = [x, y, radius, color[0], color[1], color[2], color[3], 0.0f32];
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&instance_data));
    }
}

/// Circle buffers for entity rendering
pub struct CircleBuffers {
    pub instance_buffer: Buffer,
    pub max_entities: usize,
}

impl CircleBuffers {
    /// Create circle instance buffers
    pub fn new(device: &Device, max_entities: usize) -> Self {
        let instance_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Entity Instance Buffer"),
            size: (std::mem::size_of::<EntityInstanceData>() * max_entities) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        CircleBuffers {
            instance_buffer,
            max_entities,
        }
    }

    /// Update instance buffer with entity data
    pub fn update_entities(&self, queue: &wgpu::Queue, entities: &[SubSubLogos]) {
        let instances: Vec<EntityInstanceData> = entities
            .iter()
            .map(EntityInstanceData::from_entity)
            .collect();

        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    /// Get instance buffer
    pub fn instance_buffer(&self) -> &Buffer {
        &self.instance_buffer
    }
}

/// Entity instance data for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityInstanceData {
    position: [f32; 2],
    color: [f32; 4],
    radius: f32,
}

/// 3D Entity instance data for instanced rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Entity3DInstanceData {
    pub model_matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub radius: f32,
    pub density: f32,
    pub polarity: f32,
    pub _padding: [f32; 3],
}

impl Entity3DInstanceData {
    /// Create 3D instance data from position and properties
    pub fn from_properties(
        position: [f32; 3],
        color: [f32; 4],
        radius: f32,
        density: f32,
        polarity: f32,
    ) -> Self {
        let mut model_matrix = [[0.0f32; 4]; 4];

        // Identity matrix with translation
        model_matrix[0][0] = 1.0;
        model_matrix[1][1] = 1.0;
        model_matrix[2][2] = 1.0;
        model_matrix[3][3] = 1.0;
        model_matrix[3][0] = position[0];
        model_matrix[3][1] = position[1];
        model_matrix[3][2] = position[2];

        Entity3DInstanceData {
            model_matrix,
            color,
            radius,
            density,
            polarity,
            _padding: [0.0; 3],
        }
    }
}

/// Instance buffer manager for 3D instanced rendering
#[derive(Debug)]
pub struct InstanceBufferManager {
    instance_buffer: wgpu::Buffer,
    max_instances: usize,
}

impl InstanceBufferManager {
    /// Create new instance buffer manager
    pub fn new(device: &wgpu::Device, max_instances: usize) -> Self {
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("3D Instance Buffer"),
            size: (std::mem::size_of::<Entity3DInstanceData>() * max_instances) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        InstanceBufferManager {
            instance_buffer,
            max_instances,
        }
    }

    /// Update instance buffer with entity data
    pub fn update_instances(&self, queue: &wgpu::Queue, instances: &[Entity3DInstanceData]) {
        let count = instances.len().min(self.max_instances);
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instances[..count]),
        );
    }

    /// Get instance buffer
    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    /// Get max instances
    pub fn max_instances(&self) -> usize {
        self.max_instances
    }
}

impl EntityInstanceData {
    /// Create instance data from entity
    pub fn from_entity(entity: &SubSubLogos) -> Self {
        let position = Self::get_entity_position(entity);
        let color = Self::get_entity_color(entity);
        let radius = Self::get_entity_radius(entity);

        EntityInstanceData {
            position,
            color,
            radius,
        }
    }

    /// Get entity screen position
    fn get_entity_position(entity: &SubSubLogos) -> [f32; 2] {
        // Use entity ID string hash to generate a deterministic position for now
        // In a real implementation, this would use the entity's actual 3D position
        let id_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            entity.entity_id.uuid.hash(&mut hasher);
            entity.entity_id.incarnation_number.hash(&mut hasher);
            hasher.finish()
        };
        let x = ((id_hash & 0xFFFFFFFF) as f32 % 1800.0) - 900.0;
        let y = (((id_hash >> 32) & 0xFFFFFFFF) as f32 % 1000.0) - 500.0;
        [x, y]
    }

    /// Get entity color based on density
    fn get_entity_color(entity: &SubSubLogos) -> [f32; 4] {
        match entity.evolutionary_attractor.current_density {
            DensityLevel::First => [1.0, 0.27, 0.27, 1.0],   // Red
            DensityLevel::Second => [1.0, 0.53, 0.27, 1.0],  // Orange
            DensityLevel::Third => [1.0, 0.8, 0.27, 1.0],    // Yellow
            DensityLevel::Fourth => [0.27, 1.0, 0.27, 1.0],  // Green
            DensityLevel::Fifth => [0.27, 1.0, 1.0, 1.0],    // Cyan
            DensityLevel::Sixth => [0.27, 0.27, 1.0, 1.0],   // Blue
            DensityLevel::Seventh => [0.53, 0.27, 1.0, 1.0], // Violet
            DensityLevel::Eighth => [1.0, 1.0, 1.0, 1.0],    // White
        }
    }

    /// Get entity radius based on density
    fn get_entity_radius(entity: &SubSubLogos) -> f32 {
        // Higher density = slightly larger radius for visual distinction
        match entity.evolutionary_attractor.current_density {
            DensityLevel::First => 15.0,
            DensityLevel::Second => 18.0,
            DensityLevel::Third => 20.0,
            DensityLevel::Fourth => 22.0,
            DensityLevel::Fifth => 25.0,
            DensityLevel::Sixth => 28.0,
            DensityLevel::Seventh => 32.0,
            DensityLevel::Eighth => 36.0,
        }
    }
}
