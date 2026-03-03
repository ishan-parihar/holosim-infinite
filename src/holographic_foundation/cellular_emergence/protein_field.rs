//! Protein Structure as 3D Field Configuration
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Protein structure as 3D field configuration - proteins fold into
//!  field interference minima, not arbitrary shapes."
//!
//! # Key Insight
//!
//! Protein structure is determined by field dynamics:
//! - Primary structure (amino acid sequence) determines field interference pattern
//! - Secondary structure (alpha helix, beta sheet) forms at field minima
//! - Tertiary structure is the global field interference minimum
//! - Quaternary structure is multi-protein field coherence

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::atomic_emergence::ElementAttractorField;
use crate::holographic_foundation::field_state::Position3D;
use crate::types::Float;
use std::collections::HashMap;

pub const NUM_AMINO_ACIDS: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl AminoAcid {
    pub fn from_index(idx: u8) -> Option<Self> {
        match idx {
            1 => Some(AminoAcid::Alanine),
            2 => Some(AminoAcid::Arginine),
            3 => Some(AminoAcid::Asparagine),
            4 => Some(AminoAcid::AsparticAcid),
            5 => Some(AminoAcid::Cysteine),
            6 => Some(AminoAcid::GlutamicAcid),
            7 => Some(AminoAcid::Glutamine),
            8 => Some(AminoAcid::Glycine),
            9 => Some(AminoAcid::Histidine),
            10 => Some(AminoAcid::Isoleucine),
            11 => Some(AminoAcid::Leucine),
            12 => Some(AminoAcid::Lysine),
            13 => Some(AminoAcid::Methionine),
            14 => Some(AminoAcid::Phenylalanine),
            15 => Some(AminoAcid::Proline),
            16 => Some(AminoAcid::Serine),
            17 => Some(AminoAcid::Threonine),
            18 => Some(AminoAcid::Tryptophan),
            19 => Some(AminoAcid::Tyrosine),
            20 => Some(AminoAcid::Valine),
            _ => None,
        }
    }

    pub fn to_index(&self) -> u8 {
        match self {
            AminoAcid::Alanine => 1,
            AminoAcid::Arginine => 2,
            AminoAcid::Asparagine => 3,
            AminoAcid::AsparticAcid => 4,
            AminoAcid::Cysteine => 5,
            AminoAcid::GlutamicAcid => 6,
            AminoAcid::Glutamine => 7,
            AminoAcid::Glycine => 8,
            AminoAcid::Histidine => 9,
            AminoAcid::Isoleucine => 10,
            AminoAcid::Leucine => 11,
            AminoAcid::Lysine => 12,
            AminoAcid::Methionine => 13,
            AminoAcid::Phenylalanine => 14,
            AminoAcid::Proline => 15,
            AminoAcid::Serine => 16,
            AminoAcid::Threonine => 17,
            AminoAcid::Tryptophan => 18,
            AminoAcid::Tyrosine => 19,
            AminoAcid::Valine => 20,
        }
    }

    pub fn three_letter_code(&self) -> &'static str {
        match self {
            AminoAcid::Alanine => "Ala",
            AminoAcid::Arginine => "Arg",
            AminoAcid::Asparagine => "Asn",
            AminoAcid::AsparticAcid => "Asp",
            AminoAcid::Cysteine => "Cys",
            AminoAcid::GlutamicAcid => "Glu",
            AminoAcid::Glutamine => "Gln",
            AminoAcid::Glycine => "Gly",
            AminoAcid::Histidine => "His",
            AminoAcid::Isoleucine => "Ile",
            AminoAcid::Leucine => "Leu",
            AminoAcid::Lysine => "Lys",
            AminoAcid::Methionine => "Met",
            AminoAcid::Phenylalanine => "Phe",
            AminoAcid::Proline => "Pro",
            AminoAcid::Serine => "Ser",
            AminoAcid::Threonine => "Thr",
            AminoAcid::Tryptophan => "Trp",
            AminoAcid::Tyrosine => "Tyr",
            AminoAcid::Valine => "Val",
        }
    }

    pub fn one_letter_code(&self) -> char {
        match self {
            AminoAcid::Alanine => 'A',
            AminoAcid::Arginine => 'R',
            AminoAcid::Asparagine => 'N',
            AminoAcid::AsparticAcid => 'D',
            AminoAcid::Cysteine => 'C',
            AminoAcid::GlutamicAcid => 'E',
            AminoAcid::Glutamine => 'Q',
            AminoAcid::Glycine => 'G',
            AminoAcid::Histidine => 'H',
            AminoAcid::Isoleucine => 'I',
            AminoAcid::Leucine => 'L',
            AminoAcid::Lysine => 'K',
            AminoAcid::Methionine => 'M',
            AminoAcid::Phenylalanine => 'F',
            AminoAcid::Proline => 'P',
            AminoAcid::Serine => 'S',
            AminoAcid::Threonine => 'T',
            AminoAcid::Tryptophan => 'W',
            AminoAcid::Tyrosine => 'Y',
            AminoAcid::Valine => 'V',
        }
    }

    pub fn hydrophobicity(&self) -> Float {
        match self {
            AminoAcid::Alanine => 1.8,
            AminoAcid::Arginine => -4.5,
            AminoAcid::Asparagine => -3.5,
            AminoAcid::AsparticAcid => -3.5,
            AminoAcid::Cysteine => 2.5,
            AminoAcid::GlutamicAcid => -3.5,
            AminoAcid::Glutamine => -3.5,
            AminoAcid::Glycine => -0.4,
            AminoAcid::Histidine => -3.2,
            AminoAcid::Isoleucine => 4.5,
            AminoAcid::Leucine => 3.8,
            AminoAcid::Lysine => -3.9,
            AminoAcid::Methionine => 1.9,
            AminoAcid::Phenylalanine => 2.8,
            AminoAcid::Proline => -1.6,
            AminoAcid::Serine => -0.8,
            AminoAcid::Threonine => -0.7,
            AminoAcid::Tryptophan => -0.9,
            AminoAcid::Tyrosine => -1.3,
            AminoAcid::Valine => 4.2,
        }
    }

    pub fn archetype_affinity(&self) -> [Float; NUM_ARCHETYPES] {
        let mut affinity = [0.5; NUM_ARCHETYPES];
        let hydro = self.hydrophobicity();
        let idx = self.to_index() as usize;

        affinity[0] = 0.5 + (idx as Float / 20.0 - 0.5) * 0.3;
        affinity[7] = 0.5 + (hydro / 5.0).max(-0.3).min(0.3);
        affinity[14] = 0.5 + ((idx % 7) as Float / 7.0 - 0.5) * 0.4;

        affinity
    }

    pub fn is_hydrophobic(&self) -> bool {
        self.hydrophobicity() > 0.0
    }

    pub fn is_polar(&self) -> bool {
        self.hydrophobicity() <= 0.0 && !self.is_charged()
    }

    pub fn is_charged(&self) -> bool {
        matches!(
            self,
            AminoAcid::Arginine
                | AminoAcid::Lysine
                | AminoAcid::AsparticAcid
                | AminoAcid::GlutamicAcid
        )
    }
}

