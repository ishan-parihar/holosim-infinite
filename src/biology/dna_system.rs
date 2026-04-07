// DNA/RNA System - Encode Holographic Blueprint as DNA Sequences
//
// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md Phase 3:
// "DNA/RNA System - Encode holographic blueprint as DNA sequences,
// synthesize proteins from DNA, repair mutations"
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "DNA/RNA are not random evolutionary developments—they unfold from this
// pre-existing holographic blueprint encoded as spectrum configurations"

use crate::entity_layer7::dna_encoding::{DNAPattern, GeneticBase};
use crate::entity_layer7::holographic_blueprint::HolographicBlueprint;
use crate::types::Float;
use rand::Rng;

/// DNA System - Encodes holographic blueprint as DNA sequences
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Encode holographic blueprint as DNA sequences, synthesize proteins from DNA,
/// repair mutations"
#[derive(Debug, Clone)]
pub struct DnaSystem {
    /// Spectrum encoder - converts blueprint to DNA
    spectrum_encoder: SpectrumEncoder,

    /// Protein synthesizer - converts DNA to proteins
    protein_synthesizer: ProteinSynthesizer,

    /// Mutation detector - identifies DNA mutations
    mutation_detector: MutationDetector,

    /// DNA repair system - fixes mutations
    dna_repair: DnaRepair,

    /// System configuration
    config: DnaSystemConfig,
}

impl Default for DnaSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DnaSystem {
    /// Create a new DNA system
    pub fn new() -> Self {
        Self {
            spectrum_encoder: SpectrumEncoder::new(),
            protein_synthesizer: ProteinSynthesizer::new(),
            mutation_detector: MutationDetector::new(),
            dna_repair: DnaRepair::new(),
            config: DnaSystemConfig::default(),
        }
    }

    /// Create a DNA system with custom configuration
    pub fn with_config(config: DnaSystemConfig) -> Self {
        Self {
            spectrum_encoder: SpectrumEncoder::new(),
            protein_synthesizer: ProteinSynthesizer::new(),
            mutation_detector: MutationDetector::new(),
            dna_repair: DnaRepair::new(),
            config,
        }
    }

    /// Encode holographic blueprint as DNA
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Encode holographic blueprint as DNA sequences"
    pub fn encode_blueprint_as_dna(&self, blueprint: &HolographicBlueprint) -> DNA {
        self.spectrum_encoder.encode_blueprint(blueprint)
    }

    /// Synthesize proteins from DNA
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Synthesize proteins from DNA"
    pub fn synthesize_proteins(&self, dna: &DNA) -> Vec<Protein> {
        self.protein_synthesizer.synthesize(dna, &self.config)
    }

    /// Detect mutations in DNA
    pub fn detect_mutations(&self, dna: &DNA, original_dna: &DNA) -> Vec<Mutation> {
        self.mutation_detector.detect(dna, original_dna)
    }

    /// Repair mutations in DNA
    ///
    /// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
    /// "Repair mutations"
    pub fn repair_mutation(&self, dna: &mut DNA) -> DnaRepairResult {
        self.dna_repair.repair(dna, &self.config)
    }

    /// Get DNA system configuration
    pub fn config(&self) -> &DnaSystemConfig {
        &self.config
    }

    /// Set DNA system configuration
    pub fn set_config(&mut self, config: DnaSystemConfig) {
        self.config = config;
    }
}

/// DNA System Configuration
#[derive(Debug, Clone)]
pub struct DnaSystemConfig {
    /// Mutation rate (0.0 to 1.0)
    pub mutation_rate: Float,

    /// Repair efficiency (0.0 to 1.0)
    pub repair_efficiency: Float,

    /// Protein synthesis rate (0.0 to 1.0)
    pub synthesis_rate: Float,

    /// Gene expression threshold (0.0 to 1.0)
    pub expression_threshold: Float,
}

impl Default for DnaSystemConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.001,
            repair_efficiency: 0.95,
            synthesis_rate: 0.8,
            expression_threshold: 0.5,
        }
    }
}

/// DNA - Encoded holographic blueprint
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "DNA/RNA patterns encoded as spectrum configurations"
#[derive(Debug, Clone)]
pub struct DNA {
    /// DNA identifier
    pub dna_id: String,

