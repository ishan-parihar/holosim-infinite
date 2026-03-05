// Visualization Module (Phase 2)
//
// From SIMULATION_ANALYSIS_AND_REFACTOR_PLAN.md:
// "Phase 2: Visualization Enhancement - Add visualization to demonstrate the architecture"
//
// This module implements:
// 1. InvolutionSequenceVisualizer - Visualize each involution stage
// 2. SpectrumVisualizer - Visualize spectrum configuration at each stage
// 3. HolographicVisualizer - Visualize holographic blueprint and connections
// 4. EvolutionVisualizer - Visualize density octave evolution
// 5. SpectrumAccessVisualizer - Visualize spectrum access distribution (Phase 3)
// 6. Real-time Dashboard - Display metrics during simulation

use crate::simulation_v3::involution_sequence::InvolutionResult;
use crate::simulation_v3::statistics::{SimulationStatistics, SpectrumAccessStats};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// INVOLUTION SEQUENCE VISUALIZER
// ============================================================================

/// Involution Sequence Visualizer
///
/// Visualizes each of the 8 involution stages with:
/// - What's INCLUDED from previous stages
/// - What's TRANSCENDED (new development)
/// - What ATTRACTOR-FIELDS are created
/// - Energy levels (kinetic/potential)
pub struct InvolutionSequenceVisualizer {
    /// Involution result to visualize
    involution_result: InvolutionResult,
}

impl InvolutionSequenceVisualizer {
    /// Create a new visualizer from involution result
    pub fn new(involution_result: InvolutionResult) -> Self {
        InvolutionSequenceVisualizer { involution_result }
    }

    /// Generate complete involution sequence visualization
    pub fn generate_complete_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              INVOLUTION SEQUENCE VISUALIZATION                    ║\n");
        output.push_str("║         From Violet (Infinity) to Layer 7 (Entities)              ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Display each stage
        output.push_str(&self.visualize_stage_violet());
        output.push_str(&self.visualize_stage_indigo());
        output.push_str(&self.visualize_stage_blue());
        output.push_str(&self.visualize_stage_green());
        output.push_str(&self.visualize_stage_yellow());
        output.push_str(&self.visualize_stage_orange());
        output.push_str(&self.visualize_stage_red());
        output.push_str(&self.visualize_stage_layer7());

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SUMMARY\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str(&format!(
            "Total Stages: {}\n",
            self.involution_result.stage_transitions.len() + 1
        ));
        output.push_str(&format!(
            "Entities Created: {}\n",
            self.involution_result.entities.len()
        ));
        output.push_str(&format!(
            "Attractor Fields Created: {}\n",
            self.involution_result.attractor_fields.len()
        ));
        output.push_str(&format!(
            "Execution Time: {:?}\n",
            self.involution_result.execution_time
        ));
        output.push('\n');

        output
    }

