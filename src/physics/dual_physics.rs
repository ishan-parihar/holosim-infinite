// Dual-Mode Physics System - Phase 4: Integration & Migration, Task 4.1
//
// This module implements a dual-mode physics system that can switch between:
// - Hardcoded mode: Uses existing derive_* functions from physics_derivation.rs
// - Holographic mode: Uses holographic discovery from holographic modules
// - Hybrid mode: Uses both and compares results
//
// Key Principles:
// 1. Backward compatibility maintained during migration
// 2. Seamless switching between physics modes
// 3. Comparison and validation of holographic results
// 4. Gradual migration path from hardcoded to holographic
//
// Knowledge Base References:
// - REFACTOR_ROADMAP_HOLOGRAPHIC_ARCHITECTURE.md Phase 4, Task 4.1
// - "Integration & Migration: Switch between hardcoded and holographic physics"

use crate::holographic::{ConfigurationDiscoveryEngine, HolographicField, InvolutionLayer};
use crate::physics_derivation::{
    derive_charge_from_archetypes, derive_lifetime_from_archetypes, derive_mass_from_archetypes,
    derive_spin_from_archetypes,
};
use crate::types::Float;

// ============================================================================
// PHYSICS MODE ENUM
// ============================================================================

/// Physics calculation mode
///
/// Determines which physics engine to use for property calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsMode {
    /// Use existing hardcoded derive_* functions
    Hardcoded,

    /// Use holographic discovery from holographic modules
    Holographic,

    /// Use both engines and compare/merge results
    Hybrid,
}

impl PhysicsMode {
    /// Get all physics modes
    pub fn all_modes() -> Vec<PhysicsMode> {
        vec![
            PhysicsMode::Hardcoded,
            PhysicsMode::Holographic,
            PhysicsMode::Hybrid,
        ]
    }

    /// Get mode name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            PhysicsMode::Hardcoded => "Hardcoded",
            PhysicsMode::Holographic => "Holographic",
            PhysicsMode::Hybrid => "Hybrid",
        }
    }
}

// ============================================================================
// PARTICLE PROPERTIES
// ============================================================================

/// Particle properties derived from archetype activation
///
/// This struct represents the physical properties of a particle
/// that can be derived either from hardcoded formulas or holographic discovery
#[derive(Debug, Clone, PartialEq)]
pub struct ParticleProperties {
    /// Mass in kilograms
    pub mass: Float,

    /// Charge in elementary charge units (e)
    pub charge: Float,

    /// Spin in units of ħ/2
    pub spin: Float,

    /// Lifetime in seconds (unstable particles only)
    pub lifetime: Float,

    /// Stability score (0.0 to 1.0, higher = more stable)
    pub stability_score: Float,

    /// Resonance score (0.0 to 1.0, holographic mode only)
    pub resonance_score: Option<Float>,

    /// Source mode that generated these properties
    pub source_mode: PhysicsMode,
}

impl ParticleProperties {
    /// Create new particle properties
    pub fn new(
        mass: Float,
        charge: Float,
        spin: Float,
        lifetime: Float,
        stability_score: Float,
        source_mode: PhysicsMode,
    ) -> Self {
        Self {
            mass,
            charge,
            spin,
            lifetime,
            stability_score,
            resonance_score: None,
            source_mode,
        }
    }

    /// Create particle properties with resonance score
    pub fn with_resonance(
        mass: Float,
        charge: Float,
        spin: Float,
        lifetime: Float,
        stability_score: Float,
        resonance_score: Float,
        source_mode: PhysicsMode,
    ) -> Self {
        Self {
            mass,
            charge,
            spin,
            lifetime,
            stability_score,
            resonance_score: Some(resonance_score),
            source_mode,
        }
    }

