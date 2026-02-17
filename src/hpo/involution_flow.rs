//! Involution Flow (Phase 3)
//!
//! From REFACTOR_ROADMAP_COMPREHENSIVE_2026.md:
//! "The fourth phase implements the top-down causal chain from THE ONE through LOGOS, 
//! SubLogos, SubSubLogos to individual entities."
//!
//! This module implements the cosmological hierarchy:
//! - THE ONE (Intelligent Infinity): Ground state
//! - LOGOS: Creative principle dividing into 7 rays + 3 aspects
//! - SubLogos: Galactic/solar scale structures
//! - SubSubLogos: Planetary scale + Veil crossing
//! - Entity: Emergence from field configurations
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The LOGOS divides into 7 + 3 aspects, each SubLogos creates a cosmic evol"

use super::field_state::{Complex, DensityBand, FieldNodeData, Float, HolographicFieldState, OctreeNode};
use super::spectrum_dynamics::SpectrumDynamics;

/// The Seven Rays - fundamental vibrational frequencies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ray {
    Will = 0,    // Red ray - 1st ray
    Love = 1,    // Blue ray - 2nd ray
    Wisdom = 2,  // Yellow ray - 3rd ray
    Execution = 3, // Green ray - 4th ray
    Knowledge = 4, // Orange ray - 5th ray
    Order = 5,   // Indigo ray - 6th ray
    Essence = 6, // Violet ray - 7th ray
}

impl Ray {
    pub fn count() -> usize { 7 }
    
    pub fn index(&self) -> usize {
        *self as usize
    }
    
    pub fn from_index(i: usize) -> Option<Ray> {
        match i {
            0 => Some(Ray::Will),
            1 => Some(Ray::Love),
            2 => Some(Ray::Wisdom),
            3 => Some(Ray::Execution),
            4 => Some(Ray::Knowledge),
            5 => Some(Ray::Order),
            6 => Some(Ray::Essence),
            _ => None,
        }
    }
    
    /// Get the characteristic frequency for this ray
    pub fn frequency(&self) -> Float {
        match self {
            Ray::Will => 1.0,
            Ray::Love => 2.0,
            Ray::Wisdom => 4.0,
            Ray::Execution => 8.0,
            Ray::Knowledge => 16.0,
            Ray::Order => 32.0,
            Ray::Essence => 64.0,
        }
    }
    
    /// Get the color for this ray (RGB)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Ray::Will => (255, 0, 0),       // Red
            Ray::Love => (0, 0, 255),       // Blue
            Ray::Wisdom => (255, 255, 0),   // Yellow
            Ray::Execution => (0, 255, 0),   // Green
            Ray::Knowledge => (255, 165, 0), // Orange
            Ray::Order => (75, 0, 130),     // Indigo
            Ray::Essence => (128, 0, 128),  // Violet
        }
    }
}

/// The Three Aspects (additional to 7 rays)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Aspect {
    Active = 0,   // Active masculine
    Passive = 1,  // Passive feminine  
    Neutral = 2, // Neutral/unified
}

impl Aspect {
    pub fn count() -> usize { 3 }
}

/// Configuration for the cosmic hierarchy
#[derive(Debug, Clone)]
pub struct CosmicHierarchyConfig {
    /// Number of SubLogos levels (galactic, solar, etc.)
    pub sublogos_levels: usize,
    
    /// Number of SubSubLogos per SubLogos
    pub subsublogos_per_sublogos: usize,
    
    /// Entity spawn threshold (field energy required for emergence)
    pub entity_threshold: Float,
    
    /// Initial field coherence
    pub initial_coherence: Float,
    
    /// Veil position for SubSubLogos
    pub veil_position: Float,
}

impl Default for CosmicHierarchyConfig {
    fn default() -> Self {
        CosmicHierarchyConfig {
            sublogos_levels: 2,     // Galactic, Solar
            subsublogos_per_sublogos: 10,
            entity_threshold: 0.5,
            initial_coherence: 0.5,
            veil_position: 1.0,
        }
    }
}

/// THE ONE - Intelligent Infinity (Ground State)
/// From COSMOLOGICAL-ARCHITECTURE.md: "THE ONE or Intelligent Infinity represents 
/// the ground state from which all emerges"
pub struct TheOne {
    /// Unity - perfect, undifferentiated state
    pub unity: Float,
    
    /// Potential - infinite possibility
    pub potential: Float,
    
    /// Config
    pub config: CosmicHierarchyConfig,
}

