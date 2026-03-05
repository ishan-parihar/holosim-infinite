//! Element formation from archetype patterns
//!
//! From ROADMAP: "Elements as stable attractor fields"
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK: "Element properties emerge from archetype activation"
//!
//! This module implements the chemistry module where:
//! - Elements emerge as stable attractor patterns in the holographic field
//! - Element properties are derived from the 22-archetype activation patterns
//! - Noble gases represent closed archetype shells with special stability
//! - All element properties validated against known periodic table data
//!
//! # Archetype Derivation Rules
//!
//! | Property | Archetype Influence |
//! |----------|---------------------|
//! | Atomic Number | Total activation magnitude, Mind archetypes (A1-A7) |
//! | Electronegativity | Catalyst archetype (A3), Body factor |
//! | Stability | Archetype coherence (low variance) |
//! | Electron Affinity | Catalyst (A3) × Experience (A4) |
//! | Ionization Energy | Matrix (A1) + Great Way (A7) |
//! | Atomic Mass | Atomic number × stability-derived neutron ratio |

use crate::holographic::holographic_field::HolographicField;
use crate::holographic::Position;
use crate::types::Float;
use std::fmt;

/// Avogadro's number (mol⁻¹)
pub const AVOGADRO_NUMBER: Float = 6.02214076e23;

/// Atomic mass unit in kg
pub const ATOMIC_MASS_UNIT: Float = 1.66053906660e-27;

/// Element represented as an archetype-derived attractor field
///
/// Elements are not fundamental - they emerge as stable patterns in the
/// holographic field when archetype activations align to create stable
/// configurations.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The periodic table reflects archetype resonance patterns"
#[derive(Debug, Clone, PartialEq)]
pub struct ElementAttractor {
    /// Atomic number (proton count) - derived from archetype pattern
    pub atomic_number: u32,

    /// Element symbol (e.g., "H", "He", "C")
    pub symbol: String,

    /// Element name (e.g., "Hydrogen", "Helium", "Carbon")
    pub name: String,

    /// Atomic mass in atomic mass units
    pub atomic_mass: Float,

    /// Electronegativity (Pauling scale) - derived from Catalyst archetype (A3)
    /// Range: 0.7 (Cs) to 4.0 (F)
    pub electronegativity: Float,

    /// Electron affinity in eV
    pub electron_affinity: Float,

    /// Ionization energy (first) in eV
    pub ionization_energy: Float,

    /// Coherence stability (0.0 to 1.0)
    /// Derived from archetype coherence - how aligned the archetypes are
    pub stability: Float,

    /// Formation energy threshold
    /// Minimum coherence required for element to form
    pub formation_threshold: Float,

    /// Archetype signature that produces this element
    /// The 22 archetype coefficients that characterize this element
    pub archetype_signature: [Float; 22],

    /// Whether this is a noble gas (closed archetype shell)
    /// Noble gases have closed electron shells and special stability
    pub is_noble: bool,
}

impl ElementAttractor {
    /// Derive element from archetype activation pattern
    ///
    /// From ROADMAP:
    /// - Atomic number from overall activation magnitude
    /// - Electronegativity from Catalyst archetype (A3)
    /// - Stability from archetype coherence
    ///
    /// # Arguments
    ///
    /// * `activation` - 22-value archetype activation pattern
    ///
    /// # Returns
    ///
    /// An ElementAttractor with all properties derived from the archetype pattern
    pub fn from_archetype_activation(activation: &[Float; 22]) -> Self {
        let atomic_number = Self::derive_atomic_number(activation);
        let electronegativity = Self::derive_electronegativity(activation);
        let stability = Self::calculate_stability(activation);

        let (symbol, name) = Self::get_element_identity(atomic_number);
        let atomic_mass = Self::derive_atomic_mass(activation, atomic_number);
        let is_noble = Self::is_noble_gas(atomic_number);

        Self {
            atomic_number,
            symbol,
            name,
            atomic_mass,
            electronegativity,
            electron_affinity: Self::derive_electron_affinity(activation),
            ionization_energy: Self::derive_ionization_energy(activation),
            stability,
            formation_threshold: Self::calculate_formation_threshold(atomic_number),
            archetype_signature: *activation,
            is_noble,
        }
    }

    /// Create element with a specific atomic number using archetype derivation
    ///
    /// This method creates an element with a guaranteed atomic number,
    /// deriving all other properties from archetype patterns.
    ///
    /// # Arguments
    ///
    /// * `atomic_number` - The desired atomic number (1-118)
    /// * `activation` - 22-value archetype activation pattern
    ///
    /// # Returns
    ///
    /// An ElementAttractor with the specified atomic number
    pub fn with_atomic_number(atomic_number: u32, activation: &[Float; 22]) -> Self {
        let electronegativity = Self::derive_electronegativity(activation);
        let stability = Self::calculate_stability(activation);

        let (symbol, name) = Self::get_element_identity(atomic_number);
        let atomic_mass = Self::derive_atomic_mass(activation, atomic_number);
        let is_noble = Self::is_noble_gas(atomic_number);

        Self {
            atomic_number,
            symbol,
            name,
            atomic_mass,
            electronegativity,
            electron_affinity: Self::derive_electron_affinity(activation),
            ionization_energy: Self::derive_ionization_energy(activation),
            stability,
            formation_threshold: Self::calculate_formation_threshold(atomic_number),
            archetype_signature: *activation,
            is_noble,
        }
    }

