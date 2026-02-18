//! Dynamical Veil Transformation
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.1:
//! "Implement entity-planet coupling"
//! "Veil transformation at spectrum = 1.0"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The Veil at v=1 is a STRUCTURAL FEATURE, not just a conceptual boundary"
//! "Above veil: entities experience time as navigable dimensions"
//! "Below veil: entities experience space as fixed container"
//!
//! This module implements:
//! - Dynamical veil as qualitative break, not just coordinate transform
//! - Veil transparency based on spectrum position
//! - Experience transformation across the veil

use super::*;

/// Perspective type based on spectrum position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Perspective {
    /// Below veil: 3D space, 1D time - classical physics
    SpaceTime,
    /// At veil: transition zone - mixed experience
    Transitional,
    /// Above veil: 1D space, 3D time - spiritual experience
    TimeSpace,
}

/// Transformed position with veil effects
#[derive(Debug, Clone)]
pub struct TransformedPosition {
    /// The transformed 3D position
    pub position: [f64; 3],
    
    /// Current perspective
    pub perspective: Perspective,
    
    /// How transparent the veil is (0 = opaque, 1 = fully transparent)
    pub veil_transparency: f64,
    
    /// How much transformation has occurred
    pub transformation_factor: f64,
}

/// Time flow type based on perspective
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeFlowType {
    /// Linear time flow (below veil)
    Linear,
    /// Multi-dimensional time (above veil)
    MultiDimensional,
    /// Mixed (at veil)
    Transitional,
}

/// Perception type based on perspective
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerceptionType {
    /// Fragmented perception (below veil)
    Fragmented,
    /// Holistic perception (above veil)
    Holistic,
    /// Mixed (at veil)
    Transitional,
}

/// Veil dynamics configuration
#[derive(Debug, Clone)]
pub struct VeilTransformConfig {
    /// Position on spectrum where veil exists (v = 1.0)
    pub veil_position: f64,
    
    /// Thickness of veil transition region
    pub veil_thickness: f64,
    
    /// Base transparency of veil
    pub base_transparency: f64,
    
    /// Rate of transparency change with spectrum
    pub transparency_rate: f64,
    
    /// Enable experience transformation
    pub enable_experience_transform: bool,
}

impl Default for VeilTransformConfig {
    fn default() -> Self {
        VeilTransformConfig {
            veil_position: 1.0,
            veil_thickness: 0.2,
            base_transparency: 0.5,
            transparency_rate: 0.5,
            enable_experience_transform: true,
        }
    }
}

/// Entity experience parameters
#[derive(Debug, Clone)]
pub struct EntityExperience {
    /// Current perception type
    pub perception: PerceptionType,
    
    /// Time flow type
    pub time_flow: TimeFlowType,
    
    /// Unity access (0 = fully separated, 1 = fully unified)
    pub unity_access: f64,
    
    /// Access to other densities
    pub density_access: [f64; 8],
    
    /// Experience coherence
    pub experience_coherence: f64,
}

impl Default for EntityExperience {
    fn default() -> Self {
        EntityExperience {
            perception: PerceptionType::Fragmented,
            time_flow: TimeFlowType::Linear,
            unity_access: 0.0,
            density_access: [0.0; 8],
            experience_coherence: 0.5,
        }
    }
}

/// Dynamical veil transformation system
/// 
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// > "The Veil is NOT a separator - it's a QUALITATIVE BREAK in the spectrum"
/// > "At v = 1, reality transforms from Many-ness to Oneness"
#[derive(Debug, Clone)]
pub struct VeilTransform {
    config: VeilTransformConfig,
    entity_experiences: HashMap<u64, EntityExperience>,
    statistics: VeilStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct VeilStatistics {
    pub total_entities: usize,
    pub below_veil_count: usize,
    pub at_veil_count: usize,
    pub above_veil_count: usize,
    pub average_unity_access: f64,
    pub average_transformation: f64,
}

impl VeilTransform {
    pub fn new() -> Self {
        VeilTransform {
            config: VeilTransformConfig::default(),
            entity_experiences: HashMap::new(),
            statistics: VeilStatistics::default(),
        }
    }
    
    pub fn with_config(config: VeilTransformConfig) -> Self {
        VeilTransform {
            config,
            entity_experiences: HashMap::new(),
            statistics: VeilStatistics::default(),
        }
    }
    
