//! Cosmic Sequence (Phase B)
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "The sequential creation process: undifferentiated
//! Infinity (Violet) → Intelligent Infinity through Free Will (Indigo) → Logos through Love (Blue)
//! → Light/Love field (Green) → Dimensions and the Space/Time-Time/Space spectrum with Veil (Yellow)
//! → Galactic-scale Logos (Orange) → Solar-scale Logos with archetypical minds (Red) → Individual entities (Layer 7)"
//!
//! Each layer "transcends and includes" all previous steps, creating the holographic principle
//! where each entity contains the complete architecture.

use super::field_state::{Complex, Float, HolographicFieldState, OctreeNode};

/// The Seven Layers of Cosmological Involution
/// Each layer represents a distinct stage of cosmic evolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmologicalLayer {
    /// Layer 0: Violet-Ray - The Source (undifferentiated infinity)
    Violet = 0,
    /// Layer 1: Indigo-Ray - Intelligent Infinity (Free Will breaks symmetry)
    Indigo = 1,
    /// Layer 2: Blue-Ray - Logos (Love/Logos creates coherence attractors)
    Blue = 2,
    /// Layer 3: Green-Ray - Light/Love (Light wave propagation)
    Green = 3,
    /// Layer 4: Yellow-Ray - Dimensions (Space/Time ↔ Time/Space with Veil)
    Yellow = 4,
    /// Layer 5: Orange-Ray - Galactic-Scale (Galaxy formation)
    Orange = 5,
    /// Layer 6: Red-Ray - Solar-Scale (Archetypical mind template)
    Red = 6,
    /// Layer 7: Individual Entities (SubSubLogos manifestation)
    Layer7 = 7,
}

impl CosmologicalLayer {
    pub fn count() -> usize {
        8
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn from_index(i: usize) -> Option<CosmologicalLayer> {
        match i {
            0 => Some(CosmologicalLayer::Violet),
            1 => Some(CosmologicalLayer::Indigo),
            2 => Some(CosmologicalLayer::Blue),
            3 => Some(CosmologicalLayer::Green),
            4 => Some(CosmologicalLayer::Yellow),
            5 => Some(CosmologicalLayer::Orange),
            6 => Some(CosmologicalLayer::Red),
            7 => Some(CosmologicalLayer::Layer7),
            _ => None,
        }
    }

    /// Get the characteristic frequency/energy of this layer
    pub fn frequency(&self) -> Float {
        match self {
            CosmologicalLayer::Violet => 1.0, // Ground state - infinite potential
            CosmologicalLayer::Indigo => 2.0, // First distortion - awareness
            CosmologicalLayer::Blue => 4.0,   // Love creates structure
            CosmologicalLayer::Green => 8.0,  // Light enables manifestation
            CosmologicalLayer::Yellow => 16.0, // Dimensions emerge
            CosmologicalLayer::Orange => 32.0, // Galactic scale
            CosmologicalLayer::Red => 64.0,   // Solar/archetypical scale
            CosmologicalLayer::Layer7 => 128.0, // Individual manifestation
        }
    }

    /// Get the color for visualization (RGB)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            CosmologicalLayer::Violet => (128, 0, 128),   // Violet
            CosmologicalLayer::Indigo => (75, 0, 130),    // Indigo
            CosmologicalLayer::Blue => (0, 0, 255),       // Blue
            CosmologicalLayer::Green => (0, 255, 0),      // Green
            CosmologicalLayer::Yellow => (255, 255, 0),   // Yellow
            CosmologicalLayer::Orange => (255, 165, 0),   // Orange
            CosmologicalLayer::Red => (255, 0, 0),        // Red
            CosmologicalLayer::Layer7 => (255, 255, 255), // White - all combined
        }
    }

    /// Get the attractor strength (spiritual gravity pull toward next layer)
    pub fn default_attractor_strength(&self) -> Float {
        match self {
            CosmologicalLayer::Violet => 0.5, // Pulls toward Indigo (awareness)
            CosmologicalLayer::Indigo => 0.4, // Pulls toward Blue (Love/Logos)
            CosmologicalLayer::Blue => 0.4,   // Pulls toward Green (manifestation)
            CosmologicalLayer::Green => 0.3,  // Pulls toward Yellow (dimensions)
            CosmologicalLayer::Yellow => 0.3, // Pulls toward Orange (galactic)
            CosmologicalLayer::Orange => 0.3, // Pulls toward Red (solar)
            CosmologicalLayer::Red => 0.4,    // Pulls toward Layer7 (individual)
            CosmologicalLayer::Layer7 => 0.0, // No pull - entities manifest
        }
    }

    /// Get the next layer in the sequence (transcend and include)
    pub fn next(&self) -> Option<CosmologicalLayer> {
        CosmologicalLayer::from_index(self.index() + 1)
    }
}

