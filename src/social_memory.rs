// Social Memory Complex - Collective consciousness structure
//
// Knowledge Base Reference: Densities/D4. Fourth Density.json
// "Social memory complex is a group of entities who share a collective consciousness"

use crate::entity_state::Catalyst;
// use crate::memory::soul_stream::PolarizationState; // TODO: Not found in soul_stream
use crate::polarization::PolarizationProgress as PolarizationState; // Use this instead
use crate::types::Float;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Entity ID type
pub type EntityId = usize;

/// Social Memory Complex - collective consciousness structure
///
/// Knowledge Base Reference: Densities/D4. Fourth Density.json
/// "Social memory complex is a group of entities who share a collective consciousness"
#[derive(Debug, Clone)]
pub struct SocialMemoryComplex {
    /// Member entities
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Social memory complex is a group of entities who share
    /// a collective consciousness"
    pub members: Vec<EntityId>,

    /// Shared experiences and wisdom
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Group mind formation"
    pub collective_memory: CollectiveMemory,

    /// Group polarization state
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Collective consciousness emergence"
    pub group_polarization: PolarizationState,

    /// Collective archetypal development
    ///
    /// From Densities/D4. Fourth Density.json:
    /// "Group mind formation"
    pub group_archetype_states: [Float; 22],

    /// Group name/identifier
    pub name: String,

    /// Formation timestamp
    pub formation_time: Float,
}

impl SocialMemoryComplex {
    /// Create a new social memory complex
    pub fn new(name: String, formation_time: Float) -> Self {
        Self {
            members: Vec::new(),
            collective_memory: CollectiveMemory::new(),
            group_polarization: PolarizationState::new(),
            group_archetype_states: [0.0; 22],
            name,
            formation_time,
        }
    }

    /// Add a new member to the social memory complex
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    pub fn add_member(&mut self, entity_id: EntityId) -> JoinResult {
        // Check if entity is already a member
        if self.members.contains(&entity_id) {
            return JoinResult::AlreadyMember;
        }

        // Check if entity is compatible (simplified check)
        if self.is_compatible(entity_id) {
            self.members.push(entity_id);
            JoinResult::Success
        } else {
            JoinResult::Incompatible
        }
    }

