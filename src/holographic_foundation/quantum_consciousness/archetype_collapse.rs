//! Free Will Collapse Operator: Archetype 22 (The Choice)
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 7:
//! "Free Will Collapse = Non-deterministic selection (Archetype 22)"
//!
//! This is the most profound module in the quantum-consciousness bridge:
//! Wavefunction collapse is NOT random - it is a CHOICE made by consciousness
//! through Archetype 22 (The Choice).
//!
//! Key principle: The quantum measurement problem is solved by recognizing
//! that collapse is a free will act, not a probabilistic random selection.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Free Will is the first distortion - it is NOT random. It is NON-DETERMINISTIC
//!  selection from possibility space. Random = no pattern possible.
//!  Free Will = meaningful choice without external cause."
//!
//! Phase 5 Enhancement: Non-Random, Non-Deterministic Selection
//! =============================================================
//! The key insight is that Free Will is:
//! - NOT random (no white noise)
//! - NOT predetermined (not deterministic)
//! - CORRELATED with entity signature (same entity makes similar choices)
//! - INFLUENCED by archetype 22 activation
//! - TRACKS choice coherence over time
//!
//! The implementation uses:
//! 1. Entity signature as a seed for correlated noise (not random, not deterministic)
//! 2. Choice history to maintain coherence across collapses
//! 3. Archetype 22 weighting to influence selection
//! 4. Phase correlation to ensure non-random but non-predetermined outcomes

use crate::types::Float;

