// Time/Space Coordinate - Metaphysical realm coordinates
//
// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
// "Time/Space: The metaphysical realm where entities review experiences,
// plan incarnations, and access broader consciousness"

use crate::types::Float;

/// Experience Vector - represents accumulated experience in Time/Space
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Where entities review experiences, plan incarnations"
#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceVector {
    /// Mind complex experience
    pub mind_experience: Float,

    /// Body complex experience
    pub body_experience: Float,

    /// Spirit complex experience
    pub spirit_experience: Float,

    /// Total experience level
    pub total_experience: Float,
}

impl ExperienceVector {
    /// Create new experience vector
    pub fn new() -> Self {
        ExperienceVector {
            mind_experience: 0.0,
            body_experience: 0.0,
            spirit_experience: 0.0,
            total_experience: 0.0,
        }
    }

    /// Create experience vector with specific values
    pub fn with_values(mind: Float, body: Float, spirit: Float) -> Self {
        let mind = mind.clamp(0.0, 1.0);
        let body = body.clamp(0.0, 1.0);
        let spirit = spirit.clamp(0.0, 1.0);
        let total = (mind + body + spirit) / 3.0;

        ExperienceVector {
            mind_experience: mind,
            body_experience: body,
            spirit_experience: spirit,
            total_experience: total,
        }
    }

    /// Add experience
    pub fn add_experience(&mut self, mind: Float, body: Float, spirit: Float) {
        self.mind_experience = (self.mind_experience + mind).clamp(0.0, 1.0);
        self.body_experience = (self.body_experience + body).clamp(0.0, 1.0);
        self.spirit_experience = (self.spirit_experience + spirit).clamp(0.0, 1.0);
        self.total_experience =
            (self.mind_experience + self.body_experience + self.spirit_experience) / 3.0;
    }

    /// Get experience for a specific complex type
    pub fn get_complex_experience(&self, complex_type: crate::types::ComplexType) -> Float {
        match complex_type {
            crate::types::ComplexType::Mind => self.mind_experience,
            crate::types::ComplexType::Body => self.body_experience,
            crate::types::ComplexType::Spirit => self.spirit_experience,
            crate::types::ComplexType::Choice => 0.0,
            crate::types::ComplexType::Complex => 0.0,
            crate::types::ComplexType::Experience => 0.0,
            crate::types::ComplexType::LesserCycleState => 0.0,
        }
    }
}

impl Default for ExperienceVector {
    fn default() -> Self {
        Self::new()
    }
}

/// Incarnation Plan - planned experiences for next incarnation
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Plan incarnations"
#[derive(Debug, Clone, PartialEq)]
pub struct IncarnationPlan {
    /// Planned catalyst intensity
    pub planned_catalyst_intensity: Float,

    /// Planned lessons to learn
    pub planned_lessons: Vec<String>,

    /// Planned polarization direction
    pub planned_polarization: Option<crate::types::Polarity>,

    /// Planned density target
    pub planned_density_target: Option<crate::types::Density>,

    /// Plan completeness (0.0 to 1.0)
    pub completeness: Float,
}

impl IncarnationPlan {
    /// Create new incarnation plan
    pub fn new() -> Self {
        IncarnationPlan {
            planned_catalyst_intensity: 0.0,
            planned_lessons: Vec::new(),
            planned_polarization: None,
            planned_density_target: None,
            completeness: 0.0,
        }
    }

    /// Create incarnation plan with specific values
    pub fn with_values(
        catalyst_intensity: Float,
        lessons: Vec<String>,
        polarization: Option<crate::types::Polarity>,
        density_target: Option<crate::types::Density>,
    ) -> Self {
        IncarnationPlan {
            planned_catalyst_intensity: catalyst_intensity.clamp(0.0, 1.0),
            planned_lessons: lessons,
            planned_polarization: polarization,
            planned_density_target: density_target,
            completeness: 0.0,
        }
    }

    /// Add lesson to plan
    pub fn add_lesson(&mut self, lesson: String) {
        self.planned_lessons.push(lesson);
    }

    /// Update completeness
    pub fn set_completeness(&mut self, completeness: Float) {
        self.completeness = completeness.clamp(0.0, 1.0);
    }

    /// Check if plan is complete
    pub fn is_complete(&self) -> bool {
        self.completeness >= 0.9
    }
}

impl Default for IncarnationPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// Broader Consciousness Access - access to higher consciousness
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Access broader consciousness"
#[derive(Debug, Clone, PartialEq)]
pub struct BroaderConsciousnessAccess {
    /// Access to archetypical mind
    pub archetypical_access: Float,

