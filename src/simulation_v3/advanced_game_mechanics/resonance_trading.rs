//! Week 93: Trading as Resonance-Based Exchange
//!
//! This module implements trading where items are exchanged based on resonance pattern matching,
//! not monetary value. From COSMOLOGICAL-ARCHITECTURE.md: "Each entity contains the whole"
//!
//! ## Core Principles
//!
//! 1. **Resonance-Based Trading**: Items are exchanged based on resonance pattern matching,
//!    not monetary value. Value is determined by archetypical compatibility and density alignment.
//!
//! 2. **Holographic Exchange**: Every trade contains the complete archetypical pattern.
//!    The trade itself is a holographic interaction where both parties' resonance patterns
//!    must be compatible for the exchange to occur.
//!
//! 3. **Multi-Party Trading**: Collective trades where multiple entities pool items and
//!    distribute them based on collective resonance patterns.
//!
//! 4. **Temporal Dynamics**: Trade offers expire after a certain time, creating urgency
//!    and encouraging timely resonance-based decisions.
//!
//! ## Trading Flow
//!
//! 1. Entity creates a `TradeOffer` with offered items, requested items, and resonance requirements
//! 2. Other entities can view offers and check resonance compatibility
//! 3. Compatible entities can accept the trade, transferring items
//! 4. Completed trades become `TradeAgreement`s recorded in history
//!
//! ## Integration
//!
//! - Uses `ResonancePattern` from `holographic_inventory` for compatibility calculations
//! - Uses `EntityId` from `entity_layer7::layer7` for entity identification
//! - Uses `ItemId` from `holographic_inventory` for item tracking

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::advanced_game_mechanics::{ResonanceCompatibility, Timestamp, TradeId};
use crate::simulation_v3::holographic_inventory::{ItemId, ResonancePattern};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// TRADE STATUS
// ============================================================================

/// Status of a trade offer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeStatus {
    /// Offer is open and available for acceptance
    Open,
    /// Offer has been accepted and completed
    Accepted,
    /// Offer has expired without being accepted
    Expired,
    /// Offer was cancelled by the offerer
    Cancelled,
}

impl TradeStatus {
    /// Check if the trade is still active (open)
    pub fn is_active(&self) -> bool {
        matches!(self, TradeStatus::Open)
    }

    /// Check if the trade has been completed or terminated
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TradeStatus::Accepted | TradeStatus::Expired | TradeStatus::Cancelled
        )
    }
}

impl std::fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeStatus::Open => write!(f, "Open"),
            TradeStatus::Accepted => write!(f, "Accepted"),
            TradeStatus::Expired => write!(f, "Expired"),
            TradeStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

// ============================================================================
// TRADE OFFER
// ============================================================================

/// Represents an offer to exchange items based on resonance compatibility
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Trading is not about monetary value but about resonance matching between entities.
///  Items flow to where they are most resonant."
#[derive(Debug, Clone)]
pub struct TradeOffer {
    /// Unique identifier for this trade offer
    pub offer_id: TradeId,
    /// Entity that created this offer
    pub offerer_id: EntityId,
    /// Items being offered for exchange
    pub offered_items: Vec<ItemId>,
    /// Items requested in exchange
    pub requested_items: Vec<ItemId>,
    /// Minimum resonance pattern required from acceptor
    /// The acceptor's resonance must be compatible with this pattern
    pub resonance_requirement: ResonancePattern,
    /// Minimum compatibility threshold (0.0-1.0)
    pub min_compatibility_threshold: Float,
    /// Time when this offer expires
    pub expiration_time: Timestamp,
    /// Current status of the offer
    pub status: TradeStatus,
    /// Optional description or terms
    pub terms: Option<String>,
    /// Creation time
    pub creation_time: Timestamp,
}

impl TradeOffer {
    /// Create a new trade offer
    ///
    /// # Arguments
    /// * `offer_id` - Unique identifier for this trade
    /// * `offerer_id` - Entity creating the offer
    /// * `offered_items` - Items being offered
    /// * `requested_items` - Items requested in exchange
    /// * `resonance_requirement` - Minimum resonance pattern required
    /// * `expiration_time` - When the offer expires
    /// * `current_time` - Current simulation time
    pub fn new(
        offer_id: TradeId,
        offerer_id: EntityId,
        offered_items: Vec<ItemId>,
        requested_items: Vec<ItemId>,
        resonance_requirement: ResonancePattern,
        expiration_time: Timestamp,
        current_time: Timestamp,
    ) -> Self {
        Self {
            offer_id,
            offerer_id,
            offered_items,
            requested_items,
            resonance_requirement,
            min_compatibility_threshold: 0.5, // Default 50% compatibility required
            expiration_time,
            status: TradeStatus::Open,
            terms: None,
            creation_time: current_time,
        }
    }

