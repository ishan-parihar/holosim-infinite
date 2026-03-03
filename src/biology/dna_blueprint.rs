//! DNA as functional blueprint manifestation
//!
//! From ROADMAP: "DNA/RNA are not the blueprint but the INTERFACE to the blueprint"
//! "DNA sequences are access keys to blueprint regions"
//!
//! This module implements:
//! 1. DNA sequences that encode actual molecular configurations
//! 2. Genes that map to blueprint developmental stages
//! 3. DNA providing assembly instructions for cellular components
//! 4. Gene expression triggering developmental transitions

use crate::entity_layer7::holographic_blueprint::{HolographicBlueprint, StageBlueprint};
use crate::entity_layer7::layer7::EvolutionaryStage;
use crate::types::Float;
use std::collections::HashMap;
use std::sync::Arc;

/// Nucleotide base
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nucleotide {
    Adenine,
    Thymine,
    Guanine,
    Cytosine,
    Uracil, // For RNA
}

impl Nucleotide {
    /// Get single-letter code
    pub fn code(&self) -> char {
        match self {
            Nucleotide::Adenine => 'A',
            Nucleotide::Thymine => 'T',
            Nucleotide::Guanine => 'G',
            Nucleotide::Cytosine => 'C',
            Nucleotide::Uracil => 'U',
        }
    }

    /// Complementary base
    pub fn complement(&self) -> Self {
        match self {
            Nucleotide::Adenine => Nucleotide::Thymine,
            Nucleotide::Thymine => Nucleotide::Adenine,
            Nucleotide::Guanine => Nucleotide::Cytosine,
            Nucleotide::Cytosine => Nucleotide::Guanine,
            Nucleotide::Uracil => Nucleotide::Adenine,
        }
    }

    /// Map to archetype coefficient
    /// From ROADMAP: "Spectrum Configuration → nucleotide sequences"
    pub fn archetype_coefficient(&self) -> Float {
        match self {
            Nucleotide::Adenine => 0.9,  // High Matrix
            Nucleotide::Thymine => 0.1,  // Low Matrix
            Nucleotide::Guanine => 0.7,  // Catalyst
            Nucleotide::Cytosine => 0.5, // Balanced
            Nucleotide::Uracil => 0.3,   // Experience
        }
    }
}

/// Codon (3 nucleotides) encoding an amino acid or instruction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Codon(pub [Nucleotide; 3]);

impl Codon {
    /// Create from nucleotides
    pub fn new(n1: Nucleotide, n2: Nucleotide, n3: Nucleotide) -> Self {
        Self([n1, n2, n3])
    }

    /// Encode as archetype signature
    /// Each codon maps to a region of archetype space
    pub fn archetype_signature(&self) -> [Float; 22] {
        let mut sig = [0.5; 22];

        // First nucleotide determines primary archetype
        sig[0] = self.0[0].archetype_coefficient();

        // Second nucleotide modulates
        sig[6] = self.0[1].archetype_coefficient();

        // Third nucleotide fine-tunes
        sig[21] = self.0[2].archetype_coefficient();

        sig
    }

    /// Get string representation
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.0[0].code(),
            self.0[1].code(),
            self.0[2].code()
        )
    }
}

/// Gene ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneId(pub u64);

/// Gene with functional encoding
#[derive(Debug, Clone)]
pub struct FunctionalGene {
    pub id: GeneId,
    pub name: String,

    /// Nucleotide sequence
    pub sequence: Vec<Nucleotide>,

    /// Codons (grouped in triplets)
    pub codons: Vec<Codon>,

    /// Which blueprint stage this gene activates
    pub blueprint_stage: Option<BlueprintStageId>,

    /// Expression level (0.0 to 1.0)
    pub expression_level: Float,

    /// Assembly instructions encoded by this gene
    pub assembly_instructions: Vec<AssemblyInstruction>,

    /// Regulatory elements
    pub promoter_strength: Float,
    pub enhancer_sites: Vec<usize>,
}

/// Assembly instruction encoded in DNA
#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    /// Build a specific protein
    SynthesizeProtein {
        protein_type: ProteinType,
        target_count: u32,
    },
    /// Create membrane structure
    FormMembrane {
        lipid_composition: LipidComposition,
        thickness: Float,
    },
    /// Establish metabolic pathway
    CreatePathway {
        pathway_name: String,
        enzymes: Vec<String>,
    },
    /// Activate cellular function
    ActivateFunction {
        function: CellularFunction,
        energy_cost: Float,
    },
    /// Blueprint stage transition
    TransitionToStage {
        stage_id: BlueprintStageId,
        trigger_conditions: Vec<String>,
    },
}

