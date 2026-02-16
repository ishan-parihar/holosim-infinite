//! Advanced Game Mechanics Module
//!
//! Phase 6, Week 93-96: Advanced Game Mechanics
//!
//! This module implements five interconnected game mechanics that operate on holographic principles:
//!
//! 1. **Resonance-Based Trading**: Trading system based on holographic resonance between entities
//!    - From COSMOLOGICAL-ARCHITECTURE.md: "Each entity contains the whole"
//!    - Trades occur through resonance matching, not currency exchange
//!    - Value determined by archetypical compatibility and density alignment
//!
//! 2. **Catalyst-Based Combat**: Combat system using catalyst as the primary resource
//!    - From COSMOLOGICAL-ARCHITECTURE.md: "Catalyst creates the opportunity for choice"
//!    - Combat is about polarization struggle, not physical destruction
//!    - Victory determined by polarization strength and archetype activation
//!
//! 3. **Archetypical Quests**: Quest system based on the 22 archetypes
//!    - From COSMOLOGICAL-ARCHITECTURE.md: "The archetypical mind provides the framework for evolution"
//!    - Quests emerge from archetype activation patterns
//!    - Completion advances entity through density levels
//!
//! 4. **Polarity Factions**: Faction system based on STO/STS polarization
//!    - From COSMOLOGICAL-ARCHITECTURE.md: "Archetype 22 (The Choice): Creates polarity"
//!    - Factions form around polarization orientation
//!    - Faction standing affects trading, combat, and quest availability
//!
//! 5. **Resonance Economy**: Economic system based on holographic resonance
//!    - From COSMOLOGICAL-ARCHITECTURE.md: "The whole is more than the sum of parts"
//!    - Value flows through resonance networks
//!    - Economic health tied to collective consciousness coherence
//!
//! ## Holographic Principles
//!
//! All five mechanics operate on holographic principles:
//! - **Each contains the whole**: Every trade/combat/quest/faction/economic interaction
//!   contains the complete archetypical pattern
//! - **Non-local correlation**: Interactions are not limited by physical distance
//! - **Resonance-based**: Compatibility and value determined by frequency matching
//! - **Emergent complexity**: Simple rules create complex emergent behavior
//!
//! ## Integration with Existing Systems
//!
//! This module integrates with:
//! - `entity_layer7::layer7` - Entity identification and state
//! - `simulation_v3::holographic_inventory` - Item management
//! - `simulation_v3::catalyst_system` - Catalyst generation and consumption
//! - `simulation_v3::collective_system` - Collective consciousness effects
//! - `evolution_density_octave` - Density progression
//! - `consciousness` - Free Will and polarization
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 9:
//! > "Advanced Game Mechanics: Trading, combat, quests, factions, economy"
//! > "All mechanics operate on holographic principles - resonance, not proximity"

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::density_mechanics::{Density, Polarity};
use crate::simulation_v3::holographic_inventory::ItemId;
use crate::types::Float;

// Re-export all submodules
pub mod archetypical_quests; // TODO: Week 95a
pub mod catalyst_combat;
pub mod polarity_factions;
pub mod resonance_economy;
pub mod resonance_trading; // Week 95b: Factions as Polarity Alignment // Week 96: Economy as Resonance Flow

// ============================================================================
// SHARED TYPE ALIASES
// ============================================================================

/// Timestamp for game events (in simulation time units)
pub type Timestamp = Float;

/// Resonance compatibility between entities (0.0 = incompatible, 1.0 = perfectly compatible)
pub type ResonanceCompatibility = Float;

/// Amount of catalyst energy (used in combat and other mechanics)
pub type CatalystAmount = Float;

// ============================================================================
// NEWTYPE WRAPPERS FOR IDs
// ============================================================================

/// Unique identifier for a trade transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TradeId(pub u64);

impl TradeId {
    /// Create a new TradeId from a u64 value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TradeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trade-{}", self.0)
    }
}

/// Unique identifier for a combat encounter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CombatId(pub u64);

impl CombatId {
    /// Create a new CombatId from a u64 value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for CombatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Combat-{}", self.0)
    }
}

/// Unique identifier for a quest
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuestId(pub u64);

impl QuestId {
    /// Create a new QuestId from a u64 value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for QuestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quest-{}", self.0)
    }
}

/// Unique identifier for a faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FactionId(pub u64);

impl FactionId {
    /// Create a new FactionId from a u64 value
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the underlying u64 value
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for FactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Faction-{}", self.0)
    }
}

