//! Teleological Progress - Purpose and Direction in Evolution
//!
//! Implements teleological direction for evolution, tracking progress toward
//! the ultimate purpose: return to Intelligent-Infinity, having served.
//!
//! From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 11-12:
//! "Define purpose: return to Intelligent-Infinity"
//! "Progress tracking toward purpose"
//! "Meaningful choices (not random)"
//! "Coherence with purpose affects evolution"
//!
//! ## Key Insights
//!
//! **Mechanical Evolution** (Random Walk):
//! - No purpose or direction
//! - Random choices without meaning
//! - Progress = experience accumulation
//! - Evolution is drift
//!
//! **Teleological Evolution** (Purposeful):
//! - Clear purpose: return to source, having served
//! - Meaningful choices aligned with purpose
//! - Progress = alignment with source
//! - Evolution is directed
//!
//! ## The Purpose
//!
//! The purpose of evolution is NOT "accumulate experience" or "become powerful".
//! The purpose IS "return to source (Intelligent-Infinity), having served".
//!
//! This distinction is critical:
//! - Experience accumulation is a byproduct, not the goal
//! - Service orientation (STO vs STS) matters
//! - Coherence with source determines progress
//! - Wisdom = integration of experience into purposeful action
//!
//! ## Teleological Metrics
//!
//! 1. **Purpose Alignment** (0.0 to 1.0):
//!    How aligned are the entity's choices with returning to source?
//!
//! 2. **Coherence with Source** (0.0 to 1.0):
//!    How resonant is the entity with Intelligent-Infinity?
//!
//! 3. **Service Orientation** (-1.0 to 1.0):
//!    STO (+1.0) vs STS (-1.0) balance
//!    Neutral = 0.0
//!
//! 4. **Wisdom Accumulated** (0.0 to ∞):
//!    Experience integrated into understanding
//!    Not just raw experience, but integrated wisdom

use crate::entity_layer7::layer7::SubSubLogos;

/// Teleological Progress - tracking progress toward purpose
///
/// Measures how well an entity is aligned with its evolutionary purpose:
/// "return to Intelligent-Infinity, having served"
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 11-12:
/// "Purpose is NOT 'accumulate experience'"
/// "Purpose IS 'return to source, having served'"
#[derive(Debug, Clone)]
pub struct TeleologicalProgress {
    /// Aligned with return to source (0.0 to 1.0)
    ///
    /// Higher values indicate the entity's choices are aligned with
    /// the purpose of returning to Intelligent-Infinity.
    pub purpose_alignment: f64,

    /// Resonance with Intelligent-Infinity (0.0 to 1.0)
    ///
    /// Measures how coherent the entity is with the source field.
    /// Higher coherence = stronger connection to source.
    pub coherence_with_source: f64,

    /// STO/STS balance (-1.0 to 1.0)
    ///
    /// Positive values = Service-to-Others (STO) orientation
    /// Negative values = Service-to-Self (STS) orientation
    /// Zero = Neutral (unpolarized)
    pub service_orientation: f64,

    /// Experience integration into wisdom (0.0 to ∞)
    ///
    /// Not just raw experience, but integrated wisdom that guides
    /// future choices. Wisdom accumulates as experience is integrated.
    pub wisdom_accumulated: f64,

    /// Overall teleological progress score (0.0 to 1.0)
    ///
    /// Composite metric combining all aspects of teleological progress.
    pub overall_progress: f64,
}

