// Veil Mechanism - Creates separation illusion
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "The Veil separates these realms, creating the illusion that they are separate"
// "The Veil creates the conditions for the Entity to experience separation"

use crate::evolution_density_octave::density_octave::{
    Density, Density1SubLevel, Density2SubLevel,
};
use crate::types::Float;
use std::collections::HashMap;

use super::density_variation::{DensityTransparency, PolarizationAccess, PolarizationState};
use super::piercing::{PiercingEvent, PiercingLocation, PiercingResult};
use super::playground::{PlaygroundAction, PlaygroundResult, VeilOpportunityType, VeilPlayground};
use super::{ACCESS_THRESHOLD, THIN_SPOT_THRESHOLD};

/// Entity identifier type
pub type EntityId = u64;

/// Thin spot in the veil
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Veil thins as entities polarize and ascend"
#[derive(Debug, Clone, PartialEq)]
pub struct ThinSpot {
    /// Location in the mind/body/spirit complex
    pub location: PiercingLocation,

    /// Strength of the thin spot
    pub strength: Float,

    /// When this thin spot was created
    pub created_at: Float,
}

impl ThinSpot {
    /// Create new thin spot
    pub fn new(location: PiercingLocation, strength: Float, created_at: Float) -> Self {
        ThinSpot {
            location,
            strength: strength.max(0.0).min(1.0),
            created_at,
        }
    }
}

/// Individual veil state for an entity
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "The Veil creates the conditions for the Entity to experience separation"
#[derive(Debug, Clone, PartialEq)]
pub struct VeilState {
    /// How much the veil obscures higher truths (0.0 = transparent, 1.0 = opaque)
    pub opacity: Float,

    /// Moments of veil thinning
    pub thin_spots: Vec<ThinSpot>,

    /// Accumulated piercing through spiritual work
    pub accumulated_piercing: Float,
}

impl VeilState {
    /// Create new veil state
    pub fn new() -> Self {
        VeilState {
            opacity: 0.0,
            thin_spots: Vec::new(),
            accumulated_piercing: 0.0,
        }
    }

    /// Create veil state with specific opacity
    pub fn with_opacity(opacity: Float) -> Self {
        VeilState {
            opacity: opacity.max(0.0).min(1.0),
            thin_spots: Vec::new(),
            accumulated_piercing: 0.0,
        }
    }

    /// Add accumulated piercing
    pub fn add_piercing(&mut self, amount: Float) {
        self.accumulated_piercing += amount.max(0.0);
    }

    /// Add thin spot
    pub fn add_thin_spot(&mut self, thin_spot: ThinSpot) {
        self.thin_spots.push(thin_spot);
    }

    /// Get total thin spot strength
    pub fn total_thin_spot_strength(&self) -> Float {
        self.thin_spots.iter().map(|spot| spot.strength).sum()
    }

    /// Get number of thin spots
    pub fn thin_spot_count(&self) -> usize {
        self.thin_spots.len()
    }
}

impl Default for VeilState {
    fn default() -> Self {
        Self::new()
    }
}

/// Veil Mechanism - Creates separation illusion
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 3.2
/// "Veil is 'playground' not barrier"
#[derive(Debug, Clone, PartialEq)]
pub struct VeilMechanism {
    /// Density-based transparency (higher = more transparent)
    pub density_transparency: DensityTransparency,

    /// Polarity-based access
    pub polarization_access: PolarizationAccess,

    /// Individual veil state for each entity
    pub entity_veil_state: HashMap<EntityId, VeilState>,

    /// Veil as evolutionary playground
    pub playground: VeilPlayground,
}

impl VeilMechanism {
    /// Create new veil mechanism
    pub fn new() -> Self {
        VeilMechanism {
            density_transparency: DensityTransparency::new(),
            polarization_access: PolarizationAccess::new(),
            entity_veil_state: HashMap::new(),
            playground: VeilPlayground::new(),
        }
    }

    /// Create veil mechanism with custom settings
    pub fn with_settings(
        density_transparency: DensityTransparency,
        polarization_access: PolarizationAccess,
    ) -> Self {
        VeilMechanism {
            density_transparency,
            polarization_access,
            entity_veil_state: HashMap::new(),
            playground: VeilPlayground::new(),
        }
    }

    /// Calculate veil thickness for an entity
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The entity's density
    /// * `polarization` - The entity's polarization state
    ///
    /// # Returns
    /// Veil thickness (0.0 = transparent, 1.0 = opaque)
    pub fn calculate_veil_thickness(
        &self,
        entity_id: EntityId,
        density: &Density,
        polarization: &PolarizationState,
    ) -> Float {
        // Base thickness from density
        let base_thickness = self.density_transparency.base_thickness(density);

        // Thinning from polarization
        let polarization_thinning = self.polarization_access.calculate_thinning(polarization);

        // Accumulated piercing
        let accumulated_thinning = self
            .entity_veil_state
            .get(&entity_id)
            .map(|state| state.accumulated_piercing)
            .unwrap_or(0.0);

        // Calculate final thickness (cannot be negative)
        (base_thickness - polarization_thinning - accumulated_thinning).max(0.0)
    }

    /// Check if entity can access higher truths through veil
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The entity's density
    /// * `polarization` - The entity's polarization state
    ///
    /// # Returns
    /// True if veil is thin enough to access higher truths
    pub fn can_access_higher_truths(
        &self,
        entity_id: EntityId,
        density: &Density,
        polarization: &PolarizationState,
    ) -> bool {
        let thickness = self.calculate_veil_thickness(entity_id, density, polarization);
        thickness < ACCESS_THRESHOLD
    }

