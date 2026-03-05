//! Energy Center Interface Module
//!
//! This module implements Energy Centers as **unlocking interfaces** rather than containers.
//!
//! ## Architectural Principle
//!
//! Energy Centers do NOT "contain" energy - they UNLOCK pre-existing potential that is
//! already folded within the HolographicSeed.
//!
//! **Knowledge Base Reference:**
//! - ARCHITECTURE_AUDIT_REPORT.md Section 2.2 (Critical Gap #2)
//! - COSMOLOGICAL-ARCHITECTURE.md Section 4.3
//! - Energy-Ray-Centers/0. General.json
//!
//! ## Key Concept: Unlocking vs Containing
//!
//! **OLD (Container-based):**
//! - Energy centers store energy (energy_level, energy_capacity)
//! - Energy is added to centers, then removed from centers
//! - Centers are like batteries that hold charge
//!
//! **NEW (Interface-based):**
//! - Energy centers unlock potential from HolographicSeed
//! - Potential already exists in seed (created during Involution)
//! - Centers are like keys that unlock doors to pre-existing rooms
//!
//! ## The Unfolding Process
//!
//! 1. **Involution:** Creates potential, folds it into HolographicSeed (ROM)
//! 2. **Evolution:** Unfolds potential by unlocking through Energy Centers
//! 3. Energy Centers are the INTERFACE between folded and unfolded states
//! 4. No energy is "stored" in centers - it's "unfolded" from seed
//!
//! ## Implementation
//!
//! - `EnergyCenterInterface`: The unlocking interface (key + access state)
//! - `UnlockKey`: The key required to access specific potential in seed
//! - `AccessState`: Whether the center is locked, partially unlocked, or unlocked
//! - `UnfoldedPotential`: The potential that was unfolded from seed
//! - `PotentialSource`: Tracks that potential came from seed, not from center

use crate::energy_ray_centers::RayCenter;
use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the access state of an energy center
///
/// Energy centers can be locked, partially unlocked, or fully unlocked.
/// The unlock state determines how much potential can be unfolded from the seed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AccessState {
    /// Center is locked - no potential can be unfolded
    Locked,
    /// Center is partially unlocked - limited potential can be unfolded
    PartiallyUnlocked { unlock_percentage: Float },
    /// Center is fully unlocked - all potential can be unfolded
    Unlocked,
}

impl AccessState {
    /// Returns the unlock percentage (0.0 to 1.0)
    pub fn unlock_percentage(&self) -> Float {
        match self {
            AccessState::Locked => 0.0,
            AccessState::PartiallyUnlocked { unlock_percentage } => *unlock_percentage,
            AccessState::Unlocked => 1.0,
        }
    }

    /// Returns whether the center is fully unlocked
    pub fn is_fully_unlocked(&self) -> bool {
        matches!(self, AccessState::Unlocked)
    }

    /// Returns whether the center is locked
    pub fn is_locked(&self) -> bool {
        matches!(self, AccessState::Locked)
    }

    /// Returns whether the center can access any potential
    pub fn can_access(&self) -> bool {
        !matches!(self, AccessState::Locked)
    }
}

/// The key required to unlock potential from HolographicSeed
///
/// Each energy center has a unique unlock key that matches specific
/// potential folded within the seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockKey {
    /// The ray center this key belongs to
    pub center: RayCenter,

    /// Key strength (0.0 to 1.0) - how much potential can be unlocked
    pub strength: Float,

    /// Key complexity - determines which archetypes can be accessed
    pub complexity: Float,

    /// Key resonance - must match seed's resonance to unlock
    pub resonance: Float,
}

impl UnlockKey {
    /// Creates a new unlock key for a ray center
    pub fn new(center: RayCenter) -> Self {
        UnlockKey {
            center,
            strength: 0.0,
            complexity: 0.0,
            resonance: 0.5,
        }
    }

