//! Polarity Factions Module
//!
//! Week 95b: Factions as Polarity Alignment
//!
//! This module implements factions based on Service-to-Self (STS) vs Service-to-Others (STO)
//! polarity alignment, following the 7 rays from the cosmological architecture.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "Archetype 22 (The Choice): Creates polarity by choosing between
//! > Service-to-Others (STO) and Service-to-Self (STS)"
//!
//! > "The 7 rays represent the fundamental distortions of the One Infinite Creator:
//! > 3 primal distortions (Free Will, Love/Logos, Light) and 4 sub-distortions"
//!
//! ## Key Principles
//!
//! 1. **Polarity as Foundation**: Factions form based on STS/STO polarity (the fundamental choice)
//! 2. **The 7 Rays Influence**: Each faction aligns with one of the 7 rays, affecting characteristics
//! 3. **Collective Resonance**: Faction membership affects entity's resonance pattern
//! 4. **Territory Control**: Factions control territories that generate catalyst
//! 5. **Dynamic Relationships**: Factions can form alliances or declare enmity
//! 6. **Rank Progression**: Higher ranks grant access to faction-specific quests and trade benefits
//! 7. **Holographic Structure**: Each faction contains the whole - the complete archetypical pattern

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::advanced_game_mechanics::{
    AdvancedGameMechanicsError, CatalystAmount, FactionId, Float, Polarity, QuestId, Result,
    Timestamp,
};
use crate::simulation_v3::holographic_inventory::ResonancePattern;
use std::collections::HashMap;

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Unique identifier for a territory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TerritoryId(pub u64);

impl TerritoryId {
    /// Create a new TerritoryId from a u64 value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TerritoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Territory-{}", self.0)
    }
}

/// Trade benefits provided by faction membership
#[derive(Debug, Clone, PartialEq)]
pub struct TradeBenefits {
    /// Discount on faction-aligned items (0.0-1.0, where 1.0 = 100% discount)
    pub faction_discount: Float,
    /// Bonus to resonance compatibility with faction members
    pub resonance_bonus: Float,
    /// Access to exclusive faction items
    pub exclusive_items: Vec<u64>,
    /// Catalyst generation bonus
    pub catalyst_bonus: Float,
}

impl TradeBenefits {
    /// Create default trade benefits
    pub fn new() -> Self {
        Self {
            faction_discount: 0.0,
            resonance_bonus: 0.0,
            exclusive_items: Vec::new(),
            catalyst_bonus: 0.0,
        }
    }

    /// Create trade benefits for a specific faction rank
    pub fn for_rank(rank: &FactionRank) -> Self {
        match rank {
            FactionRank::Initiate => Self {
                faction_discount: 0.05,
                resonance_bonus: 0.05,
                exclusive_items: Vec::new(),
                catalyst_bonus: 0.02,
            },
            FactionRank::Member => Self {
                faction_discount: 0.10,
                resonance_bonus: 0.10,
                exclusive_items: vec![1, 2],
                catalyst_bonus: 0.05,
            },
            FactionRank::Veteran => Self {
                faction_discount: 0.15,
                resonance_bonus: 0.15,
                exclusive_items: vec![1, 2, 3, 4],
                catalyst_bonus: 0.08,
            },
            FactionRank::Officer => Self {
                faction_discount: 0.20,
                resonance_bonus: 0.20,
                exclusive_items: vec![1, 2, 3, 4, 5, 6],
                catalyst_bonus: 0.12,
            },
            FactionRank::Leader | FactionRank::Founder => Self {
                faction_discount: 0.25,
                resonance_bonus: 0.25,
                exclusive_items: vec![1, 2, 3, 4, 5, 6, 7, 8],
                catalyst_bonus: 0.15,
            },
        }
    }
}

impl Default for TradeBenefits {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// POLARITY EXTENSION
// ============================================================================

impl Polarity {
    /// Get the opposite polarity
    /// STO <-> STS, Balanced stays Balanced
    pub fn opposite(&self) -> Self {
        match self {
            Polarity::ServiceToOthers => Polarity::ServiceToSelf,
            Polarity::ServiceToSelf => Polarity::ServiceToOthers,
            Polarity::Balanced => Polarity::Balanced,
        }
    }

    /// Check if this polarity is compatible with another
    /// Returns true if polarities are not opposing
    pub fn is_compatible(&self, other: &Polarity) -> bool {
        self.is_compatible_with(other)
    }

    /// Calculate compatibility score (0.0-1.0) with another polarity
    pub fn compatibility_with(&self, other: &Polarity) -> Float {
        match (self, other) {
            (Polarity::ServiceToOthers, Polarity::ServiceToOthers) => 1.0,
            (Polarity::ServiceToSelf, Polarity::ServiceToSelf) => 1.0,
            (Polarity::Balanced, _) => 0.5,
            (_, Polarity::Balanced) => 0.5,
            _ => 0.0,
        }
    }
}

// ============================================================================
// FACTION RANK
// ============================================================================

/// Rank within a faction hierarchy
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Each step INCLUDES all previous development and TRANSCENDS by adding new development."
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FactionRank {
    /// New member - just joined
    Initiate = 0,
    /// Established member - proven commitment
    Member = 1,
    /// Experienced member - demonstrated loyalty
    Veteran = 2,
    /// Leadership role - guides other members
    Officer = 3,
    /// Faction leader - directs faction strategy
    Leader = 4,
    /// Created the faction - highest authority
    Founder = 5,
}

impl FactionRank {
    /// Get the numeric value of this rank
    pub fn value(&self) -> u32 {
        *self as u32
    }

    /// Get the display name for this rank
    pub fn display_name(&self) -> &'static str {
        match self {
            FactionRank::Initiate => "Initiate",
            FactionRank::Member => "Member",
            FactionRank::Veteran => "Veteran",
            FactionRank::Officer => "Officer",
            FactionRank::Leader => "Leader",
            FactionRank::Founder => "Founder",
        }
    }

    /// Check if this rank can promote others
    pub fn can_promote(&self) -> bool {
        matches!(
            self,
            FactionRank::Officer | FactionRank::Leader | FactionRank::Founder
        )
    }

    /// Check if this rank can invite new members
    pub fn can_invite(&self) -> bool {
        matches!(
            self,
            FactionRank::Member
                | FactionRank::Veteran
                | FactionRank::Officer
                | FactionRank::Leader
                | FactionRank::Founder
        )
    }

    /// Get the next rank up (if any)
    pub fn next_rank(&self) -> Option<FactionRank> {
        match self {
            FactionRank::Initiate => Some(FactionRank::Member),
            FactionRank::Member => Some(FactionRank::Veteran),
            FactionRank::Veteran => Some(FactionRank::Officer),
            FactionRank::Officer => Some(FactionRank::Leader),
            FactionRank::Leader => None,
            FactionRank::Founder => None,
        }
    }
}

impl std::fmt::Display for FactionRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// FACTION SPECIALIZATION
// ============================================================================

/// Specialization within a faction
///
/// Each member can have one or more specializations that determine
/// their role and contributions to the faction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactionSpecialization {
    /// Combat focus - defending faction interests
    Warrior,
    /// Alliance focus - forming relationships with other factions
    Diplomat,
    /// Item creation - crafting for the faction
    Crafter,
    /// Territory expansion - discovering and claiming new territories
    Explorer,
    /// Knowledge/catalyst - researching and generating catalyst
    Researcher,
    /// Economic focus - managing faction resources and trade
    Trader,
}

impl FactionSpecialization {
    /// Get the display name for this specialization
    pub fn display_name(&self) -> &'static str {
        match self {
            FactionSpecialization::Warrior => "Warrior",
            FactionSpecialization::Diplomat => "Diplomat",
            FactionSpecialization::Crafter => "Crafter",
            FactionSpecialization::Explorer => "Explorer",
            FactionSpecialization::Researcher => "Researcher",
            FactionSpecialization::Trader => "Trader",
        }
    }

    /// Get the contribution type associated with this specialization
    pub fn primary_contribution(&self) -> ContributionType {
        match self {
            FactionSpecialization::Warrior => ContributionType::ActiveParticipation,
            FactionSpecialization::Diplomat => ContributionType::DiplomaticEfforts,
            FactionSpecialization::Crafter => ContributionType::ResourceContribution,
            FactionSpecialization::Explorer => ContributionType::TerritoryControl,
            FactionSpecialization::Researcher => ContributionType::ResonanceHarmonization,
            FactionSpecialization::Trader => ContributionType::ResourceContribution,
        }
    }
}

impl std::fmt::Display for FactionSpecialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// CONTRIBUTION TYPE
// ============================================================================

/// Ways an entity can contribute to their faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContributionType {
    /// Combat, quests, active participation
    ActiveParticipation,
    /// Items, catalyst, resources
    ResourceContribution,
    /// Aligning personal resonance with faction
    ResonanceHarmonization,
    /// Controlling territory for the faction
    TerritoryControl,
    /// Forming alliances with other factions
    DiplomaticEfforts,
}

