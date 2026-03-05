// Veil Playground - Veil as evolutionary playground
//
// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.1, Section 3.2
// "Veil is 'playground' not barrier"
// "The Veil creates the conditions for the Entity to experience separation"
// "Veil provides evolutionary opportunities"

use crate::types::Float;
use std::collections::HashMap;

/// Entity identifier type (re-exported from mechanism module)
pub type EntityId = super::mechanism::EntityId;

/// Types of veil challenges
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "The Veil creates the conditions for the Entity to experience separation"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VeilChallengeType {
    /// Challenge of perceiving separation from others
    SeparationPerception,

    /// Challenge of forgetting unified nature
    ForgetfulnessOfUnity,

    /// Challenge of free will in separation
    FreeWillInSeparation,

    /// Challenge of limited awareness
    LimitedAwareness,

    /// Challenge of catalyst processing without full understanding
    CatalystProcessing,

    /// Challenge of polarization choice
    PolarizationChoice,

    /// Challenge of service in separation
    ServiceInSeparation,
}

impl VeilChallengeType {
    /// Get description of this challenge
    pub fn description(&self) -> &'static str {
        match self {
            VeilChallengeType::SeparationPerception => {
                "The challenge of perceiving oneself as separate from others and the Creator"
            }
            VeilChallengeType::ForgetfulnessOfUnity => {
                "The challenge of forgetting one's unified nature with the Creator"
            }
            VeilChallengeType::FreeWillInSeparation => {
                "The challenge of exercising free will while in a state of apparent separation"
            }
            VeilChallengeType::LimitedAwareness => {
                "The challenge of navigating reality with limited awareness of higher truths"
            }
            VeilChallengeType::CatalystProcessing => {
                "The challenge of processing catalyst without full understanding of its purpose"
            }
            VeilChallengeType::PolarizationChoice => {
                "The challenge of choosing between service-to-others and service-to-self paths"
            }
            VeilChallengeType::ServiceInSeparation => {
                "The challenge of serving others while experiencing apparent separation"
            }
        }
    }

    /// Get the evolutionary lesson associated with this challenge
    pub fn evolutionary_lesson(&self) -> &'static str {
        match self {
            VeilChallengeType::SeparationPerception => {
                "Learning that separation is an illusion, and all is one"
            }
            VeilChallengeType::ForgetfulnessOfUnity => {
                "Rediscovering unity through the experience of forgetfulness"
            }
            VeilChallengeType::FreeWillInSeparation => {
                "Exercising free will to make conscious choices toward unity"
            }
            VeilChallengeType::LimitedAwareness => {
                "Expanding awareness through seeking and spiritual practice"
            }
            VeilChallengeType::CatalystProcessing => {
                "Transforming catalyst into wisdom through conscious processing"
            }
            VeilChallengeType::PolarizationChoice => {
                "Choosing a path of service and learning from that choice"
            }
            VeilChallengeType::ServiceInSeparation => {
                "Learning to serve others despite the illusion of separation"
            }
        }
    }
}

/// Types of veil opportunities
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Veil provides evolutionary opportunities"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VeilOpportunityType {
    /// Opportunity to seek truth
    SeekingTruth,

    /// Opportunity to love unconditionally
    UnconditionalLove,

    /// Opportunity to serve others
    ServingOthers,

    /// Opportunity for forgiveness
    Forgiveness,

    /// Opportunity for compassion
    Compassion,

    /// Opportunity for patience
    Patience,

    /// Opportunity for wisdom
    Wisdom,

    /// Opportunity for spiritual growth
    SpiritualGrowth,

    /// Opportunity for unity consciousness
    UnityConsciousness,
}