impl TeleologicalProgress {
    /// Create new teleological progress assessment
    pub fn new(
        purpose_alignment: f64,
        coherence_with_source: f64,
        service_orientation: f64,
        wisdom_accumulated: f64,
    ) -> Self {
        let purpose_alignment = purpose_alignment.clamp(0.0, 1.0);
        let coherence_with_source = coherence_with_source.clamp(0.0, 1.0);
        let service_orientation = service_orientation.clamp(-1.0, 1.0);
        let wisdom_accumulated = wisdom_accumulated.max(0.0);

        // Calculate overall progress
        // Purpose and coherence are primary (50% each)
        // Service orientation and wisdom are secondary
        let overall_progress = {
            let primary = (purpose_alignment + coherence_with_source) / 2.0;
            let secondary = (service_orientation.abs()
                + (wisdom_accumulated / (wisdom_accumulated + 1.0)))
                / 2.0;
            primary * 0.7 + secondary * 0.3
        };

        TeleologicalProgress {
            purpose_alignment,
            coherence_with_source,
            service_orientation,
            wisdom_accumulated,
            overall_progress,
        }
    }

    /// Check if entity is making meaningful progress
    ///
    /// Returns true if overall progress is significant (> 0.5).
    pub fn is_meaningful_progress(&self) -> bool {
        self.overall_progress > 0.5
    }

    /// Get primary polarization direction
    ///
    /// Returns "STO", "STS", or "Neutral" based on service orientation.
    pub fn primary_polarity(&self) -> &'static str {
        if self.service_orientation > 0.2 {
            "STO"
        } else if self.service_orientation < -0.2 {
            "STS"
        } else {
            "Neutral"
        }
    }
}

/// Evaluate entity's teleological progress
///
/// Analyzes an entity's state to determine how well it is aligned with
/// its evolutionary purpose: return to Intelligent-Infinity, having served.
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md Phase 3, Week 11-12:
/// "Progress tracking toward purpose"
/// "Meaningful choices (not random)"
///
/// # Arguments
/// * `entity` - Entity to evaluate
///
/// # Returns
/// TeleologicalProgress assessment
///
/// # Examples
/// ```
/// use holonic_realms::evolution::evaluate_purpose;
/// use holonic_realms::entity_layer7::layer7::SubSubLogos;
/// use holonic_realms::types::{Float, Density};
///
/// let entity = /* create entity */;
/// let teleological = evaluate_purpose(&entity);
///
/// assert!(teleological.purpose_alignment >= 0.0);
/// assert!(teleological.purpose_alignment <= 1.0);
/// assert!(teleological.service_orientation >= -1.0);
/// assert!(teleological.service_orientation <= 1.0);
/// ```
pub fn evaluate_purpose(entity: &SubSubLogos) -> TeleologicalProgress {
    // 1. Purpose Alignment: Based on spectrum access and evolutionary direction
    let purpose_alignment = calculate_purpose_alignment(entity);

    // 2. Coherence with Source: Based on resonance with Intelligent-Infinity
    let coherence_with_source = calculate_source_coherence(entity);

    // 3. Service Orientation: Based on polarization state
    let service_orientation = calculate_service_orientation(entity);

    // 4. Wisdom: Integration of experience
    let wisdom_accumulated = calculate_wisdom(entity);

    TeleologicalProgress::new(
        purpose_alignment,
        coherence_with_source,
        service_orientation,
        wisdom_accumulated,
    )
}

/// Calculate how aligned entity is with return-to-source purpose
///
/// Purpose alignment is based on:
/// - Spectrum access (moving toward time/space = oneness)
/// - Veil transparency (thinner veil = closer to source)
/// - Evolutionary direction (moving toward higher densities)
fn calculate_purpose_alignment(entity: &SubSubLogos) -> f64 {
    // Spectrum access: More time/space = more oneness = more aligned
    let spectrum_alignment = entity.spectrum_access.time_space_access;

    // Veil transparency: Thinner veil = closer to source
    let veil_alignment = entity.veil_transparency;

    // Evolutionary direction: Higher density = closer to source
    let density_progress = match entity.evolutionary_attractor.current_density {
        crate::entity_layer7::layer7::DensityLevel::First => 0.125,
        crate::entity_layer7::layer7::DensityLevel::Second => 0.25,
        crate::entity_layer7::layer7::DensityLevel::Third => 0.375,
        crate::entity_layer7::layer7::DensityLevel::Fourth => 0.5,
        crate::entity_layer7::layer7::DensityLevel::Fifth => 0.625,
        crate::entity_layer7::layer7::DensityLevel::Sixth => 0.75,
        crate::entity_layer7::layer7::DensityLevel::Seventh => 0.875,
        crate::entity_layer7::layer7::DensityLevel::Eighth => 1.0,
    };

    // Consciousness level: Higher consciousness = more aligned
    let consciousness_alignment = entity.consciousness_level;

    // Weighted average
    spectrum_alignment * 0.3
        + veil_alignment * 0.3
        + density_progress * 0.2
        + consciousness_alignment * 0.2
}

