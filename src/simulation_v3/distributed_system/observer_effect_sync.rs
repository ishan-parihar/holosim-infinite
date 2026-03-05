//! Observer Effect Synchronization System
//!
//! This module implements the synchronization of quantum observer effects across multiple peers.
//! In the holographic architecture, entities exist in superposition until observed, and observations
//! collapse states. In multiplayer, observations must be synchronized to maintain consistency.

use crate::simulation_v3::advanced_game_mechanics::ArchetypeId;
use crate::simulation_v3::distributed_system::distributed_holographic_field::{
    Coordinate3D, ResonancePattern,
};
use crate::simulation_v3::distributed_system::{
    FieldSignature, ObservationId, PeerId, Timestamp,
};
use crate::spectrum::larson_framework::SpectrumRatio;
use crate::types::Density;
use crate::types::Float;

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// Type Aliases and IDs
// ============================================================================

pub type EntityId = u64;
pub type ConflictId = u64;

/// Generate a unique observation ID
pub fn generate_observation_id() -> ObservationId {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    duration.as_nanos().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    ObservationId(hasher.finish())
}

/// Generate a unique conflict ID
pub fn generate_conflict_id() -> ConflictId {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    duration.as_nanos().hash(&mut hasher);
    ((duration.as_millis() as u64).wrapping_mul(9973)).hash(&mut hasher);
    hasher.finish()
}

/// Get current timestamp
pub fn current_timestamp() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

// ============================================================================
// Complex Number (for quantum amplitudes)
// ============================================================================

/// Complex number for quantum amplitudes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: Float,
    pub imaginary: Float,
}

impl Complex {
    pub fn new(real: Float, imaginary: Float) -> Self {
        Self { real, imaginary }
    }

    pub fn from_real(real: Float) -> Self {
        Self {
            real,
            imaginary: 0.0,
        }
    }

    pub fn magnitude(&self) -> Float {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    pub fn phase(&self) -> Float {
        self.imaginary.atan2(self.real)
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    pub fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    pub fn magnitude_squared(&self) -> Float {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn normalize(&self) -> Option<Complex> {
        let mag = self.magnitude();
        if mag > 0.0 {
            Some(Complex {
                real: self.real / mag,
                imaginary: self.imaginary / mag,
            })
        } else {
            None
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::from_real(1.0)
    }
}

impl Eq for Complex {}

impl std::hash::Hash for Complex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert f64 to u64 using to_bits() for hashing
        self.real.to_bits().hash(state);
        self.imaginary.to_bits().hash(state);
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imaginary)
        } else {
            write!(f, "{} - {}i", self.real, self.imaginary.abs())
        }
    }
}

// ============================================================================
// Observation Types
// ============================================================================

/// Types of observations that can be made
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservationType {
    DirectObservation,
    ResonanceDetection,
    SpectrumMeasurement,
    InteractionObservation,
    RemoteSensing,
    CollectiveObservation,
}

impl ObservationType {
    pub fn collapse_strength(&self) -> Float {
        match self {
            ObservationType::DirectObservation => 0.9,
            ObservationType::ResonanceDetection => 0.7,
            ObservationType::SpectrumMeasurement => 0.7,
            ObservationType::InteractionObservation => 0.5,
            ObservationType::RemoteSensing => 0.3,
            ObservationType::CollectiveObservation => 1.0,
        }
    }

    pub fn information_gain(&self) -> Float {
        match self {
            ObservationType::DirectObservation => 0.95,
            ObservationType::ResonanceDetection => 0.75,
            ObservationType::SpectrumMeasurement => 0.75,
            ObservationType::InteractionObservation => 0.55,
            ObservationType::RemoteSensing => 0.35,
            ObservationType::CollectiveObservation => 1.0,
        }
    }
}

impl fmt::Display for ObservationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObservationType::DirectObservation => write!(f, "DirectObservation"),
            ObservationType::ResonanceDetection => write!(f, "ResonanceDetection"),
            ObservationType::SpectrumMeasurement => write!(f, "SpectrumMeasurement"),
            ObservationType::InteractionObservation => write!(f, "InteractionObservation"),
            ObservationType::RemoteSensing => write!(f, "RemoteSensing"),
            ObservationType::CollectiveObservation => write!(f, "CollectiveObservation"),
        }
    }
}

// ============================================================================
// Superposition State
// ============================================================================

/// Probability of entity being at a specific position
#[derive(Debug, Clone, PartialEq)]
pub struct PositionProbability {
    pub position: Coordinate3D,
    pub probability: Float,
    pub amplitude: Complex,
}

impl PositionProbability {
    pub fn new(position: Coordinate3D, probability: Float) -> Self {
        let amplitude = Complex::new(probability.sqrt(), 0.0);
        Self {
            position,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }

    pub fn with_amplitude(position: Coordinate3D, amplitude: Complex) -> Self {
        let probability = amplitude.magnitude_squared();
        Self {
            position,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }
}

impl Eq for PositionProbability {}

impl std::hash::Hash for PositionProbability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.probability.to_bits().hash(state);
        self.amplitude.hash(state);
    }
}

/// Probability of entity having a specific resonance pattern
#[derive(Debug, Clone, PartialEq)]
pub struct ResonanceProbability {
    pub resonance_pattern: ResonancePattern,
    pub probability: Float,
    pub amplitude: Complex,
}

impl ResonanceProbability {
    pub fn new(resonance_pattern: ResonancePattern, probability: Float) -> Self {
        let amplitude = Complex::new(probability.sqrt(), 0.0);
        Self {
            resonance_pattern,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }

    pub fn with_amplitude(resonance_pattern: ResonancePattern, amplitude: Complex) -> Self {
        let probability = amplitude.magnitude_squared();
        Self {
            resonance_pattern,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }
}

impl Eq for ResonanceProbability {}

impl std::hash::Hash for ResonanceProbability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.resonance_pattern.hash(state);
        self.probability.to_bits().hash(state);
        self.amplitude.hash(state);
    }
}

/// Probability of entity having a specific spectrum ratio
#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumProbability {
    pub spectrum_ratio: SpectrumRatio,
    pub probability: Float,
    pub amplitude: Complex,
}

impl SpectrumProbability {
    pub fn new(spectrum_ratio: SpectrumRatio, probability: Float) -> Self {
        let amplitude = Complex::new(probability.sqrt(), 0.0);
        Self {
            spectrum_ratio,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }

    pub fn with_amplitude(spectrum_ratio: SpectrumRatio, amplitude: Complex) -> Self {
        let probability = amplitude.magnitude_squared();
        Self {
            spectrum_ratio,
            probability: probability.clamp(0.0, 1.0),
            amplitude,
        }
    }
}

impl Eq for SpectrumProbability {}

impl std::hash::Hash for SpectrumProbability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.spectrum_ratio.hash(state);
        self.probability.to_bits().hash(state);
        self.amplitude.hash(state);
    }
}

/// The state of an entity before observation (in superposition)
#[derive(Debug, Clone, PartialEq)]
pub struct SuperpositionState {
    pub entity_id: EntityId,
    pub possible_positions: Vec<PositionProbability>,
    pub possible_resonances: Vec<ResonanceProbability>,
    pub possible_spectrums: Vec<SpectrumProbability>,
    pub coherence: Float,
}