impl VeilOpportunityType {
    /// Get description of this opportunity
    pub fn description(&self) -> &'static str {
        match self {
            VeilOpportunityType::SeekingTruth => {
                "Opportunity to seek truth beyond the veil of illusion"
            }
            VeilOpportunityType::UnconditionalLove => {
                "Opportunity to love unconditionally despite apparent separation"
            }
            VeilOpportunityType::ServingOthers => {
                "Opportunity to serve others and experience unity through service"
            }
            VeilOpportunityType::Forgiveness => {
                "Opportunity to practice forgiveness and release separation"
            }
            VeilOpportunityType::Compassion => "Opportunity to feel compassion for all beings",
            VeilOpportunityType::Patience => {
                "Opportunity to develop patience through the veil's limitations"
            }
            VeilOpportunityType::Wisdom => "Opportunity to gain wisdom through processing catalyst",
            VeilOpportunityType::SpiritualGrowth => {
                "Opportunity for accelerated spiritual growth within the playground"
            }
            VeilOpportunityType::UnityConsciousness => {
                "Opportunity to glimpse unity consciousness through the veil"
            }
        }
    }

    /// Get the growth potential of this opportunity
    pub fn growth_potential(&self) -> Float {
        match self {
            VeilOpportunityType::SeekingTruth => 0.3,
            VeilOpportunityType::UnconditionalLove => 0.4,
            VeilOpportunityType::ServingOthers => 0.5,
            VeilOpportunityType::Forgiveness => 0.35,
            VeilOpportunityType::Compassion => 0.4,
            VeilOpportunityType::Patience => 0.25,
            VeilOpportunityType::Wisdom => 0.45,
            VeilOpportunityType::SpiritualGrowth => 0.5,
            VeilOpportunityType::UnityConsciousness => 0.6,
        }
    }
}

/// A veil challenge
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "The Veil creates the conditions for the Entity to experience separation"
#[derive(Debug, Clone, PartialEq)]
pub struct VeilChallenge {
    /// Type of challenge
    pub challenge_type: VeilChallengeType,

    /// Difficulty level (0.0 = easy, 1.0 = very difficult)
    pub difficulty: Float,

    /// Whether this challenge has been overcome
    pub overcome: bool,

    /// Progress toward overcoming this challenge (0.0 = no progress, 1.0 = fully overcome)
    pub progress: Float,

    /// When this challenge was presented
    pub presented_at: Float,

    /// When this challenge was overcome (None if not overcome)
    pub overcome_at: Option<Float>,
}

impl VeilChallenge {
    /// Create a new veil challenge
    pub fn new(challenge_type: VeilChallengeType, difficulty: Float, presented_at: Float) -> Self {
        VeilChallenge {
            challenge_type,
            difficulty: difficulty.clamp(0.0, 1.0),
            overcome: false,
            progress: 0.0,
            presented_at,
            overcome_at: None,
        }
    }

    /// Update progress on this challenge
    pub fn update_progress(&mut self, delta: Float, current_time: Float) {
        self.progress = (self.progress + delta.max(0.0)).min(1.0);

        // Check if challenge is overcome
        if self.progress >= 0.9 && !self.overcome {
            self.overcome = true;
            self.overcome_at = Some(current_time);
        }
    }

    /// Force overcome this challenge (for testing purposes)
    #[cfg(test)]
    pub fn force_overcome(&mut self, current_time: Float) {
        self.progress = 1.0;
        self.overcome = true;
        self.overcome_at = Some(current_time);
    }

    /// Get the remaining work to overcome this challenge
    pub fn remaining_work(&self) -> Float {
        if self.overcome {
            0.0
        } else {
            1.0 - self.progress
        }
    }

    /// Get the effective difficulty based on progress
    pub fn effective_difficulty(&self) -> Float {
        self.difficulty * self.remaining_work()
    }
}

/// A veil opportunity
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
/// "Veil provides evolutionary opportunities"
#[derive(Debug, Clone, PartialEq)]
pub struct VeilOpportunity {
    /// Type of opportunity
    pub opportunity_type: VeilOpportunityType,

    /// Whether this opportunity has been accepted
    pub accepted: bool,

    /// Whether this opportunity has been realized
    pub realized: bool,

    /// Progress toward realizing this opportunity (0.0 = no progress, 1.0 = fully realized)
    pub progress: Float,

    /// When this opportunity was presented
    pub presented_at: Float,