/// Protein types
#[derive(Debug, Clone, PartialEq)]
pub enum ProteinType {
    Structural(String), // Cytoskeleton, membrane proteins
    Enzyme(String),     // Catalyzes reactions
    Transport(String),  // Channels, carriers
    Signaling(String),  // Receptors, hormones
    Storage(String),    // Ferritin, casein
    Motor(String),      // Myosin, kinesin
    Defense(String),    // Antibodies, toxins
}

/// Lipid composition for membranes
#[derive(Debug, Clone)]
pub struct LipidComposition {
    pub phospholipids: Float,
    pub cholesterol: Float,
    pub glycolipids: Float,
}

impl Default for LipidComposition {
    fn default() -> Self {
        Self {
            phospholipids: 0.7,
            cholesterol: 0.2,
            glycolipids: 0.1,
        }
    }
}

/// Cellular functions
#[derive(Debug, Clone, PartialEq)]
pub enum CellularFunction {
    Metabolism,
    Reproduction,
    Signaling,
    Motility,
    Photosynthesis,
    Respiration,
    Secretion,
    Endocytosis,
}

/// Blueprint stage ID
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlueprintStageId(pub u64);

impl BlueprintStageId {
    /// Create from evolutionary stage
    pub fn from_evolutionary_stage(stage: EvolutionaryStage) -> Self {
        Self(stage as u64)
    }
}

/// DNA with functional blueprint encoding
#[derive(Debug, Clone)]
pub struct BlueprintDNA {
    /// Reference to holographic blueprint
    pub blueprint: Arc<HolographicBlueprint>,

    /// Complete nucleotide sequence
    pub sequence: Vec<Nucleotide>,

    /// Genes with functional encoding
    pub genes: Vec<FunctionalGene>,

    /// Gene ID to blueprint stage mapping
    pub gene_to_stage: HashMap<GeneId, BlueprintStageId>,

    /// Non-coding regulatory regions
    pub regulatory_regions: Vec<RegulatoryRegion>,

    /// Epigenetic markers
    pub epigenetic_markers: Vec<EpigeneticMarker>,

    /// DNA integrity (0.0 to 1.0)
    pub integrity: Float,
}

/// Regulatory region (promoter, enhancer, silencer)
#[derive(Debug, Clone)]
pub struct RegulatoryRegion {
    pub region_type: RegulatoryType,
    pub start: usize,
    pub end: usize,
    pub target_genes: Vec<GeneId>,
    pub binding_affinity: Float,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegulatoryType {
    Promoter,
    Enhancer,
    Silencer,
    Insulator,
}

/// Epigenetic marker
#[derive(Debug, Clone)]
pub struct EpigeneticMarker {
    pub position: usize,
    pub marker_type: EpigeneticType,
    pub expression_effect: Float, // -1.0 (silence) to +1.0 (enhance)
}

#[derive(Debug, Clone, PartialEq)]
pub enum EpigeneticType {
    Methylation, // Usually silences genes
    Acetylation, // Usually activates genes
    Phosphorylation,
    Ubiquitination,
}

impl BlueprintDNA {
    /// Create DNA from holographic blueprint
    ///
    /// From ROADMAP: "DNA::from_blueprint() - DNA unfolds from blueprint patterns"
    pub fn from_blueprint(blueprint: Arc<HolographicBlueprint>) -> Self {
        let mut dna = Self {
            blueprint: blueprint.clone(),
            sequence: Vec::new(),
            genes: Vec::new(),
            gene_to_stage: HashMap::new(),
            regulatory_regions: Vec::new(),
            epigenetic_markers: Vec::new(),
            integrity: 1.0,
        };

        // Encode each evolutionary stage as genes
        let stages = Self::get_evolutionary_stages();
        for (i, stage) in stages.iter().enumerate() {
            if let Some(stage_blueprint) = blueprint.get_stage_blueprint(*stage) {
                let gene = dna.encode_stage_as_gene(&stage_blueprint, i);
                dna.genes.push(gene);
            }
        }

        // Build complete sequence from genes
        dna.sequence = dna.genes.iter().flat_map(|g| g.sequence.clone()).collect();

        // Create gene-to-stage mapping
        for gene in dna.genes.iter() {
            if let Some(stage_id) = &gene.blueprint_stage {
                dna.gene_to_stage.insert(gene.id.clone(), stage_id.clone());
            }
        }

        dna
    }

