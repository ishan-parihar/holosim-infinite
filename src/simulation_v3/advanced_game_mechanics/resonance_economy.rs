//! Week 96: Economy as Resonance Flow
//!
//! This module implements the resonance economy where value is measured by
//! resonance flow (creation, circulation, transformation), NOT by currency.
//!
//! ## Core Principles
//!
//! From GAMING_ENGINE_ROADMAP_v2.md:
//! > "Week 96: Economy as Resonance Flow"
//! > "Value is measured by resonance flow, not currency"
//! > "Economy tracks how resonance is generated, flows, and transforms"
//!
//! The resonance economy operates on holographic principles:
//! - **Resonance Creation**: Catalyst generation from combat, crafting, and activities
//! - **Resonance Circulation**: Trading and exchange between entities
//! - **Resonance Transformation**: Processing through crafting, quests, and other activities
//! - **Resonance Decay**: Natural dissipation over time (prevents infinite accumulation)
//!
//! ## Economic Health Metrics
//!
//! The economy is healthy when:
//! - **High Circulation Velocity**: Resonance flows rapidly through the system
//! - **High Diversity Index**: Many different resonance patterns in circulation
//! - **Balanced Creation/Decay**: Resonance creation matches natural decay
//! - **Active Cycles**: Natural economic cycles (creation, circulation, transformation, etc.)
//!
//! ## Economic Conditions
//!
//! - **Recession**: Low circulation velocity, low diversity, resonance stagnation
//! - **Boom**: High resonance generation, high circulation velocity, active trading
//! - **Inflation**: Excessive resonance creation without transformation
//! - **Deflation**: Excessive resonance decay without creation
//!
//! ## Integration with Other Systems
//!
//! This module integrates with:
//! - `catalyst_combat` - Catalyst generation from combat
//! - `archetypical_quests` - Catalyst rewards from quest completion
//! - `archetypical_crafting` - Resonance transformation through crafting
//! - `resonance_trading` - Resonance circulation through trading
//! - `polarity_factions` - Faction contributions and taxes

use crate::entity_layer7::layer7::EntityId;
use crate::simulation_v3::advanced_game_mechanics::{
    AdvancedGameMechanicsError, CatalystAmount, Float, Result, Timestamp,
};
use crate::simulation_v3::holographic_inventory::ResonancePattern;
use std::collections::HashMap;

// ============================================================================
// RESONANCE SOURCE
// ============================================================================

/// Source of resonance generation
///
/// Tracks where resonance originates in the system, allowing for
/// economic analysis of value creation channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceSource {
    /// Catalyst generated from combat/conflict
    CatalystGeneration,
    /// Resonance from archetypical crafting
    Crafting,
    /// Resonance from trading exchanges
    Trading,
    /// Resonance from quest completion
    QuestCompletion,
    /// Resonance from collective manifestation
    Building,
    /// Resonance from holding territory
    TerritorialControl,
    /// Resonance from contributing to faction
    FactionContribution,
    /// Resonance from natural environmental fields
    Environmental,
}

impl ResonanceSource {
    /// Get display name for this source
    pub fn display_name(&self) -> &'static str {
        match self {
            ResonanceSource::CatalystGeneration => "Catalyst Generation",
            ResonanceSource::Crafting => "Crafting",
            ResonanceSource::Trading => "Trading",
            ResonanceSource::QuestCompletion => "Quest Completion",
            ResonanceSource::Building => "Building",
            ResonanceSource::TerritorialControl => "Territorial Control",
            ResonanceSource::FactionContribution => "Faction Contribution",
            ResonanceSource::Environmental => "Environmental",
        }
    }

    /// Get base decay rate for this source
    ///
    /// Different sources have different decay characteristics:
    /// - CatalystGeneration: Faster decay (0.05 per time unit)
    /// - Crafting: Moderate decay (0.03 per time unit)
    /// - Trading: Slow decay (0.02 per time unit)
    /// - QuestCompletion: Very slow decay (0.01 per time unit)
    pub fn base_decay_rate(&self) -> Float {
        match self {
            ResonanceSource::CatalystGeneration => 0.05,
            ResonanceSource::Crafting => 0.03,
            ResonanceSource::Trading => 0.02,
            ResonanceSource::QuestCompletion => 0.01,
            ResonanceSource::Building => 0.025,
            ResonanceSource::TerritorialControl => 0.015,
            ResonanceSource::FactionContribution => 0.02,
            ResonanceSource::Environmental => 0.04,
        }
    }
}

impl std::fmt::Display for ResonanceSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// FLOW TYPE
// ============================================================================

/// How resonance is moving through the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowType {
    /// Newly generated resonance entering the system
    Creation,
    /// Resonance flowing between entities
    Circulation,
    /// Resonance being transformed or processed
    Transformation,
    /// Resonance being stored or accumulated
    Accumulation,
    /// Resonance dissipating from the system
    Decay,
    /// Resonance being absorbed by an entity
    Absorption,
}

impl FlowType {
    /// Get display name for this flow type
    pub fn display_name(&self) -> &'static str {
        match self {
            FlowType::Creation => "Creation",
            FlowType::Circulation => "Circulation",
            FlowType::Transformation => "Transformation",
            FlowType::Accumulation => "Accumulation",
            FlowType::Decay => "Decay",
            FlowType::Absorption => "Absorption",
        }
    }
}

impl std::fmt::Display for FlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// RESONANCE FLOW
// ============================================================================

/// A flow of resonance through the economic system
///
/// Tracks individual resonance units as they move through the system,
/// enabling detailed analysis of economic patterns and health.
#[derive(Debug, Clone)]
pub struct ResonanceFlow {
    /// Unique identifier for this flow
    pub flow_id: u64,
    /// What created this resonance
    pub source_type: ResonanceSource,
    /// ID of the source entity or event
    pub source_id: u64,
    /// Original amount of resonance
    pub resonance_amount: CatalystAmount,
    /// The pattern signature of this resonance
    pub resonance_pattern: ResonancePattern,
    /// When this resonance was created
    pub creation_time: Timestamp,
    /// How this resonance is moving
    pub flow_type: FlowType,
    /// How quickly this resonance decays (0.0-1.0 per time unit)
    pub decay_rate: Float,
    /// Current amount after decay
    pub current_amount: CatalystAmount,
}

impl ResonanceFlow {
    /// Create a new resonance flow
    pub fn new(
        flow_id: u64,
        source_type: ResonanceSource,
        source_id: u64,
        resonance_amount: CatalystAmount,
        resonance_pattern: ResonancePattern,
        creation_time: Timestamp,
        flow_type: FlowType,
    ) -> Self {
        let decay_rate = source_type.base_decay_rate();
        ResonanceFlow {
            flow_id,
            source_type,
            source_id,
            resonance_amount,
            resonance_pattern,
            creation_time,
            flow_type,
            decay_rate,
            current_amount: resonance_amount,
        }
    }

    /// Apply decay to this resonance flow based on time passed
    ///
    /// Uses exponential decay: current = original * e^(-decay_rate * time)
    pub fn apply_decay(&mut self, time_passed: Float) -> CatalystAmount {
        let decay_factor = (-self.decay_rate * time_passed).exp();
        self.current_amount = self.resonance_amount * decay_factor;
        self.current_amount
    }

    /// Check if this resonance flow is depleted (amount is negligible)
    pub fn is_depleted(&self) -> bool {
        self.current_amount < 0.01
    }

    /// Get the quality of this resonance based on pattern purity
    ///
    /// Quality is measured by the coherence of the resonance pattern:
    /// - Higher pattern values indicate stronger resonance
    /// - More balanced patterns have higher quality
    pub fn get_resonance_quality(&self) -> Float {
        // Calculate pattern coherence from the resonance pattern
        let sum: Float = self
            .resonance_pattern
            .pattern
            .iter()
            .map(|&v| v.abs())
            .sum();
        if sum == 0.0 {
            return 0.0;
        }

        // Higher frequency patterns (later indices) contribute more to quality
        let weighted_sum: Float = self
            .resonance_pattern
            .pattern
            .iter()
            .enumerate()
            .map(|(i, &v)| v.abs() * (i + 1) as Float / 8.0)
            .sum();

        (weighted_sum / sum).clamp(0.0, 1.0)
    }