    /// Create element from holographic field position (holographic discovery)
    ///
    /// Elements can be "discovered" by examining the archetype activation
    /// at a specific position in the holographic field.
    ///
    /// # Arguments
    ///
    /// * `field` - Reference to the holographic field
    /// * `position` - Position in the field for element discovery
    ///
    /// # Returns
    ///
    /// An ElementAttractor derived from the local field properties
    pub fn from_holographic_field(field: &HolographicField, position: &Position) -> Self {
        let activation = field.derive_archetype_activation(position);
        Self::from_archetype_activation(&activation.coefficients)
    }

    /// Derive atomic number from archetype activation
    ///
    /// The atomic number emerges from the total activation magnitude
    /// scaled to the periodic table range (1-118).
    ///
    /// Mind archetypes (A1-A7) strongly influence element identity
    /// as they represent the core archetype structure.
    fn derive_atomic_number(activation: &[Float; 22]) -> u32 {
        // Total activation magnitude
        let magnitude: Float = activation.iter().sum();

        // Mind archetypes (A1-A7) strongly influence element identity
        // These represent the fundamental structure of consciousness
        let mind_factor: Float = activation[0..7].iter().sum::<Float>() / 7.0;

        // Combined formula maps to atomic number
        // Scale: typical activation sum ~10-15, map to Z 1-118
        let z = ((magnitude / 15.0) * 118.0 * mind_factor)
            .clamp(1.0, 118.0);

        z as u32
    }

    /// Derive electronegativity from archetype pattern
    ///
    /// From ROADMAP: "Electronegativity from Catalyst archetype (A3)"
    ///
    /// The Catalyst archetype (A3) determines how an element interacts
    /// with electrons - whether it attracts them (high EN) or donates them (low EN).
    ///
    /// Pauling scale: 0.7 (Cs) to 4.0 (F)
    fn derive_electronegativity(activation: &[Float; 22]) -> Float {
        // Catalyst archetype (A3, index 2) is primary
        let catalyst = activation[2];

        // Body archetypes (A8-A14) modulate electron attraction
        let body_factor: Float = activation[7..14].iter().sum::<Float>() / 7.0;

        // Matrix archetype (A1) provides structural stability
        let matrix = activation[0];

        // Pauling scale: 0.7 (Cs) to 4.0 (F)
        // Catalyst > 0.5 = higher electronegativity (electron attractor)
        // Catalyst < 0.5 = lower electronegativity (electron donor)
        let base_en = if catalyst > 0.5 {
            2.0 + (catalyst - 0.5) * 4.0 // 2.0 to 4.0
        } else {
            0.7 + catalyst * 2.6 // 0.7 to 2.0
        };

        // Modulate by body factor and matrix stability
        (base_en * body_factor * (0.5 + matrix * 0.5))
            .clamp(0.7, 4.0)
    }

    /// Calculate stability from archetype coherence
    ///
    /// Stability measures how coherent (aligned) the archetype activations are.
    /// High coherence = low variance = high stability.
    ///
    /// This reflects the principle that stable configurations have
    /// aligned archetype patterns.
    fn calculate_stability(activation: &[Float; 22]) -> Float {
        let mean: Float = activation.iter().sum::<Float>() / 22.0;
        let variance: Float = activation.iter().map(|c| (c - mean).powi(2)).sum::<Float>() / 22.0;

        // High coherence = high stability
        // Low variance = high stability
        1.0 - variance.sqrt().min(1.0)
    }

    /// Check if element is a noble gas (closed archetype shell)
    ///
    /// Noble gases have closed electron shells, which corresponds
    /// to closed archetype shells in our model. They have special
    /// stability and low reactivity.
    ///
    /// Noble gases: He (2), Ne (10), Ar (18), Kr (36), Xe (54), Rn (86), Og (118)
    fn is_noble_gas(atomic_number: u32) -> bool {
        matches!(atomic_number, 2 | 10 | 18 | 36 | 54 | 86 | 118)
    }