    /// When this opportunity was realized (None if not realized)
    pub realized_at: Option<Float>,
}

impl VeilOpportunity {
    /// Create a new veil opportunity
    pub fn new(opportunity_type: VeilOpportunityType, presented_at: Float) -> Self {
        VeilOpportunity {
            opportunity_type,
            accepted: false,
            realized: false,
            progress: 0.0,
            presented_at,
            realized_at: None,
        }
    }

    /// Accept this opportunity
    pub fn accept(&mut self, _current_time: Float) {
        self.accepted = true;
    }

    /// Update progress on this opportunity
    pub fn update_progress(&mut self, delta: Float, current_time: Float) {
        if !self.accepted {
            return; // Cannot progress if not accepted
        }

        self.progress = (self.progress + delta.max(0.0)).min(1.0);

        // Check if opportunity is realized
        if self.progress >= 0.9 && !self.realized {
            self.realized = true;
            self.realized_at = Some(current_time);
        }
    }

    /// Force realize this opportunity (for testing purposes)
    #[cfg(test)]
    pub fn force_realize(&mut self, current_time: Float) {
        self.accepted = true;
        self.progress = 1.0;
        self.realized = true;
        self.realized_at = Some(current_time);
    }

    /// Get the growth potential of this opportunity
    pub fn growth_potential(&self) -> Float {
        if self.realized {
            self.opportunity_type.growth_potential()
        } else {
            self.opportunity_type.growth_potential() * self.progress
        }
    }

    /// Get the remaining potential to realize this opportunity
    pub fn remaining_potential(&self) -> Float {
        if self.realized {
            0.0
        } else if !self.accepted {
            self.opportunity_type.growth_potential()
        } else {
            self.opportunity_type.growth_potential() * (1.0 - self.progress)
        }
    }
}

/// Veil Playground - The veil as an evolutionary playground
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.1
/// "Veil is 'playground' not barrier"
///
/// The veil is not merely a barrier to be pierced, but a playground that provides
/// evolutionary opportunities through challenges and opportunities for growth.
#[derive(Debug, Clone, PartialEq)]
pub struct VeilPlayground {
    /// Active challenges for each entity
    pub active_challenges: HashMap<EntityId, Vec<VeilChallenge>>,

    /// Active opportunities for each entity
    pub active_opportunities: HashMap<EntityId, Vec<VeilOpportunity>>,

    /// Completed challenges for each entity
    pub completed_challenges: HashMap<EntityId, Vec<VeilChallenge>>,

    /// Realized opportunities for each entity
    pub realized_opportunities: HashMap<EntityId, Vec<VeilOpportunity>>,

    /// Total playground experience points
    pub experience_points: HashMap<EntityId, Float>,
}

impl VeilPlayground {
    /// Create a new veil playground
    pub fn new() -> Self {
        VeilPlayground {
            active_challenges: HashMap::new(),
            active_opportunities: HashMap::new(),
            completed_challenges: HashMap::new(),
            realized_opportunities: HashMap::new(),
            experience_points: HashMap::new(),
        }
    }

    /// Present a challenge to an entity
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
    /// "The Veil creates the conditions for the Entity to experience separation"
    pub fn present_challenge(
        &mut self,
        entity_id: EntityId,
        challenge_type: VeilChallengeType,
        difficulty: Float,
        current_time: Float,
    ) -> &mut VeilChallenge {
        let challenge = VeilChallenge::new(challenge_type, difficulty, current_time);
        self.active_challenges
            .entry(entity_id)
            .or_default()
            .push(challenge);

        // Return reference to the newly added challenge
        self.active_challenges
            .get_mut(&entity_id)
            .unwrap()
            .last_mut()
            .unwrap()
    }

