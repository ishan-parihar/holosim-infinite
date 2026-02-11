/// True Identity - the entity's identity beyond veils
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "The true vibration of an entity"
///
/// From Ra Material:
/// - The violet ray represents the entity's true identity beyond veils
/// - The entity is the Creator experiencing Itself
/// - True identity is Unity expressing through diversity

/// The entity's true identity beyond veils
///
/// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
/// "The true vibration of an entity"
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Entity is the Creator pretending to be Many"
#[derive(Debug, Clone, PartialEq)]
pub struct TrueIdentity {
    /// Whether the entity is the Creator experiencing Itself
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Entity is the Creator pretending to be Many"
    pub is_creator_experiencing_itself: bool,

    /// Level of unity awareness (0.0 to 1.0)
    ///
    /// How aware the entity is of its unity with the Creator
    ///
    /// From Energy-Ray-Centers/7. Violet Ray.json:
    /// "The true vibration of an entity"
    pub unity_awareness: f64,

    /// Whether the entity contains complete holographic architecture
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Every generic `Entity` contains the full definition of the `Creator`"
    pub holographic_wholeness: bool,

    /// Level of self-knowledge (0.0 to 1.0)
    ///
    /// How well the entity knows itself as the Creator
    pub self_knowledge: f64,

    /// Connection to Intelligent Infinity (0.0 to 1.0)
    ///
    /// How connected the entity is to the source of all creation
    pub connection_to_infinity: f64,
}

impl Default for TrueIdentity {
    fn default() -> Self {
        TrueIdentity {
            is_creator_experiencing_itself: true,
            unity_awareness: 0.0,
            holographic_wholeness: true,
            self_knowledge: 0.0,
            connection_to_infinity: 0.0,
        }
    }
}

impl TrueIdentity {
    /// Create a new true identity
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn new(
        unity_awareness: f64,
        holographic_wholeness: bool,
        self_knowledge: f64,
        connection_to_infinity: f64,
    ) -> Self {
        TrueIdentity {
            is_creator_experiencing_itself: true,
            unity_awareness: unity_awareness.clamp(0.0, 1.0),
            holographic_wholeness,
            self_knowledge: self_knowledge.clamp(0.0, 1.0),
            connection_to_infinity: connection_to_infinity.clamp(0.0, 1.0),
        }
    }

    /// Calculate overall identity development (0.0 to 1.0)
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn identity_development(&self) -> f64 {
        // Identity development is the average of awareness, knowledge, and connection
        (self.unity_awareness + self.self_knowledge + self.connection_to_infinity) / 3.0
    }

    /// Check if the entity has awakened to its true identity
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn is_awakened(&self) -> bool {
        self.unity_awareness >= 0.7 && self.self_knowledge >= 0.6
    }

    /// Check if the entity has achieved conscious unity
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn has_achieved_unity(&self) -> bool {
        self.unity_awareness >= 0.9 && self.connection_to_infinity >= 0.8
    }

    /// Increase unity awareness
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn enhance_unity_awareness(&mut self, amount: f64) {
        self.unity_awareness = (self.unity_awareness + amount).min(1.0);
    }

    /// Increase self-knowledge
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn enhance_self_knowledge(&mut self, amount: f64) {
        self.self_knowledge = (self.self_knowledge + amount).min(1.0);
    }

    /// Increase connection to infinity
    ///
    /// Knowledge Base Reference: Energy-Ray-Centers/7. Violet Ray.json
    pub fn enhance_connection_to_infinity(&mut self, amount: f64) {
        self.connection_to_infinity = (self.connection_to_infinity + amount).min(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_identity_creation() {
        let identity = TrueIdentity::new(0.7, true, 0.6, 0.8);

        assert_eq!(identity.is_creator_experiencing_itself, true);
        assert_eq!(identity.unity_awareness, 0.7);
        assert_eq!(identity.holographic_wholeness, true);
        assert_eq!(identity.self_knowledge, 0.6);
        assert_eq!(identity.connection_to_infinity, 0.8);
    }

    #[test]
    fn test_true_identity_clamping() {
        // Values above 1.0 should be clamped
        let identity = TrueIdentity::new(1.5, true, 2.0, 0.8);

        assert_eq!(identity.unity_awareness, 1.0);
        assert_eq!(identity.self_knowledge, 1.0);
    }

    #[test]
    fn test_identity_development() {
        let identity = TrueIdentity::new(0.7, true, 0.6, 0.8);

        // Development should be average of awareness, knowledge, and connection
        let expected = (0.7 + 0.6 + 0.8) / 3.0;
        assert!((identity.identity_development() - expected).abs() < 0.001);
    }

    #[test]
    fn test_is_awakened() {
        let awakened = TrueIdentity::new(0.8, true, 0.7, 0.8);

        let not_awakened = TrueIdentity::new(0.5, true, 0.4, 0.6);

        assert!(awakened.is_awakened());
        assert!(!not_awakened.is_awakened());
    }

    #[test]
    fn test_has_achieved_unity() {
        let achieved_unity = TrueIdentity::new(0.95, true, 0.9, 0.85);

        let not_achieved_unity = TrueIdentity::new(0.8, true, 0.7, 0.8);

        assert!(achieved_unity.has_achieved_unity());
        assert!(!not_achieved_unity.has_achieved_unity());
    }

    #[test]
    fn test_enhance_unity_awareness() {
        let mut identity = TrueIdentity::new(0.5, true, 0.6, 0.7);

        identity.enhance_unity_awareness(0.2);
        assert_eq!(identity.unity_awareness, 0.7);

        // Should not exceed 1.0
        identity.enhance_unity_awareness(0.5);
        assert_eq!(identity.unity_awareness, 1.0);
    }

    #[test]
    fn test_enhance_self_knowledge() {
        let mut identity = TrueIdentity::new(0.5, true, 0.6, 0.7);

        identity.enhance_self_knowledge(0.2);
        assert_eq!(identity.self_knowledge, 0.8);

        // Should not exceed 1.0
        identity.enhance_self_knowledge(0.5);
        assert_eq!(identity.self_knowledge, 1.0);
    }

    #[test]
    fn test_enhance_connection_to_infinity() {
        let mut identity = TrueIdentity::new(0.5, true, 0.6, 0.7);

        identity.enhance_connection_to_infinity(0.2);
        assert!((identity.connection_to_infinity - 0.9).abs() < 0.001);

        // Should not exceed 1.0
        identity.enhance_connection_to_infinity(0.5);
        assert_eq!(identity.connection_to_infinity, 1.0);
    }

    #[test]
    fn test_true_identity_default() {
        let identity = TrueIdentity::default();

        assert_eq!(identity.is_creator_experiencing_itself, true);
        assert_eq!(identity.unity_awareness, 0.0);
        assert_eq!(identity.holographic_wholeness, true);
        assert_eq!(identity.self_knowledge, 0.0);
        assert_eq!(identity.connection_to_infinity, 0.0);
    }
}