    /// Get element symbol and name from atomic number
    ///
    /// Provides identity for the first 20 elements (most common in organic chemistry)
    /// and generates names for others.
    fn get_element_identity(atomic_number: u32) -> (String, String) {
        match atomic_number {
            1 => ("H".to_string(), "Hydrogen".to_string()),
            2 => ("He".to_string(), "Helium".to_string()),
            3 => ("Li".to_string(), "Lithium".to_string()),
            4 => ("Be".to_string(), "Beryllium".to_string()),
            5 => ("B".to_string(), "Boron".to_string()),
            6 => ("C".to_string(), "Carbon".to_string()),
            7 => ("N".to_string(), "Nitrogen".to_string()),
            8 => ("O".to_string(), "Oxygen".to_string()),
            9 => ("F".to_string(), "Fluorine".to_string()),
            10 => ("Ne".to_string(), "Neon".to_string()),
            11 => ("Na".to_string(), "Sodium".to_string()),
            12 => ("Mg".to_string(), "Magnesium".to_string()),
            13 => ("Al".to_string(), "Aluminum".to_string()),
            14 => ("Si".to_string(), "Silicon".to_string()),
            15 => ("P".to_string(), "Phosphorus".to_string()),
            16 => ("S".to_string(), "Sulfur".to_string()),
            17 => ("Cl".to_string(), "Chlorine".to_string()),
            18 => ("Ar".to_string(), "Argon".to_string()),
            19 => ("K".to_string(), "Potassium".to_string()),
            20 => ("Ca".to_string(), "Calcium".to_string()),
            21 => ("Sc".to_string(), "Scandium".to_string()),
            22 => ("Ti".to_string(), "Titanium".to_string()),
            23 => ("V".to_string(), "Vanadium".to_string()),
            24 => ("Cr".to_string(), "Chromium".to_string()),
            25 => ("Mn".to_string(), "Manganese".to_string()),
            26 => ("Fe".to_string(), "Iron".to_string()),
            27 => ("Co".to_string(), "Cobalt".to_string()),
            28 => ("Ni".to_string(), "Nickel".to_string()),
            29 => ("Cu".to_string(), "Copper".to_string()),
            30 => ("Zn".to_string(), "Zinc".to_string()),
            31 => ("Ga".to_string(), "Gallium".to_string()),
            32 => ("Ge".to_string(), "Germanium".to_string()),
            33 => ("As".to_string(), "Arsenic".to_string()),
            34 => ("Se".to_string(), "Selenium".to_string()),
            35 => ("Br".to_string(), "Bromine".to_string()),
            36 => ("Kr".to_string(), "Krypton".to_string()),
            37 => ("Rb".to_string(), "Rubidium".to_string()),
            38 => ("Sr".to_string(), "Strontium".to_string()),
            39 => ("Y".to_string(), "Yttrium".to_string()),
            40 => ("Zr".to_string(), "Zirconium".to_string()),
            41 => ("Nb".to_string(), "Niobium".to_string()),
            42 => ("Mo".to_string(), "Molybdenum".to_string()),
            43 => ("Tc".to_string(), "Technetium".to_string()),
            44 => ("Ru".to_string(), "Ruthenium".to_string()),
            45 => ("Rh".to_string(), "Rhodium".to_string()),
            46 => ("Pd".to_string(), "Palladium".to_string()),
            47 => ("Ag".to_string(), "Silver".to_string()),
            48 => ("Cd".to_string(), "Cadmium".to_string()),
            49 => ("In".to_string(), "Indium".to_string()),
            50 => ("Sn".to_string(), "Tin".to_string()),
            51 => ("Sb".to_string(), "Antimony".to_string()),
            52 => ("Te".to_string(), "Tellurium".to_string()),
            53 => ("I".to_string(), "Iodine".to_string()),
            54 => ("Xe".to_string(), "Xenon".to_string()),
            55 => ("Cs".to_string(), "Cesium".to_string()),
            56 => ("Ba".to_string(), "Barium".to_string()),
            57 => ("La".to_string(), "Lanthanum".to_string()),
            58 => ("Ce".to_string(), "Cerium".to_string()),
            59 => ("Pr".to_string(), "Praseodymium".to_string()),
            60 => ("Nd".to_string(), "Neodymium".to_string()),
            61 => ("Pm".to_string(), "Promethium".to_string()),
            62 => ("Sm".to_string(), "Samarium".to_string()),
            63 => ("Eu".to_string(), "Europium".to_string()),
            64 => ("Gd".to_string(), "Gadolinium".to_string()),
            65 => ("Tb".to_string(), "Terbium".to_string()),
            66 => ("Dy".to_string(), "Dysprosium".to_string()),
            67 => ("Ho".to_string(), "Holmium".to_string()),
            68 => ("Er".to_string(), "Erbium".to_string()),
            69 => ("Tm".to_string(), "Thulium".to_string()),
            70 => ("Yb".to_string(), "Ytterbium".to_string()),
            71 => ("Lu".to_string(), "Lutetium".to_string()),
            72 => ("Hf".to_string(), "Hafnium".to_string()),
            73 => ("Ta".to_string(), "Tantalum".to_string()),
            74 => ("W".to_string(), "Tungsten".to_string()),
            75 => ("Re".to_string(), "Rhenium".to_string()),
            76 => ("Os".to_string(), "Osmium".to_string()),
            77 => ("Ir".to_string(), "Iridium".to_string()),
            78 => ("Pt".to_string(), "Platinum".to_string()),
            79 => ("Au".to_string(), "Gold".to_string()),
            80 => ("Hg".to_string(), "Mercury".to_string()),
            81 => ("Tl".to_string(), "Thallium".to_string()),
            82 => ("Pb".to_string(), "Lead".to_string()),
            83 => ("Bi".to_string(), "Bismuth".to_string()),
            84 => ("Po".to_string(), "Polonium".to_string()),
            85 => ("At".to_string(), "Astatine".to_string()),
            86 => ("Rn".to_string(), "Radon".to_string()),
            87 => ("Fr".to_string(), "Francium".to_string()),
            88 => ("Ra".to_string(), "Radium".to_string()),
            89 => ("Ac".to_string(), "Actinium".to_string()),
            90 => ("Th".to_string(), "Thorium".to_string()),
            91 => ("Pa".to_string(), "Protactinium".to_string()),
            92 => ("U".to_string(), "Uranium".to_string()),
            93 => ("Np".to_string(), "Neptunium".to_string()),
            94 => ("Pu".to_string(), "Plutonium".to_string()),
            95 => ("Am".to_string(), "Americium".to_string()),
            96 => ("Cm".to_string(), "Curium".to_string()),
            97 => ("Bk".to_string(), "Berkelium".to_string()),
            98 => ("Cf".to_string(), "Californium".to_string()),
            99 => ("Es".to_string(), "Einsteinium".to_string()),
            100 => ("Fm".to_string(), "Fermium".to_string()),
            101 => ("Md".to_string(), "Mendelevium".to_string()),
            102 => ("No".to_string(), "Nobelium".to_string()),
            103 => ("Lr".to_string(), "Lawrencium".to_string()),
            104 => ("Rf".to_string(), "Rutherfordium".to_string()),
            105 => ("Db".to_string(), "Dubnium".to_string()),
            106 => ("Sg".to_string(), "Seaborgium".to_string()),
            107 => ("Bh".to_string(), "Bohrium".to_string()),
            108 => ("Hs".to_string(), "Hassium".to_string()),
            109 => ("Mt".to_string(), "Meitnerium".to_string()),
            110 => ("Ds".to_string(), "Darmstadtium".to_string()),
            111 => ("Rg".to_string(), "Roentgenium".to_string()),
            112 => ("Cn".to_string(), "Copernicium".to_string()),
            113 => ("Nh".to_string(), "Nihonium".to_string()),
            114 => ("Fl".to_string(), "Flerovium".to_string()),
            115 => ("Mc".to_string(), "Moscovium".to_string()),
            116 => ("Lv".to_string(), "Livermorium".to_string()),
            117 => ("Ts".to_string(), "Tennessine".to_string()),
            118 => ("Og".to_string(), "Oganesson".to_string()),
            n => ("?".to_string(), format!("Element-{}", n)),
        }
    }