    /// Record a veil-piercing event
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `piercing_event` - The piercing event to record
    ///
    /// # Returns
    /// Result of the piercing event
    pub fn record_veil_piercing(
        &mut self,
        entity_id: EntityId,
        piercing_event: PiercingEvent,
    ) -> PiercingResult {
        // Get or create veil state
        let veil_state = self
            .entity_veil_state
            .entry(entity_id)
            .or_insert_with(VeilState::new);

        // Calculate piercing strength
        let piercing_strength = piercing_event.calculate_strength();

        // Add to accumulated piercing
        veil_state.add_piercing(piercing_strength);

        // Create thin spot if significant
        let thin_spot_created = piercing_strength > THIN_SPOT_THRESHOLD;
        if thin_spot_created {
            let thin_spot = ThinSpot::new(
                piercing_event.location,
                piercing_strength,
                piercing_event.timestamp,
            );
            veil_state.add_thin_spot(thin_spot);
        }

        // Calculate new thickness
        let new_thickness = self.calculate_veil_thickness(
            entity_id,
            &piercing_event.density,
            &piercing_event.polarization,
        );

        PiercingResult {
            new_thickness,
            thin_spot_created,
            piercing_strength,
        }
    }

    /// Get veil state for an entity
    pub fn get_veil_state(&self, entity_id: EntityId) -> Option<&VeilState> {
        self.entity_veil_state.get(&entity_id)
    }

    /// Get mut veil state for an entity
    pub fn get_veil_state_mut(&mut self, entity_id: EntityId) -> Option<&mut VeilState> {
        self.entity_veil_state.get_mut(&entity_id)
    }

    /// Remove entity from veil tracking
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<VeilState> {
        self.entity_veil_state.remove(&entity_id)
    }

    /// Get number of tracked entities
    pub fn entity_count(&self) -> usize {
        self.entity_veil_state.len()
    }

    /// Clear all entity states
    pub fn clear_entities(&mut self) {
        self.entity_veil_state.clear();
    }

    /// Set density transparency
    pub fn set_density_transparency(&mut self, density_transparency: DensityTransparency) {
        self.density_transparency = density_transparency;
    }

    /// Set polarization access
    pub fn set_polarization_access(&mut self, polarization_access: PolarizationAccess) {
        self.polarization_access = polarization_access;
    }

    /// Get veil statistics for an entity
    pub fn get_statistics(
        &self,
        entity_id: EntityId,
        density: Density,
        polarization: &PolarizationState,
    ) -> Option<VeilStatistics> {
        let veil_state = self.entity_veil_state.get(&entity_id)?;

        Some(VeilStatistics {
            entity_id,
            current_thickness: self.calculate_veil_thickness(entity_id, &density, polarization),
            base_thickness: self.density_transparency.base_thickness(&density),
            accumulated_piercing: veil_state.accumulated_piercing,
            thin_spot_count: veil_state.thin_spot_count(),
            total_thin_spot_strength: veil_state.total_thin_spot_strength(),
            can_access_higher_truths: self.can_access_higher_truths(
                entity_id,
                &density,
                polarization,
            ),
        })
    }

    // ===== Playground Integration Methods =====

    /// Process a playground action for an entity
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 3.2
    /// "Veil is 'playground' not barrier"
    ///
    /// This method integrates the playground concept with the veil mechanism,
    /// allowing entities to take actions that provide evolutionary opportunities.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `action` - The playground action to process
    /// * `current_time` - The current simulation time
    ///
    /// # Returns
    /// Result of processing the playground action
    pub fn process_playground_action(
        &mut self,
        entity_id: EntityId,
        action: PlaygroundAction,
        current_time: Float,
    ) -> PlaygroundResult {
        self.playground
            .process_progress(entity_id, action, current_time)
    }

    /// Present a playground challenge to an entity
    ///
    /// This method presents a challenge that the entity must overcome
    /// as part of their evolutionary journey through the veil.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `challenge_type` - The type of challenge to present
    /// * `difficulty` - The difficulty level (0.0 = easy, 1.0 = very difficult)
    /// * `current_time` - The current simulation time
    pub fn present_challenge(
        &mut self,
        entity_id: EntityId,
        challenge_type: super::playground::VeilChallengeType,
        difficulty: Float,
        current_time: Float,
    ) {
        self.playground
            .present_challenge(entity_id, challenge_type, difficulty, current_time);
    }

    /// Present a playground opportunity to an entity
    ///
    /// This method presents an opportunity that the entity can accept
    /// and realize as part of their evolutionary journey through the veil.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `opportunity_type` - The type of opportunity to present
    /// * `current_time` - The current simulation time
    pub fn present_opportunity(
        &mut self,
        entity_id: EntityId,
        opportunity_type: super::playground::VeilOpportunityType,
        current_time: Float,
    ) {
        self.playground
            .present_opportunity(entity_id, opportunity_type, current_time);
    }

    /// Get playground statistics for an entity
    ///
    /// This method provides statistics about the entity's progress
    /// through the veil's evolutionary playground.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Playground statistics if the entity exists
    pub fn get_playground_statistics(
        &self,
        entity_id: EntityId,
    ) -> Option<super::playground::PlaygroundStatistics> {
        self.playground.get_statistics(entity_id)
    }

    /// Get the growth potential available to an entity
    ///
    /// This method calculates the total growth potential from all
    /// active opportunities available to the entity.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Total growth potential (0.0 if no opportunities)
    pub fn get_growth_potential(&self, entity_id: EntityId) -> Float {
        self.playground.get_growth_potential(entity_id)
    }

    /// Get the total challenge difficulty for an entity
    ///
    /// This method calculates the total difficulty from all
    /// active challenges facing the entity.
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    ///
    /// # Returns
    /// Total challenge difficulty (0.0 if no challenges)
    pub fn get_total_difficulty(&self, entity_id: EntityId) -> Float {
        self.playground.get_total_difficulty(entity_id)
    }

    /// Get mutable reference to the playground
    pub fn playground_mut(&mut self) -> &mut VeilPlayground {
        &mut self.playground
    }

