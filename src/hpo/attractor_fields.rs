//! Attractor Fields - Spiritual Gravity (Phase E)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Each layer creates attractor-fields pulling toward 
//! the next stage - 'spiritual gravity' that shapes entity evolution."
//!
//! The complete field equation:
//! ∂ψ/∂t = FreeWill_term + Love_term + Light_term + Attractor_term + Veil_term
//!
//! This module implements the Attractor_term - spiritual gravity that pulls entities 
//! and field configurations toward higher consciousness states.

use super::field_state::{Complex, Float, OctreeNode};
use super::cosmic_sequence::CosmologicalLayer;
use std::collections::HashMap;

/// Type of attractor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttractorType {
    /// Cosmic layer attractor (from cosmic_sequence)
    CosmicLayer,
    /// Entity-level attractor (individual consciousness)
    Entity,
    /// Collective attractor (group consciousness)
    Collective,
    /// Archetypal attractor (pattern-based)
    Archetypal,
}

impl AttractorType {
    pub fn name(&self) -> &'static str {
        match self {
            AttractorType::CosmicLayer => "Cosmic Layer",
            AttractorType::Entity => "Entity",
            AttractorType::Collective => "Collective",
            AttractorType::Archetypal => "Archetypal",
        }
    }
}

/// Configuration for attractor fields
#[derive(Debug, Clone)]
pub struct AttractorFieldConfig {
    /// Base strength of attractors
    pub base_strength: Float,
    
    /// Range of influence
    pub base_range: Float,
    
    /// Falloff type (inverse_square, exponential, linear)
    pub falloff_type: FalloffType,
    
    /// Enable entity-level attractors
    pub enable_entity_attractors: bool,
    
    /// Enable collective attractors
    pub enable_collective_attractors: bool,
    
    /// Maximum attractors to track
    pub max_attractors: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum FalloffType {
    InverseSquare,
    Exponential,
    Linear,
}

impl Default for AttractorFieldConfig {
    fn default() -> Self {
        AttractorFieldConfig {
            base_strength: 0.5,
            base_range: 200.0,
            falloff_type: FalloffType::InverseSquare,
            enable_entity_attractors: true,
            enable_collective_attractors: true,
            max_attractors: 1000,
        }
    }
}

/// Entity-level attractor
/// Each entity creates its own attractor field pulling toward its consciousness level
#[derive(Debug, Clone)]
pub struct EntityAttractor {
    /// Entity ID
    pub entity_id: usize,
    
    /// Position [x, y, z]
    pub position: [Float; 3],
    
    /// Consciousness level (0.0 - 1.0)
    pub consciousness: Float,
    
    /// Target layer this entity is attracting toward
    pub target_layer: CosmologicalLayer,
    
    /// Current attractor strength
    pub strength: Float,
    
    /// Range of influence
    pub range: Float,
}

impl EntityAttractor {
    pub fn new(entity_id: usize, position: [Float; 3], consciousness: Float) -> Self {
        // Target layer based on consciousness
        let target_idx = (consciousness * 7.0) as usize;
        let target_layer = CosmologicalLayer::from_index(target_idx.min(7)).unwrap_or(CosmologicalLayer::Layer7);
        
        EntityAttractor {
            entity_id,
            position,
            consciousness,
            target_layer,
            strength: 0.3,  // Base strength
            range: 50.0,   // Entity range
        }
    }
    
    /// Update position
    pub fn update_position(&mut self, new_position: [Float; 3]) {
        self.position = new_position;
    }
    
    /// Update consciousness and target
    pub fn update_consciousness(&mut self, consciousness: Float) {
        self.consciousness = consciousness;
        let target_idx = (consciousness * 7.0) as usize;
        self.target_layer = CosmologicalLayer::from_index(target_idx.min(7)).unwrap_or(CosmologicalLayer::Layer7);
    }
    
    /// Calculate attraction force at a point
    pub fn calculate_force(&self, point: &[Float; 3]) -> Float {
        // Distance calculation
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        let dz = point[2] - self.position[2];
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();
        
        if dist > self.range || dist < 0.001 {
            return 0.0;
        }
        
        // Calculate falloff
        let falloff = match FalloffType::InverseSquare {
            FalloffType::InverseSquare => 1.0 - (dist / self.range),
            FalloffType::Exponential => (-dist * 3.0 / self.range).exp(),
            FalloffType::Linear => 1.0 - (dist / self.range),
        };
        
        // Spiritual gravity: stronger attractors pull more
        self.strength * falloff * self.consciousness
    }
}

/// Complete attractor field system
pub struct AttractorFields {
    /// Configuration
    config: AttractorFieldConfig,
    
    /// Entity attractors
    entity_attractors: HashMap<usize, EntityAttractor>,
    
    /// Collective attractors (formed by groups)
    collective_attractors: HashMap<usize, CollectiveAttractor>,
    