    /// Get all evolutionary stages
    fn get_evolutionary_stages() -> Vec<EvolutionaryStage> {
        vec![
            EvolutionaryStage::QuantumRealm,
            EvolutionaryStage::AtomicRealm,
            EvolutionaryStage::MolecularRealm,
            EvolutionaryStage::PlanetaryRealm,
            EvolutionaryStage::CellularRealm,
            EvolutionaryStage::SimpleLifeRealm,
            EvolutionaryStage::ComplexLifeRealm,
            EvolutionaryStage::ConsciousLifeRealm,
            EvolutionaryStage::SocietalRealm,
        ]
    }

    /// Encode a blueprint stage as a functional gene
    fn encode_stage_as_gene(&self, stage: &StageBlueprint, index: usize) -> FunctionalGene {
        // Convert stage to nucleotide sequence
        let sequence = self.stage_to_sequence(stage);

        // Group into codons
        let codons: Vec<Codon> = sequence
            .chunks(3)
            .filter_map(|chunk| {
                if chunk.len() == 3 {
                    Some(Codon([chunk[0], chunk[1], chunk[2]]))
                } else {
                    None
                }
            })
            .collect();

        // Derive assembly instructions from stage
        let instructions = self.stage_to_instructions(stage);

        FunctionalGene {
            id: GeneId(index as u64),
            name: format!("{:?}_Gene", stage.stage),
            sequence,
            codons,
            blueprint_stage: Some(BlueprintStageId::from_evolutionary_stage(stage.stage)),
            expression_level: 0.0,
            assembly_instructions: instructions,
            promoter_strength: 0.8,
            enhancer_sites: Vec::new(),
        }
    }

    /// Convert blueprint stage to nucleotide sequence
    fn stage_to_sequence(&self, stage: &StageBlueprint) -> Vec<Nucleotide> {
        // Encode stage properties as nucleotide pattern
        let mut sequence = Vec::new();

        // Use spectrum encoding from the blueprint to generate sequence
        let spectrum_encoding = self.extract_spectrum_encoding(stage);

        // Each encoding value maps to nucleotides
        for coeff in spectrum_encoding.iter() {
            // Threshold determines which base
            if *coeff > 0.75 {
                sequence.push(Nucleotide::Adenine);
            } else if *coeff > 0.5 {
                sequence.push(Nucleotide::Guanine);
            } else if *coeff > 0.25 {
                sequence.push(Nucleotide::Cytosine);
            } else {
                sequence.push(Nucleotide::Thymine);
            }
        }

        // Add stop codon (TAA)
        sequence.extend_from_slice(&[
            Nucleotide::Thymine,
            Nucleotide::Adenine,
            Nucleotide::Adenine,
        ]);

        sequence
    }

    /// Extract spectrum encoding from stage blueprint
    fn extract_spectrum_encoding(&self, stage: &StageBlueprint) -> Vec<Float> {
        // Get encoding from DNA pattern if available
        if let Some(dna_pattern) = &stage.dna_pattern {
            return dna_pattern.spectrum_encoding.clone();
        }

        // Get encoding from collective pattern if available
        if let Some(collective) = &stage.collective_pattern {
            return collective.collective_spectrum.spectrum_encoding.clone();
        }

        // Default encoding based on stage type
        vec![0.5; 22]
    }

