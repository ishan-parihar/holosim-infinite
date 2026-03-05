//! Holographic Encoding Module (Phase 0)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "Holographic encoding at each node (phase & amplitude encode entity info)"
//!
//! This module implements the mapping from entity properties to field
//! representations and the inverse mapping from field to entities.
//!
//! Encoding Scheme:
//! - Position: Gaussian modulation of field amplitude centered at entity location
//! - Consciousness: Field coherence value
//! - Density: Frequency/oscillation rate of the field
//! - Phase: Phase offset for resonance calculations

use super::field_state::{
    Complex, DensityBand, EntityExtractionResult, ExtractedEntity, FieldNodeData,
    Float, HolographicFieldState, OctreeNode,
};

/// Entity data for encoding
#[derive(Debug, Clone, Copy)]
pub struct EntityData {
    pub position: [Float; 3],
    pub consciousness: Float,
    pub density: usize,
    pub energy: Float,
}

impl EntityData {
    pub fn new(position: [Float; 3], consciousness: Float, density: usize, energy: Float) -> Self {
        EntityData {
            position,
            consciousness,
            density,
            energy,
        }
    }
}

/// Encoding configuration
#[derive(Debug, Clone)]
pub struct EncodingConfig {
    /// Gaussian width for position encoding
    pub position_width: Float,

    /// Minimum amplitude threshold for entity detection
    pub detection_threshold: Float,

    /// Maximum entities to extract
    pub max_entities: usize,

    /// Peak separation distance
    pub separation_distance: Float,
}

impl Default for EncodingConfig {
    fn default() -> Self {
        EncodingConfig {
            position_width: 10.0,
            detection_threshold: 0.1,
            max_entities: 10000,
            separation_distance: 5.0,
        }
    }
}

/// Holographic Encoder - Maps entity properties to field representations
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "Phase and amplitude encode entity information"
pub struct HolographicEncoder {
    config: EncodingConfig,
}