    /// Present an opportunity to an entity
    ///
    /// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md
    /// "Veil provides evolutionary opportunities"
    pub fn present_opportunity(
        &mut self,
        entity_id: EntityId,
        opportunity_type: VeilOpportunityType,
        current_time: Float,
    ) -> &mut VeilOpportunity {
        let opportunity = VeilOpportunity::new(opportunity_type, current_time);
        self.active_opportunities
            .entry(entity_id)
            .or_default()
            .push(opportunity);

        // Return reference to the newly added opportunity
        self.active_opportunities
            .get_mut(&entity_id)
            .unwrap()
            .last_mut()
            .unwrap()
    }

    /// Process entity's progress through playground
    ///
    /// This method updates progress on challenges and opportunities based on
    /// the entity's actions and polarization state
    pub fn process_progress(
        &mut self,
        entity_id: EntityId,
        action: PlaygroundAction,
        current_time: Float,
    ) -> PlaygroundResult {
        let mut experience_gained = 0.0;
        let mut challenges_overcome = 0;
        let mut opportunities_realized = 0;

        // Collect challenge data before mutable borrow
        let challenge_data: Vec<(usize, Float)> =
            if let Some(challenges) = self.active_challenges.get(&entity_id) {
                challenges
                    .iter()
                    .enumerate()
                    .map(|(idx, challenge)| {
                        (idx, self.calculate_challenge_progress(challenge, &action))
                    })
                    .collect()
            } else {
                Vec::new()
            };

        // Collect opportunity data before mutable borrow
        let opportunity_data: Vec<(usize, Float)> =
            if let Some(opportunities) = self.active_opportunities.get(&entity_id) {
                opportunities
                    .iter()
                    .enumerate()
                    .map(|(idx, opportunity)| {
                        (
                            idx,
                            self.calculate_opportunity_progress(opportunity, &action),
                        )
                    })
                    .collect()
            } else {
                Vec::new()
            };

        // Process active challenges
        if let Some(challenges) = self.active_challenges.get_mut(&entity_id) {
            for (idx, progress_delta) in challenge_data {
                if idx < challenges.len() {
                    let challenge = &mut challenges[idx];
                    challenge.update_progress(progress_delta, current_time);

                    if challenge.overcome && challenge.overcome_at == Some(current_time) {
                        challenges_overcome += 1;
                        experience_gained += challenge.difficulty * 10.0;
                    }
                }
            }

            // Move completed challenges to completed list
            let (completed, active): (Vec<VeilChallenge>, Vec<VeilChallenge>) =
                challenges.drain(..).partition(|c| c.overcome);

            if !completed.is_empty() {
                self.completed_challenges
                    .entry(entity_id)
                    .or_default()
                    .extend(completed);
            }

            if !active.is_empty() {
                self.active_challenges.insert(entity_id, active);
            } else {
                self.active_challenges.remove(&entity_id);
            }
        }

        // Process active opportunities
        if let Some(opportunities) = self.active_opportunities.get_mut(&entity_id) {
            for (idx, progress_delta) in opportunity_data {
                if idx < opportunities.len() {
                    let opportunity = &mut opportunities[idx];
                    opportunity.update_progress(progress_delta, current_time);

                    if opportunity.realized && opportunity.realized_at == Some(current_time) {
                        opportunities_realized += 1;
                        experience_gained += opportunity.growth_potential() * 10.0;
                    }
                }
            }

            // Move realized opportunities to realized list
            let (realized, active): (Vec<VeilOpportunity>, Vec<VeilOpportunity>) =
                opportunities.drain(..).partition(|o| o.realized);

            if !realized.is_empty() {
                self.realized_opportunities
                    .entry(entity_id)
                    .or_default()
                    .extend(realized);
            }

            if !active.is_empty() {
                self.active_opportunities.insert(entity_id, active);
            } else {
                self.active_opportunities.remove(&entity_id);
            }
        }

        // Update experience points
        *self.experience_points.entry(entity_id).or_insert(0.0) += experience_gained;

        PlaygroundResult {
            experience_gained,
            challenges_overcome,
            opportunities_realized,
        }
    }