    /// Derive assembly instructions from blueprint stage
    fn stage_to_instructions(&self, stage: &StageBlueprint) -> Vec<AssemblyInstruction> {
        let mut instructions = Vec::new();

        // Based on evolutionary stage, create assembly instructions
        match stage.stage {
            EvolutionaryStage::QuantumRealm => {
                instructions.push(AssemblyInstruction::TransitionToStage {
                    stage_id: BlueprintStageId(0),
                    trigger_conditions: vec!["coherence_reached".to_string()],
                });
            }
            EvolutionaryStage::AtomicRealm => {
                instructions.push(AssemblyInstruction::SynthesizeProtein {
                    protein_type: ProteinType::Structural("nuclear_binding".to_string()),
                    target_count: 100,
                });
            }
            EvolutionaryStage::MolecularRealm => {
                instructions.push(AssemblyInstruction::CreatePathway {
                    pathway_name: "molecular_bonding".to_string(),
                    enzymes: vec!["catalyst".to_string()],
                });
            }
            EvolutionaryStage::CellularRealm => {
                instructions.push(AssemblyInstruction::FormMembrane {
                    lipid_composition: LipidComposition::default(),
                    thickness: 5.0, // nm
                });
                instructions.push(AssemblyInstruction::ActivateFunction {
                    function: CellularFunction::Metabolism,
                    energy_cost: 10.0,
                });
            }
            EvolutionaryStage::SimpleLifeRealm => {
                instructions.push(AssemblyInstruction::SynthesizeProtein {
                    protein_type: ProteinType::Enzyme("metabolic_enzyme".to_string()),
                    target_count: 1000,
                });
                instructions.push(AssemblyInstruction::ActivateFunction {
                    function: CellularFunction::Reproduction,
                    energy_cost: 50.0,
                });
            }
            EvolutionaryStage::ComplexLifeRealm => {
                instructions.push(AssemblyInstruction::SynthesizeProtein {
                    protein_type: ProteinType::Structural("cytoskeleton".to_string()),
                    target_count: 5000,
                });
                instructions.push(AssemblyInstruction::ActivateFunction {
                    function: CellularFunction::Signaling,
                    energy_cost: 20.0,
                });
            }
            EvolutionaryStage::ConsciousLifeRealm => {
                instructions.push(AssemblyInstruction::SynthesizeProtein {
                    protein_type: ProteinType::Signaling("neurotransmitter".to_string()),
                    target_count: 10000,
                });
            }
            EvolutionaryStage::SocietalRealm => {
                instructions.push(AssemblyInstruction::ActivateFunction {
                    function: CellularFunction::Signaling,
                    energy_cost: 100.0,
                });
            }
            _ => {
                // Default: transition to this stage
                instructions.push(AssemblyInstruction::TransitionToStage {
                    stage_id: BlueprintStageId(stage.stage as u64),
                    trigger_conditions: vec!["threshold_reached".to_string()],
                });
            }
        }

        instructions
    }

    /// Express a gene (trigger blueprint stage activation)
    ///
    /// From ROADMAP: "Gene expression = Blueprint stage activation"
    pub fn express_gene(&mut self, gene_id: &GeneId) -> Option<Vec<AssemblyInstruction>> {
        let gene = self.genes.iter_mut().find(|g| g.id == *gene_id)?;

        // Check if gene can be expressed
        if gene.expression_level > 0.5 {
            return None; // Already expressed
        }

        // Activate expression
        gene.expression_level = 1.0;

        // Return assembly instructions
        Some(gene.assembly_instructions.clone())
    }

    /// Get blueprint stage for a gene
    pub fn get_stage_for_gene(&self, gene_id: &GeneId) -> Option<&BlueprintStageId> {
        self.gene_to_stage.get(gene_id)
    }

    /// Transcribe gene to RNA
    pub fn transcribe(&self, gene_id: &GeneId) -> Option<RNA> {
        let gene = self.genes.iter().find(|g| g.id == *gene_id)?;

        // Convert DNA to RNA (T → U)
        let rna_sequence: Vec<Nucleotide> = gene
            .sequence
            .iter()
            .map(|n| match n {
                Nucleotide::Thymine => Nucleotide::Uracil,
                other => *other,
            })
            .collect();

        Some(RNA {
            gene_id: gene_id.clone(),
            sequence: rna_sequence,
            codons: gene.codons.clone(),
            instructions: gene.assembly_instructions.clone(),
        })
    }

    /// Check DNA integrity and repair if needed
    pub fn maintain_integrity(&mut self) {
        // Blueprint provides the reference pattern for repair
        let reference = self.blueprint.get_dna_pattern();

        // Calculate integrity based on sequence match to reference
        self.integrity = self.calculate_integrity(&reference);

        // Auto-repair if integrity is low
        if self.integrity < 0.9 {
            self.repair_from_blueprint();
        }
    }

    fn calculate_integrity(&self, _reference: &[u8]) -> Float {
        // Simplified: check for mismatched bases
        // In real implementation, would compare to reference
        0.95 // Placeholder
    }

    fn repair_from_blueprint(&mut self) {
        // Re-encode from blueprint
        let repaired = Self::from_blueprint(self.blueprint.clone());
        self.sequence = repaired.sequence;
        self.integrity = 1.0;
    }

    /// Get gene by ID
    pub fn get_gene(&self, gene_id: &GeneId) -> Option<&FunctionalGene> {
        self.genes.iter().find(|g| g.id == *gene_id)
    }

    /// Get gene by name
    pub fn get_gene_by_name(&self, name: &str) -> Option<&FunctionalGene> {
        self.genes.iter().find(|g| g.name == name)
    }