impl TheOne {
    pub fn new() -> Self {
        TheOne {
            unity: 1.0,  // Perfect unity
            potential: 1.0, // Infinite potential
            config: CosmicHierarchyConfig::default(),
        }
    }

    /// Apply the First Distortion (Free Will) to create perturbations
    pub fn apply_first_distortion(&self, field: &mut HolographicFieldState) {
        // Free Will breaks the perfect symmetry
        // This creates the initial field perturbations
        let bounds = field.root.bounds;
        let center = bounds.center();
        
        // Add small perturbations at the center
        field.add_energy_at(center, 0, 0.001);
        
        // The unity is now potentially broken
        // Field can evolve from this point
    }
    
    /// Initialize the field from THE ONE
    pub fn initialize_field(&self, field: &mut HolographicFieldState) {
        // Start with perfect coherence
        field.root.field_data.coherence = self.config.initial_coherence;
        
        // Initialize spectrum at the ONE's perspective (beyond densities)
        field.root.field_data.spectrum_position = 2.0; // Beyond physical octave
        
        // Apply first distortion
        self.apply_first_distortion(field);
    }
}

/// LOGOS - The Creative Principle
/// From COSMOLOGICAL-ARCHITECTURE.md: "The LOGOS divides into 7 + 3 aspects, 
/// each SubLogos creates a cosmic evol"
pub struct Logos {
    /// The Seven Rays
    pub rays: [Float; 7],
    
    /// The Three Aspects
    pub aspects: [Float; 3],
    
    /// Dominant ray
    pub dominant_ray: Ray,
    
    /// LOGOS creates coherence
    pub coherence: Float,
    
    /// Parent (THE ONE)
    pub parent_unity: Float,
}

impl Logos {
    pub fn new() -> Self {
        Logos {
            rays: [0.0; 7],
            aspects: [0.0; 3],
            dominant_ray: Ray::Love, // Default
            coherence: 1.0,
            parent_unity: 1.0,
        }
    }

    /// Initialize LOGOS from THE ONE
    pub fn from_the_one(the_one: &TheOne) -> Self {
        let mut logos = Logos::new();
        logos.parent_unity = the_one.unity;
        logos.coherence = the_one.unity;
        
        // Initialize rays at equal strength initially
        for i in 0..7 {
            logos.rays[i] = 1.0 / 7.0;
        }
        
        // Initialize aspects
        logos.aspects = [1.0/3.0; 3];
        
        logos
    }

    /// Configure LOGOS with specific ray emphasis
    pub fn with_ray_emphasis(mut self, ray: Ray, strength: Float) -> Self {
        // Increase the specified ray
        self.rays[ray.index()] = strength;
        
        // Normalize others
        let remaining = 1.0 - strength;
        for i in 0..7 {
            if i != ray.index() {
                self.rays[i] = remaining / 6.0;
            }
        }
        
        self.dominant_ray = ray;
        self
    }

    /// Apply LOGOS configuration to field
    pub fn apply_to_field(&self, field: &mut HolographicFieldState) {
        // Set root node coherence
        field.root.field_data.coherence = self.coherence;
        
        // Apply ray frequencies to density bands
        for (i, &ray_strength) in self.rays.iter().enumerate() {
            if let Some(density_idx) = self.ray_to_density(i) {
                // Map ray strength to density amplitude
                let amplitude = Complex::from_polar(ray_strength, i as Float * 0.5);
                field.root.field_data.density_amplitudes[density_idx] = amplitude;
            }
        }
    }
    
    /// Map ray index to density band
    fn ray_to_density(&self, ray_idx: usize) -> Option<usize> {
        // Rays correspond to densities 1-7 (index 0-6)
        if ray_idx < 7 {
            Some(ray_idx)
        } else {
            None
        }
    }
    
    /// Get the ray frequencies
    pub fn get_ray_frequencies(&self) -> [Float; 7] {
        let mut freqs = [0.0; 7];
        for i in 0..7 {
            if let Some(ray) = Ray::from_index(i) {
                freqs[i] = ray.frequency() * self.rays[i];
            }
        }
        freqs
    }
}

/// SubLogos - Galactic/Solar Scale Structures
/// From COSMOLOGICAL-ARCHITECTURE.md: "SubLogos at galactic and solar scales set 
/// boundary conditions and parameters for field evolution"
pub struct SubLogos {
    /// LOGOS configuration inherited
    pub rays: [Float; 7],
    
    /// Scale level (0 = galactic, 1 = solar)
    pub scale_level: usize,
    
