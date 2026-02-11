//! Holographic Principle Validation
//!
//! This module provides validation mechanisms for the holographic principle,
//! which states that "any portion contains the whole". In the context of the
//! Law of One cosmological architecture, this means that each entity contains
//! all densities and sub-densities in its holographic blueprint.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The holographic principle states that
//! each entity contains the whole cosmological architecture - all densities,
//! sub-densities, and spectrum configurations are encoded in the holographic
//! blueprint of each individual entity."

use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::entity_layer7::SubSubLogos;
use crate::evolution_density_octave::density_octave::{Density, SubDensity};

/// Holographic principle validator
///
/// Validates that entities demonstrate the holographic principle:
/// - Each entity contains all densities (octave containment)
/// - Each entity contains all sub-densities for each density
/// - Each entity reflects the macrocosm (microcosm-macrocosm reflection)
pub struct HolographicPrincipleValidator;

impl HolographicPrincipleValidator {
    /// Validate that entity contains all densities
    ///
    /// This validates the "octave containment" aspect of the holographic principle:
    /// each entity's holographic blueprint should contain encodings for all
    /// 8 densities of the density octave.
    ///
    /// # Arguments
    /// * `entity` - The entity to validate
    ///
    /// # Returns
    /// * `OctaveContainmentResult` - Result showing octave containment percentage
    pub fn validate_octave_containment(entity: &SubSubLogos) -> OctaveContainmentResult {
        let mut contained_densities = Vec::new();

        // Check if entity holographic blueprint contains all densities
        for density in Density::all() {
            if entity.holographic_blueprint.contains_density(&density) {
                contained_densities.push(density);
            }
        }

        let total_densities = Density::all().len();
        let contained_count = contained_densities.len();
        let percentage = (contained_count as f64 / total_densities as f64) * 100.0;

        OctaveContainmentResult {
            entity_id: entity.entity_id.as_u64(),
            total_densities,
            contained_densities: contained_count,
            contained_densities_list: contained_densities,
            percentage,
            is_complete: percentage >= 100.0,
        }
    }

    /// Validate that entity contains all sub-densities for a specific density
    ///
    /// Each density has multiple sub-densities (e.g., 1st density has:
    /// Quantum Realm, Atomic Realm, Molecular Realm, Planetary Realm).
    /// This validates that the entity contains all sub-densities for a given density.
    ///
    /// # Arguments
    /// * `entity` - The entity to validate
    /// * `density` - The density to check sub-densities for
    ///
    /// # Returns
    /// * `SubDensityContainmentResult` - Result showing sub-density containment percentage
    pub fn validate_sub_density_containment(
        entity: &SubSubLogos,
        density: Density,
    ) -> SubDensityContainmentResult {
        let mut contained_sub_densities = Vec::new();
        let all_sub_densities = density.sub_densities();

        for sub_density in &all_sub_densities {
            if entity
                .holographic_blueprint
                .contains_sub_density(&density, sub_density.clone())
            {
                contained_sub_densities.push(sub_density.clone());
            }
        }

        let total_sub_densities = all_sub_densities.len();
        let contained_count = contained_sub_densities.len();
        let percentage = (contained_count as f64 / total_sub_densities as f64) * 100.0;

        SubDensityContainmentResult {
            entity_id: entity.entity_id.as_u64(),
            density: density.clone(),
            total_sub_densities,
            contained_sub_densities: contained_count,
            contained_sub_densities_list: contained_sub_densities,
            percentage,
            is_complete: percentage >= 100.0,
        }
    }

    /// Validate microcosm-macrocosm reflection
    ///
    /// This validates that the entity's holographic blueprint reflects the
    /// macrocosm (the overall cosmological architecture). The similarity is
    /// calculated by comparing the entity's blueprint with a reference macrocosm blueprint.
    ///
    /// # Arguments
    /// * `entity` - The entity to validate
    /// * `macrocosm_blueprint` - Reference macrocosm blueprint
    ///
    /// # Returns
    /// * `MicrocosmMacrocosmResult` - Result showing similarity percentage
    pub fn validate_microcosm_macrocosm(
        entity: &SubSubLogos,
        macrocosm_blueprint: &HolographicBlueprint,
    ) -> MicrocosmMacrocosmResult {
        let entity_blueprint = &entity.holographic_blueprint;

        // Calculate blueprint similarity
        let similarity =
            Self::calculate_blueprint_similarity(entity_blueprint, macrocosm_blueprint);

        // Threshold for reflection: 70% similarity indicates strong macrocosm reflection
        let reflects_macrocosm = similarity >= 70.0;

        MicrocosmMacrocosmResult {
            entity_id: entity.entity_id.as_u64(),
            similarity_percentage: similarity,
            reflects_macrocosm,
        }
    }

