//! Gene Encoding from 22-Archetype Activation Patterns
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The holographic blueprint for ALL physical existence is encoded BEFORE physical atoms exist."
//!
//! # Archetype-Gene Mapping
//!
//! | Archetype Group | Gene Category | Function |
//! |-----------------|---------------|----------|
//! | A1-A7 (Mind)    | Regulatory    | Control when/where genes express |
//! | A8-A14 (Body)   | Structural    | Encode physical proteins |
//! | A15-A21 (Spirit)| Epigenetic    | Modulate expression |
//! | A22 (Choice)    | Mutation      | Evolutionary operators |

use crate::holographic_foundation::archetype_profile::{
    ArchetypeActivationProfile, NUM_ARCHETYPES,
};
use crate::types::Float;
use std::collections::HashMap;

pub const GENE_ENCODING_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneId(pub u64);

impl GeneId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn from_archetype_pattern(pattern: &[Float; NUM_ARCHETYPES]) -> Self {
        let mut hash: u64 = 0;
        for (i, val) in pattern.iter().enumerate() {
            hash = hash.wrapping_add((*val).to_bits());
            hash = hash.wrapping_mul(31);
            hash ^= i as u64;
        }
        Self(hash)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneCategory {
    Regulatory,
    Structural,
    Epigenetic,
    Mutation,
}

impl GeneCategory {
    pub fn from_archetype_index(index: usize) -> Self {
        match index {
            0..=6 => GeneCategory::Regulatory,
            7..=13 => GeneCategory::Structural,
            14..=20 => GeneCategory::Epigenetic,
            21 => GeneCategory::Mutation,
            _ => GeneCategory::Structural,
        }
    }

