// Ego/Society Dynamics - Yellow-Ray function
//
// Knowledge Base Reference: Energy-Ray-Centers/3. Yellow Ray.json
// "Solar plexus and ego center"

use crate::types::Float;
use serde::{Deserialize, Serialize};

/// Ego/Society Dynamics - Yellow-Ray function
///
/// Knowledge Base Reference: Energy-Ray-Centers/3. Yellow Ray.json
/// "Solar plexus and ego center"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgoDynamics {
    /// Self definition
    ///
    /// From Energy-Ray-Centers/3. Yellow Ray.json:
    /// "Great stepping stone ray"
    pub self_definition: SelfDefinition,

    /// Other perception
    ///
    /// From Energy-Ray-Centers/3. Yellow Ray.json:
    /// "Mind/body potentiation to fullest balance"
    pub other_perception: OtherPerception,

    /// Social identity
    ///
    /// From Energy-Ray-Centers/3. Yellow Ray.json:
    /// "Solar plexus and ego center"
    pub social_identity: SocialIdentity,
}

impl EgoDynamics {
    /// Create new ego dynamics
    pub fn new() -> Self {
        Self {
            self_definition: SelfDefinition::new(),
            other_perception: OtherPerception::new(),
            social_identity: SocialIdentity::new(),
        }
    }

    /// Define self vs other
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/3. Yellow Ray.json
    pub fn define_self_vs_other(&mut self, interactions: &[Interaction]) -> SelfOtherResult {
        // Analyze interactions to define self
        self.self_definition = self.analyze_self_definition(interactions);

        // Analyze interactions to perceive others
        self.other_perception = self.analyze_other_perception(interactions);

        // Form social identity based on self-other distinction
        self.social_identity =
            self.form_social_identity(&self.self_definition, &self.other_perception);

        SelfOtherResult {
            self_definition: self.self_definition.clone(),
            other_perception: self.other_perception.clone(),
            social_identity: self.social_identity.clone(),
        }
    }

    /// Analyze self definition from interactions
    fn analyze_self_definition(&self, interactions: &[Interaction]) -> SelfDefinition {
        let mut self_definition = SelfDefinition::new();

        for interaction in interactions {
            // Update self clarity based on interaction
            self_definition.self_clarity += interaction.intensity * 0.1;
            self_definition.self_clarity = self_definition.self_clarity.min(1.0);

            // Update boundaries based on boundary type
            match interaction.boundary_type {
                BoundaryType::Strong => {
                    self_definition.personal_boundaries += interaction.intensity * 0.15;
                }
                BoundaryType::Moderate => {
                    self_definition.personal_boundaries += interaction.intensity * 0.1;
                }
                BoundaryType::Weak => {
                    self_definition.personal_boundaries += interaction.intensity * 0.05;
                }
            }
            self_definition.personal_boundaries = self_definition.personal_boundaries.min(1.0);

            // Update autonomy based on agency
            self_definition.autonomy += interaction.agency_level * 0.1;
            self_definition.autonomy = self_definition.autonomy.min(1.0);

            // Update identity integration
            self_definition.identity_integration += interaction.intensity * 0.08;
            self_definition.identity_integration = self_definition.identity_integration.min(1.0);
        }

        self_definition
    }

    /// Analyze other perception from interactions
    fn analyze_other_perception(&self, interactions: &[Interaction]) -> OtherPerception {
        let mut other_perception = OtherPerception::new();

        for interaction in interactions {
            // Update other awareness
            other_perception.other_awareness += interaction.intensity * 0.1;
            other_perception.other_awareness = other_perception.other_awareness.min(1.0);

            // Update empathy based on emotional connection
            other_perception.empathy += interaction.emotional_connection * 0.15;
            other_perception.empathy = other_perception.empathy.min(1.0);

            // Update other understanding
            other_perception.other_understanding += interaction.intensity * 0.1;
            other_perception.other_understanding = other_perception.other_understanding.min(1.0);

            // Update recognition of separateness
            other_perception.separateness_recognition += interaction.intensity * 0.1;
            other_perception.separateness_recognition =
                other_perception.separateness_recognition.min(1.0);
        }

        other_perception
    }