    /// Remove a member from the social memory complex
    pub fn remove_member(&mut self, entity_id: EntityId) -> bool {
        if let Some(pos) = self.members.iter().position(|&id| id == entity_id) {
            self.members.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if entity is compatible with the social memory complex
    fn is_compatible(&self, _entity_id: EntityId) -> bool {
        // Simplified compatibility check
        // In a full implementation, this would check:
        // - Polarity alignment
        // - Archetypal compatibility
        // - Evolutionary level
        // - Soul group affiliations
        true
    }

    /// Share experience with all members
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    /// "Collective consciousness emergence"
    pub fn share_experience(&mut self, experience: SocialExperience) {
        // Add to collective memory
        self.collective_memory
            .shared_experiences
            .push(experience.clone());

        // Update collective archetype states based on experience
        self.update_archetypes_from_experience(&experience);
    }

    /// Update collective archetype states from an experience
    fn update_archetypes_from_experience(&mut self, experience: &SocialExperience) {
        // Simplified: update archetype states based on experience type
        for archetype_idx in 0..22 {
            let influence = experience.archetype_influence(archetype_idx);
            self.group_archetype_states[archetype_idx] =
                (self.group_archetype_states[archetype_idx] + influence).min(1.0);
        }
    }

    /// Process collective catalyst
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    pub fn process_collective_catalyst(&mut self, catalyst: Catalyst) -> CollectiveResult {
        // Each member processes the catalyst (simplified - would need entity access)
        let member_results: Vec<_> = self
            .members
            .iter()
            .map(|_id| self.simulate_member_catalyst_processing(catalyst.clone()))
            .collect();

        // Aggregate results into collective wisdom
        let collective_wisdom = self.aggregate_to_collective_wisdom(member_results);

        // Update collective archetype states
        self.update_collective_archetypes(&collective_wisdom);

        CollectiveResult::Processed(collective_wisdom)
    }

    /// Simulate member catalyst processing (simplified)
    fn simulate_member_catalyst_processing(&self, catalyst: Catalyst) -> MemberProcessingResult {
        MemberProcessingResult {
            wisdom_gained: catalyst.intensity * 0.1,
        }
    }

    /// Aggregate member results into collective wisdom
    fn aggregate_to_collective_wisdom(
        &self,
        member_results: Vec<MemberProcessingResult>,
    ) -> WisdomFragment {
        let total_wisdom: Float = member_results.iter().map(|r| r.wisdom_gained).sum();

        WisdomFragment {
            content: format!("Collective wisdom from {} members", member_results.len()),
            wisdom_level: total_wisdom / (member_results.len() as Float + 1.0),
            archetype_contributions: [0.0; 22],
        }
    }

    /// Update collective archetypes based on wisdom
    fn update_collective_archetypes(&mut self, wisdom: &WisdomFragment) {
        for i in 0..22 {
            self.group_archetype_states[i] =
                (self.group_archetype_states[i] + wisdom.archetype_contributions[i]).min(1.0);
        }
    }

    /// Make collective decision
    ///
    /// Knowledge Base Reference: Densities/D4. Fourth Density.json
    pub fn make_collective_decision(&self, context: DecisionContext) -> CollectiveDecision {
        // Gather individual perspectives (simplified)
        let perspectives: Vec<_> = self
            .members
            .iter()
            .map(|_id| self.simulate_member_perspective(context.clone()))
            .collect();

        // Harmonize perspectives into collective decision
        let harmonized = self.harmonize_perspectives(perspectives);

        // Apply group polarization bias
        let biased = self.apply_group_polarization(harmonized);

        CollectiveDecision {
            decision: biased,
            confidence: 0.7,
            member_agreement: 0.8,
        }
    }

    /// Simulate member perspective (simplified)
    fn simulate_member_perspective(&self, _context: DecisionContext) -> Perspective {
        Perspective {
            member_id: 0,
            preferred_choice: 0.5,
            confidence: 0.6,
            reasoning: "Based on individual experience".to_string(),
        }
    }

    /// Harmonize individual perspectives
    fn harmonize_perspectives(&self, perspectives: Vec<Perspective>) -> Float {
        let avg_choice: Float = perspectives
            .iter()
            .map(|p| p.preferred_choice)
            .sum::<Float>()
            / (perspectives.len() as Float + 1.0);
        avg_choice
    }

    /// Apply group polarization bias to decision
    fn apply_group_polarization(&self, base_decision: Float) -> Float {
        // Adjust decision based on group polarization
        let polarization_value = match self.group_polarization.direction {
            crate::polarization::PolarityDirection::ServiceToOthers => 1.0,
            crate::polarization::PolarityDirection::ServiceToSelf => -1.0,
            crate::polarization::PolarityDirection::Neutral => 0.0,
        };
        base_decision + polarization_value * 0.1
    }

    /// Get collective wisdom level
    pub fn collective_wisdom_level(&self) -> Float {
        self.collective_memory
            .collective_wisdom
            .iter()
            .map(|w| w.wisdom_level)
            .sum::<Float>()
            / (self.collective_memory.collective_wisdom.len() as Float + 1.0)
    }

    /// Get average archetype activation
    pub fn average_archetype_activation(&self) -> Float {
        self.group_archetype_states.iter().sum::<Float>() / 22.0
    }

    /// Check if social memory complex is mature
    pub fn is_mature(&self) -> bool {
        self.members.len() >= 3
            && self.collective_wisdom_level() > 0.5
            && self.average_archetype_activation() > 0.5
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

impl Default for SocialMemoryComplex {
    fn default() -> Self {
        Self::new("Unnamed Complex".to_string(), 0.0)
    }
}

/// Collective Memory - shared experiences and wisdom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMemory {
    /// Shared experiences
    pub shared_experiences: Vec<SocialExperience>,

    /// Collective wisdom
    pub collective_wisdom: Vec<WisdomFragment>,

    /// Group learning history
    pub learning_history: Vec<LearningEvent>,
}

impl CollectiveMemory {
    /// Create new collective memory
    pub fn new() -> Self {
        Self {
            shared_experiences: Vec::new(),
            collective_wisdom: Vec::new(),
            learning_history: Vec::new(),
        }
    }

    /// Add wisdom to collective memory
    pub fn add_wisdom(&mut self, wisdom: WisdomFragment) {
        self.collective_wisdom.push(wisdom);
    }

    /// Add learning event to history
    pub fn add_learning_event(&mut self, event: LearningEvent) {
        self.learning_history.push(event);
    }

    /// Get most recent wisdom
    pub fn recent_wisdom(&self, count: usize) -> Vec<&WisdomFragment> {
        let start = self.collective_wisdom.len().saturating_sub(count);
        self.collective_wisdom[start..].iter().collect()
    }
}

impl Default for CollectiveMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Social experience shared among group members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialExperience {
    /// Experience description
    pub description: String,

    /// Experience timestamp
    pub timestamp: Float,

    /// Experience intensity (0.0-1.0)
    pub intensity: Float,

    /// Archetypes influenced by this experience
    pub archetype_influences: [Float; 22],
}

impl SocialExperience {
    /// Create new experience
    pub fn new(description: String, timestamp: Float, intensity: Float) -> Self {
        Self {
            description,
            timestamp,
            intensity,
            archetype_influences: [0.0; 22],
        }
    }

    /// Get archetype influence for specific archetype
    pub fn archetype_influence(&self, archetype_idx: usize) -> Float {
        if archetype_idx < 22 {
            self.archetype_influences[archetype_idx]
        } else {
            0.0
        }
    }

    /// Set archetype influence
    pub fn set_archetype_influence(&mut self, archetype_idx: usize, influence: Float) {
        if archetype_idx < 22 {
            self.archetype_influences[archetype_idx] = influence.min(1.0).max(0.0);
        }
    }
}

/// Wisdom fragment in collective memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WisdomFragment {
    /// Wisdom content
    pub content: String,

    /// Wisdom level (0.0-1.0)
    pub wisdom_level: Float,

    /// Archetype contributions
    pub archetype_contributions: [Float; 22],
}

/// Learning event in group history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    /// Event description
    pub description: String,

    /// Event timestamp
    pub timestamp: Float,

    /// Learning outcome
    pub outcome: LearningOutcome,
}

/// Learning outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningOutcome {
    /// Successful learning
    Success,
    /// Partial learning
    Partial,
    /// Failed learning
    Failed,
}

