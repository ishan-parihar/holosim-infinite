//! Gene Expression as Field Resonance
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Gene expression as field resonance - genes are NOT switched on/off,
//!  but resonate with field configurations to produce protein expression."
//!
//! # Key Insight
//!
//! Gene expression is a resonance phenomenon:
//! - Field amplitude at gene locus determines expression strength
//! - Phase alignment between gene and field enables expression
//! - Regulatory genes modulate field resonance at target loci

use super::archetype_genes::{ArchetypeGene, GeneCategory, GeneId, GeneRegulatoryNetwork};
use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::field_state::HolographicFieldState;
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegulatorySignal {
    pub source_gene: GeneId,
    pub target_gene: GeneId,
    pub signal_strength: Float,
    pub signal_type: SignalType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType {
    Activation,
    Repression,
    Enhancement,
    Silencing,
}

impl RegulatorySignal {
    pub fn new(source: GeneId, target: GeneId, strength: Float, signal_type: SignalType) -> Self {
        Self {
            source_gene: source,
            target_gene: target,
            signal_strength: strength,
            signal_type,
        }
    }

    pub fn apply_to_expression(&self, base_expression: Float) -> Float {
        match self.signal_type {
            SignalType::Activation => base_expression + self.signal_strength * 0.3,
            SignalType::Repression => (base_expression - self.signal_strength * 0.3).max(0.0),
            SignalType::Enhancement => base_expression * (1.0 + self.signal_strength * 0.5),
            SignalType::Silencing => base_expression * (1.0 - self.signal_strength * 0.8),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionCondition {
    pub field_resonance: Float,
    pub phase_alignment: Float,
    pub regulatory_input: Float,
    pub epigenetic_modifier: Float,
}

impl Default for ExpressionCondition {
    fn default() -> Self {
        Self {
            field_resonance: 0.5,
            phase_alignment: 0.5,
            regulatory_input: 0.0,
            epigenetic_modifier: 1.0,
        }
    }
}

impl ExpressionCondition {
    pub fn from_field(field: &HolographicFieldState, position: &Position3D) -> Self {
        let field_resonance = field
            .get_node_at(position)
            .map(|n| n.coherence)
            .unwrap_or(0.5);
        let phase_alignment = 0.5;

        Self {
            field_resonance,
            phase_alignment,
            regulatory_input: 0.0,
            epigenetic_modifier: 1.0,
        }
    }

    pub fn with_regulatory_input(mut self, input: Float) -> Self {
        self.regulatory_input = input;
        self
    }

    pub fn with_epigenetic_modifier(mut self, modifier: Float) -> Self {
        self.epigenetic_modifier = modifier.clamp(0.0, 2.0);
        self
    }

    pub fn calculate_expression_probability(&self, gene: &ArchetypeGene) -> Float {
        let base = if gene.can_express(self.field_resonance) {
            gene.expression_strength
        } else {
            0.0
        };

        let phase_factor = self.phase_alignment;
        let regulatory_factor = 1.0 + self.regulatory_input;
        let epigenetic_factor = self.epigenetic_modifier;

        (base * phase_factor * regulatory_factor * epigenetic_factor).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionResult {
    pub gene_id: GeneId,
    pub expression_level: Float,
    pub protein_output: Float,
    pub signals_sent: Vec<RegulatorySignal>,
}

impl ExpressionResult {
    pub fn new(gene_id: GeneId, expression_level: Float) -> Self {
        Self {
            gene_id,
            expression_level,
            protein_output: expression_level,
            signals_sent: Vec::new(),
        }
    }

    pub fn with_protein_output(mut self, output: Float) -> Self {
        self.protein_output = output;
        self
    }

    pub fn add_signal(mut self, signal: RegulatorySignal) -> Self {
        self.signals_sent.push(signal);
        self
    }
}

#[derive(Debug, Clone)]
pub struct FieldResonanceExpression {
    gene: ArchetypeGene,
    resonance_threshold: Float,
    current_expression: Float,
    expression_history: Vec<Float>,
}

impl FieldResonanceExpression {
    pub fn new(gene: ArchetypeGene) -> Self {
        Self {
            resonance_threshold: gene.expression_threshold,
            gene,
            current_expression: 0.0,
            expression_history: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, condition: &ExpressionCondition) -> ExpressionResult {
        let expression_prob = condition.calculate_expression_probability(&self.gene);

        let expression_level = if condition.field_resonance >= self.resonance_threshold {
            expression_prob
        } else {
            0.0
        };

        self.current_expression = expression_level;
        self.expression_history.push(expression_level);

        if self.expression_history.len() > 100 {
            self.expression_history.remove(0);
        }

        let protein_output = if self.gene.category == GeneCategory::Structural {
            expression_level * self.gene.expression_strength
        } else {
            0.0
        };

        ExpressionResult::new(self.gene.id, expression_level).with_protein_output(protein_output)
    }

    pub fn gene(&self) -> &ArchetypeGene {
        &self.gene
    }

    pub fn current_expression(&self) -> Float {
        self.current_expression
    }

    pub fn average_expression(&self) -> Float {
        if self.expression_history.is_empty() {
            return 0.0;
        }
        self.expression_history.iter().sum::<Float>() / self.expression_history.len() as Float
    }

    pub fn expression_variance(&self) -> Float {
        if self.expression_history.len() < 2 {
            return 0.0;
        }

        let mean = self.average_expression();
        let variance: Float = self
            .expression_history
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<Float>()
            / self.expression_history.len() as Float;

        variance
    }
}

#[derive(Debug, Clone)]
pub struct GeneExpressionEngine {
    expressions: HashMap<GeneId, FieldResonanceExpression>,
    network: GeneRegulatoryNetwork,
    pending_signals: Vec<RegulatorySignal>,
    global_expression_level: Float,
}

impl GeneExpressionEngine {
    pub fn new(network: GeneRegulatoryNetwork) -> Self {
        let mut expressions = HashMap::new();

        for (id, gene) in network.genes.iter() {
            expressions.insert(*id, FieldResonanceExpression::new(gene.clone()));
        }

        Self {
            expressions,
            network,
            pending_signals: Vec::new(),
            global_expression_level: 0.5,
        }
    }

    pub fn evaluate_all(&mut self, condition: &ExpressionCondition) -> Vec<ExpressionResult> {
        let mut results = Vec::new();

        self.apply_pending_signals();

        let gene_ids: Vec<GeneId> = self.expressions.keys().cloned().collect();
        let mut regulatory_inputs: HashMap<GeneId, Float> = HashMap::new();

        for id in &gene_ids {
            if let Some(expr) = self.expressions.get(id) {
                if expr.gene().category == GeneCategory::Regulatory {
                    let targets = self.network.get_regulated_genes(id);
                    if let Some(targets) = targets {
                        for (target_id, strength) in targets {
                            let input = regulatory_inputs.entry(*target_id).or_insert(0.0);
                            *input += strength * expr.current_expression();
                        }
                    }
                }
            }
        }

        let mut new_signals = Vec::new();

        for id in gene_ids {
            if let Some(expr) = self.expressions.get_mut(&id) {
                let mut gene_condition = condition.clone();

                if let Some(&reg_input) = regulatory_inputs.get(&id) {
                    gene_condition = gene_condition.with_regulatory_input(reg_input);
                }

                if expr.gene().category == GeneCategory::Epigenetic {
                    let epigenetic_factor = expr.current_expression();
                    gene_condition =
                        gene_condition.with_epigenetic_modifier(0.5 + epigenetic_factor);
                }

                let result = expr.evaluate(&gene_condition);
                results.push(result);
            }
        }

        self.pending_signals = new_signals;
        self.calculate_global_level();

        results
    }

    fn apply_pending_signals(&mut self) {
        for signal in self.pending_signals.drain(..) {
            if let Some(expr) = self.expressions.get_mut(&signal.target_gene) {
                let adjusted = signal.apply_to_expression(expr.current_expression());
                expr.current_expression = adjusted;
            }
        }
    }

    fn calculate_global_level(&mut self) {
        let total: Float = self
            .expressions
            .values()
            .map(|e| e.current_expression())
            .sum();
        let count = self.expressions.len().max(1);

        self.global_expression_level = total / count as Float;
    }

    pub fn get_expression(&self, gene_id: &GeneId) -> Option<&FieldResonanceExpression> {
        self.expressions.get(gene_id)
    }

    pub fn global_expression_level(&self) -> Float {
        self.global_expression_level
    }

    pub fn gene_count(&self) -> usize {
        self.expressions.len()
    }

    pub fn actively_expressed_count(&self, threshold: Float) -> usize {
        self.expressions
            .values()
            .filter(|e| e.current_expression() >= threshold)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::super::archetype_genes::ArchetypeGene;
    use super::*;

    fn create_test_gene() -> ArchetypeGene {
        ArchetypeGene::from_archetype(0, 0.7)
    }

    fn create_test_gene_structural() -> ArchetypeGene {
        ArchetypeGene::from_archetype(8, 0.7)
    }

    #[test]
    fn test_regulatory_signal_activation() {
        let signal =
            RegulatorySignal::new(GeneId::new(1), GeneId::new(2), 0.5, SignalType::Activation);
        let result = signal.apply_to_expression(0.5);
        assert!(result > 0.5);
    }

    #[test]
    fn test_regulatory_signal_repression() {
        let signal =
            RegulatorySignal::new(GeneId::new(1), GeneId::new(2), 0.5, SignalType::Repression);
        let result = signal.apply_to_expression(0.5);
        assert!(result < 0.5);
    }

    #[test]
    fn test_expression_condition_default() {
        let condition = ExpressionCondition::default();
        assert_eq!(condition.field_resonance, 0.5);
        assert_eq!(condition.phase_alignment, 0.5);
    }

    #[test]
    fn test_expression_probability() {
        let gene = create_test_gene();
        let condition = ExpressionCondition::default();
        let prob = condition.calculate_expression_probability(&gene);
        assert!(prob >= 0.0 && prob <= 1.0);
    }

    #[test]
    fn test_field_resonance_expression_creation() {
        let gene = create_test_gene();
        let expr = FieldResonanceExpression::new(gene);
        assert_eq!(expr.current_expression(), 0.0);
    }

    #[test]
    fn test_field_resonance_expression_evaluate() {
        let gene = create_test_gene();
        let mut expr = FieldResonanceExpression::new(gene);
        let condition = ExpressionCondition::default();

        let result = expr.evaluate(&condition);
        assert!(result.expression_level >= 0.0);
    }

    #[test]
    fn test_structural_gene_produces_protein() {
        let gene = create_test_gene_structural();
        let mut expr = FieldResonanceExpression::new(gene);
        let condition = ExpressionCondition {
            field_resonance: 0.8,
            ..Default::default()
        };

        let result = expr.evaluate(&condition);
        assert!(result.protein_output >= 0.0);
    }

    #[test]
    fn test_expression_history() {
        let gene = create_test_gene();
        let mut expr = FieldResonanceExpression::new(gene);
        let condition = ExpressionCondition::default();

        for _ in 0..5 {
            expr.evaluate(&condition);
        }

        assert_eq!(expr.expression_history.len(), 5);
    }

    #[test]
    fn test_average_expression() {
        let gene = create_test_gene();
        let mut expr = FieldResonanceExpression::new(gene);
        let condition = ExpressionCondition::default();

        expr.evaluate(&condition);
        let avg = expr.average_expression();
        assert!(avg >= 0.0);
    }

    #[test]
    fn test_expression_result_creation() {
        let result = ExpressionResult::new(GeneId::new(1), 0.5);
        assert_eq!(result.expression_level, 0.5);
    }

    #[test]
    fn test_epigenetic_modifier_affects_expression() {
        let gene = create_test_gene();
        let condition1 = ExpressionCondition::default();
        let condition2 = ExpressionCondition {
            epigenetic_modifier: 1.5,
            ..Default::default()
        };

        let prob1 = condition1.calculate_expression_probability(&gene);
        let prob2 = condition2.calculate_expression_probability(&gene);

        assert!(prob2 >= prob1);
    }
}