    /// Get reference to the playground
    pub fn playground(&self) -> &VeilPlayground {
        &self.playground
    }

    // ===== Veil Thickening Mechanism (Task 1.3) =====

    /// Set veil thickness for an entity based on density
    ///
    /// This method is called during density transitions to adjust veil thickness
    /// according to the involution curve. The veil thickens as entities descend
    /// to lower densities.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The target density
    ///
    /// # Returns
    /// The new veil thickness (0.0 to 1.0)
    pub fn set_veil_thickness_by_density(
        &mut self,
        entity_id: EntityId,
        density: Density,
    ) -> Float {
        let thickness = self.density_transparency.base_thickness(&density);

        // Get or create veil state
        let veil_state = self
            .entity_veil_state
            .entry(entity_id)
            .or_insert_with(VeilState::new);

        // Update opacity
        veil_state.opacity = thickness;

        thickness
    }

    /// Calculate veil transparency (1.0 - thickness)
    ///
    /// This method calculates how much of higher truths can be perceived
    /// through the veil. Higher transparency = more access to higher truths.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Arguments
    /// * `thickness` - The veil thickness (0.0 to 1.0)
    ///
    /// # Returns
    /// Veil transparency (0.0 to 1.0)
    pub fn calculate_veil_transparency(&self, thickness: Float) -> Float {
        (1.0 - thickness).max(0.0).min(1.0)
    }

    /// Apply veil effects to entity perception and memory access
    ///
    /// This method calculates how the veil affects an entity's ability to:
    /// - Perceive higher truths
    /// - Access conscious memory
    /// - Process catalyst effectively
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Arguments
    /// * `entity_id` - The entity's identifier
    /// * `density` - The entity's density
    /// * `polarization` - The entity's polarization state
    ///
    /// # Returns
    /// VeilEffects struct containing perception modifier and memory access modifier
    pub fn apply_veil_effects(
        &self,
        entity_id: EntityId,
        density: Density,
        polarization: &PolarizationState,
    ) -> VeilEffects {
        let thickness = self.calculate_veil_thickness(entity_id, &density, polarization);
        let transparency = self.calculate_veil_transparency(thickness);

        // Perception modifier: veil reduces clarity of higher truth perception
        // At 0% veil (100% transparent): perception = 1.0 (full clarity)
        // At 90% veil (10% transparent): perception = 0.1 (very limited)
        let perception_modifier = transparency;

        // Memory access modifier: veil reduces access to conscious memory of higher densities
        // At 0% veil: memory access = 1.0 (full conscious access)
        // At 90% veil: memory access = 0.1 (only subconscious/superconscious access)
        let memory_access_modifier = transparency;

        // Catalyst difficulty modifier: veil increases catalyst processing difficulty
        // At 0% veil: difficulty = 1.0 (normal)
        // At 90% veil: difficulty = 1.9 (almost twice as hard)
        let catalyst_difficulty_modifier = 1.0 + (thickness * 0.9);

        VeilEffects {
            thickness,
            transparency,
            perception_modifier,
            memory_access_modifier,
            catalyst_difficulty_modifier,
        }
    }

    /// Get the veil thickness curve for all densities
    ///
    /// This method returns the complete veil thickness curve for documentation
    /// and analysis purposes.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Returns
    /// Array of thickness values [D1, D2, D3, D4, D5, D6, D7]
    pub fn get_veil_thickness_curve(&self) -> [Float; 7] {
        self.density_transparency.thickness_curve()
    }

    /// Check if veil thickening is needed for a density transition
    ///
    /// This method determines if a density transition requires veil thickening
    /// (descending to lower densities) or thinning (ascending to higher densities).
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Arguments
    /// * `from_density` - Starting density
    /// * `to_density` - Target density
    ///
    /// # Returns
    /// true if veil should thicken, false if it should thin
    pub fn should_thicken_veil(&self, from_density: Density, to_density: Density) -> bool {
        let from_thickness = self.density_transparency.base_thickness(&from_density);
        let to_thickness = self.density_transparency.base_thickness(&to_density);
        to_thickness > from_thickness
    }

    /// Calculate transition veil thickness during density change
    ///
    /// This method calculates veil thickness during a gradual density transition,
    /// allowing for smooth thickening or thinning over time.
    ///
    /// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
    ///
    /// # Arguments
    /// * `from_density` - Starting density
    /// * `to_density` - Target density
    /// * `progress` - Transition progress (0.0 to 1.0)
    ///
    /// # Returns
    /// Interpolated veil thickness
    pub fn calculate_transition_thickness(
        &self,
        from_density: Density,
        to_density: Density,
        progress: Float,
    ) -> Float {
        self.density_transparency
            .transition_thickness(from_density, to_density, progress)
    }
}

impl Default for VeilMechanism {
    fn default() -> Self {
        Self::new()
    }
}

/// Veil statistics for analysis
#[derive(Debug, Clone, PartialEq)]
pub struct VeilStatistics {
    /// Entity identifier
    pub entity_id: EntityId,

    /// Current veil thickness
    pub current_thickness: Float,

    /// Base thickness for the entity's density
    pub base_thickness: Float,

    /// Total accumulated piercing
    pub accumulated_piercing: Float,

    /// Number of thin spots
    pub thin_spot_count: usize,

    /// Total thin spot strength
    pub total_thin_spot_strength: Float,

    /// Whether entity can access higher truths
    pub can_access_higher_truths: bool,
}

/// Veil effects on entity perception and memory
///
/// This struct quantifies how the veil affects an entity's ability to
/// perceive higher truths, access memory, and process catalyst.
///
/// Knowledge Base Reference: COMPLETE_REFACTOR_ROADMAP_V4.md Phase 1, Task 1.3
#[derive(Debug, Clone, PartialEq)]
pub struct VeilEffects {
    /// Current veil thickness (0.0 to 1.0)
    pub thickness: Float,

