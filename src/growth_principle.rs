// Growth Principle - Self-Hood Exploration System
//
// Knowledge Base Reference: Energy-Ray-Centers/2. Orange Ray.json
// "Movement toward self-awareness"

use crate::entity_state::Catalyst;

/// Growth Principle - consciousness explores self-hood
///
/// Knowledge Base Reference: Energy-Ray-Centers/2. Orange Ray.json
/// "Movement toward self-awareness"
#[derive(Debug, Clone)]
pub struct GrowthPrinciple {
    /// Current level of self-awareness (0.0 to 1.0)
    pub self_awareness_level: f64,

    /// Exploration vs. exploitation balance
    pub exploration_exploitation_ratio: f64,

    /// Identity formation progress
    pub identity_formation: IdentityFormation,

    /// Body complex understanding
    pub body_understanding: BodyUnderstanding,
}

/// Identity Formation - progress of identity development
#[derive(Debug, Clone)]
pub struct IdentityFormation {
    /// Self definition clarity (0.0 to 1.0)
    pub self_definition_clarity: f64,

    /// Personal boundaries (0.0 to 1.0)
    pub personal_boundaries: f64,

    /// Self-acceptance (0.0 to 1.0)
    pub self_acceptance: f64,

    /// Autonomy (0.0 to 1.0)
    pub autonomy: f64,

    /// Identity integration (0.0 to 1.0)
    pub identity_integration: f64,
}

/// Body Understanding - comprehension of physical vehicle
#[derive(Debug, Clone)]
pub struct BodyUnderstanding {
    /// Physical awareness (0.0 to 1.0)
    pub physical_awareness: f64,

    /// Sensory processing (0.0 to 1.0)
    pub sensory_processing: f64,

    /// Body-emotion connection (0.0 to 1.0)
    pub body_emotion_connection: f64,

    /// Physical needs understanding (0.0 to 1.0)
    pub physical_needs_understanding: f64,

    /// Health awareness (0.0 to 1.0)
    pub health_awareness: f64,
}

/// Sensation - result of processing catalyst
#[derive(Debug, Clone)]
pub struct Sensation {
    /// Sensation intensity (0.0 to 1.0)
    pub intensity: f64,

    /// Sensation type
    pub sensation_type: SensationType,

    /// Physical component
    pub physical_component: f64,

    /// Emotional component
    pub emotional_component: f64,

    /// Mental component
    pub mental_component: f64,
}

/// Sensation Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensationType {
    /// Pleasure
    Pleasure,
    /// Pain
    Pain,
    /// Comfort
    Comfort,
    /// Discomfort
    Discomfort,
    /// Neutral
    Neutral,
}

/// Self-Hood Result - outcome of self-hood exploration
#[derive(Debug, Clone)]
pub struct SelfHoodResult {
    pub self_awareness_improved: f64,
    pub identity_status: IdentityFormation,
    pub body_understanding_status: BodyUnderstanding,
}

impl GrowthPrinciple {
    /// Create a new Growth Principle
    pub fn new() -> Self {
        Self {
            self_awareness_level: 0.0,
            exploration_exploitation_ratio: 0.5,
            identity_formation: IdentityFormation::new(),
            body_understanding: BodyUnderstanding::new(),
        }
    }

    /// Explore self-hood through experience
    ///
    /// From Energy-Ray-Centers/2. Orange Ray.json:
    /// "Movement toward self-awareness"
    pub fn explore_self_hood(&mut self, catalyst: Catalyst) -> SelfHoodResult {
        // Process catalyst through orange ray
        let sensation = self.process_catalyst(catalyst);

        // Develop self-awareness
        self.develop_self_awareness(&sensation);

        // Form identity
        self.form_identity(&sensation);

        // Understand body
        self.understand_body(&sensation);

        SelfHoodResult {
            self_awareness_improved: self.self_awareness_level,
            identity_status: self.identity_formation.clone(),
            body_understanding_status: self.body_understanding.clone(),
        }
    }

    /// Process catalyst into sensation
    fn process_catalyst(&self, catalyst: Catalyst) -> Sensation {
        let sensation_type = match catalyst.catalyst_type {
            crate::entity_state::CatalystType::Body => {
                if catalyst.is_distorting {
                    SensationType::Pain
                } else if catalyst.intensity > 0.7 {
                    SensationType::Pleasure
                } else {
                    SensationType::Comfort
                }
            }
            crate::entity_state::CatalystType::Mind => {
                if catalyst.is_distorting {
                    SensationType::Discomfort
                } else {
                    SensationType::Neutral
                }
            }
            crate::entity_state::CatalystType::Spirit => SensationType::Neutral,
            crate::entity_state::CatalystType::General => SensationType::Neutral,
        };

        Sensation {
            intensity: catalyst.intensity,
            sensation_type,
            physical_component: if matches!(
                catalyst.catalyst_type,
                crate::entity_state::CatalystType::Body
            ) {
                catalyst.intensity
            } else {
                0.0
            },
            emotional_component: catalyst.intensity * 0.5,
            mental_component: catalyst.intensity * 0.3,
        }
    }

