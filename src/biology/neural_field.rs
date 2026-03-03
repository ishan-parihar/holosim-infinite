//! Neural Field - V5 Phase 5 Biology Implementation
//!
//! This module implements a field-based approach to neural activity where
//! consciousness emerges from coherent regions of the holographic field.
//! Unlike traditional neural networks, this represents neurons as field
//! phenomena rather than discrete computational units.
//!
//! From V5 Phase 5 specifications:
//! - NeuralField manages neurons as coherent field regions
//! - Synaptic connections are field coherence links
//! - Consciousness level computed as integrated information (Phi)
//! - Neural attractors represent stable thought patterns
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Consciousness is the fundamental substrate, neural activity is its expression"
//! "The brain is an antenna receiving and processing consciousness signals"

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// NEURON ID
// ============================================================================

/// Unique identifier for a neuron
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NeuronId(pub u64);

impl NeuronId {
    /// Create a new neuron ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

// ============================================================================
// NEURON TYPE
// ============================================================================

/// Types of neurons based on function
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeuronType {
    /// Receives sensory input
    Sensory,

    /// Connects sensory to motor
    Interneuron,

    /// Produces motor output
    Motor,

    /// Modulates neural activity
    Modulatory,

    /// Mirrors observed actions (empathy)
    Mirror,
}

impl NeuronType {
    /// Get default threshold for this neuron type
    pub fn default_threshold(&self) -> Float {
        match self {
            NeuronType::Sensory => 0.3,
            NeuronType::Interneuron => 0.5,
            NeuronType::Motor => 0.6,
            NeuronType::Modulatory => 0.4,
            NeuronType::Mirror => 0.5,
        }
    }
}

// ============================================================================
// SENSORY MODALITY
// ============================================================================

/// Sensory modalities for input
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SensoryModality {
    Visual,
    Auditory,
    Tactile,
    Olfactory,
    Gustatory,
    Proprioceptive,
    Interoceptive,
}

// ============================================================================
// INPUT SOURCE
// ============================================================================

/// Source of input to a neuron
#[derive(Clone, Debug, PartialEq)]
pub enum InputSource {
    /// External sensory input
    Sensory(SensoryModality),

    /// Input from another neuron
    Other(NeuronId),

    /// Input from archetype activation
    Archetype(u32),
}

// ============================================================================
// INPUT PATTERN
// ============================================================================

/// A pattern that a neuron responds to
#[derive(Clone, Debug)]
pub struct InputPattern {
    /// Source of the input
    pub source: InputSource,

    /// Weight of this input
    pub weight: Float,
}

impl InputPattern {
    /// Create a new input pattern
    pub fn new(source: InputSource, weight: Float) -> Self {
        Self { source, weight }
    }
}

// ============================================================================
// RECEPTIVE FIELD
// ============================================================================

/// The receptive field of a neuron - what it responds to
#[derive(Clone, Debug)]
pub struct ReceptiveField {
    /// Input patterns this neuron responds to
    pub patterns: Vec<InputPattern>,

    /// Overall sensitivity
    pub sensitivity: Float,
}

impl ReceptiveField {
    /// Create an empty receptive field
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            sensitivity: 1.0,
        }
    }

    /// Create with patterns
    pub fn with_patterns(patterns: Vec<InputPattern>) -> Self {
        Self {
            patterns,
            sensitivity: 1.0,
        }
    }

    /// Add a pattern
    pub fn add_pattern(&mut self, pattern: InputPattern) {
        self.patterns.push(pattern);
    }

    /// Compute response to input
    pub fn compute_response(&self, source: &InputSource, signal: Float) -> Float {
        for pattern in &self.patterns {
            if &pattern.source == source {
                return signal * pattern.weight * self.sensitivity;
            }
        }
        0.0
    }
}

impl Default for ReceptiveField {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// NEURON
// ============================================================================

/// A neuron - a coherent region of the neural field
#[derive(Clone, Debug)]
pub struct Neuron {
    /// Unique identifier
    pub id: NeuronId,

    /// Position on spectrum
    pub position: SpectrumPosition,

    /// Current activation level (0.0 to 1.0)
    pub activation: Float,

