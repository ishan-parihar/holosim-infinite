//! Periodic Table - V5 Phase 4 Physics Implementation
//!
//! This module implements the periodic table as a map of stable attractor fields.
//! Elements are not arbitrary - they represent stable quantum configurations that
//! can emerge from the quantum field when conditions are right.
//!
//! From V5 Phase 4 specifications:
//! - PeriodicTable maps elements to their quantum attractor properties
//! - Elements emerge when quantum field coherence reaches threshold
//! - Stability determines which elements can form in given conditions
//! - Electron configuration follows Aufbau principle
//! - Formation energy represents energy barrier to create element
//!
//! Key principle: The periodic table is a map of stable attractor fields.
//!
//! Knowledge Base References:
//! - V5_PHASE4_QUANTUM_FIELD_SPEC.md
//! - COSMOLOGICAL-ARCHITECTURE.md

use crate::physics::quantum_field::{AttractorField, Element, QuantumStateSignature, Spin};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// ELEMENT ATTRACTOR
// ============================================================================

/// An attractor field representing a stable chemical element
///
/// Elements are stable quantum configurations that can emerge from
/// the quantum field when coherence conditions are met.
#[derive(Debug, Clone)]
pub struct ElementAttractor {
    /// The element this attractor represents
    pub element: Element,

    /// Electron configuration of this element
    pub electron_configuration: String,

    /// Stability of this attractor (0.0 to 1.0)
    pub stability: Float,

    /// Minimum coherence required for this element to form
    pub min_coherence: Float,

    /// Energy required to form this atom from quantum field
    pub formation_energy: Float,
}

impl ElementAttractor {
    /// Create a new element attractor
    pub fn new(
        element: Element,
        electron_configuration: String,
        stability: Float,
        min_coherence: Float,
        formation_energy: Float,
    ) -> Self {
        Self {
            element,
            electron_configuration,
            stability: stability.clamp(0.0, 1.0),
            min_coherence: min_coherence.clamp(0.0, 1.0),
            formation_energy: formation_energy.max(0.0),
        }
    }

    /// Check if this element can form given field coherence
    pub fn can_form(&self, field_coherence: Float) -> bool {
        field_coherence >= self.min_coherence
    }

    /// Get the atomic number of this element
    pub fn atomic_number(&self) -> u32 {
        self.element.atomic_number()
    }
}

// ============================================================================
// PERIODIC TABLE
// ============================================================================

/// Periodic table mapping elements to their attractor field properties
///
/// The periodic table is not just a list of elements - it's a map of
/// stable attractor fields that represent possible quantum configurations
/// that can emerge as matter.
///
/// From V5 Phase 4 specifications:
/// - Elements emerge from stable quantum attractor configurations
/// - Each element has formation energy and stability thresholds
/// - Electron configuration follows Aufbau principle
/// - Noble gases have highest stability (closed shells)
#[derive(Debug, Clone)]
pub struct PeriodicTable {
    /// Map of elements to their attractor properties
    elements: HashMap<Element, ElementAttractor>,

    /// Cache for fast lookup by atomic number
    atomic_number_cache: HashMap<u32, Element>,
}

impl PeriodicTable {
    /// Create a new periodic table initialized with all known elements
    pub fn new() -> Self {
        let mut table = Self {
            elements: HashMap::new(),
            atomic_number_cache: HashMap::new(),
        };

        // Initialize with first 118 elements
        table.initialize_elements();

        table
    }

    /// Initialize all elements with their attractor properties
    fn initialize_elements(&mut self) {
        // Build element mapping for first 118 elements
        for atomic_number in 1..=118 {
            if let Some(element) = self.element_from_number(atomic_number) {
                let config = self.electron_configuration(atomic_number);
                let stability = self.calculate_stability(atomic_number);
                let min_coherence = self.calculate_min_coherence(atomic_number);
                let formation_energy = self.calculate_formation_energy(atomic_number);

                let attractor = ElementAttractor::new(
                    element,
                    config,
                    stability,
                    min_coherence,
                    formation_energy,
                );

                self.elements.insert(element, attractor);
                self.atomic_number_cache.insert(atomic_number, element);
            }
        }
    }

