//! Entity Instance - GPU representation of an entity
//!
//! Phase 1: Enhanced Instance Data - Captures 7-layer holonic architecture
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md:
//! "The visualization only captures 4 fields out of 50+ fields in each SubSubLogos entity.
//! This represents a 92% visualization gap where the complete 7-layer holonic architecture
//! is invisible."
//!
//! This enhanced structure includes:
//! - 7 realm intensities (violet through red)
//! - Consciousness data (level, polarization)
//! - Spectrum data (space/time ratios, veil transparency)
//! - Evolution data (progress, density level)
//! - Archetype summary (activated count, intensity)

use crate::entity_layer7::layer7::{EntityType, SubSubLogos};
use crate::hpo::RenderableEntity;
use crate::matter::Matter;

/// Instance data for rendering an entity
///
/// Enhanced structure that captures the 7-layer holonic architecture
/// for meaningful visualization beyond just "colorful dots".
///
/// Memory layout (GPU-friendly, aligned):
/// - Total size: 112 bytes
/// - Alignment: 4 bytes (f32/u32)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityInstance {
    // Position in world space (16 bytes, aligned)
    pub position: [f32; 3],
    pub _padding0: f32,

    // Realm intensities - 7 layers of holonic architecture (28 bytes)
    /// Violet Realm intensity: Infinity as undifferentiated unity
    /// From violet_realm.rhythmic_flow * violet_realm.mystery
    pub violet_intensity: f32,
    /// Indigo Realm intensity: IntelligentInfinity + Awareness
    /// From indigo_realm.awareness
    pub indigo_intensity: f32,
    /// Blue Realm intensity: Love/Logos + Creative Principle
    /// From blue_realm.focusing_strength
    pub blue_intensity: f32,
    /// Green Realm intensity: Light/Love field of potential
    /// From green_realm.potential_strength
    pub green_intensity: f32,
    /// Yellow Realm intensity: Dimensional architecture + Veil
    /// From yellow_realm.attractor_field.strength
    pub yellow_intensity: f32,
    /// Orange Realm intensity: Galactic-scale spectrum configuration
    /// From yellow_realm.dimensional_architecture.larson_framework stability
    pub orange_intensity: f32,
    /// Red Realm intensity: Solar-scale archetypical mind system
    /// From archetype_activations average
    pub red_intensity: f32,

    // Consciousness data (8 bytes)
    /// Current consciousness level (0.0 to 1.0)
    pub consciousness_level: f32,
    /// Polarization bias (-1.0 STS to 1.0 STO)
    pub polarization: f32,

    // Spectrum data (12 bytes)
    /// Space/time ratio (>1.0 means space/time dominant)
    pub space_time_ratio: f32,
    /// Time/space ratio (<1.0 means time/space dominant)
    pub time_space_ratio: f32,
    /// Veil transparency (0.0 = thick veil, 1.0 = no veil)
    pub veil_transparency: f32,

    // Evolution data (16 bytes)
    /// Evolution progress normalized (0.0 to 1.0)
    pub evolution_progress: f32,
    /// Current density level (1-8)
    pub density_level: u8,
    pub _padding1: [u8; 3], // Align parent_id to 4 bytes
    /// Parent entity ID (0 = no parent)
    pub parent_id: u32,
    /// Environment entity ID (0 = no environment)
    pub environment_id: u32,

    // Archetype summary (8 bytes)
    /// Number of archetypes activated (>0.5 intensity)
    pub archetype_activated: u32,
    /// Maximum archetype intensity (0.0 to 1.0)
    pub archetype_intensity: f32,

    // Entity type and size (8 bytes)
    pub entity_type: u32,
    pub size: f32,

    // Morphology controls derived from 22 archetypes (16 bytes)
    // x: anisotropy, y: depth_bias, z: lobe_count, w: interference_phase
    pub morphology_params: [f32; 4],
}

/// Archetype activations data for rendering
///
/// Contains the 22 archetype activation levels from the archetypical mind system.
/// This data is stored separately in a GPU storage buffer for efficient access.
///
/// From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md Phase 3:
/// "Create 22-archetype visualization with activation intensity display"
///
/// Memory layout: 22 floats (88 bytes)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ArchetypeData {
    /// Activation levels for all 22 archetypes (0.0 to 1.0)
    pub activations: [f32; 22],
}

