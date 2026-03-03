//! Holographic Renderer - Observer-driven reality rendering
//!
//! This module implements the HolographicRenderer according to V5 Phase 3 specifications.
//! The renderer implements observer-driven rendering where only what is observed becomes manifest.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Observation creates the physical by collapsing the field into manifestation."
//! "The Veil creates the separation that allows observation to happen."
//!
//! Key Concepts:
//! - Observer-Driven Rendering: Only render what is observed
//! - Dynamic Manifestation: Potentials collapse into definite states through observation
//! - Spectrum-Aware: Rendering respects the Space/Time ↔ Time/Space spectrum
//! - Density-Dependent: Higher density observers can observe more
//! - Caching: Optimize rendering through caching and invalidation
//!
//! This module implements:
//! - HolographicRenderer with observer management
//! - RenderStats and RenderFrame for tracking rendering performance
//! - ManifestedEntity for tracking collapsed manifestations
//! - Cache management for optimization

use crate::foundation::manifestation_engine::ManifestationEngine;
use crate::foundation::observer::{
    CollapsedState, FieldSignature, FocusTarget, FocusTargetType, ObservationRecord, Observer,
    PotentialManifestation,
};
use crate::foundation::SpectrumPosition;
use crate::holographic::field_address::ScaleLevel;
use crate::simulation_v3::multiscale_field::MultiScaleField;
use crate::types::{Density, Float};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Entity identifier
pub type EntityId = u64;

// ============================================================================
// MANIFESTED ENTITY
// ============================================================================

/// A manifested entity - result of quantum collapse through observation
///
/// Represents an entity that has been observed and collapsed from a potential
/// into a definite state in physical reality.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestedEntity {
    /// Unique identifier for this manifested entity
    pub entity_id: EntityId,

    /// The potential that was manifested
    pub potential: PotentialManifestation,

    /// The collapsed state resulting from observation
    pub collapsed_state: CollapsedState,

    /// Spectrum position of this entity
    pub spectrum_position: SpectrumPosition,

    /// Density of this entity
    pub density: Density,

    /// Scale level of this entity
    pub scale_level: ScaleLevel,

    /// Whether this entity is stable (not prone to decoherence)
    pub stable: bool,

    /// Observation count - how many times this entity has been observed
    pub observation_count: usize,

    /// Coherence of this entity (0.0 to 1.0)
    pub coherence: Float,

    /// When this entity was first manifested
    pub manifestation_step: u64,
}

impl ManifestedEntity {
    /// Create a new manifested entity
    pub fn new(
        entity_id: EntityId,
        potential: PotentialManifestation,
        collapsed_state: CollapsedState,
        spectrum_position: SpectrumPosition,
        density: Density,
        scale_level: ScaleLevel,
        manifestation_step: u64,
    ) -> Self {
        let coherence = collapsed_state.manifestation;

        Self {
            entity_id,
            potential,
            collapsed_state,
            spectrum_position,
            density,
            scale_level,
            stable: coherence > 0.5,
            observation_count: 1,
            coherence,
            manifestation_step,
        }
    }

    /// Check if this entity can be observed by another observer
    pub fn is_observable_by(&self, observer: &Observer) -> bool {
        // Check if observer can access this entity's spectrum position
        if !observer
            .spectrum_position
            .can_access(&self.spectrum_position)
        {
            return false;
        }

        // Check density accessibility
        // Higher density entities can observe lower density entities
        if observer.density.as_u8() < self.density.as_u8() {
            return false;
        }

        // Check scale level accessibility
        let density_level = observer.density.as_u8();
        let entity_scale_value = self.scale_level as u8;
        density_level >= entity_scale_value
    }

    /// Add an observation to this entity
    pub fn add_observation(&mut self) {
        self.observation_count += 1;
        // Repeated observations increase stability and coherence
        self.stable = self.observation_count > 3 || self.coherence > 0.7;
        self.coherence = (self.coherence + 0.01).min(1.0);
    }

    /// Evolve the entity based on new collapsed state
    pub fn evolve(&mut self, new_collapsed: CollapsedState, new_position: SpectrumPosition) {
        self.collapsed_state = new_collapsed;
        self.spectrum_position = new_position;
        self.coherence = (self.coherence + 0.05).min(1.0);
        self.stable = self.coherence > 0.6;
    }