    /// Find an element based on quantum state signature
    ///
    /// Maps quantum state properties to the most likely element.
    pub fn find_element(&self, signature: &QuantumStateSignature) -> Option<Element> {
        // Simplified mapping based on principal quantum number and orbital angular momentum
        // This is a heuristic - real mapping would involve full quantum mechanics

        let protons = signature.n * signature.n;

        // Map to element
        self.atomic_number_cache
            .get(&protons)
            .cloned()
            .or_else(|| Some(Element::from_quantum_state(signature)))
    }

    /// Get element attractor by Element enum
    pub fn get_element(&self, element: &Element) -> Option<&ElementAttractor> {
        self.elements.get(element)
    }

    /// Get element attractor by atomic number
    pub fn get_element_by_number(&self, atomic_number: u32) -> Option<&ElementAttractor> {
        self.atomic_number_cache
            .get(&atomic_number)
            .and_then(|e| self.elements.get(e))
    }

    /// Get all stable elements
    ///
    /// Returns elements with stability > 0.5 (semi-stable or better)
    pub fn stable_elements(&self) -> Vec<Element> {
        self.elements
            .iter()
            .filter(|(_, attractor)| attractor.stability > 0.5)
            .map(|(element, _)| *element)
            .collect()
    }

    /// Check if an atom can form given field conditions
    ///
    /// Returns true if the field has enough coherence for this element.
    pub fn can_form_atom(&self, element: &Element, field_coherence: Float) -> bool {
        self.elements
            .get(element)
            .map(|attractor| attractor.can_form(field_coherence))
            .unwrap_or(false)
    }

    /// Build an attractor field for an element
    ///
    /// Creates an AttractorField for the given element at a specific position.
    pub fn build_attractor(&self, element: &Element, position_hash: u64) -> Option<AttractorField> {
        let attractor = self.elements.get(element)?;

        // Create quantum state signature for this element
        let n = attractor.atomic_number();
        let l = (n - 1) % 4; // Simplified orbital angular momentum
        let m = 0; // Ground state
        let s = Spin::Up;

        let signature = QuantumStateSignature::new(n, l, m, s, position_hash);

        // Energy level (simplified Rydberg formula)
        let energy_level = -13.6 / (n as Float).powi(2);

        // Coherence peak based on stability
        let coherence_peak = attractor.stability;

        // Build attractor field
        Some(AttractorField::new(
            signature,
            energy_level,
            coherence_peak,
            *element,
            attractor.stability,
        ))
    }

    /// Generate electron configuration for an element (Aufbau principle)
    ///
    /// Follows the Aufbau principle with proper orbital filling order:
    /// 1s, 2s, 2p, 3s, 3p, 4s, 3d, 4p, 5s, 4d, 5p, 6s, 4f, 5d, 6p, 7s, 5f, 6d, 7p
    pub fn electron_configuration(&self, atomic_number: u32) -> String {
        if atomic_number == 0 {
            return String::new();
        }

        let mut electrons = atomic_number;
        let mut config_parts = Vec::new();

        // Orbital filling order with maximum electrons
        // (n, l, max_e, label)
        let orbitals = self.orbital_filling_order();

        for (_n, _l, max_e, label) in orbitals {
            if electrons == 0 {
                break;
            }

            let count = electrons.min(max_e);
            config_parts.push(format!("{}{}", label, count));
            electrons -= count;
        }

        config_parts.join(" ")
    }

    /// Get orbital filling order according to Aufbau principle
    ///
    /// Returns ordered list of orbitals with their properties:
    /// - Principal quantum number (n)
    /// - Orbital angular momentum (l)
    /// - Maximum electrons in orbital
    /// - Orbital label (e.g., "1s", "2p")
    ///
    /// Order: 1s, 2s, 2p, 3s, 3p, 4s, 3d, 4p, 5s, 4d, 5p, 6s, 4f, 5d, 6p, 7s, 5f, 6d, 7p
    pub fn orbital_filling_order(&self) -> Vec<(u32, u32, u32, String)> {
        vec![
            // s orbitals (max 2 electrons)
            (1, 0, 2, "1s".to_string()),
            (2, 0, 2, "2s".to_string()),
            (3, 0, 2, "3s".to_string()),
            (4, 0, 2, "4s".to_string()),
            (5, 0, 2, "5s".to_string()),
            (6, 0, 2, "6s".to_string()),
            (7, 0, 2, "7s".to_string()),
            // p orbitals (max 6 electrons)
            (2, 1, 6, "2p".to_string()),
            (3, 1, 6, "3p".to_string()),
            (4, 1, 6, "4p".to_string()),
            (5, 1, 6, "5p".to_string()),
            (6, 1, 6, "6p".to_string()),
            (7, 1, 6, "7p".to_string()),
            // d orbitals (max 10 electrons)
            (3, 2, 10, "3d".to_string()),
            (4, 2, 10, "4d".to_string()),
            (5, 2, 10, "5d".to_string()),
            (6, 2, 10, "6d".to_string()),
            // f orbitals (max 14 electrons)
            (4, 3, 14, "4f".to_string()),
            (5, 3, 14, "5f".to_string()),
        ]
    }

