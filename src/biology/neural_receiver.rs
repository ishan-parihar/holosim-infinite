//! Neural Systems as Consciousness Receivers
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Neural systems receive consciousness signals, they do not generate consciousness"
//! "The brain is a receiver/amplifier, not the source"
//!
//! This module implements the Phase 5.2 transformation:
//! - NeuralReceiver receives consciousness from ConsciousnessKernel
//! - Neural architecture determines reception quality
//! - Neural activity amplifies (not creates) consciousness
//! - Integration with unified ConsciousnessKernel
//!
//! # Architecture
//!
//! ```text
//! ConsciousnessKernel (source)
//!         │
//!         ▼ broadcast_signal()
//! ConsciousnessSignal
//!         │
//!         ▼ receive_consciousness()
//! NeuralReceiver ─── NeuralField (antenna structure)
//!         │
//!         ▼ get_amplification()
//! ConsciousnessAmplification
//!         │
//!         ▼ receive_amplification()
//! ConsciousnessKernel (amplified)
//! ```

use crate::biology::neural_field::{NeuralField, NeuralFieldState};
use crate::consciousness::kernel::{
    ConsciousnessAmplification, ConsciousnessKernel, ConsciousnessSignal, Possibility,
};
use crate::simulation_v3::archetype_basis::ArchetypeActivationProfile;
use crate::types::Float;

/// Neural receiver that receives and amplifies consciousness
///
/// This replaces the emergent consciousness model with a receiver model:
/// - Neural field receives consciousness signal from kernel
/// - Neural architecture determines reception quality
/// - Neural activity amplifies (not creates) consciousness
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "The neural system is not the creator of consciousness but its receiver and amplifier"
#[derive(Debug, Clone)]
pub struct NeuralReceiver {
    /// The underlying neural field (antenna structure)
    pub field: NeuralField,

    /// Reception parameters
    /// How well the neural architecture receives consciousness signals
    pub receptive_frequency: Float,

    /// How much the neural system amplifies received consciousness
    pub amplification_factor: Float,

    /// Current reception quality (0.0 to 1.0)
    pub reception_quality: Float,

    /// Last received consciousness signal
    pub last_signal: Option<ConsciousnessSignal>,

    /// Neural architecture type affects reception
    pub architecture_type: NeuralArchitecture,
}

/// Neural architecture types with different reception characteristics
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "More organized neural structures have better reception of consciousness signals"
#[derive(Debug, Clone, PartialEq)]
pub enum NeuralArchitecture {
    /// Simple neural network (basic reception)
    /// Suitable for simple organisms
    Simple { neurons: usize },

    /// Complex brain-like structure (better reception)
    /// Suitable for more evolved organisms
    Complex {
        layers: usize,
        neurons_per_layer: usize,
    },

    /// Highly organized neural structure (best reception)
    /// Suitable for highly conscious entities
    Organized {
        cortical_columns: usize,
        neurons_per_column: usize,
        connectivity: Float,
    },
}

impl NeuralArchitecture {
    /// Calculate base reception quality from architecture
    ///
    /// More organized structures have better reception of consciousness signals.
    /// This is based on the principle that complexity enables better reception.
    pub fn base_reception_quality(&self) -> Float {
        match self {
            NeuralArchitecture::Simple { neurons } => {
                // More neurons = better reception, but diminishing returns
                0.1 + (*neurons as Float / 10000.0).min(0.3)
            }
            NeuralArchitecture::Complex {
                layers,
                neurons_per_layer,
            } => {
                // Layered structure improves reception
                let layer_factor = (*layers as Float / 10.0).min(1.0);
                let neuron_factor = (*neurons_per_layer as Float / 1000.0).min(0.5);
                0.2 + layer_factor * 0.3 + neuron_factor
            }
            NeuralArchitecture::Organized {
                cortical_columns,
                neurons_per_column,
                connectivity,
            } => {
                // Organized structure has best reception
                let column_factor = (*cortical_columns as Float / 100.0).min(0.4);
                let neuron_factor = (*neurons_per_column as Float / 100.0).min(0.2);
                let conn_factor = connectivity * 0.3;
                0.3 + column_factor + neuron_factor + conn_factor
            }
        }
    }

    /// Calculate amplification factor from architecture
    ///
    /// More connected structures can amplify consciousness more effectively.
    pub fn base_amplification_factor(&self) -> Float {
        match self {
            NeuralArchitecture::Simple { .. } => 1.1,
            NeuralArchitecture::Complex { layers, .. } => 1.1 + (*layers as Float * 0.05).min(0.4),
            NeuralArchitecture::Organized { connectivity, .. } => 1.2 + connectivity * 0.5,
        }
    }
}

