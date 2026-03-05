//! Collective Formation - Formation threshold mechanics and field merging
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Collective formation threshold mechanics
//!  Collective field configuration merging
//!  Emergent collective consciousness"

use super::{
    ConnectionType, ResonanceConnection, DISSOLUTION_THRESHOLD, MAX_COLLECTIVE_SIZE,
    MIN_RESONANCE_THRESHOLD, STABLE_COLLECTIVE_THRESHOLD,
};
use crate::holographic_foundation::distortions::FieldState;
use crate::types::Float;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectiveState {
    Forming,
    Stable,
    Growing,
    Shrinking,
    Dissolving,
}

impl CollectiveState {
    pub fn is_active(&self) -> bool {
        !matches!(self, CollectiveState::Dissolving)
    }
}

#[derive(Debug, Clone)]
pub struct CollectiveConfig {
    pub min_members: usize,
    pub max_members: usize,
    pub formation_threshold: Float,
    pub dissolution_threshold: Float,
    pub resonance_decay_rate: Float,
    pub merge_threshold: Float,
}

impl Default for CollectiveConfig {
    fn default() -> Self {
        Self {
            min_members: 2,
            max_members: MAX_COLLECTIVE_SIZE,
            formation_threshold: STABLE_COLLECTIVE_THRESHOLD,
            dissolution_threshold: DISSOLUTION_THRESHOLD,
            resonance_decay_rate: 0.01,
            merge_threshold: 0.7,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collective {
    pub collective_id: u64,
    pub members: HashSet<u64>,
    pub state: CollectiveState,
    pub field_configuration: FieldState,
    pub average_density: Float,
    pub resonance_level: Float,
    pub coherence: Float,
    pub creation_time: Float,
    pub last_activity: Float,
    pub member_resonances: HashMap<u64, Float>,
    pub dominant_connection_type: ConnectionType,
}

impl Collective {
    pub fn new(collective_id: u64, initial_member: u64, field: FieldState, time: Float) -> Self {
        let mut members = HashSet::new();
        members.insert(initial_member);

        let mut member_resonances = HashMap::new();
        member_resonances.insert(initial_member, 1.0);

        Self {
            collective_id,
            members,
            state: CollectiveState::Forming,
            field_configuration: field,
            average_density: 3.0,
            resonance_level: 1.0,
            coherence: 0.5,
            creation_time: time,
            last_activity: time,
            member_resonances,
            dominant_connection_type: ConnectionType::Resonance,
        }
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn is_stable(&self) -> bool {
        matches!(
            self.state,
            CollectiveState::Stable | CollectiveState::Growing
        )
    }

    pub fn can_accept_member(&self, config: &CollectiveConfig) -> bool {
        self.members.len() < config.max_members && self.state.is_active()
    }

    pub fn add_member(
        &mut self,
        entity_id: u64,
        resonance: Float,
        field: &FieldState,
        time: Float,
        config: &CollectiveConfig,
    ) -> bool {
        if !self.can_accept_member(config) {
            return false;
        }

        self.members.insert(entity_id);
        self.member_resonances.insert(entity_id, resonance);
        self.last_activity = time;

        self.merge_field(field);
        self.update_state(config);

        true
    }

    pub fn remove_member(&mut self, entity_id: u64, config: &CollectiveConfig) -> bool {
        if self.members.remove(&entity_id) {
            self.member_resonances.remove(&entity_id);
            self.update_state(config);
            true
        } else {
            false
        }
    }

    fn merge_field(&mut self, new_field: &FieldState) {
        let weight = 1.0 / (self.members.len() as Float + 1.0);

        for (i, amp) in self
            .field_configuration
            .density_amplitudes
            .iter_mut()
            .enumerate()
        {
            let existing = amp.magnitude();
            let new_val = new_field.density_amplitudes[i].magnitude();
            let merged = existing * (1.0 - weight) + new_val * weight;
            let phase = amp.phase();
            *amp = crate::holographic_foundation::distortions::DensityAmplitude::from_polar(
                merged, phase,
            );
        }

        self.field_configuration.coherence =
            self.field_configuration.coherence * (1.0 - weight) + new_field.coherence * weight;
        self.field_configuration.energy += new_field.energy * weight;
    }

    fn update_state(&mut self, config: &CollectiveConfig) {
        self.state = if self.members.len() < config.min_members
            || self.resonance_level < config.dissolution_threshold
        {
            CollectiveState::Dissolving
        } else if self.members.len() == config.min_members {
            CollectiveState::Forming
        } else if self.resonance_level >= config.formation_threshold {
            CollectiveState::Stable
        } else {
            CollectiveState::Shrinking
        };

        self.coherence = self.calculate_coherence();
    }

    fn calculate_coherence(&self) -> Float {
        if self.member_resonances.is_empty() {
            return 0.0;
        }

        let mean: Float =
            self.member_resonances.values().sum::<Float>() / self.member_resonances.len() as Float;
        let variance: Float = self
            .member_resonances
            .values()
            .map(|r| (r - mean).powi(2))
            .sum::<Float>()
            / self.member_resonances.len() as Float;

        (mean * (1.0 - variance.sqrt())).clamp(0.0, 1.0)
    }

    pub fn update_resonance(&mut self, entity_id: u64, new_resonance: Float) {
        if let Some(resonance) = self.member_resonances.get_mut(&entity_id) {
            *resonance = new_resonance;
        }

        self.resonance_level = self.member_resonances.values().sum::<Float>()
            / self.member_resonances.len().max(1) as Float;
    }

    pub fn decay_resonance(&mut self, rate: Float) {
        for resonance in self.member_resonances.values_mut() {
            *resonance = (*resonance * (1.0 - rate)).max(0.0);
        }

        self.resonance_level = self.member_resonances.values().sum::<Float>()
            / self.member_resonances.len().max(1) as Float;
    }
}

pub struct CollectiveFormation {
    collectives: HashMap<u64, Collective>,
    pending_connections: Vec<ResonanceConnection>,
    config: CollectiveConfig,
    next_collective_id: u64,
    formation_events: Vec<FormationEvent>,
}

#[derive(Debug, Clone)]
pub struct FormationEvent {
    pub collective_id: u64,
    pub event_type: FormationEventType,
    pub entity_id: Option<u64>,
    pub time: Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormationEventType {
    Created,
    MemberJoined,
    MemberLeft,
    Merged,
    Dissolved,
}

impl CollectiveFormation {
    pub fn new(config: CollectiveConfig) -> Self {
        Self {
            collectives: HashMap::new(),
            pending_connections: Vec::new(),
            config,
            next_collective_id: 1,
            formation_events: Vec::new(),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(CollectiveConfig::default())
    }

    pub fn consider_connection(&mut self, connection: ResonanceConnection) {
        if connection.strength >= MIN_RESONANCE_THRESHOLD {
            self.pending_connections.push(connection);
        }
    }

    pub fn process_pending(&mut self, time: Float) -> Vec<u64> {
        let mut new_collectives = Vec::new();
        let connections: Vec<_> = self.pending_connections.drain(..).collect();

        for connection in connections {
            if let Some(collective_id) = self.find_or_create_collective(&connection, time) {
                new_collectives.push(collective_id);
            }
        }

        new_collectives
    }

    fn find_or_create_collective(
        &mut self,
        connection: &ResonanceConnection,
        time: Float,
    ) -> Option<u64> {
        let collective_a = self.find_collective_for_entity(connection.entity_a);
        let collective_b = self.find_collective_for_entity(connection.entity_b);

        match (collective_a, collective_b) {
            (Some(a), Some(b)) if a != b => self.merge_collectives(a, b, time),
            (Some(c), None) => {
                self.add_to_collective(c, connection.entity_b, connection.strength, time)
            }
            (None, Some(c)) => {
                self.add_to_collective(c, connection.entity_a, connection.strength, time)
            }
            (None, None) => Some(self.create_collective(connection, time)),
            _ => None,
        }
    }

    fn find_collective_for_entity(&self, entity_id: u64) -> Option<u64> {
        self.collectives
            .iter()
            .find(|(_, c)| c.members.contains(&entity_id))
            .map(|(id, _)| *id)
    }

    fn create_collective(&mut self, connection: &ResonanceConnection, time: Float) -> u64 {
        let collective_id = self.next_collective_id;
        self.next_collective_id += 1;

        let mut collective =
            Collective::new(collective_id, connection.entity_a, FieldState::new(), time);
        collective.add_member(
            connection.entity_b,
            connection.strength,
            &FieldState::new(),
            time,
            &self.config,
        );
        collective.state = CollectiveState::Forming;

        self.collectives.insert(collective_id, collective);

        self.formation_events.push(FormationEvent {
            collective_id,
            event_type: FormationEventType::Created,
            entity_id: None,
            time,
        });

        collective_id
    }

    fn add_to_collective(
        &mut self,
        collective_id: u64,
        entity_id: u64,
        resonance: Float,
        time: Float,
    ) -> Option<u64> {
        if let Some(collective) = self.collectives.get_mut(&collective_id) {
            if collective.add_member(entity_id, resonance, &FieldState::new(), time, &self.config) {
                self.formation_events.push(FormationEvent {
                    collective_id,
                    event_type: FormationEventType::MemberJoined,
                    entity_id: Some(entity_id),
                    time,
                });
                return Some(collective_id);
            }
        }
        None
    }

    fn merge_collectives(&mut self, a: u64, b: u64, time: Float) -> Option<u64> {
        let (members_b, resonances_b) = {
            let collective_b = self.collectives.get(&b)?;
            (
                collective_b.members.clone(),
                collective_b.member_resonances.clone(),
            )
        };

        let collective_a = self.collectives.get_mut(&a)?;

        for (entity_id, resonance) in members_b.iter().zip(resonances_b.values()) {
            let entity = *members_b.iter().find(|e| *e != entity_id).unwrap_or(&0);
            collective_a.add_member(entity, *resonance, &FieldState::new(), time, &self.config);
        }

        self.collectives.remove(&b);

        self.formation_events.push(FormationEvent {
            collective_id: a,
            event_type: FormationEventType::Merged,
            entity_id: Some(b),
            time,
        });

        Some(a)
    }

    pub fn remove_from_collective(&mut self, entity_id: u64, time: Float) {
        if let Some(collective_id) = self.find_collective_for_entity(entity_id) {
            if let Some(collective) = self.collectives.get_mut(&collective_id) {
                collective.remove_member(entity_id, &self.config);

                self.formation_events.push(FormationEvent {
                    collective_id,
                    event_type: FormationEventType::MemberLeft,
                    entity_id: Some(entity_id),
                    time,
                });

                if collective.members.is_empty() || collective.state == CollectiveState::Dissolving
                {
                    self.collectives.remove(&collective_id);

                    self.formation_events.push(FormationEvent {
                        collective_id,
                        event_type: FormationEventType::Dissolved,
                        entity_id: None,
                        time,
                    });
                }
            }
        }
    }

    pub fn update(&mut self, dt: Float, time: Float) {
        let dissolution_threshold = self.config.dissolution_threshold;
        let decay_rate = self.config.resonance_decay_rate;

        let to_dissolve: Vec<u64> = self
            .collectives
            .iter()
            .filter(|(_, c)| {
                c.resonance_level < dissolution_threshold
                    || c.members.len() < self.config.min_members
            })
            .map(|(id, _)| *id)
            .collect();

        for id in to_dissolve {
            self.collectives.remove(&id);
            self.formation_events.push(FormationEvent {
                collective_id: id,
                event_type: FormationEventType::Dissolved,
                entity_id: None,
                time,
            });
        }

        for collective in self.collectives.values_mut() {
            collective.decay_resonance(decay_rate * dt);
        }
    }

    pub fn get_collective(&self, collective_id: u64) -> Option<&Collective> {
        self.collectives.get(&collective_id)
    }

    pub fn get_collective_for_entity(&self, entity_id: u64) -> Option<&Collective> {
        self.collectives
            .values()
            .find(|c| c.members.contains(&entity_id))
    }

    pub fn collective_count(&self) -> usize {
        self.collectives.len()
    }

    pub fn total_membership(&self) -> usize {
        self.collectives.values().map(|c| c.members.len()).sum()
    }

    pub fn get_formation_events(&self) -> &[FormationEvent] {
        &self.formation_events
    }

    pub fn all_collectives(&self) -> impl Iterator<Item = &Collective> {
        self.collectives.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_state_active() {
        assert!(CollectiveState::Stable.is_active());
        assert!(!CollectiveState::Dissolving.is_active());
    }

    #[test]
    fn test_collective_creation() {
        let collective = Collective::new(1, 100, FieldState::new(), 0.0);

        assert_eq!(collective.member_count(), 1);
        assert_eq!(collective.state, CollectiveState::Forming);
    }

    #[test]
    fn test_collective_add_member() {
        let config = CollectiveConfig::default();
        let mut collective = Collective::new(1, 100, FieldState::new(), 0.0);

        let added = collective.add_member(101, 0.8, &FieldState::new(), 0.0, &config);

        assert!(added);
        assert_eq!(collective.member_count(), 2);
    }

    #[test]
    fn test_collective_remove_member() {
        let config = CollectiveConfig::default();
        let mut collective = Collective::new(1, 100, FieldState::new(), 0.0);
        collective.add_member(101, 0.8, &FieldState::new(), 0.0, &config);

        let removed = collective.remove_member(101, &config);

        assert!(removed);
        assert_eq!(collective.member_count(), 1);
    }

    #[test]
    fn test_formation_creation() {
        let formation = CollectiveFormation::with_defaults();
        assert_eq!(formation.collective_count(), 0);
    }

    #[test]
    fn test_consider_connection() {
        let mut formation = CollectiveFormation::with_defaults();
        let conn = ResonanceConnection::new(1, 2, ConnectionType::Resonance, 0.7, 0.0, 0.0);

        formation.consider_connection(conn);

        assert!(!formation.pending_connections.is_empty());
    }

    #[test]
    fn test_process_pending_creates_collective() {
        let mut formation = CollectiveFormation::with_defaults();
        let conn = ResonanceConnection::new(1, 2, ConnectionType::Resonance, 0.8, 0.0, 0.0);

        formation.consider_connection(conn);
        let new_collectives = formation.process_pending(0.0);

        assert_eq!(formation.collective_count(), 1);
        assert!(!new_collectives.is_empty());
    }

    #[test]
    fn test_get_collective_for_entity() {
        let mut formation = CollectiveFormation::with_defaults();
        let conn = ResonanceConnection::new(1, 2, ConnectionType::Resonance, 0.8, 0.0, 0.0);

        formation.consider_connection(conn);
        formation.process_pending(0.0);

        let collective = formation.get_collective_for_entity(1);
        assert!(collective.is_some());
    }
}