impl ArchetypeData {
    /// Create ArchetypeData from a SubSubLogos entity
    ///
    /// Extracts archetype activations from the entity's archetypical mind system.
    ///
    /// # Mapping from SubSubLogos to ArchetypeData
    ///
    /// The 22 archetypes correspond to the Major Arcana in the Law of One system:
    /// - 0: The Fool (Matrix of the Mind)
    /// - 1: The Magician (Matrix of the Body)
    /// - 2: The High Priestess (Matrix of the Spirit)
    /// - 3-8: Mind archetypes (6 archetypes)
    /// - 9-14: Body archetypes (6 archetypes)
    /// - 15-20: Spirit archetypes (6 archetypes)
    /// - 21: The World (Archetypical Mind unifier)
    pub fn from_entity(entity: &SubSubLogos) -> Self {
        let mut activations = [0.0f32; 22];

        // Copy archetype activations from entity
        for (i, &activation) in entity.archetype_activations.iter().enumerate() {
            if i < 22 {
                activations[i] = activation as f32;
            }
        }

        Self { activations }
    }

    /// Create test archetype data for debugging
    pub fn test_data(index: usize) -> Self {
        let mut activations = [0.0f32; 22];

        // Create varying activation patterns for testing
        for (i, activation) in activations.iter_mut().enumerate() {
            // Some archetypes highly activated, others less so
            if (i + index).is_multiple_of(3) {
                *activation = 0.8 + ((index % 10) as f32 * 0.02);
            } else if (i + index).is_multiple_of(5) {
                *activation = 0.5 + ((index % 10) as f32 * 0.03);
            } else {
                *activation = 0.1 + ((index % 10) as f32 * 0.01);
            }
        }

        Self { activations }
    }
}
impl EntityInstance {
    /// Create an EntityInstance from a SubSubLogos entity
    ///
    /// This method extracts and maps the complete 7-layer holonic architecture
    /// from the SubSubLogos entity into GPU-friendly visualization data.
    ///
    /// # Mapping from SubSubLogos to EntityInstance
    ///
    /// **Realm Intensities** (7 layers):
    /// - violet_intensity: violet_realm.rhythmic_flow * violet_realm.mystery
    /// - indigo_intensity: indigo_realm.awareness
    /// - blue_intensity: blue_realm.focusing_strength
    /// - green_intensity: green_realm.potential_strength
    /// - yellow_intensity: yellow_realm.attractor_field.strength
    /// - orange_intensity: derived from yellow_realm.dimensional_architecture
    /// - red_intensity: average of archetype_activations
    ///
    /// **Consciousness Data**:
    /// - consciousness_level: direct from consciousness_level field
    /// - polarization: polarization.polarity_bias() (-1.0 to 1.0)
    ///
    /// **Spectrum Data**:
    /// - space_time_ratio: direct from space_time_ratio field
    /// - time_space_ratio: direct from time_space_ratio field
    /// - veil_transparency: direct from veil_transparency field
    ///
    /// **Evolution Data**:
    /// - evolution_progress: evolution_clock normalized to 0.0-1.0
    /// - density_level: current_density converted to u8 (1-8)
    ///
    /// **Archetype Summary**:
    /// - archetype_activated: count of archetype_activations > 0.5
    /// - archetype_intensity: max of archetype_activations
    ///
    /// **Entity Properties**:
    /// - entity_type: converted to u32
    /// - size: derived from density_level and consciousness_level
    /// - position: physical/simulation position when available (deterministic fallback)
    pub fn from_entity(entity: &SubSubLogos, _index: usize) -> Self {
        // Extract realm intensities from the 7-layer architecture
        let violet_intensity =
            (entity.violet_realm.rhythmic_flow * entity.violet_realm.mystery) as f32;
        let indigo_intensity = entity.indigo_realm.awareness as f32;
        let blue_intensity = entity.blue_realm.focusing_strength as f32;

        // Green realm intensity from potential strength
        let green_intensity = entity.green_realm.potential_strength as f32;

        // Yellow realm intensity from attractor field
        let yellow_intensity = entity.yellow_realm.attractor_field.strength as f32;

        // Orange realm intensity from dimensional architecture stability
        let orange_intensity = if entity
            .yellow_realm
            .dimensional_architecture
            .has_dimensions()
        {
            // Calculate average stability of dimensional structures
            let avg_stability = entity
                .yellow_realm
                .dimensional_architecture
                .dimensions
                .iter()
                .map(|d| d.stability)
                .sum::<f64>()
                / entity
                    .yellow_realm
                    .dimensional_architecture
                    .dimensions
                    .len()
                    .max(1) as f64;
            avg_stability as f32
        } else {
            0.0
        };

        // Red realm intensity from archetype activations
        let red_intensity = {
            let sum: f64 = entity.archetype_activations.iter().sum();
            let avg = sum / entity.archetype_activations.len() as f64;
            avg as f32
        };

        let mut archetype_activations = [0.0_f32; 22];
        for (i, activation) in entity.archetype_activations.iter().enumerate() {
            if i < 22 {
                archetype_activations[i] = *activation as f32;
            }
        }

        // Consciousness data
        let consciousness_level = entity.consciousness_level as f32;
        let polarization = entity.polarization.polarity_bias() as f32;

        // Spectrum data
        let space_time_ratio = entity.space_time_ratio as f32;
        let time_space_ratio = entity.time_space_ratio as f32;
        let veil_transparency = entity.veil_transparency as f32;

        // Evolution data
        // Normalize evolution clock to 0.0-1.0 (assuming 100.0 is "complete")
        let evolution_progress = (entity.evolution_clock / 100.0).clamp(0.0, 1.0) as f32;

        // Convert density enum to u8 (1-8)
        let density_level = match entity.current_density {
            crate::evolution_density_octave::density_octave::Density::First(_) => 1,
            crate::evolution_density_octave::density_octave::Density::Second(_) => 2,
            crate::evolution_density_octave::density_octave::Density::Third => 3,
            crate::evolution_density_octave::density_octave::Density::Fourth => 4,
            crate::evolution_density_octave::density_octave::Density::Fifth => 5,
            crate::evolution_density_octave::density_octave::Density::Sixth => 6,
            crate::evolution_density_octave::density_octave::Density::Seventh => 7,
            crate::evolution_density_octave::density_octave::Density::Eighth => 8,
        };

        // Hierarchy data
        let parent_id = entity
            .parent_id
            .as_ref()
            .map(|id| id.as_u64() as u32)
            .unwrap_or(0);
        let environment_id = entity
            .environment_id
            .as_ref()
            .map(|id| id.as_u64() as u32)
            .unwrap_or(0);

        // Archetype summary
        let archetype_activated = entity
            .archetype_activations
            .iter()
            .filter(|&&a| a > 0.5)
            .count() as u32;
        let archetype_intensity = entity
            .archetype_activations
            .iter()
            .cloned()
            .fold(0.0_f64, f64::max) as f32;

        // Entity type
        let entity_type = entity.entity_type as u32;

        // Size based on density level and consciousness level
        // Higher density + higher consciousness = larger visible size
        let base_size = 0.04 + (density_level as f32 * 0.015);
        let size = base_size * (1.0 + consciousness_level * 0.5);

        let morphology_params = Self::morphology_from_activations(
            &archetype_activations,
            consciousness_level,
            space_time_ratio,
            time_space_ratio,
        );

        let [x, y, z] = Self::position_from_entity(entity);

        Self {
            position: [x, y, z],
            _padding0: 0.0,
            violet_intensity,
            indigo_intensity,
            blue_intensity,
            green_intensity,
            yellow_intensity,
            orange_intensity,
            red_intensity,
            consciousness_level,
            polarization,
            space_time_ratio,
            time_space_ratio,
            veil_transparency,
            evolution_progress,
            density_level,
            _padding1: [0, 0, 0],
            parent_id,
            environment_id,
            archetype_activated,
            archetype_intensity,
            entity_type,
            size,
            morphology_params,
        }
    }

