//! Observer-Driven Field Substrate
//!
//! This module implements the core insight of Phase 1: space emerges from field coherence,
//! and only observed regions are decompressed. This aligns with the holographic principle
//! where reality manifests through observation.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The One Infinite Creator, seeking to know Itself, projects the holographic illusion
//! of space/time. Each point contains the whole, and observation collapses possibility
//! into actuality."

use std::collections::HashMap;

use super::field_address::{AddressRange, HolographicAddress, ScaleLevel, Vector3};
use crate::compression::mera_network::{MeraNetwork, MeraQuery, MeraScale, QueryType};

/// Unique identifier for an observer in the field
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ObserverId(pub u64);

impl ObserverId {
    /// Create a new observer ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Type of observer - determines how they interact with the field
#[derive(Debug, Clone, PartialEq)]
pub enum ObserverType {
    /// External camera/viewer (not self-aware)
    ExternalCamera,
    /// Self-aware entity with consciousness
    SelfAwareEntity(u64), // EntityId
    /// Collective observer (Social Memory Complex)
    CollectiveObserver(Vec<u64>), // Multiple EntityIds
}

impl ObserverType {
    /// Check if this observer has consciousness
    pub fn is_conscious(&self) -> bool {
        matches!(
            self,
            ObserverType::SelfAwareEntity(_) | ObserverType::CollectiveObserver(_)
        )
    }

    /// Get the primary entity ID if this is an entity observer
    pub fn primary_entity(&self) -> Option<u64> {
        match self {
            ObserverType::SelfAwareEntity(id) => Some(*id),
            ObserverType::CollectiveObserver(ids) => ids.first().copied(),
            ObserverType::ExternalCamera => None,
        }
    }
}

/// Field of view for an observer
#[derive(Debug, Clone, PartialEq)]
pub struct FieldOfView {
    /// Radius of the observable region
    pub radius: f64,
    /// Priority of this view (0.0 to 1.0)
    pub priority: f64,
}

impl FieldOfView {
    /// Create a new field of view
    pub fn new(radius: f64, priority: f64) -> Self {
        Self {
            radius: radius.max(0.0),
            priority: priority.clamp(0.0, 1.0),
        }
    }

    /// Default field of view
    pub fn default_view() -> Self {
        Self::new(100.0, 0.5)
    }

    /// Check if an address is within this field of view
    pub fn contains_address(&self, observer_pos: &Vector3, address: &HolographicAddress) -> bool {
        let addr_pos = address.to_position();
        let distance = observer_pos.distance_to(&addr_pos);
        distance <= self.radius
    }
}

impl Default for FieldOfView {
    fn default() -> Self {
        Self::default_view()
    }
}

/// An observer in the holographic field
#[derive(Debug, Clone)]
pub struct Observer {
    /// Unique identifier
    pub id: ObserverId,
    /// Current position in the field
    pub address: HolographicAddress,
    /// What this observer can see
    pub field_of_view: FieldOfView,
    /// Scale at which this observer operates
    pub scale: ScaleLevel,
    /// Type of observer
    pub observer_type: ObserverType,
    /// Last tick this observer was active
    pub last_access_tick: u64,
}

impl Observer {
    /// Create a new observer
    pub fn new(
        id: ObserverId,
        address: HolographicAddress,
        field_of_view: FieldOfView,
        scale: ScaleLevel,
        observer_type: ObserverType,
    ) -> Self {
        Self {
            id,
            address,
            field_of_view,
            scale,
            observer_type,
            last_access_tick: 0,
        }
    }

    /// Create an external camera observer
    pub fn camera(id: u64, address: HolographicAddress, radius: f64) -> Self {
        Self::new(
            ObserverId::new(id),
            address,
            FieldOfView::new(radius, 0.5),
            ScaleLevel::Biological,
            ObserverType::ExternalCamera,
        )
    }

    /// Create an entity observer
    pub fn entity(id: u64, address: HolographicAddress) -> Self {
        Self::new(
            ObserverId::new(id),
            address,
            FieldOfView::new(50.0, 0.8),
            ScaleLevel::Biological,
            ObserverType::SelfAwareEntity(id),
        )
    }