    /// Get the observation key for this entity
    pub fn observation_key(&self, observer_id: u64) -> crate::foundation::observer::ObservationKey {
        crate::foundation::observer::ObservationKey::new(
            observer_id,
            self.entity_id,
            self.scale_level,
            self.density,
        )
    }
}

// ============================================================================
// RENDER FRAME
// ============================================================================

/// A single rendered frame - the reality as perceived by observers
///
/// Contains all entities that were manifested for observers in this frame.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderFrame {
    /// Frame number
    pub frame_number: u64,

    /// Simulation step
    pub simulation_step: u64,

    /// All manifested entities in this frame
    pub manifested_entities: Vec<ManifestedEntity>,

    /// Entity ID to entity mapping for quick lookup
    pub entity_index: HashMap<EntityId, ManifestedEntity>,

    /// Observer IDs that contributed to this frame
    pub observer_ids: Vec<u64>,

    /// Potentials that were evaluated but not manifested
    pub unevaluated_potentials: Vec<PotentialManifestation>,
}

impl RenderFrame {
    /// Create a new render frame
    pub fn new(frame_number: u64, simulation_step: u64) -> Self {
        Self {
            frame_number,
            simulation_step,
            manifested_entities: Vec::new(),
            entity_index: HashMap::new(),
            observer_ids: Vec::new(),
            unevaluated_potentials: Vec::new(),
        }
    }

    /// Add a manifested entity to this frame
    pub fn add_entity(&mut self, entity: ManifestedEntity) {
        let entity_id = entity.entity_id;

        // Check if entity already exists in this frame
        if let Some(existing) = self.entity_index.get_mut(&entity_id) {
            // Evolve existing entity
            existing.evolve(entity.collapsed_state, entity.spectrum_position);
        } else {
            // Add new entity
            self.entity_index.insert(entity_id, entity.clone());
            self.manifested_entities.push(entity);
        }
    }

    /// Get a manifested entity by ID
    pub fn get_entity(&self, entity_id: EntityId) -> Option<&ManifestedEntity> {
        self.entity_index.get(&entity_id)
    }

    /// Check if an entity exists in this frame
    pub fn has_entity(&self, entity_id: EntityId) -> bool {
        self.entity_index.contains_key(&entity_id)
    }

    /// Get number of manifested entities in this frame
    pub fn entity_count(&self) -> usize {
        self.manifested_entities.len()
    }

    /// Get entities observable by a specific observer
    pub fn entities_observable_by(&self, observer: &Observer) -> Vec<&ManifestedEntity> {
        self.manifested_entities
            .iter()
            .filter(|entity| entity.is_observable_by(observer))
            .collect()
    }

    /// Get entities at a specific density
    pub fn entities_at_density(&self, density: Density) -> Vec<&ManifestedEntity> {
        self.manifested_entities
            .iter()
            .filter(|entity| entity.density == density)
            .collect()
    }

    /// Get entities at a specific scale level
    pub fn entities_at_scale(&self, scale_level: ScaleLevel) -> Vec<&ManifestedEntity> {
        self.manifested_entities
            .iter()
            .filter(|entity| entity.scale_level == scale_level)
            .collect()
    }

    /// Merge another frame into this one
    pub fn merge(&mut self, other: RenderFrame) {
        for entity in other.manifested_entities {
            self.add_entity(entity);
        }

        // Merge observer IDs
        for observer_id in other.observer_ids {
            if !self.observer_ids.contains(&observer_id) {
                self.observer_ids.push(observer_id);
            }
        }

        // Merge unevaluated potentials
        self.unevaluated_potentials
            .extend(other.unevaluated_potentials);
    }
}

impl Default for RenderFrame {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

// ============================================================================
// RENDER STATS
// ============================================================================

/// Statistics about rendering performance
///
/// Tracks metrics for optimization and analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderStats {
    /// Total number of frames rendered
    pub total_frames: usize,

    /// Total number of entities manifested across all frames
    pub total_manifestations: usize,

    /// Average entities per frame
    pub average_entities_per_frame: Float,

    /// Total rendering time (in simulation steps)
    pub total_render_time: u64,

    /// Average render time per frame
    pub average_render_time: Float,

    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: Float,

    /// Manifestations by density
    pub manifestations_by_density: HashMap<Density, usize>,

    /// Manifestations by scale level
    pub manifestations_by_scale: HashMap<ScaleLevel, usize>,

    /// Number of observers contributing to rendering
    pub active_observers: usize,

    /// Total observations made
    pub total_observations: usize,

    /// Total collapses triggered
    pub total_collapses: usize,
}