    /// Create an EntityInstance from a field-derived RenderableEntity
    pub fn from_renderable_entity(entity: &RenderableEntity) -> Self {
        let consciousness_level = entity.consciousness.clamp(0.0, 1.0) as f32;
        let density_index = entity.density_band.index().min(7) as u8;
        let density_level = density_index + 1;
        let density_factor = density_index as f32 / 7.0;

        // Deterministic realm mapping from density + consciousness
        let violet_intensity =
            (1.0 - density_factor * 0.55 + consciousness_level * 0.15).clamp(0.0, 1.0);
        let indigo_intensity = (0.85 - ((density_factor - 1.0 / 7.0).abs() * 0.9)
            + consciousness_level * 0.12)
            .clamp(0.0, 1.0);
        let blue_intensity = (0.85 - ((density_factor - 2.0 / 7.0).abs() * 1.0)
            + consciousness_level * 0.1)
            .clamp(0.0, 1.0);
        let green_intensity = (0.9 - ((density_factor - 3.0 / 7.0).abs() * 1.1)
            + consciousness_level * 0.1)
            .clamp(0.0, 1.0);
        let yellow_intensity = (0.9 - ((density_factor - 4.0 / 7.0).abs() * 1.1)
            + consciousness_level * 0.08)
            .clamp(0.0, 1.0);
        let orange_intensity = (0.85 - ((density_factor - 5.0 / 7.0).abs() * 1.0)
            + consciousness_level * 0.08)
            .clamp(0.0, 1.0);
        let red_intensity = (0.85 - ((density_factor - 6.0 / 7.0).abs() * 0.9)
            + consciousness_level * 0.1)
            .clamp(0.0, 1.0);

        let space_time_ratio = if entity.spatial_config.is_spiritual {
            0.8
        } else if entity.spatial_config.is_physical {
            1.2
        } else {
            1.0
        };
        let time_space_ratio = 1.0 / space_time_ratio;
        let veil_transparency = (1.0 - entity.veil_distance.clamp(0.0, 1.0)) as f32;
        let evolution_progress = consciousness_level;

        let archetype_activations = Self::synthetic_archetype_activations(
            entity.entity_id,
            density_level,
            consciousness_level,
        );

        let archetype_activated = archetype_activations.iter().filter(|&&a| a > 0.5).count() as u32;
        let archetype_intensity = archetype_activations
            .iter()
            .copied()
            .fold(0.0_f32, f32::max);

        let morphology_params = Self::morphology_from_activations(
            &archetype_activations,
            consciousness_level,
            space_time_ratio,
            time_space_ratio,
        );

        let base_size = 0.06 + (consciousness_level * 0.06);
        let collective_bonus = if entity.in_collective { 0.02 } else { 0.0 };
        let size = (base_size + collective_bonus) * (1.0 + density_factor * 0.2);

        Self {
            position: [
                entity.position[0] as f32,
                entity.position[1] as f32,
                entity.position[2] as f32,
            ],
            _padding0: 0.0,
            violet_intensity,
            indigo_intensity,
            blue_intensity,
            green_intensity,
            yellow_intensity,
            orange_intensity,
            red_intensity,
            consciousness_level,
            polarization: 0.0,
            space_time_ratio,
            time_space_ratio,
            veil_transparency,
            evolution_progress,
            density_level,
            _padding1: [0, 0, 0],
            parent_id: 0,
            environment_id: 0,
            archetype_activated,
            archetype_intensity,
            entity_type: EntityType::Individual as u32,
            size,
            morphology_params,
        }
    }

