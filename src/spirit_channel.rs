// Spirit Channel Module
//
// This module implements Spirit as an active channel that funnels inpourings
// from Intelligent Infinity into consciousness.
//
// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
//
// "The Spirit generates intelligent energy from source."
//
// PHASE 2.4: Spirit as Channel (HIGH IMPACT GAP #8)

use crate::entity_state::{EnergySource, IntelligentEnergy};
use crate::types::Float;
use std::fmt;

/// Spirit Channel - Spirit as an active channel for inpourings
///
/// Spirit is not just a vibration level, but an active channel that funnels
/// intelligent energy from Intelligent Infinity into consciousness.
///
/// Knowledge Base Reference: COSMOLOGICAL-ARCHITECTURE.md Section 4.4
/// "The Spirit generates intelligent energy from source"
///
/// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
/// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
#[derive(Debug, Clone)]
pub struct SpiritChannel {
    /// The current state of the spirit channel
    pub channel_state: ChannelState,

    /// Connection to the source (Intelligent Infinity)
    pub source_connection: SourceConnection,

    /// Rate at which energy is funneled through the channel (0.0 to infinity)
    pub funneling_rate: Float,
}

/// Channel State - The current condition of the spirit channel
///
/// The channel can be in various states affecting how much energy can flow through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelState {
    /// Fully open - maximum energy flow
    Open,

    /// Partially open - limited energy flow (typically 50%)
    PartiallyOpen,

    /// Closed - no energy flow (potential remains dormant)
    Closed,

    /// Blocked - no energy flow with resistance (potential distortion)
    Blocked,
}

impl ChannelState {
    /// Get the flow multiplier for this channel state
    ///
    /// Returns a multiplier (0.0 to 1.0) that determines how much energy
    /// can flow through the channel.
    pub fn flow_multiplier(&self) -> Float {
        match self {
            ChannelState::Open => 1.0,
            ChannelState::PartiallyOpen => 0.5,
            ChannelState::Closed => 0.0,
            ChannelState::Blocked => 0.0,
        }
    }

    /// Check if the channel allows any flow
    pub fn allows_flow(&self) -> bool {
        matches!(self, ChannelState::Open | ChannelState::PartiallyOpen)
    }

    /// Check if the channel is clear (not blocked)
    pub fn is_clear(&self) -> bool {
        matches!(self, ChannelState::Open | ChannelState::PartiallyOpen)
    }
}

/// Source Connection - Connection to Intelligent Infinity
///
/// Represents the entity's connection to the source of all intelligent energy.
#[derive(Debug, Clone)]
pub struct SourceConnection {
    /// Whether the connection is established
    pub is_connected: bool,

    /// Strength of the connection (0.0 to 1.0)
    pub connection_strength: Float,

    /// Type of energy source
    pub source_type: EnergySource,

    /// Connection quality (0.0 to 1.0)
    pub connection_quality: Float,
}

impl SourceConnection {
    /// Create a new source connection
    pub fn new(is_connected: bool, connection_strength: Float, source_type: EnergySource) -> Self {
        Self {
            is_connected,
            connection_strength: connection_strength.clamp(0.0, 1.0),
            source_type,
            connection_quality: if is_connected {
                connection_strength.clamp(0.0, 1.0)
            } else {
                0.0
            },
        }
    }

    /// Get the effective energy available from the source
    ///
    /// This calculates how much energy is actually available based on
    /// connection strength and quality.
    pub fn available_energy(&self) -> Float {
        if !self.is_connected {
            return 0.0;
        }

        // Available energy is based on connection strength and quality
        self.connection_strength * self.connection_quality
    }

    /// Establish or re-establish the connection
    pub fn establish(&mut self) {
        self.is_connected = true;
        self.connection_strength = (self.connection_strength + 0.1).min(1.0);
        self.connection_quality = self.connection_strength;
    }

    /// Sever the connection
    pub fn sever(&mut self) {
        self.is_connected = false;
        self.connection_strength = 0.0;
        self.connection_quality = 0.0;
    }

    /// Strengthen the connection
    pub fn strengthen(&mut self, amount: Float) {
        if self.is_connected {
            self.connection_strength = (self.connection_strength + amount).min(1.0);
            self.connection_quality = (self.connection_quality + amount * 0.5).min(1.0);
        }
    }

