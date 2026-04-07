//! Entity data extraction and GPU upload.
//!
//! Converts simulation entities (`SubSubLogos`) into GPU-friendly flat structs
//! and creates storage buffers for compute shader processing.
//!
//! # EntityGPUData Layout
//!
//! Exact layout from `.sisyphus/plans/wgsl-architecture.md` lines 186-263.
//! Total size: 208 bytes (array stride for WGSL storage buffer).

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::polarization::{PolarityDirection, PolarizationState};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::buffers::GpuBuffer;

// ============================================================================
// EntityGPUData — GPU entity representation (208 bytes)
// ============================================================================

/// Flattened entity data for GPU processing.
///
/// # Layout (from wgsl-architecture.md):
/// | Field                    | Offset | Size  |
/// |--------------------------|--------|-------|
/// | entity_id_hash           | 0      | 4     |
/// | incarnation_number       | 4      | 4     |
/// | _pad0                    | 8      | 4     |
/// | spectrum_ratio           | 12     | 4     |
/// | space_time_access        | 16     | 4     |
/// | oneness_access           | 20     | 4     |
/// | veil_transparency        | 24     | 4     |
/// | frequency                | 28     | 4     |
/// | coherence                | 32     | 4     |
/// | _pad1                    | 36     | 4     |
/// | archetype_activation[22] | 40     | 88    |
/// | _pad2                    | 128    | 8     |
/// | polarity_direction       | 136    | 4     |
/// | polarity_strength        | 140    | 4     |
/// | evolutionary_rate        | 144    | 4     |
/// | _pad3                    | 148    | 4     |
/// | holographic_signature[8] | 152    | 32    |
/// | density_level            | 184    | 4     |
/// | _pad4[3]                 | 188    | 12    |
/// | _pad5[2]                 | 200    | 8     |
/// | **Total**                | —      | **208** |
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityGPUData {
    // Entity ID packed as 2x u32 (hash of UUID string + incarnation_number)
    pub entity_id_hash: u32,
    pub incarnation_number: u32,
    pub _pad0: u32,

    // Spectrum configuration (from spectrum_configuration.ratio)
    pub spectrum_ratio: f32,
    pub space_time_access: f32,
    pub oneness_access: f32,
    pub veil_transparency: f32,

    // Vibrational state
    pub frequency: f32,
    pub coherence: f32,
    pub _pad1: u32,

    // Archetype activation (22 × f32 = 88 bytes)
    pub archetype_activation: [f32; 22],
    // Padding to match WGSL vec2 alignment after archetype block
    pub _pad2: [f32; 2],

    // Polarity (ServiceToOthers=1.0, ServiceToSelf=-1.0, Neutral=0.0)
    pub polarity_direction: f32,
    pub polarity_strength: f32,
    pub evolutionary_rate: f32,
    pub _pad3: f32,

    // Holographic signature (8 × f32 = 32 bytes)
    pub holographic_signature: [f32; 8],

    // Density level (1-8 encoded as f32)
    pub density_level: f32,
    pub _pad4: [f32; 3],
    // Round to 208 bytes (WGSL array_stride multiple of 16)
    pub _pad5: [f32; 2],
}

// Compile-time size assertion: must be exactly 208 bytes
const _: () = assert!(
    std::mem::size_of::<EntityGPUData>() == 208,
    "EntityGPUData must be exactly 208 bytes for WGSL array_stride alignment"
);

// Compile-time alignment check: must be a multiple of 16 for WGSL
const _: () = assert!(
    std::mem::size_of::<EntityGPUData>() % 16 == 0,
    "EntityGPUData size must be a multiple of 16 bytes"
);

impl EntityGPUData {
    /// The array stride for WGSL storage buffers (bytes per entity).
    pub const ARRAY_STRIDE: u64 = 208;

    /// Create a zero-initialized EntityGPUData.
    pub fn zeroed() -> Self {
        bytemuck::Zeroable::zeroed()
    }
}

// ============================================================================
// Field mapping helpers
// ============================================================================