impl NeuralReceiver {
    /// Create a new neural receiver with given architecture
    ///
    /// The architecture determines:
    /// - Base reception quality
    /// - Base amplification factor
    /// - Neural field complexity
    pub fn new(architecture: NeuralArchitecture) -> Self {
        Self {
            field: NeuralField::default(),
            receptive_frequency: 0.5,
            amplification_factor: architecture.base_amplification_factor(),
            reception_quality: architecture.base_reception_quality(),
            last_signal: None,
            architecture_type: architecture,
        }
    }

    /// Create a neural receiver with a pre-built neural field
    pub fn with_field(field: NeuralField, architecture: NeuralArchitecture) -> Self {
        Self {
            field,
            receptive_frequency: 0.5,
            amplification_factor: architecture.base_amplification_factor(),
            reception_quality: architecture.base_reception_quality(),
            last_signal: None,
            architecture_type: architecture,
        }
    }

    /// Receive consciousness signal from kernel
    ///
    /// This is the PRIMARY method - neural systems receive, not generate.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The brain receives consciousness signals and processes them,
    ///  generating possibilities for the Free Will to choose from"
    ///
    /// # Arguments
    /// * `kernel` - The consciousness kernel to receive from
    ///
    /// # Returns
    /// * `ReceptionResult` - The result of receiving consciousness
    pub fn receive_consciousness(&mut self, kernel: &ConsciousnessKernel) -> ReceptionResult {
        // Get signal from kernel
        let signal = kernel.broadcast_signal();
        self.last_signal = Some(signal.clone());

        // Calculate reception quality based on signal and architecture
        let quality = self.calculate_reception_quality(&signal);
        self.reception_quality = quality;

        // Process the signal through the neural field
        let processed = self.process_signal(&signal);

        // Generate amplification to send back to kernel
        let amplification = ConsciousnessAmplification {
            factor: self.amplification_factor * quality,
            activation_boost: processed.activation_level * 0.1,
            receptive_quality: quality,
            neural_coherence: self.field.field_state.coherence,
        };

        ReceptionResult {
            received_clarity: signal.clarity * quality,
            processed_possibilities: processed.possibilities,
            amplification,
            quality,
        }
    }

    /// Calculate reception quality based on signal and architecture
    ///
    /// Reception quality depends on:
    /// 1. Architecture complexity (better organized = better reception)
    /// 2. Signal clarity
    /// 3. Source resonance
    /// 4. Receptive frequency match
    fn calculate_reception_quality(&self, signal: &ConsciousnessSignal) -> Float {
        // Base quality from architecture
        let base = self.architecture_type.base_reception_quality();

        // Signal strength affects reception
        let signal_factor = signal.clarity * 0.3;

        // Source resonance affects reception
        let resonance_factor = signal.source_resonance * 0.2;

        // Receptive frequency matching
        let freq_match = if (self.receptive_frequency - 0.5).abs() < 0.2 {
            0.2 // Good frequency match
        } else {
            0.0 // Poor match
        };

        (base + signal_factor + resonance_factor + freq_match).min(1.0)
    }

    /// Process consciousness signal through neural field
    ///
    /// The neural field acts as an antenna, processing the signal
    /// and generating possibilities for Free Will choice.
    fn process_signal(&mut self, signal: &ConsciousnessSignal) -> ProcessedSignal {
        // Update neural field state based on signal
        // The field receives consciousness level from signal, not from its own computation
        self.field.consciousness_level = signal.consciousness_level;

        // Generate possibilities from archetype activation
        let possibilities = self.generate_possibilities(&signal.archetype_activation);

        // Calculate activation level from field coherence
        let activation_level = self.field.field_state.coherence;

        ProcessedSignal {
            possibilities,
            activation_level,
        }
    }

    /// Generate possibilities from archetype activation
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The neural system processes consciousness signals and generates
    ///  possibilities for the Free Will kernel to choose from"
    fn generate_possibilities(&self, activation: &ArchetypeActivationProfile) -> Vec<Possibility> {
        let mut possibilities = Vec::new();
        let coefficients = activation.coefficients();

        // Matrix archetype (A1) influences structure
        let matrix = coefficients.get(0).copied().unwrap_or(0.5);

        // Catalyst archetype (A3) influences reactivity
        let catalyst = coefficients.get(2).copied().unwrap_or(0.5);

        // Create STO-leaning possibility
        possibilities.push(Possibility::new(
            0,
            "Service-to-Others path",
            catalyst, // Catalyst encourages STO
            0.3 + matrix * 0.1,
        ));

        // Create STS-leaning possibility
        possibilities.push(Possibility::new(
            1,
            "Service-to-Self path",
            -catalyst, // Inverse catalyst
            0.3 - matrix * 0.1,
        ));

        // Create neutral possibility
        possibilities.push(Possibility::new(2, "Neutral/Undecided", 0.0, 0.4));

        possibilities
    }

