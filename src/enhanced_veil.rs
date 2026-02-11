// Enhanced Veil Mechanism - Phase 15
// Implements veil that creates separation illusion and enables true free will
//
// This module provides the enhanced veil mechanism that creates the illusion
// of separation from unity, limits awareness, and enables genuine free will
// choice through uncertainty. The veil thickness varies by density and
// progressively thins as entities develop.
//
// ============================================================================
// MIGRATION NOTICE (Phase 4, Migration 7)
// ============================================================================
// This module has been MIGRATED to: src/spectrum/yellow_realm.rs
//
// The following types are now available in spectrum::yellow_realm:
// - FullAwareness
// - LimitedAwareness
// - SeparationIllusion
// - VeilStatistics
// - EnhancedVeil
//
// Re-exports for backward compatibility
//
// ============================================================================

// Re-export migrated types for backward compatibility
pub use crate::spectrum::yellow_realm::{
    EnhancedVeil, FullAwareness, LimitedAwareness, SeparationIllusion, VeilStatistics,
};

// ============================================================================
// LEGACY CODE (kept for reference during migration)
// ============================================================================

/*
use crate::types::{Density, Float};

/// Full awareness (what entity would have without veil)
#[derive(Debug, Clone, PartialEq)]
pub struct FullAwareness {
    /// Awareness of time/space (metaphysical realm)
    pub timespace_awareness: Float,

    /// Awareness of holographic nature (unity, oneness)
    pub holographic_awareness: Float,

    /// Memory of all experiences (karma)
    pub memory: Float,
}

impl FullAwareness {
    /// Create new full awareness with maximum values
    pub fn new() -> Self {
        FullAwareness {
            timespace_awareness: 1.0,
            holographic_awareness: 1.0,
            memory: 1.0,
        }
    }

    /// Create full awareness with specific values
    pub fn with_values(timespace: Float, holographic: Float, memory: Float) -> Self {
        FullAwareness {
            timespace_awareness: timespace.max(0.0).min(1.0),
            holographic_awareness: holographic.max(0.0).min(1.0),
            memory: memory.max(0.0).min(1.0),
        }
    }
}

/// Limited awareness (what entity has with veil)
#[derive(Debug, Clone, PartialEq)]
pub struct LimitedAwareness {
    /// Limited awareness of time/space
    pub timespace_awareness: Float,

    /// Limited awareness of holographic nature
    pub holographic_awareness: Float,

    /// Limited memory (karma limitation)
    pub memory: Float,
}

impl LimitedAwareness {
    /// Create new limited awareness with minimum values
    pub fn new() -> Self {
        LimitedAwareness {
            timespace_awareness: 0.0,
            holographic_awareness: 0.0,
            memory: 0.0,
        }
    }

    /// Create limited awareness with specific values
    pub fn with_values(timespace: Float, holographic: Float, memory: Float) -> Self {
        LimitedAwareness {
            timespace_awareness: timespace.max(0.0).min(1.0),
            holographic_awareness: holographic.max(0.0).min(1.0),
            memory: memory.max(0.0).min(1.0),
        }
    }

    /// Calculate awareness reduction from full awareness
    pub fn reduction_from_full(&self, full: &FullAwareness) -> Float {
        let timespace_reduction = (full.timespace_awareness - self.timespace_awareness)
            / full.timespace_awareness.max(0.01);
        let holographic_reduction = (full.holographic_awareness - self.holographic_awareness)
            / full.holographic_awareness.max(0.01);
        let memory_reduction = (full.memory - self.memory) / full.memory.max(0.01);

        (timespace_reduction + holographic_reduction + memory_reduction) / 3.0
    }
}

/// Separation illusion created by veil
#[derive(Debug, Clone, PartialEq)]
pub struct SeparationIllusion {
    /// Illusion of separation from other entities
    pub entity_separation: Float,

    /// Illusion of separation from infinity
    pub infinity_separation: Float,

    /// Illusion of separation from unity (oneness)
    pub unity_separation: Float,

    /// Illusion of death (fear of non-existence)
    pub death_illusion: Float,
}

impl SeparationIllusion {
    /// Create new separation illusion with minimum values
    pub fn new() -> Self {
        SeparationIllusion {
            entity_separation: 0.0,
            infinity_separation: 0.0,
            unity_separation: 0.0,
            death_illusion: 0.0,
        }
    }

    /// Create separation illusion with specific values
    pub fn with_values(entity: Float, infinity: Float, unity: Float, death: Float) -> Self {
        SeparationIllusion {
            entity_separation: entity.max(0.0).min(1.0),
            infinity_separation: infinity.max(0.0).min(1.0),
            unity_separation: unity.max(0.0).min(1.0),
            death_illusion: death.max(0.0).min(1.0),
        }
    }

    /// Calculate total separation illusion strength
    pub fn total_illusion_strength(&self) -> Float {
        (self.entity_separation
            + self.infinity_separation
            + self.unity_separation
            + self.death_illusion)
            / 4.0
    }

    /// Check if separation illusion is strong enough to enable free will
    pub fn enables_free_will(&self) -> bool {
        // Separation illusion must be strong enough (at least 0.3)
        // but not too strong (max 0.9)
        let strength = self.total_illusion_strength();
        strength >= 0.3 && strength <= 0.9
    }
}

/// Enhanced veil mechanism
#[derive(Debug, Clone)]
pub struct Veil {
    /// Base thickness (determined by density)
    pub base_thickness: Float,

    /// Current thickness (varies by entity development)
    pub current_thickness: Float,

    /// Separation illusion created by veil
    pub separation_illusion: SeparationIllusion,

    /// Limitation on time/space awareness
    pub timespace_awareness_limitation: Float,

    /// Limitation on holographic awareness
    pub holographic_awareness_limitation: Float,

    /// Memory limitation (karma)
    pub memory_limitation: Float,
}

impl Veil {
    /// Get base thickness for a specific density (public helper)
    pub fn get_base_thickness_for_density(density: Density) -> Float {
        Self::calculate_base_thickness(density)
    }

    /// Create new veil for a specific density
    pub fn new(density: Density) -> Self {
        let base_thickness = Self::calculate_base_thickness(density);

        Veil {
            base_thickness,
            current_thickness: base_thickness,
            separation_illusion: Self::calculate_separation_illusion(base_thickness),
            timespace_awareness_limitation: base_thickness * 0.5,
            holographic_awareness_limitation: base_thickness * 0.7,
            memory_limitation: base_thickness * 0.6,
        }
    }

    /// Create veil with specific thickness
    pub fn with_thickness(thickness: Float) -> Self {
        let thickness = thickness.max(0.0).min(1.0);

        Veil {
            base_thickness: thickness,
            current_thickness: thickness,
            separation_illusion: Self::calculate_separation_illusion(thickness),
            timespace_awareness_limitation: thickness * 0.5,
            holographic_awareness_limitation: thickness * 0.7,
            memory_limitation: thickness * 0.6,
        }
    }

    /// Calculate base thickness based on density
    /// Updated per Comprehensive Analysis Report Phase 1.1 requirements
    fn calculate_base_thickness(density: Density) -> Float {
        match density {
            Density::First(Density1SubLevel::Quantum) => 0.9,  // Thickest - elemental awareness
            Density::Second(Density2SubLevel::Cellular) => 0.7,  // Growth and movement awareness
            Density::Third => 0.5,  // Self-aware awareness (human level)
            Density::Fourth => 0.3,  // Love and understanding awareness
            Density::Fifth => 0.2,  // Light and wisdom awareness
            Density::Sixth => 0.1,  // Unity awareness
            Density::Seventh => 0.05, // Thinnest - gateway awareness
        }
    }

    /// Calculate separation illusion based on thickness
    fn calculate_separation_illusion(thickness: Float) -> SeparationIllusion {
        SeparationIllusion {
            entity_separation: thickness,
            infinity_separation: thickness * 0.9,
            unity_separation: thickness * 0.8,
            death_illusion: thickness * 0.7,
        }
    }

    /// Create separation illusion based on current veil thickness
    pub fn create_separation_illusion(&self) -> SeparationIllusion {
        Self::calculate_separation_illusion(self.current_thickness)
    }

    /// Limit awareness based on veil thickness
    pub fn limit_awareness(&self, awareness: FullAwareness) -> LimitedAwareness {
        LimitedAwareness {
            timespace_awareness: awareness.timespace_awareness
                * (1.0 - self.timespace_awareness_limitation),
            holographic_awareness: awareness.holographic_awareness
                * (1.0 - self.holographic_awareness_limitation),
            memory: awareness.memory * (1.0 - self.memory_limitation),
        }
    }

    /// Thin the veil by a specific amount
    pub fn thin(&mut self, amount: Float) {
        self.current_thickness = (self.current_thickness - amount).max(0.0);
        // Update separation illusion
        self.separation_illusion = self.create_separation_illusion();
    }

    /// Thicken the veil by a specific amount
    pub fn thicken(&mut self, amount: Float) {
        self.current_thickness = (self.current_thickness + amount).min(self.base_thickness);
        // Update separation illusion
        self.separation_illusion = self.create_separation_illusion();
    }

    /// Get current thickness
    pub fn get_thickness(&self) -> Float {
        self.current_thickness
    }

    /// Get base thickness
    pub fn get_base_thickness(&self) -> Float {
        self.base_thickness
    }

    /// Get transparency (1.0 - thickness)
    pub fn get_transparency(&self) -> Float {
        1.0 - self.current_thickness
    }

    /// Reset veil to base thickness
    pub fn reset(&mut self) {
        self.current_thickness = self.base_thickness;
        self.separation_illusion = self.create_separation_illusion();
    }

    /// Update veil thickness based on polarization intensity
    /// Phase 1.1: Implement automatic veil thinning based on polarization
    ///
    /// # Arguments
    /// * `polarization_intensity` - Polarity intensity (0.0 = neutral, 1.0 = fully polarized)
    ///
    /// Higher polarization intensity leads to thinner veil (both STO and STS)
    pub fn update_based_on_polarization(&mut self, polarization_intensity: Float) {
        // Polarity intensity: 0.0 (neutral) to 1.0 (fully polarized)
        // Higher polarization = thinner veil
        let thinning_amount = polarization_intensity * self.base_thickness * 0.5;
        self.current_thickness =
            (self.base_thickness - thinning_amount).max(self.base_thickness * 0.1);

        // Update separation illusion when thickness changes
        self.separation_illusion = self.create_separation_illusion();
    }

    /// Check if veil enables genuine free will
    pub fn enables_free_will(&self) -> bool {
        // Veil must be thick enough to create uncertainty
        // but thin enough to allow choice
        self.current_thickness >= 0.3 && self.current_thickness <= 0.9
    }

    /// Calculate free will capacity based on veil thickness
    pub fn free_will_capacity(&self) -> Float {
        // Optimal free will at medium thickness (0.5-0.7)
        // Too thin = too much certainty (deterministic)
        // Too thick = too much confusion (no capacity to choose)
        let thickness = self.current_thickness;

        if thickness < 0.3 {
            // Too thin - too much certainty
            thickness / 0.3
        } else if thickness > 0.9 {
            // Too thick - too much confusion
            (1.0 - thickness) / 0.1
        } else {
            // Optimal range
            1.0 - ((thickness - 0.6).abs() / 0.3)
        }
    }

    /// Calculate veil influence on STO/STS choice
    pub fn polarization_influence(&self) -> Float {
        // Thicker veil tends toward STS (fear, scarcity, separation)
        // Thinner veil tends toward STO (love, unity, connection)
        // Neutral at 0.5 thickness
        (0.5 - self.current_thickness) * 2.0
        // Positive = STO tendency, Negative = STS tendency
    }

    /// Get veil statistics
    pub fn statistics(&self) -> VeilStatistics {
        VeilStatistics {
            base_thickness: self.base_thickness,
            current_thickness: self.current_thickness,
            thinning_progress: (self.base_thickness - self.current_thickness) / self.base_thickness,
            free_will_capacity: self.free_will_capacity(),
            separation_illusion_strength: self.separation_illusion.total_illusion_strength(),
            enables_free_will: self.enables_free_will(),
            polarization_influence: self.polarization_influence(),
        }
    }
}

/// Veil statistics for analysis
#[derive(Debug, Clone, PartialEq)]
pub struct VeilStatistics {
    pub base_thickness: Float,
    pub current_thickness: Float,
    pub thinning_progress: Float,
    pub free_will_capacity: Float,
    pub separation_illusion_strength: Float,
    pub enables_free_will: bool,
    pub polarization_influence: Float,
}

/// Integration with DecisionEngine
impl Veil {
    /// Apply veil influence to decision engine
    pub fn apply_to_decision_engine(&self, engine: &mut DecisionEngine) {
        // Update veil thickness in decision engine
        engine.veil_thickness = self.current_thickness;

        // Veil thickness affects free will capacity
        // Thicker veil = more uncertainty = more genuine choice
        // Thinner veil = less uncertainty = more deterministic
        let free_will_adjustment = self.free_will_capacity();
        engine.free_will_capacity = free_will_adjustment;
    }

    /// Calculate veil-modified STO weight
    pub fn modified_sto_weight(&self, base_sto_weight: Float) -> Float {
        // Thinner veil = higher STO tendency
        // Thicker veil = lower STO tendency
        base_sto_weight * (1.0 + self.polarization_influence() * 0.3)
    }

    /// Calculate veil-modified STS weight
    pub fn modified_sts_weight(&self, base_sts_weight: Float) -> Float {
        // Thicker veil = higher STS tendency
        // Thinner veil = lower STS tendency
        base_sts_weight * (1.0 - self.polarization_influence() * 0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_awareness_creation() {
        let awareness = FullAwareness::new();
        assert_eq!(awareness.timespace_awareness, 1.0);
        assert_eq!(awareness.holographic_awareness, 1.0);
        assert_eq!(awareness.memory, 1.0);
    }

    #[test]
    fn test_full_awareness_with_values() {
        let awareness = FullAwareness::with_values(0.8, 0.9, 0.7);
        assert_eq!(awareness.timespace_awareness, 0.8);
        assert_eq!(awareness.holographic_awareness, 0.9);
        assert_eq!(awareness.memory, 0.7);
    }

    #[test]
    fn test_full_awareness_clamping() {
        let awareness = FullAwareness::with_values(1.5, -0.5, 0.5);
        assert_eq!(awareness.timespace_awareness, 1.0);
        assert_eq!(awareness.holographic_awareness, 0.0);
        assert_eq!(awareness.memory, 0.5);
    }

    #[test]
    fn test_limited_awareness_creation() {
        let awareness = LimitedAwareness::new();
        assert_eq!(awareness.timespace_awareness, 0.0);
        assert_eq!(awareness.holographic_awareness, 0.0);
        assert_eq!(awareness.memory, 0.0);
    }

    #[test]
    fn test_limited_awareness_with_values() {
        let awareness = LimitedAwareness::with_values(0.5, 0.4, 0.6);
        assert_eq!(awareness.timespace_awareness, 0.5);
        assert_eq!(awareness.holographic_awareness, 0.4);
        assert_eq!(awareness.memory, 0.6);
    }

    #[test]
    fn test_limited_awareness_reduction() {
        let full = FullAwareness::new();
        let limited = LimitedAwareness::with_values(0.5, 0.4, 0.6);
        let reduction = limited.reduction_from_full(&full);

        assert!(reduction > 0.0);
        assert!(reduction < 1.0);
    }

    #[test]
    fn test_separation_illusion_creation() {
        let illusion = SeparationIllusion::new();
        assert_eq!(illusion.entity_separation, 0.0);
        assert_eq!(illusion.infinity_separation, 0.0);
        assert_eq!(illusion.unity_separation, 0.0);
        assert_eq!(illusion.death_illusion, 0.0);
    }

    #[test]
    fn test_separation_illusion_with_values() {
        let illusion = SeparationIllusion::with_values(0.7, 0.6, 0.5, 0.4);
        assert_eq!(illusion.entity_separation, 0.7);
        assert_eq!(illusion.infinity_separation, 0.6);
        assert_eq!(illusion.unity_separation, 0.5);
        assert_eq!(illusion.death_illusion, 0.4);
    }

    #[test]
    fn test_separation_illusion_total_strength() {
        let illusion = SeparationIllusion::with_values(0.7, 0.6, 0.5, 0.4);
        let strength = illusion.total_illusion_strength();
        assert_eq!(strength, (0.7 + 0.6 + 0.5 + 0.4) / 4.0);
    }

    #[test]
    fn test_separation_illusion_enables_free_will() {
        let weak_illusion = SeparationIllusion::with_values(0.2, 0.2, 0.2, 0.2);
        assert!(!weak_illusion.enables_free_will());

        let optimal_illusion = SeparationIllusion::with_values(0.5, 0.5, 0.5, 0.5);
        assert!(optimal_illusion.enables_free_will());

        let strong_illusion = SeparationIllusion::with_values(0.95, 0.95, 0.95, 0.95);
        assert!(!strong_illusion.enables_free_will());
    }

    #[test]
    fn test_veil_creation_d1() {
        let veil = Veil::new(Density::First(Density1SubLevel::Quantum));
        assert_eq!(veil.base_thickness, 0.9);
        assert_eq!(veil.current_thickness, 0.9);
        assert!(veil.separation_illusion.entity_separation > 0.0);
    }

    #[test]
    fn test_veil_creation_d3() {
        let veil = Veil::new(Density::Third);
        // Updated per Phase 1.1: D3 thickness is now 0.5
        assert_eq!(veil.base_thickness, 0.5);
        assert_eq!(veil.current_thickness, 0.5);
        assert!(veil.separation_illusion.entity_separation > 0.0);
    }

    #[test]
    fn test_veil_creation_d7() {
        let veil = Veil::new(Density::Seventh);
        // Updated per Phase 1.1: D7 thickness is now 0.05
        assert_eq!(veil.base_thickness, 0.05);
        assert_eq!(veil.current_thickness, 0.05);
        assert!(veil.separation_illusion.entity_separation > 0.0);
    }

    #[test]
    fn test_veil_with_thickness() {
        let veil = Veil::with_thickness(0.6);
        assert_eq!(veil.base_thickness, 0.6);
        assert_eq!(veil.current_thickness, 0.6);
    }

    #[test]
    fn test_veil_with_thickness_clamping() {
        let veil = Veil::with_thickness(1.5);
        assert_eq!(veil.base_thickness, 1.0);
        assert_eq!(veil.current_thickness, 1.0);

        let veil = Veil::with_thickness(-0.5);
        assert_eq!(veil.base_thickness, 0.0);
        assert_eq!(veil.current_thickness, 0.0);
    }

    #[test]
    fn test_create_separation_illusion() {
        let veil = Veil::new(Density::Third);
        let illusion = veil.create_separation_illusion();

        assert!(illusion.entity_separation > 0.0);
        assert!(illusion.infinity_separation > 0.0);
        assert!(illusion.unity_separation > 0.0);
        assert!(illusion.death_illusion > 0.0);

        // Entity separation should equal current thickness
        assert_eq!(illusion.entity_separation, veil.current_thickness);
    }

    #[test]
    fn test_limit_awareness() {
        let veil = Veil::new(Density::Third);
        let full = FullAwareness::new();
        let limited = veil.limit_awareness(full.clone());

        // Limited awareness should be less than full
        assert!(limited.timespace_awareness < full.timespace_awareness);
        assert!(limited.holographic_awareness < full.holographic_awareness);
        assert!(limited.memory < full.memory);

        // All should be greater than 0
        assert!(limited.timespace_awareness > 0.0);
        assert!(limited.holographic_awareness > 0.0);
        assert!(limited.memory > 0.0);
    }

    #[test]
    fn test_thin_veil() {
        let mut veil = Veil::new(Density::Third);
        let initial_thickness = veil.current_thickness;
        let initial_illusion = veil.separation_illusion.entity_separation;

        veil.thin(0.2);

        assert!(veil.current_thickness < initial_thickness);
        assert!(veil.separation_illusion.entity_separation < initial_illusion);
    }

    #[test]
    fn test_thin_veil_below_zero() {
        let mut veil = Veil::new(Density::First(Density1SubLevel::Quantum));
        veil.thin(1.5);

        assert_eq!(veil.current_thickness, 0.0);
    }

    #[test]
    fn test_thicken_veil() {
        let mut veil = Veil::new(Density::Third);
        veil.thin(0.3);

        let initial_thickness = veil.current_thickness;
        veil.thicken(0.1);

        assert!(veil.current_thickness > initial_thickness);
    }

    #[test]
    fn test_thicken_veil_above_base() {
        let mut veil = Veil::new(Density::Third);
        veil.thicken(0.5);

        assert_eq!(veil.current_thickness, veil.base_thickness);
    }

    #[test]
    fn test_reset_veil() {
        let mut veil = Veil::new(Density::Third);
        veil.thin(0.3);

        veil.reset();

        assert_eq!(veil.current_thickness, veil.base_thickness);
    }

    #[test]
    fn test_enables_free_will() {
        let thin_veil = Veil::with_thickness(0.2);
        assert!(!thin_veil.enables_free_will());

        let optimal_veil = Veil::with_thickness(0.6);
        assert!(optimal_veil.enables_free_will());

        let thick_veil = Veil::with_thickness(0.95);
        assert!(!thick_veil.enables_free_will());
    }

    #[test]
    fn test_free_will_capacity() {
        let thin_veil = Veil::with_thickness(0.2);
        let thin_capacity = thin_veil.free_will_capacity();
        assert!(thin_capacity < 1.0);

        let optimal_veil = Veil::with_thickness(0.6);
        let optimal_capacity = optimal_veil.free_will_capacity();
        assert!(optimal_capacity > thin_capacity);

        let thick_veil = Veil::with_thickness(0.95);
        let thick_capacity = thick_veil.free_will_capacity();
        assert!(thick_capacity < optimal_capacity);
    }

    #[test]
    fn test_polarization_influence() {
        let thin_veil = Veil::with_thickness(0.3);
        let thin_influence = thin_veil.polarization_influence();
        assert!(thin_influence > 0.0); // STO tendency

        let neutral_veil = Veil::with_thickness(0.5);
        let neutral_influence = neutral_veil.polarization_influence();
        assert_eq!(neutral_influence, 0.0); // Neutral

        let thick_veil = Veil::with_thickness(0.7);
        let thick_influence = thick_veil.polarization_influence();
        assert!(thick_influence < 0.0); // STS tendency
    }

    #[test]
    fn test_modified_sto_weight() {
        let veil = Veil::with_thickness(0.3);
        let base_weight = 0.5;
        let modified = veil.modified_sto_weight(base_weight);

        assert!(modified > base_weight); // Thinner veil increases STO
    }

    #[test]
    fn test_modified_sts_weight() {
        let veil = Veil::with_thickness(0.7);
        let base_weight = 0.5;
        let modified = veil.modified_sts_weight(base_weight);

        assert!(modified > base_weight); // Thicker veil increases STS
    }

    #[test]
    fn test_veil_statistics() {
        let veil = Veil::new(Density::Third);
        let stats = veil.statistics();

        // Updated per Phase 1.1: D3 thickness is now 0.5
        assert_eq!(stats.base_thickness, 0.5);
        assert_eq!(stats.current_thickness, 0.5);
        assert!(stats.free_will_capacity > 0.0);
        assert!(stats.separation_illusion_strength > 0.0);
    }

    #[test]
    fn test_density_thickness_progression() {
        let d1_veil = Veil::new(Density::First(Density1SubLevel::Quantum));
        let d3_veil = Veil::new(Density::Third);
        let d7_veil = Veil::new(Density::Seventh);

        assert!(d1_veil.base_thickness > d3_veil.base_thickness);
        assert!(d3_veil.base_thickness > d7_veil.base_thickness);

        // Updated per Phase 1.1
        assert_eq!(d1_veil.base_thickness, 0.9);
        assert_eq!(d3_veil.base_thickness, 0.5);
        assert_eq!(d7_veil.base_thickness, 0.05);
    }

    #[test]
    fn test_awareness_limitation_values() {
        let veil = Veil::new(Density::Third);

        // Updated per Phase 1.1: D3 thickness is now 0.5
        // Timespace limitation should be 50% of thickness
        assert!(
            (veil.timespace_awareness_limitation - 0.25).abs() < 0.001,
            "Expected ≈0.25, got {}",
            veil.timespace_awareness_limitation
        );

        // Holographic limitation should be 70% of thickness
        assert!(
            (veil.holographic_awareness_limitation - 0.35).abs() < 0.001,
            "Expected ≈0.35, got {}",
            veil.holographic_awareness_limitation
        );

        // Memory limitation should be 60% of thickness
        assert!(
            (veil.memory_limitation - 0.3).abs() < 0.001,
            "Expected ≈0.3, got {}",
            veil.memory_limitation
        );
    }

    #[test]
    fn test_separation_illusion_progression() {
        let veil = Veil::new(Density::Third);
        let illusion = veil.separation_illusion;

        // Updated per Phase 1.1: D3 thickness is now 0.5
        // Entity separation should equal thickness
        assert!(
            (illusion.entity_separation - 0.5).abs() < 0.001,
            "Expected ≈0.5, got {}",
            illusion.entity_separation
        );

        // Infinity separation should be 90% of thickness
        assert!(
            (illusion.infinity_separation - 0.45).abs() < 0.001,
            "Expected ≈0.45, got {}",
            illusion.infinity_separation
        );

        // Unity separation should be 80% of thickness
        assert!(
            (illusion.unity_separation - 0.4).abs() < 0.001,
            "Expected ≈0.4, got {}",
            illusion.unity_separation
        );

        // Death illusion should be 70% of thickness
        assert!(
            (illusion.death_illusion - 0.35).abs() < 0.001,
            "Expected ≈0.35, got {}",
            illusion.death_illusion
        );
    }

    #[test]
    fn test_awareness_limitation_by_density() {
        let d1_veil = Veil::new(Density::First(Density1SubLevel::Quantum));
        let d7_veil = Veil::new(Density::Seventh);

        let full = FullAwareness::new();
        let d1_limited = d1_veil.limit_awareness(full.clone());
        let d7_limited = d7_veil.limit_awareness(full);

        // D1 should have more limitation (thicker veil)
        assert!(d1_limited.timespace_awareness < d7_limited.timespace_awareness);
        assert!(d1_limited.holographic_awareness < d7_limited.holographic_awareness);
        assert!(d1_limited.memory < d7_limited.memory);
    }

    #[test]
    fn test_progressive_thinning() {
        let mut veil = Veil::new(Density::First(Density1SubLevel::Quantum));
        let initial_thickness = veil.current_thickness;

        veil.thin(0.1);
        assert!(veil.current_thickness < initial_thickness);

        veil.thin(0.1);
        assert!(veil.current_thickness < initial_thickness - 0.1);

        veil.thin(0.1);
        assert!(veil.current_thickness < initial_thickness - 0.2);
    }
}
*/
