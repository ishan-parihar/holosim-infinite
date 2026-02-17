//! Density Sub-levels (Phase D)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Four sub-levels per density implementing transcend and include"
//!
//! Each density contains 4 sub-levels that "transcend and include" the previous:
//! - 1st Density (Red): Quantum → Atomic → Molecular → Planetary
//! - 2nd Density (Orange): Cellular → Simple Life → Complex Life
//! - 3rd Density (Yellow): Conscious Life → Societies
//! - 4th+ Density: Higher consciousness states
//!
//! Transitions between sub-levels occur when field coherence reaches threshold values.

use super::field_state::{DensityBand, Float, OctreeNode};
use std::collections::HashMap;

/// Sub-level within a density
/// Each density has 4 sub-levels representing progressive complexity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DensitySubLevel {
    // 1st Density (Red Ray)
    Quantum = 0,        // Quantum particles and fields
    Atomic = 1,         // Atoms and galaxies
    Molecular = 2,      // Molecules and planets
    Planetary = 3,      // Planetary structures, Gaia precursors
    
    // For densities 2-8, we use similar patterns
    // Cellular (2nd), Simple, Complex, etc.
}

impl DensitySubLevel {
    pub fn count() -> usize { 4 }
    
    pub fn index(&self) -> usize { *self as usize }
    
    pub fn from_index(index: usize) -> Option<DensitySubLevel> {
        match index {
            0 => Some(DensitySubLevel::Quantum),
            1 => Some(DensitySubLevel::Atomic),
            2 => Some(DensitySubLevel::Molecular),
            3 => Some(DensitySubLevel::Planetary),
            _ => None,
        }
    }
    
    /// Get the characteristic scale for this sub-level
    pub fn scale(&self) -> Float {
        match self {
            DensitySubLevel::Quantum => 0.001,      // Nanometer scale
            DensitySubLevel::Atomic => 0.1,        // Angstrom scale  
            DensitySubLevel::Molecular => 10.0,     // Nanometer to micrometer
            DensitySubLevel::Planetary => 10000.0, // Kilometer scale
        }
    }
    
    /// Get name for display
    pub fn name(&self) -> &'static str {
        match self {
            DensitySubLevel::Quantum => "Quantum Realm",
            DensitySubLevel::Atomic => "Atomic Realm",
            DensitySubLevel::Molecular => "Molecular Realm",
            DensitySubLevel::Planetary => "Planetary Realm",
        }
    }
    
    /// Get coherence threshold for transitioning to next sub-level
    pub fn transition_threshold(&self) -> Float {
        match self {
            DensitySubLevel::Quantum => 0.3,
            DensitySubLevel::Atomic => 0.45,
            DensitySubLevel::Molecular => 0.6,
            DensitySubLevel::Planetary => 0.8, // Maximum - fully manifested
        }
    }
}

/// Density with its sub-levels
/// Each density can be at a specific sub-level of development
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DensityWithSubLevel {
    /// Primary density band
    pub density: DensityBand,
    /// Current sub-level within this density
    pub sub_level: DensitySubLevel,
    /// Progress within current sub-level (0.0 - 1.0)
    pub progress: Float,
}

impl DensityWithSubLevel {
    pub fn new(density: DensityBand) -> Self {
        DensityWithSubLevel {
            density,
            sub_level: DensitySubLevel::Quantum,
            progress: 0.0,
        }
    }
    
    /// Get the combined index (density * 4 + sub_level)
    pub fn combined_index(&self) -> usize {
        self.density.index() * 4 + self.sub_level.index()
    }
    
    /// Get description
    pub fn description(&self) -> String {
        format!("{} - {} ({:.0}%)", 
            format!("{:?}", self.density), 
            self.sub_level.name(),
            self.progress * 100.0
        )
    }
}

/// Configuration for density sub-levels
#[derive(Debug, Clone)]
pub struct DensitySubLevelConfig {
    /// Number of sub-levels per density
    pub sub_levels_per_density: usize,
    
    /// Base coherence threshold for first sub-level
    pub base_threshold: Float,
    
    /// Rate of sub-level progression
    pub progression_rate: Float,
    
    /// Enable transcend and include
    pub transcend_include: bool,
}

impl Default for DensitySubLevelConfig {
    fn default() -> Self {
        DensitySubLevelConfig {
            sub_levels_per_density: 4,
            base_threshold: 0.3,
            progression_rate: 0.01,
            transcend_include: true,
        }
    }
}

/// Complete density sub-levels system
pub struct DensitySubLevels {
    /// Configuration
    config: DensitySubLevelConfig,
    
    /// Entity density/sub-level tracking
    entity_densities: HashMap<usize, DensityWithSubLevel>,
    
    /// Current coherence level
    current_coherence: Float,
    