    /// Create test instance for debugging
    pub fn test_instance(index: usize) -> Self {
        let golden_angle = std::f32::consts::PI * 2.0 / 1.618_034;
        let angle = index as f32 * golden_angle;
        let radius = 0.1 + (index as f32 * 0.03);

        Self {
            position: [radius * angle.cos(), radius * angle.sin(), 0.0],
            _padding0: 0.0,
            // Varying realm intensities for test visualization
            violet_intensity: 0.8 + (index as f32 * 0.01).min(0.2),
            indigo_intensity: 0.7 + ((index % 5) as f32 * 0.05),
            blue_intensity: 0.6 + ((index % 3) as f32 * 0.1),
            green_intensity: 0.5 + ((index % 7) as f32 * 0.07),
            yellow_intensity: 0.4 + ((index % 2) as f32 * 0.3),
            orange_intensity: 0.3 + ((index % 4) as f32 * 0.15),
            red_intensity: 0.2 + ((index % 6) as f32 * 0.12),
            // Alternating polarization for test
            consciousness_level: 0.3 + ((index % 10) as f32 * 0.07),
            polarization: if index.is_multiple_of(2) { 0.6 } else { -0.6 },
            // Spectrum data
            space_time_ratio: 1.0 + ((index % 10) as f32 * 0.5),
            time_space_ratio: 1.0 / (1.0 + ((index % 10) as f32 * 0.5)),
            veil_transparency: (index % 10) as f32 * 0.1,
            // Evolution data
            evolution_progress: (index % 20) as f32 * 0.05,
            density_level: ((index % 8) + 1) as u8,
            _padding1: [0, 0, 0],
            // Hierarchy data for testing
            parent_id: if index > 0 { (index - 1) as u32 } else { 0 },
            environment_id: if index.is_multiple_of(3) { 100 } else { 0 },
            // Archetype summary
            archetype_activated: ((index % 22) + 1) as u32,
            archetype_intensity: 0.5 + ((index % 10) as f32 * 0.05),
            // Entity properties
            entity_type: (index % 5) as u32,
            size: 0.05 + ((index % 10) as f32 * 0.008),
            morphology_params: [
                0.2 + ((index % 7) as f32 * 0.08).min(0.7),
                0.25 + ((index % 5) as f32 * 0.12).min(0.6),
                3.0 + (index % 6) as f32,
                ((index as f32) * 0.173_205_08).fract(),
            ],
        }
    }