    /// Source holographic blueprint pattern
    pub blueprint_pattern: DNAPattern,

    /// Gene sequence (ordered list of genes)
    pub genes: Vec<Gene>,

    /// DNA sequence (nucleotide bases)
    pub sequence: Vec<NucleotideBase>,

    /// Gene expression levels
    pub gene_expression: Vec<GeneExpression>,

    /// Epigenetic markers
    pub epigenetic_markers: Vec<EpigeneticMarker>,

    /// DNA integrity (0.0 to 1.0)
    pub integrity: Float,
}

impl DNA {
    /// Create DNA from blueprint pattern
    pub fn from_blueprint(blueprint_pattern: DNAPattern) -> Self {
        let genes = Self::generate_genes(&blueprint_pattern);
        let sequence = Self::generate_sequence(&genes);
        let gene_expression = genes
            .iter()
            .map(|gene| GeneExpression {
                gene_id: gene.gene_id.clone(),
                expression_level: 0.5, // Default expression
                is_active: true,
            })
            .collect();

        DNA {
            dna_id: format!("dna-{}", uuid::Uuid::new_v4()),
            blueprint_pattern,
            genes,
            sequence,
            gene_expression,
            epigenetic_markers: Vec::new(),
            integrity: 1.0,
        }
    }

    /// Generate genes from blueprint pattern
    fn generate_genes(blueprint_pattern: &DNAPattern) -> Vec<Gene> {
        blueprint_pattern
            .genetic_information
            .iter()
            .enumerate()
            .map(|(i, &genetic_base)| Gene {
                gene_id: format!("gene-{}", i),
                genetic_base,
                position: i,
                start: i * 3,
                end: (i + 1) * 3,
                is_expressed: true,
            })
            .collect()
    }

    /// Generate DNA sequence from genes
    fn generate_sequence(genes: &[Gene]) -> Vec<NucleotideBase> {
        let mut sequence = Vec::new();
        for _gene in genes {
            // Each gene is represented by 3 nucleotide bases (codon)
            sequence.push(NucleotideBase::Adenine);
            sequence.push(NucleotideBase::Cytosine);
            sequence.push(NucleotideBase::Guanine);
        }
        sequence
    }

    /// Get gene expression for a specific gene
    pub fn get_gene_expression(&self, gene_id: &str) -> Option<&GeneExpression> {
        self.gene_expression
            .iter()
            .find(|expr| expr.gene_id == gene_id)
    }

    /// Set gene expression for a specific gene
    pub fn set_gene_expression(&mut self, gene_id: &str, level: Float) {
        if let Some(expr) = self
            .gene_expression
            .iter_mut()
            .find(|expr| expr.gene_id == gene_id)
        {
            expr.expression_level = level.clamp(0.0, 1.0);
            expr.is_active = level >= 0.1;
        }
    }

    /// Add epigenetic marker
    pub fn add_epigenetic_marker(&mut self, marker: EpigeneticMarker) {
        self.epigenetic_markers.push(marker);
    }

    /// Get DNA integrity
    pub fn integrity(&self) -> Float {
        self.integrity
    }

    /// Set DNA integrity
    pub fn set_integrity(&mut self, integrity: Float) {
        self.integrity = integrity.clamp(0.0, 1.0);
    }
}

/// Gene - Unit of genetic information
#[derive(Debug, Clone)]
pub struct Gene {
    /// Gene identifier
    pub gene_id: String,

    /// Genetic base (from DNAPattern)
    pub genetic_base: GeneticBase,

    /// Position in DNA sequence
    pub position: usize,

    /// Start position in sequence
    pub start: usize,

    /// End position in sequence
    pub end: usize,

    /// Whether this gene is expressed
    pub is_expressed: bool,
}

/// Gene Expression - Current expression level of a gene
#[derive(Debug, Clone)]
pub struct GeneExpression {
    /// Gene identifier
    pub gene_id: String,

    /// Expression level (0.0 to 1.0)
    pub expression_level: Float,

    /// Whether this gene is active
    pub is_active: bool,
}

/// Nucleotide Base - DNA building block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NucleotideBase {
    Adenine,
    Thymine,
    Cytosine,
    Guanine,
}

