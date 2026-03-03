//! Simultaneous Emergence: Molecules and Planets from Same Field
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 9:
//! "Simultaneous emergence of molecules AND planets"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Molecules and planets form together (1st Density - Molecular Realm)"
//!
//! Key Insight: Molecules and planets don't form separately. They are
//! different RESOLUTION views of the SAME underlying field coherence patterns.
//! The holographic principle means both emerge simultaneously from field dynamics.

use crate::types::Float;
use std::collections::HashMap;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::atomic_emergence::ElementAttractorField;
use super::super::field_state::{HolographicFieldState, Position3D};
use super::super::scale_level::ScaleLevel;
use super::bond_formation::{ArchetypeBond, BondFormation, BondType};
use super::functional_groups::{FunctionalGroup, FunctionalGroupResonance};
use super::molecular_geometry::{GeometryPrediction, MolecularShape};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MolecularId(u64);

impl MolecularId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanetaryId(u64);

impl PlanetaryId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanetType {
    Rocky,
    GasGiant,
    IceGiant,
    Dwarf,
    Terrestrial,
    Ocean,
    Desert,
}

impl PlanetType {
    pub fn archetype_signature(&self) -> [Float; NUM_ARCHETYPES] {
        let mut sig = [0.5; NUM_ARCHETYPES];

        match self {
            PlanetType::Rocky => {
                sig[0] = 0.75;
                sig[7] = 0.7;
                sig[1] = 0.6;
            }
            PlanetType::GasGiant => {
                sig[2] = 0.8;
                sig[14] = 0.7;
                sig[6] = 0.65;
            }
            PlanetType::IceGiant => {
                sig[14] = 0.75;
                sig[2] = 0.65;
                sig[15] = 0.6;
            }
            PlanetType::Dwarf => {
                sig[7] = 0.6;
                sig[8] = 0.6;
            }
            PlanetType::Terrestrial => {
                sig[0] = 0.7;
                sig[7] = 0.75;
                sig[1] = 0.65;
            }
            PlanetType::Ocean => {
                sig[14] = 0.8;
                sig[2] = 0.7;
            }
            PlanetType::Desert => {
                sig[7] = 0.7;
                sig[0] = 0.65;
            }
        }

        sig
    }

    pub fn typical_mass_range(&self) -> (Float, Float) {
        match self {
            PlanetType::Rocky => (0.1, 10.0),
            PlanetType::GasGiant => (10.0, 1000.0),
            PlanetType::IceGiant => (10.0, 50.0),
            PlanetType::Dwarf => (0.0001, 0.1),
            PlanetType::Terrestrial => (0.5, 5.0),
            PlanetType::Ocean => (1.0, 20.0),
            PlanetType::Desert => (0.1, 5.0),
        }
    }