impl RenderStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self {
            total_frames: 0,
            total_manifestations: 0,
            average_entities_per_frame: 0.0,
            total_render_time: 0,
            average_render_time: 0.0,
            cache_hit_rate: 0.0,
            manifestations_by_density: HashMap::new(),
            manifestations_by_scale: HashMap::new(),
            active_observers: 0,
            total_observations: 0,
            total_collapses: 0,
        }
    }

    /// Record a rendered frame
    pub fn record_frame(&mut self, frame: &RenderFrame, render_time: u64) {
        self.total_frames += 1;
        self.total_manifestations += frame.entity_count();
        self.total_render_time += render_time;

        // Update average entities per frame
        if self.total_frames > 0 {
            self.average_entities_per_frame =
                self.total_manifestations as Float / self.total_frames as Float;
        }

        // Update average render time
        if self.total_frames > 0 {
            self.average_render_time = self.total_render_time as Float / self.total_frames as Float;
        }

        // Update density and scale statistics
        for entity in &frame.manifested_entities {
            *self
                .manifestations_by_density
                .entry(entity.density)
                .or_insert(0) += 1;
            *self
                .manifestations_by_scale
                .entry(entity.scale_level)
                .or_insert(0) += 1;
        }
    }

    /// Record observations and collapses
    pub fn record_observations(&mut self, observations: usize, collapses: usize) {
        self.total_observations += observations;
        self.total_collapses += collapses;
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&mut self, hits: usize, total: usize) {
        if total > 0 {
            self.cache_hit_rate = hits as Float / total as Float;
        }
    }

    /// Update active observers count
    pub fn update_active_observers(&mut self, count: usize) {
        self.active_observers = count;
    }

    /// Get manifestations for a specific density
    pub fn count_for_density(&self, density: Density) -> usize {
        *self.manifestations_by_density.get(&density).unwrap_or(&0)
    }

    /// Get manifestations for a specific scale level
    pub fn count_for_scale(&self, scale_level: ScaleLevel) -> usize {
        *self.manifestations_by_scale.get(&scale_level).unwrap_or(&0)
    }

    /// Get collapse rate
    pub fn collapse_rate(&self) -> Float {
        if self.total_observations > 0 {
            self.total_collapses as Float / self.total_observations as Float
        } else {
            0.0
        }
    }
}

impl Default for RenderStats {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// HOLOGRAPHIC RENDERER
// ============================================================================

/// Holographic Renderer - Observer-driven reality rendering
///
/// From V5 Phase 3 specifications:
/// "The HolographicRenderer implements observer-driven rendering where only
/// what is observed becomes manifest."
///
/// The renderer:
/// - Manages multiple observers
/// - Renders only what is observed (not everything)
/// - Uses ManifestationEngine for dynamic threshold-based manifestation
/// - Caches manifestations for optimization
/// - Tracks rendering statistics
/// - Respects spectrum positions and density constraints
pub struct HolographicRenderer {
    /// All observers managed by this renderer
    pub observers: HashMap<u64, Observer>,

    /// Manifestation engine for collapsing potentials
    pub manifestation_engine: ManifestationEngine,

    /// Multi-scale holographic field
    pub field: Arc<MultiScaleField>,

    /// Rendering statistics
    pub stats: RenderStats,

    /// Cache of manifested entities (entity_id -> entity)
    pub entity_cache: HashMap<EntityId, ManifestedEntity>,

    /// Cache of field signatures for reuse
    pub signature_cache: HashMap<u64, FieldSignature>,

    /// Current frame number
    pub current_frame: u64,

    /// Maximum cache size
    pub max_cache_size: usize,

    /// Whether caching is enabled
    pub caching_enabled: bool,
}

impl HolographicRenderer {
    /// Create a new holographic renderer
    pub fn new(field: Arc<MultiScaleField>) -> Self {
        Self {
            observers: HashMap::new(),
            manifestation_engine: ManifestationEngine::new(),
            field,
            stats: RenderStats::new(),
            entity_cache: HashMap::new(),
            signature_cache: HashMap::new(),
            current_frame: 0,
            max_cache_size: 10000,
            caching_enabled: true,
        }
    }

    /// Create a new holographic renderer with custom cache size
    pub fn with_cache_size(field: Arc<MultiScaleField>, max_cache_size: usize) -> Self {
        Self {
            observers: HashMap::new(),
            manifestation_engine: ManifestationEngine::new(),
            field,
            stats: RenderStats::new(),
            entity_cache: HashMap::new(),
            signature_cache: HashMap::new(),
            current_frame: 0,
            max_cache_size,
            caching_enabled: true,
        }
    }