    pub fn dominant_archetype_range(&self) -> std::ops::Range<usize> {
        match self {
            GeneCategory::Regulatory => 0..7,
            GeneCategory::Structural => 7..14,
            GeneCategory::Epigenetic => 14..21,
            GeneCategory::Mutation => 21..22,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            GeneCategory::Regulatory => "Control genes - when/where other genes express",
            GeneCategory::Structural => "Protein-coding genes - physical structure",
            GeneCategory::Epigenetic => "Modulation genes - expression control",
            GeneCategory::Mutation => "Evolution genes - recombination operators",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeGene {
    pub id: GeneId,
    pub archetype_source: usize,
    pub category: GeneCategory,
    pub encoding: [Float; GENE_ENCODING_SIZE],
    pub expression_threshold: Float,
    pub expression_strength: Float,
    pub regulatory_targets: Vec<GeneId>,
    pub protein_blueprint: Option<Vec<Float>>,
}

impl ArchetypeGene {
    pub fn from_archetype(archetype_index: usize, activation: Float) -> Self {
        let category = GeneCategory::from_archetype_index(archetype_index);
        let encoding = Self::generate_encoding(archetype_index, activation);

        Self {
            id: GeneId::new((archetype_index as u64) << 32 | activation.to_bits()),
            archetype_source: archetype_index,
            category,
            encoding,
            expression_threshold: 0.3 + activation * 0.4,
            expression_strength: activation,
            regulatory_targets: Vec::new(),
            protein_blueprint: if category == GeneCategory::Structural {
                Some(Self::generate_protein_blueprint(&encoding))
            } else {
                None
            },
        }
    }

    fn generate_encoding(archetype_index: usize, activation: Float) -> [Float; GENE_ENCODING_SIZE] {
        let mut encoding = [0.0; GENE_ENCODING_SIZE];

        for (i, item) in encoding.iter_mut().enumerate() {
            let phase = (i as Float / GENE_ENCODING_SIZE as Float) * std::f64::consts::TAU;
            let archetype_wave = ((archetype_index + 1) as Float * phase).sin();
            let activation_mod = activation * archetype_wave.cos();
            *item = 0.5 + 0.3 * archetype_wave + 0.2 * activation_mod;
        }

        encoding
    }

    fn generate_protein_blueprint(encoding: &[Float; GENE_ENCODING_SIZE]) -> Vec<Float> {
        let mut blueprint = Vec::with_capacity(256);

        for i in (0..GENE_ENCODING_SIZE).step_by(4) {
            if i + 3 < GENE_ENCODING_SIZE {
                let amino_code = encoding[i] * 0.25
                    + encoding[i + 1] * 0.25
                    + encoding[i + 2] * 0.25
                    + encoding[i + 3] * 0.25;
                blueprint.push(amino_code);
            }
        }

        blueprint
    }

    pub fn can_express(&self, field_resonance: Float) -> bool {
        field_resonance >= self.expression_threshold
    }

    pub fn add_regulatory_target(&mut self, target: GeneId) {
        if !self.regulatory_targets.contains(&target) {
            self.regulatory_targets.push(target);
        }
    }

    pub fn archetype_source(&self) -> usize {
        self.archetype_source
    }
}

#[derive(Debug, Clone)]
pub struct GeneExpressionProfile {
    pub coefficients: [Float; NUM_ARCHETYPES],
    pub regulatory_genes: Vec<ArchetypeGene>,
    pub structural_genes: Vec<ArchetypeGene>,
    pub epigenetic_genes: Vec<ArchetypeGene>,
    pub mutation_gene: Option<ArchetypeGene>,
}

impl Default for GeneExpressionProfile {
    fn default() -> Self {
        Self::from_profile(&ArchetypeActivationProfile::default())
    }
}

impl GeneExpressionProfile {
    pub fn from_profile(profile: &ArchetypeActivationProfile) -> Self {
        let coefficients = *profile.coefficients();
        let mut regulatory_genes = Vec::new();
        let mut structural_genes = Vec::new();
        let mut epigenetic_genes = Vec::new();
        let mut mutation_gene = None;

        for (i, &activation) in coefficients.iter().enumerate() {
            let gene = ArchetypeGene::from_archetype(i, activation);
            match gene.category {
                GeneCategory::Regulatory => regulatory_genes.push(gene),
                GeneCategory::Structural => structural_genes.push(gene),
                GeneCategory::Epigenetic => epigenetic_genes.push(gene),
                GeneCategory::Mutation => mutation_gene = Some(gene),
            }
        }

        Self {
            coefficients,
            regulatory_genes,
            structural_genes,
            epigenetic_genes,
            mutation_gene,
        }
    }

    pub fn gene_count(&self) -> usize {
        self.regulatory_genes.len()
            + self.structural_genes.len()
            + self.epigenetic_genes.len()
            + if self.mutation_gene.is_some() { 1 } else { 0 }
    }

    pub fn gene_by_category(&self, category: GeneCategory) -> &[ArchetypeGene] {
        match category {
            GeneCategory::Regulatory => &self.regulatory_genes,
            GeneCategory::Structural => &self.structural_genes,
            GeneCategory::Epigenetic => &self.epigenetic_genes,
            GeneCategory::Mutation => {
                static EMPTY: [ArchetypeGene; 0] = [];
                &EMPTY
            }
        }
    }

    pub fn total_expression_potential(&self) -> Float {
        let reg_sum: Float = self
            .regulatory_genes
            .iter()
            .map(|g| g.expression_strength)
            .sum();
        let struct_sum: Float = self
            .structural_genes
            .iter()
            .map(|g| g.expression_strength)
            .sum();
        let epi_sum: Float = self
            .epigenetic_genes
            .iter()
            .map(|g| g.expression_strength)
            .sum();
        let mut_sum = self
            .mutation_gene
            .as_ref()
            .map(|g| g.expression_strength)
            .unwrap_or(0.0);

        (reg_sum + struct_sum + epi_sum + mut_sum) / NUM_ARCHETYPES as Float
    }
}

#[derive(Debug, Clone)]
pub struct GeneRegulatoryNetwork {
    pub genes: HashMap<GeneId, ArchetypeGene>,
    connections: HashMap<GeneId, Vec<(GeneId, Float)>>,
    network_coherence: Float,
}

impl Default for GeneRegulatoryNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneRegulatoryNetwork {
    pub fn new() -> Self {
        Self {
            genes: HashMap::new(),
            connections: HashMap::new(),
            network_coherence: 0.5,
        }
    }

    /// Get all regulatory connections as a slice
    pub fn connections(&self) -> &HashMap<GeneId, Vec<(GeneId, Float)>> {
        &self.connections
    }

    pub fn from_profile(profile: &GeneExpressionProfile) -> Self {
        let mut network = Self::new();

        for gene in &profile.regulatory_genes {
            network.add_gene(gene.clone());
        }
        for gene in &profile.structural_genes {
            network.add_gene(gene.clone());
        }
        for gene in &profile.epigenetic_genes {
            network.add_gene(gene.clone());
        }
        if let Some(gene) = &profile.mutation_gene {
            network.add_gene(gene.clone());
        }

        network.build_regulatory_connections();
        network.calculate_network_coherence();

        network
    }

    pub fn add_gene(&mut self, gene: ArchetypeGene) {
        let id = gene.id;
        self.genes.insert(id, gene);
    }

    fn build_regulatory_connections(&mut self) {
        let gene_ids: Vec<GeneId> = self.genes.keys().cloned().collect();

        for regulator_id in gene_ids.iter() {
            if let Some(regulator) = self.genes.get(regulator_id) {
                if regulator.category == GeneCategory::Regulatory {
                    let mut targets = Vec::new();

                    for target_id in gene_ids.iter() {
                        if target_id != regulator_id {
                            if let Some(target) = self.genes.get(target_id) {
                                if target.category == GeneCategory::Structural {
                                    let connection_strength =
                                        self.calculate_connection_strength(regulator, target);
                                    if connection_strength > 0.1 {
                                        targets.push((*target_id, connection_strength));
                                    }
                                }
                            }
                        }
                    }

                    if !targets.is_empty() {
                        self.connections.insert(*regulator_id, targets);
                    }
                }
            }
        }
    }

    fn calculate_connection_strength(&self, gene1: &ArchetypeGene, gene2: &ArchetypeGene) -> Float {
        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for i in 0..GENE_ENCODING_SIZE {
            dot_product += gene1.encoding[i] * gene2.encoding[i];
            norm1 += gene1.encoding[i] * gene1.encoding[i];
            norm2 += gene2.encoding[i] * gene2.encoding[i];
        }

        if norm1 > 0.0 && norm2 > 0.0 {
            (dot_product / (norm1.sqrt() * norm2.sqrt())).max(0.0)
        } else {
            0.0
        }
    }

    fn calculate_network_coherence(&mut self) {
        let total_genes = self.genes.len();
        if total_genes == 0 {
            self.network_coherence = 0.5;
            return;
        }

        let total_connections: usize = self.connections.values().map(|v| v.len()).sum();
        let connection_density =
            total_connections as Float / (total_genes * total_genes).max(1) as Float;

        let avg_strength: Float = self
            .connections
            .values()
            .flat_map(|v| v.iter().map(|(_, s)| *s))
            .sum::<Float>()
            / total_connections.max(1) as Float;

        self.network_coherence = connection_density * 0.5 + avg_strength * 0.5;
    }

    pub fn coherence(&self) -> Float {
        self.network_coherence
    }

    pub fn gene_count(&self) -> usize {
        self.genes.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.values().map(|v| v.len()).sum()
    }

    pub fn get_gene(&self, id: &GeneId) -> Option<&ArchetypeGene> {
        self.genes.get(id)
    }

    pub fn get_regulated_genes(&self, regulator_id: &GeneId) -> Option<&[(GeneId, Float)]> {
        self.connections.get(regulator_id).map(|v| v.as_slice())
    }

    pub fn gene_for_archetype(&self, archetype_idx: usize) -> Option<&ArchetypeGene> {
        self.genes
            .values()
            .find(|g| g.archetype_source == archetype_idx)
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeGeneEncoder {
    profile: GeneExpressionProfile,
    network: GeneRegulatoryNetwork,
}

impl ArchetypeGeneEncoder {
    pub fn new(profile: &ArchetypeActivationProfile) -> Self {
        let profile = GeneExpressionProfile::from_profile(profile);
        let network = GeneRegulatoryNetwork::from_profile(&profile);
        Self { profile, network }
    }

    pub fn encode(&self) -> &GeneExpressionProfile {
        &self.profile
    }

    pub fn network(&self) -> &GeneRegulatoryNetwork {
        &self.network
    }

    pub fn gene_for_archetype(&self, archetype_index: usize) -> Option<&ArchetypeGene> {
        match GeneCategory::from_archetype_index(archetype_index) {
            GeneCategory::Regulatory => self
                .profile
                .regulatory_genes
                .iter()
                .find(|g| g.archetype_source == archetype_index),
            GeneCategory::Structural => self
                .profile
                .structural_genes
                .iter()
                .find(|g| g.archetype_source == archetype_index),
            GeneCategory::Epigenetic => self
                .profile
                .epigenetic_genes
                .iter()
                .find(|g| g.archetype_source == archetype_index),
            GeneCategory::Mutation => self.profile.mutation_gene.as_ref(),
        }
    }

    pub fn expression_potential(&self) -> Float {
        self.profile.total_expression_potential()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_id_creation() {
        let id = GeneId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_gene_category_from_archetype() {
        assert_eq!(
            GeneCategory::from_archetype_index(0),
            GeneCategory::Regulatory
        );
        assert_eq!(
            GeneCategory::from_archetype_index(6),
            GeneCategory::Regulatory
        );
        assert_eq!(
            GeneCategory::from_archetype_index(7),
            GeneCategory::Structural
        );
        assert_eq!(
            GeneCategory::from_archetype_index(13),
            GeneCategory::Structural
        );
        assert_eq!(
            GeneCategory::from_archetype_index(14),
            GeneCategory::Epigenetic
        );
        assert_eq!(
            GeneCategory::from_archetype_index(20),
            GeneCategory::Epigenetic
        );
        assert_eq!(
            GeneCategory::from_archetype_index(21),
            GeneCategory::Mutation
        );
    }

    #[test]
    fn test_archetype_gene_creation() {
        let gene = ArchetypeGene::from_archetype(3, 0.7);
        assert_eq!(gene.archetype_source, 3);
        assert_eq!(gene.category, GeneCategory::Regulatory);
        assert!(gene.expression_strength > 0.0);
    }

    #[test]
    fn test_gene_expression_threshold() {
        let gene = ArchetypeGene::from_archetype(0, 0.5);
        assert!(gene.can_express(0.5));
        assert!(!gene.can_express(0.1));
    }

    #[test]
    fn test_structural_gene_has_protein_blueprint() {
        let gene = ArchetypeGene::from_archetype(8, 0.5);
        assert!(gene.protein_blueprint.is_some());
    }

    #[test]
    fn test_regulatory_gene_no_protein_blueprint() {
        let gene = ArchetypeGene::from_archetype(0, 0.5);
        assert!(gene.protein_blueprint.is_none());
    }

    #[test]
    fn test_gene_expression_profile_creation() {
        let profile = ArchetypeActivationProfile::default();
        let gene_profile = GeneExpressionProfile::from_profile(&profile);
        assert_eq!(gene_profile.gene_count(), 22);
    }

    #[test]
    fn test_gene_expression_profile_category_counts() {
        let profile = ArchetypeActivationProfile::default();
        let gene_profile = GeneExpressionProfile::from_profile(&profile);
        assert_eq!(gene_profile.regulatory_genes.len(), 7);
        assert_eq!(gene_profile.structural_genes.len(), 7);
        assert_eq!(gene_profile.epigenetic_genes.len(), 7);
        assert!(gene_profile.mutation_gene.is_some());
    }

    #[test]
    fn test_gene_regulatory_network_creation() {
        let profile = ArchetypeActivationProfile::default();
        let gene_profile = GeneExpressionProfile::from_profile(&profile);
        let network = GeneRegulatoryNetwork::from_profile(&gene_profile);
        assert_eq!(network.gene_count(), 22);
    }

    #[test]
    fn test_archetype_gene_encoder() {
        let profile = ArchetypeActivationProfile::default();
        let encoder = ArchetypeGeneEncoder::new(&profile);
        assert!(encoder.expression_potential() > 0.0);
    }

    #[test]
    fn test_gene_by_category() {
        let profile = ArchetypeActivationProfile::default();
        let gene_profile = GeneExpressionProfile::from_profile(&profile);
        let regulatory = gene_profile.gene_by_category(GeneCategory::Regulatory);
        assert_eq!(regulatory.len(), 7);
    }

    #[test]
    fn test_regulatory_targets_added() {
        let mut gene = ArchetypeGene::from_archetype(0, 0.5);
        let target = GeneId::new(100);
        gene.add_regulatory_target(target);
        assert!(gene.regulatory_targets.contains(&target));
    }
}