impl ContributionType {
    /// Get the display name for this contribution type
    pub fn display_name(&self) -> &'static str {
        match self {
            ContributionType::ActiveParticipation => "Active Participation",
            ContributionType::ResourceContribution => "Resource Contribution",
            ContributionType::ResonanceHarmonization => "Resonance Harmonization",
            ContributionType::TerritoryControl => "Territory Control",
            ContributionType::DiplomaticEfforts => "Diplomatic Efforts",
        }
    }

    /// Get the base contribution value for this type
    pub fn base_value(&self) -> Float {
        match self {
            ContributionType::ActiveParticipation => 1.0,
            ContributionType::ResourceContribution => 0.8,
            ContributionType::ResonanceHarmonization => 1.2,
            ContributionType::TerritoryControl => 1.5,
            ContributionType::DiplomaticEfforts => 1.0,
        }
    }
}

impl std::fmt::Display for ContributionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// RESONANCE CONTRIBUTION
// ============================================================================

/// How an entity contributes to faction resonance
#[derive(Debug, Clone)]
pub struct ResonanceContribution {
    /// Entity making the contribution
    pub entity_id: EntityId,
    /// Amount of contribution (0.0-1.0)
    pub contribution_amount: Float,
    /// Type of contribution
    pub contribution_type: ContributionType,
    /// Resonance pattern being contributed
    pub resonance_pattern: ResonancePattern,
    /// When the contribution was made
    pub contribution_time: Timestamp,
}

impl ResonanceContribution {
    /// Create a new resonance contribution
    pub fn new(
        entity_id: EntityId,
        contribution_type: ContributionType,
        resonance_pattern: ResonancePattern,
        time: Timestamp,
    ) -> Self {
        Self {
            entity_id,
            contribution_amount: contribution_type.base_value(),
            contribution_type,
            resonance_pattern,
            contribution_time: time,
        }
    }

    /// Create a contribution with a specific amount
    pub fn with_amount(mut self, amount: Float) -> Self {
        self.contribution_amount = amount.clamp(0.0, 1.0);
        self
    }
}

// ============================================================================
// FACTION RESONANCE
// ============================================================================

/// Faction's collective resonance pattern
///
/// The combined resonance of all faction members creates a collective
/// resonance field that extends beyond individual members.
#[derive(Debug, Clone)]
pub struct FactionResonance {
    /// Base resonance pattern of the faction
    pub base_resonance: ResonancePattern,
    /// Individual member contributions
    pub member_contributions: HashMap<EntityId, ResonanceContribution>,
    /// Calculated collective strength
    pub collective_strength: Float,
    /// Stability of the collective resonance (0.0-1.0)
    pub resonance_stability: Float,
}

impl FactionResonance {
    /// Create a new faction resonance with a base pattern
    pub fn new(base_resonance: ResonancePattern) -> Self {
        Self {
            base_resonance,
            member_contributions: HashMap::new(),
            collective_strength: 0.0,
            resonance_stability: 1.0,
        }
    }

    /// Add a member's contribution to the faction resonance
    pub fn add_member_contribution(
        &mut self,
        entity_id: EntityId,
        contribution: ResonanceContribution,
    ) -> Result<()> {
        self.member_contributions.insert(entity_id, contribution);
        self.recalculate_collective_resonance();
        Ok(())
    }

    /// Remove a member's contribution
    pub fn remove_member_contribution(&mut self, entity_id: EntityId) -> Result<()> {
        if self.member_contributions.remove(&entity_id).is_none() {
            return Err(AdvancedGameMechanicsError::EntityNotFound(entity_id));
        }
        self.recalculate_collective_resonance();
        Ok(())
    }

    /// Recalculate the collective resonance from all member contributions
    pub fn recalculate_collective_resonance(&mut self) -> ResonancePattern {
        if self.member_contributions.is_empty() {
            self.collective_strength = 0.0;
            self.resonance_stability = 1.0;
            return self.base_resonance.clone();
        }

        // Calculate weighted average of all member resonance patterns
        let mut combined_pattern = [0.0; 8];
        let mut total_weight = 0.0;
        let mut stability_sum = 0.0;

        for contribution in self.member_contributions.values() {
            let weight = contribution.contribution_amount;
            total_weight += weight;
            stability_sum += contribution.resonance_pattern.stability * weight;

            for i in 0..8 {
                combined_pattern[i] += contribution.resonance_pattern.pattern[i] * weight;
            }
        }

        // Normalize the pattern
        if total_weight > 0.0 {
            for i in 0..8 {
                combined_pattern[i] /= total_weight;
            }
        }

        // Blend with base resonance (faction has its own identity)
        let blend_factor = 0.3; // 30% base, 70% member contributions
        for i in 0..8 {
            combined_pattern[i] = self.base_resonance.pattern[i] * blend_factor
                + combined_pattern[i] * (1.0 - blend_factor);
        }

        self.collective_strength = total_weight.min(100.0); // Cap at 100
        self.resonance_stability = (stability_sum / total_weight).clamp(0.0, 1.0);

        ResonancePattern {
            pattern: combined_pattern,
            stability: self.resonance_stability,
            phase: self.base_resonance.phase,
        }
    }

    /// Get the influence radius of the faction's resonance field
    ///
    /// The influence radius determines how far the faction's resonance extends
    /// and affects nearby entities and territories.
    pub fn get_influence_radius(&self) -> Float {
        let base_radius = 10.0;
        let strength_multiplier = (self.collective_strength / 10.0).sqrt();
        let stability_factor = self.resonance_stability;

        base_radius * strength_multiplier * stability_factor
    }

    /// Get the current collective resonance pattern
    pub fn get_collective_pattern(&self) -> ResonancePattern {
        let mut pattern = self.base_resonance.clone();
        pattern.stability = self.resonance_stability;
        pattern
    }
}

// ============================================================================
// FACTION MEMBERSHIP
// ============================================================================

/// Manages an entity's membership in a faction
#[derive(Debug, Clone)]
pub struct FactionMembership {
    /// Entity that is a member
    pub entity_id: EntityId,
    /// Faction they belong to
    pub faction_id: FactionId,
    /// When they joined
    pub join_time: Timestamp,
    /// Current rank in the faction
    pub rank: FactionRank,
    /// Total contribution score
    pub contribution_score: Float,
    /// Loyalty score (0.0-1.0, affects betrayal mechanics)
    pub loyalty_score: Float,
    /// Specializations within the faction
    pub specializations: Vec<FactionSpecialization>,
}

impl FactionMembership {
    /// Create a new faction membership
    pub fn new(entity_id: EntityId, faction_id: FactionId, join_time: Timestamp) -> Self {
        Self {
            entity_id,
            faction_id,
            join_time,
            rank: FactionRank::Initiate,
            contribution_score: 0.0,
            loyalty_score: 0.5, // Start at neutral loyalty
            specializations: Vec::new(),
        }
    }

    /// Promote the member to the next rank
    pub fn promote(&mut self) -> Result<FactionRank> {
        if let Some(new_rank) = self.rank.next_rank() {
            self.rank = new_rank;
            Ok(new_rank)
        } else {
            Err(AdvancedGameMechanicsError::InvalidOperation(
                "Already at highest rank".to_string(),
            ))
        }
    }

    /// Add a contribution to the member's score
    pub fn add_contribution(&mut self, amount: Float) {
        self.contribution_score += amount;
        // Increase loyalty slightly with contribution
        self.loyalty_score = (self.loyalty_score + amount * 0.01).min(1.0);
    }

    /// Add a specialization
    pub fn add_specialization(&mut self, specialization: FactionSpecialization) {
        if !self.specializations.contains(&specialization) {
            self.specializations.push(specialization);
        }
    }

    /// Check if member has a specific specialization
    pub fn has_specialization(&self, specialization: FactionSpecialization) -> bool {
        self.specializations.contains(&specialization)
    }

    /// Get trade benefits for this membership
    pub fn get_trade_benefits(&self) -> TradeBenefits {
        TradeBenefits::for_rank(&self.rank)
    }

    /// Calculate betrayal probability (lower loyalty = higher chance)
    pub fn betrayal_probability(&self) -> Float {
        (1.0 - self.loyalty_score) * 0.5 // Max 50% chance at 0 loyalty
    }
}

// ============================================================================
// TERRITORY
// ============================================================================

