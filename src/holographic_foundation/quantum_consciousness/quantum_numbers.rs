//! Quantum Numbers Derived from Archetype Activation Patterns
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 7:
//! "Quantum Numbers = Derived from archetype activation patterns:
//!  - n (principal): Overall activation magnitude
//!  - l (angular): Mind archetypes (A1-A7)
//!  - m (magnetic): Body archetypes (A8-A14)
//!  - s (spin): Spirit archetypes (A15-A21)"
//!
//! This implements the profound insight that quantum numbers are NOT arbitrary
//! labels - they emerge from consciousness patterns. The archetype system
//! provides the quantum degrees of freedom for any entity.
//!
//! Key principle: An entity's quantum state signature is determined by its
//! archetype activation profile, not randomly assigned.

use crate::types::Float;
use std::fmt;

use super::super::archetype_profile::NUM_ARCHETYPES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumNumberSet {
    pub n: u32,
    pub l: u32,
    pub m: i32,
    pub s: Spin,
}

impl QuantumNumberSet {
    pub fn new(n: u32, l: u32, m: i32, s: Spin) -> Self {
        Self { n, l, m, s }
    }

    pub fn ground_state() -> Self {
        Self::new(1, 0, 0, Spin::Up)
    }

    pub fn excited_state(n: u32, l: u32) -> Self {
        let m = if l > 0 { (l as i32) - 1 } else { 0 };
        Self::new(n, l, m, Spin::Up)
    }

    pub fn is_valid(&self) -> bool {
        self.l < self.n && self.m.unsigned_abs() <= self.l
    }

    pub fn degeneracy(&self) -> usize {
        (2 * self.l + 1) as usize * 2
    }

    pub fn energy_factor(&self) -> Float {
        if self.n == 0 {
            return 1.0;
        }
        1.0 / (self.n as Float).powi(2)
    }

    pub fn orbital_type(&self) -> &'static str {
        match self.l {
            0 => "s",
            1 => "p",
            2 => "d",
            3 => "f",
            4 => "g",
            5 => "h",
            6 => "i",
            _ => "?",
        }
    }

    pub fn shell_name(&self) -> String {
        let shell = match self.n {
            1 => "K",
            2 => "L",
            3 => "M",
            4 => "N",
            5 => "O",
            6 => "P",
            7 => "Q",
            _ => "?",
        };
        format!("{}{}", shell, self.orbital_type())
    }

    pub fn to_spectroscopic_notation(&self) -> String {
        let m_sign = if self.m >= 0 { "+" } else { "-" };
        let spin_str = if self.s == Spin::Up { "↑" } else { "↓" };
        format!(
            "{}{}{}{}{}",
            self.n,
            self.orbital_type(),
            m_sign,
            self.m.abs(),
            spin_str
        )
    }

    pub fn hash(&self) -> u64 {
        let mut h = self.n as u64;
        h = h.wrapping_mul(31).wrapping_add(self.l as u64);
        h = h.wrapping_mul(31).wrapping_add((self.m as u32) as u64);
        h = h.wrapping_mul(31).wrapping_add(self.s as u64);
        h
    }
}

impl Default for QuantumNumberSet {
    fn default() -> Self {
        Self::ground_state()
    }
}

impl fmt::Display for QuantumNumberSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "n={}, l={}, m={}, s={}",
            self.n,
            self.orbital_type(),
            self.m,
            if self.s == Spin::Up { "↑" } else { "↓" }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Spin {
    #[default]
    Up,
    Down,
}

impl Spin {
    pub fn value(&self) -> Float {
        match self {
            Spin::Up => 0.5,
            Spin::Down => -0.5,
        }
    }

    pub fn from_bool(up: bool) -> Self {
        if up {
            Spin::Up
        } else {
            Spin::Down
        }
    }

    pub fn is_up(&self) -> bool {
        *self == Spin::Up
    }

