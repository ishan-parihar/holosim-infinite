//! Relationship Web - Inter-Entity Social Dynamics
//!
//! From V4 Roadmap Phase 6: "Social Emergence & Civilization Engine"
//! Gap #9 resolution: entities have social dynamics

use crate::entity_layer7::layer7::EntityId;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Relationship Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationshipType {
    Parent,
    Child,
    Sibling,
    Mate,
    Friend,
    Rival,
    Teacher,
    Student,
    Leader,
    Follower,
    Stranger,
}

impl RelationshipType {
    pub fn default_for_interaction(&self) -> (Float, Float) {
        match self {
            RelationshipType::Parent => (0.8, 0.7),
            RelationshipType::Child => (0.8, 0.5),
            RelationshipType::Sibling => (0.6, 0.6),
            RelationshipType::Mate => (0.9, 0.8),
            RelationshipType::Friend => (0.7, 0.6),
            RelationshipType::Rival => (-0.3, 0.2),
            RelationshipType::Teacher => (0.6, 0.8),
            RelationshipType::Student => (0.5, 0.5),
            RelationshipType::Leader => (0.4, 0.6),
            RelationshipType::Follower => (0.4, 0.4),
            RelationshipType::Stranger => (0.0, 0.1),
        }
    }
}

// ============================================================================
// Shared Experience
// ============================================================================

#[derive(Debug, Clone)]
pub struct SharedExperience {
    pub interaction_type: InteractionType,
    pub time: Float,
    pub impact: Float,
    pub description: String,
}

// ============================================================================
// Interaction Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionType {
    Cooperative,
    Competitive,
    Teaching,
    Learning,
    Hostile,
    Loving,
    Neutral,
}

impl InteractionType {
    pub fn polarity_effect(&self) -> Float {
        match self {
            InteractionType::Cooperative => 0.1,
            InteractionType::Competitive => -0.05,
            InteractionType::Teaching => 0.15,
            InteractionType::Learning => 0.1,
            InteractionType::Hostile => -0.15,
            InteractionType::Loving => 0.2,
            InteractionType::Neutral => 0.0,
        }
    }
}

// ============================================================================
// Relationship
// ============================================================================

#[derive(Debug, Clone)]
pub struct Relationship {
    pub relationship_type: RelationshipType,
    /// Strength: -1.0 (hostile) to 1.0 (loving)
    pub strength: Float,
    /// Trust: 0.0 (no trust) to 1.0 (complete trust)
    pub trust: Float,
    /// Shared experiences
    pub shared_experiences: Vec<SharedExperience>,
    /// Karmic balance
    pub karmic_balance: Float,
    /// How long this relationship has existed
    pub duration: Float,
}

impl Relationship {
    pub fn new_stranger() -> Self {
        Self {
            relationship_type: RelationshipType::Stranger,
            strength: 0.0,
            trust: 0.1,
            shared_experiences: Vec::new(),
            karmic_balance: 0.0,
            duration: 0.0,
        }
    }

    pub fn with_type(relationship_type: RelationshipType) -> Self {
        let (strength, trust) = relationship_type.default_for_interaction();
        Self {
            relationship_type,
            strength,
            trust,
            shared_experiences: Vec::new(),
            karmic_balance: 0.0,
            duration: 0.0,
        }
    }
}

// ============================================================================
// Interaction Event
// ============================================================================

#[derive(Debug, Clone)]
pub struct Interaction {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
    pub interaction_type: InteractionType,
    pub time: Float,
    pub impact: Float,
    pub context: String,
}

// ============================================================================
// Relationship Web
// ============================================================================

#[derive(Debug)]
pub struct RelationshipWeb {
    pub relationships: HashMap<(u64, u64), Relationship>,
    pub interaction_history: Vec<Interaction>,
    pub max_history: usize,
}

