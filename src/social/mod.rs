//! Social Module - Inter-Entity Dynamics and Collective Emergence

pub mod civilization;
pub mod communication;
pub mod harvest;
pub mod relationship_web;
pub mod smc_engine;
pub mod wanderer;

pub use civilization::{
    Civilization, CivilizationEngine, GovernanceType, SocialGroup, TechnologyLevel,
};
pub use communication::{
    CommunicationSystem, CommunicationType, Message, MessageContent, TelepathicLink,
    TransmissionMode,
};
pub use harvest::{HarvestEngine, HarvestEvent, HarvestResult, HarvestStats, HarvestType};
pub use relationship_web::{
    Interaction, InteractionType, Relationship, RelationshipType, RelationshipWeb,
};
pub use smc_engine::{Polarity, SharedMemory, SocialMemoryComplex, SocialMemoryComplexEngine};
pub use wanderer::{
    WandererAbility, WandererContract, WandererEngine, WandererEvent, WandererId, WandererStats,
    WandererType,
};

use crate::entity_layer7::EntityId;
use crate::evolution_density_octave::density_octave::Density;

#[derive(Debug, Clone)]
pub struct SocialTickContext {
    pub tick: u64,
    pub active_entities: Vec<EntityId>,
    pub global_density: Density,
    pub ticks_since_harvest: u64,
}

impl Default for SocialTickContext {
    fn default() -> Self {
        SocialTickContext {
            tick: 0,
            active_entities: Vec::new(),
            global_density: Density::Third,
            ticks_since_harvest: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct SocialState {
    pub relationship_web: RelationshipWeb,
    pub communication: CommunicationSystem,
    pub civilization: CivilizationEngine,
    pub social_memory: SocialMemoryComplexEngine,
    pub harvest_engine: HarvestEngine,
    pub wanderer_engine: WandererEngine,
}

impl SocialState {
    pub fn new() -> Self {
        SocialState {
            relationship_web: RelationshipWeb::new(),
            communication: CommunicationSystem::new(),
            civilization: CivilizationEngine::new(),
            social_memory: SocialMemoryComplexEngine::new(),
            harvest_engine: HarvestEngine::new(),
            wanderer_engine: WandererEngine::new(),
        }
    }

    pub fn tick(&mut self, context: &SocialTickContext) -> SocialTickResult {
        let mut result = SocialTickResult::default();

        if context.ticks_since_harvest >= 1000 {
            result.harvest_results = self
                .harvest_engine
                .process_harvest(context.tick, &context.active_entities);
        }

        result.wanderer_events = self
            .wanderer_engine
            .tick(context.tick, &context.active_entities);

        result
    }
}

#[derive(Debug, Default)]
pub struct SocialTickResult {
    pub relationship_updates: Vec<RelationshipUpdate>,
    pub communication_events: Vec<Message>,
    pub civilization_events: Vec<CivilizationEvent>,
    pub smc_events: Vec<SmcEvent>,
    pub harvest_results: Vec<HarvestResult>,
    pub wanderer_events: Vec<WandererEvent>,
}

#[derive(Debug, Clone)]
pub struct RelationshipUpdate {
    pub entity_a_key: u64,
    pub entity_b_key: u64,
    pub change_type: RelationshipChangeType,
    pub new_strength: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipChangeType {
    Formed,
    Strengthened,
    Weakened,
    Broken,
    Transformed,
}

#[derive(Debug, Clone)]
pub enum CivilizationEvent {
    GroupFormed(u64),
    GroupDissolved(u64),
    TechnologyBreakthrough(TechnologyLevel),
    GovernanceChange(GovernanceType),
}

#[derive(Debug, Clone)]
pub enum SmcEvent {
    Formed(u64),
    Dissolved(u64),
    MemberJoined(u64),
    MemberLeft(u64),
    ResonanceStrengthened,
}