    pub fn typical_radius_km(&self) -> Float {
        match self {
            PlanetType::Rocky => 6000.0,
            PlanetType::GasGiant => 50000.0,
            PlanetType::IceGiant => 25000.0,
            PlanetType::Dwarf => 1500.0,
            PlanetType::Terrestrial => 6371.0,
            PlanetType::Ocean => 8000.0,
            PlanetType::Desert => 4000.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanetaryEmergence {
    pub id: PlanetaryId,
    pub planet_type: PlanetType,
    pub position: Position3D,
    pub mass_earth_masses: Float,
    pub radius_km: Float,
    pub coherence: Float,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub formation_time: Float,
    pub molecular_correspondence: Vec<MolecularId>,
}

impl PlanetaryEmergence {
    pub fn new(
        planet_type: PlanetType,
        position: Position3D,
        coherence: Float,
        formation_time: Float,
    ) -> Self {
        let archetype_pattern = planet_type.archetype_signature();
        let (min_mass, max_mass) = planet_type.typical_mass_range();
        let mass = min_mass + coherence * (max_mass - min_mass);
        let radius = planet_type.typical_radius_km() * (mass / min_mass).powf(0.3);

        Self {
            id: PlanetaryId::new(rand_u64()),
            planet_type,
            position,
            mass_earth_masses: mass,
            radius_km: radius,
            coherence,
            archetype_pattern,
            formation_time,
            molecular_correspondence: Vec::new(),
        }
    }

    pub fn from_field(
        field: &HolographicFieldState,
        position: Position3D,
        formation_time: Float,
    ) -> Option<Self> {
        let node = field.get_node_at(&position)?;
        let planetary_amp = node.get_amplitude_at_scale(ScaleLevel::Planetary);

        if planetary_amp.magnitude() < 0.3 {
            return None;
        }

        let coherence = node.coherence;
        let planet_type = Self::determine_type(&node.archetype_vector);

        Some(Self::new(planet_type, position, coherence, formation_time))
    }

    fn determine_type(archetype: &[Float; NUM_ARCHETYPES]) -> PlanetType {
        let mind: Float = archetype[0..7].iter().sum::<Float>() / 7.0;
        let body: Float = archetype[7..14].iter().sum::<Float>() / 7.0;
        let spirit: Float = archetype[14..21].iter().sum::<Float>() / 7.0;
        let catalyst = archetype[2];

        if spirit > 0.65 && catalyst > 0.6 {
            PlanetType::Ocean
        } else if spirit > 0.6 {
            PlanetType::GasGiant
        } else if spirit > 0.5 && body > 0.5 {
            PlanetType::IceGiant
        } else if mind > 0.7 && body > 0.6 {
            PlanetType::Rocky
        } else if mind > 0.6 {
            PlanetType::Terrestrial
        } else if body > 0.6 {
            PlanetType::Desert
        } else {
            PlanetType::Dwarf
        }
    }

    pub fn add_molecular_correspondence(&mut self, mol_id: MolecularId) {
        self.molecular_correspondence.push(mol_id);
    }

    pub fn correspondence_strength(&self, archetype_pattern: &[Float; NUM_ARCHETYPES]) -> Float {
        let dot: Float = self
            .archetype_pattern
            .iter()
            .zip(archetype_pattern.iter())
            .map(|(a, b)| a * b)
            .sum();

        dot / NUM_ARCHETYPES as Float
    }

    pub fn dominant_elements(&self) -> Vec<(u32, Float)> {
        let mut correlations: Vec<(u32, Float)> = (1..=20)
            .map(|z| {
                let elem = ElementAttractorField::from_atomic_number(z);
                (
                    z,
                    self.correspondence_strength(&elem.configuration().archetype_vector),
                )
            })
            .collect();

        correlations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        correlations.truncate(5);
        correlations
    }
}

#[derive(Debug, Clone)]
pub struct MolecularManifestation {
    pub id: MolecularId,
    pub elements: Vec<ElementAttractorField>,
    pub bonds: Vec<ArchetypeBond>,
    pub geometry: GeometryPrediction,
    pub functional_groups: FunctionalGroupResonance,
    pub position: Position3D,
    pub total_energy: Float,
    pub coherence: Float,
    pub archetype_pattern: [Float; NUM_ARCHETYPES],
    pub formation_time: Float,
}

impl MolecularManifestation {
    pub fn new(
        elements: Vec<ElementAttractorField>,
        bonds: Vec<ArchetypeBond>,
        position: Position3D,
        formation_time: Float,
    ) -> Self {
        let id = MolecularId::new(rand_u64());

        let mut archetype_pattern = [0.0; NUM_ARCHETYPES];
        for elem in &elements {
            let config = elem.configuration();
            for i in 0..NUM_ARCHETYPES {
                archetype_pattern[i] += config.archetype_vector[i];
            }
        }
        for val in archetype_pattern.iter_mut() {
            *val /= elements.len() as Float;
        }

        let geometry = if !elements.is_empty() {
            GeometryPrediction::for_central_atom(&elements[0], &elements[1..], 0)
        } else {
            GeometryPrediction::new()
        };

        let mut fg_resonance = FunctionalGroupResonance::new();
        fg_resonance.analyze(&elements, &bonds);

        let total_energy: Float = bonds.iter().map(|b| b.energy).sum();

        let coherence: Float = elements
            .iter()
            .map(|e| e.configuration().coherence)
            .sum::<Float>()
            / elements.len().max(1) as Float;

        Self {
            id,
            elements,
            bonds,
            geometry,
            functional_groups: fg_resonance,
            position,
            total_energy,
            coherence,
            archetype_pattern,
            formation_time,
        }
    }

    pub fn water(position: Position3D) -> Self {
        let o = ElementAttractorField::oxygen();
        let h1 = ElementAttractorField::hydrogen();
        let h2 = ElementAttractorField::hydrogen();

        let mut bond_formation = BondFormation::new().with_threshold(0.1);
        let bond1 = bond_formation.form_bond(&o, &h1, position.clone());
        let bond2 = bond_formation.form_bond(&o, &h2, position.clone());

        let bonds = vec![bond1.bonds, bond2.bonds]
            .into_iter()
            .flatten()
            .collect();

        Self::new(vec![o, h1, h2], bonds, position, 0.0)
    }

    pub fn methane(position: Position3D) -> Self {
        let c = ElementAttractorField::carbon();
        let hydrogens = vec![
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
            ElementAttractorField::hydrogen(),
        ];

        let mut bonds = Vec::new();
        let mut bond_formation = BondFormation::new().with_threshold(0.1);

        for h in &hydrogens {
            let result = bond_formation.form_bond(&c, h, position.clone());
            bonds.extend(result.bonds);
        }

        let mut all_elements = vec![c];
        all_elements.extend(hydrogens);

        Self::new(all_elements, bonds, position, 0.0)
    }

    pub fn atom_count(&self) -> usize {
        self.elements.len()
    }

    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    pub fn molecular_weight(&self) -> Float {
        self.elements
            .iter()
            .map(|e| e.atomic_number() as Float)
            .sum()
    }

    pub fn has_functional_group(&self, group: FunctionalGroup) -> bool {
        self.functional_groups.has_group(group)
    }

    pub fn is_stable(&self) -> bool {
        self.coherence > 0.5 && !self.bonds.is_empty()
    }

    pub fn is_organic(&self) -> bool {
        self.elements.iter().any(|e| e.atomic_number() == 6)
    }
}

#[derive(Debug, Clone)]
pub struct MolecularPlanetaryPair {
    pub molecule: MolecularManifestation,
    pub planet: PlanetaryEmergence,
    pub resonance: Float,
    pub scale_ratio: Float,
    pub archetype_alignment: Float,
}

impl MolecularPlanetaryPair {
    pub fn new(molecule: MolecularManifestation, planet: PlanetaryEmergence) -> Self {
        let resonance = Self::calculate_resonance(&molecule, &planet);
        let scale_ratio = planet.radius_km / (molecule.molecular_weight() * 0.1);
        let archetype_alignment = Self::calculate_alignment(&molecule, &planet);

        Self {
            molecule,
            planet,
            resonance,
            scale_ratio,
            archetype_alignment,
        }
    }

    fn calculate_resonance(
        molecule: &MolecularManifestation,
        planet: &PlanetaryEmergence,
    ) -> Float {
        let mol_arch = molecule.archetype_pattern;
        let planet_arch = planet.archetype_pattern;

        let dot: Float = mol_arch
            .iter()
            .zip(planet_arch.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_mol: Float = mol_arch.iter().map(|a| a * a).sum::<Float>().sqrt();
        let norm_planet: Float = planet_arch.iter().map(|b| b * b).sum::<Float>().sqrt();

        if norm_mol > 1e-10 && norm_planet > 1e-10 {
            dot / (norm_mol * norm_planet)
        } else {
            0.0
        }
    }

    fn calculate_alignment(
        molecule: &MolecularManifestation,
        planet: &PlanetaryEmergence,
    ) -> Float {
        (molecule.coherence + planet.coherence) / 2.0
    }

    pub fn is_strongly_coupled(&self) -> bool {
        self.resonance > 0.6 && self.archetype_alignment > 0.5
    }

    pub fn information_content(&self) -> Float {
        self.resonance * self.archetype_alignment * (self.scale_ratio.log10().abs() + 1.0)
    }
}

#[derive(Debug, Clone)]
pub struct MolecularPlanetarySystem {
    pairs: Vec<MolecularPlanetaryPair>,
    total_molecules: u32,
    total_planets: u32,
    emergence_events: Vec<(Float, String)>,
    coherence_history: Vec<Float>,
}

impl MolecularPlanetarySystem {
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
            total_molecules: 0,
            total_planets: 0,
            emergence_events: Vec::new(),
            coherence_history: Vec::new(),
        }
    }

    pub fn process_field(&mut self, field: &HolographicFieldState, timestamp: Float) {
        let root_position = field.root().position.clone();

        if let Some(planet) =
            PlanetaryEmergence::from_field(field, root_position.clone(), timestamp)
        {
            let water_pos =
                Position3D::new(root_position.x + 0.1, root_position.y, root_position.z);
            let molecule = MolecularManifestation::water(water_pos);

            let pair = MolecularPlanetaryPair::new(molecule, planet);
            self.pairs.push(pair);
            self.total_molecules += 1;
            self.total_planets += 1;

            self.emergence_events
                .push((timestamp, "Molecule-planet pair emerged".to_string()));
        }

        let coherence = field.global_coherence();
        self.coherence_history.push(coherence);
    }

    pub fn add_pair(&mut self, pair: MolecularPlanetaryPair) {
        self.total_molecules += 1;
        self.total_planets += 1;
        self.pairs.push(pair);
    }

    pub fn pairs(&self) -> &[MolecularPlanetaryPair] {
        &self.pairs
    }

    pub fn total_molecules(&self) -> u32 {
        self.total_molecules
    }

    pub fn total_planets(&self) -> u32 {
        self.total_planets
    }

    pub fn emergence_events(&self) -> &[(Float, String)] {
        &self.emergence_events
    }

    pub fn strongly_coupled_pairs(&self) -> Vec<&MolecularPlanetaryPair> {
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

    pub fn find_by_planet_type(&self, planet_type: PlanetType) -> Vec<&MolecularPlanetaryPair> {
        self.pairs
            .iter()
            .filter(|p| p.planet.planet_type == planet_type)
            .collect()
    }

    pub fn find_organic_molecules(&self) -> Vec<&MolecularManifestation> {
        self.pairs
            .iter()
            .filter(|p| p.molecule.is_organic())
            .map(|p| &p.molecule)
            .collect()
    }

    pub fn planet_type_distribution(&self) -> HashMap<PlanetType, u32> {
        let mut dist = HashMap::new();
        for pair in &self.pairs {
            *dist.entry(pair.planet.planet_type).or_insert(0) += 1;
        }
        dist
    }

    pub fn report(&self) -> String {
        format!(
            "Molecular-Planetary Emergence Report:\n\
             - Total pairs: {}\n\
             - Strongly coupled: {}\n\
             - Average resonance: {:.3}\n\
             - Organic molecules: {}\n\
             - Emergence events: {}",
            self.pairs.len(),
            self.strongly_coupled_pairs().len(),
            self.average_resonance(),
            self.find_organic_molecules().len(),
            self.emergence_events.len()
        )
    }
}

impl Default for MolecularPlanetarySystem {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_type_archetype() {
        let rocky = PlanetType::Rocky.archetype_signature();
        assert!(rocky[0] > 0.5);

        let gas_giant = PlanetType::GasGiant.archetype_signature();
        assert!(gas_giant[2] > 0.5);
    }

    #[test]
    fn test_planetary_emergence_creation() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Rocky, pos, 0.8, 1.0);

        assert_eq!(planet.planet_type, PlanetType::Rocky);
        assert!(planet.mass_earth_masses > 0.0);
        assert!(planet.radius_km > 0.0);
    }

    #[test]
    fn test_dominant_elements() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Rocky, pos, 0.8, 1.0);