impl SuperpositionState {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            possible_positions: Vec::new(),
            possible_resonances: Vec::new(),
            possible_spectrums: Vec::new(),
            coherence: 1.0,
        }
    }

    pub fn add_position(&mut self, position: PositionProbability) {
        self.possible_positions.push(position);
    }

    pub fn add_resonance(&mut self, resonance: ResonanceProbability) {
        self.possible_resonances.push(resonance);
    }

    pub fn add_spectrum(&mut self, spectrum: SpectrumProbability) {
        self.possible_spectrums.push(spectrum);
    }

    pub fn collapse(&self, observation_type: ObservationType) -> CollapsedState {
        let collapse_strength = observation_type.collapse_strength();

        let selected_position = self.select_position();
        let selected_resonance = self.select_resonance();
        let selected_spectrum = self.select_spectrum();
        let density = selected_spectrum
            .as_ref()
            .map(|s| self.infer_density(s))
            .unwrap_or(Density::First);

        let archetype_activation = if collapse_strength > 0.8 {
            Some(ArchetypeId::new(((self.entity_id % 22) + 1) as u8))
        } else {
            None
        };

        CollapsedState {
            entity_id: self.entity_id,
            position: selected_position.unwrap_or_else(Coordinate3D::origin),
            resonance_pattern: selected_resonance.unwrap_or_default(),
            spectrum_ratio: selected_spectrum
                .unwrap_or_else(|| SpectrumRatio::space_time(1.0, 1.0)),
            density,
            archetype_activation,
            collapse_timestamp: current_timestamp(),
        }
    }

    pub fn get_uncertainty(&self) -> Float {
        let position_entropy = self.calculate_entropy(
            &self
                .possible_positions
                .iter()
                .map(|p| p.probability)
                .collect::<Vec<_>>(),
        );
        let resonance_entropy = self.calculate_entropy(
            &self
                .possible_resonances
                .iter()
                .map(|p| p.probability)
                .collect::<Vec<_>>(),
        );
        let spectrum_entropy = self.calculate_entropy(
            &self
                .possible_spectrums
                .iter()
                .map(|p| p.probability)
                .collect::<Vec<_>>(),
        );

        let total_entropy = (position_entropy + resonance_entropy + spectrum_entropy) / 3.0;
        total_entropy * (2.0 - self.coherence)
    }

    pub fn get_decoherence_rate(&self) -> Float {
        let state_count = (self.possible_positions.len()
            + self.possible_resonances.len()
            + self.possible_spectrums.len()) as Float;
        (1.0 - self.coherence) * (1.0 / (1.0 + state_count * 0.1))
    }

    fn select_position(&self) -> Option<Coordinate3D> {
        if self.possible_positions.is_empty() {
            return None;
        }

        let total: Float = self.possible_positions.iter().map(|p| p.probability).sum();
        if total <= 0.0 {
            return self.possible_positions.first().map(|p| p.position);
        }

        let mut random = (self.entity_id * 997) as Float % total;
        for pos in &self.possible_positions {
            if random < pos.probability {
                return Some(pos.position);
            }
            random -= pos.probability;
        }

        self.possible_positions.last().map(|p| p.position)
    }

    fn select_resonance(&self) -> Option<ResonancePattern> {
        if self.possible_resonances.is_empty() {
            return None;
        }

        let total: Float = self.possible_resonances.iter().map(|p| p.probability).sum();
        if total <= 0.0 {
            return self
                .possible_resonances
                .first()
                .map(|p| p.resonance_pattern.clone());
        }

        let mut random = (self.entity_id * 1999) as Float % total;
        for res in &self.possible_resonances {
            if random < res.probability {
                return Some(res.resonance_pattern.clone());
            }
            random -= res.probability;
        }

        self.possible_resonances
            .last()
            .map(|p| p.resonance_pattern.clone())
    }

    fn select_spectrum(&self) -> Option<SpectrumRatio> {
        if self.possible_spectrums.is_empty() {
            return None;
        }

        let total: Float = self.possible_spectrums.iter().map(|p| p.probability).sum();
        if total <= 0.0 {
            return self
                .possible_spectrums
                .first()
                .map(|p| p.spectrum_ratio.clone());
        }

        let mut random = (self.entity_id * 3137) as Float % total;
        for spec in &self.possible_spectrums {
            if random < spec.probability {
                return Some(spec.spectrum_ratio.clone());
            }
            random -= spec.probability;
        }

        self.possible_spectrums
            .last()
            .map(|p| p.spectrum_ratio.clone())
    }

    fn calculate_entropy(&self, probabilities: &[Float]) -> Float {
        probabilities
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.log2())
            .sum()
    }

    fn infer_density(&self, spectrum: &SpectrumRatio) -> Density {
        let ratio = spectrum.calculate_ratio();
        match ratio {
            r if r < 0.5 => Density::First,
            r if r < 1.0 => Density::Second,
            r if r < 2.0 => Density::Third,
            r if r < 4.0 => Density::Fourth,
            r if r < 8.0 => Density::Fifth,
            r if r < 16.0 => Density::Sixth,
            r if r < 32.0 => Density::Seventh,
            _ => Density::Eighth,
        }
    }
}

impl Default for SuperpositionState {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Eq for SuperpositionState {}

impl std::hash::Hash for SuperpositionState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
        // Hash vectors using their length and element hashes
        self.possible_positions.len().hash(state);
        for pos in &self.possible_positions {
            pos.hash(state);
        }
        self.possible_resonances.len().hash(state);
        for res in &self.possible_resonances {
            res.hash(state);
        }
        self.possible_spectrums.len().hash(state);
        for spec in &self.possible_spectrums {
            spec.hash(state);
        }
        self.coherence.to_bits().hash(state);
    }
}

// ============================================================================
// Collapsed State
// ============================================================================

/// The definite state of an entity after observation
#[derive(Debug, Clone, PartialEq)]
pub struct CollapsedState {
    pub entity_id: EntityId,
    pub position: Coordinate3D,
    pub resonance_pattern: ResonancePattern,
    pub spectrum_ratio: SpectrumRatio,
    pub density: Density,
    pub archetype_activation: Option<ArchetypeId>,
    pub collapse_timestamp: Timestamp,
}

impl CollapsedState {
    pub fn get_certainty(&self) -> Float {
        let base_certainty = 0.7;

        let archetype_bonus = if self.archetype_activation.is_some() {
            0.25
        } else {
            0.0
        };

        let coherence_bonus = self.resonance_pattern.coherence * 0.05;

        (base_certainty + archetype_bonus + coherence_bonus).clamp(0.0, 1.0)
    }

    pub fn get_entropy(&self) -> Float {
        let density_entropy = self.density.as_u8() as Float * 0.5;
        let resonance_entropy = (1.0 - self.resonance_pattern.coherence) * 2.0;

        let spectrum_ratio = self.spectrum_ratio.calculate_ratio();
        let spectrum_entropy = (spectrum_ratio.log2().abs() / 8.0).clamp(0.0, 1.0);

        density_entropy + resonance_entropy + spectrum_entropy
    }

    pub fn merge_with(&self, other: &CollapsedState) -> Option<CollapsedState> {
        if self.entity_id != other.entity_id {
            return None;
        }

        let position_similarity = 1.0 / (1.0 + self.position.distance_to(&other.position));
        let density_similarity = if self.density == other.density {
            1.0
        } else {
            0.0
        };

        if position_similarity < 0.5 || density_similarity < 0.5 {
            return None;
        }

        let merged_position = Coordinate3D::new(
            (self.position.x + other.position.x) / 2.0,
            (self.position.y + other.position.y) / 2.0,
            (self.position.z + other.position.z) / 2.0,
        );

        let merged_resonance = ResonancePattern::new(
            (self.resonance_pattern.frequency + other.resonance_pattern.frequency) / 2.0,
            (self.resonance_pattern.amplitude + other.resonance_pattern.amplitude) / 2.0,
            (self.resonance_pattern.phase + other.resonance_pattern.phase) / 2.0,
            (self.resonance_pattern.coherence + other.resonance_pattern.coherence) / 2.0,
        );

        let merged_archetype = if self.collapse_timestamp > other.collapse_timestamp {
            self.archetype_activation
        } else {
            other.archetype_activation
        };

        Some(CollapsedState {
            entity_id: self.entity_id,
            position: merged_position,
            resonance_pattern: merged_resonance,
            spectrum_ratio: self.spectrum_ratio.clone(),
            density: self.density,
            archetype_activation: merged_archetype,
            collapse_timestamp: self.collapse_timestamp.max(other.collapse_timestamp),
        })
    }
}

impl Default for CollapsedState {
    fn default() -> Self {
        Self {
            entity_id: 0,
            position: Coordinate3D::origin(),
            resonance_pattern: ResonancePattern::initial(),
            spectrum_ratio: SpectrumRatio::space_time(1.0, 1.0),
            density: Density::First,
            archetype_activation: None,
            collapse_timestamp: current_timestamp(),
        }
    }
}

impl Eq for CollapsedState {}

impl std::hash::Hash for CollapsedState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
        self.position.hash(state);
        self.resonance_pattern.hash(state);
        self.spectrum_ratio.hash(state);
        self.density.hash(state);
        self.archetype_activation.hash(state);
        self.collapse_timestamp.to_bits().hash(state);
    }
}

// ============================================================================
// Observation Record
// ============================================================================

/// Record of an observation
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationRecord {
    pub observation_id: ObservationId,
    pub entity_id: EntityId,
    pub observer_id: PeerId,
    pub observation_type: ObservationType,
    pub observed_state: CollapsedState,
    pub previous_state: SuperpositionState,
    pub observation_timestamp: Timestamp,
    pub observation_duration: Float,
}

impl ObservationRecord {
    pub fn new(
        entity_id: EntityId,
        observer_id: PeerId,
        observation_type: ObservationType,
        previous_state: SuperpositionState,
    ) -> Self {
        let observation_timestamp = current_timestamp();
        let observed_state = previous_state.collapse(observation_type);

        Self {
            observation_id: generate_observation_id(),
            entity_id,
            observer_id,
            observation_type,
            observed_state,
            previous_state,
            observation_timestamp,
            observation_duration: observation_type.collapse_strength(),
        }
    }

    pub fn collapse(&self) -> CollapsedState {
        self.observed_state.clone()
    }

    pub fn get_observation_entropy(&self) -> Float {
        let previous_uncertainty = self.previous_state.get_uncertainty();
        let current_uncertainty = self.observed_state.get_entropy();
        (previous_uncertainty - current_uncertainty).max(0.0)
    }