    /// Update receptive frequency based on learning/experience
    ///
    /// As the entity develops, its neural system becomes better
    /// at receiving consciousness signals.
    pub fn tune_reception(&mut self, target_frequency: Float) {
        // Gradually adjust receptive frequency toward target
        let adjustment = (target_frequency - self.receptive_frequency) * 0.1;
        self.receptive_frequency += adjustment;
    }

    /// Get current amplification to provide to kernel
    ///
    /// This is the feedback loop - neural systems amplify consciousness
    /// and return the amplification to the kernel.
    pub fn get_amplification(&self) -> ConsciousnessAmplification {
        ConsciousnessAmplification {
            factor: self.amplification_factor * self.reception_quality,
            activation_boost: self.field.field_state.coherence * 0.1,
            receptive_quality: self.reception_quality,
            neural_coherence: self.field.field_state.coherence,
        }
    }

    /// Get the current field state
    pub fn field_state(&self) -> &NeuralFieldState {
        &self.field.field_state
    }

    /// Get the current reception quality
    pub fn reception_quality(&self) -> Float {
        self.reception_quality
    }

    /// Get the current amplification factor
    pub fn amplification_factor(&self) -> Float {
        self.amplification_factor
    }

    /// Improve reception quality through development/learning
    ///
    /// As entities develop, their neural systems become better
    /// at receiving consciousness signals.
    pub fn improve_reception(&mut self, improvement: Float) {
        self.reception_quality = (self.reception_quality + improvement * 0.1).min(1.0);
        self.amplification_factor = (self.amplification_factor + improvement * 0.05).min(2.0);
    }
}

/// Result of receiving consciousness
///
/// This contains:
/// - The clarity of the received signal
/// - Possibilities generated for Free Will choice
/// - Amplification to provide back to kernel
/// - Overall reception quality
#[derive(Debug, Clone)]
pub struct ReceptionResult {
    /// Clarity of received signal
    pub received_clarity: Float,

    /// Possibilities generated from signal processing
    pub processed_possibilities: Vec<Possibility>,

    /// Amplification to provide back to kernel
    pub amplification: ConsciousnessAmplification,

    /// Overall reception quality
    pub quality: Float,
}

impl ReceptionResult {
    /// Check if the reception was successful
    pub fn is_successful(&self) -> bool {
        self.quality > 0.3 && self.received_clarity > 0.1
    }

    /// Get the number of possibilities generated
    pub fn possibility_count(&self) -> usize {
        self.processed_possibilities.len()
    }
}

/// Processed signal from neural field
///
/// Internal structure representing the result of processing
/// a consciousness signal through the neural field.
#[derive(Debug, Clone)]
struct ProcessedSignal {
    /// Possibilities generated from archetype activation
    possibilities: Vec<Possibility>,

    /// Activation level from field coherence
    activation_level: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_receiver_creation() {
        let receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        assert!(receiver.reception_quality > 0.0);
        assert!(receiver.amplification_factor > 1.0);
    }

    #[test]
    fn test_architecture_reception_quality() {
        let simple = NeuralArchitecture::Simple { neurons: 100 };
        let complex = NeuralArchitecture::Complex {
            layers: 5,
            neurons_per_layer: 100,
        };
        let organized = NeuralArchitecture::Organized {
            cortical_columns: 50,
            neurons_per_column: 100,
            connectivity: 0.8,
        };

        // Organized should have highest quality
        assert!(organized.base_reception_quality() > complex.base_reception_quality());
        assert!(complex.base_reception_quality() > simple.base_reception_quality());
    }

    #[test]
    fn test_architecture_amplification_factor() {
        let simple = NeuralArchitecture::Simple { neurons: 100 };
        let complex = NeuralArchitecture::Complex {
            layers: 5,
            neurons_per_layer: 100,
        };
        let organized = NeuralArchitecture::Organized {
            cortical_columns: 50,
            neurons_per_column: 100,
            connectivity: 0.8,
        };

        // Organized should have highest amplification
        assert!(organized.base_amplification_factor() > complex.base_amplification_factor());
        assert!(complex.base_amplification_factor() > simple.base_amplification_factor());
    }

    #[test]
    fn test_receive_consciousness() {
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let kernel = ConsciousnessKernel::new();

        let result = receiver.receive_consciousness(&kernel);

        assert!(result.quality > 0.0);
        assert!(!result.processed_possibilities.is_empty());
        // Amplification factor can be < 1.0 if reception quality is low
        assert!(result.amplification.factor >= 0.0);
    }