    /// Get the age of this flow in time units
    pub fn age(&self, current_time: Timestamp) -> Float {
        (current_time - self.creation_time).max(0.0)
    }

    /// Get the proportion of original resonance remaining
    pub fn remaining_proportion(&self) -> Float {
        if self.resonance_amount == 0.0 {
            return 0.0;
        }
        (self.current_amount / self.resonance_amount).clamp(0.0, 1.0)
    }
}

// ============================================================================
// CYCLE TYPE
// ============================================================================

/// Type of economic cycle
///
/// Natural economic cycles that govern resonance flow patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleType {
    /// Resonance generation phase - new resonance entering system
    Creation,
    /// Resonance exchange phase - high trading and circulation
    Circulation,
    /// Resonance processing phase - crafting and transformation
    Transformation,
    /// Resonance storage phase - accumulation and consolidation
    Consolidation,
    /// Resonance distribution phase - release and redistribution
    Release,
}

impl CycleType {
    /// Get display name for this cycle type
    pub fn display_name(&self) -> &'static str {
        match self {
            CycleType::Creation => "Creation Cycle",
            CycleType::Circulation => "Circulation Cycle",
            CycleType::Transformation => "Transformation Cycle",
            CycleType::Consolidation => "Consolidation Cycle",
            CycleType::Release => "Release Cycle",
        }
    }

    /// Get typical duration of this cycle type (in time units)
    pub fn typical_duration(&self) -> Float {
        match self {
            CycleType::Creation => 100.0,
            CycleType::Circulation => 150.0,
            CycleType::Transformation => 80.0,
            CycleType::Consolidation => 60.0,
            CycleType::Release => 50.0,
        }
    }
}

impl std::fmt::Display for CycleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// CYCLE PHASE
// ============================================================================

/// Phase of an economic cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CyclePhase {
    /// Cycle is building up in strength
    Growing,
    /// Cycle at maximum strength
    Peak,
    /// Cycle is winding down
    Declining,
    /// Cycle is at minimal strength
    Dormant,
    /// Cycle is collapsing rapidly
    Collapsing,
}

impl CyclePhase {
    /// Get display name for this phase
    pub fn display_name(&self) -> &'static str {
        match self {
            CyclePhase::Growing => "Growing",
            CyclePhase::Peak => "Peak",
            CyclePhase::Declining => "Declining",
            CyclePhase::Dormant => "Dormant",
            CyclePhase::Collapsing => "Collapsing",
        }
    }
}

impl std::fmt::Display for CyclePhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// ECONOMIC CYCLE
// ============================================================================

/// A natural economic cycle in the resonance economy
///
/// Economic cycles emerge naturally from resonance flow patterns,
/// similar to how seasons emerge from planetary cycles.
#[derive(Debug, Clone)]
pub struct EconomicCycle {
    /// Unique identifier for this cycle
    pub cycle_id: u64,
    /// Type of this cycle
    pub cycle_type: CycleType,
    /// When this cycle started
    pub start_time: Timestamp,
    /// When this cycle ended (None if still active)
    pub end_time: Option<Timestamp>,
    /// Flows that occurred during this cycle
    pub flows: Vec<ResonanceFlow>,
    /// Current phase of this cycle
    pub cycle_phase: CyclePhase,
    /// Overall strength of this cycle (0.0-1.0)
    pub cycle_strength: Float,
}

impl EconomicCycle {
    /// Create a new economic cycle
    pub fn new(
        cycle_id: u64,
        cycle_type: CycleType,
        start_time: Timestamp,
        cycle_strength: Float,
    ) -> Self {
        EconomicCycle {
            cycle_id,
            cycle_type,
            start_time,
            end_time: None,
            flows: Vec::new(),
            cycle_phase: CyclePhase::Growing,
            cycle_strength: cycle_strength.clamp(0.0, 1.0),
        }
    }

    /// Add a flow to this cycle
    pub fn add_flow(&mut self, flow: ResonanceFlow) {
        self.flows.push(flow);
    }

    /// Calculate the total resonance that flowed through this cycle
    pub fn total_resonance(&self) -> CatalystAmount {
        self.flows.iter().map(|f| f.resonance_amount).sum()
    }

    /// Get the age of this cycle
    pub fn age(&self, current_time: Timestamp) -> Float {
        (current_time - self.start_time).max(0.0)
    }

    /// End this cycle
    pub fn end_cycle(&mut self, end_time: Timestamp) {
        self.end_time = Some(end_time);
        self.cycle_phase = CyclePhase::Dormant;
    }

    /// Advance the cycle phase based on age and typical duration
    pub fn advance_phase(&mut self, current_time: Timestamp) {
        let age = self.age(current_time);
        let duration = self.cycle_type.typical_duration();
        let progress = (age / duration).min(1.0);

        self.cycle_phase = if progress < 0.2 {
            CyclePhase::Growing
        } else if progress < 0.4 {
            CyclePhase::Peak
        } else if progress < 0.7 {
            CyclePhase::Declining
        } else if progress <= 0.9 {
            CyclePhase::Dormant
        } else {
            CyclePhase::Collapsing
        };
    }
}

// ============================================================================
// ACCOUNT TYPE
// ============================================================================

/// Type of resonance account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    /// Personal entity account
    Individual,
    /// Faction collective account
    Faction,
    /// Territory account
    Territory,
    /// System-wide account
    System,
}

