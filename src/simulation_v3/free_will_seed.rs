//! Free Will as Seed - Non-Deterministic Choice Operator
//!
//! This module implements Free Will as a compact seed that enables
//! deterministic reconstruction of non-deterministic choices.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Free will is NOT randomness (quantum randomness is as far from freedom as determinism).
//! Free will = controlled selection from possibility space. Choice is non-deterministic
//! selection (not random, not predetermined)."
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Free Will as Seed: 8 bytes vs 100+ bytes. Traditional behavior selection: 1 ms (tree search).
//! Holographic Free Will seed: 1 μs (deterministic reconstruction)."

use crate::types::Float;

/// Free Will seed - compact representation of choice (8 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreeWillSeed(u64);

impl FreeWillSeed {
    pub const ZERO: FreeWillSeed = FreeWillSeed(0);
    pub const MAX: FreeWillSeed = FreeWillSeed(u64::MAX);

    pub fn new(seed: u64) -> Self {
        FreeWillSeed(seed)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn random() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        FreeWillSeed(duration.as_nanos() as u64)
    }
}

/// Choice outcome - the result of Free Will selection
#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub selected_index: usize,
    pub confidence: Float,
    pub seed_used: FreeWillSeed,
}

/// Possibility space - all available choices for an entity
#[derive(Debug, Clone, PartialEq)]
pub struct PossibilitySpace {
    pub choices: Vec<Possibility>,
    pub weights: Vec<Float>,
    pub archetype_constraints: Vec<ArchetypeConstraint>,
}

impl PossibilitySpace {
    pub fn new() -> Self {
        PossibilitySpace {
            choices: Vec::new(),
            weights: Vec::new(),
            archetype_constraints: Vec::new(),
        }
    }

    pub fn add_choice(&mut self, choice: Possibility, weight: Float) {
        self.choices.push(choice);
        self.weights.push(weight);
    }

    pub fn add_constraint(&mut self, constraint: ArchetypeConstraint) {
        self.archetype_constraints.push(constraint);
    }

    pub fn choice_count(&self) -> usize {
        self.choices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.choices.is_empty()
    }

    pub fn total_weight(&self) -> Float {
        self.weights.iter().sum()
    }

    pub fn apply_archetype_filter(&self, archetype_profile: &[Float]) -> Vec<usize> {
        self.choices
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                self.archetype_constraints
                    .iter()
                    .all(|c| c.allows(archetype_profile, self.choices[*i].archetype_signature()))
            })
            .map(|(i, _)| i)
            .collect()
    }
}

impl Default for PossibilitySpace {
    fn default() -> Self {
        Self::new()
    }
}

/// Single possibility within possibility space
#[derive(Debug, Clone, PartialEq)]
pub struct Possibility {
    pub id: String,
    pub action_type: ActionType,
    pub archetype_signature: [Float; 22],
    pub catalyst_value: Float,
}

impl Possibility {
    pub fn archetype_signature(&self) -> &[Float; 22] {
        &self.archetype_signature
    }
}

/// Type of action that can be chosen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    Move,
    Interact,
    Observe,
    Create,
    Transform,
    Communicate,
    Rest,
    Evolve,
}

/// Archetype constraint on choices
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypeConstraint {
    pub archetype_index: usize,
    pub min_value: Float,
    pub max_value: Float,
    pub required: bool,
}

impl ArchetypeConstraint {
    pub fn new(archetype_index: usize, min_value: Float, max_value: Float) -> Self {
        ArchetypeConstraint {
            archetype_index,
            min_value,
            max_value,
            required: false,
        }
    }

    pub fn required(archetype_index: usize, min_value: Float, max_value: Float) -> Self {
        ArchetypeConstraint {
            archetype_index,
            min_value,
            max_value,
            required: true,
        }
    }

    pub fn allows(&self, profile: &[Float], signature: &[Float; 22]) -> bool {
        let profile_value = profile.get(self.archetype_index).unwrap_or(&0.0);
        let signature_value = signature.get(self.archetype_index).unwrap_or(&0.0);

        // From COSMOLOGICAL-ARCHITECTURE.md: "Free will operates within archetype constraints"
        // The signature must always be in the valid range
        let in_range = *signature_value >= self.min_value && *signature_value <= self.max_value;
        let profile_match = *profile_value >= self.min_value && *profile_value <= self.max_value;

        // If required, both signature and profile must be in range
        // If not required, only signature needs to be in range (profile is optional guidance)
        if self.required {
            in_range && profile_match
        } else {
            in_range
        }
    }
}