/// Territory that can be controlled by factions
///
/// Territories generate catalyst and provide strategic advantages
/// to controlling factions.
#[derive(Debug, Clone)]
pub struct Territory {
    /// Unique identifier
    pub territory_id: TerritoryId,
    /// Name of the territory
    pub name: String,
    /// Which faction controls this territory (if any)
    pub controlling_faction: Option<FactionId>,
    /// Resonance field of the territory
    pub resonance_field: ResonancePattern,
    /// Strategic importance (0.0-1.0)
    pub strategic_value: Float,
    /// Catalyst generated per time unit
    pub catalyst_generation_rate: Float,
    /// Factions currently contesting control
    pub contested_by: Vec<FactionId>,
}

impl Territory {
    /// Create a new territory
    pub fn new(territory_id: TerritoryId, name: String) -> Self {
        Self {
            territory_id,
            name,
            controlling_faction: None,
            resonance_field: ResonancePattern::new(),
            strategic_value: 0.5,
            catalyst_generation_rate: 1.0,
            contested_by: Vec::new(),
        }
    }

    /// Set the controlling faction
    pub fn set_controller(&mut self, faction_id: Option<FactionId>) {
        self.controlling_faction = faction_id;
        if faction_id.is_some() {
            self.contested_by.clear(); // Clear contests when controlled
        }
    }

    /// Add a contesting faction
    pub fn add_contestant(&mut self, faction_id: FactionId) {
        if !self.contested_by.contains(&faction_id) && self.controlling_faction != Some(faction_id)
        {
            self.contested_by.push(faction_id);
        }
    }

    /// Remove a contesting faction
    pub fn remove_contestant(&mut self, faction_id: FactionId) {
        self.contested_by.retain(|&id| id != faction_id);
    }

    /// Check if territory is contested
    pub fn is_contested(&self) -> bool {
        !self.contested_by.is_empty()
    }

    /// Calculate current catalyst generation (affected by strategic value)
    pub fn current_catalyst_generation(&self) -> Float {
        self.catalyst_generation_rate * (0.5 + self.strategic_value * 0.5)
    }
}

// ============================================================================
// RELATIONSHIP TYPE
// ============================================================================

/// Type of relationship between two factions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationshipType {
    /// Neutral - no special relationship
    Neutral,
    /// Allied - cooperate and support each other
    Allied,
    /// Friendly - positive relations
    Friendly,
    /// Hostile - negative relations
    Hostile,
    /// At War - active conflict
    AtWar,
}

impl RelationshipType {
    /// Check if factions can form alliances
    pub fn can_alliance(&self) -> bool {
        matches!(self, RelationshipType::Neutral | RelationshipType::Friendly)
    }

    /// Check if factions are enemies
    pub fn is_hostile(&self) -> bool {
        matches!(self, RelationshipType::Hostile | RelationshipType::AtWar)
    }

    /// Get compatibility multiplier for this relationship
    pub fn compatibility_multiplier(&self) -> Float {
        match self {
            RelationshipType::Allied => 1.0,
            RelationshipType::Friendly => 0.8,
            RelationshipType::Neutral => 0.5,
            RelationshipType::Hostile => 0.2,
            RelationshipType::AtWar => 0.0,
        }
    }
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipType::Neutral => write!(f, "Neutral"),
            RelationshipType::Allied => write!(f, "Allied"),
            RelationshipType::Friendly => write!(f, "Friendly"),
            RelationshipType::Hostile => write!(f, "Hostile"),
            RelationshipType::AtWar => write!(f, "At War"),
        }
    }
}

// ============================================================================
// FACTION
// ============================================================================

/// A polarity-aligned faction
///
/// Factions form around STS/STO polarization and align with one of the 7 rays.
/// They provide structure for collective action and shared purpose.
#[derive(Debug, Clone)]
pub struct Faction {
    /// Unique identifier
    pub faction_id: FactionId,
    /// Name of the faction
    pub name: String,
    /// Description of the faction's purpose
    pub description: String,
    /// Primary polarity alignment
    pub polarity_alignment: Polarity,
    /// Which of the 7 rays this faction aligns with (1-7, None if not aligned)
    pub ray_association: Option<u8>,
    /// Collective resonance pattern
    pub resonance_signature: ResonancePattern,
    /// Member entity IDs
    pub members: Vec<EntityId>,
    /// When the faction was founded
    pub founding_time: Timestamp,
    /// Faction progression level
    pub faction_level: u32,
    /// Shared catalyst pool
    pub collective_catalyst: CatalystAmount,
    /// Territories controlled by this faction
    pub territory_claims: Vec<TerritoryId>,
    /// Allied factions
    pub allied_factions: Vec<FactionId>,
    /// Enemy factions
    pub enemy_factions: Vec<FactionId>,
}

impl Faction {
    /// Create a new faction
    pub fn new(
        faction_id: FactionId,
        name: String,
        description: String,
        polarity: Polarity,
        founding_time: Timestamp,
    ) -> Self {
        Self {
            faction_id,
            name,
            description,
            polarity_alignment: polarity,
            ray_association: None,
            resonance_signature: ResonancePattern::new(),
            members: Vec::new(),
            founding_time,
            faction_level: 1,
            collective_catalyst: 0.0,
            territory_claims: Vec::new(),
            allied_factions: Vec::new(),
            enemy_factions: Vec::new(),
        }
    }

    /// Set the ray association (1-7)
    pub fn with_ray(mut self, ray: u8) -> Self {
        self.ray_association = Some(ray.clamp(1, 7));
        self
    }

    /// Add a member to the faction
    pub fn add_member(&mut self, entity_id: EntityId) {
        if !self.members.contains(&entity_id) {
            self.members.push(entity_id);
        }
    }

    /// Remove a member from the faction
    pub fn remove_member(&mut self, entity_id: EntityId) {
        self.members.retain(|&id| id != entity_id);
    }

    /// Check if entity is a member
    pub fn has_member(&self, entity_id: EntityId) -> bool {
        self.members.contains(&entity_id)
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Add an ally
    pub fn add_ally(&mut self, faction_id: FactionId) {
        if !self.allied_factions.contains(&faction_id) {
            self.allied_factions.push(faction_id);
        }
        // Remove from enemies if present
        self.enemy_factions.retain(|&id| id != faction_id);
    }

    /// Add an enemy
    pub fn add_enemy(&mut self, faction_id: FactionId) {
        if !self.enemy_factions.contains(&faction_id) {
            self.enemy_factions.push(faction_id);
        }
        // Remove from allies if present
        self.allied_factions.retain(|&id| id != faction_id);
    }

    /// Remove relationship
    pub fn remove_relationship(&mut self, faction_id: FactionId) {
        self.allied_factions.retain(|&id| id != faction_id);
        self.enemy_factions.retain(|&id| id != faction_id);
    }

    /// Claim a territory
    pub fn claim_territory(&mut self, territory_id: TerritoryId) {
        if !self.territory_claims.contains(&territory_id) {
            self.territory_claims.push(territory_id);
        }
    }

    /// Release a territory
    pub fn release_territory(&mut self, territory_id: TerritoryId) {
        self.territory_claims.retain(|&id| id != territory_id);
    }

    /// Add catalyst to the collective pool
    pub fn add_catalyst(&mut self, amount: CatalystAmount) {
        self.collective_catalyst += amount;
    }

    /// Spend catalyst from the collective pool
    pub fn spend_catalyst(&mut self, amount: CatalystAmount) -> Result<()> {
        if self.collective_catalyst >= amount {
            self.collective_catalyst -= amount;
            Ok(())
        } else {
            Err(AdvancedGameMechanicsError::InsufficientCatalyst {
                current: self.collective_catalyst,
                required: amount,
            })
        }
    }

    /// Check if can accept new members (based on level)
    pub fn can_accept_members(&self) -> bool {
        let max_members = 10 * self.faction_level as usize;
        self.members.len() < max_members
    }

    /// Get maximum member capacity
    pub fn member_capacity(&self) -> usize {
        10 * self.faction_level as usize
    }

    /// Level up the faction
    pub fn level_up(&mut self) {
        self.faction_level += 1;
    }
}

// ============================================================================
// FACTION ERROR
// ============================================================================

/// Errors specific to faction operations
#[derive(Debug, Clone, PartialEq)]
pub enum FactionError {
    /// Faction not found
    FactionNotFound(FactionId),
    /// Entity not found
    EntityNotFound(EntityId),
    /// Entity is already a member of a faction
    AlreadyMemberOfFaction {
        entity_id: EntityId,
        faction_id: FactionId,
    },
    /// Entity's polarity doesn't match faction requirement
    InsufficientPolarityAlignment {
        required: Polarity,
        actual: Polarity,
    },
    /// Rank too low for action
    RankTooLow {
        required: FactionRank,
        actual: FactionRank,
    },
    /// Faction has reached maximum capacity
    FactionAtCapacity { capacity: usize },
    /// Cannot join an enemy faction
    CannotJoinEnemyFaction {
        faction_id: FactionId,
        enemy_faction_id: FactionId,
    },
    /// Entity is not a member of the faction
    NotFactionMember {
        entity_id: EntityId,
        faction_id: FactionId,
    },
    /// Cannot leave as the faction leader
    CannotLeaveAsLeader {
        entity_id: EntityId,
        faction_id: FactionId,
    },
    /// Territory not found
    TerritoryNotFound(TerritoryId),
    /// Territory is already controlled
    TerritoryAlreadyControlled {
        territory_id: TerritoryId,
        controlling_faction: FactionId,
    },
}

impl std::fmt::Display for FactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactionError::FactionNotFound(id) => write!(f, "Faction not found: {}", id),
            FactionError::EntityNotFound(id) => write!(f, "Entity not found: {:?}", id),
            FactionError::AlreadyMemberOfFaction {
                entity_id,
                faction_id,
            } => {
                write!(
                    f,
                    "Entity {:?} is already a member of faction {}",
                    entity_id, faction_id
                )
            }
            FactionError::InsufficientPolarityAlignment { required, actual } => {
                write!(
                    f,
                    "Polarity alignment required: {}, actual: {}",
                    required, actual
                )
            }
            FactionError::RankTooLow { required, actual } => {
                write!(f, "Rank too low: required {}, actual {}", required, actual)
            }
            FactionError::FactionAtCapacity { capacity } => {
                write!(f, "Faction at capacity: {} members", capacity)
            }
            FactionError::CannotJoinEnemyFaction {
                faction_id,
                enemy_faction_id,
            } => {
                write!(
                    f,
                    "Cannot join enemy faction: {} is enemy of {}",
                    faction_id, enemy_faction_id
                )
            }
            FactionError::NotFactionMember {
                entity_id,
                faction_id,
            } => {
                write!(
                    f,
                    "Entity {:?} is not a member of faction {}",
                    entity_id, faction_id
                )
            }
            FactionError::CannotLeaveAsLeader {
                entity_id,
                faction_id,
            } => {
                write!(
                    f,
                    "Entity {:?} cannot leave as leader of faction {}",
                    entity_id, faction_id
                )
            }
            FactionError::TerritoryNotFound(id) => write!(f, "Territory not found: {}", id),
            FactionError::TerritoryAlreadyControlled {
                territory_id,
                controlling_faction,
            } => {
                write!(
                    f,
                    "Territory {} already controlled by {}",
                    territory_id, controlling_faction
                )
            }
        }
    }
}