    /// Set the minimum compatibility threshold
    pub fn with_compatibility_threshold(mut self, threshold: Float) -> Self {
        self.min_compatibility_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set optional terms for the trade
    pub fn with_terms(mut self, terms: String) -> Self {
        self.terms = Some(terms);
        self
    }

    /// Check if this offer has expired
    pub fn is_expired(&self, current_time: Timestamp) -> bool {
        self.status == TradeStatus::Open && current_time >= self.expiration_time
    }

    /// Get the remaining time before expiration
    pub fn time_remaining(&self, current_time: Timestamp) -> Option<Float> {
        if self.status != TradeStatus::Open {
            None
        } else {
            let remaining = self.expiration_time - current_time;
            if remaining > 0.0 {
                Some(remaining)
            } else {
                None
            }
        }
    }

    /// Check if the offer can be accepted by a given entity
    ///
    /// Returns true if:
    /// - The offer is still open
    /// - The offer has not expired
    /// - The acceptor's resonance is compatible with the requirement
    pub fn can_accept(
        &self,
        acceptor_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> bool {
        if self.status != TradeStatus::Open {
            return false;
        }
        if self.is_expired(current_time) {
            return false;
        }
        let compatibility = ResonanceExchange::calculate_resonance_compatibility(
            &self.resonance_requirement,
            acceptor_resonance,
        );
        compatibility >= self.min_compatibility_threshold
    }
}

// ============================================================================
// TRADE AGREEMENT
// ============================================================================

/// Represents a completed trade agreement
///
/// A trade agreement is created when an offer is accepted. It records
/// the details of the exchange for historical and analytical purposes.
#[derive(Debug, Clone)]
pub struct TradeAgreement {
    /// Unique identifier for this agreement
    pub agreement_id: TradeId,
    /// ID of the original offer
    pub offer_id: TradeId,
    /// Entity that accepted the offer
    pub acceptor_id: EntityId,
    /// Entity that created the original offer
    pub offerer_id: EntityId,
    /// Items transferred from offerer to acceptor
    pub traded_items_from_offerer: Vec<ItemId>,
    /// Items transferred from acceptor to offerer
    pub traded_items_from_acceptor: Vec<ItemId>,
    /// Resonance compatibility between the two entities (0.0-1.0)
    pub resonance_compatibility: Float,
    /// Time when the trade was completed
    pub completion_time: Timestamp,
    /// Trade efficiency based on resonance (higher = more efficient)
    pub trade_efficiency: Float,
}

impl TradeAgreement {
    /// Create a new trade agreement
    pub fn new(
        agreement_id: TradeId,
        offer_id: TradeId,
        acceptor_id: EntityId,
        offerer_id: EntityId,
        traded_items_from_offerer: Vec<ItemId>,
        traded_items_from_acceptor: Vec<ItemId>,
        resonance_compatibility: Float,
        completion_time: Timestamp,
    ) -> Self {
        // Trade efficiency is higher when resonance compatibility is higher
        let trade_efficiency = 0.5 + resonance_compatibility * 0.5;

        Self {
            agreement_id,
            offer_id,
            acceptor_id,
            offerer_id,
            traded_items_from_offerer,
            traded_items_from_acceptor,
            resonance_compatibility,
            completion_time,
            trade_efficiency,
        }
    }

    /// Get the total number of items exchanged
    pub fn total_items_exchanged(&self) -> usize {
        self.traded_items_from_offerer.len() + self.traded_items_from_acceptor.len()
    }

    /// Check if a specific entity was involved in this trade
    pub fn involves_entity(&self, entity_id: EntityId) -> bool {
        self.acceptor_id == entity_id || self.offerer_id == entity_id
    }
}

// ============================================================================
// RESONANCE EXCHANGE
// ============================================================================

/// Core exchange logic for resonance-based trading
///
/// This struct provides static methods for calculating resonance compatibility
/// and validating trade compatibility between entities.
pub struct ResonanceExchange;

impl ResonanceExchange {
    /// Calculate resonance compatibility between two patterns
    ///
    /// Returns a value from 0.0 (completely incompatible) to 1.0 (perfectly compatible).
    /// The calculation uses the interference pattern between the two resonance patterns.
    ///
    /// # Arguments
    /// * `pattern_a` - First resonance pattern
    /// * `pattern_b` - Second resonance pattern
    ///
    /// # Returns
    /// Compatibility score between 0.0 and 1.0
    pub fn calculate_resonance_compatibility(
        pattern_a: &ResonancePattern,
        pattern_b: &ResonancePattern,
    ) -> Float {
        // Use the built-in interference computation from ResonancePattern
        let base_compatibility = pattern_a.compute_interference(pattern_b);

        // Factor in stability - more stable patterns have more reliable compatibility
        let stability_factor = (pattern_a.stability + pattern_b.stability) / 2.0;

        // Factor in phase alignment
        let phase_diff = (pattern_a.phase - pattern_b.phase).abs();
        let phase_alignment = 1.0 - (phase_diff / std::f64::consts::PI).min(1.0);

        // Combined compatibility score
        let compatibility =
            base_compatibility * 0.6 + stability_factor * 0.25 + phase_alignment * 0.15;

        compatibility.clamp(0.0, 1.0)
    }

    /// Calculate compatibility between an entity and a trade offer
    ///
    /// # Arguments
    /// * `offer` - The trade offer to check against
    /// * `acceptor_resonance` - The resonance pattern of the potential acceptor
    ///
    /// # Returns
    /// Compatibility score between 0.0 and 1.0
    pub fn calculate_offer_compatibility(
        offer: &TradeOffer,
        acceptor_resonance: &ResonancePattern,
    ) -> Float {
        Self::calculate_resonance_compatibility(&offer.resonance_requirement, acceptor_resonance)
    }

    /// Validate if a trade can proceed based on resonance compatibility
    ///
    /// # Arguments
    /// * `offer` - The trade offer
    /// * `acceptor_resonance` - Resonance pattern of the entity wanting to accept
    ///
    /// # Returns
    /// `true` if the trade is compatible, `false` otherwise
    pub fn validate_trade_compatibility(
        offer: &TradeOffer,
        acceptor_resonance: &ResonancePattern,
    ) -> bool {
        let compatibility = Self::calculate_offer_compatibility(offer, acceptor_resonance);
        compatibility >= offer.min_compatibility_threshold
    }

    /// Execute a trade between two entities
    ///
    /// This validates the trade compatibility and creates a TradeAgreement if successful.
    /// Note: This method does not actually transfer items - that must be done by the
    /// calling code using the returned TradeAgreement.
    ///
    /// # Arguments
    /// * `offer` - The trade offer being accepted
    /// * `acceptor_id` - ID of the entity accepting the offer
    /// * `acceptor_resonance` - Resonance pattern of the acceptor
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// `Ok(TradeAgreement)` if the trade is successful, `Err(TradingError)` otherwise
    pub fn execute_trade(
        offer: &TradeOffer,
        acceptor_id: EntityId,
        acceptor_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Result<TradeAgreement, TradingError> {
        // Check if offer is still open
        if offer.status != TradeStatus::Open {
            return Err(TradingError::InvalidTradeState {
                offer_id: offer.offer_id,
                current_status: offer.status,
                expected_status: TradeStatus::Open,
            });
        }

        // Check if offer has expired
        if offer.is_expired(current_time) {
            return Err(TradingError::ExpiredOffer {
                offer_id: offer.offer_id,
            });
        }

        // Validate resonance compatibility
        let compatibility = Self::calculate_offer_compatibility(offer, acceptor_resonance);
        if compatibility < offer.min_compatibility_threshold {
            return Err(TradingError::IncompatibleResonance {
                offer_id: offer.offer_id,
                required: offer.min_compatibility_threshold,
                actual: compatibility,
            });
        }

        // Create the trade agreement
        let agreement = TradeAgreement::new(
            TradeId::new(offer.offer_id.as_u64() + 1000000), // Generate agreement ID from offer ID
            offer.offer_id,
            acceptor_id,
            offer.offerer_id,
            offer.offered_items.clone(),
            offer.requested_items.clone(),
            compatibility,
            current_time,
        );

        Ok(agreement)
    }
}

// ============================================================================
// TRADE MATCHER
// ============================================================================

/// Matches trade offers based on resonance compatibility
///
/// The TradeMatcher maintains a registry of active trade offers and provides
/// methods to find matching offers for entities based on their resonance patterns.
#[derive(Debug, Clone)]
pub struct TradeMatcher {
    /// Active trade offers indexed by offer ID
    pub active_offers: HashMap<TradeId, TradeOffer>,
    /// Offers indexed by offerer for quick lookup
    offers_by_offerer: HashMap<EntityId, Vec<TradeId>>,
    /// Statistics for matching operations
    pub total_matches_found: u64,
    pub total_offers_processed: u64,
}

impl TradeMatcher {
    /// Create a new TradeMatcher
    pub fn new() -> Self {
        Self {
            active_offers: HashMap::new(),
            offers_by_offerer: HashMap::new(),
            total_matches_found: 0,
            total_offers_processed: 0,
        }
    }

    /// Register a new trade offer
    pub fn register_offer(&mut self, offer: TradeOffer) {
        let offer_id = offer.offer_id;
        let offerer_id = offer.offerer_id;

        self.active_offers.insert(offer_id, offer);
        self.offers_by_offerer
            .entry(offerer_id)
            .or_default()
            .push(offer_id);
    }

    /// Remove an offer from the matcher
    pub fn remove_offer(&mut self, offer_id: TradeId) -> Option<TradeOffer> {
        let offer = self.active_offers.remove(&offer_id)?;

        // Remove from offerer's list
        if let Some(offer_ids) = self.offers_by_offerer.get_mut(&offer.offerer_id) {
            offer_ids.retain(|&id| id != offer_id);
        }

        Some(offer)
    }

    /// Find matching offers for an entity based on its resonance pattern
    ///
    /// # Arguments
    /// * `entity_resonance` - The resonance pattern of the entity looking for trades
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// Vector of TradeIds for offers that are compatible with the entity
    pub fn find_matching_offers(
        &mut self,
        entity_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Vec<TradeId> {
        let mut matches = Vec::new();

        for (offer_id, offer) in &self.active_offers {
            self.total_offers_processed += 1;

            // Skip expired offers
            if offer.is_expired(current_time) {
                continue;
            }

            // Check compatibility
            if offer.can_accept(entity_resonance, current_time) {
                matches.push(*offer_id);
                self.total_matches_found += 1;
            }
        }

        matches
    }

    /// Find matching offers with detailed compatibility scores
    pub fn find_matching_offers_with_scores(
        &mut self,
        entity_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Vec<(TradeId, ResonanceCompatibility)> {
        let mut matches = Vec::new();

        for (offer_id, offer) in &self.active_offers {
            self.total_offers_processed += 1;

            if offer.is_expired(current_time) {
                continue;
            }

            let compatibility =
                ResonanceExchange::calculate_offer_compatibility(offer, entity_resonance);

            if compatibility >= offer.min_compatibility_threshold {
                matches.push((*offer_id, compatibility));
                self.total_matches_found += 1;
            }
        }

        // Sort by compatibility (highest first)
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        matches
    }

    /// Attempt to match two offers against each other
    ///
    /// This is used for counter-offers and multi-party matching.
    /// Returns the compatibility score if the offers are compatible.
    pub fn match_offers_by_resonance(
        &self,
        offer_a: &TradeOffer,
        offer_b: &TradeOffer,
    ) -> Option<Float> {
        // Calculate bidirectional compatibility
        let compatibility_a_to_b = ResonanceExchange::calculate_resonance_compatibility(
            &offer_a.resonance_requirement,
            &offer_b.resonance_requirement,
        );

        let compatibility_b_to_a = ResonanceExchange::calculate_resonance_compatibility(
            &offer_b.resonance_requirement,
            &offer_a.resonance_requirement,
        );

        // Both directions must meet their respective thresholds
        if compatibility_a_to_b >= offer_a.min_compatibility_threshold
            && compatibility_b_to_a >= offer_b.min_compatibility_threshold
        {
            // Return the average compatibility
            Some((compatibility_a_to_b + compatibility_b_to_a) / 2.0)
        } else {
            None
        }
    }

    /// Create a counter-offer based on an existing offer
    ///
    /// A counter-offer allows an entity to propose alternative terms
    /// while maintaining resonance compatibility.
    pub fn create_counter_offer(
        &self,
        original_offer: &TradeOffer,
        counter_offerer_id: EntityId,
        counter_offered_items: Vec<ItemId>,
        counter_requested_items: Vec<ItemId>,
        counter_resonance: ResonancePattern,
        expiration_time: Timestamp,
        current_time: Timestamp,
    ) -> TradeOffer {
        let counter_offer_id = TradeId::new(original_offer.offer_id.as_u64() + 500000);

        TradeOffer::new(
            counter_offer_id,
            counter_offerer_id,
            counter_offered_items,
            counter_requested_items,
            counter_resonance,
            expiration_time,
            current_time,
        )
        .with_terms(format!(
            "Counter-offer to Trade-{}: {}",
            original_offer.offer_id.as_u64(),
            original_offer
                .terms
                .as_deref()
                .unwrap_or("No terms specified")
        ))
    }

    /// Get all active offers from a specific offerer
    pub fn get_offers_by_offerer(&self, offerer_id: EntityId) -> Vec<&TradeOffer> {
        self.offers_by_offerer
            .get(&offerer_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.active_offers.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get the number of active offers
    pub fn active_offer_count(&self) -> usize {
        self.active_offers.len()
    }

    /// Clear all expired offers
    ///
    /// Returns the number of offers that were removed
    pub fn clear_expired_offers(&mut self, current_time: Timestamp) -> usize {
        let expired_ids: Vec<TradeId> = self
            .active_offers
            .iter()
            .filter(|(_, offer)| offer.is_expired(current_time))
            .map(|(id, _)| *id)
            .collect();

        for id in &expired_ids {
            if let Some(offer) = self.active_offers.remove(id) {
                if let Some(offer_ids) = self.offers_by_offerer.get_mut(&offer.offerer_id) {
                    offer_ids.retain(|&offer_id| offer_id != *id);
                }
            }
        }

        expired_ids.len()
    }
}

impl Default for TradeMatcher {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MULTI-PARTY TRADE
// ============================================================================

/// Represents a collective multi-party trade
///
/// Multi-party trades allow multiple entities to pool items and distribute
/// them based on collective resonance patterns. This enables complex
/// economic interactions that emerge from collective consciousness.
#[derive(Debug, Clone)]
pub struct MultiPartyTrade {
    /// Unique identifier for this multi-party trade
    pub trade_id: TradeId,
    /// Entities participating in the trade
    pub participants: Vec<EntityId>,
    /// Collective resonance pattern of all participants
    pub collective_resonance: ResonancePattern,
    /// Items pooled for distribution
    pub item_pool: Vec<ItemId>,
    /// Resonance pattern of each participant
    pub participant_resonances: HashMap<EntityId, ResonancePattern>,
    /// Items contributed by each participant
    pub contributions: HashMap<EntityId, Vec<ItemId>>,
    /// Time when the trade was created
    pub creation_time: Timestamp,
    /// Time when the trade was completed (if applicable)
    pub completion_time: Option<Timestamp>,
    /// Whether the trade has been completed
    pub is_completed: bool,
}

impl MultiPartyTrade {
    /// Create a new multi-party trade
    ///
    /// # Arguments
    /// * `trade_id` - Unique identifier
    /// * `participants` - Entities participating in the trade
    /// * `item_pool` - Items to be distributed
    /// * `current_time` - Current simulation time
    pub fn new(
        trade_id: TradeId,
        participants: Vec<EntityId>,
        item_pool: Vec<ItemId>,
        current_time: Timestamp,
    ) -> Self {
        Self {
            trade_id,
            participants,
            collective_resonance: ResonancePattern::new(),
            item_pool,
            participant_resonances: HashMap::new(),
            contributions: HashMap::new(),
            creation_time: current_time,
            completion_time: None,
            is_completed: false,
        }
    }

    /// Add a participant's resonance pattern
    pub fn add_participant_resonance(&mut self, entity_id: EntityId, resonance: ResonancePattern) {
        self.participant_resonances.insert(entity_id, resonance);
        self.update_collective_resonance();
    }

    /// Record items contributed by a participant
    pub fn add_contribution(&mut self, entity_id: EntityId, items: Vec<ItemId>) {
        self.contributions.insert(entity_id, items);
    }

    /// Update the collective resonance pattern based on all participants
    fn update_collective_resonance(&mut self) {
        if self.participant_resonances.is_empty() {
            self.collective_resonance = ResonancePattern::new();
            return;
        }

        let count = self.participant_resonances.len() as Float;
        let mut combined_pattern = [0.0; 8];
        let mut total_stability = 0.0;
        let mut total_phase = 0.0;

        for resonance in self.participant_resonances.values() {
            for i in 0..8 {
                combined_pattern[i] += resonance.pattern[i];
            }
            total_stability += resonance.stability;
            total_phase += resonance.phase;
        }

        // Average the patterns
        for i in 0..8 {
            combined_pattern[i] /= count;
        }

        self.collective_resonance = ResonancePattern {
            pattern: combined_pattern,
            stability: total_stability / count,
            phase: total_phase / count,
        };
    }

    /// Distribute items to participants based on their resonance compatibility
    ///
    /// Items are distributed to participants whose resonance patterns are most
    /// compatible with each item's resonance pattern.
    ///
    /// # Arguments
    /// * `item_resonances` - Map of item IDs to their resonance patterns
    ///
    /// # Returns
    /// Map of entity IDs to the items they receive
    pub fn distribute_items_by_resonance(
        &self,
        item_resonances: &HashMap<ItemId, ResonancePattern>,
    ) -> HashMap<EntityId, Vec<ItemId>> {
        let mut distribution: HashMap<EntityId, Vec<ItemId>> = HashMap::new();

        // Initialize empty vectors for all participants
        for &participant in &self.participants {
            distribution.insert(participant, Vec::new());
        }

        // For each item, find the most compatible participant
        for item_id in &self.item_pool {
            if let Some(item_resonance) = item_resonances.get(item_id) {
                let mut best_participant = None;
                let mut best_compatibility = 0.0;

                for (&entity_id, participant_resonance) in &self.participant_resonances {
                    let compatibility = ResonanceExchange::calculate_resonance_compatibility(
                        item_resonance,
                        participant_resonance,
                    );

                    if compatibility > best_compatibility {
                        best_compatibility = compatibility;
                        best_participant = Some(entity_id);
                    }
                }

                // Assign item to the most compatible participant
                if let Some(entity_id) = best_participant {
                    distribution.entry(entity_id).or_default().push(*item_id);
                }
            }
        }

        distribution
    }

    /// Distribute items equally among all participants
    pub fn distribute_items_equally(&self) -> HashMap<EntityId, Vec<ItemId>> {
        let mut distribution: HashMap<EntityId, Vec<ItemId>> = HashMap::new();
        let participant_count = self.participants.len();

        if participant_count == 0 {
            return distribution;
        }

        // Initialize empty vectors for all participants
        for &participant in &self.participants {
            distribution.insert(participant, Vec::new());
        }

        // Distribute items round-robin
        for (index, item_id) in self.item_pool.iter().enumerate() {
            let participant_index = index % participant_count;
            let entity_id = self.participants[participant_index];
            distribution.entry(entity_id).or_default().push(*item_id);
        }

        distribution
    }

    /// Calculate the fairness score of a distribution
    ///
    /// Returns a value from 0.0 (completely unfair) to 1.0 (perfectly fair)
    pub fn calculate_fairness_score(&self, distribution: &HashMap<EntityId, Vec<ItemId>>) -> Float {
        if self.participants.is_empty() {
            return 1.0;
        }

        let item_counts: Vec<usize> = self
            .participants
            .iter()
            .filter_map(|id| distribution.get(id).map(|items| items.len()))
            .collect();

        if item_counts.is_empty() {
            return 1.0;
        }

        let total_items: usize = item_counts.iter().sum();
        let ideal_count = total_items as Float / self.participants.len() as Float;

        // Calculate variance from ideal
        let variance: Float = item_counts
            .iter()
            .map(|&count| {
                let diff = count as Float - ideal_count;
                diff * diff
            })
            .sum::<Float>()
            / item_counts.len() as Float;

        // Convert variance to fairness score (lower variance = higher fairness)
        let max_variance = ideal_count * ideal_count;
        1.0 - (variance / max_variance).min(1.0)
    }

    /// Mark the trade as completed
    pub fn complete(&mut self, current_time: Timestamp) {
        self.is_completed = true;
        self.completion_time = Some(current_time);
    }
}

// ============================================================================
// TRADING ERROR
// ============================================================================

/// Errors that can occur during trading operations
#[derive(Debug, Clone, PartialEq)]
pub enum TradingError {
    /// Resonance patterns are incompatible for trading
    IncompatibleResonance {
        offer_id: TradeId,
        required: Float,
        actual: Float,
    },
    /// The trade offer has expired
    ExpiredOffer { offer_id: TradeId },
    /// Entity does not have sufficient items for the trade
    InsufficientItems {
        entity_id: EntityId,
        item_id: ItemId,
        required: u64,
        available: u64,
    },
    /// The trade is in an invalid state for the requested operation
    InvalidTradeState {
        offer_id: TradeId,
        current_status: TradeStatus,
        expected_status: TradeStatus,
    },
    /// Resonance mismatch with detailed information
    ResonanceMismatch { expected: Float, actual: Float },
    /// Offer not found
    OfferNotFound { offer_id: TradeId },
    /// Entity not authorized for this operation
    Unauthorized {
        entity_id: EntityId,
        operation: String,
    },
    /// Multi-party trade error
    MultiPartyError { reason: String },
}

impl std::fmt::Display for TradingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradingError::IncompatibleResonance {
                offer_id,
                required,
                actual,
            } => {
                write!(
                    f,
                    "Incompatible resonance for offer {}: required {:.2}, actual {:.2}",
                    offer_id, required, actual
                )
            }
            TradingError::ExpiredOffer { offer_id } => {
                write!(f, "Trade offer {} has expired", offer_id)
            }
            TradingError::InsufficientItems {
                entity_id,
                item_id,
                required,
                available,
            } => {
                write!(
                    f,
                    "Entity {} has insufficient items: need {} of {:?}, have {}",
                    entity_id, required, item_id, available
                )
            }
            TradingError::InvalidTradeState {
                offer_id,
                current_status,
                expected_status,
            } => {
                write!(
                    f,
                    "Invalid trade state for offer {}: current {:?}, expected {:?}",
                    offer_id, current_status, expected_status
                )
            }
            TradingError::ResonanceMismatch { expected, actual } => {
                write!(
                    f,
                    "Resonance mismatch: expected {:.2}, actual {:.2}",
                    expected, actual
                )
            }
            TradingError::OfferNotFound { offer_id } => {
                write!(f, "Trade offer {} not found", offer_id)
            }
            TradingError::Unauthorized {
                entity_id,
                operation,
            } => {
                write!(
                    f,
                    "Entity {} not authorized for operation: {}",
                    entity_id, operation
                )
            }
            TradingError::MultiPartyError { reason } => {
                write!(f, "Multi-party trade error: {}", reason)
            }
        }
    }
}

impl std::error::Error for TradingError {}

// ============================================================================
// RESONANCE TRADING SYSTEM
// ============================================================================

/// Main trading system that coordinates all trading activities
///
/// The ResonanceTrading system is the central coordinator for all
/// resonance-based trading activities. It maintains trade history,
/// manages the trade matcher, and provides high-level trading operations.
#[derive(Debug, Clone)]
pub struct ResonanceTrading {
    /// Matcher for finding compatible trades
    pub trade_matcher: TradeMatcher,
    /// Currently active agreements (not yet finalized)
    pub active_agreements: Vec<TradeAgreement>,
    /// Historical record of all completed trades
    pub trade_history: Vec<TradeAgreement>,
    /// Multi-party trades currently in progress
    pub active_multi_party_trades: HashMap<TradeId, MultiPartyTrade>,
    /// Counter for generating unique IDs
    next_id: u64,
}

impl ResonanceTrading {
    /// Create a new ResonanceTrading system
    pub fn new() -> Self {
        Self {
            trade_matcher: TradeMatcher::new(),
            active_agreements: Vec::new(),
            trade_history: Vec::new(),
            active_multi_party_trades: HashMap::new(),
            next_id: 1,
        }
    }

    /// Generate a new unique trade ID
    fn generate_trade_id(&mut self) -> TradeId {
        let id = self.next_id;
        self.next_id += 1;
        TradeId::new(id)
    }

    /// Create a new trade offer
    ///
    /// # Arguments
    /// * `offerer` - Entity creating the offer
    /// * `offered_items` - Items being offered
    /// * `requested_items` - Items requested in exchange
    /// * `resonance_req` - Minimum resonance pattern required from acceptor
    /// * `expiration_time` - When the offer expires
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// `Ok(TradeOffer)` if created successfully, `Err(TradingError)` otherwise
    pub fn create_trade_offer(
        &mut self,
        offerer: EntityId,
        offered_items: Vec<ItemId>,
        requested_items: Vec<ItemId>,
        resonance_req: ResonancePattern,
        expiration_time: Timestamp,
        current_time: Timestamp,
    ) -> Result<TradeOffer, TradingError> {
        // Validate that we have items to offer
        if offered_items.is_empty() {
            return Err(TradingError::MultiPartyError {
                reason: "Cannot create trade offer with no items".to_string(),
            });
        }

        let offer_id = self.generate_trade_id();

        let offer = TradeOffer::new(
            offer_id,
            offerer,
            offered_items,
            requested_items,
            resonance_req,
            expiration_time,
            current_time,
        );

        // Register the offer with the matcher
        self.trade_matcher.register_offer(offer.clone());

        Ok(offer)
    }

    /// Accept a trade offer
    ///
    /// # Arguments
    /// * `offer_id` - ID of the offer to accept
    /// * `acceptor` - Entity accepting the offer
    /// * `acceptor_resonance` - Resonance pattern of the acceptor
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// `Ok(TradeAgreement)` if successful, `Err(TradingError)` otherwise
    pub fn accept_trade_offer(
        &mut self,
        offer_id: TradeId,
        acceptor: EntityId,
        acceptor_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Result<TradeAgreement, TradingError> {
        // Get the offer from the matcher
        let offer = self
            .trade_matcher
            .active_offers
            .get(&offer_id)
            .ok_or(TradingError::OfferNotFound { offer_id })?
            .clone();

        // Check that the acceptor is not the offerer
        if acceptor == offer.offerer_id {
            return Err(TradingError::Unauthorized {
                entity_id: acceptor,
                operation: "Cannot accept your own trade offer".to_string(),
            });
        }

        // Execute the trade
        let agreement =
            ResonanceExchange::execute_trade(&offer, acceptor, acceptor_resonance, current_time)?;

        // Remove the offer from active offers
        self.trade_matcher.remove_offer(offer_id);

        // Add to active agreements
        self.active_agreements.push(agreement.clone());

        // Move to history immediately (in a real system, this might be done after item transfer)
        self.trade_history.push(agreement.clone());

        Ok(agreement)
    }

    /// Cancel a trade offer
    ///
    /// # Arguments
    /// * `offer_id` - ID of the offer to cancel
    /// * `offerer` - Entity requesting the cancellation (must be the original offerer)
    ///
    /// # Returns
    /// `Ok(())` if cancelled successfully, `Err(TradingError)` otherwise
    pub fn cancel_trade_offer(
        &mut self,
        offer_id: TradeId,
        offerer: EntityId,
    ) -> Result<(), TradingError> {
        let offer = self
            .trade_matcher
            .active_offers
            .get(&offer_id)
            .ok_or(TradingError::OfferNotFound { offer_id })?;

        // Verify the requester is the offerer
        if offer.offerer_id != offerer {
            return Err(TradingError::Unauthorized {
                entity_id: offerer,
                operation: format!("Cancel trade offer {}", offer_id.as_u64()),
            });
        }

        // Remove the offer
        self.trade_matcher.remove_offer(offer_id);

        Ok(())
    }

    /// Find trade matches for an entity
    ///
    /// # Arguments
    /// * `entity_resonance` - Resonance pattern of the entity
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// Vector of matching trade offers
    pub fn find_trade_matches(
        &mut self,
        entity_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Vec<TradeOffer> {
        let matching_ids = self
            .trade_matcher
            .find_matching_offers(entity_resonance, current_time);

        matching_ids
            .iter()
            .filter_map(|id| self.trade_matcher.active_offers.get(id).cloned())
            .collect()
    }

    /// Find trade matches with compatibility scores
    pub fn find_trade_matches_with_scores(
        &mut self,
        entity_resonance: &ResonancePattern,
        current_time: Timestamp,
    ) -> Vec<(TradeOffer, ResonanceCompatibility)> {
        let matches = self
            .trade_matcher
            .find_matching_offers_with_scores(entity_resonance, current_time);

        matches
            .into_iter()
            .filter_map(|(id, score)| {
                self.trade_matcher
                    .active_offers
                    .get(&id)
                    .map(|offer| (offer.clone(), score))
            })
            .collect()
    }

    /// Execute a multi-party trade
    ///
    /// # Arguments
    /// * `participants` - Entities participating in the trade
    /// * `item_pool` - Items to be distributed
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// `Ok(MultiPartyTrade)` if created successfully, `Err(TradingError)` otherwise
    pub fn execute_multi_party_trade(
        &mut self,
        participants: Vec<EntityId>,
        item_pool: Vec<ItemId>,
        current_time: Timestamp,
    ) -> Result<MultiPartyTrade, TradingError> {
        // Validate participants
        if participants.len() < 2 {
            return Err(TradingError::MultiPartyError {
                reason: "Multi-party trade requires at least 2 participants".to_string(),
            });
        }

        if item_pool.is_empty() {
            return Err(TradingError::MultiPartyError {
                reason: "Multi-party trade requires at least 1 item".to_string(),
            });
        }

        let trade_id = self.generate_trade_id();

        let multi_party_trade =
            MultiPartyTrade::new(trade_id, participants, item_pool, current_time);

        // Store the active multi-party trade
        self.active_multi_party_trades
            .insert(trade_id, multi_party_trade.clone());

        Ok(multi_party_trade)
    }

    /// Complete a multi-party trade
    pub fn complete_multi_party_trade(
        &mut self,
        trade_id: TradeId,
        item_resonances: &HashMap<ItemId, ResonancePattern>,
        current_time: Timestamp,
    ) -> Result<HashMap<EntityId, Vec<ItemId>>, TradingError> {
        let mut trade = self.active_multi_party_trades.remove(&trade_id).ok_or(
            TradingError::OfferNotFound {
                offer_id: trade_id, // Using offer_id field for trade_id in error
            },
        )?;

        // Distribute items
        let distribution = trade.distribute_items_by_resonance(item_resonances);

        // Mark as completed
        trade.complete(current_time);

        Ok(distribution)
    }

    /// Get trade history for a specific entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity to get history for
    ///
    /// # Returns
    /// Vector of trade agreements involving the entity
    pub fn get_trade_history(&self, entity_id: EntityId) -> Vec<TradeAgreement> {
        self.trade_history
            .iter()
            .filter(|agreement| agreement.involves_entity(entity_id))
            .cloned()
            .collect()
    }

    /// Get all trade history
    pub fn get_all_trade_history(&self) -> &[TradeAgreement] {
        &self.trade_history
    }

    /// Expire old trades
    ///
    /// # Arguments
    /// * `current_time` - Current simulation time
    ///
    /// # Returns
    /// Vector of trade IDs that were expired
    pub fn expire_old_trades(&mut self, current_time: Timestamp) -> Vec<TradeId> {
        let expired_ids: Vec<TradeId> = self
            .trade_matcher
            .active_offers
            .iter()
            .filter(|(_, offer)| offer.is_expired(current_time))
            .map(|(id, _)| *id)
            .collect();

        for id in &expired_ids {
            self.trade_matcher.remove_offer(*id);
        }

        expired_ids
    }

    /// Get statistics about the trading system
    pub fn get_statistics(&self) -> TradingStatistics {
        TradingStatistics {
            active_offers: self.trade_matcher.active_offer_count(),
            total_trades_completed: self.trade_history.len(),
            active_agreements: self.active_agreements.len(),
            active_multi_party_trades: self.active_multi_party_trades.len(),
            total_matches_found: self.trade_matcher.total_matches_found,
            total_offers_processed: self.trade_matcher.total_offers_processed,
        }
    }

    /// Get a reference to a multi-party trade
    pub fn get_multi_party_trade(&self, trade_id: TradeId) -> Option<&MultiPartyTrade> {
        self.active_multi_party_trades.get(&trade_id)
    }

    /// Get a mutable reference to a multi-party trade
    pub fn get_multi_party_trade_mut(&mut self, trade_id: TradeId) -> Option<&mut MultiPartyTrade> {
        self.active_multi_party_trades.get_mut(&trade_id)
    }
}

impl Default for ResonanceTrading {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about trading activity
#[derive(Debug, Clone, Copy)]
pub struct TradingStatistics {
    /// Number of active trade offers
    pub active_offers: usize,
    /// Total number of completed trades
    pub total_trades_completed: usize,
    /// Number of active agreements
    pub active_agreements: usize,
    /// Number of active multi-party trades
    pub active_multi_party_trades: usize,
    /// Total number of matches found
    pub total_matches_found: u64,
    /// Total number of offers processed for matching
    pub total_offers_processed: u64,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // TRADE OFFER TESTS
    // ============================================================================

    #[test]
    fn test_trade_offer_creation() {
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1), ItemId::new(2)],
            vec![ItemId::new(3)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        assert_eq!(offer.offer_id.as_u64(), 1);
        assert_eq!(offer.offerer_id, 100);
        assert_eq!(offer.offered_items.len(), 2);
        assert_eq!(offer.requested_items.len(), 1);
        assert_eq!(offer.status, TradeStatus::Open);
        assert!(!offer.is_expired(500.0));
        assert!(offer.is_expired(1000.0));
    }

    #[test]
    fn test_trade_offer_with_compatibility_threshold() {
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.75);

        assert_eq!(offer.min_compatibility_threshold, 0.75);
    }

    #[test]
    fn test_trade_offer_time_remaining() {
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        assert_eq!(offer.time_remaining(500.0), Some(500.0));
        assert_eq!(offer.time_remaining(1000.0), None);
    }

    #[test]
    fn test_trade_offer_can_accept() {
        let mut pattern = ResonancePattern::new();
        pattern.pattern[0] = 0.8;
        pattern.stability = 0.9;

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.5);

        // Same pattern should be compatible
        assert!(offer.can_accept(&pattern, 500.0));

        // Different pattern may not be compatible
        let different_pattern = ResonancePattern::new();
        // Note: Compatibility depends on the exact calculation

        // Expired offer should not be acceptable
        assert!(!offer.can_accept(&pattern, 1000.0));
    }

    // ============================================================================
    // TRADE AGREEMENT TESTS
    // ============================================================================

    #[test]
    fn test_trade_agreement_creation() {
        let agreement = TradeAgreement::new(
            TradeId::new(1),
            TradeId::new(2),
            200, // acceptor
            100, // offerer
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            0.85,
            500.0,
        );

        assert_eq!(agreement.agreement_id.as_u64(), 1);
        assert_eq!(agreement.offer_id.as_u64(), 2);
        assert_eq!(agreement.acceptor_id, 200);
        assert_eq!(agreement.offerer_id, 100);
        assert_eq!(agreement.resonance_compatibility, 0.85);
        assert_eq!(agreement.total_items_exchanged(), 2);
        assert!(agreement.involves_entity(100));
        assert!(agreement.involves_entity(200));
        assert!(!agreement.involves_entity(300));
    }

    #[test]
    fn test_trade_efficiency_calculation() {
        let agreement_high = TradeAgreement::new(
            TradeId::new(1),
            TradeId::new(2),
            200,
            100,
            vec![],
            vec![],
            1.0, // Perfect compatibility
            500.0,
        );

        let agreement_low = TradeAgreement::new(
            TradeId::new(2),
            TradeId::new(3),
            200,
            100,
            vec![],
            vec![],
            0.0, // No compatibility
            500.0,
        );

        // Efficiency = 0.5 + compatibility * 0.5
        assert_eq!(agreement_high.trade_efficiency, 1.0);
        assert_eq!(agreement_low.trade_efficiency, 0.5);
    }

    // ============================================================================
    // RESONANCE EXCHANGE TESTS
    // ============================================================================

    #[test]
    fn test_calculate_resonance_compatibility_identical() {
        let pattern = ResonancePattern::new();
        let compatibility =
            ResonanceExchange::calculate_resonance_compatibility(&pattern, &pattern);

        // Identical patterns should have high compatibility
        assert!(compatibility > 0.9);
    }

    #[test]
    fn test_calculate_resonance_compatibility_different() {
        let pattern1 = ResonancePattern::new();
        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern[0] = 1.0;
        pattern2.pattern[1] = 1.0;

        let compatibility =
            ResonanceExchange::calculate_resonance_compatibility(&pattern1, &pattern2);

        // Different patterns should have lower compatibility
        assert!(compatibility < 1.0);
    }

    #[test]
    fn test_validate_trade_compatibility() {
        let pattern = ResonancePattern::new();
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.5);

        assert!(ResonanceExchange::validate_trade_compatibility(
            &offer, &pattern
        ));
    }

    #[test]
    fn test_execute_trade_success() {
        let pattern = ResonancePattern::new();
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        );

        let result = ResonanceExchange::execute_trade(&offer, 200, &pattern, 500.0);

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.acceptor_id, 200);
        assert_eq!(agreement.offerer_id, 100);
    }

    #[test]
    fn test_execute_trade_expired() {
        let pattern = ResonancePattern::new();
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        );

        let result = ResonanceExchange::execute_trade(&offer, 200, &pattern, 1000.0);

        assert!(result.is_err());
        match result.unwrap_err() {
            TradingError::ExpiredOffer { offer_id } => assert_eq!(offer_id.as_u64(), 1),
            _ => panic!("Expected ExpiredOffer error"),
        }
    }

    #[test]
    fn test_execute_trade_incompatible_resonance() {
        let mut pattern1 = ResonancePattern::new();
        pattern1.pattern = [1.0; 8]; // All ones
        pattern1.stability = 1.0;

        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern = [0.0; 8]; // All zeros
        pattern2.stability = 1.0;

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern1,
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.9); // High threshold

        let result = ResonanceExchange::execute_trade(&offer, 200, &pattern2, 500.0);

        assert!(result.is_err());
        match result.unwrap_err() {
            TradingError::IncompatibleResonance {
                offer_id, required, ..
            } => {
                assert_eq!(offer_id.as_u64(), 1);
                assert_eq!(required, 0.9);
            }
            _ => panic!("Expected IncompatibleResonance error"),
        }
    }

