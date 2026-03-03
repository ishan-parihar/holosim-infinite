//! Civilization Engine - Groups, Governance, Technology
//!
//! From V4 Roadmap Phase 6: "Social Emergence & Civilization Engine"

use crate::entity_layer7::layer7::EntityId;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Group & Civilization Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceType {
    Tribal,
    Chiefdom,
    Kingdom,
    Republic,
    Democracy,
    Technocracy,
    Theocracy,
    SocialMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TechnologyLevel {
    PreLanguage,
    StoneAge,
    BronzeAge,
    IronAge,
    Classical,
    Medieval,
    Industrial,
    Modern,
    Information,
    PostPhysical,
}

// ============================================================================
// Social Group
// ============================================================================

#[derive(Debug, Clone)]
pub struct SocialGroup {
    pub id: u64,
    pub name: String,
    pub members: Vec<EntityId>,
    pub governance: GovernanceType,
    pub territory: Vec<[f64; 3]>, // Positions
    pub cohesion: Float,          // 0.0 - 1.0
    pub formed_at: Float,
}

impl SocialGroup {
    pub fn new(id: u64, name: String, founder: EntityId) -> Self {
        Self {
            id,
            name,
            members: vec![founder],
            governance: GovernanceType::Tribal,
            territory: Vec::new(),
            cohesion: 0.5,
            formed_at: 0.0,
        }
    }

    pub fn add_member(&mut self, entity: EntityId) {
        if !self.members.contains(&entity) {
            self.members.push(entity);
        }
    }

    pub fn remove_member(&mut self, entity: &EntityId) {
        self.members.retain(|e| e != entity);
    }
}

// ============================================================================
// Technology
// ============================================================================

#[derive(Debug, Clone)]
pub struct Technology {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub discovered_by: u64, // Group ID
    pub discovery_time: Float,
    pub prerequisites: Vec<u64>,
    pub impact: TechnologyImpact,
}

#[derive(Debug, Clone)]
pub struct TechnologyImpact {
    pub efficiency_boost: Float,
    pub capability_add: Vec<String>,
    pub social_change: Float,
}

// ============================================================================
// Civilization
// ============================================================================

#[derive(Debug, Clone)]
pub struct Civilization {
    pub id: u64,
    pub name: String,
    pub population: usize,
    pub territory: Vec<[f64; 3]>,
    pub governance: GovernanceType,
    pub technology_level: TechnologyLevel,
    pub technology_ids: Vec<u64>,
    pub cultural_values: CulturalValues,
    pub average_polarity: Float,
    pub consciousness_level: Float,
    pub founded_at: Float,
}

#[derive(Debug, Clone)]
pub struct CulturalValues {
    pub openness: Float,          // To new experiences
    pub conscientiousness: Float, // Order/structure
    pub extraversion: Float,      // Social energy
    pub agreeableness: Float,     // Cooperation
    pub neuroticism: Float,       // Stability
}

impl Default for CulturalValues {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        }
    }
}

// ============================================================================
// Civilization Engine
// ============================================================================

#[derive(Debug)]
pub struct CivilizationEngine {
    pub groups: HashMap<u64, SocialGroup>,
    pub civilizations: HashMap<u64, Civilization>,
    pub technologies: HashMap<u64, Technology>,
    pub next_group_id: u64,
    pub next_civ_id: u64,
    pub next_tech_id: u64,
}

impl Default for CivilizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CivilizationEngine {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            civilizations: HashMap::new(),
            technologies: HashMap::new(),
            next_group_id: 1,
            next_civ_id: 1,
            next_tech_id: 1,
        }
    }

    /// Create a new social group
    pub fn create_group(&mut self, name: String, founder: EntityId) -> u64 {
        let id = self.next_group_id;
        self.next_group_id += 1;

        let group = SocialGroup::new(id, name, founder);
        self.groups.insert(id, group);
        id
    }

    /// Form a civilization from groups
    pub fn form_civilization(&mut self, name: String, groups: Vec<u64>) -> Option<u64> {
        // Collect all members
        let mut members = Vec::new();
        let mut territory = Vec::new();

        for group_id in &groups {
            if let Some(group) = self.groups.get(group_id) {
                members.extend(group.members.clone());
                territory.extend(group.territory.clone());
            }
        }

        if members.len() < 10 {
            return None; // Need at least 10 entities
        }

        let civ_id = self.next_civ_id;
        self.next_civ_id += 1;

        let civ = Civilization {
            id: civ_id,
            name,
            population: members.len(),
            territory,
            governance: GovernanceType::Chiefdom,
            technology_level: TechnologyLevel::StoneAge,
            technology_ids: Vec::new(),
            cultural_values: CulturalValues::default(),
            average_polarity: 0.0,
            consciousness_level: 0.2,
            founded_at: 0.0,
        };

        self.civilizations.insert(civ_id, civ);
        Some(civ_id)
    }

    /// Advance technology level
    pub fn discover_technology(
        &mut self,
        civ_id: u64,
        name: String,
        description: String,
    ) -> Option<u64> {
        let civ = self.civilizations.get_mut(&civ_id)?;

        let tech_id = self.next_tech_id;
        self.next_tech_id += 1;

        let tech = Technology {
            id: tech_id,
            name: name.clone(),
            description,
            discovered_by: civ_id,
            discovery_time: 0.0,
            prerequisites: civ.technology_ids.clone(),
            impact: TechnologyImpact {
                efficiency_boost: 0.1,
                capability_add: vec![name],
                social_change: 0.05,
            },
        };

        self.technologies.insert(tech_id, tech);
        civ.technology_ids.push(tech_id);

        // Advance technology level
        civ.technology_level = match civ.technology_level {
            TechnologyLevel::PreLanguage => TechnologyLevel::StoneAge,
            TechnologyLevel::StoneAge => TechnologyLevel::BronzeAge,
            TechnologyLevel::BronzeAge => TechnologyLevel::IronAge,
            TechnologyLevel::IronAge => TechnologyLevel::Classical,
            TechnologyLevel::Classical => TechnologyLevel::Medieval,
            TechnologyLevel::Medieval => TechnologyLevel::Industrial,
            TechnologyLevel::Industrial => TechnologyLevel::Modern,
            TechnologyLevel::Modern => TechnologyLevel::Information,
            TechnologyLevel::Information => TechnologyLevel::PostPhysical,
            TechnologyLevel::PostPhysical => TechnologyLevel::PostPhysical,
        };

        Some(tech_id)
    }
}