impl std::error::Error for FactionError {}

// ============================================================================
// POLARITY FACTIONS SYSTEM
// ============================================================================

/// Main system for managing polarity-based factions
///
/// This is the central hub for all faction-related operations, including
/// creation, membership, relationships, and territory control.
pub struct PolarityFactions {
    /// All factions by ID
    factions: HashMap<FactionId, Faction>,
    /// All faction memberships by entity ID
    memberships: HashMap<EntityId, FactionMembership>,
    /// All territories by ID
    territories: HashMap<TerritoryId, Territory>,
    /// Relationships between faction pairs
    faction_relationships: HashMap<(FactionId, FactionId), RelationshipType>,
    /// Faction resonance data
    faction_resonances: HashMap<FactionId, FactionResonance>,
    /// Next available faction ID
    next_faction_id: u64,
    /// Next available territory ID
    next_territory_id: u64,
    /// Current simulation time
    current_time: Timestamp,
}

impl PolarityFactions {
    /// Create a new polarity factions system
    pub fn new() -> Self {
        Self {
            factions: HashMap::new(),
            memberships: HashMap::new(),
            territories: HashMap::new(),
            faction_relationships: HashMap::new(),
            faction_resonances: HashMap::new(),
            next_faction_id: 1,
            next_territory_id: 1,
            current_time: 0.0,
        }
    }

    /// Create a new faction
    pub fn create_faction(
        &mut self,
        founder_id: EntityId,
        name: String,
        description: String,
        polarity: Polarity,
        ray: Option<u8>,
    ) -> std::result::Result<Faction, FactionError> {
        let faction_id = FactionId::new(self.next_faction_id);
        self.next_faction_id += 1;

        let mut faction = Faction::new(faction_id, name, description, polarity, self.current_time);

        if let Some(r) = ray {
            faction = faction.with_ray(r);
        }

        // Founder is automatically added as first member
        faction.add_member(founder_id);

        // Create faction resonance
        let faction_resonance = FactionResonance::new(faction.resonance_signature.clone());
        self.faction_resonances
            .insert(faction_id, faction_resonance);

        // Create founder's membership
        let mut membership = FactionMembership::new(founder_id, faction_id, self.current_time);
        membership.rank = FactionRank::Founder;
        membership.loyalty_score = 1.0; // Founder has maximum loyalty
        self.memberships.insert(founder_id, membership);

        self.factions.insert(faction_id, faction.clone());

        Ok(faction)
    }

    /// Join a faction
    pub fn join_faction(
        &mut self,
        entity_id: EntityId,
        faction_id: FactionId,
    ) -> std::result::Result<FactionMembership, FactionError> {
        // Check if already in a faction
        if let Some(existing) = self.memberships.get(&entity_id) {
            return Err(FactionError::AlreadyMemberOfFaction {
                entity_id,
                faction_id: existing.faction_id,
            });
        }

        // Check faction exists
        let faction = self
            .factions
            .get(&faction_id)
            .ok_or(FactionError::FactionNotFound(faction_id))?;

        // Check capacity
        if !faction.can_accept_members() {
            return Err(FactionError::FactionAtCapacity {
                capacity: faction.member_capacity(),
            });
        }

        // Check if entity's current faction has this faction as enemy
        // (This would require tracking entity polarity separately)

        // Add member to faction
        let faction = self.factions.get_mut(&faction_id).unwrap();
        faction.add_member(entity_id);

        // Create membership
        let membership = FactionMembership::new(entity_id, faction_id, self.current_time);
        self.memberships.insert(entity_id, membership.clone());

        Ok(membership)
    }

    /// Leave a faction
    pub fn leave_faction(
        &mut self,
        entity_id: EntityId,
        faction_id: FactionId,
    ) -> std::result::Result<(), FactionError> {
        // Check membership
        let membership =
            self.memberships
                .get(&entity_id)
                .ok_or(FactionError::NotFactionMember {
                    entity_id,
                    faction_id,
                })?;

        if membership.faction_id != faction_id {
            return Err(FactionError::NotFactionMember {
                entity_id,
                faction_id,
            });
        }

        // Check if leader trying to leave
        if membership.rank == FactionRank::Leader || membership.rank == FactionRank::Founder {
            return Err(FactionError::CannotLeaveAsLeader {
                entity_id,
                faction_id,
            });
        }

        // Remove from faction
        if let Some(faction) = self.factions.get_mut(&faction_id) {
            faction.remove_member(entity_id);
        }

        // Remove from faction resonance
        if let Some(resonance) = self.faction_resonances.get_mut(&faction_id) {
            let _ = resonance.remove_member_contribution(entity_id);
        }

        // Remove membership
        self.memberships.remove(&entity_id);

        Ok(())
    }

    /// Promote a member to a new rank
    pub fn promote_member(
        &mut self,
        entity_id: EntityId,
        faction_id: FactionId,
        new_rank: FactionRank,
    ) -> std::result::Result<FactionRank, FactionError> {
        // Check membership
        let membership =
            self.memberships
                .get(&entity_id)
                .ok_or(FactionError::NotFactionMember {
                    entity_id,
                    faction_id,
                })?;

        if membership.faction_id != faction_id {
            return Err(FactionError::NotFactionMember {
                entity_id,
                faction_id,
            });
        }

        // Check if new rank is higher than current
        if new_rank <= membership.rank {
            return Err(FactionError::RankTooLow {
                required: new_rank,
                actual: membership.rank,
            });
        }

        // Update rank
        let membership = self.memberships.get_mut(&entity_id).unwrap();
        membership.rank = new_rank;

        Ok(new_rank)
    }

    /// Contribute to faction
    pub fn contribute_to_faction(
        &mut self,
        entity_id: EntityId,
        faction_id: FactionId,
        contribution: ResonanceContribution,
    ) -> std::result::Result<(), FactionError> {
        // Check membership
        let membership =
            self.memberships
                .get(&entity_id)
                .ok_or(FactionError::NotFactionMember {
                    entity_id,
                    faction_id,
                })?;

        if membership.faction_id != faction_id {
            return Err(FactionError::NotFactionMember {
                entity_id,
                faction_id,
            });
        }

        // Add contribution to faction resonance
        if let Some(resonance) = self.faction_resonances.get_mut(&faction_id) {
            resonance.add_member_contribution(entity_id, contribution)?;
        }

        // Update member's contribution score
        let membership = self.memberships.get_mut(&entity_id).unwrap();
        membership.add_contribution(1.0);

        Ok(())
    }