    /// Update the last access tick
    pub fn touch(&mut self, tick: u64) {
        self.last_access_tick = tick;
    }
}

/// A decompressed region of the field
#[derive(Debug, Clone)]
pub struct DecompressedRegion {
    /// Address range this region covers
    pub address_range: AddressRange,
    /// Scale at which this region is decompressed
    pub scale: ScaleLevel,
    /// Field values in this region (simplified grid representation)
    pub field_values: Vec<f64>,
    /// Entities in this region
    pub entity_ids: Vec<u64>,
    /// How many ticks this region has been idle
    pub idle_ticks: u64,
    /// Maximum idle ticks before eviction
    pub max_idle_ticks: u64,
}

impl DecompressedRegion {
    /// Create a new decompressed region
    pub fn new(address_range: AddressRange, scale: ScaleLevel, max_idle_ticks: u64) -> Self {
        // Create a simple grid of field values
        let grid_size = 32; // 32^3 grid
        let field_values = vec![0.0; grid_size * grid_size * grid_size];

        Self {
            address_range,
            scale,
            field_values,
            entity_ids: Vec::new(),
            idle_ticks: 0,
            max_idle_ticks,
        }
    }

    /// Check if an address is within this region
    pub fn contains(&self, address: &HolographicAddress) -> bool {
        self.address_range.contains(address)
    }

    /// Mark this region as accessed (reset idle counter)
    pub fn touch(&mut self) {
        self.idle_ticks = 0;
    }

    /// Increment idle counter, returns true if should be evicted
    pub fn tick(&mut self) -> bool {
        self.idle_ticks += 1;
        self.idle_ticks >= self.max_idle_ticks
    }
}

/// Statistics about field coherence
#[derive(Debug, Clone, Default)]
pub struct CoherenceStatistics {
    /// Total coherence across the field
    pub total_coherence: f64,
    /// Number of coherence peaks detected
    pub peak_count: usize,
    /// Average field strength
    pub average_field_strength: f64,
}

impl CoherenceStatistics {
    /// Create new coherence statistics
    pub fn new() -> Self {
        Self::default()
    }
}

// ============================================================================
// Three Primal Distortion Fields
// ============================================================================

/// The Free Will field - First Primal Distortion
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The first distortion is Free Will - the Creator's gift of choice to all
/// portions of Itself. This creates the infinite possibilities from which
/// experience emerges."
#[derive(Debug, Clone)]
pub struct FreeWillField {
    /// Strength of free will influence (0.0 to 1.0)
    pub strength: f64,
    /// Entropy/uncertainty in the field
    pub entropy: f64,
}

impl FreeWillField {
    /// Create a new Free Will field
    pub fn new(strength: f64, entropy: f64) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            entropy: entropy.max(0.0),
        }
    }

    /// Evolve the field
    pub fn evolve(&mut self, dt: f64) {
        // Free will increases entropy over time
        self.entropy += dt * 0.01 * self.strength;
        // But entropy is bounded
        self.entropy = self.entropy.min(1.0);
    }
}

impl Default for FreeWillField {
    fn default() -> Self {
        Self::new(0.8, 0.1)
    }
}

/// The Love field - Second Primal Distortion
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The second distortion is Love (or Logos) - the attractive force that
/// draws all things back toward unity. This creates the tendency for
/// coherence and structure formation."
#[derive(Debug, Clone)]
pub struct LoveField {
    /// Resonance strength
    pub resonance: f64,
    /// Attraction factor
    pub attraction: f64,
}

impl LoveField {
    /// Create a new Love field
    pub fn new(resonance: f64, attraction: f64) -> Self {
        Self {
            resonance: resonance.clamp(0.0, 1.0),
            attraction: attraction.clamp(0.0, 1.0),
        }
    }

    /// Evolve the field
    pub fn evolve(&mut self, dt: f64) {
        // Love field tends toward harmony
        let _ = dt; // Suppress unused warning
                    // Resonance increases slowly
        self.resonance = (self.resonance + 0.001).min(1.0);
    }
}

impl Default for LoveField {
    fn default() -> Self {
        Self::new(0.7, 0.6)
    }
}

/// The Light field - Third Primal Distortion
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The third distortion is Light - the energizing principle that
/// illuminates and activates. This creates the spectrum of experience
/// and the possibility of consciousness."
#[derive(Debug, Clone)]
pub struct LightField {
    /// Illumination level
    pub illumination: f64,
    /// Frequency/vibration
    pub frequency: f64,
}