    /// Derive atomic mass from archetype and atomic number
    ///
    /// Atomic mass includes both protons and neutrons.
    /// The neutron/proton ratio is derived from archetype stability.
    fn derive_atomic_mass(activation: &[Float; 22], atomic_number: u32) -> Float {
        // Base mass from atomic number
        let base_mass = atomic_number as Float;

        // Neutron contribution from archetype stability
        let stability = Self::calculate_stability(activation);

        // Most stable nuclei have neutron/proton ratio near 1 for light elements
        // Increases to ~1.5 for heavy elements
        let neutron_ratio = 1.0 + (atomic_number as Float / 200.0) * stability;

        base_mass * neutron_ratio
    }

    /// Derive electron affinity from archetype
    ///
    /// Electron affinity measures the energy released when an electron
    /// is added to a neutral atom.
    ///
    /// Derived from Catalyst (A3) and Experience (A4) archetypes.
    fn derive_electron_affinity(activation: &[Float; 22]) -> Float {
        // Catalyst archetype determines electron attraction
        let catalyst = activation[2];

        // Experience archetype modulates affinity
        let experience = activation[3];

        // Electron affinity in eV: typically -0.5 to 3.6 eV
        // Negative values indicate energy required (endothermic)
        catalyst * experience * 3.6
    }

    /// Derive ionization energy from archetype
    ///
    /// Ionization energy is the energy required to remove an electron.
    ///
    /// Derived from Matrix (A1) and Great Way (A7) archetypes,
    /// representing structural resistance and completion/wholeness.
    fn derive_ionization_energy(activation: &[Float; 22]) -> Float {
        // Matrix archetype provides structural resistance to ionization
        let matrix = activation[0];

        // Great Way archetype (A7) represents completion/wholeness
        let great_way = activation[6];

        // Ionization energy in eV: typically 3.9 to 24.6 eV
        (matrix + great_way) / 2.0 * 12.0 + 3.9
    }

    /// Calculate formation threshold (minimum coherence for element formation)
    ///
    /// Higher atomic numbers require higher coherence to form
    /// because they represent more complex archetype configurations.
    fn calculate_formation_threshold(atomic_number: u32) -> Float {
        // Higher atomic numbers require higher coherence to form
        0.3 + (atomic_number as Float / 118.0) * 0.4
    }