    /// Form an alliance between two factions
    pub fn form_alliance(
        &mut self,
        faction_a: FactionId,
        faction_b: FactionId,
    ) -> std::result::Result<(), FactionError> {
        if faction_a == faction_b {
            return Err(FactionError::InvalidOperation {
                message: "Cannot form alliance with self".to_string(),
            }
            .into());
        }

        // Check both factions exist
        let fa = self
            .factions
            .get(&faction_a)
            .ok_or(FactionError::FactionNotFound(faction_a))?;
        let _fb = self
            .factions
            .get(&faction_b)
            .ok_or(FactionError::FactionNotFound(faction_b))?;

        // Check polarity compatibility
        let fb = self.factions.get(&faction_b).unwrap();
        if !fa.polarity_alignment.is_compatible(&fb.polarity_alignment) {
            return Err(FactionError::InsufficientPolarityAlignment {
                required: fa.polarity_alignment,
                actual: fb.polarity_alignment,
            });
        }

        // Set relationship
        self.faction_relationships
            .insert((faction_a, faction_b), RelationshipType::Allied);
        self.faction_relationships
            .insert((faction_b, faction_a), RelationshipType::Allied);

        // Update faction ally lists
        if let Some(fa) = self.factions.get_mut(&faction_a) {
            fa.add_ally(faction_b);
        }
        if let Some(fb) = self.factions.get_mut(&faction_b) {
            fb.add_ally(faction_a);
        }

        Ok(())
    }

    /// Declare enmity between two factions
    pub fn declare_enmity(
        &mut self,
        faction_a: FactionId,
        faction_b: FactionId,
    ) -> std::result::Result<(), FactionError> {
        if faction_a == faction_b {
            return Err(FactionError::InvalidOperation {
                message: "Cannot declare enmity with self".to_string(),
            }
            .into());
        }

        // Check both factions exist
        let _fa = self
            .factions
            .get(&faction_a)
            .ok_or(FactionError::FactionNotFound(faction_a))?;
        let _fb = self
            .factions
            .get(&faction_b)
            .ok_or(FactionError::FactionNotFound(faction_b))?;

        // Set relationship
        self.faction_relationships
            .insert((faction_a, faction_b), RelationshipType::Hostile);
        self.faction_relationships
            .insert((faction_b, faction_a), RelationshipType::Hostile);

        // Update faction enemy lists
        if let Some(fa) = self.factions.get_mut(&faction_a) {
            fa.add_enemy(faction_b);
        }
        if let Some(fb) = self.factions.get_mut(&faction_b) {
            fb.add_enemy(faction_a);
        }

        Ok(())
    }

    /// Claim territory for a faction
    pub fn claim_territory(
        &mut self,
        faction_id: FactionId,
        territory_id: TerritoryId,
    ) -> std::result::Result<(), FactionError> {
        // Check faction exists
        let _faction = self
            .factions
            .get(&faction_id)
            .ok_or(FactionError::FactionNotFound(faction_id))?;

        // Check territory exists
        let territory = self
            .territories
            .get(&territory_id)
            .ok_or(FactionError::TerritoryNotFound(territory_id))?;

        // Check if already controlled
        if let Some(controller) = territory.controlling_faction {
            if controller != faction_id {
                return Err(FactionError::TerritoryAlreadyControlled {
                    territory_id,
                    controlling_faction: controller,
                });
            }
            // Already controlled by this faction
            return Ok(());
        }

        // Claim territory
        let territory = self.territories.get_mut(&territory_id).unwrap();
        territory.set_controller(Some(faction_id));

        // Update faction's territory list
        let faction = self.factions.get_mut(&faction_id).unwrap();
        faction.claim_territory(territory_id);

        Ok(())
    }

    /// Contest territory control
    pub fn contest_territory(
        &mut self,
        faction_id: FactionId,
        territory_id: TerritoryId,
    ) -> std::result::Result<(), FactionError> {
        // Check faction exists
        let _faction = self
            .factions
            .get(&faction_id)
            .ok_or(FactionError::FactionNotFound(faction_id))?;

        // Check territory exists
        let territory = self
            .territories
            .get(&territory_id)
            .ok_or(FactionError::TerritoryNotFound(territory_id))?;

        // Can't contest if already controlled by this faction
        if territory.controlling_faction == Some(faction_id) {
            return Err(FactionError::InvalidOperation {
                message: "Cannot contest territory already controlled".to_string(),
            }
            .into());
        }

        // Add contestant
        let territory = self.territories.get_mut(&territory_id).unwrap();
        territory.add_contestant(faction_id);

        Ok(())
    }

    /// Get all members of a faction
    pub fn get_faction_members(&self, faction_id: FactionId) -> Vec<EntityId> {
        self.factions
            .get(&faction_id)
            .map(|f| f.members.clone())
            .unwrap_or_default()
    }

    /// Get the faction an entity belongs to (if any)
    pub fn get_entity_faction(&self, entity_id: EntityId) -> Option<FactionId> {
        self.memberships.get(&entity_id).map(|m| m.faction_id)
    }

    /// Get faction resonance pattern
    pub fn get_faction_resonance(&self, faction_id: FactionId) -> Option<ResonancePattern> {
        self.faction_resonances
            .get(&faction_id)
            .map(|r| r.get_collective_pattern())
    }

    /// Get available factions for an entity to join
    pub fn get_available_factions(&self, entity_id: EntityId) -> Vec<FactionId> {
        // If already in a faction, can't join others
        if self.memberships.contains_key(&entity_id) {
            return Vec::new();
        }

        // Return all factions that can accept members
        self.factions
            .values()
            .filter(|f| f.can_accept_members())
            .map(|f| f.faction_id)
            .collect()
    }

    /// Calculate faction strength
    pub fn calculate_faction_strength(&self, faction_id: FactionId) -> Float {
        let faction = match self.factions.get(&faction_id) {
            Some(f) => f,
            None => return 0.0,
        };

        let base_strength = faction.faction_level as Float * 10.0;
        let member_strength = faction.member_count() as Float * 5.0;
        let territory_strength = faction.territory_claims.len() as Float * 8.0;

        let resonance_strength = self
            .faction_resonances
            .get(&faction_id)
            .map(|r| r.collective_strength)
            .unwrap_or(0.0);

        base_strength + member_strength + territory_strength + resonance_strength
    }

    /// Get faction-specific quests
    pub fn get_faction_quests(&self, faction_id: FactionId) -> Vec<QuestId> {
        let faction = match self.factions.get(&faction_id) {
            Some(f) => f,
            None => return Vec::new(),
        };

        // Generate quest IDs based on faction properties
        let mut quests = Vec::new();

        // Base quests for all factions
        quests.push(QuestId::new(faction_id.as_u64() * 100 + 1));
        quests.push(QuestId::new(faction_id.as_u64() * 100 + 2));

        // Additional quests based on level
        for i in 0..faction.faction_level.min(5) {
            quests.push(QuestId::new(faction_id.as_u64() * 100 + 10 + i as u64));
        }

        // Quests based on territories
        for (i, _territory) in faction.territory_claims.iter().enumerate() {
            quests.push(QuestId::new(faction_id.as_u64() * 1000 + i as u64));
        }

        quests
    }

    /// Get trade benefits for faction members
    pub fn get_faction_trade_benefits(&self, faction_id: FactionId) -> TradeBenefits {
        let faction = match self.factions.get(&faction_id) {
            Some(f) => f,
            None => return TradeBenefits::new(),
        };

        let base_benefits = TradeBenefits::new();
        let level_multiplier = 1.0 + (faction.faction_level as Float - 1.0) * 0.1;

        TradeBenefits {
            faction_discount: base_benefits.faction_discount * level_multiplier,
            resonance_bonus: base_benefits.resonance_bonus * level_multiplier,
            exclusive_items: base_benefits.exclusive_items,
            catalyst_bonus: base_benefits.catalyst_bonus * level_multiplier,
        }
    }

    /// Create a new territory
    pub fn create_territory(&mut self, name: String) -> Territory {
        let territory_id = TerritoryId::new(self.next_territory_id);
        self.next_territory_id += 1;

        let territory = Territory::new(territory_id, name);
        self.territories.insert(territory_id, territory.clone());

        territory
    }

    /// Get a faction by ID
    pub fn get_faction(&self, faction_id: FactionId) -> Option<&Faction> {
        self.factions.get(&faction_id)
    }

    /// Get a faction membership by entity ID
    pub fn get_membership(&self, entity_id: EntityId) -> Option<&FactionMembership> {
        self.memberships.get(&entity_id)
    }

    /// Get a territory by ID
    pub fn get_territory(&self, territory_id: TerritoryId) -> Option<&Territory> {
        self.territories.get(&territory_id)
    }

    /// Get relationship between two factions
    pub fn get_relationship(&self, faction_a: FactionId, faction_b: FactionId) -> RelationshipType {
        self.faction_relationships
            .get(&(faction_a, faction_b))
            .copied()
            .unwrap_or(RelationshipType::Neutral)
    }