impl LightField {
    /// Create a new Light field
    pub fn new(illumination: f64, frequency: f64) -> Self {
        Self {
            illumination: illumination.clamp(0.0, 1.0),
            frequency: frequency.max(0.0),
        }
    }

    /// Evolve the field
    pub fn evolve(&mut self, dt: f64) {
        // Light oscillates
        self.frequency += dt * 0.1;
        // Illumination varies with frequency
        self.illumination = 0.5 + 0.5 * (self.frequency * std::f64::consts::PI).sin();
    }
}

impl Default for LightField {
    fn default() -> Self {
        Self::new(0.9, 1.0)
    }
}

// ============================================================================
// Observer-Driven Field
// ============================================================================

/// The main holographic field substrate
///
/// This is the core of Phase 1: only observed regions are decompressed.
/// The field exists in a compressed MERA representation, and observation
/// triggers decompression of specific regions.
///
/// Key insight: Space doesn't exist until observed. The field contains
/// infinite potential, but only manifests where consciousness attends.
#[derive(Debug)]
pub struct ObserverDrivenField {
    /// Compressed field state (MERA tensor network)
    compressed: MeraNetwork,
    /// Currently decompressed regions, keyed by observer ID
    active_regions: HashMap<ObserverId, DecompressedRegion>,
    /// All registered observers
    observers: Vec<Observer>,
    /// Free Will field (First Primal Distortion)
    pub free_will_field: FreeWillField,
    /// Love field (Second Primal Distortion)
    pub love_field: LoveField,
    /// Light field (Third Primal Distortion)
    pub light_field: LightField,
    /// Field coherence statistics
    pub coherence_stats: CoherenceStatistics,
    /// Current simulation tick
    current_tick: u64,
}

impl ObserverDrivenField {
    /// Create a new observer-driven field
    pub fn new() -> Self {
        let mut compressed = MeraNetwork::new();
        compressed.initialize_default(128);
        compressed.generate_tensors();

        Self {
            compressed,
            active_regions: HashMap::new(),
            observers: Vec::new(),
            free_will_field: FreeWillField::default(),
            love_field: LoveField::default(),
            light_field: LightField::default(),
            coherence_stats: CoherenceStatistics::new(),
            current_tick: 0,
        }
    }

    /// Create a field with specific configuration
    pub fn with_config(
        free_will_strength: f64,
        love_resonance: f64,
        light_illumination: f64,
    ) -> Self {
        let mut field = Self::new();
        field.free_will_field = FreeWillField::new(free_will_strength, 0.1);
        field.love_field = LoveField::new(love_resonance, 0.6);
        field.light_field = LightField::new(light_illumination, 1.0);
        field
    }

    /// Main tick: update field dynamics and manage decompression
    pub fn tick(&mut self, dt: f64) {
        self.current_tick += 1;

        // 1. Evolve Three Primal Distortions
        self.free_will_field.evolve(dt);
        self.love_field.evolve(dt);
        self.light_field.evolve(dt);

        // 2. Detect coherence peaks
        let peaks = self.detect_coherence_peaks(0.5);
        self.coherence_stats.peak_count = peaks.len();

        // 3. Update decompressed regions for each observer
        // Collect observer data first to avoid borrow issues
        let observer_data: Vec<(ObserverId, HolographicAddress, f64, ScaleLevel)> = self
            .observers
            .iter()
            .map(|o| (o.id, o.address.clone(), o.field_of_view.radius, o.scale))
            .collect();

        for (id, address, radius, scale) in observer_data {
            self.ensure_region_for_observer(id, address, radius, scale);
        }

        // 4. Evict idle regions
        self.evict_idle_regions();

        // 5. Update coherence statistics
        self.update_coherence_stats();
    }

