//! Nucleotide Sequences as Field Interference Patterns
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Nucleotide sequences as interference patterns - the genetic code is NOT arbitrary,
//!  but reflects deeper holographic patterns encoded in the field."
//!
//! # Key Insight
//!
//! DNA is not just a storage medium - it's an interference pattern that:
//! - Encodes information through phase relationships
//! - Forms a double helix as a standing wave pattern
//! - Stores archetype activations in base pairing

use crate::holographic_foundation::archetype_profile::{
    ArchetypeActivationProfile, NUM_ARCHETYPES,
};
use crate::types::Float;
use std::collections::HashMap;

pub const NUCLEOTIDE_ENCODING_BITS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nucleotide {
    Adenine,
    Thymine,
    Guanine,
    Cytosine,
}

impl Nucleotide {
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0 => Nucleotide::Adenine,
            1 => Nucleotide::Thymine,
            2 => Nucleotide::Guanine,
            3 => Nucleotide::Cytosine,
            _ => Nucleotide::Adenine,
        }
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            Nucleotide::Adenine => 0,
            Nucleotide::Thymine => 1,
            Nucleotide::Guanine => 2,
            Nucleotide::Cytosine => 3,
        }
    }

    pub fn complement(&self) -> Self {
        match self {
            Nucleotide::Adenine => Nucleotide::Thymine,
            Nucleotide::Thymine => Nucleotide::Adenine,
            Nucleotide::Guanine => Nucleotide::Cytosine,
            Nucleotide::Cytosine => Nucleotide::Guanine,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            Nucleotide::Adenine => 'A',
            Nucleotide::Thymine => 'T',
            Nucleotide::Guanine => 'G',
            Nucleotide::Cytosine => 'C',
        }
    }

    pub fn from_symbol(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Nucleotide::Adenine),
            'T' => Some(Nucleotide::Thymine),
            'G' => Some(Nucleotide::Guanine),
            'C' => Some(Nucleotide::Cytosine),
            _ => None,
        }
    }

    pub fn archetype_affinity(&self) -> [Float; NUM_ARCHETYPES] {
        let mut affinity = [0.5; NUM_ARCHETYPES];

        match self {
            Nucleotide::Adenine => {
                affinity[0] = 0.8;
                affinity[7] = 0.7;
                affinity[14] = 0.6;
            }
            Nucleotide::Thymine => {
                affinity[1] = 0.8;
                affinity[8] = 0.7;
                affinity[15] = 0.6;
            }
            Nucleotide::Guanine => {
                affinity[2] = 0.8;
                affinity[9] = 0.7;
                affinity[16] = 0.6;
            }
            Nucleotide::Cytosine => {
                affinity[3] = 0.8;
                affinity[10] = 0.7;
                affinity[17] = 0.6;
            }
        }

        affinity
    }
}

#[derive(Debug, Clone)]
pub struct NucleotideSequence {
    nucleotides: Vec<Nucleotide>,
    archetype_encoding: [Float; NUM_ARCHETYPES],
    interference_pattern: Vec<Float>,
}

impl NucleotideSequence {
    pub fn empty() -> Self {
        Self {
            nucleotides: Vec::new(),
            archetype_encoding: [0.5; NUM_ARCHETYPES],
            interference_pattern: Vec::new(),
        }
    }

    pub fn from_nucleotides(nucleotides: Vec<Nucleotide>) -> Self {
        let mut seq = Self {
            nucleotides,
            archetype_encoding: [0.5; NUM_ARCHETYPES],
            interference_pattern: Vec::new(),
        };
        seq.calculate_archetype_encoding();
        seq.calculate_interference_pattern();
        seq
    }

    pub fn from_string(s: &str) -> Self {
        let nucleotides: Vec<Nucleotide> = s
            .chars()
            .filter_map(Nucleotide::from_symbol)
            .collect();
        Self::from_nucleotides(nucleotides)
    }

    pub fn from_archetype_profile(profile: &ArchetypeActivationProfile, length: usize) -> Self {
        let coeffs = profile.coefficients();
        let mut nucleotides = Vec::with_capacity(length);

        for i in 0..length {
            let archetype_idx = i % NUM_ARCHETYPES;
            let phase = coeffs[archetype_idx];

            let bits = ((phase * 4.0) as u8) & 0b11;
            nucleotides.push(Nucleotide::from_bits(bits));
        }

        let mut seq = Self {
            nucleotides,
            archetype_encoding: *coeffs,
            interference_pattern: Vec::new(),
        };
        seq.calculate_interference_pattern();
        seq
    }