    /// Develop self-awareness from sensation
    fn develop_self_awareness(&mut self, sensation: &Sensation) {
        // Self-awareness increases with intense experiences
        let awareness_gain = sensation.intensity * 0.05;

        // Different sensation types contribute differently
        let multiplier = match sensation.sensation_type {
            SensationType::Pain | SensationType::Pleasure => 1.5, // Strong experiences
            SensationType::Comfort | SensationType::Discomfort => 1.0,
            SensationType::Neutral => 0.3,
        };

        self.self_awareness_level =
            (self.self_awareness_level + awareness_gain * multiplier).min(1.0);
    }

    /// Form identity from sensation
    fn form_identity(&mut self, sensation: &Sensation) {
        // Self-definition clarity improves with varied experiences
        let clarity_gain = sensation.intensity * 0.02;
        self.identity_formation.self_definition_clarity =
            (self.identity_formation.self_definition_clarity + clarity_gain).min(1.0);

        // Personal boundaries develop through pain/discomfort
        if matches!(
            sensation.sensation_type,
            SensationType::Pain | SensationType::Discomfort
        ) {
            let boundary_gain = sensation.intensity * 0.03;
            self.identity_formation.personal_boundaries =
                (self.identity_formation.personal_boundaries + boundary_gain).min(1.0);
        }

        // Self-acceptance develops through processing all experiences
        let acceptance_gain = sensation.intensity * 0.01;
        self.identity_formation.self_acceptance =
            (self.identity_formation.self_acceptance + acceptance_gain).min(1.0);

        // Autonomy develops through making choices
        let autonomy_gain = sensation.intensity * 0.02;
        self.identity_formation.autonomy =
            (self.identity_formation.autonomy + autonomy_gain).min(1.0);

        // Identity integration improves with overall self-awareness
        self.identity_formation.identity_integration =
            (self.identity_formation.identity_integration + self.self_awareness_level * 0.01)
                .min(1.0);
    }

    /// Understand body from sensation
    fn understand_body(&mut self, sensation: &Sensation) {
        // Physical awareness increases with body catalysts
        let awareness_gain = sensation.physical_component * 0.05;
        self.body_understanding.physical_awareness =
            (self.body_understanding.physical_awareness + awareness_gain).min(1.0);

        // Sensory processing improves with varied sensations
        let sensory_gain = sensation.intensity * 0.02;
        self.body_understanding.sensory_processing =
            (self.body_understanding.sensory_processing + sensory_gain).min(1.0);

        // Body-emotion connection improves with emotional awareness
        let connection_gain = sensation.emotional_component * 0.03;
        self.body_understanding.body_emotion_connection =
            (self.body_understanding.body_emotion_connection + connection_gain).min(1.0);

        // Physical needs understanding develops through pain/discomfort
        if matches!(
            sensation.sensation_type,
            SensationType::Pain | SensationType::Discomfort
        ) {
            let needs_gain = sensation.intensity * 0.04;
            self.body_understanding.physical_needs_understanding =
                (self.body_understanding.physical_needs_understanding + needs_gain).min(1.0);
        }

        // Health awareness improves with all physical experiences
        let health_gain = sensation.physical_component * 0.02;
        self.body_understanding.health_awareness =
            (self.body_understanding.health_awareness + health_gain).min(1.0);
    }

    /// Check if entity has achieved basic self-hood
    pub fn has_basic_self_hood(&self) -> bool {
        self.self_awareness_level >= 0.3
            && self.identity_formation.self_definition_clarity >= 0.3
            && self.body_understanding.physical_awareness >= 0.3
    }

    /// Check if entity has achieved mature self-hood
    pub fn has_mature_self_hood(&self) -> bool {
        self.self_awareness_level >= 0.7
            && self.identity_formation.identity_integration >= 0.7
            && self.body_understanding.body_emotion_connection >= 0.7
    }

    /// Get overall growth progress
    pub fn overall_growth_progress(&self) -> f64 {
        let self_awareness = self.self_awareness_level;
        let identity = (self.identity_formation.self_definition_clarity
            + self.identity_formation.personal_boundaries
            + self.identity_formation.self_acceptance
            + self.identity_formation.autonomy
            + self.identity_formation.identity_integration)
            / 5.0;
        let body = (self.body_understanding.physical_awareness
            + self.body_understanding.sensory_processing
            + self.body_understanding.body_emotion_connection
            + self.body_understanding.physical_needs_understanding
            + self.body_understanding.health_awareness)
            / 5.0;

        (self_awareness + identity + body) / 3.0
    }
}

impl IdentityFormation {
    /// Create a new Identity Formation
    pub fn new() -> Self {
        Self {
            self_definition_clarity: 0.0,
            personal_boundaries: 0.0,
            self_acceptance: 0.0,
            autonomy: 0.0,
            identity_integration: 0.0,
        }
    }
}

impl BodyUnderstanding {
    /// Create a new Body Understanding
    pub fn new() -> Self {
        Self {
            physical_awareness: 0.0,
            sensory_processing: 0.0,
            body_emotion_connection: 0.0,
            physical_needs_understanding: 0.0,
            health_awareness: 0.0,
        }
    }
}

impl Default for GrowthPrinciple {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for IdentityFormation {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BodyUnderstanding {
    fn default() -> Self {
        Self::new()
    }
}