    /// Spatial bounds of this SubLogos domain
    pub bounds: [[Float; 3]; 2], // [min, max]
    
    /// Dominant ray at this scale
    pub dominant_ray: Ray,
    
    /// Consciousness density in this region
    pub consciousness_density: Float,
    
    /// Speed of light in this region
    pub speed_of_light: Float,
}

impl SubLogos {
    pub fn new(scale_level: usize, rays: [Float; 7]) -> Self {
        SubLogos {
            rays,
            scale_level,
            bounds: [[-1000.0, -1000.0, -1000.0], [1000.0, 1000.0, 1000.0]],
            dominant_ray: Ray::from_index(rays.iter().position(|&r| r == rays.iter().cloned().fold(0.0, |a, b| a.max(b))).unwrap_or(1)).unwrap_or(Ray::Love),
            consciousness_density: 0.5,
            speed_of_light: 1.0,
        }
    }

    /// Create a SubLogos at galactic scale
    pub fn galactic(rays: [Float; 7]) -> Self {
        let mut sub = SubLogos::new(0, rays);
        sub.bounds = [[-100000.0, -100000.0, -100000.0], [100000.0, 100000.0, 100000.0]];
        sub
    }

    /// Create a SubLogos at solar scale
    pub fn solar(rays: [Float; 7]) -> Self {
        let mut sub = SubLogos::new(1, rays);
        sub.bounds = [[-100.0, -100.0, -100.0], [100.0, 100.0, 100.0]];
        sub.speed_of_light = 1.0;
        sub
    }

    /// Apply this SubLogos configuration to field region
    pub fn apply_to_field(&self, field: &mut HolographicFieldState) {
        // Set consciousness density
        field.root.field_data.energy = self.consciousness_density;
        
        // Apply ray configuration
        for (i, &ray_strength) in self.rays.iter().enumerate() {
            if i < 8 {
                let amp = field.root.field_data.density_amplitudes[i];
                let new_mag = amp.magnitude() * (1.0 + ray_strength * 0.1);
                field.root.field_data.density_amplitudes[i] = Complex::from_polar(new_mag, amp.phase());
            }
        }
    }
}

/// SubSubLogos - Planetary Scale + Veil Crossing
/// From COSMOLOGICAL-ARCHITECTURE.md: "SubSubLogos at planetary scales further 
/// refine parameters for entities within a specific cosmic evol"
pub struct SubSubLogos {
    /// Parent SubLogos rays inherited
    pub rays: [Float; 7],
    
    /// Planetary bounds
    pub bounds: [[Float; 3]; 2],
    
    /// Veil parameters specific to this planet
    pub veil_position: Float,
    pub veil_thickness: Float,
    pub veil_transparency: Float,
    
    /// Physical constants for this world
    pub gravity: Float,
    pub time_flow: Float,
    
    /// Entity capacity
    pub max_entities: usize,
    
    /// Current entity count
    pub entity_count: usize,
}

impl SubSubLogos {
    pub fn new(rays: [Float; 7]) -> Self {
        SubSubLogos {
            rays,
            bounds: [[-10.0, -10.0, -10.0], [10.0, 10.0, 10.0]],
            veil_position: 1.0,
            veil_thickness: 0.1,
            veil_transparency: 0.5,
            gravity: 1.0,
            time_flow: 1.0,
            max_entities: 10000,
            entity_count: 0,
        }
    }

    /// Apply this SubSubLogos configuration
    pub fn apply_to_field(&self, field: &mut HolographicFieldState) {
        // Set veil parameters
        field.root.field_data.veil_transparency = self.veil_transparency;
        
        // The spectrum position at SubSubLogos level crosses the veil
        // This is where physical manifestation begins
        field.root.field_data.spectrum_position = self.veil_position - self.veil_thickness;
    }

    /// Check if conditions are met for entity emergence
    pub fn can_spawn_entity(&self, field_energy: Float) -> bool {
        field_energy > 0.1 && self.entity_count < self.max_entities
    }

    /// Spawn a new entity
    pub fn spawn_entity(&mut self) -> usize {
        if self.can_spawn_entity(0.5) {
            self.entity_count += 1;
            self.entity_count - 1
        } else {
            0
        }
    }
}

/// Complete cosmic hierarchy
pub struct CosmicHierarchy {
    /// THE ONE
    pub the_one: TheOne,
    
    /// LOGOS
    pub logos: Logos,
    
    /// SubLogos (galactic, solar)
    pub sublogos: Vec<SubLogos>,
    