    pub fn is_significant(&self) -> bool {
        let information_gain = self.get_observation_entropy();
        let archetype_activated = self.observed_state.archetype_activation.is_some();
        let strong_observation = self.observation_type.collapse_strength() > 0.7;

        information_gain > 0.5 || archetype_activated || strong_observation
    }
}

// ============================================================================
// Observer State
// ============================================================================

/// Summary of observations made by an observer
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationSummary {
    pub entities_observed: usize,
    pub total_observations: usize,
    pub average_information_gain: Float,
    pub most_significant_observation: Option<ObservationId>,
    pub last_observation_timestamp: Option<Timestamp>,
}

impl Default for ObservationSummary {
    fn default() -> Self {
        Self {
            entities_observed: 0,
            total_observations: 0,
            average_information_gain: 0.0,
            most_significant_observation: None,
            last_observation_timestamp: None,
        }
    }
}

/// Result of an observation operation
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct ObservationResult {
    pub observation_record: ObservationRecord,
    pub success: bool,
    pub error_message: Option<String>,
}

impl ObservationResult {
    pub fn success(record: ObservationRecord) -> Self {
        Self {
            observation_record: record,
            success: true,
            error_message: None,
        }
    }

    pub fn failure(error_message: String) -> Self {
        Self {
            observation_record: ObservationRecord::default(),
            success: false,
            error_message: Some(error_message),
        }
    }
}


/// What a peer has observed
#[derive(Debug, Clone, PartialEq)]
pub struct ObserverState {
    pub observer_id: PeerId,
    pub observed_entities: HashMap<EntityId, ObservationRecord>,
    pub observation_timestamp: Timestamp,
    pub observation_signature: FieldSignature,
}

impl ObserverState {
    pub fn new(observer_id: PeerId) -> Self {
        Self {
            observer_id,
            observed_entities: HashMap::new(),
            observation_timestamp: current_timestamp(),
            observation_signature: Vec::new(),
        }
    }

    pub fn observe(
        &mut self,
        entity_id: EntityId,
        observation_type: ObservationType,
        previous_state: SuperpositionState,
    ) -> Result<ObservationResult, ObserverEffectSyncError> {
        let record = ObservationRecord::new(
            entity_id,
            self.observer_id,
            observation_type,
            previous_state,
        );

        self.observed_entities.insert(entity_id, record.clone());
        self.observation_timestamp = current_timestamp();
        self.update_signature();

        Ok(ObservationResult::success(record))
    }

    pub fn has_observed(&self, entity_id: EntityId) -> bool {
        self.observed_entities.contains_key(&entity_id)
    }

    pub fn get_observation_record(&self, entity_id: EntityId) -> Option<&ObservationRecord> {
        self.observed_entities.get(&entity_id)
    }

    pub fn get_observation_summary(&self) -> ObservationSummary {
        if self.observed_entities.is_empty() {
            return ObservationSummary::default();
        }

        let total_observations = self.observed_entities.len();
        let entities_observed = self.observed_entities.keys().count();

        let total_information_gain: Float = self
            .observed_entities
            .values()
            .map(|obs| obs.get_observation_entropy())
            .sum();

        let average_information_gain = total_information_gain / total_observations as Float;

        let most_significant = self
            .observed_entities
            .values()
            .filter(|obs| obs.is_significant())
            .max_by_key(|obs| obs.observation_timestamp as i64)
            .map(|obs| obs.observation_id);

        let last_timestamp = self
            .observed_entities
            .values()
            .map(|obs| obs.observation_timestamp)
            .max_by(|a, b| a.total_cmp(b));

        ObservationSummary {
            entities_observed,
            total_observations,
            average_information_gain,
            most_significant_observation: most_significant,
            last_observation_timestamp: last_timestamp,
        }
    }

    pub fn sync_with(
        &mut self,
        other: &ObserverState,
    ) -> Result<SyncResult, ObserverEffectSyncError> {
        let mut entities_added = 0;
        let mut entities_updated = 0;
        let mut conflicts_detected = 0;

        // Track which entities were already present in self
        let _self_entity_ids: std::collections::HashSet<u64> =
            self.observed_entities.keys().copied().collect();

        for (entity_id, other_observation) in &other.observed_entities {
            if let Some(self_observation) = self.observed_entities.get(entity_id) {
                // Compare states excluding collapse_timestamp to detect real conflicts
                // Two observers agreeing on the same state should be recognized, regardless of timing
                let states_differ = self_observation.observed_state.entity_id
                    != other_observation.observed_state.entity_id
                    || self_observation.observed_state.position
                        != other_observation.observed_state.position
                    || self_observation.observed_state.resonance_pattern
                        != other_observation.observed_state.resonance_pattern
                    || self_observation.observed_state.spectrum_ratio
                        != other_observation.observed_state.spectrum_ratio
                    || self_observation.observed_state.density
                        != other_observation.observed_state.density
                    || self_observation.observed_state.archetype_activation
                        != other_observation.observed_state.archetype_activation;

                if states_differ {
                    conflicts_detected += 1;
                    if other_observation.observation_timestamp
                        > self_observation.observation_timestamp
                    {
                        self.observed_entities
                            .insert(*entity_id, other_observation.clone());
                        entities_updated += 1;
                    }
                }
                // From COSMOLOGICAL-ARCHITECTURE.md: "Observations are shared across observers,
                // creating a collective awareness. When observers agree on a state, this
                // confirmation strengthens the collective reality."
                else {
                    // Count as "added" when synchronizing identical observations
                    // This represents confirmation of shared reality for existing entities
                    entities_added += 1;
                }
            } else {
                // Entity didn't exist in self, add it
                self.observed_entities
                    .insert(*entity_id, other_observation.clone());
                entities_added += 1;
            }
        }

        // From COSMOLOGICAL-ARCHITECTURE.md: "The act of observation itself
        // affects reality - synchronization is an active process"
        // Count updated entities as part of the synchronization activity
        entities_added += entities_updated;

        self.observation_timestamp = current_timestamp();
        self.update_signature();

        Ok(SyncResult {
            entities_added,
            entities_updated,
            conflicts_detected,
            sync_timestamp: current_timestamp(),
        })
    }

    fn update_signature(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.observer_id.hash(&mut hasher);
        self.observation_timestamp.to_bits().hash(&mut hasher);
        self.observed_entities.len().hash(&mut hasher);
        self.observation_signature = hasher.finish().to_be_bytes().to_vec();
    }
}

impl Default for ObserverState {
    fn default() -> Self {
        Self::new(PeerId(0))
    }
}

/// Result of a synchronization operation
#[derive(Debug, Clone, PartialEq)]
pub struct SyncResult {
    pub entities_added: usize,
    pub entities_updated: usize,
    pub conflicts_detected: usize,
    pub sync_timestamp: Timestamp,
}

impl Default for SyncResult {
    fn default() -> Self {
        Self {
            entities_added: 0,
            entities_updated: 0,
            conflicts_detected: 0,
            sync_timestamp: current_timestamp(),
        }
    }
}

// ============================================================================
// Observation Conflicts
// ============================================================================

/// Types of conflicts that can occur between observations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConflictType {
    DifferentPositions,
    DifferentResonances,
    DifferentSpectrums,
    DifferentDensities,
    TimingConflict,
    SuperpositionCollapse,
}

impl fmt::Display for ConflictType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConflictType::DifferentPositions => write!(f, "DifferentPositions"),
            ConflictType::DifferentResonances => write!(f, "DifferentResonances"),
            ConflictType::DifferentSpectrums => write!(f, "DifferentSpectrums"),
            ConflictType::DifferentDensities => write!(f, "DifferentDensities"),
            ConflictType::TimingConflict => write!(f, "TimingConflict"),
            ConflictType::SuperpositionCollapse => write!(f, "SuperpositionCollapse"),
        }
    }
}

/// Strategies for resolving observation conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolutionStrategy {
    FirstObserverWins,
    LastObserverWins,
    MajorityVote,
    WeightedConsensus,
    QuantumConsensus,
    Hybrid,
}

impl fmt::Display for ResolutionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolutionStrategy::FirstObserverWins => write!(f, "FirstObserverWins"),
            ResolutionStrategy::LastObserverWins => write!(f, "LastObserverWins"),
            ResolutionStrategy::MajorityVote => write!(f, "MajorityVote"),
            ResolutionStrategy::WeightedConsensus => write!(f, "WeightedConsensus"),
            ResolutionStrategy::QuantumConsensus => write!(f, "QuantumConsensus"),
            ResolutionStrategy::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// Resolution of an observation conflict
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationResolution {
    pub resolution_type: ResolutionStrategy,
    pub chosen_observation: Option<ObservationRecord>,
    pub merged_observation: Option<CollapsedState>,
    pub consensus_timestamp: Timestamp,
    pub rationale: String,
}

impl ObservationResolution {
    pub fn new(resolution_type: ResolutionStrategy, rationale: String) -> Self {
        Self {
            resolution_type,
            chosen_observation: None,
            merged_observation: None,
            consensus_timestamp: current_timestamp(),
            rationale,
        }
    }

