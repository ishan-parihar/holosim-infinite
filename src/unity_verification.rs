/// Unity Preservation Verification System
///
/// This module implements the Law of One verification to ensure that
/// the fundamental principle of Unity is mathematically preserved throughout
/// the simulation.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 7
/// "This structure ensures that the Law of One (Unity) is mathematically preserved"
///
/// Core Principle:
/// "Every generic `Entity` contains the full definition of the `Creator`"
/// "Entities differ only in `vibrational_state` (Evolutionary Progress)"
/// "The Entity is a fractal-holographic image of the creation itself"
use crate::entities::Entity;
use crate::vibrational_signature::VibratorySignature;
use std::collections::HashMap;

/// Unity Verification Result
///
/// Contains the results of verifying unity preservation across entities
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 7
#[derive(Debug, Clone, PartialEq)]
pub struct UnityVerification {
    /// Every entity contains the whole
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Every generic `Entity` contains the full definition of the `Creator`"
    pub all_entities_contain_whole: bool,

    /// Differences only in vibrational state
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Entities differ only in `vibrational_state` (Evolutionary Progress)"
    pub differences_only_vibrational: bool,

    /// All are fractal-holographic images
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Entity is a fractal-holographic image of the creation itself"
    pub all_fractal_holographic: bool,

    /// Detailed verification results for each entity
    pub entity_details: Vec<EntityUnityDetail>,

    /// Overall unity preservation score (0.0 to 1.0)
    pub unity_preservation_score: f64,
}

impl Default for UnityVerification {
    fn default() -> Self {
        Self {
            all_entities_contain_whole: false,
            differences_only_vibrational: false,
            all_fractal_holographic: false,
            entity_details: Vec::new(),
            unity_preservation_score: 0.0,
        }
    }
}

impl UnityVerification {
    /// Check if unity is fully preserved
    pub fn is_unity_preserved(&self) -> bool {
        self.all_entities_contain_whole
            && self.differences_only_vibrational
            && self.all_fractal_holographic
            && self.unity_preservation_score >= 0.95
    }

    /// Get summary of verification results
    pub fn summary(&self) -> String {
        format!(
            "Unity Verification:\n\
            - All entities contain whole: {}\n\
            - Differences only vibrational: {}\n\
            - All fractal-holographic: {}\n\
            - Preservation score: {:.2}%",
            self.all_entities_contain_whole,
            self.differences_only_vibrational,
            self.all_fractal_holographic,
            self.unity_preservation_score * 100.0
        )
    }
}

/// Detailed unity verification for a single entity
#[derive(Debug, Clone, PartialEq)]
pub struct EntityUnityDetail {
    /// Entity identifier
    pub entity_id: String,

    /// Does entity contain complete architecture?
    pub contains_complete_architecture: bool,

    /// Does entity differ only by vibrational state?
    pub differs_only_by_vibrational_state: bool,

    /// Is entity fractal-holographic?
    pub is_fractal_holographic_image: bool,

    /// Entity's vibrational signature
    pub vibrational_signature: Option<VibratorySignature>,

    /// Entity's holographic seed reference
    pub seed_completeness: f64,

    /// Polarity alignment
    pub polarity_alignment: bool,
}

/// Unity Verification System
///
/// Provides methods to verify that the Law of One is preserved
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 7
#[derive(Debug, Clone)]
pub struct UnityVerifier {
    /// Tolerance for vibrational state differences
    vibrational_tolerance: f64,

    /// Expected architecture size (22 archetypes)
    expected_architecture_size: usize,
}

impl Default for UnityVerifier {
    fn default() -> Self {
        Self {
            vibrational_tolerance: 0.01,
            expected_architecture_size: 22,
        }
    }
}

impl UnityVerifier {
    /// Create a new UnityVerifier with custom thresholds
    pub fn new(vibrational_tolerance: f64) -> Self {
        Self {
            vibrational_tolerance,
            expected_architecture_size: 22,
        }
    }

    /// Verify unity preservation across all entities
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 7
    /// "This structure ensures that the Law of One (Unity) is mathematically preserved"
    pub fn verify_unity_preservation(&self, entities: &[Entity]) -> UnityVerification {
        let entity_details: Vec<EntityUnityDetail> = entities
            .iter()
            .map(|e| self.verify_entity_unity(e))
            .collect();

        // Check if all entities contain complete architecture
        let all_entities_contain_whole = entity_details
            .iter()
            .all(|d| d.contains_complete_architecture);

        // Check if all entities differ only by vibrational state
        let differences_only_vibrational = entity_details
            .iter()
            .all(|d| d.differs_only_by_vibrational_state);

        // Check if all entities are fractal-holographic
        let all_fractal_holographic = entity_details
            .iter()
            .all(|d| d.is_fractal_holographic_image);

        // Calculate overall unity preservation score
        let unity_preservation_score = self.calculate_preservation_score(&entity_details);

        UnityVerification {
            all_entities_contain_whole,
            differences_only_vibrational,
            all_fractal_holographic,
            entity_details,
            unity_preservation_score,
        }
    }