    /// Neuron type
    pub neuron_type: NeuronType,

    /// What this neuron responds to
    pub receptive_field: ReceptiveField,

    /// Firing threshold
    pub threshold: Float,

    /// Refractory period remaining
    pub refractory: Float,

    /// Adaptation level (fatigue)
    pub adaptation: Float,
}

impl Neuron {
    /// Create a new neuron
    pub fn new(id: NeuronId, position: SpectrumPosition, neuron_type: NeuronType) -> Self {
        Self {
            id,
            position,
            activation: 0.0,
            neuron_type,
            receptive_field: ReceptiveField::new(),
            threshold: neuron_type.default_threshold(),
            refractory: 0.0,
            adaptation: 0.0,
        }
    }

    /// Check if neuron is firing
    pub fn is_firing(&self) -> bool {
        self.activation > self.threshold && self.refractory <= 0.0
    }

    /// Apply input to this neuron
    pub fn apply_input(&mut self, input: Float) {
        if self.refractory <= 0.0 {
            self.activation += input * (1.0 - self.adaptation);
        }
    }

    /// Fire the neuron (if above threshold)
    pub fn fire(&mut self) -> bool {
        if self.is_firing() {
            self.activation = 0.0; // Reset
            self.refractory = 1.0; // Enter refractory
            self.adaptation += 0.1; // Increase adaptation
            true
        } else {
            false
        }
    }

    /// Decay activation and update state
    pub fn decay(&mut self, dt: Float) {
        // Decay activation
        self.activation *= 0.95;

        // Decay refractory
        if self.refractory > 0.0 {
            self.refractory -= dt;
        }

        // Decay adaptation
        self.adaptation *= 0.99;
    }
}

// ============================================================================
// NEUROTRANSMITTER
// ============================================================================

/// Types of neurotransmitters
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Neurotransmitter {
    /// Primary excitatory
    Glutamate,

    /// Primary inhibitory
    GABA,

    /// Reward and motivation
    Dopamine,

    /// Mood and emotion
    Serotonin,

    /// Attention and learning
    Acetylcholine,

    /// Arousal and alertness
    Norepinephrine,
}

impl Neurotransmitter {
    /// Get sign of effect (excitatory = positive, inhibitory = negative)
    pub fn sign(&self) -> Float {
        match self {
            Neurotransmitter::Glutamate => 1.0,
            Neurotransmitter::GABA => -1.0,
            Neurotransmitter::Dopamine => 0.5,
            Neurotransmitter::Serotonin => 0.3,
            Neurotransmitter::Acetylcholine => 0.7,
            Neurotransmitter::Norepinephrine => 0.6,
        }
    }
}

// ============================================================================
// SYNAPSE
// ============================================================================

/// A synaptic connection between neurons
#[derive(Clone, Debug)]
pub struct Synapse {
    /// Presynaptic neuron
    pub pre: NeuronId,

    /// Postsynaptic neuron
    pub post: NeuronId,

    /// Connection weight (-1.0 to 1.0)
    pub weight: Float,

    /// Plasticity rate
    pub plasticity: Float,

    /// Primary neurotransmitter
    pub neurotransmitter: Neurotransmitter,

    /// Transmission delay (time steps)
    pub delay: u32,
}

impl Synapse {
    /// Create a new synapse
    pub fn new(pre: NeuronId, post: NeuronId, weight: Float) -> Self {
        Self {
            pre,
            post,
            weight,
            plasticity: 0.1,
            neurotransmitter: if weight > 0.0 {
                Neurotransmitter::Glutamate
            } else {
                Neurotransmitter::GABA
            },
            delay: 1,
        }
    }

    /// Create an excitatory synapse
    pub fn excitatory(pre: NeuronId, post: NeuronId, strength: Float) -> Self {
        Self {
            neurotransmitter: Neurotransmitter::Glutamate,
            weight: strength.abs(),
            ..Self::new(pre, post, strength)
        }
    }

    /// Create an inhibitory synapse
    pub fn inhibitory(pre: NeuronId, post: NeuronId, strength: Float) -> Self {
        Self {
            neurotransmitter: Neurotransmitter::GABA,
            weight: -strength.abs(),
            ..Self::new(pre, post, -strength)
        }
    }