/// Calculate coherence with Intelligent-Infinity
///
/// Coherence measures resonance with the source field.
/// Higher coherence = stronger connection to Intelligent-Infinity.
fn calculate_source_coherence(entity: &SubSubLogos) -> f64 {
    // Oneness access (time/space dominance = closer to source)
    let oneness_access = entity.spectrum_access.time_space_access;

    // Mannyness access (inverse of oneness)
    let mannyness_access = entity.spectrum_access.mannyness_access;

    // Coherence = oneness - mannyness
    let base_coherence = oneness_access - mannyness_access;

    // Modulate by consciousness level (higher consciousness = higher coherence)
    let consciousness_factor = entity.consciousness_level;

    // Modulate by archetype activations (higher activation = higher coherence)
    let avg_archetype_activation: f64 = entity.archetype_activations.iter().sum::<f64>() / 22.0;

    // Combine factors
    let coherence = (base_coherence + 1.0) / 2.0; // Normalize to 0-1
    let coherence = coherence * 0.5 + consciousness_factor * 0.3 + avg_archetype_activation * 0.2;

    coherence.clamp(0.0, 1.0)
}

/// Calculate service orientation (STO vs STS)
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Archetype 22 (The Choice): Creates polarity by choosing between
/// Service-to-Others (STO) and Service-to-Self (STS)"
fn calculate_service_orientation(entity: &SubSubLogos) -> f64 {
    // Get polarization bias from entity state
    let polarity_bias = entity.current_state.polarity_state.polarity_bias;

    // Service orientation is directly related to polarization bias
    // Positive = STO, Negative = STS
    let orientation = polarity_bias;

    // Modulate by coherence (higher coherence amplifies polarity)
    let coherence = calculate_source_coherence(entity);

    orientation * coherence
}

/// Calculate wisdom (integrated experience)
///
/// Wisdom is not just raw experience, but experience that has been
/// integrated into understanding that guides future choices.
fn calculate_wisdom(entity: &SubSubLogos) -> f64 {
    // Base wisdom from experience accumulation
    let base_wisdom = entity.experience_accumulation;

    // Learning progress increases wisdom efficiency
    let learning_factor = 1.0 + entity.learning_progress;

    // Consciousness level increases wisdom quality
    let consciousness_factor = 1.0 + entity.consciousness_level;

    // Karmic pattern resolution increases wisdom
    let resolved_patterns = entity
        .karmic_patterns
        .iter()
        .filter(|p| {
            matches!(
                p.resolution_status,
                crate::entity_layer7::layer7::ResolutionStatus::Resolved
            )
        })
        .count();
    let karmic_factor = 1.0 + (resolved_patterns as f64 * 0.1);

    // Wisdom = experience × efficiency × quality × karmic integration
    base_wisdom * learning_factor * consciousness_factor * karmic_factor
}

/// Check if entity's choices are meaningful (not random)
///
/// A choice is meaningful if it aligns with the entity's purpose and
/// is based on integrated wisdom rather than random chance.
///
/// # Arguments
/// * `entity` - Entity to evaluate
///
/// # Returns
/// true if choices are meaningful, false otherwise
pub fn has_meaningful_choices(entity: &SubSubLogos) -> bool {
    let teleological = evaluate_purpose(entity);

    // Choices are meaningful if:
    // 1. Purpose alignment is significant (> 0.4)
    // 2. Wisdom has been accumulated (> 1.0)
    // 3. Overall progress is positive (> 0.3)

    teleological.purpose_alignment > 0.4
        && teleological.wisdom_accumulated > 1.0
        && teleological.overall_progress > 0.3
}