    /// Access to soul stream
    pub soul_stream_access: Float,

    /// Access to intelligent infinity
    pub intelligent_infinity_access: Float,

    /// Total access level
    pub total_access: Float,
}

impl BroaderConsciousnessAccess {
    /// Create new broader consciousness access
    pub fn new() -> Self {
        BroaderConsciousnessAccess {
            archetypical_access: 0.0,
            soul_stream_access: 0.0,
            intelligent_infinity_access: 0.0,
            total_access: 0.0,
        }
    }

    /// Create with specific access levels
    pub fn with_values(archetypical: Float, soul_stream: Float, infinity: Float) -> Self {
        let archetypical = archetypical.clamp(0.0, 1.0);
        let soul_stream = soul_stream.clamp(0.0, 1.0);
        let infinity = infinity.clamp(0.0, 1.0);
        let total = (archetypical + soul_stream + infinity) / 3.0;

        BroaderConsciousnessAccess {
            archetypical_access: archetypical,
            soul_stream_access: soul_stream,
            intelligent_infinity_access: infinity,
            total_access: total,
        }
    }

    /// Increase access levels
    pub fn increase_access(&mut self, archetypical: Float, soul_stream: Float, infinity: Float) {
        self.archetypical_access = (self.archetypical_access + archetypical).clamp(0.0, 1.0);
        self.soul_stream_access = (self.soul_stream_access + soul_stream).clamp(0.0, 1.0);
        self.intelligent_infinity_access = (self.intelligent_infinity_access + infinity)
            .clamp(0.0, 1.0);
        self.total_access =
            (self.archetypical_access + self.soul_stream_access + self.intelligent_infinity_access)
                / 3.0;
    }

    /// Check if entity has broad consciousness access
    pub fn has_broad_access(&self) -> bool {
        self.total_access > 0.5
    }
}

impl Default for BroaderConsciousnessAccess {
    fn default() -> Self {
        Self::new()
    }
}

/// Time/Space Coordinate - metaphysical realm coordinates
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 2.3
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetaphysicalTimeSpaceCoord {
    /// Experience point (accumulated experience)
    pub experience_point: ExperienceVector,

    /// Incarnation planning
    pub incarnation_plan: IncarnationPlan,

    /// Broader consciousness access
    pub broader_consciousness: BroaderConsciousnessAccess,
}

impl MetaphysicalTimeSpaceCoord {
    /// Create new Time/Space coordinate
    pub fn new(
        experience_point: ExperienceVector,
        incarnation_plan: IncarnationPlan,
        broader_consciousness: BroaderConsciousnessAccess,
    ) -> Self {
        MetaphysicalTimeSpaceCoord {
            experience_point,
            incarnation_plan,
            broader_consciousness,
        }
    }

    /// Create Time/Space coordinate from Space/Space coordinate
    pub fn from_space_time(
        _space_time: &crate::coordinates::space_time::PhysicalSpaceTimeCoord,
    ) -> Self {
        // Convert physical position to metaphysical experience
        let experience_point = ExperienceVector::new();
        let incarnation_plan = IncarnationPlan::new();
        let broader_consciousness = BroaderConsciousnessAccess::new();

        MetaphysicalTimeSpaceCoord {
            experience_point,
            incarnation_plan,
            broader_consciousness,
        }
    }

    /// Get total experience level
    pub fn get_total_experience(&self) -> Float {
        self.experience_point.total_experience
    }

    /// Get total consciousness access
    pub fn get_total_consciousness_access(&self) -> Float {
        self.broader_consciousness.total_access
    }

