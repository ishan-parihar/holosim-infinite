//! Periodic Table as Attractor Basin Map
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 8:
//! "The periodic table is the map of these stable attractor fields, each
//!  corresponding to unique quantum number combinations (n, l, m, s)."
//!
//! The periodic table is NOT a chart of separate elements. It is a MAP
//! of the attractor basins in field configuration space. Each position
//! represents a stable configuration that the field can fall into.
//!
//! Key Insight: Elements don't "have" positions in the periodic table.
//! The periodic table positions ARE the coordinates in attractor space.

use crate::types::Float;
use std::collections::HashMap;
use std::fmt;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::quantum_consciousness::quantum_numbers::{QuantumNumberSet, Spin};
use super::attractor_field::{AttractorBasin, AttractorId, AttractorStability, FieldConfiguration};
use super::element_attractor::{
    ChargeConfiguration, ElementAttractorField, ElementId, ElementIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementPosition {
    pub period: u32,
    pub group: u32,
    pub block: Block,
}

impl ElementPosition {
    pub fn new(period: u32, group: u32, block: Block) -> Self {
        Self {
            period,
            group,
            block,
        }
    }

    pub fn from_atomic_number(z: u32) -> Self {
        let period = Self::calculate_period(z);
        let group = Self::calculate_group(z);
        let block = Self::calculate_block(z);

        Self {
            period,
            group,
            block,
        }
    }

    fn calculate_period(z: u32) -> u32 {
        match z {
            1..=2 => 1,
            3..=10 => 2,
            11..=18 => 3,
            19..=36 => 4,
            37..=54 => 5,
            55..=86 => 6,
            87..=118 => 7,
            _ => 7,
        }
    }

    fn calculate_group(z: u32) -> u32 {
        match z {
            1 => 1,
            2 => 18,
            3 => 1,
            4 => 2,
            5 => 13,
            6 => 14,
            7 => 15,
            8 => 16,
            9 => 17,
            10 => 18,
            11 => 1,
            12 => 2,
            13 => 13,
            14 => 14,
            15 => 15,
            16 => 16,
            17 => 17,
            18 => 18,
            19..=20 => z - 18,
            21..=30 => z - 18,
            31..=36 => z - 18,
            37..=38 => z - 36,
            39..=48 => z - 36,
            49..=54 => z - 36,
            55..=56 => z - 54,
            57..=71 => 3,
            72..=80 => z - 68,
            81..=86 => z - 68,
            87..=88 => z - 86,
            89..=103 => 3,
            104..=112 => z - 100,
            113..=118 => z - 100,
            _ => 1,
        }
    }

    fn calculate_block(z: u32) -> Block {
        match z {
            1..=2 => Block::S,
            3..=4 => Block::S,
            5..=10 => Block::P,
            11..=12 => Block::S,
            13..=18 => Block::P,
            19..=20 => Block::S,
            21..=30 => Block::D,
            31..=36 => Block::P,
            37..=38 => Block::S,
            39..=48 => Block::D,
            49..=54 => Block::P,
            55..=56 => Block::S,
            57..=70 => Block::F,
            71..=80 => Block::D,
            81..=86 => Block::P,
            87..=88 => Block::S,
            89..=102 => Block::F,
            103..=112 => Block::D,
            113..=118 => Block::P,
            _ => Block::S,
        }
    }

    pub fn is_noble_gas(&self) -> bool {
        self.group == 18
    }

    pub fn is_alkali_metal(&self) -> bool {
        self.group == 1 && self.period > 1
    }

    pub fn is_alkaline_earth(&self) -> bool {
        self.group == 2
    }

    pub fn is_halogen(&self) -> bool {
        self.group == 17
    }

    pub fn is_transition_metal(&self) -> bool {
        self.block == Block::D && self.period >= 4
    }

    pub fn is_lanthanide(&self) -> bool {
        self.period == 6 && self.block == Block::F
    }

    pub fn is_actinide(&self) -> bool {
        self.period == 7 && self.block == Block::F
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    S,
    P,
    D,
    F,
}

impl Block {
    pub fn max_electrons(&self) -> u32 {
        match self {
            Block::S => 2,
            Block::P => 6,
            Block::D => 10,
            Block::F => 14,
        }
    }

    pub fn orbital_label(&self) -> &'static str {
        match self {
            Block::S => "s",
            Block::P => "p",
            Block::D => "d",
            Block::F => "f",
        }
    }

    pub fn azimuthal_quantum_number(&self) -> u32 {
        match self {
            Block::S => 0,
            Block::P => 1,
            Block::D => 2,
            Block::F => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShellConfiguration {
    pub principal: Vec<u32>,
    pub azimuthal: Vec<u32>,
    pub occupancy: Vec<u32>,
    pub total_electrons: u32,
}

impl ShellConfiguration {
    pub fn from_atomic_number(z: u32) -> Self {
        let mut principal = Vec::new();
        let mut azimuthal = Vec::new();
        let mut occupancy = Vec::new();
        let mut remaining = z;

        let filling_order = vec![
            (1, 0, 2),
            (2, 0, 2),
            (2, 1, 6),
            (3, 0, 2),
            (3, 1, 6),
            (4, 0, 2),
            (3, 2, 10),
            (4, 1, 6),
            (5, 0, 2),
            (4, 2, 10),
            (5, 1, 6),
            (6, 0, 2),
            (4, 3, 14),
            (5, 2, 10),
            (6, 1, 6),
            (7, 0, 2),
            (5, 3, 14),
            (6, 2, 10),
            (7, 1, 6),
        ];

        for (n, l, max) in filling_order {
            if remaining == 0 {
                break;
            }

            let electrons = remaining.min(max);
            principal.push(n);
            azimuthal.push(l);
            occupancy.push(electrons);
            remaining -= electrons;
        }

        Self {
            principal,
            azimuthal,
            occupancy,
            total_electrons: z,
        }
    }

    pub fn valence_electrons(&self) -> u32 {
        let period = self.principal.last().copied().unwrap_or(1);

        let mut valence = 0;
        for (i, &n) in self.principal.iter().enumerate().rev() {
            if n == period {
                valence += self.occupancy[i];
            }
        }

        valence.min(8)
    }

    pub fn core_electrons(&self) -> u32 {
        self.total_electrons - self.valence_electrons()
    }

    pub fn electron_configuration_string(&self) -> String {
        let mut config = String::new();

        for (i, (&n, &l)) in self.principal.iter().zip(self.azimuthal.iter()).enumerate() {
            if i > 0 {
                config.push(' ');
            }

            let orbital = match l {
                0 => "s",
                1 => "p",
                2 => "d",
                3 => "f",
                _ => "?",
            };

            config.push_str(&format!("{}{}{}", n, orbital, self.occupancy[i]));
        }

        config
    }

    pub fn shell_count(&self) -> usize {
        self.principal.len()
    }
}

#[derive(Debug, Clone)]
pub struct PeriodicTableAttractors {
    elements: Vec<ElementAttractorField>,
    attractor_basins: HashMap<AttractorId, AttractorBasin>,
    position_map: HashMap<u32, ElementPosition>,
    shell_configurations: HashMap<u32, ShellConfiguration>,
}

impl PeriodicTableAttractors {
    pub fn generate() -> Self {
        let mut elements = Vec::new();
        let mut attractor_basins = HashMap::new();
        let mut position_map = HashMap::new();
        let mut shell_configurations = HashMap::new();

        for z in 1..=118 {
            let element = ElementAttractorField::from_atomic_number(z);
            let position = ElementPosition::from_atomic_number(z);
            let shells = ShellConfiguration::from_atomic_number(z);

            let attractor_id = *element.attractor().id();
            let basin = element.attractor().basin().clone();

            attractor_basins.insert(attractor_id, basin);
            position_map.insert(z, position);
            shell_configurations.insert(z, shells);
            elements.push(element);
        }

        Self {
            elements,
            attractor_basins,
            position_map,
            shell_configurations,
        }
    }

    pub fn get(&self, atomic_number: u32) -> Option<&ElementAttractorField> {
        if atomic_number == 0 || atomic_number > 118 {
            None
        } else {
            self.elements.get((atomic_number - 1) as usize)
        }
    }

    pub fn get_mut(&mut self, atomic_number: u32) -> Option<&mut ElementAttractorField> {
        if atomic_number == 0 || atomic_number > 118 {
            None
        } else {
            self.elements.get_mut((atomic_number - 1) as usize)
        }
    }

    pub fn find_by_symbol(&self, symbol: &str) -> Option<&ElementAttractorField> {
        self.elements.iter().find(|e| e.symbol() == symbol)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&ElementAttractorField> {
        self.elements
            .iter()
            .find(|e| e.name().eq_ignore_ascii_case(name))
    }

    pub fn position(&self, atomic_number: u32) -> Option<&ElementPosition> {
        self.position_map.get(&atomic_number)
    }

    pub fn shell_configuration(&self, atomic_number: u32) -> Option<&ShellConfiguration> {
        self.shell_configurations.get(&atomic_number)
    }

    pub fn attractor_basin(&self, id: &AttractorId) -> Option<&AttractorBasin> {
        self.attractor_basins.get(id)
    }

    pub fn elements_formable_at(&self, coherence: Float) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| e.can_form_at_coherence(coherence))
            .collect()
    }

    pub fn noble_gases(&self) -> Vec<&ElementAttractorField> {
        self.elements.iter().filter(|e| e.is_noble_gas()).collect()
    }

    pub fn metals(&self) -> Vec<&ElementAttractorField> {
        self.elements.iter().filter(|e| e.is_metal()).collect()
    }

    pub fn nonmetals(&self) -> Vec<&ElementAttractorField> {
        self.elements.iter().filter(|e| e.is_nonmetal()).collect()
    }

    pub fn halogens(&self) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| e.identity().is_halogen())
            .collect()
    }

    pub fn elements_by_period(&self, period: u32) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| {
                self.position(e.atomic_number())
                    .map(|p| p.period == period)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn elements_by_group(&self, group: u32) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| {
                self.position(e.atomic_number())
                    .map(|p| p.group == group)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn elements_by_block(&self, block: Block) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| {
                self.position(e.atomic_number())
                    .map(|p| p.block == block)
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn find_nearest_element(
        &self,
        config: &FieldConfiguration,
    ) -> Option<&ElementAttractorField> {
        self.elements.iter().min_by(|a, b| {
            let dist_a = a.configuration().distance_to(config);
            let dist_b = b.configuration().distance_to(config);
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn find_elements_in_basin(&self, basin: &AttractorBasin) -> Vec<&ElementAttractorField> {
        self.elements
            .iter()
            .filter(|e| basin.contains(e.configuration()))
            .collect()
    }

    pub fn transition_elements(
        &self,
        from_z: u32,
        to_z: u32,
        energy_available: Float,
    ) -> Vec<&ElementAttractorField> {
        let from = match self.get(from_z) {
            Some(e) => e,
            None => return Vec::new(),
        };

        let from_id = from.attractor().id();

        self.elements
            .iter()
            .filter(|e| {
                let to_id = e.attractor().id();
                if let Some(basin) = self.attractor_basins.get(from_id) {
                    basin.can_transition_to(to_id, energy_available)
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ElementAttractorField> {
        self.elements.iter()
    }

    pub fn total_attractor_depth(&self) -> Float {
        self.attractor_basins.values().map(|b| b.depth).sum()
    }

    pub fn average_stability(&self) -> Float {
        let stable_count = self
            .elements
            .iter()
            .filter(|e| e.attractor().is_stable())
            .count();

        stable_count as Float / self.elements.len() as Float
    }

    pub fn formation_energy_range(&self) -> (Float, Float) {
        let energies: Vec<Float> = self.elements.iter().map(|e| e.formation_energy()).collect();

        let min = energies.iter().fold(Float::INFINITY, |a, &b| a.min(b));
        let max = energies.iter().fold(Float::NEG_INFINITY, |a, &b| a.max(b));

        (min, max)
    }

    pub fn electronegativity_range(&self) -> (Float, Float) {
        let en_values: Vec<Float> = self
            .elements
            .iter()
            .map(|e| e.electronegativity())
            .collect();

        let min = en_values.iter().fold(Float::INFINITY, |a, &b| a.min(b));
        let max = en_values.iter().fold(Float::NEG_INFINITY, |a, &b| a.max(b));

        (min, max)
    }
}

impl Default for PeriodicTableAttractors {
    fn default() -> Self {
        Self::generate()
    }
}

impl fmt::Display for PeriodicTableAttractors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Periodic Table (118 elements)")?;
        writeln!(f, "Noble gases: {}", self.noble_gases().len())?;
        writeln!(f, "Metals: {}", self.metals().len())?;
        writeln!(f, "Nonmetals: {}", self.nonmetals().len())?;
        writeln!(f, "Average stability: {:.2}", self.average_stability())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_periodic_table_generation() {
        let pt = PeriodicTableAttractors::generate();
        assert_eq!(pt.len(), 118);
    }

    #[test]
    fn test_get_element() {
        let pt = PeriodicTableAttractors::generate();

        let h = pt.get(1);
        assert!(h.is_some());
        assert_eq!(h.unwrap().symbol(), "H");

        let c = pt.get(6);
        assert!(c.is_some());
        assert_eq!(c.unwrap().symbol(), "C");

        let none = pt.get(0);
        assert!(none.is_none());

        let none2 = pt.get(119);
        assert!(none2.is_none());
    }

    #[test]
    fn test_find_by_symbol() {
        let pt = PeriodicTableAttractors::generate();

        assert!(pt.find_by_symbol("H").is_some());
        assert!(pt.find_by_symbol("C").is_some());
        assert!(pt.find_by_symbol("Au").is_some());
        assert!(pt.find_by_symbol("XX").is_none());
    }

    #[test]
    fn test_noble_gases() {
        let pt = PeriodicTableAttractors::generate();
        let nobles = pt.noble_gases();

        assert_eq!(nobles.len(), 7);
        assert!(nobles.iter().all(|e| e.is_noble_gas()));
    }

    #[test]
    fn test_element_position() {
        let pos = ElementPosition::from_atomic_number(6);
        assert_eq!(pos.period, 2);
        assert_eq!(pos.group, 14);
        assert_eq!(pos.block, Block::P);
    }

    #[test]
    fn test_shell_configuration() {
        let shells = ShellConfiguration::from_atomic_number(6);

        assert_eq!(shells.total_electrons, 6);
        assert_eq!(shells.valence_electrons(), 4);
        assert!(shells.electron_configuration_string().contains("1s2"));
    }

    #[test]
    fn test_elements_by_period() {
        let pt = PeriodicTableAttractors::generate();

        let period1 = pt.elements_by_period(1);
        assert_eq!(period1.len(), 2);

        let period2 = pt.elements_by_period(2);
        assert_eq!(period2.len(), 8);
    }

    #[test]
    fn test_elements_by_group() {
        let pt = PeriodicTableAttractors::generate();

        let group1 = pt.elements_by_group(1);
        assert!(group1.len() >= 2);

        let group18 = pt.elements_by_group(18);
        assert_eq!(group18.len(), 7);
    }

    #[test]
    fn test_formable_elements() {
        let pt = PeriodicTableAttractors::generate();

        let low_coherence = pt.elements_formable_at(0.35);
        assert!(low_coherence.len() < 118);

        let high_coherence = pt.elements_formable_at(0.9);
        assert_eq!(high_coherence.len(), 118);
    }

    #[test]
    fn test_find_nearest_element() {
        let pt = PeriodicTableAttractors::generate();

        let h_config = FieldConfiguration::hydrogen_configuration();
        let nearest = pt.find_nearest_element(&h_config);

        assert!(nearest.is_some());
    }

    #[test]
    fn test_electronegativity_range() {
        let pt = PeriodicTableAttractors::generate();
        let (min, max) = pt.electronegativity_range();

        assert!(min >= 0.7);
        assert!(max <= 4.0);
    }

    #[test]
    fn test_block_elements() {
        let pt = PeriodicTableAttractors::generate();

        let s_block = pt.elements_by_block(Block::S);
        assert!(s_block.len() >= 2);

        let p_block = pt.elements_by_block(Block::P);
        assert!(p_block.len() >= 6);
    }

    #[test]
    fn test_periodic_table_display() {
        let pt = PeriodicTableAttractors::generate();
        let display = format!("{}", pt);

        assert!(display.contains("118"));
        assert!(display.contains("Noble gases"));
    }
}
