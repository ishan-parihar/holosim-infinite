//! Disease as Field Distortion, Healing as Field Realignment
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Disease as field distortion - disease is NOT just pathogen invasion,
//!  but field configuration disruption. Healing is field realignment."
//!
//! # Key Insight
//!
//! Disease and healing are field phenomena:
//! - Disease = disruption of coherent field patterns
//! - Healing = restoration of coherent field configuration
//! - Immune response = field defense mechanism

use crate::holographic_foundation::archetype_profile::NUM_ARCHETYPES;
use crate::holographic_foundation::organism_physiology::organ_field::{OrganId, OrganState};
use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiseaseType {
    Infection,
    Inflammation,
    Degeneration,
    Autoimmune,
    Neoplasm,
    Metabolic,
    Toxic,
    Traumatic,
    Genetic,
    Psychosomatic,
}

impl DiseaseType {
    pub fn archetype_disruption(&self) -> [Float; NUM_ARCHETYPES] {
        let mut disruption = [0.0; NUM_ARCHETYPES];

        match self {
            DiseaseType::Infection => {
                disruption[5] = 0.3;
                disruption[11] = 0.25;
                disruption[0] = 0.1;
            }
            DiseaseType::Inflammation => {
                disruption[2] = 0.3;
                disruption[3] = 0.2;
                disruption[5] = 0.15;
            }
            DiseaseType::Degeneration => {
                disruption[8] = 0.25;
                disruption[9] = 0.2;
                disruption[10] = 0.15;
            }
            DiseaseType::Autoimmune => {
                disruption[5] = 0.4;
                disruption[11] = 0.3;
                disruption[0] = 0.2;
            }
            DiseaseType::Neoplasm => {
                disruption[0] = 0.2;
                disruption[7] = 0.3;
                disruption[21] = 0.25;
            }
            DiseaseType::Metabolic => {
                disruption[3] = 0.25;
                disruption[4] = 0.2;
                disruption[10] = 0.15;
            }
            DiseaseType::Toxic => {
                disruption[3] = 0.3;
                disruption[10] = 0.25;
                disruption[11] = 0.2;
            }
            DiseaseType::Traumatic => {
                disruption[8] = 0.3;
                disruption[9] = 0.25;
                disruption[10] = 0.2;
            }
            DiseaseType::Genetic => {
                disruption[0] = 0.2;
                disruption[14] = 0.25;
                disruption[21] = 0.15;
            }
            DiseaseType::Psychosomatic => {
                disruption[0] = 0.3;
                disruption[1] = 0.25;
                disruption[14] = 0.2;
            }
        }

        disruption
    }