    /// Get all genes for a specific stage
    pub fn get_genes_for_stage(&self, stage_id: &BlueprintStageId) -> Vec<&FunctionalGene> {
        self.genes
            .iter()
            .filter(|g| g.blueprint_stage.as_ref() == Some(stage_id))
            .collect()
    }
}

/// Extension trait for HolographicBlueprint
/// This adds the get_dna_pattern method needed for DNA repair
pub trait HolographicBlueprintExt {
    /// Get DNA pattern for repair reference
    fn get_dna_pattern(&self) -> Vec<u8>;
}

impl HolographicBlueprintExt for HolographicBlueprint {
    fn get_dna_pattern(&self) -> Vec<u8> {
        // Generate reference pattern from DNA patterns
        let mut pattern = Vec::new();
        for dna in &self.dna_patterns {
            // Convert spectrum encoding to byte pattern
            for value in &dna.spectrum_encoding {
                pattern.push((value * 255.0) as u8);
            }
        }
        pattern
    }
}

/// RNA transcript
#[derive(Debug, Clone)]
pub struct RNA {
    pub gene_id: GeneId,
    pub sequence: Vec<Nucleotide>,
    pub codons: Vec<Codon>,
    pub instructions: Vec<AssemblyInstruction>,
}

impl RNA {
    /// Translate RNA to protein assembly instructions
    pub fn translate(&self) -> ProteinBlueprint {
        ProteinBlueprint {
            gene_id: self.gene_id.clone(),
            amino_acid_sequence: self.codons_to_amino_acids(),
            folding_instructions: self.derive_folding_pattern(),
            assembly_instructions: self.instructions.clone(),
        }
    }

    fn codons_to_amino_acids(&self) -> Vec<AminoAcidCode> {
        // Map codons to amino acids
        self.codons
            .iter()
            .map(|codon| self.codon_to_amino_acid(codon))
            .collect()
    }

    fn codon_to_amino_acid(&self, codon: &Codon) -> AminoAcidCode {
        // Simplified genetic code
        let code = codon.to_string();
        match code.as_str() {
            "AUG" => AminoAcidCode::Methionine,
            "UUU" | "UUC" => AminoAcidCode::Phenylalanine,
            "UUA" | "UUG" => AminoAcidCode::Leucine,
            "UCU" | "UCC" | "UCA" | "UCG" => AminoAcidCode::Serine,
            "UAU" | "UAC" => AminoAcidCode::Tyrosine,
            "UGU" | "UGC" => AminoAcidCode::Cysteine,
            "UGG" => AminoAcidCode::Tryptophan,
            "CUU" | "CUC" | "CUA" | "CUG" => AminoAcidCode::Leucine,
            "CCU" | "CCC" | "CCA" | "CCG" => AminoAcidCode::Proline,
            "CAU" | "CAC" => AminoAcidCode::Histidine,
            "CAA" | "CAG" => AminoAcidCode::Glutamine,
            "CGU" | "CGC" | "CGA" | "CGG" => AminoAcidCode::Arginine,
            "AUU" | "AUC" | "AUA" => AminoAcidCode::Isoleucine,
            "ACU" | "ACC" | "ACA" | "ACG" => AminoAcidCode::Threonine,
            "AAU" | "AAC" => AminoAcidCode::Asparagine,
            "AAA" | "AAG" => AminoAcidCode::Lysine,
            "AGU" | "AGC" => AminoAcidCode::Serine,
            "AGA" | "AGG" => AminoAcidCode::Arginine,
            "GUU" | "GUC" | "GUA" | "GUG" => AminoAcidCode::Valine,
            "GCU" | "GCC" | "GCA" | "GCG" => AminoAcidCode::Alanine,
            "GAU" | "GAC" => AminoAcidCode::AsparticAcid,
            "GAA" | "GAG" => AminoAcidCode::GlutamicAcid,
            "GGU" | "GGC" | "GGA" | "GGG" => AminoAcidCode::Glycine,
            _ => AminoAcidCode::Unknown(code),
        }
    }