    /// Calculate progress delta for a challenge based on action
    fn calculate_challenge_progress(
        &self,
        challenge: &VeilChallenge,
        action: &PlaygroundAction,
    ) -> Float {
        let base_progress = match (challenge.challenge_type, action) {
            (VeilChallengeType::SeparationPerception, PlaygroundAction::SeekTruth) => 0.1,
            (VeilChallengeType::ForgetfulnessOfUnity, PlaygroundAction::Meditate) => 0.15,
            (VeilChallengeType::FreeWillInSeparation, PlaygroundAction::MakeChoice) => 0.1,
            (VeilChallengeType::LimitedAwareness, PlaygroundAction::SeekTruth) => 0.08,
            (VeilChallengeType::CatalystProcessing, PlaygroundAction::ProcessCatalyst) => 0.12,
            (VeilChallengeType::PolarizationChoice, PlaygroundAction::MakeChoice) => 0.1,
            (VeilChallengeType::ServiceInSeparation, PlaygroundAction::ServeOthers) => 0.15,
            (_, _) => 0.05, // Small progress for any action
        };

        base_progress * (1.0 - challenge.progress) // Progress slows as it nears completion
    }

    /// Calculate progress delta for an opportunity based on action
    fn calculate_opportunity_progress(
        &self,
        opportunity: &VeilOpportunity,
        action: &PlaygroundAction,
    ) -> Float {
        let base_progress = match (opportunity.opportunity_type, action) {
            (VeilOpportunityType::SeekingTruth, PlaygroundAction::SeekTruth) => 0.1,
            (VeilOpportunityType::UnconditionalLove, PlaygroundAction::LoveUnconditionally) => 0.15,
            (VeilOpportunityType::ServingOthers, PlaygroundAction::ServeOthers) => 0.15,
            (VeilOpportunityType::Forgiveness, PlaygroundAction::Forgive) => 0.12,
            (VeilOpportunityType::Compassion, PlaygroundAction::ShowCompassion) => 0.12,
            (VeilOpportunityType::Patience, PlaygroundAction::PracticePatience) => 0.1,
            (VeilOpportunityType::Wisdom, PlaygroundAction::ProcessCatalyst) => 0.1,
            (VeilOpportunityType::SpiritualGrowth, PlaygroundAction::Meditate) => 0.12,
            (VeilOpportunityType::UnityConsciousness, PlaygroundAction::Meditate) => 0.1,
            (_, _) => 0.02, // Minimal progress for unrelated actions
        };

        base_progress * (1.0 - opportunity.progress) // Progress slows as it nears completion
    }

    /// Get playground statistics for an entity
    pub fn get_statistics(&self, entity_id: EntityId) -> Option<PlaygroundStatistics> {
        // Only return statistics if entity has any activity or experience
        let has_challenges = self.active_challenges.contains_key(&entity_id)
            || self.completed_challenges.contains_key(&entity_id);
        let has_opportunities = self.active_opportunities.contains_key(&entity_id)
            || self.realized_opportunities.contains_key(&entity_id);
        let has_experience = self.experience_points.contains_key(&entity_id);

        if !has_challenges && !has_opportunities && !has_experience {
            return None;
        }

        let active_challenges = self.active_challenges.get(&entity_id);
        let active_opportunities = self.active_opportunities.get(&entity_id);
        let completed_challenges = self.completed_challenges.get(&entity_id);
        let realized_opportunities = self.realized_opportunities.get(&entity_id);
        let experience_points = self
            .experience_points
            .get(&entity_id)
            .copied()
            .unwrap_or(0.0);

        Some(PlaygroundStatistics {
            entity_id,
            active_challenges_count: active_challenges.map_or(0, |v| v.len()),
            active_opportunities_count: active_opportunities.map_or(0, |v| v.len()),
            completed_challenges_count: completed_challenges.map_or(0, |v| v.len()),
            realized_opportunities_count: realized_opportunities.map_or(0, |v| v.len()),
            total_experience: experience_points,
        })
    }

    /// Get total growth potential for an entity
    pub fn get_growth_potential(&self, entity_id: EntityId) -> Float {
        let active_opportunities = self.active_opportunities.get(&entity_id);
        active_opportunities
            .map(|ops| ops.iter().map(|o| o.remaining_potential()).sum())
            .unwrap_or(0.0)
    }