/// Free Will choice engine
#[derive(Debug, Clone)]
pub struct FreeWillChoiceEngine {
    seed: FreeWillSeed,
    choice_history: Vec<Choice>,
    statistics: FreeWillStatistics,
}

impl FreeWillChoiceEngine {
    pub fn new(seed: FreeWillSeed) -> Self {
        FreeWillChoiceEngine {
            seed,
            choice_history: Vec::new(),
            statistics: FreeWillStatistics::default(),
        }
    }

    pub fn with_random_seed() -> Self {
        Self::new(FreeWillSeed::random())
    }

    pub fn make_choice(
        &mut self,
        possibility_space: &PossibilitySpace,
        archetype_profile: &[Float],
    ) -> Result<Choice, FreeWillError> {
        if possibility_space.is_empty() {
            return Err(FreeWillError::EmptyPossibilitySpace);
        }

        let filtered_indices = possibility_space.apply_archetype_filter(archetype_profile);

        if filtered_indices.is_empty() {
            return Err(FreeWillError::NoValidChoices);
        }

        let (selected_index, confidence) =
            self.select_choice(possibility_space, &filtered_indices)?;

        let choice = Choice {
            selected_index,
            confidence,
            seed_used: self.seed,
        };

        self.choice_history.push(choice.clone());
        self.statistics.total_choices += 1;
        self.statistics.average_confidence = (self.statistics.average_confidence
            * (self.statistics.total_choices - 1) as Float
            + confidence)
            / self.statistics.total_choices as Float;

        Ok(choice)
    }

    fn select_choice(
        &self,
        possibility_space: &PossibilitySpace,
        valid_indices: &[usize],
    ) -> Result<(usize, Float), FreeWillError> {
        let seed_value = self.seed.value();
        let total_weight: Float = valid_indices
            .iter()
            .map(|i| possibility_space.weights[*i])
            .sum();

        let mut cumulative_weight = 0.0;
        let target_weight = (seed_value % 1_000_000) as Float / 1_000_000.0 * total_weight;

        for i in valid_indices {
            cumulative_weight += possibility_space.weights[*i];
            if cumulative_weight >= target_weight {
                let confidence = possibility_space.weights[*i] / total_weight;
                return Ok((*i, confidence));
            }
        }

        Ok((valid_indices[valid_indices.len() - 1], 1.0))
    }

    pub fn reconstruct_choice(
        &self,
        possibility_space: &PossibilitySpace,
        archetype_profile: &[Float],
        seed: FreeWillSeed,
    ) -> Result<Choice, FreeWillError> {
        let mut temp_engine = FreeWillChoiceEngine {
            seed,
            choice_history: Vec::new(),
            statistics: FreeWillStatistics::default(),
        };

        temp_engine.make_choice(possibility_space, archetype_profile)
    }

    pub fn advance_seed(&mut self) {
        self.seed = FreeWillSeed(self.seed.value().wrapping_add(1));
    }

    pub fn seed(&self) -> FreeWillSeed {
        self.seed
    }

    pub fn set_seed(&mut self, seed: FreeWillSeed) {
        self.seed = seed;
    }

    pub fn choice_history(&self) -> &[Choice] {
        &self.choice_history
    }

    pub fn statistics(&self) -> &FreeWillStatistics {
        &self.statistics
    }

    pub fn clear_history(&mut self) {
        self.choice_history.clear();
        self.statistics = FreeWillStatistics::default();
    }
}

/// Free Will statistics
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FreeWillStatistics {
    pub total_choices: usize,
    pub average_confidence: Float,
}

/// Free Will errors
#[derive(Debug, Clone, PartialEq)]
pub enum FreeWillError {
    EmptyPossibilitySpace,
    NoValidChoices,
    InvalidArchetypeProfile,
    InvalidPossibilityIndex,
}

impl std::fmt::Display for FreeWillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FreeWillError::EmptyPossibilitySpace => write!(f, "Possibility space is empty"),
            FreeWillError::NoValidChoices => {
                write!(f, "No valid choices after archetype filtering")
            }
            FreeWillError::InvalidArchetypeProfile => write!(f, "Invalid archetype profile"),
            FreeWillError::InvalidPossibilityIndex => write!(f, "Invalid possibility index"),
        }
    }
}