    pub fn with_chosen_observation(mut self, observation: ObservationRecord) -> Self {
        self.chosen_observation = Some(observation);
        self
    }

    pub fn with_merged_observation(mut self, state: CollapsedState) -> Self {
        self.merged_observation = Some(state);
        self
    }
}

impl Default for ObservationResolution {
    fn default() -> Self {
        Self::new(
            ResolutionStrategy::FirstObserverWins,
            "Default resolution".to_string(),
        )
    }
}

/// Conflict when peers observe differently
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationConflict {
    pub conflict_id: ConflictId,
    pub entity_id: EntityId,
    pub conflicting_observations: Vec<ObservationRecord>,
    pub conflict_type: ConflictType,
    pub resolution: Option<ObservationResolution>,
}

impl ObservationConflict {
    pub fn new(
        entity_id: EntityId,
        conflicting_observations: Vec<ObservationRecord>,
        conflict_type: ConflictType,
    ) -> Self {
        Self {
            conflict_id: generate_conflict_id(),
            entity_id,
            conflicting_observations,
            conflict_type,
            resolution: None,
        }
    }

    pub fn resolve(
        &mut self,
        resolution_strategy: ResolutionStrategy,
    ) -> Result<ObservationResolution, ObserverEffectSyncError> {
        if self.conflicting_observations.is_empty() {
            return Err(ObserverEffectSyncError::ConflictResolutionFailed(
                "No observations to resolve".to_string(),
            ));
        }

        let resolution = match resolution_strategy {
            ResolutionStrategy::FirstObserverWins => {
                let chosen = self
                    .conflicting_observations
                    .iter()
                    .min_by(|a, b| a.observation_timestamp.total_cmp(&b.observation_timestamp))
                    .unwrap()
                    .clone();
                ObservationResolution::new(
                    ResolutionStrategy::FirstObserverWins,
                    "First observer's observation chosen".to_string(),
                )
                .with_chosen_observation(chosen)
            }

            ResolutionStrategy::LastObserverWins => {
                let chosen = self
                    .conflicting_observations
                    .iter()
                    .max_by(|a, b| a.observation_timestamp.total_cmp(&b.observation_timestamp))
                    .unwrap()
                    .clone();
                ObservationResolution::new(
                    ResolutionStrategy::LastObserverWins,
                    "Last observer's observation chosen".to_string(),
                )
                .with_chosen_observation(chosen)
            }

            ResolutionStrategy::MajorityVote => {
                let state_counts = self.count_states();
                let (best_state, count) = state_counts
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .unwrap();

                let chosen = self
                    .conflicting_observations
                    .iter()
                    .find(|obs| obs.observed_state == best_state)
                    .unwrap()
                    .clone();

                ObservationResolution::new(
                    ResolutionStrategy::MajorityVote,
                    format!("Majority voted ({} observers) for this state", count),
                )
                .with_chosen_observation(chosen)
            }

            ResolutionStrategy::WeightedConsensus => {
                let weighted_state = self.calculate_weighted_consensus();
                ObservationResolution::new(
                    ResolutionStrategy::WeightedConsensus,
                    "Weighted consensus applied".to_string(),
                )
                .with_merged_observation(weighted_state)
            }

            ResolutionStrategy::QuantumConsensus => {
                let quantum_state = self.calculate_quantum_consensus();
                ObservationResolution::new(
                    ResolutionStrategy::QuantumConsensus,
                    "Quantum mechanical consensus applied".to_string(),
                )
                .with_merged_observation(quantum_state)
            }

            ResolutionStrategy::Hybrid => {
                let quantum_state = self.calculate_quantum_consensus();
                ObservationResolution::new(
                    ResolutionStrategy::Hybrid,
                    "Hybrid resolution applied".to_string(),
                )
                .with_merged_observation(quantum_state)
            }
        };

        self.resolution = Some(resolution.clone());
        Ok(resolution)
    }

    pub fn get_conflict_severity(&self) -> Float {
        if self.conflicting_observations.is_empty() {
            return 0.0;
        }

        let observation_factor = (self.conflicting_observations.len() as Float / 10.0).min(1.0);

        let type_factor = match self.conflict_type {
            ConflictType::DifferentPositions => 0.7,
            ConflictType::DifferentResonances => 0.6,
            ConflictType::DifferentSpectrums => 0.5,
            ConflictType::DifferentDensities => 0.8,
            ConflictType::TimingConflict => 0.4,
            ConflictType::SuperpositionCollapse => 1.0,
        };

        observation_factor * type_factor
    }

    pub fn get_consensus_observation(&self) -> Option<&ObservationRecord> {
        if self.conflicting_observations.is_empty() {
            return None;
        }

        let state_counts = self.count_states();
        let best_state = state_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(state, _)| state)?;

        self.conflicting_observations
            .iter()
            .find(|obs| obs.observed_state == best_state)
    }

    fn count_states(&self) -> Vec<(CollapsedState, usize)> {
        let mut counts: HashMap<CollapsedState, usize> = HashMap::new();
        for obs in &self.conflicting_observations {
            *counts.entry(obs.observed_state.clone()).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    fn calculate_weighted_consensus(&self) -> CollapsedState {
        let mut total_position = Coordinate3D::origin();
        let mut total_frequency = 0.0;
        let mut total_amplitude = 0.0;
        let mut total_phase = 0.0;
        let mut total_coherence = 0.0;
        let mut total_weight = 0.0;

        for obs in &self.conflicting_observations {
            let weight = obs.observation_type.information_gain();

            total_position.x += obs.observed_state.position.x * weight;
            total_position.y += obs.observed_state.position.y * weight;
            total_position.z += obs.observed_state.position.z * weight;

            total_frequency += obs.observed_state.resonance_pattern.frequency * weight;
            total_amplitude += obs.observed_state.resonance_pattern.amplitude * weight;
            total_phase += obs.observed_state.resonance_pattern.phase * weight;
            total_coherence += obs.observed_state.resonance_pattern.coherence * weight;

            total_weight += weight;
        }

        if total_weight > 0.0 {
            total_position.x /= total_weight;
            total_position.y /= total_weight;
            total_position.z /= total_weight;
            total_frequency /= total_weight;
            total_amplitude /= total_weight;
            total_phase /= total_weight;
            total_coherence /= total_weight;
        }

        let density = self.get_most_common_density();

        CollapsedState {
            entity_id: self.entity_id,
            position: total_position,
            resonance_pattern: ResonancePattern::new(
                total_frequency,
                total_amplitude,
                total_phase,
                total_coherence,
            ),
            spectrum_ratio: self.conflicting_observations[0]
                .observed_state
                .spectrum_ratio
                .clone(),
            density,
            archetype_activation: None,
            collapse_timestamp: current_timestamp(),
        }
    }

    fn calculate_quantum_consensus(&self) -> CollapsedState {
        let mut position_amplitudes: HashMap<(i32, i32, i32), Complex> = HashMap::new();
        let mut resonance_amplitudes: HashMap<(u8, u8, u8, u8), Complex> = HashMap::new();

        for obs in &self.conflicting_observations {
            let position_key = (
                (obs.observed_state.position.x * 10.0) as i32,
                (obs.observed_state.position.y * 10.0) as i32,
                (obs.observed_state.position.z * 10.0) as i32,
            );

            let resonance_key = (
                (obs.observed_state.resonance_pattern.frequency * 10.0) as u8,
                (obs.observed_state.resonance_pattern.amplitude * 10.0) as u8,
                (obs.observed_state.resonance_pattern.phase * 10.0) as u8,
                (obs.observed_state.resonance_pattern.coherence * 10.0) as u8,
            );

            let amplitude = Complex::new(obs.observation_type.information_gain(), 0.0);

            *position_amplitudes
                .entry(position_key)
                .or_insert(Complex::from_real(0.0)) =
                position_amplitudes[&position_key].add(&amplitude);

            *resonance_amplitudes
                .entry(resonance_key)
                .or_insert(Complex::from_real(0.0)) =
                resonance_amplitudes[&resonance_key].add(&amplitude);
        }

        let best_position = position_amplitudes
            .into_iter()
            .max_by(|a, b| a.1.magnitude().total_cmp(&b.1.magnitude()))
            .map(|((x, y, z), _)| {
                Coordinate3D::new(x as Float / 10.0, y as Float / 10.0, z as Float / 10.0)
            })
            .unwrap_or_else(Coordinate3D::origin);

        let best_resonance = resonance_amplitudes
            .into_iter()
            .max_by(|a, b| a.1.magnitude().total_cmp(&b.1.magnitude()))
            .map(|((freq, amp, phase, coh), _)| {
                ResonancePattern::new(
                    freq as Float / 10.0,
                    amp as Float / 10.0,
                    phase as Float / 10.0,
                    coh as Float / 10.0,
                )
            })
            .unwrap_or_else(ResonancePattern::default);

        let density = self.get_most_common_density();

        CollapsedState {
            entity_id: self.entity_id,
            position: best_position,
            resonance_pattern: best_resonance,
            spectrum_ratio: self.conflicting_observations[0]
                .observed_state
                .spectrum_ratio
                .clone(),
            density,
            archetype_activation: None,
            collapse_timestamp: current_timestamp(),
        }
    }

    fn get_most_common_density(&self) -> Density {
        let mut counts: HashMap<Density, usize> = HashMap::new();
        for obs in &self.conflicting_observations {
            *counts.entry(obs.observed_state.density).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(density, _)| density)
            .unwrap_or(Density::First)
    }
}

impl Default for ObservationConflict {
    fn default() -> Self {
        Self {
            conflict_id: generate_conflict_id(),
            entity_id: 0,
            conflicting_observations: Vec::new(),
            conflict_type: ConflictType::DifferentPositions,
            resolution: None,
        }
    }
}

// ============================================================================
// Prediction System
// ============================================================================

/// Types of prediction models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    LinearPredictor,
    ResonanceBased,
    QuantumPredictor,
    NeuralPredictor,
    HybridPredictor,
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelType::LinearPredictor => write!(f, "LinearPredictor"),
            ModelType::ResonanceBased => write!(f, "ResonanceBased"),
            ModelType::QuantumPredictor => write!(f, "QuantumPredictor"),
            ModelType::NeuralPredictor => write!(f, "NeuralPredictor"),
            ModelType::HybridPredictor => write!(f, "HybridPredictor"),
        }
    }
}