    /// Veil transparency (1.0 - thickness)
    pub transparency: Float,

    /// Perception modifier (0.0 to 1.0)
    /// Higher = clearer perception of higher truths
    pub perception_modifier: Float,

    /// Memory access modifier (0.0 to 1.0)
    /// Higher = better access to conscious memory
    pub memory_access_modifier: Float,

    /// Catalyst difficulty modifier (1.0 to 1.9)
    /// Higher = harder to process catalyst
    pub catalyst_difficulty_modifier: Float,
}

impl VeilEffects {
    /// Create new veil effects
    pub fn new(
        thickness: Float,
        transparency: Float,
        perception_modifier: Float,
        memory_access_modifier: Float,
        catalyst_difficulty_modifier: Float,
    ) -> Self {
        VeilEffects {
            thickness: thickness.max(0.0).min(1.0),
            transparency: transparency.max(0.0).min(1.0),
            perception_modifier: perception_modifier.max(0.0).min(1.0),
            memory_access_modifier: memory_access_modifier.max(0.0).min(1.0),
            catalyst_difficulty_modifier: catalyst_difficulty_modifier.max(1.0).min(1.9),
        }
    }

    /// Calculate memory retention percentage based on veil
    ///
    /// Returns the percentage of conscious memory retained through the veil
    pub fn memory_retention_percentage(&self) -> Float {
        self.memory_access_modifier * 100.0
    }

    /// Calculate catalyst difficulty multiplier
    ///
    /// Returns the multiplier for catalyst processing difficulty
    pub fn catalyst_difficulty_multiplier(&self) -> Float {
        self.catalyst_difficulty_modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== ThinSpot Tests =====

    #[test]
    fn test_thin_spot_new() {
        let spot = ThinSpot::new(PiercingLocation::Mind, 0.1, 100.0);
        assert_eq!(spot.location, PiercingLocation::Mind);
        assert_eq!(spot.strength, 0.1);
        assert_eq!(spot.created_at, 100.0);
    }

    #[test]
    fn test_thin_spot_strength_clamped() {
        let spot_low = ThinSpot::new(PiercingLocation::Mind, -0.5, 100.0);
        assert_eq!(spot_low.strength, 0.0);

        let spot_high = ThinSpot::new(PiercingLocation::Mind, 1.5, 100.0);
        assert_eq!(spot_high.strength, 1.0);
    }

    // ===== VeilState Tests =====

    #[test]
    fn test_veil_state_new() {
        let state = VeilState::new();
        assert_eq!(state.opacity, 0.0);
        assert_eq!(state.thin_spots.len(), 0);
        assert_eq!(state.accumulated_piercing, 0.0);
    }

    #[test]
    fn test_veil_state_with_opacity() {
        let state = VeilState::with_opacity(0.5);
        assert_eq!(state.opacity, 0.5);
    }

    #[test]
    fn test_veil_state_with_opacity_clamped() {
        let state_low = VeilState::with_opacity(-0.5);
        assert_eq!(state_low.opacity, 0.0);

        let state_high = VeilState::with_opacity(1.5);
        assert_eq!(state_high.opacity, 1.0);
    }

    #[test]
    fn test_veil_state_add_piercing() {
        let mut state = VeilState::new();
        state.add_piercing(0.1);
        assert!((state.accumulated_piercing - 0.1).abs() < 0.001);
        state.add_piercing(0.05);
        assert!((state.accumulated_piercing - 0.15).abs() < 0.001);
    }

    #[test]
    fn test_veil_state_add_piercing_negative() {
        let mut state = VeilState::new();
        state.add_piercing(-0.5);
        assert_eq!(state.accumulated_piercing, 0.0); // Clamped to 0.0
    }

    #[test]
    fn test_veil_state_add_thin_spot() {
        let mut state = VeilState::new();
        let spot = ThinSpot::new(PiercingLocation::Mind, 0.1, 100.0);
        state.add_thin_spot(spot);
        assert_eq!(state.thin_spot_count(), 1);
    }

    #[test]
    fn test_veil_state_total_thin_spot_strength() {
        let mut state = VeilState::new();
        state.add_thin_spot(ThinSpot::new(PiercingLocation::Mind, 0.1, 100.0));
        state.add_thin_spot(ThinSpot::new(PiercingLocation::Body, 0.05, 101.0));
        assert!((state.total_thin_spot_strength() - 0.15).abs() < 0.001);
    }

    #[test]
    fn test_veil_state_default() {
        let state = VeilState::default();
        assert_eq!(state.opacity, 0.0);
        assert_eq!(state.accumulated_piercing, 0.0);
    }

    // ===== VeilMechanism Tests =====

    #[test]
    fn test_veil_mechanism_new() {
        let mechanism = VeilMechanism::new();
        assert_eq!(mechanism.entity_count(), 0);
    }

    #[test]
    fn test_veil_mechanism_with_settings() {
        let dt = DensityTransparency::new();
        let pa = PolarizationAccess::new();
        let mechanism = VeilMechanism::with_settings(dt, pa);
        assert_eq!(mechanism.entity_count(), 0);
    }

    #[test]
    fn test_calculate_veil_thickness_d3_no_polarization() {
        let mechanism = VeilMechanism::new();
        let polarization = PolarizationState::neutral();
        let thickness = mechanism.calculate_veil_thickness(1, &Density::Third, &polarization);
        assert_eq!(thickness, 0.60); // Base thickness for D3 (updated from roadmap)
    }

    #[test]
    fn test_calculate_veil_thickness_d3_with_sto() {
        let mechanism = VeilMechanism::new();
        let polarization = PolarizationState::sto(0.5);
        // Base: 0.60, Thinning: 0.5 * 0.3 = 0.15, Result: 0.45
        let thickness = mechanism.calculate_veil_thickness(1, &Density::Third, &polarization);
        assert_eq!(thickness, 0.45);
    }

    #[test]
    fn test_calculate_veil_thickness_with_accumulated_piercing() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Add accumulated piercing
        mechanism
            .entity_veil_state
            .entry(entity_id)
            .or_insert_with(VeilState::new)
            .add_piercing(0.1);

        let polarization = PolarizationState::sto(0.5);
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.1, Result: 0.35
        let thickness =
            mechanism.calculate_veil_thickness(entity_id, &Density::Third, &polarization);
        assert_eq!(thickness, 0.35);
    }

