//! Density Progression - Continuous evolution instead of discrete jumps
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Continuous density progression (not state jumps)
//!  Evolution progress is continuous and visible"
//!
//! KEY INSIGHT: Entities evolve smoothly through density bands
//! rather than making discrete transitions.

use super::decision_feedback::{DecisionType, EntityDecision};
use super::{EvolutionFeedbackConfig, MIN_SIGNIFICANCE_THRESHOLD};
use crate::holographic_foundation::spectrum::DensityPosition;
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SpectrumShift {
    pub density_delta: Float,
    pub spectrum_position_delta: Float,
    pub catalyst_absorbed: Float,
    pub time_elapsed: Float,
}

impl SpectrumShift {
    pub fn new(density: Float, spectrum: Float, catalyst: Float, time: Float) -> Self {
        Self {
            density_delta: density,
            spectrum_position_delta: spectrum,
            catalyst_absorbed: catalyst,
            time_elapsed: time,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn is_significant(&self) -> bool {
        self.density_delta.abs() > MIN_SIGNIFICANCE_THRESHOLD
            || self.spectrum_position_delta.abs() > MIN_SIGNIFICANCE_THRESHOLD
    }

    pub fn combine(&self, other: &SpectrumShift) -> Self {
        Self::new(
            self.density_delta + other.density_delta,
            self.spectrum_position_delta + other.spectrum_position_delta,
            self.catalyst_absorbed + other.catalyst_absorbed,
            self.time_elapsed + other.time_elapsed,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ProgressionEvent {
    pub entity_id: u64,
    pub from_density: Float,
    pub to_density: Float,
    pub catalyst_type: String,
    pub significance: Float,
    pub time: Float,
}

impl ProgressionEvent {
    pub fn new(
        entity_id: u64,
        from: Float,
        to: Float,
        catalyst: &str,
        significance: Float,
        time: Float,
    ) -> Self {
        Self {
            entity_id,
            from_density: from,
            to_density: to,
            catalyst_type: catalyst.to_string(),
            significance,
            time,
        }
    }

    pub fn density_change(&self) -> Float {
        self.to_density - self.from_density
    }

    pub fn is_advancement(&self) -> bool {
        self.to_density > self.from_density
    }
}

pub struct DensityProgression {
    entity_densities: HashMap<u64, Float>,
    entity_rates: HashMap<u64, Float>,
    progression_events: Vec<ProgressionEvent>,
    total_progression: Float,
    #[allow(dead_code)]
    config: EvolutionFeedbackConfig,
}

impl DensityProgression {
    pub fn new(config: EvolutionFeedbackConfig) -> Self {
        Self {
            entity_densities: HashMap::new(),
            entity_rates: HashMap::new(),
            progression_events: Vec::new(),
            total_progression: 0.0,
            config,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(EvolutionFeedbackConfig::default())
    }

    pub fn register_entity(&mut self, entity_id: u64, initial_density: Float) {
        self.entity_densities.insert(entity_id, initial_density);
        self.entity_rates
            .insert(entity_id, Self::default_rate_for_density(initial_density));
    }

    fn default_rate_for_density(density: Float) -> Float {
        match density.floor() as usize {
            0 | 1 => 0.0001,
            2 => 0.0002,
            3 => 0.0005,
            4 => 0.001,
            5 => 0.002,
            6 => 0.004,
            7 => 0.008,
            _ => 0.01,
        }
    }

    pub fn get_density(&self, entity_id: u64) -> Float {
        self.entity_densities
            .get(&entity_id)
            .copied()
            .unwrap_or(1.0)
    }

    pub fn get_density_position(&self, entity_id: u64) -> DensityPosition {
        DensityPosition::new(self.get_density(entity_id))
    }

    pub fn get_rate(&self, entity_id: u64) -> Float {
        self.entity_rates.get(&entity_id).copied().unwrap_or(0.001)
    }

    pub fn update_from_decisions(&mut self, decisions: &[EntityDecision], time: Float) {
        let mut accumulated: HashMap<u64, (Float, Vec<&EntityDecision>)> = HashMap::new();

        for decision in decisions {
            if self.entity_densities.contains_key(&decision.entity_id) {
                let entry = accumulated
                    .entry(decision.entity_id)
                    .or_insert((0.0, Vec::new()));
                let rate_change =
                    self.rate_change_for_decision(&decision.decision_type, decision.significance);
                entry.0 += rate_change * decision.significance;
                entry.1.push(decision);
            }
        }

        for (entity_id, (total_change, decs)) in accumulated {
            if let Some(&current_density) = self.entity_densities.get(&entity_id) {
                if let Some(rate) = self.entity_rates.get_mut(&entity_id) {
                    *rate = (*rate + total_change * 0.001).clamp(0.0001, 0.05);
                }

                let density_change = total_change * 0.1;
                let new_density = (current_density + density_change).clamp(0.0, 8.0);

                if (new_density - current_density).abs() > MIN_SIGNIFICANCE_THRESHOLD {
                    if let Some(first_decision) = decs.first() {
                        let event = ProgressionEvent::new(
                            entity_id,
                            current_density,
                            new_density,
                            first_decision.decision_type.display_name(),
                            first_decision.significance,
                            time,
                        );
                        self.progression_events.push(event);
                        self.total_progression += density_change.abs();
                    }
                }

                self.entity_densities.insert(entity_id, new_density);
            }
        }
    }

    fn rate_change_for_decision(&self, decision_type: &DecisionType, significance: Float) -> Float {
        let base_change = match decision_type {
            DecisionType::Growth => 0.001,
            DecisionType::Service => 0.0008,
            DecisionType::Learning => 0.0006,
            DecisionType::Connection => 0.0005,
            DecisionType::Control => -0.0002,
            DecisionType::Isolation => -0.0003,
            DecisionType::Expression => 0.0003,
            DecisionType::Reception => 0.0004,
        };
        base_change * significance
    }

    pub fn evolve_all(&mut self, dt: Float) {
        for (entity_id, rate) in self.entity_rates.iter() {
            if let Some(density) = self.entity_densities.get_mut(entity_id) {
                let new_density = *density + rate * dt;
                *density = new_density.clamp(0.0, 8.0);
            }
        }
    }

    pub fn calculate_spectrum_shift(&self, entity_id: u64, dt: Float) -> SpectrumShift {
        let _density = self.get_density(entity_id);
        let rate = self.get_rate(entity_id);

        SpectrumShift::new(rate * dt, -rate * dt * 0.1, 0.0, dt)
    }

    pub fn get_events(&self) -> &[ProgressionEvent] {
        &self.progression_events
    }

    pub fn get_recent_events(&self, count: usize) -> Vec<&ProgressionEvent> {
        self.progression_events.iter().rev().take(count).collect()
    }

    pub fn events_for_entity(&self, entity_id: u64) -> Vec<&ProgressionEvent> {
        self.progression_events
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .collect()
    }

    pub fn total_progression(&self) -> Float {
        self.total_progression
    }

    pub fn entity_count(&self) -> usize {
        self.entity_densities.len()
    }

    pub fn average_density(&self) -> Float {
        if self.entity_densities.is_empty() {
            return 0.0;
        }
        self.entity_densities.values().sum::<Float>() / self.entity_densities.len() as Float
    }

    pub fn clear_events(&mut self) {
        self.progression_events.clear();
    }

    pub fn remove_entity(&mut self, entity_id: u64) {
        self.entity_densities.remove(&entity_id);
        self.entity_rates.remove(&entity_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_progression_creation() {
        let progression = DensityProgression::with_defaults();
        assert_eq!(progression.entity_count(), 0);
    }

    #[test]
    fn test_entity_registration() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 3.0);

        assert_eq!(progression.get_density(1), 3.0);
        assert!(progression.get_rate(1) > 0.0);
    }

    #[test]
    fn test_density_position_retrieval() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 3.5);

        let pos = progression.get_density_position(1);
        assert_eq!(pos.value, 3.5);
        assert_eq!(pos.primary_density(), 3);
    }

    #[test]
    fn test_update_from_decisions() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 3.0);

        let decisions: Vec<EntityDecision> = (0..200)
            .map(|i| EntityDecision::new(1, DecisionType::Growth, 1.0, [i as Float, 0.0, 0.0]))
            .collect();
        progression.update_from_decisions(&decisions, 0.0);

        let new_density = progression.get_density(1);
        assert!(
            new_density > 3.0,
            "Density should increase from 3.0, got {}",
            new_density
        );
        assert!(
            !progression.get_events().is_empty(),
            "Events should not be empty"
        );
    }

    #[test]
    fn test_evolve_all() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 3.0);
        progression.register_entity(2, 4.0);

        let initial_d1 = progression.get_density(1);
        progression.evolve_all(100.0);
        let final_d1 = progression.get_density(1);

        assert!(final_d1 > initial_d1);
    }

    #[test]
    fn test_spectrum_shift_calculation() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 3.0);

        let shift = progression.calculate_spectrum_shift(1, 1.0);

        assert!(shift.density_delta > 0.0);
        assert!(shift.spectrum_position_delta < 0.0);
    }

    #[test]
    fn test_average_density() {
        let mut progression = DensityProgression::with_defaults();
        progression.register_entity(1, 2.0);
        progression.register_entity(2, 4.0);

        assert_eq!(progression.average_density(), 3.0);
    }

    #[test]
    fn test_progression_event_creation() {
        let event = ProgressionEvent::new(1, 3.0, 3.5, "Growth", 0.8, 10.0);

        assert_eq!(event.density_change(), 0.5);
        assert!(event.is_advancement());
    }

    #[test]
    fn test_rate_change_by_decision_type() {
        let progression = DensityProgression::with_defaults();

        let growth_change = progression.rate_change_for_decision(&DecisionType::Growth, 1.0);
        let control_change = progression.rate_change_for_decision(&DecisionType::Control, 1.0);

        assert!(growth_change > 0.0);
        assert!(control_change < 0.0);
    }
}
