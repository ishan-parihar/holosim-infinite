//! Archetypical Quests System
//!
//! Week 95a: Quests as Archetypical Journeys
//!
//! This module implements quests that follow the 22 archetypical paths (A1-A22) through
//! the 8 densities. Each quest is a journey of consciousness evolution, aligned with
//! the archetypical patterns that shape all experience.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "The Archetypical Mind provides the framework for all experience. Each archetype
//! > represents a fundamental pattern of consciousness evolution. Quests are the journey
//! > through these patterns."
//!
//! ## Key Principles
//!
//! 1. **22 Archetypical Paths**: Each of the 22 archetypes has unique quest paths
//! 2. **8 Density Progression**: Quests progress through the 8 densities (stages aligned with densities)
//! 3. **Catalyst Accumulation**: Quests require catalyst accumulation and resonance contribution
//! 4. **Holographic Signatures**: Each quest has a unique resonance signature
//! 5. **Procedural Generation**: Quests can be generated procedurally from archetype patterns
//! 6. **Archetype Progress Tracking**: Entities track progress through each archetype path
//!
//! ## Quest Structure
//!
//! A quest consists of:
//! - **Stages**: Sequential phases aligned with densities
//! - **Objectives**: Tasks to complete within each stage
//! - **Requirements**: Density, polarity, and resonance prerequisites
//! - **Rewards**: Catalyst earned upon completion
//!
//! ## Integration
//!
//! This module integrates with:
//! - `resonance_trading`: Quests may require trading objectives
//! - `catalyst_combat`: Quests may require combat objectives
//! - `holographic_inventory`: Quest rewards include items
//! - `collective_system`: Some quests require collective participation

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::advanced_game_mechanics::{
    ArchetypeComplex, ArchetypeId, CatalystAmount, Density, Polarity, QuestId, Timestamp,
};
use crate::simulation_v3::collective_manifestation::StructureType;
use crate::simulation_v3::holographic_inventory::{ArchetypicalItemSignature, ResonancePattern};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Unique identifier for a quest stage
pub type StageId = u32;

/// Unique identifier for a quest objective
pub type ObjectiveId = u32;

/// Progress value (0.0 to 1.0)
pub type ProgressValue = Float;

// ============================================================================
// QUEST OBJECTIVE TYPES
// ============================================================================

/// Types of objectives that can be completed in a quest stage
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each archetype provides different catalyst types - the Magician provides
/// transformation catalyst, the High Priestess provides intuition catalyst, etc."
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveType {
    /// Collect a specific resonance pattern
    CollectResonance {
        /// Target resonance pattern to collect
        target_pattern: ResonancePattern,
    },
    /// Generate a specific amount of catalyst
    GenerateCatalyst {
        /// Amount of catalyst to generate
        amount: CatalystAmount,
    },
    /// Complete a number of combat encounters
    CompleteCombat {
        /// Number of combats to complete
        combat_count: u32,
    },
    /// Trade a number of items
    TradeItems {
        /// Number of items to trade
        item_count: u32,
    },
    /// Build a specific structure
    BuildStructure {
        /// Type of structure to build
        structure_type: StructureType,
    },
    /// Craft an item with specific signature
    CraftItem {
        /// Signature of the item to craft
        item_signature: ArchetypicalItemSignature,
    },
    /// Reach a specific density level
    ReachDensity {
        /// Target density to reach
        target_density: Density,
    },
    /// Shift to a specific polarity
    ShiftPolarity {
        /// Target polarity to achieve
        target_polarity: Polarity,
    },
    /// Activate a specific archetype
    ActivateArchetype {
        /// Archetype to activate
        archetype_id: ArchetypeId,
    },
}

impl ObjectiveType {
    /// Get a display name for this objective type
    pub fn display_name(&self) -> String {
        match self {
            ObjectiveType::CollectResonance { .. } => "Collect Resonance".to_string(),
            ObjectiveType::GenerateCatalyst { amount } => {
                format!("Generate {} Catalyst", amount)
            }
            ObjectiveType::CompleteCombat { combat_count } => {
                format!("Complete {} Combats", combat_count)
            }
            ObjectiveType::TradeItems { item_count } => {
                format!("Trade {} Items", item_count)
            }
            ObjectiveType::BuildStructure { structure_type } => {
                format!("Build {:?}", structure_type)
            }
            ObjectiveType::CraftItem { .. } => "Craft Item".to_string(),
            ObjectiveType::ReachDensity { target_density } => {
                format!("Reach {}", target_density.display_name())
            }
            ObjectiveType::ShiftPolarity { target_polarity } => {
                format!("Shift to {}", target_polarity.display_name())
            }
            ObjectiveType::ActivateArchetype { archetype_id } => {
                format!("Activate {}", archetype_id.display_name())
            }
        }
    }
}

// ============================================================================
// QUEST OBJECTIVE
// ============================================================================

/// An individual objective within a quest stage
#[derive(Debug, Clone)]
pub struct QuestObjective {
    /// Unique identifier for this objective
    pub objective_id: ObjectiveId,
    /// Type of objective
    pub objective_type: ObjectiveType,
    /// Target amount/progress needed to complete
    pub target_amount: Float,
    /// Current progress toward completion
    pub current_progress: Float,
    /// Whether this objective is completed
    pub is_completed: bool,
}

impl QuestObjective {
    /// Create a new quest objective
    pub fn new(objective_id: ObjectiveId, objective_type: ObjectiveType) -> Self {
        let target_amount = match &objective_type {
            ObjectiveType::CollectResonance { .. } => 1.0,
            ObjectiveType::GenerateCatalyst { amount } => *amount,
            ObjectiveType::CompleteCombat { combat_count } => *combat_count as Float,
            ObjectiveType::TradeItems { item_count } => *item_count as Float,
            ObjectiveType::BuildStructure { .. } => 1.0,
            ObjectiveType::CraftItem { .. } => 1.0,
            ObjectiveType::ReachDensity { .. } => 1.0,
            ObjectiveType::ShiftPolarity { .. } => 1.0,
            ObjectiveType::ActivateArchetype { .. } => 1.0,
        };

        Self {
            objective_id,
            objective_type,
            target_amount,
            current_progress: 0.0,
            is_completed: false,
        }
    }

    /// Update progress on this objective
    pub fn update_progress(&mut self, progress: Float) {
        self.current_progress = (self.current_progress + progress).min(self.target_amount);
        self.is_completed = self.current_progress >= self.target_amount;
    }

    /// Get completion percentage (0.0 to 1.0)
    pub fn completion_percentage(&self) -> Float {
        if self.target_amount == 0.0 {
            1.0
        } else {
            (self.current_progress / self.target_amount).clamp(0.0, 1.0)
        }
    }

    /// Reset this objective (for repeatable quests)
    pub fn reset(&mut self) {
        self.current_progress = 0.0;
        self.is_completed = false;
    }
}

impl Default for QuestObjective {
    fn default() -> Self {
        Self {
            objective_id: 0,
            objective_type: ObjectiveType::GenerateCatalyst { amount: 100.0 },
            target_amount: 100.0,
            current_progress: 0.0,
            is_completed: false,
        }
    }
}

// ============================================================================
// COMPLETION CRITERIA
// ============================================================================

/// Criteria for completing a quest stage
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CompletionCriteria {
    /// Complete all objectives in the stage
    #[default]
    AllObjectives,
    /// Complete any one objective in the stage
    AnyObjective,
    /// Accumulate a specific amount of catalyst
    CatalystAccumulation {
        /// Amount of catalyst to accumulate
        amount: CatalystAmount,
    },
    /// Reach a specific resonance threshold
    ResonanceThreshold {
        /// Resonance pattern to achieve
        pattern: ResonancePattern,
    },
}

impl CompletionCriteria {
    /// Check if stage is complete based on this criteria
    pub fn is_complete(
        &self,
        objectives: &[QuestObjective],
        catalyst_accumulated: CatalystAmount,
    ) -> bool {
        match self {
            CompletionCriteria::AllObjectives => objectives.iter().all(|o| o.is_completed),
            CompletionCriteria::AnyObjective => objectives.iter().any(|o| o.is_completed),
            CompletionCriteria::CatalystAccumulation { amount } => catalyst_accumulated >= *amount,
            CompletionCriteria::ResonanceThreshold { pattern } => {
                // Check if collective resonance matches target pattern
                // Simplified: check if any objective's resonance collection matches
                objectives.iter().any(|o| {
                    if let ObjectiveType::CollectResonance { target_pattern } = &o.objective_type {
                        o.is_completed && target_pattern.compute_interference(pattern) > 0.8
                    } else {
                        false
                    }
                })
            }
        }
    }

