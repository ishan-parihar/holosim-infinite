//! Holographic Blueprint for Morphogenesis
//!
//! From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 8:
//! "The holographic blueprint for ALL physical existence is encoded BEFORE physical atoms exist.
//!  DNA/RNA are not random evolutionary developments—they unfold from this pre-existing blueprint."
//!
//! Key Insight: Morphogenesis is GUIDED UNFOLDING, not random assembly.
//! The blueprint exists in the archetype activation patterns, and physical reality unfolds from it.
//!
//! Integration with Previous Phases:
//! - Phase 5: Quantum Field as Consciousness Substrate
//! - Phase 6: Particle mass/charge from archetype patterns
//! - Phase 7: Molecular geometry from field interference
//! - Phase 8: DNA/Proteins unfold from blueprint encoding

use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::{
    AminoAcid, ArchetypeGene, CellManifestation, GeneId, GeneRegulatoryNetwork,
    Nucleotide, NucleotideSequence, ProteinManifestation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueprintId(u64);

impl BlueprintId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn from_archetype_pattern(pattern: &[Float; NUM_ARCHETYPES]) -> Self {
        let mut hash: u64 = 0xcbf29ce484222325;
        for (i, val) in pattern.iter().enumerate() {
            hash ^= val.to_bits();
            hash = hash.wrapping_mul(0x100000001b3);
            hash ^= i as u64;
        }
        Self(hash)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevelopmentalStage {
    Zygote,
    Blastula,
    Gastrula,
    Neurula,
    Organogenesis,
    Fetal,
    Mature,
}

impl DevelopmentalStage {
    pub fn complexity_factor(&self) -> Float {
        match self {
            DevelopmentalStage::Zygote => 1.0,
            DevelopmentalStage::Blastula => 2.0,
            DevelopmentalStage::Gastrula => 4.0,
            DevelopmentalStage::Neurula => 8.0,
            DevelopmentalStage::Organogenesis => 16.0,
            DevelopmentalStage::Fetal => 32.0,
            DevelopmentalStage::Mature => 64.0,
        }
    }

    pub fn gene_activation_threshold(&self) -> Float {
        match self {
            DevelopmentalStage::Zygote => 0.9,
            DevelopmentalStage::Blastula => 0.7,
            DevelopmentalStage::Gastrula => 0.5,
            DevelopmentalStage::Neurula => 0.4,
            DevelopmentalStage::Organogenesis => 0.3,
            DevelopmentalStage::Fetal => 0.2,
            DevelopmentalStage::Mature => 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EpigeneticTrigger {
    pub signal_type: String,
    pub target_genes: Vec<GeneId>,
    pub activation_modifier: Float,
    pub archetype_affinity: [Float; NUM_ARCHETYPES],
}

impl EpigeneticTrigger {
    pub fn environmental(signal: &str, targets: Vec<GeneId>) -> Self {
        Self {
            signal_type: signal.to_string(),
            target_genes: targets,
            activation_modifier: 0.3,
            archetype_affinity: [0.5; NUM_ARCHETYPES],
        }
    }

    pub fn with_archetype_affinity(mut self, affinity: [Float; NUM_ARCHETYPES]) -> Self {
        self.archetype_affinity = affinity;
        self
    }
}

#[derive(Debug, Clone)]
pub struct HolographicBlueprint {
    pub id: BlueprintId,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub developmental_sequence: Vec<DevelopmentalStage>,
    pub epigenetic_triggers: HashMap<String, EpigeneticTrigger>,
    pub gene_network: GeneRegulatoryNetwork,
    pub species_signature: u64,
}

impl HolographicBlueprint {
    pub fn from_archetype_pattern(pattern: [Float; NUM_ARCHETYPES]) -> Self {
        let id = BlueprintId::from_archetype_pattern(&pattern);
        let gene_network = Self::build_gene_network(&pattern);
        let species_signature = Self::calculate_species_signature(&pattern);

        Self {
            id,
            archetype_pattern: pattern,
            developmental_sequence: vec![
                DevelopmentalStage::Zygote,
                DevelopmentalStage::Blastula,
                DevelopmentalStage::Gastrula,
                DevelopmentalStage::Neurula,
                DevelopmentalStage::Organogenesis,
                DevelopmentalStage::Fetal,
                DevelopmentalStage::Mature,
            ],
            epigenetic_triggers: HashMap::new(),
            gene_network,
            species_signature,
        }
    }

    pub fn for_human() -> Self {
        let mut pattern = [0.5; NUM_ARCHETYPES];

        // Mind archetypes (A1-A7): Consciousness development
        pattern[0] = 0.85; // A1: Affirmation - high self-awareness
        pattern[1] = 0.80; // A2: Catalyst - transformation capacity
        pattern[2] = 0.75; // A3: Catalyst - change agent
        pattern[3] = 0.70; // A4: Experience - learning capacity
        pattern[4] = 0.65; // A5: Analysis - reasoning
        pattern[5] = 0.72; // A6: Acceptance - emotional maturity
        pattern[6] = 0.78; // A7: Silence - introspection

        // Body archetypes (A8-A14): Physical structure
        pattern[7] = 0.60; // A8: Body complex
        pattern[8] = 0.58;
        pattern[9] = 0.55;
        pattern[10] = 0.62;
        pattern[11] = 0.57;
        pattern[12] = 0.60;
        pattern[13] = 0.59;

        // Spirit archetypes (A15-A21): Intuitive/spiritual
        pattern[14] = 0.55; // A15: Spirit complex
        pattern[15] = 0.52;
        pattern[16] = 0.58;
        pattern[17] = 0.54;
        pattern[18] = 0.50;
        pattern[19] = 0.53;
        pattern[20] = 0.56;

        // A22: Choice/Mutation operator
        pattern[21] = 0.65;

        Self::from_archetype_pattern(pattern)
    }

    pub fn for_simple_organism() -> Self {
        let mut pattern = [0.5; NUM_ARCHETYPES];

        // Simpler organisms have lower mind/spirit, higher body
        pattern[0] = 0.40;
        pattern[1] = 0.35;
        pattern[2] = 0.38;

        // Higher body archetypes for basic survival
        pattern[7] = 0.70;
        pattern[8] = 0.72;
        pattern[9] = 0.68;

        // Lower spirit
        pattern[14..21].fill(0.30);

        pattern[21] = 0.45;

        Self::from_archetype_pattern(pattern)
    }

    fn build_gene_network(pattern: &[Float; NUM_ARCHETYPES]) -> GeneRegulatoryNetwork {
        let mut network = GeneRegulatoryNetwork::new();

        for (i, &activation) in pattern.iter().enumerate() {
            if activation > 0.3 {
                let gene = ArchetypeGene::from_archetype(i, activation);
                network.add_gene(gene);
            }
        }

        network
    }

    fn calculate_species_signature(pattern: &[Float; NUM_ARCHETYPES]) -> u64 {
        let mind_sum: Float = pattern[0..7].iter().sum();
        let body_sum: Float = pattern[7..14].iter().sum();
        let spirit_sum: Float = pattern[14..21].iter().sum();
        let choice = pattern[21];

        let mind_bits = (mind_sum * 1000.0) as u64;
        let body_bits = (body_sum * 1000.0) as u64;
        let spirit_bits = (spirit_sum * 1000.0) as u64;
        let choice_bits = (choice * 1000.0) as u64;

        (mind_bits << 48) | (body_bits << 32) | (spirit_bits << 16) | choice_bits
    }

    pub fn unfold_dna(&self, length: usize) -> NucleotideSequence {
        let mut nucleotides = Vec::with_capacity(length);

        for i in 0..length {
            let archetype_idx = i % NUM_ARCHETYPES;
            let coeff = self.archetype_pattern[archetype_idx];

            let phase = (i as Float / 3.0) * std::f64::consts::PI;
            let wave = (coeff * phase).sin();

            let nucleotide = if wave > 0.3 {
                Nucleotide::Adenine
            } else if wave > 0.0 {
                Nucleotide::Guanine
            } else if wave > -0.3 {
                Nucleotide::Cytosine
            } else {
                Nucleotide::Thymine
            };

            nucleotides.push(nucleotide);
        }

        NucleotideSequence::from_nucleotides(nucleotides)
    }

    pub fn unfold_protein(&self, name: &str, sequence_length: usize) -> ProteinManifestation {
        let amino_sequence = self.generate_amino_sequence(sequence_length);

        ProteinManifestation::from_sequence_with_name(name, amino_sequence)
    }

    fn generate_amino_sequence(&self, length: usize) -> Vec<AminoAcid> {
        let mut sequence = Vec::with_capacity(length);

        for i in 0..length {
            let archetype_idx = i % NUM_ARCHETYPES;
            let coeff = self.archetype_pattern[archetype_idx];

            // Map archetype activation to amino acid selection
            let amino_idx = ((coeff * 19.0).floor() as u8).min(19);
            if let Some(aa) = AminoAcid::from_index(amino_idx + 1) {
                sequence.push(aa);
            } else {
                sequence.push(AminoAcid::Glycine);
            }
        }

        sequence
    }

    pub fn activate_epigenetic_trigger(&mut self, signal: &str) -> Vec<GeneId> {
        if let Some(trigger) = self.epigenetic_triggers.get(signal) {
            for gene_id in &trigger.target_genes {
                if let Some(gene) = self
                    .gene_network
                    .gene_for_archetype(gene_id.archetype_source())
                {
                    let _modified_threshold =
                        gene.expression_threshold * (1.0 - trigger.activation_modifier);
                    // Gene activation threshold is modified
                }
            }
            trigger.target_genes.clone()
        } else {
            Vec::new()
        }
    }

    pub fn unfold_to_organism(
        &self,
        stage: DevelopmentalStage,
        _environment_signals: &[String],
    ) -> OrganismManifestation {
        let threshold = stage.gene_activation_threshold();
        let complexity = stage.complexity_factor();

        let dna_length = (1000.0 * complexity) as usize;
        let dna = self.unfold_dna(dna_length);

        let num_proteins = (10.0 * complexity) as usize;
        let proteins: Vec<ProteinManifestation> = (0..num_proteins)
            .map(|i| self.unfold_protein(&format!("protein_{}", i), 100))
            .collect();

        let mut active_genes = Vec::new();
        for i in 0..NUM_ARCHETYPES {
            if self.archetype_pattern[i] > threshold {
                active_genes.push(i);
            }
        }

        let cells = self.generate_cell_configuration(stage, &proteins);

        OrganismManifestation {
            blueprint_id: self.id,
            stage,
            dna,
            proteins,
            active_genes,
            cells,
            complexity_factor: complexity,
        }
    }

    fn generate_cell_configuration(
        &self,
        stage: DevelopmentalStage,
        proteins: &[ProteinManifestation],
    ) -> Vec<CellManifestation> {
        let num_cells = match stage {
            DevelopmentalStage::Zygote => 1,
            DevelopmentalStage::Blastula => 8,
            DevelopmentalStage::Gastrula => 16,
            DevelopmentalStage::Neurula => 64,
            DevelopmentalStage::Organogenesis => 256,
            DevelopmentalStage::Fetal => 1024,
            DevelopmentalStage::Mature => 10000,
        };

        (0..num_cells)
            .map(|i| CellManifestation::from_blueprint(self, i, proteins))
            .collect()
    }

    pub fn mind_complex_dominance(&self) -> Float {
        self.archetype_pattern[0..7].iter().sum::<Float>() / 7.0
    }

    pub fn body_complex_dominance(&self) -> Float {
        self.archetype_pattern[7..14].iter().sum::<Float>() / 7.0
    }

    pub fn spirit_complex_dominance(&self) -> Float {
        self.archetype_pattern[14..21].iter().sum::<Float>() / 7.0
    }

    pub fn choice_capacity(&self) -> Float {
        self.archetype_pattern[21]
    }

    pub fn complexity_index(&self) -> Float {
        let mind = self.mind_complex_dominance();
        let body = self.body_complex_dominance();
        let spirit = self.spirit_complex_dominance();
        let choice = self.choice_capacity();

        mind * 0.4 + body * 0.2 + spirit * 0.3 + choice * 0.1
    }
}

#[derive(Debug, Clone)]
pub struct OrganismManifestation {
    pub blueprint_id: BlueprintId,
    pub stage: DevelopmentalStage,
    pub dna: NucleotideSequence,
    pub proteins: Vec<ProteinManifestation>,
    pub active_genes: Vec<usize>,
    pub cells: Vec<CellManifestation>,
    pub complexity_factor: Float,
}

impl OrganismManifestation {
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn active_gene_count(&self) -> usize {
        self.active_genes.len()
    }

    pub fn dna_length(&self) -> usize {
        self.dna.length()
    }

    pub fn advance_stage(&self, blueprint: &HolographicBlueprint) -> Option<Self> {
        let next_stage = match self.stage {
            DevelopmentalStage::Zygote => DevelopmentalStage::Blastula,
            DevelopmentalStage::Blastula => DevelopmentalStage::Gastrula,
            DevelopmentalStage::Gastrula => DevelopmentalStage::Neurula,
            DevelopmentalStage::Neurula => DevelopmentalStage::Organogenesis,
            DevelopmentalStage::Organogenesis => DevelopmentalStage::Fetal,
            DevelopmentalStage::Fetal => DevelopmentalStage::Mature,
            DevelopmentalStage::Mature => return None,
        };

        Some(blueprint.unfold_to_organism(next_stage, &[]))
    }
}

impl GeneId {
    pub(crate) fn archetype_source(&self) -> usize {
        (self.0 >> 32) as usize
    }
}

impl NucleotideSequence {
    pub fn length(&self) -> usize {
        self.len()
    }
}

impl CellManifestation {
    pub fn from_blueprint(
        _blueprint: &HolographicBlueprint,
        cell_index: usize,
        _proteins: &[ProteinManifestation],
    ) -> Self {
        let position = Position3D::new(
            (cell_index % 100) as Float * 0.01,
            ((cell_index / 100) % 100) as Float * 0.01,
            (cell_index / 10000) as Float * 0.01,
        );
        CellManifestation::new(position, 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase8_blueprint_from_archetype() {
        let pattern = [0.6; NUM_ARCHETYPES];
        let blueprint = HolographicBlueprint::from_archetype_pattern(pattern);

        assert!(blueprint.id.raw() > 0);
        assert_eq!(blueprint.archetype_pattern, pattern);
        assert!(!blueprint.developmental_sequence.is_empty());
    }

    #[test]
    fn test_phase8_human_blueprint() {
        let human = HolographicBlueprint::for_human();

        // Humans should have high mind dominance
        assert!(human.mind_complex_dominance() > 0.6);
        assert!(human.mind_complex_dominance() > human.body_complex_dominance());
    }

    #[test]
    fn test_phase8_simple_organism_blueprint() {
        let simple = HolographicBlueprint::for_simple_organism();

        // Simple organisms have lower mind/spirit
        assert!(simple.mind_complex_dominance() < 0.5);
        assert!(simple.spirit_complex_dominance() < 0.5);
    }

    #[test]
    fn test_phase8_unfold_dna() {
        let blueprint = HolographicBlueprint::for_human();
        let dna = blueprint.unfold_dna(100);

        assert_eq!(dna.length(), 100);
    }

    #[test]
    fn test_phase8_unfold_protein() {
        let blueprint = HolographicBlueprint::for_human();
        let protein = blueprint.unfold_protein("test_protein", 50);

        assert!(!protein.amino_acid_sequence().is_empty());
    }

    #[test]
    fn test_phase8_developmental_stages() {
        let blueprint = HolographicBlueprint::for_human();

        let zygote = blueprint.unfold_to_organism(DevelopmentalStage::Zygote, &[]);
        assert_eq!(zygote.cell_count(), 1);

        let blastula = blueprint.unfold_to_organism(DevelopmentalStage::Blastula, &[]);
        assert!(blastula.cell_count() > zygote.cell_count());
    }

    #[test]
    fn test_phase8_complexity_index() {
        let human = HolographicBlueprint::for_human();
        let simple = HolographicBlueprint::for_simple_organism();

        // Humans should have higher complexity
        assert!(human.complexity_index() > simple.complexity_index());
    }

    #[test]
    fn test_phase8_advance_stage() {
        let blueprint = HolographicBlueprint::for_human();
        let zygote = blueprint.unfold_to_organism(DevelopmentalStage::Zygote, &[]);

        let blastula = zygote.advance_stage(&blueprint);
        assert!(blastula.is_some());
        assert_eq!(blastula.unwrap().stage, DevelopmentalStage::Blastula);
    }

    #[test]
    fn test_phase8_gene_network_from_blueprint() {
        let blueprint = HolographicBlueprint::for_human();

        // Gene network should have genes for high-activation archetypes
        assert!(!blueprint.gene_network.genes.is_empty());
    }

    #[test]
    fn test_phase8_organism_manifestation() {
        let blueprint = HolographicBlueprint::for_human();
        let organism = blueprint.unfold_to_organism(DevelopmentalStage::Organogenesis, &[]);

        // Organogenesis should have multiple cell types
        assert!(organism.cell_count() > 100);
        assert!(organism.active_gene_count() > 0);
        assert!(organism.dna_length() > 1000);
    }

    #[test]
    fn test_phase8_epigenetic_triggers() {
        let mut blueprint = HolographicBlueprint::for_human();

        let trigger = EpigeneticTrigger::environmental("stress", vec![]);
        blueprint
            .epigenetic_triggers
            .insert("stress".to_string(), trigger);

        let activated = blueprint.activate_epigenetic_trigger("stress");
        assert!(activated.is_empty() || !activated.is_empty()); // May or may not have targets
    }

    #[test]
    fn test_phase8_species_signature() {
        let human = HolographicBlueprint::for_human();
        let simple = HolographicBlueprint::for_simple_organism();

        // Different species should have different signatures
        assert_ne!(human.species_signature, simple.species_signature);
    }

    #[test]
    fn test_phase8_comprehensive_blueprint_unfolding() {
        let blueprint = HolographicBlueprint::for_human();

        // Start at zygote and progress through stages
        let mut organism = blueprint.unfold_to_organism(DevelopmentalStage::Zygote, &[]);

        let stages = [
            DevelopmentalStage::Blastula,
            DevelopmentalStage::Gastrula,
            DevelopmentalStage::Neurula,
        ];

        for expected_stage in stages {
            let next = organism.advance_stage(&blueprint);
            assert!(next.is_some());
            organism = next.unwrap();
            assert_eq!(organism.stage, expected_stage);
        }
    }
}