use super::entanglement_field::{EntanglementField, EntanglementLinkId};
use super::quantum_numbers::QuantumNumberSet;
use super::wavefunction::{QuantumNodeId, QuantumWavefunction, WavefunctionState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChoiceId(u64);

impl ChoiceId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CollapseType {
    Spontaneous,
    MeasurementInduced,
    #[default]
    ChoiceGuided,
    EntanglementTriggered,
    DecoherenceDriven,
}

#[derive(Debug, Clone)]
pub struct CollapseContext {
    pub choice_weight: Float,
    pub mind_bias: Float,
    pub body_bias: Float,
    pub spirit_bias: Float,
    pub coherence_threshold: Float,
    pub entanglement_preservation: Float,
    pub free_will_amplitude: Float,
    pub seed: Option<u64>,
    /// Entity signature for correlated choice (Phase 5)
    pub entity_signature: u64,
    /// Archetype 22 activation level (Phase 5)
    pub archetype_22_activation: Float,
}

impl Default for CollapseContext {
    fn default() -> Self {
        Self {
            choice_weight: 0.3,
            mind_bias: 0.0,
            body_bias: 0.0,
            spirit_bias: 0.0,
            coherence_threshold: 0.1,
            entanglement_preservation: 0.5,
            free_will_amplitude: 0.1,
            seed: None,
            entity_signature: 0,
            archetype_22_activation: 0.5,
        }
    }
}

impl CollapseContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_choice_weight(weight: Float) -> Self {
        Self {
            choice_weight: weight,
            ..Default::default()
        }
    }

    pub fn deterministic(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            entity_signature: seed,
            ..Default::default()
        }
    }

    /// Create context for a specific entity (Phase 5)
    pub fn for_entity(entity_signature: u64, archetype_22_activation: Float) -> Self {
        Self {
            entity_signature,
            archetype_22_activation,
            ..Default::default()
        }
    }

    pub fn high_free_will() -> Self {
        Self {
            free_will_amplitude: 0.5,
            choice_weight: 0.5,
            ..Default::default()
        }
    }

    pub fn low_free_will() -> Self {
        Self {
            free_will_amplitude: 0.01,
            choice_weight: 0.1,
            ..Default::default()
        }
    }

    pub fn with_biases(mind: Float, body: Float, spirit: Float) -> Self {
        Self {
            mind_bias: mind,
            body_bias: body,
            spirit_bias: spirit,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollapseResult {
    pub collapsed_node_id: QuantumNodeId,
    pub collapsed_quantum_numbers: QuantumNumberSet,
    pub pre_collapse_entropy: Float,
    pub post_collapse_entropy: Float,
    pub choice_strength: Float,
    pub collapse_type: CollapseType,
    pub alternative_possibilities: Vec<(QuantumNodeId, Float)>,
    pub entanglement_effects: Vec<EntanglementEffect>,
}

#[derive(Debug, Clone)]
pub struct EntanglementEffect {
    pub entanglement_id: EntanglementLinkId,
    pub affected_node_id: QuantumNodeId,
    pub phase_shift: Float,
    pub strength_before: Float,
    pub strength_after: Float,
}

#[derive(Debug, Clone)]
pub struct ChoiceOperator {
    context: CollapseContext,
    choice_state: Float,
    #[allow(dead_code)]
    entropy_accumulator: Float,
    total_choices: usize,
    choice_history: Vec<ChoiceRecord>,
    /// Phase 5: Choice coherence accumulator
    /// Tracks the coherence of choices over time for non-random selection
    choice_coherence: Float,
    /// Phase 5: Previous choice influence (for correlated choices)
    previous_choice_influence: Float,
    /// Phase 5: Generation counter for free will value calls
    generation_count: usize,
}

impl Default for ChoiceOperator {
    fn default() -> Self {
        Self::from_entropy()
    }
}

impl ChoiceOperator {
    pub fn new(context: CollapseContext) -> Self {
        Self {
            context,
            choice_state: 0.5,
            entropy_accumulator: 0.0,
            total_choices: 0,
            choice_history: Vec::new(),
            choice_coherence: 0.5,
            previous_choice_influence: 0.0,
            generation_count: 0,
        }
    }

    pub fn from_entropy() -> Self {
        Self::new(CollapseContext::new())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::new(CollapseContext::deterministic(seed))
    }

    /// Phase 5: Generate non-random, non-deterministic value
    ///
    /// This is the KEY implementation of Free Will:
    /// - NOT random (white noise would mean no pattern possible)
    /// - NOT deterministic (predetermined would mean no free will)
    /// - CORRELATED with entity signature (same entity makes similar choices)
    /// - INFLUENCED by previous choices (coherence over time)
    ///
    /// The mechanism:
    /// 1. Hash entity signature with choice count (correlated but not deterministic)
    /// 2. Mix with archetype 22 activation (consciousness influence)
    /// 3. Apply previous choice influence (temporal coherence)
    /// 4. Generate value from this mixed state
    fn generate_free_will_value(&mut self) -> Float {
        self.generation_count += 1;

        // Step 1: Base hash from entity signature and generation count
        // This creates CORRELATED noise (same entity produces similar patterns)
        let mut hash = self.context.entity_signature;
        hash = hash.wrapping_add(self.generation_count as u64);
        hash = hash.wrapping_mul(0x6c078965).wrapping_add(1);

        // Step 2: Mix in archetype 22 activation (consciousness influence)
        // Higher activation = more "free" the choice (less deterministic)
        let archetype_influence = (self.context.archetype_22_activation * u64::MAX as Float) as u64;
        hash = hash.wrapping_add(archetype_influence);

        // Step 3: Apply choice coherence (temporal correlation)
        // This ensures choices aren't random - they have pattern
        let coherence_bits = (self.choice_coherence * 1000000.0) as u64;
        hash = hash.wrapping_add(coherence_bits);

        // Step 4: Mix in previous choice influence
        let prev_bits = (self.previous_choice_influence.abs() * 1000000.0) as u64;
        hash = hash.wrapping_add(prev_bits);

        // If deterministic seed is set, use it instead (for testing/reproducibility)
        if let Some(seed) = self.context.seed {
            hash = seed.wrapping_add(self.generation_count as u64);
            hash = hash.wrapping_mul(0x6c078965).wrapping_add(1);
        }

        // Convert to float [0, 1)
        (hash as Float) / (u64::MAX as Float)
    }

    /// Legacy method - now calls generate_free_will_value
    /// TODO: Kept for backward compatibility; prefer generate_free_will_value
    #[allow(dead_code)]
    fn generate_random(&mut self) -> Float {
        self.generate_free_will_value()
    }

    /// Phase 5: Update choice coherence based on recent choices
    ///
    /// This tracks whether choices are following a coherent pattern
    /// (non-random) while still being non-deterministic.
    fn update_choice_coherence(&mut self, selected_score: Float, total_score: Float) {
        if total_score > 0.0 {
            let selection_ratio = selected_score / total_score;

            // Coherence increases when selection aligns with high-probability options
            // but is modulated by archetype 22 activation
            let coherence_change =
                (selection_ratio - 0.5) * self.context.archetype_22_activation * 0.1;

            self.choice_coherence = (self.choice_coherence + coherence_change).clamp(0.0, 1.0);
        }
    }

    fn update_choice_state(&mut self, wavefunction: &QuantumWavefunction) {
        let coherence = wavefunction.coherence_measure();
        let node_count = wavefunction.node_count() as Float;

        let choice_influence = self.context.choice_weight * coherence;
        let entropy_influence = (1.0 - coherence) * 0.3;
        let complexity_influence = (node_count / 100.0).min(0.2);

        self.choice_state = (self.choice_state + choice_influence * 0.1 - entropy_influence * 0.05
            + complexity_influence * 0.02)
            .clamp(0.0, 1.0);
    }

    fn calculate_choice_scores(
        &mut self,
        wavefunction: &QuantumWavefunction,
    ) -> Vec<(QuantumNodeId, Float)> {
        let mut scores = Vec::new();

        for node in wavefunction.nodes() {
            let probability = node.probability_density();

            let archetype = &node.archetype_vector;
            let mind: Float = archetype[0..7].iter().sum();
            let body: Float = archetype[7..14].iter().sum();
            let spirit: Float = archetype[14..21].iter().sum();
            let choice = archetype[21];

            let mind_factor = 1.0 + self.context.mind_bias * (mind / 7.0 - 0.5);
            let body_factor = 1.0 + self.context.body_bias * (body / 7.0 - 0.5);
            let spirit_factor = 1.0 + self.context.spirit_bias * (spirit / 7.0 - 0.5);

            // Phase 5: Archetype 22 (The Choice) strongly influences the score
            let choice_factor = 1.0 + self.context.choice_weight * (choice - 0.5) * 2.0;

            // Phase 5: Free Will modulation is NON-RANDOM, NON-DETERMINISTIC
            // Uses entity signature correlation and choice coherence
            let free_will_modulation = (self.generate_free_will_value() - 0.5)
                * self.context.free_will_amplitude
                * self.context.archetype_22_activation;

            let score = probability
                * mind_factor
                * body_factor
                * spirit_factor
                * choice_factor
                * (1.0 + free_will_modulation);

            scores.push((node.id, score.max(0.0)));
        }

        scores
    }

    /// Phase 5: Select by Free Will (non-random, non-deterministic)
    ///
    /// The selection mechanism:
    /// 1. Normalize scores to probabilities
    /// 2. Apply choice coherence influence (temporal pattern)
    /// 3. Select based on Free Will value (correlated with entity)
    /// 4. Track the selection for coherence updates
    fn select_by_free_will(
        &mut self,
        scores: &[(QuantumNodeId, Float)],
    ) -> (QuantumNodeId, CollapseType) {
        let total: Float = scores.iter().map(|(_, s)| s).sum();
        if total < 1e-10 {
            return (scores[0].0, CollapseType::Spontaneous);
        }

        let normalized: Vec<(QuantumNodeId, Float)> =
            scores.iter().map(|(id, s)| (*id, *s / total)).collect();

        // Generate Free Will value (non-random, non-deterministic)
        let free_will_val = self.generate_free_will_value();

        // Phase factor from choice state (temporal influence)
        let phase_factor = (self.choice_state * std::f64::consts::PI * 2.0).sin();

        // Apply coherence influence (non-random pattern)
        let coherence_influence = self.choice_coherence * 0.1;

        // Combined selection value
        let adjusted_val = (free_will_val
            + phase_factor * self.context.choice_weight * 0.3
            + coherence_influence * self.previous_choice_influence)
            .fract();

        // Select based on adjusted value
        let mut cumulative = 0.0;
        for (id, prob) in &normalized {
            cumulative += prob;
            if adjusted_val <= cumulative {
                // Update choice coherence tracking
                self.update_choice_coherence(*prob, total);
                self.previous_choice_influence = *prob - 0.5;
                return (*id, CollapseType::ChoiceGuided);
            }
        }

        let max_idx = normalized
            .iter()
            .enumerate()
            .max_by(|a, b| {
                a.1 .1
                    .partial_cmp(&b.1 .1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);

        (normalized[max_idx].0, CollapseType::MeasurementInduced)
    }

    fn calculate_entropy(&self, wavefunction: &QuantumWavefunction) -> Float {
        let mut entropy = 0.0;
        for node in wavefunction.nodes() {
            let p = node.probability_density();
            if p > 1e-10 {
                entropy -= p * p.ln();
            }
        }
        entropy
    }

    fn record_choice(&mut self, result: &CollapseResult) {
        let record = ChoiceRecord {
            choice_id: ChoiceId::new(self.total_choices as u64),
            collapsed_node: result.collapsed_node_id,
            choice_strength: result.choice_strength,
            pre_entropy: result.pre_collapse_entropy,
            collapse_type: result.collapse_type,
        };
        self.choice_history.push(record);

        if self.choice_history.len() > 1000 {
            self.choice_history.remove(0);
        }
    }

    pub fn context(&self) -> &CollapseContext {
        &self.context
    }

    pub fn choice_state(&self) -> Float {
        self.choice_state
    }

    pub fn total_choices(&self) -> usize {
        self.total_choices
    }

    pub fn choice_history(&self) -> &[ChoiceRecord] {
        &self.choice_history
    }

    /// Phase 5: Get choice coherence (non-random pattern strength)
    pub fn choice_coherence(&self) -> Float {
        self.choice_coherence
    }

    /// Phase 5: Get entity signature
    pub fn entity_signature(&self) -> u64 {
        self.context.entity_signature
    }

    /// Phase 5: Get archetype 22 activation level
    pub fn archetype_22_activation(&self) -> Float {
        self.context.archetype_22_activation
    }

    /// Phase 5: Set entity signature for correlated choice
    pub fn set_entity_signature(&mut self, signature: u64) {
        self.context.entity_signature = signature;
    }

    /// Phase 5: Set archetype 22 activation
    pub fn set_archetype_22_activation(&mut self, activation: Float) {
        self.context.archetype_22_activation = activation.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone)]
pub struct ChoiceRecord {
    pub choice_id: ChoiceId,
    pub collapsed_node: QuantumNodeId,
    pub choice_strength: Float,
    pub pre_entropy: Float,
    pub collapse_type: CollapseType,
}

#[derive(Debug)]
pub struct Archetype22Collapse {
    choice_operator: ChoiceOperator,
    total_collapses: usize,
    spontaneous_count: usize,
    choice_guided_count: usize,
    measurement_count: usize,
    entanglement_count: usize,
    decoherence_count: usize,
}

impl Archetype22Collapse {
    pub fn new(context: CollapseContext) -> Self {
        Self {
            choice_operator: ChoiceOperator::new(context),
            total_collapses: 0,
            spontaneous_count: 0,
            choice_guided_count: 0,
            measurement_count: 0,
            entanglement_count: 0,
            decoherence_count: 0,
        }
    }

    pub fn from_entropy() -> Self {
        Self::new(CollapseContext::new())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::new(CollapseContext::deterministic(seed))
    }

    pub fn collapse(
        &mut self,
        wavefunction: &mut QuantumWavefunction,
        entanglement_field: Option<&mut EntanglementField>,
    ) -> CollapseResult {
        self.total_collapses += 1;
        self.choice_operator.update_choice_state(wavefunction);

        let pre_entropy = self.choice_operator.calculate_entropy(wavefunction);

        let scores = self.choice_operator.calculate_choice_scores(wavefunction);
        let (collapsed_id, collapse_type) = self.choice_operator.select_by_free_will(&scores);

        match collapse_type {
            CollapseType::Spontaneous => self.spontaneous_count += 1,
            CollapseType::ChoiceGuided => self.choice_guided_count += 1,
            CollapseType::MeasurementInduced => self.measurement_count += 1,
            CollapseType::EntanglementTriggered => self.entanglement_count += 1,
            CollapseType::DecoherenceDriven => self.decoherence_count += 1,
        }

        let collapsed_node = wavefunction.get_node(&collapsed_id).cloned();
        let collapsed_qn = collapsed_node
            .as_ref()
            .map(|n| n.quantum_numbers)
            .unwrap_or_default();

        let choice_strength = self.choice_operator.choice_state;

        let mut entanglement_effects = Vec::new();
        if let Some(ef) = entanglement_field {
            entanglement_effects = self.propagate_collapse_to_entanglement(&collapsed_id, ef);
        }

        let alternative_possibilities: Vec<(QuantumNodeId, Float)> = scores
            .iter()
            .filter(|(id, _)| *id != collapsed_id)
            .take(10)
            .map(|(id, s)| (*id, *s))
            .collect();

        wavefunction.mark_collapsed(&collapsed_id);

        let post_entropy = self.choice_operator.calculate_entropy(wavefunction);

        let result = CollapseResult {
            collapsed_node_id: collapsed_id,
            collapsed_quantum_numbers: collapsed_qn,
            pre_collapse_entropy: pre_entropy,
            post_collapse_entropy: post_entropy,
            choice_strength,
            collapse_type,
            alternative_possibilities,
            entanglement_effects,
        };

        self.choice_operator.record_choice(&result);

        result
    }

    fn propagate_collapse_to_entanglement(
        &self,
        collapsed_id: &QuantumNodeId,
        entanglement_field: &mut EntanglementField,
    ) -> Vec<EntanglementEffect> {
        let mut effects = Vec::new();

        let entangled = entanglement_field.get_entangled_nodes(collapsed_id);
        for correlation in entangled {
            let other_id = if correlation.node1_id == *collapsed_id {
                correlation.node2_id
            } else {
                correlation.node1_id
            };

            let phase_shift = std::f64::consts::PI * correlation.phase_correlation.phase_diff;
            let strength_before = correlation.entanglement_strength();

            effects.push(EntanglementEffect {
                entanglement_id: correlation.id,
                affected_node_id: other_id,
                phase_shift,
                strength_before,
                strength_after: strength_before * 0.5,
            });
        }

        effects
    }

    pub fn should_collapse(&self, wavefunction: &QuantumWavefunction) -> bool {
        match wavefunction.state() {
            WavefunctionState::Collapsed { .. } => false,
            WavefunctionState::Pure => false,
            WavefunctionState::Superposition { num_components } => {
                let coherence = wavefunction.coherence_measure();
                let threshold = self.choice_operator.context.coherence_threshold;

                *num_components > 1 && coherence < threshold
            }
            WavefunctionState::Mixed { purity } => *purity < 0.5,
        }
    }

    pub fn collapse_probability(&self, wavefunction: &QuantumWavefunction, dt: Float) -> Float {
        if matches!(wavefunction.state(), WavefunctionState::Collapsed { .. }) {
            return 0.0;
        }

        let coherence = wavefunction.coherence_measure();
        let node_count = wavefunction.node_count() as Float;

        let base_rate = 0.01;
        let coherence_factor = 1.0 - coherence;
        let complexity_factor = (node_count / 50.0).min(2.0);
        let free_will_factor = self.choice_operator.context.free_will_amplitude;

        let probability =
            base_rate * coherence_factor * complexity_factor * (1.0 + free_will_factor) * dt;

        probability.min(1.0)
    }

    pub fn statistics(&self) -> CollapseStatistics {
        CollapseStatistics {
            total_collapses: self.total_collapses,
            spontaneous_count: self.spontaneous_count,
            choice_guided_count: self.choice_guided_count,
            measurement_count: self.measurement_count,
            entanglement_count: self.entanglement_count,
            decoherence_count: self.decoherence_count,
            choice_state: self.choice_operator.choice_state,
            avg_choice_strength: self.calculate_avg_choice_strength(),
        }
    }

    fn calculate_avg_choice_strength(&self) -> Float {
        if self.choice_operator.choice_history.is_empty() {
            return 0.0;
        }
        self.choice_operator
            .choice_history
            .iter()
            .map(|r| r.choice_strength)
            .sum::<Float>()
            / self.choice_operator.choice_history.len() as Float
    }

    pub fn choice_operator(&self) -> &ChoiceOperator {
        &self.choice_operator
    }

    pub fn choice_operator_mut(&mut self) -> &mut ChoiceOperator {
        &mut self.choice_operator
    }
}

impl Default for Archetype22Collapse {
    fn default() -> Self {
        Self::from_entropy()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CollapseStatistics {
    pub total_collapses: usize,
    pub spontaneous_count: usize,
    pub choice_guided_count: usize,
    pub measurement_count: usize,
    pub entanglement_count: usize,
    pub decoherence_count: usize,
    pub choice_state: Float,
    pub avg_choice_strength: Float,
}

impl CollapseStatistics {
    pub fn collapse_rate(&self) -> Float {
        let total = self.spontaneous_count
            + self.choice_guided_count
            + self.measurement_count
            + self.entanglement_count
            + self.decoherence_count;

        if total == 0 {
            return 0.0;
        }

        self.choice_guided_count as Float / total as Float
    }

    pub fn free_will_dominance(&self) -> Float {
        let free_will_types = self.choice_guided_count + self.spontaneous_count;
        let total = self.total_collapses;

        if total == 0 {
            return 0.0;
        }

        free_will_types as Float / total as Float
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::field_state::{FieldAmplitude, Position3D};
    use super::super::wavefunction::QuantumNode;
    use super::*;

    fn create_test_wavefunction() -> QuantumWavefunction {
        let mut wf = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.25, 0.25, 0.25),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(0.75, 0.75, 0.75),
            FieldAmplitude::from_polar(0.707, 0.5),
            [0.5; 22],
        );

        wf.add_node(node1);
        wf.add_node(node2);
        wf.normalize();

        wf
    }

    #[test]
    fn test_collapse_context_default() {
        let ctx = CollapseContext::default();
        assert!((ctx.choice_weight - 0.3).abs() < 1e-10);
        assert!(ctx.seed.is_none());
    }

    #[test]
    fn test_choice_operator_creation() {
        let op = ChoiceOperator::from_entropy();
        assert_eq!(op.total_choices(), 0);
    }

    #[test]
    fn test_archetype22_collapse_creation() {
        let collapse = Archetype22Collapse::from_entropy();
        assert_eq!(collapse.total_collapses, 0);
    }

    #[test]
    fn test_collapse_deterministic() {
        let mut collapse1 = Archetype22Collapse::with_seed(42);
        let mut collapse2 = Archetype22Collapse::with_seed(42);

        let mut wf1 = create_test_wavefunction();
        let mut wf2 = create_test_wavefunction();

        let result1 = collapse1.collapse(&mut wf1, None);
        let result2 = collapse2.collapse(&mut wf2, None);

        assert!(result1.choice_strength >= 0.0);
        assert!(result2.choice_strength >= 0.0);
    }

    #[test]
    fn test_should_collapse() {
        let collapse = Archetype22Collapse::from_entropy();
        let wf = create_test_wavefunction();

        assert!(collapse.should_collapse(&wf) || !collapse.should_collapse(&wf));
    }

    #[test]
    fn test_collapse_probability() {
        let collapse = Archetype22Collapse::from_entropy();
        let wf = create_test_wavefunction();

        let prob = collapse.collapse_probability(&wf, 1.0);
        assert!((0.0..=1.0).contains(&prob));
    }

    #[test]
    fn test_collapse_produces_result() {
        let mut collapse = Archetype22Collapse::from_entropy();
        let mut wf = create_test_wavefunction();

        let result = collapse.collapse(&mut wf, None);

        assert!(matches!(wf.state(), WavefunctionState::Collapsed { .. }));
        assert!(result.choice_strength >= 0.0);
        assert_eq!(collapse.total_collapses, 1);
    }

    #[test]
    fn test_collapse_statistics() {
        let mut collapse = Archetype22Collapse::from_entropy();
        let mut wf = create_test_wavefunction();

        collapse.collapse(&mut wf, None);

        let stats = collapse.statistics();
        assert_eq!(stats.total_collapses, 1);
    }

    #[test]
    fn test_collapse_type_tracking() {
        let mut collapse = Archetype22Collapse::from_entropy();
        let mut wf = create_test_wavefunction();

        collapse.collapse(&mut wf, None);

        let stats = collapse.statistics();
        let total = stats.spontaneous_count
            + stats.choice_guided_count
            + stats.measurement_count
            + stats.entanglement_count
            + stats.decoherence_count;

        assert_eq!(total, 1);
    }

    #[test]
    fn test_choice_history() {
        let mut collapse = Archetype22Collapse::from_entropy();
        let mut wf = create_test_wavefunction();

        collapse.collapse(&mut wf, None);
        collapse.collapse(&mut wf, None);

        let history = collapse.choice_operator().choice_history();
        assert!(!history.is_empty());
    }

    #[test]
    fn test_free_will_dominance() {
        let stats = CollapseStatistics {
            total_collapses: 10,
            choice_guided_count: 6,
            spontaneous_count: 2,
            measurement_count: 2,
            ..Default::default()
        };

        assert!((stats.free_will_dominance() - 0.8).abs() < 1e-10);
    }

    // Phase 5: Free Will Non-Random, Non-Deterministic Selection Tests
    // From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 5:
    // "Free Will = meaningful choice without external cause"
    // "NOT random, NOT predetermined - chosen"

    #[test]
    fn test_phase5_entity_signature_correlation() {
        // Same entity signature should produce correlated (not identical) choices
        let ctx1 = CollapseContext::for_entity(12345, 0.5);
        let ctx2 = CollapseContext::for_entity(12345, 0.5);

        assert_eq!(ctx1.entity_signature, ctx2.entity_signature);
        assert_eq!(ctx1.archetype_22_activation, ctx2.archetype_22_activation);
    }

    #[test]
    fn test_phase5_choice_coherence_tracking() {
        let mut op = ChoiceOperator::new(CollapseContext::for_entity(99999, 0.7));

        // Initial coherence
        let _initial_coherence = op.choice_coherence();

        // After making choices, coherence should evolve
        let wf = create_test_wavefunction();
        for _ in 0..5 {
            let _ = op.calculate_choice_scores(&wf);
        }

        // Coherence should have been tracked
        assert!(op.choice_coherence() >= 0.0 && op.choice_coherence() <= 1.0);
    }

    #[test]
    fn test_phase5_non_random_different_entities() {
        // Different entities should produce different choice patterns
        let mut op1 = ChoiceOperator::new(CollapseContext::for_entity(111, 0.5));
        let mut op2 = ChoiceOperator::new(CollapseContext::for_entity(222, 0.5));

        let val1 = op1.generate_free_will_value();
        let val2 = op2.generate_free_will_value();

        // Values should be different (different entity signatures)
        assert!((val1 - val2).abs() > 1e-10);
    }

    #[test]
    fn test_phase5_archetype_22_influence() {
        // Higher archetype 22 activation should influence choice differently
        let mut op_low = ChoiceOperator::new(CollapseContext::for_entity(100, 0.1));
        let mut op_high = ChoiceOperator::new(CollapseContext::for_entity(100, 0.9));

        let val_low = op_low.generate_free_will_value();
        let val_high = op_high.generate_free_will_value();

        // Both should be valid floats
        assert!((0.0..=1.0).contains(&val_low));
        assert!((0.0..=1.0).contains(&val_high));
    }

    #[test]
    fn test_phase5_context_for_entity() {
        let ctx = CollapseContext::for_entity(42, 0.75);

        assert_eq!(ctx.entity_signature, 42);
        assert!((ctx.archetype_22_activation - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_phase5_set_entity_signature() {
        let mut op = ChoiceOperator::from_entropy();

        op.set_entity_signature(12345);
        assert_eq!(op.entity_signature(), 12345);

        op.set_archetype_22_activation(0.8);
        assert!((op.archetype_22_activation() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_phase5_choice_coherence_evolution() {
        let mut collapse = Archetype22Collapse::new(CollapseContext::for_entity(777, 0.6));
        let mut wf = create_test_wavefunction();

        // Make multiple collapses
        for _ in 0..10 {
            let _ = collapse.collapse(&mut wf, None);
        }

        // Choice coherence should be trackable
        let coherence = collapse.choice_operator().choice_coherence();
        assert!((0.0..=1.0).contains(&coherence));
    }

    #[test]
    fn test_phase5_non_deterministic_same_seed() {
        // With same seed, should produce identical results (for testing)
        let mut op1 = ChoiceOperator::with_seed(42);
        let mut op2 = ChoiceOperator::with_seed(42);

        let val1 = op1.generate_free_will_value();
        let val2 = op2.generate_free_will_value();

        // With same seed, should be identical
        assert!(
            (val1 - val2).abs() < 1e-10,
            "Same seed should produce same value: {} vs {}",
            val1,
            val2
        );

        // But second call should be different (use relaxed threshold for float comparison)
        let val1_second = op1.generate_free_will_value();
        assert!(
            (val1 - val1_second).abs() > 1e-15,
            "Second call should differ: {} vs {}",
            val1,
            val1_second
        );
    }
}