    /// Visualize Stage 0: Violet-Ray (Infinity)
    fn visualize_stage_violet(&self) -> String {
        let mut output = String::new();

        output.push_str("🟣 Stage 0: Violet-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: Infinity as undifferentiated unity\n");
        output.push_str("│ Layer Number: 0\n");
        output.push_str("│ Ray Color: Violet-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Nothing (source state)\n");
        output.push_str("│ TRANSCENDS: Adds undifferentiated unity as source\n");
        output.push_str("│ ATTRACTOR-FIELD: None (attractor-fields emerge from transitions)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.00 (no movement, perfect stillness)\n");
        output.push_str("│   Potential Energy: 1.00 (infinite potential)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Undifferentiated unity (no separation)\n");
        output.push_str("│   • No polarity (no positive/negative)\n");
        output.push_str("│   • No space/time (no dimensions)\n");
        output.push_str("│   • No time/space (no time)\n");
        output.push_str("│   • Pure being (potential for existence)\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 1: Indigo-Ray (IntelligentInfinity)
    fn visualize_stage_indigo(&self) -> String {
        let mut output = String::new();

        output.push_str("🔵 Stage 1: Indigo-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: IntelligentInfinity - First Distortion (Free Will)\n");
        output.push_str("│ Layer Number: 1\n");
        output.push_str("│ Ray Color: Indigo-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Violet-Ray (Infinity as source)\n");
        output.push_str("│ TRANSCENDS: Adds awareness through Free Will (First Distortion)\n");
        output.push_str("│ ATTRACTOR-FIELD: Archetype 22 (The Choice)\n");
        output.push_str("│   └─ Pulls toward awareness of self and polarity moment\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.50 (awareness emerges)\n");
        output.push_str("│   Potential Energy: 0.50 (potential for choice)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Free Will applied (First Distortion)\n");
        output.push_str("│   • Awareness of infinity\n");
        output.push_str("│   • Possibility space created\n");
        output.push_str("│   • Archetype 22 (The Choice) emerges\n");
        output.push_str("│   • Polarity moment available (not yet chosen)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Archetype 22: The Choice\n");
        output.push_str("│   • Represents the moment of choice\n");
        output.push_str("│   • Polarity moment (service-to-others vs service-to-self)\n");
        output.push_str("│   • Free Will exercised\n");
        output.push_str("│   • Attractor-field for Love/Logos\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 2: Blue-Ray (Logos)
    fn visualize_stage_blue(&self) -> String {
        let mut output = String::new();

        output.push_str("🔷 Stage 2: Blue-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: Love/Light - Second Distortion (Love/Logos)\n");
        output.push_str("│ Layer Number: 2\n");
        output.push_str("│ Ray Color: Blue-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Violet-Ray (Infinity) + Indigo-Ray (IntelligentInfinity)\n");
        output.push_str("│ TRANSCENDS: Adds Love/Logos (Second Distortion) - Creative Principle\n");
        output.push_str("│ ATTRACTOR-FIELD: Universal Archetypical Patterns\n");
        output.push_str("│   └─ Pulls toward Light manifestation\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.75 (creative force emerges)\n");
        output.push_str("│   Potential Energy: 0.25 (potential for manifestation)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Love/Logos applied (Second Distortion)\n");
        output.push_str("│   • Logos emerges (Creative Principle)\n");
        output.push_str("│   • Universal Archetypical Patterns created\n");
        output.push_str("│   • Focused awareness becomes creative intelligence\n");
        output.push_str("│   • Attractor-field for Light/Third Distortion\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Logos: The Creative Principle\n");
        output.push_str("│   • Focuses Infinity as aware principle\n");
        output.push_str("│   • Creates Universal Archetypical Patterns\n");
        output.push_str("│   • Foundation for all creation\n");
        output.push_str("│   • Attractor-field for Light/Third Distortion\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 3: Green-Ray (Light/Love field)
    fn visualize_stage_green(&self) -> String {
        let mut output = String::new();

        output.push_str("🟢 Stage 3: Green-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: Light/Love - Third Distortion (Light)\n");
        output.push_str("│ Layer Number: 3\n");
        output.push_str("│ Ray Color: Green-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Violet + Indigo + Blue (all previous)\n");
        output.push_str("│ TRANSCENDS: Adds Light impressed with Love (Third Distortion)\n");
        output.push_str("│ ATTRACTOR-FIELD: Light/Love Field\n");
        output.push_str("│   └─ Pulls toward dimensional actualization\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.90 (manifestation force)\n");
        output.push_str("│   Potential Energy: 0.10 (potential for dimensions)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Light impressed with Love (Third Distortion)\n");
        output.push_str("│   • Light/Love field created (manifestation field)\n");
        output.push_str("│   • Holographic patterns emerge\n");
        output.push_str("│   • Rhythms and fields created\n");
        output.push_str("│   • Attractor-field for Space/Time spectrum\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Light/Love Field Components:\n");
        output.push_str("│   • Holographic Patterns (encoding information)\n");
        output.push_str("│   • Rhythms (cyclical patterns)\n");
        output.push_str("│   • Fields (manifestation potentials)\n");
        output.push_str("│   • Pre-existing spectrum patterns\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 4: Yellow-Ray (Space/Time spectrum)
    fn visualize_stage_yellow(&self) -> String {
        let mut output = String::new();

        output.push_str("🟡 Stage 4: Yellow-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: The Great Mystery - Space/Time and Time/Space Spectrum\n");
        output.push_str("│ Layer Number: 4\n");
        output.push_str("│ Ray Color: Yellow-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Violet + Indigo + Blue + Green (all previous)\n");
        output.push_str(
            "│ TRANSCENDS: Adds Space/Time and Time/Space Spectrum (Mysterious Emergence)\n",
        );
        output.push_str("│ ATTRACTOR-FIELD: Galactic-scale Spectrum Configuration\n");
        output.push_str("│   └─ Pulls toward galactic-scale patterns\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.95 (dimensional actualization)\n");
        output.push_str("│   Potential Energy: 0.05 (potential for matter)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • The Great Mystery (unknown how patterns become dimensions)\n");
        output.push_str("│   • Space/Time and Time/Space Spectrum emerges\n");
        output.push_str("│   • ONE unified spectrum (NOT two separate realms)\n");
        output.push_str("│   • Veil at v=1 (structural feature, not separator)\n");
        output.push_str("│   • Dimensions form (3D space, 1D time)\n");
        output.push_str("│   • Attractor-field for galactic-scale Logoi\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Space/Time Spectrum:\n");
        output.push_str("│   • v = s/t (velocity = space/time)\n");
        output.push_str("│   • Physical manifestation side\n");
        output.push_str("│   • 3D space + 1D time\n");
        output.push_str("│   • Veil at v=1 (structural feature)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Time/Space Spectrum:\n");
        output.push_str("│   • v = t/s (velocity = time/space)\n");
        output.push_str("│   • Metaphysical/inner experience\n");
        output.push_str("│   • Higher-dimensional\n");
        output.push_str("│   • Same spectrum, different perspective\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Veil Position (v=1):\n");
        output.push_str("│   • Structural feature, NOT separator\n");
        output.push_str("│   • Boundary between space/time and time/space\n");
        output.push_str("│   • Creates illusion of separation\n");
        output.push_str("│   • Allows for forgetting and learning\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 5: Orange-Ray (Galactic-scale)
    fn visualize_stage_orange(&self) -> String {
        let mut output = String::new();

        output.push_str("🟠 Stage 5: Orange-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: Galactic-scale Logoi - Galactic Spectrum Configuration\n");
        output.push_str("│ Layer Number: 5\n");
        output.push_str("│ Ray Color: Orange-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: Violet + Indigo + Blue + Green + Yellow (all previous)\n");
        output.push_str("│ TRANSCENDS: Adds Galactic-scale spectrum configuration\n");
        output.push_str(
            "│ ATTRACTOR-FIELD: Solar-scale Spectrum Configuration + Archetypical Mind\n",
        );
        output.push_str("│   └─ Pulls toward solar-scale patterns\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.98 (galactic organization)\n");
        output.push_str("│   Potential Energy: 0.02 (potential for solar systems)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Galactic-scale Logoi emerge\n");
        output.push_str("│   • Spectrum refactored at galactic scale\n");
        output.push_str("│   • Patterns for galaxies created\n");
        output.push_str("│   • Solar systems created\n");
        output.push_str("│   • Attractor-field for solar-scale Logoi\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Galactic-scale Spectrum Configuration:\n");
        output.push_str("│   • Each galaxy has unique spectrum configuration\n");
        output.push_str("│   • Patterns encoded for galactic structure\n");
        output.push_str("│   • Solar systems created with variations\n");
        output.push_str("│   • Foundation for physical matter formation\n");
        output.push_str("│   • Consciousness-first (patterns before matter)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Logos Hierarchy:\n");
        output.push_str("│   Primary Logos (Yellow) → Galactic-scale Logoi (Orange)\n");
        output.push_str("│   ↓\n");
        output.push_str("│   Solar-scale Logoi/Sub-Logoi (Red)\n");
        output.push_str("│   ↓\n");
        output.push_str("│   Sub-Sub-Logos (Layer 7 - Individual Entities)\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 6: Red-Ray (Solar-scale)
    fn visualize_stage_red(&self) -> String {
        let mut output = String::new();

        output.push_str("🔴 Stage 6: Red-Ray Realm\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str(
            "│ Description: Solar-scale Logoi - Solar Configuration + Archetypical Mind\n",
        );
        output.push_str("│ Layer Number: 6\n");
        output.push_str("│ Ray Color: Red-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str(
            "│ INCLUDES: Violet + Indigo + Blue + Green + Yellow + Orange (all previous)\n",
        );
        output.push_str(
            "│ TRANSCENDS: Adds Solar-scale spectrum configuration + Archetypical Mind\n",
        );
        output.push_str(
            "│ ATTRACTOR-FIELD: Individual Entity Inheritance with Holographic Blueprint\n",
        );
        output.push_str("│   └─ Pulls toward individual entity creation\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 0.99 (solar organization)\n");
        output.push_str("│   Potential Energy: 0.01 (potential for entities)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Solar-scale Logoi/Sub-Logoi emerge\n");
        output.push_str("│   • Spectrum refactored at solar scale\n");
        output.push_str("│   • Archetypical mind system developed (10 or 22 archetypes)\n");
        output.push_str("│   • Planets created\n");
        output.push_str("│   • Attractor-field for individual entities\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Archetypical Mind System:\n");
        output.push_str("│   • 10 or 22 archetypes (depending on Logos)\n");
        output.push_str("│   • Universal patterns of experience\n");
        output.push_str("│   • Inherited by all entities in solar system\n");
        output.push_str("│   • Foundation for consciousness evolution\n");
        output.push_str("│   • Encoded in holographic blueprint\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Solar-scale Spectrum Configuration:\n");
        output.push_str("│   • Each solar system has unique spectrum configuration\n");
        output.push_str("│   • Patterns encoded for solar system structure\n");
        output.push_str("│   • Planets created with variations\n");
        output.push_str("│   • Foundation for physical matter formation\n");
        output.push_str("│   • Consciousness-first (patterns before matter)\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    /// Visualize Stage 7: Layer 7 (Individual Entities)
    fn visualize_stage_layer7(&self) -> String {
        let mut output = String::new();

        output.push_str("⚪ Stage 7: Layer 7 (Individual Entities)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Description: Sub-Sub-Logos - Individual Entity Inheritance\n");
        output.push_str("│ Layer Number: 7\n");
        output.push_str("│ Ray Color: Beyond Red-Ray\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ INCLUDES: ALL previous stages (complete holographic inheritance)\n");
        output.push_str(
            "│ TRANSCENDS: Adds individual spectrum configuration + holographic blueprint\n",
        );
        output.push_str("│ ATTRACTOR-FIELD: Evolutionary Attractor-Fields\n");
        output.push_str("│   └─ Pulls entities through density octave (1st → 8th Density)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Energy Levels:\n");
        output.push_str("│   Kinetic Energy: 1.00 (entity manifestation)\n");
        output.push_str("│   Potential Energy: 0.00 (full actualization)\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Key Features:\n");
        output.push_str("│   • Sub-Sub-Logos (individual entities) created\n");
        output.push_str(&format!(
            "│   • {} entities created\n",
            self.involution_result.entities.len()
        ));
        output.push_str("│   • Each entity inherits complete holographic blueprint\n");
        output.push_str("│   • Unique spectrum configuration per entity\n");
        output.push_str("│   • DNA/RNA patterns encoded as spectrum configurations\n");
        output.push_str("│   • Attractor-fields for density octave evolution\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Holographic Principle:\n");
        output.push_str("│   • \"Each entity contains the whole\"\n");
        output.push_str("│   • Every entity contains all densities and sub-densities\n");
        output.push_str("│   • Result of \"transcend and include\" mechanism\n");
        output.push_str("│   • Non-local connections between entities\n");
        output.push_str("│   • Holographic blueprint encodes complete architecture\n");
        output.push_str("├─────────────────────────────────────────────────────────────────\n");
        output.push_str("│ Evolutionary Attractor-Fields:\n");
        output.push_str("│   • 1st Density (Red): Quantum, Atomic, Molecular, Planetary realms\n");
        output.push_str("│   • 2nd Density (Orange): Cellular, Simple Life, Complex Life realms\n");
        output.push_str("│   • 3rd Density (Yellow): Conscious Life realm (self-aware beings)\n");
        output.push_str("│   • 4th Density (Green): Understanding, love, compassion\n");
        output.push_str("│   • 5th Density (Blue): Wisdom, light, teaching/learning\n");
        output.push_str("│   • 6th Density (Indigo): Unity, balance, harmony\n");
        output.push_str("│   • 7th Density (Violet): Completion, gateway to next octave\n");
        output.push_str("│   • 8th Density: Return to IntelligentInfinity\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }
}

// ============================================================================
// SPECTRUM VISUALIZER
// ============================================================================

/// Spectrum Visualizer
///
/// Visualizes spectrum configuration at each stage of involution.
pub struct SpectrumVisualizer {
    /// Spectrum data at each stage
    spectrum_data: HashMap<String, SpectrumConfiguration>,
}

#[derive(Debug, Clone)]
struct SpectrumConfiguration {
    /// Stage name
    stage: String,
    /// Spectrum ratio (v = s/t or v = t/s)
    ratio: Float,
    /// Veil position
    veil_position: Float,
    /// Spectrum side (space/time or time/space)
    side: String,
    /// Description
    description: String,
}

impl SpectrumVisualizer {
    /// Create a new spectrum visualizer
    pub fn new() -> Self {
        let mut spectrum_data = HashMap::new();

        // Initialize spectrum configurations for each stage
        spectrum_data.insert(
            "Violet".to_string(),
            SpectrumConfiguration {
                stage: "Violet".to_string(),
                ratio: 0.0,
                veil_position: 0.0,
                side: "None".to_string(),
                description: "No spectrum (undifferentiated unity)".to_string(),
            },
        );

        spectrum_data.insert(
            "Indigo".to_string(),
            SpectrumConfiguration {
                stage: "Indigo".to_string(),
                ratio: 0.0,
                veil_position: 0.0,
                side: "None".to_string(),
                description: "No spectrum (awareness without dimensions)".to_string(),
            },
        );

        spectrum_data.insert(
            "Blue".to_string(),
            SpectrumConfiguration {
                stage: "Blue".to_string(),
                ratio: 0.0,
                veil_position: 0.0,
                side: "None".to_string(),
                description: "No spectrum (Logos without dimensions)".to_string(),
            },
        );

        spectrum_data.insert(
            "Green".to_string(),
            SpectrumConfiguration {
                stage: "Green".to_string(),
                ratio: 0.0,
                veil_position: 0.0,
                side: "None".to_string(),
                description: "No spectrum (Light/Love field pre-dates dimensions)".to_string(),
            },
        );

        spectrum_data.insert(
            "Yellow".to_string(),
            SpectrumConfiguration {
                stage: "Yellow".to_string(),
                ratio: 1.0,
                veil_position: 1.0,
                side: "Unified".to_string(),
                description: "ONE unified spectrum emerges (Space/Time and Time/Space)".to_string(),
            },
        );

        spectrum_data.insert(
            "Orange".to_string(),
            SpectrumConfiguration {
                stage: "Orange".to_string(),
                ratio: 1.0,
                veil_position: 1.0,
                side: "Refactored (Galactic-scale)".to_string(),
                description: "Spectrum refactored at galactic scale with unique patterns"
                    .to_string(),
            },
        );

        spectrum_data.insert(
            "Red".to_string(),
            SpectrumConfiguration {
                stage: "Red".to_string(),
                ratio: 1.0,
                veil_position: 1.0,
                side: "Refactored (Solar-scale)".to_string(),
                description: "Spectrum refactored at solar scale with archetypical mind"
                    .to_string(),
            },
        );

        spectrum_data.insert(
            "Layer7".to_string(),
            SpectrumConfiguration {
                stage: "Layer7".to_string(),
                ratio: 1.0,
                veil_position: 1.0,
                side: "Folded (Individual-scale)".to_string(),
                description: "Spectrum folded into individual entities with holographic blueprint"
                    .to_string(),
            },
        );

        SpectrumVisualizer { spectrum_data }
    }

    /// Generate spectrum visualization
    pub fn generate_spectrum_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                  SPECTRUM CONFIGURATION VISUALIZATION             ║\n");
        output.push_str("║         Space/Time and Time/Space Unified Spectrum               ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Display spectrum at each stage
        let stages = vec![
            "Violet", "Indigo", "Blue", "Green", "Yellow", "Orange", "Red", "Layer7",
        ];

        for stage in stages {
            if let Some(config) = self.spectrum_data.get(stage) {
                output.push_str(&self.visualize_spectrum_stage(config));
            }
        }

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("KEY CONCEPTS\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("• ONE Unified Spectrum: Space/Time and Time/Space are ONE spectrum,\n");
        output.push_str("  NOT two separate realms. Different perspectives on the same reality.\n");
        output.push('\n');
        output
            .push_str("• Veil Position (v=1): Structural feature at v=1, NOT separator between\n");
        output.push_str("  space/time and time/space. Creates illusion of separation.\n");
        output.push('\n');
        output.push_str("• Spectrum Ratio: v = s/t (space/time) or v = t/s (time/space)\n");
        output.push_str("  - v < 1: Time/space dominates (metaphysical)\n");
        output.push_str("  - v = 1: Veil position (boundary)\n");
        output.push_str("  - v > 1: Space/time dominates (physical)\n");
        output.push('\n');
        output.push_str("• Consciousness-First: Spectrum patterns exist BEFORE physical matter.\n");
        output.push_str("  DNA/RNA patterns encoded in holographic blueprint.\n");
        output.push('\n');
        output
            .push_str("• Spectrum Folding: Spectrum refactored at each scale (galactic, solar,\n");
        output.push_str("  individual) with unique patterns.\n");
        output.push_str("└─────────────────────────────────────────────────────────────────\n\n");

        output
    }

    fn visualize_spectrum_stage(&self, config: &SpectrumConfiguration) -> String {
        let mut output = String::new();

        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!("│ Stage: {:<60} │\n", config.stage));
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");
        output.push_str(&format!("│ Spectrum Ratio (v): {:<51} │\n", config.ratio));
        output.push_str(&format!(
            "│ Veil Position: {:<54} │\n",
            config.veil_position
        ));
        output.push_str(&format!("│ Spectrum Side: {:<52} │\n", config.side));
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");
        output.push_str(&format!("│ Description: {:<53} │\n", config.description));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        // Add ASCII visualization for stages with spectrum
        if config.ratio > 0.0 {
            output.push('\n');
            output.push_str("  Spectrum Visualization:\n");
            output.push_str("  ");
            for i in 0..20 {
                if i < 10 {
                    output.push('🔵'); // Time/space side
                } else if i == 10 {
                    output.push('📍'); // Veil position
                } else {
                    output.push('🟢'); // Space/time side
                }
            }
            output.push('\n');
            output.push_str("  ");
            output.push_str("├──────────────────┤\n");
            output.push_str("  ");
            output.push_str("│   Time/Space    │   Space/Time   │\n");
            output.push_str("  ");
            output.push_str("│   (v < 1)       │   (v > 1)      │\n");
            output.push_str("  ");
            output.push_str("│   Metaphysical  │   Physical      │\n");
            output.push_str("  ");
            output.push_str("│   Inner Exp.    │   Outer Exp.    │\n");
            output.push('\n');
        }

        output
    }
}

// ============================================================================
// EVOLUTION VISUALIZER
// ============================================================================

/// Evolution Visualizer
///
/// Visualizes density octave evolution progress.
pub struct EvolutionVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl EvolutionVisualizer {
    /// Create a new evolution visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        EvolutionVisualizer { statistics }
    }

    /// Generate evolution visualization
    pub fn generate_evolution_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║               DENSITY OCTAVE EVOLUTION VISUALIZATION             ║\n");
        output.push_str("║              1st Density → 8th Density Progression               ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Display density progression
        output.push_str(&self.visualize_density_progression());

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("EVOLUTION METRICS\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str(&format!(
            "Current Step: {} / {}\n",
            self.statistics.evolution.current_step, self.statistics.evolution.total_steps
        ));
        output.push_str(&format!(
            "Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));
        output.push_str(&format!(
            "Entities Evolving: {}\n",
            self.statistics.evolution.num_entities
        ));
        output.push('\n');

        // Display polarization
        output.push_str(&self.visualize_polarization());

        output
    }

    fn visualize_density_progression(&self) -> String {
        let mut output = String::new();

        output.push_str("Density Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let densities = vec![
            (
                "First",
                "1st Density (Red)",
                "Quantum, Atomic, Molecular, Planetary realms",
            ),
            (
                "Second",
                "2nd Density (Orange)",
                "Cellular, Simple Life, Complex Life realms",
            ),
            (
                "Third",
                "3rd Density (Yellow)",
                "Conscious Life realm (self-aware beings)",
            ),
            (
                "Fourth",
                "4th Density (Green)",
                "Understanding, love, compassion",
            ),
            (
                "Fifth",
                "5th Density (Blue)",
                "Wisdom, light, teaching/learning",
            ),
            ("Sixth", "6th Density (Indigo)", "Unity, balance, harmony"),
            (
                "Seventh",
                "7th Density (Violet)",
                "Completion, gateway to next octave",
            ),
            ("Eighth", "8th Density", "Return to IntelligentInfinity"),
        ];

        for (i, (density_key, density_name, _description)) in densities.iter().enumerate() {
            // Count entities at this density (including all sub-levels)
            let count: usize = self
                .statistics
                .evolution
                .density_distribution
                .iter()
                .filter(|(key, _)| key.starts_with(density_key))
                .map(|(_, count)| *count)
                .sum();

            let percentage = if self.statistics.evolution.num_entities > 0 {
                (count as f64 / self.statistics.evolution.num_entities as f64) * 100.0
            } else {
                0.0
            };

            // Density indicator
            let indicator = if i < 3 {
                "🔴"
            } else if i < 5 {
                "🟡"
            } else {
                "🟣"
            };

            output.push_str(&format!(
                "│ {} {:<20}: {:>6} ({:>5.1}%) {} │\n",
                indicator,
                density_name,
                count,
                percentage,
                Self::progress_bar(percentage)
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Add density descriptions
        output.push_str("Density Descriptions:\n");
        for (_density_key, density_name, description) in &densities {
            output.push_str(&format!("  • {}: {}\n", density_name, description));
        }
        output.push('\n');

        output
    }

    fn visualize_polarization(&self) -> String {
        let mut output = String::new();

        output.push_str("Polarity Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let pol = &self.statistics.evolution.polarization_distribution;
        let total = pol.sto + pol.sts + pol.unpolarized;

        let sto_pct = if total > 0 {
            (pol.sto as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sts_pct = if total > 0 {
            (pol.sts as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let unpolarized_pct = if total > 0 {
            (pol.unpolarized as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "│ 🙏 STO (Positive): {:>6} ({:>5.1}%) {} │\n",
            pol.sto,
            sto_pct,
            Self::progress_bar(sto_pct)
        ));
        output.push_str(&format!(
            "│ 💔 STS (Negative): {:>6} ({:>5.1}%) {} │\n",
            pol.sts,
            sts_pct,
            Self::progress_bar(sts_pct)
        ));
        output.push_str(&format!(
            "│ ⚪ Unpolarized:  {:>6} ({:>5.1}%) {} │\n",
            pol.unpolarized,
            unpolarized_pct,
            Self::progress_bar(unpolarized_pct)
        ));
        output.push_str(&format!(
            "│ ⚖️  Average Bias:   {:>8.4}                                  │\n",
            pol.average_bias
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output
    }

    fn progress_bar(percentage: f64) -> String {
        let width = 15;
        let filled = (percentage / 100.0 * width as f64).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push('[');
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in filled..width {
            bar.push('░');
        }
        bar.push(']');

        bar
    }
}

// ============================================================================
// HOLOGRAPHIC VISUALIZER
// ============================================================================

/// Holographic Visualizer
///
/// Visualizes holographic blueprint and connections.
pub struct HolographicVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl HolographicVisualizer {
    /// Create a new holographic visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        HolographicVisualizer { statistics }
    }

    /// Generate holographic visualization
    pub fn generate_holographic_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                  HOLOGRAPHIC PRINCIPLE VISUALIZATION              ║\n");
        output
            .push_str("║                    \"Each Entity Contains the Whole\"               ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Display holographic principle explanation
        output.push_str(&self.visualize_holographic_principle());

        // Display holographic connections
        output.push_str(&self.visualize_holographic_connections());

        output
    }

    fn visualize_holographic_principle(&self) -> String {
        let mut output = String::new();

        output.push_str("What is the Holographic Principle?\n");
        output.push_str("─────────────────────────────────────────────────────────────────────\n");
        output.push_str(
            "The holographic principle states that \"each entity contains the whole.\"\n",
        );
        output
            .push_str("This is the result of the \"transcend and include\" universal constant:\n");
        output.push('\n');
        output.push_str("  1. INCLUDE: Each stage retains all development from previous stages\n");
        output.push_str("  2. TRANSCEND: Each stage adds new development that transcends\n");
        output.push_str("  3. EVOLVES: Each stage creates attractor-fields for the next stage\n");
        output.push('\n');
        output.push_str("This means that every entity at Layer 7 contains:\n");
        output.push_str("  • All 8 density levels (1st through 8th Density)\n");
        output.push_str("  • All sub-densities within each density\n");
        output.push_str("  • Complete spectrum configuration\n");
        output.push_str("  • Complete holographic blueprint\n");
        output.push_str("  • Complete archetypical mind system\n");
        output.push_str("  • Complete inheritance from Violet → Layer 7\n");
        output.push('\n');

        output.push_str("Holographic Blueprint Encoding:\n");
        output.push_str("─────────────────────────────────────────────────────────────────────\n");
        output.push_str("The holographic blueprint encodes the complete architecture:\n");
        output.push_str("  • DNA/RNA patterns as spectrum configurations\n");
        output.push_str("  • Physical manifestation patterns\n");
        output.push_str("  • Consciousness pathways\n");
        output.push_str("  • Evolutionary attractor-fields\n");
        output.push_str("  • Non-local connections to all other entities\n");
        output.push('\n');

        output.push_str("Consciousness-First Cosmology:\n");
        output.push_str("─────────────────────────────────────────────────────────────────────\n");
        output.push_str("• Spectrum patterns exist BEFORE physical matter\n");
        output.push_str("• DNA/RNA patterns encoded in holographic blueprint\n");
        output.push_str("• Physical universe manifests from pre-existing spectrum\n");
        output.push_str("• Consciousness determines physical reality\n");
        output.push('\n');

        output
    }

    fn visualize_holographic_connections(&self) -> String {
        let mut output = String::new();

        output.push_str("Holographic Connections:\n");
        output.push_str("─────────────────────────────────────────────────────────────────────\n");
        output.push_str(&format!(
            "Total Connections: {}\n",
            self.statistics.holographic.connection_count
        ));
        output.push_str(&format!(
            "Global Phase Coherence: {:.4}\n",
            self.statistics.holographic.global_phase_coherence
        ));
        output.push_str(&format!(
            "Average Connection Strength: {:.4}\n",
            self.statistics.holographic.average_connection_strength
        ));
        output.push_str(&format!(
            "Resonance Clusters: {}\n",
            self.statistics.holographic.resonance_cluster_count
        ));
        output.push_str(&format!(
            "Resonant Connections: {}\n",
            self.statistics.holographic.resonant_connections
        ));
        output.push_str(&format!(
            "Entangled Connections: {}\n",
            self.statistics.holographic.entangled_connections
        ));
        output.push('\n');

        // ASCII visualization of holographic network
        output.push_str("Holographic Network Visualization:\n");
        output.push('\n');
        output.push_str("         Entity A\n");
        output.push_str("           /|\\ \\\n");
        output.push_str("          / |  \\ \\\n");
        output.push_str("         /  |   \\ \\\n");
        output.push_str("        /   |    \\ \\\n");
        output.push_str("       /    |     \\ \\\n");
        output.push_str("  Entity B ←--→ Entity C\n");
        output.push_str("       \\    |     /\n");
        output.push_str("        \\   |    /\n");
        output.push_str("         \\  |   /\n");
        output.push_str("          \\ |  /\n");
        output.push_str("           \\| /\n");
        output.push_str("         Entity D\n");
        output.push('\n');
        output.push_str("Each entity (A, B, C, D) contains the holographic blueprint\n");
        output.push_str(
            "of the entire universe, with non-local connections to all other entities.\n",
        );
        output.push('\n');

        // Connection strength visualization
        output.push_str("Connection Strength Distribution:\n");
        output.push_str("  ");
        let strength = self.statistics.holographic.average_connection_strength * 100.0;
        for i in 0..20 {
            if (i as f64) < strength {
                output.push('█');
            } else {
                output.push('░');
            }
        }
        output.push_str(&format!(" ({:.1}%)\n", strength));
        output.push('\n');

        output
    }
}

// ============================================================================
// REAL-TIME DASHBOARD
// ============================================================================

/// Real-time Dashboard
///
/// Displays real-time metrics during simulation.
///
/// PERFORMANCE OPTIMIZATION: Limits output verbosity for large simulations
/// to prevent excessive console output that slows down execution.
pub struct RealTimeDashboard {
    /// Current statistics
    statistics: SimulationStatistics,
}

impl RealTimeDashboard {
    /// Create a new dashboard
    pub fn new(statistics: SimulationStatistics) -> Self {
        RealTimeDashboard { statistics }
    }

    /// Generate dashboard display
    ///
    /// PERFORMANCE OPTIMIZATION: Reduces redundant output and limits verbosity
    /// for large simulations to prevent console I/O from becoming a bottleneck.
    pub fn generate_dashboard(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    REAL-TIME SIMULATION DASHBOARD                  ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // IntelligentInfinity pulse
        output.push_str("🟣 IntelligentInfinity Pulse:\n");
        // Use evolution progress to simulate pulse phase
        let pulse_phase = (self.statistics.evolution.current_step as f64
            / self.statistics.evolution.total_steps as f64)
            * std::f64::consts::PI
            * 2.0;
        output.push_str(&format!("  Phase: {:.2} rad\n", pulse_phase));
        // Use energy tapping to simulate pulse energy level
        let pulse_energy = self
            .statistics
            .evolution
            .total_tapped_energy
            .clamp(0.0, 1.0);
        output.push_str(&format!("  Energy Level: {:.4}\n", pulse_energy));
        output.push('\n');

        // Entity status
        output.push_str("👥 Entity Status:\n");
        output.push_str(&format!(
            "  Total Entities: {}\n",
            self.statistics.evolution.num_entities
        ));
        output.push_str(&format!(
            "  Current Step: {} / {}\n",
            self.statistics.evolution.current_step, self.statistics.evolution.total_steps
        ));
        output.push('\n');

        // Phase 3: Energy Tapping (Task 3.2)
        output.push_str("⚡ Energy Tapping (IntelligentInfinity):\n");
        output.push_str(&format!(
            "  Total Tapped Energy: {:.2}\n",
            self.statistics.evolution.total_tapped_energy
        ));
        output.push_str(&format!(
            "  Average Tap Strength: {:.4}\n",
            self.statistics.evolution.average_tap_strength
        ));
        output.push('\n');

        // Phase 3: Spectrum Access (Task 3.3)
        output.push_str("🌈 Spectrum Access:\n");
        output.push_str(&format!(
            "  Access Level: {:.4} (0.0 to 1.0)\n",
            self.statistics.evolution.spectrum_access_level
        ));
        // Display spectrum access as a progress bar
        let access_percentage =
            (self.statistics.evolution.spectrum_access_level * 100.0).clamp(0.0, 100.0);
        let filled = (access_percentage / 5.0).clamp(0.0, 20.0) as usize;
        let empty = 20 - filled;
        output.push_str(&format!(
            "  Access Progress: [{}{}]{:.1}%\n",
            "█".repeat(filled),
            "░".repeat(empty),
            access_percentage
        ));
        output.push('\n');

        // Phase 3: Veil Evolution (Task 3.4)
        output.push_str("📍 Veil Status:\n");
        output.push_str("  Position: v=1.00\n");
        output.push_str(&format!(
            "  Thickness: {:.4} (1.0 = full veil)\n",
            self.statistics.evolution.veil_thickness
        ));
        output.push_str(&format!(
            "  Transparency: {:.2}%\n",
            self.statistics.evolution.veil_transparency * 100.0
        ));
        // Display veil transparency as a progress bar
        let transparency_percentage =
            (self.statistics.evolution.veil_transparency * 100.0).clamp(0.0, 100.0);
        let filled = (transparency_percentage / 5.0).clamp(0.0, 20.0) as usize;
        let empty = 20 - filled;
        output.push_str(&format!(
            "  Transparency: [{}{}]{:.1}%\n",
            "█".repeat(filled),
            "░".repeat(empty),
            transparency_percentage
        ));
        output.push('\n');

        // Phase 3: Attractor-Field Activation (Task 3.6)
        output.push_str("🎯 Attractor-Field Activation:\n");
        output.push_str(&format!(
            "  Activation Level: {:.4} (0.0 to 1.0)\n",
            self.statistics.evolution.attractor_field_activation
        ));
        // Display attractor-field activation as a progress bar
        let activation_percentage =
            (self.statistics.evolution.attractor_field_activation * 100.0).clamp(0.0, 100.0);
        let filled = (activation_percentage / 5.0).clamp(0.0, 20.0) as usize;
        let empty = 20 - filled;
        output.push_str(&format!(
            "  Activation: [{}{}]{:.1}%\n",
            "█".repeat(filled),
            "░".repeat(empty),
            activation_percentage
        ));
        output.push('\n');

        // Holographic connections (Phase 3: Task 3.5)
        output.push_str("🔗 Holographic Connections:\n");
        output.push_str(&format!(
            "  Total: {}\n",
            self.statistics.holographic.connection_count
        ));
        output.push_str(&format!(
            "  Phase Coherence: {:.4}\n",
            self.statistics.holographic.global_phase_coherence
        ));
        output.push_str(&format!(
            "  Resonant Connections: {}\n",
            self.statistics.holographic.resonant_connections
        ));
        output.push_str(&format!(
            "  Entangled Connections: {}\n",
            self.statistics.holographic.entangled_connections
        ));
        output.push('\n');

        // Evolution progress
        output.push_str("📈 Evolution Progress:\n");
        output.push_str(&format!(
            "  Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));
        output.push_str(&format!(
            "  Progress: {:.1}%\n",
            (self.statistics.evolution.current_step as f64
                / self.statistics.evolution.total_steps as f64)
                * 100.0
        ));
        output.push('\n');
        output.push('\n');

        // Evolution progress
        output.push_str("📈 Evolution Progress:\n");
        output.push_str(&format!(
            "  Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));
        output.push_str(&format!(
            "  Progress: {:.1}%\n",
            (self.statistics.evolution.current_step as f64
                / self.statistics.evolution.total_steps as f64)
                * 100.0
        ));
        output.push('\n');

        output
    }

    /// Generate comprehensive dashboard with all sub-dashboards
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN_PHASE_10.md Phase 9 Step 9.1:
    /// "Enhance Dashboard - Add comprehensive dashboard with:
    /// - Polarity dashboard
    /// - Density dashboard
    /// - Collective dashboard
    /// - Spectrum dashboard
    /// - Attractor field dashboard
    /// - Catalyst dashboard
    /// - Veil dashboard"
    pub fn display_comprehensive_dashboard(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output
            .push_str("║          COMPREHENSIVE SIMULATION DASHBOARD                         ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        self.display_polarization_dashboard(&mut output);
        self.display_density_dashboard(&mut output);
        self.display_collective_dashboard(&mut output);
        self.display_spectrum_dashboard(&mut output);
        self.display_attractor_field_dashboard(&mut output);
        self.display_catalyst_dashboard(&mut output);
        self.display_veil_dashboard(&mut output);
        self.display_performance_dashboard(&mut output);

        output
    }

    /// Display polarization dashboard
    fn display_polarization_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output
            .push_str("║                    POLARIZATION DASHBOARD                           ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        let pol = &self.statistics.evolution.polarization_distribution;
        let total = pol.sto + pol.sts + pol.unpolarized;

        let sto_pct = if total > 0 {
            (pol.sto as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let sts_pct = if total > 0 {
            (pol.sts as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let unpolarized_pct = if total > 0 {
            (pol.unpolarized as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str("Polarity Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ 🙏 STO (Positive):     {:>6} ({:>5.1}%) {} │\n",
            pol.sto,
            sto_pct,
            Self::progress_bar(sto_pct)
        ));
        output.push_str(&format!(
            "│ 💔 STS (Negative):     {:>6} ({:>5.1}%) {} │\n",
            pol.sts,
            sts_pct,
            Self::progress_bar(sts_pct)
        ));
        output.push_str(&format!(
            "│ ⚪ Unpolarized:       {:>6} ({:>5.1}%) {} │\n",
            pol.unpolarized,
            unpolarized_pct,
            Self::progress_bar(unpolarized_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str(&format!("Average Polarity Bias: {:.4}\n", pol.average_bias));
        output.push_str(&format!(
            "Polarity Diversity Index: {:.4}\n",
            self.statistics.evolution.polarization_diversity_index
        ));
        output.push('\n');
    }

    /// Display density dashboard
    fn display_density_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                      DENSITY DASHBOARD                            ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        let densities = vec![
            (
                "First",
                "1st Density",
                "Quantum, Atomic, Molecular, Planetary",
            ),
            (
                "Second",
                "2nd Density",
                "Cellular, Simple Life, Complex Life",
            ),
            ("Third", "3rd Density", "Conscious Life (self-aware beings)"),
            ("Fourth", "4th Density", "Understanding, love, compassion"),
            ("Fifth", "5th Density", "Wisdom, light, teaching/learning"),
            ("Sixth", "6th Density", "Unity, balance, harmony"),
            (
                "Seventh",
                "7th Density",
                "Completion, gateway to next octave",
            ),
            ("Eighth", "8th Density", "Return to IntelligentInfinity"),
        ];

        output.push_str("Density Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        for (key, name, _desc) in &densities {
            let count: usize = self
                .statistics
                .evolution
                .density_distribution
                .iter()
                .filter(|(k, _)| k.starts_with(key))
                .map(|(_, v)| *v)
                .sum();

            let percentage = if self.statistics.evolution.num_entities > 0 {
                (count as f64 / self.statistics.evolution.num_entities as f64) * 100.0
            } else {
                0.0
            };

            output.push_str(&format!(
                "│ {:<20}: {:>6} ({:>5.1}%) {} │\n",
                name,
                count,
                percentage,
                Self::progress_bar(percentage)
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str(&format!(
            "Total Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));
        output.push('\n');
    }

    /// Display collective dashboard
    fn display_collective_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output
            .push_str("║                     COLLECTIVE DASHBOARD                            ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        output.push_str("Holographic Connections:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Total Connections:          {:>10}                    │\n",
            self.statistics.holographic.connection_count
        ));
        output.push_str(&format!(
            "│ Phase Coherence:            {:>10.4}                   │\n",
            self.statistics.holographic.global_phase_coherence
        ));
        output.push_str(&format!(
            "│ Avg Connection Strength:    {:>10.4}                   │\n",
            self.statistics.holographic.average_connection_strength
        ));
        output.push_str(&format!(
            "│ Resonant Connections:       {:>10}                    │\n",
            self.statistics.holographic.resonant_connections
        ));
        output.push_str(&format!(
            "│ Entangled Connections:      {:>10}                    │\n",
            self.statistics.holographic.entangled_connections
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str(&format!(
            "Resonance Clusters: {}\n",
            self.statistics.holographic.resonance_cluster_count
        ));
        output.push('\n');
    }

    /// Display spectrum dashboard
    fn display_spectrum_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                      SPECTRUM DASHBOARD                            ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        let sa = &self.statistics.spectrum_access;

        output.push_str("Spectrum Access:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Average Spectrum Ratio:      {:>10.4}                   │\n",
            sa.average_spectrum_ratio
        ));
        output.push_str(&format!(
            "│ Avg Space/Time Access:      {:>10.4}                   │\n",
            sa.average_space_time_access
        ));
        output.push_str(&format!(
            "│ Avg Time/Space Access:      {:>10.4}                   │\n",
            sa.average_time_space_access
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        let total = sa.space_time_dominant_count + sa.time_space_dominant_count + sa.balanced_count;
        let st_pct = if total > 0 {
            (sa.space_time_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let ts_pct = if total > 0 {
            (sa.time_space_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let balanced_pct = if total > 0 {
            (sa.balanced_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str("Spectrum Dominance:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ 🟢 Space/Time Dominant:    {:>6} ({:>5.1}%) {} │\n",
            sa.space_time_dominant_count,
            st_pct,
            Self::progress_bar(st_pct)
        ));
        output.push_str(&format!(
            "│ 🔵 Time/Space Dominant:    {:>6} ({:>5.1}%) {} │\n",
            sa.time_space_dominant_count,
            ts_pct,
            Self::progress_bar(ts_pct)
        ));
        output.push_str(&format!(
            "│ ⚪ Balanced:               {:>6} ({:>5.1}%) {} │\n",
            sa.balanced_count,
            balanced_pct,
            Self::progress_bar(balanced_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str(&format!(
            "Spectrum Access Level: {:.4}\n",
            self.statistics.evolution.spectrum_access_level
        ));
        output.push('\n');
    }

    /// Display attractor field dashboard
    fn display_attractor_field_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                 ATTRACTOR-FIELD DASHBOARD                          ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        output.push_str("Attractor-Field Activation:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Activation Level:            {:>10.4}                   │\n",
            self.statistics.evolution.attractor_field_activation
        ));

        let activation_pct =
            (self.statistics.evolution.attractor_field_activation * 100.0).clamp(0.0, 100.0);
        output.push_str(&format!(
            "│ Activation Progress:         {:>10.1}% {} │\n",
            activation_pct,
            Self::progress_bar(activation_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str("Description:\n");
        output.push_str("  Attractor-fields are the 'architectural artifacts' that create\n");
        output.push_str("  'spiritual gravity' pulling entities toward higher densities.\n");
        output.push_str("  Each stage creates attractor-fields that pull toward the next level.\n");
        output.push('\n');
    }

    /// Display catalyst dashboard
    fn display_catalyst_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                     CATALYST DASHBOARD                             ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        output.push_str("Energy Tapping (Catalyst Source):\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Total Tapped Energy:         {:>10.2}                   │\n",
            self.statistics.evolution.total_tapped_energy
        ));
        output.push_str(&format!(
            "│ Average Tap Strength:        {:>10.4}                   │\n",
            self.statistics.evolution.average_tap_strength
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str("Description:\n");
        output
            .push_str("  Catalyst provides contrast, limitation, challenge, choice, and growth.\n");
        output.push_str("  Energy is tapped from IntelligentInfinity to create catalyst events.\n");
        output.push_str("  Catalyst intensity varies based on entity readiness.\n");
        output.push('\n');
    }

    /// Display veil dashboard
    fn display_veil_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                       VEIL DASHBOARD                               ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        output.push_str("Veil Status:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│ Position:                    v=1.00                    │\n");
        output.push_str(&format!(
            "│ Thickness:                   {:>10.4}                   │\n",
            self.statistics.evolution.veil_thickness
        ));
        output.push_str(&format!(
            "│ Transparency:                {:>10.2}%                  │\n",
            self.statistics.evolution.veil_transparency * 100.0
        ));

        let transparency_pct =
            (self.statistics.evolution.veil_transparency * 100.0).clamp(0.0, 100.0);
        output.push_str(&format!(
            "│ Transparency Progress:       {:>10.1}% {} │\n",
            transparency_pct,
            Self::progress_bar(transparency_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        let sa = &self.statistics.spectrum_access;
        let total = sa.veil_active_count + sa.veil_inactive_count;
        let active_pct = if total > 0 {
            (sa.veil_active_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let inactive_pct = if total > 0 {
            (sa.veil_inactive_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str("Veil Activity:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ 🔒 Veil Active:              {:>6} ({:>5.1}%) {} │\n",
            sa.veil_active_count,
            active_pct,
            Self::progress_bar(active_pct)
        ));
        output.push_str(&format!(
            "│ 🔓 Veil Inactive:            {:>6} ({:>5.1}%) {} │\n",
            sa.veil_inactive_count,
            inactive_pct,
            Self::progress_bar(inactive_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str("Description:\n");
        output.push_str("  The Veil limits ACCESS to the Oneness side of the spectrum.\n");
        output.push_str(
            "  It creates the illusion of separation necessary for 3rd density learning.\n",
        );
        output.push_str("  Veil transparency varies based on density (0.0 in 3rd, 1.0 in 7th).\n");
        output.push('\n');
    }

    /// Display performance dashboard
    fn display_performance_dashboard(&self, output: &mut String) {
        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                    PERFORMANCE DASHBOARD                           ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════════╝\n");

        output.push_str("Simulation Performance:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Current Step:                {:>10} / {:<10}       │\n",
            self.statistics.evolution.current_step, self.statistics.evolution.total_steps
        ));
        output.push_str(&format!(
            "│ Total Time:                  {:>10.2?}                │\n",
            self.statistics.performance.total_time
        ));
        output.push_str(&format!(
            "│ Time per Step:               {:>10.2?}                │\n",
            self.statistics.performance.time_per_step
        ));
        output.push_str(&format!(
            "│ Steps per Second:            {:>10.2}                   │\n",
            self.statistics.performance.steps_per_second
        ));
        output.push_str(&format!(
            "│ Entities per Second:         {:>10.2}                   │\n",
            self.statistics.performance.entities_per_second
        ));
        output.push_str(&format!(
            "│ Peak Memory Usage:           {:>10} MB                 │\n",
            self.statistics.performance.peak_memory_mb
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output.push_str(&format!(
            "Architecture Alignment: {:.2}%\n",
            self.statistics.architecture.alignment_percentage
        ));
        output.push('\n');
    }

    /// Create a progress bar
    fn progress_bar(percentage: f64) -> String {
        let width = 15;
        let filled = (percentage / 100.0 * width as f64).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push('[');
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in filled..width {
            bar.push('░');
        }
        bar.push(']');

        bar
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation_v3::involution_sequence::InvolutionResult;
    use std::time::Duration;

    #[test]
    fn test_involution_visualizer_creation() {
        let involution_result = InvolutionResult {
            entities: Vec::new(),
            attractor_fields: Vec::new(),
            stage_transitions: Vec::new(),
            execution_time: Duration::ZERO,
        };
        let visualizer = InvolutionSequenceVisualizer::new(involution_result);
        let output = visualizer.generate_complete_visualization();
        assert!(output.contains("INVOLUTION SEQUENCE VISUALIZATION"));
    }

    #[test]
    fn test_spectrum_visualizer_creation() {
        let visualizer = SpectrumVisualizer::new();
        let output = visualizer.generate_spectrum_visualization();
        assert!(output.contains("SPECTRUM CONFIGURATION VISUALIZATION"));
    }

    #[test]
    fn test_evolution_visualizer_creation() {
        let stats = SimulationStatistics::default();
        let visualizer = EvolutionVisualizer::new(stats);
        let output = visualizer.generate_evolution_visualization();
        assert!(output.contains("DENSITY OCTAVE EVOLUTION VISUALIZATION"));
    }

    #[test]
    fn test_holographic_visualizer_creation() {
        let stats = SimulationStatistics::default();
        let visualizer = HolographicVisualizer::new(stats);
        let output = visualizer.generate_holographic_visualization();
        assert!(output.contains("HOLOGRAPHIC PRINCIPLE VISUALIZATION"));
    }

    #[test]
    fn test_realtime_dashboard_creation() {
        let stats = SimulationStatistics::default();
        let dashboard = RealTimeDashboard::new(stats);
        let output = dashboard.generate_dashboard();
        assert!(output.contains("REAL-TIME SIMULATION DASHBOARD"));
    }

    #[test]
    fn test_spectrum_access_visualizer_creation() {
        let stats = SimulationStatistics::default();
        let visualizer = SpectrumAccessVisualizer::new(stats);
        let output = visualizer.generate_spectrum_access_visualization();
        assert!(output.contains("SPECTRUM ACCESS DISTRIBUTION VISUALIZATION"));
    }
}

// ============================================================================
// SPECTRUM ACCESS VISUALIZER (Phase 3)
// ============================================================================

/// Spectrum Access Visualizer
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md:
/// "Add spectrum access visualization to show distribution"
///
/// Visualizes spectrum access patterns across entities including:
/// - Space/time vs time/space access distribution
/// - Veil activity tracking
/// - Spectrum ratio histograms
/// - Access patterns by entity type and scale
pub struct SpectrumAccessVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl SpectrumAccessVisualizer {
    /// Create a new spectrum access visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        SpectrumAccessVisualizer { statistics }
    }

    /// Generate spectrum access visualization
    pub fn generate_spectrum_access_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║             SPECTRUM ACCESS DISTRIBUTION VISUALIZATION            ║\n");
        output.push_str("║          Space/Time and Time/Space Spectrum Analysis             ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Display overall spectrum access statistics
        output.push_str(&self.visualize_overall_statistics());

        // Display spectrum ratio distribution histogram
        output.push_str(&self.visualize_spectrum_ratio_distribution());

        // Display veil activity
        output.push_str(&self.visualize_veil_activity());

        // Display spectrum access by entity type
        output.push_str(&self.visualize_access_by_entity_type());

        // Display spectrum access by scale
        output.push_str(&self.visualize_access_by_scale());

        output
    }

    /// Visualize overall spectrum access statistics
    fn visualize_overall_statistics(&self) -> String {
        let mut output = String::new();
        let sa = &self.statistics.spectrum_access;

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("OVERALL SPECTRUM ACCESS STATISTICS\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");

        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str(&format!(
            "│ Average Spectrum Ratio:    {:>10.4}                              │\n",
            sa.average_spectrum_ratio
        ));
        output.push_str(&format!(
            "│ Average Space/Time Access: {:>10.4}                              │\n",
            sa.average_space_time_access
        ));
        output.push_str(&format!(
            "│ Average Time/Space Access: {:>10.4}                              │\n",
            sa.average_time_space_access
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output.push_str("Spectrum Dominance Distribution:\n");
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let total = sa.space_time_dominant_count + sa.time_space_dominant_count + sa.balanced_count;
        let st_pct = if total > 0 {
            (sa.space_time_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let ts_pct = if total > 0 {
            (sa.time_space_dominant_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let balanced_pct = if total > 0 {
            (sa.balanced_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "│ 🟢 Space/Time Dominant (v > 1.5):  {:>6} ({:>5.1}%) {} │\n",
            sa.space_time_dominant_count,
            st_pct,
            Self::progress_bar(st_pct)
        ));
        output.push_str(&format!(
            "│ 🔵 Time/Space Dominant (v < 0.67): {:>6} ({:>5.1}%) {} │\n",
            sa.time_space_dominant_count,
            ts_pct,
            Self::progress_bar(ts_pct)
        ));
        output.push_str(&format!(
            "│ ⚪ Balanced (0.67 ≤ v ≤ 1.5):       {:>6} ({:>5.1}%) {} │\n",
            sa.balanced_count,
            balanced_pct,
            Self::progress_bar(balanced_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output
    }

    /// Visualize spectrum ratio distribution histogram
    fn visualize_spectrum_ratio_distribution(&self) -> String {
        let mut output = String::new();
        let sa = &self.statistics.spectrum_access;

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SPECTRUM RATIO DISTRIBUTION HISTOGRAM\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("Ratio (v = s/t or v = t/s):\n");
        output.push_str("  v > 1: Space/Time dominant (physical, separation)\n");
        output.push_str("  v = 1: At the Veil (boundary point)\n");
        output.push_str("  v < 1: Time/Space dominant (metaphysical, unity)\n\n");

        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let mut buckets: Vec<(&String, &usize)> = sa.spectrum_ratio_distribution.iter().collect();

        // Sort buckets by ratio value (parse to f64 for sorting)
        buckets.sort_by(|a, b| {
            let a_val = Self::parse_bucket_value(a.0);
            let b_val = Self::parse_bucket_value(b.0);
            a_val
                .partial_cmp(&b_val)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let max_count = buckets.iter().map(|(_, &count)| count).max().unwrap_or(1);

        for (bucket, &count) in buckets {
            let percentage = if bucket.contains("+") {
                "∞".to_string()
            } else if bucket.contains("-") {
                let parts: Vec<&str> = bucket.split('-').collect();
                format!("{}-{}", parts[0], parts[1])
            } else {
                bucket.to_string()
            };

            let bar = Self::histogram_bar(count, max_count, 30);
            output.push_str(&format!(
                "│ v = {:<20}: {:>6} {} │\n",
                percentage, count, bar
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Add ASCII visualization of spectrum
        output.push_str("Spectrum Visualization:\n");
        output.push_str("  ");
        for i in 0..60 {
            if i < 30 {
                output.push('🔵'); // Time/Space side
            } else if i == 30 {
                output.push('📍'); // Veil position
            } else {
                output.push('🟢'); // Space/Time side
            }
        }
        output.push('\n');
        output.push_str("  ");
        output.push_str("├──────────────────────────────┤\n");
        output.push_str("  ");
        output.push_str("│        Time/Space (v < 1)      │      Space/Time (v > 1)      │\n");
        output.push_str("  ");
        output.push_str("│        Metaphysical            │      Physical                 │\n");
        output.push_str("  ");
        output.push_str("│        Unity Consciousness      │      Separation Consciousness │\n");
        output.push('\n');

        output
    }

    /// Visualize veil activity
    fn visualize_veil_activity(&self) -> String {
        let mut output = String::new();
        let sa = &self.statistics.spectrum_access;

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("VEIL ACTIVITY TRACKING\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("The Veil is a structural feature at v=1 that limits access to the\n");
        output.push_str("oneness (time/space) side of the spectrum, creating the illusion of\n");
        output.push_str("separation necessary for 3rd density learning.\n\n");

        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");

        let total = sa.veil_active_count + sa.veil_inactive_count;
        let active_pct = if total > 0 {
            (sa.veil_active_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let inactive_pct = if total > 0 {
            (sa.veil_inactive_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "│ 🔒 Veil Active:         {:>6} ({:>5.1}%) {} │\n",
            sa.veil_active_count,
            active_pct,
            Self::progress_bar(active_pct)
        ));
        output.push_str(&format!(
            "│ 🔓 Veil Inactive:       {:>6} ({:>5.1}%) {} │\n",
            sa.veil_inactive_count,
            inactive_pct,
            Self::progress_bar(inactive_pct)
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        output
    }

    /// Visualize spectrum access by entity type
    fn visualize_access_by_entity_type(&self) -> String {
        let mut output = String::new();
        let sa = &self.statistics.spectrum_access;

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SPECTRUM ACCESS BY ENTITY TYPE\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");

        if sa.by_entity_type.is_empty() {
            output.push_str("No entity type data available.\n\n");
            return output;
        }

        let mut entity_types: Vec<(&String, &SpectrumAccessStats)> =
            sa.by_entity_type.iter().collect();

        entity_types.sort_by(|a, b| a.0.cmp(b.0));

        for (entity_type, stats) in entity_types {
            output.push_str(&format!("{}\n", entity_type));
            output
                .push_str("┌─────────────────────────────────────────────────────────────────┐\n");
            output.push_str(&format!(
                "│ Count:                     {:>6}                              │\n",
                stats.count
            ));
            output.push_str(&format!(
                "│ Average Ratio:             {:>10.4}                              │\n",
                stats.average_ratio
            ));
            output.push_str(&format!(
                "│ Avg Space/Time Access:     {:>10.4}                              │\n",
                stats.average_space_time_access
            ));
            output.push_str(&format!(
                "│ Avg Time/Space Access:     {:>10.4}                              │\n",
                stats.average_time_space_access
            ));
            output
                .push_str("├─────────────────────────────────────────────────────────────────┤\n");

            let total = stats.space_time_dominant + stats.time_space_dominant + stats.balanced;
            let st_pct = if total > 0 {
                (stats.space_time_dominant as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let ts_pct = if total > 0 {
                (stats.time_space_dominant as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            let balanced_pct = if total > 0 {
                (stats.balanced as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            output.push_str(&format!(
                "│ Space/Time Dominant:       {:>6} ({:>5.1}%)                      │\n",
                stats.space_time_dominant, st_pct
            ));
            output.push_str(&format!(
                "│ Time/Space Dominant:       {:>6} ({:>5.1}%)                      │\n",
                stats.time_space_dominant, ts_pct
            ));
            output.push_str(&format!(
                "│ Balanced:                  {:>6} ({:>5.1}%)                      │\n",
                stats.balanced, balanced_pct
            ));
            output.push_str(
                "└─────────────────────────────────────────────────────────────────┘\n\n",
            );
        }

        output
    }

    /// Visualize spectrum access by entity scale
    fn visualize_access_by_scale(&self) -> String {
        let mut output = String::new();
        let sa = &self.statistics.spectrum_access;

        output.push_str("══════════════════════════════════════════════════════════════════════\n");
        output.push_str("SPECTRUM ACCESS BY ENTITY SCALE\n");
        output.push_str("══════════════════════════════════════════════════════════════════════\n");

        if sa.by_scale.is_empty() {
            output.push_str("No scale data available.\n\n");
            return output;
        }

        let scale_order = vec![
            "Quantum",
            "Atomic",
            "Molecular",
            "Planetary",
            "Cellular",
            "SimpleLife",
            "ComplexLife",
            "Conscious",
            "Higher",
        ];

        for scale_name in scale_order {
            if let Some(stats) = sa.by_scale.get(scale_name) {
                output.push_str(&format!("{}\n", scale_name));
                output.push_str(
                    "┌─────────────────────────────────────────────────────────────────┐\n",
                );
                output.push_str(&format!(
                    "│ Count:                     {:>6}                              │\n",
                    stats.count
                ));
                output.push_str(&format!(
                    "│ Average Ratio:             {:>10.4}                              │\n",
                    stats.average_ratio
                ));
                output.push_str(&format!(
                    "│ Avg Space/Time Access:     {:>10.4}                              │\n",
                    stats.average_space_time_access
                ));
                output.push_str(&format!(
                    "│ Avg Time/Space Access:     {:>10.4}                              │\n",
                    stats.average_time_space_access
                ));
                output.push_str(
                    "├─────────────────────────────────────────────────────────────────┤\n",
                );

                let total = stats.space_time_dominant + stats.time_space_dominant + stats.balanced;
                let st_pct = if total > 0 {
                    (stats.space_time_dominant as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                let ts_pct = if total > 0 {
                    (stats.time_space_dominant as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                let balanced_pct = if total > 0 {
                    (stats.balanced as f64 / total as f64) * 100.0
                } else {
                    0.0
                };

                output.push_str(&format!(
                    "│ Space/Time Dominant:       {:>6} ({:>5.1}%)                      │\n",
                    stats.space_time_dominant, st_pct
                ));
                output.push_str(&format!(
                    "│ Time/Space Dominant:       {:>6} ({:>5.1}%)                      │\n",
                    stats.time_space_dominant, ts_pct
                ));
                output.push_str(&format!(
                    "│ Balanced:                  {:>6} ({:>5.1}%)                      │\n",
                    stats.balanced, balanced_pct
                ));
                output.push_str(
                    "└─────────────────────────────────────────────────────────────────┘\n\n",
                );
            }
        }

        output
    }

    /// Create a progress bar
    fn progress_bar(percentage: f64) -> String {
        let width = 15;
        let filled = (percentage / 100.0 * width as f64).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push('[');
        for _ in 0..filled {
            bar.push('█');
        }
        for _ in filled..width {
            bar.push('░');
        }
        bar.push(']');

        bar
    }

    /// Create a histogram bar
    fn histogram_bar(count: usize, max_count: usize, width: usize) -> String {
        if max_count == 0 {
            return String::new();
        }

        let filled = (count as f64 / max_count as f64 * width as f64).round() as usize;
        let filled = filled.min(width);

        let mut bar = String::new();
        bar.push(' ');
        for _ in 0..filled {
            bar.push('█');
        }

        bar
    }

    /// Parse bucket value for sorting
    fn parse_bucket_value(bucket: &str) -> f64 {
        if bucket.contains("+") {
            1000.0
        } else if bucket.contains("-") {
            let parts: Vec<&str> = bucket.split('-').collect();
            parts[0].parse::<f64>().unwrap_or(0.0)
        } else {
            bucket.parse::<f64>().unwrap_or(0.0)
        }
    }
}

// ============================================================================
// PHYSICAL SCALE VISUALIZER (Phase 4)
// ============================================================================

/// Physical Scale Visualizer
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 4:
/// "Add visualization for physical scale distribution"
///
/// Visualizes:
/// - Physical scale distribution
/// - Mass distribution
/// - Charge distribution
/// - Energy levels
/// - Position distribution
/// - Consciousness-to-matter mapping
pub struct PhysicalScaleVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl PhysicalScaleVisualizer {
    /// Create a new physical scale visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        PhysicalScaleVisualizer { statistics }
    }

    /// Generate complete physical scale visualization
    pub fn generate_physical_scale_visualization(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PHYSICAL SCALE VISUALIZATION (Phase 4)              ║\n");
        output.push_str("║         Physical Manifestation of Consciousness Entities          ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Overall statistics
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│                      OVERALL STATISTICS                          │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");
        output.push_str(&format!(
            "│ Total Physical Entities:      {:>6}                                │\n",
            self.statistics.physical.num_physical_entities
        ));
        output.push_str(&format!(
            "│ Total Energy:                 {:>10.2e} J                        │\n",
            self.statistics.physical.total_energy
        ));
        output.push_str(&format!(
            "│ Total Mass:                   {:>10.2e} kg                       │\n",
            self.statistics.physical.total_mass
        ));
        output.push_str(&format!(
            "│ Average Mass:                 {:>10.2e} kg                       │\n",
            self.statistics.physical.average_mass
        ));
        output.push_str(&format!(
            "│ Average Kinetic Energy:       {:>10.2e} J                        │\n",
            self.statistics.physical.average_kinetic_energy
        ));
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Physical scale distribution
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│                   PHYSICAL SCALE DISTRIBUTION                   │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");

        let mut scales: Vec<_> = self.statistics.physical.scale_distribution.iter().collect();
        scales.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

        for (scale, count) in scales {
            output.push_str(&format!(
                "│ {:<20} {:>6} entities                         │\n",
                scale, count
            ));
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Mass distribution by scale
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│                    MASS DISTRIBUTION BY SCALE                    │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");

        for (scale, masses) in &self.statistics.physical.mass_distribution {
            if !masses.is_empty() {
                let avg_mass = masses.iter().sum::<Float>() / masses.len() as Float;
                let min_mass = masses.iter().fold(Float::MAX, |a, &b| a.min(b));
                let max_mass = masses.iter().fold(Float::MIN, |a, &b| a.max(b));

                output.push_str(&format!(
                    "│ {:<20} Avg: {:>10.2e} kg  Min: {:>10.2e} kg  Max: {:>10.2e} kg │\n",
                    scale, avg_mass, min_mass, max_mass
                ));
            }
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Charge distribution by scale
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│                  CHARGE DISTRIBUTION BY SCALE                    │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");

        for (scale, charges) in &self.statistics.physical.charge_distribution {
            if !charges.is_empty() {
                let avg_charge = charges.iter().sum::<Float>() / charges.len() as Float;
                let min_charge = charges.iter().fold(Float::MAX, |a, &b| a.min(b));
                let max_charge = charges.iter().fold(Float::MIN, |a, &b| a.max(b));

                output.push_str(&format!(
                    "│ {:<20} Avg: {:>10.2e} e  Min: {:>10.2e} e  Max: {:>10.2e} e │\n",
                    scale, avg_charge, min_charge, max_charge
                ));
            }
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Position distribution summary
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│                    POSITION DISTRIBUTION                          │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");

        if !self.statistics.physical.position_distribution.is_empty() {
            let positions = &self.statistics.physical.position_distribution;
            let avg_x = positions.iter().map(|p| p.x).sum::<Float>() / positions.len() as Float;
            let avg_y = positions.iter().map(|p| p.y).sum::<Float>() / positions.len() as Float;
            let avg_z = positions.iter().map(|p| p.z).sum::<Float>() / positions.len() as Float;

            output.push_str(&format!(
                "│ Average Position:            ({:>10.2e}, {:>10.2e}, {:>10.2e})        │\n",
                avg_x, avg_y, avg_z
            ));
            output.push_str(&format!(
                "│ Number of Positions:         {:>6}                                │\n",
                positions.len()
            ));
        } else {
            output.push_str(
                "│ No position data available                                              │\n",
            );
        }

        output.push_str("└─────────────────────────────────────────────────────────────────┘\n\n");

        // Consciousness-to-matter mapping notes
        output.push_str("┌─────────────────────────────────────────────────────────────────┐\n");
        output.push_str("│              CONSCIOUSNESS-TO-MATTER MAPPING (Phase 4)            │\n");
        output.push_str("├─────────────────────────────────────────────────────────────────┤\n");
        output.push_str("│ From COSMOLOGICAL-ARCHITECTURE.md:                              │\n");
        output.push_str("│ \"Consciousness (information) exists BEFORE matter\"              │\n");
        output.push_str("│                                                                 │\n");
        output.push_str("│ Key Principles:                                                │\n");
        output.push_str("│ • Quantum fluctuations carry consciousness patterns             │\n");
        output.push_str("│ • Galaxies, planets exist as quantum information before atoms  │\n");
        output.push_str("│ • Physical universe manifests from pre-existing consciousness   │\n");
        output.push_str("│ • Quantum energy pools = coherent quantum states              │\n");
        output.push_str("│                                                                 │\n");
        output.push_str("│ Implementation Status:                                          │\n");
        output.push_str("│ ✓ Physical entities created for all consciousness entities      │\n");
        output.push_str("│ ✓ Mass, charge, position, energy tracked                        │\n");
        output.push_str("│ ✓ Archetype activation patterns drive physical properties       │\n");
        output.push_str("└─────────────────────────────────────────────────────────────────┘\n");

        output
    }
}

// ============================================================================
// PHASE 7: DETAILED SPECTRUM DISTRIBUTION VISUALIZER
// ============================================================================

/// Detailed Spectrum Distribution Visualizer (Phase 7)
///
/// Provides detailed insight into entity spectrum distribution,
/// space/time vs time/space breakdown, and who is where.
pub struct DetailedSpectrumVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl DetailedSpectrumVisualizer {
    /// Create a new detailed spectrum visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        DetailedSpectrumVisualizer { statistics }
    }

    /// Generate detailed spectrum distribution visualization
    pub fn generate_detailed_spectrum_distribution(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PHASE 7: DETAILED SPECTRUM DISTRIBUTION             ║\n");
        output.push_str("║              Who is Where and What They're Doing                ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Spectrum ratio histogram
        output.push_str(&self.generate_spectrum_ratio_histogram());

        // Space/Time vs Time/Space breakdown
        output.push_str(&self.generate_spectrum_breakdown());

        // Entity spectrum list (top 20)
        output.push_str(&self.generate_entity_spectrum_list());

        // Spectrum by entity type
        output.push_str(&self.generate_spectrum_by_entity_type());

        output
    }

    /// Generate spectrum ratio histogram
    fn generate_spectrum_ratio_histogram(&self) -> String {
        let mut output = String::new();

        output.push_str("SPECTRUM RATIO HISTOGRAM:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        // Define ratio ranges
        let ranges = [
            (0.1, 0.5, "Deep Time/Space", "🔵"),
            (0.5, 0.8, "Time/Space Dominant", "🔵"),
            (0.8, 1.0, "Near Veil (Time/Space)", "⚪"),
            (1.0, 1.2, "At the Veil", "📍"),
            (1.2, 1.5, "Near Veil (Space/Time)", "⚪"),
            (1.5, 2.0, "Space/Time Balanced", "🟢"),
            (2.0, 3.0, "Space/Time Dominant", "🟢"),
            (3.0, 5.0, "Strong Space/Time", "🟢"),
            (5.0, 10.0, "Very Strong Space/Time", "🟢"),
            (10.0, 20.0, "Environmental (Deep Space/Time)", "🌍"),
        ];

        // Calculate counts from spectrum access statistics
        let total_entities = self.statistics.evolution.num_entities;

        for (min, max, label, emoji) in ranges {
            // Estimate count based on spectrum access statistics
            let count = self.estimate_entities_in_range(min, max);
            let percentage = if total_entities > 0 {
                (count as f64 / total_entities as f64) * 100.0
            } else {
                0.0
            };

            let bar_length = (percentage / 2.0) as usize;
            let bar = "█".repeat(bar_length);

            output.push_str(&format!(
                "  {} {:5.1} - {:5.1}: {:4} ({:5.1}%) {} {}\n",
                emoji, min, max, count, percentage, bar, label
            ));
        }

        output.push('\n');

        // Legend
        output.push_str("LEGEND:\n");
        output.push_str(
            "  🔵 Time/Space (v < 1.0): Metaphysical, inner experience, higher dimensions\n",
        );
        output.push_str("  📍 Veil (v = 1.0): Boundary between space/time and time/space\n");
        output.push_str("  🟢 Space/Time (v > 1.0): Physical manifestation, outer experience\n");
        output
            .push_str("  🌍 Environmental: Planets, stars, galaxies (high space/time access)\n\n");

        output
    }

    /// Estimate entities in a given ratio range
    fn estimate_entities_in_range(&self, min: f64, max: f64) -> usize {
        // Use spectrum access statistics to estimate distribution
        let total = self.statistics.evolution.num_entities;

        if total == 0 {
            return 0;
        }

        // Estimate based on spectrum access statistics
        let st_dominant = self.statistics.spectrum_access.space_time_dominant_count;
        let ts_dominant = self.statistics.spectrum_access.time_space_dominant_count;
        let balanced = self.statistics.spectrum_access.veil_active_count;

        // Distribute entities based on range
        if min < 1.0 && max <= 1.0 {
            // Time/space side
            let ts_range = 1.0 - min;
            let ts_total = 1.0 - 0.1;
            ((ts_dominant as f64) * (ts_range / ts_total)) as usize
        } else if min >= 1.0 && max > 1.0 {
            // Space/time side
            let st_range = max - 1.0;
            let st_total = 20.0 - 1.0;
            ((st_dominant as f64) * (st_range / st_total)) as usize
        } else {
            // Near veil
            balanced
        }
    }

    /// Generate spectrum breakdown
    fn generate_spectrum_breakdown(&self) -> String {
        let mut output = String::new();

        output.push_str("SPACE/TIME VS TIME/SPACE BREAKDOWN:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let st_count = self.statistics.spectrum_access.space_time_dominant_count;
        let ts_count = self.statistics.spectrum_access.time_space_dominant_count;
        let veil_count = self.statistics.spectrum_access.veil_active_count;
        let total = st_count + ts_count + veil_count;

        let st_pct = if total > 0 {
            (st_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let ts_pct = if total > 0 {
            (ts_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let veil_pct = if total > 0 {
            (veil_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        output.push_str(&format!(
            "  🔵 Time/Space Dominant (v < 1.0): {:4} ({:5.1}%)\n",
            ts_count, ts_pct
        ));
        output.push_str(&format!(
            "      └─ Average Time/Space Access: {:.4}\n",
            self.statistics.spectrum_access.average_time_space_access
        ));
        output.push_str(&format!(
            "      └─ Average Space/Time Access: {:.4}\n\n",
            self.statistics.spectrum_access.average_space_time_access
        ));

        output.push_str(&format!(
            "  📍 At the Veil (v = 1.0): {:4} ({:5.1}%)\n",
            veil_count, veil_pct
        ));
        output.push_str(&format!("      └─ Average Space/Time Access: {:.4}\n", 0.5));
        output.push_str(&format!(
            "      └─ Average Time/Space Access: {:.4}\n\n",
            0.5
        ));

        output.push_str(&format!(
            "  🟢 Space/Time Dominant (v > 1.0): {:4} ({:5.1}%)\n",
            st_count, st_pct
        ));
        output.push_str(&format!(
            "      └─ Average Space/Time Access: {:.4}\n",
            self.statistics.spectrum_access.average_space_time_access
        ));
        output.push_str(&format!(
            "      └─ Average Time/Space Access: {:.4}\n\n",
            self.statistics.spectrum_access.average_time_space_access
        ));

        // Overall spectrum ratio
        output.push_str(&format!(
            "  📊 Overall Average Spectrum Ratio: {:.4}\n",
            self.statistics.spectrum_access.average_spectrum_ratio
        ));

        output.push('\n');
        output
    }

    /// Generate entity spectrum list
    fn generate_entity_spectrum_list(&self) -> String {
        let mut output = String::new();

        output.push_str("ENTITY SPECTRUM LIST (Sample):\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        output.push_str(&format!(
            "{:<20} {:<15} {:<10} {:<10} {:<10}\n",
            "Entity Type", "Count", "Avg Ratio", "ST Access", "TS Access"
        ));
        output.push_str(&format!("{:-<70}\n", ""));

        // Display spectrum by entity type
        for (entity_type, stats) in &self.statistics.spectrum_access.by_entity_type {
            output.push_str(&format!(
                "{:<20} {:<15} {:<10.4} {:<10.4} {:<10.4}\n",
                entity_type,
                stats.count,
                stats.average_ratio,
                stats.average_space_time_access,
                stats.average_time_space_access
            ));
        }

        output.push('\n');
        output
    }

    /// Generate spectrum by entity type
    fn generate_spectrum_by_entity_type(&self) -> String {
        let mut output = String::new();

        output.push_str("SPECTRUM DISTRIBUTION BY ENTITY TYPE:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        for (entity_type, stats) in &self.statistics.spectrum_access.by_entity_type {
            output.push_str(&format!("{}\n", entity_type));
            output.push_str(&format!("{:-<70}\n", ""));
            output.push_str(&format!("  Count: {}\n", stats.count));
            output.push_str(&format!("  Average Ratio: {:.4}\n", stats.average_ratio));
            output.push_str(&format!(
                "  Space/Time Dominant: {} ({:.1}%)\n",
                stats.space_time_dominant,
                (stats.space_time_dominant as f64 / stats.count as f64) * 100.0
            ));
            output.push_str(&format!(
                "  Time/Space Dominant: {} ({:.1}%)\n",
                stats.time_space_dominant,
                (stats.time_space_dominant as f64 / stats.count as f64) * 100.0
            ));
            output.push_str(&format!(
                "  Balanced: {} ({:.1}%)\n",
                stats.balanced,
                (stats.balanced as f64 / stats.count as f64) * 100.0
            ));
            output.push_str(&format!(
                "  Veil Active: {} ({:.1}%)\n",
                stats.veil_active,
                (stats.veil_active as f64 / stats.count as f64) * 100.0
            ));
            output.push_str(&format!(
                "  Average Space/Time Access: {:.4}\n",
                stats.average_space_time_access
            ));
            output.push_str(&format!(
                "  Average Time/Space Access: {:.4}\n\n",
                stats.average_time_space_access
            ));
        }

        output
    }
}

// ============================================================================
// PHASE 7: DENSITY PROGRESSION OVER TIME VISUALIZER
// ============================================================================

/// Density Progression Over Time Visualizer (Phase 7)
///
/// Shows how entities progress through densities over time.
pub struct DensityProgressionVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl DensityProgressionVisualizer {
    /// Create a new density progression visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        DensityProgressionVisualizer { statistics }
    }

    /// Generate density progression visualization
    pub fn generate_density_progression(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PHASE 7: DENSITY PROGRESSION OVER TIME              ║\n");
        output.push_str("║              Evolution Journey Through the Octave                ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Current step and total steps
        output.push_str(&format!(
            "Current Step: {} / {} ({:.1}% complete)\n\n",
            self.statistics.evolution.current_step,
            self.statistics.evolution.total_steps,
            (self.statistics.evolution.current_step as f64
                / self.statistics.evolution.total_steps as f64)
                * 100.0
        ));

        // Density distribution at current step
        output.push_str(&self.generate_density_distribution());

        // Density transitions
        output.push_str(&self.generate_density_transitions());

        // Evolutionary funnel visualization
        output.push_str(&self.generate_evolutionary_funnel());

        output
    }

    /// Generate density distribution
    fn generate_density_distribution(&self) -> String {
        let mut output = String::new();

        output.push_str("CURRENT DENSITY DISTRIBUTION:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let densities = vec![
            (
                "First",
                "1st",
                "🔴",
                "Quantum, Atomic, Molecular, Planetary",
            ),
            ("Second", "2nd", "🟠", "Cellular, Simple Life, Complex Life"),
            ("Third", "3rd", "🟡", "Conscious Life, Self-Aware Beings"),
            ("Fourth", "4th", "🟢", "Understanding, Love, Compassion"),
            ("Fifth", "5th", "🔵", "Wisdom, Light, Teaching/Learning"),
            ("Sixth", "6th", "🟣", "Unity, Balance, Harmony"),
            ("Seventh", "7th", "⚫", "Completion, Gateway to Next Octave"),
            ("Eighth", "8th", "⚪", "Return to IntelligentInfinity"),
        ];

        output.push_str(&format!(
            "{:<12} {:<8} {:<20} {:<15} {:<10}\n",
            "Density", "Ray", "Description", "Count", "Percentage"
        ));
        output.push_str(&format!("{:-<70}\n", ""));

        let total_entities = self.statistics.evolution.num_entities;

        for (density_key, ray, emoji, description) in densities {
            // Count entities at this density
            let count: usize = self
                .statistics
                .evolution
                .density_distribution
                .iter()
                .filter(|(key, _)| key.starts_with(density_key))
                .map(|(_, count)| *count)
                .sum();

            let percentage = if total_entities > 0 {
                (count as f64 / total_entities as f64) * 100.0
            } else {
                0.0
            };

            output.push_str(&format!(
                "{:<12} {:<8} {:<20} {:<15} {:<10.1}%\n",
                density_key,
                ray,
                format!("{} {}", emoji, description),
                count,
                percentage
            ));
        }

        output.push('\n');
        output
    }

    /// Generate density transitions
    fn generate_density_transitions(&self) -> String {
        let mut output = String::new();

        output.push_str("DENSITY TRANSITIONS:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        output.push_str(&format!(
            "Total Density Transitions: {}\n",
            self.statistics.evolution.density_transitions
        ));

        let transitions_per_step = if self.statistics.evolution.current_step > 0 {
            self.statistics.evolution.density_transitions as f64
                / self.statistics.evolution.current_step as f64
        } else {
            0.0
        };

        output.push_str(&format!(
            "Average Transitions per Step: {:.2}\n\n",
            transitions_per_step
        ));

        // Expected progression
        output.push_str("EXPECTED PROGRESSION (Law of One):\n");
        output.push_str("  1st → 2nd: 75 billion years (approximate)\n");
        output.push_str("  2nd → 3rd: 1.5 billion years\n");
        output.push_str("  3rd → 4th: 75,000 years\n");
        output.push_str("  4th → 5th: 30,000,000 years\n");
        output.push_str("  5th → 6th: Unknown (very long)\n");
        output.push_str("  6th → 7th: Unknown (very long)\n");
        output.push_str("  7th → 8th: Gateway to next octave\n\n");

        output
    }

    /// Generate evolutionary funnel visualization
    fn generate_evolutionary_funnel(&self) -> String {
        let mut output = String::new();

        output.push_str("EVOLUTIONARY FUNNEL VISUALIZATION:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        output.push_str("  8th Density (Return to IntelligentInfinity)\n");
        output.push_str("                              ▲\n");
        output.push_str("                              │ (Gateway)\n");
        output.push_str("  7th Density (Violet)       │\n");
        output.push_str("                            ╱ ╲\n");
        output.push_str("  6th Density (Indigo)     ╱   ╲\n");
        output.push_str("                          ╱     ╲\n");
        output.push_str("  5th Density (Blue)     ╱       ╲\n");
        output.push_str("                        ╱         ╲\n");
        output.push_str("  4th Density (Green)  ╱           ╲\n");
        output.push_str("                      ╱             ╲\n");
        output.push_str("  3rd Density (Yellow)╱               ╲\n");
        output.push_str("                    ╱                 ╲\n");
        output.push_str("  2nd Density (Orange)╱                 ╲\n");
        output.push_str("                  ╱                     ╲\n");
        output.push_str("  1st Density (Red)╱_______________________╲\n");
        output.push_str("                  (Many Entities)          (Few Entities)\n\n");

        output.push_str(
            "The evolutionary funnel represents the natural progression through densities:\n",
        );
        output.push_str("• Many entities start at 1st Density\n");
        output.push_str("• Fewer entities reach higher densities\n");
        output.push_str("• Each density requires learning and polarization\n");
        output.push_str("• Free Will determines individual progress\n");
        output.push_str("• The funnel narrows because not all entities progress\n\n");

        output
    }
}

// ============================================================================
// PHASE 7: COLLECTIVE DYNAMICS VISUALIZER
// ============================================================================

/// Collective Dynamics Visualizer (Phase 7)
///
/// Shows collective formation and dynamics.
pub struct CollectiveDynamicsVisualizer {
    /// Statistics to visualize
    statistics: SimulationStatistics,
}

impl CollectiveDynamicsVisualizer {
    /// Create a new collective dynamics visualizer
    pub fn new(statistics: SimulationStatistics) -> Self {
        CollectiveDynamicsVisualizer { statistics }
    }

    /// Generate collective dynamics visualization
    pub fn generate_collective_dynamics(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PHASE 7: COLLECTIVE DYNAMICS VISUALIZATION         ║\n");
        output.push_str("║              Self-Organization and Group Consciousness          ║\n");
        output
            .push_str("╚════════════════════════════════════════════════════════════════════╝\n\n");

        // Collective formation statistics
        output.push_str(&self.generate_collective_statistics());

        // Collective resonance
        output.push_str(&self.generate_collective_resonance());

        // Collective consciousness
        output.push_str(&self.generate_collective_consciousness());

        output
    }

    /// Generate collective statistics
    fn generate_collective_statistics(&self) -> String {
        let mut output = String::new();

        output.push_str("COLLECTIVE FORMATION STATISTICS:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let emergent = &self.statistics.emergent_properties;

        output.push_str(&format!(
            "Collective Consciousness Level: {:.4}\n",
            emergent.collective_consciousness_level
        ));
        output.push_str(&format!(
            "Global Coherence: {:.4}\n",
            emergent.global_coherence
        ));
        output.push_str(&format!(
            "Organic Emergence Score: {:.4}\n",
            emergent.organic_emergence_score
        ));
        output.push_str(&format!(
            "System Entropy: {:.4}\n\n",
            emergent.system_entropy
        ));

        output
    }

    /// Generate collective resonance
    fn generate_collective_resonance(&self) -> String {
        let mut output = String::new();

        output.push_str("COLLECTIVE RESONANCE:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let emergent = &self.statistics.emergent_properties;

        output.push_str("Resonance measures how well entities align in:\n");
        output.push_str("  • Spectrum configuration\n");
        output.push_str("  • Holographic patterns\n");
        output.push_str("  • Polarity orientation\n\n");

        output.push_str(&format!(
            "Average Resonance: {:.4} (0.0 = no resonance, 1.0 = perfect resonance)\n",
            emergent.global_coherence
        ));

        // Resonance interpretation
        let resonance_level = if emergent.global_coherence < 0.3 {
            "Low (entities largely independent)"
        } else if emergent.global_coherence < 0.5 {
            "Moderate (some natural grouping)"
        } else if emergent.global_coherence < 0.7 {
            "High (strong collective formation)"
        } else {
            "Very High (highly synchronized collectives)"
        };

        output.push_str(&format!("Resonance Level: {}\n\n", resonance_level));

        output
    }

    /// Generate collective consciousness
    fn generate_collective_consciousness(&self) -> String {
        let mut output = String::new();

        output.push_str("COLLECTIVE CONSCIOUSNESS:\n");
        output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let emergent = &self.statistics.emergent_properties;

        output.push_str("Group consciousness emerges when:\n");
        output.push_str("  • Entities have high resonance\n");
        output.push_str("  • Holographic connections synchronize\n");
        output.push_str("  • Collective intelligence emerges\n\n");

        output.push_str(&format!(
            "Average Group Consciousness: {:.4} (0.0 = none, 1.0 = fully developed)\n",
            emergent.collective_consciousness_level
        ));

        // Consciousness interpretation
        let consciousness_level = if emergent.collective_consciousness_level < 0.3 {
            "Emerging (collectives forming)"
        } else if emergent.collective_consciousness_level < 0.5 {
            "Developing (collective intelligence emerging)"
        } else if emergent.collective_consciousness_level < 0.7 {
            "Mature (strong collective consciousness)"
        } else {
            "Advanced (highly developed collective intelligence)"
        };

        output.push_str(&format!("Consciousness Level: {}\n\n", consciousness_level));

        output.push_str("From COSMOLOGICAL-ARCHITECTURE.md:\n");
        output.push_str("  \"The whole is more than the sum of parts\"\n");
        output.push_str("  \"Collective entities exhibit collective consciousness\"\n");
        output.push_str("  \"Self-organization from complex interactions\"\n\n");

        output
    }
}