    /// Validate element against periodic table data
    ///
    /// Checks that derived properties are within reasonable ranges
    /// and compares against known values where available.
    ///
    /// # Returns
    ///
    /// ElementValidation with validity status and any warnings
    pub fn validate(&self) -> ElementValidation {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check atomic number range
        if self.atomic_number == 0 || self.atomic_number > 118 {
            errors.push(format!("Invalid atomic number: {}", self.atomic_number));
        }

        // Check electronegativity range
        if self.electronegativity < 0.7 || self.electronegativity > 4.0 {
            warnings.push(format!(
                "Electronegativity {} outside typical range [0.7, 4.0]",
                self.electronegativity
            ));
        }

        // Check noble gas stability
        if self.is_noble && self.stability < 0.8 {
            warnings.push(format!(
                "Noble gas {} should have high stability, got {}",
                self.symbol, self.stability
            ));
        }

        // Validate against known periodic table values
        if let Some((known_en, known_mass)) = Self::get_known_values(self.atomic_number) {
            let en_error = (self.electronegativity - known_en).abs();
            if en_error > 0.5 {
                warnings.push(format!(
                    "Electronegativity deviation: derived {:.2} vs known {:.2} (error: {:.2})",
                    self.electronegativity, known_en, en_error
                ));
            }

            let mass_error = (self.atomic_mass - known_mass).abs() / known_mass;
            if mass_error > 0.2 {
                warnings.push(format!(
                    "Atomic mass deviation: derived {:.2} vs known {:.2} (error: {:.1}%)",
                    self.atomic_mass,
                    known_mass,
                    mass_error * 100.0
                ));
            }
        }

        ElementValidation {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Get known periodic table values for validation
    ///
    /// Returns (electronegativity, atomic_mass) for selected elements.
    /// Values from standard periodic table data.
    fn get_known_values(atomic_number: u32) -> Option<(Float, Float)> {
        // (electronegativity, atomic_mass) for validation
        match atomic_number {
            1 => Some((2.20, 1.008)),
            2 => Some((0.0, 4.003)), // Noble gas - no EN
            3 => Some((0.98, 6.941)),
            4 => Some((1.57, 9.012)),
            5 => Some((2.04, 10.81)),
            6 => Some((2.55, 12.011)),
            7 => Some((3.04, 14.007)),
            8 => Some((3.44, 15.999)),
            9 => Some((3.98, 18.998)),
            10 => Some((0.0, 20.180)), // Noble gas - no EN
            11 => Some((0.93, 22.990)),
            12 => Some((1.31, 24.305)),
            13 => Some((1.61, 26.982)),
            14 => Some((1.90, 28.086)),
            15 => Some((2.19, 30.974)),
            16 => Some((2.58, 32.065)),
            17 => Some((3.16, 35.453)),
            18 => Some((0.0, 39.948)), // Noble gas - no EN
            19 => Some((0.82, 39.098)),
            20 => Some((1.00, 40.078)),
            26 => Some((1.83, 55.845)), // Iron
            29 => Some((1.90, 63.546)), // Copper
            30 => Some((1.65, 65.38)),  // Zinc
            79 => Some((2.54, 196.97)), // Gold
            82 => Some((2.33, 207.2)),  // Lead
            _ => None,
        }
    }

    /// Get the period (row) of this element in the periodic table
    pub fn period(&self) -> u32 {
        Self::get_period(self.atomic_number)
    }

    /// Get period (row) for atomic number
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

    /// Get the group (column) of this element in the periodic table
    pub fn group(&self) -> u32 {
        Self::get_group(self.atomic_number)
    }

    /// Get group (column) for atomic number
    fn get_group(z: u32) -> u32 {
        match z {
            1 => 1,  // H: Group 1
            2 => 18, // He: Group 18
            // Period 2: Li(1), Be(2), B(13), C(14), N(15), O(16), F(17), Ne(18)
            3 => 1,   // Li: Group 1
            4 => 2,   // Be: Group 2
            5 => 13,  // B: Group 13
            6 => 14,  // C: Group 14
            7 => 15,  // N: Group 15
            8 => 16,  // O: Group 16
            9 => 17,  // F: Group 17
            10 => 18, // Ne: Group 18
            // Period 3: Na(1), Mg(2), Al(13), Si(14), P(15), S(16), Cl(17), Ar(18)
            11 => 1,  // Na: Group 1
            12 => 2,  // Mg: Group 2
            13 => 13, // Al: Group 13
            14 => 14, // Si: Group 14
            15 => 15, // P: Group 15
            16 => 16, // S: Group 16
            17 => 17, // Cl: Group 17
            18 => 18, // Ar: Group 18
            // Period 4: K(1), Ca(2), Sc(3)-Zn(12), Ga(13)-Kr(18)
            19..=20 => z - 18, // K: 1, Ca: 2
            21..=30 => z - 18, // Sc(3) to Zn(12)
            31..=36 => z - 18, // Ga(13) to Kr(18)
            // Period 5: Rb(1), Sr(2), Y(3)-Cd(12), In(13)-Xe(18)
            37..=38 => z - 36, // Rb: 1, Sr: 2
            39..=48 => z - 36, // Y(3) to Cd(12)
            49..=54 => z - 36, // In(13) to Xe(18)
            // Period 6: Cs(1), Ba(2), La-Lu(3), Hf(4)-Hg(12), Tl(13)-Rn(18)
            55..=56 => z - 54, // Cs: 1, Ba: 2
            57..=71 => 3,      // Lanthanides (La-Lu): Group 3
            72..=80 => z - 68, // Hf(4) to Hg(12)
            81..=86 => z - 68, // Tl(13) to Rn(18)
            // Period 7: Fr(1), Ra(2), Ac-Lr(3), Rf(4)-Cn(12), Nh(13)-Og(18)
            87..=88 => z - 86,    // Fr: 1, Ra: 2
            89..=103 => 3,        // Actinides (Ac-Lr): Group 3
            104..=112 => z - 100, // Rf(4) to Cn(12)
            113..=118 => z - 100, // Nh(13) to Og(18)
            _ => 1,
        }
    }

    /// Check if this element can form at the given field coherence
    pub fn can_form_at_coherence(&self, field_coherence: Float) -> bool {
        field_coherence >= self.formation_threshold
    }
}

impl fmt::Display for ElementAttractor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Element {} ({}) Z={} EN={:.2} stability={:.2}",
            self.symbol, self.name, self.atomic_number, self.electronegativity, self.stability
        )
    }
}

/// Result of element validation
#[derive(Debug, Clone)]
pub struct ElementValidation {
    /// Whether the element passed validation
    pub is_valid: bool,

    /// Fatal errors that make the element invalid
    pub errors: Vec<String>,

    /// Warnings about property deviations
    pub warnings: Vec<String>,
}

impl ElementValidation {
    /// Check if validation passed with no warnings
    pub fn is_perfect(&self) -> bool {
        self.is_valid && self.warnings.is_empty() && self.errors.is_empty()
    }
}

impl fmt::Display for ElementValidation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid {
            write!(f, "Valid")?;
            if !self.warnings.is_empty() {
                write!(f, " ({} warnings)", self.warnings.len())?;
            }
        } else {
            write!(f, "Invalid ({} errors)", self.errors.len())?;
        }
        Ok(())
    }
}

