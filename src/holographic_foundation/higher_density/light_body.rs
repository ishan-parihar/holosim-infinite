//! Light Body Mechanics - 5th Density (Blue Ray)
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "5th Density (Blue Ray) - Wisdom:
//!  - Light-dominant field (maximized information transfer)
//!  - Light body manifestation
//!  - Teaching/learning through field resonance coupling"
//!
//! # Key Insight
//!
//! The Light Body is NOT a metaphorical concept but an actual field configuration
//! where the entity's field becomes light-dominant. This enables:
//! - Direct knowledge transfer (no language needed)
//! - Manifestation through light field manipulation
//! - Teaching through resonance (learning by proximity)

use crate::types::Float;
use std::collections::HashMap;

/// Photon Field - fundamental light-based field configuration
#[derive(Debug, Clone)]
pub struct PhotonField {
    pub frequency: Float,
    pub amplitude: Float,
    pub phase: Float,
    pub coherence: Float,
    pub polarization: Float,
    pub information_density: Float,
}

impl PhotonField {
    pub fn new(frequency: Float) -> Self {
        Self {
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            coherence: 0.5,
            polarization: 0.0,
            information_density: frequency / 100.0,
        }
    }

    pub fn with_amplitude(mut self, amplitude: Float) -> Self {
        self.amplitude = amplitude;
        self
    }

    pub fn update(&mut self, dt: Float) {
        self.phase += self.frequency * dt * 0.01;
        self.phase %= 2.0 * std::f64::consts::PI;

        let coherence_drift = 0.01 * dt * (0.5 - self.coherence).sin();
        self.coherence = (self.coherence + coherence_drift).clamp(0.1, 1.0);
    }

    pub fn energy(&self) -> Float {
        self.amplitude * self.amplitude * self.frequency
    }

    pub fn information_capacity(&self) -> Float {
        self.amplitude * self.frequency * self.coherence
    }

    pub fn resonance_with(&self, other: &PhotonField) -> Float {
        let freq_ratio = self.frequency / other.frequency;
        let harmonic_match = if freq_ratio > 0.9 && freq_ratio < 1.1 {
            1.0 - (freq_ratio - 1.0).abs() * 10.0
        } else if freq_ratio > 1.9 && freq_ratio < 2.1 {
            0.8
        } else if freq_ratio > 0.45 && freq_ratio < 0.55 {
            0.6
        } else {
            0.0
        };

        let phase_alignment = (self.phase - other.phase).cos().abs();
        let coherence_factor = self.coherence * other.coherence;

        harmonic_match * phase_alignment * coherence_factor
    }

    pub fn transfer_information(&mut self, other: &mut PhotonField, amount: Float) -> Float {
        let resonance = self.resonance_with(other);
        let transferable = amount * resonance * self.coherence;

        if self.information_density >= transferable {
            self.information_density -= transferable;
            other.information_density += transferable;
            transferable
        } else {
            0.0
        }
    }
}

/// Energy Body Field - the transitional form between physical and light body
#[derive(Debug, Clone)]
pub struct EnergyBodyField {
    pub entity_id: u64,
    pub base_frequency: Float,
    pub harmonics: Vec<Float>,
    pub luminosity: Float,
    pub density: Float,
    pub chakra_activation: [Float; 7],
    pub kundalini_factor: Float,
}

impl EnergyBodyField {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            base_frequency: 470.0,
            harmonics: vec![1.0, 0.5, 0.25, 0.125],
            luminosity: 0.1,
            density: 0.3,
            chakra_activation: [0.5; 7],
            kundalini_factor: 0.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        let harmonic_sum: Float = self.harmonics.iter().sum();
        self.luminosity = 0.1 + 0.1 * harmonic_sum;

        let chakra_sum: Float = self.chakra_activation.iter().sum();
        self.density = 0.3 + 0.1 * (chakra_sum / 7.0);

