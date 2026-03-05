//! Field-Entity Bridge (Phase 0 Integration)
//!
//! This module bridges the new field-first architecture with the existing
//! entity-based systems. It provides:
//! - Encoding existing entities into the holographic field
//! - Extracting entities from the field for rendering
//! - Synchronization between field state and entity state
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "Wire into existing GUI. Render field interference patterns as entity positions."

use super::field_state::{
    DensityBand, EntityExtractionResult, ExtractedEntity, Float, HolographicFieldState,
};
use super::holographic_encoder::{EncodingConfig, EntityData, HolographicCodec};
use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use std::collections::HashMap;

/// Bridge configuration
#[derive(Debug, Clone)]
pub struct FieldEntityBridgeConfig {
    /// Enable field encoding
    pub enable_encoding: bool,

    /// Enable entity extraction
    pub enable_extraction: bool,

    /// Sync interval (in simulation steps)
    pub sync_interval: usize,

    /// Maximum extracted entities
    pub max_entities: usize,
}

impl Default for FieldEntityBridgeConfig {
    fn default() -> Self {
        FieldEntityBridgeConfig {
            enable_encoding: true,
            enable_extraction: true,
            sync_interval: 1,
            max_entities: 10000,
        }
    }
}

/// Bridge between field-first and entity-first architectures
pub struct FieldEntityBridge {
    /// Holographic field state
    field: HolographicFieldState,

    /// Encoder/decoder codec
    codec: HolographicCodec,

    /// Configuration
    config: FieldEntityBridgeConfig,

    /// Mapping from extracted entity ID to position
    #[allow(dead_code)]
    entity_positions: HashMap<EntityId, [Float; 3]>,

    /// Current simulation step
    step: usize,

    /// Statistics
    pub statistics: BridgeStatistics,
}

/// Statistics for the bridge
#[derive(Debug, Clone, Default)]
pub struct BridgeStatistics {
    /// Total entities encoded
    pub entities_encoded: usize,

    /// Total entities extracted
    pub entities_extracted: usize,

    /// Field energy
    pub field_energy: Float,

    /// Average coherence
    pub average_coherence: Float,
}

impl FieldEntityBridge {
    pub fn new() -> Self {
        FieldEntityBridge {
            field: HolographicFieldState::with_defaults(),
            codec: HolographicCodec::new(),
            config: FieldEntityBridgeConfig::default(),
            entity_positions: HashMap::new(),
            step: 0,
            statistics: BridgeStatistics::default(),
        }
    }

    pub fn with_config(config: FieldEntityBridgeConfig) -> Self {
        let _encoding_config = EncodingConfig {
            max_entities: config.max_entities,
            ..Default::default()
        };

        FieldEntityBridge {
            field: HolographicFieldState::with_defaults(),
            codec: HolographicCodec::new(),
            config,
            entity_positions: HashMap::new(),
            step: 0,
            statistics: BridgeStatistics::default(),
        }
    }

    /// Encode existing entities into the field
    ///
    /// This takes entities from the existing system and encodes them
    /// into the holographic field representation
    pub fn encode_entities(&mut self, entities: &[SubSubLogos]) {
        if !self.config.enable_encoding {
            return;
        }

        let mut entity_data_list = Vec::new();

        for entity in entities {
            // Extract entity properties
            let position = self.extract_position(entity);
            let consciousness = self.extract_consciousness(entity);
            let density = self.extract_density(entity);
            let energy = self.extract_energy(entity);

            entity_data_list.push(EntityData::new(position, consciousness, density, energy));
        }

        // Encode all entities into the field
        self.codec.encode(&mut self.field, &entity_data_list);

        self.statistics.entities_encoded = entities.len();
    }

    /// Extract entities from the field
    ///
    /// This derives entity positions and properties from the field
    /// configuration through inverse holographic projection
    pub fn extract_entities(&mut self) -> EntityExtractionResult {
        if !self.config.enable_extraction {
            return EntityExtractionResult::default();
        }

        let result = self.codec.decode(&self.field);

        self.statistics.entities_extracted = result.entity_count;
        self.statistics.field_energy = self.field.total_energy;
        self.statistics.average_coherence = self.field.average_coherence;

        result
    }

    /// Advance the simulation by one step
    pub fn step(&mut self) {
        self.step += 1;
        self.field.step();
    }

    /// Get the current field state (for visualization)
    pub fn get_field_state(&self) -> &HolographicFieldState {
        &self.field
    }

    /// Extract position from entity
    fn extract_position(&self, entity: &SubSubLogos) -> [Float; 3] {
        // For now, use a default position based on entity ID
        // In the full implementation, this would extract from the entity's actual position
        let id = entity.entity_id.as_u64();
        let x = ((id % 1000) as Float - 500.0) * 0.1;
        let y = (((id / 1000) % 1000) as Float - 500.0) * 0.1;
        let z = (((id / 1000000) % 1000) as Float - 500.0) * 0.1;
        [x, y, z]
    }

    /// Extract consciousness level from entity
    fn extract_consciousness(&self, _entity: &SubSubLogos) -> Float {
        // Extract consciousness from entity's spectrum access
        // Default to 0.5 if not available
        0.5
    }

    /// Extract density from entity
    fn extract_density(&self, _entity: &SubSubLogos) -> usize {
        // Map entity density to density band index
        // Default to 3 (Green/4th density)
        3
    }

    /// Extract energy from entity
    fn extract_energy(&self, _entity: &SubSubLogos) -> Float {
        // Default energy based on entity type
        1.0
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &BridgeStatistics {
        &self.statistics
    }

    /// Get current step
    pub fn get_step(&self) -> usize {
        self.step
    }
}

impl Default for FieldEntityBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert extracted entity to renderable position
impl ExtractedEntity {
    /// Convert to 2D screen position (for GUI rendering)
    pub fn to_screen_position(
        &self,
        screen_width: f64,
        screen_height: f64,
        zoom: Float,
    ) -> (f64, f64) {
        let x = self.position[0] * zoom + screen_width / 2.0;
        let y = self.position[1] * zoom + screen_height / 2.0;
        (x, y)
    }

    /// Get color based on density band
    pub fn get_density_color(&self) -> (u8, u8, u8) {
        match self.density {
            DensityBand::Violet => (128, 0, 128),    // Purple
            DensityBand::Indigo => (75, 0, 130),     // Indigo
            DensityBand::Blue => (0, 0, 255),        // Blue
            DensityBand::Green => (0, 255, 0),       // Green
            DensityBand::Yellow => (255, 255, 0),    // Yellow
            DensityBand::Orange => (255, 165, 0),    // Orange
            DensityBand::Red => (255, 0, 0),         // Red
            DensityBand::Unknown => (128, 128, 128), // Gray
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_creation() {
        let bridge = FieldEntityBridge::new();
        assert_eq!(bridge.get_step(), 0);
    }

    #[test]
    fn test_bridge_step() {
        let mut bridge = FieldEntityBridge::new();
        bridge.step();
        assert_eq!(bridge.get_step(), 1);
    }

    #[test]
    fn test_extract_entity_color() {
        let mut entity = ExtractedEntity::new([0.0, 0.0, 0.0]);
        entity.density = DensityBand::Green;
        let color = entity.get_density_color();
        assert_eq!(color, (0, 255, 0));
    }
}