impl AccountType {
    /// Get display name for this account type
    pub fn display_name(&self) -> &'static str {
        match self {
            AccountType::Individual => "Individual",
            AccountType::Faction => "Faction",
            AccountType::Territory => "Territory",
            AccountType::System => "System",
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// RESONANCE ACCOUNT
// ============================================================================

/// An entity's resonance account
///
/// Tracks an entity's total resonance holdings and the patterns
/// of resonance they possess.
#[derive(Debug, Clone)]
pub struct ResonanceAccount {
    /// ID of the entity that owns this account
    pub entity_id: EntityId,
    /// Total resonance held
    pub balance: CatalystAmount,
    /// Patterns of resonance held (pattern, amount)
    pub resonance_patterns: Vec<(ResonancePattern, CatalystAmount)>,
    /// Type of this account
    pub account_type: AccountType,
    /// When this account was created
    pub creation_time: Timestamp,
    /// When the last transaction occurred
    pub last_transaction_time: Timestamp,
}

impl ResonanceAccount {
    /// Create a new resonance account
    pub fn new(entity_id: EntityId, account_type: AccountType, creation_time: Timestamp) -> Self {
        ResonanceAccount {
            entity_id,
            balance: 0.0,
            resonance_patterns: Vec::new(),
            account_type,
            creation_time,
            last_transaction_time: creation_time,
        }
    }

    /// Add resonance to this account
    pub fn add_resonance(
        &mut self,
        amount: CatalystAmount,
        pattern: ResonancePattern,
        timestamp: Timestamp,
    ) {
        self.balance += amount;

        // Find if this pattern already exists (by comparing stability and phase)
        let mut found = false;
        for (p, existing_amount) in &mut self.resonance_patterns {
            if (p.stability - pattern.stability).abs() < 0.01
                && (p.phase - pattern.phase).abs() < 0.01
            {
                *existing_amount += amount;
                // Update the pattern values
                for i in 0..8 {
                    p.pattern[i] = (p.pattern[i] + pattern.pattern[i]) / 2.0;
                }
                found = true;
                break;
            }
        }

        if !found {
            self.resonance_patterns.push((pattern, amount));
        }

        self.last_transaction_time = timestamp;
    }

    /// Remove resonance from this account
    ///
    /// Returns error if insufficient balance
    pub fn remove_resonance(
        &mut self,
        amount: CatalystAmount,
        pattern: ResonancePattern,
        timestamp: Timestamp,
    ) -> Result<CatalystAmount> {
        if self.balance < amount {
            return Err(AdvancedGameMechanicsError::InsufficientCatalyst {
                required: amount,
                current: self.balance,
            });
        }

        // Find and remove from matching pattern
        let actual_removed = amount.min(self.get_pattern_amount(&pattern));

        for i in 0..self.resonance_patterns.len() {
            let (p, existing_amount) = &self.resonance_patterns[i];
            if (p.stability - pattern.stability).abs() < 0.01
                && (p.phase - pattern.phase).abs() < 0.01
            {
                let new_amount = *existing_amount - actual_removed;
                if new_amount <= 0.01 {
                    self.resonance_patterns.remove(i);
                    break;
                } else {
                    self.resonance_patterns[i].1 = new_amount;
                    break;
                }
            }
        }

        self.balance -= actual_removed;
        self.last_transaction_time = timestamp;

        Ok(actual_removed)
    }

    /// Get the account's resonance patterns
    pub fn get_resonance_patterns(&self) -> Vec<ResonancePattern> {
        self.resonance_patterns
            .iter()
            .map(|(p, _)| p.clone())
            .collect()
    }

    /// Get the amount of a specific pattern held
    pub fn get_pattern_amount(&self, pattern: &ResonancePattern) -> CatalystAmount {
        // Find pattern by comparing stability and phase
        for (p, amount) in &self.resonance_patterns {
            if (p.stability - pattern.stability).abs() < 0.01
                && (p.phase - pattern.phase).abs() < 0.01
            {
                return *amount;
            }
        }
        0.0
    }

    /// Get the age of this account
    pub fn age(&self, current_time: Timestamp) -> Float {
        (current_time - self.creation_time).max(0.0)
    }

    /// Get the time since last transaction
    pub fn time_since_last_transaction(&self, current_time: Timestamp) -> Float {
        (current_time - self.last_transaction_time).max(0.0)
    }
}

// ============================================================================
// TRANSACTION TYPE
// ============================================================================

/// Type of resonance transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Resonance exchange between entities
    Trade,
    /// New resonance created in the system
    CatalystGeneration,
    /// Resonance transferred between accounts
    CatalystTransfer,
    /// Resonance from quest completion
    QuestReward,
    /// Faction tax contribution
    FactionTax,
    /// Territory income
    TerritoryBonus,
    /// Resonance loss from decay
    Decay,
}

impl TransactionType {
    /// Get display name for this transaction type
    pub fn display_name(&self) -> &'static str {
        match self {
            TransactionType::Trade => "Trade",
            TransactionType::CatalystGeneration => "Catalyst Generation",
            TransactionType::CatalystTransfer => "Catalyst Transfer",
            TransactionType::QuestReward => "Quest Reward",
            TransactionType::FactionTax => "Faction Tax",
            TransactionType::TerritoryBonus => "Territory Bonus",
            TransactionType::Decay => "Decay",
        }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================================
// RESONANCE TRANSACTION
// ============================================================================

/// A transaction in the resonance ledger
///
/// Records all resonance flows for transparency and economic analysis.
#[derive(Debug, Clone)]
pub struct ResonanceTransaction {
    /// Unique identifier for this transaction
    pub transaction_id: u64,
    /// Account sending resonance (or 0 for system generation)
    pub from_account: EntityId,
    /// Account receiving resonance (or 0 for system consumption)
    pub to_account: EntityId,
    /// Amount of resonance transferred
    pub amount: CatalystAmount,
    /// Pattern of resonance transferred
    pub resonance_pattern: ResonancePattern,
    /// Type of this transaction
    pub transaction_type: TransactionType,
    /// When this transaction occurred
    pub timestamp: Timestamp,
    /// Transaction fee (if applicable)
    pub transaction_fee: CatalystAmount,
}

impl ResonanceTransaction {
    /// Create a new resonance transaction
    pub fn new(
        transaction_id: u64,
        from_account: EntityId,
        to_account: EntityId,
        amount: CatalystAmount,
        resonance_pattern: ResonancePattern,
        transaction_type: TransactionType,
        timestamp: Timestamp,
    ) -> Self {
        ResonanceTransaction {
            transaction_id,
            from_account,
            to_account,
            amount,
            resonance_pattern,
            transaction_type,
            timestamp,
            transaction_fee: 0.0,
        }
    }

    /// Set the transaction fee
    pub fn with_fee(mut self, fee: CatalystAmount) -> Self {
        self.transaction_fee = fee;
        self
    }

    /// Get the net amount transferred (amount minus fee)
    pub fn net_amount(&self) -> CatalystAmount {
        (self.amount - self.transaction_fee).max(0.0)
    }
}

// ============================================================================
// RESONANCE LEDGER
// ============================================================================

/// Accounting system for resonance exchanges
///
/// The ledger tracks all resonance flows in the system, providing
/// transparency and enabling economic analysis.
#[derive(Debug, Clone)]
pub struct ResonanceLedger {
    /// Unique identifier for this ledger
    pub ledger_id: u64,
    /// Entity accounts
    pub entity_accounts: HashMap<EntityId, ResonanceAccount>,
    /// Transaction history
    pub transaction_history: Vec<ResonanceTransaction>,
    /// Total resonance in the system
    pub total_resonance_in_system: CatalystAmount,
    /// How fast resonance is circulating (transactions per time unit)
    pub circulation_velocity: Float,
    /// How diverse resonance patterns are (0.0-1.0)
    pub diversity_index: Float,
    /// Next transaction ID
    next_transaction_id: u64,
}

impl ResonanceLedger {
    /// Create a new resonance ledger
    pub fn new(ledger_id: u64) -> Self {
        ResonanceLedger {
            ledger_id,
            entity_accounts: HashMap::new(),
            transaction_history: Vec::new(),
            total_resonance_in_system: 0.0,
            circulation_velocity: 0.0,
            diversity_index: 0.0,
            next_transaction_id: 1,
        }
    }

    /// Create a new account for an entity
    pub fn create_account(
        &mut self,
        entity_id: EntityId,
        account_type: AccountType,
        creation_time: Timestamp,
    ) -> ResonanceAccount {
        let account = ResonanceAccount::new(entity_id.clone(), account_type, creation_time);
        self.entity_accounts.insert(entity_id, account.clone());
        account
    }

    /// Record a transaction in the ledger
    pub fn record_transaction(&mut self, transaction: ResonanceTransaction) -> Result<()> {
        // Extract fields to avoid borrow issues
        let from_account_id = transaction.from_account.clone();
        let to_account_id = transaction.to_account.clone();
        let transaction_type = transaction.transaction_type;
        let amount = transaction.amount;
        let transaction_fee = transaction.transaction_fee;
        let total_resonance_before = self.total_resonance_in_system;

        // Update from account
        if from_account_id.uuid != "system" && from_account_id.uuid != "decay" {
            let from_account = self
                .entity_accounts
                .get_mut(&from_account_id)
                .ok_or(AdvancedGameMechanicsError::EntityNotFound(from_account_id))?;

            from_account.remove_resonance(
                amount + transaction_fee,
                transaction.resonance_pattern.clone(),
                transaction.timestamp,
            )?;
        }

        // Update to account
        if to_account_id.uuid != "system" && to_account_id.uuid != "decay" {
            let net_amount = transaction.net_amount();
            let pattern = transaction.resonance_pattern.clone();
            let timestamp = transaction.timestamp;

            let to_account = self
                .entity_accounts
                .get_mut(&to_account_id)
                .ok_or(AdvancedGameMechanicsError::EntityNotFound(to_account_id))?;

            to_account.add_resonance(net_amount, pattern, timestamp);
        }

        // Update system totals for system transactions
        match transaction_type {
            TransactionType::CatalystGeneration | TransactionType::QuestReward => {
                self.total_resonance_in_system += amount;
            }
            TransactionType::Decay => {
                self.total_resonance_in_system -= amount.min(total_resonance_before);
            }
            _ => {}
        }

        // Record transaction
        self.transaction_history.push(transaction);

        // Update metrics
        self.calculate_system_diversity();
        self.calculate_circulation_velocity();

        Ok(())
    }

    /// Get an account's balance
    pub fn get_account_balance(&self, entity_id: EntityId) -> CatalystAmount {
        self.entity_accounts
            .get(&entity_id)
            .map(|acc| acc.balance)
            .unwrap_or(0.0)
    }