    /// Form social identity from self and other perception
    fn form_social_identity(
        &self,
        self_definition: &SelfDefinition,
        other_perception: &OtherPerception,
    ) -> SocialIdentity {
        let mut social_identity = SocialIdentity::new();

        // Social identity emerges from self-other balance
        let self_other_balance =
            (self_definition.self_clarity + other_perception.other_awareness) / 2.0;
        social_identity.social_clarity = self_other_balance;

        // Group belonging based on empathy and boundaries
        social_identity.group_belonging =
            (other_perception.empathy + self_definition.personal_boundaries) / 2.0;

        // Social role clarity
        social_identity.role_clarity =
            (self_definition.autonomy + other_perception.other_understanding) / 2.0;

        // Collective integration
        social_identity.collective_integration = (self_definition.identity_integration
            + other_perception.separateness_recognition)
            / 2.0;

        social_identity
    }

    /// Get overall ego development level
    pub fn ego_development_level(&self) -> Float {
        let self_score = self.self_definition.overall_score();
        let other_score = self.other_perception.overall_score();
        let social_score = self.social_identity.overall_score();

        (self_score + other_score + social_score) / 3.0
    }

    /// Check if ego is well-developed (Yellow-Ray activation)
    pub fn is_yellow_ray_active(&self) -> Float {
        // Yellow-Ray activation requires:
        // - Clear self definition (≥0.5)
        // - Good other perception (≥0.5)
        // - Balanced social identity (≥0.5)
        let self_threshold = self.self_definition.overall_score() >= 0.5;
        let other_threshold = self.other_perception.overall_score() >= 0.5;
        let social_threshold = self.social_identity.overall_score() >= 0.5;

        if self_threshold && other_threshold && social_threshold {
            self.ego_development_level()
        } else {
            0.0
        }
    }
}

impl Default for EgoDynamics {
    fn default() -> Self {
        Self::new()
    }
}

/// Self Definition - how entity defines itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDefinition {
    /// Clarity of self-concept (0.0-1.0)
    pub self_clarity: Float,

    /// Strength of personal boundaries (0.0-1.0)
    pub personal_boundaries: Float,

    /// Level of autonomy (0.0-1.0)
    pub autonomy: Float,

    /// Integration of identity aspects (0.0-1.0)
    pub identity_integration: Float,
}

impl SelfDefinition {
    /// Create new self definition
    pub fn new() -> Self {
        Self {
            self_clarity: 0.0,
            personal_boundaries: 0.0,
            autonomy: 0.0,
            identity_integration: 0.0,
        }
    }

    /// Calculate overall self definition score
    pub fn overall_score(&self) -> Float {
        (self.self_clarity + self.personal_boundaries + self.autonomy + self.identity_integration)
            / 4.0
    }
}

impl Default for SelfDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// Other Perception - how entity perceives others
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherPerception {
    /// Awareness of others as separate entities (0.0-1.0)
    pub other_awareness: Float,

    /// Empathy for others (0.0-1.0)
    pub empathy: Float,

    /// Understanding of other perspectives (0.0-1.0)
    pub other_understanding: Float,

    /// Recognition of separateness while maintaining connection (0.0-1.0)
    pub separateness_recognition: Float,
}

impl OtherPerception {
    /// Create new other perception
    pub fn new() -> Self {
        Self {
            other_awareness: 0.0,
            empathy: 0.0,
            other_understanding: 0.0,
            separateness_recognition: 0.0,
        }
    }

    /// Calculate overall other perception score
    pub fn overall_score(&self) -> Float {
        (self.other_awareness
            + self.empathy
            + self.other_understanding
            + self.separateness_recognition)
            / 4.0
    }
}

impl Default for OtherPerception {
    fn default() -> Self {
        Self::new()
    }
}