    // ============================================================================
    // TRADE MATCHER TESTS
    // ============================================================================

    #[test]
    fn test_trade_matcher_registration() {
        let mut matcher = TradeMatcher::new();
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        matcher.register_offer(offer);

        assert_eq!(matcher.active_offer_count(), 1);
    }

    #[test]
    fn test_trade_matcher_remove_offer() {
        let mut matcher = TradeMatcher::new();
        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        matcher.register_offer(offer);
        let removed = matcher.remove_offer(TradeId::new(1));

        assert!(removed.is_some());
        assert_eq!(matcher.active_offer_count(), 0);
    }

    #[test]
    fn test_find_matching_offers() {
        let mut matcher = TradeMatcher::new();
        let pattern = ResonancePattern::new();

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.3);

        matcher.register_offer(offer);

        let matches = matcher.find_matching_offers(&pattern, 500.0);
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_find_matching_offers_expired() {
        let mut matcher = TradeMatcher::new();
        let pattern = ResonancePattern::new();

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        );

        matcher.register_offer(offer);

        // After expiration
        let matches = matcher.find_matching_offers(&pattern, 1000.0);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_match_offers_by_resonance() {
        let matcher = TradeMatcher::new();
        let pattern1 = ResonancePattern::new();
        let pattern2 = ResonancePattern::new();

        let offer1 = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern1.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.3);