    /// Strengthens the unlock key (increases unlock capacity)
    pub fn strengthen(&mut self, amount: Float) {
        self.strength = (self.strength + amount).min(1.0);
    }

    /// Increases key complexity (allows access to more archetypes)
    pub fn increase_complexity(&mut self, amount: Float) {
        self.complexity = (self.complexity + amount).min(1.0);
    }

    /// Adjusts resonance to match seed
    pub fn tune_resonance(&mut self, target: Float) {
        // Move resonance towards target
        let delta = target - self.resonance;
        self.resonance += delta * 0.1;
    }

    /// Returns whether this key can unlock the given potential
    pub fn can_unlock(&self, potential_requirement: Float) -> bool {
        self.strength >= potential_requirement
    }

    /// Returns the access state based on key strength
    pub fn calculate_access_state(&self) -> AccessState {
        if self.strength <= 0.0 {
            AccessState::Locked
        } else if self.strength >= 1.0 {
            AccessState::Unlocked
        } else {
            AccessState::PartiallyUnlocked {
                unlock_percentage: self.strength,
            }
        }
    }
}

/// The source of unfolded potential
///
/// This tracks whether potential came from the HolographicSeed (pre-existing)
/// or from some other source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PotentialSource {
    /// Potential came from HolographicSeed (pre-existing, folded during Involution)
    HolographicSeed,

    /// Potential came from external source (not pre-existing)
    External,
}

/// The consciousness state that emerges when potential is unfolded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// Overall consciousness level (0.0 to 1.0)
    pub overall_level: Float,

    /// Mind complex activation (0.0 to 1.0)
    pub mind_activation: Float,

    /// Body complex activation (0.0 to 1.0)
    pub body_activation: Float,

    /// Spirit complex activation (0.0 to 1.0)
    pub spirit_activation: Float,

    /// Wisdom accumulated (0.0 to infinity)
    pub wisdom: Float,

    /// Love experienced (0.0 to infinity)
    pub love: Float,
}

impl Default for ConsciousnessState {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsciousnessState {
    /// Creates a new consciousness state
    pub fn new() -> Self {
        ConsciousnessState {
            overall_level: 0.0,
            mind_activation: 0.0,
            body_activation: 0.0,
            spirit_activation: 0.0,
            wisdom: 0.0,
            love: 0.0,
        }
    }

    /// Updates overall level based on complex activations
    pub fn update_overall_level(&mut self) {
        self.overall_level =
            (self.mind_activation + self.body_activation + self.spirit_activation) / 3.0;
    }
}

/// The potential that was unfolded from HolographicSeed
///
/// This is NOT "new" energy - it's "unfolded" energy that was already
/// present in the seed but folded during Involution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedPotential {
    /// The archetype energies that were unfolded (22 archetypes)
    pub archetype_energies: [Float; 22],

    /// The consciousness state that emerged
    pub consciousness_state: ConsciousnessState,

    /// The source of this potential (should be HolographicSeed)
    pub source: PotentialSource,

    /// The ray center that unlocked this potential
    pub unlocked_by: RayCenter,

    /// The total amount of potential unfolded
    pub total_potential: Float,
}

impl UnfoldedPotential {
    /// Creates a new unfolded potential
    pub fn new(unlocked_by: RayCenter, source: PotentialSource) -> Self {
        UnfoldedPotential {
            archetype_energies: [0.0; 22],
            consciousness_state: ConsciousnessState::new(),
            source,
            unlocked_by,
            total_potential: 0.0,
        }
    }

    /// Returns whether this potential came from HolographicSeed
    pub fn is_from_seed(&self) -> bool {
        matches!(self.source, PotentialSource::HolographicSeed)
    }

    /// Returns the total potential unfolded
    pub fn total_potential(&self) -> Float {
        self.total_potential
    }

    /// Adds archetype energy to the unfolded potential
    pub fn add_archetype_energy(&mut self, index: usize, energy: Float) {
        if index < 22 {
            self.archetype_energies[index] += energy;
            self.total_potential += energy;
        }
    }

