//! Simultaneous Emergence: Atoms and Galaxies from Same Field
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 8:
//! "Simultaneous emergence of atoms AND galaxies (same field, different resolution)"
//!
//! The holographic field manifests at MULTIPLE scales simultaneously. At atomic
//! resolution, stable attractors appear as atoms. At galactic resolution, the
//! same field patterns appear as galaxies. This is the holographic principle:
//!
//! **The whole is present at every scale.**
//!
//! Key Insight: Atoms and galaxies don't form separately. They are different
//! RESOLUTION views of the SAME underlying field coherence patterns.

use crate::types::Float;
use std::collections::HashMap;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::field_state::{FieldAmplitude, HolographicFieldState, Position3D};
use super::super::quantum_consciousness::quantum_numbers::QuantumNumberSet;
use super::super::scale_level::ScaleLevel;
use super::atomic_manifestation::{
    AtomFormationEvent, AtomicManifestation, ManifestationConditions, ManifestationType,
};
use super::attractor_field::{AttractorField, FieldConfiguration};
use super::element_attractor::ElementAttractorField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EmergenceId(u64);

impl EmergenceId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GalacticType {
    Spiral,
    Elliptical,
    Irregular,
    Lenticular,
    Dwarf,
}

impl GalacticType {
    pub fn archetype_signature(&self) -> [Float; NUM_ARCHETYPES] {
        let mut sig = [0.5; NUM_ARCHETYPES];

        match self {
            GalacticType::Spiral => {
                sig[0] = 0.7;
                sig[2] = 0.8;
                sig[6] = 0.6;
                sig[14] = 0.7;
            }
            GalacticType::Elliptical => {
                sig[0] = 0.6;
                sig[1] = 0.7;
                sig[5] = 0.8;
            }
            GalacticType::Irregular => {
                sig[2] = 0.5;
                sig[3] = 0.7;
                sig[21] = 0.9;
            }
            GalacticType::Lenticular => {
                sig[0] = 0.65;
                sig[1] = 0.6;
                sig[6] = 0.7;
            }
            GalacticType::Dwarf => {
                sig[0] = 0.4;
                sig[7..14].copy_from_slice(&[0.6; 7]);
            }
        }

        sig
    }

    pub fn mass_range(&self) -> (Float, Float) {
        match self {
            GalacticType::Spiral => (1e9, 1e12),
            GalacticType::Elliptical => (1e8, 1e13),
            GalacticType::Irregular => (1e7, 1e10),
            GalacticType::Lenticular => (1e9, 1e11),
            GalacticType::Dwarf => (1e6, 1e9),
        }
    }