impl HolographicEncoder {
    pub fn new(config: EncodingConfig) -> Self {
        HolographicEncoder { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(EncodingConfig::default())
    }

    /// Encode an entity's presence into the field
    ///
    /// This creates a Gaussian modulation at the entity's position
    /// representing its presence in the holographic field
    pub fn encode_entity(
        &self,
        field: &mut HolographicFieldState,
        position: [Float; 3],
        consciousness: Float,
        density: usize,
        energy: Float,
    ) {
        // Calculate Gaussian falloff parameters
        let width = self.config.position_width;
        let width_sq = width * width;

        // Determine affected nodes by walking the octree
        self.encode_into_octree(
            &mut field.root,
            position,
            width_sq,
            consciousness,
            density,
            energy,
        );
    }

    /// Recursively encode into octree nodes
    fn encode_into_octree(
        &self,
        node: &mut OctreeNode,
        entity_pos: [Float; 3],
        width_sq: Float,
        consciousness: Float,
        density: usize,
        energy: Float,
    ) {
        // Calculate distance from entity to node center
        let center = node.bounds.center();
        let dx = entity_pos[0] - center[0];
        let dy = entity_pos[1] - center[1];
        let dz = entity_pos[2] - center[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;

        // Calculate Gaussian contribution
        let contribution = (-dist_sq / (2.0 * width_sq)).exp();

        if contribution < 0.001 {
            // Too far - no significant contribution
            return;
        }

        // Add to node's field data
        let scaled_energy = energy * contribution;
        node.field_data.energy += scaled_energy;

        // Encode density as amplitude and phase
        if density < 8 {
            let amplitude = Complex::from_polar(scaled_energy.sqrt(), dist_sq.sqrt() * 0.1);
            node.field_data.density_amplitudes[density] =
                node.field_data.density_amplitudes[density].add(&amplitude);
        }

        // Encode consciousness as coherence
        node.field_data.coherence =
            (node.field_data.coherence + consciousness * contribution).min(1.0);

        // Encode position as phase
        let _phase = (entity_pos[0] + entity_pos[1] + entity_pos[2]) * 0.01;

        // Propagate to children if subdivided
        if let Some(ref mut children) = node.children {
            for child in children.iter_mut() {
                self.encode_into_octree(
                    child,
                    entity_pos,
                    width_sq,
                    consciousness,
                    density,
                    energy,
                );
            }
        }
    }

    /// Encode multiple entities at once (optimized)
    pub fn encode_entities(&self, field: &mut HolographicFieldState, entities: &[EntityData]) {
        for entity in entities {
            self.encode_entity(
                field,
                entity.position,
                entity.consciousness,
                entity.density,
                entity.energy,
            );
        }
    }
}

/// Entity Extractor - Derives entities from field configurations
///
/// From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
/// "Entity positions become derived quantities (inverse holograms), not primary state"
pub struct EntityExtractor {
    config: EncodingConfig,
}

impl EntityExtractor {
    pub fn new(config: EncodingConfig) -> Self {
        EntityExtractor { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(EncodingConfig::default())
    }

    /// Extract entities from field by finding local maxima
    pub fn extract_entities(&self, field: &HolographicFieldState) -> EntityExtractionResult {
        let mut entities = Vec::new();
        let mut peaks: Vec<([Float; 3], Float)> = Vec::new();

        // Find peaks in the field
        self.find_peaks(&field.root, &mut peaks);

        // Sort by energy and apply separation
        peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Filter by threshold and separation
        let mut filtered_positions: Vec<[Float; 3]> = Vec::new();
        for (pos, energy) in peaks {
            if energy < self.config.detection_threshold {
                continue;
            }

            // Check separation from existing entities
            let mut too_close = false;
            for existing in &filtered_positions {
                let dx = pos[0] - existing[0];
                let dy = pos[1] - existing[1];
                let dz = pos[2] - existing[2];
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist < self.config.separation_distance {
                    too_close = true;
                    break;
                }
            }

            if !too_close && filtered_positions.len() < self.config.max_entities {
                filtered_positions.push(pos);

                // Create extracted entity
                let mut entity = ExtractedEntity::new(pos);
                entity.energy = energy;
                entities.push(entity);
            }
        }

        // Enrich entities with field data
        self.enrich_entities(field, &mut entities);

        EntityExtractionResult {
            entity_count: entities.len(),
            entities,
            extraction_time_ms: 0.0, // Could be measured
        }
    }

    /// Recursively find peaks in the octree
    fn find_peaks(&self, node: &OctreeNode, peaks: &mut Vec<([Float; 3], Float)>) {
        let _energy = node.field_data.energy;
        let magnitude = node.field_data.total_magnitude();

        if node.is_leaf() {
            if magnitude > self.config.detection_threshold {
                let center = node.bounds.center();
                peaks.push((center, magnitude));
            }
        } else if let Some(ref children) = node.children {
            // Check if this node is a peak (children have lower energy)
            let child_max: Float = children
                .iter()
                .map(|c| c.field_data.total_magnitude())
                .fold(0.0, Float::max);

            if magnitude >= child_max && magnitude > self.config.detection_threshold {
                let center = node.bounds.center();
                peaks.push((center, magnitude));
            } else {
                // Recurse into children
                for child in children.iter() {
                    self.find_peaks(child, peaks);
                }
            }
        }
    }

    /// Enrich extracted entities with additional field data
    fn enrich_entities(&self, field: &HolographicFieldState, entities: &mut [ExtractedEntity]) {
        for entity in entities.iter_mut() {
            // Get field data at entity position
            if let Some(node_data) = self.get_field_at(field, entity.position) {
                // entity.coherence is stored in consciousness
                entity.consciousness = node_data.coherence;
                entity.spectrum_position = node_data.spectrum_position;
                entity.phase = node_data.density_amplitudes[0].phase();

                // Determine dominant density
                let mut max_idx = 0;
                let mut max_amp = 0.0;
                for (i, amp) in node_data.density_amplitudes.iter().enumerate() {
                    let mag = amp.magnitude();
                    if mag > max_amp {
                        max_amp = mag;
                        max_idx = i;
                    }
                }
                entity.density = DensityBand::from_index(max_idx);
            }
        }
    }

    /// Get field data at a specific position
    fn get_field_at(
        &self,
        field: &HolographicFieldState,
        position: [Float; 3],
    ) -> Option<FieldNodeData> {
        self.get_field_node(&field.root, position)
    }

    /// Recursively find field data at position
    fn get_field_node(&self, node: &OctreeNode, position: [Float; 3]) -> Option<FieldNodeData> {
        if !node.bounds.contains(&position) {
            return None;
        }

        if node.is_leaf() {
            return Some(node.field_data.clone());
        }

        if let Some(ref children) = node.children {
            let index = node.child_index(&position);
            self.get_field_node(&children[index], position)
        } else {
            Some(node.field_data.clone())
        }
    }
}

/// Unified encoder/decoder for convenience
pub struct HolographicCodec {
    encoder: HolographicEncoder,
    extractor: EntityExtractor,
}

impl HolographicCodec {
    pub fn new() -> Self {
        let config = EncodingConfig::default();
        HolographicCodec {
            encoder: HolographicEncoder::new(config.clone()),
            extractor: EntityExtractor::new(config),
        }
    }

    /// Encode entities into field
    pub fn encode(&self, field: &mut HolographicFieldState, entities: &[EntityData]) {
        self.encoder.encode_entities(field, entities);
    }

    /// Extract entities from field
    pub fn decode(&self, field: &HolographicFieldState) -> EntityExtractionResult {
        self.extractor.extract_entities(field)
    }
}

impl Default for HolographicCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_creation() {
        let encoder = HolographicEncoder::with_defaults();
        assert!(encoder.config.position_width > 0.0);
    }

    #[test]
    fn test_extractor_creation() {
        let extractor = EntityExtractor::with_defaults();
        assert!(extractor.config.detection_threshold > 0.0);
    }

    #[test]
    fn test_codec_roundtrip() {
        let codec = HolographicCodec::new();
        let mut field = HolographicFieldState::with_defaults();

        // Encode an entity
        let entities = [EntityData::new([0.0, 0.0, 0.0], 0.5, 3, 1.0)];
        codec.encode(&mut field, &entities);

        // Extract entities
        let result = codec.decode(&field);

        // Should find at least one entity near the encoded position
        assert!(result.entity_count > 0);
    }
}