/// Decision context for collective decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    /// Decision description
    pub description: String,

    /// Available choices
    pub choices: Vec<String>,

    /// Environmental factors
    pub environmental_factors: HashMap<String, Float>,
}

/// Member perspective on a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perspective {
    /// Member ID
    pub member_id: EntityId,

    /// Preferred choice (0.0-1.0)
    pub preferred_choice: Float,

    /// Confidence in choice (0.0-1.0)
    pub confidence: Float,

    /// Reasoning
    pub reasoning: String,
}

/// Member processing result
#[derive(Debug, Clone)]
struct MemberProcessingResult {
    wisdom_gained: Float,
}

/// Collective decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveDecision {
    /// Decision value (0.0-1.0)
    pub decision: Float,

    /// Confidence in decision (0.0-1.0)
    pub confidence: Float,

    /// Member agreement level (0.0-1.0)
    pub member_agreement: Float,
}

/// Collective processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectiveResult {
    /// Catalyst processed successfully
    Processed(WisdomFragment),
    /// Processing failed
    Failed(String),
}

/// Join result for new members
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinResult {
    /// Successfully joined
    Success,
    /// Entity already a member
    AlreadyMember,
    /// Entity incompatible
    Incompatible,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_memory_complex_creation() {
        let smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        assert_eq!(smc.name, "Test Complex");
        assert_eq!(smc.member_count(), 0);
    }

    #[test]
    fn test_add_member() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        let result = smc.add_member(1);
        assert_eq!(result, JoinResult::Success);
        assert_eq!(smc.member_count(), 1);
    }

    #[test]
    fn test_add_duplicate_member() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        smc.add_member(1);
        let result = smc.add_member(1);
        assert_eq!(result, JoinResult::AlreadyMember);
        assert_eq!(smc.member_count(), 1);
    }

    #[test]
    fn test_remove_member() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        smc.add_member(1);
        smc.add_member(2);
        let removed = smc.remove_member(1);
        assert!(removed);
        assert_eq!(smc.member_count(), 1);
    }

    #[test]
    fn test_share_experience() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        let experience = SocialExperience::new("Test experience".to_string(), 1.0, 0.5);
        smc.share_experience(experience);
        assert_eq!(smc.collective_memory.shared_experiences.len(), 1);
    }

    #[test]
    fn test_collective_decision() {
        let smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        let context = DecisionContext {
            description: "Test decision".to_string(),
            choices: vec!["Choice A".to_string(), "Choice B".to_string()],
            environmental_factors: HashMap::new(),
        };
        let decision = smc.make_collective_decision(context);
        assert!(decision.confidence > 0.0);
        assert!(decision.decision >= 0.0 && decision.decision <= 1.0);
    }

    #[test]
    fn test_collective_wisdom_level() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        let wisdom = WisdomFragment {
            content: "Test wisdom".to_string(),
            wisdom_level: 0.7,
            archetype_contributions: [0.0; 22],
        };
        smc.collective_memory.add_wisdom(wisdom.clone());
        // Should average the wisdom levels
        assert!((smc.collective_wisdom_level() - 0.35).abs() < 0.01);
    }

    #[test]
    fn test_maturity_check() {
        let mut smc = SocialMemoryComplex::new("Test Complex".to_string(), 0.0);
        assert!(!smc.is_mature());

        smc.add_member(1);
        smc.add_member(2);
        smc.add_member(3);

        let wisdom = WisdomFragment {
            content: "Test wisdom".to_string(),
            wisdom_level: 0.6,
            archetype_contributions: [0.6; 22],
        };
        smc.collective_memory.add_wisdom(wisdom);

        // With 3 members and one wisdom entry (0.6/2 = 0.3), should not be mature yet
        // Need more wisdom to reach maturity
        assert!(!smc.is_mature());
    }

    #[test]
    fn test_experience_archetype_influence() {
        let mut experience = SocialExperience::new("Test".to_string(), 0.0, 0.5);
        experience.set_archetype_influence(0, 0.8);
        assert_eq!(experience.archetype_influence(0), 0.8);
    }

    #[test]
    fn test_collective_memory_operations() {
        let mut memory = CollectiveMemory::new();
        let wisdom = WisdomFragment {
            content: "Test".to_string(),
            wisdom_level: 0.5,
            archetype_contributions: [0.0; 22],
        };
        memory.add_wisdom(wisdom.clone());
        assert_eq!(memory.collective_wisdom.len(), 1);

        let recent = memory.recent_wisdom(10);
        assert_eq!(recent.len(), 1);
    }
}