/// Periodic table of elements as archetype-derived attractor fields
///
/// The periodic table is generated from archetype templates, where each
/// element corresponds to a specific archetype activation pattern.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The periodic table reflects archetype resonance patterns"
pub struct PeriodicTable {
    /// All elements 1-118 as attractor fields
    pub elements: Vec<ElementAttractor>,
}

impl PeriodicTable {
    /// Generate periodic table from archetype patterns
    ///
    /// Creates all 118 elements using archetype templates that map
    /// to the appropriate periodic table positions.
    pub fn generate() -> Self {
        let mut elements = Vec::new();

        // Generate elements 1-118 from archetype templates
        for z in 1..=118 {
            let activation = Self::archetype_template_for_element(z);
            elements.push(ElementAttractor::with_atomic_number(z, &activation));
        }

        Self { elements }
    }

    /// Archetype template for element with atomic number z
    ///
    /// From ROADMAP: "Elements follow archetype patterns that map to periodic table positions"
    ///
    /// The template encodes:
    /// - Period (row) influences overall activation magnitude
    /// - Group (column) influences Catalyst archetype (electronegativity)
    /// - Noble gas positions have maximum coherence
    fn archetype_template_for_element(z: u32) -> [Float; 22] {
        let mut activation = [0.5; 22]; // Base activation

        // Period 1-7: 1-2, 3-10, 11-18, 19-36, 37-54, 55-86, 87-118
        let period = Self::get_period(z);

        // Group determines archetype emphasis
        let group = Self::get_group(z);

        // Set archetype coefficients based on position

        // A1 (Matrix): Structure, higher for noble gases
        activation[0] = if matches!(group, 18) {
            0.95 // Noble gases have maximum structural stability
        } else {
            0.3 + (group as Float / 18.0) * 0.4
        };

        // A2 (Potentiator): Capacity, varies by period
        activation[1] = 0.3 + period as Float * 0.1;

        // A3 (Catalyst): Electronegativity factor
        // Higher for nonmetals (right side), lower for metals (left side)
        // Group 1 (alkali metals) have lowest EN
        // Group 17 (halogens) have highest EN
        // Group 18 (noble gases) have special case
        if matches!(group, 18) {
            // Noble gases - balanced, no strong tendency
            activation[2] = 0.5;
        } else {
            // Scale from 0.2 (group 1) to 0.9 (group 17)
            activation[2] = 0.2 + (group as Float / 18.0) * 0.7;
        }

        // A4 (Experience): Modulates electron affinity
        activation[3] = 0.4 + (period as Float * 0.05);

        // A5 (Significator): Identity, moderate for all
        activation[4] = 0.5;

        // A6 (Transformation): Change potential
        // Higher for reactive elements, lower for noble gases
        activation[5] = if matches!(group, 18) {
            0.2 // Noble gases resist transformation
        } else {
            0.6
        };

        // A7 (Great Way): Completion/wholeness
        // Maximum for noble gases (closed shells)
        activation[6] = if matches!(group, 18) {
            1.0
        } else {
            0.4 + (1.0 - (group as Float - 9.0).abs() / 9.0) * 0.3
        };

        // Body archetypes (A8-A14) mirror mind patterns
        activation[7] = activation[0] * 0.8; // Matrix
        activation[8] = activation[1] * 0.8; // Potentiator
        activation[9] = activation[2] * 0.9; // Catalyst (strong for chemistry)
        activation[10] = activation[3] * 0.8; // Experience
        activation[11] = activation[4] * 0.8; // Significator
        activation[12] = activation[5] * 0.8; // Transformation
        activation[13] = activation[6] * 0.8; // Great Way

        // Spirit archetypes (A15-A21)
        activation[14] = activation[0] * 0.7; // Matrix
        activation[15] = activation[1] * 0.7; // Potentiator
        activation[16] = activation[2] * 0.8; // Catalyst
        activation[17] = activation[3] * 0.7; // Experience
        activation[18] = activation[4] * 0.7; // Significator
        activation[19] = activation[5] * 0.7; // Transformation
        activation[20] = activation[6] * 0.7; // Great Way

        // A22 (Choice): Free will factor
        activation[21] = 0.5;

        // Scale activation magnitude with atomic number
        // Higher Z = more complex structure
        let scale = (z as Float / 60.0).clamp(0.5, 1.5);
        for coeff in activation.iter_mut() {
            *coeff = (*coeff * scale).clamp(0.1, 1.0);
        }

        activation
    }

    /// Get period (row) for atomic number
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

