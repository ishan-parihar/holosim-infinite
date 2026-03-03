//! # Template Factory - Instantiation and Caching
//!
//! Factory for creating and caching UniversalTemplate instances.
//!
//! ## Key Features
//!
//! 1. **Template Caching**: Similar configurations reuse cached templates
//! 2. **Lazy Instantiation**: Create on first use, cache for reuse
//! 3. **Component Agnostic**: Works with any TemplateComponent type
//!
//! ## Memory Optimization
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Store once, reconstruct as needed"
//! "O(1) cache hit for previously instantiated patterns"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::evolution_density_octave::density_octave::Density;
use crate::holographic::holographic_field::HolographicField;

use super::{
    component_data::TemplateComponent, ArchetypeActivationProfile, SpectrumConfiguration,
    TemplateConfig, TemplateKey,
};

// =============================================================================
// ENHANCED TEMPLATE KEY
// =============================================================================

/// Enhanced key for template caching with component type awareness
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnhancedTemplateKey {
    /// Base key from configuration
    pub base: TemplateKey,

    /// Component type identifier
    pub component_type: ComponentTypeId,
}

/// Component type identifier for caching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentTypeId {
    Particle,
    Atom,
    Molecule,
    Cell,
    Entity,
    World,
    Star,
    Galaxy,
}

// =============================================================================
// TEMPLATE FACTORY
// =============================================================================

/// Factory for creating and caching UniversalTemplate instances
///
/// The factory maintains:
/// 1. Shared holographic field reference
/// 2. Cache of previously instantiated templates
/// 3. Statistics for performance monitoring
pub struct TemplateFactory<T: TemplateComponent> {
    /// Shared holographic field for all templates
    field: Arc<HolographicField>,

    /// Cache of instantiated component data
    cache: HashMap<EnhancedTemplateKey, Arc<T>>,

    /// Cache statistics
    stats: FactoryStats,

    /// Component type for this factory
    component_type: ComponentTypeId,
}

/// Factory performance statistics
#[derive(Debug, Clone, Default)]
pub struct FactoryStats {
    /// Total instantiation requests
    pub total_requests: u64,

    /// Cache hits
    pub cache_hits: u64,

    /// Cache misses (new instantiations)
    pub cache_misses: u64,

    /// Cache size
    pub cache_size: usize,
}

impl<T: TemplateComponent> TemplateFactory<T> {
    /// Create a new template factory with a shared holographic field
    pub fn new(field: Arc<HolographicField>, component_type: ComponentTypeId) -> Self {
        Self {
            field,
            cache: HashMap::new(),
            stats: FactoryStats::default(),
            component_type,
        }
    }

    /// Get the shared holographic field
    pub fn field(&self) -> &Arc<HolographicField> {
        &self.field
    }

    /// Instantiate a template from configuration
    ///
    /// This will:
    /// 1. Check if a similar template is already cached
    /// 2. Return cached instance if found
    /// 3. Otherwise, create new instance and cache it
    pub fn instantiate(&mut self, config: &TemplateConfig) -> Arc<T> {
        self.stats.total_requests += 1;

        let key = self.compute_key(config);

        if let Some(cached) = self.cache.get(&key) {
            self.stats.cache_hits += 1;
            return Arc::clone(cached);
        }

        self.stats.cache_misses += 1;

        let component = T::from_template_config(config);
        let arc_component = Arc::new(component);

        self.cache.insert(key, Arc::clone(&arc_component));
        self.stats.cache_size = self.cache.len();

        arc_component
    }

    /// Instantiate a unique template (not cached)
    ///
    /// Use this when you need a unique instance that shouldn't be reused
    pub fn instantiate_unique(&self, config: &TemplateConfig) -> T {
        T::from_template_config(config)
    }

    /// Check if a template is cached
    pub fn is_cached(&self, config: &TemplateConfig) -> bool {
        let key = self.compute_key(config);
        self.cache.contains_key(&key)
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.stats.cache_size = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> &FactoryStats {
        &self.stats
    }

    /// Get cache hit rate (0.0 to 1.0)
    pub fn hit_rate(&self) -> f64 {
        if self.stats.total_requests == 0 {
            0.0
        } else {
            self.stats.cache_hits as f64 / self.stats.total_requests as f64
        }
    }

    /// Compute cache key from configuration
    fn compute_key(&self, config: &TemplateConfig) -> EnhancedTemplateKey {
        EnhancedTemplateKey {
            base: TemplateKey::from_config(config),
            component_type: self.component_type,
        }
    }
}

// =============================================================================
// MULTI-TYPE TEMPLATE REGISTRY
// =============================================================================

/// Registry for managing multiple template factories
///
/// This provides a unified interface for instantiating any component type
pub struct TemplateRegistry {
    /// Shared holographic field
    field: Arc<HolographicField>,