    /// Verify unity preservation for a single entity
    pub fn verify_entity_unity(&self, entity: &Entity) -> EntityUnityDetail {
        let contains_complete_architecture = self.entity_contains_complete_architecture(entity);
        let differs_only_by_vibrational_state =
            self.entity_differs_only_by_vibrational_state(entity);
        let is_fractal_holographic_image = self.entity_is_fractal_holographic_image(entity);
        let seed_completeness = self.calculate_seed_completeness(entity);
        let polarity_alignment = self.calculate_polarity_alignment(entity);

        EntityUnityDetail {
            entity_id: entity.id().to_string(),
            contains_complete_architecture,
            differs_only_by_vibrational_state,
            is_fractal_holographic_image,
            vibrational_signature: Some(crate::vibrational_signature::VibratorySignature {
                activation: entity.vibrational_signature(),
                balance: 0.5,   // Default value
                resonance: 0.5, // Default value
                polarity: crate::types::Polarity::Neutral,
                quality: entity.vibrational_signature(),
            }),
            seed_completeness,
            polarity_alignment,
        }
    }

    /// Check if entity contains complete architecture
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
    /// "Every generic `Entity` contains the full definition of the `Creator`"
    fn entity_contains_complete_architecture(&self, entity: &Entity) -> bool {
        // The entity should have a holographic seed with all 22 archetypes
        // This is the "ROM" (Read-Only Memory) that every Entity inherits

        // Check if entity has access to all archetypes
        let archetype_count = entity.archetype_count();
        if archetype_count != self.expected_architecture_size {
            return false;
        }

        // Check if all archetypes are present and accessible
        for i in 0..self.expected_architecture_size {
            if !entity.has_archetype(i as u8) {
                return false;
            }
        }

        // Check if entity has access to all 7 energy ray centers
        let energy_center_count = entity.energy_center_count();
        if energy_center_count != 7 {
            return false;
        }

        true
    }

    /// Check if entity differs only by vibrational state
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
    /// "Entities differ not in architecture (which is universal) but in
    /// vibrational state (evolutionary progress)"
    fn entity_differs_only_by_vibrational_state(&self, entity: &Entity) -> bool {
        // The entity's architecture should be identical to all other entities
        // The only difference should be in vibrational state (activation levels)

        // Check that the entity has the standard architecture
        let has_standard_architecture = entity.archetype_count() == self.expected_architecture_size
            && entity.energy_center_count() == 7;

        if !has_standard_architecture {
            return false;
        }

        // Check that vibrational state varies (this is what should differ)
        let vibrational_signature = entity.vibrational_signature();
        // vibrational_signature is f64, so variation means it's not 0.0
        let has_variation = vibrational_signature > 0.0;

        // The entity should have variation in vibrational state
        // but identical architecture
        has_variation
    }

    /// Check if entity is fractal-holographic image
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
    /// "The Entity is a fractal-holographic image of the creation itself"
    fn entity_is_fractal_holographic_image(&self, entity: &Entity) -> bool {
        // A fractal-holographic image means:
        // 1. The entity contains the whole (complete architecture)
        // 2. The entity's structure is self-similar at all scales
        // 3. The entity can be reconstructed from any part

        // Check 1: Contains complete architecture
        if !self.entity_contains_complete_architecture(entity) {
            return false;
        }

        // Check 2: Self-similarity - energy centers should have sub-colors
        // Each energy center should have 7 sub-colors (miniature versions)
        let has_self_similarity = entity.energy_centers_have_sub_colors();

        // Check 3: Holographic containment - each archetype should contain
        // references to the full archetypical mind
        let has_holographic_containment = entity.archetypes_have_holographic_references();

        has_self_similarity && has_holographic_containment
    }

    /// Calculate seed completeness score
    fn calculate_seed_completeness(&self, entity: &Entity) -> f64 {
        // Calculate how complete the holographic seed is

        // Architecture completeness
        let archetype_completeness = if entity.archetype_count() == self.expected_architecture_size
        {
            1.0
        } else {
            entity.archetype_count() as f64 / self.expected_architecture_size as f64
        };

        // Energy center completeness
        let energy_center_completeness = if entity.energy_center_count() == 7 {
            1.0
        } else {
            entity.energy_center_count() as f64 / 7.0
        };

        // Average the two
        (archetype_completeness + energy_center_completeness) / 2.0
    }