    /// Archetypal attractors (layer-based)
    archetypal_attractors: Vec<ArchetypalAttractor>,
    
    /// Field statistics
    pub statistics: AttractorFieldStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct AttractorFieldStatistics {
    pub entity_attractor_count: usize,
    pub collective_attractor_count: usize,
    pub archetypal_attractor_count: usize,
    pub total_force_applied: Float,
    pub average_attractor_strength: Float,
    pub entities_affected: usize,
}

/// Collective attractor - formed by group consciousness
#[derive(Debug, Clone)]
pub struct CollectiveAttractor {
    pub collective_id: usize,
    pub center: [Float; 3],
    pub coherence: Float,
    pub member_count: usize,
    pub strength: Float,
    pub range: Float,
}

impl CollectiveAttractor {
    pub fn new(collective_id: usize, center: [Float; 3], coherence: Float, member_count: usize) -> Self {
        let strength = coherence * 0.5 * (member_count as Float).sqrt() * 0.1;
        let range = 100.0 + member_count as Float * 10.0;
        
        CollectiveAttractor {
            collective_id,
            center,
            coherence,
            member_count,
            strength,
            range,
        }
    }
    
    pub fn calculate_force(&self, point: &[Float; 3]) -> Float {
        let dx = point[0] - self.center[0];
        let dy = point[1] - self.center[1];
        let dz = point[2] - self.center[2];
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();
        
        if dist > self.range || dist < 0.001 {
            return 0.0;
        }
        
        let falloff = 1.0 - (dist / self.range);
        self.strength * falloff
    }
}

/// Archetypal attractor - based on cosmic layers
#[derive(Debug, Clone)]
pub struct ArchetypalAttractor {
    pub layer: CosmologicalLayer,
    pub position: [Float; 3],
    pub strength: Float,
    pub activation: Float,
    pub range: Float,
}

impl ArchetypalAttractor {
    pub fn new(layer: CosmologicalLayer, position: [Float; 3]) -> Self {
        ArchetypalAttractor {
            layer,
            position,
            strength: layer.default_attractor_strength(),
            activation: 0.0,
            range: 150.0,
        }
    }
    
