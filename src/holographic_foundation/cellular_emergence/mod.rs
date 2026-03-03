//! Phase 8: Cellular Biology from Blueprint
//!
//! From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 8:
//! "The holographic blueprint for ALL physical existence is encoded BEFORE physical atoms exist.
//!  DNA/RNA are not random evolutionary developments—they unfold from this pre-existing
//!  holographic blueprint encoded as spectrum configurations."
//!
//! # 22-Archetype → Gene Encoding Scheme
//!
//! - A1-A7 (Mind): Regulatory genes - control when/where other genes express
//! - A8-A14 (Body): Structural genes - encode proteins for physical structure
//! - A15-A21 (Spirit): Epigenetic control - modulate expression without changing sequence
//! - A22 (Choice): Mutation/recombination operators - drive evolutionary change
//!
//! # Key Components
//!
//! - `holographic_blueprint`: Master pattern for morphogenesis
//! - `archetype_genes`: Gene encoding from 22-archetype activation patterns
//! - `nucleotide_interference`: DNA sequences as field interference patterns
//! - `gene_expression`: Gene activation as field resonance
//! - `protein_field`: Protein structure as 3D field configuration
//! - `cell_manifestation`: Cells as field boundaries with membranes
//! - `simultaneous_emergence`: Cell + Gaia consciousness co-emergence

pub mod archetype_genes;
pub mod cell_manifestation;
pub mod gene_expression;
pub mod holographic_blueprint;
pub mod nucleotide_interference;
pub mod protein_field;
pub mod simultaneous_emergence;

pub use archetype_genes::{
    ArchetypeGene, ArchetypeGeneEncoder, GeneCategory, GeneExpressionProfile, GeneId,
    GeneRegulatoryNetwork,
};
pub use cell_manifestation::{
    CellBoundary, CellId, CellManifestation, CellMembrane, CellOrganelle, CellState,
    CellularFieldConfiguration,
};
pub use gene_expression::{
    ExpressionCondition, ExpressionResult, FieldResonanceExpression, GeneExpressionEngine,
    RegulatorySignal,
};
pub use holographic_blueprint::{
    BlueprintId, DevelopmentalStage, EpigeneticTrigger, HolographicBlueprint, OrganismManifestation,
};
pub use nucleotide_interference::{
    DNAHelix, DNAInterferencePattern, Nucleotide, NucleotideEncoding, NucleotideSequence,
};
pub use protein_field::{
    AminoAcid, AminoAcidField, ProteinFoldingField, ProteinId, ProteinManifestation,
    ProteinStructure,
};
pub use simultaneous_emergence::{
    CellularGaiaPair, CellularPlanetaryResonance, GaiaConsciousness, PlanetaryCellField,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _profile = GeneExpressionProfile::default();
        let _id = GeneId::new(1);
        assert!(true);
    }
}