/// Hash an EntityId's UUID string to a u32 for GPU upload.
fn hash_entity_id(entity_id: &EntityId) -> u32 {
    let mut hasher = DefaultHasher::new();
    entity_id.uuid.hash(&mut hasher);
    // Also hash incarnation_number into the same hash for uniqueness
    entity_id.incarnation_number.hash(&mut hasher);
    let h = hasher.finish();
    // Fold 64-bit hash to 32-bit
    ((h >> 32) ^ h) as u32
}

/// Map polarization direction to f32: STO=1.0, STS=-1.0, Neutral=0.0
fn polarity_direction_to_f32(state: PolarizationState, direction: PolarityDirection) -> f32 {
    match state {
        PolarizationState::Unpolarized => 0.0,
        PolarizationState::STOLeaning
        | PolarizationState::PolarizedSTO
        | PolarizationState::HarvestableSTO => match direction {
            PolarityDirection::ServiceToOthers => 1.0,
            PolarityDirection::ServiceToSelf => -1.0,
            PolarityDirection::Neutral => 0.5, // Leaning STO but neutral direction
        },
        PolarizationState::STSLeaning
        | PolarizationState::PolarizedSTS
        | PolarizationState::HarvestableSTS => match direction {
            PolarityDirection::ServiceToSelf => -1.0,
            PolarityDirection::ServiceToOthers => 1.0,
            PolarityDirection::Neutral => -0.5, // Leaning STS but neutral direction
        },
    }
}

/// Map Density enum to f32 level (1.0 through 8.0)
fn density_to_f32(density: &crate::evolution_density_octave::density_octave::Density) -> f32 {
    use crate::evolution_density_octave::density_octave::Density;
    match density {
        Density::First(_) => 1.0,
        Density::Second(_) => 2.0,
        Density::Third => 3.0,
        Density::Fourth => 4.0,
        Density::Fifth => 5.0,
        Density::Sixth => 6.0,
        Density::Seventh => 7.0,
        Density::Eighth => 8.0,
    }
}

/// Extract holographic signature from entity.
///
/// Uses energy and consciousness fields as an 8-element signature.
// TODO: Verify field mapping — this is a reasonable approximation
fn extract_holographic_signature(entity: &SubSubLogos) -> [f32; 8] {
    [
        entity.consciousness_level as f32,
        entity.experience_accumulation as f32,
        entity.learning_progress as f32,
        entity.potential_energy as f32,
        entity.kinetic_energy as f32,
        entity.energy as f32,
        entity.spectrum_position as f32,
        entity.evolution_clock as f32,
    ]
}

// ============================================================================
// Public API
// ============================================================================