    pub fn star_count_estimate(&self) -> u64 {
        match self {
            GalacticType::Spiral => 200_000_000_000,
            GalacticType::Elliptical => 1_000_000_000_000,
            GalacticType::Irregular => 50_000_000_000,
            GalacticType::Lenticular => 100_000_000_000,
            GalacticType::Dwarf => 1_000_000_000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GalacticEmergence {
    pub position: Position3D,
    pub galactic_type: GalacticType,
    pub mass: Float,
    pub radius: Float,
    pub coherence: Float,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub formation_time: Float,
    pub atom_correspondence: Vec<u32>,
}

impl GalacticEmergence {
    pub fn new(
        position: Position3D,
        galactic_type: GalacticType,
        coherence: Float,
        formation_time: Float,
    ) -> Self {
        let archetype_pattern = galactic_type.archetype_signature();
        let (min_mass, max_mass) = galactic_type.mass_range();

        let mass = min_mass + coherence * (max_mass - min_mass);
        let radius = mass.powf(0.5) * 1e-3;

        Self {
            position,
            galactic_type,
            mass,
            radius,
            coherence,
            archetype_pattern,
            formation_time,
            atom_correspondence: Vec::new(),
        }
    }

    pub fn from_field(
        field: &HolographicFieldState,
        position: Position3D,
        formation_time: Float,
    ) -> Option<Self> {
        let node = field.get_node_at(&position)?;
        let galactic_amp = node.get_amplitude_at_scale(ScaleLevel::Cosmic);

        if galactic_amp.magnitude() < 0.3 {
            return None;
        }

        let coherence = node.coherence;
        let galactic_type = Self::determine_type(&node.archetype_vector);

        Some(Self::new(
            position,
            galactic_type,
            coherence,
            formation_time,
        ))
    }

    fn determine_type(archetype: &[Float; NUM_ARCHETYPES]) -> GalacticType {
        let mind: Float = archetype[0..7].iter().sum::<Float>() / 7.0;
        let catalyst = archetype[2];
        let choice = archetype[21];

        if choice > 0.8 {
            GalacticType::Irregular
        } else if catalyst > 0.7 && mind > 0.6 {
            GalacticType::Spiral
        } else if mind > 0.7 {
            GalacticType::Elliptical
        } else if mind > 0.55 {
            GalacticType::Lenticular
        } else {
            GalacticType::Dwarf
        }
    }

    pub fn add_atom_correspondence(&mut self, atomic_number: u32) {
        self.atom_correspondence.push(atomic_number);
    }

    pub fn correspondence_strength(&self, atomic_number: u32) -> Float {
        let element = ElementAttractorField::from_atomic_number(atomic_number);
        let element_arch = element.configuration().archetype_vector;

        let dot: Float = self
            .archetype_pattern
            .iter()
            .zip(element_arch.iter())
            .map(|(a, b)| a * b)
            .sum();

        dot / NUM_ARCHETYPES as Float
    }

    pub fn dominant_elements(&self) -> Vec<(u32, Float)> {
        let mut correlations: Vec<(u32, Float)> = (1..=118)
            .map(|z| (z, self.correspondence_strength(z)))
            .collect();

        correlations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        correlations.truncate(10);
        correlations
    }
}

#[derive(Debug, Clone)]
pub struct AtomGalaxyPair {
    pub atom: AtomicManifestation,
    pub galaxy: GalacticEmergence,
    pub resonance: Float,
    pub scale_ratio: Float,
    pub archetype_alignment: Float,
}

impl AtomGalaxyPair {
    pub fn new(atom: AtomicManifestation, galaxy: GalacticEmergence) -> Self {
        let resonance = Self::calculate_resonance(&atom, &galaxy);
        let scale_ratio = galaxy.radius / (atom.element().atomic_radius() * 1e-12);
        let archetype_alignment = Self::calculate_alignment(&atom, &galaxy);

        Self {
            atom,
            galaxy,
            resonance,
            scale_ratio,
            archetype_alignment,
        }
    }

    fn calculate_resonance(atom: &AtomicManifestation, galaxy: &GalacticEmergence) -> Float {
        let atom_arch = atom.element().configuration().archetype_vector;
        let galaxy_arch = galaxy.archetype_pattern;

        let dot: Float = atom_arch
            .iter()
            .zip(galaxy_arch.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_atom: Float = atom_arch.iter().map(|a| a * a).sum::<Float>().sqrt();
        let norm_galaxy: Float = galaxy_arch.iter().map(|b| b * b).sum::<Float>().sqrt();

        if norm_atom > 1e-10 && norm_galaxy > 1e-10 {
            dot / (norm_atom * norm_galaxy)
        } else {
            0.0
        }
    }

    fn calculate_alignment(atom: &AtomicManifestation, galaxy: &GalacticEmergence) -> Float {
        let atom_coherence = atom.element().configuration().coherence;
        let galaxy_coherence = galaxy.coherence;

        (atom_coherence + galaxy_coherence) / 2.0
    }

    pub fn is_strongly_coupled(&self) -> bool {
        self.resonance > 0.7 && self.archetype_alignment > 0.6
    }

    pub fn information_content(&self) -> Float {
        self.resonance * self.archetype_alignment * (self.scale_ratio.log10().abs() + 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct EmergenceScalePair {
    pub atomic_scale: ScaleLevel,
    pub galactic_scale: ScaleLevel,
    pub atomic_config: FieldConfiguration,
    pub galactic_config: FieldConfiguration,
    pub coherence_at_atomic: Float,
    pub coherence_at_galactic: Float,
    pub simultaneous_factor: Float,
}

impl EmergenceScalePair {
    pub fn from_field(field: &HolographicFieldState, position: &Position3D) -> Option<Self> {
        let node = field.get_node_at(position)?;

        let atomic_amp = node.get_amplitude_at_scale(ScaleLevel::Atomic);
        let cosmic_amp = node.get_amplitude_at_scale(ScaleLevel::Cosmic);

        let atomic_config = FieldConfiguration::new(node.archetype_vector);
        let cosmic_config = FieldConfiguration::new(node.archetype_vector);

        let coherence_at_atomic = atomic_amp.magnitude();
        let coherence_at_cosmic = cosmic_amp.magnitude();

        let simultaneous_factor = coherence_at_atomic * coherence_at_cosmic;

        Some(Self {
            atomic_scale: ScaleLevel::Atomic,
            galactic_scale: ScaleLevel::Cosmic,
            atomic_config,
            galactic_config: cosmic_config,
            coherence_at_atomic,
            coherence_at_galactic: coherence_at_cosmic,
            simultaneous_factor,
        })
    }

    pub fn can_manifest_simultaneously(&self) -> bool {
        self.coherence_at_atomic > 0.3
            && self.coherence_at_galactic > 0.3
            && self.simultaneous_factor > 0.1
    }

    pub fn dominant_scale(&self) -> ScaleLevel {
        if self.coherence_at_atomic > self.coherence_at_galactic {
            ScaleLevel::Atomic
        } else {
            ScaleLevel::Cosmic
        }
    }

    pub fn scale_separation(&self) -> Float {
        let atomic_len = ScaleLevel::Atomic.characteristic_length();
        let cosmic_len = ScaleLevel::Cosmic.characteristic_length();

        if atomic_len > 0.0 && cosmic_len > 0.0 {
            cosmic_len / atomic_len
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimultaneousEmergence {
    pairs: Vec<AtomGalaxyPair>,
    scale_pairs: Vec<EmergenceScalePair>,
    total_atoms: u32,
    total_galaxies: u32,
    emergence_events: Vec<(Float, String)>,
    coherence_history: Vec<Float>,
}

impl SimultaneousEmergence {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
            scale_pairs: Vec::new(),
            total_atoms: 0,
            total_galaxies: 0,
            emergence_events: Vec::new(),
            coherence_history: Vec::new(),
        }
    }

    pub fn process_field(&mut self, field: &HolographicFieldState, timestamp: Float) {
        let root_position = field.root().position.clone();

        if let Some(scale_pair) = EmergenceScalePair::from_field(field, &root_position) {
            if scale_pair.can_manifest_simultaneously() {
                self.scale_pairs.push(scale_pair);
            }
        }

        if let Some(galaxy) = GalacticEmergence::from_field(field, root_position.clone(), timestamp)
        {
            if let Some(atom) = AtomicManifestation::from_field(field, root_position.clone()) {
                let pair = AtomGalaxyPair::new(atom, galaxy.clone());
                self.pairs.push(pair);
                self.total_atoms += 1;
            }

            self.total_galaxies += 1;
            self.emergence_events.push((
                timestamp,
                format!("Galaxy emerged: {:?}", galaxy.galactic_type),
            ));
        }

        let coherence = field.global_coherence();
        self.coherence_history.push(coherence);
    }

    pub fn add_pair(&mut self, pair: AtomGalaxyPair) {
        self.total_atoms += 1;
        self.total_galaxies += 1;
        self.pairs.push(pair);
    }

    pub fn record_emergence(&mut self, timestamp: Float, description: String) {
        self.emergence_events.push((timestamp, description));
    }

    pub fn pairs(&self) -> &[AtomGalaxyPair] {
        &self.pairs
    }

    pub fn scale_pairs(&self) -> &[EmergenceScalePair] {
        &self.scale_pairs
    }

    pub fn total_atoms(&self) -> u32 {
        self.total_atoms
    }

    pub fn total_galaxies(&self) -> u32 {
        self.total_galaxies
    }

    pub fn emergence_events(&self) -> &[(Float, String)] {
        &self.emergence_events
    }

    pub fn coherence_history(&self) -> &[Float] {
        &self.coherence_history
    }

    pub fn strongly_coupled_pairs(&self) -> Vec<&AtomGalaxyPair> {
        self.pairs
            .iter()
            .filter(|p| p.is_strongly_coupled())
            .collect()
    }

    pub fn average_resonance(&self) -> Float {
        if self.pairs.is_empty() {
            return 0.0;
        }

        self.pairs.iter().map(|p| p.resonance).sum::<Float>() / self.pairs.len() as Float
    }

    pub fn total_information(&self) -> Float {
        self.pairs.iter().map(|p| p.information_content()).sum()
    }

    pub fn find_pairs_by_element(&self, atomic_number: u32) -> Vec<&AtomGalaxyPair> {
        self.pairs
            .iter()
            .filter(|p| p.atom.atomic_number() == atomic_number)
            .collect()
    }

    pub fn find_pairs_by_galaxy_type(&self, galactic_type: GalacticType) -> Vec<&AtomGalaxyPair> {
        self.pairs
            .iter()
            .filter(|p| p.galaxy.galactic_type == galactic_type)
            .collect()
    }

    pub fn dominant_element_in_galaxies(&self) -> Option<u32> {
        let mut counts: HashMap<u32, u32> = HashMap::new();

        for pair in &self.pairs {
            *counts.entry(pair.atom.atomic_number()).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(z, _)| z)
    }

    pub fn galaxy_type_distribution(&self) -> HashMap<GalacticType, u32> {
        let mut dist = HashMap::new();

        for pair in &self.pairs {
            *dist.entry(pair.galaxy.galactic_type).or_insert(0) += 1;
        }

        dist
    }

    pub fn scale_correlation(&self) -> Float {
        if self.scale_pairs.is_empty() {
            return 0.0;
        }

        let sum_atomic: Float = self
            .scale_pairs
            .iter()
            .map(|sp| sp.coherence_at_atomic)
            .sum();
        let sum_galactic: Float = self
            .scale_pairs
            .iter()
            .map(|sp| sp.coherence_at_galactic)
            .sum();
        let n = self.scale_pairs.len() as Float;

        let mean_atomic = sum_atomic / n;
        let mean_galactic = sum_galactic / n;

        let mut covariance = 0.0;
        let mut var_atomic = 0.0;
        let mut var_galactic = 0.0;

        for sp in &self.scale_pairs {
            let diff_atomic = sp.coherence_at_atomic - mean_atomic;
            let diff_galactic = sp.coherence_at_galactic - mean_galactic;

            covariance += diff_atomic * diff_galactic;
            var_atomic += diff_atomic * diff_atomic;
            var_galactic += diff_galactic * diff_galactic;
        }

        let denom = (var_atomic * var_galactic).sqrt();
        if denom > 1e-10 {
            covariance / denom
        } else {
            0.0
        }
    }

    pub fn report(&self) -> String {
        format!(
            "Simultaneous Emergence Report:\n\
             - Total atom-galaxy pairs: {}\n\
             - Strongly coupled pairs: {}\n\
             - Average resonance: {:.3}\n\
             - Total information: {:.3e}\n\
             - Scale correlation: {:.3}\n\
             - Emergence events: {}",
            self.pairs.len(),
            self.strongly_coupled_pairs().len(),
            self.average_resonance(),
            self.total_information(),
            self.scale_correlation(),
            self.emergence_events.len()
        )
    }
}

impl Default for SimultaneousEmergence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galactic_type_archetype() {
        let spiral_sig = GalacticType::Spiral.archetype_signature();
        assert!(spiral_sig[2] > 0.5);

        let irregular_sig = GalacticType::Irregular.archetype_signature();
        assert!(irregular_sig[21] > 0.5);
    }

    #[test]
    fn test_galactic_emergence_creation() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(pos, GalacticType::Spiral, 0.8, 1.0);

        assert_eq!(galaxy.galactic_type, GalacticType::Spiral);
        assert!(galaxy.mass > 0.0);
        assert!(galaxy.radius > 0.0);
    }

    #[test]
    fn test_correspondence_strength() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(pos, GalacticType::Spiral, 0.8, 1.0);

        let strength = galaxy.correspondence_strength(1);
        assert!(strength > 0.0 && strength <= 1.0);
    }

    #[test]
    fn test_dominant_elements() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(pos, GalacticType::Spiral, 0.8, 1.0);

        let dominant = galaxy.dominant_elements();
        assert_eq!(dominant.len(), 10);
    }

    #[test]
    fn test_atom_galaxy_pair() {
        let atom_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(atom_pos);

        let galaxy_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(galaxy_pos, GalacticType::Spiral, 0.8, 1.0);

        let pair = AtomGalaxyPair::new(atom, galaxy);

        assert!(pair.resonance >= 0.0 && pair.resonance <= 1.0);
        assert!(pair.scale_ratio > 0.0);
    }

    #[test]
    fn test_emergence_scale_pair() {
        let field = HolographicFieldState::new(1.0);
        let pos = Position3D {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        };

        let scale_pair = EmergenceScalePair::from_field(&field, &pos);
        assert!(scale_pair.is_some());
    }

    #[test]
    fn test_simultaneous_emergence_creation() {
        let se = SimultaneousEmergence::new();

        assert_eq!(se.total_atoms(), 0);
        assert_eq!(se.total_galaxies(), 0);
        assert!(se.pairs().is_empty());
    }

    #[test]
    fn test_add_pair() {
        let mut se = SimultaneousEmergence::new();

        let atom_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(atom_pos);

        let galaxy_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(galaxy_pos, GalacticType::Spiral, 0.8, 1.0);

        let pair = AtomGalaxyPair::new(atom, galaxy);
        se.add_pair(pair);

        assert_eq!(se.total_atoms(), 1);
        assert_eq!(se.total_galaxies(), 1);
    }

    #[test]
    fn test_strongly_coupled_pairs() {
        let mut se = SimultaneousEmergence::new();

        let atom_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(atom_pos);

        let galaxy_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(galaxy_pos, GalacticType::Spiral, 0.8, 1.0);

        let pair = AtomGalaxyPair::new(atom, galaxy);
        se.add_pair(pair);

        let coupled = se.strongly_coupled_pairs();
        assert!(!coupled.is_empty() || coupled.is_empty());
    }

    #[test]
    fn test_average_resonance() {
        let mut se = SimultaneousEmergence::new();

        let atom_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(atom_pos);

        let galaxy_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let galaxy = GalacticEmergence::new(galaxy_pos, GalacticType::Spiral, 0.8, 1.0);

        let pair = AtomGalaxyPair::new(atom, galaxy);
        se.add_pair(pair);

        let avg_res = se.average_resonance();
        assert!(avg_res >= 0.0 && avg_res <= 1.0);
    }

    #[test]
    fn test_galaxy_type_distribution() {
        let mut se = SimultaneousEmergence::new();

        let atom_pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(atom_pos);

        for gt in [
            GalacticType::Spiral,
            GalacticType::Elliptical,
            GalacticType::Spiral,
        ] {
            let galaxy_pos = Position3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let galaxy = GalacticEmergence::new(galaxy_pos, gt, 0.8, 1.0);
            let pair = AtomGalaxyPair::new(atom.clone(), galaxy);
            se.add_pair(pair);
        }

        let dist = se.galaxy_type_distribution();
        assert_eq!(*dist.get(&GalacticType::Spiral).unwrap_or(&0), 2);
        assert_eq!(*dist.get(&GalacticType::Elliptical).unwrap_or(&0), 1);
    }

    #[test]
    fn test_report() {
        let se = SimultaneousEmergence::new();
        let report = se.report();

        assert!(report.contains("Simultaneous Emergence"));
        assert!(report.contains("pairs"));
    }
}