    pub fn severity_range(&self) -> (Float, Float) {
        match self {
            DiseaseType::Infection => (0.1, 0.9),
            DiseaseType::Inflammation => (0.1, 0.7),
            DiseaseType::Degeneration => (0.2, 0.8),
            DiseaseType::Autoimmune => (0.3, 0.9),
            DiseaseType::Neoplasm => (0.4, 1.0),
            DiseaseType::Metabolic => (0.1, 0.6),
            DiseaseType::Toxic => (0.2, 0.8),
            DiseaseType::Traumatic => (0.1, 0.9),
            DiseaseType::Genetic => (0.2, 0.7),
            DiseaseType::Psychosomatic => (0.1, 0.5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldDistortion {
    disease_type: DiseaseType,
    severity: Float,
    affected_organs: Vec<OrganId>,
    archetype_disruption: [Float; NUM_ARCHETYPES],
    coherence_impact: Float,
    duration: Float,
}

impl FieldDistortion {
    pub fn new(disease_type: DiseaseType, severity: Float) -> Self {
        let archetype_disruption = disease_type.archetype_disruption();
        let coherence_impact = severity * 0.5;

        Self {
            disease_type,
            severity,
            affected_organs: Vec::new(),
            archetype_disruption,
            coherence_impact,
            duration: 0.0,
        }
    }

    pub fn with_affected_organs(mut self, organs: Vec<OrganId>) -> Self {
        self.affected_organs = organs;
        self
    }

    pub fn disease_type(&self) -> DiseaseType {
        self.disease_type
    }

    pub fn severity(&self) -> Float {
        self.severity
    }

    pub fn archetype_disruption(&self) -> &[Float; NUM_ARCHETYPES] {
        &self.archetype_disruption
    }

    pub fn coherence_impact(&self) -> Float {
        self.coherence_impact
    }

    pub fn duration(&self) -> Float {
        self.duration
    }

    pub fn affects_organ(&self, organ_id: &OrganId) -> bool {
        self.affected_organs.contains(organ_id)
    }

    pub fn update(&mut self, dt: Float) {
        self.duration += dt;
    }

    pub fn progress(&mut self, amount: Float) {
        self.severity = (self.severity + amount).min(1.0);
        self.coherence_impact = self.severity * 0.5;
    }

    pub fn regress(&mut self, amount: Float) {
        self.severity = (self.severity - amount).max(0.0);
        self.coherence_impact = self.severity * 0.5;
    }

    pub fn is_resolved(&self) -> bool {
        self.severity < 0.05
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiseaseState {
    Incubating,
    Acute,
    Chronic,
    Resolving,
    Resolved,
}

impl DiseaseState {
    pub fn from_severity_and_duration(severity: Float, duration: Float) -> Self {
        if severity < 0.05 {
            DiseaseState::Resolved
        } else if duration < 1.0 {
            DiseaseState::Incubating
        } else if duration < 10.0 {
            DiseaseState::Acute
        } else if severity < 0.3 {
            DiseaseState::Resolving
        } else {
            DiseaseState::Chronic
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealingMechanism {
    ImmuneResponse,
    Regeneration,
    Adaptation,
    FieldRealignment,
    ExternalIntervention,
    SpontaneousRemission,
}

impl HealingMechanism {
    pub fn healing_rate(&self) -> Float {
        match self {
            HealingMechanism::ImmuneResponse => 0.05,
            HealingMechanism::Regeneration => 0.02,
            HealingMechanism::Adaptation => 0.01,
            HealingMechanism::FieldRealignment => 0.08,
            HealingMechanism::ExternalIntervention => 0.1,
            HealingMechanism::SpontaneousRemission => 0.15,
        }
    }

    pub fn archetype_support(&self) -> [Float; NUM_ARCHETYPES] {
        let mut support = [0.5; NUM_ARCHETYPES];

        match self {
            HealingMechanism::ImmuneResponse => {
                support[5] = 0.9;
                support[11] = 0.8;
            }
            HealingMechanism::Regeneration => {
                support[6] = 0.9;
                support[7] = 0.7;
            }
            HealingMechanism::Adaptation => {
                support[0] = 0.8;
                support[1] = 0.7;
            }
            HealingMechanism::FieldRealignment => {
                support[14] = 0.9;
                support[15] = 0.8;
            }
            HealingMechanism::ExternalIntervention => {
                support[2] = 0.85;
                support[3] = 0.75;
            }
            HealingMechanism::SpontaneousRemission => {
                support[21] = 0.95;
            }
        }

        support
    }
}

#[derive(Debug, Clone)]
pub struct HealingResult {
    pub mechanism: HealingMechanism,
    pub healing_applied: Float,
    pub new_severity: Float,
    pub coherence_restored: Float,
    pub side_effects: Vec<String>,
}

impl HealingResult {
    pub fn new(mechanism: HealingMechanism, healing_applied: Float, new_severity: Float) -> Self {
        Self {
            mechanism,
            healing_applied,
            new_severity,
            coherence_restored: healing_applied * 0.3,
            side_effects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealingSystem {
    active_distortions: Vec<FieldDistortion>,
    healing_mechanisms: Vec<HealingMechanism>,
    base_healing_rate: Float,
    immune_strength: Float,
    regeneration_capacity: Float,
}

impl HealingSystem {
    pub fn new() -> Self {
        Self {
            active_distortions: Vec::new(),
            healing_mechanisms: vec![
                HealingMechanism::ImmuneResponse,
                HealingMechanism::Regeneration,
                HealingMechanism::FieldRealignment,
            ],
            base_healing_rate: 0.01,
            immune_strength: 0.85,
            regeneration_capacity: 0.7,
        }
    }

    pub fn add_distortion(&mut self, distortion: FieldDistortion) {
        self.active_distortions.push(distortion);
    }

    pub fn remove_resolved(&mut self) {
        self.active_distortions.retain(|d| !d.is_resolved());
    }

    pub fn active_disease_count(&self) -> usize {
        self.active_distortions.len()
    }

    pub fn total_severity(&self) -> Float {
        self.active_distortions.iter().map(|d| d.severity()).sum()
    }

    pub fn total_coherence_impact(&self) -> Float {
        self.active_distortions
            .iter()
            .map(|d| d.coherence_impact())
            .sum()
    }

    pub fn apply_healing(
        &mut self,
        distortion_idx: usize,
        mechanism: HealingMechanism,
    ) -> Option<HealingResult> {
        if distortion_idx >= self.active_distortions.len() {
            return None;
        }

        let distortion = &mut self.active_distortions[distortion_idx];
        let healing_rate = mechanism.healing_rate();

        let adjusted_rate = match mechanism {
            HealingMechanism::ImmuneResponse => healing_rate * self.immune_strength,
            HealingMechanism::Regeneration => healing_rate * self.regeneration_capacity,
            HealingMechanism::FieldRealignment => {
                healing_rate * (self.immune_strength + self.regeneration_capacity) / 2.0
            }
            _ => healing_rate,
        };

        let old_severity = distortion.severity();
        distortion.regress(adjusted_rate);
        let new_severity = distortion.severity();
        let healing_applied = old_severity - new_severity;

        Some(HealingResult::new(mechanism, healing_applied, new_severity))
    }

    pub fn update(&mut self, dt: Float) {
        for distortion in &mut self.active_distortions {
            distortion.update(dt);
        }

        let mechanisms: Vec<HealingMechanism> = self
            .active_distortions
            .iter()
            .map(|d| self.select_healing_mechanism(d))
            .collect();

        for (i, distortion) in self.active_distortions.iter_mut().enumerate() {
            let mechanism = mechanisms[i];
            let rate = mechanism.healing_rate() * dt;

            if matches!(mechanism, HealingMechanism::ImmuneResponse) {
                distortion.regress(rate * self.immune_strength);
            } else if matches!(mechanism, HealingMechanism::Regeneration) {
                distortion.regress(rate * self.regeneration_capacity);
            } else {
                distortion.regress(rate);
            }
        }

        self.remove_resolved();
    }

    fn select_healing_mechanism(&self, distortion: &FieldDistortion) -> HealingMechanism {
        match distortion.disease_type() {
            DiseaseType::Infection => HealingMechanism::ImmuneResponse,
            DiseaseType::Traumatic | DiseaseType::Degeneration => HealingMechanism::Regeneration,
            DiseaseType::Autoimmune | DiseaseType::Psychosomatic => {
                HealingMechanism::FieldRealignment
            }
            DiseaseType::Toxic => HealingMechanism::ExternalIntervention,
            DiseaseType::Neoplasm => {
                if distortion.severity() < 0.3 {
                    HealingMechanism::ImmuneResponse
                } else {
                    HealingMechanism::ExternalIntervention
                }
            }
            _ => HealingMechanism::Adaptation,
        }
    }

    pub fn distortions(&self) -> &[FieldDistortion] {
        &self.active_distortions
    }
}

impl Default for HealingSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disease_type_archetype_disruption() {
        let disruption = DiseaseType::Infection.archetype_disruption();
        assert!(disruption[5] > 0.0);
    }

    #[test]
    fn test_disease_type_severity_range() {
        let (min, max) = DiseaseType::Infection.severity_range();
        assert!(min < max);
    }

    #[test]
    fn test_field_distortion_creation() {
        let distortion = FieldDistortion::new(DiseaseType::Infection, 0.5);
        assert_eq!(distortion.severity(), 0.5);
        assert_eq!(distortion.disease_type(), DiseaseType::Infection);
    }

    #[test]
    fn test_field_distortion_progress() {
        let mut distortion = FieldDistortion::new(DiseaseType::Infection, 0.3);
        distortion.progress(0.2);
        assert!(distortion.severity() > 0.3);
    }

    #[test]
    fn test_field_distortion_regress() {
        let mut distortion = FieldDistortion::new(DiseaseType::Infection, 0.5);
        distortion.regress(0.2);
        assert!(distortion.severity() < 0.5);
    }

    #[test]
    fn test_field_distortion_resolved() {
        let mut distortion = FieldDistortion::new(DiseaseType::Infection, 0.03);
        assert!(distortion.is_resolved());
    }

    #[test]
    fn test_disease_state_from_severity() {
        assert_eq!(
            DiseaseState::from_severity_and_duration(0.03, 5.0),
            DiseaseState::Resolved
        );
        assert_eq!(
            DiseaseState::from_severity_and_duration(0.5, 0.5),
            DiseaseState::Incubating
        );
    }

    #[test]
    fn test_healing_mechanism_rate() {
        assert!(
            HealingMechanism::SpontaneousRemission.healing_rate()
                > HealingMechanism::Adaptation.healing_rate()
        );
    }

    #[test]
    fn test_healing_system_creation() {
        let system = HealingSystem::new();
        assert_eq!(system.active_disease_count(), 0);
    }

    #[test]
    fn test_healing_system_add_distortion() {
        let mut system = HealingSystem::new();
        system.add_distortion(FieldDistortion::new(DiseaseType::Infection, 0.5));
        assert_eq!(system.active_disease_count(), 1);
    }

    #[test]
    fn test_healing_system_apply_healing() {
        let mut system = HealingSystem::new();
        system.add_distortion(FieldDistortion::new(DiseaseType::Infection, 0.5));
        let result = system.apply_healing(0, HealingMechanism::ImmuneResponse);
        assert!(result.is_some());
    }

    #[test]
    fn test_healing_result_creation() {
        let result = HealingResult::new(HealingMechanism::ImmuneResponse, 0.1, 0.4);
        assert_eq!(result.healing_applied, 0.1);
    }

    #[test]
    fn test_healing_system_update() {
        let mut system = HealingSystem::new();
        system.add_distortion(FieldDistortion::new(DiseaseType::Infection, 0.5));
        system.update(1.0);
        assert!(system.total_severity() < 0.5);
    }
}