    /// Get an account's resonance patterns
    pub fn get_account_resonance_patterns(&self, entity_id: EntityId) -> Vec<ResonancePattern> {
        self.entity_accounts
            .get(&entity_id)
            .map(|acc| acc.get_resonance_patterns())
            .unwrap_or_default()
    }

    /// Calculate the diversity index of resonance patterns in the system
    ///
    /// Uses Shannon entropy to measure diversity:
    /// Higher values indicate more diverse patterns (better for economy)
    pub fn calculate_system_diversity(&mut self) -> Float {
        let total = self
            .entity_accounts
            .values()
            .flat_map(|acc| acc.resonance_patterns.iter().map(|(_, amount)| amount))
            .sum::<CatalystAmount>();

        if total == 0.0 {
            self.diversity_index = 0.0;
            return 0.0;
        }

        let mut pattern_amounts: HashMap<String, CatalystAmount> = HashMap::new();

        for account in self.entity_accounts.values() {
            for (pattern, amount) in &account.resonance_patterns {
                let pattern_key = format!("{:?},{:?}", pattern.stability, pattern.phase);
                *pattern_amounts.entry(pattern_key).or_insert(0.0) += amount;
            }
        }

        let entropy: Float = pattern_amounts
            .values()
            .map(|&amount| {
                let p = amount / total;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum();

        // Normalize entropy by maximum possible entropy
        let max_entropy = (pattern_amounts.len() as Float).log2();
        self.diversity_index = if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        };

        self.diversity_index
    }

    /// Calculate the circulation velocity of resonance
    ///
    /// Measures how fast resonance is moving through the system
    /// (transactions per time unit)
    pub fn calculate_circulation_velocity(&mut self) -> Float {
        if self.transaction_history.is_empty() {
            self.circulation_velocity = 0.0;
            return 0.0;
        }

        let recent_transactions: Vec<_> = self
            .transaction_history
            .iter()
            .filter(|t| t.timestamp > self.transaction_history.last().unwrap().timestamp - 100.0)
            .collect();

        self.circulation_velocity = recent_transactions.len() as Float / 100.0;
        self.circulation_velocity
    }

    /// Get the top resonance holders
    pub fn get_top_resonance_holders(&self, count: usize) -> Vec<(EntityId, CatalystAmount)> {
        let mut holders: Vec<_> = self
            .entity_accounts
            .iter()
            .map(|(id, acc)| (id.clone(), acc.balance))
            .collect();

        holders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        holders.truncate(count);
        holders
    }

    /// Get total transactions in the ledger
    pub fn total_transactions(&self) -> usize {
        self.transaction_history.len()
    }

    /// Get next transaction ID
    pub fn next_transaction_id(&mut self) -> u64 {
        let id = self.next_transaction_id;
        self.next_transaction_id += 1;
        id
    }
}

// ============================================================================
// CYCLE STATUS
// ============================================================================

/// Current economic cycle status
#[derive(Debug, Clone)]
pub struct CycleStatus {
    /// Current cycle type
    pub current_cycle: CycleType,
    /// Current phase of the cycle
    pub cycle_phase: CyclePhase,
    /// Strength of the current cycle (0.0-1.0)
    pub cycle_strength: Float,
    /// Estimated duration of current cycle
    pub estimated_duration: Float,
}

impl CycleStatus {
    /// Create a new cycle status
    pub fn new(current_cycle: CycleType, cycle_phase: CyclePhase, cycle_strength: Float) -> Self {
        let estimated_duration = current_cycle.typical_duration();
        CycleStatus {
            current_cycle,
            cycle_phase,
            cycle_strength: cycle_strength.clamp(0.0, 1.0),
            estimated_duration,
        }
    }
}

// ============================================================================
// ECONOMIC METRICS
// ============================================================================

/// Tracks economic health indicators
///
/// Provides comprehensive metrics for analyzing the health of the
/// resonance economy.
#[derive(Debug, Clone, PartialEq)]
pub struct EconomicMetrics {
    /// Total resonance in the system
    pub total_resonance: CatalystAmount,
    /// How fast resonance is circulating
    pub circulation_velocity: Float,
    /// How diverse resonance patterns are (0.0-1.0)
    pub diversity_index: Float,
    /// Resonance created per time unit
    pub creation_rate: Float,
    /// Resonance lost per time unit
    pub decay_rate: Float,
    /// Overall economic growth/decline rate
    pub economic_growth_rate: Float,
    /// Probability of economic recession
    pub recession_probability: Float,
    /// Probability of economic boom
    pub boom_probability: Float,
}

impl EconomicMetrics {
    /// Create new economic metrics with default values
    pub fn new() -> Self {
        EconomicMetrics {
            total_resonance: 0.0,
            circulation_velocity: 0.0,
            diversity_index: 0.0,
            creation_rate: 0.0,
            decay_rate: 0.0,
            economic_growth_rate: 0.0,
            recession_probability: 0.0,
            boom_probability: 0.0,
        }
    }

    /// Calculate overall economic health score (0.0-1.0)
    ///
    /// Health is determined by:
    /// - High circulation velocity (30% weight)
    /// - High diversity index (30% weight)
    /// - Balanced creation/decay ratio (20% weight)
    /// - Positive economic growth (20% weight)
    pub fn calculate_health_score(&self) -> Float {
        let velocity_score = self.circulation_velocity.min(1.0);
        let diversity_score = self.diversity_index;
        let balance_score = if self.creation_rate > 0.0 {
            1.0 - (self.decay_rate / self.creation_rate).min(1.0).abs()
        } else {
            0.5
        };
        let growth_score = ((self.economic_growth_rate + 1.0) / 2.0).clamp(0.0, 1.0);

        velocity_score * 0.3 + diversity_score * 0.3 + balance_score * 0.2 + growth_score * 0.2
    }

    /// Detect if the economy is in recession
    ///
    /// Recession indicators:
    /// - Low circulation velocity (< 0.3)
    /// - Low diversity index (< 0.3)
    /// - Negative economic growth rate
    /// - Decay rate higher than creation rate
    pub fn detect_recession(&self) -> bool {
        self.circulation_velocity < 0.3
            && self.diversity_index < 0.3
            && self.economic_growth_rate < 0.0
            && self.decay_rate > self.creation_rate
    }

    /// Detect if the economy is in boom
    ///
    /// Boom indicators:
    /// - High circulation velocity (> 0.7)
    /// - High diversity index (> 0.7)
    /// - Positive economic growth rate
    /// - Creation rate significantly higher than decay rate
    pub fn detect_boom(&self) -> bool {
        self.circulation_velocity > 0.7
            && self.diversity_index > 0.7
            && self.economic_growth_rate > 0.1
            && self.creation_rate > self.decay_rate * 1.5
    }

    /// Get current cycle status
    pub fn get_cycle_status(&self) -> CycleStatus {
        // Determine cycle type based on creation/decay ratio
        let ratio = if self.decay_rate > 0.0 {
            self.creation_rate / self.decay_rate
        } else {
            2.0
        };

        let (cycle_type, cycle_phase, strength) = if ratio > 1.5 {
            (
                CycleType::Creation,
                CyclePhase::Growing,
                ratio.min(2.0) / 2.0,
            )
        } else if ratio > 1.0 {
            (
                CycleType::Circulation,
                CyclePhase::Peak,
                self.circulation_velocity,
            )
        } else if ratio > 0.8 {
            (
                CycleType::Transformation,
                CyclePhase::Declining,
                self.diversity_index,
            )
        } else if ratio > 0.5 {
            (CycleType::Consolidation, CyclePhase::Dormant, 0.5)
        } else {
            (CycleType::Release, CyclePhase::Collapsing, ratio)
        };

        CycleStatus::new(cycle_type, cycle_phase, strength)
    }
}

impl Default for EconomicMetrics {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ENTITY ECONOMIC STATUS
// ============================================================================

/// Economic status of a single entity
#[derive(Debug, Clone)]
pub struct EntityEconomicStatus {
    /// Entity ID
    pub entity_id: EntityId,
    /// Total resonance balance
    pub balance: CatalystAmount,
    /// Resonance patterns held
    pub resonance_patterns: Vec<ResonancePattern>,
    /// Account type
    pub account_type: AccountType,
    /// Account age
    pub account_age: Timestamp,
    /// Time since last transaction
    pub time_since_last_transaction: Timestamp,
    /// Rank in resonance holdings (optional)
    pub rank: Option<usize>,
    /// Resonance quality score
    pub resonance_quality: Float,
}

impl EntityEconomicStatus {
    /// Create a new entity economic status
    pub fn new(
        entity_id: EntityId,
        balance: CatalystAmount,
        resonance_patterns: Vec<ResonancePattern>,
        account_type: AccountType,
        account_age: Timestamp,
        time_since_last_transaction: Timestamp,
        rank: Option<usize>,
    ) -> Self {
        // Calculate average resonance quality
        let resonance_quality = if resonance_patterns.is_empty() {
            0.0
        } else {
            let total_quality: Float = resonance_patterns
                .iter()
                .map(|pattern| {
                    let sum: Float = pattern.pattern.iter().map(|&v| v.abs()).sum();
                    if sum == 0.0 {
                        return 0.0;
                    }
                    let weighted_sum: Float = pattern
                        .pattern
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| v.abs() * (i + 1) as Float / 8.0)
                        .sum();
                    (weighted_sum / sum).min(1.0)
                })
                .sum();
            total_quality / resonance_patterns.len() as Float
        };