        for harmonic in &mut self.harmonics {
            *harmonic = (*harmonic * (1.0 - 0.001 * dt)).max(0.1);
        }

        if self.kundalini_factor > 0.0 {
            for activation in &mut self.chakra_activation {
                *activation = (*activation + 0.01 * self.kundalini_factor * dt).min(1.0);
            }
            self.kundalini_factor *= 0.99;
        }
    }

    pub fn activate_chakra(&mut self, chakra_index: usize, intensity: Float) {
        if chakra_index < 7 {
            self.chakra_activation[chakra_index] =
                (self.chakra_activation[chakra_index] + intensity).min(1.0);
        }
    }

    pub fn raise_kundalini(&mut self, amount: Float) {
        self.kundalini_factor = (self.kundalini_factor + amount).min(1.0);
    }

    pub fn light_body_readiness(&self) -> Float {
        let chakra_alignment: Float = 1.0
            - self
                .chakra_activation
                .windows(2)
                .map(|w| (w[0] - w[1]).abs())
                .sum::<Float>()
                / 6.0;

        let chakra_average: Float = self.chakra_activation.iter().sum::<Float>() / 7.0;
        let harmonic_strength: Float = self.harmonics.iter().sum();

        chakra_alignment * 0.3
            + chakra_average * 0.4
            + harmonic_strength * 0.1
            + self.luminosity * 0.2
    }

    pub fn total_energy(&self) -> Float {
        self.base_frequency * self.luminosity * self.density
    }
}

/// Light Body State - stages of light body development
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightBodyState {
    Dormant,
    Awakening,
    Activating,
    Manifesting,
    Stable,
    Transcending,
}

impl LightBodyState {
    pub fn progression(&self) -> Float {
        match self {
            LightBodyState::Dormant => 0.0,
            LightBodyState::Awakening => 0.2,
            LightBodyState::Activating => 0.4,
            LightBodyState::Manifesting => 0.6,
            LightBodyState::Stable => 0.8,
            LightBodyState::Transcending => 1.0,
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            LightBodyState::Dormant => Some(LightBodyState::Awakening),
            LightBodyState::Awakening => Some(LightBodyState::Activating),
            LightBodyState::Activating => Some(LightBodyState::Manifesting),
            LightBodyState::Manifesting => Some(LightBodyState::Stable),
            LightBodyState::Stable => Some(LightBodyState::Transcending),
            LightBodyState::Transcending => None,
        }
    }
}

/// Light Body Manifestation - the actual light-based vehicle
#[derive(Debug, Clone)]
pub struct LightBodyManifestation {
    pub entity_id: u64,
    pub state: LightBodyState,
    pub photon_field: PhotonField,
    pub energy_body: EnergyBodyField,
    pub manifestation_strength: Float,
    pub teaching_capacity: Float,
    pub learning_capacity: Float,
    pub resonance_field_radius: Float,
}

impl LightBodyManifestation {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            state: LightBodyState::Dormant,
            photon_field: PhotonField::new(470.0),
            energy_body: EnergyBodyField::new(entity_id),
            manifestation_strength: 0.0,
            teaching_capacity: 0.0,
            learning_capacity: 0.5,
            resonance_field_radius: 1.0,
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.photon_field.update(dt);
        self.energy_body.update(dt);

        let readiness = self.energy_body.light_body_readiness();
        self.teaching_capacity = self.photon_field.coherence * readiness;
        self.learning_capacity = 0.5 + 0.5 * readiness;

        self.resonance_field_radius =
            1.0 + 9.0 * self.photon_field.amplitude * self.photon_field.coherence;

        let threshold = match self.state {
            LightBodyState::Dormant => 0.1,
            LightBodyState::Awakening => 0.3,
            LightBodyState::Activating => 0.5,
            LightBodyState::Manifesting => 0.7,
            LightBodyState::Stable => 0.9,
            LightBodyState::Transcending => 1.0,
        };