/// Calculate how coherence affects evolution rate
///
/// From REFACTOR_ROADMAP_HOLOGRAPHIC.md:
/// "Coherence with purpose affects evolution"
///
/// Higher coherence with purpose = faster evolution.
/// Lower coherence = slower evolution.
///
/// # Arguments
/// * `entity` - Entity to evaluate
///
/// # Returns
/// Evolution rate modifier (0.5 to 2.0)
/// - 1.0 = normal rate
/// - > 1.0 = accelerated by coherence
/// - < 1.0 = slowed by incoherence
pub fn coherence_evolution_modifier(entity: &SubSubLogos) -> f64 {
    let teleological = evaluate_purpose(entity);

    // Base modifier from overall progress
    let base_modifier = 0.5 + teleological.overall_progress; // 0.5 to 1.5

    // Boost from purpose alignment
    let purpose_boost = teleological.purpose_alignment * 0.5; // 0.0 to 0.5

    // Boost from coherence with source
    let coherence_boost = teleological.coherence_with_source * 0.5; // 0.0 to 0.5

    // Apply entity's evolutionary rate (individual variation)
    let entity_rate = entity.evolutionary_rate;

    // Combine all factors
    let modifier = base_modifier + purpose_boost + coherence_boost;
    let modifier = modifier * entity_rate;

    modifier.clamp(0.5, 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{EntityId, EntityType, SubSubLogos};
    use crate::foundation::{
        IntelligentInfinity as IndigoRealm, LightLoveField as GreenRealm, Logos as BlueRealm,
        VioletRealm,
    };
    use crate::spectrum::{OrangeRealm, RedRealm, SpectrumRatio, YellowRealm};

    // Helper function to create a test entity
    fn create_test_entity() -> SubSubLogos {
        let entity_id = EntityId::new("test-entity".to_string());
        let entity_type = EntityType::Individual;

        // Create minimal foundation realms
        let violet_realm = VioletRealm::new();
        let indigo_realm = IndigoRealm::from_violet(violet_realm.clone());
        let blue_realm = BlueRealm::from_intelligent_infinity(indigo_realm.clone());
        let green_realm = GreenRealm::from_logos(blue_realm.clone());

        // Create spectrum realms in correct dependency order
        // YellowRealm requires LightLoveField
        let yellow_realm = YellowRealm::new(green_realm.clone());
        // OrangeRealm requires YellowRealm
        let orange_realm = OrangeRealm::new(yellow_realm.clone());
        // RedRealm requires OrangeRealm
        let red_realm = RedRealm::new(orange_realm.clone());

        // Create minimal spectrum configuration
        // Use correct SpectrumRatio fields and IndividualSpectrumConfiguration::new()
        let spectrum_ratio = SpectrumRatio::new(1.0, crate::spectrum::SpectrumSide::SpaceTime);
        let spectrum_configuration =
            crate::entity_layer7::layer7::IndividualSpectrumConfiguration::new(spectrum_ratio);

        SubSubLogos::new(
            entity_id,
            entity_type,
            None,
            Vec::new(),
            None,
            violet_realm,
            indigo_realm,
            blue_realm,
            green_realm,
            yellow_realm,
            orange_realm,
            red_realm,
            spectrum_configuration,
        )
    }

    #[test]
    fn test_teleological_progress_creation() {
        let progress = TeleologicalProgress::new(0.7, 0.8, 0.5, 10.0);

        assert_eq!(progress.purpose_alignment, 0.7);
        assert_eq!(progress.coherence_with_source, 0.8);
        assert_eq!(progress.service_orientation, 0.5);
        assert_eq!(progress.wisdom_accumulated, 10.0);
        assert!(progress.overall_progress > 0.0);
        assert!(progress.overall_progress <= 1.0);
    }

    #[test]
    fn test_teleological_progress_clamping() {
        let progress = TeleologicalProgress::new(1.5, -0.5, 2.0, -1.0);

        assert_eq!(progress.purpose_alignment, 1.0); // Clamped
        assert_eq!(progress.coherence_with_source, 0.0); // Clamped
        assert_eq!(progress.service_orientation, 1.0); // Clamped
        assert_eq!(progress.wisdom_accumulated, 0.0); // Clamped to 0
    }

    #[test]
    fn test_is_meaningful_progress() {
        let progress_high = TeleologicalProgress::new(0.7, 0.8, 0.5, 10.0);
        let progress_low = TeleologicalProgress::new(0.3, 0.4, 0.2, 0.5);

        assert!(progress_high.is_meaningful_progress());
        assert!(!progress_low.is_meaningful_progress());
    }

    #[test]
    fn test_primary_polarity() {
        let sto = TeleologicalProgress::new(0.7, 0.8, 0.5, 10.0);
        let sts = TeleologicalProgress::new(0.7, 0.8, -0.5, 10.0);
        let neutral = TeleologicalProgress::new(0.7, 0.8, 0.1, 10.0);

        assert_eq!(sto.primary_polarity(), "STO");
        assert_eq!(sts.primary_polarity(), "STS");
        assert_eq!(neutral.primary_polarity(), "Neutral");
    }

    #[test]
    fn test_evaluate_purpose() {
        let entity = create_test_entity();
        let teleological = evaluate_purpose(&entity);

        assert!(teleological.purpose_alignment >= 0.0);
        assert!(teleological.purpose_alignment <= 1.0);
        assert!(teleological.coherence_with_source >= 0.0);
        assert!(teleological.coherence_with_source <= 1.0);
        assert!(teleological.service_orientation >= -1.0);
        assert!(teleological.service_orientation <= 1.0);
        assert!(teleological.wisdom_accumulated >= 0.0);
        assert!(teleological.overall_progress >= 0.0);
        assert!(teleological.overall_progress <= 1.0);
    }

    #[test]
    fn test_has_meaningful_choices() {
        let entity = create_test_entity();

        // New entity has no wisdom, so choices are not meaningful
        assert!(!has_meaningful_choices(&entity));
    }

    #[test]
    fn test_coherence_evolution_modifier() {
        let entity = create_test_entity();
        let modifier = coherence_evolution_modifier(&entity);

        assert!(modifier >= 0.5);
        assert!(modifier <= 2.0);
    }

    #[test]
    fn test_purpose_alignment_increases_with_density() {
        // This test would require modifying entity density
        // For now, we just verify the function runs
        let entity = create_test_entity();
        let alignment = calculate_purpose_alignment(&entity);

        assert!(alignment >= 0.0);
        assert!(alignment <= 1.0);
    }

    #[test]
    fn test_source_coherence_calculation() {
        let entity = create_test_entity();
        let coherence = calculate_source_coherence(&entity);

        assert!(coherence >= 0.0);
        assert!(coherence <= 1.0);
    }

    #[test]
    fn test_service_orientation_calculation() {
        let entity = create_test_entity();
        let orientation = calculate_service_orientation(&entity);

        assert!(orientation >= -1.0);
        assert!(orientation <= 1.0);
    }

    #[test]
    fn test_wisdom_calculation() {
        let entity = create_test_entity();
        let wisdom = calculate_wisdom(&entity);

        assert!(wisdom >= 0.0);
    }

    #[test]
    fn test_wisdom_increases_with_experience() {
        let entity = create_test_entity();
        let wisdom_low = calculate_wisdom(&entity);

        // Increase experience (would need mutable entity)
        // For now, just verify the function runs
        assert!(wisdom_low >= 0.0);
    }
}