    fn morphology_from_activations(
        activations: &[f32; 22],
        consciousness_level: f32,
        space_time_ratio: f32,
        time_space_ratio: f32,
    ) -> [f32; 4] {
        let mind_avg = activations[0..9].iter().copied().sum::<f32>() / 9.0;
        let body_avg = activations[9..15].iter().copied().sum::<f32>() / 6.0;
        let spirit_avg = activations[15..21].iter().copied().sum::<f32>() / 6.0;
        let choice = activations[21].clamp(0.0, 1.0);

        let archetype_mean = activations.iter().copied().sum::<f32>() / 22.0;
        let archetype_variance = activations
            .iter()
            .map(|a| {
                let d = *a - archetype_mean;
                d * d
            })
            .sum::<f32>()
            / 22.0;
        let archetype_interference = archetype_variance.sqrt().clamp(0.0, 1.0);

        let anisotropy =
            ((body_avg - mind_avg).abs() * 0.75 + archetype_interference * 0.35 + choice * 0.20)
                .clamp(0.0, 1.0);

        let spectral_bias = ((time_space_ratio - space_time_ratio)
            / (space_time_ratio + time_space_ratio + 0.001))
            .clamp(-1.0, 1.0);
        let depth_bias =
            (0.45 + spirit_avg * 0.35 + consciousness_level * 0.2 + spectral_bias * 0.15)
                .clamp(0.0, 1.0);

        let lobe_count = (3.0 + choice * 4.0 + archetype_interference * 5.0)
            .round()
            .clamp(3.0, 12.0);

        let phase_seed = activations
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let weight = ((i as f32 + 1.0) * 0.173_205_08).sin();
                a * weight
            })
            .sum::<f32>();
        let interference_phase = phase_seed.rem_euclid(1.0);

        [anisotropy, depth_bias, lobe_count, interference_phase]
    }

    fn synthetic_archetype_activations(
        entity_id: usize,
        density_level: u8,
        consciousness_level: f32,
    ) -> [f32; 22] {
        let mut activations = [0.0_f32; 22];
        let density = density_level as f32 / 8.0;
        let base = entity_id as f32 * 0.037 + density * 1.3 + consciousness_level * 0.9;

        for (i, activation) in activations.iter_mut().enumerate() {
            let t = i as f32;
            let wave_a = (base + t * 0.41).sin();
            let wave_b = (base * 1.7 + t * 0.23).cos();
            *activation = (0.5 + 0.5 * (0.62 * wave_a + 0.38 * wave_b)).clamp(0.0, 1.0);
        }

        activations
    }

    /// Get position from entity for rendering
    ///
    /// Extracts position from the entity's physical manifestation if available,
    /// otherwise generates a deterministic position based on entity ID.
    pub fn position_from_entity(entity: &SubSubLogos) -> [f32; 3] {
        if let Some(physical_entity) = &entity.physical_entity {
            match physical_entity {
                Matter::Particle(particle) => {
                    return [
                        particle.position.x as f32,
                        particle.position.y as f32,
                        particle.position.z as f32,
                    ];
                }
                Matter::Atom(atom) => {
                    return [
                        atom.position.x as f32,
                        atom.position.y as f32,
                        atom.position.z as f32,
                    ];
                }
                Matter::Molecule(molecule) if !molecule.atoms.is_empty() => {
                    let count = molecule.atoms.len() as f32;
                    let x = molecule
                        .atoms
                        .iter()
                        .map(|a| a.position.x as f32)
                        .sum::<f32>()
                        / count;
                    let y = molecule
                        .atoms
                        .iter()
                        .map(|a| a.position.y as f32)
                        .sum::<f32>()
                        / count;
                    let z = molecule
                        .atoms
                        .iter()
                        .map(|a| a.position.z as f32)
                        .sum::<f32>()
                        / count;
                    return [x, y, z];
                }
                _ => {}
            }
        }

        // Generate position from entity UUID using proper hash distribution
        // Note: Converting u64 directly to f32 loses precision (f32 only has 23-bit mantissa)
        // Instead, we use byte-level hashing for unique positions
        let uuid_bytes = entity.entity_id.uuid.as_bytes();
        let mut hash1: u32 = 0;
        let mut hash2: u32 = 0;
        for (i, &byte) in uuid_bytes.iter().enumerate() {
            let shift = (i % 4) * 8;
            if i < 4 {
                hash1 ^= (byte as u32) << shift;
            } else {
                hash2 ^= (byte as u32) << shift;
            }
        }

        // Use two independent hash values for angle and radial offset
        let angle_seed = (hash1 as f32 / u32::MAX as f32) * std::f32::consts::TAU;
        let radial_offset = (hash2 as f32 / u32::MAX as f32) * 0.3;

        // Add entity type and density level for more variation
        let type_offset = (entity.entity_type as u8 as f32) * 0.15;
        let density_offset = match entity.current_density {
            crate::evolution_density_octave::density_octave::Density::First(_) => 0.0,
            crate::evolution_density_octave::density_octave::Density::Second(_) => 0.1,
            crate::evolution_density_octave::density_octave::Density::Third => 0.2,
            crate::evolution_density_octave::density_octave::Density::Fourth => 0.3,
            crate::evolution_density_octave::density_octave::Density::Fifth => 0.4,
            crate::evolution_density_octave::density_octave::Density::Sixth => 0.5,
            crate::evolution_density_octave::density_octave::Density::Seventh => 0.6,
            crate::evolution_density_octave::density_octave::Density::Eighth => 0.7,
        };

        let radial = 0.1
            + radial_offset
            + type_offset
            + (entity.spectrum_position as f32).clamp(0.0, 1.0) * 0.5;
        let z = density_offset - 0.35
            + ((entity.time_space_ratio - entity.space_time_ratio) as f32 * 0.05).clamp(-0.2, 0.2);

        [radial * angle_seed.cos(), radial * angle_seed.sin(), z]
    }

    /// Calculate hierarchical position relative to a focus entity
    ///
    /// When drilling down into entities, children should be positioned within
    /// their parent's local space. This function:
    /// 1. If entity IS the focus, positions it at origin
    /// 2. If entity is a child/descendant of focus, positions it in local space
    /// 3. If entity is unrelated to focus, positions it at cosmic scale
    ///
    /// Parameters:
    /// - entity: The entity to position
    /// - focus_entity_id: The ID of the entity we're currently "inside" (None = universe root)
    /// - all_entities: All entities for hierarchy traversal
    /// - depth: Current hierarchy depth for scale adjustment
    pub fn position_with_hierarchy(
        entity: &SubSubLogos,
        focus_entity_id: Option<&crate::entity_layer7::EntityId>,
        all_entities: &std::collections::HashMap<crate::entity_layer7::EntityId, SubSubLogos>,
        depth: usize,
    ) -> [f32; 3] {
        let entity_id_str = entity.entity_id.uuid.as_str();

        // Case 1: This entity IS the focus - center it
        if let Some(focus_id) = focus_entity_id {
            if entity.entity_id == *focus_id {
                return [0.0, 0.0, 0.0];
            }
        }

        // Case 2: Check if this entity is a direct child of the focus
        if let Some(focus_id) = focus_entity_id {
            let _focus_id_str = focus_id.uuid.as_str();

            // Check parent_id
            if let Some(ref parent_id) = entity.parent_id {
                if parent_id == focus_id {
                    // This is a direct child - position in local space around parent
                    return Self::position_as_child(entity, depth);
                }
            }

            // Check if we're in the composition of the focus
            // (for drilling into biological structures)
            if entity_id_str.contains("quantum-")
                || entity_id_str.contains("atomic-")
                || entity_id_str.contains("molecular-")
                || entity_id_str.contains("cellular-")
                || entity_id_str.contains("organism-")
            {
                // Check if this entity's parent chain leads to focus
                if Self::is_descendant_of_by_id(entity, focus_id, all_entities) {
                    return Self::position_as_child(entity, depth);
                }
            }

            // Check environment relationship
            if let Some(ref env_id) = entity.environment_id {
                if env_id == focus_id {
                    // Entity exists IN this environment
                    return Self::position_in_environment(entity, depth);
                }
            }
        }

        // Case 3: Check if this entity is an ancestor of the focus (we're inside it)
        // These should still be visible but faded/simplified
        if let Some(focus_id) = focus_entity_id {
            if let Some(focus_entity) = all_entities.get(focus_id) {
                // Check if focus is a descendant of this entity
                if Self::is_descendant_of_by_id(focus_entity, &entity.entity_id, all_entities) {
                    // This is an ancestor - show as background context
                    return Self::position_as_ancestor(entity);
                }
            }
        }

        // Case 4: Unrelated entity - position at cosmic scale
        Self::position_from_entity(entity)
    }

    /// Position an entity as a child in local space
    fn position_as_child(entity: &SubSubLogos, depth: usize) -> [f32; 3] {
        let uuid_bytes = entity.entity_id.uuid.as_bytes();
        let mut hash1: u32 = 0;
        let mut hash2: u32 = 0;
        for (i, &byte) in uuid_bytes.iter().enumerate() {
            let shift = (i % 4) * 8;
            if i < 4 {
                hash1 ^= (byte as u32) << shift;
            } else {
                hash2 ^= (byte as u32) << shift;
            }
        }

        // Scale decreases with depth - children are smaller and closer together
        let scale = 1.0 / (1.0 + depth as f32 * 0.5);
        let orbit_radius = 0.3 * scale;

        // Orbital angle from hash
        let angle = (hash1 as f32 / u32::MAX as f32) * std::f32::consts::TAU;

        // Height variation from hash
        let height = ((hash2 as f32 / u32::MAX as f32) - 0.5) * 0.4 * scale;

        // Entity type affects positioning pattern
        let (x, y, z) = match entity.entity_type {
            crate::entity_layer7::layer7::EntityType::GalacticLogos => {
                // Galaxies: spread across universe
                let r = orbit_radius * 2.0;
                (r * angle.cos(), r * angle.sin(), height)
            }
            crate::entity_layer7::layer7::EntityType::SolarLogos => {
                // Solar systems: orbit within galaxy
                let r = orbit_radius * 1.2;
                (r * angle.cos(), r * angle.sin(), height)
            }
            crate::entity_layer7::layer7::EntityType::Environmental => {
                // Planets/stars: orbital pattern
                let r = orbit_radius * 0.8;
                // Add orbital inclination
                let inclination = (hash2 as f32 / u32::MAX as f32) * 0.3;
                (
                    r * angle.cos(),
                    r * angle.sin() * inclination.cos(),
                    height + r * inclination.sin(),
                )
            }
            crate::entity_layer7::layer7::EntityType::Individual => {
                // Individual entities: clustered within environment
                let r = orbit_radius * 0.5;
                (r * angle.cos(), r * angle.sin(), height)
            }
            crate::entity_layer7::layer7::EntityType::Collective => {
                // Collectives: centered
                (
                    orbit_radius * 0.2 * angle.cos(),
                    orbit_radius * 0.2 * angle.sin(),
                    height,
                )
            }
        };

        [x, y, z]
    }

    /// Position an entity within its environment
    fn position_in_environment(entity: &SubSubLogos, depth: usize) -> [f32; 3] {
        // Similar to child positioning but optimized for environment context
        Self::position_as_child(entity, depth)
    }

    /// Position an ancestor entity as background context
    fn position_as_ancestor(entity: &SubSubLogos) -> [f32; 3] {
        // Position far away as background context
        let base_pos = Self::position_from_entity(entity);
        [base_pos[0] * 3.0, base_pos[1] * 3.0, base_pos[2] * 3.0]
    }

    /// Check if an entity is a descendant of another entity
    #[allow(dead_code)]
    fn is_descendant_of(
        entity: &SubSubLogos,
        ancestor_id: &str,
        all_entities: &std::collections::HashMap<crate::entity_layer7::EntityId, SubSubLogos>,
    ) -> bool {
        // Check direct parent
        if let Some(ref parent_id) = entity.parent_id {
            if parent_id.uuid.as_str() == ancestor_id {
                return true;
            }
            // Recursively check parent chain
            if let Some(parent) = all_entities.get(parent_id) {
                return Self::is_descendant_of(parent, ancestor_id, all_entities);
            }
        }

        // Check environment
        if let Some(ref env_id) = entity.environment_id {
            if env_id.uuid.as_str() == ancestor_id {
                return true;
            }
        }

        false
    }

    /// Check if an entity is a descendant of another entity (using EntityId)
    fn is_descendant_of_by_id(
        entity: &SubSubLogos,
        ancestor_id: &crate::entity_layer7::EntityId,
        all_entities: &std::collections::HashMap<crate::entity_layer7::EntityId, SubSubLogos>,
    ) -> bool {
        // Check direct parent
        if let Some(ref parent_id) = entity.parent_id {
            if parent_id == ancestor_id {
                return true;
            }
            // Recursively check parent chain
            if let Some(parent) = all_entities.get(parent_id) {
                return Self::is_descendant_of_by_id(parent, ancestor_id, all_entities);
            }
        }

        // Check environment
        if let Some(ref env_id) = entity.environment_id {
            if env_id == ancestor_id {
                return true;
            }
        }

        false
    }

    /// Position an entity as an ancestor (background context) - kept for compatibility
    #[allow(dead_code)]
    fn position_as_ancestor_compat(_entity: &SubSubLogos) -> [f32; 3] {
        // Ancestors are positioned at the edges, faded
        [10.0, 10.0, 0.0] // Far corner, will be rendered faded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_instance_size() {
        // Ensure proper alignment for GPU
        assert_eq!(std::mem::size_of::<EntityInstance>(), 112);
        assert_eq!(std::mem::align_of::<EntityInstance>(), 4);
    }

    #[test]
    fn test_test_instance() {
        let instance = EntityInstance::test_instance(0);
        // Verify basic fields are set
        assert!(instance.violet_intensity > 0.0);
        assert_eq!(instance.entity_type, 0);
        assert!(instance.size > 0.0);
    }

    #[test]
    fn test_realm_intensities_range() {
        let instance = EntityInstance::test_instance(5);
        // All intensities should be in 0.0-1.0 range
        assert!(instance.violet_intensity >= 0.0 && instance.violet_intensity <= 1.0);
        assert!(instance.indigo_intensity >= 0.0 && instance.indigo_intensity <= 1.0);
        assert!(instance.blue_intensity >= 0.0 && instance.blue_intensity <= 1.0);
        assert!(instance.green_intensity >= 0.0 && instance.green_intensity <= 1.0);
        assert!(instance.yellow_intensity >= 0.0 && instance.yellow_intensity <= 1.0);
        assert!(instance.orange_intensity >= 0.0 && instance.orange_intensity <= 1.0);
        assert!(instance.red_intensity >= 0.0 && instance.red_intensity <= 1.0);
    }

    #[test]
    fn test_polarization_range() {
        let instance = EntityInstance::test_instance(0);
        assert!(instance.polarization >= -1.0 && instance.polarization <= 1.0);
    }

    #[test]
    fn test_density_level_range() {
        for i in 0..20 {
            let instance = EntityInstance::test_instance(i);
            assert!(instance.density_level >= 1 && instance.density_level <= 8);
        }
    }
}