    /// Particle factory
    particle_factory: TemplateFactory<crate::template::component_data::ParticleData>,

    /// Atom factory
    atom_factory: TemplateFactory<crate::template::component_data::AtomData>,

    /// Cell factory
    cell_factory: TemplateFactory<crate::template::component_data::CellData>,

    /// Entity factory
    entity_factory: TemplateFactory<crate::template::entity_data::EntityData>,

    /// World factory
    world_factory: TemplateFactory<crate::template::component_data::WorldData>,

    /// Star factory
    star_factory: TemplateFactory<crate::template::component_data::StarData>,

    /// Galaxy factory
    galaxy_factory: TemplateFactory<crate::template::component_data::GalaxyData>,
}

impl TemplateRegistry {
    /// Create a new template registry
    pub fn new(field: Arc<HolographicField>) -> Self {
        Self {
            particle_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Particle),
            atom_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Atom),
            cell_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Cell),
            entity_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Entity),
            world_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::World),
            star_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Star),
            galaxy_factory: TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Galaxy),
            field,
        }
    }

    /// Create with default holographic field
    pub fn with_default_field() -> Self {
        use crate::holographic::complex_vectors::ComplexArchetype;

        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for i in 0..22 {
                a[i] = ComplexArchetype {
                    amplitude: (i as f64 + 1.0) / 22.0,
                    phase: (i as f64) * std::f64::consts::PI / 11.0,
                };
            }
            a
        };

        let field = Arc::new(HolographicField::new(
            crate::holographic::InvolutionLayer::Green,
            archetypes,
        ));

        Self::new(field)
    }

    /// Get shared field reference
    pub fn field(&self) -> &Arc<HolographicField> {
        &self.field
    }

    /// Instantiate a particle
    pub fn instantiate_particle(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::ParticleData> {
        self.particle_factory.instantiate(config)
    }

    /// Instantiate an atom
    pub fn instantiate_atom(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::AtomData> {
        self.atom_factory.instantiate(config)
    }

    /// Instantiate a cell
    pub fn instantiate_cell(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::CellData> {
        self.cell_factory.instantiate(config)
    }

    /// Instantiate an entity
    pub fn instantiate_entity(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::entity_data::EntityData> {
        self.entity_factory.instantiate(config)
    }

    /// Instantiate a world
    pub fn instantiate_world(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::WorldData> {
        self.world_factory.instantiate(config)
    }

    /// Instantiate a star
    pub fn instantiate_star(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::StarData> {
        self.star_factory.instantiate(config)
    }

    /// Instantiate a galaxy
    pub fn instantiate_galaxy(
        &mut self,
        config: &TemplateConfig,
    ) -> Arc<crate::template::component_data::GalaxyData> {
        self.galaxy_factory.instantiate(config)
    }

    /// Get aggregate statistics
    pub fn stats(&self) -> RegistryStats {
        RegistryStats {
            particle: self.particle_factory.stats().clone(),
            atom: self.atom_factory.stats().clone(),
            cell: self.cell_factory.stats().clone(),
            entity: self.entity_factory.stats().clone(),
            world: self.world_factory.stats().clone(),
            star: self.star_factory.stats().clone(),
            galaxy: self.galaxy_factory.stats().clone(),
        }
    }

    /// Clear all caches
    pub fn clear_all_caches(&mut self) {
        self.particle_factory.clear_cache();
        self.atom_factory.clear_cache();
        self.cell_factory.clear_cache();
        self.entity_factory.clear_cache();
        self.world_factory.clear_cache();
        self.star_factory.clear_cache();
        self.galaxy_factory.clear_cache();
    }
}

/// Aggregate statistics for the registry
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub particle: FactoryStats,
    pub atom: FactoryStats,
    pub cell: FactoryStats,
    pub entity: FactoryStats,
    pub world: FactoryStats,
    pub star: FactoryStats,
    pub galaxy: FactoryStats,
}

impl RegistryStats {
    /// Get total cache hits across all factories
    pub fn total_hits(&self) -> u64 {
        self.particle.cache_hits
            + self.atom.cache_hits
            + self.cell.cache_hits
            + self.entity.cache_hits
            + self.world.cache_hits
            + self.star.cache_hits
            + self.galaxy.cache_hits
    }

    /// Get total cache misses across all factories
    pub fn total_misses(&self) -> u64 {
        self.particle.cache_misses
            + self.atom.cache_misses
            + self.cell.cache_misses
            + self.entity.cache_misses
            + self.world.cache_misses
            + self.star.cache_misses
            + self.galaxy.cache_misses
    }

    /// Get overall hit rate
    pub fn overall_hit_rate(&self) -> f64 {
        let total = self.total_hits() + self.total_misses();
        if total == 0 {
            0.0
        } else {
            self.total_hits() as f64 / total as f64
        }
    }
}

// =============================================================================
// TEMPLATE BUILDER
// =============================================================================

/// Builder for creating template configurations
pub struct TemplateConfigBuilder {
    spectrum: Option<SpectrumConfiguration>,
    archetype_activation: Option<ArchetypeActivationProfile>,
    density: Option<Density>,
    free_will_seed: Option<u64>,
}

impl TemplateConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            spectrum: None,
            archetype_activation: None,
            density: None,
            free_will_seed: None,
        }
    }

    /// Set spectrum configuration
    pub fn spectrum(mut self, spectrum: SpectrumConfiguration) -> Self {
        self.spectrum = Some(spectrum);
        self
    }

    /// Set archetype activation
    pub fn archetype_activation(mut self, activation: ArchetypeActivationProfile) -> Self {
        self.archetype_activation = Some(activation);
        self
    }

    /// Set archetype activation from array
    pub fn archetype_array(mut self, coefficients: [f64; 22]) -> Self {
        self.archetype_activation = Some(ArchetypeActivationProfile::new(coefficients));
        self
    }

    /// Set density
    pub fn density(mut self, density: Density) -> Self {
        self.density = Some(density);
        self
    }

    /// Set free will seed
    pub fn free_will_seed(mut self, seed: u64) -> Self {
        self.free_will_seed = Some(seed);
        self
    }

    /// Build the configuration
    pub fn build(self) -> TemplateConfig {
        TemplateConfig::new(
            self.spectrum
                .unwrap_or_else(SpectrumConfiguration::balanced),
            self.archetype_activation
                .unwrap_or_else(ArchetypeActivationProfile::default),
            self.density.unwrap_or_else(|| {
                Density::First(
                    crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
                )
            }),
            self.free_will_seed.unwrap_or(0),
        )
    }

    /// Build with a random free will seed
    pub fn build_with_random_seed(self) -> TemplateConfig {
        use rand::Rng;
        let seed = rand::thread_rng().gen();
        self.free_will_seed(seed).build()
    }
}

impl Default for TemplateConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holographic::complex_vectors::ComplexArchetype;

    fn create_test_field() -> Arc<HolographicField> {
        let archetypes = {
            let mut a = [ComplexArchetype {
                amplitude: 0.0,
                phase: 0.0,
            }; 22];
            for i in 0..22 {
                a[i] = ComplexArchetype {
                    amplitude: 0.5,
                    phase: 0.0,
                };
            }
            a
        };
        Arc::new(HolographicField::new(
            crate::holographic::InvolutionLayer::Green,
            archetypes,
        ))
    }

    #[test]
    fn test_template_factory_creation() {
        let field = create_test_field();
        let factory: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(field, ComponentTypeId::Particle);

        assert_eq!(factory.stats().total_requests, 0);
    }

    #[test]
    fn test_template_factory_instantiate() {
        let field = create_test_field();
        let mut factory: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(field, ComponentTypeId::Particle);

        let config = TemplateConfig::default();
        let particle = factory.instantiate(&config);

        assert_eq!(factory.stats().total_requests, 1);
        assert_eq!(factory.stats().cache_misses, 1);
        assert_eq!(factory.stats().cache_hits, 0);
    }

    #[test]
    fn test_template_factory_caching() {
        let field = create_test_field();
        let mut factory: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(field, ComponentTypeId::Particle);

        let config = TemplateConfig::default();

        // First instantiation
        let _p1 = factory.instantiate(&config);
        assert_eq!(factory.stats().cache_misses, 1);

        // Second instantiation with same config - should hit cache
        let _p2 = factory.instantiate(&config);
        assert_eq!(factory.stats().cache_hits, 1);
        assert_eq!(factory.hit_rate(), 0.5);
    }

    #[test]
    fn test_template_config_builder() {
        let config = TemplateConfigBuilder::new().free_will_seed(42).build();

        assert_eq!(config.free_will_seed, 42);
    }

    #[test]
    fn test_template_registry() {
        let registry = TemplateRegistry::with_default_field();
        assert!(Arc::strong_count(registry.field()) >= 1);
    }

    #[test]
    fn test_o1_caching_performance() {
        let field = create_test_field();
        let mut factory: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(field, ComponentTypeId::Particle);

        let config = TemplateConfig::default();

        // First call - cache miss
        let start = std::time::Instant::now();
        let _p1 = factory.instantiate(&config);
        let first_call_time = start.elapsed();

        // Subsequent calls - cache hits (should be O(1))
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _p = factory.instantiate(&config);
        }
        let cached_calls_time = start.elapsed();

        // Cache hit rate should be high after repeated calls
        assert!(factory.hit_rate() > 0.99);

        // Each cached call should be very fast (sub-microsecond)
        let avg_cached_time = cached_calls_time.as_nanos() / 1000;
        assert!(
            avg_cached_time < 10000,
            "Cached calls should be < 10 microseconds each"
        );
    }

    #[test]
    fn test_registry_all_component_types() {
        let mut registry = TemplateRegistry::with_default_field();
        let config = TemplateConfig::default();

        // Test all component types can be instantiated
        let _particle = registry.instantiate_particle(&config);
        let _atom = registry.instantiate_atom(&config);
        let _cell = registry.instantiate_cell(&config);
        let _entity = registry.instantiate_entity(&config);
        let _world = registry.instantiate_world(&config);
        let _star = registry.instantiate_star(&config);
        let _galaxy = registry.instantiate_galaxy(&config);

        let stats = registry.stats();
        assert_eq!(stats.total_misses(), 7);
    }

    #[test]
    fn test_registry_cache_clear() {
        let mut registry = TemplateRegistry::with_default_field();
        let config = TemplateConfig::default();

        // Instantiate components
        let _ = registry.instantiate_particle(&config);
        let _ = registry.instantiate_atom(&config);

        let stats = registry.stats();
        assert!(stats.total_hits() + stats.total_misses() > 0);

        // Clear caches - this should reset cache sizes
        registry.clear_all_caches();

        // Verify cache sizes are 0
        let stats = registry.stats();
        assert_eq!(stats.particle.cache_size, 0);
        assert_eq!(stats.atom.cache_size, 0);
    }

    #[test]
    fn test_holographic_memory_efficiency() {
        // Demonstrate that multiple factories share the same field reference
        let field = create_test_field();
        let initial_count = Arc::strong_count(&field);

        let _factory1: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Particle);
        let _factory2: TemplateFactory<crate::template::component_data::AtomData> =
            TemplateFactory::new(Arc::clone(&field), ComponentTypeId::Atom);
        let _factory3: TemplateFactory<crate::template::component_data::WorldData> =
            TemplateFactory::new(Arc::clone(&field), ComponentTypeId::World);

        // All factories share the same field reference
        assert_eq!(Arc::strong_count(&field), initial_count + 3);
    }

    #[test]
    fn test_archetype_derivation_consistency() {
        // Test that archetype activation consistently derives properties
        let field = create_test_field();
        let mut factory: TemplateFactory<crate::template::component_data::ParticleData> =
            TemplateFactory::new(field, ComponentTypeId::Particle);

        // Create config with specific archetype pattern
        let mut activation =
            crate::holographic::universal_template::ArchetypeActivationProfile::default();
        activation.coefficients = [0.1; 22]; // Low catalyst -> electron

        let config = TemplateConfigBuilder::new()
            .archetype_activation(activation)
            .free_will_seed(42)
            .build();

        // Same config should always produce same particle type
        let p1 = factory.instantiate(&config);
        let p2 = factory.instantiate(&config);

        assert_eq!(p1.particle_type, p2.particle_type);
        assert_eq!(
            p1.particle_type,
            crate::template::component_data::ParticleType::Electron
        );
    }
}