    /// Weaken the connection
    pub fn weaken(&mut self, amount: Float) {
        self.connection_strength = (self.connection_strength - amount).max(0.0);
        self.connection_quality = (self.connection_quality - amount * 0.5).max(0.0);

        if self.connection_strength <= 0.0 {
            self.is_connected = false;
        }
    }
}

impl Default for SpiritChannel {
    fn default() -> Self {
        Self {
            channel_state: ChannelState::Closed,
            source_connection: SourceConnection::new(false, 0.0, EnergySource::IntelligentInfinity),
            funneling_rate: 1.0,
        }
    }
}

impl SpiritChannel {
    /// Create a new spirit channel with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a spirit channel with specified parameters
    pub fn with_params(
        channel_state: ChannelState,
        source_connection: SourceConnection,
        funneling_rate: Float,
    ) -> Self {
        Self {
            channel_state,
            source_connection,
            funneling_rate: funneling_rate.max(0.0),
        }
    }

    /// Create a spirit channel connected to Intelligent Infinity
    pub fn connected_to_infinity(connection_strength: Float) -> Self {
        Self {
            channel_state: ChannelState::Closed,
            source_connection: SourceConnection::new(
                true,
                connection_strength.clamp(0.0, 1.0),
                EnergySource::IntelligentInfinity,
            ),
            funneling_rate: 1.0,
        }
    }

    /// Funnel intelligent energy from source through the channel
    ///
    /// This is the core method that demonstrates Spirit as an active channel.
    /// Energy flows from Intelligent Infinity through the channel into consciousness.
    ///
    /// The amount of energy funneled depends on:
    /// 1. Source connection strength
    /// 2. Channel state (flow multiplier)
    /// 3. Funneling rate
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
    /// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
    ///
    /// # Returns
    /// IntelligentEnergy that has been funneled through the channel
    pub fn funnel_intelligent_energy(&self) -> IntelligentEnergy {
        // Get available energy from source
        let source_energy = self.source_connection.available_energy();

        // Apply channel state (flow multiplier)
        let flow_multiplier = self.channel_state.flow_multiplier();
        let funneled_amount = source_energy * flow_multiplier * self.funneling_rate;

        // Create intelligent energy from the funneled amount
        // The source is always SpiritComplex (energy channeled through spirit)
        IntelligentEnergy::new(funneled_amount, EnergySource::SpiritComplex)
    }

    /// Funnel intelligent energy with a specific intensity request
    ///
    /// This allows requesting a specific amount of energy from the source.
    /// The actual amount funneled may be less depending on channel conditions.
    ///
    /// # Arguments
    /// * `requested_intensity` - The amount of energy requested
    ///
    /// # Returns
    /// IntelligentEnergy that has been funneled through the channel
    pub fn funnel_with_request(&self, requested_intensity: Float) -> IntelligentEnergy {
        // Get available energy from source
        let source_energy = self.source_connection.available_energy();

        // Apply channel state (flow multiplier)
        let flow_multiplier = self.channel_state.flow_multiplier();

        // Calculate maximum possible funneling
        let max_possible = source_energy * flow_multiplier * self.funneling_rate;

        // Return the lesser of requested and possible
        let funneled_amount = requested_intensity.min(max_possible);

        IntelligentEnergy::new(funneled_amount, EnergySource::SpiritComplex)
    }

    /// Check if the channel is clear (allows flow)
    ///
    /// Knowledge Base Reference: ARCHITECTURE_AUDIT_REPORT.md Section 2.8
    /// "Spirit functions as a 'channel' that funnels inpourings into consciousness."
    pub fn is_clear(&self) -> bool {
        self.channel_state.is_clear()
    }

    /// Check if the channel is fully open
    pub fn is_open(&self) -> bool {
        matches!(self.channel_state, ChannelState::Open)
    }

    /// Check if the channel is blocked
    pub fn is_blocked(&self) -> bool {
        matches!(self.channel_state, ChannelState::Blocked)
    }

    /// Check if the channel is connected to source
    pub fn is_connected(&self) -> bool {
        self.source_connection.is_connected
    }

    /// Open the channel
    pub fn open(&mut self) {
        self.channel_state = ChannelState::Open;
    }

    /// Partially open the channel
    pub fn partially_open(&mut self) {
        self.channel_state = ChannelState::PartiallyOpen;
    }

    /// Close the channel
    pub fn close(&mut self) {
        self.channel_state = ChannelState::Closed;
    }

    /// Block the channel
    pub fn block(&mut self) {
        self.channel_state = ChannelState::Blocked;
    }

