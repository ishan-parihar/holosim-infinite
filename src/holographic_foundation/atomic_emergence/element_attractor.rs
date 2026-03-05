//! Element Attractor Field: Elements as Stable Attractor Configurations
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 8:
//! "The periodic table is the map of these stable attractor fields, each
//!  corresponding to unique quantum number combinations (n, l, m, s)."
//!
//! Elements are NOT fundamental particles. They are stable attractor states
//! that the holographic field naturally falls into. Each element corresponds
//! to a unique combination of:
//! - Archetype activation pattern (determines chemical properties)
//! - Quantum numbers (determines shell structure)
//! - Field coherence level (determines stability)
//!
//! Key Insight: Why do protons have positive charge and electrons negative?
//! This emerges from the archetype patterns:
//! - Proton: Mind-complex dominant (A1-A7) → outward expression → positive
//! - Electron: Spirit-complex dominant (A15-A21) → inward reception → negative

use crate::types::Float;
use std::fmt;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::quantum_consciousness::quantum_numbers::QuantumNumberSet;
use super::attractor_field::{
    AttractorField, FieldConfiguration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementId(u32);

impl ElementId {
    pub fn new(atomic_number: u32) -> Self {
        Self(atomic_number)
    }

    pub fn atomic_number(&self) -> u32 {
        self.0
    }

    pub fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementIdentity {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Other(u32),
}

impl ElementIdentity {
    pub fn from_atomic_number(z: u32) -> Self {
        match z {
            1 => ElementIdentity::Hydrogen,
            2 => ElementIdentity::Helium,
            3 => ElementIdentity::Lithium,
            4 => ElementIdentity::Beryllium,
            5 => ElementIdentity::Boron,
            6 => ElementIdentity::Carbon,
            7 => ElementIdentity::Nitrogen,
            8 => ElementIdentity::Oxygen,
            9 => ElementIdentity::Fluorine,
            10 => ElementIdentity::Neon,
            _ => ElementIdentity::Other(z),
        }
    }

    pub fn atomic_number(&self) -> u32 {
        match self {
            ElementIdentity::Hydrogen => 1,
            ElementIdentity::Helium => 2,
            ElementIdentity::Lithium => 3,
            ElementIdentity::Beryllium => 4,
            ElementIdentity::Boron => 5,
            ElementIdentity::Carbon => 6,
            ElementIdentity::Nitrogen => 7,
            ElementIdentity::Oxygen => 8,
            ElementIdentity::Fluorine => 9,
            ElementIdentity::Neon => 10,
            ElementIdentity::Other(z) => *z,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            ElementIdentity::Hydrogen => "H",
            ElementIdentity::Helium => "He",
            ElementIdentity::Lithium => "Li",
            ElementIdentity::Beryllium => "Be",
            ElementIdentity::Boron => "B",
            ElementIdentity::Carbon => "C",
            ElementIdentity::Nitrogen => "N",
            ElementIdentity::Oxygen => "O",
            ElementIdentity::Fluorine => "F",
            ElementIdentity::Neon => "Ne",
            ElementIdentity::Other(z) => match z {
                11 => "Na",
                12 => "Mg",
                13 => "Al",
                14 => "Si",
                15 => "P",
                16 => "S",
                17 => "Cl",
                18 => "Ar",
                19 => "K",
                20 => "Ca",
                26 => "Fe",
                29 => "Cu",
                30 => "Zn",
                79 => "Au",
                82 => "Pb",
                _ => "?",
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ElementIdentity::Hydrogen => "Hydrogen",
            ElementIdentity::Helium => "Helium",
            ElementIdentity::Lithium => "Lithium",
            ElementIdentity::Beryllium => "Beryllium",
            ElementIdentity::Boron => "Boron",
            ElementIdentity::Carbon => "Carbon",
            ElementIdentity::Nitrogen => "Nitrogen",
            ElementIdentity::Oxygen => "Oxygen",
            ElementIdentity::Fluorine => "Fluorine",
            ElementIdentity::Neon => "Neon",
            ElementIdentity::Other(z) => match z {
                11 => "Sodium",
                12 => "Magnesium",
                13 => "Aluminum",
                14 => "Silicon",
                15 => "Phosphorus",
                16 => "Sulfur",
                17 => "Chlorine",
                18 => "Argon",
                19 => "Potassium",
                20 => "Calcium",
                26 => "Iron",
                29 => "Copper",
                30 => "Zinc",
                79 => "Gold",
                82 => "Lead",
                _ => "Unknown",
            },
        }
    }

    pub fn is_noble_gas(&self) -> bool {
        matches!(self.atomic_number(), 2 | 10 | 18 | 36 | 54 | 86 | 118)
    }

    pub fn is_metal(&self) -> bool {
        let z = self.atomic_number();
        matches!(z, 3..=4 | 11..=13 | 19..=31 | 37..=50 | 55..=83 | 87..=116)
    }

    pub fn is_nonmetal(&self) -> bool {
        let z = self.atomic_number();
        matches!(z, 1 | 6..=8 | 15..=16 | 34) || self.is_noble_gas()
    }

    pub fn is_halogen(&self) -> bool {
        matches!(self.atomic_number(), 9 | 17 | 35 | 53 | 85 | 117)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargeConfiguration {
    pub proton_charge: Float,
    pub electron_charge: Float,
    pub net_charge: Float,
    pub charge_radius: Float,
}

impl ChargeConfiguration {
    pub fn neutral(atomic_number: u32) -> Self {
        Self {
            proton_charge: atomic_number as Float,
            electron_charge: -(atomic_number as Float),
            net_charge: 0.0,
            charge_radius: 1e-10 * (atomic_number as Float).powf(1.0 / 3.0),
        }
    }

    pub fn ionized(atomic_number: u32, ionization_state: i32) -> Self {
        let electrons = atomic_number as i32 - ionization_state;
        Self {
            proton_charge: atomic_number as Float,
            electron_charge: -(electrons as Float),
            net_charge: ionization_state as Float,
            charge_radius: 1e-10 * (atomic_number as Float).powf(1.0 / 3.0),
        }
    }

    pub fn is_neutral(&self) -> bool {
        self.net_charge.abs() < 1e-10
    }

    pub fn is_cation(&self) -> bool {
        self.net_charge > 0.0
    }

    pub fn is_anion(&self) -> bool {
        self.net_charge < 0.0
    }

    pub fn electronegativity_factor(&self) -> Float {
        if self.is_neutral() {
            1.0
        } else if self.is_cation() {
            0.8 + self.net_charge * 0.1
        } else {
            1.2 - self.net_charge.abs() * 0.1
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementAttractorField {
    id: ElementId,
    identity: ElementIdentity,
    configuration: FieldConfiguration,
    attractor: AttractorField,
    charge: ChargeConfiguration,
    electronegativity: Float,
    atomic_radius: Float,
    ionization_energy: Float,
    electron_affinity: Float,
    formation_energy: Float,
}

impl ElementAttractorField {
    pub fn from_atomic_number(atomic_number: u32) -> Self {
        let archetype_pattern = Self::archetype_pattern_for_element(atomic_number);
        Self::from_archetype_pattern(atomic_number, archetype_pattern)
    }

    pub fn from_archetype_pattern(atomic_number: u32, archetype: [Float; NUM_ARCHETYPES]) -> Self {
        let id = ElementId::new(atomic_number);
        let identity = ElementIdentity::from_atomic_number(atomic_number);
        let configuration = FieldConfiguration::new(archetype);
        let attractor = AttractorField::new(configuration.clone());
        let charge = ChargeConfiguration::neutral(atomic_number);

        let electronegativity = Self::calculate_electronegativity(&archetype, atomic_number);
        let atomic_radius = Self::calculate_atomic_radius(&archetype, atomic_number);
        let ionization_energy = Self::calculate_ionization_energy(&archetype, atomic_number);
        let electron_affinity = Self::calculate_electron_affinity(&archetype, atomic_number);
        let formation_energy = Self::calculate_formation_energy(&archetype, atomic_number);

        Self {
            id,
            identity,
            configuration,
            attractor,
            charge,
            electronegativity,
            atomic_radius,
            ionization_energy,
            electron_affinity,
            formation_energy,
        }
    }

    fn archetype_pattern_for_element(atomic_number: u32) -> [Float; NUM_ARCHETYPES] {
        let mut archetype = [0.5; NUM_ARCHETYPES];

        let period = Self::get_period(atomic_number);
        let group = Self::get_group(atomic_number);

        // Mind archetypes (A1-A7) - determine structure
        archetype[0] = if group == 18 {
            0.95
        } else {
            0.3 + group as Float / 18.0 * 0.4
        };
        archetype[1] = 0.3 + period as Float * 0.1;
        archetype[2] = if group == 18 {
            0.5
        } else {
            0.2 + group as Float / 18.0 * 0.7
        };
        archetype[3] = 0.4 + period as Float * 0.05;
        archetype[4] = 0.5;
        archetype[5] = if group == 18 { 0.2 } else { 0.6 };
        archetype[6] = if group == 18 {
            1.0
        } else {
            0.4 + (1.0 - (group as Float - 9.0).abs() / 9.0) * 0.3
        };

        // Body archetypes (A8-A14) - mirror mind
        for i in 0..7 {
            archetype[7 + i] = archetype[i] * 0.8;
        }

        // Spirit archetypes (A15-A21) - determine charge characteristics
        for i in 0..7 {
            archetype[14 + i] = archetype[i] * 0.7;
        }

        // A22 (Choice) - free will factor
        archetype[21] = 0.5;

        // Scale with atomic number
        let scale = (atomic_number as Float / 60.0).clamp(0.5, 1.5);
        for coeff in archetype.iter_mut() {
            *coeff = (*coeff * scale).clamp(0.1, 1.0);
        }

        archetype
    }

    fn get_period(z: u32) -> u32 {
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

    fn get_group(z: u32) -> u32 {
        match z {
            1 => 1,
            2 => 18,
            3..=4 => z - 2,
            5..=10 => z + 8,
            11..=12 => z - 10,
            13..=18 => z,
            19..=36 => {
                // All transition metals have the same formula
                z - 18
            }
            37..=54 => {
                // All elements in this range have the same formula
                z - 36
            }
            _ => 1,
        }
    }

    fn calculate_electronegativity(archetype: &[Float; NUM_ARCHETYPES], _z: u32) -> Float {
        let catalyst = archetype[2];
        let body_factor: Float = archetype[7..14].iter().sum::<Float>() / 7.0;
        let matrix = archetype[0];

        let base_en = if catalyst > 0.5 {
            2.0 + (catalyst - 0.5) * 4.0
        } else {
            0.7 + catalyst * 2.6
        };

        (base_en * body_factor * (0.5 + matrix * 0.5))
            .clamp(0.7, 4.0)
    }

    fn calculate_atomic_radius(archetype: &[Float; NUM_ARCHETYPES], z: u32) -> Float {
        let matrix_avg = (archetype[0] + archetype[7] + archetype[14]) / 3.0;
        let period = Self::get_period(z);

        30.0 + matrix_avg * 150.0 + period as Float * 20.0
    }

    fn calculate_ionization_energy(archetype: &[Float; NUM_ARCHETYPES], _z: u32) -> Float {
        let matrix = archetype[0];
        let great_way = archetype[6];

        (matrix + great_way) / 2.0 * 12.0 + 3.9
    }

    fn calculate_electron_affinity(archetype: &[Float; NUM_ARCHETYPES], _z: u32) -> Float {
        let catalyst = archetype[2];
        let experience = archetype[3];

        catalyst * experience * 3.6
    }

    fn calculate_formation_energy(archetype: &[Float; NUM_ARCHETYPES], z: u32) -> Float {
        let mean: Float = archetype.iter().sum::<Float>() / NUM_ARCHETYPES as Float;
        let variance: Float =
            archetype.iter().map(|c| (c - mean).powi(2)).sum::<Float>() / NUM_ARCHETYPES as Float;
        let stability = 1.0 - variance.sqrt().min(1.0);

        13.6 * (z as Float).powf(2.0) * stability
    }

    pub fn hydrogen() -> Self {
        Self::from_atomic_number(1)
    }

    pub fn helium() -> Self {
        Self::from_atomic_number(2)
    }

    pub fn carbon() -> Self {
        Self::from_atomic_number(6)
    }

    pub fn oxygen() -> Self {
        Self::from_atomic_number(8)
    }

    pub fn iron() -> Self {
        Self::from_atomic_number(26)
    }

    pub fn gold() -> Self {
        Self::from_atomic_number(79)
    }

    pub fn id(&self) -> &ElementId {
        &self.id
    }

    pub fn atomic_number(&self) -> u32 {
        self.id.atomic_number()
    }

    pub fn identity(&self) -> &ElementIdentity {
        &self.identity
    }

    pub fn symbol(&self) -> &'static str {
        self.identity.symbol()
    }

    pub fn name(&self) -> &'static str {
        self.identity.name()
    }

    pub fn configuration(&self) -> &FieldConfiguration {
        &self.configuration
    }

    pub fn quantum_numbers(&self) -> &QuantumNumberSet {
        self.configuration.quantum_numbers()
    }

    pub fn attractor(&self) -> &AttractorField {
        &self.attractor
    }

    pub fn attractor_mut(&mut self) -> &mut AttractorField {
        &mut self.attractor
    }

    pub fn charge(&self) -> &ChargeConfiguration {
        &self.charge
    }

    pub fn electronegativity(&self) -> Float {
        self.electronegativity
    }

    pub fn atomic_radius(&self) -> Float {
        self.atomic_radius
    }

    pub fn ionization_energy(&self) -> Float {
        self.ionization_energy
    }

    pub fn electron_affinity(&self) -> Float {
        self.electron_affinity
    }

    pub fn formation_energy(&self) -> Float {
        self.formation_energy
    }

    pub fn is_noble_gas(&self) -> bool {
        self.identity.is_noble_gas()
    }

    pub fn is_metal(&self) -> bool {
        self.identity.is_metal()
    }

    pub fn is_nonmetal(&self) -> bool {
        self.identity.is_nonmetal()
    }

    pub fn can_form_at_coherence(&self, field_coherence: Float) -> bool {
        let threshold = 0.3 + self.atomic_number() as Float / 118.0 * 0.4;
        field_coherence >= threshold
    }

    pub fn bonding_affinity(&self, other: &ElementAttractorField) -> Float {
        let similarity = self
            .configuration
            .archetype_similarity(&other.configuration);
        let en_diff = (self.electronegativity - other.electronegativity).abs();
        let en_factor = 1.0 - en_diff / 4.0;

        similarity * en_factor
    }

    pub fn ionize(&mut self, ionization_state: i32) {
        self.charge = ChargeConfiguration::ionized(self.atomic_number(), ionization_state);
    }
}

impl fmt::Display for ElementAttractorField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) Z={} EN={:.2} radius={:.1}pm",
            self.symbol(),
            self.name(),
            self.atomic_number(),
            self.electronegativity,
            self.atomic_radius
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_id() {
        let id = ElementId::new(6);
        assert_eq!(id.atomic_number(), 6);
    }

    #[test]
    fn test_hydrogen_creation() {
        let h = ElementAttractorField::hydrogen();

        assert_eq!(h.atomic_number(), 1);
        assert_eq!(h.symbol(), "H");
        assert_eq!(h.name(), "Hydrogen");
        assert!(!h.is_noble_gas());
    }

    #[test]
    fn test_helium_noble_gas() {
        let he = ElementAttractorField::helium();

        assert_eq!(he.atomic_number(), 2);
        assert!(he.is_noble_gas());
    }

    #[test]
    fn test_carbon_properties() {
        let c = ElementAttractorField::carbon();

        assert_eq!(c.atomic_number(), 6);
        assert!(c.electronegativity() > 0.0);
        assert!(c.is_nonmetal());
    }

    #[test]
    fn test_iron_metal() {
        let fe = ElementAttractorField::iron();

        assert_eq!(fe.atomic_number(), 26);
        assert!(fe.is_metal());
    }

    #[test]
    fn test_charge_configuration() {
        let neutral = ChargeConfiguration::neutral(6);
        assert!(neutral.is_neutral());

        let cation = ChargeConfiguration::ionized(6, 2);
        assert!(cation.is_cation());

        let anion = ChargeConfiguration::ionized(8, -2);
        assert!(anion.is_anion());
    }

    #[test]
    fn test_bonding_affinity() {
        let h = ElementAttractorField::hydrogen();
        let o = ElementAttractorField::oxygen();

        let affinity = h.bonding_affinity(&o);
        assert!(affinity > 0.0 && affinity <= 1.0);
    }

    #[test]
    fn test_formation_threshold() {
        let h = ElementAttractorField::hydrogen();
        let u = ElementAttractorField::from_atomic_number(92);

        assert!(h.can_form_at_coherence(0.35));
        assert!(!u.can_form_at_coherence(0.35));
    }

    #[test]
    fn test_ionization() {
        let mut na = ElementAttractorField::from_atomic_number(11);
        assert!(na.charge().is_neutral());

        na.ionize(1);
        assert!(na.charge().is_cation());
    }

    #[test]
    fn test_element_display() {
        let c = ElementAttractorField::carbon();
        let display = format!("{}", c);

        assert!(display.contains("C"));
        assert!(display.contains("Carbon"));
    }
}