        let offer2 = TradeOffer::new(
            TradeId::new(2),
            200,
            vec![ItemId::new(2)],
            vec![ItemId::new(1)],
            pattern2.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.3);

        let compatibility = matcher.match_offers_by_resonance(&offer1, &offer2);
        assert!(compatibility.is_some());
    }

    #[test]
    fn test_create_counter_offer() {
        let matcher = TradeMatcher::new();
        let original_offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        )
        .with_terms("Original terms".to_string());

        let counter = matcher.create_counter_offer(
            &original_offer,
            200,
            vec![ItemId::new(3)],
            vec![ItemId::new(4)],
            ResonancePattern::new(),
            2000.0,
            100.0,
        );

        assert_eq!(counter.offerer_id, 200);
        assert!(counter.terms.as_ref().unwrap().contains("Counter-offer"));
    }

    #[test]
    fn test_clear_expired_offers() {
        let mut matcher = TradeMatcher::new();
        let pattern = ResonancePattern::new();

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern,
            1000.0,
            0.0,
        );

        matcher.register_offer(offer);
        let cleared = matcher.clear_expired_offers(1500.0);

        assert_eq!(cleared, 1);
        assert_eq!(matcher.active_offer_count(), 0);
    }

    // ============================================================================
    // MULTI-PARTY TRADE TESTS
    // ============================================================================

    #[test]
    fn test_multi_party_trade_creation() {
        let trade = MultiPartyTrade::new(
            TradeId::new(1),
            vec![100, 200, 300],
            vec![ItemId::new(1), ItemId::new(2), ItemId::new(3)],
            0.0,
        );

        assert_eq!(trade.trade_id.as_u64(), 1);
        assert_eq!(trade.participants.len(), 3);
        assert_eq!(trade.item_pool.len(), 3);
        assert!(!trade.is_completed);
    }

    #[test]
    fn test_multi_party_add_participant_resonance() {
        let mut trade =
            MultiPartyTrade::new(TradeId::new(1), vec![100, 200], vec![ItemId::new(1)], 0.0);

        let mut pattern = ResonancePattern::new();
        pattern.pattern[0] = 0.8;

        trade.add_participant_resonance(100, pattern.clone());

        assert!(trade.participant_resonances.contains_key(&100));
    }

    #[test]
    fn test_multi_party_distribute_items_equally() {
        let trade = MultiPartyTrade::new(
            TradeId::new(1),
            vec![100, 200],
            vec![
                ItemId::new(1),
                ItemId::new(2),
                ItemId::new(3),
                ItemId::new(4),
            ],
            0.0,
        );

        let distribution = trade.distribute_items_equally();

        assert_eq!(distribution.get(&100).unwrap().len(), 2);
        assert_eq!(distribution.get(&200).unwrap().len(), 2);
    }

    #[test]
    fn test_multi_party_distribute_items_by_resonance() {
        let mut trade = MultiPartyTrade::new(
            TradeId::new(1),
            vec![100, 200],
            vec![ItemId::new(1), ItemId::new(2)],
            0.0,
        );

        // Set up participant resonances
        let mut pattern1 = ResonancePattern::new();
        pattern1.pattern[0] = 1.0; // High in first dimension

        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern[0] = 0.0; // Low in first dimension

        trade.add_participant_resonance(100, pattern1.clone());
        trade.add_participant_resonance(200, pattern2.clone());

        // Set up item resonances
        let mut item_resonances = HashMap::new();

        let mut item1_pattern = ResonancePattern::new();
        item1_pattern.pattern[0] = 0.9; // Similar to pattern1
        item_resonances.insert(ItemId::new(1), item1_pattern);

        let mut item2_pattern = ResonancePattern::new();
        item2_pattern.pattern[0] = 0.1; // Similar to pattern2
        item_resonances.insert(ItemId::new(2), item2_pattern);

        let distribution = trade.distribute_items_by_resonance(&item_resonances);

        // Item 1 should go to entity 100, item 2 to entity 200
        assert!(distribution.get(&100).unwrap().contains(&ItemId::new(1)));
        assert!(distribution.get(&200).unwrap().contains(&ItemId::new(2)));
    }

    #[test]
    fn test_calculate_fairness_score() {
        let trade = MultiPartyTrade::new(
            TradeId::new(1),
            vec![100, 200],
            vec![ItemId::new(1), ItemId::new(2)],
            0.0,
        );

        // Fair distribution (1 item each)
        let mut fair_distribution = HashMap::new();
        fair_distribution.insert(100, vec![ItemId::new(1)]);
        fair_distribution.insert(200, vec![ItemId::new(2)]);

        let fair_score = trade.calculate_fairness_score(&fair_distribution);
        assert_eq!(fair_score, 1.0);

        // Unfair distribution (2 items to one, 0 to other)
        let mut unfair_distribution = HashMap::new();
        unfair_distribution.insert(100, vec![ItemId::new(1), ItemId::new(2)]);
        unfair_distribution.insert(200, vec![]);

        let unfair_score = trade.calculate_fairness_score(&unfair_distribution);
        assert!(unfair_score < 1.0);
    }

    #[test]
    fn test_multi_party_complete() {
        let mut trade =
            MultiPartyTrade::new(TradeId::new(1), vec![100, 200], vec![ItemId::new(1)], 0.0);

        trade.complete(1000.0);

        assert!(trade.is_completed);
        assert_eq!(trade.completion_time, Some(1000.0));
    }

    // ============================================================================
    // TRADING ERROR TESTS
    // ============================================================================

    #[test]
    fn test_trading_error_display() {
        let error = TradingError::ExpiredOffer {
            offer_id: TradeId::new(1),
        };
        assert!(error.to_string().contains("expired"));

        let error = TradingError::ResonanceMismatch {
            expected: 0.8,
            actual: 0.3,
        };
        assert!(error.to_string().contains("mismatch"));
    }

    // ============================================================================
    // RESONANCE TRADING SYSTEM TESTS
    // ============================================================================

    #[test]
    fn test_resonance_trading_creation() {
        let trading = ResonanceTrading::new();

        assert_eq!(trading.trade_matcher.active_offer_count(), 0);
        assert_eq!(trading.trade_history.len(), 0);
    }

    #[test]
    fn test_create_trade_offer() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        let result = trading.create_trade_offer(
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern,
            1000.0,
            0.0,
        );

        assert!(result.is_ok());
        let offer = result.unwrap();
        assert_eq!(trading.trade_matcher.active_offer_count(), 1);
    }

    #[test]
    fn test_create_trade_offer_empty_items() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        let result = trading.create_trade_offer(
            100,
            vec![], // Empty items
            vec![ItemId::new(2)],
            pattern,
            1000.0,
            0.0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_accept_trade_offer() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        // Create offer
        let offer = trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        // Accept offer
        let result = trading.accept_trade_offer(offer.offer_id, 200, &pattern, 500.0);

        assert!(result.is_ok());
        assert_eq!(trading.trade_history.len(), 1);
        assert_eq!(trading.trade_matcher.active_offer_count(), 0);
    }

    #[test]
    fn test_accept_own_offer_fails() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        let offer = trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        // Try to accept own offer
        let result = trading.accept_trade_offer(offer.offer_id, 100, &pattern, 500.0);

        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_trade_offer() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        let offer = trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern,
                1000.0,
                0.0,
            )
            .unwrap();

        let result = trading.cancel_trade_offer(offer.offer_id, 100);

        assert!(result.is_ok());
        assert_eq!(trading.trade_matcher.active_offer_count(), 0);
    }

    #[test]
    fn test_cancel_unauthorized() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        let offer = trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern,
                1000.0,
                0.0,
            )
            .unwrap();

        // Try to cancel as different entity
        let result = trading.cancel_trade_offer(offer.offer_id, 200);

        assert!(result.is_err());
    }

    #[test]
    fn test_find_trade_matches() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        let matches = trading.find_trade_matches(&pattern, 500.0);

        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_execute_multi_party_trade() {
        let mut trading = ResonanceTrading::new();

        let result = trading.execute_multi_party_trade(
            vec![100, 200, 300],
            vec![ItemId::new(1), ItemId::new(2)],
            0.0,
        );

        assert!(result.is_ok());
        let trade = result.unwrap();
        assert_eq!(trading.active_multi_party_trades.len(), 1);
    }

    #[test]
    fn test_execute_multi_party_trade_insufficient_participants() {
        let mut trading = ResonanceTrading::new();

        let result = trading.execute_multi_party_trade(
            vec![100], // Only 1 participant
            vec![ItemId::new(1)],
            0.0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_execute_multi_party_trade_no_items() {
        let mut trading = ResonanceTrading::new();

        let result = trading.execute_multi_party_trade(
            vec![100, 200],
            vec![], // No items
            0.0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_get_trade_history() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        // Create and accept a trade
        let offer = trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        trading
            .accept_trade_offer(offer.offer_id, 200, &pattern, 500.0)
            .unwrap();

        // Check history for both entities
        let history_100 = trading.get_trade_history(100);
        let history_200 = trading.get_trade_history(200);
        let history_300 = trading.get_trade_history(300);

        assert_eq!(history_100.len(), 1);
        assert_eq!(history_200.len(), 1);
        assert_eq!(history_300.len(), 0);
    }

    #[test]
    fn test_expire_old_trades() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern,
                1000.0,
                0.0,
            )
            .unwrap();

        let expired = trading.expire_old_trades(1500.0);

        assert_eq!(expired.len(), 1);
        assert_eq!(trading.trade_matcher.active_offer_count(), 0);
    }

    #[test]
    fn test_get_statistics() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        let stats = trading.get_statistics();

        assert_eq!(stats.active_offers, 1);
        assert_eq!(stats.total_trades_completed, 0);
    }

    #[test]
    fn test_complete_multi_party_trade() {
        let mut trading = ResonanceTrading::new();

        // Create multi-party trade
        let trade = trading
            .execute_multi_party_trade(vec![100, 200], vec![ItemId::new(1), ItemId::new(2)], 0.0)
            .unwrap();

        // Set up participant resonances
        let mut pattern1 = ResonancePattern::new();
        pattern1.pattern[0] = 1.0;

        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern[0] = 0.0;

        if let Some(trade_ref) = trading.get_multi_party_trade_mut(trade.trade_id) {
            trade_ref.add_participant_resonance(100, pattern1);
            trade_ref.add_participant_resonance(200, pattern2);
        }

        // Set up item resonances
        let mut item_resonances = HashMap::new();

        let mut item1_pattern = ResonancePattern::new();
        item1_pattern.pattern[0] = 0.9;
        item_resonances.insert(ItemId::new(1), item1_pattern);

        let mut item2_pattern = ResonancePattern::new();
        item2_pattern.pattern[0] = 0.1;
        item_resonances.insert(ItemId::new(2), item2_pattern);

        // Complete the trade
        let result = trading.complete_multi_party_trade(trade.trade_id, &item_resonances, 1000.0);

        assert!(result.is_ok());
        assert_eq!(trading.active_multi_party_trades.len(), 0);
    }

    #[test]
    fn test_trade_status_display() {
        assert_eq!(TradeStatus::Open.to_string(), "Open");
        assert_eq!(TradeStatus::Accepted.to_string(), "Accepted");
        assert_eq!(TradeStatus::Expired.to_string(), "Expired");
        assert_eq!(TradeStatus::Cancelled.to_string(), "Cancelled");
    }

    #[test]
    fn test_trade_status_is_active() {
        assert!(TradeStatus::Open.is_active());
        assert!(!TradeStatus::Accepted.is_active());
        assert!(!TradeStatus::Expired.is_active());
        assert!(!TradeStatus::Cancelled.is_active());
    }

    #[test]
    fn test_trade_status_is_terminal() {
        assert!(!TradeStatus::Open.is_terminal());
        assert!(TradeStatus::Accepted.is_terminal());
        assert!(TradeStatus::Expired.is_terminal());
        assert!(TradeStatus::Cancelled.is_terminal());
    }

    #[test]
    fn test_resonance_pattern_affects_compatibility() {
        // Create two very different patterns
        let mut pattern1 = ResonancePattern::new();
        pattern1.pattern = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        pattern1.stability = 1.0;
        pattern1.phase = 0.0;

        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        pattern2.stability = 1.0;
        pattern2.phase = std::f64::consts::PI;

        let compatibility =
            ResonanceExchange::calculate_resonance_compatibility(&pattern1, &pattern2);

        // Very different patterns should have low compatibility
        assert!(compatibility < 0.5);
    }

    #[test]
    fn test_find_matching_offers_with_scores() {
        let mut matcher = TradeMatcher::new();
        let pattern = ResonancePattern::new();

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern.clone(),
            1000.0,
            0.0,
        )
        .with_compatibility_threshold(0.3);

        matcher.register_offer(offer);

        let matches = matcher.find_matching_offers_with_scores(&pattern, 500.0);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].1 > 0.0); // Should have a positive compatibility score
    }

    #[test]
    fn test_find_trade_matches_with_scores() {
        let mut trading = ResonanceTrading::new();
        let pattern = ResonancePattern::new();

        trading
            .create_trade_offer(
                100,
                vec![ItemId::new(1)],
                vec![ItemId::new(2)],
                pattern.clone(),
                1000.0,
                0.0,
            )
            .unwrap();

        let matches = trading.find_trade_matches_with_scores(&pattern, 500.0);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].1 > 0.0);
    }

    #[test]
    fn test_multi_party_trade_with_contributions() {
        let mut trade = MultiPartyTrade::new(
            TradeId::new(1),
            vec![100, 200],
            vec![ItemId::new(1), ItemId::new(2)],
            0.0,
        );

        trade.add_contribution(100, vec![ItemId::new(1)]);
        trade.add_contribution(200, vec![ItemId::new(2)]);

        assert_eq!(trade.contributions.get(&100).unwrap().len(), 1);
        assert_eq!(trade.contributions.get(&200).unwrap().len(), 1);
    }

    #[test]
    fn test_get_offers_by_offerer() {
        let mut matcher = TradeMatcher::new();

        let offer1 = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        let offer2 = TradeOffer::new(
            TradeId::new(2),
            100,
            vec![ItemId::new(3)],
            vec![ItemId::new(4)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        let offer3 = TradeOffer::new(
            TradeId::new(3),
            200,
            vec![ItemId::new(5)],
            vec![ItemId::new(6)],
            ResonancePattern::new(),
            1000.0,
            0.0,
        );

        matcher.register_offer(offer1);
        matcher.register_offer(offer2);
        matcher.register_offer(offer3);

        let offers_100 = matcher.get_offers_by_offerer(100);
        let offers_200 = matcher.get_offers_by_offerer(200);
        let offers_300 = matcher.get_offers_by_offerer(300);

        assert_eq!(offers_100.len(), 2);
        assert_eq!(offers_200.len(), 1);
        assert_eq!(offers_300.len(), 0);
    }

    #[test]
    fn test_resonance_exchange_calculate_offer_compatibility() {
        let mut pattern1 = ResonancePattern::new();
        pattern1.pattern[0] = 0.8;

        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern[0] = 0.8;

        let offer = TradeOffer::new(
            TradeId::new(1),
            100,
            vec![ItemId::new(1)],
            vec![ItemId::new(2)],
            pattern1,
            1000.0,
            0.0,
        );

        let compatibility = ResonanceExchange::calculate_offer_compatibility(&offer, &pattern2);

        // Similar patterns should have high compatibility
        assert!(compatibility > 0.8);
    }

    #[test]
    fn test_trade_agreement_getters() {
        let agreement = TradeAgreement::new(
            TradeId::new(1),
            TradeId::new(2),
            200,
            100,
            vec![ItemId::new(1), ItemId::new(2)],
            vec![ItemId::new(3)],
            0.75,
            500.0,
        );

        assert_eq!(agreement.total_items_exchanged(), 3);
        assert!(agreement.involves_entity(100));
        assert!(agreement.involves_entity(200));
        assert!(!agreement.involves_entity(300));
    }
}