    /// Check if ready for incarnation
    pub fn is_ready_for_incarnation(&self) -> bool {
        self.incarnation_plan.is_complete() && self.get_total_experience() > 0.3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== ExperienceVector Tests =====

    #[test]
    fn test_experience_vector_new() {
        let vector = ExperienceVector::new();
        assert_eq!(vector.mind_experience, 0.0);
        assert_eq!(vector.body_experience, 0.0);
        assert_eq!(vector.spirit_experience, 0.0);
        assert_eq!(vector.total_experience, 0.0);
    }

    #[test]
    fn test_experience_vector_with_values() {
        let vector = ExperienceVector::with_values(0.5, 0.6, 0.7);
        assert_eq!(vector.mind_experience, 0.5);
        assert_eq!(vector.body_experience, 0.6);
        assert_eq!(vector.spirit_experience, 0.7);
        assert_eq!(vector.total_experience, 0.6); // (0.5 + 0.6 + 0.7) / 3 = 0.6
    }

    #[test]
    fn test_experience_vector_with_values_clamped() {
        let vector = ExperienceVector::with_values(1.5, -0.5, 0.5);
        assert_eq!(vector.mind_experience, 1.0); // Clamped to 1.0
        assert_eq!(vector.body_experience, 0.0); // Clamped to 0.0
        assert_eq!(vector.spirit_experience, 0.5);
    }

    #[test]
    fn test_experience_vector_add_experience() {
        let mut vector = ExperienceVector::new();
        vector.add_experience(0.3, 0.4, 0.5);
        assert_eq!(vector.mind_experience, 0.3);
        assert_eq!(vector.body_experience, 0.4);
        assert_eq!(vector.spirit_experience, 0.5);
        assert!((vector.total_experience - 0.4).abs() < 0.001); // (0.3 + 0.4 + 0.5) / 3 = 0.4
    }

    #[test]
    fn test_experience_vector_get_complex_experience() {
        let vector = ExperienceVector::with_values(0.5, 0.6, 0.7);
        assert_eq!(
            vector.get_complex_experience(crate::types::ComplexType::Mind),
            0.5
        );
        assert_eq!(
            vector.get_complex_experience(crate::types::ComplexType::Body),
            0.6
        );
        assert_eq!(
            vector.get_complex_experience(crate::types::ComplexType::Spirit),
            0.7
        );
    }

    #[test]
    fn test_experience_vector_default() {
        let vector = ExperienceVector::default();
        assert_eq!(vector.mind_experience, 0.0);
    }

    // ===== IncarnationPlan Tests =====

    #[test]
    fn test_incarnation_plan_new() {
        let plan = IncarnationPlan::new();
        assert_eq!(plan.planned_catalyst_intensity, 0.0);
        assert!(plan.planned_lessons.is_empty());
        assert!(plan.planned_polarization.is_none());
        assert!(plan.planned_density_target.is_none());
        assert_eq!(plan.completeness, 0.0);
    }

    #[test]
    fn test_incarnation_plan_with_values() {
        let lessons = vec![
            "Learn compassion".to_string(),
            "Practice patience".to_string(),
        ];
        let plan = IncarnationPlan::with_values(
            0.7,
            lessons,
            Some(crate::types::Polarity::STO),
            Some(crate::types::Density::Fourth),
        );
        assert_eq!(plan.planned_catalyst_intensity, 0.7);
        assert_eq!(plan.planned_lessons.len(), 2);
        assert_eq!(plan.completeness, 0.0);
    }

    #[test]
    fn test_incarnation_plan_add_lesson() {
        let mut plan = IncarnationPlan::new();
        plan.add_lesson("Lesson 1".to_string());
        plan.add_lesson("Lesson 2".to_string());
        assert_eq!(plan.planned_lessons.len(), 2);
    }

    #[test]
    fn test_incarnation_plan_set_completeness() {
        let mut plan = IncarnationPlan::new();
        plan.set_completeness(0.9);
        assert_eq!(plan.completeness, 0.9);
    }

    #[test]
    fn test_incarnation_plan_completeness_clamped() {
        let mut plan = IncarnationPlan::new();
        plan.set_completeness(1.5);
        assert_eq!(plan.completeness, 1.0); // Clamped to 1.0
    }

    #[test]
    fn test_incarnation_plan_is_complete() {
        let mut plan = IncarnationPlan::new();
        assert!(!plan.is_complete());
        plan.set_completeness(0.95);
        assert!(plan.is_complete());
    }

    #[test]
    fn test_incarnation_plan_default() {
        let plan = IncarnationPlan::default();
        assert_eq!(plan.planned_catalyst_intensity, 0.0);
    }

    // ===== BroaderConsciousnessAccess Tests =====

    #[test]
    fn test_broader_consciousness_access_new() {
        let access = BroaderConsciousnessAccess::new();
        assert_eq!(access.archetypical_access, 0.0);
        assert_eq!(access.soul_stream_access, 0.0);
        assert_eq!(access.intelligent_infinity_access, 0.0);
        assert_eq!(access.total_access, 0.0);
    }

    #[test]
    fn test_broader_consciousness_access_with_values() {
        let access = BroaderConsciousnessAccess::with_values(0.5, 0.6, 0.7);
        assert_eq!(access.archetypical_access, 0.5);
        assert_eq!(access.soul_stream_access, 0.6);
        assert_eq!(access.intelligent_infinity_access, 0.7);
        assert_eq!(access.total_access, 0.6); // (0.5 + 0.6 + 0.7) / 3 = 0.6
    }

    #[test]
    fn test_broader_consciousness_access_increase_access() {
        let mut access = BroaderConsciousnessAccess::new();
        access.increase_access(0.3, 0.4, 0.5);
        assert_eq!(access.archetypical_access, 0.3);
        assert_eq!(access.soul_stream_access, 0.4);
        assert_eq!(access.intelligent_infinity_access, 0.5);
        assert!((access.total_access - 0.4).abs() < 0.001); // (0.3 + 0.4 + 0.5) / 3 = 0.4
    }

    #[test]
    fn test_broader_consciousness_access_has_broad_access() {
        let access = BroaderConsciousnessAccess::with_values(0.4, 0.4, 0.4); // Total: 0.4
        assert!(!access.has_broad_access());

        let access = BroaderConsciousnessAccess::with_values(0.6, 0.6, 0.6); // Total: 0.6
        assert!(access.has_broad_access());
    }

    #[test]
    fn test_broader_consciousness_access_default() {
        let access = BroaderConsciousnessAccess::default();
        assert_eq!(access.archetypical_access, 0.0);
    }

    // ===== MetaphysicalTimeSpaceCoord Tests =====

    #[test]
    fn test_time_space_coord_new() {
        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::new(),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );
        assert_eq!(coord.get_total_experience(), 0.0);
        assert_eq!(coord.get_total_consciousness_access(), 0.0);
    }