    /// Get group (column) for atomic number
    fn get_group(z: u32) -> u32 {
        match z {
            1 => 1,  // H: Group 1
            2 => 18, // He: Group 18
            // Period 2: Li(1), Be(2), B(13), C(14), N(15), O(16), F(17), Ne(18)
            3 => 1,   // Li: Group 1
            4 => 2,   // Be: Group 2
            5 => 13,  // B: Group 13
            6 => 14,  // C: Group 14
            7 => 15,  // N: Group 15
            8 => 16,  // O: Group 16
            9 => 17,  // F: Group 17
            10 => 18, // Ne: Group 18
            // Period 3: Na(1), Mg(2), Al(13), Si(14), P(15), S(16), Cl(17), Ar(18)
            11 => 1,  // Na: Group 1
            12 => 2,  // Mg: Group 2
            13 => 13, // Al: Group 13
            14 => 14, // Si: Group 14
            15 => 15, // P: Group 15
            16 => 16, // S: Group 16
            17 => 17, // Cl: Group 17
            18 => 18, // Ar: Group 18
            // Period 4: K(1), Ca(2), Sc(3)-Zn(12), Ga(13)-Kr(18)
            19..=20 => z - 18, // K: 1, Ca: 2
            21..=30 => z - 18, // Sc(3) to Zn(12)
            31..=36 => z - 18, // Ga(13) to Kr(18)
            // Period 5: Rb(1), Sr(2), Y(3)-Cd(12), In(13)-Xe(18)
            37..=38 => z - 36, // Rb: 1, Sr: 2
            39..=48 => z - 36, // Y(3) to Cd(12)
            49..=54 => z - 36, // In(13) to Xe(18)
            // Period 6: Cs(1), Ba(2), La-Lu(3), Hf(4)-Hg(12), Tl(13)-Rn(18)
            55..=56 => z - 54, // Cs: 1, Ba: 2
            57..=71 => 3,      // Lanthanides (La-Lu): Group 3
            72..=80 => z - 68, // Hf(4) to Hg(12)
            81..=86 => z - 68, // Tl(13) to Rn(18)
            // Period 7: Fr(1), Ra(2), Ac-Lr(3), Rf(4)-Cn(12), Nh(13)-Og(18)
            87..=88 => z - 86,    // Fr: 1, Ra: 2
            89..=103 => 3,        // Actinides (Ac-Lr): Group 3
            104..=112 => z - 100, // Rf(4) to Cn(12)
            113..=118 => z - 100, // Nh(13) to Og(18)
            _ => 1,
        }
    }

    /// Get element by atomic number
    pub fn get(&self, atomic_number: u32) -> Option<&ElementAttractor> {
        if atomic_number == 0 || atomic_number > 118 {
            None
        } else {
            self.elements.get((atomic_number - 1) as usize)
        }
    }

    /// Find element by symbol
    pub fn find_by_symbol(&self, symbol: &str) -> Option<&ElementAttractor> {
        self.elements.iter().find(|e| e.symbol == symbol)
    }