// ============================================================================
// ARCHETYPE ID TYPE
// ============================================================================

/// Archetype identifier (1-22) representing one of the 22 archetypes
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The Archetypical Mind: 22 archetypes organized into three complexes
/// (Mind, Body, Spirit) plus Archetype 22 (The Choice)"
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeId(pub u8);

impl ArchetypeId {
    /// Archetype 1: The Magician (Matrix of the Mind)
    pub const A1_MAGICIAN: ArchetypeId = ArchetypeId(1);
    /// Archetype 2: The High Priestess (Potentiator of the Mind)
    pub const A2_HIGH_PRIESTESS: ArchetypeId = ArchetypeId(2);
    /// Archetype 3: The Empress (Catalyst of the Mind)
    pub const A3_EMPRESS: ArchetypeId = ArchetypeId(3);
    /// Archetype 4: The Emperor (Experience of the Mind)
    pub const A4_EMPEROR: ArchetypeId = ArchetypeId(4);
    /// Archetype 5: The Hierophant (Significator of the Mind)
    pub const A5_HIEROPHANT: ArchetypeId = ArchetypeId(5);
    /// Archetype 6: The Lovers (Transformation of the Mind)
    pub const A6_LOVERS: ArchetypeId = ArchetypeId(6);
    /// Archetype 7: The Chariot (Great Way of the Mind)
    pub const A7_CHARIOT: ArchetypeId = ArchetypeId(7);
    /// Archetype 8: Strength (Matrix of the Body)
    pub const A8_STRENGTH: ArchetypeId = ArchetypeId(8);
    /// Archetype 9: The Hermit (Potentiator of the Body)
    pub const A9_HERMIT: ArchetypeId = ArchetypeId(9);
    /// Archetype 10: Wheel of Fortune (Catalyst of the Body)
    pub const A10_WHEEL: ArchetypeId = ArchetypeId(10);
    /// Archetype 11: Justice (Experience of the Body)
    pub const A11_JUSTICE: ArchetypeId = ArchetypeId(11);
    /// Archetype 12: The Hanged Man (Significator of the Body)
    pub const A12_HANGED_MAN: ArchetypeId = ArchetypeId(12);
    /// Archetype 13: Death (Transformation of the Body)
    pub const A13_DEATH: ArchetypeId = ArchetypeId(13);
    /// Archetype 14: Temperance (Great Way of the Body)
    pub const A14_TEMPERANCE: ArchetypeId = ArchetypeId(14);
    /// Archetype 15: The Devil (Matrix of the Spirit)
    pub const A15_DEVIL: ArchetypeId = ArchetypeId(15);
    /// Archetype 16: The Tower (Potentiator of the Spirit)
    pub const A16_TOWER: ArchetypeId = ArchetypeId(16);
    /// Archetype 17: The Star (Catalyst of the Spirit)
    pub const A17_STAR: ArchetypeId = ArchetypeId(17);
    /// Archetype 18: The Moon (Experience of the Spirit)
    pub const A18_MOON: ArchetypeId = ArchetypeId(18);
    /// Archetype 19: The Sun (Significator of the Spirit)
    pub const A19_SUN: ArchetypeId = ArchetypeId(19);
    /// Archetype 20: Judgement (Transformation of the Spirit)
    pub const A20_JUDGEMENT: ArchetypeId = ArchetypeId(20);
    /// Archetype 21: The World (Great Way of the Spirit)
    pub const A21_WORLD: ArchetypeId = ArchetypeId(21);
    /// Archetype 22: The Choice (Free Will)
    pub const A22_CHOICE: ArchetypeId = ArchetypeId(22);

    /// Create a new ArchetypeId, clamped to valid range (1-22)
    pub fn new(value: u8) -> Self {
        Self(value.clamp(1, 22))
    }

    /// Get the archetype value as u8
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get the archetype as usize for array indexing (0-21)
    pub fn as_index(&self) -> usize {
        (self.0 - 1) as usize
    }

    /// Get the complex this archetype belongs to
    pub fn complex(&self) -> ArchetypeComplex {
        match self.0 {
            1..=7 => ArchetypeComplex::Mind,
            8..=14 => ArchetypeComplex::Body,
            15..=21 => ArchetypeComplex::Spirit,
            22 => ArchetypeComplex::Choice,
            _ => ArchetypeComplex::Mind,
        }
    }

    /// Check if this is Archetype 22 (The Choice)
    pub fn is_choice(&self) -> bool {
        self.0 == 22
    }