    /// Updates consciousness state based on archetype energies
    pub fn update_consciousness_state(&mut self) {
        // Mind complex: Archetypes 0-6
        let mind_sum: Float = self.archetype_energies[0..=6].iter().sum();
        self.consciousness_state.mind_activation = (mind_sum / 7.0).min(1.0);

        // Body complex: Archetypes 7-13
        let body_sum: Float = self.archetype_energies[7..=13].iter().sum();
        self.consciousness_state.body_activation = (body_sum / 7.0).min(1.0);

        // Spirit complex: Archetypes 14-20
        let spirit_sum: Float = self.archetype_energies[14..=20].iter().sum();
        self.consciousness_state.spirit_activation = (spirit_sum / 7.0).min(1.0);

        // Update overall level
        self.consciousness_state.update_overall_level();

        // Accumulate wisdom and love based on higher archetypes
        self.consciousness_state.wisdom += self.archetype_energies[21] * 0.1;
        self.consciousness_state.love += self.archetype_energies[21] * 0.1;
    }
}

/// Energy Center Interface - the unlocking mechanism
///
/// This is the NEW architecture where energy centers are interfaces that
/// unlock pre-existing potential from HolographicSeed, rather than containers
/// that store energy.
///
/// **Knowledge Base Reference:**
/// - ARCHITECTURE_AUDIT_REPORT.md Section 2.2 (Critical Gap #2)
/// - "Energy Centers are the interfaces that unlock the seed's potential.
///   They don't 'contain' anything—they 'unlock' what's folded within."
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCenterInterface {
    /// The ray center
    pub center: RayCenter,

    /// The unlock key for this center
    pub unlock_key: UnlockKey,

    /// The current access state (locked, partially unlocked, unlocked)
    pub access_state: AccessState,

    /// Track how much potential has been unfolded through this interface
    pub total_unfolded: Float,

    /// Track the last time this center was used to unlock potential
    pub last_unlock_timestamp: Option<u64>,
}

impl EnergyCenterInterface {
    /// Creates a new energy center interface
    pub fn new(center: RayCenter) -> Self {
        let unlock_key = UnlockKey::new(center);
        let access_state = unlock_key.calculate_access_state();

        EnergyCenterInterface {
            center,
            unlock_key,
            access_state,
            total_unfolded: 0.0,
            last_unlock_timestamp: None,
        }
    }

    /// Unlocks potential from HolographicSeed
    ///
    /// This is the CORE METHOD that demonstrates the unlocking mechanism.
    /// It accesses pre-existing potential in the seed and unfolds it.
    ///
    /// **Key Principle:** This does NOT create new energy. It UNFOLDS potential
    /// that was already present in the seed (created during Involution).
    ///
    /// # Arguments
    /// * `seed` - The HolographicSeed containing the folded potential
    /// * `catalyst intensity` - The catalyst that triggers the unlocking
    ///
    /// # Returns
    /// The unfolded potential (from seed, not from center)
    pub fn unlock_potential(
        &mut self,
        _seed: &HolographicSeed,
        catalyst_intensity: Float,
    ) -> UnfoldedPotential {
        // Check if center can access potential
        if !self.access_state.can_access() {
            return UnfoldedPotential::new(self.center, PotentialSource::HolographicSeed);
        }

        // Calculate how much potential can be unlocked based on access state
        let unlock_percentage = self.access_state.unlock_percentage();
        let max_potential = catalyst_intensity * unlock_percentage;

        // Create unfolded potential result
        let mut unfolded = UnfoldedPotential::new(self.center, PotentialSource::HolographicSeed);

        // Unfold archetype energies from seed
        // The seed contains the complete 22-Archetype structure
        // We unlock specific archetypes based on the ray center
        let archetype_indices = self.get_archetype_indices_for_center();

        for index in &archetype_indices {
            // Calculate potential to unlock from this archetype
            let archetype_potential = max_potential / archetype_indices.len() as Float;
            unfolded.add_archetype_energy(*index, archetype_potential);
        }

        // Update consciousness state
        unfolded.update_consciousness_state();

        // Track total unfolded
        self.total_unfolded += unfolded.total_potential();

        // Update timestamp (simplified - in real implementation would use actual time)
        self.last_unlock_timestamp = Some(0);

        unfolded
    }