    /// Apply veil transformation to an entity
    /// 
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "Entities move along the spectrum based on their evolution"
    pub fn apply_to_entity(&mut self, entity_id: u64, spectrum_position: f64) -> TransformedPosition {
        let distance_from_veil = (spectrum_position - self.config.veil_position).abs();
        
        if distance_from_veil > self.config.veil_thickness {
            // Outside veil region
            let perspective = if spectrum_position < self.config.veil_position {
                Perspective::SpaceTime
            } else {
                Perspective::TimeSpace
            };
            
            TransformedPosition {
                position: [0.0, 0.0, 0.0], // Will be set by caller
                perspective,
                veil_transparency: 0.0,
                transformation_factor: 0.0,
            }
        } else {
            // Within veil region - apply transformation
            self.apply_veil_transformation(spectrum_position)
        }
    }
    
    /// Apply transformation within veil region
    fn apply_veil_transformation(&self, spectrum_position: f64) -> TransformedPosition {
        let distance_from_veil = (spectrum_position - self.config.veil_position).abs();
        let factor = 1.0 - (distance_from_veil / self.config.veil_thickness);
        
        let perspective = if spectrum_position < self.config.veil_position {
            Perspective::SpaceTime
        } else if spectrum_position > self.config.veil_position {
            Perspective::TimeSpace
        } else {
            Perspective::Transitional
        };
        
        // Transparency increases as we approach veil center
        let transparency = self.config.base_transparency * factor;
        
        TransformedPosition {
            position: [0.0, 0.0, 0.0],
            perspective,
            veil_transparency: transparency,
            transformation_factor: factor,
        }
    }
    
    /// Apply veil effects to entity experience
    /// 
    /// This is the KEY implementation of veil as QUALITATIVE BREAK
    /// 
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// > "The Veil creates real qualitative differences in entity experience"
    pub fn apply_veil_effects(&mut self, entity_id: u64, spectrum_position: f64) -> EntityExperience {
        let distance_from_veil = (spectrum_position - self.config.veil_position).abs();
        
        let experience = if !self.config.enable_experience_transform {
            EntityExperience::default()
        } else if distance_from_veil > self.config.veil_thickness {
            // Outside veil
            if spectrum_position < self.config.veil_position {
                // BELOW VEIL: Space/Time dominant - Many-ness
                EntityExperience {
                    perception: PerceptionType::Fragmented,
                    time_flow: TimeFlowType::Linear,
                    unity_access: 0.0,
                    density_access: self.compute_density_access(spectrum_position),
                    experience_coherence: 0.5,
                }
            } else {
                // ABOVE VEIL: Time/Space dominant - Oneness
                EntityExperience {
                    perception: PerceptionType::Holistic,
                    time_flow: TimeFlowType::MultiDimensional,
                    unity_access: self.compute_unity_access(spectrum_position),
                    density_access: self.compute_density_access(spectrum_position),
                    experience_coherence: 0.9,
                }
            }
        } else {
            // AT VEIL: Transition zone
            let transition = 1.0 - (distance_from_veil / self.config.veil_thickness);
            EntityExperience {
                perception: PerceptionType::Transitional,
                time_flow: TimeFlowType::Transitional,
                unity_access: transition * 0.5,
                density_access: self.compute_density_access(spectrum_position),
                experience_coherence: 0.5 + transition * 0.4,
            }
        };
        
        // Store for statistics
        self.entity_experiences.insert(entity_id, experience.clone());
        
        experience
    }
    
    /// Compute unity access based on spectrum position
    fn compute_unity_access(&self, spectrum_position: f64) -> f64 {
        if spectrum_position <= self.config.veil_position {
            0.0
        } else {
            let above_veil = spectrum_position - self.config.veil_position;
            (above_veil * self.config.transparency_rate).min(1.0)
        }
    }
    
    /// Compute density access based on spectrum position
    fn compute_density_access(&self, spectrum_position: f64) -> [f64; 8] {
        // Each density corresponds to spectrum range
        let mut access = [0.0; 8];
        
        for i in 0..8 {
            let density_center = i as f64 * 0.125 + 0.0625;
            let dist = (spectrum_position - density_center).abs();
            
            // Access decreases with distance from current spectrum position
            if dist < 0.25 {
                access[i] = 1.0 - dist * 4.0;
            }
        }
        
        access
    }
    