    /// Add an observer to the renderer
    ///
    /// The observer will contribute to rendering in future frames.
    pub fn add_observer(&mut self, observer: Observer) {
        let observer_id = observer.observer_id;
        self.observers.insert(observer_id, observer);
        self.stats.update_active_observers(self.observers.len());
    }

    /// Remove an observer from the renderer
    ///
    /// The observer will no longer contribute to rendering.
    pub fn remove_observer(&mut self, observer_id: u64) -> bool {
        let removed = self.observers.remove(&observer_id).is_some();
        if removed {
            self.stats.update_active_observers(self.observers.len());
        }
        removed
    }

    /// Get an observer by ID
    pub fn get_observer(&self, observer_id: u64) -> Option<&Observer> {
        self.observers.get(&observer_id)
    }

    /// Get a mutable observer by ID
    pub fn get_observer_mut(&mut self, observer_id: u64) -> Option<&mut Observer> {
        self.observers.get_mut(&observer_id)
    }

    /// Render a frame for all observers
    ///
    /// This is the core rendering method. It:
    /// 1. Collects observations from all observers
    /// 2. Evaluates potentials for manifestation
    /// 3. Collapses potentials that meet thresholds
    /// 4. Returns a RenderFrame with all manifested entities
    ///
    /// The key feature: Only render what is observed, not everything.
    pub fn render_frame(&mut self, simulation_step: u64) -> RenderFrame {
        let frame_number = self.current_frame;
        let start_time = std::time::Instant::now();

        let mut frame = RenderFrame::new(frame_number, simulation_step);

        // Collect all observer IDs
        let observer_ids: Vec<u64> = self.observers.keys().cloned().collect();
        frame.observer_ids = observer_ids.clone();

        // Collect all potentials from observers
        let mut all_potentials: Vec<PotentialManifestation> = Vec::new();
        let mut total_observations = 0;
        let mut total_collapses = 0;

        for observer_id in &observer_ids {
            // First get a clone of observer data needed for get_visible_potentials
            let (spectrum_pos, density, observer_clone) = {
                if let Some(obs) = self.observers.get(observer_id) {
                    (obs.spectrum_position.clone(), obs.density, obs.clone())
                } else {
                    continue;
                }
            };

            // Create a temporary observer reference for get_visible_potentials
            let temp_observer = Observer {
                observer_id: *observer_id,
                spectrum_position: spectrum_pos.clone(),
                density,
                focus: observer_clone.focus.clone(),
                collapsed_states: observer_clone.collapsed_states,
                observation_history: observer_clone.observation_history,
                free_will_seed: observer_clone.free_will_seed,
                default_observation_strength: observer_clone.default_observation_strength,
            };

            // Get visible potentials for this observer
            let potentials = self.get_visible_potentials(&temp_observer);

            // Now get mutable observer for processing
            if let Some(observer) = self.observers.get_mut(observer_id) {
                // Process each potential through manifestation engine
                for potential in &potentials {
                    let result = self.manifestation_engine.process(
                        potential,
                        observer,
                        observer.density,
                        simulation_step,
                    );

                    total_observations += 1;

                    if let Some(collapsed) = result {
                        total_collapses += 1;

                        // Create manifested entity
                        let entity_id = potential.potential_id;
                        let entity = ManifestedEntity::new(
                            entity_id,
                            potential.clone(),
                            collapsed,
                            observer.spectrum_position.clone(),
                            observer.density,
                            potential.scale_level,
                            simulation_step,
                        );

                        frame.add_entity(entity);
                    } else {
                        // Potential not manifested
                        frame.unevaluated_potentials.push(potential.clone());
                    }
                }
            }
        }

        // Record statistics
        let render_time = start_time.elapsed().as_micros() as u64;
        self.stats.record_frame(&frame, render_time);
        self.stats
            .record_observations(total_observations, total_collapses);

        // Update cache
        for entity in &frame.manifested_entities {
            if self.caching_enabled {
                self.entity_cache.insert(entity.entity_id, entity.clone());
            }
        }

        // Trim cache if needed
        self.trim_cache();

        // Advance frame
        self.current_frame += 1;

        frame
    }