    /// Update simulation time
    pub fn update_time(&mut self, new_time: Timestamp) {
        self.current_time = new_time;
    }

    /// Get current simulation time
    pub fn current_time(&self) -> Timestamp {
        self.current_time
    }

    /// Get all factions
    pub fn get_all_factions(&self) -> Vec<&Faction> {
        self.factions.values().collect()
    }

    /// Get all territories
    pub fn get_all_territories(&self) -> Vec<&Territory> {
        self.territories.values().collect()
    }
}

impl Default for PolarityFactions {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // Polarity Tests
    // ============================================================================

    #[test]
    fn test_polarity_opposite() {
        assert_eq!(
            Polarity::ServiceToOthers.opposite(),
            Polarity::ServiceToSelf
        );
        assert_eq!(
            Polarity::ServiceToSelf.opposite(),
            Polarity::ServiceToOthers
        );
        assert_eq!(Polarity::Balanced.opposite(), Polarity::Balanced);
    }

    #[test]
    fn test_polarity_compatibility() {
        assert!(Polarity::ServiceToOthers.is_compatible(&Polarity::ServiceToOthers));
        assert!(Polarity::ServiceToSelf.is_compatible(&Polarity::ServiceToSelf));
        assert!(!Polarity::ServiceToOthers.is_compatible(&Polarity::ServiceToSelf));
        assert!(Polarity::Balanced.is_compatible(&Polarity::ServiceToOthers));
        assert!(Polarity::Balanced.is_compatible(&Polarity::ServiceToSelf));
    }

    #[test]
    fn test_polarity_compatibility_score() {
        assert_eq!(
            Polarity::ServiceToOthers.compatibility_with(&Polarity::ServiceToOthers),
            1.0
        );
        assert_eq!(
            Polarity::ServiceToSelf.compatibility_with(&Polarity::ServiceToSelf),
            1.0
        );
        assert_eq!(
            Polarity::ServiceToOthers.compatibility_with(&Polarity::ServiceToSelf),
            0.0
        );
        assert_eq!(
            Polarity::Balanced.compatibility_with(&Polarity::ServiceToOthers),
            0.5
        );
        assert_eq!(
            Polarity::ServiceToOthers.compatibility_with(&Polarity::Balanced),
            0.5
        );
    }

    // ============================================================================
    // Faction Rank Tests
    // ============================================================================

    #[test]
    fn test_faction_rank_ordering() {
        assert!(FactionRank::Initiate < FactionRank::Member);
        assert!(FactionRank::Member < FactionRank::Veteran);
        assert!(FactionRank::Veteran < FactionRank::Officer);
        assert!(FactionRank::Officer < FactionRank::Leader);
        assert!(FactionRank::Leader < FactionRank::Founder);
    }

    #[test]
    fn test_faction_rank_can_promote() {
        assert!(!FactionRank::Initiate.can_promote());
        assert!(!FactionRank::Member.can_promote());
        assert!(!FactionRank::Veteran.can_promote());
        assert!(FactionRank::Officer.can_promote());
        assert!(FactionRank::Leader.can_promote());
        assert!(FactionRank::Founder.can_promote());
    }

    #[test]
    fn test_faction_rank_next_rank() {
        assert_eq!(FactionRank::Initiate.next_rank(), Some(FactionRank::Member));
        assert_eq!(FactionRank::Member.next_rank(), Some(FactionRank::Veteran));
        assert_eq!(FactionRank::Veteran.next_rank(), Some(FactionRank::Officer));
        assert_eq!(FactionRank::Officer.next_rank(), Some(FactionRank::Leader));
        assert_eq!(FactionRank::Leader.next_rank(), None);
        assert_eq!(FactionRank::Founder.next_rank(), None);
    }

    // ============================================================================
    // Faction Creation Tests
    // ============================================================================