    #[test]
    fn test_time_space_coord_from_space_time() {
        let space_time =
            crate::coordinates::space_time::PhysicalSpaceTimeCoord::new(1.0, 2.0, 3.0, 4.0, 1);
        let time_space = MetaphysicalTimeSpaceCoord::from_space_time(&space_time);
        assert_eq!(time_space.get_total_experience(), 0.0);
    }

    #[test]
    fn test_time_space_coord_get_total_experience() {
        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.5, 0.6, 0.7),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );
        assert_eq!(coord.get_total_experience(), 0.6);
    }

    #[test]
    fn test_time_space_coord_get_total_consciousness_access() {
        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::new(),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::with_values(0.5, 0.6, 0.7),
        );
        assert_eq!(coord.get_total_consciousness_access(), 0.6);
    }

    #[test]
    fn test_time_space_coord_is_ready_for_incarnation() {
        let mut plan = IncarnationPlan::new();
        plan.set_completeness(0.95);

        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.4, 0.4, 0.4), // Total: 0.4
            plan,
            BroaderConsciousnessAccess::new(),
        );
        assert!(coord.is_ready_for_incarnation());
    }

    #[test]
    fn test_time_space_coord_not_ready_for_incarnation_incomplete_plan() {
        let plan = IncarnationPlan::new(); // Completeness: 0.0

        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.4, 0.4, 0.4), // Total: 0.4
            plan,
            BroaderConsciousnessAccess::new(),
        );
        assert!(!coord.is_ready_for_incarnation());
    }

    #[test]
    fn test_time_space_coord_not_ready_for_incarnation_low_experience() {
        let mut plan = IncarnationPlan::new();
        plan.set_completeness(0.95);

        let coord = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.2, 0.2, 0.2), // Total: 0.2
            plan,
            BroaderConsciousnessAccess::new(),
        );
        assert!(!coord.is_ready_for_incarnation());
    }

    #[test]
    fn test_time_space_coord_default() {
        let coord = MetaphysicalTimeSpaceCoord::default();
        assert_eq!(coord.get_total_experience(), 0.0);
    }

    #[test]
    fn test_time_space_coord_clone() {
        let coord1 = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.5, 0.6, 0.7),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );
        let coord2 = coord1.clone();
        assert_eq!(coord1, coord2);
    }

    #[test]
    fn test_time_space_coord_partial_eq() {
        let coord1 = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.5, 0.6, 0.7),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );
        let coord2 = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.5, 0.6, 0.7),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );
        let coord3 = MetaphysicalTimeSpaceCoord::new(
            ExperienceVector::with_values(0.4, 0.6, 0.7),
            IncarnationPlan::new(),
            BroaderConsciousnessAccess::new(),
        );

        assert_eq!(coord1, coord2);
        assert_ne!(coord1, coord3);
    }
}