    /// Get the display name for this archetype
    pub fn display_name(&self) -> &'static str {
        match self.0 {
            1 => "The Magician",
            2 => "The High Priestess",
            3 => "The Empress",
            4 => "The Emperor",
            5 => "The Hierophant",
            6 => "The Lovers",
            7 => "The Chariot",
            8 => "Strength",
            9 => "The Hermit",
            10 => "Wheel of Fortune",
            11 => "Justice",
            12 => "The Hanged Man",
            13 => "Death",
            14 => "Temperance",
            15 => "The Devil",
            16 => "The Tower",
            17 => "The Star",
            18 => "The Moon",
            19 => "The Sun",
            20 => "Judgement",
            21 => "The World",
            22 => "The Choice",
            _ => "Unknown Archetype",
        }
    }
}

impl Default for ArchetypeId {
    fn default() -> Self {
        ArchetypeId::A1_MAGICIAN
    }
}

impl std::fmt::Display for ArchetypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}: {}", self.0, self.display_name())
    }
}

/// The three archetype complexes plus The Choice
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchetypeComplex {
    /// Mind Complex: Archetypes 1-7
    Mind,
    /// Body Complex: Archetypes 8-14
    Body,
    /// Spirit Complex: Archetypes 15-21
    Spirit,
    /// The Choice: Archetype 22
    Choice,
}

impl ArchetypeComplex {
    /// Get all archetypes in this complex
    pub fn archetypes(&self) -> Vec<ArchetypeId> {
        match self {
            ArchetypeComplex::Mind => (1..=7).map(ArchetypeId::new).collect(),
            ArchetypeComplex::Body => (8..=14).map(ArchetypeId::new).collect(),
            ArchetypeComplex::Spirit => (15..=21).map(ArchetypeId::new).collect(),
            ArchetypeComplex::Choice => vec![ArchetypeId::A22_CHOICE],
        }
    }

    /// Get the display name for this complex
    pub fn display_name(&self) -> &'static str {
        match self {
            ArchetypeComplex::Mind => "Mind Complex",
            ArchetypeComplex::Body => "Body Complex",
            ArchetypeComplex::Spirit => "Spirit Complex",
            ArchetypeComplex::Choice => "The Choice",
        }
    }
}

impl std::fmt::Display for ArchetypeComplex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Error type for the Advanced Game Mechanics module
///
/// This enum contains all possible errors that can occur across the five
/// game mechanic subsystems: trading, combat, quests, factions, and economy.
#[derive(Debug, Clone, PartialEq)]
pub enum AdvancedGameMechanicsError {
    // Trading errors
    /// Trade not found
    TradeNotFound(TradeId),
    /// Invalid trade offer
    InvalidTradeOffer(String),
    /// Insufficient resonance for trade
    InsufficientResonanceForTrade {
        /// Current resonance level
        current: Float,
        /// Required resonance level
        required: Float,
    },
    /// Trade partner not available
    TradePartnerNotAvailable(EntityId),

    // Combat errors
    /// Combat not found
    CombatNotFound(CombatId),
    /// Invalid combat action
    InvalidCombatAction(String),
    /// Insufficient catalyst for combat
    InsufficientCatalyst {
        /// Current catalyst amount
        current: CatalystAmount,
        /// Required catalyst amount
        required: CatalystAmount,
    },
    /// Combatant not found
    CombatantNotFound(EntityId),
    /// Combat already concluded
    CombatAlreadyConcluded(CombatId),

    // Quest errors
    /// Quest not found
    QuestNotFound(QuestId),
    /// Quest not available for entity
    QuestNotAvailable {
        /// The quest ID
        quest_id: QuestId,
        /// The entity ID
        entity_id: EntityId,
        /// Reason why quest is not available
        reason: String,
    },
    /// Quest already completed
    QuestAlreadyCompleted(QuestId),
    /// Quest prerequisite not met
    QuestPrerequisiteNotMet {
        /// The quest ID
        quest_id: QuestId,
        /// The missing prerequisite
        prerequisite: String,
    },
    /// Invalid quest objective
    InvalidQuestObjective(String),

    // Faction errors
    /// Faction not found
    FactionNotFound(FactionId),
    /// Entity not in faction
    EntityNotInFaction {
        /// The entity ID
        entity_id: EntityId,
        /// The faction ID
        faction_id: FactionId,
    },
    /// Faction standing too low
    InsufficientFactionStanding {
        /// Current standing
        current: Float,
        /// Required standing
        required: Float,
    },
    /// Polarization mismatch with faction
    PolarizationMismatch {
        /// Entity's polarity
        entity_polarity: Polarity,
        /// Faction's required polarity
        faction_polarity: Polarity,
    },