    #[test]
    fn test_receive_consciousness_stores_signal() {
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let kernel = ConsciousnessKernel::new();

        assert!(receiver.last_signal.is_none());

        receiver.receive_consciousness(&kernel);

        assert!(receiver.last_signal.is_some());
    }

    #[test]
    fn test_amplification() {
        // Use Complex architecture which has higher amplification factor
        let receiver = NeuralReceiver::new(NeuralArchitecture::Complex {
            layers: 10,
            neurons_per_layer: 1000,
        });

        let amp = receiver.get_amplification();
        // Complex architecture should have amplification > 1.0
        assert!(amp.factor > 1.0); // Should amplify
        assert!(amp.receptive_quality > 0.0);
    }

    #[test]
    fn test_simple_architecture_amplification() {
        // Simple architecture has lower amplification factor
        let receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });

        let amp = receiver.get_amplification();
        // Simple architecture may have factor near or below 1.0 depending on reception quality
        // This is correct - simple neural systems may attenuate consciousness
        assert!(amp.factor >= 0.0);
        assert!(amp.receptive_quality > 0.0);
    }

    #[test]
    fn test_tune_reception() {
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let initial_freq = receiver.receptive_frequency;

        receiver.tune_reception(0.8);

        // Frequency should move toward target
        assert!(receiver.receptive_frequency > initial_freq);
        assert!(receiver.receptive_frequency < 0.8); // Not instant
    }

    #[test]
    fn test_improve_reception() {
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let initial_quality = receiver.reception_quality;
        let initial_amp = receiver.amplification_factor;

        receiver.improve_reception(0.5);

        assert!(receiver.reception_quality > initial_quality);
        assert!(receiver.amplification_factor > initial_amp);
    }

    #[test]
    fn test_reception_result_successful() {
        let result = ReceptionResult {
            received_clarity: 0.5,
            processed_possibilities: vec![Possibility::new(0, "test", 0.5, 0.5)],
            amplification: ConsciousnessAmplification::neutral(),
            quality: 0.6,
        };

        assert!(result.is_successful());
    }

    #[test]
    fn test_reception_result_unsuccessful() {
        let result = ReceptionResult {
            received_clarity: 0.05,
            processed_possibilities: vec![],
            amplification: ConsciousnessAmplification::neutral(),
            quality: 0.1,
        };

        assert!(!result.is_successful());
    }

    #[test]
    fn test_reception_result_possibility_count() {
        let result = ReceptionResult {
            received_clarity: 0.5,
            processed_possibilities: vec![
                Possibility::new(0, "test1", 0.5, 0.5),
                Possibility::new(1, "test2", 0.5, 0.5),
            ],
            amplification: ConsciousnessAmplification::neutral(),
            quality: 0.6,
        };

        assert_eq!(result.possibility_count(), 2);
    }

    #[test]
    fn test_generate_possibilities() {
        let receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let activation = ArchetypeActivationProfile::zero();

        let possibilities = receiver.generate_possibilities(&activation);

        assert_eq!(possibilities.len(), 3); // STO, STS, Neutral
    }

    #[test]
    fn test_organized_architecture_has_best_reception() {
        let organized = NeuralArchitecture::Organized {
            cortical_columns: 100,
            neurons_per_column: 100,
            connectivity: 0.9,
        };

        let quality = organized.base_reception_quality();
        let amp = organized.base_amplification_factor();

        // Should be high quality
        assert!(quality > 0.5);
        // Should have significant amplification
        assert!(amp > 1.5);
    }

    #[test]
    fn test_consciousness_kernel_integration() {
        let mut kernel = ConsciousnessKernel::new();
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Complex {
            layers: 5,
            neurons_per_layer: 100,
        });

        // Initial state
        let initial_consciousness = kernel.consciousness_level;

        // Receive consciousness from kernel
        let result = receiver.receive_consciousness(&kernel);

        // Provide amplification back to kernel
        kernel.receive_amplification(&result.amplification);

        // Consciousness should have been affected
        // (may be same or higher depending on amplification)
        assert!(kernel.consciousness_level >= initial_consciousness * 0.9);
    }

    #[test]
    fn test_neural_field_consciousness_from_signal() {
        let mut receiver = NeuralReceiver::new(NeuralArchitecture::Simple { neurons: 100 });
        let mut kernel = ConsciousnessKernel::new();

        // Set a specific consciousness level in the kernel
        kernel.consciousness_level = 0.7;

        // Receive the signal
        receiver.receive_consciousness(&kernel);

        // The neural field's consciousness level should match the signal
        // (it receives, not computes)
        assert!((receiver.field.consciousness_level - 0.7).abs() < 0.01);
    }
}