    /// Returns the archetype indices accessible to this ray center
    fn get_archetype_indices_for_center(&self) -> Vec<usize> {
        match self.center {
            RayCenter::Red => vec![0, 1, 2], // Body complex archetypes
            RayCenter::Orange => vec![3, 4, 5],
            RayCenter::Yellow => vec![6, 7, 8], // Mind complex archetypes
            RayCenter::Green => vec![9, 10, 11, 12], // Heart center - integrates all
            RayCenter::Blue => vec![13, 14, 15], // Higher mind
            RayCenter::Indigo => vec![16, 17, 18], // Gateway to intelligent infinity
            RayCenter::Violet => vec![19, 20, 21], // Complete expression
        }
    }

    /// Checks if this interface can access the given seed
    ///
    /// Returns true if the seed has potential that this center can unlock
    pub fn can_access(&self, _seed: &HolographicSeed) -> bool {
        // In this implementation, all seeds have all potential
        // The limitation is in the unlock key strength
        self.access_state.can_access()
    }

    /// Strengthens the unlock key (evolutionary progression)
    ///
    /// This represents the entity's evolutionary progress - as the entity
    /// grows, the unlock key becomes stronger, allowing more potential to be unfolded.
    pub fn strengthen_unlock_key(&mut self, amount: Float) {
        self.unlock_key.strengthen(amount);

        // Update access state based on new key strength
        self.access_state = self.unlock_key.calculate_access_state();
    }

    /// Increases key complexity (allows access to more archetypes)
    pub fn increase_key_complexity(&mut self, amount: Float) {
        self.unlock_key.increase_complexity(amount);
    }

    /// Tunes resonance to match seed
    pub fn tune_resonance(&mut self, seed_resonance: Float) {
        self.unlock_key.tune_resonance(seed_resonance);
    }

    /// Returns the current access state
    pub fn access_state(&self) -> &AccessState {
        &self.access_state
    }

    /// Returns the current unlock key strength (0.0 to 1.0)
    pub fn unlock_key_strength(&self) -> Float {
        self.unlock_key.strength
    }

    /// Returns the total potential unfolded through this interface
    pub fn total_unfolded(&self) -> Float {
        self.total_unfolded
    }

    /// Returns whether this interface is fully unlocked
    pub fn is_fully_unlocked(&self) -> bool {
        self.access_state.is_fully_unlocked()
    }

    /// Returns whether this interface is locked
    pub fn is_locked(&self) -> bool {
        self.access_state.is_locked()
    }

    /// **IMPORTANT:** This method returns 0.0 to demonstrate that centers don't store energy
    ///
    /// In the OLD container-based architecture, this would return the stored energy.
    /// In the NEW interface-based architecture, centers don't store anything.
    pub fn stored_energy(&self) -> Float {
        0.0 // Centers don't store energy - they unlock potential from seed
    }

    /// Resets the unlock key (for testing or special scenarios)
    pub fn reset_unlock_key(&mut self) {
        self.unlock_key = UnlockKey::new(self.center);
        self.access_state = self.unlock_key.calculate_access_state();
        self.total_unfolded = 0.0;
        self.last_unlock_timestamp = None;
    }
}

/// System of all energy center interfaces
///
/// Manages all 7 energy center interfaces as a unified system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyCenterInterfaceSystem {
    /// The entity ID
    pub entity_id: usize,

    /// All 7 energy center interfaces
    pub interfaces: HashMap<RayCenter, EnergyCenterInterface>,

    /// Total potential unfolded across all centers
    pub total_unfolded_potential: Float,
}