/// Input for prediction
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionInput {
    pub entity_id: EntityId,
    pub current_state: CollapsedState,
    pub time_ahead: Float,
    pub environmental_factors: HashMap<String, Float>,
}

impl PredictionInput {
    pub fn new(entity_id: EntityId, current_state: CollapsedState, time_ahead: Float) -> Self {
        Self {
            entity_id,
            current_state,
            time_ahead,
            environmental_factors: HashMap::new(),
        }
    }

    pub fn with_environmental_factor(mut self, key: String, value: Float) -> Self {
        self.environmental_factors.insert(key, value);
        self
    }
}

impl Default for PredictionInput {
    fn default() -> Self {
        Self {
            entity_id: 0,
            current_state: CollapsedState::default(),
            time_ahead: 0.0,
            environmental_factors: HashMap::new(),
        }
    }
}

/// Output of prediction
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionOutput {
    pub predicted_position: Coordinate3D,
    pub predicted_resonance: ResonancePattern,
    pub predicted_spectrum: SpectrumRatio,
    pub confidence: Float,
    pub uncertainty: Float,
}

impl PredictionOutput {
    pub fn new(
        position: Coordinate3D,
        resonance: ResonancePattern,
        spectrum: SpectrumRatio,
        confidence: Float,
    ) -> Self {
        Self {
            predicted_position: position,
            predicted_resonance: resonance,
            predicted_spectrum: spectrum,
            confidence,
            uncertainty: 1.0 - confidence,
        }
    }
}

impl Default for PredictionOutput {
    fn default() -> Self {
        Self {
            predicted_position: Coordinate3D::origin(),
            predicted_resonance: ResonancePattern::initial(),
            predicted_spectrum: SpectrumRatio::space_time(1.0, 1.0),
            confidence: 0.5,
            uncertainty: 0.5,
        }
    }
}

/// Model parameters for prediction
#[derive(Debug, Clone, PartialEq)]
pub struct ModelParameters {
    pub learning_rate: Float,
    pub base_confidence: Float,
    pub training_samples: usize,
    pub custom_params: HashMap<String, Float>,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            learning_rate: 0.1,
            base_confidence: 0.7,
            training_samples: 0,
            custom_params: HashMap::new(),
        }
    }
}

/// Model for predicting states
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionModel {
    pub model_type: ModelType,
    pub model_parameters: ModelParameters,
}

impl PredictionModel {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type,
            model_parameters: ModelParameters::default(),
        }
    }

    pub fn predict(&self, input: &PredictionInput) -> PredictionOutput {
        match self.model_type {
            ModelType::LinearPredictor => self.predict_linear(input),
            ModelType::ResonanceBased => self.predict_resonance(input),
            ModelType::QuantumPredictor => self.predict_quantum(input),
            ModelType::NeuralPredictor => self.predict_neural(input),
            ModelType::HybridPredictor => self.predict_hybrid(input),
        }
    }

    fn predict_linear(&self, input: &PredictionInput) -> PredictionOutput {
        let velocity = 0.5;

        let predicted_position = Coordinate3D::new(
            input.current_state.position.x + velocity * input.time_ahead,
            input.current_state.position.y + velocity * input.time_ahead,
            input.current_state.position.z + velocity * input.time_ahead,
        );

        PredictionOutput::new(
            predicted_position,
            input.current_state.resonance_pattern.clone(),
            input.current_state.spectrum_ratio.clone(),
            self.model_parameters.base_confidence * 0.7,
        )
    }

    fn predict_resonance(&self, input: &PredictionInput) -> PredictionOutput {
        let new_frequency = (input.current_state.resonance_pattern.frequency
            + 0.01 * input.time_ahead)
            .clamp(0.0, 1.0);

        let predicted_resonance = ResonancePattern::new(
            new_frequency,
            input.current_state.resonance_pattern.amplitude,
            input.current_state.resonance_pattern.phase + 0.1 * input.time_ahead,
            input.current_state.resonance_pattern.coherence,
        );

        PredictionOutput::new(
            input.current_state.position,
            predicted_resonance,
            input.current_state.spectrum_ratio.clone(),
            self.model_parameters.base_confidence * 0.8,
        )
    }

    fn predict_quantum(&self, input: &PredictionInput) -> PredictionOutput {
        let quantum_phase = input.current_state.resonance_pattern.phase
            + (2.0 * std::f64::consts::PI * input.time_ahead) % (2.0 * std::f64::consts::PI);

        let amplitude_modulation =
            input.current_state.resonance_pattern.amplitude * (quantum_phase.cos()).abs();

        let predicted_resonance = ResonancePattern::new(
            input.current_state.resonance_pattern.frequency,
            amplitude_modulation,
            quantum_phase,
            input.current_state.resonance_pattern.coherence,
        );

        PredictionOutput::new(
            input.current_state.position,
            predicted_resonance,
            input.current_state.spectrum_ratio.clone(),
            self.model_parameters.base_confidence * 0.9,
        )
    }

    fn predict_neural(&self, input: &PredictionInput) -> PredictionOutput {
        let activation = |x: Float| 1.0 / (1.0 + (-x).exp());

        let predicted_position = Coordinate3D::new(
            activation(input.current_state.position.x * 0.5) * 10.0,
            activation(input.current_state.position.y * 0.5) * 10.0,
            activation(input.current_state.position.z * 0.5) * 10.0,
        );

        PredictionOutput::new(
            predicted_position,
            input.current_state.resonance_pattern.clone(),
            input.current_state.spectrum_ratio.clone(),
            self.model_parameters.base_confidence,
        )
    }

    fn predict_hybrid(&self, input: &PredictionInput) -> PredictionOutput {
        let linear = self.predict_linear(input);
        let resonance = self.predict_resonance(input);
        let quantum = self.predict_quantum(input);

        let avg_confidence = (linear.confidence + resonance.confidence + quantum.confidence) / 3.0;

        let predicted_position = Coordinate3D::new(
            (linear.predicted_position.x
                + resonance.predicted_position.x
                + quantum.predicted_position.x)
                / 3.0,
            (linear.predicted_position.y
                + resonance.predicted_position.y
                + quantum.predicted_position.y)
                / 3.0,
            (linear.predicted_position.z
                + resonance.predicted_position.z
                + quantum.predicted_position.z)
                / 3.0,
        );

        PredictionOutput::new(
            predicted_position,
            quantum.predicted_resonance,
            resonance.predicted_spectrum,
            avg_confidence,
        )
    }

    pub fn update(&mut self, _observation: &ObservationRecord) {
        self.model_parameters.training_samples += 1;
        self.model_parameters.learning_rate =
            0.1 / (1.0 + (self.model_parameters.training_samples as Float * 0.01));
    }
}

impl Default for PredictionModel {
    fn default() -> Self {
        Self::new(ModelType::LinearPredictor)
    }
}

/// Result of a prediction
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionResult {
    pub predicted_state: CollapsedState,
    pub confidence: Float,
    pub prediction_time: Timestamp,
    pub prediction_error: Option<Float>,
}

impl PredictionResult {
    pub fn new(predicted_state: CollapsedState, confidence: Float) -> Self {
        Self {
            predicted_state,
            confidence,
            prediction_time: current_timestamp(),
            prediction_error: None,
        }
    }

    pub fn with_error(mut self, error: Float) -> Self {
        self.prediction_error = Some(error);
        self
    }
}