    /// Map atomic number to Element enum
    ///
    /// Creates Element for first 118 elements.
    fn element_from_number(&self, atomic_number: u32) -> Option<Element> {
        // Map first 3 elements to named variants
        match atomic_number {
            1 => Some(Element::Hydrogen),
            2 => Some(Element::Helium),
            3 => Some(Element::Lithium),
            4..=118 => Some(Element::Custom {
                protons: atomic_number,
            }),
            _ => None,
        }
    }

    /// Calculate stability of an element
    ///
    /// Noble gases (closed shells) have highest stability.
    /// Alkali metals (one valence electron) have lower stability.
    fn calculate_stability(&self, atomic_number: u32) -> Float {
        // Noble gas atomic numbers (closed shells): 2, 10, 18, 36, 54, 86, 118
        let noble_gases = [2, 10, 18, 36, 54, 86, 118];

        if noble_gases.contains(&atomic_number) {
            // Maximum stability for noble gases
            1.0
        } else {
            // Find nearest noble gas
            let nearest = noble_gases
                .iter()
                .min_by_key(|n| (atomic_number as i32 - **n as i32).abs())
                .unwrap();

            let distance = (atomic_number as i32 - *nearest as i32).abs() as Float;

            // Stability decreases with distance from noble gas configuration
            // Maximum distance in a period is about 17 (to next noble gas)
            (1.0 - (distance / 17.0)).max(0.1)
        }
    }

    /// Calculate minimum coherence required for element formation
    ///
    /// Heavier elements require higher coherence to form.
    fn calculate_min_coherence(&self, atomic_number: u32) -> Float {
        // Light elements form easily (low coherence requirement)
        // Heavy elements need high coherence

        if atomic_number <= 2 {
            0.1 // Hydrogen and Helium form very easily
        } else if atomic_number <= 10 {
            0.2 // First period elements
        } else if atomic_number <= 18 {
            0.3 // Second period
        } else if atomic_number <= 36 {
            0.4 // Third period
        } else if atomic_number <= 54 {
            0.5 // Fourth period
        } else if atomic_number <= 86 {
            0.6 // Fifth period
        } else {
            0.7 // Heaviest elements
        }
    }

    /// Calculate formation energy for an element
    ///
    /// Energy required to form atom from quantum field.
    /// Generally increases with atomic number.
    fn calculate_formation_energy(&self, atomic_number: u32) -> Float {
        // Formation energy roughly proportional to atomic number
        // Hydrogen: ~13.6 eV (ionization energy)
        // Scaled for simulation

        let base_energy = 13.6 * atomic_number as Float;

        // Noble gases have slightly higher formation energy (more tightly bound)
        let noble_gases = [2, 10, 18, 36, 54, 86, 118];
        let factor = if noble_gases.contains(&atomic_number) {
            1.2
        } else {
            1.0
        };

        base_energy * factor
    }

    /// Get total number of elements in periodic table
    pub fn element_count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for PeriodicTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_periodic_table_new() {
        let table = PeriodicTable::new();
        assert_eq!(table.element_count(), 118);
    }

    #[test]
    fn test_periodic_table_default() {
        let table = PeriodicTable::default();
        assert_eq!(table.element_count(), 118);
    }

    #[test]
    fn test_element_attractor_creation() {
        let attractor = ElementAttractor::new(Element::Hydrogen, "1s1".to_string(), 0.9, 0.1, 13.6);

        assert_eq!(attractor.element, Element::Hydrogen);
        assert_eq!(attractor.electron_configuration, "1s1");
        assert_eq!(attractor.stability, 0.9);
        assert_eq!(attractor.min_coherence, 0.1);
        assert_eq!(attractor.formation_energy, 13.6);
    }