    /// Check if properties are physically valid
    pub fn is_valid(&self) -> bool {
        // Mass should be positive
        if self.mass <= 0.0 {
            return false;
        }

        // Stability score should be between 0 and 1
        if self.stability_score < 0.0 || self.stability_score > 1.0 {
            return false;
        }

        // Resonance score should be between 0 and 1 (if present)
        if let Some(resonance) = self.resonance_score {
            if resonance < 0.0 || resonance > 1.0 {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// HARDCODED PHYSICS ENGINE WRAPPER
// ============================================================================

/// Wrapper for hardcoded physics derivation functions
///
/// This wraps the existing derive_* functions from physics_derivation.rs
/// to provide a consistent interface with the holographic engine
#[derive(Debug, Clone)]
pub struct HardcodedPhysicsEngine;

impl HardcodedPhysicsEngine {
    /// Create new hardcoded physics engine
    pub fn new() -> Self {
        Self
    }

    /// Calculate particle properties from archetype activation
    ///
    /// Uses the existing derive_* functions from physics_derivation.rs
    pub fn calculate_properties(&self, activation: &[Float; 22]) -> ParticleProperties {
        let mass = derive_mass_from_archetypes(activation);
        let charge = derive_charge_from_archetypes(activation);
        let spin = derive_spin_from_archetypes(activation);
        let lifetime = derive_lifetime_from_archetypes(activation).unwrap_or(1e-10); // Handle Option

        // Calculate stability from mass and lifetime
        // Stable particles have long lifetimes
        let stability_score = if lifetime > 1e-6 {
            1.0 - (1.0 / (1.0 + lifetime * 1e6))
        } else {
            0.5
        };

        ParticleProperties::new(
            mass,
            charge,
            spin,
            lifetime,
            stability_score,
            PhysicsMode::Hardcoded,
        )
    }

    /// Calculate force between two particles (simplified)
    pub fn calculate_force(
        &self,
        _pos1: [Float; 3],
        _mass1: Float,
        _charge1: Float,
        _pos2: [Float; 3],
        _mass2: Float,
        _charge2: Float,
    ) -> [Float; 3] {
        // Simplified force calculation
        // In a full implementation, this would use the physics_engine.rs
        [0.0, 0.0, 0.0]
    }
}

// ============================================================================
// HOLOGRAPHIC PHYSICS ENGINE WRAPPER
// ============================================================================

/// Wrapper for holographic physics discovery
///
/// This wraps the holographic modules (HolographicField, ConfigurationDiscoveryEngine)
/// to provide a consistent interface with the hardcoded engine
#[derive(Debug, Clone)]
pub struct HolographicPhysicsEngine {
    /// Configuration discovery engine
    discovery_engine: ConfigurationDiscoveryEngine,

    /// Resonance threshold for configuration discovery
    resonance_threshold: Float,
}

impl HolographicPhysicsEngine {
    /// Create new holographic physics engine
    pub fn new() -> Self {
        Self {
            discovery_engine: ConfigurationDiscoveryEngine::new(),
            resonance_threshold: 0.8,
        }
    }

    /// Create new holographic physics engine with custom threshold
    pub fn with_threshold(threshold: Float) -> Self {
        Self {
            discovery_engine: ConfigurationDiscoveryEngine::new(),
            resonance_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Set resonance threshold
    pub fn set_resonance_threshold(&mut self, threshold: Float) {
        self.resonance_threshold = threshold.clamp(0.0, 1.0);
        self.discovery_engine.set_resonance_threshold(threshold);
    }

    /// Generate holographic field from archetype activation
    ///
    /// Converts archetype activation to complex vectors and generates field
    pub fn generate_field(
        &self,
        activation: &[Float; 22],
        layer: InvolutionLayer,
    ) -> HolographicField {
        // Convert archetype activation to complex archetypes
        use crate::holographic::ComplexArchetype;

        let archetypes: [ComplexArchetype; 22] = activation
            .iter()
            .map(|&value| ComplexArchetype::new(value, 0.0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        HolographicField::new(layer, archetypes)
    }

    /// Calculate particle properties from archetype activation
    ///
    /// Uses holographic discovery to find stable configurations
    /// and extracts properties from the best configuration
    pub fn calculate_properties(&self, activation: &[Float; 22]) -> ParticleProperties {
        // Generate holographic field at Red layer (matter)
        let field = self.generate_field(activation, InvolutionLayer::Red);

        // Discover stable configurations
        let configurations = self.discovery_engine.discover_configurations(&field);

        // Extract properties from best configuration
        if let Some(best_config) = configurations.first() {
            // Map configuration to particle properties
            let mass = self.extract_mass_from_config(best_config, activation);
            let charge = self.extract_charge_from_config(best_config, activation);
            let spin = self.extract_spin_from_config(best_config, activation);
            let lifetime = self.extract_lifetime_from_config(best_config, activation);
            let stability_score = best_config.resonance_score;
            let resonance_score = best_config.resonance_score;

            ParticleProperties::with_resonance(
                mass,
                charge,
                spin,
                lifetime,
                stability_score,
                resonance_score,
                PhysicsMode::Holographic,
            )
        } else {
            // Fallback: use hardcoded values if no configurations found
            // This ensures backward compatibility
            let hardcoded = HardcodedPhysicsEngine::new();
            let props = hardcoded.calculate_properties(activation);
            ParticleProperties::with_resonance(
                props.mass,
                props.charge,
                props.spin,
                props.lifetime,
                props.stability_score,
                0.0, // Low resonance since no configuration found
                PhysicsMode::Holographic,
            )
        }
    }

    /// Extract mass from configuration
    fn extract_mass_from_config(
        &self,
        config: &crate::holographic::StableConfiguration,
        activation: &[Float; 22],
    ) -> Float {
        // Mass emerges from configuration position and internal structure
        // Use resonance score to modulate mass
        let base_mass = derive_mass_from_archetypes(activation);
        base_mass * (0.9 + config.resonance_score * 0.2) // 0.9 to 1.1 range
    }

    /// Extract charge from configuration
    fn extract_charge_from_config(
        &self,
        config: &crate::holographic::StableConfiguration,
        activation: &[Float; 22],
    ) -> Float {
        // Charge emerges from configuration position
        // Use interference alignment to modulate charge
        let base_charge = derive_charge_from_archetypes(activation);
        base_charge * config.interference_alignment
    }

    /// Extract spin from configuration
    fn extract_spin_from_config(
        &self,
        config: &crate::holographic::StableConfiguration,
        activation: &[Float; 22],
    ) -> Float {
        // Spin emerges from configuration internal structure
        // Use spatial frequency to modulate spin
        let base_spin = derive_spin_from_archetypes(activation);
        let frequency_factor = config.spatial_frequency / 1000.0;
        base_spin * frequency_factor
    }

    /// Extract lifetime from configuration
    fn extract_lifetime_from_config(
        &self,
        config: &crate::holographic::StableConfiguration,
        activation: &[Float; 22],
    ) -> Float {
        // Lifetime emerges from configuration stability
        let base_lifetime = derive_lifetime_from_archetypes(activation).unwrap_or(1e-10);
        // Higher resonance = longer lifetime
        base_lifetime * (1.0 + config.resonance_score)
    }

    /// Calculate force between two particles (simplified)
    pub fn calculate_force(
        &self,
        _pos1: [Float; 3],
        _mass1: Float,
        _charge1: Float,
        _pos2: [Float; 3],
        _mass2: Float,
        _charge2: Float,
    ) -> [Float; 3] {
        // Simplified force calculation
        // In a full implementation, this would use holographic field interactions
        [0.0, 0.0, 0.0]
    }
}

impl Default for HolographicPhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DUAL PHYSICS SYSTEM
// ============================================================================

/// Dual-mode physics system
///
/// Main entry point for physics calculations that can switch between
/// hardcoded, holographic, and hybrid modes
#[derive(Debug, Clone)]
pub struct DualPhysicsSystem {
    /// Current physics mode
    pub mode: PhysicsMode,

    /// Hardcoded physics engine
    pub hardcoded_engine: HardcodedPhysicsEngine,

    /// Holographic physics engine
    pub holographic_engine: HolographicPhysicsEngine,

    /// Tolerance for hybrid mode comparison
    pub comparison_tolerance: Float,
}

impl DualPhysicsSystem {
    /// Create new dual physics system in hardcoded mode
    pub fn new() -> Self {
        Self {
            mode: PhysicsMode::Hardcoded,
            hardcoded_engine: HardcodedPhysicsEngine::new(),
            holographic_engine: HolographicPhysicsEngine::new(),
            comparison_tolerance: 0.1, // 10% tolerance
        }
    }

    /// Create new dual physics system with specified mode
    pub fn with_mode(mode: PhysicsMode) -> Self {
        Self {
            mode,
            hardcoded_engine: HardcodedPhysicsEngine::new(),
            holographic_engine: HolographicPhysicsEngine::new(),
            comparison_tolerance: 0.1,
        }
    }

    /// Create new dual physics system with custom tolerance
    pub fn with_tolerance(mode: PhysicsMode, tolerance: Float) -> Self {
        Self {
            mode,
            hardcoded_engine: HardcodedPhysicsEngine::new(),
            holographic_engine: HolographicPhysicsEngine::new(),
            comparison_tolerance: tolerance.abs().min(1.0),
        }
    }

    /// Set physics mode
    pub fn set_mode(&mut self, mode: PhysicsMode) {
        self.mode = mode;
    }

    /// Get current physics mode
    pub fn get_mode(&self) -> PhysicsMode {
        self.mode
    }

    /// Set comparison tolerance
    pub fn set_comparison_tolerance(&mut self, tolerance: Float) {
        self.comparison_tolerance = tolerance.abs().min(1.0);
    }

    /// Calculate particle properties based on current mode
    ///
    /// In Hardcoded mode: uses derive_* functions
    /// In Holographic mode: uses holographic discovery
    /// In Hybrid mode: uses both and compares/merges results
    pub fn calculate_particle_properties(&self, activation: &[Float; 22]) -> ParticleProperties {
        match self.mode {
            PhysicsMode::Hardcoded => self.hardcoded_engine.calculate_properties(activation),
            PhysicsMode::Holographic => self.holographic_engine.calculate_properties(activation),
            PhysicsMode::Hybrid => {
                let hardcoded = self.hardcoded_engine.calculate_properties(activation);
                let holographic = self.holographic_engine.calculate_properties(activation);
                self.compare_and_merge(hardcoded, holographic)
            }
        }
    }

    /// Calculate force between two particles based on current mode
    pub fn calculate_force(
        &self,
        pos1: [Float; 3],
        mass1: Float,
        charge1: Float,
        pos2: [Float; 3],
        mass2: Float,
        charge2: Float,
    ) -> [Float; 3] {
        match self.mode {
            PhysicsMode::Hardcoded => self
                .hardcoded_engine
                .calculate_force(pos1, mass1, charge1, pos2, mass2, charge2),
            PhysicsMode::Holographic => self
                .holographic_engine
                .calculate_force(pos1, mass1, charge1, pos2, mass2, charge2),
            PhysicsMode::Hybrid => {
                // Average of both methods
                let hardcoded_force = self
                    .hardcoded_engine
                    .calculate_force(pos1, mass1, charge1, pos2, mass2, charge2);
                let holographic_force = self
                    .holographic_engine
                    .calculate_force(pos1, mass1, charge1, pos2, mass2, charge2);
                [
                    (hardcoded_force[0] + holographic_force[0]) / 2.0,
                    (hardcoded_force[1] + holographic_force[1]) / 2.0,
                    (hardcoded_force[2] + holographic_force[2]) / 2.0,
                ]
            }
        }
    }

    /// Compare and merge results from both engines
    ///
    /// If results agree within tolerance, use holographic
    /// If results disagree, use hardcoded (fallback)
    fn compare_and_merge(
        &self,
        hardcoded: ParticleProperties,
        holographic: ParticleProperties,
    ) -> ParticleProperties {
        // Calculate relative differences
        let mass_diff = (hardcoded.mass - holographic.mass).abs() / hardcoded.mass;
        let charge_diff =
            (hardcoded.charge - holographic.charge).abs() / hardcoded.charge.abs().max(1e-10);
        let spin_diff = (hardcoded.spin - holographic.spin).abs() / hardcoded.spin.abs().max(1e-10);

        // Check if results agree within tolerance
        let mass_agrees = mass_diff < self.comparison_tolerance;
        let charge_agrees = charge_diff < self.comparison_tolerance;
        let spin_agrees = spin_diff < self.comparison_tolerance;

        // If all properties agree, use holographic (prefer holographic)
        if mass_agrees && charge_agrees && spin_agrees {
            // Merge: use holographic properties but mark as hybrid
            ParticleProperties::with_resonance(
                holographic.mass,
                holographic.charge,
                holographic.spin,
                holographic.lifetime,
                holographic.stability_score,
                holographic.resonance_score.unwrap_or(0.0),
                PhysicsMode::Hybrid,
            )
        } else {
            // Results disagree - use hardcoded as fallback
            // But include holographic resonance for information
            ParticleProperties::with_resonance(
                hardcoded.mass,
                hardcoded.charge,
                hardcoded.spin,
                hardcoded.lifetime,
                hardcoded.stability_score,
                holographic.resonance_score.unwrap_or(0.0),
                PhysicsMode::Hybrid,
            )
        }
    }

    /// Get comparison report for last calculation
    ///
    /// Compares hardcoded and holographic results
    pub fn get_comparison_report(&self, activation: &[Float; 22]) -> ComparisonReport {
        let hardcoded = self.hardcoded_engine.calculate_properties(activation);
        let holographic = self.holographic_engine.calculate_properties(activation);

        let mass_diff = (hardcoded.mass - holographic.mass).abs() / hardcoded.mass;
        let charge_diff =
            (hardcoded.charge - holographic.charge).abs() / hardcoded.charge.abs().max(1e-10);
        let spin_diff = (hardcoded.spin - holographic.spin).abs() / hardcoded.spin.abs().max(1e-10);

        ComparisonReport {
            hardcoded,
            holographic,
            mass_difference: mass_diff,
            charge_difference: charge_diff,
            spin_difference: spin_diff,
            agreement: mass_diff < self.comparison_tolerance
                && charge_diff < self.comparison_tolerance
                && spin_diff < self.comparison_tolerance,
        }
    }
}

impl Default for DualPhysicsSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// COMPARISON REPORT
// ============================================================================

/// Report comparing hardcoded and holographic results
#[derive(Debug, Clone)]
pub struct ComparisonReport {
    /// Hardcoded results
    pub hardcoded: ParticleProperties,

    /// Holographic results
    pub holographic: ParticleProperties,

    /// Relative mass difference
    pub mass_difference: Float,

    /// Relative charge difference
    pub charge_difference: Float,

    /// Relative spin difference
    pub spin_difference: Float,

    /// Whether results agree within tolerance
    pub agreement: bool,
}

impl ComparisonReport {
    /// Get summary of comparison
    pub fn summary(&self) -> String {
        format!(
            "Mass diff: {:.2}%, Charge diff: {:.2}%, Spin diff: {:.2}%, Agreement: {}",
            self.mass_difference * 100.0,
            self.charge_difference * 100.0,
            self.spin_difference * 100.0,
            if self.agreement { "✓" } else { "✗" }
        )
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate test archetype activation pattern
    fn generate_test_activation() -> [Float; 22] {
        [
            0.5, 0.6, 0.7, 0.8, 0.9, 0.5, 0.6, 0.7, 0.8, 0.9, 0.5, 0.6, 0.7, 0.8, 0.9, 0.5, 0.6,
            0.7, 0.8, 0.9, 0.5, 0.6,
        ]
    }

    #[test]
    fn test_physics_mode_enum() {
        assert_eq!(PhysicsMode::Hardcoded.as_str(), "Hardcoded");
        assert_eq!(PhysicsMode::Holographic.as_str(), "Holographic");
        assert_eq!(PhysicsMode::Hybrid.as_str(), "Hybrid");

        let all_modes = PhysicsMode::all_modes();
        assert_eq!(all_modes.len(), 3);
    }

    #[test]
    fn test_particle_properties_creation() {
        let props = ParticleProperties::new(1.0, 1.0, 0.5, 1e-6, 0.9, PhysicsMode::Hardcoded);
        assert_eq!(props.mass, 1.0);
        assert_eq!(props.charge, 1.0);
        assert_eq!(props.spin, 0.5);
        assert_eq!(props.lifetime, 1e-6);
        assert_eq!(props.stability_score, 0.9);
        assert_eq!(props.source_mode, PhysicsMode::Hardcoded);
        assert!(props.resonance_score.is_none());
    }

    #[test]
    fn test_particle_properties_with_resonance() {
        let props = ParticleProperties::with_resonance(
            1.0,
            1.0,
            0.5,
            1e-6,
            0.9,
            0.85,
            PhysicsMode::Holographic,
        );
        assert_eq!(props.mass, 1.0);
        assert_eq!(props.resonance_score, Some(0.85));
        assert_eq!(props.source_mode, PhysicsMode::Holographic);
    }

    #[test]
    fn test_particle_properties_validation() {
        // Valid properties
        let valid_props = ParticleProperties::new(1.0, 1.0, 0.5, 1e-6, 0.9, PhysicsMode::Hardcoded);
        assert!(valid_props.is_valid());

        // Invalid: negative mass
        let invalid_mass =
            ParticleProperties::new(-1.0, 1.0, 0.5, 1e-6, 0.9, PhysicsMode::Hardcoded);
        assert!(!invalid_mass.is_valid());

        // Invalid: stability score > 1
        let invalid_stability =
            ParticleProperties::new(1.0, 1.0, 0.5, 1e-6, 1.5, PhysicsMode::Hardcoded);
        assert!(!invalid_stability.is_valid());
    }

    #[test]
    fn test_hardcoded_physics_engine() {
        let engine = HardcodedPhysicsEngine::new();
        let activation = generate_test_activation();

        let props = engine.calculate_properties(&activation);

        assert!(props.is_valid());
        assert_eq!(props.source_mode, PhysicsMode::Hardcoded);
        assert!(props.mass > 0.0);
        assert!(props.lifetime >= 0.0);
    }

    #[test]
    fn test_holographic_physics_engine() {
        let engine = HolographicPhysicsEngine::new();
        let activation = generate_test_activation();

        let props = engine.calculate_properties(&activation);

        assert!(props.is_valid());
        assert_eq!(props.source_mode, PhysicsMode::Holographic);
        assert!(props.mass > 0.0);
        assert!(props.lifetime >= 0.0);
        assert!(props.resonance_score.is_some());
    }

    #[test]
    fn test_holographic_physics_engine_with_threshold() {
        let engine = HolographicPhysicsEngine::with_threshold(0.9);
        assert_eq!(engine.resonance_threshold, 0.9);

        let mut engine = HolographicPhysicsEngine::new();
        engine.set_resonance_threshold(0.7);
        assert_eq!(engine.resonance_threshold, 0.7);
    }

    #[test]
    fn test_holographic_physics_engine_generate_field() {
        let engine = HolographicPhysicsEngine::new();
        let activation = generate_test_activation();

        let field = engine.generate_field(&activation, InvolutionLayer::Red);
        assert_eq!(field.layer, InvolutionLayer::Red);
        assert_eq!(field.archetype_complex_vectors.len(), 22);
    }

    #[test]
    fn test_dual_physics_system_creation() {
        let system = DualPhysicsSystem::new();
        assert_eq!(system.mode, PhysicsMode::Hardcoded);
        assert_eq!(system.comparison_tolerance, 0.1);
    }

    #[test]
    fn test_dual_physics_system_with_mode() {
        let system = DualPhysicsSystem::with_mode(PhysicsMode::Holographic);
        assert_eq!(system.mode, PhysicsMode::Holographic);
    }

    #[test]
    fn test_dual_physics_system_mode_switching() {
        let mut system = DualPhysicsSystem::new();

        // Start in hardcoded mode
        assert_eq!(system.mode, PhysicsMode::Hardcoded);

        // Switch to holographic mode
        system.set_mode(PhysicsMode::Holographic);
        assert_eq!(system.mode, PhysicsMode::Holographic);

        // Switch to hybrid mode
        system.set_mode(PhysicsMode::Hybrid);
        assert_eq!(system.mode, PhysicsMode::Hybrid);
    }

    #[test]
    fn test_dual_physics_system_hardcoded_mode() {
        let mut system = DualPhysicsSystem::new();
        system.set_mode(PhysicsMode::Hardcoded);

        let activation = generate_test_activation();
        let props = system.calculate_particle_properties(&activation);

        assert!(props.is_valid());
        assert_eq!(props.source_mode, PhysicsMode::Hardcoded);
    }

    #[test]
    fn test_dual_physics_system_holographic_mode() {
        let mut system = DualPhysicsSystem::new();
        system.set_mode(PhysicsMode::Holographic);

        let activation = generate_test_activation();
        let props = system.calculate_particle_properties(&activation);

        assert!(props.is_valid());
        assert_eq!(props.source_mode, PhysicsMode::Holographic);
    }

    #[test]
    fn test_dual_physics_system_hybrid_mode() {
        let mut system = DualPhysicsSystem::new();
        system.set_mode(PhysicsMode::Hybrid);

        let activation = generate_test_activation();
        let props = system.calculate_particle_properties(&activation);

        assert!(props.is_valid());
        assert_eq!(props.source_mode, PhysicsMode::Hybrid);
    }

    #[test]
    fn test_dual_physics_system_comparison_tolerance() {
        let mut system = DualPhysicsSystem::new();
        system.set_comparison_tolerance(0.2);
        assert_eq!(system.comparison_tolerance, 0.2);
    }

    #[test]
    fn test_comparison_report() {
        let system = DualPhysicsSystem::new();
        let activation = generate_test_activation();

        let report = system.get_comparison_report(&activation);

        assert!(report.mass_difference >= 0.0);
        assert!(report.charge_difference >= 0.0);
        assert!(report.spin_difference >= 0.0);
        assert_eq!(report.hardcoded.source_mode, PhysicsMode::Hardcoded);
        assert_eq!(report.holographic.source_mode, PhysicsMode::Holographic);
    }

    #[test]
    fn test_comparison_report_summary() {
        let system = DualPhysicsSystem::new();
        let activation = generate_test_activation();

        let report = system.get_comparison_report(&activation);
        let summary = report.summary();

        assert!(summary.contains("Mass diff"));
        assert!(summary.contains("Charge diff"));
        assert!(summary.contains("Spin diff"));
        assert!(summary.contains("Agreement"));
    }

    #[test]
    fn test_all_modes_produce_valid_properties() {
        let mut system = DualPhysicsSystem::new();
        let activation = generate_test_activation();

        for mode in PhysicsMode::all_modes() {
            system.set_mode(mode);
            let props = system.calculate_particle_properties(&activation);
            assert!(
                props.is_valid(),
                "Properties should be valid for {:?}",
                mode
            );
        }
    }

    #[test]
    fn test_holographic_default() {
        let engine = HolographicPhysicsEngine::default();
        assert_eq!(engine.resonance_threshold, 0.8);
    }

    #[test]
    fn test_dual_physics_system_default() {
        let system = DualPhysicsSystem::default();
        assert_eq!(system.mode, PhysicsMode::Hardcoded);
        assert_eq!(system.comparison_tolerance, 0.1);
    }
}