/// Social Identity - entity's place in social context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialIdentity {
    /// Clarity of social role (0.0-1.0)
    pub social_clarity: Float,

    /// Sense of belonging to groups (0.0-1.0)
    pub group_belonging: Float,

    /// Understanding of social role (0.0-1.0)
    pub role_clarity: Float,

    /// Integration with collective (0.0-1.0)
    pub collective_integration: Float,
}

impl SocialIdentity {
    /// Create new social identity
    pub fn new() -> Self {
        Self {
            social_clarity: 0.0,
            group_belonging: 0.0,
            role_clarity: 0.0,
            collective_integration: 0.0,
        }
    }

    /// Calculate overall social identity score
    pub fn overall_score(&self) -> Float {
        (self.social_clarity
            + self.group_belonging
            + self.role_clarity
            + self.collective_integration)
            / 4.0
    }
}

impl Default for SocialIdentity {
    fn default() -> Self {
        Self::new()
    }
}

/// Interaction between self and other
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Interaction description
    pub description: String,

    /// Interaction intensity (0.0-1.0)
    pub intensity: Float,

    /// Type of boundary involved
    pub boundary_type: BoundaryType,

    /// Level of agency/exercised will (0.0-1.0)
    pub agency_level: Float,

    /// Emotional connection strength (0.0-1.0)
    pub emotional_connection: Float,

    /// Interaction timestamp
    pub timestamp: Float,
}

impl Interaction {
    /// Create new interaction
    pub fn new(
        description: String,
        intensity: Float,
        boundary_type: BoundaryType,
        agency_level: Float,
        emotional_connection: Float,
        timestamp: Float,
    ) -> Self {
        Self {
            description,
            intensity,
            boundary_type,
            agency_level,
            emotional_connection,
            timestamp,
        }
    }
}

/// Boundary type in interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundaryType {
    /// Strong boundary
    Strong,
    /// Moderate boundary
    Moderate,
    /// Weak boundary
    Weak,
}

/// Result of self-other definition process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfOtherResult {
    /// Resulting self definition
    pub self_definition: SelfDefinition,

    /// Resulting other perception
    pub other_perception: OtherPerception,

    /// Resulting social identity
    pub social_identity: SocialIdentity,
}

/// Self-other distinction quality
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfOtherQuality {
    /// No distinction (pre-ego)
    Undifferentiated,
    /// Emerging distinction (early ego)
    Emerging,
    /// Clear distinction (developed ego)
    Clear,
    /// Balanced distinction (mature ego)
    Balanced,
}

impl SelfOtherResult {
    /// Get the quality of self-other distinction
    pub fn quality(&self) -> SelfOtherQuality {
        let self_score = self.self_definition.overall_score();
        let other_score = self.other_perception.overall_score();
        let avg = (self_score + other_score) / 2.0;

        if avg < 0.25 {
            SelfOtherQuality::Undifferentiated
        } else if avg < 0.5 {
            SelfOtherQuality::Emerging
        } else if avg < 0.75 {
            SelfOtherQuality::Clear
        } else {
            SelfOtherQuality::Balanced
        }
    }

    /// Check if self-other balance is healthy
    pub fn is_balanced(&self) -> bool {
        let self_score = self.self_definition.overall_score();
        let other_score = self.other_perception.overall_score();
        let difference = (self_score - other_score).abs();

        difference < 0.3 // Balanced if within 0.3 of each other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ego_dynamics_creation() {
        let ego = EgoDynamics::new();
        assert_eq!(ego.ego_development_level(), 0.0);
        assert_eq!(ego.is_yellow_ray_active(), 0.0);
    }

    #[test]
    fn test_define_self_vs_other() {
        let mut ego = EgoDynamics::new();
        let interactions = vec![Interaction::new(
            "Test interaction".to_string(),
            0.8,
            BoundaryType::Moderate,
            0.7,
            0.6,
            1.0,
        )];

        let result = ego.define_self_vs_other(&interactions);
        assert!(result.self_definition.self_clarity > 0.0);
        assert!(result.other_perception.other_awareness > 0.0);
    }