        EntityEconomicStatus {
            entity_id,
            balance,
            resonance_patterns,
            account_type,
            account_age,
            time_since_last_transaction,
            rank,
            resonance_quality,
        }
    }
}

// ============================================================================
// ECONOMY HEALTH REPORT
// ============================================================================

/// Comprehensive report on the economy's health
#[derive(Debug, Clone)]
pub struct EconomyHealthReport {
    /// Current economic metrics
    pub metrics: EconomicMetrics,
    /// Current cycle status
    pub cycle_status: CycleStatus,
    /// Overall health score (0.0-1.0)
    pub health_score: Float,
    /// Is economy in recession?
    pub in_recession: bool,
    /// Is economy in boom?
    pub in_boom: bool,
    /// Total number of entities with accounts
    pub total_entities: usize,
    /// Top resonance holders
    pub top_holders: Vec<(EntityId, CatalystAmount)>,
    /// Recommendations for economic management
    pub recommendations: Vec<String>,
}

impl EconomyHealthReport {
    /// Create a new economy health report
    pub fn new(
        metrics: EconomicMetrics,
        cycle_status: CycleStatus,
        total_entities: usize,
        top_holders: Vec<(EntityId, CatalystAmount)>,
    ) -> Self {
        let health_score = metrics.calculate_health_score();
        let in_recession = metrics.detect_recession();
        let in_boom = metrics.detect_boom();

        // Generate recommendations
        let mut recommendations = Vec::new();

        if in_recession {
            recommendations
                .push("Consider stimulating the economy by injecting resonance".to_string());
            recommendations.push("Encourage trading and crafting activities".to_string());
            recommendations.push("Launch quests to generate catalyst rewards".to_string());
        }

        if in_boom {
            recommendations
                .push("Consider contracting the economy to prevent inflation".to_string());
            recommendations.push("Increase decay rates slightly".to_string());
        }

        if metrics.circulation_velocity < 0.5 {
            recommendations.push("Increase trading incentives to boost circulation".to_string());
        }

        if metrics.diversity_index < 0.5 {
            recommendations.push("Encourage diverse crafting and quest activities".to_string());
        }

        if metrics.decay_rate > metrics.creation_rate * 1.2 {
            recommendations
                .push("Resonance decay is high - increase creation activities".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Economy is balanced - maintain current policies".to_string());
        }

        EconomyHealthReport {
            metrics,
            cycle_status,
            health_score,
            in_recession,
            in_boom,
            total_entities,
            top_holders,
            recommendations,
        }
    }
}

// ============================================================================
// RESONANCE ECONOMY
// ============================================================================

/// Main economy system
///
/// Orchestrates all resonance flow in the system, from creation
/// through circulation to transformation and decay.
#[derive(Debug, Clone)]
pub struct ResonanceEconomy {
    /// Accounting ledger for all transactions
    pub ledger: ResonanceLedger,
    /// Active resonance flows
    pub active_flows: Vec<ResonanceFlow>,
    /// Economic cycles
    pub economic_cycles: Vec<EconomicCycle>,
    /// Current economic metrics
    pub metrics: EconomicMetrics,
    /// Next flow ID
    next_flow_id: u64,
    /// Next cycle ID
    next_cycle_id: u64,
    /// Current time
    current_time: Timestamp,
}

impl ResonanceEconomy {
    /// Create a new resonance economy
    pub fn new(ledger_id: u64) -> Self {
        ResonanceEconomy {
            ledger: ResonanceLedger::new(ledger_id),
            active_flows: Vec::new(),
            economic_cycles: Vec::new(),
            metrics: EconomicMetrics::new(),
            next_flow_id: 1,
            next_cycle_id: 1,
            current_time: 0.0,
        }
    }

    /// Create a new account
    pub fn create_account(
        &mut self,
        entity_id: EntityId,
        account_type: AccountType,
    ) -> Result<ResonanceAccount> {
        let account = self
            .ledger
            .create_account(entity_id, account_type, self.current_time);
        Ok(account)
    }

    /// Record a trade between two entities
    pub fn record_trade(
        &mut self,
        from_id: EntityId,
        to_id: EntityId,
        amount: CatalystAmount,
        pattern: ResonancePattern,
    ) -> Result<ResonanceTransaction> {
        let from_id_clone = from_id.clone();
        let transaction = ResonanceTransaction::new(
            self.ledger.next_transaction_id(),
            from_id_clone.clone(),
            to_id,
            amount,
            pattern.clone(),
            TransactionType::Trade,
            self.current_time,
        );

        self.ledger.record_transaction(transaction.clone())?;

        // Create flow for circulation
        let flow = ResonanceFlow::new(
            self.next_flow_id,
            ResonanceSource::Trading,
            from_id_clone.as_u64(),
            amount,
            pattern.clone(),
            self.current_time,
            FlowType::Circulation,
        );
        self.active_flows.push(flow);
        self.next_flow_id += 1;

        Ok(transaction)
    }

    /// Record catalyst generation (new resonance in the system)
    pub fn record_catalyst_generation(
        &mut self,
        entity_id: EntityId,
        amount: CatalystAmount,
        pattern: ResonancePattern,
        source: ResonanceSource,
    ) -> Result<ResonanceTransaction> {
        let transaction = ResonanceTransaction::new(
            self.ledger.next_transaction_id(),
            EntityId::new("system".to_string()), // System
            entity_id.clone(),
            amount,
            pattern.clone(),
            TransactionType::CatalystGeneration,
            self.current_time,
        );

        self.ledger.record_transaction(transaction.clone())?;

        // Create flow for creation
        let flow = ResonanceFlow::new(
            self.next_flow_id,
            source,
            0, // source_id is just a numeric identifier
            amount,
            pattern,
            self.current_time,
            FlowType::Creation,
        );
        self.active_flows.push(flow);
        self.next_flow_id += 1;

        Ok(transaction)
    }

    /// Record a quest reward
    pub fn record_quest_reward(
        &mut self,
        entity_id: EntityId,
        amount: CatalystAmount,
        pattern: ResonancePattern,
    ) -> Result<ResonanceTransaction> {
        let transaction = ResonanceTransaction::new(
            self.ledger.next_transaction_id(),
            EntityId::new("system".to_string()), // System
            entity_id.clone(),
            amount,
            pattern.clone(),
            TransactionType::QuestReward,
            self.current_time,
        );

        self.ledger.record_transaction(transaction.clone())?;

        // Create flow for creation
        let flow = ResonanceFlow::new(
            self.next_flow_id,
            ResonanceSource::QuestCompletion,
            entity_id.as_u64(),
            amount,
            pattern,
            self.current_time,
            FlowType::Creation,
        );
        self.active_flows.push(flow);
        self.next_flow_id += 1;

        Ok(transaction)
    }