    // Economy errors
    /// Economic calculation error
    EconomicCalculationError(String),
    /// Resonance network error
    ResonanceNetworkError(String),
    /// Market not found
    MarketNotFound(String),
    /// Insufficient economic capacity
    InsufficientEconomicCapacity {
        /// Current capacity
        current: Float,
        /// Required capacity
        required: Float,
    },

    // General errors
    /// Entity not found
    EntityNotFound(EntityId),
    /// Item not found
    ItemNotFound(ItemId),
    /// Invalid operation
    InvalidOperation(String),
    /// Not implemented
    NotImplemented(String),
    /// Internal error
    InternalError(String),
}

impl std::fmt::Display for AdvancedGameMechanicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Trading errors
            AdvancedGameMechanicsError::TradeNotFound(id) => {
                write!(f, "Trade not found: {}", id)
            }
            AdvancedGameMechanicsError::InvalidTradeOffer(reason) => {
                write!(f, "Invalid trade offer: {}", reason)
            }
            AdvancedGameMechanicsError::InsufficientResonanceForTrade { current, required } => {
                write!(
                    f,
                    "Insufficient resonance for trade: current={}, required={}",
                    current, required
                )
            }
            AdvancedGameMechanicsError::TradePartnerNotAvailable(id) => {
                write!(f, "Trade partner not available: {:?}", id)
            }

            // Combat errors
            AdvancedGameMechanicsError::CombatNotFound(id) => {
                write!(f, "Combat not found: {}", id)
            }
            AdvancedGameMechanicsError::InvalidCombatAction(reason) => {
                write!(f, "Invalid combat action: {}", reason)
            }
            AdvancedGameMechanicsError::InsufficientCatalyst { current, required } => {
                write!(
                    f,
                    "Insufficient catalyst: current={}, required={}",
                    current, required
                )
            }
            AdvancedGameMechanicsError::CombatantNotFound(id) => {
                write!(f, "Combatant not found: {:?}", id)
            }
            AdvancedGameMechanicsError::CombatAlreadyConcluded(id) => {
                write!(f, "Combat already concluded: {}", id)
            }

            // Quest errors
            AdvancedGameMechanicsError::QuestNotFound(id) => {
                write!(f, "Quest not found: {}", id)
            }
            AdvancedGameMechanicsError::QuestNotAvailable {
                quest_id,
                entity_id,
                reason,
            } => {
                write!(
                    f,
                    "Quest {} not available for entity {:?}: {}",
                    quest_id, entity_id, reason
                )
            }
            AdvancedGameMechanicsError::QuestAlreadyCompleted(id) => {
                write!(f, "Quest already completed: {}", id)
            }
            AdvancedGameMechanicsError::QuestPrerequisiteNotMet {
                quest_id,
                prerequisite,
            } => {
                write!(
                    f,
                    "Quest {} prerequisite not met: {}",
                    quest_id, prerequisite
                )
            }
            AdvancedGameMechanicsError::InvalidQuestObjective(reason) => {
                write!(f, "Invalid quest objective: {}", reason)
            }

            // Faction errors
            AdvancedGameMechanicsError::FactionNotFound(id) => {
                write!(f, "Faction not found: {}", id)
            }
            AdvancedGameMechanicsError::EntityNotInFaction {
                entity_id,
                faction_id,
            } => {
                write!(f, "Entity {:?} not in faction {}", entity_id, faction_id)
            }
            AdvancedGameMechanicsError::InsufficientFactionStanding { current, required } => {
                write!(
                    f,
                    "Insufficient faction standing: current={}, required={}",
                    current, required
                )
            }
            AdvancedGameMechanicsError::PolarizationMismatch {
                entity_polarity,
                faction_polarity,
            } => {
                write!(
                    f,
                    "Polarization mismatch: entity={}, faction={}",
                    entity_polarity, faction_polarity
                )
            }

            // Economy errors
            AdvancedGameMechanicsError::EconomicCalculationError(reason) => {
                write!(f, "Economic calculation error: {}", reason)
            }
            AdvancedGameMechanicsError::ResonanceNetworkError(reason) => {
                write!(f, "Resonance network error: {}", reason)
            }
            AdvancedGameMechanicsError::MarketNotFound(market) => {
                write!(f, "Market not found: {}", market)
            }
            AdvancedGameMechanicsError::InsufficientEconomicCapacity { current, required } => {
                write!(
                    f,
                    "Insufficient economic capacity: current={}, required={}",
                    current, required
                )
            }

            // General errors
            AdvancedGameMechanicsError::EntityNotFound(id) => {
                write!(f, "Entity not found: {:?}", id)
            }
            AdvancedGameMechanicsError::ItemNotFound(id) => {
                write!(f, "Item not found: {:?}", id)
            }
            AdvancedGameMechanicsError::InvalidOperation(reason) => {
                write!(f, "Invalid operation: {}", reason)
            }
            AdvancedGameMechanicsError::NotImplemented(feature) => {
                write!(f, "Not implemented: {}", feature)
            }
            AdvancedGameMechanicsError::InternalError(reason) => {
                write!(f, "Internal error: {}", reason)
            }
        }
    }
}