impl Default for RelationshipWeb {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipWeb {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
            interaction_history: Vec::new(),
            max_history: 10000,
        }
    }

    /// Update relationship from interaction
    pub fn update_from_interaction(&mut self, interaction: &Interaction) {
        let key = (
            interaction
                .entity_a
                .as_u64()
                .min(interaction.entity_b.as_u64()),
            interaction
                .entity_a
                .as_u64()
                .max(interaction.entity_b.as_u64()),
        );

        let relationship = self
            .relationships
            .entry(key)
            .or_insert_with(Relationship::new_stranger);

        match interaction.interaction_type {
            InteractionType::Cooperative => {
                relationship.strength += 0.01;
                relationship.trust += 0.005;
            }
            InteractionType::Competitive => {
                relationship.strength -= 0.01;
                relationship.trust -= 0.005;
            }
            InteractionType::Teaching => {
                relationship.strength += 0.02;
                relationship.trust += 0.01;
                relationship.relationship_type = RelationshipType::Teacher;
            }
            InteractionType::Learning => {
                relationship.strength += 0.01;
                relationship.relationship_type = RelationshipType::Student;
            }
            InteractionType::Hostile => {
                relationship.strength -= 0.05;
                relationship.trust -= 0.02;
                relationship.relationship_type = RelationshipType::Rival;
            }
            InteractionType::Loving => {
                relationship.strength += 0.1;
                relationship.trust += 0.08;
            }
            InteractionType::Neutral => {}
        }

        // Clamp values
        relationship.strength = relationship.strength.clamp(-1.0, 1.0);
        relationship.trust = relationship.trust.clamp(0.0, 1.0);

        // Add shared experience
        relationship.shared_experiences.push(SharedExperience {
            interaction_type: interaction.interaction_type,
            time: interaction.time,
            impact: interaction.impact,
            description: interaction.context.clone(),
        });

        // Update karmic balance
        relationship.karmic_balance +=
            interaction.interaction_type.polarity_effect() * interaction.impact;

        // Update duration
        relationship.duration += 1.0;

        // Keep history bounded
        self.interaction_history.push(interaction.clone());
        if self.interaction_history.len() > self.max_history {
            self.interaction_history.remove(0);
        }
    }

    /// Get relationship between two entities
    pub fn get_relationship(
        &self,
        entity_a: EntityId,
        entity_b: EntityId,
    ) -> Option<&Relationship> {
        let key = (
            entity_a.as_u64().min(entity_b.as_u64()),
            entity_a.as_u64().max(entity_b.as_u64()),
        );
        self.relationships.get(&key)
    }

    /// Get all relationships for an entity
    pub fn get_entity_relationships(&self, entity: EntityId) -> Vec<(u64, &Relationship)> {
        let entity_key = entity.as_u64();
        self.relationships
            .iter()
            .filter(|((a, b), _)| *a == entity_key || *b == entity_key)
            .map(
                |((a, b), r)| {
                    if *a == entity_key {
                        (*b, r)
                    } else {
                        (*a, r)
                    }
                },
            )
            .collect()
    }

    /// Calculate total love/connection for an entity
    pub fn calculate_connection(&self, entity: EntityId) -> Float {
        self.get_entity_relationships(entity)
            .iter()
            .map(|(_, r)| r.strength.max(0.0) * r.trust)
            .sum()
    }

    /// Calculate total conflict for an entity
    pub fn calculate_conflict(&self, entity: EntityId) -> Float {
        self.get_entity_relationships(entity)
            .iter()
            .map(|(_, r)| r.strength.min(0.0).abs() * r.trust)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let _web = RelationshipWeb::new();
        let rel = Relationship::new_stranger();
        assert_eq!(rel.relationship_type, RelationshipType::Stranger);
    }

    #[test]
    fn test_interaction_updates_relationship() {
        let mut web = RelationshipWeb::new();

        let entity_a = EntityId::new("entity_1".to_string());
        let entity_b = EntityId::new("entity_2".to_string());

        let interaction = Interaction {
            entity_a: entity_a.clone(),
            entity_b: entity_b.clone(),
            interaction_type: InteractionType::Loving,
            time: 0.0,
            impact: 0.5,
            context: "Helped each other".to_string(),
        };

        web.update_from_interaction(&interaction);

        let rel = web.get_relationship(entity_a, entity_b).unwrap();
        assert!(rel.strength > 0.0);
        assert!(rel.trust > 0.0);
    }
}