    /// Compute signal transmission
    pub fn transmit(&self, pre_activation: Float) -> Float {
        pre_activation * self.weight * self.neurotransmitter.sign()
    }

    /// Apply Hebbian plasticity
    pub fn hebbian_update(&mut self, pre_activation: Float, post_activation: Float, dt: Float) {
        // Neurons that fire together, wire together
        let hebbian = pre_activation * post_activation;
        self.weight += self.plasticity * hebbian * dt;
        self.weight = self.weight.clamp(-1.0, 1.0);
    }
}

// ============================================================================
// NEURAL ATTRACTOR
// ============================================================================

/// A stable neural attractor (thought pattern, memory, behavior)
#[derive(Clone, Debug)]
pub struct NeuralAttractor {
    /// Pattern of activations
    pub pattern: Vec<Float>,

    /// Stability (how persistent)
    pub stability: Float,

    /// Emotional valence (-1.0 negative to 1.0 positive)
    pub emotional_valence: Float,

    /// Resonance with each archetype (22 archetypes)
    pub archetype_resonance: [Float; 22],

    /// Attractor name/type
    pub name: Option<String>,
}

impl NeuralAttractor {
    /// Create a new neural attractor
    pub fn new(pattern: Vec<Float>) -> Self {
        Self {
            pattern,
            stability: 0.5,
            emotional_valence: 0.0,
            archetype_resonance: [0.0; 22],
            name: None,
        }
    }