/// Extract entity data from simulation entities into GPU-ready format.
///
/// Converts a HashMap of SubSubLogos entities into a Vec of EntityGPUData
/// suitable for direct upload to a GPU storage buffer.
///
/// # Field Mapping
///
/// | GPU Field                | Source Field                    | Notes                    |
/// |--------------------------|---------------------------------|--------------------------|
/// | entity_id_hash           | hash(entity_id.uuid)            | 64→32 bit hash           |
/// | incarnation_number       | entity_id.incarnation_number    | usize→u32                |
/// | spectrum_ratio           | spectrum_access.ratio           | f64→f32                  |
/// | space_time_access        | spectrum_access.space_time_access| f64→f32                 |
/// | oneness_access           | spectrum_access.time_space_access| f64→f32                 |
/// | veil_transparency        | veil_transparency               | f64→f32                  |
/// | frequency                | consciousness_level             | f64→f32                  |
/// | coherence                | energy (normalized)             | f64→f32                  |
/// | archetype_activation[22] | archetype_activations           | [f64;22]→[f32;22]        |
/// | polarity_direction       | polarization state+direction    | enum→f32 (±1.0/0.0)     |
/// | polarity_strength        | polarization.intensity          | f64→f32                  |
/// | evolutionary_rate        | evolutionary_rate               | f64→f32                  |
/// | holographic_signature[8] | derived from energy/consciousness| See extract fn           |
/// | density_level            | current_density                 | enum→f32 (1.0–8.0)       |
///
/// TODO: Verify field mapping with architecture team
pub fn extract_entity_data(entities: &HashMap<EntityId, SubSubLogos>) -> Vec<EntityGPUData> {
    entities
        .values()
        .map(|entity| {
            // TODO: Verify field mapping
            let polarization = &entity.polarization;
            let polarity_dir =
                polarity_direction_to_f32(polarization.state, polarization.direction);

            // Coherence: use normalized total energy (clamped to 0-1 range)
            // A reasonable measure of entity "coherence" for GPU compute
            let coherence = (entity.energy.clamp(0.0, 1.0)) as f32;

            EntityGPUData {
                entity_id_hash: hash_entity_id(&entity.entity_id),
                incarnation_number: entity.entity_id.incarnation_number as u32,
                _pad0: 0,
                spectrum_ratio: entity.spectrum_access.ratio as f32,
                space_time_access: entity.spectrum_access.space_time_access as f32,
                oneness_access: entity.spectrum_access.time_space_access as f32,
                veil_transparency: entity.veil_transparency as f32,
                frequency: entity.consciousness_level as f32,
                coherence,
                _pad1: 0,
                archetype_activation: {
                    let mut arr = [0.0f32; 22];
                    for (i, &val) in entity.archetype_activations.iter().enumerate() {
                        arr[i] = val as f32;
                    }
                    arr
                },
                _pad2: [0.0; 2],
                polarity_direction: polarity_dir,
                polarity_strength: polarization.intensity as f32,
                evolutionary_rate: entity.evolutionary_rate as f32,
                _pad3: 0.0,
                holographic_signature: extract_holographic_signature(entity),
                density_level: density_to_f32(&entity.current_density),
                _pad4: [0.0; 3],
                _pad5: [0.0; 2],
            }
        })
        .collect()
}