    #[test]
    fn test_self_definition_analysis() {
        let ego = EgoDynamics::new();
        let interactions = vec![Interaction::new(
            "Strong boundary".to_string(),
            0.9,
            BoundaryType::Strong,
            0.8,
            0.5,
            1.0,
        )];

        let self_def = ego.analyze_self_definition(&interactions);
        assert!(self_def.personal_boundaries > 0.0);
        assert!(self_def.autonomy > 0.0);
    }

    #[test]
    fn test_other_perception_analysis() {
        let ego = EgoDynamics::new();
        let interactions = vec![Interaction::new(
            "Empathic interaction".to_string(),
            0.8,
            BoundaryType::Moderate,
            0.6,
            0.9,
            1.0,
        )];

        let other_per = ego.analyze_other_perception(&interactions);
        assert!(other_per.empathy > 0.0);
        assert!(other_per.other_awareness > 0.0);
    }

    #[test]
    fn test_yellow_ray_activation() {
        let mut ego = EgoDynamics::new();

        // Create multiple interactions to develop ego
        let interactions = vec![
            Interaction::new(
                "Interaction 1".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                1.0,
            ),
            Interaction::new(
                "Interaction 2".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                2.0,
            ),
            Interaction::new(
                "Interaction 3".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                3.0,
            ),
            Interaction::new(
                "Interaction 4".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                4.0,
            ),
            Interaction::new(
                "Interaction 5".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                5.0,
            ),
            Interaction::new(
                "Interaction 6".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                6.0,
            ),
            Interaction::new(
                "Interaction 7".to_string(),
                0.9,
                BoundaryType::Strong,
                0.8,
                0.7,
                7.0,
            ),
        ];

        ego.define_self_vs_other(&interactions);

        let activation = ego.is_yellow_ray_active();
        // With enough interactions, should activate
        assert!(activation > 0.0);
    }

    #[test]
    fn test_self_other_quality() {
        let mut ego = EgoDynamics::new();
        let interactions = vec![Interaction::new(
            "Test".to_string(),
            0.3,
            BoundaryType::Moderate,
            0.3,
            0.3,
            1.0,
        )];

        let result = ego.define_self_vs_other(&interactions);
        let quality = result.quality();

        // With low intensity interactions, should be Undifferentiated
        assert_eq!(quality, SelfOtherQuality::Undifferentiated);
    }

    #[test]
    fn test_self_other_balance() {
        let mut ego = EgoDynamics::new();
        let interactions = vec![Interaction::new(
            "Balanced interaction".to_string(),
            0.5,
            BoundaryType::Moderate,
            0.5,
            0.5,
            1.0,
        )];

        let result = ego.define_self_vs_other(&interactions);
        assert!(result.is_balanced());
    }

    #[test]
    fn test_self_definition_overall_score() {
        let mut self_def = SelfDefinition::new();
        self_def.self_clarity = 0.8;
        self_def.personal_boundaries = 0.7;
        self_def.autonomy = 0.9;
        self_def.identity_integration = 0.6;

        let score = self_def.overall_score();
        assert!((score - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_other_perception_overall_score() {
        let mut other_per = OtherPerception::new();
        other_per.other_awareness = 0.7;
        other_per.empathy = 0.8;
        other_per.other_understanding = 0.6;
        other_per.separateness_recognition = 0.9;

        let score = other_per.overall_score();
        assert!((score - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_social_identity_overall_score() {
        let mut social_id = SocialIdentity::new();
        social_id.social_clarity = 0.7;
        social_id.group_belonging = 0.8;
        social_id.role_clarity = 0.6;
        social_id.collective_integration = 0.9;

        let score = social_id.overall_score();
        assert!((score - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_interaction_creation() {
        let interaction = Interaction::new(
            "Test".to_string(),
            0.5,
            BoundaryType::Moderate,
            0.6,
            0.7,
            1.0,
        );

        assert_eq!(interaction.intensity, 0.5);
        assert_eq!(interaction.boundary_type, BoundaryType::Moderate);
    }
}