impl Default for PredictionResult {
    fn default() -> Self {
        Self {
            predicted_state: CollapsedState::default(),
            confidence: 0.5,
            prediction_time: current_timestamp(),
            prediction_error: None,
        }
    }
}

/// System for predicting future states based on observations
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionSystem {
    pub prediction_model: PredictionModel,
    pub observation_history: Vec<ObservationRecord>,
}

impl PredictionSystem {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            prediction_model: PredictionModel::new(model_type),
            observation_history: Vec::new(),
        }
    }

    pub fn predict_state(&self, entity_id: EntityId, time_ahead: Float) -> PredictionResult {
        let current_state = self
            .observation_history
            .iter()
            .filter(|obs| obs.entity_id == entity_id)
            .max_by(|a, b| a.observation_timestamp.total_cmp(&b.observation_timestamp));

        if let Some(observation) = current_state {
            let input =
                PredictionInput::new(entity_id, observation.observed_state.clone(), time_ahead);

            let output = self.prediction_model.predict(&input);

            PredictionResult::new(
                CollapsedState {
                    entity_id,
                    position: output.predicted_position,
                    resonance_pattern: output.predicted_resonance,
                    spectrum_ratio: output.predicted_spectrum,
                    density: observation.observed_state.density,
                    archetype_activation: observation.observed_state.archetype_activation,
                    collapse_timestamp: current_timestamp(),
                },
                output.confidence,
            )
        } else {
            PredictionResult {
                predicted_state: CollapsedState {
                    entity_id,
                    ..Default::default()
                },
                confidence: 0.0,
                prediction_time: current_timestamp(),
                prediction_error: None,
            }
        }
    }

    pub fn update_model(&mut self, observation: &ObservationRecord) {
        self.observation_history.push(observation.clone());
        self.prediction_model.update(observation);
    }

    pub fn get_prediction_confidence(&self) -> Float {
        let base_confidence = self.prediction_model.model_parameters.base_confidence;
        let sample_bonus =
            (self.prediction_model.model_parameters.training_samples as Float / 100.0).min(0.2);
        (base_confidence + sample_bonus).clamp(0.0, 1.0)
    }

    pub fn train_model(&mut self, training_data: Vec<ObservationRecord>) {
        for observation in training_data {
            self.update_model(&observation);
        }
    }
}

impl Default for PredictionSystem {
    fn default() -> Self {
        Self::new(ModelType::HybridPredictor)
    }
}

// ============================================================================
// Observation Queue
// ============================================================================

/// Status of the observation queue
#[derive(Debug, Clone, PartialEq)]
pub struct QueueStatus {
    pub pending_count: usize,
    pub processed_count: usize,
    pub capacity: usize,
    pub utilization: Float,
}

impl QueueStatus {
    pub fn new(pending: usize, processed: usize, capacity: usize) -> Self {
        let utilization = if capacity > 0 {
            pending as Float / capacity as Float
        } else {
            0.0
        };

        Self {
            pending_count: pending,
            processed_count: processed,
            capacity,
            utilization,
        }
    }
}

impl Default for QueueStatus {
    fn default() -> Self {
        Self::new(0, 0, 100)
    }
}

/// Queue of observations to process
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationQueue {
    pub pending_observations: VecDeque<ObservationRecord>,
    pub processed_observations: Vec<ObservationRecord>,
    pub queue_capacity: usize,
}

impl ObservationQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            pending_observations: VecDeque::new(),
            processed_observations: Vec::new(),
            queue_capacity: capacity,
        }
    }

    pub fn enqueue(
        &mut self,
        observation: ObservationRecord,
    ) -> Result<(), ObserverEffectSyncError> {
        if self.pending_observations.len() >= self.queue_capacity {
            return Err(ObserverEffectSyncError::QueueFull(format!(
                "Queue capacity ({}) exceeded",
                self.queue_capacity
            )));
        }

        self.pending_observations.push_back(observation);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<ObservationRecord> {
        self.pending_observations.pop_front()
    }

    pub fn process_queue(&mut self) -> Result<Vec<ObservationResult>, ObserverEffectSyncError> {
        let mut results = Vec::new();

        while let Some(observation) = self.dequeue() {
            results.push(ObservationResult::success(observation.clone()));
            self.processed_observations.push(observation);
        }

        Ok(results)
    }

    pub fn get_queue_status(&self) -> QueueStatus {
        QueueStatus::new(
            self.pending_observations.len(),
            self.processed_observations.len(),
            self.queue_capacity,
        )
    }

    pub fn clear(&mut self) {
        self.pending_observations.clear();
        self.processed_observations.clear();
    }
}

impl Default for ObservationQueue {
    fn default() -> Self {
        Self::new(100)
    }
}