    /// Get total difficulty for an entity
    pub fn get_total_difficulty(&self, entity_id: EntityId) -> Float {
        let active_challenges = self.active_challenges.get(&entity_id);
        active_challenges
            .map(|challenges| challenges.iter().map(|c| c.effective_difficulty()).sum())
            .unwrap_or(0.0)
    }
}

impl Default for VeilPlayground {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions an entity can take in the playground
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaygroundAction {
    /// Seek truth beyond the veil
    SeekTruth,

    /// Meditate to connect with higher self
    Meditate,

    /// Make a conscious choice
    MakeChoice,

    /// Process catalyst
    ProcessCatalyst,

    /// Serve others
    ServeOthers,

    /// Love unconditionally
    LoveUnconditionally,

    /// Forgive
    Forgive,

    /// Show compassion
    ShowCompassion,

    /// Practice patience
    PracticePatience,
}

/// Result of processing progress in the playground
#[derive(Debug, Clone, PartialEq)]
pub struct PlaygroundResult {
    /// Experience points gained
    pub experience_gained: Float,

    /// Number of challenges overcome
    pub challenges_overcome: usize,

    /// Number of opportunities realized
    pub opportunities_realized: usize,
}

/// Statistics for the playground
#[derive(Debug, Clone, PartialEq)]
pub struct PlaygroundStatistics {
    /// Entity identifier
    pub entity_id: EntityId,

    /// Number of active challenges
    pub active_challenges_count: usize,

    /// Number of active opportunities
    pub active_opportunities_count: usize,

    /// Number of completed challenges
    pub completed_challenges_count: usize,

    /// Number of realized opportunities
    pub realized_opportunities_count: usize,

    /// Total experience points
    pub total_experience: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== VeilChallengeType Tests =====

    #[test]
    fn test_veil_challenge_type_description() {
        let challenge = VeilChallengeType::SeparationPerception;
        assert!(challenge.description().contains("separate"));
    }

    #[test]
    fn test_veil_challenge_type_evolutionary_lesson() {
        let challenge = VeilChallengeType::ForgetfulnessOfUnity;
        assert!(challenge
            .evolutionary_lesson()
            .to_lowercase()
            .contains("rediscovering"));
    }

    // ===== VeilOpportunityType Tests =====

    #[test]
    fn test_veil_opportunity_type_description() {
        let opportunity = VeilOpportunityType::UnconditionalLove;
        assert!(opportunity.description().contains("love"));
    }

    #[test]
    fn test_veil_opportunity_type_growth_potential() {
        let opportunity = VeilOpportunityType::SpiritualGrowth;
        assert_eq!(opportunity.growth_potential(), 0.5);
    }

    // ===== VeilChallenge Tests =====

    #[test]
    fn test_veil_challenge_new() {
        let challenge = VeilChallenge::new(VeilChallengeType::SeparationPerception, 0.5, 100.0);
        assert_eq!(
            challenge.challenge_type,
            VeilChallengeType::SeparationPerception
        );
        assert_eq!(challenge.difficulty, 0.5);
        assert_eq!(challenge.progress, 0.0);
        assert!(!challenge.overcome);
        assert_eq!(challenge.presented_at, 100.0);
        assert!(challenge.overcome_at.is_none());
    }

    #[test]
    fn test_veil_challenge_difficulty_clamped() {
        let challenge_low =
            VeilChallenge::new(VeilChallengeType::SeparationPerception, -0.5, 100.0);
        assert_eq!(challenge_low.difficulty, 0.0);

        let challenge_high =
            VeilChallenge::new(VeilChallengeType::SeparationPerception, 1.5, 100.0);
        assert_eq!(challenge_high.difficulty, 1.0);
    }