impl EnergyCenterInterfaceSystem {
    /// Creates a new energy center interface system for an entity
    pub fn new(entity_id: usize) -> Self {
        let mut interfaces = HashMap::new();

        for center in RayCenter::all() {
            interfaces.insert(center, EnergyCenterInterface::new(center));
        }

        EnergyCenterInterfaceSystem {
            entity_id,
            interfaces,
            total_unfolded_potential: 0.0,
        }
    }

    /// Gets a specific energy center interface
    pub fn get_interface(&self, center: RayCenter) -> Option<&EnergyCenterInterface> {
        self.interfaces.get(&center)
    }

    /// Gets a mutable reference to a specific energy center interface
    pub fn get_interface_mut(&mut self, center: RayCenter) -> Option<&mut EnergyCenterInterface> {
        self.interfaces.get_mut(&center)
    }

    /// Unlocks potential through a specific energy center
    pub fn unlock_potential(
        &mut self,
        center: RayCenter,
        seed: &HolographicSeed,
        catalyst_intensity: Float,
    ) -> Option<UnfoldedPotential> {
        if let Some(interface) = self.interfaces.get_mut(&center) {
            let unfolded = interface.unlock_potential(seed, catalyst_intensity);
            self.total_unfolded_potential += unfolded.total_potential();
            Some(unfolded)
        } else {
            None
        }
    }

    /// Unlocks potential through multiple energy centers
    pub fn unlock_potential_multiple(
        &mut self,
        centers: Vec<RayCenter>,
        seed: &HolographicSeed,
        catalyst_intensity: Float,
    ) -> Vec<UnfoldedPotential> {
        let mut results = Vec::new();

        for center in centers {
            if let Some(unfolded) = self.unlock_potential(center, seed, catalyst_intensity) {
                results.push(unfolded);
            }
        }

        results
    }

    /// Strengthens unlock keys for all centers (evolutionary progression)
    pub fn strengthen_all_keys(&mut self, amount: Float) {
        for interface in self.interfaces.values_mut() {
            interface.strengthen_unlock_key(amount);
        }
    }

    /// Returns the total potential unfolded across all centers
    pub fn total_unfolded_potential(&self) -> Float {
        self.total_unfolded_potential
    }

    /// Returns whether all centers are fully unlocked
    pub fn all_centers_fully_unlocked(&self) -> bool {
        self.interfaces.values().all(|i| i.is_fully_unlocked())
    }

    /// Returns whether any center is locked
    pub fn any_center_locked(&self) -> bool {
        self.interfaces.values().any(|i| i.is_locked())
    }

    /// **IMPORTANT:** This method returns 0.0 to demonstrate that centers don't store energy
    ///
    /// In the OLD container-based architecture, this would return the total stored energy.
    /// In the NEW interface-based architecture, centers don't store anything.
    pub fn total_stored_energy(&self) -> Float {
        0.0 // Centers don't store energy - they unlock potential from seed
    }
}

impl Default for EnergyCenterInterface {
    fn default() -> Self {
        Self::new(RayCenter::Red)
    }
}

impl Default for EnergyCenterInterfaceSystem {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_state_unlock_percentage() {
        assert_eq!(AccessState::Locked.unlock_percentage(), 0.0);
        assert_eq!(AccessState::Unlocked.unlock_percentage(), 1.0);
        assert_eq!(
            AccessState::PartiallyUnlocked {
                unlock_percentage: 0.5
            }
            .unlock_percentage(),
            0.5
        );
    }

    #[test]
    fn test_access_state_is_fully_unlocked() {
        assert!(!AccessState::Locked.is_fully_unlocked());
        assert!(AccessState::Unlocked.is_fully_unlocked());
        assert!(!AccessState::PartiallyUnlocked {
            unlock_percentage: 0.5
        }
        .is_fully_unlocked());
    }