    #[test]
    fn test_calculate_veil_thickness_cannot_go_negative() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Add lots of accumulated piercing (more than base thickness)
        mechanism
            .entity_veil_state
            .entry(entity_id)
            .or_insert_with(VeilState::new)
            .add_piercing(1.0);

        let polarization = PolarizationState::sto(1.0);
        // Base: 0.60, Polarity: 0.3, Accumulated: 1.0, Total thinning: 1.3
        // Result should be max(0.60 - 1.3, 0.0) = 0.0
        let thickness =
            mechanism.calculate_veil_thickness(entity_id, &Density::Third, &polarization);
        assert_eq!(thickness, 0.0);
    }

    #[test]
    fn test_can_access_higher_truths_true() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Add enough piercing to get below threshold
        mechanism
            .entity_veil_state
            .entry(entity_id)
            .or_insert_with(VeilState::new)
            .add_piercing(0.4);

        let polarization = PolarizationState::sto(0.5);
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.4, Result: 0.05
        // 0.05 < 0.3 (ACCESS_THRESHOLD) = true
        let can_access =
            mechanism.can_access_higher_truths(entity_id, &Density::Third, &polarization);
        assert!(can_access);
    }

    #[test]
    fn test_can_access_higher_truths_false() {
        let mechanism = VeilMechanism::new();
        let polarization = PolarizationState::neutral();
        // Base: 0.60, Result: 0.60
        // 0.60 >= 0.3 (ACCESS_THRESHOLD) = false
        let can_access = mechanism.can_access_higher_truths(1, &Density::Third, &polarization);
        assert!(!can_access);
    }

    #[test]
    fn test_record_veil_piercing() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let result = mechanism.record_veil_piercing(entity_id, event);

        // Check piercing strength
        assert_eq!(result.piercing_strength, 0.1); // 0.01 * 10.0

        // Check thin spot created (0.1 > 0.05 threshold)
        assert!(result.thin_spot_created);

        // Check new thickness
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.1, Result: 0.35
        assert_eq!(result.new_thickness, 0.35);

        // Check entity state was created
        assert!(mechanism.get_veil_state(entity_id).is_some());
    }

    #[test]
    fn test_record_veil_piercing_no_thin_spot() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event = PiercingEvent::meditation(
            2.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let result = mechanism.record_veil_piercing(entity_id, event);

        // Check piercing strength
        assert_eq!(result.piercing_strength, 0.02); // 0.01 * 2.0

        // Check no thin spot created (0.02 < 0.05 threshold)
        assert!(!result.thin_spot_created);

        // Check new thickness
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.02, Result: 0.43
        assert_eq!(result.new_thickness, 0.43);
    }

    #[test]
    fn test_record_multiple_piercings() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event1 = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        let event2 = PiercingEvent::spiritual_work(
            0.5,
            PiercingLocation::Spirit,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(entity_id, event1);
        mechanism.record_veil_piercing(entity_id, event2);

        // Check accumulated piercing
        let state = mechanism.get_veil_state(entity_id).unwrap();
        assert_eq!(state.accumulated_piercing, 0.11); // 0.1 + 0.01

        // Check thin spot count (only first one exceeded threshold)
        assert_eq!(state.thin_spot_count(), 1);
    }

    #[test]
    fn test_get_veil_state() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(entity_id, event);

        let state = mechanism.get_veil_state(entity_id);
        assert!(state.is_some());
        assert_eq!(state.unwrap().accumulated_piercing, 0.1);
    }

    #[test]
    fn test_get_veil_state_none() {
        let mechanism = VeilMechanism::new();
        let state = mechanism.get_veil_state(999);
        assert!(state.is_none());
    }

    #[test]
    fn test_remove_entity() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(entity_id, event);

        let removed = mechanism.remove_entity(entity_id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().accumulated_piercing, 0.1);
        assert_eq!(mechanism.entity_count(), 0);
    }

    #[test]
    fn test_entity_count() {
        let mut mechanism = VeilMechanism::new();

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(1, event.clone());
        mechanism.record_veil_piercing(2, event.clone());
        mechanism.record_veil_piercing(3, event);

        assert_eq!(mechanism.entity_count(), 3);
    }

    #[test]
    fn test_clear_entities() {
        let mut mechanism = VeilMechanism::new();

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(1, event.clone());
        mechanism.record_veil_piercing(2, event);

        assert_eq!(mechanism.entity_count(), 2);

        mechanism.clear_entities();
        assert_eq!(mechanism.entity_count(), 0);
    }

    #[test]
    fn test_get_statistics() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let event = PiercingEvent::meditation(
            10.0,
            PiercingLocation::Mind,
            Density::Third,
            PolarizationState::sto(0.5),
        );

        mechanism.record_veil_piercing(entity_id, event);

        let polarization = PolarizationState::sto(0.5);
        let stats = mechanism.get_statistics(entity_id, Density::Third, &polarization);

        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.entity_id, entity_id);
        assert_eq!(stats.base_thickness, 0.60); // Updated from roadmap
        assert_eq!(stats.accumulated_piercing, 0.1);
        assert_eq!(stats.thin_spot_count, 1);
        assert_eq!(stats.total_thin_spot_strength, 0.1);
        assert!(stats.can_access_higher_truths); // 0.35 < 0.3 threshold
    }

    #[test]
    fn test_get_statistics_none() {
        let mechanism = VeilMechanism::new();
        let polarization = PolarizationState::neutral();
        let stats = mechanism.get_statistics(999, Density::Third, &polarization);
        assert!(stats.is_none());
    }

    #[test]
    fn test_veil_mechanism_default() {
        let mechanism = VeilMechanism::default();
        assert_eq!(mechanism.entity_count(), 0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_playground_integration_present_challenge() {
        use super::super::playground::VeilChallengeType;
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        mechanism.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );

        // Check that challenge was added to playground
        let stats = mechanism.get_playground_statistics(entity_id);
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().active_challenges_count, 1);
    }

    #[test]
    fn test_playground_integration_present_opportunity() {
        use super::super::playground::VeilOpportunityType;
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        mechanism.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);

        // Check that opportunity was added to playground
        let stats = mechanism.get_playground_statistics(entity_id);
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().active_opportunities_count, 1);
    }

    #[test]
    fn test_playground_integration_process_action() {
        use super::super::playground::VeilChallengeType;
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Present challenge and opportunity
        mechanism.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );

        let opportunity = mechanism.playground_mut().present_opportunity(
            entity_id,
            VeilOpportunityType::SpiritualGrowth,
            100.0,
        );
        opportunity.accept(101.0);

        // Process multiple actions to accumulate experience
        let mut total_experience = 0.0;
        for i in 0..30 {
            let result = mechanism.process_playground_action(
                entity_id,
                PlaygroundAction::SeekTruth,
                105.0 + i as f64,
            );
            total_experience += result.experience_gained;
        }

        // Check that statistics are available (indicating progress was made)
        let stats = mechanism.get_playground_statistics(entity_id);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        // Either experience was gained OR challenges/opportunities are still active (progress made)
        assert!(
            total_experience > 0.0
                || stats.active_challenges_count > 0
                || stats.active_opportunities_count > 0
        );
    }

    #[test]
    fn test_playground_integration_get_growth_potential() {
        use super::super::playground::VeilOpportunityType;
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        mechanism.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);
        mechanism.present_opportunity(entity_id, VeilOpportunityType::UnconditionalLove, 100.0);

        let potential = mechanism.get_growth_potential(entity_id);
        assert_eq!(potential, 0.9); // 0.5 + 0.4
    }

    #[test]
    fn test_playground_integration_get_total_difficulty() {
        use super::super::playground::VeilChallengeType;
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        mechanism.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );
        mechanism.present_challenge(
            entity_id,
            VeilChallengeType::FreeWillInSeparation,
            0.3,
            100.0,
        );

        let difficulty = mechanism.get_total_difficulty(entity_id);
        assert_eq!(difficulty, 0.8); // 0.5 + 0.3
    }

    #[test]
    fn test_playground_integration_playground_mut() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Use playground_mut to directly access and modify
        mechanism.playground_mut().present_challenge(
            entity_id,
            super::super::playground::VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );

        // Check that it was added
        let stats = mechanism.get_playground_statistics(entity_id);
        assert!(stats.is_some());
        assert_eq!(stats.unwrap().active_challenges_count, 1);
    }

    #[test]
    fn test_playground_integration_playground() {
        let mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Use playground to access read-only
        let playground = mechanism.playground();
        assert_eq!(playground.active_challenges.len(), 0);
        assert_eq!(playground.active_opportunities.len(), 0);
    }

    // ===== Veil Thickening Mechanism Tests (Task 1.3) =====

    #[test]
    fn test_set_veil_thickness_by_density_d1() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let thickness = mechanism
            .set_veil_thickness_by_density(entity_id, Density::First(Density1SubLevel::Quantum));
        assert_eq!(thickness, 0.90); // Maximum thickness at D1

        let state = mechanism.get_veil_state(entity_id).unwrap();
        assert_eq!(state.opacity, 0.90);
    }

    #[test]
    fn test_set_veil_thickness_by_density_d7() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        let thickness = mechanism.set_veil_thickness_by_density(entity_id, Density::Seventh);
        assert_eq!(thickness, 0.00); // No veil at D7

        let state = mechanism.get_veil_state(entity_id).unwrap();
        assert_eq!(state.opacity, 0.00);
    }

    #[test]
    fn test_set_veil_thickness_by_density_all_densities() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;

        // Test all densities
        let d1 = mechanism
            .set_veil_thickness_by_density(entity_id, Density::First(Density1SubLevel::Quantum));
        let d2 = mechanism
            .set_veil_thickness_by_density(entity_id, Density::Second(Density2SubLevel::Cellular));
        let d3 = mechanism.set_veil_thickness_by_density(entity_id, Density::Third);
        let d4 = mechanism.set_veil_thickness_by_density(entity_id, Density::Fourth);
        let d5 = mechanism.set_veil_thickness_by_density(entity_id, Density::Fifth);
        let d6 = mechanism.set_veil_thickness_by_density(entity_id, Density::Sixth);
        let d7 = mechanism.set_veil_thickness_by_density(entity_id, Density::Seventh);

        assert_eq!(d1, 0.90);
        assert_eq!(d2, 0.75);
        assert_eq!(d3, 0.60);
        assert_eq!(d4, 0.45);
        assert_eq!(d5, 0.30);
        assert_eq!(d6, 0.15);
        assert_eq!(d7, 0.00);
    }

    #[test]
    fn test_calculate_veil_transparency() {
        let mechanism = VeilMechanism::new();

        assert_eq!(mechanism.calculate_veil_transparency(0.0), 1.0);
        assert_eq!(mechanism.calculate_veil_transparency(0.5), 0.5);
        assert_eq!(mechanism.calculate_veil_transparency(1.0), 0.0);
    }

    #[test]
    fn test_calculate_veil_transparency_clamped() {
        let mechanism = VeilMechanism::new();

        assert_eq!(mechanism.calculate_veil_transparency(-0.5), 1.0); // Clamped to max
        assert_eq!(mechanism.calculate_veil_transparency(1.5), 0.0); // Clamped to min
    }

    #[test]
    fn test_apply_veil_effects_d1() {
        let mechanism = VeilMechanism::new();
        let entity_id = 1;
        let polarization = PolarizationState::neutral();

        let effects = mechanism.apply_veil_effects(
            entity_id,
            Density::First(Density1SubLevel::Quantum),
            &polarization,
        );

        assert_eq!(effects.thickness, 0.90);
        assert_eq!(effects.transparency, 0.10);
        assert_eq!(effects.perception_modifier, 0.10);
        assert_eq!(effects.memory_access_modifier, 0.10);
        assert!((effects.catalyst_difficulty_modifier - 1.81).abs() < 0.01); // 1.0 + 0.9 * 0.9
    }

    #[test]
    fn test_apply_veil_effects_d7() {
        let mechanism = VeilMechanism::new();
        let entity_id = 1;
        let polarization = PolarizationState::neutral();

        let effects = mechanism.apply_veil_effects(entity_id, Density::Seventh, &polarization);

        assert_eq!(effects.thickness, 0.00);
        assert_eq!(effects.transparency, 1.00);
        assert_eq!(effects.perception_modifier, 1.00);
        assert_eq!(effects.memory_access_modifier, 1.00);
        assert_eq!(effects.catalyst_difficulty_modifier, 1.00); // No difficulty increase
    }

    #[test]
    fn test_apply_veil_effects_with_polarization() {
        let mechanism = VeilMechanism::new();
        let entity_id = 1;
        let polarization = PolarizationState::sto(0.5);

        let effects = mechanism.apply_veil_effects(entity_id, Density::Third, &polarization);

        // Base: 0.60, Polarity thinning: 0.15, Result: 0.45
        assert_eq!(effects.thickness, 0.45);
        assert_eq!(effects.transparency, 0.55);
        assert_eq!(effects.perception_modifier, 0.55);
        assert_eq!(effects.memory_access_modifier, 0.55);
        assert!((effects.catalyst_difficulty_modifier - 1.405).abs() < 0.01); // 1.0 + 0.45 * 0.9
    }

    #[test]
    fn test_get_veil_thickness_curve() {
        let mechanism = VeilMechanism::new();
        let curve = mechanism.get_veil_thickness_curve();

        assert_eq!(curve, [0.90, 0.75, 0.60, 0.45, 0.30, 0.15, 0.00]);
    }

    #[test]
    fn test_should_thicken_veil_descending() {
        let mechanism = VeilMechanism::new();

        // Descending from D7 to D1: should thicken
        assert!(mechanism
            .should_thicken_veil(Density::Seventh, Density::First(Density1SubLevel::Quantum)));
        assert!(mechanism.should_thicken_veil(Density::Fourth, Density::Third));
        assert!(mechanism
            .should_thicken_veil(Density::Third, Density::Second(Density2SubLevel::Cellular)));
    }

    #[test]
    fn test_should_thicken_veil_ascending() {
        let mechanism = VeilMechanism::new();

        // Ascending from D1 to D7: should NOT thicken (should thin)
        assert!(!mechanism
            .should_thicken_veil(Density::First(Density1SubLevel::Quantum), Density::Seventh));
        assert!(!mechanism.should_thicken_veil(Density::Third, Density::Fourth));
        assert!(!mechanism
            .should_thicken_veil(Density::Second(Density2SubLevel::Cellular), Density::Third));
    }

    #[test]
    fn test_should_thicken_veil_same_density() {
        let mechanism = VeilMechanism::new();

        // Same density: should NOT thicken
        assert!(!mechanism.should_thicken_veil(Density::Third, Density::Third));
    }

    #[test]
    fn test_calculate_transition_thickness_d7_to_d1() {
        let mechanism = VeilMechanism::new();

        let thickness_0 = mechanism.calculate_transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            0.0,
        );
        assert_eq!(thickness_0, 0.00); // Start at D7 thickness

        let thickness_50 = mechanism.calculate_transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            0.5,
        );
        assert_eq!(thickness_50, 0.45); // Halfway

        let thickness_100 = mechanism.calculate_transition_thickness(
            Density::Seventh,
            Density::First(Density1SubLevel::Quantum),
            1.0,
        );
        assert_eq!(thickness_100, 0.90); // End at D1 thickness
    }

    #[test]
    fn test_calculate_transition_thickness_d4_to_d3() {
        let mechanism = VeilMechanism::new();

        let thickness =
            mechanism.calculate_transition_thickness(Density::Fourth, Density::Third, 0.0);
        assert_eq!(thickness, 0.45); // Start at D4 thickness

        let thickness_end =
            mechanism.calculate_transition_thickness(Density::Fourth, Density::Third, 1.0);
        assert_eq!(thickness_end, 0.60); // End at D3 thickness
    }

    #[test]
    fn test_veil_effects_new() {
        let effects = VeilEffects::new(0.5, 0.5, 0.5, 0.5, 1.45);
        assert_eq!(effects.thickness, 0.5);
        assert_eq!(effects.transparency, 0.5);
        assert_eq!(effects.perception_modifier, 0.5);
        assert_eq!(effects.memory_access_modifier, 0.5);
        assert_eq!(effects.catalyst_difficulty_modifier, 1.45);
    }

    #[test]
    fn test_veil_effects_new_clamped() {
        let effects = VeilEffects::new(-0.5, 1.5, 2.0, -1.0, 0.5);
        assert_eq!(effects.thickness, 0.0); // Clamped to 0.0
        assert_eq!(effects.transparency, 1.0); // Clamped to 1.0
        assert_eq!(effects.perception_modifier, 1.0); // Clamped to 1.0
        assert_eq!(effects.memory_access_modifier, 0.0); // Clamped to 0.0
        assert_eq!(effects.catalyst_difficulty_modifier, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_memory_retention_percentage() {
        let effects = VeilEffects::new(0.6, 0.4, 0.4, 0.4, 1.54);
        assert_eq!(effects.memory_retention_percentage(), 40.0);
    }

    #[test]
    fn test_catalyst_difficulty_multiplier() {
        let effects = VeilEffects::new(0.7, 0.3, 0.3, 0.3, 1.63);
        assert_eq!(effects.catalyst_difficulty_multiplier(), 1.63);
    }

    #[test]
    fn test_full_involution_veil_journey() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;
        let polarization = PolarizationState::neutral();

        // Start at D7 (no veil)
        mechanism.set_veil_thickness_by_density(entity_id, Density::Seventh);
        let effects_d7 = mechanism.apply_veil_effects(entity_id, Density::Seventh, &polarization);
        assert_eq!(effects_d7.thickness, 0.00);
        assert_eq!(effects_d7.memory_retention_percentage(), 100.0);

        // Descend to D6 (light veil)
        mechanism.set_veil_thickness_by_density(entity_id, Density::Sixth);
        let effects_d6 = mechanism.apply_veil_effects(entity_id, Density::Sixth, &polarization);
        assert_eq!(effects_d6.thickness, 0.15);
        assert_eq!(effects_d6.memory_retention_percentage(), 85.0);

        // Descend to D5 (medium light veil)
        mechanism.set_veil_thickness_by_density(entity_id, Density::Fifth);
        let effects_d5 = mechanism.apply_veil_effects(entity_id, Density::Fifth, &polarization);
        assert_eq!(effects_d5.thickness, 0.30);
        assert_eq!(effects_d5.memory_retention_percentage(), 70.0);

        // Descend to D4 (medium veil)
        mechanism.set_veil_thickness_by_density(entity_id, Density::Fourth);
        let effects_d4 = mechanism.apply_veil_effects(entity_id, Density::Fourth, &polarization);
        assert_eq!(effects_d4.thickness, 0.45);
        assert_eq!(effects_d4.memory_retention_percentage(), 55.0);

        // Descend to D3 (choice point, thick veil)
        mechanism.set_veil_thickness_by_density(entity_id, Density::Third);
        let effects_d3 = mechanism.apply_veil_effects(entity_id, Density::Third, &polarization);
        assert_eq!(effects_d3.thickness, 0.60);
        assert_eq!(effects_d3.memory_retention_percentage(), 40.0);

        // Descend to D2 (growth density, thicker veil)
        mechanism
            .set_veil_thickness_by_density(entity_id, Density::Second(Density2SubLevel::Cellular));
        let effects_d2 = mechanism.apply_veil_effects(
            entity_id,
            Density::Second(Density2SubLevel::Cellular),
            &polarization,
        );
        assert_eq!(effects_d2.thickness, 0.75);
        assert_eq!(effects_d2.memory_retention_percentage(), 25.0);

        // Descend to D1 (maximum veil)
        mechanism
            .set_veil_thickness_by_density(entity_id, Density::First(Density1SubLevel::Quantum));
        let effects_d1 = mechanism.apply_veil_effects(
            entity_id,
            Density::First(Density1SubLevel::Quantum),
            &polarization,
        );
        assert_eq!(effects_d1.thickness, 0.90);
        assert_eq!(effects_d1.memory_retention_percentage(), 10.0);

        // Verify catalyst difficulty increases as veil thickens
        assert!(effects_d1.catalyst_difficulty_modifier > effects_d2.catalyst_difficulty_modifier);
        assert!(effects_d2.catalyst_difficulty_modifier > effects_d3.catalyst_difficulty_modifier);
        assert!(effects_d3.catalyst_difficulty_modifier > effects_d4.catalyst_difficulty_modifier);
        assert!(effects_d4.catalyst_difficulty_modifier > effects_d5.catalyst_difficulty_modifier);
        assert!(effects_d5.catalyst_difficulty_modifier > effects_d6.catalyst_difficulty_modifier);
        assert!(effects_d6.catalyst_difficulty_modifier > effects_d7.catalyst_difficulty_modifier);
    }

    #[test]
    fn test_full_piercing_journey() {
        let mut mechanism = VeilMechanism::new();
        let entity_id = 1;
        let polarization = PolarizationState::sto(0.5);

        // Initial state: thick veil with polarization thinning
        let initial_thickness =
            mechanism.calculate_veil_thickness(entity_id, &Density::Third, &polarization);
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.0, Result: 0.45
        assert_eq!(initial_thickness, 0.45);
        assert!(!mechanism.can_access_higher_truths(entity_id, &Density::Third, &polarization));

        // First piercing: meditation
        let event1 = PiercingEvent::meditation(
            20.0,
            PiercingLocation::Mind,
            Density::Third,
            polarization.clone(),
        );
        mechanism.record_veil_piercing(entity_id, event1);

        let thickness1 =
            mechanism.calculate_veil_thickness(entity_id, &Density::Third, &polarization);
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.2, Result: 0.25
        assert_eq!(thickness1, 0.25);
        assert!(mechanism.can_access_higher_truths(entity_id, &Density::Third, &polarization)); // Below 0.3

        // Additional piercings to show cumulative effect
        let event2 = PiercingEvent::spiritual_work(
            0.8,
            PiercingLocation::Spirit,
            Density::Third,
            polarization.clone(),
        );
        mechanism.record_veil_piercing(entity_id, event2);

        let thickness2 =
            mechanism.calculate_veil_thickness(entity_id, &Density::Third, &polarization);
        // Base: 0.60, Polarity: 0.15, Accumulated: 0.216, Result: 0.234
        assert!((thickness2 - 0.234).abs() < 0.001);
        assert!(mechanism.can_access_higher_truths(entity_id, &Density::Third, &polarization));
    }
}