    #[test]
    fn test_create_faction() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Test Faction".to_string(),
                "A test faction".to_string(),
                Polarity::ServiceToOthers,
                Some(1),
            )
            .unwrap();

        assert_eq!(faction.name, "Test Faction");
        assert_eq!(faction.polarity_alignment, Polarity::ServiceToOthers);
        assert_eq!(faction.ray_association, Some(1));
        assert!(faction.has_member(founder_id));
        assert_eq!(faction.member_count(), 1);
    }

    #[test]
    fn test_create_faction_without_ray() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Rayless Faction".to_string(),
                "No ray alignment".to_string(),
                Polarity::Balanced,
                None,
            )
            .unwrap();

        assert_eq!(faction.ray_association, None);
    }

    #[test]
    fn test_founder_membership() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Founder Test".to_string(),
                "Testing founder".to_string(),
                Polarity::ServiceToSelf,
                None,
            )
            .unwrap();

        let membership = system.get_membership(founder_id).unwrap();
        assert_eq!(membership.faction_id, faction.faction_id);
        assert_eq!(membership.rank, FactionRank::Founder);
        assert_eq!(membership.loyalty_score, 1.0);
    }

    // ============================================================================
    // Join/Leave Faction Tests
    // ============================================================================

    #[test]
    fn test_join_faction() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let member_id: EntityId = 2;

        let faction = system
            .create_faction(
                founder_id,
                "Join Test".to_string(),
                "Testing join".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let membership = system.join_faction(member_id, faction.faction_id).unwrap();
        assert_eq!(membership.entity_id, member_id);
        assert_eq!(membership.faction_id, faction.faction_id);
        assert_eq!(membership.rank, FactionRank::Initiate);

        let faction = system.get_faction(faction.faction_id).unwrap();
        assert!(faction.has_member(member_id));
        assert_eq!(faction.member_count(), 2);
    }

    #[test]
    fn test_cannot_join_two_factions() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;
        let member_id: EntityId = 3;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Faction 1".to_string(),
                "First faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Faction 2".to_string(),
                "Second faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system.join_faction(member_id, faction1.faction_id).unwrap();

        let result = system.join_faction(member_id, faction2.faction_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_leave_faction() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let member_id: EntityId = 2;

        let faction = system
            .create_faction(
                founder_id,
                "Leave Test".to_string(),
                "Testing leave".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system.join_faction(member_id, faction.faction_id).unwrap();
        assert!(system.get_membership(member_id).is_some());

        system.leave_faction(member_id, faction.faction_id).unwrap();
        assert!(system.get_membership(member_id).is_none());

        let faction = system.get_faction(faction.faction_id).unwrap();
        assert!(!faction.has_member(member_id));
    }

    #[test]
    fn test_founder_cannot_leave() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Founder Leave Test".to_string(),
                "Testing founder cannot leave".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let result = system.leave_faction(founder_id, faction.faction_id);
        assert!(result.is_err());
    }

    // ============================================================================
    // Promotion Tests
    // ============================================================================

    #[test]
    fn test_promote_member() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let member_id: EntityId = 2;

        let faction = system
            .create_faction(
                founder_id,
                "Promote Test".to_string(),
                "Testing promotion".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system.join_faction(member_id, faction.faction_id).unwrap();

        let new_rank = system
            .promote_member(member_id, faction.faction_id, FactionRank::Member)
            .unwrap();
        assert_eq!(new_rank, FactionRank::Member);

        let membership = system.get_membership(member_id).unwrap();
        assert_eq!(membership.rank, FactionRank::Member);
    }

    #[test]
    fn test_cannot_demote_via_promote() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let member_id: EntityId = 2;

        let faction = system
            .create_faction(
                founder_id,
                "Demote Test".to_string(),
                "Testing cannot demote".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system.join_faction(member_id, faction.faction_id).unwrap();
        system
            .promote_member(member_id, faction.faction_id, FactionRank::Member)
            .unwrap();

        // Try to "promote" to lower rank
        let result = system.promote_member(member_id, faction.faction_id, FactionRank::Initiate);
        assert!(result.is_err());
    }

    // ============================================================================
    // Contribution Tests
    // ============================================================================

    #[test]
    fn test_contribute_to_faction() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Contribution Test".to_string(),
                "Testing contributions".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let contribution = ResonanceContribution::new(
            founder_id,
            ContributionType::ActiveParticipation,
            ResonancePattern::new(),
            0.0,
        );

        system
            .contribute_to_faction(founder_id, faction.faction_id, contribution)
            .unwrap();

        let membership = system.get_membership(founder_id).unwrap();
        assert!(membership.contribution_score > 0.0);
    }

    #[test]
    fn test_non_member_cannot_contribute() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let non_member_id: EntityId = 99;

        let faction = system
            .create_faction(
                founder_id,
                "Non-Member Test".to_string(),
                "Testing non-member contribution".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let contribution = ResonanceContribution::new(
            non_member_id,
            ContributionType::ActiveParticipation,
            ResonancePattern::new(),
            0.0,
        );

        let result = system.contribute_to_faction(non_member_id, faction.faction_id, contribution);
        assert!(result.is_err());
    }

    // ============================================================================
    // Alliance Tests
    // ============================================================================

    #[test]
    fn test_form_alliance() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Ally 1".to_string(),
                "First ally".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Ally 2".to_string(),
                "Second ally".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system
            .form_alliance(faction1.faction_id, faction2.faction_id)
            .unwrap();

        let relationship = system.get_relationship(faction1.faction_id, faction2.faction_id);
        assert_eq!(relationship, RelationshipType::Allied);

        let f1 = system.get_faction(faction1.faction_id).unwrap();
        let f2 = system.get_faction(faction2.faction_id).unwrap();
        assert!(f1.allied_factions.contains(&faction2.faction_id));
        assert!(f2.allied_factions.contains(&faction1.faction_id));
    }

    #[test]
    fn test_cannot_alliance_opposing_polarity() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "STO Faction".to_string(),
                "Service to Others".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "STS Faction".to_string(),
                "Service to Self".to_string(),
                Polarity::ServiceToSelf,
                None,
            )
            .unwrap();

        let result = system.form_alliance(faction1.faction_id, faction2.faction_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_balanced_can_alliance_with_any() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;
        let founder3_id: EntityId = 3;

        let sto_faction = system
            .create_faction(
                founder1_id,
                "STO".to_string(),
                "STO faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let balanced_faction = system
            .create_faction(
                founder2_id,
                "Balanced".to_string(),
                "Balanced faction".to_string(),
                Polarity::Balanced,
                None,
            )
            .unwrap();

        let sts_faction = system
            .create_faction(
                founder3_id,
                "STS".to_string(),
                "STS faction".to_string(),
                Polarity::ServiceToSelf,
                None,
            )
            .unwrap();

        // Balanced can ally with STO
        system
            .form_alliance(balanced_faction.faction_id, sto_faction.faction_id)
            .unwrap();

        // Balanced can ally with STS
        system
            .form_alliance(balanced_faction.faction_id, sts_faction.faction_id)
            .unwrap();
    }

    // ============================================================================
    // Enmity Tests
    // ============================================================================

    #[test]
    fn test_declare_enmity() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Enemy 1".to_string(),
                "First enemy".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Enemy 2".to_string(),
                "Second enemy".to_string(),
                Polarity::ServiceToSelf,
                None,
            )
            .unwrap();

        system
            .declare_enmity(faction1.faction_id, faction2.faction_id)
            .unwrap();

        let relationship = system.get_relationship(faction1.faction_id, faction2.faction_id);
        assert_eq!(relationship, RelationshipType::Hostile);

        let f1 = system.get_faction(faction1.faction_id).unwrap();
        let f2 = system.get_faction(faction2.faction_id).unwrap();
        assert!(f1.enemy_factions.contains(&faction2.faction_id));
        assert!(f2.enemy_factions.contains(&faction1.faction_id));
    }

    #[test]
    fn test_enmity_removes_alliance() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Former Ally 1".to_string(),
                "First former ally".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Former Ally 2".to_string(),
                "Second former ally".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        // First form alliance
        system
            .form_alliance(faction1.faction_id, faction2.faction_id)
            .unwrap();

        // Then declare enmity
        system
            .declare_enmity(faction1.faction_id, faction2.faction_id)
            .unwrap();

        let f1 = system.get_faction(faction1.faction_id).unwrap();
        assert!(!f1.allied_factions.contains(&faction2.faction_id));
        assert!(f1.enemy_factions.contains(&faction2.faction_id));
    }

    // ============================================================================
    // Territory Tests
    // ============================================================================

    #[test]
    fn test_create_territory() {
        let mut system = PolarityFactions::new();

        let territory = system.create_territory("Test Territory".to_string());

        assert_eq!(territory.name, "Test Territory");
        assert!(territory.controlling_faction.is_none());
        assert!(!territory.is_contested());
    }

    #[test]
    fn test_claim_territory() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Territory Test".to_string(),
                "Testing territory".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let territory = system.create_territory("Claimable Land".to_string());

        system
            .claim_territory(faction.faction_id, territory.territory_id)
            .unwrap();

        let territory = system.get_territory(territory.territory_id).unwrap();
        assert_eq!(territory.controlling_faction, Some(faction.faction_id));

        let faction = system.get_faction(faction.faction_id).unwrap();
        assert!(faction.territory_claims.contains(&territory.territory_id));
    }

    #[test]
    fn test_cannot_claim_already_controlled() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Controller".to_string(),
                "Controlling faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Contender".to_string(),
                "Contending faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let territory = system.create_territory("Disputed Land".to_string());

        system
            .claim_territory(faction1.faction_id, territory.territory_id)
            .unwrap();

        let result = system.claim_territory(faction2.faction_id, territory.territory_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_contest_territory() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Controller".to_string(),
                "Controlling faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Contender".to_string(),
                "Contending faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let territory = system.create_territory("Contested Land".to_string());

        system
            .claim_territory(faction1.faction_id, territory.territory_id)
            .unwrap();
        system
            .contest_territory(faction2.faction_id, territory.territory_id)
            .unwrap();

        let territory = system.get_territory(territory.territory_id).unwrap();
        assert!(territory.is_contested());
        assert!(territory.contested_by.contains(&faction2.faction_id));
    }

    // ============================================================================
    // Faction Strength Tests
    // ============================================================================

    #[test]
    fn test_calculate_faction_strength() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Strength Test".to_string(),
                "Testing strength calculation".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let base_strength = system.calculate_faction_strength(faction.faction_id);
        assert!(base_strength > 0.0);

        // Add members
        for i in 2..=5 {
            system.join_faction(i, faction.faction_id).unwrap();
        }

        let with_members = system.calculate_faction_strength(faction.faction_id);
        assert!(with_members > base_strength);

        // Add territory
        let territory = system.create_territory("Power Base".to_string());
        system
            .claim_territory(faction.faction_id, territory.territory_id)
            .unwrap();

        let with_territory = system.calculate_faction_strength(faction.faction_id);
        assert!(with_territory > with_members);
    }

    #[test]
    fn test_faction_level_affects_strength() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Level Test".to_string(),
                "Testing level effects".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let level1_strength = system.calculate_faction_strength(faction.faction_id);

        // Level up the faction
        {
            let f = system.factions.get_mut(&faction.faction_id).unwrap();
            f.level_up();
            f.level_up();
        }

        let level3_strength = system.calculate_faction_strength(faction.faction_id);
        assert!(level3_strength > level1_strength);
    }

    // ============================================================================
    // Quest Tests
    // ============================================================================

    #[test]
    fn test_get_faction_quests() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Quest Test".to_string(),
                "Testing quests".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let quests = system.get_faction_quests(faction.faction_id);
        assert!(!quests.is_empty());

        // Add territory for more quests
        let territory = system.create_territory("Quest Hub".to_string());
        system
            .claim_territory(faction.faction_id, territory.territory_id)
            .unwrap();

        let quests_with_territory = system.get_faction_quests(faction.faction_id);
        assert!(quests_with_territory.len() > quests.len());
    }

    // ============================================================================
    // Trade Benefits Tests
    // ============================================================================

    #[test]
    fn test_get_faction_trade_benefits() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Trade Test".to_string(),
                "Testing trade benefits".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let benefits = system.get_faction_trade_benefits(faction.faction_id);
        assert!(benefits.faction_discount >= 0.0);
        assert!(benefits.resonance_bonus >= 0.0);
        assert!(benefits.catalyst_bonus >= 0.0);
    }

    #[test]
    fn test_trade_benefits_by_rank() {
        let benefits_initiate = TradeBenefits::for_rank(&FactionRank::Initiate);
        let benefits_leader = TradeBenefits::for_rank(&FactionRank::Leader);

        assert!(benefits_leader.faction_discount > benefits_initiate.faction_discount);
        assert!(benefits_leader.resonance_bonus > benefits_initiate.resonance_bonus);
        assert!(benefits_leader.exclusive_items.len() > benefits_initiate.exclusive_items.len());
    }

    // ============================================================================
    // Faction Capacity Tests
    // ============================================================================

    #[test]
    fn test_faction_capacity() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Capacity Test".to_string(),
                "Testing capacity limits".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        // Level 1 faction has capacity of 10
        let f = system.get_faction(faction.faction_id).unwrap();
        assert_eq!(f.member_capacity(), 10);
        assert!(f.can_accept_members());

        // Fill to capacity
        for i in 2..=10 {
            system.join_faction(i, faction.faction_id).unwrap();
        }

        let f = system.get_faction(faction.faction_id).unwrap();
        assert_eq!(f.member_count(), 10);
        assert!(!f.can_accept_members());

        // Try to add one more
        let result = system.join_faction(11, faction.faction_id);
        assert!(result.is_err());
    }

    // ============================================================================
    // Resonance Tests
    // ============================================================================

    #[test]
    fn test_faction_resonance_calculation() {
        let base_pattern = ResonancePattern::new();
        let mut resonance = FactionResonance::new(base_pattern);

        assert_eq!(resonance.collective_strength, 0.0);
        assert_eq!(resonance.resonance_stability, 1.0);

        // Add contributions
        for i in 1..=5 {
            let contribution = ResonanceContribution::new(
                i,
                ContributionType::ResonanceHarmonization,
                ResonancePattern::new(),
                0.0,
            );
            resonance.add_member_contribution(i, contribution).unwrap();
        }

        assert!(resonance.collective_strength > 0.0);
        assert!(resonance.get_influence_radius() > 0.0);
    }

    #[test]
    fn test_faction_resonance_remove_member() {
        let base_pattern = ResonancePattern::new();
        let mut resonance = FactionResonance::new(base_pattern);

        let contribution = ResonanceContribution::new(
            1,
            ContributionType::ActiveParticipation,
            ResonancePattern::new(),
            0.0,
        );
        resonance.add_member_contribution(1, contribution).unwrap();

        let strength_with_member = resonance.collective_strength;

        resonance.remove_member_contribution(1).unwrap();

        assert!(resonance.collective_strength < strength_with_member);
        assert!(!resonance.member_contributions.contains_key(&1));
    }

    // ============================================================================
    // Membership Query Tests
    // ============================================================================

    #[test]
    fn test_get_entity_faction() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;
        let member_id: EntityId = 2;
        let non_member_id: EntityId = 99;

        let faction = system
            .create_faction(
                founder_id,
                "Query Test".to_string(),
                "Testing queries".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        system.join_faction(member_id, faction.faction_id).unwrap();

        assert_eq!(
            system.get_entity_faction(founder_id),
            Some(faction.faction_id)
        );
        assert_eq!(
            system.get_entity_faction(member_id),
            Some(faction.faction_id)
        );
        assert_eq!(system.get_entity_faction(non_member_id), None);
    }

    #[test]
    fn test_get_available_factions() {
        let mut system = PolarityFactions::new();
        let founder1_id: EntityId = 1;
        let founder2_id: EntityId = 2;
        let unaffiliated_id: EntityId = 99;

        let faction1 = system
            .create_faction(
                founder1_id,
                "Faction 1".to_string(),
                "First faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        let faction2 = system
            .create_faction(
                founder2_id,
                "Faction 2".to_string(),
                "Second faction".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        // Unaffiliated entity can join any faction
        let available = system.get_available_factions(unaffiliated_id);
        assert_eq!(available.len(), 2);
        assert!(available.contains(&faction1.faction_id));
        assert!(available.contains(&faction2.faction_id));

        // Member cannot join other factions
        let member_available = system.get_available_factions(founder1_id);
        assert!(member_available.is_empty());
    }

    // ============================================================================
    // Catalyst Tests
    // ============================================================================

    #[test]
    fn test_faction_catalyst() {
        let mut system = PolarityFactions::new();
        let founder_id: EntityId = 1;

        let faction = system
            .create_faction(
                founder_id,
                "Catalyst Test".to_string(),
                "Testing catalyst".to_string(),
                Polarity::ServiceToOthers,
                None,
            )
            .unwrap();

        // Add catalyst
        {
            let f = system.factions.get_mut(&faction.faction_id).unwrap();
            f.add_catalyst(100.0);
            assert_eq!(f.collective_catalyst, 100.0);
        }

        // Spend catalyst
        {
            let f = system.factions.get_mut(&faction.faction_id).unwrap();
            f.spend_catalyst(30.0).unwrap();
            assert_eq!(f.collective_catalyst, 70.0);
        }

        // Try to spend too much
        {
            let f = system.factions.get_mut(&faction.faction_id).unwrap();
            let result = f.spend_catalyst(100.0);
            assert!(result.is_err());
        }
    }

    // ============================================================================
    // Territory Catalyst Generation Tests
    // ============================================================================

    #[test]
    fn test_territory_catalyst_generation() {
        let mut territory = Territory::new(TerritoryId::new(1), "Rich Land".to_string());
        territory.strategic_value = 1.0;
        territory.catalyst_generation_rate = 10.0;

        let generation = territory.current_catalyst_generation();
        assert_eq!(generation, 10.0); // 10.0 * (0.5 + 1.0 * 0.5) = 10.0

        territory.strategic_value = 0.0;
        let low_generation = territory.current_catalyst_generation();
        assert_eq!(low_generation, 5.0); // 10.0 * (0.5 + 0.0 * 0.5) = 5.0
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_full_faction_lifecycle() {
        let mut system = PolarityFactions::new();

        // Create founder and faction
        let founder_id: EntityId = 1;
        let faction = system
            .create_faction(
                founder_id,
                "Lifecycle Faction".to_string(),
                "Testing full lifecycle".to_string(),
                Polarity::ServiceToOthers,
                Some(3),
            )
            .unwrap();

        // Add members
        let member_ids: Vec<EntityId> = (2..=5).collect();
        for &id in &member_ids {
            system.join_faction(id, faction.faction_id).unwrap();
        }

        // Promote some members
        system
            .promote_member(member_ids[0], faction.faction_id, FactionRank::Member)
            .unwrap();
        system
            .promote_member(member_ids[1], faction.faction_id, FactionRank::Veteran)
            .unwrap();

        // Create and claim territory
        let territory = system.create_territory("Home Base".to_string());
        system
            .claim_territory(faction.faction_id, territory.territory_id)
            .unwrap();

        // Add contributions
        for &id in &member_ids {
            let contribution = ResonanceContribution::new(
                id,
                ContributionType::ActiveParticipation,
                ResonancePattern::new(),
                system.current_time(),
            );
            system
                .contribute_to_faction(id, faction.faction_id, contribution)
                .unwrap();
        }

        // Verify final state
        let faction = system.get_faction(faction.faction_id).unwrap();
        assert_eq!(faction.member_count(), 5);
        assert_eq!(faction.territory_claims.len(), 1);

        let strength = system.calculate_faction_strength(faction.faction_id);
        assert!(strength > 0.0);

        let quests = system.get_faction_quests(faction.faction_id);
        assert!(!quests.is_empty());
    }

    #[test]
    fn test_faction_war_scenario() {
        let mut system = PolarityFactions::new();

        // Create two opposing factions
        let sto_founder: EntityId = 1;
        let sts_founder: EntityId = 2;

        let sto_faction = system
            .create_faction(
                sto_founder,
                "The Lightbringers".to_string(),
                "Service to Others".to_string(),
                Polarity::ServiceToOthers,
                Some(4), // Green ray - love/understanding
            )
            .unwrap();

        let sts_faction = system
            .create_faction(
                sts_founder,
                "The Dominators".to_string(),
                "Service to Self".to_string(),
                Polarity::ServiceToSelf,
                Some(1), // Red ray - will/power
            )
            .unwrap();

        // Declare war
        system
            .declare_enmity(sto_faction.faction_id, sts_faction.faction_id)
            .unwrap();

        // Create contested territory
        let territory = system.create_territory("The Disputed Valley".to_string());
        territory.strategic_value = 0.8;

        // STO claims first
        system
            .claim_territory(sto_faction.faction_id, territory.territory_id)
            .unwrap();

        // STS contests
        system
            .contest_territory(sts_faction.faction_id, territory.territory_id)
            .unwrap();

        // Verify state
        let territory = system.get_territory(territory.territory_id).unwrap();
        assert!(territory.is_contested());
        assert_eq!(territory.controlling_faction, Some(sto_faction.faction_id));
        assert!(territory.contested_by.contains(&sts_faction.faction_id));

        let relationship = system.get_relationship(sto_faction.faction_id, sts_faction.faction_id);
        assert_eq!(relationship, RelationshipType::Hostile);
    }
}

// Helper trait for error conversion
impl From<FactionError> for AdvancedGameMechanicsError {
    fn from(error: FactionError) -> Self {
        AdvancedGameMechanicsError::InvalidOperation(error.to_string())
    }
}

// Implement From for FactionError to include InvalidOperation variant
impl FactionError {
    fn into_advanced_error(self) -> AdvancedGameMechanicsError {
        AdvancedGameMechanicsError::InvalidOperation(self.to_string())
    }
}

// Add InvalidOperation variant helper
impl FactionError {
    fn invalid_operation(message: &str) -> Self {
        FactionError::InvalidOperation {
            message: message.to_string(),
        }
    }
}

// Add missing InvalidOperation variant to FactionError
impl FactionError {
    const fn invalid_op(message: String) -> Self {
        FactionError::InvalidOperation { message }
    }
}

// Add InvalidOperation to FactionError enum
// This is handled via the conversion trait above