        let dominant = planet.dominant_elements();
        assert_eq!(dominant.len(), 5);
    }

    #[test]
    fn test_molecular_manifestation_water() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let water = MolecularManifestation::water(pos);

        assert_eq!(water.atom_count(), 3);
        assert!(water.bond_count() >= 2);
    }

    #[test]
    fn test_molecular_manifestation_methane() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let methane = MolecularManifestation::methane(pos);

        assert_eq!(methane.atom_count(), 5);
        assert!(methane.bond_count() >= 4);
    }

    #[test]
    fn test_molecular_weight() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let water = MolecularManifestation::water(pos);

        let weight = water.molecular_weight();
        assert!(weight > 0.0);
    }

    #[test]
    fn test_is_organic() {
        let pos = Position3D::new(0.0, 0.0, 0.0);
        let methane = MolecularManifestation::methane(pos);

        assert!(methane.is_organic());
    }

    #[test]
    fn test_molecular_planetary_pair() {
        let mol_pos = Position3D::new(0.0, 0.0, 0.0);
        let molecule = MolecularManifestation::water(mol_pos);

        let planet_pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Ocean, planet_pos, 0.8, 1.0);

        let pair = MolecularPlanetaryPair::new(molecule, planet);

        assert!(pair.resonance >= 0.0 && pair.resonance <= 1.0);
        assert!(pair.scale_ratio > 0.0);
    }

    #[test]
    fn test_molecular_planetary_system_creation() {
        let system = MolecularPlanetarySystem::new();

        assert_eq!(system.total_molecules(), 0);
        assert_eq!(system.total_planets(), 0);
        assert!(system.pairs().is_empty());
    }

    #[test]
    fn test_add_pair() {
        let mut system = MolecularPlanetarySystem::new();

        let mol_pos = Position3D::new(0.0, 0.0, 0.0);
        let molecule = MolecularManifestation::water(mol_pos);

        let planet_pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Ocean, planet_pos, 0.8, 1.0);

        let pair = MolecularPlanetaryPair::new(molecule, planet);
        system.add_pair(pair);

        assert_eq!(system.total_molecules(), 1);
        assert_eq!(system.total_planets(), 1);
    }

    #[test]
    fn test_strongly_coupled_pairs() {
        let mut system = MolecularPlanetarySystem::new();

        let mol_pos = Position3D::new(0.0, 0.0, 0.0);
        let molecule = MolecularManifestation::water(mol_pos);

        let planet_pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Ocean, planet_pos, 0.8, 1.0);

        let pair = MolecularPlanetaryPair::new(molecule, planet);
        system.add_pair(pair);

        let coupled = system.strongly_coupled_pairs();
        assert!(!coupled.is_empty() || coupled.is_empty());
    }

    #[test]
    fn test_average_resonance() {
        let mut system = MolecularPlanetarySystem::new();

        let mol_pos = Position3D::new(0.0, 0.0, 0.0);
        let molecule = MolecularManifestation::water(mol_pos);

        let planet_pos = Position3D::new(0.0, 0.0, 0.0);
        let planet = PlanetaryEmergence::new(PlanetType::Ocean, planet_pos, 0.8, 1.0);

        let pair = MolecularPlanetaryPair::new(molecule, planet);
        system.add_pair(pair);

        let avg_res = system.average_resonance();
        assert!(avg_res >= 0.0 && avg_res <= 1.0);
    }

    #[test]
    fn test_planet_type_distribution() {
        let mut system = MolecularPlanetarySystem::new();

        let mol_pos = Position3D::new(0.0, 0.0, 0.0);
        let molecule = MolecularManifestation::water(mol_pos);

        for pt in [PlanetType::Rocky, PlanetType::GasGiant, PlanetType::Rocky] {
            let planet_pos = Position3D::new(0.0, 0.0, 0.0);
            let planet = PlanetaryEmergence::new(pt, planet_pos, 0.8, 1.0);
            let pair = MolecularPlanetaryPair::new(molecule.clone(), planet);
            system.add_pair(pair);
        }

        let dist = system.planet_type_distribution();
        assert_eq!(*dist.get(&PlanetType::Rocky).unwrap_or(&0), 2);
        assert_eq!(*dist.get(&PlanetType::GasGiant).unwrap_or(&0), 1);
    }

    #[test]
    fn test_report() {
        let system = MolecularPlanetarySystem::new();
        let report = system.report();

        assert!(report.contains("Molecular-Planetary"));
        assert!(report.contains("pairs"));
    }
}