/// Configuration for the cosmic sequence
#[derive(Debug, Clone)]
pub struct CosmicSequenceConfig {
    /// Number of layers to activate (1-8)
    pub active_layers: usize,

    /// Initial field coherence at Violet layer
    pub initial_coherence: Float,

    /// Rate of layer transition
    pub layer_transition_rate: Float,

    /// Attractor field strength multiplier
    pub attractor_multiplier: Float,

    /// Enable "transcend and include" principle
    pub transcend_include: bool,
}

impl Default for CosmicSequenceConfig {
    fn default() -> Self {
        CosmicSequenceConfig {
            active_layers: 8,
            initial_coherence: 1.0,
            layer_transition_rate: 0.01,
            attractor_multiplier: 1.0,
            transcend_include: true,
        }
    }
}

/// Attractor-field for a cosmological layer (spiritual gravity)
/// Pulls the field toward the next stage of evolution
#[derive(Debug, Clone)]
pub struct LayerAttractor {
    /// Which layer this attractor belongs to
    pub layer: CosmologicalLayer,

    /// Strength of the attractor (spiritual gravity)
    pub strength: Float,

    /// Range of influence
    pub range: Float,

    /// Center position of attractor [x, y, z]
    pub center: [Float; 3],

    /// Current activation level (0.0 - 1.0)
    pub activation: Float,
}

impl LayerAttractor {
    pub fn new(layer: CosmologicalLayer, center: [Float; 3]) -> Self {
        LayerAttractor {
            layer,
            strength: layer.default_attractor_strength(),
            range: 100.0,
            center,
            activation: 0.0,
        }
    }

    /// Apply attractor to field node - spiritual gravity effect
    pub fn apply(&self, node: &mut OctreeNode) {
        if self.activation <= 0.0 {
            return;
        }

        // Calculate distance from attractor center
        let node_center = node.bounds.center();
        let dx = node_center[0] - self.center[0];
        let dy = node_center[1] - self.center[1];
        let dz = node_center[2] - self.center[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();

        if dist > self.range || dist < 0.001 {
            return;
        }

        // Inverse square falloff (like gravity)
        let falloff = 1.0 - (dist / self.range);
        let force = self.strength * falloff * self.activation;

        // Apply to coherence - attract toward higher coherence
        // This is "spiritual gravity" pulling toward the attractor state
        let coherence_change = force * 0.1;
        node.field_data.coherence = (node.field_data.coherence + coherence_change).clamp(0.0, 1.0);

        // Apply to density amplitudes - shift toward next layer's frequency
        if let Some(next_layer) = self.layer.next() {
            let next_freq = next_layer.frequency();
            let current_freq = self.layer.frequency();
            let _freq_ratio = next_freq / current_freq;

            // Shift amplitude toward next layer's frequency
            for (i, amp) in node.field_data.density_amplitudes.iter_mut().enumerate() {
                let target = if i == next_layer.index() {
                    amp.magnitude() * (1.0 + force * 0.1)
                } else {
                    amp.magnitude() * (1.0 - force * 0.05)
                };
                let new_mag = amp.magnitude() * (1.0 - force * 0.1) + target * force * 0.1;
                let scale = if amp.magnitude() > 0.001 {
                    new_mag / amp.magnitude()
                } else {
                    1.0
                };
                *amp = amp.scale(scale.clamp(0.5, 2.0));
            }
        }
    }
}

/// The complete cosmic sequence driving field evolution
pub struct CosmicSequence {
    /// Configuration
    pub config: CosmicSequenceConfig,

    /// Current active layer (which layer is most dominant)
    pub current_layer: CosmologicalLayer,

    /// Layer activations (0.0 - 1.0 for each layer)
    pub layer_activations: [Float; 8],