    /// Get display description
    pub fn description(&self) -> String {
        match self {
            CompletionCriteria::AllObjectives => "Complete all objectives".to_string(),
            CompletionCriteria::AnyObjective => "Complete any objective".to_string(),
            CompletionCriteria::CatalystAccumulation { amount } => {
                format!("Accumulate {} catalyst", amount)
            }
            CompletionCriteria::ResonanceThreshold { .. } => {
                "Reach resonance threshold".to_string()
            }
        }
    }
}

// ============================================================================
// QUEST STAGE
// ============================================================================

/// An individual stage within a quest
///
/// Each stage represents a density level that the entity must work through
#[derive(Debug, Clone)]
pub struct QuestStage {
    /// Unique identifier for this stage
    pub stage_id: StageId,
    /// Name of the stage
    pub name: String,
    /// Description of the stage
    pub description: String,
    /// Which density this stage operates in
    pub target_density: Density,
    /// Objectives to complete in this stage
    pub objectives: Vec<QuestObjective>,
    /// Catalyst threshold to complete this stage
    pub catalyst_threshold: CatalystAmount,
    /// Minimum resonance required to attempt this stage
    pub resonance_requirement: ResonancePattern,
    /// How to complete this stage
    pub completion_criteria: CompletionCriteria,
}

impl QuestStage {
    /// Create a new quest stage
    pub fn new(stage_id: StageId, name: String, target_density: Density) -> Self {
        Self {
            stage_id,
            name,
            description: String::new(),
            target_density,
            objectives: Vec::new(),
            catalyst_threshold: 0.0,
            resonance_requirement: ResonancePattern::new(),
            completion_criteria: CompletionCriteria::AllObjectives,
        }
    }

    /// Add an objective to this stage
    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Set the catalyst threshold
    pub fn with_catalyst_threshold(mut self, threshold: CatalystAmount) -> Self {
        self.catalyst_threshold = threshold;
        self
    }

    /// Set the resonance requirement
    pub fn with_resonance_requirement(mut self, requirement: ResonancePattern) -> Self {
        self.resonance_requirement = requirement;
        self
    }

    /// Set the completion criteria
    pub fn with_completion_criteria(mut self, criteria: CompletionCriteria) -> Self {
        self.completion_criteria = criteria;
        self
    }

    /// Get overall stage progress (0.0 to 1.0)
    pub fn stage_progress(&self) -> Float {
        if self.objectives.is_empty() {
            return 1.0;
        }
        let total_progress: Float = self
            .objectives
            .iter()
            .map(|o| o.completion_percentage())
            .sum();
        total_progress / self.objectives.len() as Float
    }

    /// Check if all objectives are completed
    pub fn are_objectives_complete(&self) -> bool {
        self.objectives.iter().all(|o| o.is_completed)
    }

    /// Find an objective by ID
    pub fn get_objective(&self, objective_id: ObjectiveId) -> Option<&QuestObjective> {
        self.objectives
            .iter()
            .find(|o| o.objective_id == objective_id)
    }

    /// Find a mutable objective by ID
    pub fn get_objective_mut(&mut self, objective_id: ObjectiveId) -> Option<&mut QuestObjective> {
        self.objectives
            .iter_mut()
            .find(|o| o.objective_id == objective_id)
    }
}

impl Default for QuestStage {
    fn default() -> Self {
        Self {
            stage_id: 0,
            name: "Default Stage".to_string(),
            description: String::new(),
            target_density: Density::FIRST,
            objectives: Vec::new(),
            catalyst_threshold: 0.0,
            resonance_requirement: ResonancePattern::new(),
            completion_criteria: CompletionCriteria::AllObjectives,
        }
    }
}

// ============================================================================
// QUEST
// ============================================================================

/// Represents an archetypical journey through the densities
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each quest follows an archetypical path - the journey of the Magician,
/// the transformation of Death, the wisdom of the Hermit."
#[derive(Debug, Clone)]
pub struct Quest {
    /// Unique identifier for this quest
    pub quest_id: QuestId,
    /// Which archetype this quest follows
    pub archetype_path: ArchetypeId,
    /// Name of the quest
    pub name: String,
    /// Description of the quest
    pub description: String,
    /// Stages of this quest
    pub stages: Vec<QuestStage>,
    /// Minimum density required to start this quest
    pub required_density: Density,
    /// Optional polarity requirement
    pub required_polarity: Option<Polarity>,
    /// Catalyst earned on completion
    pub catalyst_reward: CatalystAmount,
    /// Quest's holographic resonance signature
    pub resonance_signature: ResonancePattern,
    /// Whether this quest can be repeated
    pub is_repeatable: bool,
}