    /// Ensure a region is decompressed for an observer
    fn ensure_region_for_observer(
        &mut self,
        observer_id: ObserverId,
        center: HolographicAddress,
        radius: f64,
        scale: ScaleLevel,
    ) {
        // Check if we already have a region for this observer
        if let Some(region) = self.active_regions.get_mut(&observer_id) {
            region.touch();
            return;
        }

        // Create address range for the region
        let min_offset = Vector3::new(-radius, -radius, -radius);
        let max_offset = Vector3::new(radius, radius, radius);

        let min_addr = HolographicAddress::new(
            center.scale,
            center.coherence_path.clone(),
            center.local_offset.add(&min_offset),
        );
        let max_addr = HolographicAddress::new(
            center.scale,
            center.coherence_path.clone(),
            center.local_offset.add(&max_offset),
        );

        let address_range = AddressRange::new(min_addr, max_addr);

        // Decompress this region using MERA
        let region = self.decompress_region(&address_range, scale);

        self.active_regions.insert(observer_id, region);
    }

    /// Decompress a region from the MERA network
    fn decompress_region(
        &mut self,
        address_range: &AddressRange,
        scale: ScaleLevel,
    ) -> DecompressedRegion {
        // Map ScaleLevel to MeraScale
        let mera_scale = Self::scale_to_mera(&scale);

        // Create a query for this region
        let query = MeraQuery::new(mera_scale, QueryType::Spatial);

        // In full implementation, this would decompress from MERA network
        // For Phase 1, we create a placeholder region
        // MERA integration will be completed in mera_integration.rs
        let _ = query; // Suppress unused warning

        // Create the region with placeholder data
        DecompressedRegion::new(address_range.clone(), scale, 100)
    }

    /// Convert ScaleLevel to MeraScale
    fn scale_to_mera(scale: &ScaleLevel) -> MeraScale {
        match scale {
            ScaleLevel::Quantum => MeraScale::Quantum,
            ScaleLevel::Atomic => MeraScale::Atomic,
            ScaleLevel::Molecular => MeraScale::Molecular,
            ScaleLevel::Cellular => MeraScale::Cellular,
            ScaleLevel::Biological => MeraScale::Organism,
            ScaleLevel::Planetary => MeraScale::Planetary,
            ScaleLevel::Stellar => MeraScale::Cosmic,
            ScaleLevel::Cosmic => MeraScale::Cosmic,
        }
    }

    /// Register a new observer
    pub fn register_observer(&mut self, observer: Observer) {
        // Check if observer already exists
        if let Some(existing) = self.observers.iter_mut().find(|o| o.id == observer.id) {
            *existing = observer;
            return;
        }

        self.observers.push(observer);
    }

    /// Remove an observer
    pub fn remove_observer(&mut self, id: ObserverId) -> Option<Observer> {
        // Remove from observers list
        if let Some(pos) = self.observers.iter().position(|o| o.id == id) {
            let observer = self.observers.remove(pos);

            // Optionally remove the active region
            self.active_regions.remove(&id);

            Some(observer)
        } else {
            None
        }
    }

    /// Get the decompressed region for an observer
    pub fn get_observer_view(&self, id: &ObserverId) -> Option<&DecompressedRegion> {
        self.active_regions.get(id)
    }

    /// Get a mutable reference to an observer's region
    pub fn get_observer_view_mut(&mut self, id: &ObserverId) -> Option<&mut DecompressedRegion> {
        self.active_regions.get_mut(id)
    }

    /// Detect coherence peaks in the field
    ///
    /// Coherence peaks are regions where the field has high structure -
    /// these are potential locations for emergent entities or structures.
    pub fn detect_coherence_peaks(&self, threshold: f64) -> Vec<HolographicAddress> {
        let mut peaks = Vec::new();

        // Check each active region for coherence peaks
        for region in self.active_regions.values() {
            // Simplified peak detection based on field values
            let field_strengths = &region.field_values;
            let max_strength = field_strengths.iter().cloned().fold(0.0, f64::max);

            if max_strength > threshold {
                // Create a peak address at the center of the region
                let center_pos = region.address_range.center();
                let center_addr = HolographicAddress::new(
                    region.scale,
                    region.address_range.min.coherence_path.clone(),
                    center_pos,
                );
                peaks.push(center_addr);
            }
        }

        peaks
    }