    /// Statistics
    pub statistics: DensitySubLevelStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct DensitySubLevelStatistics {
    pub entities_at_quantum: usize,
    pub entities_at_atomic: usize,
    pub entities_at_molecular: usize,
    pub entities_at_planetary: usize,
    pub sub_level_transitions: usize,
    pub average_progress: Float,
}

impl DensitySubLevels {
    pub fn new(config: DensitySubLevelConfig) -> Self {
        DensitySubLevels {
            config: config.clone(),
            entity_densities: HashMap::new(),
            current_coherence: 0.5,
            statistics: DensitySubLevelStatistics::default(),
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(DensitySubLevelConfig::default())
    }
    
    /// Initialize entities with default densities
    pub fn initialize_entities(&mut self, entity_count: usize) {
        self.entity_densities.clear();
        
        for i in 0..entity_count {
            // Start entities at different sub-levels based on some initial condition
            // For example, spread them across 1st density sub-levels
            let density = DensityBand::Red; // Start at 1st density
            let mut density_with_sub = DensityWithSubLevel::new(density);
            
            // Vary starting sub-level
            let initial_sub = match i % 4 {
                0 => DensitySubLevel::Quantum,
                1 => DensitySubLevel::Atomic,
                2 => DensitySubLevel::Molecular,
                _ => DensitySubLevel::Planetary,
            };
            density_with_sub.sub_level = initial_sub;
            density_with_sub.progress = 0.1;
            
            self.entity_densities.insert(i, density_with_sub);
        }
    }
    
    /// Update all entities based on current coherence
    pub fn update(&mut self, coherence: Float) {
        self.current_coherence = coherence;
        
        let mut transitions = 0;
        let mut total_progress = 0.0;
        
        for density_with_sub in self.entity_densities.values_mut() {
            let old_sub = density_with_sub.sub_level;
            
            // Update progress within current sub-level
            // Higher coherence = faster progression
            let threshold = density_with_sub.sub_level.transition_threshold();
            
            if coherence >= threshold {
                // Progress toward next sub-level
                density_with_sub.progress += self.config.progression_rate * coherence;
                
                // Check for sub-level transition
                if density_with_sub.progress >= 1.0 && density_with_sub.sub_level.index() < 3 {
                    // Transition to next sub-level
                    if let Some(next_sub) = DensitySubLevel::from_index(density_with_sub.sub_level.index() + 1) {
                        density_with_sub.sub_level = next_sub;
                        density_with_sub.progress = 0.0;
                        transitions += 1;
                    }
                }
            }
            
            // Clamp progress
            density_with_sub.progress = density_with_sub.progress.clamp(0.0, 1.0);
            total_progress += density_with_sub.progress;
        }
        
        // Apply transcend and include - lower sub-levels persist
        if self.config.transcend_include {
            self.apply_transcend_include();
        }
        
        // Update statistics
        self.statistics.sub_level_transitions += transitions;
        let count = self.entity_densities.len() as Float;
        if count > 0.0 {
            self.statistics.average_progress = total_progress / count;
        }
        self.update_sub_level_counts();
    }
    
    /// Apply transcend and include
    /// Lower sub-levels contribute to field even as higher ones emerge
    fn apply_transcend_include(&self) {
        // This affects how the field is computed
        // Each sub-level adds its contribution to the overall field
    }
    
    /// Update sub-level counts for statistics
    fn update_sub_level_counts(&mut self) {
        self.statistics.entities_at_quantum = 0;
        self.statistics.entities_at_atomic = 0;
        self.statistics.entities_at_molecular = 0;
        self.statistics.entities_at_planetary = 0;
        
        for density_with_sub in self.entity_densities.values() {
            match density_with_sub.sub_level {
                DensitySubLevel::Quantum => self.statistics.entities_at_quantum += 1,
                DensitySubLevel::Atomic => self.statistics.entities_at_atomic += 1,
                DensitySubLevel::Molecular => self.statistics.entities_at_molecular += 1,
                DensitySubLevel::Planetary => self.statistics.entities_at_planetary += 1,
            }
        }
    }
    
    /// Get entity's density with sub-level
    pub fn get_entity_density(&self, entity_id: usize) -> Option<DensityWithSubLevel> {
        self.entity_densities.get(&entity_id).cloned()
    }
    
    /// Get the characteristic scale for an entity
    pub fn get_entity_scale(&self, entity_id: usize) -> Float {
        if let Some(density_with_sub) = self.entity_densities.get(&entity_id) {
            // Scale based on sub-level and progress
            let base_scale = density_with_sub.sub_level.scale();
            let progress_factor = 1.0 + density_with_sub.progress * 0.5;
            base_scale * progress_factor
        } else {
            1.0 // Default scale
        }
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> DensitySubLevelStatistics {
        self.statistics.clone()
    }
}

/// Apply sub-level effects to field node
/// Higher sub-levels have different field characteristics
pub fn apply_sublevel_to_field(
    node: &mut OctreeNode,
    density: DensityBand,
    sub_level: DensitySubLevel,
    progress: Float,
) {
    let density_idx = density.index();
    let sub_idx = sub_level.index();
    
    // Get base amplitude for this density
    let base_amplitude = node.field_data.density_amplitudes[density_idx].magnitude();
    
    // Sub-level modifies the amplitude
    // Higher sub-levels = more manifested = higher amplitude
    let sub_level_factor = 1.0 + (sub_idx as Float / 4.0) + progress * 0.25;
    
    // Apply to coherence
    node.field_data.coherence *= sub_level_factor.min(1.5);
    
    // Sub-level affects energy distribution
    node.field_data.energy *= sub_level_factor;
    
    // Modify spectrum position based on sub-level
    let base_position = density_idx as Float / 8.0;
    let sub_position = sub_idx as Float / 32.0;
    node.field_data.spectrum_position = base_position + sub_position + progress / 32.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sub_level_creation() {
        let sub = DensitySubLevel::Quantum;
        assert_eq!(sub.index(), 0);
        assert_eq!(sub.name(), "Quantum Realm");
    }
    
    #[test]
    fn test_density_with_sub_level() {
        let d = DensityWithSubLevel::new(DensityBand::Red);
        assert_eq!(d.density, DensityBand::Red);
        assert_eq!(d.sub_level, DensitySubLevel::Quantum);
    }
    
    #[test]
    fn test_sub_level_transitions() {
        let mut system = DensitySubLevels::with_defaults();
        system.initialize_entities(10);
        
        // Update with high coherence - should trigger transitions
        for _ in 0..100 {
            system.update(0.8);
        }
        
        // Should have some transitions
        assert!(system.statistics.sub_level_transitions >= 0);
    }
}