impl Quest {
    /// Create a new quest
    pub fn new(quest_id: QuestId, archetype_path: ArchetypeId, name: String) -> Self {
        Self {
            quest_id,
            archetype_path,
            name,
            description: String::new(),
            stages: Vec::new(),
            required_density: Density::FIRST,
            required_polarity: None,
            catalyst_reward: 100.0,
            resonance_signature: ResonancePattern::new(),
            is_repeatable: false,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a stage to this quest
    pub fn with_stage(mut self, stage: QuestStage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Set the required density
    pub fn with_required_density(mut self, density: Density) -> Self {
        self.required_density = density;
        self
    }

    /// Set the required polarity
    pub fn with_required_polarity(mut self, polarity: Polarity) -> Self {
        self.required_polarity = Some(polarity);
        self
    }

    /// Set the catalyst reward
    pub fn with_catalyst_reward(mut self, reward: CatalystAmount) -> Self {
        self.catalyst_reward = reward;
        self
    }

    /// Set the resonance signature
    pub fn with_resonance_signature(mut self, signature: ResonancePattern) -> Self {
        self.resonance_signature = signature;
        self
    }

    /// Set whether this quest is repeatable
    pub fn with_repeatable(mut self, repeatable: bool) -> Self {
        self.is_repeatable = repeatable;
        self
    }

    /// Get the number of stages
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    /// Get a stage by ID
    pub fn get_stage(&self, stage_id: StageId) -> Option<&QuestStage> {
        self.stages.iter().find(|s| s.stage_id == stage_id)
    }

    /// Get a mutable stage by ID
    pub fn get_stage_mut(&mut self, stage_id: StageId) -> Option<&mut QuestStage> {
        self.stages.iter_mut().find(|s| s.stage_id == stage_id)
    }

    /// Check if an entity meets the requirements to start this quest
    pub fn check_requirements(
        &self,
        entity_density: Density,
        entity_polarity: Option<Polarity>,
    ) -> Result<(), QuestError> {
        if entity_density < self.required_density {
            return Err(QuestError::InsufficientDensity {
                required: self.required_density,
                actual: entity_density,
            });
        }

        if let Some(required) = self.required_polarity {
            if let Some(actual) = entity_polarity {
                if actual != required {
                    return Err(QuestError::WrongPolarity { required, actual });
                }
            } else {
                return Err(QuestError::WrongPolarity {
                    required,
                    actual: Polarity::Balanced,
                });
            }
        }

        Ok(())
    }

    /// Generate a resonance pattern based on the archetype
    pub fn generate_archetype_resonance(&self) -> ResonancePattern {
        let mut pattern = ResonancePattern::new();
        // Use archetype ID to influence the resonance pattern
        let archetype_index = self.archetype_path.as_index();
        for i in 0..8 {
            pattern.pattern[i] = ((archetype_index + i) % 8) as Float / 8.0;
        }
        pattern.stability = 0.8 + (self.archetype_path.value() as Float / 22.0) * 0.2;
        pattern
    }
}

impl Default for Quest {
    fn default() -> Self {
        Self {
            quest_id: QuestId::new(0),
            archetype_path: ArchetypeId::A1_MAGICIAN,
            name: "Default Quest".to_string(),
            description: String::new(),
            stages: Vec::new(),
            required_density: Density::FIRST,
            required_polarity: None,
            catalyst_reward: 100.0,
            resonance_signature: ResonancePattern::new(),
            is_repeatable: false,
        }
    }
}

// ============================================================================
// STAGE PROGRESS
// ============================================================================

/// Tracks progress through a specific quest stage
#[derive(Debug, Clone)]
pub struct StageProgress {
    /// Stage identifier
    pub stage_id: StageId,
    /// Which objectives have been completed
    pub objectives_completed: Vec<bool>,
    /// Catalyst contributed toward this stage
    pub catalyst_contributed: CatalystAmount,
    /// Whether this stage is complete
    pub is_complete: bool,
}

impl StageProgress {
    /// Create new stage progress
    pub fn new(stage_id: StageId, objective_count: usize) -> Self {
        Self {
            stage_id,
            objectives_completed: vec![false; objective_count],
            catalyst_contributed: 0.0,
            is_complete: false,
        }
    }

    /// Mark an objective as completed
    pub fn complete_objective(&mut self, objective_index: usize) {
        if objective_index < self.objectives_completed.len() {
            self.objectives_completed[objective_index] = true;
        }
    }

    /// Add catalyst contribution
    pub fn add_catalyst(&mut self, amount: CatalystAmount) {
        self.catalyst_contributed += amount;
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> Float {
        if self.objectives_completed.is_empty() {
            return if self.is_complete { 1.0 } else { 0.0 };
        }
        let completed = self.objectives_completed.iter().filter(|&&x| x).count();
        completed as Float / self.objectives_completed.len() as Float
    }

    /// Check if all objectives are complete
    pub fn all_objectives_complete(&self) -> bool {
        self.objectives_completed.iter().all(|&x| x)
    }
}

impl Default for StageProgress {
    fn default() -> Self {
        Self {
            stage_id: 0,
            objectives_completed: Vec::new(),
            catalyst_contributed: 0.0,
            is_complete: false,
        }
    }
}

// ============================================================================
// ACTIVE QUEST
// ============================================================================

/// An active quest instance for a specific entity
#[derive(Debug, Clone)]
pub struct ActiveQuest {
    /// Quest identifier
    pub quest_id: QuestId,
    /// Current stage the entity is on
    pub current_stage: StageId,
    /// Progress for each stage
    pub stage_progress: HashMap<StageId, StageProgress>,
    /// When the quest was started
    pub start_time: Timestamp,
    /// Total catalyst accumulated during this quest
    pub catalyst_accumulated: CatalystAmount,
    /// Resonance contributed during this quest
    pub resonance_contributed: ResonancePattern,
}

impl ActiveQuest {
    /// Create a new active quest instance
    pub fn new(quest_id: QuestId, start_time: Timestamp) -> Self {
        Self {
            quest_id,
            current_stage: 1, // Start at stage 1
            stage_progress: HashMap::new(),
            start_time,
            catalyst_accumulated: 0.0,
            resonance_contributed: ResonancePattern::new(),
        }
    }

    /// Initialize stage progress for a quest
    pub fn initialize_stages(&mut self, quest: &Quest) {
        for stage in &quest.stages {
            let progress = StageProgress::new(stage.stage_id, stage.objectives.len());
            self.stage_progress.insert(stage.stage_id, progress);
        }
    }

    /// Advance to the next stage
    pub fn advance_stage(&mut self) -> Result<(), QuestError> {
        self.current_stage += 1;
        Ok(())
    }

    /// Add catalyst to the quest
    pub fn add_catalyst(&mut self, amount: CatalystAmount) {
        self.catalyst_accumulated += amount;
    }

    /// Add resonance contribution
    pub fn add_resonance(&mut self, resonance: &ResonancePattern) {
        for i in 0..8 {
            self.resonance_contributed.pattern[i] =
                (self.resonance_contributed.pattern[i] + resonance.pattern[i]) / 2.0;
        }
    }

    /// Get progress for a specific stage
    pub fn get_stage_progress(&self, stage_id: StageId) -> Option<&StageProgress> {
        self.stage_progress.get(&stage_id)
    }

    /// Get mutable progress for a specific stage
    pub fn get_stage_progress_mut(&mut self, stage_id: StageId) -> Option<&mut StageProgress> {
        self.stage_progress.get_mut(&stage_id)
    }

    /// Get overall quest progress (0.0 to 1.0)
    pub fn overall_progress(&self, total_stages: usize) -> Float {
        if total_stages == 0 {
            return 1.0;
        }
        let stage_progress = (self.current_stage as Float - 1.0) / total_stages as Float;
        let current_stage_progress = self
            .stage_progress
            .get(&self.current_stage)
            .map(|p| p.completion_percentage())
            .unwrap_or(0.0)
            / total_stages as Float;
        stage_progress + current_stage_progress
    }
}

impl Default for ActiveQuest {
    fn default() -> Self {
        Self {
            quest_id: QuestId::new(0),
            current_stage: 1,
            stage_progress: HashMap::new(),
            start_time: 0.0,
            catalyst_accumulated: 0.0,
            resonance_contributed: ResonancePattern::new(),
        }
    }
}

// ============================================================================
// QUEST PROGRESS
// ============================================================================

/// Tracks an entity's overall progress through the quest system
#[derive(Debug, Clone)]
pub struct QuestProgress {
    /// Entity identifier
    pub entity_id: EntityId,
    /// Currently active quests
    pub active_quests: HashMap<QuestId, ActiveQuest>,
    /// Completed quest IDs
    pub completed_quests: Vec<QuestId>,
    /// Total catalyst earned from quests
    pub total_catalyst_earned: CatalystAmount,
    /// Progress per archetype (0.0 to 1.0)
    pub archetype_progress: HashMap<ArchetypeId, Float>,
}

impl QuestProgress {
    /// Create new quest progress for an entity
    pub fn new(entity_id: EntityId) -> Self {
        let mut archetype_progress = HashMap::new();
        // Initialize all 22 archetypes with 0 progress
        for i in 1..=22 {
            archetype_progress.insert(ArchetypeId::new(i), 0.0);
        }

        Self {
            entity_id,
            active_quests: HashMap::new(),
            completed_quests: Vec::new(),
            total_catalyst_earned: 0.0,
            archetype_progress,
        }
    }

    /// Start a new quest
    pub fn start_quest(
        &mut self,
        quest: &Quest,
        start_time: Timestamp,
    ) -> Result<ActiveQuest, QuestError> {
        let quest_id = quest.quest_id;

        // Check if already active
        if self.active_quests.contains_key(&quest_id) {
            return Err(QuestError::QuestAlreadyActive);
        }

        // Check if already completed (and not repeatable)
        if self.completed_quests.contains(&quest_id) && !quest.is_repeatable {
            return Err(QuestError::QuestAlreadyCompleted);
        }

        let mut active = ActiveQuest::new(quest_id, start_time);
        active.initialize_stages(quest);
        self.active_quests.insert(quest_id, active.clone());

        Ok(active)
    }

    /// Complete a quest
    pub fn complete_quest(
        &mut self,
        quest_id: QuestId,
        archetype_id: ArchetypeId,
    ) -> Result<CatalystAmount, QuestError> {
        let active = self
            .active_quests
            .remove(&quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        self.completed_quests.push(quest_id);

        // Update archetype progress
        let current_progress = self
            .archetype_progress
            .get(&archetype_id)
            .copied()
            .unwrap_or(0.0);
        let new_progress = (current_progress + 0.1).min(1.0);
        self.archetype_progress.insert(archetype_id, new_progress);

        Ok(active.catalyst_accumulated)
    }

    /// Get active quest
    pub fn get_active_quest(&self, quest_id: QuestId) -> Option<&ActiveQuest> {
        self.active_quests.get(&quest_id)
    }

    /// Get mutable active quest
    pub fn get_active_quest_mut(&mut self, quest_id: QuestId) -> Option<&mut ActiveQuest> {
        self.active_quests.get_mut(&quest_id)
    }

    /// Check if quest is completed
    pub fn is_quest_completed(&self, quest_id: QuestId) -> bool {
        self.completed_quests.contains(&quest_id)
    }

    /// Get archetype progress
    pub fn get_archetype_progress(&self, archetype_id: ArchetypeId) -> Float {
        self.archetype_progress
            .get(&archetype_id)
            .copied()
            .unwrap_or(0.0)
    }

    /// Update archetype progress
    pub fn update_archetype_progress(&mut self, archetype_id: ArchetypeId, progress: Float) {
        let current = self
            .archetype_progress
            .get(&archetype_id)
            .copied()
            .unwrap_or(0.0);
        self.archetype_progress
            .insert(archetype_id, (current + progress).min(1.0));
    }

    /// Get total number of completed quests
    pub fn completed_quest_count(&self) -> usize {
        self.completed_quests.len()
    }

    /// Get number of active quests
    pub fn active_quest_count(&self) -> usize {
        self.active_quests.len()
    }
}

impl Default for QuestProgress {
    fn default() -> Self {
        Self::new(EntityId::new("default".to_string()))
    }
}

// ============================================================================
// ARCHETYPICAL PATH
// ============================================================================

/// Represents one of the 22 archetypical paths through consciousness evolution
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each archetype provides a unique path of evolution - the Magician's path of
/// transformation, the Hermit's path of introspection, the Star's path of hope."
#[derive(Debug, Clone)]
pub struct ArchetypicalPath {
    /// Archetype identifier
    pub archetype_id: ArchetypeId,
    /// Name of this path
    pub path_name: String,
    /// Description of this path
    pub path_description: String,
    /// Quests associated with this path
    pub associated_quests: Vec<QuestId>,
    /// Dominant polarity for this path
    pub dominant_polarity: Polarity,
    /// Density range for this path (min, max)
    pub density_range: (Density, Density),
    /// Resonance signature of this path
    pub resonance_pattern: ResonancePattern,
}

impl ArchetypicalPath {
    /// Create a new archetypical path
    pub fn new(archetype_id: ArchetypeId, path_name: String) -> Self {
        Self {
            archetype_id,
            path_name,
            path_description: String::new(),
            associated_quests: Vec::new(),
            dominant_polarity: Polarity::Balanced,
            density_range: (Density::FIRST, Density::EIGHTH),
            resonance_pattern: ResonancePattern::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: String) -> Self {
        self.path_description = description;
        self
    }

    /// Add an associated quest
    pub fn with_quest(mut self, quest_id: QuestId) -> Self {
        self.associated_quests.push(quest_id);
        self
    }

    /// Set the dominant polarity
    pub fn with_polarity(mut self, polarity: Polarity) -> Self {
        self.dominant_polarity = polarity;
        self
    }

    /// Set the density range
    pub fn with_density_range(mut self, min: Density, max: Density) -> Self {
        self.density_range = (min, max);
        self
    }

    /// Set the resonance pattern
    pub fn with_resonance_pattern(mut self, pattern: ResonancePattern) -> Self {
        self.resonance_pattern = pattern;
        self
    }

    /// Check if a density is within this path's range
    pub fn is_density_in_range(&self, density: Density) -> bool {
        density >= self.density_range.0 && density <= self.density_range.1
    }

    /// Get the archetype complex this path belongs to
    pub fn complex(&self) -> ArchetypeComplex {
        self.archetype_id.complex()
    }

    /// Generate a default path for an archetype
    pub fn generate_default_path(archetype_id: ArchetypeId) -> Self {
        let path_name = format!("Path of {}", archetype_id.display_name());
        let description = format!(
            "The journey through the {} archetype, exploring its lessons across all densities.",
            archetype_id.display_name()
        );

        // Determine dominant polarity based on archetype
        let polarity = match archetype_id.value() {
            1..=7 => Polarity::Balanced,         // Mind complex - balanced
            8..=14 => Polarity::ServiceToOthers, // Body complex - STO leaning
            15..=21 => Polarity::ServiceToSelf,  // Spirit complex - STS leaning
            22 => Polarity::Balanced,            // The Choice - balanced
            _ => Polarity::Balanced,
        };

        // Generate resonance pattern based on archetype
        let mut pattern = ResonancePattern::new();
        let idx = archetype_id.as_index();
        for i in 0..8 {
            pattern.pattern[i] = ((idx + i * 3) % 8) as Float / 8.0;
        }
        pattern.stability = 0.7 + (archetype_id.value() as Float / 22.0) * 0.3;

        Self::new(archetype_id, path_name)
            .with_description(description)
            .with_polarity(polarity)
            .with_density_range(Density::FIRST, Density::EIGHTH)
            .with_resonance_pattern(pattern)
    }
}

impl Default for ArchetypicalPath {
    fn default() -> Self {
        Self {
            archetype_id: ArchetypeId::A1_MAGICIAN,
            path_name: "Default Path".to_string(),
            path_description: String::new(),
            associated_quests: Vec::new(),
            dominant_polarity: Polarity::Balanced,
            density_range: (Density::FIRST, Density::EIGHTH),
            resonance_pattern: ResonancePattern::new(),
        }
    }
}

// ============================================================================
// QUEST ERROR
// ============================================================================

/// Error types for the quest system
#[derive(Debug, Clone, PartialEq)]
pub enum QuestError {
    /// Quest not found
    QuestNotFound,
    /// Stage not found
    StageNotFound,
    /// Entity doesn't have sufficient density
    InsufficientDensity {
        /// Required density
        required: Density,
        /// Actual density
        actual: Density,
    },
    /// Entity doesn't have sufficient catalyst
    InsufficientCatalyst {
        /// Required catalyst
        required: CatalystAmount,
        /// Actual catalyst
        actual: CatalystAmount,
    },
    /// Resonance is incompatible
    IncompatibleResonance,
    /// Wrong polarity for quest
    WrongPolarity {
        /// Required polarity
        required: Polarity,
        /// Actual polarity
        actual: Polarity,
    },
    /// Quest is already active
    QuestAlreadyActive,
    /// Quest is already completed and not repeatable
    QuestAlreadyCompleted,
    /// Stage is not complete
    StageNotComplete,
    /// Objective not met
    ObjectiveNotMet,
    /// Objective not found
    ObjectiveNotFound,
    /// Entity not found
    EntityNotFound(EntityId),
}

impl std::fmt::Display for QuestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestError::QuestNotFound => write!(f, "Quest not found"),
            QuestError::StageNotFound => write!(f, "Stage not found"),
            QuestError::InsufficientDensity { required, actual } => {
                write!(
                    f,
                    "Insufficient density: required={}, actual={}",
                    required, actual
                )
            }
            QuestError::InsufficientCatalyst { required, actual } => {
                write!(
                    f,
                    "Insufficient catalyst: required={}, actual={}",
                    required, actual
                )
            }
            QuestError::IncompatibleResonance => write!(f, "Incompatible resonance"),
            QuestError::WrongPolarity { required, actual } => {
                write!(
                    f,
                    "Wrong polarity: required={}, actual={}",
                    required, actual
                )
            }
            QuestError::QuestAlreadyActive => write!(f, "Quest already active"),
            QuestError::QuestAlreadyCompleted => write!(f, "Quest already completed"),
            QuestError::StageNotComplete => write!(f, "Stage not complete"),
            QuestError::ObjectiveNotMet => write!(f, "Objective not met"),
            QuestError::ObjectiveNotFound => write!(f, "Objective not found"),
            QuestError::EntityNotFound(id) => write!(f, "Entity not found: {:?}", id),
        }
    }
}

impl std::error::Error for QuestError {}

// ============================================================================
// ARCHETYPICAL QUESTS SYSTEM
// ============================================================================

/// Main quest system that manages all archetypical quests
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The quest system provides the framework for consciousness evolution through
/// the archetypical patterns. Each entity's journey is unique yet follows
/// universal patterns."
pub struct ArchetypicalQuests {
    /// Available quests by ID
    pub available_quests: HashMap<QuestId, Quest>,
    /// Archetype paths by archetype ID
    pub archetype_paths: HashMap<ArchetypeId, ArchetypicalPath>,
    /// Entity quest progress
    pub entity_progress: HashMap<EntityId, QuestProgress>,
    /// Next quest ID to assign
    next_quest_id: u64,
    /// Current simulation time
    current_time: Timestamp,
}

impl ArchetypicalQuests {
    /// Create a new quest system
    pub fn new() -> Self {
        let mut system = Self {
            available_quests: HashMap::new(),
            archetype_paths: HashMap::new(),
            entity_progress: HashMap::new(),
            next_quest_id: 1,
            current_time: 0.0,
        };

        // Initialize default archetype paths
        system.initialize_archetype_paths();

        system
    }

    /// Initialize all 22 archetype paths
    fn initialize_archetype_paths(&mut self) {
        for i in 1..=22 {
            let archetype_id = ArchetypeId::new(i);
            let path = ArchetypicalPath::generate_default_path(archetype_id);
            self.archetype_paths.insert(archetype_id, path);
        }
    }

    /// Get the next quest ID
    fn next_quest_id(&mut self) -> QuestId {
        let id = QuestId::new(self.next_quest_id);
        self.next_quest_id += 1;
        id
    }

    /// Create a new quest
    pub fn create_quest(
        &mut self,
        archetype_id: ArchetypeId,
        name: String,
        description: String,
        stages: Vec<QuestStage>,
    ) -> Quest {
        let quest_id = self.next_quest_id();
        let mut quest = Quest::new(quest_id, archetype_id, name)
            .with_description(description)
            .with_resonance_signature(ResonancePattern::new());

        for stage in stages {
            quest.stages.push(stage);
        }

        // Generate resonance signature based on archetype
        quest.resonance_signature = quest.generate_archetype_resonance();

        self.available_quests.insert(quest_id, quest.clone());

        // Add to archetype path
        if let Some(path) = self.archetype_paths.get_mut(&archetype_id) {
            path.associated_quests.push(quest_id);
        }

        quest
    }

    /// Start a quest for an entity
    pub fn start_quest(
        &mut self,
        entity_id: EntityId,
        quest_id: QuestId,
        entity_density: Density,
        entity_polarity: Option<Polarity>,
    ) -> Result<ActiveQuest, QuestError> {
        let quest = self
            .available_quests
            .get(&quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        // Check requirements
        quest.check_requirements(entity_density, entity_polarity)?;

        // Get or create entity progress
        let progress = self
            .entity_progress
            .entry(entity_id.clone())
            .or_insert_with(|| QuestProgress::new(entity_id));

        // Start the quest
        let active = progress.start_quest(quest, self.current_time)?;

        Ok(active)
    }

    /// Update quest progress with catalyst and resonance contribution
    pub fn update_quest_progress(
        &mut self,
        entity_id: EntityId,
        quest_id: QuestId,
        catalyst_contributed: CatalystAmount,
        resonance_contributed: &ResonancePattern,
    ) -> Result<(), QuestError> {
        let progress = self
            .entity_progress
            .get_mut(&entity_id)
            .ok_or(QuestError::EntityNotFound(entity_id))?;

        let active = progress
            .get_active_quest_mut(quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        active.add_catalyst(catalyst_contributed);
        active.add_resonance(resonance_contributed);

        // Update stage progress
        if let Some(stage_progress) = active.get_stage_progress_mut(active.current_stage) {
            stage_progress.add_catalyst(catalyst_contributed);
        }

        Ok(())
    }

    /// Complete a quest stage
    pub fn complete_stage(
        &mut self,
        entity_id: EntityId,
        quest_id: QuestId,
        stage_id: StageId,
    ) -> Result<(), QuestError> {
        let quest = self
            .available_quests
            .get(&quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        let progress = self
            .entity_progress
            .get_mut(&entity_id)
            .ok_or(QuestError::EntityNotFound(entity_id))?;

        let active = progress
            .get_active_quest_mut(quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        // Check if current stage matches
        if active.current_stage != stage_id {
            return Err(QuestError::StageNotComplete);
        }

        // Clone catalyst_accumulated before mutable borrow
        let catalyst_accumulated = active.catalyst_accumulated;

        // Get stage progress
        let stage_progress = active
            .get_stage_progress_mut(stage_id)
            .ok_or(QuestError::StageNotFound)?;

        // Get stage definition
        let stage = quest.get_stage(stage_id).ok_or(QuestError::StageNotFound)?;

        // Check if stage can be completed
        if !stage
            .completion_criteria
            .is_complete(&stage.objectives, catalyst_accumulated)
        {
            return Err(QuestError::StageNotComplete);
        }

        // Mark stage as complete
        stage_progress.is_complete = true;

        // Advance to next stage or complete quest
        let is_last_stage = stage_id as usize >= quest.stages.len();
        if is_last_stage {
            // Quest will be completed separately
        } else {
            active.advance_stage()?;
        }

        Ok(())
    }

    /// Complete a quest
    pub fn complete_quest(
        &mut self,
        entity_id: EntityId,
        quest_id: QuestId,
    ) -> Result<CatalystAmount, QuestError> {
        let quest = self
            .available_quests
            .get(&quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        let archetype_id = quest.archetype_path;
        let catalyst_reward = quest.catalyst_reward;

        let progress = self
            .entity_progress
            .get_mut(&entity_id)
            .ok_or(QuestError::EntityNotFound(entity_id))?;

        // Complete the quest
        let earned = progress.complete_quest(quest_id, archetype_id)?;

        // Add reward
        let total_earned = earned + catalyst_reward;
        progress.total_catalyst_earned += total_earned;

        Ok(total_earned)
    }

    /// Get available quests for an entity
    pub fn get_available_quests(
        &self,
        entity_id: EntityId,
        entity_density: Density,
        entity_polarity: Option<Polarity>,
    ) -> Vec<QuestId> {
        let completed: Vec<QuestId> = self
            .entity_progress
            .get(&entity_id)
            .map(|p| p.completed_quests.clone())
            .unwrap_or_default();

        let active: Vec<QuestId> = self
            .entity_progress
            .get(&entity_id)
            .map(|p| p.active_quests.keys().copied().collect())
            .unwrap_or_default();

        self.available_quests
            .values()
            .filter(|quest| {
                // Check if already active
                if active.contains(&quest.quest_id) {
                    return false;
                }

                // Check if completed and not repeatable
                if completed.contains(&quest.quest_id) && !quest.is_repeatable {
                    return false;
                }

                // Check requirements
                quest
                    .check_requirements(entity_density, entity_polarity)
                    .is_ok()
            })
            .map(|quest| quest.quest_id)
            .collect()
    }

    /// Get active quests for an entity
    pub fn get_active_quests(&self, entity_id: EntityId) -> Vec<&ActiveQuest> {
        self.entity_progress
            .get(&entity_id)
            .map(|p| p.active_quests.values().collect())
            .unwrap_or_default()
    }

    /// Get completed quests for an entity
    pub fn get_completed_quests(&self, entity_id: EntityId) -> Vec<QuestId> {
        self.entity_progress
            .get(&entity_id)
            .map(|p| p.completed_quests.clone())
            .unwrap_or_default()
    }

    /// Get archetype progress for an entity
    pub fn get_archetype_progress(&self, entity_id: EntityId, archetype_id: ArchetypeId) -> Float {
        self.entity_progress
            .get(&entity_id)
            .map(|p| p.get_archetype_progress(archetype_id))
            .unwrap_or(0.0)
    }

    /// Generate a quest procedurally from an archetype
    pub fn generate_quest_from_archetype(&mut self, archetype_id: ArchetypeId) -> Quest {
        let path = self
            .archetype_paths
            .get(&archetype_id)
            .cloned()
            .unwrap_or_else(|| ArchetypicalPath::generate_default_path(archetype_id));

        let quest_name = format!("Journey of the {}", archetype_id.display_name());
        let description = format!(
            "A quest following the path of {} through the densities of consciousness evolution.",
            archetype_id.display_name()
        );

        // Generate stages based on archetype complex
        let stages = self.generate_stages_for_archetype(archetype_id);

        let mut quest = self.create_quest(archetype_id, quest_name, description, stages);

        // Set polarity requirement based on path
        if path.dominant_polarity != Polarity::Balanced {
            quest.required_polarity = Some(path.dominant_polarity);
        }

        // Set resonance signature from path
        quest.resonance_signature = path.resonance_pattern.clone();

        // Update in available quests
        self.available_quests.insert(quest.quest_id, quest.clone());

        quest
    }

    /// Generate stages for an archetype
    fn generate_stages_for_archetype(&self, archetype_id: ArchetypeId) -> Vec<QuestStage> {
        let mut stages = Vec::new();

        // Generate 3-5 stages based on archetype
        let num_stages = 3 + (archetype_id.value() % 3);

        for i in 0..num_stages {
            let stage_id = (i + 1) as StageId;
            let density = Density::new(i + 1);

            let stage_name = format!("Stage {}: {}", stage_id, density.display_name());
            let description = format!(
                "Explore the lessons of {} in the {}.",
                archetype_id.display_name(),
                density.display_name()
            );

            let mut stage = QuestStage::new(stage_id, stage_name, density)
                .with_description(description)
                .with_catalyst_threshold(50.0 * (i + 1) as Float);

            // Add objectives based on archetype
            let objectives = self.generate_objectives_for_archetype(archetype_id, i as usize);
            for objective in objectives {
                stage.objectives.push(objective);
            }

            stages.push(stage);
        }

        stages
    }

    /// Generate objectives for an archetype at a given stage
    fn generate_objectives_for_archetype(
        &self,
        archetype_id: ArchetypeId,
        stage_index: usize,
    ) -> Vec<QuestObjective> {
        let mut objectives = Vec::new();

        // Generate 1-3 objectives per stage
        let num_objectives = 1 + (stage_index % 3);

        for i in 0..num_objectives {
            let objective_id = (stage_index * 10 + i) as ObjectiveId;

            let objective_type = match (archetype_id.value(), i) {
                // Magician - transformation and catalyst
                (1, 0) => ObjectiveType::GenerateCatalyst { amount: 100.0 },
                (1, 1) => ObjectiveType::ActivateArchetype {
                    archetype_id: ArchetypeId::A1_MAGICIAN,
                },
                (1, _) => ObjectiveType::ReachDensity {
                    target_density: Density::new((stage_index + 2) as u8),
                },

                // High Priestess - intuition and resonance
                (2, 0) => ObjectiveType::CollectResonance {
                    target_pattern: ResonancePattern::new(),
                },
                (2, 1) => ObjectiveType::ActivateArchetype {
                    archetype_id: ArchetypeId::A2_HIGH_PRIESTESS,
                },
                (2, _) => ObjectiveType::CraftItem {
                    item_signature: ArchetypicalItemSignature::new(),
                },

                // Empress - abundance and creation
                (3, 0) => ObjectiveType::TradeItems { item_count: 5 },
                (3, 1) => ObjectiveType::BuildStructure {
                    structure_type: StructureType::ResonanceCollector,
                },
                (3, _) => ObjectiveType::GenerateCatalyst { amount: 150.0 },

                // Emperor - structure and authority
                (4, 0) => ObjectiveType::BuildStructure {
                    structure_type: StructureType::ResonanceCollector,
                },
                (4, 1) => ObjectiveType::CompleteCombat { combat_count: 3 },
                (4, _) => ObjectiveType::ReachDensity {
                    target_density: Density::new((stage_index + 2) as u8),
                },

                // Default - mix of objectives
                (_, 0) => ObjectiveType::GenerateCatalyst {
                    amount: 100.0 + (i as Float * 50.0),
                },
                (_, 1) => ObjectiveType::ActivateArchetype { archetype_id },
                (_, _) => ObjectiveType::ReachDensity {
                    target_density: Density::new((stage_index + 2) as u8),
                },
            };

            objectives.push(QuestObjective::new(objective_id, objective_type));
        }

        objectives
    }

    /// Update an objective's progress
    pub fn update_objective_progress(
        &mut self,
        entity_id: EntityId,
        quest_id: QuestId,
        stage_id: StageId,
        objective_id: ObjectiveId,
        _progress: Float,
    ) -> Result<(), QuestError> {
        let quest = self
            .available_quests
            .get(&quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        let progress_data = self
            .entity_progress
            .get_mut(&entity_id)
            .ok_or(QuestError::EntityNotFound(entity_id))?;

        let active = progress_data
            .get_active_quest_mut(quest_id)
            .ok_or(QuestError::QuestNotFound)?;

        // Update stage progress
        if let Some(stage_progress) = active.get_stage_progress_mut(stage_id) {
            // Find the objective index
            if let Some(stage) = quest.get_stage(stage_id) {
                for (idx, objective) in stage.objectives.iter().enumerate() {
                    if objective.objective_id == objective_id {
                        stage_progress.complete_objective(idx);
                        return Ok(());
                    }
                }
            }
        }

        Err(QuestError::ObjectiveNotFound)
    }

    /// Advance simulation time
    pub fn advance_time(&mut self, delta_time: Float) {
        self.current_time += delta_time;
    }

    /// Get current time
    pub fn current_time(&self) -> Timestamp {
        self.current_time
    }

    /// Get quest by ID
    pub fn get_quest(&self, quest_id: QuestId) -> Option<&Quest> {
        self.available_quests.get(&quest_id)
    }

    /// Get archetype path
    pub fn get_archetype_path(&self, archetype_id: ArchetypeId) -> Option<&ArchetypicalPath> {
        self.archetype_paths.get(&archetype_id)
    }

    /// Get entity progress
    pub fn get_entity_progress(&self, entity_id: EntityId) -> Option<&QuestProgress> {
        self.entity_progress.get(&entity_id)
    }

    /// Get all quests for an archetype
    pub fn get_quests_for_archetype(&self, archetype_id: ArchetypeId) -> Vec<&Quest> {
        self.available_quests
            .values()
            .filter(|q| q.archetype_path == archetype_id)
            .collect()
    }
}

impl Default for ArchetypicalQuests {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // QUEST CREATION TESTS
    // ============================================================================

    #[test]
    fn test_quest_creation() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
        )
        .with_description("A test quest".to_string())
        .with_required_density(Density::THIRD)
        .with_catalyst_reward(500.0)
        .with_repeatable(true);

        assert_eq!(quest.quest_id.as_u64(), 1);
        assert_eq!(quest.archetype_path, ArchetypeId::A1_MAGICIAN);
        assert_eq!(quest.name, "Test Quest");
        assert_eq!(quest.required_density, Density::THIRD);
        assert_eq!(quest.catalyst_reward, 500.0);
        assert!(quest.is_repeatable);
    }

    #[test]
    fn test_quest_stage_creation() {
        let stage = QuestStage::new(1, "First Stage".to_string(), Density::THIRD)
            .with_description("Complete the first challenges".to_string())
            .with_catalyst_threshold(100.0);

        assert_eq!(stage.stage_id, 1);
        assert_eq!(stage.name, "First Stage");
        assert_eq!(stage.target_density, Density::THIRD);
        assert_eq!(stage.catalyst_threshold, 100.0);
    }

    #[test]
    fn test_quest_objective_creation() {
        let objective = QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 });

        assert_eq!(objective.objective_id, 1);
        assert_eq!(objective.target_amount, 100.0);
        assert_eq!(objective.current_progress, 0.0);
        assert!(!objective.is_completed);
    }

    // ============================================================================
    // QUEST REQUIREMENTS TESTS
    // ============================================================================

    #[test]
    fn test_quest_requirements_met() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        )
        .with_required_density(Density::THIRD);

        let result = quest.check_requirements(Density::FOURTH, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quest_insufficient_density() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        )
        .with_required_density(Density::FOURTH);

        let result = quest.check_requirements(Density::THIRD, None);
        assert!(matches!(
            result,
            Err(QuestError::InsufficientDensity { .. })
        ));
    }

    #[test]
    fn test_quest_wrong_polarity() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        )
        .with_required_polarity(Polarity::ServiceToOthers);

        let result = quest.check_requirements(Density::THIRD, Some(Polarity::ServiceToSelf));
        assert!(matches!(result, Err(QuestError::WrongPolarity { .. })));
    }

    #[test]
    fn test_quest_correct_polarity() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        )
        .with_required_polarity(Polarity::ServiceToOthers);