    fn calculate_archetype_encoding(&mut self) {
        if self.nucleotides.is_empty() {
            return;
        }

        let mut encoding = [0.0; NUM_ARCHETYPES];
        let mut counts = [0.0; NUM_ARCHETYPES];

        for nuc in &self.nucleotides {
            let affinity = nuc.archetype_affinity();
            for i in 0..NUM_ARCHETYPES {
                encoding[i] += affinity[i];
                counts[i] += 1.0;
            }
        }

        for i in 0..NUM_ARCHETYPES {
            self.archetype_encoding[i] = if counts[i] > 0.0 {
                encoding[i] / counts[i]
            } else {
                0.5
            };
        }
    }

    fn calculate_interference_pattern(&mut self) {
        let n = self.nucleotides.len();
        if n == 0 {
            return;
        }

        self.interference_pattern = vec![0.0; n];

        for i in 0..n {
            let nuc1 = self.nucleotides[i];
            let aff1 = nuc1.archetype_affinity();

            for j in 0..n {
                if i != j {
                    let nuc2 = self.nucleotides[j];
                    let aff2 = nuc2.archetype_affinity();

                    let distance = (i as Float - j as Float).abs();
                    let phase_shift = (i as Float / n as Float) * std::f64::consts::TAU;

                    let mut interference_sum = 0.0;
                    for k in 0..NUM_ARCHETYPES {
                        let product = aff1[k] * aff2[k];
                        interference_sum += product * phase_shift.cos();
                    }

                    self.interference_pattern[i] +=
                        interference_sum / (NUM_ARCHETYPES as Float * (1.0 + distance * 0.1));
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.nucleotides.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nucleotides.is_empty()
    }

    pub fn nucleotides(&self) -> &[Nucleotide] {
        &self.nucleotides
    }

    pub fn archetype_encoding(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_encoding
    }

    pub fn interference_pattern(&self) -> &[Float] {
        &self.interference_pattern
    }

    pub fn complement(&self) -> Self {
        let complement_nucs: Vec<Nucleotide> =
            self.nucleotides.iter().map(|n| n.complement()).collect();
        Self::from_nucleotides(complement_nucs)
    }

    pub fn to_string(&self) -> String {
        self.nucleotides.iter().map(|n| n.symbol()).collect()
    }

    pub fn total_interference(&self) -> Float {
        self.interference_pattern.iter().sum()
    }

    pub fn average_interference(&self) -> Float {
        if self.interference_pattern.is_empty() {
            return 0.0;
        }
        self.total_interference() / self.interference_pattern.len() as Float
    }
}

#[derive(Debug, Clone)]
pub struct NucleotideEncoding {
    sequence_to_codon: HashMap<[Nucleotide; 3], u8>,
    codon_to_amino: HashMap<u8, u8>,
}

impl Default for NucleotideEncoding {
    fn default() -> Self {
        Self::standard_genetic_code()
    }
}

impl NucleotideEncoding {
    pub fn standard_genetic_code() -> Self {
        let mut encoding = Self {
            sequence_to_codon: HashMap::new(),
            codon_to_amino: HashMap::new(),
        };

        encoding.build_standard_code();
        encoding
    }

    fn build_standard_code(&mut self) {
        let codons = [
            "TTT", "TTC", "TTA", "TTG", "TCT", "TCC", "TCA", "TCG", "TAT", "TAC", "TAA", "TAG",
            "TGT", "TGC", "TGA", "TGG", "CTT", "CTC", "CTA", "CTG", "CCT", "CCC", "CCA", "CCG",
            "CAT", "CAC", "CAA", "CAG", "CGT", "CGC", "CGA", "CGG", "ATT", "ATC", "ATA", "ATG",
            "ACT", "ACC", "ACA", "ACG", "AAT", "AAC", "AAA", "AAG", "AGT", "AGC", "AGA", "AGG",
            "GTT", "GTC", "GTA", "GTG", "GCT", "GCC", "GCA", "GCG", "GAT", "GAC", "GAA", "GAG",
            "GGT", "GGC", "GGA", "GGG",
        ];

        let amino_acids: &[u8] = &[
            1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 0, 0, 5, 5, 0, 6, 2, 2, 2, 2, 3, 3, 3, 3, 7, 7, 8, 8, 9,
            9, 9, 9, 10, 10, 10, 11, 12, 12, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 17,
            17, 18, 18, 18, 18, 19, 19, 20, 20, 21, 21, 21, 21,
        ];

        for (i, codon_str) in codons.iter().enumerate() {
            let nucs: Vec<Nucleotide> = codon_str
                .chars()
                .filter_map(Nucleotide::from_symbol)
                .collect();

            if nucs.len() == 3 {
                let codon_key = [nucs[0], nucs[1], nucs[2]];
                self.sequence_to_codon.insert(codon_key, i as u8);
                self.codon_to_amino.insert(i as u8, amino_acids[i]);
            }
        }
    }

    pub fn translate_codon(&self, codon: [Nucleotide; 3]) -> Option<u8> {
        self.sequence_to_codon.get(&codon).copied()
    }

    pub fn codon_to_amino_acid(&self, codon_idx: u8) -> Option<u8> {
        self.codon_to_amino.get(&codon_idx).copied()
    }

    pub fn translate_sequence(&self, sequence: &NucleotideSequence) -> Vec<u8> {
        let nucs = sequence.nucleotides();
        let mut amino_acids = Vec::new();

        for chunk in nucs.chunks(3) {
            if chunk.len() == 3 {
                if let Some(codon_idx) = self.translate_codon([chunk[0], chunk[1], chunk[2]]) {
                    if let Some(amino) = self.codon_to_amino_acid(codon_idx) {
                        if amino > 0 {
                            amino_acids.push(amino);
                        }
                    }
                }
            }
        }

        amino_acids
    }
}

#[derive(Debug, Clone)]
pub struct DNAInterferencePattern {
    strand1: NucleotideSequence,
    strand2: NucleotideSequence,
    #[allow(dead_code)]
    helix_phase: Float,
    coherence: Float,
}

impl DNAInterferencePattern {
    pub fn from_sequence(sequence: NucleotideSequence) -> Self {
        let strand2 = sequence.complement();
        let helix_phase = 0.0;
        let coherence = Self::calculate_coherence(&sequence, &strand2);

        Self {
            strand1: sequence,
            strand2,
            helix_phase,
            coherence,
        }
    }

    pub fn from_double_stranded(strand1: NucleotideSequence, strand2: NucleotideSequence) -> Self {
        let helix_phase = 0.0;
        let coherence = Self::calculate_coherence(&strand1, &strand2);

        Self {
            strand1,
            strand2,
            helix_phase,
            coherence,
        }
    }

    fn calculate_coherence(s1: &NucleotideSequence, s2: &NucleotideSequence) -> Float {
        let nucs1 = s1.nucleotides();
        let nucs2 = s2.nucleotides();

        if nucs1.len() != nucs2.len() {
            return 0.0;
        }

        let mut correct_pairs = 0;
        for (n1, n2) in nucs1.iter().zip(nucs2.iter()) {
            if *n1 == n2.complement() {
                correct_pairs += 1;
            }
        }

        correct_pairs as Float / nucs1.len().max(1) as Float
    }

    pub fn length(&self) -> usize {
        self.strand1.len()
    }

    pub fn coherence(&self) -> Float {
        self.coherence
    }

    pub fn strand1(&self) -> &NucleotideSequence {
        &self.strand1
    }

    pub fn strand2(&self) -> &NucleotideSequence {
        &self.strand2
    }

    pub fn total_interference(&self) -> Float {
        (self.strand1.total_interference() + self.strand2.total_interference()) / 2.0
    }
}

#[derive(Debug, Clone)]
pub struct DNAHelix {
    pattern: DNAInterferencePattern,
    encoding: NucleotideEncoding,
    gc_content: Float,
    melting_temperature: Float,
}

impl DNAHelix {
    pub fn from_sequence(sequence: NucleotideSequence) -> Self {
        let pattern = DNAInterferencePattern::from_sequence(sequence);
        let encoding = NucleotideEncoding::standard_genetic_code();
        let gc_content = Self::calculate_gc_content(pattern.strand1());
        let melting_temperature = Self::estimate_melting_temp(gc_content, pattern.length());

        Self {
            pattern,
            encoding,
            gc_content,
            melting_temperature,
        }
    }

    fn calculate_gc_content(sequence: &NucleotideSequence) -> Float {
        let nucs = sequence.nucleotides();
        if nucs.is_empty() {
            return 0.0;
        }

        let gc_count = nucs
            .iter()
            .filter(|n| matches!(n, Nucleotide::Guanine | Nucleotide::Cytosine))
            .count();

        gc_count as Float / nucs.len() as Float
    }

    fn estimate_melting_temp(gc_content: Float, length: usize) -> Float {
        64.9 + 41.0 * (gc_content - 0.05) - 500.0 / length.max(1) as Float
    }

    pub fn length(&self) -> usize {
        self.pattern.length()
    }

    pub fn coherence(&self) -> Float {
        self.pattern.coherence()
    }

    pub fn gc_content(&self) -> Float {
        self.gc_content
    }

    pub fn melting_temperature(&self) -> Float {
        self.melting_temperature
    }

    pub fn translate(&self) -> Vec<u8> {
        self.encoding.translate_sequence(self.pattern.strand1())
    }

    pub fn interference(&self) -> Float {
        self.pattern.total_interference()
    }

    pub fn sequence(&self) -> &NucleotideSequence {
        self.pattern.strand1()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleotide_complement() {
        assert_eq!(Nucleotide::Adenine.complement(), Nucleotide::Thymine);
        assert_eq!(Nucleotide::Thymine.complement(), Nucleotide::Adenine);
        assert_eq!(Nucleotide::Guanine.complement(), Nucleotide::Cytosine);
        assert_eq!(Nucleotide::Cytosine.complement(), Nucleotide::Guanine);
    }

    #[test]
    fn test_nucleotide_bits_roundtrip() {
        for bits in 0..4 {
            let nuc = Nucleotide::from_bits(bits);
            assert_eq!(nuc.to_bits(), bits);
        }
    }

    #[test]
    fn test_nucleotide_symbol() {
        assert_eq!(Nucleotide::Adenine.symbol(), 'A');
        assert_eq!(Nucleotide::Thymine.symbol(), 'T');
        assert_eq!(Nucleotide::Guanine.symbol(), 'G');
        assert_eq!(Nucleotide::Cytosine.symbol(), 'C');
    }

    #[test]
    fn test_nucleotide_sequence_from_string() {
        let seq = NucleotideSequence::from_string("ATGC");
        assert_eq!(seq.len(), 4);
        assert_eq!(seq.nucleotides()[0], Nucleotide::Adenine);
        assert_eq!(seq.nucleotides()[3], Nucleotide::Cytosine);
    }

    #[test]
    fn test_nucleotide_sequence_complement() {
        let seq = NucleotideSequence::from_string("ATGC");
        let comp = seq.complement();
        assert_eq!(comp.nucleotides()[0], Nucleotide::Thymine);
        assert_eq!(comp.nucleotides()[3], Nucleotide::Guanine);
    }

    #[test]
    fn test_nucleotide_sequence_archetype_encoding() {
        let seq = NucleotideSequence::from_string("AAAA");
        let encoding = seq.archetype_encoding();
        assert!(encoding[0] > 0.5);
    }

    #[test]
    fn test_nucleotide_sequence_from_profile() {
        let profile = ArchetypeActivationProfile::initial();
        let seq = NucleotideSequence::from_archetype_profile(&profile, 100);
        assert_eq!(seq.len(), 100);
    }

    #[test]
    fn test_dna_interference_pattern() {
        let seq = NucleotideSequence::from_string("ATGCATGC");
        let pattern = DNAInterferencePattern::from_sequence(seq);
        assert_eq!(pattern.length(), 8);
        assert!(pattern.coherence() > 0.99);
    }

    #[test]
    fn test_dna_helix_creation() {
        let seq = NucleotideSequence::from_string("ATGCATGCATGC");
        let helix = DNAHelix::from_sequence(seq);
        assert_eq!(helix.length(), 12);
        assert!(helix.gc_content() > 0.0 && helix.gc_content() < 1.0);
    }

    #[test]
    fn test_nucleotide_encoding_translate() {
        let encoding = NucleotideEncoding::standard_genetic_code();
        let codon = [
            Nucleotide::Thymine,
            Nucleotide::Thymine,
            Nucleotide::Thymine,
        ];
        let codon_idx = encoding.translate_codon(codon);
        assert!(codon_idx.is_some());
    }

    #[test]
    fn test_gc_content() {
        let seq = NucleotideSequence::from_string("GGCC");
        let helix = DNAHelix::from_sequence(seq);
        assert!((helix.gc_content() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_interference_pattern_nonempty() {
        let seq = NucleotideSequence::from_string("ATGCATGC");
        assert!(!seq.interference_pattern().is_empty());
    }
}