    /// Apply decay to all active flows
    ///
    /// Returns total amount decayed
    pub fn apply_decay(&mut self, time_passed: Float) -> Result<CatalystAmount> {
        let mut total_decayed = 0.0;

        // Apply decay to flows
        for flow in &mut self.active_flows {
            let decayed = flow.resonance_amount - flow.apply_decay(time_passed);
            total_decayed += decayed;
        }

        // Remove depleted flows
        self.active_flows.retain(|f| !f.is_depleted());

        // Record decay transaction and update system total
        if total_decayed > 0.01 {
            // Create decay pattern (all zeros - represents loss)
            let decay_pattern = ResonancePattern::new();

            let transaction = ResonanceTransaction::new(
                self.ledger.next_transaction_id(),
                EntityId::new("system".to_string()), // System accounts
                EntityId::new("decay".to_string()),  // Decay goes to system
                total_decayed,
                decay_pattern,
                TransactionType::Decay,
                self.current_time,
            );

            // Update system resonance total
            self.ledger.total_resonance_in_system =
                (self.ledger.total_resonance_in_system - total_decayed).max(0.0);

            // Add to transaction history for tracking
            self.ledger.transaction_history.push(transaction);
        }

        self.current_time += time_passed;
        Ok(total_decayed)
    }

    /// Process pending transactions
    pub fn process_transactions(&mut self) -> Result<()> {
        // All transactions are processed immediately when recorded
        // This method is for consistency with other systems
        Ok(())
    }

    /// Calculate current economic metrics
    pub fn calculate_economic_metrics(&mut self) -> EconomicMetrics {
        let total_resonance = self.ledger.total_resonance_in_system;
        let circulation_velocity = self.ledger.circulation_velocity;
        let diversity_index = self.ledger.diversity_index;

        // Calculate creation rate (resonance created in last 100 time units)
        let creation_transactions: Vec<_> = self
            .ledger
            .transaction_history
            .iter()
            .filter(|t| {
                matches!(
                    t.transaction_type,
                    TransactionType::CatalystGeneration | TransactionType::QuestReward
                ) && t.timestamp > self.current_time - 100.0
            })
            .collect();

        let creation_rate: Float = creation_transactions
            .iter()
            .map(|t| t.amount)
            .sum::<CatalystAmount>()
            / 100.0;

        // Calculate decay rate (resonance lost in last 100 time units)
        let decay_transactions: Vec<_> = self
            .ledger
            .transaction_history
            .iter()
            .filter(|t| {
                matches!(t.transaction_type, TransactionType::Decay)
                    && t.timestamp > self.current_time - 100.0
            })
            .collect();

        let decay_rate: Float = decay_transactions
            .iter()
            .map(|t| t.amount)
            .sum::<CatalystAmount>()
            / 100.0;

        // Calculate economic growth rate
        let previous_total = if self.ledger.transaction_history.len() > 10 {
            let recent_time = self.current_time - 50.0;
            let old_transactions: Vec<_> = self
                .ledger
                .transaction_history
                .iter()
                .filter(|t| t.timestamp < recent_time)
                .collect();

            let old_created: Float = old_transactions
                .iter()
                .filter(|t| {
                    matches!(
                        t.transaction_type,
                        TransactionType::CatalystGeneration | TransactionType::QuestReward
                    )
                })
                .map(|t| t.amount)
                .sum();

            let old_decayed: Float = old_transactions
                .iter()
                .filter(|t| matches!(t.transaction_type, TransactionType::Decay))
                .map(|t| t.amount)
                .sum();

            old_created - old_decayed
        } else {
            0.0
        };

        let current_net = creation_rate - decay_rate;
        let economic_growth_rate = if previous_total > 0.0 {
            (current_net - previous_total) / previous_total.abs()
        } else {
            0.0
        };

        // Calculate recession and boom probabilities
        let recession_probability = if circulation_velocity < 0.3 && diversity_index < 0.3 {
            0.8
        } else if circulation_velocity < 0.5 && diversity_index < 0.5 {
            0.5
        } else {
            0.1
        };

        let boom_probability = if circulation_velocity > 0.7 && diversity_index > 0.7 {
            0.8
        } else if circulation_velocity > 0.5 && diversity_index > 0.5 {
            0.5
        } else {
            0.1
        };

        self.metrics = EconomicMetrics {
            total_resonance,
            circulation_velocity,
            diversity_index,
            creation_rate,
            decay_rate,
            economic_growth_rate,
            recession_probability,
            boom_probability,
        };

        self.metrics.clone()
    }

    /// Detect active economic cycles
    pub fn detect_economic_cycles(&mut self) -> Vec<EconomicCycle> {
        let mut cycles = Vec::new();

        // Check for creation cycle
        if self.metrics.creation_rate > self.metrics.decay_rate * 1.5 {
            let cycle = EconomicCycle::new(
                self.next_cycle_id,
                CycleType::Creation,
                self.current_time,
                self.metrics.creation_rate / (self.metrics.creation_rate + self.metrics.decay_rate),
            );
            self.economic_cycles.push(cycle.clone());
            cycles.push(cycle);
            self.next_cycle_id += 1;
        }

        // Check for circulation cycle
        if self.metrics.circulation_velocity > 0.6 {
            let cycle = EconomicCycle::new(
                self.next_cycle_id,
                CycleType::Circulation,
                self.current_time,
                self.metrics.circulation_velocity,
            );
            self.economic_cycles.push(cycle.clone());
            cycles.push(cycle);
            self.next_cycle_id += 1;
        }

        // Check for transformation cycle
        if self.metrics.diversity_index > 0.6 {
            let cycle = EconomicCycle::new(
                self.next_cycle_id,
                CycleType::Transformation,
                self.current_time,
                self.metrics.diversity_index,
            );
            self.economic_cycles.push(cycle.clone());
            cycles.push(cycle);
            self.next_cycle_id += 1;
        }

        cycles
    }

    /// Stimulate the economy (inject resonance to combat recession)
    pub fn stimulate_economy(&mut self, stimulus: CatalystAmount) -> Result<()> {
        // Distribute stimulus to active entities
        let active_entities: Vec<_> = self.ledger.entity_accounts.keys().cloned().collect();

        if active_entities.is_empty() {
            return Ok(());
        }

        let per_entity = stimulus / active_entities.len() as Float;
        let stimulus_pattern = ResonancePattern::new();

        for entity_id in active_entities {
            let transaction = ResonanceTransaction::new(
                self.ledger.next_transaction_id(),
                EntityId::new("system".to_string()), // System
                entity_id.clone(),
                per_entity,
                stimulus_pattern.clone(),
                TransactionType::CatalystGeneration,
                self.current_time,
            );

            self.ledger.record_transaction(transaction)?;
        }

        Ok(())
    }

    /// Contract the economy (remove resonance to combat inflation)
    pub fn contract_economy(&mut self, contraction: CatalystAmount) -> Result<()> {
        // Remove resonance from top holders
        let mut top_holders = self.ledger.get_top_resonance_holders(10);

        let mut remaining = contraction;

        for (entity_id, _) in &mut top_holders {
            if remaining <= 0.0 {
                break;
            }

            let balance = self.ledger.get_account_balance(entity_id.clone());
            let to_remove = (balance * 0.1).min(remaining);

            if to_remove > 0.0 {
                let patterns = self
                    .ledger
                    .get_account_resonance_patterns(entity_id.clone());
                if !patterns.is_empty() {
                    let pattern = patterns[0].clone();
                    let transaction = ResonanceTransaction::new(
                        self.ledger.next_transaction_id(),
                        entity_id.clone(),
                        EntityId::new("decay".to_string()), // System
                        to_remove,
                        pattern,
                        TransactionType::Decay,
                        self.current_time,
                    );

                    // Manually deduct from account
                    if let Some(account) = self.ledger.entity_accounts.get_mut(entity_id) {
                        account.balance -= to_remove;
                        self.ledger.total_resonance_in_system -= to_remove;
                    }

                    self.ledger.transaction_history.push(transaction);
                    remaining -= to_remove;
                }
            }
        }

        Ok(())
    }