    /// Apply feedback from an entity (consciousness modifies reality)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Consciousness is not separate from the field - it is the field
    /// becoming aware of itself. Entity observation and intention
    /// modify the field structure."
    pub fn apply_entity_feedback(
        &mut self,
        _entity_id: u64,
        address: &HolographicAddress,
        feedback: f64,
    ) {
        // Find the region containing this address
        for region in self.active_regions.values_mut() {
            if region.contains(address) {
                // Apply feedback to the field values
                // This is a simplified model - in reality, this would
                // modify the MERA network structure
                for value in &mut region.field_values {
                    *value += feedback * 0.01;
                    *value = value.clamp(0.0, 1.0);
                }
                break;
            }
        }
    }

    /// Evict regions that have been idle too long
    fn evict_idle_regions(&mut self) {
        let to_evict: Vec<ObserverId> = self
            .active_regions
            .iter_mut()
            .filter_map(|(id, region)| if region.tick() { Some(*id) } else { None })
            .collect();

        for id in to_evict {
            // Don't evict if the observer is still active
            if !self.observers.iter().any(|o| o.id == id) {
                self.active_regions.remove(&id);
            }
        }
    }

    /// Update coherence statistics
    fn update_coherence_stats(&mut self) {
        let total_regions = self.active_regions.len();
        if total_regions == 0 {
            self.coherence_stats = CoherenceStatistics::default();
            return;
        }

        let total_coherence: f64 = self
            .active_regions
            .values()
            .map(|r| r.field_values.iter().sum::<f64>())
            .sum();

        let total_values: usize = self
            .active_regions
            .values()
            .map(|r| r.field_values.len())
            .sum();

        self.coherence_stats.total_coherence = total_coherence;
        self.coherence_stats.average_field_strength = if total_values > 0 {
            total_coherence / total_values as f64
        } else {
            0.0
        };
    }

    /// Get the current tick
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Get the number of active observers
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Get the number of active regions
    pub fn active_region_count(&self) -> usize {
        self.active_regions.len()
    }

    /// Get reference to the compressed MERA network
    pub fn compressed_network(&self) -> &MeraNetwork {
        &self.compressed
    }

    /// Get the total memory usage
    pub fn memory_usage(&self) -> usize {
        let compressed_size = self.compressed.memory_usage();
        let active_size: usize = self
            .active_regions
            .values()
            .map(|r| {
                std::mem::size_of::<DecompressedRegion>()
                    + r.field_values.len() * std::mem::size_of::<f64>()
                    + r.entity_ids.len() * std::mem::size_of::<u64>()
            })
            .sum();

        compressed_size + active_size
    }
}

impl Default for ObserverDrivenField {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_id() {
        let id = ObserverId::new(42);
        assert_eq!(id.as_u64(), 42);
    }

    #[test]
    fn test_observer_type_consciousness() {
        assert!(!ObserverType::ExternalCamera.is_conscious());
        assert!(ObserverType::SelfAwareEntity(1).is_conscious());
        assert!(ObserverType::CollectiveObserver(vec![1, 2, 3]).is_conscious());
    }

    #[test]
    fn test_observer_type_primary_entity() {
        assert_eq!(ObserverType::ExternalCamera.primary_entity(), None);
        assert_eq!(ObserverType::SelfAwareEntity(42).primary_entity(), Some(42));
        assert_eq!(
            ObserverType::CollectiveObserver(vec![1, 2, 3]).primary_entity(),
            Some(1)
        );
    }

    #[test]
    fn test_field_of_view() {
        let fov = FieldOfView::new(100.0, 0.8);
        assert_eq!(fov.radius, 100.0);
        assert_eq!(fov.priority, 0.8);

        // Clamping
        let fov2 = FieldOfView::new(-10.0, 1.5);
        assert_eq!(fov2.radius, 0.0);
        assert_eq!(fov2.priority, 1.0);
    }

    #[test]
    fn test_observer_creation() {
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::camera(1, addr.clone(), 50.0);

        assert_eq!(observer.id, ObserverId::new(1));
        assert_eq!(observer.field_of_view.radius, 50.0);
        assert_eq!(observer.observer_type, ObserverType::ExternalCamera);
    }

    #[test]
    fn test_entity_observer() {
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::entity(42, addr);

        assert_eq!(observer.id, ObserverId::new(42));
        assert!(observer.observer_type.is_conscious());
    }