    #[test]
    fn test_unlock_key_creation() {
        let key = UnlockKey::new(RayCenter::Red);
        assert_eq!(key.center, RayCenter::Red);
        assert_eq!(key.strength, 0.0);
        assert_eq!(key.complexity, 0.0);
    }

    #[test]
    fn test_unlock_key_strengthen() {
        let mut key = UnlockKey::new(RayCenter::Red);
        key.strengthen(0.5);
        assert_eq!(key.strength, 0.5);

        key.strengthen(0.6);
        assert_eq!(key.strength, 1.0); // Capped at 1.0
    }

    #[test]
    fn test_unlock_key_calculate_access_state() {
        let mut key = UnlockKey::new(RayCenter::Red);
        assert!(matches!(key.calculate_access_state(), AccessState::Locked));

        key.strengthen(0.5);
        let state = key.calculate_access_state();
        assert!(matches!(state, AccessState::PartiallyUnlocked { .. }));

        key.strengthen(0.5);
        assert!(matches!(
            key.calculate_access_state(),
            AccessState::Unlocked
        ));
    }

    #[test]
    fn test_energy_center_interface_creation() {
        let interface = EnergyCenterInterface::new(RayCenter::Red);
        assert_eq!(interface.center, RayCenter::Red);
        assert!(interface.is_locked());
        assert_eq!(interface.total_unfolded(), 0.0);
    }

    #[test]
    fn test_energy_center_interface_stored_energy() {
        let mut interface = EnergyCenterInterface::new(RayCenter::Red);
        // IMPORTANT: Centers don't store energy
        assert_eq!(interface.stored_energy(), 0.0);

        // Even after unlocking, still no stored energy
        let seed = HolographicSeed::new_from_source();
        let _unfolded = interface.unlock_potential(&seed, 1.0);
        assert_eq!(interface.stored_energy(), 0.0);
    }

    #[test]
    fn test_energy_center_interface_unlock_potential() {
        let mut interface = EnergyCenterInterface::new(RayCenter::Red);
        let seed = HolographicSeed::new_from_source();

        // Initially locked, so no potential unfolded
        let unfolded = interface.unlock_potential(&seed, 1.0);
        assert_eq!(unfolded.total_potential(), 0.0);

        // Strengthen the unlock key
        interface.strengthen_unlock_key(0.5);

        // Now partially unlocked, some potential unfolded
        let unfolded = interface.unlock_potential(&seed, 1.0);
        assert!(unfolded.total_potential() > 0.0);
        assert!(unfolded.is_from_seed());
        assert_eq!(unfolded.unlocked_by, RayCenter::Red);
    }

    #[test]
    fn test_unfolded_potential_is_from_seed() {
        let unfolded = UnfoldedPotential::new(RayCenter::Red, PotentialSource::HolographicSeed);
        assert!(unfolded.is_from_seed());

        let external = UnfoldedPotential::new(RayCenter::Red, PotentialSource::External);
        assert!(!external.is_from_seed());
    }

    #[test]
    fn test_unfolded_potential_add_archetype_energy() {
        let mut unfolded = UnfoldedPotential::new(RayCenter::Red, PotentialSource::HolographicSeed);
        unfolded.add_archetype_energy(0, 1.0);
        unfolded.add_archetype_energy(1, 2.0);

        assert_eq!(unfolded.archetype_energies[0], 1.0);
        assert_eq!(unfolded.archetype_energies[1], 2.0);
        assert_eq!(unfolded.total_potential(), 3.0);
    }

    #[test]
    fn test_unfolded_potential_update_consciousness_state() {
        let mut unfolded = UnfoldedPotential::new(RayCenter::Red, PotentialSource::HolographicSeed);

        // Add some archetype energies
        unfolded.add_archetype_energy(0, 1.0);
        unfolded.add_archetype_energy(7, 1.0);
        unfolded.add_archetype_energy(14, 1.0);

        unfolded.update_consciousness_state();

        // Check that consciousness state is updated
        assert!(unfolded.consciousness_state.mind_activation > 0.0);
        assert!(unfolded.consciousness_state.body_activation > 0.0);
        assert!(unfolded.consciousness_state.spirit_activation > 0.0);
    }