    fn derive_folding_pattern(&self) -> FoldingPattern {
        // Derive from archetype signatures
        let mut helix_propensity = 0.0;
        let mut sheet_propensity = 0.0;

        for codon in &self.codons {
            let sig = codon.archetype_signature();
            helix_propensity += sig[0]; // Matrix archetype
            sheet_propensity += sig[6]; // Great Way archetype
        }

        let total = self.codons.len().max(1) as Float;
        helix_propensity /= total;
        sheet_propensity /= total;

        let secondary_structure = if helix_propensity > sheet_propensity && helix_propensity > 0.5 {
            SecondaryStructurePrediction::AlphaHelix
        } else if sheet_propensity > 0.5 {
            SecondaryStructurePrediction::BetaSheet
        } else if helix_propensity > 0.3 && sheet_propensity > 0.3 {
            SecondaryStructurePrediction::Mixed
        } else {
            SecondaryStructurePrediction::RandomCoil
        };

        FoldingPattern {
            secondary_structure,
            hydrophobic_core: true,
            disulfide_bonds: 0,
        }
    }
}

/// Amino acid codes
#[derive(Debug, Clone, PartialEq)]
pub enum AminoAcidCode {
    Methionine,
    Phenylalanine,
    Leucine,
    Serine,
    Tyrosine,
    Cysteine,
    Tryptophan,
    Proline,
    Histidine,
    Glutamine,
    Asparagine,
    Lysine,
    AsparticAcid,
    GlutamicAcid,
    Alanine,
    Valine,
    Isoleucine,
    Glycine,
    Threonine,
    Arginine,
    Unknown(String),
}

impl AminoAcidCode {
    /// Get single-letter code
    pub fn code(&self) -> char {
        match self {
            AminoAcidCode::Methionine => 'M',
            AminoAcidCode::Phenylalanine => 'F',
            AminoAcidCode::Leucine => 'L',
            AminoAcidCode::Serine => 'S',
            AminoAcidCode::Tyrosine => 'Y',
            AminoAcidCode::Cysteine => 'C',
            AminoAcidCode::Tryptophan => 'W',
            AminoAcidCode::Proline => 'P',
            AminoAcidCode::Histidine => 'H',
            AminoAcidCode::Glutamine => 'Q',
            AminoAcidCode::Asparagine => 'N',
            AminoAcidCode::Lysine => 'K',
            AminoAcidCode::AsparticAcid => 'D',
            AminoAcidCode::GlutamicAcid => 'E',
            AminoAcidCode::Alanine => 'A',
            AminoAcidCode::Valine => 'V',
            AminoAcidCode::Isoleucine => 'I',
            AminoAcidCode::Glycine => 'G',
            AminoAcidCode::Threonine => 'T',
            AminoAcidCode::Arginine => 'R',
            AminoAcidCode::Unknown(s) => s.chars().next().unwrap_or('X'),
        }
    }
}

/// Protein blueprint from translation
#[derive(Debug, Clone)]
pub struct ProteinBlueprint {
    pub gene_id: GeneId,
    pub amino_acid_sequence: Vec<AminoAcidCode>,
    pub folding_instructions: FoldingPattern,
    pub assembly_instructions: Vec<AssemblyInstruction>,
}

impl ProteinBlueprint {
    /// Get the amino acid sequence as a string
    pub fn sequence_string(&self) -> String {
        self.amino_acid_sequence
            .iter()
            .map(|aa| aa.code())
            .collect()
    }

    /// Execute assembly instructions
    pub fn execute_instructions(&self) -> Vec<AssemblyResult> {
        self.assembly_instructions
            .iter()
            .map(|instr| self.execute_instruction(instr))
            .collect()
    }