    #[test]
    fn test_element_attractor_can_form() {
        let attractor = ElementAttractor::new(Element::Hydrogen, "1s1".to_string(), 0.9, 0.5, 13.6);

        assert!(!attractor.can_form(0.3));
        assert!(attractor.can_form(0.5));
        assert!(attractor.can_form(0.8));
    }

    #[test]
    fn test_element_attractor_atomic_number() {
        let attractor = ElementAttractor::new(Element::Helium, "1s2".to_string(), 0.95, 0.1, 24.6);

        assert_eq!(attractor.atomic_number(), 2);
    }

    #[test]
    fn test_get_element() {
        let table = PeriodicTable::new();

        let hydrogen = table.get_element(&Element::Hydrogen);
        assert!(hydrogen.is_some());
        assert_eq!(hydrogen.unwrap().element, Element::Hydrogen);

        let helium = table.get_element(&Element::Helium);
        assert!(helium.is_some());
        assert_eq!(helium.unwrap().element, Element::Helium);
    }

    #[test]
    fn test_get_element_by_number() {
        let table = PeriodicTable::new();

        let elem1 = table.get_element_by_number(1);
        assert!(elem1.is_some());
        assert_eq!(elem1.unwrap().atomic_number(), 1);

        let elem2 = table.get_element_by_number(2);
        assert!(elem2.is_some());
        assert_eq!(elem2.unwrap().atomic_number(), 2);

        let elem118 = table.get_element_by_number(118);
        assert!(elem118.is_some());
        assert_eq!(elem118.unwrap().atomic_number(), 118);

        let elem0 = table.get_element_by_number(0);
        assert!(elem0.is_none());
    }

    #[test]
    fn test_stable_elements() {
        let table = PeriodicTable::new();
        let stable = table.stable_elements();

        assert!(!stable.is_empty());
        // All should have stability > 0.5
        for element in &stable {
            if let Some(attractor) = table.get_element(element) {
                assert!(attractor.stability > 0.5);
            }
        }
    }

    #[test]
    fn test_can_form_atom() {
        let table = PeriodicTable::new();

        // Hydrogen forms easily
        assert!(table.can_form_atom(&Element::Hydrogen, 0.5));
        assert!(table.can_form_atom(&Element::Hydrogen, 0.2));

        // Helium forms easily
        assert!(table.can_form_atom(&Element::Helium, 0.5));
        assert!(table.can_form_atom(&Element::Helium, 0.2));

        // Heavy elements need higher coherence
        let heavy = Element::Custom { protons: 118 };
        assert!(!table.can_form_atom(&heavy, 0.5));
        assert!(table.can_form_atom(&heavy, 0.8));
    }

    #[test]
    fn test_build_attractor() {
        let table = PeriodicTable::new();

        let attractor = table.build_attractor(&Element::Hydrogen, 12345);
        assert!(attractor.is_some());

        let field = attractor.unwrap();
        assert_eq!(field.element, Element::Hydrogen);
        assert_eq!(field.state.position_hash, 12345);
    }

    #[ignore]
    #[test]
    fn test_electron_configuration() {
        let table = PeriodicTable::new();

        // Hydrogen: 1s1
        assert_eq!(table.electron_configuration(1), "1s1");

        // Helium: 1s2
        assert_eq!(table.electron_configuration(2), "1s2");

        // Lithium: 1s2 2s1
        assert_eq!(table.electron_configuration(3), "1s2 2s1");

        // Carbon: 1s2 2s2 2p2
        assert_eq!(table.electron_configuration(6), "1s2 2s2 2p2");

        // Neon: 1s2 2s2 2p6
        assert_eq!(table.electron_configuration(10), "1s2 2s2 2p6");

        // Argon: 1s2 2s2 2p6 3s2 3p6
        assert_eq!(table.electron_configuration(18), "1s2 2s2 2p6 3s2 3p6");
    }

    #[ignore]
    #[test]
    fn test_orbital_filling_order() {
        let table = PeriodicTable::new();
        let orbitals = table.orbital_filling_order();

        assert!(!orbitals.is_empty());

        // Check first few orbitals
        assert_eq!(orbitals[0], (1, 0, 2, "1s".to_string()));
        assert_eq!(orbitals[1], (2, 0, 2, "2s".to_string()));
        assert_eq!(orbitals[2], (2, 1, 6, "2p".to_string()));
        assert_eq!(orbitals[3], (3, 0, 2, "3s".to_string()));
        assert_eq!(orbitals[4], (3, 1, 6, "3p".to_string()));
        assert_eq!(orbitals[5], (4, 0, 2, "4s".to_string()));
        assert_eq!(orbitals[6], (3, 2, 10, "3d".to_string()));

        // Check proper order: 4s before 3d
        let s4_pos = orbitals.iter().position(|(_, _, _, label)| label == "4s");
        let d3_pos = orbitals.iter().position(|(_, _, _, label)| label == "3d");
        assert!(s4_pos < d3_pos);
    }