impl std::error::Error for FreeWillError {}

/// Deterministic choice reconstruction function
pub fn deterministic_choice_reconstruction(
    seed: FreeWillSeed,
    possibility_space: &PossibilitySpace,
    archetype_profile: &[Float],
) -> Result<Choice, FreeWillError> {
    let mut engine = FreeWillChoiceEngine::new(seed);
    engine.make_choice(possibility_space, archetype_profile)
}

/// Generate possibility space from archetype profile
pub fn generate_possibility_space(
    archetype_profile: &[Float],
    available_actions: &[ActionType],
    catalyst_intensity: Float,
) -> PossibilitySpace {
    let mut space = PossibilitySpace::new();

    for action in available_actions {
        let possibility =
            create_possibility_from_action(*action, archetype_profile, catalyst_intensity);
        let weight = calculate_action_weight(*action, archetype_profile);
        space.add_choice(possibility, weight);
    }

    apply_archetype_constraints(&mut space, archetype_profile);

    space
}

fn create_possibility_from_action(
    action: ActionType,
    archetype_profile: &[Float],
    catalyst_intensity: Float,
) -> Possibility {
    let archetype_signature = derive_action_archetype_signature(action, archetype_profile);

    Possibility {
        id: format!("{:?}_{}", action, catalyst_intensity),
        action_type: action,
        archetype_signature,
        catalyst_value: catalyst_intensity,
    }
}

fn calculate_action_weight(action: ActionType, archetype_profile: &[Float]) -> Float {
    match action {
        ActionType::Move => archetype_profile[0] * 0.3 + archetype_profile[1] * 0.7,
        ActionType::Interact => archetype_profile[2] * 0.4 + archetype_profile[3] * 0.6,
        ActionType::Observe => archetype_profile[4] * 0.5 + archetype_profile[5] * 0.5,
        ActionType::Create => archetype_profile[6] * 0.3 + archetype_profile[7] * 0.7,
        ActionType::Transform => archetype_profile[8] * 0.4 + archetype_profile[9] * 0.6,
        ActionType::Communicate => archetype_profile[10] * 0.3 + archetype_profile[11] * 0.7,
        ActionType::Rest => archetype_profile[12] * 0.6 + archetype_profile[13] * 0.4,
        ActionType::Evolve => archetype_profile[14] * 0.2 + archetype_profile[15] * 0.8,
    }
}

fn derive_action_archetype_signature(action: ActionType, profile: &[Float]) -> [Float; 22] {
    let mut signature = [0.0; 22];

    match action {
        ActionType::Move => {
            signature[0] = profile[0];
            signature[1] = profile[1];
            signature[2] = profile[2];
        }
        ActionType::Interact => {
            signature[3] = profile[3];
            signature[4] = profile[4];
            signature[5] = profile[5];
        }
        ActionType::Observe => {
            signature[6] = profile[6];
            signature[7] = profile[7];
            signature[8] = profile[8];
        }
        ActionType::Create => {
            signature[9] = profile[9];
            signature[10] = profile[10];
            signature[11] = profile[11];
        }
        ActionType::Transform => {
            signature[12] = profile[12];
            signature[13] = profile[13];
            signature[14] = profile[14];
        }
        ActionType::Communicate => {
            signature[15] = profile[15];
            signature[16] = profile[16];
            signature[17] = profile[17];
        }
        ActionType::Rest => {
            signature[18] = profile[18];
            signature[19] = profile[19];
            signature[20] = profile[20];
        }
        ActionType::Evolve => {
            signature[21] = profile[21];
        }
    }

    signature
}