        let result = quest.check_requirements(Density::THIRD, Some(Polarity::ServiceToOthers));
        assert!(result.is_ok());
    }

    // ============================================================================
    // QUEST PROGRESS TESTS
    // ============================================================================

    #[test]
    fn test_quest_progress_creation() {
        let progress = QuestProgress::new(EntityId::new("test-entity-1".to_string()));

        assert_eq!(progress.active_quest_count(), 0);
        assert_eq!(progress.completed_quest_count(), 0);
        assert_eq!(progress.total_catalyst_earned, 0.0);
    }

    #[test]
    fn test_start_quest() {
        let mut progress = QuestProgress::new(EntityId::new("test-entity-2".to_string()));
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        );

        let active = progress.start_quest(&quest, 0.0);
        assert!(active.is_ok());
        assert_eq!(progress.active_quest_count(), 1);
    }

    #[test]
    fn test_start_duplicate_quest_fails() {
        let mut progress = QuestProgress::new(EntityId::new("test-entity-3".to_string()));
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        );

        let _ = progress.start_quest(&quest, 0.0);
        let result = progress.start_quest(&quest, 0.0);

        assert!(matches!(result, Err(QuestError::QuestAlreadyActive)));
    }

    #[test]
    fn test_complete_quest() {
        let mut progress = QuestProgress::new(EntityId::new("test-entity-4".to_string()));
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        )
        .with_catalyst_reward(100.0);

        let _ = progress.start_quest(&quest, 0.0);
        let earned = progress.complete_quest(QuestId::new(1), ArchetypeId::A1_MAGICIAN);

        assert!(earned.is_ok());
        assert_eq!(progress.completed_quest_count(), 1);
        assert_eq!(progress.active_quest_count(), 0);
    }

    #[test]
    fn test_archetype_progress() {
        let mut progress = QuestProgress::new(EntityId::new("test-entity-5".to_string()));

        progress.update_archetype_progress(ArchetypeId::A1_MAGICIAN, 0.25);
        assert_eq!(
            progress.get_archetype_progress(ArchetypeId::A1_MAGICIAN),
            0.25
        );

        progress.update_archetype_progress(ArchetypeId::A1_MAGICIAN, 0.5);
        assert_eq!(
            progress.get_archetype_progress(ArchetypeId::A1_MAGICIAN),
            0.75
        );
    }

    // ============================================================================
    // OBJECTIVE PROGRESS TESTS
    // ============================================================================

    #[test]
    fn test_objective_progress_update() {
        let mut objective =
            QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 });

        assert_eq!(objective.completion_percentage(), 0.0);

        objective.update_progress(50.0);
        assert_eq!(objective.current_progress, 50.0);
        assert_eq!(objective.completion_percentage(), 0.5);
        assert!(!objective.is_completed);

        objective.update_progress(60.0);
        assert!(objective.is_completed);
        assert_eq!(objective.completion_percentage(), 1.0);
    }

    #[test]
    fn test_objective_reset() {
        let mut objective =
            QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 });

        objective.update_progress(100.0);
        assert!(objective.is_completed);

        objective.reset();
        assert!(!objective.is_completed);
        assert_eq!(objective.current_progress, 0.0);
    }

    // ============================================================================
    // STAGE PROGRESS TESTS
    // ============================================================================

    #[test]
    fn test_stage_progress_creation() {
        let progress = StageProgress::new(1, 3);

        assert_eq!(progress.stage_id, 1);
        assert_eq!(progress.objectives_completed.len(), 3);
        assert!(!progress.is_complete);
    }

    #[test]
    fn test_stage_progress_completion() {
        let mut progress = StageProgress::new(1, 3);

        progress.complete_objective(0);
        progress.complete_objective(1);
        progress.complete_objective(2);

        assert!(progress.all_objectives_complete());
        assert_eq!(progress.completion_percentage(), 1.0);
    }

    #[test]
    fn test_stage_catalyst_contribution() {
        let mut progress = StageProgress::new(1, 2);

        progress.add_catalyst(50.0);
        assert_eq!(progress.catalyst_contributed, 50.0);

        progress.add_catalyst(30.0);
        assert_eq!(progress.catalyst_contributed, 80.0);
    }

    // ============================================================================
    // ACTIVE QUEST TESTS
    // ============================================================================

    #[test]
    fn test_active_quest_creation() {
        let active = ActiveQuest::new(QuestId::new(1), 100.0);

        assert_eq!(active.quest_id.as_u64(), 1);
        assert_eq!(active.current_stage, 1);
        assert_eq!(active.start_time, 100.0);
        assert_eq!(active.catalyst_accumulated, 0.0);
    }

    #[test]
    fn test_active_quest_catalyst_accumulation() {
        let mut active = ActiveQuest::new(QuestId::new(1), 0.0);

        active.add_catalyst(50.0);
        assert_eq!(active.catalyst_accumulated, 50.0);

        active.add_catalyst(75.0);
        assert_eq!(active.catalyst_accumulated, 125.0);
    }

    #[test]
    fn test_active_quest_overall_progress() {
        let mut active = ActiveQuest::new(QuestId::new(1), 0.0);
        active.current_stage = 2;

        let progress = active.overall_progress(4);
        assert!(progress > 0.2 && progress < 0.3); // ~25%
    }

    // ============================================================================
    // ARCHETYPICAL PATH TESTS
    // ============================================================================

    #[test]
    fn test_archetypical_path_creation() {
        let path =
            ArchetypicalPath::new(ArchetypeId::A1_MAGICIAN, "Path of the Magician".to_string())
                .with_description("The path of transformation".to_string())
                .with_polarity(Polarity::Balanced)
                .with_density_range(Density::THIRD, Density::FIFTH);

        assert_eq!(path.archetype_id, ArchetypeId::A1_MAGICIAN);
        assert_eq!(path.path_name, "Path of the Magician");
        assert_eq!(path.dominant_polarity, Polarity::Balanced);
        assert_eq!(path.density_range.0, Density::THIRD);
        assert_eq!(path.density_range.1, Density::FIFTH);
    }

    #[test]
    fn test_archetypical_path_density_check() {
        let path = ArchetypicalPath::new(ArchetypeId::A1_MAGICIAN, "Test".to_string())
            .with_density_range(Density::THIRD, Density::FIFTH);

        assert!(path.is_density_in_range(Density::THIRD));
        assert!(path.is_density_in_range(Density::FOURTH));
        assert!(path.is_density_in_range(Density::FIFTH));
        assert!(!path.is_density_in_range(Density::SECOND));
        assert!(!path.is_density_in_range(Density::SIXTH));
    }

    #[test]
    fn test_generate_default_path() {
        let path = ArchetypicalPath::generate_default_path(ArchetypeId::A1_MAGICIAN);

        assert_eq!(path.archetype_id, ArchetypeId::A1_MAGICIAN);
        assert!(path.path_name.contains("Magician"));
        assert!(path.resonance_pattern.stability > 0.0);
    }

    // ============================================================================
    // COMPLETION CRITERIA TESTS
    // ============================================================================

    #[test]
    fn test_completion_criteria_all_objectives() {
        let criteria = CompletionCriteria::AllObjectives;
        let objectives = vec![
            QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 }),
            QuestObjective::new(2, ObjectiveType::GenerateCatalyst { amount: 50.0 }),
        ];

        // Not all complete
        assert!(!criteria.is_complete(&objectives, 0.0));

        // Complete all
        let mut complete_objectives = objectives.clone();
        complete_objectives[0].is_completed = true;
        complete_objectives[1].is_completed = true;
        assert!(criteria.is_complete(&complete_objectives, 0.0));
    }

    #[test]
    fn test_completion_criteria_any_objective() {
        let criteria = CompletionCriteria::AnyObjective;
        let objectives = vec![
            QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 }),
            QuestObjective::new(2, ObjectiveType::GenerateCatalyst { amount: 50.0 }),
        ];

        // None complete
        assert!(!criteria.is_complete(&objectives, 0.0));

        // One complete
        let mut partial_objectives = objectives.clone();
        partial_objectives[0].is_completed = true;
        assert!(criteria.is_complete(&partial_objectives, 0.0));
    }

    #[test]
    fn test_completion_criteria_catalyst() {
        let criteria = CompletionCriteria::CatalystAccumulation { amount: 100.0 };
        let objectives = vec![];

        assert!(!criteria.is_complete(&objectives, 50.0));
        assert!(criteria.is_complete(&objectives, 100.0));
        assert!(criteria.is_complete(&objectives, 150.0));
    }

    // ============================================================================
    // ARCHETYPICAL QUESTS SYSTEM TESTS
    // ============================================================================

    #[test]
    fn test_archetypical_quests_creation() {
        let system = ArchetypicalQuests::new();

        assert_eq!(system.archetype_paths.len(), 22);
        assert_eq!(system.available_quests.len(), 0);
    }

    #[test]
    fn test_create_quest_in_system() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
            "A test quest".to_string(),
            vec![],
        );

        assert_eq!(system.available_quests.len(), 1);
        assert!(system.available_quests.contains_key(&quest.quest_id));
    }

    #[test]
    fn test_start_quest_in_system() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
            "A test quest".to_string(),
            vec![],
        );

        let entity_id = EntityId::new("test-entity-6".to_string());
        let result = system.start_quest(entity_id.clone(), quest.quest_id, Density::THIRD, None);

        assert!(result.is_ok());
        assert_eq!(system.get_active_quests(entity_id).len(), 1);
    }

    #[test]
    fn test_start_quest_insufficient_density() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
            "A test quest".to_string(),
            vec![],
        );

        // Set required density
        let mut quest_mut = quest.clone();
        quest_mut.required_density = Density::FOURTH;
        system.available_quests.insert(quest.quest_id, quest_mut);

        let entity_id = EntityId::new("test-entity-7".to_string());
        let result = system.start_quest(entity_id, quest.quest_id, Density::THIRD, None);

        assert!(matches!(
            result,
            Err(QuestError::InsufficientDensity { .. })
        ));
    }

    #[test]
    fn test_get_available_quests() {
        let mut system = ArchetypicalQuests::new();

        system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Quest 1".to_string(),
            "First quest".to_string(),
            vec![],
        );

        system.create_quest(
            ArchetypeId::A2_HIGH_PRIESTESS,
            "Quest 2".to_string(),
            "Second quest".to_string(),
            vec![],
        );

        let entity_id = EntityId::new("test-entity-8".to_string());
        let available = system.get_available_quests(entity_id, Density::THIRD, None);

        assert_eq!(available.len(), 2);
    }

    #[test]
    fn test_generate_quest_from_archetype() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.generate_quest_from_archetype(ArchetypeId::A1_MAGICIAN);

        assert_eq!(quest.archetype_path, ArchetypeId::A1_MAGICIAN);
        assert!(!quest.stages.is_empty());
        assert!(quest.name.contains("Magician"));
    }

    #[test]
    fn test_update_quest_progress() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
            "A test quest".to_string(),
            vec![],
        );

        let entity_id = EntityId::new("test-entity-9".to_string());
        let _ = system.start_quest(entity_id.clone(), quest.quest_id, Density::THIRD, None);

        let result = system.update_quest_progress(
            entity_id.clone(),
            quest.quest_id,
            50.0,
            &ResonancePattern::new(),
        );

        assert!(result.is_ok());

        let progress = system.get_entity_progress(entity_id).unwrap();
        let active = progress.get_active_quest(quest.quest_id).unwrap();
        assert_eq!(active.catalyst_accumulated, 50.0);
    }

    #[test]
    fn test_archetype_progress_tracking() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Test Quest".to_string(),
            "A test quest".to_string(),
            vec![],
        );

        let entity_id = EntityId::new("test-entity-10".to_string());
        let _ = system.start_quest(entity_id.clone(), quest.quest_id, Density::THIRD, None);

        // Initially no progress
        let progress = system.get_archetype_progress(entity_id.clone(), ArchetypeId::A1_MAGICIAN);
        assert_eq!(progress, 0.0);

        // Complete quest to gain progress
        let _ = system.complete_quest(entity_id.clone(), quest.quest_id);

        let progress = system.get_archetype_progress(entity_id, ArchetypeId::A1_MAGICIAN);
        assert!(progress > 0.0);
    }

    #[test]
    fn test_quest_with_stages() {
        let mut system = ArchetypicalQuests::new();

        let stage1 =
            QuestStage::new(1, "Stage 1".to_string(), Density::THIRD).with_catalyst_threshold(50.0);

        let stage2 = QuestStage::new(2, "Stage 2".to_string(), Density::FOURTH)
            .with_catalyst_threshold(100.0);

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Multi-Stage Quest".to_string(),
            "A quest with multiple stages".to_string(),
            vec![stage1, stage2],
        );

        assert_eq!(quest.stage_count(), 2);
        assert!(quest.get_stage(1).is_some());
        assert!(quest.get_stage(2).is_some());
    }

    #[test]
    fn test_objective_type_display_name() {
        let obj_type = ObjectiveType::GenerateCatalyst { amount: 100.0 };
        assert_eq!(obj_type.display_name(), "Generate 100 Catalyst");

        let obj_type = ObjectiveType::CompleteCombat { combat_count: 5 };
        assert_eq!(obj_type.display_name(), "Complete 5 Combats");

        let obj_type = ObjectiveType::ReachDensity {
            target_density: Density::FOURTH,
        };
        assert_eq!(obj_type.display_name(), "Reach Fourth Density");
    }

    #[test]
    fn test_quest_resonance_generation() {
        let quest = Quest::new(
            QuestId::new(1),
            ArchetypeId::A1_MAGICIAN,
            "Test".to_string(),
        );
        let resonance = quest.generate_archetype_resonance();

        assert!(resonance.stability > 0.0);
        assert!(resonance.stability <= 1.0);
    }

    #[test]
    fn test_repeatable_quest() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "Repeatable Quest".to_string(),
            "Can be done multiple times".to_string(),
            vec![],
        );

        // Make it repeatable
        let mut quest_mut = quest.clone();
        quest_mut.is_repeatable = true;
        system.available_quests.insert(quest.quest_id, quest_mut);

        let entity_id = EntityId::new("test-entity-11".to_string());

        // Complete once
        let _ = system.start_quest(entity_id.clone(), quest.quest_id, Density::THIRD, None);
        let _ = system.complete_quest(entity_id.clone(), quest.quest_id);

        // Should be able to start again
        let result = system.start_quest(entity_id, quest.quest_id, Density::THIRD, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_repeatable_quest() {
        let mut system = ArchetypicalQuests::new();

        let quest = system.create_quest(
            ArchetypeId::A1_MAGICIAN,
            "One-Time Quest".to_string(),
            "Can only be done once".to_string(),
            vec![],
        );

        let entity_id = EntityId::new("test-entity-12".to_string());

        // Complete once
        let _ = system.start_quest(entity_id.clone(), quest.quest_id, Density::THIRD, None);
        let _ = system.complete_quest(entity_id.clone(), quest.quest_id);

        // Should NOT be able to start again
        let result = system.start_quest(entity_id, quest.quest_id, Density::THIRD, None);
        assert!(matches!(result, Err(QuestError::QuestAlreadyCompleted)));
    }

    #[test]
    fn test_time_advancement() {
        let mut system = ArchetypicalQuests::new();

        assert_eq!(system.current_time(), 0.0);

        system.advance_time(10.0);
        assert_eq!(system.current_time(), 10.0);

        system.advance_time(5.0);
        assert_eq!(system.current_time(), 15.0);
    }

    #[test]
    fn test_quest_with_objectives() {
        let objective1 = QuestObjective::new(1, ObjectiveType::GenerateCatalyst { amount: 100.0 });
        let objective2 = QuestObjective::new(2, ObjectiveType::CompleteCombat { combat_count: 3 });

        let stage = QuestStage::new(1, "Test Stage".to_string(), Density::THIRD)
            .with_objective(objective1)
            .with_objective(objective2);

        assert_eq!(stage.objectives.len(), 2);
        assert_eq!(stage.stage_progress(), 0.0);
    }

    #[test]
    fn test_all_archetype_paths_initialized() {
        let system = ArchetypicalQuests::new();

        // Check all 22 archetypes have paths
        for i in 1..=22 {
            let archetype_id = ArchetypeId::new(i);
            assert!(
                system.get_archetype_path(archetype_id).is_some(),
                "Archetype {} should have a path",
                i
            );
        }
    }
}