impl std::error::Error for AdvancedGameMechanicsError {}

// ============================================================================
// RESULT TYPE
// ============================================================================

/// Result type for Advanced Game Mechanics operations
pub type Result<T> = std::result::Result<T, AdvancedGameMechanicsError>;

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Calculate resonance compatibility between two entities based on their densities
/// and polarities
///
/// Returns a value between 0.0 (incompatible) and 1.0 (perfectly compatible)
pub fn calculate_resonance_compatibility(
    density1: Density,
    polarity1: Polarity,
    density2: Density,
    polarity2: Polarity,
) -> ResonanceCompatibility {
    // Density compatibility: closer densities = higher compatibility
    let density_diff = (density1.value() as i16 - density2.value() as i16).abs() as Float;
    let density_compatibility = 1.0 - (density_diff / 7.0).min(1.0);

    // Polarity compatibility
    let polarity_compatibility = polarity1.compatibility_with(&polarity2);

    // Combined compatibility (weighted average)
    (density_compatibility * 0.4 + polarity_compatibility * 0.6).clamp(0.0, 1.0)
}

/// Check if an entity meets the requirements for a given density level
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Polarity is required for density progression beyond 3rd density"
pub fn meets_density_requirements(
    current_density: Density,
    target_density: Density,
    polarization_strength: Float,
    polarization: Polarity,
) -> bool {
    if target_density <= current_density {
        return true;
    }

    // Check polarization requirements for 3rd density and above
    if target_density.value() >= 3 {
        let required_polarization = match polarization {
            Polarity::ServiceToOthers => 0.51, // 51% for STO
            Polarity::ServiceToSelf => 0.95,   // 95% for STS
            Polarity::Balanced | Polarity::Undecided => return false, // Balanced/Undecided cannot progress beyond 3rd
        };

        if polarization_strength < required_polarization {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_id_creation() {
        let id = TradeId::new(42);
        assert_eq!(id.as_u64(), 42);
        assert_eq!(id.to_string(), "Trade-42");
    }

    #[test]
    fn test_combat_id_creation() {
        let id = CombatId::new(100);
        assert_eq!(id.as_u64(), 100);
        assert_eq!(id.to_string(), "Combat-100");
    }

    #[test]
    fn test_quest_id_creation() {
        let id = QuestId::new(1);
        assert_eq!(id.as_u64(), 1);
        assert_eq!(id.to_string(), "Quest-1");
    }

    #[test]
    fn test_faction_id_creation() {
        let id = FactionId::new(5);
        assert_eq!(id.as_u64(), 5);
        assert_eq!(id.to_string(), "Faction-5");
    }

    #[test]
    fn test_polarity_compatibility() {
        assert!(Polarity::ServiceToOthers.is_compatible_with(&Polarity::ServiceToOthers));
        assert!(Polarity::ServiceToSelf.is_compatible_with(&Polarity::ServiceToSelf));
        assert!(!Polarity::ServiceToOthers.is_compatible_with(&Polarity::ServiceToSelf));
        assert!(Polarity::Balanced.is_compatible_with(&Polarity::ServiceToOthers));
        assert!(Polarity::Balanced.is_compatible_with(&Polarity::ServiceToSelf));
    }

    #[test]
    fn test_polarity_as_f64() {
        assert_eq!(Polarity::ServiceToOthers.as_f64(), 1.0);
        assert_eq!(Polarity::ServiceToSelf.as_f64(), -1.0);
        assert_eq!(Polarity::Balanced.as_f64(), 0.0);
    }

    #[test]
    fn test_density_creation() {
        let d = Density::new(5);
        assert_eq!(d.value(), 5);
        assert_eq!(d.display_name(), "Fifth Density");
    }

    #[test]
    fn test_density_clamping() {
        let d_low = Density::new(0);
        assert_eq!(d_low.value(), 1);

        let d_high = Density::new(10);
        assert_eq!(d_high.value(), 8);
    }

    #[test]
    fn test_archetype_id_creation() {
        let a = ArchetypeId::new(22);
        assert_eq!(a.value(), 22);
        assert!(a.is_choice());
        assert_eq!(a.display_name(), "The Choice");
    }

    #[test]
    fn test_archetype_id_clamping() {
        let a_low = ArchetypeId::new(0);
        assert_eq!(a_low.value(), 1);

        let a_high = ArchetypeId::new(25);
        assert_eq!(a_high.value(), 22);
    }

    #[test]
    fn test_archetype_complex() {
        assert_eq!(ArchetypeId::A1_MAGICIAN.complex(), ArchetypeComplex::Mind);
        assert_eq!(ArchetypeId::A8_STRENGTH.complex(), ArchetypeComplex::Body);
        assert_eq!(ArchetypeId::A15_DEVIL.complex(), ArchetypeComplex::Spirit);
        assert_eq!(ArchetypeId::A22_CHOICE.complex(), ArchetypeComplex::Choice);
    }

    #[test]
    fn test_archetype_complex_archetypes() {
        let mind_archetypes = ArchetypeComplex::Mind.archetypes();
        assert_eq!(mind_archetypes.len(), 7);
        assert_eq!(mind_archetypes[0].value(), 1);
        assert_eq!(mind_archetypes[6].value(), 7);
    }

    #[test]
    fn test_resonance_compatibility() {
        // Same density and polarity = high compatibility
        let compat1 = calculate_resonance_compatibility(
            Density::THIRD,
            Polarity::ServiceToOthers,
            Density::THIRD,
            Polarity::ServiceToOthers,
        );
        assert!(compat1 > 0.9);

        // Same density, different polarity = low compatibility
        let compat2 = calculate_resonance_compatibility(
            Density::THIRD,
            Polarity::ServiceToOthers,
            Density::THIRD,
            Polarity::ServiceToSelf,
        );
        assert!(compat2 < 0.5);

        // Different density, same polarity = high compatibility
        // (polarity is weighted 60%, so same polarity drives compatibility up)
        let compat3 = calculate_resonance_compatibility(
            Density::THIRD,
            Polarity::ServiceToOthers,
            Density::FOURTH,
            Polarity::ServiceToOthers,
        );
        assert!(compat3 > 0.8 && compat3 <= 1.0);
    }

    #[test]
    fn test_meets_density_requirements() {
        // Progression within same density
        assert!(meets_density_requirements(
            Density::THIRD,
            Density::THIRD,
            0.0,
            Polarity::Balanced
        ));

        // STO with 51% polarization can progress to 4th
        assert!(meets_density_requirements(
            Density::THIRD,
            Density::FOURTH,
            0.51,
            Polarity::ServiceToOthers
        ));

        // STO with 50% polarization cannot progress to 4th
        assert!(!meets_density_requirements(
            Density::THIRD,
            Density::FOURTH,
            0.50,
            Polarity::ServiceToOthers
        ));

        // STS with 95% polarization can progress to 4th
        assert!(meets_density_requirements(
            Density::THIRD,
            Density::FOURTH,
            0.95,
            Polarity::ServiceToSelf
        ));

        // STS with 94% polarization cannot progress to 4th
        assert!(!meets_density_requirements(
            Density::THIRD,
            Density::FOURTH,
            0.94,
            Polarity::ServiceToSelf
        ));

        // Balanced cannot progress beyond 3rd
        assert!(!meets_density_requirements(
            Density::THIRD,
            Density::FOURTH,
            1.0,
            Polarity::Balanced
        ));
    }

    #[test]
    fn test_error_display() {
        let err = AdvancedGameMechanicsError::TradeNotFound(TradeId::new(1));
        assert_eq!(err.to_string(), "Trade not found: Trade-1");

        let err = AdvancedGameMechanicsError::InsufficientCatalyst {
            current: 10.0,
            required: 20.0,
        };
        assert_eq!(
            err.to_string(),
            "Insufficient catalyst: current=10, required=20"
        );
    }
}