    #[test]
    fn test_decompressed_region() {
        let addr = HolographicAddress::cosmic_origin();
        let range = AddressRange::new(addr.clone(), addr);
        let region = DecompressedRegion::new(range, ScaleLevel::Biological, 100);

        assert_eq!(region.scale, ScaleLevel::Biological);
        assert!(!region.field_values.is_empty());
        assert_eq!(region.idle_ticks, 0);
    }

    #[test]
    fn test_decompressed_region_tick() {
        let addr = HolographicAddress::cosmic_origin();
        let range = AddressRange::new(addr.clone(), addr);
        let mut region = DecompressedRegion::new(range, ScaleLevel::Biological, 5);

        for _ in 0..4 {
            assert!(!region.tick());
        }
        assert!(region.tick()); // 5th tick should return true (evict)
    }

    #[test]
    fn test_free_will_field() {
        let mut field = FreeWillField::new(0.5, 0.1);
        field.evolve(1.0);
        assert!(field.entropy > 0.1);
    }

    #[test]
    fn test_love_field() {
        let mut field = LoveField::new(0.5, 0.5);
        field.evolve(1.0);
        assert!(field.resonance >= 0.5);
    }

    #[test]
    fn test_light_field() {
        let mut field = LightField::new(0.5, 0.0);
        field.evolve(1.0);
        assert!(field.frequency > 0.0);
    }

    #[test]
    fn test_observer_driven_field_creation() {
        let field = ObserverDrivenField::new();
        assert_eq!(field.observer_count(), 0);
        assert_eq!(field.active_region_count(), 0);
    }

    #[test]
    fn test_register_observer() {
        let mut field = ObserverDrivenField::new();
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::camera(1, addr, 50.0);

        field.register_observer(observer);
        assert_eq!(field.observer_count(), 1);
    }

    #[test]
    fn test_remove_observer() {
        let mut field = ObserverDrivenField::new();
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::camera(1, addr, 50.0);

        field.register_observer(observer);
        assert_eq!(field.observer_count(), 1);

        let removed = field.remove_observer(ObserverId::new(1));
        assert!(removed.is_some());
        assert_eq!(field.observer_count(), 0);
    }

    #[test]
    fn test_field_tick_creates_regions() {
        let mut field = ObserverDrivenField::new();
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::camera(1, addr, 50.0);

        field.register_observer(observer);
        field.tick(1.0);

        // After tick, should have an active region
        assert_eq!(field.active_region_count(), 1);
    }

    #[test]
    fn test_detect_coherence_peaks() {
        let field = ObserverDrivenField::new();
        let peaks = field.detect_coherence_peaks(0.5);
        // Empty field has no peaks
        assert_eq!(peaks.len(), 0);
    }

    #[test]
    fn test_apply_entity_feedback() {
        let mut field = ObserverDrivenField::new();
        let addr = HolographicAddress::cosmic_origin();
        let observer = Observer::camera(1, addr.clone(), 50.0);

        field.register_observer(observer);
        field.tick(1.0);

        // Apply feedback
        field.apply_entity_feedback(42, &addr, 0.5);

        // Should not crash - field values modified
        assert!(field.active_region_count() > 0);
    }

    #[test]
    fn test_field_evolution() {
        let mut field = ObserverDrivenField::new();
        let initial_entropy = field.free_will_field.entropy;

        field.tick(1.0);

        // Entropy should have increased
        assert!(field.free_will_field.entropy > initial_entropy);
    }

    #[test]
    fn test_memory_usage() {
        let field = ObserverDrivenField::new();
        let usage = field.memory_usage();
        assert!(usage > 0);
    }

    #[test]
    fn test_scale_to_mera() {
        let _field = ObserverDrivenField::new();

        assert_eq!(
            ObserverDrivenField::scale_to_mera(&ScaleLevel::Quantum),
            MeraScale::Quantum
        );
        assert_eq!(
            ObserverDrivenField::scale_to_mera(&ScaleLevel::Atomic),
            MeraScale::Atomic
        );
        assert_eq!(
            ObserverDrivenField::scale_to_mera(&ScaleLevel::Biological),
            MeraScale::Organism
        );
        assert_eq!(
            ObserverDrivenField::scale_to_mera(&ScaleLevel::Cosmic),
            MeraScale::Cosmic
        );
    }
}