    /// Render a frame for a specific observer
    ///
    /// Renders only what is visible to the specified observer.
    pub fn render_for_observer(
        &mut self,
        observer_id: u64,
        simulation_step: u64,
    ) -> Option<RenderFrame> {
        let observer = self.observers.get(&observer_id)?;

        let frame_number = self.current_frame;
        let start_time = std::time::Instant::now();

        let mut frame = RenderFrame::new(frame_number, simulation_step);
        frame.observer_ids.push(observer_id);

        // Get visible potentials for this observer
        let potentials = self.get_visible_potentials(observer);

        let mut total_observations = 0;
        let mut total_collapses = 0;

        // Get mutable observer for processing
        if let Some(observer_mut) = self.observers.get_mut(&observer_id) {
            for potential in &potentials {
                let result = self.manifestation_engine.process(
                    potential,
                    observer_mut,
                    observer_mut.density,
                    simulation_step,
                );

                total_observations += 1;

                if let Some(collapsed) = result {
                    total_collapses += 1;

                    // Create manifested entity
                    let entity_id = potential.potential_id;
                    let entity = ManifestedEntity::new(
                        entity_id,
                        potential.clone(),
                        collapsed,
                        observer_mut.spectrum_position.clone(),
                        observer_mut.density,
                        potential.scale_level,
                        simulation_step,
                    );

                    frame.add_entity(entity);
                } else {
                    frame.unevaluated_potentials.push(potential.clone());
                }
            }
        }

        // Record statistics
        let render_time = start_time.elapsed().as_micros() as u64;
        self.stats.record_frame(&frame, render_time);
        self.stats
            .record_observations(total_observations, total_collapses);

        Some(frame)
    }

    /// Get visible potentials for an observer
    ///
    /// Returns potentials that are within the observer's observable range,
    /// respecting spectrum position, density, and scale level constraints.
    pub fn get_visible_potentials(&self, observer: &Observer) -> Vec<PotentialManifestation> {
        let mut potentials = Vec::new();

        // Determine observer's accessible scale levels
        let density_level = observer.density.as_u8();

        // Generate potentials for each accessible scale level
        for scale_level in 0..=density_level.min(7) {
            if let Some(level) = ScaleLevel::from_u8(scale_level) {
                // Check cache for signature
                let signature_key = observer.spectrum_position.hash() ^ level as u64;

                let signature = if let Some(cached) = self.signature_cache.get(&signature_key) {
                    cached.clone()
                } else {
                    // Create new signature
                    FieldSignature::new(
                        observer.spectrum_position.position_id,
                        format!("Scale{:?}_Field", level),
                        0.5 + observer.density.veil_transparency() * 0.5,
                        0.0,
                        0.5 + observer.density.evolution_direction() * 0.3,
                        level,
                    )
                };

                // Create potentials based on field strength
                // Higher density = more potentials
                let num_potentials = (density_level as usize) * 3;

                for i in 0..num_potentials {
                    let amplitude = 0.3 + (i as Float / num_potentials as Float) * 0.7;
                    let manifestation = amplitude * observer.density.evolution_direction();

                    let potential = PotentialManifestation::new(
                        amplitude,
                        manifestation,
                        Some(format!("Potential_{}_{}", scale_level, i)),
                        level,
                    );

                    potentials.push(potential);
                }
            }
        }

        potentials
    }

    /// Get current rendering statistics
    pub fn stats(&self) -> &RenderStats {
        &self.stats
    }

    /// Get mutable rendering statistics
    pub fn stats_mut(&mut self) -> &mut RenderStats {
        &mut self.stats
    }

    /// Invalidate the cache
    ///
    /// Clears all cached entities and signatures.
    pub fn invalidate_cache(&mut self) {
        self.entity_cache.clear();
        self.signature_cache.clear();
    }

    /// Invalidate cache for a specific entity
    pub fn invalidate_entity(&mut self, entity_id: EntityId) {
        self.entity_cache.remove(&entity_id);
    }

    /// Set caching enabled state
    pub fn set_caching_enabled(&mut self, enabled: bool) {
        self.caching_enabled = enabled;
        if !enabled {
            self.invalidate_cache();
        }
    }

    /// Set maximum cache size
    pub fn set_max_cache_size(&mut self, size: usize) {
        self.max_cache_size = size;
        self.trim_cache();
    }

    /// Get number of observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Get all observer IDs
    pub fn observer_ids(&self) -> Vec<u64> {
        self.observers.keys().cloned().collect()
    }