    /// Calculate similarity between two holographic blueprints
    ///
    /// This compares two blueprints by checking:
    /// 1. Density encodings overlap
    /// 2. Sub-density encodings overlap
    /// 3. Spectrum pattern similarity
    ///
    /// # Arguments
    /// * `blueprint1` - First blueprint
    /// * `blueprint2` - Second blueprint
    ///
    /// # Returns
    /// * `f64` - Similarity percentage (0.0 to 100.0)
    fn calculate_blueprint_similarity(
        blueprint1: &HolographicBlueprint,
        blueprint2: &HolographicBlueprint,
    ) -> f64 {
        let mut total_checks = 0;
        let mut matching_checks = 0;

        // Check density containment
        for density in Density::all() {
            total_checks += 1;
            if blueprint1.contains_density(&density) && blueprint2.contains_density(&density) {
                matching_checks += 1;
            }
        }

        // Check sub-density containment
        for density in Density::all() {
            for sub_density in density.sub_densities() {
                total_checks += 1;
                let contains1 = blueprint1.contains_sub_density(&density, sub_density.clone());
                let contains2 = blueprint2.contains_sub_density(&density, sub_density.clone());
                if contains1 && contains2 {
                    matching_checks += 1;
                }
            }
        }

        // Calculate similarity percentage
        if total_checks == 0 {
            0.0
        } else {
            (matching_checks as f64 / total_checks as f64) * 100.0
        }
    }

    /// Validate complete holographic principle for an entity
    ///
    /// This performs all holographic principle validations:
    /// - Octave containment
    /// - Sub-density containment (for all densities)
    /// - Microcosm-macrocosm reflection
    ///
    /// # Arguments
    /// * `entity` - The entity to validate
    /// * `macrocosm_blueprint` - Reference macrocosm blueprint
    ///
    /// # Returns
    /// * `HolographicPrincipleValidationResult` - Complete validation result
    pub fn validate_complete(
        entity: &SubSubLogos,
        macrocosm_blueprint: &HolographicBlueprint,
    ) -> HolographicPrincipleValidationResult {
        // Validate octave containment
        let octave_result = Self::validate_octave_containment(entity);

        // Validate sub-density containment for each density
        let mut sub_density_results = Vec::new();
        for density in Density::all() {
            let result = Self::validate_sub_density_containment(entity, density.clone());
            sub_density_results.push(result);
        }

        // Validate microcosm-macrocosm reflection
        let microcosm_result = Self::validate_microcosm_macrocosm(entity, macrocosm_blueprint);

        // Calculate overall validation score
        let octave_score = octave_result.percentage;
        let sub_density_score: f64 = sub_density_results
            .iter()
            .map(|r| r.percentage)
            .sum::<f64>()
            / sub_density_results.len() as f64;
        let microcosm_score = microcosm_result.similarity_percentage;

        let overall_score = (octave_score + sub_density_score + microcosm_score) / 3.0;

        // Determine if holographic principle is validated
        let is_validated = overall_score >= 70.0; // 70% threshold for validation

        HolographicPrincipleValidationResult {
            entity_id: entity.entity_id.as_u64(),
            octave_result,
            sub_density_results,
            microcosm_result,
            overall_score,
            is_validated,
        }
    }
}

/// Result of octave containment validation
#[derive(Debug, Clone)]
pub struct OctaveContainmentResult {
    /// Entity ID
    pub entity_id: u64,
    /// Total number of densities in the octave
    pub total_densities: usize,
    /// Number of densities contained in the entity's blueprint
    pub contained_densities: usize,
    /// List of contained densities
    pub contained_densities_list: Vec<Density>,
    /// Percentage of densities contained (0.0 to 100.0)
    pub percentage: f64,
    /// Whether octave containment is complete (100%)
    pub is_complete: bool,
}