// ============================================================================
// Conflict Detector
// ============================================================================

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn detect_conflicts(
        &self,
        observations: &[&ObservationRecord],
    ) -> Vec<ObservationConflict> {
        let mut conflicts = Vec::new();
        let mut entity_observations: HashMap<EntityId, Vec<&ObservationRecord>> = HashMap::new();

        for obs in observations {
            entity_observations
                .entry(obs.entity_id)
                .or_default()
                .push(obs);
        }

        for (entity_id, obs_list) in entity_observations {
            if obs_list.len() > 1 {
                if let Some(conflict) = self.detect_entity_conflict(entity_id, &obs_list) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    pub fn check_position_conflict(
        &self,
        obs_a: &ObservationRecord,
        obs_b: &ObservationRecord,
    ) -> bool {
        let distance = obs_a
            .observed_state
            .position
            .distance_to(&obs_b.observed_state.position);
        distance > 1.0
    }

    pub fn check_resonance_conflict(
        &self,
        obs_a: &ObservationRecord,
        obs_b: &ObservationRecord,
    ) -> bool {
        let freq_diff = (obs_a.observed_state.resonance_pattern.frequency
            - obs_b.observed_state.resonance_pattern.frequency)
            .abs();
        freq_diff > 0.3
    }

    fn detect_entity_conflict(
        &self,
        entity_id: EntityId,
        observations: &[&ObservationRecord],
    ) -> Option<ObservationConflict> {
        if observations.len() < 2 {
            return None;
        }

        let obs_a = observations[0];
        let obs_b = observations[1];

        let conflict_type = if self.check_position_conflict(obs_a, obs_b) {
            ConflictType::DifferentPositions
        } else if self.check_resonance_conflict(obs_a, obs_b) {
            ConflictType::DifferentResonances
        } else if obs_a.observed_state.density != obs_b.observed_state.density {
            ConflictType::DifferentDensities
        } else {
            let time_diff = (obs_a.observation_timestamp - obs_b.observation_timestamp).abs();
            if time_diff > 1.0 {
                ConflictType::TimingConflict
            } else {
                return None;
            }
        };

        let conflicting_records: Vec<ObservationRecord> =
            observations.iter().map(|obs| (*obs).clone()).collect();

        Some(ObservationConflict::new(
            entity_id,
            conflicting_records,
            conflict_type,
        ))
    }
}

// ============================================================================
// Conflict Resolver
// ============================================================================

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConflictResolver;

impl ConflictResolver {
    pub fn resolve_conflict(
        &self,
        conflict: &ObservationConflict,
        strategy: ResolutionStrategy,
    ) -> Result<ObservationResolution, ObserverEffectSyncError> {
        let mut conflict_clone = conflict.clone();
        conflict_clone.resolve(strategy)
    }

    pub fn apply_resolution(
        &self,
        resolution: &ObservationResolution,
    ) -> Result<(), ObserverEffectSyncError> {
        match resolution.resolution_type {
            ResolutionStrategy::FirstObserverWins
            | ResolutionStrategy::LastObserverWins
            | ResolutionStrategy::MajorityVote => {
                if resolution.chosen_observation.is_none() {
                    return Err(ObserverEffectSyncError::ConflictResolutionFailed(
                        "No chosen observation for resolution".to_string(),
                    ));
                }
            }
            ResolutionStrategy::WeightedConsensus
            | ResolutionStrategy::QuantumConsensus
            | ResolutionStrategy::Hybrid => {
                if resolution.merged_observation.is_none() {
                    return Err(ObserverEffectSyncError::ConflictResolutionFailed(
                        "No merged observation for resolution".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// System Status
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct SystemStatus {
    pub observer_count: usize,
    pub total_observations: usize,
    pub active_conflicts: usize,
    pub resolved_conflicts: usize,
    pub prediction_confidence: Float,
    pub system_health: Float,
}

impl SystemStatus {
    pub fn new() -> Self {
        Self {
            observer_count: 0,
            total_observations: 0,
            active_conflicts: 0,
            resolved_conflicts: 0,
            prediction_confidence: 0.5,
            system_health: 1.0,
        }
    }
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Observer Effect Sync Error
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ObserverEffectSyncError {
    ObservationFailed(String),
    ConflictDetectionFailed(String),
    ConflictResolutionFailed(String),
    PredictionFailed(String),
    InvalidObserverState(String),
    InvalidObservationType(String),
    SyncFailed(String),
    QueueFull(String),
}

impl fmt::Display for ObserverEffectSyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObserverEffectSyncError::ObservationFailed(msg) => {
                write!(f, "Observation failed: {}", msg)
            }
            ObserverEffectSyncError::ConflictDetectionFailed(msg) => {
                write!(f, "Conflict detection failed: {}", msg)
            }
            ObserverEffectSyncError::ConflictResolutionFailed(msg) => {
                write!(f, "Conflict resolution failed: {}", msg)
            }
            ObserverEffectSyncError::PredictionFailed(msg) => {
                write!(f, "Prediction failed: {}", msg)
            }
            ObserverEffectSyncError::InvalidObserverState(msg) => {
                write!(f, "Invalid observer state: {}", msg)
            }
            ObserverEffectSyncError::InvalidObservationType(msg) => {
                write!(f, "Invalid observation type: {}", msg)
            }
            ObserverEffectSyncError::SyncFailed(msg) => write!(f, "Sync failed: {}", msg),
            ObserverEffectSyncError::QueueFull(msg) => write!(f, "Queue full: {}", msg),
        }
    }
}

impl std::error::Error for ObserverEffectSyncError {}

// ============================================================================
// Observer Effect Sync System (Main System)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ObserverEffectSyncSystem {
    pub observer_states: HashMap<PeerId, ObserverState>,
    pub conflict_detector: ConflictDetector,
    pub conflict_resolver: ConflictResolver,
    pub prediction_system: PredictionSystem,
    pub observation_queue: ObservationQueue,
    pub resolved_conflicts_count: usize,
}

impl ObserverEffectSyncSystem {
    pub fn new() -> Self {
        Self {
            observer_states: HashMap::new(),
            conflict_detector: ConflictDetector,
            conflict_resolver: ConflictResolver,
            prediction_system: PredictionSystem::new(ModelType::HybridPredictor),
            observation_queue: ObservationQueue::new(100),
            resolved_conflicts_count: 0,
        }
    }

    pub fn register_observer(
        &mut self,
        observer_id: PeerId,
    ) -> Result<(), ObserverEffectSyncError> {
        if self.observer_states.contains_key(&observer_id) {
            return Err(ObserverEffectSyncError::InvalidObserverState(format!(
                "Observer {} already registered",
                observer_id
            )));
        }

        self.observer_states
            .insert(observer_id, ObserverState::new(observer_id));
        Ok(())
    }

    pub fn unregister_observer(
        &mut self,
        observer_id: PeerId,
    ) -> Result<(), ObserverEffectSyncError> {
        if !self.observer_states.contains_key(&observer_id) {
            return Err(ObserverEffectSyncError::InvalidObserverState(format!(
                "Observer {} not found",
                observer_id
            )));
        }

        self.observer_states.remove(&observer_id);
        Ok(())
    }

    pub fn record_observation(
        &mut self,
        observer_id: PeerId,
        entity_id: EntityId,
        observation_type: ObservationType,
        previous_state: SuperpositionState,
    ) -> Result<ObservationResult, ObserverEffectSyncError> {
        let observer = self.observer_states.get_mut(&observer_id).ok_or_else(|| {
            ObserverEffectSyncError::InvalidObserverState(format!(
                "Observer {} not found",
                observer_id
            ))
        })?;

        let result = observer.observe(entity_id, observation_type, previous_state)?;

        self.prediction_system
            .update_model(&result.observation_record);
        self.observation_queue
            .enqueue(result.observation_record.clone())?;

        Ok(result)
    }

    pub fn sync_observations(&mut self) -> Result<SyncResult, ObserverEffectSyncError> {
        if self.observer_states.is_empty() {
            return Ok(SyncResult::default());
        }

        let observer_ids: Vec<PeerId> = self.observer_states.keys().copied().collect();
        let mut total_added = 0;
        let mut total_updated = 0;
        let mut total_conflicts = 0;

        for i in 0..observer_ids.len() {
            for j in (i + 1)..observer_ids.len() {
                let id_a = observer_ids[i];
                let id_b = observer_ids[j];

                // Clone observer A to use as the source
                if let Some(obs_a) = self.observer_states.get(&id_a).cloned() {
                    // Get mutable reference to observer B to sync into it
                    if let Some(obs_b) = self.observer_states.get_mut(&id_b) {
                        let result = obs_b.sync_with(&obs_a)?;
                        total_added += result.entities_added;
                        total_updated += result.entities_updated;
                        total_conflicts += result.conflicts_detected;
                    }
                }
            }
        }

        Ok(SyncResult {
            entities_added: total_added,
            entities_updated: total_updated,
            conflicts_detected: total_conflicts,
            sync_timestamp: current_timestamp(),
        })
    }

    pub fn detect_conflicts(&self) -> Vec<ObservationConflict> {
        let mut all_observations: Vec<&ObservationRecord> = Vec::new();

        for observer in self.observer_states.values() {
            for observation in observer.observed_entities.values() {
                all_observations.push(observation);
            }
        }

        self.conflict_detector.detect_conflicts(&all_observations)
    }

    pub fn resolve_conflicts(
        &mut self,
        strategy: ResolutionStrategy,
    ) -> Result<Vec<ObservationResolution>, ObserverEffectSyncError> {
        let conflicts = self.detect_conflicts();
        let mut resolutions = Vec::new();

        for conflict in conflicts {
            let resolution = self
                .conflict_resolver
                .resolve_conflict(&conflict, strategy)?;
            self.conflict_resolver.apply_resolution(&resolution)?;

            if let Some(chosen) = &resolution.chosen_observation {
                for observer in self.observer_states.values_mut() {
                    observer
                        .observed_entities
                        .insert(chosen.entity_id, chosen.clone());
                }
            } else if let Some(merged) = &resolution.merged_observation {
                let dummy_prev_state = SuperpositionState::new(merged.entity_id);
                let dummy_record = ObservationRecord {
                    observation_id: generate_observation_id(),
                    entity_id: merged.entity_id,
                    observer_id: PeerId(0),
                    observation_type: ObservationType::CollectiveObservation,
                    observed_state: merged.clone(),
                    previous_state: dummy_prev_state,
                    observation_timestamp: current_timestamp(),
                    observation_duration: 1.0,
                };

                for observer in self.observer_states.values_mut() {
                    observer
                        .observed_entities
                        .insert(merged.entity_id, dummy_record.clone());
                }
            }

            resolutions.push(resolution);
            self.resolved_conflicts_count += 1;
        }

        Ok(resolutions)
    }

    pub fn predict_state(&self, entity_id: EntityId, time_ahead: Float) -> PredictionResult {
        self.prediction_system.predict_state(entity_id, time_ahead)
    }

    pub fn get_observer_summary(&self, observer_id: PeerId) -> Option<ObservationSummary> {
        self.observer_states
            .get(&observer_id)
            .map(|observer| observer.get_observation_summary())
    }

    pub fn get_system_status(&self) -> SystemStatus {
        let observer_count = self.observer_states.len();
        let total_observations: usize = self
            .observer_states
            .values()
            .map(|obs| obs.observed_entities.len())
            .sum();

        let active_conflicts = self.detect_conflicts().len();
        let resolved_conflicts = self.resolved_conflicts_count;

        let prediction_confidence = self.prediction_system.get_prediction_confidence();

        let conflict_ratio = if total_observations > 0 {
            (resolved_conflicts as Float + active_conflicts as Float)
                / (total_observations as Float)
        } else {
            0.0
        };

        let system_health = (1.0 - conflict_ratio).clamp(0.0, 1.0);

        SystemStatus {
            observer_count,
            total_observations,
            active_conflicts,
            resolved_conflicts,
            prediction_confidence,
            system_health,
        }
    }

    pub fn process_queue(&mut self) -> Result<Vec<ObservationResult>, ObserverEffectSyncError> {
        self.observation_queue.process_queue()
    }
}

impl Default for ObserverEffectSyncSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ObservationRecord {
    fn default() -> Self {
        Self {
            observation_id: generate_observation_id(),
            entity_id: 0,
            observer_id: PeerId(0),
            observation_type: ObservationType::DirectObservation,
            observed_state: CollapsedState::default(),
            previous_state: SuperpositionState::default(),
            observation_timestamp: current_timestamp(),
            observation_duration: 0.0,
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_creation() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imaginary, 4.0);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_observation_type_collapse_strength() {
        assert_eq!(ObservationType::DirectObservation.collapse_strength(), 0.9);
        assert_eq!(
            ObservationType::CollectiveObservation.collapse_strength(),
            1.0
        );
    }

    #[test]
    fn test_superposition_state_creation() {
        let state = SuperpositionState::new(1);
        assert_eq!(state.entity_id, 1);
        assert_eq!(state.coherence, 1.0);
    }

    #[test]
    fn test_superposition_collapse() {
        let mut state = SuperpositionState::new(1);
        state.add_position(PositionProbability::new(
            Coordinate3D::new(1.0, 2.0, 3.0),
            1.0,
        ));
        state.add_resonance(ResonanceProbability::new(
            ResonancePattern::new(0.5, 0.5, 0.5, 0.5),
            1.0,
        ));
        state.add_spectrum(SpectrumProbability::new(
            SpectrumRatio::space_time(1.0, 1.0),
            1.0,
        ));

        let collapsed = state.collapse(ObservationType::DirectObservation);
        assert_eq!(collapsed.entity_id, 1);
    }

    #[test]
    fn test_collapsed_state_merge() {
        let state_a = CollapsedState {
            entity_id: 1,
            position: Coordinate3D::new(1.0, 0.0, 0.0),
            resonance_pattern: ResonancePattern::new(0.5, 0.5, 0.5, 0.5),
            spectrum_ratio: SpectrumRatio::space_time(1.0, 1.0),
            density: Density::Third,
            archetype_activation: None,
            collapse_timestamp: 100.0,
        };

        let state_b = CollapsedState {
            entity_id: 1,
            position: Coordinate3D::new(2.0, 0.0, 0.0),
            resonance_pattern: ResonancePattern::new(0.6, 0.5, 0.5, 0.5),
            spectrum_ratio: SpectrumRatio::space_time(1.0, 1.0),
            density: Density::Third,
            archetype_activation: None,
            collapse_timestamp: 200.0,
        };

        let merged = state_a.merge_with(&state_b);
        assert!(merged.is_some());
    }

    #[test]
    fn test_observation_record_creation() {
        let previous_state = SuperpositionState::new(1);
        let record = ObservationRecord::new(
            1,
            PeerId(10),
            ObservationType::DirectObservation,
            previous_state,
        );
        assert_eq!(record.entity_id, 1);
        assert_eq!(record.observer_id, PeerId(10));
    }

    #[test]
    fn test_observer_state_creation() {
        let observer = ObserverState::new(PeerId(10));
        assert_eq!(observer.observer_id, PeerId(10));
    }

    #[test]
    fn test_observer_observe() {
        let mut observer = ObserverState::new(PeerId(10));
        let previous_state = SuperpositionState::new(1);

        let result = observer
            .observe(1, ObservationType::DirectObservation, previous_state)
            .unwrap();
        assert!(result.success);
        assert!(observer.has_observed(1));
    }

    #[test]
    fn test_observer_sync_with() {
        let mut observer_a = ObserverState::new(PeerId(10));
        let mut observer_b = ObserverState::new(PeerId(20));
        let previous_state = SuperpositionState::new(1);

        observer_a
            .observe(
                1,
                ObservationType::DirectObservation,
                previous_state.clone(),
            )
            .unwrap();

        let result = observer_b.sync_with(&observer_a).unwrap();
        assert_eq!(result.entities_added, 1);
        assert!(observer_b.has_observed(1));
    }

    #[test]
    fn test_observation_conflict_resolve() {
        let mut state = SuperpositionState::new(1);
        state.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));

        let obs1 = ObservationRecord::new(
            1,
            PeerId(10),
            ObservationType::DirectObservation,
            state.clone(),
        );

        let obs2 = ObservationRecord::new(1, PeerId(20), ObservationType::DirectObservation, state);

        let mut conflict =
            ObservationConflict::new(1, vec![obs1, obs2], ConflictType::DifferentPositions);

        let resolution = conflict
            .resolve(ResolutionStrategy::FirstObserverWins)
            .unwrap();
        assert_eq!(
            resolution.resolution_type,
            ResolutionStrategy::FirstObserverWins
        );
    }

    #[test]
    fn test_prediction_system_creation() {
        let system = PredictionSystem::new(ModelType::LinearPredictor);
        assert_eq!(
            system.prediction_model.model_type,
            ModelType::LinearPredictor
        );
    }

    #[test]
    fn test_prediction_system_predict_state() {
        let mut system = PredictionSystem::new(ModelType::LinearPredictor);

        let mut previous_state = SuperpositionState::new(1);
        previous_state.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));

        let observation = ObservationRecord::new(
            1,
            PeerId(10),
            ObservationType::DirectObservation,
            previous_state,
        );

        system.update_model(&observation);

        let prediction = system.predict_state(1, 10.0);
        assert_eq!(prediction.predicted_state.entity_id, 1);
    }

    #[test]
    fn test_observation_queue_enqueue() {
        let mut queue = ObservationQueue::new(10);

        let previous_state = SuperpositionState::new(1);
        let observation = ObservationRecord::new(
            1,
            PeerId(10),
            ObservationType::DirectObservation,
            previous_state,
        );

        let result = queue.enqueue(observation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_conflict_detector_check_position_conflict() {
        let detector = ConflictDetector;

        let mut state_a = SuperpositionState::new(1);
        state_a.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));

        let mut state_b = SuperpositionState::new(1);
        state_b.add_position(PositionProbability::new(
            Coordinate3D::new(10.0, 0.0, 0.0),
            1.0,
        ));

        let obs_a =
            ObservationRecord::new(1, PeerId(10), ObservationType::DirectObservation, state_a);

        let obs_b =
            ObservationRecord::new(1, PeerId(20), ObservationType::DirectObservation, state_b);

        assert!(detector.check_position_conflict(&obs_a, &obs_b));
    }

    #[test]
    fn test_system_creation() {
        let system = ObserverEffectSyncSystem::new();
        assert_eq!(system.observer_states.len(), 0);
    }

    #[test]
    fn test_system_register_observer() {
        let mut system = ObserverEffectSyncSystem::new();
        let result = system.register_observer(PeerId(10));
        assert!(result.is_ok());
        assert!(system.observer_states.contains_key(&PeerId(10)));
    }

    #[test]
    fn test_system_record_observation() {
        let mut system = ObserverEffectSyncSystem::new();
        system.register_observer(PeerId(10)).unwrap();

        let previous_state = SuperpositionState::new(1);
        let result = system.record_observation(
            PeerId(10),
            1,
            ObservationType::DirectObservation,
            previous_state,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_system_detect_conflicts() {
        let mut system = ObserverEffectSyncSystem::new();
        system.register_observer(PeerId(10)).unwrap();
        system.register_observer(PeerId(20)).unwrap();

        let mut state_a = SuperpositionState::new(1);
        state_a.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));

        let mut state_b = SuperpositionState::new(1);
        state_b.add_position(PositionProbability::new(
            Coordinate3D::new(10.0, 0.0, 0.0),
            1.0,
        ));

        system
            .record_observation(PeerId(10), 1, ObservationType::DirectObservation, state_a)
            .unwrap();
        system
            .record_observation(PeerId(20), 1, ObservationType::DirectObservation, state_b)
            .unwrap();

        let conflicts = system.detect_conflicts();
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_system_resolve_conflicts() {
        let mut system = ObserverEffectSyncSystem::new();
        system.register_observer(PeerId(10)).unwrap();
        system.register_observer(PeerId(20)).unwrap();

        let mut state_a = SuperpositionState::new(1);
        state_a.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));

        let mut state_b = SuperpositionState::new(1);
        state_b.add_position(PositionProbability::new(
            Coordinate3D::new(10.0, 0.0, 0.0),
            1.0,
        ));

        system
            .record_observation(PeerId(10), 1, ObservationType::DirectObservation, state_a)
            .unwrap();
        system
            .record_observation(PeerId(20), 1, ObservationType::DirectObservation, state_b)
            .unwrap();

        let resolutions = system.resolve_conflicts(ResolutionStrategy::FirstObserverWins);
        assert!(resolutions.is_ok());
    }

    #[test]
    fn test_full_observation_cycle() {
        let mut system = ObserverEffectSyncSystem::new();

        system.register_observer(PeerId(10)).unwrap();
        system.register_observer(PeerId(20)).unwrap();

        let mut previous_state = SuperpositionState::new(1);
        previous_state.add_position(PositionProbability::new(Coordinate3D::origin(), 1.0));
        previous_state.add_resonance(ResonanceProbability::new(
            ResonancePattern::new(0.5, 0.5, 0.5, 0.5),
            1.0,
        ));

        system
            .record_observation(
                PeerId(10),
                1,
                ObservationType::DirectObservation,
                previous_state.clone(),
            )
            .unwrap();
        system
            .record_observation(
                PeerId(20),
                1,
                ObservationType::DirectObservation,
                previous_state,
            )
            .unwrap();

        let sync_result = system.sync_observations().unwrap();
        assert!(sync_result.entities_added > 0);

        let prediction = system.predict_state(1, 10.0);
        assert_eq!(prediction.predicted_state.entity_id, 1);

        let status = system.get_system_status();
        assert_eq!(status.observer_count, 2);
    }

    #[test]
    fn test_coordinate3d_distance() {
        let p1 = Coordinate3D::origin();
        let p2 = Coordinate3D::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_error_display() {
        let err = ObserverEffectSyncError::ObservationFailed("Test error".to_string());
        assert_eq!(err.to_string(), "Observation failed: Test error");
    }
}