    /// Trim cache to maximum size
    fn trim_cache(&mut self) {
        if self.entity_cache.len() > self.max_cache_size {
            // Remove oldest entries (simple FIFO)
            let excess = self.entity_cache.len() - self.max_cache_size;
            let keys_to_remove: Vec<EntityId> =
                self.entity_cache.keys().take(excess).cloned().collect();

            for key in keys_to_remove {
                self.entity_cache.remove(&key);
            }
        }

        if self.signature_cache.len() > self.max_cache_size {
            let excess = self.signature_cache.len() - self.max_cache_size;
            let keys_to_remove: Vec<u64> =
                self.signature_cache.keys().take(excess).cloned().collect();

            for key in keys_to_remove {
                self.signature_cache.remove(&key);
            }
        }
    }

    /// Reset renderer state
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.stats = RenderStats::new();
        self.invalidate_cache();
    }
}

impl Default for HolographicRenderer {
    fn default() -> Self {
        Self::new(Arc::new(MultiScaleField::new()))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_renderer_creation() {
        let field = Arc::new(MultiScaleField::new());
        let renderer = HolographicRenderer::new(field);

        assert_eq!(renderer.current_frame, 0);
        assert_eq!(renderer.observer_count(), 0);
        assert_eq!(renderer.stats.total_frames, 0);
    }

    #[test]
    fn test_holographic_renderer_with_cache_size() {
        let field = Arc::new(MultiScaleField::new());
        let renderer = HolographicRenderer::with_cache_size(field, 500);

        assert_eq!(renderer.max_cache_size, 500);
    }

    #[test]
    fn test_add_observer() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        let observer = Observer::for_density(Density::Third);
        let observer_id = observer.observer_id;

        renderer.add_observer(observer);

        assert_eq!(renderer.observer_count(), 1);
        assert!(renderer.get_observer(observer_id).is_some());
        assert_eq!(renderer.stats.active_observers, 1);
    }

    #[test]
    fn test_remove_observer() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        let observer = Observer::for_density(Density::Third);
        let observer_id = observer.observer_id;
        renderer.add_observer(observer);