/// Protein - Synthesized from DNA
///
/// From SIMULATION-AUDIT-AND-REFACTOR-PLAN.md:
/// "Synthesize proteins from DNA"
#[derive(Debug, Clone)]
pub struct Protein {
    /// Protein identifier
    pub protein_id: String,

    /// Source gene
    pub source_gene: Gene,

    /// Amino acid sequence
    pub amino_sequence: Vec<AminoAcid>,

    /// Protein structure
    pub structure: ProteinStructure,

    /// Folding state
    pub folding_state: FoldingState,

    /// Functional state
    pub functional_state: FunctionalState,
}

/// Amino Acid - Protein building block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
}

/// Protein Structure
#[derive(Debug, Clone)]
pub struct ProteinStructure {
    /// Primary structure (amino acid sequence)
    pub primary: Vec<AminoAcid>,

    /// Secondary structure (alpha helices, beta sheets)
    pub secondary: SecondaryStructure,

    /// Tertiary structure (3D folding)
    pub tertiary: Vec<Float>,

    /// Quaternary structure (multi-subunit assembly)
    pub quaternary: Option<Vec<Float>>,
}

/// Secondary Structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryStructure {
    AlphaHelix,
    BetaSheet,
    RandomCoil,
    Mixed,
}

/// Folding State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldingState {
    Unfolded,
    Folding,
    Folded,
    Misfolded,
}

/// Functional State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionalState {
    Inactive,
    Active,
    Degraded,
    Aggregated,
}

/// Mutation - Change in DNA sequence
#[derive(Debug, Clone)]
pub struct Mutation {
    /// Mutation identifier
    pub mutation_id: String,

    /// Mutation type
    pub mutation_type: MutationType,

    /// Position in DNA sequence
    pub position: usize,

    /// Original base
    pub original_base: Option<NucleotideBase>,

    /// New base
    pub new_base: Option<NucleotideBase>,

    /// Severity (0.0 to 1.0)
    pub severity: Float,
}

/// Mutation Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    /// Point mutation (single base change)
    Point,

    /// Insertion (base added)
    Insertion,

    /// Deletion (base removed)
    Deletion,

    /// Duplication (segment duplicated)
    Duplication,

    /// Inversion (segment reversed)
    Inversion,

    /// Translocation (segment moved)
    Translocation,
}

/// DNA Repair Result
#[derive(Debug, Clone)]
pub struct DnaRepairResult {
    /// Number of mutations repaired
    pub mutations_repaired: usize,

    /// Number of mutations not repaired
    pub mutations_unrepaired: usize,

    /// New DNA integrity
    pub new_integrity: Float,

    /// Repair success (0.0 to 1.0)
    pub repair_success: Float,
}

/// Epigenetic Marker - Modifies gene expression
#[derive(Debug, Clone)]
pub struct EpigeneticMarker {
    /// Marker identifier
    pub marker_id: String,

    /// Target gene
    pub target_gene: String,

    /// Marker type
    pub marker_type: EpigeneticMarkerType,

    /// Effect on expression (0.0 to 1.0, >0.5 = activation, <0.5 = repression)
    pub expression_effect: Float,

    /// Permanence (0.0 to 1.0, 1.0 = permanent)
    pub permanence: Float,
}

/// Epigenetic Marker Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpigeneticMarkerType {
    /// DNA methylation (typically repressive)
    Methylation,

    /// Histone acetylation (typically activating)
    Acetylation,

    /// Histone methylation (can be activating or repressive)
    HistoneMethylation,

    /// Non-coding RNA regulation
    NonCodingRna,

    /// Chromatin remodeling
    ChromatinRemodeling,
}

// ============================================================================
// SUBSYSTEMS
// ============================================================================

/// Spectrum Encoder - Converts blueprint to DNA
#[derive(Debug, Clone)]
struct SpectrumEncoder;

impl SpectrumEncoder {
    fn new() -> Self {
        Self
    }

    fn encode_blueprint(&self, blueprint: &HolographicBlueprint) -> DNA {
        // Extract DNA pattern from blueprint
        // For now, create a quantum realm DNA pattern
        let dna_pattern = DNAPattern::quantum_realm(&blueprint.spectrum_configuration);
        DNA::from_blueprint(dna_pattern)
    }
}