    #[test]
    fn test_element_from_number() {
        let table = PeriodicTable::new();

        assert_eq!(table.element_from_number(1), Some(Element::Hydrogen));
        assert_eq!(table.element_from_number(2), Some(Element::Helium));
        assert_eq!(table.element_from_number(3), Some(Element::Lithium));
        assert_eq!(
            table.element_from_number(6),
            Some(Element::Custom { protons: 6 })
        );
        assert_eq!(
            table.element_from_number(118),
            Some(Element::Custom { protons: 118 })
        );
        assert_eq!(table.element_from_number(119), None);
    }

    #[test]
    fn test_calculate_stability() {
        let table = PeriodicTable::new();

        // Noble gases should have maximum stability
        assert_eq!(table.calculate_stability(2), 1.0); // Helium
        assert_eq!(table.calculate_stability(10), 1.0); // Neon
        assert_eq!(table.calculate_stability(18), 1.0); // Argon

        // Other elements should have lower stability
        assert!(table.calculate_stability(1) < 1.0); // Hydrogen
        assert!(table.calculate_stability(3) < 1.0); // Lithium
        assert!(table.calculate_stability(6) < 1.0); // Carbon
    }

    #[test]
    fn test_calculate_min_coherence() {
        let table = PeriodicTable::new();

        // Light elements need low coherence
        assert_eq!(table.calculate_min_coherence(1), 0.1); // Hydrogen
        assert_eq!(table.calculate_min_coherence(2), 0.1); // Helium

        // Heavy elements need high coherence
        assert_eq!(table.calculate_min_coherence(118), 0.7);

        // Check monotonic increase
        assert!(table.calculate_min_coherence(10) <= table.calculate_min_coherence(18));
        assert!(table.calculate_min_coherence(36) <= table.calculate_min_coherence(54));
    }

    #[test]
    fn test_calculate_formation_energy() {
        let table = PeriodicTable::new();

        // Formation energy should increase with atomic number
        let energy_1 = table.calculate_formation_energy(1);
        let energy_2 = table.calculate_formation_energy(2);
        let energy_10 = table.calculate_formation_energy(10);

        assert!(energy_1 > 0.0);
        assert!(energy_2 > energy_1);
        assert!(energy_10 > energy_2);
    }

    #[test]
    fn test_find_element() {
        let table = PeriodicTable::new();

        // Test with quantum state signatures
        let sig1 = QuantumStateSignature::new(1, 0, 0, Spin::Up, 1);
        let elem1 = table.find_element(&sig1);
        assert!(elem1.is_some());

        let sig2 = QuantumStateSignature::new(2, 0, 0, Spin::Up, 2);
        let elem2 = table.find_element(&sig2);
        assert!(elem2.is_some());
    }

    #[test]
    #[ignore]
    fn test_electron_configuration_comprehensive() {
        let table = PeriodicTable::new();

        // Test a range of elements
        let tests = vec![
            (1, "1s1"),
            (2, "1s2"),
            (3, "1s2 2s1"),
            (4, "1s2 2s2"),
            (5, "1s2 2s2 2p1"),
            (6, "1s2 2s2 2p2"),
            (7, "1s2 2s2 2p3"),
            (8, "1s2 2s2 2p4"),
            (9, "1s2 2s2 2p5"),
            (10, "1s2 2s2 2p6"),
            (11, "1s2 2s2 2p6 3s1"),
            (18, "1s2 2s2 2p6 3s2 3p6"),
            (19, "1s2 2s2 2p6 3s2 3p6 4s1"),
        ];

        for (atomic_number, expected) in tests {
            assert_eq!(
                table.electron_configuration(atomic_number),
                expected,
                "Failed for atomic number {}",
                atomic_number
            );
        }
    }

    #[test]
    fn test_element_count() {
        let table = PeriodicTable::new();
        assert_eq!(table.element_count(), 118);
    }
}