        if readiness >= threshold {
            if let Some(next) = self.state.next() {
                self.state = next;
            }
        }

        self.manifestation_strength = readiness * self.state.progression();
    }

    pub fn teach(&self, student: &mut LightBodyManifestation, knowledge_amount: Float) -> Float {
        let resonance = self.photon_field.resonance_with(&student.photon_field);
        let transfer_amount = knowledge_amount * self.teaching_capacity * resonance;

        student.photon_field.information_density += transfer_amount;
        transfer_amount
    }

    pub fn learn(&mut self, teacher: &LightBodyManifestation, knowledge_amount: Float) -> Float {
        let resonance = self.photon_field.resonance_with(&teacher.photon_field);
        let absorption = knowledge_amount * self.learning_capacity * resonance;

        self.photon_field.information_density += absorption;
        self.photon_field.coherence = (self.photon_field.coherence + absorption * 0.1).min(1.0);

        absorption
    }

    pub fn can_manifest(&self) -> bool {
        self.state == LightBodyState::Stable || self.state == LightBodyState::Transcending
    }

    pub fn activate(&mut self) -> bool {
        if self.energy_body.light_body_readiness() >= 0.3 {
            self.state = LightBodyState::Awakening;
            true
        } else {
            false
        }
    }
}

/// Light Body - main 5th density entity representation
#[derive(Debug, Clone)]
pub struct LightBody {
    pub entity_id: u64,
    pub manifestation: LightBodyManifestation,
    pub wisdom_accumulated: Float,
    pub teaching_log: Vec<(u64, Float, Float)>,
    pub learning_log: Vec<(u64, Float, Float)>,
    pub resonance_connections: HashMap<u64, Float>,
}