    /// SubSubLogos (planetary)
    pub subsublogos: Vec<SubSubLogos>,
    
    /// Current active SubSubLogos
    pub active_subsublogos: Option<usize>,
    
    /// Configuration
    pub config: CosmicHierarchyConfig,
    
    /// Statistics
    pub statistics: HierarchyStatistics,
}

/// Statistics for cosmic hierarchy
#[derive(Debug, Clone, Default)]
pub struct HierarchyStatistics {
    /// Total entities spawned
    pub entities_spawned: usize,
    
    /// Total veil crossings
    pub veil_crossings: usize,
    
    /// Average consciousness level
    pub average_consciousness: Float,
    
    /// Hierarchy depth reached
    pub max_depth: usize,
}

impl CosmicHierarchy {
    pub fn new() -> Self {
        let the_one = TheOne::new();
        let logos = Logos::from_the_one(&the_one);
        
        CosmicHierarchy {
            the_one,
            logos,
            sublogos: Vec::new(),
            subsublogos: Vec::new(),
            active_subsublogos: None,
            config: CosmicHierarchyConfig::default(),
            statistics: HierarchyStatistics::default(),
        }
    }

    /// Initialize the hierarchy from THE ONE
    pub fn initialize(&mut self, field: &mut HolographicFieldState) {
        // THE ONE initializes the field
        self.the_one.initialize_field(field);
        
        // LOGOS configures the field
        self.logos.apply_to_field(field);
        
        self.statistics.max_depth = 1;
    }

    /// Add a SubLogos level (galactic/solar)
    pub fn add_sublogos(&mut self, sublogos: SubLogos) {
        self.sublogos.push(sublogos);
        self.statistics.max_depth = 2 + self.sublogos.len();
    }

    /// Add a SubSubLogos (planetary)
    pub fn add_subsublogos(&mut self, subsublogos: SubSubLogos) {
        self.subsublogos.push(subsublogos);
        self.active_subsublogos = Some(self.subsublogos.len() - 1);
        self.statistics.max_depth = 3;
    }

    /// Propagate configuration down the hierarchy
    pub fn propagate_down(&self, field: &mut HolographicFieldState) {
        // Apply LOGOS
        self.logos.apply_to_field(field);
        
        // Apply SubLogos
        for sub in &self.sublogos {
            sub.apply_to_field(field);
        }
        
        // Apply active SubSubLogos
        if let Some(idx) = self.active_subsublogos {
            if idx < self.subsublogos.len() {
                self.subsublogos[idx].apply_to_field(field);
            }
        }
    }

    /// Process entity emergence
    pub fn process_entity_emergence(&mut self, field: &mut HolographicFieldState) -> Vec<[Float; 3]> {
        let mut new_entities = Vec::new();
        
        if let Some(idx) = self.active_subsublogos {
            if idx < self.subsublogos.len() {
                let subsub = &mut self.subsublogos[idx];
                
                // Check root energy for entity spawn
                if subsub.can_spawn_entity(field.root.field_data.energy) {
                    // Spawn at random position within bounds
                    let pos = [
                        (rand_pos() * 20.0 - 10.0),
                        (rand_pos() * 20.0 - 10.0),
                        (rand_pos() * 20.0 - 10.0),
                    ];
                    
                    // Add energy at position
                    field.add_energy_at(pos, 3, 0.5);
                    
                    new_entities.push(pos);
                    self.statistics.entities_spawned += 1;
                    subsub.entity_count += 1;
                    
                    // Check for veil crossing
                    if field.root.field_data.spectrum_position < subsub.veil_position {
                        self.statistics.veil_crossings += 1;
                    }
                }
            }
        }
        
        new_entities
    }
}

// Simple pseudo-random for entity positions
fn rand_pos() -> Float {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as Float % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_one_creation() {
        let one = TheOne::new();
        assert_eq!(one.unity, 1.0);
    }

    #[test]
    fn test_logos_creation() {
        let logos = Logos::new();
        assert_eq!(logos.rays.len(), 7);
    }

    #[test]
    fn test_sublogos_creation() {
        let sub = SubLogos::galactic([0.0; 7]);
        assert_eq!(sub.scale_level, 0);
    }

    #[test]
    fn test_cosmic_hierarchy() {
        let mut hierarchy = CosmicHierarchy::new();
        let mut field = HolographicFieldState::with_defaults();
        hierarchy.initialize(&mut field);
        
        assert!(hierarchy.statistics.max_depth >= 1);
    }
}