/// Create a GPU storage buffer from entity data.
///
/// The buffer is created with `STORAGE | COPY_DST` usage, suitable for
/// compute shader read access.
pub fn create_entity_buffer(device: &wgpu::Device, entities: &[EntityGPUData]) -> GpuBuffer {
    if entities.is_empty() {
        return GpuBuffer::new(
            device,
            EntityGPUData::ARRAY_STRIDE,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            Some("entity_buffer_empty"),
        );
    }

    GpuBuffer::from_slice(
        device,
        entities,
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        Some("entity_buffer"),
    )
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_gpu_data_size() {
        assert_eq!(
            std::mem::size_of::<EntityGPUData>(),
            208,
            "EntityGPUData must be exactly 208 bytes"
        );
    }

    #[test]
    fn test_entity_gpu_data_alignment() {
        assert_eq!(
            std::mem::align_of::<EntityGPUData>(),
            4,
            "EntityGPUData should have 4-byte alignment (no f64 fields)"
        );
    }

    #[test]
    fn test_entity_gpu_data_array_stride() {
        assert_eq!(EntityGPUData::ARRAY_STRIDE, 208);
        assert_eq!(
            EntityGPUData::ARRAY_STRIDE % 16,
            0,
            "Array stride must be a multiple of 16 for WGSL"
        );
    }

    #[test]
    fn test_entity_gpu_data_field_offsets() {
        // Verify exact field layout matches WGSL spec
        let data = EntityGPUData::zeroed();
        let base = &data as *const EntityGPUData as usize;

        // entity_id_hash: offset 0
        assert_eq!(
            (&data.entity_id_hash as *const u32 as usize) - base,
            0,
            "entity_id_hash offset"
        );

        // incarnation_number: offset 4
        assert_eq!(
            (&data.incarnation_number as *const u32 as usize) - base,
            4,
            "incarnation_number offset"
        );

        // spectrum_ratio: offset 12
        assert_eq!(
            (&data.spectrum_ratio as *const f32 as usize) - base,
            12,
            "spectrum_ratio offset"
        );

        // space_time_access: offset 16
        assert_eq!(
            (&data.space_time_access as *const f32 as usize) - base,
            16,
            "space_time_access offset"
        );

        // oneness_access: offset 20
        assert_eq!(
            (&data.oneness_access as *const f32 as usize) - base,
            20,
            "oneness_access offset"
        );

        // veil_transparency: offset 24
        assert_eq!(
            (&data.veil_transparency as *const f32 as usize) - base,
            24,
            "veil_transparency offset"
        );

        // frequency: offset 28
        assert_eq!(
            (&data.frequency as *const f32 as usize) - base,
            28,
            "frequency offset"
        );

        // coherence: offset 32
        assert_eq!(
            (&data.coherence as *const f32 as usize) - base,
            32,
            "coherence offset"
        );

        // archetype_activation: offset 40
        assert_eq!(
            (&data.archetype_activation as *const [f32; 22] as usize) - base,
            40,
            "archetype_activation offset"
        );

        // polarity_direction: offset 136 (40 + 88 + 8 padding)
        assert_eq!(
            (&data.polarity_direction as *const f32 as usize) - base,
            136,
            "polarity_direction offset"
        );

        // polarity_strength: offset 140
        assert_eq!(
            (&data.polarity_strength as *const f32 as usize) - base,
            140,
            "polarity_strength offset"
        );

        // evolutionary_rate: offset 144
        assert_eq!(
            (&data.evolutionary_rate as *const f32 as usize) - base,
            144,
            "evolutionary_rate offset"
        );

        // holographic_signature: offset 152
        assert_eq!(
            (&data.holographic_signature as *const [f32; 8] as usize) - base,
            152,
            "holographic_signature offset"
        );

        // density_level: offset 184
        assert_eq!(
            (&data.density_level as *const f32 as usize) - base,
            184,
            "density_level offset"
        );

        // _pad4 starts at offset 188
        assert_eq!(
            (&data._pad4 as *const [f32; 3] as usize) - base,
            188,
            "_pad4 offset"
        );

        // _pad5 starts at offset 200 (rounds struct to 208)
        assert_eq!(
            (&data._pad5 as *const [f32; 2] as usize) - base,
            200,
            "_pad5 offset"
        );
    }

    #[test]
    fn test_entity_gpu_data_is_pod() {
        // Verify the struct is truly Pod (can be safely transmuted)
        let data = EntityGPUData::zeroed();
        let bytes = bytemuck::bytes_of(&data);
        assert_eq!(bytes.len(), 208);
    }

    #[test]
    fn test_extract_empty() {
        let entities: HashMap<EntityId, SubSubLogos> = HashMap::new();
        let gpu_data = extract_entity_data(&entities);
        assert!(gpu_data.is_empty());
    }

    #[test]
    fn test_hash_entity_id_deterministic() {
        let id1 = EntityId {
            uuid: "test-uuid-1".to_string(),
            incarnation_number: 1,
        };
        let id2 = EntityId {
            uuid: "test-uuid-1".to_string(),
            incarnation_number: 1,
        };
        let id3 = EntityId {
            uuid: "test-uuid-1".to_string(),
            incarnation_number: 2,
        };

        assert_eq!(
            hash_entity_id(&id1),
            hash_entity_id(&id2),
            "Same ID should produce same hash"
        );
        assert_ne!(
            hash_entity_id(&id1),
            hash_entity_id(&id3),
            "Different incarnation should produce different hash"
        );
    }

    #[test]
    fn test_density_to_f32() {
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};

        assert_eq!(
            density_to_f32(&Density::First(Density1SubLevel::Quantum)),
            1.0
        );
        assert_eq!(density_to_f32(&Density::Third), 3.0);
        assert_eq!(density_to_f32(&Density::Eighth), 8.0);
    }

    #[test]
    fn test_polarity_direction_mapping() {
        assert_eq!(
            polarity_direction_to_f32(PolarizationState::Unpolarized, PolarityDirection::Neutral),
            0.0
        );
        assert_eq!(
            polarity_direction_to_f32(
                PolarizationState::PolarizedSTO,
                PolarityDirection::ServiceToOthers
            ),
            1.0
        );
        assert_eq!(
            polarity_direction_to_f32(
                PolarizationState::PolarizedSTS,
                PolarityDirection::ServiceToSelf
            ),
            -1.0
        );
    }
}