    /// Get veil transparency at a spectrum position
    pub fn get_veil_transparency(&self, spectrum_position: f64) -> f64 {
        let distance_from_veil = (spectrum_position - self.config.veil_position).abs();
        
        if distance_from_veil > self.config.veil_thickness {
            if spectrum_position < self.config.veil_position {
                0.0 // Below veil - opaque
            } else {
                self.config.base_transparency // Above veil - always some transparency
            }
        } else {
            // Within veil - interpolate
            let factor = 1.0 - (distance_from_veil / self.config.veil_thickness);
            self.config.base_transparency * factor + (1.0 - factor) * 0.5
        }
    }
    
    /// Check if spectrum position is within veil
    pub fn is_in_veil(&self, spectrum_position: f64) -> bool {
        let distance_from_veil = (spectrum_position - self.config.veil_position).abs();
        distance_from_veil <= self.config.veil_thickness
    }
    
    /// Update statistics
    pub fn update_statistics(&mut self) {
        self.statistics.total_entities = self.entity_experiences.len();
        
        if self.entity_experiences.is_empty() {
            return;
        }
        
        let mut below_veil = 0;
        let mut at_veil = 0;
        let mut above_veil = 0;
        let mut total_unity = 0.0;
        let mut total_transform = 0.0;
        
        for (entity_id, exp) in &self.entity_experiences {
            let spectrum = self.get_spectrum_for_entity(*entity_id);
            
            if spectrum < self.config.veil_position - self.config.veil_thickness {
                below_veil += 1;
            } else if spectrum > self.config.veil_position + self.config.veil_thickness {
                above_veil += 1;
            } else {
                at_veil += 1;
            }
            
            total_unity += exp.unity_access;
            total_transform += exp.experience_coherence;
        }
        
        self.statistics.below_veil_count = below_veil;
        self.statistics.at_veil_count = at_veil;
        self.statistics.above_veil_count = above_veil;
        self.statistics.average_unity_access = total_unity / self.entity_experiences.len() as f64;
        self.statistics.average_transformation = total_transform / self.entity_experiences.len() as f64;
    }
    
    /// Get spectrum position for entity (placeholder)
    fn get_spectrum_for_entity(&self, entity_id: u64) -> f64 {
        // This would be retrieved from entity state
        entity_id as f64 * 0.01 % 2.0
    }
    
    /// Get experience for entity
    pub fn get_experience(&self, entity_id: u64) -> Option<&EntityExperience> {
        self.entity_experiences.get(&entity_id)
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &VeilStatistics {
        &self.statistics
    }
}

impl Default for VeilTransform {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_veil_transparency() {
        let veil = VeilTransform::new();
        
        // Below veil - opaque
        assert_eq!(veil.get_veil_transparency(0.5), 0.0);
        
        // Above veil - transparent
        let above = veil.get_veil_transparency(1.5);
        assert!(above > 0.0 && above <= 1.0);
        
        // At veil - transition
        let at = veil.get_veil_transparency(1.0);
        assert!(at > 0.0);
    }
    
    #[test]
    fn test_veil_effects() {
        let mut veil = VeilTransform::new();
        
        // Below veil
        let below = veil.apply_veil_effects(1, 0.5);
        assert_eq!(below.perception, PerceptionType::Fragmented);
        assert_eq!(below.time_flow, TimeFlowType::Linear);
        assert_eq!(below.unity_access, 0.0);
        
        // Above veil
        let above = veil.apply_veil_effects(2, 1.5);
        assert_eq!(above.perception, PerceptionType::Holistic);
        assert_eq!(above.time_flow, TimeFlowType::MultiDimensional);
        assert!(above.unity_access > 0.0);
        
        // At veil
        let at = veil.apply_veil_effects(3, 1.0);
        assert_eq!(at.perception, PerceptionType::Transitional);
    }
    
    #[test]
    fn test_density_access() {
        let veil = VeilTransform::new();
        
        // At center of spectrum
        let access = veil.apply_veil_effects(1, 0.5).density_access;
        
        // Should have some access to nearby densities
        assert!(access[0] > 0.0); // First density
    }
}