impl LightBody {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            manifestation: LightBodyManifestation::new(entity_id),
            wisdom_accumulated: 0.0,
            teaching_log: Vec::new(),
            learning_log: Vec::new(),
            resonance_connections: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: Float) {
        self.manifestation.update(dt);

        let info_growth = self.manifestation.photon_field.information_density * 0.001 * dt;
        self.wisdom_accumulated += info_growth;

        self.resonance_connections
            .retain(|_, strength| *strength > 0.01);
        for strength in self.resonance_connections.values_mut() {
            *strength = (*strength * (1.0 - 0.01 * dt)).max(0.0);
        }
    }

    pub fn teach_entity(
        &mut self,
        student_id: u64,
        student: &mut LightBody,
        amount: Float,
    ) -> Float {
        let transferred = self.manifestation.teach(&mut student.manifestation, amount);

        if transferred > 0.0 {
            self.teaching_log.push((student_id, transferred, 0.0));
            student
                .learning_log
                .push((self.entity_id, transferred, 0.0));

            let resonance = self
                .manifestation
                .photon_field
                .resonance_with(&student.manifestation.photon_field);
            self.resonance_connections.insert(student_id, resonance);
            student
                .resonance_connections
                .insert(self.entity_id, resonance);
        }

        transferred
    }

    pub fn learn_from(&mut self, teacher_id: u64, teacher: &LightBody, amount: Float) -> Float {
        let absorbed = self.manifestation.learn(&teacher.manifestation, amount);

        if absorbed > 0.0 {
            self.learning_log.push((teacher_id, absorbed, 0.0));

            let resonance = self
                .manifestation
                .photon_field
                .resonance_with(&teacher.manifestation.photon_field);
            self.resonance_connections.insert(teacher_id, resonance);
        }

        absorbed
    }

    pub fn establish_resonance(&mut self, other_id: u64, other: &LightBody) {
        let resonance = self
            .manifestation
            .photon_field
            .resonance_with(&other.manifestation.photon_field);
        self.resonance_connections.insert(other_id, resonance);
    }

    pub fn total_connections(&self) -> usize {
        self.resonance_connections.len()
    }

    pub fn average_resonance(&self) -> Float {
        if self.resonance_connections.is_empty() {
            return 0.0;
        }
        self.resonance_connections.values().sum::<Float>()
            / self.resonance_connections.len() as Float
    }

    pub fn wisdom_level(&self) -> Float {
        self.wisdom_accumulated.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photon_field_creation() {
        let field = PhotonField::new(500.0);
        assert!((field.frequency - 500.0).abs() < 0.001);
    }

    #[test]
    fn test_photon_field_update() {
        let mut field = PhotonField::new(500.0);
        field.update(1.0);
        assert!(field.phase > 0.0);
    }

    #[test]
    fn test_photon_field_energy() {
        let field = PhotonField::new(500.0).with_amplitude(2.0);
        let energy = field.energy();
        assert!(energy > 0.0);
    }

    #[test]
    fn test_photon_field_resonance() {
        let field1 = PhotonField::new(500.0);
        let field2 = PhotonField::new(500.0);
        let resonance = field1.resonance_with(&field2);
        assert!(resonance > 0.0);
    }

    #[test]
    fn test_energy_body_creation() {
        let body = EnergyBodyField::new(1);
        assert_eq!(body.entity_id, 1);
        assert_eq!(body.chakra_activation.len(), 7);
    }

    #[test]
    fn test_energy_body_update() {
        let mut body = EnergyBodyField::new(1);
        body.update(1.0);
        assert!(body.luminosity > 0.0);
    }

    #[test]
    fn test_chakra_activation() {
        let mut body = EnergyBodyField::new(1);
        body.activate_chakra(0, 0.5);
        assert!(body.chakra_activation[0] > 0.5);
    }

    #[test]
    fn test_kundalini_rise() {
        let mut body = EnergyBodyField::new(1);
        body.raise_kundalini(0.5);
        assert!(body.kundalini_factor > 0.0);
    }

    #[test]
    fn test_light_body_readiness() {
        let body = EnergyBodyField::new(1);
        let readiness = body.light_body_readiness();
        assert!((0.0..=1.0).contains(&readiness));
    }

    #[test]
    fn test_light_body_state_progression() {
        assert_eq!(LightBodyState::Dormant.progression(), 0.0);
        assert_eq!(LightBodyState::Stable.progression(), 0.8);
    }

    #[test]
    fn test_light_body_state_next() {
        assert_eq!(
            LightBodyState::Dormant.next(),
            Some(LightBodyState::Awakening)
        );
        assert_eq!(LightBodyState::Transcending.next(), None);
    }

    #[test]
    fn test_light_body_manifestation_creation() {
        let manifestation = LightBodyManifestation::new(1);
        assert_eq!(manifestation.entity_id, 1);
        assert_eq!(manifestation.state, LightBodyState::Dormant);
    }

    #[test]
    fn test_light_body_manifestation_update() {
        let mut manifestation = LightBodyManifestation::new(1);
        manifestation.update(1.0);
        assert!(manifestation.resonance_field_radius > 0.0);
    }

    #[test]
    fn test_light_body_creation() {
        let body = LightBody::new(1);
        assert_eq!(body.entity_id, 1);
    }

    #[test]
    fn test_light_body_update() {
        let mut body = LightBody::new(1);
        body.update(1.0);
        assert!(body.wisdom_accumulated >= 0.0);
    }

    #[test]
    fn test_teaching_learning() {
        let mut teacher = LightBody::new(1);
        let mut student = LightBody::new(2);

        teacher.manifestation.state = LightBodyState::Stable;
        teacher.manifestation.photon_field.coherence = 0.9;

        let transferred = teacher.teach_entity(2, &mut student, 1.0);
        assert!(transferred >= 0.0);
    }

    #[test]
    fn test_resonance_connections() {
        let mut body1 = LightBody::new(1);
        let body2 = LightBody::new(2);

        body1.establish_resonance(2, &body2);
        assert!(body1.resonance_connections.contains_key(&2));
    }
}