    /// Establish source connection
    pub fn establish_connection(&mut self) {
        self.source_connection.establish();
    }

    /// Sever source connection
    pub fn sever_connection(&mut self) {
        self.source_connection.sever();
    }

    /// Strengthen the source connection
    pub fn strengthen_connection(&mut self, amount: Float) {
        self.source_connection.strengthen(amount);
    }

    /// Weaken the source connection
    pub fn weaken_connection(&mut self, amount: Float) {
        self.source_connection.weaken(amount);
    }

    /// Set the funneling rate
    pub fn set_funneling_rate(&mut self, rate: Float) {
        self.funneling_rate = rate.max(0.0);
    }

    /// Get the current funneling rate
    pub fn funneling_rate(&self) -> Float {
        self.funneling_rate
    }

    /// Get the channel state
    pub fn channel_state(&self) -> ChannelState {
        self.channel_state
    }

    /// Get the source connection
    pub fn source_connection(&self) -> &SourceConnection {
        &self.source_connection
    }

    /// Calculate the effective throughput of the channel
    ///
    /// This represents the maximum amount of energy that can flow through
    /// the channel given its current state.
    pub fn effective_throughput(&self) -> Float {
        self.source_connection.available_energy()
            * self.channel_state.flow_multiplier()
            * self.funneling_rate
    }

    /// Get a philosophical description of the channel state
    ///
    /// This provides insight into the spiritual condition represented by
    /// the channel state.
    pub fn philosophical_description(&self) -> &str {
        match (self.channel_state, self.source_connection.is_connected) {
            (ChannelState::Open, true) => {
                "Spirit is fully open and connected to Intelligent Infinity. \
                 Maximum inpouring of intelligent energy is available."
            }
            (ChannelState::PartiallyOpen, true) => {
                "Spirit is partially open and connected to Intelligent Infinity. \
                 Limited inpouring of intelligent energy is available (50% capacity)."
            }
            (ChannelState::Closed, true) => {
                "Spirit is closed but connected to Intelligent Infinity. \
                 Intelligent energy is available but not flowing. \
                 Potential remains dormant."
            }
            (ChannelState::Blocked, true) => {
                "Spirit is blocked despite connection to Intelligent Infinity. \
                 Resistance prevents energy flow. \
                 This creates distortion in the energy system."
            }
            (ChannelState::Open, false) => {
                "Spirit is open but not connected to Intelligent Infinity. \
                 Channel is ready but no source connection exists."
            }
            (ChannelState::PartiallyOpen, false) => {
                "Spirit is partially open but not connected to Intelligent Infinity. \
                 Limited channel capacity with no source connection."
            }
            (ChannelState::Closed, false) => {
                "Spirit is closed and not connected to Intelligent Infinity. \
                 No energy flow possible. Complete dormancy."
            }
            (ChannelState::Blocked, false) => {
                "Spirit is blocked and not connected to Intelligent Infinity. \
                 No energy flow possible. Complete blockage."
            }
        }
    }
}