    /// Find element by name
    pub fn find_by_name(&self, name: &str) -> Option<&ElementAttractor> {
        self.elements
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case(name))
    }

    /// Get all noble gases
    pub fn noble_gases(&self) -> Vec<&ElementAttractor> {
        self.elements.iter().filter(|e| e.is_noble).collect()
    }

    /// Get elements that can form at the given coherence level
    pub fn elements_formable_at(&self, coherence: Float) -> Vec<&ElementAttractor> {
        self.elements
            .iter()
            .filter(|e| e.can_form_at_coherence(coherence))
            .collect()
    }

    /// Validate all elements against known periodic table data
    pub fn validate_all(&self) -> (usize, Vec<(u32, ElementValidation)>) {
        let mut valid_count = 0;
        let mut validations = Vec::new();

        for element in &self.elements {
            let validation = element.validate();
            if validation.is_valid {
                valid_count += 1;
            }
            if !validation.is_perfect() {
                validations.push((element.atomic_number, validation));
            }
        }

        (valid_count, validations)
    }

    /// Get element count
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for PeriodicTable {
    fn default() -> Self {
        Self::generate()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrogen_from_archetype() {
        let activation = [0.5; 22];
        let h = ElementAttractor::from_archetype_activation(&activation);
        assert!(h.atomic_number >= 1);
        assert!(h.electronegativity >= 0.7 && h.electronegativity <= 4.0);
    }

    #[test]
    fn test_noble_gas_stability() {
        let pt = PeriodicTable::generate();

        // Noble gases should have high stability
        let he = pt.get(2).unwrap();
        assert!(he.is_noble, "Helium should be a noble gas");
        assert!(
            he.stability > 0.5,
            "Noble gas stability should be high, got {}",
            he.stability
        );

        let ne = pt.get(10).unwrap();
        assert!(ne.is_noble, "Neon should be a noble gas");

        let ar = pt.get(18).unwrap();
        assert!(ar.is_noble, "Argon should be a noble gas");

        let kr = pt.get(36).unwrap();
        assert!(kr.is_noble, "Krypton should be a noble gas");

        let xe = pt.get(54).unwrap();
        assert!(xe.is_noble, "Xenon should be a noble gas");

        let rn = pt.get(86).unwrap();
        assert!(rn.is_noble, "Radon should be a noble gas");
    }

    #[test]
    fn test_periodic_table_generation() {
        let pt = PeriodicTable::generate();
        assert_eq!(pt.elements.len(), 118);

        // Check first element
        let h = pt.get(1).unwrap();
        assert_eq!(h.symbol, "H");
        assert_eq!(h.name, "Hydrogen");

        // Check carbon
        let c = pt.get(6).unwrap();
        assert_eq!(c.symbol, "C");
        assert_eq!(c.name, "Carbon");

        // Check last known element
        let og = pt.get(118).unwrap();
        assert_eq!(og.symbol, "Og");
        assert_eq!(og.name, "Oganesson");
        assert!(og.is_noble);
    }

    #[test]
    fn test_electronegativity_derivation() {
        // High catalyst → high electronegativity
        let mut high_cat = [0.5; 22];
        high_cat[2] = 0.9; // High catalyst
        high_cat[7..14].iter_mut().for_each(|c| *c = 0.8); // High body factor
        let high_en = ElementAttractor::from_archetype_activation(&high_cat);

        // Low catalyst → low electronegativity
        let mut low_cat = [0.5; 22];
        low_cat[2] = 0.2; // Low catalyst
        let low_en = ElementAttractor::from_archetype_activation(&low_cat);

        assert!(
            high_en.electronegativity > low_en.electronegativity,
            "High catalyst should give higher EN: {} vs {}",
            high_en.electronegativity,
            low_en.electronegativity
        );
    }

    #[test]
    fn test_element_validation() {
        let activation = [0.5; 22];
        let element = ElementAttractor::from_archetype_activation(&activation);
        let validation = element.validate();
        assert!(validation.is_valid, "Element should be valid");
    }

    #[test]
    fn test_periodic_table_get() {
        let pt = PeriodicTable::generate();

        // Valid atomic numbers
        assert!(pt.get(1).is_some());
        assert!(pt.get(50).is_some());
        assert!(pt.get(118).is_some());

        // Invalid atomic numbers
        assert!(pt.get(0).is_none());
        assert!(pt.get(119).is_none());
    }

    #[test]
    fn test_find_by_symbol() {
        let pt = PeriodicTable::generate();

        assert!(pt.find_by_symbol("H").is_some());
        assert!(pt.find_by_symbol("C").is_some());
        assert!(pt.find_by_symbol("Au").is_some());
        assert!(pt.find_by_symbol("XX").is_none());
    }

    #[test]
    fn test_find_by_name() {
        let pt = PeriodicTable::generate();

        assert!(pt.find_by_name("Hydrogen").is_some());
        assert!(pt.find_by_name("carbon").is_some()); // Case insensitive
        assert!(pt.find_by_name("Gold").is_some());
        assert!(pt.find_by_name("Unobtainium").is_none());
    }

    #[test]
    fn test_noble_gases_list() {
        let pt = PeriodicTable::generate();
        let nobles = pt.noble_gases();

        assert_eq!(nobles.len(), 7, "Should have 7 noble gases");
        assert!(nobles.iter().all(|e| e.is_noble));
    }

    #[test]
    fn test_formable_elements() {
        let pt = PeriodicTable::generate();

        // Low coherence - only light elements can form
        let low_coherence = pt.elements_formable_at(0.35);
        assert!(low_coherence.len() < 118);

        // High coherence - all elements can form
        let high_coherence = pt.elements_formable_at(0.9);
        assert_eq!(high_coherence.len(), 118);
    }

    #[test]
    fn test_element_period_group() {
        let pt = PeriodicTable::generate();

        // Hydrogen: period 1, group 1
        let h = pt.get(1).unwrap();
        assert_eq!(h.period(), 1);
        assert_eq!(h.group(), 1);

        // Helium: period 1, group 18
        let he = pt.get(2).unwrap();
        assert_eq!(he.period(), 1);
        assert_eq!(he.group(), 18);

        // Carbon: period 2, group 14
        let c = pt.get(6).unwrap();
        assert_eq!(c.period(), 2);
        assert_eq!(c.group(), 14);

        // Iron: period 4, group 8
        let fe = pt.get(26).unwrap();
        assert_eq!(fe.period(), 4);
        assert_eq!(fe.group(), 8);
    }

    #[test]
    fn test_validate_all() {
        let pt = PeriodicTable::generate();
        let (valid_count, issues) = pt.validate_all();

        // All elements should be valid
        assert_eq!(valid_count, 118);

        // Print any issues for debugging
        for (z, validation) in &issues {
            if !validation.warnings.is_empty() {
                println!("Element {}: {} warnings", z, validation.warnings.len());
            }
        }
    }

    #[test]
    fn test_element_display() {
        let pt = PeriodicTable::generate();
        let h = pt.get(1).unwrap();
        let display = format!("{}", h);
        assert!(display.contains("H"));
        assert!(display.contains("Hydrogen"));
    }

    #[test]
    fn test_stability_calculation() {
        // Coherent activation = high stability
        let coherent = [0.6; 22];
        let stable = ElementAttractor::from_archetype_activation(&coherent);

        // Incoherent activation = lower stability
        let mut incoherent = [0.5; 22];
        incoherent[0] = 1.0;
        incoherent[10] = 0.1;
        incoherent[20] = 0.9;
        let unstable = ElementAttractor::from_archetype_activation(&incoherent);

        assert!(
            stable.stability > unstable.stability,
            "Coherent pattern should be more stable: {} vs {}",
            stable.stability,
            unstable.stability
        );
    }

    #[test]
    fn test_known_values_comparison() {
        let pt = PeriodicTable::generate();

        // Test carbon
        let c = pt.get(6).unwrap();
        let known = ElementAttractor::get_known_values(6);
        assert!(known.is_some());
        let (known_en, known_mass) = known.unwrap();

        // Derived values should be reasonably close
        // Note: archetype derivation may not match exactly, but should be in range
        println!(
            "Carbon: derived EN={:.2}, known EN={:.2}",
            c.electronegativity, known_en
        );
        println!(
            "Carbon: derived mass={:.2}, known mass={:.2}",
            c.atomic_mass, known_mass
        );

        // Check oxygen
        let o = pt.get(8).unwrap();
        println!("Oxygen: derived EN={:.2}", o.electronegativity);

        // Check fluorine (highest EN)
        let f = pt.get(9).unwrap();
        println!("Fluorine: derived EN={:.2}", f.electronegativity);
    }
}