    /// Get comprehensive economy health report
    pub fn get_economy_health_report(&mut self) -> EconomyHealthReport {
        self.calculate_economic_metrics();
        let cycle_status = self.metrics.get_cycle_status();
        let total_entities = self.ledger.entity_accounts.len();
        let top_holders = self.ledger.get_top_resonance_holders(10);

        EconomyHealthReport::new(
            self.metrics.clone(),
            cycle_status,
            total_entities,
            top_holders,
        )
    }

    /// Get economic status for a specific entity
    pub fn get_entity_economic_status(
        &mut self,
        entity_id: EntityId,
    ) -> Option<EntityEconomicStatus> {
        let account = self.ledger.entity_accounts.get(&entity_id)?;

        let rank = self
            .ledger
            .get_top_resonance_holders(usize::MAX)
            .iter()
            .position(|(id, _)| *id == entity_id)
            .map(|pos| pos + 1);

        Some(EntityEconomicStatus::new(
            entity_id,
            account.balance,
            account.get_resonance_patterns(),
            account.account_type,
            account.age(self.current_time),
            account.time_since_last_transaction(self.current_time),
            rank,
        ))
    }

    /// Advance time in the economy
    pub fn advance_time(&mut self, time_passed: Float) -> Result<()> {
        self.apply_decay(time_passed)?;
        self.calculate_economic_metrics();

        // Advance cycles
        for cycle in &mut self.economic_cycles {
            cycle.advance_phase(self.current_time);
        }

        Ok(())
    }
}

impl Default for ResonanceEconomy {
    fn default() -> Self {
        Self::new(1)
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_economy() -> ResonanceEconomy {
        ResonanceEconomy::new(1)
    }

    fn create_test_pattern() -> ResonancePattern {
        let mut pattern = ResonancePattern::new();
        // Add some resonance activation in the pattern array
        pattern.pattern[0] = 0.5;
        pattern.pattern[3] = 0.7;
        pattern.pattern[7] = 0.8;
        pattern.stability = 0.9;
        pattern.phase = 0.5;
        pattern
    }

    #[test]
    fn test_create_resonance_flow() {
        let flow = ResonanceFlow::new(
            1,
            ResonanceSource::CatalystGeneration,
            100,
            100.0,
            create_test_pattern(),
            0.0,
            FlowType::Creation,
        );

        assert_eq!(flow.flow_id, 1);
        assert_eq!(flow.resonance_amount, 100.0);
        assert_eq!(flow.current_amount, 100.0);
        assert!(!flow.is_depleted());
    }

    #[test]
    fn test_apply_decay() {
        let mut flow = ResonanceFlow::new(
            1,
            ResonanceSource::CatalystGeneration,
            100,
            100.0,
            create_test_pattern(),
            0.0,
            FlowType::Creation,
        );

        let decayed = flow.apply_decay(10.0);
        assert!(decayed < 100.0);
        assert!(flow.current_amount < flow.resonance_amount);
    }

    #[test]
    fn test_flow_depletion() {
        let mut flow = ResonanceFlow::new(
            1,
            ResonanceSource::CatalystGeneration,
            100,
            100.0,
            create_test_pattern(),
            0.0,
            FlowType::Creation,
        );

        // Apply enough decay to deplete
        // With decay_rate=0.05 and original=100.0, need about 185 time units to get below 0.01
        // Formula: 100 * e^(-0.05 * t) < 0.01
        // e^(-0.05 * t) < 0.0001
        // -0.05 * t < ln(0.0001) ≈ -9.21
        // t > 184.2
        flow.apply_decay(200.0);
        assert!(flow.is_depleted());
    }

    #[test]
    fn test_resonance_quality() {
        let flow = ResonanceFlow::new(
            1,
            ResonanceSource::QuestCompletion,
            100,
            100.0,
            create_test_pattern(),
            0.0,
            FlowType::Creation,
        );

        let quality = flow.get_resonance_quality();
        assert!(quality >= 0.0);
        assert!(quality <= 1.0);
    }

    #[test]
    fn test_create_resonance_account() {
        let mut economy = create_test_economy();
        let account = economy
            .create_account(
                EntityId::new("test-resonance-0".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        assert_eq!(
            account.entity_id,
            EntityId::new("test-resonance-0".to_string())
        );
        assert_eq!(account.balance, 0.0);
        assert!(account.resonance_patterns.is_empty());
    }

    #[test]
    fn test_account_add_resonance() {
        let mut economy = create_test_economy();
        economy
            .create_account(
                EntityId::new("test-resonance-2".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        let account = economy
            .ledger
            .entity_accounts
            .get_mut(&EntityId::new("test-resonance-2".to_string()))
            .unwrap();
        account.add_resonance(100.0, create_test_pattern(), 10.0);

        assert_eq!(account.balance, 100.0);
        assert_eq!(account.resonance_patterns.len(), 1);
    }

    #[test]
    fn test_account_remove_resonance() {
        let mut economy = create_test_economy();
        economy
            .create_account(
                EntityId::new("test-resonance-4".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        let account = economy
            .ledger
            .entity_accounts
            .get_mut(&EntityId::new("test-resonance-4".to_string()))
            .unwrap();
        account.add_resonance(100.0, create_test_pattern(), 10.0);

        let result = account.remove_resonance(50.0, create_test_pattern(), 20.0);
        assert!(result.is_ok());
        assert_eq!(account.balance, 50.0);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut economy = create_test_economy();
        economy
            .create_account(
                EntityId::new("test-resonance-6".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        let account = economy
            .ledger
            .entity_accounts
            .get_mut(&EntityId::new("test-resonance-6".to_string()))
            .unwrap();
        account.add_resonance(30.0, create_test_pattern(), 10.0);

        let result = account.remove_resonance(50.0, create_test_pattern(), 20.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_record_trade() {
        let mut economy = create_test_economy();
        economy
            .create_account(
                EntityId::new("test-resonance-8".to_string()),
                AccountType::Individual,
            )
            .unwrap();
        economy
            .create_account(
                EntityId::new("test-resonance-9".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        // Give first entity some resonance
        let account = economy
            .ledger
            .entity_accounts
            .get_mut(&EntityId::new("test-resonance-8".to_string()))
            .unwrap();
        account.add_resonance(100.0, create_test_pattern(), 0.0);

        let result = economy.record_trade(
            EntityId::new("test-resonance-8".to_string()),
            EntityId::new("test-resonance-9".to_string()),
            50.0,
            create_test_pattern(),
        );

        assert!(result.is_ok());
        assert_eq!(economy.ledger.total_transactions(), 1);
    }

    #[test]
    fn test_record_catalyst_generation() {
        let mut economy = create_test_economy();
        let entity_id = EntityId::new("test-resonance-13".to_string());
        economy
            .create_account(entity_id.clone(), AccountType::Individual)
            .unwrap();

        let result = economy.record_catalyst_generation(
            entity_id.clone(),
            100.0,
            create_test_pattern(),
            ResonanceSource::CatalystGeneration,
        );

        assert!(result.is_ok());
        assert!(economy.ledger.total_resonance_in_system > 0.0);
    }

    #[test]
    fn test_record_quest_reward() {
        let mut economy = create_test_economy();
        let entity_id = EntityId::new("test-resonance-15".to_string());
        economy
            .create_account(entity_id.clone(), AccountType::Individual)
            .unwrap();

        let result = economy.record_quest_reward(entity_id.clone(), 75.0, create_test_pattern());

        assert!(result.is_ok());
        assert!(economy.ledger.total_resonance_in_system > 0.0);
    }

    #[test]
    fn test_apply_decay_removes_resonance() {
        let mut economy = create_test_economy();
        let entity_id = EntityId::new("test-resonance-17".to_string());
        economy
            .create_account(entity_id.clone(), AccountType::Individual)
            .unwrap();

        economy
            .record_catalyst_generation(
                entity_id.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        let original_total = economy.ledger.total_resonance_in_system;
        economy.apply_decay(50.0).unwrap();

        assert!(economy.ledger.total_resonance_in_system < original_total);
    }

    #[test]
    fn test_calculate_economic_metrics() {
        let mut economy = create_test_economy();
        let entity1 = EntityId::new("test-resonance-19".to_string());
        let entity2 = EntityId::new("test-resonance-20".to_string());
        economy
            .create_account(entity1.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity2.clone(), AccountType::Individual)
            .unwrap();

        // Give first entity some resonance to trade
        economy
            .record_catalyst_generation(
                entity1.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        economy
            .record_trade(
                entity1.clone(),
                entity2.clone(),
                50.0,
                create_test_pattern(),
            )
            .unwrap();

        let metrics = economy.calculate_economic_metrics();
        assert!(metrics.total_resonance > 0.0);
    }

    #[test]
    fn test_detect_recession() {
        let mut metrics = EconomicMetrics::new();
        metrics.circulation_velocity = 0.2;
        metrics.diversity_index = 0.2;
        metrics.economic_growth_rate = -0.1;
        metrics.decay_rate = 0.1;
        metrics.creation_rate = 0.05;

        assert!(metrics.detect_recession());
    }

    #[test]
    fn test_detect_boom() {
        let mut metrics = EconomicMetrics::new();
        metrics.circulation_velocity = 0.8;
        metrics.diversity_index = 0.8;
        metrics.economic_growth_rate = 0.2;
        metrics.decay_rate = 0.05;
        metrics.creation_rate = 0.2;

        assert!(metrics.detect_boom());
    }

    #[test]
    fn test_health_score() {
        let mut metrics = EconomicMetrics::new();
        metrics.circulation_velocity = 0.7;
        metrics.diversity_index = 0.7;
        metrics.economic_growth_rate = 0.1;
        metrics.creation_rate = 0.1;
        metrics.decay_rate = 0.09;

        let health = metrics.calculate_health_score();
        assert!(health >= 0.0);
        assert!(health <= 1.0);
        assert!(health > 0.5);
    }

    #[test]
    fn test_stimulate_economy() {
        let mut economy = create_test_economy();
        economy
            .create_account(
                EntityId::new("test-resonance-24".to_string()),
                AccountType::Individual,
            )
            .unwrap();
        economy
            .create_account(
                EntityId::new("test-resonance-25".to_string()),
                AccountType::Individual,
            )
            .unwrap();

        let original_total = economy.ledger.total_resonance_in_system;
        economy.stimulate_economy(100.0).unwrap();

        assert!(economy.ledger.total_resonance_in_system > original_total);
    }

    #[test]
    fn test_contract_economy() {
        let mut economy = create_test_economy();
        let entity1 = EntityId::new("test-resonance-26".to_string());
        let entity2 = EntityId::new("test-resonance-27".to_string());
        economy
            .create_account(entity1.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity2.clone(), AccountType::Individual)
            .unwrap();

        // Give entities resonance
        economy
            .record_catalyst_generation(
                entity1.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        economy
            .record_catalyst_generation(
                entity2.clone(),
                50.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        let original_total = economy.ledger.total_resonance_in_system;
        economy.contract_economy(20.0).unwrap();

        assert!(economy.ledger.total_resonance_in_system < original_total);
    }

    #[test]
    fn test_get_economy_health_report() {
        let mut economy = create_test_economy();
        let entity1 = EntityId::new("test-resonance-30".to_string());
        let entity2 = EntityId::new("test-resonance-31".to_string());
        economy
            .create_account(entity1.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity2.clone(), AccountType::Individual)
            .unwrap();

        economy
            .record_catalyst_generation(
                entity1.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        let report = economy.get_economy_health_report();

        assert!(report.health_score >= 0.0);
        assert!(report.health_score <= 1.0);
        assert!(report.total_entities > 0);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_get_entity_economic_status() {
        let mut economy = create_test_economy();
        let entity_id = EntityId::new("test-resonance-33".to_string());
        economy
            .create_account(entity_id.clone(), AccountType::Individual)
            .unwrap();

        economy
            .record_catalyst_generation(
                entity_id.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        let status = economy.get_entity_economic_status(entity_id.clone());

        assert!(status.is_some());
        let status = status.unwrap();
        assert_eq!(status.entity_id, entity_id);
        assert!(status.balance > 0.0);
    }

    #[test]
    fn test_economic_cycles() {
        let mut economy = create_test_economy();
        let entity1 = EntityId::new("test-resonance-37".to_string());
        let entity2 = EntityId::new("test-resonance-38".to_string());
        economy
            .create_account(entity1.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity2.clone(), AccountType::Individual)
            .unwrap();

        // Generate enough resonance to trigger creation cycle
        // Creation cycle requires: creation_rate > decay_rate * 1.5
        // Since we haven't decayed anything, this should trigger with enough creation
        for _ in 0..10 {
            economy
                .record_catalyst_generation(
                    entity1.clone(),
                    50.0,
                    create_test_pattern(),
                    ResonanceSource::CatalystGeneration,
                )
                .unwrap();
        }

        // Update metrics before detecting cycles
        economy.calculate_economic_metrics();

        let cycles = economy.detect_economic_cycles();
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_top_resonance_holders() {
        let mut economy = create_test_economy();
        let entity1 = EntityId::new("test-resonance-42".to_string());
        let entity2 = EntityId::new("test-resonance-43".to_string());
        let entity3 = EntityId::new("test-resonance-44".to_string());
        economy
            .create_account(entity1.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity2.clone(), AccountType::Individual)
            .unwrap();
        economy
            .create_account(entity3.clone(), AccountType::Individual)
            .unwrap();

        economy
            .record_catalyst_generation(
                entity1.clone(),
                100.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        economy
            .record_catalyst_generation(
                entity2.clone(),
                50.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        economy
            .record_catalyst_generation(
                entity3.clone(),
                75.0,
                create_test_pattern(),
                ResonanceSource::CatalystGeneration,
            )
            .unwrap();

        let top_holders = economy.ledger.get_top_resonance_holders(2);

        assert_eq!(top_holders.len(), 2);
        assert_eq!(top_holders[0].0, entity1); // Should be top with 100.0
        assert!(top_holders[0].1 > top_holders[1].1);
    }

    #[test]
    fn test_cycle_phase_advancement() {
        let mut cycle = EconomicCycle::new(1, CycleType::Creation, 0.0, 0.8);

        assert_eq!(cycle.cycle_phase, CyclePhase::Growing);

        cycle.advance_phase(30.0);
        assert_eq!(cycle.cycle_phase, CyclePhase::Peak);

        cycle.advance_phase(60.0);
        assert_eq!(cycle.cycle_phase, CyclePhase::Declining);

        cycle.advance_phase(90.0);
        assert_eq!(cycle.cycle_phase, CyclePhase::Dormant);
    }

    #[test]
    fn test_flow_remaining_proportion() {
        let flow = ResonanceFlow::new(
            1,
            ResonanceSource::CatalystGeneration,
            100,
            100.0,
            create_test_pattern(),
            0.0,
            FlowType::Creation,
        );

        assert_eq!(flow.remaining_proportion(), 1.0);

        let mut flow_clone = flow.clone();
        flow_clone.apply_decay(10.0);
        assert!(flow_clone.remaining_proportion() < 1.0);
        assert!(flow_clone.remaining_proportion() > 0.0);
    }

    #[test]
    fn test_resonance_source_decay_rates() {
        assert!(
            ResonanceSource::CatalystGeneration.base_decay_rate()
                > ResonanceSource::QuestCompletion.base_decay_rate()
        );
        assert!(ResonanceSource::QuestCompletion.base_decay_rate() > 0.0);
    }

    #[test]
    fn test_account_age_and_last_transaction() {
        let mut economy = create_test_economy();
        let entity_id = EntityId::new("test-resonance-49".to_string());
        economy
            .create_account(entity_id.clone(), AccountType::Individual)
            .unwrap();

        let account = economy.ledger.entity_accounts.get(&entity_id).unwrap();
        assert_eq!(account.age(10.0), 10.0);
        assert_eq!(account.time_since_last_transaction(10.0), 10.0);

        let account = economy.ledger.entity_accounts.get_mut(&entity_id).unwrap();
        account.add_resonance(100.0, create_test_pattern(), 15.0);

        assert_eq!(account.time_since_last_transaction(20.0), 5.0);
    }
}