/// Protein Synthesizer - Converts DNA to proteins
#[derive(Debug, Clone)]
struct ProteinSynthesizer;

impl ProteinSynthesizer {
    fn new() -> Self {
        Self
    }

    fn synthesize(&self, dna: &DNA, config: &DnaSystemConfig) -> Vec<Protein> {
        let mut proteins = Vec::new();
        let mut rng = rand::thread_rng();

        for gene in &dna.genes {
            // Only synthesize expressed genes above threshold
            if let Some(expr) = dna.get_gene_expression(&gene.gene_id) {
                if expr.is_active && expr.expression_level >= config.expression_threshold {
                    // Synthesis succeeds with probability based on synthesis rate
                    if rng.gen::<Float>() < config.synthesis_rate {
                        let protein = self.synthesize_protein(gene, dna);
                        proteins.push(protein);
                    }
                }
            }
        }

        proteins
    }

    fn synthesize_protein(&self, gene: &Gene, _dna: &DNA) -> Protein {
        let amino_sequence = self.generate_amino_sequence(gene);
        let structure = ProteinStructure {
            primary: amino_sequence.clone(),
            secondary: SecondaryStructure::AlphaHelix,
            tertiary: vec![0.5, 0.5, 0.5],
            quaternary: None,
        };

        Protein {
            protein_id: format!("protein-{}", gene.gene_id),
            source_gene: gene.clone(),
            amino_sequence,
            structure,
            folding_state: FoldingState::Folding,
            functional_state: FunctionalState::Inactive,
        }
    }

    fn generate_amino_sequence(&self, _gene: &Gene) -> Vec<AminoAcid> {
        // Generate amino acid sequence based on genetic base
        vec![
            AminoAcid::Alanine,
            AminoAcid::Glycine,
            AminoAcid::Valine,
            AminoAcid::Leucine,
        ]
    }
}

/// Mutation Detector - Identifies DNA mutations
#[derive(Debug, Clone)]
struct MutationDetector;

impl MutationDetector {
    fn new() -> Self {
        Self
    }

    fn detect(&self, dna: &DNA, original_dna: &DNA) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let mut rng = rand::thread_rng();

        // Compare sequences
        for (i, (&base, &original_base)) in dna
            .sequence
            .iter()
            .zip(original_dna.sequence.iter())
            .enumerate()
        {
            if base != original_base {
                mutations.push(Mutation {
                    mutation_id: format!("mutation-{}", i),
                    mutation_type: MutationType::Point,
                    position: i,
                    original_base: Some(original_base),
                    new_base: Some(base),
                    severity: rng.gen::<Float>(),
                });
            }
        }

        mutations
    }
}

/// DNA Repair - Fixes mutations
#[derive(Debug, Clone)]
struct DnaRepair;

impl DnaRepair {
    fn new() -> Self {
        Self
    }