#[derive(Debug, Clone)]
pub struct AminoAcidField {
    amino_acid: AminoAcid,
    position: Position3D,
    archetype_affinity: [Float; NUM_ARCHETYPES],
    local_field_strength: Float,
}

impl AminoAcidField {
    pub fn new(amino_acid: AminoAcid, position: Position3D) -> Self {
        Self {
            amino_acid,
            position,
            archetype_affinity: amino_acid.archetype_affinity(),
            local_field_strength: 0.5,
        }
    }

    pub fn from_sequence_index(idx: usize, position: Position3D) -> Self {
        let amino = AminoAcid::from_index(((idx % 20) + 1) as u8).unwrap_or(AminoAcid::Alanine);
        Self::new(amino, position)
    }

    pub fn amino_acid(&self) -> AminoAcid {
        self.amino_acid
    }

    pub fn position(&self) -> &Position3D {
        &self.position
    }

    pub fn archetype_affinity(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_affinity
    }

    pub fn hydrophobicity(&self) -> Float {
        self.amino_acid.hydrophobicity()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecondaryStructure {
    AlphaHelix,
    BetaSheet,
    BetaTurn,
    RandomCoil,
    Unknown,
}

impl SecondaryStructure {
    pub fn from_archetype_pattern(pattern: &[Float; NUM_ARCHETYPES]) -> Self {
        let mind_sum: Float = pattern[0..7].iter().sum();
        let body_sum: Float = pattern[7..14].iter().sum();

        if mind_sum > body_sum * 1.2 {
            SecondaryStructure::AlphaHelix
        } else if body_sum > mind_sum * 1.2 {
            SecondaryStructure::BetaSheet
        } else if (mind_sum - body_sum).abs() < 0.5 {
            SecondaryStructure::BetaTurn
        } else {
            SecondaryStructure::RandomCoil
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProteinStructure {
    amino_acids: Vec<AminoAcidField>,
    secondary_structures: Vec<(usize, usize, SecondaryStructure)>,
    tertiary_contacts: Vec<(usize, usize, Float)>,
    total_field_energy: Float,
}

impl ProteinStructure {
    pub fn from_sequence(sequence: &[AminoAcid]) -> Self {
        let amino_acids: Vec<AminoAcidField> = sequence
            .iter()
            .enumerate()
            .map(|(i, aa)| {
                let z = i as Float * 3.8;
                AminoAcidField::new(*aa, Position3D::new(0.0, 0.0, z))
            })
            .collect();

        let mut structure = Self {
            amino_acids,
            secondary_structures: Vec::new(),
            tertiary_contacts: Vec::new(),
            total_field_energy: 0.0,
        };

        structure.predict_secondary_structure();
        structure.calculate_tertiary_contacts();
        structure.calculate_field_energy();

        structure
    }

    fn predict_secondary_structure(&mut self) {
        let mut i = 0;
        while i < self.amino_acids.len() {
            let window_end = (i + 8).min(self.amino_acids.len());

            if window_end - i >= 4 {
                let mut avg_pattern = [0.0; NUM_ARCHETYPES];
                for j in i..window_end {
                    let aff = self.amino_acids[j].archetype_affinity();
                    for k in 0..NUM_ARCHETYPES {
                        avg_pattern[k] += aff[k];
                    }
                }
                for k in 0..NUM_ARCHETYPES {
                    avg_pattern[k] /= (window_end - i) as Float;
                }

                let ss = SecondaryStructure::from_archetype_pattern(&avg_pattern);
                self.secondary_structures.push((i, window_end - 1, ss));
            }

            i += 4;
        }
    }

    fn calculate_tertiary_contacts(&mut self) {
        for i in 0..self.amino_acids.len() {
            for j in (i + 4)..self.amino_acids.len() {
                let pos_i = self.amino_acids[i].position();
                let pos_j = self.amino_acids[j].position();

                let distance = pos_i.distance(pos_j);
                if distance < 8.0 {
                    let aff_i = self.amino_acids[i].archetype_affinity();
                    let aff_j = self.amino_acids[j].archetype_affinity();

                    let mut interaction = 0.0;
                    for k in 0..NUM_ARCHETYPES {
                        interaction += aff_i[k] * aff_j[k];
                    }
                    interaction /= NUM_ARCHETYPES as Float;

                    self.tertiary_contacts.push((i, j, interaction));
                }
            }
        }
    }

    fn calculate_field_energy(&mut self) {
        let mut energy = 0.0;

        for contact in &self.tertiary_contacts {
            energy -= contact.2 * 0.5;
        }

        for (_, _, ss) in &self.secondary_structures {
            match ss {
                SecondaryStructure::AlphaHelix => energy -= 1.0,
                SecondaryStructure::BetaSheet => energy -= 1.2,
                SecondaryStructure::BetaTurn => energy -= 0.3,
                SecondaryStructure::RandomCoil => energy -= 0.1,
                SecondaryStructure::Unknown => {}
            }
        }

        self.total_field_energy = energy;
    }

    pub fn len(&self) -> usize {
        self.amino_acids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.amino_acids.is_empty()
    }

    pub fn amino_acids(&self) -> &[AminoAcidField] {
        &self.amino_acids
    }

    pub fn secondary_structures(&self) -> &[(usize, usize, SecondaryStructure)] {
        &self.secondary_structures
    }

    pub fn tertiary_contacts(&self) -> &[(usize, usize, Float)] {
        &self.tertiary_contacts
    }

    pub fn field_energy(&self) -> Float {
        self.total_field_energy
    }

    pub fn helix_content(&self) -> Float {
        let total_residues = self.amino_acids.len() as Float;
        if total_residues == 0.0 {
            return 0.0;
        }

        let helix_residues: Float = self
            .secondary_structures
            .iter()
            .filter(|(_, _, ss)| *ss == SecondaryStructure::AlphaHelix)
            .map(|(start, end, _)| (end - start + 1) as Float)
            .sum();

        helix_residues / total_residues
    }

    pub fn sheet_content(&self) -> Float {
        let total_residues = self.amino_acids.len() as Float;
        if total_residues == 0.0 {
            return 0.0;
        }

        let sheet_residues: Float = self
            .secondary_structures
            .iter()
            .filter(|(_, _, ss)| *ss == SecondaryStructure::BetaSheet)
            .map(|(start, end, _)| (end - start + 1) as Float)
            .sum();

        sheet_residues / total_residues
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProteinId(pub u64);

impl ProteinId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct ProteinManifestation {
    pub id: ProteinId,
    pub structure: ProteinStructure,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub folding_coherence: Float,
    pub position: Position3D,
}

impl ProteinManifestation {
    pub fn from_sequence(sequence: &[AminoAcid], position: Position3D) -> Self {
        let structure = ProteinStructure::from_sequence(sequence);

        let mut archetype_pattern = [0.5; NUM_ARCHETYPES];
        for aa in structure.amino_acids() {
            let aff = aa.archetype_affinity();
            for i in 0..NUM_ARCHETYPES {
                archetype_pattern[i] += aff[i];
            }
        }
        let len = structure.len().max(1) as Float;
        for i in 0..NUM_ARCHETYPES {
            archetype_pattern[i] /= len;
        }

        let folding_coherence = (-structure.field_energy() / len).min(1.0).max(0.0);

        Self {
            id: ProteinId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            structure,
            archetype_pattern,
            folding_coherence,
            position,
        }
    }

    pub fn from_sequence_with_name(_name: &str, sequence: Vec<AminoAcid>) -> Self {
        Self::from_sequence(&sequence, Position3D::new(0.0, 0.0, 0.0))
    }

    pub fn amino_acid_sequence(&self) -> Vec<AminoAcid> {
        self.structure
            .amino_acids()
            .iter()
            .map(|aa| aa.amino_acid())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.structure.len()
    }

    pub fn is_empty(&self) -> bool {
        self.structure.is_empty()
    }

    pub fn coherence(&self) -> Float {
        self.folding_coherence
    }

    pub fn archetype_pattern(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_pattern
    }
}

#[derive(Debug, Clone)]
pub struct ProteinFoldingField {
    proteins: Vec<ProteinManifestation>,
    total_coherence: Float,
}

impl ProteinFoldingField {
    pub fn new() -> Self {
        Self {
            proteins: Vec::new(),
            total_coherence: 0.0,
        }
    }

    pub fn add_protein(&mut self, protein: ProteinManifestation) {
        self.proteins.push(protein);
        self.calculate_total_coherence();
    }

    fn calculate_total_coherence(&mut self) {
        if self.proteins.is_empty() {
            self.total_coherence = 0.5;
            return;
        }

        let sum: Float = self.proteins.iter().map(|p| p.coherence()).sum();
        self.total_coherence = sum / self.proteins.len() as Float;
    }

    pub fn coherence(&self) -> Float {
        self.total_coherence
    }

    pub fn protein_count(&self) -> usize {
        self.proteins.len()
    }

    pub fn total_amino_acids(&self) -> usize {
        self.proteins.iter().map(|p| p.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amino_acid_from_index() {
        assert_eq!(AminoAcid::from_index(1), Some(AminoAcid::Alanine));
        assert_eq!(AminoAcid::from_index(20), Some(AminoAcid::Valine));
        assert_eq!(AminoAcid::from_index(21), None);
    }

    #[test]
    fn test_amino_acid_codes() {
        let ala = AminoAcid::Alanine;
        assert_eq!(ala.three_letter_code(), "Ala");
        assert_eq!(ala.one_letter_code(), 'A');
    }

    #[test]
    fn test_hydrophobicity_range() {
        for i in 1..=20 {
            if let Some(aa) = AminoAcid::from_index(i) {
                let hydro = aa.hydrophobicity();
                assert!(hydro >= -5.0 && hydro <= 5.0);
            }
        }
    }

    #[test]
    fn test_amino_acid_classification() {
        assert!(AminoAcid::Isoleucine.is_hydrophobic());
        assert!(AminoAcid::Arginine.is_charged());
        assert!(AminoAcid::Serine.is_polar());
    }

    #[test]
    fn test_amino_acid_field_creation() {
        let field = AminoAcidField::new(AminoAcid::Alanine, Position3D::new(0.0, 0.0, 0.0));
        assert_eq!(field.amino_acid(), AminoAcid::Alanine);
    }

    #[test]
    fn test_secondary_structure_prediction() {
        let pattern = [0.7; NUM_ARCHETYPES];
        let ss = SecondaryStructure::from_archetype_pattern(&pattern);
        assert!(matches!(
            ss,
            SecondaryStructure::AlphaHelix
                | SecondaryStructure::BetaSheet
                | SecondaryStructure::BetaTurn
                | SecondaryStructure::RandomCoil
        ));
    }

    #[test]
    fn test_protein_structure_from_sequence() {
        let sequence = vec![
            AminoAcid::Alanine,
            AminoAcid::Glycine,
            AminoAcid::Leucine,
            AminoAcid::Valine,
            AminoAcid::Isoleucine,
        ];
        let structure = ProteinStructure::from_sequence(&sequence);
        assert_eq!(structure.len(), 5);
    }

    #[test]
    fn test_protein_structure_secondary() {
        let sequence = vec![AminoAcid::Alanine; 20];
        let structure = ProteinStructure::from_sequence(&sequence);
        assert!(!structure.secondary_structures().is_empty());
    }

    #[test]
    fn test_protein_field_energy() {
        let sequence = vec![AminoAcid::Alanine; 10];
        let structure = ProteinStructure::from_sequence(&sequence);
        let energy = structure.field_energy();
        assert!(energy <= 0.0);
    }

    #[test]
    fn test_protein_manifestation() {
        let sequence = vec![AminoAcid::Alanine; 10];
        let protein =
            ProteinManifestation::from_sequence(&sequence, Position3D::new(0.0, 0.0, 0.0));
        assert_eq!(protein.len(), 10);
        assert!(protein.coherence() >= 0.0);
    }

    #[test]
    fn test_protein_folding_field() {
        let mut field = ProteinFoldingField::new();
        let sequence = vec![AminoAcid::Alanine; 10];
        let protein =
            ProteinManifestation::from_sequence(&sequence, Position3D::new(0.0, 0.0, 0.0));
        field.add_protein(protein);
        assert_eq!(field.protein_count(), 1);
    }

    #[test]
    fn test_archetype_affinity_range() {
        for i in 1..=20 {
            if let Some(aa) = AminoAcid::from_index(i) {
                let aff = aa.archetype_affinity();
                for &val in aff.iter() {
                    assert!(val >= 0.0 && val <= 1.0);
                }
            }
        }
    }
}