fn apply_archetype_constraints(space: &mut PossibilitySpace, archetype_profile: &[Float]) {
    for i in 0..22 {
        if archetype_profile[i] < 0.1 {
            space.add_constraint(ArchetypeConstraint::new(i, 0.1, 1.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_archetype_profile() -> [Float; 22] {
        [
            0.5, 0.3, 0.7, 0.4, 0.6, 0.2, 0.8, 0.5, 0.3, 0.9, 0.4, 0.6, 0.2, 0.7, 0.5, 0.3, 0.8,
            0.4, 0.6, 0.2, 0.7, 0.5,
        ]
    }

    #[test]
    fn test_free_will_seed_creation() {
        let seed = FreeWillSeed::new(12345);
        assert_eq!(seed.value(), 12345);
        assert!(!seed.is_zero());

        let zero = FreeWillSeed::ZERO;
        assert!(zero.is_zero());
    }

    #[test]
    fn test_free_will_seed_random() {
        let seed1 = FreeWillSeed::random();
        let seed2 = FreeWillSeed::random();
        assert_ne!(seed1.value(), seed2.value());
    }

    #[test]
    fn test_possibility_space_creation() {
        let space = PossibilitySpace::new();
        assert!(space.is_empty());
        assert_eq!(space.choice_count(), 0);
    }

    #[test]
    fn test_add_choice_to_space() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        assert_eq!(space.choice_count(), 1);
        assert!(!space.is_empty());
    }

    #[test]
    fn test_archetype_constraint_creation() {
        let constraint = ArchetypeConstraint::new(0, 0.5, 1.0);
        assert_eq!(constraint.archetype_index, 0);
        assert_eq!(constraint.min_value, 0.5);
        assert_eq!(constraint.max_value, 1.0);
        assert!(!constraint.required);
    }

    #[test]
    fn test_archetype_constraint_required() {
        let constraint = ArchetypeConstraint::required(0, 0.5, 1.0);
        assert!(constraint.required);
    }

    #[test]
    fn test_archetype_constraint_allows() {
        let constraint = ArchetypeConstraint::new(0, 0.3, 0.7);
        let profile = [0.5; 22];
        let signature = [0.6; 22];

        assert!(constraint.allows(&profile, &signature));

        let signature_low = [0.2; 22];
        assert!(!constraint.allows(&profile, &signature_low));
    }

    #[test]
    fn test_free_will_choice_engine_creation() {
        let seed = FreeWillSeed::new(12345);
        let engine = FreeWillChoiceEngine::new(seed);

        assert_eq!(engine.seed().value(), 12345);
        assert_eq!(engine.choice_history().len(), 0);
    }

    #[test]
    fn test_free_will_choice_engine_random() {
        let engine = FreeWillChoiceEngine::with_random_seed();
        assert_ne!(engine.seed().value(), 0);
    }

    #[test]
    fn test_make_choice_basic() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = create_test_archetype_profile();

        let choice = engine.make_choice(&space, &profile).unwrap();
        assert_eq!(choice.selected_index, 0);
        assert_eq!(choice.seed_used.value(), 100);
    }

    #[test]
    fn test_make_choice_empty_space_error() {
        let space = PossibilitySpace::new();
        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = create_test_archetype_profile();

        let result = engine.make_choice(&space, &profile);
        assert_eq!(result, Err(FreeWillError::EmptyPossibilitySpace));
    }

    #[test]
    fn test_make_choice_no_valid_choices_error() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.0; 22],
            catalyst_value: 0.5,
        };
        space.add_constraint(ArchetypeConstraint::required(0, 0.5, 1.0));
        space.add_choice(possibility, 1.0);

        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = [0.0; 22];

        let result = engine.make_choice(&space, &profile);
        assert_eq!(result, Err(FreeWillError::NoValidChoices));
    }

    #[test]
    fn test_advance_seed() {
        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        engine.advance_seed();
        assert_eq!(engine.seed().value(), 101);
    }

    #[test]
    fn test_set_seed() {
        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        engine.set_seed(FreeWillSeed::new(200));
        assert_eq!(engine.seed().value(), 200);
    }

    #[test]
    fn test_choice_history_tracking() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = create_test_archetype_profile();

        engine.make_choice(&space, &profile).unwrap();
        engine.make_choice(&space, &profile).unwrap();

        assert_eq!(engine.choice_history().len(), 2);
        assert_eq!(engine.statistics().total_choices, 2);
    }

    #[test]
    fn test_clear_history() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = create_test_archetype_profile();

        engine.make_choice(&space, &profile).unwrap();
        engine.clear_history();

        assert_eq!(engine.choice_history().len(), 0);
        assert_eq!(engine.statistics().total_choices, 0);
    }

    #[test]
    fn test_deterministic_choice_reconstruction() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let profile = create_test_archetype_profile();
        let seed = FreeWillSeed::new(500);

        let choice1 = deterministic_choice_reconstruction(seed, &space, &profile).unwrap();
        let choice2 = deterministic_choice_reconstruction(seed, &space, &profile).unwrap();

        assert_eq!(choice1.selected_index, choice2.selected_index);
        assert_eq!(choice1.seed_used.value(), choice2.seed_used.value());
    }

    #[test]
    fn test_generate_possibility_space() {
        let profile = create_test_archetype_profile();
        let actions = vec![ActionType::Move, ActionType::Interact, ActionType::Observe];

        let space = generate_possibility_space(&profile, &actions, 0.5);

        assert_eq!(space.choice_count(), 3);
        assert!(!space.is_empty());
    }

    #[test]
    fn test_total_weight_calculation() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility.clone(), 0.5);
        space.add_choice(possibility, 0.3);

        assert!((space.total_weight() - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_apply_archetype_filter() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.6; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);
        space.add_constraint(ArchetypeConstraint::required(0, 0.5, 1.0));

        let profile = [0.7; 22];
        let filtered = space.apply_archetype_filter(&profile);

        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_free_will_statistics_tracking() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let mut engine = FreeWillChoiceEngine::new(FreeWillSeed::new(100));
        let profile = create_test_archetype_profile();

        engine.make_choice(&space, &profile).unwrap();
        engine.make_choice(&space, &profile).unwrap();
        engine.make_choice(&space, &profile).unwrap();

        assert_eq!(engine.statistics().total_choices, 3);
        assert!(engine.statistics().average_confidence > 0.0);
    }

    #[test]
    fn test_action_type_variations() {
        let space = PossibilitySpace::new();
        let actions = [
            ActionType::Move,
            ActionType::Interact,
            ActionType::Observe,
            ActionType::Create,
            ActionType::Transform,
            ActionType::Communicate,
            ActionType::Rest,
            ActionType::Evolve,
        ];

        let profile = create_test_archetype_profile();

        for action in &actions {
            let possibility = create_possibility_from_action(*action, &profile, 0.5);
            assert_eq!(possibility.action_type, *action);
        }
    }

    #[test]
    fn test_reconstruct_choice_different_seeds() {
        let mut space = PossibilitySpace::new();
        let possibility = Possibility {
            id: "test_action".to_string(),
            action_type: ActionType::Move,
            archetype_signature: [0.5; 22],
            catalyst_value: 0.5,
        };
        space.add_choice(possibility, 1.0);

        let profile = create_test_archetype_profile();

        let choice1 =
            deterministic_choice_reconstruction(FreeWillSeed::new(100), &space, &profile).unwrap();
        let choice2 =
            deterministic_choice_reconstruction(FreeWillSeed::new(200), &space, &profile).unwrap();

        assert_eq!(choice1.selected_index, choice2.selected_index);
    }

    #[test]
    fn test_multiple_choices_consistency() {
        let mut space = PossibilitySpace::new();

        for i in 0..10 {
            let possibility = Possibility {
                id: format!("action_{}", i),
                action_type: ActionType::Move,
                archetype_signature: [0.5; 22],
                catalyst_value: 0.5,
            };
            space.add_choice(possibility, 1.0);
        }

        let profile = create_test_archetype_profile();

        let mut engine1 = FreeWillChoiceEngine::new(FreeWillSeed::new(500));
        let mut engine2 = FreeWillChoiceEngine::new(FreeWillSeed::new(500));

        let choice1 = engine1.make_choice(&space, &profile).unwrap();
        let choice2 = engine2.make_choice(&space, &profile).unwrap();

        assert_eq!(choice1.selected_index, choice2.selected_index);
        assert_eq!(choice1.confidence, choice2.confidence);
    }

    #[test]
    fn test_free_will_seed_memory_size() {
        let seed = FreeWillSeed::new(12345);
        assert_eq!(std::mem::size_of_val(&seed), 8);
    }

    #[test]
    fn test_choice_memory_size() {
        let choice = Choice {
            selected_index: 5,
            confidence: 0.75,
            seed_used: FreeWillSeed::new(12345),
        };
        let size = std::mem::size_of_val(&choice);
        assert!(size < 100);
    }

    #[test]
    fn test_possibility_space_memory_size() {
        let space = PossibilitySpace::new();
        let size = std::mem::size_of_val(&space);
        assert!(size < 200);
    }

    #[test]
    fn test_free_will_engine_memory_size() {
        let engine = FreeWillChoiceEngine::new(FreeWillSeed::new(12345));
        let size = std::mem::size_of_val(&engine);
        assert!(size < 200);
    }
}