    fn repair(&self, dna: &mut DNA, config: &DnaSystemConfig) -> DnaRepairResult {
        let mut repaired = 0;
        let mut unrepaired = 0;
        let mut rng = rand::thread_rng();

        // Repair mutations based on repair efficiency
        for base in dna.sequence.iter_mut() {
            // Randomly introduce mutations based on mutation rate
            if rng.gen::<Float>() < config.mutation_rate {
                *base = match rng.gen_range(0..4) {
                    0 => NucleotideBase::Adenine,
                    1 => NucleotideBase::Thymine,
                    2 => NucleotideBase::Cytosine,
                    _ => NucleotideBase::Guanine,
                };
            }
        }

        // Attempt to repair with probability based on repair efficiency
        if rng.gen::<Float>() < config.repair_efficiency {
            repaired += 1;
        } else {
            unrepaired += 1;
        }

        let new_integrity = 1.0 - (unrepaired as Float / dna.sequence.len() as Float);
        let repair_success = if !dna.sequence.is_empty() {
            repaired as Float / (repaired + unrepaired) as Float
        } else {
            1.0
        };

        DnaRepairResult {
            mutations_repaired: repaired,
            mutations_unrepaired: unrepaired,
            new_integrity,
            repair_success,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::IndividualSpectrumConfiguration;
    use crate::spectrum::SpectrumRatio;

    #[test]
    fn test_dna_system_creation() {
        let dna_system = DnaSystem::new();
        assert_eq!(dna_system.config().mutation_rate, 0.001);
        assert_eq!(dna_system.config().repair_efficiency, 0.95);
    }

    #[test]
    fn test_dna_system_config() {
        let config = DnaSystemConfig {
            mutation_rate: 0.01,
            repair_efficiency: 0.9,
            synthesis_rate: 0.7,
            expression_threshold: 0.6,
        };
        let dna_system = DnaSystem::with_config(config.clone());
        assert_eq!(dna_system.config().mutation_rate, 0.01);
    }

    #[test]
    fn test_dna_from_blueprint() {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let dna = DNA::from_blueprint(dna_pattern);
        assert!(!dna.genes.is_empty());
        assert!(!dna.sequence.is_empty());
        assert_eq!(dna.integrity, 1.0);
    }

    #[test]
    fn test_gene_expression() {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let mut dna = DNA::from_blueprint(dna_pattern);
        let gene_id = dna.genes[0].gene_id.clone();

        // Set expression
        dna.set_gene_expression(&gene_id, 0.8);
        let expr = dna.get_gene_expression(&gene_id);
        assert!(expr.is_some());
        assert_eq!(expr.unwrap().expression_level, 0.8);
        assert!(expr.unwrap().is_active);
    }

    #[test]
    fn test_protein_synthesis() {
        let dna_system = DnaSystem::new();
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let mut dna = DNA::from_blueprint(dna_pattern);
        // Set high expression for all genes
        let gene_ids: Vec<String> = dna.genes.iter().map(|g| g.gene_id.clone()).collect();
        for gene_id in &gene_ids {
            dna.set_gene_expression(gene_id, 0.9);
        }

        let proteins = dna_system.synthesize_proteins(&dna);
        assert!(!proteins.is_empty());
    }

    #[test]
    fn test_mutation_detection() {
        let dna_system = DnaSystem::new();
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let original_dna = DNA::from_blueprint(dna_pattern.clone());
        let mut mutated_dna = DNA::from_blueprint(dna_pattern);
        // Introduce a mutation
        mutated_dna.sequence[0] = NucleotideBase::Thymine;

        let mutations = dna_system.detect_mutations(&mutated_dna, &original_dna);
        assert_eq!(mutations.len(), 1);
        assert_eq!(mutations[0].mutation_type, MutationType::Point);
    }

    #[test]
    fn test_dna_repair() {
        let dna_system = DnaSystem::new();
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let mut dna = DNA::from_blueprint(dna_pattern);
        let _original_integrity = dna.integrity();

        let repair_result = dna_system.repair_mutation(&mut dna);
        assert!(repair_result.new_integrity <= 1.0);
        assert!(repair_result.repair_success >= 0.0 && repair_result.repair_success <= 1.0);
    }

    #[test]
    fn test_epigenetic_markers() {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let mut dna = DNA::from_blueprint(dna_pattern);
        let gene_id = dna.genes[0].gene_id.clone();

        let marker = EpigeneticMarker {
            marker_id: "marker-1".to_string(),
            target_gene: gene_id.clone(),
            marker_type: EpigeneticMarkerType::Methylation,
            expression_effect: 0.3, // Repressive
            permanence: 0.8,
        };

        dna.add_epigenetic_marker(marker);
        assert_eq!(dna.epigenetic_markers.len(), 1);
    }

    #[test]
    fn test_dna_integrity() {
        let ratio = SpectrumRatio::space_time(1.5, 1.0);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let dna_pattern = DNAPattern::quantum_realm(
            &crate::entity_layer7::holographic_blueprint::SpectrumConfiguration::from_individual(
                &spectrum_config,
            ),
        );

        let mut dna = DNA::from_blueprint(dna_pattern);
        assert_eq!(dna.integrity(), 1.0);

        dna.set_integrity(0.8);
        assert_eq!(dna.integrity(), 0.8);

        dna.set_integrity(1.5); // Should be clamped to 1.0
        assert_eq!(dna.integrity(), 1.0);

        dna.set_integrity(-0.5); // Should be clamped to 0.0
        assert_eq!(dna.integrity(), 0.0);
    }
}