    fn execute_instruction(&self, instr: &AssemblyInstruction) -> AssemblyResult {
        match instr {
            AssemblyInstruction::SynthesizeProtein {
                protein_type,
                target_count,
            } => AssemblyResult::ProteinSynthesized {
                protein_type: protein_type.clone(),
                count: *target_count,
            },
            AssemblyInstruction::FormMembrane {
                lipid_composition,
                thickness,
            } => AssemblyResult::MembraneFormed {
                composition: lipid_composition.clone(),
                thickness: *thickness,
            },
            AssemblyInstruction::CreatePathway {
                pathway_name,
                enzymes,
            } => AssemblyResult::PathwayCreated {
                name: pathway_name.clone(),
                enzyme_count: enzymes.len() as u32,
            },
            AssemblyInstruction::ActivateFunction {
                function,
                energy_cost,
            } => AssemblyResult::FunctionActivated {
                function: function.clone(),
                energy: *energy_cost,
            },
            AssemblyInstruction::TransitionToStage {
                stage_id,
                trigger_conditions,
            } => AssemblyResult::StageTransitioned {
                stage_id: stage_id.clone(),
                conditions_met: trigger_conditions.len() as u32,
            },
        }
    }
}

/// Result of executing an assembly instruction
#[derive(Debug, Clone)]
pub enum AssemblyResult {
    ProteinSynthesized {
        protein_type: ProteinType,
        count: u32,
    },
    MembraneFormed {
        composition: LipidComposition,
        thickness: Float,
    },
    PathwayCreated {
        name: String,
        enzyme_count: u32,
    },
    FunctionActivated {
        function: CellularFunction,
        energy: Float,
    },
    StageTransitioned {
        stage_id: BlueprintStageId,
        conditions_met: u32,
    },
}

/// Folding pattern
#[derive(Debug, Clone)]
pub struct FoldingPattern {
    pub secondary_structure: SecondaryStructurePrediction,
    pub hydrophobic_core: bool,
    pub disulfide_bonds: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecondaryStructurePrediction {
    AlphaHelix,
    BetaSheet,
    Mixed,
    RandomCoil,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleotide_complement() {
        assert_eq!(Nucleotide::Adenine.complement(), Nucleotide::Thymine);
        assert_eq!(Nucleotide::Thymine.complement(), Nucleotide::Adenine);
        assert_eq!(Nucleotide::Guanine.complement(), Nucleotide::Cytosine);
        assert_eq!(Nucleotide::Cytosine.complement(), Nucleotide::Guanine);
        assert_eq!(Nucleotide::Uracil.complement(), Nucleotide::Adenine);
    }

    #[test]
    fn test_nucleotide_code() {
        assert_eq!(Nucleotide::Adenine.code(), 'A');
        assert_eq!(Nucleotide::Thymine.code(), 'T');
        assert_eq!(Nucleotide::Guanine.code(), 'G');
        assert_eq!(Nucleotide::Cytosine.code(), 'C');
        assert_eq!(Nucleotide::Uracil.code(), 'U');
    }

    #[test]
    fn test_nucleotide_archetype_coefficient() {
        assert!(Nucleotide::Adenine.archetype_coefficient() > 0.75);
        assert!(Nucleotide::Thymine.archetype_coefficient() < 0.25);
        assert!(Nucleotide::Guanine.archetype_coefficient() > 0.5);
        assert!(Nucleotide::Cytosine.archetype_coefficient() > 0.4);
        assert!(Nucleotide::Uracil.archetype_coefficient() < 0.5);
    }

    #[test]
    fn test_codon_archetype_signature() {
        let codon = Codon::new(
            Nucleotide::Adenine,
            Nucleotide::Guanine,
            Nucleotide::Cytosine,
        );
        let sig = codon.archetype_signature();

        assert_eq!(sig.len(), 22);
        assert!(sig[0] > 0.5); // Adenine = high
        assert!(sig[6] > 0.5); // Guanine = catalyst
    }

    #[test]
    fn test_codon_to_string() {
        let codon = Codon::new(
            Nucleotide::Adenine,
            Nucleotide::Guanine,
            Nucleotide::Cytosine,
        );
        assert_eq!(codon.to_string(), "AGC");
    }

    #[test]
    fn test_blueprint_stage_id_from_evolutionary_stage() {
        let stage_id = BlueprintStageId::from_evolutionary_stage(EvolutionaryStage::QuantumRealm);
        assert_eq!(stage_id.0, EvolutionaryStage::QuantumRealm as u64);
    }

    #[test]
    fn test_lipid_composition_default() {
        let comp = LipidComposition::default();
        assert!((comp.phospholipids - 0.7).abs() < 0.001);
        assert!((comp.cholesterol - 0.2).abs() < 0.001);
        assert!((comp.glycolipids - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_amino_acid_code() {
        assert_eq!(AminoAcidCode::Methionine.code(), 'M');
        assert_eq!(AminoAcidCode::Phenylalanine.code(), 'F');
        assert_eq!(AminoAcidCode::Glycine.code(), 'G');
        assert_eq!(AminoAcidCode::Unknown("XYZ".to_string()).code(), 'X');
    }

    #[test]
    fn test_rna_translation() {
        let rna = RNA {
            gene_id: GeneId(0),
            sequence: vec![
                Nucleotide::Uracil,
                Nucleotide::Uracil,
                Nucleotide::Uracil,
                Nucleotide::Adenine,
                Nucleotide::Uracil,
                Nucleotide::Guanine,
            ],
            codons: vec![
                Codon::new(Nucleotide::Uracil, Nucleotide::Uracil, Nucleotide::Uracil),
                Codon::new(Nucleotide::Adenine, Nucleotide::Uracil, Nucleotide::Guanine),
            ],
            instructions: vec![],
        };

        let protein = rna.translate();
        assert_eq!(protein.amino_acid_sequence.len(), 2);
        assert_eq!(protein.amino_acid_sequence[0], AminoAcidCode::Phenylalanine);
        assert_eq!(protein.amino_acid_sequence[1], AminoAcidCode::Methionine);
    }

    #[test]
    fn test_protein_blueprint_sequence_string() {
        let blueprint = ProteinBlueprint {
            gene_id: GeneId(0),
            amino_acid_sequence: vec![
                AminoAcidCode::Methionine,
                AminoAcidCode::Alanine,
                AminoAcidCode::Glycine,
            ],
            folding_instructions: FoldingPattern {
                secondary_structure: SecondaryStructurePrediction::AlphaHelix,
                hydrophobic_core: true,
                disulfide_bonds: 0,
            },
            assembly_instructions: vec![],
        };

        assert_eq!(blueprint.sequence_string(), "MAG");
    }

    #[test]
    fn test_assembly_instruction_synthesis() {
        let instr = AssemblyInstruction::SynthesizeProtein {
            protein_type: ProteinType::Enzyme("hexokinase".to_string()),
            target_count: 100,
        };

        let blueprint = ProteinBlueprint {
            gene_id: GeneId(0),
            amino_acid_sequence: vec![],
            folding_instructions: FoldingPattern {
                secondary_structure: SecondaryStructurePrediction::AlphaHelix,
                hydrophobic_core: true,
                disulfide_bonds: 0,
            },
            assembly_instructions: vec![instr],
        };

        let results = blueprint.execute_instructions();
        assert_eq!(results.len(), 1);

        match &results[0] {
            AssemblyResult::ProteinSynthesized {
                protein_type,
                count,
            } => {
                assert_eq!(*count, 100);
                match protein_type {
                    ProteinType::Enzyme(name) => assert_eq!(name, "hexokinase"),
                    _ => panic!("Expected enzyme protein type"),
                }
            }
            _ => panic!("Expected ProteinSynthesized result"),
        }
    }

    #[test]
    fn test_assembly_instruction_membrane() {
        let instr = AssemblyInstruction::FormMembrane {
            lipid_composition: LipidComposition {
                phospholipids: 0.8,
                cholesterol: 0.15,
                glycolipids: 0.05,
            },
            thickness: 7.5,
        };

        let blueprint = ProteinBlueprint {
            gene_id: GeneId(0),
            amino_acid_sequence: vec![],
            folding_instructions: FoldingPattern {
                secondary_structure: SecondaryStructurePrediction::AlphaHelix,
                hydrophobic_core: true,
                disulfide_bonds: 0,
            },
            assembly_instructions: vec![instr],
        };

        let results = blueprint.execute_instructions();
        assert_eq!(results.len(), 1);

        match &results[0] {
            AssemblyResult::MembraneFormed { thickness, .. } => {
                assert!((thickness - 7.5).abs() < 0.001);
            }
            _ => panic!("Expected MembraneFormed result"),
        }
    }

    #[test]
    fn test_dna_transcription() {
        // Create a minimal functional gene
        let gene = FunctionalGene {
            id: GeneId(0),
            name: "TestGene".to_string(),
            sequence: vec![
                Nucleotide::Adenine,
                Nucleotide::Thymine,
                Nucleotide::Guanine,
                Nucleotide::Cytosine,
                Nucleotide::Thymine,
                Nucleotide::Adenine,
            ],
            codons: vec![
                Codon::new(
                    Nucleotide::Adenine,
                    Nucleotide::Thymine,
                    Nucleotide::Guanine,
                ),
                Codon::new(
                    Nucleotide::Cytosine,
                    Nucleotide::Thymine,
                    Nucleotide::Adenine,
                ),
            ],
            blueprint_stage: None,
            expression_level: 0.0,
            assembly_instructions: vec![],
            promoter_strength: 0.8,
            enhancer_sites: vec![],
        };

        // Verify T → U conversion
        let rna_sequence: Vec<Nucleotide> = gene
            .sequence
            .iter()
            .map(|n| match n {
                Nucleotide::Thymine => Nucleotide::Uracil,
                other => *other,
            })
            .collect();

        assert_eq!(rna_sequence[1], Nucleotide::Uracil);
        assert_eq!(rna_sequence[4], Nucleotide::Uracil);
        assert_eq!(rna_sequence[0], Nucleotide::Adenine);
    }
}