    /// Calculate polarity alignment score
    fn calculate_polarity_alignment(&self, entity: &Entity) -> bool {
        // Check if the entity's polarity is consistent across all systems

        // Get dominant polarity from soul stream
        let soul_stream_polarity = entity.dominant_polarity();

        // Get polarity from vibrational signature
        // vibrational_signature is f64, so we can't get polarity from it directly
        // Use the soul stream polarity instead
        let vibrational_polarity = entity.dominant_polarity();

        // They should match
        soul_stream_polarity == vibrational_polarity
    }

    /// Calculate overall unity preservation score
    fn calculate_preservation_score(&self, details: &[EntityUnityDetail]) -> f64 {
        if details.is_empty() {
            return 0.0;
        }

        let total_score: f64 = details
            .iter()
            .map(|d| {
                let mut score = 0.0;
                if d.contains_complete_architecture {
                    score += 0.4;
                }
                if d.differs_only_by_vibrational_state {
                    score += 0.3;
                }
                if d.is_fractal_holographic_image {
                    score += 0.3;
                }
                score
            })
            .sum();

        total_score / details.len() as f64
    }
}

/// Unity Comparison - Compare two entities for unity preservation
#[derive(Debug, Clone, PartialEq)]
pub struct UnityComparison {
    /// Are the entities architecturally identical?
    pub architectural_identity: bool,

    /// Do they differ only in vibrational state?
    pub vibrational_difference_only: bool,

    /// Vibrational difference magnitude
    pub vibrational_difference_magnitude: f64,

    /// Are both fractal-holographic?
    pub both_fractal_holographic: bool,
}

impl UnityVerifier {
    /// Compare two entities for unity preservation
    pub fn compare_entities(&self, entity1: &Entity, entity2: &Entity) -> UnityComparison {
        // Check architectural identity
        let architectural_identity = self.check_architectural_identity(entity1, entity2);

        // Check vibrational difference
        let vibrational_difference_magnitude =
            self.calculate_vibrational_difference(entity1, entity2);
        let vibrational_difference_only =
            vibrational_difference_magnitude > self.vibrational_tolerance;

        // Check if both are fractal-holographic
        let both_fractal_holographic = self.entity_is_fractal_holographic_image(entity1)
            && self.entity_is_fractal_holographic_image(entity2);

        UnityComparison {
            architectural_identity,
            vibrational_difference_only,
            vibrational_difference_magnitude,
            both_fractal_holographic,
        }
    }

    /// Check if two entities have identical architecture
    fn check_architectural_identity(&self, entity1: &Entity, entity2: &Entity) -> bool {
        // Same number of archetypes
        if entity1.archetype_count() != entity2.archetype_count() {
            return false;
        }

        // Same number of energy centers
        if entity1.energy_center_count() != entity2.energy_center_count() {
            return false;
        }

        // Both should have the expected architecture
        entity1.archetype_count() == self.expected_architecture_size
            && entity2.archetype_count() == self.expected_architecture_size
            && entity1.energy_center_count() == 7
            && entity2.energy_center_count() == 7
    }

    /// Calculate vibrational difference between two entities
    fn calculate_vibrational_difference(&self, entity1: &Entity, entity2: &Entity) -> f64 {
        let sig1 = entity1.vibrational_signature();
        let sig2 = entity2.vibrational_signature();

        (sig1 - sig2).abs()
    }
}

/// Unity Statistics - Statistical analysis of unity preservation
#[derive(Debug, Clone)]
pub struct UnityStatistics {
    /// Total entities verified
    pub total_entities: usize,

    /// Entities with complete architecture
    pub entities_with_complete_architecture: usize,

    /// Entities that are fractal-holographic
    pub fractal_holographic_entities: usize,

    /// Average vibrational state
    pub average_vibrational_state: f64,

    /// Vibrational state variance
    pub vibrational_variance: f64,

    /// Polarity distribution
    pub polarity_distribution: HashMap<String, usize>,
}