    #[test]
    fn test_energy_center_interface_system_creation() {
        let system = EnergyCenterInterfaceSystem::new(1);
        assert_eq!(system.entity_id, 1);
        assert_eq!(system.interfaces.len(), 7);
        assert_eq!(system.total_unfolded_potential(), 0.0);
    }

    #[test]
    fn test_energy_center_interface_system_get_interface() {
        let system = EnergyCenterInterfaceSystem::new(1);
        assert!(system.get_interface(RayCenter::Red).is_some());
        assert!(system.get_interface(RayCenter::Violet).is_some());
    }

    #[test]
    fn test_energy_center_interface_system_unlock_potential() {
        let mut system = EnergyCenterInterfaceSystem::new(1);
        let seed = HolographicSeed::new_from_source();

        // Strengthen the red ray unlock key
        if let Some(red_interface) = system.get_interface_mut(RayCenter::Red) {
            red_interface.strengthen_unlock_key(0.5);
        }

        // Unlock potential through red ray
        let unfolded = system.unlock_potential(RayCenter::Red, &seed, 1.0);
        assert!(unfolded.is_some());
        assert!(unfolded.unwrap().is_from_seed());
    }

    #[test]
    fn test_energy_center_interface_system_total_stored_energy() {
        let system = EnergyCenterInterfaceSystem::new(1);
        // IMPORTANT: System doesn't store energy
        assert_eq!(system.total_stored_energy(), 0.0);
    }

    #[test]
    fn test_energy_center_interface_system_strengthen_all_keys() {
        let mut system = EnergyCenterInterfaceSystem::new(1);
        system.strengthen_all_keys(0.5);

        // All interfaces should have strengthened keys
        for interface in system.interfaces.values() {
            assert!(interface.unlock_key_strength() > 0.0);
        }
    }

    #[test]
    fn test_unlocking_vs_containing() {
        // This test demonstrates the architectural principle:
        // Energy centers UNLOCK potential from seed, they don't CONTAIN energy

        let mut interface = EnergyCenterInterface::new(RayCenter::Red);
        let seed = HolographicSeed::new_from_source();

        // Strengthen the unlock key
        interface.strengthen_unlock_key(1.0);

        // Unlock potential
        let unfolded = interface.unlock_potential(&seed, 100.0);

        // Verify:
        // 1. Potential came from seed (not from center)
        assert!(unfolded.is_from_seed());

        // 2. Center doesn't store energy
        assert_eq!(interface.stored_energy(), 0.0);

        // 3. Potential was unfolded from seed
        assert!(unfolded.total_potential() > 0.0);

        // 4. Total unfolded is tracked
        assert_eq!(interface.total_unfolded(), unfolded.total_potential());
    }

    #[test]
    fn test_progressive_unlocking() {
        // Test that progressive unlocking works
        let mut interface = EnergyCenterInterface::new(RayCenter::Red);
        let seed = HolographicSeed::new_from_source();

        // Initially locked
        assert!(interface.is_locked());

        // Partially unlock
        interface.strengthen_unlock_key(0.3);
        let unfolded1 = interface.unlock_potential(&seed, 100.0);
        assert!(!interface.is_fully_unlocked());
        assert!(unfolded1.total_potential() > 0.0);

        // More unlock
        interface.strengthen_unlock_key(0.3);
        let unfolded2 = interface.unlock_potential(&seed, 100.0);
        assert!(unfolded2.total_potential() > unfolded1.total_potential());

        // Fully unlock
        interface.strengthen_unlock_key(0.4);
        let unfolded3 = interface.unlock_potential(&seed, 100.0);
        assert!(interface.is_fully_unlocked());
        assert!(unfolded3.total_potential() >= unfolded2.total_potential());
    }
}