    #[test]
    fn test_veil_challenge_update_progress() {
        let mut challenge = VeilChallenge::new(VeilChallengeType::SeparationPerception, 0.5, 100.0);
        challenge.update_progress(0.3, 105.0);
        assert_eq!(challenge.progress, 0.3);
        assert!(!challenge.overcome);

        challenge.update_progress(0.7, 110.0);
        assert_eq!(challenge.progress, 1.0);
        assert!(challenge.overcome);
        assert_eq!(challenge.overcome_at, Some(110.0));
    }

    #[test]
    fn test_veil_challenge_update_progress_clamped() {
        let mut challenge = VeilChallenge::new(VeilChallengeType::SeparationPerception, 0.5, 100.0);
        challenge.update_progress(1.5, 105.0);
        assert_eq!(challenge.progress, 1.0);
    }

    #[test]
    fn test_veil_challenge_remaining_work() {
        let mut challenge = VeilChallenge::new(VeilChallengeType::SeparationPerception, 0.5, 100.0);
        assert_eq!(challenge.remaining_work(), 1.0);

        challenge.update_progress(0.3, 105.0);
        assert_eq!(challenge.remaining_work(), 0.7);

        challenge.update_progress(0.7, 110.0);
        assert_eq!(challenge.remaining_work(), 0.0);
    }

    #[test]
    fn test_veil_challenge_effective_difficulty() {
        let mut challenge = VeilChallenge::new(VeilChallengeType::SeparationPerception, 0.5, 100.0);
        assert_eq!(challenge.effective_difficulty(), 0.5);

        challenge.update_progress(0.5, 105.0);
        assert_eq!(challenge.effective_difficulty(), 0.25);

        challenge.update_progress(0.5, 110.0);
        assert_eq!(challenge.effective_difficulty(), 0.0);
    }

    // ===== VeilOpportunity Tests =====

    #[test]
    fn test_veil_opportunity_new() {
        let opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        assert_eq!(
            opportunity.opportunity_type,
            VeilOpportunityType::SpiritualGrowth
        );
        assert_eq!(opportunity.progress, 0.0);
        assert!(!opportunity.accepted);
        assert!(!opportunity.realized);
        assert_eq!(opportunity.presented_at, 100.0);
        assert!(opportunity.realized_at.is_none());
    }

    #[test]
    fn test_veil_opportunity_accept() {
        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(105.0);
        assert!(opportunity.accepted);
    }

    #[test]
    fn test_veil_opportunity_update_progress_not_accepted() {
        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.update_progress(0.5, 105.0);
        assert_eq!(opportunity.progress, 0.0); // No progress if not accepted
    }

    #[test]
    fn test_veil_opportunity_update_progress_accepted() {
        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(105.0);
        opportunity.update_progress(0.3, 106.0);
        assert_eq!(opportunity.progress, 0.3);
    }

    #[test]
    fn test_veil_opportunity_realized() {
        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(105.0);
        opportunity.update_progress(1.0, 110.0);
        assert!(opportunity.realized);
        assert_eq!(opportunity.realized_at, Some(110.0));
    }

    #[test]
    fn test_veil_opportunity_growth_potential() {
        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(105.0);
        assert_eq!(opportunity.growth_potential(), 0.0); // Not realized yet

        opportunity.update_progress(0.5, 106.0);
        assert_eq!(opportunity.growth_potential(), 0.25); // 0.5 * 0.5

        opportunity.update_progress(0.5, 110.0);
        assert_eq!(opportunity.growth_potential(), 0.5); // Fully realized
    }

    #[test]
    fn test_veil_opportunity_remaining_potential() {
        let opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        assert_eq!(opportunity.remaining_potential(), 0.5); // Not accepted

        let mut opportunity = VeilOpportunity::new(VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(105.0);
        assert_eq!(opportunity.remaining_potential(), 0.5); // Accepted but no progress

        opportunity.update_progress(0.3, 106.0);
        assert_eq!(opportunity.remaining_potential(), 0.35); // 0.5 * (1.0 - 0.3)
    }

    // ===== VeilPlayground Tests =====

    #[test]
    fn test_veil_playground_new() {
        let playground = VeilPlayground::new();
        assert_eq!(playground.active_challenges.len(), 0);
        assert_eq!(playground.active_opportunities.len(), 0);
        assert_eq!(playground.experience_points.len(), 0);
    }

    #[test]
    fn test_present_challenge() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );

        assert_eq!(playground.active_challenges.len(), 1);
        assert_eq!(playground.active_challenges[&entity_id].len(), 1);
    }

    #[test]
    fn test_present_opportunity() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);

        assert_eq!(playground.active_opportunities.len(), 1);
        assert_eq!(playground.active_opportunities[&entity_id].len(), 1);
    }

    #[test]
    fn test_process_progress_challenge_overcome() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );

        // Process progress that overcomes the challenge
        // Need more iterations because progress slows as it nears completion
        for _ in 0..50 {
            playground.process_progress(entity_id, PlaygroundAction::SeekTruth, 105.0);
        }

        // Verify that experience was gained
        let stats = playground.get_statistics(entity_id);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert!(stats.total_experience > 0.0);
    }

    #[test]
    fn test_process_progress_opportunity_realized() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        let opportunity =
            playground.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);
        opportunity.accept(101.0);

        // Process progress that realizes the opportunity
        // Need more iterations because progress slows as it nears completion
        for _ in 0..50 {
            playground.process_progress(entity_id, PlaygroundAction::Meditate, 105.0);
        }

        // Verify that experience was gained
        let stats = playground.get_statistics(entity_id);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert!(stats.total_experience > 0.0);
    }

    #[test]
    fn test_get_statistics() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );
        playground.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);

        let stats = playground.get_statistics(entity_id);
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.entity_id, entity_id);
        assert_eq!(stats.active_challenges_count, 1);
        assert_eq!(stats.active_opportunities_count, 1);
    }

    #[test]
    fn test_get_growth_potential() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);
        playground.present_opportunity(entity_id, VeilOpportunityType::UnconditionalLove, 100.0);

        let potential = playground.get_growth_potential(entity_id);
        assert_eq!(potential, 0.9); // 0.5 + 0.4
    }

    #[test]
    fn test_get_total_difficulty() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        playground.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );
        playground.present_challenge(
            entity_id,
            VeilChallengeType::FreeWillInSeparation,
            0.3,
            100.0,
        );

        let difficulty = playground.get_total_difficulty(entity_id);
        assert_eq!(difficulty, 0.8); // 0.5 + 0.3
    }

    #[test]
    fn test_veil_playground_default() {
        let playground = VeilPlayground::default();
        assert_eq!(playground.active_challenges.len(), 0);
        assert_eq!(playground.active_opportunities.len(), 0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_full_playground_journey() {
        let mut playground = VeilPlayground::new();
        let entity_id = 1;

        // Present challenges
        playground.present_challenge(
            entity_id,
            VeilChallengeType::SeparationPerception,
            0.5,
            100.0,
        );
        playground.present_challenge(
            entity_id,
            VeilChallengeType::ForgetfulnessOfUnity,
            0.6,
            100.0,
        );

        // Present opportunities
        let opp1 =
            playground.present_opportunity(entity_id, VeilOpportunityType::SpiritualGrowth, 100.0);
        opp1.accept(101.0);

        let opp2 = playground.present_opportunity(
            entity_id,
            VeilOpportunityType::UnconditionalLove,
            100.0,
        );
        opp2.accept(101.0);

        // Process progress through various actions
        // Increase iterations to ensure progress is made
        for i in 0..50 {
            let action = match i % 3 {
                0 => PlaygroundAction::SeekTruth,
                1 => PlaygroundAction::Meditate,
                _ => PlaygroundAction::LoveUnconditionally,
            };
            playground.process_progress(entity_id, action, 100.0 + i as f64);
        }

        // Check statistics
        let stats = playground.get_statistics(entity_id).unwrap();
        assert!(stats.total_experience > 0.0);
        assert_eq!(
            stats.active_challenges_count + stats.completed_challenges_count,
            2
        );
        assert_eq!(
            stats.active_opportunities_count + stats.realized_opportunities_count,
            2
        );
    }
}