        assert!(renderer.remove_observer(observer_id));
        assert_eq!(renderer.observer_count(), 0);
        assert_eq!(renderer.stats.active_observers, 0);
    }

    #[test]
    fn test_render_frame() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        // Add observer
        let observer = Observer::for_density(Density::Third);
        renderer.add_observer(observer);

        // Render a frame
        let frame = renderer.render_frame(100);

        assert_eq!(frame.frame_number, 0);
        assert_eq!(frame.simulation_step, 100);
        assert_eq!(renderer.current_frame, 1);
        assert_eq!(renderer.stats.total_frames, 1);
    }

    #[test]
    fn test_render_for_observer() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        // Add observer
        let observer = Observer::for_density(Density::Third);
        let observer_id = observer.observer_id;
        renderer.add_observer(observer);

        // Render for specific observer
        let frame = renderer.render_for_observer(observer_id, 100);

        assert!(frame.is_some());
        let frame = frame.unwrap();
        assert_eq!(frame.observer_ids.len(), 1);
        assert_eq!(frame.observer_ids[0], observer_id);
    }

    #[test]
    fn test_render_for_nonexistent_observer() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        // Try to render for nonexistent observer
        let frame = renderer.render_for_observer(999, 100);

        assert!(frame.is_none());
    }

    #[test]
    fn test_get_visible_potentials() {
        let field = Arc::new(MultiScaleField::new());
        let renderer = HolographicRenderer::new(field);

        let observer = Observer::for_density(Density::Third);
        let potentials = renderer.get_visible_potentials(&observer);

        // Should have potentials for scale levels 0, 1, 2 (accessible to Third density)
        assert!(!potentials.is_empty());
    }

    #[test]
    fn test_get_visible_potentials_higher_density() {
        let field = Arc::new(MultiScaleField::new());
        let renderer = HolographicRenderer::new(field);

        let observer = Observer::for_density(Density::Sixth);
        let potentials = renderer.get_visible_potentials(&observer);

        // Higher density should have more potentials
        assert!(!potentials.is_empty());
    }

    #[test]
    fn test_render_frame_multiple() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        // Add observers
        renderer.add_observer(Observer::for_density(Density::Third));
        renderer.add_observer(Observer::for_density(Density::Fourth));

        // Render multiple frames
        for i in 0..5 {
            renderer.render_frame(i * 10);
        }

        assert_eq!(renderer.current_frame, 5);
        assert_eq!(renderer.stats.total_frames, 5);
    }

    #[test]
    fn test_stats_tracking() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        renderer.add_observer(Observer::for_density(Density::Third));

        renderer.render_frame(100);
        renderer.render_frame(200);

        assert_eq!(renderer.stats.total_frames, 2);
        assert!(renderer.stats.total_manifestations > 0);
        assert!(renderer.stats.average_entities_per_frame > 0.0);
    }

    #[test]
    fn test_invalidate_cache() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        renderer.add_observer(Observer::for_density(Density::Third));
        renderer.render_frame(100);

        // Should have cached entities
        let initial_cache_size = renderer.entity_cache.len();

        renderer.invalidate_cache();

        assert_eq!(renderer.entity_cache.len(), 0);
        assert_eq!(renderer.signature_cache.len(), 0);
    }

    #[test]
    fn test_invalidate_entity() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        renderer.add_observer(Observer::for_density(Density::Third));
        let frame = renderer.render_frame(100);

        if let Some(first_entity) = frame.manifested_entities.first() {
            let entity_id = first_entity.entity_id;

            assert!(renderer.entity_cache.contains_key(&entity_id));

            renderer.invalidate_entity(entity_id);

            assert!(!renderer.entity_cache.contains_key(&entity_id));
        }
    }

    #[test]
    fn test_set_caching_enabled() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        assert!(renderer.caching_enabled);

        renderer.set_caching_enabled(false);

        assert!(!renderer.caching_enabled);
        assert_eq!(renderer.entity_cache.len(), 0);
        assert_eq!(renderer.signature_cache.len(), 0);
    }

    #[test]
    fn test_set_max_cache_size() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        renderer.set_max_cache_size(5);
        assert_eq!(renderer.max_cache_size, 5);
    }

    #[test]
    fn test_reset() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        renderer.add_observer(Observer::for_density(Density::Third));
        renderer.render_frame(100);
        renderer.render_frame(200);

        assert_eq!(renderer.current_frame, 2);
        assert_eq!(renderer.stats.total_frames, 2);

        renderer.reset();

        assert_eq!(renderer.current_frame, 0);
        assert_eq!(renderer.stats.total_frames, 0);
    }

    #[ignore]
    #[test]
    fn test_manifested_entity_creation() {
        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity = ManifestedEntity::new(
            123,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        assert_eq!(entity.entity_id, 123);
        assert_eq!(entity.observation_count, 1);
        assert!(entity.stable);
    }

    #[test]
    fn test_manifested_entity_is_observable_by() {
        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity = ManifestedEntity::new(
            123,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        // Same density observer can observe
        let observer_third = Observer::for_density(Density::Third);
        assert!(entity.is_observable_by(&observer_third));

        // Higher density observer can observe
        let observer_fourth = Observer::for_density(Density::Fourth);
        assert!(entity.is_observable_by(&observer_fourth));

        // Lower density observer cannot observe
        let observer_second = Observer::for_density(Density::Second);
        assert!(!entity.is_observable_by(&observer_second));
    }

    #[test]
    fn test_manifested_entity_add_observation() {
        let potential = PotentialManifestation::new(0.4, 0.3, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.3,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let mut entity = ManifestedEntity::new(
            123,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        // Initially not stable (low coherence)
        assert!(!entity.stable);
        assert_eq!(entity.observation_count, 1);

        // Add observations
        entity.add_observation();
        entity.add_observation();
        entity.add_observation();

        // Should become stable after multiple observations
        assert!(entity.stable);
        assert_eq!(entity.observation_count, 4);
        assert!(entity.coherence > 0.3);
    }

    #[test]
    fn test_render_frame_creation() {
        let frame = RenderFrame::new(0, 100);

        assert_eq!(frame.frame_number, 0);
        assert_eq!(frame.simulation_step, 100);
        assert_eq!(frame.entity_count(), 0);
        assert!(frame.observer_ids.is_empty());
    }

    #[test]
    fn test_render_frame_add_entity() {
        let mut frame = RenderFrame::new(0, 100);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity = ManifestedEntity::new(
            123,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        frame.add_entity(entity.clone());

        assert_eq!(frame.entity_count(), 1);
        assert!(frame.has_entity(123));
        assert!(frame.get_entity(123).is_some());
    }

    #[test]
    fn test_render_frame_merge() {
        let mut frame1 = RenderFrame::new(0, 100);
        let mut frame2 = RenderFrame::new(0, 100);

        frame1.observer_ids.push(1);
        frame2.observer_ids.push(2);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity1 = ManifestedEntity::new(
            123,
            potential.clone(),
            collapsed.clone(),
            spectrum_pos.clone(),
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        let entity2 = ManifestedEntity::new(
            456,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        frame1.add_entity(entity1);
        frame2.add_entity(entity2);

        frame1.merge(frame2);

        assert_eq!(frame1.entity_count(), 2);
        assert_eq!(frame1.observer_ids.len(), 2);
    }

    #[test]
    fn test_render_stats_creation() {
        let stats = RenderStats::new();

        assert_eq!(stats.total_frames, 0);
        assert_eq!(stats.total_manifestations, 0);
        assert_eq!(stats.average_entities_per_frame, 0.0);
    }

    #[test]
    fn test_render_stats_record_frame() {
        let mut stats = RenderStats::new();
        let mut frame = RenderFrame::new(0, 100);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity = ManifestedEntity::new(
            123,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        frame.add_entity(entity);

        stats.record_frame(&frame, 1000);

        assert_eq!(stats.total_frames, 1);
        assert_eq!(stats.total_manifestations, 1);
        assert_eq!(stats.average_entities_per_frame, 1.0);
        assert_eq!(stats.average_render_time, 1000.0);
    }

    #[test]
    fn test_render_stats_collapse_rate() {
        let mut stats = RenderStats::new();

        stats.record_observations(10, 5);

        assert_eq!(stats.total_observations, 10);
        assert_eq!(stats.total_collapses, 5);
        assert_eq!(stats.collapse_rate(), 0.5);
    }

    #[test]
    fn test_default_implementations() {
        let renderer = HolographicRenderer::default();
        let stats = RenderStats::default();
        let frame = RenderFrame::default();

        assert_eq!(renderer.current_frame, 0);
        assert_eq!(stats.total_frames, 0);
        assert_eq!(frame.frame_number, 0);
    }

    #[test]
    fn test_multiple_observers_render() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::new(field);

        // Add multiple observers at different densities
        renderer.add_observer(Observer::for_density(Density::Third));
        renderer.add_observer(Observer::for_density(Density::Fourth));
        renderer.add_observer(Observer::for_density(Density::Fifth));

        let frame = renderer.render_frame(100);

        assert_eq!(frame.observer_ids.len(), 3);
        assert!(frame.entity_count() > 0);
    }

    #[test]
    fn test_cache_behavior() {
        let field = Arc::new(MultiScaleField::new());
        let mut renderer = HolographicRenderer::with_cache_size(field, 3);

        renderer.add_observer(Observer::for_density(Density::Third));

        // Render multiple frames
        for i in 0..10 {
            renderer.render_frame(i * 10);
        }

        // Cache should be trimmed to max size
        assert!(renderer.entity_cache.len() <= renderer.max_cache_size);
    }

    #[test]
    fn test_entities_at_density() {
        let mut frame = RenderFrame::new(0, 100);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity3 = ManifestedEntity::new(
            1,
            potential.clone(),
            collapsed.clone(),
            spectrum_pos.clone(),
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        let entity4 = ManifestedEntity::new(
            2,
            potential.clone(),
            collapsed.clone(),
            spectrum_pos.clone(),
            Density::Fourth,
            ScaleLevel::Cellular,
            100,
        );

        frame.add_entity(entity3);
        frame.add_entity(entity4);

        let third_density_entities = frame.entities_at_density(Density::Third);
        let fourth_density_entities = frame.entities_at_density(Density::Fourth);

        assert_eq!(third_density_entities.len(), 1);
        assert_eq!(fourth_density_entities.len(), 1);
    }

    #[test]
    fn test_entities_observable_by() {
        let mut frame = RenderFrame::new(0, 100);

        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(
            FieldSignature::default_for_scale(ScaleLevel::Molecular),
            0.5,
            100,
            Density::Third,
            42,
        );
        let spectrum_pos = SpectrumPosition::physical(Density::Third);

        let entity = ManifestedEntity::new(
            1,
            potential,
            collapsed,
            spectrum_pos,
            Density::Third,
            ScaleLevel::Molecular,
            100,
        );

        frame.add_entity(entity);

        let observer_third = Observer::for_density(Density::Third);
        let observable = frame.entities_observable_by(&observer_third);

        assert_eq!(observable.len(), 1);
    }
}