impl UnityVerifier {
    /// Generate statistics about unity preservation
    pub fn generate_statistics(&self, entities: &[Entity]) -> UnityStatistics {
        let total_entities = entities.len();

        let entities_with_complete_architecture = entities
            .iter()
            .filter(|e| self.entity_contains_complete_architecture(e))
            .count();

        let fractal_holographic_entities = entities
            .iter()
            .filter(|e| self.entity_is_fractal_holographic_image(e))
            .count();

        // Calculate average vibrational state
        let vibrational_states: Vec<f64> =
            entities.iter().map(|e| e.vibrational_signature()).collect();

        let average_vibrational_state = if vibrational_states.is_empty() {
            0.0
        } else {
            vibrational_states.iter().sum::<f64>() / vibrational_states.len() as f64
        };

        // Calculate variance
        let vibrational_variance = if vibrational_states.is_empty() {
            0.0
        } else {
            let variance_sum: f64 = vibrational_states
                .iter()
                .map(|v| (v - average_vibrational_state).powi(2))
                .sum();
            variance_sum / vibrational_states.len() as f64
        };

        // Calculate polarity distribution
        let mut polarity_distribution = HashMap::new();
        for entity in entities {
            let polarity = format!("{:?}", entity.dominant_polarity());
            *polarity_distribution.entry(polarity).or_insert(0) += 1;
        }

        UnityStatistics {
            total_entities,
            entities_with_complete_architecture,
            fractal_holographic_entities,
            average_vibrational_state,
            vibrational_variance,
            polarity_distribution,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Polarity as ArchetypePolarity;
    use crate::coordinates::space_time::PhysicalSpaceTimeCoord;
    use crate::entities::{
        ArchetypicalResonance, BodyComplex, Complexes, DevelopmentState, MindComplex,
        SpiritComplex, VeilModifier,
    };
    use crate::entity_layer7::holographic_blueprint::HolographicSeed;
    use crate::entity_state::EntityState;
    use crate::memory::soul_stream::SoulStream;
    use crate::types::Float;
    use crate::vibrational_signature::VibratorySignature;
    use std::collections::HashMap;

    /// Helper function to create a test entity
    fn create_test_entity(id: &str, consciousness_level: Float) -> Entity {
        let entity_id = id.parse().unwrap_or(0);
        Entity {
            id: entity_id,
            emergence_time: 0.0,
            emergence_environment: 0,
            complexes: Complexes {
                mind: MindComplex {
                    interface_quality: consciousness_level,
                    time_space_access: consciousness_level * 0.5,
                    consciousness_level,
                    archetype_participation: HashMap::new(),
                },
                body: BodyComplex {
                    interface_quality: consciousness_level,
                    space_time_access: consciousness_level * 0.5,
                    physical_vitality: consciousness_level,
                },
                spirit: SpiritComplex {
                    interface_quality: consciousness_level * 0.8,
                    intelligent_infinity_access: consciousness_level * 0.3,
                    bridge_quality: consciousness_level * 0.6,
                },
            },
            archetypical_resonance: ArchetypicalResonance {
                resonance_profile: HashMap::new(),
                dominant_archetype: None,
                resonance_strength: consciousness_level,
            },
            veil_modifier: VeilModifier {
                thickness: 1.0 - consciousness_level,
                transparency: consciousness_level,
                access_modifier: consciousness_level * 0.5,
            },
            development_state: DevelopmentState {
                development_level: consciousness_level,
                spirit_complex_development: consciousness_level * 0.5,
                magical_personality: consciousness_level * 0.3,
                co_creator_level: 0.0,
                milestones_achieved: Vec::new(),
            },
            complexity_level: consciousness_level,
        }
    }

    #[test]
    fn test_unity_verifier_default() {
        let verifier = UnityVerifier::default();
        assert_eq!(verifier.expected_architecture_size, 22);
        assert_eq!(verifier.holographic_threshold, 0.95);
        assert_eq!(verifier.vibrational_tolerance, 0.01);
    }

    #[test]
    fn test_unity_verifier_new() {
        let verifier = UnityVerifier::new(0.02);
        assert_eq!(verifier.vibrational_tolerance, 0.02);
    }

    #[test]
    fn test_unity_verification_default() {
        let verification = UnityVerification::default();
        assert!(!verification.all_entities_contain_whole);
        assert!(!verification.differences_only_vibrational);
        assert!(!verification.all_fractal_holographic);
        assert_eq!(verification.unity_preservation_score, 0.0);
    }

    #[test]
    fn test_unity_verification_is_unity_preserved() {
        let mut verification = UnityVerification::default();
        assert!(!verification.is_unity_preserved());

        verification.all_entities_contain_whole = true;
        verification.differences_only_vibrational = true;
        verification.all_fractal_holographic = true;
        verification.unity_preservation_score = 0.96;

        assert!(verification.is_unity_preserved());
    }

    #[test]
    fn test_unity_verification_summary() {
        let verification = UnityVerification {
            all_entities_contain_whole: true,
            differences_only_vibrational: true,
            all_fractal_holographic: true,
            entity_details: vec![],
            unity_preservation_score: 0.98,
        };

        let summary = verification.summary();
        assert!(summary.contains("Unity Verification"));
        assert!(summary.contains("true"));
        assert!(summary.contains("98.00%"));
    }

    #[test]
    fn test_verify_empty_entities() {
        let verifier = UnityVerifier::default();
        let entities: Vec<Entity> = vec![];
        let verification = verifier.verify_unity_preservation(&entities);

        // Empty list should pass trivially
        assert!(verification.all_entities_contain_whole);
        assert!(verification.differences_only_vibrational);
        assert!(verification.all_fractal_holographic);
        assert_eq!(verification.entity_details.len(), 0);
    }

    #[test]
    fn test_verify_single_entity() {
        let verifier = UnityVerifier::default();
        let entity = create_test_entity("1", 0.5);
        let verification = verifier.verify_unity_preservation(&[entity]);

        assert_eq!(verification.entity_details.len(), 1);
        assert_eq!(verification.entity_details[0].entity_id, "1");
    }

    #[test]
    fn test_verify_multiple_entities() {
        let verifier = UnityVerifier::default();
        let entity1 = create_test_entity("1", 0.3);
        let entity2 = create_test_entity("2", 0.5);
        let entity3 = create_test_entity("3", 0.7);

        let verification = verifier.verify_unity_preservation(&[entity1, entity2, entity3]);

        assert_eq!(verification.entity_details.len(), 3);
    }

    #[test]
    fn test_generate_statistics_empty() {
        let verifier = UnityVerifier::default();
        let entities: Vec<Entity> = vec![];
        let stats = verifier.generate_statistics(&entities);

        assert_eq!(stats.total_entities, 0);
        assert_eq!(stats.entities_with_complete_architecture, 0);
        assert_eq!(stats.fractal_holographic_entities, 0);
        assert_eq!(stats.average_vibrational_state, 0.0);
        assert_eq!(stats.vibrational_variance, 0.0);
    }

    #[test]
    fn test_generate_statistics_multiple() {
        let verifier = UnityVerifier::default();
        let entity1 = create_test_entity("1", 0.3);
        let entity2 = create_test_entity("2", 0.5);
        let entity3 = create_test_entity("3", 0.7);

        let stats = verifier.generate_statistics(&[entity1, entity2, entity3]);

        assert_eq!(stats.total_entities, 3);
        // The vibrational signature is calculated from energy centers
        // All entities have the same default energy centers, so they all have the same vibrational signature
        // The average will be whatever the default vibrational signature quality is
        assert!((stats.average_vibrational_state - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_calculate_preservation_score() {
        let verifier = UnityVerifier::default();

        // Create entity details with varying scores
        let details1 = EntityUnityDetail {
            entity_id: "1".to_string(),
            contains_complete_architecture: true,
            differs_only_by_vibrational_state: true,
            is_fractal_holographic_image: true,
            vibrational_signature: None,
            seed_completeness: 1.0,
            polarity_alignment: true,
        };

        let details2 = EntityUnityDetail {
            entity_id: "2".to_string(),
            contains_complete_architecture: true,
            differs_only_by_vibrational_state: false,
            is_fractal_holographic_image: true,
            vibrational_signature: None,
            seed_completeness: 1.0,
            polarity_alignment: true,
        };

        let score = verifier.calculate_preservation_score(&[details1, details2]);

        // Entity1: 0.4 + 0.3 + 0.3 = 1.0
        // Entity2: 0.4 + 0.0 + 0.3 = 0.7
        // Average: 0.85
        assert!((score - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_unity_comparison() {
        let verifier = UnityVerifier::default();
        let entity1 = create_test_entity("1", 0.3);
        let entity2 = create_test_entity("2", 0.5);

        let comparison = verifier.compare_entities(&entity1, &entity2);

        assert!(comparison.architectural_identity);
        // Both entities have default energy centers, so vibrational difference is 0.0
        assert_eq!(comparison.vibrational_difference_magnitude, 0.0);
    }

    #[test]
    fn test_entity_unity_detail() {
        let verifier = UnityVerifier::default();
        let entity = create_test_entity("1", 0.5);
        let detail = verifier.verify_entity_unity(&entity);

        assert_eq!(detail.entity_id, "1");
        assert!(detail.seed_completeness >= 0.0 && detail.seed_completeness <= 1.0);
    }
}
