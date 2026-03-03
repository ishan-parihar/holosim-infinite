//! Atomic Manifestation: Atoms Emerging from Field Coherence
//!
//! From HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md Phase 6:
//! "Atoms emerge as STABLE ATTRACTOR FIELDS at atomic resolution (~10^-15m).
//!  Mass/charge derived from archetype patterns - NOT hardcoded."
//!
//! Atoms don't "form" in the traditional sense. They MANIFEST when the
//! holographic field reaches sufficient coherence at atomic resolution.
//! The field falls into stable attractor basins, and we call these "atoms".
//!
//! Key Insight: Proton/Electron/Neutron emerge from archetype combinations:
//! - Proton: Mind-complex dominant (A1-A7) → outward expression → positive charge
//! - Electron: Spirit-complex dominant (A15-A21) → inward reception → negative charge
//! - Neutron: Balanced mind/spirit → neutral
//!
//! Phase 6: Mass/charge now DERIVED from archetype patterns using ParticleProperties

use crate::types::Float;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::field_state::{HolographicFieldState, Position3D};
use super::super::quantum_consciousness::quantum_numbers::QuantumNumberSet;
use super::super::scale_level::ScaleLevel;
use super::attractor_field::{AttractorField, AttractorStability, FieldConfiguration};
use super::element_attractor::{ChargeConfiguration, ElementAttractorField, ElementIdentity};
use super::particle_derivation::{ParticleArchetypePattern, ParticleProperties, ParticleType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManifestationId(u64);

impl ManifestationId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ManifestationType {
    Proton,
    Electron,
    Neutron,
    Nucleus,
    CompleteAtom,
    Ion,
}

impl ManifestationType {
    pub fn archetype_dominance(&self) -> &'static str {
        match self {
            ManifestationType::Proton => "Mind-complex (A1-A7)",
            ManifestationType::Electron => "Spirit-complex (A15-A21)",
            ManifestationType::Neutron => "Balanced mind/spirit",
            ManifestationType::Nucleus => "Mind-dominant collective",
            ManifestationType::CompleteAtom => "Full archetype integration",
            ManifestationType::Ion => "Perturbed balance",
        }
    }

    pub fn charge_sign(&self) -> i32 {
        match self {
            ManifestationType::Proton => 1,
            ManifestationType::Electron => -1,
            ManifestationType::Neutron => 0,
            ManifestationType::Nucleus => 1,
            ManifestationType::CompleteAtom => 0,
            ManifestationType::Ion => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManifestationConditions {
    pub coherence_threshold: Float,
    pub energy_threshold: Float,
    pub stability_threshold: Float,
    pub field_density_threshold: Float,
    pub quantum_coherence_min: Float,
    pub archetype_alignment_min: Float,
    pub min_spatial_resolution: Float,
}

impl ManifestationConditions {
    pub fn for_hydrogen() -> Self {
        Self {
            coherence_threshold: 0.3,
            energy_threshold: 13.6,
            stability_threshold: 0.5,
            field_density_threshold: 0.1,
            quantum_coherence_min: 0.7,
            archetype_alignment_min: 0.6,
            min_spatial_resolution: 1e-10,
        }
    }

    pub fn for_element(atomic_number: u32) -> Self {
        let base_coherence = 0.3 + (atomic_number as Float / 118.0) * 0.4;
        let base_energy = 13.6 * (atomic_number as Float).powi(2);

        Self {
            coherence_threshold: base_coherence,
            energy_threshold: base_energy,
            stability_threshold: 0.4 + (atomic_number as Float / 118.0) * 0.3,
            field_density_threshold: 0.1 * (1.0 + atomic_number as Float * 0.01),
            quantum_coherence_min: 0.5 + (atomic_number as Float / 118.0) * 0.3,
            archetype_alignment_min: 0.5,
            min_spatial_resolution: 1e-10 / (atomic_number as Float).powf(1.0 / 3.0),
        }
    }

    pub fn check(&self, field: &HolographicFieldState, position: &Position3D) -> bool {
        if let Some(node) = field.get_node_at(position) {
            let coherence = node.coherence;
            let amplitude = node.get_amplitude_at_scale(ScaleLevel::Atomic);

            coherence >= self.coherence_threshold
                && amplitude.magnitude() >= self.field_density_threshold
        } else {
            false
        }
    }

    pub fn check_attractor(&self, attractor: &AttractorField) -> bool {
        let stability = attractor.stability().stability_factor();
        let energy = attractor.energy();
        let coherence = attractor.configuration().coherence;

        stability >= self.stability_threshold
            && energy >= self.energy_threshold * 0.1
            && coherence >= self.coherence_threshold * 0.8
    }
}

impl Default for ManifestationConditions {
    fn default() -> Self {
        Self::for_hydrogen()
    }
}

#[derive(Debug, Clone)]
pub struct AtomFormationEvent {
    pub id: ManifestationId,
    pub element: ElementIdentity,
    pub atomic_number: u32,
    pub position: Position3D,
    pub manifestation_type: ManifestationType,
    pub formation_energy: Float,
    pub coherence_at_formation: Float,
    pub quantum_numbers: QuantumNumberSet,
    pub archetype_vector: [Float; NUM_ARCHETYPES],
    pub timestamp: Float,
    pub stability: AttractorStability,
}

impl AtomFormationEvent {
    pub fn new(
        element: ElementIdentity,
        position: Position3D,
        manifestation_type: ManifestationType,
        formation_energy: Float,
        coherence: Float,
        quantum_numbers: QuantumNumberSet,
        archetype_vector: [Float; NUM_ARCHETYPES],
        timestamp: Float,
    ) -> Self {
        let stability = AttractorStability::from_coherence(coherence, element.is_noble_gas());

        Self {
            id: ManifestationId::new(rand_u64()),
            element,
            atomic_number: element.atomic_number(),
            position,
            manifestation_type,
            formation_energy,
            coherence_at_formation: coherence,
            quantum_numbers,
            archetype_vector,
            timestamp,
            stability,
        }
    }

    pub fn hydrogen(position: Position3D, coherence: Float, timestamp: Float) -> Self {
        let config = FieldConfiguration::hydrogen_configuration();
        Self::new(
            ElementIdentity::Hydrogen,
            position,
            ManifestationType::CompleteAtom,
            13.6,
            coherence,
            config.quantum_numbers.clone(),
            config.archetype_vector,
            timestamp,
        )
    }

    pub fn is_stable(&self) -> bool {
        matches!(
            self.stability,
            AttractorStability::Stable | AttractorStability::NobleGas
        )
    }

    pub fn binding_energy(&self) -> Float {
        self.formation_energy * self.stability.stability_factor()
    }
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}

#[derive(Debug, Clone)]
pub struct SubatomicManifestation {
    pub manifestation_type: ManifestationType,
    pub position: Position3D,
    pub archetype_contribution: [Float; NUM_ARCHETYPES],
    pub charge: Float,
    pub mass_factor: Float,
    pub spin: Float,
}

impl SubatomicManifestation {
    /// Phase 6: Proton with mass/charge DERIVED from archetype pattern
    pub fn proton(position: Position3D, archetype: [Float; NUM_ARCHETYPES]) -> Self {
        let props = ParticleProperties::derive_from_archetype(ParticleType::Proton, archetype);

        Self {
            manifestation_type: ManifestationType::Proton,
            position,
            archetype_contribution: archetype,
            charge: props.charge,
            mass_factor: props.mass_factor,
            spin: props.spin,
        }
    }

    /// Phase 6: Electron with mass/charge DERIVED from archetype pattern
    pub fn electron(position: Position3D, archetype: [Float; NUM_ARCHETYPES]) -> Self {
        let props = ParticleProperties::derive_from_archetype(ParticleType::Electron, archetype);

        Self {
            manifestation_type: ManifestationType::Electron,
            position,
            archetype_contribution: archetype,
            charge: props.charge,
            mass_factor: props.mass_factor,
            spin: props.spin,
        }
    }

    /// Phase 6: Neutron with mass DERIVED from archetype pattern (charge = 0)
    pub fn neutron(position: Position3D, archetype: [Float; NUM_ARCHETYPES]) -> Self {
        let props = ParticleProperties::derive_from_archetype(ParticleType::Neutron, archetype);

        Self {
            manifestation_type: ManifestationType::Neutron,
            position,
            archetype_contribution: archetype,
            charge: props.charge,
            mass_factor: props.mass_factor,
            spin: props.spin,
        }
    }

    /// Create from archetype pattern with DERIVED properties (Phase 6)
    pub fn from_archetype_pattern(
        manifestation_type: ManifestationType,
        position: Position3D,
        archetype: [Float; NUM_ARCHETYPES],
    ) -> Self {
        match manifestation_type {
            ManifestationType::Proton => Self::proton(position, archetype),
            ManifestationType::Electron => Self::electron(position, archetype),
            ManifestationType::Neutron => Self::neutron(position, archetype),
            _ => Self {
                manifestation_type,
                position,
                archetype_contribution: archetype,
                charge: 0.0,
                mass_factor: 1.0,
                spin: 0.5,
            },
        }
    }

    /// Create proton with canonical archetype pattern
    pub fn proton_canonical(position: Position3D) -> Self {
        let archetype = ParticleArchetypePattern::proton_pattern();
        Self::proton(position, archetype)
    }

    /// Create electron with canonical archetype pattern
    pub fn electron_canonical(position: Position3D) -> Self {
        let archetype = ParticleArchetypePattern::electron_pattern();
        Self::electron(position, archetype)
    }

    /// Create neutron with canonical archetype pattern
    pub fn neutron_canonical(position: Position3D) -> Self {
        let archetype = ParticleArchetypePattern::neutron_pattern();
        Self::neutron(position, archetype)
    }

    /// Get mass in electron mass units
    pub fn mass_in_electron_units(&self) -> Float {
        self.mass_factor
    }

    /// Check if this particle is stable
    pub fn is_stable(&self) -> bool {
        matches!(
            self.manifestation_type,
            ManifestationType::Proton | ManifestationType::Electron
        )
    }
}

#[derive(Debug, Clone)]
pub struct AtomicManifestation {
    id: ManifestationId,
    element: ElementAttractorField,
    position: Position3D,
    subatomic_components: Vec<SubatomicManifestation>,
    formation_event: Option<AtomFormationEvent>,
    current_energy: Float,
    lifetime: Float,
    decay_rate: Float,
    is_manifested: bool,
}

impl AtomicManifestation {
    pub fn new(element: ElementAttractorField, position: Position3D) -> Self {
        let id = ManifestationId::new(rand_u64());
        let formation_energy = element.formation_energy();
        let decay_rate = if element.is_noble_gas() {
            0.0
        } else {
            1e-18 / (element.atomic_number() as Float).sqrt()
        };

        Self {
            id,
            element,
            position,
            subatomic_components: Vec::new(),
            formation_event: None,
            current_energy: 0.0,
            lifetime: 0.0,
            decay_rate,
            is_manifested: false,
        }
    }

    pub fn hydrogen(position: Position3D) -> Self {
        Self::new(ElementAttractorField::hydrogen(), position)
    }

    pub fn from_field(field: &HolographicFieldState, position: Position3D) -> Option<Self> {
        let node = field.get_node_at(&position)?;
        let coherence = node.coherence;

        if coherence < 0.3 {
            return None;
        }

        let config = FieldConfiguration::new(node.archetype_vector);
        let attractor = AttractorField::new(config.clone());

        if !attractor.is_stable() {
            return None;
        }

        let atomic_number = Self::estimate_atomic_number(&config);
        let element = ElementAttractorField::from_atomic_number(atomic_number);

        Some(Self::new(element, position))
    }

    fn estimate_atomic_number(config: &FieldConfiguration) -> u32 {
        let n = config.quantum_numbers.n;
        let coherence = config.coherence;

        let estimated = (n as Float * coherence * 10.0) as u32;
        estimated.clamp(1, 118)
    }

    pub fn attempt_manifestation(
        &mut self,
        conditions: &ManifestationConditions,
        timestamp: Float,
    ) -> bool {
        if conditions.check_attractor(self.element.attractor()) {
            self.manifest(timestamp);
            true
        } else {
            false
        }
    }

    pub fn manifest(&mut self, timestamp: Float) {
        self.is_manifested = true;
        self.current_energy = self.element.formation_energy();

        let event = AtomFormationEvent::new(
            *self.element.identity(),
            self.position.clone(),
            ManifestationType::CompleteAtom,
            self.current_energy,
            self.element.configuration().coherence,
            *self.element.quantum_numbers(),
            self.element.configuration().archetype_vector,
            timestamp,
        );

        self.formation_event = Some(event);
        self.generate_subatomic_components();
    }

    fn generate_subatomic_components(&mut self) {
        let z = self.element.atomic_number();
        let archetype = self.element.configuration().archetype_vector;

        let nucleus_offset = Position3D {
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
        };

        for _ in 0..z {
            self.subatomic_components
                .push(SubatomicManifestation::proton(
                    nucleus_offset.clone(),
                    archetype,
                ));
        }

        let neutron_count = Self::neutron_count_for_element(z);
        for _ in 0..neutron_count {
            self.subatomic_components
                .push(SubatomicManifestation::neutron(
                    nucleus_offset.clone(),
                    archetype,
                ));
        }

        for i in 0..z {
            let angle = (i as Float) * 2.0 * std::f64::consts::PI / (z as Float);
            let radius = self.element.atomic_radius() * 1e-12;
            let electron_pos = Position3D {
                x: self.position.x + radius * angle.cos(),
                y: self.position.y + radius * angle.sin(),
                z: self.position.z,
            };
            self.subatomic_components
                .push(SubatomicManifestation::electron(electron_pos, archetype));
        }
    }

    fn neutron_count_for_element(z: u32) -> u32 {
        match z {
            1 => 0,
            2 => 2,
            3..=20 => z,
            _ => (z as Float * 1.5) as u32,
        }
    }

    pub fn id(&self) -> &ManifestationId {
        &self.id
    }

    pub fn element(&self) -> &ElementAttractorField {
        &self.element
    }

    pub fn position(&self) -> &Position3D {
        &self.position
    }

    pub fn atomic_number(&self) -> u32 {
        self.element.atomic_number()
    }

    pub fn symbol(&self) -> &'static str {
        self.element.symbol()
    }

    pub fn charge(&self) -> &ChargeConfiguration {
        self.element.charge()
    }

    pub fn formation_event(&self) -> Option<&AtomFormationEvent> {
        self.formation_event.as_ref()
    }

    pub fn energy(&self) -> Float {
        self.current_energy
    }

    pub fn lifetime(&self) -> Float {
        self.lifetime
    }

    pub fn is_manifested(&self) -> bool {
        self.is_manifested
    }

    pub fn subatomic_components(&self) -> &[SubatomicManifestation] {
        &self.subatomic_components
    }

    pub fn proton_count(&self) -> usize {
        self.subatomic_components
            .iter()
            .filter(|c| c.manifestation_type == ManifestationType::Proton)
            .count()
    }

    pub fn electron_count(&self) -> usize {
        self.subatomic_components
            .iter()
            .filter(|c| c.manifestation_type == ManifestationType::Electron)
            .count()
    }

    pub fn neutron_count(&self) -> usize {
        self.subatomic_components
            .iter()
            .filter(|c| c.manifestation_type == ManifestationType::Neutron)
            .count()
    }

    pub fn update(&mut self, dt: Float, field: &HolographicFieldState) {
        if !self.is_manifested {
            return;
        }

        self.lifetime += dt;

        if self.decay_rate > 0.0 && !self.element.is_noble_gas() {
            self.current_energy *= 1.0 - self.decay_rate * dt;
        }

        if let Some(node) = field.get_node_at(&self.position) {
            let field_energy = node.get_amplitude_at_scale(ScaleLevel::Atomic).magnitude();
            self.current_energy += field_energy * dt * 0.1;
        }

        if self.current_energy < self.element.formation_energy() * 0.1 {
            self.dematerialize();
        }
    }

    pub fn dematerialize(&mut self) {
        self.is_manifested = false;
        self.subatomic_components.clear();
    }

    pub fn ionize(&mut self, electron_change: i32) {
        self.element.ionize(electron_change);

        if electron_change > 0 {
            for _ in 0..electron_change {
                if let Some(idx) = self
                    .subatomic_components
                    .iter()
                    .position(|c| c.manifestation_type == ManifestationType::Electron)
                {
                    self.subatomic_components.remove(idx);
                }
            }
        }
    }

    pub fn excite(&mut self, energy: Float) {
        self.current_energy += energy;
    }

    pub fn can_absorb(&self, other: &AtomicManifestation) -> bool {
        if !self.is_manifested || !other.is_manifested {
            return false;
        }

        let distance = self.position.distance(&other.position);
        let combined_radius =
            (self.element.atomic_radius() + other.element.atomic_radius()) * 1e-12;

        distance < combined_radius * 2.0
    }

    pub fn absorb(&mut self, other: &AtomicManifestation) -> Option<AtomicManifestation> {
        if !self.can_absorb(other) {
            return None;
        }

        let new_z = self.atomic_number() + other.atomic_number();
        if new_z > 118 {
            return None;
        }

        let new_element = ElementAttractorField::from_atomic_number(new_z);
        let mut new_manifestation = AtomicManifestation::new(new_element, self.position.clone());
        new_manifestation.current_energy = self.current_energy + other.current_energy;
        new_manifestation.lifetime = self.lifetime;

        Some(new_manifestation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifestation_conditions_hydrogen() {
        let conditions = ManifestationConditions::for_hydrogen();
        assert!(conditions.coherence_threshold > 0.0);
        assert!(conditions.energy_threshold > 0.0);
    }

    #[test]
    fn test_manifestation_conditions_heavier_element() {
        let conditions_h = ManifestationConditions::for_element(1);
        let conditions_u = ManifestationConditions::for_element(92);

        assert!(conditions_u.coherence_threshold > conditions_h.coherence_threshold);
        assert!(conditions_u.energy_threshold > conditions_h.energy_threshold);
    }

    #[test]
    fn test_hydrogen_manifestation() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let atom = AtomicManifestation::hydrogen(pos.clone());

        assert_eq!(atom.atomic_number(), 1);
        assert_eq!(atom.symbol(), "H");
        assert!(!atom.is_manifested());
    }

    #[test]
    fn test_atom_manifestation() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut atom = AtomicManifestation::hydrogen(pos);

        atom.manifest(0.0);

        assert!(atom.is_manifested());
        assert!(atom.formation_event().is_some());
        assert!(atom.proton_count() >= 1);
        assert!(atom.electron_count() >= 1);
    }

    #[test]
    fn test_subatomic_manifestation_proton() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let archetype = [0.5; NUM_ARCHETYPES];
        let proton = SubatomicManifestation::proton(pos, archetype);

        assert_eq!(proton.manifestation_type, ManifestationType::Proton);
        assert!(proton.charge > 0.0);
    }

    #[test]
    fn test_subatomic_manifestation_electron() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let archetype = [0.5; NUM_ARCHETYPES];
        let electron = SubatomicManifestation::electron(pos, archetype);

        assert_eq!(electron.manifestation_type, ManifestationType::Electron);
        assert!(electron.charge < 0.0);
    }

    #[test]
    fn test_subatomic_manifestation_neutron() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let archetype = [0.5; NUM_ARCHETYPES];
        let neutron = SubatomicManifestation::neutron(pos, archetype);

        assert_eq!(neutron.manifestation_type, ManifestationType::Neutron);
        assert!(neutron.charge.abs() < 1e-10);
    }

    #[test]
    fn test_ionization() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut atom = AtomicManifestation::new(ElementAttractorField::from_atomic_number(11), pos);

        atom.manifest(0.0);
        let initial_electrons = atom.electron_count();

        atom.ionize(1);

        assert!(atom.electron_count() < initial_electrons);
        assert!(atom.charge().is_cation());
    }

    #[test]
    fn test_atom_update() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut atom = AtomicManifestation::hydrogen(pos);
        atom.manifest(0.0);

        let field = HolographicFieldState::new(1.0);
        atom.update(1.0, &field);

        assert!(atom.lifetime() > 0.0);
    }

    #[test]
    fn test_atom_formation_event() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let event = AtomFormationEvent::hydrogen(pos, 0.8, 1.0);

        assert_eq!(event.atomic_number, 1);
        assert!(event.coherence_at_formation > 0.0);
        assert!(event.formation_energy > 0.0);
    }

    #[test]
    fn test_manifestation_type_charge() {
        assert_eq!(ManifestationType::Proton.charge_sign(), 1);
        assert_eq!(ManifestationType::Electron.charge_sign(), -1);
        assert_eq!(ManifestationType::Neutron.charge_sign(), 0);
    }

    #[test]
    fn test_heavier_element_manifestation() {
        let pos = Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let carbon = AtomicManifestation::new(ElementAttractorField::carbon(), pos);

        assert_eq!(carbon.atomic_number(), 6);
    }
}