impl fmt::Display for SpiritChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SpiritChannel:\n\
             - State: {:?}\n\
             - Connected: {}\n\
             - Connection Strength: {:.2}\n\
             - Funneling Rate: {:.2}\n\
             - Effective Throughput: {:.2}\n\
             - Description: {}",
            self.channel_state,
            self.is_connected(),
            self.source_connection.connection_strength,
            self.funneling_rate,
            self.effective_throughput(),
            self.philosophical_description()
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // ChannelState Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_channel_state_flow_multiplier() {
        assert_eq!(ChannelState::Open.flow_multiplier(), 1.0);
        assert_eq!(ChannelState::PartiallyOpen.flow_multiplier(), 0.5);
        assert_eq!(ChannelState::Closed.flow_multiplier(), 0.0);
        assert_eq!(ChannelState::Blocked.flow_multiplier(), 0.0);
    }

    #[test]
    fn test_channel_state_allows_flow() {
        assert!(ChannelState::Open.allows_flow());
        assert!(ChannelState::PartiallyOpen.allows_flow());
        assert!(!ChannelState::Closed.allows_flow());
        assert!(!ChannelState::Blocked.allows_flow());
    }

    #[test]
    fn test_channel_state_is_clear() {
        assert!(ChannelState::Open.is_clear());
        assert!(ChannelState::PartiallyOpen.is_clear());
        assert!(!ChannelState::Closed.is_clear());
        assert!(!ChannelState::Blocked.is_clear());
    }

    // ------------------------------------------------------------------------
    // SourceConnection Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_source_connection_creation() {
        let connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);

        assert!(connection.is_connected);
        assert_eq!(connection.connection_strength, 0.8);
        assert_eq!(connection.connection_quality, 0.8);
    }

    #[test]
    fn test_source_connection_available_energy() {
        let mut connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);

        assert!((connection.available_energy() - 0.64).abs() < 0.0001); // 0.8 * 0.8

        connection.is_connected = false;
        assert_eq!(connection.available_energy(), 0.0);
    }

    #[test]
    fn test_source_connection_establish() {
        let mut connection = SourceConnection::new(false, 0.0, EnergySource::IntelligentInfinity);

        connection.establish();

        assert!(connection.is_connected);
        assert_eq!(connection.connection_strength, 0.1);
        assert_eq!(connection.connection_quality, 0.1);
    }

    #[test]
    fn test_source_connection_sever() {
        let mut connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);

        connection.sever();

        assert!(!connection.is_connected);
        assert_eq!(connection.connection_strength, 0.0);
        assert_eq!(connection.connection_quality, 0.0);
    }

    #[test]
    fn test_source_connection_strengthen() {
        let mut connection = SourceConnection::new(true, 0.5, EnergySource::IntelligentInfinity);

        connection.strengthen(0.3);

        assert_eq!(connection.connection_strength, 0.8);
        assert_eq!(connection.connection_quality, 0.65); // 0.5 + 0.3 * 0.5
    }

    #[test]
    fn test_source_connection_weaken() {
        let mut connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);

        connection.weaken(0.3);

        assert_eq!(connection.connection_strength, 0.5);
        assert_eq!(connection.connection_quality, 0.65); // 0.8 - 0.3 * 0.5
    }

    #[test]
    fn test_source_connection_clamping() {
        let connection = SourceConnection::new(true, 1.5, EnergySource::IntelligentInfinity);

        assert_eq!(connection.connection_strength, 1.0); // Clamped to 1.0
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Creation Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_new() {
        let channel = SpiritChannel::new();

        assert_eq!(channel.channel_state, ChannelState::Closed);
        assert!(!channel.is_connected());
        assert_eq!(channel.funneling_rate, 1.0);
    }

    #[test]
    fn test_spirit_channel_with_params() {
        let source_connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.5);

        assert_eq!(channel.channel_state, ChannelState::Open);
        assert!(channel.is_connected());
        assert_eq!(channel.funneling_rate, 1.5);
    }

    #[test]
    fn test_spirit_channel_connected_to_infinity() {
        let channel = SpiritChannel::connected_to_infinity(0.7);

        assert_eq!(channel.channel_state, ChannelState::Closed);
        assert!(channel.is_connected());
        assert_eq!(channel.source_connection.connection_strength, 0.7);
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Funneling Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_funnel_open() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let energy = channel.funnel_intelligent_energy();

        assert!((energy.intensity - 1.0).abs() < 0.0001);
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_partially_open() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel =
            SpiritChannel::with_params(ChannelState::PartiallyOpen, source_connection, 1.0);

        let energy = channel.funnel_intelligent_energy();

        assert!((energy.intensity - 0.5).abs() < 0.0001); // 1.0 * 0.5 (flow multiplier)
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_closed() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Closed, source_connection, 1.0);

        let energy = channel.funnel_intelligent_energy();

        assert_eq!(energy.intensity, 0.0); // No flow when closed
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_blocked() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Blocked, source_connection, 1.0);

        let energy = channel.funnel_intelligent_energy();

        assert_eq!(energy.intensity, 0.0); // No flow when blocked
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_with_connection_strength() {
        let source_connection = SourceConnection::new(true, 0.5, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let energy = channel.funnel_intelligent_energy();

        assert!((energy.intensity - 0.25).abs() < 0.0001); // 0.5 * 0.5 (connection strength * quality)
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_with_funneling_rate() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 2.0);

        let energy = channel.funnel_intelligent_energy();

        assert!((energy.intensity - 2.0).abs() < 0.0001); // 1.0 * 2.0 (funneling rate)
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_with_request() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let energy = channel.funnel_with_request(0.5);

        assert!((energy.intensity - 0.5).abs() < 0.0001);
        assert_eq!(energy.source, EnergySource::SpiritComplex);
    }

    #[test]
    fn test_spirit_channel_funnel_with_request_exceeds_capacity() {
        let source_connection = SourceConnection::new(true, 0.5, EnergySource::IntelligentInfinity);
        let channel =
            SpiritChannel::with_params(ChannelState::PartiallyOpen, source_connection, 1.0);

        // Request more than available
        let energy = channel.funnel_with_request(1.0);

        // Should return less than requested (limited by capacity)
        assert!(energy.intensity < 1.0);
        assert!((energy.intensity - 0.125).abs() < 0.0001); // 0.5 * 0.5 * 0.5 (strength * quality * flow)
    }

    // ------------------------------------------------------------------------
    // SpiritChannel State Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_is_clear() {
        let mut channel = SpiritChannel::new();

        assert!(!channel.is_clear());

        channel.open();
        assert!(channel.is_clear());

        channel.partially_open();
        assert!(channel.is_clear());

        channel.close();
        assert!(!channel.is_clear());

        channel.block();
        assert!(!channel.is_clear());
    }

    #[test]
    fn test_spirit_channel_is_open() {
        let mut channel = SpiritChannel::new();

        assert!(!channel.is_open());

        channel.open();
        assert!(channel.is_open());

        channel.partially_open();
        assert!(!channel.is_open());
    }

    #[test]
    fn test_spirit_channel_is_blocked() {
        let mut channel = SpiritChannel::new();

        assert!(!channel.is_blocked());

        channel.block();
        assert!(channel.is_blocked());

        channel.open();
        assert!(!channel.is_blocked());
    }

    #[test]
    fn test_spirit_channel_is_connected() {
        let mut channel = SpiritChannel::new();

        assert!(!channel.is_connected());

        channel.establish_connection();
        assert!(channel.is_connected());

        channel.sever_connection();
        assert!(!channel.is_connected());
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Modification Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_open() {
        let mut channel = SpiritChannel::new();

        channel.open();

        assert_eq!(channel.channel_state, ChannelState::Open);
    }

    #[test]
    fn test_spirit_channel_partially_open() {
        let mut channel = SpiritChannel::new();

        channel.partially_open();

        assert_eq!(channel.channel_state, ChannelState::PartiallyOpen);
    }

    #[test]
    fn test_spirit_channel_close() {
        let mut channel = SpiritChannel::new();

        channel.open();
        channel.close();

        assert_eq!(channel.channel_state, ChannelState::Closed);
    }

    #[test]
    fn test_spirit_channel_block() {
        let mut channel = SpiritChannel::new();

        channel.block();

        assert_eq!(channel.channel_state, ChannelState::Blocked);
    }

    #[test]
    fn test_spirit_channel_establish_connection() {
        let mut channel = SpiritChannel::new();

        channel.establish_connection();

        assert!(channel.is_connected());
        assert_eq!(channel.source_connection.connection_strength, 0.1);
    }

    #[test]
    fn test_spirit_channel_sever_connection() {
        let mut channel = SpiritChannel::connected_to_infinity(0.8);

        channel.sever_connection();

        assert!(!channel.is_connected());
        assert_eq!(channel.source_connection.connection_strength, 0.0);
    }

    #[test]
    fn test_spirit_channel_strengthen_connection() {
        let mut channel = SpiritChannel::connected_to_infinity(0.5);

        channel.strengthen_connection(0.3);

        assert_eq!(channel.source_connection.connection_strength, 0.8);
    }

    #[test]
    fn test_spirit_channel_weaken_connection() {
        let mut channel = SpiritChannel::connected_to_infinity(0.8);

        channel.weaken_connection(0.3);

        assert_eq!(channel.source_connection.connection_strength, 0.5);
    }

    #[test]
    fn test_spirit_channel_set_funneling_rate() {
        let mut channel = SpiritChannel::new();

        channel.set_funneling_rate(2.5);

        assert_eq!(channel.funneling_rate, 2.5);
    }

    #[test]
    fn test_spirit_channel_set_funneling_rate_negative() {
        let mut channel = SpiritChannel::new();

        channel.set_funneling_rate(-1.0);

        assert_eq!(channel.funneling_rate, 0.0); // Clamped to 0.0
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Throughput Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_effective_throughput() {
        let source_connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let throughput = channel.effective_throughput();

        assert!((throughput - 0.64).abs() < 0.0001); // 0.8 * 0.8 * 1.0
    }

    #[test]
    fn test_spirit_channel_effective_throughput_partially_open() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel =
            SpiritChannel::with_params(ChannelState::PartiallyOpen, source_connection, 1.0);

        let throughput = channel.effective_throughput();

        assert!((throughput - 0.5).abs() < 0.0001); // 1.0 * 1.0 * 0.5
    }

    #[test]
    fn test_spirit_channel_effective_throughput_closed() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Closed, source_connection, 1.0);

        let throughput = channel.effective_throughput();

        assert_eq!(throughput, 0.0);
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Philosophical Description Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_philosophical_description_open_connected() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let description = channel.philosophical_description();

        assert!(description.contains("fully open"));
        assert!(description.contains("connected"));
        assert!(description.contains("Maximum inpouring"));
    }

    #[test]
    fn test_spirit_channel_philosophical_description_partially_open_connected() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel =
            SpiritChannel::with_params(ChannelState::PartiallyOpen, source_connection, 1.0);

        let description = channel.philosophical_description();

        assert!(description.contains("partially open"));
        assert!(description.contains("connected"));
        assert!(description.contains("Limited inpouring"));
    }

    #[test]
    fn test_spirit_channel_philosophical_description_closed_connected() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Closed, source_connection, 1.0);

        let description = channel.philosophical_description();

        assert!(description.contains("closed"));
        assert!(description.contains("connected"));
        assert!(description.contains("Potential remains dormant"));
    }

    #[test]
    fn test_spirit_channel_philosophical_description_blocked_connected() {
        let source_connection = SourceConnection::new(true, 1.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Blocked, source_connection, 1.0);

        let description = channel.philosophical_description();

        assert!(description.contains("blocked"));
        assert!(description.contains("Resistance prevents"));
        assert!(description.contains("distortion"));
    }

    #[test]
    fn test_spirit_channel_philosophical_description_open_not_connected() {
        let source_connection =
            SourceConnection::new(false, 0.0, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.0);

        let description = channel.philosophical_description();

        assert!(description.contains("open"));
        assert!(description.contains("not connected"));
    }

    // ------------------------------------------------------------------------
    // SpiritChannel Display Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_display() {
        let source_connection = SourceConnection::new(true, 0.8, EnergySource::IntelligentInfinity);
        let channel = SpiritChannel::with_params(ChannelState::Open, source_connection, 1.5);

        let display = format!("{}", channel);

        assert!(display.contains("SpiritChannel"));
        assert!(display.contains("Open"));
        assert!(display.contains("Connected: true"));
        assert!(display.contains("1.50"));
    }

    // ------------------------------------------------------------------------
    // Integration Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_spirit_channel_complete_flow() {
        // Create a fully open channel with strong connection
        let mut channel = SpiritChannel::new();
        channel.establish_connection();
        channel.strengthen_connection(0.9);
        channel.open();

        // Funnel energy
        let energy = channel.funnel_intelligent_energy();

        // Verify energy flows
        assert!(energy.intensity > 0.0);
        assert_eq!(energy.source, EnergySource::SpiritComplex);

        // Verify channel state
        assert!(channel.is_clear());
        assert!(channel.is_connected());
        assert!(channel.effective_throughput() > 0.0);
    }

    #[test]
    fn test_spirit_channel_blocked_flow() {
        // Create a blocked channel
        let mut channel = SpiritChannel::connected_to_infinity(0.8);
        channel.block();

        // Try to funnel energy
        let energy = channel.funnel_intelligent_energy();

        // Verify no energy flows
        assert_eq!(energy.intensity, 0.0);

        // Verify channel state
        assert!(!channel.is_clear());
        assert!(channel.is_blocked());
        assert_eq!(channel.effective_throughput(), 0.0);
    }

    #[test]
    fn test_spirit_channel_gradual_opening() {
        // Start with closed channel
        let mut channel = SpiritChannel::connected_to_infinity(0.8);

        // Channel is closed - no flow
        assert_eq!(channel.funnel_intelligent_energy().intensity, 0.0);

        // Open partially - limited flow
        channel.partially_open();
        let partial_energy = channel.funnel_intelligent_energy();
        assert!(partial_energy.intensity > 0.0);
        assert!(partial_energy.intensity < 1.0);

        // Open fully - maximum flow
        channel.open();
        let full_energy = channel.funnel_intelligent_energy();
        assert!(full_energy.intensity > partial_energy.intensity);
    }
}