    pub fn calculate_force(&self, point: &[Float; 3]) -> Float {
        if self.activation <= 0.0 {
            return 0.0;
        }
        
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        let dz = point[2] - self.position[2];
        let dist = (dx*dx + dy*dy + dz*dz).sqrt();
        
        if dist > self.range || dist < 0.001 {
            return 0.0;
        }
        
        let falloff = 1.0 - (dist / self.range);
        self.strength * falloff * self.activation
    }
}

impl AttractorFields {
    pub fn new(config: AttractorFieldConfig) -> Self {
        AttractorFields {
            config: config.clone(),
            entity_attractors: HashMap::new(),
            collective_attractors: HashMap::new(),
            archetypal_attractors: Vec::new(),
            statistics: AttractorFieldStatistics::default(),
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(AttractorFieldConfig::default())
    }
    
    /// Initialize archetypal attractors for all cosmic layers
    pub fn initialize_archetypal(&mut self) {
        self.archetypal_attractors.clear();
        
        // Create attractor for each cosmological layer
        for i in 0..8 {
            if let Some(layer) = CosmologicalLayer::from_index(i) {
                // Position layers in a line extending from origin
                let position = [
                    i as Float * 50.0,
                    (i as Float * 17.0).sin() * 30.0,
                    (i as Float * 23.0).cos() * 20.0,
                ];
                self.archetypal_attractors.push(ArchetypalAttractor::new(layer, position));
            }
        }
        
        self.statistics.archetypal_attractor_count = self.archetypal_attractors.len();
    }
    
    /// Initialize entity attractors
    pub fn initialize_entities(&mut self, entity_count: usize) {
        self.entity_attractors.clear();
        
        if !self.config.enable_entity_attractors {
            return;
        }
        
        for i in 0..entity_count {
            // Position based on entity ID (pseudo-random but deterministic)
            let position = [
                (i as Float * 17.0).sin() * 100.0,
                (i as Float * 23.0).cos() * 100.0,
                (i as Float * 31.0).sin() * 50.0,
            ];
            
            // Vary consciousness
            let consciousness = 0.3 + (i as Float % 10.0) / 15.0;
            
            self.entity_attractors.insert(i, EntityAttractor::new(i, position, consciousness));
        }
        
        self.statistics.entity_attractor_count = self.entity_attractors.len();
    }
    
    /// Add a collective attractor
    pub fn add_collective(&mut self, collective_id: usize, center: [Float; 3], coherence: Float, member_count: usize) {
        if !self.config.enable_collective_attractors {
            return;
        }
        
        if self.collective_attractors.len() >= self.config.max_attractors {
            return;
        }
        
        let collective = CollectiveAttractor::new(collective_id, center, coherence, member_count);
        self.collective_attractors.insert(collective_id, collective);
        self.statistics.collective_attractor_count = self.collective_attractors.len();
    }
    
    /// Remove a collective attractor
    pub fn remove_collective(&mut self, collective_id: usize) {
        self.collective_attractors.remove(&collective_id);
        self.statistics.collective_attractor_count = self.collective_attractors.len();
    }
    
    /// Update all attractors based on entity states
    pub fn update_entity_attractors(&mut self, entity_id: usize, position: [Float; 3], consciousness: Float) {
        if let Some(attractor) = self.entity_attractors.get_mut(&entity_id) {
            attractor.update_position(position);
            attractor.update_consciousness(consciousness);
        }
    }
    
    /// Update archetypal activations based on coherence
    pub fn update_archetypal(&mut self, layer_activations: [Float; 8]) {
        for (i, attractor) in self.archetypal_attractors.iter_mut().enumerate() {
            if i < layer_activations.len() {
                attractor.activation = layer_activations[i];
            }
        }
    }
    
    /// Calculate total attractor force at a point
    pub fn calculate_total_force(&self, point: &[Float; 3]) -> Float {
        let mut total_force = 0.0;
        
        // Entity attractors
        for attractor in self.entity_attractors.values() {
            total_force += attractor.calculate_force(point);
        }
        
        // Collective attractors
        for attractor in self.collective_attractors.values() {
            total_force += attractor.calculate_force(point);
        }
        
        // Archetypal attractors
        for attractor in &self.archetypal_attractors {
            total_force += attractor.calculate_force(point);
        }
        
        total_force
    }
    
    /// Apply attractor fields to a field node
    pub fn apply_to_node(&mut self, node: &mut OctreeNode) {
        let node_center = node.bounds.center();
        
        let force = self.calculate_total_force(&node_center);
        
        if force > 0.001 {
            // Apply force to coherence (spiritual gravity)
            let coherence_change = force * 0.05;
            node.field_data.coherence = (node.field_data.coherence + coherence_change).clamp(0.0, 1.0);
            
            // Apply to energy
            node.field_data.energy *= 1.0 + force * 0.01;
        }
        
        self.statistics.total_force_applied += force;
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> AttractorFieldStatistics {
        let mut stats = self.statistics.clone();
        
        let total_attractors = stats.entity_attractor_count + stats.collective_attractor_count + stats.archetypal_attractor_count;
        if total_attractors > 0 {
            stats.average_attractor_strength = stats.total_force_applied / total_attractors as Float;
        }
        
        stats
    }
    
    /// Get attractor visualization data
    pub fn get_visualization_data(&self) -> Vec<AttractorVisualizationData> {
        let mut data = Vec::new();
        
        // Entity attractors
        for attractor in self.entity_attractors.values() {
            data.push(AttractorVisualizationData {
                attractor_type: AttractorType::Entity,
                position: attractor.position,
                strength: attractor.strength * attractor.consciousness,
                range: attractor.range,
                layer: Some(attractor.target_layer),
            });
        }
        
        // Collective attractors
        for attractor in self.collective_attractors.values() {
            data.push(AttractorVisualizationData {
                attractor_type: AttractorType::Collective,
                position: attractor.center,
                strength: attractor.strength,
                range: attractor.range,
                layer: None,
            });
        }
        
        // Archetypal attractors
        for attractor in &self.archetypal_attractors {
            data.push(AttractorVisualizationData {
                attractor_type: AttractorType::Archetypal,
                position: attractor.position,
                strength: attractor.strength * attractor.activation,
                range: attractor.range,
                layer: Some(attractor.layer),
            });
        }
        
        data
    }
}

/// Visualization data for attractors
#[derive(Debug, Clone)]
pub struct AttractorVisualizationData {
    pub attractor_type: AttractorType,
    pub position: [Float; 3],
    pub strength: Float,
    pub range: Float,
    pub layer: Option<CosmologicalLayer>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_attractor() {
        let attractor = EntityAttractor::new(0, [0.0, 0.0, 0.0], 0.5);
        assert_eq!(attractor.entity_id, 0);
        assert_eq!(attractor.consciousness, 0.5);
    }
    
    #[test]
    fn test_attractor_force() {
        let attractor = EntityAttractor::new(0, [0.0, 0.0, 0.0], 0.5);
        let force = attractor.calculate_force(&[10.0, 0.0, 0.0]);
        assert!(force > 0.0);
    }
    
    #[test]
    fn test_attractor_fields() {
        let mut fields = AttractorFields::with_defaults();
        fields.initialize_archetypal();
        fields.initialize_entities(10);
        
        let stats = fields.get_statistics();
        assert!(stats.entity_attractor_count > 0);
        assert!(stats.archetypal_attractor_count > 0);
    }
}