/// Result of sub-density containment validation
#[derive(Debug, Clone)]
pub struct SubDensityContainmentResult {
    /// Entity ID
    pub entity_id: u64,
    /// Density being validated
    pub density: Density,
    /// Total number of sub-densities for this density
    pub total_sub_densities: usize,
    /// Number of sub-densities contained in the entity's blueprint
    pub contained_sub_densities: usize,
    /// List of contained sub-densities
    pub contained_sub_densities_list: Vec<SubDensity>,
    /// Percentage of sub-densities contained (0.0 to 100.0)
    pub percentage: f64,
    /// Whether sub-density containment is complete (100%)
    pub is_complete: bool,
}

/// Result of microcosm-macrocosm reflection validation
#[derive(Debug, Clone)]
pub struct MicrocosmMacrocosmResult {
    /// Entity ID
    pub entity_id: u64,
    /// Similarity percentage between entity and macrocosm (0.0 to 100.0)
    pub similarity_percentage: f64,
    /// Whether entity reflects macrocosm (>= 70% similarity)
    pub reflects_macrocosm: bool,
}

/// Result of complete holographic principle validation
#[derive(Debug, Clone)]
pub struct HolographicPrincipleValidationResult {
    /// Entity ID
    pub entity_id: u64,
    /// Octave containment result
    pub octave_result: OctaveContainmentResult,
    /// Sub-density containment results for all densities
    pub sub_density_results: Vec<SubDensityContainmentResult>,
    /// Microcosm-macrocosm reflection result
    pub microcosm_result: MicrocosmMacrocosmResult,
    /// Overall validation score (average of all scores)
    pub overall_score: f64,
    /// Whether holographic principle is validated (>= 70% overall score)
    pub is_validated: bool,
}

impl HolographicPrincipleValidationResult {
    /// Get a human-readable summary of the validation result
    pub fn get_summary(&self) -> String {
        let mut summary = format!(
            "Entity {} Holographic Principle Validation:\n",
            self.entity_id
        );
        summary.push_str(&format!(
            "  Overall Score: {:.1}% ({})\n",
            self.overall_score,
            if self.is_validated {
                "VALID ✓"
            } else {
                "INVALID ✗"
            }
        ));
        summary.push_str(&format!(
            "  Octave Containment: {:.1}% ({}/{})\n",
            self.octave_result.percentage,
            self.octave_result.contained_densities,
            self.octave_result.total_densities
        ));
        summary.push_str("  Sub-Density Containment:\n");
        for result in &self.sub_density_results {
            summary.push_str(&format!(
                "    {:?}: {:.1}% ({}/{})\n",
                result.density,
                result.percentage,
                result.contained_sub_densities,
                result.total_sub_densities
            ));
        }
        summary.push_str(&format!(
            "  Microcosm-Macrocosm: {:.1}% similarity ({})\n",
            self.microcosm_result.similarity_percentage,
            if self.microcosm_result.reflects_macrocosm {
                "Reflects ✓"
            } else {
                "Does not reflect ✗"
            }
        ));
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
    use crate::entity_layer7::layer7::SubSubLogos;

    #[test]
    fn test_validate_octave_containment() {
        // This test requires a fully implemented entity with holographic blueprint
        // For now, we'll skip this test as it requires the complete density octave implementation
        // TODO: Implement this test once density octave is fully integrated
    }

    #[test]
    fn test_validate_sub_density_containment() {
        // This test requires a fully implemented entity with holographic blueprint
        // TODO: Implement this test once density octave is fully integrated
    }

    #[test]
    fn test_validate_microcosm_macrocosm() {
        // This test requires a fully implemented entity with holographic blueprint
        // TODO: Implement this test once density octave is fully integrated
    }

    #[test]
    fn test_validate_complete() {
        // This test requires a fully implemented entity with holographic blueprint
        // TODO: Implement this test once density octave is fully integrated
    }
}