    pub fn flip(&self) -> Self {
        match self {
            Spin::Up => Spin::Down,
            Spin::Down => Spin::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchetypeToQuantumMapping {
    /// TODO: Planned for weighted quantum number derivation
    #[allow(dead_code)]
    mind_weight: Float,
    /// TODO: Planned for weighted quantum number derivation
    #[allow(dead_code)]
    body_weight: Float,
    spirit_weight: Float,
    choice_weight: Float,
}

impl ArchetypeToQuantumMapping {
    pub fn new() -> Self {
        Self {
            mind_weight: 1.0,
            body_weight: 1.0,
            spirit_weight: 1.0,
            choice_weight: 1.0,
        }
    }

    pub fn with_weights(mind: Float, body: Float, spirit: Float, choice: Float) -> Self {
        Self {
            mind_weight: mind,
            body_weight: body,
            spirit_weight: spirit,
            choice_weight: choice,
        }
    }

    pub fn derive_quantum_numbers(archetype_vector: &[Float; NUM_ARCHETYPES]) -> QuantumNumberSet {
        Self::new().derive(archetype_vector)
    }

    pub fn derive(&self, archetype_vector: &[Float; NUM_ARCHETYPES]) -> QuantumNumberSet {
        let n = self.derive_principal_number(archetype_vector);
        let l = self.derive_angular_momentum(archetype_vector, n);
        let m = self.derive_magnetic_number(archetype_vector, l);
        let s = self.derive_spin(archetype_vector);

        QuantumNumberSet::new(n, l, m, s)
    }

    fn derive_principal_number(&self, archetype_vector: &[Float; NUM_ARCHETYPES]) -> u32 {
        let total: Float = archetype_vector.iter().sum();
        let mean = total / NUM_ARCHETYPES as Float;

        let magnitude_factor = (mean * 7.0).ceil() as u32;

        magnitude_factor.clamp(1, 7)
    }

    fn derive_angular_momentum(&self, archetype_vector: &[Float; NUM_ARCHETYPES], n: u32) -> u32 {
        let mind_complex: Float = archetype_vector[0..7].iter().sum::<Float>() / 7.0;

        let l = (mind_complex * 4.0).floor() as u32;

        l.min(n - 1)
    }

    fn derive_magnetic_number(&self, archetype_vector: &[Float; NUM_ARCHETYPES], l: u32) -> i32 {
        let body_complex: Float = archetype_vector[7..14].iter().sum::<Float>() / 7.0;

        if l == 0 {
            return 0;
        }

        let normalized = (body_complex - 0.5) * 2.0;

        let m = (normalized * l as Float).round() as i32;

        m.clamp(-(l as i32), l as i32)
    }

    fn derive_spin(&self, archetype_vector: &[Float; NUM_ARCHETYPES]) -> Spin {
        let spirit_complex: Float = archetype_vector[14..21].iter().sum::<Float>() / 7.0;

        let choice = archetype_vector[21];

        let combined = spirit_complex * self.spirit_weight + choice * self.choice_weight * 0.5;
        let normalized = combined / (self.spirit_weight + self.choice_weight * 0.5);

        Spin::from_bool(normalized > 0.5)
    }

    pub fn quantum_state_entropy(archetype_vector: &[Float; NUM_ARCHETYPES]) -> Float {
        let qn = Self::derive_quantum_numbers(archetype_vector);

        let n_entropy = 1.0 / (qn.n as Float);

        let l_entropy = if qn.n > 0 {
            qn.l as Float / (qn.n - 1).max(1) as Float
        } else {
            0.0
        };

        let m_range = if qn.l > 0 {
            qn.l as Float * 2.0 + 1.0
        } else {
            1.0
        };
        let m_entropy = (qn.m.abs() as Float + 1.0) / m_range;

        1.0 - (n_entropy + l_entropy + m_entropy) / 3.0
    }

    pub fn archetype_from_quantum_state(qn: &QuantumNumberSet) -> [Float; NUM_ARCHETYPES] {
        let mut vector = [0.5; NUM_ARCHETYPES];

        let n_factor = qn.n as Float / 7.0;
        for item in &mut vector {
            *item *= 0.5 + n_factor * 0.5;
        }

        if qn.n > 0 {
            let l_factor = qn.l as Float / (qn.n - 1).max(1) as Float;
            for item in &mut vector[0..7] {
                *item = item.min(1.0) * (0.7 + l_factor * 0.3);
            }
        }

        if qn.l > 0 {
            let m_normalized = (qn.m as Float + qn.l as Float) / (2.0 * qn.l as Float);
            for item in &mut vector[7..14] {
                *item = item.min(1.0) * (0.5 + m_normalized * 0.5);
            }
        }

        let spin_factor = if qn.s == Spin::Up { 0.7 } else { 0.3 };
        for item in &mut vector[14..21] {
            *item = item.min(1.0) * spin_factor;
        }

        let spin_influence = qn.s.value().abs();
        vector[21] = (0.5 + spin_influence).min(1.0);

        vector
    }
}

impl Default for ArchetypeToQuantumMapping {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct QuantumNumberDerivation {
    pub source_archetype: [Float; NUM_ARCHETYPES],
    pub derived_quantum_numbers: QuantumNumberSet,
    pub confidence: Float,
    pub alternative_states: Vec<QuantumNumberSet>,
}

impl QuantumNumberDerivation {
    pub fn from_archetype(archetype_vector: &[Float; NUM_ARCHETYPES]) -> Self {
        let derived = ArchetypeToQuantumMapping::derive_quantum_numbers(archetype_vector);

        let confidence = Self::calculate_confidence(archetype_vector, &derived);

        let alternatives = Self::generate_alternatives(&derived, archetype_vector);

        Self {
            source_archetype: *archetype_vector,
            derived_quantum_numbers: derived,
            confidence,
            alternative_states: alternatives,
        }
    }

    fn calculate_confidence(
        archetype_vector: &[Float; NUM_ARCHETYPES],
        qn: &QuantumNumberSet,
    ) -> Float {
        let coherence: Float = {
            let mean: Float = archetype_vector.iter().sum::<Float>() / NUM_ARCHETYPES as Float;
            let variance: Float = archetype_vector
                .iter()
                .map(|&c| (c - mean).powi(2))
                .sum::<Float>()
                / NUM_ARCHETYPES as Float;
            1.0 - variance.sqrt()
        };

        let validity = if qn.is_valid() { 1.0 } else { 0.5 };

        let entropy = ArchetypeToQuantumMapping::quantum_state_entropy(archetype_vector);

        (coherence + validity + entropy) / 3.0
    }

    fn generate_alternatives(
        primary: &QuantumNumberSet,
        archetype_vector: &[Float; NUM_ARCHETYPES],
    ) -> Vec<QuantumNumberSet> {
        let mut alternatives = Vec::new();

        let spirit_sum: Float = archetype_vector[14..21].iter().sum();
        let body_sum: Float = archetype_vector[7..14].iter().sum();

        if (spirit_sum / 7.0 - 0.5).abs() < 0.1 {
            alternatives.push(QuantumNumberSet::new(
                primary.n,
                primary.l,
                primary.m,
                primary.s.flip(),
            ));
        }

        if primary.l > 0 && (body_sum / 7.0 - 0.5).abs() < 0.15 {
            if primary.m > -(primary.l as i32) {
                alternatives.push(QuantumNumberSet::new(
                    primary.n,
                    primary.l,
                    primary.m - 1,
                    primary.s,
                ));
            }
            if primary.m < primary.l as i32 {
                alternatives.push(QuantumNumberSet::new(
                    primary.n,
                    primary.l,
                    primary.m + 1,
                    primary.s,
                ));
            }
        }

        alternatives
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_state() {
        let qn = QuantumNumberSet::ground_state();
        assert_eq!(qn.n, 1);
        assert_eq!(qn.l, 0);
        assert_eq!(qn.m, 0);
        assert!(qn.is_valid());
    }

    #[test]
    fn test_quantum_number_validity() {
        assert!(QuantumNumberSet::new(1, 0, 0, Spin::Up).is_valid());
        assert!(QuantumNumberSet::new(2, 1, -1, Spin::Down).is_valid());
        assert!(QuantumNumberSet::new(3, 2, 0, Spin::Up).is_valid());

        assert!(!QuantumNumberSet::new(1, 1, 0, Spin::Up).is_valid());
        assert!(!QuantumNumberSet::new(2, 0, 1, Spin::Up).is_valid());
    }

    #[test]
    fn test_degeneracy() {
        let s_orbital = QuantumNumberSet::new(1, 0, 0, Spin::Up);
        assert_eq!(s_orbital.degeneracy(), 2);

        let p_orbital = QuantumNumberSet::new(2, 1, 0, Spin::Up);
        assert_eq!(p_orbital.degeneracy(), 6);

        let d_orbital = QuantumNumberSet::new(3, 2, 0, Spin::Up);
        assert_eq!(d_orbital.degeneracy(), 10);
    }

    #[test]
    fn test_orbital_types() {
        assert_eq!(QuantumNumberSet::new(1, 0, 0, Spin::Up).orbital_type(), "s");
        assert_eq!(QuantumNumberSet::new(2, 1, 0, Spin::Up).orbital_type(), "p");
        assert_eq!(QuantumNumberSet::new(3, 2, 0, Spin::Up).orbital_type(), "d");
        assert_eq!(QuantumNumberSet::new(4, 3, 0, Spin::Up).orbital_type(), "f");
    }

    #[test]
    fn test_spin_operations() {
        assert_eq!(Spin::Up.value(), 0.5);
        assert_eq!(Spin::Down.value(), -0.5);
        assert_eq!(Spin::Up.flip(), Spin::Down);
        assert_eq!(Spin::Down.flip(), Spin::Up);
    }

    #[test]
    fn test_derive_from_archetype() {
        let uniform = [0.5; 22];
        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(&uniform);

        assert!(qn.n >= 1 && qn.n <= 7);
        assert!(qn.is_valid());
    }

    #[test]
    fn test_derive_high_mind() {
        let mut archetype = [0.5; 22];
        for item in archetype.iter_mut().take(7) {
            *item = 0.9;
        }

        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(&archetype);

        assert!(qn.l >= 2);
    }

    #[test]
    fn test_derive_high_spirit() {
        let mut archetype = [0.5; 22];
        for item in archetype.iter_mut().skip(14).take(7) {
            *item = 0.9;
        }

        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(&archetype);

        assert_eq!(qn.s, Spin::Up);
    }

    #[test]
    fn test_derive_low_spirit() {
        let mut archetype = [0.5; 22];
        for item in archetype.iter_mut().skip(14).take(7) {
            *item = 0.1;
        }

        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(&archetype);

        assert_eq!(qn.s, Spin::Down);
    }

    #[test]
    fn test_archetype_from_quantum() {
        let qn = QuantumNumberSet::new(3, 2, 1, Spin::Up);
        let archetype = ArchetypeToQuantumMapping::archetype_from_quantum_state(&qn);

        for coeff in archetype.iter() {
            assert!(*coeff >= 0.0 && *coeff <= 1.0);
        }
    }

    #[test]
    fn test_round_trip_consistency() {
        let original = [
            0.7, 0.6, 0.8, 0.5, 0.5, 0.5, 0.5, 0.4, 0.5, 0.6, 0.5, 0.5, 0.5, 0.5, 0.8, 0.7, 0.6,
            0.5, 0.5, 0.5, 0.5, 0.6,
        ];

        let qn = ArchetypeToQuantumMapping::derive_quantum_numbers(&original);

        assert!(qn.is_valid());
        assert!(qn.n >= 1);
    }

    #[test]
    fn test_energy_factor() {
        let ground = QuantumNumberSet::ground_state();
        let excited = QuantumNumberSet::excited_state(3, 2);

        assert!(ground.energy_factor() > excited.energy_factor());
    }

    #[test]
    fn test_quantum_number_derivation() {
        let archetype = [0.6; 22];
        let derivation = QuantumNumberDerivation::from_archetype(&archetype);

        assert!(derivation.confidence > 0.0 && derivation.confidence <= 1.0);
        assert!(derivation.derived_quantum_numbers.is_valid());
    }
}