    /// Attractor fields for each layer
    pub attractors: Vec<LayerAttractor>,

    /// Evolution time (cumulative)
    pub evolution_time: Float,

    /// Statistics
    pub statistics: CosmicSequenceStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct CosmicSequenceStatistics {
    pub layer_transitions: usize,
    pub total_attractor_applications: usize,
    pub average_coherence: Float,
    pub max_attractor_strength: Float,
}

impl CosmicSequence {
    pub fn new(config: CosmicSequenceConfig) -> Self {
        let mut seq = CosmicSequence {
            config,
            current_layer: CosmologicalLayer::Violet,
            layer_activations: [0.0; 8],
            attractors: Vec::new(),
            evolution_time: 0.0,
            statistics: CosmicSequenceStatistics::default(),
        };

        // Initialize attractors for each layer
        seq.initialize_attractors();

        seq
    }

    pub fn with_defaults() -> Self {
        Self::new(CosmicSequenceConfig::default())
    }

    /// Initialize attractor fields at standard positions
    fn initialize_attractors(&mut self) {
        // Violet at origin
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Violet,
            [0.0, 0.0, 0.0],
        ));

        // Indigo slightly offset
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Indigo,
            [10.0, 0.0, 0.0],
        ));

        // Blue at center
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Blue,
            [0.0, 0.0, 0.0],
        ));

        // Green spreads out
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Green,
            [50.0, 0.0, 0.0],
        ));

        // Yellow - veil position
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Yellow,
            [100.0, 0.0, 0.0],
        ));

        // Orange - galactic scale
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Orange,
            [200.0, 50.0, 0.0],
        ));

        // Red - solar scale
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Red,
            [300.0, 100.0, 50.0],
        ));

        // Layer 7 - individual entities spread
        self.attractors.push(LayerAttractor::new(
            CosmologicalLayer::Layer7,
            [100.0, 100.0, 50.0],
        ));
    }

    /// Initialize the field from Violet (undifferentiated infinity)
    pub fn initialize_field(&mut self, field: &mut HolographicFieldState) {
        // Start at Violet layer - maximum unity, no structure
        field.root.field_data.coherence = self.config.initial_coherence;

        // Violet is the ground state - perfect unity
        // Initialize all density amplitudes equally (undifferentiated)
        for amp in field.root.field_data.density_amplitudes.iter_mut() {
            *amp = Complex::from_polar(1.0 / 8.0, 0.0);
        }

        // Spectrum position at origin (before physical octave)
        field.root.field_data.spectrum_position = 0.0;

        // Activate Violet layer fully
        let violet_idx = CosmologicalLayer::Violet.index();
        self.layer_activations[violet_idx] = 1.0;
    }

    /// Advance the cosmic sequence by one step
    pub fn step(&mut self, field: &mut HolographicFieldState, coherence: Float) {
        self.evolution_time += self.config.layer_transition_rate;

        // Update layer activations based on field coherence
        self.update_layer_activations(coherence);

        // Apply attractor fields
        self.apply_attractors(field);

        // Check for layer transitions
        self.check_layer_transitions(coherence);

        // Update statistics
        self.statistics.average_coherence = coherence;
    }

    /// Update which layers are active based on field state
    fn update_layer_activations(&mut self, coherence: Float) {
        // The higher the coherence, the further along the sequence
        // This implements "transcend and include" - each layer adds to previous

        let _violet_base = 1.0; // Always present at base
        let coherence_factor = coherence.clamp(0.0, 1.0);

        // Each layer activates based on coherence progression
        for i in 0..8 {
            let _layer = CosmologicalLayer::from_index(i).unwrap();
            let target = (coherence_factor * (i + 1) as Float / 8.0).min(1.0);

            // Smooth transition
            let current = self.layer_activations[i];
            self.layer_activations[i] = current + (target - current) * 0.1;
        }

        // Determine current dominant layer
        let mut max_activation = 0.0;
        for (i, &activation) in self.layer_activations.iter().enumerate() {
            if activation > max_activation {
                max_activation = activation;
                self.current_layer = CosmologicalLayer::from_index(i).unwrap();
            }
        }
    }

    /// Apply all active attractor fields to the field
    fn apply_attractors(&mut self, field: &mut HolographicFieldState) {
        // Apply each active attractor
        for attractor in self.attractors.iter_mut() {
            let layer_idx = attractor.layer.index();
            let activation = self.layer_activations[layer_idx];

            if activation > 0.01 {
                attractor.activation = activation * self.config.attractor_multiplier;

                // Apply to root node
                attractor.apply(&mut field.root);

                self.statistics.total_attractor_applications += 1;
                self.statistics.max_attractor_strength = self
                    .statistics
                    .max_attractor_strength
                    .max(attractor.strength * attractor.activation);
            }
        }

        // If transcend_include is enabled, lower layers also affect the field
        // This implements "transcend and include"
        if self.config.transcend_include {
            self.apply_transcend_include(field);
        }
    }

    /// Apply "transcend and include" - lower layers persist even as higher layers emerge
    fn apply_transcend_include(&self, field: &mut HolographicFieldState) {
        // Each layer adds its contribution while higher layers emerge
        let mut total_weight = 0.0;
        for &activation in &self.layer_activations {
            total_weight += activation;
        }

        if total_weight < 0.001 {
            return;
        }

        // Apply weighted contribution from each active layer
        for (i, &activation) in self.layer_activations.iter().enumerate() {
            if activation < 0.01 {
                continue;
            }

            let layer = CosmologicalLayer::from_index(i).unwrap();
            let weight = activation / total_weight;

            // Each layer adds its characteristic frequency contribution
            let freq = layer.frequency();

            // Modulate density amplitudes
            let density_idx = i.min(7);
            if density_idx < field.root.field_data.density_amplitudes.len() {
                let amp = &mut field.root.field_data.density_amplitudes[density_idx];
                let contribution = weight * freq * 0.01;
                let current_mag = amp.magnitude();
                let new_mag = (current_mag + contribution).min(1.0);
                if current_mag > 0.001 {
                    let scale = new_mag / current_mag;
                    *amp = amp.scale(scale);
                }
            }
        }
    }

    /// Check if a layer transition should occur
    fn check_layer_transitions(&mut self, _coherence: Float) {
        let current_idx = self.current_layer.index();

        // Transition occurs when current layer is highly activated
        // and coherence has reached threshold for next layer
        if current_idx < 7 {
            let current_activation = self.layer_activations[current_idx];
            let next_activation = self.layer_activations[current_idx + 1];

            // Transition when next layer starts becoming significant
            if next_activation > 0.3 && current_activation < 0.8 {
                self.statistics.layer_transitions += 1;
            }
        }
    }

    /// Get the current layer for visualization
    pub fn get_current_layer(&self) -> CosmologicalLayer {
        self.current_layer
    }

    /// Get layer activations for visualization
    pub fn get_layer_activations(&self) -> [Float; 8] {
        self.layer_activations
    }

    /// Get attractor data for visualization
    pub fn get_attractor_data(&self) -> Vec<AttractorVisualization> {
        self.attractors
            .iter()
            .map(|a| AttractorVisualization {
                layer: a.layer,
                position: a.center,
                strength: a.strength * a.activation,
                range: a.range,
            })
            .collect()
    }
}

/// Data for visualizing attractor fields
#[derive(Debug, Clone)]
pub struct AttractorVisualization {
    pub layer: CosmologicalLayer,
    pub position: [Float; 3],
    pub strength: Float,
    pub range: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmic_sequence_creation() {
        let seq = CosmicSequence::with_defaults();
        assert_eq!(seq.current_layer, CosmologicalLayer::Violet);
    }

    #[test]
    fn test_layer_frequencies() {
        assert_eq!(CosmologicalLayer::Violet.frequency(), 1.0);
        assert_eq!(CosmologicalLayer::Indigo.frequency(), 2.0);
        assert_eq!(CosmologicalLayer::Blue.frequency(), 4.0);
    }

    #[test]
    fn test_layer_transition() {
        let mut seq = CosmicSequence::with_defaults();
        let mut field = HolographicFieldState::with_defaults();

        seq.initialize_field(&mut field);

        // Step forward with high coherence
        for _ in 0..100 {
            seq.step(&mut field, 0.8);
        }

        // Should have progressed beyond Violet
        assert!(seq.layer_activations[1] > 0.0 || seq.layer_activations[2] > 0.0);
    }
}