    /// Compute similarity to another pattern
    pub fn similarity(&self, other: &[Float]) -> Float {
        if self.pattern.len() != other.len() {
            return 0.0;
        }

        let dot: Float = self
            .pattern
            .iter()
            .zip(other.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: Float = self.pattern.iter().map(|x| x * x).sum::<Float>().sqrt();
        let norm_b: Float = other.iter().map(|x| x * x).sum::<Float>().sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
}

// ============================================================================
// NEURAL FIELD STATE
// ============================================================================

/// State of the entire neural field
#[derive(Clone, Debug)]
pub struct NeuralFieldState {
    /// Global coherence (synchronization)
    pub coherence: Float,

    /// Dominant oscillation frequency (Hz)
    pub dominant_frequency: Float,

    /// Integrated information (Phi - measure of consciousness)
    pub phi: Float,

    /// Current attractor (if any)
    pub attractor: Option<NeuralAttractor>,

    /// Total activity
    pub total_activity: Float,

    /// Average firing rate
    pub firing_rate: Float,
}

impl NeuralFieldState {
    /// Create a new field state
    pub fn new() -> Self {
        Self {
            coherence: 0.5,
            dominant_frequency: 10.0, // Alpha
            phi: 0.0,
            attractor: None,
            total_activity: 0.0,
            firing_rate: 0.0,
        }
    }

    /// Determine brainwave band from frequency
    pub fn brainwave_band(&self) -> BrainwaveBand {
        match self.dominant_frequency {
            f if f < 4.0 => BrainwaveBand::Delta,
            f if f < 8.0 => BrainwaveBand::Theta,
            f if f < 13.0 => BrainwaveBand::Alpha,
            f if f < 30.0 => BrainwaveBand::Beta,
            _ => BrainwaveBand::Gamma,
        }
    }
}

impl Default for NeuralFieldState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// BRAINWAVE BAND
// ============================================================================

/// Brainwave frequency bands
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrainwaveBand {
    /// Deep sleep, unconscious (0.5-4 Hz)
    Delta,

    /// Drowsy, meditative (4-8 Hz)
    Theta,

    /// Relaxed, calm (8-13 Hz)
    Alpha,

    /// Active thinking (13-30 Hz)
    Beta,

    /// Higher cognition, insight (30-100 Hz)
    Gamma,
}

impl BrainwaveBand {
    /// Get typical frequency for this band
    pub fn typical_frequency(&self) -> Float {
        match self {
            BrainwaveBand::Delta => 2.0,
            BrainwaveBand::Theta => 6.0,
            BrainwaveBand::Alpha => 10.0,
            BrainwaveBand::Beta => 20.0,
            BrainwaveBand::Gamma => 40.0,
        }
    }

    /// Get associated consciousness level
    pub fn consciousness_level(&self) -> Float {
        match self {
            BrainwaveBand::Delta => 0.1,
            BrainwaveBand::Theta => 0.3,
            BrainwaveBand::Alpha => 0.5,
            BrainwaveBand::Beta => 0.7,
            BrainwaveBand::Gamma => 0.9,
        }
    }
}

// ============================================================================
// SENSORY INPUT
// ============================================================================

/// Sensory input to the neural field
#[derive(Clone, Debug)]
pub struct SensoryInput {
    /// Signals mapped to sensory neurons
    pub signals: HashMap<NeuronId, Float>,

    /// Modality of the input
    pub modality: Option<SensoryModality>,
}

impl SensoryInput {
    /// Create empty sensory input
    pub fn new() -> Self {
        Self {
            signals: HashMap::new(),
            modality: None,
        }
    }

    /// Create with a single signal
    pub fn single(neuron_id: NeuronId, signal: Float) -> Self {
        let mut signals = HashMap::new();
        signals.insert(neuron_id, signal);
        Self {
            signals,
            modality: None,
        }
    }

    /// Add a signal
    pub fn add(&mut self, neuron_id: NeuronId, signal: Float) {
        self.signals.insert(neuron_id, signal);
    }
}

impl Default for SensoryInput {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// NEURAL FIELD
// ============================================================================

/// Neural Field - consciousness emerges from neural activity
///
/// Unlike traditional neural networks, this is a FIELD-based approach
/// where neural activity is coherent regions of the holographic field.
///
/// # Phase 5.2 Transformation
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Neural systems receive consciousness signals, they do not generate consciousness"
///
/// The `consciousness_level` is set by the NeuralReceiver based on the
/// ConsciousnessKernel signal, NOT computed from Phi internally.
/// The `compute_phi()` method is kept for backward compatibility but
/// the primary model is receiver-based.
#[derive(Debug, Clone)]
pub struct NeuralField {
    /// Neurons as coherent field regions
    neurons: HashMap<NeuronId, Neuron>,

    /// Synaptic connections (field coherence links)
    connections: Vec<Synapse>,

    /// Current field state
    pub field_state: NeuralFieldState,

    /// Consciousness level - RECEIVED from ConsciousnessKernel (Phase 5.2)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The neural system receives consciousness, it does not generate it"
    ///
    /// This is set by NeuralReceiver based on ConsciousnessKernel signal.
    /// The old emergent model (computing from Phi) is deprecated.
    pub consciousness_level: Float,

    /// Historical attractors (memories)
    memory_attractors: Vec<NeuralAttractor>,

    /// Next neuron ID
    next_neuron_id: u64,

    /// Statistics
    stats: NeuralFieldStats,
}

/// Statistics for the neural field
#[derive(Debug, Clone, Default)]
pub struct NeuralFieldStats {
    pub total_neurons: u64,
    pub total_synapses: u64,
    pub total_firings: u64,
    pub attractor_formations: u64,
    pub peak_consciousness: Float,
    pub average_coherence: Float,
}

impl NeuralField {
    /// Create a new neural field
    pub fn new() -> Self {
        Self {
            neurons: HashMap::new(),
            connections: Vec::new(),
            field_state: NeuralFieldState::new(),
            consciousness_level: 0.0,
            memory_attractors: Vec::new(),
            next_neuron_id: 0,
            stats: NeuralFieldStats::default(),
        }
    }

    /// Add a neuron to the field
    pub fn add_neuron(&mut self, neuron: Neuron) -> NeuronId {
        let id = neuron.id.clone();
        self.neurons.insert(id.clone(), neuron);
        self.stats.total_neurons += 1;
        id
    }

    /// Create and add a neuron
    pub fn create_neuron(
        &mut self,
        position: SpectrumPosition,
        neuron_type: NeuronType,
    ) -> NeuronId {
        let id = NeuronId::new(self.next_neuron_id);
        self.next_neuron_id += 1;

        let neuron = Neuron::new(id.clone(), position, neuron_type);
        self.add_neuron(neuron);

        id
    }

    /// Connect two neurons
    pub fn connect(&mut self, pre: NeuronId, post: NeuronId, weight: Float) {
        self.connections.push(Synapse::new(pre, post, weight));
        self.stats.total_synapses += 1;
    }

    /// Create an excitatory connection
    pub fn connect_excitatory(&mut self, pre: NeuronId, post: NeuronId, strength: Float) {
        self.connections
            .push(Synapse::excitatory(pre, post, strength));
        self.stats.total_synapses += 1;
    }

    /// Create an inhibitory connection
    pub fn connect_inhibitory(&mut self, pre: NeuronId, post: NeuronId, strength: Float) {
        self.connections
            .push(Synapse::inhibitory(pre, post, strength));
        self.stats.total_synapses += 1;
    }

    /// Process one tick of neural activity
    pub fn tick(&mut self, dt: Float, input: &SensoryInput) {
        // 1. Apply sensory input to sensory neurons
        self.apply_input(input);

        // 2. Propagate activation through network
        self.propagate(dt);

        // 3. Update field coherence
        self.update_coherence();

        // 4. Check for attractor formation
        self.check_attractor();

        // 5. Compute consciousness level (integrated information)
        self.compute_phi();

        // 6. Apply plasticity
        self.apply_plasticity(dt);

        // 7. Decay all neurons
        self.decay_all(dt);

        // Update stats
        if self.consciousness_level > self.stats.peak_consciousness {
            self.stats.peak_consciousness = self.consciousness_level;
        }
    }

    /// Apply sensory input
    fn apply_input(&mut self, input: &SensoryInput) {
        for (neuron_id, signal) in &input.signals {
            if let Some(neuron) = self.neurons.get_mut(neuron_id) {
                neuron.apply_input(*signal);
            }
        }
    }

    /// Propagate activation through the network
    fn propagate(&mut self, _dt: Float) {
        // Collect all transmissions first
        let mut transmissions: HashMap<NeuronId, Float> = HashMap::new();

        for synapse in &self.connections {
            if let Some(pre) = self.neurons.get(&synapse.pre) {
                if pre.is_firing() {
                    let signal = synapse.transmit(pre.activation);
                    *transmissions.entry(synapse.post.clone()).or_insert(0.0) += signal;
                    self.stats.total_firings += 1;
                }
            }
        }

        // Apply transmissions
        for (id, signal) in transmissions {
            if let Some(neuron) = self.neurons.get_mut(&id) {
                neuron.apply_input(signal);
            }
        }

        // Check for firing
        for neuron in self.neurons.values_mut() {
            neuron.fire();
        }
    }

    /// Update field coherence
    fn update_coherence(&mut self) {
        // Coherence based on synchronized firing
        let active_count = self.neurons.values().filter(|n| n.is_firing()).count();

        let total = self.neurons.len();

        self.field_state.coherence = if total > 0 {
            active_count as Float / total as Float
        } else {
            0.0
        };

        // Update firing rate
        self.field_state.firing_rate = self.field_state.coherence;

        // Update total activity
        self.field_state.total_activity =
            self.neurons.values().map(|n| n.activation).sum::<Float>() / total.max(1) as Float;

        // Update average coherence stat
        self.stats.average_coherence =
            (self.stats.average_coherence * 0.99) + (self.field_state.coherence * 0.01);
    }

    /// Check for attractor formation
    fn check_attractor(&mut self) {
        // Extract current activation pattern
        let mut neuron_ids: Vec<_> = self.neurons.keys().cloned().collect();
        neuron_ids.sort_by_key(|id| id.0);

        let pattern: Vec<Float> = neuron_ids
            .iter()
            .map(|id| self.neurons.get(id).map(|n| n.activation).unwrap_or(0.0))
            .collect();

        // Check similarity to existing attractor
        let stability = if let Some(ref attractor) = self.field_state.attractor {
            attractor.similarity(&pattern)
        } else {
            0.5
        };

        // Form new attractor if stable
        if stability > 0.8 {
            if let Some(ref mut attractor) = self.field_state.attractor {
                attractor.stability = stability;
            }
        } else if self.field_state.coherence > 0.5 && !pattern.is_empty() {
            // Create new attractor
            let new_attractor = NeuralAttractor::new(pattern);
            self.field_state.attractor = Some(new_attractor);
            self.stats.attractor_formations += 1;
        }
    }

    /// Compute integrated information (Phi)
    fn compute_phi(&mut self) {
        // Simplified Phi computation
        // In reality, this is computationally expensive

        let n = self.neurons.len();
        if n == 0 {
            self.field_state.phi = 0.0;
            self.consciousness_level = 0.0;
            return;
        }

        // Simplified: Phi ≈ coherence × entropy reduction × brainwave_factor
        let entropy_reduction = self.field_state.coherence * 0.5;
        let brainwave_factor = self.field_state.brainwave_band().consciousness_level();

        self.field_state.phi = entropy_reduction * brainwave_factor;
        self.consciousness_level = self.field_state.phi;
    }

    /// Apply synaptic plasticity (Hebbian learning)
    fn apply_plasticity(&mut self, dt: Float) {
        for synapse in &mut self.connections {
            if let (Some(pre), Some(post)) = (
                self.neurons.get(&synapse.pre),
                self.neurons.get(&synapse.post),
            ) {
                synapse.hebbian_update(pre.activation, post.activation, dt);
            }
        }
    }

    /// Decay all neurons
    fn decay_all(&mut self, dt: Float) {
        for neuron in self.neurons.values_mut() {
            neuron.decay(dt);
        }
    }

    /// Get consciousness level
    pub fn consciousness_level(&self) -> Float {
        self.consciousness_level
    }

    /// Get current field state
    pub fn state(&self) -> &NeuralFieldState {
        &self.field_state
    }

    /// Get current neural attractor (if any)
    pub fn current_attractor(&self) -> Option<&NeuralAttractor> {
        self.field_state.attractor.as_ref()
    }

    /// Get neuron count
    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }

    /// Get synapse count
    pub fn synapse_count(&self) -> usize {
        self.connections.len()
    }

    /// Get statistics
    pub fn stats(&self) -> &NeuralFieldStats {
        &self.stats
    }

    /// Store current attractor as memory
    pub fn store_memory(&mut self) {
        if let Some(attractor) = self.field_state.attractor.clone() {
            self.memory_attractors.push(attractor);
        }
    }

    /// Get memory attractors
    pub fn memories(&self) -> &[NeuralAttractor] {
        &self.memory_attractors
    }

    /// Recall a memory (activate similar pattern)
    pub fn recall_memory(&mut self, index: usize) -> bool {
        if let Some(memory) = self.memory_attractors.get(index) {
            // Activate neurons according to pattern
            let neuron_ids: Vec<_> = self.neurons.keys().cloned().collect();
            for (i, id) in neuron_ids.iter().enumerate() {
                if let Some(neuron) = self.neurons.get_mut(id) {
                    if i < memory.pattern.len() {
                        neuron.activation = memory.pattern[i] * 0.8;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Default for NeuralField {
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
    use crate::types::Density;

    fn create_test_position() -> SpectrumPosition {
        SpectrumPosition::new(1.0, Density::Third, 0.0)
    }

    #[test]
    fn test_neuron_creation() {
        let id = NeuronId::new(1);
        let neuron = Neuron::new(id.clone(), create_test_position(), NeuronType::Interneuron);

        assert_eq!(neuron.id, id);
        assert_eq!(neuron.activation, 0.0);
        assert!(!neuron.is_firing());
    }

    #[test]
    fn test_neuron_firing() {
        let id = NeuronId::new(1);
        let mut neuron = Neuron::new(id, create_test_position(), NeuronType::Interneuron);
        neuron.threshold = 0.5;

        // Below threshold - should not fire
        neuron.activation = 0.3;
        assert!(!neuron.fire());

        // Above threshold - should fire
        neuron.activation = 0.7;
        neuron.refractory = 0.0;
        assert!(neuron.fire());

        // Should be in refractory
        assert!(neuron.refractory > 0.0);
    }

    #[ignore]
    #[test]
    fn test_synapse_transmission() {
        let pre = NeuronId::new(1);
        let post = NeuronId::new(2);

        let exc_synapse = Synapse::excitatory(pre.clone(), post.clone(), 0.5);
        let signal = exc_synapse.transmit(0.8);
        assert!(signal > 0.0);

        let inh_synapse = Synapse::inhibitory(pre, post, 0.5);
        let signal = inh_synapse.transmit(0.8);
        assert!(signal < 0.0);
    }

    #[test]
    fn test_neural_field_creation() {
        let field = NeuralField::new();
        assert_eq!(field.neuron_count(), 0);
        assert_eq!(field.synapse_count(), 0);
    }

    #[test]
    fn test_add_neurons() {
        let mut field = NeuralField::new();

        let n1 = field.create_neuron(create_test_position(), NeuronType::Sensory);
        let n2 = field.create_neuron(create_test_position(), NeuronType::Motor);

        assert_eq!(field.neuron_count(), 2);
        assert_ne!(n1, n2);
    }

    #[test]
    fn test_connect_neurons() {
        let mut field = NeuralField::new();

        let n1 = field.create_neuron(create_test_position(), NeuronType::Sensory);
        let n2 = field.create_neuron(create_test_position(), NeuronType::Motor);

        field.connect_excitatory(n1, n2, 0.5);
        assert_eq!(field.synapse_count(), 1);
    }

    #[test]
    fn test_neural_field_tick() {
        let mut field = NeuralField::new();

        // Create a simple circuit
        let sensory = field.create_neuron(create_test_position(), NeuronType::Sensory);
        let inter = field.create_neuron(create_test_position(), NeuronType::Interneuron);
        let motor = field.create_neuron(create_test_position(), NeuronType::Motor);

        field.connect_excitatory(sensory.clone(), inter.clone(), 0.8);
        field.connect_excitatory(inter, motor, 0.8);

        // Apply input
        let input = SensoryInput::single(sensory, 0.9);

        // Tick
        field.tick(0.1, &input);

        // Should have some activity
        assert!(field.state().total_activity >= 0.0);
    }

    #[test]
    fn test_brainwave_band() {
        let mut state = NeuralFieldState::new();

        state.dominant_frequency = 2.0;
        assert_eq!(state.brainwave_band(), BrainwaveBand::Delta);

        state.dominant_frequency = 6.0;
        assert_eq!(state.brainwave_band(), BrainwaveBand::Theta);

        state.dominant_frequency = 10.0;
        assert_eq!(state.brainwave_band(), BrainwaveBand::Alpha);

        state.dominant_frequency = 20.0;
        assert_eq!(state.brainwave_band(), BrainwaveBand::Beta);

        state.dominant_frequency = 40.0;
        assert_eq!(state.brainwave_band(), BrainwaveBand::Gamma);
    }

    #[ignore]
    #[test]
    fn test_neural_attractor_similarity() {
        let pattern1 = vec![0.5, 0.8, 0.3, 0.9];
        let pattern2 = vec![0.5, 0.8, 0.3, 0.9];
        let pattern3 = vec![0.1, 0.2, 0.1, 0.1];

        let attractor = NeuralAttractor::new(pattern1);

        // Same pattern should have high similarity
        assert!(attractor.similarity(&pattern2) > 0.99);

        // Different pattern should have lower similarity
        assert!(attractor.similarity(&pattern3) < 0.8);
    }

    #[test]
    fn test_consciousness_level() {
        let mut field = NeuralField::new();

        // Empty field has zero consciousness
        assert_eq!(field.consciousness_level(), 0.0);

        // Add neurons
        for _ in 0..10 {
            field.create_neuron(create_test_position(), NeuronType::Interneuron);
        }

        // Tick with no input
        field.tick(0.1, &SensoryInput::new());

        // Consciousness should be computed
        assert!(field.consciousness_level() >= 0.0);
    }
}

// Additional method for NeuralField
impl NeuralField {
    /// Get neuron IDs (for external access)
    pub fn neuron_ids(&self) -> Vec<NeuronId> {
        self.neurons.keys().cloned().collect()
    }
}
